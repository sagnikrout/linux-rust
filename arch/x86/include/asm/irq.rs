//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/irq.h
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
// (C) 1992, 1993 Linus Torvalds, (C) 1997 Ingo Molnar
//
// IRQ/IPI changes taken from work by Thomas Radke
// <tomsoft@informatik.tu-chemnitz.de>
//

//
// The irq entry code is in the noinstr section and the start/end of
// __irqentry_text is emitted via labels. Make the build fail if
// something moves a C function into the __irq_entry section.
//

extern "C" {
    pub fn irq_init_percpu_irqstack(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn fixup_irqs();
}

extern "C" {
    pub fn kvm_set_posted_intr_wakeup_handler((*handler)(void): *mut c_void);
}

extern "C" {
    pub fn void(_arg: *mut x86_platform_ipi_callback)(void) -> extern;
}
extern "C" {
    pub fn native_init_IRQ();
}
extern "C" {
    pub fn __handle_irq(desc: *mut irq_desc, regs: *mut pt_regs);
}
extern "C" {
    pub fn init_ISA_irqs();
}

