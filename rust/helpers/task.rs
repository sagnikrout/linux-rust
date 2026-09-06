//! Automatically rewritten from C to Rust
//! Source: rust/helpers/task.c
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

#[no_mangle]
pub unsafe extern "C" fn rust_helper_might_resched() -> __rust_helper void {
    __rust_helper void rust_helper_might_resched(void)
    {
    might_resched();
    }
    __rust_helper struct task_struct *rust_helper_get_current(void)
    {
    return current;
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_get_task_struct(t: *mut task_struct) -> __rust_helper void {
    __rust_helper void rust_helper_get_task_struct(struct task_struct *t)
    {
    get_task_struct(t);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_put_task_struct(t: *mut task_struct) -> __rust_helper void {
    __rust_helper void rust_helper_put_task_struct(struct task_struct *t)
    {
    put_task_struct(t);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_task_uid(task: *mut task_struct) -> __rust_helper kuid_t {
    __rust_helper kuid_t rust_helper_task_uid(struct task_struct *task)
    {
    return task_uid(task);
    }

#[no_mangle]
pub unsafe extern "C" fn rust_helper_from_kuid(to: *mut user_namespace, uid: kuid_t) -> __rust_helper uid_t {
    __rust_helper uid_t rust_helper_from_kuid(struct user_namespace *to, kuid_t uid)
    {
    return from_kuid(to, uid);
    }

#[no_mangle]
pub unsafe extern "C" fn rust_helper_uid_eq(left: kuid_t, right: kuid_t) -> __rust_helper bool {
    __rust_helper bool rust_helper_uid_eq(kuid_t left, kuid_t right)
    {
    return uid_eq(left, right);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_current_euid() -> __rust_helper kuid_t {
    __rust_helper kuid_t rust_helper_current_euid(void)
    {
    return current_euid();
    }
    __rust_helper struct user_namespace *rust_helper_current_user_ns(void)
    {
    return current_user_ns();
    }
    __rust_helper pid_t rust_helper_task_tgid_nr_ns(struct task_struct *tsk,
    struct pid_namespace *ns)
    {
    return task_tgid_nr_ns(tsk, ns);
    }
