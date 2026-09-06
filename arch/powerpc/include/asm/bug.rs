//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/bug.h
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

// _EMIT_BUG_ENTRY expects args %0,%1,%2,%3 to be FILE, LINE, flags and

//
// BUG_ON() and WARN_ON() do their best to cooperate with compile-time
// optimisations. However depending on the complexity of the condition
// some compiler versions may not produce optimal results.
//

// Macro flag: #define HAVE_ARCH_BUG

// Macro flag: #define HAVE_ARCH_BUG_ON
// Macro flag: #define HAVE_ARCH_WARN_ON

extern "C" {
    pub fn hash__do_page_fault(: *mut pt_regs);
}
extern "C" {
    pub fn bad_page_fault(: *mut pt_regs, _arg: c_int);
}
extern "C" {
    pub fn emulate_single_step(regs: *mut pt_regs);
}
extern "C" {
    pub fn _exception(_arg: c_int, : *mut pt_regs, _arg: c_int, long: unsigned);
}
extern "C" {
    pub fn _exception_pkey(: *mut pt_regs, long: unsigned, _arg: c_int);
}
extern "C" {
    pub fn die(: *const c_char, : *mut pt_regs, _arg: c_long);
}
extern "C" {
    pub fn die_mce(str: *const c_char, regs: *mut pt_regs, err: c_long);
}
extern "C" {
    pub fn die_will_crash() -> bool;
}
extern "C" {
    pub fn panic_flush_kmsg_start();
}
extern "C" {
    pub fn panic_flush_kmsg_end();
}

