//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-lm3642.c
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
// Simple driver for Texas Instruments LM3642 LED Flash driver chip
// Copyright (C) 2012 Texas Instruments
//

    enum lm3642_mode {
    MODES_STASNDBY = 0,
    MODES_INDIC,
    MODES_TORCH,
    MODES_FLASH
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3642_chip_data {
    pub dev: *mut device,
    pub cdev_flash: led_classdev,
    pub cdev_torch: led_classdev,
    pub cdev_indicator: led_classdev,
    pub br_flash: u8,
    pub br_torch: u8,
    pub br_indicator: u8,
    pub torch_pin: enum lm3642_torch_pin_enable,
    pub strobe_pin: enum lm3642_strobe_pin_enable,
    pub tx_pin: enum lm3642_tx_pin_enable,
    pub pdata: *mut lm3642_platform_data,
    pub regmap: *mut regmap,
    pub lock: mutex,
    pub last_flag: c_uint,
}

// chip initialize
#[no_mangle]
unsafe extern "C" fn lm3642_chip_init(chip: *mut lm3642_chip_data) -> c_int {
    static int lm3642_chip_init(struct lm3642_chip_data *chip)
    {
    int ret;
    struct lm3642_platform_data *pdata = chip.pdata;
// set enable register
    ret = regmap_update_bits(chip.regmap, REG_ENABLE, EX_PIN_ENABLE_MASK,
    pdata.tx_pin);
    if (ret < 0)
    dev_err(chip.dev, "Failed to update REG_ENABLE Register\n");
    return ret;
    }
// chip control
    static int lm3642_control(struct lm3642_chip_data *chip,
    u8 brightness, enum lm3642_mode opmode)
    {
    int ret;
    ret = regmap_read(chip.regmap, REG_FLAG, &chip.last_flag);
    if (ret < 0) {
    dev_err(chip.dev, "Failed to read REG_FLAG Register\n");
    return ret;
    }
    if (chip.last_flag)
    dev_info(chip.dev, "Last FLAG is 0x%x\n", chip.last_flag);
// brightness 0 means off-state
    if (!brightness)
    opmode = MODES_STASNDBY;
    switch (opmode) {
    case MODES_TORCH:
    ret = regmap_update_bits(chip.regmap, REG_I_CTRL,
    TORCH_I_MASK << TORCH_I_SHIFT,
    (brightness - 1) << TORCH_I_SHIFT);
    if (chip.torch_pin)
    opmode |= (TORCH_PIN_EN_MASK << TORCH_PIN_EN_SHIFT);
    break;
    case MODES_FLASH:
    ret = regmap_update_bits(chip.regmap, REG_I_CTRL,
    FLASH_I_MASK << FLASH_I_SHIFT,
    (brightness - 1) << FLASH_I_SHIFT);
    if (chip.strobe_pin)
    opmode |= (STROBE_PIN_EN_MASK << STROBE_PIN_EN_SHIFT);
    break;
    case MODES_INDIC:
    ret = regmap_update_bits(chip.regmap, REG_I_CTRL,
    TORCH_I_MASK << TORCH_I_SHIFT,
    (brightness - 1) << TORCH_I_SHIFT);
    break;
    case MODES_STASNDBY:
    break;
    default:
    return -EINVAL;
    }
    if (ret < 0) {
    dev_err(chip.dev, "Failed to write REG_I_CTRL Register\n");
    return ret;
    }
    if (chip.tx_pin)
    opmode |= (TX_PIN_EN_MASK << TX_PIN_EN_SHIFT);
    ret = regmap_update_bits(chip.regmap, REG_ENABLE,
    MODE_BITS_MASK << MODE_BITS_SHIFT,
    opmode << MODE_BITS_SHIFT);
    return ret;
    }
// torch
// torch pin config for lm3642
    static ssize_t torch_pin_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    ssize_t ret;
    struct led_classdev *led_cdev = dev_get_drvdata(dev);
    struct lm3642_chip_data *chip =
    container_of(led_cdev, struct lm3642_chip_data, cdev_indicator);
    unsigned int state;
    ret = kstrtouint(buf, 10, &state);
    if (ret)
    return ret;
    if (state != 0)
    state = 0x01 << TORCH_PIN_EN_SHIFT;
    chip.torch_pin = state;
    ret = regmap_update_bits(chip.regmap, REG_ENABLE,
    TORCH_PIN_EN_MASK << TORCH_PIN_EN_SHIFT,
    state);
    if (ret < 0) {
    dev_err(chip.dev, "%s:i2c access fail to register\n", __func__);
    return ret;
    }
    return size;
    }
    static DEVICE_ATTR_WO(torch_pin);
    static int lm3642_torch_brightness_set(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    struct lm3642_chip_data *chip =
    container_of(cdev, struct lm3642_chip_data, cdev_torch);
    int ret;
    guard(mutex)(&chip.lock);
    chip.br_torch = brightness;
    ret = lm3642_control(chip, chip.br_torch, MODES_TORCH);
    return ret;
    }
// flash
// strobe pin config for lm3642
    static ssize_t strobe_pin_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    ssize_t ret;
    struct led_classdev *led_cdev = dev_get_drvdata(dev);
    struct lm3642_chip_data *chip =
    container_of(led_cdev, struct lm3642_chip_data, cdev_indicator);
    unsigned int state;
    ret = kstrtouint(buf, 10, &state);
    if (ret)
    return ret;
    if (state != 0)
    state = 0x01 << STROBE_PIN_EN_SHIFT;
    chip.strobe_pin = state;
    ret = regmap_update_bits(chip.regmap, REG_ENABLE,
    STROBE_PIN_EN_MASK << STROBE_PIN_EN_SHIFT,
    state);
    if (ret < 0) {
    dev_err(chip.dev, "%s:i2c access fail to register\n", __func__);
    return ret;
    }
    return size;
    }
    static DEVICE_ATTR_WO(strobe_pin);
    static int lm3642_strobe_brightness_set(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    struct lm3642_chip_data *chip =
    container_of(cdev, struct lm3642_chip_data, cdev_flash);
    int ret;
    guard(mutex)(&chip.lock);
    chip.br_flash = brightness;
    ret = lm3642_control(chip, chip.br_flash, MODES_FLASH);
    return ret;
    }
// indicator
    static int lm3642_indicator_brightness_set(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    struct lm3642_chip_data *chip =
    container_of(cdev, struct lm3642_chip_data, cdev_indicator);
    int ret;
    guard(mutex)(&chip.lock);
    chip.br_indicator = brightness;
    ret = lm3642_control(chip, chip.br_indicator, MODES_INDIC);
    return ret;
    }
    static const struct regmap_config lm3642_regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = REG_MAX,
    };
    static struct attribute *lm3642_flash_attrs[] = {
    &dev_attr_strobe_pin.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(lm3642_flash);
    static struct attribute *lm3642_torch_attrs[] = {
    &dev_attr_torch_pin.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(lm3642_torch);
#[no_mangle]
unsafe extern "C" fn lm3642_probe(client: *mut i2c_client) -> c_int {
    static int lm3642_probe(struct i2c_client *client)
    {
    struct lm3642_platform_data *pdata = dev_get_platdata(&client.dev);
    struct lm3642_chip_data *chip;
    int err;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C)) {
    dev_err(&client.dev, "i2c functionality check fail.\n");
    return -EOPNOTSUPP;
    }
    if (pdata == core::ptr::null_mut()) {
    dev_err(&client.dev, "needs Platform Data.\n");
    return -ENODATA;
    }
    chip = devm_kzalloc(&client.dev,
    sizeof(struct lm3642_chip_data), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.dev = &client.dev;
    chip.pdata = pdata;
    chip.tx_pin = pdata.tx_pin;
    chip.torch_pin = pdata.torch_pin;
    chip.strobe_pin = pdata.strobe_pin;
    chip.regmap = devm_regmap_init_i2c(client, &lm3642_regmap);
    if (IS_ERR(chip.regmap)) {
    err = PTR_ERR(chip.regmap);
    dev_err(&client.dev, "Failed to allocate register map: %d\n",
    err);
    return err;
    }
    mutex_init(&chip.lock);
    i2c_set_clientdata(client, chip);
    err = lm3642_chip_init(chip);
    if (err < 0)
    goto err_out;
// flash
    chip.cdev_flash.name = "flash";
    chip.cdev_flash.max_brightness = 16;
    chip.cdev_flash.brightness_set_blocking = lm3642_strobe_brightness_set;
    chip.cdev_flash.default_trigger = "flash";
    chip.cdev_flash.groups = lm3642_flash_groups;
    err = led_classdev_register(&client.dev, &chip.cdev_flash);
    if (err < 0) {
    dev_err(chip.dev, "failed to register flash\n");
    goto err_out;
    }
// torch
    chip.cdev_torch.name = "torch";
    chip.cdev_torch.max_brightness = 8;
    chip.cdev_torch.brightness_set_blocking = lm3642_torch_brightness_set;
    chip.cdev_torch.default_trigger = "torch";
    chip.cdev_torch.groups = lm3642_torch_groups;
    err = led_classdev_register(&client.dev, &chip.cdev_torch);
    if (err < 0) {
    dev_err(chip.dev, "failed to register torch\n");
    goto err_create_torch_file;
    }
// indicator
    chip.cdev_indicator.name = "indicator";
    chip.cdev_indicator.max_brightness = 8;
    chip.cdev_indicator.brightness_set_blocking =
    lm3642_indicator_brightness_set;
    err = led_classdev_register(&client.dev, &chip.cdev_indicator);
    if (err < 0) {
    dev_err(chip.dev, "failed to register indicator\n");
    goto err_create_indicator_file;
    }
    dev_info(&client.dev, "LM3642 is initialized\n");
    return 0;
    err_create_indicator_file:
    led_classdev_unregister(&chip.cdev_torch);
    err_create_torch_file:
    led_classdev_unregister(&chip.cdev_flash);
    err_out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn lm3642_remove(client: *mut i2c_client) {
    static void lm3642_remove(struct i2c_client *client)
    {
    struct lm3642_chip_data *chip = i2c_get_clientdata(client);
    led_classdev_unregister(&chip.cdev_indicator);
    led_classdev_unregister(&chip.cdev_torch);
    led_classdev_unregister(&chip.cdev_flash);
    regmap_write(chip.regmap, REG_ENABLE, 0);
    }
    static const struct i2c_device_id lm3642_id[] = {
    { .name = LM3642_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lm3642_id);
    static struct i2c_driver lm3642_i2c_driver = {
    .driver = {
    .name = LM3642_NAME,
    .pm = core::ptr::null_mut(),
    },
    .probe = lm3642_probe,
    .remove = lm3642_remove,
    .id_table = lm3642_id,
    };
    module_i2c_driver(lm3642_i2c_driver);
    MODULE_DESCRIPTION("Texas Instruments Flash Lighting driver for LM3642");
    MODULE_AUTHOR("Daniel Jeong <daniel.jeong@ti.com>");
    MODULE_AUTHOR("G.Shark Jeong <gshark.jeong@gmail.com>");
    MODULE_LICENSE("GPL v2");
