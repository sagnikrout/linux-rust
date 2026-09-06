//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-img-spfi.c
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
// IMG SPFI controller driver
//
// Copyright (C) 2007,2008,2013 Imagination Technologies Ltd.
// Copyright (C) 2014 Google, Inc.
//

pub const SPFI_DEVICE_PARAMETER_BITCLK_SHIFT: c_int = 24;
pub const SPFI_DEVICE_PARAMETER_BITCLK_MASK: c_uint = 0xff;
pub const SPFI_DEVICE_PARAMETER_CSSETUP_SHIFT: c_int = 16;
pub const SPFI_DEVICE_PARAMETER_CSSETUP_MASK: c_uint = 0xff;
pub const SPFI_DEVICE_PARAMETER_CSHOLD_SHIFT: c_int = 8;
pub const SPFI_DEVICE_PARAMETER_CSHOLD_MASK: c_uint = 0xff;
pub const SPFI_DEVICE_PARAMETER_CSDELAY_SHIFT: c_int = 0;
pub const SPFI_DEVICE_PARAMETER_CSDELAY_MASK: c_uint = 0xff;
pub const SPFI_CONTROL: c_uint = 0x14;

pub const SPFI_CONTROL_TMODE_SHIFT: c_int = 5;
pub const SPFI_CONTROL_TMODE_MASK: c_uint = 0x7;
pub const SPFI_CONTROL_TMODE_SINGLE: c_int = 0;
pub const SPFI_CONTROL_TMODE_DUAL: c_int = 1;
pub const SPFI_CONTROL_TMODE_QUAD: c_int = 2;

pub const SPFI_TRANSACTION: c_uint = 0x18;
pub const SPFI_TRANSACTION_TSIZE_SHIFT: c_int = 16;
pub const SPFI_TRANSACTION_TSIZE_MASK: c_uint = 0xffff;
pub const SPFI_PORT_STATE: c_uint = 0x1c;
pub const SPFI_PORT_STATE_DEV_SEL_SHIFT: c_int = 20;
pub const SPFI_PORT_STATE_DEV_SEL_MASK: c_uint = 0x7;

pub const SPFI_TX_32BIT_VALID_DATA: c_uint = 0x20;
pub const SPFI_TX_8BIT_VALID_DATA: c_uint = 0x24;
pub const SPFI_RX_32BIT_VALID_DATA: c_uint = 0x28;
pub const SPFI_RX_8BIT_VALID_DATA: c_uint = 0x2c;
pub const SPFI_INTERRUPT_STATUS: c_uint = 0x30;
pub const SPFI_INTERRUPT_ENABLE: c_uint = 0x34;
pub const SPFI_INTERRUPT_CLEAR: c_uint = 0x38;

//
// There are four parallel FIFOs of 16 bytes each.  The word buffer
// (*_32BIT_VALID_DATA) accesses all four FIFOs at once, resulting in an
// effective FIFO size of 64 bytes.  The byte buffer (*_8BIT_VALID_DATA)
// accesses only a single FIFO, resulting in an effective FIFO size of
// 16 bytes.
//
pub const SPFI_32BIT_FIFO_SIZE: c_int = 64;
pub const SPFI_8BIT_FIFO_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_spfi {
    pub dev: *mut device,
    pub host: *mut spi_controller,
    pub lock: spinlock_t,
    pub regs: *mut void __iomem,
    pub phys: phys_addr_t,
    pub irq: c_int,
    pub spfi_clk: *mut clk,
    pub sys_clk: *mut clk,
    pub rx_ch: *mut dma_chan,
    pub tx_ch: *mut dma_chan,
    pub tx_dma_busy: bool,
    pub rx_dma_busy: bool,
}

#[no_mangle]
pub unsafe extern "C" fn spfi_readl(spfi: *mut img_spfi, reg: u32) -> u32 {
    static inline u32 spfi_readl(struct img_spfi *spfi, u32 reg)
    {
    return readl(spfi.regs + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn spfi_writel(spfi: *mut img_spfi, val: u32, reg: u32) {
    static inline void spfi_writel(struct img_spfi *spfi, u32 val, u32 reg)
    {
    writel(val, spfi.regs + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn spfi_start(spfi: *mut img_spfi) {
    static inline void spfi_start(struct img_spfi *spfi)
    {
    u32 val;
    val = spfi_readl(spfi, SPFI_CONTROL);
    val |= SPFI_CONTROL_SPFI_EN;
    spfi_writel(spfi, val, SPFI_CONTROL);
    }
#[no_mangle]
pub unsafe extern "C" fn spfi_reset(spfi: *mut img_spfi) {
    static inline void spfi_reset(struct img_spfi *spfi)
    {
    spfi_writel(spfi, SPFI_CONTROL_SOFT_RESET, SPFI_CONTROL);
    spfi_writel(spfi, 0, SPFI_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn spfi_wait_all_done(spfi: *mut img_spfi) -> c_int {
    static int spfi_wait_all_done(struct img_spfi *spfi)
    {
    let mut timeout: c_ulong = jiffies + msecs_to_jiffies(50);
    while (time_before(jiffies, timeout)) {
    let mut status: u32 = spfi_readl(spfi, SPFI_INTERRUPT_STATUS);
    if (status & SPFI_INTERRUPT_ALLDONETRIG) {
    spfi_writel(spfi, SPFI_INTERRUPT_ALLDONETRIG,
    SPFI_INTERRUPT_CLEAR);
    return 0;
    }
    cpu_relax();
    }
    dev_err(spfi.dev, "Timed out waiting for transaction to complete\n");
    spfi_reset(spfi);
    return -ETIMEDOUT;
    }
    static unsigned int spfi_pio_write32(struct img_spfi *spfi, const u32 *buf,
    unsigned int max)
    {
    let mut count: c_uint = 0;
    u32 status;
    while (count < max / 4) {
    spfi_writel(spfi, SPFI_INTERRUPT_SDFUL, SPFI_INTERRUPT_CLEAR);
    status = spfi_readl(spfi, SPFI_INTERRUPT_STATUS);
    if (status & SPFI_INTERRUPT_SDFUL)
    break;
    spfi_writel(spfi, buf[count], SPFI_TX_32BIT_VALID_DATA);
    count++;
    }
    return count * 4;
    }
    static unsigned int spfi_pio_write8(struct img_spfi *spfi, const u8 *buf,
    unsigned int max)
    {
    let mut count: c_uint = 0;
    u32 status;
    while (count < max) {
    spfi_writel(spfi, SPFI_INTERRUPT_SDFUL, SPFI_INTERRUPT_CLEAR);
    status = spfi_readl(spfi, SPFI_INTERRUPT_STATUS);
    if (status & SPFI_INTERRUPT_SDFUL)
    break;
    spfi_writel(spfi, buf[count], SPFI_TX_8BIT_VALID_DATA);
    count++;
    }
    return count;
    }
    static unsigned int spfi_pio_read32(struct img_spfi *spfi, u32 *buf,
    unsigned int max)
    {
    let mut count: c_uint = 0;
    u32 status;
    while (count < max / 4) {
    spfi_writel(spfi, SPFI_INTERRUPT_GDEX32BIT,
    SPFI_INTERRUPT_CLEAR);
    status = spfi_readl(spfi, SPFI_INTERRUPT_STATUS);
    if (!(status & SPFI_INTERRUPT_GDEX32BIT))
    break;
    buf[count] = spfi_readl(spfi, SPFI_RX_32BIT_VALID_DATA);
    count++;
    }
    return count * 4;
    }
    static unsigned int spfi_pio_read8(struct img_spfi *spfi, u8 *buf,
    unsigned int max)
    {
    let mut count: c_uint = 0;
    u32 status;
    while (count < max) {
    spfi_writel(spfi, SPFI_INTERRUPT_GDEX8BIT,
    SPFI_INTERRUPT_CLEAR);
    status = spfi_readl(spfi, SPFI_INTERRUPT_STATUS);
    if (!(status & SPFI_INTERRUPT_GDEX8BIT))
    break;
    buf[count] = spfi_readl(spfi, SPFI_RX_8BIT_VALID_DATA);
    count++;
    }
    return count;
    }
    static int img_spfi_start_pio(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct img_spfi *spfi = spi_controller_get_devdata(spi.controller);
    let mut tx_bytes: c_uint = 0, rx_bytes = 0;
    const void *tx_buf = xfer.tx_buf;
    void *rx_buf = xfer.rx_buf;
    unsigned long timeout;
    int ret;
    if (tx_buf)
    tx_bytes = xfer.len;
    if (rx_buf)
    rx_bytes = xfer.len;
    spfi_start(spfi);
    timeout = jiffies +
    msecs_to_jiffies(xfer.len * 8 * 1000 / xfer.speed_hz + 100);
    while ((tx_bytes > 0 || rx_bytes > 0) &&
    time_before(jiffies, timeout)) {
    unsigned int tx_count, rx_count;
    if (tx_bytes >= 4)
    tx_count = spfi_pio_write32(spfi, tx_buf, tx_bytes);
    else
    tx_count = spfi_pio_write8(spfi, tx_buf, tx_bytes);
    if (rx_bytes >= 4)
    rx_count = spfi_pio_read32(spfi, rx_buf, rx_bytes);
    else
    rx_count = spfi_pio_read8(spfi, rx_buf, rx_bytes);
    tx_buf += tx_count;
    rx_buf += rx_count;
    tx_bytes -= tx_count;
    rx_bytes -= rx_count;
    cpu_relax();
    }
    if (rx_bytes > 0 || tx_bytes > 0) {
    dev_err(spfi.dev, "PIO transfer timed out\n");
    return -ETIMEDOUT;
    }
    ret = spfi_wait_all_done(spfi);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_spfi_dma_rx_cb(data: *mut c_void) {
    static void img_spfi_dma_rx_cb(void *data)
    {
    struct img_spfi *spfi = data;
    unsigned long flags;
    spfi_wait_all_done(spfi);
    spin_lock_irqsave(&spfi.lock, flags);
    spfi.rx_dma_busy = false;
    if (!spfi.tx_dma_busy)
    spi_finalize_current_transfer(spfi.host);
    spin_unlock_irqrestore(&spfi.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn img_spfi_dma_tx_cb(data: *mut c_void) {
    static void img_spfi_dma_tx_cb(void *data)
    {
    struct img_spfi *spfi = data;
    unsigned long flags;
    spfi_wait_all_done(spfi);
    spin_lock_irqsave(&spfi.lock, flags);
    spfi.tx_dma_busy = false;
    if (!spfi.rx_dma_busy)
    spi_finalize_current_transfer(spfi.host);
    spin_unlock_irqrestore(&spfi.lock, flags);
    }
    static int img_spfi_start_dma(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct img_spfi *spfi = spi_controller_get_devdata(spi.controller);
    struct dma_async_tx_descriptor *rxdesc = core::ptr::null_mut(), *txdesc = core::ptr::null_mut();
    struct dma_slave_config rxconf, txconf;
    spfi.rx_dma_busy = false;
    spfi.tx_dma_busy = false;
    if (xfer.rx_buf) {
    rxconf.direction = DMA_DEV_TO_MEM;
    if (xfer.len % 4 == 0) {
    rxconf.src_addr = spfi.phys + SPFI_RX_32BIT_VALID_DATA;
    rxconf.src_addr_width = 4;
    rxconf.src_maxburst = 4;
    } else {
    rxconf.src_addr = spfi.phys + SPFI_RX_8BIT_VALID_DATA;
    rxconf.src_addr_width = 1;
    rxconf.src_maxburst = 4;
    }
    dmaengine_slave_config(spfi.rx_ch, &rxconf);
    rxdesc = dmaengine_prep_slave_sg(spfi.rx_ch, xfer.rx_sg.sgl,
    xfer.rx_sg.nents,
    DMA_DEV_TO_MEM,
    DMA_PREP_INTERRUPT);
    if (!rxdesc)
    goto stop_dma;
    rxdesc.callback = img_spfi_dma_rx_cb;
    rxdesc.callback_param = spfi;
    }
    if (xfer.tx_buf) {
    txconf.direction = DMA_MEM_TO_DEV;
    if (xfer.len % 4 == 0) {
    txconf.dst_addr = spfi.phys + SPFI_TX_32BIT_VALID_DATA;
    txconf.dst_addr_width = 4;
    txconf.dst_maxburst = 4;
    } else {
    txconf.dst_addr = spfi.phys + SPFI_TX_8BIT_VALID_DATA;
    txconf.dst_addr_width = 1;
    txconf.dst_maxburst = 4;
    }
    dmaengine_slave_config(spfi.tx_ch, &txconf);
    txdesc = dmaengine_prep_slave_sg(spfi.tx_ch, xfer.tx_sg.sgl,
    xfer.tx_sg.nents,
    DMA_MEM_TO_DEV,
    DMA_PREP_INTERRUPT);
    if (!txdesc)
    goto stop_dma;
    txdesc.callback = img_spfi_dma_tx_cb;
    txdesc.callback_param = spfi;
    }
    if (xfer.rx_buf) {
    spfi.rx_dma_busy = true;
    dmaengine_submit(rxdesc);
    dma_async_issue_pending(spfi.rx_ch);
    }
    spfi_start(spfi);
    if (xfer.tx_buf) {
    spfi.tx_dma_busy = true;
    dmaengine_submit(txdesc);
    dma_async_issue_pending(spfi.tx_ch);
    }
    return 1;
    stop_dma:
    dmaengine_terminate_all(spfi.rx_ch);
    dmaengine_terminate_all(spfi.tx_ch);
    return -EIO;
    }
    static void img_spfi_handle_err(struct spi_controller *host,
    struct spi_message *msg)
    {
    struct img_spfi *spfi = spi_controller_get_devdata(host);
    unsigned long flags;
//
// Stop all DMA and reset the controller if the previous transaction
// timed-out and never completed it's DMA.
//
    spin_lock_irqsave(&spfi.lock, flags);
    if (spfi.tx_dma_busy || spfi.rx_dma_busy) {
    spfi.tx_dma_busy = false;
    spfi.rx_dma_busy = false;
    dmaengine_terminate_all(spfi.tx_ch);
    dmaengine_terminate_all(spfi.rx_ch);
    }
    spin_unlock_irqrestore(&spfi.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn img_spfi_prepare(host: *mut spi_controller, msg: *mut spi_message) -> c_int {
    static int img_spfi_prepare(struct spi_controller *host, struct spi_message *msg)
    {
    struct img_spfi *spfi = spi_controller_get_devdata(host);
    u32 val;
    val = spfi_readl(spfi, SPFI_PORT_STATE);
    val &= ~(SPFI_PORT_STATE_DEV_SEL_MASK <<
    SPFI_PORT_STATE_DEV_SEL_SHIFT);
    val |= spi_get_chipselect(msg.spi, 0) << SPFI_PORT_STATE_DEV_SEL_SHIFT;
    if (msg.spi.mode & SPI_CPHA)
    val |= SPFI_PORT_STATE_CK_PHASE(spi_get_chipselect(msg.spi, 0));
    else
    val &= ~SPFI_PORT_STATE_CK_PHASE(spi_get_chipselect(msg.spi, 0));
    if (msg.spi.mode & SPI_CPOL)
    val |= SPFI_PORT_STATE_CK_POL(spi_get_chipselect(msg.spi, 0));
    else
    val &= ~SPFI_PORT_STATE_CK_POL(spi_get_chipselect(msg.spi, 0));
    spfi_writel(spfi, val, SPFI_PORT_STATE);
    return 0;
    }
    static int img_spfi_unprepare(struct spi_controller *host,
    struct spi_message *msg)
    {
    struct img_spfi *spfi = spi_controller_get_devdata(host);
    spfi_reset(spfi);
    return 0;
    }
    static void img_spfi_config(struct spi_controller *host, struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct img_spfi *spfi = spi_controller_get_devdata(spi.controller);
    u32 val, div;
//
// output = spfi_clk * (BITCLK / 512), where BITCLK must be a
// power of 2 up to 128
//
    div = DIV_ROUND_UP(clk_get_rate(spfi.spfi_clk), xfer.speed_hz);
    div = clamp(512 / (1 << get_count_order(div)), 1, 128);
    val = spfi_readl(spfi, SPFI_DEVICE_PARAMETER(spi_get_chipselect(spi, 0)));
    val &= ~(SPFI_DEVICE_PARAMETER_BITCLK_MASK <<
    SPFI_DEVICE_PARAMETER_BITCLK_SHIFT);
    val |= div << SPFI_DEVICE_PARAMETER_BITCLK_SHIFT;
    spfi_writel(spfi, val, SPFI_DEVICE_PARAMETER(spi_get_chipselect(spi, 0)));
    spfi_writel(spfi, xfer.len << SPFI_TRANSACTION_TSIZE_SHIFT,
    SPFI_TRANSACTION);
    val = spfi_readl(spfi, SPFI_CONTROL);
    val &= ~(SPFI_CONTROL_SEND_DMA | SPFI_CONTROL_GET_DMA);
    if (xfer.tx_buf)
    val |= SPFI_CONTROL_SEND_DMA;
    if (xfer.rx_buf)
    val |= SPFI_CONTROL_GET_DMA;
    val &= ~(SPFI_CONTROL_TMODE_MASK << SPFI_CONTROL_TMODE_SHIFT);
    if (xfer.tx_nbits == SPI_NBITS_DUAL &&
    xfer.rx_nbits == SPI_NBITS_DUAL)
    val |= SPFI_CONTROL_TMODE_DUAL << SPFI_CONTROL_TMODE_SHIFT;
    else if (xfer.tx_nbits == SPI_NBITS_QUAD &&
    xfer.rx_nbits == SPI_NBITS_QUAD)
    val |= SPFI_CONTROL_TMODE_QUAD << SPFI_CONTROL_TMODE_SHIFT;
    val |= SPFI_CONTROL_SE;
    spfi_writel(spfi, val, SPFI_CONTROL);
    }
    static int img_spfi_transfer_one(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct img_spfi *spfi = spi_controller_get_devdata(spi.controller);
    int ret;
    if (xfer.len > SPFI_TRANSACTION_TSIZE_MASK) {
    dev_err(spfi.dev,
    "Transfer length (%d) is greater than the max supported (%d)",
    xfer.len, SPFI_TRANSACTION_TSIZE_MASK);
    return -EINVAL;
    }
    img_spfi_config(host, spi, xfer);
    if (host.can_dma && host.can_dma(host, spi, xfer))
    ret = img_spfi_start_dma(host, spi, xfer);
    else
    ret = img_spfi_start_pio(host, spi, xfer);
    return ret;
    }
    static bool img_spfi_can_dma(struct spi_controller *host, struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    if (xfer.len > SPFI_32BIT_FIFO_SIZE)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn img_spfi_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t img_spfi_irq(int irq, void *dev_id)
    {
    struct img_spfi *spfi = (struct img_spfi *)dev_id;
    u32 status;
    status = spfi_readl(spfi, SPFI_INTERRUPT_STATUS);
    if (status & SPFI_INTERRUPT_IACCESS) {
    spfi_writel(spfi, SPFI_INTERRUPT_IACCESS, SPFI_INTERRUPT_CLEAR);
    dev_err(spfi.dev, "Illegal access interrupt");
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn img_spfi_probe(pdev: *mut platform_device) -> c_int {
    static int img_spfi_probe(struct platform_device *pdev)
    {
    struct spi_controller *host;
    struct img_spfi *spfi;
    struct resource *res;
    int ret;
    u32 max_speed_hz;
    host = devm_spi_alloc_host(&pdev.dev, sizeof(*spfi));
    if (!host)
    return -ENOMEM;
    platform_set_drvdata(pdev, host);
    spfi = spi_controller_get_devdata(host);
    spfi.dev = &pdev.dev;
    spfi.host = host;
    spin_lock_init(&spfi.lock);
    spfi.regs = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(spfi.regs))
    return PTR_ERR(spfi.regs);
    spfi.phys = res.start;
    spfi.irq = platform_get_irq(pdev, 0);
    if (spfi.irq < 0)
    return spfi.irq;
    ret = devm_request_irq(spfi.dev, spfi.irq, img_spfi_irq,
    IRQ_TYPE_LEVEL_HIGH, dev_name(spfi.dev), spfi);
    if (ret)
    return ret;
    spfi.sys_clk = devm_clk_get(spfi.dev, "sys");
    if (IS_ERR(spfi.sys_clk))
    return PTR_ERR(spfi.sys_clk);
    spfi.spfi_clk = devm_clk_get(spfi.dev, "spfi");
    if (IS_ERR(spfi.spfi_clk))
    return PTR_ERR(spfi.spfi_clk);
    ret = clk_prepare_enable(spfi.sys_clk);
    if (ret)
    return ret;
    ret = clk_prepare_enable(spfi.spfi_clk);
    if (ret)
    goto disable_pclk;
    spfi_reset(spfi);
//
// Only enable the error (IACCESS) interrupt.  In PIO mode we'll
// poll the status of the FIFOs.
//
    spfi_writel(spfi, SPFI_INTERRUPT_IACCESS, SPFI_INTERRUPT_ENABLE);
    host.auto_runtime_pm = true;
    host.bus_num = pdev.id;
    host.mode_bits = SPI_CPOL | SPI_CPHA | SPI_TX_DUAL | SPI_RX_DUAL;
    if (of_property_read_bool(spfi.dev.of_node, "img,supports-quad-mode"))
    host.mode_bits |= SPI_TX_QUAD | SPI_RX_QUAD;
    host.bits_per_word_mask = SPI_BPW_MASK(32) | SPI_BPW_MASK(8);
    host.max_speed_hz = clk_get_rate(spfi.spfi_clk) / 4;
    host.min_speed_hz = clk_get_rate(spfi.spfi_clk) / 512;
//
// Maximum speed supported by spfi is limited to the lower value
// between 1/4 of the SPFI clock or to "spfi-max-frequency"
// defined in the device tree.
// If no value is defined in the device tree assume the maximum
// speed supported to be 1/4 of the SPFI clock.
//
    if (!of_property_read_u32(spfi.dev.of_node, "spfi-max-frequency",
    &max_speed_hz)) {
    if (host.max_speed_hz > max_speed_hz)
    host.max_speed_hz = max_speed_hz;
    }
    host.transfer_one = img_spfi_transfer_one;
    host.prepare_message = img_spfi_prepare;
    host.unprepare_message = img_spfi_unprepare;
    host.handle_err = img_spfi_handle_err;
    host.use_gpio_descriptors = true;
    spfi.tx_ch = dma_request_chan(spfi.dev, "tx");
    if (IS_ERR(spfi.tx_ch)) {
    ret = PTR_ERR(spfi.tx_ch);
    spfi.tx_ch = core::ptr::null_mut();
    if (ret == -EPROBE_DEFER)
    goto free_dma;
    }
    spfi.rx_ch = dma_request_chan(spfi.dev, "rx");
    if (IS_ERR(spfi.rx_ch)) {
    ret = PTR_ERR(spfi.rx_ch);
    spfi.rx_ch = core::ptr::null_mut();
    if (ret == -EPROBE_DEFER)
    goto free_dma;
    }
    if (!spfi.tx_ch || !spfi.rx_ch) {
    if (spfi.tx_ch)
    dma_release_channel(spfi.tx_ch);
    if (spfi.rx_ch)
    dma_release_channel(spfi.rx_ch);
    spfi.tx_ch = core::ptr::null_mut();
    spfi.rx_ch = core::ptr::null_mut();
    dev_warn(spfi.dev, "Failed to get DMA channels, falling back to PIO mode\n");
    } else {
    host.dma_tx = spfi.tx_ch;
    host.dma_rx = spfi.rx_ch;
    host.can_dma = img_spfi_can_dma;
    }
    pm_runtime_set_active(spfi.dev);
    pm_runtime_enable(spfi.dev);
    ret = spi_register_controller(host);
    if (ret)
    goto disable_pm;
    return 0;
    disable_pm:
    pm_runtime_disable(spfi.dev);
    free_dma:
    if (spfi.rx_ch)
    dma_release_channel(spfi.rx_ch);
    if (spfi.tx_ch)
    dma_release_channel(spfi.tx_ch);
    clk_disable_unprepare(spfi.spfi_clk);
    disable_pclk:
    clk_disable_unprepare(spfi.sys_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn img_spfi_remove(pdev: *mut platform_device) {
    static void img_spfi_remove(struct platform_device *pdev)
    {
    struct spi_controller *host = platform_get_drvdata(pdev);
    struct img_spfi *spfi = spi_controller_get_devdata(host);
    spi_unregister_controller(host);
    if (spfi.tx_ch)
    dma_release_channel(spfi.tx_ch);
    if (spfi.rx_ch)
    dma_release_channel(spfi.rx_ch);
    pm_runtime_disable(spfi.dev);
    if (!pm_runtime_status_suspended(spfi.dev)) {
    clk_disable_unprepare(spfi.spfi_clk);
    clk_disable_unprepare(spfi.sys_clk);
    }
    }
#[no_mangle]
unsafe extern "C" fn img_spfi_runtime_suspend(dev: *mut device) -> c_int {
    static int img_spfi_runtime_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct img_spfi *spfi = spi_controller_get_devdata(host);
    clk_disable_unprepare(spfi.spfi_clk);
    clk_disable_unprepare(spfi.sys_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_spfi_runtime_resume(dev: *mut device) -> c_int {
    static int img_spfi_runtime_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct img_spfi *spfi = spi_controller_get_devdata(host);
    int ret;
    ret = clk_prepare_enable(spfi.sys_clk);
    if (ret)
    return ret;
    ret = clk_prepare_enable(spfi.spfi_clk);
    if (ret) {
    clk_disable_unprepare(spfi.sys_clk);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_spfi_suspend(dev: *mut device) -> c_int {
    static int img_spfi_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    return spi_controller_suspend(host);
    }
#[no_mangle]
unsafe extern "C" fn img_spfi_resume(dev: *mut device) -> c_int {
    static int img_spfi_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct img_spfi *spfi = spi_controller_get_devdata(host);
    int ret;
    ret = pm_runtime_resume_and_get(dev);
    if (ret < 0)
    return ret;
    spfi_reset(spfi);
    pm_runtime_put(dev);
    return spi_controller_resume(host);
    }
    static const struct dev_pm_ops img_spfi_pm_ops = {
    RUNTIME_PM_OPS(img_spfi_runtime_suspend, img_spfi_runtime_resume, core::ptr::null_mut())
    SYSTEM_SLEEP_PM_OPS(img_spfi_suspend, img_spfi_resume)
    };
    static const struct of_device_id img_spfi_of_match[] = {
    { .compatible = "img,spfi", },
    { },
    };
    MODULE_DEVICE_TABLE(of, img_spfi_of_match);
    static struct platform_driver img_spfi_driver = {
    .driver = {
    .name = "img-spfi",
    .pm = pm_ptr(&img_spfi_pm_ops),
    .of_match_table = of_match_ptr(img_spfi_of_match),
    },
    .probe = img_spfi_probe,
    .remove = img_spfi_remove,
    };
    module_platform_driver(img_spfi_driver);
    MODULE_DESCRIPTION("IMG SPFI controller driver");
    MODULE_AUTHOR("Andrew Bresticker <abrestic@chromium.org>");
    MODULE_LICENSE("GPL v2");
