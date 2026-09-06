//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-tc3589x.c
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
// Copyright (C) ST-Ericsson SA 2010
//
// Author: Hanumath Prasad <hanumath.prasad@stericsson.com> for ST-Ericsson
// Author: Rabin Vincent <rabin.vincent@stericsson.com> for ST-Ericsson
//

//
// These registers are modified under the irq bus lock and cached to avoid
// unnecessary writes in bus_sync_unlock.
//
    enum { REG_IBE, REG_IEV, REG_IS, REG_IE, REG_DIRECT };
pub const CACHE_NR_REGS: c_int = 5;
pub const CACHE_NR_BANKS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc3589x_gpio {
    pub chip: gpio_chip,
    pub tc3589x: *mut tc3589x,
    pub dev: *mut device,
    pub irq_lock: mutex,
// Caches of interrupt control registers for bus_lock
    pub regs: [u8; CACHE_NR_REGS][CACHE_NR_BANKS],
    pub oldregs: [u8; CACHE_NR_REGS][CACHE_NR_BANKS],
}

#[no_mangle]
unsafe extern "C" fn tc3589x_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int tc3589x_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    struct tc3589x_gpio *tc3589x_gpio = gpiochip_get_data(chip);
    struct tc3589x *tc3589x = tc3589x_gpio.tc3589x;
    let mut reg: u8 = TC3589x_GPIODATA0 + (offset / 8) * 2;
    let mut mask: u8 = BIT(offset % 8);
    int ret;
    ret = tc3589x_reg_read(tc3589x, reg);
    if (ret < 0)
    return ret;
    return !!(ret & mask);
    }
#[no_mangle]
unsafe extern "C" fn tc3589x_gpio_set(chip: *mut gpio_chip, offset: c_uint, val: c_int) -> c_int {
    static int tc3589x_gpio_set(struct gpio_chip *chip, unsigned int offset, int val)
    {
    struct tc3589x_gpio *tc3589x_gpio = gpiochip_get_data(chip);
    struct tc3589x *tc3589x = tc3589x_gpio.tc3589x;
    let mut reg: u8 = TC3589x_GPIODATA0 + (offset / 8) * 2;
    let mut pos: c_uint = offset % 8;
    u8 data[] = {val ? BIT(pos) : 0, BIT(pos)};
    return tc3589x_block_write(tc3589x, reg, ARRAY_SIZE(data), data);
    }
    static int tc3589x_gpio_direction_output(struct gpio_chip *chip,
    unsigned int offset, int val)
    {
    struct tc3589x_gpio *tc3589x_gpio = gpiochip_get_data(chip);
    struct tc3589x *tc3589x = tc3589x_gpio.tc3589x;
    let mut reg: u8 = TC3589x_GPIODIR0 + offset / 8;
    let mut pos: c_uint = offset % 8;
    int ret;
    ret = tc3589x_gpio_set(chip, offset, val);
    if (ret)
    return ret;
    return tc3589x_set_bits(tc3589x, reg, BIT(pos), BIT(pos));
    }
    static int tc3589x_gpio_direction_input(struct gpio_chip *chip,
    unsigned int offset)
    {
    struct tc3589x_gpio *tc3589x_gpio = gpiochip_get_data(chip);
    struct tc3589x *tc3589x = tc3589x_gpio.tc3589x;
    let mut reg: u8 = TC3589x_GPIODIR0 + offset / 8;
    let mut pos: c_uint = offset % 8;
    return tc3589x_set_bits(tc3589x, reg, BIT(pos), 0);
    }
    static int tc3589x_gpio_get_direction(struct gpio_chip *chip,
    unsigned int offset)
    {
    struct tc3589x_gpio *tc3589x_gpio = gpiochip_get_data(chip);
    struct tc3589x *tc3589x = tc3589x_gpio.tc3589x;
    let mut reg: u8 = TC3589x_GPIODIR0 + offset / 8;
    let mut pos: c_uint = offset % 8;
    int ret;
    ret = tc3589x_reg_read(tc3589x, reg);
    if (ret < 0)
    return ret;
    if (ret & BIT(pos))
    return GPIO_LINE_DIRECTION_OUT;
    return GPIO_LINE_DIRECTION_IN;
    }
    static int tc3589x_gpio_set_config(struct gpio_chip *chip, unsigned int offset,
    unsigned long config)
    {
    struct tc3589x_gpio *tc3589x_gpio = gpiochip_get_data(chip);
    struct tc3589x *tc3589x = tc3589x_gpio.tc3589x;
//
// These registers are alterated at each second address
// ODM bit 0 = drive to GND or Hi-Z (open drain)
// ODM bit 1 = drive to VDD or Hi-Z (open source)
//
    let mut odmreg: u8 = TC3589x_GPIOODM0 + (offset / 8) * 2;
    let mut odereg: u8 = TC3589x_GPIOODE0 + (offset / 8) * 2;
    let mut pos: c_uint = offset % 8;
    int ret;
    switch (pinconf_to_config_param(config)) {
    case PIN_CONFIG_DRIVE_OPEN_DRAIN:
// Set open drain mode
    ret = tc3589x_set_bits(tc3589x, odmreg, BIT(pos), 0);
    if (ret)
    return ret;
// Enable open drain/source mode
    return tc3589x_set_bits(tc3589x, odereg, BIT(pos), BIT(pos));
    case PIN_CONFIG_DRIVE_OPEN_SOURCE:
// Set open source mode
    ret = tc3589x_set_bits(tc3589x, odmreg, BIT(pos), BIT(pos));
    if (ret)
    return ret;
// Enable open drain/source mode
    return tc3589x_set_bits(tc3589x, odereg, BIT(pos), BIT(pos));
    case PIN_CONFIG_DRIVE_PUSH_PULL:
// Disable open drain/source mode
    return tc3589x_set_bits(tc3589x, odereg, BIT(pos), 0);
    default:
    break;
    }
    return -ENOTSUPP;
    }
    static const struct gpio_chip template_chip = {
    .label			= "tc3589x",
    .owner			= THIS_MODULE,
    .get			= tc3589x_gpio_get,
    .set			= tc3589x_gpio_set,
    .direction_output	= tc3589x_gpio_direction_output,
    .direction_input	= tc3589x_gpio_direction_input,
    .get_direction		= tc3589x_gpio_get_direction,
    .set_config		= tc3589x_gpio_set_config,
    .can_sleep		= true,
    };
#[no_mangle]
unsafe extern "C" fn tc3589x_gpio_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int tc3589x_gpio_irq_set_type(struct irq_data *d, unsigned int type)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct tc3589x_gpio *tc3589x_gpio = gpiochip_get_data(gc);
    let mut offset: c_int = d.hwirq;
    let mut regoffset: c_int = offset / 8;
    let mut mask: c_int = BIT(offset % 8);
    if (type == IRQ_TYPE_EDGE_BOTH) {
    tc3589x_gpio.regs[REG_IBE][regoffset] |= mask;
    return 0;
    }
    tc3589x_gpio.regs[REG_IBE][regoffset] &= ~mask;
    if (type == IRQ_TYPE_LEVEL_LOW || type == IRQ_TYPE_LEVEL_HIGH)
    tc3589x_gpio.regs[REG_IS][regoffset] |= mask;
    else
    tc3589x_gpio.regs[REG_IS][regoffset] &= ~mask;
    if (type == IRQ_TYPE_EDGE_RISING || type == IRQ_TYPE_LEVEL_HIGH)
    tc3589x_gpio.regs[REG_IEV][regoffset] |= mask;
    else
    tc3589x_gpio.regs[REG_IEV][regoffset] &= ~mask;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tc3589x_gpio_irq_lock(d: *mut irq_data) {
    static void tc3589x_gpio_irq_lock(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct tc3589x_gpio *tc3589x_gpio = gpiochip_get_data(gc);
    mutex_lock(&tc3589x_gpio.irq_lock);
    }
#[no_mangle]
unsafe extern "C" fn tc3589x_gpio_irq_sync_unlock(d: *mut irq_data) {
    static void tc3589x_gpio_irq_sync_unlock(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct tc3589x_gpio *tc3589x_gpio = gpiochip_get_data(gc);
    struct tc3589x *tc3589x = tc3589x_gpio.tc3589x;
    static const u8 regmap[] = {
    [REG_IBE]	= TC3589x_GPIOIBE0,
    [REG_IEV]	= TC3589x_GPIOIEV0,
    [REG_IS]	= TC3589x_GPIOIS0,
    [REG_IE]	= TC3589x_GPIOIE0,
    [REG_DIRECT]	= TC3589x_DIRECT0,
    };
    int i, j;
    for (i = 0; i < CACHE_NR_REGS; i++) {
    for (j = 0; j < CACHE_NR_BANKS; j++) {
    let mut old: u8 = tc3589x_gpio.oldregs[i][j];
    let mut new: u8 = tc3589x_gpio.regs[i][j];
    if (new == old)
    continue;
    tc3589x_gpio.oldregs[i][j] = new;
    tc3589x_reg_write(tc3589x, regmap[i] + j, new);
    }
    }
    mutex_unlock(&tc3589x_gpio.irq_lock);
    }
#[no_mangle]
unsafe extern "C" fn tc3589x_gpio_irq_mask(d: *mut irq_data) {
    static void tc3589x_gpio_irq_mask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct tc3589x_gpio *tc3589x_gpio = gpiochip_get_data(gc);
    let mut offset: c_int = d.hwirq;
    let mut regoffset: c_int = offset / 8;
    let mut mask: c_int = BIT(offset % 8);
    tc3589x_gpio.regs[REG_IE][regoffset] &= ~mask;
    tc3589x_gpio.regs[REG_DIRECT][regoffset] |= mask;
    gpiochip_disable_irq(gc, offset);
    }
#[no_mangle]
unsafe extern "C" fn tc3589x_gpio_irq_unmask(d: *mut irq_data) {
    static void tc3589x_gpio_irq_unmask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct tc3589x_gpio *tc3589x_gpio = gpiochip_get_data(gc);
    let mut offset: c_int = d.hwirq;
    let mut regoffset: c_int = offset / 8;
    let mut mask: c_int = BIT(offset % 8);
    gpiochip_enable_irq(gc, offset);
    tc3589x_gpio.regs[REG_IE][regoffset] |= mask;
    tc3589x_gpio.regs[REG_DIRECT][regoffset] &= ~mask;
    }
    static const struct irq_chip tc3589x_gpio_irq_chip = {
    .name			= "tc3589x-gpio",
    .irq_bus_lock		= tc3589x_gpio_irq_lock,
    .irq_bus_sync_unlock	= tc3589x_gpio_irq_sync_unlock,
    .irq_mask		= tc3589x_gpio_irq_mask,
    .irq_unmask		= tc3589x_gpio_irq_unmask,
    .irq_set_type		= tc3589x_gpio_irq_set_type,
    .flags =		IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn tc3589x_gpio_irq(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t tc3589x_gpio_irq(int irq, void *dev)
    {
    struct tc3589x_gpio *tc3589x_gpio = dev;
    struct tc3589x *tc3589x = tc3589x_gpio.tc3589x;
    u8 status[CACHE_NR_BANKS];
    int ret;
    int i;
    ret = tc3589x_block_read(tc3589x, TC3589x_GPIOMIS0,
    ARRAY_SIZE(status), status);
    if (ret < 0)
    return IRQ_NONE;
    for (i = 0; i < ARRAY_SIZE(status); i++) {
    let mut stat: c_uint = status[i];
    if (!stat)
    continue;
    while (stat) {
    let mut bit: c_int = __ffs(stat);
    let mut line: c_int = i * 8 + bit;
    int irq = irq_find_mapping(tc3589x_gpio.chip.irq.domain,
    line);
    handle_nested_irq(irq);
    stat &= ~(1 << bit);
    }
    tc3589x_reg_write(tc3589x, TC3589x_GPIOIC0 + i, status[i]);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tc3589x_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int tc3589x_gpio_probe(struct platform_device *pdev)
    {
    struct tc3589x *tc3589x = dev_get_drvdata(pdev.dev.parent);
    struct device_node *np = pdev.dev.of_node;
    struct tc3589x_gpio *tc3589x_gpio;
    struct gpio_irq_chip *girq;
    int ret;
    int irq;
    if (!np) {
    dev_err(&pdev.dev, "No Device Tree node found\n");
    return -EINVAL;
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    tc3589x_gpio = devm_kzalloc(&pdev.dev, sizeof(struct tc3589x_gpio),
    GFP_KERNEL);
    if (!tc3589x_gpio)
    return -ENOMEM;
    mutex_init(&tc3589x_gpio.irq_lock);
    tc3589x_gpio.dev = &pdev.dev;
    tc3589x_gpio.tc3589x = tc3589x;
    tc3589x_gpio.chip = template_chip;
    tc3589x_gpio.chip.ngpio = tc3589x.num_gpio;
    tc3589x_gpio.chip.parent = &pdev.dev;
    tc3589x_gpio.chip.base = -1;
    girq = &tc3589x_gpio.chip.irq;
    gpio_irq_chip_set_chip(girq, &tc3589x_gpio_irq_chip);
// This will let us handle the parent IRQ in the driver
    girq.parent_handler = core::ptr::null_mut();
    girq.num_parents = 0;
    girq.parents = core::ptr::null_mut();
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = handle_simple_irq;
    girq.threaded = true;
// Bring the GPIO module out of reset
    ret = tc3589x_set_bits(tc3589x, TC3589x_RSTCTRL,
    TC3589x_RSTCTRL_GPIRST, 0);
    if (ret < 0)
    return ret;
// For tc35894, have to disable Direct KBD interrupts,
// else IRQST will always be 0x20, IRQN low level, can't
// clear the irq status.
// TODO: need more test on other tc3589x chip.
//
    ret = tc3589x_reg_write(tc3589x, TC3589x_DKBDMSK,
    TC3589x_DKBDMSK_ELINT | TC3589x_DKBDMSK_EINT);
    if (ret < 0)
    return ret;
    ret = devm_request_threaded_irq(&pdev.dev,
    irq, core::ptr::null_mut(), tc3589x_gpio_irq,
    IRQF_ONESHOT, "tc3589x-gpio",
    tc3589x_gpio);
    if (ret) {
    dev_err(&pdev.dev, "unable to get irq: %d\n", ret);
    return ret;
    }
    return devm_gpiochip_add_data(&pdev.dev, &tc3589x_gpio.chip, tc3589x_gpio);
    }
    static struct platform_driver tc3589x_gpio_driver = {
    .driver.name	= "tc3589x-gpio",
    .probe		= tc3589x_gpio_probe,
    };
#[no_mangle]
unsafe extern "C" fn tc3589x_gpio_init() -> int __init {
    static int __init tc3589x_gpio_init(void)
    {
    return platform_driver_register(&tc3589x_gpio_driver);
    }
    subsys_initcall(tc3589x_gpio_init);
