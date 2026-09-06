//! Automatically rewritten from C to Rust
//! Source: drivers/bcma/driver_gpio.c
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


//
// Broadcom specific AMBA
// GPIO driver
//
// Copyright 2011, Broadcom Corporation
// Copyright 2012, Hauke Mehrtens <hauke@hauke-m.de>
//
// Licensed under the GNU/GPL. See COPYING for details.
//

pub const BCMA_GPIO_MAX_PINS: c_int = 32;
    const struct software_node bcma_gpio_swnode = {
    .name = "bcma-gpio",
    };
    EXPORT_SYMBOL_GPL(bcma_gpio_swnode);
#[no_mangle]
unsafe extern "C" fn bcma_gpio_get_value(chip: *mut gpio_chip, gpio: unsigned) -> c_int {
    static int bcma_gpio_get_value(struct gpio_chip *chip, unsigned gpio)
    {
    struct bcma_drv_cc *cc = gpiochip_get_data(chip);
    return !!bcma_chipco_gpio_in(cc, 1 << gpio);
    }
    static int bcma_gpio_set_value(struct gpio_chip *chip, unsigned int gpio,
    int value)
    {
    struct bcma_drv_cc *cc = gpiochip_get_data(chip);
    bcma_chipco_gpio_out(cc, 1 << gpio, value ? 1 << gpio : 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcma_gpio_direction_input(chip: *mut gpio_chip, gpio: unsigned) -> c_int {
    static int bcma_gpio_direction_input(struct gpio_chip *chip, unsigned gpio)
    {
    struct bcma_drv_cc *cc = gpiochip_get_data(chip);
    bcma_chipco_gpio_outen(cc, 1 << gpio, 0);
    return 0;
    }
    static int bcma_gpio_direction_output(struct gpio_chip *chip, unsigned gpio,
    int value)
    {
    struct bcma_drv_cc *cc = gpiochip_get_data(chip);
    bcma_chipco_gpio_outen(cc, 1 << gpio, 1 << gpio);
    bcma_chipco_gpio_out(cc, 1 << gpio, value ? 1 << gpio : 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcma_gpio_request(chip: *mut gpio_chip, gpio: unsigned) -> c_int {
    static int bcma_gpio_request(struct gpio_chip *chip, unsigned gpio)
    {
    struct bcma_drv_cc *cc = gpiochip_get_data(chip);
    bcma_chipco_gpio_control(cc, 1 << gpio, 0);
// clear pulldown
    bcma_chipco_gpio_pulldown(cc, 1 << gpio, 0);
// Set pullup
    bcma_chipco_gpio_pullup(cc, 1 << gpio, 1 << gpio);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcma_gpio_free(chip: *mut gpio_chip, gpio: unsigned) {
    static void bcma_gpio_free(struct gpio_chip *chip, unsigned gpio)
    {
    struct bcma_drv_cc *cc = gpiochip_get_data(chip);
// clear pullup
    bcma_chipco_gpio_pullup(cc, 1 << gpio, 0);
    }

#[no_mangle]
unsafe extern "C" fn bcma_gpio_irq_unmask(d: *mut irq_data) {
    static void bcma_gpio_irq_unmask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct bcma_drv_cc *cc = gpiochip_get_data(gc);
    let mut gpio: c_int = irqd_to_hwirq(d);
    let mut val: u32 = bcma_chipco_gpio_in(cc, BIT(gpio));
    gpiochip_enable_irq(gc, gpio);
    bcma_chipco_gpio_polarity(cc, BIT(gpio), val);
    bcma_chipco_gpio_intmask(cc, BIT(gpio), BIT(gpio));
    }
#[no_mangle]
unsafe extern "C" fn bcma_gpio_irq_mask(d: *mut irq_data) {
    static void bcma_gpio_irq_mask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct bcma_drv_cc *cc = gpiochip_get_data(gc);
    let mut gpio: c_int = irqd_to_hwirq(d);
    bcma_chipco_gpio_intmask(cc, BIT(gpio), 0);
    gpiochip_disable_irq(gc, gpio);
    }
    static const struct irq_chip bcma_gpio_irq_chip = {
    .name		= "BCMA-GPIO",
    .irq_mask	= bcma_gpio_irq_mask,
    .irq_unmask	= bcma_gpio_irq_unmask,
    .flags		= IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn bcma_gpio_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t bcma_gpio_irq_handler(int irq, void *dev_id)
    {
    struct bcma_drv_cc *cc = dev_id;
    struct gpio_chip *gc = &cc.gpio;
    let mut val: u32 = bcma_cc_read32(cc, BCMA_CC_GPIOIN);
    let mut mask: u32 = bcma_cc_read32(cc, BCMA_CC_GPIOIRQ);
    let mut pol: u32 = bcma_cc_read32(cc, BCMA_CC_GPIOPOL);
    let mut irqs: c_ulong = (val ^ pol) & mask;
    int gpio;
    if (!irqs)
    return IRQ_NONE;
    for_each_set_bit(gpio, &irqs, gc.ngpio)
    generic_handle_domain_irq_safe(gc.irq.domain, gpio);
    bcma_chipco_gpio_polarity(cc, irqs, val & irqs);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn bcma_gpio_irq_init(cc: *mut bcma_drv_cc) -> c_int {
    static int bcma_gpio_irq_init(struct bcma_drv_cc *cc)
    {
    struct gpio_chip *chip = &cc.gpio;
    struct gpio_irq_chip *girq = &chip.irq;
    int hwirq, err;
    if (cc.core.bus.hosttype != BCMA_HOSTTYPE_SOC)
    return 0;
    hwirq = bcma_core_irq(cc.core, 0);
    err = request_irq(hwirq, bcma_gpio_irq_handler, IRQF_SHARED, "gpio",
    cc);
    if (err)
    return err;
    bcma_chipco_gpio_intmask(cc, ~0, 0);
    bcma_cc_set32(cc, BCMA_CC_IRQMASK, BCMA_CC_IRQ_GPIO);
    gpio_irq_chip_set_chip(girq, &bcma_gpio_irq_chip);
// This will let us handle the parent IRQ in the driver
    girq.parent_handler = core::ptr::null_mut();
    girq.num_parents = 0;
    girq.parents = core::ptr::null_mut();
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = handle_simple_irq;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcma_gpio_irq_exit(cc: *mut bcma_drv_cc) {
    static void bcma_gpio_irq_exit(struct bcma_drv_cc *cc)
    {
    if (cc.core.bus.hosttype != BCMA_HOSTTYPE_SOC)
    return;
    bcma_cc_mask32(cc, BCMA_CC_IRQMASK, ~BCMA_CC_IRQ_GPIO);
    free_irq(bcma_core_irq(cc.core, 0), cc);
    }

#[no_mangle]
unsafe extern "C" fn bcma_gpio_irq_init(cc: *mut bcma_drv_cc) -> c_int {
    static int bcma_gpio_irq_init(struct bcma_drv_cc *cc)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcma_gpio_irq_exit(cc: *mut bcma_drv_cc) {
    static void bcma_gpio_irq_exit(struct bcma_drv_cc *cc)
    {
    }

#[no_mangle]
pub unsafe extern "C" fn bcma_gpio_init(cc: *mut bcma_drv_cc) -> c_int {
    int bcma_gpio_init(struct bcma_drv_cc *cc)
    {
    struct bcma_bus *bus = cc.core.bus;
    struct gpio_chip *chip = &cc.gpio;
    int err;
    chip.label		= "bcma_gpio";
    chip.owner		= THIS_MODULE;
    chip.request		= bcma_gpio_request;
    chip.free		= bcma_gpio_free;
    chip.get		= bcma_gpio_get_value;
    chip.set		= bcma_gpio_set_value;
    chip.direction_input	= bcma_gpio_direction_input;
    chip.direction_output	= bcma_gpio_direction_output;
    chip.parent		= bus.dev;
//
// Register software node only for the host SoC bus, unless there is
// already a firmware node assigned. There is only one SoC instance
// in the system, so there are no concerns with registration conflicts.
//
    if (bus.hosttype == BCMA_HOSTTYPE_SOC && !dev_fwnode(&cc.core.dev)) {
    err = software_node_register(&bcma_gpio_swnode);
    if (err)
    return err;
    chip.fwnode = software_node_fwnode(&bcma_gpio_swnode);
    } else {
    chip.fwnode = dev_fwnode(&cc.core.dev);
    }
    switch (bus.chipinfo.id) {
    case BCMA_CHIP_ID_BCM4707:
    case BCMA_CHIP_ID_BCM5357:
    case BCMA_CHIP_ID_BCM53572:
    case BCMA_CHIP_ID_BCM53573:
    case BCMA_CHIP_ID_BCM47094:
    chip.ngpio	= 32;
    break;
    default:
    chip.ngpio	= 16;
    }
//
// Register SoC GPIO devices with absolute GPIO pin base.
// On MIPS, we don't have Device Tree and we can't use relative (per chip)
// GPIO numbers.
// On some ARM devices, user space may want to access some system GPIO
// pins directly, which is easier to do with a predictable GPIO base.
//
    if (IS_BUILTIN(CONFIG_BCM47XX) ||
    cc.core.bus.hosttype == BCMA_HOSTTYPE_SOC)
    chip.base		= bus.num * BCMA_GPIO_MAX_PINS;
    else
    chip.base		= -1;
    err = bcma_gpio_irq_init(cc);
    if (err)
    goto err_unregister_swnode;
    err = gpiochip_add_data(chip, cc);
    if (err)
    goto err_irq_exit;
    return 0;
    err_irq_exit:
    bcma_gpio_irq_exit(cc);
    err_unregister_swnode:
    if (bus.hosttype == BCMA_HOSTTYPE_SOC &&
    chip.fwnode && is_software_node(chip.fwnode)) {
    software_node_unregister(&bcma_gpio_swnode);
    chip.fwnode = core::ptr::null_mut();
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn bcma_gpio_unregister(cc: *mut bcma_drv_cc) -> c_int {
    int bcma_gpio_unregister(struct bcma_drv_cc *cc)
    {
    bcma_gpio_irq_exit(cc);
    gpiochip_remove(&cc.gpio);
    if (cc.core.bus.hosttype == BCMA_HOSTTYPE_SOC &&
    cc.gpio.fwnode && is_software_node(cc.gpio.fwnode)) {
    software_node_unregister(&bcma_gpio_swnode);
    cc.gpio.fwnode = core::ptr::null_mut();
    }
    return 0;
    }
