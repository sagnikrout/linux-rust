//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/runtime_instr.h
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

pub const S390_RUNTIME_INSTR_START: c_uint = 0x1;
pub const S390_RUNTIME_INSTR_STOP: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct runtime_instr_cb {
    pub rca: __u64,
    pub roa: __u64,
    pub rla: __u64,
    pub 1: __u32 v :,
    pub 1: __u32 s :,
    pub 1: __u32 k :,
    pub 1: __u32 h :,
    pub 1: __u32 a :,
    pub 3: __u32 reserved1 :,
    pub 1: __u32 ps :,
    pub 1: __u32 qs :,
    pub 1: __u32 pc :,
    pub 1: __u32 qc :,
    pub 1: __u32 reserved2 :,
    pub 1: __u32 g :,
    pub 1: __u32 u :,
    pub 1: __u32 l :,
    pub 4: __u32 key :,
    pub 8: __u32 reserved3 :,
    pub 1: __u32 t :,
    pub 3: __u32 rgs :,
    pub 4: __u32 m :,
    pub 1: __u32 n :,
    pub 1: __u32 mae :,
    pub 2: __u32 reserved4 :,
    pub 1: __u32 c :,
    pub 1: __u32 r :,
    pub 1: __u32 b :,
    pub 1: __u32 j :,
    pub 1: __u32 e :,
    pub 1: __u32 x :,
    pub 2: __u32 reserved5 :,
    pub 1: __u32 bpxn :,
    pub 1: __u32 bpxt :,
    pub 1: __u32 bpti :,
    pub 1: __u32 bpni :,
    pub 2: __u32 reserved6 :,
    pub 1: __u32 d :,
    pub 1: __u32 f :,
    pub 4: __u32 ic :,
    pub 4: __u32 dc :,
    pub reserved7: __u64,
    pub sf: __u64,
    pub rsic: __u64,
    pub reserved8: __u64,
// C attribute field omitted
    pub (*cb)): *mut : : "Q",
    pub "cc"): *mut *mut : "=Q" (cb) : :,
