//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-macsmc.c
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Apple SMC GPIO driver
// Copyright The Asahi Linux Contributors
//
// This driver implements basic SMC PMU GPIO support that can read inputs
// and write outputs. Mode changes and IRQ config are not yet implemented.
//

pub const MAX_GPIO: c_int = 64;
//
// Commands 0-6 are, presumably, the intended API.
// Command 0xff lets you get/set the pin configuration in detail directly,
// but the bit meanings seem not to be stable between devices/PMU hardware
// versions.
//
// We're going to try to make do with the low commands for now.
// We don't implement pin mode changes at this time.
//

pub const MODE_INPUT: c_int = 0;
pub const MODE_OUTPUT: c_int = 1;
pub const MODE_VALUE_0: c_int = 0;
pub const MODE_VALUE_1: c_int = 2;
pub const IRQ_MODE_HIGH: c_int = 0;
pub const IRQ_MODE_LOW: c_int = 1;
pub const IRQ_MODE_RISING: c_int = 2;
pub const IRQ_MODE_FALLING: c_int = 3;
pub const IRQ_MODE_BOTH: c_int = 4;

//
// Output modes seem to differ depending on the PMU in use... ?
// j274 / M1 (Sera PMU):
// 0 = input
// 1 = output
// 2 = open drain
// 3 = disable
// j314 / M1Pro (Maverick PMU):
// 0 = input
// 1 = open drain
// 2 = output
// 3 = ?
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsmc_gpio {
    pub dev: *mut device,
    pub smc: *mut apple_smc,
    pub gc: gpio_chip,
    pub first_index: c_int,
}

#[no_mangle]
unsafe extern "C" fn macsmc_gpio_nr(key: smc_key) -> c_int {
    static int macsmc_gpio_nr(smc_key key)
    {
    let mut low: c_int = hex_to_bin(key & 0xff);
    let mut high: c_int = hex_to_bin((key >> 8) & 0xff);
    if (low < 0 || high < 0)
    return -1;
    return low | (high << 4);
    }
#[no_mangle]
unsafe extern "C" fn macsmc_gpio_key(offset: c_uint) -> c_int {
    static int macsmc_gpio_key(unsigned int offset)
    {
    return _SMC_KEY("gP\0\0") | hex_asc_hi(offset) << 8 | hex_asc_lo(offset);
    }
#[no_mangle]
unsafe extern "C" fn macsmc_gpio_find_first_gpio_index(smcgp: *mut macsmc_gpio) -> c_int {
    static int macsmc_gpio_find_first_gpio_index(struct macsmc_gpio *smcgp)
    {
    struct apple_smc *smc = smcgp.smc;
    let mut key: smc_key = macsmc_gpio_key(0);
    smc_key first_key, last_key;
    int start, count, ret;
// Return early if the key is out of bounds
    ret = apple_smc_get_key_by_index(smc, 0, &first_key);
    if (ret)
    return ret;
    if (key <= first_key)
    return -ENODEV;
    ret = apple_smc_get_key_by_index(smc, smc.key_count - 1, &last_key);
    if (ret)
    return ret;
    if (key > last_key)
    return -ENODEV;
// Binary search to find index of first SMC key bigger or equal to key
    start = 0;
    count = smc.key_count;
    while (count > 1) {
    smc_key pkey;
    let mut pivot: c_int = start + ((count - 1) >> 1);
    ret = apple_smc_get_key_by_index(smc, pivot, &pkey);
    if (ret < 0)
    return ret;
    if (pkey == key)
    return pivot;
    pivot++;
    if (pkey < key) {
    count -= pivot - start;
    start = pivot;
    } else {
    count = pivot - start;
    }
    }
    return start;
    }
#[no_mangle]
unsafe extern "C" fn macsmc_gpio_get_direction(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int macsmc_gpio_get_direction(struct gpio_chip *gc, unsigned int offset)
    {
    struct macsmc_gpio *smcgp = gpiochip_get_data(gc);
    let mut key: smc_key = macsmc_gpio_key(offset);
    u32 val;
    int ret;
// First try reading the explicit pin mode register
    ret = apple_smc_rw_u32(smcgp.smc, key, CMD_PINMODE, &val);
    if (!ret)
    return (val & MODE_OUTPUT) ? GPIO_LINE_DIRECTION_OUT : GPIO_LINE_DIRECTION_IN;
//
// Less common IRQ configs cause CMD_PINMODE to fail, and so does open drain mode.
// Fall back to reading IRQ mode, which will only succeed for inputs.
//
    ret = apple_smc_rw_u32(smcgp.smc, key, CMD_IRQ_MODE, &val);
    return ret ? GPIO_LINE_DIRECTION_OUT : GPIO_LINE_DIRECTION_IN;
    }
#[no_mangle]
unsafe extern "C" fn macsmc_gpio_get(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int macsmc_gpio_get(struct gpio_chip *gc, unsigned int offset)
    {
    struct macsmc_gpio *smcgp = gpiochip_get_data(gc);
    let mut key: smc_key = macsmc_gpio_key(offset);
    u32 cmd, val;
    int ret;
    ret = macsmc_gpio_get_direction(gc, offset);
    if (ret < 0)
    return ret;
    if (ret == GPIO_LINE_DIRECTION_OUT)
    cmd = CMD_OUTPUT;
    else
    cmd = CMD_INPUT;
    ret = apple_smc_rw_u32(smcgp.smc, key, cmd, &val);
    if (ret < 0)
    return ret;
    return val ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn macsmc_gpio_set(gc: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int macsmc_gpio_set(struct gpio_chip *gc, unsigned int offset, int value)
    {
    struct macsmc_gpio *smcgp = gpiochip_get_data(gc);
    let mut key: smc_key = macsmc_gpio_key(offset);
    int ret;
    value |= CMD_OUTPUT;
    ret = apple_smc_write_u32(smcgp.smc, key, CMD_OUTPUT | value);
    if (ret < 0)
    dev_err(smcgp.dev, "GPIO set failed %p4ch = 0x%x\n",
    &key, value);
    return ret;
    }
    static int macsmc_gpio_init_valid_mask(struct gpio_chip *gc,
    unsigned long *valid_mask, unsigned int ngpios)
    {
    struct macsmc_gpio *smcgp = gpiochip_get_data(gc);
    int count;
    int i;
    count = min(smcgp.smc.key_count, MAX_GPIO);
    bitmap_zero(valid_mask, ngpios);
    for (i = 0; i < count; i++) {
    int ret, gpio_nr;
    smc_key key;
    ret = apple_smc_get_key_by_index(smcgp.smc, smcgp.first_index + i, &key);
    if (ret < 0)
    return ret;
    if (key > SMC_KEY(gPff))
    break;
    gpio_nr = macsmc_gpio_nr(key);
    if (gpio_nr < 0 || gpio_nr > MAX_GPIO) {
    dev_err(smcgp.dev, "Bad GPIO key %p4ch\n", &key);
    continue;
    }
    set_bit(gpio_nr, valid_mask);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn macsmc_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int macsmc_gpio_probe(struct platform_device *pdev)
    {
    struct macsmc_gpio *smcgp;
    struct apple_smc *smc = dev_get_drvdata(pdev.dev.parent);
    smc_key key;
    int ret;
    smcgp = devm_kzalloc(&pdev.dev, sizeof(*smcgp), GFP_KERNEL);
    if (!smcgp)
    return -ENOMEM;
    smcgp.dev = &pdev.dev;
    smcgp.smc = smc;
    smcgp.first_index = macsmc_gpio_find_first_gpio_index(smcgp);
    if (smcgp.first_index < 0)
    return smcgp.first_index;
    ret = apple_smc_get_key_by_index(smc, smcgp.first_index, &key);
    if (ret < 0)
    return ret;
    if (key > macsmc_gpio_key(MAX_GPIO - 1))
    return -ENODEV;
    dev_info(smcgp.dev, "First GPIO key: %p4ch\n", &key);
    smcgp.gc.label = "macsmc-pmu-gpio";
    smcgp.gc.owner = THIS_MODULE;
    smcgp.gc.get = macsmc_gpio_get;
    smcgp.gc.set = macsmc_gpio_set;
    smcgp.gc.get_direction = macsmc_gpio_get_direction;
    smcgp.gc.init_valid_mask = macsmc_gpio_init_valid_mask;
    smcgp.gc.can_sleep = true;
    smcgp.gc.ngpio = MAX_GPIO;
    smcgp.gc.base = -1;
    smcgp.gc.parent = &pdev.dev;
    return devm_gpiochip_add_data(&pdev.dev, &smcgp.gc, smcgp);
    }
    static const struct of_device_id macsmc_gpio_of_table[] = {
    { .compatible = "apple,smc-gpio", },
    {}
    };
    MODULE_DEVICE_TABLE(of, macsmc_gpio_of_table);
    static struct platform_driver macsmc_gpio_driver = {
    .driver = {
    .name = "macsmc-gpio",
    .of_match_table = macsmc_gpio_of_table,
    },
    .probe = macsmc_gpio_probe,
    };
    module_platform_driver(macsmc_gpio_driver);
    MODULE_AUTHOR("Hector Martin <marcan@marcan.st>");
    MODULE_LICENSE("Dual MIT/GPL");
    MODULE_DESCRIPTION("Apple SMC GPIO driver");
