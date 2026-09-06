//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-rockchip-sfc.c
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
// Rockchip Serial Flash Controller Driver
//
// Copyright (c) 2017-2021, Rockchip Inc.
// Author: Shawn Lin <shawn.lin@rock-chips.com>
// Chris Morgan <macroalpha82@gmail.com>
// Jon Lin <Jon.lin@rock-chips.com>
//

// System control
pub const SFC_CTRL: c_uint = 0x0;

pub const SFC_CTRL_CMD_BITS_SHIFT: c_int = 8;
pub const SFC_CTRL_ADDR_BITS_SHIFT: c_int = 10;
pub const SFC_CTRL_DATA_BITS_SHIFT: c_int = 12;
// Interrupt mask
pub const SFC_IMR: c_uint = 0x4;

// Interrupt clear
pub const SFC_ICLR: c_uint = 0x8;

// FIFO threshold level
pub const SFC_FTLR: c_uint = 0xc;
pub const SFC_FTLR_TX_SHIFT: c_int = 0;
pub const SFC_FTLR_TX_MASK: c_uint = 0x1f;
pub const SFC_FTLR_RX_SHIFT: c_int = 8;
pub const SFC_FTLR_RX_MASK: c_uint = 0x1f;
// Reset FSM and FIFO
pub const SFC_RCVR: c_uint = 0x10;

// Enhanced mode
pub const SFC_AX: c_uint = 0x14;
// Address Bit number
pub const SFC_ABIT: c_uint = 0x18;
// Interrupt status
pub const SFC_ISR: c_uint = 0x1c;

// FIFO status
pub const SFC_FSR: c_uint = 0x20;

pub const SFC_FSR_TXLV_SHIFT: c_int = 8;

pub const SFC_FSR_RXLV_SHIFT: c_int = 16;
// FSM status
pub const SFC_SR: c_uint = 0x24;
pub const SFC_SR_IS_IDLE: c_uint = 0x0;
pub const SFC_SR_IS_BUSY: c_uint = 0x1;
// Raw interrupt status
pub const SFC_RISR: c_uint = 0x28;

// Version
pub const SFC_VER: c_uint = 0x2C;
pub const SFC_VER_3: c_uint = 0x3;
pub const SFC_VER_4: c_uint = 0x4;
pub const SFC_VER_5: c_uint = 0x5;
pub const SFC_VER_8: c_uint = 0x8;
// Delay line controller register
pub const SFC_DLL_CTRL0: c_uint = 0x3C;

pub const SFC_DLL_CTRL0_DLL_MAX_VER4: c_uint = 0xFFU;
pub const SFC_DLL_CTRL0_DLL_MAX_VER5: c_uint = 0x1FFU;
// Master trigger
pub const SFC_DMA_TRIGGER: c_uint = 0x80;
pub const SFC_DMA_TRIGGER_START: c_int = 1;
// Src or Dst addr for master
pub const SFC_DMA_ADDR: c_uint = 0x84;
// Length control register extension 32GB
pub const SFC_LEN_CTRL: c_uint = 0x88;
pub const SFC_LEN_CTRL_TRB_SEL: c_int = 1;
pub const SFC_LEN_EXT: c_uint = 0x8C;
// Command
pub const SFC_CMD: c_uint = 0x100;
pub const SFC_CMD_IDX_SHIFT: c_int = 0;
pub const SFC_CMD_DUMMY_SHIFT: c_int = 8;
pub const SFC_CMD_DIR_SHIFT: c_int = 12;
pub const SFC_CMD_DIR_RD: c_int = 0;
pub const SFC_CMD_DIR_WR: c_int = 1;
pub const SFC_CMD_ADDR_SHIFT: c_int = 14;
pub const SFC_CMD_ADDR_0BITS: c_int = 0;
pub const SFC_CMD_ADDR_24BITS: c_int = 1;
pub const SFC_CMD_ADDR_32BITS: c_int = 2;
pub const SFC_CMD_ADDR_XBITS: c_int = 3;
pub const SFC_CMD_TRAN_BYTES_SHIFT: c_int = 16;
pub const SFC_CMD_CS_SHIFT: c_int = 30;
// Address
pub const SFC_ADDR: c_uint = 0x104;
// Data
pub const SFC_DATA: c_uint = 0x108;
pub const SFC_CS1_REG_OFFSET: c_uint = 0x200;
pub const SFC_MAX_CHIPSELECT_NUM: c_int = 2;

// Although up to 4GB, 64KB is enough with less mem reserved

// DMA is only enabled for large data transmission

// Maximum clock values from datasheet suggest keeping clock value under
// 150MHz. No minimum or average value is suggested.
//

pub const ROCKCHIP_AUTOSUSPEND_DELAY: c_int = 2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_sfc {
    pub dev: *mut device,
    pub regbase: *mut void __iomem,
    pub hclk: *mut clk,
    pub clk: *mut clk,
    pub speed: [u32; SFC_MAX_CHIPSELECT_NUM],
// virtual mapped addr for dma_buffer
    pub buffer: *mut c_void,
    pub dma_buffer: dma_addr_t,
    pub cp: completion,
    pub use_dma: bool,
    pub max_iosize: u32,
    pub version: u16,
    pub host: *mut spi_controller,
}

#[no_mangle]
unsafe extern "C" fn rockchip_sfc_reset(sfc: *mut rockchip_sfc) -> c_int {
    static int rockchip_sfc_reset(struct rockchip_sfc *sfc)
    {
    int err;
    u32 status;
    writel_relaxed(SFC_RCVR_RESET, sfc.regbase + SFC_RCVR);
    err = readl_poll_timeout(sfc.regbase + SFC_RCVR, status,
    !(status & SFC_RCVR_RESET), 20,
    jiffies_to_usecs(HZ));
    if (err)
    dev_err(sfc.dev, "SFC reset never finished\n");
// Still need to clear the masked interrupt from RISR
    writel_relaxed(0xFFFFFFFF, sfc.regbase + SFC_ICLR);
    dev_dbg(sfc.dev, "reset\n");
    return err;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_get_version(sfc: *mut rockchip_sfc) -> u16 {
    static u16 rockchip_sfc_get_version(struct rockchip_sfc *sfc)
    {
    return  (u16)(readl(sfc.regbase + SFC_VER) & 0xffff);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_get_max_iosize(sfc: *mut rockchip_sfc) -> u32 {
    static u32 rockchip_sfc_get_max_iosize(struct rockchip_sfc *sfc)
    {
    return SFC_MAX_IOSIZE_VER3;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_clk_set_rate(sfc: *mut rockchip_sfc, speed: c_ulong) -> c_int {
    static int rockchip_sfc_clk_set_rate(struct rockchip_sfc *sfc, unsigned long  speed)
    {
    if (sfc.version >= SFC_VER_8)
    return clk_set_rate(sfc.clk, speed * 2);
    else
    return clk_set_rate(sfc.clk, speed);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_clk_get_rate(sfc: *mut rockchip_sfc) -> c_ulong {
    static unsigned long rockchip_sfc_clk_get_rate(struct rockchip_sfc *sfc)
    {
    if (sfc.version >= SFC_VER_8)
    return clk_get_rate(sfc.clk) / 2;
    else
    return clk_get_rate(sfc.clk);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_irq_unmask(sfc: *mut rockchip_sfc, mask: u32) {
    static void rockchip_sfc_irq_unmask(struct rockchip_sfc *sfc, u32 mask)
    {
    u32 reg;
// Enable transfer complete interrupt
    reg = readl(sfc.regbase + SFC_IMR);
    reg &= ~mask;
    writel(reg, sfc.regbase + SFC_IMR);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_irq_mask(sfc: *mut rockchip_sfc, mask: u32) {
    static void rockchip_sfc_irq_mask(struct rockchip_sfc *sfc, u32 mask)
    {
    u32 reg;
// Disable transfer finish interrupt
    reg = readl(sfc.regbase + SFC_IMR);
    reg |= mask;
    writel(reg, sfc.regbase + SFC_IMR);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_init(sfc: *mut rockchip_sfc) -> c_int {
    static int rockchip_sfc_init(struct rockchip_sfc *sfc)
    {
    writel(0, sfc.regbase + SFC_CTRL);
    writel(0xFFFFFFFF, sfc.regbase + SFC_ICLR);
    rockchip_sfc_irq_mask(sfc, 0xFFFFFFFF);
    if (rockchip_sfc_get_version(sfc) >= SFC_VER_4)
    writel(SFC_LEN_CTRL_TRB_SEL, sfc.regbase + SFC_LEN_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_wait_txfifo_ready(sfc: *mut rockchip_sfc, timeout_us: u32) -> c_int {
    static int rockchip_sfc_wait_txfifo_ready(struct rockchip_sfc *sfc, u32 timeout_us)
    {
    let mut ret: c_int = 0;
    u32 status;
    ret = readl_poll_timeout(sfc.regbase + SFC_FSR, status,
    status & SFC_FSR_TXLV_MASK, 0,
    timeout_us);
    if (ret) {
    dev_dbg(sfc.dev, "sfc wait tx fifo timeout\n");
    return -ETIMEDOUT;
    }
    return (status & SFC_FSR_TXLV_MASK) >> SFC_FSR_TXLV_SHIFT;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_wait_rxfifo_ready(sfc: *mut rockchip_sfc, timeout_us: u32) -> c_int {
    static int rockchip_sfc_wait_rxfifo_ready(struct rockchip_sfc *sfc, u32 timeout_us)
    {
    let mut ret: c_int = 0;
    u32 status;
    ret = readl_poll_timeout(sfc.regbase + SFC_FSR, status,
    status & SFC_FSR_RXLV_MASK, 0,
    timeout_us);
    if (ret) {
    dev_dbg(sfc.dev, "sfc wait rx fifo timeout\n");
    return -ETIMEDOUT;
    }
    return (status & SFC_FSR_RXLV_MASK) >> SFC_FSR_RXLV_SHIFT;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_adjust_op_work(op: *mut spi_mem_op) {
    static void rockchip_sfc_adjust_op_work(struct spi_mem_op *op)
    {
    if (unlikely(op.dummy.nbytes && !op.addr.nbytes)) {
//
// SFC not support output DUMMY cycles right after CMD cycles, so
// treat it as ADDR cycles.
//
    op.addr.nbytes = op.dummy.nbytes;
    op.addr.buswidth = op.dummy.buswidth;
    op.addr.val = 0xFFFFFFFFF;
    op.dummy.nbytes = 0;
    }
    }
    static int rockchip_sfc_xfer_setup(struct rockchip_sfc *sfc,
    struct spi_mem *mem,
    const struct spi_mem_op *op,
    u32 len)
    {
    let mut ctrl: u32 = 0, cmd = 0;
    let mut cs: u8 = spi_get_chipselect(mem.spi, 0);
// set CMD
    cmd = op.cmd.opcode;
    ctrl |= ((op.cmd.buswidth >> 1) << SFC_CTRL_CMD_BITS_SHIFT);
// set ADDR
    if (op.addr.nbytes) {
    if (op.addr.nbytes == 4) {
    cmd |= SFC_CMD_ADDR_32BITS << SFC_CMD_ADDR_SHIFT;
    } else if (op.addr.nbytes == 3) {
    cmd |= SFC_CMD_ADDR_24BITS << SFC_CMD_ADDR_SHIFT;
    } else {
    cmd |= SFC_CMD_ADDR_XBITS << SFC_CMD_ADDR_SHIFT;
    writel(op.addr.nbytes * 8 - 1,
    sfc.regbase + cs * SFC_CS1_REG_OFFSET + SFC_ABIT);
    }
    ctrl |= ((op.addr.buswidth >> 1) << SFC_CTRL_ADDR_BITS_SHIFT);
    }
// set DUMMY
    if (op.dummy.nbytes) {
    if (op.dummy.buswidth == 4)
    cmd |= op.dummy.nbytes * 2 << SFC_CMD_DUMMY_SHIFT;
#[no_mangle]
pub unsafe extern "C" fn if(2: op->dummy.buswidth ==) -> else {
    else if (op.dummy.buswidth == 2)
    cmd |= op.dummy.nbytes * 4 << SFC_CMD_DUMMY_SHIFT;
    else
    cmd |= op.dummy.nbytes * 8 << SFC_CMD_DUMMY_SHIFT;
    }
// set DATA
    if (sfc.version >= SFC_VER_4) /* Clear it if no data to transfer */
    writel(len, sfc.regbase + SFC_LEN_EXT);
    else
    cmd |= len << SFC_CMD_TRAN_BYTES_SHIFT;
    if (len) {
    if (op.data.dir == SPI_MEM_DATA_OUT)
    cmd |= SFC_CMD_DIR_WR << SFC_CMD_DIR_SHIFT;
    ctrl |= ((op.data.buswidth >> 1) << SFC_CTRL_DATA_BITS_SHIFT);
    }
    if (!len && op.addr.nbytes)
    cmd |= SFC_CMD_DIR_WR << SFC_CMD_DIR_SHIFT;
// set the Controller
    ctrl |= SFC_CTRL_PHASE_SEL_NEGETIVE;
    cmd |= cs << SFC_CMD_CS_SHIFT;
    dev_dbg(sfc.dev, "sfc addr.nbytes=%x(x%d) dummy.nbytes=%x(x%d)\n",
    op.addr.nbytes, op.addr.buswidth,
    op.dummy.nbytes, op.dummy.buswidth);
    dev_dbg(sfc.dev, "sfc ctrl=%x cmd=%x addr=%llx len=%x\n",
    ctrl, cmd, op.addr.val, len);
    writel(ctrl, sfc.regbase + cs * SFC_CS1_REG_OFFSET + SFC_CTRL);
    writel(cmd, sfc.regbase + SFC_CMD);
    if (op.addr.nbytes)
    writel(op.addr.val, sfc.regbase + SFC_ADDR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_write_fifo(sfc: *mut rockchip_sfc, buf: *const u8, len: c_int) -> c_int {
    static int rockchip_sfc_write_fifo(struct rockchip_sfc *sfc, const u8 *buf, int len)
    {
    let mut bytes: u8 = len & 0x3;
    u32 dwords;
    int tx_level;
    u32 write_words;
    let mut tmp: u32 = 0;
    dwords = len >> 2;
    while (dwords) {
    tx_level = rockchip_sfc_wait_txfifo_ready(sfc, 1000);
    if (tx_level < 0)
    return tx_level;
    write_words = min_t(u32, tx_level, dwords);
    iowrite32_rep(sfc.regbase + SFC_DATA, buf, write_words);
    buf += write_words << 2;
    dwords -= write_words;
    }
// write the rest non word aligned bytes
    if (bytes) {
    tx_level = rockchip_sfc_wait_txfifo_ready(sfc, 1000);
    if (tx_level < 0)
    return tx_level;
    memcpy(&tmp, buf, bytes);
    writel(tmp, sfc.regbase + SFC_DATA);
    }
    return len;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_read_fifo(sfc: *mut rockchip_sfc, buf: *mut u8, len: c_int) -> c_int {
    static int rockchip_sfc_read_fifo(struct rockchip_sfc *sfc, u8 *buf, int len)
    {
    let mut bytes: u8 = len & 0x3;
    u32 dwords;
    u8 read_words;
    int rx_level;
    int tmp;
// word aligned access only
    dwords = len >> 2;
    while (dwords) {
    rx_level = rockchip_sfc_wait_rxfifo_ready(sfc, 1000);
    if (rx_level < 0)
    return rx_level;
    read_words = min_t(u32, rx_level, dwords);
    ioread32_rep(sfc.regbase + SFC_DATA, buf, read_words);
    buf += read_words << 2;
    dwords -= read_words;
    }
// read the rest non word aligned bytes
    if (bytes) {
    rx_level = rockchip_sfc_wait_rxfifo_ready(sfc, 1000);
    if (rx_level < 0)
    return rx_level;
    tmp = readl(sfc.regbase + SFC_DATA);
    memcpy(buf, &tmp, bytes);
    }
    return len;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_fifo_transfer_dma(sfc: *mut rockchip_sfc, dma_buf: dma_addr_t, len: usize) -> c_int {
    static int rockchip_sfc_fifo_transfer_dma(struct rockchip_sfc *sfc, dma_addr_t dma_buf, size_t len)
    {
    writel(0xFFFFFFFF, sfc.regbase + SFC_ICLR);
    writel((u32)dma_buf, sfc.regbase + SFC_DMA_ADDR);
    writel(SFC_DMA_TRIGGER_START, sfc.regbase + SFC_DMA_TRIGGER);
    return len;
    }
    static int rockchip_sfc_xfer_data_poll(struct rockchip_sfc *sfc,
    const struct spi_mem_op *op, u32 len)
    {
    dev_dbg(sfc.dev, "sfc xfer_poll len=%x\n", len);
    if (op.data.dir == SPI_MEM_DATA_OUT)
    return rockchip_sfc_write_fifo(sfc, op.data.buf.out, len);
    else
    return rockchip_sfc_read_fifo(sfc, op.data.buf.in, len);
    }
    static int rockchip_sfc_xfer_data_dma(struct rockchip_sfc *sfc,
    const struct spi_mem_op *op, u32 len)
    {
    int ret;
    dev_dbg(sfc.dev, "sfc xfer_dma len=%x\n", len);
    if (op.data.dir == SPI_MEM_DATA_OUT) {
    memcpy(sfc.buffer, op.data.buf.out, len);
    dma_sync_single_for_device(sfc.dev, sfc.dma_buffer, len, DMA_TO_DEVICE);
    }
    ret = rockchip_sfc_fifo_transfer_dma(sfc, sfc.dma_buffer, len);
    if (!wait_for_completion_timeout(&sfc.cp, msecs_to_jiffies(2000))) {
    dev_err(sfc.dev, "DMA wait for transfer finish timeout\n");
    ret = -ETIMEDOUT;
    }
    rockchip_sfc_irq_mask(sfc, SFC_IMR_DMA);
    if (op.data.dir == SPI_MEM_DATA_IN) {
    dma_sync_single_for_cpu(sfc.dev, sfc.dma_buffer, len, DMA_FROM_DEVICE);
    memcpy(op.data.buf.in, sfc.buffer, len);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_xfer_done(sfc: *mut rockchip_sfc, timeout_us: u32) -> c_int {
    static int rockchip_sfc_xfer_done(struct rockchip_sfc *sfc, u32 timeout_us)
    {
    let mut ret: c_int = 0;
    u32 status;
//
// There is very little data left in fifo, and the controller will
// complete the transmission in a short period of time.
//
    ret = readl_poll_timeout(sfc.regbase + SFC_SR, status,
    !(status & SFC_SR_IS_BUSY),
    0, 10);
    if (!ret)
    return 0;
    ret = readl_poll_timeout(sfc.regbase + SFC_SR, status,
    !(status & SFC_SR_IS_BUSY),
    20, timeout_us);
    if (ret) {
    dev_err(sfc.dev, "wait sfc idle timeout\n");
    rockchip_sfc_reset(sfc);
    ret = -EIO;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_exec_mem_op(mem: *mut spi_mem, op: *const spi_mem_op) -> c_int {
    static int rockchip_sfc_exec_mem_op(struct spi_mem *mem, const struct spi_mem_op *op)
    {
    struct rockchip_sfc *sfc = spi_controller_get_devdata(mem.spi.controller);
    let mut len: u32 = op.data.nbytes;
    int ret;
    let mut cs: u8 = spi_get_chipselect(mem.spi, 0);
    ret = pm_runtime_get_sync(sfc.dev);
    if (ret < 0) {
    pm_runtime_put_noidle(sfc.dev);
    return ret;
    }
    if (unlikely(op.max_freq != sfc.speed[cs]) &&
    !has_acpi_companion(sfc.dev)) {
    ret = rockchip_sfc_clk_set_rate(sfc, op.max_freq);
    if (ret)
    goto out;
    sfc.speed[cs] = op.max_freq;
    dev_dbg(sfc.dev, "set_freq=%dHz real_freq=%ldHz\n",
    sfc.speed[cs], rockchip_sfc_clk_get_rate(sfc));
    }
    rockchip_sfc_adjust_op_work((struct spi_mem_op *)op);
    rockchip_sfc_xfer_setup(sfc, mem, op, len);
    if (len) {
    if (likely(sfc.use_dma) && len >= SFC_DMA_TRANS_THRETHOLD && !(len & 0x3)) {
    init_completion(&sfc.cp);
    rockchip_sfc_irq_unmask(sfc, SFC_IMR_DMA);
    ret = rockchip_sfc_xfer_data_dma(sfc, op, len);
    } else {
    ret = rockchip_sfc_xfer_data_poll(sfc, op, len);
    }
    if (ret != len) {
    dev_err(sfc.dev, "xfer data failed ret %d dir %d\n", ret, op.data.dir);
    ret = -EIO;
    goto out;
    }
    }
    ret = rockchip_sfc_xfer_done(sfc, 100000);
    out:
    pm_runtime_put_autosuspend(sfc.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_adjust_op_size(mem: *mut spi_mem, op: *mut spi_mem_op) -> c_int {
    static int rockchip_sfc_adjust_op_size(struct spi_mem *mem, struct spi_mem_op *op)
    {
    struct rockchip_sfc *sfc = spi_controller_get_devdata(mem.spi.controller);
    op.data.nbytes = min(op.data.nbytes, sfc.max_iosize);
    return 0;
    }
    static const struct spi_controller_mem_ops rockchip_sfc_mem_ops = {
    .exec_op = rockchip_sfc_exec_mem_op,
    .adjust_op_size = rockchip_sfc_adjust_op_size,
    };
    static const struct spi_controller_mem_caps rockchip_sfc_mem_caps = {
    .per_op_freq = true,
    };
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t rockchip_sfc_irq_handler(int irq, void *dev_id)
    {
    struct rockchip_sfc *sfc = dev_id;
    u32 reg;
    reg = readl(sfc.regbase + SFC_RISR);
// Clear interrupt
    writel_relaxed(reg, sfc.regbase + SFC_ICLR);
    if (reg & SFC_RISR_DMA) {
    complete(&sfc.cp);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_probe(pdev: *mut platform_device) -> c_int {
    static int rockchip_sfc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct spi_controller *host;
    struct rockchip_sfc *sfc;
    int ret;
    u32 i, val;
    host = devm_spi_alloc_host(&pdev.dev, sizeof(*sfc));
    if (!host)
    return -ENOMEM;
    host.flags = SPI_CONTROLLER_HALF_DUPLEX;
    host.mem_ops = &rockchip_sfc_mem_ops;
    host.mem_caps = &rockchip_sfc_mem_caps;
    host.mode_bits = SPI_TX_QUAD | SPI_TX_DUAL | SPI_RX_QUAD | SPI_RX_DUAL;
    host.max_speed_hz = SFC_MAX_SPEED;
    host.num_chipselect = SFC_MAX_CHIPSELECT_NUM;
    sfc = spi_controller_get_devdata(host);
    sfc.dev = dev;
    sfc.host = host;
    sfc.regbase = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(sfc.regbase))
    return PTR_ERR(sfc.regbase);
    if (!has_acpi_companion(&pdev.dev))
    sfc.clk = devm_clk_get(&pdev.dev, "clk_sfc");
    if (IS_ERR(sfc.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(sfc.clk),
    "Failed to get sfc interface clk\n");
    if (!has_acpi_companion(&pdev.dev))
    sfc.hclk = devm_clk_get(&pdev.dev, "hclk_sfc");
    if (IS_ERR(sfc.hclk))
    return dev_err_probe(&pdev.dev, PTR_ERR(sfc.hclk),
    "Failed to get sfc ahb clk\n");
    if (has_acpi_companion(&pdev.dev)) {
    ret = device_property_read_u32(&pdev.dev, "clock-frequency", &val);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "Failed to find clock-frequency in ACPI\n");
    for (i = 0; i < SFC_MAX_CHIPSELECT_NUM; i++)
    sfc.speed[i] = val;
    }
    sfc.use_dma = !of_property_read_bool(sfc.dev.of_node, "rockchip,sfc-no-dma");
    ret = clk_prepare_enable(sfc.hclk);
    if (ret) {
    dev_err(&pdev.dev, "Failed to enable ahb clk\n");
    goto err_hclk;
    }
    ret = clk_prepare_enable(sfc.clk);
    if (ret) {
    dev_err(&pdev.dev, "Failed to enable interface clk\n");
    goto err_clk;
    }
// Find the irq
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    goto err_irq;
    ret = devm_request_irq(dev, ret, rockchip_sfc_irq_handler,
    0, pdev.name, sfc);
    if (ret) {
    dev_err(dev, "Failed to request irq\n");
    goto err_irq;
    }
    platform_set_drvdata(pdev, sfc);
    ret = rockchip_sfc_init(sfc);
    if (ret)
    goto err_irq;
    sfc.version = rockchip_sfc_get_version(sfc);
    sfc.max_iosize = rockchip_sfc_get_max_iosize(sfc);
    pm_runtime_set_autosuspend_delay(dev, ROCKCHIP_AUTOSUSPEND_DELAY);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    pm_runtime_get_noresume(dev);
    if (sfc.use_dma) {
    sfc.buffer = (u8 *)__get_free_pages(GFP_KERNEL | GFP_DMA32,
    get_order(sfc.max_iosize));
    if (!sfc.buffer) {
    ret = -ENOMEM;
    goto err_dma;
    }
    sfc.dma_buffer = dma_map_single(dev, sfc.buffer,
    sfc.max_iosize, DMA_BIDIRECTIONAL);
    if (dma_mapping_error(dev, sfc.dma_buffer)) {
    ret = -ENOMEM;
    goto err_dma_map;
    }
    }
    ret = spi_register_controller(host);
    if (ret)
    goto err_register;
    pm_runtime_put_autosuspend(dev);
    return 0;
    err_register:
    dma_unmap_single(dev, sfc.dma_buffer, sfc.max_iosize,
    DMA_BIDIRECTIONAL);
    err_dma_map:
    free_pages((unsigned long)sfc.buffer, get_order(sfc.max_iosize));
    err_dma:
    pm_runtime_get_sync(dev);
    pm_runtime_put_noidle(dev);
    pm_runtime_disable(dev);
    pm_runtime_set_suspended(dev);
    pm_runtime_dont_use_autosuspend(dev);
    err_irq:
    clk_disable_unprepare(sfc.clk);
    err_clk:
    clk_disable_unprepare(sfc.hclk);
    err_hclk:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_remove(pdev: *mut platform_device) {
    static void rockchip_sfc_remove(struct platform_device *pdev)
    {
    struct rockchip_sfc *sfc = platform_get_drvdata(pdev);
    struct spi_controller *host = sfc.host;
    spi_unregister_controller(host);
    dma_unmap_single(&pdev.dev, sfc.dma_buffer, sfc.max_iosize,
    DMA_BIDIRECTIONAL);
    free_pages((unsigned long)sfc.buffer, get_order(sfc.max_iosize));
    clk_disable_unprepare(sfc.clk);
    clk_disable_unprepare(sfc.hclk);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_runtime_suspend(dev: *mut device) -> c_int {
    static int rockchip_sfc_runtime_suspend(struct device *dev)
    {
    struct rockchip_sfc *sfc = dev_get_drvdata(dev);
    clk_disable_unprepare(sfc.clk);
    clk_disable_unprepare(sfc.hclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_runtime_resume(dev: *mut device) -> c_int {
    static int rockchip_sfc_runtime_resume(struct device *dev)
    {
    struct rockchip_sfc *sfc = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(sfc.hclk);
    if (ret < 0)
    return ret;
    ret = clk_prepare_enable(sfc.clk);
    if (ret < 0)
    clk_disable_unprepare(sfc.hclk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_suspend(dev: *mut device) -> c_int {
    static int rockchip_sfc_suspend(struct device *dev)
    {
    pinctrl_pm_select_sleep_state(dev);
    return pm_runtime_force_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_sfc_resume(dev: *mut device) -> c_int {
    static int rockchip_sfc_resume(struct device *dev)
    {
    struct rockchip_sfc *sfc = dev_get_drvdata(dev);
    int ret;
    ret = pm_runtime_force_resume(dev);
    if (ret < 0)
    return ret;
    pinctrl_pm_select_default_state(dev);
    ret = pm_runtime_get_sync(dev);
    if (ret < 0) {
    pm_runtime_put_noidle(dev);
    return ret;
    }
    rockchip_sfc_init(sfc);
    pm_runtime_put_autosuspend(dev);
    return 0;
    }
    static const struct dev_pm_ops rockchip_sfc_pm_ops = {
    RUNTIME_PM_OPS(rockchip_sfc_runtime_suspend,
    rockchip_sfc_runtime_resume, core::ptr::null_mut())
    SYSTEM_SLEEP_PM_OPS(rockchip_sfc_suspend, rockchip_sfc_resume)
    };
    static const struct of_device_id rockchip_sfc_dt_ids[] = {
    { .compatible = "rockchip,sfc"},
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rockchip_sfc_dt_ids);
    static struct platform_driver rockchip_sfc_driver = {
    .driver = {
    .name	= "rockchip-sfc",
    .of_match_table = rockchip_sfc_dt_ids,
    .pm = pm_ptr(&rockchip_sfc_pm_ops),
    },
    .probe	= rockchip_sfc_probe,
    .remove = rockchip_sfc_remove,
    };
    module_platform_driver(rockchip_sfc_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Rockchip Serial Flash Controller Driver");
    MODULE_AUTHOR("Shawn Lin <shawn.lin@rock-chips.com>");
    MODULE_AUTHOR("Chris Morgan <macromorgan@hotmail.com>");
    MODULE_AUTHOR("Jon Lin <Jon.lin@rock-chips.com>");
