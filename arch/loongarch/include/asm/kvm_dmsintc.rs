//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/kvm_dmsintc.h
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
// Copyright (C) 2025 Loongson Technology Corporation Limited
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongarch_dmsintc {
    pub kvm: *mut kvm,
    pub msg_addr_base: u64,
    pub msg_addr_size: u64,
    pub cpu_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmsintc_state {
    pub vector_map: [core::sync::atomic::AtomicI64; 4],
}

extern "C" {
    pub fn kvm_loongarch_register_dmsintc_device() -> c_int;
}
extern "C" {
    pub fn kvm_loongarch_unregister_dmsintc_device();
}
extern "C" {
    pub fn dmsintc_inject_irq(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn dmsintc_set_irq(kvm: *mut kvm, addr: u64, data: c_int, level: c_int) -> c_int;
}
extern "C" {
    pub fn dmsintc_deliver_msi_to_vcpu(kvm: *mut kvm, vcpu: *mut kvm_vcpu, vector: u32, level: c_int) -> c_int;
}
