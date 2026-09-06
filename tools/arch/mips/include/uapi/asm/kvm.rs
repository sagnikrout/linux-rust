//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/mips/include/uapi/asm/kvm.h
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
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Copyright (C) 2012  MIPS Technologies, Inc.  All rights reserved.
// Copyright (C) 2013 Cavium, Inc.
// Authors: Sanjay Lal <sanjayl@kymasys.com>
//

//
// KVM MIPS specific structures and definitions.
//
// Some parts derived from the x86 version of this file.
//
// for KVM_GET_REGS and KVM_SET_REGS
//
// If Config[AT] is zero (32-bit CPU), the register contents are
// stored in the lower 32-bits of the struct kvm_regs fields and sign
// extended to 64-bits.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_regs {
// out (KVM_GET_REGS) / in (KVM_SET_REGS)
    pub gpr: [__u64; 32],
    pub hi: __u64,
    pub lo: __u64,
    pub pc: __u64,
}

//
// for KVM_GET_FPU and KVM_SET_FPU
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_fpu {
}

//
// For MIPS, we use KVM_SET_ONE_REG and KVM_GET_ONE_REG to access various
// registers.  The id field is broken down as follows:
//
// bits[63..52] - As per linux/kvm.h
// bits[51..32] - Must be zero.
// bits[31..16] - Register set.
//
// Register set = 0: GP registers from kvm_regs (see definitions below).
//
// Register set = 1: CP0 registers.
// bits[15..8]  - Must be zero.
// bits[7..3]   - Register 'rd'  index.
// bits[2..0]   - Register 'sel' index.
//
// Register set = 2: KVM specific registers (see definitions below).
//
// Register set = 3: FPU / MSA registers (see definitions below).
//
// Other sets registers may be added in the future.  Each set would
// have its own identifier in bits[31..16].
//

//
// KVM_REG_MIPS_GP - General purpose registers from kvm_regs.
//

//
// KVM_REG_MIPS_KVM - KVM specific control registers.
//
// CP0_Count control
// DC:    Set 0: Master disable CP0_Count and set COUNT_RESUME to now
// Set 1: Master re-enable CP0_Count with unchanged bias, handling timer
// interrupts since COUNT_RESUME
// This can be used to freeze the timer to get a consistent snapshot of
// the CP0_Count and timer interrupt pending state, while also resuming
// safely without losing time or guest timer interrupts.
// Other: Reserved, do not change.
//

pub const KVM_REG_MIPS_COUNT_CTL_DC: c_uint = 0x00000001;
//
// CP0_Count resume monotonic nanoseconds
// The monotonic nanosecond time of the last set of COUNT_CTL.DC (master
// disable). Any reads and writes of Count related registers while
// COUNT_CTL.DC=1 will appear to occur at this time. When COUNT_CTL.DC is
// cleared again (master enable) any timer interrupts since this time will be
// emulated.
// Modifications to times in the future are rejected.
//

//
// CP0_Count rate in Hz
// Specifies the rate of the CP0_Count timer in Hz. Modifications occur without
// discontinuities in CP0_Count.
//

//
// KVM_REG_MIPS_FPU - Floating Point and MIPS SIMD Architecture (MSA) registers.
//
// bits[15..8]  - Register subset (see definitions below).
// bits[7..5]   - Must be zero.
// bits[4..0]   - Register number within register subset.
//

//
// KVM_REG_MIPS_FPR - Floating point / Vector registers.
//

//
// KVM_REG_MIPS_FCR - Floating point control registers.
//

//
// KVM_REG_MIPS_MSACR - MIPS SIMD Architecture (MSA) control registers.
//

//
// KVM MIPS specific structures and definitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_debug_exit_arch {
    pub epc: __u64,
}

// for KVM_SET_GUEST_DEBUG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_guest_debug_arch {
}

// definition of registers in kvm_run
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sync_regs {
}

// dummy definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sregs {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_mips_interrupt {
// in
    pub cpu: __u32,
    pub irq: __u32,
}
