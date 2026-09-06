//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kvm/vcpu_sbi_base.c
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

    static int kvm_sbi_ext_base_handler(struct kvm_vcpu *vcpu, struct kvm_run *run,
    struct kvm_vcpu_sbi_return *retdata)
    {
    struct kvm_cpu_context *cp = &vcpu.arch.guest_context;
    const struct kvm_vcpu_sbi_extension *sbi_ext;
    unsigned long *out_val = &retdata.out_val;
    switch (cp.a6) {
    case SBI_EXT_BASE_GET_SPEC_VERSION:
// out_val = (KVM_SBI_VERSION_MAJOR <<
    SBI_SPEC_VERSION_MAJOR_SHIFT) |
    KVM_SBI_VERSION_MINOR;
    break;
    case SBI_EXT_BASE_GET_IMP_ID:
// out_val = KVM_SBI_IMPID;
    break;
    case SBI_EXT_BASE_GET_IMP_VERSION:
// out_val = LINUX_VERSION_CODE;
    break;
    case SBI_EXT_BASE_PROBE_EXT:
    if ((cp.a0 >= SBI_EXT_EXPERIMENTAL_START &&
    cp.a0 <= SBI_EXT_EXPERIMENTAL_END) ||
    (cp.a0 >= SBI_EXT_VENDOR_START &&
    cp.a0 <= SBI_EXT_VENDOR_END)) {
//
// For experimental/vendor extensions
// forward it to the userspace
//
    return kvm_riscv_vcpu_sbi_forward_handler(vcpu, run, retdata);
    } else {
    sbi_ext = kvm_vcpu_sbi_find_ext(vcpu, cp.a0);
// out_val = sbi_ext && sbi_ext->probe ?
    sbi_ext.probe(vcpu) : !!sbi_ext;
    }
    break;
    case SBI_EXT_BASE_GET_MVENDORID:
// out_val = vcpu->arch.mvendorid;
    break;
    case SBI_EXT_BASE_GET_MARCHID:
// out_val = vcpu->arch.marchid;
    break;
    case SBI_EXT_BASE_GET_MIMPID:
// out_val = vcpu->arch.mimpid;
    break;
    default:
    retdata.err_val = SBI_ERR_NOT_SUPPORTED;
    break;
    }
    return 0;
    }
    const struct kvm_vcpu_sbi_extension vcpu_sbi_ext_base = {
    .extid_start = SBI_EXT_BASE,
    .extid_end = SBI_EXT_BASE,
    .handler = kvm_sbi_ext_base_handler,
    };
