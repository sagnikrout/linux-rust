//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-cavium-thunderx.c
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
// Cavium ThunderX SPI driver.
//
// Copyright (C) 2016 Cavium Inc.
// Authors: Jan Glauber <jglauber@cavium.com>
//

    static int thunderx_spi_probe(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    struct device *dev = &pdev.dev;
    struct spi_controller *host;
    struct octeon_spi *p;
    int ret;
    host = devm_spi_alloc_host(dev, sizeof(struct octeon_spi));
    if (!host)
    return -ENOMEM;
    p = spi_controller_get_devdata(host);
    ret = pcim_enable_device(pdev);
    if (ret)
    return ret;
    ret = pcim_request_all_regions(pdev, DRV_NAME);
    if (ret)
    return ret;
    p.register_base = pcim_iomap(pdev, 0, pci_resource_len(pdev, 0));
    if (!p.register_base)
    return -EINVAL;
    p.regs.config = 0x1000;
    p.regs.status = 0x1008;
    p.regs.tx = 0x1010;
    p.regs.data = 0x1080;
    p.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(p.clk))
    return PTR_ERR(p.clk);
    p.sys_freq = clk_get_rate(p.clk);
    if (!p.sys_freq)
    p.sys_freq = SYS_FREQ_DEFAULT;
    dev_info(dev, "Set system clock to %u\n", p.sys_freq);
    host.flags = SPI_CONTROLLER_HALF_DUPLEX;
    host.num_chipselect = 4;
    host.mode_bits = SPI_CPHA | SPI_CPOL | SPI_CS_HIGH |
    SPI_LSB_FIRST | SPI_3WIRE;
    host.transfer_one_message = octeon_spi_transfer_one_message;
    host.bits_per_word_mask = SPI_BPW_MASK(8);
    host.max_speed_hz = OCTEON_SPI_MAX_CLOCK_HZ;
    pci_set_drvdata(pdev, host);
    return spi_register_controller(host);
    }
#[no_mangle]
unsafe extern "C" fn thunderx_spi_remove(pdev: *mut pci_dev) {
    static void thunderx_spi_remove(struct pci_dev *pdev)
    {
    struct spi_controller *host = pci_get_drvdata(pdev);
    struct octeon_spi *p;
    p = spi_controller_get_devdata(host);
    if (!p)
    return;
    spi_unregister_controller(host);
// Put everything in a known state.
    writeq(0, p.register_base + OCTEON_SPI_CFG(p));
    }
    static const struct pci_device_id thunderx_spi_pci_id_table[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_CAVIUM, 0xa00b) },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, thunderx_spi_pci_id_table);
    static struct pci_driver thunderx_spi_driver = {
    .name		= DRV_NAME,
    .id_table	= thunderx_spi_pci_id_table,
    .probe		= thunderx_spi_probe,
    .remove		= thunderx_spi_remove,
    };
    module_pci_driver(thunderx_spi_driver);
    MODULE_DESCRIPTION("Cavium, Inc. ThunderX SPI bus driver");
    MODULE_AUTHOR("Jan Glauber");
    MODULE_LICENSE("GPL");
