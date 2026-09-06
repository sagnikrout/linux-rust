//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sock_diag.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_diag_handler {
    pub owner: *mut module,
    pub family: __u8,
    pub nlh): *mut *mut *mut int (dump)(struct sk_buff skb, struct nlmsghdr,
    pub sk): *mut *mut *mut int (get_info)(struct sk_buff skb, struct sock,
    pub nlh): *mut *mut *mut int (destroy)(struct sk_buff skb, struct nlmsghdr,
}

extern "C" {
    pub fn sock_diag_register(h: *const sock_diag_handler) -> c_int;
}
extern "C" {
    pub fn sock_diag_unregister(h: *const sock_diag_handler);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_diag_inet_compat {
    pub owner: *mut module,
    pub nlh): *mut *mut *mut int (fn)(struct sk_buff skb, struct nlmsghdr,
}

extern "C" {
    pub fn sock_diag_register_inet_compat(ptr: *const sock_diag_inet_compat);
}
extern "C" {
    pub fn sock_diag_unregister_inet_compat(ptr: *const sock_diag_inet_compat);
}
extern "C" {
    pub fn __sock_gen_cookie(sk: *mut sock) -> u64;
}
extern "C" {
    pub fn sock_diag_check_cookie(sk: *mut sock, cookie: *const __u32) -> c_int;
}
extern "C" {
    pub fn sock_diag_save_cookie(sk: *mut sock, cookie: *mut __u32);
}
extern "C" {
    pub fn sock_diag_put_meminfo(sk: *mut sock, skb: *mut sk_buff, attr: c_int) -> c_int;
}
extern "C" {
    pub fn sock_diag_broadcast_destroy(sk: *mut sock);
}
extern "C" {
    pub fn sock_diag_destroy(sk: *mut sock, err: c_int) -> c_int;
}
