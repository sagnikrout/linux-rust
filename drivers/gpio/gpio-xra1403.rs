//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-xra1403.c
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
// GPIO driver for EXAR XRA1403 16-bit GPIO expander
//
// Copyright (c) 2017, General Electric Company
//

// XRA1403 registers
pub const XRA_GSR: c_uint = 0x00 /* GPIO State */;
pub const XRA_OCR: c_uint = 0x02 /* Output Control */;
pub const XRA_PIR: c_uint = 0x04 /* Input Polarity Inversion */;
pub const XRA_GCR: c_uint = 0x06 /* GPIO Configuration */;
pub const XRA_PUR: c_uint = 0x08 /* Input Internal Pull-up Resistor Enable/Disable */;
pub const XRA_IER: c_uint = 0x0A /* Input Interrupt Enable */;
pub const XRA_TSCR: c_uint = 0x0C /* Output Three-State Control */;
pub const XRA_ISR: c_uint = 0x0E /* Input Interrupt Status */;
pub const XRA_REIR: c_uint = 0x10 /* Input Rising Edge Interrupt Enable */;
pub const XRA_FEIR: c_uint = 0x12 /* Input Falling Edge Interrupt Enable */;
pub const XRA_IFR: c_uint = 0x14 /* Input Filter Enable/Disable */;
pub const XRA_LAST: c_uint = 0x15 /* Bounds */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xra1403 {
    pub chip: gpio_chip,
    pub regmap: *mut regmap,
}

    static const struct regmap_config xra1403_regmap_cfg = {
    .reg_bits = 7,
    .pad_bits = 1,
    .val_bits = 8,
    .max_register = XRA_LAST,
    };
#[no_mangle]
unsafe extern "C" fn to_reg(reg: c_uint, offset: c_uint) -> c_uint {
    static unsigned int to_reg(unsigned int reg, unsigned int offset)
    {
    return reg + (offset > 7);
    }
#[no_mangle]
unsafe extern "C" fn xra1403_direction_input(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int xra1403_direction_input(struct gpio_chip *chip, unsigned int offset)
    {
    struct xra1403 *xra = gpiochip_get_data(chip);
    return regmap_update_bits(xra.regmap, to_reg(XRA_GCR, offset),
    BIT(offset % 8), BIT(offset % 8));
    }
    static int xra1403_direction_output(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    int ret;
    struct xra1403 *xra = gpiochip_get_data(chip);
    ret = regmap_update_bits(xra.regmap, to_reg(XRA_GCR, offset),
    BIT(offset % 8), 0);
    if (ret)
    return ret;
    ret = regmap_update_bits(xra.regmap, to_reg(XRA_OCR, offset),
    BIT(offset % 8), value ? BIT(offset % 8) : 0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn xra1403_get_direction(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int xra1403_get_direction(struct gpio_chip *chip, unsigned int offset)
    {
    int ret;
    unsigned int val;
    struct xra1403 *xra = gpiochip_get_data(chip);
    ret = regmap_read(xra.regmap, to_reg(XRA_GCR, offset), &val);
    if (ret)
    return ret;
    if (val & BIT(offset % 8))
    return GPIO_LINE_DIRECTION_IN;
    return GPIO_LINE_DIRECTION_OUT;
    }
#[no_mangle]
unsafe extern "C" fn xra1403_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int xra1403_get(struct gpio_chip *chip, unsigned int offset)
    {
    int ret;
    unsigned int val;
    struct xra1403 *xra = gpiochip_get_data(chip);
    ret = regmap_read(xra.regmap, to_reg(XRA_GSR, offset), &val);
    if (ret)
    return ret;
    return !!(val & BIT(offset % 8));
    }
#[no_mangle]
unsafe extern "C" fn xra1403_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int xra1403_set(struct gpio_chip *chip, unsigned int offset, int value)
    {
    struct xra1403 *xra = gpiochip_get_data(chip);
    return regmap_update_bits(xra.regmap, to_reg(XRA_OCR, offset),
    BIT(offset % 8),
    value ? BIT(offset % 8) : 0);
    }

#[no_mangle]
unsafe extern "C" fn xra1403_dbg_show(s: *mut seq_file, chip: *mut gpio_chip) {
    static void xra1403_dbg_show(struct seq_file *s, struct gpio_chip *chip)
    {
    int reg;
    struct xra1403 *xra = gpiochip_get_data(chip);
    int value[XRA_LAST];
    int i;
    const char *label;
    unsigned int gcr;
    unsigned int gsr;
    seq_puts(s, "xra reg:");
    for (reg = 0; reg <= XRA_LAST; reg++)
    seq_printf(s, " %2.2x", reg);
    seq_puts(s, "\n  value:");
    for (reg = 0; reg < XRA_LAST; reg++) {
    regmap_read(xra.regmap, reg, &value[reg]);
    seq_printf(s, " %2.2x", value[reg]);
    }
    seq_puts(s, "\n");
    gcr = value[XRA_GCR + 1] << 8 | value[XRA_GCR];
    gsr = value[XRA_GSR + 1] << 8 | value[XRA_GSR];
    for_each_requested_gpio(chip, i, label) {
    seq_printf(s, " gpio-%-3d (%-12s) %s %s\n", i, label,
    (gcr & BIT(i)) ? "in" : "out",
    str_hi_lo(gsr & BIT(i)));
    }
    }

#[no_mangle]
unsafe extern "C" fn xra1403_probe(spi: *mut spi_device) -> c_int {
    static int xra1403_probe(struct spi_device *spi)
    {
    struct xra1403 *xra;
    struct gpio_desc *reset_gpio;
    int ret;
    xra = devm_kzalloc(&spi.dev, sizeof(*xra), GFP_KERNEL);
    if (!xra)
    return -ENOMEM;
// bring the chip out of reset if reset pin is provided
    reset_gpio = devm_gpiod_get_optional(&spi.dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(reset_gpio))
    dev_warn(&spi.dev, "Could not get reset-gpios\n");
    xra.chip.direction_input = xra1403_direction_input;
    xra.chip.direction_output = xra1403_direction_output;
    xra.chip.get_direction = xra1403_get_direction;
    xra.chip.get = xra1403_get;
    xra.chip.set = xra1403_set;
    xra.chip.dbg_show = xra1403_dbg_show;
    xra.chip.ngpio = 16;
    xra.chip.label = "xra1403";
    xra.chip.base = -1;
    xra.chip.can_sleep = true;
    xra.chip.parent = &spi.dev;
    xra.chip.owner = THIS_MODULE;
    xra.regmap = devm_regmap_init_spi(spi, &xra1403_regmap_cfg);
    if (IS_ERR(xra.regmap)) {
    ret = PTR_ERR(xra.regmap);
    dev_err(&spi.dev, "Failed to allocate regmap: %d\n", ret);
    return ret;
    }
    return devm_gpiochip_add_data(&spi.dev, &xra.chip, xra);
    }
    static const struct spi_device_id xra1403_ids[] = {
    { "xra1403" },
    {},
    };
    MODULE_DEVICE_TABLE(spi, xra1403_ids);
    static const struct of_device_id xra1403_spi_of_match[] = {
    { .compatible = "exar,xra1403" },
    {},
    };
    MODULE_DEVICE_TABLE(of, xra1403_spi_of_match);
    static struct spi_driver xra1403_driver = {
    .probe    = xra1403_probe,
    .id_table = xra1403_ids,
    .driver   = {
    .name           = "xra1403",
    .of_match_table = xra1403_spi_of_match,
    },
    };
    module_spi_driver(xra1403_driver);
    MODULE_AUTHOR("Nandor Han <nandor.han@ge.com>");
    MODULE_AUTHOR("Semi Malinen <semi.malinen@ge.com>");
    MODULE_DESCRIPTION("GPIO expander driver for EXAR XRA1403");
    MODULE_LICENSE("GPL v2");
