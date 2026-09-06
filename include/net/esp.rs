//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/esp.h
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

// Fill padding...
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esp_info {
    pub esph: *mut ip_esp_hdr,
    pub seqno: __be64,
    pub tfclen: c_int,
    pub tailen: c_int,
    pub plen: c_int,
    pub clen: c_int,
    pub len: c_int,
    pub nfrags: c_int,
    pub proto: __u8,
    pub inplace: bool,
}

extern "C" {
    pub fn esp_output_head(x: *mut xfrm_state, skb: *mut sk_buff, esp: *mut esp_info) -> c_int;
}
extern "C" {
    pub fn esp_output_tail(x: *mut xfrm_state, skb: *mut sk_buff, esp: *mut esp_info) -> c_int;
}
extern "C" {
    pub fn esp_input_done2(skb: *mut sk_buff, err: c_int) -> c_int;
}
extern "C" {
    pub fn esp6_output_head(x: *mut xfrm_state, skb: *mut sk_buff, esp: *mut esp_info) -> c_int;
}
extern "C" {
    pub fn esp6_output_tail(x: *mut xfrm_state, skb: *mut sk_buff, esp: *mut esp_info) -> c_int;
}
extern "C" {
    pub fn esp6_input_done2(skb: *mut sk_buff, err: c_int) -> c_int;
}
