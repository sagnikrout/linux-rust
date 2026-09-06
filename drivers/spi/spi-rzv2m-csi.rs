//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-rzv2m-csi.c
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
// Renesas RZ/V2M Clocked Serial Interface (CSI) driver
//
// Copyright (C) 2023 Renesas Electronics Corporation
//

// Registers
pub const CSI_MODE: c_uint = 0x00	/* CSI mode control */;
pub const CSI_CLKSEL: c_uint = 0x04	/* CSI clock select */;
pub const CSI_CNT: c_uint = 0x08	/* CSI control */;
pub const CSI_INT: c_uint = 0x0C	/* CSI interrupt status */;
pub const CSI_IFIFOL: c_uint = 0x10	/* CSI receive FIFO level display */;
pub const CSI_OFIFOL: c_uint = 0x14	/* CSI transmit FIFO level display */;
pub const CSI_IFIFO: c_uint = 0x18	/* CSI receive window */;
pub const CSI_OFIFO: c_uint = 0x1C	/* CSI transmit window */;
pub const CSI_FIFOTRG: c_uint = 0x20	/* CSI FIFO trigger level */;
// CSI_MODE

pub const CSI_MODE_SETUP: c_uint = 0x00000040;
// CSI_CLKSEL

// CSI_CNT

// CSI_INT

// CSI_FIFOTRG

pub const CSI_EN_DIS_TIMEOUT_US: c_int = 100;
//
// Clock "csiclk" gets divided by 2 * CSI_CLKSEL_CKS in order to generate the
// serial clock (output from master), with CSI_CLKSEL_CKS ranging from 0x1 (that
// means "csiclk" is divided by 2) to 0x3FFF ("csiclk" is divided by 32766).
//

pub const CSI_CLKSEL_SS_DISABLED: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzv2m_csi_priv {
    pub base: *mut void __iomem,
    pub csiclk: *mut clk,
    pub pclk: *mut clk,
    pub dev: *mut device,
    pub controller: *mut spi_controller,
    pub txbuf: *const c_void,
    pub rxbuf: *mut c_void,
    pub buffer_len: c_uint,
    pub bytes_sent: c_uint,
    pub bytes_received: c_uint,
    pub bytes_to_transfer: c_uint,
    pub words_to_transfer: c_uint,
    pub bytes_per_word: c_uint,
    pub wait: wait_queue_head_t,
    pub errors: u32,
    pub status: u32,
    pub target_aborted: bool,
    pub use_ss_pin: bool,
}

    static void rzv2m_csi_reg_write_bit(const struct rzv2m_csi_priv *csi,
    int reg_offs, int bit_mask, u32 value)
    {
    int nr_zeros;
    u32 tmp;
    nr_zeros = count_trailing_zeros(bit_mask);
    value <<= nr_zeros;
    tmp = (readl(csi.base + reg_offs) & ~bit_mask) | value;
    writel(tmp, csi.base + reg_offs);
    }
#[no_mangle]
unsafe extern "C" fn rzv2m_csi_sw_reset(csi: *mut rzv2m_csi_priv, assert: c_int) -> c_int {
    static int rzv2m_csi_sw_reset(struct rzv2m_csi_priv *csi, int assert)
    {
    u32 reg;
    rzv2m_csi_reg_write_bit(csi, CSI_CNT, CSI_CNT_CSIRST, assert);
    if (!assert)
    return 0;
    return readl_poll_timeout(csi.base + CSI_MODE, reg,
    !(reg & CSI_MODE_CSOT), 0,
    CSI_EN_DIS_TIMEOUT_US);
    }
    static int rzv2m_csi_start_stop_operation(const struct rzv2m_csi_priv *csi,
    int enable, bool wait)
    {
    u32 reg;
    rzv2m_csi_reg_write_bit(csi, CSI_MODE, CSI_MODE_CSIE, enable);
    if (enable || !wait)
    return 0;
    return readl_poll_timeout(csi.base + CSI_MODE, reg,
    !(reg & CSI_MODE_CSOT), 0,
    CSI_EN_DIS_TIMEOUT_US);
    }
#[no_mangle]
unsafe extern "C" fn rzv2m_csi_fill_txfifo(csi: *mut rzv2m_csi_priv) -> c_int {
    static int rzv2m_csi_fill_txfifo(struct rzv2m_csi_priv *csi)
    {
    unsigned int i;
    if (readl(csi.base + CSI_OFIFOL))
    return -EIO;
    if (csi.bytes_per_word == 2) {
    const u16 *buf = csi.txbuf;
    for (i = 0; i < csi.words_to_transfer; i++)
    writel(buf[i], csi.base + CSI_OFIFO);
    } else {
    const u8 *buf = csi.txbuf;
    for (i = 0; i < csi.words_to_transfer; i++)
    writel(buf[i], csi.base + CSI_OFIFO);
    }
    csi.txbuf += csi.bytes_to_transfer;
    csi.bytes_sent += csi.bytes_to_transfer;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzv2m_csi_read_rxfifo(csi: *mut rzv2m_csi_priv) -> c_int {
    static int rzv2m_csi_read_rxfifo(struct rzv2m_csi_priv *csi)
    {
    unsigned int i;
    if (readl(csi.base + CSI_IFIFOL) != csi.bytes_to_transfer)
    return -EIO;
    if (csi.bytes_per_word == 2) {
    u16 *buf = csi.rxbuf;
    for (i = 0; i < csi.words_to_transfer; i++)
    buf[i] = (u16)readl(csi.base + CSI_IFIFO);
    } else {
    u8 *buf = csi.rxbuf;
    for (i = 0; i < csi.words_to_transfer; i++)
    buf[i] = (u8)readl(csi.base + CSI_IFIFO);
    }
    csi.rxbuf += csi.bytes_to_transfer;
    csi.bytes_received += csi.bytes_to_transfer;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rzv2m_csi_empty_rxfifo(csi: *mut rzv2m_csi_priv) {
    static inline void rzv2m_csi_empty_rxfifo(struct rzv2m_csi_priv *csi)
    {
    unsigned int i;
    for (i = 0; i < csi.words_to_transfer; i++)
    readl(csi.base + CSI_IFIFO);
    }
#[no_mangle]
pub unsafe extern "C" fn rzv2m_csi_calc_current_transfer(csi: *mut rzv2m_csi_priv) {
    static inline void rzv2m_csi_calc_current_transfer(struct rzv2m_csi_priv *csi)
    {
    let mut bytes_transferred: c_uint = max(csi.bytes_received, csi.bytes_sent);
    let mut bytes_remaining: c_uint = csi.buffer_len - bytes_transferred;
    unsigned int to_transfer;
    if (csi.txbuf)
//
// Leaving a little bit of headroom in the FIFOs makes it very
// hard to raise an overflow error (which is only possible
// when IP transmits and receives at the same time).
//
    to_transfer = min(CSI_FIFO_HALF_SIZE, bytes_remaining);
    else
    to_transfer = min(CSI_FIFO_SIZE_BYTES, bytes_remaining);
    if (csi.bytes_per_word == 2)
    to_transfer >>= 1;
//
// We can only choose a trigger level from a predefined set of values.
// This will pick a value that is the greatest possible integer that's
// less than or equal to the number of bytes we need to transfer.
// This may result in multiple smaller transfers.
//
    csi.words_to_transfer = rounddown_pow_of_two(to_transfer);
    if (csi.bytes_per_word == 2)
    csi.bytes_to_transfer = csi.words_to_transfer << 1;
    else
    csi.bytes_to_transfer = csi.words_to_transfer;
    }
#[no_mangle]
pub unsafe extern "C" fn rzv2m_csi_set_rx_fifo_trigger_level(csi: *mut rzv2m_csi_priv) {
    static inline void rzv2m_csi_set_rx_fifo_trigger_level(struct rzv2m_csi_priv *csi)
    {
    rzv2m_csi_reg_write_bit(csi, CSI_FIFOTRG, CSI_FIFOTRG_R_TRG,
    ilog2(csi.words_to_transfer));
    }
    static inline void rzv2m_csi_enable_rx_trigger(struct rzv2m_csi_priv *csi,
    bool enable)
    {
    rzv2m_csi_reg_write_bit(csi, CSI_CNT, CSI_CNT_R_TRGEN, enable);
    }
    static void rzv2m_csi_disable_irqs(const struct rzv2m_csi_priv *csi,
    u32 enable_bits)
    {
    let mut cnt: u32 = readl(csi.base + CSI_CNT);
    writel(cnt & ~enable_bits, csi.base + CSI_CNT);
    }
#[no_mangle]
unsafe extern "C" fn rzv2m_csi_disable_all_irqs(csi: *mut rzv2m_csi_priv) {
    static void rzv2m_csi_disable_all_irqs(struct rzv2m_csi_priv *csi)
    {
    rzv2m_csi_disable_irqs(csi, CSI_CNT_R_TRGR_E | CSI_CNT_T_TRGR_E |
    CSI_CNT_CSIEND_E | CSI_CNT_TREND_E |
    CSI_CNT_OVERF_E | CSI_CNT_UNDER_E);
    }
#[no_mangle]
pub unsafe extern "C" fn rzv2m_csi_clear_irqs(csi: *mut rzv2m_csi_priv, irqs: u32) {
    static inline void rzv2m_csi_clear_irqs(struct rzv2m_csi_priv *csi, u32 irqs)
    {
    writel(irqs, csi.base + CSI_INT);
    }
#[no_mangle]
unsafe extern "C" fn rzv2m_csi_clear_all_irqs(csi: *mut rzv2m_csi_priv) {
    static void rzv2m_csi_clear_all_irqs(struct rzv2m_csi_priv *csi)
    {
    rzv2m_csi_clear_irqs(csi, CSI_INT_UNDER | CSI_INT_OVERF |
    CSI_INT_TREND | CSI_INT_CSIEND |  CSI_INT_T_TRGR |
    CSI_INT_R_TRGR);
    }
#[no_mangle]
unsafe extern "C" fn rzv2m_csi_enable_irqs(csi: *mut rzv2m_csi_priv, enable_bits: u32) {
    static void rzv2m_csi_enable_irqs(struct rzv2m_csi_priv *csi, u32 enable_bits)
    {
    let mut cnt: u32 = readl(csi.base + CSI_CNT);
    writel(cnt | enable_bits, csi.base + CSI_CNT);
    }
    static int rzv2m_csi_wait_for_interrupt(struct rzv2m_csi_priv *csi,
    u32 wait_mask, u32 enable_bits)
    {
    int ret;
    rzv2m_csi_enable_irqs(csi, enable_bits);
    if (spi_controller_is_target(csi.controller)) {
    ret = wait_event_interruptible(csi.wait,
    ((csi.status & wait_mask) == wait_mask) ||
    csi.errors || csi.target_aborted);
    if (ret || csi.target_aborted)
    ret = -EINTR;
    } else {
    ret = wait_event_timeout(csi.wait,
    ((csi.status & wait_mask) == wait_mask) ||
    csi.errors, HZ) == 0 ? -ETIMEDOUT : 0;
    }
    rzv2m_csi_disable_irqs(csi, enable_bits);
    if (csi.errors)
    return -EIO;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn rzv2m_csi_wait_for_rx_ready(csi: *mut rzv2m_csi_priv) -> c_int {
    static inline int rzv2m_csi_wait_for_rx_ready(struct rzv2m_csi_priv *csi)
    {
    int ret;
    if (readl(csi.base + CSI_IFIFOL) >= csi.bytes_to_transfer)
    return 0;
    ret = rzv2m_csi_wait_for_interrupt(csi, CSI_INT_R_TRGR,
    CSI_CNT_R_TRGR_E);
    if (ret == -ETIMEDOUT)
    csi.errors |= RX_TIMEOUT_ERROR;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rzv2m_csi_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t rzv2m_csi_irq_handler(int irq, void *data)
    {
    struct rzv2m_csi_priv *csi = data;
    csi.status = readl(csi.base + CSI_INT);
    rzv2m_csi_disable_irqs(csi, csi.status);
    if (csi.status & CSI_INT_OVERF)
    csi.errors |= OVERFLOW_ERROR;
    if (csi.status & CSI_INT_UNDER)
    csi.errors |= UNDERRUN_ERROR;
    wake_up(&csi.wait);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rzv2m_csi_setup_clock(csi: *mut rzv2m_csi_priv, spi_hz: u32) {
    static void rzv2m_csi_setup_clock(struct rzv2m_csi_priv *csi, u32 spi_hz)
    {
    let mut csiclk_rate: c_ulong = clk_get_rate(csi.csiclk);
    let mut pclk_rate: c_ulong = clk_get_rate(csi.pclk);
    let mut csiclk_rate_limit: c_ulong = pclk_rate >> 1;
    u32 cks;
//
// There is a restriction on the frequency of CSICLK, it has to be <=
// PCLK / 2.
//
    if (csiclk_rate > csiclk_rate_limit) {
    clk_set_rate(csi.csiclk, csiclk_rate >> 1);
    csiclk_rate = clk_get_rate(csi.csiclk);
    } else if ((csiclk_rate << 1) <= csiclk_rate_limit) {
    clk_set_rate(csi.csiclk, csiclk_rate << 1);
    csiclk_rate = clk_get_rate(csi.csiclk);
    }
    spi_hz = spi_hz > CSI_MAX_SPI_SCKO ? CSI_MAX_SPI_SCKO : spi_hz;
    cks = DIV_ROUND_UP(csiclk_rate, spi_hz << 1);
    if (cks > CSI_CKS_MAX)
    cks = CSI_CKS_MAX;
    dev_dbg(csi.dev, "SPI clk rate is %ldHz\n", csiclk_rate / (cks << 1));
    rzv2m_csi_reg_write_bit(csi, CSI_CLKSEL, CSI_CLKSEL_CKS, cks);
    }
    static void rzv2m_csi_setup_operating_mode(struct rzv2m_csi_priv *csi,
    struct spi_transfer *t)
    {
    if (t.rx_buf && !t.tx_buf)
// Reception-only mode
    rzv2m_csi_reg_write_bit(csi, CSI_MODE, CSI_MODE_TRMD, 0);
    else
// Send and receive mode
    rzv2m_csi_reg_write_bit(csi, CSI_MODE, CSI_MODE_TRMD, 1);
    csi.bytes_per_word = t.bits_per_word / 8;
    rzv2m_csi_reg_write_bit(csi, CSI_MODE, CSI_MODE_CCL,
    csi.bytes_per_word == 2);
    }
#[no_mangle]
unsafe extern "C" fn rzv2m_csi_setup(spi: *mut spi_device) -> c_int {
    static int rzv2m_csi_setup(struct spi_device *spi)
    {
    struct rzv2m_csi_priv *csi = spi_controller_get_devdata(spi.controller);
    let mut slave_selection: u32 = CSI_CLKSEL_SS_DISABLED;
    int ret;
    rzv2m_csi_sw_reset(csi, 0);
    writel(CSI_MODE_SETUP, csi.base + CSI_MODE);
// Setup clock polarity and phase timing
    rzv2m_csi_reg_write_bit(csi, CSI_CLKSEL, CSI_CLKSEL_MODE,
    ~spi.mode & SPI_MODE_X_MASK);
// Setup serial data order
    rzv2m_csi_reg_write_bit(csi, CSI_MODE, CSI_MODE_DIR,
    !!(spi.mode & SPI_LSB_FIRST));
// Set the role, 1 for target and 0 for host
    rzv2m_csi_reg_write_bit(csi, CSI_CLKSEL, CSI_CLKSEL_SLAVE,
    !!spi_controller_is_target(csi.controller));
    if (csi.use_ss_pin)
    slave_selection = spi.mode & SPI_CS_HIGH ?
    CSI_CLKSEL_SS_ENABLED_ACTIVE_HIGH :
    CSI_CLKSEL_SS_ENABLED_ACTIVE_LOW;
// Configure the slave selection (SS) pin
    rzv2m_csi_reg_write_bit(csi, CSI_CLKSEL, CSI_CLKSEL_SS, slave_selection);
// Give the IP a SW reset
    ret = rzv2m_csi_sw_reset(csi, 1);
    if (ret)
    return ret;
    rzv2m_csi_sw_reset(csi, 0);
//
// We need to enable the communication so that the clock will settle
// for the right polarity before enabling the CS.
//
    rzv2m_csi_start_stop_operation(csi, 1, false);
    udelay(10);
    rzv2m_csi_start_stop_operation(csi, 0, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzv2m_csi_pio_transfer(csi: *mut rzv2m_csi_priv) -> c_int {
    static int rzv2m_csi_pio_transfer(struct rzv2m_csi_priv *csi)
    {
    let mut tx_completed: bool = !csi.txbuf;
    let mut rx_completed: bool = !csi.rxbuf;
    let mut ret: c_int = 0;
// Make sure the TX FIFO is empty
    writel(0, csi.base + CSI_OFIFOL);
// Make sure the RX FIFO is empty
    writel(0, csi.base + CSI_IFIFOL);
    csi.bytes_sent = 0;
    csi.bytes_received = 0;
    csi.errors = 0;
    csi.target_aborted = false;
    rzv2m_csi_disable_all_irqs(csi);
    rzv2m_csi_clear_all_irqs(csi);
    rzv2m_csi_enable_rx_trigger(csi, true);
    while (!tx_completed || !rx_completed) {
//
// Decide how many words we are going to transfer during
// this cycle (for both TX and RX), then set the RX FIFO trigger
// level accordingly. No need to set a trigger level for the
// TX FIFO, as this IP comes with an interrupt that fires when
// the TX FIFO is empty.
//
    rzv2m_csi_calc_current_transfer(csi);
    rzv2m_csi_set_rx_fifo_trigger_level(csi);
    rzv2m_csi_enable_irqs(csi, CSI_INT_OVERF | CSI_INT_UNDER);
    writel(readl(csi.base + CSI_INT), csi.base + CSI_INT);
    csi.status = 0;
// TX
    if (csi.txbuf) {
    ret = rzv2m_csi_fill_txfifo(csi);
    if (ret)
    break;
    if (csi.bytes_sent == csi.buffer_len)
    tx_completed = true;
    }
    rzv2m_csi_start_stop_operation(csi, 1, false);
//
// Make sure the RX FIFO contains the desired number of words.
// We then either flush its content, or we copy it onto
// csi->rxbuf.
//
    ret = rzv2m_csi_wait_for_rx_ready(csi);
    if (ret)
    break;
    if (!spi_controller_is_target(csi.controller))
    rzv2m_csi_start_stop_operation(csi, 0, false);
// RX
    if (csi.rxbuf) {
    ret = rzv2m_csi_read_rxfifo(csi);
    if (ret)
    break;
    if (csi.bytes_received == csi.buffer_len)
    rx_completed = true;
    } else {
    rzv2m_csi_empty_rxfifo(csi);
    }
    if (csi.errors) {
    ret = -EIO;
    break;
    }
    }
    rzv2m_csi_start_stop_operation(csi, 0, true);
    rzv2m_csi_disable_all_irqs(csi);
    rzv2m_csi_enable_rx_trigger(csi, false);
    rzv2m_csi_clear_all_irqs(csi);
    return ret;
    }
    static int rzv2m_csi_transfer_one(struct spi_controller *controller,
    struct spi_device *spi,
    struct spi_transfer *transfer)
    {
    struct rzv2m_csi_priv *csi = spi_controller_get_devdata(controller);
    struct device *dev = csi.dev;
    int ret;
    csi.txbuf = transfer.tx_buf;
    csi.rxbuf = transfer.rx_buf;
    csi.buffer_len = transfer.len;
    rzv2m_csi_setup_operating_mode(csi, transfer);
    if (!spi_controller_is_target(csi.controller))
    rzv2m_csi_setup_clock(csi, transfer.speed_hz);
    ret = rzv2m_csi_pio_transfer(csi);
    if (ret) {
    if (csi.errors & UNDERRUN_ERROR)
    dev_err(dev, "Underrun error\n");
    if (csi.errors & OVERFLOW_ERROR)
    dev_err(dev, "Overflow error\n");
    if (csi.errors & TX_TIMEOUT_ERROR)
    dev_err(dev, "TX timeout error\n");
    if (csi.errors & RX_TIMEOUT_ERROR)
    dev_err(dev, "RX timeout error\n");
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rzv2m_csi_target_abort(ctlr: *mut spi_controller) -> c_int {
    static int rzv2m_csi_target_abort(struct spi_controller *ctlr)
    {
    struct rzv2m_csi_priv *csi = spi_controller_get_devdata(ctlr);
    csi.target_aborted = true;
    wake_up(&csi.wait);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzv2m_csi_probe(pdev: *mut platform_device) -> c_int {
    static int rzv2m_csi_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct spi_controller *controller;
    struct device *dev = &pdev.dev;
    struct rzv2m_csi_priv *csi;
    struct reset_control *rstc;
    bool target_mode;
    int irq;
    int ret;
    target_mode = of_property_read_bool(np, "spi-slave");
    if (target_mode)
    controller = devm_spi_alloc_target(dev, sizeof(*csi));
    else
    controller = devm_spi_alloc_host(dev, sizeof(*csi));
    if (!controller)
    return -ENOMEM;
    csi = spi_controller_get_devdata(controller);
    platform_set_drvdata(pdev, csi);
    csi.use_ss_pin = false;
    if (spi_controller_is_target(controller) &&
    !of_property_read_bool(np, "renesas,csi-no-ss"))
    csi.use_ss_pin = true;
    csi.dev = dev;
    csi.controller = controller;
    csi.target_aborted = false;
    csi.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(csi.base))
    return PTR_ERR(csi.base);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    csi.csiclk = devm_clk_get(dev, "csiclk");
    if (IS_ERR(csi.csiclk))
    return dev_err_probe(dev, PTR_ERR(csi.csiclk),
    "could not get csiclk\n");
    csi.pclk = devm_clk_get(dev, "pclk");
    if (IS_ERR(csi.pclk))
    return dev_err_probe(dev, PTR_ERR(csi.pclk),
    "could not get pclk\n");
    rstc = devm_reset_control_get_shared(dev, core::ptr::null_mut());
    if (IS_ERR(rstc))
    return dev_err_probe(dev, PTR_ERR(rstc), "Missing reset ctrl\n");
    init_waitqueue_head(&csi.wait);
    controller.mode_bits = SPI_CPOL | SPI_CPHA | SPI_LSB_FIRST | SPI_CS_HIGH;
    controller.bits_per_word_mask = SPI_BPW_MASK(16) | SPI_BPW_MASK(8);
    controller.setup = rzv2m_csi_setup;
    controller.transfer_one = rzv2m_csi_transfer_one;
    controller.use_gpio_descriptors = true;
    controller.target_abort = rzv2m_csi_target_abort;
    ret = devm_request_irq(dev, irq, rzv2m_csi_irq_handler, 0,
    dev_name(dev), csi);
    if (ret)
    return dev_err_probe(dev, ret, "cannot request IRQ\n");
//
// The reset also affects other HW that is not under the control
// of Linux. Therefore, all we can do is make sure the reset is
// deasserted.
//
    reset_control_deassert(rstc);
// Make sure the IP is in SW reset state
    ret = rzv2m_csi_sw_reset(csi, 1);
    if (ret)
    return ret;
    ret = clk_prepare_enable(csi.csiclk);
    if (ret)
    return dev_err_probe(dev, ret, "could not enable csiclk\n");
    ret = spi_register_controller(controller);
    if (ret) {
    clk_disable_unprepare(csi.csiclk);
    return dev_err_probe(dev, ret, "register controller failed\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzv2m_csi_remove(pdev: *mut platform_device) {
    static void rzv2m_csi_remove(struct platform_device *pdev)
    {
    struct rzv2m_csi_priv *csi = platform_get_drvdata(pdev);
    spi_unregister_controller(csi.controller);
    rzv2m_csi_sw_reset(csi, 1);
    clk_disable_unprepare(csi.csiclk);
    }
    static const struct of_device_id rzv2m_csi_match[] = {
    { .compatible = "renesas,rzv2m-csi" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rzv2m_csi_match);
    static struct platform_driver rzv2m_csi_drv = {
    .probe = rzv2m_csi_probe,
    .remove = rzv2m_csi_remove,
    .driver = {
    .name = "rzv2m_csi",
    .of_match_table = rzv2m_csi_match,
    },
    };
    module_platform_driver(rzv2m_csi_drv);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Fabrizio Castro <castro.fabrizio.jz@renesas.com>");
    MODULE_DESCRIPTION("Clocked Serial Interface Driver");
