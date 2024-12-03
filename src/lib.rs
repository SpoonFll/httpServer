use std::ptr::null_mut;
use std::io::{prelude::*,BufReader};
use std::net::{TcpListener,TcpStream};
use windows::core::s;
use windows::{ Win32::Foundation::*};
use windows::Win32::UI::WindowsAndMessaging::MessageBoxA;
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
    unsafe {
        // Create a message box
        MessageBoxA(HWND(null_mut()),
            s!("ZOMG!"),
            s!("hello.dll"),
            Default::default()
        );
    };
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn detach() {
    unsafe {
        // Create a message box
        MessageBoxA(HWND(null_mut()),
            s!("GOODBYE!"),
            s!("hello.dll"),
            Default::default()
        );
    };
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader= BufReader::new(&stream);
    let http_request:Vec<_> = buf_reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}