//
//  Generated code. Do not modify.
//  source: data.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use pongDescriptor instead')
const Pong$json = {
  '1': 'Pong',
  '2': [
    {'1': 'port', '3': 1, '4': 1, '5': 9, '10': 'port'},
  ],
};

/// Descriptor for `Pong`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List pongDescriptor = $convert.base64Decode(
    'CgRQb25nEhIKBHBvcnQYASABKAlSBHBvcnQ=');

@$core.Deprecated('Use dataRequestDescriptor instead')
const DataRequest$json = {
  '1': 'DataRequest',
  '2': [
    {'1': 'version', '3': 1, '4': 1, '5': 13, '10': 'version'},
  ],
};

/// Descriptor for `DataRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataRequestDescriptor = $convert.base64Decode(
    'CgtEYXRhUmVxdWVzdBIYCgd2ZXJzaW9uGAEgASgNUgd2ZXJzaW9u');

@$core.Deprecated('Use dataResponseDescriptor instead')
const DataResponse$json = {
  '1': 'DataResponse',
  '2': [
    {'1': 'counter', '3': 1, '4': 1, '5': 13, '10': 'counter'},
  ],
};

/// Descriptor for `DataResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataResponseDescriptor = $convert.base64Decode(
    'CgxEYXRhUmVzcG9uc2USGAoHY291bnRlchgBIAEoDVIHY291bnRlcg==');

