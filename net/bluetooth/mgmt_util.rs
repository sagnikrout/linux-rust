//! Automatically rewritten from C Header to Rust Module
//! Source: net/bluetooth/mgmt_util.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_mesh_tx {
    pub list: list_head,
    pub index: c_int,
    pub param_len: usize,
    pub sk: *mut sock,
    pub handle: u8,
    pub instance: u8,
    pub 31]: u8 param[sizeof(struct mgmt_cp_mesh_send) +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_pending_cmd {
    pub list: list_head,
    pub opcode: u16,
    pub hdev: *mut hci_dev,
    pub param: *mut c_void,
    pub param_len: usize,
    pub sk: *mut sock,
    pub skb: *mut sk_buff,
    pub user_data: *mut c_void,
    pub status): *mut *mut *mut int (cmd_complete)(struct mgmt_pending_cmd cmd, u8,
}

extern "C" {
    pub fn mgmt_cmd_status(sk: *mut sock, index: u16, cmd: u16, status: u8) -> c_int;
}
extern "C" {
    pub fn mgmt_pending_free(cmd: *mut mgmt_pending_cmd);
}
extern "C" {
    pub fn mgmt_pending_remove(cmd: *mut mgmt_pending_cmd);
}
extern "C" {
    pub fn __mgmt_pending_listed(hdev: *mut hci_dev, cmd: *mut mgmt_pending_cmd) -> bool;
}
extern "C" {
    pub fn mgmt_pending_listed(hdev: *mut hci_dev, cmd: *mut mgmt_pending_cmd) -> bool;
}
extern "C" {
    pub fn mgmt_pending_valid(hdev: *mut hci_dev, cmd: *mut mgmt_pending_cmd) -> bool;
}
extern "C" {
    pub fn mgmt_mesh_remove(mesh_tx: *mut mgmt_mesh_tx);
}
