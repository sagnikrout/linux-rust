//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpi_lpit.c
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
// acpi_lpit.c - LPIT table processing functions
//
// Copyright (C) 2017 Intel Corporation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpit_residency_info {
    pub gaddr: acpi_generic_address,
    pub frequency: u64,
    pub iomem_addr: *mut void __iomem,
}

// Storage for an memory mapped and FFH based entries
    static struct lpit_residency_info residency_info_mem;
    static struct lpit_residency_info residency_info_ffh;
#[no_mangle]
unsafe extern "C" fn lpit_read_residency_counter_us(counter: *mut u64, io_mem: bool) -> c_int {
    static int lpit_read_residency_counter_us(u64 *counter, bool io_mem)
    {
    int err;
    if (io_mem) {
    let mut count: u64 = 0;
    int error;
    error = acpi_os_read_iomem(residency_info_mem.iomem_addr, &count,
    residency_info_mem.gaddr.bit_width);
    if (error)
    return error;
// counter = div64_u64(count * 1000000ULL, residency_info_mem.frequency);
    return 0;
    }
    err = rdmsrq_safe(residency_info_ffh.gaddr.address, counter);
    if (!err) {
    u64 mask = GENMASK_ULL(residency_info_ffh.gaddr.bit_offset +
    residency_info_ffh.gaddr. bit_width - 1,
    residency_info_ffh.gaddr.bit_offset);
// counter &= mask;
// counter >>= residency_info_ffh.gaddr.bit_offset;
// counter = div64_u64(*counter * 1000000ULL, residency_info_ffh.frequency);
    return 0;
    }
    return -ENODATA;
    }
    static ssize_t low_power_idle_system_residency_us_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    u64 counter;
    int ret;
    ret = lpit_read_residency_counter_us(&counter, true);
    if (ret)
    return ret;
    return sprintf(buf, "%llu\n", counter);
    }
    static DEVICE_ATTR_RO(low_power_idle_system_residency_us);
    static ssize_t low_power_idle_cpu_residency_us_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    u64 counter;
    int ret;
    ret = lpit_read_residency_counter_us(&counter, false);
    if (ret)
    return ret;
    return sprintf(buf, "%llu\n", counter);
    }
    static DEVICE_ATTR_RO(low_power_idle_cpu_residency_us);
#[no_mangle]
pub unsafe extern "C" fn lpit_read_residency_count_address(address: *mut u64) -> c_int {
    int lpit_read_residency_count_address(u64 *address)
    {
    if (!residency_info_mem.gaddr.address)
    return -EINVAL;
// address = residency_info_mem.gaddr.address;
    return 0;
    }
    EXPORT_SYMBOL_GPL(lpit_read_residency_count_address);
    static void lpit_update_residency(struct lpit_residency_info *info,
    struct acpi_lpit_native *lpit_native)
    {
    struct device *dev_root = bus_get_dev_root(&cpu_subsys);
// Silently fail, if cpuidle attribute group is not present
    if (!dev_root)
    return;
    info.frequency = lpit_native.counter_frequency ?
    lpit_native.counter_frequency : mul_u32_u32(tsc_khz, 1000U);
    if (!info.frequency)
    info.frequency = 1;
    info.gaddr = lpit_native.residency_counter;
    if (info.gaddr.space_id == ACPI_ADR_SPACE_SYSTEM_MEMORY) {
    info.iomem_addr = ioremap(info.gaddr.address,
    info.gaddr.bit_width / 8);
    if (!info.iomem_addr)
    goto exit;
    sysfs_add_file_to_group(&dev_root.kobj,
    &dev_attr_low_power_idle_system_residency_us.attr,
    "cpuidle");
    } else if (info.gaddr.space_id == ACPI_ADR_SPACE_FIXED_HARDWARE) {
    sysfs_add_file_to_group(&dev_root.kobj,
    &dev_attr_low_power_idle_cpu_residency_us.attr,
    "cpuidle");
    }
    exit:
    put_device(dev_root);
    }
#[no_mangle]
unsafe extern "C" fn lpit_process(begin: u64, end: u64) {
    static void lpit_process(u64 begin, u64 end)
    {
    while (begin + sizeof(struct acpi_lpit_native) <= end) {
    struct acpi_lpit_native *lpit_native = (struct acpi_lpit_native *)begin;
    if (!lpit_native.header.type && !lpit_native.header.flags) {
    if (lpit_native.residency_counter.space_id == ACPI_ADR_SPACE_SYSTEM_MEMORY &&
    !residency_info_mem.gaddr.address) {
    lpit_update_residency(&residency_info_mem, lpit_native);
    } else if (lpit_native.residency_counter.space_id == ACPI_ADR_SPACE_FIXED_HARDWARE &&
    !residency_info_ffh.gaddr.address) {
    lpit_update_residency(&residency_info_ffh, lpit_native);
    }
    }
    begin += lpit_native.header.length;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_init_lpit() {
    void acpi_init_lpit(void)
    {
    acpi_status status;
    struct acpi_table_lpit *lpit;
    status = acpi_get_table(ACPI_SIG_LPIT, 0, (struct acpi_table_header **)&lpit);
    if (ACPI_FAILURE(status))
    return;
    lpit_process((u64)lpit + sizeof(*lpit),
    (u64)lpit + lpit.header.length);
    acpi_put_table((struct acpi_table_header *)lpit);
    }
