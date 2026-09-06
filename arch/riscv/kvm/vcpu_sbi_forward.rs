//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kvm/vcpu_sbi_forward.c
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
// Copyright (c) 2025 Ventana Micro Systems Inc.
//

    const struct kvm_vcpu_sbi_extension vcpu_sbi_ext_experimental = {
    .extid_start = SBI_EXT_EXPERIMENTAL_START,
    .extid_end = SBI_EXT_EXPERIMENTAL_END,
    .handler = kvm_riscv_vcpu_sbi_forward_handler,
    };
    const struct kvm_vcpu_sbi_extension vcpu_sbi_ext_vendor = {
    .extid_start = SBI_EXT_VENDOR_START,
    .extid_end = SBI_EXT_VENDOR_END,
    .handler = kvm_riscv_vcpu_sbi_forward_handler,
    };
    const struct kvm_vcpu_sbi_extension vcpu_sbi_ext_dbcn = {
    .extid_start = SBI_EXT_DBCN,
    .extid_end = SBI_EXT_DBCN,
    .default_disabled = true,
    .handler = kvm_riscv_vcpu_sbi_forward_handler,
    };
    const struct kvm_vcpu_sbi_extension vcpu_sbi_ext_mpxy = {
    .extid_start = SBI_EXT_MPXY,
    .extid_end = SBI_EXT_MPXY,
    .default_disabled = true,
    .handler = kvm_riscv_vcpu_sbi_forward_handler,
    };
