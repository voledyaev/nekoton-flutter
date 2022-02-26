import 'package:freezed_annotation/freezed_annotation.dart';

import 'tokens_object.dart';

part 'function_call.freezed.dart';
part 'function_call.g.dart';

@freezed
class FunctionCall with _$FunctionCall {
  const factory FunctionCall({
    required String abi,
    required String method,
    required TokensObject params,
  }) = _FunctionCall;

  factory FunctionCall.fromJson(Map<String, dynamic> json) => _$FunctionCallFromJson(json);
}