//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/rsi/rsi_coex.h
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
// Copyright (c) 2018 Redpine Signals Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

pub const COMMON_CARD_READY_IND: c_int = 0;
pub const NUM_COEX_TX_QUEUES: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_coex_ctrl_block {
    pub priv: *mut rsi_common,
    pub coex_tx_qs: [sk_buff_head; NUM_COEX_TX_QUEUES],
    pub coex_tx_thread: rsi_thread,
}

extern "C" {
    pub fn rsi_coex_attach(common: *mut rsi_common) -> c_int;
}
extern "C" {
    pub fn rsi_coex_detach(common: *mut rsi_common);
}
extern "C" {
    pub fn rsi_coex_send_pkt(priv: *mut c_void, skb: *mut sk_buff, proto_type: u8) -> c_int;
}
extern "C" {
    pub fn rsi_coex_recv_pkt(common: *mut rsi_common, msg: *mut u8) -> c_int;
}

