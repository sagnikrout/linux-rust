//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-bcm6328.c
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
// Driver for BCM6328 memory-mapped LEDs, based on leds-syscon.c
//
// Copyright 2015 Álvaro Fernández Rojas <noltari@gmail.com>
// Copyright 2015 Jonas Gorski <jogo@openwrt.org>
//

pub const BCM6328_REG_INIT: c_uint = 0x00;
pub const BCM6328_REG_MODE_HI: c_uint = 0x04;
pub const BCM6328_REG_MODE_LO: c_uint = 0x08;
pub const BCM6328_REG_HWDIS: c_uint = 0x0c;
pub const BCM6328_REG_STROBE: c_uint = 0x10;
pub const BCM6328_REG_LNKACTSEL_HI: c_uint = 0x14;
pub const BCM6328_REG_LNKACTSEL_LO: c_uint = 0x18;
pub const BCM6328_REG_RBACK: c_uint = 0x1c;
pub const BCM6328_REG_SERMUX: c_uint = 0x20;
pub const BCM6328_LED_MAX_COUNT: c_int = 24;
pub const BCM6328_LED_DEF_DELAY: c_int = 500;
pub const BCM6328_LED_BLINK_DELAYS: c_int = 2;
pub const BCM6328_LED_BLINK_MS: c_int = 20;
pub const BCM6328_LED_BLINK_MASK: c_uint = 0x3f;
pub const BCM6328_LED_BLINK1_SHIFT: c_int = 0;

    BCM6328_LED_BLINK1_SHIFT)
pub const BCM6328_LED_BLINK2_SHIFT: c_int = 6;

    BCM6328_LED_BLINK2_SHIFT)

    BCM6328_SERIAL_LED_MUX | \
    BCM6328_SERIAL_LED_CLK_NPOL | \
    BCM6328_SERIAL_LED_DATA_PPOL | \
    BCM6328_SERIAL_LED_SHIFT_DIR)
pub const BCM6328_LED_MODE_MASK: c_int = 3;
pub const BCM6328_LED_MODE_ON: c_int = 0;
pub const BCM6328_LED_MODE_BLINK1: c_int = 1;
pub const BCM6328_LED_MODE_BLINK2: c_int = 2;
pub const BCM6328_LED_MODE_OFF: c_int = 3;

//
// struct bcm6328_led - state container for bcm6328 based LEDs
// @cdev: LED class device for this LED
// @mem: memory resource
// @lock: memory lock
// @pin: LED pin number
// @blink_leds: blinking LEDs
// @blink_delay: blinking delay
// @active_low: LED is active low
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm6328_led {
    pub cdev: led_classdev,
    pub mem: *mut void __iomem,
    pub lock: *mut spinlock_t,
    pub pin: c_ulong,
    pub blink_leds: *mut c_ulong,
    pub blink_delay: *mut c_ulong,
    pub active_low: bool,
}

#[no_mangle]
unsafe extern "C" fn bcm6328_led_write(reg: *mut void __iomem, data: c_ulong) {
    static void bcm6328_led_write(void __iomem *reg, unsigned long data)
    {

    iowrite32be(data, reg);

    writel(data, reg);

    }
#[no_mangle]
unsafe extern "C" fn bcm6328_led_read(reg: *mut void __iomem) -> c_ulong {
    static unsigned long bcm6328_led_read(void __iomem *reg)
    {

    return ioread32be(reg);

    return readl(reg);

    }
//
// LEDMode 64 bits / 24 LEDs
// bits [31:0] -> LEDs 8-23
// bits [47:32] -> LEDs 0-7
// bits [63:48] -> unused
//
#[no_mangle]
unsafe extern "C" fn bcm6328_pin2shift(pin: c_ulong) -> c_ulong {
    static unsigned long bcm6328_pin2shift(unsigned long pin)
    {
    if (pin < 8)
    return pin + 16; /* LEDs 0-7 (bits 47:32) */
    else
    return pin - 8; /* LEDs 8-23 (bits 31:0) */
    }
#[no_mangle]
unsafe extern "C" fn bcm6328_led_mode(led: *mut bcm6328_led, value: c_ulong) {
    static void bcm6328_led_mode(struct bcm6328_led *led, unsigned long value)
    {
    void __iomem *mode;
    unsigned long val, shift;
    shift = bcm6328_pin2shift(led.pin);
    if (shift >= 16)
    mode = led.mem + BCM6328_REG_MODE_HI;
    else
    mode = led.mem + BCM6328_REG_MODE_LO;
    val = bcm6328_led_read(mode);
    val &= ~(BCM6328_LED_MODE_MASK << BCM6328_LED_SHIFT(shift % 16));
    val |= (value << BCM6328_LED_SHIFT(shift % 16));
    bcm6328_led_write(mode, val);
    }
    static void bcm6328_led_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    struct bcm6328_led *led =
    container_of(led_cdev, struct bcm6328_led, cdev);
    unsigned long flags;
    spin_lock_irqsave(led.lock, flags);
// Remove LED from cached HW blinking intervals
    led.blink_leds[0] &= ~BIT(led.pin);
    led.blink_leds[1] &= ~BIT(led.pin);
// Set LED on/off
    if ((led.active_low && value == LED_OFF) ||
    (!led.active_low && value != LED_OFF))
    bcm6328_led_mode(led, BCM6328_LED_MODE_ON);
    else
    bcm6328_led_mode(led, BCM6328_LED_MODE_OFF);
    spin_unlock_irqrestore(led.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn bcm6328_blink_delay(delay: c_ulong) -> c_ulong {
    static unsigned long bcm6328_blink_delay(unsigned long delay)
    {
    unsigned long bcm6328_delay;
    bcm6328_delay = delay + BCM6328_LED_BLINK_MS / 2;
    bcm6328_delay = bcm6328_delay / BCM6328_LED_BLINK_MS;
    if (bcm6328_delay == 0)
    bcm6328_delay = 1;
    return bcm6328_delay;
    }
    static int bcm6328_blink_set(struct led_classdev *led_cdev,
    unsigned long *delay_on, unsigned long *delay_off)
    {
    struct bcm6328_led *led =
    container_of(led_cdev, struct bcm6328_led, cdev);
    unsigned long delay, flags;
    int rc;
    if (!*delay_on)
// delay_on = BCM6328_LED_DEF_DELAY;
    if (!*delay_off)
// delay_off = BCM6328_LED_DEF_DELAY;
    delay = bcm6328_blink_delay(*delay_on);
    if (delay != bcm6328_blink_delay(*delay_off)) {
    dev_dbg(led_cdev.dev,
    "fallback to soft blinking (delay_on != delay_off)\n");
    return -EINVAL;
    }
    if (delay > BCM6328_LED_BLINK_MASK) {
    dev_dbg(led_cdev.dev,
    "fallback to soft blinking (delay > %ums)\n",
    BCM6328_LED_BLINK_MASK * BCM6328_LED_BLINK_MS);
    return -EINVAL;
    }
    spin_lock_irqsave(led.lock, flags);
//
// Check if any of the two configurable HW blinking intervals is
// available:
// 1. No LEDs assigned to the HW blinking interval.
// 2. Only this LED is assigned to the HW blinking interval.
// 3. LEDs with the same delay assigned.
//
    if (led.blink_leds[0] == 0 ||
    led.blink_leds[0] == BIT(led.pin) ||
    led.blink_delay[0] == delay) {
    unsigned long val;
// Add LED to the first HW blinking interval cache
    led.blink_leds[0] |= BIT(led.pin);
// Remove LED from the second HW blinking interval cache
    led.blink_leds[1] &= ~BIT(led.pin);
// Cache first HW blinking interval delay
    led.blink_delay[0] = delay;
// Update the delay for the first HW blinking interval
    val = bcm6328_led_read(led.mem + BCM6328_REG_INIT);
    val &= ~BCM6328_LED_BLINK1_MASK;
    val |= (delay << BCM6328_LED_BLINK1_SHIFT);
    bcm6328_led_write(led.mem + BCM6328_REG_INIT, val);
// Set the LED to first HW blinking interval
    bcm6328_led_mode(led, BCM6328_LED_MODE_BLINK1);
    rc = 0;
    } else if (led.blink_leds[1] == 0 ||
    led.blink_leds[1] == BIT(led.pin) ||
    led.blink_delay[1] == delay) {
    unsigned long val;
// Remove LED from the first HW blinking interval
    led.blink_leds[0] &= ~BIT(led.pin);
// Add LED to the second HW blinking interval
    led.blink_leds[1] |= BIT(led.pin);
// Cache second HW blinking interval delay
    led.blink_delay[1] = delay;
// Update the delay for the second HW blinking interval
    val = bcm6328_led_read(led.mem + BCM6328_REG_INIT);
    val &= ~BCM6328_LED_BLINK2_MASK;
    val |= (delay << BCM6328_LED_BLINK2_SHIFT);
    bcm6328_led_write(led.mem + BCM6328_REG_INIT, val);
// Set the LED to second HW blinking interval
    bcm6328_led_mode(led, BCM6328_LED_MODE_BLINK2);
    rc = 0;
    } else {
    dev_dbg(led_cdev.dev,
    "fallback to soft blinking (delay already set)\n");
    rc = -EINVAL;
    }
    spin_unlock_irqrestore(led.lock, flags);
    return rc;
    }
    static int bcm6328_hwled(struct device *dev, struct device_node *nc, u32 reg,
    void __iomem *mem, spinlock_t *lock)
    {
    int i, cnt;
    unsigned long flags, val;
    spin_lock_irqsave(lock, flags);
    val = bcm6328_led_read(mem + BCM6328_REG_HWDIS);
    val &= ~BIT(reg);
    bcm6328_led_write(mem + BCM6328_REG_HWDIS, val);
    spin_unlock_irqrestore(lock, flags);
// Only LEDs 0-7 can be activity/link controlled
    if (reg >= 8)
    return 0;
    cnt = of_property_count_elems_of_size(nc, "brcm,link-signal-sources",
    sizeof(u32));
    for (i = 0; i < cnt; i++) {
    u32 sel;
    void __iomem *addr;
    if (reg < 4)
    addr = mem + BCM6328_REG_LNKACTSEL_LO;
    else
    addr = mem + BCM6328_REG_LNKACTSEL_HI;
    of_property_read_u32_index(nc, "brcm,link-signal-sources", i,
    &sel);
    if (reg / 4 != sel / 4) {
    dev_warn(dev, "invalid link signal source\n");
    continue;
    }
    spin_lock_irqsave(lock, flags);
    val = bcm6328_led_read(addr);
    val |= (BIT(reg % 4) << (((sel % 4) * 4) + 16));
    bcm6328_led_write(addr, val);
    spin_unlock_irqrestore(lock, flags);
    }
    cnt = of_property_count_elems_of_size(nc,
    "brcm,activity-signal-sources",
    sizeof(u32));
    for (i = 0; i < cnt; i++) {
    u32 sel;
    void __iomem *addr;
    if (reg < 4)
    addr = mem + BCM6328_REG_LNKACTSEL_LO;
    else
    addr = mem + BCM6328_REG_LNKACTSEL_HI;
    of_property_read_u32_index(nc, "brcm,activity-signal-sources",
    i, &sel);
    if (reg / 4 != sel / 4) {
    dev_warn(dev, "invalid activity signal source\n");
    continue;
    }
    spin_lock_irqsave(lock, flags);
    val = bcm6328_led_read(addr);
    val |= (BIT(reg % 4) << ((sel % 4) * 4));
    bcm6328_led_write(addr, val);
    spin_unlock_irqrestore(lock, flags);
    }
    return 0;
    }
    static int bcm6328_led(struct device *dev, struct device_node *nc, u32 reg,
    void __iomem *mem, spinlock_t *lock,
    unsigned long *blink_leds, unsigned long *blink_delay)
    {
    let mut init_data: led_init_data = {};
    struct bcm6328_led *led;
    enum led_default_state state;
    unsigned long val, shift;
    void __iomem *mode;
    int rc;
    led = devm_kzalloc(dev, sizeof(*led), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    led.pin = reg;
    led.mem = mem;
    led.lock = lock;
    led.blink_leds = blink_leds;
    led.blink_delay = blink_delay;
    if (of_property_read_bool(nc, "active-low"))
    led.active_low = true;
    init_data.fwnode = of_fwnode_handle(nc);
    state = led_init_default_state_get(init_data.fwnode);
    switch (state) {
    case LEDS_DEFSTATE_ON:
    led.cdev.brightness = LED_FULL;
    break;
    case LEDS_DEFSTATE_KEEP:
    shift = bcm6328_pin2shift(led.pin);
    if (shift >= 16)
    mode = mem + BCM6328_REG_MODE_HI;
    else
    mode = mem + BCM6328_REG_MODE_LO;
    val = bcm6328_led_read(mode) >> BCM6328_LED_SHIFT(shift % 16);
    val &= BCM6328_LED_MODE_MASK;
    if ((led.active_low && val == BCM6328_LED_MODE_OFF) ||
    (!led.active_low && val == BCM6328_LED_MODE_ON))
    led.cdev.brightness = LED_FULL;
    else
    led.cdev.brightness = LED_OFF;
    break;
    default:
    led.cdev.brightness = LED_OFF;
    }
    bcm6328_led_set(&led.cdev, led.cdev.brightness);
    led.cdev.brightness_set = bcm6328_led_set;
    led.cdev.blink_set = bcm6328_blink_set;
    rc = devm_led_classdev_register_ext(dev, &led.cdev, &init_data);
    if (rc < 0)
    return rc;
    dev_dbg(dev, "registered LED %s\n", led.cdev.name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm6328_leds_probe(pdev: *mut platform_device) -> c_int {
    static int bcm6328_leds_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev_of_node(&pdev.dev);
    void __iomem *mem;
    spinlock_t *lock; /* memory lock */
    unsigned long val, *blink_leds, *blink_delay;
    mem = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mem))
    return PTR_ERR(mem);
    lock = devm_kzalloc(dev, sizeof(*lock), GFP_KERNEL);
    if (!lock)
    return -ENOMEM;
    blink_leds = devm_kcalloc(dev, BCM6328_LED_BLINK_DELAYS,
    sizeof(*blink_leds), GFP_KERNEL);
    if (!blink_leds)
    return -ENOMEM;
    blink_delay = devm_kcalloc(dev, BCM6328_LED_BLINK_DELAYS,
    sizeof(*blink_delay), GFP_KERNEL);
    if (!blink_delay)
    return -ENOMEM;
    spin_lock_init(lock);
    bcm6328_led_write(mem + BCM6328_REG_HWDIS, ~0);
    bcm6328_led_write(mem + BCM6328_REG_LNKACTSEL_HI, 0);
    bcm6328_led_write(mem + BCM6328_REG_LNKACTSEL_LO, 0);
    val = bcm6328_led_read(mem + BCM6328_REG_INIT);
    val &= ~(BCM6328_INIT_MASK);
    if (of_property_read_bool(np, "brcm,serial-leds"))
    val |= BCM6328_SERIAL_LED_EN;
    if (of_property_read_bool(np, "brcm,serial-mux"))
    val |= BCM6328_SERIAL_LED_MUX;
    if (of_property_read_bool(np, "brcm,serial-clk-low"))
    val |= BCM6328_SERIAL_LED_CLK_NPOL;
    if (!of_property_read_bool(np, "brcm,serial-dat-low"))
    val |= BCM6328_SERIAL_LED_DATA_PPOL;
    if (!of_property_read_bool(np, "brcm,serial-shift-inv"))
    val |= BCM6328_SERIAL_LED_SHIFT_DIR;
    bcm6328_led_write(mem + BCM6328_REG_INIT, val);
    for_each_available_child_of_node_scoped(np, child) {
    int rc;
    u32 reg;
    if (of_property_read_u32(child, "reg", &reg))
    continue;
    if (reg >= BCM6328_LED_MAX_COUNT) {
    dev_err(dev, "invalid LED (%u >= %d)\n", reg,
    BCM6328_LED_MAX_COUNT);
    continue;
    }
    if (of_property_read_bool(child, "brcm,hardware-controlled"))
    rc = bcm6328_hwled(dev, child, reg, mem, lock);
    else
    rc = bcm6328_led(dev, child, reg, mem, lock,
    blink_leds, blink_delay);
    if (rc < 0)
    return rc;
    }
    return 0;
    }
    static const struct of_device_id bcm6328_leds_of_match[] = {
    { .compatible = "brcm,bcm6328-leds", },
    { },
    };
    MODULE_DEVICE_TABLE(of, bcm6328_leds_of_match);
    static struct platform_driver bcm6328_leds_driver = {
    .probe = bcm6328_leds_probe,
    .driver = {
    .name = "leds-bcm6328",
    .of_match_table = bcm6328_leds_of_match,
    },
    };
    module_platform_driver(bcm6328_leds_driver);
    MODULE_AUTHOR("Álvaro Fernández Rojas <noltari@gmail.com>");
    MODULE_AUTHOR("Jonas Gorski <jogo@openwrt.org>");
    MODULE_DESCRIPTION("LED driver for BCM6328 controllers");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:leds-bcm6328");
