//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rolling_buffer.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Rolling buffer of folios
//
// Copyright (C) 2024 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Rolling buffer.  Whilst the buffer is live and in use, folios and folio
// queue segments can be added to one end by one thread and removed from the
// other end by another thread.  The buffer isn't allowed to be empty; it must
// always have at least one folio_queue in it so that neither side has to
// modify both queue pointers.
//
// The iterator in the buffer is extended as buffers are inserted.  It can be
// snapshotted to use a segment of the buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rolling_buffer {
    pub /: *mut *mut *mut folio_queue head; / Producer's insertion point,
    pub /: *mut *mut *mut folio_queue tail; / Consumer's removal point,
    pub /: *mut *mut iov_iter iter; / Iterator tracking what's left in the buffer,
    pub /: *mut *mut u8 next_head_slot; / Next slot in ->head,
    pub /: *mut *mut u8 first_tail_slot; / First slot in ->tail,
}

//
// Snapshot of a rolling buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rolling_buffer_snapshot {
    pub /: *mut *mut *mut folio_queue curr_folioq; / Queue segment in which current folio resides,
    pub /: *mut *mut unsigned char curr_slot; / Folio currently being read,
    pub /: *mut *mut unsigned char curr_order; / Order of folio,
}

// Marks to store per-folio in the internal folio_queue structs.

extern "C" {
    pub fn rolling_buffer_make_space(roll: *mut rolling_buffer, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn rolling_buffer_clear(roll: *mut rolling_buffer);
}
