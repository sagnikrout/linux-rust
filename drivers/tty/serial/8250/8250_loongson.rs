//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_loongson.c
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
// Serial Port driver for Loongson family chips
//
// Copyright (C) 2020-2025 Loongson Technology Corporation Limited
//

// Divisor Latch Fraction Register
pub const LOONGSON_UART_DLF: c_uint = 0x2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson_uart_ddata {
    pub has_frac: bool,
    pub mcr_invert: u8,
    pub msr_invert: u8,
}

    static const struct loongson_uart_ddata ls2k0500_uart_data = {
    .has_frac = false,
    .mcr_invert = UART_MCR_RTS | UART_MCR_DTR,
    .msr_invert = UART_MSR_CTS | UART_MSR_DSR,
    };
    static const struct loongson_uart_ddata ls2k1500_uart_data = {
    .has_frac = true,
    .mcr_invert = UART_MCR_RTS | UART_MCR_DTR,
    .msr_invert = 0,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson_uart_priv {
    pub line: c_int,
    pub clk: *mut clk,
    pub res: *mut resource,
    pub rst: *mut reset_control,
    pub ddata: *const loongson_uart_ddata,
}

#[no_mangle]
unsafe extern "C" fn serial_fixup(p: *mut uart_port, offset: c_uint, val: u8) -> u8 {
    static u8 serial_fixup(struct uart_port *p, unsigned int offset, u8 val)
    {
    struct loongson_uart_priv *priv = p.private_data;
    switch (offset) {
    case UART_MCR:
    return val ^ priv.ddata.mcr_invert;
    case UART_MSR:
    return val ^ priv.ddata.msr_invert;
    default:
    return val;
    }
    }
#[no_mangle]
unsafe extern "C" fn loongson_serial_in(p: *mut uart_port, offset: c_uint) -> u32 {
    static u32 loongson_serial_in(struct uart_port *p, unsigned int offset)
    {
    u8 val;
    val = readb(p.membase + (offset << p.regshift));
    return serial_fixup(p, offset, val);
    }
#[no_mangle]
unsafe extern "C" fn loongson_serial_out(p: *mut uart_port, offset: c_uint, value: c_uint) {
    static void loongson_serial_out(struct uart_port *p, unsigned int offset, unsigned int value)
    {
    u8 val;
    offset <<= p.regshift;
    val = serial_fixup(p, offset, value);
    writeb(val, p.membase + offset);
    }
    static unsigned int loongson_frac_get_divisor(struct uart_port *port, unsigned int baud,
    unsigned int *frac)
    {
    unsigned int quot;
    quot = DIV_ROUND_CLOSEST((port.uartclk << 4), baud);
// frac = FIELD_GET(LOONGSON_QUOT_FRAC_MASK, quot);
    return FIELD_GET(LOONGSON_QUOT_DIV_MASK, quot);
    }
    static void loongson_frac_set_divisor(struct uart_port *port, unsigned int baud,
    unsigned int quot, unsigned int quot_frac)
    {
    struct uart_8250_port *up = up_to_u8250p(port);
    serial_port_out(port, UART_LCR, up.lcr | UART_LCR_DLAB);
    serial_dl_write(up, quot);
    serial_port_out(port, LOONGSON_UART_DLF, quot_frac);
    }
#[no_mangle]
unsafe extern "C" fn loongson_uart_probe(pdev: *mut platform_device) -> c_int {
    static int loongson_uart_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    let mut uart: uart_8250_port = {};
    struct loongson_uart_priv *priv;
    struct uart_port *port;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.ddata = device_get_match_data(dev);
    port = &uart.port;
    spin_lock_init(&port.lock);
    port.flags = UPF_SHARE_IRQ | UPF_FIXED_PORT | UPF_FIXED_TYPE | UPF_IOREMAP;
    port.iotype = UPIO_MEM;
    port.regshift = 0;
    port.dev = dev;
    port.type = PORT_16550A;
    port.private_data = priv;
    port.membase = devm_platform_get_and_ioremap_resource(pdev, 0, &priv.res);
    if (IS_ERR(port.membase))
    return PTR_ERR(port.membase);
    port.mapbase = priv.res.start;
    port.mapsize = resource_size(priv.res);
    port.serial_in = loongson_serial_in;
    port.serial_out = loongson_serial_out;
    if (priv.ddata.has_frac) {
    port.get_divisor = loongson_frac_get_divisor;
    port.set_divisor = loongson_frac_set_divisor;
    }
    ret = uart_read_port_properties(port);
    if (ret)
    return ret;
    if (!port.uartclk) {
    priv.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return dev_err_probe(dev, PTR_ERR(priv.clk),
    "Unable to determine clock frequency!\n");
    port.uartclk = clk_get_rate(priv.clk);
    }
    priv.rst = devm_reset_control_get_optional_shared(dev, core::ptr::null_mut());
    if (IS_ERR(priv.rst))
    return PTR_ERR(priv.rst);
    ret = reset_control_deassert(priv.rst);
    if (ret)
    return ret;
    ret = serial8250_register_8250_port(&uart);
    if (ret < 0) {
    reset_control_assert(priv.rst);
    return ret;
    }
    priv.line = ret;
    platform_set_drvdata(pdev, priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loongson_uart_remove(pdev: *mut platform_device) {
    static void loongson_uart_remove(struct platform_device *pdev)
    {
    struct loongson_uart_priv *priv = platform_get_drvdata(pdev);
    serial8250_unregister_port(priv.line);
    reset_control_assert(priv.rst);
    }
#[no_mangle]
unsafe extern "C" fn loongson_uart_suspend(dev: *mut device) -> c_int {
    static int loongson_uart_suspend(struct device *dev)
    {
    struct loongson_uart_priv *priv = dev_get_drvdata(dev);
    struct uart_8250_port *up = serial8250_get_port(priv.line);
    serial8250_suspend_port(priv.line);
    if (!uart_console(&up.port) || console_suspend_enabled)
    clk_disable_unprepare(priv.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loongson_uart_resume(dev: *mut device) -> c_int {
    static int loongson_uart_resume(struct device *dev)
    {
    struct loongson_uart_priv *priv = dev_get_drvdata(dev);
    struct uart_8250_port *up = serial8250_get_port(priv.line);
    int ret;
    if (!uart_console(&up.port) || console_suspend_enabled) {
    ret = clk_prepare_enable(priv.clk);
    if (ret)
    return ret;
    }
    serial8250_resume_port(priv.line);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(loongson_uart_pm_ops, loongson_uart_suspend,
    loongson_uart_resume);
    static const struct of_device_id loongson_uart_of_ids[] = {
    { .compatible = "loongson,ls2k0500-uart", .data = &ls2k0500_uart_data },
    { .compatible = "loongson,ls2k1500-uart", .data = &ls2k1500_uart_data },
    { },
    };
    MODULE_DEVICE_TABLE(of, loongson_uart_of_ids);
    static struct platform_driver loongson_uart_driver = {
    .probe = loongson_uart_probe,
    .remove = loongson_uart_remove,
    .driver = {
    .name = "loongson-uart",
    .pm = pm_ptr(&loongson_uart_pm_ops),
    .of_match_table = loongson_uart_of_ids,
    },
    };
    module_platform_driver(loongson_uart_driver);
    MODULE_DESCRIPTION("Loongson UART driver");
    MODULE_AUTHOR("Loongson Technology Corporation Limited.");
    MODULE_LICENSE("GPL");
