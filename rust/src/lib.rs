#![allow(
    clippy::missing_safety_doc,
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    clippy::borrowed_box
)]

mod core;
mod crypto;
mod external;
mod helpers;
mod transport;

use std::{
    ffi::{CStr, CString},
    intrinsics::transmute,
    io,
    os::raw::{c_char, c_void},
    str::FromStr,
    sync::Arc,
};

use allo_isolate::{
    ffi::{DartCObject, DartPort},
    IntoDart, Isolate,
};
use anyhow::Result;
use lazy_static::lazy_static;
use nekoton_utils::SimpleClock;
use serde::Serialize;
use tokio::runtime::{Builder, Runtime};
use ton_block::MsgAddressInt;

lazy_static! {
    static ref RUNTIME: io::Result<Runtime> = Builder::new_multi_thread()
        .enable_all()
        .thread_name("nekoton_flutter")
        .build();
    static ref CLOCK: Arc<SimpleClock> = Arc::new(SimpleClock {});
}

#[macro_export]
macro_rules! runtime {
    () => {
        RUNTIME.as_ref().unwrap()
    };
}

#[macro_export]
macro_rules! clock {
    () => {
        CLOCK.clone()
    };
}

#[no_mangle]
pub unsafe extern "C" fn nt_store_dart_post_cobject(ptr: *mut c_void) {
    let ptr = transmute::<
        *mut c_void,
        unsafe extern "C" fn(port_id: DartPort, message: *mut DartCObject) -> bool,
    >(ptr);

    allo_isolate::store_dart_post_cobject(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn nt_free_cstring(ptr: *mut c_char) {
    ptr.to_string_from_ptr();
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum ExecutionResult<T>
where
    T: Serialize,
{
    Ok(T),
    Err(String),
}

pub trait MatchResult {
    fn match_result(self) -> *mut c_char;
}

impl<T> MatchResult for Result<T, String>
where
    T: Serialize,
{
    fn match_result(self) -> *mut c_char {
        let result = match self {
            Ok(ok) => ExecutionResult::Ok(ok),
            Err(err) => ExecutionResult::Err(err),
        };

        serde_json::to_string(&result).unwrap().to_cstring_ptr()
    }
}

pub trait HandleError {
    type Output;

    fn handle_error(self) -> Result<Self::Output, String>;
}

impl<T, E> HandleError for Result<T, E>
where
    E: ToString,
{
    type Output = T;

    fn handle_error(self) -> Result<Self::Output, String> {
        self.map_err(|e| e.to_string())
    }
}

trait PostWithResult {
    fn post_with_result(&self, data: impl IntoDart) -> Result<(), String>;
}

impl PostWithResult for Isolate {
    fn post_with_result(&self, data: impl IntoDart) -> Result<(), String> {
        match self.post(data) {
            true => Ok(()),
            false => Err("Message was not posted successfully").handle_error(),
        }
    }
}

fn parse_public_key(public_key: &str) -> Result<ed25519_dalek::PublicKey, String> {
    ed25519_dalek::PublicKey::from_bytes(&hex::decode(&public_key).handle_error()?).handle_error()
}

fn parse_address(address: &str) -> Result<MsgAddressInt, String> {
    MsgAddressInt::from_str(address).handle_error()
}

pub trait ToCStringPtr {
    fn to_cstring_ptr(self) -> *mut c_char;
}

impl ToCStringPtr for String {
    fn to_cstring_ptr(self) -> *mut c_char {
        CString::new(self).unwrap().into_raw()
    }
}

pub trait ToStringFromPtr {
    unsafe fn to_string_from_ptr(self) -> String;
}

impl ToStringFromPtr for *mut c_char {
    unsafe fn to_string_from_ptr(self) -> String {
        CStr::from_ptr(self).to_str().unwrap().to_owned()
    }
}

pub trait ToOptionalStringFromPtr {
    unsafe fn to_optional_string_from_ptr(self) -> Option<String>;
}

impl ToOptionalStringFromPtr for *mut c_char {
    unsafe fn to_optional_string_from_ptr(self) -> Option<String> {
        match !self.is_null() {
            true => Some(self.to_string_from_ptr()),
            false => None,
        }
    }
}
