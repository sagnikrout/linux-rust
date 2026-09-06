//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel_scu_wdt.c
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
// Intel Merrifield watchdog platform device library file
//
// (C) Copyright 2014 Intel Corporation
// Author: David Cohen <david.a.cohen@linux.intel.com>
//

pub const TANGIER_EXT_TIMER0_MSI: c_int = 12;
    static struct platform_device wdt_dev = {
    .name = "intel_mid_wdt",
    .id = -1,
    };
#[no_mangle]
unsafe extern "C" fn tangier_probe(pdev: *mut platform_device) -> c_int {
    static int tangier_probe(struct platform_device *pdev)
    {
    struct irq_alloc_info info;
    struct intel_mid_wdt_pdata *pdata = pdev.dev.platform_data;
    let mut gsi: c_int = TANGIER_EXT_TIMER0_MSI;
    int irq;
    if (!pdata)
    return -EINVAL;
// IOAPIC builds identity mapping between GSI and IRQ on MID
    ioapic_set_alloc_attr(&info, cpu_to_node(0), 1, 0);
    irq = mp_map_gsi_to_irq(gsi, IOAPIC_MAP_ALLOC, &info);
    if (irq < 0) {
    dev_warn(&pdev.dev, "cannot find interrupt %d in ioapic\n", gsi);
    return irq;
    }
    pdata.irq = irq;
    return 0;
    }
    static struct intel_mid_wdt_pdata tangier_pdata = {
    .probe = tangier_probe,
    };
    static const struct x86_cpu_id intel_mid_cpu_ids[] = {
    X86_MATCH_VFM(INTEL_ATOM_SILVERMONT_MID, &tangier_pdata),
    {}
    };
#[no_mangle]
unsafe extern "C" fn register_mid_wdt() -> int __init {
    static int __init register_mid_wdt(void)
    {
    const struct x86_cpu_id *id;
    id = x86_match_cpu(intel_mid_cpu_ids);
    if (!id)
    return -ENODEV;
    wdt_dev.dev.platform_data = (struct intel_mid_wdt_pdata *)id.driver_data;
    return platform_device_register(&wdt_dev);
    }
    arch_initcall(register_mid_wdt);
#[no_mangle]
unsafe extern "C" fn unregister_mid_wdt() -> void __exit {
    static void __exit unregister_mid_wdt(void)
    {
    platform_device_unregister(&wdt_dev);
    }
    __exitcall(unregister_mid_wdt);
