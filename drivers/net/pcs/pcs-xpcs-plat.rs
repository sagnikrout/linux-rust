//! Automatically rewritten from C to Rust
//! Source: drivers/net/pcs/pcs-xpcs-plat.c
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
//
// Synopsys DesignWare XPCS platform device driver
//
// Copyright (C) 2024 Serge Semin
//

// Page select register for the indirect MMIO CSRs access
pub const DW_VR_CSR_VIEWPORT: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_xpcs_plat {
    pub pdev: *mut platform_device,
    pub bus: *mut mii_bus,
    pub reg_indir: bool,
    pub reg_width: c_int,
    pub reg_base: *mut void __iomem,
    pub cclk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn xpcs_mmio_addr_format(dev: c_int, reg: c_int) -> ptrdiff_t {
    static ptrdiff_t xpcs_mmio_addr_format(int dev, int reg)
    {
    return FIELD_PREP(0x1f0000, dev) | FIELD_PREP(0xffff, reg);
    }
#[no_mangle]
unsafe extern "C" fn xpcs_mmio_addr_page(csr: ptrdiff_t) -> u16 {
    static u16 xpcs_mmio_addr_page(ptrdiff_t csr)
    {
    return FIELD_GET(0x1fff00, csr);
    }
#[no_mangle]
unsafe extern "C" fn xpcs_mmio_addr_offset(csr: ptrdiff_t) -> ptrdiff_t {
    static ptrdiff_t xpcs_mmio_addr_offset(ptrdiff_t csr)
    {
    return FIELD_GET(0xff, csr);
    }
    static int xpcs_mmio_read_reg_indirect(struct dw_xpcs_plat *pxpcs,
    int dev, int reg)
    {
    ptrdiff_t csr, ofs;
    u16 page;
    int ret;
    csr = xpcs_mmio_addr_format(dev, reg);
    page = xpcs_mmio_addr_page(csr);
    ofs = xpcs_mmio_addr_offset(csr);
    ret = pm_runtime_resume_and_get(&pxpcs.pdev.dev);
    if (ret)
    return ret;
    switch (pxpcs.reg_width) {
    case 4:
    writel(page, pxpcs.reg_base + (DW_VR_CSR_VIEWPORT << 2));
    ret = readl(pxpcs.reg_base + (ofs << 2)) & 0xffff;
    break;
    default:
    writew(page, pxpcs.reg_base + (DW_VR_CSR_VIEWPORT << 1));
    ret = readw(pxpcs.reg_base + (ofs << 1));
    break;
    }
    pm_runtime_put(&pxpcs.pdev.dev);
    return ret;
    }
    static int xpcs_mmio_write_reg_indirect(struct dw_xpcs_plat *pxpcs,
    int dev, int reg, u16 val)
    {
    ptrdiff_t csr, ofs;
    u16 page;
    int ret;
    csr = xpcs_mmio_addr_format(dev, reg);
    page = xpcs_mmio_addr_page(csr);
    ofs = xpcs_mmio_addr_offset(csr);
    ret = pm_runtime_resume_and_get(&pxpcs.pdev.dev);
    if (ret)
    return ret;
    switch (pxpcs.reg_width) {
    case 4:
    writel(page, pxpcs.reg_base + (DW_VR_CSR_VIEWPORT << 2));
    writel(val, pxpcs.reg_base + (ofs << 2));
    break;
    default:
    writew(page, pxpcs.reg_base + (DW_VR_CSR_VIEWPORT << 1));
    writew(val, pxpcs.reg_base + (ofs << 1));
    break;
    }
    pm_runtime_put(&pxpcs.pdev.dev);
    return 0;
    }
    static int xpcs_mmio_read_reg_direct(struct dw_xpcs_plat *pxpcs,
    int dev, int reg)
    {
    ptrdiff_t csr;
    int ret;
    csr = xpcs_mmio_addr_format(dev, reg);
    ret = pm_runtime_resume_and_get(&pxpcs.pdev.dev);
    if (ret)
    return ret;
    switch (pxpcs.reg_width) {
    case 4:
    ret = readl(pxpcs.reg_base + (csr << 2)) & 0xffff;
    break;
    default:
    ret = readw(pxpcs.reg_base + (csr << 1));
    break;
    }
    pm_runtime_put(&pxpcs.pdev.dev);
    return ret;
    }
    static int xpcs_mmio_write_reg_direct(struct dw_xpcs_plat *pxpcs,
    int dev, int reg, u16 val)
    {
    ptrdiff_t csr;
    int ret;
    csr = xpcs_mmio_addr_format(dev, reg);
    ret = pm_runtime_resume_and_get(&pxpcs.pdev.dev);
    if (ret)
    return ret;
    switch (pxpcs.reg_width) {
    case 4:
    writel(val, pxpcs.reg_base + (csr << 2));
    break;
    default:
    writew(val, pxpcs.reg_base + (csr << 1));
    break;
    }
    pm_runtime_put(&pxpcs.pdev.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xpcs_mmio_read_c22(bus: *mut mii_bus, addr: c_int, reg: c_int) -> c_int {
    static int xpcs_mmio_read_c22(struct mii_bus *bus, int addr, int reg)
    {
    struct dw_xpcs_plat *pxpcs = bus.priv;
    if (addr != 0)
    return -ENODEV;
    if (pxpcs.reg_indir)
    return xpcs_mmio_read_reg_indirect(pxpcs, MDIO_MMD_VEND2, reg);
    else
    return xpcs_mmio_read_reg_direct(pxpcs, MDIO_MMD_VEND2, reg);
    }
#[no_mangle]
unsafe extern "C" fn xpcs_mmio_write_c22(bus: *mut mii_bus, addr: c_int, reg: c_int, val: u16) -> c_int {
    static int xpcs_mmio_write_c22(struct mii_bus *bus, int addr, int reg, u16 val)
    {
    struct dw_xpcs_plat *pxpcs = bus.priv;
    if (addr != 0)
    return -ENODEV;
    if (pxpcs.reg_indir)
    return xpcs_mmio_write_reg_indirect(pxpcs, MDIO_MMD_VEND2, reg, val);
    else
    return xpcs_mmio_write_reg_direct(pxpcs, MDIO_MMD_VEND2, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn xpcs_mmio_read_c45(bus: *mut mii_bus, addr: c_int, dev: c_int, reg: c_int) -> c_int {
    static int xpcs_mmio_read_c45(struct mii_bus *bus, int addr, int dev, int reg)
    {
    struct dw_xpcs_plat *pxpcs = bus.priv;
    if (addr != 0)
    return -ENODEV;
    if (pxpcs.reg_indir)
    return xpcs_mmio_read_reg_indirect(pxpcs, dev, reg);
    else
    return xpcs_mmio_read_reg_direct(pxpcs, dev, reg);
    }
    static int xpcs_mmio_write_c45(struct mii_bus *bus, int addr, int dev,
    int reg, u16 val)
    {
    struct dw_xpcs_plat *pxpcs = bus.priv;
    if (addr != 0)
    return -ENODEV;
    if (pxpcs.reg_indir)
    return xpcs_mmio_write_reg_indirect(pxpcs, dev, reg, val);
    else
    return xpcs_mmio_write_reg_direct(pxpcs, dev, reg, val);
    }
    static struct dw_xpcs_plat *xpcs_plat_create_data(struct platform_device *pdev)
    {
    struct dw_xpcs_plat *pxpcs;
    pxpcs = devm_kzalloc(&pdev.dev, sizeof(*pxpcs), GFP_KERNEL);
    if (!pxpcs)
    return ERR_PTR(-ENOMEM);
    pxpcs.pdev = pdev;
    dev_set_drvdata(&pdev.dev, pxpcs);
    return pxpcs;
    }
#[no_mangle]
unsafe extern "C" fn xpcs_plat_init_res(pxpcs: *mut dw_xpcs_plat) -> c_int {
    static int xpcs_plat_init_res(struct dw_xpcs_plat *pxpcs)
    {
    struct platform_device *pdev = pxpcs.pdev;
    struct device *dev = &pdev.dev;
    resource_size_t spc_size;
    struct resource *res;
    if (!device_property_read_u32(dev, "reg-io-width", &pxpcs.reg_width)) {
    if (pxpcs.reg_width != 2 && pxpcs.reg_width != 4) {
    dev_err(dev, "Invalid reg-space data width\n");
    return -EINVAL;
    }
    } else {
    pxpcs.reg_width = 2;
    }
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "direct") ?:
    platform_get_resource_byname(pdev, IORESOURCE_MEM, "indirect");
    if (!res) {
    dev_err(dev, "No reg-space found\n");
    return -EINVAL;
    }
    if (!strcmp(res.name, "indirect"))
    pxpcs.reg_indir = true;
    if (pxpcs.reg_indir)
    spc_size = pxpcs.reg_width * SZ_256;
    else
    spc_size = pxpcs.reg_width * SZ_2M;
    if (resource_size(res) < spc_size) {
    dev_err(dev, "Invalid reg-space size\n");
    return -EINVAL;
    }
    pxpcs.reg_base = devm_ioremap_resource(dev, res);
    if (IS_ERR(pxpcs.reg_base)) {
    dev_err(dev, "Failed to map reg-space\n");
    return PTR_ERR(pxpcs.reg_base);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xpcs_plat_init_clk(pxpcs: *mut dw_xpcs_plat) -> c_int {
    static int xpcs_plat_init_clk(struct dw_xpcs_plat *pxpcs)
    {
    struct device *dev = &pxpcs.pdev.dev;
    int ret;
    pxpcs.cclk = devm_clk_get_optional(dev, "csr");
    if (IS_ERR(pxpcs.cclk))
    return dev_err_probe(dev, PTR_ERR(pxpcs.cclk),
    "Failed to get CSR clock\n");
    pm_runtime_set_active(dev);
    ret = devm_pm_runtime_enable(dev);
    if (ret) {
    dev_err(dev, "Failed to enable runtime-PM\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xpcs_plat_init_bus(pxpcs: *mut dw_xpcs_plat) -> c_int {
    static int xpcs_plat_init_bus(struct dw_xpcs_plat *pxpcs)
    {
    struct device *dev = &pxpcs.pdev.dev;
    let mut id: static atomic_t = ATOMIC_INIT(-1);
    int ret;
    pxpcs.bus = devm_mdiobus_alloc_size(dev, 0);
    if (!pxpcs.bus)
    return -ENOMEM;
    pxpcs.bus.name = "DW XPCS MCI/APB3";
    pxpcs.bus.read = xpcs_mmio_read_c22;
    pxpcs.bus.write = xpcs_mmio_write_c22;
    pxpcs.bus.read_c45 = xpcs_mmio_read_c45;
    pxpcs.bus.write_c45 = xpcs_mmio_write_c45;
    pxpcs.bus.phy_mask = ~0;
    pxpcs.bus.parent = dev;
    pxpcs.bus.priv = pxpcs;
    snprintf(pxpcs.bus.id, MII_BUS_ID_SIZE,
    "dwxpcs-%x", atomic_inc_return(&id));
// MDIO-bus here serves as just a back-end engine abstracting out
// the MDIO and MCI/APB3 IO interfaces utilized for the DW XPCS CSRs
// access.
//
    ret = devm_mdiobus_register(dev, pxpcs.bus);
    if (ret) {
    dev_err(dev, "Failed to create MDIO bus\n");
    return ret;
    }
    return 0;
    }
// Note there is no need in the next function antagonist because the MDIO-bus
// de-registration will effectively remove and destroy all the MDIO-devices
// registered on the bus.
//
#[no_mangle]
unsafe extern "C" fn xpcs_plat_init_dev(pxpcs: *mut dw_xpcs_plat) -> c_int {
    static int xpcs_plat_init_dev(struct dw_xpcs_plat *pxpcs)
    {
    struct device *dev = &pxpcs.pdev.dev;
    struct mdio_device *mdiodev;
    int ret;
// There is a single memory-mapped DW XPCS device
    mdiodev = mdio_device_create(pxpcs.bus, 0);
    if (IS_ERR(mdiodev))
    return PTR_ERR(mdiodev);
// Associate the FW-node with the device structure so it can be looked
// up later. Make sure DD-core is aware of the OF-node being re-used.
//
    device_set_node(&mdiodev.dev, fwnode_handle_get(dev_fwnode(dev)));
    dev_set_of_node_reused(&mdiodev.dev);
// Pass the data further so the DW XPCS driver core could use it
    mdiodev.dev.platform_data = (void *)device_get_match_data(dev);
    ret = mdio_device_register(mdiodev);
    if (ret) {
    dev_err(dev, "Failed to register MDIO device\n");
    goto err_clean_data;
    }
    return 0;
    err_clean_data:
    mdiodev.dev.platform_data = core::ptr::null_mut();
    mdio_device_free(mdiodev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn xpcs_plat_probe(pdev: *mut platform_device) -> c_int {
    static int xpcs_plat_probe(struct platform_device *pdev)
    {
    struct dw_xpcs_plat *pxpcs;
    int ret;
    pxpcs = xpcs_plat_create_data(pdev);
    if (IS_ERR(pxpcs))
    return PTR_ERR(pxpcs);
    ret = xpcs_plat_init_res(pxpcs);
    if (ret)
    return ret;
    ret = xpcs_plat_init_clk(pxpcs);
    if (ret)
    return ret;
    ret = xpcs_plat_init_bus(pxpcs);
    if (ret)
    return ret;
    ret = xpcs_plat_init_dev(pxpcs);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xpcs_plat_pm_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused xpcs_plat_pm_runtime_suspend(struct device *dev)
    {
    struct dw_xpcs_plat *pxpcs = dev_get_drvdata(dev);
    clk_disable_unprepare(pxpcs.cclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xpcs_plat_pm_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused xpcs_plat_pm_runtime_resume(struct device *dev)
    {
    struct dw_xpcs_plat *pxpcs = dev_get_drvdata(dev);
    return clk_prepare_enable(pxpcs.cclk);
    }
    static const struct dev_pm_ops xpcs_plat_pm_ops = {
    SET_RUNTIME_PM_OPS(xpcs_plat_pm_runtime_suspend,
    xpcs_plat_pm_runtime_resume,
    core::ptr::null_mut())
    };
    DW_XPCS_INFO_DECLARE(xpcs_generic, DW_XPCS_ID_NATIVE, DW_XPCS_PMA_ID_NATIVE);
    DW_XPCS_INFO_DECLARE(xpcs_pma_gen1_3g, DW_XPCS_ID_NATIVE, DW_XPCS_PMA_GEN1_3G_ID);
    DW_XPCS_INFO_DECLARE(xpcs_pma_gen2_3g, DW_XPCS_ID_NATIVE, DW_XPCS_PMA_GEN2_3G_ID);
    DW_XPCS_INFO_DECLARE(xpcs_pma_gen2_6g, DW_XPCS_ID_NATIVE, DW_XPCS_PMA_GEN2_6G_ID);
    DW_XPCS_INFO_DECLARE(xpcs_pma_gen4_3g, DW_XPCS_ID_NATIVE, DW_XPCS_PMA_GEN4_3G_ID);
    DW_XPCS_INFO_DECLARE(xpcs_pma_gen4_6g, DW_XPCS_ID_NATIVE, DW_XPCS_PMA_GEN4_6G_ID);
    DW_XPCS_INFO_DECLARE(xpcs_pma_gen5_10g, DW_XPCS_ID_NATIVE, DW_XPCS_PMA_GEN5_10G_ID);
    DW_XPCS_INFO_DECLARE(xpcs_pma_gen5_12g, DW_XPCS_ID_NATIVE, DW_XPCS_PMA_GEN5_12G_ID);
    static const struct of_device_id xpcs_of_ids[] = {
    { .compatible = "snps,dw-xpcs", .data = &xpcs_generic },
    { .compatible = "snps,dw-xpcs-gen1-3g", .data = &xpcs_pma_gen1_3g },
    { .compatible = "snps,dw-xpcs-gen2-3g", .data = &xpcs_pma_gen2_3g },
    { .compatible = "snps,dw-xpcs-gen2-6g", .data = &xpcs_pma_gen2_6g },
    { .compatible = "snps,dw-xpcs-gen4-3g", .data = &xpcs_pma_gen4_3g },
    { .compatible = "snps,dw-xpcs-gen4-6g", .data = &xpcs_pma_gen4_6g },
    { .compatible = "snps,dw-xpcs-gen5-10g", .data = &xpcs_pma_gen5_10g },
    { .compatible = "snps,dw-xpcs-gen5-12g", .data = &xpcs_pma_gen5_12g },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, xpcs_of_ids);
    static struct platform_driver xpcs_plat_driver = {
    .probe = xpcs_plat_probe,
    .driver = {
    .name = "dwxpcs",
    .pm = &xpcs_plat_pm_ops,
    .of_match_table = xpcs_of_ids,
    },
    };
    module_platform_driver(xpcs_plat_driver);
    MODULE_DESCRIPTION("Synopsys DesignWare XPCS platform device driver");
    MODULE_AUTHOR("Serge Semin <fancer.lancer@gmail.com>");
    MODULE_LICENSE("GPL");
