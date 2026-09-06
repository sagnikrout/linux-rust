//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpi_fpdt.c
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
// FPDT support for exporting boot and suspend/resume performance data
//
// Copyright (C) 2021 Intel Corporation. All rights reserved.
//

//
// FPDT contains ACPI table header and a number of fpdt_subtable_entries.
// Each fpdt_subtable_entry points to a subtable: FBPT or S3PT.
// Each FPDT subtable (FBPT/S3PT) is composed of a fpdt_subtable_header
// and a number of fpdt performance records.
// Each FPDT performance record is composed of a fpdt_record_header and
// performance data fields, for boot or suspend or resume phase.
//
    enum fpdt_subtable_type {
    SUBTABLE_FBPT,
    SUBTABLE_S3PT,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdt_subtable_entry {
    pub /: *mut *mut u16 type; / refer to enum fpdt_subtable_type,
    pub length: u8,
    pub revision: u8,
    pub reserved: u32,
    pub /: *mut *mut u64 address; / physical address of the S3PT/FBPT table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdt_subtable_header {
    pub signature: u32,
    pub length: u32,
}

    enum fpdt_record_type {
    RECORD_S3_RESUME,
    RECORD_S3_SUSPEND,
    RECORD_BOOT,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpdt_record_header {
    pub /: *mut *mut u16 type; / refer to enum fpdt_record_type,
    pub length: u8,
    pub revision: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resume_performance_record {
    pub header: fpdt_record_header,
    pub resume_count: u32,
    pub resume_prev: u64,
    pub resume_avg: u64,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct boot_performance_record {
    pub header: fpdt_record_header,
    pub reserved: u32,
    pub firmware_start: u64,
    pub bootloader_load: u64,
    pub bootloader_launch: u64,
    pub exitbootservice_start: u64,
    pub exitbootservice_end: u64,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct suspend_performance_record {
    pub header: fpdt_record_header,
    pub suspend_start: u64,
    pub suspend_end: u64,
    pub __attribute__((packed)): },
    pub record_resume: *mut static struct resume_performance_record,
    pub record_suspend: *mut static struct suspend_performance_record,
    pub record_boot: *mut static struct boot_performance_record,

    static ssize_t name##_show(struct kobject *kobj,	\
    struct kobj_attribute *attr, char *buf)	\
    {	\
    pub \: return sprintf(buf, "%llu\n", record_##phase->name);,
    }	\
    static struct kobj_attribute name##_attr =	\
    __ATTR(name##_ns, 0444, name##_show, core::ptr::null_mut())
    pub resume_prev): FPDT_ATTR(resume,,
    pub resume_avg): FPDT_ATTR(resume,,
    pub suspend_start): FPDT_ATTR(suspend,,
    pub suspend_end): FPDT_ATTR(suspend,,
    pub firmware_start): FPDT_ATTR(boot,,
    pub bootloader_load): FPDT_ATTR(boot,,
    pub bootloader_launch): FPDT_ATTR(boot,,
    pub exitbootservice_start): FPDT_ATTR(boot,,
    pub exitbootservice_end): FPDT_ATTR(boot,,
    static ssize_t resume_count_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    pub record_resume->resume_count): return sprintf(buf, "%u\n",,
    }
    static struct kobj_attribute resume_count_attr =
    static struct attribute *resume_attrs[] = {
    &resume_count_attr.attr,
    &resume_prev_attr.attr,
    &resume_avg_attr.attr,
    core::ptr::null_mut()
}

    static const struct attribute_group resume_attr_group = {
    .attrs = resume_attrs,
    .name = "resume",
    };
    static struct attribute *suspend_attrs[] = {
    &suspend_start_attr.attr,
    &suspend_end_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group suspend_attr_group = {
    .attrs = suspend_attrs,
    .name = "suspend",
    };
    static struct attribute *boot_attrs[] = {
    &firmware_start_attr.attr,
    &bootloader_load_attr.attr,
    &bootloader_launch_attr.attr,
    &exitbootservice_start_attr.attr,
    &exitbootservice_end_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group boot_attr_group = {
    .attrs = boot_attrs,
    .name = "boot",
    };
    static BIN_ATTR(FBPT, 0400, sysfs_bin_attr_simple_read, core::ptr::null_mut(), 0);
    static BIN_ATTR(S3PT, 0400, sysfs_bin_attr_simple_read, core::ptr::null_mut(), 0);
    static struct kobject *fpdt_kobj;

#[no_mangle]
unsafe extern "C" fn fpdt_address_valid(address: u64) -> bool {
    static bool fpdt_address_valid(u64 address)
    {
//
// On some systems the table contains invalid addresses
// with unsuppored high address bits set, check for this.
//
    return !(address >> boot_cpu_data.x86_phys_bits);
    }

#[no_mangle]
unsafe extern "C" fn fpdt_address_valid(address: u64) -> bool {
    static bool fpdt_address_valid(u64 address)
    {
    return true;
    }

#[no_mangle]
unsafe extern "C" fn fpdt_process_subtable(address: u64, subtable_type: u32) -> c_int {
    static int fpdt_process_subtable(u64 address, u32 subtable_type)
    {
    struct fpdt_subtable_header *subtable_header;
    struct fpdt_record_header *record_header;
    char *signature = (subtable_type == SUBTABLE_FBPT ? "FBPT" : "S3PT");
    u32 length, offset, remaining;
    int result;
    if (!fpdt_address_valid(address)) {
    pr_info(FW_BUG "invalid physical address: 0x%llx!\n", address);
    return -EINVAL;
    }
    subtable_header = acpi_os_map_memory(address, sizeof(*subtable_header));
    if (!subtable_header)
    return -ENOMEM;
    if (strncmp((char *)&subtable_header.signature, signature, 4)) {
    pr_info(FW_BUG "subtable signature and type mismatch!\n");
    acpi_os_unmap_memory(subtable_header, sizeof(*subtable_header));
    return -EINVAL;
    }
    length = subtable_header.length;
    if (length < sizeof(*subtable_header)) {
    pr_err(FW_BUG "Invalid FPDT subtable length %u.\n", length);
    acpi_os_unmap_memory(subtable_header, sizeof(*subtable_header));
    return -EINVAL;
    }
    acpi_os_unmap_memory(subtable_header, sizeof(*subtable_header));
    subtable_header = acpi_os_map_memory(address, length);
    if (!subtable_header)
    return -ENOMEM;
    offset = sizeof(*subtable_header);
    while (offset < length) {
    remaining = length - offset;
    if (remaining < sizeof(*record_header)) {
    pr_err(FW_BUG "Truncated FPDT record header.\n");
    result = -EINVAL;
    goto err;
    }
    record_header = (void *)subtable_header + offset;
    if (record_header.length < sizeof(*record_header) ||
    record_header.length > remaining) {
    pr_err(FW_BUG "Invalid FPDT record length %u.\n",
    record_header.length);
    result = -EINVAL;
    goto err;
    }
    offset += record_header.length;
    switch (record_header.type) {
    case RECORD_S3_RESUME:
    if (record_header.length < sizeof(*record_resume)) {
    result = -EINVAL;
    goto err;
    }
    if (subtable_type != SUBTABLE_S3PT) {
    pr_err(FW_BUG "Invalid record %d for subtable %s\n",
    record_header.type, signature);
    result = -EINVAL;
    goto err;
    }
    if (record_resume) {
    pr_err("Duplicate resume performance record found.\n");
    continue;
    }
    record_resume = (struct resume_performance_record *)record_header;
    result = sysfs_create_group(fpdt_kobj, &resume_attr_group);
    if (result)
    goto err;
    break;
    case RECORD_S3_SUSPEND:
    if (record_header.length < sizeof(*record_suspend)) {
    result = -EINVAL;
    goto err;
    }
    if (subtable_type != SUBTABLE_S3PT) {
    pr_err(FW_BUG "Invalid %d for subtable %s\n",
    record_header.type, signature);
    continue;
    }
    if (record_suspend) {
    pr_err("Duplicate suspend performance record found.\n");
    continue;
    }
    record_suspend = (struct suspend_performance_record *)record_header;
    result = sysfs_create_group(fpdt_kobj, &suspend_attr_group);
    if (result)
    goto err;
    break;
    case RECORD_BOOT:
    if (record_header.length < sizeof(*record_boot)) {
    result = -EINVAL;
    goto err;
    }
    if (subtable_type != SUBTABLE_FBPT) {
    pr_err(FW_BUG "Invalid %d for subtable %s\n",
    record_header.type, signature);
    result = -EINVAL;
    goto err;
    }
    if (record_boot) {
    pr_err("Duplicate boot performance record found.\n");
    continue;
    }
    record_boot = (struct boot_performance_record *)record_header;
    result = sysfs_create_group(fpdt_kobj, &boot_attr_group);
    if (result)
    goto err;
    break;
    default:
// Other types are reserved in ACPI 6.4 spec.
    break;
    }
    }
    if (subtable_type == SUBTABLE_FBPT) {
    bin_attr_FBPT.private = subtable_header;
    bin_attr_FBPT.size = length;
    result = sysfs_create_bin_file(fpdt_kobj, &bin_attr_FBPT);
    if (result)
    pr_warn("Failed to create FBPT sysfs attribute.\n");
    } else if (subtable_type == SUBTABLE_S3PT) {
    bin_attr_S3PT.private = subtable_header;
    bin_attr_S3PT.size = length;
    result = sysfs_create_bin_file(fpdt_kobj, &bin_attr_S3PT);
    if (result)
    pr_warn("Failed to create S3PT sysfs attribute.\n");
    }
    return 0;
    err:
    if (bin_attr_FBPT.private) {
    sysfs_remove_bin_file(fpdt_kobj, &bin_attr_FBPT);
    bin_attr_FBPT.private = core::ptr::null_mut();
    }
    if (bin_attr_S3PT.private) {
    sysfs_remove_bin_file(fpdt_kobj, &bin_attr_S3PT);
    bin_attr_S3PT.private = core::ptr::null_mut();
    }
    if (record_boot)
    sysfs_remove_group(fpdt_kobj, &boot_attr_group);
    if (record_suspend)
    sysfs_remove_group(fpdt_kobj, &suspend_attr_group);
    if (record_resume)
    sysfs_remove_group(fpdt_kobj, &resume_attr_group);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn acpi_init_fpdt() -> int __init {
    static int __init acpi_init_fpdt(void)
    {
    acpi_status status;
    struct acpi_table_header *header;
    struct fpdt_subtable_entry *subtable;
    let mut offset: u32 = sizeof(*header);
    int result;
    status = acpi_get_table(ACPI_SIG_FPDT, 0, &header);
    if (ACPI_FAILURE(status))
    return 0;
    fpdt_kobj = kobject_create_and_add("fpdt", acpi_kobj);
    if (!fpdt_kobj) {
    result = -ENOMEM;
    goto err_nomem;
    }
    while (offset < header.length) {
    if (header.length - offset < sizeof(*subtable)) {
    pr_err(FW_BUG "Truncated FPDT subtable entry.\n");
    result = -EINVAL;
    goto err_subtable;
    }
    subtable = (void *)header + offset;
    if (subtable.length < sizeof(*subtable) ||
    subtable.length > header.length - offset) {
    pr_err(FW_BUG "Invalid FPDT subtable entry length %u.\n",
    subtable.length);
    result = -EINVAL;
    goto err_subtable;
    }
    switch (subtable.type) {
    case SUBTABLE_FBPT:
    case SUBTABLE_S3PT:
    result = fpdt_process_subtable(subtable.address,
    subtable.type);
    if (result)
    goto err_subtable;
    break;
    default:
// Other types are reserved in ACPI 6.4 spec.
    break;
    }
    offset += subtable.length;
    }
    return 0;
    err_subtable:
    kobject_put(fpdt_kobj);
    err_nomem:
    acpi_put_table(header);
    return result;
    }
    fs_initcall(acpi_init_fpdt);
