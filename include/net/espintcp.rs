//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/espintcp.h
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

extern "C" {
    pub fn espintcp_init() -> void __init;
}
extern "C" {
    pub fn espintcp_push_skb(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn espintcp_queue_out(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn tcp_is_ulp_esp(sk: *mut sock) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct espintcp_msg {
    pub skb: *mut sk_buff,
    pub skmsg: sk_msg,
    pub offset: c_int,
    pub len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct espintcp_ctx {
    pub strp: strparser,
    pub ike_queue: sk_buff_head,
    pub out_queue: sk_buff_head,
    pub partial: espintcp_msg,
    pub sk): *mut *mut void (saved_data_ready)(struct sock,
    pub sk): *mut *mut void (saved_write_space)(struct sock,
    pub sk): *mut *mut void (saved_destruct)(struct sock,
    pub work: work_struct,
    pub tx_running: bool,
}

// RCU is only needed for diag
