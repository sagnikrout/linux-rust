//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/kvm/hyp/include/hyp/adjust_pc.h
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
// Guest PC manipulation helpers
//
// Copyright (C) 2012,2013 - ARM Ltd
// Copyright (C) 2020 - Google LLC
// Author: Marc Zyngier <maz@kernel.org>
//

// vcpu_pc(vcpu) += 4;
// vcpu_cpsr(vcpu) &= ~PSR_BTYPE_MASK;
// advance the singlestep state machine
// vcpu_cpsr(vcpu) &= ~DBG_SPSR_SS;
//
// Skip an instruction which has been emulated at hyp while most guest sysregs
// are live.
//
// vcpu_pc(vcpu) = read_sysreg_el2(SYS_ELR);
//
// Skip an instruction while host sysregs are live.
// Assumes host is always 64-bit.
//
