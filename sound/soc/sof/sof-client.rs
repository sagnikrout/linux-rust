//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/sof-client.h
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
// struct sof_client_dev - SOF client device
// @auxdev:	auxiliary device
// @data:	device specific data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_client_dev {
    pub auxdev: auxiliary_device,
    pub data: *mut c_void,
}

extern "C" {
    pub fn sof_client_ipc_tx_message(_arg: cdev, _arg: ipc_msg, _arg: NULL, _arg: 0) -> return;
}
extern "C" {
    pub fn sof_client_get_ipc_max_payload_size(cdev: *mut sof_client_dev) -> usize;
}
extern "C" {
    pub fn sof_client_get_ipc_type(cdev: *mut sof_client_dev) -> sof_ipc_type;
}
// DSP/firmware boot request
extern "C" {
    pub fn sof_client_boot_dsp(cdev: *mut sof_client_dev) -> c_int;
}
// module refcount management of SOF core
extern "C" {
    pub fn sof_client_core_module_get(cdev: *mut sof_client_dev) -> c_int;
}
extern "C" {
    pub fn sof_client_core_module_put(cdev: *mut sof_client_dev);
}
// IPC notification
extern "C" {
    pub fn void(cdev: *mut *mut sof_client_event_callback)(struct sof_client_dev, msg_buf: *mut c_void) -> typedef;
}
// DSP state notification and query
extern "C" {
    pub fn sof_client_unregister_fw_state_handler(cdev: *mut sof_client_dev);
}
extern "C" {
    pub fn sof_client_get_fw_state(cdev: *mut sof_client_dev) -> sof_fw_state;
}
extern "C" {
    pub fn sof_client_ipc_rx_message(cdev: *mut sof_client_dev, ipc_msg: *mut c_void, msg_buf: *mut c_void) -> c_int;
}
