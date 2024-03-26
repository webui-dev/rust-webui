// Event struct
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct webui_event_t {
    pub window: *mut ::std::os::raw::c_void,
    pub type_: ::std::os::raw::c_uint,
    pub element: *mut ::std::os::raw::c_char,
    pub data: *mut ::std::os::raw::c_char,
    pub response: *mut ::std::os::raw::c_char,
}

extern "C" {
    #[doc = " @brief Create a new WebUI window object.\n\n @return Returns the window number.\n\n @example size_t myWindow = webui_new_window();"]
    pub fn webui_new_window() -> usize;
}
extern "C" {
    #[doc = " @brief Create a new webui window object using a specified window number.\n\n @param window_number The window number (should be > 0, and < WEBUI_MAX_IDS)\n\n @return Returns the window number.\n\n @example size_t myWindow = webui_new_window_id(123);"]
    pub fn webui_new_window_id(window_number: usize) -> usize;
}
extern "C" {
    #[doc = " @brief Get a free window number that can be used with\n `webui_new_window_id()`.\n\n @return Returns the first available free window number. Starting from 1.\n\n @example size_t myWindowNumber = webui_get_new_window_id();"]
    pub fn webui_get_new_window_id() -> usize;
}
extern "C" {
    #[doc = " @brief Bind a specific html element click event with a function. Empty\n element means all events.\n\n @param window The window number\n @param element The HTML ID\n @param func The callback function\n\n @return Returns a unique bind ID.\n\n @example webui_bind(myWindow, \"myID\", myFunction);"]
    pub fn webui_bind(
        window: usize,
        element: *const ::std::os::raw::c_char,
        func: ::std::option::Option<unsafe extern "C" fn(e: *mut webui_event_t)>,
    ) -> usize;
}
extern "C" {
    #[doc = " @brief Show a window using embedded HTML, or a file. If the window is already\n open, it will be refreshed.\n\n @param window The window number\n @param content The HTML, URL, Or a local file\n\n @return Returns True if showing the window is successed.\n\n @example webui_show(myWindow, \"<html>...</html>\"); | webui_show(myWindow,\n \"index.html\"); | webui_show(myWindow, \"http://...\");"]
    pub fn webui_show(window: usize, content: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    #[doc = " @brief Same as `webui_show()`. But using a specific web browser.\n\n @param window The window number\n @param content The HTML, Or a local file\n @param browser The web browser to be used\n\n @return Returns True if showing the window is successed.\n\n @example webui_show_browser(myWindow, \"<html>...</html>\", Chrome); |\n webui_show(myWindow, \"index.html\", Firefox);"]
    pub fn webui_show_browser(
        window: usize,
        content: *const ::std::os::raw::c_char,
        browser: usize,
    ) -> bool;
}
extern "C" {
    #[doc = " @brief Set the window in Kiosk mode (Full screen)\n\n @param window The window number\n @param status True or False\n\n @example webui_set_kiosk(myWindow, true);"]
    pub fn webui_set_kiosk(window: usize, status: bool);
}
extern "C" {
    #[doc = " @brief Wait until all opened windows get closed.\n\n @example webui_wait();"]
    pub fn webui_wait();
}
extern "C" {
    #[doc = " @brief Close a specific window only. The window object will still exist.\n\n @param window The window number\n\n @example webui_close(myWindow);"]
    pub fn webui_close(window: usize);
}
extern "C" {
    #[doc = " @brief Close a specific window and free all memory resources.\n\n @param window The window number\n\n @example webui_destroy(myWindow);"]
    pub fn webui_destroy(window: usize);
}
extern "C" {
    #[doc = " @brief Close all open windows. `webui_wait()` will return (Break).\n\n @example webui_exit();"]
    pub fn webui_exit();
}
extern "C" {
    #[doc = " @brief Set the web-server root folder path for a specific window.\n\n @param window The window number\n @param path The local folder full path\n\n @example webui_set_root_folder(myWindow, \"/home/Foo/Bar/\");"]
    pub fn webui_set_root_folder(window: usize, path: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    #[doc = " @brief Set the web-server root folder path for all windows. Should be used\n before `webui_show()`.\n\n @param path The local folder full path\n\n @example webui_set_default_root_folder(\"/home/Foo/Bar/\");"]
    pub fn webui_set_default_root_folder(path: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    #[doc = " @brief Set a custom handler to serve files.\n\n @param window The window number\n @param handler The handler function: `void myHandler(const char* filename,\n int* length)`\n\n @return Returns a unique bind ID.\n\n @example webui_set_file_handler(myWindow, myHandlerFunction);"]
    pub fn webui_set_file_handler(
        window: usize,
        handler: ::std::option::Option<
            unsafe extern "C" fn(
                filename: *const ::std::os::raw::c_char,
                length: *mut ::std::os::raw::c_int,
            ) -> *const ::std::os::raw::c_void,
        >,
    );
}
extern "C" {
    #[doc = " @brief Check if the specified window is still running.\n\n @param window The window number\n\n @example webui_is_shown(myWindow);"]
    pub fn webui_is_shown(window: usize) -> bool;
}
extern "C" {
    #[doc = " @brief Set the maximum time in seconds to wait for the browser to start.\n\n @param second The timeout in seconds\n\n @example webui_set_timeout(30);"]
    pub fn webui_set_timeout(second: usize);
}
extern "C" {
    #[doc = " @brief Set the default embedded HTML favicon.\n\n @param window The window number\n @param icon The icon as string: `<svg>...</svg>`\n @param icon_type The icon type: `image/svg+xml`\n\n @example webui_set_icon(myWindow, \"<svg>...</svg>\", \"image/svg+xml\");"]
    pub fn webui_set_icon(
        window: usize,
        icon: *const ::std::os::raw::c_char,
        icon_type: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " @brief Base64 encoding. Use this to safely send text based data to the UI. If\n it fails it will return NULL.\n\n @param str The string to encode (Should be null terminated)\n\n @example webui_encode(\"Hello\");"]
    pub fn webui_encode(str_: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " @brief Base64 decoding. Use this to safely decode received Base64 text from\n the UI. If it fails it will return NULL.\n\n @param str The string to decode (Should be null terminated)\n\n @example webui_decode(\"SGVsbG8=\");"]
    pub fn webui_decode(str_: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " @brief Safely free a buffer allocated by WebUI using `webui_malloc()`.\n\n @param ptr The buffer to be freed\n\n @example webui_free(myBuffer);"]
    pub fn webui_free(ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[doc = " @brief Safely allocate memory using the WebUI memory management system. It\n can be safely freed using `webui_free()` at any time.\n\n @param size The size of memory in bytes\n\n @example char* myBuffer = (char*)webui_malloc(1024);"]
    pub fn webui_malloc(size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " @brief Safely send raw data to the UI.\n\n @param window The window number\n @param function The JavaScript function to receive raw data: `function\n myFunc(myData){}`\n @param raw The raw data buffer\n @param size The raw data size in bytes\n\n @example webui_send_raw(myWindow, \"myJavascriptFunction\", myBuffer, 64);"]
    pub fn webui_send_raw(
        window: usize,
        function: *const ::std::os::raw::c_char,
        raw: *const ::std::os::raw::c_void,
        size: usize,
    );
}
extern "C" {
    #[doc = " @brief Set a window in hidden mode. Should be called before `webui_show()`.\n\n @param window The window number\n @param status The status: True or False\n\n @example webui_set_hide(myWindow, True);"]
    pub fn webui_set_hide(window: usize, status: bool);
}
extern "C" {
    #[doc = " @brief Set the window size.\n\n @param window The window number\n @param width The window width\n @param height The window height\n\n @example webui_set_size(myWindow, 800, 600);"]
    pub fn webui_set_size(
        window: usize,
        width: ::std::os::raw::c_uint,
        height: ::std::os::raw::c_uint,
    );
}
extern "C" {
    #[doc = " @brief Set the window position.\n\n @param window The window number\n @param x The window X\n @param y The window Y\n\n @example webui_set_position(myWindow, 100, 100);"]
    pub fn webui_set_position(window: usize, x: ::std::os::raw::c_uint, y: ::std::os::raw::c_uint);
}
extern "C" {
    #[doc = " @brief Set the web browser profile to use. An empty `name` and `path` means\n the default user profile. Need to be called before `webui_show()`.\n\n @param window The window number\n @param name The web browser profile name\n @param path The web browser profile full path\n\n @example webui_set_profile(myWindow, \"Bar\", \"/Home/Foo/Bar\"); |\n webui_set_profile(myWindow, \"\", \"\");"]
    pub fn webui_set_profile(
        window: usize,
        name: *const ::std::os::raw::c_char,
        path: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " @brief Set the web browser proxy_server to use. Need to be called before `webui_show()`.\n\n @param window The window number\n @param proxy_server The web browser proxy_server\n\n @example webui_set_proxy(myWindow, \"http://127.0.0.1:8888\");"]
    pub fn webui_set_proxy(window: usize, proxy_server: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " @brief Get the full current URL.\n\n @param window The window number\n\n @return Returns the full URL string\n\n @example const char* url = webui_get_url(myWindow);"]
    pub fn webui_get_url(window: usize) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " @brief Allow a specific window address to be accessible from a public network\n\n @param window The window number\n @param status True or False\n\n @example webui_set_public(myWindow, true);"]
    pub fn webui_set_public(window: usize, status: bool);
}
extern "C" {
    #[doc = " @brief Navigate to a specific URL\n\n @param window The window number\n @param url Full HTTP URL\n\n @example webui_navigate(myWindow, \"http://domain.com\");"]
    pub fn webui_navigate(window: usize, url: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " @brief Free all memory resources. Should be called only at the end.\n\n @example\n webui_wait();\n webui_clean();"]
    pub fn webui_clean();
}
extern "C" {
    #[doc = " @brief Delete all local web-browser profiles folder. It should called at the\n end.\n\n @example\n webui_wait();\n webui_delete_all_profiles();\n webui_clean();"]
    pub fn webui_delete_all_profiles();
}
extern "C" {
    #[doc = " @brief Delete a specific window web-browser local folder profile.\n\n @param window The window number\n\n @example\n webui_wait();\n webui_delete_profile(myWindow);\n webui_clean();\n\n @note This can break functionality of other windows if using the same\n web-browser."]
    pub fn webui_delete_profile(window: usize);
}
extern "C" {
    #[doc = " @brief Get the ID of the parent process (The web browser may re-create\n another new process).\n\n @param window The window number\n\n @return Returns the the parent process id as integer\n\n @example size_t id = webui_get_parent_process_id(myWindow);"]
    pub fn webui_get_parent_process_id(window: usize) -> usize;
}
extern "C" {
    #[doc = " @brief Get the ID of the last child process.\n\n @param window The window number\n\n @return Returns the the child process id as integer\n\n @example size_t id = webui_get_child_process_id(myWindow);"]
    pub fn webui_get_child_process_id(window: usize) -> usize;
}
extern "C" {
    #[doc = " @brief Set a custom web-server network port to be used by WebUI.\n This can be useful to determine the HTTP link of `webui.js` in case\n you are trying to use WebUI with an external web-server like NGNIX\n\n @param window The window number\n @param port The web-server network port WebUI should use\n\n @return Returns True if the port is free and usable by WebUI\n\n @example bool ret = webui_set_port(myWindow, 8080);"]
    pub fn webui_set_port(window: usize, port: usize) -> bool;
}
extern "C" {
    #[doc = " @brief Set the SSL/TLS certificate and the private key content, both in PEM\n format. This works only with `webui-2-secure` library. If set empty WebUI\n will generate a self-signed certificate.\n\n @param certificate_pem The SSL/TLS certificate content in PEM format\n @param private_key_pem The private key content in PEM format\n\n @return Returns True if the certificate and the key are valid.\n\n @example bool ret = webui_set_tls_certificate(\"-----BEGIN\n CERTIFICATE-----\\n...\", \"-----BEGIN PRIVATE KEY-----\\n...\");"]
    pub fn webui_set_tls_certificate(
        certificate_pem: *const ::std::os::raw::c_char,
        private_key_pem: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    #[doc = " @brief Run JavaScript without waiting for the response.\n\n @param window The window number\n @param script The JavaScript to be run\n\n @example webui_run(myWindow, \"alert('Hello');\");"]
    pub fn webui_run(window: usize, script: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " @brief Run JavaScript and get the response back.\n Make sure your local buffer can hold the response.\n\n @param window The window number\n @param script The JavaScript to be run\n @param timeout The execution timeout\n @param buffer The local buffer to hold the response\n @param buffer_length The local buffer size\n\n @return Returns True if there is no execution error\n\n @example bool err = webui_script(myWindow, \"return 4 + 6;\", 0, myBuffer, myBufferSize);"]
    pub fn webui_script(
        window: usize,
        script: *const ::std::os::raw::c_char,
        timeout: usize,
        buffer: *mut ::std::os::raw::c_char,
        buffer_length: usize,
    ) -> bool;
}
extern "C" {
    #[doc = " @brief Chose between Deno and Nodejs as runtime for .js and .ts files.\n\n @param window The window number\n @param runtime Deno | Nodejs\n\n @example webui_set_runtime(myWindow, Deno);"]
    pub fn webui_set_runtime(window: usize, runtime: usize);
}
extern "C" {
    #[doc = " @brief Get an argument as integer at a specific index\n\n @param e The event struct\n @param index The argument position starting from 0\n\n @return Returns argument as integer\n\n @example long long int myNum = webui_get_int_at(e, 0);"]
    pub fn webui_get_int_at(e: *mut webui_event_t, index: usize) -> ::std::os::raw::c_longlong;
}
extern "C" {
    #[doc = " @brief Get the first argument as integer\n\n @param e The event struct\n\n @return Returns argument as integer\n\n @example long long int myNum = webui_get_int(e);"]
    pub fn webui_get_int(e: *mut webui_event_t) -> ::std::os::raw::c_longlong;
}
extern "C" {
    #[doc = " @brief Get an argument as string at a specific index\n\n @param e The event struct\n @param index The argument position starting from 0\n\n @return Returns argument as string\n\n @example const char* myStr = webui_get_string_at(e, 0);"]
    pub fn webui_get_string_at(
        e: *mut webui_event_t,
        index: usize,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " @brief Get the first argument as string\n\n @param e The event struct\n\n @return Returns argument as string\n\n @example const char* myStr = webui_get_string(e);"]
    pub fn webui_get_string(e: *mut webui_event_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " @brief Get an argument as boolean at a specific index\n\n @param e The event struct\n @param index The argument position starting from 0\n\n @return Returns argument as boolean\n\n @example bool myBool = webui_get_bool_at(e, 0);"]
    pub fn webui_get_bool_at(e: *mut webui_event_t, index: usize) -> bool;
}
extern "C" {
    #[doc = " @brief Get the first argument as boolean\n\n @param e The event struct\n\n @return Returns argument as boolean\n\n @example bool myBool = webui_get_bool(e);"]
    pub fn webui_get_bool(e: *mut webui_event_t) -> bool;
}
extern "C" {
    #[doc = " @brief Get the size in bytes of an argument at a specific index\n\n @param e The event struct\n @param index The argument position starting from 0\n\n @return Returns size in bytes\n\n @example size_t argLen = webui_get_size_at(e, 0);"]
    pub fn webui_get_size_at(e: *mut webui_event_t, index: usize) -> usize;
}
extern "C" {
    #[doc = " @brief Get size in bytes of the first argument\n\n @param e The event struct\n\n @return Returns size in bytes\n\n @example size_t argLen = webui_get_size(e);"]
    pub fn webui_get_size(e: *mut webui_event_t) -> usize;
}
extern "C" {
    #[doc = " @brief Return the response to JavaScript as integer.\n\n @param e The event struct\n @param n The integer to be send to JavaScript\n\n @example webui_return_int(e, 123);"]
    pub fn webui_return_int(e: *mut webui_event_t, n: ::std::os::raw::c_longlong);
}
extern "C" {
    #[doc = " @brief Return the response to JavaScript as string.\n\n @param e The event struct\n @param n The string to be send to JavaScript\n\n @example webui_return_string(e, \"Response...\");"]
    pub fn webui_return_string(e: *mut webui_event_t, s: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " @brief Return the response to JavaScript as boolean.\n\n @param e The event struct\n @param n The boolean to be send to JavaScript\n\n @example webui_return_bool(e, true);"]
    pub fn webui_return_bool(e: *mut webui_event_t, b: bool);
}
extern "C" {
    #[doc = " @brief Bind a specific HTML element click event with a function. Empty element means all events.\n\n @param window The window number\n @param element The element ID\n @param func The callback as myFunc(Window, EventType, Element, EventNumber, BindID)\n\n @return Returns unique bind ID\n\n @example size_t id = webui_interface_bind(myWindow, \"myID\", myCallback);"]
    pub fn webui_interface_bind(
        window: usize,
        element: *const ::std::os::raw::c_char,
        func: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: usize,
                arg2: usize,
                arg3: *mut ::std::os::raw::c_char,
                arg4: usize,
                arg5: usize,
            ),
        >,
    ) -> usize;
}
extern "C" {
    #[doc = " @brief When using `webui_interface_bind()`, you may need this function to easily set a response.\n\n @param window The window number\n @param event_number The event number\n @param response The response as string to be send to JavaScript\n\n @example webui_interface_set_response(myWindow, e->event_number, \"Response...\");"]
    pub fn webui_interface_set_response(
        window: usize,
        event_number: usize,
        response: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " @brief Check if the app still running.\n\n @return Returns True if app is running\n\n @example bool status = webui_interface_is_app_running();"]
    pub fn webui_interface_is_app_running() -> bool;
}
extern "C" {
    #[doc = " @brief Get a unique window ID.\n\n @param window The window number\n\n @return Returns the unique window ID as integer\n\n @example size_t id = webui_interface_get_window_id(myWindow);"]
    pub fn webui_interface_get_window_id(window: usize) -> usize;
}
extern "C" {
    #[doc = " @brief Get an argument as string at a specific index\n\n @param window The window number\n @param event_number The event number\n @param index The argument position\n\n @return Returns argument as string\n\n @example const char* myStr = webui_interface_get_string_at(myWindow, e->event_number, 0);"]
    pub fn webui_interface_get_string_at(
        window: usize,
        event_number: usize,
        index: usize,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " @brief Get an argument as integer at a specific index\n\n @param window The window number\n @param event_number The event number\n @param index The argument position\n\n @return Returns argument as integer\n\n @example long long int myNum = webui_interface_get_int_at(myWindow, e->event_number, 0);"]
    pub fn webui_interface_get_int_at(
        window: usize,
        event_number: usize,
        index: usize,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    #[doc = " @brief Get an argument as boolean at a specific index\n\n @param window The window number\n @param event_number The event number\n @param index The argument position\n\n @return Returns argument as boolean\n\n @example bool myBool = webui_interface_get_bool_at(myWindow, e->event_number, 0);"]
    pub fn webui_interface_get_bool_at(window: usize, event_number: usize, index: usize) -> bool;
}
extern "C" {
    #[doc = " @brief Get the size in bytes of an argument at a specific index\n\n @param window The window number\n @param event_number The event number\n @param index The argument position\n\n @return Returns size in bytes\n\n @example size_t argLen = webui_interface_get_size_at(myWindow, e->event_number, 0);"]
    pub fn webui_interface_get_size_at(window: usize, event_number: usize, index: usize) -> usize;
}
