//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/guarded_storage.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gs_cb {
    pub reserved: __u64,
    pub gsd: __u64,
    pub gssm: __u64,
    pub gs_epl_a: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gs_epl {
    pub pad1: __u8,
    pub gs_eam: __u8,
    pub 6: __u8 :,
    pub 1: __u8 e :,
    pub 1: __u8 b :,
}

pub const GS_ENABLE: c_int = 0;
pub const GS_DISABLE: c_int = 1;
pub const GS_SET_BC_CB: c_int = 2;
pub const GS_CLEAR_BC_CB: c_int = 3;
pub const GS_BROADCAST: c_int = 4;
extern "C" {
    pub fn volatile(rxy: ".insn, _arg: 0xe3000000004d, _arg: 0, (*gs_cb): *mut %0" : : "Q") -> asm;
}
extern "C" {
    pub fn volatile(rxy: ".insn, _arg: 0xe30000000049, _arg: 0, (*gs_cb): *mut %0" : : "Q") -> asm;
}
