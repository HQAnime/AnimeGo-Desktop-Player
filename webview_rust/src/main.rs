#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

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

// A simple html with a single video element
const HTML_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>AnimeGo Player</title>
    </head>
    <body style="background-color: black;">
        <!-- Video should fit the window and support fullscreem -->
        <video id="video" width="100%" height="100%" controls autoplay style="object-fit: fit;">
            <source src="|HOLDER|">
        </video>
    </body>
</html>
"#;

const JS_SCRIPT: &str = r#"
    // a wrapper to send messages to the rust side
    function send_rust(...args) {
        window.ipc.postMessage(JSON.stringify(args));
    }

    // send an event message with event:: prefix
    function send_event(event, message) {
        send_rust('event::' + event, message);
    }

    const valid_video_extensions = [".m3u8", ".ts", ".mp4", ".webm", ".mkv", ".avi", ".mov", ".wmv", ".flv", ".mpg", ".mpeg", ".m4v", ".3gp", ".3g2", ".f4v", ".f4p", ".f4a", ".f4b"];
    source_url = window.location.href;
    var has_seen_video = false;
    var has_setup = false;

    // remove iframes and check for video src
    var removed_iframe_count = 0;
    const timer_map = {};
    const timer = () => {
        if (has_seen_video) {
            clearInterval(timer_map.interval);
        }

        if (!has_setup) {
            send_rust('Setup');
            // Add a LOADING text to the center of the screen with a black background and on top of everything
            var loading_text = document.createElement("div");
            loading_text.style.position = "absolute";
            loading_text.style.top = "50%";
            loading_text.style.left = "50%";
            loading_text.style.transform = "translate(-50%, -50%)";
            loading_text.style.color = "white";
            loading_text.style.fontSize = "128px";
            loading_text.style.fontWeight = "bold";
            loading_text.style.backgroundColor = "black";
            loading_text.style.padding = "10px";
            loading_text.style.borderRadius = "10px";
            loading_text.innerHTML = "LOADING...";
            document.body.appendChild(loading_text);

            has_setup = true;
        }

        send_rust('tick');
        const iframes = document.querySelectorAll('iframe');
        iframes.forEach(function(iframe) {
            iframe.remove();
            removed_iframe_count += 1;
            send_rust('iframe removed');
        });

        const videos = document.querySelectorAll('video');
        if (videos && videos.length > 0) {
            const the_video = videos[0];
            const video_src = the_video.src;
            if (video_src != "") {
                has_seen_video = true;
                clearInterval(timer_map.interval);
                send_event('video_player', video_src);
            } else {
                send_rust("trying to play video");
                // play and pause the video to get the src
                the_video.click()
                the_video.play();
                setTimeout(() => {
                    the_video.pause();
                }, 1000);
            }
        }
    }
    timer_map.interval = setInterval(timer, 1000);

    XMLHttpRequest.prototype.orgOpen = XMLHttpRequest.prototype.open;
    XMLHttpRequest.prototype.open = function(method, url, async, user, password) {
        send_rust(method, url, async, user, password);

        // we haven't find the video yet
        if (!has_seen_video) {
            // always allow before we find the video
            send_rust("Allowed");
            this.orgOpen.apply(this, arguments);
            return;
        }

        // apply very strict rules once the video starts playing
        // check if the url is a video and allow it
        if (valid_video_extensions.some((ext) => url.includes(ext))) {
            send_rust('Streaming');
            has_seen_video = true;
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
        // links.forEach(function(link) {
        //     link.href = 'javascript:void(0)';
        // });
    });

    // detect fullscreen
    document.addEventListener('fullscreenchange', function() {
        send_event('fullscreen');
    });

    // // enable fullscreen on macOS
    // document.addEventListener('keydown', function(e) {
    //     if (e.key === 'f' && e.metaKey) {
    //         send_event('fullscreen');
    //         // adjust the video size to fit the screen
    //         const video = document.querySelector('video');
    //         if (video) {
    //             const first_video = video[0]
    //             first_video.style.width = '100%';
    //             first_video.style.height = '100%';
    //             # add -webkit-full-screen
    //             first_video.webkitEnterFullscreen();
    //         }
    //     }
    // });
"#;

// debug println macro
macro_rules! dprintln {
    ($($arg:tt)*) => {
        if IS_DEBUG {
            println!($($arg)*);
        }
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
        .with_inner_size(Size::Logical(LogicalSize::new(1280.0, 720.0)))
        .build(&event_loop)
        .expect("Failed to create window");

    let _webview = WebViewBuilder::new(window)?
        .with_url(url)?
        .with_ipc_handler(ipc_callback)
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

fn ipc_callback(window: &Window, message: String) {
    let is_events = message.contains("event::");
    if is_events {
        let event_message: Vec<String> = message
            .replace("\"", "")
            .replace("event::", "")
            .split_terminator(',')
            .map(|s| s.to_string())
            .collect();
        assert_eq!(event_message.len(), 2);
        let event_name = event_message[0].as_str();
        let event_value = &event_message[1];
        dprintln!("Event: {}", event_name);
        match event_name {
            "[fullscreen" => handle_fullscreen(window, event_value),
            "[resize_window" => handle_resize_window(window, event_value),
            "[video_player" => {
                let _ = handle_video_player(window, event_value);
            }
            _ => {}
        }
    } else {
        dprintln!("Message: {}", message);
    }
}

fn handle_fullscreen(window: &Window, _event_value: &String) {
    let fullscreen = window.fullscreen();
    if fullscreen.is_some() {
        window.set_fullscreen(None);
        dprintln!("Fullscreen disabled");
    } else {
        window.set_fullscreen(Some(Fullscreen::Borderless(window.current_monitor())));
        dprintln!("Fullscreen enabled");
    }
}

fn handle_resize_window(window: &Window, _event_value: &String) {
    window.set_min_inner_size(Some(Size::Logical(LogicalSize::new(1280.0, 720.0))));
}

fn handle_video_player(window: &Window, event_value: &String) -> wry::Result<()> {
    let video_src = event_value.replace("]", "");
    dprintln!("Video player: {}", video_src);
    let final_html = HTML_TEMPLATE.replace("|HOLDER|", &video_src);

    // create a new window to play the video
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("AnimeGo Player")
        // set a minimum size for the window
        .with_fullscreen(Some(Fullscreen::Borderless(window.current_monitor())))
        .build(&event_loop)
        .expect("Failed to create window");

    let _webview = WebViewBuilder::new(window)?
        .with_html(final_html)?
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
