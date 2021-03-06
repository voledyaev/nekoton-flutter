import 'dart:convert';
import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:nekoton_flutter/src/bindings.dart';
import 'package:nekoton_flutter/src/ffi_utils.dart';
import 'package:nekoton_flutter/src/helpers/abi/models/decoded_output.dart';
import 'package:nekoton_flutter/src/helpers/abi/models/method_name.dart';

DecodedOutput? decodeOutput({
  required String messageBody,
  required String contractAbi,
  required MethodName method,
}) {
  final methodStr = jsonEncode(method);

  final result = executeSync(
    () => NekotonFlutter.instance().bindings.nt_decode_output(
          messageBody.toNativeUtf8().cast<Char>(),
          contractAbi.toNativeUtf8().cast<Char>(),
          methodStr.toNativeUtf8().cast<Char>(),
        ),
  );

  final json = result != null ? result as Map<String, dynamic> : null;
  final decodedOutput = json != null ? DecodedOutput.fromJson(json) : null;

  return decodedOutput;
}
