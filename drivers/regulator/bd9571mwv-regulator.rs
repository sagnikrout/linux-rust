//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/bd9571mwv-regulator.c
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
// ROHM BD9571MWV-M and BD9574MWF-M regulator driver
//
// Copyright (C) 2017 Marek Vasut <marek.vasut+renesas@gmail.com>
//
// Based on the TPS65086 driver
//
// NOTE: VD09 is missing
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd9571mwv_reg {
    pub regmap: *mut regmap,
// DDR Backup Power
    pub /: *mut *mut u8 bkup_mode_cnt_keepon; / from "rohm,ddr-backup-power",
    pub bkup_mode_cnt_saved: u8,
    pub bkup_mode_enabled: bool,
// Power switch type
    pub rstbmode_level: bool,
    pub rstbmode_pulse: bool,
}

    enum bd9571mwv_regulators { VD09, VD18, VD25, VD33, DVFS };

    {							\
    .name			= _name,		\
    .of_match		= of_match_ptr(_of),	\
    .regulators_node	= "regulators",		\
    .id			= _id,			\
    .ops			= &_ops,		\
    .n_voltages		= _nv,			\
    .type			= REGULATOR_VOLTAGE,	\
    .owner			= THIS_MODULE,		\
    .vsel_reg		= _vr,			\
    .vsel_mask		= _vm,			\
    .min_uV			= _min,			\
    .uV_step		= _step,		\
    .linear_min_sel		= _lmin,		\
    }
#[no_mangle]
unsafe extern "C" fn bd9571mwv_avs_get_moni_state(rdev: *mut regulator_dev) -> c_int {
    static int bd9571mwv_avs_get_moni_state(struct regulator_dev *rdev)
    {
    unsigned int val;
    int ret;
    ret = regmap_read(rdev.regmap, BD9571MWV_AVS_SET_MONI, &val);
    if (ret != 0)
    return ret;
    return val & BD9571MWV_AVS_SET_MONI_MASK;
    }
    static int bd9571mwv_avs_set_voltage_sel_regmap(struct regulator_dev *rdev,
    unsigned int sel)
    {
    int ret;
    ret = bd9571mwv_avs_get_moni_state(rdev);
    if (ret < 0)
    return ret;
    return regmap_write_bits(rdev.regmap, BD9571MWV_AVS_VD09_VID(ret),
    rdev.desc.vsel_mask, sel);
    }
#[no_mangle]
unsafe extern "C" fn bd9571mwv_avs_get_voltage_sel_regmap(rdev: *mut regulator_dev) -> c_int {
    static int bd9571mwv_avs_get_voltage_sel_regmap(struct regulator_dev *rdev)
    {
    unsigned int val;
    int ret;
    ret = bd9571mwv_avs_get_moni_state(rdev);
    if (ret < 0)
    return ret;
    ret = regmap_read(rdev.regmap, BD9571MWV_AVS_VD09_VID(ret), &val);
    if (ret != 0)
    return ret;
    val &= rdev.desc.vsel_mask;
    val >>= ffs(rdev.desc.vsel_mask) - 1;
    return val;
    }
    static int bd9571mwv_reg_set_voltage_sel_regmap(struct regulator_dev *rdev,
    unsigned int sel)
    {
    return regmap_write_bits(rdev.regmap, BD9571MWV_DVFS_SETVID,
    rdev.desc.vsel_mask, sel);
    }
// Operations permitted on AVS voltage regulator
    static const struct regulator_ops avs_ops = {
    .set_voltage_sel	= bd9571mwv_avs_set_voltage_sel_regmap,
    .map_voltage		= regulator_map_voltage_linear,
    .get_voltage_sel	= bd9571mwv_avs_get_voltage_sel_regmap,
    .list_voltage		= regulator_list_voltage_linear,
    };
// Operations permitted on voltage regulators
    static const struct regulator_ops reg_ops = {
    .set_voltage_sel	= bd9571mwv_reg_set_voltage_sel_regmap,
    .map_voltage		= regulator_map_voltage_linear,
    .get_voltage_sel	= regulator_get_voltage_sel_regmap,
    .list_voltage		= regulator_list_voltage_linear,
    };
// Operations permitted on voltage monitors
    static const struct regulator_ops vid_ops = {
    .map_voltage		= regulator_map_voltage_linear,
    .get_voltage_sel	= regulator_get_voltage_sel_regmap,
    .list_voltage		= regulator_list_voltage_linear,
    };
    static const struct regulator_desc regulators[] = {
    BD9571MWV_REG("VD09", "vd09", VD09, avs_ops, 0, 0x7f,
    0x6f, 600000, 10000, 0x3c),
    BD9571MWV_REG("VD18", "vd18", VD18, vid_ops, BD9571MWV_VD18_VID, 0xf,
    16, 1625000, 25000, 0),
    BD9571MWV_REG("VD25", "vd25", VD25, vid_ops, BD9571MWV_VD25_VID, 0xf,
    16, 2150000, 50000, 0),
    BD9571MWV_REG("VD33", "vd33", VD33, vid_ops, BD9571MWV_VD33_VID, 0xf,
    11, 2800000, 100000, 0),
    BD9571MWV_REG("DVFS", "dvfs", DVFS, reg_ops,
    BD9571MWV_DVFS_MONIVDAC, 0x7f,
    0x6f, 600000, 10000, 0x3c),
    };

    static int bd9571mwv_bkup_mode_read(struct bd9571mwv_reg *bdreg,
    unsigned int *mode)
    {
    int ret;
    ret = regmap_read(bdreg.regmap, BD9571MWV_BKUP_MODE_CNT, mode);
    if (ret) {
    dev_err(regmap_get_device(bdreg.regmap),
    "failed to read backup mode (%d)\n", ret);
    return ret;
    }
    return 0;
    }
    static int bd9571mwv_bkup_mode_write(struct bd9571mwv_reg *bdreg,
    unsigned int mode)
    {
    int ret;
    ret = regmap_write(bdreg.regmap, BD9571MWV_BKUP_MODE_CNT, mode);
    if (ret) {
    dev_err(regmap_get_device(bdreg.regmap),
    "failed to configure backup mode 0x%x (%d)\n",
    mode, ret);
    return ret;
    }
    return 0;
    }
    static ssize_t backup_mode_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct bd9571mwv_reg *bdreg = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%s\n", bdreg.bkup_mode_enabled ? "on" : "off");
    }
    static ssize_t backup_mode_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct bd9571mwv_reg *bdreg = dev_get_drvdata(dev);
    unsigned int mode;
    int ret;
    if (!count)
    return 0;
    ret = kstrtobool(buf, &bdreg.bkup_mode_enabled);
    if (ret)
    return ret;
    if (!bdreg.rstbmode_level)
    return count;
//
// Configure DDR Backup Mode, to change the role of the accessory power
// switch from a power switch to a wake-up switch, or vice versa
//
    ret = bd9571mwv_bkup_mode_read(bdreg, &mode);
    if (ret)
    return ret;
    mode &= ~BD9571MWV_BKUP_MODE_CNT_KEEPON_MASK;
    if (bdreg.bkup_mode_enabled)
    mode |= bdreg.bkup_mode_cnt_keepon;
    ret = bd9571mwv_bkup_mode_write(bdreg, mode);
    if (ret)
    return ret;
    return count;
    }
    static DEVICE_ATTR_RW(backup_mode);
#[no_mangle]
unsafe extern "C" fn bd9571mwv_suspend(dev: *mut device) -> c_int {
    static int bd9571mwv_suspend(struct device *dev)
    {
    struct bd9571mwv_reg *bdreg = dev_get_drvdata(dev);
    unsigned int mode;
    int ret;
    if (!bdreg.bkup_mode_enabled)
    return 0;
// Save DDR Backup Mode
    ret = bd9571mwv_bkup_mode_read(bdreg, &mode);
    if (ret)
    return ret;
    bdreg.bkup_mode_cnt_saved = mode;
    if (!bdreg.rstbmode_pulse)
    return 0;
// Enable DDR Backup Mode
    mode &= ~BD9571MWV_BKUP_MODE_CNT_KEEPON_MASK;
    mode |= bdreg.bkup_mode_cnt_keepon;
    if (mode != bdreg.bkup_mode_cnt_saved)
    return bd9571mwv_bkup_mode_write(bdreg, mode);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bd9571mwv_resume(dev: *mut device) -> c_int {
    static int bd9571mwv_resume(struct device *dev)
    {
    struct bd9571mwv_reg *bdreg = dev_get_drvdata(dev);
    if (!bdreg.bkup_mode_enabled)
    return 0;
// Restore DDR Backup Mode
    return bd9571mwv_bkup_mode_write(bdreg, bdreg.bkup_mode_cnt_saved);
    }
    static const struct dev_pm_ops bd9571mwv_pm  = {
    SET_SYSTEM_SLEEP_PM_OPS(bd9571mwv_suspend, bd9571mwv_resume)
    };
#[no_mangle]
unsafe extern "C" fn bd9571mwv_regulator_remove(pdev: *mut platform_device) {
    static void bd9571mwv_regulator_remove(struct platform_device *pdev)
    {
    device_remove_file(&pdev.dev, &dev_attr_backup_mode);
    }

#[no_mangle]
unsafe extern "C" fn bd9571mwv_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int bd9571mwv_regulator_probe(struct platform_device *pdev)
    {
    let mut config: regulator_config = { };
    struct bd9571mwv_reg *bdreg;
    struct regulator_dev *rdev;
    unsigned int val;
    int i;
    let mut chip: enum rohm_chip_type = platform_get_device_id(pdev).driver_data;
    bdreg = devm_kzalloc(&pdev.dev, sizeof(*bdreg), GFP_KERNEL);
    if (!bdreg)
    return -ENOMEM;
    bdreg.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    platform_set_drvdata(pdev, bdreg);
    device_set_of_node_from_dev(&pdev.dev, pdev.dev.parent);
    config.dev = &pdev.dev;
    config.driver_data = bdreg;
    config.regmap = bdreg.regmap;
    for (i = 0; i < ARRAY_SIZE(regulators); i++) {
// BD9574MWF supports DVFS only
    if (chip == ROHM_CHIP_TYPE_BD9574 && regulators[i].id != DVFS)
    continue;
    rdev = devm_regulator_register(&pdev.dev, &regulators[i],
    &config);
    if (IS_ERR(rdev)) {
    dev_err(&pdev.dev, "failed to register %s regulator\n",
    regulators[i].name);
    return PTR_ERR(rdev);
    }
    }
    val = 0;
    of_property_read_u32(config.dev.of_node, "rohm,ddr-backup-power", &val);
    if (val & ~BD9571MWV_BKUP_MODE_CNT_KEEPON_MASK) {
    dev_err(&pdev.dev, "invalid %s mode %u\n",
    "rohm,ddr-backup-power", val);
    return -EINVAL;
    }
    bdreg.bkup_mode_cnt_keepon = val;
    bdreg.rstbmode_level = of_property_read_bool(config.dev.of_node,
    "rohm,rstbmode-level");
    bdreg.rstbmode_pulse = of_property_read_bool(config.dev.of_node,
    "rohm,rstbmode-pulse");
    if (bdreg.rstbmode_level && bdreg.rstbmode_pulse) {
    dev_err(&pdev.dev, "only one rohm,rstbmode-* may be specified");
    return -EINVAL;
    }

    if (bdreg.bkup_mode_cnt_keepon) {
    int ret;
//
// Backup mode is enabled by default in pulse mode, but needs
// explicit user setup in level mode.
//
    bdreg.bkup_mode_enabled = bdreg.rstbmode_pulse;
    ret = device_create_file(&pdev.dev, &dev_attr_backup_mode);
    if (ret)
    return ret;
    }

    return 0;
    }
    static const struct platform_device_id bd9571mwv_regulator_id_table[] = {
    { .name = "bd9571mwv-regulator", .driver_data = ROHM_CHIP_TYPE_BD9571 },
    { .name = "bd9574mwf-regulator", .driver_data = ROHM_CHIP_TYPE_BD9574 },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, bd9571mwv_regulator_id_table);
    static struct platform_driver bd9571mwv_regulator_driver = {
    .driver = {
    .name = "bd9571mwv-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .pm = DEV_PM_OPS,
    },
    .probe = bd9571mwv_regulator_probe,
    .remove = bd9571mwv_regulator_remove,
    .id_table = bd9571mwv_regulator_id_table,
    };
    module_platform_driver(bd9571mwv_regulator_driver);
    MODULE_AUTHOR("Marek Vasut <marek.vasut+renesas@gmail.com>");
    MODULE_DESCRIPTION("BD9571MWV Regulator driver");
    MODULE_LICENSE("GPL v2");
