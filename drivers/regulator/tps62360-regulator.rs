//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/tps62360-regulator.c
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
// tps62360.c -- TI tps62360
//
// Driver for processor core supply tps62360, tps62361B, tps62362 and tps62363.
//
// Copyright (c) 2012, NVIDIA Corporation.
//
// Author: Laxman Dewangan <ldewangan@nvidia.com>
//

// Register definitions
pub const REG_VSET0: c_int = 0;
pub const REG_VSET1: c_int = 1;
pub const REG_VSET2: c_int = 2;
pub const REG_VSET3: c_int = 3;
pub const REG_CONTROL: c_int = 4;
pub const REG_TEMP: c_int = 5;
pub const REG_RAMPCTRL: c_int = 6;
pub const REG_CHIPID: c_int = 8;

    enum chips {TPS62360, TPS62361, TPS62362, TPS62363};
pub const TPS62360_BASE_VOLTAGE: c_int = 770000;
pub const TPS62360_N_VOLTAGES: c_int = 64;
pub const TPS62361_BASE_VOLTAGE: c_int = 500000;
pub const TPS62361_N_VOLTAGES: c_int = 128;
// tps 62360 chip information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps62360_chip {
    pub dev: *mut device,
    pub desc: regulator_desc,
    pub rdev: *mut regulator_dev,
    pub regmap: *mut regmap,
    pub vsel0_gpio: *mut gpio_desc,
    pub vsel1_gpio: *mut gpio_desc,
    pub voltage_reg_mask: u8,
    pub en_internal_pulldn: bool,
    pub en_discharge: bool,
    pub valid_gpios: bool,
    pub lru_index: [c_int; 4],
    pub curr_vset_vsel: [c_int; 4],
    pub curr_vset_id: c_int,
}

//
// find_voltage_set_register: Find new voltage configuration register
// (VSET) id.
// The finding of the new VSET register will be based on the LRU mechanism.
// Each VSET register will have different voltage configured . This
// Function will look if any of the VSET register have requested voltage set
// or not.
// - If it is already there then it will make that register as most
// recently used and return as found so that caller need not to set
// the VSET register but need to set the proper gpios to select this
// VSET register.
// - If requested voltage is not found then it will use the least
// recently mechanism to get new VSET register for new configuration
// and will return not_found so that caller need to set new VSET
// register and then gpios (both).
//
    static bool find_voltage_set_register(struct tps62360_chip *tps,
    int req_vsel, int *vset_reg_id)
    {
    int i;
    let mut found: bool = false;
    let mut new_vset_reg: c_int = tps.lru_index[3];
    let mut found_index: c_int = 3;
    for (i = 0; i < 4; ++i) {
    if (tps.curr_vset_vsel[tps.lru_index[i]] == req_vsel) {
    new_vset_reg = tps.lru_index[i];
    found_index = i;
    found = true;
    goto update_lru_index;
    }
    }
    update_lru_index:
    for (i = found_index; i > 0; i--)
    tps.lru_index[i] = tps.lru_index[i - 1];
    tps.lru_index[0] = new_vset_reg;
// vset_reg_id = new_vset_reg;
    return found;
    }
#[no_mangle]
unsafe extern "C" fn tps62360_dcdc_get_voltage_sel(dev: *mut regulator_dev) -> c_int {
    static int tps62360_dcdc_get_voltage_sel(struct regulator_dev *dev)
    {
    struct tps62360_chip *tps = rdev_get_drvdata(dev);
    int vsel;
    unsigned int data;
    int ret;
    ret = regmap_read(tps.regmap, REG_VSET0 + tps.curr_vset_id, &data);
    if (ret < 0) {
    dev_err(tps.dev, "%s(): register %d read failed with err %d\n",
    __func__, REG_VSET0 + tps.curr_vset_id, ret);
    return ret;
    }
    vsel = (int)data & tps.voltage_reg_mask;
    return vsel;
    }
    static int tps62360_dcdc_set_voltage_sel(struct regulator_dev *dev,
    unsigned selector)
    {
    struct tps62360_chip *tps = rdev_get_drvdata(dev);
    int ret;
    let mut found: bool = false;
    let mut new_vset_id: c_int = tps.curr_vset_id;
//
// If gpios are available to select the VSET register then least
// recently used register for new configuration.
//
    if (tps.valid_gpios)
    found = find_voltage_set_register(tps, selector, &new_vset_id);
    if (!found) {
    ret = regmap_update_bits(tps.regmap, REG_VSET0 + new_vset_id,
    tps.voltage_reg_mask, selector);
    if (ret < 0) {
    dev_err(tps.dev,
    "%s(): register %d update failed with err %d\n",
    __func__, REG_VSET0 + new_vset_id, ret);
    return ret;
    }
    tps.curr_vset_id = new_vset_id;
    tps.curr_vset_vsel[new_vset_id] = selector;
    }
// Select proper VSET register vio gpios
    if (tps.valid_gpios) {
    gpiod_set_value_cansleep(tps.vsel0_gpio, new_vset_id & 0x1);
    gpiod_set_value_cansleep(tps.vsel1_gpio,
    (new_vset_id >> 1) & 0x1);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tps62360_set_mode(rdev: *mut regulator_dev, mode: c_uint) -> c_int {
    static int tps62360_set_mode(struct regulator_dev *rdev, unsigned int mode)
    {
    struct tps62360_chip *tps = rdev_get_drvdata(rdev);
    int i;
    int val;
    int ret;
// Enable force PWM mode in FAST mode only.
    switch (mode) {
    case REGULATOR_MODE_FAST:
    val = FORCE_PWM_ENABLE;
    break;
    case REGULATOR_MODE_NORMAL:
    val = 0;
    break;
    default:
    return -EINVAL;
    }
    if (!tps.valid_gpios) {
    ret = regmap_update_bits(tps.regmap,
    REG_VSET0 + tps.curr_vset_id, FORCE_PWM_ENABLE, val);
    if (ret < 0)
    dev_err(tps.dev,
    "%s(): register %d update failed with err %d\n",
    __func__, REG_VSET0 + tps.curr_vset_id, ret);
    return ret;
    }
// If gpios are valid then all register set need to be control
    for (i = 0; i < 4; ++i) {
    ret = regmap_update_bits(tps.regmap,
    REG_VSET0 + i, FORCE_PWM_ENABLE, val);
    if (ret < 0) {
    dev_err(tps.dev,
    "%s(): register %d update failed with err %d\n",
    __func__, REG_VSET0 + i, ret);
    return ret;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tps62360_get_mode(rdev: *mut regulator_dev) -> c_uint {
    static unsigned int tps62360_get_mode(struct regulator_dev *rdev)
    {
    struct tps62360_chip *tps = rdev_get_drvdata(rdev);
    unsigned int data;
    int ret;
    ret = regmap_read(tps.regmap, REG_VSET0 + tps.curr_vset_id, &data);
    if (ret < 0) {
    dev_err(tps.dev, "%s(): register %d read failed with err %d\n",
    __func__, REG_VSET0 + tps.curr_vset_id, ret);
    return ret;
    }
    return (data & FORCE_PWM_ENABLE) ?
    REGULATOR_MODE_FAST : REGULATOR_MODE_NORMAL;
    }
    static const struct regulator_ops tps62360_dcdc_ops = {
    .get_voltage_sel	= tps62360_dcdc_get_voltage_sel,
    .set_voltage_sel	= tps62360_dcdc_set_voltage_sel,
    .list_voltage		= regulator_list_voltage_linear,
    .map_voltage		= regulator_map_voltage_linear,
    .set_voltage_time_sel	= regulator_set_voltage_time_sel,
    .set_mode		= tps62360_set_mode,
    .get_mode		= tps62360_get_mode,
    };
    static int tps62360_init_dcdc(struct tps62360_chip *tps,
    struct tps62360_regulator_platform_data *pdata)
    {
    int ret;
    unsigned int ramp_ctrl;
// Initialize internal pull up/down control
    if (tps.en_internal_pulldn)
    ret = regmap_write(tps.regmap, REG_CONTROL, 0xE0);
    else
    ret = regmap_write(tps.regmap, REG_CONTROL, 0x0);
    if (ret < 0) {
    dev_err(tps.dev,
    "%s(): register %d write failed with err %d\n",
    __func__, REG_CONTROL, ret);
    return ret;
    }
// Reset output discharge path to reduce power consumption
    ret = regmap_update_bits(tps.regmap, REG_RAMPCTRL, BIT(2), 0);
    if (ret < 0) {
    dev_err(tps.dev,
    "%s(): register %d update failed with err %d\n",
    __func__, REG_RAMPCTRL, ret);
    return ret;
    }
// Get ramp value from ramp control register
    ret = regmap_read(tps.regmap, REG_RAMPCTRL, &ramp_ctrl);
    if (ret < 0) {
    dev_err(tps.dev,
    "%s(): register %d read failed with err %d\n",
    __func__, REG_RAMPCTRL, ret);
    return ret;
    }
    ramp_ctrl = (ramp_ctrl >> 5) & 0x7;
// ramp mV/us = 32/(2^ramp_ctrl)
    tps.desc.ramp_delay = DIV_ROUND_UP(32000, BIT(ramp_ctrl));
    return ret;
    }
    static const struct regmap_config tps62360_regmap_config = {
    .reg_bits		= 8,
    .val_bits		= 8,
    .max_register		= REG_CHIPID,
    .cache_type		= REGCACHE_MAPLE,
    };
    static struct tps62360_regulator_platform_data *
    of_get_tps62360_platform_data(struct device *dev,
    const struct regulator_desc *desc)
    {
    struct tps62360_regulator_platform_data *pdata;
    struct device_node *np = dev.of_node;
    pdata = devm_kzalloc(dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return core::ptr::null_mut();
    pdata.reg_init_data = of_get_regulator_init_data(dev, dev.of_node,
    desc);
    if (!pdata.reg_init_data) {
    dev_err(dev, "Not able to get OF regulator init data\n");
    return core::ptr::null_mut();
    }
    pdata.vsel0_def_state = of_property_read_bool(np, "ti,vsel0-state-high");
    pdata.vsel1_def_state = of_property_read_bool(np, "ti,vsel1-state-high");
    pdata.en_internal_pulldn = of_property_read_bool(np, "ti,enable-pull-down");
    pdata.en_discharge = of_property_read_bool(np, "ti,enable-vout-discharge");
    return pdata;
    }

    static const struct of_device_id tps62360_of_match[] = {
    { .compatible = "ti,tps62360", .data = (void *)TPS62360},
    { .compatible = "ti,tps62361", .data = (void *)TPS62361},
    { .compatible = "ti,tps62362", .data = (void *)TPS62362},
    { .compatible = "ti,tps62363", .data = (void *)TPS62363},
    {},
    };
    MODULE_DEVICE_TABLE(of, tps62360_of_match);

#[no_mangle]
unsafe extern "C" fn tps62360_probe(client: *mut i2c_client) -> c_int {
    static int tps62360_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    let mut config: regulator_config = { };
    struct tps62360_regulator_platform_data *pdata;
    struct regulator_dev *rdev;
    struct tps62360_chip *tps;
    int ret;
    int i;
    int chip_id;
    int gpio_flags;
    pdata = dev_get_platdata(&client.dev);
    tps = devm_kzalloc(&client.dev, sizeof(*tps), GFP_KERNEL);
    if (!tps)
    return -ENOMEM;
    tps.desc.name = client.name;
    tps.desc.id = 0;
    tps.desc.ops = &tps62360_dcdc_ops;
    tps.desc.type = REGULATOR_VOLTAGE;
    tps.desc.owner = THIS_MODULE;
    tps.desc.uV_step = 10000;
    if (client.dev.of_node) {
    const struct of_device_id *match;
    match = of_match_device(of_match_ptr(tps62360_of_match),
    &client.dev);
    if (!match) {
    dev_err(&client.dev, "Error: No device match found\n");
    return -ENODEV;
    }
    chip_id = (int)(long)match.data;
    if (!pdata)
    pdata = of_get_tps62360_platform_data(&client.dev,
    &tps.desc);
    } else if (id) {
    chip_id = id.driver_data;
    } else {
    dev_err(&client.dev, "No device tree match or id table match found\n");
    return -ENODEV;
    }
    if (!pdata) {
    dev_err(&client.dev, "%s(): Platform data not found\n",
    __func__);
    return -EIO;
    }
    tps.en_discharge = pdata.en_discharge;
    tps.en_internal_pulldn = pdata.en_internal_pulldn;
    tps.dev = &client.dev;
    switch (chip_id) {
    case TPS62360:
    case TPS62362:
    tps.desc.min_uV = TPS62360_BASE_VOLTAGE;
    tps.voltage_reg_mask = 0x3F;
    tps.desc.n_voltages = TPS62360_N_VOLTAGES;
    break;
    case TPS62361:
    case TPS62363:
    tps.desc.min_uV = TPS62361_BASE_VOLTAGE;
    tps.voltage_reg_mask = 0x7F;
    tps.desc.n_voltages = TPS62361_N_VOLTAGES;
    break;
    default:
    return -ENODEV;
    }
    tps.regmap = devm_regmap_init_i2c(client, &tps62360_regmap_config);
    if (IS_ERR(tps.regmap)) {
    ret = PTR_ERR(tps.regmap);
    dev_err(&client.dev,
    "%s(): regmap allocation failed with err %d\n",
    __func__, ret);
    return ret;
    }
    i2c_set_clientdata(client, tps);
    tps.curr_vset_id = (pdata.vsel1_def_state & 1) * 2 +
    (pdata.vsel0_def_state & 1);
    tps.lru_index[0] = tps.curr_vset_id;
    tps.valid_gpios = false;
    gpio_flags = (pdata.vsel0_def_state) ?
    GPIOD_OUT_HIGH : GPIOD_OUT_LOW;
    tps.vsel0_gpio = devm_gpiod_get_optional(&client.dev, "vsel0", gpio_flags);
    if (IS_ERR(tps.vsel0_gpio)) {
    dev_err(&client.dev,
    "%s(): Could not obtain vsel0 GPIO: %ld\n",
    __func__, PTR_ERR(tps.vsel0_gpio));
    return PTR_ERR(tps.vsel0_gpio);
    }
    gpio_flags = (pdata.vsel1_def_state) ?
    GPIOD_OUT_HIGH : GPIOD_OUT_LOW;
    tps.vsel1_gpio = devm_gpiod_get_optional(&client.dev, "vsel1", gpio_flags);
    if (IS_ERR(tps.vsel1_gpio)) {
    dev_err(&client.dev,
    "%s(): Could not obtain vsel1 GPIO: %ld\n",
    __func__, PTR_ERR(tps.vsel1_gpio));
    return PTR_ERR(tps.vsel1_gpio);
    }
    if (tps.vsel0_gpio != core::ptr::null_mut() && tps.vsel1_gpio != core::ptr::null_mut()) {
    tps.valid_gpios = true;
//
// Initialize the lru index with vset_reg id
// The index 0 will be most recently used and
// set with the tps->curr_vset_id
    for (i = 0; i < 4; ++i)
    tps.lru_index[i] = i;
    tps.lru_index[0] = tps.curr_vset_id;
    tps.lru_index[tps.curr_vset_id] = 0;
    }
    ret = tps62360_init_dcdc(tps, pdata);
    if (ret < 0) {
    dev_err(tps.dev, "%s(): Init failed with err = %d\n",
    __func__, ret);
    return ret;
    }
    config.dev = &client.dev;
    config.init_data = pdata.reg_init_data;
    config.driver_data = tps;
    config.of_node = client.dev.of_node;
// Register the regulators
    rdev = devm_regulator_register(&client.dev, &tps.desc, &config);
    if (IS_ERR(rdev)) {
    dev_err(tps.dev,
    "%s(): regulator register failed with err %s\n",
    __func__, id.name);
    return PTR_ERR(rdev);
    }
    tps.rdev = rdev;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tps62360_shutdown(client: *mut i2c_client) {
    static void tps62360_shutdown(struct i2c_client *client)
    {
    struct tps62360_chip *tps = i2c_get_clientdata(client);
    int st;
    if (!tps.en_discharge)
    return;
// Configure the output discharge path
    st = regmap_update_bits(tps.regmap, REG_RAMPCTRL, BIT(2), BIT(2));
    if (st < 0)
    dev_err(tps.dev,
    "%s(): register %d update failed with err %d\n",
    __func__, REG_RAMPCTRL, st);
    }
    static const struct i2c_device_id tps62360_id[] = {
    { .name = "tps62360", .driver_data = TPS62360 },
    { .name = "tps62361", .driver_data = TPS62361 },
    { .name = "tps62362", .driver_data = TPS62362 },
    { .name = "tps62363", .driver_data = TPS62363 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tps62360_id);
    static struct i2c_driver tps62360_i2c_driver = {
    .driver = {
    .name = "tps62360",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = of_match_ptr(tps62360_of_match),
    },
    .probe = tps62360_probe,
    .shutdown = tps62360_shutdown,
    .id_table = tps62360_id,
    };
#[no_mangle]
unsafe extern "C" fn tps62360_init() -> int __init {
    static int __init tps62360_init(void)
    {
    return i2c_add_driver(&tps62360_i2c_driver);
    }
    subsys_initcall(tps62360_init);
#[no_mangle]
unsafe extern "C" fn tps62360_cleanup() -> void __exit {
    static void __exit tps62360_cleanup(void)
    {
    i2c_del_driver(&tps62360_i2c_driver);
    }
    module_exit(tps62360_cleanup);
    MODULE_AUTHOR("Laxman Dewangan <ldewangan@nvidia.com>");
    MODULE_DESCRIPTION("TPS6236x voltage regulator driver");
    MODULE_LICENSE("GPL v2");
