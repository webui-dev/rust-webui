pub mod webui;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webui_window() {
        let win = webui::Window::new();
        assert_eq!(win.id, 1);
        win.show("<span>Hello World</span>");

        // Wait 2 seconds, then kill
        std::thread::sleep(std::time::Duration::from_secs(2));

        win.destroy();
    }
}