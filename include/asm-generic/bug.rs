//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/bug.h
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
#[derive(Copy, Clone)]
pub struct bug_entry {
    pub bug_addr): BUG_REL(unsigned long,,

    pub format): *const *const BUG_REL(char ,,

    pub file): *const *const BUG_REL(char ,,
    pub line: c_ushort,

    pub flags: c_ushort,
}

//
// Don't use BUG() or BUG_ON() unless there's really no way out; one
// example might be detecting data structure corruption in the middle
// of an operation that can't be backed out of.  If the (sub)system
// can somehow continue operating, perhaps with reduced functionality,
// it's probably not BUG-worthy.
//
// If you're tempted to BUG(), think again:  is completely giving up
// really the *only* solution?  There are usually better options, where
// users don't need to reboot ASAP and can mostly shut down cleanly.
//

//
// WARN(), WARN_ON(), WARN_ON_ONCE(), and so on can be used to report
// significant kernel issues that need prompt attention if they should ever
// appear at runtime.
//
// Do not use these macros when checking for invalid external inputs
// (e.g. invalid system call arguments, or invalid data coming from
// network/devices), and on transient conditions like ENOMEM or EAGAIN.
// These macros should be used for recoverable kernel issues only.
// For invalid external inputs, transient conditions, etc use
// pr_err[_once/_ratelimited]() followed by dump_stack(), if necessary.
// Do not include "BUG"/"WARNING" in format strings manually to make these
// conditions distinguishable from kernel issues.
//
// Use the versions with printk format strings to provide better diagnostics.
//
extern "C" {
    pub fn __printf(_arg: 1, fmt: *const 2) void __warn_printk(char, ...) -> extern;
}

// used internally by panic.c

//
// WARN_ON_SMP() is for cases that the warning is either
// meaningless for !SMP or may even cause failures.
// It can also be used with values that are only defined
// on SMP:
//
// struct foo {
// [...]
// #ifdef CONFIG_SMP
// int bar;
// #endif
// };
//
// void func(struct foo *zoot)
// {
// WARN_ON_SMP(!zoot->bar);
//
// For CONFIG_SMP, WARN_ON_SMP() should act the same as WARN_ON(),
// and should be a nop and return false for uniprocessor.
//
// if (WARN_ON_SMP(x)) returns true only when CONFIG_SMP is set
// and x is true.
//

//
// Use of ({0;}) because WARN_ON_SMP(x) may be used either as
// a stand alone line statement or as a condition in an if ()
// statement.
// A simple "0" would cause gcc to give a "statement has no effect"
// warning.
//

