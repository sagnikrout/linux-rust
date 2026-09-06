//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_keba.c
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
// Copyright (C) 2025 KEBA Industrial Automation GmbH
//
// Driver for KEBA UART FPGA IP core
//

// flags

// registers
pub const KUART_VERSION: c_uint = 0x0000;
pub const KUART_REVISION: c_uint = 0x0001;
pub const KUART_CAPABILITY: c_uint = 0x0002;
pub const KUART_CONTROL: c_uint = 0x0004;
pub const KUART_BASE: c_uint = 0x000C;
pub const KUART_REGSHIFT: c_int = 2;
pub const KUART_CLK: c_int = 1843200;
// mode flags
    enum kuart_mode {
    KUART_MODE_NONE = 0,
    KUART_MODE_RS485,
    KUART_MODE_RS422,
    KUART_MODE_RS232
    };
// capability flags

// registers for Indexed Control Register access in enhanced mode

// Additional Control Register DTR line configuration
pub const UART_ACR_DTRLC_MASK: c_uint = 0x18;
pub const UART_ACR_DTRLC_COMPAT: c_uint = 0x00;
pub const UART_ACR_DTRLC_ENABLE_LOW: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kuart {
    pub auxdev: *mut keba_uart_auxdev,
    pub base: *mut void __iomem,
    pub line: c_uint,
    pub flags: c_uint,
    pub capability: u8,
    pub mode: enum kuart_mode,
}

#[no_mangle]
unsafe extern "C" fn kuart_set_phy_mode(kuart: *mut kuart, mode: enum kuart_mode) {
    static void kuart_set_phy_mode(struct kuart *kuart, enum kuart_mode mode)
    {
    iowrite8(mode, kuart.base + KUART_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn kuart_enhanced_mode(up: *mut uart_8250_port, enable: bool) {
    static void kuart_enhanced_mode(struct uart_8250_port *up, bool enable)
    {
    u8 lcr, efr;
// backup LCR register
    lcr = serial_in(up, UART_LCR);
// enable 650 compatible register set (EFR, ...)
    serial_out(up, UART_LCR, UART_LCR_CONF_MODE_B);
// enable/disable enhanced mode with indexed control registers
    efr = serial_in(up, UART_EFR);
    if (enable)
    efr |= UART_EFR_ECB;
    else
    efr &= ~UART_EFR_ECB;
    serial_out(up, UART_EFR, efr);
// disable 650 compatible register set, restore LCR
    serial_out(up, UART_LCR, lcr);
    }
#[no_mangle]
unsafe extern "C" fn kuart_dtr_line_config(up: *mut uart_8250_port, dtrlc: u8) {
    static void kuart_dtr_line_config(struct uart_8250_port *up, u8 dtrlc)
    {
    u8 acr;
// set index register to 0 to access ACR register
    serial_out(up, KUART_EMODE_ICR_OFFSET, UART_ACR);
// set value register to 0x10 writing DTR mode (1,0)
    acr = serial_in(up, KUART_EMODE_ICR_VALUE);
    acr &= ~UART_ACR_DTRLC_MASK;
    acr |= dtrlc;
    serial_out(up, KUART_EMODE_ICR_VALUE, acr);
    }
    static int kuart_rs485_config(struct uart_port *port, struct ktermios *termios,
    struct serial_rs485 *rs485)
    {
    struct uart_8250_port *up = up_to_u8250p(port);
    struct kuart *kuart = port.private_data;
    enum kuart_mode mode;
    u8 dtrlc;
    if (rs485.flags & SER_RS485_ENABLED) {
    if (rs485.flags & SER_RS485_MODE_RS422)
    mode = KUART_MODE_RS422;
    else
    mode = KUART_MODE_RS485;
    } else {
    mode = KUART_MODE_RS232;
    }
    if (mode == kuart.mode)
    return 0;
    if (kuart.flags & KUART_USE_CAPABILITY) {
// deactivate physical interface, break before make
    kuart_set_phy_mode(kuart, KUART_MODE_NONE);
    }
    if (mode == KUART_MODE_RS485) {
//
// Set DTR line configuration of 95x UART to DTR mode (1,0).
// In this mode the DTR pin drives the active-low enable pin of
// an external RS485 buffer. The DTR pin will be forced low
// whenever the transmitter is not empty, otherwise DTR pin is
// high.
//
    dtrlc = UART_ACR_DTRLC_ENABLE_LOW;
    } else {
//
// Set DTR line configuration of 95x UART to DTR mode (0,0).
// In this mode the DTR pin is compatible with 16C450, 16C550,
// 16C650 and 16c670 (i.e. normal).
//
    dtrlc = UART_ACR_DTRLC_COMPAT;
    }
    kuart_enhanced_mode(up, true);
    kuart_dtr_line_config(up, dtrlc);
    kuart_enhanced_mode(up, false);
    if (kuart.flags & KUART_USE_CAPABILITY) {
// activate selected physical interface
    kuart_set_phy_mode(kuart, mode);
    }
    kuart.mode = mode;
    return 0;
    }
    static int kuart_probe(struct auxiliary_device *auxdev,
    const struct auxiliary_device_id *id)
    {
    struct device *dev = &auxdev.dev;
    let mut uart: uart_8250_port = {};
    struct resource res;
    struct kuart *kuart;
    int retval;
    kuart = devm_kzalloc(dev, sizeof(*kuart), GFP_KERNEL);
    if (!kuart)
    return -ENOMEM;
    kuart.auxdev = container_of(auxdev, struct keba_uart_auxdev, auxdev);
    kuart.flags = id.driver_data;
    auxiliary_set_drvdata(auxdev, kuart);
//
// map only memory in front of UART registers, UART registers will be
// mapped by serial port
//
    res = kuart.auxdev.io;
    res.end = res.start + KUART_BASE - 1;
    kuart.base = devm_ioremap_resource(dev, &res);
    if (IS_ERR(kuart.base))
    return PTR_ERR(kuart.base);
    if (kuart.flags & KUART_USE_CAPABILITY) {
//
// supported modes are read from capability register, at least
// one mode other than none must be supported
//
    kuart.capability = ioread8(kuart.base + KUART_CAPABILITY) &
    KUART_CAPABILITY_MASK;
    if ((kuart.capability & ~KUART_CAPABILITY_NONE) == 0)
    return -EIO;
    }
    spin_lock_init(&uart.port.lock);
    uart.port.dev = dev;
    uart.port.mapbase = kuart.auxdev.io.start + KUART_BASE;
    uart.port.irq = kuart.auxdev.irq;
    uart.port.uartclk = KUART_CLK;
    uart.port.private_data = kuart;
// 8 bit registers are 32 bit aligned => shift register offset
    uart.port.iotype = UPIO_MEM32;
    uart.port.regshift = KUART_REGSHIFT;
//
// UART mixes 16550, 16750 and 16C950 (for RS485) standard => auto
// configuration works best
//
    uart.port.flags = UPF_SKIP_TEST | UPF_BOOT_AUTOCONF | UPF_IOREMAP;
//
// UART supports RS485, RS422 and RS232 with switching of physical
// interface
//
    uart.port.rs485_config = kuart_rs485_config;
    if (kuart.flags & KUART_RS485) {
    uart.port.rs485_supported.flags = SER_RS485_ENABLED |
    SER_RS485_RTS_ON_SEND;
    uart.port.rs485.flags = SER_RS485_ENABLED |
    SER_RS485_RTS_ON_SEND;
    }
    if (kuart.flags & KUART_USE_CAPABILITY) {
// default mode priority is RS485 > RS422 > RS232
    if (kuart.capability & KUART_CAPABILITY_RS422) {
    uart.port.rs485_supported.flags |= SER_RS485_ENABLED |
    SER_RS485_RTS_ON_SEND |
    SER_RS485_MODE_RS422;
    uart.port.rs485.flags = SER_RS485_ENABLED |
    SER_RS485_RTS_ON_SEND |
    SER_RS485_MODE_RS422;
    }
    if (kuart.capability & KUART_CAPABILITY_RS485) {
    uart.port.rs485_supported.flags |= SER_RS485_ENABLED |
    SER_RS485_RTS_ON_SEND;
    uart.port.rs485.flags = SER_RS485_ENABLED |
    SER_RS485_RTS_ON_SEND;
    }
    }
    retval = serial8250_register_8250_port(&uart);
    if (retval < 0)
    return dev_err_probe(&auxdev.dev, retval,
    "UART registration failed!\n");
    kuart.line = retval;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kuart_remove(auxdev: *mut auxiliary_device) {
    static void kuart_remove(struct auxiliary_device *auxdev)
    {
    struct kuart *kuart = auxiliary_get_drvdata(auxdev);
    if (kuart.flags & KUART_USE_CAPABILITY)
    kuart_set_phy_mode(kuart, KUART_MODE_NONE);
    serial8250_unregister_port(kuart.line);
    }
    static const struct auxiliary_device_id kuart_devtype_aux[] = {
    { .name = "keba.rs485-uart", .driver_data = KUART_RS485 },
    { .name = "keba.rs232-uart", .driver_data = 0 },
    { .name = "keba.uart", .driver_data = KUART_USE_CAPABILITY },
    {}
    };
    MODULE_DEVICE_TABLE(auxiliary, kuart_devtype_aux);
    static struct auxiliary_driver kuart_driver_aux = {
    .name = KUART,
    .id_table = kuart_devtype_aux,
    .probe  = kuart_probe,
    .remove = kuart_remove,
    };
    module_auxiliary_driver(kuart_driver_aux);
    MODULE_AUTHOR("Gerhard Engleder <eg@keba.com>");
    MODULE_DESCRIPTION("KEBA 8250 serial port driver");
    MODULE_LICENSE("GPL");
