//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mailbox_client.h
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
// Copyright (C) 2013-2014 Linaro Ltd.
// Author: Jassi Brar <jassisinghbrar@gmail.com>
//

//
// struct mbox_client - User of a mailbox
// @dev:		The client device
// @tx_block:		If the mbox_send_message should block until data is
// transmitted.
// @tx_tout:		Max block period in ms before TX is assumed failure
// @knows_txdone:	If the client could run the TX state machine. Usually
// if the client receives some ACK packet for transmission.
// Unused if the controller already has TX_Done/RTR IRQ.
// @rx_callback:	Atomic callback to provide client the data received
// @tx_prepare: 	Atomic callback to ask client to prepare the payload
// before initiating the transmission if required.
// @tx_done:		Atomic callback to tell client of data transmission
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_client {
    pub dev: *mut device,
    pub tx_block: bool,
    pub tx_tout: c_ulong,
    pub knows_txdone: bool,
    pub mssg): *mut *mut *mut void (rx_callback)(struct mbox_client cl, void,
    pub mssg): *mut *mut *mut void (tx_prepare)(struct mbox_client cl, void,
    pub r): *mut *mut *mut *mut void (tx_done)(struct mbox_client cl, void mssg, int,
}

extern "C" {
    pub fn mbox_bind_client(chan: *mut mbox_chan, cl: *mut mbox_client) -> c_int;
}
extern "C" {
    pub fn mbox_send_message(chan: *mut mbox_chan, mssg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mbox_flush(chan: *mut mbox_chan, timeout: c_ulong) -> c_int;
}
