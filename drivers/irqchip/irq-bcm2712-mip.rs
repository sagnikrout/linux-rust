//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-bcm2712-mip.c
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
// Copyright (C) 2024 Raspberry Pi Ltd., All Rights Reserved.
// Copyright (c) 2024 SUSE
//

pub const MIP_INT_RAISE: c_uint = 0x00;
pub const MIP_INT_CLEAR: c_uint = 0x10;
pub const MIP_INT_CFGL_HOST: c_uint = 0x20;
pub const MIP_INT_CFGH_HOST: c_uint = 0x30;
pub const MIP_INT_MASKL_HOST: c_uint = 0x40;
pub const MIP_INT_MASKH_HOST: c_uint = 0x50;
pub const MIP_INT_MASKL_VPU: c_uint = 0x60;
pub const MIP_INT_MASKH_VPU: c_uint = 0x70;
pub const MIP_INT_STATUSL_HOST: c_uint = 0x80;
pub const MIP_INT_STATUSH_HOST: c_uint = 0x90;
pub const MIP_INT_STATUSL_VPU: c_uint = 0xa0;
pub const MIP_INT_STATUSH_VPU: c_uint = 0xb0;
//
// struct mip_priv - MSI-X interrupt controller data
// @lock:	Used to protect bitmap alloc/free
// @base:	Base address of MMIO area
// @msg_addr:	PCIe MSI-X address
// @msi_base:	MSI base
// @num_msis:	Count of MSIs
// @msi_offset:	MSI offset
// @bitmap:	A bitmap for hwirqs
// @parent:	Parent domain (GIC)
// @dev:	A device pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mip_priv {
    pub lock: spinlock_t,
    pub base: *mut void __iomem,
    pub msg_addr: u64,
    pub msi_base: u32,
    pub num_msis: u32,
    pub msi_offset: u32,
    pub bitmap: *mut c_ulong,
    pub parent: *mut irq_domain,
    pub dev: *mut device,
}

#[no_mangle]
unsafe extern "C" fn mip_compose_msi_msg(d: *mut irq_data, msg: *mut msi_msg) {
    static void mip_compose_msi_msg(struct irq_data *d, struct msi_msg *msg)
    {
    struct mip_priv *mip = irq_data_get_irq_chip_data(d);
    msg.address_hi = upper_32_bits(mip.msg_addr);
    msg.address_lo = lower_32_bits(mip.msg_addr);
    msg.data = d.hwirq;
    }
    static struct irq_chip mip_middle_irq_chip = {
    .name			= "MIP",
    .irq_mask		= irq_chip_mask_parent,
    .irq_unmask		= irq_chip_unmask_parent,
    .irq_eoi		= irq_chip_eoi_parent,
    .irq_set_affinity	= irq_chip_set_affinity_parent,
    .irq_set_type		= irq_chip_set_type_parent,
    .irq_compose_msi_msg	= mip_compose_msi_msg,
    };
#[no_mangle]
unsafe extern "C" fn mip_alloc_hwirq(mip: *mut mip_priv, nr_irqs: c_uint) -> c_int {
    static int mip_alloc_hwirq(struct mip_priv *mip, unsigned int nr_irqs)
    {
    guard(spinlock)(&mip.lock);
    return bitmap_find_free_region(mip.bitmap, mip.num_msis, ilog2(nr_irqs));
    }
    static void mip_free_hwirq(struct mip_priv *mip, unsigned int hwirq,
    unsigned int nr_irqs)
    {
    guard(spinlock)(&mip.lock);
    bitmap_release_region(mip.bitmap, hwirq, ilog2(nr_irqs));
    }
    static int mip_middle_domain_alloc(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs, void *arg)
    {
    struct mip_priv *mip = domain.host_data;
    let mut fwspec: irq_fwspec = {0};
    unsigned int hwirq, i;
    struct irq_data *irqd;
    int irq, ret;
    irq = mip_alloc_hwirq(mip, nr_irqs);
    if (irq < 0)
    return irq;
    hwirq = irq + mip.msi_offset;
    fwspec.fwnode = domain.parent.fwnode;
    fwspec.param_count = 3;
    fwspec.param[0] = 0;
    fwspec.param[1] = hwirq + mip.msi_base;
    fwspec.param[2] = IRQ_TYPE_EDGE_RISING;
    ret = irq_domain_alloc_irqs_parent(domain, virq, nr_irqs, &fwspec);
    if (ret)
    goto err_free_hwirq;
    for (i = 0; i < nr_irqs; i++) {
    irqd = irq_domain_get_irq_data(domain.parent, virq + i);
    irqd.chip.irq_set_type(irqd, IRQ_TYPE_EDGE_RISING);
    ret = irq_domain_set_hwirq_and_chip(domain, virq + i, hwirq + i,
    &mip_middle_irq_chip, mip);
    if (ret)
    goto err_free;
    irqd = irq_get_irq_data(virq + i);
    irqd_set_single_target(irqd);
    irqd_set_affinity_on_activate(irqd);
    }
    return 0;
    err_free:
    irq_domain_free_irqs_parent(domain, virq, nr_irqs);
    err_free_hwirq:
    mip_free_hwirq(mip, irq, nr_irqs);
    return ret;
    }
    static void mip_middle_domain_free(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs)
    {
    struct irq_data *irqd = irq_domain_get_irq_data(domain, virq);
    struct mip_priv *mip;
    unsigned int hwirq;
    if (!irqd)
    return;
    mip = irq_data_get_irq_chip_data(irqd);
    hwirq = irqd_to_hwirq(irqd);
    irq_domain_free_irqs_parent(domain, virq, nr_irqs);
    mip_free_hwirq(mip, hwirq - mip.msi_offset, nr_irqs);
    }
    static const struct irq_domain_ops mip_middle_domain_ops = {
    .select		= msi_lib_irq_domain_select,
    .alloc		= mip_middle_domain_alloc,
    .free		= mip_middle_domain_free,
    };

    MSI_FLAG_USE_DEF_CHIP_OPS |	\
    MSI_FLAG_PCI_MSI_MASK_PARENT)

    MSI_FLAG_MULTI_PCI_MSI |	\
    MSI_FLAG_PCI_MSIX)
    static const struct msi_parent_ops mip_msi_parent_ops = {
    .supported_flags	= MIP_MSI_FLAGS_SUPPORTED,
    .required_flags		= MIP_MSI_FLAGS_REQUIRED,
    .chip_flags		= MSI_CHIP_FLAG_SET_EOI | MSI_CHIP_FLAG_SET_ACK,
    .bus_select_token       = DOMAIN_BUS_GENERIC_MSI,
    .bus_select_mask	= MATCH_PCI_MSI,
    .prefix			= "MIP-MSI-",
    .init_dev_msi_info	= msi_lib_init_dev_msi_info,
    };
#[no_mangle]
unsafe extern "C" fn mip_init_domains(mip: *mut mip_priv, np: *mut device_node) -> c_int {
    static int mip_init_domains(struct mip_priv *mip, struct device_node *np)
    {
    struct irq_domain_info info = {
    .fwnode		= of_fwnode_handle(np),
    .ops		= &mip_middle_domain_ops,
    .host_data	= mip,
    .size		= mip.num_msis,
    .parent		= mip.parent,
    .dev		= mip.dev,
    };
    if (!msi_create_parent_irq_domain(&info, &mip_msi_parent_ops))
    return -ENOMEM;
//
// All MSI-X unmasked for the host, masked for the VPU, and edge-triggered.
//
    writel(0, mip.base + MIP_INT_MASKL_HOST);
    writel(0, mip.base + MIP_INT_MASKH_HOST);
    writel(~0, mip.base + MIP_INT_MASKL_VPU);
    writel(~0, mip.base + MIP_INT_MASKH_VPU);
    writel(~0, mip.base + MIP_INT_CFGL_HOST);
    writel(~0, mip.base + MIP_INT_CFGH_HOST);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mip_parse_dt(mip: *mut mip_priv, np: *mut device_node) -> c_int {
    static int mip_parse_dt(struct mip_priv *mip, struct device_node *np)
    {
    struct of_phandle_args args;
    u64 size;
    int ret;
    ret = of_property_read_u32(np, "brcm,msi-offset", &mip.msi_offset);
    if (ret)
    mip.msi_offset = 0;
    ret = of_parse_phandle_with_args(np, "msi-ranges", "#interrupt-cells",
    0, &args);
    if (ret)
    return ret;
    ret = of_property_read_u32_index(np, "msi-ranges", args.args_count + 1,
    &mip.num_msis);
    if (ret)
    goto err_put;
    ret = of_property_read_reg(np, 1, &mip.msg_addr, &size);
    if (ret)
    goto err_put;
    mip.msi_base = args.args[1];
    mip.parent = irq_find_host(args.np);
    if (!mip.parent)
    ret = -EINVAL;
    err_put:
    of_node_put(args.np);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mip_msi_probe(pdev: *mut platform_device, parent: *mut device_node) -> c_int {
    static int mip_msi_probe(struct platform_device *pdev, struct device_node *parent)
    {
    struct device_node *node = pdev.dev.of_node;
    struct mip_priv *mip;
    int ret;
    mip = kzalloc_obj(*mip);
    if (!mip)
    return -ENOMEM;
    spin_lock_init(&mip.lock);
    mip.dev = &pdev.dev;
    ret = mip_parse_dt(mip, node);
    if (ret)
    goto err_priv;
    mip.base = of_iomap(node, 0);
    if (!mip.base) {
    ret = -ENXIO;
    goto err_priv;
    }
    mip.bitmap = bitmap_zalloc(mip.num_msis, GFP_KERNEL);
    if (!mip.bitmap) {
    ret = -ENOMEM;
    goto err_base;
    }
    ret = mip_init_domains(mip, node);
    if (ret)
    goto err_map;
    dev_dbg(&pdev.dev, "MIP: MSI-X count: %u, base: %u, offset: %u, msg_addr: %llx\n",
    mip.num_msis, mip.msi_base, mip.msi_offset, mip.msg_addr);
    return 0;
    err_map:
    bitmap_free(mip.bitmap);
    err_base:
    iounmap(mip.base);
    err_priv:
    kfree(mip);
    return ret;
    }
    IRQCHIP_PLATFORM_DRIVER_BEGIN(mip_msi)
    IRQCHIP_MATCH("brcm,bcm2712-mip", mip_msi_probe)
    IRQCHIP_PLATFORM_DRIVER_END(mip_msi)
    MODULE_DESCRIPTION("Broadcom BCM2712 MSI-X interrupt controller");
    MODULE_AUTHOR("Phil Elwell <phil@raspberrypi.com>");
    MODULE_AUTHOR("Stanimir Varbanov <svarbanov@suse.de>");
    MODULE_LICENSE("GPL");
