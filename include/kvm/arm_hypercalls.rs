//! Automatically rewritten from C Header to Rust Module
//! Source: include/kvm/arm_hypercalls.h
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
// Copyright (C) 2019 Arm Ltd.

extern "C" {
    pub fn kvm_smccc_call_handler(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn vcpu_get_reg(_arg: vcpu, _arg: 0) -> return;
}
extern "C" {
    pub fn vcpu_get_reg(_arg: vcpu, _arg: 1) -> return;
}
extern "C" {
    pub fn vcpu_get_reg(_arg: vcpu, _arg: 2) -> return;
}
extern "C" {
    pub fn vcpu_get_reg(_arg: vcpu, _arg: 3) -> return;
}
extern "C" {
    pub fn kvm_arm_init_hypercalls(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_arm_teardown_hypercalls(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_arm_get_fw_num_regs(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_arm_copy_fw_reg_indices(vcpu: *mut kvm_vcpu, uindices: *mut u64 __user) -> c_int;
}
extern "C" {
    pub fn kvm_arm_get_fw_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> c_int;
}
extern "C" {
    pub fn kvm_arm_set_fw_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> c_int;
}
extern "C" {
    pub fn kvm_vm_smccc_has_attr(kvm: *mut kvm, attr: *mut kvm_device_attr) -> c_int;
}
extern "C" {
    pub fn kvm_vm_smccc_set_attr(kvm: *mut kvm, attr: *mut kvm_device_attr) -> c_int;
}
