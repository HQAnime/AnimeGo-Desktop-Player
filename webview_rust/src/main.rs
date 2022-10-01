use wry::{
    application::{
        dpi::{LogicalSize, Size},
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Window, WindowBuilder},
    },
    webview::WebViewBuilder,
};

const JS_SCRIPT: &str = r#"
    function send_rust(...args) {
        window.ipc.postMessage(JSON.stringify(args));
    }

    XMLHttpRequest.prototype.orgOpen = XMLHttpRequest.prototype.open;
    var counter = 0;
    XMLHttpRequest.prototype.open = function(method, url, async, user, password) {
        send_rust([method, url, async, user, password]);
        if (counter < 5) {
            counter++;
            send_rust(counter);
            this.orgOpen.apply(this, arguments);
        }
        if (url.includes('.m3u8')) {
            send_rust('start streaming video');
            this.orgOpen.apply(this, arguments);
        };
    };

    // block popups
    window.open = function() { send_rust('Disabled popup') };

    // remove all href links to prevent going to another page
    document.addEventListener('DOMContentLoaded', function() {
        const links = document.querySelectorAll('[href]');
        links.forEach(function(link) {
            link.href = 'javascript:void(0)';
        });
    });
"#;

fn callback(_window: &Window, message: String) {
    println!("Message from JS: {}", message);
}

fn main() -> wry::Result<()> {
    // check if this is debug
    let is_debug = cfg!(debug_assertions);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("AnimeGo")
        // set a minimum size for the window
        .with_min_inner_size(Size::Logical(LogicalSize::new(800.0, 600.0)))
        .build(&event_loop)
        .expect("Failed to create window");

    let _webview = WebViewBuilder::new(window)?
        .with_url("https://gogohd.net/streaming.php?id=MTkzMTQ1&title=Jiu+Tian+Xuan+Di+Jue+2+Episode+44&typesub=SUB")?
        .with_ipc_handler(callback)
        .with_initialization_script(JS_SCRIPT)
        .with_devtools(is_debug)
        .build()
        .expect("Failed to create webview");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
