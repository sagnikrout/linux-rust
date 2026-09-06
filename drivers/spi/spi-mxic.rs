//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-mxic.c
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
// Copyright (C) 2018 Macronix International Co., Ltd.
//
// Authors:
// Mason Yang <masonccyang@mxic.com.tw>
// zhengxunli <zhengxunli@mxic.com.tw>
// Boris Brezillon <boris.brezillon@bootlin.com>
//

pub const HC_CFG: c_uint = 0x0;

pub const HC_CFG_TYPE_SPI_NOR: c_int = 0;
pub const HC_CFG_TYPE_SPI_NAND: c_int = 1;
pub const HC_CFG_TYPE_SPI_RAM: c_int = 2;
pub const HC_CFG_TYPE_RAW_NAND: c_int = 3;

pub const INT_STS: c_uint = 0x4;
pub const INT_STS_EN: c_uint = 0x8;
pub const INT_SIG_EN: c_uint = 0xc;

pub const HC_EN: c_uint = 0x10;

pub const RXD: c_uint = 0x24;

pub const LRD_CFG: c_uint = 0x44;
pub const LWR_CFG: c_uint = 0x80;
pub const RWW_CFG: c_uint = 0x70;

pub const OP_BUSW_1: c_int = 0;
pub const OP_BUSW_2: c_int = 1;
pub const OP_BUSW_4: c_int = 2;
pub const OP_BUSW_8: c_int = 3;
pub const OCTA_CRC: c_uint = 0x38;

pub const LRD_CTRL: c_uint = 0x48;
pub const RWW_CTRL: c_uint = 0x74;
pub const LWR_CTRL: c_uint = 0x84;

pub const LRD_ADDR: c_uint = 0x4c;
pub const LWR_ADDR: c_uint = 0x88;
pub const LRD_RANGE: c_uint = 0x50;
pub const LWR_RANGE: c_uint = 0x8c;
pub const AXI_SLV_ADDR: c_uint = 0x54;
pub const DMAC_RD_CFG: c_uint = 0x58;
pub const DMAC_WR_CFG: c_uint = 0x94;

pub const DMAC_RD_CNT: c_uint = 0x5c;
pub const DMAC_WR_CNT: c_uint = 0x98;
pub const SDMA_ADDR: c_uint = 0x60;
pub const DMAM_CFG: c_uint = 0x64;

pub const DMAM_CNT: c_uint = 0x68;
pub const LNR_TIMER_TH: c_uint = 0x6c;
pub const RDM_CFG0: c_uint = 0x78;

pub const RDM_CFG1: c_uint = 0x7c;

pub const LWR_SUSP_CTRL: c_uint = 0x90;

pub const DMAS_CTRL: c_uint = 0x9c;

pub const DATA_STROB: c_uint = 0xa0;

pub const GPIO: c_uint = 0xc4;

pub const HC_VER: c_uint = 0xd0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxic_spi {
    pub dev: *mut device,
    pub ps_clk: *mut clk,
    pub send_clk: *mut clk,
    pub send_dly_clk: *mut clk,
    pub regs: *mut void __iomem,
    pub cur_speed_hz: u32,
    struct {
    pub map: *mut void __iomem,
    pub dma: dma_addr_t,
    pub size: usize,
    pub linear: },
    struct {
    pub use_pipelined_conf: bool,
    pub pipelined_engine: *mut nand_ecc_engine,
    pub ctx: *mut c_void,
    pub ecc: },
}

#[no_mangle]
unsafe extern "C" fn mxic_spi_clk_enable(mxic: *mut mxic_spi) -> c_int {
    static int mxic_spi_clk_enable(struct mxic_spi *mxic)
    {
    int ret;
    ret = clk_prepare_enable(mxic.send_clk);
    if (ret)
    return ret;
    ret = clk_prepare_enable(mxic.send_dly_clk);
    if (ret)
    goto err_send_dly_clk;
    return ret;
    err_send_dly_clk:
    clk_disable_unprepare(mxic.send_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mxic_spi_clk_disable(mxic: *mut mxic_spi) {
    static void mxic_spi_clk_disable(struct mxic_spi *mxic)
    {
    clk_disable_unprepare(mxic.send_clk);
    clk_disable_unprepare(mxic.send_dly_clk);
    }
#[no_mangle]
unsafe extern "C" fn mxic_spi_set_input_delay_dqs(mxic: *mut mxic_spi, idly_code: u8) {
    static void mxic_spi_set_input_delay_dqs(struct mxic_spi *mxic, u8 idly_code)
    {
    writel(IDLY_CODE_VAL(0, idly_code) |
    IDLY_CODE_VAL(1, idly_code) |
    IDLY_CODE_VAL(2, idly_code) |
    IDLY_CODE_VAL(3, idly_code),
    mxic.regs + IDLY_CODE(0));
    writel(IDLY_CODE_VAL(4, idly_code) |
    IDLY_CODE_VAL(5, idly_code) |
    IDLY_CODE_VAL(6, idly_code) |
    IDLY_CODE_VAL(7, idly_code),
    mxic.regs + IDLY_CODE(1));
    }
#[no_mangle]
unsafe extern "C" fn mxic_spi_clk_setup(mxic: *mut mxic_spi, freq: c_ulong) -> c_int {
    static int mxic_spi_clk_setup(struct mxic_spi *mxic, unsigned long freq)
    {
    int ret;
    ret = clk_set_rate(mxic.send_clk, freq);
    if (ret)
    return ret;
    ret = clk_set_rate(mxic.send_dly_clk, freq);
    if (ret)
    return ret;
//
// A constant delay range from 0x0 ~ 0x1F for input delay,
// the unit is 78 ps, the max input delay is 2.418 ns.
//
    mxic_spi_set_input_delay_dqs(mxic, 0xf);
//
// Phase degree = 360 * freq * output-delay
// where output-delay is a constant value 1 ns in FPGA.
//
// Get Phase degree = 360 * freq * 1 ns
// = 360 * freq * 1 sec / 1000000000
// = 9 * freq / 25000000
//
    ret = clk_set_phase(mxic.send_dly_clk, 9 * freq / 25000000);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxic_spi_set_freq(mxic: *mut mxic_spi, freq: c_ulong) -> c_int {
    static int mxic_spi_set_freq(struct mxic_spi *mxic, unsigned long freq)
    {
    int ret;
    if (mxic.cur_speed_hz == freq)
    return 0;
    mxic_spi_clk_disable(mxic);
    ret = mxic_spi_clk_setup(mxic, freq);
    if (ret)
    return ret;
    ret = mxic_spi_clk_enable(mxic);
    if (ret)
    return ret;
    mxic.cur_speed_hz = freq;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxic_spi_hw_init(mxic: *mut mxic_spi) {
    static void mxic_spi_hw_init(struct mxic_spi *mxic)
    {
    writel(0, mxic.regs + DATA_STROB);
    writel(INT_STS_ALL, mxic.regs + INT_STS_EN);
    writel(0, mxic.regs + HC_EN);
    writel(0, mxic.regs + LRD_CFG);
    writel(0, mxic.regs + LRD_CTRL);
    writel(HC_CFG_NIO(1) | HC_CFG_TYPE(0, HC_CFG_TYPE_SPI_NOR) |
    HC_CFG_SLV_ACT(0) | HC_CFG_MAN_CS_EN | HC_CFG_IDLE_SIO_LVL(1),
    mxic.regs + HC_CFG);
    }
    static u32 mxic_spi_prep_hc_cfg(struct spi_device *spi, u32 flags,
    bool swap16)
    {
    let mut nio: c_int = 1;
    if (spi.mode & (SPI_TX_OCTAL | SPI_RX_OCTAL))
    nio = 8;
#[no_mangle]
pub unsafe extern "C" fn if(SPI_RX_QUAD): spi->mode & (SPI_TX_QUAD |) -> else {
    else if (spi.mode & (SPI_TX_QUAD | SPI_RX_QUAD))
    nio = 4;
#[no_mangle]
pub unsafe extern "C" fn if(SPI_RX_DUAL): spi->mode & (SPI_TX_DUAL |) -> else {
    else if (spi.mode & (SPI_TX_DUAL | SPI_RX_DUAL))
    nio = 2;
    if (swap16)
    flags &= ~HC_CFG_DATA_PASS;
    else
    flags |= HC_CFG_DATA_PASS;
    return flags | HC_CFG_NIO(nio) |
    HC_CFG_TYPE(spi_get_chipselect(spi, 0), HC_CFG_TYPE_SPI_NOR) |
    HC_CFG_SLV_ACT(spi_get_chipselect(spi, 0)) | HC_CFG_IDLE_SIO_LVL(1);
    }
    static u32 mxic_spi_mem_prep_op_cfg(const struct spi_mem_op *op,
    unsigned int data_len)
    {
    u32 cfg = OP_CMD_BYTES(op.cmd.nbytes) |
    OP_CMD_BUSW(fls(op.cmd.buswidth) - 1) |
    (op.cmd.dtr ? OP_CMD_DDR : 0);
    if (op.addr.nbytes)
    cfg |= OP_ADDR_BYTES(op.addr.nbytes) |
    OP_ADDR_BUSW(fls(op.addr.buswidth) - 1) |
    (op.addr.dtr ? OP_ADDR_DDR : 0);
    if (op.dummy.nbytes)
    cfg |= OP_DUMMY_CYC(op.dummy.nbytes);
// Direct mapping data.nbytes field is not populated
    if (data_len) {
    cfg |= OP_DATA_BUSW(fls(op.data.buswidth) - 1) |
    (op.data.dtr ? OP_DATA_DDR : 0);
    if (op.data.dir == SPI_MEM_DATA_IN) {
    cfg |= OP_READ;
    if (op.data.dtr)
    cfg |= OP_DQS_EN;
    }
    }
    return cfg;
    }
    static int mxic_spi_data_xfer(struct mxic_spi *mxic, const void *txbuf,
    void *rxbuf, unsigned int len)
    {
    let mut pos: c_uint = 0;
    while (pos < len) {
    let mut nbytes: c_uint = len - pos;
    let mut data: u32 = 0xffffffff;
    u32 sts;
    int ret;
    if (nbytes > 4)
    nbytes = 4;
    if (txbuf)
    memcpy(&data, txbuf + pos, nbytes);
    ret = readl_poll_timeout(mxic.regs + INT_STS, sts,
    sts & INT_TX_EMPTY, 0, USEC_PER_SEC);
    if (ret)
    return ret;
    writel(data, mxic.regs + TXD(nbytes % 4));
    ret = readl_poll_timeout(mxic.regs + INT_STS, sts,
    sts & INT_TX_EMPTY, 0, USEC_PER_SEC);
    if (ret)
    return ret;
    ret = readl_poll_timeout(mxic.regs + INT_STS, sts,
    sts & INT_RX_NOT_EMPTY, 0,
    USEC_PER_SEC);
    if (ret)
    return ret;
    data = readl(mxic.regs + RXD);
    if (rxbuf) {
    data >>= (8 * (4 - nbytes));
    memcpy(rxbuf + pos, &data, nbytes);
    }
    WARN_ON(readl(mxic.regs + INT_STS) & INT_RX_NOT_EMPTY);
    pos += nbytes;
    }
    return 0;
    }
    static ssize_t mxic_spi_mem_dirmap_read(struct spi_mem_dirmap_desc *desc,
    u64 offs, size_t len, void *buf)
    {
    struct mxic_spi *mxic = spi_controller_get_devdata(desc.mem.spi.controller);
    int ret;
    u32 sts;
    if (WARN_ON(offs + desc.info.offset + len > U32_MAX))
    return -EINVAL;
    writel(mxic_spi_prep_hc_cfg(desc.mem.spi, 0, desc.info.op_tmpl.data.swap16),
    mxic.regs + HC_CFG);
    writel(mxic_spi_mem_prep_op_cfg(desc.info.op_tmpl, len),
    mxic.regs + LRD_CFG);
    writel(desc.info.offset + offs, mxic.regs + LRD_ADDR);
    len = min_t(size_t, len, mxic.linear.size);
    writel(len, mxic.regs + LRD_RANGE);
    writel(LMODE_CMD0(desc.info.op_tmpl.cmd.opcode) |
    LMODE_SLV_ACT(spi_get_chipselect(desc.mem.spi, 0)) |
    LMODE_EN,
    mxic.regs + LRD_CTRL);
    if (mxic.ecc.use_pipelined_conf && desc.info.op_tmpl.data.ecc) {
    ret = mxic_ecc_process_data_pipelined(mxic.ecc.pipelined_engine,
    NAND_PAGE_READ,
    mxic.linear.dma + offs);
    if (ret)
    return ret;
    } else {
    memcpy_fromio(buf, mxic.linear.map, len);
    }
    writel(INT_LRD_DIS, mxic.regs + INT_STS);
    writel(0, mxic.regs + LRD_CTRL);
    ret = readl_poll_timeout(mxic.regs + INT_STS, sts,
    sts & INT_LRD_DIS, 0, USEC_PER_SEC);
    if (ret)
    return ret;
    return len;
    }
    static ssize_t mxic_spi_mem_dirmap_write(struct spi_mem_dirmap_desc *desc,
    u64 offs, size_t len,
    const void *buf)
    {
    struct mxic_spi *mxic = spi_controller_get_devdata(desc.mem.spi.controller);
    u32 sts;
    int ret;
    if (WARN_ON(offs + desc.info.offset + len > U32_MAX))
    return -EINVAL;
    writel(mxic_spi_prep_hc_cfg(desc.mem.spi, 0, desc.info.op_tmpl.data.swap16),
    mxic.regs + HC_CFG);
    writel(mxic_spi_mem_prep_op_cfg(desc.info.op_tmpl, len),
    mxic.regs + LWR_CFG);
    writel(desc.info.offset + offs, mxic.regs + LWR_ADDR);
    len = min_t(size_t, len, mxic.linear.size);
    writel(len, mxic.regs + LWR_RANGE);
    writel(LMODE_CMD0(desc.info.op_tmpl.cmd.opcode) |
    LMODE_SLV_ACT(spi_get_chipselect(desc.mem.spi, 0)) |
    LMODE_EN,
    mxic.regs + LWR_CTRL);
    if (mxic.ecc.use_pipelined_conf && desc.info.op_tmpl.data.ecc) {
    ret = mxic_ecc_process_data_pipelined(mxic.ecc.pipelined_engine,
    NAND_PAGE_WRITE,
    mxic.linear.dma + offs);
    if (ret)
    return ret;
    } else {
    memcpy_toio(mxic.linear.map, buf, len);
    }
    writel(INT_LWR_DIS, mxic.regs + INT_STS);
    writel(0, mxic.regs + LWR_CTRL);
    ret = readl_poll_timeout(mxic.regs + INT_STS, sts,
    sts & INT_LWR_DIS, 0, USEC_PER_SEC);
    if (ret)
    return ret;
    return len;
    }
    static bool mxic_spi_mem_supports_op(struct spi_mem *mem,
    const struct spi_mem_op *op)
    {
    if (op.data.buswidth > 8 || op.addr.buswidth > 8 ||
    op.dummy.buswidth > 8 || op.cmd.buswidth > 8)
    return false;
    if (op.data.nbytes && op.dummy.nbytes &&
    op.data.buswidth != op.dummy.buswidth)
    return false;
    if (op.addr.nbytes > 7)
    return false;
    return spi_mem_default_supports_op(mem, op);
    }
#[no_mangle]
unsafe extern "C" fn mxic_spi_mem_dirmap_create(desc: *mut spi_mem_dirmap_desc) -> c_int {
    static int mxic_spi_mem_dirmap_create(struct spi_mem_dirmap_desc *desc)
    {
    struct mxic_spi *mxic = spi_controller_get_devdata(desc.mem.spi.controller);
    if (!mxic.linear.map)
    return -EOPNOTSUPP;
    if (desc.info.offset + desc.info.length > U32_MAX)
    return -EINVAL;
    if (!mxic_spi_mem_supports_op(desc.mem, desc.info.op_tmpl))
    return -EOPNOTSUPP;
    return 0;
    }
    static int mxic_spi_mem_exec_op(struct spi_mem *mem,
    const struct spi_mem_op *op)
    {
    struct mxic_spi *mxic = spi_controller_get_devdata(mem.spi.controller);
    int i, ret;
    u8 addr[8], cmd[2];
    ret = mxic_spi_set_freq(mxic, op.max_freq);
    if (ret)
    return ret;
    writel(mxic_spi_prep_hc_cfg(mem.spi, HC_CFG_MAN_CS_EN, op.data.swap16),
    mxic.regs + HC_CFG);
    writel(HC_EN_BIT, mxic.regs + HC_EN);
    writel(mxic_spi_mem_prep_op_cfg(op, op.data.nbytes),
    mxic.regs + SS_CTRL(spi_get_chipselect(mem.spi, 0)));
    writel(readl(mxic.regs + HC_CFG) | HC_CFG_MAN_CS_ASSERT,
    mxic.regs + HC_CFG);
    for (i = 0; i < op.cmd.nbytes; i++)
    cmd[i] = op.cmd.opcode >> (8 * (op.cmd.nbytes - i - 1));
    ret = mxic_spi_data_xfer(mxic, cmd, core::ptr::null_mut(), op.cmd.nbytes);
    if (ret)
    goto out;
    for (i = 0; i < op.addr.nbytes; i++)
    addr[i] = op.addr.val >> (8 * (op.addr.nbytes - i - 1));
    ret = mxic_spi_data_xfer(mxic, addr, core::ptr::null_mut(), op.addr.nbytes);
    if (ret)
    goto out;
    ret = mxic_spi_data_xfer(mxic, core::ptr::null_mut(), core::ptr::null_mut(), op.dummy.nbytes);
    if (ret)
    goto out;
    ret = mxic_spi_data_xfer(mxic,
    op.data.dir == SPI_MEM_DATA_OUT ?
    op.data.buf.out : core::ptr::null_mut(),
    op.data.dir == SPI_MEM_DATA_IN ?
    op.data.buf.in : core::ptr::null_mut(),
    op.data.nbytes);
    out:
    writel(readl(mxic.regs + HC_CFG) & ~HC_CFG_MAN_CS_ASSERT,
    mxic.regs + HC_CFG);
    writel(0, mxic.regs + HC_EN);
    return ret;
    }
    static const struct spi_controller_mem_ops mxic_spi_mem_ops = {
    .supports_op = mxic_spi_mem_supports_op,
    .exec_op = mxic_spi_mem_exec_op,
    .dirmap_create = mxic_spi_mem_dirmap_create,
    .dirmap_read = mxic_spi_mem_dirmap_read,
    .dirmap_write = mxic_spi_mem_dirmap_write,
    };
    static const struct spi_controller_mem_caps mxic_spi_mem_caps = {
    .dtr = true,
    .ecc = true,
    .swap16 = true,
    .per_op_freq = true,
    };
#[no_mangle]
unsafe extern "C" fn mxic_spi_set_cs(spi: *mut spi_device, lvl: bool) {
    static void mxic_spi_set_cs(struct spi_device *spi, bool lvl)
    {
    struct mxic_spi *mxic = spi_controller_get_devdata(spi.controller);
    if (!lvl) {
    writel(readl(mxic.regs + HC_CFG) | HC_CFG_MAN_CS_EN,
    mxic.regs + HC_CFG);
    writel(HC_EN_BIT, mxic.regs + HC_EN);
    writel(readl(mxic.regs + HC_CFG) | HC_CFG_MAN_CS_ASSERT,
    mxic.regs + HC_CFG);
    } else {
    writel(readl(mxic.regs + HC_CFG) & ~HC_CFG_MAN_CS_ASSERT,
    mxic.regs + HC_CFG);
    writel(0, mxic.regs + HC_EN);
    }
    }
    static int mxic_spi_transfer_one(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct mxic_spi *mxic = spi_controller_get_devdata(host);
    let mut busw: c_uint = OP_BUSW_1;
    int ret;
    if (t.rx_buf && t.tx_buf) {
    if (((spi.mode & SPI_TX_QUAD) &&
    !(spi.mode & SPI_RX_QUAD)) ||
    ((spi.mode & SPI_TX_DUAL) &&
    !(spi.mode & SPI_RX_DUAL)))
    return -ENOTSUPP;
    }
    ret = mxic_spi_set_freq(mxic, t.speed_hz);
    if (ret)
    return ret;
    if (t.tx_buf) {
    if (spi.mode & SPI_TX_QUAD)
    busw = OP_BUSW_4;
#[no_mangle]
pub unsafe extern "C" fn if(SPI_TX_DUAL: spi->mode &) -> else {
    else if (spi.mode & SPI_TX_DUAL)
    busw = OP_BUSW_2;
    } else if (t.rx_buf) {
    if (spi.mode & SPI_RX_QUAD)
    busw = OP_BUSW_4;
#[no_mangle]
pub unsafe extern "C" fn if(SPI_RX_DUAL: spi->mode &) -> else {
    else if (spi.mode & SPI_RX_DUAL)
    busw = OP_BUSW_2;
    }
    writel(OP_CMD_BYTES(1) | OP_CMD_BUSW(busw) |
    OP_DATA_BUSW(busw) | (t.rx_buf ? OP_READ : 0),
    mxic.regs + SS_CTRL(0));
    ret = mxic_spi_data_xfer(mxic, t.tx_buf, t.rx_buf, t.len);
    if (ret)
    return ret;
    spi_finalize_current_transfer(host);
    return 0;
    }
// ECC wrapper
#[no_mangle]
unsafe extern "C" fn mxic_spi_mem_ecc_init_ctx(nand: *mut nand_device) -> c_int {
    static int mxic_spi_mem_ecc_init_ctx(struct nand_device *nand)
    {
    const struct nand_ecc_engine_ops *ops = mxic_ecc_get_pipelined_ops();
    struct mxic_spi *mxic = nand.ecc.engine.priv;
    mxic.ecc.use_pipelined_conf = true;
    return ops.init_ctx(nand);
    }
#[no_mangle]
unsafe extern "C" fn mxic_spi_mem_ecc_cleanup_ctx(nand: *mut nand_device) {
    static void mxic_spi_mem_ecc_cleanup_ctx(struct nand_device *nand)
    {
    const struct nand_ecc_engine_ops *ops = mxic_ecc_get_pipelined_ops();
    struct mxic_spi *mxic = nand.ecc.engine.priv;
    mxic.ecc.use_pipelined_conf = false;
    ops.cleanup_ctx(nand);
    }
    static int mxic_spi_mem_ecc_prepare_io_req(struct nand_device *nand,
    struct nand_page_io_req *req)
    {
    const struct nand_ecc_engine_ops *ops = mxic_ecc_get_pipelined_ops();
    return ops.prepare_io_req(nand, req);
    }
    static int mxic_spi_mem_ecc_finish_io_req(struct nand_device *nand,
    struct nand_page_io_req *req)
    {
    const struct nand_ecc_engine_ops *ops = mxic_ecc_get_pipelined_ops();
    return ops.finish_io_req(nand, req);
    }
    static const struct nand_ecc_engine_ops mxic_spi_mem_ecc_engine_pipelined_ops = {
    .init_ctx = mxic_spi_mem_ecc_init_ctx,
    .cleanup_ctx = mxic_spi_mem_ecc_cleanup_ctx,
    .prepare_io_req = mxic_spi_mem_ecc_prepare_io_req,
    .finish_io_req = mxic_spi_mem_ecc_finish_io_req,
    };
#[no_mangle]
unsafe extern "C" fn mxic_spi_mem_ecc_remove(mxic: *mut mxic_spi) {
    static void mxic_spi_mem_ecc_remove(struct mxic_spi *mxic)
    {
    if (mxic.ecc.pipelined_engine) {
    mxic_ecc_put_pipelined_engine(mxic.ecc.pipelined_engine);
    nand_ecc_unregister_on_host_hw_engine(mxic.ecc.pipelined_engine);
    }
    }
    static int mxic_spi_mem_ecc_probe(struct platform_device *pdev,
    struct mxic_spi *mxic)
    {
    struct nand_ecc_engine *eng;
    if (!mxic_ecc_get_pipelined_ops())
    return -EOPNOTSUPP;
    eng = mxic_ecc_get_pipelined_engine(pdev);
    if (IS_ERR(eng))
    return PTR_ERR(eng);
    eng.dev = &pdev.dev;
    eng.integration = NAND_ECC_ENGINE_INTEGRATION_PIPELINED;
    eng.ops = &mxic_spi_mem_ecc_engine_pipelined_ops;
    eng.priv = mxic;
    mxic.ecc.pipelined_engine = eng;
    nand_ecc_register_on_host_hw_engine(eng);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxic_spi_runtime_suspend(dev: *mut device) -> c_int {
    static int mxic_spi_runtime_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct mxic_spi *mxic = spi_controller_get_devdata(host);
    mxic_spi_clk_disable(mxic);
    clk_disable_unprepare(mxic.ps_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxic_spi_runtime_resume(dev: *mut device) -> c_int {
    static int mxic_spi_runtime_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct mxic_spi *mxic = spi_controller_get_devdata(host);
    int ret;
    ret = clk_prepare_enable(mxic.ps_clk);
    if (ret) {
    dev_err(dev, "Cannot enable ps_clock.\n");
    return ret;
    }
    return mxic_spi_clk_enable(mxic);
    }
    static const struct dev_pm_ops mxic_spi_dev_pm_ops = {
    RUNTIME_PM_OPS(mxic_spi_runtime_suspend, mxic_spi_runtime_resume, core::ptr::null_mut())
    };
#[no_mangle]
unsafe extern "C" fn mxic_spi_probe(pdev: *mut platform_device) -> c_int {
    static int mxic_spi_probe(struct platform_device *pdev)
    {
    struct spi_controller *host;
    struct resource *res;
    struct mxic_spi *mxic;
    int ret;
    host = devm_spi_alloc_host(&pdev.dev, sizeof(struct mxic_spi));
    if (!host)
    return -ENOMEM;
    platform_set_drvdata(pdev, host);
    mxic = spi_controller_get_devdata(host);
    mxic.dev = &pdev.dev;
    mxic.ps_clk = devm_clk_get(&pdev.dev, "ps_clk");
    if (IS_ERR(mxic.ps_clk))
    return PTR_ERR(mxic.ps_clk);
    mxic.send_clk = devm_clk_get(&pdev.dev, "send_clk");
    if (IS_ERR(mxic.send_clk))
    return PTR_ERR(mxic.send_clk);
    mxic.send_dly_clk = devm_clk_get(&pdev.dev, "send_dly_clk");
    if (IS_ERR(mxic.send_dly_clk))
    return PTR_ERR(mxic.send_dly_clk);
    mxic.regs = devm_platform_ioremap_resource_byname(pdev, "regs");
    if (IS_ERR(mxic.regs))
    return PTR_ERR(mxic.regs);
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "dirmap");
    mxic.linear.map = devm_ioremap_resource(&pdev.dev, res);
    if (!IS_ERR(mxic.linear.map)) {
    mxic.linear.dma = res.start;
    mxic.linear.size = resource_size(res);
    } else {
    mxic.linear.map = core::ptr::null_mut();
    }
    pm_runtime_enable(&pdev.dev);
    host.auto_runtime_pm = true;
    host.num_chipselect = 1;
    host.mem_ops = &mxic_spi_mem_ops;
    host.mem_caps = &mxic_spi_mem_caps;
    host.set_cs = mxic_spi_set_cs;
    host.transfer_one = mxic_spi_transfer_one;
    host.bits_per_word_mask = SPI_BPW_MASK(8);
    host.mode_bits = SPI_CPOL | SPI_CPHA |
    SPI_RX_DUAL | SPI_TX_DUAL |
    SPI_RX_QUAD | SPI_TX_QUAD |
    SPI_RX_OCTAL | SPI_TX_OCTAL;
    mxic_spi_hw_init(mxic);
    ret = mxic_spi_mem_ecc_probe(pdev, mxic);
    if (ret == -EPROBE_DEFER) {
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
    ret = spi_register_controller(host);
    if (ret) {
    dev_err(&pdev.dev, "spi_register_controller failed\n");
    pm_runtime_disable(&pdev.dev);
    mxic_spi_mem_ecc_remove(mxic);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mxic_spi_remove(pdev: *mut platform_device) {
    static void mxic_spi_remove(struct platform_device *pdev)
    {
    struct spi_controller *host = platform_get_drvdata(pdev);
    struct mxic_spi *mxic = spi_controller_get_devdata(host);
    spi_unregister_controller(host);
    pm_runtime_disable(&pdev.dev);
    mxic_spi_mem_ecc_remove(mxic);
    }
    static const struct of_device_id mxic_spi_of_ids[] = {
    { .compatible = "mxicy,mx25f0a-spi", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, mxic_spi_of_ids);
    static struct platform_driver mxic_spi_driver = {
    .probe = mxic_spi_probe,
    .remove = mxic_spi_remove,
    .driver = {
    .name = "mxic-spi",
    .of_match_table = mxic_spi_of_ids,
    .pm = pm_ptr(&mxic_spi_dev_pm_ops),
    },
    };
    module_platform_driver(mxic_spi_driver);
    MODULE_AUTHOR("Mason Yang <masonccyang@mxic.com.tw>");
    MODULE_DESCRIPTION("MX25F0A SPI controller driver");
    MODULE_LICENSE("GPL v2");
