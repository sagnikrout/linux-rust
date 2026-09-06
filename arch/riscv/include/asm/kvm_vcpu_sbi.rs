//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/kvm_vcpu_sbi.h
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
// Copyright (c) 2021 Western Digital Corporation or its affiliates.
//
// Authors:
// Atish Patra <atish.patra@wdc.com>
//
pub const KVM_SBI_IMPID: c_int = 3;
pub const KVM_SBI_VERSION_MAJOR: c_int = 3;
pub const KVM_SBI_VERSION_MINOR: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_riscv_sbi_ext_status {
    KVM_RISCV_SBI_EXT_STATUS_UNINITIALIZED,
    KVM_RISCV_SBI_EXT_STATUS_UNAVAILABLE,
    KVM_RISCV_SBI_EXT_STATUS_ENABLED,
    KVM_RISCV_SBI_EXT_STATUS_DISABLED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_sbi_context {
    pub return_handled: c_int,
    pub ext_status: [kvm_riscv_sbi_ext_status; KVM_RISCV_SBI_EXT_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_sbi_return {
    pub out_val: c_ulong,
    pub err_val: c_ulong,
    pub utrap: *mut kvm_cpu_trap,
    pub uexit: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_sbi_extension {
    pub extid_start: c_ulong,
    pub extid_end: c_ulong,
    pub default_disabled: bool,
//
// SBI extension handler. It can be defined for a given extension or group of
// extension. But it should always return linux error codes rather than SBI
// specific error codes.
//
    pub retdata): *mut kvm_vcpu_sbi_return,
// Extension specific probe function
    pub vcpu): *mut *mut unsigned long (probe)(struct kvm_vcpu,
//
// Init/deinit function called once during VCPU init/destroy. These
// might be use if the SBI extensions need to allocate or do specific
// init time only configuration.
//
    pub vcpu): *mut *mut int (init)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (deinit)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (reset)(struct kvm_vcpu,
// Allow the extension to correct its parameters before the first run
    pub vcpu): *mut *mut void (validate)(struct kvm_vcpu,
    pub state_reg_subtype: c_ulong,
    pub vcpu): *mut *mut unsigned long (get_state_reg_count)(struct kvm_vcpu,
    pub reg_id): *mut *mut *mut int (get_state_reg_id)(struct kvm_vcpu vcpu, int index, u64,
    pub reg_val): *mut unsigned long reg_size, void,
    pub reg_val): *const unsigned long reg_size, void,
}

extern "C" {
    pub fn kvm_riscv_vcpu_sbi_load_reset_state(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_sbi_return(vcpu: *mut kvm_vcpu, run: *mut kvm_run) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_reg_indices_sbi_ext(vcpu: *mut kvm_vcpu, uindices: *mut u64 __user) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_reg_indices_sbi(vcpu: *mut kvm_vcpu, uindices: *mut u64 __user) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_set_reg_sbi(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_get_reg_sbi(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_sbi_ecall(vcpu: *mut kvm_vcpu, run: *mut kvm_run) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_sbi_init(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_sbi_deinit(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_sbi_reset(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_sbi_validate(vcpu: *mut kvm_vcpu);
}

