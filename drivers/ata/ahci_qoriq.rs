//! Automatically rewritten from C to Rust
//! Source: drivers/ata/ahci_qoriq.c
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
// Freescale QorIQ AHCI SATA platform driver
//
// Copyright 2015 Freescale, Inc.
// Tang Yuantian <Yuantian.Tang@freescale.com>
//

// port register definition
pub const PORT_PHY1: c_uint = 0xA8;
pub const PORT_PHY2: c_uint = 0xAC;
pub const PORT_PHY3: c_uint = 0xB0;
pub const PORT_PHY4: c_uint = 0xB4;
pub const PORT_PHY5: c_uint = 0xB8;
pub const PORT_AXICC: c_uint = 0xBC;
pub const PORT_TRANS: c_uint = 0xC8;
// port register default value
pub const AHCI_PORT_PHY_1_CFG: c_uint = 0xa003fffe;
pub const AHCI_PORT_PHY2_CFG: c_uint = 0x28184d1f;
pub const AHCI_PORT_PHY3_CFG: c_uint = 0x0e081509;
pub const AHCI_PORT_TRANS_CFG: c_uint = 0x08000029;
pub const AHCI_PORT_AXICC_CFG: c_uint = 0x3fffffff;
// for ls1021a
pub const LS1021A_PORT_PHY2: c_uint = 0x28183414;
pub const LS1021A_PORT_PHY3: c_uint = 0x0e080e06;
pub const LS1021A_PORT_PHY4: c_uint = 0x064a080b;
pub const LS1021A_PORT_PHY5: c_uint = 0x2aa86470;
pub const LS1021A_AXICC_ADDR: c_uint = 0xC0;
pub const SATA_ECC_DISABLE: c_uint = 0x00020000;
pub const ECC_DIS_ARMV8_CH2: c_uint = 0x80000000;
pub const ECC_DIS_LS1088A: c_uint = 0x40000000;
    enum ahci_qoriq_type {
    AHCI_LS1021A,
    AHCI_LS1028A,
    AHCI_LS1043A,
    AHCI_LS2080A,
    AHCI_LS1046A,
    AHCI_LS1088A,
    AHCI_LS2088A,
    AHCI_LX2160A,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahci_qoriq_priv {
    pub reg_base: *mut ccsr_ahci,
    pub type: enum ahci_qoriq_type,
    pub ecc_addr: *mut void __iomem,
    pub is_dmacoherent: bool,
}

    static bool ecc_initialized;
    static const struct of_device_id ahci_qoriq_of_match[] = {
    { .compatible = "fsl,ls1021a-ahci", .data = (void *)AHCI_LS1021A},
    { .compatible = "fsl,ls1028a-ahci", .data = (void *)AHCI_LS1028A},
    { .compatible = "fsl,ls1043a-ahci", .data = (void *)AHCI_LS1043A},
    { .compatible = "fsl,ls2080a-ahci", .data = (void *)AHCI_LS2080A},
    { .compatible = "fsl,ls1046a-ahci", .data = (void *)AHCI_LS1046A},
    { .compatible = "fsl,ls1088a-ahci", .data = (void *)AHCI_LS1088A},
    { .compatible = "fsl,ls2088a-ahci", .data = (void *)AHCI_LS2088A},
    { .compatible = "fsl,lx2160a-ahci", .data = (void *)AHCI_LX2160A},
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ahci_qoriq_of_match);
    static const struct acpi_device_id ahci_qoriq_acpi_match[] = {
    { .id = "NXP0004", .driver_data = (kernel_ulong_t)AHCI_LX2160A },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, ahci_qoriq_acpi_match);
    static int ahci_qoriq_hardreset(struct ata_link *link, unsigned int *class,
    unsigned long deadline)
    {
    const unsigned int *timing = sata_ehc_deb_timing(&link.eh_context);
    void __iomem *port_mmio = ahci_port_base(link.ap);
    u32 px_cmd, px_is, px_val;
    struct ata_port *ap = link.ap;
    struct ahci_port_priv *pp = ap.private_data;
    struct ahci_host_priv *hpriv = ap.host.private_data;
    struct ahci_qoriq_priv *qoriq_priv = hpriv.plat_data;
    u8 *d2h_fis = pp.rx_fis + RX_FIS_D2H_REG;
    struct ata_taskfile tf;
    bool online;
    int rc;
    let mut ls1021a_workaround: bool = (qoriq_priv.type == AHCI_LS1021A);
    hpriv.stop_engine(ap);
//
// There is a errata on ls1021a Rev1.0 and Rev2.0 which is:
// A-009042: The device detection initialization sequence
// mistakenly resets some registers.
//
// Workaround for this is:
// The software should read and store PxCMD and PxIS values
// before issuing the device detection initialization sequence.
// After the sequence is complete, software should restore the
// PxCMD and PxIS with the stored values.
//
    if (ls1021a_workaround) {
    px_cmd = readl(port_mmio + PORT_CMD);
    px_is = readl(port_mmio + PORT_IRQ_STAT);
    }
// clear D2H reception area to properly wait for D2H FIS
    ata_tf_init(link.device, &tf);
    tf.status = ATA_BUSY;
    ata_tf_to_fis(&tf, 0, 0, d2h_fis);
    rc = sata_link_hardreset(link, timing, deadline, &online,
    ahci_check_ready);
// restore the PxCMD and PxIS on ls1021
    if (ls1021a_workaround) {
    px_val = readl(port_mmio + PORT_CMD);
    if (px_val != px_cmd)
    writel(px_cmd, port_mmio + PORT_CMD);
    px_val = readl(port_mmio + PORT_IRQ_STAT);
    if (px_val != px_is)
    writel(px_is, port_mmio + PORT_IRQ_STAT);
    }
    hpriv.start_engine(ap);
    if (online)
// class = ahci_dev_classify(ap);
    return rc;
    }
    static struct ata_port_operations ahci_qoriq_ops = {
    .inherits		= &ahci_ops,
    .reset.hardreset	= ahci_qoriq_hardreset,
    };
    static const struct ata_port_info ahci_qoriq_port_info = {
    .flags		= AHCI_FLAG_COMMON | ATA_FLAG_NCQ,
    .pio_mask	= ATA_PIO4,
    .udma_mask	= ATA_UDMA6,
    .port_ops	= &ahci_qoriq_ops,
    };
    static const struct scsi_host_template ahci_qoriq_sht = {
    AHCI_SHT(DRV_NAME),
    };
#[no_mangle]
unsafe extern "C" fn ahci_qoriq_phy_init(hpriv: *mut ahci_host_priv) -> c_int {
    static int ahci_qoriq_phy_init(struct ahci_host_priv *hpriv)
    {
    struct ahci_qoriq_priv *qpriv = hpriv.plat_data;
    void __iomem *reg_base = hpriv.mmio;
    switch (qpriv.type) {
    case AHCI_LS1021A:
    if (!(qpriv.ecc_addr || ecc_initialized))
    return -EINVAL;
#[no_mangle]
pub unsafe extern "C" fn if(!ecc_initialized: qpriv->ecc_addr &&) -> else {
    else if (qpriv.ecc_addr && !ecc_initialized)
    writel(SATA_ECC_DISABLE, qpriv.ecc_addr);
    writel(AHCI_PORT_PHY_1_CFG, reg_base + PORT_PHY1);
    writel(LS1021A_PORT_PHY2, reg_base + PORT_PHY2);
    writel(LS1021A_PORT_PHY3, reg_base + PORT_PHY3);
    writel(LS1021A_PORT_PHY4, reg_base + PORT_PHY4);
    writel(LS1021A_PORT_PHY5, reg_base + PORT_PHY5);
    writel(AHCI_PORT_TRANS_CFG, reg_base + PORT_TRANS);
    if (qpriv.is_dmacoherent)
    writel(AHCI_PORT_AXICC_CFG,
    reg_base + LS1021A_AXICC_ADDR);
    break;
    case AHCI_LS1043A:
    if (!(qpriv.ecc_addr || ecc_initialized))
    return -EINVAL;
#[no_mangle]
pub unsafe extern "C" fn if(!ecc_initialized: qpriv->ecc_addr &&) -> else {
    else if (qpriv.ecc_addr && !ecc_initialized)
    writel(readl(qpriv.ecc_addr) |
    ECC_DIS_ARMV8_CH2,
    qpriv.ecc_addr);
    writel(AHCI_PORT_PHY_1_CFG, reg_base + PORT_PHY1);
    writel(AHCI_PORT_PHY2_CFG, reg_base + PORT_PHY2);
    writel(AHCI_PORT_PHY3_CFG, reg_base + PORT_PHY3);
    writel(AHCI_PORT_TRANS_CFG, reg_base + PORT_TRANS);
    if (qpriv.is_dmacoherent)
    writel(AHCI_PORT_AXICC_CFG, reg_base + PORT_AXICC);
    break;
    case AHCI_LS2080A:
    writel(AHCI_PORT_PHY_1_CFG, reg_base + PORT_PHY1);
    writel(AHCI_PORT_PHY2_CFG, reg_base + PORT_PHY2);
    writel(AHCI_PORT_PHY3_CFG, reg_base + PORT_PHY3);
    writel(AHCI_PORT_TRANS_CFG, reg_base + PORT_TRANS);
    if (qpriv.is_dmacoherent)
    writel(AHCI_PORT_AXICC_CFG, reg_base + PORT_AXICC);
    break;
    case AHCI_LS1046A:
    if (!(qpriv.ecc_addr || ecc_initialized))
    return -EINVAL;
#[no_mangle]
pub unsafe extern "C" fn if(!ecc_initialized: qpriv->ecc_addr &&) -> else {
    else if (qpriv.ecc_addr && !ecc_initialized)
    writel(readl(qpriv.ecc_addr) |
    ECC_DIS_ARMV8_CH2,
    qpriv.ecc_addr);
    writel(AHCI_PORT_PHY_1_CFG, reg_base + PORT_PHY1);
    writel(AHCI_PORT_PHY2_CFG, reg_base + PORT_PHY2);
    writel(AHCI_PORT_PHY3_CFG, reg_base + PORT_PHY3);
    writel(AHCI_PORT_TRANS_CFG, reg_base + PORT_TRANS);
    if (qpriv.is_dmacoherent)
    writel(AHCI_PORT_AXICC_CFG, reg_base + PORT_AXICC);
    break;
    case AHCI_LS1028A:
    case AHCI_LS1088A:
    case AHCI_LX2160A:
    if (!(qpriv.ecc_addr || ecc_initialized))
    return -EINVAL;
#[no_mangle]
pub unsafe extern "C" fn if(!ecc_initialized: qpriv->ecc_addr &&) -> else {
    else if (qpriv.ecc_addr && !ecc_initialized)
    writel(readl(qpriv.ecc_addr) |
    ECC_DIS_LS1088A,
    qpriv.ecc_addr);
    writel(AHCI_PORT_PHY_1_CFG, reg_base + PORT_PHY1);
    writel(AHCI_PORT_PHY2_CFG, reg_base + PORT_PHY2);
    writel(AHCI_PORT_PHY3_CFG, reg_base + PORT_PHY3);
    writel(AHCI_PORT_TRANS_CFG, reg_base + PORT_TRANS);
    if (qpriv.is_dmacoherent)
    writel(AHCI_PORT_AXICC_CFG, reg_base + PORT_AXICC);
    break;
    case AHCI_LS2088A:
    writel(AHCI_PORT_PHY_1_CFG, reg_base + PORT_PHY1);
    writel(AHCI_PORT_PHY2_CFG, reg_base + PORT_PHY2);
    writel(AHCI_PORT_PHY3_CFG, reg_base + PORT_PHY3);
    writel(AHCI_PORT_TRANS_CFG, reg_base + PORT_TRANS);
    if (qpriv.is_dmacoherent)
    writel(AHCI_PORT_AXICC_CFG, reg_base + PORT_AXICC);
    break;
    }
    ecc_initialized = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ahci_qoriq_probe(pdev: *mut platform_device) -> c_int {
    static int ahci_qoriq_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    const struct acpi_device_id *acpi_id;
    struct device *dev = &pdev.dev;
    struct ahci_host_priv *hpriv;
    struct ahci_qoriq_priv *qoriq_priv;
    const struct of_device_id *of_id;
    struct resource *res;
    int rc;
    hpriv = ahci_platform_get_resources(pdev, 0);
    if (IS_ERR(hpriv))
    return PTR_ERR(hpriv);
    of_id = of_match_node(ahci_qoriq_of_match, np);
    acpi_id = acpi_match_device(ahci_qoriq_acpi_match, &pdev.dev);
    if (!(of_id || acpi_id))
    return -ENODEV;
    qoriq_priv = devm_kzalloc(dev, sizeof(*qoriq_priv), GFP_KERNEL);
    if (!qoriq_priv)
    return -ENOMEM;
    if (of_id)
    qoriq_priv.type = (unsigned long)of_id.data;
    else
    qoriq_priv.type = (enum ahci_qoriq_type)acpi_id.driver_data;
    if (unlikely(!ecc_initialized)) {
    res = platform_get_resource_byname(pdev,
    IORESOURCE_MEM,
    "sata-ecc");
    if (res) {
    qoriq_priv.ecc_addr =
    devm_ioremap_resource(dev, res);
    if (IS_ERR(qoriq_priv.ecc_addr))
    return PTR_ERR(qoriq_priv.ecc_addr);
    }
    }
    if (device_get_dma_attr(&pdev.dev) == DEV_DMA_COHERENT)
    qoriq_priv.is_dmacoherent = true;
    rc = ahci_platform_enable_resources(hpriv);
    if (rc)
    return rc;
    hpriv.plat_data = qoriq_priv;
    rc = ahci_qoriq_phy_init(hpriv);
    if (rc)
    goto disable_resources;
    rc = ahci_platform_init_host(pdev, hpriv, &ahci_qoriq_port_info,
    &ahci_qoriq_sht);
    if (rc)
    goto disable_resources;
    return 0;
    disable_resources:
    ahci_platform_disable_resources(hpriv);
    return rc;
    }

#[no_mangle]
unsafe extern "C" fn ahci_qoriq_resume(dev: *mut device) -> c_int {
    static int ahci_qoriq_resume(struct device *dev)
    {
    struct ata_host *host = dev_get_drvdata(dev);
    struct ahci_host_priv *hpriv = host.private_data;
    int rc;
    rc = ahci_platform_enable_resources(hpriv);
    if (rc)
    return rc;
    rc = ahci_qoriq_phy_init(hpriv);
    if (rc)
    goto disable_resources;
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

    static SIMPLE_DEV_PM_OPS(ahci_qoriq_pm_ops, ahci_platform_suspend,
    ahci_qoriq_resume);
    static struct platform_driver ahci_qoriq_driver = {
    .probe = ahci_qoriq_probe,
    .remove = ata_platform_remove_one,
    .driver = {
    .name = DRV_NAME,
    .of_match_table = ahci_qoriq_of_match,
    .acpi_match_table = ahci_qoriq_acpi_match,
    .pm = &ahci_qoriq_pm_ops,
    },
    };
    module_platform_driver(ahci_qoriq_driver);
    MODULE_DESCRIPTION("Freescale QorIQ AHCI SATA platform driver");
    MODULE_AUTHOR("Tang Yuantian <Yuantian.Tang@freescale.com>");
    MODULE_LICENSE("GPL");
