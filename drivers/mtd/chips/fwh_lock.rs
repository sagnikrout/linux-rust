//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/chips/fwh_lock.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fwh_lock_state {
    FWH_UNLOCKED   = 0,
    FWH_DENY_WRITE = 1,
    FWH_IMMUTABLE  = 2,
    FWH_DENY_READ  = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwh_xxlock_thunk {
    pub val: fwh_lock_state,
    pub state: flstate_t,
}

//
// This locking/unlock is specific to firmware hub parts.  Only one
// is known that supports the Intel command set.    Firmware
// hub parts cannot be interleaved as they are on the LPC bus
// so this code has not been tested with interleaved chips,
// and will likely fail in that context.
//
// Refuse the operation if the we cannot look behind the chip
//
// lock block registers:
// - on 64k boundariesand
// - bit 1 set high
// - block lock registers are 4MiB lower - overflow subtract (danger)
//
// The address manipulation is first done on the logical address
// which is 0 at the start of the chip, and then the offset of
// the individual chip is addted to it.  Any other order a weird
// map offset could cause problems.
//
// This is easy because these are writes to registers and not writes
// to flash memory - that means that we don't have to check status
// and timeout.
//
// Done and happy.
// Setup for the chips with the fwh lock method
