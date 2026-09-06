//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/vmw_vmci/vmci_driver.h
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
// VMware VMCI Driver
//
// Copyright (C) 2012 VMware, Inc. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmci_obj_type {
    VMCIOBJ_VMX_VM = 10,
    VMCIOBJ_CONTEXT,
    VMCIOBJ_SOCKET,
    VMCIOBJ_NOT_SET,
}

// For storing VMCI structures in file handles.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_obj {
    pub ptr: *mut c_void,
    pub type: vmci_obj_type,
}

//
// Needed by other components of this module.  It's okay to have one global
// instance of this because there can only ever be one VMCI device.  Our
// virtual hardware enforces this.
//
extern "C" {
    pub fn vmci_get_context_id() -> u32;
}
extern "C" {
    pub fn vmci_send_datagram(dg: *mut vmci_datagram) -> c_int;
}
extern "C" {
    pub fn vmci_call_vsock_callback(is_host: bool);
}
extern "C" {
    pub fn vmci_host_init() -> c_int;
}
extern "C" {
    pub fn vmci_host_exit();
}
extern "C" {
    pub fn vmci_host_code_active() -> bool;
}
extern "C" {
    pub fn vmci_host_users() -> c_int;
}
extern "C" {
    pub fn vmci_guest_init() -> c_int;
}
extern "C" {
    pub fn vmci_guest_exit();
}
extern "C" {
    pub fn vmci_guest_code_active() -> bool;
}
extern "C" {
    pub fn vmci_get_vm_context_id() -> u32;
}
extern "C" {
    pub fn vmci_use_ppn64() -> bool;
}
