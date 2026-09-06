//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/arch_timer.h
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
// arch/arm64/include/asm/arch_timer.h
//
// Copyright (C) 2012 ARM Ltd.
// Author: Marc Zyngier <marc.zyngier@arm.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arch_timer_erratum_match_type {
    ate_match_dt,
    ate_match_local_cap_id,
    ate_match_acpi_oem_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_timer_erratum_workaround {
    pub match_type: arch_timer_erratum_match_type,
    pub id: *const c_void,
    pub desc: *const c_char,
    pub (*read_cntpct_el0)(void): *mut u64,
    pub (*read_cntvct_el0)(void): *mut u64,
    pub ): *mut *mut int (set_next_event_phys)(unsigned long, struct clock_event_device,
    pub ): *mut *mut int (set_next_event_virt)(unsigned long, struct clock_event_device,
    pub disable_compat_vdso: bool,
}

//
// These register accessors are marked inline so the compiler can
// nicely work out which register we want, and chuck away the rest of
// the code.
//
extern "C" {
    pub fn read_sysreg(_arg: cntp_ctl_el0) -> return;
}
extern "C" {
    pub fn read_sysreg(_arg: cntv_ctl_el0) -> return;
}
extern "C" {
    pub fn read_sysreg(_arg: cntfrq_el0) -> return;
}
extern "C" {
    pub fn read_sysreg(_arg: cntkctl_el1) -> return;
}

extern "C" {
    pub fn cpu_have_named_feature(_arg: EVTSTRM) -> return;
}
