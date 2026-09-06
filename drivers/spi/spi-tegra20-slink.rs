//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-tegra20-slink.c
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
// SPI driver for Nvidia's Tegra20/Tegra30 SLINK Controller.
//
// Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
//

pub const SLINK_COMMAND: c_uint = 0x000;

pub const SLINK_COMMAND2: c_uint = 0x004;

pub const SLINK_STATUS: c_uint = 0x008;

    SLINK_TX_UNF | SLINK_RX_OVF)

pub const SLINK_MAS_DATA: c_uint = 0x010;
pub const SLINK_SLAVE_DATA: c_uint = 0x014;
pub const SLINK_DMA_CTL: c_uint = 0x018;

pub const SLINK_STATUS2: c_uint = 0x01c;

pub const SLINK_TX_FIFO: c_uint = 0x100;
pub const SLINK_RX_FIFO: c_uint = 0x180;

    (TX_FIFO_EMPTY_COUNT_MAX | RX_FIFO_FULL_COUNT_ZERO << 16)
pub const MAX_CHIP_SELECT: c_int = 4;
pub const SLINK_FIFO_DEPTH: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_slink_chip_data {
    pub cs_hold_time: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_slink_data {
    pub dev: *mut device,
    pub host: *mut spi_controller,
    pub chip_data: *const tegra_slink_chip_data,
    pub lock: spinlock_t,
    pub clk: *mut clk,
    pub rst: *mut reset_control,
    pub base: *mut void __iomem,
    pub phys: phys_addr_t,
    pub irq: unsigned,
    pub cur_speed: u32,
    pub cur_spi: *mut spi_device,
    pub cur_pos: unsigned,
    pub cur_len: unsigned,
    pub words_per_32bit: unsigned,
    pub bytes_per_word: unsigned,
    pub curr_dma_words: unsigned,
    pub cur_direction: unsigned,
    pub cur_rx_pos: unsigned,
    pub cur_tx_pos: unsigned,
    pub dma_buf_size: unsigned,
    pub max_buf_size: unsigned,
    pub is_curr_dma_xfer: bool,
    pub rx_dma_complete: completion,
    pub tx_dma_complete: completion,
    pub tx_status: u32,
    pub rx_status: u32,
    pub status_reg: u32,
    pub is_packed: bool,
    pub packed_size: u32,
    pub command_reg: u32,
    pub command2_reg: u32,
    pub dma_control_reg: u32,
    pub def_command_reg: u32,
    pub def_command2_reg: u32,
    pub xfer_completion: completion,
    pub curr_xfer: *mut spi_transfer,
    pub rx_dma_chan: *mut dma_chan,
    pub rx_dma_buf: *mut u32,
    pub rx_dma_phys: dma_addr_t,
    pub rx_dma_desc: *mut dma_async_tx_descriptor,
    pub tx_dma_chan: *mut dma_chan,
    pub tx_dma_buf: *mut u32,
    pub tx_dma_phys: dma_addr_t,
    pub tx_dma_desc: *mut dma_async_tx_descriptor,
}

    static inline u32 tegra_slink_readl(struct tegra_slink_data *tspi,
    unsigned long reg)
    {
    return readl(tspi.base + reg);
    }
    static inline void tegra_slink_writel(struct tegra_slink_data *tspi,
    u32 val, unsigned long reg)
    {
    writel(val, tspi.base + reg);
// Read back register to make sure that register writes completed
    if (reg != SLINK_TX_FIFO)
    readl(tspi.base + SLINK_MAS_DATA);
    }
#[no_mangle]
unsafe extern "C" fn tegra_slink_clear_status(tspi: *mut tegra_slink_data) {
    static void tegra_slink_clear_status(struct tegra_slink_data *tspi)
    {
    u32 val_write;
    tegra_slink_readl(tspi, SLINK_STATUS);
// Write 1 to clear status register
    val_write = SLINK_RDY | SLINK_FIFO_ERROR;
    tegra_slink_writel(tspi, val_write, SLINK_STATUS);
    }
    static u32 tegra_slink_get_packed_size(struct tegra_slink_data *tspi,
    struct spi_transfer *t)
    {
    switch (tspi.bytes_per_word) {
    case 0:
    return SLINK_PACK_SIZE_4;
    case 1:
    return SLINK_PACK_SIZE_8;
    case 2:
    return SLINK_PACK_SIZE_16;
    case 4:
    return SLINK_PACK_SIZE_32;
    default:
    return 0;
    }
    }
    static unsigned tegra_slink_calculate_curr_xfer_param(
    struct spi_device *spi, struct tegra_slink_data *tspi,
    struct spi_transfer *t)
    {
    let mut remain_len: unsigned = t.len - tspi.cur_pos;
    unsigned max_word;
    unsigned bits_per_word;
    unsigned max_len;
    unsigned total_fifo_words;
    bits_per_word = t.bits_per_word;
    tspi.bytes_per_word = DIV_ROUND_UP(bits_per_word, 8);
    if (bits_per_word == 8 || bits_per_word == 16) {
    tspi.is_packed = true;
    tspi.words_per_32bit = 32/bits_per_word;
    } else {
    tspi.is_packed = false;
    tspi.words_per_32bit = 1;
    }
    tspi.packed_size = tegra_slink_get_packed_size(tspi, t);
    if (tspi.is_packed) {
    max_len = min(remain_len, tspi.max_buf_size);
    tspi.curr_dma_words = max_len/tspi.bytes_per_word;
    total_fifo_words = max_len/4;
    } else {
    max_word = (remain_len - 1) / tspi.bytes_per_word + 1;
    max_word = min(max_word, tspi.max_buf_size/4);
    tspi.curr_dma_words = max_word;
    total_fifo_words = max_word;
    }
    return total_fifo_words;
    }
    static unsigned tegra_slink_fill_tx_fifo_from_client_txbuf(
    struct tegra_slink_data *tspi, struct spi_transfer *t)
    {
    unsigned nbytes;
    unsigned tx_empty_count;
    u32 fifo_status;
    unsigned max_n_32bit;
    unsigned i, count;
    unsigned int written_words;
    unsigned fifo_words_left;
    u8 *tx_buf = (u8 *)t.tx_buf + tspi.cur_tx_pos;
    fifo_status = tegra_slink_readl(tspi, SLINK_STATUS2);
    tx_empty_count = SLINK_TX_FIFO_EMPTY_COUNT(fifo_status);
    if (tspi.is_packed) {
    fifo_words_left = tx_empty_count * tspi.words_per_32bit;
    written_words = min(fifo_words_left, tspi.curr_dma_words);
    nbytes = written_words * tspi.bytes_per_word;
    max_n_32bit = DIV_ROUND_UP(nbytes, 4);
    for (count = 0; count < max_n_32bit; count++) {
    let mut x: u32 = 0;
    for (i = 0; (i < 4) && nbytes; i++, nbytes--)
    x |= (u32)(*tx_buf++) << (i * 8);
    tegra_slink_writel(tspi, x, SLINK_TX_FIFO);
    }
    } else {
    max_n_32bit = min(tspi.curr_dma_words,  tx_empty_count);
    written_words = max_n_32bit;
    nbytes = written_words * tspi.bytes_per_word;
    for (count = 0; count < max_n_32bit; count++) {
    let mut x: u32 = 0;
    for (i = 0; nbytes && (i < tspi.bytes_per_word);
    i++, nbytes--)
    x |= (u32)(*tx_buf++) << (i * 8);
    tegra_slink_writel(tspi, x, SLINK_TX_FIFO);
    }
    }
    tspi.cur_tx_pos += written_words * tspi.bytes_per_word;
    return written_words;
    }
    static unsigned int tegra_slink_read_rx_fifo_to_client_rxbuf(
    struct tegra_slink_data *tspi, struct spi_transfer *t)
    {
    unsigned rx_full_count;
    u32 fifo_status;
    unsigned i, count;
    let mut read_words: c_uint = 0;
    unsigned len;
    u8 *rx_buf = (u8 *)t.rx_buf + tspi.cur_rx_pos;
    fifo_status = tegra_slink_readl(tspi, SLINK_STATUS2);
    rx_full_count = SLINK_RX_FIFO_FULL_COUNT(fifo_status);
    if (tspi.is_packed) {
    len = tspi.curr_dma_words * tspi.bytes_per_word;
    for (count = 0; count < rx_full_count; count++) {
    let mut x: u32 = tegra_slink_readl(tspi, SLINK_RX_FIFO);
    for (i = 0; len && (i < 4); i++, len--)
// rx_buf++ = (x >> i*8) & 0xFF;
    }
    tspi.cur_rx_pos += tspi.curr_dma_words * tspi.bytes_per_word;
    read_words += tspi.curr_dma_words;
    } else {
    for (count = 0; count < rx_full_count; count++) {
    let mut x: u32 = tegra_slink_readl(tspi, SLINK_RX_FIFO);
    for (i = 0; (i < tspi.bytes_per_word); i++)
// rx_buf++ = (x >> (i*8)) & 0xFF;
    }
    tspi.cur_rx_pos += rx_full_count * tspi.bytes_per_word;
    read_words += rx_full_count;
    }
    return read_words;
    }
    static void tegra_slink_copy_client_txbuf_to_spi_txbuf(
    struct tegra_slink_data *tspi, struct spi_transfer *t)
    {
// Make the dma buffer to read by cpu
    dma_sync_single_for_cpu(tspi.dev, tspi.tx_dma_phys,
    tspi.dma_buf_size, DMA_TO_DEVICE);
    if (tspi.is_packed) {
    let mut len: unsigned = tspi.curr_dma_words * tspi.bytes_per_word;
    memcpy(tspi.tx_dma_buf, t.tx_buf + tspi.cur_pos, len);
    } else {
    unsigned int i;
    unsigned int count;
    u8 *tx_buf = (u8 *)t.tx_buf + tspi.cur_tx_pos;
    let mut consume: unsigned = tspi.curr_dma_words * tspi.bytes_per_word;
    for (count = 0; count < tspi.curr_dma_words; count++) {
    let mut x: u32 = 0;
    for (i = 0; consume && (i < tspi.bytes_per_word);
    i++, consume--)
    x |= (u32)(*tx_buf++) << (i * 8);
    tspi.tx_dma_buf[count] = x;
    }
    }
    tspi.cur_tx_pos += tspi.curr_dma_words * tspi.bytes_per_word;
// Make the dma buffer to read by dma
    dma_sync_single_for_device(tspi.dev, tspi.tx_dma_phys,
    tspi.dma_buf_size, DMA_TO_DEVICE);
    }
    static void tegra_slink_copy_spi_rxbuf_to_client_rxbuf(
    struct tegra_slink_data *tspi, struct spi_transfer *t)
    {
    unsigned len;
// Make the dma buffer to read by cpu
    dma_sync_single_for_cpu(tspi.dev, tspi.rx_dma_phys,
    tspi.dma_buf_size, DMA_FROM_DEVICE);
    if (tspi.is_packed) {
    len = tspi.curr_dma_words * tspi.bytes_per_word;
    memcpy(t.rx_buf + tspi.cur_rx_pos, tspi.rx_dma_buf, len);
    } else {
    unsigned int i;
    unsigned int count;
    unsigned char *rx_buf = t.rx_buf + tspi.cur_rx_pos;
    let mut rx_mask: u32 = ((u32)1 << t.bits_per_word) - 1;
    for (count = 0; count < tspi.curr_dma_words; count++) {
    let mut x: u32 = tspi.rx_dma_buf[count] & rx_mask;
    for (i = 0; (i < tspi.bytes_per_word); i++)
// rx_buf++ = (x >> (i*8)) & 0xFF;
    }
    }
    tspi.cur_rx_pos += tspi.curr_dma_words * tspi.bytes_per_word;
// Make the dma buffer to read by dma
    dma_sync_single_for_device(tspi.dev, tspi.rx_dma_phys,
    tspi.dma_buf_size, DMA_FROM_DEVICE);
    }
#[no_mangle]
unsafe extern "C" fn tegra_slink_dma_complete(args: *mut c_void) {
    static void tegra_slink_dma_complete(void *args)
    {
    struct completion *dma_complete = args;
    complete(dma_complete);
    }
#[no_mangle]
unsafe extern "C" fn tegra_slink_start_tx_dma(tspi: *mut tegra_slink_data, len: c_int) -> c_int {
    static int tegra_slink_start_tx_dma(struct tegra_slink_data *tspi, int len)
    {
    reinit_completion(&tspi.tx_dma_complete);
    tspi.tx_dma_desc = dmaengine_prep_slave_single(tspi.tx_dma_chan,
    tspi.tx_dma_phys, len, DMA_MEM_TO_DEV,
    DMA_PREP_INTERRUPT |  DMA_CTRL_ACK);
    if (!tspi.tx_dma_desc) {
    dev_err(tspi.dev, "Not able to get desc for Tx\n");
    return -EIO;
    }
    tspi.tx_dma_desc.callback = tegra_slink_dma_complete;
    tspi.tx_dma_desc.callback_param = &tspi.tx_dma_complete;
    dmaengine_submit(tspi.tx_dma_desc);
    dma_async_issue_pending(tspi.tx_dma_chan);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_slink_start_rx_dma(tspi: *mut tegra_slink_data, len: c_int) -> c_int {
    static int tegra_slink_start_rx_dma(struct tegra_slink_data *tspi, int len)
    {
    reinit_completion(&tspi.rx_dma_complete);
    tspi.rx_dma_desc = dmaengine_prep_slave_single(tspi.rx_dma_chan,
    tspi.rx_dma_phys, len, DMA_DEV_TO_MEM,
    DMA_PREP_INTERRUPT |  DMA_CTRL_ACK);
    if (!tspi.rx_dma_desc) {
    dev_err(tspi.dev, "Not able to get desc for Rx\n");
    return -EIO;
    }
    tspi.rx_dma_desc.callback = tegra_slink_dma_complete;
    tspi.rx_dma_desc.callback_param = &tspi.rx_dma_complete;
    dmaengine_submit(tspi.rx_dma_desc);
    dma_async_issue_pending(tspi.rx_dma_chan);
    return 0;
    }
    static int tegra_slink_start_dma_based_transfer(
    struct tegra_slink_data *tspi, struct spi_transfer *t)
    {
    u32 val;
    unsigned int len;
    let mut ret: c_int = 0;
    u32 status;
// Make sure that Rx and Tx fifo are empty
    status = tegra_slink_readl(tspi, SLINK_STATUS);
    if ((status & SLINK_FIFO_EMPTY) != SLINK_FIFO_EMPTY) {
    dev_err(tspi.dev, "Rx/Tx fifo are not empty status 0x%08x\n",
    (unsigned)status);
    return -EIO;
    }
    val = SLINK_DMA_BLOCK_SIZE(tspi.curr_dma_words - 1);
    val |= tspi.packed_size;
    if (tspi.is_packed)
    len = DIV_ROUND_UP(tspi.curr_dma_words * tspi.bytes_per_word,
    4) * 4;
    else
    len = tspi.curr_dma_words * 4;
// Set attention level based on length of transfer
    if (len & 0xF)
    val |= SLINK_TX_TRIG_1 | SLINK_RX_TRIG_1;
#[no_mangle]
pub unsafe extern "C" fn if(0x1: ((len) >> 4) &) -> else {
    else if (((len) >> 4) & 0x1)
    val |= SLINK_TX_TRIG_4 | SLINK_RX_TRIG_4;
    else
    val |= SLINK_TX_TRIG_8 | SLINK_RX_TRIG_8;
    if (tspi.cur_direction & DATA_DIR_TX)
    val |= SLINK_IE_TXC;
    if (tspi.cur_direction & DATA_DIR_RX)
    val |= SLINK_IE_RXC;
    tegra_slink_writel(tspi, val, SLINK_DMA_CTL);
    tspi.dma_control_reg = val;
    if (tspi.cur_direction & DATA_DIR_TX) {
    tegra_slink_copy_client_txbuf_to_spi_txbuf(tspi, t);
    wmb();
    ret = tegra_slink_start_tx_dma(tspi, len);
    if (ret < 0) {
    dev_err(tspi.dev,
    "Starting tx dma failed, err %d\n", ret);
    return ret;
    }
// Wait for tx fifo to be fill before starting slink
    status = tegra_slink_readl(tspi, SLINK_STATUS);
    while (!(status & SLINK_TX_FULL))
    status = tegra_slink_readl(tspi, SLINK_STATUS);
    }
    if (tspi.cur_direction & DATA_DIR_RX) {
// Make the dma buffer to read by dma
    dma_sync_single_for_device(tspi.dev, tspi.rx_dma_phys,
    tspi.dma_buf_size, DMA_FROM_DEVICE);
    ret = tegra_slink_start_rx_dma(tspi, len);
    if (ret < 0) {
    dev_err(tspi.dev,
    "Starting rx dma failed, err %d\n", ret);
    if (tspi.cur_direction & DATA_DIR_TX)
    dmaengine_terminate_all(tspi.tx_dma_chan);
    return ret;
    }
    }
    tspi.is_curr_dma_xfer = true;
    if (tspi.is_packed) {
    val |= SLINK_PACKED;
    tegra_slink_writel(tspi, val, SLINK_DMA_CTL);
// HW need small delay after setting Packed mode
    udelay(1);
    }
    tspi.dma_control_reg = val;
    val |= SLINK_DMA_EN;
    tegra_slink_writel(tspi, val, SLINK_DMA_CTL);
    return ret;
    }
    static int tegra_slink_start_cpu_based_transfer(
    struct tegra_slink_data *tspi, struct spi_transfer *t)
    {
    u32 val;
    unsigned cur_words;
    val = tspi.packed_size;
    if (tspi.cur_direction & DATA_DIR_TX)
    val |= SLINK_IE_TXC;
    if (tspi.cur_direction & DATA_DIR_RX)
    val |= SLINK_IE_RXC;
    tegra_slink_writel(tspi, val, SLINK_DMA_CTL);
    tspi.dma_control_reg = val;
    if (tspi.cur_direction & DATA_DIR_TX)
    cur_words = tegra_slink_fill_tx_fifo_from_client_txbuf(tspi, t);
    else
    cur_words = tspi.curr_dma_words;
    val |= SLINK_DMA_BLOCK_SIZE(cur_words - 1);
    tegra_slink_writel(tspi, val, SLINK_DMA_CTL);
    tspi.dma_control_reg = val;
    tspi.is_curr_dma_xfer = false;
    if (tspi.is_packed) {
    val |= SLINK_PACKED;
    tegra_slink_writel(tspi, val, SLINK_DMA_CTL);
    udelay(1);
    wmb();
    }
    tspi.dma_control_reg = val;
    val |= SLINK_DMA_EN;
    tegra_slink_writel(tspi, val, SLINK_DMA_CTL);
    return 0;
    }
    static int tegra_slink_init_dma_param(struct tegra_slink_data *tspi,
    bool dma_to_memory)
    {
    struct dma_chan *dma_chan;
    u32 *dma_buf;
    dma_addr_t dma_phys;
    int ret;
    struct dma_slave_config dma_sconfig;
    dma_chan = dma_request_chan(tspi.dev, dma_to_memory ? "rx" : "tx");
    if (IS_ERR(dma_chan))
    return dev_err_probe(tspi.dev, PTR_ERR(dma_chan),
    "Dma channel is not available\n");
    dma_buf = dma_alloc_coherent(tspi.dev, tspi.dma_buf_size,
    &dma_phys, GFP_KERNEL);
    if (!dma_buf) {
    dev_err(tspi.dev, " Not able to allocate the dma buffer\n");
    dma_release_channel(dma_chan);
    return -ENOMEM;
    }
    if (dma_to_memory) {
    dma_sconfig.src_addr = tspi.phys + SLINK_RX_FIFO;
    dma_sconfig.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    dma_sconfig.src_maxburst = 0;
    } else {
    dma_sconfig.dst_addr = tspi.phys + SLINK_TX_FIFO;
    dma_sconfig.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    dma_sconfig.dst_maxburst = 0;
    }
    ret = dmaengine_slave_config(dma_chan, &dma_sconfig);
    if (ret)
    goto scrub;
    if (dma_to_memory) {
    tspi.rx_dma_chan = dma_chan;
    tspi.rx_dma_buf = dma_buf;
    tspi.rx_dma_phys = dma_phys;
    } else {
    tspi.tx_dma_chan = dma_chan;
    tspi.tx_dma_buf = dma_buf;
    tspi.tx_dma_phys = dma_phys;
    }
    return 0;
    scrub:
    dma_free_coherent(tspi.dev, tspi.dma_buf_size, dma_buf, dma_phys);
    dma_release_channel(dma_chan);
    return ret;
    }
    static void tegra_slink_deinit_dma_param(struct tegra_slink_data *tspi,
    bool dma_to_memory)
    {
    u32 *dma_buf;
    dma_addr_t dma_phys;
    struct dma_chan *dma_chan;
    if (dma_to_memory) {
    dma_buf = tspi.rx_dma_buf;
    dma_chan = tspi.rx_dma_chan;
    dma_phys = tspi.rx_dma_phys;
    tspi.rx_dma_chan = core::ptr::null_mut();
    tspi.rx_dma_buf = core::ptr::null_mut();
    } else {
    dma_buf = tspi.tx_dma_buf;
    dma_chan = tspi.tx_dma_chan;
    dma_phys = tspi.tx_dma_phys;
    tspi.tx_dma_buf = core::ptr::null_mut();
    tspi.tx_dma_chan = core::ptr::null_mut();
    }
    if (!dma_chan)
    return;
    dma_free_coherent(tspi.dev, tspi.dma_buf_size, dma_buf, dma_phys);
    dma_release_channel(dma_chan);
    }
    static int tegra_slink_start_transfer_one(struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct tegra_slink_data *tspi = spi_controller_get_devdata(spi.controller);
    u32 speed;
    u8 bits_per_word;
    unsigned total_fifo_words;
    int ret;
    u32 command;
    u32 command2;
    bits_per_word = t.bits_per_word;
    speed = t.speed_hz;
    if (speed != tspi.cur_speed) {
    dev_pm_opp_set_rate(tspi.dev, speed * 4);
    tspi.cur_speed = speed;
    }
    tspi.cur_spi = spi;
    tspi.cur_pos = 0;
    tspi.cur_rx_pos = 0;
    tspi.cur_tx_pos = 0;
    tspi.curr_xfer = t;
    total_fifo_words = tegra_slink_calculate_curr_xfer_param(spi, tspi, t);
    command = tspi.command_reg;
    command &= ~SLINK_BIT_LENGTH(~0);
    command |= SLINK_BIT_LENGTH(bits_per_word - 1);
    command2 = tspi.command2_reg;
    command2 &= ~(SLINK_RXEN | SLINK_TXEN);
    tspi.cur_direction = 0;
    if (t.rx_buf) {
    command2 |= SLINK_RXEN;
    tspi.cur_direction |= DATA_DIR_RX;
    }
    if (t.tx_buf) {
    command2 |= SLINK_TXEN;
    tspi.cur_direction |= DATA_DIR_TX;
    }
//
// Writing to the command2 register bevore the command register prevents
// a spike in chip_select line 0. This selects the chip_select line
// before changing the chip_select value.
//
    tegra_slink_writel(tspi, command2, SLINK_COMMAND2);
    tspi.command2_reg = command2;
    tegra_slink_writel(tspi, command, SLINK_COMMAND);
    tspi.command_reg = command;
    if (total_fifo_words > SLINK_FIFO_DEPTH)
    ret = tegra_slink_start_dma_based_transfer(tspi, t);
    else
    ret = tegra_slink_start_cpu_based_transfer(tspi, t);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tegra_slink_setup(spi: *mut spi_device) -> c_int {
    static int tegra_slink_setup(struct spi_device *spi)
    {
    static const u32 cs_pol_bit[MAX_CHIP_SELECT] = {
    SLINK_CS_POLARITY,
    SLINK_CS_POLARITY1,
    SLINK_CS_POLARITY2,
    SLINK_CS_POLARITY3,
    };
    struct tegra_slink_data *tspi = spi_controller_get_devdata(spi.controller);
    u32 val;
    unsigned long flags;
    int ret;
    dev_dbg(&spi.dev, "setup %d bpw, %scpol, %scpha, %dHz\n",
    spi.bits_per_word,
    spi.mode & SPI_CPOL ? "" : "~",
    spi.mode & SPI_CPHA ? "" : "~",
    spi.max_speed_hz);
    ret = pm_runtime_resume_and_get(tspi.dev);
    if (ret < 0) {
    dev_err(tspi.dev, "pm runtime failed, e = %d\n", ret);
    return ret;
    }
    spin_lock_irqsave(&tspi.lock, flags);
    val = tspi.def_command_reg;
    if (spi.mode & SPI_CS_HIGH)
    val |= cs_pol_bit[spi_get_chipselect(spi, 0)];
    else
    val &= ~cs_pol_bit[spi_get_chipselect(spi, 0)];
    tspi.def_command_reg = val;
    tegra_slink_writel(tspi, tspi.def_command_reg, SLINK_COMMAND);
    spin_unlock_irqrestore(&tspi.lock, flags);
    pm_runtime_put(tspi.dev);
    return 0;
    }
    static int tegra_slink_prepare_message(struct spi_controller *host,
    struct spi_message *msg)
    {
    struct tegra_slink_data *tspi = spi_controller_get_devdata(host);
    struct spi_device *spi = msg.spi;
    tegra_slink_clear_status(tspi);
    tspi.command_reg = tspi.def_command_reg;
    tspi.command_reg |= SLINK_CS_SW | SLINK_CS_VALUE;
    tspi.command2_reg = tspi.def_command2_reg;
    tspi.command2_reg |= SLINK_SS_EN_CS(spi_get_chipselect(spi, 0));
    tspi.command_reg &= ~SLINK_MODES;
    if (spi.mode & SPI_CPHA)
    tspi.command_reg |= SLINK_CK_SDA;
    if (spi.mode & SPI_CPOL)
    tspi.command_reg |= SLINK_IDLE_SCLK_DRIVE_HIGH;
    else
    tspi.command_reg |= SLINK_IDLE_SCLK_DRIVE_LOW;
    return 0;
    }
    static int tegra_slink_transfer_one(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct tegra_slink_data *tspi = spi_controller_get_devdata(host);
    int ret;
    reinit_completion(&tspi.xfer_completion);
    ret = tegra_slink_start_transfer_one(spi, xfer);
    if (ret < 0) {
    dev_err(tspi.dev,
    "spi can not start transfer, err %d\n", ret);
    return ret;
    }
    ret = wait_for_completion_timeout(&tspi.xfer_completion,
    SLINK_DMA_TIMEOUT);
    if (WARN_ON(ret == 0)) {
    dev_err(tspi.dev,
    "spi transfer timeout, err %d\n", ret);
    return -EIO;
    }
    if (tspi.tx_status)
    return tspi.tx_status;
    if (tspi.rx_status)
    return tspi.rx_status;
    return 0;
    }
    static int tegra_slink_unprepare_message(struct spi_controller *host,
    struct spi_message *msg)
    {
    struct tegra_slink_data *tspi = spi_controller_get_devdata(host);
    tegra_slink_writel(tspi, tspi.def_command_reg, SLINK_COMMAND);
    tegra_slink_writel(tspi, tspi.def_command2_reg, SLINK_COMMAND2);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn handle_cpu_based_xfer(tspi: *mut tegra_slink_data) -> irqreturn_t {
    static irqreturn_t handle_cpu_based_xfer(struct tegra_slink_data *tspi)
    {
    struct spi_transfer *t = tspi.curr_xfer;
    unsigned long flags;
    spin_lock_irqsave(&tspi.lock, flags);
    if (tspi.tx_status ||  tspi.rx_status ||
    (tspi.status_reg & SLINK_BSY)) {
    dev_err(tspi.dev,
    "CpuXfer ERROR bit set 0x%x\n", tspi.status_reg);
    dev_err(tspi.dev,
    "CpuXfer 0x%08x:0x%08x:0x%08x\n", tspi.command_reg,
    tspi.command2_reg, tspi.dma_control_reg);
    reset_control_assert(tspi.rst);
    udelay(2);
    reset_control_deassert(tspi.rst);
    complete(&tspi.xfer_completion);
    goto exit;
    }
    if (tspi.cur_direction & DATA_DIR_RX)
    tegra_slink_read_rx_fifo_to_client_rxbuf(tspi, t);
    if (tspi.cur_direction & DATA_DIR_TX)
    tspi.cur_pos = tspi.cur_tx_pos;
    else
    tspi.cur_pos = tspi.cur_rx_pos;
    if (tspi.cur_pos == t.len) {
    complete(&tspi.xfer_completion);
    goto exit;
    }
    tegra_slink_calculate_curr_xfer_param(tspi.cur_spi, tspi, t);
    tegra_slink_start_cpu_based_transfer(tspi, t);
    exit:
    spin_unlock_irqrestore(&tspi.lock, flags);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn handle_dma_based_xfer(tspi: *mut tegra_slink_data) -> irqreturn_t {
    static irqreturn_t handle_dma_based_xfer(struct tegra_slink_data *tspi)
    {
    struct spi_transfer *t = tspi.curr_xfer;
    long wait_status;
    let mut err: c_int = 0;
    unsigned total_fifo_words;
    unsigned long flags;
// Abort dmas if any error
    if (tspi.cur_direction & DATA_DIR_TX) {
    if (tspi.tx_status) {
    dmaengine_terminate_all(tspi.tx_dma_chan);
    err += 1;
    } else {
    wait_status = wait_for_completion_interruptible_timeout(
    &tspi.tx_dma_complete, SLINK_DMA_TIMEOUT);
    if (wait_status <= 0) {
    dmaengine_terminate_all(tspi.tx_dma_chan);
    dev_err(tspi.dev, "TxDma Xfer failed\n");
    err += 1;
    }
    }
    }
    if (tspi.cur_direction & DATA_DIR_RX) {
    if (tspi.rx_status) {
    dmaengine_terminate_all(tspi.rx_dma_chan);
    err += 2;
    } else {
    wait_status = wait_for_completion_interruptible_timeout(
    &tspi.rx_dma_complete, SLINK_DMA_TIMEOUT);
    if (wait_status <= 0) {
    dmaengine_terminate_all(tspi.rx_dma_chan);
    dev_err(tspi.dev, "RxDma Xfer failed\n");
    err += 2;
    }
    }
    }
    spin_lock_irqsave(&tspi.lock, flags);
    if (err) {
    dev_err(tspi.dev,
    "DmaXfer: ERROR bit set 0x%x\n", tspi.status_reg);
    dev_err(tspi.dev,
    "DmaXfer 0x%08x:0x%08x:0x%08x\n", tspi.command_reg,
    tspi.command2_reg, tspi.dma_control_reg);
    reset_control_assert(tspi.rst);
    udelay(2);
    reset_control_assert(tspi.rst);
    complete(&tspi.xfer_completion);
    spin_unlock_irqrestore(&tspi.lock, flags);
    return IRQ_HANDLED;
    }
    if (tspi.cur_direction & DATA_DIR_RX)
    tegra_slink_copy_spi_rxbuf_to_client_rxbuf(tspi, t);
    if (tspi.cur_direction & DATA_DIR_TX)
    tspi.cur_pos = tspi.cur_tx_pos;
    else
    tspi.cur_pos = tspi.cur_rx_pos;
    if (tspi.cur_pos == t.len) {
    complete(&tspi.xfer_completion);
    goto exit;
    }
// Continue transfer in current message
    total_fifo_words = tegra_slink_calculate_curr_xfer_param(tspi.cur_spi,
    tspi, t);
    if (total_fifo_words > SLINK_FIFO_DEPTH)
    err = tegra_slink_start_dma_based_transfer(tspi, t);
    else
    err = tegra_slink_start_cpu_based_transfer(tspi, t);
    exit:
    spin_unlock_irqrestore(&tspi.lock, flags);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tegra_slink_isr_thread(irq: c_int, context_data: *mut c_void) -> irqreturn_t {
    static irqreturn_t tegra_slink_isr_thread(int irq, void *context_data)
    {
    struct tegra_slink_data *tspi = context_data;
    if (!tspi.is_curr_dma_xfer)
    return handle_cpu_based_xfer(tspi);
    return handle_dma_based_xfer(tspi);
    }
#[no_mangle]
unsafe extern "C" fn tegra_slink_isr(irq: c_int, context_data: *mut c_void) -> irqreturn_t {
    static irqreturn_t tegra_slink_isr(int irq, void *context_data)
    {
    struct tegra_slink_data *tspi = context_data;
    tspi.status_reg = tegra_slink_readl(tspi, SLINK_STATUS);
    if (tspi.cur_direction & DATA_DIR_TX)
    tspi.tx_status = tspi.status_reg &
    (SLINK_TX_OVF | SLINK_TX_UNF);
    if (tspi.cur_direction & DATA_DIR_RX)
    tspi.rx_status = tspi.status_reg &
    (SLINK_RX_OVF | SLINK_RX_UNF);
    tegra_slink_clear_status(tspi);
    return IRQ_WAKE_THREAD;
    }
    static const struct tegra_slink_chip_data tegra30_spi_cdata = {
    .cs_hold_time = true,
    };
    static const struct tegra_slink_chip_data tegra20_spi_cdata = {
    .cs_hold_time = false,
    };
    static const struct of_device_id tegra_slink_of_match[] = {
    { .compatible = "nvidia,tegra30-slink", .data = &tegra30_spi_cdata, },
    { .compatible = "nvidia,tegra20-slink", .data = &tegra20_spi_cdata, },
    {}
    };
    MODULE_DEVICE_TABLE(of, tegra_slink_of_match);
#[no_mangle]
unsafe extern "C" fn tegra_slink_probe(pdev: *mut platform_device) -> c_int {
    static int tegra_slink_probe(struct platform_device *pdev)
    {
    struct spi_controller	*host;
    struct tegra_slink_data	*tspi;
    struct resource		*r;
    int ret, spi_irq;
    const struct tegra_slink_chip_data *cdata = core::ptr::null_mut();
    cdata = of_device_get_match_data(&pdev.dev);
    host = devm_spi_alloc_host(&pdev.dev, sizeof(*tspi));
    if (!host) {
    dev_err(&pdev.dev, "host allocation failed\n");
    return -ENOMEM;
    }
// the spi->mode bits understood by this driver:
    host.mode_bits = SPI_CPOL | SPI_CPHA | SPI_CS_HIGH;
    host.setup = tegra_slink_setup;
    host.prepare_message = tegra_slink_prepare_message;
    host.transfer_one = tegra_slink_transfer_one;
    host.unprepare_message = tegra_slink_unprepare_message;
    host.auto_runtime_pm = true;
    host.num_chipselect = MAX_CHIP_SELECT;
    platform_set_drvdata(pdev, host);
    tspi = spi_controller_get_devdata(host);
    tspi.host = host;
    tspi.dev = &pdev.dev;
    tspi.chip_data = cdata;
    spin_lock_init(&tspi.lock);
    if (of_property_read_u32(tspi.dev.of_node, "spi-max-frequency",
    &host.max_speed_hz))
    host.max_speed_hz = 25000000; /* 25MHz */
    tspi.base = devm_platform_get_and_ioremap_resource(pdev, 0, &r);
    if (IS_ERR(tspi.base))
    return PTR_ERR(tspi.base);
    tspi.phys = r.start;
// disabled clock may cause interrupt storm upon request
    tspi.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(tspi.clk)) {
    ret = PTR_ERR(tspi.clk);
    return dev_err_probe(&pdev.dev, ret, "Can not get clock\n");
    }
    tspi.rst = devm_reset_control_get_exclusive(&pdev.dev, "spi");
    if (IS_ERR(tspi.rst)) {
    ret = PTR_ERR(tspi.rst);
    return dev_err_probe(&pdev.dev, ret, "can not get reset\n");
    }
    ret = devm_tegra_core_dev_init_opp_table_common(&pdev.dev);
    if (ret)
    return ret;
    tspi.max_buf_size = SLINK_FIFO_DEPTH << 2;
    tspi.dma_buf_size = DEFAULT_SPI_DMA_BUF_LEN;
    ret = tegra_slink_init_dma_param(tspi, true);
    if (ret < 0)
    return ret;
    ret = tegra_slink_init_dma_param(tspi, false);
    if (ret < 0)
    goto exit_rx_dma_free;
    tspi.max_buf_size = tspi.dma_buf_size;
    init_completion(&tspi.tx_dma_complete);
    init_completion(&tspi.rx_dma_complete);
    init_completion(&tspi.xfer_completion);
    pm_runtime_enable(&pdev.dev);
    ret = pm_runtime_resume_and_get(&pdev.dev);
    if (ret) {
    dev_err(&pdev.dev, "pm runtime get failed, e = %d\n", ret);
    goto exit_pm_disable;
    }
    reset_control_assert(tspi.rst);
    udelay(2);
    reset_control_deassert(tspi.rst);
    spi_irq = platform_get_irq(pdev, 0);
    if (spi_irq < 0) {
    ret = spi_irq;
    goto exit_pm_put;
    }
    tspi.irq = spi_irq;
    ret = request_threaded_irq(tspi.irq, tegra_slink_isr,
    tegra_slink_isr_thread, IRQF_ONESHOT,
    dev_name(&pdev.dev), tspi);
    if (ret < 0) {
    dev_err(&pdev.dev, "Failed to register ISR for IRQ %d\n",
    tspi.irq);
    goto exit_pm_put;
    }
    tspi.def_command_reg  = SLINK_M_S;
    tspi.def_command2_reg = SLINK_CS_ACTIVE_BETWEEN;
    tegra_slink_writel(tspi, tspi.def_command_reg, SLINK_COMMAND);
    tegra_slink_writel(tspi, tspi.def_command2_reg, SLINK_COMMAND2);
    ret = spi_register_controller(host);
    if (ret < 0) {
    dev_err(&pdev.dev, "can not register to host err %d\n", ret);
    goto exit_free_irq;
    }
    pm_runtime_put(&pdev.dev);
    return ret;
    exit_free_irq:
    free_irq(spi_irq, tspi);
    exit_pm_put:
    pm_runtime_put(&pdev.dev);
    exit_pm_disable:
    pm_runtime_force_suspend(&pdev.dev);
    tegra_slink_deinit_dma_param(tspi, false);
    exit_rx_dma_free:
    tegra_slink_deinit_dma_param(tspi, true);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tegra_slink_remove(pdev: *mut platform_device) {
    static void tegra_slink_remove(struct platform_device *pdev)
    {
    struct spi_controller *host = platform_get_drvdata(pdev);
    struct tegra_slink_data	*tspi = spi_controller_get_devdata(host);
    spi_unregister_controller(host);
    free_irq(tspi.irq, tspi);
    pm_runtime_force_suspend(&pdev.dev);
    if (tspi.tx_dma_chan)
    tegra_slink_deinit_dma_param(tspi, false);
    if (tspi.rx_dma_chan)
    tegra_slink_deinit_dma_param(tspi, true);
    }
#[no_mangle]
unsafe extern "C" fn tegra_slink_suspend(dev: *mut device) -> c_int {
    static int tegra_slink_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    return spi_controller_suspend(host);
    }
#[no_mangle]
unsafe extern "C" fn tegra_slink_resume(dev: *mut device) -> c_int {
    static int tegra_slink_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct tegra_slink_data *tspi = spi_controller_get_devdata(host);
    int ret;
    ret = pm_runtime_resume_and_get(dev);
    if (ret < 0) {
    dev_err(dev, "pm runtime failed, e = %d\n", ret);
    return ret;
    }
    tegra_slink_writel(tspi, tspi.command_reg, SLINK_COMMAND);
    tegra_slink_writel(tspi, tspi.command2_reg, SLINK_COMMAND2);
    pm_runtime_put(dev);
    return spi_controller_resume(host);
    }
#[no_mangle]
unsafe extern "C" fn tegra_slink_runtime_suspend(dev: *mut device) -> c_int {
    static int tegra_slink_runtime_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct tegra_slink_data *tspi = spi_controller_get_devdata(host);
// Flush all write which are in PPSB queue by reading back
    tegra_slink_readl(tspi, SLINK_MAS_DATA);
    clk_disable_unprepare(tspi.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_slink_runtime_resume(dev: *mut device) -> c_int {
    static int tegra_slink_runtime_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct tegra_slink_data *tspi = spi_controller_get_devdata(host);
    int ret;
    ret = clk_prepare_enable(tspi.clk);
    if (ret < 0) {
    dev_err(tspi.dev, "clk_prepare failed: %d\n", ret);
    return ret;
    }
    return 0;
    }
    static const struct dev_pm_ops slink_pm_ops = {
    RUNTIME_PM_OPS(tegra_slink_runtime_suspend,
    tegra_slink_runtime_resume, core::ptr::null_mut())
    SYSTEM_SLEEP_PM_OPS(tegra_slink_suspend, tegra_slink_resume)
    };
    static struct platform_driver tegra_slink_driver = {
    .driver = {
    .name		= "spi-tegra-slink",
    .pm		= pm_ptr(&slink_pm_ops),
    .of_match_table	= tegra_slink_of_match,
    },
    .probe =	tegra_slink_probe,
    .remove =	tegra_slink_remove,
    };
    module_platform_driver(tegra_slink_driver);
    MODULE_ALIAS("platform:spi-tegra-slink");
    MODULE_DESCRIPTION("NVIDIA Tegra20/Tegra30 SLINK Controller Driver");
    MODULE_AUTHOR("Laxman Dewangan <ldewangan@nvidia.com>");
    MODULE_LICENSE("GPL v2");
