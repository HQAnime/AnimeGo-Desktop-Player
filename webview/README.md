# WebView
```sh
# Linux
g++ basic.cc -std=c++11 -Ilibs/webview $(pkg-config --cflags --libs gtk+-3.0 webkit2gtk-4.0) && ./a.out
# macOS
g++ basic.cpp -std=c++11 -framework WebKit && ./a.out
# Windows/MinGW
g++ basic.cpp -std=c++17 -mwindows -Ilibs/webview -Ilibs/webview2/build/native/include -Llibs/webview2/build/native/x64 -lWebView2Loader.dll -lole32 -lshell32 -lshlwapi -luser32 && ./a.exe
```

https://github.com/webview/webview
