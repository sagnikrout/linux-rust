//! Automatically rewritten from C to Rust
//! Source: arch/x86/um/stub_segv.c
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
// Copyright (C) 2004 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
// Licensed under the GPL
//

#[no_mangle]
pub unsafe extern "C" fn __attribute__((".__syscall_stub")): (__section__) {
    void __attribute__ ((__section__ (".__syscall_stub")))
    stub_segv_handler(int sig, siginfo_t *info, void *p)
    {
    struct faultinfo *f = get_stub_data();
    ucontext_t *uc = p;
    GET_FAULTINFO_FROM_MC(*f, &uc.uc_mcontext);
    trap_myself();
    }
