//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/shared/msr.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msr {
    pub l: u32,
    pub h: u32,
}

//
// The kernel proper already defines rdmsr()/wrmsr(), but they are not for the
// boot kernel since they rely on tracepoint/exception handling infrastructure
// that's not available here.
//
extern "C" {
    pub fn volatile((m->l): "rdmsr" : "=a", (reg): "=d" (m->h) : "c") -> asm;
}
extern "C" {
    pub fn volatile((reg): "wrmsr" : : "c", _arg: "a"(m->l), "memory": "d" (m->h) :) -> asm;
}
