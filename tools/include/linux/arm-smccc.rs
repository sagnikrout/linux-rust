//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/arm-smccc.h
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
// Copyright (c) 2015, Linaro Limited
//

//
// This file provides common defines for ARM SMC Calling Convention as
// specified in
// https://developer.arm.com/docs/den0028/latest
//
// This code is up-to-date with version DEN 0028 C
//

pub const ARM_SMCCC_TYPE_SHIFT: c_int = 31;
pub const ARM_SMCCC_SMC_32: c_int = 0;
pub const ARM_SMCCC_SMC_64: c_int = 1;
pub const ARM_SMCCC_CALL_CONV_SHIFT: c_int = 30;
pub const ARM_SMCCC_OWNER_MASK: c_uint = 0x3F;
pub const ARM_SMCCC_OWNER_SHIFT: c_int = 24;
pub const ARM_SMCCC_FUNC_MASK: c_uint = 0xFFFF;

pub const ARM_SMCCC_OWNER_ARCH: c_int = 0;
pub const ARM_SMCCC_OWNER_CPU: c_int = 1;
pub const ARM_SMCCC_OWNER_SIP: c_int = 2;
pub const ARM_SMCCC_OWNER_OEM: c_int = 3;
pub const ARM_SMCCC_OWNER_STANDARD: c_int = 4;
pub const ARM_SMCCC_OWNER_STANDARD_HYP: c_int = 5;
pub const ARM_SMCCC_OWNER_VENDOR_HYP: c_int = 6;
pub const ARM_SMCCC_OWNER_TRUSTED_APP: c_int = 48;
pub const ARM_SMCCC_OWNER_TRUSTED_APP_END: c_int = 49;
pub const ARM_SMCCC_OWNER_TRUSTED_OS: c_int = 50;
pub const ARM_SMCCC_OWNER_TRUSTED_OS_END: c_int = 63;
pub const ARM_SMCCC_FUNC_QUERY_CALL_UID: c_uint = 0xff01;
pub const ARM_SMCCC_QUIRK_NONE: c_int = 0;

pub const ARM_SMCCC_VERSION_1_0: c_uint = 0x10000;
pub const ARM_SMCCC_VERSION_1_1: c_uint = 0x10001;
pub const ARM_SMCCC_VERSION_1_2: c_uint = 0x10002;
pub const ARM_SMCCC_VERSION_1_3: c_uint = 0x10003;
pub const ARM_SMCCC_1_3_SVE_HINT: c_uint = 0x10000;

// KVM UID value: 28b46fb6-2ec5-11e9-a9ca-4b564d003a74
pub const ARM_SMCCC_VENDOR_HYP_UID_KVM_REG_0: c_uint = 0xb66fb428U;
pub const ARM_SMCCC_VENDOR_HYP_UID_KVM_REG_1: c_uint = 0xe911c52eU;
pub const ARM_SMCCC_VENDOR_HYP_UID_KVM_REG_2: c_uint = 0x564bcaa9U;
pub const ARM_SMCCC_VENDOR_HYP_UID_KVM_REG_3: c_uint = 0x743a004dU;
// KVM "vendor specific" services
pub const ARM_SMCCC_KVM_FUNC_FEATURES: c_int = 0;
pub const ARM_SMCCC_KVM_FUNC_PTP: c_int = 1;
pub const ARM_SMCCC_KVM_FUNC_FEATURES_2: c_int = 127;
pub const ARM_SMCCC_KVM_NUM_FUNCS: c_int = 128;

pub const SMCCC_ARCH_WORKAROUND_RET_UNAFFECTED: c_int = 1;
//
// ptp_kvm is a feature used for time sync between vm and host.
// ptp_kvm module in guest kernel will get service from host using
// this hypercall ID.
//

// ptp_kvm counter type ID
pub const KVM_PTP_VIRT_COUNTER: c_int = 0;
pub const KVM_PTP_PHYS_COUNTER: c_int = 1;
// Paravirtualised time calls (defined by ARM DEN0057A)

// TRNG entropy source calls (defined by ARM DEN0098)

//
// Return codes defined in ARM DEN 0070A
// ARM DEN 0070A is now merged/consolidated into ARM DEN 0028 C
//
pub const SMCCC_RET_SUCCESS: c_int = 0;

