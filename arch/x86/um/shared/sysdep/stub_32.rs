//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/um/shared/sysdep/stub_32.h
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


//
// Copyright (C) 2004 Jeff Dike (jdike@addtoit.com)
// Licensed under the GPL
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_args {
    pub ebp: int ebx,,
    pub }: } args = { arg1, arg6,
    pub ret: c_long,
    pub %%ebp;": __asm__ volatile ("pushl,
    pub 0x4(%%ebx),%%ebp;": "movl,
    pub (%%ebx),%%ebx;": "movl,
    pub $0x80;": "int,
    pub "memory"): :,
    pub ret: return,
    pub ret: c_ulong,
    pub _here_%=;": "call,
    pub %0;": "popl,
    pub ;": "andl %1, %0,
    pub ;": "addl %2, %0,
    pub (UM_KERN_PAGE_SIZE)): "g",
    pub )ret: *mut return (void,

    pub \: "subl %0,%%esp ;",
    pub \: "movl %1, %%eax ; ",
    pub \: *mut *mut "call %%eax ;",
    pub {: for (int i = 0; i < sizeof(arch->tls) / sizeof(arch->tls[0]); i++),
    pub &arch->tls[i]): (unsigned long),
    pub 0: arch->sync =,
