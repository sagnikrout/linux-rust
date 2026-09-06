//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/can/rx-offload.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/can/rx-offload.h
//
// Copyright (c) 2014 David Jander, Protonic Holland
// Copyright (c) 2014-2017, 2023 Pengutronix, Marc Kleine-Budde <kernel@pengutronix.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_rx_offload {
    pub dev: *mut net_device,
    pub drop): bool,
    pub skb_queue: sk_buff_head,
    pub skb_irq_queue: sk_buff_head,
    pub skb_queue_len_max: u32,
    pub mb_first: c_uint,
    pub mb_last: c_uint,
    pub napi: napi_struct,
    pub inc: bool,
}

extern "C" {
    pub fn can_rx_offload_irq_offload_fifo(offload: *mut can_rx_offload) -> c_int;
}
extern "C" {
    pub fn can_rx_offload_irq_finish(offload: *mut can_rx_offload);
}
extern "C" {
    pub fn can_rx_offload_threaded_irq_finish(offload: *mut can_rx_offload);
}
extern "C" {
    pub fn can_rx_offload_del(offload: *mut can_rx_offload);
}
extern "C" {
    pub fn can_rx_offload_enable(offload: *mut can_rx_offload);
}
