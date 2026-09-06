//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/wmt-sdmmc.c
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
// WM8505/WM8650 SD/MMC Host Controller
//
// Copyright (C) 2010 Tony Prisk
// Copyright (C) 2008 WonderMedia Technologies, Inc.
//

// MMC/SD controller registers
pub const SDMMC_CTLR: c_uint = 0x00;
pub const SDMMC_CMD: c_uint = 0x01;
pub const SDMMC_RSPTYPE: c_uint = 0x02;
pub const SDMMC_ARG: c_uint = 0x04;
pub const SDMMC_BUSMODE: c_uint = 0x08;
pub const SDMMC_BLKLEN: c_uint = 0x0C;
pub const SDMMC_BLKCNT: c_uint = 0x0E;
pub const SDMMC_RSP: c_uint = 0x10;
pub const SDMMC_CBCR: c_uint = 0x20;
pub const SDMMC_INTMASK0: c_uint = 0x24;
pub const SDMMC_INTMASK1: c_uint = 0x25;
pub const SDMMC_STS0: c_uint = 0x28;
pub const SDMMC_STS1: c_uint = 0x29;
pub const SDMMC_STS2: c_uint = 0x2A;
pub const SDMMC_STS3: c_uint = 0x2B;
pub const SDMMC_RSPTIMEOUT: c_uint = 0x2C;
pub const SDMMC_CLK: c_uint = 0x30	/* VT8500 only */;
pub const SDMMC_EXTCTRL: c_uint = 0x34;
pub const SDMMC_SBLKLEN: c_uint = 0x38;
pub const SDMMC_DMATIMEOUT: c_uint = 0x3C;
// SDMMC_CTLR bit fields
pub const CTLR_CMD_START: c_uint = 0x01;
pub const CTLR_CMD_WRITE: c_uint = 0x04;
pub const CTLR_FIFO_RESET: c_uint = 0x08;
// SDMMC_BUSMODE bit fields
pub const BM_SPI_MODE: c_uint = 0x01;
pub const BM_FOURBIT_MODE: c_uint = 0x02;
pub const BM_EIGHTBIT_MODE: c_uint = 0x04;
pub const BM_SD_OFF: c_uint = 0x10;
pub const BM_SPI_CS: c_uint = 0x20;
pub const BM_SD_POWER: c_uint = 0x40;
pub const BM_SOFT_RESET: c_uint = 0x80;
// SDMMC_BLKLEN bit fields
pub const BLKL_CRCERR_ABORT: c_uint = 0x0800;
pub const BLKL_CD_POL_HIGH: c_uint = 0x1000;
pub const BLKL_GPI_CD: c_uint = 0x2000;
pub const BLKL_DATA3_CD: c_uint = 0x4000;
pub const BLKL_INT_ENABLE: c_uint = 0x8000;
// SDMMC_INTMASK0 bit fields
pub const INT0_MBLK_TRAN_DONE_INT_EN: c_uint = 0x10;
pub const INT0_BLK_TRAN_DONE_INT_EN: c_uint = 0x20;
pub const INT0_CD_INT_EN: c_uint = 0x40;
pub const INT0_DI_INT_EN: c_uint = 0x80;
// SDMMC_INTMASK1 bit fields
pub const INT1_CMD_RES_TRAN_DONE_INT_EN: c_uint = 0x02;
pub const INT1_CMD_RES_TOUT_INT_EN: c_uint = 0x04;
pub const INT1_MBLK_AUTO_STOP_INT_EN: c_uint = 0x08;
pub const INT1_DATA_TOUT_INT_EN: c_uint = 0x10;
pub const INT1_RESCRC_ERR_INT_EN: c_uint = 0x20;
pub const INT1_RCRC_ERR_INT_EN: c_uint = 0x40;
pub const INT1_WCRC_ERR_INT_EN: c_uint = 0x80;
// SDMMC_STS0 bit fields
pub const STS0_WRITE_PROTECT: c_uint = 0x02;
pub const STS0_CD_DATA3: c_uint = 0x04;
pub const STS0_CD_GPI: c_uint = 0x08;
pub const STS0_MBLK_DONE: c_uint = 0x10;
pub const STS0_BLK_DONE: c_uint = 0x20;
pub const STS0_CARD_DETECT: c_uint = 0x40;
pub const STS0_DEVICE_INS: c_uint = 0x80;
// SDMMC_STS1 bit fields
pub const STS1_SDIO_INT: c_uint = 0x01;
pub const STS1_CMDRSP_DONE: c_uint = 0x02;
pub const STS1_RSP_TIMEOUT: c_uint = 0x04;
pub const STS1_AUTOSTOP_DONE: c_uint = 0x08;
pub const STS1_DATA_TIMEOUT: c_uint = 0x10;
pub const STS1_RSP_CRC_ERR: c_uint = 0x20;
pub const STS1_RCRC_ERR: c_uint = 0x40;
pub const STS1_WCRC_ERR: c_uint = 0x80;
// SDMMC_STS2 bit fields
pub const STS2_CMD_RES_BUSY: c_uint = 0x10;
pub const STS2_DATARSP_BUSY: c_uint = 0x20;
pub const STS2_DIS_FORCECLK: c_uint = 0x80;
// SDMMC_EXTCTRL bit fields
pub const EXT_EIGHTBIT: c_uint = 0x04;
// MMC/SD DMA Controller Registers
pub const SDDMA_GCR: c_uint = 0x100;
pub const SDDMA_IER: c_uint = 0x104;
pub const SDDMA_ISR: c_uint = 0x108;
pub const SDDMA_DESPR: c_uint = 0x10C;
pub const SDDMA_RBR: c_uint = 0x110;
pub const SDDMA_DAR: c_uint = 0x114;
pub const SDDMA_BAR: c_uint = 0x118;
pub const SDDMA_CPR: c_uint = 0x11C;
pub const SDDMA_CCR: c_uint = 0x120;
// SDDMA_GCR bit fields
pub const DMA_GCR_DMA_EN: c_uint = 0x00000001;
pub const DMA_GCR_SOFT_RESET: c_uint = 0x00000100;
// SDDMA_IER bit fields
pub const DMA_IER_INT_EN: c_uint = 0x00000001;
// SDDMA_ISR bit fields
pub const DMA_ISR_INT_STS: c_uint = 0x00000001;
// SDDMA_RBR bit fields
pub const DMA_RBR_FORMAT: c_uint = 0x40000000;
pub const DMA_RBR_END: c_uint = 0x80000000;
// SDDMA_CCR bit fields
pub const DMA_CCR_RUN: c_uint = 0x00000080;
pub const DMA_CCR_IF_TO_PERIPHERAL: c_uint = 0x00000000;
pub const DMA_CCR_PERIPHERAL_TO_IF: c_uint = 0x00400000;
// SDDMA_CCR event status
pub const DMA_CCR_EVT_NO_STATUS: c_uint = 0x00000000;
pub const DMA_CCR_EVT_UNDERRUN: c_uint = 0x00000001;
pub const DMA_CCR_EVT_OVERRUN: c_uint = 0x00000002;
pub const DMA_CCR_EVT_DESP_READ: c_uint = 0x00000003;
pub const DMA_CCR_EVT_DATA_RW: c_uint = 0x00000004;
pub const DMA_CCR_EVT_EARLY_END: c_uint = 0x00000005;
pub const DMA_CCR_EVT_SUCCESS: c_uint = 0x0000000F;
pub const PDMA_READ: c_uint = 0x00;
pub const PDMA_WRITE: c_uint = 0x01;
pub const WMT_SD_POWER_OFF: c_int = 0;
pub const WMT_SD_POWER_ON: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmt_dma_descriptor {
    pub flags: u32,
    pub data_buffer_addr: u32,
    pub branch_addr: u32,
    pub reserved1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmt_mci_caps {
    pub f_min: c_uint,
    pub f_max: c_uint,
    pub ocr_avail: u32,
    pub caps: u32,
    pub max_seg_size: u32,
    pub max_segs: u32,
    pub max_blk_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmt_mci_priv {
    pub mmc: *mut mmc_host,
    pub sdmmc_base: *mut void __iomem,
    pub irq_regular: c_int,
    pub irq_dma: c_int,
    pub dma_desc_buffer: *mut c_void,
    pub dma_desc_device_addr: dma_addr_t,
    pub cmdcomp: completion,
    pub datacomp: completion,
    pub comp_cmd: *mut completion,
    pub comp_dma: *mut completion,
    pub req: *mut mmc_request,
    pub cmd: *mut mmc_command,
    pub clk_sdmmc: *mut clk,
    pub dev: *mut device,
    pub power_inverted: u8,
    pub cd_inverted: u8,
}

#[no_mangle]
unsafe extern "C" fn wmt_set_sd_power(priv: *mut wmt_mci_priv, enable: c_int) {
    static void wmt_set_sd_power(struct wmt_mci_priv *priv, int enable)
    {
    let mut reg_tmp: u32 = readb(priv.sdmmc_base + SDMMC_BUSMODE);
    if (enable ^ priv.power_inverted)
    reg_tmp &= ~BM_SD_OFF;
    else
    reg_tmp |= BM_SD_OFF;
    writeb(reg_tmp, priv.sdmmc_base + SDMMC_BUSMODE);
    }
#[no_mangle]
unsafe extern "C" fn wmt_mci_read_response(mmc: *mut mmc_host) {
    static void wmt_mci_read_response(struct mmc_host *mmc)
    {
    struct wmt_mci_priv *priv;
    int idx1, idx2;
    u8 tmp_resp;
    u32 response;
    priv = mmc_priv(mmc);
    for (idx1 = 0; idx1 < 4; idx1++) {
    response = 0;
    for (idx2 = 0; idx2 < 4; idx2++) {
    if ((idx1 == 3) && (idx2 == 3))
    tmp_resp = readb(priv.sdmmc_base + SDMMC_RSP);
    else
    tmp_resp = readb(priv.sdmmc_base + SDMMC_RSP +
    (idx1*4) + idx2 + 1);
    response |= (tmp_resp << (idx2 * 8));
    }
    priv.cmd.resp[idx1] = cpu_to_be32(response);
    }
    }
#[no_mangle]
unsafe extern "C" fn wmt_mci_start_command(priv: *mut wmt_mci_priv) {
    static void wmt_mci_start_command(struct wmt_mci_priv *priv)
    {
    u32 reg_tmp;
    reg_tmp = readb(priv.sdmmc_base + SDMMC_CTLR);
    writeb(reg_tmp | CTLR_CMD_START, priv.sdmmc_base + SDMMC_CTLR);
    }
    static int wmt_mci_send_command(struct mmc_host *mmc, u8 command, u8 cmdtype,
    u32 arg, u8 rsptype)
    {
    struct wmt_mci_priv *priv;
    u32 reg_tmp;
    priv = mmc_priv(mmc);
// write command, arg, resptype registers
    writeb(command, priv.sdmmc_base + SDMMC_CMD);
    writel(arg, priv.sdmmc_base + SDMMC_ARG);
    writeb(rsptype, priv.sdmmc_base + SDMMC_RSPTYPE);
// reset response FIFO
    reg_tmp = readb(priv.sdmmc_base + SDMMC_CTLR);
    writeb(reg_tmp | CTLR_FIFO_RESET, priv.sdmmc_base + SDMMC_CTLR);
// ensure clock enabled - VT3465
    wmt_set_sd_power(priv, WMT_SD_POWER_ON);
// clear status bits
    writeb(0xFF, priv.sdmmc_base + SDMMC_STS0);
    writeb(0xFF, priv.sdmmc_base + SDMMC_STS1);
    writeb(0xFF, priv.sdmmc_base + SDMMC_STS2);
    writeb(0xFF, priv.sdmmc_base + SDMMC_STS3);
// set command type
    reg_tmp = readb(priv.sdmmc_base + SDMMC_CTLR);
    writeb((reg_tmp & 0x0F) | (cmdtype << 4),
    priv.sdmmc_base + SDMMC_CTLR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wmt_mci_disable_dma(priv: *mut wmt_mci_priv) {
    static void wmt_mci_disable_dma(struct wmt_mci_priv *priv)
    {
    writel(DMA_ISR_INT_STS, priv.sdmmc_base + SDDMA_ISR);
    writel(0, priv.sdmmc_base + SDDMA_IER);
    }
#[no_mangle]
unsafe extern "C" fn wmt_complete_data_request(priv: *mut wmt_mci_priv) {
    static void wmt_complete_data_request(struct wmt_mci_priv *priv)
    {
    struct mmc_request *req;
    req = priv.req;
    req.data.bytes_xfered = req.data.blksz * req.data.blocks;
// unmap the DMA pages used for write data
    if (req.data.flags & MMC_DATA_WRITE)
    dma_unmap_sg(mmc_dev(priv.mmc), req.data.sg,
    req.data.sg_len, DMA_TO_DEVICE);
    else
    dma_unmap_sg(mmc_dev(priv.mmc), req.data.sg,
    req.data.sg_len, DMA_FROM_DEVICE);
// Check if the DMA ISR returned a data error
    if ((req.cmd.error) || (req.data.error))
    mmc_request_done(priv.mmc, req);
    else {
    wmt_mci_read_response(priv.mmc);
    if (!req.data.stop) {
// single-block read/write requests end here
    mmc_request_done(priv.mmc, req);
    } else {
//
// we change the priv->cmd variable so the response is
// stored in the stop struct rather than the original
// calling command struct
//
    priv.comp_cmd = &priv.cmdcomp;
    init_completion(priv.comp_cmd);
    priv.cmd = req.data.stop;
    wmt_mci_send_command(priv.mmc, req.data.stop.opcode,
    7, req.data.stop.arg, 9);
    wmt_mci_start_command(priv);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn wmt_mci_dma_isr(irq_num: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t wmt_mci_dma_isr(int irq_num, void *data)
    {
    struct wmt_mci_priv *priv;
    int status;
    priv = (struct wmt_mci_priv *)data;
    status = readl(priv.sdmmc_base + SDDMA_CCR) & 0x0F;
    if (status != DMA_CCR_EVT_SUCCESS) {
    dev_err(priv.dev, "DMA Error: Status = %d\n", status);
    priv.req.data.error = -ETIMEDOUT;
    complete(priv.comp_dma);
    return IRQ_HANDLED;
    }
    priv.req.data.error = 0;
    wmt_mci_disable_dma(priv);
    complete(priv.comp_dma);
    if (priv.comp_cmd) {
    if (completion_done(priv.comp_cmd)) {
//
// if the command (regular) interrupt has already
// completed, finish off the request otherwise we wait
// for the command interrupt and finish from there.
//
    wmt_complete_data_request(priv);
    }
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn wmt_mci_regular_isr(irq_num: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t wmt_mci_regular_isr(int irq_num, void *data)
    {
    struct wmt_mci_priv *priv;
    u32 status0;
    u32 status1;
    u32 status2;
    u32 reg_tmp;
    int cmd_done;
    priv = (struct wmt_mci_priv *)data;
    cmd_done = 0;
    status0 = readb(priv.sdmmc_base + SDMMC_STS0);
    status1 = readb(priv.sdmmc_base + SDMMC_STS1);
    status2 = readb(priv.sdmmc_base + SDMMC_STS2);
// Check for card insertion
    reg_tmp = readb(priv.sdmmc_base + SDMMC_INTMASK0);
    if ((reg_tmp & INT0_DI_INT_EN) && (status0 & STS0_DEVICE_INS)) {
    mmc_detect_change(priv.mmc, 0);
    if (priv.cmd)
    priv.cmd.error = -ETIMEDOUT;
    if (priv.comp_cmd)
    complete(priv.comp_cmd);
    if (priv.comp_dma) {
    wmt_mci_disable_dma(priv);
    complete(priv.comp_dma);
    }
    writeb(STS0_DEVICE_INS, priv.sdmmc_base + SDMMC_STS0);
    return IRQ_HANDLED;
    }
    if ((!priv.req.data) ||
    ((priv.req.data.stop) && (priv.cmd == priv.req.data.stop))) {
// handle non-data & stop_transmission requests
    if (status1 & STS1_CMDRSP_DONE) {
    priv.cmd.error = 0;
    cmd_done = 1;
    } else if ((status1 & STS1_RSP_TIMEOUT) ||
    (status1 & STS1_DATA_TIMEOUT)) {
    priv.cmd.error = -ETIMEDOUT;
    cmd_done = 1;
    }
    if (cmd_done) {
    priv.comp_cmd = core::ptr::null_mut();
    if (!priv.cmd.error)
    wmt_mci_read_response(priv.mmc);
    priv.cmd = core::ptr::null_mut();
    mmc_request_done(priv.mmc, priv.req);
    }
    } else {
// handle data requests
    if (status1 & STS1_CMDRSP_DONE) {
    if (priv.cmd)
    priv.cmd.error = 0;
    if (priv.comp_cmd)
    complete(priv.comp_cmd);
    }
    if ((status1 & STS1_RSP_TIMEOUT) ||
    (status1 & STS1_DATA_TIMEOUT)) {
    if (priv.cmd)
    priv.cmd.error = -ETIMEDOUT;
    if (priv.comp_cmd)
    complete(priv.comp_cmd);
    if (priv.comp_dma) {
    wmt_mci_disable_dma(priv);
    complete(priv.comp_dma);
    }
    }
    if (priv.comp_dma) {
//
// If the dma interrupt has already completed, finish
// off the request; otherwise we wait for the DMA
// interrupt and finish from there.
//
    if (completion_done(priv.comp_dma))
    wmt_complete_data_request(priv);
    }
    }
    writeb(status0, priv.sdmmc_base + SDMMC_STS0);
    writeb(status1, priv.sdmmc_base + SDMMC_STS1);
    writeb(status2, priv.sdmmc_base + SDMMC_STS2);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn wmt_reset_hardware(mmc: *mut mmc_host) {
    static void wmt_reset_hardware(struct mmc_host *mmc)
    {
    struct wmt_mci_priv *priv;
    u32 reg_tmp;
    priv = mmc_priv(mmc);
// reset controller
    reg_tmp = readb(priv.sdmmc_base + SDMMC_BUSMODE);
    writeb(reg_tmp | BM_SOFT_RESET, priv.sdmmc_base + SDMMC_BUSMODE);
// reset response FIFO
    reg_tmp = readb(priv.sdmmc_base + SDMMC_CTLR);
    writeb(reg_tmp | CTLR_FIFO_RESET, priv.sdmmc_base + SDMMC_CTLR);
// enable GPI pin to detect card
    writew(BLKL_INT_ENABLE | BLKL_GPI_CD, priv.sdmmc_base + SDMMC_BLKLEN);
// clear interrupt status
    writeb(0xFF, priv.sdmmc_base + SDMMC_STS0);
    writeb(0xFF, priv.sdmmc_base + SDMMC_STS1);
// setup interrupts
    writeb(INT0_CD_INT_EN | INT0_DI_INT_EN, priv.sdmmc_base +
    SDMMC_INTMASK0);
    writeb(INT1_DATA_TOUT_INT_EN | INT1_CMD_RES_TRAN_DONE_INT_EN |
    INT1_CMD_RES_TOUT_INT_EN, priv.sdmmc_base + SDMMC_INTMASK1);
// set the DMA timeout
    writew(8191, priv.sdmmc_base + SDMMC_DMATIMEOUT);
// auto clock freezing enable
    reg_tmp = readb(priv.sdmmc_base + SDMMC_STS2);
    writeb(reg_tmp | STS2_DIS_FORCECLK, priv.sdmmc_base + SDMMC_STS2);
// set a default clock speed of 400Khz
    clk_set_rate(priv.clk_sdmmc, 400000);
    }
#[no_mangle]
unsafe extern "C" fn wmt_dma_init(mmc: *mut mmc_host) -> c_int {
    static int wmt_dma_init(struct mmc_host *mmc)
    {
    struct wmt_mci_priv *priv;
    priv = mmc_priv(mmc);
    writel(DMA_GCR_SOFT_RESET, priv.sdmmc_base + SDDMA_GCR);
    writel(DMA_GCR_DMA_EN, priv.sdmmc_base + SDDMA_GCR);
    if ((readl(priv.sdmmc_base + SDDMA_GCR) & DMA_GCR_DMA_EN) != 0)
    return 0;
    else
    return 1;
    }
    static void wmt_dma_init_descriptor(struct wmt_dma_descriptor *desc,
    u16 req_count, u32 buffer_addr, u32 branch_addr, int end)
    {
    desc.flags = 0x40000000 | req_count;
    if (end)
    desc.flags |= 0x80000000;
    desc.data_buffer_addr = buffer_addr;
    desc.branch_addr = branch_addr;
    }
#[no_mangle]
unsafe extern "C" fn wmt_dma_config(mmc: *mut mmc_host, descaddr: u32, dir: u8) {
    static void wmt_dma_config(struct mmc_host *mmc, u32 descaddr, u8 dir)
    {
    struct wmt_mci_priv *priv;
    u32 reg_tmp;
    priv = mmc_priv(mmc);
// Enable DMA Interrupts
    writel(DMA_IER_INT_EN, priv.sdmmc_base + SDDMA_IER);
// Write DMA Descriptor Pointer Register
    writel(descaddr, priv.sdmmc_base + SDDMA_DESPR);
    writel(0x00, priv.sdmmc_base + SDDMA_CCR);
    if (dir == PDMA_WRITE) {
    reg_tmp = readl(priv.sdmmc_base + SDDMA_CCR);
    writel(reg_tmp & DMA_CCR_IF_TO_PERIPHERAL, priv.sdmmc_base +
    SDDMA_CCR);
    } else {
    reg_tmp = readl(priv.sdmmc_base + SDDMA_CCR);
    writel(reg_tmp | DMA_CCR_PERIPHERAL_TO_IF, priv.sdmmc_base +
    SDDMA_CCR);
    }
    }
#[no_mangle]
unsafe extern "C" fn wmt_dma_start(priv: *mut wmt_mci_priv) {
    static void wmt_dma_start(struct wmt_mci_priv *priv)
    {
    u32 reg_tmp;
    reg_tmp = readl(priv.sdmmc_base + SDDMA_CCR);
    writel(reg_tmp | DMA_CCR_RUN, priv.sdmmc_base + SDDMA_CCR);
    }
#[no_mangle]
unsafe extern "C" fn wmt_mci_request(mmc: *mut mmc_host, req: *mut mmc_request) {
    static void wmt_mci_request(struct mmc_host *mmc, struct mmc_request *req)
    {
    struct wmt_mci_priv *priv;
    struct wmt_dma_descriptor *desc;
    u8 command;
    u8 cmdtype;
    u32 arg;
    u8 rsptype;
    u32 reg_tmp;
    struct scatterlist *sg;
    int i;
    int sg_cnt;
    int offset;
    u32 dma_address;
    int desc_cnt;
    priv = mmc_priv(mmc);
    priv.req = req;
//
// Use the cmd variable to pass a pointer to the resp[] structure
// This is required on multi-block requests to pass the pointer to the
// stop command
//
    priv.cmd = req.cmd;
    command = req.cmd.opcode;
    arg = req.cmd.arg;
    rsptype = mmc_resp_type(req.cmd);
    cmdtype = 0;
// rsptype=7 only valid for SPI commands - should be =2 for SD
    if (rsptype == 7)
    rsptype = 2;
// rsptype=21 is R1B, convert for controller
    if (rsptype == 21)
    rsptype = 9;
    if (!req.data) {
    wmt_mci_send_command(mmc, command, cmdtype, arg, rsptype);
    wmt_mci_start_command(priv);
// completion is now handled in the regular_isr()
    }
    if (req.data) {
    priv.comp_cmd = &priv.cmdcomp;
    init_completion(priv.comp_cmd);
    wmt_dma_init(mmc);
// set controller data length
    reg_tmp = readw(priv.sdmmc_base + SDMMC_BLKLEN);
    writew((reg_tmp & 0xF800) | (req.data.blksz - 1),
    priv.sdmmc_base + SDMMC_BLKLEN);
// set controller block count
    writew(req.data.blocks, priv.sdmmc_base + SDMMC_BLKCNT);
    desc = (struct wmt_dma_descriptor *)priv.dma_desc_buffer;
    if (req.data.flags & MMC_DATA_WRITE) {
    sg_cnt = dma_map_sg(mmc_dev(mmc), req.data.sg,
    req.data.sg_len, DMA_TO_DEVICE);
    cmdtype = 1;
    if (req.data.blocks > 1)
    cmdtype = 3;
    } else {
    sg_cnt = dma_map_sg(mmc_dev(mmc), req.data.sg,
    req.data.sg_len, DMA_FROM_DEVICE);
    cmdtype = 2;
    if (req.data.blocks > 1)
    cmdtype = 4;
    }
    dma_address = priv.dma_desc_device_addr + 16;
    desc_cnt = 0;
    for_each_sg(req.data.sg, sg, sg_cnt, i) {
    offset = 0;
    while (offset < sg_dma_len(sg)) {
    wmt_dma_init_descriptor(desc, req.data.blksz,
    sg_dma_address(sg)+offset,
    dma_address, 0);
    desc++;
    desc_cnt++;
    offset += req.data.blksz;
    dma_address += 16;
    if (desc_cnt == req.data.blocks)
    break;
    }
    }
    desc--;
    desc.flags |= 0x80000000;
    if (req.data.flags & MMC_DATA_WRITE)
    wmt_dma_config(mmc, priv.dma_desc_device_addr,
    PDMA_WRITE);
    else
    wmt_dma_config(mmc, priv.dma_desc_device_addr,
    PDMA_READ);
    wmt_mci_send_command(mmc, command, cmdtype, arg, rsptype);
    priv.comp_dma = &priv.datacomp;
    init_completion(priv.comp_dma);
    wmt_dma_start(priv);
    wmt_mci_start_command(priv);
    }
    }
#[no_mangle]
unsafe extern "C" fn wmt_mci_set_ios(mmc: *mut mmc_host, ios: *mut mmc_ios) {
    static void wmt_mci_set_ios(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    struct wmt_mci_priv *priv;
    u32 busmode, extctrl;
    priv = mmc_priv(mmc);
    if (ios.power_mode == MMC_POWER_UP) {
    wmt_reset_hardware(mmc);
    wmt_set_sd_power(priv, WMT_SD_POWER_ON);
    }
    if (ios.power_mode == MMC_POWER_OFF)
    wmt_set_sd_power(priv, WMT_SD_POWER_OFF);
    if (ios.clock != 0)
    clk_set_rate(priv.clk_sdmmc, ios.clock);
    busmode = readb(priv.sdmmc_base + SDMMC_BUSMODE);
    extctrl = readb(priv.sdmmc_base + SDMMC_EXTCTRL);
    busmode &= ~(BM_EIGHTBIT_MODE | BM_FOURBIT_MODE);
    extctrl &= ~EXT_EIGHTBIT;
    switch (ios.bus_width) {
    case MMC_BUS_WIDTH_8:
    busmode |= BM_EIGHTBIT_MODE;
    extctrl |= EXT_EIGHTBIT;
    break;
    case MMC_BUS_WIDTH_4:
    busmode |= BM_FOURBIT_MODE;
    break;
    case MMC_BUS_WIDTH_1:
    break;
    }
    writeb(busmode, priv.sdmmc_base + SDMMC_BUSMODE);
    writeb(extctrl, priv.sdmmc_base + SDMMC_EXTCTRL);
    }
#[no_mangle]
unsafe extern "C" fn wmt_mci_get_ro(mmc: *mut mmc_host) -> c_int {
    static int wmt_mci_get_ro(struct mmc_host *mmc)
    {
    struct wmt_mci_priv *priv = mmc_priv(mmc);
    return !(readb(priv.sdmmc_base + SDMMC_STS0) & STS0_WRITE_PROTECT);
    }
#[no_mangle]
unsafe extern "C" fn wmt_mci_get_cd(mmc: *mut mmc_host) -> c_int {
    static int wmt_mci_get_cd(struct mmc_host *mmc)
    {
    struct wmt_mci_priv *priv = mmc_priv(mmc);
    let mut cd: u32 = (readb(priv.sdmmc_base + SDMMC_STS0) & STS0_CD_GPI) >> 3;
    return !(cd ^ priv.cd_inverted);
    }
    static const struct mmc_host_ops wmt_mci_ops = {
    .request = wmt_mci_request,
    .set_ios = wmt_mci_set_ios,
    .get_ro = wmt_mci_get_ro,
    .get_cd = wmt_mci_get_cd,
    };
// Controller capabilities
    static struct wmt_mci_caps wm8505_caps = {
    .f_min = 390425,
    .f_max = 50000000,
    .ocr_avail = MMC_VDD_32_33 | MMC_VDD_33_34,
    .caps = MMC_CAP_4_BIT_DATA | MMC_CAP_MMC_HIGHSPEED |
    MMC_CAP_SD_HIGHSPEED,
    .max_seg_size = 65024,
    .max_segs = 128,
    .max_blk_size = 2048,
    };
    static const struct of_device_id wmt_mci_dt_ids[] = {
    { .compatible = "wm,wm8505-sdhc", .data = &wm8505_caps },
    { /* Sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, wmt_mci_dt_ids);
#[no_mangle]
unsafe extern "C" fn wmt_mci_probe(pdev: *mut platform_device) -> c_int {
    static int wmt_mci_probe(struct platform_device *pdev)
    {
    struct mmc_host *mmc;
    struct wmt_mci_priv *priv;
    struct device_node *np = pdev.dev.of_node;
    const struct wmt_mci_caps *wmt_caps;
    int ret;
    int regular_irq, dma_irq;
    wmt_caps = of_device_get_match_data(&pdev.dev);
    if (!wmt_caps) {
    dev_err(&pdev.dev, "Controller capabilities data missing\n");
    return -EFAULT;
    }
    if (!np) {
    dev_err(&pdev.dev, "Missing SDMMC description in devicetree\n");
    return -EFAULT;
    }
    regular_irq = irq_of_parse_and_map(np, 0);
    dma_irq = irq_of_parse_and_map(np, 1);
    if (!regular_irq || !dma_irq) {
    dev_err(&pdev.dev, "Getting IRQs failed!\n");
    ret = -ENXIO;
    goto fail1;
    }
    mmc = devm_mmc_alloc_host(&pdev.dev, sizeof(*priv));
    if (!mmc) {
    dev_err(&pdev.dev, "Failed to allocate mmc_host\n");
    ret = -ENOMEM;
    goto fail1;
    }
    mmc.ops = &wmt_mci_ops;
    mmc.f_min = wmt_caps.f_min;
    mmc.f_max = wmt_caps.f_max;
    mmc.ocr_avail = wmt_caps.ocr_avail;
    mmc.caps = wmt_caps.caps;
    mmc.max_seg_size = wmt_caps.max_seg_size;
    mmc.max_segs = wmt_caps.max_segs;
    mmc.max_blk_size = wmt_caps.max_blk_size;
    mmc.max_req_size = (16*512*mmc.max_segs);
    mmc.max_blk_count = mmc.max_req_size / 512;
    priv = mmc_priv(mmc);
    priv.mmc = mmc;
    priv.dev = &pdev.dev;
    priv.power_inverted = 0;
    priv.cd_inverted = 0;
    priv.power_inverted = of_property_read_bool(np, "sdon-inverted");
    priv.cd_inverted = of_property_read_bool(np, "cd-inverted");
    priv.sdmmc_base = of_iomap(np, 0);
    if (!priv.sdmmc_base) {
    dev_err(&pdev.dev, "Failed to map IO space\n");
    ret = -ENOMEM;
    goto fail1;
    }
    priv.irq_regular = regular_irq;
    priv.irq_dma = dma_irq;
    ret = request_irq(regular_irq, wmt_mci_regular_isr, 0, "sdmmc", priv);
    if (ret) {
    dev_err(&pdev.dev, "Register regular IRQ fail\n");
    goto fail3;
    }
    ret = request_irq(dma_irq, wmt_mci_dma_isr, 0, "sdmmc", priv);
    if (ret) {
    dev_err(&pdev.dev, "Register DMA IRQ fail\n");
    goto fail4;
    }
// alloc some DMA buffers for descriptors/transfers
    priv.dma_desc_buffer = dma_alloc_coherent(&pdev.dev,
    mmc.max_blk_count * 16,
    &priv.dma_desc_device_addr,
    GFP_KERNEL);
    if (!priv.dma_desc_buffer) {
    dev_err(&pdev.dev, "DMA alloc fail\n");
    ret = -EPERM;
    goto fail5;
    }
    platform_set_drvdata(pdev, mmc);
    priv.clk_sdmmc = of_clk_get(np, 0);
    if (IS_ERR(priv.clk_sdmmc)) {
    dev_err(&pdev.dev, "Error getting clock\n");
    ret = PTR_ERR(priv.clk_sdmmc);
    goto fail5_and_a_half;
    }
    ret = clk_prepare_enable(priv.clk_sdmmc);
    if (ret)
    goto fail6;
// configure the controller to a known 'ready' state
    wmt_reset_hardware(mmc);
    ret = mmc_add_host(mmc);
    if (ret)
    goto fail7;
    dev_info(&pdev.dev, "WMT SDHC Controller initialized\n");
    return 0;
    fail7:
    clk_disable_unprepare(priv.clk_sdmmc);
    fail6:
    clk_put(priv.clk_sdmmc);
    fail5_and_a_half:
    dma_free_coherent(&pdev.dev, mmc.max_blk_count * 16,
    priv.dma_desc_buffer, priv.dma_desc_device_addr);
    fail5:
    free_irq(dma_irq, priv);
    fail4:
    free_irq(regular_irq, priv);
    fail3:
    iounmap(priv.sdmmc_base);
    fail1:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn wmt_mci_remove(pdev: *mut platform_device) {
    static void wmt_mci_remove(struct platform_device *pdev)
    {
    struct mmc_host *mmc;
    struct wmt_mci_priv *priv;
    u32 reg_tmp;
    mmc = platform_get_drvdata(pdev);
    priv = mmc_priv(mmc);
// reset SD controller
    reg_tmp = readb(priv.sdmmc_base + SDMMC_BUSMODE);
    writel(reg_tmp | BM_SOFT_RESET, priv.sdmmc_base + SDMMC_BUSMODE);
    reg_tmp = readw(priv.sdmmc_base + SDMMC_BLKLEN);
    writew(reg_tmp & ~(0xA000), priv.sdmmc_base + SDMMC_BLKLEN);
    writeb(0xFF, priv.sdmmc_base + SDMMC_STS0);
    writeb(0xFF, priv.sdmmc_base + SDMMC_STS1);
// release the dma buffers
    dma_free_coherent(&pdev.dev, priv.mmc.max_blk_count * 16,
    priv.dma_desc_buffer, priv.dma_desc_device_addr);
    mmc_remove_host(mmc);
    free_irq(priv.irq_regular, priv);
    free_irq(priv.irq_dma, priv);
    iounmap(priv.sdmmc_base);
    clk_disable_unprepare(priv.clk_sdmmc);
    clk_put(priv.clk_sdmmc);
    dev_info(&pdev.dev, "WMT MCI device removed\n");
    }
#[no_mangle]
unsafe extern "C" fn wmt_mci_suspend(dev: *mut device) -> c_int {
    static int wmt_mci_suspend(struct device *dev)
    {
    u32 reg_tmp;
    struct mmc_host *mmc = dev_get_drvdata(dev);
    struct wmt_mci_priv *priv;
    if (!mmc)
    return 0;
    priv = mmc_priv(mmc);
    reg_tmp = readb(priv.sdmmc_base + SDMMC_BUSMODE);
    writeb(reg_tmp | BM_SOFT_RESET, priv.sdmmc_base +
    SDMMC_BUSMODE);
    reg_tmp = readw(priv.sdmmc_base + SDMMC_BLKLEN);
    writew(reg_tmp & 0x5FFF, priv.sdmmc_base + SDMMC_BLKLEN);
    writeb(0xFF, priv.sdmmc_base + SDMMC_STS0);
    writeb(0xFF, priv.sdmmc_base + SDMMC_STS1);
    clk_disable(priv.clk_sdmmc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wmt_mci_resume(dev: *mut device) -> c_int {
    static int wmt_mci_resume(struct device *dev)
    {
    u32 reg_tmp;
    struct mmc_host *mmc = dev_get_drvdata(dev);
    struct wmt_mci_priv *priv;
    if (mmc) {
    priv = mmc_priv(mmc);
    clk_enable(priv.clk_sdmmc);
    reg_tmp = readb(priv.sdmmc_base + SDMMC_BUSMODE);
    writeb(reg_tmp | BM_SOFT_RESET, priv.sdmmc_base +
    SDMMC_BUSMODE);
    reg_tmp = readw(priv.sdmmc_base + SDMMC_BLKLEN);
    writew(reg_tmp | (BLKL_GPI_CD | BLKL_INT_ENABLE),
    priv.sdmmc_base + SDMMC_BLKLEN);
    reg_tmp = readb(priv.sdmmc_base + SDMMC_INTMASK0);
    writeb(reg_tmp | INT0_DI_INT_EN, priv.sdmmc_base +
    SDMMC_INTMASK0);
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(wmt_mci_pm_ops, wmt_mci_suspend, wmt_mci_resume);
    static struct platform_driver wmt_mci_driver = {
    .probe = wmt_mci_probe,
    .remove = wmt_mci_remove,
    .driver = {
    .name = DRIVER_NAME,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .pm = pm_sleep_ptr(&wmt_mci_pm_ops),
    .of_match_table = wmt_mci_dt_ids,
    },
    };
    module_platform_driver(wmt_mci_driver);
    MODULE_DESCRIPTION("Wondermedia MMC/SD Driver");
    MODULE_AUTHOR("Tony Prisk");
    MODULE_LICENSE("GPL v2");
