//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kvm/vcpu_sbi_system.c
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
// Copyright (c) 2024 Ventana Micro Systems Inc.
//

    static int kvm_sbi_ext_susp_handler(struct kvm_vcpu *vcpu, struct kvm_run *run,
    struct kvm_vcpu_sbi_return *retdata)
    {
    struct kvm_cpu_context *cp = &vcpu.arch.guest_context;
    let mut funcid: c_ulong = cp.a6;
    unsigned long hva, i;
    struct kvm_vcpu *tmp;
    switch (funcid) {
    case SBI_EXT_SUSP_SYSTEM_SUSPEND:
    if (lower_32_bits(cp.a0) != SBI_SUSP_SLEEP_TYPE_SUSPEND_TO_RAM) {
    retdata.err_val = SBI_ERR_INVALID_PARAM;
    return 0;
    }
    if (!(cp.sstatus & SR_SPP)) {
    retdata.err_val = SBI_ERR_FAILURE;
    return 0;
    }
    hva = kvm_vcpu_gfn_to_hva_prot(vcpu, cp.a1 >> PAGE_SHIFT, core::ptr::null_mut());
    if (kvm_is_error_hva(hva)) {
    retdata.err_val = SBI_ERR_INVALID_ADDRESS;
    return 0;
    }
//
// Check that all other vCPUs are stopped before entering
// system suspend.
//
// There is a known TOCTOU race here: a concurrent HSM
// HART_START on another vCPU can start a vCPU after it
// has already passed this check, violating the invariant.
//
// We do not fix this because:
// 1. Triggering the race requires a pathological guest.
// 2. Only guest state is at risk, not host integrity.
// 3. Userspace can double-check vCPU states before
// proceeding with suspend.
//
    kvm_for_each_vcpu(i, tmp, vcpu.kvm) {
    if (tmp == vcpu)
    continue;
    if (!kvm_riscv_vcpu_stopped(tmp)) {
    retdata.err_val = SBI_ERR_DENIED;
    return 0;
    }
    }
    kvm_riscv_vcpu_sbi_request_reset(vcpu, cp.a1, cp.a2);
// userspace provides the suspend implementation
    return kvm_riscv_vcpu_sbi_forward_handler(vcpu, run, retdata);
    default:
    retdata.err_val = SBI_ERR_NOT_SUPPORTED;
    break;
    }
    return 0;
    }
    const struct kvm_vcpu_sbi_extension vcpu_sbi_ext_susp = {
    .extid_start = SBI_EXT_SUSP,
    .extid_end = SBI_EXT_SUSP,
    .default_disabled = true,
    .handler = kvm_sbi_ext_susp_handler,
    };
