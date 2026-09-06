//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/dynack.h
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
// Copyright (c) 2014, Lorenzo Bianconi <lorenzo.bianconi83@gmail.com>
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
pub const ATH_DYN_BUF: c_int = 64;
//
// struct ath_dyn_rxbuf - ACK frame ring buffer
// @h_rb: ring buffer head
// @t_rb: ring buffer tail
// @tstamp: ACK RX timestamp buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_dyn_rxbuf {
    pub t_rb: u16 h_rb,,
    pub tstamp: [u32; ATH_DYN_BUF],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ts_info {
    pub tstamp: u32,
    pub dur: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct haddr_pair {
    pub h_dest: [u8; ETH_ALEN],
    pub h_src: [u8; ETH_ALEN],
}

//
// struct ath_dyn_txbuf - tx frame ring buffer
// @h_rb: ring buffer head
// @t_rb: ring buffer tail
// @addr: dest/src address pair for a given TX frame
// @ts: TX frame timestamp buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_dyn_txbuf {
    pub t_rb: u16 h_rb,,
    pub addr: [haddr_pair; ATH_DYN_BUF],
    pub ts: [ts_info; ATH_DYN_BUF],
}

//
// struct ath_dynack - dynack processing info
// @enabled: enable dyn ack processing
// @ackto: current ACK timeout
// @lto: last ACK timeout computation
// @nodes: ath_node linked list
// @qlock: ts queue spinlock
// @ack_rbf: ACK ts ring buffer
// @st_rbf: status ts ring buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_dynack {
    pub enabled: bool,
    pub ackto: c_int,
    pub lto: c_ulong,
    pub nodes: list_head,
// protect timestamp queue access
    pub qlock: spinlock_t,
    pub ack_rbf: ath_dyn_rxbuf,
    pub st_rbf: ath_dyn_txbuf,
}

extern "C" {
    pub fn ath_dynack_reset(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath_dynack_node_init(ah: *mut ath_hw, an: *mut ath_node);
}
extern "C" {
    pub fn ath_dynack_node_deinit(ah: *mut ath_hw, an: *mut ath_node);
}
extern "C" {
    pub fn ath_dynack_init(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath_dynack_sample_ack_ts(ah: *mut ath_hw, skb: *mut sk_buff, ts: u32);
}

