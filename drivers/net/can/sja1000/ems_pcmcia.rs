//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/sja1000/ems_pcmcia.c
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
// Copyright (C) 2008 Sebastian Haas (initial chardev implementation)
// Copyright (C) 2010 Markus Plessing <plessing@ems-wuensche.com>
// Rework for mainline by Oliver Hartkopp <socketcan@hartkopp.net>
//

    MODULE_AUTHOR("Markus Plessing <plessing@ems-wuensche.com>");
    MODULE_DESCRIPTION("Socket-CAN driver for EMS CPC-CARD cards");
    MODULE_LICENSE("GPL v2");
pub const EMS_PCMCIA_MAX_CHAN: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ems_pcmcia_card {
    pub channels: c_int,
    pub pcmcia_dev: *mut pcmcia_device,
    pub net_dev: [*mut net_device; EMS_PCMCIA_MAX_CHAN],
    pub base_addr: *mut void __iomem,
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
// You will probably also want to set the clock divider value to 7
// (meaning direct oscillator output) because the second SJA1000 chip
// is driven by the first one CLKOUT output.
//

pub const EMS_PCMCIA_CAN_BASE_OFFSET: c_uint = 0x100 /* Offset where controllers starts */;
pub const EMS_PCMCIA_CAN_CTRL_SIZE: c_uint = 0x80 /* Memory size for each controller */;
pub const EMS_CMD_RESET: c_uint = 0x00 /* Perform a reset of the card */;
pub const EMS_CMD_MAP: c_uint = 0x03 /* Map CAN controllers into card' memory */;
pub const EMS_CMD_UMAP: c_uint = 0x02 /* Unmap CAN controllers from card' memory */;
    static struct pcmcia_device_id ems_pcmcia_tbl[] = {
    PCMCIA_DEVICE_PROD_ID123("EMS_T_W", "CPC-Card", "V2.0", 0xeab1ea23,
    0xa338573f, 0xe4575800),
    PCMCIA_DEVICE_NULL,
    };
    MODULE_DEVICE_TABLE(pcmcia, ems_pcmcia_tbl);
#[no_mangle]
unsafe extern "C" fn ems_pcmcia_read_reg(priv: *const sja1000_priv, port: c_int) -> u8 {
    static u8 ems_pcmcia_read_reg(const struct sja1000_priv *priv, int port)
    {
    return readb(priv.reg_base + port);
    }
    static void ems_pcmcia_write_reg(const struct sja1000_priv *priv, int port,
    u8 val)
    {
    writeb(val, priv.reg_base + port);
    }
#[no_mangle]
unsafe extern "C" fn ems_pcmcia_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ems_pcmcia_interrupt(int irq, void *dev_id)
    {
    struct ems_pcmcia_card *card = dev_id;
    struct net_device *dev;
    let mut retval: irqreturn_t = IRQ_NONE;
    int i, again;
// Card not present
    if (readw(card.base_addr) != 0xAA55)
    return IRQ_HANDLED;
    do {
    again = 0;
// Check interrupt for each channel
    for (i = 0; i < card.channels; i++) {
    dev = card.net_dev[i];
    if (!dev)
    continue;
    if (sja1000_interrupt(irq, dev) == IRQ_HANDLED)
    again = 1;
    }
// At least one channel handled the interrupt
    if (again)
    retval = IRQ_HANDLED;
    } while (again);
    return retval;
    }
//
// Check if a CAN controller is present at the specified location
// by trying to set 'em into the PeliCAN mode
//
#[no_mangle]
pub unsafe extern "C" fn ems_pcmcia_check_chan(priv: *mut sja1000_priv) -> c_int {
    static inline int ems_pcmcia_check_chan(struct sja1000_priv *priv)
    {
// Make sure SJA1000 is in reset mode
    ems_pcmcia_write_reg(priv, SJA1000_MOD, 1);
    ems_pcmcia_write_reg(priv, SJA1000_CDR, CDR_PELICAN);
// read reset-values
    if (ems_pcmcia_read_reg(priv, SJA1000_CDR) == CDR_PELICAN)
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ems_pcmcia_del_card(pdev: *mut pcmcia_device) {
    static void ems_pcmcia_del_card(struct pcmcia_device *pdev)
    {
    struct ems_pcmcia_card *card = pdev.priv;
    struct net_device *dev;
    int i;
    free_irq(pdev.irq, card);
    for (i = 0; i < card.channels; i++) {
    dev = card.net_dev[i];
    if (!dev)
    continue;
    printk(KERN_INFO "%s: removing %s on channel #%d\n",
    DRV_NAME, dev.name, i);
    unregister_sja1000dev(dev);
    free_sja1000dev(dev);
    }
    writeb(EMS_CMD_UMAP, card.base_addr);
    iounmap(card.base_addr);
    kfree(card);
    pdev.priv = core::ptr::null_mut();
    }
//
// Probe PCI device for EMS CAN signature and register each available
// CAN channel to SJA1000 Socket-CAN subsystem.
//
#[no_mangle]
unsafe extern "C" fn ems_pcmcia_add_card(pdev: *mut pcmcia_device, base: c_ulong) -> c_int {
    static int ems_pcmcia_add_card(struct pcmcia_device *pdev, unsigned long base)
    {
    struct sja1000_priv *priv;
    struct net_device *dev;
    struct ems_pcmcia_card *card;
    int err, i;
// Allocating card structures to hold addresses, ...
    card = kzalloc_obj(struct ems_pcmcia_card);
    if (!card)
    return -ENOMEM;
    pdev.priv = card;
    card.channels = 0;
    card.base_addr = ioremap(base, EMS_PCMCIA_MEM_SIZE);
    if (!card.base_addr) {
    err = -ENOMEM;
    goto failure_cleanup;
    }
// Check for unique EMS CAN signature
    if (readw(card.base_addr) != 0xAA55) {
    err = -ENODEV;
    goto failure_cleanup;
    }
// Request board reset
    writeb(EMS_CMD_RESET, card.base_addr);
// Make sure CAN controllers are mapped into card's memory space
    writeb(EMS_CMD_MAP, card.base_addr);
// Detect available channels
    for (i = 0; i < EMS_PCMCIA_MAX_CHAN; i++) {
    dev = alloc_sja1000dev(0);
    if (!dev) {
    err = -ENOMEM;
    goto failure_cleanup;
    }
    card.net_dev[i] = dev;
    priv = netdev_priv(dev);
    priv.priv = card;
    SET_NETDEV_DEV(dev, &pdev.dev);
    dev.dev_id = i;
    priv.irq_flags = IRQF_SHARED;
    dev.irq = pdev.irq;
    priv.reg_base = card.base_addr + EMS_PCMCIA_CAN_BASE_OFFSET +
    (i * EMS_PCMCIA_CAN_CTRL_SIZE);
// Check if channel is present
    if (ems_pcmcia_check_chan(priv)) {
    priv.read_reg  = ems_pcmcia_read_reg;
    priv.write_reg = ems_pcmcia_write_reg;
    priv.can.clock.freq = EMS_PCMCIA_CAN_CLOCK;
    priv.ocr = EMS_PCMCIA_OCR;
    priv.cdr = EMS_PCMCIA_CDR;
    priv.flags |= SJA1000_CUSTOM_IRQ_HANDLER;
// Register SJA1000 device
    err = register_sja1000dev(dev);
    if (err) {
    free_sja1000dev(dev);
    goto failure_cleanup;
    }
    card.channels++;
    printk(KERN_INFO "%s: registered %s on channel "
    "#%d at 0x%p, irq %d\n", DRV_NAME, dev.name,
    i, priv.reg_base, dev.irq);
    } else
    free_sja1000dev(dev);
    }
    if (!card.channels) {
    err = -ENODEV;
    goto failure_cleanup;
    }
    err = request_irq(pdev.irq, &ems_pcmcia_interrupt, IRQF_SHARED,
    DRV_NAME, card);
    if (!err)
    return 0;
    failure_cleanup:
    ems_pcmcia_del_card(pdev);
    return err;
    }
//
// Setup PCMCIA socket and probe for EMS CPC-CARD
//
#[no_mangle]
unsafe extern "C" fn ems_pcmcia_probe(dev: *mut pcmcia_device) -> c_int {
    static int ems_pcmcia_probe(struct pcmcia_device *dev)
    {
    int csval;
// General socket configuration
    dev.config_flags |= CONF_ENABLE_IRQ;
    dev.config_index = 1;
    dev.config_regs = PRESENT_OPTION;
// The io structure describes IO port mapping
    dev.resource[0].end = 16;
    dev.resource[0].flags |= IO_DATA_PATH_WIDTH_8;
    dev.resource[1].end = 16;
    dev.resource[1].flags |= IO_DATA_PATH_WIDTH_16;
    dev.io_lines = 5;
// Allocate a memory window
    dev.resource[2].flags =
    (WIN_DATA_WIDTH_8 | WIN_MEMORY_TYPE_CM | WIN_ENABLE);
    dev.resource[2].start = dev.resource[2].end = 0;
    csval = pcmcia_request_window(dev, dev.resource[2], 0);
    if (csval) {
    dev_err(&dev.dev, "pcmcia_request_window failed (err=%d)\n",
    csval);
    return 0;
    }
    csval = pcmcia_map_mem_page(dev, dev.resource[2], dev.config_base);
    if (csval) {
    dev_err(&dev.dev, "pcmcia_map_mem_page failed (err=%d)\n",
    csval);
    return 0;
    }
    csval = pcmcia_enable_device(dev);
    if (csval) {
    dev_err(&dev.dev, "pcmcia_enable_device failed (err=%d)\n",
    csval);
    return 0;
    }
    ems_pcmcia_add_card(dev, dev.resource[2].start);
    return 0;
    }
//
// Release claimed resources
//
#[no_mangle]
unsafe extern "C" fn ems_pcmcia_remove(dev: *mut pcmcia_device) {
    static void ems_pcmcia_remove(struct pcmcia_device *dev)
    {
    ems_pcmcia_del_card(dev);
    pcmcia_disable_device(dev);
    }
    static struct pcmcia_driver ems_pcmcia_driver = {
    .name = DRV_NAME,
    .probe = ems_pcmcia_probe,
    .remove = ems_pcmcia_remove,
    .id_table = ems_pcmcia_tbl,
    };
    module_pcmcia_driver(ems_pcmcia_driver);
