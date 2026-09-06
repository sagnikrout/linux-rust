//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-pmic-eic-sprd.c
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
// Copyright (C) 2018 Spreadtrum Communications Inc.
// Copyright (C) 2018 Linaro Ltd.
//

// EIC registers definition
pub const SPRD_PMIC_EIC_DATA: c_uint = 0x0;
pub const SPRD_PMIC_EIC_DMSK: c_uint = 0x4;
pub const SPRD_PMIC_EIC_IEV: c_uint = 0x14;
pub const SPRD_PMIC_EIC_IE: c_uint = 0x18;
pub const SPRD_PMIC_EIC_RIS: c_uint = 0x1c;
pub const SPRD_PMIC_EIC_MIS: c_uint = 0x20;
pub const SPRD_PMIC_EIC_IC: c_uint = 0x24;
pub const SPRD_PMIC_EIC_TRIG: c_uint = 0x28;
pub const SPRD_PMIC_EIC_CTRL0: c_uint = 0x40;
//
// The PMIC EIC controller only has one bank, and each bank now can contain
// 16 EICs.
//
pub const SPRD_PMIC_EIC_PER_BANK_NR: c_int = 16;

//
// These registers are modified under the irq bus lock and cached to avoid
// unnecessary writes in bus_sync_unlock.
//
    enum {
    REG_IEV,
    REG_IE,
    REG_TRIG,
    CACHE_NR_REGS
    };
//
// struct sprd_pmic_eic - PMIC EIC controller
// @chip: the gpio_chip structure.
// @map:  the regmap from the parent device.
// @offset: the EIC controller's offset address of the PMIC.
// @reg: the array to cache the EIC registers.
// @buslock: for bus lock/sync and unlock.
// @irq: the interrupt number of the PMIC EIC conteroller.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_pmic_eic {
    pub chip: gpio_chip,
    pub map: *mut regmap,
    pub offset: u32,
    pub reg: [u8; CACHE_NR_REGS],
    pub buslock: mutex,
    pub irq: c_int,
}

    static void sprd_pmic_eic_update(struct gpio_chip *chip, unsigned int offset,
    u16 reg, unsigned int val)
    {
    struct sprd_pmic_eic *pmic_eic = gpiochip_get_data(chip);
    let mut shift: u32 = SPRD_PMIC_EIC_BIT(offset);
    regmap_update_bits(pmic_eic.map, pmic_eic.offset + reg,
    BIT(shift), val << shift);
    }
    static int sprd_pmic_eic_read(struct gpio_chip *chip, unsigned int offset,
    u16 reg)
    {
    struct sprd_pmic_eic *pmic_eic = gpiochip_get_data(chip);
    u32 value;
    int ret;
    ret = regmap_read(pmic_eic.map, pmic_eic.offset + reg, &value);
    if (ret)
    return ret;
    return !!(value & BIT(SPRD_PMIC_EIC_BIT(offset)));
    }
#[no_mangle]
unsafe extern "C" fn sprd_pmic_eic_request(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int sprd_pmic_eic_request(struct gpio_chip *chip, unsigned int offset)
    {
    sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_DMSK, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_pmic_eic_free(chip: *mut gpio_chip, offset: c_uint) {
    static void sprd_pmic_eic_free(struct gpio_chip *chip, unsigned int offset)
    {
    sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_DMSK, 0);
    }
#[no_mangle]
unsafe extern "C" fn sprd_pmic_eic_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int sprd_pmic_eic_get(struct gpio_chip *chip, unsigned int offset)
    {
    return sprd_pmic_eic_read(chip, offset, SPRD_PMIC_EIC_DATA);
    }
    static int sprd_pmic_eic_direction_input(struct gpio_chip *chip,
    unsigned int offset)
    {
// EICs are always input, nothing need to do here.
    return 0;
    }
    static int sprd_pmic_eic_set_debounce(struct gpio_chip *chip,
    unsigned int offset,
    unsigned int debounce)
    {
    struct sprd_pmic_eic *pmic_eic = gpiochip_get_data(chip);
    u32 reg, value;
    int ret;
    reg = SPRD_PMIC_EIC_CTRL0 + SPRD_PMIC_EIC_BIT(offset) * 0x4;
    ret = regmap_read(pmic_eic.map, pmic_eic.offset + reg, &value);
    if (ret)
    return ret;
    value &= ~SPRD_PMIC_EIC_DBNC_MASK;
    value |= (debounce / 1000) & SPRD_PMIC_EIC_DBNC_MASK;
    return regmap_write(pmic_eic.map, pmic_eic.offset + reg, value);
    }
    static int sprd_pmic_eic_set_config(struct gpio_chip *chip, unsigned int offset,
    unsigned long config)
    {
    let mut param: c_ulong = pinconf_to_config_param(config);
    let mut arg: u32 = pinconf_to_config_argument(config);
    if (param == PIN_CONFIG_INPUT_DEBOUNCE)
    return sprd_pmic_eic_set_debounce(chip, offset, arg);
    return -ENOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn sprd_pmic_eic_irq_mask(data: *mut irq_data) {
    static void sprd_pmic_eic_irq_mask(struct irq_data *data)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct sprd_pmic_eic *pmic_eic = gpiochip_get_data(chip);
    let mut offset: u32 = irqd_to_hwirq(data);
    pmic_eic.reg[REG_IE] &= ~BIT(offset);
    pmic_eic.reg[REG_TRIG] &= ~BIT(offset);
    gpiochip_disable_irq(chip, offset);
    }
#[no_mangle]
unsafe extern "C" fn sprd_pmic_eic_irq_unmask(data: *mut irq_data) {
    static void sprd_pmic_eic_irq_unmask(struct irq_data *data)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct sprd_pmic_eic *pmic_eic = gpiochip_get_data(chip);
    let mut offset: u32 = irqd_to_hwirq(data);
    gpiochip_enable_irq(chip, offset);
    pmic_eic.reg[REG_IE] |= BIT(offset);
    pmic_eic.reg[REG_TRIG] |= BIT(offset);
    }
    static int sprd_pmic_eic_irq_set_type(struct irq_data *data,
    unsigned int flow_type)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct sprd_pmic_eic *pmic_eic = gpiochip_get_data(chip);
    let mut offset: u32 = irqd_to_hwirq(data);
    switch (flow_type) {
    case IRQ_TYPE_LEVEL_HIGH:
    pmic_eic.reg[REG_IEV] |= BIT(offset);
    break;
    case IRQ_TYPE_LEVEL_LOW:
    pmic_eic.reg[REG_IEV] &= ~BIT(offset);
    break;
    case IRQ_TYPE_EDGE_RISING:
    case IRQ_TYPE_EDGE_FALLING:
    case IRQ_TYPE_EDGE_BOTH:
//
// Will set the trigger level according to current EIC level
// in irq_bus_sync_unlock() interface, so here nothing to do.
//
    break;
    default:
    return -ENOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_pmic_eic_bus_lock(data: *mut irq_data) {
    static void sprd_pmic_eic_bus_lock(struct irq_data *data)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct sprd_pmic_eic *pmic_eic = gpiochip_get_data(chip);
    mutex_lock(&pmic_eic.buslock);
    }
#[no_mangle]
unsafe extern "C" fn sprd_pmic_eic_bus_sync_unlock(data: *mut irq_data) {
    static void sprd_pmic_eic_bus_sync_unlock(struct irq_data *data)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct sprd_pmic_eic *pmic_eic = gpiochip_get_data(chip);
    let mut trigger: u32 = irqd_get_trigger_type(data);
    let mut offset: u32 = irqd_to_hwirq(data);
    int state;
// Set irq type
    if (trigger & IRQ_TYPE_EDGE_BOTH) {
    state = sprd_pmic_eic_get(chip, offset);
    if (state)
    sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_IEV, 0);
    else
    sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_IEV, 1);
    } else {
    sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_IEV,
    !!(pmic_eic.reg[REG_IEV] & BIT(offset)));
    }
// Set irq unmask
    sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_IE,
    !!(pmic_eic.reg[REG_IE] & BIT(offset)));
// Generate trigger start pulse for debounce EIC
    sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_TRIG,
    !!(pmic_eic.reg[REG_TRIG] & BIT(offset)));
    mutex_unlock(&pmic_eic.buslock);
    }
    static void sprd_pmic_eic_toggle_trigger(struct gpio_chip *chip,
    unsigned int irq, unsigned int offset)
    {
    let mut trigger: u32 = irq_get_trigger_type(irq);
    int state, post_state;
    if (!(trigger & IRQ_TYPE_EDGE_BOTH))
    return;
    state = sprd_pmic_eic_get(chip, offset);
    retry:
    if (state)
    sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_IEV, 0);
    else
    sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_IEV, 1);
    post_state = sprd_pmic_eic_get(chip, offset);
    if (state != post_state) {
    dev_warn(chip.parent, "PMIC EIC level was changed.\n");
    state = post_state;
    goto retry;
    }
// Set irq unmask
    sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_IE, 1);
// Generate trigger start pulse for debounce EIC
    sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_TRIG, 1);
    }
#[no_mangle]
unsafe extern "C" fn sprd_pmic_eic_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t sprd_pmic_eic_irq_handler(int irq, void *data)
    {
    struct sprd_pmic_eic *pmic_eic = data;
    struct gpio_chip *chip = &pmic_eic.chip;
    unsigned long status;
    u32 n, girq, val;
    int ret;
    ret = regmap_read(pmic_eic.map, pmic_eic.offset + SPRD_PMIC_EIC_MIS,
    &val);
    if (ret)
    return IRQ_RETVAL(ret);
    status = val & SPRD_PMIC_EIC_DATA_MASK;
    for_each_set_bit(n, &status, chip.ngpio) {
// Clear the interrupt
    sprd_pmic_eic_update(chip, n, SPRD_PMIC_EIC_IC, 1);
    girq = irq_find_mapping(chip.irq.domain, n);
    handle_nested_irq(girq);
//
// The PMIC EIC can only support level trigger, so we can
// toggle the level trigger to emulate the edge trigger.
//
    sprd_pmic_eic_toggle_trigger(chip, girq, n);
    }
    return IRQ_HANDLED;
    }
    static const struct irq_chip pmic_eic_irq_chip = {
    .name			= "sprd-pmic-eic",
    .irq_mask		= sprd_pmic_eic_irq_mask,
    .irq_unmask		= sprd_pmic_eic_irq_unmask,
    .irq_set_type		= sprd_pmic_eic_irq_set_type,
    .irq_bus_lock		= sprd_pmic_eic_bus_lock,
    .irq_bus_sync_unlock	= sprd_pmic_eic_bus_sync_unlock,
    .flags			= IRQCHIP_SKIP_SET_WAKE | IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn sprd_pmic_eic_probe(pdev: *mut platform_device) -> c_int {
    static int sprd_pmic_eic_probe(struct platform_device *pdev)
    {
    struct gpio_irq_chip *irq;
    struct sprd_pmic_eic *pmic_eic;
    int ret;
    pmic_eic = devm_kzalloc(&pdev.dev, sizeof(*pmic_eic), GFP_KERNEL);
    if (!pmic_eic)
    return -ENOMEM;
    mutex_init(&pmic_eic.buslock);
    pmic_eic.irq = platform_get_irq(pdev, 0);
    if (pmic_eic.irq < 0)
    return pmic_eic.irq;
    pmic_eic.map = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!pmic_eic.map)
    return -ENODEV;
    ret = of_property_read_u32(pdev.dev.of_node, "reg", &pmic_eic.offset);
    if (ret) {
    dev_err(&pdev.dev, "Failed to get PMIC EIC base address.\n");
    return ret;
    }
    ret = devm_request_threaded_irq(&pdev.dev, pmic_eic.irq, core::ptr::null_mut(),
    sprd_pmic_eic_irq_handler,
    IRQF_ONESHOT | IRQF_NO_SUSPEND,
    dev_name(&pdev.dev), pmic_eic);
    if (ret) {
    dev_err(&pdev.dev, "Failed to request PMIC EIC IRQ.\n");
    return ret;
    }
    pmic_eic.chip.label = dev_name(&pdev.dev);
    pmic_eic.chip.ngpio = SPRD_PMIC_EIC_NR;
    pmic_eic.chip.base = -1;
    pmic_eic.chip.parent = &pdev.dev;
    pmic_eic.chip.direction_input = sprd_pmic_eic_direction_input;
    pmic_eic.chip.request = sprd_pmic_eic_request;
    pmic_eic.chip.free = sprd_pmic_eic_free;
    pmic_eic.chip.set_config = sprd_pmic_eic_set_config;
    pmic_eic.chip.get = sprd_pmic_eic_get;
    pmic_eic.chip.can_sleep = true;
    irq = &pmic_eic.chip.irq;
    gpio_irq_chip_set_chip(irq, &pmic_eic_irq_chip);
    irq.threaded = true;
    ret = devm_gpiochip_add_data(&pdev.dev, &pmic_eic.chip, pmic_eic);
    if (ret < 0) {
    dev_err(&pdev.dev, "Could not register gpiochip %d.\n", ret);
    return ret;
    }
    return 0;
    }
    static const struct of_device_id sprd_pmic_eic_of_match[] = {
    { .compatible = "sprd,sc2731-eic", },
    { /* end of list */ }
    };
    MODULE_DEVICE_TABLE(of, sprd_pmic_eic_of_match);
    static struct platform_driver sprd_pmic_eic_driver = {
    .probe = sprd_pmic_eic_probe,
    .driver = {
    .name = "sprd-pmic-eic",
    .of_match_table	= sprd_pmic_eic_of_match,
    },
    };
    module_platform_driver(sprd_pmic_eic_driver);
    MODULE_DESCRIPTION("Spreadtrum PMIC EIC driver");
    MODULE_LICENSE("GPL v2");
