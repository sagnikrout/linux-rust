//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-loongson-pch-pic.c
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
// Copyright (C) 2020, Jiaxun Yang <jiaxun.yang@flygoat.com>
// Loongson PCH PIC support
//

// Registers
pub const PCH_PIC_MASK: c_uint = 0x20;
pub const PCH_PIC_HTMSI_EN: c_uint = 0x40;
pub const PCH_PIC_EDGE: c_uint = 0x60;
pub const PCH_PIC_CLR: c_uint = 0x80;
pub const PCH_PIC_AUTO0: c_uint = 0xc0;
pub const PCH_PIC_AUTO1: c_uint = 0xe0;

pub const PCH_PIC_POL: c_uint = 0x3e0;
pub const PIC_COUNT_PER_REG: c_int = 32;
pub const PIC_REG_COUNT: c_int = 2;

pub const PIC_UNDEF_VECTOR: c_int = 255;
    static int nr_pics;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_pic {
    pub base: *mut void __iomem,
    pub pic_domain: *mut irq_domain,
    pub ht_vec_base: u32,
    pub pic_lock: raw_spinlock_t,
    pub vec_count: u32,
    pub gsi_base: u32,
    pub saved_vec_en: [u32; PIC_REG_COUNT],
    pub saved_vec_pol: [u32; PIC_REG_COUNT],
    pub saved_vec_edge: [u32; PIC_REG_COUNT],
    pub table: [u8; PIC_COUNT],
    pub inuse: c_int,
}

    static struct pch_pic *pch_pic_priv[MAX_IO_PICS];
    struct fwnode_handle *pch_pic_handle[MAX_IO_PICS];
#[no_mangle]
pub unsafe extern "C" fn hwirq_to_bit(priv: *mut pch_pic, hirq: c_int) -> u8 {
    static inline u8 hwirq_to_bit(struct pch_pic *priv, int hirq)
    {
    return priv.table[hirq];
    }
#[no_mangle]
unsafe extern "C" fn pch_pic_bitset(priv: *mut pch_pic, offset: c_int, bit: c_int) {
    static void pch_pic_bitset(struct pch_pic *priv, int offset, int bit)
    {
    u32 reg;
    void __iomem *addr = priv.base + offset + PIC_REG_IDX(bit) * 4;
    raw_spin_lock(&priv.pic_lock);
    reg = readl(addr);
    reg |= BIT(PIC_REG_BIT(bit));
    writel(reg, addr);
    raw_spin_unlock(&priv.pic_lock);
    }
#[no_mangle]
unsafe extern "C" fn pch_pic_bitclr(priv: *mut pch_pic, offset: c_int, bit: c_int) {
    static void pch_pic_bitclr(struct pch_pic *priv, int offset, int bit)
    {
    u32 reg;
    void __iomem *addr = priv.base + offset + PIC_REG_IDX(bit) * 4;
    raw_spin_lock(&priv.pic_lock);
    reg = readl(addr);
    reg &= ~BIT(PIC_REG_BIT(bit));
    writel(reg, addr);
    raw_spin_unlock(&priv.pic_lock);
    }
#[no_mangle]
unsafe extern "C" fn pch_pic_mask_irq(d: *mut irq_data) {
    static void pch_pic_mask_irq(struct irq_data *d)
    {
    struct pch_pic *priv = irq_data_get_irq_chip_data(d);
    pch_pic_bitset(priv, PCH_PIC_MASK, hwirq_to_bit(priv, d.hwirq));
    irq_chip_mask_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn pch_pic_unmask_irq(d: *mut irq_data) {
    static void pch_pic_unmask_irq(struct irq_data *d)
    {
    struct pch_pic *priv = irq_data_get_irq_chip_data(d);
    let mut bit: c_int = hwirq_to_bit(priv, d.hwirq);
    writel(BIT(PIC_REG_BIT(bit)),
    priv.base + PCH_PIC_CLR + PIC_REG_IDX(bit) * 4);
    irq_chip_unmask_parent(d);
    pch_pic_bitclr(priv, PCH_PIC_MASK, bit);
    }
#[no_mangle]
unsafe extern "C" fn pch_pic_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int pch_pic_set_type(struct irq_data *d, unsigned int type)
    {
    struct pch_pic *priv = irq_data_get_irq_chip_data(d);
    let mut bit: c_int = hwirq_to_bit(priv, d.hwirq);
    let mut ret: c_int = 0;
    switch (type) {
    case IRQ_TYPE_EDGE_RISING:
    pch_pic_bitset(priv, PCH_PIC_EDGE, bit);
    pch_pic_bitclr(priv, PCH_PIC_POL, bit);
    irq_set_handler_locked(d, handle_edge_irq);
    break;
    case IRQ_TYPE_EDGE_FALLING:
    pch_pic_bitset(priv, PCH_PIC_EDGE, bit);
    pch_pic_bitset(priv, PCH_PIC_POL, bit);
    irq_set_handler_locked(d, handle_edge_irq);
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    pch_pic_bitclr(priv, PCH_PIC_EDGE, bit);
    pch_pic_bitclr(priv, PCH_PIC_POL, bit);
    irq_set_handler_locked(d, handle_level_irq);
    break;
    case IRQ_TYPE_LEVEL_LOW:
    pch_pic_bitclr(priv, PCH_PIC_EDGE, bit);
    pch_pic_bitset(priv, PCH_PIC_POL, bit);
    irq_set_handler_locked(d, handle_level_irq);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pch_pic_ack_irq(d: *mut irq_data) {
    static void pch_pic_ack_irq(struct irq_data *d)
    {
    unsigned int reg;
    struct pch_pic *priv = irq_data_get_irq_chip_data(d);
    let mut bit: c_int = hwirq_to_bit(priv, d.hwirq);
    reg = readl(priv.base + PCH_PIC_EDGE + PIC_REG_IDX(bit) * 4);
    if (reg & BIT(PIC_REG_BIT(bit))) {
    writel(BIT(PIC_REG_BIT(bit)),
    priv.base + PCH_PIC_CLR + PIC_REG_IDX(bit) * 4);
    }
    irq_chip_ack_parent(d);
    }
    static struct irq_chip pch_pic_irq_chip = {
    .name			= "PCH PIC",
    .irq_mask		= pch_pic_mask_irq,
    .irq_unmask		= pch_pic_unmask_irq,
    .irq_ack		= pch_pic_ack_irq,
    .irq_set_affinity	= irq_chip_set_affinity_parent,
    .irq_set_type		= pch_pic_set_type,
    .flags			= IRQCHIP_SKIP_SET_WAKE,
    };
    static int pch_pic_domain_translate(struct irq_domain *d,
    struct irq_fwspec *fwspec,
    unsigned long *hwirq,
    unsigned int *type)
    {
    struct pch_pic *priv = d.host_data;
    struct device_node *of_node = to_of_node(fwspec.fwnode);
    unsigned long flags;
    int i;
    if (of_node) {
    if (fwspec.param_count < 2)
    return -EINVAL;
// hwirq = fwspec->param[0];
// type = fwspec->param[1] & IRQ_TYPE_SENSE_MASK;
    } else {
    if (fwspec.param_count < 1)
    return -EINVAL;
// hwirq = fwspec->param[0] - priv->gsi_base;
    if (fwspec.param_count > 1)
// type = fwspec->param[1] & IRQ_TYPE_SENSE_MASK;
    else
// type = IRQ_TYPE_NONE;
    }
    raw_spin_lock_irqsave(&priv.pic_lock, flags);
// Check pic-table to confirm if the hwirq has been assigned
    for (i = 0; i < priv.inuse; i++) {
    if (priv.table[i] == *hwirq) {
// hwirq = i;
    break;
    }
    }
    if (i == priv.inuse) {
// Assign a new hwirq in pic-table
    if (priv.inuse >= PIC_COUNT) {
    pr_err("pch-pic domain has no free vectors\n");
    raw_spin_unlock_irqrestore(&priv.pic_lock, flags);
    return -EINVAL;
    }
    priv.table[priv.inuse] = *hwirq;
// hwirq = priv->inuse++;
    }
    raw_spin_unlock_irqrestore(&priv.pic_lock, flags);
    return 0;
    }
    static int pch_pic_alloc(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs, void *arg)
    {
    int err;
    unsigned int type;
    unsigned long hwirq;
    struct irq_fwspec *fwspec = arg;
    struct irq_fwspec parent_fwspec;
    struct pch_pic *priv = domain.host_data;
    err = pch_pic_domain_translate(domain, fwspec, &hwirq, &type);
    if (err)
    return err;
// Write vector ID
    writeb(priv.ht_vec_base + hwirq, priv.base + PCH_INT_HTVEC(hwirq_to_bit(priv, hwirq)));
    parent_fwspec.fwnode = domain.parent.fwnode;
    parent_fwspec.param_count = 1;
    parent_fwspec.param[0] = hwirq + priv.ht_vec_base;
    err = irq_domain_alloc_irqs_parent(domain, virq, 1, &parent_fwspec);
    if (err)
    return err;
    irq_domain_set_info(domain, virq, hwirq,
    &pch_pic_irq_chip, priv,
    handle_level_irq, core::ptr::null_mut(), core::ptr::null_mut());
    irq_set_probe(virq);
    return 0;
    }
    static const struct irq_domain_ops pch_pic_domain_ops = {
    .translate	= pch_pic_domain_translate,
    .alloc		= pch_pic_alloc,
    .free		= irq_domain_free_irqs_parent,
    };
#[no_mangle]
unsafe extern "C" fn pch_pic_reset(priv: *mut pch_pic) {
    static void pch_pic_reset(struct pch_pic *priv)
    {
    int i;
    for (i = 0; i < PIC_COUNT; i++) {
// Write vector ID
    writeb(priv.ht_vec_base + i, priv.base + PCH_INT_HTVEC(hwirq_to_bit(priv, i)));
// Hardcode route to HT0 Lo
    writeb(1, priv.base + PCH_INT_ROUTE(i));
    }
    for (i = 0; i < PIC_REG_COUNT; i++) {
// Clear IRQ cause registers, mask all interrupts
    writel_relaxed(0xFFFFFFFF, priv.base + PCH_PIC_MASK + 4 * i);
    writel_relaxed(0xFFFFFFFF, priv.base + PCH_PIC_CLR + 4 * i);
// Clear auto bounce, we don't need that
    writel_relaxed(0, priv.base + PCH_PIC_AUTO0 + 4 * i);
    writel_relaxed(0, priv.base + PCH_PIC_AUTO1 + 4 * i);
// Enable HTMSI transformer
    writel_relaxed(0xFFFFFFFF, priv.base + PCH_PIC_HTMSI_EN + 4 * i);
    }
    }
#[no_mangle]
unsafe extern "C" fn pch_pic_suspend(data: *mut c_void) -> c_int {
    static int pch_pic_suspend(void *data)
    {
    int i, j;
    for (i = 0; i < nr_pics; i++) {
    for (j = 0; j < PIC_REG_COUNT; j++) {
    pch_pic_priv[i].saved_vec_pol[j] =
    readl(pch_pic_priv[i].base + PCH_PIC_POL + 4 * j);
    pch_pic_priv[i].saved_vec_edge[j] =
    readl(pch_pic_priv[i].base + PCH_PIC_EDGE + 4 * j);
    pch_pic_priv[i].saved_vec_en[j] =
    readl(pch_pic_priv[i].base + PCH_PIC_MASK + 4 * j);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pch_pic_resume(data: *mut c_void) {
    static void pch_pic_resume(void *data)
    {
    int i, j;
    for (i = 0; i < nr_pics; i++) {
    pch_pic_reset(pch_pic_priv[i]);
    for (j = 0; j < PIC_REG_COUNT; j++) {
    writel(pch_pic_priv[i].saved_vec_pol[j],
    pch_pic_priv[i].base + PCH_PIC_POL + 4 * j);
    writel(pch_pic_priv[i].saved_vec_edge[j],
    pch_pic_priv[i].base + PCH_PIC_EDGE + 4 * j);
    writel(pch_pic_priv[i].saved_vec_en[j],
    pch_pic_priv[i].base + PCH_PIC_MASK + 4 * j);
    }
    }
    }
    static const struct syscore_ops pch_pic_syscore_ops = {
    .suspend =  pch_pic_suspend,
    .resume =  pch_pic_resume,
    };
    static struct syscore pch_pic_syscore = {
    .ops = &pch_pic_syscore_ops,
    };
    static int pch_pic_init(phys_addr_t addr, unsigned long size, int vec_base,
    struct irq_domain *parent_domain, struct fwnode_handle *domain_handle,
    u32 gsi_base)
    {
    struct pch_pic *priv;
    int i;
    priv = kzalloc_obj(*priv);
    if (!priv)
    return -ENOMEM;
    raw_spin_lock_init(&priv.pic_lock);
    priv.base = ioremap(addr, size);
    if (!priv.base)
    goto free_priv;
    priv.inuse = 0;
    for (i = 0; i < PIC_COUNT; i++)
    priv.table[i] = PIC_UNDEF_VECTOR;
    priv.ht_vec_base = vec_base;
    priv.vec_count = ((readl(priv.base + 4) >> 16) & 0xff) + 1;
    priv.gsi_base = gsi_base;
    priv.pic_domain = irq_domain_create_hierarchy(parent_domain, 0,
    priv.vec_count, domain_handle,
    &pch_pic_domain_ops, priv);
    if (!priv.pic_domain) {
    pr_err("Failed to create IRQ domain\n");
    goto iounmap_base;
    }
    pch_pic_reset(priv);
    pch_pic_handle[nr_pics] = domain_handle;
    pch_pic_priv[nr_pics++] = priv;
    if (nr_pics == 1)
    register_syscore(&pch_pic_syscore);
    return 0;
    iounmap_base:
    iounmap(priv.base);
    free_priv:
    kfree(priv);
    return -EINVAL;
    }

    static int pch_pic_of_init(struct device_node *node,
    struct device_node *parent)
    {
    int err, vec_base;
    struct resource res;
    struct irq_domain *parent_domain;
    if (of_address_to_resource(node, 0, &res))
    return -EINVAL;
    parent_domain = irq_find_host(parent);
    if (!parent_domain) {
    pr_err("Failed to find the parent domain\n");
    return -ENXIO;
    }
    if (of_property_read_u32(node, "loongson,pic-base-vec", &vec_base)) {
    pr_err("Failed to determine pic-base-vec\n");
    return -EINVAL;
    }
    err = pch_pic_init(res.start, resource_size(&res), vec_base,
    parent_domain, of_fwnode_handle(node), 0);
    if (err < 0)
    return err;
    return 0;
    }
    IRQCHIP_DECLARE(pch_pic, "loongson,pch-pic-1.0", pch_pic_of_init);

#[no_mangle]
pub unsafe extern "C" fn find_pch_pic(gsi: u32) -> c_int {
    int find_pch_pic(u32 gsi)
    {
    int i;
// Find the PCH_PIC that manages this GSI.
    for (i = 0; i < MAX_IO_PICS; i++) {
    struct pch_pic *priv = pch_pic_priv[i];
    if (!priv)
    return -1;
    if (gsi >= priv.gsi_base && gsi < (priv.gsi_base + priv.vec_count))
    return i;
    }
    pr_err("ERROR: Unable to locate PCH_PIC for GSI %d\n", gsi);
    return -1;
    }
    static int __init pch_lpc_parse_madt(union acpi_subtable_headers *header,
    const unsigned long end)
    {
    struct acpi_madt_lpc_pic *pchlpc_entry = (struct acpi_madt_lpc_pic *)header;
    return pch_lpc_acpi_init(pch_pic_priv[0].pic_domain, pchlpc_entry);
    }
#[no_mangle]
unsafe extern "C" fn acpi_cascade_irqdomain_init() -> int __init {
    static int __init acpi_cascade_irqdomain_init(void)
    {
    int r;
    r = acpi_table_parse_madt(ACPI_MADT_TYPE_LPC_PIC, pch_lpc_parse_madt, 0);
    if (r < 0)
    return r;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pch_pic_acpi_init(parent: *mut irq_domain, acpi_pchpic: *mut acpi_madt_bio_pic) -> int __init {
    int __init pch_pic_acpi_init(struct irq_domain *parent, struct acpi_madt_bio_pic *acpi_pchpic)
    {
    let mut addr: phys_addr_t = (phys_addr_t)acpi_pchpic.address;
    struct fwnode_handle *domain_handle;
    int ret;
    if (find_pch_pic(acpi_pchpic.gsi_base) >= 0)
    return 0;
    domain_handle = irq_domain_alloc_fwnode(&addr);
    if (!domain_handle) {
    pr_err("Unable to allocate domain handle\n");
    return -ENOMEM;
    }
    ret = pch_pic_init(addr, acpi_pchpic.size, 0, parent,
    domain_handle, acpi_pchpic.gsi_base);
    if (ret < 0) {
    irq_domain_free_fwnode(domain_handle);
    return ret;
    }
    if (acpi_pchpic.id == 0)
    ret = acpi_cascade_irqdomain_init();
    return ret;
    }
