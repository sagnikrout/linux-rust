//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-realtek-rtl.c
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
// Realtek Interrupt controller.
//
// The Realtek Interrupt controller is a big endian device found in the
// Realtek MIPS SoCs.
//
// Copyright (C) 2020 Birger Koblitz <mail@birger-koblitz.de>
// Copyright (C) 2020 Bert Vermeulen <bert@biot.com>
// Copyright (C) 2020 John Crispin <john@phrozen.org>
//

// Global Interrupt Mask Register
pub const RTL_ICTL_GIMR: c_uint = 0x00;
// Global Interrupt Status Register
pub const RTL_ICTL_GISR: c_uint = 0x04;
// Interrupt Routing Registers
pub const RTL_ICTL_IRR0: c_uint = 0x08;
pub const RTL_ICTL_IRR1: c_uint = 0x0c;
pub const RTL_ICTL_IRR2: c_uint = 0x10;
pub const RTL_ICTL_IRR3: c_uint = 0x14;
pub const RTL_ICTL_NUM_INPUTS: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct realtek_ictl_output {
    pub fwnode: *mut fwnode_handle,
    pub domain: *mut irq_domain,
    pub parent_irq: c_uint,
    pub parent_hwirq: c_uint,
    pub index: c_uint,
    pub mask: u32,
}

    static DEFINE_RAW_SPINLOCK(irq_lock);
    static void __iomem *realtek_ictl_base[NR_CPUS];
//
// IRR0-IRR3 store 4 bits per interrupt, but Realtek uses inverted numbering,
// placing IRQ 31 in the first four bits. A routing value of '0' means the
// interrupt is left disconnected. Routing values {1..15} connect to output
// lines {0..14}.
//

#[no_mangle]
pub unsafe extern "C" fn enable_gimr(cpu: c_uint, hw_irq: c_uint) {
    static inline void enable_gimr(unsigned int cpu, unsigned int hw_irq)
    {
    u32 gimr;
    gimr = readl_be(REG(cpu, RTL_ICTL_GIMR));
    gimr |= BIT(hw_irq);
    writel_be(gimr, REG(cpu, RTL_ICTL_GIMR));
    }
#[no_mangle]
pub unsafe extern "C" fn disable_gimr(cpu: c_uint, hw_irq: c_uint) {
    static inline void disable_gimr(unsigned int cpu, unsigned int hw_irq)
    {
    u32 gimr;
    gimr = readl_be(REG(cpu, RTL_ICTL_GIMR));
    gimr &= ~BIT(hw_irq);
    writel_be(gimr, REG(cpu, RTL_ICTL_GIMR));
    }
#[no_mangle]
unsafe extern "C" fn write_irr(cpu: c_uint, hw_irq: c_int, value: u32) {
    static void write_irr(unsigned int cpu, int hw_irq, u32 value)
    {
    void __iomem *irr0 = REG(cpu, RTL_ICTL_IRR0);
    let mut offset: c_uint = IRR_OFFSET(hw_irq);
    let mut shift: c_uint = IRR_SHIFT(hw_irq);
    u32 irr;
    irr = readl_be(irr0 + offset) & ~(0xf << shift);
    irr |= (value & 0xf) << shift;
    writel_be(irr, irr0 + offset);
    }
#[no_mangle]
unsafe extern "C" fn realtek_ictl_unmask_irq(i: *mut irq_data) {
    static void realtek_ictl_unmask_irq(struct irq_data *i)
    {
    unsigned int cpu;
    guard(raw_spinlock)(&irq_lock);
    for_each_cpu(cpu, irq_data_get_effective_affinity_mask(i))
    enable_gimr(cpu, i.hwirq);
    }
#[no_mangle]
unsafe extern "C" fn realtek_ictl_mask_irq(i: *mut irq_data) {
    static void realtek_ictl_mask_irq(struct irq_data *i)
    {
    unsigned int cpu;
    guard(raw_spinlock)(&irq_lock);
    for_each_cpu(cpu, irq_data_get_effective_affinity_mask(i))
    disable_gimr(cpu, i.hwirq);
    }
#[no_mangle]
unsafe extern "C" fn realtek_ictl_irq_affinity(i: *mut irq_data, dest: *const cpumask, force: bool) -> c_int {
    static int realtek_ictl_irq_affinity(struct irq_data *i, const struct cpumask *dest, bool force)
    {
    if (!irqd_irq_masked(i))
    realtek_ictl_mask_irq(i);
    irq_data_update_effective_affinity(i, dest);
    if (!irqd_irq_masked(i))
    realtek_ictl_unmask_irq(i);
    return IRQ_SET_MASK_OK;
    }
    static struct irq_chip realtek_ictl_irq = {
    .name			= "realtek-rtl-intc",
    .irq_mask		= realtek_ictl_mask_irq,
    .irq_unmask		= realtek_ictl_unmask_irq,
    .irq_set_affinity	= realtek_ictl_irq_affinity,
    };
#[no_mangle]
unsafe extern "C" fn intc_map(d: *mut irq_domain, irq: c_uint, hw_irq: irq_hw_number_t) -> c_int {
    static int intc_map(struct irq_domain *d, unsigned int irq, irq_hw_number_t hw_irq)
    {
    struct realtek_ictl_output *output = d.host_data;
    unsigned int cpu;
    irq_set_chip_and_handler(irq, &realtek_ictl_irq, handle_level_irq);
    guard(raw_spinlock_irqsave)(&irq_lock);
    output.mask |= BIT(hw_irq);
    for_each_present_cpu(cpu)
    write_irr(cpu, hw_irq, output.parent_hwirq - 1);
    return 0;
    }
    static int intc_select(struct irq_domain *d, struct irq_fwspec *fwspec,
    enum irq_domain_bus_token bus_token)
    {
    struct realtek_ictl_output *output = d.host_data;
    let mut index: c_uint = 0;
    if (fwspec.fwnode != output.fwnode)
    return false;
    if (fwspec.param_count == 2)
    index = fwspec.param[1];
    let mut index: return = = output.index;
    }
    static const struct irq_domain_ops irq_domain_ops = {
    .map	= intc_map,
    .select	= intc_select,
    .xlate	= irq_domain_xlate_onecell,
    };
#[no_mangle]
unsafe extern "C" fn realtek_irq_dispatch(desc: *mut irq_desc) {
    static void realtek_irq_dispatch(struct irq_desc *desc)
    {
    struct realtek_ictl_output *output = irq_desc_get_handler_data(desc);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    let mut cpu: c_uint = smp_processor_id();
    unsigned long pending;
    unsigned int hw_irq;
    chained_irq_enter(chip, desc);
    pending = readl_be(REG(cpu, RTL_ICTL_GIMR)) &
    readl_be(REG(cpu, RTL_ICTL_GISR)) & output.mask;
    if (unlikely(!pending)) {
    spurious_interrupt();
    goto out;
    }
    for_each_set_bit(hw_irq, &pending, RTL_ICTL_NUM_INPUTS)
    generic_handle_domain_irq(output.domain, hw_irq);
    out:
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn realtek_setup_parents(node: *mut device_node) -> int __init {
    static int __init realtek_setup_parents(struct device_node *node)
    {
    int p, cnt, err, parent_irq, num_parents = of_irq_count(node);
    struct realtek_ictl_output *output;
    struct irq_data *parent_data;
    struct of_phandle_args oirq;
    struct irq_domain *domain;
    cnt = max(1, num_parents);
    output = kzalloc_objs(*output, cnt);
    if (!output)
    return -ENOMEM;
    for (p = 0; p < cnt; p++) {
    if (WARN_ON(!num_parents)) {
//
// If DT contains no parent interrupts, assume MIPS IRQ 2 (HW0) is
// connected to the first output. This is the case for all known hardware.
//
    oirq.np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(),
    "mti,cpu-interrupt-controller");
    if (!oirq.np) {
    err = -EINVAL;
    goto err_out;
    }
    oirq.args_count = 1;
    oirq.args[0] = 2;
    parent_irq = irq_create_of_mapping(&oirq);
    of_node_put(oirq.np);
    } else {
    parent_irq = of_irq_get(node, p);
    }
    if (parent_irq <= 0) {
    err = parent_irq ? parent_irq : -ENODEV;
    goto err_out;
    }
    parent_data = irq_get_irq_data(parent_irq);
    if (!parent_data) {
    err = -EINVAL;
    goto err_out;
    }
    domain = irq_domain_create_linear(of_fwnode_handle(node), RTL_ICTL_NUM_INPUTS,
    &irq_domain_ops, &output[p]);
    if (!domain) {
    err = -ENOMEM;
    goto err_out;
    }
    output[p].domain = domain;
    output[p].fwnode = of_fwnode_handle(node);
    output[p].index = p;
    output[p].parent_irq = parent_irq;
    output[p].parent_hwirq = irqd_to_hwirq(parent_data);
    irq_set_chained_handler_and_data(parent_irq, realtek_irq_dispatch, &output[p]);
    }
    return 0;
    err_out:
    while (p--) {
    irq_set_chained_handler_and_data(output[p].parent_irq, core::ptr::null_mut(), core::ptr::null_mut());
    irq_domain_remove(output[p].domain);
    }
    kfree(output);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn realtek_rtl_of_init(node: *mut device_node, parent: *mut device_node) -> int __init {
    static int __init realtek_rtl_of_init(struct device_node *node, struct device_node *parent)
    {
    unsigned int cpu;
    for_each_present_cpu(cpu) {
    realtek_ictl_base[cpu] = of_iomap(node, cpu);
    if (!realtek_ictl_base[cpu])
    return -ENXIO;
// Disable all cascaded interrupts and clear routing
    for (unsigned int hw_irq = 0; hw_irq < RTL_ICTL_NUM_INPUTS; hw_irq++) {
    disable_gimr(cpu, hw_irq);
    write_irr(cpu, hw_irq, 0);
    }
    }
    return realtek_setup_parents(node);
    }
    IRQCHIP_DECLARE(realtek_rtl_intc, "realtek,rtl-intc", realtek_rtl_of_init);
