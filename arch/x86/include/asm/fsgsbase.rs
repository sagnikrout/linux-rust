//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/fsgsbase.h
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
// Read/write a task's FSBASE or GSBASE. This returns the value that
// the FS/GS base would have (if the task were to be resumed). These
// work on the current task or on a non-running (typically stopped
// ptrace child) task.
//
extern "C" {
    pub fn x86_fsbase_read_task(task: *mut task_struct) -> c_ulong;
}
extern "C" {
    pub fn x86_gsbase_read_task(task: *mut task_struct) -> c_ulong;
}
extern "C" {
    pub fn x86_fsbase_write_task(task: *mut task_struct, fsbase: c_ulong);
}
extern "C" {
    pub fn x86_gsbase_write_task(task: *mut task_struct, gsbase: c_ulong);
}
// Must be protected by X86_FEATURE_FSGSBASE check.
extern "C" {
    pub fn volatile((fsbase): "rdfsbase %0" : "=r") -> asm;
}
extern "C" {
    pub fn volatile((gsbase): "rdgsbase %0" : "=r") -> asm;
}
extern "C" {
    pub fn volatile("memory": "wrfsbase %0" :: "r" (fsbase) :) -> asm;
}
extern "C" {
    pub fn volatile("memory": "wrgsbase %0" :: "r" (gsbase) :) -> asm;
}

// Helper functions for reading/writing FS/GS base
extern "C" {
    pub fn x86_gsbase_read_cpu_inactive() -> c_ulong;
}
extern "C" {
    pub fn x86_gsbase_write_cpu_inactive(gsbase: c_ulong);
}

