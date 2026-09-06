//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cc_platform.h
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
// Confidential Computing Platform Capability checks
//
// Copyright (C) 2021 Advanced Micro Devices, Inc.
//
// Author: Tom Lendacky <thomas.lendacky@amd.com>
//

//
// enum cc_attr - Confidential computing attributes
//
// These attributes represent confidential computing features that are
// currently active.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_attr {
//
// @CC_ATTR_MEM_ENCRYPT: Memory encryption is active
//
// The platform/OS is running with active memory encryption. This
// includes running either as a bare-metal system or a hypervisor
// and actively using memory encryption or as a guest/virtual machine
// and actively using memory encryption.
//
// Examples include SME, SEV and SEV-ES.
//
    CC_ATTR_MEM_ENCRYPT,

//
// @CC_ATTR_HOST_MEM_ENCRYPT: Host memory encryption is active
//
// The platform/OS is running as a bare-metal system or a hypervisor
// and actively using memory encryption.
//
// Examples include SME.
//
    CC_ATTR_HOST_MEM_ENCRYPT,

//
// @CC_ATTR_GUEST_MEM_ENCRYPT: Guest memory encryption is active
//
// The platform/OS is running as a guest/virtual machine and actively
// using memory encryption.
//
// Examples include SEV and SEV-ES.
//
    CC_ATTR_GUEST_MEM_ENCRYPT,

//
// @CC_ATTR_GUEST_STATE_ENCRYPT: Guest state encryption is active
//
// The platform/OS is running as a guest/virtual machine and actively
// using memory encryption and register state encryption.
//
// Examples include SEV-ES.
//
    CC_ATTR_GUEST_STATE_ENCRYPT,

//
// @CC_ATTR_GUEST_UNROLL_STRING_IO: String I/O is implemented with
// IN/OUT instructions
//
// The platform/OS is running as a guest/virtual machine and uses
// IN/OUT instructions in place of string I/O.
//
// Examples include TDX guest & SEV.
//
    CC_ATTR_GUEST_UNROLL_STRING_IO,

//
// @CC_ATTR_GUEST_SEV_SNP: Guest SNP is active.
//
// The platform/OS is running as a guest/virtual machine and actively
// using AMD SEV-SNP features.
//
    CC_ATTR_GUEST_SEV_SNP,

//
// @CC_ATTR_GUEST_SNP_SECURE_TSC: SNP Secure TSC is active.
//
// The platform/OS is running as a guest/virtual machine and actively
// using AMD SEV-SNP Secure TSC feature.
//
    CC_ATTR_GUEST_SNP_SECURE_TSC,

//
// @CC_ATTR_HOST_SEV_SNP: AMD SNP enabled on the host.
//
// The host kernel is running with the necessary features
// enabled to run SEV-SNP guests.
//
    CC_ATTR_HOST_SEV_SNP,

//
// @CC_ATTR_SNP_SECURE_AVIC: Secure AVIC mode is active.
//
// The host kernel is running with the necessary features enabled
// to run SEV-SNP guests with full Secure AVIC capabilities.
//
    CC_ATTR_SNP_SECURE_AVIC,
}

//
// cc_platform_has() - Checks if the specified cc_attr attribute is active
// @attr: Confidential computing attribute to check
//
// The cc_platform_has() function will return an indicator as to whether the
// specified Confidential Computing attribute is currently active.
//
// Context: Any context
// Return:
// * TRUE  - Specified Confidential Computing attribute is active
// * FALSE - Specified Confidential Computing attribute is not active
//
extern "C" {
    pub fn cc_platform_has(attr: cc_attr) -> bool;
}
extern "C" {
    pub fn cc_platform_set(attr: cc_attr);
}
extern "C" {
    pub fn cc_platform_clear(attr: cc_attr);
}

