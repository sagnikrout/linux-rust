//! Automatically rewritten from C to Rust
//! Source: drivers/memory/dfl-emif.c
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
// DFL device driver for EMIF private feature
//
// Copyright (C) 2020 Intel Corporation, Inc.
//

pub const FME_FEATURE_ID_EMIF: c_uint = 0x9;
pub const EMIF_STAT: c_uint = 0x8;
pub const EMIF_STAT_INIT_DONE_SFT: c_int = 0;
pub const EMIF_STAT_CALC_FAIL_SFT: c_int = 8;
pub const EMIF_STAT_CLEAR_BUSY_SFT: c_int = 16;
pub const EMIF_CTRL: c_uint = 0x10;
pub const EMIF_CTRL_CLEAR_EN_SFT: c_int = 0;

//
// The Capability Register replaces the Control Register (at the same
// offset) for EMIF feature revisions > 0. The bitmask that indicates
// the presence of memory channels exists in both the Capability Register
// and Control Register definitions. These can be thought of as a C union.
// The Capability Register definitions are used to check for the existence
// of a memory channel, and the Control Register definitions are used for
// managing the memory-clear functionality in revision 0.
//
pub const EMIF_CAPABILITY_BASE: c_uint = 0x10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_emif {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub /: *mut *mut spinlock_t lock; / Serialises access to EMIF_CTRL reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct emif_attr {
    pub attr: device_attribute,
    pub shift: u32,
    pub index: u32,
}

    container_of(dev_attr, struct emif_attr, attr)
    static ssize_t emif_state_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct emif_attr *eattr = to_emif_attr(attr);
    struct dfl_emif *de = dev_get_drvdata(dev);
    u64 val;
    val = readq(de.base + EMIF_STAT);
    return sysfs_emit(buf, "%u\n",
    !!(val & BIT_ULL(eattr.shift + eattr.index)));
    }
    static ssize_t emif_clear_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct emif_attr *eattr = to_emif_attr(attr);
    struct dfl_emif *de = dev_get_drvdata(dev);
    u64 clear_busy_msk, clear_en_msk, val;
    void __iomem *base = de.base;
    if (!sysfs_streq(buf, "1"))
    return -EINVAL;
    clear_busy_msk = BIT_ULL(EMIF_STAT_CLEAR_BUSY_SFT + eattr.index);
    clear_en_msk = BIT_ULL(EMIF_CTRL_CLEAR_EN_SFT + eattr.index);
    spin_lock(&de.lock);
// The CLEAR_EN field is WO, but other fields are RW
    val = readq(base + EMIF_CTRL);
    val &= ~EMIF_CTRL_CLEAR_EN_MSK;
    val |= clear_en_msk;
    writeq(val, base + EMIF_CTRL);
    spin_unlock(&de.lock);
    if (readq_poll_timeout(base + EMIF_STAT, val,
    !(val & clear_busy_msk),
    EMIF_POLL_INVL, EMIF_POLL_TIMEOUT)) {
    dev_err(de.dev, "timeout, fail to clear\n");
    return -ETIMEDOUT;
    }
    return count;
    }

    static struct emif_attr emif_attr_##inf##_index##_##_name =	\
    { .attr = __ATTR(inf##_index##_##_name, 0444,		\
    emif_state_show, core::ptr::null_mut()),		\
    .shift = (_shift), .index = (_index) }

    static struct emif_attr emif_attr_##inf##_index##_clear =	\
    { .attr = __ATTR(inf##_index##_clear, 0200,		\
    core::ptr::null_mut(), emif_clear_store),		\
    .index = (_index) }
    emif_state_attr(init_done, EMIF_STAT_INIT_DONE_SFT, 0);
    emif_state_attr(init_done, EMIF_STAT_INIT_DONE_SFT, 1);
    emif_state_attr(init_done, EMIF_STAT_INIT_DONE_SFT, 2);
    emif_state_attr(init_done, EMIF_STAT_INIT_DONE_SFT, 3);
    emif_state_attr(init_done, EMIF_STAT_INIT_DONE_SFT, 4);
    emif_state_attr(init_done, EMIF_STAT_INIT_DONE_SFT, 5);
    emif_state_attr(init_done, EMIF_STAT_INIT_DONE_SFT, 6);
    emif_state_attr(init_done, EMIF_STAT_INIT_DONE_SFT, 7);
    emif_state_attr(cal_fail, EMIF_STAT_CALC_FAIL_SFT, 0);
    emif_state_attr(cal_fail, EMIF_STAT_CALC_FAIL_SFT, 1);
    emif_state_attr(cal_fail, EMIF_STAT_CALC_FAIL_SFT, 2);
    emif_state_attr(cal_fail, EMIF_STAT_CALC_FAIL_SFT, 3);
    emif_state_attr(cal_fail, EMIF_STAT_CALC_FAIL_SFT, 4);
    emif_state_attr(cal_fail, EMIF_STAT_CALC_FAIL_SFT, 5);
    emif_state_attr(cal_fail, EMIF_STAT_CALC_FAIL_SFT, 6);
    emif_state_attr(cal_fail, EMIF_STAT_CALC_FAIL_SFT, 7);
    emif_clear_attr(0);
    emif_clear_attr(1);
    emif_clear_attr(2);
    emif_clear_attr(3);
    emif_clear_attr(4);
    emif_clear_attr(5);
    emif_clear_attr(6);
    emif_clear_attr(7);
    static struct attribute *dfl_emif_attrs[] = {
    &emif_attr_inf0_init_done.attr.attr,
    &emif_attr_inf0_cal_fail.attr.attr,
    &emif_attr_inf0_clear.attr.attr,
    &emif_attr_inf1_init_done.attr.attr,
    &emif_attr_inf1_cal_fail.attr.attr,
    &emif_attr_inf1_clear.attr.attr,
    &emif_attr_inf2_init_done.attr.attr,
    &emif_attr_inf2_cal_fail.attr.attr,
    &emif_attr_inf2_clear.attr.attr,
    &emif_attr_inf3_init_done.attr.attr,
    &emif_attr_inf3_cal_fail.attr.attr,
    &emif_attr_inf3_clear.attr.attr,
    &emif_attr_inf4_init_done.attr.attr,
    &emif_attr_inf4_cal_fail.attr.attr,
    &emif_attr_inf4_clear.attr.attr,
    &emif_attr_inf5_init_done.attr.attr,
    &emif_attr_inf5_cal_fail.attr.attr,
    &emif_attr_inf5_clear.attr.attr,
    &emif_attr_inf6_init_done.attr.attr,
    &emif_attr_inf6_cal_fail.attr.attr,
    &emif_attr_inf6_clear.attr.attr,
    &emif_attr_inf7_init_done.attr.attr,
    &emif_attr_inf7_cal_fail.attr.attr,
    &emif_attr_inf7_clear.attr.attr,
    core::ptr::null_mut(),
    };
    static umode_t dfl_emif_visible(struct kobject *kobj,
    struct attribute *attr, int n)
    {
    struct dfl_emif *de = dev_get_drvdata(kobj_to_dev(kobj));
    struct emif_attr *eattr = container_of(attr, struct emif_attr,
    attr.attr);
    struct dfl_device *ddev = to_dfl_dev(de.dev);
    u64 val;
//
// This device supports up to 8 memory interfaces, but not all
// interfaces are used on different platforms. The read out value of
// CAPABILITY_CHN_MSK field (which is a bitmap) indicates which
// interfaces are available.
//
    if (ddev.revision > 0 && strstr(attr.name, "_clear"))
    return 0;
    if (ddev.revision == 0)
    val = FIELD_GET(EMIF_CAPABILITY_CHN_MSK_V0,
    readq(de.base + EMIF_CAPABILITY_BASE));
    else
    val = FIELD_GET(EMIF_CAPABILITY_CHN_MSK,
    readq(de.base + EMIF_CAPABILITY_BASE));
    return (val & BIT_ULL(eattr.index)) ? attr.mode : 0;
    }
    static const struct attribute_group dfl_emif_group = {
    .is_visible = dfl_emif_visible,
    .attrs = dfl_emif_attrs,
    };
    static const struct attribute_group *dfl_emif_groups[] = {
    &dfl_emif_group,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn dfl_emif_probe(ddev: *mut dfl_device) -> c_int {
    static int dfl_emif_probe(struct dfl_device *ddev)
    {
    struct device *dev = &ddev.dev;
    struct dfl_emif *de;
    de = devm_kzalloc(dev, sizeof(*de), GFP_KERNEL);
    if (!de)
    return -ENOMEM;
    de.base = devm_ioremap_resource(dev, &ddev.mmio_res);
    if (IS_ERR(de.base))
    return PTR_ERR(de.base);
    de.dev = dev;
    spin_lock_init(&de.lock);
    dev_set_drvdata(dev, de);
    return 0;
    }
    static const struct dfl_device_id dfl_emif_ids[] = {
    { FME_ID, FME_FEATURE_ID_EMIF },
    { }
    };
    MODULE_DEVICE_TABLE(dfl, dfl_emif_ids);
    static struct dfl_driver dfl_emif_driver = {
    .drv	= {
    .name       = "dfl-emif",
    .dev_groups = dfl_emif_groups,
    },
    .id_table = dfl_emif_ids,
    .probe   = dfl_emif_probe,
    };
    module_dfl_driver(dfl_emif_driver);
    MODULE_DESCRIPTION("DFL EMIF driver");
    MODULE_AUTHOR("Intel Corporation");
    MODULE_LICENSE("GPL v2");
