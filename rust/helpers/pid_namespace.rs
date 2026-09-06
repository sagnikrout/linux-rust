//! Automatically rewritten from C to Rust
//! Source: rust/helpers/pid_namespace.c
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

    __rust_helper struct pid_namespace *
    rust_helper_get_pid_ns(struct pid_namespace *ns)
    {
    return get_pid_ns(ns);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_put_pid_ns(ns: *mut pid_namespace) -> __rust_helper void {
    __rust_helper void rust_helper_put_pid_ns(struct pid_namespace *ns)
    {
    put_pid_ns(ns);
    }
// Get a reference on a task's pid namespace.
    __rust_helper struct pid_namespace *
    rust_helper_task_get_pid_ns(struct task_struct *task)
    {
    struct pid_namespace *pid_ns;
    guard(rcu)();
    pid_ns = task_active_pid_ns(task);
    if (pid_ns)
    get_pid_ns(pid_ns);
    return pid_ns;
    }
