//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/lp8788-buck.c
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
// TI LP8788 MFD - buck regulator driver
//
// Copyright 2012 Texas Instruments
//
// Author: Milo(Woogyom) Kim <milo.kim@ti.com>
//

// register address
pub const LP8788_EN_BUCK: c_uint = 0x0C;
pub const LP8788_BUCK_DVS_SEL: c_uint = 0x1D;
pub const LP8788_BUCK1_VOUT0: c_uint = 0x1E;
pub const LP8788_BUCK1_VOUT1: c_uint = 0x1F;
pub const LP8788_BUCK1_VOUT2: c_uint = 0x20;
pub const LP8788_BUCK1_VOUT3: c_uint = 0x21;
pub const LP8788_BUCK2_VOUT0: c_uint = 0x22;
pub const LP8788_BUCK2_VOUT1: c_uint = 0x23;
pub const LP8788_BUCK2_VOUT2: c_uint = 0x24;
pub const LP8788_BUCK2_VOUT3: c_uint = 0x25;
pub const LP8788_BUCK3_VOUT: c_uint = 0x26;
pub const LP8788_BUCK4_VOUT: c_uint = 0x27;
pub const LP8788_BUCK1_TIMESTEP: c_uint = 0x28;
pub const LP8788_BUCK_PWM: c_uint = 0x2D;
// mask/shift bits

pub const LP8788_BUCK1_DVS_SEL_M: c_uint = 0x04	/* Addr 1Dh */;
pub const LP8788_BUCK1_DVS_M: c_uint = 0x03;
pub const LP8788_BUCK1_DVS_S: c_int = 0;
pub const LP8788_BUCK2_DVS_SEL_M: c_uint = 0x40;
pub const LP8788_BUCK2_DVS_M: c_uint = 0x30;
pub const LP8788_BUCK2_DVS_S: c_int = 4;

pub const LP8788_VOUT_M: c_uint = 0x1F	/* Addr 1Eh ~ 27h */;
pub const LP8788_STARTUP_TIME_M: c_uint = 0xF8	/* Addr 28h ~ 2Bh */;
pub const LP8788_STARTUP_TIME_S: c_int = 3;

pub const LP8788_FPWM_BUCK1_S: c_int = 0;

pub const LP8788_FPWM_BUCK2_S: c_int = 1;

pub const LP8788_FPWM_BUCK3_S: c_int = 2;

pub const LP8788_FPWM_BUCK4_S: c_int = 3;
pub const INVALID_ADDR: c_uint = 0xFF;
pub const LP8788_FORCE_PWM: c_int = 1;
pub const LP8788_AUTO_PWM: c_int = 0;
pub const PIN_LOW: c_int = 0;
pub const PIN_HIGH: c_int = 1;
pub const ENABLE_TIME_USEC: c_int = 32;

    enum lp8788_dvs_state {
    DVS_LOW  = 0,
    DVS_HIGH = 1,
    };
    enum lp8788_dvs_mode {
    REGISTER,
    EXTPIN,
    };
    enum lp8788_buck_id {
    BUCK1,
    BUCK2,
    BUCK3,
    BUCK4,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp8788_buck {
    pub lp: *mut lp8788,
    pub regulator: *mut regulator_dev,
    pub dvs: *mut c_void,
    pub gpio1: *mut gpio_desc,
    pub /: *mut *mut *mut gpio_desc gpio2; / Only used on BUCK2,
}

// BUCK 1 ~ 4 voltage ranges
    static const struct linear_range buck_volt_ranges[] = {
    REGULATOR_LINEAR_RANGE(500000, 0, 0, 0),
    REGULATOR_LINEAR_RANGE(800000, 1, 25, 50000),
    };
#[no_mangle]
unsafe extern "C" fn lp8788_buck1_set_dvs(buck: *mut lp8788_buck) {
    static void lp8788_buck1_set_dvs(struct lp8788_buck *buck)
    {
    struct lp8788_buck1_dvs *dvs = (struct lp8788_buck1_dvs *)buck.dvs;
    enum lp8788_dvs_state pinstate;
    if (!dvs)
    return;
    pinstate = dvs.vsel == DVS_SEL_V0 ? DVS_LOW : DVS_HIGH;
    gpiod_set_value(buck.gpio1, pinstate);
    }
#[no_mangle]
unsafe extern "C" fn lp8788_buck2_set_dvs(buck: *mut lp8788_buck) {
    static void lp8788_buck2_set_dvs(struct lp8788_buck *buck)
    {
    struct lp8788_buck2_dvs *dvs = (struct lp8788_buck2_dvs *)buck.dvs;
    enum lp8788_dvs_state pin1, pin2;
    if (!dvs)
    return;
    switch (dvs.vsel) {
    case DVS_SEL_V0:
    pin1 = DVS_LOW;
    pin2 = DVS_LOW;
    break;
    case DVS_SEL_V1:
    pin1 = DVS_HIGH;
    pin2 = DVS_LOW;
    break;
    case DVS_SEL_V2:
    pin1 = DVS_LOW;
    pin2 = DVS_HIGH;
    break;
    case DVS_SEL_V3:
    pin1 = DVS_HIGH;
    pin2 = DVS_HIGH;
    break;
    default:
    return;
    }
    gpiod_set_value(buck.gpio1, pin1);
    gpiod_set_value(buck.gpio2, pin2);
    }
#[no_mangle]
unsafe extern "C" fn lp8788_set_dvs(buck: *mut lp8788_buck, id: enum lp8788_buck_id) {
    static void lp8788_set_dvs(struct lp8788_buck *buck, enum lp8788_buck_id id)
    {
    switch (id) {
    case BUCK1:
    lp8788_buck1_set_dvs(buck);
    break;
    case BUCK2:
    lp8788_buck2_set_dvs(buck);
    break;
    default:
    break;
    }
    }
    static enum lp8788_dvs_mode
    lp8788_get_buck_dvs_ctrl_mode(struct lp8788_buck *buck, enum lp8788_buck_id id)
    {
    u8 val, mask;
    switch (id) {
    case BUCK1:
    mask = LP8788_BUCK1_DVS_SEL_M;
    break;
    case BUCK2:
    mask = LP8788_BUCK2_DVS_SEL_M;
    break;
    default:
    return REGISTER;
    }
    lp8788_read_byte(buck.lp, LP8788_BUCK_DVS_SEL, &val);
    return val & mask ? REGISTER : EXTPIN;
    }
#[no_mangle]
unsafe extern "C" fn lp8788_is_valid_buck_addr(addr: u8) -> bool {
    static bool lp8788_is_valid_buck_addr(u8 addr)
    {
    switch (addr) {
    case LP8788_BUCK1_VOUT0:
    case LP8788_BUCK1_VOUT1:
    case LP8788_BUCK1_VOUT2:
    case LP8788_BUCK1_VOUT3:
    case LP8788_BUCK2_VOUT0:
    case LP8788_BUCK2_VOUT1:
    case LP8788_BUCK2_VOUT2:
    case LP8788_BUCK2_VOUT3:
    return true;
    default:
    return false;
    }
    }
    static u8 lp8788_select_buck_vout_addr(struct lp8788_buck *buck,
    enum lp8788_buck_id id)
    {
    let mut mode: enum lp8788_dvs_mode = lp8788_get_buck_dvs_ctrl_mode(buck, id);
    u8 val, idx, addr;
    int pin1, pin2;
    switch (id) {
    case BUCK1:
    if (mode == EXTPIN) {
    idx = gpiod_get_value(buck.gpio1);
    } else {
    lp8788_read_byte(buck.lp, LP8788_BUCK_DVS_SEL, &val);
    idx = (val & LP8788_BUCK1_DVS_M) >> LP8788_BUCK1_DVS_S;
    }
    addr = LP8788_BUCK1_VOUT0 + idx;
    break;
    case BUCK2:
    if (mode == EXTPIN) {
    pin1 = gpiod_get_value(buck.gpio1);
    pin2 = gpiod_get_value(buck.gpio2);
    if (pin1 == PIN_LOW && pin2 == PIN_LOW)
    idx = 0;
#[no_mangle]
pub unsafe extern "C" fn if(PIN_HIGH: pin1 == PIN_LOW && pin2 ==) -> else {
    else if (pin1 == PIN_LOW && pin2 == PIN_HIGH)
    idx = 2;
#[no_mangle]
pub unsafe extern "C" fn if(PIN_LOW: pin1 == PIN_HIGH && pin2 ==) -> else {
    else if (pin1 == PIN_HIGH && pin2 == PIN_LOW)
    idx = 1;
    else
    idx = 3;
    } else {
    lp8788_read_byte(buck.lp, LP8788_BUCK_DVS_SEL, &val);
    idx = (val & LP8788_BUCK2_DVS_M) >> LP8788_BUCK2_DVS_S;
    }
    addr = LP8788_BUCK2_VOUT0 + idx;
    break;
    default:
    goto err;
    }
    return addr;
    err:
    return INVALID_ADDR;
    }
    static int lp8788_buck12_set_voltage_sel(struct regulator_dev *rdev,
    unsigned selector)
    {
    struct lp8788_buck *buck = rdev_get_drvdata(rdev);
    let mut id: enum lp8788_buck_id = rdev_get_id(rdev);
    u8 addr;
    if (buck.dvs)
    lp8788_set_dvs(buck, id);
    addr = lp8788_select_buck_vout_addr(buck, id);
    if (!lp8788_is_valid_buck_addr(addr))
    return -EINVAL;
    return lp8788_update_bits(buck.lp, addr, LP8788_VOUT_M, selector);
    }
#[no_mangle]
unsafe extern "C" fn lp8788_buck12_get_voltage_sel(rdev: *mut regulator_dev) -> c_int {
    static int lp8788_buck12_get_voltage_sel(struct regulator_dev *rdev)
    {
    struct lp8788_buck *buck = rdev_get_drvdata(rdev);
    let mut id: enum lp8788_buck_id = rdev_get_id(rdev);
    int ret;
    u8 val, addr;
    addr = lp8788_select_buck_vout_addr(buck, id);
    if (!lp8788_is_valid_buck_addr(addr))
    return -EINVAL;
    ret = lp8788_read_byte(buck.lp, addr, &val);
    if (ret)
    return ret;
    return val & LP8788_VOUT_M;
    }
#[no_mangle]
unsafe extern "C" fn lp8788_buck_enable_time(rdev: *mut regulator_dev) -> c_int {
    static int lp8788_buck_enable_time(struct regulator_dev *rdev)
    {
    struct lp8788_buck *buck = rdev_get_drvdata(rdev);
    let mut id: enum lp8788_buck_id = rdev_get_id(rdev);
    u8 val, addr = LP8788_BUCK1_TIMESTEP + id;
    if (lp8788_read_byte(buck.lp, addr, &val))
    return -EINVAL;
    val = (val & LP8788_STARTUP_TIME_M) >> LP8788_STARTUP_TIME_S;
    return ENABLE_TIME_USEC * val;
    }
#[no_mangle]
unsafe extern "C" fn lp8788_buck_set_mode(rdev: *mut regulator_dev, mode: c_uint) -> c_int {
    static int lp8788_buck_set_mode(struct regulator_dev *rdev, unsigned int mode)
    {
    struct lp8788_buck *buck = rdev_get_drvdata(rdev);
    let mut id: enum lp8788_buck_id = rdev_get_id(rdev);
    u8 mask, val;
    mask = BUCK_FPWM_MASK(id);
    switch (mode) {
    case REGULATOR_MODE_FAST:
    val = LP8788_FORCE_PWM << BUCK_FPWM_SHIFT(id);
    break;
    case REGULATOR_MODE_NORMAL:
    val = LP8788_AUTO_PWM << BUCK_FPWM_SHIFT(id);
    break;
    default:
    return -EINVAL;
    }
    return lp8788_update_bits(buck.lp, LP8788_BUCK_PWM, mask, val);
    }
#[no_mangle]
unsafe extern "C" fn lp8788_buck_get_mode(rdev: *mut regulator_dev) -> c_uint {
    static unsigned int lp8788_buck_get_mode(struct regulator_dev *rdev)
    {
    struct lp8788_buck *buck = rdev_get_drvdata(rdev);
    let mut id: enum lp8788_buck_id = rdev_get_id(rdev);
    u8 val;
    int ret;
    ret = lp8788_read_byte(buck.lp, LP8788_BUCK_PWM, &val);
    if (ret)
    return ret;
    return val & BUCK_FPWM_MASK(id) ?
    REGULATOR_MODE_FAST : REGULATOR_MODE_NORMAL;
    }
    static const struct regulator_ops lp8788_buck12_ops = {
    .list_voltage = regulator_list_voltage_linear_range,
    .map_voltage = regulator_map_voltage_linear_range,
    .set_voltage_sel = lp8788_buck12_set_voltage_sel,
    .get_voltage_sel = lp8788_buck12_get_voltage_sel,
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .enable_time = lp8788_buck_enable_time,
    .set_mode = lp8788_buck_set_mode,
    .get_mode = lp8788_buck_get_mode,
    };
    static const struct regulator_ops lp8788_buck34_ops = {
    .list_voltage = regulator_list_voltage_linear_range,
    .map_voltage = regulator_map_voltage_linear_range,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .enable_time = lp8788_buck_enable_time,
    .set_mode = lp8788_buck_set_mode,
    .get_mode = lp8788_buck_get_mode,
    };
    static const struct regulator_desc lp8788_buck_desc[] = {
    {
    .name = "buck1",
    .id = BUCK1,
    .ops = &lp8788_buck12_ops,
    .n_voltages = 26,
    .linear_ranges = buck_volt_ranges,
    .n_linear_ranges = ARRAY_SIZE(buck_volt_ranges),
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .enable_reg = LP8788_EN_BUCK,
    .enable_mask = LP8788_EN_BUCK1_M,
    },
    {
    .name = "buck2",
    .id = BUCK2,
    .ops = &lp8788_buck12_ops,
    .n_voltages = 26,
    .linear_ranges = buck_volt_ranges,
    .n_linear_ranges = ARRAY_SIZE(buck_volt_ranges),
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .enable_reg = LP8788_EN_BUCK,
    .enable_mask = LP8788_EN_BUCK2_M,
    },
    {
    .name = "buck3",
    .id = BUCK3,
    .ops = &lp8788_buck34_ops,
    .n_voltages = 26,
    .linear_ranges = buck_volt_ranges,
    .n_linear_ranges = ARRAY_SIZE(buck_volt_ranges),
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .vsel_reg = LP8788_BUCK3_VOUT,
    .vsel_mask = LP8788_VOUT_M,
    .enable_reg = LP8788_EN_BUCK,
    .enable_mask = LP8788_EN_BUCK3_M,
    },
    {
    .name = "buck4",
    .id = BUCK4,
    .ops = &lp8788_buck34_ops,
    .n_voltages = 26,
    .linear_ranges = buck_volt_ranges,
    .n_linear_ranges = ARRAY_SIZE(buck_volt_ranges),
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .vsel_reg = LP8788_BUCK4_VOUT,
    .vsel_mask = LP8788_VOUT_M,
    .enable_reg = LP8788_EN_BUCK,
    .enable_mask = LP8788_EN_BUCK4_M,
    },
    };
    static int lp8788_dvs_gpio_request(struct platform_device *pdev,
    struct lp8788_buck *buck,
    enum lp8788_buck_id id)
    {
    struct lp8788_platform_data *pdata = buck.lp.pdata;
    struct device *dev = &pdev.dev;
    switch (id) {
    case BUCK1:
    buck.gpio1 = devm_gpiod_get(dev, "dvs", GPIOD_OUT_LOW);
    if (IS_ERR(buck.gpio1))
    return PTR_ERR(buck.gpio1);
    gpiod_set_consumer_name(buck.gpio1, "LP8788_B1_DVS");
    buck.dvs = pdata.buck1_dvs;
    break;
    case BUCK2:
    buck.gpio1 = devm_gpiod_get_index(dev, "dvs", 0, GPIOD_OUT_LOW);
    if (IS_ERR(buck.gpio1))
    return PTR_ERR(buck.gpio1);
    gpiod_set_consumer_name(buck.gpio1, "LP8788_B2_DVS1");
    buck.gpio2 = devm_gpiod_get_index(dev, "dvs", 1, GPIOD_OUT_LOW);
    if (IS_ERR(buck.gpio2))
    return PTR_ERR(buck.gpio2);
    gpiod_set_consumer_name(buck.gpio2, "LP8788_B2_DVS2");
    buck.dvs = pdata.buck2_dvs;
    break;
    default:
    break;
    }
    return 0;
    }
    static int lp8788_init_dvs(struct platform_device *pdev,
    struct lp8788_buck *buck, enum lp8788_buck_id id)
    {
    struct lp8788_platform_data *pdata = buck.lp.pdata;
    u8 mask[] = { LP8788_BUCK1_DVS_SEL_M, LP8788_BUCK2_DVS_SEL_M };
    u8 val[]  = { LP8788_BUCK1_DVS_PIN, LP8788_BUCK2_DVS_PIN };
    u8 default_dvs_mode[] = { LP8788_BUCK1_DVS_I2C, LP8788_BUCK2_DVS_I2C };
// no dvs for buck3, 4
    if (id > BUCK2)
    return 0;
// no dvs platform data, then dvs will be selected by I2C registers
    if (!pdata)
    goto set_default_dvs_mode;
    if ((id == BUCK1 && !pdata.buck1_dvs) ||
    (id == BUCK2 && !pdata.buck2_dvs))
    goto set_default_dvs_mode;
    if (lp8788_dvs_gpio_request(pdev, buck, id))
    goto set_default_dvs_mode;
    return lp8788_update_bits(buck.lp, LP8788_BUCK_DVS_SEL, mask[id],
    val[id]);
    set_default_dvs_mode:
    return lp8788_update_bits(buck.lp, LP8788_BUCK_DVS_SEL, mask[id],
    default_dvs_mode[id]);
    }
#[no_mangle]
unsafe extern "C" fn lp8788_buck_probe(pdev: *mut platform_device) -> c_int {
    static int lp8788_buck_probe(struct platform_device *pdev)
    {
    struct lp8788 *lp = dev_get_drvdata(pdev.dev.parent);
    let mut id: c_int = pdev.id;
    struct lp8788_buck *buck;
    let mut cfg: regulator_config = { };
    struct regulator_dev *rdev;
    int ret;
    if (id >= LP8788_NUM_BUCKS)
    return -EINVAL;
    buck = devm_kzalloc(&pdev.dev, sizeof(struct lp8788_buck), GFP_KERNEL);
    if (!buck)
    return -ENOMEM;
    buck.lp = lp;
    ret = lp8788_init_dvs(pdev, buck, id);
    if (ret)
    return ret;
    cfg.dev = pdev.dev.parent;
    cfg.init_data = lp.pdata ? lp.pdata.buck_data[id] : core::ptr::null_mut();
    cfg.driver_data = buck;
    cfg.regmap = lp.regmap;
    rdev = devm_regulator_register(&pdev.dev, &lp8788_buck_desc[id], &cfg);
    if (IS_ERR(rdev)) {
    ret = PTR_ERR(rdev);
    dev_err(&pdev.dev, "BUCK%d regulator register err = %d\n",
    id + 1, ret);
    return ret;
    }
    buck.regulator = rdev;
    platform_set_drvdata(pdev, buck);
    return 0;
    }
    static struct platform_driver lp8788_buck_driver = {
    .probe = lp8788_buck_probe,
    .driver = {
    .name = LP8788_DEV_BUCK,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
#[no_mangle]
unsafe extern "C" fn lp8788_buck_init() -> int __init {
    static int __init lp8788_buck_init(void)
    {
    return platform_driver_register(&lp8788_buck_driver);
    }
    subsys_initcall(lp8788_buck_init);
#[no_mangle]
unsafe extern "C" fn lp8788_buck_exit() -> void __exit {
    static void __exit lp8788_buck_exit(void)
    {
    platform_driver_unregister(&lp8788_buck_driver);
    }
    module_exit(lp8788_buck_exit);
    MODULE_DESCRIPTION("TI LP8788 BUCK Driver");
    MODULE_AUTHOR("Milo Kim");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:lp8788-buck");
