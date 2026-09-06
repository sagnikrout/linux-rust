//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/mxcmmc.c
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
// linux/drivers/mmc/host/mxcmmc.c - Freescale i.MX MMCI driver
//
// This is a driver for the SDHC controller found in Freescale MX2/MX3
// SoCs. It is basically the same hardware as found on MX1 (imxmmc.c).
// Unlike the hardware found on MX1, this hardware just works and does
// not need all the quirks found in imxmmc.c, hence the separate driver.
//
// Copyright (C) 2008 Sascha Hauer, Pengutronix <s.hauer@pengutronix.de>
// Copyright (C) 2006 Pavel Pisa, PiKRON <ppisa@pikron.com>
//
// derived from pxamci.c by Russell King
//

pub const MXCMCI_TIMEOUT_MS: c_int = 10000;
pub const MMC_REG_STR_STP_CLK: c_uint = 0x00;
pub const MMC_REG_STATUS: c_uint = 0x04;
pub const MMC_REG_CLK_RATE: c_uint = 0x08;
pub const MMC_REG_CMD_DAT_CONT: c_uint = 0x0C;
pub const MMC_REG_RES_TO: c_uint = 0x10;
pub const MMC_REG_READ_TO: c_uint = 0x14;
pub const MMC_REG_BLK_LEN: c_uint = 0x18;
pub const MMC_REG_NOB: c_uint = 0x1C;
pub const MMC_REG_REV_NO: c_uint = 0x20;
pub const MMC_REG_INT_CNTR: c_uint = 0x24;
pub const MMC_REG_CMD: c_uint = 0x28;
pub const MMC_REG_ARG: c_uint = 0x2C;
pub const MMC_REG_RES_FIFO: c_uint = 0x34;
pub const MMC_REG_BUFFER_ACCESS: c_uint = 0x38;

pub const STATUS_ERR_MASK: c_uint = 0x2f;

    enum mxcmci_type {
    IMX21_MMC,
    IMX31_MMC,
    MPC512X_MMC,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxcmci_host {
    pub mmc: *mut mmc_host,
    pub base: *mut void __iomem,
    pub phys_base: dma_addr_t,
    pub detect_irq: c_int,
    pub dma: *mut dma_chan,
    pub desc: *mut dma_async_tx_descriptor,
    pub do_dma: c_int,
    pub default_irq_mask: c_int,
    pub use_sdio: c_int,
    pub power_mode: c_uint,
    pub pdata: *mut imxmmc_platform_data,
    pub req: *mut mmc_request,
    pub cmd: *mut mmc_command,
    pub data: *mut mmc_data,
    pub datasize: c_uint,
    pub dma_dir: c_uint,
    pub rev_no: u16,
    pub cmdat: c_uint,
    pub clk_ipg: *mut clk,
    pub clk_per: *mut clk,
    pub clock: c_int,
    pub datawork: work_struct,
    pub lock: spinlock_t,
    pub burstlen: c_int,
    pub dmareq: c_int,
    pub dma_slave_config: dma_slave_config,
    pub dma_data: imx_dma_data,
    pub watchdog: timer_list,
    pub devtype: enum mxcmci_type,
}

    static const struct of_device_id mxcmci_of_match[] = {
    {
    .compatible = "fsl,imx21-mmc",
    .data = (void *) IMX21_MMC,
    }, {
    .compatible = "fsl,imx31-mmc",
    .data = (void *) IMX31_MMC,
    }, {
    .compatible = "fsl,mpc5121-sdhc",
    .data = (void *) MPC512X_MMC,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, mxcmci_of_match);
#[no_mangle]
pub unsafe extern "C" fn is_imx31_mmc(host: *mut mxcmci_host) -> c_int {
    static inline int is_imx31_mmc(struct mxcmci_host *host)
    {
    return host.devtype == IMX31_MMC;
    }
#[no_mangle]
pub unsafe extern "C" fn is_mpc512x_mmc(host: *mut mxcmci_host) -> c_int {
    static inline int is_mpc512x_mmc(struct mxcmci_host *host)
    {
    return host.devtype == MPC512X_MMC;
    }
#[no_mangle]
pub unsafe extern "C" fn mxcmci_readl(host: *mut mxcmci_host, reg: c_int) -> u32 {
    static inline u32 mxcmci_readl(struct mxcmci_host *host, int reg)
    {
    if (IS_ENABLED(CONFIG_PPC_MPC512x))
    return ioread32be(host.base + reg);
    else
    return readl(host.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn mxcmci_writel(host: *mut mxcmci_host, val: u32, reg: c_int) {
    static inline void mxcmci_writel(struct mxcmci_host *host, u32 val, int reg)
    {
    if (IS_ENABLED(CONFIG_PPC_MPC512x))
    iowrite32be(val, host.base + reg);
    else
    writel(val, host.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn mxcmci_readw(host: *mut mxcmci_host, reg: c_int) -> u16 {
    static inline u16 mxcmci_readw(struct mxcmci_host *host, int reg)
    {
    if (IS_ENABLED(CONFIG_PPC_MPC512x))
    return ioread32be(host.base + reg);
    else
    return readw(host.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn mxcmci_writew(host: *mut mxcmci_host, val: u16, reg: c_int) {
    static inline void mxcmci_writew(struct mxcmci_host *host, u16 val, int reg)
    {
    if (IS_ENABLED(CONFIG_PPC_MPC512x))
    iowrite32be(val, host.base + reg);
    else
    writew(val, host.base + reg);
    }
    static void mxcmci_set_clk_rate(struct mxcmci_host *host, unsigned int clk_ios);
#[no_mangle]
unsafe extern "C" fn mxcmci_set_power(host: *mut mxcmci_host, vdd: c_uint) {
    static void mxcmci_set_power(struct mxcmci_host *host, unsigned int vdd)
    {
    if (!IS_ERR(host.mmc.supply.vmmc)) {
    if (host.power_mode == MMC_POWER_UP)
    mmc_regulator_set_ocr(host.mmc,
    host.mmc.supply.vmmc, vdd);
#[no_mangle]
pub unsafe extern "C" fn if(MMC_POWER_OFF: host->power_mode ==) -> else {
    else if (host.power_mode == MMC_POWER_OFF)
    mmc_regulator_set_ocr(host.mmc,
    host.mmc.supply.vmmc, 0);
    }
    if (host.pdata && host.pdata.setpower)
    host.pdata.setpower(mmc_dev(host.mmc), vdd);
    }
#[no_mangle]
pub unsafe extern "C" fn mxcmci_use_dma(host: *mut mxcmci_host) -> c_int {
    static inline int mxcmci_use_dma(struct mxcmci_host *host)
    {
    return host.do_dma;
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_softreset(host: *mut mxcmci_host) {
    static void mxcmci_softreset(struct mxcmci_host *host)
    {
    int i;
    dev_dbg(mmc_dev(host.mmc), "mxcmci_softreset\n");
// reset sequence
    mxcmci_writew(host, STR_STP_CLK_RESET, MMC_REG_STR_STP_CLK);
    mxcmci_writew(host, STR_STP_CLK_RESET | STR_STP_CLK_START_CLK,
    MMC_REG_STR_STP_CLK);
    for (i = 0; i < 8; i++)
    mxcmci_writew(host, STR_STP_CLK_START_CLK, MMC_REG_STR_STP_CLK);
    mxcmci_writew(host, 0xff, MMC_REG_RES_TO);
    }

#[no_mangle]
pub unsafe extern "C" fn buffer_swap32(buf: *mut u32, len: c_int) {
    static inline void buffer_swap32(u32 *buf, int len)
    {
    int i;
    for (i = 0; i < ((len + 3) / 4); i++) {
// buf = swab32(*buf);
    buf++;
    }
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_swap_buffers(data: *mut mmc_data) {
    static void mxcmci_swap_buffers(struct mmc_data *data)
    {
    struct sg_mapping_iter sgm;
    u32 *buf;
    sg_miter_start(&sgm, data.sg, data.sg_len,
    SG_MITER_TO_SG | SG_MITER_FROM_SG);
    while (sg_miter_next(&sgm)) {
    buf = sgm.addr;
    buffer_swap32(buf, sgm.length);
    }
    sg_miter_stop(&sgm);
    }

    static inline void mxcmci_swap_buffers(struct mmc_data *data) {}

#[no_mangle]
unsafe extern "C" fn mxcmci_setup_data(host: *mut mxcmci_host, data: *mut mmc_data) -> c_int {
    static int mxcmci_setup_data(struct mxcmci_host *host, struct mmc_data *data)
    {
    let mut nob: c_uint = data.blocks;
    let mut blksz: c_uint = data.blksz;
    let mut datasize: c_uint = nob * blksz;
    struct scatterlist *sg;
    enum dma_transfer_direction slave_dirn;
    int i, nents;
    host.data = data;
    data.bytes_xfered = 0;
    mxcmci_writew(host, nob, MMC_REG_NOB);
    mxcmci_writew(host, blksz, MMC_REG_BLK_LEN);
    host.datasize = datasize;
    if (!mxcmci_use_dma(host))
    return 0;
    for_each_sg(data.sg, sg, data.sg_len, i) {
    if (sg.offset & 3 || sg.length & 3 || sg.length < 512) {
    host.do_dma = 0;
    return 0;
    }
    }
    if (data.flags & MMC_DATA_READ) {
    host.dma_dir = DMA_FROM_DEVICE;
    slave_dirn = DMA_DEV_TO_MEM;
    } else {
    host.dma_dir = DMA_TO_DEVICE;
    slave_dirn = DMA_MEM_TO_DEV;
    mxcmci_swap_buffers(data);
    }
    nents = dma_map_sg(host.dma.device.dev, data.sg,
    data.sg_len,  host.dma_dir);
    if (nents != data.sg_len)
    return -EINVAL;
    host.desc = dmaengine_prep_slave_sg(host.dma,
    data.sg, data.sg_len, slave_dirn,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!host.desc) {
    dma_unmap_sg(host.dma.device.dev, data.sg, data.sg_len,
    host.dma_dir);
    host.do_dma = 0;
    return 0; /* Fall back to PIO */
    }
    wmb();
    dmaengine_submit(host.desc);
    dma_async_issue_pending(host.dma);
    mod_timer(&host.watchdog, jiffies + msecs_to_jiffies(MXCMCI_TIMEOUT_MS));
    return 0;
    }
    static void mxcmci_cmd_done(struct mxcmci_host *host, unsigned int stat);
    static void mxcmci_data_done(struct mxcmci_host *host, unsigned int stat);
#[no_mangle]
unsafe extern "C" fn mxcmci_dma_callback(data: *mut c_void) {
    static void mxcmci_dma_callback(void *data)
    {
    struct mxcmci_host *host = data;
    u32 stat;
    timer_delete(&host.watchdog);
    stat = mxcmci_readl(host, MMC_REG_STATUS);
    dev_dbg(mmc_dev(host.mmc), "%s: 0x%08x\n", __func__, stat);
    mxcmci_data_done(host, stat);
    }
    static int mxcmci_start_cmd(struct mxcmci_host *host, struct mmc_command *cmd,
    unsigned int cmdat)
    {
    let mut int_cntr: u32 = host.default_irq_mask;
    unsigned long flags;
    WARN_ON(host.cmd != core::ptr::null_mut());
    host.cmd = cmd;
    switch (mmc_resp_type(cmd)) {
    case MMC_RSP_R1: /* short CRC, OPCODE */
    case MMC_RSP_R1B:/* short CRC, OPCODE, BUSY */
    cmdat |= CMD_DAT_CONT_RESPONSE_48BIT_CRC;
    break;
    case MMC_RSP_R2: /* long 136 bit + CRC */
    cmdat |= CMD_DAT_CONT_RESPONSE_136BIT;
    break;
    case MMC_RSP_R3: /* short */
    cmdat |= CMD_DAT_CONT_RESPONSE_48BIT;
    break;
    case MMC_RSP_NONE:
    break;
    default:
    dev_err(mmc_dev(host.mmc), "unhandled response type 0x%x\n",
    mmc_resp_type(cmd));
    cmd.error = -EINVAL;
    return -EINVAL;
    }
    int_cntr = INT_END_CMD_RES_EN;
    if (mxcmci_use_dma(host)) {
    if (host.dma_dir == DMA_FROM_DEVICE) {
    host.desc.callback = mxcmci_dma_callback;
    host.desc.callback_param = host;
    } else {
    int_cntr |= INT_WRITE_OP_DONE_EN;
    }
    }
    spin_lock_irqsave(&host.lock, flags);
    if (host.use_sdio)
    int_cntr |= INT_SDIO_IRQ_EN;
    mxcmci_writel(host, int_cntr, MMC_REG_INT_CNTR);
    spin_unlock_irqrestore(&host.lock, flags);
    mxcmci_writew(host, cmd.opcode, MMC_REG_CMD);
    mxcmci_writel(host, cmd.arg, MMC_REG_ARG);
    mxcmci_writew(host, cmdat, MMC_REG_CMD_DAT_CONT);
    return 0;
    }
    static void mxcmci_finish_request(struct mxcmci_host *host,
    struct mmc_request *req)
    {
    let mut int_cntr: u32 = host.default_irq_mask;
    unsigned long flags;
    spin_lock_irqsave(&host.lock, flags);
    if (host.use_sdio)
    int_cntr |= INT_SDIO_IRQ_EN;
    mxcmci_writel(host, int_cntr, MMC_REG_INT_CNTR);
    spin_unlock_irqrestore(&host.lock, flags);
    host.req = core::ptr::null_mut();
    host.cmd = core::ptr::null_mut();
    host.data = core::ptr::null_mut();
    mmc_request_done(host.mmc, req);
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_finish_data(host: *mut mxcmci_host, stat: c_uint) -> c_int {
    static int mxcmci_finish_data(struct mxcmci_host *host, unsigned int stat)
    {
    struct mmc_data *data = host.data;
    int data_error;
    if (mxcmci_use_dma(host)) {
    dma_unmap_sg(host.dma.device.dev, data.sg, data.sg_len,
    host.dma_dir);
    mxcmci_swap_buffers(data);
    }
    if (stat & STATUS_ERR_MASK) {
    dev_dbg(mmc_dev(host.mmc), "request failed. status: 0x%08x\n",
    stat);
    if (stat & STATUS_CRC_READ_ERR) {
    dev_err(mmc_dev(host.mmc), "%s: -EILSEQ\n", __func__);
    data.error = -EILSEQ;
    } else if (stat & STATUS_CRC_WRITE_ERR) {
    let mut err_code: u32 = (stat >> 9) & 0x3;
    if (err_code == 2) { /* No CRC response */
    dev_err(mmc_dev(host.mmc),
    "%s: No CRC -ETIMEDOUT\n", __func__);
    data.error = -ETIMEDOUT;
    } else {
    dev_err(mmc_dev(host.mmc),
    "%s: -EILSEQ\n", __func__);
    data.error = -EILSEQ;
    }
    } else if (stat & STATUS_TIME_OUT_READ) {
    dev_err(mmc_dev(host.mmc),
    "%s: read -ETIMEDOUT\n", __func__);
    data.error = -ETIMEDOUT;
    } else {
    dev_err(mmc_dev(host.mmc), "%s: -EIO\n", __func__);
    data.error = -EIO;
    }
    } else {
    data.bytes_xfered = host.datasize;
    }
    data_error = data.error;
    host.data = core::ptr::null_mut();
    return data_error;
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_read_response(host: *mut mxcmci_host, stat: c_uint) {
    static void mxcmci_read_response(struct mxcmci_host *host, unsigned int stat)
    {
    struct mmc_command *cmd = host.cmd;
    int i;
    u32 a, b, c;
    if (!cmd)
    return;
    if (stat & STATUS_TIME_OUT_RESP) {
    dev_dbg(mmc_dev(host.mmc), "CMD TIMEOUT\n");
    cmd.error = -ETIMEDOUT;
    } else if (stat & STATUS_RESP_CRC_ERR && cmd.flags & MMC_RSP_CRC) {
    dev_dbg(mmc_dev(host.mmc), "cmd crc error\n");
    cmd.error = -EILSEQ;
    }
    if (cmd.flags & MMC_RSP_PRESENT) {
    if (cmd.flags & MMC_RSP_136) {
    for (i = 0; i < 4; i++) {
    a = mxcmci_readw(host, MMC_REG_RES_FIFO);
    b = mxcmci_readw(host, MMC_REG_RES_FIFO);
    cmd.resp[i] = a << 16 | b;
    }
    } else {
    a = mxcmci_readw(host, MMC_REG_RES_FIFO);
    b = mxcmci_readw(host, MMC_REG_RES_FIFO);
    c = mxcmci_readw(host, MMC_REG_RES_FIFO);
    cmd.resp[0] = a << 24 | b << 8 | c >> 8;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_poll_status(host: *mut mxcmci_host, mask: u32) -> c_int {
    static int mxcmci_poll_status(struct mxcmci_host *host, u32 mask)
    {
    u32 stat;
    let mut timeout: c_ulong = jiffies + HZ;
    do {
    stat = mxcmci_readl(host, MMC_REG_STATUS);
    if (stat & STATUS_ERR_MASK)
    return stat;
    if (time_after(jiffies, timeout)) {
    mxcmci_softreset(host);
    mxcmci_set_clk_rate(host, host.clock);
    return STATUS_TIME_OUT_READ;
    }
    if (stat & mask)
    return 0;
    cpu_relax();
    } while (1);
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_pull(host: *mut mxcmci_host, buf: *mut u32, bytes: c_int) -> c_int {
    static int mxcmci_pull(struct mxcmci_host *host, u32 *buf, int bytes)
    {
    unsigned int stat;
    while (bytes > 3) {
    stat = mxcmci_poll_status(host,
    STATUS_BUF_READ_RDY | STATUS_READ_OP_DONE);
    if (stat)
    return stat;
// buf++ = cpu_to_le32(mxcmci_readl(host, MMC_REG_BUFFER_ACCESS));
    bytes -= 4;
    }
    if (bytes) {
    u8 *b = (u8 *)buf;
    u32 tmp;
    stat = mxcmci_poll_status(host,
    STATUS_BUF_READ_RDY | STATUS_READ_OP_DONE);
    if (stat)
    return stat;
    tmp = cpu_to_le32(mxcmci_readl(host, MMC_REG_BUFFER_ACCESS));
    memcpy(b, &tmp, bytes);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_push(host: *mut mxcmci_host, buf: *mut u32, bytes: c_int) -> c_int {
    static int mxcmci_push(struct mxcmci_host *host, u32 *buf, int bytes)
    {
    unsigned int stat;
    while (bytes > 3) {
    stat = mxcmci_poll_status(host, STATUS_BUF_WRITE_RDY);
    if (stat)
    return stat;
    mxcmci_writel(host, cpu_to_le32(*buf++), MMC_REG_BUFFER_ACCESS);
    bytes -= 4;
    }
    if (bytes) {
    u8 *b = (u8 *)buf;
    u32 tmp;
    stat = mxcmci_poll_status(host, STATUS_BUF_WRITE_RDY);
    if (stat)
    return stat;
    memcpy(&tmp, b, bytes);
    mxcmci_writel(host, cpu_to_le32(tmp), MMC_REG_BUFFER_ACCESS);
    }
    return mxcmci_poll_status(host, STATUS_BUF_WRITE_RDY);
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_transfer_data(host: *mut mxcmci_host) -> c_int {
    static int mxcmci_transfer_data(struct mxcmci_host *host)
    {
    struct mmc_data *data = host.req.data;
    struct sg_mapping_iter sgm;
    int stat;
    u32 *buf;
    host.data = data;
    host.datasize = 0;
    sg_miter_start(&sgm, data.sg, data.sg_len,
    (data.flags & MMC_DATA_READ) ? SG_MITER_TO_SG : SG_MITER_FROM_SG);
    if (data.flags & MMC_DATA_READ) {
    while (sg_miter_next(&sgm)) {
    buf = sgm.addr;
    stat = mxcmci_pull(host, buf, sgm.length);
    if (stat)
    goto transfer_error;
    host.datasize += sgm.length;
    }
    } else {
    while (sg_miter_next(&sgm)) {
    buf = sgm.addr;
    stat = mxcmci_push(host, buf, sgm.length);
    if (stat)
    goto transfer_error;
    host.datasize += sgm.length;
    }
    stat = mxcmci_poll_status(host, STATUS_WRITE_OP_DONE);
    if (stat)
    goto transfer_error;
    }
    transfer_error:
    sg_miter_stop(&sgm);
    return stat;
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_datawork(work: *mut work_struct) {
    static void mxcmci_datawork(struct work_struct *work)
    {
    struct mxcmci_host *host = container_of(work, struct mxcmci_host,
    datawork);
    let mut datastat: c_int = mxcmci_transfer_data(host);
    mxcmci_writel(host, STATUS_READ_OP_DONE | STATUS_WRITE_OP_DONE,
    MMC_REG_STATUS);
    mxcmci_finish_data(host, datastat);
    if (host.req.stop) {
    if (mxcmci_start_cmd(host, host.req.stop, 0)) {
    mxcmci_finish_request(host, host.req);
    return;
    }
    } else {
    mxcmci_finish_request(host, host.req);
    }
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_data_done(host: *mut mxcmci_host, stat: c_uint) {
    static void mxcmci_data_done(struct mxcmci_host *host, unsigned int stat)
    {
    struct mmc_request *req;
    int data_error;
    unsigned long flags;
    spin_lock_irqsave(&host.lock, flags);
    if (!host.data) {
    spin_unlock_irqrestore(&host.lock, flags);
    return;
    }
    if (!host.req) {
    spin_unlock_irqrestore(&host.lock, flags);
    return;
    }
    req = host.req;
    if (!req.stop)
    host.req = core::ptr::null_mut(); /* we will handle finish req below */
    data_error = mxcmci_finish_data(host, stat);
    spin_unlock_irqrestore(&host.lock, flags);
    if (data_error)
    return;
    mxcmci_read_response(host, stat);
    host.cmd = core::ptr::null_mut();
    if (req.stop) {
    if (mxcmci_start_cmd(host, req.stop, 0)) {
    mxcmci_finish_request(host, req);
    return;
    }
    } else {
    mxcmci_finish_request(host, req);
    }
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_cmd_done(host: *mut mxcmci_host, stat: c_uint) {
    static void mxcmci_cmd_done(struct mxcmci_host *host, unsigned int stat)
    {
    mxcmci_read_response(host, stat);
    host.cmd = core::ptr::null_mut();
    if (!host.data && host.req) {
    mxcmci_finish_request(host, host.req);
    return;
    }
// For the DMA case the DMA engine handles the data transfer
// automatically. For non DMA we have to do it ourselves.
// Don't do it in interrupt context though.
//
    if (!mxcmci_use_dma(host) && host.data)
    schedule_work(&host.datawork);
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_irq(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t mxcmci_irq(int irq, void *devid)
    {
    struct mxcmci_host *host = devid;
    bool sdio_irq;
    u32 stat;
    stat = mxcmci_readl(host, MMC_REG_STATUS);
    mxcmci_writel(host,
    stat & ~(STATUS_SDIO_INT_ACTIVE | STATUS_DATA_TRANS_DONE |
    STATUS_WRITE_OP_DONE),
    MMC_REG_STATUS);
    dev_dbg(mmc_dev(host.mmc), "%s: 0x%08x\n", __func__, stat);
    spin_lock(&host.lock);
    sdio_irq = (stat & STATUS_SDIO_INT_ACTIVE) && host.use_sdio;
    spin_unlock(&host.lock);
    if (mxcmci_use_dma(host) && (stat & (STATUS_WRITE_OP_DONE)))
    mxcmci_writel(host, STATUS_WRITE_OP_DONE, MMC_REG_STATUS);
    if (sdio_irq) {
    mxcmci_writel(host, STATUS_SDIO_INT_ACTIVE, MMC_REG_STATUS);
    mmc_signal_sdio_irq(host.mmc);
    }
    if (stat & STATUS_END_CMD_RESP)
    mxcmci_cmd_done(host, stat);
    if (mxcmci_use_dma(host) && (stat & STATUS_WRITE_OP_DONE)) {
    timer_delete(&host.watchdog);
    mxcmci_data_done(host, stat);
    }
    if (host.default_irq_mask &&
    (stat & (STATUS_CARD_INSERTION | STATUS_CARD_REMOVAL)))
    mmc_detect_change(host.mmc, msecs_to_jiffies(200));
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_request(mmc: *mut mmc_host, req: *mut mmc_request) {
    static void mxcmci_request(struct mmc_host *mmc, struct mmc_request *req)
    {
    struct mxcmci_host *host = mmc_priv(mmc);
    let mut cmdat: c_uint = host.cmdat;
    int error;
    WARN_ON(host.req != core::ptr::null_mut());
    host.req = req;
    host.cmdat &= ~CMD_DAT_CONT_INIT;
    if (host.dma)
    host.do_dma = 1;
    if (req.data) {
    error = mxcmci_setup_data(host, req.data);
    if (error) {
    req.cmd.error = error;
    goto out;
    }
    cmdat |= CMD_DAT_CONT_DATA_ENABLE;
    if (req.data.flags & MMC_DATA_WRITE)
    cmdat |= CMD_DAT_CONT_WRITE;
    }
    error = mxcmci_start_cmd(host, req.cmd, cmdat);
    out:
    if (error)
    mxcmci_finish_request(host, req);
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_set_clk_rate(host: *mut mxcmci_host, clk_ios: c_uint) {
    static void mxcmci_set_clk_rate(struct mxcmci_host *host, unsigned int clk_ios)
    {
    unsigned int divider;
    let mut prescaler: c_int = 0;
    let mut clk_in: c_uint = clk_get_rate(host.clk_per);
    while (prescaler <= 0x800) {
    for (divider = 1; divider <= 0xF; divider++) {
    int x;
    x = (clk_in / (divider + 1));
    if (prescaler)
    x /= (prescaler * 2);
    if (x <= clk_ios)
    break;
    }
    if (divider < 0x10)
    break;
    if (prescaler == 0)
    prescaler = 1;
    else
    prescaler <<= 1;
    }
    mxcmci_writew(host, (prescaler << 4) | divider, MMC_REG_CLK_RATE);
    dev_dbg(mmc_dev(host.mmc), "scaler: %d divider: %d in: %d out: %d\n",
    prescaler, divider, clk_in, clk_ios);
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_setup_dma(mmc: *mut mmc_host) -> c_int {
    static int mxcmci_setup_dma(struct mmc_host *mmc)
    {
    struct mxcmci_host *host = mmc_priv(mmc);
    struct dma_slave_config *config = &host.dma_slave_config;
    config.dst_addr = host.phys_base + MMC_REG_BUFFER_ACCESS;
    config.src_addr = host.phys_base + MMC_REG_BUFFER_ACCESS;
    config.dst_addr_width = 4;
    config.src_addr_width = 4;
    config.dst_maxburst = host.burstlen;
    config.src_maxburst = host.burstlen;
    config.device_fc = false;
    return dmaengine_slave_config(host.dma, config);
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_set_ios(mmc: *mut mmc_host, ios: *mut mmc_ios) {
    static void mxcmci_set_ios(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    struct mxcmci_host *host = mmc_priv(mmc);
    int burstlen, ret;
//
// use burstlen of 64 (16 words) in 4 bit mode (--> reg value  0)
// use burstlen of 16 (4 words) in 1 bit mode (--> reg value 16)
//
    if (ios.bus_width == MMC_BUS_WIDTH_4)
    burstlen = 16;
    else
    burstlen = 4;
    if (mxcmci_use_dma(host) && burstlen != host.burstlen) {
    host.burstlen = burstlen;
    ret = mxcmci_setup_dma(mmc);
    if (ret) {
    dev_err(mmc_dev(host.mmc),
    "failed to config DMA channel. Falling back to PIO\n");
    dma_release_channel(host.dma);
    host.do_dma = 0;
    host.dma = core::ptr::null_mut();
    }
    }
    if (ios.bus_width == MMC_BUS_WIDTH_4)
    host.cmdat |= CMD_DAT_CONT_BUS_WIDTH_4;
    else
    host.cmdat &= ~CMD_DAT_CONT_BUS_WIDTH_4;
    if (host.power_mode != ios.power_mode) {
    host.power_mode = ios.power_mode;
    mxcmci_set_power(host, ios.vdd);
    if (ios.power_mode == MMC_POWER_ON)
    host.cmdat |= CMD_DAT_CONT_INIT;
    }
    if (ios.clock) {
    mxcmci_set_clk_rate(host, ios.clock);
    mxcmci_writew(host, STR_STP_CLK_START_CLK, MMC_REG_STR_STP_CLK);
    } else {
    mxcmci_writew(host, STR_STP_CLK_STOP_CLK, MMC_REG_STR_STP_CLK);
    }
    host.clock = ios.clock;
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_detect_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t mxcmci_detect_irq(int irq, void *data)
    {
    struct mmc_host *mmc = data;
    dev_dbg(mmc_dev(mmc), "%s\n", __func__);
    mmc_detect_change(mmc, msecs_to_jiffies(250));
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_get_ro(mmc: *mut mmc_host) -> c_int {
    static int mxcmci_get_ro(struct mmc_host *mmc)
    {
    struct mxcmci_host *host = mmc_priv(mmc);
    if (host.pdata && host.pdata.get_ro)
    return !!host.pdata.get_ro(mmc_dev(mmc));
//
// If board doesn't support read only detection (no mmc_gpio
// context or gpio is invalid), then let the mmc core decide
// what to do.
//
    return mmc_gpio_get_ro(mmc);
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_enable_sdio_irq(mmc: *mut mmc_host, enable: c_int) {
    static void mxcmci_enable_sdio_irq(struct mmc_host *mmc, int enable)
    {
    struct mxcmci_host *host = mmc_priv(mmc);
    unsigned long flags;
    u32 int_cntr;
    spin_lock_irqsave(&host.lock, flags);
    host.use_sdio = enable;
    int_cntr = mxcmci_readl(host, MMC_REG_INT_CNTR);
    if (enable)
    int_cntr |= INT_SDIO_IRQ_EN;
    else
    int_cntr &= ~INT_SDIO_IRQ_EN;
    mxcmci_writel(host, int_cntr, MMC_REG_INT_CNTR);
    spin_unlock_irqrestore(&host.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_init_card(host: *mut mmc_host, card: *mut mmc_card) {
    static void mxcmci_init_card(struct mmc_host *host, struct mmc_card *card)
    {
    struct mxcmci_host *mxcmci = mmc_priv(host);
//
// MX3 SoCs have a silicon bug which corrupts CRC calculation of
// multi-block transfers when connected SDIO peripheral doesn't
// drive the BUSY line as required by the specs.
// One way to prevent this is to only allow 1-bit transfers.
//
    if (is_imx31_mmc(mxcmci) && mmc_card_sdio(card))
    host.caps &= ~MMC_CAP_4_BIT_DATA;
    else
    host.caps |= MMC_CAP_4_BIT_DATA;
    }
#[no_mangle]
unsafe extern "C" fn filter(chan: *mut dma_chan, param: *mut c_void) -> bool {
    static bool filter(struct dma_chan *chan, void *param)
    {
    struct mxcmci_host *host = param;
    if (!imx_dma_is_general_purpose(chan))
    return false;
    chan.private = &host.dma_data;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_watchdog(t: *mut timer_list) {
    static void mxcmci_watchdog(struct timer_list *t)
    {
    struct mxcmci_host *host = timer_container_of(host, t, watchdog);
    struct mmc_request *req = host.req;
    let mut stat: c_uint = mxcmci_readl(host, MMC_REG_STATUS);
    if (host.dma_dir == DMA_FROM_DEVICE) {
    dmaengine_terminate_all(host.dma);
    dev_err(mmc_dev(host.mmc),
    "%s: read time out (status = 0x%08x)\n",
    __func__, stat);
    } else {
    dev_err(mmc_dev(host.mmc),
    "%s: write time out (status = 0x%08x)\n",
    __func__, stat);
    mxcmci_softreset(host);
    }
// Mark transfer as erroneus and inform the upper layers
    if (host.data)
    host.data.error = -ETIMEDOUT;
    host.req = core::ptr::null_mut();
    host.cmd = core::ptr::null_mut();
    host.data = core::ptr::null_mut();
    mmc_request_done(host.mmc, req);
    }
    static const struct mmc_host_ops mxcmci_ops = {
    .request		= mxcmci_request,
    .set_ios		= mxcmci_set_ios,
    .get_ro			= mxcmci_get_ro,
    .enable_sdio_irq	= mxcmci_enable_sdio_irq,
    .init_card		= mxcmci_init_card,
    };
#[no_mangle]
unsafe extern "C" fn mxcmci_probe(pdev: *mut platform_device) -> c_int {
    static int mxcmci_probe(struct platform_device *pdev)
    {
    struct mmc_host *mmc;
    struct mxcmci_host *host;
    struct resource *res;
    let mut ret: c_int = 0, irq;
    bool dat3_card_detect;
    dma_cap_mask_t mask;
    struct imxmmc_platform_data *pdata = pdev.dev.platform_data;
    pr_info("i.MX/MPC512x SDHC driver\n");
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    mmc = devm_mmc_alloc_host(&pdev.dev, sizeof(*host));
    if (!mmc)
    return -ENOMEM;
    host = mmc_priv(mmc);
    host.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(host.base))
    return PTR_ERR(host.base);
    host.phys_base = res.start;
    ret = mmc_of_parse(mmc);
    if (ret)
    return ret;
    mmc.ops = &mxcmci_ops;
// For devicetree parsing, the bus width is read from devicetree
    if (pdata)
    mmc.caps = MMC_CAP_4_BIT_DATA | MMC_CAP_SDIO_IRQ;
    else
    mmc.caps |= MMC_CAP_SDIO_IRQ;
// MMC core transfer sizes tunable parameters
    mmc.max_blk_size = 2048;
    mmc.max_blk_count = 65535;
    mmc.max_req_size = mmc.max_blk_size * mmc.max_blk_count;
    mmc.max_seg_size = mmc.max_req_size;
    host.devtype = (uintptr_t)of_device_get_match_data(&pdev.dev);
// adjust max_segs after devtype detection
    if (!is_mpc512x_mmc(host))
    mmc.max_segs = 64;
    host.mmc = mmc;
    host.pdata = pdata;
    spin_lock_init(&host.lock);
    if (pdata)
    dat3_card_detect = pdata.dat3_card_detect;
    else
    dat3_card_detect = mmc_card_is_removable(mmc) &&
    !of_property_present(pdev.dev.of_node, "cd-gpios");
    ret = mmc_regulator_get_supply(mmc);
    if (ret)
    return ret;
    if (!mmc.ocr_avail) {
    if (pdata && pdata.ocr_avail)
    mmc.ocr_avail = pdata.ocr_avail;
    else
    mmc.ocr_avail = MMC_VDD_32_33 | MMC_VDD_33_34;
    }
    if (dat3_card_detect)
    host.default_irq_mask =
    INT_CARD_INSERTION_EN | INT_CARD_REMOVAL_EN;
    else
    host.default_irq_mask = 0;
    host.clk_ipg = devm_clk_get(&pdev.dev, "ipg");
    if (IS_ERR(host.clk_ipg))
    return PTR_ERR(host.clk_ipg);
    host.clk_per = devm_clk_get(&pdev.dev, "per");
    if (IS_ERR(host.clk_per))
    return PTR_ERR(host.clk_per);
    ret = clk_prepare_enable(host.clk_per);
    if (ret)
    return ret;
    ret = clk_prepare_enable(host.clk_ipg);
    if (ret)
    goto out_clk_per_put;
    mxcmci_softreset(host);
    host.rev_no = mxcmci_readw(host, MMC_REG_REV_NO);
    if (host.rev_no != 0x400) {
    ret = -ENODEV;
    dev_err(mmc_dev(host.mmc), "wrong rev.no. 0x%08x. aborting.\n",
    host.rev_no);
    goto out_clk_put;
    }
    mmc.f_min = clk_get_rate(host.clk_per) >> 16;
    mmc.f_max = clk_get_rate(host.clk_per) >> 1;
// recommended in data sheet
    mxcmci_writew(host, 0x2db4, MMC_REG_READ_TO);
    mxcmci_writel(host, host.default_irq_mask, MMC_REG_INT_CNTR);
    if (!host.pdata) {
    host.dma = dma_request_chan(&pdev.dev, "rx-tx");
    if (IS_ERR(host.dma)) {
    if (PTR_ERR(host.dma) == -EPROBE_DEFER) {
    ret = -EPROBE_DEFER;
    goto out_clk_put;
    }
// Ignore errors to fall back to PIO mode
    host.dma = core::ptr::null_mut();
    }
    } else {
    res = platform_get_resource(pdev, IORESOURCE_DMA, 0);
    if (res) {
    host.dmareq = res.start;
    host.dma_data.peripheral_type = IMX_DMATYPE_SDHC;
    host.dma_data.priority = DMA_PRIO_LOW;
    host.dma_data.dma_request = host.dmareq;
    dma_cap_zero(mask);
    dma_cap_set(DMA_SLAVE, mask);
    host.dma = dma_request_channel(mask, filter, host);
    }
    }
    if (host.dma)
    mmc.max_seg_size = dma_get_max_seg_size(
    host.dma.device.dev);
    else
    dev_info(mmc_dev(host.mmc), "dma not available. Using PIO\n");
    INIT_WORK(&host.datawork, mxcmci_datawork);
    ret = devm_request_irq(&pdev.dev, irq, mxcmci_irq, 0,
    dev_name(&pdev.dev), host);
    if (ret)
    goto out_free_dma;
    platform_set_drvdata(pdev, mmc);
    if (host.pdata && host.pdata.init) {
    ret = host.pdata.init(&pdev.dev, mxcmci_detect_irq,
    host.mmc);
    if (ret)
    goto out_free_dma;
    }
    timer_setup(&host.watchdog, mxcmci_watchdog, 0);
    ret = mmc_add_host(mmc);
    if (ret)
    goto out_free_dma;
    return 0;
    out_free_dma:
    if (host.dma)
    dma_release_channel(host.dma);
    out_clk_put:
    clk_disable_unprepare(host.clk_ipg);
    out_clk_per_put:
    clk_disable_unprepare(host.clk_per);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_remove(pdev: *mut platform_device) {
    static void mxcmci_remove(struct platform_device *pdev)
    {
    struct mmc_host *mmc = platform_get_drvdata(pdev);
    struct mxcmci_host *host = mmc_priv(mmc);
    mmc_remove_host(mmc);
    if (host.pdata && host.pdata.exit)
    host.pdata.exit(&pdev.dev, mmc);
    if (host.dma)
    dma_release_channel(host.dma);
    clk_disable_unprepare(host.clk_per);
    clk_disable_unprepare(host.clk_ipg);
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_suspend(dev: *mut device) -> c_int {
    static int mxcmci_suspend(struct device *dev)
    {
    struct mmc_host *mmc = dev_get_drvdata(dev);
    struct mxcmci_host *host = mmc_priv(mmc);
    clk_disable_unprepare(host.clk_per);
    clk_disable_unprepare(host.clk_ipg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxcmci_resume(dev: *mut device) -> c_int {
    static int mxcmci_resume(struct device *dev)
    {
    struct mmc_host *mmc = dev_get_drvdata(dev);
    struct mxcmci_host *host = mmc_priv(mmc);
    int ret;
    ret = clk_prepare_enable(host.clk_per);
    if (ret)
    return ret;
    ret = clk_prepare_enable(host.clk_ipg);
    if (ret)
    clk_disable_unprepare(host.clk_per);
    return ret;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(mxcmci_pm_ops, mxcmci_suspend, mxcmci_resume);
    static struct platform_driver mxcmci_driver = {
    .probe		= mxcmci_probe,
    .remove		= mxcmci_remove,
    .driver		= {
    .name		= DRIVER_NAME,
    .probe_type	= PROBE_PREFER_ASYNCHRONOUS,
    .pm	= pm_sleep_ptr(&mxcmci_pm_ops),
    .of_match_table	= mxcmci_of_match,
    }
    };
    module_platform_driver(mxcmci_driver);
    MODULE_DESCRIPTION("i.MX Multimedia Card Interface Driver");
    MODULE_AUTHOR("Sascha Hauer, Pengutronix");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:mxc-mmc");
