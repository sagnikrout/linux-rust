//! Automatically rewritten from C to Rust
//! Source: drivers/ata/ahci_mtk.c
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
// MediaTek AHCI SATA driver
//
// Copyright (c) 2017 MediaTek Inc.
// Author: Ryder Lee <ryder.lee@mediatek.com>
//

pub const SYS_CFG: c_uint = 0x14;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_ahci_plat {
    pub mode: *mut regmap,
    pub axi_rst: *mut reset_control,
    pub sw_rst: *mut reset_control,
    pub reg_rst: *mut reset_control,
}

    static const struct ata_port_info ahci_port_info = {
    .flags		= AHCI_FLAG_COMMON,
    .pio_mask	= ATA_PIO4,
    .udma_mask	= ATA_UDMA6,
    .port_ops	= &ahci_platform_ops,
    };
    static const struct scsi_host_template ahci_platform_sht = {
    AHCI_SHT(DRV_NAME),
    };
    static int mtk_ahci_platform_resets(struct ahci_host_priv *hpriv,
    struct device *dev)
    {
    struct mtk_ahci_plat *plat = hpriv.plat_data;
    int err;
// reset AXI bus and PHY part
    plat.axi_rst = devm_reset_control_get_optional_exclusive(dev, "axi");
    if (PTR_ERR(plat.axi_rst) == -EPROBE_DEFER)
    return PTR_ERR(plat.axi_rst);
    plat.sw_rst = devm_reset_control_get_optional_exclusive(dev, "sw");
    if (PTR_ERR(plat.sw_rst) == -EPROBE_DEFER)
    return PTR_ERR(plat.sw_rst);
    plat.reg_rst = devm_reset_control_get_optional_exclusive(dev, "reg");
    if (PTR_ERR(plat.reg_rst) == -EPROBE_DEFER)
    return PTR_ERR(plat.reg_rst);
    err = reset_control_assert(plat.axi_rst);
    if (err) {
    dev_err(dev, "failed to assert AXI bus\n");
    return err;
    }
    err = reset_control_assert(plat.sw_rst);
    if (err) {
    dev_err(dev, "failed to assert PHY digital part\n");
    return err;
    }
    err = reset_control_assert(plat.reg_rst);
    if (err) {
    dev_err(dev, "failed to assert PHY register part\n");
    return err;
    }
    err = reset_control_deassert(plat.reg_rst);
    if (err) {
    dev_err(dev, "failed to deassert PHY register part\n");
    return err;
    }
    err = reset_control_deassert(plat.sw_rst);
    if (err) {
    dev_err(dev, "failed to deassert PHY digital part\n");
    return err;
    }
    err = reset_control_deassert(plat.axi_rst);
    if (err) {
    dev_err(dev, "failed to deassert AXI bus\n");
    return err;
    }
    return 0;
    }
    static int mtk_ahci_parse_property(struct ahci_host_priv *hpriv,
    struct device *dev)
    {
    struct mtk_ahci_plat *plat = hpriv.plat_data;
    struct device_node *np = dev.of_node;
// enable SATA function if needed
    if (of_property_present(np, "mediatek,phy-mode")) {
    plat.mode = syscon_regmap_lookup_by_phandle(
    np, "mediatek,phy-mode");
    if (IS_ERR(plat.mode)) {
    dev_err(dev, "missing phy-mode phandle\n");
    return PTR_ERR(plat.mode);
    }
    regmap_update_bits(plat.mode, SYS_CFG, SYS_CFG_SATA_MSK,
    SYS_CFG_SATA_EN);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_ahci_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_ahci_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mtk_ahci_plat *plat;
    struct ahci_host_priv *hpriv;
    int err;
    plat = devm_kzalloc(dev, sizeof(*plat), GFP_KERNEL);
    if (!plat)
    return -ENOMEM;
    hpriv = ahci_platform_get_resources(pdev, 0);
    if (IS_ERR(hpriv))
    return PTR_ERR(hpriv);
    hpriv.plat_data = plat;
    err = mtk_ahci_parse_property(hpriv, dev);
    if (err)
    return err;
    err = mtk_ahci_platform_resets(hpriv, dev);
    if (err)
    return err;
    err = ahci_platform_enable_resources(hpriv);
    if (err)
    return err;
    err = ahci_platform_init_host(pdev, hpriv, &ahci_port_info,
    &ahci_platform_sht);
    if (err)
    goto disable_resources;
    return 0;
    disable_resources:
    ahci_platform_disable_resources(hpriv);
    return err;
    }
    static SIMPLE_DEV_PM_OPS(ahci_pm_ops, ahci_platform_suspend,
    ahci_platform_resume);
    static const struct of_device_id ahci_of_match[] = {
    { .compatible = "mediatek,mtk-ahci", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ahci_of_match);
    static struct platform_driver mtk_ahci_driver = {
    .probe = mtk_ahci_probe,
    .remove = ata_platform_remove_one,
    .driver = {
    .name = DRV_NAME,
    .of_match_table = ahci_of_match,
    .pm = &ahci_pm_ops,
    },
    };
    module_platform_driver(mtk_ahci_driver);
    MODULE_DESCRIPTION("MediaTek SATA AHCI Driver");
    MODULE_LICENSE("GPL v2");
