//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/resume_user_mode.h
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


// SPDX-License-Identifier: GPL-2.0-only

//
// set_notify_resume - cause resume_user_mode_work() to be called
// @task:		task that will call resume_user_mode_work()
//
// Calling this arranges that @task will call resume_user_mode_work()
// before returning to user mode.  If it's already running in user mode,
// it will enter the kernel and call resume_user_mode_work() soon.
// If it's blocked, it will not be woken.
//
// resume_user_mode_work - Perform work before returning to user mode
// @regs:		user-mode registers of @current task
//
// This is called when %TIF_NOTIFY_RESUME has been set.  Now we are
// about to return to user mode, and the user state in @regs can be
// inspected or adjusted.  The caller in arch code has cleared
// %TIF_NOTIFY_RESUME before the call.  If the flag gets set again
// asynchronously, this will be called again before we return to
// user mode.
//
// Called without locks.
//
// This barrier pairs with task_work_add()->set_notify_resume() after
// hlist_add_head(task->task_works);
//

