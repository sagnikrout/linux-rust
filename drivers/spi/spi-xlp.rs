//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-xlp.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2003-2015 Broadcom Corporation
// All Rights Reserved
//

// SPI Configuration Register
pub const XLP_SPI_CONFIG: c_uint = 0x00;

// SPI Frequency Divider Register
pub const XLP_SPI_FDIV: c_uint = 0x04;
// SPI Command Register
pub const XLP_SPI_CMD: c_uint = 0x08;
pub const XLP_SPI_CMD_IDLE_MASK: c_uint = 0x0;
pub const XLP_SPI_CMD_TX_MASK: c_uint = 0x1;
pub const XLP_SPI_CMD_RX_MASK: c_uint = 0x2;
pub const XLP_SPI_CMD_TXRX_MASK: c_uint = 0x3;

pub const XLP_SPI_XFR_BITCNT_SHIFT: c_int = 16;
// SPI Status Register
pub const XLP_SPI_STATUS: c_uint = 0x0c;

pub const XLP_SPI_STAT_MASK: c_uint = 0x3f;
// SPI Interrupt Enable Register
pub const XLP_SPI_INTR_EN: c_uint = 0x10;

// SPI FIFO Threshold Register
pub const XLP_SPI_FIFO_THRESH: c_uint = 0x14;
// SPI FIFO Word Count Register
pub const XLP_SPI_FIFO_WCNT: c_uint = 0x18;
pub const XLP_SPI_RXFIFO_WCNT_MASK: c_uint = 0xf;
pub const XLP_SPI_TXFIFO_WCNT_MASK: c_uint = 0xf0;
pub const XLP_SPI_TXFIFO_WCNT_SHIFT: c_int = 4;
// SPI Transmit Data FIFO Register
pub const XLP_SPI_TXDATA_FIFO: c_uint = 0x1c;
// SPI Receive Data FIFO Register
pub const XLP_SPI_RXDATA_FIFO: c_uint = 0x20;
// SPI System Control Register
pub const XLP_SPI_SYSCTRL: c_uint = 0x100;

pub const SPI_CS_OFFSET: c_uint = 0x40;
pub const XLP_SPI_TXRXTH: c_uint = 0x80;
pub const XLP_SPI_FIFO_SIZE: c_int = 8;
pub const XLP_SPI_MAX_CS: c_int = 4;
pub const XLP_SPI_DEFAULT_FREQ: c_int = 133333333;
pub const XLP_SPI_FDIV_MIN: c_int = 4;
pub const XLP_SPI_FDIV_MAX: c_int = 65535;
//
// SPI can transfer only 28 bytes properly at a time. So split the
// transfer into 28 bytes size.
//
pub const XLP_SPI_XFER_SIZE: c_int = 28;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlp_spi_priv {
    pub /: *mut *mut device dev; / device structure,
    pub /: *mut *mut *mut void __iomem base; / spi registers base address,
    pub /: *const *const *const u8 tx_buf; / tx data buffer,
    pub /: *mut *mut *mut u8 rx_buf; / rx data buffer,
    pub /: *mut *mut int tx_len; / tx xfer length,
    pub /: *mut *mut int rx_len; / rx xfer length,
    pub /: *mut *mut int txerrors; / TXFIFO underflow count,
    pub /: *mut *mut int rxerrors; / RXFIFO overflow count,
    pub /: *mut *mut int cs; / target device chip select,
    pub /: *mut *mut u32 spi_clk; / spi clock frequency,
    pub /: *mut *mut bool cmd_cont; / cs active,
    pub /: *mut *mut completion done; / completion notification,
}

    static inline u32 xlp_spi_reg_read(struct xlp_spi_priv *priv,
    int cs, int regoff)
    {
    return readl(priv.base + regoff + cs * SPI_CS_OFFSET);
    }
    static inline void xlp_spi_reg_write(struct xlp_spi_priv *priv, int cs,
    int regoff, u32 val)
    {
    writel(val, priv.base + regoff + cs * SPI_CS_OFFSET);
    }
    static inline void xlp_spi_sysctl_write(struct xlp_spi_priv *priv,
    int regoff, u32 val)
    {
    writel(val, priv.base + regoff);
    }
//
// Setup global SPI_SYSCTRL register for all SPI channels.
//
#[no_mangle]
unsafe extern "C" fn xlp_spi_sysctl_setup(xspi: *mut xlp_spi_priv) {
    static void xlp_spi_sysctl_setup(struct xlp_spi_priv *xspi)
    {
    int cs;
    for (cs = 0; cs < XLP_SPI_MAX_CS; cs++)
    xlp_spi_sysctl_write(xspi, XLP_SPI_SYSCTRL,
    XLP_SPI_SYS_RESET << cs);
    xlp_spi_sysctl_write(xspi, XLP_SPI_SYSCTRL, XLP_SPI_SYS_PMEN);
    }
#[no_mangle]
unsafe extern "C" fn xlp_spi_setup(spi: *mut spi_device) -> c_int {
    static int xlp_spi_setup(struct spi_device *spi)
    {
    struct xlp_spi_priv *xspi;
    u32 fdiv, cfg;
    int cs;
    xspi = spi_controller_get_devdata(spi.controller);
    cs = spi_get_chipselect(spi, 0);
//
// The value of fdiv must be between 4 and 65535.
//
    fdiv = DIV_ROUND_UP(xspi.spi_clk, spi.max_speed_hz);
    if (fdiv > XLP_SPI_FDIV_MAX)
    fdiv = XLP_SPI_FDIV_MAX;
#[no_mangle]
pub unsafe extern "C" fn if(XLP_SPI_FDIV_MIN: fdiv <) -> else {
    else if (fdiv < XLP_SPI_FDIV_MIN)
    fdiv = XLP_SPI_FDIV_MIN;
    xlp_spi_reg_write(xspi, cs, XLP_SPI_FDIV, fdiv);
    xlp_spi_reg_write(xspi, cs, XLP_SPI_FIFO_THRESH, XLP_SPI_TXRXTH);
    cfg = xlp_spi_reg_read(xspi, cs, XLP_SPI_CONFIG);
    if (spi.mode & SPI_CPHA)
    cfg |= XLP_SPI_CPHA;
    else
    cfg &= ~XLP_SPI_CPHA;
    if (spi.mode & SPI_CPOL)
    cfg |= XLP_SPI_CPOL;
    else
    cfg &= ~XLP_SPI_CPOL;
    if (!(spi.mode & SPI_CS_HIGH))
    cfg |= XLP_SPI_CS_POL;
    else
    cfg &= ~XLP_SPI_CS_POL;
    if (spi.mode & SPI_LSB_FIRST)
    cfg |= XLP_SPI_CS_LSBFE;
    else
    cfg &= ~XLP_SPI_CS_LSBFE;
    cfg |= XLP_SPI_TXMOSI_EN | XLP_SPI_RXMISO_EN;
    if (fdiv == 4)
    cfg |= XLP_SPI_RXCAP_EN;
    xlp_spi_reg_write(xspi, cs, XLP_SPI_CONFIG, cfg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xlp_spi_read_rxfifo(xspi: *mut xlp_spi_priv) {
    static void xlp_spi_read_rxfifo(struct xlp_spi_priv *xspi)
    {
    u32 rx_data, rxfifo_cnt;
    int i, j, nbytes;
    rxfifo_cnt = xlp_spi_reg_read(xspi, xspi.cs, XLP_SPI_FIFO_WCNT);
    rxfifo_cnt &= XLP_SPI_RXFIFO_WCNT_MASK;
    while (rxfifo_cnt) {
    rx_data = xlp_spi_reg_read(xspi, xspi.cs, XLP_SPI_RXDATA_FIFO);
    j = 0;
    nbytes = min(xspi.rx_len, 4);
    for (i = nbytes - 1; i >= 0; i--, j++)
    xspi.rx_buf[i] = (rx_data >> (j * 8)) & 0xff;
    xspi.rx_len -= nbytes;
    xspi.rx_buf += nbytes;
    rxfifo_cnt--;
    }
    }
#[no_mangle]
unsafe extern "C" fn xlp_spi_fill_txfifo(xspi: *mut xlp_spi_priv) {
    static void xlp_spi_fill_txfifo(struct xlp_spi_priv *xspi)
    {
    u32 tx_data, txfifo_cnt;
    int i, j, nbytes;
    txfifo_cnt = xlp_spi_reg_read(xspi, xspi.cs, XLP_SPI_FIFO_WCNT);
    txfifo_cnt &= XLP_SPI_TXFIFO_WCNT_MASK;
    txfifo_cnt >>= XLP_SPI_TXFIFO_WCNT_SHIFT;
    while (xspi.tx_len && (txfifo_cnt < XLP_SPI_FIFO_SIZE)) {
    j = 0;
    tx_data = 0;
    nbytes = min(xspi.tx_len, 4);
    for (i = nbytes - 1; i >= 0; i--, j++)
    tx_data |= xspi.tx_buf[i] << (j * 8);
    xlp_spi_reg_write(xspi, xspi.cs, XLP_SPI_TXDATA_FIFO, tx_data);
    xspi.tx_len -= nbytes;
    xspi.tx_buf += nbytes;
    txfifo_cnt++;
    }
    }
#[no_mangle]
unsafe extern "C" fn xlp_spi_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t xlp_spi_interrupt(int irq, void *dev_id)
    {
    struct xlp_spi_priv *xspi = dev_id;
    u32 stat;
    stat = xlp_spi_reg_read(xspi, xspi.cs, XLP_SPI_STATUS) &
    XLP_SPI_STAT_MASK;
    if (!stat)
    return IRQ_NONE;
    if (stat & XLP_SPI_TX_INT) {
    if (xspi.tx_len)
    xlp_spi_fill_txfifo(xspi);
    if (stat & XLP_SPI_TX_UF)
    xspi.txerrors++;
    }
    if (stat & XLP_SPI_RX_INT) {
    if (xspi.rx_len)
    xlp_spi_read_rxfifo(xspi);
    if (stat & XLP_SPI_RX_OF)
    xspi.rxerrors++;
    }
// write status back to clear interrupts
    xlp_spi_reg_write(xspi, xspi.cs, XLP_SPI_STATUS, stat);
    if (stat & XLP_SPI_XFR_DONE)
    complete(&xspi.done);
    return IRQ_HANDLED;
    }
    static void xlp_spi_send_cmd(struct xlp_spi_priv *xspi, int xfer_len,
    int cmd_cont)
    {
    let mut cmd: u32 = 0;
    if (xspi.tx_buf)
    cmd |= XLP_SPI_CMD_TX_MASK;
    if (xspi.rx_buf)
    cmd |= XLP_SPI_CMD_RX_MASK;
    if (cmd_cont)
    cmd |= XLP_SPI_CMD_CONT;
    cmd |= ((xfer_len * 8 - 1) << XLP_SPI_XFR_BITCNT_SHIFT);
    xlp_spi_reg_write(xspi, xspi.cs, XLP_SPI_CMD, cmd);
    }
    static int xlp_spi_xfer_block(struct  xlp_spi_priv *xs,
    const unsigned char *tx_buf,
    unsigned char *rx_buf, int xfer_len, int cmd_cont)
    {
    unsigned long time_left;
    let mut intr_mask: u32 = 0;
    xs.tx_buf = tx_buf;
    xs.rx_buf = rx_buf;
    xs.tx_len = (xs.tx_buf == core::ptr::null_mut()) ? 0 : xfer_len;
    xs.rx_len = (xs.rx_buf == core::ptr::null_mut()) ? 0 : xfer_len;
    xs.txerrors = xs.rxerrors = 0;
// fill TXDATA_FIFO, then send the CMD
    if (xs.tx_len)
    xlp_spi_fill_txfifo(xs);
    xlp_spi_send_cmd(xs, xfer_len, cmd_cont);
//
// We are getting some spurious tx interrupts, so avoid enabling
// tx interrupts when only rx is in process.
// Enable all the interrupts in tx case.
//
    if (xs.tx_len)
    intr_mask |= XLP_SPI_INTR_TXTH | XLP_SPI_INTR_TXUF |
    XLP_SPI_INTR_RXTH | XLP_SPI_INTR_RXOF;
    else
    intr_mask |= XLP_SPI_INTR_RXTH | XLP_SPI_INTR_RXOF;
    intr_mask |= XLP_SPI_INTR_DONE;
    xlp_spi_reg_write(xs, xs.cs, XLP_SPI_INTR_EN, intr_mask);
    time_left = wait_for_completion_timeout(&xs.done,
    msecs_to_jiffies(1000));
// Disable interrupts
    xlp_spi_reg_write(xs, xs.cs, XLP_SPI_INTR_EN, 0x0);
    if (!time_left) {
    dev_err(&xs.dev, "xfer timedout!\n");
    goto out;
    }
    if (xs.txerrors || xs.rxerrors)
    dev_err(&xs.dev, "Over/Underflow rx %d tx %d xfer %d!\n",
    xs.rxerrors, xs.txerrors, xfer_len);
    return xfer_len;
    out:
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn xlp_spi_txrx_bufs(xs: *mut xlp_spi_priv, t: *mut spi_transfer) -> c_int {
    static int xlp_spi_txrx_bufs(struct xlp_spi_priv *xs, struct spi_transfer *t)
    {
    int bytesleft, sz;
    unsigned char *rx_buf;
    const unsigned char *tx_buf;
    tx_buf = t.tx_buf;
    rx_buf = t.rx_buf;
    bytesleft = t.len;
    while (bytesleft) {
    if (bytesleft > XLP_SPI_XFER_SIZE)
    sz = xlp_spi_xfer_block(xs, tx_buf, rx_buf,
    XLP_SPI_XFER_SIZE, 1);
    else
    sz = xlp_spi_xfer_block(xs, tx_buf, rx_buf,
    bytesleft, xs.cmd_cont);
    if (sz < 0)
    return sz;
    bytesleft -= sz;
    if (tx_buf)
    tx_buf += sz;
    if (rx_buf)
    rx_buf += sz;
    }
    return bytesleft;
    }
    static int xlp_spi_transfer_one(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct xlp_spi_priv *xspi = spi_controller_get_devdata(host);
    let mut ret: c_int = 0;
    xspi.cs = spi_get_chipselect(spi, 0);
    xspi.dev = spi.dev;
    if (spi_transfer_is_last(host, t))
    xspi.cmd_cont = 0;
    else
    xspi.cmd_cont = 1;
    if (xlp_spi_txrx_bufs(xspi, t))
    ret = -EIO;
    spi_finalize_current_transfer(host);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn xlp_spi_probe(pdev: *mut platform_device) -> c_int {
    static int xlp_spi_probe(struct platform_device *pdev)
    {
    struct spi_controller *host;
    struct xlp_spi_priv *xspi;
    struct clk *clk;
    int irq, err;
    xspi = devm_kzalloc(&pdev.dev, sizeof(*xspi), GFP_KERNEL);
    if (!xspi)
    return -ENOMEM;
    xspi.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(xspi.base))
    return PTR_ERR(xspi.base);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    err = devm_request_irq(&pdev.dev, irq, xlp_spi_interrupt, 0,
    pdev.name, xspi);
    if (err) {
    dev_err(&pdev.dev, "unable to request irq %d\n", irq);
    return err;
    }
    clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(clk)) {
    dev_err(&pdev.dev, "could not get spi clock\n");
    return PTR_ERR(clk);
    }
    xspi.spi_clk = clk_get_rate(clk);
    host = devm_spi_alloc_host(&pdev.dev, 0);
    if (!host) {
    dev_err(&pdev.dev, "could not alloc host\n");
    return -ENOMEM;
    }
    host.bus_num = 0;
    host.num_chipselect = XLP_SPI_MAX_CS;
    host.mode_bits = SPI_CPOL | SPI_CPHA | SPI_CS_HIGH;
    host.setup = xlp_spi_setup;
    host.transfer_one = xlp_spi_transfer_one;
    init_completion(&xspi.done);
    spi_controller_set_devdata(host, xspi);
    xlp_spi_sysctl_setup(xspi);
// register spi controller
    err = devm_spi_register_controller(&pdev.dev, host);
    if (err) {
    dev_err(&pdev.dev, "spi register host failed!\n");
    return err;
    }
    return 0;
    }

    static const struct acpi_device_id xlp_spi_acpi_match[] = {
    { "BRCM900D", 0 },
    { "CAV900D",  0 },
    { },
    };
    MODULE_DEVICE_TABLE(acpi, xlp_spi_acpi_match);

    static struct platform_driver xlp_spi_driver = {
    .probe	= xlp_spi_probe,
    .driver = {
    .name	= "xlp-spi",
    .acpi_match_table = ACPI_PTR(xlp_spi_acpi_match),
    },
    };
    module_platform_driver(xlp_spi_driver);
    MODULE_AUTHOR("Kamlakant Patel <kamlakant.patel@broadcom.com>");
    MODULE_DESCRIPTION("Netlogic XLP SPI controller driver");
    MODULE_LICENSE("GPL v2");
