//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-latch.c
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
// GPIO latch driver
//
// Copyright (C) 2022 Sascha Hauer <s.hauer@pengutronix.de>
//
// This driver implements a GPIO (or better GPO as there is no input)
// multiplexer based on latches like this:
//
// CLK0 ----------------------.        ,--------.
// CLK1 -------------------.  `--------|>    #0 |
// |           |        |
// OUT0 ----------------+--|-----------|D0    Q0|-----|<
// OUT1 --------------+-|--|-----------|D1    Q1|-----|<
// OUT2 ------------+-|-|--|-----------|D2    Q2|-----|<
// OUT3 ----------+-|-|-|--|-----------|D3    Q3|-----|<
// OUT4 --------+-|-|-|-|--|-----------|D4    Q4|-----|<
// OUT5 ------+-|-|-|-|-|--|-----------|D5    Q5|-----|<
// OUT6 ----+-|-|-|-|-|-|--|-----------|D6    Q6|-----|<
// OUT7 --+-|-|-|-|-|-|-|--|-----------|D7    Q7|-----|<
// | | | | | | | |  |           `--------'
// | | | | | | | |  |
// | | | | | | | |  |           ,--------.
// | | | | | | | |  `-----------|>    #1 |
// | | | | | | | |              |        |
// | | | | | | | `--------------|D0    Q0|-----|<
// | | | | | | `----------------|D1    Q1|-----|<
// | | | | | `------------------|D2    Q2|-----|<
// | | | | `--------------------|D3    Q3|-----|<
// | | | `----------------------|D4    Q4|-----|<
// | | `------------------------|D5    Q5|-----|<
// | `--------------------------|D6    Q6|-----|<
// `----------------------------|D7    Q7|-----|<
// `--------'
//
// The above is just an example. The actual number of number of latches and
// the number of inputs per latch is derived from the number of GPIOs given
// in the corresponding device tree properties.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_latch_priv {
    pub gc: gpio_chip,
    pub clk_gpios: *mut gpio_descs,
    pub latched_gpios: *mut gpio_descs,
    pub n_latched_gpios: c_int,
    pub setup_duration_ns: c_uint,
    pub clock_duration_ns: c_uint,
    pub shadow: *mut c_ulong,
//
// Depending on whether any of the underlying GPIOs may sleep we either
// use a mutex or a spinlock to protect our shadow map.
//
    union {
    pub /: *mut *mut mutex mutex; / protects @shadow,
    pub /: *mut *mut spinlock_t spinlock; / protects @shadow,
}

    };
#[no_mangle]
unsafe extern "C" fn gpio_latch_get_direction(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int gpio_latch_get_direction(struct gpio_chip *gc, unsigned int offset)
    {
    return GPIO_LINE_DIRECTION_OUT;
    }
    static int gpio_latch_set_unlocked(struct gpio_latch_priv *priv,
    int (*set)(struct gpio_desc *desc, int value),
    unsigned int offset, bool val)
    {
    let mut latch: c_int = offset / priv.n_latched_gpios, i, ret;
    assign_bit(offset, priv.shadow, val);
    for (i = 0; i < priv.n_latched_gpios; i++) {
    ret = set(priv.latched_gpios.desc[i],
    test_bit(latch * priv.n_latched_gpios + i,
    priv.shadow));
    if (ret)
    return ret;
    }
    ndelay(priv.setup_duration_ns);
    set(priv.clk_gpios.desc[latch], 1);
    ndelay(priv.clock_duration_ns);
    set(priv.clk_gpios.desc[latch], 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gpio_latch_set(gc: *mut gpio_chip, offset: c_uint, val: c_int) -> c_int {
    static int gpio_latch_set(struct gpio_chip *gc, unsigned int offset, int val)
    {
    struct gpio_latch_priv *priv = gpiochip_get_data(gc);
    guard(spinlock_irqsave)(&priv.spinlock);
    return gpio_latch_set_unlocked(priv, gpiod_set_value, offset, val);
    }
#[no_mangle]
unsafe extern "C" fn gpio_latch_set_can_sleep(gc: *mut gpio_chip, offset: c_uint, val: c_int) -> c_int {
    static int gpio_latch_set_can_sleep(struct gpio_chip *gc, unsigned int offset, int val)
    {
    struct gpio_latch_priv *priv = gpiochip_get_data(gc);
    guard(mutex)(&priv.mutex);
    return gpio_latch_set_unlocked(priv, gpiod_set_value_cansleep, offset, val);
    }
#[no_mangle]
unsafe extern "C" fn gpio_latch_can_sleep(priv: *mut gpio_latch_priv, n_latches: c_uint) -> bool {
    static bool gpio_latch_can_sleep(struct gpio_latch_priv *priv, unsigned int n_latches)
    {
    int i;
    for (i = 0; i < n_latches; i++)
    if (gpiod_cansleep(priv.clk_gpios.desc[i]))
    return true;
    for (i = 0; i < priv.n_latched_gpios; i++)
    if (gpiod_cansleep(priv.latched_gpios.desc[i]))
    return true;
    return false;
    }
//
// Some value which is still acceptable to delay in atomic context.
// If we need to go higher we might have to switch to usleep_range(),
// but that cannot ne used in atomic context and the driver would have
// to be adjusted to support that.
//
pub const DURATION_NS_MAX: c_int = 5000;
#[no_mangle]
unsafe extern "C" fn gpio_latch_probe(pdev: *mut platform_device) -> c_int {
    static int gpio_latch_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct gpio_latch_priv *priv;
    unsigned int n_latches;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.clk_gpios = devm_gpiod_get_array(dev, "clk", GPIOD_OUT_LOW);
    if (IS_ERR(priv.clk_gpios))
    return PTR_ERR(priv.clk_gpios);
    priv.latched_gpios = devm_gpiod_get_array(dev, "latched", GPIOD_OUT_LOW);
    if (IS_ERR(priv.latched_gpios))
    return PTR_ERR(priv.latched_gpios);
    n_latches = priv.clk_gpios.ndescs;
    priv.n_latched_gpios = priv.latched_gpios.ndescs;
    priv.shadow = devm_bitmap_zalloc(dev, n_latches * priv.n_latched_gpios,
    GFP_KERNEL);
    if (!priv.shadow)
    return -ENOMEM;
    if (gpio_latch_can_sleep(priv, n_latches)) {
    priv.gc.can_sleep = true;
    priv.gc.set = gpio_latch_set_can_sleep;
    mutex_init(&priv.mutex);
    } else {
    priv.gc.can_sleep = false;
    priv.gc.set = gpio_latch_set;
    spin_lock_init(&priv.spinlock);
    }
    device_property_read_u32(dev, "setup-duration-ns",
    &priv.setup_duration_ns);
    if (priv.setup_duration_ns > DURATION_NS_MAX) {
    dev_warn(dev, "setup-duration-ns too high, limit to %d\n",
    DURATION_NS_MAX);
    priv.setup_duration_ns = DURATION_NS_MAX;
    }
    device_property_read_u32(dev, "clock-duration-ns",
    &priv.clock_duration_ns);
    if (priv.clock_duration_ns > DURATION_NS_MAX) {
    dev_warn(dev, "clock-duration-ns too high, limit to %d\n",
    DURATION_NS_MAX);
    priv.clock_duration_ns = DURATION_NS_MAX;
    }
    priv.gc.get_direction = gpio_latch_get_direction;
    priv.gc.ngpio = n_latches * priv.n_latched_gpios;
    priv.gc.owner = THIS_MODULE;
    priv.gc.base = -1;
    priv.gc.parent = dev;
    platform_set_drvdata(pdev, priv);
    return devm_gpiochip_add_data(dev, &priv.gc, priv);
    }
    static const struct of_device_id gpio_latch_ids[] = {
    {
    .compatible = "gpio-latch",
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, gpio_latch_ids);
    static struct platform_driver gpio_latch_driver = {
    .driver	= {
    .name		= "gpio-latch",
    .of_match_table	= gpio_latch_ids,
    },
    .probe	= gpio_latch_probe,
    };
    module_platform_driver(gpio_latch_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Sascha Hauer <s.hauer@pengutronix.de>");
    MODULE_DESCRIPTION("GPIO latch driver");
