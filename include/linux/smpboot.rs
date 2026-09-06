//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/smpboot.h
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

// Cookie handed to the thread_fn
//
// struct smp_hotplug_thread - CPU hotplug related thread descriptor
// @store:		Pointer to per cpu storage for the task pointers
// @list:		List head for core management
// @thread_should_run:	Check whether the thread should run or not. Called with
// preemption disabled.
// @thread_fn:		The associated thread function
// @create:		Optional setup function, called when the thread gets
// created (Not called from the thread context)
// @setup:		Optional setup function, called when the thread gets
// operational the first time
// @cleanup:		Optional cleanup function, called when the thread
// should stop (module exit)
// @park:		Optional park function, called when the thread is
// parked (cpu offline)
// @unpark:		Optional unpark function, called when the thread is
// unparked (cpu online)
// @selfparking:	Thread is not parked by the park function.
// @thread_comm:	The base name of the thread
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_hotplug_thread {
    pub store: *mut *mut task_ __percpu,
    pub list: list_head,
    pub cpu): *mut *mut int (thread_should_run)(unsigned int,
    pub cpu): *mut *mut void (thread_fn)(unsigned int,
    pub cpu): *mut *mut void (create)(unsigned int,
    pub cpu): *mut *mut void (setup)(unsigned int,
    pub online): *mut *mut void (cleanup)(unsigned int cpu, bool,
    pub cpu): *mut *mut void (park)(unsigned int,
    pub cpu): *mut *mut void (unpark)(unsigned int,
    pub selfparking: bool,
    pub thread_comm: *const c_char,
}

extern "C" {
    pub fn smpboot_register_percpu_thread(plug_thread: *mut smp_hotplug_thread) -> c_int;
}
extern "C" {
    pub fn smpboot_unregister_percpu_thread(plug_thread: *mut smp_hotplug_thread);
}
