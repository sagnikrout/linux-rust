//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-octeon.c
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


//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Copyright (C) 2011, 2012 Cavium Inc.
//

pub const RX_DAT: c_uint = 0x80;
pub const TX_SET: c_uint = 0x88;
pub const TX_CLEAR: c_uint = 0x90;
//
// The address offset of the GPIO configuration register for a given
// line.
//
#[no_mangle]
unsafe extern "C" fn bit_cfg_reg(offset: c_uint) -> c_uint {
    static unsigned int bit_cfg_reg(unsigned int offset)
    {
//
// The register stride is 8, with a discontinuity after the
// first 16.
//
    if (offset < 16)
    return 8 * offset;
    else
    return 8 * (offset - 16) + 0x100;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_gpio {
    pub chip: gpio_chip,
    pub register_base: u64,
}

#[no_mangle]
unsafe extern "C" fn octeon_gpio_dir_in(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int octeon_gpio_dir_in(struct gpio_chip *chip, unsigned offset)
    {
    struct octeon_gpio *gpio = gpiochip_get_data(chip);
    cvmx_write_csr(gpio.register_base + bit_cfg_reg(offset), 0);
    return 0;
    }
    static int octeon_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct octeon_gpio *gpio = gpiochip_get_data(chip);
    let mut mask: u64 = 1ull << offset;
    let mut reg: u64 = gpio.register_base + (value ? TX_SET : TX_CLEAR);
    cvmx_write_csr(reg, mask);
    return 0;
    }
    static int octeon_gpio_dir_out(struct gpio_chip *chip, unsigned offset,
    int value)
    {
    struct octeon_gpio *gpio = gpiochip_get_data(chip);
    union cvmx_gpio_bit_cfgx cfgx;
    octeon_gpio_set(chip, offset, value);
    cfgx.u64 = 0;
    cfgx.s.tx_oe = 1;
    cvmx_write_csr(gpio.register_base + bit_cfg_reg(offset), cfgx.u64);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn octeon_gpio_get(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int octeon_gpio_get(struct gpio_chip *chip, unsigned offset)
    {
    struct octeon_gpio *gpio = gpiochip_get_data(chip);
    let mut read_bits: u64 = cvmx_read_csr(gpio.register_base + RX_DAT);
    return ((1ull << offset) & read_bits) != 0;
    }
#[no_mangle]
unsafe extern "C" fn octeon_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int octeon_gpio_probe(struct platform_device *pdev)
    {
    struct octeon_gpio *gpio;
    struct gpio_chip *chip;
    void __iomem *reg_base;
    let mut err: c_int = 0;
    gpio = devm_kzalloc(&pdev.dev, sizeof(*gpio), GFP_KERNEL);
    if (!gpio)
    return -ENOMEM;
    chip = &gpio.chip;
    reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(reg_base))
    return PTR_ERR(reg_base);
    gpio.register_base = (u64)reg_base;
    pdev.dev.platform_data = chip;
    chip.label = "octeon-gpio";
    chip.parent = &pdev.dev;
    chip.owner = THIS_MODULE;
    chip.base = 0;
    chip.can_sleep = false;
    chip.ngpio = 20;
    chip.direction_input = octeon_gpio_dir_in;
    chip.get = octeon_gpio_get;
    chip.direction_output = octeon_gpio_dir_out;
    chip.set = octeon_gpio_set;
    err = devm_gpiochip_add_data(&pdev.dev, chip, gpio);
    if (err)
    return err;
    dev_info(&pdev.dev, "OCTEON GPIO driver probed.\n");
    return 0;
    }
    static const struct of_device_id octeon_gpio_match[] = {
    {
    .compatible = "cavium,octeon-3860-gpio",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, octeon_gpio_match);
    static struct platform_driver octeon_gpio_driver = {
    .driver = {
    .name		= "octeon_gpio",
    .of_match_table = octeon_gpio_match,
    },
    .probe		= octeon_gpio_probe,
    };
    module_platform_driver(octeon_gpio_driver);
    MODULE_DESCRIPTION("Cavium Inc. OCTEON GPIO Driver");
    MODULE_AUTHOR("David Daney");
    MODULE_LICENSE("GPL");
