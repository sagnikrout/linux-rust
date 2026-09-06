//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/iosm/iosm_ipc_port.h
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

//
// struct iosm_cdev - State of the char driver layer.
// @iosm_port:		Pointer of type wwan_port
// @ipc_imem:		imem instance
// @dev:		Pointer to device struct
// @pcie:		PCIe component
// @port_type:		WWAN port type
// @channel:		Channel instance
// @chl_id:		Channel Identifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_cdev {
    pub iosm_port: *mut wwan_port,
    pub ipc_imem: *mut iosm_imem,
    pub dev: *mut device,
    pub pcie: *mut iosm_pcie,
    pub port_type: wwan_port_type,
    pub channel: *mut ipc_mem_channel,
    pub chl_id: ipc_channel_id,
}

//
// ipc_port_init - Allocate IPC port & register to wwan subsystem for AT/MBIM
// communication.
// @ipc_imem:		Pointer to iosm_imem structure
// @ipc_port_cfg:	IPC Port Config
//
// Returns: 0 on success & NULL on failure
//
// ipc_port_deinit - Free IPC port & unregister port with wwan subsystem.
// @ipc_port:	Array of pointer to the ipc port data-struct
//
extern "C" {
    pub fn ipc_port_deinit(ipc_port[]: *mut iosm_cdev);
}
