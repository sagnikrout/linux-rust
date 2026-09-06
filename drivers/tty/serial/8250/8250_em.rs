//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_em.c
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
// Renesas Emma Mobile 8250 driver
//
// Copyright (C) 2012 Magnus Damm
//

pub const UART_DLL_EM: c_int = 9;
pub const UART_DLM_EM: c_int = 10;
pub const UART_HCR0_EM: c_int = 11;
//
// A high value for UART_FCR_EM avoids overlapping with existing UART_
// register defines. UART_FCR_EM_HW is the real HW register offset.
//
pub const UART_FCR_EM: c_uint = 0x10003;
pub const UART_FCR_EM_HW: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct serial8250_em_priv {
    pub line: c_int,
}

    static void serial8250_em_serial_out_helper(struct uart_port *p, int offset,
    int value)
    {
    switch (offset) {
    case UART_TX: /* TX @ 0x00 */
    writeb(value, p.membase);
    break;
    case UART_LCR: /* LCR @ 0x10 (+1) */
    case UART_MCR: /* MCR @ 0x14 (+1) */
    case UART_SCR: /* SCR @ 0x20 (+1) */
    writel(value, p.membase + ((offset + 1) << 2));
    break;
    case UART_FCR_EM:
    writel(value, p.membase + (UART_FCR_EM_HW << 2));
    break;
    case UART_IER: /* IER @ 0x04 */
    value &= 0x0f; /* only 4 valid bits - not Xscale */
    fallthrough;
    case UART_DLL_EM: /* DLL @ 0x24 (+9) */
    case UART_DLM_EM: /* DLM @ 0x28 (+9) */
    case UART_HCR0_EM: /* HCR0 @ 0x2c */
    writel(value, p.membase + (offset << 2));
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn serial8250_em_serial_in(p: *mut uart_port, offset: c_uint) -> u32 {
    static u32 serial8250_em_serial_in(struct uart_port *p, unsigned int offset)
    {
    switch (offset) {
    case UART_RX: /* RX @ 0x00 */
    return readb(p.membase);
    case UART_LCR: /* LCR @ 0x10 (+1) */
    case UART_MCR: /* MCR @ 0x14 (+1) */
    case UART_LSR: /* LSR @ 0x18 (+1) */
    case UART_MSR: /* MSR @ 0x1c (+1) */
    case UART_SCR: /* SCR @ 0x20 (+1) */
    return readl(p.membase + ((offset + 1) << 2));
    case UART_FCR_EM:
    return readl(p.membase + (UART_FCR_EM_HW << 2));
    case UART_IER: /* IER @ 0x04 */
    case UART_IIR: /* IIR @ 0x08 */
    case UART_DLL_EM: /* DLL @ 0x24 (+9) */
    case UART_DLM_EM: /* DLM @ 0x28 (+9) */
    case UART_HCR0_EM: /* HCR0 @ 0x2c */
    return readl(p.membase + (offset << 2));
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn serial8250_em_reg_update(p: *mut uart_port, off: c_int, value: c_int) {
    static void serial8250_em_reg_update(struct uart_port *p, int off, int value)
    {
    unsigned int ier, fcr, lcr, mcr, hcr0;
    ier = serial8250_em_serial_in(p, UART_IER);
    fcr = serial8250_em_serial_in(p, UART_FCR_EM);
    lcr = serial8250_em_serial_in(p, UART_LCR);
    mcr = serial8250_em_serial_in(p, UART_MCR);
    hcr0 = serial8250_em_serial_in(p, UART_HCR0_EM);
    serial8250_em_serial_out_helper(p, UART_FCR_EM, fcr |
    UART_FCR_CLEAR_RCVR |
    UART_FCR_CLEAR_XMIT);
    serial8250_em_serial_out_helper(p, UART_HCR0_EM, hcr0 |
    UART_HCR0_EM_SW_RESET);
    serial8250_em_serial_out_helper(p, UART_HCR0_EM, hcr0 &
    ~UART_HCR0_EM_SW_RESET);
    switch (off) {
    case UART_FCR_EM:
    fcr = value;
    break;
    case UART_LCR:
    lcr = value;
    break;
    case UART_MCR:
    mcr = value;
    break;
    }
    serial8250_em_serial_out_helper(p, UART_IER, ier);
    serial8250_em_serial_out_helper(p, UART_FCR_EM, fcr);
    serial8250_em_serial_out_helper(p, UART_MCR, mcr);
    serial8250_em_serial_out_helper(p, UART_LCR, lcr);
    serial8250_em_serial_out_helper(p, UART_HCR0_EM, hcr0);
    }
#[no_mangle]
unsafe extern "C" fn serial8250_em_serial_out(p: *mut uart_port, offset: c_uint, value: u32) {
    static void serial8250_em_serial_out(struct uart_port *p, unsigned int offset, u32 value)
    {
    switch (offset) {
    case UART_TX:
    case UART_SCR:
    case UART_IER:
    case UART_DLL_EM:
    case UART_DLM_EM:
    serial8250_em_serial_out_helper(p, offset, value);
    break;
    case UART_FCR:
    serial8250_em_reg_update(p, UART_FCR_EM, value);
    break;
    case UART_LCR:
    case UART_MCR:
    serial8250_em_reg_update(p, offset, value);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn serial8250_em_serial_dl_read(up: *mut uart_8250_port) -> u32 {
    static u32 serial8250_em_serial_dl_read(struct uart_8250_port *up)
    {
    return serial_in(up, UART_DLL_EM) | serial_in(up, UART_DLM_EM) << 8;
    }
#[no_mangle]
unsafe extern "C" fn serial8250_em_serial_dl_write(up: *mut uart_8250_port, value: u32) {
    static void serial8250_em_serial_dl_write(struct uart_8250_port *up, u32 value)
    {
    serial_out(up, UART_DLL_EM, value & 0xff);
    serial_out(up, UART_DLM_EM, value >> 8 & 0xff);
    }
#[no_mangle]
unsafe extern "C" fn serial8250_em_probe(pdev: *mut platform_device) -> c_int {
    static int serial8250_em_probe(struct platform_device *pdev)
    {
    struct serial8250_em_priv *priv;
    struct device *dev = &pdev.dev;
    struct uart_8250_port up;
    struct resource *regs;
    struct clk *sclk;
    int irq, ret;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    regs = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!regs)
    return dev_err_probe(dev, -EINVAL, "missing registers\n");
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    sclk = devm_clk_get_enabled(dev, "sclk");
    if (IS_ERR(sclk))
    return dev_err_probe(dev, PTR_ERR(sclk), "unable to get clock\n");
    memset(&up, 0, sizeof(up));
    up.port.mapbase = regs.start;
    up.port.irq = irq;
    up.port.type = PORT_16750;
    up.port.flags = UPF_FIXED_PORT | UPF_IOREMAP | UPF_FIXED_TYPE;
    up.port.dev = dev;
    up.port.private_data = priv;
    up.port.uartclk = clk_get_rate(sclk);
    up.port.iotype = UPIO_MEM32;
    up.port.serial_in = serial8250_em_serial_in;
    up.port.serial_out = serial8250_em_serial_out;
    up.dl_read = serial8250_em_serial_dl_read;
    up.dl_write = serial8250_em_serial_dl_write;
    ret = serial8250_register_8250_port(&up);
    if (ret < 0)
    return dev_err_probe(dev, ret, "unable to register 8250 port\n");
    priv.line = ret;
    platform_set_drvdata(pdev, priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn serial8250_em_remove(pdev: *mut platform_device) {
    static void serial8250_em_remove(struct platform_device *pdev)
    {
    struct serial8250_em_priv *priv = platform_get_drvdata(pdev);
    serial8250_unregister_port(priv.line);
    }
    static const struct of_device_id serial8250_em_dt_ids[] = {
    { .compatible = "renesas,em-uart", },
    {},
    };
    MODULE_DEVICE_TABLE(of, serial8250_em_dt_ids);
    static struct platform_driver serial8250_em_platform_driver = {
    .driver = {
    .name		= "serial8250-em",
    .of_match_table = serial8250_em_dt_ids,
    },
    .probe			= serial8250_em_probe,
    .remove			= serial8250_em_remove,
    };
    module_platform_driver(serial8250_em_platform_driver);
    MODULE_AUTHOR("Magnus Damm");
    MODULE_DESCRIPTION("Renesas Emma Mobile 8250 Driver");
    MODULE_LICENSE("GPL v2");
