//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-htc-egpio.c
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
// Support for the GPIO/IRQ expander chips present on several HTC phones.
// These are implemented in CPLD chips present on the board.
//
// Copyright (c) 2007 Kevin O'Connor <kevin@koconnor.net>
// Copyright (c) 2007 Philipp Zabel <philipp.zabel@gmail.com>
//
// This file may be distributed under the terms of the GNU GPL license.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct egpio_chip {
    pub reg_start: c_int,
    pub cached_values: c_int,
    pub is_out: c_ulong,
    pub dev: *mut device,
    pub chip: gpio_chip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct egpio_info {
    pub lock: spinlock_t,
// iomem info
    pub base_addr: *mut void __iomem,
    pub /: *mut *mut int bus_shift; / byte shift,
    pub /: *mut *mut int reg_shift; / bit shift,
    pub reg_mask: c_int,
// irq info
    pub ack_register: c_int,
    pub ack_write: c_int,
    pub irqs_enabled: u16,
    pub irq_start: c_uint,
    pub nirqs: c_int,
    pub chained_irq: c_uint,
// egpio info
    pub nchips: c_int,
    pub __counted_by(nchips): egpio_chip chip[],
}

#[no_mangle]
pub unsafe extern "C" fn egpio_writew(value: u16, ei: *mut egpio_info, reg: c_int) {
    static inline void egpio_writew(u16 value, struct egpio_info *ei, int reg)
    {
    writew(value, ei.base_addr + (reg << ei.bus_shift));
    }
#[no_mangle]
pub unsafe extern "C" fn egpio_readw(ei: *mut egpio_info, reg: c_int) -> u16 {
    static inline u16 egpio_readw(struct egpio_info *ei, int reg)
    {
    return readw(ei.base_addr + (reg << ei.bus_shift));
    }
//
// IRQs
//
#[no_mangle]
pub unsafe extern "C" fn ack_irqs(ei: *mut egpio_info) {
    static inline void ack_irqs(struct egpio_info *ei)
    {
    egpio_writew(ei.ack_write, ei, ei.ack_register);
    pr_debug("EGPIO ack - write %x to base+%x\n",
    ei.ack_write, ei.ack_register << ei.bus_shift);
    }
#[no_mangle]
unsafe extern "C" fn egpio_ack(data: *mut irq_data) {
    static void egpio_ack(struct irq_data *data)
    {
    }
// There does not appear to be a way to proactively mask interrupts
// on the egpio chip itself.  So, we simply ignore interrupts that
// aren't desired.
#[no_mangle]
unsafe extern "C" fn egpio_mask(data: *mut irq_data) {
    static void egpio_mask(struct irq_data *data)
    {
    struct egpio_info *ei = irq_data_get_irq_chip_data(data);
    ei.irqs_enabled &= ~(1 << (data.irq - ei.irq_start));
    pr_debug("EGPIO mask %d %04x\n", data.irq, ei.irqs_enabled);
    }
#[no_mangle]
unsafe extern "C" fn egpio_unmask(data: *mut irq_data) {
    static void egpio_unmask(struct irq_data *data)
    {
    struct egpio_info *ei = irq_data_get_irq_chip_data(data);
    ei.irqs_enabled |= 1 << (data.irq - ei.irq_start);
    pr_debug("EGPIO unmask %d %04x\n", data.irq, ei.irqs_enabled);
    }
    static struct irq_chip egpio_muxed_chip = {
    .name		= "htc-egpio",
    .irq_ack	= egpio_ack,
    .irq_mask	= egpio_mask,
    .irq_unmask	= egpio_unmask,
    };
#[no_mangle]
unsafe extern "C" fn egpio_handler(desc: *mut irq_desc) {
    static void egpio_handler(struct irq_desc *desc)
    {
    struct egpio_info *ei = irq_desc_get_handler_data(desc);
    int irqpin;
// Read current pins.
    let mut readval: c_ulong = egpio_readw(ei, ei.ack_register);
    pr_debug("IRQ reg: %x\n", (unsigned int)readval);
// Ack/unmask interrupts.
    ack_irqs(ei);
// Process all set pins.
    readval &= ei.irqs_enabled;
    for_each_set_bit(irqpin, &readval, ei.nirqs) {
// Run irq handler
    pr_debug("got IRQ %d\n", irqpin);
    generic_handle_irq(ei.irq_start + irqpin);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn egpio_pos(ei: *mut egpio_info, bit: c_int) -> c_int {
    static inline int egpio_pos(struct egpio_info *ei, int bit)
    {
    return bit >> ei.reg_shift;
    }
#[no_mangle]
pub unsafe extern "C" fn egpio_bit(ei: *mut egpio_info, bit: c_int) -> c_int {
    static inline int egpio_bit(struct egpio_info *ei, int bit)
    {
    return 1 << (bit & ((1 << ei.reg_shift)-1));
    }
//
// Input pins
//
#[no_mangle]
unsafe extern "C" fn egpio_get(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int egpio_get(struct gpio_chip *chip, unsigned offset)
    {
    struct egpio_chip *egpio;
    struct egpio_info *ei;
    unsigned           bit;
    int                reg;
    int                value;
    pr_debug("egpio_get_value(%d)\n", chip.base + offset);
    egpio = gpiochip_get_data(chip);
    ei    = dev_get_drvdata(egpio.dev);
    bit   = egpio_bit(ei, offset);
    reg   = egpio.reg_start + egpio_pos(ei, offset);
    if (test_bit(offset, &egpio.is_out)) {
    return !!(egpio.cached_values & (1 << offset));
    } else {
    value = egpio_readw(ei, reg);
    pr_debug("readw(%p + %x) = %x\n",
    ei.base_addr, reg << ei.bus_shift, value);
    return !!(value & bit);
    }
    }
#[no_mangle]
unsafe extern "C" fn egpio_direction_input(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int egpio_direction_input(struct gpio_chip *chip, unsigned offset)
    {
    struct egpio_chip *egpio;
    egpio = gpiochip_get_data(chip);
    return test_bit(offset, &egpio.is_out) ? -EINVAL : 0;
    }
//
// Output pins
//
#[no_mangle]
unsafe extern "C" fn egpio_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int egpio_set(struct gpio_chip *chip, unsigned int offset, int value)
    {
    unsigned long     flag;
    struct egpio_chip *egpio;
    struct egpio_info *ei;
    int               pos;
    int               reg;
    int               shift;
    pr_debug("egpio_set(%s, %d(%d), %d)\n",
    chip.label, offset, offset+chip.base, value);
    egpio = gpiochip_get_data(chip);
    ei    = dev_get_drvdata(egpio.dev);
    pos   = egpio_pos(ei, offset);
    reg   = egpio.reg_start + pos;
    shift = pos << ei.reg_shift;
    pr_debug("egpio %s: reg %d = 0x%04x\n", value ? "set" : "clear",
    reg, (egpio.cached_values >> shift) & ei.reg_mask);
    spin_lock_irqsave(&ei.lock, flag);
    if (value)
    egpio.cached_values |= (1 << offset);
    else
    egpio.cached_values &= ~(1 << offset);
    egpio_writew((egpio.cached_values >> shift) & ei.reg_mask, ei, reg);
    spin_unlock_irqrestore(&ei.lock, flag);
    return 0;
    }
    static int egpio_direction_output(struct gpio_chip *chip,
    unsigned offset, int value)
    {
    struct egpio_chip *egpio;
    egpio = gpiochip_get_data(chip);
    if (test_bit(offset, &egpio.is_out))
    return egpio_set(chip, offset, value);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn egpio_get_direction(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int egpio_get_direction(struct gpio_chip *chip, unsigned offset)
    {
    struct egpio_chip *egpio;
    egpio = gpiochip_get_data(chip);
    if (test_bit(offset, &egpio.is_out))
    return GPIO_LINE_DIRECTION_OUT;
    return GPIO_LINE_DIRECTION_IN;
    }
#[no_mangle]
unsafe extern "C" fn egpio_write_cache(ei: *mut egpio_info) {
    static void egpio_write_cache(struct egpio_info *ei)
    {
    int               i;
    struct egpio_chip *egpio;
    int               shift;
    for (i = 0; i < ei.nchips; i++) {
    egpio = &(ei.chip[i]);
    if (!egpio.is_out)
    continue;
    for (shift = 0; shift < egpio.chip.ngpio;
    shift += (1<<ei.reg_shift)) {
    let mut reg: c_int = egpio.reg_start + egpio_pos(ei, shift);
    if (!((egpio.is_out >> shift) & ei.reg_mask))
    continue;
    pr_debug("EGPIO: setting %x to %x, was %x\n", reg,
    (egpio.cached_values >> shift) & ei.reg_mask,
    egpio_readw(ei, reg));
    egpio_writew((egpio.cached_values >> shift)
    & ei.reg_mask, ei, reg);
    }
    }
    }
//
// Setup
//
#[no_mangle]
unsafe extern "C" fn egpio_probe(pdev: *mut platform_device) -> int __init {
    static int __init egpio_probe(struct platform_device *pdev)
    {
    struct htc_egpio_platform_data *pdata = dev_get_platdata(&pdev.dev);
    struct resource   *res;
    struct egpio_info *ei;
    struct gpio_chip  *chip;
    unsigned int      irq, irq_end;
    int               i;
    int               ret;
// Initialize ei data structure.
    ei = devm_kzalloc(&pdev.dev, struct_size(ei, chip, pdata.num_chips), GFP_KERNEL);
    if (!ei)
    return -ENOMEM;
    ei.nchips = pdata.num_chips;
    spin_lock_init(&ei.lock);
// Find chained irq
    res = platform_get_resource(pdev, IORESOURCE_IRQ, 0);
    if (res)
    ei.chained_irq = res.start;
// Map egpio chip into virtual address space.
    ei.base_addr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ei.base_addr))
    return PTR_ERR(ei.base_addr);
    if ((pdata.bus_width != 16) && (pdata.bus_width != 32))
    return -EINVAL;
    ei.bus_shift = fls(pdata.bus_width - 1) - 3;
    pr_debug("bus_shift = %d\n", ei.bus_shift);
    if ((pdata.reg_width != 8) && (pdata.reg_width != 16))
    return -EINVAL;
    ei.reg_shift = fls(pdata.reg_width - 1);
    pr_debug("reg_shift = %d\n", ei.reg_shift);
    ei.reg_mask = (1 << pdata.reg_width) - 1;
    platform_set_drvdata(pdev, ei);
    for (i = 0; i < ei.nchips; i++) {
    ei.chip[i].reg_start = pdata.chip[i].reg_start;
    ei.chip[i].cached_values = pdata.chip[i].initial_values;
    ei.chip[i].is_out = pdata.chip[i].direction;
    ei.chip[i].dev = &(pdev.dev);
    chip = &(ei.chip[i].chip);
    chip.label = devm_kasprintf(&pdev.dev, GFP_KERNEL,
    "htc-egpio-%d",
    i);
    if (!chip.label)
    return -ENOMEM;
    chip.parent          = &pdev.dev;
    chip.owner           = THIS_MODULE;
    chip.get             = egpio_get;
    chip.set             = egpio_set;
    chip.direction_input = egpio_direction_input;
    chip.direction_output = egpio_direction_output;
    chip.get_direction   = egpio_get_direction;
    chip.base            = pdata.chip[i].gpio_base;
    chip.ngpio           = pdata.chip[i].num_gpios;
    ret = devm_gpiochip_add_data(&pdev.dev, chip, &ei.chip[i]);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "failed to register gpiochip %d\n", i);
    }
// Set initial pin values
    egpio_write_cache(ei);
    ei.irq_start = pdata.irq_base;
    ei.nirqs = pdata.num_irqs;
    ei.ack_register = pdata.ack_register;
    if (ei.chained_irq) {
// Setup irq handlers
    ei.ack_write = 0xFFFF;
    if (pdata.invert_acks)
    ei.ack_write = 0;
    irq_end = ei.irq_start + ei.nirqs;
    for (irq = ei.irq_start; irq < irq_end; irq++) {
    irq_set_chip_and_handler(irq, &egpio_muxed_chip,
    handle_simple_irq);
    irq_set_chip_data(irq, ei);
    irq_clear_status_flags(irq, IRQ_NOREQUEST | IRQ_NOPROBE);
    }
    irq_set_irq_type(ei.chained_irq, IRQ_TYPE_EDGE_RISING);
    irq_set_chained_handler_and_data(ei.chained_irq,
    egpio_handler, ei);
    ack_irqs(ei);
    device_init_wakeup(&pdev.dev, 1);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn egpio_suspend(dev: *mut device) -> c_int {
    static int egpio_suspend(struct device *dev)
    {
    struct egpio_info *ei = dev_get_drvdata(dev);
    if (ei.chained_irq && device_may_wakeup(dev))
    enable_irq_wake(ei.chained_irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn egpio_resume(dev: *mut device) -> c_int {
    static int egpio_resume(struct device *dev)
    {
    struct egpio_info *ei = dev_get_drvdata(dev);
    if (ei.chained_irq && device_may_wakeup(dev))
    disable_irq_wake(ei.chained_irq);
// Update registers from the cache, in case
    the CPLD was powered off during suspend */
    egpio_write_cache(ei);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(egpio_pm_ops, egpio_suspend, egpio_resume);
    static struct platform_driver egpio_driver = {
    .driver = {
    .name = "htc-egpio",
    .suppress_bind_attrs = true,
    .pm = pm_sleep_ptr(&egpio_pm_ops),
    },
    };
#[no_mangle]
unsafe extern "C" fn egpio_init() -> int __init {
    static int __init egpio_init(void)
    {
    return platform_driver_probe(&egpio_driver, egpio_probe);
    }
// start early for dependencies
    subsys_initcall(egpio_init);
