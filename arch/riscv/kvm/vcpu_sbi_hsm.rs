//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kvm/vcpu_sbi_hsm.c
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
// Copyright (c) 2021 Western Digital Corporation or its affiliates.
//
// Authors:
// Atish Patra <atish.patra@wdc.com>
//

#[no_mangle]
unsafe extern "C" fn kvm_sbi_hsm_vcpu_start(vcpu: *mut kvm_vcpu) -> c_int {
    static int kvm_sbi_hsm_vcpu_start(struct kvm_vcpu *vcpu)
    {
    struct kvm_cpu_context *cp = &vcpu.arch.guest_context;
    struct kvm_vcpu *target_vcpu;
    let mut target_vcpuid: c_ulong = cp.a0;
    let mut ret: c_int = 0;
    target_vcpu = kvm_get_vcpu_by_id(vcpu.kvm, target_vcpuid);
    if (!target_vcpu)
    return SBI_ERR_INVALID_PARAM;
    spin_lock(&target_vcpu.arch.mp_state_lock);
    if (!kvm_riscv_vcpu_stopped(target_vcpu)) {
    ret = SBI_ERR_ALREADY_AVAILABLE;
    goto out;
    }
    kvm_riscv_vcpu_sbi_request_reset(target_vcpu, cp.a1, cp.a2);
    __kvm_riscv_vcpu_power_on(target_vcpu);
    out:
    spin_unlock(&target_vcpu.arch.mp_state_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn kvm_sbi_hsm_vcpu_stop(vcpu: *mut kvm_vcpu) -> c_int {
    static int kvm_sbi_hsm_vcpu_stop(struct kvm_vcpu *vcpu)
    {
    let mut ret: c_int = 0;
    spin_lock(&vcpu.arch.mp_state_lock);
    if (kvm_riscv_vcpu_stopped(vcpu)) {
    ret = SBI_ERR_FAILURE;
    goto out;
    }
    __kvm_riscv_vcpu_power_off(vcpu);
    out:
    spin_unlock(&vcpu.arch.mp_state_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn kvm_sbi_hsm_vcpu_get_status(vcpu: *mut kvm_vcpu) -> c_int {
    static int kvm_sbi_hsm_vcpu_get_status(struct kvm_vcpu *vcpu)
    {
    struct kvm_cpu_context *cp = &vcpu.arch.guest_context;
    let mut target_vcpuid: c_ulong = cp.a0;
    struct kvm_vcpu *target_vcpu;
    target_vcpu = kvm_get_vcpu_by_id(vcpu.kvm, target_vcpuid);
    if (!target_vcpu)
    return SBI_ERR_INVALID_PARAM;
    if (kvm_riscv_vcpu_stopped(target_vcpu))
    return SBI_HSM_STATE_STOPPED;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: target_vcpu->stat.generic.blocking) -> else {
    else if (target_vcpu.stat.generic.blocking)
    return SBI_HSM_STATE_SUSPENDED;
    else
    return SBI_HSM_STATE_STARTED;
    }
    static int kvm_sbi_ext_hsm_handler(struct kvm_vcpu *vcpu, struct kvm_run *run,
    struct kvm_vcpu_sbi_return *retdata)
    {
    let mut ret: c_int = 0;
    struct kvm_cpu_context *cp = &vcpu.arch.guest_context;
    let mut funcid: c_ulong = cp.a6;
    switch (funcid) {
    case SBI_EXT_HSM_HART_START:
    ret = kvm_sbi_hsm_vcpu_start(vcpu);
    break;
    case SBI_EXT_HSM_HART_STOP:
    ret = kvm_sbi_hsm_vcpu_stop(vcpu);
    break;
    case SBI_EXT_HSM_HART_STATUS:
    ret = kvm_sbi_hsm_vcpu_get_status(vcpu);
    if (ret >= 0) {
    retdata.out_val = ret;
    retdata.err_val = 0;
    }
    return 0;
    case SBI_EXT_HSM_HART_SUSPEND:
    switch (lower_32_bits(cp.a0)) {
    case SBI_HSM_SUSPEND_RET_DEFAULT:
    kvm_riscv_vcpu_wfi(vcpu);
    break;
    case SBI_HSM_SUSPEND_NON_RET_DEFAULT:
    ret = SBI_ERR_NOT_SUPPORTED;
    break;
    default:
    ret = SBI_ERR_INVALID_PARAM;
    }
    break;
    default:
    ret = SBI_ERR_NOT_SUPPORTED;
    }
    retdata.err_val = ret;
    return 0;
    }
    const struct kvm_vcpu_sbi_extension vcpu_sbi_ext_hsm = {
    .extid_start = SBI_EXT_HSM,
    .extid_end = SBI_EXT_HSM,
    .handler = kvm_sbi_ext_hsm_handler,
    };
