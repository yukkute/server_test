//
//  Generated code. Do not modify.
//  source: data.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class CounterRequest extends $pb.GeneratedMessage {
  factory CounterRequest({
    $core.int? version,
  }) {
    final $result = create();
    if (version != null) {
      $result.version = version;
    }
    return $result;
  }
  CounterRequest._() : super();
  factory CounterRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CounterRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CounterRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'scheme'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'version', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CounterRequest clone() => CounterRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CounterRequest copyWith(void Function(CounterRequest) updates) => super.copyWith((message) => updates(message as CounterRequest)) as CounterRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CounterRequest create() => CounterRequest._();
  CounterRequest createEmptyInstance() => create();
  static $pb.PbList<CounterRequest> createRepeated() => $pb.PbList<CounterRequest>();
  @$core.pragma('dart2js:noInline')
  static CounterRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CounterRequest>(create);
  static CounterRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get version => $_getIZ(0);
  @$pb.TagNumber(1)
  set version($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasVersion() => $_has(0);
  @$pb.TagNumber(1)
  void clearVersion() => clearField(1);
}

class CounterResponse extends $pb.GeneratedMessage {
  factory CounterResponse({
    $core.int? counter,
  }) {
    final $result = create();
    if (counter != null) {
      $result.counter = counter;
    }
    return $result;
  }
  CounterResponse._() : super();
  factory CounterResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CounterResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CounterResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'scheme'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'counter', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CounterResponse clone() => CounterResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CounterResponse copyWith(void Function(CounterResponse) updates) => super.copyWith((message) => updates(message as CounterResponse)) as CounterResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CounterResponse create() => CounterResponse._();
  CounterResponse createEmptyInstance() => create();
  static $pb.PbList<CounterResponse> createRepeated() => $pb.PbList<CounterResponse>();
  @$core.pragma('dart2js:noInline')
  static CounterResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CounterResponse>(create);
  static CounterResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get counter => $_getIZ(0);
  @$pb.TagNumber(1)
  set counter($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasCounter() => $_has(0);
  @$pb.TagNumber(1)
  void clearCounter() => clearField(1);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
