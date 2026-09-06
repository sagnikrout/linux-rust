//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/kvm_para.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// This header file provides a method for making a hypercall to the host
// Architectures should define:
// - kvm_hypercall0, kvm_hypercall1...
// - kvm_arch_para_features
// - kvm_para_available
//
// Return values for hypercalls
pub const KVM_ENOSYS: c_int = 1000;

pub const KVM_EOPNOTSUPP: c_int = 95;
pub const KVM_HC_VAPIC_POLL_IRQ: c_int = 1;
pub const KVM_HC_MMU_OP: c_int = 2;
pub const KVM_HC_FEATURES: c_int = 3;
pub const KVM_HC_PPC_MAP_MAGIC_PAGE: c_int = 4;
pub const KVM_HC_KICK_CPU: c_int = 5;
pub const KVM_HC_MIPS_GET_CLOCK_FREQ: c_int = 6;
pub const KVM_HC_MIPS_EXIT_VM: c_int = 7;
pub const KVM_HC_MIPS_CONSOLE_OUTPUT: c_int = 8;
pub const KVM_HC_CLOCK_PAIRING: c_int = 9;
pub const KVM_HC_SEND_IPI: c_int = 10;
pub const KVM_HC_SCHED_YIELD: c_int = 11;
pub const KVM_HC_MAP_GPA_RANGE: c_int = 12;
//
// hypercalls use architecture specific
//

