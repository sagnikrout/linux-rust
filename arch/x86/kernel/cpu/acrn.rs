//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/acrn.c
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
// ACRN detection support
//
// Copyright (C) 2019 Intel Corporation. All rights reserved.
//
// Jason Chen CJ <jason.cj.chen@intel.com>
// Zhao Yakui <yakui.zhao@intel.com>
//

#[no_mangle]
unsafe extern "C" fn acrn_detect() -> u32 __init {
    static u32 __init acrn_detect(void)
    {
    return acrn_cpuid_base();
    }
#[no_mangle]
unsafe extern "C" fn acrn_init_platform() -> void __init {
    static void __init acrn_init_platform(void)
    {
// Install system interrupt handler for ACRN hypervisor callback
    sysvec_install(HYPERVISOR_CALLBACK_VECTOR, sysvec_acrn_hv_callback);
    x86_platform.calibrate_tsc = acrn_get_tsc_khz;
    x86_platform.calibrate_cpu = acrn_get_tsc_khz;
    }
#[no_mangle]
unsafe extern "C" fn acrn_x2apic_available() -> bool {
    static bool acrn_x2apic_available(void)
    {
    return boot_cpu_has(X86_FEATURE_X2APIC);
    }
    static void (*acrn_intr_handler)(void);
    DEFINE_IDTENTRY_SYSVEC(sysvec_acrn_hv_callback)
    {
    struct pt_regs *old_regs = set_irq_regs(regs);
//
// The hypervisor requires that the APIC EOI should be acked.
// If the APIC EOI is not acked, the APIC ISR bit for the
// HYPERVISOR_CALLBACK_VECTOR will not be cleared and then it
// will block the interrupt whose vector is lower than
// HYPERVISOR_CALLBACK_VECTOR.
//
    apic_eoi();
    inc_irq_stat(HYPERVISOR_CALLBACK);
    if (acrn_intr_handler)
    acrn_intr_handler();
    set_irq_regs(old_regs);
    }
#[no_mangle]
pub unsafe extern "C" fn acrn_setup_intr_handler((*handler)(void): *mut c_void) {
    void acrn_setup_intr_handler(void (*handler)(void))
    {
    acrn_intr_handler = handler;
    }
    EXPORT_SYMBOL_GPL(acrn_setup_intr_handler);
#[no_mangle]
pub unsafe extern "C" fn acrn_remove_intr_handler() {
    void acrn_remove_intr_handler(void)
    {
    acrn_intr_handler = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(acrn_remove_intr_handler);
    const __initconst struct hypervisor_x86 x86_hyper_acrn = {
    .name                   = "ACRN",
    .detect                 = acrn_detect,
    .type			= X86_HYPER_ACRN,
    .init.init_platform     = acrn_init_platform,
    .init.x2apic_available  = acrn_x2apic_available,
    };
