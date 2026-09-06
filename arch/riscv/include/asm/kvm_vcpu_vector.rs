//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/kvm_vcpu_vector.h
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
// Copyright (C) 2022 SiFive
//
// Authors:
// Vincent Chen <vincent.chen@sifive.com>
// Greentime Hu <greentime.hu@sifive.com>
//

extern "C" {
    pub fn kvm_riscv_vcpu_vector_reset(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_host_vector_save(cntx: *mut kvm_cpu_context);
}
extern "C" {
    pub fn kvm_riscv_vcpu_host_vector_restore(cntx: *mut kvm_cpu_context);
}
extern "C" {
    pub fn kvm_riscv_vcpu_alloc_vector_context(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_free_vector_context(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_register_vctx_callback((*func)(void): *mut c_void);
}
extern "C" {
    pub fn kvm_riscv_unregister_vctx_callback();
}
extern "C" {
    pub fn kvm_riscv_vcpu_flush_vector();
}

