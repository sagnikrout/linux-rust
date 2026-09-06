//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/imx.c
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
// Driver for Motorola/Freescale IMX serial ports
//
// Based on drivers/char/serial.c, by Linus Torvalds, Theodore Ts'o.
//
// Author: Sascha Hauer <sascha@saschahauer.de>
// Copyright (C) 2004 Pengutronix
//

// Register definitions
pub const URXD0: c_uint = 0x0  /* Receiver Register */;
pub const URTX0: c_uint = 0x40 /* Transmitter Register */;
pub const UCR1: c_uint = 0x80 /* Control Register 1 */;
pub const UCR2: c_uint = 0x84 /* Control Register 2 */;
pub const UCR3: c_uint = 0x88 /* Control Register 3 */;
pub const UCR4: c_uint = 0x8c /* Control Register 4 */;
pub const UFCR: c_uint = 0x90 /* FIFO Control Register */;
pub const USR1: c_uint = 0x94 /* Status Register 1 */;
pub const USR2: c_uint = 0x98 /* Status Register 2 */;
pub const UESC: c_uint = 0x9c /* Escape Character Register */;
pub const UTIM: c_uint = 0xa0 /* Escape Timer Register */;
pub const UBIR: c_uint = 0xa4 /* BRM Incremental Register */;
pub const UBMR: c_uint = 0xa8 /* BRM Modulator Register */;
pub const UBRC: c_uint = 0xac /* Baud Rate Count Register */;
pub const IMX21_ONEMS: c_uint = 0xb0 /* One Millisecond register */;
pub const IMX1_UTS: c_uint = 0xd0 /* UART Test Register on i.mx1 */;
pub const IMX21_UTS: c_uint = 0xb4 /* UART Test Register on all other i.mx*/;
// UART Control Register Bit Fields.

pub const UCR4_CTSTL_MASK: c_uint = 0x3F	/* CTS trigger is 6 bits wide */;

pub const UFCR_RXTL_MASK: c_uint = 0x3F	/* Receiver trigger 6 bits wide */;

// We've been assigned a range on the "Low-density serial ports" major
pub const SERIAL_IMX_MAJOR: c_int = 207;
pub const MINOR_START: c_int = 16;

//
// This determines how often we check the modem status signals
// for any change.  They generally aren't connected to an IRQ
// so we have to poll them.  We also check immediately before
// filling the TX fifo incase CTS has been dropped.
//

pub const UART_NR: c_int = 8;
// i.MX21 type uart runs on all i.mx except i.MX1 and i.MX6q
    enum imx_uart_type {
    IMX1_UART,
    IMX21_UART,
    };
// device type dependent stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_uart_data {
    pub uts_reg: unsigned,
    pub devtype: enum imx_uart_type,
}

    enum imx_tx_state {
    OFF,
    WAIT_AFTER_RTS,
    SEND,
    WAIT_AFTER_SEND,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_port {
    pub port: uart_port,
    pub timer: timer_list,
    pub old_status: c_uint,
    pub have_rtscts:1: c_uint,
    pub have_rtsgpio:1: c_uint,
    pub dte_mode:1: c_uint,
    pub inverted_tx:1: c_uint,
    pub inverted_rx:1: c_uint,
    pub clk_ipg: *mut clk,
    pub clk_per: *mut clk,
    pub devdata: *const imx_uart_data,
    pub gpios: *mut mctrl_gpios,
// counter to stop 0xff flood
    pub idle_counter: c_int,
// DMA fields
    pub dma_is_enabled:1: c_uint,
    pub dma_is_rxing:1: c_uint,
    pub dma_is_txing:1: c_uint,
    pub dma_chan_tx: *mut *mut dma_chan dma_chan_rx,,
    pub tx_sgl: [scatterlist rx_sgl,; 2],
    pub rx_buf: *mut c_void,
    pub rx_ring: circ_buf,
    pub rx_buf_size: c_uint,
    pub rx_period_length: c_uint,
    pub rx_periods: c_uint,
    pub rx_cookie: dma_cookie_t,
    pub tx_bytes: c_uint,
    pub dma_tx_nents: c_uint,
    pub saved_reg: [c_uint; 10],
    pub context_saved: bool,
    pub last_putchar_was_newline: bool,
    pub tx_state: enum imx_tx_state,
    pub trigger_start_tx: hrtimer,
    pub trigger_stop_tx: hrtimer,
    pub rxtl: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_port_ucrs {
    pub ucr1: c_uint,
    pub ucr2: c_uint,
    pub ucr3: c_uint,
}

    static const struct imx_uart_data imx_uart_imx1_devdata = {
    .uts_reg = IMX1_UTS,
    .devtype = IMX1_UART,
    };
    static const struct imx_uart_data imx_uart_imx21_devdata = {
    .uts_reg = IMX21_UTS,
    .devtype = IMX21_UART,
    };
    static const struct of_device_id imx_uart_dt_ids[] = {
//
// For reasons unknown to me, some UART devices (e.g. imx6ul's) are
// compatible to fsl,imx6q-uart, but not fsl,imx21-uart, while the
// original imx6q's UART is compatible to fsl,imx21-uart. This driver
// doesn't make any distinction between these two variants.
//
    { .compatible = "fsl,imx6q-uart", .data = &imx_uart_imx21_devdata, },
    { .compatible = "fsl,imx1-uart", .data = &imx_uart_imx1_devdata, },
    { .compatible = "fsl,imx21-uart", .data = &imx_uart_imx21_devdata, },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, imx_uart_dt_ids);
    static inline struct imx_port *to_imx_port(struct uart_port *port)
    {
    return container_of(port, struct imx_port, port);
    }
#[no_mangle]
pub unsafe extern "C" fn imx_uart_writel(sport: *mut imx_port, val: u32, offset: u32) {
    static inline void imx_uart_writel(struct imx_port *sport, u32 val, u32 offset)
    {
    writel(val, sport.port.membase + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn imx_uart_readl(sport: *mut imx_port, offset: u32) -> u32 {
    static inline u32 imx_uart_readl(struct imx_port *sport, u32 offset)
    {
    return readl(sport.port.membase + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn imx_uart_uts_reg(sport: *mut imx_port) -> unsigned {
    static inline unsigned imx_uart_uts_reg(struct imx_port *sport)
    {
    return sport.devdata.uts_reg;
    }
#[no_mangle]
pub unsafe extern "C" fn imx_uart_is_imx1(sport: *mut imx_port) -> c_int {
    static inline int imx_uart_is_imx1(struct imx_port *sport)
    {
    return sport.devdata.devtype == IMX1_UART;
    }
//
// Save and restore functions for UCR1, UCR2 and UCR3 registers
//

    static void imx_uart_ucrs_save(struct imx_port *sport,
    struct imx_port_ucrs *ucr)
    {
// save control registers
    ucr.ucr1 = imx_uart_readl(sport, UCR1);
    ucr.ucr2 = imx_uart_readl(sport, UCR2);
    ucr.ucr3 = imx_uart_readl(sport, UCR3);
    }
    static void imx_uart_ucrs_restore(struct imx_port *sport,
    struct imx_port_ucrs *ucr)
    {
// restore control registers
    imx_uart_writel(sport, ucr.ucr1, UCR1);
    imx_uart_writel(sport, ucr.ucr2, UCR2);
    imx_uart_writel(sport, ucr.ucr3, UCR3);
    }

// called with port.lock taken and irqs caller dependent
#[no_mangle]
unsafe extern "C" fn imx_uart_rts_active(sport: *mut imx_port, ucr2: *mut u32) {
    static void imx_uart_rts_active(struct imx_port *sport, u32 *ucr2)
    {
// ucr2 &= ~(UCR2_CTSC | UCR2_CTS);
    mctrl_gpio_set(sport.gpios, sport.port.mctrl | TIOCM_RTS);
    }
// called with port.lock taken and irqs caller dependent
#[no_mangle]
unsafe extern "C" fn imx_uart_rts_inactive(sport: *mut imx_port, ucr2: *mut u32) {
    static void imx_uart_rts_inactive(struct imx_port *sport, u32 *ucr2)
    {
// ucr2 &= ~UCR2_CTSC;
// ucr2 |= UCR2_CTS;
    mctrl_gpio_set(sport.gpios, sport.port.mctrl & ~TIOCM_RTS);
    }
#[no_mangle]
unsafe extern "C" fn start_hrtimer_ms(hrt: *mut hrtimer, msec: c_ulong) {
    static void start_hrtimer_ms(struct hrtimer *hrt, unsigned long msec)
    {
    hrtimer_start(hrt, ms_to_ktime(msec), HRTIMER_MODE_REL);
    }
// called with port.lock taken and irqs off
#[no_mangle]
unsafe extern "C" fn imx_uart_soft_reset(sport: *mut imx_port) {
    static void imx_uart_soft_reset(struct imx_port *sport)
    {
    let mut i: c_int = 10;
    u32 ucr2, ubir, ubmr, uts;
//
// According to the Reference Manual description of the UART SRST bit:
//
// "Reset the transmit and receive state machines,
// all FIFOs and register USR1, USR2, UBIR, UBMR, UBRC, URXD, UTXD
// and UTS[6-3]".
//
// We don't need to restore the old values from USR1, USR2, URXD and
// UTXD. UBRC is read only, so only save/restore the other three
// registers.
//
    ubir = imx_uart_readl(sport, UBIR);
    ubmr = imx_uart_readl(sport, UBMR);
    uts = imx_uart_readl(sport, IMX21_UTS);
    ucr2 = imx_uart_readl(sport, UCR2);
    imx_uart_writel(sport, ucr2 & ~UCR2_SRST, UCR2);
    while (!(imx_uart_readl(sport, UCR2) & UCR2_SRST) && (--i > 0))
    udelay(1);
// Restore the registers
    imx_uart_writel(sport, ubir, UBIR);
    imx_uart_writel(sport, ubmr, UBMR);
    imx_uart_writel(sport, uts, IMX21_UTS);
    sport.idle_counter = 0;
    }
// called with port.lock taken and irqs off
#[no_mangle]
unsafe extern "C" fn imx_uart_disable_loopback_rs485(sport: *mut imx_port) {
    static void imx_uart_disable_loopback_rs485(struct imx_port *sport)
    {
    unsigned int uts;
// See SER_RS485_ENABLED/UTS_LOOP comment in imx_uart_probe()
    uts = imx_uart_readl(sport, imx_uart_uts_reg(sport));
    uts &= ~UTS_LOOP;
    imx_uart_writel(sport, uts, imx_uart_uts_reg(sport));
    }
// called with port.lock taken and irqs off
#[no_mangle]
unsafe extern "C" fn imx_uart_start_rx(port: *mut uart_port) {
    static void imx_uart_start_rx(struct uart_port *port)
    {
    struct imx_port *sport = to_imx_port(port);
    unsigned int ucr1, ucr2;
    ucr1 = imx_uart_readl(sport, UCR1);
    ucr2 = imx_uart_readl(sport, UCR2);
    ucr2 |= UCR2_RXEN;
    if (sport.dma_is_enabled) {
    ucr1 |= UCR1_RXDMAEN | UCR1_ATDMAEN;
    } else {
    ucr1 |= UCR1_RRDYEN;
    ucr2 |= UCR2_ATEN;
    }
// Write UCR2 first as it includes RXEN
    imx_uart_writel(sport, ucr2, UCR2);
    imx_uart_writel(sport, ucr1, UCR1);
    imx_uart_disable_loopback_rs485(sport);
    }
// called with port.lock taken and irqs off
#[no_mangle]
unsafe extern "C" fn imx_uart_stop_tx(port: *mut uart_port) {
    static void imx_uart_stop_tx(struct uart_port *port)
    {
    struct imx_port *sport = to_imx_port(port);
    u32 ucr1, ucr4, usr2;
    if (sport.tx_state == OFF)
    return;
//
// We are maybe in the SMP context, so if the DMA TX thread is running
// on other cpu, we have to wait for it to finish.
//
    if (sport.dma_is_txing)
    return;
    ucr1 = imx_uart_readl(sport, UCR1);
    imx_uart_writel(sport, ucr1 & ~UCR1_TRDYEN, UCR1);
    ucr4 = imx_uart_readl(sport, UCR4);
    usr2 = imx_uart_readl(sport, USR2);
    if ((!(usr2 & USR2_TXDC)) && (ucr4 & UCR4_TCEN)) {
// The shifter is still busy, so retry once TC triggers
    return;
    }
    ucr4 &= ~UCR4_TCEN;
    imx_uart_writel(sport, ucr4, UCR4);
// in rs485 mode disable transmitter
    if (port.rs485.flags & SER_RS485_ENABLED) {
    if (sport.tx_state == SEND) {
    sport.tx_state = WAIT_AFTER_SEND;
    if (port.rs485.delay_rts_after_send > 0) {
    start_hrtimer_ms(&sport.trigger_stop_tx,
    port.rs485.delay_rts_after_send);
    return;
    }
// continue without any delay
    }
    if (sport.tx_state == WAIT_AFTER_RTS ||
    sport.tx_state == WAIT_AFTER_SEND) {
    u32 ucr2;
    hrtimer_try_to_cancel(&sport.trigger_start_tx);
    ucr2 = imx_uart_readl(sport, UCR2);
    if (port.rs485.flags & SER_RS485_RTS_AFTER_SEND)
    imx_uart_rts_active(sport, &ucr2);
    else
    imx_uart_rts_inactive(sport, &ucr2);
    imx_uart_writel(sport, ucr2, UCR2);
    if (!port.rs485_rx_during_tx_gpio)
    imx_uart_start_rx(port);
    sport.tx_state = OFF;
    }
    } else {
    sport.tx_state = OFF;
    }
    }
// called with port.lock taken and irqs off
#[no_mangle]
unsafe extern "C" fn imx_uart_stop_rx_with_loopback_ctrl(port: *mut uart_port, loopback: bool) {
    static void imx_uart_stop_rx_with_loopback_ctrl(struct uart_port *port, bool loopback)
    {
    struct imx_port *sport = to_imx_port(port);
    u32 ucr1, ucr2, ucr4, uts;
    ucr1 = imx_uart_readl(sport, UCR1);
    ucr2 = imx_uart_readl(sport, UCR2);
    ucr4 = imx_uart_readl(sport, UCR4);
    if (sport.dma_is_enabled) {
    ucr1 &= ~(UCR1_RXDMAEN | UCR1_ATDMAEN);
    } else {
    ucr1 &= ~UCR1_RRDYEN;
    ucr2 &= ~UCR2_ATEN;
    ucr4 &= ~UCR4_OREN;
    }
    imx_uart_writel(sport, ucr1, UCR1);
    imx_uart_writel(sport, ucr4, UCR4);
// See SER_RS485_ENABLED/UTS_LOOP comment in imx_uart_probe()
    if (port.rs485.flags & SER_RS485_ENABLED &&
    port.rs485.flags & SER_RS485_RTS_ON_SEND &&
    sport.have_rtscts && !sport.have_rtsgpio && loopback) {
    uts = imx_uart_readl(sport, imx_uart_uts_reg(sport));
    uts |= UTS_LOOP;
    imx_uart_writel(sport, uts, imx_uart_uts_reg(sport));
    ucr2 |= UCR2_RXEN;
    } else {
    ucr2 &= ~UCR2_RXEN;
    }
    imx_uart_writel(sport, ucr2, UCR2);
    }
// called with port.lock taken and irqs off
#[no_mangle]
unsafe extern "C" fn imx_uart_stop_rx(port: *mut uart_port) {
    static void imx_uart_stop_rx(struct uart_port *port)
    {
//
// Stop RX and enable loopback in order to make sure RS485 bus
// is not blocked. Se comment in imx_uart_probe().
//
    imx_uart_stop_rx_with_loopback_ctrl(port, true);
    }
// called with port.lock taken and irqs off
#[no_mangle]
unsafe extern "C" fn imx_uart_enable_ms(port: *mut uart_port) {
    static void imx_uart_enable_ms(struct uart_port *port)
    {
    struct imx_port *sport = to_imx_port(port);
    mod_timer(&sport.timer, jiffies);
    mctrl_gpio_enable_ms(sport.gpios);
    }
    static void imx_uart_dma_tx(struct imx_port *sport);
// called with port.lock taken and irqs off
#[no_mangle]
pub unsafe extern "C" fn imx_uart_transmit_buffer(sport: *mut imx_port) {
    static inline void imx_uart_transmit_buffer(struct imx_port *sport)
    {
    struct tty_port *tport = &sport.port.state.port;
    unsigned char c;
    if (sport.port.x_char) {
// Send next char
    imx_uart_writel(sport, sport.port.x_char, URTX0);
    sport.port.icount.tx++;
    sport.port.x_char = 0;
    return;
    }
    if (kfifo_is_empty(&tport.xmit_fifo) ||
    uart_tx_stopped(&sport.port)) {
    imx_uart_stop_tx(&sport.port);
    return;
    }
    if (sport.dma_is_enabled) {
    u32 ucr1;
//
// We've just sent a X-char Ensure the TX DMA is enabled
// and the TX IRQ is disabled.
//
    ucr1 = imx_uart_readl(sport, UCR1);
    ucr1 &= ~UCR1_TRDYEN;
    if (sport.dma_is_txing) {
    ucr1 |= UCR1_TXDMAEN;
    imx_uart_writel(sport, ucr1, UCR1);
    } else {
    imx_uart_writel(sport, ucr1, UCR1);
    imx_uart_dma_tx(sport);
    }
    return;
    }
    while (!(imx_uart_readl(sport, imx_uart_uts_reg(sport)) & UTS_TXFULL) &&
    uart_fifo_get(&sport.port, &c))
    imx_uart_writel(sport, c, URTX0);
    if (kfifo_len(&tport.xmit_fifo) < WAKEUP_CHARS)
    uart_write_wakeup(&sport.port);
    if (kfifo_is_empty(&tport.xmit_fifo))
    imx_uart_stop_tx(&sport.port);
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_dma_tx_callback(data: *mut c_void) {
    static void imx_uart_dma_tx_callback(void *data)
    {
    struct imx_port *sport = data;
    struct tty_port *tport = &sport.port.state.port;
    struct scatterlist *sgl = &sport.tx_sgl[0];
    unsigned long flags;
    u32 ucr1;
    uart_port_lock_irqsave(&sport.port, &flags);
    dma_unmap_sg(sport.port.dev, sgl, sport.dma_tx_nents, DMA_TO_DEVICE);
    ucr1 = imx_uart_readl(sport, UCR1);
    ucr1 &= ~UCR1_TXDMAEN;
    imx_uart_writel(sport, ucr1, UCR1);
    uart_xmit_advance(&sport.port, sport.tx_bytes);
    dev_dbg(sport.port.dev, "we finish the TX DMA.\n");
    sport.dma_is_txing = 0;
    if (kfifo_len(&tport.xmit_fifo) < WAKEUP_CHARS)
    uart_write_wakeup(&sport.port);
    if (!kfifo_is_empty(&tport.xmit_fifo) &&
    !uart_tx_stopped(&sport.port))
    imx_uart_dma_tx(sport);
#[no_mangle]
pub unsafe extern "C" fn if(SER_RS485_ENABLED: sport->port.rs485.flags &) -> else {
    let mut ucr4: u32 = imx_uart_readl(sport, UCR4);
    ucr4 |= UCR4_TCEN;
    imx_uart_writel(sport, ucr4, UCR4);
    }
    uart_port_unlock_irqrestore(&sport.port, flags);
    }
// called with port.lock taken and irqs off
#[no_mangle]
unsafe extern "C" fn imx_uart_dma_tx(sport: *mut imx_port) {
    static void imx_uart_dma_tx(struct imx_port *sport)
    {
    struct tty_port *tport = &sport.port.state.port;
    struct scatterlist *sgl = sport.tx_sgl;
    struct dma_async_tx_descriptor *desc;
    struct dma_chan	*chan = sport.dma_chan_tx;
    struct device *dev = sport.port.dev;
    u32 ucr1, ucr4;
    int ret;
    if (sport.dma_is_txing)
    return;
    ucr4 = imx_uart_readl(sport, UCR4);
    ucr4 &= ~UCR4_TCEN;
    imx_uart_writel(sport, ucr4, UCR4);
    sg_init_table(sgl, ARRAY_SIZE(sport.tx_sgl));
    sport.tx_bytes = kfifo_len(&tport.xmit_fifo);
    sport.dma_tx_nents = kfifo_dma_out_prepare(&tport.xmit_fifo, sgl,
    ARRAY_SIZE(sport.tx_sgl), sport.tx_bytes);
    ret = dma_map_sg(dev, sgl, sport.dma_tx_nents, DMA_TO_DEVICE);
    if (ret == 0) {
    dev_err(dev, "DMA mapping error for TX.\n");
    return;
    }
    desc = dmaengine_prep_slave_sg(chan, sgl, ret,
    DMA_MEM_TO_DEV, DMA_PREP_INTERRUPT);
    if (!desc) {
    dma_unmap_sg(dev, sgl, sport.dma_tx_nents,
    DMA_TO_DEVICE);
    dev_err(dev, "We cannot prepare for the TX slave dma!\n");
    return;
    }
    desc.callback = imx_uart_dma_tx_callback;
    desc.callback_param = sport;
    dev_dbg(dev, "TX: prepare to send %u bytes by DMA.\n", sport.tx_bytes);
    ucr1 = imx_uart_readl(sport, UCR1);
    ucr1 |= UCR1_TXDMAEN;
    imx_uart_writel(sport, ucr1, UCR1);
// fire it
    sport.dma_is_txing = 1;
    dmaengine_submit(desc);
    dma_async_issue_pending(chan);
    return;
    }
// called with port.lock taken and irqs off
#[no_mangle]
unsafe extern "C" fn imx_uart_start_tx(port: *mut uart_port) {
    static void imx_uart_start_tx(struct uart_port *port)
    {
    struct imx_port *sport = to_imx_port(port);
    struct tty_port *tport = &sport.port.state.port;
    u32 ucr1;
    if (!sport.port.x_char && kfifo_is_empty(&tport.xmit_fifo))
    return;
//
// We cannot simply do nothing here if sport->tx_state == SEND already
// because UCR1_TXMPTYEN might already have been cleared in
// imx_uart_stop_tx(), but tx_state is still SEND.
//
    if (port.rs485.flags & SER_RS485_ENABLED) {
    if (sport.tx_state == OFF) {
    let mut ucr2: u32 = imx_uart_readl(sport, UCR2);
    if (port.rs485.flags & SER_RS485_RTS_ON_SEND)
    imx_uart_rts_active(sport, &ucr2);
    else
    imx_uart_rts_inactive(sport, &ucr2);
    imx_uart_writel(sport, ucr2, UCR2);
//
// Since we are about to transmit we can not stop RX
// with loopback enabled because that will make our
// transmitted data being just looped to RX.
//
    if (!(port.rs485.flags & SER_RS485_RX_DURING_TX) &&
    !port.rs485_rx_during_tx_gpio)
    imx_uart_stop_rx_with_loopback_ctrl(port, false);
    sport.tx_state = WAIT_AFTER_RTS;
    if (port.rs485.delay_rts_before_send > 0) {
    start_hrtimer_ms(&sport.trigger_start_tx,
    port.rs485.delay_rts_before_send);
    return;
    }
// continue without any delay
    }
    if (sport.tx_state == WAIT_AFTER_SEND
    || sport.tx_state == WAIT_AFTER_RTS) {
    hrtimer_try_to_cancel(&sport.trigger_stop_tx);
//
// Enable transmitter and shifter empty irq only if DMA
// is off.  In the DMA case this is done in the
// tx-callback.
//
    if (!sport.dma_is_enabled) {
    let mut ucr4: u32 = imx_uart_readl(sport, UCR4);
    ucr4 |= UCR4_TCEN;
    imx_uart_writel(sport, ucr4, UCR4);
    }
    sport.tx_state = SEND;
    }
    } else {
    sport.tx_state = SEND;
    }
    if (!sport.dma_is_enabled) {
    ucr1 = imx_uart_readl(sport, UCR1);
    imx_uart_writel(sport, ucr1 | UCR1_TRDYEN, UCR1);
    }
    if (sport.dma_is_enabled) {
    if (sport.port.x_char) {
// We have X-char to send, so enable TX IRQ and
// disable TX DMA to let TX interrupt to send X-char
    ucr1 = imx_uart_readl(sport, UCR1);
    ucr1 &= ~UCR1_TXDMAEN;
    ucr1 |= UCR1_TRDYEN;
    imx_uart_writel(sport, ucr1, UCR1);
    return;
    }
    if (!kfifo_is_empty(&tport.xmit_fifo) &&
    !uart_tx_stopped(port))
    imx_uart_dma_tx(sport);
    return;
    }
    }
#[no_mangle]
unsafe extern "C" fn __imx_uart_rtsint(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t __imx_uart_rtsint(int irq, void *dev_id)
    {
    struct imx_port *sport = dev_id;
    u32 usr1;
    imx_uart_writel(sport, USR1_RTSD, USR1);
    usr1 = imx_uart_readl(sport, USR1) & USR1_RTSS;
//
// Update sport->old_status here, so any follow-up calls to
// imx_uart_mctrl_check() will be able to recognize that RTS
// state changed since last imx_uart_mctrl_check() call.
//
// In case RTS has been detected as asserted here and later on
// deasserted by the time imx_uart_mctrl_check() was called,
// imx_uart_mctrl_check() can detect the RTS state change and
// trigger uart_handle_cts_change() to unblock the port for
// further TX transfers.
//
    if (usr1 & USR1_RTSS)
    sport.old_status |= TIOCM_CTS;
    else
    sport.old_status &= ~TIOCM_CTS;
    uart_handle_cts_change(&sport.port, usr1);
    wake_up_interruptible(&sport.port.state.port.delta_msr_wait);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_rtsint(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t imx_uart_rtsint(int irq, void *dev_id)
    {
    struct imx_port *sport = dev_id;
    irqreturn_t ret;
    uart_port_lock(&sport.port);
    ret = __imx_uart_rtsint(irq, dev_id);
    uart_port_unlock(&sport.port);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_txint(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t imx_uart_txint(int irq, void *dev_id)
    {
    struct imx_port *sport = dev_id;
    uart_port_lock(&sport.port);
    imx_uart_transmit_buffer(sport);
    uart_port_unlock(&sport.port);
    return IRQ_HANDLED;
    }
// Check if hardware Rx flood is in progress, and issue soft reset to stop it.
// This is to be called from Rx ISRs only when some bytes were actually
// received.
//
// A way to reproduce the flood (checked on iMX6SX) is: open iMX UART at 9600
// 8N1, and from external source send 0xf0 char at 115200 8N1. In about 90% of
// cases this starts a flood of "receiving" of 0xff characters by the iMX6 UART
// that is terminated by any activity on RxD line, or could be stopped by
// issuing soft reset to the UART (just stop/start of RX does not help). Note
// that what we do here is sending isolated start bit about 2.4 times shorter
// than it is to be on UART configured baud rate.
//
// Called with port.lock taken and irqs off.
//
#[no_mangle]
unsafe extern "C" fn imx_uart_check_flood(sport: *mut imx_port, usr2: u32) {
    static void imx_uart_check_flood(struct imx_port *sport, u32 usr2)
    {
// To detect hardware 0xff flood we monitor RxD line between RX
// interrupts to isolate "receiving" of char(s) with no activity
// on RxD line, that'd never happen on actual data transfers.
//
// We use USR2_WAKE bit to check for activity on RxD line, but we have a
// race here if we clear USR2_WAKE when receiving of a char is in
// progress, so we might get RX interrupt later with USR2_WAKE bit
// cleared. Note though that as we don't try to clear USR2_WAKE when we
// detected no activity, this race may hide actual activity only once.
//
// Yet another case where receive interrupt may occur without RxD
// activity is expiration of aging timer, so we consider this as well.
//
// We use 'idle_counter' to ensure that we got at least so many RX
// interrupts without any detected activity on RxD line. 2 cases
// described plus 1 to be on the safe side gives us a margin of 3,
// below. In practice I was not able to produce a false positive to
// induce soft reset at regular data transfers even using 1 as the
// margin, so 3 is actually very strong.
//
// We count interrupts, not chars in 'idle-counter' for simplicity.
//
    if (usr2 & USR2_WAKE) {
    imx_uart_writel(sport, USR2_WAKE, USR2);
    sport.idle_counter = 0;
    } else if (++sport.idle_counter > 3) {
    dev_warn(sport.port.dev, "RX flood detected: soft reset.");
    imx_uart_soft_reset(sport); /* also clears 'sport.idle_counter' */
    }
    }
// called with port.lock taken and irqs off
#[no_mangle]
unsafe extern "C" fn __imx_uart_rxint(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t __imx_uart_rxint(int irq, void *dev_id)
    {
    struct imx_port *sport = dev_id;
    struct tty_port *port = &sport.port.state.port;
    u32 usr2, rx;
// If we received something, check for 0xff flood
    usr2 = imx_uart_readl(sport, USR2);
    if (usr2 & USR2_RDR)
    imx_uart_check_flood(sport, usr2);
    while ((rx = imx_uart_readl(sport, URXD0)) & URXD_CHARRDY) {
    let mut flg: c_uint = TTY_NORMAL;
    sport.port.icount.rx++;
    if (unlikely(rx & URXD_ERR)) {
    if (rx & URXD_BRK) {
    sport.port.icount.brk++;
    if (uart_handle_break(&sport.port))
    continue;
    }
#[no_mangle]
pub unsafe extern "C" fn if(URXD_PRERR: rx &) -> else {
    else if (rx & URXD_PRERR)
    sport.port.icount.parity++;
#[no_mangle]
pub unsafe extern "C" fn if(URXD_FRMERR: rx &) -> else {
    else if (rx & URXD_FRMERR)
    sport.port.icount.frame++;
    if (rx & URXD_OVRRUN)
    sport.port.icount.overrun++;
    if (rx & sport.port.ignore_status_mask)
    continue;
    rx &= (sport.port.read_status_mask | 0xFF);
    if (rx & URXD_BRK)
    flg = TTY_BREAK;
#[no_mangle]
pub unsafe extern "C" fn if(URXD_PRERR: rx &) -> else {
    else if (rx & URXD_PRERR)
    flg = TTY_PARITY;
#[no_mangle]
pub unsafe extern "C" fn if(URXD_FRMERR: rx &) -> else {
    else if (rx & URXD_FRMERR)
    flg = TTY_FRAME;
    if (rx & URXD_OVRRUN)
    flg = TTY_OVERRUN;
    sport.port.sysrq = 0;
    } else if (uart_handle_sysrq_char(&sport.port, (unsigned char)rx)) {
    continue;
    }
    if (sport.port.ignore_status_mask & URXD_DUMMY_READ)
    continue;
    if (tty_insert_flip_char(port, rx, flg) == 0)
    sport.port.icount.buf_overrun++;
    }
    tty_flip_buffer_push(port);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_rxint(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t imx_uart_rxint(int irq, void *dev_id)
    {
    struct imx_port *sport = dev_id;
    irqreturn_t ret;
    uart_port_lock(&sport.port);
    ret = __imx_uart_rxint(irq, dev_id);
    uart_port_unlock(&sport.port);
    return ret;
    }
    static void imx_uart_clear_rx_errors(struct imx_port *sport);
//
// We have a modem side uart, so the meanings of RTS and CTS are inverted.
//
// called with port.lock taken and irqs off
#[no_mangle]
unsafe extern "C" fn imx_uart_get_hwmctrl(sport: *mut imx_port) -> c_uint {
    static unsigned int imx_uart_get_hwmctrl(struct imx_port *sport)
    {
    let mut tmp: c_uint = TIOCM_DSR;
    let mut usr1: unsigned = imx_uart_readl(sport, USR1);
    let mut usr2: unsigned = imx_uart_readl(sport, USR2);
    if (usr1 & USR1_RTSS)
    tmp |= TIOCM_CTS;
// in DCE mode DCDIN is always 0
    if (!(usr2 & USR2_DCDIN))
    tmp |= TIOCM_CAR;
    if (sport.dte_mode)
    if (!(imx_uart_readl(sport, USR2) & USR2_RIIN))
    tmp |= TIOCM_RI;
    return tmp;
    }
//
// Handle any change of modem status signal since we were last called.
//
// Called with port.lock taken and irqs off.
//
#[no_mangle]
unsafe extern "C" fn imx_uart_mctrl_check(sport: *mut imx_port) {
    static void imx_uart_mctrl_check(struct imx_port *sport)
    {
    unsigned int status, changed;
    status = imx_uart_get_hwmctrl(sport);
    changed = status ^ sport.old_status;
    if (changed == 0)
    return;
    sport.old_status = status;
    if (changed & TIOCM_RI && status & TIOCM_RI)
    sport.port.icount.rng++;
    if (changed & TIOCM_DSR)
    sport.port.icount.dsr++;
    if (changed & TIOCM_CAR)
    uart_handle_dcd_change(&sport.port, status & TIOCM_CAR);
    if (changed & TIOCM_CTS)
    uart_handle_cts_change(&sport.port, status & TIOCM_CTS);
    wake_up_interruptible(&sport.port.state.port.delta_msr_wait);
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_int(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t imx_uart_int(int irq, void *dev_id)
    {
    struct imx_port *sport = dev_id;
    unsigned int usr1, usr2, ucr1, ucr2, ucr3, ucr4;
    let mut ret: irqreturn_t = IRQ_NONE;
    uart_port_lock(&sport.port);
    usr1 = imx_uart_readl(sport, USR1);
    usr2 = imx_uart_readl(sport, USR2);
    ucr1 = imx_uart_readl(sport, UCR1);
    ucr2 = imx_uart_readl(sport, UCR2);
    ucr3 = imx_uart_readl(sport, UCR3);
    ucr4 = imx_uart_readl(sport, UCR4);
//
// Even if a condition is true that can trigger an irq only handle it if
// the respective irq source is enabled. This prevents some undesired
// actions, for example if a character that sits in the RX FIFO and that
// should be fetched via DMA is tried to be fetched using PIO. Or the
// receiver is currently off and so reading from URXD0 results in an
// exception. So just mask the (raw) status bits for disabled irqs.
//
    if ((ucr1 & UCR1_RRDYEN) == 0)
    usr1 &= ~USR1_RRDY;
    if ((ucr2 & UCR2_ATEN) == 0)
    usr1 &= ~USR1_AGTIM;
    if ((ucr1 & UCR1_TRDYEN) == 0)
    usr1 &= ~USR1_TRDY;
    if ((ucr4 & UCR4_TCEN) == 0)
    usr2 &= ~USR2_TXDC;
    if ((ucr3 & UCR3_DTRDEN) == 0)
    usr1 &= ~USR1_DTRD;
    if ((ucr1 & UCR1_RTSDEN) == 0)
    usr1 &= ~USR1_RTSD;
    if ((ucr3 & UCR3_AWAKEN) == 0)
    usr1 &= ~USR1_AWAKE;
    if ((ucr4 & UCR4_OREN) == 0)
    usr2 &= ~USR2_ORE;
    if (usr1 & (USR1_RRDY | USR1_AGTIM)) {
    imx_uart_writel(sport, USR1_AGTIM, USR1);
    __imx_uart_rxint(irq, dev_id);
    ret = IRQ_HANDLED;
    }
    if ((usr1 & USR1_TRDY) || (usr2 & USR2_TXDC)) {
    imx_uart_transmit_buffer(sport);
    ret = IRQ_HANDLED;
    }
    if (usr1 & USR1_DTRD) {
    imx_uart_writel(sport, USR1_DTRD, USR1);
    imx_uart_mctrl_check(sport);
    ret = IRQ_HANDLED;
    }
    if (usr1 & USR1_RTSD) {
    __imx_uart_rtsint(irq, dev_id);
    ret = IRQ_HANDLED;
    }
    if (usr1 & USR1_AWAKE) {
    imx_uart_writel(sport, USR1_AWAKE, USR1);
    ret = IRQ_HANDLED;
    }
    if (usr2 & USR2_ORE) {
    sport.port.icount.overrun++;
    imx_uart_writel(sport, USR2_ORE, USR2);
    ret = IRQ_HANDLED;
    }
    uart_port_unlock(&sport.port);
    return ret;
    }
//
// Return TIOCSER_TEMT when transmitter is not busy.
//
#[no_mangle]
unsafe extern "C" fn imx_uart_tx_empty(port: *mut uart_port) -> c_uint {
    static unsigned int imx_uart_tx_empty(struct uart_port *port)
    {
    struct imx_port *sport = to_imx_port(port);
    unsigned int ret;
    ret = (imx_uart_readl(sport, USR2) & USR2_TXDC) ?  TIOCSER_TEMT : 0;
// If the TX DMA is working, return 0.
    if (sport.dma_is_txing)
    ret = 0;
    return ret;
    }
// called with port.lock taken and irqs off
#[no_mangle]
unsafe extern "C" fn imx_uart_get_mctrl(port: *mut uart_port) -> c_uint {
    static unsigned int imx_uart_get_mctrl(struct uart_port *port)
    {
    struct imx_port *sport = to_imx_port(port);
    let mut ret: c_uint = imx_uart_get_hwmctrl(sport);
    mctrl_gpio_get(sport.gpios, &ret);
    return ret;
    }
// called with port.lock taken and irqs off
#[no_mangle]
unsafe extern "C" fn imx_uart_set_mctrl(port: *mut uart_port, mctrl: c_uint) {
    static void imx_uart_set_mctrl(struct uart_port *port, unsigned int mctrl)
    {
    struct imx_port *sport = to_imx_port(port);
    u32 ucr3, uts;
    if (!(port.rs485.flags & SER_RS485_ENABLED)) {
    u32 ucr2;
//
// Turn off autoRTS if RTS is lowered and restore autoRTS
// setting if RTS is raised.
//
    ucr2 = imx_uart_readl(sport, UCR2);
    ucr2 &= ~(UCR2_CTS | UCR2_CTSC);
    if (mctrl & TIOCM_RTS) {
    ucr2 |= UCR2_CTS;
//
// UCR2_IRTS is unset if and only if the port is
// configured for CRTSCTS, so we use inverted UCR2_IRTS
// to get the state to restore to.
//
    if (!(ucr2 & UCR2_IRTS))
    ucr2 |= UCR2_CTSC;
    }
    imx_uart_writel(sport, ucr2, UCR2);
    }
    ucr3 = imx_uart_readl(sport, UCR3) & ~UCR3_DSR;
    if (!(mctrl & TIOCM_DTR))
    ucr3 |= UCR3_DSR;
    imx_uart_writel(sport, ucr3, UCR3);
    uts = imx_uart_readl(sport, imx_uart_uts_reg(sport)) & ~UTS_LOOP;
    if (mctrl & TIOCM_LOOP)
    uts |= UTS_LOOP;
    imx_uart_writel(sport, uts, imx_uart_uts_reg(sport));
    mctrl_gpio_set(sport.gpios, mctrl);
    }
//
// Interrupts always disabled.
//
#[no_mangle]
unsafe extern "C" fn imx_uart_break_ctl(port: *mut uart_port, break_state: c_int) {
    static void imx_uart_break_ctl(struct uart_port *port, int break_state)
    {
    struct imx_port *sport = to_imx_port(port);
    unsigned long flags;
    u32 ucr1;
    uart_port_lock_irqsave(&sport.port, &flags);
    ucr1 = imx_uart_readl(sport, UCR1) & ~UCR1_SNDBRK;
    if (break_state != 0)
    ucr1 |= UCR1_SNDBRK;
    imx_uart_writel(sport, ucr1, UCR1);
    uart_port_unlock_irqrestore(&sport.port, flags);
    }
//
// This is our per-port timeout handler, for checking the
// modem status signals.
//
#[no_mangle]
unsafe extern "C" fn imx_uart_timeout(t: *mut timer_list) {
    static void imx_uart_timeout(struct timer_list *t)
    {
    struct imx_port *sport = timer_container_of(sport, t, timer);
    unsigned long flags;
    if (sport.port.state) {
    uart_port_lock_irqsave(&sport.port, &flags);
    imx_uart_mctrl_check(sport);
    uart_port_unlock_irqrestore(&sport.port, flags);
    mod_timer(&sport.timer, jiffies + MCTRL_TIMEOUT);
    }
    }
//
// There are two kinds of RX DMA interrupts(such as in the MX6Q):
// [1] the RX DMA buffer is full.
// [2] the aging timer expires
//
// Condition [2] is triggered when a character has been sitting in the FIFO
// for at least 8 byte durations.
//
#[no_mangle]
unsafe extern "C" fn imx_uart_dma_rx_callback(data: *mut c_void) {
    static void imx_uart_dma_rx_callback(void *data)
    {
    struct imx_port *sport = data;
    struct dma_chan	*chan = sport.dma_chan_rx;
    struct scatterlist *sgl = &sport.rx_sgl;
    struct tty_port *port = &sport.port.state.port;
    struct dma_tx_state state;
    struct circ_buf *rx_ring = &sport.rx_ring;
    enum dma_status status;
    let mut w_bytes: c_uint = 0;
    unsigned int r_bytes;
    unsigned int bd_size;
    status = dmaengine_tx_status(chan, sport.rx_cookie, &state);
    if (status == DMA_ERROR) {
    uart_port_lock(&sport.port);
    imx_uart_clear_rx_errors(sport);
    uart_port_unlock(&sport.port);
    return;
    }
//
// The state-residue variable represents the empty space
// relative to the entire buffer. Taking this in consideration
// the head is always calculated base on the buffer total
// length - DMA transaction residue. The UART script from the
// SDMA firmware will jump to the next buffer descriptor,
// once a DMA transaction if finalized (IMX53 RM - A.4.1.2.4).
// Taking this in consideration the tail is always at the
// beginning of the buffer descriptor that contains the head.
//
// Calculate the head
    rx_ring.head = sg_dma_len(sgl) - state.residue;
// Calculate the tail.
    bd_size = sg_dma_len(sgl) / sport.rx_periods;
    rx_ring.tail = ((rx_ring.head-1) / bd_size) * bd_size;
    if (rx_ring.head <= sg_dma_len(sgl) &&
    rx_ring.head > rx_ring.tail) {
// Move data from tail to head
    r_bytes = rx_ring.head - rx_ring.tail;
// If we received something, check for 0xff flood
    uart_port_lock(&sport.port);
    imx_uart_check_flood(sport, imx_uart_readl(sport, USR2));
    uart_port_unlock(&sport.port);
    if (!(sport.port.ignore_status_mask & URXD_DUMMY_READ)) {
// CPU claims ownership of RX DMA buffer
    dma_sync_sg_for_cpu(sport.port.dev, sgl, 1,
    DMA_FROM_DEVICE);
    w_bytes = tty_insert_flip_string(port,
    sport.rx_buf + rx_ring.tail, r_bytes);
// UART retrieves ownership of RX DMA buffer
    dma_sync_sg_for_device(sport.port.dev, sgl, 1,
    DMA_FROM_DEVICE);
    if (w_bytes != r_bytes)
    sport.port.icount.buf_overrun++;
    sport.port.icount.rx += w_bytes;
    }
    } else	{
    WARN_ON(rx_ring.head > sg_dma_len(sgl));
    WARN_ON(rx_ring.head <= rx_ring.tail);
    }
    if (w_bytes) {
    tty_flip_buffer_push(port);
    dev_dbg(sport.port.dev, "We get %d bytes.\n", w_bytes);
    }
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_start_rx_dma(sport: *mut imx_port) -> c_int {
    static int imx_uart_start_rx_dma(struct imx_port *sport)
    {
    struct scatterlist *sgl = &sport.rx_sgl;
    struct dma_chan	*chan = sport.dma_chan_rx;
    struct device *dev = sport.port.dev;
    struct dma_async_tx_descriptor *desc;
    int ret;
    sport.rx_ring.head = 0;
    sport.rx_ring.tail = 0;
    sg_init_one(sgl, sport.rx_buf, sport.rx_buf_size);
    ret = dma_map_sg(dev, sgl, 1, DMA_FROM_DEVICE);
    if (ret == 0) {
    dev_err(dev, "DMA mapping error for RX.\n");
    return -EINVAL;
    }
    desc = dmaengine_prep_dma_cyclic(chan, sg_dma_address(sgl),
    sg_dma_len(sgl), sg_dma_len(sgl) / sport.rx_periods,
    DMA_DEV_TO_MEM, DMA_PREP_INTERRUPT);
    if (!desc) {
    dma_unmap_sg(dev, sgl, 1, DMA_FROM_DEVICE);
    dev_err(dev, "We cannot prepare for the RX slave dma!\n");
    return -EINVAL;
    }
    desc.callback = imx_uart_dma_rx_callback;
    desc.callback_param = sport;
    dev_dbg(dev, "RX: prepare for the DMA.\n");
    sport.dma_is_rxing = 1;
    sport.rx_cookie = dmaengine_submit(desc);
    dma_async_issue_pending(chan);
    return 0;
    }
// called with port.lock taken and irqs off
#[no_mangle]
unsafe extern "C" fn imx_uart_clear_rx_errors(sport: *mut imx_port) {
    static void imx_uart_clear_rx_errors(struct imx_port *sport)
    {
    struct tty_port *port = &sport.port.state.port;
    u32 usr1, usr2;
    usr1 = imx_uart_readl(sport, USR1);
    usr2 = imx_uart_readl(sport, USR2);
    if (usr2 & USR2_BRCD) {
    sport.port.icount.brk++;
    imx_uart_writel(sport, USR2_BRCD, USR2);
    uart_handle_break(&sport.port);
    if (tty_insert_flip_char(port, 0, TTY_BREAK) == 0)
    sport.port.icount.buf_overrun++;
    tty_flip_buffer_push(port);
    } else {
    if (usr1 & USR1_FRAMERR) {
    sport.port.icount.frame++;
    imx_uart_writel(sport, USR1_FRAMERR, USR1);
    } else if (usr1 & USR1_PARITYERR) {
    sport.port.icount.parity++;
    imx_uart_writel(sport, USR1_PARITYERR, USR1);
    }
    }
    if (usr2 & USR2_ORE) {
    sport.port.icount.overrun++;
    imx_uart_writel(sport, USR2_ORE, USR2);
    }
    sport.idle_counter = 0;
    }
pub const TXTL_DEFAULT: c_int = 8;

pub const RXTL_CONSOLE_DEFAULT: c_int = 1;

    static void imx_uart_setup_ufcr(struct imx_port *sport,
    unsigned char txwl, unsigned char rxwl)
    {
    unsigned int val;
// set receiver / transmitter trigger level
    val = imx_uart_readl(sport, UFCR) & (UFCR_RFDIV | UFCR_DCEDTE);
    val |= txwl << UFCR_TXTL_SHF | rxwl;
    imx_uart_writel(sport, val, UFCR);
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_dma_exit(sport: *mut imx_port) {
    static void imx_uart_dma_exit(struct imx_port *sport)
    {
    if (sport.dma_chan_rx) {
    dmaengine_terminate_sync(sport.dma_chan_rx);
    dma_release_channel(sport.dma_chan_rx);
    sport.dma_chan_rx = core::ptr::null_mut();
    sport.rx_cookie = -EINVAL;
    kfree(sport.rx_buf);
    sport.rx_buf = core::ptr::null_mut();
    }
    if (sport.dma_chan_tx) {
    dmaengine_terminate_sync(sport.dma_chan_tx);
    dma_release_channel(sport.dma_chan_tx);
    sport.dma_chan_tx = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_dma_init(sport: *mut imx_port) -> c_int {
    static int imx_uart_dma_init(struct imx_port *sport)
    {
    let mut slave_config: dma_slave_config = {};
    struct device *dev = sport.port.dev;
    struct dma_chan *chan;
    int ret;
// Prepare for RX :
    chan = dma_request_chan(dev, "rx");
    if (IS_ERR(chan)) {
    dev_dbg(dev, "cannot get the DMA channel.\n");
    sport.dma_chan_rx = core::ptr::null_mut();
    ret = PTR_ERR(chan);
    goto err;
    }
    sport.dma_chan_rx = chan;
    slave_config.direction = DMA_DEV_TO_MEM;
    slave_config.src_addr = sport.port.mapbase + URXD0;
    slave_config.src_addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE;
// one byte less than the watermark level to enable the aging timer
    slave_config.src_maxburst = RXTL_DMA - 1;
    ret = dmaengine_slave_config(sport.dma_chan_rx, &slave_config);
    if (ret) {
    dev_err(dev, "error in RX dma configuration.\n");
    goto err;
    }
    sport.rx_buf_size = sport.rx_period_length * sport.rx_periods;
    sport.rx_buf = kzalloc(sport.rx_buf_size, GFP_KERNEL);
    if (!sport.rx_buf) {
    ret = -ENOMEM;
    goto err;
    }
    sport.rx_ring.buf = sport.rx_buf;
// Prepare for TX :
    chan = dma_request_chan(dev, "tx");
    if (IS_ERR(chan)) {
    dev_err(dev, "cannot get the TX DMA channel!\n");
    sport.dma_chan_tx = core::ptr::null_mut();
    ret = PTR_ERR(chan);
    goto err;
    }
    sport.dma_chan_tx = chan;
    slave_config.direction = DMA_MEM_TO_DEV;
    slave_config.dst_addr = sport.port.mapbase + URTX0;
    slave_config.dst_addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE;
    slave_config.dst_maxburst = TXTL_DMA;
    ret = dmaengine_slave_config(sport.dma_chan_tx, &slave_config);
    if (ret) {
    dev_err(dev, "error in TX dma configuration.");
    goto err;
    }
    return 0;
    err:
    imx_uart_dma_exit(sport);
    return ret;
    }
// called with port.lock taken and irqs off
#[no_mangle]
unsafe extern "C" fn imx_uart_enable_dma(sport: *mut imx_port) {
    static void imx_uart_enable_dma(struct imx_port *sport)
    {
    u32 ucr1;
    imx_uart_setup_ufcr(sport, TXTL_DMA, RXTL_DMA);
// set UCR1 except TXDMAEN which would be enabled in imx_uart_dma_tx
    ucr1 = imx_uart_readl(sport, UCR1);
    ucr1 |= UCR1_RXDMAEN | UCR1_ATDMAEN;
    imx_uart_writel(sport, ucr1, UCR1);
    sport.dma_is_enabled = 1;
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_disable_dma(sport: *mut imx_port) {
    static void imx_uart_disable_dma(struct imx_port *sport)
    {
    u32 ucr1;
// clear UCR1
    ucr1 = imx_uart_readl(sport, UCR1);
    ucr1 &= ~(UCR1_RXDMAEN | UCR1_TXDMAEN | UCR1_ATDMAEN);
    imx_uart_writel(sport, ucr1, UCR1);
    imx_uart_setup_ufcr(sport, TXTL_DEFAULT, sport.rxtl);
    sport.dma_is_enabled = 0;
    }
// half the RX buffer size
pub const CTSTL: c_int = 16;
#[no_mangle]
unsafe extern "C" fn imx_uart_startup(port: *mut uart_port) -> c_int {
    static int imx_uart_startup(struct uart_port *port)
    {
    struct imx_port *sport = to_imx_port(port);
    int retval;
    unsigned long flags;
    let mut dma_is_inited: c_int = 0;
    u32 ucr1, ucr2, ucr3, ucr4;
    retval = clk_prepare_enable(sport.clk_per);
    if (retval)
    return retval;
    retval = clk_prepare_enable(sport.clk_ipg);
    if (retval) {
    clk_disable_unprepare(sport.clk_per);
    return retval;
    }
    if (uart_console(&sport.port))
    sport.rxtl = RXTL_CONSOLE_DEFAULT;
    else
    sport.rxtl = RXTL_DEFAULT;
    imx_uart_setup_ufcr(sport, TXTL_DEFAULT, sport.rxtl);
// disable the DREN bit (Data Ready interrupt enable) before
// requesting IRQs
//
    ucr4 = imx_uart_readl(sport, UCR4);
// set the trigger level for CTS
    ucr4 &= ~(UCR4_CTSTL_MASK << UCR4_CTSTL_SHF);
    ucr4 |= CTSTL << UCR4_CTSTL_SHF;
    imx_uart_writel(sport, ucr4 & ~UCR4_DREN, UCR4);
// Can we enable the DMA support?
    if (!uart_console(port) && imx_uart_dma_init(sport) == 0) {
    lockdep_set_subclass(&port.lock, 1);
    dma_is_inited = 1;
    }
    uart_port_lock_irqsave(&sport.port, &flags);
// Reset fifo's and state machines
    imx_uart_soft_reset(sport);
//
// Finally, clear and enable interrupts
//
    imx_uart_writel(sport, USR1_RTSD | USR1_DTRD, USR1);
    imx_uart_writel(sport, USR2_ORE, USR2);
    ucr1 = imx_uart_readl(sport, UCR1) & ~UCR1_RRDYEN;
    ucr1 |= UCR1_UARTEN;
    if (sport.have_rtscts)
    ucr1 |= UCR1_RTSDEN;
    imx_uart_writel(sport, ucr1, UCR1);
    ucr4 = imx_uart_readl(sport, UCR4) & ~(UCR4_OREN | UCR4_INVR);
    if (!dma_is_inited)
    ucr4 |= UCR4_OREN;
    if (sport.inverted_rx)
    ucr4 |= UCR4_INVR;
    imx_uart_writel(sport, ucr4, UCR4);
    ucr3 = imx_uart_readl(sport, UCR3) & ~UCR3_INVT;
//
// configure tx polarity before enabling tx
//
    if (sport.inverted_tx)
    ucr3 |= UCR3_INVT;
    if (!imx_uart_is_imx1(sport)) {
    ucr3 |= UCR3_DTRDEN | UCR3_RI | UCR3_DCD;
    if (sport.dte_mode)
// disable broken interrupts
    ucr3 &= ~(UCR3_RI | UCR3_DCD);
    }
    imx_uart_writel(sport, ucr3, UCR3);
    ucr2 = imx_uart_readl(sport, UCR2) & ~UCR2_ATEN;
    ucr2 |= (UCR2_RXEN | UCR2_TXEN);
    if (!sport.have_rtscts)
    ucr2 |= UCR2_IRTS;
//
// make sure the edge sensitive RTS-irq is disabled,
// we're using RTSD instead.
//
    if (!imx_uart_is_imx1(sport))
    ucr2 &= ~UCR2_RTSEN;
    imx_uart_writel(sport, ucr2, UCR2);
//
// Enable modem status interrupts
//
    imx_uart_enable_ms(&sport.port);
    if (dma_is_inited) {
// Note: enable dma request after transfer start!
    imx_uart_start_rx_dma(sport);
    imx_uart_enable_dma(sport);
    } else {
    ucr1 = imx_uart_readl(sport, UCR1);
    ucr1 |= UCR1_RRDYEN;
    imx_uart_writel(sport, ucr1, UCR1);
    ucr2 = imx_uart_readl(sport, UCR2);
    ucr2 |= UCR2_ATEN;
    imx_uart_writel(sport, ucr2, UCR2);
    }
    imx_uart_disable_loopback_rs485(sport);
    uart_port_unlock_irqrestore(&sport.port, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_shutdown(port: *mut uart_port) {
    static void imx_uart_shutdown(struct uart_port *port)
    {
    struct imx_port *sport = to_imx_port(port);
    unsigned long flags;
    u32 ucr1, ucr2, ucr4, uts;
    int loops;
    if (sport.dma_is_enabled) {
    dmaengine_terminate_sync(sport.dma_chan_tx);
    if (sport.dma_is_txing) {
    dma_unmap_sg(sport.port.dev, &sport.tx_sgl[0],
    sport.dma_tx_nents, DMA_TO_DEVICE);
    sport.dma_is_txing = 0;
    }
    dmaengine_terminate_sync(sport.dma_chan_rx);
    if (sport.dma_is_rxing) {
    dma_unmap_sg(sport.port.dev, &sport.rx_sgl,
    1, DMA_FROM_DEVICE);
    sport.dma_is_rxing = 0;
    }
    uart_port_lock_irqsave(&sport.port, &flags);
    imx_uart_stop_tx(port);
    imx_uart_stop_rx(port);
    imx_uart_disable_dma(sport);
    uart_port_unlock_irqrestore(&sport.port, flags);
    imx_uart_dma_exit(sport);
    }
    mctrl_gpio_disable_ms_sync(sport.gpios);
    uart_port_lock_irqsave(&sport.port, &flags);
    ucr2 = imx_uart_readl(sport, UCR2);
    ucr2 &= ~(UCR2_TXEN | UCR2_ATEN);
    imx_uart_writel(sport, ucr2, UCR2);
    uart_port_unlock_irqrestore(&sport.port, flags);
//
// Stop our timer.
//
    timer_delete_sync(&sport.timer);
//
// Disable all interrupts, port and break condition.
//
    uart_port_lock_irqsave(&sport.port, &flags);
    ucr1 = imx_uart_readl(sport, UCR1);
    ucr1 &= ~(UCR1_TRDYEN | UCR1_RRDYEN | UCR1_RTSDEN | UCR1_RXDMAEN |
    UCR1_ATDMAEN | UCR1_SNDBRK);
// See SER_RS485_ENABLED/UTS_LOOP comment in imx_uart_probe()
    if (port.rs485.flags & SER_RS485_ENABLED &&
    port.rs485.flags & SER_RS485_RTS_ON_SEND &&
    sport.have_rtscts && !sport.have_rtsgpio) {
    uts = imx_uart_readl(sport, imx_uart_uts_reg(sport));
    uts |= UTS_LOOP;
    imx_uart_writel(sport, uts, imx_uart_uts_reg(sport));
    ucr1 |= UCR1_UARTEN;
    } else {
    ucr1 &= ~UCR1_UARTEN;
    }
    imx_uart_writel(sport, ucr1, UCR1);
    ucr4 = imx_uart_readl(sport, UCR4);
    ucr4 &= ~UCR4_TCEN;
    imx_uart_writel(sport, ucr4, UCR4);
//
// We have to ensure the tx state machine ends up in OFF. This
// is especially important for rs485 where we must not leave
// the RTS signal high, blocking the bus indefinitely.
//
// All interrupts are now disabled, so imx_uart_stop_tx() will
// no longer be called from imx_uart_transmit_buffer(). It may
// still be called via the hrtimers, and if those are in play,
// we have to honour the delays.
//
    if (sport.tx_state == WAIT_AFTER_RTS || sport.tx_state == SEND)
    imx_uart_stop_tx(port);
//
// In many cases (rs232 mode, or if tx_state was
// WAIT_AFTER_RTS, or if tx_state was SEND and there is no
// delay_rts_after_send), this will have moved directly to
// OFF. In rs485 mode, tx_state might already have been
// WAIT_AFTER_SEND and the hrtimer thus already started, or
// the above imx_uart_stop_tx() call could have started it. In
// those cases, we have to wait for the hrtimer to fire and
// complete the transition to OFF.
//
    loops = port.rs485.flags & SER_RS485_ENABLED ?
    port.rs485.delay_rts_after_send : 0;
    while (sport.tx_state != OFF && loops--) {
    uart_port_unlock_irqrestore(&sport.port, flags);
    msleep(1);
    uart_port_lock_irqsave(&sport.port, &flags);
    }
    if (sport.tx_state != OFF) {
    dev_warn(sport.port.dev, "unexpected tx_state %d\n",
    sport.tx_state);
//
// This machine may be busted, but ensure the RTS
// signal is inactive in order not to block other
// devices.
//
    if (port.rs485.flags & SER_RS485_ENABLED) {
    ucr2 = imx_uart_readl(sport, UCR2);
    if (port.rs485.flags & SER_RS485_RTS_AFTER_SEND)
    imx_uart_rts_active(sport, &ucr2);
    else
    imx_uart_rts_inactive(sport, &ucr2);
    imx_uart_writel(sport, ucr2, UCR2);
    }
    sport.tx_state = OFF;
    }
    uart_port_unlock_irqrestore(&sport.port, flags);
    clk_disable_unprepare(sport.clk_per);
    clk_disable_unprepare(sport.clk_ipg);
    }
// called with port.lock taken and irqs off
#[no_mangle]
unsafe extern "C" fn imx_uart_flush_buffer(port: *mut uart_port) {
    static void imx_uart_flush_buffer(struct uart_port *port)
    {
    struct imx_port *sport = to_imx_port(port);
    struct scatterlist *sgl = &sport.tx_sgl[0];
    if (!sport.dma_chan_tx)
    return;
    sport.tx_bytes = 0;
    dmaengine_terminate_all(sport.dma_chan_tx);
    if (sport.dma_is_txing) {
    u32 ucr1;
    dma_unmap_sg(sport.port.dev, sgl, sport.dma_tx_nents,
    DMA_TO_DEVICE);
    ucr1 = imx_uart_readl(sport, UCR1);
    ucr1 &= ~UCR1_TXDMAEN;
    imx_uart_writel(sport, ucr1, UCR1);
    sport.dma_is_txing = 0;
    }
    imx_uart_soft_reset(sport);
    }
    static void
    imx_uart_set_termios(struct uart_port *port, struct ktermios *termios,
    const struct ktermios *old)
    {
    struct imx_port *sport = to_imx_port(port);
    unsigned long flags;
    u32 ucr2, old_ucr2, ufcr;
    unsigned int baud, quot;
    let mut old_csize: c_uint = old ? old.c_cflag & CSIZE : CS8;
    unsigned long div;
    unsigned long num, denom, old_ubir, old_ubmr;
    uint64_t tdiv64;
//
// We only support CS7 and CS8.
//
    while ((termios.c_cflag & CSIZE) != CS7 &&
    (termios.c_cflag & CSIZE) != CS8) {
    termios.c_cflag &= ~CSIZE;
    termios.c_cflag |= old_csize;
    old_csize = CS8;
    }
    timer_delete_sync(&sport.timer);
//
// Ask the core to calculate the divisor for us.
//
    baud = uart_get_baud_rate(port, termios, old, 50, port.uartclk / 16);
    quot = uart_get_divisor(port, baud);
    uart_port_lock_irqsave(&sport.port, &flags);
//
// Read current UCR2 and save it for future use, then clear all the bits
// except those we will or may need to preserve.
//
    old_ucr2 = imx_uart_readl(sport, UCR2);
    ucr2 = old_ucr2 & (UCR2_TXEN | UCR2_RXEN | UCR2_ATEN | UCR2_CTS);
    ucr2 |= UCR2_SRST | UCR2_IRTS;
    if ((termios.c_cflag & CSIZE) == CS8)
    ucr2 |= UCR2_WS;
    if (!sport.have_rtscts)
    termios.c_cflag &= ~CRTSCTS;
    if (port.rs485.flags & SER_RS485_ENABLED) {
//
// RTS is mandatory for rs485 operation, so keep
// it under manual control and keep transmitter
// disabled.
//
    if (port.rs485.flags & SER_RS485_RTS_AFTER_SEND)
    imx_uart_rts_active(sport, &ucr2);
    else
    imx_uart_rts_inactive(sport, &ucr2);
    } else if (termios.c_cflag & CRTSCTS) {
//
// Only let receiver control RTS output if we were not requested
// to have RTS inactive (which then should take precedence).
//
    if (ucr2 & UCR2_CTS)
    ucr2 |= UCR2_CTSC;
    }
    if (termios.c_cflag & CRTSCTS)
    ucr2 &= ~UCR2_IRTS;
    if (termios.c_cflag & CSTOPB)
    ucr2 |= UCR2_STPB;
    if (termios.c_cflag & PARENB) {
    ucr2 |= UCR2_PREN;
    if (termios.c_cflag & PARODD)
    ucr2 |= UCR2_PROE;
    }
    sport.port.read_status_mask = 0;
    if (termios.c_iflag & INPCK)
    sport.port.read_status_mask |= (URXD_FRMERR | URXD_PRERR);
    if (termios.c_iflag & (BRKINT | PARMRK))
    sport.port.read_status_mask |= URXD_BRK;
//
// Characters to ignore
//
    sport.port.ignore_status_mask = 0;
    if (termios.c_iflag & IGNPAR)
    sport.port.ignore_status_mask |= URXD_PRERR | URXD_FRMERR;
    if (termios.c_iflag & IGNBRK) {
    sport.port.ignore_status_mask |= URXD_BRK;
//
// If we're ignoring parity and break indicators,
// ignore overruns too (for real raw support).
//
    if (termios.c_iflag & IGNPAR)
    sport.port.ignore_status_mask |= URXD_OVRRUN;
    }
    if ((termios.c_cflag & CREAD) == 0)
    sport.port.ignore_status_mask |= URXD_DUMMY_READ;
//
// Update the per-port timeout.
//
    uart_update_timeout(port, termios.c_cflag, baud);
// custom-baudrate handling
    div = sport.port.uartclk / (baud * 16);
    if (baud == 38400 && quot != div)
    baud = sport.port.uartclk / (quot * 16);
    div = sport.port.uartclk / (baud * 16);
    if (div > 7)
    div = 7;
    if (!div)
    div = 1;
    rational_best_approximation(16 * div * baud, sport.port.uartclk,
    1 << 16, 1 << 16, &num, &denom);
    tdiv64 = sport.port.uartclk;
    tdiv64 *= num;
    do_div(tdiv64, denom * 16 * div);
    tty_termios_encode_baud_rate(termios,
    (speed_t)tdiv64, (speed_t)tdiv64);
    num -= 1;
    denom -= 1;
    ufcr = imx_uart_readl(sport, UFCR);
    ufcr = (ufcr & (~UFCR_RFDIV)) | UFCR_RFDIV_REG(div);
    imx_uart_writel(sport, ufcr, UFCR);
//
// Two registers below should always be written both and in this
// particular order. One consequence is that we need to check if any of
// them changes and then update both. We do need the check for change
// as even writing the same values seem to "restart"
// transmission/receiving logic in the hardware, that leads to data
// breakage even when rate doesn't in fact change. E.g., user switches
// RTS/CTS handshake and suddenly gets broken bytes.
//
    old_ubir = imx_uart_readl(sport, UBIR);
    old_ubmr = imx_uart_readl(sport, UBMR);
    if (old_ubir != num || old_ubmr != denom) {
    imx_uart_writel(sport, num, UBIR);
    imx_uart_writel(sport, denom, UBMR);
    }
    if (!imx_uart_is_imx1(sport))
    imx_uart_writel(sport, sport.port.uartclk / div / 1000,
    IMX21_ONEMS);
    imx_uart_writel(sport, ucr2, UCR2);
    if (UART_ENABLE_MS(&sport.port, termios.c_cflag))
    imx_uart_enable_ms(&sport.port);
    uart_port_unlock_irqrestore(&sport.port, flags);
    }
    static const char *imx_uart_type(struct uart_port *port)
    {
    return port.type == PORT_IMX ? "IMX" : core::ptr::null_mut();
    }
//
// Configure/autoconfigure the port.
//
#[no_mangle]
unsafe extern "C" fn imx_uart_config_port(port: *mut uart_port, flags: c_int) {
    static void imx_uart_config_port(struct uart_port *port, int flags)
    {
    if (flags & UART_CONFIG_TYPE)
    port.type = PORT_IMX;
    }
//
// Verify the new serial_struct (for TIOCSSERIAL).
// The only change we allow are to the flags and type, and
// even then only between PORT_IMX and PORT_UNKNOWN
//
    static int
    imx_uart_verify_port(struct uart_port *port, struct serial_struct *ser)
    {
    let mut ret: c_int = 0;
    if (ser.type != PORT_UNKNOWN && ser.type != PORT_IMX)
    ret = -EINVAL;
    if (port.irq != ser.irq)
    ret = -EINVAL;
    if (ser.io_type != UPIO_MEM)
    ret = -EINVAL;
    if (port.uartclk / 16 != ser.baud_base)
    ret = -EINVAL;
    if (port.mapbase != (unsigned long)ser.iomem_base)
    ret = -EINVAL;
    if (port.iobase != ser.port)
    ret = -EINVAL;
    if (ser.hub6 != 0)
    ret = -EINVAL;
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn imx_uart_poll_init(port: *mut uart_port) -> c_int {
    static int imx_uart_poll_init(struct uart_port *port)
    {
    struct imx_port *sport = to_imx_port(port);
    unsigned long flags;
    u32 ucr1, ucr2;
    int retval;
    retval = clk_prepare_enable(sport.clk_ipg);
    if (retval)
    return retval;
    retval = clk_prepare_enable(sport.clk_per);
    if (retval)
    clk_disable_unprepare(sport.clk_ipg);
    imx_uart_setup_ufcr(sport, TXTL_DEFAULT, sport.rxtl);
    uart_port_lock_irqsave(&sport.port, &flags);
//
// Be careful about the order of enabling bits here. First enable the
// receiver (UARTEN + RXEN) and only then the corresponding irqs.
// This prevents that a character that already sits in the RX fifo is
// triggering an irq but the try to fetch it from there results in an
// exception because UARTEN or RXEN is still off.
//
    ucr1 = imx_uart_readl(sport, UCR1);
    ucr2 = imx_uart_readl(sport, UCR2);
    if (imx_uart_is_imx1(sport))
    ucr1 |= IMX1_UCR1_UARTCLKEN;
    ucr1 |= UCR1_UARTEN;
    ucr1 &= ~(UCR1_TRDYEN | UCR1_RTSDEN | UCR1_RRDYEN);
    ucr2 |= UCR2_RXEN | UCR2_TXEN;
    ucr2 &= ~UCR2_ATEN;
    imx_uart_writel(sport, ucr1, UCR1);
    imx_uart_writel(sport, ucr2, UCR2);
// now enable irqs
    imx_uart_writel(sport, ucr1 | UCR1_RRDYEN, UCR1);
    imx_uart_writel(sport, ucr2 | UCR2_ATEN, UCR2);
    uart_port_unlock_irqrestore(&sport.port, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_poll_get_char(port: *mut uart_port) -> c_int {
    static int imx_uart_poll_get_char(struct uart_port *port)
    {
    struct imx_port *sport = to_imx_port(port);
    if (!(imx_uart_readl(sport, USR2) & USR2_RDR))
    return NO_POLL_CHAR;
    return imx_uart_readl(sport, URXD0) & URXD_RX_DATA;
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_poll_put_char(port: *mut uart_port, c: c_uchar) {
    static void imx_uart_poll_put_char(struct uart_port *port, unsigned char c)
    {
    struct imx_port *sport = to_imx_port(port);
    unsigned int status;
// drain
    do {
    status = imx_uart_readl(sport, USR1);
    } while (~status & USR1_TRDY);
// write
    imx_uart_writel(sport, c, URTX0);
// flush
    do {
    status = imx_uart_readl(sport, USR2);
    } while (~status & USR2_TXDC);
    }

// called with port.lock taken and irqs off or from .probe without locking
    static int imx_uart_rs485_config(struct uart_port *port, struct ktermios *termios,
    struct serial_rs485 *rs485conf)
    {
    struct imx_port *sport = to_imx_port(port);
    u32 ucr2, ufcr;
    if (rs485conf.flags & SER_RS485_ENABLED) {
// Enable receiver if low-active RTS signal is requested
    if (sport.have_rtscts &&  !sport.have_rtsgpio &&
    !(rs485conf.flags & SER_RS485_RTS_ON_SEND))
    rs485conf.flags |= SER_RS485_RX_DURING_TX;
// disable transmitter
    ucr2 = imx_uart_readl(sport, UCR2);
    if (rs485conf.flags & SER_RS485_RTS_AFTER_SEND)
    imx_uart_rts_active(sport, &ucr2);
    else
    imx_uart_rts_inactive(sport, &ucr2);
    imx_uart_writel(sport, ucr2, UCR2);
    }
// Make sure Rx is enabled in case Tx is active with Rx disabled
    if (!(rs485conf.flags & SER_RS485_ENABLED) ||
    rs485conf.flags & SER_RS485_RX_DURING_TX) {
// If the receiver trigger is 0, set it to a default value
    ufcr = imx_uart_readl(sport, UFCR);
    if ((ufcr & UFCR_RXTL_MASK) == 0)
    imx_uart_setup_ufcr(sport, TXTL_DEFAULT, sport.rxtl);
    imx_uart_start_rx(port);
    }
    return 0;
    }
    static const struct uart_ops imx_uart_pops = {
    .tx_empty	= imx_uart_tx_empty,
    .set_mctrl	= imx_uart_set_mctrl,
    .get_mctrl	= imx_uart_get_mctrl,
    .stop_tx	= imx_uart_stop_tx,
    .start_tx	= imx_uart_start_tx,
    .stop_rx	= imx_uart_stop_rx,
    .enable_ms	= imx_uart_enable_ms,
    .break_ctl	= imx_uart_break_ctl,
    .startup	= imx_uart_startup,
    .shutdown	= imx_uart_shutdown,
    .flush_buffer	= imx_uart_flush_buffer,
    .set_termios	= imx_uart_set_termios,
    .type		= imx_uart_type,
    .config_port	= imx_uart_config_port,
    .verify_port	= imx_uart_verify_port,

    .poll_init      = imx_uart_poll_init,
    .poll_get_char  = imx_uart_poll_get_char,
    .poll_put_char  = imx_uart_poll_put_char,

    };
    static struct imx_port *imx_uart_ports[UART_NR];
// Held across uart_add/remove_one_port(); console callbacks must not take it.
    static DEFINE_MUTEX(imx_uart_ports_lock);

#[no_mangle]
unsafe extern "C" fn imx_uart_console_putchar(port: *mut uart_port, ch: c_uchar) {
    static void imx_uart_console_putchar(struct uart_port *port, unsigned char ch)
    {
    struct imx_port *sport = to_imx_port(port);
    while (imx_uart_readl(sport, imx_uart_uts_reg(sport)) & UTS_TXFULL)
    barrier();
    imx_uart_writel(sport, ch, URTX0);
    sport.last_putchar_was_newline = (ch == '\n');
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_console_device_lock(co: *mut console, flags: *mut c_ulong) {
    static void imx_uart_console_device_lock(struct console *co, unsigned long *flags)
    {
    struct uart_port *up = &imx_uart_ports[co.index].port;
    return __uart_port_lock_irqsave(up, flags);
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_console_device_unlock(co: *mut console, flags: c_ulong) {
    static void imx_uart_console_device_unlock(struct console *co, unsigned long flags)
    {
    struct uart_port *up = &imx_uart_ports[co.index].port;
    return __uart_port_unlock_irqrestore(up, flags);
    }
    static void imx_uart_console_write_atomic(struct console *co,
    struct nbcon_write_context *wctxt)
    {
    struct imx_port *sport = imx_uart_ports[co.index];
    struct uart_port *port = &sport.port;
    struct imx_port_ucrs old_ucr;
    unsigned int ucr1, usr2;
    if (!nbcon_enter_unsafe(wctxt))
    return;
//
// First, save UCR1/2/3 and then disable interrupts
//
    imx_uart_ucrs_save(sport, &old_ucr);
    ucr1 = old_ucr.ucr1;
    if (imx_uart_is_imx1(sport))
    ucr1 |= IMX1_UCR1_UARTCLKEN;
    ucr1 |= UCR1_UARTEN;
    ucr1 &= ~(UCR1_TRDYEN | UCR1_RRDYEN | UCR1_RTSDEN);
    imx_uart_writel(sport, ucr1, UCR1);
    imx_uart_writel(sport, old_ucr.ucr2 | UCR2_TXEN, UCR2);
    if (!sport.last_putchar_was_newline)
    uart_console_write(port, "\n", 1, imx_uart_console_putchar);
    uart_console_write(port, wctxt.outbuf, wctxt.len,
    imx_uart_console_putchar);
//
// Finally, wait for transmitter to become empty
// and restore UCR1/2/3
//
    read_poll_timeout_atomic(imx_uart_readl, usr2, usr2 & USR2_TXDC,
    0, USEC_PER_SEC, false, sport, USR2);
    imx_uart_ucrs_restore(sport, &old_ucr);
    nbcon_exit_unsafe(wctxt);
    }
    static void imx_uart_console_write_thread(struct console *co,
    struct nbcon_write_context *wctxt)
    {
    struct imx_port *sport = imx_uart_ports[co.index];
    struct uart_port *port = &sport.port;
    struct imx_port_ucrs old_ucr;
    unsigned int ucr1, usr2;
    if (!nbcon_enter_unsafe(wctxt))
    return;
//
// First, save UCR1/2/3 and then disable interrupts
//
    imx_uart_ucrs_save(sport, &old_ucr);
    ucr1 = old_ucr.ucr1;
    if (imx_uart_is_imx1(sport))
    ucr1 |= IMX1_UCR1_UARTCLKEN;
    ucr1 |= UCR1_UARTEN;
    ucr1 &= ~(UCR1_TRDYEN | UCR1_RRDYEN | UCR1_RTSDEN);
    imx_uart_writel(sport, ucr1, UCR1);
    imx_uart_writel(sport, old_ucr.ucr2 | UCR2_TXEN, UCR2);
    if (nbcon_exit_unsafe(wctxt)) {
    let mut len: c_int = READ_ONCE(wctxt.len);
    int i;
//
// Write out the message. Toggle unsafe for each byte in order
// to give another (higher priority) context the opportunity
// for a friendly takeover. If such a takeover occurs, this
// context must reacquire ownership in order to perform final
// actions (such as re-enabling the interrupts).
//
// IMPORTANT: wctxt->outbuf and wctxt->len are no longer valid
// after a reacquire so writing the message must be
// aborted.
//
    for (i = 0; i < len; i++) {
    if (!nbcon_enter_unsafe(wctxt))
    break;
    uart_console_write(port, wctxt.outbuf + i, 1,
    imx_uart_console_putchar);
    if (!nbcon_exit_unsafe(wctxt))
    break;
    }
    }
    while (!nbcon_enter_unsafe(wctxt))
    nbcon_reacquire_nobuf(wctxt);
//
// Finally, wait for transmitter to become empty
// and restore UCR1/2/3
//
    read_poll_timeout(imx_uart_readl, usr2, usr2 & USR2_TXDC,
    0, USEC_PER_SEC, false, sport, USR2);
    imx_uart_ucrs_restore(sport, &old_ucr);
    nbcon_exit_unsafe(wctxt);
    }
//
// If the port was already initialised (eg, by a boot loader),
// try to determine the current setup.
//
    static void
    imx_uart_console_get_options(struct imx_port *sport, int *baud,
    int *parity, int *bits)
    {
    if (imx_uart_readl(sport, UCR1) & UCR1_UARTEN) {
// ok, the port was enabled
    unsigned int ucr2, ubir, ubmr, uartclk;
    unsigned int baud_raw;
    unsigned int ucfr_rfdiv;
    ucr2 = imx_uart_readl(sport, UCR2);
// parity = 'n';
    if (ucr2 & UCR2_PREN) {
    if (ucr2 & UCR2_PROE)
// parity = 'o';
    else
// parity = 'e';
    }
    if (ucr2 & UCR2_WS)
// bits = 8;
    else
// bits = 7;
    ubir = imx_uart_readl(sport, UBIR) & 0xffff;
    ubmr = imx_uart_readl(sport, UBMR) & 0xffff;
    ucfr_rfdiv = (imx_uart_readl(sport, UFCR) & UFCR_RFDIV) >> 7;
    if (ucfr_rfdiv == 6)
    ucfr_rfdiv = 7;
    else
    ucfr_rfdiv = 6 - ucfr_rfdiv;
    uartclk = clk_get_rate(sport.clk_per);
    uartclk /= ucfr_rfdiv;
    {	/*
// The next code provides exact computation of
// baud_raw = round(((uartclk/16) * (ubir + 1)) / (ubmr + 1))
// without need of float support or long long division,
// which would be required to prevent 32bit arithmetic overflow
//
    let mut mul: c_uint = ubir + 1;
    let mut div: c_uint = 16 * (ubmr + 1);
    let mut rem: c_uint = uartclk % div;
    baud_raw = (uartclk / div) * mul;
    baud_raw += (rem * mul + div / 2) / div;
// baud = (baud_raw + 50) / 100 * 100;
    }
    if (*baud != baud_raw)
    dev_info(sport.port.dev, "Console IMX rounded baud rate from %d to %d\n",
    baud_raw, *baud);
    }
    }
    static int
    imx_uart_console_setup(struct console *co, char *options)
    {
    struct imx_port *sport;
    let mut baud: c_int = 9600;
    let mut bits: c_int = 8;
    let mut parity: c_int = 'n';
    let mut flow: c_int = 'n';
    int retval;
//
// Check whether an invalid uart number has been specified, and
// if so, search for the first available port that does have
// console support.
//
    if (co.index == -1 || co.index >= ARRAY_SIZE(imx_uart_ports))
    co.index = 0;
    sport = imx_uart_ports[co.index];
    if (sport == core::ptr::null_mut())
    return -ENODEV;
// For setting the registers, we only need to enable the ipg clock.
    retval = clk_prepare_enable(sport.clk_ipg);
    if (retval)
    goto error_console;
    sport.last_putchar_was_newline = true;
    if (options)
    uart_parse_options(options, &baud, &parity, &bits, &flow);
    else
    imx_uart_console_get_options(sport, &baud, &parity, &bits);
    imx_uart_setup_ufcr(sport, TXTL_DEFAULT, sport.rxtl);
    retval = uart_set_options(&sport.port, co, baud, parity, bits, flow);
    if (retval) {
    clk_disable_unprepare(sport.clk_ipg);
    goto error_console;
    }
    retval = clk_prepare_enable(sport.clk_per);
    if (retval)
    clk_disable_unprepare(sport.clk_ipg);
    error_console:
    return retval;
    }
    static int
    imx_uart_console_exit(struct console *co)
    {
    struct imx_port *sport = imx_uart_ports[co.index];
    clk_disable_unprepare(sport.clk_per);
    clk_disable_unprepare(sport.clk_ipg);
    return 0;
    }
    static struct uart_driver imx_uart_uart_driver;
    static struct console imx_uart_console = {
    .name		= DEV_NAME,
    .write_atomic	= imx_uart_console_write_atomic,
    .write_thread	= imx_uart_console_write_thread,
    .device_lock	= imx_uart_console_device_lock,
    .device_unlock	= imx_uart_console_device_unlock,
    .flags		= CON_PRINTBUFFER | CON_NBCON,
    .device		= uart_console_device,
    .setup		= imx_uart_console_setup,
    .exit		= imx_uart_console_exit,
    .index		= -1,
    .data		= &imx_uart_uart_driver,
    };

    static struct uart_driver imx_uart_uart_driver = {
    .owner          = THIS_MODULE,
    .driver_name    = DRIVER_NAME,
    .dev_name       = DEV_NAME,
    .major          = SERIAL_IMX_MAJOR,
    .minor          = MINOR_START,
    .nr             = ARRAY_SIZE(imx_uart_ports),
    .cons           = IMX_CONSOLE,
    };
#[no_mangle]
unsafe extern "C" fn imx_trigger_start_tx(t: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart imx_trigger_start_tx(struct hrtimer *t)
    {
    struct imx_port *sport = container_of(t, struct imx_port, trigger_start_tx);
    unsigned long flags;
    uart_port_lock_irqsave(&sport.port, &flags);
    if (sport.tx_state == WAIT_AFTER_RTS)
    imx_uart_start_tx(&sport.port);
    uart_port_unlock_irqrestore(&sport.port, flags);
    return HRTIMER_NORESTART;
    }
#[no_mangle]
unsafe extern "C" fn imx_trigger_stop_tx(t: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart imx_trigger_stop_tx(struct hrtimer *t)
    {
    struct imx_port *sport = container_of(t, struct imx_port, trigger_stop_tx);
    unsigned long flags;
    uart_port_lock_irqsave(&sport.port, &flags);
    if (sport.tx_state == WAIT_AFTER_SEND)
    imx_uart_stop_tx(&sport.port);
    uart_port_unlock_irqrestore(&sport.port, flags);
    return HRTIMER_NORESTART;
    }
    static const struct serial_rs485 imx_rs485_supported = {
    .flags = SER_RS485_ENABLED | SER_RS485_RTS_ON_SEND | SER_RS485_RTS_AFTER_SEND |
    SER_RS485_RX_DURING_TX,
    .delay_rts_before_send = 1,
    .delay_rts_after_send = 1,
    };
// Default RX DMA buffer configuration
pub const RX_DMA_PERIODS: c_int = 16;

#[no_mangle]
unsafe extern "C" fn imx_uart_probe(pdev: *mut platform_device) -> c_int {
    static int imx_uart_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct imx_port *sport;
    void __iomem *base;
    u32 dma_buf_conf[2];
    let mut ret: c_int = 0;
    u32 ucr1, ucr2, uts;
    struct resource *res;
    int txirq, rxirq, rtsirq;
    sport = devm_kzalloc(&pdev.dev, sizeof(*sport), GFP_KERNEL);
    if (!sport)
    return -ENOMEM;
    sport.devdata = of_device_get_match_data(&pdev.dev);
    ret = of_alias_get_id(np, "serial");
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to get alias id, errno %d\n", ret);
    return ret;
    }
    sport.port.line = ret;
    sport.have_rtscts = of_property_read_bool(np, "uart-has-rtscts") ||
    of_property_read_bool(np, "fsl,uart-has-rtscts"); /* deprecated */
    sport.dte_mode = of_property_read_bool(np, "fsl,dte-mode");
    sport.have_rtsgpio = of_property_present(np, "rts-gpios");
    sport.inverted_tx = of_property_read_bool(np, "fsl,inverted-tx");
    sport.inverted_rx = of_property_read_bool(np, "fsl,inverted-rx");
    if (!of_property_read_u32_array(np, "fsl,dma-info", dma_buf_conf, 2)) {
    sport.rx_period_length = dma_buf_conf[0];
    sport.rx_periods = dma_buf_conf[1];
    } else {
    sport.rx_period_length = RX_DMA_PERIOD_LEN;
    sport.rx_periods = RX_DMA_PERIODS;
    }
    if (sport.port.line >= ARRAY_SIZE(imx_uart_ports)) {
    dev_err(&pdev.dev, "serial%d out of range\n",
    sport.port.line);
    return -EINVAL;
    }
    base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    rxirq = platform_get_irq(pdev, 0);
    if (rxirq < 0)
    return rxirq;
    txirq = platform_get_irq_optional(pdev, 1);
    rtsirq = platform_get_irq_optional(pdev, 2);
    sport.port.dev = &pdev.dev;
    sport.port.mapbase = res.start;
    sport.port.membase = base;
    sport.port.type = PORT_IMX;
    sport.port.iotype = UPIO_MEM;
    sport.port.irq = rxirq;
    sport.port.fifosize = 32;
    sport.port.has_sysrq = IS_ENABLED(CONFIG_SERIAL_IMX_CONSOLE);
    sport.port.ops = &imx_uart_pops;
    sport.port.rs485_config = imx_uart_rs485_config;
// RTS is required to control the RS485 transmitter
    if (sport.have_rtscts || sport.have_rtsgpio)
    sport.port.rs485_supported = imx_rs485_supported;
    sport.port.flags = UPF_BOOT_AUTOCONF;
    timer_setup(&sport.timer, imx_uart_timeout, 0);
    sport.gpios = mctrl_gpio_init(&sport.port, 0);
    if (IS_ERR(sport.gpios))
    return PTR_ERR(sport.gpios);
    sport.clk_ipg = devm_clk_get(&pdev.dev, "ipg");
    if (IS_ERR(sport.clk_ipg)) {
    ret = PTR_ERR(sport.clk_ipg);
    dev_err(&pdev.dev, "failed to get ipg clk: %d\n", ret);
    return ret;
    }
    sport.clk_per = devm_clk_get(&pdev.dev, "per");
    if (IS_ERR(sport.clk_per)) {
    ret = PTR_ERR(sport.clk_per);
    dev_err(&pdev.dev, "failed to get per clk: %d\n", ret);
    return ret;
    }
    sport.port.uartclk = clk_get_rate(sport.clk_per);
// For register access, we only need to enable the ipg clock.
    ret = clk_prepare_enable(sport.clk_ipg);
    if (ret) {
    dev_err(&pdev.dev, "failed to enable ipg clk: %d\n", ret);
    return ret;
    }
    ret = uart_get_rs485_mode(&sport.port);
    if (ret)
    goto err_clk;
//
// If using the i.MX UART RTS/CTS control then the RTS (CTS_B)
// signal cannot be set low during transmission in case the
// receiver is off (limitation of the i.MX UART IP).
//
    if (sport.port.rs485.flags & SER_RS485_ENABLED &&
    sport.have_rtscts && !sport.have_rtsgpio &&
    (!(sport.port.rs485.flags & SER_RS485_RTS_ON_SEND) &&
    !(sport.port.rs485.flags & SER_RS485_RX_DURING_TX)))
    dev_err(&pdev.dev,
    "low-active RTS not possible when receiver is off, enabling receiver\n");
// Disable interrupts before requesting them
    ucr1 = imx_uart_readl(sport, UCR1);
    ucr1 &= ~(UCR1_ADEN | UCR1_TRDYEN | UCR1_IDEN | UCR1_RRDYEN | UCR1_RTSDEN);
    imx_uart_writel(sport, ucr1, UCR1);
// Disable Ageing Timer interrupt
    ucr2 = imx_uart_readl(sport, UCR2);
    ucr2 &= ~UCR2_ATEN;
    imx_uart_writel(sport, ucr2, UCR2);
//
// In case RS485 is enabled without GPIO RTS control, the UART IP
// is used to control CTS signal. Keep both the UART and Receiver
// enabled, otherwise the UART IP pulls CTS signal always HIGH no
// matter how the UCR2 CTSC and CTS bits are set. To prevent any
// data from being fed into the RX FIFO, enable loopback mode in
// UTS register, which disconnects the RX path from external RXD
// pin and connects it to the Transceiver, which is disabled, so
// no data can be fed to the RX FIFO that way.
//
    if (sport.port.rs485.flags & SER_RS485_ENABLED &&
    sport.have_rtscts && !sport.have_rtsgpio) {
    uts = imx_uart_readl(sport, imx_uart_uts_reg(sport));
    uts |= UTS_LOOP;
    imx_uart_writel(sport, uts, imx_uart_uts_reg(sport));
    ucr1 = imx_uart_readl(sport, UCR1);
    ucr1 |= UCR1_UARTEN;
    imx_uart_writel(sport, ucr1, UCR1);
    ucr2 = imx_uart_readl(sport, UCR2);
    ucr2 |= UCR2_RXEN;
    imx_uart_writel(sport, ucr2, UCR2);
    }
    if (!imx_uart_is_imx1(sport) && sport.dte_mode) {
//
// The DCEDTE bit changes the direction of DSR, DCD, DTR and RI
// and influences if UCR3_RI and UCR3_DCD changes the level of RI
// and DCD (when they are outputs) or enables the respective
// irqs. So set this bit early, i.e. before requesting irqs.
//
    let mut ufcr: u32 = imx_uart_readl(sport, UFCR);
    if (!(ufcr & UFCR_DCEDTE))
    imx_uart_writel(sport, ufcr | UFCR_DCEDTE, UFCR);
//
// Disable UCR3_RI and UCR3_DCD irqs. They are also not
// enabled later because they cannot be cleared
// (confirmed on i.MX25) which makes them unusable.
//
    imx_uart_writel(sport,
    IMX21_UCR3_RXDMUXSEL | UCR3_ADNIMP | UCR3_DSR,
    UCR3);
    } else {
    let mut ucr3: u32 = UCR3_DSR;
    let mut ufcr: u32 = imx_uart_readl(sport, UFCR);
    if (ufcr & UFCR_DCEDTE)
    imx_uart_writel(sport, ufcr & ~UFCR_DCEDTE, UFCR);
    if (!imx_uart_is_imx1(sport))
    ucr3 |= IMX21_UCR3_RXDMUXSEL | UCR3_ADNIMP;
    imx_uart_writel(sport, ucr3, UCR3);
    }
    hrtimer_setup(&sport.trigger_start_tx, imx_trigger_start_tx, CLOCK_MONOTONIC,
    HRTIMER_MODE_REL);
    hrtimer_setup(&sport.trigger_stop_tx, imx_trigger_stop_tx, CLOCK_MONOTONIC,
    HRTIMER_MODE_REL);
//
// Allocate the IRQ(s) i.MX1 has three interrupts whereas later
// chips only have one interrupt.
//
    if (txirq > 0) {
    ret = devm_request_irq(&pdev.dev, rxirq, imx_uart_rxint, 0,
    dev_name(&pdev.dev), sport);
    if (ret)
    goto err_clk;
    ret = devm_request_irq(&pdev.dev, txirq, imx_uart_txint, 0,
    dev_name(&pdev.dev), sport);
    if (ret)
    goto err_clk;
    ret = devm_request_irq(&pdev.dev, rtsirq, imx_uart_rtsint, 0,
    dev_name(&pdev.dev), sport);
    if (ret)
    goto err_clk;
    } else {
    ret = devm_request_irq(&pdev.dev, rxirq, imx_uart_int, 0,
    dev_name(&pdev.dev), sport);
    if (ret)
    goto err_clk;
    }
    platform_set_drvdata(pdev, sport);
    scoped_guard(mutex, &imx_uart_ports_lock) {
    if (imx_uart_ports[sport.port.line]) {
    ret = -EBUSY;
    } else {
    imx_uart_ports[sport.port.line] = sport;
    ret = uart_add_one_port(&imx_uart_uart_driver,
    &sport.port);
    if (ret)
    imx_uart_ports[sport.port.line] = core::ptr::null_mut();
    }
    }
    err_clk:
    clk_disable_unprepare(sport.clk_ipg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_remove(pdev: *mut platform_device) {
    static void imx_uart_remove(struct platform_device *pdev)
    {
    struct imx_port *sport = platform_get_drvdata(pdev);
    guard(mutex)(&imx_uart_ports_lock);
    uart_remove_one_port(&imx_uart_uart_driver, &sport.port);
    imx_uart_ports[sport.port.line] = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_restore_context(sport: *mut imx_port) {
    static void imx_uart_restore_context(struct imx_port *sport)
    {
    unsigned long flags;
    uart_port_lock_irqsave(&sport.port, &flags);
    if (!sport.context_saved) {
    uart_port_unlock_irqrestore(&sport.port, flags);
    return;
    }
    imx_uart_writel(sport, sport.saved_reg[4], UFCR);
    imx_uart_writel(sport, sport.saved_reg[5], UESC);
    imx_uart_writel(sport, sport.saved_reg[6], UTIM);
    imx_uart_writel(sport, sport.saved_reg[7], UBIR);
    imx_uart_writel(sport, sport.saved_reg[8], UBMR);
    imx_uart_writel(sport, sport.saved_reg[9], IMX21_UTS);
    imx_uart_writel(sport, sport.saved_reg[0], UCR1);
    imx_uart_writel(sport, sport.saved_reg[1] | UCR2_SRST, UCR2);
    imx_uart_writel(sport, sport.saved_reg[2], UCR3);
    imx_uart_writel(sport, sport.saved_reg[3], UCR4);
    sport.context_saved = false;
    uart_port_unlock_irqrestore(&sport.port, flags);
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_save_context(sport: *mut imx_port) {
    static void imx_uart_save_context(struct imx_port *sport)
    {
    unsigned long flags;
// Save necessary regs
    uart_port_lock_irqsave(&sport.port, &flags);
    sport.saved_reg[0] = imx_uart_readl(sport, UCR1);
    sport.saved_reg[1] = imx_uart_readl(sport, UCR2);
    sport.saved_reg[2] = imx_uart_readl(sport, UCR3);
    sport.saved_reg[3] = imx_uart_readl(sport, UCR4);
    sport.saved_reg[4] = imx_uart_readl(sport, UFCR);
    sport.saved_reg[5] = imx_uart_readl(sport, UESC);
    sport.saved_reg[6] = imx_uart_readl(sport, UTIM);
    sport.saved_reg[7] = imx_uart_readl(sport, UBIR);
    sport.saved_reg[8] = imx_uart_readl(sport, UBMR);
    sport.saved_reg[9] = imx_uart_readl(sport, IMX21_UTS);
    sport.context_saved = true;
    uart_port_unlock_irqrestore(&sport.port, flags);
    }
// called with irq off
#[no_mangle]
unsafe extern "C" fn imx_uart_enable_wakeup(sport: *mut imx_port, on: bool) {
    static void imx_uart_enable_wakeup(struct imx_port *sport, bool on)
    {
    struct tty_port *port = &sport.port.state.port;
    struct device *tty_dev;
    let mut may_wake: bool = false, wake_active = false;
    u32 ucr3, usr1;
    scoped_guard(tty_port_tty, port) {
    struct tty_struct *tty = scoped_tty();
    tty_dev = tty.dev;
    may_wake = tty_dev && device_may_wakeup(tty_dev);
    }
// only configure the wake register when device set as wakeup source
    if (!may_wake)
    return;
    uart_port_lock_irq(&sport.port);
    usr1 = imx_uart_readl(sport, USR1);
    ucr3 = imx_uart_readl(sport, UCR3);
    if (on) {
    imx_uart_writel(sport, USR1_AWAKE, USR1);
    ucr3 |= UCR3_AWAKEN;
    } else {
    ucr3 &= ~UCR3_AWAKEN;
    wake_active = usr1 & USR1_AWAKE;
    }
    imx_uart_writel(sport, ucr3, UCR3);
    if (sport.have_rtscts) {
    let mut ucr1: u32 = imx_uart_readl(sport, UCR1);
    if (on) {
    imx_uart_writel(sport, USR1_RTSD, USR1);
    ucr1 |= UCR1_RTSDEN;
    } else {
    ucr1 &= ~UCR1_RTSDEN;
    wake_active = wake_active || (usr1 & USR1_RTSD);
    }
    imx_uart_writel(sport, ucr1, UCR1);
    }
    if (wake_active && irqd_is_wakeup_set(irq_get_irq_data(sport.port.irq)))
    pm_wakeup_event(tty_port_tty_get(port).dev, 0);
    uart_port_unlock_irq(&sport.port);
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_suspend_noirq(dev: *mut device) -> c_int {
    static int imx_uart_suspend_noirq(struct device *dev)
    {
    struct imx_port *sport = dev_get_drvdata(dev);
    imx_uart_save_context(sport);
    clk_disable(sport.clk_ipg);
    pinctrl_pm_select_sleep_state(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_resume_noirq(dev: *mut device) -> c_int {
    static int imx_uart_resume_noirq(struct device *dev)
    {
    struct imx_port *sport = dev_get_drvdata(dev);
    int ret;
    pinctrl_pm_select_default_state(dev);
    ret = clk_enable(sport.clk_ipg);
    if (ret)
    return ret;
    imx_uart_restore_context(sport);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_suspend(dev: *mut device) -> c_int {
    static int imx_uart_suspend(struct device *dev)
    {
    struct imx_port *sport = dev_get_drvdata(dev);
    int ret;
    uart_suspend_port(&imx_uart_uart_driver, &sport.port);
    disable_irq(sport.port.irq);
    ret = clk_prepare_enable(sport.clk_ipg);
    if (ret)
    return ret;
// enable wakeup from i.MX UART
    imx_uart_enable_wakeup(sport, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_resume(dev: *mut device) -> c_int {
    static int imx_uart_resume(struct device *dev)
    {
    struct imx_port *sport = dev_get_drvdata(dev);
// disable wakeup from i.MX UART
    imx_uart_enable_wakeup(sport, false);
    uart_resume_port(&imx_uart_uart_driver, &sport.port);
    enable_irq(sport.port.irq);
    clk_disable_unprepare(sport.clk_ipg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_freeze(dev: *mut device) -> c_int {
    static int imx_uart_freeze(struct device *dev)
    {
    struct imx_port *sport = dev_get_drvdata(dev);
    uart_suspend_port(&imx_uart_uart_driver, &sport.port);
    return clk_prepare_enable(sport.clk_ipg);
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_thaw(dev: *mut device) -> c_int {
    static int imx_uart_thaw(struct device *dev)
    {
    struct imx_port *sport = dev_get_drvdata(dev);
    uart_resume_port(&imx_uart_uart_driver, &sport.port);
    clk_disable_unprepare(sport.clk_ipg);
    return 0;
    }
    static const struct dev_pm_ops imx_uart_pm_ops = {
    .suspend_noirq = imx_uart_suspend_noirq,
    .resume_noirq = imx_uart_resume_noirq,
    .freeze_noirq = imx_uart_suspend_noirq,
    .thaw_noirq = imx_uart_resume_noirq,
    .restore_noirq = imx_uart_resume_noirq,
    .suspend = imx_uart_suspend,
    .resume = imx_uart_resume,
    .freeze = imx_uart_freeze,
    .thaw = imx_uart_thaw,
    .restore = imx_uart_thaw,
    };
    static struct platform_driver imx_uart_platform_driver = {
    .probe = imx_uart_probe,
    .remove = imx_uart_remove,
    .driver = {
    .name = "imx-uart",
    .of_match_table = imx_uart_dt_ids,
    .pm = &imx_uart_pm_ops,
    },
    };
#[no_mangle]
unsafe extern "C" fn imx_uart_init() -> int __init {
    static int __init imx_uart_init(void)
    {
    let mut ret: c_int = uart_register_driver(&imx_uart_uart_driver);
    if (ret)
    return ret;
    ret = platform_driver_register(&imx_uart_platform_driver);
    if (ret != 0)
    uart_unregister_driver(&imx_uart_uart_driver);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx_uart_exit() -> void __exit {
    static void __exit imx_uart_exit(void)
    {
    platform_driver_unregister(&imx_uart_platform_driver);
    uart_unregister_driver(&imx_uart_uart_driver);
    }
    module_init(imx_uart_init);
    module_exit(imx_uart_exit);
    MODULE_AUTHOR("Sascha Hauer");
    MODULE_DESCRIPTION("IMX generic serial port driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:imx-uart");
