//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-wcove.c
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
// Intel Whiskey Cove PMIC GPIO Driver
//
// This driver is written based on gpio-crystalcove.c
//
// Copyright (C) 2016 Intel Corporation. All rights reserved.
//

//
// Whiskey Cove PMIC has 13 physical GPIO pins divided into 3 banks:
// Bank 0: Pin  0 - 6
// Bank 1: Pin  7 - 10
// Bank 2: Pin 11 - 12
// Each pin has one output control register and one input control register.
//
pub const BANK0_NR_PINS: c_int = 7;
pub const BANK1_NR_PINS: c_int = 4;
pub const BANK2_NR_PINS: c_int = 2;

pub const WCOVE_VGPIO_NUM: c_int = 94;
// GPIO output control registers (one per pin): 0x4e44 - 0x4e50
pub const GPIO_OUT_CTRL_BASE: c_uint = 0x4e44;
// GPIO input control registers (one per pin): 0x4e51 - 0x4e5d
pub const GPIO_IN_CTRL_BASE: c_uint = 0x4e51;
//
// GPIO interrupts are organized in two groups:
// Group 0: Bank 0 pins (Pin 0 - 6)
// Group 1: Bank 1 and Bank 2 pins (Pin 7 - 12)
// Each group has two registers (one bit per pin): status and mask.
//
pub const GROUP0_NR_IRQS: c_int = 7;
pub const GROUP1_NR_IRQS: c_int = 6;
pub const IRQ_MASK_BASE: c_uint = 0x4e19;
pub const IRQ_STATUS_BASE: c_uint = 0x4e0b;

    enum ctrl_register {
    CTRL_IN,
    CTRL_OUT,
    IRQ_STATUS,
    IRQ_MASK,
    };
//
// struct wcove_gpio - Whiskey Cove GPIO controller
// @buslock: for bus lock/sync and unlock.
// @chip: the abstract gpio_chip structure.
// @dev: the gpio device
// @regmap: the regmap from the parent device.
// @regmap_irq_chip: the regmap of the gpio irq chip.
// @update: pending IRQ setting update, to be written to the chip upon unlock.
// @intcnt: the Interrupt Detect value to be written.
// @set_irq_mask: true if the IRQ mask needs to be set, false to clear.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcove_gpio {
    pub buslock: mutex,
    pub chip: gpio_chip,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub regmap_irq_chip: *mut regmap_irq_chip_data,
    pub update: c_int,
    pub intcnt: c_int,
    pub set_irq_mask: bool,
}

#[no_mangle]
pub unsafe extern "C" fn to_reg(gpio: c_int, type: enum ctrl_register) -> c_int {
    static inline int to_reg(int gpio, enum ctrl_register type)
    {
    let mut reg: c_uint = type == CTRL_IN ? GPIO_IN_CTRL_BASE : GPIO_OUT_CTRL_BASE;
    if (gpio >= WCOVE_GPIO_NUM)
    return -ENOTSUPP;
    return reg + gpio;
    }
#[no_mangle]
pub unsafe extern "C" fn to_ireg(gpio: c_int, type: enum ctrl_register, mask: *mut c_uint) -> c_int {
    static inline int to_ireg(int gpio, enum ctrl_register type, unsigned int *mask)
    {
    let mut reg: c_uint = type == IRQ_STATUS ? IRQ_STATUS_BASE : IRQ_MASK_BASE;
    if (gpio < GROUP0_NR_IRQS) {
    reg += 0;
// mask = BIT(gpio);
    } else {
    reg += 1;
// mask = BIT(gpio - GROUP0_NR_IRQS);
    }
    return reg;
    }
#[no_mangle]
unsafe extern "C" fn wcove_update_irq_mask(wg: *mut wcove_gpio, gpio: irq_hw_number_t) {
    static void wcove_update_irq_mask(struct wcove_gpio *wg, irq_hw_number_t gpio)
    {
    unsigned int mask, reg = to_ireg(gpio, IRQ_MASK, &mask);
    if (wg.set_irq_mask)
    regmap_set_bits(wg.regmap, reg, mask);
    else
    regmap_clear_bits(wg.regmap, reg, mask);
    }
#[no_mangle]
unsafe extern "C" fn wcove_update_irq_ctrl(wg: *mut wcove_gpio, gpio: irq_hw_number_t) {
    static void wcove_update_irq_ctrl(struct wcove_gpio *wg, irq_hw_number_t gpio)
    {
    let mut reg: c_int = to_reg(gpio, CTRL_IN);
    regmap_update_bits(wg.regmap, reg, CTLI_INTCNT_BE, wg.intcnt);
    }
#[no_mangle]
unsafe extern "C" fn wcove_gpio_dir_in(chip: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int wcove_gpio_dir_in(struct gpio_chip *chip, unsigned int gpio)
    {
    struct wcove_gpio *wg = gpiochip_get_data(chip);
    let mut reg: c_int = to_reg(gpio, CTRL_OUT);
    if (reg < 0)
    return 0;
    return regmap_write(wg.regmap, reg, CTLO_INPUT_SET);
    }
    static int wcove_gpio_dir_out(struct gpio_chip *chip, unsigned int gpio,
    int value)
    {
    struct wcove_gpio *wg = gpiochip_get_data(chip);
    let mut reg: c_int = to_reg(gpio, CTRL_OUT);
    if (reg < 0)
    return 0;
    return regmap_write(wg.regmap, reg, CTLO_OUTPUT_SET | value);
    }
#[no_mangle]
unsafe extern "C" fn wcove_gpio_get_direction(chip: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int wcove_gpio_get_direction(struct gpio_chip *chip, unsigned int gpio)
    {
    struct wcove_gpio *wg = gpiochip_get_data(chip);
    unsigned int val;
    int ret, reg = to_reg(gpio, CTRL_OUT);
    if (reg < 0)
    return GPIO_LINE_DIRECTION_OUT;
    ret = regmap_read(wg.regmap, reg, &val);
    if (ret)
    return ret;
    if (val & CTLO_DIR_OUT)
    return GPIO_LINE_DIRECTION_OUT;
    return GPIO_LINE_DIRECTION_IN;
    }
#[no_mangle]
unsafe extern "C" fn wcove_gpio_get(chip: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int wcove_gpio_get(struct gpio_chip *chip, unsigned int gpio)
    {
    struct wcove_gpio *wg = gpiochip_get_data(chip);
    unsigned int val;
    int ret, reg = to_reg(gpio, CTRL_IN);
    if (reg < 0)
    return 0;
    ret = regmap_read(wg.regmap, reg, &val);
    if (ret)
    return ret;
    return val & 0x1;
    }
#[no_mangle]
unsafe extern "C" fn wcove_gpio_set(chip: *mut gpio_chip, gpio: c_uint, value: c_int) -> c_int {
    static int wcove_gpio_set(struct gpio_chip *chip, unsigned int gpio, int value)
    {
    struct wcove_gpio *wg = gpiochip_get_data(chip);
    let mut reg: c_int = to_reg(gpio, CTRL_OUT);
    if (reg < 0)
    return 0;
    return regmap_assign_bits(wg.regmap, reg, 1, value);
    }
    static int wcove_gpio_set_config(struct gpio_chip *chip, unsigned int gpio,
    unsigned long config)
    {
    struct wcove_gpio *wg = gpiochip_get_data(chip);
    let mut reg: c_int = to_reg(gpio, CTRL_OUT);
    if (reg < 0)
    return 0;
    switch (pinconf_to_config_param(config)) {
    case PIN_CONFIG_DRIVE_OPEN_DRAIN:
    return regmap_update_bits(wg.regmap, reg, CTLO_DRV_MASK,
    CTLO_DRV_OD);
    case PIN_CONFIG_DRIVE_PUSH_PULL:
    return regmap_update_bits(wg.regmap, reg, CTLO_DRV_MASK,
    CTLO_DRV_CMOS);
    default:
    break;
    }
    return -ENOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn wcove_irq_type(data: *mut irq_data, type: c_uint) -> c_int {
    static int wcove_irq_type(struct irq_data *data, unsigned int type)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct wcove_gpio *wg = gpiochip_get_data(chip);
    let mut gpio: irq_hw_number_t = irqd_to_hwirq(data);
    if (gpio >= WCOVE_GPIO_NUM)
    return 0;
    switch (type) {
    case IRQ_TYPE_NONE:
    wg.intcnt = CTLI_INTCNT_DIS;
    break;
    case IRQ_TYPE_EDGE_BOTH:
    wg.intcnt = CTLI_INTCNT_BE;
    break;
    case IRQ_TYPE_EDGE_RISING:
    wg.intcnt = CTLI_INTCNT_PE;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    wg.intcnt = CTLI_INTCNT_NE;
    break;
    default:
    return -EINVAL;
    }
    wg.update |= UPDATE_IRQ_TYPE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wcove_bus_lock(data: *mut irq_data) {
    static void wcove_bus_lock(struct irq_data *data)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct wcove_gpio *wg = gpiochip_get_data(chip);
    mutex_lock(&wg.buslock);
    }
#[no_mangle]
unsafe extern "C" fn wcove_bus_sync_unlock(data: *mut irq_data) {
    static void wcove_bus_sync_unlock(struct irq_data *data)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct wcove_gpio *wg = gpiochip_get_data(chip);
    let mut gpio: irq_hw_number_t = irqd_to_hwirq(data);
    if (wg.update & UPDATE_IRQ_TYPE)
    wcove_update_irq_ctrl(wg, gpio);
    if (wg.update & UPDATE_IRQ_MASK)
    wcove_update_irq_mask(wg, gpio);
    wg.update = 0;
    mutex_unlock(&wg.buslock);
    }
#[no_mangle]
unsafe extern "C" fn wcove_irq_unmask(data: *mut irq_data) {
    static void wcove_irq_unmask(struct irq_data *data)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct wcove_gpio *wg = gpiochip_get_data(chip);
    let mut gpio: irq_hw_number_t = irqd_to_hwirq(data);
    if (gpio >= WCOVE_GPIO_NUM)
    return;
    gpiochip_enable_irq(chip, gpio);
    wg.set_irq_mask = false;
    wg.update |= UPDATE_IRQ_MASK;
    }
#[no_mangle]
unsafe extern "C" fn wcove_irq_mask(data: *mut irq_data) {
    static void wcove_irq_mask(struct irq_data *data)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct wcove_gpio *wg = gpiochip_get_data(chip);
    let mut gpio: irq_hw_number_t = irqd_to_hwirq(data);
    if (gpio >= WCOVE_GPIO_NUM)
    return;
    wg.set_irq_mask = true;
    wg.update |= UPDATE_IRQ_MASK;
    gpiochip_disable_irq(chip, gpio);
    }
    static const struct irq_chip wcove_irqchip = {
    .name			= "Whiskey Cove",
    .irq_mask		= wcove_irq_mask,
    .irq_unmask		= wcove_irq_unmask,
    .irq_set_type		= wcove_irq_type,
    .irq_bus_lock		= wcove_bus_lock,
    .irq_bus_sync_unlock	= wcove_bus_sync_unlock,
    .flags			= IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn wcove_gpio_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t wcove_gpio_irq_handler(int irq, void *data)
    {
    struct wcove_gpio *wg = (struct wcove_gpio *)data;
    unsigned int virq, gpio;
    unsigned long pending;
    u8 p[2];
    if (regmap_bulk_read(wg.regmap, IRQ_STATUS_BASE, p, 2)) {
    dev_err(wg.dev, "Failed to read irq status register\n");
    return IRQ_NONE;
    }
    pending = (p[0] & GPIO_IRQ0_MASK) | ((p[1] & GPIO_IRQ1_MASK) << 7);
    if (!pending)
    return IRQ_NONE;
// Iterate until no interrupt is pending
    while (pending) {
// One iteration is for all pending bits
    for_each_set_bit(gpio, &pending, WCOVE_GPIO_NUM) {
    unsigned int mask, reg = to_ireg(gpio, IRQ_STATUS, &mask);
    virq = irq_find_mapping(wg.chip.irq.domain, gpio);
    handle_nested_irq(virq);
    regmap_set_bits(wg.regmap, reg, mask);
    }
// Next iteration
    if (regmap_bulk_read(wg.regmap, IRQ_STATUS_BASE, p, 2)) {
    dev_err(wg.dev, "Failed to read irq status\n");
    break;
    }
    pending = (p[0] & GPIO_IRQ0_MASK) | ((p[1] & GPIO_IRQ1_MASK) << 7);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn wcove_gpio_dbg_show(s: *mut seq_file, chip: *mut gpio_chip) {
    static void wcove_gpio_dbg_show(struct seq_file *s, struct gpio_chip *chip)
    {
    unsigned int ctlo, ctli, irq_mask, irq_status;
    struct wcove_gpio *wg = gpiochip_get_data(chip);
    int gpio, mask, ret = 0;
    for (gpio = 0; gpio < WCOVE_GPIO_NUM; gpio++) {
    ret += regmap_read(wg.regmap, to_reg(gpio, CTRL_OUT), &ctlo);
    ret += regmap_read(wg.regmap, to_reg(gpio, CTRL_IN), &ctli);
    if (ret) {
    dev_err(wg.dev, "Failed to read registers: CTRL out/in\n");
    break;
    }
    ret += regmap_read(wg.regmap, to_ireg(gpio, IRQ_MASK, &mask), &irq_mask);
    ret += regmap_read(wg.regmap, to_ireg(gpio, IRQ_STATUS, &mask), &irq_status);
    if (ret) {
    dev_err(wg.dev, "Failed to read registers: IRQ status/mask\n");
    break;
    }
    seq_printf(s, " gpio-%-2d %s %s %s %s ctlo=%2x,%s %s\n",
    gpio, ctlo & CTLO_DIR_OUT ? "out" : "in ",
    str_hi_lo(ctli & 0x1),
    ctli & CTLI_INTCNT_NE ? "fall" : "    ",
    ctli & CTLI_INTCNT_PE ? "rise" : "    ",
    ctlo,
    irq_mask & mask ? "mask  " : "unmask",
    irq_status & mask ? "pending" : "       ");
    }
    }
#[no_mangle]
unsafe extern "C" fn wcove_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int wcove_gpio_probe(struct platform_device *pdev)
    {
    struct intel_soc_pmic *pmic;
    struct wcove_gpio *wg;
    int virq, ret, irq;
    struct device *dev;
    struct gpio_irq_chip *girq;
//
// This gpio platform device is created by a mfd device (see
// drivers/mfd/intel_soc_pmic_bxtwc.c for details). Information
// shared by all sub-devices created by the mfd device, the regmap
// pointer for instance, is stored as driver data of the mfd device
// driver.
//
    pmic = dev_get_drvdata(pdev.dev.parent);
    if (!pmic)
    return -ENODEV;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    dev = &pdev.dev;
    wg = devm_kzalloc(dev, sizeof(*wg), GFP_KERNEL);
    if (!wg)
    return -ENOMEM;
    wg.regmap_irq_chip = pmic.irq_chip_data;
    platform_set_drvdata(pdev, wg);
    mutex_init(&wg.buslock);
    wg.chip.label = KBUILD_MODNAME;
    wg.chip.direction_input = wcove_gpio_dir_in;
    wg.chip.direction_output = wcove_gpio_dir_out;
    wg.chip.get_direction = wcove_gpio_get_direction;
    wg.chip.get = wcove_gpio_get;
    wg.chip.set = wcove_gpio_set;
    wg.chip.set_config = wcove_gpio_set_config;
    wg.chip.base = -1;
    wg.chip.ngpio = WCOVE_VGPIO_NUM;
    wg.chip.can_sleep = true;
    wg.chip.parent = pdev.dev.parent;
    wg.chip.dbg_show = wcove_gpio_dbg_show;
    wg.dev = dev;
    wg.regmap = pmic.regmap;
    virq = regmap_irq_get_virq(wg.regmap_irq_chip, irq);
    if (virq < 0) {
    dev_err(dev, "Failed to get virq by irq %d\n", irq);
    return virq;
    }
    girq = &wg.chip.irq;
    gpio_irq_chip_set_chip(girq, &wcove_irqchip);
// This will let us handle the parent IRQ in the driver
    girq.parent_handler = core::ptr::null_mut();
    girq.num_parents = 0;
    girq.parents = core::ptr::null_mut();
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = handle_simple_irq;
    girq.threaded = true;
    ret = devm_request_threaded_irq(dev, virq, core::ptr::null_mut(), wcove_gpio_irq_handler,
    IRQF_ONESHOT, pdev.name, wg);
    if (ret) {
    dev_err(dev, "Failed to request irq %d\n", virq);
    return ret;
    }
    ret = devm_gpiochip_add_data(dev, &wg.chip, wg);
    if (ret) {
    dev_err(dev, "Failed to add gpiochip: %d\n", ret);
    return ret;
    }
// Enable GPIO0 interrupts
    ret = regmap_clear_bits(wg.regmap, IRQ_MASK_BASE + 0, GPIO_IRQ0_MASK);
    if (ret)
    return ret;
// Enable GPIO1 interrupts
    ret = regmap_clear_bits(wg.regmap, IRQ_MASK_BASE + 1, GPIO_IRQ1_MASK);
    if (ret)
    return ret;
    return 0;
    }
//
// Whiskey Cove PMIC itself is a analog device(but with digital control
// interface) providing power management support for other devices in
// the accompanied SoC, so we have no .pm for Whiskey Cove GPIO driver.
//
    static struct platform_driver wcove_gpio_driver = {
    .driver = {
    .name = "bxt_wcove_gpio",
    },
    .probe = wcove_gpio_probe,
    };
    module_platform_driver(wcove_gpio_driver);
    MODULE_AUTHOR("Ajay Thomas <ajay.thomas.david.rajamanickam@intel.com>");
    MODULE_AUTHOR("Bin Gao <bin.gao@intel.com>");
    MODULE_DESCRIPTION("Intel Whiskey Cove GPIO Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:bxt_wcove_gpio");
