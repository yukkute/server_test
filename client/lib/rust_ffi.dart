import "dart:ffi";
import "dart:io";

typedef StartLocalServerNative = Uint16 Function();
typedef StartLocalServerDart = int Function();

class RustBindings {
	late DynamicLibrary _lib;
	late StartLocalServerDart startLocalServer;

	RustBindings() {
		if (Platform.isLinux || Platform.isAndroid) {
			_lib = DynamicLibrary.open("libmoreonigiri_server.so");
		} else if (Platform.isWindows) {
			_lib = DynamicLibrary.open("moreonigiri_server.dll");
		}
		startLocalServer = _lib
				.lookup<NativeFunction<StartLocalServerNative>>("start_local_server")
				.asFunction();
	}
}
