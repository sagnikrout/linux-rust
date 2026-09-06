//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_lpc18xx.c
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
// Serial port driver for NXP LPC18xx/43xx UART
//
// Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
//
// Based on 8250_mtk.c:
// Copyright (c) 2014 MundoReader S.L.
// Matthias Brugger <matthias.bgg@gmail.com>
//

// Additional LPC18xx/43xx 8250 registers and bits

pub const LPC18XX_UART_RS485DLY_MAX: c_int = 255;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc18xx_uart_data {
    pub dma: uart_8250_dma,
    pub clk_uart: *mut clk,
    pub clk_reg: *mut clk,
    pub line: c_int,
}

    static int lpc18xx_rs485_config(struct uart_port *port, struct ktermios *termios,
    struct serial_rs485 *rs485)
    {
    struct uart_8250_port *up = up_to_u8250p(port);
    let mut rs485_ctrl_reg: u32 = 0;
    let mut rs485_dly_reg: u32 = 0;
    unsigned baud_clk;
    if (rs485.flags & SER_RS485_ENABLED) {
    rs485_ctrl_reg |= LPC18XX_UART_RS485CTRL_NMMEN |
    LPC18XX_UART_RS485CTRL_DCTRL;
    if (rs485.flags & SER_RS485_RTS_ON_SEND)
    rs485_ctrl_reg |= LPC18XX_UART_RS485CTRL_OINV;
    }
    if (rs485.delay_rts_after_send) {
    baud_clk = port.uartclk / up.dl_read(up);
    rs485_dly_reg = DIV_ROUND_UP(rs485.delay_rts_after_send
// baud_clk, MSEC_PER_SEC);
    if (rs485_dly_reg > LPC18XX_UART_RS485DLY_MAX)
    rs485_dly_reg = LPC18XX_UART_RS485DLY_MAX;
// Calculate the resulting delay in ms
    rs485.delay_rts_after_send = (rs485_dly_reg * MSEC_PER_SEC)
    / baud_clk;
    }
    serial_out(up, LPC18XX_UART_RS485CTRL, rs485_ctrl_reg);
    serial_out(up, LPC18XX_UART_RS485DLY, rs485_dly_reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_uart_serial_out(p: *mut uart_port, offset: c_uint, value: u32) {
    static void lpc18xx_uart_serial_out(struct uart_port *p, unsigned int offset, u32 value)
    {
//
// For DMA mode one must ensure that the UART_FCR_DMA_SELECT
// bit is set when FIFO is enabled. Even if DMA is not used
// setting this bit doesn't seem to affect anything.
//
    if (offset == UART_FCR && (value & UART_FCR_ENABLE_FIFO))
    value |= UART_FCR_DMA_SELECT;
    offset = offset << p.regshift;
    writel(value, p.membase + offset);
    }
    static const struct serial_rs485 lpc18xx_rs485_supported = {
    .flags = SER_RS485_ENABLED | SER_RS485_RTS_ON_SEND | SER_RS485_RTS_AFTER_SEND,
    .delay_rts_after_send = 1,
// Delay RTS before send is not supported
    };
#[no_mangle]
unsafe extern "C" fn lpc18xx_serial_probe(pdev: *mut platform_device) -> c_int {
    static int lpc18xx_serial_probe(struct platform_device *pdev)
    {
    struct lpc18xx_uart_data *data;
    struct uart_8250_port uart;
    struct resource *res;
    int ret;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res) {
    dev_err(&pdev.dev, "memory resource not found");
    return -EINVAL;
    }
    memset(&uart, 0, sizeof(uart));
    uart.port.membase = devm_ioremap(&pdev.dev, res.start,
    resource_size(res));
    if (!uart.port.membase)
    return -ENOMEM;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.clk_uart = devm_clk_get(&pdev.dev, "uartclk");
    if (IS_ERR(data.clk_uart)) {
    dev_err(&pdev.dev, "uart clock not found\n");
    return PTR_ERR(data.clk_uart);
    }
    data.clk_reg = devm_clk_get(&pdev.dev, "reg");
    if (IS_ERR(data.clk_reg)) {
    dev_err(&pdev.dev, "reg clock not found\n");
    return PTR_ERR(data.clk_reg);
    }
    ret = clk_prepare_enable(data.clk_reg);
    if (ret) {
    dev_err(&pdev.dev, "unable to enable reg clock\n");
    return ret;
    }
    ret = clk_prepare_enable(data.clk_uart);
    if (ret) {
    dev_err(&pdev.dev, "unable to enable uart clock\n");
    goto dis_clk_reg;
    }
    data.dma.rx_param = data;
    data.dma.tx_param = data;
    spin_lock_init(&uart.port.lock);
    uart.port.dev = &pdev.dev;
    uart.port.mapbase = res.start;
    uart.port.type = PORT_16550A;
    uart.port.flags = UPF_FIXED_PORT | UPF_FIXED_TYPE | UPF_SKIP_TEST;
    uart.port.uartclk = clk_get_rate(data.clk_uart);
    uart.port.private_data = data;
    uart.port.rs485_config = lpc18xx_rs485_config;
    uart.port.rs485_supported = lpc18xx_rs485_supported;
    uart.port.serial_out = lpc18xx_uart_serial_out;
    ret = uart_read_port_properties(&uart.port);
    if (ret)
    goto dis_uart_clk;
    uart.port.iotype = UPIO_MEM32;
    uart.port.regshift = 2;
    uart.dma = &data.dma;
    uart.dma.rxconf.src_maxburst = 1;
    uart.dma.txconf.dst_maxburst = 1;
    ret = serial8250_register_8250_port(&uart);
    if (ret < 0) {
    dev_err(&pdev.dev, "unable to register 8250 port\n");
    goto dis_uart_clk;
    }
    data.line = ret;
    platform_set_drvdata(pdev, data);
    return 0;
    dis_uart_clk:
    clk_disable_unprepare(data.clk_uart);
    dis_clk_reg:
    clk_disable_unprepare(data.clk_reg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_serial_remove(pdev: *mut platform_device) {
    static void lpc18xx_serial_remove(struct platform_device *pdev)
    {
    struct lpc18xx_uart_data *data = platform_get_drvdata(pdev);
    serial8250_unregister_port(data.line);
    clk_disable_unprepare(data.clk_uart);
    clk_disable_unprepare(data.clk_reg);
    }
    static const struct of_device_id lpc18xx_serial_match[] = {
    { .compatible = "nxp,lpc1850-uart" },
    { },
    };
    MODULE_DEVICE_TABLE(of, lpc18xx_serial_match);
    static struct platform_driver lpc18xx_serial_driver = {
    .probe  = lpc18xx_serial_probe,
    .remove = lpc18xx_serial_remove,
    .driver = {
    .name = "lpc18xx-uart",
    .of_match_table = lpc18xx_serial_match,
    },
    };
    module_platform_driver(lpc18xx_serial_driver);
    MODULE_AUTHOR("Joachim Eastwood <manabian@gmail.com>");
    MODULE_DESCRIPTION("Serial port driver NXP LPC18xx/43xx devices");
    MODULE_LICENSE("GPL v2");
