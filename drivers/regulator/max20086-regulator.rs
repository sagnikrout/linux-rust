//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/max20086-regulator.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// max20086-regulator.c - MAX20086-MAX20089 camera power protector driver
//
// Copyright (C) 2022 Laurent Pinchart <laurent.pinchart@idesonboard.com>
// Copyright (C) 2018 Avnet, Inc.

// Register Offset
pub const MAX20086_REG_MASK: c_uint = 0x00;
pub const MAX20086_REG_CONFIG: c_uint = 0x01;
pub const MAX20086_REG_ID: c_uint = 0x02;
pub const MAX20086_REG_STAT1: c_uint = 0x03;
pub const MAX20086_REG_STAT2_L: c_uint = 0x04;
pub const MAX20086_REG_STAT2_H: c_uint = 0x05;
pub const MAX20086_REG_ADC1: c_uint = 0x06;
pub const MAX20086_REG_ADC2: c_uint = 0x07;
pub const MAX20086_REG_ADC3: c_uint = 0x08;
pub const MAX20086_REG_ADC4: c_uint = 0x09;
// DEVICE IDs
pub const MAX20086_DEVICE_ID_MAX20086: c_uint = 0x30;
pub const MAX20086_DEVICE_ID_MAX20087: c_uint = 0x20;
pub const MAX20086_DEVICE_ID_MAX20088: c_uint = 0x10;
pub const MAX20086_DEVICE_ID_MAX20089: c_uint = 0x00;
pub const DEVICE_ID_MASK: c_uint = 0xf0;
// Register bits
pub const MAX20086_EN_MASK: c_uint = 0x0f;
pub const MAX20086_EN_OUT1: c_uint = 0x01;
pub const MAX20086_EN_OUT2: c_uint = 0x02;
pub const MAX20086_EN_OUT3: c_uint = 0x04;
pub const MAX20086_EN_OUT4: c_uint = 0x08;
pub const MAX20086_INT_DISABLE_ALL: c_uint = 0x3f;
pub const MAX20086_MAX_REGULATORS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max20086_chip_info {
    pub id: u8,
    pub num_outputs: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max20086_regulator {
    pub of_node: *mut device_node,
    pub init_data: *mut regulator_init_data,
    pub desc: *const regulator_desc,
    pub rdev: *mut regulator_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max20086 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub ena_gpiod: *mut gpio_desc,
    pub info: *const max20086_chip_info,
    pub regulators: [max20086_regulator; MAX20086_MAX_REGULATORS],
}

    static const struct regulator_ops max20086_buck_ops = {
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    };

    {						\
    .name = "OUT"#n,			\
    .supply_name = "in",			\
    .id = (n) - 1,				\
    .ops = &max20086_buck_ops,		\
    .type = REGULATOR_VOLTAGE,		\
    .owner = THIS_MODULE,			\
    .enable_reg = MAX20086_REG_CONFIG,	\
    .enable_mask = 1 << ((n) - 1),		\
    .enable_val = 1 << ((n) - 1),		\
    .disable_val = 0,			\
    }
    static const char * const max20086_output_names[] = {
    "OUT1",
    "OUT2",
    "OUT3",
    "OUT4",
    };
    static const struct regulator_desc max20086_regulators[] = {
    MAX20086_REGULATOR_DESC(1),
    MAX20086_REGULATOR_DESC(2),
    MAX20086_REGULATOR_DESC(3),
    MAX20086_REGULATOR_DESC(4),
    };
#[no_mangle]
unsafe extern "C" fn max20086_regulators_register(chip: *mut max20086) -> c_int {
    static int max20086_regulators_register(struct max20086 *chip)
    {
    unsigned int i;
    for (i = 0; i < chip.info.num_outputs; i++) {
    struct max20086_regulator *reg = &chip.regulators[i];
    let mut config: regulator_config = { };
    struct regulator_dev *rdev;
    config.dev = chip.dev;
    config.init_data = reg.init_data;
    config.driver_data = chip;
    config.of_node = reg.of_node;
    config.regmap = chip.regmap;
    config.ena_gpiod = chip.ena_gpiod;
    rdev = devm_regulator_register(chip.dev, reg.desc, &config);
    if (IS_ERR(rdev)) {
    dev_err(chip.dev,
    "Failed to register regulator output %s\n",
    reg.desc.name);
    return PTR_ERR(rdev);
    }
    reg.rdev = rdev;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max20086_parse_regulators_dt(chip: *mut max20086, boot_on: *mut bool) -> c_int {
    static int max20086_parse_regulators_dt(struct max20086 *chip, bool *boot_on)
    {
    struct of_regulator_match *matches;
    unsigned int i;
    int ret;
    struct device_node *node __free(device_node) =
    of_get_child_by_name(chip.dev.of_node, "regulators");
    if (!node) {
    dev_err(chip.dev, "regulators node not found\n");
    return -ENODEV;
    }
    matches = devm_kcalloc(chip.dev, chip.info.num_outputs,
    sizeof(*matches), GFP_KERNEL);
    if (!matches)
    return -ENOMEM;
    for (i = 0; i < chip.info.num_outputs; ++i)
    matches[i].name = max20086_output_names[i];
    ret = of_regulator_match(chip.dev, node, matches,
    chip.info.num_outputs);
    if (ret < 0) {
    dev_err(chip.dev, "Failed to match regulators\n");
    return -EINVAL;
    }
// boot_on = false;
    for (i = 0; i < chip.info.num_outputs; i++) {
    struct max20086_regulator *reg = &chip.regulators[i];
    reg.init_data = matches[i].init_data;
    reg.of_node = matches[i].of_node;
    reg.desc = &max20086_regulators[i];
    if (reg.init_data) {
    if (reg.init_data.constraints.always_on ||
    reg.init_data.constraints.boot_on)
// boot_on = true;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max20086_detect(chip: *mut max20086) -> c_int {
    static int max20086_detect(struct max20086 *chip)
    {
    unsigned int data;
    int ret;
    ret = regmap_read(chip.regmap, MAX20086_REG_ID, &data);
    if (ret < 0) {
    dev_err(chip.dev, "Failed to read DEVICE_ID reg: %d\n", ret);
    return ret;
    }
    if ((data & DEVICE_ID_MASK) != chip.info.id) {
    dev_err(chip.dev, "Invalid device ID 0x%02x\n", data);
    return -ENXIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max20086_gen_is_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool max20086_gen_is_writeable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MAX20086_REG_MASK:
    case MAX20086_REG_CONFIG:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config max20086_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .writeable_reg = max20086_gen_is_writeable_reg,
    .max_register = 0x9,
    .cache_type = REGCACHE_NONE,
    };
#[no_mangle]
unsafe extern "C" fn max20086_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int max20086_i2c_probe(struct i2c_client *i2c)
    {
    struct max20086 *chip;
    enum gpiod_flags flags;
    bool boot_on;
    int ret;
    chip = devm_kzalloc(&i2c.dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.dev = &i2c.dev;
    chip.info = i2c_get_match_data(i2c);
    i2c_set_clientdata(i2c, chip);
    chip.regmap = devm_regmap_init_i2c(i2c, &max20086_regmap_config);
    if (IS_ERR(chip.regmap)) {
    ret = PTR_ERR(chip.regmap);
    dev_err(chip.dev, "Failed to allocate register map: %d\n", ret);
    return ret;
    }
    ret = max20086_parse_regulators_dt(chip, &boot_on);
    if (ret < 0)
    return ret;
    ret = max20086_detect(chip);
    if (ret < 0)
    return ret;
// Until IRQ support is added, just disable all interrupts.
    ret = regmap_update_bits(chip.regmap, MAX20086_REG_MASK,
    MAX20086_INT_DISABLE_ALL,
    MAX20086_INT_DISABLE_ALL);
    if (ret < 0) {
    dev_err(chip.dev, "Failed to disable interrupts: %d\n", ret);
    return ret;
    }
//
// Get the enable GPIO. If any of the outputs is marked as being
// enabled at boot, request the GPIO with an initial high state to
// avoid disabling outputs that may have been turned on by the boot
// loader. Otherwise, request it with a low state to enter lower-power
// shutdown.
//
    flags = boot_on ? GPIOD_OUT_HIGH : GPIOD_OUT_LOW;
    chip.ena_gpiod = devm_gpiod_get_optional(chip.dev, "enable", flags);
    if (IS_ERR(chip.ena_gpiod)) {
    ret = PTR_ERR(chip.ena_gpiod);
    dev_err(chip.dev, "Failed to get enable GPIO: %d\n", ret);
    return ret;
    }
    ret = max20086_regulators_register(chip);
    if (ret < 0) {
    dev_err(chip.dev, "Failed to register regulators: %d\n", ret);
    return ret;
    }
    return 0;
    }
    static const struct max20086_chip_info max20086_chip_info = {
    .id = MAX20086_DEVICE_ID_MAX20086,
    .num_outputs = 4,
    };
    static const struct max20086_chip_info max20087_chip_info = {
    .id = MAX20086_DEVICE_ID_MAX20087,
    .num_outputs = 4,
    };
    static const struct max20086_chip_info max20088_chip_info = {
    .id = MAX20086_DEVICE_ID_MAX20088,
    .num_outputs = 2,
    };
    static const struct max20086_chip_info max20089_chip_info = {
    .id = MAX20086_DEVICE_ID_MAX20089,
    .num_outputs = 2,
    };
    static const struct i2c_device_id max20086_i2c_id[] = {
    { .name = "max20086", .driver_data = (kernel_ulong_t)&max20086_chip_info },
    { .name = "max20087", .driver_data = (kernel_ulong_t)&max20087_chip_info },
    { .name = "max20088", .driver_data = (kernel_ulong_t)&max20088_chip_info },
    { .name = "max20089", .driver_data = (kernel_ulong_t)&max20089_chip_info },
    { /* Sentinel */ }
    };
    MODULE_DEVICE_TABLE(i2c, max20086_i2c_id);
    static const struct of_device_id max20086_dt_ids[] __maybe_unused = {
    { .compatible = "maxim,max20086", .data = &max20086_chip_info },
    { .compatible = "maxim,max20087", .data = &max20087_chip_info },
    { .compatible = "maxim,max20088", .data = &max20088_chip_info },
    { .compatible = "maxim,max20089", .data = &max20089_chip_info },
    { /* Sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, max20086_dt_ids);
    static struct i2c_driver max20086_regulator_driver = {
    .driver = {
    .name = "max20086",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = of_match_ptr(max20086_dt_ids),
    },
    .probe = max20086_i2c_probe,
    .id_table = max20086_i2c_id,
    };
    module_i2c_driver(max20086_regulator_driver);
    MODULE_AUTHOR("Watson Chow <watson.chow@avnet.com>");
    MODULE_DESCRIPTION("MAX20086-MAX20089 Camera Power Protector Driver");
    MODULE_LICENSE("GPL");
