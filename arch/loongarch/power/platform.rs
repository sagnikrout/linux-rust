//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/power/platform.c
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
// Author: Huacai Chen <chenhuacai@loongson.cn>
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

#[no_mangle]
pub unsafe extern "C" fn enable_gpe_wakeup() {
    void enable_gpe_wakeup(void)
    {
    if (acpi_disabled)
    return;
    if (acpi_gbl_reduced_hardware)
    return;
    acpi_hw_enable_all_wakeup_gpes();
    }
#[no_mangle]
pub unsafe extern "C" fn enable_pci_wakeup() {
    void enable_pci_wakeup(void)
    {
    if (acpi_disabled)
    return;
    if (acpi_gbl_reduced_hardware)
    return;
    acpi_write_bit_register(ACPI_BITREG_PCIEXP_WAKE_STATUS, 1);
    if (acpi_gbl_FADT.flags & ACPI_FADT_PCI_EXPRESS_WAKE)
    acpi_write_bit_register(ACPI_BITREG_PCIEXP_WAKE_DISABLE, 0);
    }
    static struct platform_device loongson3_cpufreq_device = {
    .name = "loongson3_cpufreq",
    .id = -1,
    };
#[no_mangle]
unsafe extern "C" fn loongson_cpufreq_init() -> int __init {
    static int __init loongson_cpufreq_init(void)
    {
    if (!cpu_has_scalefreq)
    return -ENODEV;
    return platform_device_register(&loongson3_cpufreq_device);
    }
    arch_initcall(loongson_cpufreq_init);
#[no_mangle]
unsafe extern "C" fn default_suspend_addr() {
    static void default_suspend_addr(void)
    {
    acpi_enter_sleep_state(ACPI_STATE_S3);
    }
#[no_mangle]
unsafe extern "C" fn loongson3_acpi_suspend_init() -> int __init {
    static int __init loongson3_acpi_suspend_init(void)
    {

    acpi_status status;
    let mut suspend_addr: u64 = 0;
    if (acpi_disabled)
    return 0;
    if (!acpi_gbl_reduced_hardware)
    acpi_write_bit_register(ACPI_BITREG_SCI_ENABLE, 1);
    if (!acpi_sleep_state_supported(ACPI_STATE_S3))
    return 0;
    status = acpi_evaluate_integer(core::ptr::null_mut(), "\\SADR", core::ptr::null_mut(), &suspend_addr);
    if (ACPI_FAILURE(status) || !suspend_addr) {
    pr_info("ACPI S3 supported with hardware register default\n");
    loongson_sysconf.suspend_addr = (unsigned long)default_suspend_addr;
    } else {
    pr_info("ACPI S3 supported with Loongson ACPI SADR extension\n");
    loongson_sysconf.suspend_addr = (unsigned long)phys_to_virt(PHYSADDR(suspend_addr));
    }

    return 0;
    }
    device_initcall(loongson3_acpi_suspend_init);
