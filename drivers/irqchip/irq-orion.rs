//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-orion.c
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
// Marvell Orion SoCs IRQ chip driver.
//
// Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2.  This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

//
// Orion SoC main interrupt controller
//
pub const ORION_IRQS_PER_CHIP: c_int = 32;
pub const ORION_IRQ_CAUSE: c_uint = 0x00;
pub const ORION_IRQ_MASK: c_uint = 0x04;
pub const ORION_IRQ_FIQ_MASK: c_uint = 0x08;
pub const ORION_IRQ_ENDP_MASK: c_uint = 0x0c;
    static struct irq_domain *orion_irq_domain;
    static void
#[no_mangle]
pub unsafe extern "C" fn orion_handle_irq(regs: *mut pt_regs) -> __exception_irq_entry {
    __exception_irq_entry orion_handle_irq(struct pt_regs *regs)
    {
    struct irq_domain_chip_generic *dgc = orion_irq_domain.gc;
    int n, base = 0;
    for (n = 0; n < dgc.num_chips; n++, base += ORION_IRQS_PER_CHIP) {
    struct irq_chip_generic *gc =
    irq_get_domain_generic_chip(orion_irq_domain, base);
    u32 stat = readl_relaxed(gc.reg_base + ORION_IRQ_CAUSE) &
    gc.mask_cache;
    while (stat) {
    let mut hwirq: u32 = __fls(stat);
    generic_handle_domain_irq(orion_irq_domain,
    gc.irq_base + hwirq);
    stat &= ~(1 << hwirq);
    }
    }
    }
    static int __init orion_irq_init(struct device_node *np,
    struct device_node *parent)
    {
    let mut clr: c_uint = IRQ_NOREQUEST | IRQ_NOPROBE | IRQ_NOAUTOEN;
    int n, ret, base, num_chips = 0;
    struct resource r;
// count number of irq chips by valid reg addresses
    num_chips = of_address_count(np);
    orion_irq_domain = irq_domain_create_linear(of_fwnode_handle(np),
    num_chips * ORION_IRQS_PER_CHIP,
    &irq_generic_chip_ops, core::ptr::null_mut());
    if (!orion_irq_domain)
    panic("%pOFn: unable to add irq domain\n", np);
    ret = irq_alloc_domain_generic_chips(orion_irq_domain,
    ORION_IRQS_PER_CHIP, 1, np.full_name,
    handle_level_irq, clr, 0,
    IRQ_GC_INIT_MASK_CACHE);
    if (ret)
    panic("%pOFn: unable to alloc irq domain gc\n", np);
    for (n = 0, base = 0; n < num_chips; n++, base += ORION_IRQS_PER_CHIP) {
    struct irq_chip_generic *gc =
    irq_get_domain_generic_chip(orion_irq_domain, base);
    of_address_to_resource(np, n, &r);
    if (!request_mem_region(r.start, resource_size(&r), np.name))
    panic("%pOFn: unable to request mem region %d",
    np, n);
    gc.reg_base = ioremap(r.start, resource_size(&r));
    if (!gc.reg_base)
    panic("%pOFn: unable to map resource %d", np, n);
    gc.chip_types[0].regs.mask = ORION_IRQ_MASK;
    gc.chip_types[0].chip.irq_mask = irq_gc_mask_clr_bit;
    gc.chip_types[0].chip.irq_unmask = irq_gc_mask_set_bit;
// mask all interrupts
    writel(0, gc.reg_base + ORION_IRQ_MASK);
    }
    set_handle_irq(orion_handle_irq);
    return 0;
    }
    IRQCHIP_DECLARE(orion_intc, "marvell,orion-intc", orion_irq_init);
//
// Orion SoC bridge interrupt controller
//
pub const ORION_BRIDGE_IRQ_CAUSE: c_uint = 0x00;
pub const ORION_BRIDGE_IRQ_MASK: c_uint = 0x04;
#[no_mangle]
unsafe extern "C" fn orion_bridge_irq_handler(desc: *mut irq_desc) {
    static void orion_bridge_irq_handler(struct irq_desc *desc)
    {
    struct irq_domain *d = irq_desc_get_handler_data(desc);
    struct irq_chip_generic *gc = irq_get_domain_generic_chip(d, 0);
    u32 stat = readl_relaxed(gc.reg_base + ORION_BRIDGE_IRQ_CAUSE) &
    gc.mask_cache;
    while (stat) {
    let mut hwirq: u32 = __fls(stat);
    generic_handle_domain_irq(d, gc.irq_base + hwirq);
    stat &= ~(1 << hwirq);
    }
    }
//
// Bridge IRQ_CAUSE is asserted regardless of IRQ_MASK register.
// To avoid interrupt events on stale irqs, we clear them before unmask.
//
#[no_mangle]
unsafe extern "C" fn orion_bridge_irq_startup(d: *mut irq_data) -> c_uint {
    static unsigned int orion_bridge_irq_startup(struct irq_data *d)
    {
    struct irq_chip_type *ct = irq_data_get_chip_type(d);
    ct.chip.irq_ack(d);
    ct.chip.irq_unmask(d);
    return 0;
    }
    static int __init orion_bridge_irq_init(struct device_node *np,
    struct device_node *parent)
    {
    let mut clr: c_uint = IRQ_NOREQUEST | IRQ_NOPROBE | IRQ_NOAUTOEN;
    struct resource r;
    struct irq_domain *domain;
    struct irq_chip_generic *gc;
    int ret, irq, nrirqs = 32;
// get optional number of interrupts provided
    of_property_read_u32(np, "marvell,#interrupts", &nrirqs);
    domain = irq_domain_create_linear(of_fwnode_handle(np), nrirqs,
    &irq_generic_chip_ops, core::ptr::null_mut());
    if (!domain) {
    pr_err("%pOFn: unable to add irq domain\n", np);
    return -ENOMEM;
    }
    ret = irq_alloc_domain_generic_chips(domain, nrirqs, 1, np.name,
    handle_edge_irq, clr, 0, IRQ_GC_INIT_MASK_CACHE);
    if (ret) {
    pr_err("%pOFn: unable to alloc irq domain gc\n", np);
    return ret;
    }
    ret = of_address_to_resource(np, 0, &r);
    if (ret) {
    pr_err("%pOFn: unable to get resource\n", np);
    return ret;
    }
    if (!request_mem_region(r.start, resource_size(&r), np.name)) {
    pr_err("%s: unable to request mem region\n", np.name);
    return -ENOMEM;
    }
// Map the parent interrupt for the chained handler
    irq = irq_of_parse_and_map(np, 0);
    if (irq <= 0) {
    pr_err("%pOFn: unable to parse irq\n", np);
    return -EINVAL;
    }
    gc = irq_get_domain_generic_chip(domain, 0);
    gc.reg_base = ioremap(r.start, resource_size(&r));
    if (!gc.reg_base) {
    pr_err("%pOFn: unable to map resource\n", np);
    return -ENOMEM;
    }
    gc.chip_types[0].regs.ack = ORION_BRIDGE_IRQ_CAUSE;
    gc.chip_types[0].regs.mask = ORION_BRIDGE_IRQ_MASK;
    gc.chip_types[0].chip.irq_startup = orion_bridge_irq_startup;
    gc.chip_types[0].chip.irq_ack = irq_gc_ack_clr_bit;
    gc.chip_types[0].chip.irq_mask = irq_gc_mask_clr_bit;
    gc.chip_types[0].chip.irq_unmask = irq_gc_mask_set_bit;
// mask and clear all interrupts
    writel(0, gc.reg_base + ORION_BRIDGE_IRQ_MASK);
    writel(0, gc.reg_base + ORION_BRIDGE_IRQ_CAUSE);
    irq_set_chained_handler_and_data(irq, orion_bridge_irq_handler,
    domain);
    return 0;
    }
    IRQCHIP_DECLARE(orion_bridge_intc,
    "marvell,orion-bridge-intc", orion_bridge_irq_init);
