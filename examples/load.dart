// This is free and unencumbered software released into the public domain.

import 'dart:ffi' as ffi;
//import 'dart:isolate';

typedef Dart_InitializeApiDL_t = ffi.Int64 Function(ffi.Pointer<ffi.Void>);

final dylib = ffi.DynamicLibrary.open('libdart_port.dylib');

final int Function(ffi.Pointer<ffi.Void>) Dart_InitializeApiDL = dylib.lookup<ffi.NativeFunction<Dart_InitializeApiDL_t>>('DartPort_InitializeApiDL').asFunction();

void main() {
  // See: https://api.dart.dev/stable/3.3.3/dart-ffi/NativeApi/initializeApiDLData.html
  print(Dart_InitializeApiDL);
  print(Dart_InitializeApiDL(ffi.NativeApi.initializeApiDLData));
}
