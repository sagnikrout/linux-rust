//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-crystalcove.c
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
// Intel Crystal Cove GPIO Driver
//
// Copyright (C) 2012, 2014 Intel Corporation. All rights reserved.
//
// Author: Yang, Bin <bin.yang@intel.com>
//

pub const CRYSTALCOVE_GPIO_NUM: c_int = 16;
pub const CRYSTALCOVE_VGPIO_NUM: c_int = 95;

pub const GPIO0IRQ: c_uint = 0x0b;
pub const GPIO1IRQ: c_uint = 0x0c;
pub const MGPIO0IRQS0: c_uint = 0x19;
pub const MGPIO1IRQS0: c_uint = 0x1a;
pub const MGPIO0IRQSX: c_uint = 0x1b;
pub const MGPIO1IRQSX: c_uint = 0x1c;
pub const GPIO0P0CTLO: c_uint = 0x2b;
pub const GPIO0P0CTLI: c_uint = 0x33;
pub const GPIO1P0CTLO: c_uint = 0x3b;
pub const GPIO1P0CTLI: c_uint = 0x43;
pub const GPIOPANELCTL: c_uint = 0x52;

    enum ctrl_register {
    CTRL_IN,
    CTRL_OUT,
    };
//
// struct crystalcove_gpio - Crystal Cove GPIO controller
// @buslock: for bus lock/sync and unlock.
// @chip: the abstract gpio_chip structure.
// @regmap: the regmap from the parent device.
// @update: pending IRQ setting update, to be written to the chip upon unlock.
// @intcnt_value: the Interrupt Detect value to be written.
// @set_irq_mask: true if the IRQ mask needs to be set, false to clear.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crystalcove_gpio {
    pub /: *mut *mut mutex buslock; / irq_bus_lock,
    pub chip: gpio_chip,
    pub regmap: *mut regmap,
    pub update: c_int,
    pub intcnt_value: c_int,
    pub set_irq_mask: bool,
}

#[no_mangle]
pub unsafe extern "C" fn to_reg(gpio: c_int, reg_type: enum ctrl_register) -> c_int {
    static inline int to_reg(int gpio, enum ctrl_register reg_type)
    {
    int reg;
    if (gpio >= CRYSTALCOVE_GPIO_NUM) {
//
// Virtual GPIO called from ACPI, for now we only support
// the panel ctl.
//
    switch (gpio) {
    case 0x5e:
    return GPIOPANELCTL;
    default:
    return -ENOTSUPP;
    }
    }
    if (reg_type == CTRL_IN) {
    if (gpio < 8)
    reg = GPIO0P0CTLI;
    else
    reg = GPIO1P0CTLI;
    } else {
    if (gpio < 8)
    reg = GPIO0P0CTLO;
    else
    reg = GPIO1P0CTLO;
    }
    return reg + gpio % 8;
    }
#[no_mangle]
unsafe extern "C" fn crystalcove_update_irq_mask(cg: *mut crystalcove_gpio, gpio: c_int) {
    static void crystalcove_update_irq_mask(struct crystalcove_gpio *cg, int gpio)
    {
    let mut mirqs0: u8 = gpio < 8 ? MGPIO0IRQS0 : MGPIO1IRQS0;
    let mut mask: c_int = BIT(gpio % 8);
    if (cg.set_irq_mask)
    regmap_update_bits(cg.regmap, mirqs0, mask, mask);
    else
    regmap_update_bits(cg.regmap, mirqs0, mask, 0);
    }
#[no_mangle]
unsafe extern "C" fn crystalcove_update_irq_ctrl(cg: *mut crystalcove_gpio, gpio: c_int) {
    static void crystalcove_update_irq_ctrl(struct crystalcove_gpio *cg, int gpio)
    {
    let mut reg: c_int = to_reg(gpio, CTRL_IN);
    regmap_update_bits(cg.regmap, reg, CTLI_INTCNT_BE, cg.intcnt_value);
    }
#[no_mangle]
unsafe extern "C" fn crystalcove_gpio_dir_in(chip: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int crystalcove_gpio_dir_in(struct gpio_chip *chip, unsigned int gpio)
    {
    struct crystalcove_gpio *cg = gpiochip_get_data(chip);
    let mut reg: c_int = to_reg(gpio, CTRL_OUT);
    if (reg < 0)
    return 0;
    return regmap_write(cg.regmap, reg, CTLO_INPUT_SET);
    }
#[no_mangle]
unsafe extern "C" fn crystalcove_gpio_dir_out(chip: *mut gpio_chip, gpio: c_uint, value: c_int) -> c_int {
    static int crystalcove_gpio_dir_out(struct gpio_chip *chip, unsigned int gpio, int value)
    {
    struct crystalcove_gpio *cg = gpiochip_get_data(chip);
    let mut reg: c_int = to_reg(gpio, CTRL_OUT);
    if (reg < 0)
    return 0;
    return regmap_write(cg.regmap, reg, CTLO_OUTPUT_SET | value);
    }
#[no_mangle]
unsafe extern "C" fn crystalcove_gpio_get(chip: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int crystalcove_gpio_get(struct gpio_chip *chip, unsigned int gpio)
    {
    struct crystalcove_gpio *cg = gpiochip_get_data(chip);
    unsigned int val;
    int ret, reg = to_reg(gpio, CTRL_IN);
    if (reg < 0)
    return 0;
    ret = regmap_read(cg.regmap, reg, &val);
    if (ret)
    return ret;
    return val & 0x1;
    }
#[no_mangle]
unsafe extern "C" fn crystalcove_gpio_set(chip: *mut gpio_chip, gpio: c_uint, value: c_int) -> c_int {
    static int crystalcove_gpio_set(struct gpio_chip *chip, unsigned int gpio, int value)
    {
    struct crystalcove_gpio *cg = gpiochip_get_data(chip);
    let mut reg: c_int = to_reg(gpio, CTRL_OUT);
    if (reg < 0)
    return 0;
    if (value)
    return regmap_update_bits(cg.regmap, reg, 1, 1);
    return regmap_update_bits(cg.regmap, reg, 1, 0);
    }
#[no_mangle]
unsafe extern "C" fn crystalcove_irq_type(data: *mut irq_data, type: c_uint) -> c_int {
    static int crystalcove_irq_type(struct irq_data *data, unsigned int type)
    {
    struct crystalcove_gpio *cg = gpiochip_get_data(irq_data_get_irq_chip_data(data));
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(data);
    if (hwirq >= CRYSTALCOVE_GPIO_NUM)
    return 0;
    switch (type) {
    case IRQ_TYPE_NONE:
    cg.intcnt_value = CTLI_INTCNT_DIS;
    break;
    case IRQ_TYPE_EDGE_BOTH:
    cg.intcnt_value = CTLI_INTCNT_BE;
    break;
    case IRQ_TYPE_EDGE_RISING:
    cg.intcnt_value = CTLI_INTCNT_PE;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    cg.intcnt_value = CTLI_INTCNT_NE;
    break;
    default:
    return -EINVAL;
    }
    cg.update |= UPDATE_IRQ_TYPE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crystalcove_bus_lock(data: *mut irq_data) {
    static void crystalcove_bus_lock(struct irq_data *data)
    {
    struct crystalcove_gpio *cg = gpiochip_get_data(irq_data_get_irq_chip_data(data));
    mutex_lock(&cg.buslock);
    }
#[no_mangle]
unsafe extern "C" fn crystalcove_bus_sync_unlock(data: *mut irq_data) {
    static void crystalcove_bus_sync_unlock(struct irq_data *data)
    {
    struct crystalcove_gpio *cg = gpiochip_get_data(irq_data_get_irq_chip_data(data));
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(data);
    if (cg.update & UPDATE_IRQ_TYPE)
    crystalcove_update_irq_ctrl(cg, hwirq);
    if (cg.update & UPDATE_IRQ_MASK)
    crystalcove_update_irq_mask(cg, hwirq);
    cg.update = 0;
    mutex_unlock(&cg.buslock);
    }
#[no_mangle]
unsafe extern "C" fn crystalcove_irq_unmask(data: *mut irq_data) {
    static void crystalcove_irq_unmask(struct irq_data *data)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(data);
    struct crystalcove_gpio *cg = gpiochip_get_data(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(data);
    if (hwirq >= CRYSTALCOVE_GPIO_NUM)
    return;
    gpiochip_enable_irq(gc, hwirq);
    cg.set_irq_mask = false;
    cg.update |= UPDATE_IRQ_MASK;
    }
#[no_mangle]
unsafe extern "C" fn crystalcove_irq_mask(data: *mut irq_data) {
    static void crystalcove_irq_mask(struct irq_data *data)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(data);
    struct crystalcove_gpio *cg = gpiochip_get_data(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(data);
    if (hwirq >= CRYSTALCOVE_GPIO_NUM)
    return;
    cg.set_irq_mask = true;
    cg.update |= UPDATE_IRQ_MASK;
    gpiochip_disable_irq(gc, hwirq);
    }
    static const struct irq_chip crystalcove_irqchip = {
    .name			= "Crystal Cove",
    .irq_mask		= crystalcove_irq_mask,
    .irq_unmask		= crystalcove_irq_unmask,
    .irq_set_type		= crystalcove_irq_type,
    .irq_bus_lock		= crystalcove_bus_lock,
    .irq_bus_sync_unlock	= crystalcove_bus_sync_unlock,
    .flags			= IRQCHIP_SKIP_SET_WAKE | IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn crystalcove_gpio_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t crystalcove_gpio_irq_handler(int irq, void *data)
    {
    struct crystalcove_gpio *cg = data;
    unsigned long pending;
    unsigned int p0, p1;
    int gpio;
    unsigned int virq;
    if (regmap_read(cg.regmap, GPIO0IRQ, &p0) ||
    regmap_read(cg.regmap, GPIO1IRQ, &p1))
    return IRQ_NONE;
    regmap_write(cg.regmap, GPIO0IRQ, p0);
    regmap_write(cg.regmap, GPIO1IRQ, p1);
    pending = p0 | p1 << 8;
    for_each_set_bit(gpio, &pending, CRYSTALCOVE_GPIO_NUM) {
    virq = irq_find_mapping(cg.chip.irq.domain, gpio);
    handle_nested_irq(virq);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn crystalcove_gpio_dbg_show(s: *mut seq_file, chip: *mut gpio_chip) {
    static void crystalcove_gpio_dbg_show(struct seq_file *s, struct gpio_chip *chip)
    {
    struct crystalcove_gpio *cg = gpiochip_get_data(chip);
    int gpio, offset;
    unsigned int ctlo, ctli, mirqs0, mirqsx, irq;
    for (gpio = 0; gpio < CRYSTALCOVE_GPIO_NUM; gpio++) {
    regmap_read(cg.regmap, to_reg(gpio, CTRL_OUT), &ctlo);
    regmap_read(cg.regmap, to_reg(gpio, CTRL_IN), &ctli);
    regmap_read(cg.regmap, gpio < 8 ? MGPIO0IRQS0 : MGPIO1IRQS0,
    &mirqs0);
    regmap_read(cg.regmap, gpio < 8 ? MGPIO0IRQSX : MGPIO1IRQSX,
    &mirqsx);
    regmap_read(cg.regmap, gpio < 8 ? GPIO0IRQ : GPIO1IRQ,
    &irq);
    offset = gpio % 8;
    seq_printf(s, " gpio-%-2d %s %s %s %s ctlo=%2x,%s %s %s\n",
    gpio, ctlo & CTLO_DIR_OUT ? "out" : "in ",
    str_hi_lo(ctli & 0x1),
    ctli & CTLI_INTCNT_NE ? "fall" : "    ",
    ctli & CTLI_INTCNT_PE ? "rise" : "    ",
    ctlo,
    mirqs0 & BIT(offset) ? "s0 mask  " : "s0 unmask",
    mirqsx & BIT(offset) ? "sx mask  " : "sx unmask",
    irq & BIT(offset) ? "pending" : "       ");
    }
    }
#[no_mangle]
unsafe extern "C" fn crystalcove_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int crystalcove_gpio_probe(struct platform_device *pdev)
    {
    let mut irq: c_int = platform_get_irq(pdev, 0);
    struct crystalcove_gpio *cg;
    int retval;
    struct device *dev = pdev.dev.parent;
    struct intel_soc_pmic *pmic = dev_get_drvdata(dev);
    struct gpio_irq_chip *girq;
    if (irq < 0)
    return irq;
    cg = devm_kzalloc(&pdev.dev, sizeof(*cg), GFP_KERNEL);
    if (!cg)
    return -ENOMEM;
    mutex_init(&cg.buslock);
    cg.chip.label = KBUILD_MODNAME;
    cg.chip.direction_input = crystalcove_gpio_dir_in;
    cg.chip.direction_output = crystalcove_gpio_dir_out;
    cg.chip.get = crystalcove_gpio_get;
    cg.chip.set = crystalcove_gpio_set;
    cg.chip.base = -1;
    cg.chip.ngpio = CRYSTALCOVE_VGPIO_NUM;
    cg.chip.can_sleep = true;
    cg.chip.parent = dev;
    cg.chip.dbg_show = crystalcove_gpio_dbg_show;
    cg.regmap = pmic.regmap;
    girq = &cg.chip.irq;
    gpio_irq_chip_set_chip(girq, &crystalcove_irqchip);
// This will let us handle the parent IRQ in the driver
    girq.parent_handler = core::ptr::null_mut();
    girq.num_parents = 0;
    girq.parents = core::ptr::null_mut();
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = handle_simple_irq;
    girq.threaded = true;
    retval = devm_request_threaded_irq(&pdev.dev, irq, core::ptr::null_mut(),
    crystalcove_gpio_irq_handler,
    IRQF_ONESHOT, KBUILD_MODNAME, cg);
    if (retval) {
    dev_warn(&pdev.dev, "request irq failed: %d\n", retval);
    return retval;
    }
    retval = devm_gpiochip_add_data(&pdev.dev, &cg.chip, cg);
    if (retval)
    return retval;
// Distuingish IRQ domain from others sharing (MFD) the same fwnode
    irq_domain_update_bus_token(cg.chip.irq.domain, DOMAIN_BUS_WIRED);
    return 0;
    }
    static struct platform_driver crystalcove_gpio_driver = {
    .probe = crystalcove_gpio_probe,
    .driver = {
    .name = "crystal_cove_gpio",
    },
    };
    module_platform_driver(crystalcove_gpio_driver);
    MODULE_AUTHOR("Yang, Bin <bin.yang@intel.com>");
    MODULE_DESCRIPTION("Intel Crystal Cove GPIO Driver");
    MODULE_LICENSE("GPL v2");
