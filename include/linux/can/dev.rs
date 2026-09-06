//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/can/dev.h
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
// linux/can/dev.h
//
// Definitions for the CAN network device driver interface
//
// Copyright (C) 2006 Andrey Volkov <avolkov@varma-el.com>
// Varma Electronics Oy
//
// Copyright (C) 2008 Wolfgang Grandegger <wg@grandegger.com>
//

//
// CAN mode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum can_mode {
    CAN_MODE_STOP = 0,
    CAN_MODE_START,
    CAN_MODE_SLEEP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum can_termination_gpio {
    CAN_TERMINATION_GPIO_DISABLED = 0,
    CAN_TERMINATION_GPIO_ENABLED,
    CAN_TERMINATION_GPIO_MAX,
}

//
// CAN common private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_priv {
    pub dev: *mut net_device,
    pub can_stats: can_device_stats,
    pub bittiming_const: *const can_bittiming_,
    pub bittiming: can_bittiming,
    pub xl: data_bittiming_params fd,,
    pub bitrate_const_cnt: c_uint,
    pub bitrate_const: *const u32,
    pub bitrate_max: u32,
    pub clock: can_clock,
    pub termination_const_cnt: c_uint,
    pub termination_const: *const u16,
    pub termination: u16,
    pub termination_gpio: *mut gpio_desc,
    pub termination_gpio_ohms: [u16; CAN_TERMINATION_GPIO_MAX],
    pub echo_skb_max: c_uint,
    pub echo_skb: *mut sk_buff,
    pub state: can_state,
// CAN controller features - see include/uapi/linux/can/netlink.h
    pub /: *mut *mut u32 ctrlmode; / current options setting,
    pub /: *mut *mut u32 ctrlmode_supported; / options that can be modified by netlink,
    pub restart_ms: c_int,
    pub restart_work: delayed_work,
    pub dev): *mut *mut int (do_set_bittiming)(struct net_device,
    pub mode): *mut *mut *mut int (do_set_mode)(struct net_device dev, enum can_mode,
    pub term): *mut *mut *mut int (do_set_termination)(struct net_device dev, u16,
    pub state): *mut can_state,
    pub bec): *mut can_berr_counter,
}

extern "C" {
    pub fn can_setup(dev: *mut net_device);
}

extern "C" {
    pub fn free_candev(dev: *mut net_device);
}
// a candev safe wrapper around netdev_priv
extern "C" {
    pub fn open_candev(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn close_candev(dev: *mut net_device);
}
extern "C" {
    pub fn can_set_default_mtu(dev: *mut net_device);
}
extern "C" {
    pub fn can_set_cap_info(dev: *mut net_device);
}
extern "C" {
    pub fn register_candev(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn unregister_candev(dev: *mut net_device);
}
extern "C" {
    pub fn can_restart_now(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn can_bus_off(dev: *mut net_device);
}
// When CAN XL is enabled but FD is disabled we are running in
// the so-called 'CANXL-only mode' where the error signalling is
// disabled. This helper function determines the required value
// to disable error signalling in the CAN XL controller.
// The so-called CC/FD/XL 'mixed mode' requires error signalling.
//
// drop skb if it does not contain a valid CAN frame for sending
extern "C" {
    pub fn can_dropped_invalid_skb(_arg: dev, _arg: skb) -> return;
}

extern "C" {
    pub fn of_can_transceiver(dev: *mut net_device);
}

extern "C" {
    pub fn can_netlink_register() -> c_int;
}
extern "C" {
    pub fn can_netlink_unregister();
}
