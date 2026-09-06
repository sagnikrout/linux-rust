//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-csky-mpintc.c
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
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

    static struct irq_domain *root_domain;
    static void __iomem *INTCG_base;
    static void __iomem *INTCL_base;
pub const IPI_IRQ: c_int = 15;
pub const INTC_IRQS: c_int = 256;
pub const COMM_IRQ_BASE: c_int = 32;
pub const INTCG_SIZE: c_uint = 0x8000;
pub const INTCL_SIZE: c_uint = 0x1000;
pub const INTCG_ICTLR: c_uint = 0x0;
pub const INTCG_CICFGR: c_uint = 0x100;
pub const INTCG_CIDSTR: c_uint = 0x1000;
pub const INTCL_PICTLR: c_uint = 0x0;
pub const INTCL_CFGR: c_uint = 0x14;
pub const INTCL_SIGR: c_uint = 0x60;
pub const INTCL_RDYIR: c_uint = 0x6c;
pub const INTCL_SENR: c_uint = 0xa0;
pub const INTCL_CENR: c_uint = 0xa4;
pub const INTCL_CACR: c_uint = 0xb4;
    static DEFINE_PER_CPU(void __iomem *, intcl_reg);
    static unsigned long *__trigger;

    (TRIG_BYTE_OFFSET(IRQ_OFFSET(irq)) + ((irq < COMM_IRQ_BASE) ? \
    (this_cpu_read(intcl_reg) + INTCL_CFGR) : (INTCG_base + INTCG_CICFGR)))
    static DEFINE_SPINLOCK(setup_lock);
#[no_mangle]
unsafe extern "C" fn setup_trigger(irq: c_ulong, trigger: c_ulong) {
    static void setup_trigger(unsigned long irq, unsigned long trigger)
    {
    unsigned int tmp;
    spin_lock(&setup_lock);
// setup trigger
    tmp = readl_relaxed(TRIG_BASE(irq)) & TRIG_VAL_MSK(irq);
    writel_relaxed(tmp | TRIG_VAL(trigger, irq), TRIG_BASE(irq));
    spin_unlock(&setup_lock);
    }
#[no_mangle]
unsafe extern "C" fn csky_mpintc_handler(regs: *mut pt_regs) {
    static void csky_mpintc_handler(struct pt_regs *regs)
    {
    void __iomem *reg_base = this_cpu_read(intcl_reg);
    generic_handle_domain_irq(root_domain,
    readl_relaxed(reg_base + INTCL_RDYIR));
    }
#[no_mangle]
unsafe extern "C" fn csky_mpintc_unmask(d: *mut irq_data) {
    static void csky_mpintc_unmask(struct irq_data *d)
    {
    void __iomem *reg_base = this_cpu_read(intcl_reg);
    setup_trigger(d.hwirq, __trigger[d.hwirq]);
    writel_relaxed(d.hwirq, reg_base + INTCL_SENR);
    }
#[no_mangle]
unsafe extern "C" fn csky_mpintc_mask(d: *mut irq_data) {
    static void csky_mpintc_mask(struct irq_data *d)
    {
    void __iomem *reg_base = this_cpu_read(intcl_reg);
    writel_relaxed(d.hwirq, reg_base + INTCL_CENR);
    }
#[no_mangle]
unsafe extern "C" fn csky_mpintc_eoi(d: *mut irq_data) {
    static void csky_mpintc_eoi(struct irq_data *d)
    {
    void __iomem *reg_base = this_cpu_read(intcl_reg);
    writel_relaxed(d.hwirq, reg_base + INTCL_CACR);
    }
#[no_mangle]
unsafe extern "C" fn csky_mpintc_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int csky_mpintc_set_type(struct irq_data *d, unsigned int type)
    {
    switch (type & IRQ_TYPE_SENSE_MASK) {
    case IRQ_TYPE_LEVEL_HIGH:
    __trigger[d.hwirq] = 0;
    break;
    case IRQ_TYPE_LEVEL_LOW:
    __trigger[d.hwirq] = 1;
    break;
    case IRQ_TYPE_EDGE_RISING:
    __trigger[d.hwirq] = 2;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    __trigger[d.hwirq] = 3;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }

    static int csky_irq_set_affinity(struct irq_data *d,
    const struct cpumask *mask_val,
    bool force)
    {
    unsigned int cpu;
    let mut offset: c_uint = 4 * (d.hwirq - COMM_IRQ_BASE);
    if (!force)
    cpu = cpumask_any_and(mask_val, cpu_online_mask);
    else
    cpu = cpumask_first(mask_val);
    if (cpu >= nr_cpu_ids)
    return -EINVAL;
//
// The csky,mpintc could support auto irq deliver, but it only
// could deliver external irq to one cpu or all cpus. So it
// doesn't support deliver external irq to a group of cpus
// with cpu_mask.
// SO we only use auto deliver mode when affinity mask_val is
// equal to cpu_present_mask.
//
    if (cpumask_equal(mask_val, cpu_present_mask))
    cpu = 0;
    else
    cpu |= BIT(31);
    writel_relaxed(cpu, INTCG_base + INTCG_CIDSTR + offset);
    irq_data_update_effective_affinity(d, cpumask_of(cpu));
    return IRQ_SET_MASK_OK_DONE;
    }

    static struct irq_chip csky_irq_chip = {
    .name           = "C-SKY SMP Intc",
    .irq_eoi	= csky_mpintc_eoi,
    .irq_unmask	= csky_mpintc_unmask,
    .irq_mask	= csky_mpintc_mask,
    .irq_set_type	= csky_mpintc_set_type,

    .irq_set_affinity = csky_irq_set_affinity,

    };
    static int csky_irqdomain_map(struct irq_domain *d, unsigned int irq,
    irq_hw_number_t hwirq)
    {
    if (hwirq < COMM_IRQ_BASE) {
    irq_set_percpu_devid(irq);
    irq_set_chip_and_handler(irq, &csky_irq_chip,
    handle_percpu_irq);
    } else {
    irq_set_chip_and_handler(irq, &csky_irq_chip,
    handle_fasteoi_irq);
    }
    return 0;
    }
    static int csky_irq_domain_xlate_cells(struct irq_domain *d,
    struct device_node *ctrlr, const u32 *intspec,
    unsigned int intsize, unsigned long *out_hwirq,
    unsigned int *out_type)
    {
    if (WARN_ON(intsize < 1))
    return -EINVAL;
// out_hwirq = intspec[0];
    if (intsize > 1)
// out_type = intspec[1] & IRQ_TYPE_SENSE_MASK;
    else
// out_type = IRQ_TYPE_LEVEL_HIGH;
    return 0;
    }
    static const struct irq_domain_ops csky_irqdomain_ops = {
    .map	= csky_irqdomain_map,
    .xlate	= csky_irq_domain_xlate_cells,
    };

#[no_mangle]
unsafe extern "C" fn csky_mpintc_send_ipi(mask: *const cpumask) {
    static void csky_mpintc_send_ipi(const struct cpumask *mask)
    {
    void __iomem *reg_base = this_cpu_read(intcl_reg);
//
// INTCL_SIGR[3:0] INTID
// INTCL_SIGR[8:15] CPUMASK
//
    writel_relaxed((*cpumask_bits(mask)) << 8 | IPI_IRQ,
    reg_base + INTCL_SIGR);
    }

// C-SKY multi processor interrupt controller
    static int __init
    csky_mpintc_init(struct device_node *node, struct device_node *parent)
    {
    int ret;
    unsigned int cpu, nr_irq;

    unsigned int ipi_irq;

    if (parent)
    return 0;
    ret = of_property_read_u32(node, "csky,num-irqs", &nr_irq);
    if (ret < 0)
    nr_irq = INTC_IRQS;
    __trigger  = kcalloc(nr_irq, sizeof(unsigned long), GFP_KERNEL);
    if (__trigger == core::ptr::null_mut())
    return -ENXIO;
    if (INTCG_base == core::ptr::null_mut()) {
    INTCG_base = ioremap(mfcr("cr<31, 14>"),
    INTCL_SIZE*nr_cpu_ids + INTCG_SIZE);
    if (INTCG_base == core::ptr::null_mut())
    return -EIO;
    INTCL_base = INTCG_base + INTCG_SIZE;
    writel_relaxed(BIT(0), INTCG_base + INTCG_ICTLR);
    }
    root_domain = irq_domain_create_linear(of_fwnode_handle(node), nr_irq, &csky_irqdomain_ops,
    core::ptr::null_mut());
    if (!root_domain)
    return -ENXIO;
// for every cpu
    for_each_present_cpu(cpu) {
    per_cpu(intcl_reg, cpu) = INTCL_base + (INTCL_SIZE * cpu);
    writel_relaxed(BIT(0), per_cpu(intcl_reg, cpu) + INTCL_PICTLR);
    }
    set_handle_irq(&csky_mpintc_handler);

    ipi_irq = irq_create_mapping(root_domain, IPI_IRQ);
    if (!ipi_irq)
    return -EIO;
    set_send_ipi(&csky_mpintc_send_ipi, ipi_irq);

    return 0;
    }
    IRQCHIP_DECLARE(csky_mpintc, "csky,mpintc", csky_mpintc_init);
