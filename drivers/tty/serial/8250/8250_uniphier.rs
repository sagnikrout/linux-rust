//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_uniphier.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2015 Masahiro Yamada <yamada.masahiro@socionext.com>
//

//
// This hardware is similar to 8250, but its register map is a bit different:
// - MMIO32 (regshift = 2)
// - FCR is not at 2, but 3
// - LCR and MCR are not at 3 and 4, they share 4
// - No SCR (Instead, CHAR can be used as a scratch register)
// - Divisor latch at 9, no divisor latch access bit
//
pub const UNIPHIER_UART_REGSHIFT: c_int = 2;
// bit[15:8] = CHAR, bit[7:0] = FCR

// bit[15:8] = LCR, bit[7:0] = MCR

// Divisor Latch Register

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier8250_priv {
    pub line: c_int,
    pub clk: *mut clk,
    pub atomic_write_lock: spinlock_t,
}

    static int __init uniphier_early_console_setup(struct earlycon_device *device,
    const char *options)
    {
    if (!device.port.membase)
    return -ENODEV;
// This hardware always expects MMIO32 register interface.
    device.port.iotype = UPIO_MEM32;
    device.port.regshift = UNIPHIER_UART_REGSHIFT;
//
// Do not touch the divisor register in early_serial8250_setup();
// we assume it has been initialized by a boot loader.
//
    device.baud = 0;
    return early_serial8250_setup(device, options);
    }
    OF_EARLYCON_DECLARE(uniphier, "socionext,uniphier-uart",
    uniphier_early_console_setup);

//
// The register map is slightly different from that of 8250.
// IO callbacks must be overridden for correct access to FCR, LCR, MCR and SCR.
//
#[no_mangle]
unsafe extern "C" fn uniphier_serial_in(p: *mut uart_port, offset: c_uint) -> u32 {
    static u32 uniphier_serial_in(struct uart_port *p, unsigned int offset)
    {
    let mut valshift: c_uint = 0;
    switch (offset) {
    case UART_SCR:
// No SCR for this hardware.  Use CHAR as a scratch register
    valshift = 8;
    offset = UNIPHIER_UART_CHAR_FCR;
    break;
    case UART_LCR:
    valshift = 8;
    fallthrough;
    case UART_MCR:
    offset = UNIPHIER_UART_LCR_MCR;
    break;
    default:
    offset <<= UNIPHIER_UART_REGSHIFT;
    break;
    }
//
// The return value must be masked with 0xff because some registers
// share the same offset that must be accessed by 32-bit write/read.
// 8 or 16 bit access to this hardware result in unexpected behavior.
//
    return (readl(p.membase + offset) >> valshift) & 0xff;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_serial_out(p: *mut uart_port, offset: c_uint, value: u32) {
    static void uniphier_serial_out(struct uart_port *p, unsigned int offset, u32 value)
    {
    let mut valshift: c_uint = 0;
    let mut normal: bool = false;
    switch (offset) {
    case UART_SCR:
// No SCR for this hardware.  Use CHAR as a scratch register
    valshift = 8;
    fallthrough;
    case UART_FCR:
    offset = UNIPHIER_UART_CHAR_FCR;
    break;
    case UART_LCR:
    valshift = 8;
// Divisor latch access bit does not exist.
    value &= ~UART_LCR_DLAB;
    fallthrough;
    case UART_MCR:
    offset = UNIPHIER_UART_LCR_MCR;
    break;
    default:
    offset <<= UNIPHIER_UART_REGSHIFT;
    normal = true;
    break;
    }
    if (normal) {
    writel(value, p.membase + offset);
    } else {
//
// Special case: two registers share the same address that
// must be 32-bit accessed.  As this is not longer atomic safe,
// take a lock just in case.
//
    struct uniphier8250_priv *priv = p.private_data;
    unsigned long flags;
    u32 tmp;
    spin_lock_irqsave(&priv.atomic_write_lock, flags);
    tmp = readl(p.membase + offset);
    tmp &= ~(0xff << valshift);
    tmp |= value << valshift;
    writel(tmp, p.membase + offset);
    spin_unlock_irqrestore(&priv.atomic_write_lock, flags);
    }
    }
//
// This hardware does not have the divisor latch access bit.
// The divisor latch register exists at different address.
// Override dl_read/write callbacks.
//
#[no_mangle]
unsafe extern "C" fn uniphier_serial_dl_read(up: *mut uart_8250_port) -> u32 {
    static u32 uniphier_serial_dl_read(struct uart_8250_port *up)
    {
    return readl(up.port.membase + UNIPHIER_UART_DLR);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_serial_dl_write(up: *mut uart_8250_port, value: u32) {
    static void uniphier_serial_dl_write(struct uart_8250_port *up, u32 value)
    {
    writel(value, up.port.membase + UNIPHIER_UART_DLR);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_uart_probe(pdev: *mut platform_device) -> c_int {
    static int uniphier_uart_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct uart_8250_port up;
    struct uniphier8250_priv *priv;
    struct resource *regs;
    void __iomem *membase;
    int ret;
    regs = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!regs) {
    dev_err(dev, "failed to get memory resource\n");
    return -EINVAL;
    }
    membase = devm_ioremap(dev, regs.start, resource_size(regs));
    if (!membase)
    return -ENOMEM;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    memset(&up, 0, sizeof(up));
    priv.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk)) {
    dev_err(dev, "failed to get clock\n");
    return PTR_ERR(priv.clk);
    }
    ret = clk_prepare_enable(priv.clk);
    if (ret)
    return ret;
    up.port.uartclk = clk_get_rate(priv.clk);
    spin_lock_init(&priv.atomic_write_lock);
    up.port.dev = dev;
    up.port.private_data = priv;
    up.port.mapbase = regs.start;
    up.port.mapsize = resource_size(regs);
    up.port.membase = membase;
    ret = uart_read_port_properties(&up.port);
    if (ret)
    return ret;
    up.port.type = PORT_16550A;
    up.port.iotype = UPIO_MEM32;
    up.port.fifosize = 64;
    up.port.regshift = UNIPHIER_UART_REGSHIFT;
    up.port.flags = UPF_FIXED_PORT | UPF_FIXED_TYPE;
    up.capabilities = UART_CAP_FIFO;
    if (of_property_read_bool(dev.of_node, "auto-flow-control"))
    up.capabilities |= UART_CAP_AFE;
    up.port.serial_in = uniphier_serial_in;
    up.port.serial_out = uniphier_serial_out;
    up.dl_read = uniphier_serial_dl_read;
    up.dl_write = uniphier_serial_dl_write;
    ret = serial8250_register_8250_port(&up);
    if (ret < 0) {
    dev_err(dev, "failed to register 8250 port\n");
    clk_disable_unprepare(priv.clk);
    return ret;
    }
    priv.line = ret;
    platform_set_drvdata(pdev, priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_uart_remove(pdev: *mut platform_device) {
    static void uniphier_uart_remove(struct platform_device *pdev)
    {
    struct uniphier8250_priv *priv = platform_get_drvdata(pdev);
    serial8250_unregister_port(priv.line);
    clk_disable_unprepare(priv.clk);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_uart_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused uniphier_uart_suspend(struct device *dev)
    {
    struct uniphier8250_priv *priv = dev_get_drvdata(dev);
    struct uart_8250_port *up = serial8250_get_port(priv.line);
    serial8250_suspend_port(priv.line);
    if (!uart_console(&up.port) || console_suspend_enabled)
    clk_disable_unprepare(priv.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_uart_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused uniphier_uart_resume(struct device *dev)
    {
    struct uniphier8250_priv *priv = dev_get_drvdata(dev);
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
    static const struct dev_pm_ops uniphier_uart_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(uniphier_uart_suspend, uniphier_uart_resume)
    };
    static const struct of_device_id uniphier_uart_match[] = {
    { .compatible = "socionext,uniphier-uart" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, uniphier_uart_match);
    static struct platform_driver uniphier_uart_platform_driver = {
    .probe		= uniphier_uart_probe,
    .remove		= uniphier_uart_remove,
    .driver = {
    .name	= "uniphier-uart",
    .of_match_table = uniphier_uart_match,
    .pm = &uniphier_uart_pm_ops,
    },
    };
    module_platform_driver(uniphier_uart_platform_driver);
    MODULE_AUTHOR("Masahiro Yamada <yamada.masahiro@socionext.com>");
    MODULE_DESCRIPTION("UniPhier UART driver");
    MODULE_LICENSE("GPL");
