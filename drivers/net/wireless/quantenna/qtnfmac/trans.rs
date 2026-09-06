//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/quantenna/qtnfmac/trans.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2015-2016 Quantenna Communications. All rights reserved.

pub const QTNF_MAX_CMD_BUF_SIZE: c_int = 2048;
pub const QTNF_DEF_CMD_HROOM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_cmd_ctl_node {
    pub cmd_resp_completion: completion,
    pub resp_skb: *mut sk_buff,
    pub seq_num: u16,
    pub waiting_for_resp: bool,
    pub /: *mut *mut spinlock_t resp_lock; / lock for resp_skb & waiting_for_resp changes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_qlink_transport {
    pub curr_cmd: qtnf_cmd_ctl_node,
    pub event_queue: sk_buff_head,
    pub event_queue_max_len: usize,
}

extern "C" {
    pub fn qtnf_trans_init(bus: *mut qtnf_bus);
}
extern "C" {
    pub fn qtnf_trans_free(bus: *mut qtnf_bus);
}
extern "C" {
    pub fn qtnf_trans_send_next_cmd(bus: *mut qtnf_bus) -> c_int;
}
extern "C" {
    pub fn qtnf_trans_handle_rx_ctl_packet(bus: *mut qtnf_bus, skb: *mut sk_buff) -> c_int;
}
