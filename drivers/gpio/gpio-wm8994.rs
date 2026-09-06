//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-wm8994.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// gpiolib support for Wolfson WM8994
//
// Copyright 2009 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8994_gpio {
    pub wm8994: *mut wm8994,
    pub gpio_chip: gpio_chip,
}

#[no_mangle]
unsafe extern "C" fn wm8994_gpio_request(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int wm8994_gpio_request(struct gpio_chip *chip, unsigned offset)
    {
    struct wm8994_gpio *wm8994_gpio = gpiochip_get_data(chip);
    struct wm8994 *wm8994 = wm8994_gpio.wm8994;
    switch (wm8994.type) {
    case WM8958:
    switch (offset) {
    case 1:
    case 2:
    case 3:
    case 4:
    case 6:
    return -EINVAL;
    }
    break;
    default:
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wm8994_gpio_direction_in(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int wm8994_gpio_direction_in(struct gpio_chip *chip, unsigned offset)
    {
    struct wm8994_gpio *wm8994_gpio = gpiochip_get_data(chip);
    struct wm8994 *wm8994 = wm8994_gpio.wm8994;
    return wm8994_set_bits(wm8994, WM8994_GPIO_1 + offset,
    WM8994_GPN_DIR, WM8994_GPN_DIR);
    }
#[no_mangle]
unsafe extern "C" fn wm8994_gpio_get(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int wm8994_gpio_get(struct gpio_chip *chip, unsigned offset)
    {
    struct wm8994_gpio *wm8994_gpio = gpiochip_get_data(chip);
    struct wm8994 *wm8994 = wm8994_gpio.wm8994;
    int ret;
    ret = wm8994_reg_read(wm8994, WM8994_GPIO_1 + offset);
    if (ret < 0)
    return ret;
    if (ret & WM8994_GPN_LVL)
    return 1;
    else
    return 0;
    }
    static int wm8994_gpio_direction_out(struct gpio_chip *chip,
    unsigned offset, int value)
    {
    struct wm8994_gpio *wm8994_gpio = gpiochip_get_data(chip);
    struct wm8994 *wm8994 = wm8994_gpio.wm8994;
    if (value)
    value = WM8994_GPN_LVL;
    return wm8994_set_bits(wm8994, WM8994_GPIO_1 + offset,
    WM8994_GPN_DIR | WM8994_GPN_LVL, value);
    }
    static int wm8994_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct wm8994_gpio *wm8994_gpio = gpiochip_get_data(chip);
    struct wm8994 *wm8994 = wm8994_gpio.wm8994;
    if (value)
    value = WM8994_GPN_LVL;
    return wm8994_set_bits(wm8994, WM8994_GPIO_1 + offset, WM8994_GPN_LVL,
    value);
    }
    static int wm8994_gpio_set_config(struct gpio_chip *chip, unsigned int offset,
    unsigned long config)
    {
    struct wm8994_gpio *wm8994_gpio = gpiochip_get_data(chip);
    struct wm8994 *wm8994 = wm8994_gpio.wm8994;
    switch (pinconf_to_config_param(config)) {
    case PIN_CONFIG_DRIVE_OPEN_DRAIN:
    return wm8994_set_bits(wm8994, WM8994_GPIO_1 + offset,
    WM8994_GPN_OP_CFG_MASK,
    WM8994_GPN_OP_CFG);
    case PIN_CONFIG_DRIVE_PUSH_PULL:
    return wm8994_set_bits(wm8994, WM8994_GPIO_1 + offset,
    WM8994_GPN_OP_CFG_MASK, 0);
    default:
    break;
    }
    return -ENOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn wm8994_gpio_to_irq(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int wm8994_gpio_to_irq(struct gpio_chip *chip, unsigned offset)
    {
    struct wm8994_gpio *wm8994_gpio = gpiochip_get_data(chip);
    struct wm8994 *wm8994 = wm8994_gpio.wm8994;
    return regmap_irq_get_virq(wm8994.irq_data, offset);
    }

    static const char *wm8994_gpio_fn(u16 fn)
    {
    switch (fn) {
    case WM8994_GP_FN_PIN_SPECIFIC:
    return "pin-specific";
    case WM8994_GP_FN_GPIO:
    return "GPIO";
    case WM8994_GP_FN_SDOUT:
    return "SDOUT";
    case WM8994_GP_FN_IRQ:
    return "IRQ";
    case WM8994_GP_FN_TEMPERATURE:
    return "Temperature";
    case WM8994_GP_FN_MICBIAS1_DET:
    return "MICBIAS1 detect";
    case WM8994_GP_FN_MICBIAS1_SHORT:
    return "MICBIAS1 short";
    case WM8994_GP_FN_MICBIAS2_DET:
    return "MICBIAS2 detect";
    case WM8994_GP_FN_MICBIAS2_SHORT:
    return "MICBIAS2 short";
    case WM8994_GP_FN_FLL1_LOCK:
    return "FLL1 lock";
    case WM8994_GP_FN_FLL2_LOCK:
    return "FLL2 lock";
    case WM8994_GP_FN_SRC1_LOCK:
    return "SRC1 lock";
    case WM8994_GP_FN_SRC2_LOCK:
    return "SRC2 lock";
    case WM8994_GP_FN_DRC1_ACT:
    return "DRC1 activity";
    case WM8994_GP_FN_DRC2_ACT:
    return "DRC2 activity";
    case WM8994_GP_FN_DRC3_ACT:
    return "DRC3 activity";
    case WM8994_GP_FN_WSEQ_STATUS:
    return "Write sequencer";
    case WM8994_GP_FN_FIFO_ERROR:
    return "FIFO error";
    case WM8994_GP_FN_OPCLK:
    return "OPCLK";
    case WM8994_GP_FN_THW:
    return "Thermal warning";
    case WM8994_GP_FN_DCS_DONE:
    return "DC servo";
    case WM8994_GP_FN_FLL1_OUT:
    return "FLL1 output";
    case WM8994_GP_FN_FLL2_OUT:
    return "FLL1 output";
    default:
    return "Unknown";
    }
    }
#[no_mangle]
unsafe extern "C" fn wm8994_gpio_dbg_show(s: *mut seq_file, chip: *mut gpio_chip) {
    static void wm8994_gpio_dbg_show(struct seq_file *s, struct gpio_chip *chip)
    {
    struct wm8994_gpio *wm8994_gpio = gpiochip_get_data(chip);
    struct wm8994 *wm8994 = wm8994_gpio.wm8994;
    int i;
    for (i = 0; i < chip.ngpio; i++) {
    int reg;
// We report the GPIO even if it's not requested since
// we're also reporting things like alternate
// functions which apply even when the GPIO is not in
// use as a GPIO.
//
    char *label __free(kfree) = gpiochip_dup_line_label(chip, i);
    if (IS_ERR(label)) {
    dev_err(wm8994.dev, "Failed to duplicate label\n");
    continue;
    }
    seq_printf(s, " gpio-%-3d (%-20.20s) ", i,
    label ?: "Unrequested");
    reg = wm8994_reg_read(wm8994, WM8994_GPIO_1 + i);
    if (reg < 0) {
    dev_err(wm8994.dev,
    "GPIO control %d read failed: %d\n", i, reg);
    seq_printf(s, "\n");
    continue;
    }
    if (reg & WM8994_GPN_DIR)
    seq_printf(s, "in ");
    else
    seq_printf(s, "out ");
    if (reg & WM8994_GPN_PU)
    seq_printf(s, "pull up ");
    if (reg & WM8994_GPN_PD)
    seq_printf(s, "pull down ");
    if (reg & WM8994_GPN_POL)
    seq_printf(s, "inverted ");
    else
    seq_printf(s, "noninverted ");
    if (reg & WM8994_GPN_OP_CFG)
    seq_printf(s, "open drain ");
    else
    seq_printf(s, "push-pull ");
    seq_printf(s, "%s (%x)\n",
    wm8994_gpio_fn(reg & WM8994_GPN_FN_MASK), reg);
    }
    }

    static const struct gpio_chip template_chip = {
    .label			= "wm8994",
    .owner			= THIS_MODULE,
    .request		= wm8994_gpio_request,
    .direction_input	= wm8994_gpio_direction_in,
    .get			= wm8994_gpio_get,
    .direction_output	= wm8994_gpio_direction_out,
    .set			= wm8994_gpio_set,
    .set_config		= wm8994_gpio_set_config,
    .to_irq			= wm8994_gpio_to_irq,
    .dbg_show		= wm8994_gpio_dbg_show,
    .can_sleep		= true,
    };
#[no_mangle]
unsafe extern "C" fn wm8994_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int wm8994_gpio_probe(struct platform_device *pdev)
    {
    struct wm8994 *wm8994 = dev_get_drvdata(pdev.dev.parent);
    struct wm8994_pdata *pdata = dev_get_platdata(wm8994.dev);
    struct wm8994_gpio *wm8994_gpio;
    wm8994_gpio = devm_kzalloc(&pdev.dev, sizeof(*wm8994_gpio),
    GFP_KERNEL);
    if (wm8994_gpio == core::ptr::null_mut())
    return -ENOMEM;
    wm8994_gpio.wm8994 = wm8994;
    wm8994_gpio.gpio_chip = template_chip;
    wm8994_gpio.gpio_chip.ngpio = WM8994_GPIO_MAX;
    wm8994_gpio.gpio_chip.parent = &pdev.dev;
    if (pdata && pdata.gpio_base)
    wm8994_gpio.gpio_chip.base = pdata.gpio_base;
    else
    wm8994_gpio.gpio_chip.base = -1;
    return devm_gpiochip_add_data(&pdev.dev, &wm8994_gpio.gpio_chip, wm8994_gpio);
    }
    static struct platform_driver wm8994_gpio_driver = {
    .driver.name	= "wm8994-gpio",
    .probe		= wm8994_gpio_probe,
    };
#[no_mangle]
unsafe extern "C" fn wm8994_gpio_init() -> int __init {
    static int __init wm8994_gpio_init(void)
    {
    return platform_driver_register(&wm8994_gpio_driver);
    }
    subsys_initcall(wm8994_gpio_init);
#[no_mangle]
unsafe extern "C" fn wm8994_gpio_exit() -> void __exit {
    static void __exit wm8994_gpio_exit(void)
    {
    platform_driver_unregister(&wm8994_gpio_driver);
    }
    module_exit(wm8994_gpio_exit);
    MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
    MODULE_DESCRIPTION("GPIO interface for WM8994");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:wm8994-gpio");
