//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_dfl.c
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
// Driver for FPGA UART
//
// Copyright (C) 2022 Intel Corporation.
//
// Authors:
// Ananda Ravuri <ananda.ravuri@intel.com>
// Matthew Gerlach <matthew.gerlach@linux.intel.com>
//

pub const DFHv1_PARAM_ID_CLK_FRQ: c_uint = 0x2;
pub const DFHv1_PARAM_ID_FIFO_LEN: c_uint = 0x3;
pub const DFHv1_PARAM_ID_REG_LAYOUT: c_uint = 0x4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_uart {
    pub line: c_int,
}

#[no_mangle]
unsafe extern "C" fn dfh_get_u64_param_val(dfl_dev: *mut dfl_device, param_id: c_int, pval: *mut u64) -> c_int {
    static int dfh_get_u64_param_val(struct dfl_device *dfl_dev, int param_id, u64 *pval)
    {
    size_t psize;
    u64 *p;
    p = dfh_find_param(dfl_dev, param_id, &psize);
    if (IS_ERR(p))
    return PTR_ERR(p);
    if (psize != sizeof(*pval))
    return -EINVAL;
// pval = *p;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dfl_uart_get_params(dfl_dev: *mut dfl_device, uart: *mut uart_8250_port) -> c_int {
    static int dfl_uart_get_params(struct dfl_device *dfl_dev, struct uart_8250_port *uart)
    {
    struct device *dev = &dfl_dev.dev;
    u64 fifo_len, clk_freq, reg_layout;
    u32 reg_width;
    int ret;
    ret = dfh_get_u64_param_val(dfl_dev, DFHv1_PARAM_ID_CLK_FRQ, &clk_freq);
    if (ret)
    return dev_err_probe(dev, ret, "missing CLK_FRQ param\n");
    uart.port.uartclk = clk_freq;
    ret = dfh_get_u64_param_val(dfl_dev, DFHv1_PARAM_ID_FIFO_LEN, &fifo_len);
    if (ret)
    return dev_err_probe(dev, ret, "missing FIFO_LEN param\n");
    switch (fifo_len) {
    case 32:
    uart.port.type = PORT_ALTR_16550_F32;
    break;
    case 64:
    uart.port.type = PORT_ALTR_16550_F64;
    break;
    case 128:
    uart.port.type = PORT_ALTR_16550_F128;
    break;
    default:
    return dev_err_probe(dev, -EINVAL, "unsupported FIFO_LEN %llu\n", fifo_len);
    }
    ret = dfh_get_u64_param_val(dfl_dev, DFHv1_PARAM_ID_REG_LAYOUT, &reg_layout);
    if (ret)
    return dev_err_probe(dev, ret, "missing REG_LAYOUT param\n");
    uart.port.regshift = FIELD_GET(DFHv1_PARAM_REG_LAYOUT_SHIFT, reg_layout);
    reg_width = FIELD_GET(DFHv1_PARAM_REG_LAYOUT_WIDTH, reg_layout);
    switch (reg_width) {
    case 4:
    uart.port.iotype = UPIO_MEM32;
    break;
    case 2:
    uart.port.iotype = UPIO_MEM16;
    break;
    default:
    return dev_err_probe(dev, -EINVAL, "unsupported reg-width %u\n", reg_width);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dfl_uart_probe(dfl_dev: *mut dfl_device) -> c_int {
    static int dfl_uart_probe(struct dfl_device *dfl_dev)
    {
    struct device *dev = &dfl_dev.dev;
    let mut uart: uart_8250_port = { };
    struct dfl_uart *dfluart;
    int ret;
    uart.port.flags = UPF_IOREMAP;
    uart.port.mapbase = dfl_dev.mmio_res.start;
    uart.port.mapsize = resource_size(&dfl_dev.mmio_res);
    ret = dfl_uart_get_params(dfl_dev, &uart);
    if (ret < 0)
    return dev_err_probe(dev, ret, "failed uart feature walk\n");
    if (dfl_dev.num_irqs == 1)
    uart.port.irq = dfl_dev.irqs[0];
    dfluart = devm_kzalloc(dev, sizeof(*dfluart), GFP_KERNEL);
    if (!dfluart)
    return -ENOMEM;
    dfluart.line = serial8250_register_8250_port(&uart);
    if (dfluart.line < 0)
    return dev_err_probe(dev, dfluart.line, "unable to register 8250 port.\n");
    dev_set_drvdata(dev, dfluart);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dfl_uart_remove(dfl_dev: *mut dfl_device) {
    static void dfl_uart_remove(struct dfl_device *dfl_dev)
    {
    struct dfl_uart *dfluart = dev_get_drvdata(&dfl_dev.dev);
    serial8250_unregister_port(dfluart.line);
    }
pub const FME_FEATURE_ID_UART: c_uint = 0x24;
    static const struct dfl_device_id dfl_uart_ids[] = {
    { FME_ID, FME_FEATURE_ID_UART },
    { }
    };
    MODULE_DEVICE_TABLE(dfl, dfl_uart_ids);
    static struct dfl_driver dfl_uart_driver = {
    .drv = {
    .name = "dfl-uart",
    },
    .id_table = dfl_uart_ids,
    .probe = dfl_uart_probe,
    .remove = dfl_uart_remove,
    };
    module_dfl_driver(dfl_uart_driver);
    MODULE_DESCRIPTION("DFL Intel UART driver");
    MODULE_AUTHOR("Intel Corporation");
    MODULE_LICENSE("GPL");
