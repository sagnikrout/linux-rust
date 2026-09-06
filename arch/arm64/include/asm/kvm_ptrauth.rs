//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/kvm_ptrauth.h
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
// arch/arm64/include/asm/kvm_ptrauth.h: Guest/host ptrauth save/restore
// Copyright 2019 Arm Limited
// Authors: Mark Rutland <mark.rutland@arm.com>
// Amit Daniel Kachhap <amit.kachhap@arm.com>
//

//
// CPU_AP*_EL1 values exceed immediate offset range (512) for stp
// instruction so below macros takes CPU_APIAKEYLO_EL1 as base and
// calculates the offset of the keys from this base to avoid an extra add
// instruction. These macros assumes the keys offsets follow the order of
// the sysreg enum in kvm_host.h.
//
// Both ptrauth_switch_to_guest and ptrauth_switch_to_hyp macros will
// check for the presence ARM64_HAS_ADDRESS_AUTH, which is defined as
// (ARM64_HAS_ADDRESS_AUTH_ARCH || ARM64_HAS_ADDRESS_AUTH_IMP_DEF) and
// then proceed ahead with the save/restore of Pointer Authentication
// key registers if enabled for the guest.
//

