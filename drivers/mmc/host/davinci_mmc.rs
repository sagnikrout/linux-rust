//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/davinci_mmc.c
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
// davinci_mmc.c - TI DaVinci MMC/SD/SDIO driver
//
// Copyright (C) 2006 Texas Instruments.
// Original author: Purushotam Kumar
// Copyright (C) 2009 David Brownell
//

//
// Register Definitions
//
pub const DAVINCI_MMCCTL: c_uint = 0x00 /* Control Register                  */;
pub const DAVINCI_MMCCLK: c_uint = 0x04 /* Memory Clock Control Register     */;
pub const DAVINCI_MMCST0: c_uint = 0x08 /* Status Register 0                 */;
pub const DAVINCI_MMCST1: c_uint = 0x0C /* Status Register 1                 */;
pub const DAVINCI_MMCIM: c_uint = 0x10 /* Interrupt Mask Register           */;
pub const DAVINCI_MMCTOR: c_uint = 0x14 /* Response Time-Out Register        */;
pub const DAVINCI_MMCTOD: c_uint = 0x18 /* Data Read Time-Out Register       */;
pub const DAVINCI_MMCBLEN: c_uint = 0x1C /* Block Length Register             */;
pub const DAVINCI_MMCNBLK: c_uint = 0x20 /* Number of Blocks Register         */;
pub const DAVINCI_MMCNBLC: c_uint = 0x24 /* Number of Blocks Counter Register */;
pub const DAVINCI_MMCDRR: c_uint = 0x28 /* Data Receive Register             */;
pub const DAVINCI_MMCDXR: c_uint = 0x2C /* Data Transmit Register            */;
pub const DAVINCI_MMCCMD: c_uint = 0x30 /* Command Register                  */;
pub const DAVINCI_MMCARGHL: c_uint = 0x34 /* Argument Register                 */;
pub const DAVINCI_MMCRSP01: c_uint = 0x38 /* Response Register 0 and 1         */;
pub const DAVINCI_MMCRSP23: c_uint = 0x3C /* Response Register 0 and 1         */;
pub const DAVINCI_MMCRSP45: c_uint = 0x40 /* Response Register 0 and 1         */;
pub const DAVINCI_MMCRSP67: c_uint = 0x44 /* Response Register 0 and 1         */;
pub const DAVINCI_MMCDRSP: c_uint = 0x48 /* Data Response Register            */;
pub const DAVINCI_MMCETOK: c_uint = 0x4C;
pub const DAVINCI_MMCCIDX: c_uint = 0x50 /* Command Index Register            */;
pub const DAVINCI_MMCCKC: c_uint = 0x54;
pub const DAVINCI_MMCTORC: c_uint = 0x58;
pub const DAVINCI_MMCTODC: c_uint = 0x5C;
pub const DAVINCI_MMCBLNC: c_uint = 0x60;
pub const DAVINCI_SDIOCTL: c_uint = 0x64;
pub const DAVINCI_SDIOST0: c_uint = 0x68;
pub const DAVINCI_SDIOIEN: c_uint = 0x6C;
pub const DAVINCI_SDIOIST: c_uint = 0x70;
pub const DAVINCI_MMCFIFOCTL: c_uint = 0x74 /* FIFO Control Register             */;
// DAVINCI_MMCCTL definitions

// DAVINCI_MMCCLK definitions

// IRQ bit definitions, for DAVINCI_MMCST0 and DAVINCI_MMCIM

// DAVINCI_MMCST1 definitions

// DAVINCI_MMCCMD definitions

// DAVINCI_MMCFIFOCTL definitions

// DAVINCI_SDIOST0 definitions

// DAVINCI_SDIOIEN definitions

// DAVINCI_SDIOIST definitions

// MMCSD Init clock in Hz in opendrain mode
pub const MMCSD_INIT_CLOCK: c_int = 200000;
//
// One scatterlist dma "segment" is at most MAX_CCNT rw_threshold units,
// and we handle up to MAX_NR_SG segments.  MMC_BLOCK_BOUNCE kicks in only
// for drivers with max_segs == 1, making the segments bigger (64KB)
// than the page or two that's otherwise typical. nr_sg (passed from
// platform data) == 16 gives at least the same throughput boost, using
// EDMA transfer linkage instead of spending CPU time copying pages.
//

pub const MAX_NR_SG: c_int = 16;
    let mut rw_threshold: static unsigned = 32;
    module_param(rw_threshold, uint, 0444);
    MODULE_PARM_DESC(rw_threshold,
    "Read/Write threshold. Default = 32");
    let mut poll_threshold: static unsigned = 128;
    module_param(poll_threshold, uint, 0444);
    MODULE_PARM_DESC(poll_threshold,
    "Polling transaction size threshold. Default = 128");
    let mut poll_loopcount: static unsigned = 32;
    module_param(poll_loopcount, uint, 0444);
    MODULE_PARM_DESC(poll_loopcount,
    "Maximum polling loop count. Default = 32");
    let mut use_dma: static unsigned = 1;
    module_param(use_dma, uint, 0);
    MODULE_PARM_DESC(use_dma, "Whether to use DMA or not. Default = 1");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_davinci_host {
    pub cmd: *mut mmc_command,
    pub data: *mut mmc_data,
    pub mmc: *mut mmc_host,
    pub clk: *mut clk,
    pub mmc_input_clk: c_uint,
    pub base: *mut void __iomem,
    pub mem_res: *mut resource,
    pub sdio_irq: int mmc_irq,,
    pub bus_mode: c_uchar,
pub const DAVINCI_MMC_DATADIR_NONE: c_int = 0;
pub const DAVINCI_MMC_DATADIR_READ: c_int = 1;
pub const DAVINCI_MMC_DATADIR_WRITE: c_int = 2;
    pub data_dir: c_uchar,
    pub bytes_left: u32,
    pub dma_tx: *mut dma_chan,
    pub dma_rx: *mut dma_chan,
    pub use_dma: bool,
    pub do_dma: bool,
    pub sdio_int: bool,
    pub active_request: bool,
// For PIO we walk scatterlists one segment at a time.
    pub sg_miter: sg_mapping_iter,
    pub sg_len: c_uint,
// Version of the MMC/SD controller
    pub version: u8,
// for ns in one cycle calculation
    pub ns_in_one_cycle: unsigned,
// Number of sg segments
    pub nr_sg: u8,

    pub freq_transition: notifier_block,

}

    static irqreturn_t mmc_davinci_irq(int irq, void *dev_id);
// PIO only
    static void davinci_fifo_data_trans(struct mmc_davinci_host *host,
    unsigned int n)
    {
    struct sg_mapping_iter *sgm = &host.sg_miter;
    u8 *p;
    unsigned int i;
//
// By adjusting sgm->consumed this will give a pointer to the
// current index into the sgm.
//
    if (!sg_miter_next(sgm)) {
    dev_err(mmc_dev(host.mmc), "ran out of sglist prematurely\n");
    return;
    }
    p = sgm.addr;
    if (n > sgm.length)
    n = sgm.length;
// NOTE:  we never transfer more than rw_threshold bytes
// to/from the fifo here; there's no I/O overlap.
// This also assumes that access width( i.e. ACCWD) is 4 bytes
//
    if (host.data_dir == DAVINCI_MMC_DATADIR_WRITE) {
    for (i = 0; i < (n >> 2); i++) {
    writel(*((u32 *)p), host.base + DAVINCI_MMCDXR);
    p = p + 4;
    }
    if (n & 3) {
    iowrite8_rep(host.base + DAVINCI_MMCDXR, p, (n & 3));
    p = p + (n & 3);
    }
    } else {
    for (i = 0; i < (n >> 2); i++) {
// ((u32 *)p) = readl(host->base + DAVINCI_MMCDRR);
    p  = p + 4;
    }
    if (n & 3) {
    ioread8_rep(host.base + DAVINCI_MMCDRR, p, (n & 3));
    p = p + (n & 3);
    }
    }
    sgm.consumed = n;
    host.bytes_left -= n;
    }
    static void mmc_davinci_start_command(struct mmc_davinci_host *host,
    struct mmc_command *cmd)
    {
    let mut cmd_reg: u32 = 0;
    u32 im_val;
    dev_dbg(mmc_dev(host.mmc), "CMD%d, arg 0x%08x%s\n",
    cmd.opcode, cmd.arg,
    ({ char *s;
    switch (mmc_resp_type(cmd)) {
    case MMC_RSP_R1:
    s = ", R1/R5/R6/R7 response";
    break;
    case MMC_RSP_R1B:
    s = ", R1b response";
    break;
    case MMC_RSP_R2:
    s = ", R2 response";
    break;
    case MMC_RSP_R3:
    s = ", R3/R4 response";
    break;
    default:
    s = ", (R? response)";
    break;
    } s; }));
    host.cmd = cmd;
    switch (mmc_resp_type(cmd)) {
    case MMC_RSP_R1B:
// There's some spec confusion about when R1B is
// allowed, but if the card doesn't issue a BUSY
// then it's harmless for us to allow it.
//
    cmd_reg |= MMCCMD_BSYEXP;
    fallthrough;
    case MMC_RSP_R1:		/* 48 bits, CRC */
    cmd_reg |= MMCCMD_RSPFMT_R1456;
    break;
    case MMC_RSP_R2:		/* 136 bits, CRC */
    cmd_reg |= MMCCMD_RSPFMT_R2;
    break;
    case MMC_RSP_R3:		/* 48 bits, no CRC */
    cmd_reg |= MMCCMD_RSPFMT_R3;
    break;
    default:
    cmd_reg |= MMCCMD_RSPFMT_NONE;
    dev_dbg(mmc_dev(host.mmc), "unknown resp_type %04x\n",
    mmc_resp_type(cmd));
    break;
    }
// Set command index
    cmd_reg |= cmd.opcode;
// Enable EDMA transfer triggers
    if (host.do_dma)
    cmd_reg |= MMCCMD_DMATRIG;
    if (host.version == MMC_CTLR_VERSION_2 && host.data != core::ptr::null_mut() &&
    host.data_dir == DAVINCI_MMC_DATADIR_READ)
    cmd_reg |= MMCCMD_DMATRIG;
// Setting whether command involves data transfer or not
    if (cmd.data)
    cmd_reg |= MMCCMD_WDATX;
// Setting whether data read or write
    if (host.data_dir == DAVINCI_MMC_DATADIR_WRITE)
    cmd_reg |= MMCCMD_DTRW;
    if (host.bus_mode == MMC_BUSMODE_PUSHPULL)
    cmd_reg |= MMCCMD_PPLEN;
// set Command timeout
    writel(0x1FFF, host.base + DAVINCI_MMCTOR);
// Enable interrupt (calculate here, defer until FIFO is stuffed).
    im_val =  MMCST0_RSPDNE | MMCST0_CRCRS | MMCST0_TOUTRS;
    if (host.data_dir == DAVINCI_MMC_DATADIR_WRITE) {
    im_val |= MMCST0_DATDNE | MMCST0_CRCWR;
    if (!host.do_dma)
    im_val |= MMCST0_DXRDY;
    } else if (host.data_dir == DAVINCI_MMC_DATADIR_READ) {
    im_val |= MMCST0_DATDNE | MMCST0_CRCRD | MMCST0_TOUTRD;
    if (!host.do_dma)
    im_val |= MMCST0_DRRDY;
    }
//
// Before non-DMA WRITE commands the controller needs priming:
// FIFO should be populated with 32 bytes i.e. whatever is the FIFO size
//
    if (!host.do_dma && (host.data_dir == DAVINCI_MMC_DATADIR_WRITE))
    davinci_fifo_data_trans(host, rw_threshold);
    writel(cmd.arg, host.base + DAVINCI_MMCARGHL);
    writel(cmd_reg,  host.base + DAVINCI_MMCCMD);
    host.active_request = true;
    if (!host.do_dma && host.bytes_left <= poll_threshold) {
    let mut count: u32 = poll_loopcount;
    while (host.active_request && count--) {
    mmc_davinci_irq(0, host);
    cpu_relax();
    }
    }
    if (host.active_request)
    writel(im_val, host.base + DAVINCI_MMCIM);
    }
// ----------------------------------------------------------------------
// DMA infrastructure
#[no_mangle]
unsafe extern "C" fn davinci_abort_dma(host: *mut mmc_davinci_host) {
    static void davinci_abort_dma(struct mmc_davinci_host *host)
    {
    struct dma_chan *sync_dev;
    if (host.data_dir == DAVINCI_MMC_DATADIR_READ)
    sync_dev = host.dma_rx;
    else
    sync_dev = host.dma_tx;
    dmaengine_terminate_all(sync_dev);
    }
    static int mmc_davinci_send_dma_request(struct mmc_davinci_host *host,
    struct mmc_data *data)
    {
    struct dma_chan *chan;
    struct dma_async_tx_descriptor *desc;
    let mut ret: c_int = 0;
    if (host.data_dir == DAVINCI_MMC_DATADIR_WRITE) {
    struct dma_slave_config dma_tx_conf = {
    .direction = DMA_MEM_TO_DEV,
    .dst_addr = host.mem_res.start + DAVINCI_MMCDXR,
    .dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES,
    .dst_maxburst =
    rw_threshold / DMA_SLAVE_BUSWIDTH_4_BYTES,
    };
    chan = host.dma_tx;
    dmaengine_slave_config(host.dma_tx, &dma_tx_conf);
    desc = dmaengine_prep_slave_sg(host.dma_tx,
    data.sg,
    host.sg_len,
    DMA_MEM_TO_DEV,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!desc) {
    dev_dbg(mmc_dev(host.mmc),
    "failed to allocate DMA TX descriptor");
    ret = -1;
    goto out;
    }
    } else {
    struct dma_slave_config dma_rx_conf = {
    .direction = DMA_DEV_TO_MEM,
    .src_addr = host.mem_res.start + DAVINCI_MMCDRR,
    .src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES,
    .src_maxburst =
    rw_threshold / DMA_SLAVE_BUSWIDTH_4_BYTES,
    };
    chan = host.dma_rx;
    dmaengine_slave_config(host.dma_rx, &dma_rx_conf);
    desc = dmaengine_prep_slave_sg(host.dma_rx,
    data.sg,
    host.sg_len,
    DMA_DEV_TO_MEM,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!desc) {
    dev_dbg(mmc_dev(host.mmc),
    "failed to allocate DMA RX descriptor");
    ret = -1;
    goto out;
    }
    }
    dmaengine_submit(desc);
    dma_async_issue_pending(chan);
    out:
    return ret;
    }
    static int mmc_davinci_start_dma_transfer(struct mmc_davinci_host *host,
    struct mmc_data *data)
    {
    int i;
    let mut mask: c_int = rw_threshold - 1;
    let mut ret: c_int = 0;
    host.sg_len = dma_map_sg(mmc_dev(host.mmc), data.sg, data.sg_len,
    mmc_get_dma_dir(data));
// no individual DMA segment should need a partial FIFO
    for (i = 0; i < host.sg_len; i++) {
    if (sg_dma_len(data.sg + i) & mask) {
    dma_unmap_sg(mmc_dev(host.mmc),
    data.sg, data.sg_len,
    mmc_get_dma_dir(data));
    return -1;
    }
    }
    host.do_dma = 1;
    ret = mmc_davinci_send_dma_request(host, data);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn davinci_release_dma_channels(host: *mut mmc_davinci_host) {
    static void davinci_release_dma_channels(struct mmc_davinci_host *host)
    {
    if (!host.use_dma)
    return;
    dma_release_channel(host.dma_tx);
    dma_release_channel(host.dma_rx);
    }
#[no_mangle]
unsafe extern "C" fn davinci_acquire_dma_channels(host: *mut mmc_davinci_host) -> c_int {
    static int davinci_acquire_dma_channels(struct mmc_davinci_host *host)
    {
    host.dma_tx = dma_request_chan(mmc_dev(host.mmc), "tx");
    if (IS_ERR(host.dma_tx)) {
    dev_err(mmc_dev(host.mmc), "Can't get dma_tx channel\n");
    return PTR_ERR(host.dma_tx);
    }
    host.dma_rx = dma_request_chan(mmc_dev(host.mmc), "rx");
    if (IS_ERR(host.dma_rx)) {
    dev_err(mmc_dev(host.mmc), "Can't get dma_rx channel\n");
    dma_release_channel(host.dma_tx);
    return PTR_ERR(host.dma_rx);
    }
    return 0;
    }
// ----------------------------------------------------------------------
    static void
    mmc_davinci_prepare_data(struct mmc_davinci_host *host, struct mmc_request *req)
    {
    let mut fifo_lev: c_int = (rw_threshold == 32) ? MMCFIFOCTL_FIFOLEV : 0;
    int timeout;
    struct mmc_data *data = req.data;
    unsigned int flags = SG_MITER_ATOMIC; /* Used from IRQ */
    if (host.version == MMC_CTLR_VERSION_2)
    fifo_lev = (rw_threshold == 64) ? MMCFIFOCTL_FIFOLEV : 0;
    host.data = data;
    if (data == core::ptr::null_mut()) {
    host.data_dir = DAVINCI_MMC_DATADIR_NONE;
    writel(0, host.base + DAVINCI_MMCBLEN);
    writel(0, host.base + DAVINCI_MMCNBLK);
    return;
    }
    dev_dbg(mmc_dev(host.mmc), "%s, %d blocks of %d bytes\n",
    (data.flags & MMC_DATA_WRITE) ? "write" : "read",
    data.blocks, data.blksz);
    dev_dbg(mmc_dev(host.mmc), "  DTO %d cycles + %d ns\n",
    data.timeout_clks, data.timeout_ns);
    timeout = data.timeout_clks +
    (data.timeout_ns / host.ns_in_one_cycle);
    if (timeout > 0xffff)
    timeout = 0xffff;
    writel(timeout, host.base + DAVINCI_MMCTOD);
    writel(data.blocks, host.base + DAVINCI_MMCNBLK);
    writel(data.blksz, host.base + DAVINCI_MMCBLEN);
// Configure the FIFO
    if (data.flags & MMC_DATA_WRITE) {
    flags |= SG_MITER_FROM_SG;
    host.data_dir = DAVINCI_MMC_DATADIR_WRITE;
    writel(fifo_lev | MMCFIFOCTL_FIFODIR_WR | MMCFIFOCTL_FIFORST,
    host.base + DAVINCI_MMCFIFOCTL);
    writel(fifo_lev | MMCFIFOCTL_FIFODIR_WR,
    host.base + DAVINCI_MMCFIFOCTL);
    } else {
    flags |= SG_MITER_TO_SG;
    host.data_dir = DAVINCI_MMC_DATADIR_READ;
    writel(fifo_lev | MMCFIFOCTL_FIFODIR_RD | MMCFIFOCTL_FIFORST,
    host.base + DAVINCI_MMCFIFOCTL);
    writel(fifo_lev | MMCFIFOCTL_FIFODIR_RD,
    host.base + DAVINCI_MMCFIFOCTL);
    }
    host.bytes_left = data.blocks * data.blksz;
// For now we try to use DMA whenever we won't need partial FIFO
// reads or writes, either for the whole transfer (as tested here)
// or for any individual scatterlist segment (tested when we call
// start_dma_transfer).
//
// While we *could* change that, unusual block sizes are rarely
// used.  The occasional fallback to PIO should't hurt.
//
    if (host.use_dma && (host.bytes_left & (rw_threshold - 1)) == 0
    && mmc_davinci_start_dma_transfer(host, data) == 0) {
// zero this to ensure we take no PIO paths
    host.bytes_left = 0;
    } else {
// Revert to CPU Copy
    host.sg_len = data.sg_len;
    sg_miter_start(&host.sg_miter, data.sg, data.sg_len, flags);
    }
    }
#[no_mangle]
unsafe extern "C" fn mmc_davinci_request(mmc: *mut mmc_host, req: *mut mmc_request) {
    static void mmc_davinci_request(struct mmc_host *mmc, struct mmc_request *req)
    {
    struct mmc_davinci_host *host = mmc_priv(mmc);
    let mut timeout: c_ulong = jiffies + msecs_to_jiffies(900);
    let mut mmcst1: u32 = 0;
// Card may still be sending BUSY after a previous operation,
// typically some kind of write.  If so, we can't proceed yet.
//
    while (time_before(jiffies, timeout)) {
    mmcst1  = readl(host.base + DAVINCI_MMCST1);
    if (!(mmcst1 & MMCST1_BUSY))
    break;
    cpu_relax();
    }
    if (mmcst1 & MMCST1_BUSY) {
    dev_err(mmc_dev(host.mmc), "still BUSY? bad ...\n");
    req.cmd.error = -ETIMEDOUT;
    mmc_request_done(mmc, req);
    return;
    }
    host.do_dma = 0;
    mmc_davinci_prepare_data(host, req);
    mmc_davinci_start_command(host, req.cmd);
    }
    static unsigned int calculate_freq_for_card(struct mmc_davinci_host *host,
    unsigned int mmc_req_freq)
    {
    let mut mmc_freq: c_uint = 0, mmc_pclk = 0, mmc_push_pull_divisor = 0;
    mmc_pclk = host.mmc_input_clk;
    if (mmc_req_freq && mmc_pclk > (2 * mmc_req_freq))
    mmc_push_pull_divisor = ((unsigned int)mmc_pclk
    / (2 * mmc_req_freq)) - 1;
    else
    mmc_push_pull_divisor = 0;
    mmc_freq = (unsigned int)mmc_pclk
    / (2 * (mmc_push_pull_divisor + 1));
    if (mmc_freq > mmc_req_freq)
    mmc_push_pull_divisor = mmc_push_pull_divisor + 1;
// Convert ns to clock cycles
    if (mmc_req_freq <= 400000)
    host.ns_in_one_cycle = (1000000) / (((mmc_pclk
    / (2 * (mmc_push_pull_divisor + 1)))/1000));
    else
    host.ns_in_one_cycle = (1000000) / (((mmc_pclk
    / (2 * (mmc_push_pull_divisor + 1)))/1000000));
    return mmc_push_pull_divisor;
    }
#[no_mangle]
unsafe extern "C" fn calculate_clk_divider(mmc: *mut mmc_host, ios: *mut mmc_ios) {
    static void calculate_clk_divider(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    let mut open_drain_freq: c_uint = 0, mmc_pclk = 0;
    let mut mmc_push_pull_freq: c_uint = 0;
    struct mmc_davinci_host *host = mmc_priv(mmc);
    if (ios.bus_mode == MMC_BUSMODE_OPENDRAIN) {
    u32 temp;
// Ignoring the init clock value passed for fixing the inter
// operability with different cards.
//
    open_drain_freq = ((unsigned int)mmc_pclk
    / (2 * MMCSD_INIT_CLOCK)) - 1;
    if (open_drain_freq > 0xFF)
    open_drain_freq = 0xFF;
    temp = readl(host.base + DAVINCI_MMCCLK) & ~MMCCLK_CLKRT_MASK;
    temp |= open_drain_freq;
    writel(temp, host.base + DAVINCI_MMCCLK);
// Convert ns to clock cycles
    host.ns_in_one_cycle = (1000000) / (MMCSD_INIT_CLOCK/1000);
    } else {
    u32 temp;
    mmc_push_pull_freq = calculate_freq_for_card(host, ios.clock);
    if (mmc_push_pull_freq > 0xFF)
    mmc_push_pull_freq = 0xFF;
    temp = readl(host.base + DAVINCI_MMCCLK) & ~MMCCLK_CLKEN;
    writel(temp, host.base + DAVINCI_MMCCLK);
    udelay(10);
    temp = readl(host.base + DAVINCI_MMCCLK) & ~MMCCLK_CLKRT_MASK;
    temp |= mmc_push_pull_freq;
    writel(temp, host.base + DAVINCI_MMCCLK);
    writel(temp | MMCCLK_CLKEN, host.base + DAVINCI_MMCCLK);
    udelay(10);
    }
    }
#[no_mangle]
unsafe extern "C" fn mmc_davinci_set_ios(mmc: *mut mmc_host, ios: *mut mmc_ios) {
    static void mmc_davinci_set_ios(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    struct mmc_davinci_host *host = mmc_priv(mmc);
    struct platform_device *pdev = to_platform_device(mmc.parent);
    struct davinci_mmc_config *config = pdev.dev.platform_data;
    dev_dbg(mmc_dev(host.mmc),
    "clock %dHz busmode %d powermode %d Vdd %04x\n",
    ios.clock, ios.bus_mode, ios.power_mode,
    ios.vdd);
    switch (ios.power_mode) {
    case MMC_POWER_OFF:
    if (config && config.set_power)
    config.set_power(pdev.id, false);
    break;
    case MMC_POWER_UP:
    if (config && config.set_power)
    config.set_power(pdev.id, true);
    break;
    }
    switch (ios.bus_width) {
    case MMC_BUS_WIDTH_8:
    dev_dbg(mmc_dev(host.mmc), "Enabling 8 bit mode\n");
    writel((readl(host.base + DAVINCI_MMCCTL) &
    ~MMCCTL_WIDTH_4_BIT) | MMCCTL_WIDTH_8_BIT,
    host.base + DAVINCI_MMCCTL);
    break;
    case MMC_BUS_WIDTH_4:
    dev_dbg(mmc_dev(host.mmc), "Enabling 4 bit mode\n");
    if (host.version == MMC_CTLR_VERSION_2)
    writel((readl(host.base + DAVINCI_MMCCTL) &
    ~MMCCTL_WIDTH_8_BIT) | MMCCTL_WIDTH_4_BIT,
    host.base + DAVINCI_MMCCTL);
    else
    writel(readl(host.base + DAVINCI_MMCCTL) |
    MMCCTL_WIDTH_4_BIT,
    host.base + DAVINCI_MMCCTL);
    break;
    case MMC_BUS_WIDTH_1:
    dev_dbg(mmc_dev(host.mmc), "Enabling 1 bit mode\n");
    if (host.version == MMC_CTLR_VERSION_2)
    writel(readl(host.base + DAVINCI_MMCCTL) &
    ~(MMCCTL_WIDTH_8_BIT | MMCCTL_WIDTH_4_BIT),
    host.base + DAVINCI_MMCCTL);
    else
    writel(readl(host.base + DAVINCI_MMCCTL) &
    ~MMCCTL_WIDTH_4_BIT,
    host.base + DAVINCI_MMCCTL);
    break;
    }
    calculate_clk_divider(mmc, ios);
    host.bus_mode = ios.bus_mode;
    if (ios.power_mode == MMC_POWER_UP) {
    let mut timeout: c_ulong = jiffies + msecs_to_jiffies(50);
    let mut lose: bool = true;
// Send clock cycles, poll completion
    writel(0, host.base + DAVINCI_MMCARGHL);
    writel(MMCCMD_INITCK, host.base + DAVINCI_MMCCMD);
    while (time_before(jiffies, timeout)) {
    let mut tmp: u32 = readl(host.base + DAVINCI_MMCST0);
    if (tmp & MMCST0_RSPDNE) {
    lose = false;
    break;
    }
    cpu_relax();
    }
    if (lose)
    dev_warn(mmc_dev(host.mmc), "powerup timeout\n");
    }
// FIXME on power OFF, reset things ...
    }
    static void
    mmc_davinci_xfer_done(struct mmc_davinci_host *host, struct mmc_data *data)
    {
    host.data = core::ptr::null_mut();
    if (host.mmc.caps & MMC_CAP_SDIO_IRQ) {
//
// SDIO Interrupt Detection work-around as suggested by
// Davinci Errata (TMS320DM355 Silicon Revision 1.1 Errata
// 2.1.6): Signal SDIO interrupt only if it is enabled by core
//
    if (host.sdio_int && !(readl(host.base + DAVINCI_SDIOST0) &
    SDIOST0_DAT1_HI)) {
    writel(SDIOIST_IOINT, host.base + DAVINCI_SDIOIST);
    mmc_signal_sdio_irq(host.mmc);
    }
    }
    if (host.do_dma) {
    davinci_abort_dma(host);
    dma_unmap_sg(mmc_dev(host.mmc), data.sg, data.sg_len,
    mmc_get_dma_dir(data));
    host.do_dma = false;
    }
    host.data_dir = DAVINCI_MMC_DATADIR_NONE;
    if (!data.stop || (host.cmd && host.cmd.error)) {
    mmc_request_done(host.mmc, data.mrq);
    writel(0, host.base + DAVINCI_MMCIM);
    host.active_request = false;
    } else
    mmc_davinci_start_command(host, data.stop);
    }
    static void mmc_davinci_cmd_done(struct mmc_davinci_host *host,
    struct mmc_command *cmd)
    {
    host.cmd = core::ptr::null_mut();
    if (cmd.flags & MMC_RSP_PRESENT) {
    if (cmd.flags & MMC_RSP_136) {
// response type 2
    cmd.resp[3] = readl(host.base + DAVINCI_MMCRSP01);
    cmd.resp[2] = readl(host.base + DAVINCI_MMCRSP23);
    cmd.resp[1] = readl(host.base + DAVINCI_MMCRSP45);
    cmd.resp[0] = readl(host.base + DAVINCI_MMCRSP67);
    } else {
// response types 1, 1b, 3, 4, 5, 6
    cmd.resp[0] = readl(host.base + DAVINCI_MMCRSP67);
    }
    }
    if (host.data == core::ptr::null_mut() || cmd.error) {
    if (cmd.error == -ETIMEDOUT)
    cmd.mrq.cmd.retries = 0;
    mmc_request_done(host.mmc, cmd.mrq);
    writel(0, host.base + DAVINCI_MMCIM);
    host.active_request = false;
    }
    }
    static inline void mmc_davinci_reset_ctrl(struct mmc_davinci_host *host,
    int val)
    {
    u32 temp;
    temp = readl(host.base + DAVINCI_MMCCTL);
    if (val)	/* reset */
    temp |= MMCCTL_CMDRST | MMCCTL_DATRST;
    else		/* enable */
    temp &= ~(MMCCTL_CMDRST | MMCCTL_DATRST);
    writel(temp, host.base + DAVINCI_MMCCTL);
    udelay(10);
    }
    static void
    davinci_abort_data(struct mmc_davinci_host *host, struct mmc_data *data)
    {
    mmc_davinci_reset_ctrl(host, 1);
    mmc_davinci_reset_ctrl(host, 0);
    if (!host.do_dma)
    sg_miter_stop(&host.sg_miter);
    }
#[no_mangle]
unsafe extern "C" fn mmc_davinci_sdio_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mmc_davinci_sdio_irq(int irq, void *dev_id)
    {
    struct mmc_davinci_host *host = dev_id;
    unsigned int status;
    status = readl(host.base + DAVINCI_SDIOIST);
    if (status & SDIOIST_IOINT) {
    dev_dbg(mmc_dev(host.mmc),
    "SDIO interrupt status %x\n", status);
    writel(status | SDIOIST_IOINT, host.base + DAVINCI_SDIOIST);
    mmc_signal_sdio_irq(host.mmc);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mmc_davinci_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mmc_davinci_irq(int irq, void *dev_id)
    {
    struct mmc_davinci_host *host = (struct mmc_davinci_host *)dev_id;
    unsigned int status, qstatus;
    let mut end_command: c_int = 0;
    let mut end_transfer: c_int = 0;
    struct mmc_data *data = host.data;
    if (host.cmd == core::ptr::null_mut() && host.data == core::ptr::null_mut()) {
    status = readl(host.base + DAVINCI_MMCST0);
    dev_dbg(mmc_dev(host.mmc),
    "Spurious interrupt 0x%04x\n", status);
// Disable the interrupt from mmcsd
    writel(0, host.base + DAVINCI_MMCIM);
    return IRQ_NONE;
    }
    status = readl(host.base + DAVINCI_MMCST0);
    qstatus = status;
// handle FIFO first when using PIO for data.
// bytes_left will decrease to zero as I/O progress and status will
// read zero over iteration because this controller status
// register(MMCST0) reports any status only once and it is cleared
// by read. So, it is not unbouned loop even in the case of
// non-dma.
//
    if (host.bytes_left && (status & (MMCST0_DXRDY | MMCST0_DRRDY))) {
    unsigned long im_val;
//
// If interrupts fire during the following loop, they will be
// handled by the handler, but the PIC will still buffer these.
// As a result, the handler will be called again to serve these
// needlessly. In order to avoid these spurious interrupts,
// keep interrupts masked during the loop.
//
    im_val = readl(host.base + DAVINCI_MMCIM);
    writel(0, host.base + DAVINCI_MMCIM);
    do {
    davinci_fifo_data_trans(host, rw_threshold);
    status = readl(host.base + DAVINCI_MMCST0);
    qstatus |= status;
    } while (host.bytes_left &&
    (status & (MMCST0_DXRDY | MMCST0_DRRDY)));
//
// If an interrupt is pending, it is assumed it will fire when
// it is unmasked. This assumption is also taken when the MMCIM
// is first set. Otherwise, writing to MMCIM after reading the
// status is race-prone.
//
    writel(im_val, host.base + DAVINCI_MMCIM);
    }
    if (qstatus & MMCST0_DATDNE) {
// All blocks sent/received, and CRC checks passed
    if (data != core::ptr::null_mut()) {
    if (!host.do_dma) {
    if (host.bytes_left > 0)
// if datasize < rw_threshold
// no RX ints are generated
//
    davinci_fifo_data_trans(host, host.bytes_left);
    sg_miter_stop(&host.sg_miter);
    }
    end_transfer = 1;
    data.bytes_xfered = data.blocks * data.blksz;
    } else {
    dev_err(mmc_dev(host.mmc),
    "DATDNE with no host.data\n");
    }
    }
    if (data && (qstatus & MMCST0_TOUTRD)) {
// Read data timeout
    data.error = -ETIMEDOUT;
    end_transfer = 1;
    dev_dbg(mmc_dev(host.mmc),
    "read data timeout, status %x\n",
    qstatus);
    davinci_abort_data(host, data);
    }
    if (data && (qstatus & (MMCST0_CRCWR | MMCST0_CRCRD))) {
// Data CRC error
    data.error = -EILSEQ;
    end_transfer = 1;
// NOTE:  this controller uses CRCWR to report both CRC
// errors and timeouts (on writes).  MMCDRSP values are
// only weakly documented, but 0x9f was clearly a timeout
// case and the two three-bit patterns in various SD specs
// (101, 010) aren't part of it ...
//
    if (qstatus & MMCST0_CRCWR) {
    let mut temp: u32 = readb(host.base + DAVINCI_MMCDRSP);
    if (temp == 0x9f)
    data.error = -ETIMEDOUT;
    }
    dev_dbg(mmc_dev(host.mmc), "data %s %s error\n",
    (qstatus & MMCST0_CRCWR) ? "write" : "read",
    (data.error == -ETIMEDOUT) ? "timeout" : "CRC");
    davinci_abort_data(host, data);
    }
    if (qstatus & MMCST0_TOUTRS) {
// Command timeout
    if (host.cmd) {
    dev_dbg(mmc_dev(host.mmc),
    "CMD%d timeout, status %x\n",
    host.cmd.opcode, qstatus);
    host.cmd.error = -ETIMEDOUT;
    if (data) {
    end_transfer = 1;
    davinci_abort_data(host, data);
    } else
    end_command = 1;
    }
    }
    if (qstatus & MMCST0_CRCRS) {
// Command CRC error
    dev_dbg(mmc_dev(host.mmc), "Command CRC error\n");
    if (host.cmd) {
    host.cmd.error = -EILSEQ;
    end_command = 1;
    }
    }
    if (qstatus & MMCST0_RSPDNE) {
// End of command phase
    end_command = host.cmd ? 1 : 0;
    }
    if (end_command)
    mmc_davinci_cmd_done(host, host.cmd);
    if (end_transfer)
    mmc_davinci_xfer_done(host, data);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mmc_davinci_get_cd(mmc: *mut mmc_host) -> c_int {
    static int mmc_davinci_get_cd(struct mmc_host *mmc)
    {
    struct platform_device *pdev = to_platform_device(mmc.parent);
    struct davinci_mmc_config *config = pdev.dev.platform_data;
    if (config && config.get_cd)
    return config.get_cd(pdev.id);
    return mmc_gpio_get_cd(mmc);
    }
#[no_mangle]
unsafe extern "C" fn mmc_davinci_get_ro(mmc: *mut mmc_host) -> c_int {
    static int mmc_davinci_get_ro(struct mmc_host *mmc)
    {
    struct platform_device *pdev = to_platform_device(mmc.parent);
    struct davinci_mmc_config *config = pdev.dev.platform_data;
    if (config && config.get_ro)
    return config.get_ro(pdev.id);
    return mmc_gpio_get_ro(mmc);
    }
#[no_mangle]
unsafe extern "C" fn mmc_davinci_enable_sdio_irq(mmc: *mut mmc_host, enable: c_int) {
    static void mmc_davinci_enable_sdio_irq(struct mmc_host *mmc, int enable)
    {
    struct mmc_davinci_host *host = mmc_priv(mmc);
    if (enable) {
    if (!(readl(host.base + DAVINCI_SDIOST0) & SDIOST0_DAT1_HI)) {
    writel(SDIOIST_IOINT, host.base + DAVINCI_SDIOIST);
    mmc_signal_sdio_irq(host.mmc);
    } else {
    host.sdio_int = true;
    writel(readl(host.base + DAVINCI_SDIOIEN) |
    SDIOIEN_IOINTEN, host.base + DAVINCI_SDIOIEN);
    }
    } else {
    host.sdio_int = false;
    writel(readl(host.base + DAVINCI_SDIOIEN) & ~SDIOIEN_IOINTEN,
    host.base + DAVINCI_SDIOIEN);
    }
    }
    static const struct mmc_host_ops mmc_davinci_ops = {
    .request	= mmc_davinci_request,
    .set_ios	= mmc_davinci_set_ios,
    .get_cd		= mmc_davinci_get_cd,
    .get_ro		= mmc_davinci_get_ro,
    .enable_sdio_irq = mmc_davinci_enable_sdio_irq,
    };
// ----------------------------------------------------------------------

    static int mmc_davinci_cpufreq_transition(struct notifier_block *nb,
    unsigned long val, void *data)
    {
    struct mmc_davinci_host *host;
    unsigned int mmc_pclk;
    struct mmc_host *mmc;
    unsigned long flags;
    host = container_of(nb, struct mmc_davinci_host, freq_transition);
    mmc = host.mmc;
    mmc_pclk = clk_get_rate(host.clk);
    if (val == CPUFREQ_POSTCHANGE) {
    spin_lock_irqsave(&mmc.lock, flags);
    host.mmc_input_clk = mmc_pclk;
    calculate_clk_divider(mmc, &mmc.ios);
    spin_unlock_irqrestore(&mmc.lock, flags);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mmc_davinci_cpufreq_register(host: *mut mmc_davinci_host) -> c_int {
    static inline int mmc_davinci_cpufreq_register(struct mmc_davinci_host *host)
    {
    host.freq_transition.notifier_call = mmc_davinci_cpufreq_transition;
    return cpufreq_register_notifier(&host.freq_transition,
    CPUFREQ_TRANSITION_NOTIFIER);
    }
#[no_mangle]
pub unsafe extern "C" fn mmc_davinci_cpufreq_deregister(host: *mut mmc_davinci_host) {
    static inline void mmc_davinci_cpufreq_deregister(struct mmc_davinci_host *host)
    {
    cpufreq_unregister_notifier(&host.freq_transition,
    CPUFREQ_TRANSITION_NOTIFIER);
    }

#[no_mangle]
pub unsafe extern "C" fn mmc_davinci_cpufreq_register(host: *mut mmc_davinci_host) -> c_int {
    static inline int mmc_davinci_cpufreq_register(struct mmc_davinci_host *host)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mmc_davinci_cpufreq_deregister(host: *mut mmc_davinci_host) {
    static inline void mmc_davinci_cpufreq_deregister(struct mmc_davinci_host *host)
    {
    }

#[no_mangle]
unsafe extern "C" fn init_mmcsd_host(host: *mut mmc_davinci_host) {
    static void init_mmcsd_host(struct mmc_davinci_host *host)
    {
    mmc_davinci_reset_ctrl(host, 1);
    writel(0, host.base + DAVINCI_MMCCLK);
    writel(MMCCLK_CLKEN, host.base + DAVINCI_MMCCLK);
    writel(0x1FFF, host.base + DAVINCI_MMCTOR);
    writel(0xFFFF, host.base + DAVINCI_MMCTOD);
    mmc_davinci_reset_ctrl(host, 0);
    }
    static const struct platform_device_id davinci_mmc_devtype[] = {
    {
    .name	= "dm6441-mmc",
    .driver_data = MMC_CTLR_VERSION_1,
    }, {
    .name	= "da830-mmc",
    .driver_data = MMC_CTLR_VERSION_2,
    },
    {},
    };
    MODULE_DEVICE_TABLE(platform, davinci_mmc_devtype);
    static const struct of_device_id davinci_mmc_dt_ids[] = {
    {
    .compatible = "ti,dm6441-mmc",
    .data = &davinci_mmc_devtype[MMC_CTLR_VERSION_1],
    },
    {
    .compatible = "ti,da830-mmc",
    .data = &davinci_mmc_devtype[MMC_CTLR_VERSION_2],
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, davinci_mmc_dt_ids);
#[no_mangle]
unsafe extern "C" fn mmc_davinci_parse_pdata(mmc: *mut mmc_host) -> c_int {
    static int mmc_davinci_parse_pdata(struct mmc_host *mmc)
    {
    struct platform_device *pdev = to_platform_device(mmc.parent);
    struct davinci_mmc_config *pdata = pdev.dev.platform_data;
    struct mmc_davinci_host *host;
    int ret;
    if (!pdata)
    return -EINVAL;
    host = mmc_priv(mmc);
    if (!host)
    return -EINVAL;
    if (pdata && pdata.nr_sg)
    host.nr_sg = pdata.nr_sg - 1;
    if (pdata && (pdata.wires == 4 || pdata.wires == 0))
    mmc.caps |= MMC_CAP_4_BIT_DATA;
    if (pdata && (pdata.wires == 8))
    mmc.caps |= (MMC_CAP_4_BIT_DATA | MMC_CAP_8_BIT_DATA);
    mmc.f_min = 312500;
    mmc.f_max = 25000000;
    if (pdata && pdata.max_freq)
    mmc.f_max = pdata.max_freq;
    if (pdata && pdata.caps)
    mmc.caps |= pdata.caps;
// Register a cd gpio, if there is not one, enable polling
    ret = mmc_gpiod_request_cd(mmc, "cd", 0, false, 0);
    if (ret == -EPROBE_DEFER)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ret) -> else {
    else if (ret)
    mmc.caps |= MMC_CAP_NEEDS_POLL;
    ret = mmc_gpiod_request_ro(mmc, "wp", 0, 0);
    if (ret == -EPROBE_DEFER)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn davinci_mmcsd_probe(pdev: *mut platform_device) -> c_int {
    static int davinci_mmcsd_probe(struct platform_device *pdev)
    {
    struct mmc_davinci_host *host = core::ptr::null_mut();
    struct mmc_host *mmc = core::ptr::null_mut();
    struct resource *r, *mem = core::ptr::null_mut();
    int ret, irq, bus_width;
    size_t mem_size;
    const struct platform_device_id *id_entry;
    r = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!r)
    return -ENODEV;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    mem_size = resource_size(r);
    mem = devm_request_mem_region(&pdev.dev, r.start, mem_size,
    pdev.name);
    if (!mem)
    return -EBUSY;
    mmc = devm_mmc_alloc_host(&pdev.dev, sizeof(*host));
    if (!mmc)
    return -ENOMEM;
    host = mmc_priv(mmc);
    host.mmc = mmc;	/* Important */
    host.mem_res = mem;
    host.base = devm_ioremap(&pdev.dev, mem.start, mem_size);
    if (!host.base)
    return -ENOMEM;
    host.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(host.clk))
    return PTR_ERR(host.clk);
    ret = clk_prepare_enable(host.clk);
    if (ret)
    return ret;
    host.mmc_input_clk = clk_get_rate(host.clk);
    pdev.id_entry = device_get_match_data(&pdev.dev);
    if (pdev.id_entry) {
    ret = mmc_of_parse(mmc);
    if (ret) {
    dev_err_probe(&pdev.dev, ret,
    "could not parse of data\n");
    goto parse_fail;
    }
    } else {
    ret = mmc_davinci_parse_pdata(mmc);
    if (ret) {
    dev_err(&pdev.dev,
    "could not parse platform data: %d\n", ret);
    goto parse_fail;
    }	}
    if (host.nr_sg > MAX_NR_SG || !host.nr_sg)
    host.nr_sg = MAX_NR_SG;
    init_mmcsd_host(host);
    host.use_dma = use_dma;
    host.mmc_irq = irq;
    host.sdio_irq = platform_get_irq_optional(pdev, 1);
    if (host.use_dma) {
    ret = davinci_acquire_dma_channels(host);
    if (ret == -EPROBE_DEFER)
    goto dma_probe_defer;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ret) -> else {
    else if (ret)
    host.use_dma = 0;
    }
    mmc.caps |= MMC_CAP_WAIT_WHILE_BUSY;
    id_entry = platform_get_device_id(pdev);
    if (id_entry)
    host.version = id_entry.driver_data;
    mmc.ops = &mmc_davinci_ops;
    mmc.ocr_avail = MMC_VDD_32_33 | MMC_VDD_33_34;
// With no iommu coalescing pages, each phys_seg is a hw_seg.
// Each hw_seg uses one EDMA parameter RAM slot, always one
// channel and then usually some linked slots.
//
    mmc.max_segs		= MAX_NR_SG;
// EDMA limit per hw segment (one or two MBytes)
    mmc.max_seg_size	= MAX_CCNT * rw_threshold;
// MMC/SD controller limits for multiblock requests
    mmc.max_blk_size	= 4095;  /* BLEN is 12 bits */
    mmc.max_blk_count	= 65535; /* NBLK is 16 bits */
    mmc.max_req_size	= mmc.max_blk_size * mmc.max_blk_count;
    dev_dbg(mmc_dev(host.mmc), "max_segs=%d\n", mmc.max_segs);
    dev_dbg(mmc_dev(host.mmc), "max_blk_size=%d\n", mmc.max_blk_size);
    dev_dbg(mmc_dev(host.mmc), "max_req_size=%d\n", mmc.max_req_size);
    dev_dbg(mmc_dev(host.mmc), "max_seg_size=%d\n", mmc.max_seg_size);
    platform_set_drvdata(pdev, host);
    ret = mmc_davinci_cpufreq_register(host);
    if (ret) {
    dev_err(&pdev.dev, "failed to register cpufreq\n");
    goto cpu_freq_fail;
    }
    ret = devm_request_irq(&pdev.dev, irq, mmc_davinci_irq, 0,
    mmc_hostname(mmc), host);
    if (ret)
    goto mmc_add_host_fail;
    if (host.sdio_irq >= 0) {
    ret = devm_request_irq(&pdev.dev, host.sdio_irq,
    mmc_davinci_sdio_irq, 0,
    mmc_hostname(mmc), host);
    if (!ret)
    mmc.caps |= MMC_CAP_SDIO_IRQ;
    }
    ret = mmc_add_host(mmc);
    if (ret < 0)
    goto mmc_add_host_fail;
    rename_region(mem, mmc_hostname(mmc));
    if (mmc.caps & MMC_CAP_8_BIT_DATA)
    bus_width = 8;
#[no_mangle]
pub unsafe extern "C" fn if(MMC_CAP_4_BIT_DATA: mmc->caps &) -> else {
    else if (mmc.caps & MMC_CAP_4_BIT_DATA)
    bus_width = 4;
    else
    bus_width = 1;
    dev_info(mmc_dev(host.mmc), "Using %s, %d-bit mode\n",
    host.use_dma ? "DMA" : "PIO", bus_width);
    return 0;
    mmc_add_host_fail:
    mmc_davinci_cpufreq_deregister(host);
    cpu_freq_fail:
    davinci_release_dma_channels(host);
    parse_fail:
    dma_probe_defer:
    clk_disable_unprepare(host.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn davinci_mmcsd_remove(pdev: *mut platform_device) {
    static void davinci_mmcsd_remove(struct platform_device *pdev)
    {
    struct mmc_davinci_host *host = platform_get_drvdata(pdev);
    mmc_remove_host(host.mmc);
    mmc_davinci_cpufreq_deregister(host);
    davinci_release_dma_channels(host);
    clk_disable_unprepare(host.clk);
    }
#[no_mangle]
unsafe extern "C" fn davinci_mmcsd_suspend(dev: *mut device) -> c_int {
    static int davinci_mmcsd_suspend(struct device *dev)
    {
    struct mmc_davinci_host *host = dev_get_drvdata(dev);
    writel(0, host.base + DAVINCI_MMCIM);
    mmc_davinci_reset_ctrl(host, 1);
    clk_disable(host.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn davinci_mmcsd_resume(dev: *mut device) -> c_int {
    static int davinci_mmcsd_resume(struct device *dev)
    {
    struct mmc_davinci_host *host = dev_get_drvdata(dev);
    int ret;
    ret = clk_enable(host.clk);
    if (ret)
    return ret;
    mmc_davinci_reset_ctrl(host, 0);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(davinci_mmcsd_pm_ops,
    davinci_mmcsd_suspend, davinci_mmcsd_resume);
    static struct platform_driver davinci_mmcsd_driver = {
    .driver		= {
    .name	= "davinci_mmc",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .pm	= pm_sleep_ptr(&davinci_mmcsd_pm_ops),
    .of_match_table = davinci_mmc_dt_ids,
    },
    .probe		= davinci_mmcsd_probe,
    .remove		= davinci_mmcsd_remove,
    .id_table	= davinci_mmc_devtype,
    };
    module_platform_driver(davinci_mmcsd_driver);
    MODULE_AUTHOR("Texas Instruments India");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("MMC/SD driver for Davinci MMC controller");
    MODULE_ALIAS("platform:davinci_mmc");
