//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/sh_mmcif.c
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
//
// MMCIF eMMC driver.
//
// Copyright (C) 2010 Renesas Solutions Corp.
// Yusuke Goda <yusuke.goda.sx@renesas.com>
//
// The MMCIF driver is now processing MMC requests asynchronously, according
// to the Linux MMC API requirement.
//
// The MMCIF driver processes MMC requests in up to 3 stages: command, optional
// data, and optional stop. To achieve asynchronous processing each of these
// stages is split into two halves: a top and a bottom half. The top half
// initialises the hardware, installs a timeout handler to handle completion
// timeouts, and returns. In case of the command stage this immediately returns
// control to the caller, leaving all further processing to run asynchronously.
// All further request processing is performed by the bottom halves.
//
// The bottom half further consists of a "hard" IRQ handler, an IRQ handler
// thread, a DMA completion callback, if DMA is used, a timeout work, and
// request- and stage-specific handler methods.
//
// Each bottom half run begins with either a hardware interrupt, a DMA callback
// invocation, or a timeout work run. In case of an error or a successful
// processing completion, the MMC core is informed and the request processing is
// finished. In case processing has to continue, i.e., if data has to be read
// from or written to the card, or if a stop command has to be sent, the next
// top half is called, which performs the necessary hardware handling and
// reschedules the timeout work. This returns the driver state machine into the
// bottom half waiting state.
//

// CE_CMD_SET
pub const CMD_MASK: c_uint = 0x3f000000;

// CE_CMD_CTRL

// CE_BLOCK_SET
pub const BLOCK_SIZE_MASK: c_uint = 0x0000ffff;
// CE_INT

    INT_RDATERR | INT_RIDXERR | INT_RSPERR | \
    INT_CCSTO | INT_CRCSTO | INT_WDATTO |	  \
    INT_RDATTO | INT_RBSYTO | INT_RSPTO)

    INT_BUFWEN | INT_CMD12DRE | INT_BUFRE | \
    INT_DTRANE | INT_CMD12RBE | INT_CMD12CRE)

// CE_INT_MASK
pub const MASK_ALL: c_uint = 0x00000000;

    MASK_MRDATERR | MASK_MRIDXERR | MASK_MRSPERR | \
    MASK_MCRCSTO | MASK_MWDATTO | \
    MASK_MRDATTO | MASK_MRBSYTO | MASK_MRSPTO)

    MASK_MBUFREN | MASK_MBUFWEN |			\
    MASK_MCMD12DRE | MASK_MBUFRE | MASK_MDTRANE |	\
    MASK_MCMD12RBE | MASK_MCMD12CRE)
// CE_HOST_STS1

// CE_HOST_STS2

    STS2_AC12CRCE | STS2_RSPCRC7E | STS2_CRCSTEBE)

    STS2_DATBSYTO | STS2_CRCSTTO |		\
    STS2_AC12BSYTO | STS2_RSPBSYTO |	\
    STS2_AC12RSPTO | STS2_RSPTO)

    enum sh_mmcif_state {
    STATE_IDLE,
    STATE_REQUEST,
    STATE_IOS,
    STATE_TIMEOUT,
    };
    enum sh_mmcif_wait_for {
    MMCIF_WAIT_FOR_REQUEST,
    MMCIF_WAIT_FOR_CMD,
    MMCIF_WAIT_FOR_MREAD,
    MMCIF_WAIT_FOR_MWRITE,
    MMCIF_WAIT_FOR_READ,
    MMCIF_WAIT_FOR_WRITE,
    MMCIF_WAIT_FOR_READ_END,
    MMCIF_WAIT_FOR_WRITE_END,
    MMCIF_WAIT_FOR_STOP,
    };
//
// difference for each SoC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_mmcif_host {
    pub mmc: *mut mmc_host,
    pub mrq: *mut mmc_request,
    pub pd: *mut platform_device,
    pub clk: *mut clk,
    pub bus_width: c_int,
    pub timing: c_uchar,
    pub sd_error: bool,
    pub dying: bool,
    pub timeout: c_long,
    pub addr: *mut void __iomem,
    pub /: *mut *mut spinlock_t lock; / protect sh_mmcif_host::state,
    pub state: enum sh_mmcif_state,
    pub wait_for: enum sh_mmcif_wait_for,
    pub timeout_work: delayed_work,
    pub blocksize: usize,
    pub sg_miter: sg_mapping_iter,
    pub power: bool,
    pub /: *mut *mut bool ccs_enable; / Command Completion Signal support,
    pub clk_ctrl2_enable: bool,
    pub thread_lock: mutex,
    pub /: *mut *mut u32 clkdiv_map; / see CE_CLK_CTRL::CLKDIV,
// DMA support
    pub chan_rx: *mut dma_chan,
    pub chan_tx: *mut dma_chan,
    pub dma_complete: completion,
    pub dma_active: bool,
}

    static const struct of_device_id sh_mmcif_of_match[] = {
    { .compatible = "renesas,sh-mmcif" },
    { }
    };
    MODULE_DEVICE_TABLE(of, sh_mmcif_of_match);

    static inline void sh_mmcif_bitset(struct sh_mmcif_host *host,
    unsigned int reg, u32 val)
    {
    writel(val | readl(host.addr + reg), host.addr + reg);
    }
    static inline void sh_mmcif_bitclr(struct sh_mmcif_host *host,
    unsigned int reg, u32 val)
    {
    writel(~val & readl(host.addr + reg), host.addr + reg);
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_dma_complete(arg: *mut c_void) {
    static void sh_mmcif_dma_complete(void *arg)
    {
    struct sh_mmcif_host *host = arg;
    struct mmc_request *mrq = host.mrq;
    struct device *dev = sh_mmcif_host_to_dev(host);
    dev_dbg(dev, "Command completed\n");
    if (WARN(!mrq || !mrq.data, "%s: core::ptr::null_mut() data in DMA completion!\n",
    dev_name(dev)))
    return;
    complete(&host.dma_complete);
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_start_dma_rx(host: *mut sh_mmcif_host) {
    static void sh_mmcif_start_dma_rx(struct sh_mmcif_host *host)
    {
    struct mmc_data *data = host.mrq.data;
    struct scatterlist *sg = data.sg;
    struct dma_async_tx_descriptor *desc = core::ptr::null_mut();
    struct dma_chan *chan = host.chan_rx;
    struct device *dev = sh_mmcif_host_to_dev(host);
    let mut cookie: dma_cookie_t = -EINVAL;
    int ret;
    ret = dma_map_sg(chan.device.dev, sg, data.sg_len,
    DMA_FROM_DEVICE);
    if (ret > 0) {
    host.dma_active = true;
    desc = dmaengine_prep_slave_sg(chan, sg, ret,
    DMA_DEV_TO_MEM, DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    }
    if (desc) {
    desc.callback = sh_mmcif_dma_complete;
    desc.callback_param = host;
    cookie = dmaengine_submit(desc);
    sh_mmcif_bitset(host, MMCIF_CE_BUF_ACC, BUF_ACC_DMAREN);
    dma_async_issue_pending(chan);
    }
    dev_dbg(dev, "%s(): mapped %d . %d, cookie %d\n",
    __func__, data.sg_len, ret, cookie);
    if (!desc) {
// DMA failed, fall back to PIO
    if (ret >= 0)
    ret = -EIO;
    host.chan_rx = core::ptr::null_mut();
    host.dma_active = false;
    dma_release_channel(chan);
// Free the Tx channel too
    chan = host.chan_tx;
    if (chan) {
    host.chan_tx = core::ptr::null_mut();
    dma_release_channel(chan);
    }
    dev_warn(dev,
    "DMA failed: %d, falling back to PIO\n", ret);
    sh_mmcif_bitclr(host, MMCIF_CE_BUF_ACC, BUF_ACC_DMAREN | BUF_ACC_DMAWEN);
    }
    dev_dbg(dev, "%s(): desc %p, cookie %d, sg[%d]\n", __func__,
    desc, cookie, data.sg_len);
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_start_dma_tx(host: *mut sh_mmcif_host) {
    static void sh_mmcif_start_dma_tx(struct sh_mmcif_host *host)
    {
    struct mmc_data *data = host.mrq.data;
    struct scatterlist *sg = data.sg;
    struct dma_async_tx_descriptor *desc = core::ptr::null_mut();
    struct dma_chan *chan = host.chan_tx;
    struct device *dev = sh_mmcif_host_to_dev(host);
    let mut cookie: dma_cookie_t = -EINVAL;
    int ret;
    ret = dma_map_sg(chan.device.dev, sg, data.sg_len,
    DMA_TO_DEVICE);
    if (ret > 0) {
    host.dma_active = true;
    desc = dmaengine_prep_slave_sg(chan, sg, ret,
    DMA_MEM_TO_DEV, DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    }
    if (desc) {
    desc.callback = sh_mmcif_dma_complete;
    desc.callback_param = host;
    cookie = dmaengine_submit(desc);
    sh_mmcif_bitset(host, MMCIF_CE_BUF_ACC, BUF_ACC_DMAWEN);
    dma_async_issue_pending(chan);
    }
    dev_dbg(dev, "%s(): mapped %d . %d, cookie %d\n",
    __func__, data.sg_len, ret, cookie);
    if (!desc) {
// DMA failed, fall back to PIO
    if (ret >= 0)
    ret = -EIO;
    host.chan_tx = core::ptr::null_mut();
    host.dma_active = false;
    dma_release_channel(chan);
// Free the Rx channel too
    chan = host.chan_rx;
    if (chan) {
    host.chan_rx = core::ptr::null_mut();
    dma_release_channel(chan);
    }
    dev_warn(dev,
    "DMA failed: %d, falling back to PIO\n", ret);
    sh_mmcif_bitclr(host, MMCIF_CE_BUF_ACC, BUF_ACC_DMAREN | BUF_ACC_DMAWEN);
    }
    dev_dbg(dev, "%s(): desc %p, cookie %d\n", __func__,
    desc, cookie);
    }
    static struct dma_chan *
    sh_mmcif_request_dma_pdata(struct sh_mmcif_host *host, uintptr_t slave_id)
    {
    dma_cap_mask_t mask;
    dma_cap_zero(mask);
    dma_cap_set(DMA_SLAVE, mask);
    if (slave_id <= 0)
    return core::ptr::null_mut();
    return dma_request_channel(mask, shdma_chan_filter, (void *)slave_id);
    }
    static int sh_mmcif_dma_slave_config(struct sh_mmcif_host *host,
    struct dma_chan *chan,
    enum dma_transfer_direction direction)
    {
    struct resource *res;
    let mut cfg: dma_slave_config = { 0, };
    res = platform_get_resource(host.pd, IORESOURCE_MEM, 0);
    if (!res)
    return -EINVAL;
    cfg.direction = direction;
    if (direction == DMA_DEV_TO_MEM) {
    cfg.src_addr = res.start + MMCIF_CE_DATA;
    cfg.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    } else {
    cfg.dst_addr = res.start + MMCIF_CE_DATA;
    cfg.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    }
    return dmaengine_slave_config(chan, &cfg);
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_request_dma(host: *mut sh_mmcif_host) {
    static void sh_mmcif_request_dma(struct sh_mmcif_host *host)
    {
    struct device *dev = sh_mmcif_host_to_dev(host);
    host.dma_active = false;
// We can only either use DMA for both Tx and Rx or not use it at all
    if (IS_ENABLED(CONFIG_SUPERH) && dev.platform_data) {
    struct sh_mmcif_plat_data *pdata = dev.platform_data;
    host.chan_tx = sh_mmcif_request_dma_pdata(host,
    pdata.slave_id_tx);
    host.chan_rx = sh_mmcif_request_dma_pdata(host,
    pdata.slave_id_rx);
    } else {
    host.chan_tx = dma_request_chan(dev, "tx");
    if (IS_ERR(host.chan_tx))
    host.chan_tx = core::ptr::null_mut();
    host.chan_rx = dma_request_chan(dev, "rx");
    if (IS_ERR(host.chan_rx))
    host.chan_rx = core::ptr::null_mut();
    }
    if (!host.chan_tx || !host.chan_rx ||
    sh_mmcif_dma_slave_config(host, host.chan_tx, DMA_MEM_TO_DEV) ||
    sh_mmcif_dma_slave_config(host, host.chan_rx, DMA_DEV_TO_MEM))
    goto error;
    dev_dbg(dev, "%s: got channel TX %p RX %p\n", __func__, host.chan_tx,
    host.chan_rx);
    return;
    error:
    if (host.chan_tx)
    dma_release_channel(host.chan_tx);
    if (host.chan_rx)
    dma_release_channel(host.chan_rx);
    host.chan_tx = host.chan_rx = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_release_dma(host: *mut sh_mmcif_host) {
    static void sh_mmcif_release_dma(struct sh_mmcif_host *host)
    {
    sh_mmcif_bitclr(host, MMCIF_CE_BUF_ACC, BUF_ACC_DMAREN | BUF_ACC_DMAWEN);
// Descriptors are freed automatically
    if (host.chan_tx) {
    struct dma_chan *chan = host.chan_tx;
    host.chan_tx = core::ptr::null_mut();
    dma_release_channel(chan);
    }
    if (host.chan_rx) {
    struct dma_chan *chan = host.chan_rx;
    host.chan_rx = core::ptr::null_mut();
    dma_release_channel(chan);
    }
    host.dma_active = false;
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_clock_control(host: *mut sh_mmcif_host, clk: c_uint) {
    static void sh_mmcif_clock_control(struct sh_mmcif_host *host, unsigned int clk)
    {
    struct device *dev = sh_mmcif_host_to_dev(host);
    struct sh_mmcif_plat_data *p = dev.platform_data;
    let mut sup_pclk: bool = p ? p.sup_pclk : false;
    let mut current_clk: c_uint = clk_get_rate(host.clk);
    unsigned int clkdiv;
    sh_mmcif_bitclr(host, MMCIF_CE_CLK_CTRL, CLK_ENABLE);
    sh_mmcif_bitclr(host, MMCIF_CE_CLK_CTRL, CLK_CLEAR);
    if (!clk)
    return;
    if (host.clkdiv_map) {
    unsigned int freq, best_freq, myclk, div, diff_min, diff;
    int i;
    clkdiv = 0;
    diff_min = ~0;
    best_freq = 0;
    for (i = 31; i >= 0; i--) {
    if (!((1 << i) & host.clkdiv_map))
    continue;
//
// clk = parent_freq / div
// -> parent_freq = clk x div
//
    div = 1 << (i + 1);
    freq = clk_round_rate(host.clk, clk * div);
    myclk = freq / div;
    diff = (myclk > clk) ? myclk - clk : clk - myclk;
    if (diff <= diff_min) {
    best_freq = freq;
    clkdiv = i;
    diff_min = diff;
    }
    }
    dev_dbg(dev, "clk %u/%u (%u, 0x%x)\n",
    (best_freq >> (clkdiv + 1)), clk, best_freq, clkdiv);
    clk_set_rate(host.clk, best_freq);
    clkdiv = clkdiv << 16;
    } else if (sup_pclk && clk == current_clk) {
    clkdiv = CLK_SUP_PCLK;
    } else {
    clkdiv = (fls(DIV_ROUND_UP(current_clk, clk) - 1) - 1) << 16;
    }
    sh_mmcif_bitset(host, MMCIF_CE_CLK_CTRL, CLK_CLEAR & clkdiv);
    sh_mmcif_bitset(host, MMCIF_CE_CLK_CTRL, CLK_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_sync_reset(host: *mut sh_mmcif_host) {
    static void sh_mmcif_sync_reset(struct sh_mmcif_host *host)
    {
    u32 tmp;
    tmp = 0x010f0000 & sh_mmcif_readl(host.addr, MMCIF_CE_CLK_CTRL);
    sh_mmcif_writel(host.addr, MMCIF_CE_VERSION, SOFT_RST_ON);
    sh_mmcif_writel(host.addr, MMCIF_CE_VERSION, SOFT_RST_OFF);
    if (host.ccs_enable)
    tmp |= SCCSTO_29;
    if (host.clk_ctrl2_enable)
    sh_mmcif_writel(host.addr, MMCIF_CE_CLK_CTRL2, 0x0F0F0000);
    sh_mmcif_bitset(host, MMCIF_CE_CLK_CTRL, tmp |
    SRSPTO_256 | SRBSYTO_29 | SRWDTO_29);
// byte swap on
    sh_mmcif_bitset(host, MMCIF_CE_BUF_ACC, BUF_ACC_ATYP);
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_error_manage(host: *mut sh_mmcif_host) -> c_int {
    static int sh_mmcif_error_manage(struct sh_mmcif_host *host)
    {
    struct device *dev = sh_mmcif_host_to_dev(host);
    u32 state1, state2;
    int ret, timeout;
    host.sd_error = false;
    state1 = sh_mmcif_readl(host.addr, MMCIF_CE_HOST_STS1);
    state2 = sh_mmcif_readl(host.addr, MMCIF_CE_HOST_STS2);
    dev_dbg(dev, "ERR HOST_STS1 = %08x\n", state1);
    dev_dbg(dev, "ERR HOST_STS2 = %08x\n", state2);
    if (state1 & STS1_CMDSEQ) {
    sh_mmcif_bitset(host, MMCIF_CE_CMD_CTRL, CMD_CTRL_BREAK);
    sh_mmcif_bitset(host, MMCIF_CE_CMD_CTRL, ~CMD_CTRL_BREAK);
    for (timeout = 10000; timeout; timeout--) {
    if (!(sh_mmcif_readl(host.addr, MMCIF_CE_HOST_STS1)
    & STS1_CMDSEQ))
    break;
    mdelay(1);
    }
    if (!timeout) {
    dev_err(dev,
    "Forced end of command sequence timeout err\n");
    return -EIO;
    }
    sh_mmcif_sync_reset(host);
    dev_dbg(dev, "Forced end of command sequence\n");
    return -EIO;
    }
    if (state2 & STS2_CRC_ERR) {
    dev_err(dev, " CRC error: state %u, wait %u\n",
    host.state, host.wait_for);
    ret = -EIO;
    } else if (state2 & STS2_TIMEOUT_ERR) {
    dev_err(dev, " Timeout: state %u, wait %u\n",
    host.state, host.wait_for);
    ret = -ETIMEDOUT;
    } else {
    dev_dbg(dev, " End/Index error: state %u, wait %u\n",
    host.state, host.wait_for);
    ret = -EIO;
    }
    return ret;
    }
    static void sh_mmcif_single_read(struct sh_mmcif_host *host,
    struct mmc_request *mrq)
    {
    struct mmc_data *data = mrq.data;
    host.blocksize = (sh_mmcif_readl(host.addr, MMCIF_CE_BLOCK_SET) &
    BLOCK_SIZE_MASK) + 3;
    sg_miter_start(&host.sg_miter, data.sg, data.sg_len,
    SG_MITER_TO_SG);
    host.wait_for = MMCIF_WAIT_FOR_READ;
// buf read enable
    sh_mmcif_bitset(host, MMCIF_CE_INT_MASK, MASK_MBUFREN);
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_read_block(host: *mut sh_mmcif_host) -> bool {
    static bool sh_mmcif_read_block(struct sh_mmcif_host *host)
    {
    struct sg_mapping_iter *sgm = &host.sg_miter;
    struct device *dev = sh_mmcif_host_to_dev(host);
    struct mmc_data *data = host.mrq.data;
    u32 *p;
    int i;
    if (host.sd_error) {
    sg_miter_stop(sgm);
    data.error = sh_mmcif_error_manage(host);
    dev_dbg(dev, "%s(): %d\n", __func__, data.error);
    return false;
    }
    if (!sg_miter_next(sgm)) {
// This should not happen on single blocks
    sg_miter_stop(sgm);
    return false;
    }
    p = sgm.addr;
    for (i = 0; i < host.blocksize / 4; i++)
// p++ = sh_mmcif_readl(host->addr, MMCIF_CE_DATA);
    sg_miter_stop(&host.sg_miter);
// buffer read end
    sh_mmcif_bitset(host, MMCIF_CE_INT_MASK, MASK_MBUFRE);
    host.wait_for = MMCIF_WAIT_FOR_READ_END;
    return true;
    }
    static void sh_mmcif_multi_read(struct sh_mmcif_host *host,
    struct mmc_request *mrq)
    {
    struct sg_mapping_iter *sgm = &host.sg_miter;
    struct mmc_data *data = mrq.data;
    if (!data.sg_len || !data.sg.length)
    return;
    host.blocksize = sh_mmcif_readl(host.addr, MMCIF_CE_BLOCK_SET) &
    BLOCK_SIZE_MASK;
    sg_miter_start(sgm, data.sg, data.sg_len,
    SG_MITER_TO_SG);
// Advance to the first sglist entry
    if (!sg_miter_next(sgm)) {
    sg_miter_stop(sgm);
    return;
    }
    host.wait_for = MMCIF_WAIT_FOR_MREAD;
    sh_mmcif_bitset(host, MMCIF_CE_INT_MASK, MASK_MBUFREN);
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_mread_block(host: *mut sh_mmcif_host) -> bool {
    static bool sh_mmcif_mread_block(struct sh_mmcif_host *host)
    {
    struct sg_mapping_iter *sgm = &host.sg_miter;
    struct device *dev = sh_mmcif_host_to_dev(host);
    struct mmc_data *data = host.mrq.data;
    u32 *p;
    int i;
    if (host.sd_error) {
    sg_miter_stop(sgm);
    data.error = sh_mmcif_error_manage(host);
    dev_dbg(dev, "%s(): %d\n", __func__, data.error);
    return false;
    }
    p = sgm.addr;
    for (i = 0; i < host.blocksize / 4; i++)
// p++ = sh_mmcif_readl(host->addr, MMCIF_CE_DATA);
    sgm.consumed = host.blocksize;
    sh_mmcif_bitset(host, MMCIF_CE_INT_MASK, MASK_MBUFREN);
    if (!sg_miter_next(sgm)) {
    sg_miter_stop(sgm);
    return false;
    }
    return true;
    }
    static void sh_mmcif_single_write(struct sh_mmcif_host *host,
    struct mmc_request *mrq)
    {
    struct mmc_data *data = mrq.data;
    host.blocksize = (sh_mmcif_readl(host.addr, MMCIF_CE_BLOCK_SET) &
    BLOCK_SIZE_MASK) + 3;
    sg_miter_start(&host.sg_miter, data.sg, data.sg_len,
    SG_MITER_FROM_SG);
    host.wait_for = MMCIF_WAIT_FOR_WRITE;
// buf write enable
    sh_mmcif_bitset(host, MMCIF_CE_INT_MASK, MASK_MBUFWEN);
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_write_block(host: *mut sh_mmcif_host) -> bool {
    static bool sh_mmcif_write_block(struct sh_mmcif_host *host)
    {
    struct sg_mapping_iter *sgm = &host.sg_miter;
    struct device *dev = sh_mmcif_host_to_dev(host);
    struct mmc_data *data = host.mrq.data;
    u32 *p;
    int i;
    if (host.sd_error) {
    sg_miter_stop(sgm);
    data.error = sh_mmcif_error_manage(host);
    dev_dbg(dev, "%s(): %d\n", __func__, data.error);
    return false;
    }
    if (!sg_miter_next(sgm)) {
// This should not happen on single blocks
    sg_miter_stop(sgm);
    return false;
    }
    p = sgm.addr;
    for (i = 0; i < host.blocksize / 4; i++)
    sh_mmcif_writel(host.addr, MMCIF_CE_DATA, *p++);
    sg_miter_stop(&host.sg_miter);
// buffer write end
    sh_mmcif_bitset(host, MMCIF_CE_INT_MASK, MASK_MDTRANE);
    host.wait_for = MMCIF_WAIT_FOR_WRITE_END;
    return true;
    }
    static void sh_mmcif_multi_write(struct sh_mmcif_host *host,
    struct mmc_request *mrq)
    {
    struct sg_mapping_iter *sgm = &host.sg_miter;
    struct mmc_data *data = mrq.data;
    if (!data.sg_len || !data.sg.length)
    return;
    host.blocksize = sh_mmcif_readl(host.addr, MMCIF_CE_BLOCK_SET) &
    BLOCK_SIZE_MASK;
    sg_miter_start(sgm, data.sg, data.sg_len,
    SG_MITER_FROM_SG);
// Advance to the first sglist entry
    if (!sg_miter_next(sgm)) {
    sg_miter_stop(sgm);
    return;
    }
    host.wait_for = MMCIF_WAIT_FOR_MWRITE;
    sh_mmcif_bitset(host, MMCIF_CE_INT_MASK, MASK_MBUFWEN);
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_mwrite_block(host: *mut sh_mmcif_host) -> bool {
    static bool sh_mmcif_mwrite_block(struct sh_mmcif_host *host)
    {
    struct sg_mapping_iter *sgm = &host.sg_miter;
    struct device *dev = sh_mmcif_host_to_dev(host);
    struct mmc_data *data = host.mrq.data;
    u32 *p;
    int i;
    if (host.sd_error) {
    sg_miter_stop(sgm);
    data.error = sh_mmcif_error_manage(host);
    dev_dbg(dev, "%s(): %d\n", __func__, data.error);
    return false;
    }
    p = sgm.addr;
    for (i = 0; i < host.blocksize / 4; i++)
    sh_mmcif_writel(host.addr, MMCIF_CE_DATA, *p++);
    sgm.consumed = host.blocksize;
    if (!sg_miter_next(sgm)) {
    sg_miter_stop(sgm);
    return false;
    }
    sh_mmcif_bitset(host, MMCIF_CE_INT_MASK, MASK_MBUFWEN);
    return true;
    }
    static void sh_mmcif_get_response(struct sh_mmcif_host *host,
    struct mmc_command *cmd)
    {
    if (cmd.flags & MMC_RSP_136) {
    cmd.resp[0] = sh_mmcif_readl(host.addr, MMCIF_CE_RESP3);
    cmd.resp[1] = sh_mmcif_readl(host.addr, MMCIF_CE_RESP2);
    cmd.resp[2] = sh_mmcif_readl(host.addr, MMCIF_CE_RESP1);
    cmd.resp[3] = sh_mmcif_readl(host.addr, MMCIF_CE_RESP0);
    } else
    cmd.resp[0] = sh_mmcif_readl(host.addr, MMCIF_CE_RESP0);
    }
    static void sh_mmcif_get_cmd12response(struct sh_mmcif_host *host,
    struct mmc_command *cmd)
    {
    cmd.resp[0] = sh_mmcif_readl(host.addr, MMCIF_CE_RESP_CMD12);
    }
    static u32 sh_mmcif_set_cmd(struct sh_mmcif_host *host,
    struct mmc_request *mrq)
    {
    struct device *dev = sh_mmcif_host_to_dev(host);
    struct mmc_data *data = mrq.data;
    struct mmc_command *cmd = mrq.cmd;
    let mut opc: u32 = cmd.opcode;
    let mut tmp: u32 = 0;
// Response Type check
    switch (mmc_resp_type(cmd)) {
    case MMC_RSP_NONE:
    tmp |= CMD_SET_RTYP_NO;
    break;
    case MMC_RSP_R1:
    case MMC_RSP_R3:
    tmp |= CMD_SET_RTYP_6B;
    break;
    case MMC_RSP_R1B:
    tmp |= CMD_SET_RBSY | CMD_SET_RTYP_6B;
    break;
    case MMC_RSP_R2:
    tmp |= CMD_SET_RTYP_17B;
    break;
    default:
    dev_err(dev, "Unsupported response type.\n");
    break;
    }
// WDAT / DATW
    if (data) {
    tmp |= CMD_SET_WDAT;
    switch (host.bus_width) {
    case MMC_BUS_WIDTH_1:
    tmp |= CMD_SET_DATW_1;
    break;
    case MMC_BUS_WIDTH_4:
    tmp |= CMD_SET_DATW_4;
    break;
    case MMC_BUS_WIDTH_8:
    tmp |= CMD_SET_DATW_8;
    break;
    default:
    dev_err(dev, "Unsupported bus width.\n");
    break;
    }
    switch (host.timing) {
    case MMC_TIMING_MMC_DDR52:
//
// MMC core will only set this timing, if the host
// advertises the MMC_CAP_1_8V_DDR/MMC_CAP_1_2V_DDR
// capability. MMCIF implementations with this
// capability, e.g. sh73a0, will have to set it
// in their platform data.
//
    tmp |= CMD_SET_DARS;
    break;
    }
    }
// DWEN
    if (opc == MMC_WRITE_BLOCK || opc == MMC_WRITE_MULTIPLE_BLOCK)
    tmp |= CMD_SET_DWEN;
// CMLTE/CMD12EN
    if (opc == MMC_READ_MULTIPLE_BLOCK || opc == MMC_WRITE_MULTIPLE_BLOCK) {
    tmp |= CMD_SET_CMLTE | CMD_SET_CMD12EN;
    sh_mmcif_bitset(host, MMCIF_CE_BLOCK_SET,
    data.blocks << 16);
    }
// RIDXC[1:0] check bits
    if (opc == MMC_SEND_OP_COND || opc == MMC_ALL_SEND_CID ||
    opc == MMC_SEND_CSD || opc == MMC_SEND_CID)
    tmp |= CMD_SET_RIDXC_BITS;
// RCRC7C[1:0] check bits
    if (opc == MMC_SEND_OP_COND)
    tmp |= CMD_SET_CRC7C_BITS;
// RCRC7C[1:0] internal CRC7
    if (opc == MMC_ALL_SEND_CID ||
    opc == MMC_SEND_CSD || opc == MMC_SEND_CID)
    tmp |= CMD_SET_CRC7C_INTERNAL;
    return (opc << 24) | tmp;
    }
    static int sh_mmcif_data_trans(struct sh_mmcif_host *host,
    struct mmc_request *mrq, u32 opc)
    {
    struct device *dev = sh_mmcif_host_to_dev(host);
    switch (opc) {
    case MMC_READ_MULTIPLE_BLOCK:
    sh_mmcif_multi_read(host, mrq);
    return 0;
    case MMC_WRITE_MULTIPLE_BLOCK:
    sh_mmcif_multi_write(host, mrq);
    return 0;
    case MMC_WRITE_BLOCK:
    sh_mmcif_single_write(host, mrq);
    return 0;
    case MMC_READ_SINGLE_BLOCK:
    case MMC_SEND_EXT_CSD:
    sh_mmcif_single_read(host, mrq);
    return 0;
    default:
    dev_err(dev, "Unsupported CMD%d\n", opc);
    return -EINVAL;
    }
    }
    static void sh_mmcif_start_cmd(struct sh_mmcif_host *host,
    struct mmc_request *mrq)
    {
    struct mmc_command *cmd = mrq.cmd;
    u32 opc;
    let mut mask: u32 = 0;
    unsigned long flags;
    if (cmd.flags & MMC_RSP_BUSY)
    mask = MASK_START_CMD | MASK_MRBSYE;
    else
    mask = MASK_START_CMD | MASK_MCRSPE;
    if (host.ccs_enable)
    mask |= MASK_MCCSTO;
    if (mrq.data) {
    sh_mmcif_writel(host.addr, MMCIF_CE_BLOCK_SET, 0);
    sh_mmcif_writel(host.addr, MMCIF_CE_BLOCK_SET,
    mrq.data.blksz);
    }
    opc = sh_mmcif_set_cmd(host, mrq);
    if (host.ccs_enable)
    sh_mmcif_writel(host.addr, MMCIF_CE_INT, 0xD80430C0);
    else
    sh_mmcif_writel(host.addr, MMCIF_CE_INT, 0xD80430C0 | INT_CCS);
    sh_mmcif_writel(host.addr, MMCIF_CE_INT_MASK, mask);
// set arg
    sh_mmcif_writel(host.addr, MMCIF_CE_ARG, cmd.arg);
// set cmd
    spin_lock_irqsave(&host.lock, flags);
    sh_mmcif_writel(host.addr, MMCIF_CE_CMD_SET, opc);
    host.wait_for = MMCIF_WAIT_FOR_CMD;
    schedule_delayed_work(&host.timeout_work, host.timeout);
    spin_unlock_irqrestore(&host.lock, flags);
    }
    static void sh_mmcif_stop_cmd(struct sh_mmcif_host *host,
    struct mmc_request *mrq)
    {
    struct device *dev = sh_mmcif_host_to_dev(host);
    switch (mrq.cmd.opcode) {
    case MMC_READ_MULTIPLE_BLOCK:
    sh_mmcif_bitset(host, MMCIF_CE_INT_MASK, MASK_MCMD12DRE);
    break;
    case MMC_WRITE_MULTIPLE_BLOCK:
    sh_mmcif_bitset(host, MMCIF_CE_INT_MASK, MASK_MCMD12RBE);
    break;
    default:
    dev_err(dev, "unsupported stop cmd\n");
    mrq.stop.error = sh_mmcif_error_manage(host);
    return;
    }
    host.wait_for = MMCIF_WAIT_FOR_STOP;
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_request(mmc: *mut mmc_host, mrq: *mut mmc_request) {
    static void sh_mmcif_request(struct mmc_host *mmc, struct mmc_request *mrq)
    {
    struct sh_mmcif_host *host = mmc_priv(mmc);
    struct device *dev = sh_mmcif_host_to_dev(host);
    unsigned long flags;
    spin_lock_irqsave(&host.lock, flags);
    if (host.state != STATE_IDLE) {
    dev_dbg(dev, "%s() rejected, state %u\n",
    __func__, host.state);
    spin_unlock_irqrestore(&host.lock, flags);
    mrq.cmd.error = -EAGAIN;
    mmc_request_done(mmc, mrq);
    return;
    }
    host.state = STATE_REQUEST;
    spin_unlock_irqrestore(&host.lock, flags);
    host.mrq = mrq;
    sh_mmcif_start_cmd(host, mrq);
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_clk_setup(host: *mut sh_mmcif_host) {
    static void sh_mmcif_clk_setup(struct sh_mmcif_host *host)
    {
    struct device *dev = sh_mmcif_host_to_dev(host);
    if (host.mmc.f_max) {
    unsigned int f_max, f_min = 0, f_min_old;
    f_max = host.mmc.f_max;
    for (f_min_old = f_max; f_min_old > 2;) {
    f_min = clk_round_rate(host.clk, f_min_old / 2);
    if (f_min == f_min_old)
    break;
    f_min_old = f_min;
    }
//
// This driver assumes this SoC is R-Car Gen2 or later
//
    host.clkdiv_map = 0x3ff;
    host.mmc.f_max = f_max >> ffs(host.clkdiv_map);
    host.mmc.f_min = f_min >> fls(host.clkdiv_map);
    } else {
    let mut clk: c_uint = clk_get_rate(host.clk);
    host.mmc.f_max = clk / 2;
    host.mmc.f_min = clk / 512;
    }
    dev_dbg(dev, "clk max/min = %d/%d\n",
    host.mmc.f_max, host.mmc.f_min);
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_set_ios(mmc: *mut mmc_host, ios: *mut mmc_ios) {
    static void sh_mmcif_set_ios(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    struct sh_mmcif_host *host = mmc_priv(mmc);
    struct device *dev = sh_mmcif_host_to_dev(host);
    unsigned long flags;
    spin_lock_irqsave(&host.lock, flags);
    if (host.state != STATE_IDLE) {
    dev_dbg(dev, "%s() rejected, state %u\n",
    __func__, host.state);
    spin_unlock_irqrestore(&host.lock, flags);
    return;
    }
    host.state = STATE_IOS;
    spin_unlock_irqrestore(&host.lock, flags);
    switch (ios.power_mode) {
    case MMC_POWER_UP:
    if (!IS_ERR(mmc.supply.vmmc))
    mmc_regulator_set_ocr(mmc, mmc.supply.vmmc, ios.vdd);
    if (!host.power) {
    clk_prepare_enable(host.clk);
    pm_runtime_get_sync(dev);
    sh_mmcif_sync_reset(host);
    sh_mmcif_request_dma(host);
    host.power = true;
    }
    break;
    case MMC_POWER_OFF:
    if (!IS_ERR(mmc.supply.vmmc))
    mmc_regulator_set_ocr(mmc, mmc.supply.vmmc, 0);
    if (host.power) {
    sh_mmcif_clock_control(host, 0);
    sh_mmcif_release_dma(host);
    pm_runtime_put(dev);
    clk_disable_unprepare(host.clk);
    host.power = false;
    }
    break;
    case MMC_POWER_ON:
    sh_mmcif_clock_control(host, ios.clock);
    break;
    }
    host.timing = ios.timing;
    host.bus_width = ios.bus_width;
    host.state = STATE_IDLE;
    }
    static const struct mmc_host_ops sh_mmcif_ops = {
    .request	= sh_mmcif_request,
    .set_ios	= sh_mmcif_set_ios,
    .get_cd		= mmc_gpio_get_cd,
    };
#[no_mangle]
unsafe extern "C" fn sh_mmcif_end_cmd(host: *mut sh_mmcif_host) -> bool {
    static bool sh_mmcif_end_cmd(struct sh_mmcif_host *host)
    {
    struct mmc_command *cmd = host.mrq.cmd;
    struct mmc_data *data = host.mrq.data;
    struct device *dev = sh_mmcif_host_to_dev(host);
    long time;
    if (host.sd_error) {
    switch (cmd.opcode) {
    case MMC_ALL_SEND_CID:
    case MMC_SELECT_CARD:
    case MMC_APP_CMD:
    cmd.error = -ETIMEDOUT;
    break;
    default:
    cmd.error = sh_mmcif_error_manage(host);
    break;
    }
    dev_dbg(dev, "CMD%d error %d\n",
    cmd.opcode, cmd.error);
    host.sd_error = false;
    return false;
    }
    if (!(cmd.flags & MMC_RSP_PRESENT)) {
    cmd.error = 0;
    return false;
    }
    sh_mmcif_get_response(host, cmd);
    if (!data)
    return false;
//
// Completion can be signalled from DMA callback and error, so, have to
// reset here, before setting .dma_active
//
    init_completion(&host.dma_complete);
    if (data.flags & MMC_DATA_READ) {
    if (host.chan_rx)
    sh_mmcif_start_dma_rx(host);
    } else {
    if (host.chan_tx)
    sh_mmcif_start_dma_tx(host);
    }
    if (!host.dma_active) {
    data.error = sh_mmcif_data_trans(host, host.mrq, cmd.opcode);
    return !data.error;
    }
// Running in the IRQ thread, can sleep
    time = wait_for_completion_interruptible_timeout(&host.dma_complete,
    host.timeout);
    if (data.flags & MMC_DATA_READ)
    dma_unmap_sg(host.chan_rx.device.dev,
    data.sg, data.sg_len,
    DMA_FROM_DEVICE);
    else
    dma_unmap_sg(host.chan_tx.device.dev,
    data.sg, data.sg_len,
    DMA_TO_DEVICE);
    if (host.sd_error) {
    dev_err(host.mmc.parent,
    "Error IRQ while waiting for DMA completion!\n");
// Woken up by an error IRQ: abort DMA
    data.error = sh_mmcif_error_manage(host);
    } else if (!time) {
    dev_err(host.mmc.parent, "DMA timeout!\n");
    data.error = -ETIMEDOUT;
    } else if (time < 0) {
    dev_err(host.mmc.parent,
    "wait_for_completion_...() error %ld!\n", time);
    data.error = time;
    }
    sh_mmcif_bitclr(host, MMCIF_CE_BUF_ACC,
    BUF_ACC_DMAREN | BUF_ACC_DMAWEN);
    host.dma_active = false;
    if (data.error) {
    data.bytes_xfered = 0;
// Abort DMA
    if (data.flags & MMC_DATA_READ)
    dmaengine_terminate_sync(host.chan_rx);
    else
    dmaengine_terminate_sync(host.chan_tx);
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_irqt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sh_mmcif_irqt(int irq, void *dev_id)
    {
    struct sh_mmcif_host *host = dev_id;
    struct mmc_request *mrq;
    struct device *dev = sh_mmcif_host_to_dev(host);
    let mut wait: bool = false;
    unsigned long flags;
    int wait_work;
    spin_lock_irqsave(&host.lock, flags);
    wait_work = host.wait_for;
    spin_unlock_irqrestore(&host.lock, flags);
    cancel_delayed_work_sync(&host.timeout_work);
    mutex_lock(&host.thread_lock);
    mrq = host.mrq;
    if (!mrq) {
    dev_dbg(dev, "IRQ thread state %u, wait %u: core::ptr::null_mut() mrq!\n",
    host.state, host.wait_for);
    mutex_unlock(&host.thread_lock);
    return IRQ_HANDLED;
    }
//
// All handlers return true, if processing continues, and false, if the
// request has to be completed - successfully or not
//
    switch (wait_work) {
    case MMCIF_WAIT_FOR_REQUEST:
// We're too late, the timeout has already kicked in
    mutex_unlock(&host.thread_lock);
    return IRQ_HANDLED;
    case MMCIF_WAIT_FOR_CMD:
// Wait for data?
    wait = sh_mmcif_end_cmd(host);
    break;
    case MMCIF_WAIT_FOR_MREAD:
// Wait for more data?
    wait = sh_mmcif_mread_block(host);
    break;
    case MMCIF_WAIT_FOR_READ:
// Wait for data end?
    wait = sh_mmcif_read_block(host);
    break;
    case MMCIF_WAIT_FOR_MWRITE:
// Wait data to write?
    wait = sh_mmcif_mwrite_block(host);
    break;
    case MMCIF_WAIT_FOR_WRITE:
// Wait for data end?
    wait = sh_mmcif_write_block(host);
    break;
    case MMCIF_WAIT_FOR_STOP:
    if (host.sd_error) {
    mrq.stop.error = sh_mmcif_error_manage(host);
    dev_dbg(dev, "%s(): %d\n", __func__, mrq.stop.error);
    break;
    }
    sh_mmcif_get_cmd12response(host, mrq.stop);
    mrq.stop.error = 0;
    break;
    case MMCIF_WAIT_FOR_READ_END:
    case MMCIF_WAIT_FOR_WRITE_END:
    if (host.sd_error) {
    mrq.data.error = sh_mmcif_error_manage(host);
    dev_dbg(dev, "%s(): %d\n", __func__, mrq.data.error);
    }
    break;
    default:
    BUG();
    }
    if (wait) {
    schedule_delayed_work(&host.timeout_work, host.timeout);
// Wait for more data
    mutex_unlock(&host.thread_lock);
    return IRQ_HANDLED;
    }
    if (host.wait_for != MMCIF_WAIT_FOR_STOP) {
    struct mmc_data *data = mrq.data;
    if (!mrq.cmd.error && data && !data.error)
    data.bytes_xfered =
    data.blocks * data.blksz;
    if (mrq.stop && !mrq.cmd.error && (!data || !data.error)) {
    sh_mmcif_stop_cmd(host, mrq);
    if (!mrq.stop.error) {
    schedule_delayed_work(&host.timeout_work, host.timeout);
    mutex_unlock(&host.thread_lock);
    return IRQ_HANDLED;
    }
    }
    }
    host.wait_for = MMCIF_WAIT_FOR_REQUEST;
    host.state = STATE_IDLE;
    host.mrq = core::ptr::null_mut();
    mmc_request_done(host.mmc, mrq);
    mutex_unlock(&host.thread_lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_intr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sh_mmcif_intr(int irq, void *dev_id)
    {
    struct sh_mmcif_host *host = dev_id;
    struct device *dev = sh_mmcif_host_to_dev(host);
    u32 state, mask;
    state = sh_mmcif_readl(host.addr, MMCIF_CE_INT);
    mask = sh_mmcif_readl(host.addr, MMCIF_CE_INT_MASK);
    if (host.ccs_enable)
    sh_mmcif_writel(host.addr, MMCIF_CE_INT, ~(state & mask));
    else
    sh_mmcif_writel(host.addr, MMCIF_CE_INT, INT_CCS | ~(state & mask));
    sh_mmcif_bitclr(host, MMCIF_CE_INT_MASK, state & MASK_CLEAN);
    if (state & ~MASK_CLEAN)
    dev_dbg(dev, "IRQ state = 0x%08x incompletely cleared\n",
    state);
    if (state & INT_ERR_STS || state & ~INT_ALL) {
    host.sd_error = true;
    dev_dbg(dev, "int err state = 0x%08x\n", state);
    }
    if (state & ~(INT_CMD12RBE | INT_CMD12CRE)) {
    if (!host.mrq)
    dev_dbg(dev, "core::ptr::null_mut() IRQ state = 0x%08x\n", state);
    if (!host.dma_active)
    return IRQ_WAKE_THREAD;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: host->sd_error) -> else {
    else if (host.sd_error)
    sh_mmcif_dma_complete(host);
    } else {
    dev_dbg(dev, "Unexpected IRQ 0x%x\n", state);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_timeout_work(work: *mut work_struct) {
    static void sh_mmcif_timeout_work(struct work_struct *work)
    {
    struct delayed_work *d = to_delayed_work(work);
    struct sh_mmcif_host *host = container_of(d, struct sh_mmcif_host, timeout_work);
    struct mmc_request *mrq = host.mrq;
    struct device *dev = sh_mmcif_host_to_dev(host);
    unsigned long flags;
    if (host.dying)
// Don't run after mmc_remove_host()
    return;
    spin_lock_irqsave(&host.lock, flags);
    if (host.state == STATE_IDLE) {
    spin_unlock_irqrestore(&host.lock, flags);
    return;
    }
    dev_err(dev, "Timeout waiting for %u on CMD%u\n",
    host.wait_for, mrq.cmd.opcode);
    host.state = STATE_TIMEOUT;
    spin_unlock_irqrestore(&host.lock, flags);
//
// Handle races with cancel_delayed_work(), unless
// cancel_delayed_work_sync() is used
//
    switch (host.wait_for) {
    case MMCIF_WAIT_FOR_CMD:
    mrq.cmd.error = sh_mmcif_error_manage(host);
    break;
    case MMCIF_WAIT_FOR_STOP:
    mrq.stop.error = sh_mmcif_error_manage(host);
    break;
    case MMCIF_WAIT_FOR_MREAD:
    case MMCIF_WAIT_FOR_MWRITE:
    case MMCIF_WAIT_FOR_READ:
    case MMCIF_WAIT_FOR_WRITE:
    case MMCIF_WAIT_FOR_READ_END:
    case MMCIF_WAIT_FOR_WRITE_END:
    mrq.data.error = sh_mmcif_error_manage(host);
    break;
    default:
    BUG();
    }
    host.state = STATE_IDLE;
    host.wait_for = MMCIF_WAIT_FOR_REQUEST;
    host.mrq = core::ptr::null_mut();
    mmc_request_done(host.mmc, mrq);
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_init_ocr(host: *mut sh_mmcif_host) {
    static void sh_mmcif_init_ocr(struct sh_mmcif_host *host)
    {
    struct device *dev = sh_mmcif_host_to_dev(host);
    struct sh_mmcif_plat_data *pd = dev.platform_data;
    struct mmc_host *mmc = host.mmc;
    mmc_regulator_get_supply(mmc);
    if (!pd)
    return;
    if (!mmc.ocr_avail)
    mmc.ocr_avail = pd.ocr;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: pd->ocr) -> else {
    else if (pd.ocr)
    dev_warn(mmc_dev(mmc), "Platform OCR mask is ignored\n");
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_probe(pdev: *mut platform_device) -> c_int {
    static int sh_mmcif_probe(struct platform_device *pdev)
    {
    let mut ret: c_int = 0, irq[2];
    struct mmc_host *mmc;
    struct sh_mmcif_host *host;
    struct device *dev = &pdev.dev;
    struct sh_mmcif_plat_data *pd = dev.platform_data;
    void __iomem *reg;
    const char *name;
    irq[0] = platform_get_irq(pdev, 0);
    irq[1] = platform_get_irq_optional(pdev, 1);
    if (irq[0] < 0)
    return irq[0];
    reg = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(reg))
    return PTR_ERR(reg);
    mmc = devm_mmc_alloc_host(dev, sizeof(*host));
    if (!mmc)
    return -ENOMEM;
    ret = mmc_of_parse(mmc);
    if (ret < 0)
    return ret;
    host		= mmc_priv(mmc);
    host.mmc	= mmc;
    host.addr	= reg;
    host.timeout	= msecs_to_jiffies(10000);
    host.ccs_enable = true;
    host.clk_ctrl2_enable = false;
    host.pd = pdev;
    spin_lock_init(&host.lock);
    mmc.ops = &sh_mmcif_ops;
    sh_mmcif_init_ocr(host);
    mmc.caps |= MMC_CAP_MMC_HIGHSPEED | MMC_CAP_WAIT_WHILE_BUSY;
    mmc.caps2 |= MMC_CAP2_NO_SD | MMC_CAP2_NO_SDIO;
    mmc.max_busy_timeout = 10000;
    if (pd && pd.caps)
    mmc.caps |= pd.caps;
    mmc.max_segs = 32;
    mmc.max_blk_size = 512;
    mmc.max_req_size = PAGE_SIZE * mmc.max_segs;
    mmc.max_blk_count = mmc.max_req_size / mmc.max_blk_size;
    mmc.max_seg_size = mmc.max_req_size;
    platform_set_drvdata(pdev, host);
    host.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(host.clk))
    return dev_err_probe(dev, PTR_ERR(host.clk),
    "cannot get clock\n");
    ret = clk_prepare_enable(host.clk);
    if (ret < 0)
    return ret;
    sh_mmcif_clk_setup(host);
    pm_runtime_enable(dev);
    host.power = false;
    ret = pm_runtime_get_sync(dev);
    if (ret < 0)
    goto err_clk;
    INIT_DELAYED_WORK(&host.timeout_work, sh_mmcif_timeout_work);
    sh_mmcif_sync_reset(host);
    sh_mmcif_writel(host.addr, MMCIF_CE_INT_MASK, MASK_ALL);
    name = irq[1] < 0 ? dev_name(dev) : "sh_mmc:error";
    ret = devm_request_threaded_irq(dev, irq[0], sh_mmcif_intr,
    sh_mmcif_irqt, 0, name, host);
    if (ret)
    goto err_clk;
    if (irq[1] >= 0) {
    ret = devm_request_threaded_irq(dev, irq[1],
    sh_mmcif_intr, sh_mmcif_irqt,
    0, "sh_mmc:int", host);
    if (ret)
    goto err_clk;
    }
    mutex_init(&host.thread_lock);
    ret = mmc_add_host(mmc);
    if (ret < 0)
    goto err_clk;
    dev_pm_qos_expose_latency_limit(dev, 100);
    dev_info(dev, "Chip version 0x%04x, clock rate %luMHz\n",
    sh_mmcif_readl(host.addr, MMCIF_CE_VERSION) & 0xffff,
    clk_get_rate(host.clk) / 1000000UL);
    pm_runtime_put(dev);
    clk_disable_unprepare(host.clk);
    return ret;
    err_clk:
    clk_disable_unprepare(host.clk);
    pm_runtime_put_sync(dev);
    pm_runtime_disable(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_remove(pdev: *mut platform_device) {
    static void sh_mmcif_remove(struct platform_device *pdev)
    {
    struct sh_mmcif_host *host = platform_get_drvdata(pdev);
    host.dying = true;
    clk_prepare_enable(host.clk);
    pm_runtime_get_sync(&pdev.dev);
    dev_pm_qos_hide_latency_limit(&pdev.dev);
    mmc_remove_host(host.mmc);
    sh_mmcif_writel(host.addr, MMCIF_CE_INT_MASK, MASK_ALL);
//
// FIXME: cancel_delayed_work(_sync)() and free_irq() race with the
// mmc_remove_host() call above. But swapping order doesn't help either
// (a query on the linux-mmc mailing list didn't bring any replies).
//
    cancel_delayed_work_sync(&host.timeout_work);
    clk_disable_unprepare(host.clk);
    pm_runtime_put_sync(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn sh_mmcif_suspend(dev: *mut device) -> c_int {
    static int sh_mmcif_suspend(struct device *dev)
    {
    struct sh_mmcif_host *host = dev_get_drvdata(dev);
    pm_runtime_get_sync(dev);
    sh_mmcif_writel(host.addr, MMCIF_CE_INT_MASK, MASK_ALL);
    pm_runtime_put(dev);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(sh_mmcif_dev_pm_ops, sh_mmcif_suspend, core::ptr::null_mut());
    static struct platform_driver sh_mmcif_driver = {
    .probe		= sh_mmcif_probe,
    .remove		= sh_mmcif_remove,
    .driver		= {
    .name	= DRIVER_NAME,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .pm	= pm_sleep_ptr(&sh_mmcif_dev_pm_ops),
    .of_match_table = sh_mmcif_of_match,
    },
    };
    module_platform_driver(sh_mmcif_driver);
    MODULE_DESCRIPTION("SuperH on-chip MMC/eMMC interface driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:" DRIVER_NAME);
    MODULE_AUTHOR("Yusuke Goda <yusuke.goda.sx@renesas.com>");
