//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-ltc4283.c
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
// Analog Devices LTC4283 GPIO driver
//
// Copyright 2025 Analog Devices Inc.
//

pub const LTC4283_PINS_MAX: c_int = 8;
pub const LTC4283_PGIOX_START_NR: c_int = 4;
pub const LTC4283_INPUT_STATUS: c_uint = 0x02;
pub const LTC4283_PGIO_CONFIG: c_uint = 0x10;

    GENMASK(((pin) - LTC4283_PGIOX_START_NR) * 2 + 1, (((pin) - LTC4283_PGIOX_START_NR) * 2))
pub const LTC4283_PGIO_CONFIG_2: c_uint = 0x11;
pub const LTC4283_ADIO_CONFIG: c_uint = 0x12;
// starts at bit 4

pub const LTC4283_PGIO_DIR_IN: c_int = 3;
pub const LTC4283_PGIO_DIR_OUT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc4283_gpio {
    pub gpio_chip: gpio_chip,
    pub regmap: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn ltc4283_pgio_get_direction(st: *const ltc4283_gpio, off: c_uint) -> c_int {
    static int ltc4283_pgio_get_direction(const struct ltc4283_gpio *st, unsigned int off)
    {
    unsigned int val;
    int ret;
    ret = regmap_read(st.regmap, LTC4283_PGIO_CONFIG, &val);
    if (ret)
    return ret;
    val = field_get(LTC4283_PGIO_CFG_MASK(off), val);
    if (val == LTC4283_PGIO_DIR_IN)
    return GPIO_LINE_DIRECTION_IN;
    return GPIO_LINE_DIRECTION_OUT;
    }
#[no_mangle]
unsafe extern "C" fn ltc4283_gpio_get_direction(gc: *mut gpio_chip, off: c_uint) -> c_int {
    static int ltc4283_gpio_get_direction(struct gpio_chip *gc, unsigned int off)
    {
    struct ltc4283_gpio *st = gpiochip_get_data(gc);
    unsigned int val;
    int ret;
    if (off >= LTC4283_PGIOX_START_NR)
    return ltc4283_pgio_get_direction(st, off);
    ret = regmap_read(st.regmap, LTC4283_ADIO_CONFIG, &val);
    if (ret)
    return ret;
    if (val & LTC4283_ADIOX_CONFIG_MASK(off))
    return GPIO_LINE_DIRECTION_IN;
    return GPIO_LINE_DIRECTION_OUT;
    }
    static int ltc4283_gpio_direction_set(const struct ltc4283_gpio *st,
    unsigned int off, bool input)
    {
    if (off >= LTC4283_PGIOX_START_NR) {
    let mut val: c_uint = LTC4283_PGIO_DIR_OUT;
    if (input)
    val = LTC4283_PGIO_DIR_IN;
    val = field_prep(LTC4283_PGIO_CFG_MASK(off), val);
    return regmap_update_bits(st.regmap, LTC4283_PGIO_CONFIG,
    LTC4283_PGIO_CFG_MASK(off), val);
    }
    return regmap_update_bits(st.regmap, LTC4283_ADIO_CONFIG,
    LTC4283_ADIOX_CONFIG_MASK(off),
    field_prep(LTC4283_ADIOX_CONFIG_MASK(off), input));
    }
    static int __ltc4283_gpio_set_value(const struct ltc4283_gpio *st,
    unsigned int off, int val)
    {
    let mut reg: u32 = off < LTC4283_PGIOX_START_NR ? LTC4283_ADIO_CONFIG : LTC4283_PGIO_CONFIG_2;
    return regmap_update_bits(st.regmap, reg, BIT(off),
    field_prep(BIT(off), !!val));
    }
#[no_mangle]
unsafe extern "C" fn ltc4283_gpio_direction_input(gc: *mut gpio_chip, off: c_uint) -> c_int {
    static int ltc4283_gpio_direction_input(struct gpio_chip *gc, unsigned int off)
    {
    struct ltc4283_gpio *st = gpiochip_get_data(gc);
    return ltc4283_gpio_direction_set(st, off, true);
    }
#[no_mangle]
unsafe extern "C" fn ltc4283_gpio_direction_output(gc: *mut gpio_chip, off: c_uint, val: c_int) -> c_int {
    static int ltc4283_gpio_direction_output(struct gpio_chip *gc, unsigned int off, int val)
    {
    struct ltc4283_gpio *st = gpiochip_get_data(gc);
    int ret;
    ret = ltc4283_gpio_direction_set(st, off, false);
    if (ret)
    return ret;
    return __ltc4283_gpio_set_value(st, off, val);
    }
#[no_mangle]
unsafe extern "C" fn ltc4283_gpio_get_value(gc: *mut gpio_chip, off: c_uint) -> c_int {
    static int ltc4283_gpio_get_value(struct gpio_chip *gc, unsigned int off)
    {
    struct ltc4283_gpio *st = gpiochip_get_data(gc);
    unsigned int val, reg;
    int ret, dir;
    dir = ltc4283_gpio_get_direction(gc, off);
    if (dir < 0)
    return dir;
    if (dir == GPIO_LINE_DIRECTION_IN) {
    ret = regmap_read(st.regmap, LTC4283_INPUT_STATUS, &val);
    if (ret)
    return ret;
// ADIO1 is at bit 3.
    if (off < LTC4283_PGIOX_START_NR)
    return !!(val & BIT(3 - off));
// PGIO1 is at bit 7.
    return !!(val & BIT(7 - (off - LTC4283_PGIOX_START_NR)));
    }
    if (off < LTC4283_PGIOX_START_NR)
    reg = LTC4283_ADIO_CONFIG;
    else
    reg = LTC4283_PGIO_CONFIG_2;
    ret = regmap_read(st.regmap, reg, &val);
    if (ret)
    return ret;
    return !!(val & BIT(off));
    }
#[no_mangle]
unsafe extern "C" fn ltc4283_gpio_set_value(gc: *mut gpio_chip, off: c_uint, val: c_int) -> c_int {
    static int ltc4283_gpio_set_value(struct gpio_chip *gc, unsigned int off, int val)
    {
    struct ltc4283_gpio *st = gpiochip_get_data(gc);
    return __ltc4283_gpio_set_value(st, off, val);
    }
    static int ltc4283_init_valid_mask(struct gpio_chip *gc, unsigned long *valid_mask,
    unsigned int ngpios)
    {
    unsigned long *mask = dev_get_platdata(gc.parent);
    bitmap_copy(valid_mask, mask, ngpios);
    return 0;
    }
    static int ltc4283_gpio_probe(struct auxiliary_device *adev,
    const struct auxiliary_device_id *id)
    {
    struct device *dev = &adev.dev;
    struct ltc4283_gpio *st;
    struct gpio_chip *gc;
    st = devm_kzalloc(dev, sizeof(*st), GFP_KERNEL);
    if (!st)
    return -ENOMEM;
    st.regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!st.regmap)
    return dev_err_probe(dev, -ENODEV,
    "Failed to get regmap\n");
    gc = &st.gpio_chip;
    gc.parent = dev;
    gc.get_direction = ltc4283_gpio_get_direction;
    gc.direction_input = ltc4283_gpio_direction_input;
    gc.direction_output = ltc4283_gpio_direction_output;
    gc.get = ltc4283_gpio_get_value;
    gc.set = ltc4283_gpio_set_value;
    gc.init_valid_mask = ltc4283_init_valid_mask;
    gc.can_sleep = true;
    gc.base = -1;
    gc.ngpio = LTC4283_PINS_MAX;
    gc.label = adev.name;
    gc.owner = THIS_MODULE;
    return devm_gpiochip_add_data(dev, &st.gpio_chip, st);
    }
    static const struct auxiliary_device_id ltc4283_aux_id_table[] = {
    { "ltc4283.gpio" },
    { }
    };
    MODULE_DEVICE_TABLE(auxiliary, ltc4283_aux_id_table);
    static struct auxiliary_driver ltc4283_gpio_driver = {
    .probe = ltc4283_gpio_probe,
    .id_table = ltc4283_aux_id_table,
    };
    module_auxiliary_driver(ltc4283_gpio_driver);
    MODULE_AUTHOR("Nuno Sá <nuno.sa@analog.com>");
    MODULE_DESCRIPTION("GPIO LTC4283 Driver");
    MODULE_LICENSE("GPL");
