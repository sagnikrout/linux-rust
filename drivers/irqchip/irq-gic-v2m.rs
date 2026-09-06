//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-gic-v2m.c
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
// ARM GIC v2m MSI(-X) support
// Support for Message Signaled Interrupts for systems that
// implement ARM Generic Interrupt Controller: GICv2m.
//
// Copyright (C) 2014 Advanced Micro Devices, Inc.
// Authors: Suravee Suthikulpanit <suravee.suthikulpanit@amd.com>
// Harish Kasiviswanathan <harish.kasiviswanathan@amd.com>
// Brandon Anderson <brandon.anderson@amd.com>
//

//
// MSI_TYPER:
// [31:26] Reserved
// [25:16] lowest SPI assigned to MSI
// [15:10] Reserved
// [9:0]   Numer of SPIs assigned to MSI
//
pub const V2M_MSI_TYPER: c_uint = 0x008;
pub const V2M_MSI_TYPER_BASE_SHIFT: c_int = 16;
pub const V2M_MSI_TYPER_BASE_MASK: c_uint = 0x3FF;
pub const V2M_MSI_TYPER_NUM_MASK: c_uint = 0x3FF;
pub const V2M_MSI_SETSPI_NS: c_uint = 0x040;
pub const V2M_MIN_SPI: c_int = 32;
pub const V2M_MAX_SPI: c_int = 1019;
pub const V2M_MSI_IIDR: c_uint = 0xFCC;

    (((x) >> V2M_MSI_TYPER_BASE_SHIFT) & V2M_MSI_TYPER_BASE_MASK)

// APM X-Gene with GICv2m MSI_IIDR register value
pub const XGENE_GICV2M_MSI_IIDR: c_uint = 0x06000170;
// Broadcom NS2 GICv2m MSI_IIDR register value
pub const BCM_NS2_GICV2M_MSI_IIDR: c_uint = 0x0000013f;
// List of flags for specific v2m implementation
pub const GICV2M_NEEDS_SPI_OFFSET: c_uint = 0x00000001;
pub const GICV2M_GRAVITON_ADDRESS_ONLY: c_uint = 0x00000002;
    static LIST_HEAD(v2m_nodes);
    static DEFINE_SPINLOCK(v2m_lock);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v2m_data {
    pub entry: list_head,
    pub fwnode: *mut fwnode_handle,
    pub /: *mut *mut resource res; / GICv2m resource,
    pub /: *mut *mut *mut void __iomem base; / GICv2m virt address,
    pub /: *mut *mut u32 spi_start; / The SPI number that MSIs start,
    pub /: *mut *mut u32 nr_spis; / The number of SPIs for MSIs,
    pub /: *mut *mut u32 spi_offset; / offset to be subtracted from SPI number,
    pub /: *mut *mut *mut unsigned long bm; / MSI vector bitmap,
    pub /: *mut *mut u32 flags; / v2m flags for specific implementation,
}

#[no_mangle]
unsafe extern "C" fn gicv2m_get_msi_addr(v2m: *mut v2m_data, hwirq: c_int) -> phys_addr_t {
    static phys_addr_t gicv2m_get_msi_addr(struct v2m_data *v2m, int hwirq)
    {
    if (v2m.flags & GICV2M_GRAVITON_ADDRESS_ONLY)
    return v2m.res.start | ((hwirq - 32) << 3);
    else
    return v2m.res.start + V2M_MSI_SETSPI_NS;
    }
#[no_mangle]
unsafe extern "C" fn gicv2m_compose_msi_msg(data: *mut irq_data, msg: *mut msi_msg) {
    static void gicv2m_compose_msi_msg(struct irq_data *data, struct msi_msg *msg)
    {
    struct v2m_data *v2m = irq_data_get_irq_chip_data(data);
    let mut addr: phys_addr_t = gicv2m_get_msi_addr(v2m, data.hwirq);
    if (v2m.flags & GICV2M_GRAVITON_ADDRESS_ONLY)
    msg.data = 0;
    else
    msg.data = data.hwirq;
    if (v2m.flags & GICV2M_NEEDS_SPI_OFFSET)
    msg.data -= v2m.spi_offset;
    msi_msg_set_addr(irq_data_get_msi_desc(data), msg, addr);
    }
    static struct irq_chip gicv2m_irq_chip = {
    .name			= "GICv2m",
    .irq_mask		= irq_chip_mask_parent,
    .irq_unmask		= irq_chip_unmask_parent,
    .irq_eoi		= irq_chip_eoi_parent,
    .irq_set_affinity	= irq_chip_set_affinity_parent,
    .irq_compose_msi_msg	= gicv2m_compose_msi_msg,
    };
    static int gicv2m_irq_gic_domain_alloc(struct irq_domain *domain,
    unsigned int virq,
    irq_hw_number_t hwirq)
    {
    struct irq_fwspec fwspec;
    struct irq_data *d;
    int err;
    if (is_of_node(domain.parent.fwnode)) {
    fwspec.fwnode = domain.parent.fwnode;
    fwspec.param_count = 3;
    fwspec.param[0] = 0;
    fwspec.param[1] = hwirq - 32;
    fwspec.param[2] = IRQ_TYPE_EDGE_RISING;
    } else if (is_fwnode_irqchip(domain.parent.fwnode)) {
    fwspec.fwnode = domain.parent.fwnode;
    fwspec.param_count = 2;
    fwspec.param[0] = hwirq;
    fwspec.param[1] = IRQ_TYPE_EDGE_RISING;
    } else {
    return -EINVAL;
    }
    err = irq_domain_alloc_irqs_parent(domain, virq, 1, &fwspec);
    if (err)
    return err;
// Configure the interrupt line to be edge
    d = irq_domain_get_irq_data(domain.parent, virq);
    d.chip.irq_set_type(d, IRQ_TYPE_EDGE_RISING);
    return 0;
    }
    static void gicv2m_unalloc_msi(struct v2m_data *v2m, unsigned int hwirq,
    int nr_irqs)
    {
    spin_lock(&v2m_lock);
    bitmap_release_region(v2m.bm, hwirq - v2m.spi_start,
    get_count_order(nr_irqs));
    spin_unlock(&v2m_lock);
    }
    static int gicv2m_irq_domain_alloc(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs, void *args)
    {
    msi_alloc_info_t *info = args;
    struct v2m_data *v2m = core::ptr::null_mut(), *tmp;
    int hwirq, i, err = 0;
    unsigned long offset;
    let mut align_mask: c_ulong = nr_irqs - 1;
    spin_lock(&v2m_lock);
    list_for_each_entry(tmp, &v2m_nodes, entry) {
    let mut align_off: c_ulong = tmp.spi_start - (tmp.spi_start & ~align_mask);
    offset = bitmap_find_next_zero_area_off(tmp.bm, tmp.nr_spis, 0,
    nr_irqs, align_mask, align_off);
    if (offset < tmp.nr_spis) {
    v2m = tmp;
    bitmap_set(v2m.bm, offset, nr_irqs);
    break;
    }
    }
    spin_unlock(&v2m_lock);
    if (!v2m)
    return -ENOSPC;
    hwirq = v2m.spi_start + offset;
    err = iommu_dma_prepare_msi(info.desc,
    gicv2m_get_msi_addr(v2m, hwirq));
    if (err)
    return err;
    for (i = 0; i < nr_irqs; i++) {
    err = gicv2m_irq_gic_domain_alloc(domain, virq + i, hwirq + i);
    if (err)
    goto fail;
    irq_domain_set_hwirq_and_chip(domain, virq + i, hwirq + i,
    &gicv2m_irq_chip, v2m);
    }
    return 0;
    fail:
    irq_domain_free_irqs_parent(domain, virq, nr_irqs);
    gicv2m_unalloc_msi(v2m, hwirq, nr_irqs);
    return err;
    }
    static void gicv2m_irq_domain_free(struct irq_domain *domain,
    unsigned int virq, unsigned int nr_irqs)
    {
    struct irq_data *d = irq_domain_get_irq_data(domain, virq);
    struct v2m_data *v2m = irq_data_get_irq_chip_data(d);
    gicv2m_unalloc_msi(v2m, d.hwirq, nr_irqs);
    irq_domain_free_irqs_parent(domain, virq, nr_irqs);
    }
    static const struct irq_domain_ops gicv2m_domain_ops = {
    .select			= msi_lib_irq_domain_select,
    .alloc			= gicv2m_irq_domain_alloc,
    .free			= gicv2m_irq_domain_free,
    };
#[no_mangle]
unsafe extern "C" fn is_msi_spi_valid(base: u32, num: u32) -> bool {
    static bool is_msi_spi_valid(u32 base, u32 num)
    {
    if (base < V2M_MIN_SPI) {
    pr_err("Invalid MSI base SPI (base:%u)\n", base);
    return false;
    }
    if ((num == 0) || (base + num > V2M_MAX_SPI)) {
    pr_err("Number of SPIs (%u) exceed maximum (%u)\n",
    num, V2M_MAX_SPI - V2M_MIN_SPI + 1);
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn gicv2m_teardown() -> void __init {
    static void __init gicv2m_teardown(void)
    {
    struct v2m_data *v2m, *tmp;
    list_for_each_entry_safe(v2m, tmp, &v2m_nodes, entry) {
    list_del(&v2m.entry);
    bitmap_free(v2m.bm);
    iounmap(v2m.base);
    of_node_put(to_of_node(v2m.fwnode));
    if (is_fwnode_irqchip(v2m.fwnode))
    irq_domain_free_fwnode(v2m.fwnode);
    kfree(v2m);
    }
    }

    MSI_FLAG_USE_DEF_CHIP_OPS |		\
    MSI_FLAG_PCI_MSI_MASK_PARENT)

    MSI_FLAG_PCI_MSIX      |	\
    MSI_FLAG_MULTI_PCI_MSI)
    static struct msi_parent_ops gicv2m_msi_parent_ops = {
    .supported_flags	= GICV2M_MSI_FLAGS_SUPPORTED,
    .required_flags		= GICV2M_MSI_FLAGS_REQUIRED,
    .chip_flags		= MSI_CHIP_FLAG_SET_EOI,
    .bus_select_token	= DOMAIN_BUS_NEXUS,
    .bus_select_mask	= MATCH_PCI_MSI | MATCH_PLATFORM_MSI,
    .prefix			= "GICv2m-",
    .init_dev_msi_info	= msi_lib_init_dev_msi_info,
    };
#[no_mangle]
unsafe extern "C" fn gicv2m_allocate_domains(parent: *mut irq_domain) -> __init int {
    static __init int gicv2m_allocate_domains(struct irq_domain *parent)
    {
    struct irq_domain_info info = {
    .ops		= &gicv2m_domain_ops,
    .parent		= parent,
    };
    struct v2m_data *v2m;
    v2m = list_first_entry_or_null(&v2m_nodes, struct v2m_data, entry);
    if (!v2m)
    return 0;
    info.host_data = v2m;
    info.fwnode = v2m.fwnode;
    if (!msi_create_parent_irq_domain(&info, &gicv2m_msi_parent_ops)) {
    pr_err("Failed to create GICv2m domain\n");
    return -ENOMEM;
    }
    return 0;
    }
    static int __init gicv2m_init_one(struct fwnode_handle *fwnode,
    u32 spi_start, u32 nr_spis,
    struct resource *res, u32 flags)
    {
    int ret;
    struct v2m_data *v2m;
    v2m = kzalloc_obj(struct v2m_data);
    if (!v2m)
    return -ENOMEM;
    INIT_LIST_HEAD(&v2m.entry);
    v2m.fwnode = fwnode;
    v2m.flags = flags;
    memcpy(&v2m.res, res, sizeof(struct resource));
    v2m.base = ioremap(v2m.res.start, resource_size(&v2m.res));
    if (!v2m.base) {
    pr_err("Failed to map GICv2m resource\n");
    ret = -ENOMEM;
    goto err_free_v2m;
    }
    if (spi_start && nr_spis) {
    v2m.spi_start = spi_start;
    v2m.nr_spis = nr_spis;
    } else {
    u32 typer;
// Graviton should always have explicit spi_start/nr_spis
    if (v2m.flags & GICV2M_GRAVITON_ADDRESS_ONLY) {
    ret = -EINVAL;
    goto err_iounmap;
    }
    typer = readl_relaxed(v2m.base + V2M_MSI_TYPER);
    v2m.spi_start = V2M_MSI_TYPER_BASE_SPI(typer);
    v2m.nr_spis = V2M_MSI_TYPER_NUM_SPI(typer);
    }
    if (!is_msi_spi_valid(v2m.spi_start, v2m.nr_spis)) {
    ret = -EINVAL;
    goto err_iounmap;
    }
//
// APM X-Gene GICv2m implementation has an erratum where
// the MSI data needs to be the offset from the spi_start
// in order to trigger the correct MSI interrupt. This is
// different from the standard GICv2m implementation where
// the MSI data is the absolute value within the range from
// spi_start to (spi_start + num_spis).
//
// Broadcom NS2 GICv2m implementation has an erratum where the MSI data
// is 'spi_number - 32'
//
// Reading that register fails on the Graviton implementation
//
    if (!(v2m.flags & GICV2M_GRAVITON_ADDRESS_ONLY)) {
    switch (readl_relaxed(v2m.base + V2M_MSI_IIDR)) {
    case XGENE_GICV2M_MSI_IIDR:
    v2m.flags |= GICV2M_NEEDS_SPI_OFFSET;
    v2m.spi_offset = v2m.spi_start;
    break;
    case BCM_NS2_GICV2M_MSI_IIDR:
    v2m.flags |= GICV2M_NEEDS_SPI_OFFSET;
    v2m.spi_offset = 32;
    break;
    }
    }
    v2m.bm = bitmap_zalloc(v2m.nr_spis, GFP_KERNEL);
    if (!v2m.bm) {
    ret = -ENOMEM;
    goto err_iounmap;
    }
    list_add_tail(&v2m.entry, &v2m_nodes);
    pr_info("range%pR, SPI[%d:%d]\n", res,
    v2m.spi_start, (v2m.spi_start + v2m.nr_spis - 1));
    return 0;
    err_iounmap:
    iounmap(v2m.base);
    err_free_v2m:
    kfree(v2m);
    return ret;
    }
    static __initconst struct of_device_id gicv2m_device_id[] = {
    {	.compatible	= "arm,gic-v2m-frame",	},
    {},
    };
    static int __init gicv2m_of_init(struct fwnode_handle *parent_handle,
    struct irq_domain *parent)
    {
    let mut ret: c_int = 0;
    struct device_node *node = to_of_node(parent_handle);
    struct device_node *child;
    for (child = of_find_matching_node(node, gicv2m_device_id); child;
    child = of_find_matching_node(child, gicv2m_device_id)) {
    let mut spi_start: u32 = 0, nr_spis = 0;
    struct resource res;
    if (!of_property_read_bool(child, "msi-controller"))
    continue;
    ret = of_address_to_resource(child, 0, &res);
    if (ret) {
    pr_err("Failed to allocate v2m resource.\n");
    break;
    }
    if (!of_property_read_u32(child, "arm,msi-base-spi",
    &spi_start) &&
    !of_property_read_u32(child, "arm,msi-num-spis", &nr_spis))
    pr_info("DT overriding V2M MSI_TYPER (base:%u, num:%u)\n",
    spi_start, nr_spis);
    ret = gicv2m_init_one(&child.fwnode, spi_start, nr_spis,
    &res, 0);
    if (ret)
    break;
    }
    if (ret && child)
    of_node_put(child);
    if (!ret)
    ret = gicv2m_allocate_domains(parent);
    if (ret)
    gicv2m_teardown();
    return ret;
    }

    static int acpi_num_msi;
    static struct fwnode_handle *gicv2m_get_fwnode(struct device *dev)
    {
    struct v2m_data *data;
    if (WARN_ON(acpi_num_msi <= 0))
    return core::ptr::null_mut();
// We only return the fwnode of the first MSI frame.
    data = list_first_entry_or_null(&v2m_nodes, struct v2m_data, entry);
    if (!data)
    return core::ptr::null_mut();
    return data.fwnode;
    }
#[no_mangle]
unsafe extern "C" fn acpi_check_amazon_graviton_quirks() -> __init bool {
    static __init bool acpi_check_amazon_graviton_quirks(void)
    {
    static struct acpi_table_madt *madt;
    acpi_status status;
    let mut rc: bool = false;

    status = acpi_get_table(ACPI_SIG_MADT, 0,
    (struct acpi_table_header **)&madt);
    if (ACPI_FAILURE(status) || !madt)
    return rc;
    rc = !memcmp(madt.header.oem_id, ACPI_AMZN_OEM_ID, ACPI_OEM_ID_SIZE);
    acpi_put_table((struct acpi_table_header *)madt);
    return rc;
    }
    static int __init
    acpi_parse_madt_msi(union acpi_subtable_headers *header,
    const unsigned long end)
    {
    int ret;
    struct resource res;
    let mut spi_start: u32 = 0, nr_spis = 0;
    struct acpi_madt_generic_msi_frame *m;
    struct fwnode_handle *fwnode;
    let mut flags: u32 = 0;
    m = (struct acpi_madt_generic_msi_frame *)header;
    if (BAD_MADT_ENTRY(m, end))
    return -EINVAL;
    res.start = m.base_address;
    res.end = m.base_address + SZ_4K - 1;
    res.flags = IORESOURCE_MEM;
    if (acpi_check_amazon_graviton_quirks()) {
    pr_info("applying Amazon Graviton quirk\n");
    res.end = res.start + SZ_8K - 1;
    flags |= GICV2M_GRAVITON_ADDRESS_ONLY;
    gicv2m_msi_parent_ops.supported_flags &= ~MSI_FLAG_MULTI_PCI_MSI;
    }
    if (m.flags & ACPI_MADT_OVERRIDE_SPI_VALUES) {
    spi_start = m.spi_base;
    nr_spis = m.spi_count;
    pr_info("ACPI overriding V2M MSI_TYPER (base:%u, num:%u)\n",
    spi_start, nr_spis);
    }
    fwnode = irq_domain_alloc_fwnode(&res.start);
    if (!fwnode) {
    pr_err("Unable to allocate GICv2m domain token\n");
    return -EINVAL;
    }
    ret = gicv2m_init_one(fwnode, spi_start, nr_spis, &res, flags);
    if (ret)
    irq_domain_free_fwnode(fwnode);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gicv2m_acpi_init(parent: *mut irq_domain) -> int __init {
    static int __init gicv2m_acpi_init(struct irq_domain *parent)
    {
    int ret;
    if (acpi_num_msi > 0)
    return 0;
    acpi_num_msi = acpi_table_parse_madt(ACPI_MADT_TYPE_GENERIC_MSI_FRAME,
    acpi_parse_madt_msi, 0);
    if (acpi_num_msi <= 0)
    goto err_out;
    ret = gicv2m_allocate_domains(parent);
    if (ret)
    goto err_out;
    pci_msi_register_fwnode_provider(&gicv2m_get_fwnode);
    return 0;
    err_out:
    gicv2m_teardown();
    return -EINVAL;
    }

#[no_mangle]
unsafe extern "C" fn gicv2m_acpi_init(parent: *mut irq_domain) -> int __init {
    static int __init gicv2m_acpi_init(struct irq_domain *parent)
    {
    return -EINVAL;
    }

    int __init gicv2m_init(struct fwnode_handle *parent_handle,
    struct irq_domain *parent)
    {
    if (is_of_node(parent_handle))
    return gicv2m_of_init(parent_handle, parent);
    return gicv2m_acpi_init(parent);
    }
