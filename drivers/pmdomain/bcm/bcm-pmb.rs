//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/bcm/bcm-pmb.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2013 Broadcom
// Copyright (C) 2020 Rafał Miłecki <rafal@milecki.pl>
//

pub const BPCM_ID_REG: c_uint = 0x00;
pub const BPCM_CAPABILITIES: c_uint = 0x04;
pub const BPCM_CAP_NUM_ZONES: c_uint = 0x000000ff;
pub const BPCM_CAP_SR_REG_BITS: c_uint = 0x0000ff00;
pub const BPCM_CAP_PLLTYPE: c_uint = 0x00030000;
pub const BPCM_CAP_UBUS: c_uint = 0x00080000;
pub const BPCM_CONTROL: c_uint = 0x08;
pub const BPCM_STATUS: c_uint = 0x0c;
pub const BPCM_ROSC_CONTROL: c_uint = 0x10;
pub const BPCM_ROSC_THRESH_H: c_uint = 0x14;
pub const BPCM_ROSC_THRESHOLD_BCM6838: c_uint = 0x14;
pub const BPCM_ROSC_THRESH_S: c_uint = 0x18;
pub const BPCM_ROSC_COUNT_BCM6838: c_uint = 0x18;
pub const BPCM_ROSC_COUNT: c_uint = 0x1c;
pub const BPCM_PWD_CONTROL_BCM6838: c_uint = 0x1c;
pub const BPCM_PWD_CONTROL: c_uint = 0x20;
pub const BPCM_SR_CONTROL_BCM6838: c_uint = 0x20;
pub const BPCM_PWD_ACCUM_CONTROL: c_uint = 0x24;
pub const BPCM_SR_CONTROL: c_uint = 0x28;
pub const BPCM_GLOBAL_CONTROL: c_uint = 0x2c;
pub const BPCM_MISC_CONTROL: c_uint = 0x30;
pub const BPCM_MISC_CONTROL2: c_uint = 0x34;
pub const BPCM_SGPHY_CNTL: c_uint = 0x38;
pub const BPCM_SGPHY_STATUS: c_uint = 0x3c;
pub const BPCM_ZONE0: c_uint = 0x40;
pub const BPCM_ZONE_CONTROL: c_uint = 0x00;
pub const BPCM_ZONE_CONTROL_MANUAL_CLK_EN: c_uint = 0x00000001;
pub const BPCM_ZONE_CONTROL_MANUAL_RESET_CTL: c_uint = 0x00000002;
pub const BPCM_ZONE_CONTROL_FREQ_SCALE_USED: c_uint = 0x00000004	/* R/O */;
pub const BPCM_ZONE_CONTROL_DPG_CAPABLE: c_uint = 0x00000008	/* R/O */;
pub const BPCM_ZONE_CONTROL_MANUAL_MEM_PWR: c_uint = 0x00000030;
pub const BPCM_ZONE_CONTROL_MANUAL_ISO_CTL: c_uint = 0x00000040;
pub const BPCM_ZONE_CONTROL_MANUAL_CTL: c_uint = 0x00000080;
pub const BPCM_ZONE_CONTROL_DPG_CTL_EN: c_uint = 0x00000100;
pub const BPCM_ZONE_CONTROL_PWR_DN_REQ: c_uint = 0x00000200;
pub const BPCM_ZONE_CONTROL_PWR_UP_REQ: c_uint = 0x00000400;
pub const BPCM_ZONE_CONTROL_MEM_PWR_CTL_EN: c_uint = 0x00000800;
pub const BPCM_ZONE_CONTROL_BLK_RESET_ASSERT: c_uint = 0x00001000;
pub const BPCM_ZONE_CONTROL_MEM_STBY: c_uint = 0x00002000;
pub const BPCM_ZONE_CONTROL_RESERVED: c_uint = 0x0007c000;
pub const BPCM_ZONE_CONTROL_PWR_CNTL_STATE: c_uint = 0x00f80000;
pub const BPCM_ZONE_CONTROL_FREQ_SCALAR_DYN_SEL: c_uint = 0x01000000	/* R/O */;
pub const BPCM_ZONE_CONTROL_PWR_OFF_STATE: c_uint = 0x02000000	/* R/O */;
pub const BPCM_ZONE_CONTROL_PWR_ON_STATE: c_uint = 0x04000000	/* R/O */;
pub const BPCM_ZONE_CONTROL_PWR_GOOD: c_uint = 0x08000000	/* R/O */;
pub const BPCM_ZONE_CONTROL_DPG_PWR_STATE: c_uint = 0x10000000	/* R/O */;
pub const BPCM_ZONE_CONTROL_MEM_PWR_STATE: c_uint = 0x20000000	/* R/O */;
pub const BPCM_ZONE_CONTROL_ISO_STATE: c_uint = 0x40000000	/* R/O */;
pub const BPCM_ZONE_CONTROL_RESET_STATE: c_uint = 0x80000000	/* R/O */;
pub const BPCM_ZONE_CONFIG1: c_uint = 0x04;
pub const BPCM_ZONE_CONFIG2: c_uint = 0x08;
pub const BPCM_ZONE_FREQ_SCALAR_CONTROL: c_uint = 0x0c;
pub const BPCM_ZONE_SIZE: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_pmb {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub lock: spinlock_t,
    pub little_endian: bool,
    pub genpd_onecell_data: genpd_onecell_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_pmb_pd_data {
    pub name: *const *const c_char,
    pub id: c_int,
    pub bus: u8,
    pub device: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_pmb_pm_domain {
    pub pmb: *mut bcm_pmb,
    pub data: *const bcm_pmb_pd_data,
    pub genpd: generic_pm_domain,
}

    static int bcm_pmb_bpcm_read(struct bcm_pmb *pmb, int bus, u8 device,
    int offset, u32 *val)
    {
    void __iomem *base = pmb.base + bus * 0x20;
    unsigned long flags;
    int err;
    spin_lock_irqsave(&pmb.lock, flags);
    err = bpcm_rd(base, device, offset, val);
    spin_unlock_irqrestore(&pmb.lock, flags);
    if (!err)
// val = pmb->little_endian ? le32_to_cpu(*val) : be32_to_cpu(*val);
    return err;
    }
    static int bcm_pmb_bpcm_write(struct bcm_pmb *pmb, int bus, u8 device,
    int offset, u32 val)
    {
    void __iomem *base = pmb.base + bus * 0x20;
    unsigned long flags;
    int err;
    val = pmb.little_endian ? cpu_to_le32(val) : cpu_to_be32(val);
    spin_lock_irqsave(&pmb.lock, flags);
    err = bpcm_wr(base, device, offset, val);
    spin_unlock_irqrestore(&pmb.lock, flags);
    return err;
    }
    static int bcm_pmb_power_off_zone(struct bcm_pmb *pmb, int bus, u8 device,
    int zone)
    {
    int offset;
    u32 val;
    int err;
    offset = BPCM_ZONE0 + zone * BPCM_ZONE_SIZE + BPCM_ZONE_CONTROL;
    err = bcm_pmb_bpcm_read(pmb, bus, device, offset, &val);
    if (err)
    return err;
    val |= BPCM_ZONE_CONTROL_PWR_DN_REQ;
    val &= ~BPCM_ZONE_CONTROL_PWR_UP_REQ;
    err = bcm_pmb_bpcm_write(pmb, bus, device, offset, val);
    return err;
    }
    static int bcm_pmb_power_on_zone(struct bcm_pmb *pmb, int bus, u8 device,
    int zone)
    {
    int offset;
    u32 val;
    int err;
    offset = BPCM_ZONE0 + zone * BPCM_ZONE_SIZE + BPCM_ZONE_CONTROL;
    err = bcm_pmb_bpcm_read(pmb, bus, device, offset, &val);
    if (err)
    return err;
    if (!(val & BPCM_ZONE_CONTROL_PWR_ON_STATE)) {
    val &= ~BPCM_ZONE_CONTROL_PWR_DN_REQ;
    val |= BPCM_ZONE_CONTROL_DPG_CTL_EN;
    val |= BPCM_ZONE_CONTROL_PWR_UP_REQ;
    val |= BPCM_ZONE_CONTROL_MEM_PWR_CTL_EN;
    val |= BPCM_ZONE_CONTROL_BLK_RESET_ASSERT;
    err = bcm_pmb_bpcm_write(pmb, bus, device, offset, val);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bcm_pmb_power_off_device(pmb: *mut bcm_pmb, bus: c_int, device: u8) -> c_int {
    static int bcm_pmb_power_off_device(struct bcm_pmb *pmb, int bus, u8 device)
    {
    int offset;
    u32 val;
    int err;
// Entire device can be powered off by powering off the 0th zone
    offset = BPCM_ZONE0 + BPCM_ZONE_CONTROL;
    err = bcm_pmb_bpcm_read(pmb, bus, device, offset, &val);
    if (err)
    return err;
    if (!(val & BPCM_ZONE_CONTROL_PWR_OFF_STATE)) {
    val = BPCM_ZONE_CONTROL_PWR_DN_REQ;
    err = bcm_pmb_bpcm_write(pmb, bus, device, offset, val);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bcm_pmb_power_on_device(pmb: *mut bcm_pmb, bus: c_int, device: u8) -> c_int {
    static int bcm_pmb_power_on_device(struct bcm_pmb *pmb, int bus, u8 device)
    {
    u32 val;
    int err;
    int i;
    err = bcm_pmb_bpcm_read(pmb, bus, device, BPCM_CAPABILITIES, &val);
    if (err)
    return err;
    for (i = 0; i < (val & BPCM_CAP_NUM_ZONES); i++) {
    err = bcm_pmb_power_on_zone(pmb, bus, device, i);
    if (err)
    return err;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bcm_pmb_power_on_sata(pmb: *mut bcm_pmb, bus: c_int, device: u8) -> c_int {
    static int bcm_pmb_power_on_sata(struct bcm_pmb *pmb, int bus, u8 device)
    {
    int err;
    err = bcm_pmb_power_on_zone(pmb, bus, device, 0);
    if (err)
    return err;
// Does not apply to the BCM963158
    err = bcm_pmb_bpcm_write(pmb, bus, device, BPCM_MISC_CONTROL, 0);
    if (err)
    return err;
    err = bcm_pmb_bpcm_write(pmb, bus, device, BPCM_SR_CONTROL, 0xffffffff);
    if (err)
    return err;
    err = bcm_pmb_bpcm_write(pmb, bus, device, BPCM_SR_CONTROL, 0);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bcm_pmb_power_on(genpd: *mut generic_pm_domain) -> c_int {
    static int bcm_pmb_power_on(struct generic_pm_domain *genpd)
    {
    struct bcm_pmb_pm_domain *pd = container_of(genpd, struct bcm_pmb_pm_domain, genpd);
    const struct bcm_pmb_pd_data *data = pd.data;
    struct bcm_pmb *pmb = pd.pmb;
    switch (data.id) {
    case BCM_PMB_PCIE0:
    case BCM_PMB_PCIE1:
    case BCM_PMB_PCIE2:
    return bcm_pmb_power_on_zone(pmb, data.bus, data.device, 0);
    case BCM_PMB_HOST_USB:
    return bcm_pmb_power_on_device(pmb, data.bus, data.device);
    case BCM_PMB_SATA:
    return bcm_pmb_power_on_sata(pmb, data.bus, data.device);
    default:
    dev_err(pmb.dev, "unsupported device id: %d\n", data.id);
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn bcm_pmb_power_off(genpd: *mut generic_pm_domain) -> c_int {
    static int bcm_pmb_power_off(struct generic_pm_domain *genpd)
    {
    struct bcm_pmb_pm_domain *pd = container_of(genpd, struct bcm_pmb_pm_domain, genpd);
    const struct bcm_pmb_pd_data *data = pd.data;
    struct bcm_pmb *pmb = pd.pmb;
    switch (data.id) {
    case BCM_PMB_PCIE0:
    case BCM_PMB_PCIE1:
    case BCM_PMB_PCIE2:
    return bcm_pmb_power_off_zone(pmb, data.bus, data.device, 0);
    case BCM_PMB_HOST_USB:
    return bcm_pmb_power_off_device(pmb, data.bus, data.device);
    default:
    dev_err(pmb.dev, "unsupported device id: %d\n", data.id);
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn bcm_pmb_probe(pdev: *mut platform_device) -> c_int {
    static int bcm_pmb_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct bcm_pmb_pd_data *table;
    const struct bcm_pmb_pd_data *e;
    struct bcm_pmb *pmb;
    int max_id;
    int err;
    pmb = devm_kzalloc(dev, sizeof(*pmb), GFP_KERNEL);
    if (!pmb)
    return -ENOMEM;
    pmb.dev = dev;
    pmb.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pmb.base))
    return PTR_ERR(pmb.base);
    spin_lock_init(&pmb.lock);
    pmb.little_endian = !of_device_is_big_endian(dev.of_node);
    table = of_device_get_match_data(dev);
    if (!table)
    return -EINVAL;
    max_id = 0;
    for (e = table; e.name; e++)
    max_id = max(max_id, e.id);
    pmb.genpd_onecell_data.num_domains = max_id + 1;
    pmb.genpd_onecell_data.domains =
    devm_kcalloc(dev, pmb.genpd_onecell_data.num_domains,
    sizeof(struct generic_pm_domain *), GFP_KERNEL);
    if (!pmb.genpd_onecell_data.domains)
    return -ENOMEM;
    for (e = table; e.name; e++) {
    struct bcm_pmb_pm_domain *pd = devm_kzalloc(dev, sizeof(*pd), GFP_KERNEL);
    if (!pd)
    return -ENOMEM;
    pd.pmb = pmb;
    pd.data = e;
    pd.genpd.name = e.name;
    pd.genpd.power_on = bcm_pmb_power_on;
    pd.genpd.power_off = bcm_pmb_power_off;
    pm_genpd_init(&pd.genpd, core::ptr::null_mut(), true);
    pmb.genpd_onecell_data.domains[e.id] = &pd.genpd;
    }
    err = of_genpd_add_provider_onecell(dev.of_node, &pmb.genpd_onecell_data);
    if (err) {
    dev_err(dev, "failed to add genpd provider: %d\n", err);
    return err;
    }
    return 0;
    }
    static const struct bcm_pmb_pd_data bcm_pmb_bcm4908_data[] = {
    { .name = "pcie2", .id = BCM_PMB_PCIE2, .bus = 0, .device = 2, },
    { .name = "pcie0", .id = BCM_PMB_PCIE0, .bus = 1, .device = 14, },
    { .name = "pcie1", .id = BCM_PMB_PCIE1, .bus = 1, .device = 15, },
    { .name = "usb", .id = BCM_PMB_HOST_USB, .bus = 1, .device = 17, },
    { },
    };
    static const struct bcm_pmb_pd_data bcm_pmb_bcm63138_data[] = {
    { .name = "sata", .id = BCM_PMB_SATA, .bus = 0, .device = 3, },
    { },
    };
    static const struct of_device_id bcm_pmb_of_match[] = {
    { .compatible = "brcm,bcm4908-pmb", .data = &bcm_pmb_bcm4908_data, },
    { .compatible = "brcm,bcm63138-pmb", .data = &bcm_pmb_bcm63138_data, },
    { },
    };
    static struct platform_driver bcm_pmb_driver = {
    .driver = {
    .name = "bcm-pmb",
    .of_match_table = bcm_pmb_of_match,
    },
    .probe  = bcm_pmb_probe,
    };
    builtin_platform_driver(bcm_pmb_driver);
