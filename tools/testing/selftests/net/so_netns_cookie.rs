//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/so_netns_cookie.c
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
// Macro flag: #define _GNU_SOURCE

pub const SO_NETNS_COOKIE: c_int = 71;

    ({ \
    fprintf(stderr, "%s:%d:" fmt ": %m\n", \
    __func__, __LINE__, ##__VA_ARGS__); \
    1; \
    })
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argvp[]: *mut c_char) -> c_int {
    int main(int argc, char *argvp[])
    {
    uint64_t cookie1, cookie2;
    socklen_t vallen;
    int sock1, sock2;
    sock1 = socket(AF_INET, SOCK_STREAM, 0);
    if (sock1 < 0)
    return pr_err("Unable to create TCP socket");
    vallen = sizeof(cookie1);
    if (getsockopt(sock1, SOL_SOCKET, SO_NETNS_COOKIE, &cookie1, &vallen) != 0)
    return pr_err("getsockopt(SOL_SOCKET, SO_NETNS_COOKIE)");
    if (!cookie1)
    return pr_err("SO_NETNS_COOKIE returned zero cookie");
    if (unshare(CLONE_NEWNET))
    return pr_err("unshare");
    sock2 = socket(AF_INET, SOCK_STREAM, 0);
    if (sock2 < 0)
    return pr_err("Unable to create TCP socket");
    vallen = sizeof(cookie2);
    if (getsockopt(sock2, SOL_SOCKET, SO_NETNS_COOKIE, &cookie2, &vallen) != 0)
    return pr_err("getsockopt(SOL_SOCKET, SO_NETNS_COOKIE)");
    if (!cookie2)
    return pr_err("SO_NETNS_COOKIE returned zero cookie");
    if (cookie1 == cookie2)
    return pr_err("SO_NETNS_COOKIE returned identical cookies for distinct ns");
    close(sock1);
    close(sock2);
    return 0;
    }
