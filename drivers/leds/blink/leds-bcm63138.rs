//! Automatically rewritten from C to Rust
//! Source: drivers/leds/blink/leds-bcm63138.c
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
// Copyright (C) 2021 Rafał Miłecki <rafal@milecki.pl>
//

pub const BCM63138_MAX_LEDS: c_int = 32;
pub const BCM63138_MAX_BRIGHTNESS: c_int = 9;

pub const BCM63138_GLB_CTRL: c_uint = 0x00;

pub const BCM63138_MASK: c_uint = 0x04;
pub const BCM63138_HW_LED_EN: c_uint = 0x08;
pub const BCM63138_SERIAL_LED_SHIFT_SEL: c_uint = 0x0c;
pub const BCM63138_FLASH_RATE_CTRL1: c_uint = 0x10;
pub const BCM63138_FLASH_RATE_CTRL2: c_uint = 0x14;
pub const BCM63138_FLASH_RATE_CTRL3: c_uint = 0x18;
pub const BCM63138_FLASH_RATE_CTRL4: c_uint = 0x1c;
pub const BCM63138_BRIGHT_CTRL1: c_uint = 0x20;
pub const BCM63138_BRIGHT_CTRL2: c_uint = 0x24;
pub const BCM63138_BRIGHT_CTRL3: c_uint = 0x28;
pub const BCM63138_BRIGHT_CTRL4: c_uint = 0x2c;
pub const BCM63138_POWER_LED_CFG: c_uint = 0x30;
pub const BCM63138_POWER_LUT_BASE0: c_uint = 0x34 /* -> b0 */;
pub const BCM63138_HW_POLARITY: c_uint = 0xb4;
pub const BCM63138_SW_DATA: c_uint = 0xb8;
pub const BCM63138_SW_POLARITY: c_uint = 0xbc;
pub const BCM63138_PARALLEL_LED_POLARITY: c_uint = 0xc0;
pub const BCM63138_SERIAL_LED_POLARITY: c_uint = 0xc4;
pub const BCM63138_HW_LED_STATUS: c_uint = 0xc8;
pub const BCM63138_FLASH_CTRL_STATUS: c_uint = 0xcc;
pub const BCM63138_FLASH_BRT_CTRL: c_uint = 0xd0;
pub const BCM63138_FLASH_P_LED_OUT_STATUS: c_uint = 0xd4;
pub const BCM63138_FLASH_S_LED_OUT_STATUS: c_uint = 0xd8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm63138_leds {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm63138_led {
    pub leds: *mut bcm63138_leds,
    pub cdev: led_classdev,
    pub pin: u32,
    pub active_low: bool,
}

//
// I/O access
//
    static void bcm63138_leds_write(struct bcm63138_leds *leds, unsigned int reg,
    u32 data)
    {
    writel(data, leds.base + reg);
    }
    static unsigned long bcm63138_leds_read(struct bcm63138_leds *leds,
    unsigned int reg)
    {
    return readl(leds.base + reg);
    }
    static void bcm63138_leds_update_bits(struct bcm63138_leds *leds,
    unsigned int reg, u32 mask, u32 val)
    {
    WARN_ON(val & ~mask);
    bcm63138_leds_write(leds, reg, (bcm63138_leds_read(leds, reg) & ~mask) | (val & mask));
    }
//
// Helpers
//
    static void bcm63138_leds_set_flash_rate(struct bcm63138_leds *leds,
    struct bcm63138_led *led,
    u8 value)
    {
    let mut reg_offset: c_int = (led.pin >> fls((BCM63138_LEDS_PER_REG - 1))) * 4;
    let mut shift: c_int = (led.pin & (BCM63138_LEDS_PER_REG - 1)) * BCM63138_LED_BITS;
    bcm63138_leds_update_bits(leds, BCM63138_FLASH_RATE_CTRL1 + reg_offset,
    BCM63138_LED_MASK << shift, value << shift);
    }
    static void bcm63138_leds_set_bright(struct bcm63138_leds *leds,
    struct bcm63138_led *led,
    u8 value)
    {
    let mut reg_offset: c_int = (led.pin >> fls((BCM63138_LEDS_PER_REG - 1))) * 4;
    let mut shift: c_int = (led.pin & (BCM63138_LEDS_PER_REG - 1)) * BCM63138_LED_BITS;
    bcm63138_leds_update_bits(leds, BCM63138_BRIGHT_CTRL1 + reg_offset,
    BCM63138_LED_MASK << shift, value << shift);
    }
    static void bcm63138_leds_enable_led(struct bcm63138_leds *leds,
    struct bcm63138_led *led,
    enum led_brightness value)
    {
    let mut bit: u32 = BIT(led.pin);
    bcm63138_leds_update_bits(leds, BCM63138_SW_DATA, bit, value ? bit : 0);
    }
//
// API callbacks
//
    static void bcm63138_leds_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    struct bcm63138_led *led = container_of(led_cdev, struct bcm63138_led, cdev);
    struct bcm63138_leds *leds = led.leds;
    guard(spinlock_irqsave)(&leds.lock);
    bcm63138_leds_enable_led(leds, led, value);
    if (!value)
    bcm63138_leds_set_flash_rate(leds, led, 0);
    else
    bcm63138_leds_set_bright(leds, led, value);
    }
    static int bcm63138_leds_blink_set(struct led_classdev *led_cdev,
    unsigned long *delay_on,
    unsigned long *delay_off)
    {
    struct bcm63138_led *led = container_of(led_cdev, struct bcm63138_led, cdev);
    struct bcm63138_leds *leds = led.leds;
    u8 value;
    if (!*delay_on && !*delay_off) {
// delay_on = 640;
// delay_off = 640;
    }
    if (*delay_on != *delay_off) {
    dev_dbg(led_cdev.dev, "Blinking at unequal delays is not supported\n");
    return -EINVAL;
    }
    switch (*delay_on) {
    case 1152 ... 1408: /* 1280 ms ± 10% */
    value = 0x7;
    break;
    case 576 ... 704: /* 640 ms ± 10% */
    value = 0x6;
    break;
    case 288 ... 352: /* 320 ms ± 10% */
    value = 0x5;
    break;
    case 126 ... 154: /* 140 ms ± 10% */
    value = 0x4;
    break;
    case 59 ... 72: /* 65 ms ± 10% */
    value = 0x3;
    break;
    default:
    dev_dbg(led_cdev.dev, "Blinking delay value %lu is unsupported\n",
// delay_on);
    return -EINVAL;
    }
    guard(spinlock_irqsave)(&leds.lock);
    bcm63138_leds_enable_led(leds, led, BCM63138_MAX_BRIGHTNESS);
    bcm63138_leds_set_flash_rate(leds, led, value);
    return 0;
    }
//
// LED driver
//
    static void bcm63138_leds_create_led(struct bcm63138_leds *leds,
    struct device_node *np)
    {
    struct led_init_data init_data = {
    .fwnode = of_fwnode_handle(np),
    };
    struct device *dev = leds.dev;
    struct bcm63138_led *led;
    struct pinctrl *pinctrl;
    u32 bit;
    int err;
    led = devm_kzalloc(dev, sizeof(*led), GFP_KERNEL);
    if (!led) {
    dev_err(dev, "Failed to alloc LED\n");
    return;
    }
    led.leds = leds;
    if (of_property_read_u32(np, "reg", &led.pin)) {
    dev_err(dev, "Missing \"reg\" property in %pOF\n", np);
    goto err_free;
    }
    if (led.pin >= BCM63138_MAX_LEDS) {
    dev_err(dev, "Invalid \"reg\" value %d\n", led.pin);
    goto err_free;
    }
    led.active_low = of_property_read_bool(np, "active-low");
    led.cdev.max_brightness = BCM63138_MAX_BRIGHTNESS;
    led.cdev.brightness_set = bcm63138_leds_brightness_set;
    led.cdev.blink_set = bcm63138_leds_blink_set;
    err = devm_led_classdev_register_ext(dev, &led.cdev, &init_data);
    if (err) {
    dev_err(dev, "Failed to register LED %pOF: %d\n", np, err);
    goto err_free;
    }
    pinctrl = devm_pinctrl_get_select_default(led.cdev.dev);
    if (IS_ERR(pinctrl) && PTR_ERR(pinctrl) != -ENODEV) {
    dev_warn(led.cdev.dev, "Failed to select %pOF pinctrl: %pe\n",
    np, pinctrl);
    }
    bit = BIT(led.pin);
    bcm63138_leds_update_bits(leds, BCM63138_PARALLEL_LED_POLARITY, bit,
    led.active_low ? 0 : bit);
    bcm63138_leds_update_bits(leds, BCM63138_HW_LED_EN, bit, 0);
    bcm63138_leds_set_flash_rate(leds, led, 0);
    bcm63138_leds_enable_led(leds, led, led.cdev.brightness);
    return;
    err_free:
    devm_kfree(dev, led);
    }
#[no_mangle]
unsafe extern "C" fn bcm63138_leds_probe(pdev: *mut platform_device) -> c_int {
    static int bcm63138_leds_probe(struct platform_device *pdev)
    {
    struct device_node *np = dev_of_node(&pdev.dev);
    struct device *dev = &pdev.dev;
    struct bcm63138_leds *leds;
    u32 shift_bits;
    leds = devm_kzalloc(dev, sizeof(*leds), GFP_KERNEL);
    if (!leds)
    return -ENOMEM;
    leds.dev = dev;
    leds.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(leds.base))
    return PTR_ERR(leds.base);
    spin_lock_init(&leds.lock);
// If this property is not present, we use boot defaults
    if (!of_property_read_u32(np, "brcm,serial-shift-bits", &shift_bits)) {
    bcm63138_leds_write(leds, BCM63138_SERIAL_LED_SHIFT_SEL,
    GENMASK(shift_bits - 1, 0));
    }
    bcm63138_leds_write(leds, BCM63138_GLB_CTRL,
    BCM63138_GLB_CTRL_SERIAL_LED_DATA_PPOL |
    BCM63138_GLB_CTRL_SERIAL_LED_EN_POL);
    bcm63138_leds_write(leds, BCM63138_HW_LED_EN, 0);
    bcm63138_leds_write(leds, BCM63138_SERIAL_LED_POLARITY, 0);
    bcm63138_leds_write(leds, BCM63138_PARALLEL_LED_POLARITY, 0);
    for_each_available_child_of_node_scoped(np, child) {
    bcm63138_leds_create_led(leds, child);
    }
    return 0;
    }
    static const struct of_device_id bcm63138_leds_of_match_table[] = {
    { .compatible = "brcm,bcm63138-leds", },
    { },
    };
    MODULE_DEVICE_TABLE(of, bcm63138_leds_of_match_table);
    static struct platform_driver bcm63138_leds_driver = {
    .probe = bcm63138_leds_probe,
    .driver = {
    .name = "leds-bcm63xxx",
    .of_match_table = bcm63138_leds_of_match_table,
    },
    };
    module_platform_driver(bcm63138_leds_driver);
    MODULE_AUTHOR("Rafał Miłecki");
    MODULE_DESCRIPTION("Broadcom BCM63138 SoC LED driver");
    MODULE_LICENSE("GPL");
