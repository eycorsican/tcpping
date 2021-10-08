use std::io;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;

use argh::FromArgs;
use socket2::{Domain, Socket, Type};

#[derive(FromArgs)]
/// TCP ping utility.
struct TcpPing {
    /// target host
    #[argh(positional)]
    host: String,
    /// target port
    #[argh(positional)]
    port: u16,
    /// ping interval (Default 1)
    #[argh(option, short = 'i', default = "1")]
    interval: u64,
    /// handshake timeout (Default 4)
    #[argh(option, short = 't', default = "4")]
    timeout: u64,
    /// stop after sending N pings
    #[argh(option, short = 'c')]
    count: Option<usize>,
    /// bound interface, this does not apply to DNS resolution (Unix only)
    #[argh(option, short = 'b')]
    boundif: Option<String>,
}

fn bind_socket(socket: &Socket, iface: Option<&String>) -> io::Result<()> {
    if let Some(iface) = iface {
        #[cfg(target_os = "macos")]
        unsafe {
            use std::ffi::CString;
            use std::os::unix::io::AsRawFd;
            let ifa = CString::new(iface.as_bytes()).unwrap();
            let ifidx: libc::c_uint = libc::if_nametoindex(ifa.as_ptr());
            if ifidx == 0 {
                return Err(io::Error::last_os_error());
            }

            // https://github.com/apple/darwin-xnu/blob/8f02f2a044b9bb1ad951987ef5bab20ec9486310/bsd/netinet/in.h#L484
            const IP_BOUND_IF: libc::c_int = 25;
            let ret = libc::setsockopt(
                socket.as_raw_fd(),
                libc::IPPROTO_IP,
                IP_BOUND_IF,
                &ifidx as *const _ as *const libc::c_void,
                std::mem::size_of::<libc::c_uint>() as libc::socklen_t,
            );
            if ret == -1 {
                return Err(io::Error::last_os_error());
            }
            return Ok(());
        }
        #[cfg(target_os = "linux")]
        unsafe {
            use std::ffi::CString;
            use std::os::unix::io::AsRawFd;
            let ifa = CString::new(iface.as_bytes()).unwrap();
            let ret = libc::setsockopt(
                socket.as_raw_fd(),
                libc::SOL_SOCKET,
                libc::SO_BINDTODEVICE,
                ifa.as_ptr() as *const libc::c_void,
                ifa.as_bytes().len() as libc::socklen_t,
            );
            if ret == -1 {
                return Err(io::Error::last_os_error());
            }
            return Ok(());
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        {
            return Err(io::Error::new(io::ErrorKind::Other, "not supported"));
        }
    }
    socket.bind(&"0.0.0.0:0".parse::<SocketAddr>().unwrap().into())
}

fn main() {
    let args: TcpPing = argh::from_env();
    let addr: SocketAddr = format!("{}:{}", args.host, args.port)
        .to_socket_addrs()
        .unwrap()
        .filter(|a| a.is_ipv4())
        .next()
        .unwrap();
    let timeout: Duration = Duration::from_secs(args.timeout);
    let mut total_pings = 0;
    loop {
        let start = std::time::Instant::now();
        let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
        if let Err(e) = bind_socket(&socket, args.boundif.as_ref()) {
            println!("Bind socket failed: {}", e);
            std::process::exit(1);
        }
        let res = socket.connect_timeout(&addr.into(), timeout);
        let elapsed = std::time::Instant::now().duration_since(start);
        match res {
            Ok(_) => {
                println!("Connected to {} in {} ms", &addr, elapsed.as_millis());
            }
            Err(e) => {
                println!("Connect to {} failed: {}", &addr, e);
            }
        }
        total_pings += 1;
        if let Some(c) = args.count {
            if total_pings >= c {
                std::process::exit(0);
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(args.interval));
    }
}
