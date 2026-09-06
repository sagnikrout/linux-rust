//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-sg2042-msi.c
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
// SG2042 MSI Controller
//
// Copyright (C) 2024 Sophgo Technology Inc.
// Copyright (C) 2024 Chen Wang <unicorn_wang@outlook.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg204x_msi_chip_info {
    pub irqchip: *const irq_chip,
    pub parent_ops: *const msi_parent_ops,
}

//
// struct sg204x_msi_chipdata - chip data for the SG204x MSI IRQ controller
// @reg_clr:		clear reg, see TRM, 10.1.33, GP_INTR0_CLR
// @doorbell_addr:	see TRM, 10.1.32, GP_INTR0_SET
// @irq_first:		First vectors number that MSIs starts
// @num_irqs:		Number of vectors for MSIs
// @irq_type:		IRQ type for MSIs
// @msi_map:		mapping for allocated MSI vectors.
// @msi_map_lock:	Lock for msi_map
// @chip_info:		chip specific infomations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg204x_msi_chipdata {
    pub reg_clr: *mut void __iomem,
    pub doorbell_addr: phys_addr_t,
    pub irq_first: u32,
    pub num_irqs: u32,
    pub irq_type: c_uint,
    pub msi_map: *mut c_ulong,
    pub msi_map_lock: mutex,
    pub chip_info: *const sg204x_msi_chip_info,
}

#[no_mangle]
unsafe extern "C" fn sg204x_msi_allocate_hwirq(data: *mut sg204x_msi_chipdata, num_req: c_int) -> c_int {
    static int sg204x_msi_allocate_hwirq(struct sg204x_msi_chipdata *data, int num_req)
    {
    int first;
    guard(mutex)(&data.msi_map_lock);
    first = bitmap_find_free_region(data.msi_map, data.num_irqs,
    get_count_order(num_req));
    return first >= 0 ? first : -ENOSPC;
    }
#[no_mangle]
unsafe extern "C" fn sg204x_msi_free_hwirq(data: *mut sg204x_msi_chipdata, hwirq: c_int, num_req: c_int) {
    static void sg204x_msi_free_hwirq(struct sg204x_msi_chipdata *data, int hwirq, int num_req)
    {
    guard(mutex)(&data.msi_map_lock);
    bitmap_release_region(data.msi_map, hwirq, get_count_order(num_req));
    }
#[no_mangle]
unsafe extern "C" fn sg2042_msi_irq_ack(d: *mut irq_data) {
    static void sg2042_msi_irq_ack(struct irq_data *d)
    {
    struct sg204x_msi_chipdata *data  = irq_data_get_irq_chip_data(d);
    let mut bit_off: c_int = d.hwirq;
    writel(1 << bit_off, data.reg_clr);
    irq_chip_ack_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn sg2042_msi_irq_compose_msi_msg(d: *mut irq_data, msg: *mut msi_msg) {
    static void sg2042_msi_irq_compose_msi_msg(struct irq_data *d, struct msi_msg *msg)
    {
    struct sg204x_msi_chipdata *data = irq_data_get_irq_chip_data(d);
    msg.address_hi = upper_32_bits(data.doorbell_addr);
    msg.address_lo = lower_32_bits(data.doorbell_addr);
    msg.data = 1 << d.hwirq;
    }
    static const struct irq_chip sg2042_msi_middle_irq_chip = {
    .name			= "SG2042 MSI",
    .irq_startup		= irq_chip_startup_parent,
    .irq_shutdown		= irq_chip_shutdown_parent,
    .irq_ack		= sg2042_msi_irq_ack,
    .irq_mask		= irq_chip_mask_parent,
    .irq_unmask		= irq_chip_unmask_parent,

    .irq_set_affinity	= irq_chip_set_affinity_parent,

    .irq_compose_msi_msg	= sg2042_msi_irq_compose_msi_msg,
    };
#[no_mangle]
unsafe extern "C" fn sg2044_msi_irq_ack(d: *mut irq_data) {
    static void sg2044_msi_irq_ack(struct irq_data *d)
    {
    struct sg204x_msi_chipdata *data = irq_data_get_irq_chip_data(d);
    writel(0, (u32 __iomem *)data.reg_clr + d.hwirq);
    irq_chip_ack_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn sg2044_msi_irq_compose_msi_msg(d: *mut irq_data, msg: *mut msi_msg) {
    static void sg2044_msi_irq_compose_msi_msg(struct irq_data *d, struct msi_msg *msg)
    {
    struct sg204x_msi_chipdata *data = irq_data_get_irq_chip_data(d);
    let mut doorbell: phys_addr_t = data.doorbell_addr + 4 * (d.hwirq / 32);
    msg.address_lo = lower_32_bits(doorbell);
    msg.address_hi = upper_32_bits(doorbell);
    msg.data = d.hwirq % 32;
    }
    static struct irq_chip sg2044_msi_middle_irq_chip = {
    .name			= "SG2044 MSI",
    .irq_startup		= irq_chip_startup_parent,
    .irq_shutdown		= irq_chip_shutdown_parent,
    .irq_ack		= sg2044_msi_irq_ack,
    .irq_mask		= irq_chip_mask_parent,
    .irq_unmask		= irq_chip_unmask_parent,

    .irq_set_affinity	= irq_chip_set_affinity_parent,

    .irq_compose_msi_msg	= sg2044_msi_irq_compose_msi_msg,
    };
#[no_mangle]
unsafe extern "C" fn sg204x_msi_parent_domain_alloc(domain: *mut irq_domain, virq: c_uint, hwirq: c_int) -> c_int {
    static int sg204x_msi_parent_domain_alloc(struct irq_domain *domain, unsigned int virq, int hwirq)
    {
    struct sg204x_msi_chipdata *data = domain.host_data;
    struct irq_fwspec fwspec;
    struct irq_data *d;
    int ret;
    fwspec.fwnode = domain.parent.fwnode;
    fwspec.param_count = 2;
    fwspec.param[0] = data.irq_first + hwirq;
    fwspec.param[1] = data.irq_type;
    ret = irq_domain_alloc_irqs_parent(domain, virq, 1, &fwspec);
    if (ret)
    return ret;
    d = irq_domain_get_irq_data(domain.parent, virq);
    return d.chip.irq_set_type(d, data.irq_type);
    }
    static int sg204x_msi_middle_domain_alloc(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs, void *args)
    {
    struct sg204x_msi_chipdata *data = domain.host_data;
    int hwirq, err, i;
    hwirq = sg204x_msi_allocate_hwirq(data, nr_irqs);
    if (hwirq < 0)
    return hwirq;
    for (i = 0; i < nr_irqs; i++) {
    err = sg204x_msi_parent_domain_alloc(domain, virq + i, hwirq + i);
    if (err)
    goto err_hwirq;
    irq_domain_set_hwirq_and_chip(domain, virq + i, hwirq + i,
    data.chip_info.irqchip, data);
    }
    return 0;
    err_hwirq:
    sg204x_msi_free_hwirq(data, hwirq, nr_irqs);
    irq_domain_free_irqs_parent(domain, virq, i);
    return err;
    }
    static void sg204x_msi_middle_domain_free(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs)
    {
    struct irq_data *d = irq_domain_get_irq_data(domain, virq);
    struct sg204x_msi_chipdata *data = irq_data_get_irq_chip_data(d);
    irq_domain_free_irqs_parent(domain, virq, nr_irqs);
    sg204x_msi_free_hwirq(data, d.hwirq, nr_irqs);
    }
    static const struct irq_domain_ops sg204x_msi_middle_domain_ops = {
    .alloc	= sg204x_msi_middle_domain_alloc,
    .free	= sg204x_msi_middle_domain_free,
    .select	= msi_lib_irq_domain_select,
    };

    MSI_FLAG_USE_DEF_CHIP_OPS |		\
    MSI_FLAG_PCI_MSI_MASK_PARENT |	\
    MSI_FLAG_PCI_MSI_STARTUP_PARENT)

    static const struct msi_parent_ops sg2042_msi_parent_ops = {
    .required_flags		= SG2042_MSI_FLAGS_REQUIRED,
    .supported_flags	= SG2042_MSI_FLAGS_SUPPORTED,
    .chip_flags		= MSI_CHIP_FLAG_SET_ACK,
    .bus_select_mask	= MATCH_PCI_MSI,
    .bus_select_token	= DOMAIN_BUS_NEXUS,
    .prefix			= "SG2042-",
    .init_dev_msi_info	= msi_lib_init_dev_msi_info,
    };

    MSI_FLAG_USE_DEF_CHIP_OPS |		\
    MSI_FLAG_PCI_MSI_MASK_PARENT |	\
    MSI_FLAG_PCI_MSI_STARTUP_PARENT)

    MSI_FLAG_MULTI_PCI_MSI |		\
    MSI_FLAG_PCI_MSIX)
    static const struct msi_parent_ops sg2044_msi_parent_ops = {
    .required_flags		= SG2044_MSI_FLAGS_REQUIRED,
    .supported_flags	= SG2044_MSI_FLAGS_SUPPORTED,
    .chip_flags		= MSI_CHIP_FLAG_SET_EOI | MSI_CHIP_FLAG_SET_ACK,
    .bus_select_mask	= MATCH_PCI_MSI,
    .bus_select_token	= DOMAIN_BUS_NEXUS,
    .prefix			= "SG2044-",
    .init_dev_msi_info	= msi_lib_init_dev_msi_info,
    };
    static int sg204x_msi_init_domains(struct sg204x_msi_chipdata *data,
    struct irq_domain *plic_domain, struct device *dev)
    {
    struct irq_domain_info info = {
    .ops		= &sg204x_msi_middle_domain_ops,
    .parent		= plic_domain,
    .size		= data.num_irqs,
    .fwnode		= dev_fwnode(dev),
    .host_data	= data,
    };
    if (!msi_create_parent_irq_domain(&info, data.chip_info.parent_ops)) {
    pr_err("Failed to create the MSI middle domain\n");
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sg2042_msi_probe(pdev: *mut platform_device) -> c_int {
    static int sg2042_msi_probe(struct platform_device *pdev)
    {
    let mut args: fwnode_reference_args = { };
    struct sg204x_msi_chipdata *data;
    struct device *dev = &pdev.dev;
    struct irq_domain *plic_domain;
    struct resource *res;
    int ret;
    data = devm_kzalloc(dev, sizeof(struct sg204x_msi_chipdata), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.chip_info = device_get_match_data(&pdev.dev);
    if (!data.chip_info) {
    dev_err(&pdev.dev, "Failed to get irqchip\n");
    return -EINVAL;
    }
    data.reg_clr = devm_platform_ioremap_resource_byname(pdev, "clr");
    if (IS_ERR(data.reg_clr)) {
    dev_err(dev, "Failed to map clear register\n");
    return PTR_ERR(data.reg_clr);
    }
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "doorbell");
    if (!res) {
    dev_err(dev, "Failed get resource from set\n");
    return -EINVAL;
    }
    data.doorbell_addr = res.start;
    ret = fwnode_property_get_reference_args(dev_fwnode(dev), "msi-ranges",
    "#interrupt-cells", 0, 0, &args);
    if (ret) {
    dev_err(dev, "Unable to parse MSI vec base\n");
    return ret;
    }
    fwnode_handle_put(args.fwnode);
    ret = fwnode_property_get_reference_args(dev_fwnode(dev), "msi-ranges", core::ptr::null_mut(),
    args.nargs + 1, 0, &args);
    if (ret) {
    dev_err(dev, "Unable to parse MSI vec number\n");
    return ret;
    }
    plic_domain = irq_find_matching_fwnode(args.fwnode, DOMAIN_BUS_ANY);
    fwnode_handle_put(args.fwnode);
    if (!plic_domain) {
    pr_err("Failed to find the PLIC domain\n");
    return -ENXIO;
    }
    data.irq_first = (u32)args.args[0];
    data.irq_type = (unsigned int)args.args[1];
    data.num_irqs = (u32)args.args[args.nargs - 1];
    mutex_init(&data.msi_map_lock);
    data.msi_map = devm_bitmap_zalloc(&pdev.dev, data.num_irqs, GFP_KERNEL);
    if (!data.msi_map) {
    dev_err(&pdev.dev, "Unable to allocate msi mapping\n");
    return -ENOMEM;
    }
    return sg204x_msi_init_domains(data, plic_domain, dev);
    }
    static const struct sg204x_msi_chip_info sg2042_chip_info = {
    .irqchip	= &sg2042_msi_middle_irq_chip,
    .parent_ops	= &sg2042_msi_parent_ops,
    };
    static const struct sg204x_msi_chip_info sg2044_chip_info = {
    .irqchip	= &sg2044_msi_middle_irq_chip,
    .parent_ops	= &sg2044_msi_parent_ops,
    };
    static const struct of_device_id sg2042_msi_of_match[] = {
    { .compatible	= "sophgo,sg2042-msi", .data	= &sg2042_chip_info },
    { .compatible	= "sophgo,sg2044-msi", .data	= &sg2044_chip_info },
    { }
    };
    static struct platform_driver sg2042_msi_driver = {
    .driver = {
    .name		= "sg2042-msi",
    .of_match_table	= sg2042_msi_of_match,
    },
    .probe = sg2042_msi_probe,
    };
    builtin_platform_driver(sg2042_msi_driver);
