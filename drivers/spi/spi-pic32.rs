//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-pic32.c
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
// Microchip PIC32 SPI controller driver.
//
// Purna Chandra Mandal <purna.mandal@microchip.com>
// Copyright (c) 2016, Microchip Technology Inc.
//

// SPI controller registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pic32_spi_regs {
    pub ctrl: u32,
    pub ctrl_clr: u32,
    pub ctrl_set: u32,
    pub ctrl_inv: u32,
    pub status: u32,
    pub status_clr: u32,
    pub status_set: u32,
    pub status_inv: u32,
    pub buf: u32,
    pub dontuse: [u32; 3],
    pub baud: u32,
    pub dontuse2: [u32; 3],
    pub ctrl2: u32,
    pub ctrl2_clr: u32,
    pub ctrl2_set: u32,
    pub ctrl2_inv: u32,
}

// Bit fields of SPI Control Register

pub const RX_FIFO_EMPTY: c_int = 0;

pub const CTRL_BPW_MASK: c_uint = 0x03   /* bits per word/sample */;
pub const CTRL_BPW_SHIFT: c_int = 10;
pub const PIC32_BPW_8: c_int = 0;
pub const PIC32_BPW_16: c_int = 1;
pub const PIC32_BPW_32: c_int = 2;

// Bit fields of SPI Status Register

pub const STAT_TF_LVL_MASK: c_uint = 0x1F;
pub const STAT_TF_LVL_SHIFT: c_int = 16;
pub const STAT_RF_LVL_MASK: c_uint = 0x1F;
pub const STAT_RF_LVL_SHIFT: c_int = 24;
// Bit fields of SPI Baud Register
pub const BAUD_MASK: c_uint = 0x1ff;
// Bit fields of SPI Control2 Register

// Minimum DMA transfer size
pub const PIC32_DMA_LEN_MIN: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pic32_spi {
    pub dma_base: dma_addr_t,
    pub regs: *mut pic32_spi_regs __iomem,
    pub fault_irq: c_int,
    pub rx_irq: c_int,
    pub tx_irq: c_int,
    pub /: *mut *mut u32 fifo_n_byte; / FIFO depth in bytes,
    pub clk: *mut clk,
    pub host: *mut spi_controller,
// Current controller setting
    pub /: *mut *mut u32 speed_hz; / spi-clk rate,
    pub mode: u32,
    pub bits_per_word: u32,
    pub /: *mut *mut u32 fifo_n_elm; / FIFO depth in words,

    pub flags: c_ulong,
// Current transfer state
    pub xfer_done: completion,
// PIO transfer specific
    pub tx: *const c_void,
    pub tx_end: *const c_void,
    pub rx: *const c_void,
    pub rx_end: *const c_void,
    pub len: c_int,
    pub ): *mut *mut void (rx_fifo)(struct pic32_spi,
    pub ): *mut *mut void (tx_fifo)(struct pic32_spi,
}

#[no_mangle]
pub unsafe extern "C" fn pic32_spi_enable(pic32s: *mut pic32_spi) {
    static inline void pic32_spi_enable(struct pic32_spi *pic32s)
    {
    writel(CTRL_ON | CTRL_SIDL, &pic32s.regs.ctrl_set);
    }
#[no_mangle]
pub unsafe extern "C" fn pic32_spi_disable(pic32s: *mut pic32_spi) {
    static inline void pic32_spi_disable(struct pic32_spi *pic32s)
    {
    writel(CTRL_ON | CTRL_SIDL, &pic32s.regs.ctrl_clr);
// avoid SPI registers read/write at immediate next CPU clock
    ndelay(20);
    }
#[no_mangle]
unsafe extern "C" fn pic32_spi_set_clk_rate(pic32s: *mut pic32_spi, spi_ck: u32) {
    static void pic32_spi_set_clk_rate(struct pic32_spi *pic32s, u32 spi_ck)
    {
    u32 div;
// div = (clk_in / 2 * spi_ck) - 1
    div = DIV_ROUND_CLOSEST(clk_get_rate(pic32s.clk), 2 * spi_ck) - 1;
    writel(div & BAUD_MASK, &pic32s.regs.baud);
    }
#[no_mangle]
pub unsafe extern "C" fn pic32_rx_fifo_level(pic32s: *mut pic32_spi) -> u32 {
    static inline u32 pic32_rx_fifo_level(struct pic32_spi *pic32s)
    {
    let mut sr: u32 = readl(&pic32s.regs.status);
    return (sr >> STAT_RF_LVL_SHIFT) & STAT_RF_LVL_MASK;
    }
#[no_mangle]
pub unsafe extern "C" fn pic32_tx_fifo_level(pic32s: *mut pic32_spi) -> u32 {
    static inline u32 pic32_tx_fifo_level(struct pic32_spi *pic32s)
    {
    let mut sr: u32 = readl(&pic32s.regs.status);
    return (sr >> STAT_TF_LVL_SHIFT) & STAT_TF_LVL_MASK;
    }
// Return the max entries we can fill into tx fifo
#[no_mangle]
unsafe extern "C" fn pic32_tx_max(pic32s: *mut pic32_spi, n_bytes: c_int) -> u32 {
    static u32 pic32_tx_max(struct pic32_spi *pic32s, int n_bytes)
    {
    u32 tx_left, tx_room, rxtx_gap;
    tx_left = (pic32s.tx_end - pic32s.tx) / n_bytes;
    tx_room = pic32s.fifo_n_elm - pic32_tx_fifo_level(pic32s);
//
// Another concern is about the tx/rx mismatch, we
// though to use (pic32s->fifo_n_byte - rxfl - txfl) as
// one maximum value for tx, but it doesn't cover the
// data which is out of tx/rx fifo and inside the
// shift registers. So a ctrl from sw point of
// view is taken.
//
    rxtx_gap = ((pic32s.rx_end - pic32s.rx) -
    (pic32s.tx_end - pic32s.tx)) / n_bytes;
    return min3(tx_left, tx_room, (u32)(pic32s.fifo_n_elm - rxtx_gap));
    }
// Return the max entries we should read out of rx fifo
#[no_mangle]
unsafe extern "C" fn pic32_rx_max(pic32s: *mut pic32_spi, n_bytes: c_int) -> u32 {
    static u32 pic32_rx_max(struct pic32_spi *pic32s, int n_bytes)
    {
    let mut rx_left: u32 = (pic32s.rx_end - pic32s.rx) / n_bytes;
    return min_t(u32, rx_left, pic32_rx_fifo_level(pic32s));
    }

    static void pic32_spi_rx_##__name(struct pic32_spi *pic32s)	\
    {								\
    __type v;						\
    u32 mx = pic32_rx_max(pic32s, sizeof(__type));		\
    for (; mx; mx--) {					\
    v = read##__bwl(&pic32s.regs.buf);		\
    if (pic32s.rx_end - pic32s.len)		\
// (__type *)(pic32s->rx) = v;		\
    pic32s.rx += sizeof(__type);			\
    }							\
    }								\
    \
    static void pic32_spi_tx_##__name(struct pic32_spi *pic32s)	\
    {								\
    __type v;						\
    u32 mx = pic32_tx_max(pic32s, sizeof(__type));		\
    for (; mx ; mx--) {					\
    v = (__type)~0U;				\
    if (pic32s.tx_end - pic32s.len)		\
    v = *(__type *)(pic32s.tx);		\
    write##__bwl(v, &pic32s.regs.buf);		\
    pic32s.tx += sizeof(__type);			\
    }							\
    }
    BUILD_SPI_FIFO_RW(byte, u8, b);
    BUILD_SPI_FIFO_RW(word, u16, w);
    BUILD_SPI_FIFO_RW(dword, u32, l);
#[no_mangle]
unsafe extern "C" fn pic32_err_stop(pic32s: *mut pic32_spi, msg: *const c_char) {
    static void pic32_err_stop(struct pic32_spi *pic32s, const char *msg)
    {
// disable all interrupts
    disable_irq_nosync(pic32s.fault_irq);
    disable_irq_nosync(pic32s.rx_irq);
    disable_irq_nosync(pic32s.tx_irq);
// Show err message and abort xfer with err
    dev_err(&pic32s.host.dev, "%s\n", msg);
    if (pic32s.host.cur_msg)
    pic32s.host.cur_msg.status = -EIO;
    complete(&pic32s.xfer_done);
    }
#[no_mangle]
unsafe extern "C" fn pic32_spi_fault_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pic32_spi_fault_irq(int irq, void *dev_id)
    {
    struct pic32_spi *pic32s = dev_id;
    u32 status;
    status = readl(&pic32s.regs.status);
// Error handling
    if (status & (STAT_RX_OV | STAT_TX_UR)) {
    writel(STAT_RX_OV, &pic32s.regs.status_clr);
    writel(STAT_TX_UR, &pic32s.regs.status_clr);
    pic32_err_stop(pic32s, "err_irq: fifo ov/ur-run\n");
    return IRQ_HANDLED;
    }
    if (status & STAT_FRM_ERR) {
    pic32_err_stop(pic32s, "err_irq: frame error");
    return IRQ_HANDLED;
    }
    if (!pic32s.host.cur_msg) {
    pic32_err_stop(pic32s, "err_irq: no mesg");
    return IRQ_NONE;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn pic32_spi_rx_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pic32_spi_rx_irq(int irq, void *dev_id)
    {
    struct pic32_spi *pic32s = dev_id;
    pic32s.rx_fifo(pic32s);
// rx complete ?
    if (pic32s.rx_end == pic32s.rx) {
// disable all interrupts
    disable_irq_nosync(pic32s.fault_irq);
    disable_irq_nosync(pic32s.rx_irq);
// complete current xfer
    complete(&pic32s.xfer_done);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pic32_spi_tx_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pic32_spi_tx_irq(int irq, void *dev_id)
    {
    struct pic32_spi *pic32s = dev_id;
    pic32s.tx_fifo(pic32s);
// tx complete? disable tx interrupt
    if (pic32s.tx_end == pic32s.tx)
    disable_irq_nosync(pic32s.tx_irq);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pic32_spi_dma_rx_notify(data: *mut c_void) {
    static void pic32_spi_dma_rx_notify(void *data)
    {
    struct pic32_spi *pic32s = data;
    complete(&pic32s.xfer_done);
    }
    static int pic32_spi_dma_transfer(struct pic32_spi *pic32s,
    struct spi_transfer *xfer)
    {
    struct spi_controller *host = pic32s.host;
    struct dma_async_tx_descriptor *desc_rx;
    struct dma_async_tx_descriptor *desc_tx;
    dma_cookie_t cookie;
    int ret;
    if (!host.dma_rx || !host.dma_tx)
    return -ENODEV;
    desc_rx = dmaengine_prep_slave_sg(host.dma_rx,
    xfer.rx_sg.sgl,
    xfer.rx_sg.nents,
    DMA_DEV_TO_MEM,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!desc_rx) {
    ret = -EINVAL;
    goto err_dma;
    }
    desc_tx = dmaengine_prep_slave_sg(host.dma_tx,
    xfer.tx_sg.sgl,
    xfer.tx_sg.nents,
    DMA_MEM_TO_DEV,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!desc_tx) {
    ret = -EINVAL;
    goto err_dma;
    }
// Put callback on the RX transfer, that should finish last
    desc_rx.callback = pic32_spi_dma_rx_notify;
    desc_rx.callback_param = pic32s;
    cookie = dmaengine_submit(desc_rx);
    ret = dma_submit_error(cookie);
    if (ret)
    goto err_dma;
    cookie = dmaengine_submit(desc_tx);
    ret = dma_submit_error(cookie);
    if (ret)
    goto err_dma_tx;
    dma_async_issue_pending(host.dma_rx);
    dma_async_issue_pending(host.dma_tx);
    return 0;
    err_dma_tx:
    dmaengine_terminate_all(host.dma_rx);
    err_dma:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pic32_spi_dma_config(pic32s: *mut pic32_spi, dma_width: u32) -> c_int {
    static int pic32_spi_dma_config(struct pic32_spi *pic32s, u32 dma_width)
    {
    let mut buf_offset: c_int = offsetof(struct pic32_spi_regs, buf);
    struct spi_controller *host = pic32s.host;
    struct dma_slave_config cfg;
    int ret;
    memset(&cfg, 0, sizeof(cfg));
    cfg.device_fc = true;
    cfg.src_addr = pic32s.dma_base + buf_offset;
    cfg.dst_addr = pic32s.dma_base + buf_offset;
    cfg.src_maxburst = pic32s.fifo_n_elm / 2; /* fill one-half */
    cfg.dst_maxburst = pic32s.fifo_n_elm / 2; /* drain one-half */
    cfg.src_addr_width = dma_width;
    cfg.dst_addr_width = dma_width;
// tx channel
    cfg.direction = DMA_MEM_TO_DEV;
    ret = dmaengine_slave_config(host.dma_tx, &cfg);
    if (ret) {
    dev_err(&host.dev, "tx channel setup failed\n");
    return ret;
    }
// rx channel
    cfg.direction = DMA_DEV_TO_MEM;
    ret = dmaengine_slave_config(host.dma_rx, &cfg);
    if (ret)
    dev_err(&host.dev, "rx channel setup failed\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pic32_spi_set_word_size(pic32s: *mut pic32_spi, bits_per_word: u8) -> c_int {
    static int pic32_spi_set_word_size(struct pic32_spi *pic32s, u8 bits_per_word)
    {
    enum dma_slave_buswidth dmawidth;
    u32 buswidth, v;
    switch (bits_per_word) {
    case 8:
    pic32s.rx_fifo = pic32_spi_rx_byte;
    pic32s.tx_fifo = pic32_spi_tx_byte;
    buswidth = PIC32_BPW_8;
    dmawidth = DMA_SLAVE_BUSWIDTH_1_BYTE;
    break;
    case 16:
    pic32s.rx_fifo = pic32_spi_rx_word;
    pic32s.tx_fifo = pic32_spi_tx_word;
    buswidth = PIC32_BPW_16;
    dmawidth = DMA_SLAVE_BUSWIDTH_2_BYTES;
    break;
    case 32:
    pic32s.rx_fifo = pic32_spi_rx_dword;
    pic32s.tx_fifo = pic32_spi_tx_dword;
    buswidth = PIC32_BPW_32;
    dmawidth = DMA_SLAVE_BUSWIDTH_4_BYTES;
    break;
    default:
// not supported
    return -EINVAL;
    }
// calculate maximum number of words fifos can hold
    pic32s.fifo_n_elm = DIV_ROUND_UP(pic32s.fifo_n_byte,
    bits_per_word / 8);
// set word size
    v = readl(&pic32s.regs.ctrl);
    v &= ~(CTRL_BPW_MASK << CTRL_BPW_SHIFT);
    v |= buswidth << CTRL_BPW_SHIFT;
    writel(v, &pic32s.regs.ctrl);
// re-configure dma width, if required
    if (test_bit(PIC32F_DMA_PREP, &pic32s.flags))
    pic32_spi_dma_config(pic32s, dmawidth);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pic32_spi_prepare_hardware(host: *mut spi_controller) -> c_int {
    static int pic32_spi_prepare_hardware(struct spi_controller *host)
    {
    struct pic32_spi *pic32s = spi_controller_get_devdata(host);
    pic32_spi_enable(pic32s);
    return 0;
    }
    static int pic32_spi_prepare_message(struct spi_controller *host,
    struct spi_message *msg)
    {
    struct pic32_spi *pic32s = spi_controller_get_devdata(host);
    struct spi_device *spi = msg.spi;
    u32 val;
// set device specific bits_per_word
    if (pic32s.bits_per_word != spi.bits_per_word) {
    pic32_spi_set_word_size(pic32s, spi.bits_per_word);
    pic32s.bits_per_word = spi.bits_per_word;
    }
// device specific speed change
    if (pic32s.speed_hz != spi.max_speed_hz) {
    pic32_spi_set_clk_rate(pic32s, spi.max_speed_hz);
    pic32s.speed_hz = spi.max_speed_hz;
    }
// device specific mode change
    if (pic32s.mode != spi.mode) {
    val = readl(&pic32s.regs.ctrl);
// active low
    if (spi.mode & SPI_CPOL)
    val |= CTRL_CKP;
    else
    val &= ~CTRL_CKP;
// tx on rising edge
    if (spi.mode & SPI_CPHA)
    val &= ~CTRL_CKE;
    else
    val |= CTRL_CKE;
// rx at end of tx
    val |= CTRL_SMP;
    writel(val, &pic32s.regs.ctrl);
    pic32s.mode = spi.mode;
    }
    return 0;
    }
    static bool pic32_spi_can_dma(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct pic32_spi *pic32s = spi_controller_get_devdata(host);
// skip using DMA on small size transfer to avoid overhead.
    return (xfer.len >= PIC32_DMA_LEN_MIN) &&
    test_bit(PIC32F_DMA_PREP, &pic32s.flags);
    }
    static int pic32_spi_one_transfer(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *transfer)
    {
    struct pic32_spi *pic32s;
    let mut dma_issued: bool = false;
    unsigned long time_left;
    int ret;
    pic32s = spi_controller_get_devdata(host);
// handle transfer specific word size change
    if (transfer.bits_per_word &&
    (transfer.bits_per_word != pic32s.bits_per_word)) {
    ret = pic32_spi_set_word_size(pic32s, transfer.bits_per_word);
    if (ret)
    return ret;
    pic32s.bits_per_word = transfer.bits_per_word;
    }
// handle transfer specific speed change
    if (transfer.speed_hz && (transfer.speed_hz != pic32s.speed_hz)) {
    pic32_spi_set_clk_rate(pic32s, transfer.speed_hz);
    pic32s.speed_hz = transfer.speed_hz;
    }
    reinit_completion(&pic32s.xfer_done);
// transact by DMA mode
    if (transfer.rx_sg.nents && transfer.tx_sg.nents) {
    ret = pic32_spi_dma_transfer(pic32s, transfer);
    if (ret) {
    dev_err(&spi.dev, "dma submit error\n");
    return ret;
    }
// DMA issued
    dma_issued = true;
    } else {
// set current transfer information
    pic32s.tx = (const void *)transfer.tx_buf;
    pic32s.rx = (const void *)transfer.rx_buf;
    pic32s.tx_end = pic32s.tx + transfer.len;
    pic32s.rx_end = pic32s.rx + transfer.len;
    pic32s.len = transfer.len;
// transact by interrupt driven PIO
    enable_irq(pic32s.fault_irq);
    enable_irq(pic32s.rx_irq);
    enable_irq(pic32s.tx_irq);
    }
// wait for completion
    time_left = wait_for_completion_timeout(&pic32s.xfer_done, 2 * HZ);
    if (time_left == 0) {
    dev_err(&spi.dev, "wait error/timedout\n");
    if (dma_issued) {
    dmaengine_terminate_all(host.dma_rx);
    dmaengine_terminate_all(host.dma_tx);
    }
    ret = -ETIMEDOUT;
    } else {
    ret = 0;
    }
    return ret;
    }
    static int pic32_spi_unprepare_message(struct spi_controller *host,
    struct spi_message *msg)
    {
// nothing to do
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pic32_spi_unprepare_hardware(host: *mut spi_controller) -> c_int {
    static int pic32_spi_unprepare_hardware(struct spi_controller *host)
    {
    struct pic32_spi *pic32s = spi_controller_get_devdata(host);
    pic32_spi_disable(pic32s);
    return 0;
    }
// This may be called multiple times by same spi dev
#[no_mangle]
unsafe extern "C" fn pic32_spi_setup(spi: *mut spi_device) -> c_int {
    static int pic32_spi_setup(struct spi_device *spi)
    {
    if (!spi.max_speed_hz) {
    dev_err(&spi.dev, "No max speed HZ parameter\n");
    return -EINVAL;
    }
// PIC32 spi controller can drive /CS during transfer depending
// on tx fifo fill-level. /CS will stay asserted as long as TX
// fifo is non-empty, else will be deasserted indicating
// completion of the ongoing transfer. This might result into
// unreliable/erroneous SPI transactions.
// To avoid that we will always handle /CS by toggling GPIO.
//
    if (!spi_get_csgpiod(spi, 0))
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pic32_spi_cleanup(spi: *mut spi_device) {
    static void pic32_spi_cleanup(struct spi_device *spi)
    {
// de-activate cs-gpio, gpiolib will handle inversion
    gpiod_direction_output(spi_get_csgpiod(spi, 0), 0);
    }
#[no_mangle]
unsafe extern "C" fn pic32_spi_dma_prep(pic32s: *mut pic32_spi, dev: *mut device) -> c_int {
    static int pic32_spi_dma_prep(struct pic32_spi *pic32s, struct device *dev)
    {
    struct spi_controller *host = pic32s.host;
    let mut ret: c_int = 0;
    host.dma_rx = dma_request_chan(dev, "spi-rx");
    if (IS_ERR(host.dma_rx)) {
    if (PTR_ERR(host.dma_rx) == -EPROBE_DEFER)
    ret = -EPROBE_DEFER;
    else
    dev_warn(dev, "RX channel not found.\n");
    host.dma_rx = core::ptr::null_mut();
    goto out_err;
    }
    host.dma_tx = dma_request_chan(dev, "spi-tx");
    if (IS_ERR(host.dma_tx)) {
    if (PTR_ERR(host.dma_tx) == -EPROBE_DEFER)
    ret = -EPROBE_DEFER;
    else
    dev_warn(dev, "TX channel not found.\n");
    host.dma_tx = core::ptr::null_mut();
    goto out_err;
    }
    if (pic32_spi_dma_config(pic32s, DMA_SLAVE_BUSWIDTH_1_BYTE))
    goto out_err;
// DMA chnls allocated and prepared
    set_bit(PIC32F_DMA_PREP, &pic32s.flags);
    return 0;
    out_err:
    if (host.dma_rx) {
    dma_release_channel(host.dma_rx);
    host.dma_rx = core::ptr::null_mut();
    }
    if (host.dma_tx) {
    dma_release_channel(host.dma_tx);
    host.dma_tx = core::ptr::null_mut();
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pic32_spi_dma_unprep(pic32s: *mut pic32_spi) {
    static void pic32_spi_dma_unprep(struct pic32_spi *pic32s)
    {
    if (!test_bit(PIC32F_DMA_PREP, &pic32s.flags))
    return;
    clear_bit(PIC32F_DMA_PREP, &pic32s.flags);
    if (pic32s.host.dma_rx)
    dma_release_channel(pic32s.host.dma_rx);
    if (pic32s.host.dma_tx)
    dma_release_channel(pic32s.host.dma_tx);
    }
#[no_mangle]
unsafe extern "C" fn pic32_spi_hw_init(pic32s: *mut pic32_spi) {
    static void pic32_spi_hw_init(struct pic32_spi *pic32s)
    {
    u32 ctrl;
// disable hardware
    pic32_spi_disable(pic32s);
    ctrl = readl(&pic32s.regs.ctrl);
// enable enhanced fifo of 128bit deep
    ctrl |= CTRL_ENHBUF;
    pic32s.fifo_n_byte = 16;
// disable framing mode
    ctrl &= ~CTRL_FRMEN;
// enable host mode while disabled
    ctrl |= CTRL_MSTEN;
// set tx fifo threshold interrupt
    ctrl &= ~(0x3 << CTRL_TX_INT_SHIFT);
    ctrl |= (TX_FIFO_HALF_EMPTY << CTRL_TX_INT_SHIFT);
// set rx fifo threshold interrupt
    ctrl &= ~(0x3 << CTRL_RX_INT_SHIFT);
    ctrl |= (RX_FIFO_NOT_EMPTY << CTRL_RX_INT_SHIFT);
// select clk source
    ctrl &= ~CTRL_MCLKSEL;
// set manual /CS mode
    ctrl &= ~CTRL_MSSEN;
    writel(ctrl, &pic32s.regs.ctrl);
// enable error reporting
    ctrl = CTRL2_TX_UR_EN | CTRL2_RX_OV_EN | CTRL2_FRM_ERR_EN;
    writel(ctrl, &pic32s.regs.ctrl2_set);
    }
    static int pic32_spi_hw_probe(struct platform_device *pdev,
    struct pic32_spi *pic32s)
    {
    struct resource *mem;
    int ret;
    pic32s.regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mem);
    if (IS_ERR(pic32s.regs))
    return PTR_ERR(pic32s.regs);
    pic32s.dma_base = mem.start;
// get irq resources: err-irq, rx-irq, tx-irq
    pic32s.fault_irq = platform_get_irq_byname(pdev, "fault");
    if (pic32s.fault_irq < 0)
    return pic32s.fault_irq;
    pic32s.rx_irq = platform_get_irq_byname(pdev, "rx");
    if (pic32s.rx_irq < 0)
    return pic32s.rx_irq;
    pic32s.tx_irq = platform_get_irq_byname(pdev, "tx");
    if (pic32s.tx_irq < 0)
    return pic32s.tx_irq;
// get clock
    pic32s.clk = devm_clk_get_enabled(&pdev.dev, "mck0");
    if (IS_ERR(pic32s.clk)) {
    dev_err(&pdev.dev, "clk not found\n");
    ret = PTR_ERR(pic32s.clk);
    goto err_unmap_mem;
    }
    pic32_spi_hw_init(pic32s);
    return 0;
    err_unmap_mem:
    dev_err(&pdev.dev, "%s failed, err %d\n", __func__, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pic32_spi_probe(pdev: *mut platform_device) -> c_int {
    static int pic32_spi_probe(struct platform_device *pdev)
    {
    struct spi_controller *host;
    struct pic32_spi *pic32s;
    int ret;
    host = devm_spi_alloc_host(&pdev.dev, sizeof(*pic32s));
    if (!host)
    return -ENOMEM;
    pic32s = spi_controller_get_devdata(host);
    pic32s.host = host;
    ret = pic32_spi_hw_probe(pdev, pic32s);
    if (ret)
    return ret;
    host.dev.of_node	= pdev.dev.of_node;
    host.mode_bits	= SPI_MODE_3 | SPI_MODE_0 | SPI_CS_HIGH;
    host.num_chipselect	= 1; /* single chip-select */
    host.max_speed_hz	= clk_get_rate(pic32s.clk);
    host.setup		= pic32_spi_setup;
    host.cleanup		= pic32_spi_cleanup;
    host.flags		= SPI_CONTROLLER_MUST_TX | SPI_CONTROLLER_MUST_RX;
    host.bits_per_word_mask	= SPI_BPW_MASK(8) | SPI_BPW_MASK(16) |
    SPI_BPW_MASK(32);
    host.transfer_one		= pic32_spi_one_transfer;
    host.prepare_message		= pic32_spi_prepare_message;
    host.unprepare_message	= pic32_spi_unprepare_message;
    host.prepare_transfer_hardware	= pic32_spi_prepare_hardware;
    host.unprepare_transfer_hardware	= pic32_spi_unprepare_hardware;
    host.use_gpio_descriptors = true;
// optional DMA support
    ret = pic32_spi_dma_prep(pic32s, &pdev.dev);
    if (ret)
    goto err_bailout;
    if (test_bit(PIC32F_DMA_PREP, &pic32s.flags))
    host.can_dma	= pic32_spi_can_dma;
    init_completion(&pic32s.xfer_done);
    pic32s.mode = -1;
// install irq handlers (with irq-disabled)
    irq_set_status_flags(pic32s.fault_irq, IRQ_NOAUTOEN);
    ret = devm_request_irq(&pdev.dev, pic32s.fault_irq,
    pic32_spi_fault_irq, IRQF_NO_THREAD,
    dev_name(&pdev.dev), pic32s);
    if (ret < 0) {
    dev_err(&pdev.dev, "request fault-irq %d\n", pic32s.rx_irq);
    goto err_bailout;
    }
// receive interrupt handler
    irq_set_status_flags(pic32s.rx_irq, IRQ_NOAUTOEN);
    ret = devm_request_irq(&pdev.dev, pic32s.rx_irq,
    pic32_spi_rx_irq, IRQF_NO_THREAD,
    dev_name(&pdev.dev), pic32s);
    if (ret < 0) {
    dev_err(&pdev.dev, "request rx-irq %d\n", pic32s.rx_irq);
    goto err_bailout;
    }
// transmit interrupt handler
    irq_set_status_flags(pic32s.tx_irq, IRQ_NOAUTOEN);
    ret = devm_request_irq(&pdev.dev, pic32s.tx_irq,
    pic32_spi_tx_irq, IRQF_NO_THREAD,
    dev_name(&pdev.dev), pic32s);
    if (ret < 0) {
    dev_err(&pdev.dev, "request tx-irq %d\n", pic32s.tx_irq);
    goto err_bailout;
    }
// register host
    ret = spi_register_controller(host);
    if (ret) {
    dev_err(&host.dev, "failed registering spi host\n");
    goto err_bailout;
    }
    platform_set_drvdata(pdev, pic32s);
    return 0;
    err_bailout:
    pic32_spi_dma_unprep(pic32s);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pic32_spi_remove(pdev: *mut platform_device) {
    static void pic32_spi_remove(struct platform_device *pdev)
    {
    struct pic32_spi *pic32s = platform_get_drvdata(pdev);
    spi_unregister_controller(pic32s.host);
    pic32_spi_disable(pic32s);
    pic32_spi_dma_unprep(pic32s);
    }
    static const struct of_device_id pic32_spi_of_match[] = {
    {.compatible = "microchip,pic32mzda-spi",},
    {},
    };
    MODULE_DEVICE_TABLE(of, pic32_spi_of_match);
    static struct platform_driver pic32_spi_driver = {
    .driver = {
    .name = "spi-pic32",
    .of_match_table = of_match_ptr(pic32_spi_of_match),
    },
    .probe = pic32_spi_probe,
    .remove = pic32_spi_remove,
    };
    module_platform_driver(pic32_spi_driver);
    MODULE_AUTHOR("Purna Chandra Mandal <purna.mandal@microchip.com>");
    MODULE_DESCRIPTION("Microchip SPI driver for PIC32 SPI controller.");
    MODULE_LICENSE("GPL v2");
