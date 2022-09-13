#include "webview.h"

#define WINDOW_WIDTH 1280
#define WINDOW_HEIGHT 800

#ifdef _WIN32
bool enterFullScreen(HWND hwnd) {
    HMONITOR hmon = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
    MONITORINFO mi = {sizeof(mi)};
    if (!GetMonitorInfo(hmon, &mi)) {
        return false;
    }
    SetWindowLong(hwnd, GWL_STYLE, WS_POPUP);
    SetWindowPos(hwnd, HWND_TOP, mi.rcMonitor.left, mi.rcMonitor.top,
                 mi.rcMonitor.right - mi.rcMonitor.left,
                 mi.rcMonitor.bottom - mi.rcMonitor.top, SWP_FRAMECHANGED);
    return true;
}
#endif

int main(int argc, char* argv[]) {
    webview::webview w(true, nullptr);
    w.set_title("AnimeGo");
    w.set_size(WINDOW_WIDTH, WINDOW_HEIGHT, WEBVIEW_HINT_NONE);

#ifdef _WIN32
    // setup console for printf
    AttachConsole(ATTACH_PARENT_PROCESS);
    freopen("CONOUT$", "w", stdout);

    // move the window to the center of the screen
    HWND hwnd = (HWND)w.window();
    int x = (GetSystemMetrics(SM_CXSCREEN) - WINDOW_WIDTH) / 2;
    int y = (GetSystemMetrics(SM_CYSCREEN) - WINDOW_HEIGHT) / 2;
    SetWindowPos(hwnd, 0, x, y, 0, 0, SWP_NOZORDER | SWP_NOSIZE);

    // hide vertical and horizontal scrollbars
    SetWindowLong(hwnd, GWL_STYLE,
                  GetWindowLong(hwnd, GWL_STYLE) & ~(WS_HSCROLL | WS_VSCROLL));
#endif

    // send console.log to stdout
    // remove all video ads with autoplay
    // w.init("document.addEventListener('DOMContentLoaded', function() {"
    //         "var autoplay = document.querySelectorAll('iframe');"
    //         "window.logger(autoplay);"
    //         "for (var i = 0; i < autoplay.length; i++) {"
    //           "autoplay[i].remove();"
    //         "}"
    //       "});");

    // disable right click
    // w.init("document.addEventListener('contextmenu', event =>
    // event.preventDefault());");

    // allow send method when counter is less than 4
    // w.init(
    //     "XMLHttpRequest.prototype.orgSend = XMLHttpRequest.prototype.send;"
    //     "var counter = 0;"
    //     "XMLHttpRequest.prototype.send = function(body) {"
    //     "window.logger(body);"
    //     "window.logger('send');"
    //     // block future requests to block ads hopefully
    //     "if (counter < 5) {"
    //     "counter++;"
    //     "window.logger(counter);"
    //     "this.orgSend.apply(this, arguments);"
    //     "}"
    //     "};"
    // );
    // override open method to not do anything
    w.init(
        "XMLHttpRequest.prototype.orgOpen = XMLHttpRequest.prototype.open;"
        "var counter = 0;"
        "XMLHttpRequest.prototype.open = function(method, url, async, user, "
        "password) {"
        "   window.logger(method, url, async, user, password);"
        "   if (counter < 5) {"
        "       counter++;"
        "       window.logger(counter);"
        "       this.orgOpen.apply(this, arguments);"
        "   }"
        "   if (url.includes('.m3u8')) {"
        "       window.logger('start streaming video');"
        "       this.orgOpen.apply(this, arguments);"
        "   };"
        "};");

    // detect fullscreen
    w.init(
        "var fullscreen = false;"
        "document.addEventListener('fullscreenchange', function() {"
        "fullscreen = !fullscreen;"
        "window.callback('fullscreen');"
        "});");

    // override window.open method to not do anything
    w.init("window.open = function() { window.logger('Disabled popup') };");
    // remove all href links to prevent going to another page
    w.init(
        "document.addEventListener('DOMContentLoaded', function() {"
        "   const links = document.querySelectorAll('[href]');"
        "   links.forEach(function(link) {"
        "       link.href = 'javascript:void(0)';"
        "    });"
        "});");

    w.bind("logger", [](const std::string& msg) {
        printf("log: %s\n", msg.c_str());
        return "";
    });

#ifdef _WIN32
    w.bind("callback", [&w](const std::string& msg) {
        printf("callback: %s\n", msg.c_str());
        if (msg == "[\"fullscreen\"]") {
            printf("enter fullscreen\n");
            // toggle fullscreen or disable it using win32
            HWND hwnd = (HWND)w.window();
            enterFullScreen(hwnd);
        }

        return "";
    });
#endif

    if (argc < 2) {
        // load the usage so the program will still run
        w.set_html("<center><h1>Usage: player &lt;url&gt;</h1></center>");
    } else {
        w.navigate(argv[1]);
    }

    w.run();
    return 0;
}
