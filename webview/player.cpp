#include "webview.h"
#include <iostream>

#ifdef _WIN32
int WINAPI WinMain(HINSTANCE hInt, HINSTANCE hPrevInst, LPSTR lpCmdLine,
                   int nCmdShow) {
#else
int main(int argc, char* argv[]) {
#endif
  if (argc < 2) {
      std::cout << "Usage: " << argv[0] << " URL" << std::endl;
      return 1;
  }
  webview::webview w(true, nullptr);
  w.set_title("AnimeGo");
  w.set_size(1280, 720, WEBVIEW_HINT_NONE);
  // send console.log to stdout
  // remove all video ads with autoplay
  // w.init("document.addEventListener('DOMContentLoaded', function() {"
  //         "var autoplay = document.querySelectorAll('iframe');"
  //         "window.logger(autoplay);"
  //         "for (var i = 0; i < autoplay.length; i++) {"
  //           "autoplay[i].remove();"
  //         "}"
  //       "});");
  // allow send method when counter is less than 10
  w.init("XMLHttpRequest.prototype.orgSend = XMLHttpRequest.prototype.send;"
          "var counter = 0;"
          "XMLHttpRequest.prototype.send = function() {"
            "window.logger('send');"
            // block future requests to block ads hopefully
            "if (counter < 4) {"
              "counter++;"
              "window.logger(counter);"
              "this.orgSend.apply(this, arguments);"
            "}"
          "};");
  w.bind("logger", [&](const std::string &msg) {
    std::cout << msg << std::endl;
    return "";
  });
  w.navigate(argv[1]);
  w.run();
  return 0;
}
