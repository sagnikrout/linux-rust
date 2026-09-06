//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/ltc3589.c
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
// Linear Technology LTC3589,LTC3589-1 regulator support
//
// Copyright (c) 2014 Philipp Zabel <p.zabel@pengutronix.de>, Pengutronix

pub const LTC3589_IRQSTAT: c_uint = 0x02;
pub const LTC3589_SCR1: c_uint = 0x07;
pub const LTC3589_OVEN: c_uint = 0x10;
pub const LTC3589_SCR2: c_uint = 0x12;
pub const LTC3589_PGSTAT: c_uint = 0x13;
pub const LTC3589_VCCR: c_uint = 0x20;
pub const LTC3589_CLIRQ: c_uint = 0x21;
pub const LTC3589_B1DTV1: c_uint = 0x23;
pub const LTC3589_B1DTV2: c_uint = 0x24;
pub const LTC3589_VRRCR: c_uint = 0x25;
pub const LTC3589_B2DTV1: c_uint = 0x26;
pub const LTC3589_B2DTV2: c_uint = 0x27;
pub const LTC3589_B3DTV1: c_uint = 0x29;
pub const LTC3589_B3DTV2: c_uint = 0x2a;
pub const LTC3589_L2DTV1: c_uint = 0x32;
pub const LTC3589_L2DTV2: c_uint = 0x33;

    enum ltc3589_reg {
    LTC3589_SW1,
    LTC3589_SW2,
    LTC3589_SW3,
    LTC3589_BB_OUT,
    LTC3589_LDO1,
    LTC3589_LDO2,
    LTC3589_LDO3,
    LTC3589_LDO4,
    LTC3589_NUM_REGULATORS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc3589_info {
    pub volt_table: *const c_uint,
    pub fixed_uV: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc3589 {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub regulator_descs: [regulator_desc; LTC3589_NUM_REGULATORS],
    pub regulators: [*mut regulator_dev; LTC3589_NUM_REGULATORS],
}

    static const int ltc3589_ldo4[] = {
    2800000, 2500000, 1800000, 3300000,
    };
    static const int ltc3589_12_ldo4[] = {
    1200000, 1800000, 2500000, 3200000,
    };
    static const unsigned int ltc3589_ramp_table[] = {
    880, 1750, 3500, 7000
    };
#[no_mangle]
unsafe extern "C" fn ltc3589_set_suspend_voltage(rdev: *mut regulator_dev, uV: c_int) -> c_int {
    static int ltc3589_set_suspend_voltage(struct regulator_dev *rdev, int uV)
    {
    struct ltc3589 *ltc3589 = rdev_get_drvdata(rdev);
    int sel;
    sel = regulator_map_voltage_linear(rdev, uV, uV);
    if (sel < 0)
    return sel;
// DTV2 register follows right after the corresponding DTV1 register
    return regmap_update_bits(ltc3589.regmap, rdev.desc.vsel_reg + 1,
    rdev.desc.vsel_mask, sel);
    }
    static int ltc3589_set_suspend_mode(struct regulator_dev *rdev,
    unsigned int mode)
    {
    struct ltc3589 *ltc3589 = rdev_get_drvdata(rdev);
    int mask, bit = 0;
// VCCR reference selects are right next to the VCCR go bits
    mask = rdev.desc.apply_bit << 1;
    if (mode == REGULATOR_MODE_STANDBY)
    bit = mask;	/* Select DTV2 */
    mask |= rdev.desc.apply_bit;
    bit |= rdev.desc.apply_bit;
    return regmap_update_bits(ltc3589.regmap, LTC3589_VCCR, mask, bit);
    }
// SW1, SW2, SW3, LDO2
    static const struct regulator_ops ltc3589_linear_regulator_ops = {
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .list_voltage = regulator_list_voltage_linear,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_ramp_delay = regulator_set_ramp_delay_regmap,
    .set_voltage_time_sel = regulator_set_voltage_time_sel,
    .set_suspend_voltage = ltc3589_set_suspend_voltage,
    .set_suspend_mode = ltc3589_set_suspend_mode,
    };
// BB_OUT, LDO3
    static const struct regulator_ops ltc3589_fixed_regulator_ops = {
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    };
// LDO1
    static const struct regulator_ops ltc3589_fixed_standby_regulator_ops = {
    };
// LDO4
    static const struct regulator_ops ltc3589_table_regulator_ops = {
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .list_voltage = regulator_list_voltage_table,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    };
#[no_mangle]
pub unsafe extern "C" fn ltc3589_scale(uV: c_uint, r1: u32, r2: u32) -> c_uint {
    static inline unsigned int ltc3589_scale(unsigned int uV, u32 r1, u32 r2)
    {
    uint64_t tmp;
    if (uV == 0)
    return 0;
    tmp = (uint64_t)uV * r1;
    do_div(tmp, r2);
    return uV + (unsigned int)tmp;
    }
    static int ltc3589_of_parse_cb(struct device_node *np,
    const struct regulator_desc *desc,
    struct regulator_config *config)
    {
    struct ltc3589 *ltc3589 = config.driver_data;
    struct regulator_desc *rdesc = &ltc3589.regulator_descs[desc.id];
    u32 r[2];
    int ret;
// Parse feedback voltage dividers. LDO3 and LDO4 don't have them
    if (desc.id >= LTC3589_LDO3)
    return 0;
    ret = of_property_read_u32_array(np, "lltc,fb-voltage-divider", r, 2);
    if (ret) {
    dev_err(ltc3589.dev, "Failed to parse voltage divider: %d\n",
    ret);
    return ret;
    }
    if (!r[0] || !r[1])
    return 0;
    rdesc.min_uV = ltc3589_scale(desc.min_uV, r[0], r[1]);
    rdesc.uV_step = ltc3589_scale(desc.uV_step, r[0], r[1]);
    rdesc.fixed_uV = ltc3589_scale(desc.fixed_uV, r[0], r[1]);
    return 0;
    }

    [LTC3589_ ## _name] = {						\
    .name = #_name,						\
    .of_match = of_match_ptr(#_of_name),			\
    .regulators_node = of_match_ptr("regulators"),		\
    .of_parse_cb = ltc3589_of_parse_cb,			\
    .n_voltages = (dtv_mask) + 1,				\
    .fixed_uV = (dtv_mask) ? 0 : 800000,			\
    .ops = &ltc3589_ ## _ops ## _regulator_ops,		\
    .type = REGULATOR_VOLTAGE,				\
    .id = LTC3589_ ## _name,				\
    .owner = THIS_MODULE,					\
    .vsel_reg = (dtv1_reg),					\
    .vsel_mask = (dtv_mask),				\
    .enable_reg = (en_bit) ? LTC3589_OVEN : 0,		\
    .enable_mask = (en_bit),				\
    }

    [LTC3589_ ## _name] = {						\
    .name = #_name,						\
    .of_match = of_match_ptr(#_of_name),			\
    .regulators_node = of_match_ptr("regulators"),		\
    .of_parse_cb = ltc3589_of_parse_cb,			\
    .n_voltages = 32,					\
    .min_uV = 362500,					\
    .uV_step = 12500,					\
    .ramp_delay = 1750,					\
    .ops = &ltc3589_linear_regulator_ops,			\
    .type = REGULATOR_VOLTAGE,				\
    .id = LTC3589_ ## _name,				\
    .owner = THIS_MODULE,					\
    .vsel_reg = LTC3589_ ## _dtv1,				\
    .vsel_mask = 0x1f,					\
    .apply_reg = LTC3589_VCCR,				\
    .apply_bit = LTC3589_VCCR_ ## _name ## _GO,		\
    .enable_reg = LTC3589_OVEN,				\
    .enable_mask = (LTC3589_OVEN_ ## _name),		\
    .ramp_reg = LTC3589_VRRCR,				\
    .ramp_mask = LTC3589_VRRCR_ ## _name ## _RAMP_MASK,	\
    .ramp_delay_table = ltc3589_ramp_table,			\
    .n_ramp_values = ARRAY_SIZE(ltc3589_ramp_table),	\
    }

    LTC3589_REG(_name, _of_name, fixed, LTC3589_OVEN_ ## _name, 0, 0)
    static const struct regulator_desc ltc3589_regulators[] = {
    LTC3589_LINEAR_REG(SW1, sw1, B1DTV1),
    LTC3589_LINEAR_REG(SW2, sw2, B2DTV1),
    LTC3589_LINEAR_REG(SW3, sw3, B3DTV1),
    LTC3589_FIXED_REG(BB_OUT, bb-out),
    LTC3589_REG(LDO1, ldo1, fixed_standby, 0, 0, 0),
    LTC3589_LINEAR_REG(LDO2, ldo2, L2DTV1),
    LTC3589_FIXED_REG(LDO3, ldo3),
    LTC3589_REG(LDO4, ldo4, table, LTC3589_OVEN_LDO4, LTC3589_L2DTV2, 0x60),
    };
#[no_mangle]
unsafe extern "C" fn ltc3589_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool ltc3589_writeable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case LTC3589_IRQSTAT:
    case LTC3589_SCR1:
    case LTC3589_OVEN:
    case LTC3589_SCR2:
    case LTC3589_VCCR:
    case LTC3589_CLIRQ:
    case LTC3589_B1DTV1:
    case LTC3589_B1DTV2:
    case LTC3589_VRRCR:
    case LTC3589_B2DTV1:
    case LTC3589_B2DTV2:
    case LTC3589_B3DTV1:
    case LTC3589_B3DTV2:
    case LTC3589_L2DTV1:
    case LTC3589_L2DTV2:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn ltc3589_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool ltc3589_readable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case LTC3589_IRQSTAT:
    case LTC3589_SCR1:
    case LTC3589_OVEN:
    case LTC3589_SCR2:
    case LTC3589_PGSTAT:
    case LTC3589_VCCR:
    case LTC3589_B1DTV1:
    case LTC3589_B1DTV2:
    case LTC3589_VRRCR:
    case LTC3589_B2DTV1:
    case LTC3589_B2DTV2:
    case LTC3589_B3DTV1:
    case LTC3589_B3DTV2:
    case LTC3589_L2DTV1:
    case LTC3589_L2DTV2:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn ltc3589_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool ltc3589_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case LTC3589_IRQSTAT:
    case LTC3589_PGSTAT:
    case LTC3589_VCCR:
    return true;
    }
    return false;
    }
    static const struct reg_default ltc3589_reg_defaults[] = {
    { LTC3589_SCR1,   0x00 },
    { LTC3589_OVEN,   0x00 },
    { LTC3589_SCR2,   0x00 },
    { LTC3589_VCCR,   0x00 },
    { LTC3589_B1DTV1, 0x19 },
    { LTC3589_B1DTV2, 0x19 },
    { LTC3589_VRRCR,  0xff },
    { LTC3589_B2DTV1, 0x19 },
    { LTC3589_B2DTV2, 0x19 },
    { LTC3589_B3DTV1, 0x19 },
    { LTC3589_B3DTV2, 0x19 },
    { LTC3589_L2DTV1, 0x19 },
    { LTC3589_L2DTV2, 0x19 },
    };
    static const struct regmap_config ltc3589_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .writeable_reg = ltc3589_writeable_reg,
    .readable_reg = ltc3589_readable_reg,
    .volatile_reg = ltc3589_volatile_reg,
    .max_register = LTC3589_L2DTV2,
    .reg_defaults = ltc3589_reg_defaults,
    .num_reg_defaults = ARRAY_SIZE(ltc3589_reg_defaults),
    .use_single_read = true,
    .use_single_write = true,
    .cache_type = REGCACHE_MAPLE,
    };
#[no_mangle]
unsafe extern "C" fn ltc3589_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ltc3589_isr(int irq, void *dev_id)
    {
    struct ltc3589 *ltc3589 = dev_id;
    unsigned int i, irqstat, event;
    regmap_read(ltc3589.regmap, LTC3589_IRQSTAT, &irqstat);
    if (irqstat & LTC3589_IRQSTAT_THERMAL_WARN) {
    event = REGULATOR_EVENT_OVER_TEMP;
    for (i = 0; i < LTC3589_NUM_REGULATORS; i++)
    regulator_notifier_call_chain(ltc3589.regulators[i],
    event, core::ptr::null_mut());
    }
    if (irqstat & LTC3589_IRQSTAT_UNDERVOLT_WARN) {
    event = REGULATOR_EVENT_UNDER_VOLTAGE;
    for (i = 0; i < LTC3589_NUM_REGULATORS; i++)
    regulator_notifier_call_chain(ltc3589.regulators[i],
    event, core::ptr::null_mut());
    }
// Clear warning condition
    regmap_write(ltc3589.regmap, LTC3589_CLIRQ, 0);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ltc3589_probe(client: *mut i2c_client) -> c_int {
    static int ltc3589_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    const struct ltc3589_info *info;
    struct regulator_desc *descs;
    struct ltc3589 *ltc3589;
    int i, ret;
    ltc3589 = devm_kzalloc(dev, sizeof(*ltc3589), GFP_KERNEL);
    if (!ltc3589)
    return -ENOMEM;
    i2c_set_clientdata(client, ltc3589);
    info = i2c_get_match_data(client);
    ltc3589.dev = dev;
    descs = ltc3589.regulator_descs;
    memcpy(descs, ltc3589_regulators, sizeof(ltc3589_regulators));
    descs[LTC3589_LDO3].fixed_uV = info.fixed_uV;
    descs[LTC3589_LDO4].volt_table = info.volt_table;
    ltc3589.regmap = devm_regmap_init_i2c(client, &ltc3589_regmap_config);
    if (IS_ERR(ltc3589.regmap)) {
    ret = PTR_ERR(ltc3589.regmap);
    dev_err(dev, "failed to initialize regmap: %d\n", ret);
    return ret;
    }
    for (i = 0; i < LTC3589_NUM_REGULATORS; i++) {
    struct regulator_desc *desc = &ltc3589.regulator_descs[i];
    let mut config: regulator_config = { };
    config.dev = dev;
    config.driver_data = ltc3589;
    ltc3589.regulators[i] = devm_regulator_register(dev, desc,
    &config);
    if (IS_ERR(ltc3589.regulators[i])) {
    ret = PTR_ERR(ltc3589.regulators[i]);
    dev_err(dev, "failed to register regulator %s: %d\n",
    desc.name, ret);
    return ret;
    }
    }
    if (client.irq) {
    ret = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(),
    ltc3589_isr,
    IRQF_TRIGGER_LOW | IRQF_ONESHOT,
    client.name, ltc3589);
    if (ret) {
    dev_err(dev, "Failed to request IRQ: %d\n", ret);
    return ret;
    }
    }
    return 0;
    }
    static const struct ltc3589_info ltc3589_info = {
    .fixed_uV = 1800000,
    .volt_table = ltc3589_ldo4,
    };
    static const struct ltc3589_info ltc3589_12_info = {
    .fixed_uV = 2800000,
    .volt_table = ltc3589_12_ldo4,
    };
    static const struct i2c_device_id ltc3589_i2c_id[] = {
    { .name = "ltc3589", .driver_data = (kernel_ulong_t)&ltc3589_info },
    { .name = "ltc3589-1", .driver_data = (kernel_ulong_t)&ltc3589_12_info },
    { .name = "ltc3589-2", .driver_data = (kernel_ulong_t)&ltc3589_12_info },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ltc3589_i2c_id);
    static const struct of_device_id __maybe_unused ltc3589_of_match[] = {
    { .compatible = "lltc,ltc3589",   .data = &ltc3589_info },
    { .compatible = "lltc,ltc3589-1", .data = &ltc3589_12_info },
    { .compatible = "lltc,ltc3589-2", .data = &ltc3589_12_info },
    { }
    };
    MODULE_DEVICE_TABLE(of, ltc3589_of_match);
    static struct i2c_driver ltc3589_driver = {
    .driver = {
    .name = DRIVER_NAME,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = of_match_ptr(ltc3589_of_match),
    },
    .probe = ltc3589_probe,
    .id_table = ltc3589_i2c_id,
    };
    module_i2c_driver(ltc3589_driver);
    MODULE_AUTHOR("Philipp Zabel <p.zabel@pengutronix.de>");
    MODULE_DESCRIPTION("Regulator driver for Linear Technology LTC3589(-1,2)");
    MODULE_LICENSE("GPL v2");
