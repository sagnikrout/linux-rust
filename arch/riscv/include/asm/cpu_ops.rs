//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/cpu_ops.h
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
// Copyright (c) 2020 Western Digital Corporation or its affiliates.
// Based on arch/arm64/include/asm/cpu_ops.h
//

//
// struct cpu_operations - Callback operations for hotplugging CPUs.
//
// @cpu_start:		Boots a cpu into the kernel.
// @cpu_stop:		Makes a cpu leave the kernel. Must not fail. Called from
// the cpu being stopped.
// @cpu_is_stopped:	Ensures a cpu has left the kernel. Called from another
// cpu.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_operations {
    pub tidle): *mut task_struct,

    pub (*cpu_stop)(void): *mut c_void,
    pub cpu): *mut *mut bool (cpu_is_stopped)(unsigned int,

}

extern "C" {
    pub fn cpu_set_ops() -> void __init;
}
