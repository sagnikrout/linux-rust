//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/dptf/dptf_pch_fivr.c
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
// dptf_pch_fivr:  DPTF PCH FIVR Participant driver
// Copyright (c) 2020, Intel Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_fivr_resp {
    pub status: u64,
    pub result: u64,
}

#[no_mangle]
unsafe extern "C" fn pch_fivr_read(handle: acpi_handle, method: *mut c_char, fivr_resp: *mut pch_fivr_resp) -> c_int {
    static int pch_fivr_read(acpi_handle handle, char *method, struct pch_fivr_resp *fivr_resp)
    {
    let mut resp: acpi_buffer = { sizeof(struct pch_fivr_resp), fivr_resp};
    let mut buffer: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    let mut format: acpi_buffer = { sizeof("NN"), "NN" };
    union acpi_object *obj;
    acpi_status status;
    let mut ret: c_int = -EFAULT;
    status = acpi_evaluate_object(handle, method, core::ptr::null_mut(), &buffer);
    if (ACPI_FAILURE(status))
    return ret;
    obj = buffer.pointer;
    if (!obj || obj.type != ACPI_TYPE_PACKAGE)
    goto release_buffer;
    status = acpi_extract_package(obj, &format, &resp);
    if (ACPI_FAILURE(status))
    goto release_buffer;
    if (fivr_resp.status)
    goto release_buffer;
    ret = 0;
    release_buffer:
    ACPI_FREE(buffer.pointer);
    return ret;
    }
//
// Presentation of attributes which are defined for INTC10xx
// They are:
// freq_mhz_low_clock : Set PCH FIVR switching freq for
// FIVR clock 19.2MHz and 24MHz
// freq_mhz_high_clock : Set PCH FIVR switching freq for
// FIVR clock 38.4MHz
//

    static ssize_t name##_show(struct device *dev,\
    struct device_attribute *attr,\
    char *buf)\
    {\
    struct acpi_device *acpi_dev = dev_get_drvdata(dev);\
    struct pch_fivr_resp fivr_resp;\
    int status;\
    \
    status = pch_fivr_read(acpi_dev.handle, #method, &fivr_resp);\
    if (status)\
    return status;\
    \
    return sprintf(buf, "%llu\n", fivr_resp.result);\
    }

    static ssize_t name##_store(struct device *dev,\
    struct device_attribute *attr,\
    const char *buf, size_t count)\
    {\
    struct acpi_device *acpi_dev = dev_get_drvdata(dev);\
    acpi_status status;\
    u32 val;\
    \
    if (kstrtouint(buf, 0, &val) < 0)\
    return -EINVAL;\
    \
    status = acpi_execute_simple_method(acpi_dev.handle, #method, val);\
    if (ACPI_SUCCESS(status))\
    return count;\
    \
    return -EINVAL;\
    }
    PCH_FIVR_SHOW(freq_mhz_low_clock, GFC0)
    PCH_FIVR_SHOW(freq_mhz_high_clock, GFC1)
    PCH_FIVR_SHOW(ssc_clock_info, GEMI)
    PCH_FIVR_SHOW(fivr_switching_freq_mhz, GFCS)
    PCH_FIVR_SHOW(fivr_switching_fault_status, GFFS)
    PCH_FIVR_STORE(freq_mhz_low_clock, RFC0)
    PCH_FIVR_STORE(freq_mhz_high_clock, RFC1)
    static DEVICE_ATTR_RW(freq_mhz_low_clock);
    static DEVICE_ATTR_RW(freq_mhz_high_clock);
    static DEVICE_ATTR_RO(ssc_clock_info);
    static DEVICE_ATTR_RO(fivr_switching_freq_mhz);
    static DEVICE_ATTR_RO(fivr_switching_fault_status);
    static struct attribute *fivr_attrs[] = {
    &dev_attr_freq_mhz_low_clock.attr,
    &dev_attr_freq_mhz_high_clock.attr,
    &dev_attr_ssc_clock_info.attr,
    &dev_attr_fivr_switching_freq_mhz.attr,
    &dev_attr_fivr_switching_fault_status.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group pch_fivr_attribute_group = {
    .attrs = fivr_attrs,
    .name = "pch_fivr_switch_frequency"
    };
#[no_mangle]
unsafe extern "C" fn pch_fivr_add(pdev: *mut platform_device) -> c_int {
    static int pch_fivr_add(struct platform_device *pdev)
    {
    struct acpi_device *acpi_dev;
    unsigned long long ptype;
    acpi_status status;
    int result;
    acpi_dev = ACPI_COMPANION(&(pdev.dev));
    if (!acpi_dev)
    return -ENODEV;
    status = acpi_evaluate_integer(acpi_dev.handle, "PTYP", core::ptr::null_mut(), &ptype);
    if (ACPI_FAILURE(status) || ptype != 0x05)
    return -ENODEV;
    result = sysfs_create_group(&pdev.dev.kobj,
    &pch_fivr_attribute_group);
    if (result)
    return result;
    platform_set_drvdata(pdev, acpi_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pch_fivr_remove(pdev: *mut platform_device) {
    static void pch_fivr_remove(struct platform_device *pdev)
    {
    sysfs_remove_group(&pdev.dev.kobj, &pch_fivr_attribute_group);
    }
    static const struct acpi_device_id pch_fivr_device_ids[] = {
    {"INTC1045", 0},
    {"INTC1049", 0},
    {"INTC1064", 0},
    {"INTC106B", 0},
    {"INTC10A3", 0},
    {"INTC10D7", 0},
    {"", 0},
    };
    MODULE_DEVICE_TABLE(acpi, pch_fivr_device_ids);
    static struct platform_driver pch_fivr_driver = {
    .probe = pch_fivr_add,
    .remove = pch_fivr_remove,
    .driver = {
    .name = "dptf_pch_fivr",
    .acpi_match_table = pch_fivr_device_ids,
    },
    };
    module_platform_driver(pch_fivr_driver);
    MODULE_AUTHOR("Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("ACPI DPTF PCH FIVR driver");
