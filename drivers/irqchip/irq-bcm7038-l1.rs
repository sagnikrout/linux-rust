//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-bcm7038-l1.c
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
// Broadcom BCM7038 style Level 1 interrupt controller driver
//
// Copyright (C) 2014 Broadcom Corporation
// Author: Kevin Cernekee
//

pub const IRQS_PER_WORD: c_int = 32;

pub const MAX_WORDS: c_int = 8;
    struct bcm7038_l1_cpu;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm7038_l1_chip {
    pub lock: raw_spinlock_t,
    pub n_words: c_uint,
    pub domain: *mut irq_domain,
    pub cpus: [*mut bcm7038_l1_cpu; NR_CPUS],
    pub list: list_head,
    pub wake_mask: [u32; MAX_WORDS],    pub irq_fwd_mask: [u32; MAX_WORDS],
    pub IRQS_PER_WORD]: *mut *mut u8 affinity[MAX_WORDS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm7038_l1_cpu {
    pub map_base: *mut void __iomem,
    pub mask_cache: [u32; ],
}

//
// STATUS/MASK_STATUS/MASK_SET/MASK_CLEAR are packed one right after another:
//
// 7038:
// 0x1000_1400: W0_STATUS
// 0x1000_1404: W1_STATUS
// 0x1000_1408: W0_MASK_STATUS
// 0x1000_140c: W1_MASK_STATUS
// 0x1000_1410: W0_MASK_SET
// 0x1000_1414: W1_MASK_SET
// 0x1000_1418: W0_MASK_CLEAR
// 0x1000_141c: W1_MASK_CLEAR
//
// 7445:
// 0xf03e_1500: W0_STATUS
// 0xf03e_1504: W1_STATUS
// 0xf03e_1508: W2_STATUS
// 0xf03e_150c: W3_STATUS
// 0xf03e_1510: W4_STATUS
// 0xf03e_1514: W0_MASK_STATUS
// 0xf03e_1518: W1_MASK_STATUS
// [...]
//
    static inline unsigned int reg_status(struct bcm7038_l1_chip *intc,
    unsigned int word)
    {
    return (0 * intc.n_words + word) * sizeof(u32);
    }
    static inline unsigned int reg_mask_set(struct bcm7038_l1_chip *intc,
    unsigned int word)
    {
    return (2 * intc.n_words + word) * sizeof(u32);
    }
    static inline unsigned int reg_mask_clr(struct bcm7038_l1_chip *intc,
    unsigned int word)
    {
    return (3 * intc.n_words + word) * sizeof(u32);
    }
#[no_mangle]
pub unsafe extern "C" fn l1_readl(reg: *mut void __iomem) -> u32 {
    static inline u32 l1_readl(void __iomem *reg)
    {
    if (IS_ENABLED(CONFIG_MIPS) && IS_ENABLED(CONFIG_CPU_BIG_ENDIAN))
    return ioread32be(reg);
    else
    return readl(reg);
    }
#[no_mangle]
pub unsafe extern "C" fn l1_writel(val: u32, reg: *mut void __iomem) {
    static inline void l1_writel(u32 val, void __iomem *reg)
    {
    if (IS_ENABLED(CONFIG_MIPS) && IS_ENABLED(CONFIG_CPU_BIG_ENDIAN))
    iowrite32be(val, reg);
    else
    writel(val, reg);
    }
#[no_mangle]
unsafe extern "C" fn bcm7038_l1_irq_handle(desc: *mut irq_desc) {
    static void bcm7038_l1_irq_handle(struct irq_desc *desc)
    {
    struct bcm7038_l1_chip *intc = irq_desc_get_handler_data(desc);
    struct bcm7038_l1_cpu *cpu;
    struct irq_chip *chip = irq_desc_get_chip(desc);
    unsigned int idx;

    cpu = intc.cpus[cpu_logical_map(smp_processor_id())];

    cpu = intc.cpus[0];

    chained_irq_enter(chip, desc);
    for (idx = 0; idx < intc.n_words; idx++) {
    let mut base: c_int = idx * IRQS_PER_WORD;
    unsigned long pending, flags;
    int hwirq;
    raw_spin_lock_irqsave(&intc.lock, flags);
    pending = l1_readl(cpu.map_base + reg_status(intc, idx)) &
    ~cpu.mask_cache[idx];
    raw_spin_unlock_irqrestore(&intc.lock, flags);
    for_each_set_bit(hwirq, &pending, IRQS_PER_WORD)
    generic_handle_domain_irq(intc.domain, base + hwirq);
    }
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn __bcm7038_l1_unmask(d: *mut irq_data, cpu_idx: c_uint) {
    static void __bcm7038_l1_unmask(struct irq_data *d, unsigned int cpu_idx)
    {
    struct bcm7038_l1_chip *intc = irq_data_get_irq_chip_data(d);
    let mut word: u32 = d.hwirq / IRQS_PER_WORD;
    let mut mask: u32 = BIT(d.hwirq % IRQS_PER_WORD);
    intc.cpus[cpu_idx].mask_cache[word] &= ~mask;
    l1_writel(mask, intc.cpus[cpu_idx].map_base +
    reg_mask_clr(intc, word));
    }
#[no_mangle]
unsafe extern "C" fn __bcm7038_l1_mask(d: *mut irq_data, cpu_idx: c_uint) {
    static void __bcm7038_l1_mask(struct irq_data *d, unsigned int cpu_idx)
    {
    struct bcm7038_l1_chip *intc = irq_data_get_irq_chip_data(d);
    let mut word: u32 = d.hwirq / IRQS_PER_WORD;
    let mut mask: u32 = BIT(d.hwirq % IRQS_PER_WORD);
    intc.cpus[cpu_idx].mask_cache[word] |= mask;
    l1_writel(mask, intc.cpus[cpu_idx].map_base +
    reg_mask_set(intc, word));
    }
#[no_mangle]
unsafe extern "C" fn bcm7038_l1_unmask(d: *mut irq_data) {
    static void bcm7038_l1_unmask(struct irq_data *d)
    {
    struct bcm7038_l1_chip *intc = irq_data_get_irq_chip_data(d);
    unsigned long flags;
    raw_spin_lock_irqsave(&intc.lock, flags);
    __bcm7038_l1_unmask(d, intc.affinity[d.hwirq]);
    raw_spin_unlock_irqrestore(&intc.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn bcm7038_l1_mask(d: *mut irq_data) {
    static void bcm7038_l1_mask(struct irq_data *d)
    {
    struct bcm7038_l1_chip *intc = irq_data_get_irq_chip_data(d);
    unsigned long flags;
    raw_spin_lock_irqsave(&intc.lock, flags);
    __bcm7038_l1_mask(d, intc.affinity[d.hwirq]);
    raw_spin_unlock_irqrestore(&intc.lock, flags);
    }

    static int bcm7038_l1_set_affinity(struct irq_data *d,
    const struct cpumask *dest,
    bool force)
    {
    struct bcm7038_l1_chip *intc = irq_data_get_irq_chip_data(d);
    unsigned long flags;
    let mut hw: irq_hw_number_t = d.hwirq;
    let mut word: u32 = hw / IRQS_PER_WORD;
    let mut mask: u32 = BIT(hw % IRQS_PER_WORD);
    let mut first_cpu: c_uint = cpumask_any_and(dest, cpu_online_mask);
    bool was_disabled;
    raw_spin_lock_irqsave(&intc.lock, flags);
    was_disabled = !!(intc.cpus[intc.affinity[hw]].mask_cache[word] &
    mask);
    __bcm7038_l1_mask(d, intc.affinity[hw]);
    intc.affinity[hw] = first_cpu;
    if (!was_disabled)
    __bcm7038_l1_unmask(d, first_cpu);
    raw_spin_unlock_irqrestore(&intc.lock, flags);
    irq_data_update_effective_affinity(d, cpumask_of(first_cpu));
    return 0;
    }

    static int bcm7038_l1_init_one(struct device_node *dn, unsigned int idx,
    struct bcm7038_l1_chip *intc)
    {
    struct resource res;
    resource_size_t sz;
    struct bcm7038_l1_cpu *cpu;
    unsigned int i, n_words, parent_irq;
    int ret;
    if (of_address_to_resource(dn, idx, &res))
    return -EINVAL;
    sz = resource_size(&res);
    n_words = sz / REG_BYTES_PER_IRQ_WORD;
    if (n_words > MAX_WORDS)
    return -EINVAL;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !intc->n_words) -> else {
    else if (!intc.n_words)
    intc.n_words = n_words;
#[no_mangle]
pub unsafe extern "C" fn if(n_words: intc->n_words !=) -> else {
    else if (intc.n_words != n_words)
    return -EINVAL;
    ret = of_property_read_u32_array(dn , "brcm,int-fwd-mask",
    intc.irq_fwd_mask, n_words);
    if (ret != 0 && ret != -EINVAL) {
// property exists but has the wrong number of words
    pr_err("invalid brcm,int-fwd-mask property\n");
    return -EINVAL;
    }
    cpu = intc.cpus[idx] = kzalloc_flex(*cpu, mask_cache, n_words);
    if (!cpu)
    return -ENOMEM;
    cpu.map_base = ioremap(res.start, sz);
    if (!cpu.map_base)
    return -ENOMEM;
    for (i = 0; i < n_words; i++) {
    l1_writel(~intc.irq_fwd_mask[i],
    cpu.map_base + reg_mask_set(intc, i));
    l1_writel(intc.irq_fwd_mask[i],
    cpu.map_base + reg_mask_clr(intc, i));
    cpu.mask_cache[i] = ~intc.irq_fwd_mask[i];
    }
    parent_irq = irq_of_parse_and_map(dn, idx);
    if (!parent_irq) {
    pr_err("failed to map parent interrupt %d\n", parent_irq);
    return -EINVAL;
    }
    if (of_property_read_bool(dn, "brcm,irq-can-wake"))
    enable_irq_wake(parent_irq);
    irq_set_chained_handler_and_data(parent_irq, bcm7038_l1_irq_handle,
    intc);
    return 0;
    }

//
// We keep a list of bcm7038_l1_chip used for suspend/resume. This hack is
// used because the struct chip_type suspend/resume hooks are not called
// unless chip_type is hooked onto a generic_chip. Since this driver does
// not use generic_chip, we need to manually hook our resume/suspend to
// syscore_ops.
//
    static LIST_HEAD(bcm7038_l1_intcs_list);
    static DEFINE_RAW_SPINLOCK(bcm7038_l1_intcs_lock);
#[no_mangle]
unsafe extern "C" fn bcm7038_l1_suspend(data: *mut c_void) -> c_int {
    static int bcm7038_l1_suspend(void *data)
    {
    struct bcm7038_l1_chip *intc;
    int boot_cpu, word;
    u32 val;
// Wakeup interrupt should only come from the boot cpu

    boot_cpu = cpu_logical_map(0);

    boot_cpu = 0;

    list_for_each_entry(intc, &bcm7038_l1_intcs_list, list) {
    for (word = 0; word < intc.n_words; word++) {
    val = intc.wake_mask[word] | intc.irq_fwd_mask[word];
    l1_writel(~val,
    intc.cpus[boot_cpu].map_base + reg_mask_set(intc, word));
    l1_writel(val,
    intc.cpus[boot_cpu].map_base + reg_mask_clr(intc, word));
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm7038_l1_resume(data: *mut c_void) {
    static void bcm7038_l1_resume(void *data)
    {
    struct bcm7038_l1_chip *intc;
    int boot_cpu, word;

    boot_cpu = cpu_logical_map(0);

    boot_cpu = 0;

    list_for_each_entry(intc, &bcm7038_l1_intcs_list, list) {
    for (word = 0; word < intc.n_words; word++) {
    l1_writel(intc.cpus[boot_cpu].mask_cache[word],
    intc.cpus[boot_cpu].map_base + reg_mask_set(intc, word));
    l1_writel(~intc.cpus[boot_cpu].mask_cache[word],
    intc.cpus[boot_cpu].map_base + reg_mask_clr(intc, word));
    }
    }
    }
    static const struct syscore_ops bcm7038_l1_syscore_ops = {
    .suspend	= bcm7038_l1_suspend,
    .resume		= bcm7038_l1_resume,
    };
    static struct syscore bcm7038_l1_syscore = {
    .ops = &bcm7038_l1_syscore_ops,
    };
#[no_mangle]
unsafe extern "C" fn bcm7038_l1_set_wake(d: *mut irq_data, on: c_uint) -> c_int {
    static int bcm7038_l1_set_wake(struct irq_data *d, unsigned int on)
    {
    struct bcm7038_l1_chip *intc = irq_data_get_irq_chip_data(d);
    unsigned long flags;
    let mut word: u32 = d.hwirq / IRQS_PER_WORD;
    let mut mask: u32 = BIT(d.hwirq % IRQS_PER_WORD);
    raw_spin_lock_irqsave(&intc.lock, flags);
    if (on)
    intc.wake_mask[word] |= mask;
    else
    intc.wake_mask[word] &= ~mask;
    raw_spin_unlock_irqrestore(&intc.lock, flags);
    return 0;
    }

    static struct irq_chip bcm7038_l1_irq_chip = {
    .name			= "bcm7038-l1",
    .irq_mask		= bcm7038_l1_mask,
    .irq_unmask		= bcm7038_l1_unmask,

    .irq_set_affinity	= bcm7038_l1_set_affinity,

    .irq_set_wake		= bcm7038_l1_set_wake,

    };
    static int bcm7038_l1_map(struct irq_domain *d, unsigned int virq,
    irq_hw_number_t hw_irq)
    {
    struct bcm7038_l1_chip *intc = d.host_data;
    let mut mask: u32 = BIT(hw_irq % IRQS_PER_WORD);
    let mut word: u32 = hw_irq / IRQS_PER_WORD;
    if (intc.irq_fwd_mask[word] & mask)
    return -EPERM;
    irq_set_chip_and_handler(virq, &bcm7038_l1_irq_chip, handle_level_irq);
    irq_set_chip_data(virq, d.host_data);
    irqd_set_single_target(irq_get_irq_data(virq));
    return 0;
    }
    static const struct irq_domain_ops bcm7038_l1_domain_ops = {
    .xlate			= irq_domain_xlate_onecell,
    .map			= bcm7038_l1_map,
    };
#[no_mangle]
unsafe extern "C" fn bcm7038_l1_probe(pdev: *mut platform_device, parent: *mut device_node) -> c_int {
    static int bcm7038_l1_probe(struct platform_device *pdev, struct device_node *parent)
    {
    struct device_node *dn = pdev.dev.of_node;
    struct bcm7038_l1_chip *intc;
    int idx, ret;
    intc = kzalloc_obj(*intc);
    if (!intc)
    return -ENOMEM;
    raw_spin_lock_init(&intc.lock);
    for_each_possible_cpu(idx) {
    ret = bcm7038_l1_init_one(dn, idx, intc);
    if (ret < 0) {
    if (idx)
    break;
    pr_err("failed to remap intc L1 registers\n");
    goto out_free;
    }
    }
    intc.domain = irq_domain_create_linear(of_fwnode_handle(dn), IRQS_PER_WORD * intc.n_words,
    &bcm7038_l1_domain_ops,
    intc);
    if (!intc.domain) {
    ret = -ENOMEM;
    goto out_unmap;
    }

// Add bcm7038_l1_chip into a list
    raw_spin_lock(&bcm7038_l1_intcs_lock);
    list_add_tail(&intc.list, &bcm7038_l1_intcs_list);
    raw_spin_unlock(&bcm7038_l1_intcs_lock);
    if (list_is_singular(&bcm7038_l1_intcs_list))
    register_syscore(&bcm7038_l1_syscore);

    pr_info("registered BCM7038 L1 intc (%pOF, IRQs: %d)\n",
    dn, IRQS_PER_WORD * intc.n_words);
    return 0;
    out_unmap:
    for_each_possible_cpu(idx) {
    struct bcm7038_l1_cpu *cpu = intc.cpus[idx];
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
    IRQCHIP_PLATFORM_DRIVER_BEGIN(bcm7038_l1)
    IRQCHIP_MATCH("brcm,bcm7038-l1-intc", bcm7038_l1_probe)
    IRQCHIP_PLATFORM_DRIVER_END(bcm7038_l1)
    MODULE_DESCRIPTION("Broadcom STB 7038-style L1/L2 interrupt controller");
    MODULE_LICENSE("GPL v2");
