//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/flush.h
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
// Copyright 2023 Red Hat
//

// A marker for tracking which journal entries are affected by a flush request.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdo_flush {
// The completion for enqueueing this flush request.
    pub completion: vdo_completion,
// The flush bios covered by this request
    pub bios: bio_list,
// The wait queue entry for this flush
    pub waiter: vdo_waiter,
// Which flush this struct represents
    pub flush_generation: sequence_number_t,
}

extern "C" {
    pub fn vdo_make_flusher(vdo: *mut vdo) -> int __must_check;
}
extern "C" {
    pub fn vdo_free_flusher(flusher: *mut flusher);
}
extern "C" {
    pub fn vdo_get_flusher_thread_id(flusher: *mut flusher) -> thread_id_t __must_check;
}
extern "C" {
    pub fn vdo_complete_flushes(flusher: *mut flusher);
}
extern "C" {
    pub fn vdo_dump_flusher(flusher: *const flusher);
}
extern "C" {
    pub fn vdo_launch_flush(vdo: *mut vdo, bio: *mut bio);
}
extern "C" {
    pub fn vdo_drain_flusher(flusher: *mut flusher, completion: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_resume_flusher(flusher: *mut flusher, parent: *mut vdo_completion);
}
