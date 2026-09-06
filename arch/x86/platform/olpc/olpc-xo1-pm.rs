//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/olpc/olpc-xo1-pm.c
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
// Support for power management features of the OLPC XO-1 laptop
//
// Copyright (C) 2010 Andres Salomon <dilinger@queued.net>
// Copyright (C) 2010 One Laptop per Child
// Copyright (C) 2006 Red Hat, Inc.
// Copyright (C) 2006 Advanced Micro Devices, Inc.
//

    static unsigned long acpi_base;
    static unsigned long pms_base;
    let mut wakeup_mask: static u16 = CS5536_PM_PWRBTN;
    static struct {
    unsigned long address;
    unsigned short segment;
    } ofw_bios_entry = { 0xF0000 + PAGE_OFFSET, __KERNEL_CS };
// Set bits in the wakeup mask
#[no_mangle]
pub unsafe extern "C" fn olpc_xo1_pm_wakeup_set(value: u16) {
    void olpc_xo1_pm_wakeup_set(u16 value)
    {
    wakeup_mask |= value;
    }
    EXPORT_SYMBOL_GPL(olpc_xo1_pm_wakeup_set);
// Clear bits in the wakeup mask
#[no_mangle]
pub unsafe extern "C" fn olpc_xo1_pm_wakeup_clear(value: u16) {
    void olpc_xo1_pm_wakeup_clear(u16 value)
    {
    wakeup_mask &= ~value;
    }
    EXPORT_SYMBOL_GPL(olpc_xo1_pm_wakeup_clear);
#[no_mangle]
unsafe extern "C" fn xo1_power_state_enter(pm_state: suspend_state_t) -> c_int {
    static int xo1_power_state_enter(suspend_state_t pm_state)
    {
    unsigned long saved_sci_mask;
// Only STR is supported
    if (pm_state != PM_SUSPEND_MEM)
    return -EINVAL;
//
// Save SCI mask (this gets lost since PM1_EN is used as a mask for
// wakeup events, which is not necessarily the same event set)
//
    saved_sci_mask = inl(acpi_base + CS5536_PM1_STS);
    saved_sci_mask &= 0xffff0000;
// Save CPU state
    do_olpc_suspend_lowlevel();
// Resume path starts here
// Restore SCI mask (using dword access to CS5536_PM1_EN)
    outl(saved_sci_mask, acpi_base + CS5536_PM1_STS);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn xo1_do_sleep(sleep_state: u8) -> asmlinkage __visible int {
    asmlinkage __visible int xo1_do_sleep(u8 sleep_state)
    {
    void *pgd_addr = __va(read_cr3_pa());
// Program wakeup mask (using dword access to CS5536_PM1_EN)
    outl(wakeup_mask << 16, acpi_base + CS5536_PM1_STS);
    __asm__("movl %0,%%eax" : : "r" (pgd_addr));
    __asm__("call *(%%edi); cld"
    : : "D" (&ofw_bios_entry));
    __asm__("movb $0x34, %al\n\t"
    "outb %al, $0x70\n\t"
    "movb $0x30, %al\n\t"
    "outb %al, $0x71\n\t");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xo1_power_off() {
    static void xo1_power_off(void)
    {
    printk(KERN_INFO "OLPC XO-1 power off sequence...\n");
// Enable all of these controls with 0 delay
    outl(0x40000000, pms_base + CS5536_PM_SCLK);
    outl(0x40000000, pms_base + CS5536_PM_IN_SLPCTL);
    outl(0x40000000, pms_base + CS5536_PM_WKXD);
    outl(0x40000000, pms_base + CS5536_PM_WKD);
// Clear status bits (possibly unnecessary)
    outl(0x0002ffff, pms_base  + CS5536_PM_SSC);
    outl(0xffffffff, acpi_base + CS5536_PM_GPE0_STS);
// Write SLP_EN bit to start the machinery
    outl(0x00002000, acpi_base + CS5536_PM1_CNT);
    }
#[no_mangle]
unsafe extern "C" fn xo1_power_state_valid(pm_state: suspend_state_t) -> c_int {
    static int xo1_power_state_valid(suspend_state_t pm_state)
    {
// suspend-to-RAM only
    let mut pm_state: return = = PM_SUSPEND_MEM;
    }
    static const struct platform_suspend_ops xo1_suspend_ops = {
    .valid = xo1_power_state_valid,
    .enter = xo1_power_state_enter,
    };
#[no_mangle]
unsafe extern "C" fn xo1_pm_probe(pdev: *mut platform_device) -> c_int {
    static int xo1_pm_probe(struct platform_device *pdev)
    {
    struct resource *res;
// don't run on non-XOs
    if (!machine_is_olpc())
    return -ENODEV;
    res = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if (!res) {
    dev_err(&pdev.dev, "can't fetch device resource info\n");
    return -EIO;
    }
    if (strcmp(pdev.name, "cs5535-pms") == 0)
    pms_base = res.start;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(pdev->name, 0: "olpc-xo1-pm-acpi") ==) -> else {
    else if (strcmp(pdev.name, "olpc-xo1-pm-acpi") == 0)
    acpi_base = res.start;
// If we have both addresses, we can override the poweroff hook
    if (pms_base && acpi_base) {
    suspend_set_ops(&xo1_suspend_ops);
    pm_power_off = xo1_power_off;
    printk(KERN_INFO "OLPC XO-1 support registered\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xo1_pm_remove(pdev: *mut platform_device) {
    static void xo1_pm_remove(struct platform_device *pdev)
    {
    if (strcmp(pdev.name, "cs5535-pms") == 0)
    pms_base = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(pdev->name, 0: "olpc-xo1-pm-acpi") ==) -> else {
    else if (strcmp(pdev.name, "olpc-xo1-pm-acpi") == 0)
    acpi_base = 0;
    pm_power_off = core::ptr::null_mut();
    }
    static struct platform_driver cs5535_pms_driver = {
    .driver = {
    .name = "cs5535-pms",
    },
    .probe = xo1_pm_probe,
    .remove = xo1_pm_remove,
    };
    static struct platform_driver cs5535_acpi_driver = {
    .driver = {
    .name = "olpc-xo1-pm-acpi",
    },
    .probe = xo1_pm_probe,
    .remove = xo1_pm_remove,
    };
#[no_mangle]
unsafe extern "C" fn xo1_pm_init() -> int __init {
    static int __init xo1_pm_init(void)
    {
    int r;
    r = platform_driver_register(&cs5535_pms_driver);
    if (r)
    return r;
    r = platform_driver_register(&cs5535_acpi_driver);
    if (r)
    platform_driver_unregister(&cs5535_pms_driver);
    return r;
    }
    arch_initcall(xo1_pm_init);
