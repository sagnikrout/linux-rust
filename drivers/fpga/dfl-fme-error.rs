//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/dfl-fme-error.c
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
// Driver for FPGA Management Engine Error Management
//
// Copyright 2019 Intel Corporation, Inc.
//
// Authors:
// Kang Luwei <luwei.kang@intel.com>
// Xiao Guangrong <guangrong.xiao@linux.intel.com>
// Wu Hao <hao.wu@intel.com>
// Joseph Grecco <joe.grecco@intel.com>
// Enno Luebbers <enno.luebbers@intel.com>
// Tim Whisonant <tim.whisonant@intel.com>
// Ananda Ravuri <ananda.ravuri@intel.com>
// Mitchel, Henry <henry.mitchel@intel.com>
//

pub const FME_ERROR_MASK: c_uint = 0x8;
pub const FME_ERROR: c_uint = 0x10;

pub const PCIE0_ERROR_MASK: c_uint = 0x18;
pub const PCIE0_ERROR: c_uint = 0x20;
pub const PCIE1_ERROR_MASK: c_uint = 0x28;
pub const PCIE1_ERROR: c_uint = 0x30;
pub const FME_FIRST_ERROR: c_uint = 0x38;
pub const FME_NEXT_ERROR: c_uint = 0x40;
pub const RAS_NONFAT_ERROR_MASK: c_uint = 0x48;
pub const RAS_NONFAT_ERROR: c_uint = 0x50;
pub const RAS_CATFAT_ERROR_MASK: c_uint = 0x58;
pub const RAS_CATFAT_ERROR: c_uint = 0x60;
pub const RAS_ERROR_INJECT: c_uint = 0x68;

    static ssize_t pcie0_errors_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct dfl_feature_dev_data *fdata = to_dfl_feature_dev_data(dev);
    void __iomem *base;
    u64 value;
    base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR);
    mutex_lock(&fdata.lock);
    value = readq(base + PCIE0_ERROR);
    mutex_unlock(&fdata.lock);
    return sprintf(buf, "0x%llx\n", (unsigned long long)value);
    }
    static ssize_t pcie0_errors_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct dfl_feature_dev_data *fdata = to_dfl_feature_dev_data(dev);
    void __iomem *base;
    let mut ret: c_int = 0;
    u64 v, val;
    if (kstrtou64(buf, 0, &val))
    return -EINVAL;
    base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR);
    mutex_lock(&fdata.lock);
    writeq(GENMASK_ULL(63, 0), base + PCIE0_ERROR_MASK);
    v = readq(base + PCIE0_ERROR);
    if (val == v)
    writeq(v, base + PCIE0_ERROR);
    else
    ret = -EINVAL;
    writeq(0ULL, base + PCIE0_ERROR_MASK);
    mutex_unlock(&fdata.lock);
    return ret ? ret : count;
    }
    static DEVICE_ATTR_RW(pcie0_errors);
    static ssize_t pcie1_errors_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct dfl_feature_dev_data *fdata = to_dfl_feature_dev_data(dev);
    void __iomem *base;
    u64 value;
    base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR);
    mutex_lock(&fdata.lock);
    value = readq(base + PCIE1_ERROR);
    mutex_unlock(&fdata.lock);
    return sprintf(buf, "0x%llx\n", (unsigned long long)value);
    }
    static ssize_t pcie1_errors_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct dfl_feature_dev_data *fdata = to_dfl_feature_dev_data(dev);
    void __iomem *base;
    let mut ret: c_int = 0;
    u64 v, val;
    if (kstrtou64(buf, 0, &val))
    return -EINVAL;
    base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR);
    mutex_lock(&fdata.lock);
    writeq(GENMASK_ULL(63, 0), base + PCIE1_ERROR_MASK);
    v = readq(base + PCIE1_ERROR);
    if (val == v)
    writeq(v, base + PCIE1_ERROR);
    else
    ret = -EINVAL;
    writeq(0ULL, base + PCIE1_ERROR_MASK);
    mutex_unlock(&fdata.lock);
    return ret ? ret : count;
    }
    static DEVICE_ATTR_RW(pcie1_errors);
    static ssize_t nonfatal_errors_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct dfl_feature_dev_data *fdata = to_dfl_feature_dev_data(dev);
    void __iomem *base;
    base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR);
    return sprintf(buf, "0x%llx\n",
    (unsigned long long)readq(base + RAS_NONFAT_ERROR));
    }
    static DEVICE_ATTR_RO(nonfatal_errors);
    static ssize_t catfatal_errors_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct dfl_feature_dev_data *fdata = to_dfl_feature_dev_data(dev);
    void __iomem *base;
    base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR);
    return sprintf(buf, "0x%llx\n",
    (unsigned long long)readq(base + RAS_CATFAT_ERROR));
    }
    static DEVICE_ATTR_RO(catfatal_errors);
    static ssize_t inject_errors_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct dfl_feature_dev_data *fdata = to_dfl_feature_dev_data(dev);
    void __iomem *base;
    u64 v;
    base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR);
    mutex_lock(&fdata.lock);
    v = readq(base + RAS_ERROR_INJECT);
    mutex_unlock(&fdata.lock);
    return sprintf(buf, "0x%llx\n",
    (unsigned long long)FIELD_GET(INJECT_ERROR_MASK, v));
    }
    static ssize_t inject_errors_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct dfl_feature_dev_data *fdata = to_dfl_feature_dev_data(dev);
    void __iomem *base;
    u8 inject_error;
    u64 v;
    if (kstrtou8(buf, 0, &inject_error))
    return -EINVAL;
    if (inject_error & ~INJECT_ERROR_MASK)
    return -EINVAL;
    base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR);
    mutex_lock(&fdata.lock);
    v = readq(base + RAS_ERROR_INJECT);
    v &= ~INJECT_ERROR_MASK;
    v |= FIELD_PREP(INJECT_ERROR_MASK, inject_error);
    writeq(v, base + RAS_ERROR_INJECT);
    mutex_unlock(&fdata.lock);
    return count;
    }
    static DEVICE_ATTR_RW(inject_errors);
    static ssize_t fme_errors_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct dfl_feature_dev_data *fdata = to_dfl_feature_dev_data(dev);
    void __iomem *base;
    u64 value;
    base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR);
    mutex_lock(&fdata.lock);
    value = readq(base + FME_ERROR);
    mutex_unlock(&fdata.lock);
    return sprintf(buf, "0x%llx\n", (unsigned long long)value);
    }
    static ssize_t fme_errors_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct dfl_feature_dev_data *fdata = to_dfl_feature_dev_data(dev);
    void __iomem *base;
    u64 v, val;
    let mut ret: c_int = 0;
    if (kstrtou64(buf, 0, &val))
    return -EINVAL;
    base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR);
    mutex_lock(&fdata.lock);
    writeq(GENMASK_ULL(63, 0), base + FME_ERROR_MASK);
    v = readq(base + FME_ERROR);
    if (val == v)
    writeq(v, base + FME_ERROR);
    else
    ret = -EINVAL;
// Workaround: disable MBP_ERROR if feature revision is 0
    writeq(dfl_feature_revision(base) ? 0ULL : MBP_ERROR,
    base + FME_ERROR_MASK);
    mutex_unlock(&fdata.lock);
    return ret ? ret : count;
    }
    static DEVICE_ATTR_RW(fme_errors);
    static ssize_t first_error_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct dfl_feature_dev_data *fdata = to_dfl_feature_dev_data(dev);
    void __iomem *base;
    u64 value;
    base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR);
    mutex_lock(&fdata.lock);
    value = readq(base + FME_FIRST_ERROR);
    mutex_unlock(&fdata.lock);
    return sprintf(buf, "0x%llx\n", (unsigned long long)value);
    }
    static DEVICE_ATTR_RO(first_error);
    static ssize_t next_error_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct dfl_feature_dev_data *fdata = to_dfl_feature_dev_data(dev);
    void __iomem *base;
    u64 value;
    base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR);
    mutex_lock(&fdata.lock);
    value = readq(base + FME_NEXT_ERROR);
    mutex_unlock(&fdata.lock);
    return sprintf(buf, "0x%llx\n", (unsigned long long)value);
    }
    static DEVICE_ATTR_RO(next_error);
    static struct attribute *fme_global_err_attrs[] = {
    &dev_attr_pcie0_errors.attr,
    &dev_attr_pcie1_errors.attr,
    &dev_attr_nonfatal_errors.attr,
    &dev_attr_catfatal_errors.attr,
    &dev_attr_inject_errors.attr,
    &dev_attr_fme_errors.attr,
    &dev_attr_first_error.attr,
    &dev_attr_next_error.attr,
    core::ptr::null_mut(),
    };
    static umode_t fme_global_err_attrs_visible(struct kobject *kobj,
    struct attribute *attr, int n)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct dfl_feature_dev_data *fdata;
    fdata = to_dfl_feature_dev_data(dev);
//
// sysfs entries are visible only if related private feature is
// enumerated.
//
    if (!dfl_get_feature_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR))
    return 0;
    return attr.mode;
    }
    const struct attribute_group fme_global_err_group = {
    .name       = "errors",
    .attrs      = fme_global_err_attrs,
    .is_visible = fme_global_err_attrs_visible,
    };
#[no_mangle]
unsafe extern "C" fn fme_err_mask(dev: *mut device, mask: bool) {
    static void fme_err_mask(struct device *dev, bool mask)
    {
    struct dfl_feature_dev_data *fdata = to_dfl_feature_dev_data(dev);
    void __iomem *base;
    base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_GLOBAL_ERR);
    mutex_lock(&fdata.lock);
// Workaround: keep MBP_ERROR always masked if revision is 0
    if (dfl_feature_revision(base))
    writeq(mask ? ERROR_MASK : 0, base + FME_ERROR_MASK);
    else
    writeq(mask ? ERROR_MASK : MBP_ERROR, base + FME_ERROR_MASK);
    writeq(mask ? ERROR_MASK : 0, base + PCIE0_ERROR_MASK);
    writeq(mask ? ERROR_MASK : 0, base + PCIE1_ERROR_MASK);
    writeq(mask ? ERROR_MASK : 0, base + RAS_NONFAT_ERROR_MASK);
    writeq(mask ? ERROR_MASK : 0, base + RAS_CATFAT_ERROR_MASK);
    mutex_unlock(&fdata.lock);
    }
    static int fme_global_err_init(struct platform_device *pdev,
    struct dfl_feature *feature)
    {
    fme_err_mask(&pdev.dev, false);
    return 0;
    }
    static void fme_global_err_uinit(struct platform_device *pdev,
    struct dfl_feature *feature)
    {
    fme_err_mask(&pdev.dev, true);
    }
    static long
    fme_global_error_ioctl(struct platform_device *pdev,
    struct dfl_feature *feature,
    unsigned int cmd, unsigned long arg)
    {
    switch (cmd) {
    case DFL_FPGA_FME_ERR_GET_IRQ_NUM:
    return dfl_feature_ioctl_get_num_irqs(pdev, feature, arg);
    case DFL_FPGA_FME_ERR_SET_IRQ:
    return dfl_feature_ioctl_set_irq(pdev, feature, arg);
    default:
    dev_dbg(&pdev.dev, "%x cmd not handled", cmd);
    return -ENODEV;
    }
    }
    const struct dfl_feature_id fme_global_err_id_table[] = {
    {.id = FME_FEATURE_ID_GLOBAL_ERR,},
    {0,}
    };
    const struct dfl_feature_ops fme_global_err_ops = {
    .init = fme_global_err_init,
    .uinit = fme_global_err_uinit,
    .ioctl = fme_global_error_ioctl,
    };
