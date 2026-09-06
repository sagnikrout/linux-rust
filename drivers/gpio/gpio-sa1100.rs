//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-sa1100.c
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
// linux/arch/arm/mach-sa1100/gpio.c
//
// Generic SA-1100 GPIO handling
//

    const struct software_node sa1100_gpiochip_node = {
    .name = "sa1100-gpio",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa1100_gpio_chip {
    pub chip: gpio_chip,
    pub membase: *mut void __iomem,
    pub irqbase: c_int,
    pub irqmask: u32,
    pub irqrising: u32,
    pub irqfalling: u32,
    pub irqwake: u32,
}

    enum {
    R_GPLR = 0x00,
    R_GPDR = 0x04,
    R_GPSR = 0x08,
    R_GPCR = 0x0c,
    R_GRER = 0x10,
    R_GFER = 0x14,
    R_GEDR = 0x18,
    R_GAFR = 0x1c,
    };
#[no_mangle]
unsafe extern "C" fn sa1100_gpio_get(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int sa1100_gpio_get(struct gpio_chip *chip, unsigned offset)
    {
    return readl_relaxed(sa1100_gpio_chip(chip).membase + R_GPLR) &
    BIT(offset);
    }
    static int sa1100_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    let mut reg: c_int = value ? R_GPSR : R_GPCR;
    writel_relaxed(BIT(offset), sa1100_gpio_chip(chip).membase + reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sa1100_get_direction(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int sa1100_get_direction(struct gpio_chip *chip, unsigned offset)
    {
    void __iomem *gpdr = sa1100_gpio_chip(chip).membase + R_GPDR;
    if (readl_relaxed(gpdr) & BIT(offset))
    return GPIO_LINE_DIRECTION_OUT;
    return GPIO_LINE_DIRECTION_IN;
    }
#[no_mangle]
unsafe extern "C" fn sa1100_direction_input(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int sa1100_direction_input(struct gpio_chip *chip, unsigned offset)
    {
    void __iomem *gpdr = sa1100_gpio_chip(chip).membase + R_GPDR;
    unsigned long flags;
    local_irq_save(flags);
    writel_relaxed(readl_relaxed(gpdr) & ~BIT(offset), gpdr);
    local_irq_restore(flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sa1100_direction_output(chip: *mut gpio_chip, offset: unsigned, value: c_int) -> c_int {
    static int sa1100_direction_output(struct gpio_chip *chip, unsigned offset, int value)
    {
    void __iomem *gpdr = sa1100_gpio_chip(chip).membase + R_GPDR;
    unsigned long flags;
    local_irq_save(flags);
    sa1100_gpio_set(chip, offset, value);
    writel_relaxed(readl_relaxed(gpdr) | BIT(offset), gpdr);
    local_irq_restore(flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sa1100_to_irq(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int sa1100_to_irq(struct gpio_chip *chip, unsigned offset)
    {
    return sa1100_gpio_chip(chip).irqbase + offset;
    }
    static struct sa1100_gpio_chip sa1100_gpio_chip = {
    .chip = {
    .label			= "gpio",
    .get_direction		= sa1100_get_direction,
    .direction_input	= sa1100_direction_input,
    .direction_output	= sa1100_direction_output,
    .set			= sa1100_gpio_set,
    .get			= sa1100_gpio_get,
    .to_irq			= sa1100_to_irq,
    .base			= 0,
    .ngpio			= GPIO_MAX + 1,
    },
    .membase = (void *)&GPLR,
    .irqbase = IRQ_GPIO0,
    };
//
// SA1100 GPIO edge detection for IRQs:
// IRQs are generated on Falling-Edge, Rising-Edge, or both.
// Use this instead of directly setting GRER/GFER.
//
#[no_mangle]
unsafe extern "C" fn sa1100_update_edge_regs(sgc: *mut sa1100_gpio_chip) {
    static void sa1100_update_edge_regs(struct sa1100_gpio_chip *sgc)
    {
    void *base = sgc.membase;
    u32 grer, gfer;
    grer = sgc.irqrising & sgc.irqmask;
    gfer = sgc.irqfalling & sgc.irqmask;
    writel_relaxed(grer, base + R_GRER);
    writel_relaxed(gfer, base + R_GFER);
    }
#[no_mangle]
unsafe extern "C" fn sa1100_gpio_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int sa1100_gpio_type(struct irq_data *d, unsigned int type)
    {
    struct sa1100_gpio_chip *sgc = irq_data_get_irq_chip_data(d);
    let mut mask: c_uint = BIT(d.hwirq);
    if (type == IRQ_TYPE_PROBE) {
    if ((sgc.irqrising | sgc.irqfalling) & mask)
    return 0;
    type = IRQ_TYPE_EDGE_BOTH;
    }
    if (type & IRQ_TYPE_EDGE_RISING)
    sgc.irqrising |= mask;
    else
    sgc.irqrising &= ~mask;
    if (type & IRQ_TYPE_EDGE_FALLING)
    sgc.irqfalling |= mask;
    else
    sgc.irqfalling &= ~mask;
    sa1100_update_edge_regs(sgc);
    return 0;
    }
//
// GPIO IRQs must be acknowledged.
//
#[no_mangle]
unsafe extern "C" fn sa1100_gpio_ack(d: *mut irq_data) {
    static void sa1100_gpio_ack(struct irq_data *d)
    {
    struct sa1100_gpio_chip *sgc = irq_data_get_irq_chip_data(d);
    writel_relaxed(BIT(d.hwirq), sgc.membase + R_GEDR);
    }
#[no_mangle]
unsafe extern "C" fn sa1100_gpio_mask(d: *mut irq_data) {
    static void sa1100_gpio_mask(struct irq_data *d)
    {
    struct sa1100_gpio_chip *sgc = irq_data_get_irq_chip_data(d);
    let mut mask: c_uint = BIT(d.hwirq);
    sgc.irqmask &= ~mask;
    sa1100_update_edge_regs(sgc);
    }
#[no_mangle]
unsafe extern "C" fn sa1100_gpio_unmask(d: *mut irq_data) {
    static void sa1100_gpio_unmask(struct irq_data *d)
    {
    struct sa1100_gpio_chip *sgc = irq_data_get_irq_chip_data(d);
    let mut mask: c_uint = BIT(d.hwirq);
    sgc.irqmask |= mask;
    sa1100_update_edge_regs(sgc);
    }
#[no_mangle]
unsafe extern "C" fn sa1100_gpio_wake(d: *mut irq_data, on: c_uint) -> c_int {
    static int sa1100_gpio_wake(struct irq_data *d, unsigned int on)
    {
    struct sa1100_gpio_chip *sgc = irq_data_get_irq_chip_data(d);
    let mut ret: c_int = sa11x0_gpio_set_wake(d.hwirq, on);
    if (!ret) {
    if (on)
    sgc.irqwake |= BIT(d.hwirq);
    else
    sgc.irqwake &= ~BIT(d.hwirq);
    }
    return ret;
    }
//
// This is for GPIO IRQs
//
    static struct irq_chip sa1100_gpio_irq_chip = {
    .name		= "GPIO",
    .irq_ack	= sa1100_gpio_ack,
    .irq_mask	= sa1100_gpio_mask,
    .irq_unmask	= sa1100_gpio_unmask,
    .irq_set_type	= sa1100_gpio_type,
    .irq_set_wake	= sa1100_gpio_wake,
    };
    static int sa1100_gpio_irqdomain_map(struct irq_domain *d,
    unsigned int irq, irq_hw_number_t hwirq)
    {
    struct sa1100_gpio_chip *sgc = d.host_data;
    irq_set_chip_data(irq, sgc);
    irq_set_chip_and_handler(irq, &sa1100_gpio_irq_chip, handle_edge_irq);
    irq_set_probe(irq);
    return 0;
    }
    static const struct irq_domain_ops sa1100_gpio_irqdomain_ops = {
    .map = sa1100_gpio_irqdomain_map,
    .xlate = irq_domain_xlate_onetwocell,
    };
    static struct irq_domain *sa1100_gpio_irqdomain;
//
// IRQ 0-11 (GPIO) handler.  We enter here with the
// irq_controller_lock held, and IRQs disabled.  Decode the IRQ
// and call the handler.
//
#[no_mangle]
unsafe extern "C" fn sa1100_gpio_handler(desc: *mut irq_desc) {
    static void sa1100_gpio_handler(struct irq_desc *desc)
    {
    struct sa1100_gpio_chip *sgc = irq_desc_get_handler_data(desc);
    unsigned int irq, mask;
    void __iomem *gedr = sgc.membase + R_GEDR;
    mask = readl_relaxed(gedr);
    do {
//
// clear down all currently active IRQ sources.
// We will be processing them all.
//
    writel_relaxed(mask, gedr);
    irq = sgc.irqbase;
    do {
    if (mask & 1)
    generic_handle_irq(irq);
    mask >>= 1;
    irq++;
    } while (mask);
    mask = readl_relaxed(gedr);
    } while (mask);
    }
#[no_mangle]
unsafe extern "C" fn sa1100_gpio_suspend(data: *mut c_void) -> c_int {
    static int sa1100_gpio_suspend(void *data)
    {
    struct sa1100_gpio_chip *sgc = &sa1100_gpio_chip;
//
// Set the appropriate edges for wakeup.
//
    writel_relaxed(sgc.irqwake & sgc.irqrising, sgc.membase + R_GRER);
    writel_relaxed(sgc.irqwake & sgc.irqfalling, sgc.membase + R_GFER);
//
// Clear any pending GPIO interrupts.
//
    writel_relaxed(readl_relaxed(sgc.membase + R_GEDR),
    sgc.membase + R_GEDR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sa1100_gpio_resume(data: *mut c_void) {
    static void sa1100_gpio_resume(void *data)
    {
    sa1100_update_edge_regs(&sa1100_gpio_chip);
    }
    static const struct syscore_ops sa1100_gpio_syscore_ops = {
    .suspend	= sa1100_gpio_suspend,
    .resume		= sa1100_gpio_resume,
    };
    static struct syscore sa1100_gpio_syscore = {
    .ops = &sa1100_gpio_syscore_ops,
    };
#[no_mangle]
unsafe extern "C" fn sa1100_gpio_init_devicefs() -> int __init {
    static int __init sa1100_gpio_init_devicefs(void)
    {
    register_syscore(&sa1100_gpio_syscore);
    return 0;
    }
    device_initcall(sa1100_gpio_init_devicefs);
    static const int sa1100_gpio_irqs[] __initconst = {
// Install handlers for GPIO 0-10 edge detect interrupts
    IRQ_GPIO0_SC,
    IRQ_GPIO1_SC,
    IRQ_GPIO2_SC,
    IRQ_GPIO3_SC,
    IRQ_GPIO4_SC,
    IRQ_GPIO5_SC,
    IRQ_GPIO6_SC,
    IRQ_GPIO7_SC,
    IRQ_GPIO8_SC,
    IRQ_GPIO9_SC,
    IRQ_GPIO10_SC,
// Install handler for GPIO 11-27 edge detect interrupts
    IRQ_GPIO11_27,
    };
#[no_mangle]
pub unsafe extern "C" fn sa1100_init_gpio() -> void __init {
    void __init sa1100_init_gpio(void)
    {
    struct sa1100_gpio_chip *sgc = &sa1100_gpio_chip;
    struct gpio_chip *gc = &sgc.chip;
    int i;
// clear all GPIO edge detects
    writel_relaxed(0, sgc.membase + R_GFER);
    writel_relaxed(0, sgc.membase + R_GRER);
    writel_relaxed(-1, sgc.membase + R_GEDR);
    software_node_register(&sa1100_gpiochip_node);
    gc.fwnode = software_node_fwnode(&sa1100_gpiochip_node);
    gpiochip_add_data(gc, core::ptr::null_mut());
    sa1100_gpio_irqdomain = irq_domain_create_simple(core::ptr::null_mut(),
    28, IRQ_GPIO0,
    &sa1100_gpio_irqdomain_ops, sgc);
    for (i = 0; i < ARRAY_SIZE(sa1100_gpio_irqs); i++)
    irq_set_chained_handler_and_data(sa1100_gpio_irqs[i],
    sa1100_gpio_handler, sgc);
    }
