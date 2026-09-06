//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/rpi-panel-attiny-regulator.c
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
// Copyright (C) 2020 Marek Vasut <marex@denx.de>
//
// Based on rpi_touchscreen.c by Eric Anholt <eric@anholt.net>
//

// I2C registers of the Atmel microcontroller.
pub const REG_ID: c_uint = 0x80;
pub const REG_PORTA: c_uint = 0x81;
pub const REG_PORTB: c_uint = 0x82;
pub const REG_PORTC: c_uint = 0x83;
pub const REG_POWERON: c_uint = 0x85;
pub const REG_PWM: c_uint = 0x86;
pub const REG_ADDR_L: c_uint = 0x8c;
pub const REG_ADDR_H: c_uint = 0x8d;
pub const REG_WRITE_DATA_H: c_uint = 0x90;
pub const REG_WRITE_DATA_L: c_uint = 0x91;

    enum gpio_signals {
    RST_BRIDGE_N,	/* TC358762 bridge reset */
    RST_TP_N,	/* Touch controller reset */
    NUM_GPIO
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_signal_mappings {
    pub reg: c_uint,
    pub mask: c_uint,
}

    static const struct gpio_signal_mappings mappings[NUM_GPIO] = {
    [RST_BRIDGE_N] = { REG_PORTC, PC_RST_BRIDGE_N | PC_RST_LCD_N  },
    [RST_TP_N] = { REG_PORTC, PC_RST_TP_N },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct attiny_lcd {
// lock to serialise overall accesses to the Atmel
    pub lock: mutex,
    pub regmap: *mut regmap,
    pub gpio_states: [bool; NUM_GPIO],
    pub port_states: [u8; 3],
    pub gc: gpio_chip,
}

    static const struct regmap_config attiny_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .disable_locking = 1,
    .max_register = REG_WRITE_DATA_L,
    .cache_type = REGCACHE_MAPLE,
    };
#[no_mangle]
unsafe extern "C" fn attiny_set_port_state(state: *mut attiny_lcd, reg: c_int, val: u8) -> c_int {
    static int attiny_set_port_state(struct attiny_lcd *state, int reg, u8 val)
    {
    state.port_states[reg - REG_PORTA] = val;
    return regmap_write(state.regmap, reg, val);
    };
#[no_mangle]
unsafe extern "C" fn attiny_get_port_state(state: *mut attiny_lcd, reg: c_int) -> u8 {
    static u8 attiny_get_port_state(struct attiny_lcd *state, int reg)
    {
    return state.port_states[reg - REG_PORTA];
    };
#[no_mangle]
unsafe extern "C" fn attiny_lcd_power_enable(rdev: *mut regulator_dev) -> c_int {
    static int attiny_lcd_power_enable(struct regulator_dev *rdev)
    {
    struct attiny_lcd *state = rdev_get_drvdata(rdev);
    guard(mutex)(&state.lock);
// Ensure bridge, and tp stay in reset
    attiny_set_port_state(state, REG_PORTC, 0);
    usleep_range(5000, 10000);
// Default to the same orientation as the closed source
// firmware used for the panel.  Runtime rotation
// configuration will be supported using VC4's plane
// orientation bits.
//
    attiny_set_port_state(state, REG_PORTA, PA_LCD_LR);
    usleep_range(5000, 10000);
// Main regulator on, and power to the panel (LCD_VCC_N)
    attiny_set_port_state(state, REG_PORTB, PB_LCD_MAIN);
    usleep_range(5000, 10000);
// Bring controllers out of reset
    attiny_set_port_state(state, REG_PORTC, PC_LED_EN);
    msleep(80);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn attiny_lcd_power_disable(rdev: *mut regulator_dev) -> c_int {
    static int attiny_lcd_power_disable(struct regulator_dev *rdev)
    {
    struct attiny_lcd *state = rdev_get_drvdata(rdev);
    guard(mutex)(&state.lock);
    regmap_write(rdev.regmap, REG_PWM, 0);
    usleep_range(5000, 10000);
    attiny_set_port_state(state, REG_PORTA, 0);
    usleep_range(5000, 10000);
    attiny_set_port_state(state, REG_PORTB, PB_LCD_VCC_N);
    usleep_range(5000, 10000);
    attiny_set_port_state(state, REG_PORTC, 0);
    msleep(30);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn attiny_lcd_power_is_enabled(rdev: *mut regulator_dev) -> c_int {
    static int attiny_lcd_power_is_enabled(struct regulator_dev *rdev)
    {
    struct attiny_lcd *state = rdev_get_drvdata(rdev);
    unsigned int data;
    let mut ret: c_int = 0, i;
    scoped_guard(mutex, &state.lock) {
    for (i = 0; i < 10; i++) {
    ret = regmap_read(rdev.regmap, REG_PORTC, &data);
    if (!ret)
    break;
    usleep_range(10000, 12000);
    }
    }
    if (ret < 0)
    return ret;
    return data & PC_RST_BRIDGE_N;
    }
    static const struct regulator_init_data attiny_regulator_default = {
    .constraints = {
    .valid_ops_mask = REGULATOR_CHANGE_STATUS,
    },
    };
    static const struct regulator_ops attiny_regulator_ops = {
    .enable = attiny_lcd_power_enable,
    .disable = attiny_lcd_power_disable,
    .is_enabled = attiny_lcd_power_is_enabled,
    };
    static const struct regulator_desc attiny_regulator = {
    .name	= "tc358762-power",
    .ops	= &attiny_regulator_ops,
    .type	= REGULATOR_VOLTAGE,
    .owner	= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn attiny_update_status(bl: *mut backlight_device) -> c_int {
    static int attiny_update_status(struct backlight_device *bl)
    {
    struct attiny_lcd *state = bl_get_data(bl);
    struct regmap *regmap = state.regmap;
    let mut brightness: c_int = backlight_get_brightness(bl);
    int ret, i;
    guard(mutex)(&state.lock);
    for (i = 0; i < 10; i++) {
    ret = regmap_write(regmap, REG_PWM, brightness);
    if (!ret)
    break;
    }
    return ret;
    }
    static const struct backlight_ops attiny_bl = {
    .update_status	= attiny_update_status,
    };
#[no_mangle]
unsafe extern "C" fn attiny_gpio_get_direction(gc: *mut gpio_chip, off: c_uint) -> c_int {
    static int attiny_gpio_get_direction(struct gpio_chip *gc, unsigned int off)
    {
    return GPIO_LINE_DIRECTION_OUT;
    }
#[no_mangle]
unsafe extern "C" fn attiny_gpio_set(gc: *mut gpio_chip, off: c_uint, val: c_int) -> c_int {
    static int attiny_gpio_set(struct gpio_chip *gc, unsigned int off, int val)
    {
    struct attiny_lcd *state = gpiochip_get_data(gc);
    u8 last_val;
    guard(mutex)(&state.lock);
    last_val = attiny_get_port_state(state, mappings[off].reg);
    if (val)
    last_val |= mappings[off].mask;
    else
    last_val &= ~mappings[off].mask;
    attiny_set_port_state(state, mappings[off].reg, last_val);
    if (off == RST_BRIDGE_N && val) {
    usleep_range(5000, 8000);
    regmap_write(state.regmap, REG_ADDR_H, 0x04);
    usleep_range(5000, 8000);
    regmap_write(state.regmap, REG_ADDR_L, 0x7c);
    usleep_range(5000, 8000);
    regmap_write(state.regmap, REG_WRITE_DATA_H, 0x00);
    usleep_range(5000, 8000);
    regmap_write(state.regmap, REG_WRITE_DATA_L, 0x00);
    msleep(100);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn attiny_i2c_read(client: *mut i2c_client, reg: u8, buf: *mut c_uint) -> c_int {
    static int attiny_i2c_read(struct i2c_client *client, u8 reg, unsigned int *buf)
    {
    struct i2c_msg msgs[1];
    u8 addr_buf[1] = { reg };
    u8 data_buf[1] = { 0, };
    int ret;
// Write register address
    msgs[0].addr = client.addr;
    msgs[0].flags = 0;
    msgs[0].len = ARRAY_SIZE(addr_buf);
    msgs[0].buf = addr_buf;
    ret = i2c_transfer(client.adapter, msgs, ARRAY_SIZE(msgs));
    if (ret != ARRAY_SIZE(msgs))
    return -EIO;
    usleep_range(5000, 10000);
// Read data from register
    msgs[0].addr = client.addr;
    msgs[0].flags = I2C_M_RD;
    msgs[0].len = 1;
    msgs[0].buf = data_buf;
    ret = i2c_transfer(client.adapter, msgs, ARRAY_SIZE(msgs));
    if (ret != ARRAY_SIZE(msgs))
    return -EIO;
// buf = data_buf[0];
    return 0;
    }
//
// I2C driver interface functions
//
#[no_mangle]
unsafe extern "C" fn attiny_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int attiny_i2c_probe(struct i2c_client *i2c)
    {
    let mut props: backlight_properties = { };
    let mut config: regulator_config = { };
    struct backlight_device *bl;
    struct regulator_dev *rdev;
    struct attiny_lcd *state;
    struct regmap *regmap;
    unsigned int data;
    int ret;
    state = devm_kzalloc(&i2c.dev, sizeof(*state), GFP_KERNEL);
    if (!state)
    return -ENOMEM;
    ret = devm_mutex_init(&i2c.dev, &state.lock);
    if (ret)
    return ret;
    i2c_set_clientdata(i2c, state);
    regmap = devm_regmap_init_i2c(i2c, &attiny_regmap_config);
    if (IS_ERR(regmap)) {
    ret = PTR_ERR(regmap);
    dev_err(&i2c.dev, "Failed to allocate register map: %d\n",
    ret);
    return ret;
    }
    ret = attiny_i2c_read(i2c, REG_ID, &data);
    if (ret < 0) {
    dev_err(&i2c.dev, "Failed to read REG_ID reg: %d\n", ret);
    return ret;
    }
    switch (data) {
    case 0xde: /* ver 1 */
    case 0xc3: /* ver 2 */
    break;
    default:
    dev_err(&i2c.dev, "Unknown Atmel firmware revision: 0x%02x\n", data);
    return -ENODEV;
    }
    regmap_write(regmap, REG_POWERON, 0);
    msleep(30);
    regmap_write(regmap, REG_PWM, 0);
    config.dev = &i2c.dev;
    config.regmap = regmap;
    config.of_node = i2c.dev.of_node;
    config.init_data = &attiny_regulator_default;
    config.driver_data = state;
    rdev = devm_regulator_register(&i2c.dev, &attiny_regulator, &config);
    if (IS_ERR(rdev)) {
    dev_err(&i2c.dev, "Failed to register ATTINY regulator\n");
    return PTR_ERR(rdev);
    }
    props.type = BACKLIGHT_RAW;
    props.max_brightness = 0xff;
    state.regmap = regmap;
    bl = devm_backlight_device_register(&i2c.dev, dev_name(&i2c.dev),
    &i2c.dev, state, &attiny_bl,
    &props);
    if (IS_ERR(bl))
    return PTR_ERR(bl);
    bl.props.brightness = 0xff;
    state.gc.parent = &i2c.dev;
    state.gc.label = i2c.name;
    state.gc.owner = THIS_MODULE;
    state.gc.base = -1;
    state.gc.ngpio = NUM_GPIO;
    state.gc.set = attiny_gpio_set;
    state.gc.get_direction = attiny_gpio_get_direction;
    state.gc.can_sleep = true;
    ret = devm_gpiochip_add_data(&i2c.dev, &state.gc, state);
    if (ret)
    dev_err(&i2c.dev, "Failed to create gpiochip: %d\n", ret);
    return ret;
    }
    static const struct of_device_id attiny_dt_ids[] = {
    { .compatible = "raspberrypi,7inch-touchscreen-panel-regulator" },
    {},
    };
    MODULE_DEVICE_TABLE(of, attiny_dt_ids);
    static struct i2c_driver attiny_regulator_driver = {
    .driver = {
    .name = "rpi_touchscreen_attiny",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = attiny_dt_ids,
    },
    .probe = attiny_i2c_probe,
    };
    module_i2c_driver(attiny_regulator_driver);
    MODULE_AUTHOR("Marek Vasut <marex@denx.de>");
    MODULE_DESCRIPTION("Regulator device driver for Raspberry Pi 7-inch touchscreen");
    MODULE_LICENSE("GPL v2");
