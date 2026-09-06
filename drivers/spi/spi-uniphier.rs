//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-uniphier.c
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
// spi-uniphier.c - Socionext UniPhier SPI controller driver
// Copyright 2012      Panasonic Corporation
// Copyright 2016-2018 Socionext Inc.

pub const SSI_TIMEOUT_MS: c_int = 2000;
pub const SSI_POLL_TIMEOUT_US: c_int = 200;
pub const SSI_MAX_CLK_DIVIDER: c_int = 254;
pub const SSI_MIN_CLK_DIVIDER: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_spi_priv {
    pub base: *mut void __iomem,
    pub base_dma_addr: dma_addr_t,
    pub clk: *mut clk,
    pub host: *mut spi_controller,
    pub xfer_done: completion,
    pub error: c_int,
    pub tx_bytes: c_uint,
    pub rx_bytes: c_uint,
    pub tx_buf: *const u8,
    pub rx_buf: *mut u8,
    pub dma_busy: core::sync::atomic::AtomicI32,
    pub is_save_param: bool,
    pub bits_per_word: u8,
    pub mode: u16,
    pub speed_hz: u32,
}

pub const SSI_CTL: c_uint = 0x00;

pub const SSI_CKS: c_uint = 0x04;

pub const SSI_TXWDS: c_uint = 0x08;

pub const SSI_RXWDS: c_uint = 0x0c;

pub const SSI_FPS: c_uint = 0x10;

pub const SSI_SR: c_uint = 0x14;

pub const SSI_IE: c_uint = 0x18;

pub const SSI_IS: c_uint = 0x1c;

pub const SSI_IC: c_uint = 0x1c;

pub const SSI_FC: c_uint = 0x20;

pub const SSI_TXDR: c_uint = 0x24;
pub const SSI_RXDR: c_uint = 0x24;

pub const SSI_FIFO_BURST_NUM: c_int = 1;

#[no_mangle]
pub unsafe extern "C" fn bytes_per_word(bits: c_uint) -> c_uint {
    static inline unsigned int bytes_per_word(unsigned int bits)
    {
    return bits <= 8 ? 1 : (bits <= 16 ? 2 : 4);
    }
    static inline void uniphier_spi_irq_enable(struct uniphier_spi_priv *priv,
    u32 mask)
    {
    u32 val;
    val = readl(priv.base + SSI_IE);
    val |= mask;
    writel(val, priv.base + SSI_IE);
    }
    static inline void uniphier_spi_irq_disable(struct uniphier_spi_priv *priv,
    u32 mask)
    {
    u32 val;
    val = readl(priv.base + SSI_IE);
    val &= ~mask;
    writel(val, priv.base + SSI_IE);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_spi_set_mode(spi: *mut spi_device) {
    static void uniphier_spi_set_mode(struct spi_device *spi)
    {
    struct uniphier_spi_priv *priv = spi_controller_get_devdata(spi.controller);
    u32 val1, val2;
//
// clock setting
// CKPHS    capture timing. 0:rising edge, 1:falling edge
// CKINIT   clock initial level. 0:low, 1:high
// CKDLY    clock delay. 0:no delay, 1:delay depending on FSTRT
// (FSTRT=0: 1 clock, FSTRT=1: 0.5 clock)
//
// frame setting
// FSPOL    frame signal porarity. 0: low, 1: high
// FSTRT    start frame timing
// 0: rising edge of clock, 1: falling edge of clock
//
    switch (spi.mode & SPI_MODE_X_MASK) {
    case SPI_MODE_0:
// CKPHS=1, CKINIT=0, CKDLY=1, FSTRT=0
    val1 = SSI_CKS_CKPHS | SSI_CKS_CKDLY;
    val2 = 0;
    break;
    case SPI_MODE_1:
// CKPHS=0, CKINIT=0, CKDLY=0, FSTRT=1
    val1 = 0;
    val2 = SSI_FPS_FSTRT;
    break;
    case SPI_MODE_2:
// CKPHS=0, CKINIT=1, CKDLY=1, FSTRT=1
    val1 = SSI_CKS_CKINIT | SSI_CKS_CKDLY;
    val2 = SSI_FPS_FSTRT;
    break;
    case SPI_MODE_3:
// CKPHS=1, CKINIT=1, CKDLY=0, FSTRT=0
    val1 = SSI_CKS_CKPHS | SSI_CKS_CKINIT;
    val2 = 0;
    break;
    }
    if (!(spi.mode & SPI_CS_HIGH))
    val2 |= SSI_FPS_FSPOL;
    writel(val1, priv.base + SSI_CKS);
    writel(val2, priv.base + SSI_FPS);
    val1 = 0;
    if (spi.mode & SPI_LSB_FIRST)
    val1 |= FIELD_PREP(SSI_TXWDS_TDTF_MASK, 1);
    writel(val1, priv.base + SSI_TXWDS);
    writel(val1, priv.base + SSI_RXWDS);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_spi_set_transfer_size(spi: *mut spi_device, size: c_int) {
    static void uniphier_spi_set_transfer_size(struct spi_device *spi, int size)
    {
    struct uniphier_spi_priv *priv = spi_controller_get_devdata(spi.controller);
    u32 val;
    val = readl(priv.base + SSI_TXWDS);
    FIELD_MODIFY(SSI_TXWDS_WDLEN_MASK, &val, size);
    FIELD_MODIFY(SSI_TXWDS_DTLEN_MASK, &val, size);
    writel(val, priv.base + SSI_TXWDS);
    val = readl(priv.base + SSI_RXWDS);
    FIELD_MODIFY(SSI_RXWDS_DTLEN_MASK, &val, size);
    writel(val, priv.base + SSI_RXWDS);
    }
    static void uniphier_spi_set_baudrate(struct spi_device *spi,
    unsigned int speed)
    {
    struct uniphier_spi_priv *priv = spi_controller_get_devdata(spi.controller);
    u32 val, ckdiv;
//
// the supported rates are even numbers from 4 to 254. (4,6,8...254)
// round up as we look for equal or less speed
//
    ckdiv = DIV_ROUND_UP(clk_get_rate(priv.clk), speed);
    ckdiv = round_up(ckdiv, 2);
    val = readl(priv.base + SSI_CKS);
    val &= ~SSI_CKS_CKRAT_MASK;
    val |= ckdiv & SSI_CKS_CKRAT_MASK;
    writel(val, priv.base + SSI_CKS);
    }
    static void uniphier_spi_setup_transfer(struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct uniphier_spi_priv *priv = spi_controller_get_devdata(spi.controller);
    u32 val;
    priv.error = 0;
    priv.tx_buf = t.tx_buf;
    priv.rx_buf = t.rx_buf;
    priv.tx_bytes = priv.rx_bytes = t.len;
    if (!priv.is_save_param || priv.mode != spi.mode) {
    uniphier_spi_set_mode(spi);
    priv.mode = spi.mode;
    priv.is_save_param = false;
    }
    if (!priv.is_save_param || priv.bits_per_word != t.bits_per_word) {
    uniphier_spi_set_transfer_size(spi, t.bits_per_word);
    priv.bits_per_word = t.bits_per_word;
    }
    if (!priv.is_save_param || priv.speed_hz != t.speed_hz) {
    uniphier_spi_set_baudrate(spi, t.speed_hz);
    priv.speed_hz = t.speed_hz;
    }
    priv.is_save_param = true;
// reset FIFOs
    val = SSI_FC_TXFFL | SSI_FC_RXFFL;
    writel(val, priv.base + SSI_FC);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_spi_send(priv: *mut uniphier_spi_priv) {
    static void uniphier_spi_send(struct uniphier_spi_priv *priv)
    {
    int wsize;
    let mut val: u32 = 0;
    wsize = min(bytes_per_word(priv.bits_per_word), priv.tx_bytes);
    priv.tx_bytes -= wsize;
    if (priv.tx_buf) {
    switch (wsize) {
    case 1:
    val = *priv.tx_buf;
    break;
    case 2:
    val = get_unaligned_le16(priv.tx_buf);
    break;
    case 4:
    val = get_unaligned_le32(priv.tx_buf);
    break;
    }
    priv.tx_buf += wsize;
    }
    writel(val, priv.base + SSI_TXDR);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_spi_recv(priv: *mut uniphier_spi_priv) {
    static void uniphier_spi_recv(struct uniphier_spi_priv *priv)
    {
    int rsize;
    u32 val;
    rsize = min(bytes_per_word(priv.bits_per_word), priv.rx_bytes);
    priv.rx_bytes -= rsize;
    val = readl(priv.base + SSI_RXDR);
    if (priv.rx_buf) {
    switch (rsize) {
    case 1:
// priv->rx_buf = val;
    break;
    case 2:
    put_unaligned_le16(val, priv.rx_buf);
    break;
    case 4:
    put_unaligned_le32(val, priv.rx_buf);
    break;
    }
    priv.rx_buf += rsize;
    }
    }
    static void uniphier_spi_set_fifo_threshold(struct uniphier_spi_priv *priv,
    unsigned int threshold)
    {
    u32 val;
    val = readl(priv.base + SSI_FC);
    FIELD_MODIFY(SSI_FC_TXFTH_MASK, &val, SSI_FIFO_DEPTH - threshold);
    FIELD_MODIFY(SSI_FC_RXFTH_MASK, &val, threshold);
    writel(val, priv.base + SSI_FC);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_spi_fill_tx_fifo(priv: *mut uniphier_spi_priv) {
    static void uniphier_spi_fill_tx_fifo(struct uniphier_spi_priv *priv)
    {
    unsigned int fifo_threshold, fill_words;
    let mut bpw: c_uint = bytes_per_word(priv.bits_per_word);
    fifo_threshold = DIV_ROUND_UP(priv.rx_bytes, bpw);
    fifo_threshold = min(fifo_threshold, SSI_FIFO_DEPTH);
    uniphier_spi_set_fifo_threshold(priv, fifo_threshold);
    fill_words = fifo_threshold -
    DIV_ROUND_UP(priv.rx_bytes - priv.tx_bytes, bpw);
    while (fill_words--)
    uniphier_spi_send(priv);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_spi_set_cs(spi: *mut spi_device, enable: bool) {
    static void uniphier_spi_set_cs(struct spi_device *spi, bool enable)
    {
    struct uniphier_spi_priv *priv = spi_controller_get_devdata(spi.controller);
    u32 val;
    val = readl(priv.base + SSI_FPS);
    if (enable)
    val |= SSI_FPS_FSPOL;
    else
    val &= ~SSI_FPS_FSPOL;
    writel(val, priv.base + SSI_FPS);
    }
    static bool uniphier_spi_can_dma(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct uniphier_spi_priv *priv = spi_controller_get_devdata(host);
    let mut bpw: c_uint = bytes_per_word(priv.bits_per_word);
    if ((!host.dma_tx && !host.dma_rx)
    || (!host.dma_tx && t.tx_buf)
    || (!host.dma_rx && t.rx_buf))
    return false;
    return DIV_ROUND_UP(t.len, bpw) > SSI_FIFO_DEPTH;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_spi_dma_rxcb(data: *mut c_void) {
    static void uniphier_spi_dma_rxcb(void *data)
    {
    struct spi_controller *host = data;
    struct uniphier_spi_priv *priv = spi_controller_get_devdata(host);
    let mut state: c_int = atomic_fetch_andnot(SSI_DMA_RX_BUSY, &priv.dma_busy);
    uniphier_spi_irq_disable(priv, SSI_IE_RXRE);
    if (!(state & SSI_DMA_TX_BUSY))
    spi_finalize_current_transfer(host);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_spi_dma_txcb(data: *mut c_void) {
    static void uniphier_spi_dma_txcb(void *data)
    {
    struct spi_controller *host = data;
    struct uniphier_spi_priv *priv = spi_controller_get_devdata(host);
    let mut state: c_int = atomic_fetch_andnot(SSI_DMA_TX_BUSY, &priv.dma_busy);
    uniphier_spi_irq_disable(priv, SSI_IE_TXRE);
    if (!(state & SSI_DMA_RX_BUSY))
    spi_finalize_current_transfer(host);
    }
    static int uniphier_spi_transfer_one_dma(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct uniphier_spi_priv *priv = spi_controller_get_devdata(host);
    struct dma_async_tx_descriptor *rxdesc = core::ptr::null_mut(), *txdesc = core::ptr::null_mut();
    int buswidth;
    atomic_set(&priv.dma_busy, 0);
    uniphier_spi_set_fifo_threshold(priv, SSI_FIFO_BURST_NUM);
    if (priv.bits_per_word <= 8)
    buswidth = DMA_SLAVE_BUSWIDTH_1_BYTE;
#[no_mangle]
pub unsafe extern "C" fn if(16: priv->bits_per_word <=) -> else {
    else if (priv.bits_per_word <= 16)
    buswidth = DMA_SLAVE_BUSWIDTH_2_BYTES;
    else
    buswidth = DMA_SLAVE_BUSWIDTH_4_BYTES;
    if (priv.rx_buf) {
    struct dma_slave_config rxconf = {
    .direction = DMA_DEV_TO_MEM,
    .src_addr = priv.base_dma_addr + SSI_RXDR,
    .src_addr_width = buswidth,
    .src_maxburst = SSI_FIFO_BURST_NUM,
    };
    dmaengine_slave_config(host.dma_rx, &rxconf);
    rxdesc = dmaengine_prep_slave_sg(
    host.dma_rx,
    t.rx_sg.sgl, t.rx_sg.nents,
    DMA_DEV_TO_MEM, DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!rxdesc)
    goto out_err_prep;
    rxdesc.callback = uniphier_spi_dma_rxcb;
    rxdesc.callback_param = host;
    uniphier_spi_irq_enable(priv, SSI_IE_RXRE);
    atomic_or(SSI_DMA_RX_BUSY, &priv.dma_busy);
    dmaengine_submit(rxdesc);
    dma_async_issue_pending(host.dma_rx);
    }
    if (priv.tx_buf) {
    struct dma_slave_config txconf = {
    .direction = DMA_MEM_TO_DEV,
    .dst_addr = priv.base_dma_addr + SSI_TXDR,
    .dst_addr_width = buswidth,
    .dst_maxburst = SSI_FIFO_BURST_NUM,
    };
    dmaengine_slave_config(host.dma_tx, &txconf);
    txdesc = dmaengine_prep_slave_sg(
    host.dma_tx,
    t.tx_sg.sgl, t.tx_sg.nents,
    DMA_MEM_TO_DEV, DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!txdesc)
    goto out_err_prep;
    txdesc.callback = uniphier_spi_dma_txcb;
    txdesc.callback_param = host;
    uniphier_spi_irq_enable(priv, SSI_IE_TXRE);
    atomic_or(SSI_DMA_TX_BUSY, &priv.dma_busy);
    dmaengine_submit(txdesc);
    dma_async_issue_pending(host.dma_tx);
    }
// signal that we need to wait for completion
    return (priv.tx_buf || priv.rx_buf);
    out_err_prep:
    if (rxdesc)
    dmaengine_terminate_sync(host.dma_rx);
    return -EINVAL;
    }
    static int uniphier_spi_transfer_one_irq(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct uniphier_spi_priv *priv = spi_controller_get_devdata(host);
    struct device *dev = host.dev.parent;
    unsigned long time_left;
    reinit_completion(&priv.xfer_done);
    uniphier_spi_fill_tx_fifo(priv);
    uniphier_spi_irq_enable(priv, SSI_IE_RCIE | SSI_IE_RORIE);
    time_left = wait_for_completion_timeout(&priv.xfer_done,
    msecs_to_jiffies(SSI_TIMEOUT_MS));
    uniphier_spi_irq_disable(priv, SSI_IE_RCIE | SSI_IE_RORIE);
    if (!time_left) {
    dev_err(dev, "transfer timeout.\n");
    return -ETIMEDOUT;
    }
    return priv.error;
    }
    static int uniphier_spi_transfer_one_poll(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct uniphier_spi_priv *priv = spi_controller_get_devdata(host);
    let mut loop: c_int = SSI_POLL_TIMEOUT_US * 10;
    while (priv.tx_bytes) {
    uniphier_spi_fill_tx_fifo(priv);
    while ((priv.rx_bytes - priv.tx_bytes) > 0) {
    while (!(readl(priv.base + SSI_SR) & SSI_SR_RNE)
    && loop--)
    ndelay(100);
    if (loop == -1)
    goto irq_transfer;
    uniphier_spi_recv(priv);
    }
    }
    return 0;
    irq_transfer:
    return uniphier_spi_transfer_one_irq(host, spi, t);
    }
    static int uniphier_spi_transfer_one(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct uniphier_spi_priv *priv = spi_controller_get_devdata(host);
    unsigned long threshold;
    bool use_dma;
// Terminate and return success for 0 byte length transfer
    if (!t.len)
    return 0;
    uniphier_spi_setup_transfer(spi, t);
    use_dma = host.can_dma ? host.can_dma(host, spi, t) : false;
    if (use_dma)
    return uniphier_spi_transfer_one_dma(host, spi, t);
//
// If the transfer operation will take longer than
// SSI_POLL_TIMEOUT_US, it should use irq.
//
    threshold = DIV_ROUND_UP(SSI_POLL_TIMEOUT_US * priv.speed_hz,
    USEC_PER_SEC * BITS_PER_BYTE);
    if (t.len > threshold)
    return uniphier_spi_transfer_one_irq(host, spi, t);
    else
    return uniphier_spi_transfer_one_poll(host, spi, t);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_spi_prepare_transfer_hardware(host: *mut spi_controller) -> c_int {
    static int uniphier_spi_prepare_transfer_hardware(struct spi_controller *host)
    {
    struct uniphier_spi_priv *priv = spi_controller_get_devdata(host);
    writel(SSI_CTL_EN, priv.base + SSI_CTL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_spi_unprepare_transfer_hardware(host: *mut spi_controller) -> c_int {
    static int uniphier_spi_unprepare_transfer_hardware(struct spi_controller *host)
    {
    struct uniphier_spi_priv *priv = spi_controller_get_devdata(host);
    writel(0, priv.base + SSI_CTL);
    return 0;
    }
    static void uniphier_spi_handle_err(struct spi_controller *host,
    struct spi_message *msg)
    {
    struct uniphier_spi_priv *priv = spi_controller_get_devdata(host);
    u32 val;
// stop running spi transfer
    writel(0, priv.base + SSI_CTL);
// reset FIFOs
    val = SSI_FC_TXFFL | SSI_FC_RXFFL;
    writel(val, priv.base + SSI_FC);
    uniphier_spi_irq_disable(priv, SSI_IE_ALL_MASK);
    if (atomic_read(&priv.dma_busy) & SSI_DMA_TX_BUSY) {
    dmaengine_terminate_async(host.dma_tx);
    atomic_andnot(SSI_DMA_TX_BUSY, &priv.dma_busy);
    }
    if (atomic_read(&priv.dma_busy) & SSI_DMA_RX_BUSY) {
    dmaengine_terminate_async(host.dma_rx);
    atomic_andnot(SSI_DMA_RX_BUSY, &priv.dma_busy);
    }
    }
#[no_mangle]
unsafe extern "C" fn uniphier_spi_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t uniphier_spi_handler(int irq, void *dev_id)
    {
    struct uniphier_spi_priv *priv = dev_id;
    u32 val, stat;
    stat = readl(priv.base + SSI_IS);
    val = SSI_IC_TCIC | SSI_IC_RCIC | SSI_IC_RORIC;
    writel(val, priv.base + SSI_IC);
// rx fifo overrun
    if (stat & SSI_IS_RORID) {
    priv.error = -EIO;
    goto done;
    }
// rx complete
    if ((stat & SSI_IS_RCID) && (stat & SSI_IS_RXRS)) {
    while ((readl(priv.base + SSI_SR) & SSI_SR_RNE) &&
    (priv.rx_bytes - priv.tx_bytes) > 0)
    uniphier_spi_recv(priv);
    if ((readl(priv.base + SSI_SR) & SSI_SR_RNE) ||
    (priv.rx_bytes != priv.tx_bytes)) {
    priv.error = -EIO;
    goto done;
    } else if (priv.rx_bytes == 0)
    goto done;
// next tx transfer
    uniphier_spi_fill_tx_fifo(priv);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    done:
    complete(&priv.xfer_done);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_spi_probe(pdev: *mut platform_device) -> c_int {
    static int uniphier_spi_probe(struct platform_device *pdev)
    {
    struct uniphier_spi_priv *priv;
    struct spi_controller *host;
    struct resource *res;
    struct dma_slave_caps caps;
    let mut dma_tx_burst: u32 = 0, dma_rx_burst = 0;
    unsigned long clk_rate;
    int irq;
    int ret;
    host = devm_spi_alloc_host(&pdev.dev, sizeof(*priv));
    if (!host)
    return -ENOMEM;
    platform_set_drvdata(pdev, host);
    priv = spi_controller_get_devdata(host);
    priv.host = host;
    priv.is_save_param = false;
    init_completion(&priv.xfer_done);
    priv.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    priv.base_dma_addr = res.start;
    priv.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk)) {
    dev_err(&pdev.dev, "failed to get clock\n");
    return PTR_ERR(priv.clk);
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(&pdev.dev, irq, uniphier_spi_handler,
    0, "uniphier-spi", priv);
    if (ret) {
    dev_err(&pdev.dev, "failed to request IRQ\n");
    return ret;
    }
    clk_rate = clk_get_rate(priv.clk);
    host.max_speed_hz = DIV_ROUND_UP(clk_rate, SSI_MIN_CLK_DIVIDER);
    host.min_speed_hz = DIV_ROUND_UP(clk_rate, SSI_MAX_CLK_DIVIDER);
    host.mode_bits = SPI_CPOL | SPI_CPHA | SPI_CS_HIGH | SPI_LSB_FIRST;
    host.bus_num = pdev.id;
    host.bits_per_word_mask = SPI_BPW_RANGE_MASK(1, 32);
    host.set_cs = uniphier_spi_set_cs;
    host.transfer_one = uniphier_spi_transfer_one;
    host.prepare_transfer_hardware
    = uniphier_spi_prepare_transfer_hardware;
    host.unprepare_transfer_hardware
    = uniphier_spi_unprepare_transfer_hardware;
    host.handle_err = uniphier_spi_handle_err;
    host.can_dma = uniphier_spi_can_dma;
    host.num_chipselect = 1;
    host.flags = SPI_CONTROLLER_MUST_RX | SPI_CONTROLLER_MUST_TX;
    host.dma_tx = dma_request_chan(&pdev.dev, "tx");
    if (IS_ERR_OR_NULL(host.dma_tx)) {
    if (PTR_ERR(host.dma_tx) == -EPROBE_DEFER)
    return -EPROBE_DEFER;
    host.dma_tx = core::ptr::null_mut();
    dma_tx_burst = INT_MAX;
    } else {
    ret = dma_get_slave_caps(host.dma_tx, &caps);
    if (ret) {
    dev_err(&pdev.dev, "failed to get TX DMA capacities: %d\n",
    ret);
    goto out_release_dma;
    }
    dma_tx_burst = caps.max_burst;
    }
    host.dma_rx = dma_request_chan(&pdev.dev, "rx");
    if (IS_ERR_OR_NULL(host.dma_rx)) {
    if (PTR_ERR(host.dma_rx) == -EPROBE_DEFER) {
    ret = -EPROBE_DEFER;
    goto out_release_dma;
    }
    host.dma_rx = core::ptr::null_mut();
    dma_rx_burst = INT_MAX;
    } else {
    ret = dma_get_slave_caps(host.dma_rx, &caps);
    if (ret) {
    dev_err(&pdev.dev, "failed to get RX DMA capacities: %d\n",
    ret);
    goto out_release_dma;
    }
    dma_rx_burst = caps.max_burst;
    }
    host.max_dma_len = min(dma_tx_burst, dma_rx_burst);
    ret = spi_register_controller(host);
    if (ret)
    goto out_release_dma;
    return 0;
    out_release_dma:
    if (!IS_ERR_OR_NULL(host.dma_rx)) {
    dma_release_channel(host.dma_rx);
    host.dma_rx = core::ptr::null_mut();
    }
    if (!IS_ERR_OR_NULL(host.dma_tx)) {
    dma_release_channel(host.dma_tx);
    host.dma_tx = core::ptr::null_mut();
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_spi_remove(pdev: *mut platform_device) {
    static void uniphier_spi_remove(struct platform_device *pdev)
    {
    struct spi_controller *host = platform_get_drvdata(pdev);
    spi_unregister_controller(host);
    if (host.dma_tx)
    dma_release_channel(host.dma_tx);
    if (host.dma_rx)
    dma_release_channel(host.dma_rx);
    }
    static const struct of_device_id uniphier_spi_match[] = {
    { .compatible = "socionext,uniphier-scssi" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, uniphier_spi_match);
    static struct platform_driver uniphier_spi_driver = {
    .probe = uniphier_spi_probe,
    .remove = uniphier_spi_remove,
    .driver = {
    .name = "uniphier-spi",
    .of_match_table = uniphier_spi_match,
    },
    };
    module_platform_driver(uniphier_spi_driver);
    MODULE_AUTHOR("Kunihiko Hayashi <hayashi.kunihiko@socionext.com>");
    MODULE_AUTHOR("Keiji Hayashibara <hayashibara.keiji@socionext.com>");
    MODULE_DESCRIPTION("Socionext UniPhier SPI controller driver");
    MODULE_LICENSE("GPL v2");
