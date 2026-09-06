//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/kernel/setup.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Prototypes for functions that are shared between setup_(32|64|common).c
//
// Copyright 2016 Michael Ellerman, IBM Corporation.
//
extern "C" {
    pub fn initialize_cache_info();
}
extern "C" {
    pub fn irqstack_early_init();
}

extern "C" {
    pub fn setup_power_save();
}

extern "C" {
    pub fn check_smt_enabled();
}

extern "C" {
    pub fn setup_tlb_core_data();
}

extern "C" {
    pub fn exc_lvl_early_init();
}

extern "C" {
    pub fn emergency_stack_init();
}

extern "C" {
    pub fn ppc64_bolted_size() -> u64;
}
// Default SPR values from firmware/kexec

//
// Having this in kvm_ppc.h makes include dependencies too
// tricky to solve for setup-common.c so have it here.
//

extern "C" {
    pub fn kvm_cma_reserve();
}

extern "C" {
    pub fn cpu_temp(cpu: c_ulong) -> u32;
}
extern "C" {
    pub fn cpu_temp_both(cpu: c_ulong) -> u32;
}
extern "C" {
    pub fn tau_interrupts(cpu: c_ulong) -> u32;
}

