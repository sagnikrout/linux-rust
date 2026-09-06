//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_fintek.c
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
// Probe for F81216A LPC to 4 UART and F81214E LPC/eSPI to 2 UART
//
// Copyright (C) 2014-2016 Ricardo Ribalda, Qtechnology A/S
//

pub const ADDR_PORT: c_int = 0;
pub const DATA_PORT: c_int = 1;
pub const EXIT_KEY: c_uint = 0xAA;
pub const CHIP_ID1: c_uint = 0x20;
pub const CHIP_ID2: c_uint = 0x21;
pub const CHIP_ID_F81865: c_uint = 0x0407;
pub const CHIP_ID_F81866: c_uint = 0x1010;
pub const CHIP_ID_F81966: c_uint = 0x0215;
pub const CHIP_ID_F81216AD: c_uint = 0x1602;
pub const CHIP_ID_F81216E: c_uint = 0x1617;
pub const CHIP_ID_F81216H: c_uint = 0x0501;
pub const CHIP_ID_F81214E: c_uint = 0x1417;
pub const CHIP_ID_F81216: c_uint = 0x0802;
pub const VENDOR_ID1: c_uint = 0x23;
pub const VENDOR_ID1_VAL: c_uint = 0x19;
pub const VENDOR_ID2: c_uint = 0x24;
pub const VENDOR_ID2_VAL: c_uint = 0x34;
pub const IO_ADDR1: c_uint = 0x61;
pub const IO_ADDR2: c_uint = 0x60;
pub const LDN: c_uint = 0x7;
pub const FINTEK_IRQ_MODE: c_uint = 0x70;

pub const IRQ_LEVEL_LOW: c_int = 0;

//
// F81216H clock source register, the value and mask is the same with F81866,
// but it's on F0h.
//
// Clock speeds for UART (register F0h)
// 00: 1.8432MHz.
// 01: 18.432MHz.
// 10: 24MHz.
// 11: 14.769MHz.
//
pub const RS485: c_uint = 0xF0;

pub const FIFO_CTRL: c_uint = 0xF6;

pub const F81216_LDN_LOW: c_uint = 0x0;
pub const F81216_LDN_HIGH: c_uint = 0x4;
//
// F81866/966 registers
//
// The IRQ setting mode of F81866/966 is not the same with F81216 series.
// Level/Low: IRQ_MODE0:0, IRQ_MODE1:0
// Edge/High: IRQ_MODE0:1, IRQ_MODE1:0
//
// Clock speeds for UART (register F2h)
// 00: 1.8432MHz.
// 01: 18.432MHz.
// 10: 24MHz.
// 11: 14.769MHz.
//
pub const F81866_IRQ_MODE: c_uint = 0xf0;

pub const F81866_LDN_LOW: c_uint = 0x10;
pub const F81866_LDN_HIGH: c_uint = 0x16;
pub const F81866_UART_CLK: c_uint = 0xF2;

pub const F81866_UART_CLK_1_8432MHZ: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fintek_8250 {
    pub pid: u16,
    pub base_port: u16,
    pub index: u8,
    pub key: u8,
}

#[no_mangle]
unsafe extern "C" fn sio_read_reg(pdata: *mut fintek_8250, reg: u8) -> u8 {
    static u8 sio_read_reg(struct fintek_8250 *pdata, u8 reg)
    {
    outb(reg, pdata.base_port + ADDR_PORT);
    return inb(pdata.base_port + DATA_PORT);
    }
#[no_mangle]
unsafe extern "C" fn sio_write_reg(pdata: *mut fintek_8250, reg: u8, data: u8) {
    static void sio_write_reg(struct fintek_8250 *pdata, u8 reg, u8 data)
    {
    outb(reg, pdata.base_port + ADDR_PORT);
    outb(data, pdata.base_port + DATA_PORT);
    }
    static void sio_write_mask_reg(struct fintek_8250 *pdata, u8 reg, u8 mask,
    u8 data)
    {
    u8 tmp;
    tmp = (sio_read_reg(pdata, reg) & ~mask) | (mask & data);
    sio_write_reg(pdata, reg, tmp);
    }
#[no_mangle]
unsafe extern "C" fn fintek_8250_enter_key(base_port: u16, key: u8) -> c_int {
    static int fintek_8250_enter_key(u16 base_port, u8 key)
    {
    if (!request_muxed_region(base_port, 2, "8250_fintek"))
    return -EBUSY;
// Force to deactivate all SuperIO in this base_port
    outb(EXIT_KEY, base_port + ADDR_PORT);
    outb(key, base_port + ADDR_PORT);
    outb(key, base_port + ADDR_PORT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fintek_8250_exit_key(base_port: u16) {
    static void fintek_8250_exit_key(u16 base_port)
    {
    outb(EXIT_KEY, base_port + ADDR_PORT);
    release_region(base_port + ADDR_PORT, 2);
    }
#[no_mangle]
unsafe extern "C" fn fintek_8250_check_id(pdata: *mut fintek_8250) -> c_int {
    static int fintek_8250_check_id(struct fintek_8250 *pdata)
    {
    u16 chip;
    if (sio_read_reg(pdata, VENDOR_ID1) != VENDOR_ID1_VAL)
    return -ENODEV;
    if (sio_read_reg(pdata, VENDOR_ID2) != VENDOR_ID2_VAL)
    return -ENODEV;
    chip = sio_read_reg(pdata, CHIP_ID1);
    chip |= sio_read_reg(pdata, CHIP_ID2) << 8;
    switch (chip) {
    case CHIP_ID_F81865:
    case CHIP_ID_F81866:
    case CHIP_ID_F81966:
    case CHIP_ID_F81216AD:
    case CHIP_ID_F81216E:
    case CHIP_ID_F81216H:
    case CHIP_ID_F81214E:
    case CHIP_ID_F81216:
    break;
    default:
    return -ENODEV;
    }
    pdata.pid = chip;
    return 0;
    }
    static int fintek_8250_get_ldn_range(struct fintek_8250 *pdata, int *min,
    int *max)
    {
    switch (pdata.pid) {
    case CHIP_ID_F81966:
    case CHIP_ID_F81865:
    case CHIP_ID_F81866:
// min = F81866_LDN_LOW;
// max = F81866_LDN_HIGH;
    return 0;
    case CHIP_ID_F81216AD:
    case CHIP_ID_F81216E:
    case CHIP_ID_F81216H:
    case CHIP_ID_F81214E:
    case CHIP_ID_F81216:
// min = F81216_LDN_LOW;
// max = F81216_LDN_HIGH;
    return 0;
    }
    return -ENODEV;
    }
    static int fintek_8250_rs485_config(struct uart_port *port, struct ktermios *termios,
    struct serial_rs485 *rs485)
    {
    let mut config: u8 = 0;
    struct fintek_8250 *pdata = port.private_data;
    if (!pdata)
    return -EINVAL;
    if (rs485.flags & SER_RS485_ENABLED) {
// Hardware do not support same RTS level on send and receive
    if (!(rs485.flags & SER_RS485_RTS_ON_SEND) ==
    !(rs485.flags & SER_RS485_RTS_AFTER_SEND))
    return -EINVAL;
    config |= RS485_URA;
    }
    if (rs485.delay_rts_before_send) {
    rs485.delay_rts_before_send = 1;
    config |= TXW4C_IRA;
    }
    if (rs485.delay_rts_after_send) {
    rs485.delay_rts_after_send = 1;
    config |= RXW4C_IRA;
    }
    if (rs485.flags & SER_RS485_RTS_ON_SEND)
    config |= RTS_INVERT;
    if (fintek_8250_enter_key(pdata.base_port, pdata.key))
    return -EBUSY;
    sio_write_reg(pdata, LDN, pdata.index);
    sio_write_reg(pdata, RS485, config);
    fintek_8250_exit_key(pdata.base_port);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fintek_8250_set_irq_mode(pdata: *mut fintek_8250, is_level: bool) {
    static void fintek_8250_set_irq_mode(struct fintek_8250 *pdata, bool is_level)
    {
    sio_write_reg(pdata, LDN, pdata.index);
    switch (pdata.pid) {
    case CHIP_ID_F81966:
    case CHIP_ID_F81866:
    sio_write_mask_reg(pdata, F81866_FIFO_CTRL, F81866_IRQ_MODE1,
    0);
    fallthrough;
    case CHIP_ID_F81865:
    sio_write_mask_reg(pdata, F81866_IRQ_MODE, F81866_IRQ_SHARE,
    F81866_IRQ_SHARE);
    sio_write_mask_reg(pdata, F81866_IRQ_MODE, F81866_IRQ_MODE0,
    is_level ? 0 : F81866_IRQ_MODE0);
    break;
    case CHIP_ID_F81216AD:
    case CHIP_ID_F81216E:
    case CHIP_ID_F81216H:
    case CHIP_ID_F81214E:
    case CHIP_ID_F81216:
    sio_write_mask_reg(pdata, FINTEK_IRQ_MODE, IRQ_SHARE,
    IRQ_SHARE);
    sio_write_mask_reg(pdata, FINTEK_IRQ_MODE, IRQ_MODE_MASK,
    is_level ? IRQ_LEVEL_LOW : IRQ_EDGE_HIGH);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn fintek_8250_set_max_fifo(pdata: *mut fintek_8250) {
    static void fintek_8250_set_max_fifo(struct fintek_8250 *pdata)
    {
    switch (pdata.pid) {
    case CHIP_ID_F81216E: /* 128Bytes FIFO */
    case CHIP_ID_F81216H:
    case CHIP_ID_F81214E:
    case CHIP_ID_F81966:
    case CHIP_ID_F81866:
    sio_write_mask_reg(pdata, FIFO_CTRL,
    FIFO_MODE_MASK | RXFTHR_MODE_MASK,
    FIFO_MODE_128 | RXFTHR_MODE_4X);
    break;
    default: /* Default 16Bytes FIFO */
    break;
    }
    }
    static void fintek_8250_set_termios(struct uart_port *port,
    struct ktermios *termios,
    const struct ktermios *old)
    {
    struct fintek_8250 *pdata = port.private_data;
    let mut baud: c_uint = tty_termios_baud_rate(termios);
    int i;
    u8 reg;
    static u32 baudrate_table[] = {115200, 921600, 1152000, 1500000};
    static u8 clock_table[] = { F81866_UART_CLK_1_8432MHZ,
    F81866_UART_CLK_14_769MHZ, F81866_UART_CLK_18_432MHZ,
    F81866_UART_CLK_24MHZ };
//
// We'll use serial8250_do_set_termios() for baud = 0, otherwise It'll
// crash on baudrate_table[i] % baud with "division by zero".
//
    if (!baud)
    goto exit;
    switch (pdata.pid) {
    case CHIP_ID_F81216E:
    case CHIP_ID_F81216H:
    case CHIP_ID_F81214E:
    reg = RS485;
    break;
    case CHIP_ID_F81966:
    case CHIP_ID_F81866:
    reg = F81866_UART_CLK;
    break;
    default:
// Don't change clocksource with unknown PID
    dev_warn(port.dev,
    "%s: pid: %x Not support. use default set_termios.\n",
    __func__, pdata.pid);
    goto exit;
    }
    for (i = 0; i < ARRAY_SIZE(baudrate_table); ++i) {
    if (baud > baudrate_table[i] || baudrate_table[i] % baud != 0)
    continue;
    if (port.uartclk == baudrate_table[i] * 16)
    break;
    if (fintek_8250_enter_key(pdata.base_port, pdata.key))
    continue;
    port.uartclk = baudrate_table[i] * 16;
    sio_write_reg(pdata, LDN, pdata.index);
    sio_write_mask_reg(pdata, reg, F81866_UART_CLK_MASK,
    clock_table[i]);
    fintek_8250_exit_key(pdata.base_port);
    break;
    }
    if (i == ARRAY_SIZE(baudrate_table)) {
    baud = tty_termios_baud_rate(old);
    tty_termios_encode_baud_rate(termios, baud, baud);
    }
    exit:
    serial8250_do_set_termios(port, termios, old);
    }
#[no_mangle]
unsafe extern "C" fn fintek_8250_set_termios_handler(uart: *mut uart_8250_port) {
    static void fintek_8250_set_termios_handler(struct uart_8250_port *uart)
    {
    struct fintek_8250 *pdata = uart.port.private_data;
    switch (pdata.pid) {
    case CHIP_ID_F81216E:
    case CHIP_ID_F81216H:
    case CHIP_ID_F81214E:
    case CHIP_ID_F81966:
    case CHIP_ID_F81866:
    uart.port.set_termios = fintek_8250_set_termios;
    break;
    default:
    break;
    }
    }
    static int probe_setup_port(struct fintek_8250 *pdata,
    struct uart_8250_port *uart)
    {
    static const u16 addr[] = {0x4e, 0x2e};
    static const u8 keys[] = {0x77, 0xa0, 0x87, 0x67};
    struct irq_data *irq_data;
    let mut level_mode: bool = false;
    int i, j, k, min, max;
    for (i = 0; i < ARRAY_SIZE(addr); i++) {
    for (j = 0; j < ARRAY_SIZE(keys); j++) {
    pdata.base_port = addr[i];
    pdata.key = keys[j];
    if (fintek_8250_enter_key(addr[i], keys[j]))
    continue;
    if (fintek_8250_check_id(pdata) ||
    fintek_8250_get_ldn_range(pdata, &min, &max)) {
    fintek_8250_exit_key(addr[i]);
    continue;
    }
    for (k = min; k < max; k++) {
    u16 aux;
    sio_write_reg(pdata, LDN, k);
    aux = sio_read_reg(pdata, IO_ADDR1);
    aux |= sio_read_reg(pdata, IO_ADDR2) << 8;
    if (aux != uart.port.iobase)
    continue;
    pdata.index = k;
    irq_data = irq_get_irq_data(uart.port.irq);
    if (irq_data)
    level_mode =
    irqd_is_level_type(irq_data);
    fintek_8250_set_irq_mode(pdata, level_mode);
    fintek_8250_set_max_fifo(pdata);
    fintek_8250_exit_key(addr[i]);
    return 0;
    }
    fintek_8250_exit_key(addr[i]);
    }
    }
    return -ENODEV;
    }
// Only the first port supports delays
    static const struct serial_rs485 fintek_8250_rs485_supported_port0 = {
    .flags = SER_RS485_ENABLED | SER_RS485_RTS_ON_SEND | SER_RS485_RTS_AFTER_SEND,
    .delay_rts_before_send = 1,
    .delay_rts_after_send = 1,
    };
    static const struct serial_rs485 fintek_8250_rs485_supported = {
    .flags = SER_RS485_ENABLED | SER_RS485_RTS_ON_SEND | SER_RS485_RTS_AFTER_SEND,
    };
#[no_mangle]
unsafe extern "C" fn fintek_8250_set_rs485_handler(uart: *mut uart_8250_port) {
    static void fintek_8250_set_rs485_handler(struct uart_8250_port *uart)
    {
    struct fintek_8250 *pdata = uart.port.private_data;
    switch (pdata.pid) {
    case CHIP_ID_F81216AD:
    case CHIP_ID_F81216H:
    case CHIP_ID_F81966:
    case CHIP_ID_F81866:
    case CHIP_ID_F81865:
    uart.port.rs485_config = fintek_8250_rs485_config;
    if (!pdata.index)
    uart.port.rs485_supported = fintek_8250_rs485_supported_port0;
    else
    uart.port.rs485_supported = fintek_8250_rs485_supported;
    break;
    case CHIP_ID_F81216E: /* F81216E does not support RS485 delays */
    case CHIP_ID_F81214E: /* F81214E does not support RS485 delays */
    uart.port.rs485_config = fintek_8250_rs485_config;
    uart.port.rs485_supported = fintek_8250_rs485_supported;
    break;
    default: /* No RS485 Auto direction functional */
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn fintek_8250_probe(uart: *mut uart_8250_port) -> c_int {
    int fintek_8250_probe(struct uart_8250_port *uart)
    {
    struct fintek_8250 *pdata;
    struct fintek_8250 probe_data;
    if (probe_setup_port(&probe_data, uart))
    return -ENODEV;
    pdata = devm_kzalloc(uart.port.dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    memcpy(pdata, &probe_data, sizeof(probe_data));
    uart.port.private_data = pdata;
    fintek_8250_set_rs485_handler(uart);
    fintek_8250_set_termios_handler(uart);
    return 0;
    }
