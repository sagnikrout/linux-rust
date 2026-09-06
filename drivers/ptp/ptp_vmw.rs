//! Automatically rewritten from C to Rust
//! Source: drivers/ptp/ptp_vmw.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// Copyright (C) 2020 VMware, Inc., Palo Alto, CA., USA
//
// PTP clock driver for VMware precision clock virtual device.
//

    static struct ptp_clock *ptp_vmw_clock;
#[no_mangle]
unsafe extern "C" fn ptp_vmw_pclk_read(ns: *mut u64) -> c_int {
    static int ptp_vmw_pclk_read(u64 *ns)
    {
    u32 ret, nsec_hi, nsec_lo;
    ret = vmware_hypercall3(VMWARE_CMD_PCLK_GETTIME, 0,
    &nsec_hi, &nsec_lo);
    if (ret == 0)
// ns = ((u64)nsec_hi << 32) | nsec_lo;
    return ret;
    }
//
// PTP clock ops.
//
#[no_mangle]
unsafe extern "C" fn ptp_vmw_adjtime(info: *mut ptp_clock_info, delta: i64) -> c_int {
    static int ptp_vmw_adjtime(struct ptp_clock_info *info, s64 delta)
    {
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn ptp_vmw_adjfine(info: *mut ptp_clock_info, delta: c_long) -> c_int {
    static int ptp_vmw_adjfine(struct ptp_clock_info *info, long delta)
    {
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn ptp_vmw_gettime(info: *mut ptp_clock_info, ts: *mut timespec64) -> c_int {
    static int ptp_vmw_gettime(struct ptp_clock_info *info, struct timespec64 *ts)
    {
    u64 ns;
    if (ptp_vmw_pclk_read(&ns) != 0)
    return -EIO;
// ts = ns_to_timespec64(ns);
    return 0;
    }
    static int ptp_vmw_settime(struct ptp_clock_info *info,
    const struct timespec64 *ts)
    {
    return -EOPNOTSUPP;
    }
    static int ptp_vmw_enable(struct ptp_clock_info *info,
    struct ptp_clock_request *request, int on)
    {
    return -EOPNOTSUPP;
    }
    static struct ptp_clock_info ptp_vmw_clock_info = {
    .owner		= THIS_MODULE,
    .name		= "ptp_vmw",
    .max_adj	= 0,
    .adjtime	= ptp_vmw_adjtime,
    .adjfine	= ptp_vmw_adjfine,
    .gettime64	= ptp_vmw_gettime,
    .settime64	= ptp_vmw_settime,
    .enable		= ptp_vmw_enable,
    };
//
// ACPI driver ops for VMware "precision clock" virtual device.
//
#[no_mangle]
unsafe extern "C" fn ptp_vmw_acpi_probe(pdev: *mut platform_device) -> c_int {
    static int ptp_vmw_acpi_probe(struct platform_device *pdev)
    {
    ptp_vmw_clock = ptp_clock_register(&ptp_vmw_clock_info, core::ptr::null_mut());
    if (IS_ERR(ptp_vmw_clock)) {
    pr_err("failed to register ptp clock\n");
    return PTR_ERR(ptp_vmw_clock);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ptp_vmw_acpi_remove(pdev: *mut platform_device) {
    static void ptp_vmw_acpi_remove(struct platform_device *pdev)
    {
    ptp_clock_unregister(ptp_vmw_clock);
    }
    static const struct acpi_device_id ptp_vmw_acpi_device_ids[] = {
    { "VMW0005", 0 },
    { "", 0 },
    };
    MODULE_DEVICE_TABLE(acpi, ptp_vmw_acpi_device_ids);
    static struct platform_driver ptp_vmw_acpi_driver = {
    .probe = ptp_vmw_acpi_probe,
    .remove = ptp_vmw_acpi_remove,
    .driver = {
    .name = "ptp_vmw_acpi",
    .acpi_match_table = ptp_vmw_acpi_device_ids,
    },
    };
#[no_mangle]
unsafe extern "C" fn ptp_vmw_init() -> int __init {
    static int __init ptp_vmw_init(void)
    {
    if (x86_hyper_type != X86_HYPER_VMWARE)
    return -1;
    return platform_driver_register(&ptp_vmw_acpi_driver);
    }
#[no_mangle]
unsafe extern "C" fn ptp_vmw_exit() -> void __exit {
    static void __exit ptp_vmw_exit(void)
    {
    platform_driver_unregister(&ptp_vmw_acpi_driver);
    }
    module_init(ptp_vmw_init);
    module_exit(ptp_vmw_exit);
    MODULE_DESCRIPTION("VMware virtual PTP clock driver");
    MODULE_AUTHOR("VMware, Inc.");
    MODULE_LICENSE("Dual BSD/GPL");
