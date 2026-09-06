//! Automatically rewritten from C to Rust
//! Source: drivers/memory/brcmstb_memc.c
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
// DDR Self-Refresh Power Down (SRPD) support for Broadcom STB SoCs
//

pub const REG_MEMC_CNTRLR_CONFIG: c_uint = 0x00;
pub const CNTRLR_CONFIG_LPDDR4_SHIFT: c_int = 5;
pub const CNTRLR_CONFIG_LPDDR5_SHIFT: c_int = 6;
pub const CNTRLR_CONFIG_MASK: c_uint = 0xf;
pub const REG_MEMC_SRPD_CFG_21: c_uint = 0x20;
pub const REG_MEMC_SRPD_CFG_20: c_uint = 0x34;
pub const REG_MEMC_SRPD_CFG_1x: c_uint = 0x3c;
pub const INACT_COUNT_SHIFT: c_int = 0;
pub const INACT_COUNT_MASK: c_uint = 0xffff;
pub const SRPD_EN_SHIFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmstb_memc_data {
    pub srpd_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmstb_memc {
    pub dev: *mut device,
    pub ddr_ctrl: *mut void __iomem,
    pub timeout_cycles: c_uint,
    pub frequency: u32,
    pub srpd_offset: u32,
}

#[no_mangle]
unsafe extern "C" fn brcmstb_memc_uses_lpddr45(memc: *mut brcmstb_memc) -> c_int {
    static int brcmstb_memc_uses_lpddr45(struct brcmstb_memc *memc)
    {
    void __iomem *config = memc.ddr_ctrl + REG_MEMC_CNTRLR_CONFIG;
    u32 reg;
    reg = readl_relaxed(config) & CNTRLR_CONFIG_MASK;
    return reg == CNTRLR_CONFIG_LPDDR4_SHIFT ||
    reg == CNTRLR_CONFIG_LPDDR5_SHIFT;
    }
    static int brcmstb_memc_srpd_config(struct brcmstb_memc *memc,
    unsigned int cycles)
    {
    void __iomem *cfg = memc.ddr_ctrl + memc.srpd_offset;
    u32 val;
// Max timeout supported in HW
    if (cycles > INACT_COUNT_MASK)
    return -EINVAL;
    memc.timeout_cycles = cycles;
    val = (cycles << INACT_COUNT_SHIFT) & INACT_COUNT_MASK;
    if (cycles)
    val |= BIT(SRPD_EN_SHIFT);
    writel_relaxed(val, cfg);
// Ensure the write is committed to the controller
    (void)readl_relaxed(cfg);
    return 0;
    }
    static ssize_t frequency_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct brcmstb_memc *memc = dev_get_drvdata(dev);
    return sprintf(buf, "%d\n", memc.frequency);
    }
    static ssize_t srpd_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct brcmstb_memc *memc = dev_get_drvdata(dev);
    return sprintf(buf, "%d\n", memc.timeout_cycles);
    }
    static ssize_t srpd_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct brcmstb_memc *memc = dev_get_drvdata(dev);
    unsigned int val;
    int ret;
//
// Cannot change the inactivity timeout on LPDDR4 chips because the
// dynamic tuning process will also get affected by the inactivity
// timeout, thus making it non functional.
//
    if (brcmstb_memc_uses_lpddr45(memc))
    return -EOPNOTSUPP;
    ret = kstrtouint(buf, 10, &val);
    if (ret < 0)
    return ret;
    ret = brcmstb_memc_srpd_config(memc, val);
    if (ret)
    return ret;
    return count;
    }
    static DEVICE_ATTR_RO(frequency);
    static DEVICE_ATTR_RW(srpd);
    static struct attribute *dev_attrs[] = {
    &dev_attr_frequency.attr,
    &dev_attr_srpd.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group dev_attr_group = {
    .attrs = dev_attrs,
    };
#[no_mangle]
unsafe extern "C" fn brcmstb_memc_probe(pdev: *mut platform_device) -> c_int {
    static int brcmstb_memc_probe(struct platform_device *pdev)
    {
    const struct brcmstb_memc_data *memc_data;
    struct device *dev = &pdev.dev;
    struct brcmstb_memc *memc;
    int ret;
    memc = devm_kzalloc(dev, sizeof(*memc), GFP_KERNEL);
    if (!memc)
    return -ENOMEM;
    dev_set_drvdata(dev, memc);
    memc_data = device_get_match_data(dev);
    memc.srpd_offset = memc_data.srpd_offset;
    memc.ddr_ctrl = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(memc.ddr_ctrl))
    return PTR_ERR(memc.ddr_ctrl);
    of_property_read_u32(pdev.dev.of_node, "clock-frequency",
    &memc.frequency);
    ret = sysfs_create_group(&dev.kobj, &dev_attr_group);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn brcmstb_memc_remove(pdev: *mut platform_device) {
    static void brcmstb_memc_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    sysfs_remove_group(&dev.kobj, &dev_attr_group);
    }
    enum brcmstb_memc_hwtype {
    BRCMSTB_MEMC_V21,
    BRCMSTB_MEMC_V20,
    BRCMSTB_MEMC_V1X,
    };
    static const struct brcmstb_memc_data brcmstb_memc_versions[] = {
    { .srpd_offset = REG_MEMC_SRPD_CFG_21 },
    { .srpd_offset = REG_MEMC_SRPD_CFG_20 },
    { .srpd_offset = REG_MEMC_SRPD_CFG_1x },
    };
    static const struct of_device_id brcmstb_memc_of_match[] = {
    {
    .compatible = "brcm,brcmstb-memc-ddr-rev-b.1.x",
    .data = &brcmstb_memc_versions[BRCMSTB_MEMC_V1X]
    },
    {
    .compatible = "brcm,brcmstb-memc-ddr-rev-b.2.0",
    .data = &brcmstb_memc_versions[BRCMSTB_MEMC_V20]
    },
    {
    .compatible = "brcm,brcmstb-memc-ddr-rev-b.2.1",
    .data = &brcmstb_memc_versions[BRCMSTB_MEMC_V21]
    },
// default to the V21 offset
    {
    .compatible = "brcm,brcmstb-memc-ddr",
    .data = &brcmstb_memc_versions[BRCMSTB_MEMC_V21]
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, brcmstb_memc_of_match);
#[no_mangle]
unsafe extern "C" fn brcmstb_memc_suspend(dev: *mut device) -> c_int {
    static int brcmstb_memc_suspend(struct device *dev)
    {
    struct brcmstb_memc *memc = dev_get_drvdata(dev);
    void __iomem *cfg = memc.ddr_ctrl + memc.srpd_offset;
    u32 val;
    if (memc.timeout_cycles == 0)
    return 0;
//
// Disable SRPD prior to suspending the system since that can
// cause issues with other memory clients managed by the ARM
// trusted firmware to access memory.
//
    val = readl_relaxed(cfg);
    val &= ~BIT(SRPD_EN_SHIFT);
    writel_relaxed(val, cfg);
// Ensure the write is committed to the controller
    (void)readl_relaxed(cfg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn brcmstb_memc_resume(dev: *mut device) -> c_int {
    static int brcmstb_memc_resume(struct device *dev)
    {
    struct brcmstb_memc *memc = dev_get_drvdata(dev);
    if (memc.timeout_cycles == 0)
    return 0;
    return brcmstb_memc_srpd_config(memc, memc.timeout_cycles);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(brcmstb_memc_pm_ops, brcmstb_memc_suspend,
    brcmstb_memc_resume);
    static struct platform_driver brcmstb_memc_driver = {
    .probe = brcmstb_memc_probe,
    .remove = brcmstb_memc_remove,
    .driver = {
    .name		= "brcmstb_memc",
    .of_match_table	= brcmstb_memc_of_match,
    .pm		= pm_ptr(&brcmstb_memc_pm_ops),
    },
    };
    module_platform_driver(brcmstb_memc_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Broadcom");
    MODULE_DESCRIPTION("DDR SRPD driver for Broadcom STB chips");
