//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/irqflags.h
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
// Copyright (C) 2012 ARM Ltd.
//

//
// Aarch64 has flags for masking: Debug, Asynchronous (serror), Interrupts and
// FIQ exceptions, in the 'daif' register. We mask and unmask them in 'daif'
// order:
// Masking debug exceptions causes all other exceptions to be masked too
// Masking SError masks IRQ/FIQ, but not debug exceptions. IRQ and FIQ are
// always masked and unmasked together, and have no side effects for other
// flags. Keeping to this order makes it easier for entry.S to know which
// exceptions should be unmasked.
//
extern "C" {
    pub fn volatile(daifclr: "msr, _arg: #3") -> asm;
}
extern "C" {
    pub fn volatile(daifset: "msr, _arg: #3") -> asm;
}
extern "C" {
    pub fn read_sysreg(_arg: daif) -> return;
}
extern "C" {
    pub fn read_sysreg_s(_arg: SYS_ICC_PMR_EL1) -> return;
}
//
// Save the current interrupt enable state.
//
extern "C" {
    pub fn __pmr_local_save_flags() -> return;
}
extern "C" {
    pub fn __daif_local_save_flags() -> return;
}
extern "C" {
    pub fn __pmr_irqs_disabled_flags(_arg: flags) -> return;
}
extern "C" {
    pub fn __daif_irqs_disabled_flags(_arg: flags) -> return;
}
extern "C" {
    pub fn __daif_irqs_disabled_flags(_arg: __daif_local_save_flags()) -> return;
}
extern "C" {
    pub fn __pmr_irqs_disabled_flags(_arg: __pmr_local_save_flags()) -> return;
}
extern "C" {
    pub fn __pmr_irqs_disabled() -> return;
}
extern "C" {
    pub fn __daif_irqs_disabled() -> return;
}
//
// There are too many states with IRQs disabled, just keep the current
// state if interrupts are already disabled/masked.
//
extern "C" {
    pub fn __pmr_local_irq_save() -> return;
}
extern "C" {
    pub fn __daif_local_irq_save() -> return;
}
//
// restore saved IRQ state
//
