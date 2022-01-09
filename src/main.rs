extern crate nix;

use std::env;
use std::fs::File;
use std::io::Read;
use std::os::unix::io;
use std::process::exit;
use std::ptr;

use nix::libc::{
    addrinfo, c_char, c_int, close, connect, freeaddrinfo, gai_strerror, getaddrinfo, socket,
};
use std::ffi::CString;

const AF_UNSPEC: c_int = 0_i32;
const SOCK_STREAM: c_int = 1_i32;

fn main() {
    let arg = env::args().nth(1).unwrap();
    let host = CString::new(arg).unwrap();
    let service = CString::new("daytime").unwrap();
    let socket: io::RawFd = open_connect(host.as_ptr(), service.as_ptr());
    let mut f: File = unsafe { io::FromRawFd::from_raw_fd(socket) };
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("failed to read contents");
    print!("{}", contents);
}

fn open_connect(host: *const c_char, service: *const c_char) -> i32 {
    let mut hints: addrinfo = default_addrinfo();
    hints.ai_family = AF_UNSPEC;
    hints.ai_socktype = SOCK_STREAM;
    let mut res: *mut addrinfo = default_mut_addrinfo();

    let err = unsafe { getaddrinfo(host, service, &hints, &mut res) };
    if err != 0 {
        eprintln!("getaddrinfo(3): {:?}", unsafe { gai_strerror(err) });
        return 0;
    }

    while !res.is_null() {
        let sock = unsafe { socket((*res).ai_family, (*res).ai_socktype, (*res).ai_protocol) };
        if sock < 0 {
            continue;
        }
        let con = unsafe { connect(sock, (*res).ai_addr, (*res).ai_addrlen) };
        if con < 0 {
            unsafe { close(sock) };
            continue;
        }

        res = unsafe { (*res).ai_next };
        if res.is_null() {
            unsafe { freeaddrinfo(res) };
            return sock;
        }
    }

    eprintln!("socket(2)/connect(2) failed");
    unsafe { freeaddrinfo(res) };
    exit(1);
}

fn default_addrinfo() -> addrinfo {
    return addrinfo {
        ai_flags: 0i32,
        ai_family: 0i32,
        ai_socktype: 0i32,
        ai_protocol: 0i32,
        ai_addrlen: 0u32,
        ai_addr: ptr::null_mut(),
        ai_canonname: ptr::null_mut(),
        ai_next: ptr::null_mut(),
    };
}

fn default_mut_addrinfo() -> *mut addrinfo {
    return Box::new(default_addrinfo()).as_mut();
}
