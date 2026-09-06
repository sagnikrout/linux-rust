//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/sja1000/kvaser_pci.c
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
// Copyright (C) 2008 Per Dalen <per.dalen@cnw.se>
//
// Parts of this software are based on (derived) the following:
//
// - Kvaser linux driver, version 4.72 BETA
// Copyright (C) 2002-2007 KVASER AB
//
// - Lincan driver, version 0.3.3, OCERA project
// Copyright (C) 2004 Pavel Pisa
// Copyright (C) 2001 Arnaud Westenberg
//
// - Socketcan SJA1000 drivers
// Copyright (C) 2007 Wolfgang Grandegger <wg@grandegger.com>
// Copyright (c) 2002-2007 Volkswagen Group Electronic Research
// Copyright (c) 2003 Matthias Brukner, Trajet Gmbh, Rebenring 33,
// 38106 Braunschweig, GERMANY
//

    MODULE_AUTHOR("Per Dalen <per.dalen@cnw.se>");
    MODULE_DESCRIPTION("Socket-CAN driver for KVASER PCAN PCI cards");
    MODULE_LICENSE("GPL v2");

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvaser_pci {
    pub channel: c_int,
    pub pci_dev: *mut pci_dev,
    pub slave_dev: [*mut net_device; MAX_NO_OF_CHANNELS-1],
    pub conf_addr: *mut void __iomem,
    pub res_addr: *mut void __iomem,
    pub no_channels: c_int,
    pub xilinx_ver: u8,
}

//
// The board configuration is probably following:
// RX1 is connected to ground.
// TX1 is not connected.
// CLKO is not connected.
// Setting the OCR register to 0xDA is a good idea.
// This means  normal output mode , push-pull and the correct polarity.
//

//
// In the CDR register, you should set CBP to 1.
// You will probably also want to set the clock divider value to 0
// (meaning divide-by-2), the Pelican bit, and the clock-off bit
// (you will have no need for CLKOUT anyway).
//

//
// These register values are valid for revision 14 of the Xilinx logic.
//

    high nibble version number. */
pub const XILINX_PRESUMED_VERSION: c_int = 14;
//
// Important S5920 registers
//
pub const S5920_INTCSR: c_uint = 0x38;
pub const S5920_PTCR: c_uint = 0x60;
pub const INTCSR_ADDON_INTENABLE_M: c_uint = 0x2000;
pub const KVASER_PCI_PORT_BYTES: c_uint = 0x20;
pub const PCI_CONFIG_PORT_SIZE: c_uint = 0x80      /* size of the config io-memory */;
pub const PCI_PORT_SIZE: c_uint = 0x80      /* size of a channel io-memory */;
pub const PCI_PORT_XILINX_SIZE: c_uint = 0x08      /* size of a xilinx io-memory */;
pub const KVASER_PCI_VENDOR_ID1: c_uint = 0x10e8    /* the PCI device and vendor IDs */;
pub const KVASER_PCI_DEVICE_ID1: c_uint = 0x8406;
pub const KVASER_PCI_VENDOR_ID2: c_uint = 0x1a07    /* the PCI device and vendor IDs */;
pub const KVASER_PCI_DEVICE_ID2: c_uint = 0x0008;
    static const struct pci_device_id kvaser_pci_tbl[] = {
    {KVASER_PCI_VENDOR_ID1, KVASER_PCI_DEVICE_ID1, PCI_ANY_ID, PCI_ANY_ID,},
    {KVASER_PCI_VENDOR_ID2, KVASER_PCI_DEVICE_ID2, PCI_ANY_ID, PCI_ANY_ID,},
    { 0,}
    };
    MODULE_DEVICE_TABLE(pci, kvaser_pci_tbl);
#[no_mangle]
unsafe extern "C" fn kvaser_pci_read_reg(priv: *const sja1000_priv, port: c_int) -> u8 {
    static u8 kvaser_pci_read_reg(const struct sja1000_priv *priv, int port)
    {
    return ioread8(priv.reg_base + port);
    }
    static void kvaser_pci_write_reg(const struct sja1000_priv *priv,
    int port, u8 val)
    {
    iowrite8(val, priv.reg_base + port);
    }
#[no_mangle]
unsafe extern "C" fn kvaser_pci_disable_irq(dev: *mut net_device) {
    static void kvaser_pci_disable_irq(struct net_device *dev)
    {
    struct sja1000_priv *priv = netdev_priv(dev);
    struct kvaser_pci *board = priv.priv;
    u32 intcsr;
// Disable interrupts from card
    intcsr = ioread32(board.conf_addr + S5920_INTCSR);
    intcsr &= ~INTCSR_ADDON_INTENABLE_M;
    iowrite32(intcsr, board.conf_addr + S5920_INTCSR);
    }
#[no_mangle]
unsafe extern "C" fn kvaser_pci_enable_irq(dev: *mut net_device) {
    static void kvaser_pci_enable_irq(struct net_device *dev)
    {
    struct sja1000_priv *priv = netdev_priv(dev);
    struct kvaser_pci *board = priv.priv;
    u32 tmp_en_io;
// Enable interrupts from card
    tmp_en_io = ioread32(board.conf_addr + S5920_INTCSR);
    tmp_en_io |= INTCSR_ADDON_INTENABLE_M;
    iowrite32(tmp_en_io, board.conf_addr + S5920_INTCSR);
    }
#[no_mangle]
unsafe extern "C" fn number_of_sja1000_chip(base_addr: *mut void __iomem) -> c_int {
    static int number_of_sja1000_chip(void __iomem *base_addr)
    {
    u8 status;
    int i;
    for (i = 0; i < MAX_NO_OF_CHANNELS; i++) {
// reset chip
    iowrite8(MOD_RM, base_addr +
    (i * KVASER_PCI_PORT_BYTES) + SJA1000_MOD);
    status = ioread8(base_addr +
    (i * KVASER_PCI_PORT_BYTES) + SJA1000_MOD);
// check reset bit
    if (!(status & MOD_RM))
    break;
    }
    return i;
    }
#[no_mangle]
unsafe extern "C" fn kvaser_pci_del_chan(dev: *mut net_device) {
    static void kvaser_pci_del_chan(struct net_device *dev)
    {
    struct sja1000_priv *priv;
    struct kvaser_pci *board;
    int i;
    if (!dev)
    return;
    priv = netdev_priv(dev);
    board = priv.priv;
    if (!board)
    return;
    dev_info(&board.pci_dev.dev, "Removing device %s\n",
    dev.name);
// Disable PCI interrupts
    kvaser_pci_disable_irq(dev);
    for (i = 0; i < board.no_channels - 1; i++) {
    if (board.slave_dev[i]) {
    dev_info(&board.pci_dev.dev, "Removing device %s\n",
    board.slave_dev[i].name);
    unregister_sja1000dev(board.slave_dev[i]);
    free_sja1000dev(board.slave_dev[i]);
    }
    }
    unregister_sja1000dev(dev);
    pci_iounmap(board.pci_dev, priv.reg_base);
    pci_iounmap(board.pci_dev, board.conf_addr);
    pci_iounmap(board.pci_dev, board.res_addr);
    free_sja1000dev(dev);
    }
    static int kvaser_pci_add_chan(struct pci_dev *pdev, int channel,
    struct net_device **master_dev,
    void __iomem *conf_addr,
    void __iomem *res_addr,
    void __iomem *base_addr)
    {
    struct net_device *dev;
    struct sja1000_priv *priv;
    struct kvaser_pci *board;
    int err;
    dev = alloc_sja1000dev(sizeof(struct kvaser_pci));
    if (dev == core::ptr::null_mut())
    return -ENOMEM;
    priv = netdev_priv(dev);
    board = priv.priv;
    board.pci_dev = pdev;
    board.channel = channel;
// S5920
    board.conf_addr = conf_addr;
// XILINX board wide address
    board.res_addr = res_addr;
    if (channel == 0) {
    board.xilinx_ver =
    ioread8(board.res_addr + XILINX_VERINT) >> 4;
// Assert PTADR# - we're in passive mode so the other bits are
    not important */
    iowrite32(0x80808080UL, board.conf_addr + S5920_PTCR);
// Enable interrupts from card
    kvaser_pci_enable_irq(dev);
    } else {
    struct sja1000_priv *master_priv = netdev_priv(*master_dev);
    struct kvaser_pci *master_board = master_priv.priv;
    master_board.slave_dev[channel - 1] = dev;
    master_board.no_channels = channel + 1;
    board.xilinx_ver = master_board.xilinx_ver;
    }
    priv.reg_base = base_addr + channel * KVASER_PCI_PORT_BYTES;
    priv.read_reg = kvaser_pci_read_reg;
    priv.write_reg = kvaser_pci_write_reg;
    priv.can.clock.freq = KVASER_PCI_CAN_CLOCK;
    priv.ocr = KVASER_PCI_OCR;
    priv.cdr = KVASER_PCI_CDR;
    priv.irq_flags = IRQF_SHARED;
    dev.irq = pdev.irq;
    dev_info(&pdev.dev, "reg_base=%p conf_addr=%p irq=%d\n",
    priv.reg_base, board.conf_addr, dev.irq);
    SET_NETDEV_DEV(dev, &pdev.dev);
    dev.dev_id = channel;
// Register SJA1000 device
    err = register_sja1000dev(dev);
    if (err) {
    dev_err(&pdev.dev, "Registering device failed (err=%d)\n",
    err);
    goto failure;
    }
    if (channel == 0)
// master_dev = dev;
    return 0;
    failure:
    kvaser_pci_del_chan(dev);
    return err;
    }
    static int kvaser_pci_init_one(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    int err;
    struct net_device *master_dev = core::ptr::null_mut();
    struct sja1000_priv *priv;
    struct kvaser_pci *board;
    int no_channels;
    void __iomem *base_addr = core::ptr::null_mut();
    void __iomem *conf_addr = core::ptr::null_mut();
    void __iomem *res_addr = core::ptr::null_mut();
    int i;
    dev_info(&pdev.dev, "initializing device %04x:%04x\n",
    pdev.vendor, pdev.device);
    err = pci_enable_device(pdev);
    if (err)
    goto failure;
    err = pci_request_regions(pdev, DRV_NAME);
    if (err)
    goto failure_release_pci;
// S5920
    conf_addr = pci_iomap(pdev, 0, PCI_CONFIG_PORT_SIZE);
    if (conf_addr == core::ptr::null_mut()) {
    err = -ENODEV;
    goto failure_release_regions;
    }
// XILINX board wide address
    res_addr = pci_iomap(pdev, 2, PCI_PORT_XILINX_SIZE);
    if (res_addr == core::ptr::null_mut()) {
    err = -ENOMEM;
    goto failure_iounmap;
    }
    base_addr = pci_iomap(pdev, 1, PCI_PORT_SIZE);
    if (base_addr == core::ptr::null_mut()) {
    err = -ENOMEM;
    goto failure_iounmap;
    }
    no_channels = number_of_sja1000_chip(base_addr);
    if (no_channels == 0) {
    err = -ENOMEM;
    goto failure_iounmap;
    }
    for (i = 0; i < no_channels; i++) {
    err = kvaser_pci_add_chan(pdev, i, &master_dev,
    conf_addr, res_addr,
    base_addr);
    if (err)
    goto failure_cleanup;
    }
    priv = netdev_priv(master_dev);
    board = priv.priv;
    dev_info(&pdev.dev, "xilinx version=%d number of channels=%d\n",
    board.xilinx_ver, board.no_channels);
    pci_set_drvdata(pdev, master_dev);
    return 0;
    failure_cleanup:
    kvaser_pci_del_chan(master_dev);
    failure_iounmap:
    if (conf_addr != core::ptr::null_mut())
    pci_iounmap(pdev, conf_addr);
    if (res_addr != core::ptr::null_mut())
    pci_iounmap(pdev, res_addr);
    if (base_addr != core::ptr::null_mut())
    pci_iounmap(pdev, base_addr);
    failure_release_regions:
    pci_release_regions(pdev);
    failure_release_pci:
    pci_disable_device(pdev);
    failure:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn kvaser_pci_remove_one(pdev: *mut pci_dev) {
    static void kvaser_pci_remove_one(struct pci_dev *pdev)
    {
    struct net_device *dev = pci_get_drvdata(pdev);
    kvaser_pci_del_chan(dev);
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    }
    static struct pci_driver kvaser_pci_driver = {
    .name = DRV_NAME,
    .id_table = kvaser_pci_tbl,
    .probe = kvaser_pci_init_one,
    .remove = kvaser_pci_remove_one,
    };
    module_pci_driver(kvaser_pci_driver);
