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

import 'data.pb.dart' as $1;
import 'google/protobuf/empty.pb.dart' as $0;

export 'data.pb.dart';

@$pb.GrpcServiceName('scheme.MoreOnigiriServices')
class MoreOnigiriServicesClient extends $grpc.Client {
  static final _$sendPing = $grpc.ClientMethod<$0.Empty, $1.Pong>(
      '/scheme.MoreOnigiriServices/SendPing',
      ($0.Empty value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $1.Pong.fromBuffer(value));
  static final _$getData = $grpc.ClientMethod<$1.DataRequest, $1.DataResponse>(
      '/scheme.MoreOnigiriServices/GetData',
      ($1.DataRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $1.DataResponse.fromBuffer(value));

  MoreOnigiriServicesClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options, interceptors: interceptors);

  $grpc.ResponseFuture<$1.Pong> sendPing($0.Empty request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$sendPing, request, options: options);
  }

  $grpc.ResponseStream<$1.DataResponse> getData($1.DataRequest request,
      {$grpc.CallOptions? options}) {
    return $createStreamingCall(
        _$getData, $async.Stream.fromIterable([request]),
        options: options);
  }
}

@$pb.GrpcServiceName('scheme.MoreOnigiriServices')
abstract class MoreOnigiriServicesServiceBase extends $grpc.Service {
  $core.String get $name => 'scheme.MoreOnigiriServices';

  MoreOnigiriServicesServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.Empty, $1.Pong>(
        'SendPing',
        sendPing_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.Empty.fromBuffer(value),
        ($1.Pong value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.DataRequest, $1.DataResponse>(
        'GetData',
        getData_Pre,
        false,
        true,
        ($core.List<$core.int> value) => $1.DataRequest.fromBuffer(value),
        ($1.DataResponse value) => value.writeToBuffer()));
  }

  $async.Future<$1.Pong> sendPing_Pre(
      $grpc.ServiceCall call, $async.Future<$0.Empty> request) async {
    return sendPing(call, await request);
  }

  $async.Stream<$1.DataResponse> getData_Pre(
      $grpc.ServiceCall call, $async.Future<$1.DataRequest> request) async* {
    yield* getData(call, await request);
  }

  $async.Future<$1.Pong> sendPing($grpc.ServiceCall call, $0.Empty request);
  $async.Stream<$1.DataResponse> getData(
      $grpc.ServiceCall call, $1.DataRequest request);
}
