//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kernel/cpu/cpu.h
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

// attempt to consolidate cpu attributes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_dev {
    pub c_vendor: *const c_char,
// some have two possibilities for cpuid string
    pub c_ident: [*const c_char; 2],
    pub ): *mut *mut void (c_early_init)(struct cpuinfo_x86,
    pub ): *mut *mut void (c_bsp_init)(struct cpuinfo_x86,
    pub ): *mut *mut void (c_init)(struct cpuinfo_x86,
    pub ): *mut *mut void (c_identify)(struct cpuinfo_x86,
    pub ): *mut *mut void (c_detect_tlb)(struct cpuinfo_x86,
    pub c_x86_vendor: c_int,

// Optional vendor specific routine to obtain the cache size.
    pub int): unsigned,
// Family/stepping-based lookup table for model names.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct legacy_cpu_model_info {
    pub family: c_int,
    pub model_names: [*const c_char; 16],
    pub legacy_models: [}; 5],
}

// const __x86_cpu_dev_end[];

extern "C" {
    pub fn tsx_init() -> void __init;
}
extern "C" {
    pub fn tsx_ap_init();
}
extern "C" {
    pub fn intel_unlock_cpuid_leafs(c: *mut cpuinfo_x86);
}

extern "C" {
    pub fn init_spectral_chicken(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn get_cpu_cap(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn get_cpu_address_sizes(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn cpu_detect_cache_sizes(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn init_scattered_cpuid_features(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn init_intel_cacheinfo(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn init_amd_cacheinfo(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn init_hygon_cacheinfo(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn check_null_seg_clears_base(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn cacheinfo_amd_init_llc_id(c: *mut cpuinfo_x86, die_id: u16);
}
extern "C" {
    pub fn cacheinfo_hygon_init_llc_id(c: *mut cpuinfo_x86);
}

extern "C" {
    pub fn cpu_select_mitigations();
}
extern "C" {
    pub fn x86_spec_ctrl_setup_ap();
}
extern "C" {
    pub fn update_srbds_msr();
}
extern "C" {
    pub fn update_gds_msr();
}
