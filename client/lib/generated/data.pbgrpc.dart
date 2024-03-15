//
//  Generated code. Do not modify.
//  source: data.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'package:protobuf/protobuf.dart' as $pb;

import 'data.pb.dart' as $0;

export 'data.pb.dart';

@$pb.GrpcServiceName('scheme.DataRequest')
class DataRequestClient extends $grpc.Client {
  static final _$getCounter = $grpc.ClientMethod<$0.CounterRequest, $0.CounterResponse>(
      '/scheme.DataRequest/GetCounter',
      ($0.CounterRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.CounterResponse.fromBuffer(value));

  DataRequestClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options,
        interceptors: interceptors);

  $grpc.ResponseFuture<$0.CounterResponse> getCounter($0.CounterRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getCounter, request, options: options);
  }
}

@$pb.GrpcServiceName('scheme.DataRequest')
abstract class DataRequestServiceBase extends $grpc.Service {
  $core.String get $name => 'scheme.DataRequest';

  DataRequestServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.CounterRequest, $0.CounterResponse>(
        'GetCounter',
        getCounter_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.CounterRequest.fromBuffer(value),
        ($0.CounterResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.CounterResponse> getCounter_Pre($grpc.ServiceCall call, $async.Future<$0.CounterRequest> request) async {
    return getCounter(call, await request);
  }

  $async.Future<$0.CounterResponse> getCounter($grpc.ServiceCall call, $0.CounterRequest request);
}
