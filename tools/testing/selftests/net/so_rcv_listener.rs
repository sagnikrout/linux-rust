//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/so_rcv_listener.c
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


// SPDX-License-Identifier: GPL-2.0

pub const SO_RCVPRIORITY: c_int = 82;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct options {
    pub val: __u32,
    pub name: c_int,
    pub rcvname: c_int,
    pub host: *const c_char,
    pub service: *const c_char,
    pub opt: },
#[no_mangle]
unsafe extern "C" fn __attribute__(bin: *const (noreturn)) usage(char) {
    static void __attribute__((noreturn)) usage(const char *bin)
    {
    pub bin): printf("Usage: %s [opts] <dst host> <dst port / service>\n",,
    printf("Options:\n"
    "\t\t-M val  Test SO_RCVMARK\n"
    "\t\t-P val  Test SO_RCVPRIORITY\n"
    }
#[no_mangle]
unsafe extern "C" fn parse_args(argc: c_int, argv[]: *mut c_char) {
    static void parse_args(int argc, char *argv[])
    {
    pub o: c_int,
    while ((o = getopt(argc, argv, "M:P:")) != -1) {
    switch (o) {
    case 'M':
    pub atoi(optarg): opt.val =,
    pub SO_MARK: opt.name =,
    pub SO_RCVMARK: opt.rcvname =,
    case 'P':
    pub atoi(optarg): opt.val =,
    pub SO_PRIORITY: opt.name =,
    pub SO_RCVPRIORITY: opt.rcvname =,
    default:
    }
    }
    if (optind != argc - 2)
    pub argv: [opt.host =; optind],
    pub 1]: opt.service = argv[optind +,
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    pub 0: int err =,
    pub -1: int recv_fd =,
    pub 0: int ret_value =,
    pub recv_val: __u32,
    pub cmsg: *mut cmsghdr,
    pub cbuf: [c_char; CMSG_SPACE(sizeof(__u32))],
    pub recv_buf: [c_char; CMSG_SPACE(sizeof(__u32))],
    pub iov: [iovec; 1],
    pub msg: msghdr,
    pub recv_addr4: sockaddr_in,
    pub recv_addr6: sockaddr_in6,
    pub argv): parse_args(argc,,
    pub AF_INET: int family = strchr(opt.host, ':') ? AF_INET6 :,
    pub IPPROTO_UDP): recv_fd = socket(family, SOCK_DGRAM,,
    if (recv_fd < 0) {
    pub socket"): perror("Can't open recv,
    pub -errno: ret_value =,
    pub cleanup: goto,
    }
    pub sizeof(opt.val)): err = setsockopt(recv_fd, SOL_SOCKET, opt.rcvname, &opt.val,,
    if (err < 0) {
    pub error"): perror("Recv setsockopt,
    pub -errno: ret_value =,
    pub cleanup: goto,
    }
    if (family == AF_INET) {
    pub sizeof(recv_addr4)): memset(&recv_addr4, 0,,
    pub family: recv_addr4.sin_family =,
    pub htons(atoi(opt.service)): recv_addr4.sin_port =,
    if (inet_pton(family, opt.host, &recv_addr4.sin_addr) <= 0) {
    pub address"): perror("Invalid IPV4,
    pub -errno: ret_value =,
    pub cleanup: goto,
    }
    pub sizeof(recv_addr4)): *mut *mut err = bind(recv_fd, (struct sockaddr )&recv_addr4,,
    } else {
    pub sizeof(recv_addr6)): memset(&recv_addr6, 0,,
    pub family: recv_addr6.sin6_family =,
    pub htons(atoi(opt.service)): recv_addr6.sin6_port =,
    if (inet_pton(family, opt.host, &recv_addr6.sin6_addr) <= 0) {
    pub address"): perror("Invalid IPV6,
    pub -errno: ret_value =,
    pub cleanup: goto,
    }
    pub sizeof(recv_addr6)): *mut *mut err = bind(recv_fd, (struct sockaddr )&recv_addr6,,
    }
    if (err < 0) {
    pub error"): perror("Recv bind,
    pub -errno: ret_value =,
    pub cleanup: goto,
    }
    pub recv_buf: iov[0].iov_base =,
    pub sizeof(recv_buf): iov[0].iov_len =,
    pub sizeof(msg)): memset(&msg, 0,,
    pub iov: msg.msg_iov =,
    pub 1: msg.msg_iovlen =,
    pub cbuf: msg.msg_control =,
    pub sizeof(cbuf): msg.msg_controllen =,
    pub 0): err = recvmsg(recv_fd, &msg,,
    if (err < 0) {
    pub error"): perror("Message receive,
    pub -errno: ret_value =,
    pub cleanup: goto,
    }
    pub {: for (cmsg = CMSG_FIRSTHDR(&msg); cmsg != NULL; cmsg = CMSG_NXTHDR(&msg, cmsg)),
    if (cmsg.cmsg_level == SOL_SOCKET && cmsg.cmsg_type == opt.name) {
    pub )CMSG_DATA(cmsg): *mut *mut recv_val = (__u32,
    pub recv_val): printf("Received value: %u\n",,
    if (recv_val != opt.val) {
    fprintf(stderr, "Error: expected value: %u, got: %u\n",
    pub recv_val): opt.val,,
    pub -EINVAL: ret_value =,
    }
    pub cleanup: goto,
    }
    }
    pub received\n"): fprintf(stderr, "Error: No matching cmsg,
    pub -ENOMSG: ret_value =,
    cleanup:
    if (recv_fd >= 0)
    pub ret_value: return,
    }
