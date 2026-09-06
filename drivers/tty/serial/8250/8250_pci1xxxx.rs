//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_pci1xxxx.c
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
// Probe module for 8250/16550-type MCHP PCI serial ports.
//
// Based on drivers/tty/serial/8250/8250_pci.c,
//
// Copyright (C) 2022 Microchip Technology Inc., All Rights Reserved.
//

pub const PCI_DEVICE_ID_EFAR_PCI12000: c_uint = 0xa002;
pub const PCI_DEVICE_ID_EFAR_PCI11010: c_uint = 0xa012;
pub const PCI_DEVICE_ID_EFAR_PCI11101: c_uint = 0xa022;
pub const PCI_DEVICE_ID_EFAR_PCI11400: c_uint = 0xa032;
pub const PCI_DEVICE_ID_EFAR_PCI11414: c_uint = 0xa042;
pub const PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_4p: c_uint = 0x0001;
pub const PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_3p012: c_uint = 0x0002;
pub const PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_3p013: c_uint = 0x0003;
pub const PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_3p023: c_uint = 0x0004;
pub const PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_3p123: c_uint = 0x0005;
pub const PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_2p01: c_uint = 0x0006;
pub const PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_2p02: c_uint = 0x0007;
pub const PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_2p03: c_uint = 0x0008;
pub const PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_2p12: c_uint = 0x0009;
pub const PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_2p13: c_uint = 0x000a;
pub const PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_2p23: c_uint = 0x000b;
pub const PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_1p0: c_uint = 0x000c;
pub const PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_1p1: c_uint = 0x000d;
pub const PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_1p2: c_uint = 0x000e;
pub const PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_1p3: c_uint = 0x000f;

pub const UART_SYSTEM_ADDR_BASE: c_uint = 0x1000;

pub const SYSLOCK_SLEEP_TIMEOUT: c_int = 100;
pub const SYSLOCK_RETRY_CNT: c_int = 1000;
pub const UART_RX_BYTE_FIFO: c_uint = 0x00;
pub const UART_TX_BYTE_FIFO: c_uint = 0x00;
pub const UART_FIFO_CTL: c_uint = 0x02;
pub const UART_MODEM_CTL_REG: c_uint = 0x04;

pub const UART_LINE_STAT_REG: c_uint = 0x05;

pub const UART_ACTV_REG: c_uint = 0x11;

pub const UART_PCI_CTRL_REG: c_uint = 0x80;

pub const ADCL_CFG_REG: c_uint = 0x40;

pub const UART_BIT_SAMPLE_CNT_8: c_int = 8;
pub const UART_BIT_SAMPLE_CNT_16: c_int = 16;

pub const UART_WAKE_REG: c_uint = 0x8C;
pub const UART_WAKE_MASK_REG: c_uint = 0x90;

    (UART_WAKE_N_PIN | UART_WAKE_NCTS | UART_WAKE_INT)
pub const UART_BAUD_CLK_DIVISOR_REG: c_uint = 0x54;
pub const FRAC_DIV_CFG_REG: c_uint = 0x58;
pub const UART_RESET_REG: c_uint = 0x94;

pub const UART_BURST_STATUS_REG: c_uint = 0x9C;
pub const UART_TX_BURST_FIFO: c_uint = 0xA0;
pub const UART_RX_BURST_FIFO: c_uint = 0xA4;
pub const UART_BIT_DIVISOR_8: c_uint = 0x26731000;
pub const UART_BIT_DIVISOR_16: c_uint = 0x6ef71000;
pub const UART_BAUD_4MBPS: c_int = 4000000;
pub const MAX_PORTS: c_int = 4;
pub const PORT_OFFSET: c_uint = 0x100;
pub const RX_BUF_SIZE: c_int = 512;
pub const UART_BYTE_SIZE: c_int = 1;
pub const UART_BURST_SIZE: c_int = 4;
pub const UART_BST_STAT_RX_COUNT_MASK: c_uint = 0x00FF;
pub const UART_BST_STAT_TX_COUNT_MASK: c_uint = 0xFF00;
pub const UART_BST_STAT_IIR_INT_PEND: c_uint = 0x100000;
pub const UART_LSR_OVERRUN_ERR_CLR: c_uint = 0x43;
pub const UART_BST_STAT_LSR_RX_MASK: c_uint = 0x9F000000;
pub const UART_BST_STAT_LSR_RX_ERR_MASK: c_uint = 0x9E000000;
pub const UART_BST_STAT_LSR_OVERRUN_ERR: c_uint = 0x2000000;
pub const UART_BST_STAT_LSR_PARITY_ERR: c_uint = 0x4000000;
pub const UART_BST_STAT_LSR_FRAME_ERR: c_uint = 0x8000000;
pub const UART_BST_STAT_LSR_THRE: c_uint = 0x20000000;

    != GET_RTS_PIN_STATUS(val))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci1xxxx_8250 {
    pub nr: c_uint,
    pub dev_rev: u8,
    pub pad: [u8; 3],
    pub membase: *mut void __iomem,
    pub __counted_by(nr): int line[],
}

    static const struct serial_rs485 pci1xxxx_rs485_supported = {
    .flags = SER_RS485_ENABLED | SER_RS485_RTS_ON_SEND |
    SER_RS485_RTS_AFTER_SEND,
    .delay_rts_after_send = 1,
// Delay RTS before send is not supported
    };
#[no_mangle]
unsafe extern "C" fn pci1xxxx_set_sys_lock(port: *mut pci1xxxx_8250) -> c_int {
    static int pci1xxxx_set_sys_lock(struct pci1xxxx_8250 *port)
    {
    writel(UART_SYSLOCK, port.membase + UART_SYSLOCK_REG);
    return readl(port.membase + UART_SYSLOCK_REG);
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_acquire_sys_lock(port: *mut pci1xxxx_8250) -> c_int {
    static int pci1xxxx_acquire_sys_lock(struct pci1xxxx_8250 *port)
    {
    u32 regval;
    return readx_poll_timeout(pci1xxxx_set_sys_lock, port, regval,
    (regval & UART_SYSLOCK),
    SYSLOCK_SLEEP_TIMEOUT,
    SYSLOCK_RETRY_CNT * SYSLOCK_SLEEP_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_release_sys_lock(port: *mut pci1xxxx_8250) {
    static void pci1xxxx_release_sys_lock(struct pci1xxxx_8250 *port)
    {
    writel(0x0, port.membase + UART_SYSLOCK_REG);
    }
    static const int logical_to_physical_port_idx[][MAX_PORTS] = {
    {0,  1,  2,  3}, /* PCI12000, PCI11010, PCI11101, PCI11400, PCI11414 */
    {0,  1,  2,  3}, /* PCI4p */
    {0,  1,  2, -1}, /* PCI3p012 */
    {0,  1,  3, -1}, /* PCI3p013 */
    {0,  2,  3, -1}, /* PCI3p023 */
    {1,  2,  3, -1}, /* PCI3p123 */
    {0,  1, -1, -1}, /* PCI2p01 */
    {0,  2, -1, -1}, /* PCI2p02 */
    {0,  3, -1, -1}, /* PCI2p03 */
    {1,  2, -1, -1}, /* PCI2p12 */
    {1,  3, -1, -1}, /* PCI2p13 */
    {2,  3, -1, -1}, /* PCI2p23 */
    {0, -1, -1, -1}, /* PCI1p0 */
    {1, -1, -1, -1}, /* PCI1p1 */
    {2, -1, -1, -1}, /* PCI1p2 */
    {3, -1, -1, -1}, /* PCI1p3 */
    };
#[no_mangle]
unsafe extern "C" fn pci1xxxx_get_num_ports(dev: *mut pci_dev) -> c_int {
    static int pci1xxxx_get_num_ports(struct pci_dev *dev)
    {
    switch (dev.subsystem_device) {
    case PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_1p0:
    case PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_1p1:
    case PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_1p2:
    case PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_1p3:
    case PCI_SUBDEVICE_ID_EFAR_PCI12000:
    case PCI_SUBDEVICE_ID_EFAR_PCI11010:
    case PCI_SUBDEVICE_ID_EFAR_PCI11101:
    case PCI_SUBDEVICE_ID_EFAR_PCI11400:
    default:
    return 1;
    case PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_2p01:
    case PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_2p02:
    case PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_2p03:
    case PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_2p12:
    case PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_2p13:
    case PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_2p23:
    return 2;
    case PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_3p012:
    case PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_3p123:
    case PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_3p013:
    case PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_3p023:
    return 3;
    case PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_4p:
    case PCI_SUBDEVICE_ID_EFAR_PCI11414:
    return 4;
    }
    }
    static unsigned int pci1xxxx_get_divisor(struct uart_port *port,
    unsigned int baud, unsigned int *frac)
    {
    unsigned int uart_sample_cnt;
    unsigned int quot;
    if (baud >= UART_BAUD_4MBPS)
    uart_sample_cnt = UART_BIT_SAMPLE_CNT_8;
    else
    uart_sample_cnt = UART_BIT_SAMPLE_CNT_16;
//
// Calculate baud rate sampling period in nanoseconds.
// Fractional part x denotes x/255 parts of a nanosecond.
//
    quot = NSEC_PER_SEC / (baud * uart_sample_cnt);
// frac = (NSEC_PER_SEC - quot * baud * uart_sample_cnt)
    255 / uart_sample_cnt / baud;
    return quot;
    }
    static void pci1xxxx_set_divisor(struct uart_port *port, unsigned int baud,
    unsigned int quot, unsigned int frac)
    {
    if (baud >= UART_BAUD_4MBPS)
    writel(UART_BIT_DIVISOR_8, port.membase + FRAC_DIV_CFG_REG);
    else
    writel(UART_BIT_DIVISOR_16, port.membase + FRAC_DIV_CFG_REG);
    writel(FIELD_PREP(BAUD_CLOCK_DIV_INT_MSK, quot) | frac,
    port.membase + UART_BAUD_CLK_DIVISOR_REG);
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_set_mctrl(port: *mut uart_port, mctrl: c_uint) {
    static void pci1xxxx_set_mctrl(struct uart_port *port, unsigned int mctrl)
    {
    u32 fract_div_cfg_reg;
    u32 line_stat_reg;
    u32 modem_ctl_reg;
    u32 adcl_cfg_reg;
    adcl_cfg_reg = readl(port.membase + ADCL_CFG_REG);
// HW is responsible in ADCL_EN case
    if ((adcl_cfg_reg & (ADCL_CFG_EN | ADCL_CFG_PIN_SEL)))
    return;
    modem_ctl_reg = readl(port.membase + UART_MODEM_CTL_REG);
    serial8250_do_set_mctrl(port, mctrl);
    if (RTS_TOGGLE_STATUS_MASK(mctrl, modem_ctl_reg)) {
    line_stat_reg = readl(port.membase + UART_LINE_STAT_REG);
    if (line_stat_reg & UART_LINE_XMIT_CHECK_MASK) {
    fract_div_cfg_reg = readl(port.membase +
    FRAC_DIV_CFG_REG);
    writel((fract_div_cfg_reg &
    ~(FRAC_DIV_TX_END_POINT_MASK)),
    port.membase + FRAC_DIV_CFG_REG);
// Enable ADC and set the nRTS pin
    writel((adcl_cfg_reg | (ADCL_CFG_EN |
    ADCL_CFG_PIN_SEL)),
    port.membase + ADCL_CFG_REG);
// Revert to the original settings
    writel(adcl_cfg_reg, port.membase + ADCL_CFG_REG);
    writel(fract_div_cfg_reg, port.membase +
    FRAC_DIV_CFG_REG);
    }
    }
    }
    static int pci1xxxx_rs485_config(struct uart_port *port,
    struct ktermios *termios,
    struct serial_rs485 *rs485)
    {
    u32 delay_in_baud_periods;
    u32 baud_period_in_ns;
    let mut mode_cfg: u32 = 0;
    u32 sample_cnt;
    u32 clock_div;
    u32 frac_div;
    frac_div = readl(port.membase + FRAC_DIV_CFG_REG);
    if (frac_div == UART_BIT_DIVISOR_16)
    sample_cnt = UART_BIT_SAMPLE_CNT_16;
    else
    sample_cnt = UART_BIT_SAMPLE_CNT_8;
//
// pci1xxxx's uart hardware supports only RTS delay after
// Tx and in units of bit times to a maximum of 15
//
    if (rs485.flags & SER_RS485_ENABLED) {
    mode_cfg = ADCL_CFG_EN | ADCL_CFG_PIN_SEL;
    if (!(rs485.flags & SER_RS485_RTS_ON_SEND))
    mode_cfg |= ADCL_CFG_POL_SEL;
    if (rs485.delay_rts_after_send) {
    clock_div = readl(port.membase + UART_BAUD_CLK_DIVISOR_REG);
    baud_period_in_ns =
    FIELD_GET(BAUD_CLOCK_DIV_INT_MSK, clock_div) *
    sample_cnt;
    delay_in_baud_periods =
    rs485.delay_rts_after_send * NSEC_PER_MSEC /
    baud_period_in_ns;
    delay_in_baud_periods =
    min_t(u32, delay_in_baud_periods,
    FIELD_MAX(ADCL_CFG_RTS_DELAY_MASK));
    mode_cfg |= FIELD_PREP(ADCL_CFG_RTS_DELAY_MASK,
    delay_in_baud_periods);
    rs485.delay_rts_after_send =
    baud_period_in_ns * delay_in_baud_periods /
    NSEC_PER_MSEC;
    }
    }
    writel(mode_cfg, port.membase + ADCL_CFG_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_read_burst_status(port: *mut uart_port) -> u32 {
    static u32 pci1xxxx_read_burst_status(struct uart_port *port)
    {
    u32 status;
    status = readl(port.membase + UART_BURST_STATUS_REG);
    if (status & UART_BST_STAT_LSR_RX_ERR_MASK) {
    if (status & UART_BST_STAT_LSR_OVERRUN_ERR) {
    writeb(UART_LSR_OVERRUN_ERR_CLR,
    port.membase + UART_FIFO_CTL);
    port.icount.overrun++;
    }
    if (status & UART_BST_STAT_LSR_FRAME_ERR)
    port.icount.frame++;
    if (status & UART_BST_STAT_LSR_PARITY_ERR)
    port.icount.parity++;
    }
    return status;
    }
    static void pci1xxxx_process_read_data(struct uart_port *port,
    unsigned char *rx_buff, u32 *buff_index,
    u32 *valid_byte_count)
    {
    let mut valid_burst_count: u32 = *valid_byte_count / UART_BURST_SIZE;
    u32 *burst_buf;
//
// Depending on the RX Trigger Level the number of bytes that can be
// stored in RX FIFO at a time varies. Each transaction reads data
// in DWORDs. If there are less than four remaining valid_byte_count
// to read, the data is received one byte at a time.
//
    while (valid_burst_count--) {
    if (*buff_index > (RX_BUF_SIZE - UART_BURST_SIZE))
    break;
    burst_buf = (u32 *)&rx_buff[*buff_index];
// burst_buf = readl(port->membase + UART_RX_BURST_FIFO);
// buff_index += UART_BURST_SIZE;
// valid_byte_count -= UART_BURST_SIZE;
    }
    while (*valid_byte_count) {
    if (*buff_index >= RX_BUF_SIZE)
    break;
    rx_buff[*buff_index] = readb(port.membase +
    UART_RX_BYTE_FIFO);
// buff_index += UART_BYTE_SIZE;
// valid_byte_count -= UART_BYTE_SIZE;
    }
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_rx_burst(port: *mut uart_port, uart_status: u32) {
    static void pci1xxxx_rx_burst(struct uart_port *port, u32 uart_status)
    {
    let mut valid_byte_count: u32 = uart_status & UART_BST_STAT_RX_COUNT_MASK;
    struct tty_port *tty_port = &port.state.port;
    unsigned char rx_buff[RX_BUF_SIZE];
    let mut buff_index: u32 = 0;
    u32 copied_len;
    if (valid_byte_count != 0 &&
    valid_byte_count < RX_BUF_SIZE) {
    pci1xxxx_process_read_data(port, rx_buff, &buff_index,
    &valid_byte_count);
    copied_len = (u32)tty_insert_flip_string(tty_port, rx_buff,
    buff_index);
    if (copied_len != buff_index)
    port.icount.overrun += buff_index - copied_len;
    port.icount.rx += buff_index;
    tty_flip_buffer_push(tty_port);
    }
    }
    static void pci1xxxx_process_write_data(struct uart_port *port,
    int *data_empty_count,
    u32 *valid_byte_count)
    {
    struct tty_port *tport = &port.state.port;
    let mut valid_burst_count: u32 = *valid_byte_count / UART_BURST_SIZE;
//
// Each transaction transfers data in DWORDs. If there are less than
// four remaining valid_byte_count to transfer or if the circular
// buffer has insufficient space for a DWORD, the data is transferred
// one byte at a time.
//
    while (valid_burst_count) {
    u32 c;
    if (*data_empty_count - UART_BURST_SIZE < 0)
    break;
    if (kfifo_len(&tport.xmit_fifo) < UART_BURST_SIZE)
    break;
    if (WARN_ON(kfifo_out(&tport.xmit_fifo, (u8 *)&c, sizeof(c)) !=
    sizeof(c)))
    break;
    writel(c, port.membase + UART_TX_BURST_FIFO);
// valid_byte_count -= UART_BURST_SIZE;
// data_empty_count -= UART_BURST_SIZE;
    valid_burst_count -= UART_BYTE_SIZE;
    }
    while (*valid_byte_count) {
    u8 c;
    if (!kfifo_get(&tport.xmit_fifo, &c))
    break;
    writeb(c, port.membase + UART_TX_BYTE_FIFO);
// data_empty_count -= UART_BYTE_SIZE;
// valid_byte_count -= UART_BYTE_SIZE;
//
// If there are any pending burst count, data is handled by
// transmitting DWORDs at a time.
//
    if (valid_burst_count &&
    kfifo_len(&tport.xmit_fifo) >= UART_BURST_SIZE)
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_tx_burst(port: *mut uart_port, uart_status: u32) {
    static void pci1xxxx_tx_burst(struct uart_port *port, u32 uart_status)
    {
    struct uart_8250_port *up = up_to_u8250p(port);
    struct tty_port *tport = &port.state.port;
    u32 valid_byte_count;
    int data_empty_count;
    if (port.x_char) {
    writeb(port.x_char, port.membase + UART_TX);
    port.icount.tx++;
    port.x_char = 0;
    return;
    }
    if ((uart_tx_stopped(port)) || kfifo_is_empty(&tport.xmit_fifo)) {
    port.ops.stop_tx(port);
    } else {
    data_empty_count = (pci1xxxx_read_burst_status(port) &
    UART_BST_STAT_TX_COUNT_MASK) >> 8;
    do {
    valid_byte_count = kfifo_len(&tport.xmit_fifo);
    pci1xxxx_process_write_data(port,
    &data_empty_count,
    &valid_byte_count);
    port.icount.tx++;
    if (kfifo_is_empty(&tport.xmit_fifo))
    break;
    } while (data_empty_count && valid_byte_count);
    }
    if (kfifo_len(&tport.xmit_fifo) < WAKEUP_CHARS)
    uart_write_wakeup(port);
//
// With RPM enabled, we have to wait until the FIFO is empty before
// the HW can go idle. So we get here once again with empty FIFO and
// disable the interrupt and RPM in __stop_tx()
//
    if (kfifo_is_empty(&tport.xmit_fifo) &&
    !(up.capabilities & UART_CAP_RPM))
    port.ops.stop_tx(port);
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_handle_irq(port: *mut uart_port) -> c_int {
    static int pci1xxxx_handle_irq(struct uart_port *port)
    {
    unsigned long flags;
    u32 status;
    status = pci1xxxx_read_burst_status(port);
    if (status & UART_BST_STAT_IIR_INT_PEND)
    return 0;
    spin_lock_irqsave(&port.lock, flags);
    if (status & UART_BST_STAT_LSR_RX_MASK)
    pci1xxxx_rx_burst(port, status);
    if (status & UART_BST_STAT_LSR_THRE)
    pci1xxxx_tx_burst(port, status);
    spin_unlock_irqrestore(&port.lock, flags);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_port_suspend(line: c_int) -> bool {
    static bool pci1xxxx_port_suspend(int line)
    {
    struct uart_8250_port *up = serial8250_get_port(line);
    struct uart_port *port = &up.port;
    struct tty_port *tport = &port.state.port;
    unsigned long flags;
    let mut ret: bool = false;
    u8 wakeup_mask;
    mutex_lock(&tport.mutex);
    if (port.suspended == 0 && port.dev) {
    wakeup_mask = readb(up.port.membase + UART_WAKE_MASK_REG);
    uart_port_lock_irqsave(port, &flags);
    port.mctrl &= ~TIOCM_OUT2;
    port.ops.set_mctrl(port, port.mctrl);
    uart_port_unlock_irqrestore(port, flags);
    ret = (wakeup_mask & UART_WAKE_SRCS) != UART_WAKE_SRCS;
    }
    writeb(UART_WAKE_SRCS, port.membase + UART_WAKE_REG);
    mutex_unlock(&tport.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_port_resume(line: c_int) {
    static void pci1xxxx_port_resume(int line)
    {
    struct uart_8250_port *up = serial8250_get_port(line);
    struct uart_port *port = &up.port;
    struct tty_port *tport = &port.state.port;
    unsigned long flags;
    mutex_lock(&tport.mutex);
    writeb(UART_BLOCK_SET_ACTIVE, port.membase + UART_ACTV_REG);
    writeb(UART_WAKE_SRCS, port.membase + UART_WAKE_REG);
    if (port.suspended == 0) {
    uart_port_lock_irqsave(port, &flags);
    port.mctrl |= TIOCM_OUT2;
    port.ops.set_mctrl(port, port.mctrl);
    uart_port_unlock_irqrestore(port, flags);
    }
    mutex_unlock(&tport.mutex);
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_suspend(dev: *mut device) -> c_int {
    static int pci1xxxx_suspend(struct device *dev)
    {
    struct pci1xxxx_8250 *priv = dev_get_drvdata(dev);
    struct pci_dev *pcidev = to_pci_dev(dev);
    let mut wakeup: bool = false;
    unsigned int data;
    void __iomem *p;
    int i;
    for (i = 0; i < priv.nr; i++) {
    if (priv.line[i] >= 0) {
    serial8250_suspend_port(priv.line[i]);
    wakeup |= pci1xxxx_port_suspend(priv.line[i]);
    }
    }
    p = pci_ioremap_bar(pcidev, 0);
    if (!p) {
    dev_err(dev, "remapping of bar 0 memory failed");
    return -ENOMEM;
    }
    data = readl(p + UART_RESET_REG);
    if (priv.dev_rev >= 0xC0)
    data |= UART_RESET_HOT_RESET_DISABLE;
    writel(data | UART_RESET_D3_RESET_DISABLE, p + UART_RESET_REG);
    if (wakeup)
    writeb(UART_PCI_CTRL_D3_CLK_ENABLE, p + UART_PCI_CTRL_REG);
    iounmap(p);
    device_set_wakeup_enable(dev, true);
    pci_wake_from_d3(pcidev, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_resume(dev: *mut device) -> c_int {
    static int pci1xxxx_resume(struct device *dev)
    {
    struct pci1xxxx_8250 *priv = dev_get_drvdata(dev);
    struct pci_dev *pcidev = to_pci_dev(dev);
    unsigned int data;
    void __iomem *p;
    int i;
    p = pci_ioremap_bar(pcidev, 0);
    if (!p) {
    dev_err(dev, "remapping of bar 0 memory failed");
    return -ENOMEM;
    }
    data = readl(p + UART_RESET_REG);
    if (priv.dev_rev >= 0xC0)
    data &= ~UART_RESET_HOT_RESET_DISABLE;
    writel(data & ~UART_RESET_D3_RESET_DISABLE, p + UART_RESET_REG);
    iounmap(p);
    for (i = 0; i < priv.nr; i++) {
    if (priv.line[i] >= 0) {
    pci1xxxx_port_resume(priv.line[i]);
    serial8250_resume_port(priv.line[i]);
    }
    }
    return 0;
    }
    static int pci1xxxx_setup(struct pci_dev *pdev,
    struct uart_8250_port *port, int port_idx, struct pci1xxxx_8250 *priv)
    {
    int ret;
    port.port.flags |= UPF_FIXED_TYPE | UPF_SKIP_TEST;
    port.port.type = PORT_MCHP16550A;
//
// 8250 core considers prescaller value to be always 16.
// The MCHP ports support downscaled mode and hence the
// functional UART clock can be lower, i.e. 62.5MHz, than
// software expects in order to support higher baud rates.
// Assign here 64MHz to support 4Mbps.
//
// The value itself is not really used anywhere except baud
// rate calculations, so we can mangle it as we wish.
//
    port.port.uartclk = 64 * HZ_PER_MHZ;
    port.port.set_termios = serial8250_do_set_termios;
    port.port.get_divisor = pci1xxxx_get_divisor;
    port.port.set_divisor = pci1xxxx_set_divisor;
    port.port.rs485_config = pci1xxxx_rs485_config;
    port.port.rs485_supported = pci1xxxx_rs485_supported;
//
// C0 and later revisions support Burst operation.
// RTS workaround in mctrl is applicable only to B0.
//
    if (priv.dev_rev >= 0xC0)
    port.port.handle_irq = pci1xxxx_handle_irq;
#[no_mangle]
pub unsafe extern "C" fn if(0xB0: priv->dev_rev ==) -> else {
    else if (priv.dev_rev == 0xB0)
    port.port.set_mctrl = pci1xxxx_set_mctrl;
    ret = serial8250_pci_setup_port(pdev, port, 0, PORT_OFFSET * port_idx, 0, priv.membase);
    if (ret < 0)
    return ret;
    writeb(UART_BLOCK_SET_ACTIVE, port.port.membase + UART_ACTV_REG);
    writeb(UART_WAKE_SRCS, port.port.membase + UART_WAKE_REG);
    writeb(UART_WAKE_N_PIN, port.port.membase + UART_WAKE_MASK_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_get_max_port(subsys_dev: c_int) -> c_uint {
    static unsigned int pci1xxxx_get_max_port(int subsys_dev)
    {
    let mut i: c_uint = MAX_PORTS;
    if (subsys_dev < ARRAY_SIZE(logical_to_physical_port_idx))
    while (i--) {
    if (logical_to_physical_port_idx[subsys_dev][i] != -1)
    return logical_to_physical_port_idx[subsys_dev][i] + 1;
    }
    if (subsys_dev == PCI_SUBDEVICE_ID_EFAR_PCI11414)
    return 4;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_logical_to_physical_port_translate(subsys_dev: c_int, port: c_int) -> c_int {
    static int pci1xxxx_logical_to_physical_port_translate(int subsys_dev, int port)
    {
    if (subsys_dev < ARRAY_SIZE(logical_to_physical_port_idx))
    return logical_to_physical_port_idx[subsys_dev][port];
    return logical_to_physical_port_idx[0][port];
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_get_device_revision(priv: *mut pci1xxxx_8250) -> c_int {
    static int pci1xxxx_get_device_revision(struct pci1xxxx_8250 *priv)
    {
    u32 regval;
    int ret;
//
// DEV REV is a system register, HW Syslock bit
// should be acquired before accessing the register
//
    ret = pci1xxxx_acquire_sys_lock(priv);
    if (ret)
    return ret;
    regval = readl(priv.membase + UART_DEV_REV_REG);
    priv.dev_rev = regval & UART_DEV_REV_MASK;
    pci1xxxx_release_sys_lock(priv);
    return 0;
    }
    static int pci1xxxx_serial_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct device *dev = &pdev.dev;
    struct pci1xxxx_8250 *priv;
    struct uart_8250_port uart;
    unsigned int max_vec_reqd;
    unsigned int nr_ports, i;
    int num_vectors;
    int subsys_dev;
    int port_idx;
    int ret;
    int rc;
    rc = pcim_enable_device(pdev);
    if (rc)
    return rc;
    nr_ports = pci1xxxx_get_num_ports(pdev);
    priv = devm_kzalloc(dev, struct_size(priv, line, nr_ports), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.membase = pci_ioremap_bar(pdev, 0);
    if (!priv.membase)
    return -ENOMEM;
    ret = pci1xxxx_get_device_revision(priv);
    if (ret)
    return ret;
    pci_set_master(pdev);
    priv.nr = nr_ports;
    subsys_dev = pdev.subsystem_device;
    max_vec_reqd = pci1xxxx_get_max_port(subsys_dev);
    num_vectors = pci_alloc_irq_vectors(pdev, 1, max_vec_reqd, PCI_IRQ_ALL_TYPES);
    if (num_vectors < 0) {
    pci_iounmap(pdev, priv.membase);
    return num_vectors;
    }
    memset(&uart, 0, sizeof(uart));
    uart.port.flags = UPF_SHARE_IRQ | UPF_FIXED_PORT;
    uart.port.dev = dev;
    if (num_vectors == max_vec_reqd)
    writeb(UART_PCI_CTRL_SET_MULTIPLE_MSI, priv.membase + UART_PCI_CTRL_REG);
    for (i = 0; i < nr_ports; i++) {
    priv.line[i] = -ENODEV;
    port_idx = pci1xxxx_logical_to_physical_port_translate(subsys_dev, i);
    if (num_vectors == max_vec_reqd)
    uart.port.irq = pci_irq_vector(pdev, port_idx);
    else
    uart.port.irq = pci_irq_vector(pdev, 0);
    rc = pci1xxxx_setup(pdev, &uart, port_idx, priv);
    if (rc) {
    dev_warn(dev, "Failed to setup port %u\n", i);
    continue;
    }
    priv.line[i] = serial8250_register_8250_port(&uart);
    if (priv.line[i] < 0) {
    dev_warn(dev,
    "Couldn't register serial port %lx, irq %d, type %d, error %d\n",
    uart.port.iobase, uart.port.irq, uart.port.iotype,
    priv.line[i]);
    }
    }
    pci_set_drvdata(pdev, priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_serial_remove(dev: *mut pci_dev) {
    static void pci1xxxx_serial_remove(struct pci_dev *dev)
    {
    struct pci1xxxx_8250 *priv = pci_get_drvdata(dev);
    unsigned int i;
    for (i = 0; i < priv.nr; i++) {
    if (priv.line[i] >= 0)
    serial8250_unregister_port(priv.line[i]);
    }
    pci_free_irq_vectors(dev);
    pci_iounmap(dev, priv.membase);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(pci1xxxx_pm_ops, pci1xxxx_suspend, pci1xxxx_resume);
    static const struct pci_device_id pci1xxxx_pci_tbl[] = {
    { PCI_VDEVICE(EFAR, PCI_DEVICE_ID_EFAR_PCI11010) },
    { PCI_VDEVICE(EFAR, PCI_DEVICE_ID_EFAR_PCI11101) },
    { PCI_VDEVICE(EFAR, PCI_DEVICE_ID_EFAR_PCI11400) },
    { PCI_VDEVICE(EFAR, PCI_DEVICE_ID_EFAR_PCI11414) },
    { PCI_VDEVICE(EFAR, PCI_DEVICE_ID_EFAR_PCI12000) },
    {}
    };
    MODULE_DEVICE_TABLE(pci, pci1xxxx_pci_tbl);
    static struct pci_driver pci1xxxx_pci_driver = {
    .name = "pci1xxxx serial",
    .probe = pci1xxxx_serial_probe,
    .remove = pci1xxxx_serial_remove,
    .driver = {
    .pm     = pm_sleep_ptr(&pci1xxxx_pm_ops),
    },
    .id_table = pci1xxxx_pci_tbl,
    };
    module_pci_driver(pci1xxxx_pci_driver);
    static_assert((ARRAY_SIZE(logical_to_physical_port_idx) == PCI_SUBDEVICE_ID_EFAR_PCI1XXXX_1p3 + 1));
    MODULE_IMPORT_NS("SERIAL_8250_PCI");
    MODULE_DESCRIPTION("Microchip Technology Inc. PCIe to UART module");
    MODULE_AUTHOR("Kumaravel Thiagarajan <kumaravel.thiagarajan@microchip.com>");
    MODULE_AUTHOR("Tharun Kumar P <tharunkumar.pasumarthi@microchip.com>");
    MODULE_LICENSE("GPL");
