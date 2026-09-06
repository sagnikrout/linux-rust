//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/debug/debug_core.h
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
// Created by: Jason Wessel <jason.wessel@windriver.com>
//
// Copyright (c) 2009 Wind River Systems, Inc.  All Rights Reserved.
//
// These are the private implementation headers between the kernel
// debugger core and the debugger front end code.
//
// kernel debug core data structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kgdb_state {
    pub ex_vector: c_int,
    pub signo: c_int,
    pub err_code: c_int,
    pub cpu: c_int,
    pub pass_exception: c_int,
    pub thr_query: c_ulong,
    pub threadid: c_ulong,
    pub kgdb_usethreadid: c_long,
    pub linux_regs: *mut pt_regs,
    pub send_ready: *mut core::sync::atomic::AtomicI32,
}

// Exception state values
pub const DCPU_WANT_MASTER: c_uint = 0x1 /* Waiting to become a master kgdb cpu */;
pub const DCPU_NEXT_MASTER: c_uint = 0x2 /* Transition from one master cpu to another */;
pub const DCPU_IS_SLAVE: c_uint = 0x4 /* Slave cpu enter exception */;
pub const DCPU_WANT_BT: c_uint = 0x8 /* Slave cpu should backtrace then clear flag */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct debuggerinfo_struct {
    pub debuggerinfo: *mut c_void,
    pub task: *mut task_struct,
    pub exception_state: c_int,
    pub ret_state: c_int,
    pub irq_depth: c_int,
    pub enter_kgdb: c_int,
    pub rounding_up: bool,
}

// kernel debug core break point routines
extern "C" {
    pub fn dbg_remove_all_break() -> c_int;
}
extern "C" {
    pub fn dbg_set_sw_break(addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn dbg_remove_sw_break(addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn dbg_activate_sw_breakpoints() -> c_int;
}
extern "C" {
    pub fn dbg_deactivate_sw_breakpoints() -> c_int;
}
// polled character access to i/o module
extern "C" {
    pub fn dbg_io_get_char() -> c_int;
}
// stub return value for switching between the gdbstub and kdb

// Switch from one cpu to another

// gdbstub interface functions
extern "C" {
    pub fn gdb_serial_stub(ks: *mut kgdb_state) -> c_int;
}
extern "C" {
    pub fn gdbstub_msg_write(s: *const c_char, len: c_int);
}
// gdbstub functions used for kdb <-> gdbstub transition
extern "C" {
    pub fn gdbstub_state(ks: *mut kgdb_state, cmd: *mut c_char) -> c_int;
}

extern "C" {
    pub fn kdb_stub(ks: *mut kgdb_state) -> c_int;
}
extern "C" {
    pub fn kdb_parse(cmdstr: *const c_char) -> c_int;
}
extern "C" {
    pub fn kdb_common_init_state(ks: *mut kgdb_state) -> c_int;
}
extern "C" {
    pub fn kdb_common_deinit_state() -> c_int;
}
extern "C" {
    pub fn kdb_dump_stack_on_cpu(cpu: c_int);
}

