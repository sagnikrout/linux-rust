//! Automatically rewritten from C Header to Rust Module
//! Source: fs/pstore/ram_internal.h
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
// Copyright (C) 2010 Marco Stornelli <marco.stornelli@gmail.com>
// Copyright (C) 2011 Kees Cook <keescook@chromium.org>
// Copyright (C) 2011 Google, Inc.
//

//
// Choose whether access to the RAM zone requires locking or not.  If a zone
// can be written to from different CPUs like with ftrace for example, then
// PRZ_FLAG_NO_LOCK is used. For all other cases, locking is required.
//

//
// If a PRZ should only have a single-boot lifetime, this marks it as
// getting wiped after its contents get copied out after boot.
//

//
// struct persistent_ram_zone - Details of a persistent RAM zone (PRZ)
// used as a pstore backend
//
// @paddr:	physical address of the mapped RAM area
// @size:	size of mapping
// @label:	unique name of this PRZ
// @type:	frontend type for this PRZ
// @flags:	holds PRZ_FLAGS_* bits
//
// @buffer_lock:
// locks access to @buffer "size" bytes and "start" offset
// @buffer:
// pointer to actual RAM area managed by this PRZ
// @buffer_size:
// bytes in @buffer->data (not including any trailing ECC bytes)
//
// @par_buffer:
// pointer into @buffer->data containing ECC bytes for @buffer->data
// @par_header:
// pointer into @buffer->data containing ECC bytes for @buffer header
// (i.e. all fields up to @data)
// @rs_decoder:
// RSLIB instance for doing ECC calculations
// @corrected_bytes:
// ECC corrected bytes accounting since boot
// @bad_blocks:
// ECC uncorrectable bytes accounting since boot
// @ecc_info:
// ECC configuration details
//
// @old_log:
// saved copy of @buffer->data prior to most recent wipe
// @old_log_size:
// bytes contained in @old_log
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct persistent_ram_zone {
    pub paddr: phys_addr_t,
    pub size: usize,
    pub vaddr: *mut c_void,
    pub label: *mut c_char,
    pub type: pstore_type_id,
    pub flags: u32,
    pub buffer_lock: raw_spinlock_t,
    pub buffer: *mut persistent_ram_buffer,
    pub buffer_size: usize,
    pub par_buffer: *mut c_char,
    pub par_header: *mut c_char,
    pub rs_decoder: *mut rs_control,
    pub corrected_bytes: c_int,
    pub bad_blocks: c_int,
    pub ecc_info: persistent_ram_ecc_info,
    pub old_log: *mut c_char,
    pub old_log_size: usize,
}

extern "C" {
    pub fn persistent_ram_free(_prz: *mut persistent_ram_zone);
}
extern "C" {
    pub fn persistent_ram_zap(prz: *mut persistent_ram_zone);
}
extern "C" {
    pub fn persistent_ram_save_old(prz: *mut persistent_ram_zone);
}
extern "C" {
    pub fn persistent_ram_old_size(prz: *mut persistent_ram_zone) -> usize;
}
extern "C" {
    pub fn persistent_ram_free_old(prz: *mut persistent_ram_zone);
}
