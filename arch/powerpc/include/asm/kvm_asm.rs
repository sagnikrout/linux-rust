//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/kvm_asm.h
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
// Copyright IBM Corp. 2008
//
// Authors: Hollis Blanchard <hollisb@us.ibm.com>
//

// IVPR must be 64KiB-aligned.
pub const VCPU_SIZE_ORDER: c_int = 4;

pub const BOOKE_INTERRUPT_CRITICAL: c_int = 0;
pub const BOOKE_INTERRUPT_MACHINE_CHECK: c_int = 1;
pub const BOOKE_INTERRUPT_DATA_STORAGE: c_int = 2;
pub const BOOKE_INTERRUPT_INST_STORAGE: c_int = 3;
pub const BOOKE_INTERRUPT_EXTERNAL: c_int = 4;
pub const BOOKE_INTERRUPT_ALIGNMENT: c_int = 5;
pub const BOOKE_INTERRUPT_PROGRAM: c_int = 6;
pub const BOOKE_INTERRUPT_FP_UNAVAIL: c_int = 7;
pub const BOOKE_INTERRUPT_SYSCALL: c_int = 8;
pub const BOOKE_INTERRUPT_AP_UNAVAIL: c_int = 9;
pub const BOOKE_INTERRUPT_DECREMENTER: c_int = 10;
pub const BOOKE_INTERRUPT_FIT: c_int = 11;
pub const BOOKE_INTERRUPT_WATCHDOG: c_int = 12;
pub const BOOKE_INTERRUPT_DTLB_MISS: c_int = 13;
pub const BOOKE_INTERRUPT_ITLB_MISS: c_int = 14;
pub const BOOKE_INTERRUPT_DEBUG: c_int = 15;
// E500

pub const BOOKE_INTERRUPT_SPE_UNAVAIL: c_int = 32;
pub const BOOKE_INTERRUPT_SPE_FP_DATA: c_int = 33;
pub const BOOKE_INTERRUPT_SPE_FP_ROUND: c_int = 34;

pub const BOOKE_INTERRUPT_ALTIVEC_UNAVAIL: c_int = 32;
pub const BOOKE_INTERRUPT_ALTIVEC_ASSIST: c_int = 33;

pub const BOOKE_INTERRUPT_PERFORMANCE_MONITOR: c_int = 35;
pub const BOOKE_INTERRUPT_DOORBELL: c_int = 36;
pub const BOOKE_INTERRUPT_DOORBELL_CRITICAL: c_int = 37;
// booke_hv
pub const BOOKE_INTERRUPT_GUEST_DBELL: c_int = 38;
pub const BOOKE_INTERRUPT_GUEST_DBELL_CRIT: c_int = 39;
pub const BOOKE_INTERRUPT_HV_SYSCALL: c_int = 40;
pub const BOOKE_INTERRUPT_HV_PRIV: c_int = 41;
pub const BOOKE_INTERRUPT_LRAT_ERROR: c_int = 42;
// book3s
pub const BOOK3S_INTERRUPT_SYSTEM_RESET: c_uint = 0x100;
pub const BOOK3S_INTERRUPT_MACHINE_CHECK: c_uint = 0x200;
pub const BOOK3S_INTERRUPT_DATA_STORAGE: c_uint = 0x300;
pub const BOOK3S_INTERRUPT_DATA_SEGMENT: c_uint = 0x380;
pub const BOOK3S_INTERRUPT_INST_STORAGE: c_uint = 0x400;
pub const BOOK3S_INTERRUPT_INST_SEGMENT: c_uint = 0x480;
pub const BOOK3S_INTERRUPT_EXTERNAL: c_uint = 0x500;
pub const BOOK3S_INTERRUPT_EXTERNAL_HV: c_uint = 0x502;
pub const BOOK3S_INTERRUPT_ALIGNMENT: c_uint = 0x600;
pub const BOOK3S_INTERRUPT_PROGRAM: c_uint = 0x700;
pub const BOOK3S_INTERRUPT_FP_UNAVAIL: c_uint = 0x800;
pub const BOOK3S_INTERRUPT_DECREMENTER: c_uint = 0x900;
pub const BOOK3S_INTERRUPT_HV_DECREMENTER: c_uint = 0x980;
pub const BOOK3S_INTERRUPT_NESTED_HV_DECREMENTER: c_uint = 0x1980;
pub const BOOK3S_INTERRUPT_DOORBELL: c_uint = 0xa00;
pub const BOOK3S_INTERRUPT_SYSCALL: c_uint = 0xc00;
pub const BOOK3S_INTERRUPT_TRACE: c_uint = 0xd00;
pub const BOOK3S_INTERRUPT_H_DATA_STORAGE: c_uint = 0xe00;
pub const BOOK3S_INTERRUPT_H_INST_STORAGE: c_uint = 0xe20;
pub const BOOK3S_INTERRUPT_H_EMUL_ASSIST: c_uint = 0xe40;
pub const BOOK3S_INTERRUPT_HMI: c_uint = 0xe60;
pub const BOOK3S_INTERRUPT_H_DOORBELL: c_uint = 0xe80;
pub const BOOK3S_INTERRUPT_H_VIRT: c_uint = 0xea0;
pub const BOOK3S_INTERRUPT_PERFMON: c_uint = 0xf00;
pub const BOOK3S_INTERRUPT_ALTIVEC: c_uint = 0xf20;
pub const BOOK3S_INTERRUPT_VSX: c_uint = 0xf40;
pub const BOOK3S_INTERRUPT_FAC_UNAVAIL: c_uint = 0xf60;
pub const BOOK3S_INTERRUPT_H_FAC_UNAVAIL: c_uint = 0xf80;
// book3s_hv
pub const BOOK3S_INTERRUPT_HV_SOFTPATCH: c_uint = 0x1500;
//
// Special trap used to indicate to host that this is a
// passthrough interrupt that could not be handled
// completely in the guest.
//
pub const BOOK3S_INTERRUPT_HV_RM_HARD: c_uint = 0x5555;
pub const BOOK3S_IRQPRIO_SYSTEM_RESET: c_int = 0;
pub const BOOK3S_IRQPRIO_DATA_SEGMENT: c_int = 1;
pub const BOOK3S_IRQPRIO_INST_SEGMENT: c_int = 2;
pub const BOOK3S_IRQPRIO_DATA_STORAGE: c_int = 3;
pub const BOOK3S_IRQPRIO_INST_STORAGE: c_int = 4;
pub const BOOK3S_IRQPRIO_ALIGNMENT: c_int = 5;
pub const BOOK3S_IRQPRIO_PROGRAM: c_int = 6;
pub const BOOK3S_IRQPRIO_FP_UNAVAIL: c_int = 7;
pub const BOOK3S_IRQPRIO_ALTIVEC: c_int = 8;
pub const BOOK3S_IRQPRIO_VSX: c_int = 9;
pub const BOOK3S_IRQPRIO_FAC_UNAVAIL: c_int = 10;
pub const BOOK3S_IRQPRIO_SYSCALL: c_int = 11;
pub const BOOK3S_IRQPRIO_MACHINE_CHECK: c_int = 12;
pub const BOOK3S_IRQPRIO_DEBUG: c_int = 13;
pub const BOOK3S_IRQPRIO_EXTERNAL: c_int = 14;
pub const BOOK3S_IRQPRIO_DECREMENTER: c_int = 15;
pub const BOOK3S_IRQPRIO_PERFORMANCE_MONITOR: c_int = 16;
pub const BOOK3S_IRQPRIO_MAX: c_int = 17;
pub const BOOK3S_HFLAG_DCBZ32: c_uint = 0x1;
pub const BOOK3S_HFLAG_SLB: c_uint = 0x2;
pub const BOOK3S_HFLAG_PAIRED_SINGLE: c_uint = 0x4;
pub const BOOK3S_HFLAG_NATIVE_PS: c_uint = 0x8;
pub const BOOK3S_HFLAG_MULTI_PGSIZE: c_uint = 0x10;
pub const BOOK3S_HFLAG_NEW_TLBIE: c_uint = 0x20;
pub const BOOK3S_HFLAG_SPLIT_HACK: c_uint = 0x40;

pub const RESUME_GUEST: c_int = 0;

pub const KVM_GUEST_MODE_NONE: c_int = 0;
pub const KVM_GUEST_MODE_GUEST: c_int = 1;
pub const KVM_GUEST_MODE_SKIP: c_int = 2;
pub const KVM_GUEST_MODE_GUEST_HV: c_int = 3;
pub const KVM_GUEST_MODE_HOST_HV: c_int = 4;

// Extract PO and XOP opcode fields
pub const PO_XOP_OPCODE_MASK: c_uint = 0xfc0007fe;
