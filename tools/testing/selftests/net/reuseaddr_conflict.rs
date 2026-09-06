//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/reuseaddr_conflict.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


//
// Test for the regression introduced by
//
// b9470c27607b ("inet: kill smallest_size and smallest_port")
//
// If we open an ipv4 socket on a port with reuseaddr we shouldn't reset the tb
// when we open the ipv6 conterpart, which is what was happening previously.
//

pub const PORT: c_int = 9999;
#[no_mangle]
pub unsafe extern "C" fn open_port(ipv6: c_int, any: c_int) -> c_int {
    int open_port(int ipv6, int any)
    {
    let mut fd: c_int = -1;
    let mut reuseaddr: c_int = 1;
    let mut v6only: c_int = 1;
    int addrlen;
    let mut ret: c_int = -1;
    struct sockaddr *addr;
    let mut family: c_int = ipv6 ? AF_INET6 : AF_INET;
    struct sockaddr_in6 addr6 = {
    .sin6_family = AF_INET6,
    .sin6_port = htons(PORT),
    .sin6_addr = in6addr_any
    };
    struct sockaddr_in addr4 = {
    .sin_family = AF_INET,
    .sin_port = htons(PORT),
    .sin_addr.s_addr = any ? htonl(INADDR_ANY) : inet_addr("127.0.0.1"),
    };
    if (ipv6) {
    addr = (struct sockaddr*)&addr6;
    addrlen = sizeof(addr6);
    } else {
    addr = (struct sockaddr*)&addr4;
    addrlen = sizeof(addr4);
    }
    if ((fd = socket(family, SOCK_STREAM, IPPROTO_TCP)) < 0) {
    perror("socket");
    goto out;
    }
    if (ipv6 && setsockopt(fd, IPPROTO_IPV6, IPV6_V6ONLY, (void*)&v6only,
    sizeof(v6only)) < 0) {
    perror("setsockopt IPV6_V6ONLY");
    goto out;
    }
    if (setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &reuseaddr,
    sizeof(reuseaddr)) < 0) {
    perror("setsockopt SO_REUSEADDR");
    goto out;
    }
    if (bind(fd, addr, addrlen) < 0) {
    perror("bind");
    goto out;
    }
    if (any)
    return fd;
    if (listen(fd, 1) < 0) {
    perror("listen");
    goto out;
    }
    return fd;
    out:
    close(fd);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    int listenfd;
    int fd1, fd2;
    fprintf(stderr, "Opening 127.0.0.1:%d\n", PORT);
    listenfd = open_port(0, 0);
    if (listenfd < 0)
    error(1, errno, "Couldn't open listen socket");
    fprintf(stderr, "Opening INADDR_ANY:%d\n", PORT);
    fd1 = open_port(0, 1);
    if (fd1 >= 0)
    error(1, 0, "Was allowed to create an ipv4 reuseport on a already bound non-reuseport socket");
    fprintf(stderr, "Opening in6addr_any:%d\n", PORT);
    fd1 = open_port(1, 1);
    if (fd1 < 0)
    error(1, errno, "Couldn't open ipv6 reuseport");
    fprintf(stderr, "Opening INADDR_ANY:%d\n", PORT);
    fd2 = open_port(0, 1);
    if (fd2 >= 0)
    error(1, 0, "Was allowed to create an ipv4 reuseport on a already bound non-reuseport socket");
    close(fd1);
    fprintf(stderr, "Opening INADDR_ANY:%d after closing ipv6 socket\n", PORT);
    fd1 = open_port(0, 1);
    if (fd1 >= 0)
    error(1, 0, "Was allowed to create an ipv4 reuseport on an already bound non-reuseport socket with no ipv6");
    fprintf(stderr, "Success\n");
    return 0;
    }
