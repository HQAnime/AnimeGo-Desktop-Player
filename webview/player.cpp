#include "webview.h"

#define WINDOW_WIDTH 1280
#define WINDOW_HEIGHT 720

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

    // override window.open method to not do anything
    w.init("window.open = function() {};");
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
        "window.logger(method, url, async, user, password);"
        "if (counter < 5) {"
        "counter++;"
        "window.logger(counter);"
        "this.orgOpen.apply(this, arguments);"
        "}"
        "if (url.includes('.m3u8')) {"
        "this.orgOpen.apply(this, arguments);"
        "};");
    w.bind("logger", [&](const std::string& msg) {
        printf("log: %s\n", msg.c_str());
        return "";
    });

    if (argc < 2) {
        // load the usage so the program will still run
        w.set_html("<center><h1>Usage: player.exe &lt;url&gt;</h1></center>");
    } else {
        w.navigate(argv[1]);
    }

    w.run();
    return 0;
}
