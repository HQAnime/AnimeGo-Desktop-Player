// #![cfg_attr(
//     all(
//         not(debug_assertions), 
//         target_os = "windows"
//     ),
//     windows_subsystem = "windows"
// )]


use wry::{
    application::{
        dpi::{LogicalSize, Size},
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Fullscreen, Window, WindowBuilder},
    },
    webview::WebViewBuilder,
};

const IS_DEBUG: bool = cfg!(debug_assertions);

const JS_SCRIPT: &str = r#"
    // a wrapper to send messages to the rust side
    function send_rust(...args) {
        window.ipc.postMessage(JSON.stringify(args));
    }

    // send an event message with event:: prefix
    function send_event(event) {
        send_rust('event::' + event);
    }

    const valid_video_extensions = [".m3u8", ".ts", ".mp4", ".webm", ".mkv", ".avi", ".mov", ".wmv", ".flv", ".mpg", ".mpeg", ".m4v", ".3gp", ".3g2", ".f4v", ".f4p", ".f4a", ".f4b"];

    XMLHttpRequest.prototype.orgOpen = XMLHttpRequest.prototype.open;
    var counter = 0;
    XMLHttpRequest.prototype.open = function(method, url, async, user, password) {
        send_rust(method, url, async, user, password);
        if (counter < 5) {
            counter++;
            this.orgOpen.apply(this, arguments);
            send_rust("Allowed");
            return;
        }

        // check if the url is a video and allow it
        if (valid_video_extensions.some((ext) => url.includes(ext))) {
            send_rust('Streaming');
            this.orgOpen.apply(this, arguments);
            return;
        };

        send_rust("Blocked");
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

    // detect fullscreen
    document.addEventListener('fullscreenchange', function() {
        send_event('fullscreen');
    });
"#;

// debug println macro
macro_rules! dprintln {
    ($($arg:tt)*) => {
        if IS_DEBUG {
            println!($($arg)*);
        }
    }
}

fn callback(window: &Window, message: String) {
    let events = message.contains("event::");
    if events {
        let event = message.replace("\"", "").replace("event::", "");
        dprintln!("Event: {}", event);
        match event.as_str() {
            "[fullscreen]" => handle_fullscreen(window),
            _ => {}
        }
    } else {
        dprintln!("Message: {}", message);
    }
}

fn main() -> wry::Result<()> {
    // get arguments
    let args: Vec<String> = std::env::args().collect();
    let default_url = "about:blank".to_string();
    let url = args.get(1).unwrap_or(&default_url);
    dprintln!("URL: {}", url);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("AnimeGo")
        // set a minimum size for the window
        .with_min_inner_size(Size::Logical(LogicalSize::new(800.0, 600.0)))
        .build(&event_loop)
        .expect("Failed to create window");

    let _webview = WebViewBuilder::new(window)?
        .with_url(url)?
        .with_ipc_handler(callback)
        .with_initialization_script(JS_SCRIPT)
        .with_devtools(IS_DEBUG)
        .build()
        .expect("Failed to create webview");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => dprintln!("Webview started"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

fn handle_fullscreen(window: &Window) {
    let fullscreen = window.fullscreen();
    if fullscreen.is_some() {
        window.set_fullscreen(None);
        dprintln!("Fullscreen disabled");
    } else {
        window.set_fullscreen(Some(Fullscreen::Borderless(window.current_monitor())));
        dprintln!("Fullscreen enabled");
    }
}
