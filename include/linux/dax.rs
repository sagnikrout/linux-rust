//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dax.h
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

pub type dax_entry_t = c_ulong;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dax_access_mode {
    DAX_ACCESS,
    DAX_RECOVERY_WRITE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dax_operations {
//
// direct_access: translate a device-relative
// logical-page-offset into an absolute physical pfn. Return the
// number of pages available for DAX at that pfn.
//
    pub ): *mut *mut *mut dax_access_mode, void , unsigned long,
// zero_page_range: required operation. Zero page range
    pub size_t): *mut *mut *mut int (zero_page_range)(struct dax_device , pgoff_t,,
//
// recovery_write: recover a poisoned range by DAX device driver
// capable of clearing poison.
//
    pub iter): *mut *mut void addr, size_t bytes, struct iov_iter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dax_holder_operations {
//
// notify_failure - notify memory failure into inner holder device
// @dax_dev: the dax device which contains the holder
// @offset: offset on this dax device where memory failure occurs
// @len: length of this memory failure event
// @flags: action flags for memory failure handler
//
    pub mf_flags): u64 len, int,
}

extern "C" {
    pub fn put_dax(dax_dev: *mut dax_device);
}
extern "C" {
    pub fn kill_dax(dax_dev: *mut dax_device);
}
extern "C" {
    pub fn dax_write_cache(dax_dev: *mut dax_device, wc: bool);
}
extern "C" {
    pub fn dax_write_cache_enabled(dax_dev: *mut dax_device) -> bool;
}
extern "C" {
    pub fn dax_synchronous(dax_dev: *mut dax_device) -> bool;
}
extern "C" {
    pub fn set_dax_nocache(dax_dev: *mut dax_device);
}
extern "C" {
    pub fn set_dax_nomc(dax_dev: *mut dax_device);
}
extern "C" {
    pub fn set_dax_synchronous(dax_dev: *mut dax_device);
}
//
// Check if given mapping is supported by the file / underlying device.
//
extern "C" {
    pub fn dax_synchronous(_arg: dax_dev) -> return;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

extern "C" {
    pub fn dax_add_host(dax_dev: *mut dax_device, disk: *mut gendisk) -> c_int;
}
extern "C" {
    pub fn dax_remove_host(disk: *mut gendisk);
}

extern "C" {
    pub fn fs_put_dax(dax_dev: *mut dax_device, holder: *mut c_void);
}
extern "C" {
    pub fn dax_folio_reset_order(folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn dax_lock_folio(folio: *mut folio) -> dax_entry_t;
}
extern "C" {
    pub fn dax_unlock_folio(folio: *mut folio, cookie: dax_entry_t);
}

extern "C" {
    pub fn dax_read_lock() -> c_int;
}
extern "C" {
    pub fn dax_read_unlock(id: c_int);
}

extern "C" {
    pub fn dax_alive(dax_dev: *mut dax_device) -> bool;
}
extern "C" {
    pub fn dax_set_ops(dax_dev: *mut dax_device, ops: *const dax_operations) -> c_int;
}
extern "C" {
    pub fn dax_flush(dax_dev: *mut dax_device, addr: *mut c_void, size: usize);
}
extern "C" {
    pub fn dax_delete_mapping_entry(mapping: *mut address_space, index: pgoff_t) -> c_int;
}
extern "C" {
    pub fn dax_break_layout(_arg: inode, _arg: 0, _arg: LLONG_MAX, _arg: cb) -> return;
}
extern "C" {
    pub fn dax_break_layout_final(inode: *mut inode);
}
//
// Due to dax's memory and block duo personalities, hwpoison reporting
// takes into consideration which personality is presently visible.
// When dax acts like a block device, such as in block IO, an encounter of
// dax hwpoison is reported as -EIO.
// When dax acts like memory, such as in page fault, a detection of hwpoison
// is reported as -EHWPOISON which leads to VM_FAULT_HWPOISON.
//

extern "C" {
    pub fn hmem_register_resource(target_nid: c_int, r: *mut resource);
}

extern "C" {
    pub fn walk_hmem_resources(dev: *mut device, fn: walk_hmem_fn) -> c_int;
}
