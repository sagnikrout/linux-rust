//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-omap2-mcspi.c
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
// OMAP2 McSPI controller driver
//
// Copyright (C) 2005, 2006 Nokia Corporation
// Author:	Samuel Ortiz <samuel.ortiz@nokia.com> and
// Juha Yrjola <juha.yrjola@nokia.com>
//

pub const OMAP2_MCSPI_MAX_FREQ: c_int = 48000000;
pub const OMAP2_MCSPI_MAX_DIVIDER: c_int = 4096;
pub const OMAP2_MCSPI_MAX_FIFODEPTH: c_int = 64;
pub const OMAP2_MCSPI_MAX_FIFOWCNT: c_uint = 0xFFFF;
pub const SPI_AUTOSUSPEND_TIMEOUT: c_int = 2000;
pub const OMAP2_MCSPI_REVISION: c_uint = 0x00;
pub const OMAP2_MCSPI_SYSSTATUS: c_uint = 0x14;
pub const OMAP2_MCSPI_IRQSTATUS: c_uint = 0x18;
pub const OMAP2_MCSPI_IRQENABLE: c_uint = 0x1c;
pub const OMAP2_MCSPI_WAKEUPENABLE: c_uint = 0x20;
pub const OMAP2_MCSPI_SYST: c_uint = 0x24;
pub const OMAP2_MCSPI_MODULCTRL: c_uint = 0x28;
pub const OMAP2_MCSPI_XFERLEVEL: c_uint = 0x7c;
// per-channel banks, 0x14 bytes each, first is:
pub const OMAP2_MCSPI_CHCONF0: c_uint = 0x2c;
pub const OMAP2_MCSPI_CHSTAT0: c_uint = 0x30;
pub const OMAP2_MCSPI_CHCTRL0: c_uint = 0x34;
pub const OMAP2_MCSPI_TX0: c_uint = 0x38;
pub const OMAP2_MCSPI_RX0: c_uint = 0x3c;
// per-register bitmasks:

// We have 2 DMA channels per CS, one for RX and one for TX
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap2_mcspi_dma {
    pub dma_tx: *mut dma_chan,
    pub dma_rx: *mut dma_chan,
    pub dma_tx_completion: completion,
    pub dma_rx_completion: completion,
    pub dma_rx_ch_name: [c_char; 14],
    pub dma_tx_ch_name: [c_char; 14],
}

// use PIO for small transfers, avoiding DMA setup/teardown overhead and
// cache operations; better heuristics consider wordsize and bitrate.
//
pub const DMA_MIN_BYTES: c_int = 160;
//
// Used for context save and restore, structure members to be updated whenever
// corresponding registers are modified.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap2_mcspi_regs {
    pub modulctrl: u32,
    pub wakeupenable: u32,
    pub cs: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap2_mcspi {
    pub txdone: completion,
    pub ctlr: *mut spi_controller,
// Virtual base address of the controller
    pub base: *mut void __iomem,
    pub phys: c_ulong,
// SPI1 has 4 channels, while SPI2 has 2
    pub dma_channels: *mut omap2_mcspi_dma,
    pub dev: *mut device,
    pub ctx: omap2_mcspi_regs,
    pub ref_clk: *mut clk,
    pub fifo_depth: c_int,
    pub target_aborted: bool,
    pub pin_dir:1: c_uint,
    pub max_xfer_len: usize,
    pub ref_clk_hz: u32,
    pub use_multi_mode: bool,
    pub last_msg_kept_cs: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap2_mcspi_cs {
    pub base: *mut void __iomem,
    pub phys: c_ulong,
    pub word_len: c_int,
    pub mode: u16,
    pub node: list_head,
// Context save and restore shadow register
    pub chctrl0: u32 chconf0,,
}

    static inline void mcspi_write_reg(struct spi_controller *ctlr,
    int idx, u32 val)
    {
    struct omap2_mcspi *mcspi = spi_controller_get_devdata(ctlr);
    writel_relaxed(val, mcspi.base + idx);
    }
#[no_mangle]
pub unsafe extern "C" fn mcspi_read_reg(ctlr: *mut spi_controller, idx: c_int) -> u32 {
    static inline u32 mcspi_read_reg(struct spi_controller *ctlr, int idx)
    {
    struct omap2_mcspi *mcspi = spi_controller_get_devdata(ctlr);
    return readl_relaxed(mcspi.base + idx);
    }
    static inline void mcspi_write_cs_reg(const struct spi_device *spi,
    int idx, u32 val)
    {
    struct omap2_mcspi_cs	*cs = spi.controller_state;
    writel_relaxed(val, cs.base +  idx);
    }
#[no_mangle]
pub unsafe extern "C" fn mcspi_read_cs_reg(spi: *const spi_device, idx: c_int) -> u32 {
    static inline u32 mcspi_read_cs_reg(const struct spi_device *spi, int idx)
    {
    struct omap2_mcspi_cs	*cs = spi.controller_state;
    return readl_relaxed(cs.base + idx);
    }
#[no_mangle]
pub unsafe extern "C" fn mcspi_cached_chconf0(spi: *const spi_device) -> u32 {
    static inline u32 mcspi_cached_chconf0(const struct spi_device *spi)
    {
    struct omap2_mcspi_cs *cs = spi.controller_state;
    return cs.chconf0;
    }
#[no_mangle]
pub unsafe extern "C" fn mcspi_write_chconf0(spi: *const spi_device, val: u32) {
    static inline void mcspi_write_chconf0(const struct spi_device *spi, u32 val)
    {
    struct omap2_mcspi_cs *cs = spi.controller_state;
    cs.chconf0 = val;
    mcspi_write_cs_reg(spi, OMAP2_MCSPI_CHCONF0, val);
    mcspi_read_cs_reg(spi, OMAP2_MCSPI_CHCONF0);
    }
#[no_mangle]
pub unsafe extern "C" fn mcspi_bytes_per_word(word_len: c_int) -> c_int {
    static inline int mcspi_bytes_per_word(int word_len)
    {
    if (word_len <= 8)
    return 1;
#[no_mangle]
pub unsafe extern "C" fn if(16: word_len <=) -> else {
    else if (word_len <= 16)
    return 2;
    else /* word_len <= 32 */
    return 4;
    }
    static void omap2_mcspi_set_dma_req(const struct spi_device *spi,
    int is_read, int enable)
    {
    u32 l, rw;
    l = mcspi_cached_chconf0(spi);
    if (is_read) /* 1 is read, 0 write */
    rw = OMAP2_MCSPI_CHCONF_DMAR;
    else
    rw = OMAP2_MCSPI_CHCONF_DMAW;
    if (enable)
    l |= rw;
    else
    l &= ~rw;
    mcspi_write_chconf0(spi, l);
    }
#[no_mangle]
unsafe extern "C" fn omap2_mcspi_set_enable(spi: *const spi_device, enable: c_int) {
    static void omap2_mcspi_set_enable(const struct spi_device *spi, int enable)
    {
    struct omap2_mcspi_cs *cs = spi.controller_state;
    u32 l;
    l = cs.chctrl0;
    if (enable)
    l |= OMAP2_MCSPI_CHCTRL_EN;
    else
    l &= ~OMAP2_MCSPI_CHCTRL_EN;
    cs.chctrl0 = l;
    mcspi_write_cs_reg(spi, OMAP2_MCSPI_CHCTRL0, cs.chctrl0);
// Flash post-writes
    mcspi_read_cs_reg(spi, OMAP2_MCSPI_CHCTRL0);
    }
#[no_mangle]
unsafe extern "C" fn omap2_mcspi_set_cs(spi: *mut spi_device, enable: bool) {
    static void omap2_mcspi_set_cs(struct spi_device *spi, bool enable)
    {
    struct omap2_mcspi *mcspi = spi_controller_get_devdata(spi.controller);
    u32 l;
// The controller handles the inverted chip selects
// using the OMAP2_MCSPI_CHCONF_EPOL bit so revert
// the inversion from the core spi_set_cs function.
//
    if (spi.mode & SPI_CS_HIGH)
    enable = !enable;
    if (spi.controller_state) {
    let mut err: c_int = pm_runtime_resume_and_get(mcspi.dev);
    if (err < 0) {
    dev_err(mcspi.dev, "failed to get sync: %d\n", err);
    return;
    }
    l = mcspi_cached_chconf0(spi);
// Only enable chip select manually if single mode is used
    if (mcspi.use_multi_mode) {
    l &= ~OMAP2_MCSPI_CHCONF_FORCE;
    } else {
    if (enable)
    l &= ~OMAP2_MCSPI_CHCONF_FORCE;
    else
    l |= OMAP2_MCSPI_CHCONF_FORCE;
    }
    mcspi_write_chconf0(spi, l);
    pm_runtime_put_autosuspend(mcspi.dev);
    }
    }
#[no_mangle]
unsafe extern "C" fn omap2_mcspi_set_mode(ctlr: *mut spi_controller) {
    static void omap2_mcspi_set_mode(struct spi_controller *ctlr)
    {
    struct omap2_mcspi	*mcspi = spi_controller_get_devdata(ctlr);
    struct omap2_mcspi_regs	*ctx = &mcspi.ctx;
    u32 l;
//
// Choose host or target mode
//
    l = mcspi_read_reg(ctlr, OMAP2_MCSPI_MODULCTRL);
    l &= ~(OMAP2_MCSPI_MODULCTRL_STEST);
    if (spi_controller_is_target(ctlr)) {
    l |= (OMAP2_MCSPI_MODULCTRL_MS);
    } else {
    l &= ~(OMAP2_MCSPI_MODULCTRL_MS);
// Enable single mode if needed
    if (mcspi.use_multi_mode)
    l &= ~OMAP2_MCSPI_MODULCTRL_SINGLE;
    else
    l |= OMAP2_MCSPI_MODULCTRL_SINGLE;
    }
    mcspi_write_reg(ctlr, OMAP2_MCSPI_MODULCTRL, l);
    ctx.modulctrl = l;
    }
    static void omap2_mcspi_set_fifo(const struct spi_device *spi,
    struct spi_transfer *t, int enable)
    {
    struct spi_controller *ctlr = spi.controller;
    struct omap2_mcspi_cs *cs = spi.controller_state;
    struct omap2_mcspi *mcspi;
    unsigned int wcnt;
    int max_fifo_depth, bytes_per_word;
    u32 chconf, xferlevel;
    mcspi = spi_controller_get_devdata(ctlr);
    chconf = mcspi_cached_chconf0(spi);
    if (enable) {
    bytes_per_word = mcspi_bytes_per_word(cs.word_len);
    if (t.len % bytes_per_word != 0)
    goto disable_fifo;
    if (t.rx_buf != core::ptr::null_mut() && t.tx_buf != core::ptr::null_mut())
    max_fifo_depth = OMAP2_MCSPI_MAX_FIFODEPTH / 2;
    else
    max_fifo_depth = OMAP2_MCSPI_MAX_FIFODEPTH;
    wcnt = t.len / bytes_per_word;
    if (wcnt > OMAP2_MCSPI_MAX_FIFOWCNT)
    goto disable_fifo;
    xferlevel = wcnt << 16;
    if (t.rx_buf != core::ptr::null_mut()) {
    chconf |= OMAP2_MCSPI_CHCONF_FFER;
    xferlevel |= (bytes_per_word - 1) << 8;
    }
    if (t.tx_buf != core::ptr::null_mut()) {
    chconf |= OMAP2_MCSPI_CHCONF_FFET;
    xferlevel |= bytes_per_word - 1;
    }
    mcspi_write_reg(ctlr, OMAP2_MCSPI_XFERLEVEL, xferlevel);
    mcspi_write_chconf0(spi, chconf);
    mcspi.fifo_depth = max_fifo_depth;
    return;
    }
    disable_fifo:
    if (t.rx_buf != core::ptr::null_mut())
    chconf &= ~OMAP2_MCSPI_CHCONF_FFER;
    if (t.tx_buf != core::ptr::null_mut())
    chconf &= ~OMAP2_MCSPI_CHCONF_FFET;
    mcspi_write_chconf0(spi, chconf);
    mcspi.fifo_depth = 0;
    }
#[no_mangle]
unsafe extern "C" fn mcspi_wait_for_reg_bit(reg: *mut void __iomem, bit: c_ulong) -> c_int {
    static int mcspi_wait_for_reg_bit(void __iomem *reg, unsigned long bit)
    {
    unsigned long timeout;
    timeout = jiffies + msecs_to_jiffies(1000);
    while (!(readl_relaxed(reg) & bit)) {
    if (time_after(jiffies, timeout)) {
    if (!(readl_relaxed(reg) & bit))
    return -ETIMEDOUT;
    else
    return 0;
    }
    cpu_relax();
    }
    return 0;
    }
    static int mcspi_wait_for_completion(struct  omap2_mcspi *mcspi,
    struct completion *x)
    {
    if (spi_controller_is_target(mcspi.ctlr)) {
    if (wait_for_completion_interruptible(x) ||
    mcspi.target_aborted)
    return -EINTR;
    } else {
    wait_for_completion(x);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap2_mcspi_rx_callback(data: *mut c_void) {
    static void omap2_mcspi_rx_callback(void *data)
    {
    struct spi_device *spi = data;
    struct omap2_mcspi *mcspi = spi_controller_get_devdata(spi.controller);
    struct omap2_mcspi_dma *mcspi_dma = &mcspi.dma_channels[spi_get_chipselect(spi, 0)];
// We must disable the DMA RX request
    omap2_mcspi_set_dma_req(spi, 1, 0);
    complete(&mcspi_dma.dma_rx_completion);
    }
#[no_mangle]
unsafe extern "C" fn omap2_mcspi_tx_callback(data: *mut c_void) {
    static void omap2_mcspi_tx_callback(void *data)
    {
    struct spi_device *spi = data;
    struct omap2_mcspi *mcspi = spi_controller_get_devdata(spi.controller);
    struct omap2_mcspi_dma *mcspi_dma = &mcspi.dma_channels[spi_get_chipselect(spi, 0)];
// We must disable the DMA TX request
    omap2_mcspi_set_dma_req(spi, 0, 0);
    complete(&mcspi_dma.dma_tx_completion);
    }
    static void omap2_mcspi_tx_dma(struct spi_device *spi,
    struct spi_transfer *xfer,
    struct dma_slave_config cfg)
    {
    struct omap2_mcspi	*mcspi;
    struct omap2_mcspi_dma  *mcspi_dma;
    struct dma_async_tx_descriptor *tx;
    mcspi = spi_controller_get_devdata(spi.controller);
    mcspi_dma = &mcspi.dma_channels[spi_get_chipselect(spi, 0)];
    dmaengine_slave_config(mcspi_dma.dma_tx, &cfg);
    tx = dmaengine_prep_slave_sg(mcspi_dma.dma_tx, xfer.tx_sg.sgl,
    xfer.tx_sg.nents,
    DMA_MEM_TO_DEV,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (tx) {
    tx.callback = omap2_mcspi_tx_callback;
    tx.callback_param = spi;
    dmaengine_submit(tx);
    } else {
// FIXME: fall back to PIO?
    }
    dma_async_issue_pending(mcspi_dma.dma_tx);
    omap2_mcspi_set_dma_req(spi, 0, 1);
    }
    static unsigned
    omap2_mcspi_rx_dma(struct spi_device *spi, struct spi_transfer *xfer,
    struct dma_slave_config cfg,
    unsigned es)
    {
    struct omap2_mcspi	*mcspi;
    struct omap2_mcspi_dma  *mcspi_dma;
    unsigned int		count, transfer_reduction = 0;
    struct scatterlist	*sg_out[2];
    let mut nb_sizes: c_int = 0, out_mapped_nents[2], ret, x;
    size_t			sizes[2];
    u32			l;
    let mut elements: c_int = 0;
    int			word_len, element_count;
    struct omap2_mcspi_cs	*cs = spi.controller_state;
    void __iomem		*chstat_reg = cs.base + OMAP2_MCSPI_CHSTAT0;
    struct dma_async_tx_descriptor *tx;
    mcspi = spi_controller_get_devdata(spi.controller);
    mcspi_dma = &mcspi.dma_channels[spi_get_chipselect(spi, 0)];
    count = xfer.len;
//
// In the "End-of-Transfer Procedure" section for DMA RX in OMAP35x TRM
// it mentions reducing DMA transfer length by one element in host
// normal mode.
//
    if (mcspi.fifo_depth == 0)
    transfer_reduction = es;
    word_len = cs.word_len;
    l = mcspi_cached_chconf0(spi);
    if (word_len <= 8)
    element_count = count;
#[no_mangle]
pub unsafe extern "C" fn if(16: word_len <=) -> else {
    else if (word_len <= 16)
    element_count = count >> 1;
    else /* word_len <= 32 */
    element_count = count >> 2;
    dmaengine_slave_config(mcspi_dma.dma_rx, &cfg);
//
// Reduce DMA transfer length by one more if McSPI is
// configured in turbo mode.
//
    if ((l & OMAP2_MCSPI_CHCONF_TURBO) && mcspi.fifo_depth == 0)
    transfer_reduction += es;
    if (transfer_reduction) {
// Split sgl into two. The second sgl won't be used.
    sizes[0] = count - transfer_reduction;
    sizes[1] = transfer_reduction;
    nb_sizes = 2;
    } else {
//
// Don't bother splitting the sgl. This essentially
// clones the original sgl.
//
    sizes[0] = count;
    nb_sizes = 1;
    }
    ret = sg_split(xfer.rx_sg.sgl, xfer.rx_sg.nents, 0, nb_sizes,
    sizes, sg_out, out_mapped_nents, GFP_KERNEL);
    if (ret < 0) {
    dev_err(&spi.dev, "sg_split failed\n");
    return 0;
    }
    tx = dmaengine_prep_slave_sg(mcspi_dma.dma_rx, sg_out[0],
    out_mapped_nents[0], DMA_DEV_TO_MEM,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (tx) {
    tx.callback = omap2_mcspi_rx_callback;
    tx.callback_param = spi;
    dmaengine_submit(tx);
    } else {
// FIXME: fall back to PIO?
    }
    dma_async_issue_pending(mcspi_dma.dma_rx);
    omap2_mcspi_set_dma_req(spi, 1, 1);
    ret = mcspi_wait_for_completion(mcspi, &mcspi_dma.dma_rx_completion);
    if (ret || mcspi.target_aborted) {
    dmaengine_terminate_sync(mcspi_dma.dma_rx);
    omap2_mcspi_set_dma_req(spi, 1, 0);
    return 0;
    }
    for (x = 0; x < nb_sizes; x++)
    kfree(sg_out[x]);
    if (mcspi.fifo_depth > 0)
    return count;
//
// Due to the DMA transfer length reduction the missing bytes must
// be read manually to receive all of the expected data.
//
    omap2_mcspi_set_enable(spi, 0);
    elements = element_count - 1;
    if (l & OMAP2_MCSPI_CHCONF_TURBO) {
    elements--;
    if (!mcspi_wait_for_reg_bit(chstat_reg,
    OMAP2_MCSPI_CHSTAT_RXS)) {
    u32 w;
    w = mcspi_read_cs_reg(spi, OMAP2_MCSPI_RX0);
    if (word_len <= 8)
    ((u8 *)xfer.rx_buf)[elements++] = w;
#[no_mangle]
pub unsafe extern "C" fn if(16: word_len <=) -> else {
    else if (word_len <= 16)
    ((u16 *)xfer.rx_buf)[elements++] = w;
    else /* word_len <= 32 */
    ((u32 *)xfer.rx_buf)[elements++] = w;
    } else {
    let mut bytes_per_word: c_int = mcspi_bytes_per_word(word_len);
    dev_err(&spi.dev, "DMA RX penultimate word empty\n");
    count -= (bytes_per_word << 1);
    omap2_mcspi_set_enable(spi, 1);
    return count;
    }
    }
    if (!mcspi_wait_for_reg_bit(chstat_reg, OMAP2_MCSPI_CHSTAT_RXS)) {
    u32 w;
    w = mcspi_read_cs_reg(spi, OMAP2_MCSPI_RX0);
    if (word_len <= 8)
    ((u8 *)xfer.rx_buf)[elements] = w;
#[no_mangle]
pub unsafe extern "C" fn if(16: word_len <=) -> else {
    else if (word_len <= 16)
    ((u16 *)xfer.rx_buf)[elements] = w;
    else /* word_len <= 32 */
    ((u32 *)xfer.rx_buf)[elements] = w;
    } else {
    dev_err(&spi.dev, "DMA RX last word empty\n");
    count -= mcspi_bytes_per_word(word_len);
    }
    omap2_mcspi_set_enable(spi, 1);
    return count;
    }
    static unsigned
    omap2_mcspi_txrx_dma(struct spi_device *spi, struct spi_transfer *xfer)
    {
    struct omap2_mcspi	*mcspi;
    struct omap2_mcspi_cs	*cs = spi.controller_state;
    struct omap2_mcspi_dma  *mcspi_dma;
    unsigned int		count;
    u8			*rx;
    const u8		*tx;
    struct dma_slave_config	cfg;
    enum dma_slave_buswidth width;
    unsigned es;
    void __iomem		*chstat_reg;
    void __iomem            *irqstat_reg;
    int			wait_res;
    mcspi = spi_controller_get_devdata(spi.controller);
    mcspi_dma = &mcspi.dma_channels[spi_get_chipselect(spi, 0)];
    if (cs.word_len <= 8) {
    width = DMA_SLAVE_BUSWIDTH_1_BYTE;
    es = 1;
    } else if (cs.word_len <= 16) {
    width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    es = 2;
    } else {
    width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    es = 4;
    }
    count = xfer.len;
    memset(&cfg, 0, sizeof(cfg));
    cfg.src_addr = cs.phys + OMAP2_MCSPI_RX0;
    cfg.dst_addr = cs.phys + OMAP2_MCSPI_TX0;
    cfg.src_addr_width = width;
    cfg.dst_addr_width = width;
    cfg.src_maxburst = 1;
    cfg.dst_maxburst = 1;
    rx = xfer.rx_buf;
    tx = xfer.tx_buf;
    mcspi.target_aborted = false;
    reinit_completion(&mcspi_dma.dma_tx_completion);
    reinit_completion(&mcspi_dma.dma_rx_completion);
    reinit_completion(&mcspi.txdone);
    if (tx) {
// Enable EOW IRQ to know end of tx in target mode
    if (spi_controller_is_target(spi.controller))
    mcspi_write_reg(spi.controller,
    OMAP2_MCSPI_IRQENABLE,
    OMAP2_MCSPI_IRQSTATUS_EOW);
    omap2_mcspi_tx_dma(spi, xfer, cfg);
    }
    if (rx != core::ptr::null_mut())
    count = omap2_mcspi_rx_dma(spi, xfer, cfg, es);
    if (tx != core::ptr::null_mut()) {
    int ret;
    ret = mcspi_wait_for_completion(mcspi, &mcspi_dma.dma_tx_completion);
    if (ret || mcspi.target_aborted) {
    dmaengine_terminate_sync(mcspi_dma.dma_tx);
    omap2_mcspi_set_dma_req(spi, 0, 0);
    return 0;
    }
    if (spi_controller_is_target(mcspi.ctlr)) {
    ret = mcspi_wait_for_completion(mcspi, &mcspi.txdone);
    if (ret || mcspi.target_aborted)
    return 0;
    }
    if (mcspi.fifo_depth > 0) {
    irqstat_reg = mcspi.base + OMAP2_MCSPI_IRQSTATUS;
    if (mcspi_wait_for_reg_bit(irqstat_reg,
    OMAP2_MCSPI_IRQSTATUS_EOW) < 0)
    dev_err(&spi.dev, "EOW timed out\n");
    mcspi_write_reg(mcspi.ctlr, OMAP2_MCSPI_IRQSTATUS,
    OMAP2_MCSPI_IRQSTATUS_EOW);
    }
// for TX_ONLY mode, be sure all words have shifted out
    if (rx == core::ptr::null_mut()) {
    chstat_reg = cs.base + OMAP2_MCSPI_CHSTAT0;
    if (mcspi.fifo_depth > 0) {
    wait_res = mcspi_wait_for_reg_bit(chstat_reg,
    OMAP2_MCSPI_CHSTAT_TXFFE);
    if (wait_res < 0)
    dev_err(&spi.dev, "TXFFE timed out\n");
    } else {
    wait_res = mcspi_wait_for_reg_bit(chstat_reg,
    OMAP2_MCSPI_CHSTAT_TXS);
    if (wait_res < 0)
    dev_err(&spi.dev, "TXS timed out\n");
    }
    if (wait_res >= 0 &&
    (mcspi_wait_for_reg_bit(chstat_reg,
    OMAP2_MCSPI_CHSTAT_EOT) < 0))
    dev_err(&spi.dev, "EOT timed out\n");
    }
    }
    return count;
    }
    static unsigned
    omap2_mcspi_txrx_pio(struct spi_device *spi, struct spi_transfer *xfer)
    {
    struct omap2_mcspi_cs	*cs = spi.controller_state;
    unsigned int		count, c;
    u32			l;
    void __iomem		*base = cs.base;
    void __iomem		*tx_reg;
    void __iomem		*rx_reg;
    void __iomem		*chstat_reg;
    int			word_len;
    count = xfer.len;
    c = count;
    word_len = cs.word_len;
    l = mcspi_cached_chconf0(spi);
// We store the pre-calculated register addresses on stack to speed
// up the transfer loop.
    tx_reg		= base + OMAP2_MCSPI_TX0;
    rx_reg		= base + OMAP2_MCSPI_RX0;
    chstat_reg	= base + OMAP2_MCSPI_CHSTAT0;
    if (c < (word_len>>3))
    return 0;
    if (word_len <= 8) {
    u8		*rx;
    const u8	*tx;
    rx = xfer.rx_buf;
    tx = xfer.tx_buf;
    do {
    c -= 1;
    if (tx != core::ptr::null_mut()) {
    if (mcspi_wait_for_reg_bit(chstat_reg,
    OMAP2_MCSPI_CHSTAT_TXS) < 0) {
    dev_err(&spi.dev, "TXS timed out\n");
    goto out;
    }
    dev_vdbg(&spi.dev, "write-%d %02x\n",
    word_len, *tx);
    writel_relaxed(*tx++, tx_reg);
    }
    if (rx != core::ptr::null_mut()) {
    if (mcspi_wait_for_reg_bit(chstat_reg,
    OMAP2_MCSPI_CHSTAT_RXS) < 0) {
    dev_err(&spi.dev, "RXS timed out\n");
    goto out;
    }
    if (c == 1 && tx == core::ptr::null_mut() &&
    (l & OMAP2_MCSPI_CHCONF_TURBO)) {
    omap2_mcspi_set_enable(spi, 0);
// rx++ = readl_relaxed(rx_reg);
    dev_vdbg(&spi.dev, "read-%d %02x\n",
    word_len, *(rx - 1));
    if (mcspi_wait_for_reg_bit(chstat_reg,
    OMAP2_MCSPI_CHSTAT_RXS) < 0) {
    dev_err(&spi.dev,
    "RXS timed out\n");
    goto out;
    }
    c = 0;
    } else if (c == 0 && tx == core::ptr::null_mut()) {
    omap2_mcspi_set_enable(spi, 0);
    }
// rx++ = readl_relaxed(rx_reg);
    dev_vdbg(&spi.dev, "read-%d %02x\n",
    word_len, *(rx - 1));
    }
// Add word delay between each word
    spi_delay_exec(&xfer.word_delay, xfer);
    } while (c);
    } else if (word_len <= 16) {
    u16		*rx;
    const u16	*tx;
    rx = xfer.rx_buf;
    tx = xfer.tx_buf;
    do {
    c -= 2;
    if (tx != core::ptr::null_mut()) {
    if (mcspi_wait_for_reg_bit(chstat_reg,
    OMAP2_MCSPI_CHSTAT_TXS) < 0) {
    dev_err(&spi.dev, "TXS timed out\n");
    goto out;
    }
    dev_vdbg(&spi.dev, "write-%d %04x\n",
    word_len, *tx);
    writel_relaxed(*tx++, tx_reg);
    }
    if (rx != core::ptr::null_mut()) {
    if (mcspi_wait_for_reg_bit(chstat_reg,
    OMAP2_MCSPI_CHSTAT_RXS) < 0) {
    dev_err(&spi.dev, "RXS timed out\n");
    goto out;
    }
    if (c == 2 && tx == core::ptr::null_mut() &&
    (l & OMAP2_MCSPI_CHCONF_TURBO)) {
    omap2_mcspi_set_enable(spi, 0);
// rx++ = readl_relaxed(rx_reg);
    dev_vdbg(&spi.dev, "read-%d %04x\n",
    word_len, *(rx - 1));
    if (mcspi_wait_for_reg_bit(chstat_reg,
    OMAP2_MCSPI_CHSTAT_RXS) < 0) {
    dev_err(&spi.dev,
    "RXS timed out\n");
    goto out;
    }
    c = 0;
    } else if (c == 0 && tx == core::ptr::null_mut()) {
    omap2_mcspi_set_enable(spi, 0);
    }
// rx++ = readl_relaxed(rx_reg);
    dev_vdbg(&spi.dev, "read-%d %04x\n",
    word_len, *(rx - 1));
    }
// Add word delay between each word
    spi_delay_exec(&xfer.word_delay, xfer);
    } while (c >= 2);
    } else if (word_len <= 32) {
    u32		*rx;
    const u32	*tx;
    rx = xfer.rx_buf;
    tx = xfer.tx_buf;
    do {
    c -= 4;
    if (tx != core::ptr::null_mut()) {
    if (mcspi_wait_for_reg_bit(chstat_reg,
    OMAP2_MCSPI_CHSTAT_TXS) < 0) {
    dev_err(&spi.dev, "TXS timed out\n");
    goto out;
    }
    dev_vdbg(&spi.dev, "write-%d %08x\n",
    word_len, *tx);
    writel_relaxed(*tx++, tx_reg);
    }
    if (rx != core::ptr::null_mut()) {
    if (mcspi_wait_for_reg_bit(chstat_reg,
    OMAP2_MCSPI_CHSTAT_RXS) < 0) {
    dev_err(&spi.dev, "RXS timed out\n");
    goto out;
    }
    if (c == 4 && tx == core::ptr::null_mut() &&
    (l & OMAP2_MCSPI_CHCONF_TURBO)) {
    omap2_mcspi_set_enable(spi, 0);
// rx++ = readl_relaxed(rx_reg);
    dev_vdbg(&spi.dev, "read-%d %08x\n",
    word_len, *(rx - 1));
    if (mcspi_wait_for_reg_bit(chstat_reg,
    OMAP2_MCSPI_CHSTAT_RXS) < 0) {
    dev_err(&spi.dev,
    "RXS timed out\n");
    goto out;
    }
    c = 0;
    } else if (c == 0 && tx == core::ptr::null_mut()) {
    omap2_mcspi_set_enable(spi, 0);
    }
// rx++ = readl_relaxed(rx_reg);
    dev_vdbg(&spi.dev, "read-%d %08x\n",
    word_len, *(rx - 1));
    }
// Add word delay between each word
    spi_delay_exec(&xfer.word_delay, xfer);
    } while (c >= 4);
    }
// for TX_ONLY mode, be sure all words have shifted out
    if (xfer.rx_buf == core::ptr::null_mut()) {
    if (mcspi_wait_for_reg_bit(chstat_reg,
    OMAP2_MCSPI_CHSTAT_TXS) < 0) {
    dev_err(&spi.dev, "TXS timed out\n");
    } else if (mcspi_wait_for_reg_bit(chstat_reg,
    OMAP2_MCSPI_CHSTAT_EOT) < 0)
    dev_err(&spi.dev, "EOT timed out\n");
// disable chan to purge rx datas received in TX_ONLY transfer,
// otherwise these rx datas will affect the direct following
// RX_ONLY transfer.
//
    omap2_mcspi_set_enable(spi, 0);
    }
    out:
    omap2_mcspi_set_enable(spi, 1);
    return count - c;
    }
#[no_mangle]
unsafe extern "C" fn omap2_mcspi_calc_divisor(speed_hz: u32, ref_clk_hz: u32) -> u32 {
    static u32 omap2_mcspi_calc_divisor(u32 speed_hz, u32 ref_clk_hz)
    {
    u32 div;
    for (div = 0; div < 15; div++)
    if (speed_hz >= (ref_clk_hz >> div))
    return div;
    return 15;
    }
// called only when no transfer is active to this device
    static int omap2_mcspi_setup_transfer(struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct omap2_mcspi_cs *cs = spi.controller_state;
    struct omap2_mcspi *mcspi;
    u32 ref_clk_hz, l = 0, clkd = 0, div, extclk = 0, clkg = 0;
    let mut word_len: u8 = spi.bits_per_word;
    let mut speed_hz: u32 = spi.max_speed_hz;
    mcspi = spi_controller_get_devdata(spi.controller);
    if (t != core::ptr::null_mut() && t.bits_per_word)
    word_len = t.bits_per_word;
    cs.word_len = word_len;
    if (t && t.speed_hz)
    speed_hz = t.speed_hz;
    ref_clk_hz = mcspi.ref_clk_hz;
    speed_hz = min_t(u32, speed_hz, ref_clk_hz);
    if (speed_hz < (ref_clk_hz / OMAP2_MCSPI_MAX_DIVIDER)) {
    clkd = omap2_mcspi_calc_divisor(speed_hz, ref_clk_hz);
    speed_hz = ref_clk_hz >> clkd;
    clkg = 0;
    } else {
    div = (ref_clk_hz + speed_hz - 1) / speed_hz;
    speed_hz = ref_clk_hz / div;
    clkd = (div - 1) & 0xf;
    extclk = (div - 1) >> 4;
    clkg = OMAP2_MCSPI_CHCONF_CLKG;
    }
    l = mcspi_cached_chconf0(spi);
    if (spi.mode & SPI_3WIRE) {
    if (t && !t.tx_buf) {
    l &= ~OMAP2_MCSPI_CHCONF_IS;
    l |= OMAP2_MCSPI_CHCONF_DPE0;
    } else if (t && !t.rx_buf) {
    l |= OMAP2_MCSPI_CHCONF_IS;
    l &= ~OMAP2_MCSPI_CHCONF_DPE0;
    }
    l |= OMAP2_MCSPI_CHCONF_DPE1;
    } else if (mcspi.pin_dir == MCSPI_PINDIR_D0_IN_D1_OUT) {
    l &= ~OMAP2_MCSPI_CHCONF_IS;
    l &= ~OMAP2_MCSPI_CHCONF_DPE1;
    l |= OMAP2_MCSPI_CHCONF_DPE0;
    } else {
    l |= OMAP2_MCSPI_CHCONF_IS;
    l |= OMAP2_MCSPI_CHCONF_DPE1;
    l &= ~OMAP2_MCSPI_CHCONF_DPE0;
    }
// wordlength
    l &= ~OMAP2_MCSPI_CHCONF_WL_MASK;
    l |= (word_len - 1) << 7;
// set chipselect polarity; manage with FORCE
    if (!(spi.mode & SPI_CS_HIGH))
    l |= OMAP2_MCSPI_CHCONF_EPOL;	/* active-low; normal */
    else
    l &= ~OMAP2_MCSPI_CHCONF_EPOL;
// set clock divisor
    l &= ~OMAP2_MCSPI_CHCONF_CLKD_MASK;
    l |= clkd << 2;
// set clock granularity
    l &= ~OMAP2_MCSPI_CHCONF_CLKG;
    l |= clkg;
    if (clkg) {
    cs.chctrl0 &= ~OMAP2_MCSPI_CHCTRL_EXTCLK_MASK;
    cs.chctrl0 |= extclk << 8;
    mcspi_write_cs_reg(spi, OMAP2_MCSPI_CHCTRL0, cs.chctrl0);
    }
// set SPI mode 0..3
    if (spi.mode & SPI_CPOL)
    l |= OMAP2_MCSPI_CHCONF_POL;
    else
    l &= ~OMAP2_MCSPI_CHCONF_POL;
    if (spi.mode & SPI_CPHA)
    l |= OMAP2_MCSPI_CHCONF_PHA;
    else
    l &= ~OMAP2_MCSPI_CHCONF_PHA;
    mcspi_write_chconf0(spi, l | OMAP2_MCSPI_CHCONF_FORCE);
    mcspi_write_chconf0(spi, l);
    cs.mode = spi.mode;
    dev_dbg(&spi.dev, "setup: speed %d, sample %s edge, clk %s\n",
    speed_hz,
    (spi.mode & SPI_CPHA) ? "trailing" : "leading",
    (spi.mode & SPI_CPOL) ? "inverted" : "normal");
    return 0;
    }
//
// Note that we currently allow DMA only if we get a channel
// for both rx and tx. Otherwise we'll do PIO for both rx and tx.
//
    static int omap2_mcspi_request_dma(struct omap2_mcspi *mcspi,
    struct omap2_mcspi_dma *mcspi_dma)
    {
    let mut ret: c_int = 0;
    mcspi_dma.dma_rx = dma_request_chan(mcspi.dev,
    mcspi_dma.dma_rx_ch_name);
    if (IS_ERR(mcspi_dma.dma_rx)) {
    ret = PTR_ERR(mcspi_dma.dma_rx);
    mcspi_dma.dma_rx = core::ptr::null_mut();
    goto no_dma;
    }
    mcspi_dma.dma_tx = dma_request_chan(mcspi.dev,
    mcspi_dma.dma_tx_ch_name);
    if (IS_ERR(mcspi_dma.dma_tx)) {
    ret = PTR_ERR(mcspi_dma.dma_tx);
    mcspi_dma.dma_tx = core::ptr::null_mut();
    dma_release_channel(mcspi_dma.dma_rx);
    mcspi_dma.dma_rx = core::ptr::null_mut();
    }
    init_completion(&mcspi_dma.dma_rx_completion);
    init_completion(&mcspi_dma.dma_tx_completion);
    no_dma:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn omap2_mcspi_release_dma(ctlr: *mut spi_controller) {
    static void omap2_mcspi_release_dma(struct spi_controller *ctlr)
    {
    struct omap2_mcspi *mcspi = spi_controller_get_devdata(ctlr);
    struct omap2_mcspi_dma	*mcspi_dma;
    int i;
    for (i = 0; i < ctlr.num_chipselect; i++) {
    mcspi_dma = &mcspi.dma_channels[i];
    if (mcspi_dma.dma_rx) {
    dma_release_channel(mcspi_dma.dma_rx);
    mcspi_dma.dma_rx = core::ptr::null_mut();
    }
    if (mcspi_dma.dma_tx) {
    dma_release_channel(mcspi_dma.dma_tx);
    mcspi_dma.dma_tx = core::ptr::null_mut();
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn omap2_mcspi_cleanup(spi: *mut spi_device) {
    static void omap2_mcspi_cleanup(struct spi_device *spi)
    {
    struct omap2_mcspi_cs	*cs;
    if (spi.controller_state) {
// Unlink controller state from context save list
    cs = spi.controller_state;
    list_del(&cs.node);
    kfree(cs);
    }
    }
#[no_mangle]
unsafe extern "C" fn omap2_mcspi_setup(spi: *mut spi_device) -> c_int {
    static int omap2_mcspi_setup(struct spi_device *spi)
    {
    let mut initial_setup: bool = false;
    int			ret;
    struct omap2_mcspi	*mcspi = spi_controller_get_devdata(spi.controller);
    struct omap2_mcspi_regs	*ctx = &mcspi.ctx;
    struct omap2_mcspi_cs	*cs = spi.controller_state;
    if (!cs) {
    cs = kzalloc_obj(*cs);
    if (!cs)
    return -ENOMEM;
    cs.base = mcspi.base + spi_get_chipselect(spi, 0) * 0x14;
    cs.phys = mcspi.phys + spi_get_chipselect(spi, 0) * 0x14;
    cs.mode = 0;
    cs.chconf0 = 0;
    cs.chctrl0 = 0;
    spi.controller_state = cs;
// Link this to context save list
    list_add_tail(&cs.node, &ctx.cs);
    initial_setup = true;
    }
    ret = pm_runtime_resume_and_get(mcspi.dev);
    if (ret < 0) {
    if (initial_setup)
    omap2_mcspi_cleanup(spi);
    return ret;
    }
    ret = omap2_mcspi_setup_transfer(spi, core::ptr::null_mut());
    if (ret && initial_setup)
    omap2_mcspi_cleanup(spi);
    pm_runtime_put_autosuspend(mcspi.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn omap2_mcspi_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t omap2_mcspi_irq_handler(int irq, void *data)
    {
    struct omap2_mcspi *mcspi = data;
    u32 irqstat;
    irqstat	= mcspi_read_reg(mcspi.ctlr, OMAP2_MCSPI_IRQSTATUS);
    if (!irqstat)
    return IRQ_NONE;
// Disable IRQ and wakeup target xfer task
    mcspi_write_reg(mcspi.ctlr, OMAP2_MCSPI_IRQENABLE, 0);
    if (irqstat & OMAP2_MCSPI_IRQSTATUS_EOW)
    complete(&mcspi.txdone);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn omap2_mcspi_target_abort(ctlr: *mut spi_controller) -> c_int {
    static int omap2_mcspi_target_abort(struct spi_controller *ctlr)
    {
    struct omap2_mcspi *mcspi = spi_controller_get_devdata(ctlr);
    struct omap2_mcspi_dma *mcspi_dma = mcspi.dma_channels;
    mcspi.target_aborted = true;
    complete(&mcspi_dma.dma_rx_completion);
    complete(&mcspi_dma.dma_tx_completion);
    complete(&mcspi.txdone);
    return 0;
    }
    static int omap2_mcspi_transfer_one(struct spi_controller *ctlr,
    struct spi_device *spi,
    struct spi_transfer *t)
    {
// We only enable one channel at a time -- the one whose message is
// -- although this controller would gladly
// arbitrate among multiple channels.  This corresponds to "single
// channel" host mode.  As a side effect, we need to manage the
// chipselect with the FORCE bit ... CS != channel enable.
//
    struct omap2_mcspi		*mcspi;
    struct omap2_mcspi_dma		*mcspi_dma;
    struct omap2_mcspi_cs		*cs;
    struct omap2_mcspi_device_config *cd;
    let mut par_override: c_int = 0;
    let mut status: c_int = 0;
    u32				chconf;
    mcspi = spi_controller_get_devdata(ctlr);
    mcspi_dma = mcspi.dma_channels + spi_get_chipselect(spi, 0);
    cs = spi.controller_state;
    cd = spi.controller_data;
//
// The target driver could have changed spi->mode in which case
// it will be different from cs->mode (the current hardware setup).
// If so, set par_override (even though its not a parity issue) so
// omap2_mcspi_setup_transfer will be called to configure the hardware
// with the correct mode on the first iteration of the loop below.
//
    if (spi.mode != cs.mode)
    par_override = 1;
    omap2_mcspi_set_enable(spi, 0);
    if (spi_get_csgpiod(spi, 0))
    omap2_mcspi_set_cs(spi, spi.mode & SPI_CS_HIGH);
    if (par_override ||
    (spi.mode & SPI_3WIRE) ||
    (t.speed_hz != spi.max_speed_hz) ||
    (t.bits_per_word != spi.bits_per_word)) {
    par_override = 1;
    status = omap2_mcspi_setup_transfer(spi, t);
    if (status < 0)
    goto out;
    if (t.speed_hz == spi.max_speed_hz &&
    t.bits_per_word == spi.bits_per_word)
    par_override = 0;
    }
    chconf = mcspi_cached_chconf0(spi);
    chconf &= ~OMAP2_MCSPI_CHCONF_TRM_MASK;
    chconf &= ~OMAP2_MCSPI_CHCONF_TURBO;
    if (t.tx_buf == core::ptr::null_mut())
    chconf |= OMAP2_MCSPI_CHCONF_TRM_RX_ONLY;
#[no_mangle]
pub unsafe extern "C" fn if(NULL: t->rx_buf ==) -> else {
    else if (t.rx_buf == core::ptr::null_mut())
    chconf |= OMAP2_MCSPI_CHCONF_TRM_TX_ONLY;
    if (cd && cd.turbo_mode && t.tx_buf == core::ptr::null_mut()) {
// Turbo mode is for more than one word
    if (t.len > ((cs.word_len + 7) >> 3))
    chconf |= OMAP2_MCSPI_CHCONF_TURBO;
    }
    mcspi_write_chconf0(spi, chconf);
    if (t.len) {
    unsigned	count;
    if ((mcspi_dma.dma_rx && mcspi_dma.dma_tx) &&
    spi_xfer_is_dma_mapped(ctlr, spi, t))
    omap2_mcspi_set_fifo(spi, t, 1);
    omap2_mcspi_set_enable(spi, 1);
// RX_ONLY mode needs dummy data in TX reg
    if (t.tx_buf == core::ptr::null_mut())
    writel_relaxed(0, cs.base
    + OMAP2_MCSPI_TX0);
    if ((mcspi_dma.dma_rx && mcspi_dma.dma_tx) &&
    spi_xfer_is_dma_mapped(ctlr, spi, t))
    count = omap2_mcspi_txrx_dma(spi, t);
    else
    count = omap2_mcspi_txrx_pio(spi, t);
    if (count != t.len) {
    status = -EIO;
    goto out;
    }
    }
    omap2_mcspi_set_enable(spi, 0);
    if (mcspi.fifo_depth > 0)
    omap2_mcspi_set_fifo(spi, t, 0);
    out:
// Restore defaults if they were overriden
    if (par_override) {
    par_override = 0;
    status = omap2_mcspi_setup_transfer(spi, core::ptr::null_mut());
    }
    omap2_mcspi_set_enable(spi, 0);
    if (spi_get_csgpiod(spi, 0))
    omap2_mcspi_set_cs(spi, !(spi.mode & SPI_CS_HIGH));
    if (mcspi.fifo_depth > 0 && t)
    omap2_mcspi_set_fifo(spi, t, 0);
    return status;
    }
    static int omap2_mcspi_prepare_message(struct spi_controller *ctlr,
    struct spi_message *msg)
    {
    struct omap2_mcspi	*mcspi = spi_controller_get_devdata(ctlr);
    struct omap2_mcspi_regs	*ctx = &mcspi.ctx;
    struct omap2_mcspi_cs	*cs;
    struct spi_transfer	*tr;
    u8 bits_per_word;
//
// The conditions are strict, it is mandatory to check each transfer of the list to see if
// multi-mode is applicable.
//
    mcspi.use_multi_mode = true;
    if (mcspi.last_msg_kept_cs)
    mcspi.use_multi_mode = false;
    list_for_each_entry(tr, &msg.transfers, transfer_list) {
    if (!tr.bits_per_word)
    bits_per_word = msg.spi.bits_per_word;
    else
    bits_per_word = tr.bits_per_word;
//
// Check if this transfer contains only one word;
//
    if (bits_per_word < 8 && tr.len == 1) {
// multi-mode is applicable, only one word (1..7 bits)
    } else if (bits_per_word >= 8 && tr.len == bits_per_word / 8) {
// multi-mode is applicable, only one word (8..32 bits)
    } else {
// multi-mode is not applicable: more than one word in the transfer
    mcspi.use_multi_mode = false;
    }
    if (list_is_last(&tr.transfer_list, &msg.transfers)) {
// Check if transfer asks to keep the CS status after the whole message
    if (tr.cs_change) {
    mcspi.use_multi_mode = false;
    mcspi.last_msg_kept_cs = true;
    } else {
    mcspi.last_msg_kept_cs = false;
    }
    } else {
// Check if transfer asks to change the CS status after the transfer
    if (!tr.cs_change)
    mcspi.use_multi_mode = false;
    }
    }
    omap2_mcspi_set_mode(ctlr);
// In single mode only a single channel can have the FORCE bit enabled
// in its chconf0 register.
// Scan all channels and disable them except the current one.
// A FORCE can remain from a last transfer having cs_change enabled
//
// In multi mode all FORCE bits must be disabled.
//
    list_for_each_entry(cs, &ctx.cs, node) {
    if (msg.spi.controller_state == cs && !mcspi.use_multi_mode) {
    continue;
    }
    if ((cs.chconf0 & OMAP2_MCSPI_CHCONF_FORCE)) {
    cs.chconf0 &= ~OMAP2_MCSPI_CHCONF_FORCE;
    writel_relaxed(cs.chconf0,
    cs.base + OMAP2_MCSPI_CHCONF0);
    readl_relaxed(cs.base + OMAP2_MCSPI_CHCONF0);
    }
    }
    return 0;
    }
    static bool omap2_mcspi_can_dma(struct spi_controller *ctlr,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct omap2_mcspi *mcspi = spi_controller_get_devdata(spi.controller);
    struct omap2_mcspi_dma *mcspi_dma =
    &mcspi.dma_channels[spi_get_chipselect(spi, 0)];
    if (!mcspi_dma.dma_rx || !mcspi_dma.dma_tx)
    return false;
    if (spi_controller_is_target(ctlr))
    return true;
    ctlr.dma_rx = mcspi_dma.dma_rx;
    ctlr.dma_tx = mcspi_dma.dma_tx;
    return (xfer.len >= DMA_MIN_BYTES);
    }
#[no_mangle]
unsafe extern "C" fn omap2_mcspi_max_xfer_size(spi: *mut spi_device) -> usize {
    static size_t omap2_mcspi_max_xfer_size(struct spi_device *spi)
    {
    struct omap2_mcspi *mcspi = spi_controller_get_devdata(spi.controller);
    struct omap2_mcspi_dma *mcspi_dma =
    &mcspi.dma_channels[spi_get_chipselect(spi, 0)];
    if (mcspi.max_xfer_len && mcspi_dma.dma_rx)
    return mcspi.max_xfer_len;
    return SIZE_MAX;
    }
#[no_mangle]
unsafe extern "C" fn omap2_mcspi_controller_setup(mcspi: *mut omap2_mcspi) -> c_int {
    static int omap2_mcspi_controller_setup(struct omap2_mcspi *mcspi)
    {
    struct spi_controller	*ctlr = mcspi.ctlr;
    struct omap2_mcspi_regs	*ctx = &mcspi.ctx;
    let mut ret: c_int = 0;
    ret = pm_runtime_resume_and_get(mcspi.dev);
    if (ret < 0)
    return ret;
    mcspi_write_reg(ctlr, OMAP2_MCSPI_WAKEUPENABLE,
    OMAP2_MCSPI_WAKEUPENABLE_WKEN);
    ctx.wakeupenable = OMAP2_MCSPI_WAKEUPENABLE_WKEN;
    omap2_mcspi_set_mode(ctlr);
    pm_runtime_put_autosuspend(mcspi.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_mcspi_runtime_suspend(dev: *mut device) -> c_int {
    static int omap_mcspi_runtime_suspend(struct device *dev)
    {
    int error;
    error = pinctrl_pm_select_idle_state(dev);
    if (error)
    dev_warn(dev, "%s: failed to set pins: %i\n", __func__, error);
    return 0;
    }
//
// When SPI wake up from off-mode, CS is in activate state. If it was in
// inactive state when driver was suspend, then force it to inactive state at
// wake up.
//
#[no_mangle]
unsafe extern "C" fn omap_mcspi_runtime_resume(dev: *mut device) -> c_int {
    static int omap_mcspi_runtime_resume(struct device *dev)
    {
    struct spi_controller *ctlr = dev_get_drvdata(dev);
    struct omap2_mcspi *mcspi = spi_controller_get_devdata(ctlr);
    struct omap2_mcspi_regs *ctx = &mcspi.ctx;
    struct omap2_mcspi_cs *cs;
    int error;
    error = pinctrl_pm_select_default_state(dev);
    if (error)
    dev_warn(dev, "%s: failed to set pins: %i\n", __func__, error);
// McSPI: context restore
    mcspi_write_reg(ctlr, OMAP2_MCSPI_MODULCTRL, ctx.modulctrl);
    mcspi_write_reg(ctlr, OMAP2_MCSPI_WAKEUPENABLE, ctx.wakeupenable);
    list_for_each_entry(cs, &ctx.cs, node) {
//
// We need to toggle CS state for OMAP take this
// change in account.
//
    if ((cs.chconf0 & OMAP2_MCSPI_CHCONF_FORCE) == 0) {
    cs.chconf0 |= OMAP2_MCSPI_CHCONF_FORCE;
    writel_relaxed(cs.chconf0,
    cs.base + OMAP2_MCSPI_CHCONF0);
    cs.chconf0 &= ~OMAP2_MCSPI_CHCONF_FORCE;
    writel_relaxed(cs.chconf0,
    cs.base + OMAP2_MCSPI_CHCONF0);
    } else {
    writel_relaxed(cs.chconf0,
    cs.base + OMAP2_MCSPI_CHCONF0);
    }
    }
    return 0;
    }
    static struct omap2_mcspi_platform_config omap2_pdata = {
    .regs_offset = 0,
    };
    static struct omap2_mcspi_platform_config omap4_pdata = {
    .regs_offset = OMAP4_MCSPI_REG_OFFSET,
    };
    static struct omap2_mcspi_platform_config am654_pdata = {
    .regs_offset = OMAP4_MCSPI_REG_OFFSET,
    .max_xfer_len = SZ_4K - 1,
    };
    static const struct of_device_id omap_mcspi_of_match[] = {
    {
    .compatible = "ti,omap2-mcspi",
    .data = &omap2_pdata,
    },
    {
    .compatible = "ti,omap4-mcspi",
    .data = &omap4_pdata,
    },
    {
    .compatible = "ti,am654-mcspi",
    .data = &am654_pdata,
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, omap_mcspi_of_match);
#[no_mangle]
unsafe extern "C" fn omap2_mcspi_probe(pdev: *mut platform_device) -> c_int {
    static int omap2_mcspi_probe(struct platform_device *pdev)
    {
    struct spi_controller	*ctlr;
    const struct omap2_mcspi_platform_config *pdata;
    struct omap2_mcspi	*mcspi;
    struct resource		*r;
    let mut status: c_int = 0, i;
    let mut regs_offset: u32 = 0;
    struct device_node	*node = pdev.dev.of_node;
    if (of_property_read_bool(node, "spi-slave"))
    ctlr = devm_spi_alloc_target(&pdev.dev, sizeof(*mcspi));
    else
    ctlr = devm_spi_alloc_host(&pdev.dev, sizeof(*mcspi));
    if (!ctlr)
    return -ENOMEM;
// the spi->mode bits understood by this driver:
    ctlr.mode_bits = SPI_CPOL | SPI_CPHA | SPI_CS_HIGH | SPI_3WIRE;
    ctlr.bits_per_word_mask = SPI_BPW_RANGE_MASK(4, 32);
    ctlr.setup = omap2_mcspi_setup;
    ctlr.auto_runtime_pm = true;
    ctlr.prepare_message = omap2_mcspi_prepare_message;
    ctlr.can_dma = omap2_mcspi_can_dma;
    ctlr.transfer_one = omap2_mcspi_transfer_one;
    ctlr.set_cs = omap2_mcspi_set_cs;
    ctlr.cleanup = omap2_mcspi_cleanup;
    ctlr.target_abort = omap2_mcspi_target_abort;
    ctlr.dev.of_node = node;
    ctlr.use_gpio_descriptors = true;
    platform_set_drvdata(pdev, ctlr);
    mcspi = spi_controller_get_devdata(ctlr);
    mcspi.ctlr = ctlr;
    pdata = of_device_get_match_data(&pdev.dev);
    if (pdata) {
    u32 num_cs = 1; /* default number of chipselect */
    of_property_read_u32(node, "ti,spi-num-cs", &num_cs);
    ctlr.num_chipselect = num_cs;
    if (of_property_read_bool(node, "ti,pindir-d0-out-d1-in"))
    mcspi.pin_dir = MCSPI_PINDIR_D0_OUT_D1_IN;
    } else {
    pdata = dev_get_platdata(&pdev.dev);
    ctlr.num_chipselect = pdata.num_cs;
    mcspi.pin_dir = pdata.pin_dir;
    }
    regs_offset = pdata.regs_offset;
    if (pdata.max_xfer_len) {
    mcspi.max_xfer_len = pdata.max_xfer_len;
    ctlr.max_transfer_size = omap2_mcspi_max_xfer_size;
    }
    mcspi.base = devm_platform_get_and_ioremap_resource(pdev, 0, &r);
    if (IS_ERR(mcspi.base))
    return PTR_ERR(mcspi.base);
    mcspi.phys = r.start + regs_offset;
    mcspi.base += regs_offset;
    mcspi.dev = &pdev.dev;
    INIT_LIST_HEAD(&mcspi.ctx.cs);
    mcspi.dma_channels = devm_kcalloc(&pdev.dev, ctlr.num_chipselect,
    sizeof(struct omap2_mcspi_dma),
    GFP_KERNEL);
    if (mcspi.dma_channels == core::ptr::null_mut())
    return -ENOMEM;
    for (i = 0; i < ctlr.num_chipselect; i++) {
    sprintf(mcspi.dma_channels[i].dma_rx_ch_name, "rx%d", i);
    sprintf(mcspi.dma_channels[i].dma_tx_ch_name, "tx%d", i);
    status = omap2_mcspi_request_dma(mcspi,
    &mcspi.dma_channels[i]);
    if (status == -EPROBE_DEFER)
    goto err_release_dma;
    }
    status = platform_get_irq(pdev, 0);
    if (status < 0)
    goto err_release_dma;
    init_completion(&mcspi.txdone);
    status = devm_request_irq(&pdev.dev, status,
    omap2_mcspi_irq_handler, 0, pdev.name,
    mcspi);
    if (status) {
    dev_err(&pdev.dev, "Cannot request IRQ");
    goto err_release_dma;
    }
    mcspi.ref_clk = devm_clk_get_optional_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(mcspi.ref_clk)) {
    status = PTR_ERR(mcspi.ref_clk);
    dev_err_probe(&pdev.dev, status, "Failed to get ref_clk");
    goto err_release_dma;
    }
    if (mcspi.ref_clk)
    mcspi.ref_clk_hz = clk_get_rate(mcspi.ref_clk);
    else
    mcspi.ref_clk_hz = OMAP2_MCSPI_MAX_FREQ;
    ctlr.max_speed_hz = mcspi.ref_clk_hz;
    ctlr.min_speed_hz = mcspi.ref_clk_hz >> 15;
    pm_runtime_use_autosuspend(&pdev.dev);
    pm_runtime_set_autosuspend_delay(&pdev.dev, SPI_AUTOSUSPEND_TIMEOUT);
    pm_runtime_enable(&pdev.dev);
    status = omap2_mcspi_controller_setup(mcspi);
    if (status < 0)
    goto err_disable_rpm;
    status = spi_register_controller(ctlr);
    if (status < 0)
    goto err_disable_rpm;
    return 0;
    err_disable_rpm:
    pm_runtime_dont_use_autosuspend(&pdev.dev);
    pm_runtime_put_sync(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    err_release_dma:
    omap2_mcspi_release_dma(ctlr);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn omap2_mcspi_remove(pdev: *mut platform_device) {
    static void omap2_mcspi_remove(struct platform_device *pdev)
    {
    struct spi_controller *ctlr = platform_get_drvdata(pdev);
    struct omap2_mcspi *mcspi = spi_controller_get_devdata(ctlr);
    spi_unregister_controller(ctlr);
    omap2_mcspi_release_dma(ctlr);
    pm_runtime_dont_use_autosuspend(mcspi.dev);
    pm_runtime_put_sync(mcspi.dev);
    pm_runtime_disable(&pdev.dev);
    }
// work with hotplug and coldplug
    MODULE_ALIAS("platform:omap2_mcspi");
#[no_mangle]
unsafe extern "C" fn omap2_mcspi_suspend(dev: *mut device) -> c_int {
    static int omap2_mcspi_suspend(struct device *dev)
    {
    struct spi_controller *ctlr = dev_get_drvdata(dev);
    struct omap2_mcspi *mcspi = spi_controller_get_devdata(ctlr);
    int error;
    error = pinctrl_pm_select_sleep_state(dev);
    if (error)
    dev_warn(mcspi.dev, "%s: failed to set pins: %i\n",
    __func__, error);
    error = spi_controller_suspend(ctlr);
    if (error)
    dev_warn(mcspi.dev, "%s: controller suspend failed: %i\n",
    __func__, error);
    return pm_runtime_force_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn omap2_mcspi_resume(dev: *mut device) -> c_int {
    static int omap2_mcspi_resume(struct device *dev)
    {
    struct spi_controller *ctlr = dev_get_drvdata(dev);
    struct omap2_mcspi *mcspi = spi_controller_get_devdata(ctlr);
    int error;
    error = spi_controller_resume(ctlr);
    if (error)
    dev_warn(mcspi.dev, "%s: controller resume failed: %i\n",
    __func__, error);
    return pm_runtime_force_resume(dev);
    }
    static const struct dev_pm_ops omap2_mcspi_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(omap2_mcspi_suspend, omap2_mcspi_resume)
    .runtime_suspend	= omap_mcspi_runtime_suspend,
    .runtime_resume		= omap_mcspi_runtime_resume,
    };
    static struct platform_driver omap2_mcspi_driver = {
    .driver = {
    .name =		"omap2_mcspi",
    .pm =		pm_ptr(&omap2_mcspi_pm_ops),
    .of_match_table = omap_mcspi_of_match,
    },
    .probe =	omap2_mcspi_probe,
    .remove =	omap2_mcspi_remove,
    };
    module_platform_driver(omap2_mcspi_driver);
    MODULE_DESCRIPTION("OMAP2 McSPI controller driver");
    MODULE_LICENSE("GPL");
