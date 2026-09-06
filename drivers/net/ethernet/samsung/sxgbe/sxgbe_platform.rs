//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/samsung/sxgbe/sxgbe_platform.c
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
// 10G controller driver for Samsung SoCs
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Siva Reddy Kallam <siva.kallam@samsung.com>
//

    static int sxgbe_probe_config_dt(struct platform_device *pdev,
    struct sxgbe_plat_data *plat)
    {
    struct device_node *np = pdev.dev.of_node;
    struct sxgbe_dma_cfg *dma_cfg;
    int err;
    if (!np)
    return -ENODEV;
    err = of_get_phy_mode(np, &plat.interface);
    if (err && err != -ENODEV)
    return err;
    plat.bus_id = of_alias_get_id(np, "ethernet");
    if (plat.bus_id < 0)
    plat.bus_id = 0;
    plat.mdio_bus_data = devm_kzalloc(&pdev.dev,
    sizeof(*plat.mdio_bus_data),
    GFP_KERNEL);
    if (!plat.mdio_bus_data)
    return -ENOMEM;
    dma_cfg = devm_kzalloc(&pdev.dev, sizeof(*dma_cfg), GFP_KERNEL);
    if (!dma_cfg)
    return -ENOMEM;
    plat.dma_cfg = dma_cfg;
    of_property_read_u32(np, "samsung,pbl", &dma_cfg.pbl);
    if (of_property_read_u32(np, "samsung,burst-map", &dma_cfg.burst_map) == 0)
    dma_cfg.fixed_burst = true;
    return 0;
    }

    static int sxgbe_probe_config_dt(struct platform_device *pdev,
    struct sxgbe_plat_data *plat)
    {
    return -ENOSYS;
    }

//
// sxgbe_platform_probe
// @pdev: platform device pointer
// Description: platform_device probe function. It allocates
// the necessary resources and invokes the main to init
// the net device, register the mdio bus etc.
//
#[no_mangle]
unsafe extern "C" fn sxgbe_platform_probe(pdev: *mut platform_device) -> c_int {
    static int sxgbe_platform_probe(struct platform_device *pdev)
    {
    int ret;
    int i, chan;
    struct device *dev = &pdev.dev;
    void __iomem *addr;
    struct sxgbe_priv_data *priv = core::ptr::null_mut();
    struct sxgbe_plat_data *plat_dat = core::ptr::null_mut();
    struct device_node *node = dev.of_node;
// Get memory resource
    addr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(addr))
    return PTR_ERR(addr);
    if (pdev.dev.of_node) {
    plat_dat = devm_kzalloc(&pdev.dev,
    sizeof(struct sxgbe_plat_data),
    GFP_KERNEL);
    if (!plat_dat)
    return  -ENOMEM;
    ret = sxgbe_probe_config_dt(pdev, plat_dat);
    if (ret) {
    pr_err("%s: main dt probe failed\n", __func__);
    return ret;
    }
    }
    priv = sxgbe_drv_probe(&(pdev.dev), plat_dat, addr);
    if (!priv) {
    pr_err("%s: main driver probe failed\n", __func__);
    goto err_out;
    }
// Get the SXGBE common INT information
    priv.irq  = irq_of_parse_and_map(node, 0);
    if (priv.irq <= 0) {
    dev_err(dev, "sxgbe common irq parsing failed\n");
    goto err_drv_remove;
    }
// Get MAC address if available (DT)
    of_get_ethdev_address(node, priv.dev);
// Get the TX/RX IRQ numbers
    for (i = 0, chan = 1; i < SXGBE_TX_QUEUES; i++) {
    priv.txq[i].irq_no = irq_of_parse_and_map(node, chan++);
    if (priv.txq[i].irq_no <= 0) {
    dev_err(dev, "sxgbe tx irq parsing failed\n");
    goto err_tx_irq_unmap;
    }
    }
    for (i = 0; i < SXGBE_RX_QUEUES; i++) {
    priv.rxq[i].irq_no = irq_of_parse_and_map(node, chan++);
    if (priv.rxq[i].irq_no <= 0) {
    dev_err(dev, "sxgbe rx irq parsing failed\n");
    goto err_rx_irq_unmap;
    }
    }
    priv.lpi_irq = irq_of_parse_and_map(node, chan);
    if (priv.lpi_irq <= 0) {
    dev_err(dev, "sxgbe lpi irq parsing failed\n");
    goto err_rx_irq_unmap;
    }
    platform_set_drvdata(pdev, priv.dev);
    pr_debug("platform driver registration completed\n");
    return 0;
    err_rx_irq_unmap:
    while (i--)
    irq_dispose_mapping(priv.rxq[i].irq_no);
    i = SXGBE_TX_QUEUES;
    err_tx_irq_unmap:
    while (i--)
    irq_dispose_mapping(priv.txq[i].irq_no);
    irq_dispose_mapping(priv.irq);
    err_drv_remove:
    sxgbe_drv_remove(priv.dev);
    err_out:
    return -ENODEV;
    }
//
// sxgbe_platform_remove
// @pdev: platform device pointer
// Description: this function calls the main to free the net resources
// and calls the platforms hook and release the resources (e.g. mem).
//
#[no_mangle]
unsafe extern "C" fn sxgbe_platform_remove(pdev: *mut platform_device) {
    static void sxgbe_platform_remove(struct platform_device *pdev)
    {
    struct net_device *ndev = platform_get_drvdata(pdev);
    sxgbe_drv_remove(ndev);
    }

#[no_mangle]
unsafe extern "C" fn sxgbe_platform_suspend(dev: *mut device) -> c_int {
    static int sxgbe_platform_suspend(struct device *dev)
    {
    struct net_device *ndev = dev_get_drvdata(dev);
    return sxgbe_suspend(ndev);
    }
#[no_mangle]
unsafe extern "C" fn sxgbe_platform_resume(dev: *mut device) -> c_int {
    static int sxgbe_platform_resume(struct device *dev)
    {
    struct net_device *ndev = dev_get_drvdata(dev);
    return sxgbe_resume(ndev);
    }
#[no_mangle]
unsafe extern "C" fn sxgbe_platform_freeze(dev: *mut device) -> c_int {
    static int sxgbe_platform_freeze(struct device *dev)
    {
    struct net_device *ndev = dev_get_drvdata(dev);
    return sxgbe_freeze(ndev);
    }
#[no_mangle]
unsafe extern "C" fn sxgbe_platform_restore(dev: *mut device) -> c_int {
    static int sxgbe_platform_restore(struct device *dev)
    {
    struct net_device *ndev = dev_get_drvdata(dev);
    return sxgbe_restore(ndev);
    }
    static const struct dev_pm_ops sxgbe_platform_pm_ops = {
    .suspend	= sxgbe_platform_suspend,
    .resume		= sxgbe_platform_resume,
    .freeze		= sxgbe_platform_freeze,
    .thaw		= sxgbe_platform_restore,
    .restore	= sxgbe_platform_restore,
    };

    static const struct dev_pm_ops sxgbe_platform_pm_ops;

    static const struct of_device_id sxgbe_dt_ids[] = {
    { .compatible = "samsung,sxgbe-v2.0a"},
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, sxgbe_dt_ids);
    static struct platform_driver sxgbe_platform_driver = {
    .probe	= sxgbe_platform_probe,
    .remove = sxgbe_platform_remove,
    .driver	= {
    .name		= SXGBE_RESOURCE_NAME,
    .pm		= &sxgbe_platform_pm_ops,
    .of_match_table	= sxgbe_dt_ids,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn sxgbe_register_platform() -> c_int {
    int sxgbe_register_platform(void)
    {
    int err;
    err = platform_driver_register(&sxgbe_platform_driver);
    if (err)
    pr_err("failed to register the platform driver\n");
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn sxgbe_unregister_platform() {
    void sxgbe_unregister_platform(void)
    {
    platform_driver_unregister(&sxgbe_platform_driver);
    }
