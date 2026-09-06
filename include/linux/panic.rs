//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/panic.h
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

extern "C" {
    pub fn long(state: *mut *mut panic_blink)(int) -> extern;
}
extern "C" {
    pub fn nmi_panic(regs: *mut pt_regs, msg: *const c_char);
}
extern "C" {
    pub fn check_panic_on_warn(origin: *const c_char);
}
extern "C" {
    pub fn oops_enter();
}
extern "C" {
    pub fn oops_exit();
}
extern "C" {
    pub fn oops_may_print() -> bool;
}
extern "C" {
    pub fn __stack_chk_fail();
}
extern "C" {
    pub fn abort();
}
//
// panic_cpu is used for synchronizing panic() and crash_kexec() execution. It
// holds a CPU number which is executing panic() currently. A value of
// PANIC_CPU_INVALID means no CPU has entered panic() or crash_kexec().
//
// panic_redirect_cpu is used when panic is redirected to a specific CPU via
// the panic_force_cpu= boot parameter. It holds the CPU number that originally
// triggered the panic before redirection. A value of PANIC_CPU_INVALID means
// no redirection has occurred.
//

extern "C" {
    pub fn panic_try_start() -> bool;
}
extern "C" {
    pub fn panic_reset();
}
extern "C" {
    pub fn panic_in_progress() -> bool;
}
extern "C" {
    pub fn panic_on_this_cpu() -> bool;
}
extern "C" {
    pub fn panic_on_other_cpu() -> bool;
}
//
// Only to be used by arch init code. If the user over-wrote the default
// CONFIG_PANIC_TIMEOUT, honor it.
//
// This cannot be an enum because some may be used in assembly source.
pub const TAINT_PROPRIETARY_MODULE: c_int = 0;
pub const TAINT_FORCED_MODULE: c_int = 1;
pub const TAINT_CPU_OUT_OF_SPEC: c_int = 2;
pub const TAINT_FORCED_RMMOD: c_int = 3;
pub const TAINT_MACHINE_CHECK: c_int = 4;
pub const TAINT_BAD_PAGE: c_int = 5;
pub const TAINT_USER: c_int = 6;
pub const TAINT_DIE: c_int = 7;
pub const TAINT_OVERRIDDEN_ACPI_TABLE: c_int = 8;
pub const TAINT_WARN: c_int = 9;
pub const TAINT_CRAP: c_int = 10;
pub const TAINT_FIRMWARE_WORKAROUND: c_int = 11;
pub const TAINT_OOT_MODULE: c_int = 12;
pub const TAINT_UNSIGNED_MODULE: c_int = 13;
pub const TAINT_SOFTLOCKUP: c_int = 14;
pub const TAINT_LIVEPATCH: c_int = 15;
pub const TAINT_AUX: c_int = 16;
pub const TAINT_RANDSTRUCT: c_int = 17;
pub const TAINT_TEST: c_int = 18;
pub const TAINT_FWCTL: c_int = 19;
pub const TAINT_FLAGS_COUNT: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct taint_flag {
    pub /: *mut *mut char c_true; / character printed when tainted,
    pub /: *mut *mut char c_false; / character printed when not tainted,
    pub /: *const *const *const char desc; / verbose description of the set taint flag,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lockdep_ok {
    LOCKDEP_STILL_OK,
    LOCKDEP_NOW_UNRELIABLE,
}

extern "C" {
    pub fn add_taint(flag: unsigned, lockdep_ok: enum);
}
extern "C" {
    pub fn test_taint(flag: unsigned) -> c_int;
}
extern "C" {
    pub fn get_taint() -> c_ulong;
}
