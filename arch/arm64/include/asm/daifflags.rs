//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/daifflags.h
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
// Copyright (C) 2017 ARM Ltd.
//

pub const DAIF_PROCCTX: c_int = 0;

// mask/save/unmask/restore all exceptions, including interrupts.
// Don't really care for a dsb here, we don't intend to enable IRQs
// If IRQs are masked with PMR, reflect it in the flags
//
// If interrupts are disabled but we can take
// asynchronous errors, we can take NMIs
//
// There has been concern that the write to daif
// might be reordered before this write to PMR.
// From the ARM ARM DDI 0487D.a, section D1.7.1
// "Accessing PSTATE fields":
// Writes to the PSTATE fields have side-effects on
// various aspects of the PE operation. All of these
// side-effects are guaranteed:
// - Not to be visible to earlier instructions in
// the execution stream.
// - To be visible to later instructions in the
// execution stream
//
// Also, writes to PMR are self-synchronizing, so no
// interrupts with a lower priority than PMR is signaled
// to the PE after the write.
//
// So we don't need additional synchronization here.
//
// Called by synchronous exception handlers to restore the DAIF bits that were
// modified by taking an exception.
//
// We can't use local_daif_restore(regs->pstate) here as
// system_has_prio_mask_debugging() won't restore the I bit if it can
// use the pmr instead.
//
