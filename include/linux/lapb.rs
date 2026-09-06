//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/lapb.h
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
//
// These are the public elements of the Linux LAPB module.
//

pub const LAPB_OK: c_int = 0;
pub const LAPB_BADTOKEN: c_int = 1;
pub const LAPB_INVALUE: c_int = 2;
pub const LAPB_CONNECTED: c_int = 3;
pub const LAPB_NOTCONNECTED: c_int = 4;
pub const LAPB_REFUSED: c_int = 5;
pub const LAPB_TIMEDOUT: c_int = 6;
pub const LAPB_NOMEM: c_int = 7;
pub const LAPB_STANDARD: c_uint = 0x00;
pub const LAPB_EXTENDED: c_uint = 0x01;
pub const LAPB_SLP: c_uint = 0x00;
pub const LAPB_MLP: c_uint = 0x02;
pub const LAPB_DTE: c_uint = 0x00;
pub const LAPB_DCE: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lapb_register_struct {
    pub reason): *mut *mut *mut void (connect_confirmation)(struct net_device dev, int,
    pub reason): *mut *mut *mut void (connect_indication)(struct net_device dev, int,
    pub reason): *mut *mut *mut void (disconnect_confirmation)(struct net_device dev, int,
    pub reason): *mut *mut *mut void (disconnect_indication)(struct net_device dev, int,
    pub skb): *mut *mut *mut int (data_indication)(struct net_device dev, struct sk_buff,
    pub skb): *mut *mut *mut void (data_transmit)(struct net_device dev, struct sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lapb_parms_struct {
    pub t1: c_uint,
    pub t1timer: c_uint,
    pub t2: c_uint,
    pub t2timer: c_uint,
    pub n2: c_uint,
    pub n2count: c_uint,
    pub window: c_uint,
    pub state: c_uint,
    pub mode: c_uint,
}

extern "C" {
    pub fn lapb_unregister(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn lapb_getparms(dev: *mut net_device, parms: *mut lapb_parms_struct) -> c_int;
}
extern "C" {
    pub fn lapb_setparms(dev: *mut net_device, parms: *mut lapb_parms_struct) -> c_int;
}
extern "C" {
    pub fn lapb_connect_request(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn lapb_disconnect_request(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn lapb_data_request(dev: *mut net_device, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn lapb_data_received(dev: *mut net_device, skb: *mut sk_buff) -> c_int;
}
