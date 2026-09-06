//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/rsi_91x.h
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
// Copyright (c) 2017 Redpine Signals Inc.
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

// HAL queue information
pub const RSI_COEX_Q: c_uint = 0x0;
pub const RSI_BT_Q: c_uint = 0x2;
pub const RSI_WLAN_Q: c_uint = 0x3;
pub const RSI_WIFI_MGMT_Q: c_uint = 0x4;
pub const RSI_WIFI_DATA_Q: c_uint = 0x5;
pub const RSI_BT_MGMT_Q: c_uint = 0x6;
pub const RSI_BT_DATA_Q: c_uint = 0x7;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rsi_coex_queues {
    RSI_COEX_Q_INVALID = -1,
    RSI_COEX_Q_COMMON = 0,
    RSI_COEX_Q_BT,
    RSI_COEX_Q_WLAN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rsi_host_intf {
    RSI_HOST_INTF_SDIO = 0,
    RSI_HOST_INTF_USB
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_proto_ops {
    pub hal_queue): *mut *mut *mut *mut int (coex_send_pkt)(void priv, struct sk_buff skb, u8,
    pub priv): *mut *mut rsi_host_intf (get_host_intf)(void,
    pub context): *mut *mut *mut void (set_bt_context)(void priv, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_mod_ops {
    pub ops): *mut *mut *mut int (attach)(void priv, struct rsi_proto_ops,
    pub priv): *mut *mut void (detach)(void,
    pub msg): *const *const *const int (recv_pkt)(void priv, u8,
}
