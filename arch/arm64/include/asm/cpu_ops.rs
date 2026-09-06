//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/cpu_ops.h
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
// Copyright (C) 2013 ARM Ltd.
//

//
// struct cpu_operations - Callback operations for hotplugging CPUs.
//
// @name:	Name of the property as appears in a devicetree cpu node's
// enable-method property. On systems booting with ACPI, @name
// identifies the struct cpu_operations entry corresponding to
// the boot protocol specified in the ACPI MADT table.
// @cpu_init:	Reads any data necessary for a specific enable-method for a
// proposed logical id.
// @cpu_prepare: Early one-time preparation step for a cpu. If there is a
// mechanism for doing so, tests whether it is possible to boot
// the given CPU.
// @cpu_boot:	Boots a cpu into the kernel.
// @cpu_postboot: Optionally, perform any post-boot cleanup or necessary
// synchronisation. Called from the cpu being booted.
// @cpu_can_disable: Determines whether a CPU can be disabled based on
// mechanism-specific information.
// @cpu_disable: Prepares a cpu to die. May fail for some mechanism-specific
// reason, which will cause the hot unplug to be aborted. Called
// from the cpu to be killed.
// @cpu_die:	Makes a cpu leave the kernel. Must not fail. Called from the
// cpu being killed.
// @cpu_kill:  Ensures a cpu has left the kernel. Called from another cpu.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_operations {
    pub name: *const c_char,
    pub int): *mut *mut int (cpu_init)(unsigned,
    pub int): *mut *mut int (cpu_prepare)(unsigned,
    pub int): *mut *mut int (cpu_boot)(unsigned,
    pub (*cpu_postboot)(void): *mut c_void,

    pub cpu): *mut *mut bool (cpu_can_disable)(unsigned int,
    pub cpu): *mut *mut int (cpu_disable)(unsigned int,
    pub cpu): *mut *mut void (cpu_die)(unsigned int,
    pub cpu): *mut *mut int (cpu_kill)(unsigned int,

}

extern "C" {
    pub fn init_cpu_ops(cpu: c_int) -> int __init;
}
