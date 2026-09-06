//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/ad5398.c
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
// Voltage and current regulation for AD5398 and AD5821
//
// Copyright 2010 Analog Devices Inc.
//
// Enter bugs at http://blackfin.uclinux.org
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5398_chip_info {
    pub client: *mut i2c_client,
    pub min_uA: c_int,
    pub max_uA: c_int,
    pub current_level: c_uint,
    pub current_mask: c_uint,
    pub current_offset: c_uint,
    pub rdev: *mut regulator_dev,
}

    static int ad5398_calc_current(struct ad5398_chip_info *chip,
    unsigned selector)
    {
    let mut range_uA: unsigned = chip.max_uA - chip.min_uA;
    return chip.min_uA + (selector * range_uA / chip.current_level);
    }
#[no_mangle]
unsafe extern "C" fn ad5398_read_reg(client: *mut i2c_client, data: *mut c_ushort) -> c_int {
    static int ad5398_read_reg(struct i2c_client *client, unsigned short *data)
    {
    unsigned short val;
    int ret;
    ret = i2c_master_recv(client, (char *)&val, 2);
    if (ret < 0) {
    dev_err(&client.dev, "I2C read error\n");
    return ret;
    }
// data = be16_to_cpu(val);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ad5398_write_reg(client: *mut i2c_client, data: c_ushort) -> c_int {
    static int ad5398_write_reg(struct i2c_client *client, const unsigned short data)
    {
    unsigned short val;
    int ret;
    val = cpu_to_be16(data);
    ret = i2c_master_send(client, (char *)&val, 2);
    if (ret != 2) {
    dev_err(&client.dev, "I2C write error\n");
    return ret < 0 ? ret : -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad5398_get_current_limit(rdev: *mut regulator_dev) -> c_int {
    static int ad5398_get_current_limit(struct regulator_dev *rdev)
    {
    struct ad5398_chip_info *chip = rdev_get_drvdata(rdev);
    struct i2c_client *client = chip.client;
    unsigned short data;
    int ret;
    ret = ad5398_read_reg(client, &data);
    if (ret < 0)
    return ret;
    ret = (data & chip.current_mask) >> chip.current_offset;
    return ad5398_calc_current(chip, ret);
    }
#[no_mangle]
unsafe extern "C" fn ad5398_set_current_limit(rdev: *mut regulator_dev, min_uA: c_int, max_uA: c_int) -> c_int {
    static int ad5398_set_current_limit(struct regulator_dev *rdev, int min_uA, int max_uA)
    {
    struct ad5398_chip_info *chip = rdev_get_drvdata(rdev);
    struct i2c_client *client = chip.client;
    let mut range_uA: unsigned = chip.max_uA - chip.min_uA;
    unsigned selector;
    unsigned short data;
    int ret;
    if (min_uA < chip.min_uA)
    min_uA = chip.min_uA;
    if (max_uA > chip.max_uA)
    max_uA = chip.max_uA;
    if (min_uA > chip.max_uA || max_uA < chip.min_uA)
    return -EINVAL;
    selector = DIV_ROUND_UP((min_uA - chip.min_uA) * chip.current_level,
    range_uA);
    if (ad5398_calc_current(chip, selector) > max_uA)
    return -EINVAL;
    dev_dbg(&client.dev, "changing current %duA\n",
    ad5398_calc_current(chip, selector));
// read chip enable bit
    ret = ad5398_read_reg(client, &data);
    if (ret < 0)
    return ret;
// prepare register data
    selector = (selector << chip.current_offset) & chip.current_mask;
    data = (unsigned short)selector | (data & AD5398_SW_POWER_DOWN);
// write the new current value back as well as enable bit
    ret = ad5398_write_reg(client, data);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ad5398_is_enabled(rdev: *mut regulator_dev) -> c_int {
    static int ad5398_is_enabled(struct regulator_dev *rdev)
    {
    struct ad5398_chip_info *chip = rdev_get_drvdata(rdev);
    struct i2c_client *client = chip.client;
    unsigned short data;
    int ret;
    ret = ad5398_read_reg(client, &data);
    if (ret < 0)
    return ret;
    if (data & AD5398_SW_POWER_DOWN)
    return 0;
    else
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn ad5398_enable(rdev: *mut regulator_dev) -> c_int {
    static int ad5398_enable(struct regulator_dev *rdev)
    {
    struct ad5398_chip_info *chip = rdev_get_drvdata(rdev);
    struct i2c_client *client = chip.client;
    unsigned short data;
    int ret;
    ret = ad5398_read_reg(client, &data);
    if (ret < 0)
    return ret;
    if (!(data & AD5398_SW_POWER_DOWN))
    return 0;
    data &= ~AD5398_SW_POWER_DOWN;
    ret = ad5398_write_reg(client, data);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ad5398_disable(rdev: *mut regulator_dev) -> c_int {
    static int ad5398_disable(struct regulator_dev *rdev)
    {
    struct ad5398_chip_info *chip = rdev_get_drvdata(rdev);
    struct i2c_client *client = chip.client;
    unsigned short data;
    int ret;
    ret = ad5398_read_reg(client, &data);
    if (ret < 0)
    return ret;
    if (data & AD5398_SW_POWER_DOWN)
    return 0;
    data |= AD5398_SW_POWER_DOWN;
    ret = ad5398_write_reg(client, data);
    return ret;
    }
    static const struct regulator_ops ad5398_ops = {
    .get_current_limit = ad5398_get_current_limit,
    .set_current_limit = ad5398_set_current_limit,
    .enable = ad5398_enable,
    .disable = ad5398_disable,
    .is_enabled = ad5398_is_enabled,
    };
    static const struct regulator_desc ad5398_reg = {
    .name = "isink",
    .id = 0,
    .ops = &ad5398_ops,
    .type = REGULATOR_CURRENT,
    .owner = THIS_MODULE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5398_current_data_format {
    pub current_bits: c_int,
    pub current_offset: c_int,
    pub min_uA: c_int,
    pub max_uA: c_int,
}

    let mut df_10_4_120: static struct ad5398_current_data_format = {10, 4, 0, 120000};
    static const struct i2c_device_id ad5398_id[] = {
    { .name = "ad5398", .driver_data = (kernel_ulong_t)&df_10_4_120 },
    { .name = "ad5821", .driver_data = (kernel_ulong_t)&df_10_4_120 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ad5398_id);
#[no_mangle]
unsafe extern "C" fn ad5398_probe(client: *mut i2c_client) -> c_int {
    static int ad5398_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct regulator_init_data *init_data = dev_get_platdata(&client.dev);
    let mut config: regulator_config = { };
    struct ad5398_chip_info *chip;
    const struct ad5398_current_data_format *df =
    (struct ad5398_current_data_format *)id.driver_data;
    chip = devm_kzalloc(&client.dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    config.dev = &client.dev;
    if (client.dev.of_node)
    init_data = of_get_regulator_init_data(&client.dev,
    client.dev.of_node,
    &ad5398_reg);
    if (!init_data)
    return -EINVAL;
    config.init_data = init_data;
    config.of_node = client.dev.of_node;
    config.driver_data = chip;
    chip.client = client;
    chip.min_uA = df.min_uA;
    chip.max_uA = df.max_uA;
    chip.current_level = 1 << df.current_bits;
    chip.current_offset = df.current_offset;
    chip.current_mask = (chip.current_level - 1) << chip.current_offset;
    chip.rdev = devm_regulator_register(&client.dev, &ad5398_reg,
    &config);
    if (IS_ERR(chip.rdev)) {
    dev_err(&client.dev, "failed to register %s %s\n",
    id.name, ad5398_reg.name);
    return PTR_ERR(chip.rdev);
    }
    i2c_set_clientdata(client, chip);
    dev_dbg(&client.dev, "%s regulator driver is registered.\n", id.name);
    return 0;
    }
    static struct i2c_driver ad5398_driver = {
    .probe = ad5398_probe,
    .driver		= {
    .name	= "ad5398",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .id_table	= ad5398_id,
    };
#[no_mangle]
unsafe extern "C" fn ad5398_init() -> int __init {
    static int __init ad5398_init(void)
    {
    return i2c_add_driver(&ad5398_driver);
    }
    subsys_initcall(ad5398_init);
#[no_mangle]
unsafe extern "C" fn ad5398_exit() -> void __exit {
    static void __exit ad5398_exit(void)
    {
    i2c_del_driver(&ad5398_driver);
    }
    module_exit(ad5398_exit);
    MODULE_DESCRIPTION("AD5398 and AD5821 current regulator driver");
    MODULE_AUTHOR("Sonic Zhang");
    MODULE_LICENSE("GPL");
