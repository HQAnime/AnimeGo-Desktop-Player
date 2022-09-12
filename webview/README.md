# WebView
```sh
# Linux
g++ player.cpp -std=c++11 -Ilibs/webview $(pkg-config --cflags --libs gtk+-3.0 webkit2gtk-4.0) && ./a.out
# macOS
g++ player.cpp -std=c++11 -framework WebKit && ./a.out
# Windows/MinGW
g++ player.cpp -std=c++17 -mwindows -I"libs/webview2/build/native/include" -L"libs/webview2/build/native/x64" -l"WebView2Loader.dll" -lole32 -lshell32 -lshlwapi -luser32
```

https://github.com/webview/webview

* For Windows, you may need the latest compiler to support C++17.
