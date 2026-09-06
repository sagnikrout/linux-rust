//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/arch_gicv3.h
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
// arch/arm64/include/asm/arch_gicv3.h
//
// Copyright (C) 2015 ARM Ltd.
//

//
// Low-level accessors
//
// These system registers are 32 bits, but we make sure that the compiler
// sets the GP register's most significant bits to 0 with an explicit cast.
//
// Cavium ThunderX erratum 23154
//
// The gicv3 of ThunderX requires a modified version for reading the
// IAR status to ensure data synchronization (access to icc_iar1_el1
// is not sync'ed before and after).
//
// Erratum 38545
//
// When a IAR register read races with a GIC interrupt RELEASE event,
// GIC-CPU interface could wrongly return a valid INTID to the CPU
// for an interrupt that is already released(non activated) instead of 0x3ff.
//
// To workaround this, return a valid interrupt ID only if there is a change
// in the active priority list after the IAR read.
//
// Common function used for both the workarounds since,
// 1. On Thunderx 88xx 1.x both erratas are applicable.
// 2. Having extra nops doesn't add any side effects for Silicons where
// erratum 23154 is not applicable.
//
// Max priority groups implemented is only 32
extern "C" {
    pub fn gic_read_iar_cavium_thunderx() -> return;
}
extern "C" {
    pub fn gic_read_iar_common() -> return;
}
extern "C" {
    pub fn read_sysreg_s(_arg: SYS_ICC_CTLR_EL1) -> return;
}
extern "C" {
    pub fn read_sysreg_s(_arg: SYS_ICC_SRE_EL1) -> return;
}
extern "C" {
    pub fn read_sysreg_s(_arg: SYS_ICC_PMR_EL1) -> return;
}
extern "C" {
    pub fn read_sysreg_s(_arg: SYS_ICC_RPR_EL1) -> return;
}

extern "C" {
    pub fn system_uses_irq_prio_masking() -> return;
}
extern "C" {
    pub fn volatile(daifclr: "msr, "memory": #3" : : :) -> asm;
}
extern "C" {
    pub fn cpus_have_cap(_arg: ARM64_HAS_GIC_PRIO_RELAXED_SYNC) -> return;
}

