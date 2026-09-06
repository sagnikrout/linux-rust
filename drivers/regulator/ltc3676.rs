//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/ltc3676.c
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
// Copyright (C) 2016 Gateworks Corporation, Inc. All Rights Reserved.
//

// LTC3676 Registers
pub const LTC3676_BUCK1: c_uint = 0x01;
pub const LTC3676_BUCK2: c_uint = 0x02;
pub const LTC3676_BUCK3: c_uint = 0x03;
pub const LTC3676_BUCK4: c_uint = 0x04;
pub const LTC3676_LDOA: c_uint = 0x05;
pub const LTC3676_LDOB: c_uint = 0x06;
pub const LTC3676_SQD1: c_uint = 0x07;
pub const LTC3676_SQD2: c_uint = 0x08;
pub const LTC3676_CNTRL: c_uint = 0x09;
pub const LTC3676_DVB1A: c_uint = 0x0A;
pub const LTC3676_DVB1B: c_uint = 0x0B;
pub const LTC3676_DVB2A: c_uint = 0x0C;
pub const LTC3676_DVB2B: c_uint = 0x0D;
pub const LTC3676_DVB3A: c_uint = 0x0E;
pub const LTC3676_DVB3B: c_uint = 0x0F;
pub const LTC3676_DVB4A: c_uint = 0x10;
pub const LTC3676_DVB4B: c_uint = 0x11;
pub const LTC3676_MSKIRQ: c_uint = 0x12;
pub const LTC3676_MSKPG: c_uint = 0x13;
pub const LTC3676_USER: c_uint = 0x14;
pub const LTC3676_IRQSTAT: c_uint = 0x15;
pub const LTC3676_PGSTATL: c_uint = 0x16;
pub const LTC3676_PGSTATRT: c_uint = 0x17;
pub const LTC3676_HRST: c_uint = 0x1E;
pub const LTC3676_CLIRQ: c_uint = 0x1F;

    enum ltc3676_reg {
    LTC3676_SW1,
    LTC3676_SW2,
    LTC3676_SW3,
    LTC3676_SW4,
    LTC3676_LDO1,
    LTC3676_LDO2,
    LTC3676_LDO3,
    LTC3676_LDO4,
    LTC3676_NUM_REGULATORS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc3676 {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub regulator_descs: [regulator_desc; LTC3676_NUM_REGULATORS],
    pub regulators: [*mut regulator_dev; LTC3676_NUM_REGULATORS],
}

#[no_mangle]
unsafe extern "C" fn ltc3676_set_suspend_voltage(rdev: *mut regulator_dev, uV: c_int) -> c_int {
    static int ltc3676_set_suspend_voltage(struct regulator_dev *rdev, int uV)
    {
    struct ltc3676 *ltc3676 = rdev_get_drvdata(rdev);
    struct device *dev = ltc3676.dev;
    let mut dcdc: c_int = rdev_get_id(rdev);
    int sel;
    dev_dbg(dev, "%s id=%d uV=%d\n", __func__, dcdc, uV);
    sel = regulator_map_voltage_linear(rdev, uV, uV);
    if (sel < 0)
    return sel;
// DVBB register follows right after the corresponding DVBA register
    return regmap_update_bits(ltc3676.regmap, rdev.desc.vsel_reg + 1,
    rdev.desc.vsel_mask, sel);
    }
    static int ltc3676_set_suspend_mode(struct regulator_dev *rdev,
    unsigned int mode)
    {
    struct ltc3676 *ltc3676= rdev_get_drvdata(rdev);
    struct device *dev = ltc3676.dev;
    int mask, val;
    let mut dcdc: c_int = rdev_get_id(rdev);
    dev_dbg(dev, "%s id=%d mode=%d\n", __func__, dcdc, mode);
    mask = LTC3676_DVBxA_REF_SELECT;
    switch (mode) {
    case REGULATOR_MODE_STANDBY:
    val = 0; /* select DVBxA */
    break;
    case REGULATOR_MODE_NORMAL:
    val = LTC3676_DVBxA_REF_SELECT; /* select DVBxB */
    break;
    default:
    dev_warn(&rdev.dev, "%s: regulator mode: 0x%x not supported\n",
    rdev.desc.name, mode);
    return -EINVAL;
    }
    return regmap_update_bits(ltc3676.regmap, rdev.desc.vsel_reg,
    mask, val);
    }
#[no_mangle]
unsafe extern "C" fn ltc3676_set_voltage_sel(rdev: *mut regulator_dev, selector: unsigned) -> c_int {
    static int ltc3676_set_voltage_sel(struct regulator_dev *rdev, unsigned selector)
    {
    struct ltc3676 *ltc3676 = rdev_get_drvdata(rdev);
    struct device *dev = ltc3676.dev;
    int ret, dcdc = rdev_get_id(rdev);
    dev_dbg(dev, "%s id=%d selector=%d\n", __func__, dcdc, selector);
    ret = regmap_update_bits(ltc3676.regmap, rdev.desc.vsel_reg + 1,
    LTC3676_DVBxB_PGOOD_MASK,
    LTC3676_DVBxB_PGOOD_MASK);
    if (ret)
    return ret;
    return regulator_set_voltage_sel_regmap(rdev, selector);
    }
#[no_mangle]
pub unsafe extern "C" fn ltc3676_scale(uV: c_uint, r1: u32, r2: u32) -> c_uint {
    static inline unsigned int ltc3676_scale(unsigned int uV, u32 r1, u32 r2)
    {
    uint64_t tmp;
    if (uV == 0)
    return 0;
    tmp = (uint64_t)uV * r1;
    do_div(tmp, r2);
    return uV + (unsigned int)tmp;
    }
    static int ltc3676_of_parse_cb(struct device_node *np,
    const struct regulator_desc *desc,
    struct regulator_config *config)
    {
    struct ltc3676 *ltc3676 = config.driver_data;
    struct regulator_desc *rdesc = &ltc3676.regulator_descs[desc.id];
    u32 r[2];
    int ret;
// LDO3 has a fixed output
    if (desc.id == LTC3676_LDO3)
    return 0;
    ret = of_property_read_u32_array(np, "lltc,fb-voltage-divider", r, 2);
    if (ret) {
    dev_err(ltc3676.dev, "Failed to parse voltage divider: %d\n",
    ret);
    return ret;
    }
    rdesc.min_uV = ltc3676_scale(desc.min_uV, r[0], r[1]);
    rdesc.uV_step = ltc3676_scale(desc.uV_step, r[0], r[1]);
    rdesc.fixed_uV = ltc3676_scale(desc.fixed_uV, r[0], r[1]);
    return 0;
    }
// SW1, SW2, SW3, SW4 linear 0.8V-3.3V with scalar via R1/R2 feeback res
    static const struct regulator_ops ltc3676_linear_regulator_ops = {
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .list_voltage = regulator_list_voltage_linear,
    .set_voltage_sel = ltc3676_set_voltage_sel,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_suspend_voltage = ltc3676_set_suspend_voltage,
    .set_suspend_mode = ltc3676_set_suspend_mode,
    };
// LDO1 always on fixed 0.8V-3.3V via scalar via R1/R2 feeback res
    static const struct regulator_ops ltc3676_fixed_standby_regulator_ops = {
    };
// LDO2, LDO3 fixed (LDO2 has external scalar via R1/R2 feedback res)
    static const struct regulator_ops ltc3676_fixed_regulator_ops = {
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    };

    [LTC3676_ ## _id] = {                                        \
    .name = #_name,                                \
    .of_match = of_match_ptr(#_name),              \
    .regulators_node = of_match_ptr("regulators"), \
    .of_parse_cb = ltc3676_of_parse_cb,            \
    .n_voltages = (dvb_mask) + 1,                  \
    .min_uV = (dvba_reg) ? 412500 : 0,             \
    .uV_step = (dvba_reg) ? 12500 : 0,             \
    .ramp_delay = (dvba_reg) ? 800 : 0,            \
    .fixed_uV = (dvb_mask) ? 0 : 725000,           \
    .ops = &ltc3676_ ## _ops ## _regulator_ops,    \
    .type = REGULATOR_VOLTAGE,                     \
    .id = LTC3676_ ## _id,                         \
    .owner = THIS_MODULE,                          \
    .vsel_reg = (dvba_reg),                        \
    .vsel_mask = (dvb_mask),                       \
    .enable_reg = (en_reg),                        \
    .enable_mask = (1 << en_bit),                  \
    }

    LTC3676_REG(_id, _name, linear,                                \
    LTC3676_ ## _en, 7,                                \
    LTC3676_ ## _dvba, 0x1f)

    LTC3676_REG(_id, _name, fixed, LTC3676_ ## _en_reg, _en_bit, 0, 0)
    static const struct regulator_desc ltc3676_regulators[LTC3676_NUM_REGULATORS] = {
    LTC3676_LINEAR_REG(SW1, sw1, BUCK1, DVB1A),
    LTC3676_LINEAR_REG(SW2, sw2, BUCK2, DVB2A),
    LTC3676_LINEAR_REG(SW3, sw3, BUCK3, DVB3A),
    LTC3676_LINEAR_REG(SW4, sw4, BUCK4, DVB4A),
    LTC3676_REG(LDO1, ldo1, fixed_standby, 0, 0, 0, 0),
    LTC3676_FIXED_REG(LDO2, ldo2, LDOA, 2),
    LTC3676_FIXED_REG(LDO3, ldo3, LDOA, 5),
    LTC3676_FIXED_REG(LDO4, ldo4, LDOB, 2),
    };
#[no_mangle]
unsafe extern "C" fn ltc3676_readable_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool ltc3676_readable_writeable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case LTC3676_BUCK1 ... LTC3676_IRQSTAT:
    case LTC3676_HRST:
    case LTC3676_CLIRQ:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn ltc3676_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool ltc3676_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case LTC3676_IRQSTAT ... LTC3676_PGSTATRT:
    return true;
    }
    return false;
    }
    static const struct regmap_config ltc3676_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .writeable_reg = ltc3676_readable_writeable_reg,
    .readable_reg = ltc3676_readable_writeable_reg,
    .volatile_reg = ltc3676_volatile_reg,
    .max_register = LTC3676_CLIRQ,
    .use_single_read = true,
    .use_single_write = true,
    .cache_type = REGCACHE_MAPLE,
    };
#[no_mangle]
unsafe extern "C" fn ltc3676_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ltc3676_isr(int irq, void *dev_id)
    {
    struct ltc3676 *ltc3676 = dev_id;
    struct device *dev = ltc3676.dev;
    unsigned int i, irqstat, event;
    regmap_read(ltc3676.regmap, LTC3676_IRQSTAT, &irqstat);
    dev_dbg(dev, "irq%d irqstat=0x%02x\n", irq, irqstat);
    if (irqstat & LTC3676_IRQSTAT_THERMAL_WARN) {
    dev_warn(dev, "Over-temperature Warning\n");
    event = REGULATOR_EVENT_OVER_TEMP;
    for (i = 0; i < LTC3676_NUM_REGULATORS; i++)
    regulator_notifier_call_chain(ltc3676.regulators[i],
    event, core::ptr::null_mut());
    }
    if (irqstat & LTC3676_IRQSTAT_UNDERVOLT_WARN) {
    dev_info(dev, "Undervoltage Warning\n");
    event = REGULATOR_EVENT_UNDER_VOLTAGE;
    for (i = 0; i < LTC3676_NUM_REGULATORS; i++)
    regulator_notifier_call_chain(ltc3676.regulators[i],
    event, core::ptr::null_mut());
    }
// Clear warning condition
    regmap_write(ltc3676.regmap, LTC3676_CLIRQ, 0);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ltc3676_regulator_probe(client: *mut i2c_client) -> c_int {
    static int ltc3676_regulator_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct regulator_init_data *init_data = dev_get_platdata(dev);
    struct regulator_desc *descs;
    struct ltc3676 *ltc3676;
    int i, ret;
    ltc3676 = devm_kzalloc(dev, sizeof(*ltc3676), GFP_KERNEL);
    if (!ltc3676)
    return -ENOMEM;
    i2c_set_clientdata(client, ltc3676);
    ltc3676.dev = dev;
    descs = ltc3676.regulator_descs;
    memcpy(descs, ltc3676_regulators, sizeof(ltc3676_regulators));
    descs[LTC3676_LDO3].fixed_uV = 1800000; /* LDO3 is fixed 1.8V */
    ltc3676.regmap = devm_regmap_init_i2c(client, &ltc3676_regmap_config);
    if (IS_ERR(ltc3676.regmap)) {
    ret = PTR_ERR(ltc3676.regmap);
    dev_err(dev, "failed to initialize regmap: %d\n", ret);
    return ret;
    }
    for (i = 0; i < LTC3676_NUM_REGULATORS; i++) {
    struct regulator_desc *desc = &ltc3676.regulator_descs[i];
    let mut config: regulator_config = { };
    if (init_data)
    config.init_data = &init_data[i];
    config.dev = dev;
    config.driver_data = ltc3676;
    ltc3676.regulators[i] = devm_regulator_register(dev, desc,
    &config);
    if (IS_ERR(ltc3676.regulators[i])) {
    ret = PTR_ERR(ltc3676.regulators[i]);
    dev_err(dev, "failed to register regulator %s: %d\n",
    desc.name, ret);
    return ret;
    }
    }
    regmap_write(ltc3676.regmap, LTC3676_CLIRQ, 0);
    if (client.irq) {
    ret = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(),
    ltc3676_isr,
    IRQF_TRIGGER_LOW | IRQF_ONESHOT,
    client.name, ltc3676);
    if (ret) {
    dev_err(dev, "Failed to request IRQ: %d\n", ret);
    return ret;
    }
    }
    return 0;
    }
    static const struct i2c_device_id ltc3676_i2c_id[] = {
    { .name = "ltc3676" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ltc3676_i2c_id);
    static const struct of_device_id __maybe_unused ltc3676_of_match[] = {
    { .compatible = "lltc,ltc3676" },
    { },
    };
    MODULE_DEVICE_TABLE(of, ltc3676_of_match);
    static struct i2c_driver ltc3676_driver = {
    .driver = {
    .name = DRIVER_NAME,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = of_match_ptr(ltc3676_of_match),
    },
    .probe = ltc3676_regulator_probe,
    .id_table = ltc3676_i2c_id,
    };
    module_i2c_driver(ltc3676_driver);
    MODULE_AUTHOR("Tim Harvey <tharvey@gateworks.com>");
    MODULE_DESCRIPTION("Regulator driver for Linear Technology LTC3676");
    MODULE_LICENSE("GPL v2");
