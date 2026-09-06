//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/kvm_booke_hv_asm.h
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
// Copyright 2010-2011 Freescale Semiconductor, Inc.
//

//
// All exceptions from guest state must go through KVM
// (except for those which are delivered directly to the guest) --
// there are no exceptions for which we fall through directly to
// the normal host handler.
//
// 32-bit host
// Expected inputs (normal exceptions):
// SCRATCH0 = saved r10
// r10 = thread struct
// r11 = appropriate SRR1 variant (currently used as scratch)
// r13 = saved CR
// *(r10 + THREAD_NORMSAVE(0)) = saved r11
// *(r10 + THREAD_NORMSAVE(2)) = saved r13
//
// Expected inputs (crit/mcheck/debug exceptions):
// appropriate SCRATCH = saved r8
// r8 = exception level stack frame
// r9 = *(r8 + _CCR) = saved CR
// r11 = appropriate SRR1 variant (currently used as scratch)
// *(r8 + GPR9) = saved r9
// *(r8 + GPR10) = saved r10 (r10 not yet clobbered)
// *(r8 + GPR11) = saved r11
//
// 64-bit host
// Expected inputs (GEN/GDBELL/DBG/CRIT/MC exception types):
// r10 = saved CR
// r13 = PACA_POINTER
// *(r13 + PACA_EX##type + EX_R10) = saved r10
// *(r13 + PACA_EX##type + EX_R11) = saved r11
// SPRN_SPRG_##type##_SCRATCH = saved r13
//
// Expected inputs (TLB exception type):
// r10 = saved CR
// r12 = extlb pointer
// r13 = PACA_POINTER
// *(r12 + EX_TLB_R10) = saved r10
// *(r12 + EX_TLB_R11) = saved r11
// *(r12 + EX_TLB_R13) = saved r13
// SPRN_SPRG_GEN_SCRATCH = saved r12
//
// Only the bolted version of TLB miss exception handlers is supported now.
//

