//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/iosm/iosm_ipc_wwan.h
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
// Copyright (C) 2020-21 Intel Corporation.
//
// ipc_wwan_init - Allocate, Init and register WWAN device
// @ipc_imem:		Pointer to imem data-struct
// @dev:		Pointer to device structure
//
// Returns: Pointer to instance on success else NULL
//
// ipc_wwan_deinit - Unregister and free WWAN device, clear pointer
// @ipc_wwan:	Pointer to wwan instance data
//
extern "C" {
    pub fn ipc_wwan_deinit(ipc_wwan: *mut iosm_wwan);
}
//
// ipc_wwan_receive - Receive a downlink packet from CP.
// @ipc_wwan:	Pointer to wwan instance
// @skb_arg:	Pointer to struct sk_buff
// @dss:	Set to true if interafce id is from 257 to 261,
// else false
// @if_id:	Interface ID
//
// Return: 0 on success and failure value on error
//
// ipc_wwan_tx_flowctrl - Enable/Disable TX flow control
// @ipc_wwan:	Pointer to wwan instance
// @id:		Ipc mux channel session id
// @on:		if true then flow ctrl would be enabled else disable
//
extern "C" {
    pub fn ipc_wwan_tx_flowctrl(ipc_wwan: *mut iosm_wwan, id: c_int, on: bool);
}
