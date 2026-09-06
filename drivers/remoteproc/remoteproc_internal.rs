//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/remoteproc/remoteproc_internal.h
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
// Remote processor framework
//
// Copyright (C) 2011 Texas Instruments, Inc.
// Copyright (C) 2011 Google, Inc.
//
// Ohad Ben-Cohen <ohad@wizery.com>
// Brian Swetland <swetland@google.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rproc_debug_trace {
    pub rproc: *mut rproc,
    pub tfile: *mut dentry,
    pub node: list_head,
    pub trace_mem: rproc_mem_entry,
}

//
// struct rproc_vdev_data - remoteproc virtio device data
// @rsc_offset: offset of the vdev's resource entry
// @id: virtio device id (as in virtio_ids.h)
// @index: vdev position versus other vdev declared in resource table
// @rsc: pointer to the vdev resource entry. Valid only during vdev init as
// the resource can be cached by rproc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rproc_vdev_data {
    pub rsc_offset: u32,
    pub id: c_uint,
    pub index: u32,
    pub rsc: *mut fw_rsc_vdev,
}

extern "C" {
    pub fn test_bit(_arg: feature, _arg: rproc->features) -> return;
}
// from remoteproc_core.c
extern "C" {
    pub fn rproc_release(kref: *mut kref);
}
// from remoteproc_virtio.c
extern "C" {
    pub fn rproc_vq_interrupt(rproc: *mut rproc, vq_id: c_int) -> irqreturn_t;
}
// from remoteproc_debugfs.c
extern "C" {
    pub fn rproc_remove_trace_file(tfile: *mut dentry);
}
extern "C" {
    pub fn rproc_delete_debug_dir(rproc: *mut rproc);
}
extern "C" {
    pub fn rproc_create_debug_dir(rproc: *mut rproc);
}
extern "C" {
    pub fn rproc_init_debugfs();
}
extern "C" {
    pub fn rproc_exit_debugfs();
}
// from remoteproc_sysfs.c
extern "C" {
    pub fn rproc_init_sysfs() -> c_int;
}
extern "C" {
    pub fn rproc_exit_sysfs();
}

extern "C" {
    pub fn rproc_init_cdev();
}
extern "C" {
    pub fn rproc_exit_cdev();
}
extern "C" {
    pub fn rproc_char_device_add(rproc: *mut rproc) -> c_int;
}
extern "C" {
    pub fn rproc_char_device_remove(rproc: *mut rproc);
}

//
// The character device interface is an optional feature, if it is not enabled
// the function should not return an error.
//

extern "C" {
    pub fn rproc_free_vring(rvring: *mut rproc_vring);
}
extern "C" {
    pub fn rproc_alloc_vring(rvdev: *mut rproc_vdev, i: c_int) -> c_int;
}
extern "C" {
    pub fn rproc_parse_vring(rvdev: *mut rproc_vdev, rsc: *mut fw_rsc_vdev, i: c_int) -> c_int;
}
extern "C" {
    pub fn rproc_va_to_pa(cpu_addr: *mut c_void) -> phys_addr_t;
}
extern "C" {
    pub fn rproc_trigger_recovery(rproc: *mut rproc) -> c_int;
}
extern "C" {
    pub fn rproc_elf_sanity_check(rproc: *mut rproc, fw: *const firmware) -> c_int;
}
extern "C" {
    pub fn rproc_elf_get_boot_addr(rproc: *mut rproc, fw: *const firmware) -> u64;
}
extern "C" {
    pub fn rproc_elf_load_segments(rproc: *mut rproc, fw: *const firmware) -> c_int;
}
extern "C" {
    pub fn rproc_elf_load_rsc_table(rproc: *mut rproc, fw: *const firmware) -> c_int;
}
extern "C" {
    pub fn rproc_add_rvdev(rproc: *mut rproc, rvdev: *mut rproc_vdev);
}
extern "C" {
    pub fn rproc_remove_rvdev(rvdev: *mut rproc_vdev);
}

