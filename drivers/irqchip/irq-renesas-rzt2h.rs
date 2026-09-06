//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-renesas-rzt2h.c
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

pub const RZT2H_ICU_INTCPU_NS_START: c_int = 0;
pub const RZT2H_ICU_INTCPU_NS_COUNT: c_int = 14;

    RZT2H_ICU_INTCPU_NS_COUNT)
pub const RZT2H_ICU_INTCPU_S_COUNT: c_int = 2;

    RZT2H_ICU_INTCPU_S_COUNT)
pub const RZT2H_ICU_IRQ_NS_COUNT: c_int = 14;

    RZT2H_ICU_IRQ_NS_COUNT)
pub const RZT2H_ICU_IRQ_S_COUNT: c_int = 2;

    RZT2H_ICU_IRQ_S_COUNT)
pub const RZT2H_ICU_SEI_COUNT: c_int = 1;

    RZT2H_ICU_SEI_COUNT)
pub const RZT2H_ICU_CA55_ERR_COUNT: c_int = 2;

    RZT2H_ICU_CA55_ERR_COUNT)
pub const RZT2H_ICU_CR52_ERR_COUNT: c_int = 4;

    RZT2H_ICU_CR52_ERR_COUNT)
pub const RZT2H_ICU_PERI_ERR_COUNT: c_int = 2;

    RZT2H_ICU_PERI_ERR_COUNT)
pub const RZT2H_ICU_DSMIF_ERR_COUNT: c_int = 2;

    RZT2H_ICU_DSMIF_ERR_COUNT)
pub const RZT2H_ICU_ENCIF_ERR_COUNT: c_int = 2;

    RZT2H_ICU_INTCPU_S_COUNT +	\
    RZT2H_ICU_IRQ_NS_COUNT +	\
    RZT2H_ICU_IRQ_S_COUNT +	\
    RZT2H_ICU_SEI_COUNT +		\
    RZT2H_ICU_CA55_ERR_COUNT +	\
    RZT2H_ICU_CR52_ERR_COUNT +	\
    RZT2H_ICU_PERI_ERR_COUNT +	\
    RZT2H_ICU_DSMIF_ERR_COUNT +	\
    RZT2H_ICU_ENCIF_ERR_COUNT)

    ((n) >= RZT2H_ICU_##type##_START &&					\
    (n) <  RZT2H_ICU_##type##_START + RZT2H_ICU_##type##_COUNT)
pub const RZT2H_ICU_SWINT: c_uint = 0x0;

pub const RZT2H_ICU_PORTNF_MD: c_uint = 0xc;

pub const RZT2H_ICU_CA55ERR_E0MSK: c_uint = 0x50;
pub const RZT2H_ICU_CA55ERR_CLR: c_uint = 0x60;
pub const RZT2H_ICU_CA55ERR_STAT: c_uint = 0x64;

pub const RZT2H_ICU_PERIERR_STAT: c_uint = 0xd4;
pub const RZT2H_ICU_PERIERR_NUM: c_int = 3;

pub const RZT2H_ICU_DSMIFERR_STAT: c_uint = 0x1d0;
pub const RZT2H_ICU_DSMIFERR_NUM: c_int = 12;

pub const RZT2H_ICU_ENCIFERR_STAT: c_uint = 0x264;
pub const RZT2H_ICU_ENCIFERR_NUM: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzt2h_icu_priv {
    pub base_ns: *mut void __iomem,
    pub base_s: *mut void __iomem,
    pub fwspec: [irq_fwspec; RZT2H_ICU_NUM_IRQ],
    pub lock: raw_spinlock_t,
}

    void rzt2h_icu_register_dma_req(struct platform_device *icu_dev, u8 dmac_index, u8 dmac_channel,
    u16 req_no)
    {
    struct rzt2h_icu_priv *priv = platform_get_drvdata(icu_dev);
    u8 y, upper;
    u32 val;
    y = dmac_channel / 3;
    upper = dmac_channel % 3;
    guard(raw_spinlock_irqsave)(&priv.lock);
    val = readl(priv.base_ns + RZT2H_ICU_DMACn_RSSELi(dmac_index, y));
    val &= ~RZT2H_ICU_DMAC_REQ_SELx_MASK(upper);
    val |= RZT2H_ICU_DMAC_REQ_SELx_PREP(upper, req_no);
    writel(val, priv.base_ns + RZT2H_ICU_DMACn_RSSELi(dmac_index, y));
    }
    EXPORT_SYMBOL_GPL(rzt2h_icu_register_dma_req);
    static inline struct rzt2h_icu_priv *irq_data_to_priv(struct irq_data *data)
    {
    return data.domain.host_data;
    }
    static inline int rzt2h_icu_irq_to_offset(struct irq_data *d, void __iomem **base,
    unsigned int *offset)
    {
    struct rzt2h_icu_priv *priv = irq_data_to_priv(d);
    let mut hwirq: c_uint = irqd_to_hwirq(d);
//
// Safety IRQs and SEI use a separate register space from the non-safety IRQs.
// SEI interrupt number follows immediately after the safety IRQs.
//
    if (RZT2H_ICU_IRQ_IN_RANGE(hwirq, IRQ_NS)) {
// offset = hwirq - RZT2H_ICU_IRQ_NS_START;
// base = priv->base_ns;
    } else if (RZT2H_ICU_IRQ_IN_RANGE(hwirq, IRQ_S) || RZT2H_ICU_IRQ_IN_RANGE(hwirq, SEI)) {
// offset = hwirq - RZT2H_ICU_IRQ_S_START;
// base = priv->base_s;
    } else if (RZT2H_ICU_IRQ_IN_RANGE(hwirq, INTCPU_NS)) {
// offset = hwirq - RZT2H_ICU_INTCPU_NS_START;
// base = priv->base_ns;
    } else if (RZT2H_ICU_IRQ_IN_RANGE(hwirq, INTCPU_S)) {
// offset = hwirq - RZT2H_ICU_INTCPU_S_START;
// base = priv->base_s;
    } else {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzt2h_icu_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int rzt2h_icu_irq_set_type(struct irq_data *d, unsigned int type)
    {
    struct rzt2h_icu_priv *priv = irq_data_to_priv(d);
    unsigned int offset, parent_type;
    void __iomem *base;
    u32 val, md;
    int ret;
    ret = rzt2h_icu_irq_to_offset(d, &base, &offset);
    if (ret)
    return ret;
    switch (type & IRQ_TYPE_SENSE_MASK) {
    case IRQ_TYPE_LEVEL_LOW:
    md = RZT2H_ICU_MD_LOW_LEVEL;
    parent_type = IRQ_TYPE_LEVEL_HIGH;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    md = RZT2H_ICU_MD_FALLING_EDGE;
    parent_type = IRQ_TYPE_EDGE_RISING;
    break;
    case IRQ_TYPE_EDGE_RISING:
    md = RZT2H_ICU_MD_RISING_EDGE;
    parent_type = IRQ_TYPE_EDGE_RISING;
    break;
    case IRQ_TYPE_EDGE_BOTH:
    md = RZT2H_ICU_MD_BOTH_EDGES;
    parent_type = IRQ_TYPE_EDGE_RISING;
    break;
    default:
    return -EINVAL;
    }
    scoped_guard(raw_spinlock, &priv.lock) {
    val = readl_relaxed(base + RZT2H_ICU_PORTNF_MD);
    val &= ~RZT2H_ICU_PORTNF_MDi_MASK(offset);
    val |= RZT2H_ICU_PORTNF_MDi_PREP(offset, md);
    writel_relaxed(val, base + RZT2H_ICU_PORTNF_MD);
    }
    return irq_chip_set_type_parent(d, parent_type);
    }
#[no_mangle]
unsafe extern "C" fn rzt2h_icu_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int rzt2h_icu_set_type(struct irq_data *d, unsigned int type)
    {
    let mut hw_irq: c_uint = irqd_to_hwirq(d);
// IRQn and SEI are selectable, others are edge-only.
    if (RZT2H_ICU_IRQ_IN_RANGE(hw_irq, IRQ_NS) ||
    RZT2H_ICU_IRQ_IN_RANGE(hw_irq, IRQ_S) ||
    RZT2H_ICU_IRQ_IN_RANGE(hw_irq, SEI))
    return rzt2h_icu_irq_set_type(d, type);
    if ((type & IRQ_TYPE_SENSE_MASK) != IRQ_TYPE_EDGE_RISING)
    return -EINVAL;
    return irq_chip_set_type_parent(d, IRQ_TYPE_EDGE_RISING);
    }
    static int rzt2h_icu_intcpu_set_irqchip_state(struct irq_data *d, enum irqchip_irq_state which,
    bool state)
    {
    unsigned int offset;
    void __iomem *base;
    int ret;
    if (which != IRQCHIP_STATE_PENDING)
    return irq_chip_set_parent_state(d, which, state);
    if (!state)
    return 0;
    ret = rzt2h_icu_irq_to_offset(d, &base, &offset);
    if (ret)
    return ret;
    writel_relaxed(RZT2H_ICU_SWINT_IC_MASK(offset), base + RZT2H_ICU_SWINT);
    return 0;
    }
    static const struct irq_chip rzt2h_icu_chip = {
    .name			= "rzt2h-icu",
    .irq_mask		= irq_chip_mask_parent,
    .irq_unmask		= irq_chip_unmask_parent,
    .irq_eoi		= irq_chip_eoi_parent,
    .irq_set_type		= rzt2h_icu_set_type,
    .irq_set_wake		= irq_chip_set_wake_parent,
    .irq_set_affinity	= irq_chip_set_affinity_parent,
    .irq_retrigger		= irq_chip_retrigger_hierarchy,
    .irq_get_irqchip_state	= irq_chip_get_parent_state,
    .irq_set_irqchip_state	= irq_chip_set_parent_state,
    .flags			= IRQCHIP_MASK_ON_SUSPEND |
    IRQCHIP_SET_TYPE_MASKED |
    IRQCHIP_SKIP_SET_WAKE,
    };
    static const struct irq_chip rzt2h_icu_intcpu_chip = {
    .name			= "rzt2h-icu",
    .irq_mask		= irq_chip_mask_parent,
    .irq_unmask		= irq_chip_unmask_parent,
    .irq_eoi		= irq_chip_eoi_parent,
    .irq_set_type		= irq_chip_set_type_parent,
    .irq_set_wake		= irq_chip_set_wake_parent,
    .irq_set_affinity	= irq_chip_set_affinity_parent,
    .irq_retrigger		= irq_chip_retrigger_hierarchy,
    .irq_get_irqchip_state	= irq_chip_get_parent_state,
    .irq_set_irqchip_state	= rzt2h_icu_intcpu_set_irqchip_state,
    .flags			= IRQCHIP_MASK_ON_SUSPEND |
    IRQCHIP_SET_TYPE_MASKED |
    IRQCHIP_SKIP_SET_WAKE,
    };
    static int rzt2h_icu_alloc(struct irq_domain *domain, unsigned int virq, unsigned int nr_irqs,
    void *arg)
    {
    struct rzt2h_icu_priv *priv = domain.host_data;
    const struct irq_chip *chip;
    irq_hw_number_t hwirq;
    unsigned int type;
    int ret;
    ret = irq_domain_translate_twocell(domain, arg, &hwirq, &type);
    if (ret)
    return ret;
    if (RZT2H_ICU_IRQ_IN_RANGE(hwirq, INTCPU_NS) || RZT2H_ICU_IRQ_IN_RANGE(hwirq, INTCPU_S))
    chip = &rzt2h_icu_intcpu_chip;
    else
    chip = &rzt2h_icu_chip;
    ret = irq_domain_set_hwirq_and_chip(domain, virq, hwirq, chip, core::ptr::null_mut());
    if (ret)
    return ret;
    return irq_domain_alloc_irqs_parent(domain, virq, nr_irqs, &priv.fwspec[hwirq]);
    }
    static const struct irq_domain_ops rzt2h_icu_domain_ops = {
    .alloc		= rzt2h_icu_alloc,
    .free		= irq_domain_free_irqs_common,
    .translate	= irq_domain_translate_twocell,
    };
#[no_mangle]
unsafe extern "C" fn rzt2h_icu_parse_interrupts(priv: *mut rzt2h_icu_priv, np: *mut device_node) -> c_int {
    static int rzt2h_icu_parse_interrupts(struct rzt2h_icu_priv *priv, struct device_node *np)
    {
    struct of_phandle_args map;
    unsigned int i;
    int ret;
    for (i = 0; i < RZT2H_ICU_NUM_IRQ; i++) {
    ret = of_irq_parse_one(np, i, &map);
    if (ret)
    return ret;
    of_phandle_args_to_fwspec(np, map.args, map.args_count, &priv.fwspec[i]);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzt2h_icu_intcpu_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t rzt2h_icu_intcpu_irq(int irq, void *data)
    {
    let mut intcpu: c_uint = (uintptr_t)data;
    pr_info("INTCPU%u software interrupt\n", intcpu);
    return IRQ_HANDLED;
    }
    static irqreturn_t rzt2h_icu_err_irq(struct rzt2h_icu_priv *priv, const char *name,
    unsigned int num, u32 stat_base, u32 clr_base)
    {
    let mut handled: bool = false;
    for (unsigned int n = 0; n < num; n++) {
    let mut stat: u32 = readl(priv.base_ns + stat_base + n * 0x4);
    if (!stat)
    continue;
    handled = true;
    pr_err("rzt2h-icu: %s error n=%u status=0x%08x\n", name, n, stat);
    writel_relaxed(stat, priv.base_ns + clr_base + n * 0x4);
    }
    return handled ? IRQ_HANDLED : IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn rzt2h_icu_ca55_err_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t rzt2h_icu_ca55_err_irq(int irq, void *data)
    {
    return rzt2h_icu_err_irq(data, "CA55", 1, RZT2H_ICU_CA55ERR_STAT, RZT2H_ICU_CA55ERR_CLR);
    }
#[no_mangle]
unsafe extern "C" fn rzt2h_icu_peri_err_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t rzt2h_icu_peri_err_irq(int irq, void *data)
    {
    return rzt2h_icu_err_irq(data, "peripheral", RZT2H_ICU_PERIERR_NUM, RZT2H_ICU_PERIERR_STAT,
    RZT2H_ICU_PERIERR_CLRn(0));
    }
#[no_mangle]
unsafe extern "C" fn rzt2h_icu_dsmif_err_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t rzt2h_icu_dsmif_err_irq(int irq, void *data)
    {
    return rzt2h_icu_err_irq(data, "DSMIF", RZT2H_ICU_DSMIFERR_NUM, RZT2H_ICU_DSMIFERR_STAT,
    RZT2H_ICU_DSMIFERR_CLRn(0));
    }
#[no_mangle]
unsafe extern "C" fn rzt2h_icu_encif_err_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t rzt2h_icu_encif_err_irq(int irq, void *data)
    {
    return rzt2h_icu_err_irq(data, "ENCIF", RZT2H_ICU_ENCIFERR_NUM, RZT2H_ICU_ENCIFERR_STAT,
    RZT2H_ICU_ENCIFERR_CLRn(0));
    }
    static int rzt2h_icu_request_irqs(struct platform_device *pdev, struct irq_domain *irq_domain,
    unsigned int start, unsigned int count, irq_handler_t handler,
    void *data)
    {
    struct device *dev = &pdev.dev;
    unsigned int offset, virq;
    struct irq_fwspec fwspec;
    int ret;
    for (offset = start; offset < start + count; offset++) {
    fwspec.fwnode = irq_domain.fwnode;
    fwspec.param_count = 2;
    fwspec.param[0] = offset;
    fwspec.param[1] = IRQ_TYPE_EDGE_RISING;
    virq = irq_create_fwspec_mapping(&fwspec);
    if (!virq)
    return dev_err_probe(dev, -EINVAL, "Failed to create IRQ %u mapping\n", offset);
    ret = devm_request_irq(dev, virq, handler, 0, dev_name(dev),
    data ?: (void *)(uintptr_t)offset);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzt2h_icu_setup_irqs(pdev: *mut platform_device, irq_domain: *mut irq_domain) -> c_int {
    static int rzt2h_icu_setup_irqs(struct platform_device *pdev, struct irq_domain *irq_domain)
    {
    struct rzt2h_icu_priv *priv = platform_get_drvdata(pdev);
    unsigned int n;
    int ret;
    if (IS_ENABLED(CONFIG_GENERIC_IRQ_INJECTION)) {
    ret = rzt2h_icu_request_irqs(pdev, irq_domain, RZT2H_ICU_INTCPU_NS_START,
    RZT2H_ICU_INTCPU_NS_COUNT, rzt2h_icu_intcpu_irq, core::ptr::null_mut());
    if (ret)
    return ret;
    ret = rzt2h_icu_request_irqs(pdev, irq_domain, RZT2H_ICU_INTCPU_S_START,
    RZT2H_ICU_INTCPU_S_COUNT, rzt2h_icu_intcpu_irq, core::ptr::null_mut());
    if (ret)
    return ret;
    }
//
// There are two error interrupts and two error masks that can be used
// separately for each error type. It would not be very useful to
// receive two interrupts for the same error, so use only the first one.
//
    ret = rzt2h_icu_request_irqs(pdev, irq_domain, RZT2H_ICU_CA55_ERR_START, 1,
    rzt2h_icu_ca55_err_irq, priv);
    if (ret)
    return ret;
    ret = rzt2h_icu_request_irqs(pdev, irq_domain, RZT2H_ICU_PERI_ERR_START, 1,
    rzt2h_icu_peri_err_irq, priv);
    if (ret)
    return ret;
    ret = rzt2h_icu_request_irqs(pdev, irq_domain, RZT2H_ICU_DSMIF_ERR_START, 1,
    rzt2h_icu_dsmif_err_irq, priv);
    if (ret)
    return ret;
    ret = rzt2h_icu_request_irqs(pdev, irq_domain, RZT2H_ICU_ENCIF_ERR_START, 1,
    rzt2h_icu_encif_err_irq, priv);
    if (ret)
    return ret;
// Clear and unmask CA55 error events
    writel_relaxed(RZT2H_ICU_CA55ERR_MASK, priv.base_ns + RZT2H_ICU_CA55ERR_CLR);
    writel_relaxed(0, priv.base_ns + RZT2H_ICU_CA55ERR_E0MSK);
// Clear and unmask peripheral error events
    for (n = 0; n < RZT2H_ICU_PERIERR_NUM; n++) {
    writel_relaxed(RZT2H_ICU_PERIERR_MASK, priv.base_ns + RZT2H_ICU_PERIERR_CLRn(n));
    writel_relaxed(0, priv.base_ns + RZT2H_ICU_PERIERR_E0MSKn(n));
    }
// Clear and unmask DSMIF error events
    for (n = 0; n < RZT2H_ICU_DSMIFERR_NUM; n++) {
    writel_relaxed(RZT2H_ICU_DSMIFERR_MASK, priv.base_ns + RZT2H_ICU_DSMIFERR_CLRn(n));
    writel_relaxed(0, priv.base_ns + RZT2H_ICU_DSMIFERR_E0MSKn(n));
    }
// Clear and unmask ENCIF error events
    for (n = 0; n < RZT2H_ICU_ENCIFERR_NUM; n++) {
    writel_relaxed(RZT2H_ICU_ENCIFERR_MASK, priv.base_ns + RZT2H_ICU_ENCIFERR_CLRn(n));
    writel_relaxed(0, priv.base_ns + RZT2H_ICU_ENCIFERR_E0MSKn(n));
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzt2h_icu_init(pdev: *mut platform_device, parent: *mut device_node) -> c_int {
    static int rzt2h_icu_init(struct platform_device *pdev, struct device_node *parent)
    {
    struct irq_domain *irq_domain, *parent_domain;
    struct device_node *node = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    struct rzt2h_icu_priv *priv;
    int ret;
    parent_domain = irq_find_host(parent);
    if (!parent_domain)
    return dev_err_probe(dev, -ENODEV, "cannot find parent domain\n");
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    raw_spin_lock_init(&priv.lock);
    platform_set_drvdata(pdev, priv);
    priv.base_ns = devm_of_iomap(dev, dev.of_node, 0, core::ptr::null_mut());
    if (IS_ERR(priv.base_ns))
    return PTR_ERR(priv.base_ns);
    priv.base_s = devm_of_iomap(dev, dev.of_node, 1, core::ptr::null_mut());
    if (IS_ERR(priv.base_s))
    return PTR_ERR(priv.base_s);
    ret = rzt2h_icu_parse_interrupts(priv, node);
    if (ret)
    return dev_err_probe(dev, ret, "cannot parse interrupts: %d\n", ret);
    ret = devm_pm_runtime_enable(dev);
    if (ret)
    return dev_err_probe(dev, ret, "devm_pm_runtime_enable failed: %d\n", ret);
    ret = pm_runtime_resume_and_get(dev);
    if (ret)
    return dev_err_probe(dev, ret, "pm_runtime_resume_and_get failed: %d\n", ret);
    irq_domain = irq_domain_create_hierarchy(parent_domain, 0, RZT2H_ICU_NUM_IRQ,
    dev_fwnode(dev), &rzt2h_icu_domain_ops, priv);
    if (!irq_domain) {
    ret = -ENOMEM;
    goto err_pm_put;
    }
    ret = rzt2h_icu_setup_irqs(pdev, irq_domain);
    if (ret)
    goto err_irq_domain_free;
    return 0;
    err_irq_domain_free:
    irq_domain_remove(irq_domain);
    err_pm_put:
    pm_runtime_put_sync(dev);
    return ret;
    }
    IRQCHIP_PLATFORM_DRIVER_BEGIN(rzt2h_icu)
    IRQCHIP_MATCH("renesas,r9a09g077-icu", rzt2h_icu_init)
    IRQCHIP_PLATFORM_DRIVER_END(rzt2h_icu)
    MODULE_AUTHOR("Cosmin Tanislav <cosmin-gabriel.tanislav.xa@renesas.com>");
    MODULE_DESCRIPTION("Renesas RZ/T2H ICU Driver");
    MODULE_LICENSE("GPL");
