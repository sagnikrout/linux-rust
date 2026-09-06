//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/bhyve.c
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
// FreeBSD Bhyve guest enlightenments
//
// Copyright © 2025 Amazon.com, Inc. or its affiliates.
//
// Author: David Woodhouse <dwmw2@infradead.org>
//

    static uint32_t bhyve_cpuid_base;
    static uint32_t bhyve_cpuid_max;

pub const CPUID_BHYVE_FEATURES: c_uint = 0x40000001;
// Features advertised in CPUID_BHYVE_FEATURES %eax
// MSI Extended Dest ID

#[no_mangle]
unsafe extern "C" fn bhyve_detect() -> uint32_t __init {
    static uint32_t __init bhyve_detect(void)
    {
    if (!cpu_feature_enabled(X86_FEATURE_HYPERVISOR))
    return 0;
    bhyve_cpuid_base = cpuid_base_hypervisor(BHYVE_SIGNATURE, 0);
    if (!bhyve_cpuid_base)
    return 0;
    bhyve_cpuid_max = cpuid_eax(bhyve_cpuid_base);
    return bhyve_cpuid_max;
    }
#[no_mangle]
unsafe extern "C" fn bhyve_features() -> u32 {
    static uint32_t bhyve_features(void)
    {
    let mut cpuid_leaf: c_uint = bhyve_cpuid_base | CPUID_BHYVE_FEATURES;
    if (bhyve_cpuid_max < cpuid_leaf)
    return 0;
    return cpuid_eax(cpuid_leaf);
    }
#[no_mangle]
unsafe extern "C" fn bhyve_ext_dest_id() -> bool __init {
    static bool __init bhyve_ext_dest_id(void)
    {
    return !!(bhyve_features() & CPUID_BHYVE_FEAT_EXT_DEST_ID);
    }
#[no_mangle]
unsafe extern "C" fn bhyve_x2apic_available() -> bool __init {
    static bool __init bhyve_x2apic_available(void)
    {
    return true;
    }
    const struct hypervisor_x86 x86_hyper_bhyve __refconst = {
    .name			= "Bhyve",
    .detect			= bhyve_detect,
    .init.init_platform	= x86_init_noop,
    .init.x2apic_available	= bhyve_x2apic_available,
    .init.msi_ext_dest_id	= bhyve_ext_dest_id,
    };
