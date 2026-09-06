//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/kvm_vmid.h
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
// Copyright (c) 2025 Ventana Micro Systems Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vmid {
//
// Writes to vmid_version and vmid happen with vmid_lock held
// whereas reads happen without any lock held.
//
    pub vmid_version: c_ulong,
    pub vmid: c_ulong,
}

extern "C" {
    pub fn kvm_riscv_gstage_vmid_detect() -> void __init;
}
extern "C" {
    pub fn kvm_riscv_gstage_vmid_bits() -> c_ulong;
}
extern "C" {
    pub fn kvm_riscv_gstage_vmid_init(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_gstage_vmid_ver_changed(vmid: *mut kvm_vmid) -> bool;
}
extern "C" {
    pub fn kvm_riscv_gstage_vmid_update(vcpu: *mut kvm_vcpu);
}
