//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-dwapb.c
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
// Copyright (c) 2011 Jamie Iles
//
// All enquiries to support@picochip.com
//

pub const GPIO_SWPORTA_DR: c_uint = 0x00;
pub const GPIO_SWPORTA_DDR: c_uint = 0x04;
pub const GPIO_SWPORTB_DR: c_uint = 0x0c;
pub const GPIO_SWPORTB_DDR: c_uint = 0x10;
pub const GPIO_SWPORTC_DR: c_uint = 0x18;
pub const GPIO_SWPORTC_DDR: c_uint = 0x1c;
pub const GPIO_SWPORTD_DR: c_uint = 0x24;
pub const GPIO_SWPORTD_DDR: c_uint = 0x28;
pub const GPIO_INTEN: c_uint = 0x30;
pub const GPIO_INTMASK: c_uint = 0x34;
pub const GPIO_INTTYPE_LEVEL: c_uint = 0x38;
pub const GPIO_INT_POLARITY: c_uint = 0x3c;
pub const GPIO_INTSTATUS: c_uint = 0x40;
pub const GPIO_PORTA_DEBOUNCE: c_uint = 0x48;
pub const GPIO_PORTA_EOI: c_uint = 0x4c;
pub const GPIO_EXT_PORTA: c_uint = 0x50;
pub const GPIO_EXT_PORTB: c_uint = 0x54;
pub const GPIO_EXT_PORTC: c_uint = 0x58;
pub const GPIO_EXT_PORTD: c_uint = 0x5c;

pub const DWAPB_MAX_PORTS: c_int = 4;
pub const DWAPB_MAX_GPIOS: c_int = 32;
pub const GPIO_EXT_PORT_STRIDE: c_uint = 0x04 /* register stride 32 bits */;
pub const GPIO_SWPORT_DR_STRIDE: c_uint = 0x0c /* register stride 3*32 bits */;
pub const GPIO_SWPORT_DDR_STRIDE: c_uint = 0x0c /* register stride 3*32 bits */;
pub const GPIO_REG_OFFSET_V1: c_int = 0;
pub const GPIO_REG_OFFSET_V2: c_int = 1;

pub const GPIO_INTMASK_V2: c_uint = 0x44;
pub const GPIO_INTTYPE_LEVEL_V2: c_uint = 0x34;
pub const GPIO_INT_POLARITY_V2: c_uint = 0x38;
pub const GPIO_INTSTATUS_V2: c_uint = 0x3c;
pub const GPIO_PORTA_EOI_V2: c_uint = 0x40;
pub const DWAPB_NR_CLOCKS: c_int = 2;
    struct dwapb_gpio;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwapb_port_property {
    pub fwnode: *mut fwnode_handle,
    pub idx: c_uint,
    pub ngpio: c_uint,
    pub gpio_base: c_uint,
    pub irq: [c_int; DWAPB_MAX_GPIOS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwapb_platform_data {
    pub nports: c_uint,
    pub __counted_by(nports): dwapb_port_property properties[],
}

// Store GPIO context across system-wide suspend/resume transitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwapb_context {
    pub data: u32,
    pub dir: u32,
    pub ext: u32,
    pub int_en: u32,
    pub int_mask: u32,
    pub int_type: u32,
    pub int_pol: u32,
    pub int_deb: u32,
    pub wake_en: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwapb_gpio_port_irqchip {
    pub nr_irqs: c_uint,
    pub irq: [c_uint; DWAPB_MAX_GPIOS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwapb_gpio_port {
    pub chip: gpio_generic_chip,
    pub pirq: *mut dwapb_gpio_port_irqchip,
    pub gpio: *mut dwapb_gpio,
    pub ctx: *mut dwapb_context,
    pub idx: c_uint,
}

    static inline struct dwapb_gpio *to_dwapb_gpio(struct gpio_chip *gc)
    {
    return container_of(to_gpio_generic_chip(gc),
    struct dwapb_gpio_port, chip).gpio;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwapb_gpio {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub nr_ports: c_uint,
    pub flags: c_uint,
    pub rst: *mut reset_control,
    pub clks: [clk_bulk_data; DWAPB_NR_CLOCKS],
    pub clocks_on_for_wake: bool,
    pub __counted_by(nr_ports): dwapb_gpio_port ports[],
}

#[no_mangle]
pub unsafe extern "C" fn gpio_reg_v2_convert(offset: c_uint) -> u32 {
    static inline u32 gpio_reg_v2_convert(unsigned int offset)
    {
    switch (offset) {
    case GPIO_INTMASK:
    return GPIO_INTMASK_V2;
    case GPIO_INTTYPE_LEVEL:
    return GPIO_INTTYPE_LEVEL_V2;
    case GPIO_INT_POLARITY:
    return GPIO_INT_POLARITY_V2;
    case GPIO_INTSTATUS:
    return GPIO_INTSTATUS_V2;
    case GPIO_PORTA_EOI:
    return GPIO_PORTA_EOI_V2;
    }
    return offset;
    }
#[no_mangle]
pub unsafe extern "C" fn gpio_reg_convert(gpio: *mut dwapb_gpio, offset: c_uint) -> u32 {
    static inline u32 gpio_reg_convert(struct dwapb_gpio *gpio, unsigned int offset)
    {
    if ((gpio.flags & GPIO_REG_OFFSET_MASK) == GPIO_REG_OFFSET_V2)
    return gpio_reg_v2_convert(offset);
    return offset;
    }
#[no_mangle]
pub unsafe extern "C" fn dwapb_read(gpio: *mut dwapb_gpio, offset: c_uint) -> u32 {
    static inline u32 dwapb_read(struct dwapb_gpio *gpio, unsigned int offset)
    {
    struct gpio_generic_chip *chip = &gpio.ports[0].chip;
    void __iomem *reg_base = gpio.regs;
    return gpio_generic_read_reg(chip, reg_base + gpio_reg_convert(gpio, offset));
    }
    static inline void dwapb_write(struct dwapb_gpio *gpio, unsigned int offset,
    u32 val)
    {
    struct gpio_generic_chip *chip = &gpio.ports[0].chip;
    void __iomem *reg_base = gpio.regs;
    gpio_generic_write_reg(chip, reg_base + gpio_reg_convert(gpio, offset), val);
    }
    static struct dwapb_gpio_port *dwapb_offs_to_port(struct dwapb_gpio *gpio, unsigned int offs)
    {
    struct dwapb_gpio_port *port;
    int i;
    for (i = 0; i < gpio.nr_ports; i++) {
    port = &gpio.ports[i];
    if (port.idx == offs / DWAPB_MAX_GPIOS)
    return port;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn dwapb_toggle_trigger(gpio: *mut dwapb_gpio, offs: c_uint) {
    static void dwapb_toggle_trigger(struct dwapb_gpio *gpio, unsigned int offs)
    {
    struct dwapb_gpio_port *port = dwapb_offs_to_port(gpio, offs);
    struct gpio_chip *gc;
    u32 pol;
    int val;
    if (!port)
    return;
    gc = &port.chip.gc;
    pol = dwapb_read(gpio, GPIO_INT_POLARITY);
// Just read the current value right out of the data register
    val = gc.get(gc, offs % DWAPB_MAX_GPIOS);
    if (val)
    pol &= ~BIT(offs);
    else
    pol |= BIT(offs);
    dwapb_write(gpio, GPIO_INT_POLARITY, pol);
    }
#[no_mangle]
unsafe extern "C" fn dwapb_irq_init_hw(gc: *mut gpio_chip) -> c_int {
    static int dwapb_irq_init_hw(struct gpio_chip *gc)
    {
    struct dwapb_gpio *gpio = to_dwapb_gpio(gc);
//
// GPIO interrupts may retain stale state across warm reboots when
// peripherals stay powered. Force a known-safe state before the GPIO
// irqchip and irq domain are set up.
//
    dwapb_write(gpio, GPIO_INTEN, 0);
    dwapb_write(gpio, GPIO_INTMASK, 0xffffffff);
    dwapb_write(gpio, GPIO_PORTA_EOI, 0xffffffff);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwapb_do_irq(gpio: *mut dwapb_gpio) -> u32 {
    static u32 dwapb_do_irq(struct dwapb_gpio *gpio)
    {
    struct gpio_generic_chip *gen_gc = &gpio.ports[0].chip;
    unsigned long irq_status;
    irq_hw_number_t hwirq;
    irq_status = dwapb_read(gpio, GPIO_INTSTATUS);
    for_each_set_bit(hwirq, &irq_status, DWAPB_MAX_GPIOS) {
    let mut gpio_irq: c_int = irq_find_mapping(gen_gc.gc.irq.domain, hwirq);
    let mut irq_type: u32 = irq_get_trigger_type(gpio_irq);
    generic_handle_irq(gpio_irq);
    if ((irq_type & IRQ_TYPE_SENSE_MASK) == IRQ_TYPE_EDGE_BOTH)
    dwapb_toggle_trigger(gpio, hwirq);
    }
    return irq_status;
    }
#[no_mangle]
unsafe extern "C" fn dwapb_irq_handler(desc: *mut irq_desc) {
    static void dwapb_irq_handler(struct irq_desc *desc)
    {
    struct dwapb_gpio *gpio = irq_desc_get_handler_data(desc);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    chained_irq_enter(chip, desc);
    dwapb_do_irq(gpio);
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn dwapb_irq_handler_mfd(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t dwapb_irq_handler_mfd(int irq, void *dev_id)
    {
    return IRQ_RETVAL(dwapb_do_irq(dev_id));
    }
#[no_mangle]
unsafe extern "C" fn dwapb_irq_ack(d: *mut irq_data) {
    static void dwapb_irq_ack(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct gpio_generic_chip *gen_gc = to_gpio_generic_chip(gc);
    struct dwapb_gpio *gpio = to_dwapb_gpio(gc);
    let mut val: u32 = BIT(irqd_to_hwirq(d));
    guard(gpio_generic_lock_irqsave)(gen_gc);
    dwapb_write(gpio, GPIO_PORTA_EOI, val);
    }
#[no_mangle]
unsafe extern "C" fn dwapb_irq_mask(d: *mut irq_data) {
    static void dwapb_irq_mask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct gpio_generic_chip *gen_gc = to_gpio_generic_chip(gc);
    struct dwapb_gpio *gpio = to_dwapb_gpio(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    u32 val;
    scoped_guard(gpio_generic_lock_irqsave, gen_gc) {
    val = dwapb_read(gpio, GPIO_INTMASK) | BIT(hwirq);
    dwapb_write(gpio, GPIO_INTMASK, val);
    }
    gpiochip_disable_irq(gc, hwirq);
    }
#[no_mangle]
unsafe extern "C" fn dwapb_irq_unmask(d: *mut irq_data) {
    static void dwapb_irq_unmask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct gpio_generic_chip *gen_gc = to_gpio_generic_chip(gc);
    struct dwapb_gpio *gpio = to_dwapb_gpio(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    u32 val;
    gpiochip_enable_irq(gc, hwirq);
    guard(gpio_generic_lock_irqsave)(gen_gc);
    val = dwapb_read(gpio, GPIO_INTMASK) & ~BIT(hwirq);
    dwapb_write(gpio, GPIO_INTMASK, val);
    }
#[no_mangle]
unsafe extern "C" fn dwapb_irq_enable(d: *mut irq_data) {
    static void dwapb_irq_enable(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct gpio_generic_chip *gen_gc = to_gpio_generic_chip(gc);
    struct dwapb_gpio *gpio = to_dwapb_gpio(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    u32 val;
    guard(gpio_generic_lock_irqsave)(gen_gc);
    val = dwapb_read(gpio, GPIO_INTEN) | BIT(hwirq);
    dwapb_write(gpio, GPIO_INTEN, val);
    val = dwapb_read(gpio, GPIO_INTMASK) & ~BIT(hwirq);
    dwapb_write(gpio, GPIO_INTMASK, val);
    }
#[no_mangle]
unsafe extern "C" fn dwapb_irq_disable(d: *mut irq_data) {
    static void dwapb_irq_disable(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct gpio_generic_chip *gen_gc = to_gpio_generic_chip(gc);
    struct dwapb_gpio *gpio = to_dwapb_gpio(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    u32 val;
    guard(gpio_generic_lock_irqsave)(gen_gc);
    val = dwapb_read(gpio, GPIO_INTMASK) | BIT(hwirq);
    dwapb_write(gpio, GPIO_INTMASK, val);
    val = dwapb_read(gpio, GPIO_INTEN) & ~BIT(hwirq);
    dwapb_write(gpio, GPIO_INTEN, val);
    }
#[no_mangle]
unsafe extern "C" fn dwapb_irq_set_type(d: *mut irq_data, type: u32) -> c_int {
    static int dwapb_irq_set_type(struct irq_data *d, u32 type)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct gpio_generic_chip *gen_gc = to_gpio_generic_chip(gc);
    struct dwapb_gpio *gpio = to_dwapb_gpio(gc);
    let mut bit: irq_hw_number_t = irqd_to_hwirq(d);
    unsigned long level, polarity;
    guard(gpio_generic_lock_irqsave)(gen_gc);
    level = dwapb_read(gpio, GPIO_INTTYPE_LEVEL);
    polarity = dwapb_read(gpio, GPIO_INT_POLARITY);
    switch (type) {
    case IRQ_TYPE_EDGE_BOTH:
    level |= BIT(bit);
    dwapb_toggle_trigger(gpio, bit);
    break;
    case IRQ_TYPE_EDGE_RISING:
    level |= BIT(bit);
    polarity |= BIT(bit);
    break;
    case IRQ_TYPE_EDGE_FALLING:
    level |= BIT(bit);
    polarity &= ~BIT(bit);
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    level &= ~BIT(bit);
    polarity |= BIT(bit);
    break;
    case IRQ_TYPE_LEVEL_LOW:
    level &= ~BIT(bit);
    polarity &= ~BIT(bit);
    break;
    }
    if (type & IRQ_TYPE_LEVEL_MASK)
    irq_set_handler_locked(d, handle_level_irq);
#[no_mangle]
pub unsafe extern "C" fn if(IRQ_TYPE_EDGE_BOTH: type &) -> else {
    else if (type & IRQ_TYPE_EDGE_BOTH)
    irq_set_handler_locked(d, handle_edge_irq);
    dwapb_write(gpio, GPIO_INTTYPE_LEVEL, level);
    if (type != IRQ_TYPE_EDGE_BOTH)
    dwapb_write(gpio, GPIO_INT_POLARITY, polarity);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwapb_irq_set_wake(d: *mut irq_data, enable: c_uint) -> c_int {
    static int dwapb_irq_set_wake(struct irq_data *d, unsigned int enable)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct dwapb_gpio *gpio = to_dwapb_gpio(gc);
    struct dwapb_context *ctx = gpio.ports[0].ctx;
    let mut bit: irq_hw_number_t = irqd_to_hwirq(d);
    let mut wake_en: u32 = ctx.wake_en;
    if (enable)
    wake_en |= BIT(bit);
    else
    wake_en &= ~BIT(bit);

    if (d.parent_data && !!ctx.wake_en != !!wake_en) {
    int err;
    err = irq_chip_set_wake_parent(d, enable);
    if (err)
    return err;
    }

    ctx.wake_en = wake_en;
    return 0;
    }
    static const struct irq_chip dwapb_irq_chip = {
    .name		= DWAPB_DRIVER_NAME,
    .irq_ack	= dwapb_irq_ack,
    .irq_mask	= dwapb_irq_mask,
    .irq_unmask	= dwapb_irq_unmask,
    .irq_set_type	= dwapb_irq_set_type,
    .irq_enable	= dwapb_irq_enable,
    .irq_disable	= dwapb_irq_disable,
    .irq_set_wake	= pm_sleep_ptr(dwapb_irq_set_wake),
    .flags		= IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
    static int dwapb_gpio_set_debounce(struct gpio_chip *gc,
    unsigned offset, unsigned debounce)
    {
    struct dwapb_gpio_port *port = gpiochip_get_data(gc);
    struct gpio_generic_chip *gen_gc = to_gpio_generic_chip(gc);
    struct dwapb_gpio *gpio = port.gpio;
    unsigned long val_deb;
    let mut mask: c_ulong = BIT(offset);
    guard(gpio_generic_lock_irqsave)(gen_gc);
    val_deb = dwapb_read(gpio, GPIO_PORTA_DEBOUNCE);
    if (debounce)
    val_deb |= mask;
    else
    val_deb &= ~mask;
    dwapb_write(gpio, GPIO_PORTA_DEBOUNCE, val_deb);
    return 0;
    }
    static int dwapb_gpio_set_config(struct gpio_chip *gc, unsigned offset,
    unsigned long config)
    {
    u32 debounce;
    if (pinconf_to_config_param(config) == PIN_CONFIG_INPUT_DEBOUNCE) {
    debounce = pinconf_to_config_argument(config);
    return dwapb_gpio_set_debounce(gc, offset, debounce);
    }
    return gpiochip_generic_config(gc, offset, config);
    }
    static int dwapb_convert_irqs(struct dwapb_gpio_port_irqchip *pirq,
    struct dwapb_port_property *pp)
    {
    int i;
// Group all available IRQs into an array of parental IRQs.
    for (i = 0; i < pp.ngpio; ++i) {
    if (!pp.irq[i])
    continue;
    pirq.irq[pirq.nr_irqs++] = pp.irq[i];
    }
    return pirq.nr_irqs ? 0 : -ENOENT;
    }
    static void dwapb_configure_irqs(struct dwapb_gpio *gpio,
    struct dwapb_gpio_port *port,
    struct dwapb_port_property *pp)
    {
    struct dwapb_gpio_port_irqchip *pirq;
    struct gpio_chip *gc = &port.chip.gc;
    struct gpio_irq_chip *girq;
    int err;
    pirq = devm_kzalloc(gpio.dev, sizeof(*pirq), GFP_KERNEL);
    if (!pirq)
    return;
    if (dwapb_convert_irqs(pirq, pp)) {
    dev_warn(gpio.dev, "no IRQ for port%d\n", pp.idx);
    goto err_kfree_pirq;
    }
    girq = &gc.irq;
    girq.handler = handle_bad_irq;
    girq.default_type = IRQ_TYPE_NONE;
    girq.init_hw = dwapb_irq_init_hw;
    port.pirq = pirq;
//
// Intel ACPI-based platforms mostly have the DesignWare APB GPIO
// IRQ lane shared between several devices. In that case the parental
// IRQ has to be handled in the shared way so to be properly delivered
// to all the connected devices.
//
    if (has_acpi_companion(gpio.dev)) {
    girq.num_parents = 0;
    girq.parents = core::ptr::null_mut();
    girq.parent_handler = core::ptr::null_mut();
    err = devm_request_irq(gpio.dev, pp.irq[0],
    dwapb_irq_handler_mfd,
    IRQF_SHARED, DWAPB_DRIVER_NAME, gpio);
    if (err) {
    dev_err(gpio.dev, "error requesting IRQ\n");
    goto err_kfree_pirq;
    }
    } else {
    girq.num_parents = pirq.nr_irqs;
    girq.parents = pirq.irq;
    girq.parent_handler_data = gpio;
    girq.parent_handler = dwapb_irq_handler;
    }
    gpio_irq_chip_set_chip(girq, &dwapb_irq_chip);
    return;
    err_kfree_pirq:
    devm_kfree(gpio.dev, pirq);
    }
    static int dwapb_gpio_add_port(struct dwapb_gpio *gpio,
    struct dwapb_port_property *pp,
    unsigned int offs)
    {
    struct gpio_generic_chip_config config;
    struct dwapb_gpio_port *port;
    void __iomem *dat, *set, *dirout;
    int err;
    port = &gpio.ports[offs];
    port.gpio = gpio;
    port.idx = pp.idx;

    port.ctx = devm_kzalloc(gpio.dev, sizeof(*port.ctx), GFP_KERNEL);
    if (!port.ctx)
    return -ENOMEM;

    dat = gpio.regs + GPIO_EXT_PORTA + pp.idx * GPIO_EXT_PORT_STRIDE;
    set = gpio.regs + GPIO_SWPORTA_DR + pp.idx * GPIO_SWPORT_DR_STRIDE;
    dirout = gpio.regs + GPIO_SWPORTA_DDR + pp.idx * GPIO_SWPORT_DDR_STRIDE;
    config = (struct gpio_generic_chip_config) {
    .dev = gpio.dev,
    .sz = 4,
    .dat = dat,
    .set = set,
    .dirout = dirout,
    };
// This registers 32 GPIO lines per port
    err = gpio_generic_chip_init(&port.chip, &config);
    if (err) {
    dev_err(gpio.dev, "failed to init gpio chip for port%d\n",
    port.idx);
    return err;
    }
    port.chip.gc.fwnode = pp.fwnode;
    port.chip.gc.ngpio = pp.ngpio;
    port.chip.gc.base = pp.gpio_base;
    port.chip.gc.request = gpiochip_generic_request;
    port.chip.gc.free = gpiochip_generic_free;
// Only port A support debounce
    if (pp.idx == 0)
    port.chip.gc.set_config = dwapb_gpio_set_config;
    else
    port.chip.gc.set_config = gpiochip_generic_config;
// Only port A can provide interrupts in all configurations of the IP
    if (pp.idx == 0)
    dwapb_configure_irqs(gpio, port, pp);
    err = devm_gpiochip_add_data(gpio.dev, &port.chip.gc, port);
    if (err) {
    dev_err(gpio.dev, "failed to register gpiochip for port%d\n",
    port.idx);
    return err;
    }
    return 0;
    }
    static void dwapb_get_irq(struct device *dev, struct fwnode_handle *fwnode,
    struct dwapb_port_property *pp)
    {
    int irq, j;
    for (j = 0; j < pp.ngpio; j++) {
    if (has_acpi_companion(dev))
    irq = platform_get_irq_optional(to_platform_device(dev), j);
    else
    irq = fwnode_irq_get(fwnode, j);
    if (irq > 0)
    pp.irq[j] = irq;
    }
    }
    static struct dwapb_platform_data *dwapb_gpio_get_pdata(struct device *dev)
    {
    struct dwapb_platform_data *pdata;
    struct dwapb_port_property *pp;
    int nports;
    int i;
    nports = device_get_child_node_count(dev);
    if (nports == 0)
    return ERR_PTR(-ENODEV);
    pdata = devm_kzalloc(dev, struct_size(pdata, properties, nports), GFP_KERNEL);
    if (!pdata)
    return ERR_PTR(-ENOMEM);
    pdata.nports = nports;
    i = 0;
    device_for_each_child_node_scoped(dev, fwnode)  {
    pp = &pdata.properties[i++];
    pp.fwnode = fwnode;
    if (fwnode_property_read_u32(fwnode, "reg", &pp.idx) ||
    pp.idx >= DWAPB_MAX_PORTS) {
    dev_err(dev,
    "missing/invalid port index for port%d\n", i);
    return ERR_PTR(-EINVAL);
    }
    if (fwnode_property_read_u32(fwnode, "ngpios", &pp.ngpio) &&
    fwnode_property_read_u32(fwnode, "snps,nr-gpios", &pp.ngpio)) {
    dev_info(dev,
    "failed to get number of gpios for port%d\n",
    i);
    pp.ngpio = DWAPB_MAX_GPIOS;
    }
    pp.gpio_base	= -1;
// For internal use only, new platforms mustn't exercise this
    if (is_software_node(fwnode))
    fwnode_property_read_u32(fwnode, "gpio-base", &pp.gpio_base);
//
// Only port A can provide interrupts in all configurations of
// the IP.
//
    if (pp.idx == 0)
    dwapb_get_irq(dev, fwnode, pp);
    }
    return pdata;
    }
#[no_mangle]
unsafe extern "C" fn dwapb_assert_reset(data: *mut c_void) {
    static void dwapb_assert_reset(void *data)
    {
    struct dwapb_gpio *gpio = data;
    reset_control_assert(gpio.rst);
    }
#[no_mangle]
unsafe extern "C" fn dwapb_get_reset(gpio: *mut dwapb_gpio) -> c_int {
    static int dwapb_get_reset(struct dwapb_gpio *gpio)
    {
    int err;
    gpio.rst = devm_reset_control_get_optional_shared(gpio.dev, core::ptr::null_mut());
    if (IS_ERR(gpio.rst))
    return dev_err_probe(gpio.dev, PTR_ERR(gpio.rst),
    "Cannot get reset descriptor\n");
    err = reset_control_deassert(gpio.rst);
    if (err) {
    dev_err(gpio.dev, "Cannot deassert reset lane\n");
    return err;
    }
    return devm_add_action_or_reset(gpio.dev, dwapb_assert_reset, gpio);
    }
#[no_mangle]
unsafe extern "C" fn dwapb_disable_clks(data: *mut c_void) {
    static void dwapb_disable_clks(void *data)
    {
    struct dwapb_gpio *gpio = data;
    clk_bulk_disable_unprepare(DWAPB_NR_CLOCKS, gpio.clks);
    }
#[no_mangle]
unsafe extern "C" fn dwapb_get_clks(gpio: *mut dwapb_gpio) -> c_int {
    static int dwapb_get_clks(struct dwapb_gpio *gpio)
    {
    int err;
// Optional bus and debounce clocks
    gpio.clks[0].id = "bus";
    gpio.clks[1].id = "db";
    err = devm_clk_bulk_get_optional(gpio.dev, DWAPB_NR_CLOCKS,
    gpio.clks);
    if (err)
    return dev_err_probe(gpio.dev, err,
    "Cannot get APB/Debounce clocks\n");
    err = clk_bulk_prepare_enable(DWAPB_NR_CLOCKS, gpio.clks);
    if (err) {
    dev_err(gpio.dev, "Cannot enable APB/Debounce clocks\n");
    return err;
    }
    return devm_add_action_or_reset(gpio.dev, dwapb_disable_clks, gpio);
    }
    static const struct of_device_id dwapb_of_match[] = {
    { .compatible = "snps,dw-apb-gpio", .data = (void *)GPIO_REG_OFFSET_V1},
    { .compatible = "apm,xgene-gpio-v2", .data = (void *)GPIO_REG_OFFSET_V2},
    { /* Sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, dwapb_of_match);
    static const struct acpi_device_id dwapb_acpi_match[] = {
    {"HISI0181", GPIO_REG_OFFSET_V1},
    {"APMC0D07", GPIO_REG_OFFSET_V1},
    {"APMC0D81", GPIO_REG_OFFSET_V2},
    {"FUJI200A", GPIO_REG_OFFSET_V1},
    {"LECA0001", GPIO_REG_OFFSET_V1},
    { }
    };
    MODULE_DEVICE_TABLE(acpi, dwapb_acpi_match);
#[no_mangle]
unsafe extern "C" fn dwapb_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int dwapb_gpio_probe(struct platform_device *pdev)
    {
    unsigned int i;
    struct dwapb_gpio *gpio;
    int err;
    struct dwapb_platform_data *pdata;
    struct device *dev = &pdev.dev;
    pdata = dwapb_gpio_get_pdata(dev);
    if (IS_ERR(pdata))
    return PTR_ERR(pdata);
    gpio = devm_kzalloc(&pdev.dev, struct_size(gpio, ports, pdata.nports), GFP_KERNEL);
    if (!gpio)
    return -ENOMEM;
    gpio.nr_ports = pdata.nports;
    gpio.dev = &pdev.dev;
    err = dwapb_get_reset(gpio);
    if (err)
    return err;
    gpio.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(gpio.regs))
    return PTR_ERR(gpio.regs);
    err = dwapb_get_clks(gpio);
    if (err)
    return err;
    gpio.flags = (uintptr_t)device_get_match_data(dev);
    for (i = 0; i < gpio.nr_ports; i++) {
    err = dwapb_gpio_add_port(gpio, &pdata.properties[i], i);
    if (err)
    return err;
    }
    platform_set_drvdata(pdev, gpio);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwapb_gpio_suspend(dev: *mut device) -> c_int {
    static int dwapb_gpio_suspend(struct device *dev)
    {
    struct dwapb_gpio *gpio = dev_get_drvdata(dev);
    struct gpio_generic_chip *gen_gc = &gpio.ports[0].chip;
    int i;
    scoped_guard(gpio_generic_lock_irqsave, gen_gc) {
    gpio.clocks_on_for_wake = false;
    for (i = 0; i < gpio.nr_ports; i++) {
    unsigned int offset;
    let mut idx: c_uint = gpio.ports[i].idx;
    struct dwapb_context *ctx = gpio.ports[i].ctx;
    offset = GPIO_SWPORTA_DDR + idx * GPIO_SWPORT_DDR_STRIDE;
    ctx.dir = dwapb_read(gpio, offset);
    offset = GPIO_SWPORTA_DR + idx * GPIO_SWPORT_DR_STRIDE;
    ctx.data = dwapb_read(gpio, offset);
    offset = GPIO_EXT_PORTA + idx * GPIO_EXT_PORT_STRIDE;
    ctx.ext = dwapb_read(gpio, offset);
// Only port A can provide interrupts
    if (idx == 0) {
    ctx.int_mask = dwapb_read(gpio, GPIO_INTMASK);
    ctx.int_en = dwapb_read(gpio, GPIO_INTEN);
    ctx.int_pol = dwapb_read(gpio, GPIO_INT_POLARITY);
    ctx.int_type = dwapb_read(gpio, GPIO_INTTYPE_LEVEL);
    ctx.int_deb = dwapb_read(gpio, GPIO_PORTA_DEBOUNCE);
    }
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwapb_gpio_suspend_noirq(dev: *mut device) -> c_int {
    static int dwapb_gpio_suspend_noirq(struct device *dev)
    {
    struct dwapb_gpio *gpio = dev_get_drvdata(dev);
    struct gpio_generic_chip *gen_gc = &gpio.ports[0].chip;
    let mut wake_enabled: bool = false;
    int i;
    scoped_guard(gpio_generic_lock_irqsave, gen_gc) {
    for (i = 0; i < gpio.nr_ports; i++) {
    let mut idx: c_uint = gpio.ports[i].idx;
    struct dwapb_context *ctx = gpio.ports[i].ctx;
    if (idx == 0) {
    wake_enabled = ctx.wake_en;
    dwapb_write(gpio, GPIO_INTMASK, ~ctx.wake_en);
    break;
    }
    }
    gpio.clocks_on_for_wake = wake_enabled;
    }
    if (wake_enabled) {
    device_set_wakeup_path(dev);
    return 0;
    }
    clk_bulk_disable_unprepare(DWAPB_NR_CLOCKS, gpio.clks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwapb_gpio_resume_noirq(dev: *mut device) -> c_int {
    static int dwapb_gpio_resume_noirq(struct device *dev)
    {
    struct dwapb_gpio *gpio = dev_get_drvdata(dev);
    int err;
    if (gpio.clocks_on_for_wake)
    return 0;
    err = clk_bulk_prepare_enable(DWAPB_NR_CLOCKS, gpio.clks);
    if (err)
    dev_err(gpio.dev, "Cannot reenable APB/Debounce clocks\n");
    return err;
    }
#[no_mangle]
unsafe extern "C" fn dwapb_gpio_resume(dev: *mut device) -> c_int {
    static int dwapb_gpio_resume(struct device *dev)
    {
    struct dwapb_gpio *gpio = dev_get_drvdata(dev);
    struct gpio_chip *gc = &gpio.ports[0].chip.gc;
    struct gpio_generic_chip *gen_gc = to_gpio_generic_chip(gc);
    int i;
    guard(gpio_generic_lock_irqsave)(gen_gc);
    for (i = 0; i < gpio.nr_ports; i++) {
    unsigned int offset;
    let mut idx: c_uint = gpio.ports[i].idx;
    struct dwapb_context *ctx = gpio.ports[i].ctx;
    offset = GPIO_SWPORTA_DR + idx * GPIO_SWPORT_DR_STRIDE;
    dwapb_write(gpio, offset, ctx.data);
    offset = GPIO_SWPORTA_DDR + idx * GPIO_SWPORT_DDR_STRIDE;
    dwapb_write(gpio, offset, ctx.dir);
    offset = GPIO_EXT_PORTA + idx * GPIO_EXT_PORT_STRIDE;
    dwapb_write(gpio, offset, ctx.ext);
// Only port A can provide interrupts
    if (idx == 0) {
    dwapb_write(gpio, GPIO_INTTYPE_LEVEL, ctx.int_type);
    dwapb_write(gpio, GPIO_INT_POLARITY, ctx.int_pol);
    dwapb_write(gpio, GPIO_PORTA_DEBOUNCE, ctx.int_deb);
    dwapb_write(gpio, GPIO_INTEN, ctx.int_en);
    dwapb_write(gpio, GPIO_INTMASK, ctx.int_mask);
// Clear out spurious interrupts
    dwapb_write(gpio, GPIO_PORTA_EOI, 0xffffffff);
    }
    }
    return 0;
    }
    static const struct dev_pm_ops dwapb_gpio_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(dwapb_gpio_suspend, dwapb_gpio_resume)
    NOIRQ_SYSTEM_SLEEP_PM_OPS(dwapb_gpio_suspend_noirq,
    dwapb_gpio_resume_noirq)
    };
    static struct platform_driver dwapb_gpio_driver = {
    .driver		= {
    .name	= DWAPB_DRIVER_NAME,
    .pm	= pm_sleep_ptr(&dwapb_gpio_pm_ops),
    .of_match_table = dwapb_of_match,
    .acpi_match_table = dwapb_acpi_match,
    },
    .probe		= dwapb_gpio_probe,
    };
    module_platform_driver(dwapb_gpio_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Jamie Iles");
    MODULE_DESCRIPTION("Synopsys DesignWare APB GPIO driver");
    MODULE_ALIAS("platform:" DWAPB_DRIVER_NAME);
