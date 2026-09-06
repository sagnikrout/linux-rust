//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kdb.h
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
// Kernel Debugger Architecture Independent Global Headers
//
// Copyright (c) 2000-2007 Silicon Graphics, Inc.  All Rights Reserved.
// Copyright (C) 2000 Stephane Eranian <eranian@hpl.hp.com>
// Copyright (C) 2009 Jason Wessel <jason.wessel@windriver.com>
//

// Shifted versions of the command enable bits are be used if the command
// has no arguments (see kdb_check_flags). This allows commands, such as
// go, to have different permissions depending upon whether it is called
// with an argument.
//
pub const KDB_ENABLE_NO_ARGS_SHIFT: c_int = 10;
// User exposed values stop here, all remaining flags are
// exclusively used to describe a commands behaviour.
//
extern "C" {
    pub fn int(_arg: *mut kdb_func_t)(int, : *const c_char) -> typedef;
}
// The KDB shell command table

pub const KDB_POLL_FUNC_MAX: c_int = 5;
//
// kdb_initial_cpu is initialized to -1, and is set to the cpu
// number whenever the kernel debugger is entered.
//
// Types and messages used for dynamically added kdb shell commands

// KDB return codes from a command or internal kdb function

//
// kdb_diemsg
//
// Contains a pointer to the last string supplied to the
// kernel 'die' panic function.
//

// kdb is disabled

// not use keyboard

// not use keyboard

//
// External entry point for the kernel debugger.  The pt_regs
// at the time of entry are supplied along with the reason for
// entry to the kernel debugger.
//
// regs probably valid
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kdb_msgsrc {
    KDB_MSGSRC_INTERNAL, /* direct call to kdb_printf() */
    KDB_MSGSRC_PRINTK, /* trapped from printk() */
}

extern "C" {
    pub fn __printf(_arg: 1, : *const 2) int kdb_printf(char, ...) -> extern;
}
extern "C" {
    pub fn __printf(_arg: 1, : *const *const 2) int (kdb_printf_t)(char, ...) -> typedef;
}
extern "C" {
    pub fn kdb_init(level: c_int);
}
// Access to kdb specific polling devices
extern "C" {
    pub fn int(_arg: *mut get_char_func)(void) -> typedef;
}
extern "C" {
    pub fn kdb_get_kbd_char() -> c_int;
}
extern "C" {
    pub fn kdb_send_sig(p: *mut task_struct, sig: c_int);
}

// Dynamic kdb shell command registration
extern "C" {
    pub fn kdb_register(cmd: *mut kdbtab_t) -> c_int;
}
extern "C" {
    pub fn kdb_unregister(cmd: *mut kdbtab_t);
}
// Return true when KDB as locked for printing a message on this CPU.
//
// We can use raw_smp_processor_id() here because the task could
// not get migrated when KDB has locked for printing on this CPU.
//
extern "C" {
    pub fn unlikely(raw_smp_processor_id(): READ_ONCE(kdb_printf_cpu) ==) -> return;
}

extern "C" {
    pub fn kdbgetintenv(: *const c_char, : *mut c_int) -> c_int;
}
extern "C" {
    pub fn kdb_set(_arg: c_int, : *const c_char) -> c_int;
}
extern "C" {
    pub fn kdb_lsmod(argc: c_int, argv: *const c_char) -> c_int;
}
