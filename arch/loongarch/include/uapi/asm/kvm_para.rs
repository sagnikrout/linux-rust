//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/uapi/asm/kvm_para.h
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

//
// CPUCFG index area: 0x40000000 -- 0x400000ff
// SW emulation for KVM hypervirsor
//
pub const CPUCFG_KVM_BASE: c_uint = 0x40000000;
pub const CPUCFG_KVM_SIZE: c_uint = 0x100;

pub const KVM_FEATURE_IPI: c_int = 1;
pub const KVM_FEATURE_STEAL_TIME: c_int = 2;
pub const KVM_FEATURE_PREEMPT: c_int = 3;
// BIT 24 - 31 are features configurable by user space vmm
pub const KVM_FEATURE_VIRT_EXTIOI: c_int = 24;
pub const KVM_FEATURE_USER_HCALL: c_int = 25;
