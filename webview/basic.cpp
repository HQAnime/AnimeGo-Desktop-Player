#include "webview.h"

#ifdef _WIN32
int WINAPI WinMain(HINSTANCE hInt, HINSTANCE hPrevInst, LPSTR lpCmdLine,
                   int nCmdShow) {
#else
int main() {
#endif
  webview::webview w(true, nullptr);
  w.set_title("AnimeGo");
  w.set_size(1920, 1080, WEBVIEW_HINT_NONE);
  // remove all video ads with autoplay
  w.init("document.addEventListener('DOMContentLoaded', function() {"
          "var autoplay = document.querySelectorAll('video[autoplay]');"
          "console.log(autoplay);"
          "for (var i = 0; i < autoplay.length; i++) {"
            "autoplay[i].remove();"
          "}"
        "});");
  // remove all popup ads
  w.init("document.addEventListener('DOMContentLoaded', function() {"
          "var popup = document.querySelectorAll('div[class^=\"popup\"]');"
          "console.log(popup);"
          "for (var i = 0; i < popup.length; i++) {"
            "popup[i].remove();"
          "}"
        "});");
  // remove devtool listener
  w.init("window.addEventListener('devtoolschange', function(event) {"
          "console.log(event);"
          "event.preventDefault();"
        "});");
  w.navigate("https://goload.io/streaming.php?id=MTkxOTcz&title=Saikin+Yatotta+Maid+ga+Ayashii+Episode+7&typesub=SUB");
  w.run();
  return 0;
}
