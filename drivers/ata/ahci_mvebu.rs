//! Automatically rewritten from C to Rust
//! Source: drivers/ata/ahci_mvebu.c
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
// AHCI glue platform driver for Marvell EBU SOCs
//
// Copyright (C) 2014 Marvell
//
// Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
// Marcin Wojtas <mw@semihalf.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2.  This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

pub const AHCI_VENDOR_SPECIFIC_0_ADDR: c_uint = 0xa0;
pub const AHCI_VENDOR_SPECIFIC_0_DATA: c_uint = 0xa4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahci_mvebu_plat_data {
    pub hpriv): *mut *mut int (plat_config)(struct ahci_host_priv,
    pub flags: c_uint,
}

    static void ahci_mvebu_mbus_config(struct ahci_host_priv *hpriv,
    const struct mbus_dram_target_info *dram)
    {
    int i;
    for (i = 0; i < 4; i++) {
    writel(0, hpriv.mmio + AHCI_WINDOW_CTRL(i));
    writel(0, hpriv.mmio + AHCI_WINDOW_BASE(i));
    writel(0, hpriv.mmio + AHCI_WINDOW_SIZE(i));
    }
    for (i = 0; i < dram.num_cs; i++) {
    const struct mbus_dram_window *cs = dram.cs + i;
    writel((cs.mbus_attr << 8) |
    (dram.mbus_dram_target_id << 4) | 1,
    hpriv.mmio + AHCI_WINDOW_CTRL(i));
    writel(cs.base >> 16, hpriv.mmio + AHCI_WINDOW_BASE(i));
    writel(((cs.size - 1) & 0xffff0000),
    hpriv.mmio + AHCI_WINDOW_SIZE(i));
    }
    }
#[no_mangle]
unsafe extern "C" fn ahci_mvebu_regret_option(hpriv: *mut ahci_host_priv) {
    static void ahci_mvebu_regret_option(struct ahci_host_priv *hpriv)
    {
//
// Enable the regret bit to allow the SATA unit to regret a
// request that didn't receive an acknowlegde and avoid a
// deadlock
//
    writel(0x4, hpriv.mmio + AHCI_VENDOR_SPECIFIC_0_ADDR);
    writel(0x80, hpriv.mmio + AHCI_VENDOR_SPECIFIC_0_DATA);
    }
#[no_mangle]
unsafe extern "C" fn ahci_mvebu_armada_380_config(hpriv: *mut ahci_host_priv) -> c_int {
    static int ahci_mvebu_armada_380_config(struct ahci_host_priv *hpriv)
    {
    const struct mbus_dram_target_info *dram;
    let mut rc: c_int = 0;
    dram = mv_mbus_dram_info();
    if (dram)
    ahci_mvebu_mbus_config(hpriv, dram);
    else
    rc = -ENODEV;
    ahci_mvebu_regret_option(hpriv);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn ahci_mvebu_armada_3700_config(hpriv: *mut ahci_host_priv) -> c_int {
    static int ahci_mvebu_armada_3700_config(struct ahci_host_priv *hpriv)
    {
    u32 reg;
    writel(0, hpriv.mmio + AHCI_VENDOR_SPECIFIC_0_ADDR);
    reg = readl(hpriv.mmio + AHCI_VENDOR_SPECIFIC_0_DATA);
    reg |= BIT(6);
    writel(reg, hpriv.mmio + AHCI_VENDOR_SPECIFIC_0_DATA);
    return 0;
    }
//
// ahci_mvebu_stop_engine
//
// @ap:	Target ata port
//
// Errata Ref#226 - SATA Disk HOT swap issue when connected through
// Port Multiplier in FIS-based Switching mode.
//
// To avoid the issue, according to design, the bits[11:8, 0] of
// register PxFBS are cleared when Port Command and Status (0x18) bit[0]
// changes its value from 1 to 0, i.e. falling edge of Port
// Command and Status bit[0] sends PULSE that resets PxFBS
// bits[11:8; 0].
//
// This function is used to override function of "ahci_stop_engine"
// from libahci.c by adding the mvebu work around(WA) to save PxFBS
// value before the PxCMD ST write of 0, then restore PxFBS value.
//
// Return: 0 on success; Error code otherwise.
//
#[no_mangle]
unsafe extern "C" fn ahci_mvebu_stop_engine(ap: *mut ata_port) -> c_int {
    static int ahci_mvebu_stop_engine(struct ata_port *ap)
    {
    void __iomem *port_mmio = ahci_port_base(ap);
    u32 tmp, port_fbs;
    tmp = readl(port_mmio + PORT_CMD);
// check if the HBA is idle
    if ((tmp & (PORT_CMD_START | PORT_CMD_LIST_ON)) == 0)
    return 0;
// save the port PxFBS register for later restore
    port_fbs = readl(port_mmio + PORT_FBS);
// setting HBA to idle
    tmp &= ~PORT_CMD_START;
    writel(tmp, port_mmio + PORT_CMD);
//
// bit #15 PxCMD signal doesn't clear PxFBS,
// restore the PxFBS register right after clearing the PxCMD ST,
// no need to wait for the PxCMD bit #15.
//
    writel(port_fbs, port_mmio + PORT_FBS);
// wait for engine to stop. This could be as long as 500 msec
    tmp = ata_wait_register(ap, port_mmio + PORT_CMD,
    PORT_CMD_LIST_ON, PORT_CMD_LIST_ON, 1, 500);
    if (tmp & PORT_CMD_LIST_ON)
    return -EIO;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn ahci_mvebu_suspend(pdev: *mut platform_device, state: pm_message_t) -> c_int {
    static int ahci_mvebu_suspend(struct platform_device *pdev, pm_message_t state)
    {
    return ahci_platform_suspend_host(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn ahci_mvebu_resume(pdev: *mut platform_device) -> c_int {
    static int ahci_mvebu_resume(struct platform_device *pdev)
    {
    struct ata_host *host = platform_get_drvdata(pdev);
    struct ahci_host_priv *hpriv = host.private_data;
    const struct ahci_mvebu_plat_data *pdata = hpriv.plat_data;
    pdata.plat_config(hpriv);
    return ahci_platform_resume_host(&pdev.dev);
    }

    static const struct ata_port_info ahci_mvebu_port_info = {
    .flags	   = AHCI_FLAG_COMMON,
    .pio_mask  = ATA_PIO4,
    .udma_mask = ATA_UDMA6,
    .port_ops  = &ahci_platform_ops,
    };
    static const struct scsi_host_template ahci_platform_sht = {
    AHCI_SHT(DRV_NAME),
    };
#[no_mangle]
unsafe extern "C" fn ahci_mvebu_probe(pdev: *mut platform_device) -> c_int {
    static int ahci_mvebu_probe(struct platform_device *pdev)
    {
    const struct ahci_mvebu_plat_data *pdata;
    struct ahci_host_priv *hpriv;
    int rc;
    pdata = of_device_get_match_data(&pdev.dev);
    if (!pdata)
    return -EINVAL;
    hpriv = ahci_platform_get_resources(pdev, 0);
    if (IS_ERR(hpriv))
    return PTR_ERR(hpriv);
    hpriv.flags |= pdata.flags;
    hpriv.plat_data = (void *)pdata;
    rc = ahci_platform_enable_resources(hpriv);
    if (rc)
    return rc;
    hpriv.stop_engine = ahci_mvebu_stop_engine;
    rc = pdata.plat_config(hpriv);
    if (rc)
    goto disable_resources;
    rc = ahci_platform_init_host(pdev, hpriv, &ahci_mvebu_port_info,
    &ahci_platform_sht);
    if (rc)
    goto disable_resources;
    return 0;
    disable_resources:
    ahci_platform_disable_resources(hpriv);
    return rc;
    }
    static const struct ahci_mvebu_plat_data ahci_mvebu_armada_380_plat_data = {
    .plat_config = ahci_mvebu_armada_380_config,
    };
    static const struct ahci_mvebu_plat_data ahci_mvebu_armada_3700_plat_data = {
    .plat_config = ahci_mvebu_armada_3700_config,
    .flags = AHCI_HFLAG_SUSPEND_PHYS,
    };
    static const struct of_device_id ahci_mvebu_of_match[] = {
    {
    .compatible = "marvell,armada-380-ahci",
    .data = &ahci_mvebu_armada_380_plat_data,
    },
    {
    .compatible = "marvell,armada-3700-ahci",
    .data = &ahci_mvebu_armada_3700_plat_data,
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ahci_mvebu_of_match);
    static struct platform_driver ahci_mvebu_driver = {
    .probe = ahci_mvebu_probe,
    .remove = ata_platform_remove_one,
    .suspend = ahci_mvebu_suspend,
    .resume = ahci_mvebu_resume,
    .driver = {
    .name = DRV_NAME,
    .of_match_table = ahci_mvebu_of_match,
    },
    };
    module_platform_driver(ahci_mvebu_driver);
    MODULE_DESCRIPTION("Marvell EBU AHCI SATA driver");
    MODULE_AUTHOR("Thomas Petazzoni <thomas.petazzoni@free-electrons.com>, Marcin Wojtas <mw@semihalf.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:ahci_mvebu");
