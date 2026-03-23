// WebUI Rust - Public Network Access Example
//
// Opens two windows:
//   - A private window (localhost only) that shows the public window's URLs and connection logs.
//   - A public window (network accessible) that any device on the LAN can open.

use std::sync::atomic::{AtomicUsize, Ordering};
use webui_rs::webui;
use webui_rs::webui::{Event, WebUIBrowser, WebUIEvent, WebUIConfig};

static PRIVATE_WIN: AtomicUsize = AtomicUsize::new(0);
static PUBLIC_WIN: AtomicUsize = AtomicUsize::new(0);

const PRIVATE_HTML: &str = r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <script src="webui.js"></script>
    <title>Public Network Access Example</title>
    <style>
      body {
        font-family: 'Arial', sans-serif;
        color: white;
        background: linear-gradient(to right, #507d91, #1c596f, #022737);
        text-align: center;
        font-size: 18px;
      }
      button, textarea {
        padding: 10px;
        margin: 10px;
        border-radius: 3px;
        border: 1px solid #ccc;
        box-shadow: 0 3px 5px rgba(0,0,0,0.1);
        transition: 0.2s;
      }
      button {
        background: #3498db;
        color: #fff;
        cursor: pointer;
        font-size: 16px;
      }
      h1 { text-shadow: -7px 10px 7px rgb(67 57 57 / 76%); }
      button:hover { background: #c9913d; }
    </style>
  </head>
  <body>
    <h1>WebUI - Public Network Access Example</h1>
    <br>
    The public window is accessible from any device on the network.<br>
    <br>
    Public window links:<br>
    <h1 id="urlSpan1" style="color:#c9913d">...</h1>
    <h1 id="urlSpan2" style="color:#c9913d">...</h1>
    <h1 id="urlSpan3" style="color:#c9913d">...</h1>
    Public window events:<br>
    <textarea id="Logs" rows="4" cols="50" style="width:60%"></textarea>
    <br>
    <button id="Exit">Exit</button>
  </body>
</html>"#;

const PUBLIC_HTML: &str = r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <script src="webui.js"></script>
    <title>Welcome to Public UI</title>
  </head>
  <body>
    <h1>Welcome to Public UI!</h1>
  </body>
</html>"#;

fn app_exit(_e: Event) {
    webui::exit();
}

fn public_window_events(e: Event) {
    let private = PRIVATE_WIN.load(Ordering::SeqCst);
    match e.event_type {
        WebUIEvent::WebUiEventConnected => {
            webui::run(private, "document.getElementById('Logs').value += 'New connection.\\n';");
        }
        WebUIEvent::WebUiEventDisconnected => {
            webui::run(private, "document.getElementById('Logs').value += 'Disconnected.\\n';");
        }
        _ => {}
    }
}

fn private_window_events(e: Event) {
    if let WebUIEvent::WebUiEventConnected = e.event_type {
        let private = PRIVATE_WIN.load(Ordering::SeqCst);
        let public = PUBLIC_WIN.load(Ordering::SeqCst);

        let port = webui::get_port(public);
        let url = webui::get_url(public);

        webui::run(private, format!(
            "document.getElementById('urlSpan1').innerHTML = 'http://localhost:{port}';"
        ));
        webui::run(private, format!(
            "document.getElementById('urlSpan2').innerHTML = '{url}';"
        ));
        webui::run(private, format!(
            "document.getElementById('urlSpan3').innerHTML = 'http://[ANY_IP_OF_THIS_MACHINE]:{port}';"
        ));
    }
}

fn main() {

    // Allow multiple connections to the same window
    webui::set_config(WebUIConfig::MultiClient, true);

    // Create both windows before binding (creating a second window resets the callback table)
    let private = webui::Window::new();
    let public = webui::Window::new();

    PRIVATE_WIN.store(private.id, Ordering::SeqCst);
    PUBLIC_WIN.store(public.id, Ordering::SeqCst);

    webui::set_timeout(0); // Wait forever

    // Public window: network-accessible, no browser, custom port
    public.set_public(true);
    public.bind("", public_window_events); // bind all events
    public.set_port(9000);
    public.show_browser(PUBLIC_HTML, WebUIBrowser::NoBrowser);

    // Private window: localhost only
    private.bind("", private_window_events);
    private.bind("Exit", app_exit);
    private.show(PRIVATE_HTML);

    webui::wait();
    webui::clean();
}
