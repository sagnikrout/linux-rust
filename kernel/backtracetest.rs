//! Automatically rewritten from C to Rust
//! Source: kernel/backtracetest.c
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

macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0-only
//
// Simple stack backtrace regression test module
//
// (C) Copyright 2008 Intel Corporation
// Author: Arjan van de Ven <arjan@linux.intel.com>
//

#[no_mangle]
unsafe extern "C" fn backtrace_test_normal() {
    pr_info("Testing a backtrace from process context.\n");
    pr_info("The following trace is a kernel self test and not a bug!\n");
    dump_stack();
    }
#[no_mangle]
unsafe extern "C" fn backtrace_test_bh_workfn(work: *mut work_struct) {
    dump_stack();
    }
// static DECLARE_WORK(backtrace_bh_work, &backtrace_test_bh_workfn);
#[no_mangle]
unsafe extern "C" fn backtrace_test_bh() {
    pr_info("Testing a backtrace from BH context.\n");
    pr_info("The following trace is a kernel self test and not a bug!\n");
    queue_work(system_bh_wq, &backtrace_bh_work);
    flush_work(&backtrace_bh_work);
    }

#[no_mangle]
unsafe extern "C" fn backtrace_test_saved() {
    unsigned long entries[8];
    let mut nr_entries = 0;
    pr_info("Testing a saved backtrace.\n");
    pr_info("The following trace is a kernel self test and not a bug!\n");
    nr_entries = stack_trace_save(entries, ARRAY_SIZE(entries), 0);
    stack_trace_print(entries, nr_entries, 0);
    }

#[no_mangle]
unsafe extern "C" fn backtrace_test_saved() {
    pr_info("Saved backtrace test skipped.\n");
    }

#[no_mangle]
unsafe extern "C" fn backtrace_regression_test() -> c_int {
    pr_info("====[ backtrace testing ]===========\n");
    backtrace_test_normal();
    backtrace_test_bh();
    backtrace_test_saved();
    pr_info("====[ end of backtrace testing ]====\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exitf() {
    }
// module_init;
// module_exit;
// MODULE_DESCRIPTION;
// MODULE_LICENSE;
// MODULE_AUTHOR;