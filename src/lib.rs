pub mod webui;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webui() {
        let win = webui::new_window();
        assert_eq!(win, 1);
        webui::show(win, "<span>Hello World</span>");
        
        // Wait 2 seconds, then webui_exit
        std::thread::sleep(std::time::Duration::from_secs(2));

        webui::destroy(win);
    }
}