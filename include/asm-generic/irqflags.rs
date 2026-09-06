//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/irqflags.h
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
//
// All architectures should implement at least the first two functions,
// usually inline assembly will be the best way.
//

pub const ARCH_IRQ_DISABLED: c_int = 0;
pub const ARCH_IRQ_ENABLED: c_int = 1;

// read interrupt enabled status

extern "C" {
    pub fn arch_local_save_flags() -> c_ulong;
}

// set interrupt enabled status

extern "C" {
    pub fn arch_local_irq_restore(flags: c_ulong);
}

// get status and disable interrupts

// test flags

// unconditionally enable interrupts

// unconditionally disable interrupts

// test hardware interrupt enable bit

extern "C" {
    pub fn arch_irqs_disabled_flags(_arg: arch_local_save_flags()) -> return;
}

