use webui_rs::bindgen::*;

fn main() {
    // Scary! The bindgen exports are interfacing directly with the C library, so no guarantees!
    unsafe {
        let win: usize = webui_new_window();
        webui_show(win, "<html>Hello World!</html>".as_ptr() as *const i8);
        webui_wait();
    }
}
