pub mod webui;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_webui() {
        let win = webui::new_window();
        assert_eq!(win, 1);
        webui::show(win, "Hello World");
        
        // Wait 2 seconds, then webui_exit
        std::thread::spawn(|| {
            std::thread::sleep(std::time::Duration::from_secs(2));
            webui::exit();
        });
    }
}