use std::ptr::null_mut;
use std::io::{prelude::*,BufReader};
use std::net::{TcpListener,TcpStream};
use windows::core::{s, PCSTR};
use windows::{ Win32::Foundation::*};
use windows::Win32::UI::WindowsAndMessaging::MessageBoxA;
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH,DLL_PROCESS_DETACH};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    _: *mut ())
    -> bool
{
    
    match call_reason {
        DLL_PROCESS_ATTACH => attach(),
        DLL_PROCESS_DETACH => detach(),
        _ => ()
    }

    true
}

fn attach() {
    open_msg_box("attached\0");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream)=>stream,
            Err(e)=>{
                let mut msg = e.to_string();
                msg.push('\0');
                open_msg_box(msg.as_str());
                panic!("failure at {e}");
        },
        };
        handle_connection(stream);
    }
}

fn detach() {
    open_msg_box("detached\0");
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader= BufReader::new(&stream);
    let http_request:Vec<_> = buf_reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();
    //let status_line = match 
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    open_msg_box("response detected\0");
    stream.write_all(response.as_bytes()).unwrap();
}

fn open_msg_box(message:&str){
    unsafe {
        // Create a message box
        MessageBoxA(HWND(null_mut()),
            PCSTR(message.as_ptr()),
            s!("hello.dll"),
            Default::default()
        );
    };
}