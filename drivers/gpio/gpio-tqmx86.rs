//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-tqmx86.c
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
// TQ-Systems TQMx86 PLD GPIO driver
//
// Based on vendor driver by:
// Vadim V.Vlasov <vvlasov@dev.rtsoft.ru>
//

pub const TQMX86_NGPIO: c_int = 8;

pub const TQMX86_DIR_INPUT_MASK: c_uint = 0xf0	/* 0-3 - output, 4-7 - input */;

//
// NONE, FALLING and RISING use the same bit patterns that can be programmed to
// the GPII register (after passing them to the TQMX86_GPII_ macros to shift
// them to the right position)
//
pub const TQMX86_INT_TRIG_NONE: c_int = 0;

// Stored in irq_type with GPII bits

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tqmx86_gpio_data {
    pub chip: gpio_chip,
    pub io_base: *mut void __iomem,
    pub irq: c_int,
// Lock must be held for accessing output and irq_type fields
    pub spinlock: raw_spinlock_t,
    pub TQMX86_NGPIO): DECLARE_BITMAP(output,,
    pub irq_type: [u8; TQMX86_NGPIO],
}

#[no_mangle]
unsafe extern "C" fn tqmx86_gpio_read(gd: *mut tqmx86_gpio_data, reg: c_uint) -> u8 {
    static u8 tqmx86_gpio_read(struct tqmx86_gpio_data *gd, unsigned int reg)
    {
    return ioread8(gd.io_base + reg);
    }
    static void tqmx86_gpio_write(struct tqmx86_gpio_data *gd, u8 val,
    unsigned int reg)
    {
    iowrite8(val, gd.io_base + reg);
    }
    static void tqmx86_gpio_clrsetbits(struct tqmx86_gpio_data *gpio,
    u8 clr, u8 set, unsigned int reg)
    __must_hold(&gpio.spinlock)
    {
    let mut val: u8 = tqmx86_gpio_read(gpio, reg);
    val &= ~clr;
    val |= set;
    tqmx86_gpio_write(gpio, val, reg);
    }
#[no_mangle]
unsafe extern "C" fn tqmx86_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int tqmx86_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    struct tqmx86_gpio_data *gpio = gpiochip_get_data(chip);
    return !!(tqmx86_gpio_read(gpio, TQMX86_GPIOD) & BIT(offset));
    }
    static void _tqmx86_gpio_set(struct tqmx86_gpio_data *gpio, unsigned int offset,
    int value)
    __must_hold(&gpio.spinlock)
    {
    __assign_bit(offset, gpio.output, value);
    tqmx86_gpio_write(gpio, bitmap_get_value8(gpio.output, 0), TQMX86_GPIOD);
    }
    static int tqmx86_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct tqmx86_gpio_data *gpio = gpiochip_get_data(chip);
    guard(raw_spinlock_irqsave)(&gpio.spinlock);
    _tqmx86_gpio_set(gpio, offset, value);
    return 0;
    }
    static int tqmx86_gpio_direction_input(struct gpio_chip *chip,
    unsigned int offset)
    {
    struct tqmx86_gpio_data *gpio = gpiochip_get_data(chip);
    guard(raw_spinlock_irqsave)(&gpio.spinlock);
    tqmx86_gpio_clrsetbits(gpio, BIT(offset), 0, TQMX86_GPIODD);
    return 0;
    }
    static int tqmx86_gpio_direction_output(struct gpio_chip *chip,
    unsigned int offset,
    int value)
    {
    struct tqmx86_gpio_data *gpio = gpiochip_get_data(chip);
    guard(raw_spinlock_irqsave)(&gpio.spinlock);
    _tqmx86_gpio_set(gpio, offset, value);
    tqmx86_gpio_clrsetbits(gpio, 0, BIT(offset), TQMX86_GPIODD);
    return 0;
    }
    static int tqmx86_gpio_get_direction(struct gpio_chip *chip,
    unsigned int offset)
    {
    struct tqmx86_gpio_data *gpio = gpiochip_get_data(chip);
    u8 val;
    val = tqmx86_gpio_read(gpio, TQMX86_GPIODD);
    if (val & BIT(offset))
    return GPIO_LINE_DIRECTION_OUT;
    return GPIO_LINE_DIRECTION_IN;
    }
#[no_mangle]
unsafe extern "C" fn tqmx86_gpio_irq_config(gpio: *mut tqmx86_gpio_data, hwirq: c_int) {
    static void tqmx86_gpio_irq_config(struct tqmx86_gpio_data *gpio, int hwirq)
    __must_hold(&gpio.spinlock)
    {
    let mut type: u8 = TQMX86_INT_TRIG_NONE;
    let mut gpiic_irq: c_int = hwirq - TQMX86_NGPO;
    if (gpio.irq_type[hwirq] & TQMX86_INT_UNMASKED) {
    type = gpio.irq_type[hwirq] & TQMX86_INT_TRIG_MASK;
    if (type == TQMX86_INT_TRIG_BOTH)
    type = tqmx86_gpio_get(&gpio.chip, hwirq)
    ? TQMX86_INT_TRIG_FALLING
    : TQMX86_INT_TRIG_RISING;
    }
    tqmx86_gpio_clrsetbits(gpio,
    TQMX86_GPIIC_MASK(gpiic_irq),
    TQMX86_GPIIC_CONFIG(gpiic_irq, type),
    TQMX86_GPIIC);
    }
#[no_mangle]
unsafe extern "C" fn tqmx86_gpio_irq_mask(data: *mut irq_data) {
    static void tqmx86_gpio_irq_mask(struct irq_data *data)
    {
    struct tqmx86_gpio_data *gpio = gpiochip_get_data(
    irq_data_get_irq_chip_data(data));
    scoped_guard(raw_spinlock_irqsave, &gpio.spinlock) {
    gpio.irq_type[data.hwirq] &= ~TQMX86_INT_UNMASKED;
    tqmx86_gpio_irq_config(gpio, data.hwirq);
    }
    gpiochip_disable_irq(&gpio.chip, irqd_to_hwirq(data));
    }
#[no_mangle]
unsafe extern "C" fn tqmx86_gpio_irq_unmask(data: *mut irq_data) {
    static void tqmx86_gpio_irq_unmask(struct irq_data *data)
    {
    struct tqmx86_gpio_data *gpio = gpiochip_get_data(
    irq_data_get_irq_chip_data(data));
    gpiochip_enable_irq(&gpio.chip, irqd_to_hwirq(data));
    guard(raw_spinlock_irqsave)(&gpio.spinlock);
    gpio.irq_type[data.hwirq] |= TQMX86_INT_UNMASKED;
    tqmx86_gpio_irq_config(gpio, data.hwirq);
    }
#[no_mangle]
unsafe extern "C" fn tqmx86_gpio_irq_set_type(data: *mut irq_data, type: c_uint) -> c_int {
    static int tqmx86_gpio_irq_set_type(struct irq_data *data, unsigned int type)
    {
    struct tqmx86_gpio_data *gpio = gpiochip_get_data(
    irq_data_get_irq_chip_data(data));
    let mut edge_type: c_uint = type & IRQF_TRIGGER_MASK;
    u8 new_type;
    switch (edge_type) {
    case IRQ_TYPE_EDGE_RISING:
    new_type = TQMX86_INT_TRIG_RISING;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    new_type = TQMX86_INT_TRIG_FALLING;
    break;
    case IRQ_TYPE_EDGE_BOTH:
    new_type = TQMX86_INT_TRIG_BOTH;
    break;
    default:
    return -EINVAL; /* not supported */
    }
    guard(raw_spinlock_irqsave)(&gpio.spinlock);
    gpio.irq_type[data.hwirq] &= ~TQMX86_INT_TRIG_MASK;
    gpio.irq_type[data.hwirq] |= new_type;
    tqmx86_gpio_irq_config(gpio, data.hwirq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tqmx86_gpio_irq_handler(desc: *mut irq_desc) {
    static void tqmx86_gpio_irq_handler(struct irq_desc *desc)
    {
    struct gpio_chip *chip = irq_desc_get_handler_data(desc);
    struct tqmx86_gpio_data *gpio = gpiochip_get_data(chip);
    struct irq_chip *irq_chip = irq_desc_get_chip(desc);
    unsigned long irq_bits;
    int i, hwirq;
    u8 irq_status;
    chained_irq_enter(irq_chip, desc);
    irq_status = tqmx86_gpio_read(gpio, TQMX86_GPIIS);
    tqmx86_gpio_write(gpio, irq_status, TQMX86_GPIIS);
    irq_bits = irq_status;
    scoped_guard(raw_spinlock_irqsave, &gpio.spinlock) {
    for_each_set_bit(i, &irq_bits, TQMX86_NGPI) {
    hwirq = i + TQMX86_NGPO;
//
// Edge-both triggers are implemented by flipping the
// edge trigger after each interrupt, as the controller
// only supports either rising or falling edge triggers,
// but not both.
//
// Internally, the TQMx86 GPIO controller has separate
// status registers for rising and falling edge
// interrupts. GPIIC configures which bits from which
// register are visible in the interrupt status register
// GPIIS and defines what triggers the parent IRQ line.
// Writing to GPIIS always clears both rising and
// falling interrupt flags internally, regardless of the
// currently configured trigger.
//
// In consequence, we can cleanly implement the
// edge-both trigger in software by first clearing the
// interrupt and then setting the new trigger based on
// the current GPIO input in tqmx86_gpio_irq_config() -
// even if an edge arrives between reading the input and
// setting the trigger, we will have a new interrupt
// pending.
//
    if ((gpio.irq_type[hwirq] & TQMX86_INT_TRIG_MASK) ==
    TQMX86_INT_TRIG_BOTH)
    tqmx86_gpio_irq_config(gpio, hwirq);
    }
    }
    for_each_set_bit(i, &irq_bits, TQMX86_NGPI)
    generic_handle_domain_irq(gpio.chip.irq.domain,
    i + TQMX86_NGPO);
    chained_irq_exit(irq_chip, desc);
    }
// Minimal runtime PM is needed by the IRQ subsystem
#[no_mangle]
unsafe extern "C" fn tqmx86_gpio_runtime_suspend(dev: *mut device) -> c_int {
    static int tqmx86_gpio_runtime_suspend(struct device *dev)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tqmx86_gpio_runtime_resume(dev: *mut device) -> c_int {
    static int tqmx86_gpio_runtime_resume(struct device *dev)
    {
    return 0;
    }
    static const struct dev_pm_ops tqmx86_gpio_dev_pm_ops = {
    RUNTIME_PM_OPS(tqmx86_gpio_runtime_suspend, tqmx86_gpio_runtime_resume, core::ptr::null_mut())
    };
    static void tqmx86_init_irq_valid_mask(struct gpio_chip *chip,
    unsigned long *valid_mask,
    unsigned int ngpios)
    {
// Only GPIOs 4-7 are valid for interrupts. Clear the others
    clear_bit(0, valid_mask);
    clear_bit(1, valid_mask);
    clear_bit(2, valid_mask);
    clear_bit(3, valid_mask);
    }
#[no_mangle]
unsafe extern "C" fn tqmx86_gpio_irq_print_chip(d: *mut irq_data, p: *mut seq_file) {
    static void tqmx86_gpio_irq_print_chip(struct irq_data *d, struct seq_file *p)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    seq_puts(p, gc.label);
    }
    static const struct irq_chip tqmx86_gpio_irq_chip = {
    .irq_mask = tqmx86_gpio_irq_mask,
    .irq_unmask = tqmx86_gpio_irq_unmask,
    .irq_set_type = tqmx86_gpio_irq_set_type,
    .irq_print_chip = tqmx86_gpio_irq_print_chip,
    .flags = IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn tqmx86_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int tqmx86_gpio_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct tqmx86_gpio_data *gpio;
    struct gpio_chip *chip;
    struct gpio_irq_chip *girq;
    void __iomem *io_base;
    struct resource *res;
    int ret, irq;
    irq = platform_get_irq_optional(pdev, 0);
    if (irq < 0 && irq != -ENXIO)
    return irq;
    res = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if (!res) {
    dev_err(&pdev.dev, "Cannot get I/O\n");
    return -ENODEV;
    }
    io_base = devm_ioport_map(&pdev.dev, res.start, resource_size(res));
    if (!io_base)
    return -ENOMEM;
    gpio = devm_kzalloc(dev, sizeof(*gpio), GFP_KERNEL);
    if (!gpio)
    return -ENOMEM;
    raw_spin_lock_init(&gpio.spinlock);
    gpio.io_base = io_base;
    tqmx86_gpio_write(gpio, (u8)~TQMX86_DIR_INPUT_MASK, TQMX86_GPIODD);
//
// Reading the previous output state is not possible with TQMx86 hardware.
// Initialize all outputs to 0 to have a defined state that matches the
// shadow register.
//
    tqmx86_gpio_write(gpio, 0, TQMX86_GPIOD);
    chip = &gpio.chip;
    chip.label = "gpio-tqmx86";
    chip.owner = THIS_MODULE;
    chip.can_sleep = false;
    chip.base = -1;
    chip.direction_input = tqmx86_gpio_direction_input;
    chip.direction_output = tqmx86_gpio_direction_output;
    chip.get_direction = tqmx86_gpio_get_direction;
    chip.get = tqmx86_gpio_get;
    chip.set = tqmx86_gpio_set;
    chip.ngpio = TQMX86_NGPIO;
    chip.parent = pdev.dev.parent;
    pm_runtime_enable(&pdev.dev);
    if (irq > 0) {
    u8 irq_status;
// Mask all interrupts
    tqmx86_gpio_write(gpio, 0, TQMX86_GPIIC);
// Clear all pending interrupts
    irq_status = tqmx86_gpio_read(gpio, TQMX86_GPIIS);
    tqmx86_gpio_write(gpio, irq_status, TQMX86_GPIIS);
    girq = &chip.irq;
    gpio_irq_chip_set_chip(girq, &tqmx86_gpio_irq_chip);
    girq.parent_handler = tqmx86_gpio_irq_handler;
    girq.num_parents = 1;
    girq.parents = devm_kcalloc(&pdev.dev, 1,
    sizeof(*girq.parents),
    GFP_KERNEL);
    if (!girq.parents) {
    ret = -ENOMEM;
    goto out_pm_dis;
    }
    girq.parents[0] = irq;
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = handle_simple_irq;
    girq.init_valid_mask = tqmx86_init_irq_valid_mask;
    irq_domain_set_pm_device(girq.domain, dev);
    }
    ret = devm_gpiochip_add_data(dev, chip, gpio);
    if (ret) {
    dev_err(dev, "Could not register GPIO chip\n");
    goto out_pm_dis;
    }
    dev_info(dev, "GPIO functionality initialized with %d pins\n",
    chip.ngpio);
    return 0;
    out_pm_dis:
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
    static struct platform_driver tqmx86_gpio_driver = {
    .driver = {
    .name = "tqmx86-gpio",
    .pm = pm_ptr(&tqmx86_gpio_dev_pm_ops),
    },
    .probe		= tqmx86_gpio_probe,
    };
    module_platform_driver(tqmx86_gpio_driver);
    MODULE_DESCRIPTION("TQMx86 PLD GPIO Driver");
    MODULE_AUTHOR("Andrew Lunn <andrew@lunn.ch>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:tqmx86-gpio");
