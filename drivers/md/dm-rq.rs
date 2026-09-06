//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-rq.h
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
// Internal header file for device mapper
//
// Copyright (C) 2016 Red Hat, Inc. All rights reserved.
//
// This file is released under the LGPL.
//

//
// For request-based dm - the bio clones we allocate are embedded in these
// structs.
//
// We allocate these with bio_alloc_bioset, using the front_pad parameter when
// the bioset is created - this means the bio has to come at the end of the
// struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_rq_clone_bio_info {
    pub orig: *mut bio,
    pub tio: *mut dm_rq_target_io,
    pub clone: bio,
}

extern "C" {
    pub fn dm_mq_init_request_queue(md: *mut mapped_device, t: *mut dm_table) -> c_int;
}
extern "C" {
    pub fn dm_mq_cleanup_mapped_device(md: *mut mapped_device);
}
extern "C" {
    pub fn dm_start_queue(q: *mut request_queue);
}
extern "C" {
    pub fn dm_stop_queue(q: *mut request_queue);
}
extern "C" {
    pub fn dm_mq_kick_requeue_list(md: *mut mapped_device);
}
extern "C" {
    pub fn dm_get_reserved_rq_based_ios() -> c_uint;
}
extern "C" {
    pub fn dm_attr_rq_based_seq_io_merge_deadline_show(md: *mut mapped_device, buf: *mut c_char) -> isize;
}
