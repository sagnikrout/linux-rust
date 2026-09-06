//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/irq.h
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

// Total number of virq in the platform

// Number of irqs reserved for a legacy isa controller
pub const NR_IRQS_LEGACY: c_int = 16;
extern "C" {
    pub fn virq_to_hw(virq: c_uint) -> irq_hw_number_t;
}

//
// Per-cpu stacks for handling critical, debug and machine check
// level interrupts.
//

//
// Per-cpu stacks for handling hard and soft interrupts.
//
extern "C" {
    pub fn __do_IRQ(regs: *mut pt_regs);
}
extern "C" {
    pub fn irq_choose_cpu(mask: *const cpumask) -> c_int;
}

