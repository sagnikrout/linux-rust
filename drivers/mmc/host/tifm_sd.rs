//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/tifm_sd.c
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
// tifm_sd.c - TI FlashMedia driver
//
// Copyright (C) 2006 Alex Dubov <oakad@yahoo.com>
//
// Special thanks to Brad Campbell for extensive testing of this driver.
//

    let mut no_dma: static bool = 0;
    let mut fixed_timeout: static bool = 0;
    module_param(no_dma, bool, 0644);
    module_param(fixed_timeout, bool, 0644);
// Constants here are mostly from OMAP5912 datasheet
pub const TIFM_MMCSD_RESET: c_uint = 0x0002;
pub const TIFM_MMCSD_CLKMASK: c_uint = 0x03ff;
pub const TIFM_MMCSD_POWER: c_uint = 0x0800;
pub const TIFM_MMCSD_4BBUS: c_uint = 0x8000;
pub const TIFM_MMCSD_RXDE: c_uint = 0x8000   /* rx dma enable */;
pub const TIFM_MMCSD_TXDE: c_uint = 0x0080   /* tx dma enable */;
pub const TIFM_MMCSD_BUFINT: c_uint = 0x0c00   /* set bits: AE, AF */;
pub const TIFM_MMCSD_DPE: c_uint = 0x0020   /* data timeout counted in kilocycles */;
pub const TIFM_MMCSD_INAB: c_uint = 0x0080   /* abort / initialize command */;
pub const TIFM_MMCSD_READ: c_uint = 0x8000;
pub const TIFM_MMCSD_ERRMASK: c_uint = 0x01e0   /* set bits: CCRC, CTO, DCRC, DTO */;
pub const TIFM_MMCSD_EOC: c_uint = 0x0001   /* end of command phase  */;
pub const TIFM_MMCSD_CD: c_uint = 0x0002   /* card detect           */;
pub const TIFM_MMCSD_CB: c_uint = 0x0004   /* card enter busy state */;
pub const TIFM_MMCSD_BRS: c_uint = 0x0008   /* block received/sent   */;
pub const TIFM_MMCSD_EOFB: c_uint = 0x0010   /* card exit busy state  */;
pub const TIFM_MMCSD_DTO: c_uint = 0x0020   /* data time-out         */;
pub const TIFM_MMCSD_DCRC: c_uint = 0x0040   /* data crc error        */;
pub const TIFM_MMCSD_CTO: c_uint = 0x0080   /* command time-out      */;
pub const TIFM_MMCSD_CCRC: c_uint = 0x0100   /* command crc error     */;
pub const TIFM_MMCSD_AF: c_uint = 0x0400   /* fifo almost full      */;
pub const TIFM_MMCSD_AE: c_uint = 0x0800   /* fifo almost empty     */;
pub const TIFM_MMCSD_OCRB: c_uint = 0x1000   /* OCR busy              */;
pub const TIFM_MMCSD_CIRQ: c_uint = 0x2000   /* card irq (cmd40/sdio) */;
pub const TIFM_MMCSD_CERR: c_uint = 0x4000   /* card status error     */;
pub const TIFM_MMCSD_ODTO: c_uint = 0x0040   /* open drain / extended timeout */;
pub const TIFM_MMCSD_CARD_RO: c_uint = 0x0200   /* card is read-only     */;
pub const TIFM_MMCSD_FIFO_SIZE: c_uint = 0x0020;
pub const TIFM_MMCSD_RSP_R0: c_uint = 0x0000;
pub const TIFM_MMCSD_RSP_R1: c_uint = 0x0100;
pub const TIFM_MMCSD_RSP_R2: c_uint = 0x0200;
pub const TIFM_MMCSD_RSP_R3: c_uint = 0x0300;
pub const TIFM_MMCSD_RSP_R4: c_uint = 0x0400;
pub const TIFM_MMCSD_RSP_R5: c_uint = 0x0500;
pub const TIFM_MMCSD_RSP_R6: c_uint = 0x0600;
pub const TIFM_MMCSD_RSP_BUSY: c_uint = 0x0800;
pub const TIFM_MMCSD_CMD_BC: c_uint = 0x0000;
pub const TIFM_MMCSD_CMD_BCR: c_uint = 0x1000;
pub const TIFM_MMCSD_CMD_AC: c_uint = 0x2000;
pub const TIFM_MMCSD_CMD_ADTC: c_uint = 0x3000;
pub const TIFM_MMCSD_MAX_BLOCK_SIZE: c_uint = 0x0800UL;
pub const TIFM_MMCSD_REQ_TIMEOUT_MS: c_int = 1000;
    enum {
    CMD_READY    = 0x0001,
    FIFO_READY   = 0x0002,
    BRS_READY    = 0x0004,
    SCMD_ACTIVE  = 0x0008,
    SCMD_READY   = 0x0010,
    CARD_BUSY    = 0x0020,
    DATA_CARRY   = 0x0040
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tifm_sd {
    pub dev: *mut tifm_dev,
    unsigned short        eject:1,
    open_drain:1,
    pub cmd_flags: c_ushort,
    pub clk_freq: c_uint,
    pub clk_div: c_uint,
    pub timeout_jiffies: c_ulong,
    pub finish_bh_work: work_struct,
    pub timer: timer_list,
    pub req: *mut mmc_request,
    pub sg_len: c_int,
    pub sg_pos: c_int,
    pub block_pos: c_uint,
    pub bounce_buf: scatterlist,
    pub bounce_buf_data: [c_uchar; TIFM_MMCSD_MAX_BLOCK_SIZE],
}

// for some reason, host won't respond correctly to readw/writew
    static void tifm_sd_read_fifo(struct tifm_sd *host, struct page *pg,
    unsigned int off, unsigned int cnt)
    {
    struct tifm_dev *sock = host.dev;
    unsigned char *buf;
    let mut pos: c_uint = 0, val;
    buf = kmap_local_page(pg) + off;
    if (host.cmd_flags & DATA_CARRY) {
    buf[pos++] = host.bounce_buf_data[0];
    host.cmd_flags &= ~DATA_CARRY;
    }
    while (pos < cnt) {
    val = readl(sock.addr + SOCK_MMCSD_DATA);
    buf[pos++] = val & 0xff;
    if (pos == cnt) {
    host.bounce_buf_data[0] = (val >> 8) & 0xff;
    host.cmd_flags |= DATA_CARRY;
    break;
    }
    buf[pos++] = (val >> 8) & 0xff;
    }
    kunmap_local(buf - off);
    }
    static void tifm_sd_write_fifo(struct tifm_sd *host, struct page *pg,
    unsigned int off, unsigned int cnt)
    {
    struct tifm_dev *sock = host.dev;
    unsigned char *buf;
    let mut pos: c_uint = 0, val;
    buf = kmap_local_page(pg) + off;
    if (host.cmd_flags & DATA_CARRY) {
    val = host.bounce_buf_data[0] | ((buf[pos++] << 8) & 0xff00);
    writel(val, sock.addr + SOCK_MMCSD_DATA);
    host.cmd_flags &= ~DATA_CARRY;
    }
    while (pos < cnt) {
    val = buf[pos++];
    if (pos == cnt) {
    host.bounce_buf_data[0] = val & 0xff;
    host.cmd_flags |= DATA_CARRY;
    break;
    }
    val |= (buf[pos++] << 8) & 0xff00;
    writel(val, sock.addr + SOCK_MMCSD_DATA);
    }
    kunmap_local(buf - off);
    }
#[no_mangle]
unsafe extern "C" fn tifm_sd_transfer_data(host: *mut tifm_sd) {
    static void tifm_sd_transfer_data(struct tifm_sd *host)
    {
    struct mmc_data *r_data = host.req.cmd.data;
    struct scatterlist *sg = r_data.sg;
    unsigned int off, cnt, t_size = TIFM_MMCSD_FIFO_SIZE * 2;
    unsigned int p_off, p_cnt;
    struct page *pg;
    if (host.sg_pos == host.sg_len)
    return;
    while (t_size) {
    cnt = sg[host.sg_pos].length - host.block_pos;
    if (!cnt) {
    host.block_pos = 0;
    host.sg_pos++;
    if (host.sg_pos == host.sg_len) {
    if ((r_data.flags & MMC_DATA_WRITE)
    && (host.cmd_flags & DATA_CARRY))
    writel(host.bounce_buf_data[0],
    host.dev.addr
    + SOCK_MMCSD_DATA);
    return;
    }
    cnt = sg[host.sg_pos].length;
    }
    off = sg[host.sg_pos].offset + host.block_pos;
    pg = sg_page(&sg[host.sg_pos]) + (off >> PAGE_SHIFT);
    p_off = offset_in_page(off);
    p_cnt = min3(PAGE_SIZE - p_off, cnt, t_size);
    if (r_data.flags & MMC_DATA_READ)
    tifm_sd_read_fifo(host, pg, p_off, p_cnt);
#[no_mangle]
pub unsafe extern "C" fn if(MMC_DATA_WRITE: r_data->flags &) -> else {
    else if (r_data.flags & MMC_DATA_WRITE)
    tifm_sd_write_fifo(host, pg, p_off, p_cnt);
    t_size -= p_cnt;
    host.block_pos += p_cnt;
    }
    }
    static void tifm_sd_copy_page(struct page *dst, unsigned int dst_off,
    struct page *src, unsigned int src_off,
    unsigned int count)
    {
    unsigned char *src_buf = kmap_local_page(src) + src_off;
    unsigned char *dst_buf = kmap_local_page(dst) + dst_off;
    memcpy(dst_buf, src_buf, count);
    kunmap_local(dst_buf - dst_off);
    kunmap_local(src_buf - src_off);
    }
#[no_mangle]
unsafe extern "C" fn tifm_sd_bounce_block(host: *mut tifm_sd, r_data: *mut mmc_data) {
    static void tifm_sd_bounce_block(struct tifm_sd *host, struct mmc_data *r_data)
    {
    struct scatterlist *sg = r_data.sg;
    let mut t_size: c_uint = r_data.blksz;
    unsigned int off, cnt;
    unsigned int p_off, p_cnt;
    struct page *pg;
    dev_dbg(&host.dev.dev, "bouncing block\n");
    while (t_size) {
    cnt = sg[host.sg_pos].length - host.block_pos;
    if (!cnt) {
    host.block_pos = 0;
    host.sg_pos++;
    if (host.sg_pos == host.sg_len)
    return;
    cnt = sg[host.sg_pos].length;
    }
    off = sg[host.sg_pos].offset + host.block_pos;
    pg = sg_page(&sg[host.sg_pos]) + (off >> PAGE_SHIFT);
    p_off = offset_in_page(off);
    p_cnt = PAGE_SIZE - p_off;
    p_cnt = min(p_cnt, cnt);
    p_cnt = min(p_cnt, t_size);
    if (r_data.flags & MMC_DATA_WRITE)
    tifm_sd_copy_page(sg_page(&host.bounce_buf),
    r_data.blksz - t_size,
    pg, p_off, p_cnt);
#[no_mangle]
pub unsafe extern "C" fn if(MMC_DATA_READ: r_data->flags &) -> else {
    else if (r_data.flags & MMC_DATA_READ)
    tifm_sd_copy_page(pg, p_off, sg_page(&host.bounce_buf),
    r_data.blksz - t_size, p_cnt);
    t_size -= p_cnt;
    host.block_pos += p_cnt;
    }
    }
#[no_mangle]
unsafe extern "C" fn tifm_sd_set_dma_data(host: *mut tifm_sd, r_data: *mut mmc_data) -> c_int {
    static int tifm_sd_set_dma_data(struct tifm_sd *host, struct mmc_data *r_data)
    {
    struct tifm_dev *sock = host.dev;
    let mut t_size: c_uint = TIFM_DMA_TSIZE * r_data.blksz;
    unsigned int dma_len, dma_blk_cnt, dma_off;
    struct scatterlist *sg = core::ptr::null_mut();
    if (host.sg_pos == host.sg_len)
    return 1;
    if (host.cmd_flags & DATA_CARRY) {
    host.cmd_flags &= ~DATA_CARRY;
    tifm_sd_bounce_block(host, r_data);
    if (host.sg_pos == host.sg_len)
    return 1;
    }
    dma_len = sg_dma_len(&r_data.sg[host.sg_pos]) - host.block_pos;
    if (!dma_len) {
    host.block_pos = 0;
    host.sg_pos++;
    if (host.sg_pos == host.sg_len)
    return 1;
    dma_len = sg_dma_len(&r_data.sg[host.sg_pos]);
    }
    if (dma_len < t_size) {
    dma_blk_cnt = dma_len / r_data.blksz;
    dma_off = host.block_pos;
    host.block_pos += dma_blk_cnt * r_data.blksz;
    } else {
    dma_blk_cnt = TIFM_DMA_TSIZE;
    dma_off = host.block_pos;
    host.block_pos += t_size;
    }
    if (dma_blk_cnt)
    sg = &r_data.sg[host.sg_pos];
#[no_mangle]
pub unsafe extern "C" fn if(_arg: dma_len) -> else {
    if (r_data.flags & MMC_DATA_WRITE)
    tifm_sd_bounce_block(host, r_data);
    else
    host.cmd_flags |= DATA_CARRY;
    sg = &host.bounce_buf;
    dma_off = 0;
    dma_blk_cnt = 1;
    } else
    return 1;
    dev_dbg(&sock.dev, "setting dma for %d blocks\n", dma_blk_cnt);
    writel(sg_dma_address(sg) + dma_off, sock.addr + SOCK_DMA_ADDRESS);
    if (r_data.flags & MMC_DATA_WRITE)
    writel((dma_blk_cnt << 8) | TIFM_DMA_TX | TIFM_DMA_EN,
    sock.addr + SOCK_DMA_CONTROL);
    else
    writel((dma_blk_cnt << 8) | TIFM_DMA_EN,
    sock.addr + SOCK_DMA_CONTROL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tifm_sd_op_flags(cmd: *mut mmc_command) -> c_uint {
    static unsigned int tifm_sd_op_flags(struct mmc_command *cmd)
    {
    let mut rc: c_uint = 0;
    switch (mmc_resp_type(cmd)) {
    case MMC_RSP_NONE:
    rc |= TIFM_MMCSD_RSP_R0;
    break;
    case MMC_RSP_R1B:
    rc |= TIFM_MMCSD_RSP_BUSY;
    fallthrough;
    case MMC_RSP_R1:
    rc |= TIFM_MMCSD_RSP_R1;
    break;
    case MMC_RSP_R2:
    rc |= TIFM_MMCSD_RSP_R2;
    break;
    case MMC_RSP_R3:
    rc |= TIFM_MMCSD_RSP_R3;
    break;
    default:
    BUG();
    }
    switch (mmc_cmd_type(cmd)) {
    case MMC_CMD_BC:
    rc |= TIFM_MMCSD_CMD_BC;
    break;
    case MMC_CMD_BCR:
    rc |= TIFM_MMCSD_CMD_BCR;
    break;
    case MMC_CMD_AC:
    rc |= TIFM_MMCSD_CMD_AC;
    break;
    case MMC_CMD_ADTC:
    rc |= TIFM_MMCSD_CMD_ADTC;
    break;
    default:
    BUG();
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn tifm_sd_exec(host: *mut tifm_sd, cmd: *mut mmc_command) {
    static void tifm_sd_exec(struct tifm_sd *host, struct mmc_command *cmd)
    {
    struct tifm_dev *sock = host.dev;
    let mut cmd_mask: c_uint = tifm_sd_op_flags(cmd);
    if (host.open_drain)
    cmd_mask |= TIFM_MMCSD_ODTO;
    if (cmd.data && (cmd.data.flags & MMC_DATA_READ))
    cmd_mask |= TIFM_MMCSD_READ;
    dev_dbg(&sock.dev, "executing opcode 0x%x, arg: 0x%x, mask: 0x%x\n",
    cmd.opcode, cmd.arg, cmd_mask);
    writel((cmd.arg >> 16) & 0xffff, sock.addr + SOCK_MMCSD_ARG_HIGH);
    writel(cmd.arg & 0xffff, sock.addr + SOCK_MMCSD_ARG_LOW);
    writel(cmd.opcode | cmd_mask, sock.addr + SOCK_MMCSD_COMMAND);
    }
#[no_mangle]
unsafe extern "C" fn tifm_sd_fetch_resp(cmd: *mut mmc_command, sock: *mut tifm_dev) {
    static void tifm_sd_fetch_resp(struct mmc_command *cmd, struct tifm_dev *sock)
    {
    cmd.resp[0] = (readl(sock.addr + SOCK_MMCSD_RESPONSE + 0x1c) << 16)
    | readl(sock.addr + SOCK_MMCSD_RESPONSE + 0x18);
    cmd.resp[1] = (readl(sock.addr + SOCK_MMCSD_RESPONSE + 0x14) << 16)
    | readl(sock.addr + SOCK_MMCSD_RESPONSE + 0x10);
    cmd.resp[2] = (readl(sock.addr + SOCK_MMCSD_RESPONSE + 0x0c) << 16)
    | readl(sock.addr + SOCK_MMCSD_RESPONSE + 0x08);
    cmd.resp[3] = (readl(sock.addr + SOCK_MMCSD_RESPONSE + 0x04) << 16)
    | readl(sock.addr + SOCK_MMCSD_RESPONSE + 0x00);
    }
#[no_mangle]
unsafe extern "C" fn tifm_sd_check_status(host: *mut tifm_sd) {
    static void tifm_sd_check_status(struct tifm_sd *host)
    {
    struct tifm_dev *sock = host.dev;
    struct mmc_command *cmd = host.req.cmd;
    if (cmd.error)
    goto finish_request;
    if (!(host.cmd_flags & CMD_READY))
    return;
    if (cmd.data) {
    if (cmd.data.error) {
    if ((host.cmd_flags & SCMD_ACTIVE)
    && !(host.cmd_flags & SCMD_READY))
    return;
    goto finish_request;
    }
    if (!(host.cmd_flags & BRS_READY))
    return;
    if (!(host.no_dma || (host.cmd_flags & FIFO_READY)))
    return;
    if (cmd.data.flags & MMC_DATA_WRITE) {
    if (host.req.stop) {
    if (!(host.cmd_flags & SCMD_ACTIVE)) {
    host.cmd_flags |= SCMD_ACTIVE;
    writel(TIFM_MMCSD_EOFB
    | readl(sock.addr
    + SOCK_MMCSD_INT_ENABLE),
    sock.addr
    + SOCK_MMCSD_INT_ENABLE);
    tifm_sd_exec(host, host.req.stop);
    return;
    } else {
    if (!(host.cmd_flags & SCMD_READY)
    || (host.cmd_flags & CARD_BUSY))
    return;
    writel((~TIFM_MMCSD_EOFB)
    & readl(sock.addr
    + SOCK_MMCSD_INT_ENABLE),
    sock.addr
    + SOCK_MMCSD_INT_ENABLE);
    }
    } else {
    if (host.cmd_flags & CARD_BUSY)
    return;
    writel((~TIFM_MMCSD_EOFB)
    & readl(sock.addr
    + SOCK_MMCSD_INT_ENABLE),
    sock.addr + SOCK_MMCSD_INT_ENABLE);
    }
    } else {
    if (host.req.stop) {
    if (!(host.cmd_flags & SCMD_ACTIVE)) {
    host.cmd_flags |= SCMD_ACTIVE;
    tifm_sd_exec(host, host.req.stop);
    return;
    } else {
    if (!(host.cmd_flags & SCMD_READY))
    return;
    }
    }
    }
    }
    finish_request:
    queue_work(system_bh_wq, &host.finish_bh_work);
    }
// Called from interrupt handler
#[no_mangle]
unsafe extern "C" fn tifm_sd_data_event(sock: *mut tifm_dev) {
    static void tifm_sd_data_event(struct tifm_dev *sock)
    {
    struct tifm_sd *host;
    let mut fifo_status: c_uint = 0;
    struct mmc_data *r_data = core::ptr::null_mut();
    spin_lock(&sock.lock);
    host = mmc_priv((struct mmc_host*)tifm_get_drvdata(sock));
    fifo_status = readl(sock.addr + SOCK_DMA_FIFO_STATUS);
    dev_dbg(&sock.dev, "data event: fifo_status %x, flags %x\n",
    fifo_status, host.cmd_flags);
    if (host.req) {
    r_data = host.req.cmd.data;
    if (r_data && (fifo_status & TIFM_FIFO_READY)) {
    if (tifm_sd_set_dma_data(host, r_data)) {
    host.cmd_flags |= FIFO_READY;
    tifm_sd_check_status(host);
    }
    }
    }
    writel(fifo_status, sock.addr + SOCK_DMA_FIFO_STATUS);
    spin_unlock(&sock.lock);
    }
// Called from interrupt handler
#[no_mangle]
unsafe extern "C" fn tifm_sd_card_event(sock: *mut tifm_dev) {
    static void tifm_sd_card_event(struct tifm_dev *sock)
    {
    struct tifm_sd *host;
    let mut host_status: c_uint = 0;
    let mut cmd_error: c_int = 0;
    struct mmc_command *cmd = core::ptr::null_mut();
    spin_lock(&sock.lock);
    host = mmc_priv((struct mmc_host*)tifm_get_drvdata(sock));
    host_status = readl(sock.addr + SOCK_MMCSD_STATUS);
    dev_dbg(&sock.dev, "host event: host_status %x, flags %x\n",
    host_status, host.cmd_flags);
    if (host.req) {
    cmd = host.req.cmd;
    if (host_status & TIFM_MMCSD_ERRMASK) {
    writel(host_status & TIFM_MMCSD_ERRMASK,
    sock.addr + SOCK_MMCSD_STATUS);
    if (host_status & TIFM_MMCSD_CTO)
    cmd_error = -ETIMEDOUT;
#[no_mangle]
pub unsafe extern "C" fn if(TIFM_MMCSD_CCRC: host_status &) -> else {
    else if (host_status & TIFM_MMCSD_CCRC)
    cmd_error = -EILSEQ;
    if (cmd.data) {
    if (host_status & TIFM_MMCSD_DTO)
    cmd.data.error = -ETIMEDOUT;
#[no_mangle]
pub unsafe extern "C" fn if(TIFM_MMCSD_DCRC: host_status &) -> else {
    else if (host_status & TIFM_MMCSD_DCRC)
    cmd.data.error = -EILSEQ;
    }
    writel(TIFM_FIFO_INT_SETALL,
    sock.addr + SOCK_DMA_FIFO_INT_ENABLE_CLEAR);
    writel(TIFM_DMA_RESET, sock.addr + SOCK_DMA_CONTROL);
    if (host.req.stop) {
    if (host.cmd_flags & SCMD_ACTIVE) {
    host.req.stop.error = cmd_error;
    host.cmd_flags |= SCMD_READY;
    } else {
    cmd.error = cmd_error;
    host.cmd_flags |= SCMD_ACTIVE;
    tifm_sd_exec(host, host.req.stop);
    goto done;
    }
    } else
    cmd.error = cmd_error;
    } else {
    if (host_status & (TIFM_MMCSD_EOC | TIFM_MMCSD_CERR)) {
    if (!(host.cmd_flags & CMD_READY)) {
    host.cmd_flags |= CMD_READY;
    tifm_sd_fetch_resp(cmd, sock);
    } else if (host.cmd_flags & SCMD_ACTIVE) {
    host.cmd_flags |= SCMD_READY;
    tifm_sd_fetch_resp(host.req.stop,
    sock);
    }
    }
    if (host_status & TIFM_MMCSD_BRS)
    host.cmd_flags |= BRS_READY;
    }
    if (host.no_dma && cmd.data) {
    if (host_status & TIFM_MMCSD_AE)
    writel(host_status & TIFM_MMCSD_AE,
    sock.addr + SOCK_MMCSD_STATUS);
    if (host_status & (TIFM_MMCSD_AE | TIFM_MMCSD_AF
    | TIFM_MMCSD_BRS)) {
    tifm_sd_transfer_data(host);
    host_status &= ~TIFM_MMCSD_AE;
    }
    }
    if (host_status & TIFM_MMCSD_EOFB)
    host.cmd_flags &= ~CARD_BUSY;
#[no_mangle]
pub unsafe extern "C" fn if(TIFM_MMCSD_CB: host_status &) -> else {
    else if (host_status & TIFM_MMCSD_CB)
    host.cmd_flags |= CARD_BUSY;
    tifm_sd_check_status(host);
    }
    done:
    writel(host_status, sock.addr + SOCK_MMCSD_STATUS);
    spin_unlock(&sock.lock);
    }
    static void tifm_sd_set_data_timeout(struct tifm_sd *host,
    struct mmc_data *data)
    {
    struct tifm_dev *sock = host.dev;
    let mut data_timeout: c_uint = data.timeout_clks;
    if (fixed_timeout)
    return;
    data_timeout += data.timeout_ns /
    ((1000000000UL / host.clk_freq) * host.clk_div);
    if (data_timeout < 0xffff) {
    writel(data_timeout, sock.addr + SOCK_MMCSD_DATA_TO);
    writel((~TIFM_MMCSD_DPE)
    & readl(sock.addr + SOCK_MMCSD_SDIO_MODE_CONFIG),
    sock.addr + SOCK_MMCSD_SDIO_MODE_CONFIG);
    } else {
    data_timeout = (data_timeout >> 10) + 1;
    if (data_timeout > 0xffff)
    data_timeout = 0;	/* set to unlimited */
    writel(data_timeout, sock.addr + SOCK_MMCSD_DATA_TO);
    writel(TIFM_MMCSD_DPE
    | readl(sock.addr + SOCK_MMCSD_SDIO_MODE_CONFIG),
    sock.addr + SOCK_MMCSD_SDIO_MODE_CONFIG);
    }
    }
#[no_mangle]
unsafe extern "C" fn tifm_sd_request(mmc: *mut mmc_host, mrq: *mut mmc_request) {
    static void tifm_sd_request(struct mmc_host *mmc, struct mmc_request *mrq)
    {
    struct tifm_sd *host = mmc_priv(mmc);
    struct tifm_dev *sock = host.dev;
    unsigned long flags;
    struct mmc_data *r_data = mrq.cmd.data;
    spin_lock_irqsave(&sock.lock, flags);
    if (host.eject) {
    mrq.cmd.error = -ENOMEDIUM;
    goto err_out;
    }
    if (host.req) {
    pr_err("%s : unfinished request detected\n",
    dev_name(&sock.dev));
    mrq.cmd.error = -ETIMEDOUT;
    goto err_out;
    }
    host.cmd_flags = 0;
    host.block_pos = 0;
    host.sg_pos = 0;
    if (mrq.data && !is_power_of_2(mrq.data.blksz))
    host.no_dma = 1;
    else
    host.no_dma = no_dma ? 1 : 0;
    if (r_data) {
    tifm_sd_set_data_timeout(host, r_data);
    if ((r_data.flags & MMC_DATA_WRITE) && !mrq.stop)
    writel(TIFM_MMCSD_EOFB
    | readl(sock.addr + SOCK_MMCSD_INT_ENABLE),
    sock.addr + SOCK_MMCSD_INT_ENABLE);
    if (host.no_dma) {
    writel(TIFM_MMCSD_BUFINT
    | readl(sock.addr + SOCK_MMCSD_INT_ENABLE),
    sock.addr + SOCK_MMCSD_INT_ENABLE);
    writel(((TIFM_MMCSD_FIFO_SIZE - 1) << 8)
    | (TIFM_MMCSD_FIFO_SIZE - 1),
    sock.addr + SOCK_MMCSD_BUFFER_CONFIG);
    host.sg_len = r_data.sg_len;
    } else {
    sg_init_one(&host.bounce_buf, host.bounce_buf_data,
    r_data.blksz);
    if(1 != tifm_map_sg(sock, &host.bounce_buf, 1,
    r_data.flags & MMC_DATA_WRITE
    ? DMA_TO_DEVICE
    : DMA_FROM_DEVICE)) {
    pr_err("%s : scatterlist map failed\n",
    dev_name(&sock.dev));
    mrq.cmd.error = -ENOMEM;
    goto err_out;
    }
    host.sg_len = tifm_map_sg(sock, r_data.sg,
    r_data.sg_len,
    r_data.flags
    & MMC_DATA_WRITE
    ? DMA_TO_DEVICE
    : DMA_FROM_DEVICE);
    if (host.sg_len < 1) {
    pr_err("%s : scatterlist map failed\n",
    dev_name(&sock.dev));
    tifm_unmap_sg(sock, &host.bounce_buf, 1,
    r_data.flags & MMC_DATA_WRITE
    ? DMA_TO_DEVICE
    : DMA_FROM_DEVICE);
    mrq.cmd.error = -ENOMEM;
    goto err_out;
    }
    writel(TIFM_FIFO_INT_SETALL,
    sock.addr + SOCK_DMA_FIFO_INT_ENABLE_CLEAR);
    writel(ilog2(r_data.blksz) - 2,
    sock.addr + SOCK_FIFO_PAGE_SIZE);
    writel(TIFM_FIFO_ENABLE,
    sock.addr + SOCK_FIFO_CONTROL);
    writel(TIFM_FIFO_INTMASK,
    sock.addr + SOCK_DMA_FIFO_INT_ENABLE_SET);
    if (r_data.flags & MMC_DATA_WRITE)
    writel(TIFM_MMCSD_TXDE,
    sock.addr + SOCK_MMCSD_BUFFER_CONFIG);
    else
    writel(TIFM_MMCSD_RXDE,
    sock.addr + SOCK_MMCSD_BUFFER_CONFIG);
    tifm_sd_set_dma_data(host, r_data);
    }
    writel(r_data.blocks - 1,
    sock.addr + SOCK_MMCSD_NUM_BLOCKS);
    writel(r_data.blksz - 1,
    sock.addr + SOCK_MMCSD_BLOCK_LEN);
    }
    host.req = mrq;
    mod_timer(&host.timer, jiffies + host.timeout_jiffies);
    writel(TIFM_CTRL_LED | readl(sock.addr + SOCK_CONTROL),
    sock.addr + SOCK_CONTROL);
    tifm_sd_exec(host, mrq.cmd);
    spin_unlock_irqrestore(&sock.lock, flags);
    return;
    err_out:
    spin_unlock_irqrestore(&sock.lock, flags);
    mmc_request_done(mmc, mrq);
    }
#[no_mangle]
unsafe extern "C" fn tifm_sd_end_cmd(t: *mut work_struct) {
    static void tifm_sd_end_cmd(struct work_struct *t)
    {
    struct tifm_sd *host = from_work(host, t, finish_bh_work);
    struct tifm_dev *sock = host.dev;
    struct mmc_host *mmc = tifm_get_drvdata(sock);
    struct mmc_request *mrq;
    struct mmc_data *r_data = core::ptr::null_mut();
    unsigned long flags;
    spin_lock_irqsave(&sock.lock, flags);
    timer_delete(&host.timer);
    mrq = host.req;
    host.req = core::ptr::null_mut();
    if (!mrq) {
    pr_err(" %s : no request to complete?\n",
    dev_name(&sock.dev));
    spin_unlock_irqrestore(&sock.lock, flags);
    return;
    }
    r_data = mrq.cmd.data;
    if (r_data) {
    if (host.no_dma) {
    writel((~TIFM_MMCSD_BUFINT)
    & readl(sock.addr + SOCK_MMCSD_INT_ENABLE),
    sock.addr + SOCK_MMCSD_INT_ENABLE);
    } else {
    tifm_unmap_sg(sock, &host.bounce_buf, 1,
    (r_data.flags & MMC_DATA_WRITE)
    ? DMA_TO_DEVICE : DMA_FROM_DEVICE);
    tifm_unmap_sg(sock, r_data.sg, r_data.sg_len,
    (r_data.flags & MMC_DATA_WRITE)
    ? DMA_TO_DEVICE : DMA_FROM_DEVICE);
    }
    r_data.bytes_xfered = r_data.blocks
    - readl(sock.addr + SOCK_MMCSD_NUM_BLOCKS) - 1;
    r_data.bytes_xfered *= r_data.blksz;
    r_data.bytes_xfered += r_data.blksz
    - readl(sock.addr + SOCK_MMCSD_BLOCK_LEN) + 1;
    }
    writel((~TIFM_CTRL_LED) & readl(sock.addr + SOCK_CONTROL),
    sock.addr + SOCK_CONTROL);
    spin_unlock_irqrestore(&sock.lock, flags);
    mmc_request_done(mmc, mrq);
    }
#[no_mangle]
unsafe extern "C" fn tifm_sd_abort(t: *mut timer_list) {
    static void tifm_sd_abort(struct timer_list *t)
    {
    struct tifm_sd *host = timer_container_of(host, t, timer);
    pr_err("%s : card failed to respond for a long period of time "
    "(%x, %x)\n",
    dev_name(&host.dev.dev), host.req.cmd.opcode, host.cmd_flags);
    tifm_eject(host.dev);
    }
#[no_mangle]
unsafe extern "C" fn tifm_sd_ios(mmc: *mut mmc_host, ios: *mut mmc_ios) {
    static void tifm_sd_ios(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    struct tifm_sd *host = mmc_priv(mmc);
    struct tifm_dev *sock = host.dev;
    unsigned int clk_div1, clk_div2;
    unsigned long flags;
    spin_lock_irqsave(&sock.lock, flags);
    dev_dbg(&sock.dev, "ios: clock = %u, vdd = %x, bus_mode = %x, "
    "chip_select = %x, power_mode = %x, bus_width = %x\n",
    ios.clock, ios.vdd, ios.bus_mode, ios.chip_select,
    ios.power_mode, ios.bus_width);
    if (ios.bus_width == MMC_BUS_WIDTH_4) {
    writel(TIFM_MMCSD_4BBUS | readl(sock.addr + SOCK_MMCSD_CONFIG),
    sock.addr + SOCK_MMCSD_CONFIG);
    } else {
    writel((~TIFM_MMCSD_4BBUS)
    & readl(sock.addr + SOCK_MMCSD_CONFIG),
    sock.addr + SOCK_MMCSD_CONFIG);
    }
    if (ios.clock) {
    clk_div1 = 20000000 / ios.clock;
    if (!clk_div1)
    clk_div1 = 1;
    clk_div2 = 24000000 / ios.clock;
    if (!clk_div2)
    clk_div2 = 1;
    if ((20000000 / clk_div1) > ios.clock)
    clk_div1++;
    if ((24000000 / clk_div2) > ios.clock)
    clk_div2++;
    if ((20000000 / clk_div1) > (24000000 / clk_div2)) {
    host.clk_freq = 20000000;
    host.clk_div = clk_div1;
    writel((~TIFM_CTRL_FAST_CLK)
    & readl(sock.addr + SOCK_CONTROL),
    sock.addr + SOCK_CONTROL);
    } else {
    host.clk_freq = 24000000;
    host.clk_div = clk_div2;
    writel(TIFM_CTRL_FAST_CLK
    | readl(sock.addr + SOCK_CONTROL),
    sock.addr + SOCK_CONTROL);
    }
    } else {
    host.clk_div = 0;
    }
    host.clk_div &= TIFM_MMCSD_CLKMASK;
    writel(host.clk_div
    | ((~TIFM_MMCSD_CLKMASK)
    & readl(sock.addr + SOCK_MMCSD_CONFIG)),
    sock.addr + SOCK_MMCSD_CONFIG);
    host.open_drain = (ios.bus_mode == MMC_BUSMODE_OPENDRAIN);
// chip_select : maybe later
// vdd
// power is set before probe / after remove
    spin_unlock_irqrestore(&sock.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn tifm_sd_ro(mmc: *mut mmc_host) -> c_int {
    static int tifm_sd_ro(struct mmc_host *mmc)
    {
    let mut rc: c_int = 0;
    struct tifm_sd *host = mmc_priv(mmc);
    struct tifm_dev *sock = host.dev;
    unsigned long flags;
    spin_lock_irqsave(&sock.lock, flags);
    if (TIFM_MMCSD_CARD_RO & readl(sock.addr + SOCK_PRESENT_STATE))
    rc = 1;
    spin_unlock_irqrestore(&sock.lock, flags);
    return rc;
    }
    static const struct mmc_host_ops tifm_sd_ops = {
    .request = tifm_sd_request,
    .set_ios = tifm_sd_ios,
    .get_ro  = tifm_sd_ro
    };
#[no_mangle]
unsafe extern "C" fn tifm_sd_initialize_host(host: *mut tifm_sd) -> c_int {
    static int tifm_sd_initialize_host(struct tifm_sd *host)
    {
    int rc;
    let mut host_status: c_uint = 0;
    struct tifm_dev *sock = host.dev;
    writel(0, sock.addr + SOCK_MMCSD_INT_ENABLE);
    host.clk_div = 61;
    host.clk_freq = 20000000;
    writel(TIFM_MMCSD_RESET, sock.addr + SOCK_MMCSD_SYSTEM_CONTROL);
    writel(host.clk_div | TIFM_MMCSD_POWER,
    sock.addr + SOCK_MMCSD_CONFIG);
// wait up to 0.51 sec for reset
    for (rc = 32; rc <= 256; rc <<= 1) {
    if (1 & readl(sock.addr + SOCK_MMCSD_SYSTEM_STATUS)) {
    rc = 0;
    break;
    }
    msleep(rc);
    }
    if (rc) {
    pr_err("%s : controller failed to reset\n",
    dev_name(&sock.dev));
    return -ENODEV;
    }
    writel(0, sock.addr + SOCK_MMCSD_NUM_BLOCKS);
    writel(host.clk_div | TIFM_MMCSD_POWER,
    sock.addr + SOCK_MMCSD_CONFIG);
    writel(TIFM_MMCSD_RXDE, sock.addr + SOCK_MMCSD_BUFFER_CONFIG);
// command timeout fixed to 64 clocks for now
    writel(64, sock.addr + SOCK_MMCSD_COMMAND_TO);
    writel(TIFM_MMCSD_INAB, sock.addr + SOCK_MMCSD_COMMAND);
    for (rc = 16; rc <= 64; rc <<= 1) {
    host_status = readl(sock.addr + SOCK_MMCSD_STATUS);
    writel(host_status, sock.addr + SOCK_MMCSD_STATUS);
    if (!(host_status & TIFM_MMCSD_ERRMASK)
    && (host_status & TIFM_MMCSD_EOC)) {
    rc = 0;
    break;
    }
    msleep(rc);
    }
    if (rc) {
    pr_err("%s : card not ready - probe failed on initialization\n",
    dev_name(&sock.dev));
    return -ENODEV;
    }
    writel(TIFM_MMCSD_CERR | TIFM_MMCSD_BRS | TIFM_MMCSD_EOC
    | TIFM_MMCSD_ERRMASK,
    sock.addr + SOCK_MMCSD_INT_ENABLE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tifm_sd_probe(sock: *mut tifm_dev) -> c_int {
    static int tifm_sd_probe(struct tifm_dev *sock)
    {
    struct mmc_host *mmc;
    struct tifm_sd *host;
    let mut rc: c_int = -EIO;
    if (!(TIFM_SOCK_STATE_OCCUPIED
    & readl(sock.addr + SOCK_PRESENT_STATE))) {
    pr_warn("%s : card gone, unexpectedly\n",
    dev_name(&sock.dev));
    return rc;
    }
    mmc = devm_mmc_alloc_host(&sock.dev, sizeof(*host));
    if (!mmc)
    return -ENOMEM;
    host = mmc_priv(mmc);
    tifm_set_drvdata(sock, mmc);
    host.dev = sock;
    host.timeout_jiffies = msecs_to_jiffies(TIFM_MMCSD_REQ_TIMEOUT_MS);
//
// We use a fixed request timeout of 1s, hence inform the core about it.
// A future improvement should instead respect the cmd->busy_timeout.
//
    mmc.max_busy_timeout = TIFM_MMCSD_REQ_TIMEOUT_MS;
    INIT_WORK(&host.finish_bh_work, tifm_sd_end_cmd);
    timer_setup(&host.timer, tifm_sd_abort, 0);
    mmc.ops = &tifm_sd_ops;
    mmc.ocr_avail = MMC_VDD_32_33 | MMC_VDD_33_34;
    mmc.caps = MMC_CAP_4_BIT_DATA;
    mmc.f_min = 20000000 / 60;
    mmc.f_max = 24000000;
    mmc.max_blk_count = 2048;
    mmc.max_segs = mmc.max_blk_count;
    mmc.max_blk_size = min(TIFM_MMCSD_MAX_BLOCK_SIZE, PAGE_SIZE);
    mmc.max_seg_size = mmc.max_blk_count * mmc.max_blk_size;
    mmc.max_req_size = mmc.max_seg_size;
    sock.card_event = tifm_sd_card_event;
    sock.data_event = tifm_sd_data_event;
    rc = tifm_sd_initialize_host(host);
    if (!rc)
    rc = mmc_add_host(mmc);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn tifm_sd_remove(sock: *mut tifm_dev) {
    static void tifm_sd_remove(struct tifm_dev *sock)
    {
    struct mmc_host *mmc = tifm_get_drvdata(sock);
    struct tifm_sd *host = mmc_priv(mmc);
    unsigned long flags;
    spin_lock_irqsave(&sock.lock, flags);
    host.eject = 1;
    writel(0, sock.addr + SOCK_MMCSD_INT_ENABLE);
    spin_unlock_irqrestore(&sock.lock, flags);
    cancel_work_sync(&host.finish_bh_work);
    spin_lock_irqsave(&sock.lock, flags);
    if (host.req) {
    writel(TIFM_FIFO_INT_SETALL,
    sock.addr + SOCK_DMA_FIFO_INT_ENABLE_CLEAR);
    writel(0, sock.addr + SOCK_DMA_FIFO_INT_ENABLE_SET);
    host.req.cmd.error = -ENOMEDIUM;
    if (host.req.stop)
    host.req.stop.error = -ENOMEDIUM;
    queue_work(system_bh_wq, &host.finish_bh_work);
    }
    spin_unlock_irqrestore(&sock.lock, flags);
    mmc_remove_host(mmc);
    dev_dbg(&sock.dev, "after remove\n");
    }

#[no_mangle]
unsafe extern "C" fn tifm_sd_suspend(sock: *mut tifm_dev, state: pm_message_t) -> c_int {
    static int tifm_sd_suspend(struct tifm_dev *sock, pm_message_t state)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tifm_sd_resume(sock: *mut tifm_dev) -> c_int {
    static int tifm_sd_resume(struct tifm_dev *sock)
    {
    struct mmc_host *mmc = tifm_get_drvdata(sock);
    struct tifm_sd *host = mmc_priv(mmc);
    int rc;
    rc = tifm_sd_initialize_host(host);
    dev_dbg(&sock.dev, "resume initialize %d\n", rc);
    if (rc)
    host.eject = 1;
    return rc;
    }

    static const struct tifm_device_id tifm_sd_id_tbl[] = {
    { TIFM_TYPE_SD }, { }
    };
    MODULE_DEVICE_TABLE(tifm, tifm_sd_id_tbl);
    static struct tifm_driver tifm_sd_driver = {
    .driver = {
    .name  = DRIVER_NAME,
    .owner = THIS_MODULE
    },
    .id_table = tifm_sd_id_tbl,
    .probe    = tifm_sd_probe,
    .remove   = tifm_sd_remove,
    .suspend  = tifm_sd_suspend,
    .resume   = tifm_sd_resume
    };
#[no_mangle]
unsafe extern "C" fn tifm_sd_init() -> int __init {
    static int __init tifm_sd_init(void)
    {
    return tifm_register_driver(&tifm_sd_driver);
    }
#[no_mangle]
unsafe extern "C" fn tifm_sd_exit() -> void __exit {
    static void __exit tifm_sd_exit(void)
    {
    tifm_unregister_driver(&tifm_sd_driver);
    }
    MODULE_AUTHOR("Alex Dubov");
    MODULE_DESCRIPTION("TI FlashMedia SD driver");
    MODULE_LICENSE("GPL");
    MODULE_VERSION(DRIVER_VERSION);
    module_init(tifm_sd_init);
    module_exit(tifm_sd_exit);
