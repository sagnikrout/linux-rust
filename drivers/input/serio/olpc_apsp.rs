//! Automatically rewritten from C to Rust
//! Source: drivers/input/serio/olpc_apsp.c
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
// OLPC serio driver for multiplexed input from Marvell MMP security processor
//
// Copyright (C) 2011-2013 One Laptop Per Child
//

//
// The OLPC XO-1.75 and XO-4 laptops do not have a hardware PS/2 controller.
// Instead, the OLPC firmware runs a bit-banging PS/2 implementation on an
// otherwise-unused slow processor which is included in the Marvell MMP2/MMP3
// SoC, known as the "Security Processor" (SP) or "Wireless Trusted Module"
// (WTM). This firmware then reports its results via the WTM registers,
// which we read from the Application Processor (AP, i.e. main CPU) in this
// driver.
//
// On the hardware side we have a PS/2 mouse and an AT keyboard, the data
// is multiplexed through this system. We create a serio port for each one,
// and demultiplex the data accordingly.
//
// WTM register offsets
pub const SECURE_PROCESSOR_COMMAND: c_uint = 0x40;
pub const COMMAND_RETURN_STATUS: c_uint = 0x80;
pub const COMMAND_FIFO_STATUS: c_uint = 0xc4;
pub const PJ_RST_INTERRUPT: c_uint = 0xc8;
pub const PJ_INTERRUPT_MASK: c_uint = 0xcc;
//
// The upper byte of SECURE_PROCESSOR_COMMAND and COMMAND_RETURN_STATUS is
// used to identify which port (device) is being talked to. The lower byte
// is the data being sent/received.
//
pub const PORT_MASK: c_uint = 0xff00;
pub const DATA_MASK: c_uint = 0x00ff;
pub const PORT_SHIFT: c_int = 8;
pub const KEYBOARD_PORT: c_int = 0;
pub const TOUCHPAD_PORT: c_int = 1;
// COMMAND_FIFO_STATUS
pub const CMD_CNTR_MASK: c_uint = 0x7 /* Number of pending/unprocessed commands */;

// PJ_RST_INTERRUPT
pub const SP_COMMAND_COMPLETE_RESET: c_uint = 0x1;
// PJ_INTERRUPT_MASK

// COMMAND_FIFO_STATUS
pub const CMD_STS_MASK: c_uint = 0x100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct olpc_apsp {
    pub dev: *mut device,
    pub kbio: *mut serio,
    pub padio: *mut serio,
    pub base: *mut void __iomem,
    pub open_count: c_int,
    pub irq: c_int,
}

#[no_mangle]
unsafe extern "C" fn olpc_apsp_write(port: *mut serio, val: c_uchar) -> c_int {
    static int olpc_apsp_write(struct serio *port, unsigned char val)
    {
    struct olpc_apsp *priv = port.port_data;
    unsigned int i;
    let mut which: u32 = 0;
    if (port == priv.padio)
    which = TOUCHPAD_PORT << PORT_SHIFT;
    else
    which = KEYBOARD_PORT << PORT_SHIFT;
    dev_dbg(priv.dev, "olpc_apsp_write which=%x val=%x\n", which, val);
    for (i = 0; i < 50; i++) {
    let mut sts: u32 = readl(priv.base + COMMAND_FIFO_STATUS);
    if ((sts & CMD_CNTR_MASK) < MAX_PENDING_CMDS) {
    writel(which | val,
    priv.base + SECURE_PROCESSOR_COMMAND);
    return 0;
    }
// SP busy. This has not been seen in practice.
    mdelay(1);
    }
    dev_dbg(priv.dev, "olpc_apsp_write timeout, status=%x\n",
    readl(priv.base + COMMAND_FIFO_STATUS));
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn olpc_apsp_rx(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t olpc_apsp_rx(int irq, void *dev_id)
    {
    struct olpc_apsp *priv = dev_id;
    unsigned int w, tmp;
    struct serio *serio;
//
// Write 1 to PJ_RST_INTERRUPT to acknowledge and clear the interrupt
// Write 0xff00 to SECURE_PROCESSOR_COMMAND.
//
    tmp = readl(priv.base + PJ_RST_INTERRUPT);
    if (!(tmp & SP_COMMAND_COMPLETE_RESET)) {
    dev_warn(priv.dev, "spurious interrupt?\n");
    return IRQ_NONE;
    }
    w = readl(priv.base + COMMAND_RETURN_STATUS);
    dev_dbg(priv.dev, "olpc_apsp_rx %x\n", w);
    if (w >> PORT_SHIFT == KEYBOARD_PORT)
    serio = priv.kbio;
    else
    serio = priv.padio;
    serio_interrupt(serio, w & DATA_MASK, 0);
// Ack and clear interrupt
    writel(tmp | SP_COMMAND_COMPLETE_RESET, priv.base + PJ_RST_INTERRUPT);
    writel(PORT_MASK, priv.base + SECURE_PROCESSOR_COMMAND);
    pm_wakeup_event(priv.dev, 1000);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn olpc_apsp_open(port: *mut serio) -> c_int {
    static int olpc_apsp_open(struct serio *port)
    {
    struct olpc_apsp *priv = port.port_data;
    unsigned int tmp;
    unsigned long l;
    if (priv.open_count++ == 0) {
    l = readl(priv.base + COMMAND_FIFO_STATUS);
    if (!(l & CMD_STS_MASK)) {
    dev_err(priv.dev, "SP cannot accept commands.\n");
    return -EIO;
    }
// Enable interrupt 0 by clearing its bit
    tmp = readl(priv.base + PJ_INTERRUPT_MASK);
    writel(tmp & ~INT_0, priv.base + PJ_INTERRUPT_MASK);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn olpc_apsp_close(port: *mut serio) {
    static void olpc_apsp_close(struct serio *port)
    {
    struct olpc_apsp *priv = port.port_data;
    unsigned int tmp;
    if (--priv.open_count == 0) {
// Disable interrupt 0
    tmp = readl(priv.base + PJ_INTERRUPT_MASK);
    writel(tmp | INT_0, priv.base + PJ_INTERRUPT_MASK);
    }
    }
#[no_mangle]
unsafe extern "C" fn olpc_apsp_probe(pdev: *mut platform_device) -> c_int {
    static int olpc_apsp_probe(struct platform_device *pdev)
    {
    struct serio *kb_serio, *pad_serio;
    struct olpc_apsp *priv;
    int error;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = &pdev.dev;
    priv.base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(priv.base)) {
    dev_err(&pdev.dev, "Failed to map WTM registers\n");
    return PTR_ERR(priv.base);
    }
    priv.irq = platform_get_irq(pdev, 0);
    if (priv.irq < 0)
    return priv.irq;
// KEYBOARD
    kb_serio = kzalloc_obj(*kb_serio);
    if (!kb_serio)
    return -ENOMEM;
    kb_serio.id.type	= SERIO_8042_XL;
    kb_serio.write		= olpc_apsp_write;
    kb_serio.open		= olpc_apsp_open;
    kb_serio.close		= olpc_apsp_close;
    kb_serio.port_data	= priv;
    kb_serio.dev.parent	= &pdev.dev;
    strscpy(kb_serio.name, "sp keyboard", sizeof(kb_serio.name));
    strscpy(kb_serio.phys, "sp/serio0", sizeof(kb_serio.phys));
    priv.kbio		= kb_serio;
    serio_register_port(kb_serio);
// TOUCHPAD
    pad_serio = kzalloc_obj(*pad_serio);
    if (!pad_serio) {
    error = -ENOMEM;
    goto err_pad;
    }
    pad_serio.id.type	= SERIO_8042;
    pad_serio.write	= olpc_apsp_write;
    pad_serio.open		= olpc_apsp_open;
    pad_serio.close	= olpc_apsp_close;
    pad_serio.port_data	= priv;
    pad_serio.dev.parent	= &pdev.dev;
    strscpy(pad_serio.name, "sp touchpad", sizeof(pad_serio.name));
    strscpy(pad_serio.phys, "sp/serio1", sizeof(pad_serio.phys));
    priv.padio		= pad_serio;
    serio_register_port(pad_serio);
    error = request_irq(priv.irq, olpc_apsp_rx, 0, "olpc-apsp", priv);
    if (error) {
    dev_err(&pdev.dev, "Failed to request IRQ\n");
    goto err_irq;
    }
    device_init_wakeup(priv.dev, 1);
    platform_set_drvdata(pdev, priv);
    dev_dbg(&pdev.dev, "probed successfully.\n");
    return 0;
    err_irq:
    serio_unregister_port(pad_serio);
    err_pad:
    serio_unregister_port(kb_serio);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn olpc_apsp_remove(pdev: *mut platform_device) {
    static void olpc_apsp_remove(struct platform_device *pdev)
    {
    struct olpc_apsp *priv = platform_get_drvdata(pdev);
    free_irq(priv.irq, priv);
    serio_unregister_port(priv.kbio);
    serio_unregister_port(priv.padio);
    }
    static const struct of_device_id olpc_apsp_dt_ids[] = {
    { .compatible = "olpc,ap-sp", },
    {}
    };
    MODULE_DEVICE_TABLE(of, olpc_apsp_dt_ids);
    static struct platform_driver olpc_apsp_driver = {
    .probe		= olpc_apsp_probe,
    .remove		= olpc_apsp_remove,
    .driver		= {
    .name	= "olpc-apsp",
    .of_match_table = olpc_apsp_dt_ids,
    },
    };
    MODULE_DESCRIPTION("OLPC AP-SP serio driver");
    MODULE_LICENSE("GPL");
    module_platform_driver(olpc_apsp_driver);
