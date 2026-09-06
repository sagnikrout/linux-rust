//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-bcm6358.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for BCM6358 memory-mapped LEDs, based on leds-syscon.c
//
// Copyright 2015 Álvaro Fernández Rojas <noltari@gmail.com>
//

pub const BCM6358_REG_MODE: c_uint = 0x0;
pub const BCM6358_REG_CTRL: c_uint = 0x4;
pub const BCM6358_SLED_CLKDIV_MASK: c_int = 3;
pub const BCM6358_SLED_CLKDIV_1: c_int = 0;
pub const BCM6358_SLED_CLKDIV_2: c_int = 1;
pub const BCM6358_SLED_CLKDIV_4: c_int = 2;
pub const BCM6358_SLED_CLKDIV_8: c_int = 3;

pub const BCM6358_SLED_MAX_COUNT: c_int = 32;
pub const BCM6358_SLED_WAIT: c_int = 100;
//
// struct bcm6358_led - state container for bcm6358 based LEDs
// @cdev: LED class device for this LED
// @mem: memory resource
// @lock: memory lock
// @pin: LED pin number
// @active_low: LED is active low
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm6358_led {
    pub cdev: led_classdev,
    pub mem: *mut void __iomem,
    pub lock: *mut spinlock_t,
    pub pin: c_ulong,
    pub active_low: bool,
}

#[no_mangle]
unsafe extern "C" fn bcm6358_led_write(reg: *mut void __iomem, data: c_ulong) {
    static void bcm6358_led_write(void __iomem *reg, unsigned long data)
    {

    iowrite32be(data, reg);

    writel(data, reg);

    }
#[no_mangle]
unsafe extern "C" fn bcm6358_led_read(reg: *mut void __iomem) -> c_ulong {
    static unsigned long bcm6358_led_read(void __iomem *reg)
    {

    return ioread32be(reg);

    return readl(reg);

    }
#[no_mangle]
unsafe extern "C" fn bcm6358_led_busy(mem: *mut void __iomem) -> c_ulong {
    static unsigned long bcm6358_led_busy(void __iomem *mem)
    {
    unsigned long val;
    while ((val = bcm6358_led_read(mem + BCM6358_REG_CTRL)) &
    BCM6358_SLED_BUSY)
    udelay(BCM6358_SLED_WAIT);
    return val;
    }
    static void bcm6358_led_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    struct bcm6358_led *led =
    container_of(led_cdev, struct bcm6358_led, cdev);
    unsigned long flags, val;
    spin_lock_irqsave(led.lock, flags);
    bcm6358_led_busy(led.mem);
    val = bcm6358_led_read(led.mem + BCM6358_REG_MODE);
    if ((led.active_low && value == LED_OFF) ||
    (!led.active_low && value != LED_OFF))
    val |= BIT(led.pin);
    else
    val &= ~(BIT(led.pin));
    bcm6358_led_write(led.mem + BCM6358_REG_MODE, val);
    spin_unlock_irqrestore(led.lock, flags);
    }
    static int bcm6358_led(struct device *dev, struct device_node *nc, u32 reg,
    void __iomem *mem, spinlock_t *lock)
    {
    let mut init_data: led_init_data = {};
    struct bcm6358_led *led;
    enum led_default_state state;
    unsigned long val;
    int rc;
    led = devm_kzalloc(dev, sizeof(*led), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    led.pin = reg;
    led.mem = mem;
    led.lock = lock;
    if (of_property_read_bool(nc, "active-low"))
    led.active_low = true;
    init_data.fwnode = of_fwnode_handle(nc);
    state = led_init_default_state_get(init_data.fwnode);
    switch (state) {
    case LEDS_DEFSTATE_ON:
    led.cdev.brightness = LED_FULL;
    break;
    case LEDS_DEFSTATE_KEEP:
    val = bcm6358_led_read(led.mem + BCM6358_REG_MODE);
    val &= BIT(led.pin);
    if ((led.active_low && !val) || (!led.active_low && val))
    led.cdev.brightness = LED_FULL;
    else
    led.cdev.brightness = LED_OFF;
    break;
    default:
    led.cdev.brightness = LED_OFF;
    }
    bcm6358_led_set(&led.cdev, led.cdev.brightness);
    led.cdev.brightness_set = bcm6358_led_set;
    rc = devm_led_classdev_register_ext(dev, &led.cdev, &init_data);
    if (rc < 0)
    return rc;
    dev_dbg(dev, "registered LED %s\n", led.cdev.name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm6358_leds_probe(pdev: *mut platform_device) -> c_int {
    static int bcm6358_leds_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev_of_node(&pdev.dev);
    void __iomem *mem;
    spinlock_t *lock; /* memory lock */
    unsigned long val;
    u32 clk_div;
    mem = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mem))
    return PTR_ERR(mem);
    lock = devm_kzalloc(dev, sizeof(*lock), GFP_KERNEL);
    if (!lock)
    return -ENOMEM;
    spin_lock_init(lock);
    val = bcm6358_led_busy(mem);
    val &= ~(BCM6358_SLED_POLARITY | BCM6358_SLED_CLKDIV_MASK);
    if (of_property_read_bool(np, "brcm,clk-dat-low"))
    val |= BCM6358_SLED_POLARITY;
    of_property_read_u32(np, "brcm,clk-div", &clk_div);
    switch (clk_div) {
    case 8:
    val |= BCM6358_SLED_CLKDIV_8;
    break;
    case 4:
    val |= BCM6358_SLED_CLKDIV_4;
    break;
    case 2:
    val |= BCM6358_SLED_CLKDIV_2;
    break;
    default:
    val |= BCM6358_SLED_CLKDIV_1;
    break;
    }
    bcm6358_led_write(mem + BCM6358_REG_CTRL, val);
    for_each_available_child_of_node_scoped(np, child) {
    int rc;
    u32 reg;
    if (of_property_read_u32(child, "reg", &reg))
    continue;
    if (reg >= BCM6358_SLED_MAX_COUNT) {
    dev_err(dev, "invalid LED (%u >= %d)\n", reg,
    BCM6358_SLED_MAX_COUNT);
    continue;
    }
    rc = bcm6358_led(dev, child, reg, mem, lock);
    if (rc < 0)
    return rc;
    }
    return 0;
    }
    static const struct of_device_id bcm6358_leds_of_match[] = {
    { .compatible = "brcm,bcm6358-leds", },
    { },
    };
    MODULE_DEVICE_TABLE(of, bcm6358_leds_of_match);
    static struct platform_driver bcm6358_leds_driver = {
    .probe = bcm6358_leds_probe,
    .driver = {
    .name = "leds-bcm6358",
    .of_match_table = bcm6358_leds_of_match,
    },
    };
    module_platform_driver(bcm6358_leds_driver);
    MODULE_AUTHOR("Álvaro Fernández Rojas <noltari@gmail.com>");
    MODULE_DESCRIPTION("LED driver for BCM6358 controllers");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:leds-bcm6358");
