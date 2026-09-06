//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/kvm_nacl.h
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
// Copyright (c) 2024 Ventana Micro Systems Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_riscv_nacl {
    pub shmem: *mut c_void,
    pub shmem_phys: phys_addr_t,
}

extern "C" {
    pub fn kvm_riscv_nacl_enable() -> c_int;
}
extern "C" {
    pub fn kvm_riscv_nacl_disable();
}
extern "C" {
    pub fn kvm_riscv_nacl_exit();
}
extern "C" {
    pub fn kvm_riscv_nacl_init() -> c_int;
}

// __p = cpu_to_lelong(__val);					\

//
// Each ncsr_xyz() macro defined below has it's own static-branch so every
// use of ncsr_xyz() macro emits a patchable direct jump. This means multiple
// back-to-back ncsr_xyz() macro usage will emit multiple patchable direct
// jumps which is sub-optimal.
//
// Based on the above, it is recommended to avoid multiple back-to-back
// ncsr_xyz() macro usage.
//

