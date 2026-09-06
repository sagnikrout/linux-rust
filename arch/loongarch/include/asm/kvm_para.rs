//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/kvm_para.h
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
// Hypercall code field
//
pub const HYPERVISOR_KVM: c_int = 1;
pub const HYPERVISOR_VENDOR_SHIFT: c_int = 8;

pub const KVM_HCALL_CODE_SERVICE: c_int = 0;
pub const KVM_HCALL_CODE_SWDBG: c_int = 1;
pub const KVM_HCALL_CODE_USER_SERVICE: c_int = 2;

pub const KVM_HCALL_FUNC_IPI: c_int = 1;
pub const KVM_HCALL_FUNC_NOTIFY: c_int = 2;

//
// LoongArch hypercall return code
//
pub const KVM_HCALL_SUCCESS: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_steal_time {
    pub steal: __u64,
    pub version: __u32,
    pub flags: __u32,
    pub preempted: __u8,
    pub pad: [__u8; 47],
}

//
// Hypercall interface for KVM hypervisor
//
// a0: function identifier
// a1-a5: args
// Return value will be placed in a0.
// Up to 5 arguments are passed in a1, a2, a3, a4, a5.
//
extern "C" {
    pub fn asm(_arg: "a0") -> register long ret;
}
extern "C" {
    pub fn asm(_arg: "a0") -> register long ret;
}
extern "C" {
    pub fn asm(_arg: "a0") -> register long ret;
}
extern "C" {
    pub fn asm(_arg: "a0") -> register long ret;
}
extern "C" {
    pub fn asm(_arg: "a0") -> register long ret;
}
extern "C" {
    pub fn asm(_arg: "a0") -> register long ret;
}

extern "C" {
    pub fn kvm_para_available() -> bool;
}
extern "C" {
    pub fn kvm_arch_para_features() -> c_uint;
}

