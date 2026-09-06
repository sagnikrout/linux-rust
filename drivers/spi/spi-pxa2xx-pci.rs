//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-pxa2xx-pci.c
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
// PCI glue driver for SPI PXA2xx compatible controllers.
// CE4100's SPI device is more or less the same one as found on PXA.
//
// Copyright (C) 2016, 2021 Intel Corporation
//

pub const PCI_DEVICE_ID_INTEL_QUARK_X1000: c_uint = 0x0935;
pub const PCI_DEVICE_ID_INTEL_BYT: c_uint = 0x0f0e;
pub const PCI_DEVICE_ID_INTEL_MRFLD: c_uint = 0x1194;
pub const PCI_DEVICE_ID_INTEL_BSW0: c_uint = 0x228e;
pub const PCI_DEVICE_ID_INTEL_BSW1: c_uint = 0x2290;
pub const PCI_DEVICE_ID_INTEL_BSW2: c_uint = 0x22ac;
pub const PCI_DEVICE_ID_INTEL_CE4100: c_uint = 0x2e6a;
pub const PCI_DEVICE_ID_INTEL_LPT0_0: c_uint = 0x9c65;
pub const PCI_DEVICE_ID_INTEL_LPT0_1: c_uint = 0x9c66;
pub const PCI_DEVICE_ID_INTEL_LPT1_0: c_uint = 0x9ce5;
pub const PCI_DEVICE_ID_INTEL_LPT1_1: c_uint = 0x9ce6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa_spi_info {
    pub c): *mut *mut *mut int (setup)(struct pci_dev pdev, struct pxa2xx_spi_controller,
}

    let mut byt_tx_param: static struct dw_dma_slave = { .dst_id = 0 };
    let mut byt_rx_param: static struct dw_dma_slave = { .src_id = 1 };
    let mut mrfld3_tx_param: static struct dw_dma_slave = { .dst_id = 15 };
    let mut mrfld3_rx_param: static struct dw_dma_slave = { .src_id = 14 };
    let mut mrfld5_tx_param: static struct dw_dma_slave = { .dst_id = 13 };
    let mut mrfld5_rx_param: static struct dw_dma_slave = { .src_id = 12 };
    let mut mrfld6_tx_param: static struct dw_dma_slave = { .dst_id = 11 };
    let mut mrfld6_rx_param: static struct dw_dma_slave = { .src_id = 10 };
    let mut bsw0_tx_param: static struct dw_dma_slave = { .dst_id = 0 };
    let mut bsw0_rx_param: static struct dw_dma_slave = { .src_id = 1 };
    let mut bsw1_tx_param: static struct dw_dma_slave = { .dst_id = 6 };
    let mut bsw1_rx_param: static struct dw_dma_slave = { .src_id = 7 };
    let mut bsw2_tx_param: static struct dw_dma_slave = { .dst_id = 8 };
    let mut bsw2_rx_param: static struct dw_dma_slave = { .src_id = 9 };
    let mut lpt1_tx_param: static struct dw_dma_slave = { .dst_id = 0 };
    let mut lpt1_rx_param: static struct dw_dma_slave = { .src_id = 1 };
    let mut lpt0_tx_param: static struct dw_dma_slave = { .dst_id = 2 };
    let mut lpt0_rx_param: static struct dw_dma_slave = { .src_id = 3 };
#[no_mangle]
unsafe extern "C" fn pxa2xx_spi_pci_clk_unregister(clk: *mut c_void) {
    static void pxa2xx_spi_pci_clk_unregister(void *clk)
    {
    clk_unregister(clk);
    }
    static int pxa2xx_spi_pci_clk_register(struct pci_dev *dev, struct ssp_device *ssp,
    unsigned long rate)
    {
    char buf[40];
    snprintf(buf, sizeof(buf), "pxa2xx-spi.%d", ssp.port_id);
    ssp.clk = clk_register_fixed_rate(&dev.dev, buf, core::ptr::null_mut(), 0, rate);
    if (IS_ERR(ssp.clk))
    return PTR_ERR(ssp.clk);
    return devm_add_action_or_reset(&dev.dev, pxa2xx_spi_pci_clk_unregister, ssp.clk);
    }
#[no_mangle]
unsafe extern "C" fn lpss_dma_filter(chan: *mut dma_chan, param: *mut c_void) -> bool {
    static bool lpss_dma_filter(struct dma_chan *chan, void *param)
    {
    struct dw_dma_slave *dws = param;
    if (dws.dma_dev != chan.device.dev)
    return false;
    chan.private = dws;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn lpss_dma_put_device(dma_dev: *mut c_void) {
    static void lpss_dma_put_device(void *dma_dev)
    {
    pci_dev_put(dma_dev);
    }
#[no_mangle]
unsafe extern "C" fn lpss_spi_setup(dev: *mut pci_dev, c: *mut pxa2xx_spi_controller) -> c_int {
    static int lpss_spi_setup(struct pci_dev *dev, struct pxa2xx_spi_controller *c)
    {
    struct ssp_device *ssp = &c.ssp;
    struct dw_dma_slave *tx, *rx;
    struct pci_dev *dma_dev;
    int ret;
    switch (dev.device) {
    case PCI_DEVICE_ID_INTEL_BYT:
    ssp.type = LPSS_BYT_SSP;
    ssp.port_id = 0;
    c.tx_param = &byt_tx_param;
    c.rx_param = &byt_rx_param;
    break;
    case PCI_DEVICE_ID_INTEL_BSW0:
    ssp.type = LPSS_BSW_SSP;
    ssp.port_id = 0;
    c.tx_param = &bsw0_tx_param;
    c.rx_param = &bsw0_rx_param;
    break;
    case PCI_DEVICE_ID_INTEL_BSW1:
    ssp.type = LPSS_BSW_SSP;
    ssp.port_id = 1;
    c.tx_param = &bsw1_tx_param;
    c.rx_param = &bsw1_rx_param;
    break;
    case PCI_DEVICE_ID_INTEL_BSW2:
    ssp.type = LPSS_BSW_SSP;
    ssp.port_id = 2;
    c.tx_param = &bsw2_tx_param;
    c.rx_param = &bsw2_rx_param;
    break;
    case PCI_DEVICE_ID_INTEL_LPT0_0:
    case PCI_DEVICE_ID_INTEL_LPT1_0:
    ssp.type = LPSS_LPT_SSP;
    ssp.port_id = 0;
    c.tx_param = &lpt0_tx_param;
    c.rx_param = &lpt0_rx_param;
    break;
    case PCI_DEVICE_ID_INTEL_LPT0_1:
    case PCI_DEVICE_ID_INTEL_LPT1_1:
    ssp.type = LPSS_LPT_SSP;
    ssp.port_id = 1;
    c.tx_param = &lpt1_tx_param;
    c.rx_param = &lpt1_rx_param;
    break;
    default:
    return -ENODEV;
    }
    c.num_chipselect = 1;
    ret = pxa2xx_spi_pci_clk_register(dev, ssp, 50000000);
    if (ret)
    return ret;
    dma_dev = pci_get_slot(dev.bus, PCI_DEVFN(PCI_SLOT(dev.devfn), 0));
    ret = devm_add_action_or_reset(&dev.dev, lpss_dma_put_device, dma_dev);
    if (ret)
    return ret;
    tx = c.tx_param;
    tx.dma_dev = &dma_dev.dev;
    tx.m_master = 0;
    tx.p_master = 1;
    rx = c.rx_param;
    rx.dma_dev = &dma_dev.dev;
    rx.m_master = 0;
    rx.p_master = 1;
    c.dma_filter = lpss_dma_filter;
    c.dma_burst_size = 1;
    c.enable_dma = 1;
    return 0;
    }
    static const struct pxa_spi_info lpss_info_config = {
    .setup = lpss_spi_setup,
    };
#[no_mangle]
unsafe extern "C" fn ce4100_spi_setup(dev: *mut pci_dev, c: *mut pxa2xx_spi_controller) -> c_int {
    static int ce4100_spi_setup(struct pci_dev *dev, struct pxa2xx_spi_controller *c)
    {
    struct ssp_device *ssp = &c.ssp;
    ssp.type = PXA25x_SSP;
    ssp.port_id = dev.devfn;
    c.num_chipselect = dev.devfn;
    return pxa2xx_spi_pci_clk_register(dev, ssp, 3686400);
    }
    static const struct pxa_spi_info ce4100_info_config = {
    .setup = ce4100_spi_setup,
    };
#[no_mangle]
unsafe extern "C" fn mrfld_spi_setup(dev: *mut pci_dev, c: *mut pxa2xx_spi_controller) -> c_int {
    static int mrfld_spi_setup(struct pci_dev *dev, struct pxa2xx_spi_controller *c)
    {
    struct ssp_device *ssp = &c.ssp;
    struct dw_dma_slave *tx, *rx;
    struct pci_dev *dma_dev;
    int ret;
    ssp.type = MRFLD_SSP;
    switch (PCI_FUNC(dev.devfn)) {
    case 0:
    ssp.port_id = 3;
    c.num_chipselect = 1;
    c.tx_param = &mrfld3_tx_param;
    c.rx_param = &mrfld3_rx_param;
    break;
    case 1:
    ssp.port_id = 5;
    c.num_chipselect = 4;
    c.tx_param = &mrfld5_tx_param;
    c.rx_param = &mrfld5_rx_param;
    break;
    case 2:
    ssp.port_id = 6;
    c.num_chipselect = 1;
    c.tx_param = &mrfld6_tx_param;
    c.rx_param = &mrfld6_rx_param;
    break;
    default:
    return -ENODEV;
    }
    ret = pxa2xx_spi_pci_clk_register(dev, ssp, 25000000);
    if (ret)
    return ret;
    dma_dev = pci_get_slot(dev.bus, PCI_DEVFN(21, 0));
    ret = devm_add_action_or_reset(&dev.dev, lpss_dma_put_device, dma_dev);
    if (ret)
    return ret;
    tx = c.tx_param;
    tx.dma_dev = &dma_dev.dev;
    rx = c.rx_param;
    rx.dma_dev = &dma_dev.dev;
    c.dma_filter = lpss_dma_filter;
    c.dma_burst_size = 8;
    c.enable_dma = 1;
    return 0;
    }
    static const struct pxa_spi_info mrfld_info_config = {
    .setup = mrfld_spi_setup,
    };
#[no_mangle]
unsafe extern "C" fn qrk_spi_setup(dev: *mut pci_dev, c: *mut pxa2xx_spi_controller) -> c_int {
    static int qrk_spi_setup(struct pci_dev *dev, struct pxa2xx_spi_controller *c)
    {
    struct ssp_device *ssp = &c.ssp;
    ssp.type = QUARK_X1000_SSP;
    ssp.port_id = dev.devfn;
    c.num_chipselect = 1;
    return pxa2xx_spi_pci_clk_register(dev, ssp, 50000000);
    }
    static const struct pxa_spi_info qrk_info_config = {
    .setup = qrk_spi_setup,
    };
    static int pxa2xx_spi_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *ent)
    {
    const struct pxa_spi_info *info;
    int ret;
    struct pxa2xx_spi_controller *pdata;
    struct ssp_device *ssp;
    ret = pcim_enable_device(dev);
    if (ret)
    return ret;
    pdata = devm_kzalloc(&dev.dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    ssp = &pdata.ssp;
    ssp.dev = &dev.dev;
    ssp.phys_base = pci_resource_start(dev, 0);
    ssp.mmio_base = pcim_iomap_region(dev, 0, "PXA2xx SPI");
    if (IS_ERR(ssp.mmio_base))
    return PTR_ERR(ssp.mmio_base);
    info = (struct pxa_spi_info *)ent.driver_data;
    ret = info.setup(dev, pdata);
    if (ret)
    return ret;
    pci_set_master(dev);
    ret = pci_alloc_irq_vectors(dev, 1, 1, PCI_IRQ_ALL_TYPES);
    if (ret < 0)
    return ret;
    ssp.irq = pci_irq_vector(dev, 0);
    ret = pxa2xx_spi_probe(&dev.dev, ssp, pdata);
    if (ret)
    return ret;
    pm_runtime_set_autosuspend_delay(&dev.dev, 50);
    pm_runtime_use_autosuspend(&dev.dev);
    pm_runtime_put_autosuspend(&dev.dev);
    pm_runtime_allow(&dev.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa2xx_spi_pci_remove(dev: *mut pci_dev) {
    static void pxa2xx_spi_pci_remove(struct pci_dev *dev)
    {
    pm_runtime_forbid(&dev.dev);
    pm_runtime_get_noresume(&dev.dev);
    pxa2xx_spi_remove(&dev.dev);
    }
    static const struct pci_device_id pxa2xx_spi_pci_devices[] = {
    { PCI_DEVICE_DATA(INTEL, QUARK_X1000, &qrk_info_config) },
    { PCI_DEVICE_DATA(INTEL, BYT, &lpss_info_config) },
    { PCI_DEVICE_DATA(INTEL, MRFLD, &mrfld_info_config) },
    { PCI_DEVICE_DATA(INTEL, BSW0, &lpss_info_config) },
    { PCI_DEVICE_DATA(INTEL, BSW1, &lpss_info_config) },
    { PCI_DEVICE_DATA(INTEL, BSW2, &lpss_info_config) },
    { PCI_DEVICE_DATA(INTEL, CE4100, &ce4100_info_config) },
    { PCI_DEVICE_DATA(INTEL, LPT0_0, &lpss_info_config) },
    { PCI_DEVICE_DATA(INTEL, LPT0_1, &lpss_info_config) },
    { PCI_DEVICE_DATA(INTEL, LPT1_0, &lpss_info_config) },
    { PCI_DEVICE_DATA(INTEL, LPT1_1, &lpss_info_config) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, pxa2xx_spi_pci_devices);
    static struct pci_driver pxa2xx_spi_pci_driver = {
    .name           = "pxa2xx_spi_pci",
    .id_table       = pxa2xx_spi_pci_devices,
    .driver = {
    .pm	= pm_ptr(&pxa2xx_spi_pm_ops),
    },
    .probe          = pxa2xx_spi_pci_probe,
    .remove         = pxa2xx_spi_pci_remove,
    };
    module_pci_driver(pxa2xx_spi_pci_driver);
    MODULE_DESCRIPTION("CE4100/LPSS PCI-SPI glue code for PXA's driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("SPI_PXA2xx");
    MODULE_AUTHOR("Sebastian Andrzej Siewior <bigeasy@linutronix.de>");
