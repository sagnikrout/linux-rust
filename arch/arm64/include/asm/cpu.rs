//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/cpu.h
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
// Copyright (C) 2014 ARM Ltd.
//

//
// Records attributes of an individual CPU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuinfo_32bit {
    pub reg_id_dfr0: u32,
    pub reg_id_dfr1: u32,
    pub reg_id_isar0: u32,
    pub reg_id_isar1: u32,
    pub reg_id_isar2: u32,
    pub reg_id_isar3: u32,
    pub reg_id_isar4: u32,
    pub reg_id_isar5: u32,
    pub reg_id_isar6: u32,
    pub reg_id_mmfr0: u32,
    pub reg_id_mmfr1: u32,
    pub reg_id_mmfr2: u32,
    pub reg_id_mmfr3: u32,
    pub reg_id_mmfr4: u32,
    pub reg_id_mmfr5: u32,
    pub reg_id_pfr0: u32,
    pub reg_id_pfr1: u32,
    pub reg_id_pfr2: u32,
    pub reg_mvfr0: u32,
    pub reg_mvfr1: u32,
    pub reg_mvfr2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuinfo_arm64 {
    pub kobj: kobject,
    pub reg_ctr: u64,
    pub reg_cntfrq: u64,
    pub reg_dczid: u64,
    pub reg_midr: u64,
    pub reg_revidr: u64,
    pub reg_aidr: u64,
    pub reg_gmid: u64,
    pub reg_smidr: u64,
    pub reg_mpamidr: u64,
    pub reg_id_aa64dfr0: u64,
    pub reg_id_aa64dfr1: u64,
    pub reg_id_aa64isar0: u64,
    pub reg_id_aa64isar1: u64,
    pub reg_id_aa64isar2: u64,
    pub reg_id_aa64isar3: u64,
    pub reg_id_aa64mmfr0: u64,
    pub reg_id_aa64mmfr1: u64,
    pub reg_id_aa64mmfr2: u64,
    pub reg_id_aa64mmfr3: u64,
    pub reg_id_aa64mmfr4: u64,
    pub reg_id_aa64pfr0: u64,
    pub reg_id_aa64pfr1: u64,
    pub reg_id_aa64pfr2: u64,
    pub reg_id_aa64zfr0: u64,
    pub reg_id_aa64smfr0: u64,
    pub reg_id_aa64fpfr0: u64,
    pub aarch32: cpuinfo_32bit,
}

extern "C" {
    pub fn cpuinfo_store_cpu();
}
extern "C" {
    pub fn cpuinfo_store_boot_cpu() -> void __init;
}
extern "C" {
    pub fn init_cpu_features(info: *mut cpuinfo_arm64) -> void __init;
}
extern "C" {
    pub fn gmid_el1_accessible(info: *const cpuinfo_arm64) -> bool;
}
