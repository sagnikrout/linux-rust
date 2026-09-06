//! Automatically rewritten from C to Rust
//! Source: drivers/ata/ahci_ceva.c
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
// Copyright (C) 2015 Xilinx, Inc.
// CEVA AHCI SATA platform driver
//
// based on the AHCI SATA platform driver by Jeff Garzik and Anton Vorontsov
//

// Vendor Specific Register Offsets
pub const AHCI_VEND_PCFG: c_uint = 0xA4;
pub const AHCI_VEND_PPCFG: c_uint = 0xA8;
pub const AHCI_VEND_PP2C: c_uint = 0xAC;
pub const AHCI_VEND_PP3C: c_uint = 0xB0;
pub const AHCI_VEND_PP4C: c_uint = 0xB4;
pub const AHCI_VEND_PP5C: c_uint = 0xB8;
pub const AHCI_VEND_AXICC: c_uint = 0xBC;
pub const AHCI_VEND_PAXIC: c_uint = 0xC0;
pub const AHCI_VEND_PTC: c_uint = 0xC8;
// Vendor Specific Register bit definitions
pub const PAXIC_ADBW_BW64: c_uint = 0x1;

// Register bit definitions for cache control

pub const PCFG_PAD_VAL: c_uint = 0x2;
pub const PPCFG_TTA: c_uint = 0x1FFFE;

pub const PP5C_RIT: c_uint = 0x60216;

pub const PTC_RX_WM_VAL: c_uint = 0x40;

pub const PORT0_BASE: c_uint = 0x100;
pub const PORT1_BASE: c_uint = 0x180;
// Port Control Register Bit Definitions

pub const PORT_BASE: c_uint = 0x100;
pub const PORT_OFFSET: c_uint = 0x80;
pub const NR_PORTS: c_int = 2;

pub const CEVA_FLAG_BROKEN_GEN2: c_int = 1;
    let mut rx_watermark: static unsigned int = PTC_RX_WM_VAL;
    module_param(rx_watermark, uint, 0644);
    MODULE_PARM_DESC(rx_watermark, "RxWaterMark value (0 - 0x80)");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceva_ahci_priv {
    pub ahci_pdev: *mut platform_device,
// Port Phy2Cfg Register
    pub pp2c: [u32; NR_PORTS],
    pub pp3c: [u32; NR_PORTS],
    pub pp4c: [u32; NR_PORTS],
    pub pp5c: [u32; NR_PORTS],
// Axi Cache Control Register
    pub axicc: u32,
    pub is_cci_enabled: bool,
    pub flags: c_int,
}

    static unsigned int ceva_ahci_read_id(struct ata_device *dev,
    struct ata_taskfile *tf, __le16 *id)
    {
    u32 err_mask;
    err_mask = ata_do_dev_read_id(dev, tf, id);
    if (err_mask)
    return err_mask;
//
// Since CEVA controller does not support device sleep feature, we
// need to clear DEVSLP (bit 8) in word78 of the IDENTIFY DEVICE data.
//
    id[ATA_ID_FEATURE_SUPP] &= cpu_to_le16(~(1 << 8));
    return 0;
    }
    static struct ata_port_operations ahci_ceva_ops = {
    .inherits = &ahci_platform_ops,
    .read_id = ceva_ahci_read_id,
    };
    static const struct ata_port_info ahci_ceva_port_info = {
    .flags          = AHCI_FLAG_COMMON,
    .pio_mask       = ATA_PIO4,
    .udma_mask      = ATA_UDMA6,
    .port_ops	= &ahci_ceva_ops,
    };
#[no_mangle]
unsafe extern "C" fn ahci_ceva_setup(hpriv: *mut ahci_host_priv) {
    static void ahci_ceva_setup(struct ahci_host_priv *hpriv)
    {
    void __iomem *mmio = hpriv.mmio;
    struct ceva_ahci_priv *cevapriv = hpriv.plat_data;
    u32 tmp;
    int i;
// Set AHCI Enable
    tmp = readl(mmio + HOST_CTL);
    tmp |= HOST_AHCI_EN;
    writel(tmp, mmio + HOST_CTL);
    for (i = 0; i < NR_PORTS; i++) {
// TPSS TPRS scalars, CISE and Port Addr
    tmp = PCFG_TPSS_VAL | PCFG_TPRS_VAL | (PCFG_PAD_VAL + i);
    writel(tmp, mmio + AHCI_VEND_PCFG);
//
// AXI Data bus width to 64
// Set Mem Addr Read, Write ID for data transfers
// Set Mem Addr Read ID, Write ID for non-data transfers
// Transfer limit to 72 DWord
//
    tmp = PAXIC_ADBW_BW64 | PAXIC_MAWIDD(i) | PAXIC_MARIDD(i) |
    PAXIC_MAWID(i) | PAXIC_MARID(i) | PAXIC_OTL;
    writel(tmp, mmio + AHCI_VEND_PAXIC);
// Set AXI cache control register if CCi is enabled
    if (cevapriv.is_cci_enabled) {
    tmp = readl(mmio + AHCI_VEND_AXICC);
    tmp |= AXICC_ARCA_VAL | AXICC_ARCF_VAL |
    AXICC_ARCH_VAL | AXICC_ARCP_VAL |
    AXICC_AWCFD_VAL | AXICC_AWCD_VAL |
    AXICC_AWCF_VAL;
    writel(tmp, mmio + AHCI_VEND_AXICC);
    }
// Port Phy Cfg register enables
    tmp = PPCFG_TTA | PPCFG_PSS_EN | PPCFG_ESDF_EN;
    writel(tmp, mmio + AHCI_VEND_PPCFG);
// Phy Control OOB timing parameters COMINIT
    writel(cevapriv.pp2c[i], mmio + AHCI_VEND_PP2C);
// Phy Control OOB timing parameters COMWAKE
    writel(cevapriv.pp3c[i], mmio + AHCI_VEND_PP3C);
// Phy Control Burst timing setting
    writel(cevapriv.pp4c[i], mmio + AHCI_VEND_PP4C);
// Rate Change Timer and Retry Interval Timer setting
    writel(cevapriv.pp5c[i], mmio + AHCI_VEND_PP5C);
// Rx Watermark setting
    tmp = rx_watermark | PTC_RSVD;
    writel(tmp, mmio + AHCI_VEND_PTC);
// Default to Gen 3 Speed and Gen 1 if Gen2 is broken
    tmp = PORT_SCTL_SPD_GEN3 | PORT_SCTL_IPM;
    if (cevapriv.flags & CEVA_FLAG_BROKEN_GEN2)
    tmp = PORT_SCTL_SPD_GEN1 | PORT_SCTL_IPM;
    writel(tmp, mmio + PORT_SCR_CTL + PORT_BASE + PORT_OFFSET * i);
    }
    }
    static const struct scsi_host_template ahci_platform_sht = {
    AHCI_SHT(DRV_NAME),
    };
#[no_mangle]
unsafe extern "C" fn ceva_ahci_platform_enable_resources(hpriv: *mut ahci_host_priv) -> c_int {
    static int ceva_ahci_platform_enable_resources(struct ahci_host_priv *hpriv)
    {
    int rc, i;
    rc = ahci_platform_enable_regulators(hpriv);
    if (rc)
    return rc;
    rc = ahci_platform_enable_clks(hpriv);
    if (rc)
    goto disable_regulator;
// Assert the controller reset
    rc = ahci_platform_assert_rsts(hpriv);
    if (rc)
    goto disable_clks;
    for (i = 0; i < hpriv.nports; i++) {
    if (ahci_ignore_port(hpriv, i))
    continue;
    rc = phy_init(hpriv.phys[i]);
    if (rc)
    goto exit_phys;
    }
// De-assert the controller reset
    ahci_platform_deassert_rsts(hpriv);
    for (i = 0; i < hpriv.nports; i++) {
    if (ahci_ignore_port(hpriv, i))
    continue;
    rc = phy_power_on(hpriv.phys[i]);
    if (rc) {
    phy_exit(hpriv.phys[i]);
    goto disable_phys;
    }
    }
    return 0;
    disable_phys:
    while (--i >= 0) {
    if (ahci_ignore_port(hpriv, i))
    continue;
    phy_power_off(hpriv.phys[i]);
    phy_exit(hpriv.phys[i]);
    }
    ahci_platform_assert_rsts(hpriv);
    goto disable_clks;
    exit_phys:
    while (--i >= 0) {
    if (ahci_ignore_port(hpriv, i))
    continue;
    phy_exit(hpriv.phys[i]);
    }
    disable_clks:
    ahci_platform_disable_clks(hpriv);
    disable_regulator:
    ahci_platform_disable_regulators(hpriv);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn ceva_ahci_probe(pdev: *mut platform_device) -> c_int {
    static int ceva_ahci_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    struct ahci_host_priv *hpriv;
    struct ceva_ahci_priv *cevapriv;
    enum dev_dma_attr attr;
    int rc;
    cevapriv = devm_kzalloc(dev, sizeof(*cevapriv), GFP_KERNEL);
    if (!cevapriv)
    return -ENOMEM;
    cevapriv.ahci_pdev = pdev;
    hpriv = ahci_platform_get_resources(pdev, 0);
    if (IS_ERR(hpriv))
    return PTR_ERR(hpriv);
    hpriv.rsts = devm_reset_control_get_optional_exclusive(&pdev.dev,
    core::ptr::null_mut());
    if (IS_ERR(hpriv.rsts))
    return dev_err_probe(&pdev.dev, PTR_ERR(hpriv.rsts),
    "failed to get reset\n");
    rc = ceva_ahci_platform_enable_resources(hpriv);
    if (rc)
    return rc;
    if (of_property_read_bool(np, "ceva,broken-gen2"))
    cevapriv.flags = CEVA_FLAG_BROKEN_GEN2;
// Read OOB timing value for COMINIT from device-tree
    if (of_property_read_u8_array(np, "ceva,p0-cominit-params",
    (u8 *)&cevapriv.pp2c[0], 4) < 0) {
    dev_warn(dev, "ceva,p0-cominit-params property not defined\n");
    rc = -EINVAL;
    goto disable_resources;
    }
    if (of_property_read_u8_array(np, "ceva,p1-cominit-params",
    (u8 *)&cevapriv.pp2c[1], 4) < 0) {
    dev_warn(dev, "ceva,p1-cominit-params property not defined\n");
    rc = -EINVAL;
    goto disable_resources;
    }
// Read OOB timing value for COMWAKE from device-tree
    if (of_property_read_u8_array(np, "ceva,p0-comwake-params",
    (u8 *)&cevapriv.pp3c[0], 4) < 0) {
    dev_warn(dev, "ceva,p0-comwake-params property not defined\n");
    rc = -EINVAL;
    goto disable_resources;
    }
    if (of_property_read_u8_array(np, "ceva,p1-comwake-params",
    (u8 *)&cevapriv.pp3c[1], 4) < 0) {
    dev_warn(dev, "ceva,p1-comwake-params property not defined\n");
    rc = -EINVAL;
    goto disable_resources;
    }
// Read phy BURST timing value from device-tree
    if (of_property_read_u8_array(np, "ceva,p0-burst-params",
    (u8 *)&cevapriv.pp4c[0], 4) < 0) {
    dev_warn(dev, "ceva,p0-burst-params property not defined\n");
    rc = -EINVAL;
    goto disable_resources;
    }
    if (of_property_read_u8_array(np, "ceva,p1-burst-params",
    (u8 *)&cevapriv.pp4c[1], 4) < 0) {
    dev_warn(dev, "ceva,p1-burst-params property not defined\n");
    rc = -EINVAL;
    goto disable_resources;
    }
// Read phy RETRY interval timing value from device-tree
    if (of_property_read_u16_array(np, "ceva,p0-retry-params",
    (u16 *)&cevapriv.pp5c[0], 2) < 0) {
    dev_warn(dev, "ceva,p0-retry-params property not defined\n");
    rc = -EINVAL;
    goto disable_resources;
    }
    if (of_property_read_u16_array(np, "ceva,p1-retry-params",
    (u16 *)&cevapriv.pp5c[1], 2) < 0) {
    dev_warn(dev, "ceva,p1-retry-params property not defined\n");
    rc = -EINVAL;
    goto disable_resources;
    }
//
// Check if CCI is enabled for SATA. The DEV_DMA_COHERENT is returned
// if CCI is enabled, so check for DEV_DMA_COHERENT.
//
    attr = device_get_dma_attr(dev);
    cevapriv.is_cci_enabled = (attr == DEV_DMA_COHERENT);
    hpriv.plat_data = cevapriv;
// CEVA specific initialization
    ahci_ceva_setup(hpriv);
    rc = ahci_platform_init_host(pdev, hpriv, &ahci_ceva_port_info,
    &ahci_platform_sht);
    if (rc)
    goto disable_resources;
    return 0;
    disable_resources:
    ahci_platform_disable_resources(hpriv);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn ceva_ahci_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ceva_ahci_suspend(struct device *dev)
    {
    return ahci_platform_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn ceva_ahci_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ceva_ahci_resume(struct device *dev)
    {
    struct ata_host *host = dev_get_drvdata(dev);
    struct ahci_host_priv *hpriv = host.private_data;
    int rc;
    rc = ceva_ahci_platform_enable_resources(hpriv);
    if (rc)
    return rc;
// Configure CEVA specific config before resuming HBA
    ahci_ceva_setup(hpriv);
    rc = ahci_platform_resume_host(dev);
    if (rc)
    goto disable_resources;
// We resumed so update PM runtime state
    pm_runtime_disable(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    return 0;
    disable_resources:
    ahci_platform_disable_resources(hpriv);
    return rc;
    }
    static SIMPLE_DEV_PM_OPS(ahci_ceva_pm_ops, ceva_ahci_suspend, ceva_ahci_resume);
    static const struct of_device_id ceva_ahci_of_match[] = {
    { .compatible = "ceva,ahci-1v84" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ceva_ahci_of_match);
    static struct platform_driver ceva_ahci_driver = {
    .probe = ceva_ahci_probe,
    .remove = ata_platform_remove_one,
    .driver = {
    .name = DRV_NAME,
    .of_match_table = ceva_ahci_of_match,
    .pm = &ahci_ceva_pm_ops,
    },
    };
    module_platform_driver(ceva_ahci_driver);
    MODULE_DESCRIPTION("CEVA AHCI SATA platform driver");
    MODULE_AUTHOR("Xilinx Inc.");
    MODULE_LICENSE("GPL v2");
