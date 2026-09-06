//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/iosm/iosm_ipc_imem_ops.h
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

// Maximum wait time for blocking read
pub const IPC_READ_TIMEOUT: c_int = 3000;
// The delay in ms for defering the unregister
pub const SIO_UNREGISTER_DEFER_DELAY_MS: c_int = 1;
// Default delay till CP PSI image is running and modem updates the
// execution stage.
// unit : milliseconds
//
pub const PSI_START_DEFAULT_TIMEOUT: c_int = 3000;
// Default time out when closing SIO, till the modem is in
// running state.
// unit : milliseconds
//
pub const BOOT_CHECK_DEFAULT_TIMEOUT: c_int = 400;
// IP MUX channel range
pub const IP_MUX_SESSION_START: c_int = 0;
pub const IP_MUX_SESSION_END: c_int = 7;
// Default IP MUX channel
pub const IP_MUX_SESSION_DEFAULT: c_int = 0;
//
// ipc_imem_sys_port_open - Open a port link to CP.
// @ipc_imem:	Imem instance.
// @chl_id:	Channel Identifier.
// @hp_id:	HP Identifier.
//
// Return: channel instance on success, NULL for failure
//
// ipc_imem_sys_cdev_write - Route the uplink buffer to CP.
// @ipc_cdev:		iosm_cdev instance.
// @skb:		Pointer to skb.
//
// Return: 0 on success and failure value on error
//
extern "C" {
    pub fn ipc_imem_sys_cdev_write(ipc_cdev: *mut iosm_cdev, skb: *mut sk_buff) -> c_int;
}
//
// ipc_imem_sys_wwan_open - Open packet data online channel between network
// layer and CP.
// @ipc_imem:		Imem instance.
// @if_id:		ip link tag of the net device.
//
// Return: Channel ID on success and failure value on error
//
extern "C" {
    pub fn ipc_imem_sys_wwan_open(ipc_imem: *mut iosm_imem, if_id: c_int) -> c_int;
}
//
// ipc_imem_sys_wwan_close - Close packet data online channel between network
// layer and CP.
// @ipc_imem:		Imem instance.
// @if_id:		IP link id net device.
// @channel_id:		Channel ID to be closed.
//
// ipc_imem_sys_wwan_transmit - Function for transfer UL data
// @ipc_imem:		Imem instance.
// @if_id:		link ID of the device.
// @channel_id:		Channel ID used
// @skb:		Pointer to sk buffer
//
// Return: 0 on success and failure value on error
//
// ipc_imem_wwan_channel_init - Initializes WWAN channels and the channel for
// MUX.
// @ipc_imem:		Pointer to iosm_imem struct.
// @mux_type:		Type of mux protocol.
//
// Return: 0 on success and failure value on error
//
// ipc_imem_sys_devlink_open - Open a Flash/CD Channel link to CP
// @ipc_imem:   iosm_imem instance
//
// Return:	channel instance on success, NULL for failure
//
// ipc_imem_sys_devlink_close - Release a Flash/CD channel link to CP
// @ipc_devlink:	Pointer to ipc_devlink data-struct
//
extern "C" {
    pub fn ipc_imem_sys_devlink_close(ipc_devlink: *mut iosm_devlink);
}
//
// ipc_imem_sys_devlink_notify_rx - Receive downlink characters from CP,
// the downlink skbuf is added at the end of the
// downlink or rx list
// @ipc_devlink:	Pointer to ipc_devlink data-struct
// @skb:		Pointer to sk buffer
//
// ipc_imem_sys_devlink_read - Copy the rx data and free the skbuf
// @ipc_devlink:	Devlink instance
// @data:		Buffer to read the data from modem
// @bytes_to_read:	Size of destination buffer
// @bytes_read:		Number of bytes read
//
// Return: 0 on success and failure value on error
//
// ipc_imem_sys_devlink_write - Route the uplink buffer to CP
// @ipc_devlink:	Devlink_sio instance
// @buf:		Pointer to buffer
// @count:		Number of data bytes to write
// Return:		0 on success and failure value on error
//
