const JS_SCRIPT: &str = r#"
    XMLHttpRequest.prototype.orgSend = XMLHttpRequest.prototype.send;
    var counter = 0;
    XMLHttpRequest.prototype.send = function() {
        window.logger('send');
        // block future requests to block ads hopefully
        if (counter < 4) {
            counter++;
            window.logger(counter);
            this.orgSend.apply(this, arguments);
        }
    };
"#;

fn main() -> wry::Result<()> {
    use wry::{
        application::{
            dpi::{LogicalSize, Size},
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
        },
        webview::WebViewBuilder,
    };

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("AnimeGo")
        // set a minimum size for the window
        .with_min_inner_size(Size::Logical(LogicalSize::new(800.0, 600.0)))
        .build(&event_loop)
        .expect("Failed to create window");
    let _webview = WebViewBuilder::new(window)?
        .with_url("https://tauri.studio")?
        .with_initialization_script(JS_SCRIPT)
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
