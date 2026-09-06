//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/lp872x.c
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
// Copyright 2012 Texas Instruments
//
// Author: Milo(Woogyom) Kim <milo.kim@ti.com>
//

// Registers : LP8720/8725 shared
pub const LP872X_GENERAL_CFG: c_uint = 0x00;
pub const LP872X_LDO1_VOUT: c_uint = 0x01;
pub const LP872X_LDO2_VOUT: c_uint = 0x02;
pub const LP872X_LDO3_VOUT: c_uint = 0x03;
pub const LP872X_LDO4_VOUT: c_uint = 0x04;
pub const LP872X_LDO5_VOUT: c_uint = 0x05;
// Registers : LP8720
pub const LP8720_BUCK_VOUT1: c_uint = 0x06;
pub const LP8720_BUCK_VOUT2: c_uint = 0x07;
pub const LP8720_ENABLE: c_uint = 0x08;
// Registers : LP8725
pub const LP8725_LILO1_VOUT: c_uint = 0x06;
pub const LP8725_LILO2_VOUT: c_uint = 0x07;
pub const LP8725_BUCK1_VOUT1: c_uint = 0x08;
pub const LP8725_BUCK1_VOUT2: c_uint = 0x09;
pub const LP8725_BUCK2_VOUT1: c_uint = 0x0A;
pub const LP8725_BUCK2_VOUT2: c_uint = 0x0B;
pub const LP8725_BUCK_CTRL: c_uint = 0x0C;
pub const LP8725_LDO_CTRL: c_uint = 0x0D;
// Mask/shift : LP8720/LP8725 shared
pub const LP872X_VOUT_M: c_uint = 0x1F;
pub const LP872X_START_DELAY_M: c_uint = 0xE0;
pub const LP872X_START_DELAY_S: c_int = 5;

// Mask/shift : LP8720

// Mask/shift : LP8725
pub const LP8725_TIMESTEP_M: c_uint = 0xC0		/* Addr 00h */;
pub const LP8725_TIMESTEP_S: c_int = 6;

pub const LP8725_BUCK_CL_M: c_uint = 0xC0		/* Addr 09h, 0Bh */;
pub const LP8725_BUCK_CL_S: c_int = 6;

pub const LP8725_BUCK2_FPWM_S: c_int = 5;

// PWM mode
pub const LP872X_FORCE_PWM: c_int = 1;
pub const LP872X_AUTO_PWM: c_int = 0;
pub const LP8720_NUM_REGULATORS: c_int = 6;
pub const LP8725_NUM_REGULATORS: c_int = 9;
pub const EXTERN_DVS_USED: c_int = 0;
pub const MAX_DELAY: c_int = 6;
// Default DVS Mode
pub const LP8720_DEFAULT_DVS: c_int = 0;

// dump registers in regmap-debugfs
pub const MAX_REGISTERS: c_uint = 0x0F;
    enum lp872x_id {
    LP8720,
    LP8725,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp872x {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub chipid: enum lp872x_id,
    pub pdata: *mut lp872x_platform_data,
    pub num_regulators: c_int,
    pub dvs_pin: enum gpiod_flags,
}

// LP8720/LP8725 shared voltage table for LDOs
    static const unsigned int lp872x_ldo_vtbl[] = {
    1200000, 1250000, 1300000, 1350000, 1400000, 1450000, 1500000, 1550000,
    1600000, 1650000, 1700000, 1750000, 1800000, 1850000, 1900000, 2000000,
    2100000, 2200000, 2300000, 2400000, 2500000, 2600000, 2650000, 2700000,
    2750000, 2800000, 2850000, 2900000, 2950000, 3000000, 3100000, 3300000,
    };
// LP8720 LDO4 voltage table
    static const unsigned int lp8720_ldo4_vtbl[] = {
    800000,  850000,  900000, 1000000, 1100000, 1200000, 1250000, 1300000,
    1350000, 1400000, 1450000, 1500000, 1550000, 1600000, 1650000, 1700000,
    1750000, 1800000, 1850000, 1900000, 2000000, 2100000, 2200000, 2300000,
    2400000, 2500000, 2600000, 2650000, 2700000, 2750000, 2800000, 2850000,
    };
// LP8725 LILO(Low Input Low Output) voltage table
    static const unsigned int lp8725_lilo_vtbl[] = {
    800000,  850000,  900000,  950000, 1000000, 1050000, 1100000, 1150000,
    1200000, 1250000, 1300000, 1350000, 1400000, 1500000, 1600000, 1700000,
    1800000, 1900000, 2000000, 2100000, 2200000, 2300000, 2400000, 2500000,
    2600000, 2700000, 2800000, 2850000, 2900000, 3000000, 3100000, 3300000,
    };
// LP8720 BUCK voltage table

    static const unsigned int lp8720_buck_vtbl[] = {
    EXT_R,  800000,  850000,  900000,  950000, 1000000, 1050000, 1100000,
    1150000, 1200000, 1250000, 1300000, 1350000, 1400000, 1450000, 1500000,
    1550000, 1600000, 1650000, 1700000, 1750000, 1800000, 1850000, 1900000,
    1950000, 2000000, 2050000, 2100000, 2150000, 2200000, 2250000, 2300000,
    };
// LP8725 BUCK voltage table
    static const unsigned int lp8725_buck_vtbl[] = {
    800000,  850000,  900000,  950000, 1000000, 1050000, 1100000, 1150000,
    1200000, 1250000, 1300000, 1350000, 1400000, 1500000, 1600000, 1700000,
    1750000, 1800000, 1850000, 1900000, 2000000, 2100000, 2200000, 2300000,
    2400000, 2500000, 2600000, 2700000, 2800000, 2850000, 2900000, 3000000,
    };
// LP8725 BUCK current limit
    static const unsigned int lp8725_buck_uA[] = {
    460000, 780000, 1050000, 1370000,
    };
#[no_mangle]
unsafe extern "C" fn lp872x_read_byte(lp: *mut lp872x, addr: u8, data: *mut u8) -> c_int {
    static int lp872x_read_byte(struct lp872x *lp, u8 addr, u8 *data)
    {
    int ret;
    unsigned int val;
    ret = regmap_read(lp.regmap, addr, &val);
    if (ret < 0) {
    dev_err(lp.dev, "failed to read 0x%.2x\n", addr);
    return ret;
    }
// data = (u8)val;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn lp872x_write_byte(lp: *mut lp872x, addr: u8, data: u8) -> c_int {
    static inline int lp872x_write_byte(struct lp872x *lp, u8 addr, u8 data)
    {
    return regmap_write(lp.regmap, addr, data);
    }
    static inline int lp872x_update_bits(struct lp872x *lp, u8 addr,
    unsigned int mask, u8 data)
    {
    return regmap_update_bits(lp.regmap, addr, mask, data);
    }
#[no_mangle]
unsafe extern "C" fn lp872x_get_timestep_usec(lp: *mut lp872x) -> c_int {
    static int lp872x_get_timestep_usec(struct lp872x *lp)
    {
    let mut chip: enum lp872x_id = lp.chipid;
    u8 val, mask, shift;
    int *time_usec, size, ret;
    int lp8720_time_usec[] = { 25, 50 };
    int lp8725_time_usec[] = { 32, 64, 128, 256 };
    switch (chip) {
    case LP8720:
    mask = LP8720_TIMESTEP_M;
    shift = LP8720_TIMESTEP_S;
    time_usec = &lp8720_time_usec[0];
    size = ARRAY_SIZE(lp8720_time_usec);
    break;
    case LP8725:
    mask = LP8725_TIMESTEP_M;
    shift = LP8725_TIMESTEP_S;
    time_usec = &lp8725_time_usec[0];
    size = ARRAY_SIZE(lp8725_time_usec);
    break;
    default:
    return -EINVAL;
    }
    ret = lp872x_read_byte(lp, LP872X_GENERAL_CFG, &val);
    if (ret)
    return ret;
    val = (val & mask) >> shift;
    if (val >= size)
    return -EINVAL;
    return *(time_usec + val);
    }
#[no_mangle]
unsafe extern "C" fn lp872x_regulator_enable_time(rdev: *mut regulator_dev) -> c_int {
    static int lp872x_regulator_enable_time(struct regulator_dev *rdev)
    {
    struct lp872x *lp = rdev_get_drvdata(rdev);
    let mut rid: enum lp872x_regulator_id = rdev_get_id(rdev);
    let mut time_step_us: c_int = lp872x_get_timestep_usec(lp);
    int ret;
    u8 addr, val;
    if (time_step_us < 0)
    return time_step_us;
    switch (rid) {
    case LP8720_ID_LDO1 ... LP8720_ID_BUCK:
    addr = LP872X_LDO1_VOUT + rid;
    break;
    case LP8725_ID_LDO1 ... LP8725_ID_BUCK1:
    addr = LP872X_LDO1_VOUT + rid - LP8725_ID_BASE;
    break;
    case LP8725_ID_BUCK2:
    addr = LP8725_BUCK2_VOUT1;
    break;
    default:
    return -EINVAL;
    }
    ret = lp872x_read_byte(lp, addr, &val);
    if (ret)
    return ret;
    val = (val & LP872X_START_DELAY_M) >> LP872X_START_DELAY_S;
    return val > MAX_DELAY ? 0 : val * time_step_us;
    }
    static void lp872x_set_dvs(struct lp872x *lp, enum lp872x_dvs_sel dvs_sel,
    struct gpio_desc *gpio)
    {
    enum gpiod_flags state;
    state = dvs_sel == SEL_V1 ? GPIOD_OUT_HIGH : GPIOD_OUT_LOW;
    gpiod_set_value(gpio, state);
    lp.dvs_pin = state;
    }
    static u8 lp872x_select_buck_vout_addr(struct lp872x *lp,
    enum lp872x_regulator_id buck)
    {
    u8 val, addr;
    if (lp872x_read_byte(lp, LP872X_GENERAL_CFG, &val))
    return 0;
    switch (buck) {
    case LP8720_ID_BUCK:
    if (val & LP8720_EXT_DVS_M) {
    addr = (lp.dvs_pin == GPIOD_OUT_HIGH) ?
    LP8720_BUCK_VOUT1 : LP8720_BUCK_VOUT2;
    } else {
    if (lp872x_read_byte(lp, LP8720_ENABLE, &val))
    return 0;
    addr = val & LP8720_DVS_SEL_M ?
    LP8720_BUCK_VOUT1 : LP8720_BUCK_VOUT2;
    }
    break;
    case LP8725_ID_BUCK1:
    if (val & LP8725_DVS1_M)
    addr = LP8725_BUCK1_VOUT1;
    else
    addr = (lp.dvs_pin == GPIOD_OUT_HIGH) ?
    LP8725_BUCK1_VOUT1 : LP8725_BUCK1_VOUT2;
    break;
    case LP8725_ID_BUCK2:
    addr =  val & LP8725_DVS2_M ?
    LP8725_BUCK2_VOUT1 : LP8725_BUCK2_VOUT2;
    break;
    default:
    return 0;
    }
    return addr;
    }
#[no_mangle]
unsafe extern "C" fn lp872x_is_valid_buck_addr(addr: u8) -> bool {
    static bool lp872x_is_valid_buck_addr(u8 addr)
    {
    switch (addr) {
    case LP8720_BUCK_VOUT1:
    case LP8720_BUCK_VOUT2:
    case LP8725_BUCK1_VOUT1:
    case LP8725_BUCK1_VOUT2:
    case LP8725_BUCK2_VOUT1:
    case LP8725_BUCK2_VOUT2:
    return true;
    default:
    return false;
    }
    }
    static int lp872x_buck_set_voltage_sel(struct regulator_dev *rdev,
    unsigned selector)
    {
    struct lp872x *lp = rdev_get_drvdata(rdev);
    let mut buck: enum lp872x_regulator_id = rdev_get_id(rdev);
    u8 addr, mask = LP872X_VOUT_M;
    struct lp872x_dvs *dvs = lp.pdata ? lp.pdata.dvs : core::ptr::null_mut();
    if (dvs && dvs.gpio)
    lp872x_set_dvs(lp, dvs.vsel, dvs.gpio);
    addr = lp872x_select_buck_vout_addr(lp, buck);
    if (!lp872x_is_valid_buck_addr(addr))
    return -EINVAL;
    return lp872x_update_bits(lp, addr, mask, selector);
    }
#[no_mangle]
unsafe extern "C" fn lp872x_buck_get_voltage_sel(rdev: *mut regulator_dev) -> c_int {
    static int lp872x_buck_get_voltage_sel(struct regulator_dev *rdev)
    {
    struct lp872x *lp = rdev_get_drvdata(rdev);
    let mut buck: enum lp872x_regulator_id = rdev_get_id(rdev);
    u8 addr, val;
    int ret;
    addr = lp872x_select_buck_vout_addr(lp, buck);
    if (!lp872x_is_valid_buck_addr(addr))
    return -EINVAL;
    ret = lp872x_read_byte(lp, addr, &val);
    if (ret)
    return ret;
    return val & LP872X_VOUT_M;
    }
#[no_mangle]
unsafe extern "C" fn lp872x_buck_set_mode(rdev: *mut regulator_dev, mode: c_uint) -> c_int {
    static int lp872x_buck_set_mode(struct regulator_dev *rdev, unsigned int mode)
    {
    struct lp872x *lp = rdev_get_drvdata(rdev);
    let mut buck: enum lp872x_regulator_id = rdev_get_id(rdev);
    u8 addr, mask, shift, val;
    switch (buck) {
    case LP8720_ID_BUCK:
    addr = LP8720_BUCK_VOUT2;
    mask = LP8720_BUCK_FPWM_M;
    shift = LP8720_BUCK_FPWM_S;
    break;
    case LP8725_ID_BUCK1:
    addr = LP8725_BUCK_CTRL;
    mask = LP8725_BUCK1_FPWM_M;
    shift = LP8725_BUCK1_FPWM_S;
    break;
    case LP8725_ID_BUCK2:
    addr = LP8725_BUCK_CTRL;
    mask = LP8725_BUCK2_FPWM_M;
    shift = LP8725_BUCK2_FPWM_S;
    break;
    default:
    return -EINVAL;
    }
    if (mode == REGULATOR_MODE_FAST)
    val = LP872X_FORCE_PWM << shift;
#[no_mangle]
pub unsafe extern "C" fn if(REGULATOR_MODE_NORMAL: mode ==) -> else {
    else if (mode == REGULATOR_MODE_NORMAL)
    val = LP872X_AUTO_PWM << shift;
    else
    return -EINVAL;
    return lp872x_update_bits(lp, addr, mask, val);
    }
#[no_mangle]
unsafe extern "C" fn lp872x_buck_get_mode(rdev: *mut regulator_dev) -> c_uint {
    static unsigned int lp872x_buck_get_mode(struct regulator_dev *rdev)
    {
    struct lp872x *lp = rdev_get_drvdata(rdev);
    let mut buck: enum lp872x_regulator_id = rdev_get_id(rdev);
    u8 addr, mask, val;
    int ret;
    switch (buck) {
    case LP8720_ID_BUCK:
    addr = LP8720_BUCK_VOUT2;
    mask = LP8720_BUCK_FPWM_M;
    break;
    case LP8725_ID_BUCK1:
    addr = LP8725_BUCK_CTRL;
    mask = LP8725_BUCK1_FPWM_M;
    break;
    case LP8725_ID_BUCK2:
    addr = LP8725_BUCK_CTRL;
    mask = LP8725_BUCK2_FPWM_M;
    break;
    default:
    return -EINVAL;
    }
    ret = lp872x_read_byte(lp, addr, &val);
    if (ret)
    return ret;
    return val & mask ? REGULATOR_MODE_FAST : REGULATOR_MODE_NORMAL;
    }
    static const struct regulator_ops lp872x_ldo_ops = {
    .list_voltage = regulator_list_voltage_table,
    .map_voltage = regulator_map_voltage_ascend,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .enable_time = lp872x_regulator_enable_time,
    };
    static const struct regulator_ops lp8720_buck_ops = {
    .list_voltage = regulator_list_voltage_table,
    .map_voltage = regulator_map_voltage_ascend,
    .set_voltage_sel = lp872x_buck_set_voltage_sel,
    .get_voltage_sel = lp872x_buck_get_voltage_sel,
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .enable_time = lp872x_regulator_enable_time,
    .set_mode = lp872x_buck_set_mode,
    .get_mode = lp872x_buck_get_mode,
    };
    static const struct regulator_ops lp8725_buck_ops = {
    .list_voltage = regulator_list_voltage_table,
    .map_voltage = regulator_map_voltage_ascend,
    .set_voltage_sel = lp872x_buck_set_voltage_sel,
    .get_voltage_sel = lp872x_buck_get_voltage_sel,
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .enable_time = lp872x_regulator_enable_time,
    .set_mode = lp872x_buck_set_mode,
    .get_mode = lp872x_buck_get_mode,
    .set_current_limit = regulator_set_current_limit_regmap,
    .get_current_limit = regulator_get_current_limit_regmap,
    };
    static const struct regulator_desc lp8720_regulator_desc[] = {
    {
    .name = "ldo1",
    .of_match = of_match_ptr("ldo1"),
    .id = LP8720_ID_LDO1,
    .ops = &lp872x_ldo_ops,
    .n_voltages = ARRAY_SIZE(lp872x_ldo_vtbl),
    .volt_table = lp872x_ldo_vtbl,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .vsel_reg = LP872X_LDO1_VOUT,
    .vsel_mask = LP872X_VOUT_M,
    .enable_reg = LP8720_ENABLE,
    .enable_mask = LP872X_EN_LDO1_M,
    },
    {
    .name = "ldo2",
    .of_match = of_match_ptr("ldo2"),
    .id = LP8720_ID_LDO2,
    .ops = &lp872x_ldo_ops,
    .n_voltages = ARRAY_SIZE(lp872x_ldo_vtbl),
    .volt_table = lp872x_ldo_vtbl,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .vsel_reg = LP872X_LDO2_VOUT,
    .vsel_mask = LP872X_VOUT_M,
    .enable_reg = LP8720_ENABLE,
    .enable_mask = LP872X_EN_LDO2_M,
    },
    {
    .name = "ldo3",
    .of_match = of_match_ptr("ldo3"),
    .id = LP8720_ID_LDO3,
    .ops = &lp872x_ldo_ops,
    .n_voltages = ARRAY_SIZE(lp872x_ldo_vtbl),
    .volt_table = lp872x_ldo_vtbl,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .vsel_reg = LP872X_LDO3_VOUT,
    .vsel_mask = LP872X_VOUT_M,
    .enable_reg = LP8720_ENABLE,
    .enable_mask = LP872X_EN_LDO3_M,
    },
    {
    .name = "ldo4",
    .of_match = of_match_ptr("ldo4"),
    .id = LP8720_ID_LDO4,
    .ops = &lp872x_ldo_ops,
    .n_voltages = ARRAY_SIZE(lp8720_ldo4_vtbl),
    .volt_table = lp8720_ldo4_vtbl,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .vsel_reg = LP872X_LDO4_VOUT,
    .vsel_mask = LP872X_VOUT_M,
    .enable_reg = LP8720_ENABLE,
    .enable_mask = LP872X_EN_LDO4_M,
    },
    {
    .name = "ldo5",
    .of_match = of_match_ptr("ldo5"),
    .id = LP8720_ID_LDO5,
    .ops = &lp872x_ldo_ops,
    .n_voltages = ARRAY_SIZE(lp872x_ldo_vtbl),
    .volt_table = lp872x_ldo_vtbl,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .vsel_reg = LP872X_LDO5_VOUT,
    .vsel_mask = LP872X_VOUT_M,
    .enable_reg = LP8720_ENABLE,
    .enable_mask = LP872X_EN_LDO5_M,
    },
    {
    .name = "buck",
    .of_match = of_match_ptr("buck"),
    .id = LP8720_ID_BUCK,
    .ops = &lp8720_buck_ops,
    .n_voltages = ARRAY_SIZE(lp8720_buck_vtbl),
    .volt_table = lp8720_buck_vtbl,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .enable_reg = LP8720_ENABLE,
    .enable_mask = LP8720_EN_BUCK_M,
    },
    };
    static const struct regulator_desc lp8725_regulator_desc[] = {
    {
    .name = "ldo1",
    .of_match = of_match_ptr("ldo1"),
    .id = LP8725_ID_LDO1,
    .ops = &lp872x_ldo_ops,
    .n_voltages = ARRAY_SIZE(lp872x_ldo_vtbl),
    .volt_table = lp872x_ldo_vtbl,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .vsel_reg = LP872X_LDO1_VOUT,
    .vsel_mask = LP872X_VOUT_M,
    .enable_reg = LP8725_LDO_CTRL,
    .enable_mask = LP872X_EN_LDO1_M,
    },
    {
    .name = "ldo2",
    .of_match = of_match_ptr("ldo2"),
    .id = LP8725_ID_LDO2,
    .ops = &lp872x_ldo_ops,
    .n_voltages = ARRAY_SIZE(lp872x_ldo_vtbl),
    .volt_table = lp872x_ldo_vtbl,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .vsel_reg = LP872X_LDO2_VOUT,
    .vsel_mask = LP872X_VOUT_M,
    .enable_reg = LP8725_LDO_CTRL,
    .enable_mask = LP872X_EN_LDO2_M,
    },
    {
    .name = "ldo3",
    .of_match = of_match_ptr("ldo3"),
    .id = LP8725_ID_LDO3,
    .ops = &lp872x_ldo_ops,
    .n_voltages = ARRAY_SIZE(lp872x_ldo_vtbl),
    .volt_table = lp872x_ldo_vtbl,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .vsel_reg = LP872X_LDO3_VOUT,
    .vsel_mask = LP872X_VOUT_M,
    .enable_reg = LP8725_LDO_CTRL,
    .enable_mask = LP872X_EN_LDO3_M,
    },
    {
    .name = "ldo4",
    .of_match = of_match_ptr("ldo4"),
    .id = LP8725_ID_LDO4,
    .ops = &lp872x_ldo_ops,
    .n_voltages = ARRAY_SIZE(lp872x_ldo_vtbl),
    .volt_table = lp872x_ldo_vtbl,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .vsel_reg = LP872X_LDO4_VOUT,
    .vsel_mask = LP872X_VOUT_M,
    .enable_reg = LP8725_LDO_CTRL,
    .enable_mask = LP872X_EN_LDO4_M,
    },
    {
    .name = "ldo5",
    .of_match = of_match_ptr("ldo5"),
    .id = LP8725_ID_LDO5,
    .ops = &lp872x_ldo_ops,
    .n_voltages = ARRAY_SIZE(lp872x_ldo_vtbl),
    .volt_table = lp872x_ldo_vtbl,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .vsel_reg = LP872X_LDO5_VOUT,
    .vsel_mask = LP872X_VOUT_M,
    .enable_reg = LP8725_LDO_CTRL,
    .enable_mask = LP872X_EN_LDO5_M,
    },
    {
    .name = "lilo1",
    .of_match = of_match_ptr("lilo1"),
    .id = LP8725_ID_LILO1,
    .ops = &lp872x_ldo_ops,
    .n_voltages = ARRAY_SIZE(lp8725_lilo_vtbl),
    .volt_table = lp8725_lilo_vtbl,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .vsel_reg = LP8725_LILO1_VOUT,
    .vsel_mask = LP872X_VOUT_M,
    .enable_reg = LP8725_LDO_CTRL,
    .enable_mask = LP8725_EN_LILO1_M,
    },
    {
    .name = "lilo2",
    .of_match = of_match_ptr("lilo2"),
    .id = LP8725_ID_LILO2,
    .ops = &lp872x_ldo_ops,
    .n_voltages = ARRAY_SIZE(lp8725_lilo_vtbl),
    .volt_table = lp8725_lilo_vtbl,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .vsel_reg = LP8725_LILO2_VOUT,
    .vsel_mask = LP872X_VOUT_M,
    .enable_reg = LP8725_LDO_CTRL,
    .enable_mask = LP8725_EN_LILO2_M,
    },
    {
    .name = "buck1",
    .of_match = of_match_ptr("buck1"),
    .id = LP8725_ID_BUCK1,
    .ops = &lp8725_buck_ops,
    .n_voltages = ARRAY_SIZE(lp8725_buck_vtbl),
    .volt_table = lp8725_buck_vtbl,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .enable_reg = LP872X_GENERAL_CFG,
    .enable_mask = LP8725_BUCK1_EN_M,
    .curr_table = lp8725_buck_uA,
    .n_current_limits = ARRAY_SIZE(lp8725_buck_uA),
    .csel_reg = LP8725_BUCK1_VOUT2,
    .csel_mask = LP8725_BUCK_CL_M,
    },
    {
    .name = "buck2",
    .of_match = of_match_ptr("buck2"),
    .id = LP8725_ID_BUCK2,
    .ops = &lp8725_buck_ops,
    .n_voltages = ARRAY_SIZE(lp8725_buck_vtbl),
    .volt_table = lp8725_buck_vtbl,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .enable_reg = LP872X_GENERAL_CFG,
    .enable_mask = LP8725_BUCK2_EN_M,
    .curr_table = lp8725_buck_uA,
    .n_current_limits = ARRAY_SIZE(lp8725_buck_uA),
    .csel_reg = LP8725_BUCK2_VOUT2,
    .csel_mask = LP8725_BUCK_CL_M,
    },
    };
#[no_mangle]
unsafe extern "C" fn lp872x_init_dvs(lp: *mut lp872x) -> c_int {
    static int lp872x_init_dvs(struct lp872x *lp)
    {
    struct lp872x_dvs *dvs = lp.pdata ? lp.pdata.dvs : core::ptr::null_mut();
    enum gpiod_flags pinstate;
    u8 mask[] = { LP8720_EXT_DVS_M, LP8725_DVS1_M | LP8725_DVS2_M };
    u8 default_dvs_mode[] = { LP8720_DEFAULT_DVS, LP8725_DEFAULT_DVS };
    if (!dvs)
    goto set_default_dvs_mode;
    if (!dvs.gpio)
    goto set_default_dvs_mode;
    pinstate = dvs.init_state;
    dvs.gpio = devm_gpiod_get_optional(lp.dev, "ti,dvs", pinstate);
    if (IS_ERR(dvs.gpio)) {
    dev_err(lp.dev, "gpio request err: %ld\n", PTR_ERR(dvs.gpio));
    return PTR_ERR(dvs.gpio);
    }
    lp.dvs_pin = pinstate;
    return 0;
    set_default_dvs_mode:
    return lp872x_update_bits(lp, LP872X_GENERAL_CFG, mask[lp.chipid],
    default_dvs_mode[lp.chipid]);
    }
#[no_mangle]
unsafe extern "C" fn lp872x_hw_enable(lp: *mut lp872x) -> c_int {
    static int lp872x_hw_enable(struct lp872x *lp)
    {
    if (!lp.pdata)
    return -EINVAL;
    if (!lp.pdata.enable_gpio)
    return 0;
// Always set enable GPIO high.
    lp.pdata.enable_gpio = devm_gpiod_get_optional(lp.dev, "enable", GPIOD_OUT_HIGH);
    if (IS_ERR(lp.pdata.enable_gpio)) {
    dev_err(lp.dev, "gpio request err: %ld\n", PTR_ERR(lp.pdata.enable_gpio));
    return PTR_ERR(lp.pdata.enable_gpio);
    }
// Each chip has a different enable delay.
    if (lp.chipid == LP8720)
    usleep_range(LP8720_ENABLE_DELAY, 1.5 * LP8720_ENABLE_DELAY);
    else
    usleep_range(LP8725_ENABLE_DELAY, 1.5 * LP8725_ENABLE_DELAY);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lp872x_config(lp: *mut lp872x) -> c_int {
    static int lp872x_config(struct lp872x *lp)
    {
    struct lp872x_platform_data *pdata = lp.pdata;
    int ret;
    if (!pdata || !pdata.update_config)
    goto init_dvs;
    ret = lp872x_write_byte(lp, LP872X_GENERAL_CFG, pdata.general_config);
    if (ret)
    return ret;
    init_dvs:
    return lp872x_init_dvs(lp);
    }
    static struct regulator_init_data
// lp872x_find_regulator_init_data(int id, struct lp872x *lp)
    {
    struct lp872x_platform_data *pdata = lp.pdata;
    int i;
    if (!pdata)
    return core::ptr::null_mut();
    for (i = 0; i < lp.num_regulators; i++) {
    if (pdata.regulator_data[i].id == id)
    return pdata.regulator_data[i].init_data;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn lp872x_regulator_register(lp: *mut lp872x) -> c_int {
    static int lp872x_regulator_register(struct lp872x *lp)
    {
    const struct regulator_desc *desc;
    let mut cfg: regulator_config = { };
    struct regulator_dev *rdev;
    int i;
    for (i = 0; i < lp.num_regulators; i++) {
    desc = (lp.chipid == LP8720) ? &lp8720_regulator_desc[i] :
    &lp8725_regulator_desc[i];
    cfg.dev = lp.dev;
    cfg.init_data = lp872x_find_regulator_init_data(desc.id, lp);
    cfg.driver_data = lp;
    cfg.regmap = lp.regmap;
    rdev = devm_regulator_register(lp.dev, desc, &cfg);
    if (IS_ERR(rdev)) {
    dev_err(lp.dev, "regulator register err");
    return PTR_ERR(rdev);
    }
    }
    return 0;
    }
    static const struct regmap_config lp872x_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = MAX_REGISTERS,
    };

    static struct of_regulator_match lp8720_matches[] = {
    { .name = "ldo1", .driver_data = (void *)LP8720_ID_LDO1, },
    { .name = "ldo2", .driver_data = (void *)LP8720_ID_LDO2, },
    { .name = "ldo3", .driver_data = (void *)LP8720_ID_LDO3, },
    { .name = "ldo4", .driver_data = (void *)LP8720_ID_LDO4, },
    { .name = "ldo5", .driver_data = (void *)LP8720_ID_LDO5, },
    { .name = "buck", .driver_data = (void *)LP8720_ID_BUCK, },
    };
    static struct of_regulator_match lp8725_matches[] = {
    { .name = "ldo1", .driver_data = (void *)LP8725_ID_LDO1, },
    { .name = "ldo2", .driver_data = (void *)LP8725_ID_LDO2, },
    { .name = "ldo3", .driver_data = (void *)LP8725_ID_LDO3, },
    { .name = "ldo4", .driver_data = (void *)LP8725_ID_LDO4, },
    { .name = "ldo5", .driver_data = (void *)LP8725_ID_LDO5, },
    { .name = "lilo1", .driver_data = (void *)LP8725_ID_LILO1, },
    { .name = "lilo2", .driver_data = (void *)LP8725_ID_LILO2, },
    { .name = "buck1", .driver_data = (void *)LP8725_ID_BUCK1, },
    { .name = "buck2", .driver_data = (void *)LP8725_ID_BUCK2, },
    };
    static struct lp872x_platform_data
// lp872x_populate_pdata_from_dt(struct device *dev, enum lp872x_id which)
    {
    struct device_node *np = dev.of_node;
    struct lp872x_platform_data *pdata;
    struct of_regulator_match *match;
    int num_matches;
    int count;
    int i;
    u8 dvs_state;
    pdata = devm_kzalloc(dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return ERR_PTR(-ENOMEM);
    of_property_read_u8(np, "ti,general-config", &pdata.general_config);
    pdata.update_config = of_property_read_bool(np, "ti,update-config");
    pdata.dvs = devm_kzalloc(dev, sizeof(struct lp872x_dvs), GFP_KERNEL);
    if (!pdata.dvs)
    return ERR_PTR(-ENOMEM);
    of_property_read_u8(np, "ti,dvs-vsel", (u8 *)&pdata.dvs.vsel);
    of_property_read_u8(np, "ti,dvs-state", &dvs_state);
    pdata.dvs.init_state = dvs_state ? GPIOD_OUT_HIGH : GPIOD_OUT_LOW;
    if (of_get_child_count(np) == 0)
    goto out;
    switch (which) {
    case LP8720:
    match = lp8720_matches;
    num_matches = ARRAY_SIZE(lp8720_matches);
    break;
    case LP8725:
    match = lp8725_matches;
    num_matches = ARRAY_SIZE(lp8725_matches);
    break;
    default:
    goto out;
    }
    count = of_regulator_match(dev, np, match, num_matches);
    if (count <= 0)
    goto out;
    for (i = 0; i < num_matches; i++) {
    pdata.regulator_data[i].id =
    (uintptr_t)match[i].driver_data;
    pdata.regulator_data[i].init_data = match[i].init_data;
    }
    out:
    return pdata;
    }

    static struct lp872x_platform_data
// lp872x_populate_pdata_from_dt(struct device *dev, enum lp872x_id which)
    {
    return core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn lp872x_probe(cl: *mut i2c_client) -> c_int {
    static int lp872x_probe(struct i2c_client *cl)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(cl);
    struct lp872x *lp;
    struct lp872x_platform_data *pdata;
    int ret;
    static const int lp872x_num_regulators[] = {
    [LP8720] = LP8720_NUM_REGULATORS,
    [LP8725] = LP8725_NUM_REGULATORS,
    };
    if (cl.dev.of_node) {
    pdata = lp872x_populate_pdata_from_dt(&cl.dev,
    (enum lp872x_id)id.driver_data);
    if (IS_ERR(pdata))
    return PTR_ERR(pdata);
    } else {
    pdata = dev_get_platdata(&cl.dev);
    }
    lp = devm_kzalloc(&cl.dev, sizeof(struct lp872x), GFP_KERNEL);
    if (!lp)
    return -ENOMEM;
    lp.num_regulators = lp872x_num_regulators[id.driver_data];
    lp.regmap = devm_regmap_init_i2c(cl, &lp872x_regmap_config);
    if (IS_ERR(lp.regmap)) {
    ret = PTR_ERR(lp.regmap);
    dev_err(&cl.dev, "regmap init i2c err: %d\n", ret);
    return ret;
    }
    lp.dev = &cl.dev;
    lp.pdata = pdata;
    lp.chipid = id.driver_data;
    i2c_set_clientdata(cl, lp);
    ret = lp872x_hw_enable(lp);
    if (ret)
    return ret;
    ret = lp872x_config(lp);
    if (ret)
    return ret;
    return lp872x_regulator_register(lp);
    }
    static const struct of_device_id lp872x_dt_ids[] __maybe_unused = {
    { .compatible = "ti,lp8720", },
    { .compatible = "ti,lp8725", },
    { }
    };
    MODULE_DEVICE_TABLE(of, lp872x_dt_ids);
    static const struct i2c_device_id lp872x_ids[] = {
    { .name = "lp8720", .driver_data = LP8720 },
    { .name = "lp8725", .driver_data = LP8725 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lp872x_ids);
    static struct i2c_driver lp872x_driver = {
    .driver = {
    .name = "lp872x",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = of_match_ptr(lp872x_dt_ids),
    },
    .probe = lp872x_probe,
    .id_table = lp872x_ids,
    };
    module_i2c_driver(lp872x_driver);
    MODULE_DESCRIPTION("TI/National Semiconductor LP872x PMU Regulator Driver");
    MODULE_AUTHOR("Milo Kim");
    MODULE_LICENSE("GPL");
