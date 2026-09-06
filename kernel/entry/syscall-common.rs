//! Automatically rewritten from C to Rust
//! Source: kernel/entry/syscall-common.c
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

// Macro flag: #define CREATE_TRACE_POINTS

// Out of line to prevent tracepoint code duplication
#[no_mangle]
pub unsafe extern "C" fn trace_syscall_enter(regs: *mut pt_regs) {
    void trace_syscall_enter(struct pt_regs *regs)
    {
    trace_sys_enter(regs, syscall_get_nr(current, regs));
    }
#[no_mangle]
pub unsafe extern "C" fn trace_syscall_exit(regs: *mut pt_regs, ret: c_long) {
    void trace_syscall_exit(struct pt_regs *regs, long ret)
    {
    trace_sys_exit(regs, ret);
    }

#[no_mangle]
pub unsafe extern "C" fn syscall_enter_audit(regs: *mut pt_regs) {
    void syscall_enter_audit(struct pt_regs *regs)
    {
    let mut syscall: c_long = syscall_get_nr(current, regs);
    unsigned long args[6];
    syscall_get_arguments(current, regs, args);
    __audit_syscall_entry(syscall, args[0], args[1], args[2], args[3]);
    }
