//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-bcm-kona.c
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
// Broadcom Kona GPIO Driver
//
// Author: Broadcom Corporation <bcm-kernel-feedback-list@broadcom.com>
// Copyright (C) 2012-2014 Broadcom Corporation
//

pub const BCM_GPIO_PASSWD: c_uint = 0x00a5a501;
pub const GPIO_PER_BANK: c_int = 32;
pub const GPIO_MAX_BANK_NUM: c_int = 8;

// There is a GPIO control register for each GPIO

// The remaining registers are per GPIO bank

pub const GPIO_GPPWR_OFFSET: c_uint = 0x00000520;
pub const GPIO_GPCTR0_DBR_SHIFT: c_int = 5;
pub const GPIO_GPCTR0_DBR_MASK: c_uint = 0x000001e0;
pub const GPIO_GPCTR0_ITR_SHIFT: c_int = 3;
pub const GPIO_GPCTR0_ITR_MASK: c_uint = 0x00000018;
pub const GPIO_GPCTR0_ITR_CMD_RISING_EDGE: c_uint = 0x00000001;
pub const GPIO_GPCTR0_ITR_CMD_FALLING_EDGE: c_uint = 0x00000002;
pub const GPIO_GPCTR0_ITR_CMD_BOTH_EDGE: c_uint = 0x00000003;
pub const GPIO_GPCTR0_IOTR_MASK: c_uint = 0x00000001;
pub const GPIO_GPCTR0_IOTR_CMD_0UTPUT: c_uint = 0x00000000;
pub const GPIO_GPCTR0_IOTR_CMD_INPUT: c_uint = 0x00000001;
pub const GPIO_GPCTR0_DB_ENABLE_MASK: c_uint = 0x00000100;
pub const LOCK_CODE: c_uint = 0xffffffff;
pub const UNLOCK_CODE: c_uint = 0x00000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_kona_gpio_bank {
    pub id: c_int,
    pub irq: c_int,
//
// Used to keep track of lock/unlock operations for each GPIO in the
// bank.
//
// All GPIOs are locked by default (see bcm_kona_gpio_reset), and the
// unlock count for all GPIOs is 0 by default. Each unlock increments
// the counter, and each lock decrements the counter.
//
// The lock function only locks the GPIO once its unlock counter is
// down to 0. This is necessary because the GPIO is unlocked in two
// places in this driver: once for requested GPIOs, and once for
// requested IRQs. Since it is possible for a GPIO to be requested
// as both a GPIO and an IRQ, we need to ensure that we don't lock it
// too early.
//
    pub gpio_unlock_count: [u8; GPIO_PER_BANK],
// Used in the interrupt handler
    pub kona_gpio: *mut bcm_kona_gpio,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_kona_gpio {
    pub reg_base: *mut void __iomem,
    pub num_bank: c_int,
    pub lock: raw_spinlock_t,
    pub gpio_chip: gpio_chip,
    pub irq_domain: *mut irq_domain,
    pub __counted_by(num_bank): bcm_kona_gpio_bank banks[],
}

    static inline void bcm_kona_gpio_write_lock_regs(void __iomem *reg_base,
    int bank_id, u32 lockcode)
    {
    writel(BCM_GPIO_PASSWD, reg_base + GPIO_GPPWR_OFFSET);
    writel(lockcode, reg_base + GPIO_PWD_STATUS(bank_id));
    }
    static void bcm_kona_gpio_lock_gpio(struct bcm_kona_gpio *kona_gpio,
    unsigned gpio)
    {
    u32 val;
    let mut bank_id: c_int = GPIO_BANK(gpio);
    let mut bit: c_int = GPIO_BIT(gpio);
    struct bcm_kona_gpio_bank *bank = &kona_gpio.banks[bank_id];
    if (bank.gpio_unlock_count[bit] == 0) {
    dev_err(kona_gpio.gpio_chip.parent,
    "Unbalanced locks for GPIO %u\n", gpio);
    return;
    }
    if (--bank.gpio_unlock_count[bit] == 0) {
    guard(raw_spinlock_irqsave)(&kona_gpio.lock);
    val = readl(kona_gpio.reg_base + GPIO_PWD_STATUS(bank_id));
    val |= BIT(bit);
    bcm_kona_gpio_write_lock_regs(kona_gpio.reg_base, bank_id, val);
    }
    }
    static void bcm_kona_gpio_unlock_gpio(struct bcm_kona_gpio *kona_gpio,
    unsigned gpio)
    {
    u32 val;
    let mut bank_id: c_int = GPIO_BANK(gpio);
    let mut bit: c_int = GPIO_BIT(gpio);
    struct bcm_kona_gpio_bank *bank = &kona_gpio.banks[bank_id];
    if (bank.gpio_unlock_count[bit] == 0) {
    guard(raw_spinlock_irqsave)(&kona_gpio.lock);
    val = readl(kona_gpio.reg_base + GPIO_PWD_STATUS(bank_id));
    val &= ~BIT(bit);
    bcm_kona_gpio_write_lock_regs(kona_gpio.reg_base, bank_id, val);
    }
    ++bank.gpio_unlock_count[bit];
    }
#[no_mangle]
unsafe extern "C" fn bcm_kona_gpio_get_dir(chip: *mut gpio_chip, gpio: unsigned) -> c_int {
    static int bcm_kona_gpio_get_dir(struct gpio_chip *chip, unsigned gpio)
    {
    struct bcm_kona_gpio *kona_gpio = gpiochip_get_data(chip);
    void __iomem *reg_base = kona_gpio.reg_base;
    u32 val;
    val = readl(reg_base + GPIO_CONTROL(gpio)) & GPIO_GPCTR0_IOTR_MASK;
    return val ? GPIO_LINE_DIRECTION_IN : GPIO_LINE_DIRECTION_OUT;
    }
    static int bcm_kona_gpio_set(struct gpio_chip *chip, unsigned int gpio,
    int value)
    {
    struct bcm_kona_gpio *kona_gpio;
    void __iomem *reg_base;
    let mut bank_id: c_int = GPIO_BANK(gpio);
    let mut bit: c_int = GPIO_BIT(gpio);
    u32 val, reg_offset;
    kona_gpio = gpiochip_get_data(chip);
    reg_base = kona_gpio.reg_base;
    guard(raw_spinlock_irqsave)(&kona_gpio.lock);
// this function only applies to output pin
    if (bcm_kona_gpio_get_dir(chip, gpio) == GPIO_LINE_DIRECTION_IN)
    return 0;
    reg_offset = value ? GPIO_OUT_SET(bank_id) : GPIO_OUT_CLEAR(bank_id);
    val = readl(reg_base + reg_offset);
    val |= BIT(bit);
    writel(val, reg_base + reg_offset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm_kona_gpio_get(chip: *mut gpio_chip, gpio: unsigned) -> c_int {
    static int bcm_kona_gpio_get(struct gpio_chip *chip, unsigned gpio)
    {
    struct bcm_kona_gpio *kona_gpio;
    void __iomem *reg_base;
    let mut bank_id: c_int = GPIO_BANK(gpio);
    let mut bit: c_int = GPIO_BIT(gpio);
    u32 val, reg_offset;
    kona_gpio = gpiochip_get_data(chip);
    reg_base = kona_gpio.reg_base;
    guard(raw_spinlock_irqsave)(&kona_gpio.lock);
    if (bcm_kona_gpio_get_dir(chip, gpio) == GPIO_LINE_DIRECTION_IN)
    reg_offset = GPIO_IN_STATUS(bank_id);
    else
    reg_offset = GPIO_OUT_STATUS(bank_id);
// read the GPIO bank status
    val = readl(reg_base + reg_offset);
// return the specified bit status
    return !!(val & BIT(bit));
    }
#[no_mangle]
unsafe extern "C" fn bcm_kona_gpio_request(chip: *mut gpio_chip, gpio: unsigned) -> c_int {
    static int bcm_kona_gpio_request(struct gpio_chip *chip, unsigned gpio)
    {
    struct bcm_kona_gpio *kona_gpio = gpiochip_get_data(chip);
    bcm_kona_gpio_unlock_gpio(kona_gpio, gpio);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm_kona_gpio_free(chip: *mut gpio_chip, gpio: unsigned) {
    static void bcm_kona_gpio_free(struct gpio_chip *chip, unsigned gpio)
    {
    struct bcm_kona_gpio *kona_gpio = gpiochip_get_data(chip);
    bcm_kona_gpio_lock_gpio(kona_gpio, gpio);
    }
#[no_mangle]
unsafe extern "C" fn bcm_kona_gpio_direction_input(chip: *mut gpio_chip, gpio: unsigned) -> c_int {
    static int bcm_kona_gpio_direction_input(struct gpio_chip *chip, unsigned gpio)
    {
    struct bcm_kona_gpio *kona_gpio;
    void __iomem *reg_base;
    u32 val;
    kona_gpio = gpiochip_get_data(chip);
    reg_base = kona_gpio.reg_base;
    guard(raw_spinlock_irqsave)(&kona_gpio.lock);
    val = readl(reg_base + GPIO_CONTROL(gpio));
    val &= ~GPIO_GPCTR0_IOTR_MASK;
    val |= GPIO_GPCTR0_IOTR_CMD_INPUT;
    writel(val, reg_base + GPIO_CONTROL(gpio));
    return 0;
    }
    static int bcm_kona_gpio_direction_output(struct gpio_chip *chip,
    unsigned gpio, int value)
    {
    struct bcm_kona_gpio *kona_gpio;
    void __iomem *reg_base;
    let mut bank_id: c_int = GPIO_BANK(gpio);
    let mut bit: c_int = GPIO_BIT(gpio);
    u32 val, reg_offset;
    kona_gpio = gpiochip_get_data(chip);
    reg_base = kona_gpio.reg_base;
    guard(raw_spinlock_irqsave)(&kona_gpio.lock);
    val = readl(reg_base + GPIO_CONTROL(gpio));
    val &= ~GPIO_GPCTR0_IOTR_MASK;
    val |= GPIO_GPCTR0_IOTR_CMD_0UTPUT;
    writel(val, reg_base + GPIO_CONTROL(gpio));
    reg_offset = value ? GPIO_OUT_SET(bank_id) : GPIO_OUT_CLEAR(bank_id);
    val = readl(reg_base + reg_offset);
    val |= BIT(bit);
    writel(val, reg_base + reg_offset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm_kona_gpio_to_irq(chip: *mut gpio_chip, gpio: unsigned) -> c_int {
    static int bcm_kona_gpio_to_irq(struct gpio_chip *chip, unsigned gpio)
    {
    struct bcm_kona_gpio *kona_gpio;
    kona_gpio = gpiochip_get_data(chip);
    if (gpio >= kona_gpio.gpio_chip.ngpio)
    return -ENXIO;
    return irq_create_mapping(kona_gpio.irq_domain, gpio);
    }
    static int bcm_kona_gpio_set_debounce(struct gpio_chip *chip, unsigned gpio,
    unsigned debounce)
    {
    struct bcm_kona_gpio *kona_gpio;
    void __iomem *reg_base;
    u32 val, res;
    kona_gpio = gpiochip_get_data(chip);
    reg_base = kona_gpio.reg_base;
// debounce must be 1-128ms (or 0)
    if ((debounce > 0 && debounce < 1000) || debounce > 128000) {
    dev_err(chip.parent, "Debounce value %u not in range\n",
    debounce);
    return -EINVAL;
    }
// calculate debounce bit value
    if (debounce != 0) {
// Convert to ms
    debounce /= 1000;
// find the MSB
    res = fls(debounce) - 1;
// Check if MSB-1 is set (round up or down)
    if (res > 0 && (debounce & BIT(res - 1)))
    res++;
    }
// spin lock for read-modify-write of the GPIO register
    guard(raw_spinlock_irqsave)(&kona_gpio.lock);
    val = readl(reg_base + GPIO_CONTROL(gpio));
    val &= ~GPIO_GPCTR0_DBR_MASK;
    if (debounce == 0) {
// disable debounce
    val &= ~GPIO_GPCTR0_DB_ENABLE_MASK;
    } else {
    val |= GPIO_GPCTR0_DB_ENABLE_MASK |
    (res << GPIO_GPCTR0_DBR_SHIFT);
    }
    writel(val, reg_base + GPIO_CONTROL(gpio));
    return 0;
    }
    static int bcm_kona_gpio_set_config(struct gpio_chip *chip, unsigned gpio,
    unsigned long config)
    {
    u32 debounce;
    if (pinconf_to_config_param(config) != PIN_CONFIG_INPUT_DEBOUNCE)
    return -ENOTSUPP;
    debounce = pinconf_to_config_argument(config);
    return bcm_kona_gpio_set_debounce(chip, gpio, debounce);
    }
    static const struct gpio_chip template_chip = {
    .label = "bcm-kona-gpio",
    .owner = THIS_MODULE,
    .request = bcm_kona_gpio_request,
    .free = bcm_kona_gpio_free,
    .get_direction = bcm_kona_gpio_get_dir,
    .direction_input = bcm_kona_gpio_direction_input,
    .get = bcm_kona_gpio_get,
    .direction_output = bcm_kona_gpio_direction_output,
    .set = bcm_kona_gpio_set,
    .set_config = bcm_kona_gpio_set_config,
    .to_irq = bcm_kona_gpio_to_irq,
    .base = 0,
    };
#[no_mangle]
unsafe extern "C" fn bcm_kona_gpio_irq_ack(d: *mut irq_data) {
    static void bcm_kona_gpio_irq_ack(struct irq_data *d)
    {
    struct bcm_kona_gpio *kona_gpio;
    void __iomem *reg_base;
    let mut gpio: unsigned = d.hwirq;
    let mut bank_id: c_int = GPIO_BANK(gpio);
    let mut bit: c_int = GPIO_BIT(gpio);
    u32 val;
    kona_gpio = irq_data_get_irq_chip_data(d);
    reg_base = kona_gpio.reg_base;
    guard(raw_spinlock_irqsave)(&kona_gpio.lock);
    val = readl(reg_base + GPIO_INT_STATUS(bank_id));
    val |= BIT(bit);
    writel(val, reg_base + GPIO_INT_STATUS(bank_id));
    }
#[no_mangle]
unsafe extern "C" fn bcm_kona_gpio_irq_mask(d: *mut irq_data) {
    static void bcm_kona_gpio_irq_mask(struct irq_data *d)
    {
    struct bcm_kona_gpio *kona_gpio;
    void __iomem *reg_base;
    let mut gpio: unsigned = d.hwirq;
    let mut bank_id: c_int = GPIO_BANK(gpio);
    let mut bit: c_int = GPIO_BIT(gpio);
    u32 val;
    kona_gpio = irq_data_get_irq_chip_data(d);
    reg_base = kona_gpio.reg_base;
    guard(raw_spinlock_irqsave)(&kona_gpio.lock);
    val = readl(reg_base + GPIO_INT_MASK(bank_id));
    val |= BIT(bit);
    writel(val, reg_base + GPIO_INT_MASK(bank_id));
    gpiochip_disable_irq(&kona_gpio.gpio_chip, gpio);
    }
#[no_mangle]
unsafe extern "C" fn bcm_kona_gpio_irq_unmask(d: *mut irq_data) {
    static void bcm_kona_gpio_irq_unmask(struct irq_data *d)
    {
    struct bcm_kona_gpio *kona_gpio;
    void __iomem *reg_base;
    let mut gpio: unsigned = d.hwirq;
    let mut bank_id: c_int = GPIO_BANK(gpio);
    let mut bit: c_int = GPIO_BIT(gpio);
    u32 val;
    kona_gpio = irq_data_get_irq_chip_data(d);
    reg_base = kona_gpio.reg_base;
    guard(raw_spinlock_irqsave)(&kona_gpio.lock);
    val = readl(reg_base + GPIO_INT_MSKCLR(bank_id));
    val |= BIT(bit);
    writel(val, reg_base + GPIO_INT_MSKCLR(bank_id));
    gpiochip_enable_irq(&kona_gpio.gpio_chip, gpio);
    }
#[no_mangle]
unsafe extern "C" fn bcm_kona_gpio_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int bcm_kona_gpio_irq_set_type(struct irq_data *d, unsigned int type)
    {
    struct bcm_kona_gpio *kona_gpio;
    void __iomem *reg_base;
    let mut gpio: unsigned = d.hwirq;
    u32 lvl_type;
    u32 val;
    kona_gpio = irq_data_get_irq_chip_data(d);
    reg_base = kona_gpio.reg_base;
    switch (type & IRQ_TYPE_SENSE_MASK) {
    case IRQ_TYPE_EDGE_RISING:
    lvl_type = GPIO_GPCTR0_ITR_CMD_RISING_EDGE;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    lvl_type = GPIO_GPCTR0_ITR_CMD_FALLING_EDGE;
    break;
    case IRQ_TYPE_EDGE_BOTH:
    lvl_type = GPIO_GPCTR0_ITR_CMD_BOTH_EDGE;
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    case IRQ_TYPE_LEVEL_LOW:
// BCM GPIO doesn't support level triggering
    default:
    dev_err(kona_gpio.gpio_chip.parent,
    "Invalid BCM GPIO irq type 0x%x\n", type);
    return -EINVAL;
    }
    guard(raw_spinlock_irqsave)(&kona_gpio.lock);
    val = readl(reg_base + GPIO_CONTROL(gpio));
    val &= ~GPIO_GPCTR0_ITR_MASK;
    val |= lvl_type << GPIO_GPCTR0_ITR_SHIFT;
    writel(val, reg_base + GPIO_CONTROL(gpio));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm_kona_gpio_irq_handler(desc: *mut irq_desc) {
    static void bcm_kona_gpio_irq_handler(struct irq_desc *desc)
    {
    void __iomem *reg_base;
    int bit, bank_id;
    unsigned long sta;
    struct bcm_kona_gpio_bank *bank = irq_desc_get_handler_data(desc);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    chained_irq_enter(chip, desc);
//
// For bank interrupts, we can't use chip_data to store the kona_gpio
// pointer, since GIC needs it for its own purposes. Therefore, we get
// our pointer from the bank structure.
//
    reg_base = bank.kona_gpio.reg_base;
    bank_id = bank.id;
    while ((sta = readl(reg_base + GPIO_INT_STATUS(bank_id)) &
    (~(readl(reg_base + GPIO_INT_MASK(bank_id)))))) {
    for_each_set_bit(bit, &sta, 32) {
    let mut hwirq: c_int = GPIO_PER_BANK * bank_id + bit;
//
// Clear interrupt before handler is called so we don't
// miss any interrupt occurred during executing them.
//
    writel(readl(reg_base + GPIO_INT_STATUS(bank_id)) |
    BIT(bit), reg_base + GPIO_INT_STATUS(bank_id));
// Invoke interrupt handler
    generic_handle_domain_irq(bank.kona_gpio.irq_domain,
    hwirq);
    }
    }
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn bcm_kona_gpio_irq_reqres(d: *mut irq_data) -> c_int {
    static int bcm_kona_gpio_irq_reqres(struct irq_data *d)
    {
    struct bcm_kona_gpio *kona_gpio = irq_data_get_irq_chip_data(d);
    let mut gpio: c_uint = d.hwirq;
//
// We need to unlock the GPIO before any other operations are performed
// on the relevant GPIO configuration registers
//
    bcm_kona_gpio_unlock_gpio(kona_gpio, gpio);
    return gpiochip_reqres_irq(&kona_gpio.gpio_chip, gpio);
    }
#[no_mangle]
unsafe extern "C" fn bcm_kona_gpio_irq_relres(d: *mut irq_data) {
    static void bcm_kona_gpio_irq_relres(struct irq_data *d)
    {
    struct bcm_kona_gpio *kona_gpio = irq_data_get_irq_chip_data(d);
    let mut gpio: c_uint = d.hwirq;
// Once we no longer use it, lock the GPIO again
    bcm_kona_gpio_lock_gpio(kona_gpio, gpio);
    gpiochip_relres_irq(&kona_gpio.gpio_chip, gpio);
    }
    static struct irq_chip bcm_gpio_irq_chip = {
    .name = "bcm-kona-gpio",
    .irq_ack = bcm_kona_gpio_irq_ack,
    .irq_mask = bcm_kona_gpio_irq_mask,
    .irq_unmask = bcm_kona_gpio_irq_unmask,
    .irq_set_type = bcm_kona_gpio_irq_set_type,
    .irq_request_resources = bcm_kona_gpio_irq_reqres,
    .irq_release_resources = bcm_kona_gpio_irq_relres,
    .flags = IRQCHIP_IMMUTABLE,
    };
    static struct of_device_id const bcm_kona_gpio_of_match[] = {
    { .compatible = "brcm,kona-gpio" },
    {}
    };
//
// This lock class tells lockdep that GPIO irqs are in a different
// category than their parents, so it won't report false recursion.
//
    static struct lock_class_key gpio_lock_class;
    static struct lock_class_key gpio_request_class;
    static int bcm_kona_gpio_irq_map(struct irq_domain *d, unsigned int irq,
    irq_hw_number_t hwirq)
    {
    int ret;
    ret = irq_set_chip_data(irq, d.host_data);
    if (ret < 0)
    return ret;
    irq_set_lockdep_class(irq, &gpio_lock_class, &gpio_request_class);
    irq_set_chip_and_handler(irq, &bcm_gpio_irq_chip, handle_simple_irq);
    irq_set_noprobe(irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm_kona_gpio_irq_unmap(d: *mut irq_domain, irq: c_uint) {
    static void bcm_kona_gpio_irq_unmap(struct irq_domain *d, unsigned int irq)
    {
    irq_set_chip_and_handler(irq, core::ptr::null_mut(), core::ptr::null_mut());
    irq_set_chip_data(irq, core::ptr::null_mut());
    }
    static const struct irq_domain_ops bcm_kona_irq_ops = {
    .map = bcm_kona_gpio_irq_map,
    .unmap = bcm_kona_gpio_irq_unmap,
    .xlate = irq_domain_xlate_twocell,
    };
#[no_mangle]
unsafe extern "C" fn bcm_kona_gpio_reset(kona_gpio: *mut bcm_kona_gpio) {
    static void bcm_kona_gpio_reset(struct bcm_kona_gpio *kona_gpio)
    {
    void __iomem *reg_base;
    int i;
    reg_base = kona_gpio.reg_base;
// disable interrupts and clear status
    for (i = 0; i < kona_gpio.num_bank; i++) {
// Unlock the entire bank first
    bcm_kona_gpio_write_lock_regs(reg_base, i, UNLOCK_CODE);
    writel(0xffffffff, reg_base + GPIO_INT_MASK(i));
    writel(0xffffffff, reg_base + GPIO_INT_STATUS(i));
// Now re-lock the bank
    bcm_kona_gpio_write_lock_regs(reg_base, i, LOCK_CODE);
    }
    }
#[no_mangle]
unsafe extern "C" fn bcm_kona_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int bcm_kona_gpio_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct bcm_kona_gpio_bank *bank;
    struct bcm_kona_gpio *kona_gpio;
    struct gpio_chip *chip;
    int ret;
    int i;
    ret = platform_irq_count(pdev);
    if (!ret) {
    dev_err(dev, "Couldn't determine # GPIO banks\n");
    return -ENOENT;
    } else if (ret < 0) {
    return dev_err_probe(dev, ret, "Couldn't determine GPIO banks\n");
    }
    kona_gpio = devm_kzalloc(dev, struct_size(kona_gpio, banks, ret), GFP_KERNEL);
    if (!kona_gpio)
    return -ENOMEM;
    kona_gpio.num_bank = ret;
    if (kona_gpio.num_bank > GPIO_MAX_BANK_NUM) {
    dev_err(dev, "Too many GPIO banks configured (max=%d)\n",
    GPIO_MAX_BANK_NUM);
    return -ENXIO;
    }
    kona_gpio.gpio_chip = template_chip;
    chip = &kona_gpio.gpio_chip;
    chip.parent = dev;
    chip.ngpio = kona_gpio.num_bank * GPIO_PER_BANK;
    kona_gpio.irq_domain = irq_domain_create_linear(dev_fwnode(dev),
    chip.ngpio,
    &bcm_kona_irq_ops,
    kona_gpio);
    if (!kona_gpio.irq_domain) {
    dev_err(dev, "Couldn't allocate IRQ domain\n");
    return -ENXIO;
    }
    kona_gpio.reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(kona_gpio.reg_base)) {
    ret = PTR_ERR(kona_gpio.reg_base);
    goto err_irq_domain;
    }
    for (i = 0; i < kona_gpio.num_bank; i++) {
    bank = &kona_gpio.banks[i];
    bank.id = i;
    bank.irq = platform_get_irq(pdev, i);
    bank.kona_gpio = kona_gpio;
    if (bank.irq < 0) {
    dev_err(dev, "Couldn't get IRQ for bank %d\n", i);
    ret = -ENOENT;
    goto err_irq_domain;
    }
    }
    dev_info(&pdev.dev, "Setting up Kona GPIO\n");
    bcm_kona_gpio_reset(kona_gpio);
    ret = devm_gpiochip_add_data(dev, chip, kona_gpio);
    if (ret < 0) {
    dev_err(dev, "Couldn't add GPIO chip -- %d\n", ret);
    goto err_irq_domain;
    }
    for (i = 0; i < kona_gpio.num_bank; i++) {
    bank = &kona_gpio.banks[i];
    irq_set_chained_handler_and_data(bank.irq,
    bcm_kona_gpio_irq_handler,
    bank);
    }
    raw_spin_lock_init(&kona_gpio.lock);
    return 0;
    err_irq_domain:
    irq_domain_remove(kona_gpio.irq_domain);
    return ret;
    }
    static struct platform_driver bcm_kona_gpio_driver = {
    .driver = {
    .name = "bcm-kona-gpio",
    .of_match_table = bcm_kona_gpio_of_match,
    },
    .probe = bcm_kona_gpio_probe,
    };
    builtin_platform_driver(bcm_kona_gpio_driver);
