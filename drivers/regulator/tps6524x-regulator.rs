//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/tps6524x-regulator.c
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


//
// Regulator driver for TPS6524x PMIC
//
// Copyright (C) 2010 Texas Instruments
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation version 2.
//
// This program is distributed "as is" WITHOUT ANY WARRANTY of any kind,
// whether express or implied; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//

pub const REG_LDO_SET: c_uint = 0x0;

pub const LDO_VSEL_MASK: c_uint = 0x0f;
pub const LDO2_ILIM_SHIFT: c_int = 12;
pub const LDO2_VSEL_SHIFT: c_int = 4;
pub const LDO1_ILIM_SHIFT: c_int = 8;
pub const LDO1_VSEL_SHIFT: c_int = 0;
pub const REG_BLOCK_EN: c_uint = 0x1;
pub const BLOCK_MASK: c_int = 1;
pub const BLOCK_LDO1_SHIFT: c_int = 0;
pub const BLOCK_LDO2_SHIFT: c_int = 1;
pub const BLOCK_LCD_SHIFT: c_int = 2;
pub const BLOCK_USB_SHIFT: c_int = 3;
pub const REG_DCDC_SET: c_uint = 0x2;
pub const DCDC_VDCDC_MASK: c_uint = 0x1f;
pub const DCDC_VDCDC1_SHIFT: c_int = 0;
pub const DCDC_VDCDC2_SHIFT: c_int = 5;
pub const DCDC_VDCDC3_SHIFT: c_int = 10;
pub const REG_DCDC_EN: c_uint = 0x3;
pub const DCDCDCDC_EN_MASK: c_uint = 0x1;
pub const DCDCDCDC1_EN_SHIFT: c_int = 0;

pub const DCDCDCDC2_EN_SHIFT: c_int = 2;

pub const DCDCDCDC3_EN_SHIFT: c_int = 4;

pub const REG_USB: c_uint = 0x4;
pub const USB_ILIM_SHIFT: c_int = 0;
pub const USB_ILIM_MASK: c_uint = 0x3;
pub const USB_TSD_SHIFT: c_int = 2;
pub const USB_TSD_MASK: c_uint = 0x3;
pub const USB_TWARN_SHIFT: c_int = 4;
pub const USB_TWARN_MASK: c_uint = 0x3;

pub const REG_ALARM: c_uint = 0x5;

pub const REG_INT_ENABLE: c_uint = 0x6;

pub const REG_INT_STATUS: c_uint = 0x7;

pub const REG_SOFTWARE_RESET: c_uint = 0xb;
pub const REG_WRITE_ENABLE: c_uint = 0xd;
pub const REG_REV_ID: c_uint = 0xf;
pub const N_DCDC: c_int = 3;
pub const N_LDO: c_int = 2;
pub const N_SWITCH: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct field {
    pub reg: c_int,
    pub shift: c_int,
    pub mask: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct supply_info {
    pub name: *const c_char,
    pub n_voltages: c_int,
    pub voltages: *const c_uint,
    pub n_ilimsels: c_int,
    pub ilimsels: *const c_uint,
    pub ilimsel: field enable, voltage,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps6524x {
    pub dev: *mut device,
    pub spi: *mut spi_device,
    pub lock: mutex,
    pub desc: [regulator_desc; N_REGULATORS],
}

#[no_mangle]
unsafe extern "C" fn __read_reg(hw: *mut tps6524x, reg: c_int) -> c_int {
    static int __read_reg(struct tps6524x *hw, int reg)
    {
    let mut error: c_int = 0;
    let mut cmd: u16 = CMD_READ(reg), in;
    u8 status;
    struct spi_message m;
    struct spi_transfer t[3];
    spi_message_init(&m);
    memset(t, 0, sizeof(t));
    t[0].tx_buf = &cmd;
    t[0].len = 2;
    t[0].bits_per_word = 12;
    spi_message_add_tail(&t[0], &m);
    t[1].rx_buf = &in;
    t[1].len = 2;
    t[1].bits_per_word = 16;
    spi_message_add_tail(&t[1], &m);
    t[2].rx_buf = &status;
    t[2].len = 1;
    t[2].bits_per_word = 4;
    spi_message_add_tail(&t[2], &m);
    error = spi_sync(hw.spi, &m);
    if (error < 0)
    return error;
    dev_dbg(hw.dev, "read reg %d, data %x, status %x\n",
    reg, in, status);
    if (!(status & STAT_CLK) || (status & STAT_WRITE))
    return -EIO;
    if (status & STAT_INVALID)
    return -EINVAL;
    return in;
    }
#[no_mangle]
unsafe extern "C" fn read_reg(hw: *mut tps6524x, reg: c_int) -> c_int {
    static int read_reg(struct tps6524x *hw, int reg)
    {
    int ret;
    mutex_lock(&hw.lock);
    ret = __read_reg(hw, reg);
    mutex_unlock(&hw.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __write_reg(hw: *mut tps6524x, reg: c_int, val: c_int) -> c_int {
    static int __write_reg(struct tps6524x *hw, int reg, int val)
    {
    let mut error: c_int = 0;
    let mut cmd: u16 = CMD_WRITE(reg), out = val;
    u8 status;
    struct spi_message m;
    struct spi_transfer t[3];
    spi_message_init(&m);
    memset(t, 0, sizeof(t));
    t[0].tx_buf = &cmd;
    t[0].len = 2;
    t[0].bits_per_word = 12;
    spi_message_add_tail(&t[0], &m);
    t[1].tx_buf = &out;
    t[1].len = 2;
    t[1].bits_per_word = 16;
    spi_message_add_tail(&t[1], &m);
    t[2].rx_buf = &status;
    t[2].len = 1;
    t[2].bits_per_word = 4;
    spi_message_add_tail(&t[2], &m);
    error = spi_sync(hw.spi, &m);
    if (error < 0)
    return error;
    dev_dbg(hw.dev, "wrote reg %d, data %x, status %x\n",
    reg, out, status);
    if (!(status & STAT_CLK) || !(status & STAT_WRITE))
    return -EIO;
    if (status & (STAT_INVALID | STAT_WP))
    return -EINVAL;
    return error;
    }
#[no_mangle]
unsafe extern "C" fn __rmw_reg(hw: *mut tps6524x, reg: c_int, mask: c_int, val: c_int) -> c_int {
    static int __rmw_reg(struct tps6524x *hw, int reg, int mask, int val)
    {
    int ret;
    ret = __read_reg(hw, reg);
    if (ret < 0)
    return ret;
    ret &= ~mask;
    ret |= val;
    ret = __write_reg(hw, reg, ret);
    return (ret < 0) ? ret : 0;
    }
#[no_mangle]
unsafe extern "C" fn rmw_protect(hw: *mut tps6524x, reg: c_int, mask: c_int, val: c_int) -> c_int {
    static int rmw_protect(struct tps6524x *hw, int reg, int mask, int val)
    {
    int ret;
    mutex_lock(&hw.lock);
    ret = __write_reg(hw, REG_WRITE_ENABLE, 1);
    if (ret) {
    dev_err(hw.dev, "failed to set write enable\n");
    goto error;
    }
    ret = __rmw_reg(hw, reg, mask, val);
    if (ret)
    dev_err(hw.dev, "failed to rmw register %d\n", reg);
    ret = __write_reg(hw, REG_WRITE_ENABLE, 0);
    if (ret) {
    dev_err(hw.dev, "failed to clear write enable\n");
    goto error;
    }
    error:
    mutex_unlock(&hw.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn read_field(hw: *mut tps6524x, field: *const field) -> c_int {
    static int read_field(struct tps6524x *hw, const struct field *field)
    {
    int tmp;
    tmp = read_reg(hw, field.reg);
    if (tmp < 0)
    return tmp;
    return (tmp >> field.shift) & field.mask;
    }
    static int write_field(struct tps6524x *hw, const struct field *field,
    int val)
    {
    if (val & ~field.mask)
    return -EOVERFLOW;
    return rmw_protect(hw, field.reg,
    field.mask << field.shift,
    val << field.shift);
    }
    static const unsigned int dcdc1_voltages[] = {
    800000,  825000,  850000,  875000,
    900000,  925000,  950000,  975000,
    1000000, 1025000, 1050000, 1075000,
    1100000, 1125000, 1150000, 1175000,
    1200000, 1225000, 1250000, 1275000,
    1300000, 1325000, 1350000, 1375000,
    1400000, 1425000, 1450000, 1475000,
    1500000, 1525000, 1550000, 1575000,
    };
    static const unsigned int dcdc2_voltages[] = {
    1400000, 1450000, 1500000, 1550000,
    1600000, 1650000, 1700000, 1750000,
    1800000, 1850000, 1900000, 1950000,
    2000000, 2050000, 2100000, 2150000,
    2200000, 2250000, 2300000, 2350000,
    2400000, 2450000, 2500000, 2550000,
    2600000, 2650000, 2700000, 2750000,
    2800000, 2850000, 2900000, 2950000,
    };
    static const unsigned int dcdc3_voltages[] = {
    2400000, 2450000, 2500000, 2550000, 2600000,
    2650000, 2700000, 2750000, 2800000, 2850000,
    2900000, 2950000, 3000000, 3050000, 3100000,
    3150000, 3200000, 3250000, 3300000, 3350000,
    3400000, 3450000, 3500000, 3550000, 3600000,
    };
    static const unsigned int ldo1_voltages[] = {
    4300000, 4350000, 4400000, 4450000,
    4500000, 4550000, 4600000, 4650000,
    4700000, 4750000, 4800000, 4850000,
    4900000, 4950000, 5000000, 5050000,
    };
    static const unsigned int ldo2_voltages[] = {
    1100000, 1150000, 1200000, 1250000,
    1300000, 1700000, 1750000, 1800000,
    1850000, 1900000, 3150000, 3200000,
    3250000, 3300000, 3350000, 3400000,
    };
    static const unsigned int fixed_5000000_voltage[] = {
    5000000
    };
    static const unsigned int ldo_ilimsel[] = {
    400000, 1500000
    };
    static const unsigned int usb_ilimsel[] = {
    200000, 400000, 800000, 1000000
    };
    static const unsigned int fixed_2400000_ilimsel[] = {
    2400000
    };
    static const unsigned int fixed_1200000_ilimsel[] = {
    1200000
    };
    static const unsigned int fixed_400000_ilimsel[] = {
    400000
    };

    { .reg = (_reg), .mask = (_mask), .shift = (_shift), }
    static const struct supply_info supply_info[N_REGULATORS] = {
    {
    .name		= "DCDC1",
    .n_voltages	= ARRAY_SIZE(dcdc1_voltages),
    .voltages	= dcdc1_voltages,
    .n_ilimsels	= ARRAY_SIZE(fixed_2400000_ilimsel),
    .ilimsels	= fixed_2400000_ilimsel,
    .enable		= __MK_FIELD(REG_DCDC_EN, DCDCDCDC_EN_MASK,
    DCDCDCDC1_EN_SHIFT),
    .voltage	= __MK_FIELD(REG_DCDC_SET, DCDC_VDCDC_MASK,
    DCDC_VDCDC1_SHIFT),
    },
    {
    .name		= "DCDC2",
    .n_voltages	= ARRAY_SIZE(dcdc2_voltages),
    .voltages	= dcdc2_voltages,
    .n_ilimsels	= ARRAY_SIZE(fixed_1200000_ilimsel),
    .ilimsels	= fixed_1200000_ilimsel,
    .enable		= __MK_FIELD(REG_DCDC_EN, DCDCDCDC_EN_MASK,
    DCDCDCDC2_EN_SHIFT),
    .voltage	= __MK_FIELD(REG_DCDC_SET, DCDC_VDCDC_MASK,
    DCDC_VDCDC2_SHIFT),
    },
    {
    .name		= "DCDC3",
    .n_voltages	= ARRAY_SIZE(dcdc3_voltages),
    .voltages	= dcdc3_voltages,
    .n_ilimsels	= ARRAY_SIZE(fixed_1200000_ilimsel),
    .ilimsels	= fixed_1200000_ilimsel,
    .enable		= __MK_FIELD(REG_DCDC_EN, DCDCDCDC_EN_MASK,
    DCDCDCDC3_EN_SHIFT),
    .voltage	= __MK_FIELD(REG_DCDC_SET, DCDC_VDCDC_MASK,
    DCDC_VDCDC3_SHIFT),
    },
    {
    .name		= "LDO1",
    .n_voltages	= ARRAY_SIZE(ldo1_voltages),
    .voltages	= ldo1_voltages,
    .n_ilimsels	= ARRAY_SIZE(ldo_ilimsel),
    .ilimsels	= ldo_ilimsel,
    .enable		= __MK_FIELD(REG_BLOCK_EN, BLOCK_MASK,
    BLOCK_LDO1_SHIFT),
    .voltage	= __MK_FIELD(REG_LDO_SET, LDO_VSEL_MASK,
    LDO1_VSEL_SHIFT),
    .ilimsel	= __MK_FIELD(REG_LDO_SET, LDO_ILIM_MASK,
    LDO1_ILIM_SHIFT),
    },
    {
    .name		= "LDO2",
    .n_voltages	= ARRAY_SIZE(ldo2_voltages),
    .voltages	= ldo2_voltages,
    .n_ilimsels	= ARRAY_SIZE(ldo_ilimsel),
    .ilimsels	= ldo_ilimsel,
    .enable		= __MK_FIELD(REG_BLOCK_EN, BLOCK_MASK,
    BLOCK_LDO2_SHIFT),
    .voltage	= __MK_FIELD(REG_LDO_SET, LDO_VSEL_MASK,
    LDO2_VSEL_SHIFT),
    .ilimsel	= __MK_FIELD(REG_LDO_SET, LDO_ILIM_MASK,
    LDO2_ILIM_SHIFT),
    },
    {
    .name		= "USB",
    .n_voltages	= ARRAY_SIZE(fixed_5000000_voltage),
    .voltages	= fixed_5000000_voltage,
    .n_ilimsels	= ARRAY_SIZE(usb_ilimsel),
    .ilimsels	= usb_ilimsel,
    .enable		= __MK_FIELD(REG_BLOCK_EN, BLOCK_MASK,
    BLOCK_USB_SHIFT),
    .ilimsel	= __MK_FIELD(REG_USB, USB_ILIM_MASK,
    USB_ILIM_SHIFT),
    },
    {
    .name		= "LCD",
    .n_voltages	= ARRAY_SIZE(fixed_5000000_voltage),
    .voltages	= fixed_5000000_voltage,
    .n_ilimsels	= ARRAY_SIZE(fixed_400000_ilimsel),
    .ilimsels	= fixed_400000_ilimsel,
    .enable		= __MK_FIELD(REG_BLOCK_EN, BLOCK_MASK,
    BLOCK_LCD_SHIFT),
    },
    };
#[no_mangle]
unsafe extern "C" fn set_voltage_sel(rdev: *mut regulator_dev, selector: unsigned) -> c_int {
    static int set_voltage_sel(struct regulator_dev *rdev, unsigned selector)
    {
    const struct supply_info *info;
    struct tps6524x *hw;
    hw	= rdev_get_drvdata(rdev);
    info	= &supply_info[rdev_get_id(rdev)];
    if (rdev.desc.n_voltages == 1)
    return -EINVAL;
    return write_field(hw, &info.voltage, selector);
    }
#[no_mangle]
unsafe extern "C" fn get_voltage_sel(rdev: *mut regulator_dev) -> c_int {
    static int get_voltage_sel(struct regulator_dev *rdev)
    {
    const struct supply_info *info;
    struct tps6524x *hw;
    int ret;
    hw	= rdev_get_drvdata(rdev);
    info	= &supply_info[rdev_get_id(rdev)];
    if (rdev.desc.n_voltages == 1)
    return 0;
    ret = read_field(hw, &info.voltage);
    if (ret < 0)
    return ret;
    if (WARN_ON(ret >= info.n_voltages))
    return -EIO;
    return ret;
    }
    static int set_current_limit(struct regulator_dev *rdev, int min_uA,
    int max_uA)
    {
    const struct supply_info *info;
    struct tps6524x *hw;
    int i;
    hw	= rdev_get_drvdata(rdev);
    info	= &supply_info[rdev_get_id(rdev)];
    if (info.n_ilimsels == 1)
    return -EINVAL;
    for (i = info.n_ilimsels - 1; i >= 0; i--) {
    if (min_uA <= info.ilimsels[i] &&
    max_uA >= info.ilimsels[i])
    return write_field(hw, &info.ilimsel, i);
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn get_current_limit(rdev: *mut regulator_dev) -> c_int {
    static int get_current_limit(struct regulator_dev *rdev)
    {
    const struct supply_info *info;
    struct tps6524x *hw;
    int ret;
    hw	= rdev_get_drvdata(rdev);
    info	= &supply_info[rdev_get_id(rdev)];
    if (info.n_ilimsels == 1)
    return info.ilimsels[0];
    ret = read_field(hw, &info.ilimsel);
    if (ret < 0)
    return ret;
    if (WARN_ON(ret >= info.n_ilimsels))
    return -EIO;
    return info.ilimsels[ret];
    }
#[no_mangle]
unsafe extern "C" fn enable_supply(rdev: *mut regulator_dev) -> c_int {
    static int enable_supply(struct regulator_dev *rdev)
    {
    const struct supply_info *info;
    struct tps6524x *hw;
    hw	= rdev_get_drvdata(rdev);
    info	= &supply_info[rdev_get_id(rdev)];
    return write_field(hw, &info.enable, 1);
    }
#[no_mangle]
unsafe extern "C" fn disable_supply(rdev: *mut regulator_dev) -> c_int {
    static int disable_supply(struct regulator_dev *rdev)
    {
    const struct supply_info *info;
    struct tps6524x *hw;
    hw	= rdev_get_drvdata(rdev);
    info	= &supply_info[rdev_get_id(rdev)];
    return write_field(hw, &info.enable, 0);
    }
#[no_mangle]
unsafe extern "C" fn is_supply_enabled(rdev: *mut regulator_dev) -> c_int {
    static int is_supply_enabled(struct regulator_dev *rdev)
    {
    const struct supply_info *info;
    struct tps6524x *hw;
    hw	= rdev_get_drvdata(rdev);
    info	= &supply_info[rdev_get_id(rdev)];
    return read_field(hw, &info.enable);
    }
    static const struct regulator_ops regulator_ops = {
    .is_enabled		= is_supply_enabled,
    .enable			= enable_supply,
    .disable		= disable_supply,
    .get_voltage_sel	= get_voltage_sel,
    .set_voltage_sel	= set_voltage_sel,
    .list_voltage		= regulator_list_voltage_table,
    .map_voltage		= regulator_map_voltage_ascend,
    .set_current_limit	= set_current_limit,
    .get_current_limit	= get_current_limit,
    };
#[no_mangle]
unsafe extern "C" fn pmic_probe(spi: *mut spi_device) -> c_int {
    static int pmic_probe(struct spi_device *spi)
    {
    struct tps6524x *hw;
    struct device *dev = &spi.dev;
    const struct supply_info *info = supply_info;
    struct regulator_init_data *init_data;
    let mut config: regulator_config = { };
    struct regulator_dev *rdev;
    int i;
    init_data = dev_get_platdata(dev);
    if (!init_data) {
    dev_err(dev, "could not find regulator platform data\n");
    return -EINVAL;
    }
    hw = devm_kzalloc(&spi.dev, sizeof(struct tps6524x), GFP_KERNEL);
    if (!hw)
    return -ENOMEM;
    spi_set_drvdata(spi, hw);
    hw.dev = dev;
    hw.spi = spi;
    mutex_init(&hw.lock);
    for (i = 0; i < N_REGULATORS; i++, info++, init_data++) {
    hw.desc[i].name	= info.name;
    hw.desc[i].id		= i;
    hw.desc[i].n_voltages	= info.n_voltages;
    hw.desc[i].volt_table	= info.voltages;
    hw.desc[i].ops		= &regulator_ops;
    hw.desc[i].type	= REGULATOR_VOLTAGE;
    hw.desc[i].owner	= THIS_MODULE;
    config.dev = dev;
    config.init_data = init_data;
    config.driver_data = hw;
    rdev = devm_regulator_register(dev, &hw.desc[i], &config);
    if (IS_ERR(rdev))
    return PTR_ERR(rdev);
    }
    return 0;
    }
    static struct spi_driver pmic_driver = {
    .probe		= pmic_probe,
    .driver		= {
    .name	= "tps6524x",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
    module_spi_driver(pmic_driver);
    MODULE_DESCRIPTION("TPS6524X PMIC Driver");
    MODULE_AUTHOR("Cyril Chemparathy");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("spi:tps6524x");
