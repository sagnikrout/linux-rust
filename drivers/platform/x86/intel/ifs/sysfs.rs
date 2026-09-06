//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/ifs/sysfs.c
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
// Copyright(c) 2022 Intel Corporation.

//
// Protects against simultaneous tests on multiple cores, or
// reloading can file while a test is in progress
//
    static DEFINE_SEMAPHORE(ifs_sem, 1);
//
// The sysfs interface to check additional details of last test
// cat /sys/devices/system/platform/ifs/details
//
    static ssize_t details_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct ifs_data *ifsd = ifs_get_data(dev);
    return sysfs_emit(buf, "%#llx\n", ifsd.scan_details);
    }
    static DEVICE_ATTR_RO(details);
    static const char * const status_msg[] = {
    [SCAN_NOT_TESTED] = "untested",
    [SCAN_TEST_PASS] = "pass",
    [SCAN_TEST_FAIL] = "fail"
    };
//
// The sysfs interface to check the test status:
// To check the status of last test
// cat /sys/devices/platform/ifs/status
//
    static ssize_t status_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct ifs_data *ifsd = ifs_get_data(dev);
    return sysfs_emit(buf, "%s\n", status_msg[ifsd.status]);
    }
    static DEVICE_ATTR_RO(status);
//
// The sysfs interface for single core testing
// To start test, for example, cpu5
// echo 5 > /sys/devices/platform/ifs/run_test
// To check the result:
// cat /sys/devices/platform/ifs/result
// The sibling core gets tested at the same time.
//
    static ssize_t run_test_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    unsigned int cpu;
    int rc;
    rc = kstrtouint(buf, 0, &cpu);
    if (rc < 0 || cpu >= nr_cpu_ids)
    return -EINVAL;
    if (down_interruptible(&ifs_sem))
    return -EINTR;
    rc = do_core_test(cpu, dev);
    up(&ifs_sem);
    return rc ? rc : count;
    }
    static DEVICE_ATTR_WO(run_test);
    static ssize_t current_batch_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct ifs_data *ifsd = ifs_get_data(dev);
    unsigned int cur_batch;
    int rc;
    rc = kstrtouint(buf, 0, &cur_batch);
    if (rc < 0 || cur_batch > 0xff)
    return -EINVAL;
    if (down_interruptible(&ifs_sem))
    return -EINTR;
    ifsd.cur_batch = cur_batch;
    rc = ifs_load_firmware(dev);
    up(&ifs_sem);
    return (rc == 0) ? count : rc;
    }
    static ssize_t current_batch_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ifs_data *ifsd = ifs_get_data(dev);
    if (!ifsd.loaded)
    return sysfs_emit(buf, "none\n");
    else
    return sysfs_emit(buf, "0x%02x\n", ifsd.cur_batch);
    }
    static DEVICE_ATTR_RW(current_batch);
//
// Display currently loaded IFS image version.
//
    static ssize_t image_version_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ifs_data *ifsd = ifs_get_data(dev);
    if (!ifsd.loaded)
    return sysfs_emit(buf, "%s\n", "none");
    else
    return sysfs_emit(buf, "%#x\n", ifsd.loaded_version);
    }
    static DEVICE_ATTR_RO(image_version);
// global scan sysfs attributes
    struct attribute *plat_ifs_attrs[] = {
    &dev_attr_details.attr,
    &dev_attr_status.attr,
    &dev_attr_run_test.attr,
    &dev_attr_current_batch.attr,
    &dev_attr_image_version.attr,
    core::ptr::null_mut()
    };
// global array sysfs attributes
    struct attribute *plat_ifs_array_attrs[] = {
    &dev_attr_details.attr,
    &dev_attr_status.attr,
    &dev_attr_run_test.attr,
    core::ptr::null_mut()
    };
