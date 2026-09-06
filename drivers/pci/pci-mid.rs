//! Automatically rewritten from C to Rust
//! Source: drivers/pci/pci-mid.c
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
// Intel MID platform PM support
//
// Copyright (C) 2016, Intel Corporation
//
// Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
//

    static bool pci_mid_pm_enabled __read_mostly;
#[no_mangle]
pub unsafe extern "C" fn pci_use_mid_pm() -> bool {
    bool pci_use_mid_pm(void)
    {
    return pci_mid_pm_enabled;
    }
#[no_mangle]
pub unsafe extern "C" fn mid_pci_set_power_state(pdev: *mut pci_dev, state: pci_power_t) -> c_int {
    int mid_pci_set_power_state(struct pci_dev *pdev, pci_power_t state)
    {
    return intel_mid_pci_set_power_state(pdev, state);
    }
#[no_mangle]
pub unsafe extern "C" fn mid_pci_get_power_state(pdev: *mut pci_dev) -> pci_power_t {
    pci_power_t mid_pci_get_power_state(struct pci_dev *pdev)
    {
    return intel_mid_pci_get_power_state(pdev);
    }
//
// This table should be in sync with the one in
// arch/x86/platform/intel-mid/pwr.c.
//
    static const struct x86_cpu_id lpss_cpu_ids[] = {
    X86_MATCH_VFM(INTEL_ATOM_SALTWELL_MID, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ATOM_SILVERMONT_MID, core::ptr::null_mut()),
    {}
    };
#[no_mangle]
unsafe extern "C" fn mid_pci_init() -> int __init {
    static int __init mid_pci_init(void)
    {
    const struct x86_cpu_id *id;
    id = x86_match_cpu(lpss_cpu_ids);
    if (id)
    pci_mid_pm_enabled = true;
    return 0;
    }
    arch_initcall(mid_pci_init);
