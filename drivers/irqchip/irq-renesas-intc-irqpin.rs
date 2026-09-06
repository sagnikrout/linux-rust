//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-renesas-intc-irqpin.c
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
// Renesas INTC External IRQ Pin Driver
//
// Copyright (C) 2013 Magnus Damm
//

pub const INTC_IRQPIN_REG_NR_MANDATORY: c_int = 5;

pub const INTC_IRQPIN_REG_NR: c_int = 6;
// INTC external IRQ PIN hardware register access:
//
// SENSE is read-write 32-bit with 2-bits or 4-bits per IRQ (*)
// PRIO is read-write 32-bit with 4-bits per IRQ (**)
// SOURCE is read-only 32-bit or 8-bit with 1-bit per IRQ (***)
// MASK is write-only 32-bit or 8-bit with 1-bit per IRQ (***)
// CLEAR is write-only 32-bit or 8-bit with 1-bit per IRQ (***)
//
// (*) May be accessed by more than one driver instance - lock needed
// (**) Read-modify-write access by one driver instance - lock needed
// (***) Accessed by one driver instance only - no locking needed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intc_irqpin_iomem {
    pub iomem: *mut void __iomem,
    pub iomem): *mut *mut unsigned long (read)(void __iomem,
    pub data): *mut *mut *mut void (write)(void __iomem iomem, unsigned long,
    pub width: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intc_irqpin_irq {
    pub hw_irq: c_int,
    pub requested_irq: c_int,
    pub domain_irq: c_int,
    pub p: *mut intc_irqpin_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intc_irqpin_priv {
    pub iomem: [intc_irqpin_iomem; INTC_IRQPIN_REG_NR],
    pub irq: [intc_irqpin_irq; INTC_IRQPIN_MAX],
    pub sense_bitfield_width: c_uint,
    pub pdev: *mut platform_device,
    pub irq_chip: irq_chip,
    pub irq_domain: *mut irq_domain,
    pub wakeup_path: core::sync::atomic::AtomicI32,
    pub shared_irqs:1: unsigned,
    pub shared_irq_mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intc_irqpin_config {
    pub /: *mut *mut int irlm_bit; / -1 if non-existent,
}

#[no_mangle]
unsafe extern "C" fn intc_irqpin_read32(iomem: *mut void __iomem) -> c_ulong {
    static unsigned long intc_irqpin_read32(void __iomem *iomem)
    {
    return ioread32(iomem);
    }
#[no_mangle]
unsafe extern "C" fn intc_irqpin_read8(iomem: *mut void __iomem) -> c_ulong {
    static unsigned long intc_irqpin_read8(void __iomem *iomem)
    {
    return ioread8(iomem);
    }
#[no_mangle]
unsafe extern "C" fn intc_irqpin_write32(iomem: *mut void __iomem, data: c_ulong) {
    static void intc_irqpin_write32(void __iomem *iomem, unsigned long data)
    {
    iowrite32(data, iomem);
    }
#[no_mangle]
unsafe extern "C" fn intc_irqpin_write8(iomem: *mut void __iomem, data: c_ulong) {
    static void intc_irqpin_write8(void __iomem *iomem, unsigned long data)
    {
    iowrite8(data, iomem);
    }
    static inline unsigned long intc_irqpin_read(struct intc_irqpin_priv *p,
    int reg)
    {
    struct intc_irqpin_iomem *i = &p.iomem[reg];
    return i.read(i.iomem);
    }
    static inline void intc_irqpin_write(struct intc_irqpin_priv *p,
    int reg, unsigned long data)
    {
    struct intc_irqpin_iomem *i = &p.iomem[reg];
    i.write(i.iomem, data);
    }
    static inline unsigned long intc_irqpin_hwirq_mask(struct intc_irqpin_priv *p,
    int reg, int hw_irq)
    {
    return BIT((p.iomem[reg].width - 1) - hw_irq);
    }
    static inline void intc_irqpin_irq_write_hwirq(struct intc_irqpin_priv *p,
    int reg, int hw_irq)
    {
    intc_irqpin_write(p, reg, intc_irqpin_hwirq_mask(p, reg, hw_irq));
    }
    static DEFINE_RAW_SPINLOCK(intc_irqpin_lock); /* only used by slow path */
    static void intc_irqpin_read_modify_write(struct intc_irqpin_priv *p,
    int reg, int shift,
    int width, int value)
    {
    unsigned long flags;
    unsigned long tmp;
    raw_spin_lock_irqsave(&intc_irqpin_lock, flags);
    tmp = intc_irqpin_read(p, reg);
    tmp &= ~(((1 << width) - 1) << shift);
    tmp |= value << shift;
    intc_irqpin_write(p, reg, tmp);
    raw_spin_unlock_irqrestore(&intc_irqpin_lock, flags);
    }
    static void intc_irqpin_mask_unmask_prio(struct intc_irqpin_priv *p,
    int irq, int do_mask)
    {
// The PRIO register is assumed to be 32-bit with fixed 4-bit fields.
    let mut bitfield_width: c_int = 4;
    let mut shift: c_int = 32 - (irq + 1) * bitfield_width;
    intc_irqpin_read_modify_write(p, INTC_IRQPIN_REG_PRIO,
    shift, bitfield_width,
    do_mask ? 0 : (1 << bitfield_width) - 1);
    }
#[no_mangle]
unsafe extern "C" fn intc_irqpin_set_sense(p: *mut intc_irqpin_priv, irq: c_int, value: c_int) -> c_int {
    static int intc_irqpin_set_sense(struct intc_irqpin_priv *p, int irq, int value)
    {
// The SENSE register is assumed to be 32-bit.
    let mut bitfield_width: c_int = p.sense_bitfield_width;
    let mut shift: c_int = 32 - (irq + 1) * bitfield_width;
    dev_dbg(&p.pdev.dev, "sense irq = %d, mode = %d\n", irq, value);
    if (value >= (1 << bitfield_width))
    return -EINVAL;
    intc_irqpin_read_modify_write(p, INTC_IRQPIN_REG_SENSE, shift,
    bitfield_width, value);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intc_irqpin_dbg(i: *mut intc_irqpin_irq, str: *mut c_char) {
    static void intc_irqpin_dbg(struct intc_irqpin_irq *i, char *str)
    {
    dev_dbg(&i.p.pdev.dev, "%s (%d:%d:%d)\n",
    str, i.requested_irq, i.hw_irq, i.domain_irq);
    }
#[no_mangle]
unsafe extern "C" fn intc_irqpin_irq_enable(d: *mut irq_data) {
    static void intc_irqpin_irq_enable(struct irq_data *d)
    {
    struct intc_irqpin_priv *p = irq_data_get_irq_chip_data(d);
    let mut hw_irq: c_int = irqd_to_hwirq(d);
    intc_irqpin_dbg(&p.irq[hw_irq], "enable");
    intc_irqpin_irq_write_hwirq(p, INTC_IRQPIN_REG_CLEAR, hw_irq);
    }
#[no_mangle]
unsafe extern "C" fn intc_irqpin_irq_disable(d: *mut irq_data) {
    static void intc_irqpin_irq_disable(struct irq_data *d)
    {
    struct intc_irqpin_priv *p = irq_data_get_irq_chip_data(d);
    let mut hw_irq: c_int = irqd_to_hwirq(d);
    intc_irqpin_dbg(&p.irq[hw_irq], "disable");
    intc_irqpin_irq_write_hwirq(p, INTC_IRQPIN_REG_MASK, hw_irq);
    }
#[no_mangle]
unsafe extern "C" fn intc_irqpin_shared_irq_enable(d: *mut irq_data) {
    static void intc_irqpin_shared_irq_enable(struct irq_data *d)
    {
    struct intc_irqpin_priv *p = irq_data_get_irq_chip_data(d);
    let mut hw_irq: c_int = irqd_to_hwirq(d);
    intc_irqpin_dbg(&p.irq[hw_irq], "shared enable");
    intc_irqpin_irq_write_hwirq(p, INTC_IRQPIN_REG_CLEAR, hw_irq);
    p.shared_irq_mask &= ~BIT(hw_irq);
    }
#[no_mangle]
unsafe extern "C" fn intc_irqpin_shared_irq_disable(d: *mut irq_data) {
    static void intc_irqpin_shared_irq_disable(struct irq_data *d)
    {
    struct intc_irqpin_priv *p = irq_data_get_irq_chip_data(d);
    let mut hw_irq: c_int = irqd_to_hwirq(d);
    intc_irqpin_dbg(&p.irq[hw_irq], "shared disable");
    intc_irqpin_irq_write_hwirq(p, INTC_IRQPIN_REG_MASK, hw_irq);
    p.shared_irq_mask |= BIT(hw_irq);
    }
#[no_mangle]
unsafe extern "C" fn intc_irqpin_irq_enable_force(d: *mut irq_data) {
    static void intc_irqpin_irq_enable_force(struct irq_data *d)
    {
    struct intc_irqpin_priv *p = irq_data_get_irq_chip_data(d);
    let mut irq: c_int = p.irq[irqd_to_hwirq(d)].requested_irq;
    intc_irqpin_irq_enable(d);
// enable interrupt through parent interrupt controller,
// assumes non-shared interrupt with 1:1 mapping
// needed for busted IRQs on some SoCs like sh73a0
//
    irq_get_chip(irq).irq_unmask(irq_get_irq_data(irq));
    }
#[no_mangle]
unsafe extern "C" fn intc_irqpin_irq_disable_force(d: *mut irq_data) {
    static void intc_irqpin_irq_disable_force(struct irq_data *d)
    {
    struct intc_irqpin_priv *p = irq_data_get_irq_chip_data(d);
    let mut irq: c_int = p.irq[irqd_to_hwirq(d)].requested_irq;
// disable interrupt through parent interrupt controller,
// assumes non-shared interrupt with 1:1 mapping
// needed for busted IRQs on some SoCs like sh73a0
//
    irq_get_chip(irq).irq_mask(irq_get_irq_data(irq));
    intc_irqpin_irq_disable(d);
    }
pub const INTC_IRQ_SENSE_VALID: c_uint = 0x10;

    static unsigned char intc_irqpin_sense[IRQ_TYPE_SENSE_MASK + 1] = {
    [IRQ_TYPE_EDGE_FALLING] = INTC_IRQ_SENSE(0x00),
    [IRQ_TYPE_EDGE_RISING] = INTC_IRQ_SENSE(0x01),
    [IRQ_TYPE_LEVEL_LOW] = INTC_IRQ_SENSE(0x02),
    [IRQ_TYPE_LEVEL_HIGH] = INTC_IRQ_SENSE(0x03),
    [IRQ_TYPE_EDGE_BOTH] = INTC_IRQ_SENSE(0x04),
    };
#[no_mangle]
unsafe extern "C" fn intc_irqpin_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int intc_irqpin_irq_set_type(struct irq_data *d, unsigned int type)
    {
    let mut value: c_uchar = intc_irqpin_sense[type & IRQ_TYPE_SENSE_MASK];
    struct intc_irqpin_priv *p = irq_data_get_irq_chip_data(d);
    if (!(value & INTC_IRQ_SENSE_VALID))
    return -EINVAL;
    return intc_irqpin_set_sense(p, irqd_to_hwirq(d),
    value ^ INTC_IRQ_SENSE_VALID);
    }
#[no_mangle]
unsafe extern "C" fn intc_irqpin_irq_set_wake(d: *mut irq_data, on: c_uint) -> c_int {
    static int intc_irqpin_irq_set_wake(struct irq_data *d, unsigned int on)
    {
    struct intc_irqpin_priv *p = irq_data_get_irq_chip_data(d);
    let mut hw_irq: c_int = irqd_to_hwirq(d);
    irq_set_irq_wake(p.irq[hw_irq].requested_irq, on);
    if (on)
    atomic_inc(&p.wakeup_path);
    else
    atomic_dec(&p.wakeup_path);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intc_irqpin_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t intc_irqpin_irq_handler(int irq, void *dev_id)
    {
    struct intc_irqpin_irq *i = dev_id;
    struct intc_irqpin_priv *p = i.p;
    unsigned long bit;
    intc_irqpin_dbg(i, "demux1");
    bit = intc_irqpin_hwirq_mask(p, INTC_IRQPIN_REG_SOURCE, i.hw_irq);
    if (intc_irqpin_read(p, INTC_IRQPIN_REG_SOURCE) & bit) {
    intc_irqpin_write(p, INTC_IRQPIN_REG_SOURCE, ~bit);
    intc_irqpin_dbg(i, "demux2");
    generic_handle_irq(i.domain_irq);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn intc_irqpin_shared_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t intc_irqpin_shared_irq_handler(int irq, void *dev_id)
    {
    struct intc_irqpin_priv *p = dev_id;
    let mut reg_source: c_uint = intc_irqpin_read(p, INTC_IRQPIN_REG_SOURCE);
    let mut status: irqreturn_t = IRQ_NONE;
    int k;
    for (k = 0; k < 8; k++) {
    if (reg_source & BIT(7 - k)) {
    if (BIT(k) & p.shared_irq_mask)
    continue;
    status |= intc_irqpin_irq_handler(irq, &p.irq[k]);
    }
    }
    return status;
    }
//
// This lock class tells lockdep that INTC External IRQ Pin irqs are in a
// different category than their parents, so it won't report false recursion.
//
    static struct lock_class_key intc_irqpin_irq_lock_class;
// And this is for the request mutex
    static struct lock_class_key intc_irqpin_irq_request_class;
    static int intc_irqpin_irq_domain_map(struct irq_domain *h, unsigned int virq,
    irq_hw_number_t hw)
    {
    struct intc_irqpin_priv *p = h.host_data;
    p.irq[hw].domain_irq = virq;
    p.irq[hw].hw_irq = hw;
    intc_irqpin_dbg(&p.irq[hw], "map");
    irq_set_chip_data(virq, h.host_data);
    irq_set_lockdep_class(virq, &intc_irqpin_irq_lock_class,
    &intc_irqpin_irq_request_class);
    irq_set_chip_and_handler(virq, &p.irq_chip, handle_level_irq);
    return 0;
    }
    static const struct irq_domain_ops intc_irqpin_irq_domain_ops = {
    .map	= intc_irqpin_irq_domain_map,
    .xlate  = irq_domain_xlate_twocell,
    };
    static const struct intc_irqpin_config intc_irqpin_irlm_r8a777x = {
    .irlm_bit = 23, /* ICR0.IRLM0 */
    };
    static const struct intc_irqpin_config intc_irqpin_rmobile = {
    .irlm_bit = -1,
    };
    static const struct of_device_id intc_irqpin_dt_ids[] = {
    { .compatible = "renesas,intc-irqpin", },
    { .compatible = "renesas,intc-irqpin-r8a7778",
    .data = &intc_irqpin_irlm_r8a777x },
    { .compatible = "renesas,intc-irqpin-r8a7779",
    .data = &intc_irqpin_irlm_r8a777x },
    { .compatible = "renesas,intc-irqpin-r8a7740",
    .data = &intc_irqpin_rmobile },
    { .compatible = "renesas,intc-irqpin-sh73a0",
    .data = &intc_irqpin_rmobile },
    {},
    };
    MODULE_DEVICE_TABLE(of, intc_irqpin_dt_ids);
#[no_mangle]
unsafe extern "C" fn intc_irqpin_probe(pdev: *mut platform_device) -> c_int {
    static int intc_irqpin_probe(struct platform_device *pdev)
    {
    const struct intc_irqpin_config *config;
    struct device *dev = &pdev.dev;
    struct intc_irqpin_priv *p;
    struct intc_irqpin_iomem *i;
    struct resource *io[INTC_IRQPIN_REG_NR];
    struct irq_chip *irq_chip;
    void (*enable_fn)(struct irq_data *d);
    void (*disable_fn)(struct irq_data *d);
    const char *name = dev_name(dev);
    bool control_parent;
    unsigned int nirqs;
    int ref_irq;
    int ret;
    int k;
    p = devm_kzalloc(dev, sizeof(*p), GFP_KERNEL);
    if (!p)
    return -ENOMEM;
// deal with driver instance configuration
    of_property_read_u32(dev.of_node, "sense-bitfield-width",
    &p.sense_bitfield_width);
    control_parent = of_property_read_bool(dev.of_node, "control-parent");
    if (!p.sense_bitfield_width)
    p.sense_bitfield_width = 4; /* default to 4 bits */
    p.pdev = pdev;
    platform_set_drvdata(pdev, p);
    config = of_device_get_match_data(dev);
    pm_runtime_enable(dev);
    pm_runtime_get_sync(dev);
// get hold of register banks
    memset(io, 0, sizeof(io));
    for (k = 0; k < INTC_IRQPIN_REG_NR; k++) {
    io[k] = platform_get_resource(pdev, IORESOURCE_MEM, k);
    if (!io[k] && k < INTC_IRQPIN_REG_NR_MANDATORY) {
    dev_err(dev, "not enough IOMEM resources\n");
    ret = -EINVAL;
    goto err0;
    }
    }
// allow any number of IRQs between 1 and INTC_IRQPIN_MAX
    for (k = 0; k < INTC_IRQPIN_MAX; k++) {
    ret = platform_get_irq_optional(pdev, k);
    if (ret == -ENXIO)
    break;
    if (ret < 0)
    goto err0;
    p.irq[k].p = p;
    p.irq[k].requested_irq = ret;
    }
    nirqs = k;
    if (nirqs < 1) {
    dev_err(dev, "not enough IRQ resources\n");
    ret = -EINVAL;
    goto err0;
    }
// ioremap IOMEM and setup read/write callbacks
    for (k = 0; k < INTC_IRQPIN_REG_NR; k++) {
    i = &p.iomem[k];
// handle optional registers
    if (!io[k])
    continue;
    switch (resource_size(io[k])) {
    case 1:
    i.width = 8;
    i.read = intc_irqpin_read8;
    i.write = intc_irqpin_write8;
    break;
    case 4:
    i.width = 32;
    i.read = intc_irqpin_read32;
    i.write = intc_irqpin_write32;
    break;
    default:
    dev_err(dev, "IOMEM size mismatch\n");
    ret = -EINVAL;
    goto err0;
    }
    i.iomem = devm_ioremap(dev, io[k].start,
    resource_size(io[k]));
    if (!i.iomem) {
    dev_err(dev, "failed to remap IOMEM\n");
    ret = -ENXIO;
    goto err0;
    }
    }
// configure "individual IRQ mode" where needed
    if (config && config.irlm_bit >= 0) {
    if (io[INTC_IRQPIN_REG_IRLM])
    intc_irqpin_read_modify_write(p, INTC_IRQPIN_REG_IRLM,
    config.irlm_bit, 1, 1);
    else
    dev_warn(dev, "unable to select IRLM mode\n");
    }
// mask all interrupts using priority
    for (k = 0; k < nirqs; k++)
    intc_irqpin_mask_unmask_prio(p, k, 1);
// clear all pending interrupts
    intc_irqpin_write(p, INTC_IRQPIN_REG_SOURCE, 0x0);
// scan for shared interrupt lines
    ref_irq = p.irq[0].requested_irq;
    p.shared_irqs = 1;
    for (k = 1; k < nirqs; k++) {
    if (ref_irq != p.irq[k].requested_irq) {
    p.shared_irqs = 0;
    break;
    }
    }
// use more severe masking method if requested
    if (control_parent) {
    enable_fn = intc_irqpin_irq_enable_force;
    disable_fn = intc_irqpin_irq_disable_force;
    } else if (!p.shared_irqs) {
    enable_fn = intc_irqpin_irq_enable;
    disable_fn = intc_irqpin_irq_disable;
    } else {
    enable_fn = intc_irqpin_shared_irq_enable;
    disable_fn = intc_irqpin_shared_irq_disable;
    }
    irq_chip = &p.irq_chip;
    irq_chip.name = "intc-irqpin";
    irq_chip.irq_mask = disable_fn;
    irq_chip.irq_unmask = enable_fn;
    irq_chip.irq_set_type = intc_irqpin_irq_set_type;
    irq_chip.irq_set_wake = intc_irqpin_irq_set_wake;
    irq_chip.flags	= IRQCHIP_MASK_ON_SUSPEND;
    p.irq_domain = irq_domain_create_simple(dev_fwnode(dev), nirqs, 0,
    &intc_irqpin_irq_domain_ops, p);
    if (!p.irq_domain) {
    ret = -ENXIO;
    dev_err(dev, "cannot initialize irq domain\n");
    goto err0;
    }
    irq_domain_set_pm_device(p.irq_domain, dev);
    if (p.shared_irqs) {
// request one shared interrupt
    if (devm_request_irq(dev, p.irq[0].requested_irq,
    intc_irqpin_shared_irq_handler,
    IRQF_SHARED, name, p)) {
    dev_err(dev, "failed to request low IRQ\n");
    ret = -ENOENT;
    goto err1;
    }
    } else {
// request interrupts one by one
    for (k = 0; k < nirqs; k++) {
    if (devm_request_irq(dev, p.irq[k].requested_irq,
    intc_irqpin_irq_handler, 0, name,
    &p.irq[k])) {
    dev_err(dev, "failed to request low IRQ\n");
    ret = -ENOENT;
    goto err1;
    }
    }
    }
// unmask all interrupts on prio level
    for (k = 0; k < nirqs; k++)
    intc_irqpin_mask_unmask_prio(p, k, 0);
    dev_info(dev, "driving %d irqs\n", nirqs);
    return 0;
    err1:
    irq_domain_remove(p.irq_domain);
    err0:
    pm_runtime_put(dev);
    pm_runtime_disable(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn intc_irqpin_remove(pdev: *mut platform_device) {
    static void intc_irqpin_remove(struct platform_device *pdev)
    {
    struct intc_irqpin_priv *p = platform_get_drvdata(pdev);
    irq_domain_remove(p.irq_domain);
    pm_runtime_put(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn intc_irqpin_suspend(dev: *mut device) -> c_int {
    static int intc_irqpin_suspend(struct device *dev)
    {
    struct intc_irqpin_priv *p = dev_get_drvdata(dev);
    if (atomic_read(&p.wakeup_path))
    device_set_wakeup_path(dev);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(intc_irqpin_pm_ops, intc_irqpin_suspend, core::ptr::null_mut());
    static struct platform_driver intc_irqpin_device_driver = {
    .probe		= intc_irqpin_probe,
    .remove		= intc_irqpin_remove,
    .driver		= {
    .name		= "renesas_intc_irqpin",
    .of_match_table	= intc_irqpin_dt_ids,
    .pm		= pm_sleep_ptr(&intc_irqpin_pm_ops),
    }
    };
#[no_mangle]
unsafe extern "C" fn intc_irqpin_init() -> int __init {
    static int __init intc_irqpin_init(void)
    {
    return platform_driver_register(&intc_irqpin_device_driver);
    }
    postcore_initcall(intc_irqpin_init);
#[no_mangle]
unsafe extern "C" fn intc_irqpin_exit() -> void __exit {
    static void __exit intc_irqpin_exit(void)
    {
    platform_driver_unregister(&intc_irqpin_device_driver);
    }
    module_exit(intc_irqpin_exit);
    MODULE_AUTHOR("Magnus Damm");
    MODULE_DESCRIPTION("Renesas INTC External IRQ Pin Driver");
