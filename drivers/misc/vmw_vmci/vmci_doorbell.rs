//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/vmw_vmci/vmci_doorbell.h
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

//
// VMCINotifyResourceInfo: Used to create and destroy doorbells, and
// generate a notification for a doorbell or queue pair.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_dbell_notify_resource_info {
    pub handle: vmci_handle,
    pub resource: u16,
    pub action: u16,
    pub result: i32,
}

//
// Structure used for checkpointing the doorbell mappings. It is
// written to the checkpoint as is, so changing this structure will
// break checkpoint compatibility.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbell_cpt_state {
    pub handle: vmci_handle,
    pub bitmap_idx: u64,
}

extern "C" {
    pub fn vmci_dbell_host_context_notify(src_cid: u32, handle: vmci_handle) -> c_int;
}
extern "C" {
    pub fn vmci_dbell_get_priv_flags(handle: vmci_handle, priv_flags: *mut u32) -> c_int;
}
extern "C" {
    pub fn vmci_dbell_register_notification_bitmap(bitmap_ppn: u64) -> bool;
}
extern "C" {
    pub fn vmci_dbell_scan_notification_entries(bitmap: *mut u8);
}
