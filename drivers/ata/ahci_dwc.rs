//! Automatically rewritten from C to Rust
//! Source: drivers/ata/ahci_dwc.c
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
// DWC AHCI SATA Platform driver
//
// Copyright (C) 2021 BAIKAL ELECTRONICS, JSC
//

pub const AHCI_DWC_FBS_PMPN_MAX: c_int = 15;
// DWC AHCI SATA controller specific registers
pub const AHCI_DWC_HOST_OOBR: c_uint = 0xbc;

pub const AHCI_DWC_HOST_GPCR: c_uint = 0xd0;
pub const AHCI_DWC_HOST_GPSR: c_uint = 0xd4;
pub const AHCI_DWC_HOST_TIMER1MS: c_uint = 0xe0;

pub const AHCI_DWC_HOST_GPARAM1R: c_uint = 0xe8;

pub const AHCI_DWC_HOST_GPARAM2R: c_uint = 0xec;

pub const AHCI_DWC_HOST_PPARAMR: c_uint = 0xf0;

pub const AHCI_DWC_HOST_TESTR: c_uint = 0xf4;

pub const AHCI_DWC_HOST_VERSIONR: c_uint = 0xf8;
pub const AHCI_DWC_HOST_IDR: c_uint = 0xfc;
pub const AHCI_DWC_PORT_DMACR: c_uint = 0x70;

pub const AHCI_DWC_PORT_PHYCR: c_uint = 0x74;
pub const AHCI_DWC_PORT_PHYSR: c_uint = 0x78;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahci_dwc_plat_data {
    pub pflags: c_uint,
    pub hflags: c_uint,
    pub hpriv): *mut *mut int (init)(struct ahci_host_priv,
    pub hpriv): *mut *mut int (reinit)(struct ahci_host_priv,
    pub hpriv): *mut *mut void (clear)(struct ahci_host_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahci_dwc_host_priv {
    pub pdata: *const ahci_dwc_plat_data,
    pub pdev: *mut platform_device,
    pub timv: u32,
    pub dmacr: [u32; AHCI_MAX_PORTS],
}

    static struct ahci_host_priv *ahci_dwc_get_resources(struct platform_device *pdev)
    {
    struct ahci_dwc_host_priv *dpriv;
    struct ahci_host_priv *hpriv;
    dpriv = devm_kzalloc(&pdev.dev, sizeof(*dpriv), GFP_KERNEL);
    if (!dpriv)
    return ERR_PTR(-ENOMEM);
    dpriv.pdev = pdev;
    dpriv.pdata = device_get_match_data(&pdev.dev);
    if (!dpriv.pdata)
    return ERR_PTR(-EINVAL);
    hpriv = ahci_platform_get_resources(pdev, dpriv.pdata.pflags);
    if (IS_ERR(hpriv))
    return hpriv;
    hpriv.flags |= dpriv.pdata.hflags;
    hpriv.plat_data = (void *)dpriv;
    return hpriv;
    }
#[no_mangle]
unsafe extern "C" fn ahci_dwc_check_cap(hpriv: *mut ahci_host_priv) {
    static void ahci_dwc_check_cap(struct ahci_host_priv *hpriv)
    {
    let mut port_map: c_ulong = hpriv.saved_port_map | hpriv.mask_port_map;
    struct ahci_dwc_host_priv *dpriv = hpriv.plat_data;
    bool dev_mp, dev_cp, fbs_sup;
    unsigned int fbs_pmp;
    u32 param;
    int i;
    param = readl(hpriv.mmio + AHCI_DWC_HOST_GPARAM2R);
    dev_mp = !!(param & AHCI_DWC_HOST_DEV_MP);
    dev_cp = !!(param & AHCI_DWC_HOST_DEV_CP);
    fbs_sup = !!(param & AHCI_DWC_HOST_FBS_SUP);
    fbs_pmp = 5 * FIELD_GET(AHCI_DWC_HOST_FBS_PMPN_MASK, param);
    if (!dev_mp && hpriv.saved_cap & HOST_CAP_MPS) {
    dev_warn(&dpriv.pdev.dev, "MPS is unsupported\n");
    hpriv.saved_cap &= ~HOST_CAP_MPS;
    }
    if (fbs_sup && fbs_pmp < AHCI_DWC_FBS_PMPN_MAX) {
    dev_warn(&dpriv.pdev.dev, "PMPn is limited up to %u ports\n",
    fbs_pmp);
    }
    for_each_set_bit(i, &port_map, AHCI_MAX_PORTS) {
    if (!dev_mp && hpriv.saved_port_cap[i] & PORT_CMD_MPSP) {
    dev_warn(&dpriv.pdev.dev, "MPS incapable port %d\n", i);
    hpriv.saved_port_cap[i] &= ~PORT_CMD_MPSP;
    }
    if (!dev_cp && hpriv.saved_port_cap[i] & PORT_CMD_CPD) {
    dev_warn(&dpriv.pdev.dev, "CPD incapable port %d\n", i);
    hpriv.saved_port_cap[i] &= ~PORT_CMD_CPD;
    }
    if (!fbs_sup && hpriv.saved_port_cap[i] & PORT_CMD_FBSCP) {
    dev_warn(&dpriv.pdev.dev, "FBS incapable port %d\n", i);
    hpriv.saved_port_cap[i] &= ~PORT_CMD_FBSCP;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn ahci_dwc_init_timer(hpriv: *mut ahci_host_priv) {
    static void ahci_dwc_init_timer(struct ahci_host_priv *hpriv)
    {
    struct ahci_dwc_host_priv *dpriv = hpriv.plat_data;
    unsigned long rate;
    struct clk *aclk;
    u32 cap, cap2;
// 1ms tick is generated only for the CCC or DevSleep features
    cap = readl(hpriv.mmio + HOST_CAP);
    cap2 = readl(hpriv.mmio + HOST_CAP2);
    if (!(cap & HOST_CAP_CCC) && !(cap2 & HOST_CAP2_SDS))
    return;
//
// Tick is generated based on the AXI/AHB application clocks signal
// so we need to be sure in the clock we are going to use.
//
    aclk = ahci_platform_find_clk(hpriv, "aclk");
    if (!aclk)
    return;
// 1ms timer interval is set as TIMV = AMBA_FREQ[MHZ] * 1000
    dpriv.timv = readl(hpriv.mmio + AHCI_DWC_HOST_TIMER1MS);
    dpriv.timv = FIELD_GET(AHCI_DWC_HOST_TIMV_MASK, dpriv.timv);
    rate = clk_get_rate(aclk) / 1000UL;
    if (rate == dpriv.timv)
    return;
    dev_info(&dpriv.pdev.dev, "Update CCC/DevSlp timer for Fapp %lu MHz\n",
    rate / 1000UL);
    dpriv.timv = FIELD_PREP(AHCI_DWC_HOST_TIMV_MASK, rate);
    writel(dpriv.timv, hpriv.mmio + AHCI_DWC_HOST_TIMER1MS);
    }
#[no_mangle]
unsafe extern "C" fn ahci_dwc_init_dmacr(hpriv: *mut ahci_host_priv) -> c_int {
    static int ahci_dwc_init_dmacr(struct ahci_host_priv *hpriv)
    {
    struct ahci_dwc_host_priv *dpriv = hpriv.plat_data;
    void __iomem *port_mmio;
    u32 port, dmacr, ts;
//
// Update the DMA Tx/Rx transaction sizes in accordance with the
// platform setup. Note values exceeding maximal or minimal limits will
// be automatically clamped. Also note the register isn't affected by
// the HBA global reset so we can freely initialize it once until the
// next system reset.
//
    for_each_available_child_of_node_scoped(dpriv.pdev.dev.of_node, child) {
    if (of_property_read_u32(child, "reg", &port))
    return -EINVAL;
    port_mmio = __ahci_port_base(hpriv, port);
    dmacr = readl(port_mmio + AHCI_DWC_PORT_DMACR);
    if (!of_property_read_u32(child, "snps,tx-ts-max", &ts)) {
    ts = ilog2(ts);
    dmacr &= ~AHCI_DWC_PORT_TXTS_MASK;
    dmacr |= FIELD_PREP(AHCI_DWC_PORT_TXTS_MASK, ts);
    }
    if (!of_property_read_u32(child, "snps,rx-ts-max", &ts)) {
    ts = ilog2(ts);
    dmacr &= ~AHCI_DWC_PORT_RXTS_MASK;
    dmacr |= FIELD_PREP(AHCI_DWC_PORT_RXTS_MASK, ts);
    }
    writel(dmacr, port_mmio + AHCI_DWC_PORT_DMACR);
    dpriv.dmacr[port] = dmacr;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ahci_dwc_init_host(hpriv: *mut ahci_host_priv) -> c_int {
    static int ahci_dwc_init_host(struct ahci_host_priv *hpriv)
    {
    struct ahci_dwc_host_priv *dpriv = hpriv.plat_data;
    int rc;
    rc = ahci_platform_enable_resources(hpriv);
    if (rc)
    return rc;
    if (dpriv.pdata.init) {
    rc = dpriv.pdata.init(hpriv);
    if (rc)
    goto err_disable_resources;
    }
    ahci_dwc_check_cap(hpriv);
    ahci_dwc_init_timer(hpriv);
    rc = ahci_dwc_init_dmacr(hpriv);
    if (rc)
    goto err_clear_platform;
    return 0;
    err_clear_platform:
    if (dpriv.pdata.clear)
    dpriv.pdata.clear(hpriv);
    err_disable_resources:
    ahci_platform_disable_resources(hpriv);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn ahci_dwc_reinit_host(hpriv: *mut ahci_host_priv) -> c_int {
    static int ahci_dwc_reinit_host(struct ahci_host_priv *hpriv)
    {
    struct ahci_dwc_host_priv *dpriv = hpriv.plat_data;
    let mut port_map: c_ulong = hpriv.port_map;
    void __iomem *port_mmio;
    int i, rc;
    rc = ahci_platform_enable_resources(hpriv);
    if (rc)
    return rc;
    if (dpriv.pdata.reinit) {
    rc = dpriv.pdata.reinit(hpriv);
    if (rc)
    goto err_disable_resources;
    }
    writel(dpriv.timv, hpriv.mmio + AHCI_DWC_HOST_TIMER1MS);
    for_each_set_bit(i, &port_map, AHCI_MAX_PORTS) {
    port_mmio = __ahci_port_base(hpriv, i);
    writel(dpriv.dmacr[i], port_mmio + AHCI_DWC_PORT_DMACR);
    }
    return 0;
    err_disable_resources:
    ahci_platform_disable_resources(hpriv);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn ahci_dwc_clear_host(hpriv: *mut ahci_host_priv) {
    static void ahci_dwc_clear_host(struct ahci_host_priv *hpriv)
    {
    struct ahci_dwc_host_priv *dpriv = hpriv.plat_data;
    if (dpriv.pdata.clear)
    dpriv.pdata.clear(hpriv);
    ahci_platform_disable_resources(hpriv);
    }
#[no_mangle]
unsafe extern "C" fn ahci_dwc_stop_host(host: *mut ata_host) {
    static void ahci_dwc_stop_host(struct ata_host *host)
    {
    struct ahci_host_priv *hpriv = host.private_data;
    ahci_dwc_clear_host(hpriv);
    }
    static struct ata_port_operations ahci_dwc_port_ops = {
    .inherits	= &ahci_platform_ops,
    .host_stop	= ahci_dwc_stop_host,
    };
    static const struct ata_port_info ahci_dwc_port_info = {
    .flags		= AHCI_FLAG_COMMON,
    .pio_mask	= ATA_PIO4,
    .udma_mask	= ATA_UDMA6,
    .port_ops	= &ahci_dwc_port_ops,
    };
    static const struct scsi_host_template ahci_dwc_scsi_info = {
    AHCI_SHT(DRV_NAME),
    };
#[no_mangle]
unsafe extern "C" fn ahci_dwc_probe(pdev: *mut platform_device) -> c_int {
    static int ahci_dwc_probe(struct platform_device *pdev)
    {
    struct ahci_host_priv *hpriv;
    int rc;
    hpriv = ahci_dwc_get_resources(pdev);
    if (IS_ERR(hpriv))
    return PTR_ERR(hpriv);
    rc = ahci_dwc_init_host(hpriv);
    if (rc)
    return rc;
    rc = ahci_platform_init_host(pdev, hpriv, &ahci_dwc_port_info,
    &ahci_dwc_scsi_info);
    if (rc)
    goto err_clear_host;
    return 0;
    err_clear_host:
    ahci_dwc_clear_host(hpriv);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn ahci_dwc_suspend(dev: *mut device) -> c_int {
    static int ahci_dwc_suspend(struct device *dev)
    {
    struct ata_host *host = dev_get_drvdata(dev);
    struct ahci_host_priv *hpriv = host.private_data;
    int rc;
    rc = ahci_platform_suspend_host(dev);
    if (rc)
    return rc;
    ahci_dwc_clear_host(hpriv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ahci_dwc_resume(dev: *mut device) -> c_int {
    static int ahci_dwc_resume(struct device *dev)
    {
    struct ata_host *host = dev_get_drvdata(dev);
    struct ahci_host_priv *hpriv = host.private_data;
    int rc;
    rc = ahci_dwc_reinit_host(hpriv);
    if (rc)
    return rc;
    return ahci_platform_resume_host(dev);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(ahci_dwc_pm_ops, ahci_dwc_suspend,
    ahci_dwc_resume);
    static struct ahci_dwc_plat_data ahci_dwc_plat = {
    .pflags = AHCI_PLATFORM_GET_RESETS,
    };
    static const struct of_device_id ahci_dwc_of_match[] = {
    { .compatible = "snps,dwc-ahci", &ahci_dwc_plat },
    { .compatible = "snps,spear-ahci", &ahci_dwc_plat },
    {},
    };
    MODULE_DEVICE_TABLE(of, ahci_dwc_of_match);
    static struct platform_driver ahci_dwc_driver = {
    .probe = ahci_dwc_probe,
    .remove = ata_platform_remove_one,
    .shutdown = ahci_platform_shutdown,
    .driver = {
    .name = DRV_NAME,
    .of_match_table = ahci_dwc_of_match,
    .pm = &ahci_dwc_pm_ops,
    },
    };
    module_platform_driver(ahci_dwc_driver);
    MODULE_DESCRIPTION("DWC AHCI SATA platform driver");
    MODULE_AUTHOR("Serge Semin <Sergey.Semin@baikalelectronics.ru>");
    MODULE_LICENSE("GPL");
