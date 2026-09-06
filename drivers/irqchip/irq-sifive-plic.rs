//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-sifive-plic.c
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
// Copyright (C) 2017 SiFive
// Copyright (C) 2018 Christoph Hellwig
//

//
// This driver implements a version of the RISC-V PLIC with the actual layout
// specified in chapter 8 of the SiFive U5 Coreplex Series Manual:
//
// https://static.dev.sifive.com/U54-MC-RVCoreIP.pdf
//
// The largest number supported by devices marked as 'sifive,plic-1.0.0', is
// 1024, of which device 0 is defined as non-existent by the RISC-V Privileged
// Spec.
//
pub const MAX_DEVICES: c_int = 1024;
pub const MAX_CONTEXTS: c_int = 15872;
//
// Each interrupt source has a priority register associated with it.
// We always hardwire it to one in Linux.
//
pub const PRIORITY_BASE: c_int = 0;
pub const PRIORITY_PER_ID: c_int = 4;
//
// Each hart context has a vector of interrupt enable bits associated with it.
// There's one bit for each interrupt source.
//
pub const CONTEXT_ENABLE_BASE: c_uint = 0x2000;
pub const CONTEXT_ENABLE_SIZE: c_uint = 0x80;
pub const PENDING_BASE: c_uint = 0x1000;
//
// Each hart context has a set of control registers associated with it.  Right
// now there's only two: a source priority threshold over which the hart will
// take an interrupt, and a register to claim interrupts.
//
pub const CONTEXT_BASE: c_uint = 0x200000;
pub const CONTEXT_SIZE: c_uint = 0x1000;
pub const CONTEXT_THRESHOLD: c_uint = 0x00;
pub const CONTEXT_CLAIM: c_uint = 0x04;
pub const PLIC_DISABLE_THRESHOLD: c_uint = 0x7;
pub const PLIC_ENABLE_THRESHOLD: c_int = 0;
pub const PLIC_QUIRK_EDGE_INTERRUPT: c_int = 0;
pub const PLIC_QUIRK_CP100_CLAIM_REGISTER_ERRATUM: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct plic_priv {
    pub fwnode: *mut fwnode_handle,
    pub lmask: cpumask,
    pub irqdomain: *mut irq_domain,
    pub regs: *mut void __iomem,
    pub plic_quirks: c_ulong,
// device interrupts + 1 to compensate for the reserved hwirq 0
    pub total_irqs: unsigned int __private,
    pub irq_groups: c_uint,
    pub prio_save: *mut c_ulong,
    pub gsi_base: u32,
    pub acpi_plic_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plic_handler {
    pub present: bool,
    pub hart_base: *mut void __iomem,
//
// Protect mask operations on the registers given that we can't
// assume atomic memory operations work on them.
//
    pub enable_lock: raw_spinlock_t,
    pub enable_base: *mut void __iomem,
    pub enable_save: *mut u32,
    pub priv: *mut plic_priv,
}

//
// Macro to deal with the insanity of hardware interrupt 0 being reserved

    for (unsigned int iter = 1; iter < ACCESS_PRIVATE(priv, total_irqs); iter++)
    static int plic_parent_irq __ro_after_init;
    static bool plic_global_setup_done __ro_after_init;
    static DEFINE_PER_CPU(struct plic_handler, plic_handlers);
    static int plic_irq_set_type(struct irq_data *d, unsigned int type);
#[no_mangle]
unsafe extern "C" fn __plic_toggle(handler: *mut plic_handler, hwirq: c_int, enable: c_int) {
    static void __plic_toggle(struct plic_handler *handler, int hwirq, int enable)
    {
    u32 __iomem *base = handler.enable_base;
    let mut hwirq_mask: u32 = 1 << (hwirq % 32);
    let mut group: c_int = hwirq / 32;
    u32 value;
    value = readl(base + group);
    if (enable)
    value |= hwirq_mask;
    else
    value &= ~hwirq_mask;
    handler.enable_save[group] = value;
    writel(value, base + group);
    }
#[no_mangle]
unsafe extern "C" fn plic_toggle(handler: *mut plic_handler, hwirq: c_int, enable: c_int) {
    static void plic_toggle(struct plic_handler *handler, int hwirq, int enable)
    {
    unsigned long flags;
    raw_spin_lock_irqsave(&handler.enable_lock, flags);
    __plic_toggle(handler, hwirq, enable);
    raw_spin_unlock_irqrestore(&handler.enable_lock, flags);
    }
    static inline void plic_irq_toggle(const struct cpumask *mask,
    struct irq_data *d, int enable)
    {
    int cpu;
    for_each_cpu(cpu, mask) {
    struct plic_handler *handler = per_cpu_ptr(&plic_handlers, cpu);
    plic_toggle(handler, d.hwirq, enable);
    }
    }
#[no_mangle]
unsafe extern "C" fn plic_irq_unmask(d: *mut irq_data) {
    static void plic_irq_unmask(struct irq_data *d)
    {
    struct plic_priv *priv = irq_data_get_irq_chip_data(d);
    writel(1, priv.regs + PRIORITY_BASE + d.hwirq * PRIORITY_PER_ID);
    }
#[no_mangle]
unsafe extern "C" fn plic_irq_mask(d: *mut irq_data) {
    static void plic_irq_mask(struct irq_data *d)
    {
    struct plic_priv *priv = irq_data_get_irq_chip_data(d);
    writel(0, priv.regs + PRIORITY_BASE + d.hwirq * PRIORITY_PER_ID);
    }
#[no_mangle]
unsafe extern "C" fn plic_irq_enable(d: *mut irq_data) {
    static void plic_irq_enable(struct irq_data *d)
    {
    plic_irq_toggle(irq_data_get_effective_affinity_mask(d), d, 1);
    plic_irq_unmask(d);
    }
#[no_mangle]
unsafe extern "C" fn plic_irq_disable(d: *mut irq_data) {
    static void plic_irq_disable(struct irq_data *d)
    {
    plic_irq_toggle(irq_data_get_effective_affinity_mask(d), d, 0);
    }
#[no_mangle]
unsafe extern "C" fn plic_irq_eoi(d: *mut irq_data) {
    static void plic_irq_eoi(struct irq_data *d)
    {
    struct plic_handler *handler = this_cpu_ptr(&plic_handlers);
    u32 __iomem *reg;
    bool enabled;
    reg = handler.enable_base + (d.hwirq / 32) * sizeof(u32);
    enabled = readl(reg) & BIT(d.hwirq % 32);
    if (unlikely(!enabled)) {
    plic_toggle(handler, d.hwirq, 1);
    writel(d.hwirq, handler.hart_base + CONTEXT_CLAIM);
    plic_toggle(handler, d.hwirq, 0);
    } else {
    writel(d.hwirq, handler.hart_base + CONTEXT_CLAIM);
    }
    }

    static int plic_set_affinity(struct irq_data *d,
    const struct cpumask *mask_val, bool force)
    {
    unsigned int cpu;
    struct plic_priv *priv = irq_data_get_irq_chip_data(d);
    if (force)
    cpu = cpumask_first_and(&priv.lmask, mask_val);
    else
    cpu = cpumask_first_and_and(&priv.lmask, mask_val, cpu_online_mask);
    if (cpu >= nr_cpu_ids)
    return -EINVAL;
// Invalidate the original routing entry
    plic_irq_toggle(irq_data_get_effective_affinity_mask(d), d, 0);
    irq_data_update_effective_affinity(d, cpumask_of(cpu));
// Setting the new routing entry if irq is enabled
    if (!irqd_irq_disabled(d))
    plic_irq_toggle(irq_data_get_effective_affinity_mask(d), d, 1);
    return IRQ_SET_MASK_OK_DONE;
    }

    static struct irq_chip plic_edge_chip = {
    .name		= "SiFive PLIC",
    .irq_enable	= plic_irq_enable,
    .irq_disable	= plic_irq_disable,
    .irq_ack	= plic_irq_eoi,
    .irq_mask	= plic_irq_mask,
    .irq_unmask	= plic_irq_unmask,

    .irq_set_affinity = plic_set_affinity,

    .irq_set_type	= plic_irq_set_type,
    .flags		= IRQCHIP_SKIP_SET_WAKE |
    IRQCHIP_AFFINITY_PRE_STARTUP,
    };
    static struct irq_chip plic_chip = {
    .name		= "SiFive PLIC",
    .irq_enable	= plic_irq_enable,
    .irq_disable	= plic_irq_disable,
    .irq_mask	= plic_irq_mask,
    .irq_unmask	= plic_irq_unmask,
    .irq_eoi	= plic_irq_eoi,

    .irq_set_affinity = plic_set_affinity,

    .irq_set_type	= plic_irq_set_type,
    .flags		= IRQCHIP_SKIP_SET_WAKE |
    IRQCHIP_AFFINITY_PRE_STARTUP,
    };
#[no_mangle]
unsafe extern "C" fn plic_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int plic_irq_set_type(struct irq_data *d, unsigned int type)
    {
    struct plic_priv *priv = irq_data_get_irq_chip_data(d);
    if (!test_bit(PLIC_QUIRK_EDGE_INTERRUPT, &priv.plic_quirks))
    return IRQ_SET_MASK_OK_NOCOPY;
    switch (type) {
    case IRQ_TYPE_EDGE_RISING:
    irq_set_chip_handler_name_locked(d, &plic_edge_chip,
    handle_edge_irq, core::ptr::null_mut());
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    irq_set_chip_handler_name_locked(d, &plic_chip,
    handle_fasteoi_irq, core::ptr::null_mut());
    break;
    default:
    return -EINVAL;
    }
    return IRQ_SET_MASK_OK;
    }
#[no_mangle]
unsafe extern "C" fn plic_irq_suspend(data: *mut c_void) -> c_int {
    static int plic_irq_suspend(void *data)
    {
    struct plic_priv *priv = this_cpu_ptr(&plic_handlers).priv;
    for_each_device_irq(irq, priv) {
    __assign_bit(irq, priv.prio_save,
    readl(priv.regs + PRIORITY_BASE + irq * PRIORITY_PER_ID));
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn plic_irq_resume(data: *mut c_void) {
    static void plic_irq_resume(void *data)
    {
    struct plic_priv *priv = this_cpu_ptr(&plic_handlers).priv;
    unsigned int index, cpu;
    unsigned long flags;
    u32 __iomem *reg;
    for_each_device_irq(irq, priv) {
    index = BIT_WORD(irq);
    writel((priv.prio_save[index] & BIT_MASK(irq)) ? 1 : 0,
    priv.regs + PRIORITY_BASE + irq * PRIORITY_PER_ID);
    }
    for_each_present_cpu(cpu) {
    struct plic_handler *handler = per_cpu_ptr(&plic_handlers, cpu);
    if (!handler.present)
    continue;
    raw_spin_lock_irqsave(&handler.enable_lock, flags);
    for (unsigned int i = 0; i < priv.irq_groups; i++) {
    reg = handler.enable_base + i * sizeof(u32);
    writel(handler.enable_save[i], reg);
    }
    raw_spin_unlock_irqrestore(&handler.enable_lock, flags);
    }
    }
    static const struct syscore_ops plic_irq_syscore_ops = {
    .suspend	= plic_irq_suspend,
    .resume		= plic_irq_resume,
    };
    static struct syscore plic_irq_syscore = {
    .ops = &plic_irq_syscore_ops,
    };
    static int plic_irqdomain_map(struct irq_domain *d, unsigned int irq,
    irq_hw_number_t hwirq)
    {
    struct plic_priv *priv = d.host_data;
    irq_domain_set_info(d, irq, hwirq, &plic_chip, d.host_data,
    handle_fasteoi_irq, core::ptr::null_mut(), core::ptr::null_mut());
    irq_set_noprobe(irq);
    irq_set_affinity(irq, &priv.lmask);
    return 0;
    }
    static int plic_irq_domain_translate(struct irq_domain *d,
    struct irq_fwspec *fwspec,
    unsigned long *hwirq,
    unsigned int *type)
    {
    struct plic_priv *priv = d.host_data;
// For DT, gsi_base is always zero.
    if (fwspec.param[0] >= priv.gsi_base)
    fwspec.param[0] = fwspec.param[0] - priv.gsi_base;
    if (test_bit(PLIC_QUIRK_EDGE_INTERRUPT, &priv.plic_quirks))
    return irq_domain_translate_twocell(d, fwspec, hwirq, type);
    return irq_domain_translate_onecell(d, fwspec, hwirq, type);
    }
    static int plic_irq_domain_alloc(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs, void *arg)
    {
    int i, ret;
    irq_hw_number_t hwirq;
    unsigned int type;
    struct irq_fwspec *fwspec = arg;
    ret = plic_irq_domain_translate(domain, fwspec, &hwirq, &type);
    if (ret)
    return ret;
    for (i = 0; i < nr_irqs; i++) {
    ret = plic_irqdomain_map(domain, virq + i, hwirq + i);
    if (ret)
    return ret;
    }
    return 0;
    }
    static const struct irq_domain_ops plic_irqdomain_ops = {
    .translate	= plic_irq_domain_translate,
    .alloc		= plic_irq_domain_alloc,
    .free		= irq_domain_free_irqs_top,
    };
//
// Handling an interrupt is a two-step process: first you claim the interrupt
// by reading the claim register, then you complete the interrupt by writing
// that source ID back to the same claim register.  This automatically enables
// and disables the interrupt, so there's nothing else to do.
//
#[no_mangle]
unsafe extern "C" fn plic_handle_irq(desc: *mut irq_desc) {
    static void plic_handle_irq(struct irq_desc *desc)
    {
    struct plic_handler *handler = this_cpu_ptr(&plic_handlers);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    void __iomem *claim = handler.hart_base + CONTEXT_CLAIM;
    irq_hw_number_t hwirq;
    WARN_ON_ONCE(!handler.present);
    chained_irq_enter(chip, desc);
    while ((hwirq = readl(claim))) {
    int err = generic_handle_domain_irq(handler.priv.irqdomain,
    hwirq);
    if (unlikely(err)) {
    pr_warn_ratelimited("%pfwP: can't find mapping for hwirq %lu\n",
    handler.priv.fwnode, hwirq);
    }
    }
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn cp100_isolate_pending_irq(nr_irq_groups: c_int, handler: *mut plic_handler) -> u32 {
    static u32 cp100_isolate_pending_irq(int nr_irq_groups, struct plic_handler *handler)
    {
    u32 __iomem *pending = handler.priv.regs + PENDING_BASE;
    u32 __iomem *enable = handler.enable_base;
    let mut pending_irqs: u32 = 0;
    int i, j;
// Look for first pending interrupt
    for (i = 0; i < nr_irq_groups; i++) {
// Any pending interrupts would be annihilated, so skip checking them
    if (!handler.enable_save[i])
    continue;
    pending_irqs = handler.enable_save[i] & readl_relaxed(pending + i);
    if (pending_irqs)
    break;
    }
    if (!pending_irqs)
    return 0;
// Isolate lowest set bit
    pending_irqs &= -pending_irqs;
// Disable all interrupts but the first pending one
    for (j = 0; j < nr_irq_groups; j++) {
    let mut new_mask: u32 = j == i ? pending_irqs : 0;
    if (new_mask != handler.enable_save[j])
    writel_relaxed(new_mask, enable + j);
    }
    return pending_irqs;
    }
#[no_mangle]
unsafe extern "C" fn cp100_get_hwirq(handler: *mut plic_handler, claim: *mut void __iomem) -> irq_hw_number_t {
    static irq_hw_number_t cp100_get_hwirq(struct plic_handler *handler, void __iomem *claim)
    {
    let mut nr_irq_groups: c_int = handler.priv.irq_groups;
    u32 __iomem *enable = handler.enable_base;
    let mut hwirq: irq_hw_number_t = 0;
    u32 iso_mask;
    int i;
    guard(raw_spinlock)(&handler.enable_lock);
// Existing enable state is already cached in enable_save
    iso_mask = cp100_isolate_pending_irq(nr_irq_groups, handler);
    if (!iso_mask)
    return 0;
//
// Interrupts delievered to hardware still become pending, but only
// interrupts that are both pending and enabled can be claimed.
// Clearing the enable bit for all interrupts but the first pending
// one avoids a hardware bug that occurs during read from the claim
// register with more than one eligible interrupt.
//
    hwirq = readl(claim);
// Restore previous state
    for (i = 0; i < nr_irq_groups; i++) {
    let mut written: u32 = i == hwirq / 32 ? iso_mask : 0;
    let mut stored: u32 = handler.enable_save[i];
    if (stored != written)
    writel_relaxed(stored, enable + i);
    }
    return hwirq;
    }
#[no_mangle]
unsafe extern "C" fn plic_handle_irq_cp100(desc: *mut irq_desc) {
    static void plic_handle_irq_cp100(struct irq_desc *desc)
    {
    struct plic_handler *handler = this_cpu_ptr(&plic_handlers);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    void __iomem *claim = handler.hart_base + CONTEXT_CLAIM;
    irq_hw_number_t hwirq;
    WARN_ON_ONCE(!handler.present);
    chained_irq_enter(chip, desc);
    while ((hwirq = cp100_get_hwirq(handler, claim))) {
    let mut err: c_int = generic_handle_domain_irq(handler.priv.irqdomain, hwirq);
    if (unlikely(err)) {
    pr_warn_ratelimited("%pfwP: can't find mapping for hwirq %lu\n",
    handler.priv.fwnode, hwirq);
    }
    }
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn plic_set_threshold(handler: *mut plic_handler, threshold: u32) {
    static void plic_set_threshold(struct plic_handler *handler, u32 threshold)
    {
// priority must be > threshold to trigger an interrupt
    writel(threshold, handler.hart_base + CONTEXT_THRESHOLD);
    }
#[no_mangle]
unsafe extern "C" fn plic_dying_cpu(cpu: c_uint) -> c_int {
    static int plic_dying_cpu(unsigned int cpu)
    {
    if (plic_parent_irq)
    disable_percpu_irq(plic_parent_irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn plic_starting_cpu(cpu: c_uint) -> c_int {
    static int plic_starting_cpu(unsigned int cpu)
    {
    struct plic_handler *handler = this_cpu_ptr(&plic_handlers);
    if (plic_parent_irq)
    enable_percpu_irq(plic_parent_irq,
    irq_get_trigger_type(plic_parent_irq));
    else
    pr_warn("%pfwP: cpu%d: parent irq not available\n",
    handler.priv.fwnode, cpu);
    plic_set_threshold(handler, PLIC_ENABLE_THRESHOLD);
    return 0;
    }
    static const struct of_device_id plic_match[] = {
    { .compatible = "sifive,plic-1.0.0" },
    { .compatible = "riscv,plic0" },
    { .compatible = "andestech,nceplic100",
    .data = (const void *)BIT(PLIC_QUIRK_EDGE_INTERRUPT) },
    { .compatible = "thead,c900-plic",
    .data = (const void *)BIT(PLIC_QUIRK_EDGE_INTERRUPT) },
    { .compatible = "ultrarisc,cp100-plic",
    .data = (const void *)BIT(PLIC_QUIRK_CP100_CLAIM_REGISTER_ERRATUM) },
    {}
    };

    static const struct acpi_device_id plic_acpi_match[] = {
    { "RSCV0001", 0 },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, plic_acpi_match);

    static int plic_parse_nr_irqs_and_contexts(struct fwnode_handle *fwnode,
    u32 *nr_irqs, u32 *nr_contexts,
    u32 *gsi_base, u32 *id)
    {
    int rc;
    if (!is_of_node(fwnode)) {
    rc = riscv_acpi_get_gsi_info(fwnode, gsi_base, id, nr_irqs, core::ptr::null_mut());
    if (rc) {
    pr_err("%pfwP: failed to find GSI mapping\n", fwnode);
    return rc;
    }
// nr_contexts = acpi_rintc_get_plic_nr_contexts(*id);
    if (WARN_ON(!*nr_contexts)) {
    pr_err("%pfwP: no PLIC context available\n", fwnode);
    return -EINVAL;
    }
    return 0;
    }
    rc = of_property_read_u32(to_of_node(fwnode), "riscv,ndev", nr_irqs);
    if (rc) {
    pr_err("%pfwP: riscv,ndev property not available\n", fwnode);
    return rc;
    }
// nr_contexts = of_irq_count(to_of_node(fwnode));
    if (WARN_ON(!(*nr_contexts))) {
    pr_err("%pfwP: no PLIC context available\n", fwnode);
    return -EINVAL;
    }
// gsi_base = 0;
// id = 0;
    return 0;
    }
    static int plic_parse_context_parent(struct fwnode_handle *fwnode, u32 context,
    u32 *parent_hwirq, int *parent_cpu, u32 id)
    {
    struct of_phandle_args parent;
    unsigned long hartid;
    int rc;
    if (!is_of_node(fwnode)) {
    hartid = acpi_rintc_ext_parent_to_hartid(id, context);
    if (hartid == INVALID_HARTID)
    return -EINVAL;
// parent_cpu = riscv_hartid_to_cpuid(hartid);
// parent_hwirq = RV_IRQ_EXT;
    return 0;
    }
    rc = of_irq_parse_one(to_of_node(fwnode), context, &parent);
    if (rc)
    return rc;
    rc = riscv_of_parent_hartid(parent.np, &hartid);
    if (rc)
    return rc;
// parent_hwirq = parent.args[0];
// parent_cpu = riscv_hartid_to_cpuid(hartid);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn plic_probe(fwnode: *mut fwnode_handle) -> c_int {
    static int plic_probe(struct fwnode_handle *fwnode)
    {
    let mut error: c_int = 0, nr_contexts, nr_handlers = 0, cpu, i;
    let mut plic_quirks: c_ulong = 0;
    struct plic_handler *handler;
    u32 nr_irqs, parent_hwirq;
    struct plic_priv *priv;
    void __iomem *regs;
    int id, context_id;
    u32 gsi_base;
    if (is_of_node(fwnode)) {
    const struct of_device_id *id;
    id = of_match_node(plic_match, to_of_node(fwnode));
    if (id)
    plic_quirks = (unsigned long)id.data;
    regs = of_iomap(to_of_node(fwnode), 0);
    if (!regs)
    return -ENOMEM;
    } else {
    regs = devm_platform_ioremap_resource(to_platform_device(fwnode.dev), 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    }
    error = plic_parse_nr_irqs_and_contexts(fwnode, &nr_irqs, &nr_contexts, &gsi_base, &id);
    if (error)
    goto fail_free_regs;
    priv = kzalloc_obj(*priv);
    if (!priv) {
    error = -ENOMEM;
    goto fail_free_regs;
    }
    priv.fwnode = fwnode;
    priv.plic_quirks = plic_quirks;
//
// The firmware provides the number of device interrupts. As
// hardware interrupt 0 is reserved, the number of total interrupts
// is nr_irqs + 1.
//
    nr_irqs++;
    ACCESS_PRIVATE(priv, total_irqs) = nr_irqs;
// Precalculate the number of register groups
    priv.irq_groups = DIV_ROUND_UP(nr_irqs, 32);
    priv.regs = regs;
    priv.gsi_base = gsi_base;
    priv.acpi_plic_id = id;
    priv.prio_save = bitmap_zalloc(nr_irqs, GFP_KERNEL);
    if (!priv.prio_save) {
    error = -ENOMEM;
    goto fail_free_priv;
    }
    for (i = 0; i < nr_contexts; i++) {
    error = plic_parse_context_parent(fwnode, i, &parent_hwirq, &cpu,
    priv.acpi_plic_id);
    if (error) {
    pr_warn("%pfwP: hwirq for context%d not found\n", fwnode, i);
    continue;
    }
    if (is_of_node(fwnode)) {
    context_id = i;
    } else {
    context_id = acpi_rintc_get_plic_context(priv.acpi_plic_id, i);
    if (context_id == INVALID_CONTEXT) {
    pr_warn("%pfwP: invalid context id for context%d\n", fwnode, i);
    continue;
    }
    }
//
// Skip contexts other than external interrupts for our
// privilege level.
//
    if (parent_hwirq != RV_IRQ_EXT) {
// Disable S-mode enable bits if running in M-mode.
    if (IS_ENABLED(CONFIG_RISCV_M_MODE)) {
    u32 __iomem *enable_base = priv.regs +	CONTEXT_ENABLE_BASE +
    i * CONTEXT_ENABLE_SIZE;
    for (int j = 0; j < priv.irq_groups; j++)
    writel(0, enable_base + j);
    }
    continue;
    }
    if (cpu < 0) {
    pr_warn("%pfwP: Invalid cpuid for context %d\n", fwnode, i);
    continue;
    }
//
// When running in M-mode we need to ignore the S-mode handler.
// Here we assume it always comes later, but that might be a
// little fragile.
//
    handler = per_cpu_ptr(&plic_handlers, cpu);
    if (handler.present) {
    pr_warn("%pfwP: handler already present for context %d.\n", fwnode, i);
    plic_set_threshold(handler, PLIC_DISABLE_THRESHOLD);
    goto done;
    }
    cpumask_set_cpu(cpu, &priv.lmask);
    handler.present = true;
    handler.hart_base = priv.regs + CONTEXT_BASE +
    context_id * CONTEXT_SIZE;
    raw_spin_lock_init(&handler.enable_lock);
    handler.enable_base = priv.regs + CONTEXT_ENABLE_BASE +
    context_id * CONTEXT_ENABLE_SIZE;
    handler.priv = priv;
    handler.enable_save = kcalloc(priv.irq_groups, sizeof(*handler.enable_save),
    GFP_KERNEL);
    if (!handler.enable_save) {
    error = -ENOMEM;
    goto fail_cleanup_contexts;
    }
    done:
    for_each_device_irq(hwirq, priv) {
    plic_toggle(handler, hwirq, 0);
    writel(1, priv.regs + PRIORITY_BASE + hwirq * PRIORITY_PER_ID);
    }
    nr_handlers++;
    }
    priv.irqdomain = irq_domain_create_linear(fwnode, nr_irqs, &plic_irqdomain_ops, priv);
    if (WARN_ON(!priv.irqdomain)) {
    error = -ENOMEM;
    goto fail_cleanup_contexts;
    }
//
// We can have multiple PLIC instances so setup global state
// and register syscore operations only once after context
// handlers of all online CPUs are initialized.
//
    if (!plic_global_setup_done) {
    struct irq_domain *domain;
    let mut global_setup: bool = true;
    for_each_online_cpu(cpu) {
    handler = per_cpu_ptr(&plic_handlers, cpu);
    if (!handler.present) {
    global_setup = false;
    break;
    }
    }
    if (global_setup) {
    void (*handler_fn)(struct irq_desc *) = plic_handle_irq;
    if (test_bit(PLIC_QUIRK_CP100_CLAIM_REGISTER_ERRATUM, &handler.priv.plic_quirks))
    handler_fn = plic_handle_irq_cp100;
// Find parent domain and register chained handler
    domain = irq_find_matching_fwnode(riscv_get_intc_hwnode(), DOMAIN_BUS_ANY);
    if (domain)
    plic_parent_irq = irq_create_mapping(domain, RV_IRQ_EXT);
    if (plic_parent_irq)
    irq_set_chained_handler(plic_parent_irq, handler_fn);
    cpuhp_setup_state(CPUHP_AP_IRQ_SIFIVE_PLIC_STARTING,
    "irqchip/sifive/plic:starting",
    plic_starting_cpu, plic_dying_cpu);
    register_syscore(&plic_irq_syscore);
    plic_global_setup_done = true;
    }
    }

    if (!acpi_disabled)
    acpi_dev_clear_dependencies(ACPI_COMPANION(fwnode.dev));

    pr_info("%pfwP: mapped %d interrupts with %d handlers for %d contexts.\n",
    fwnode, nr_irqs, nr_handlers, nr_contexts);
    return 0;
    fail_cleanup_contexts:
    for (i = 0; i < nr_contexts; i++) {
    if (plic_parse_context_parent(fwnode, i, &parent_hwirq, &cpu, priv.acpi_plic_id))
    continue;
    if (parent_hwirq != RV_IRQ_EXT || cpu < 0)
    continue;
    handler = per_cpu_ptr(&plic_handlers, cpu);
    handler.present = false;
    handler.hart_base = core::ptr::null_mut();
    handler.enable_base = core::ptr::null_mut();
    kfree(handler.enable_save);
    handler.enable_save = core::ptr::null_mut();
    handler.priv = core::ptr::null_mut();
    }
    bitmap_free(priv.prio_save);
    fail_free_priv:
    kfree(priv);
    fail_free_regs:
    iounmap(regs);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn plic_platform_probe(pdev: *mut platform_device) -> c_int {
    static int plic_platform_probe(struct platform_device *pdev)
    {
    return plic_probe(pdev.dev.fwnode);
    }
    static struct platform_driver plic_driver = {
    .driver = {
    .name		= "riscv-plic",
    .of_match_table	= plic_match,
    .suppress_bind_attrs = true,
    .acpi_match_table = ACPI_PTR(plic_acpi_match),
    },
    .probe = plic_platform_probe,
    };
    builtin_platform_driver(plic_driver);
    static int __init plic_early_probe(struct device_node *node,
    struct device_node *parent)
    {
    return plic_probe(&node.fwnode);
    }
    IRQCHIP_DECLARE(riscv, "allwinner,sun20i-d1-plic", plic_early_probe);
