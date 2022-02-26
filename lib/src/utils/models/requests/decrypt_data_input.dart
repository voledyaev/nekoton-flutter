import 'package:freezed_annotation/freezed_annotation.dart';

import '../common/encrypted_data.dart';

part 'decrypt_data_input.freezed.dart';
part 'decrypt_data_input.g.dart';

@freezed
class DecryptDataInput with _$DecryptDataInput {
  @JsonSerializable(explicitToJson: true)
  const factory DecryptDataInput({
    required EncryptedData encryptedData,
  }) = _DecryptDataInput;

  factory DecryptDataInput.fromJson(Map<String, dynamic> json) => _$DecryptDataInputFromJson(json);
}