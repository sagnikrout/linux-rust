//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/flashchip.h
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
//
// Copyright © 2000      Red Hat UK Limited
// Copyright © 2000-2010 David Woodhouse <dwmw2@infradead.org>
//
// For spinlocks. sched.h includes spinlock.h from whichever directory it
// happens to be in - so we don't have to care whether we're on 2.2, which
// has asm/spinlock.h, or 2.4, which has linux/spinlock.h
//

// These 2 come from nand_state_t, which has been unified here
// These 4 come from onenand_state_t, which has been unified here
// NOTE: confusingly, this can be used to refer to more than one chip at a time,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flchip {
    pub /: *mut *mut unsigned long start; / Offset within the map,
// unsigned long len;
// We omit len for now, because when we group them together
//
    pub ref_point_counter: c_int,
    pub state: flstate_t,
    pub oldstate: flstate_t,
    pub write_suspended:1: c_uint,
    pub erase_suspended:1: c_uint,
    pub in_progress_block_addr: c_ulong,
    pub in_progress_block_mask: c_ulong,
    pub mutex: mutex,
    pub chip: *mut *mut wait_queue_head_t wq; / Wait on here when we're waiting for the,
    pub word_write_time: c_int,
    pub buffer_write_time: c_int,
    pub erase_time: c_int,
    pub word_write_time_max: c_int,
    pub buffer_write_time_max: c_int,
    pub erase_time_max: c_int,
    pub priv: *mut c_void,
}

// This is used to handle contention on write/erase operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flchip_shared {
    pub lock: mutex,
    pub writing: *mut flchip,
    pub erasing: *mut flchip,
}
