//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/max7360.c
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
// Maxim MAX7360 Core Driver
//
// Copyright 2025 Bootlin
//
// Authors:
// Kamel Bouhara <kamel.bouhara@bootlin.com>
// Mathieu Dubois-Briand <mathieu.dubois-briand@bootlin.com>
//

    static const struct mfd_cell max7360_cells[] = {
    { .name           = "max7360-pinctrl" },
    { .name           = "max7360-pwm" },
    { .name           = "max7360-keypad" },
    { .name           = "max7360-rotary" },
    {
    .name           = "max7360-gpo",
    .of_compatible	= "maxim,max7360-gpo",
    },
    {
    .name           = "max7360-gpio",
    .of_compatible	= "maxim,max7360-gpio",
    },
    };
    static const struct regmap_range max7360_volatile_ranges[] = {
    regmap_reg_range(MAX7360_REG_KEYFIFO, MAX7360_REG_KEYFIFO),
    regmap_reg_range(MAX7360_REG_I2C_TIMEOUT, MAX7360_REG_RTR_CNT),
    };
    static const struct regmap_access_table max7360_volatile_table = {
    .yes_ranges = max7360_volatile_ranges,
    .n_yes_ranges = ARRAY_SIZE(max7360_volatile_ranges),
    };
    static const struct regmap_config max7360_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = MAX7360_REG_PWMCFG(MAX7360_PORT_PWM_COUNT - 1),
    .volatile_table = &max7360_volatile_table,
    .cache_type = REGCACHE_MAPLE,
    };
#[no_mangle]
unsafe extern "C" fn max7360_mask_irqs(regmap: *mut regmap) -> c_int {
    static int max7360_mask_irqs(struct regmap *regmap)
    {
    struct device *dev = regmap_get_device(regmap);
    unsigned int val;
    int ret;
//
// GPIO/PWM interrupts are not masked on reset: as the MAX7360 "INTI"
// interrupt line is shared between GPIOs and rotary encoder, this could
// result in repeated spurious interrupts on the rotary encoder driver
// if the GPIO driver is not loaded. Mask them now to avoid this
// situation.
//
    for (unsigned int i = 0; i < MAX7360_PORT_PWM_COUNT; i++) {
    ret = regmap_write_bits(regmap, MAX7360_REG_PWMCFG(i),
    MAX7360_PORT_CFG_INTERRUPT_MASK,
    MAX7360_PORT_CFG_INTERRUPT_MASK);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to write MAX7360 port configuration\n");
    }
// Read GPIO in register, to ACK any pending IRQ.
    ret = regmap_read(regmap, MAX7360_REG_GPIOIN, &val);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to read GPIO values\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max7360_reset(regmap: *mut regmap) -> c_int {
    static int max7360_reset(struct regmap *regmap)
    {
    struct device *dev = regmap_get_device(regmap);
    int ret;
    ret = regmap_write(regmap, MAX7360_REG_GPIOCFG, MAX7360_GPIO_CFG_GPIO_RST);
    if (ret) {
    dev_err(dev, "Failed to reset GPIO configuration: %x\n", ret);
    return ret;
    }
    ret = regcache_drop_region(regmap, MAX7360_REG_GPIOCFG, MAX7360_REG_GPIO_LAST);
    if (ret) {
    dev_err(dev, "Failed to drop regmap cache: %x\n", ret);
    return ret;
    }
    ret = regmap_write(regmap, MAX7360_REG_SLEEP, 0);
    if (ret) {
    dev_err(dev, "Failed to reset autosleep configuration: %x\n", ret);
    return ret;
    }
    ret = regmap_write(regmap, MAX7360_REG_DEBOUNCE, 0);
    if (ret)
    dev_err(dev, "Failed to reset GPO port count: %x\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn max7360_probe(client: *mut i2c_client) -> c_int {
    static int max7360_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct regmap *regmap;
    int ret;
    regmap = devm_regmap_init_i2c(client, &max7360_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap), "Failed to initialise regmap\n");
    ret = max7360_reset(regmap);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to reset device\n");
// Get the device out of shutdown mode.
    ret = regmap_write_bits(regmap, MAX7360_REG_GPIOCFG,
    MAX7360_GPIO_CFG_GPIO_EN,
    MAX7360_GPIO_CFG_GPIO_EN);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to enable GPIO and PWM module\n");
    ret = max7360_mask_irqs(regmap);
    if (ret)
    return dev_err_probe(dev, ret, "Could not mask interrupts\n");
    ret = devm_mfd_add_devices(dev, PLATFORM_DEVID_NONE,
    max7360_cells, ARRAY_SIZE(max7360_cells),
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret)
    return dev_err_probe(dev, ret, "Failed to register child devices\n");
    return 0;
    }
    static const struct of_device_id max7360_dt_match[] = {
    { .compatible = "maxim,max7360" },
    {}
    };
    MODULE_DEVICE_TABLE(of, max7360_dt_match);
    static struct i2c_driver max7360_driver = {
    .driver = {
    .name = "max7360",
    .of_match_table = max7360_dt_match,
    },
    .probe = max7360_probe,
    };
    module_i2c_driver(max7360_driver);
    MODULE_DESCRIPTION("Maxim MAX7360 I2C IO Expander core driver");
    MODULE_AUTHOR("Kamel Bouhara <kamel.bouhara@bootlin.com>");
    MODULE_LICENSE("GPL");
