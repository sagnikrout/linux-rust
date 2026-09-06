//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-dw-pci.c
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
// PCI interface driver for DW SPI Core
//
// Copyright (c) 2009, 2014 Intel Corporation.
//

// HW info for MRST Clk Control Unit, 32b reg per controller

pub const MRST_CLK_SPI_REG: c_uint = 0xff11d86c;
pub const CLK_SPI_BDIV_OFFSET: c_int = 0;
pub const CLK_SPI_BDIV_MASK: c_uint = 0x00000007;
pub const CLK_SPI_CDIV_OFFSET: c_int = 9;
pub const CLK_SPI_CDIV_MASK: c_uint = 0x00000e00;
pub const CLK_SPI_DISABLE_OFFSET: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_spi_pci_desc {
    pub ): *mut *mut int (setup)(struct dw_spi,
    pub num_cs: u16,
    pub bus_num: u16,
    pub max_freq: u32,
}

#[no_mangle]
unsafe extern "C" fn dw_spi_pci_mid_init(dws: *mut dw_spi) -> c_int {
    static int dw_spi_pci_mid_init(struct dw_spi *dws)
    {
    void __iomem *clk_reg;
    u32 clk_cdiv;
    clk_reg = ioremap(MRST_CLK_SPI_REG, 16);
    if (!clk_reg)
    return -ENOMEM;
// Get SPI controller operating freq info
    clk_cdiv = readl(clk_reg + dws.bus_num * sizeof(u32));
    clk_cdiv &= CLK_SPI_CDIV_MASK;
    clk_cdiv >>= CLK_SPI_CDIV_OFFSET;
    dws.max_freq = MRST_SPI_CLK_BASE / (clk_cdiv + 1);
    iounmap(clk_reg);
    dw_spi_dma_setup_mfld(dws);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_spi_pci_generic_init(dws: *mut dw_spi) -> c_int {
    static int dw_spi_pci_generic_init(struct dw_spi *dws)
    {
    dw_spi_dma_setup_generic(dws);
    return 0;
    }
    static struct dw_spi_pci_desc dw_spi_pci_mid_desc_1 = {
    .setup = dw_spi_pci_mid_init,
    .num_cs = 5,
    .bus_num = 0,
    };
    static struct dw_spi_pci_desc dw_spi_pci_mid_desc_2 = {
    .setup = dw_spi_pci_mid_init,
    .num_cs = 2,
    .bus_num = 1,
    };
    static struct dw_spi_pci_desc dw_spi_pci_ehl_desc = {
    .setup = dw_spi_pci_generic_init,
    .num_cs = 2,
    .bus_num = -1,
    .max_freq = 100000000,
    };
#[no_mangle]
unsafe extern "C" fn dw_spi_pci_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int dw_spi_pci_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    struct dw_spi_pci_desc *desc = (struct dw_spi_pci_desc *)ent.driver_data;
    struct dw_spi *dws;
    let mut pci_bar: c_int = 0;
    int ret;
    ret = pcim_enable_device(pdev);
    if (ret)
    return ret;
    dws = devm_kzalloc(&pdev.dev, sizeof(*dws), GFP_KERNEL);
    if (!dws)
    return -ENOMEM;
// Get basic io resource and map it
    dws.paddr = pci_resource_start(pdev, pci_bar);
    pci_set_master(pdev);
    ret = pci_alloc_irq_vectors(pdev, 1, 1, PCI_IRQ_ALL_TYPES);
    if (ret < 0)
    return ret;
    dws.regs = pcim_iomap_region(pdev, pci_bar, pci_name(pdev));
    if (IS_ERR(dws.regs))
    return PTR_ERR(dws.regs);
    dws.irq = pci_irq_vector(pdev, 0);
//
// Specific handling for platforms, like dma setup,
// clock rate, FIFO depth.
//
    if (desc) {
    dws.num_cs = desc.num_cs;
    dws.bus_num = desc.bus_num;
    dws.max_freq = desc.max_freq;
    if (desc.setup) {
    ret = desc.setup(dws);
    if (ret)
    return ret;
    }
    } else {
    return -ENODEV;
    }
    ret = dw_spi_add_controller(&pdev.dev, dws);
    if (ret)
    return ret;
// PCI hook and SPI hook use the same drv data
    pci_set_drvdata(pdev, dws);
    dev_info(&pdev.dev, "found PCI SPI controller(ID: %04x:%04x)\n",
    pdev.vendor, pdev.device);
    pm_runtime_set_autosuspend_delay(&pdev.dev, 1000);
    pm_runtime_use_autosuspend(&pdev.dev);
    pm_runtime_put_autosuspend(&pdev.dev);
    pm_runtime_allow(&pdev.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_spi_pci_remove(pdev: *mut pci_dev) {
    static void dw_spi_pci_remove(struct pci_dev *pdev)
    {
    struct dw_spi *dws = pci_get_drvdata(pdev);
    pm_runtime_forbid(&pdev.dev);
    pm_runtime_get_noresume(&pdev.dev);
    dw_spi_remove_controller(dws);
    }
#[no_mangle]
unsafe extern "C" fn dw_spi_pci_suspend(dev: *mut device) -> c_int {
    static int dw_spi_pci_suspend(struct device *dev)
    {
    struct dw_spi *dws = dev_get_drvdata(dev);
    return dw_spi_suspend_controller(dws);
    }
#[no_mangle]
unsafe extern "C" fn dw_spi_pci_resume(dev: *mut device) -> c_int {
    static int dw_spi_pci_resume(struct device *dev)
    {
    struct dw_spi *dws = dev_get_drvdata(dev);
    return dw_spi_resume_controller(dws);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(dw_spi_pci_pm_ops, dw_spi_pci_suspend, dw_spi_pci_resume);
    static const struct pci_device_id dw_spi_pci_ids[] = {
// Intel MID platform SPI controller 0
//
// The access to the device 8086:0801 is disabled by HW, since it's
// exclusively used by SCU to communicate with MSIC.
//
// Intel MID platform SPI controller 1
    { PCI_VDEVICE(INTEL, 0x0800), .driver_data = (kernel_ulong_t)&dw_spi_pci_mid_desc_1 },
// Intel MID platform SPI controller 2
    { PCI_VDEVICE(INTEL, 0x0812), .driver_data = (kernel_ulong_t)&dw_spi_pci_mid_desc_2 },
// Intel Elkhart Lake PSE SPI controllers
    { PCI_VDEVICE(INTEL, 0x4b84), .driver_data = (kernel_ulong_t)&dw_spi_pci_ehl_desc },
    { PCI_VDEVICE(INTEL, 0x4b85), .driver_data = (kernel_ulong_t)&dw_spi_pci_ehl_desc },
    { PCI_VDEVICE(INTEL, 0x4b86), .driver_data = (kernel_ulong_t)&dw_spi_pci_ehl_desc },
    { PCI_VDEVICE(INTEL, 0x4b87), .driver_data = (kernel_ulong_t)&dw_spi_pci_ehl_desc },
    { },
    };
    MODULE_DEVICE_TABLE(pci, dw_spi_pci_ids);
    static struct pci_driver dw_spi_pci_driver = {
    .name =		DRIVER_NAME,
    .id_table =	dw_spi_pci_ids,
    .probe =	dw_spi_pci_probe,
    .remove =	dw_spi_pci_remove,
    .driver         = {
    .pm     = pm_sleep_ptr(&dw_spi_pci_pm_ops),
    },
    };
    module_pci_driver(dw_spi_pci_driver);
    MODULE_AUTHOR("Feng Tang <feng.tang@intel.com>");
    MODULE_DESCRIPTION("PCI interface driver for DW SPI Core");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("SPI_DW_CORE");
