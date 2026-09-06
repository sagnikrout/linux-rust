//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-bcm6345-l1.c
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
// Broadcom BCM6345 style Level 1 interrupt controller driver
//
// Copyright (C) 2014 Broadcom Corporation
// Copyright 2015 Simon Arlott
//
// This is based on the BCM7038 (which supports SMP) but with a single
// enable register instead of separate mask/set/clear registers.
//
// The BCM3380 has a similar mask/status register layout, but each pair
// of words is at separate locations (and SMP is not supported).
//
// ENABLE/STATUS words are packed next to each other for each CPU:
//
// BCM6368:
// 0x1000_0020: CPU0_W0_ENABLE
// 0x1000_0024: CPU0_W1_ENABLE
// 0x1000_0028: CPU0_W0_STATUS		IRQs 31-63
// 0x1000_002c: CPU0_W1_STATUS		IRQs 0-31
// 0x1000_0030: CPU1_W0_ENABLE
// 0x1000_0034: CPU1_W1_ENABLE
// 0x1000_0038: CPU1_W0_STATUS		IRQs 31-63
// 0x1000_003c: CPU1_W1_STATUS		IRQs 0-31
//
// BCM63168:
// 0x1000_0020: CPU0_W0_ENABLE
// 0x1000_0024: CPU0_W1_ENABLE
// 0x1000_0028: CPU0_W2_ENABLE
// 0x1000_002c: CPU0_W3_ENABLE
// 0x1000_0030: CPU0_W0_STATUS	IRQs 96-127
// 0x1000_0034: CPU0_W1_STATUS	IRQs 64-95
// 0x1000_0038: CPU0_W2_STATUS	IRQs 32-63
// 0x1000_003c: CPU0_W3_STATUS	IRQs 0-31
// 0x1000_0040: CPU1_W0_ENABLE
// 0x1000_0044: CPU1_W1_ENABLE
// 0x1000_0048: CPU1_W2_ENABLE
// 0x1000_004c: CPU1_W3_ENABLE
// 0x1000_0050: CPU1_W0_STATUS	IRQs 96-127
// 0x1000_0054: CPU1_W1_STATUS	IRQs 64-95
// 0x1000_0058: CPU1_W2_STATUS	IRQs 32-63
// 0x1000_005c: CPU1_W3_STATUS	IRQs 0-31
//
// IRQs are numbered in CPU native endian order
// (which is big-endian in these examples)
//

pub const IRQS_PER_WORD: c_int = 32;

    struct bcm6345_l1_cpu;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm6345_l1_chip {
    pub lock: raw_spinlock_t,
    pub n_words: c_uint,
    pub domain: *mut irq_domain,
    pub cpumask: cpumask,
    pub cpus: [*mut bcm6345_l1_cpu; NR_CPUS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm6345_l1_cpu {
    pub intc: *mut bcm6345_l1_chip,
    pub map_base: *mut void __iomem,
    pub parent_irq: c_uint,
    pub enable_cache: [u32; ],
}

    static inline unsigned int reg_enable(struct bcm6345_l1_chip *intc,
    unsigned int word)
    {

    return (1 * intc.n_words - word - 1) * sizeof(u32);

    return (0 * intc.n_words + word) * sizeof(u32);

    }
    static inline unsigned int reg_status(struct bcm6345_l1_chip *intc,
    unsigned int word)
    {

    return (2 * intc.n_words - word - 1) * sizeof(u32);

    return (1 * intc.n_words + word) * sizeof(u32);

    }
    static inline unsigned int cpu_for_irq(struct bcm6345_l1_chip *intc,
    struct irq_data *d)
    {
    return cpumask_first_and(&intc.cpumask, irq_data_get_affinity_mask(d));
    }
#[no_mangle]
unsafe extern "C" fn bcm6345_l1_irq_handle(desc: *mut irq_desc) {
    static void bcm6345_l1_irq_handle(struct irq_desc *desc)
    {
    struct bcm6345_l1_cpu *cpu = irq_desc_get_handler_data(desc);
    struct bcm6345_l1_chip *intc = cpu.intc;
    struct irq_chip *chip = irq_desc_get_chip(desc);
    unsigned int idx;
    chained_irq_enter(chip, desc);
    for (idx = 0; idx < intc.n_words; idx++) {
    let mut base: c_int = idx * IRQS_PER_WORD;
    unsigned long pending;
    irq_hw_number_t hwirq;
    pending = __raw_readl(cpu.map_base + reg_status(intc, idx));
    pending &= __raw_readl(cpu.map_base + reg_enable(intc, idx));
    for_each_set_bit(hwirq, &pending, IRQS_PER_WORD) {
    if (generic_handle_domain_irq(intc.domain, base + hwirq))
    spurious_interrupt();
    }
    }
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
pub unsafe extern "C" fn __bcm6345_l1_unmask(d: *mut irq_data) {
    static inline void __bcm6345_l1_unmask(struct irq_data *d)
    {
    struct bcm6345_l1_chip *intc = irq_data_get_irq_chip_data(d);
    let mut word: u32 = d.hwirq / IRQS_PER_WORD;
    let mut mask: u32 = BIT(d.hwirq % IRQS_PER_WORD);
    let mut cpu_idx: c_uint = cpu_for_irq(intc, d);
    intc.cpus[cpu_idx].enable_cache[word] |= mask;
    __raw_writel(intc.cpus[cpu_idx].enable_cache[word],
    intc.cpus[cpu_idx].map_base + reg_enable(intc, word));
    }
#[no_mangle]
pub unsafe extern "C" fn __bcm6345_l1_mask(d: *mut irq_data) {
    static inline void __bcm6345_l1_mask(struct irq_data *d)
    {
    struct bcm6345_l1_chip *intc = irq_data_get_irq_chip_data(d);
    let mut word: u32 = d.hwirq / IRQS_PER_WORD;
    let mut mask: u32 = BIT(d.hwirq % IRQS_PER_WORD);
    let mut cpu_idx: c_uint = cpu_for_irq(intc, d);
    intc.cpus[cpu_idx].enable_cache[word] &= ~mask;
    __raw_writel(intc.cpus[cpu_idx].enable_cache[word],
    intc.cpus[cpu_idx].map_base + reg_enable(intc, word));
    }
#[no_mangle]
unsafe extern "C" fn bcm6345_l1_unmask(d: *mut irq_data) {
    static void bcm6345_l1_unmask(struct irq_data *d)
    {
    struct bcm6345_l1_chip *intc = irq_data_get_irq_chip_data(d);
    unsigned long flags;
    raw_spin_lock_irqsave(&intc.lock, flags);
    __bcm6345_l1_unmask(d);
    raw_spin_unlock_irqrestore(&intc.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn bcm6345_l1_mask(d: *mut irq_data) {
    static void bcm6345_l1_mask(struct irq_data *d)
    {
    struct bcm6345_l1_chip *intc = irq_data_get_irq_chip_data(d);
    unsigned long flags;
    raw_spin_lock_irqsave(&intc.lock, flags);
    __bcm6345_l1_mask(d);
    raw_spin_unlock_irqrestore(&intc.lock, flags);
    }
    static int bcm6345_l1_set_affinity(struct irq_data *d,
    const struct cpumask *dest,
    bool force)
    {
    struct bcm6345_l1_chip *intc = irq_data_get_irq_chip_data(d);
    let mut word: u32 = d.hwirq / IRQS_PER_WORD;
    let mut mask: u32 = BIT(d.hwirq % IRQS_PER_WORD);
    let mut old_cpu: c_uint = cpu_for_irq(intc, d);
    unsigned int new_cpu;
    unsigned long flags;
    bool enabled;
    new_cpu = cpumask_first_and_and(&intc.cpumask, dest, cpu_online_mask);
    if (new_cpu >= nr_cpu_ids)
    return -EINVAL;
    dest = cpumask_of(new_cpu);
    raw_spin_lock_irqsave(&intc.lock, flags);
    if (old_cpu != new_cpu) {
    enabled = intc.cpus[old_cpu].enable_cache[word] & mask;
    if (enabled)
    __bcm6345_l1_mask(d);
    irq_data_update_affinity(d, dest);
    if (enabled)
    __bcm6345_l1_unmask(d);
    } else {
    irq_data_update_affinity(d, dest);
    }
    raw_spin_unlock_irqrestore(&intc.lock, flags);
    irq_data_update_effective_affinity(d, cpumask_of(new_cpu));
    return IRQ_SET_MASK_OK_NOCOPY;
    }
    static int __init bcm6345_l1_init_one(struct device_node *dn,
    unsigned int idx,
    struct bcm6345_l1_chip *intc)
    {
    struct resource res;
    resource_size_t sz;
    struct bcm6345_l1_cpu *cpu;
    unsigned int i, n_words;
    if (of_address_to_resource(dn, idx, &res))
    return -EINVAL;
    sz = resource_size(&res);
    n_words = sz / REG_BYTES_PER_IRQ_WORD;
    if (!intc.n_words)
    intc.n_words = n_words;
#[no_mangle]
pub unsafe extern "C" fn if(n_words: intc->n_words !=) -> else {
    else if (intc.n_words != n_words)
    return -EINVAL;
    cpu = intc.cpus[idx] = kzalloc_flex(*cpu, enable_cache, n_words);
    if (!cpu)
    return -ENOMEM;
    cpu.intc = intc;
    cpu.map_base = ioremap(res.start, sz);
    if (!cpu.map_base)
    return -ENOMEM;
    if (!request_mem_region(res.start, sz, res.name))
    pr_err("failed to request intc memory");
    for (i = 0; i < n_words; i++) {
    cpu.enable_cache[i] = 0;
    __raw_writel(0, cpu.map_base + reg_enable(intc, i));
    }
    cpu.parent_irq = irq_of_parse_and_map(dn, idx);
    if (!cpu.parent_irq) {
    pr_err("failed to map parent interrupt %d\n", cpu.parent_irq);
    return -EINVAL;
    }
    irq_set_chained_handler_and_data(cpu.parent_irq,
    bcm6345_l1_irq_handle, cpu);
    return 0;
    }
    static struct irq_chip bcm6345_l1_irq_chip = {
    .name			= "bcm6345-l1",
    .irq_mask		= bcm6345_l1_mask,
    .irq_unmask		= bcm6345_l1_unmask,
    .irq_set_affinity	= bcm6345_l1_set_affinity,
    };
    static int bcm6345_l1_map(struct irq_domain *d, unsigned int virq,
    irq_hw_number_t hw_irq)
    {
    irq_set_chip_and_handler(virq,
    &bcm6345_l1_irq_chip, handle_percpu_irq);
    irq_set_chip_data(virq, d.host_data);
    irqd_set_single_target(irq_desc_get_irq_data(irq_to_desc(virq)));
    return 0;
    }
    static const struct irq_domain_ops bcm6345_l1_domain_ops = {
    .xlate			= irq_domain_xlate_onecell,
    .map			= bcm6345_l1_map,
    };
    static int __init bcm6345_l1_of_init(struct device_node *dn,
    struct device_node *parent)
    {
    struct bcm6345_l1_chip *intc;
    unsigned int idx;
    int ret;
    intc = kzalloc_obj(*intc);
    if (!intc)
    return -ENOMEM;
    for_each_possible_cpu(idx) {
    ret = bcm6345_l1_init_one(dn, idx, intc);
    if (ret)
    pr_err("failed to init intc L1 for cpu %d: %d\n",
    idx, ret);
    else
    cpumask_set_cpu(idx, &intc.cpumask);
    }
    if (cpumask_empty(&intc.cpumask)) {
    ret = -ENODEV;
    goto out_free;
    }
    raw_spin_lock_init(&intc.lock);
    intc.domain = irq_domain_create_linear(of_fwnode_handle(dn), IRQS_PER_WORD * intc.n_words,
    &bcm6345_l1_domain_ops,
    intc);
    if (!intc.domain) {
    ret = -ENOMEM;
    goto out_unmap;
    }
    pr_info("registered BCM6345 L1 intc (IRQs: %d)\n",
    IRQS_PER_WORD * intc.n_words);
    for_each_cpu(idx, &intc.cpumask) {
    struct bcm6345_l1_cpu *cpu = intc.cpus[idx];
    pr_info("  CPU%u (irq = %d)\n", idx, cpu.parent_irq);
    }
    return 0;
    out_unmap:
    for_each_possible_cpu(idx) {
    struct bcm6345_l1_cpu *cpu = intc.cpus[idx];
    if (cpu) {
    if (cpu.map_base)
    iounmap(cpu.map_base);
    kfree(cpu);
    }
    }
    out_free:
    kfree(intc);
    return ret;
    }
    IRQCHIP_DECLARE(bcm6345_l1, "brcm,bcm6345-l1-intc", bcm6345_l1_of_init);
