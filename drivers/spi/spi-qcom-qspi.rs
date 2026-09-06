//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-qcom-qspi.c
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
// Copyright (c) 2017-2018, The Linux foundation. All rights reserved.

pub const QSPI_NUM_CS: c_int = 2;
pub const QSPI_BYTES_PER_WORD: c_int = 4;
pub const MSTR_CONFIG: c_uint = 0x0000;

pub const SPI_MODE_MSK: c_uint = 0xc00;
pub const SPI_MODE_SHFT: c_int = 10;

pub const LPA_BASE_MSK: c_uint = 0x3c000;
pub const LPA_BASE_SHFT: c_int = 14;
pub const TX_DATA_DELAY_MSK: c_uint = 0xc0000;
pub const TX_DATA_DELAY_SHFT: c_int = 18;
pub const TX_CLK_DELAY_MSK: c_uint = 0x300000;
pub const TX_CLK_DELAY_SHFT: c_int = 20;
pub const TX_CS_N_DELAY_MSK: c_uint = 0xc00000;
pub const TX_CS_N_DELAY_SHFT: c_int = 22;
pub const TX_DATA_OE_DELAY_MSK: c_uint = 0x3000000;
pub const TX_DATA_OE_DELAY_SHFT: c_int = 24;
pub const AHB_MASTER_CFG: c_uint = 0x0004;
pub const HMEM_TYPE_START_MID_TRANS_MSK: c_uint = 0x7;
pub const HMEM_TYPE_START_MID_TRANS_SHFT: c_int = 0;
pub const HMEM_TYPE_LAST_TRANS_MSK: c_uint = 0x38;
pub const HMEM_TYPE_LAST_TRANS_SHFT: c_int = 3;
pub const USE_HMEMTYPE_LAST_ON_DESC_OR_CHAIN_MSK: c_uint = 0xc0;
pub const USE_HMEMTYPE_LAST_ON_DESC_OR_CHAIN_SHFT: c_int = 6;
pub const HMEMTYPE_READ_TRANS_MSK: c_uint = 0x700;
pub const HMEMTYPE_READ_TRANS_SHFT: c_int = 8;

pub const MSTR_INT_EN: c_uint = 0x000C;
pub const MSTR_INT_STATUS: c_uint = 0x0010;

    WR_FIFO_OVERRUN)

    WR_FIFO_EMPTY | WR_FIFO_FULL | \
    TRANSACTION_DONE | DMA_CHAIN_DONE)
pub const PIO_XFER_CTRL: c_uint = 0x0014;
pub const REQUEST_COUNT_MSK: c_uint = 0xffff;
pub const PIO_XFER_CFG: c_uint = 0x0018;

pub const MULTI_IO_MODE_MSK: c_uint = 0xe;
pub const MULTI_IO_MODE_SHFT: c_int = 1;

pub const SDR_1BIT: c_int = 1;
pub const SDR_2BIT: c_int = 2;
pub const SDR_4BIT: c_int = 3;
pub const DDR_1BIT: c_int = 5;
pub const DDR_2BIT: c_int = 6;
pub const DDR_4BIT: c_int = 7;
pub const DMA_DESC_SINGLE_SPI: c_int = 1;
pub const DMA_DESC_DUAL_SPI: c_int = 2;
pub const DMA_DESC_QUAD_SPI: c_int = 3;
pub const PIO_XFER_STATUS: c_uint = 0x001c;
pub const WR_FIFO_BYTES_MSK: c_uint = 0xffff0000;
pub const WR_FIFO_BYTES_SHFT: c_int = 16;
pub const PIO_DATAOUT_1B: c_uint = 0x0020;
pub const PIO_DATAOUT_4B: c_uint = 0x0024;
pub const RD_FIFO_CFG: c_uint = 0x0028;

pub const RD_FIFO_STATUS: c_uint = 0x002c;

pub const WR_CNTS_MSK: c_uint = 0x7f0;
pub const WR_CNTS_SHFT: c_int = 4;

pub const RD_FIFO_RESET: c_uint = 0x0030;

pub const NEXT_DMA_DESC_ADDR: c_uint = 0x0040;
pub const CURRENT_DMA_DESC_ADDR: c_uint = 0x0044;
pub const CURRENT_MEM_ADDR: c_uint = 0x0048;
pub const CUR_MEM_ADDR: c_uint = 0x0048;
pub const HW_VERSION: c_uint = 0x004c;
pub const RD_FIFO: c_uint = 0x0050;
pub const SAMPLING_CLK_CFG: c_uint = 0x0090;
pub const SAMPLING_CLK_STATUS: c_uint = 0x0094;
pub const QSPI_ALIGN_REQ: c_int = 32;
    enum qspi_dir {
    QSPI_READ,
    QSPI_WRITE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qspi_cmd_desc {
    pub data_address: u32,
    pub next_descriptor: u32,
    pub direction:1: u32,
    pub multi_io_mode:3: u32,
    pub reserved1:4: u32,
    pub fragment:1: u32,
    pub reserved2:7: u32,
    pub length:16: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qspi_xfer {
    union {
    pub tx_buf: *const c_void,
    pub rx_buf: *mut c_void,
}

    unsigned int rem_bytes;
    unsigned int buswidth;
    enum qspi_dir dir;
    bool is_last;
    };
    enum qspi_clocks {
    QSPI_CLK_CORE,
    QSPI_CLK_IFACE,
    QSPI_NUM_CLKS
    };
//
// Number of entries in sgt returned from spi framework that-
// will be supported. Can be modified as required.
// In practice, given max_dma_len is 64KB, the number of
// entries is not expected to exceed 1.
//
pub const QSPI_MAX_SG: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_qspi {
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub clks: *mut clk_bulk_data,
    pub xfer: qspi_xfer,
    pub dma_cmd_pool: *mut dma_pool,
    pub dma_cmd_desc: [dma_addr_t; QSPI_MAX_SG],
    pub virt_cmd_desc: [*mut c_void; QSPI_MAX_SG],
    pub n_cmd_desc: c_uint,
    pub icc_path_cpu_to_qspi: *mut icc_path,
    pub icc_path_mem: *mut icc_path,
    pub last_speed: c_ulong,
// Lock to protect data accessed by IRQs
    pub lock: spinlock_t,
}

    static u32 qspi_buswidth_to_iomode(struct qcom_qspi *ctrl,
    unsigned int buswidth)
    {
    switch (buswidth) {
    case 1:
    return SDR_1BIT;
    case 2:
    return SDR_2BIT;
    case 4:
    return SDR_4BIT;
    default:
    dev_warn_once(ctrl.dev,
    "Unexpected bus width: %u\n", buswidth);
    return SDR_1BIT;
    }
    }
#[no_mangle]
unsafe extern "C" fn qcom_qspi_pio_xfer_cfg(ctrl: *mut qcom_qspi) {
    static void qcom_qspi_pio_xfer_cfg(struct qcom_qspi *ctrl)
    {
    u32 pio_xfer_cfg;
    u32 iomode;
    const struct qspi_xfer *xfer;
    xfer = &ctrl.xfer;
    pio_xfer_cfg = readl(ctrl.base + PIO_XFER_CFG);
    pio_xfer_cfg &= ~TRANSFER_DIRECTION;
    pio_xfer_cfg |= xfer.dir;
    if (xfer.is_last)
    pio_xfer_cfg &= ~TRANSFER_FRAGMENT;
    else
    pio_xfer_cfg |= TRANSFER_FRAGMENT;
    pio_xfer_cfg &= ~MULTI_IO_MODE_MSK;
    iomode = qspi_buswidth_to_iomode(ctrl, xfer.buswidth);
    pio_xfer_cfg |= iomode << MULTI_IO_MODE_SHFT;
    writel(pio_xfer_cfg, ctrl.base + PIO_XFER_CFG);
    }
#[no_mangle]
unsafe extern "C" fn qcom_qspi_pio_xfer_ctrl(ctrl: *mut qcom_qspi) {
    static void qcom_qspi_pio_xfer_ctrl(struct qcom_qspi *ctrl)
    {
    u32 pio_xfer_ctrl;
    pio_xfer_ctrl = readl(ctrl.base + PIO_XFER_CTRL);
    pio_xfer_ctrl &= ~REQUEST_COUNT_MSK;
    pio_xfer_ctrl |= ctrl.xfer.rem_bytes;
    writel(pio_xfer_ctrl, ctrl.base + PIO_XFER_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn qcom_qspi_pio_xfer(ctrl: *mut qcom_qspi) {
    static void qcom_qspi_pio_xfer(struct qcom_qspi *ctrl)
    {
    u32 ints;
    qcom_qspi_pio_xfer_cfg(ctrl);
// Ack any previous interrupts that might be hanging around
    writel(QSPI_ALL_IRQS, ctrl.base + MSTR_INT_STATUS);
// Setup new interrupts
    if (ctrl.xfer.dir == QSPI_WRITE)
    ints = QSPI_ERR_IRQS | WR_FIFO_EMPTY;
    else
    ints = QSPI_ERR_IRQS | RESP_FIFO_RDY;
    writel(ints, ctrl.base + MSTR_INT_EN);
// Kick off the transfer
    qcom_qspi_pio_xfer_ctrl(ctrl);
    }
    static void qcom_qspi_handle_err(struct spi_controller *host,
    struct spi_message *msg)
    {
    u32 int_status;
    struct qcom_qspi *ctrl = spi_controller_get_devdata(host);
    unsigned long flags;
    int i;
    spin_lock_irqsave(&ctrl.lock, flags);
    writel(0, ctrl.base + MSTR_INT_EN);
    int_status = readl(ctrl.base + MSTR_INT_STATUS);
    writel(int_status, ctrl.base + MSTR_INT_STATUS);
    ctrl.xfer.rem_bytes = 0;
// free cmd descriptors if they are around (DMA mode)
    for (i = 0; i < ctrl.n_cmd_desc; i++)
    dma_pool_free(ctrl.dma_cmd_pool, ctrl.virt_cmd_desc[i],
    ctrl.dma_cmd_desc[i]);
    ctrl.n_cmd_desc = 0;
    spin_unlock_irqrestore(&ctrl.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn qcom_qspi_set_speed(ctrl: *mut qcom_qspi, speed_hz: c_ulong) -> c_int {
    static int qcom_qspi_set_speed(struct qcom_qspi *ctrl, unsigned long speed_hz)
    {
    int ret;
    unsigned int avg_bw_cpu, avg_bw_mem;
    if (speed_hz == ctrl.last_speed)
    return 0;
// In regular operation (SBL_EN=1) core must be 4x transfer clock
    ret = dev_pm_opp_set_rate(ctrl.dev, speed_hz * 4);
    if (ret) {
    dev_err(ctrl.dev, "Failed to set core clk %d\n", ret);
    return ret;
    }
//
// Set BW quota for CPU and memory paths.
// We don't have explicit peak requirement so keep it equal to avg_bw.
//
    avg_bw_cpu = Bps_to_icc(speed_hz);
    ret = icc_set_bw(ctrl.icc_path_cpu_to_qspi, avg_bw_cpu, avg_bw_cpu);
    if (ret) {
    dev_err(ctrl.dev, "%s: ICC BW voting failed for cpu: %d\n",
    __func__, ret);
    return ret;
    }
    avg_bw_mem = Bps_to_icc(speed_hz);
    ret = icc_set_bw(ctrl.icc_path_mem, avg_bw_mem, avg_bw_mem);
    if (ret) {
    dev_err(ctrl.dev, "ICC BW voting failed for memory: %d\n", ret);
    return ret;
    }
    ctrl.last_speed = speed_hz;
    return 0;
    }
    static int qcom_qspi_alloc_desc(struct qcom_qspi *ctrl, dma_addr_t dma_ptr,
    uint32_t n_bytes)
    {
    struct qspi_cmd_desc *virt_cmd_desc, *prev;
    dma_addr_t dma_cmd_desc;
// allocate for dma cmd descriptor
    virt_cmd_desc = dma_pool_alloc(ctrl.dma_cmd_pool, GFP_ATOMIC | __GFP_ZERO, &dma_cmd_desc);
    if (!virt_cmd_desc) {
    dev_warn_once(ctrl.dev, "Couldn't find memory for descriptor\n");
    return -EAGAIN;
    }
    ctrl.virt_cmd_desc[ctrl.n_cmd_desc] = virt_cmd_desc;
    ctrl.dma_cmd_desc[ctrl.n_cmd_desc] = dma_cmd_desc;
    ctrl.n_cmd_desc++;
// setup cmd descriptor
    virt_cmd_desc.data_address = dma_ptr;
    virt_cmd_desc.direction = ctrl.xfer.dir;
    virt_cmd_desc.multi_io_mode = qspi_buswidth_to_iomode(ctrl, ctrl.xfer.buswidth);
    virt_cmd_desc.fragment = !ctrl.xfer.is_last;
    virt_cmd_desc.length = n_bytes;
// update previous descriptor
    if (ctrl.n_cmd_desc >= 2) {
    prev = (ctrl.virt_cmd_desc)[ctrl.n_cmd_desc - 2];
    prev.next_descriptor = dma_cmd_desc;
    prev.fragment = 1;
    }
    return 0;
    }
    static int qcom_qspi_setup_dma_desc(struct qcom_qspi *ctrl,
    struct spi_transfer *xfer)
    {
    int ret;
    struct sg_table *sgt;
    dma_addr_t dma_ptr_sg;
    unsigned int dma_len_sg;
    int i;
    if (ctrl.n_cmd_desc) {
    dev_err(ctrl.dev, "Remnant dma buffers n_cmd_desc-%d\n", ctrl.n_cmd_desc);
    return -EIO;
    }
    sgt = (ctrl.xfer.dir == QSPI_READ) ? &xfer.rx_sg : &xfer.tx_sg;
    if (!sgt.nents || sgt.nents > QSPI_MAX_SG) {
    dev_warn_once(ctrl.dev, "Cannot handle %d entries in scatter list\n", sgt.nents);
    return -EAGAIN;
    }
    for (i = 0; i < sgt.nents; i++) {
    dma_ptr_sg = sg_dma_address(sgt.sgl + i);
    dma_len_sg = sg_dma_len(sgt.sgl + i);
    if (!IS_ALIGNED(dma_ptr_sg, QSPI_ALIGN_REQ)) {
    dev_warn_once(ctrl.dev, "dma_address not aligned to %d\n", QSPI_ALIGN_REQ);
    return -EAGAIN;
    }
//
// When reading with DMA the controller writes to memory 1 word
// at a time. If the length isn't a multiple of 4 bytes then
// the controller can clobber the things later in memory.
// Fallback to PIO to be safe.
//
    if (ctrl.xfer.dir == QSPI_READ && (dma_len_sg & 0x03)) {
    dev_warn_once(ctrl.dev, "fallback to PIO for read of size %#010x\n",
    dma_len_sg);
    return -EAGAIN;
    }
    }
    for (i = 0; i < sgt.nents; i++) {
    dma_ptr_sg = sg_dma_address(sgt.sgl + i);
    dma_len_sg = sg_dma_len(sgt.sgl + i);
    ret = qcom_qspi_alloc_desc(ctrl, dma_ptr_sg, dma_len_sg);
    if (ret)
    goto cleanup;
    }
    return 0;
    cleanup:
    for (i = 0; i < ctrl.n_cmd_desc; i++)
    dma_pool_free(ctrl.dma_cmd_pool, ctrl.virt_cmd_desc[i],
    ctrl.dma_cmd_desc[i]);
    ctrl.n_cmd_desc = 0;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_qspi_dma_xfer(ctrl: *mut qcom_qspi) {
    static void qcom_qspi_dma_xfer(struct qcom_qspi *ctrl)
    {
// Setup new interrupts
    writel(DMA_CHAIN_DONE, ctrl.base + MSTR_INT_EN);
// kick off transfer
    writel((u32)((ctrl.dma_cmd_desc)[0]), ctrl.base + NEXT_DMA_DESC_ADDR);
    }
// Switch to DMA if transfer length exceeds this
pub const QSPI_MAX_BYTES_FIFO: c_int = 64;
    static bool qcom_qspi_can_dma(struct spi_controller *ctlr,
    struct spi_device *slv, struct spi_transfer *xfer)
    {
    return xfer.len > QSPI_MAX_BYTES_FIFO;
    }
    static int qcom_qspi_transfer_one(struct spi_controller *host,
    struct spi_device *slv,
    struct spi_transfer *xfer)
    {
    struct qcom_qspi *ctrl = spi_controller_get_devdata(host);
    int ret;
    unsigned long speed_hz;
    unsigned long flags;
    u32 mstr_cfg;
    speed_hz = slv.max_speed_hz;
    if (xfer.speed_hz)
    speed_hz = xfer.speed_hz;
    ret = qcom_qspi_set_speed(ctrl, speed_hz);
    if (ret)
    return ret;
    spin_lock_irqsave(&ctrl.lock, flags);
    mstr_cfg = readl(ctrl.base + MSTR_CONFIG);
// We are half duplex, so either rx or tx will be set
    if (xfer.rx_buf) {
    ctrl.xfer.dir = QSPI_READ;
    ctrl.xfer.buswidth = xfer.rx_nbits;
    ctrl.xfer.rx_buf = xfer.rx_buf;
    } else {
    ctrl.xfer.dir = QSPI_WRITE;
    ctrl.xfer.buswidth = xfer.tx_nbits;
    ctrl.xfer.tx_buf = xfer.tx_buf;
    }
    ctrl.xfer.is_last = list_is_last(&xfer.transfer_list,
    &host.cur_msg.transfers);
    ctrl.xfer.rem_bytes = xfer.len;
    if (xfer.rx_sg.nents || xfer.tx_sg.nents) {
// do DMA transfer
    if (!(mstr_cfg & DMA_ENABLE)) {
    mstr_cfg |= DMA_ENABLE;
    writel(mstr_cfg, ctrl.base + MSTR_CONFIG);
    }
    ret = qcom_qspi_setup_dma_desc(ctrl, xfer);
    if (ret != -EAGAIN) {
    if (!ret) {
    dma_wmb();
    qcom_qspi_dma_xfer(ctrl);
    }
    goto exit;
    }
    dev_warn_once(ctrl.dev, "DMA failure, falling back to PIO\n");
    ret = 0; /* We'll retry w/ PIO */
    }
    if (mstr_cfg & DMA_ENABLE) {
    mstr_cfg &= ~DMA_ENABLE;
    writel(mstr_cfg, ctrl.base + MSTR_CONFIG);
    }
    qcom_qspi_pio_xfer(ctrl);
    exit:
    spin_unlock_irqrestore(&ctrl.lock, flags);
    if (ret)
    return ret;
// We'll call spi_finalize_current_transfer() when done
    return 1;
    }
    static int qcom_qspi_prepare_message(struct spi_controller *host,
    struct spi_message *message)
    {
    u32 mstr_cfg;
    struct qcom_qspi *ctrl;
    let mut tx_data_oe_delay: c_int = 1;
    let mut tx_data_delay: c_int = 1;
    unsigned long flags;
    ctrl = spi_controller_get_devdata(host);
    spin_lock_irqsave(&ctrl.lock, flags);
    mstr_cfg = readl(ctrl.base + MSTR_CONFIG);
    mstr_cfg &= ~CHIP_SELECT_NUM;
    if (spi_get_chipselect(message.spi, 0))
    mstr_cfg |= CHIP_SELECT_NUM;
    mstr_cfg |= FB_CLK_EN | PIN_WPN | PIN_HOLDN | SBL_EN | FULL_CYCLE_MODE;
    mstr_cfg &= ~(SPI_MODE_MSK | TX_DATA_OE_DELAY_MSK | TX_DATA_DELAY_MSK);
    mstr_cfg |= message.spi.mode << SPI_MODE_SHFT;
    mstr_cfg |= tx_data_oe_delay << TX_DATA_OE_DELAY_SHFT;
    mstr_cfg |= tx_data_delay << TX_DATA_DELAY_SHFT;
    mstr_cfg &= ~DMA_ENABLE;
    writel(mstr_cfg, ctrl.base + MSTR_CONFIG);
    spin_unlock_irqrestore(&ctrl.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_qspi_alloc_dma(ctrl: *mut qcom_qspi) -> c_int {
    static int qcom_qspi_alloc_dma(struct qcom_qspi *ctrl)
    {
    ctrl.dma_cmd_pool = dmam_pool_create("qspi cmd desc pool",
    ctrl.dev, sizeof(struct qspi_cmd_desc), 0, 0);
    if (!ctrl.dma_cmd_pool)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pio_read(ctrl: *mut qcom_qspi) -> irqreturn_t {
    static irqreturn_t pio_read(struct qcom_qspi *ctrl)
    {
    u32 rd_fifo_status;
    u32 rd_fifo;
    unsigned int wr_cnts;
    unsigned int bytes_to_read;
    unsigned int words_to_read;
    u32 *word_buf;
    u8 *byte_buf;
    int i;
    rd_fifo_status = readl(ctrl.base + RD_FIFO_STATUS);
    if (!(rd_fifo_status & FIFO_RDY)) {
    dev_dbg(ctrl.dev, "Spurious IRQ %#x\n", rd_fifo_status);
    return IRQ_NONE;
    }
    wr_cnts = (rd_fifo_status & WR_CNTS_MSK) >> WR_CNTS_SHFT;
    wr_cnts = min(wr_cnts, ctrl.xfer.rem_bytes);
    words_to_read = wr_cnts / QSPI_BYTES_PER_WORD;
    bytes_to_read = wr_cnts % QSPI_BYTES_PER_WORD;
    if (words_to_read) {
    word_buf = ctrl.xfer.rx_buf;
    ctrl.xfer.rem_bytes -= words_to_read * QSPI_BYTES_PER_WORD;
    ioread32_rep(ctrl.base + RD_FIFO, word_buf, words_to_read);
    ctrl.xfer.rx_buf = word_buf + words_to_read;
    }
    if (bytes_to_read) {
    byte_buf = ctrl.xfer.rx_buf;
    rd_fifo = readl(ctrl.base + RD_FIFO);
    ctrl.xfer.rem_bytes -= bytes_to_read;
    for (i = 0; i < bytes_to_read; i++)
// byte_buf++ = rd_fifo >> (i * BITS_PER_BYTE);
    ctrl.xfer.rx_buf = byte_buf;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pio_write(ctrl: *mut qcom_qspi) -> irqreturn_t {
    static irqreturn_t pio_write(struct qcom_qspi *ctrl)
    {
    const void *xfer_buf = ctrl.xfer.tx_buf;
    const int *word_buf;
    const char *byte_buf;
    unsigned int wr_fifo_bytes;
    unsigned int wr_fifo_words;
    unsigned int wr_size;
    unsigned int rem_words;
    wr_fifo_bytes = readl(ctrl.base + PIO_XFER_STATUS);
    wr_fifo_bytes >>= WR_FIFO_BYTES_SHFT;
    if (ctrl.xfer.rem_bytes < QSPI_BYTES_PER_WORD) {
// Process the last 1-3 bytes
    wr_size = min(wr_fifo_bytes, ctrl.xfer.rem_bytes);
    ctrl.xfer.rem_bytes -= wr_size;
    byte_buf = xfer_buf;
    while (wr_size--)
    writel(*byte_buf++,
    ctrl.base + PIO_DATAOUT_1B);
    ctrl.xfer.tx_buf = byte_buf;
    } else {
//
// Process all the whole words; to keep things simple we'll
// just wait for the next interrupt to handle the last 1-3
// bytes if we don't have an even number of words.
//
    rem_words = ctrl.xfer.rem_bytes / QSPI_BYTES_PER_WORD;
    wr_fifo_words = wr_fifo_bytes / QSPI_BYTES_PER_WORD;
    wr_size = min(rem_words, wr_fifo_words);
    ctrl.xfer.rem_bytes -= wr_size * QSPI_BYTES_PER_WORD;
    word_buf = xfer_buf;
    iowrite32_rep(ctrl.base + PIO_DATAOUT_4B, word_buf, wr_size);
    ctrl.xfer.tx_buf = word_buf + wr_size;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn qcom_qspi_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t qcom_qspi_irq(int irq, void *dev_id)
    {
    u32 int_status;
    struct qcom_qspi *ctrl = dev_id;
    let mut ret: irqreturn_t = IRQ_NONE;
    spin_lock(&ctrl.lock);
    int_status = readl(ctrl.base + MSTR_INT_STATUS);
    writel(int_status, ctrl.base + MSTR_INT_STATUS);
// Ignore disabled interrupts
    int_status &= readl(ctrl.base + MSTR_INT_EN);
// PIO mode handling
    if (ctrl.xfer.dir == QSPI_WRITE) {
    if (int_status & WR_FIFO_EMPTY)
    ret = pio_write(ctrl);
    } else {
    if (int_status & RESP_FIFO_RDY)
    ret = pio_read(ctrl);
    }
    if (int_status & QSPI_ERR_IRQS) {
    if (int_status & RESP_FIFO_UNDERRUN)
    dev_err(ctrl.dev, "IRQ error: FIFO underrun\n");
    if (int_status & WR_FIFO_OVERRUN)
    dev_err(ctrl.dev, "IRQ error: FIFO overrun\n");
    if (int_status & HRESP_FROM_NOC_ERR)
    dev_err(ctrl.dev, "IRQ error: NOC response error\n");
    ret = IRQ_HANDLED;
    }
    if (!ctrl.xfer.rem_bytes) {
    writel(0, ctrl.base + MSTR_INT_EN);
    spi_finalize_current_transfer(dev_get_drvdata(ctrl.dev));
    }
// DMA mode handling
    if (int_status & DMA_CHAIN_DONE) {
    int i;
    writel(0, ctrl.base + MSTR_INT_EN);
    ctrl.xfer.rem_bytes = 0;
    for (i = 0; i < ctrl.n_cmd_desc; i++)
    dma_pool_free(ctrl.dma_cmd_pool, ctrl.virt_cmd_desc[i],
    ctrl.dma_cmd_desc[i]);
    ctrl.n_cmd_desc = 0;
    ret = IRQ_HANDLED;
    spi_finalize_current_transfer(dev_get_drvdata(ctrl.dev));
    }
    spin_unlock(&ctrl.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_qspi_adjust_op_size(mem: *mut spi_mem, op: *mut spi_mem_op) -> c_int {
    static int qcom_qspi_adjust_op_size(struct spi_mem *mem, struct spi_mem_op *op)
    {
//
// If qcom_qspi_can_dma() is going to return false we don't need to
// adjust anything.
//
    if (op.data.nbytes <= QSPI_MAX_BYTES_FIFO)
    return 0;
//
// When reading, the transfer needs to be a multiple of 4 bytes so
// shrink the transfer if that's not true. The caller will then do a
// second transfer to finish things up.
//
    if (op.data.dir == SPI_MEM_DATA_IN && (op.data.nbytes & 0x3))
    op.data.nbytes &= ~0x3;
    return 0;
    }
    static const struct spi_controller_mem_ops qcom_qspi_mem_ops = {
    .adjust_op_size = qcom_qspi_adjust_op_size,
    };
#[no_mangle]
unsafe extern "C" fn qcom_qspi_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_qspi_probe(struct platform_device *pdev)
    {
    int ret;
    struct device *dev;
    struct spi_controller *host;
    struct qcom_qspi *ctrl;
    dev = &pdev.dev;
    host = devm_spi_alloc_host(dev, sizeof(*ctrl));
    if (!host)
    return -ENOMEM;
    platform_set_drvdata(pdev, host);
    ctrl = spi_controller_get_devdata(host);
    spin_lock_init(&ctrl.lock);
    ctrl.dev = dev;
    ctrl.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ctrl.base))
    return PTR_ERR(ctrl.base);
    ctrl.clks = devm_kcalloc(dev, QSPI_NUM_CLKS,
    sizeof(*ctrl.clks), GFP_KERNEL);
    if (!ctrl.clks)
    return -ENOMEM;
    ctrl.clks[QSPI_CLK_CORE].id = "core";
    ctrl.clks[QSPI_CLK_IFACE].id = "iface";
    ret = devm_clk_bulk_get(dev, QSPI_NUM_CLKS, ctrl.clks);
    if (ret)
    return ret;
    ctrl.icc_path_cpu_to_qspi = devm_of_icc_get(dev, "qspi-config");
    if (IS_ERR(ctrl.icc_path_cpu_to_qspi))
    return dev_err_probe(dev, PTR_ERR(ctrl.icc_path_cpu_to_qspi),
    "Failed to get cpu path\n");
    ctrl.icc_path_mem = devm_of_icc_get(dev, "qspi-memory");
    if (IS_ERR(ctrl.icc_path_mem)) {
    if (PTR_ERR(ctrl.icc_path_mem) != -ENODATA)
    return dev_err_probe(dev, PTR_ERR(ctrl.icc_path_mem),
    "Failed to get memory path\n");
    ctrl.icc_path_mem = core::ptr::null_mut();
    }
// Set BW vote for register access
    ret = icc_set_bw(ctrl.icc_path_cpu_to_qspi, Bps_to_icc(1000),
    Bps_to_icc(1000));
    if (ret) {
    dev_err(ctrl.dev, "%s: ICC BW voting failed for cpu: %d\n",
    __func__, ret);
    return ret;
    }
    ret = icc_disable(ctrl.icc_path_cpu_to_qspi);
    if (ret) {
    dev_err(ctrl.dev, "%s: ICC disable failed for cpu: %d\n",
    __func__, ret);
    return ret;
    }
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    return ret;
    ret = devm_request_irq(dev, ret, qcom_qspi_irq, 0, dev_name(dev), ctrl);
    if (ret) {
    dev_err(dev, "Failed to request irq %d\n", ret);
    return ret;
    }
    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(32));
    if (ret)
    return dev_err_probe(dev, ret, "could not set DMA mask\n");
    host.max_speed_hz = 300000000;
// as per HPG, it is 64KB, limit to 60KB to avoid boundary condition failures
    host.max_dma_len = 0xf000;
    host.dma_alignment = QSPI_ALIGN_REQ;
    host.num_chipselect = QSPI_NUM_CS;
    host.bus_num = -1;
    host.mode_bits = SPI_MODE_0 |
    SPI_TX_DUAL | SPI_RX_DUAL |
    SPI_TX_QUAD | SPI_RX_QUAD;
    host.flags = SPI_CONTROLLER_HALF_DUPLEX;
    host.prepare_message = qcom_qspi_prepare_message;
    host.transfer_one = qcom_qspi_transfer_one;
    host.handle_err = qcom_qspi_handle_err;
    if (of_property_present(pdev.dev.of_node, "iommus"))
    host.can_dma = qcom_qspi_can_dma;
    host.auto_runtime_pm = true;
    host.mem_ops = &qcom_qspi_mem_ops;
    ret = devm_pm_opp_set_clkname(&pdev.dev, "core");
    if (ret)
    return ret;
// OPP table is optional
    ret = devm_pm_opp_of_add_table(&pdev.dev);
    if (ret && ret != -ENODEV) {
    dev_err(&pdev.dev, "invalid OPP table in device tree\n");
    return ret;
    }
    ret = qcom_qspi_alloc_dma(ctrl);
    if (ret)
    return ret;
    pm_runtime_use_autosuspend(dev);
    pm_runtime_set_autosuspend_delay(dev, 250);
    pm_runtime_enable(dev);
    ret = spi_register_controller(host);
    if (!ret)
    return 0;
    pm_runtime_disable(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_qspi_remove(pdev: *mut platform_device) {
    static void qcom_qspi_remove(struct platform_device *pdev)
    {
    struct spi_controller *host = platform_get_drvdata(pdev);
// Unregister _before_ disabling pm_runtime() so we stop transfers
    spi_unregister_controller(host);
    pm_runtime_disable(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn qcom_qspi_runtime_suspend(dev: *mut device) -> c_int {
    static int qcom_qspi_runtime_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct qcom_qspi *ctrl = spi_controller_get_devdata(host);
    int ret;
    clk_bulk_disable_unprepare(QSPI_NUM_CLKS, ctrl.clks);
    ret = icc_disable(ctrl.icc_path_cpu_to_qspi);
    if (ret) {
    dev_err_ratelimited(ctrl.dev, "%s: ICC disable failed for cpu: %d\n",
    __func__, ret);
    goto err_enable_clk;
    }
    ret = icc_disable(ctrl.icc_path_mem);
    if (ret) {
    dev_err_ratelimited(ctrl.dev, "ICC disable failed for memory: %d\n", ret);
    goto err_enable_icc_cpu;
    }
    ret = pinctrl_pm_select_sleep_state(dev);
    if (ret)
    goto err_enable_icc_mem;
// Drop the performance state vote
    ret = dev_pm_opp_set_rate(dev, 0);
    if (ret)
    goto err_select_default_state;
    return 0;
    err_select_default_state:
    pinctrl_pm_select_default_state(dev);
    err_enable_icc_mem:
    icc_enable(ctrl.icc_path_mem);
    err_enable_icc_cpu:
    icc_enable(ctrl.icc_path_cpu_to_qspi);
    err_enable_clk:
    if (clk_bulk_prepare_enable(QSPI_NUM_CLKS, ctrl.clks))
    dev_err_ratelimited(ctrl.dev, "Failed to re-enable clocks\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_qspi_runtime_resume(dev: *mut device) -> c_int {
    static int qcom_qspi_runtime_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct qcom_qspi *ctrl = spi_controller_get_devdata(host);
    int ret;
    ret = dev_pm_opp_set_rate(dev, ctrl.last_speed * 4);
    if (ret)
    return ret;
    ret = pinctrl_pm_select_default_state(dev);
    if (ret)
    goto err_opp_set_rate_zero;
    ret = icc_enable(ctrl.icc_path_cpu_to_qspi);
    if (ret) {
    dev_err_ratelimited(ctrl.dev, "%s: ICC enable failed for cpu: %d\n",
    __func__, ret);
    goto err_select_sleep_state;
    }
    ret = icc_enable(ctrl.icc_path_mem);
    if (ret) {
    dev_err_ratelimited(ctrl.dev, "ICC enable failed for memory: %d\n", ret);
    goto err_disable_icc_cpu;
    }
    ret = clk_bulk_prepare_enable(QSPI_NUM_CLKS, ctrl.clks);
    if (ret)
    goto err_disable_icc_mem;
    return 0;
    err_disable_icc_mem:
    icc_disable(ctrl.icc_path_mem);
    err_disable_icc_cpu:
    icc_disable(ctrl.icc_path_cpu_to_qspi);
    err_select_sleep_state:
    pinctrl_pm_select_sleep_state(dev);
    err_opp_set_rate_zero:
    dev_pm_opp_set_rate(dev, 0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_qspi_suspend(dev: *mut device) -> c_int {
    static int qcom_qspi_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    int ret;
    ret = spi_controller_suspend(host);
    if (ret)
    return ret;
    ret = pm_runtime_force_suspend(dev);
    if (ret)
    spi_controller_resume(host);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_qspi_resume(dev: *mut device) -> c_int {
    static int qcom_qspi_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    int ret;
    ret = pm_runtime_force_resume(dev);
    if (ret)
    return ret;
    ret = spi_controller_resume(host);
    if (ret)
    pm_runtime_force_suspend(dev);
    return ret;
    }
    static const struct dev_pm_ops qcom_qspi_dev_pm_ops = {
    RUNTIME_PM_OPS(qcom_qspi_runtime_suspend,
    qcom_qspi_runtime_resume, core::ptr::null_mut())
    SYSTEM_SLEEP_PM_OPS(qcom_qspi_suspend, qcom_qspi_resume)
    };
    static const struct of_device_id qcom_qspi_dt_match[] = {
    { .compatible = "qcom,qspi-v1", },
    { }
    };
    MODULE_DEVICE_TABLE(of, qcom_qspi_dt_match);
    static struct platform_driver qcom_qspi_driver = {
    .driver = {
    .name		= "qcom_qspi",
    .pm		= pm_ptr(&qcom_qspi_dev_pm_ops),
    .of_match_table = qcom_qspi_dt_match,
    },
    .probe = qcom_qspi_probe,
    .remove = qcom_qspi_remove,
    };
    module_platform_driver(qcom_qspi_driver);
    MODULE_DESCRIPTION("SPI driver for QSPI cores");
    MODULE_LICENSE("GPL v2");
