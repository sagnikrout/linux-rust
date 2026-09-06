//! Automatically rewritten from C to Rust
//! Source: drivers/input/serio/arc_ps2.c
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
// Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
//
// Driver is originally developed by Pavel Sokolov <psokolov@synopsys.com>
//

pub const ARC_PS2_PORTS: c_int = 2;
pub const ARC_ARC_PS2_ID: c_uint = 0x0001f609;
pub const STAT_TIMEOUT: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arc_ps2_port {
    pub data_addr: *mut void __iomem,
    pub status_addr: *mut void __iomem,
    pub io: *mut serio,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arc_ps2_data {
    pub port: [arc_ps2_port; ARC_PS2_PORTS],
    pub addr: *mut void __iomem,
    pub frame_error: c_uint,
    pub buf_overflow: c_uint,
    pub total_int: c_uint,
}

    static void arc_ps2_check_rx(struct arc_ps2_data *arc_ps2,
    struct arc_ps2_port *port)
    {
    let mut timeout: c_uint = 1000;
    unsigned int flag, status;
    unsigned char data;
    do {
    status = ioread32(port.status_addr);
    if (!(status & PS2_STAT_RX_VAL))
    return;
    data = ioread32(port.data_addr) & 0xff;
    flag = 0;
    arc_ps2.total_int++;
    if (status & PS2_STAT_RX_FRM_ERR) {
    arc_ps2.frame_error++;
    flag |= SERIO_PARITY;
    } else if (status & PS2_STAT_RX_BUF_OVER) {
    arc_ps2.buf_overflow++;
    flag |= SERIO_FRAME;
    }
    serio_interrupt(port.io, data, flag);
    } while (--timeout);
    dev_err(&port.io.dev, "PS/2 hardware stuck\n");
    }
#[no_mangle]
unsafe extern "C" fn arc_ps2_interrupt(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t arc_ps2_interrupt(int irq, void *dev)
    {
    struct arc_ps2_data *arc_ps2 = dev;
    int i;
    for (i = 0; i < ARC_PS2_PORTS; i++)
    arc_ps2_check_rx(arc_ps2, &arc_ps2.port[i]);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn arc_ps2_write(io: *mut serio, val: c_uchar) -> c_int {
    static int arc_ps2_write(struct serio *io, unsigned char val)
    {
    unsigned status;
    struct arc_ps2_port *port = io.port_data;
    let mut timeout: c_int = STAT_TIMEOUT;
    do {
    status = ioread32(port.status_addr);
    cpu_relax();
    if (status & PS2_STAT_TX_ISNOT_FUL) {
    iowrite32(val & 0xff, port.data_addr);
    return 0;
    }
    } while (--timeout);
    dev_err(&io.dev, "write timeout\n");
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn arc_ps2_open(io: *mut serio) -> c_int {
    static int arc_ps2_open(struct serio *io)
    {
    struct arc_ps2_port *port = io.port_data;
    iowrite32(PS2_STAT_RX_INT_EN, port.status_addr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arc_ps2_close(io: *mut serio) {
    static void arc_ps2_close(struct serio *io)
    {
    struct arc_ps2_port *port = io.port_data;
    iowrite32(ioread32(port.status_addr) & ~PS2_STAT_RX_INT_EN,
    port.status_addr);
    }
    static void __iomem *arc_ps2_calc_addr(struct arc_ps2_data *arc_ps2,
    int index, bool status)
    {
    void __iomem *addr;
    addr = arc_ps2.addr + 4 + 4 * index;
    if (status)
    addr += ARC_PS2_PORTS * 4;
    return addr;
    }
#[no_mangle]
unsafe extern "C" fn arc_ps2_inhibit_ports(arc_ps2: *mut arc_ps2_data) {
    static void arc_ps2_inhibit_ports(struct arc_ps2_data *arc_ps2)
    {
    void __iomem *addr;
    u32 val;
    int i;
    for (i = 0; i < ARC_PS2_PORTS; i++) {
    addr = arc_ps2_calc_addr(arc_ps2, i, true);
    val = ioread32(addr);
    val &= ~(PS2_STAT_RX_INT_EN | PS2_STAT_TX_INT_EN);
    iowrite32(val, addr);
    }
    }
    static int arc_ps2_create_port(struct platform_device *pdev,
    struct arc_ps2_data *arc_ps2,
    int index)
    {
    struct arc_ps2_port *port = &arc_ps2.port[index];
    struct serio *io;
    io = kzalloc_obj(*io);
    if (!io)
    return -ENOMEM;
    io.id.type = SERIO_8042;
    io.write = arc_ps2_write;
    io.open = arc_ps2_open;
    io.close = arc_ps2_close;
    snprintf(io.name, sizeof(io.name), "ARC PS/2 port%d", index);
    snprintf(io.phys, sizeof(io.phys), "arc/serio%d", index);
    io.port_data = port;
    port.io = io;
    port.data_addr = arc_ps2_calc_addr(arc_ps2, index, false);
    port.status_addr = arc_ps2_calc_addr(arc_ps2, index, true);
    dev_dbg(&pdev.dev, "port%d is allocated (data = 0x%p, status = 0x%p)\n",
    index, port.data_addr, port.status_addr);
    serio_register_port(port.io);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arc_ps2_probe(pdev: *mut platform_device) -> c_int {
    static int arc_ps2_probe(struct platform_device *pdev)
    {
    struct arc_ps2_data *arc_ps2;
    int irq;
    int error, id, i;
    irq = platform_get_irq_byname(pdev, "arc_ps2_irq");
    if (irq < 0)
    return -EINVAL;
    arc_ps2 = devm_kzalloc(&pdev.dev, sizeof(*arc_ps2), GFP_KERNEL);
    if (!arc_ps2) {
    dev_err(&pdev.dev, "out of memory\n");
    return -ENOMEM;
    }
    arc_ps2.addr = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(arc_ps2.addr))
    return PTR_ERR(arc_ps2.addr);
    dev_info(&pdev.dev, "irq = %d, address = 0x%p, ports = %i\n",
    irq, arc_ps2.addr, ARC_PS2_PORTS);
    id = ioread32(arc_ps2.addr);
    if (id != ARC_ARC_PS2_ID) {
    dev_err(&pdev.dev, "device id does not match\n");
    return -ENXIO;
    }
    arc_ps2_inhibit_ports(arc_ps2);
    error = devm_request_irq(&pdev.dev, irq, arc_ps2_interrupt,
    0, "arc_ps2", arc_ps2);
    if (error) {
    dev_err(&pdev.dev, "Could not allocate IRQ\n");
    return error;
    }
    for (i = 0; i < ARC_PS2_PORTS; i++) {
    error = arc_ps2_create_port(pdev, arc_ps2, i);
    if (error) {
    while (--i >= 0)
    serio_unregister_port(arc_ps2.port[i].io);
    return error;
    }
    }
    platform_set_drvdata(pdev, arc_ps2);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arc_ps2_remove(pdev: *mut platform_device) {
    static void arc_ps2_remove(struct platform_device *pdev)
    {
    struct arc_ps2_data *arc_ps2 = platform_get_drvdata(pdev);
    int i;
    for (i = 0; i < ARC_PS2_PORTS; i++)
    serio_unregister_port(arc_ps2.port[i].io);
    dev_dbg(&pdev.dev, "interrupt count = %i\n", arc_ps2.total_int);
    dev_dbg(&pdev.dev, "frame error count = %i\n", arc_ps2.frame_error);
    dev_dbg(&pdev.dev, "buffer overflow count = %i\n",
    arc_ps2.buf_overflow);
    }

    static const struct of_device_id arc_ps2_match[] = {
    { .compatible = "snps,arc_ps2" },
    { },
    };
    MODULE_DEVICE_TABLE(of, arc_ps2_match);

    static struct platform_driver arc_ps2_driver = {
    .driver	= {
    .name		= "arc_ps2",
    .of_match_table	= of_match_ptr(arc_ps2_match),
    },
    .probe	= arc_ps2_probe,
    .remove	= arc_ps2_remove,
    };
    module_platform_driver(arc_ps2_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Pavel Sokolov <psokolov@synopsys.com>");
    MODULE_DESCRIPTION("ARC PS/2 Driver");
