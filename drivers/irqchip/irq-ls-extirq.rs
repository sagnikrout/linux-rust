//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-ls-extirq.c
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

pub const MAXIRQ: c_int = 12;
pub const LS1021A_SCFGREVCR: c_uint = 0x200;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls_extirq_data {
    pub intpcr: *mut void __iomem,
    pub lock: raw_spinlock_t,
    pub big_endian: bool,
    pub is_ls1021a_or_ls1043a: bool,
    pub nirq: u32,
    pub map: [irq_fwspec; MAXIRQ],
}

    static void ls_extirq_intpcr_rmw(struct ls_extirq_data *priv, u32 mask,
    u32 value)
    {
    u32 intpcr;
//
// Serialize concurrent calls to ls_extirq_set_type() from multiple
// IRQ descriptors, making sure the read-modify-write is atomic.
//
    raw_spin_lock(&priv.lock);
    if (priv.big_endian)
    intpcr = ioread32be(priv.intpcr);
    else
    intpcr = ioread32(priv.intpcr);
    intpcr &= ~mask;
    intpcr |= value;
    if (priv.big_endian)
    iowrite32be(intpcr, priv.intpcr);
    else
    iowrite32(intpcr, priv.intpcr);
    raw_spin_unlock(&priv.lock);
    }
    static int
    ls_extirq_set_type(struct irq_data *data, unsigned int type)
    {
    struct ls_extirq_data *priv = data.chip_data;
    let mut hwirq: irq_hw_number_t = data.hwirq;
    u32 value, mask;
    if (priv.is_ls1021a_or_ls1043a)
    mask = 1U << (31 - hwirq);
    else
    mask = 1U << hwirq;
    switch (type) {
    case IRQ_TYPE_LEVEL_LOW:
    type = IRQ_TYPE_LEVEL_HIGH;
    value = mask;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    type = IRQ_TYPE_EDGE_RISING;
    value = mask;
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    case IRQ_TYPE_EDGE_RISING:
    value = 0;
    break;
    default:
    return -EINVAL;
    }
    ls_extirq_intpcr_rmw(priv, mask, value);
    return irq_chip_set_type_parent(data, type);
    }
    static struct irq_chip ls_extirq_chip = {
    .name			= "ls-extirq",
    .irq_mask		= irq_chip_mask_parent,
    .irq_unmask		= irq_chip_unmask_parent,
    .irq_eoi		= irq_chip_eoi_parent,
    .irq_set_type		= ls_extirq_set_type,
    .irq_retrigger		= irq_chip_retrigger_hierarchy,
    .irq_set_affinity	= irq_chip_set_affinity_parent,
    .flags                  = IRQCHIP_SET_TYPE_MASKED | IRQCHIP_SKIP_SET_WAKE,
    };
    static int
    ls_extirq_domain_alloc(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs, void *arg)
    {
    struct ls_extirq_data *priv = domain.host_data;
    struct irq_fwspec *fwspec = arg;
    irq_hw_number_t hwirq;
    if (fwspec.param_count != 2)
    return -EINVAL;
    hwirq = fwspec.param[0];
    if (hwirq >= priv.nirq)
    return -EINVAL;
    irq_domain_set_hwirq_and_chip(domain, virq, hwirq, &ls_extirq_chip,
    priv);
    return irq_domain_alloc_irqs_parent(domain, virq, 1, &priv.map[hwirq]);
    }
    static const struct irq_domain_ops extirq_domain_ops = {
    .xlate		= irq_domain_xlate_twocell,
    .alloc		= ls_extirq_domain_alloc,
    .free		= irq_domain_free_irqs_common,
    };
    static int
    ls_extirq_parse_map(struct ls_extirq_data *priv, struct device_node *node)
    {
    const __be32 *map;
    u32 mapsize;
    int ret;
    map = of_get_property(node, "interrupt-map", &mapsize);
    if (!map)
    return -ENOENT;
    if (mapsize % sizeof(*map))
    return -EINVAL;
    mapsize /= sizeof(*map);
    while (mapsize) {
    struct device_node *ipar;
    u32 hwirq, intsize, j;
    if (mapsize < 3)
    return -EINVAL;
    hwirq = be32_to_cpup(map);
    if (hwirq >= MAXIRQ)
    return -EINVAL;
    priv.nirq = max(priv.nirq, hwirq + 1);
    ipar = of_find_node_by_phandle(be32_to_cpup(map + 2));
    map += 3;
    mapsize -= 3;
    if (!ipar)
    return -EINVAL;
    priv.map[hwirq].fwnode = &ipar.fwnode;
    ret = of_property_read_u32(ipar, "#interrupt-cells", &intsize);
    if (ret)
    return ret;
    if (intsize > mapsize)
    return -EINVAL;
    priv.map[hwirq].param_count = intsize;
    for (j = 0; j < intsize; ++j)
    priv.map[hwirq].param[j] = be32_to_cpup(map++);
    mapsize -= intsize;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls_extirq_probe(pdev: *mut platform_device) -> c_int {
    static int ls_extirq_probe(struct platform_device *pdev)
    {
    struct irq_domain *domain, *parent_domain;
    struct device_node *node, *parent;
    struct device *dev = &pdev.dev;
    struct ls_extirq_data *priv;
    int ret;
    node = dev.of_node;
    parent = of_irq_find_parent(node);
    if (!parent)
    return dev_err_probe(dev, -ENODEV, "Failed to get IRQ parent node\n");
    parent_domain = irq_find_host(parent);
    if (!parent_domain)
    return dev_err_probe(dev, -EPROBE_DEFER, "Cannot find parent domain\n");
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return dev_err_probe(dev, -ENOMEM, "Failed to allocate memory\n");
    priv.intpcr = devm_of_iomap(dev, node, 0, core::ptr::null_mut());
    if (IS_ERR(priv.intpcr)) {
    return dev_err_probe(dev, PTR_ERR(priv.intpcr),
    "Cannot ioremap OF node %pOF\n", node);
    }
    ret = ls_extirq_parse_map(priv, node);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to parse IRQ map\n");
    priv.big_endian = of_device_is_big_endian(node.parent);
    priv.is_ls1021a_or_ls1043a = of_device_is_compatible(node, "fsl,ls1021a-extirq") ||
    of_device_is_compatible(node, "fsl,ls1043a-extirq");
    raw_spin_lock_init(&priv.lock);
    domain = irq_domain_create_hierarchy(parent_domain, 0, priv.nirq, of_fwnode_handle(node),
    &extirq_domain_ops, priv);
    if (!domain)
    return dev_err_probe(dev, -ENOMEM, "Failed to add IRQ domain\n");
    return 0;
    }
    static const struct of_device_id ls_extirq_dt_ids[] = {
    { .compatible = "fsl,ls1021a-extirq" },
    { .compatible = "fsl,ls1043a-extirq" },
    { .compatible = "fsl,ls1088a-extirq" },
    {}
    };
    MODULE_DEVICE_TABLE(of, ls_extirq_dt_ids);
    static struct platform_driver ls_extirq_driver = {
    .probe = ls_extirq_probe,
    .driver = {
    .name = "ls-extirq",
    .of_match_table = ls_extirq_dt_ids,
    }
    };
    builtin_platform_driver(ls_extirq_driver);
