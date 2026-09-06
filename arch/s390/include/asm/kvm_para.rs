//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/kvm_para.h
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
// definition for paravirtual devices on s390
//
// Copyright IBM Corp. 2008
//
// Author(s): Christian Borntraeger <borntraeger@de.ibm.com>
//
// Hypercalls for KVM on s390. The calling convention is similar to the
// s390 ABI, so we use R2-R6 for parameters 1-5. In addition we use R1
// as hypercall number and R7 as parameter 6. The return value is
// written to R2. We use the diagnose instruction as hypercall. To avoid
// conflicts with existing diagnoses for LPAR and z/VM, we do not use
// the instruction encoded number, but specify the number in R1 and
// use 0x500 as KVM hypercall
//
// Copyright IBM Corp. 2007,2008
// Author(s): Christian Borntraeger <borntraeger@de.ibm.com>
//

// Macro flag: #define HYPERCALL_FMT_0

// Macro flag: #define HYPERCALL_PARM_0

// Macro flag: #define HYPERCALL_REGS_0

// Macro flag: #define HYPERCALL_ARGS_0

// kvm on s390 is always paravirtualization enabled
// No feature bits are currently assigned for kvm on s390
