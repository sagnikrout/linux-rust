//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-mvebu-icu.c
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
// Copyright (C) 2017 Marvell
//
// Hanna Hawa <hannah@marvell.com>
// Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

// ICU registers
pub const ICU_SETSPI_NSR_AL: c_uint = 0x10;
pub const ICU_SETSPI_NSR_AH: c_uint = 0x14;
pub const ICU_CLRSPI_NSR_AL: c_uint = 0x18;
pub const ICU_CLRSPI_NSR_AH: c_uint = 0x1c;
pub const ICU_SET_SEI_AL: c_uint = 0x50;
pub const ICU_SET_SEI_AH: c_uint = 0x54;
pub const ICU_CLR_SEI_AL: c_uint = 0x58;
pub const ICU_CLR_SEI_AH: c_uint = 0x5C;

pub const ICU_GROUP_SHIFT: c_int = 29;
// ICU definitions
pub const ICU_MAX_IRQS: c_int = 207;
pub const ICU_SATA0_ICU_ID: c_int = 109;
pub const ICU_SATA1_ICU_ID: c_int = 107;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvebu_icu_subset_data {
    pub icu_group: c_uint,
    pub offset_set_ah: c_uint,
    pub offset_set_al: c_uint,
    pub offset_clr_ah: c_uint,
    pub offset_clr_al: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvebu_icu {
    pub base: *mut void __iomem,
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvebu_icu_msi_data {
    pub icu: *mut mvebu_icu,
    pub initialized: core::sync::atomic::AtomicI32,
    pub subset_data: *const mvebu_icu_subset_data,
}

    static DEFINE_STATIC_KEY_FALSE(legacy_bindings);
    static int mvebu_icu_translate(struct irq_domain *d, struct irq_fwspec *fwspec,
    unsigned long *hwirq, unsigned int *type)
    {
    let mut param_count: c_uint = static_branch_unlikely(&legacy_bindings) ? 3 : 2;
    struct msi_domain_info *info = d.host_data;
    struct mvebu_icu_msi_data *msi_data = info.chip_data;
    struct mvebu_icu *icu = msi_data.icu;
// Check the count of the parameters in dt
    if (WARN_ON(fwspec.param_count != param_count)) {
    dev_err(icu.dev, "wrong ICU parameter count %d\n",
    fwspec.param_count);
    return -EINVAL;
    }
    if (static_branch_unlikely(&legacy_bindings)) {
// hwirq = fwspec->param[1];
// type = fwspec->param[2] & IRQ_TYPE_SENSE_MASK;
    if (fwspec.param[0] != ICU_GRP_NSR) {
    dev_err(icu.dev, "wrong ICU group type %x\n",
    fwspec.param[0]);
    return -EINVAL;
    }
    } else {
// hwirq = fwspec->param[0];
// type = fwspec->param[1] & IRQ_TYPE_SENSE_MASK;
//
// The ICU receives level interrupts. While the NSR are also
// level interrupts, SEI are edge interrupts. Force the type
// here in this case. Please note that this makes the interrupt
// handling unreliable.
//
    if (msi_data.subset_data.icu_group == ICU_GRP_SEI)
// type = IRQ_TYPE_EDGE_RISING;
    }
    if (*hwirq >= ICU_MAX_IRQS) {
    dev_err(icu.dev, "invalid interrupt number %ld\n", *hwirq);
    return -EINVAL;
    }
    return 0;
    }
    static void mvebu_icu_init(struct mvebu_icu *icu,
    struct mvebu_icu_msi_data *msi_data,
    struct msi_msg *msg)
    {
    const struct mvebu_icu_subset_data *subset = msi_data.subset_data;
    if (atomic_cmpxchg(&msi_data.initialized, false, true))
    return;
// Set 'SET' ICU SPI message address in AP
    writel_relaxed(msg[0].address_hi, icu.base + subset.offset_set_ah);
    writel_relaxed(msg[0].address_lo, icu.base + subset.offset_set_al);
    if (subset.icu_group != ICU_GRP_NSR)
    return;
// Set 'CLEAR' ICU SPI message address in AP (level-MSI only)
    writel_relaxed(msg[1].address_hi, icu.base + subset.offset_clr_ah);
    writel_relaxed(msg[1].address_lo, icu.base + subset.offset_clr_al);
    }
    static int mvebu_icu_msi_init(struct irq_domain *domain, struct msi_domain_info *info,
    unsigned int virq, irq_hw_number_t hwirq, msi_alloc_info_t *arg)
    {
    irq_domain_set_hwirq_and_chip(domain, virq, hwirq, info.chip, info.chip_data);
    return irq_set_irqchip_state(virq, IRQCHIP_STATE_PENDING, false);
    }
#[no_mangle]
unsafe extern "C" fn mvebu_icu_set_desc(arg: *mut msi_alloc_info_t, desc: *mut msi_desc) {
    static void mvebu_icu_set_desc(msi_alloc_info_t *arg, struct msi_desc *desc)
    {
    arg.desc = desc;
    arg.hwirq = (u32)desc.data.icookie.value;
    }
#[no_mangle]
unsafe extern "C" fn mvebu_icu_write_msi_msg(d: *mut irq_data, msg: *mut msi_msg) {
    static void mvebu_icu_write_msi_msg(struct irq_data *d, struct msi_msg *msg)
    {
    struct mvebu_icu_msi_data *msi_data = d.chip_data;
    let mut icu_group: c_uint = msi_data.subset_data.icu_group;
    struct msi_desc *desc = irq_data_get_msi_desc(d);
    struct mvebu_icu *icu = msi_data.icu;
    unsigned int type;
    u32 icu_int;
    if (msg.address_lo || msg.address_hi) {
// One off initialization per domain
    mvebu_icu_init(icu, msi_data, msg);
// Configure the ICU with irq number & type
    icu_int = msg.data | ICU_INT_ENABLE;
    type = (unsigned int)(desc.data.icookie.value >> 32);
    if (type & IRQ_TYPE_EDGE_RISING)
    icu_int |= ICU_IS_EDGE;
    icu_int |= icu_group << ICU_GROUP_SHIFT;
    } else {
// De-configure the ICU
    icu_int = 0;
    }
    writel_relaxed(icu_int, icu.base + ICU_INT_CFG(d.hwirq));
//
// The SATA unit has 2 ports, and a dedicated ICU entry per
// port. The ahci sata driver supports only one irq interrupt
// per SATA unit. To solve this conflict, we configure the 2
// SATA wired interrupts in the south bridge into 1 GIC
// interrupt in the north bridge. Even if only a single port
// is enabled, if sata node is enabled, both interrupts are
// configured (regardless of which port is actually in use).
//
    if (d.hwirq == ICU_SATA0_ICU_ID || d.hwirq == ICU_SATA1_ICU_ID) {
    writel_relaxed(icu_int, icu.base + ICU_INT_CFG(ICU_SATA0_ICU_ID));
    writel_relaxed(icu_int, icu.base + ICU_INT_CFG(ICU_SATA1_ICU_ID));
    }
    }
    static const struct msi_domain_template mvebu_icu_nsr_msi_template = {
    .chip = {
    .name			= "ICU-NSR",
    .irq_mask		= irq_chip_mask_parent,
    .irq_unmask		= irq_chip_unmask_parent,
    .irq_eoi		= irq_chip_eoi_parent,
    .irq_set_type		= irq_chip_set_type_parent,
    .irq_write_msi_msg	= mvebu_icu_write_msi_msg,
    .flags			= IRQCHIP_SUPPORTS_LEVEL_MSI,
    },
    .ops = {
    .msi_translate		= mvebu_icu_translate,
    .msi_init		= mvebu_icu_msi_init,
    .set_desc		= mvebu_icu_set_desc,
    },
    .info = {
    .bus_token		= DOMAIN_BUS_WIRED_TO_MSI,
    .flags			= MSI_FLAG_LEVEL_CAPABLE |
    MSI_FLAG_USE_DEV_FWNODE,
    },
    };
    static const struct msi_domain_template mvebu_icu_sei_msi_template = {
    .chip = {
    .name			= "ICU-SEI",
    .irq_mask		= irq_chip_mask_parent,
    .irq_unmask		= irq_chip_unmask_parent,
    .irq_ack		= irq_chip_ack_parent,
    .irq_set_type		= irq_chip_set_type_parent,
    .irq_write_msi_msg	= mvebu_icu_write_msi_msg,
    .flags			= IRQCHIP_SUPPORTS_LEVEL_MSI,
    },
    .ops = {
    .msi_translate		= mvebu_icu_translate,
    .msi_init		= mvebu_icu_msi_init,
    .set_desc		= mvebu_icu_set_desc,
    },
    .info = {
    .bus_token		= DOMAIN_BUS_WIRED_TO_MSI,
    .flags			= MSI_FLAG_LEVEL_CAPABLE |
    MSI_FLAG_USE_DEV_FWNODE,
    },
    };
    static const struct mvebu_icu_subset_data mvebu_icu_nsr_subset_data = {
    .icu_group = ICU_GRP_NSR,
    .offset_set_ah = ICU_SETSPI_NSR_AH,
    .offset_set_al = ICU_SETSPI_NSR_AL,
    .offset_clr_ah = ICU_CLRSPI_NSR_AH,
    .offset_clr_al = ICU_CLRSPI_NSR_AL,
    };
    static const struct mvebu_icu_subset_data mvebu_icu_sei_subset_data = {
    .icu_group = ICU_GRP_SEI,
    .offset_set_ah = ICU_SET_SEI_AH,
    .offset_set_al = ICU_SET_SEI_AL,
    };
    static const struct of_device_id mvebu_icu_subset_of_match[] = {
    {
    .compatible = "marvell,cp110-icu-nsr",
    .data = &mvebu_icu_nsr_subset_data,
    },
    {
    .compatible = "marvell,cp110-icu-sei",
    .data = &mvebu_icu_sei_subset_data,
    },
    {},
    };
#[no_mangle]
unsafe extern "C" fn mvebu_icu_subset_probe(pdev: *mut platform_device) -> c_int {
    static int mvebu_icu_subset_probe(struct platform_device *pdev)
    {
    const struct msi_domain_template *tmpl;
    struct mvebu_icu_msi_data *msi_data;
    struct device *dev = &pdev.dev;
    bool sei;
    msi_data = devm_kzalloc(dev, sizeof(*msi_data), GFP_KERNEL);
    if (!msi_data)
    return -ENOMEM;
    if (static_branch_unlikely(&legacy_bindings)) {
    msi_data.icu = dev_get_drvdata(dev);
    msi_data.subset_data = &mvebu_icu_nsr_subset_data;
    } else {
    msi_data.icu = dev_get_drvdata(dev.parent);
    msi_data.subset_data = of_device_get_match_data(dev);
    }
    dev.msi.domain = of_msi_get_domain(dev, dev.of_node, DOMAIN_BUS_PLATFORM_MSI);
    if (!dev.msi.domain)
    return -EPROBE_DEFER;
    if (!irq_domain_get_of_node(dev.msi.domain))
    return -ENODEV;
    sei = msi_data.subset_data.icu_group == ICU_GRP_SEI;
    tmpl = sei ? &mvebu_icu_sei_msi_template : &mvebu_icu_nsr_msi_template;
    if (!msi_create_device_irq_domain(dev, MSI_DEFAULT_DOMAIN, tmpl,
    ICU_MAX_IRQS, core::ptr::null_mut(), msi_data)) {
    dev_err(dev, "Failed to create ICU MSI domain\n");
    return -ENOMEM;
    }
    return 0;
    }
    static struct platform_driver mvebu_icu_subset_driver = {
    .probe  = mvebu_icu_subset_probe,
    .driver = {
    .name = "mvebu-icu-subset",
    .of_match_table = mvebu_icu_subset_of_match,
    },
    };
    builtin_platform_driver(mvebu_icu_subset_driver);
#[no_mangle]
unsafe extern "C" fn mvebu_icu_probe(pdev: *mut platform_device) -> c_int {
    static int mvebu_icu_probe(struct platform_device *pdev)
    {
    struct mvebu_icu *icu;
    int i;
    icu = devm_kzalloc(&pdev.dev, sizeof(struct mvebu_icu),
    GFP_KERNEL);
    if (!icu)
    return -ENOMEM;
    icu.dev = &pdev.dev;
    icu.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(icu.base))
    return PTR_ERR(icu.base);
//
// Legacy bindings: ICU is one node with one MSI parent: force manually
// the probe of the NSR interrupts side.
// New bindings: ICU node has children, one per interrupt controller
// having its own MSI parent: call platform_populate().
// All ICU instances should use the same bindings.
//
    if (!of_get_child_count(pdev.dev.of_node))
    static_branch_enable(&legacy_bindings);
//
// Clean all ICU interrupts of type NSR and SEI, required to
// avoid unpredictable SPI assignments done by firmware.
//
    for (i = 0 ; i < ICU_MAX_IRQS ; i++) {
    u32 icu_int, icu_grp;
    icu_int = readl_relaxed(icu.base + ICU_INT_CFG(i));
    icu_grp = icu_int >> ICU_GROUP_SHIFT;
    if (icu_grp == ICU_GRP_NSR ||
    (icu_grp == ICU_GRP_SEI &&
    !static_branch_unlikely(&legacy_bindings)))
    writel_relaxed(0x0, icu.base + ICU_INT_CFG(i));
    }
    platform_set_drvdata(pdev, icu);
    if (static_branch_unlikely(&legacy_bindings))
    return mvebu_icu_subset_probe(pdev);
    else
    return devm_of_platform_populate(&pdev.dev);
    }
    static const struct of_device_id mvebu_icu_of_match[] = {
    { .compatible = "marvell,cp110-icu", },
    {},
    };
    static struct platform_driver mvebu_icu_driver = {
    .probe  = mvebu_icu_probe,
    .driver = {
    .name = "mvebu-icu",
    .of_match_table = mvebu_icu_of_match,
    },
    };
    builtin_platform_driver(mvebu_icu_driver);
