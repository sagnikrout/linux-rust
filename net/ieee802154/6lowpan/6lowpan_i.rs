//! Automatically rewritten from C Header to Rust Module
//! Source: net/ieee802154/6lowpan/6lowpan_i.h
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

pub type lowpan_rx_result = unsigned ;

pub const LOWPAN_DISPATCH_FRAG1: c_uint = 0xc0;
pub const LOWPAN_DISPATCH_FRAGN: c_uint = 0xe0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct frag_lowpan_compare_key {
    pub tag: u16,
    pub d_size: u16,
    pub src: ieee802154_addr,
    pub dst: ieee802154_addr,
}

// Equivalent of ipv4 struct ipq
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lowpan_frag_queue {
    pub q: inet_frag_queue,
}

extern "C" {
    pub fn lowpan_frag_rcv(skb: *mut sk_buff, frag_type: u8) -> c_int;
}
extern "C" {
    pub fn lowpan_net_frag_exit();
}
extern "C" {
    pub fn lowpan_net_frag_init() -> c_int;
}
extern "C" {
    pub fn lowpan_rx_init();
}
extern "C" {
    pub fn lowpan_rx_exit();
}
extern "C" {
    pub fn lowpan_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn lowpan_iphc_decompress(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn lowpan_rx_h_ipv6(skb: *mut sk_buff) -> lowpan_rx_result;
}
