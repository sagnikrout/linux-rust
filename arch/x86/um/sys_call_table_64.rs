//! Automatically rewritten from C to Rust
//! Source: arch/x86/um/sys_call_table_64.c
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
// System call table for UML/x86-64, copied from arch/x86/kernel/syscall_*.c
// with some changes for UML.
//

    extern asmlinkage long sys_ni_syscall(unsigned long, unsigned long,
    unsigned long, unsigned long,
    unsigned long, unsigned long);
//
// Below you can see, in terms of #define's, the differences between the x86-64
// and the UML syscall table.
//
// Not going to be implemented by UML, since we have no hardware.

    const sys_call_ptr_t sys_call_table[] ____cacheline_aligned = {

    };
    let mut syscall_table_size: c_int = sizeof(sys_call_table);
