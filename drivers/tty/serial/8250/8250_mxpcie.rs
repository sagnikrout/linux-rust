//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_mxpcie.c
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
// Moxa PCIe multiport serial device driver
//
// Copyright (C) 2025 Moxa Inc. (support@moxa.com)
// Author: Crescent Hsieh <crescentcy.hsieh@moxa.com>
//

pub const PCI_DEVICE_ID_MOXA_CP102E: c_uint = 0x1024;
pub const PCI_DEVICE_ID_MOXA_CP102EL: c_uint = 0x1025;
pub const PCI_DEVICE_ID_MOXA_CP102N: c_uint = 0x1027;
pub const PCI_DEVICE_ID_MOXA_CP104EL_A: c_uint = 0x1045;
pub const PCI_DEVICE_ID_MOXA_CP104N: c_uint = 0x1046;
pub const PCI_DEVICE_ID_MOXA_CP112N: c_uint = 0x1121;
pub const PCI_DEVICE_ID_MOXA_CP114EL: c_uint = 0x1144;
pub const PCI_DEVICE_ID_MOXA_CP114N: c_uint = 0x1145;
pub const PCI_DEVICE_ID_MOXA_CP116E_A_A: c_uint = 0x1160;
pub const PCI_DEVICE_ID_MOXA_CP116E_A_B: c_uint = 0x1161;
pub const PCI_DEVICE_ID_MOXA_CP118EL_A: c_uint = 0x1182;
pub const PCI_DEVICE_ID_MOXA_CP118E_A_I: c_uint = 0x1183;
pub const PCI_DEVICE_ID_MOXA_CP132EL: c_uint = 0x1322;
pub const PCI_DEVICE_ID_MOXA_CP132N: c_uint = 0x1323;
pub const PCI_DEVICE_ID_MOXA_CP134EL_A: c_uint = 0x1342;
pub const PCI_DEVICE_ID_MOXA_CP134N: c_uint = 0x1343;
pub const PCI_DEVICE_ID_MOXA_CP138E_A: c_uint = 0x1381;
pub const PCI_DEVICE_ID_MOXA_CP168EL_A: c_uint = 0x1683;
// Bits in PCI device ID encoding board capabilities

// UART
pub const MOXA_PUART_BASE_BAUD: c_int = 921600;
pub const MOXA_PUART_OFFSET: c_uint = 0x200;
pub const MOXA_PUART_TX_TRIG_DEFAULT: c_int = 0;
pub const MOXA_PUART_RX_TRIG_DEFAULT: c_int = 96;
pub const MOXA_PUART_RX_FLOW_LOW_DEFAULT: c_int = 16;
pub const MOXA_PUART_RX_FLOW_HIGH_DEFAULT: c_int = 110;
// Special Function Register (SFR)
pub const MOXA_PUART_SFR: c_uint = 0x07;

// Enhanced Function Register (EFR)
pub const MOXA_PUART_EFR: c_uint = 0x0A;

pub const MOXA_PUART_EFR_RX_FLOW_DISABLED: c_uint = 0x0;
pub const MOXA_PUART_EFR_RX_FLOW_XON2_XOFF2: c_uint = 0x1;
pub const MOXA_PUART_EFR_RX_FLOW_XON1_XOFF1: c_uint = 0x2;
pub const MOXA_PUART_EFR_RX_FLOW_COPY_TX: c_uint = 0x3;

pub const MOXA_PUART_EFR_TX_FLOW_DISABLED: c_uint = 0x0;
pub const MOXA_PUART_EFR_TX_FLOW_XON2_XOFF2: c_uint = 0x1;
pub const MOXA_PUART_EFR_TX_FLOW_XON1_XOFF1: c_uint = 0x2;
pub const MOXA_PUART_EFR_TX_FLOW_RESERVED: c_uint = 0x3;
pub const MOXA_PUART_XON1: c_uint = 0x0B;
pub const MOXA_PUART_XON2: c_uint = 0x0C;
pub const MOXA_PUART_XOFF1: c_uint = 0x0D;
pub const MOXA_PUART_XOFF2: c_uint = 0x0E;
pub const MOXA_PUART_TTL: c_uint = 0x10	/* Tx Interrupt Trigger Level */;
pub const MOXA_PUART_RTL: c_uint = 0x11	/* Rx Interrupt Trigger Level */;
pub const MOXA_PUART_FCL: c_uint = 0x12	/* Flow Control Low Trigger Level */;
pub const MOXA_PUART_FCH: c_uint = 0x13	/* Flow Control High Trigger Level */;
pub const MOXA_PUART_RX_FIFO_CNT: c_uint = 0x15	/* Rx FIFO Data Counter */;
pub const MOXA_PUART_TX_FIFO_CNT: c_uint = 0x16	/* Tx FIFO Data Counter */;
pub const MOXA_PUART_RX_FIFO_MEM: c_uint = 0x100	/* Memory Space to Rx FIFO Data Register */;
pub const MOXA_PUART_TX_FIFO_MEM: c_uint = 0x100	/* Memory Space to Tx FIFO Data Register */;
pub const MOXA_GPIO_DIRECTION: c_uint = 0x09;
pub const MOXA_GPIO_OUTPUT: c_uint = 0x0A;

pub const MOXA_UIR_OFFSET: c_uint = 0x04;
pub const MOXA_UIR_RS232: c_uint = 0x00;
pub const MOXA_UIR_RS422: c_uint = 0x01;
pub const MOXA_UIR_RS485_4W: c_uint = 0x0B;
pub const MOXA_UIR_RS485_2W: c_uint = 0x0F;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxpcie8250_port {
    pub line: c_int,
    pub rx_trig_level: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxpcie8250 {
    pub supp_rs: c_uint,
    pub num_ports: c_uint,
    pub /: *mut *mut *mut void __iomem bar1_base; / UART registers (MMIO),
    pub /: *mut *mut *mut void __iomem bar2_base; / UIR / GPIO / CPLD (IO),
    pub __counted_by(num_ports): mxpcie8250_port port[],
}

    enum {
    MOXA_SUPP_RS232 = BIT(0),
    MOXA_SUPP_RS422 = BIT(1),
    MOXA_SUPP_RS485 = BIT(2),
    };
    static const struct serial_rs485 mxpcie8250_rs485_supported = {
    .flags = SER_RS485_ENABLED | SER_RS485_RTS_ON_SEND | SER_RS485_RX_DURING_TX | SER_RS485_MODE_RS422,
    };
#[no_mangle]
unsafe extern "C" fn mxpcie8250_is_mini_pcie(device: c_ushort) -> bool {
    static bool mxpcie8250_is_mini_pcie(unsigned short device)
    {
    if (device == PCI_DEVICE_ID_MOXA_CP102N ||
    device == PCI_DEVICE_ID_MOXA_CP104N ||
    device == PCI_DEVICE_ID_MOXA_CP112N ||
    device == PCI_DEVICE_ID_MOXA_CP114N ||
    device == PCI_DEVICE_ID_MOXA_CP132N ||
    device == PCI_DEVICE_ID_MOXA_CP134N)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn mxpcie8250_get_supp_rs(device: c_ushort) -> c_uint {
    static unsigned int mxpcie8250_get_supp_rs(unsigned short device)
    {
    switch (device & MOXA_DEV_ID_IFACE_MASK) {
    case 0x0000:
    case 0x0600:
    return MOXA_SUPP_RS232;
    case 0x0100:
    return MOXA_SUPP_RS232 | MOXA_SUPP_RS422 | MOXA_SUPP_RS485;
    case 0x0300:
    return MOXA_SUPP_RS422 | MOXA_SUPP_RS485;
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn mxpcie8250_get_nports(device: c_ushort) -> c_ushort {
    static unsigned short mxpcie8250_get_nports(unsigned short device)
    {
    switch (device) {
    case PCI_DEVICE_ID_MOXA_CP116E_A_A:
    case PCI_DEVICE_ID_MOXA_CP116E_A_B:
    return 8;
    default:
    return FIELD_GET(MOXA_DEV_ID_NPORTS_MASK, device);
    }
    }
    static void mxpcie8250_set_interface(struct mxpcie8250 *priv,
    unsigned int port_idx,
    u8 mode)
    {
    void __iomem *uir_addr = priv.bar2_base + MOXA_UIR_OFFSET + port_idx / 2;
    u8 cval;
    cval = ioread8(uir_addr);
    if (port_idx % 2)
    FIELD_MODIFY(MOXA_ODD_RS_MASK, &cval, mode);
    else
    FIELD_MODIFY(MOXA_EVEN_RS_MASK, &cval, mode);
    iowrite8(cval, uir_addr);
    }
//
// Moxa PCIe multiport serial boards support switching serial interfaces
// via the ioctl() command "TIOCSRS485". Supported modes and corresponding
// flags in "serial_rs485":
//
// RS232			= (no flags set)
// RS422			= SER_RS485_ENABLED | SER_RS485_MODE_RS422
// RS485_2W (half-duplex)	= SER_RS485_ENABLED
// RS485_4W (full-duplex)	= SER_RS485_ENABLED | SER_RS485_RX_DURING_TX
//
    static int mxpcie8250_rs485_config(struct uart_port *port,
    struct ktermios *termios,
    struct serial_rs485 *rs485)
    {
    struct mxpcie8250 *priv = dev_get_drvdata(port.dev);
    let mut mode: u8 = MOXA_UIR_RS232;
    if (rs485.flags & SER_RS485_ENABLED) {
    if (rs485.flags & SER_RS485_MODE_RS422)
    mode = MOXA_UIR_RS422;
#[no_mangle]
pub unsafe extern "C" fn if(SER_RS485_RX_DURING_TX: rs485->flags &) -> else {
    else if (rs485.flags & SER_RS485_RX_DURING_TX)
    mode = MOXA_UIR_RS485_4W;
    else
    mode = MOXA_UIR_RS485_2W;
    } else if (!(priv.supp_rs & MOXA_SUPP_RS232)) {
    return -ENODEV;
    }
    mxpcie8250_set_interface(priv, port.port_id, mode);
    return 0;
    }
    static void mxpcie8250_set_termios(struct uart_port *port,
    struct ktermios *new,
    const struct ktermios *old)
    {
    struct uart_8250_port *up = up_to_u8250p(port);
    struct tty_struct *tty = port.state.port.tty;
    let mut cflag: c_uint = tty.termios.c_cflag;
    u8 efr, val;
    serial8250_do_set_termios(port, new, old);
    up.port.status &= ~(UPSTAT_AUTORTS | UPSTAT_AUTOCTS | UPSTAT_AUTOXOFF);
    efr = serial_in(up, MOXA_PUART_EFR);
    efr &= ~(MOXA_PUART_EFR_AUTO_RTS | MOXA_PUART_EFR_AUTO_CTS);
    if (cflag & CRTSCTS) {
    efr |= (MOXA_PUART_EFR_AUTO_RTS | MOXA_PUART_EFR_AUTO_CTS);
    up.port.status |= (UPSTAT_AUTORTS | UPSTAT_AUTOCTS);
    }
// Set on-chip software flow control character
    serial_out(up, MOXA_PUART_XON1, START_CHAR(tty));
    serial_out(up, MOXA_PUART_XON2, START_CHAR(tty));
    serial_out(up, MOXA_PUART_XOFF1, STOP_CHAR(tty));
    serial_out(up, MOXA_PUART_XOFF2, STOP_CHAR(tty));
    val = I_IXON(tty) ? MOXA_PUART_EFR_RX_FLOW_XON1_XOFF1 : MOXA_PUART_EFR_RX_FLOW_DISABLED;
    FIELD_MODIFY(MOXA_PUART_EFR_RX_FLOW_MASK, &efr, val);
    val = I_IXOFF(tty) ? MOXA_PUART_EFR_TX_FLOW_XON1_XOFF1 : MOXA_PUART_EFR_TX_FLOW_DISABLED;
    FIELD_MODIFY(MOXA_PUART_EFR_TX_FLOW_MASK, &efr, val);
    if (I_IXOFF(tty))
    up.port.status |= UPSTAT_AUTOXOFF;
    serial_out(up, MOXA_PUART_EFR, efr);
    }
#[no_mangle]
unsafe extern "C" fn mxpcie8250_get_rxtrig(port: *mut uart_port) -> c_int {
    static int mxpcie8250_get_rxtrig(struct uart_port *port)
    {
    return serial_port_in(port, MOXA_PUART_RTL);
    }
#[no_mangle]
unsafe extern "C" fn mxpcie8250_set_rxtrig(port: *mut uart_port, bytes: c_uchar) -> c_int {
    static int mxpcie8250_set_rxtrig(struct uart_port *port, unsigned char bytes)
    {
    struct mxpcie8250 *priv = dev_get_drvdata(port.dev);
    if (bytes > port.fifosize)
    return -EINVAL;
    serial_port_out(port, MOXA_PUART_RTL, bytes);
    priv.port[port.port_id].rx_trig_level = bytes;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxpcie8250_startup(port: *mut uart_port) -> c_int {
    static int mxpcie8250_startup(struct uart_port *port)
    {
    struct mxpcie8250 *priv = dev_get_drvdata(port.dev);
    struct uart_8250_port *up = up_to_u8250p(port);
    int ret;
    ret = serial8250_do_startup(port);
    if (ret)
    return ret;
//
// The TX FIFO write pointer (w_ptr) and read pointer (r_ptr)
// are driven by different clocks: w_ptr uses the PCIe clock
// and r_ptr uses the UART clock. When TX FIFO flush is requested,
// w_ptr may be cleared before r_ptr, so the UART can still observe
// pending TX data.
//
// It is recommended to clear the FIFOs at least 5 times to ensure
// both pointers are reset.
//
    for (unsigned int i = 0; i < 5; ++i)
    serial_out(up, UART_FCR, UART_FCR_CLEAR_RCVR | UART_FCR_CLEAR_XMIT);
    serial_out(up, MOXA_PUART_EFR, MOXA_PUART_EFR_ENHANCED);
    serial_out(up, MOXA_PUART_SFR, MOXA_PUART_SFR_950);
    serial_out(up, MOXA_PUART_TTL, MOXA_PUART_TX_TRIG_DEFAULT);
    serial_out(up, MOXA_PUART_RTL, priv.port[port.port_id].rx_trig_level);
    serial_out(up, MOXA_PUART_FCL, MOXA_PUART_RX_FLOW_LOW_DEFAULT);
    serial_out(up, MOXA_PUART_FCH, MOXA_PUART_RX_FLOW_HIGH_DEFAULT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxpcie8250_shutdown(port: *mut uart_port) {
    static void mxpcie8250_shutdown(struct uart_port *port)
    {
    struct uart_8250_port *up = up_to_u8250p(port);
    serial_out(up, MOXA_PUART_EFR, 0);
    serial_out(up, MOXA_PUART_SFR, 0);
    serial8250_do_shutdown(port);
    }
#[no_mangle]
unsafe extern "C" fn mxpcie8250_throttle(port: *mut uart_port) {
    static void mxpcie8250_throttle(struct uart_port *port)
    {
    guard(uart_port_lock_irqsave)(port);
    port.ops.stop_rx(port);
    }
#[no_mangle]
unsafe extern "C" fn mxpcie8250_unthrottle(port: *mut uart_port) {
    static void mxpcie8250_unthrottle(struct uart_port *port)
    {
    struct uart_8250_port *up = up_to_u8250p(port);
    guard(uart_port_lock_irqsave)(port);
    up.ier |= UART_IER_RLSI | UART_IER_RDI;
    port.read_status_mask |= UART_LSR_DR;
    serial_out(up, UART_IER, up.ier);
    }
#[no_mangle]
unsafe extern "C" fn mxpcie8250_rx_chars(up: *mut uart_8250_port) {
    static void mxpcie8250_rx_chars(struct uart_8250_port *up)
    {
    struct uart_port *port = &up.port;
    struct tty_port *tport = &port.state.port;
    unsigned int count;
    u8 *buf;
    count = serial_in(up, MOXA_PUART_RX_FIFO_CNT);
    count = min(count, port.fifosize);
    count = tty_prepare_flip_string(tport, &buf, count);
    if (!count)
    return;
    for (unsigned int i = 0; i < count; ++i)
    buf[i] = serial_in(up, MOXA_PUART_RX_FIFO_MEM + i);
    port.icount.rx += count;
    tty_flip_buffer_push(tport);
    }
#[no_mangle]
unsafe extern "C" fn mxpcie8250_should_rx(up: *mut uart_8250_port, lsr: u16) -> bool {
    static bool mxpcie8250_should_rx(struct uart_8250_port *up, u16 lsr)
    {
    struct uart_port *port = &up.port;
    if (!(lsr & (UART_LSR_DR | UART_LSR_BI)))
    return false;
    if (!(port.status & (UPSTAT_AUTOCTS | UPSTAT_AUTORTS)))
    return true;
    if (lsr & (UART_LSR_FIFOE | UART_LSR_BRK_ERROR_BITS))
    return true;
    if (port.read_status_mask & UART_LSR_DR)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn mxpcie8250_tx_chars(up: *mut uart_8250_port) {
    static void mxpcie8250_tx_chars(struct uart_8250_port *up)
    {
    struct uart_port *port = &up.port;
    let mut offset: c_uint = 0;
    unsigned char c;
    uart_port_tx_limited(port, c, port.fifosize - serial_in(up, MOXA_PUART_TX_FIFO_CNT),
    true,
    serial_out(up, MOXA_PUART_TX_FIFO_MEM + offset++, c),
    ({}));
    }
#[no_mangle]
unsafe extern "C" fn mxpcie8250_handle_irq(port: *mut uart_port) -> c_int {
    static int mxpcie8250_handle_irq(struct uart_port *port)
    {
    struct uart_8250_port *up = up_to_u8250p(port);
    u16 lsr;
    u8 iir;
    iir = serial_in(up, UART_IIR);
    if (iir & UART_IIR_NO_INT)
    return 0;
    guard(uart_port_lock_check_sysrq_irqsave)(port);
    lsr = serial_lsr_in(up);
    if (mxpcie8250_should_rx(up, lsr)) {
    if (!(lsr & UART_LSR_BRK_ERROR_BITS))
    mxpcie8250_rx_chars(up);
    else
    lsr = serial8250_rx_chars(up, lsr);
    }
    serial8250_modem_status(up);
    if ((lsr & UART_LSR_THRE) && (up.ier & UART_IER_THRI))
    mxpcie8250_tx_chars(up);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn mxpcie8250_software_break_ctl(port: *mut uart_port, break_state: c_int) {
    static void mxpcie8250_software_break_ctl(struct uart_port *port, int break_state)
    {
    struct uart_8250_port *up = up_to_u8250p(port);
    struct tty_struct *tty = port.state.port.tty;
    unsigned int baud, quot;
    u8 sfr, tx_byte = 0x01;
    guard(uart_port_lock_irqsave)(port);
    if (break_state == -1) {
    serial_out(up, UART_LCR, up.lcr | UART_LCR_DLAB);
    serial_dl_write(up, 0);
    serial_out(up, UART_LCR, up.lcr);
    serial_out(up, MOXA_PUART_TX_FIFO_MEM, tx_byte);
    sfr = serial_in(up, MOXA_PUART_SFR);
    serial_out(up, MOXA_PUART_SFR, sfr | MOXA_PUART_SFR_FORCE_TX);
    up.lcr |= UART_LCR_SBC;
    serial_out(up, UART_LCR, up.lcr);
    } else {
    up.lcr &= ~UART_LCR_SBC;
    serial_out(up, UART_LCR, up.lcr);
    sfr = serial_in(up, MOXA_PUART_SFR);
    serial_out(up, MOXA_PUART_SFR, sfr & ~MOXA_PUART_SFR_FORCE_TX);
    serial_out(up, UART_FCR, UART_FCR_CLEAR_XMIT);
    baud = tty_get_baud_rate(tty);
    quot = uart_get_divisor(port, baud);
    serial8250_do_set_divisor(port, baud, quot);
    serial_out(up, UART_LCR, up.lcr);
    }
    }
#[no_mangle]
unsafe extern "C" fn mxpcie8250_break_ctl(port: *mut uart_port, break_state: c_int) {
    static void mxpcie8250_break_ctl(struct uart_port *port, int break_state)
    {
    if (port.rs485.flags & SER_RS485_ENABLED &&
    !(port.rs485.flags & SER_RS485_MODE_RS422))
    mxpcie8250_software_break_ctl(port, break_state);
    else
    serial8250_do_break_ctl(port, break_state);
    }
#[no_mangle]
unsafe extern "C" fn mxpcie8250_init_board(pdev: *mut pci_dev, priv: *mut mxpcie8250) {
    static void mxpcie8250_init_board(struct pci_dev *pdev, struct mxpcie8250 *priv)
    {
    void __iomem *bar2_base = priv.bar2_base;
    let mut device: c_ushort = pdev.device;
    u8 cval;
// Initial terminator
    if (device == PCI_DEVICE_ID_MOXA_CP114EL ||
    device == PCI_DEVICE_ID_MOXA_CP118EL_A) {
    iowrite8(0xff, bar2_base + MOXA_GPIO_DIRECTION);
    iowrite8(0x00, bar2_base + MOXA_GPIO_OUTPUT);
    }
//
// Enable hardware buffer to prevent break signal output when system boots up.
// This hardware buffer is only supported on Mini PCIe series.
//
    if (mxpcie8250_is_mini_pcie(device)) {
// Set GPIO direction
    cval = ioread8(bar2_base + MOXA_GPIO_DIRECTION);
    cval |= MOXA_GPIO_PIN2;
    iowrite8(cval, bar2_base + MOXA_GPIO_DIRECTION);
// Enable low GPIO
    cval = ioread8(bar2_base + MOXA_GPIO_OUTPUT);
    cval &= ~MOXA_GPIO_PIN2;
    iowrite8(cval, bar2_base + MOXA_GPIO_OUTPUT);
    }
    }
    static void mxpcie8250_setup_port(struct pci_dev *pdev,
    struct mxpcie8250 *priv,
    struct uart_8250_port *up,
    int idx)
    {
    let mut device: c_ushort = pdev.device;
    let mut offset: c_int = idx * MOXA_PUART_OFFSET;
    let mut init_mode: u8 = MOXA_UIR_RS232;
    if (priv.supp_rs & MOXA_SUPP_RS485) {
    up.port.rs485_config = mxpcie8250_rs485_config;
    up.port.rs485_supported = mxpcie8250_rs485_supported;
    }
    if (!(priv.supp_rs & MOXA_SUPP_RS232)) {
    init_mode = MOXA_UIR_RS422;
    up.port.rs485.flags = SER_RS485_ENABLED | SER_RS485_MODE_RS422;
    }
    mxpcie8250_set_interface(priv, idx, init_mode);
    if (idx == 3 &&
    (device == PCI_DEVICE_ID_MOXA_CP104EL_A ||
    device == PCI_DEVICE_ID_MOXA_CP114EL   ||
    device == PCI_DEVICE_ID_MOXA_CP134EL_A))
    offset = 7 * MOXA_PUART_OFFSET;
    up.port.mapbase = pci_resource_start(pdev, FL_BASE1) + offset;
    up.port.membase = pcim_iomap_table(pdev)[FL_BASE1] + offset;
    }
#[no_mangle]
unsafe extern "C" fn mxpcie8250_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int mxpcie8250_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    struct device *dev = &pdev.dev;
    let mut up: uart_8250_port = {};
    struct mxpcie8250 *priv;
    let mut device: c_ushort = pdev.device;
    unsigned int num_ports;
    int ret;
    ret = pcim_enable_device(pdev);
    if (ret)
    return ret;
    num_ports = mxpcie8250_get_nports(device);
    priv = devm_kzalloc(dev, struct_size(priv, port, num_ports), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.num_ports = num_ports;
    priv.supp_rs = mxpcie8250_get_supp_rs(device);
    priv.bar1_base = pcim_iomap(pdev, FL_BASE1, 0);
    if (!priv.bar1_base)
    return -ENOMEM;
    priv.bar2_base = pcim_iomap(pdev, FL_BASE2, 0);
    if (!priv.bar2_base)
    return -ENOMEM;
    mxpcie8250_init_board(pdev, priv);
    up.port.dev = dev;
    up.port.irq = pdev.irq;
    up.port.uartclk = MOXA_PUART_BASE_BAUD * 16;
    up.port.flags = UPF_SKIP_TEST | UPF_BOOT_AUTOCONF | UPF_SHARE_IRQ | UPF_FIXED_TYPE;
    up.port.type = PORT_MUEX50;
    up.port.iotype = UPIO_MEM;
    up.port.iobase = 0;
    up.port.regshift = 0;
    up.port.set_termios = mxpcie8250_set_termios;
    up.port.get_rxtrig = mxpcie8250_get_rxtrig;
    up.port.set_rxtrig = mxpcie8250_set_rxtrig;
    up.port.startup = mxpcie8250_startup;
    up.port.shutdown = mxpcie8250_shutdown;
    up.port.throttle = mxpcie8250_throttle;
    up.port.unthrottle = mxpcie8250_unthrottle;
    up.port.handle_irq = mxpcie8250_handle_irq;
    up.port.break_ctl = mxpcie8250_break_ctl;
    for (unsigned int i = 0; i < num_ports; i++) {
    mxpcie8250_setup_port(pdev, priv, &up, i);
    dev_dbg(dev, "Setup PCI port: port %lx, irq %d, type %d\n",
    up.port.iobase, up.port.irq, up.port.iotype);
    priv.port[i].line = serial8250_register_8250_port(&up);
    if (priv.port[i].line < 0) {
    dev_err(dev,
    "Couldn't register serial port %lx, irq %d, type %d, error %d\n",
    up.port.iobase, up.port.irq,
    up.port.iotype, priv.port[i].line);
    break;
    }
    priv.port[i].rx_trig_level = MOXA_PUART_RX_TRIG_DEFAULT;
    }
    pci_set_drvdata(pdev, priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxpcie8250_remove(pdev: *mut pci_dev) {
    static void mxpcie8250_remove(struct pci_dev *pdev)
    {
    struct mxpcie8250 *priv = pci_get_drvdata(pdev);
    for (unsigned int i = 0; i < priv.num_ports; i++)
    serial8250_unregister_port(priv.port[i].line);
    }
    static const struct pci_device_id mxpcie8250_pci_ids[] = {
    { PCI_VDEVICE(MOXA, PCI_DEVICE_ID_MOXA_CP102E) },
    { PCI_VDEVICE(MOXA, PCI_DEVICE_ID_MOXA_CP102EL) },
    { PCI_VDEVICE(MOXA, PCI_DEVICE_ID_MOXA_CP102N) },
    { PCI_VDEVICE(MOXA, PCI_DEVICE_ID_MOXA_CP104EL_A) },
    { PCI_VDEVICE(MOXA, PCI_DEVICE_ID_MOXA_CP104N) },
    { PCI_VDEVICE(MOXA, PCI_DEVICE_ID_MOXA_CP112N) },
    { PCI_VDEVICE(MOXA, PCI_DEVICE_ID_MOXA_CP114EL) },
    { PCI_VDEVICE(MOXA, PCI_DEVICE_ID_MOXA_CP114N) },
    { PCI_VDEVICE(MOXA, PCI_DEVICE_ID_MOXA_CP116E_A_A) },
    { PCI_VDEVICE(MOXA, PCI_DEVICE_ID_MOXA_CP116E_A_B) },
    { PCI_VDEVICE(MOXA, PCI_DEVICE_ID_MOXA_CP118EL_A) },
    { PCI_VDEVICE(MOXA, PCI_DEVICE_ID_MOXA_CP118E_A_I) },
    { PCI_VDEVICE(MOXA, PCI_DEVICE_ID_MOXA_CP132EL) },
    { PCI_VDEVICE(MOXA, PCI_DEVICE_ID_MOXA_CP132N) },
    { PCI_VDEVICE(MOXA, PCI_DEVICE_ID_MOXA_CP134EL_A) },
    { PCI_VDEVICE(MOXA, PCI_DEVICE_ID_MOXA_CP134N) },
    { PCI_VDEVICE(MOXA, PCI_DEVICE_ID_MOXA_CP138E_A) },
    { PCI_VDEVICE(MOXA, PCI_DEVICE_ID_MOXA_CP168EL_A) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, mxpcie8250_pci_ids);
    static struct pci_driver mxpcie8250_pci_driver = {
    .name		= "8250_mxpcie",
    .id_table	= mxpcie8250_pci_ids,
    .probe		= mxpcie8250_probe,
    .remove		= mxpcie8250_remove,
    };
    module_pci_driver(mxpcie8250_pci_driver);
    MODULE_AUTHOR("Moxa Inc.");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Moxa PCIe Multiport Serial Device Driver");
