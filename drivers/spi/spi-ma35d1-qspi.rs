//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-ma35d1-qspi.c
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
// Nuvoton MA35D1 QSPI controller driver
//
// Copyright (c) 2026 Nuvoton Technology Corp.
// Author: Chi-Wen Weng <cwweng@nuvoton.com>

// Register offset definitions
pub const NUVOTON_QSPI_CTL_OFFSET: c_uint = 0x00 /* Control Register, RW */;
pub const NUVOTON_QSPI_CLKDIV_OFFSET: c_uint = 0x04 /* Clock Divider Register, RW */;
pub const NUVOTON_QSPI_SSCTL_OFFSET: c_uint = 0x08 /* Slave Select Register, RW */;
pub const NUVOTON_QSPI_FIFOCTL_OFFSET: c_uint = 0x10 /* FIFO Control Register, RW */;
pub const NUVOTON_QSPI_STATUS_OFFSET: c_uint = 0x14 /* Status Register, RW */;
pub const NUVOTON_QSPI_TX_OFFSET: c_uint = 0x20 /* Data Transmit Register, WO */;
pub const NUVOTON_QSPI_RX_OFFSET: c_uint = 0x30 /* Data Receive Register, RO */;
// QSPI Control Register bit masks

// QSPI Clock Divider Register bit masks

// QSPI Slave Select Control Register bit masks

// QSPI FIFO Control Register bit masks

// QSPI Status Register bit masks

pub const NUVOTON_QSPI_MAX_NUM_CS: c_int = 2;
pub const NUVOTON_QSPI_DEFAULT_NUM_CS: c_int = 2;
pub const NUVOTON_QSPI_DEFAULT_BPW: c_int = 8;
// Bound PIO operations to avoid long atomic polling loops.

pub const NUVOTON_QSPI_TIMEOUT_US: c_int = 10000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nuvoton_qspi {
    pub regs: *mut void __iomem,
    pub clk: *mut clk,
    pub dev: *mut device,
// Protects read-modify-write accesses to the SSCTL register.
    pub ssctl_lock: spinlock_t,
    pub speed_hz: u32,
}

#[no_mangle]
unsafe extern "C" fn nuvoton_qspi_read(qspi: *mut nuvoton_qspi, reg: u32) -> u32 {
    static u32 nuvoton_qspi_read(struct nuvoton_qspi *qspi, u32 reg)
    {
    return readl(qspi.regs + reg);
    }
#[no_mangle]
unsafe extern "C" fn nuvoton_qspi_write(qspi: *mut nuvoton_qspi, val: u32, reg: u32) {
    static void nuvoton_qspi_write(struct nuvoton_qspi *qspi, u32 val, u32 reg)
    {
    writel(val, qspi.regs + reg);
    }
    static void nuvoton_qspi_update_bits(struct nuvoton_qspi *qspi, u32 reg,
    u32 mask, u32 val)
    {
    u32 tmp;
    tmp = nuvoton_qspi_read(qspi, reg);
    tmp &= ~mask;
    tmp |= val & mask;
    nuvoton_qspi_write(qspi, tmp, reg);
    }
#[no_mangle]
unsafe extern "C" fn nuvoton_qspi_wait_ready(qspi: *mut nuvoton_qspi) -> c_int {
    static int nuvoton_qspi_wait_ready(struct nuvoton_qspi *qspi)
    {
    u32 val;
    return readl_poll_timeout(qspi.regs + NUVOTON_QSPI_STATUS_OFFSET,
    val,
    !(val & NUVOTON_QSPI_STATUS_BUSY_MASK),
    0, NUVOTON_QSPI_TIMEOUT_US);
    }
#[no_mangle]
unsafe extern "C" fn nuvoton_qspi_reset_fifo(qspi: *mut nuvoton_qspi) -> c_int {
    static int nuvoton_qspi_reset_fifo(struct nuvoton_qspi *qspi)
    {
    u32 val;
    nuvoton_qspi_update_bits(qspi, NUVOTON_QSPI_FIFOCTL_OFFSET,
    NUVOTON_QSPI_FIFOCTL_TXRST_MASK |
    NUVOTON_QSPI_FIFOCTL_RXRST_MASK,
    NUVOTON_QSPI_FIFOCTL_TXRST_MASK |
    NUVOTON_QSPI_FIFOCTL_RXRST_MASK);
//
// Give the controller a short time to latch the FIFO reset request
// before polling the reset status bit.
//
    udelay(1);
    return readl_poll_timeout_atomic(qspi.regs + NUVOTON_QSPI_STATUS_OFFSET,
    val,
    !(val & NUVOTON_QSPI_STATUS_TXRXRST_MASK),
    1, NUVOTON_QSPI_TIMEOUT_US);
    }
#[no_mangle]
unsafe extern "C" fn nuvoton_qspi_set_speed(spi: *mut spi_device, speed_hz: u32, dtr: bool) -> c_int {
    static int nuvoton_qspi_set_speed(struct spi_device *spi, u32 speed_hz, bool dtr)
    {
    struct nuvoton_qspi *qspi = spi_controller_get_devdata(spi.controller);
    unsigned long clk_rate;
    u32 div;
    if (!speed_hz)
    speed_hz = spi.max_speed_hz;
    if (!speed_hz)
    return -EINVAL;
// Experimentally, when enabling DTR the frequency is cut in half
    if (dtr)
    speed_hz *= 2;
    if (qspi.speed_hz == speed_hz)
    return 0;
    clk_rate = clk_get_rate(qspi.clk);
    if (!clk_rate) {
    dev_err(qspi.dev, "failed to get clock rate\n");
    return -EINVAL;
    }
    div = DIV_ROUND_UP(clk_rate, speed_hz) - 1;
    if (div > FIELD_MAX(NUVOTON_QSPI_CLKDIV_MASK)) {
    dev_err(qspi.dev, "unsupported SPI clock %u Hz\n", speed_hz);
    return -EINVAL;
    }
    nuvoton_qspi_write(qspi, FIELD_PREP(NUVOTON_QSPI_CLKDIV_MASK, div),
    NUVOTON_QSPI_CLKDIV_OFFSET);
    qspi.speed_hz = speed_hz;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nuvoton_qspi_set_bits_per_word(qspi: *mut nuvoton_qspi, bpw: u8) -> c_int {
    static int nuvoton_qspi_set_bits_per_word(struct nuvoton_qspi *qspi, u8 bpw)
    {
    if (bpw != NUVOTON_QSPI_DEFAULT_BPW)
    return -EINVAL;
    nuvoton_qspi_update_bits(qspi, NUVOTON_QSPI_CTL_OFFSET,
    NUVOTON_QSPI_CTL_DWIDTH_MASK |
    NUVOTON_QSPI_CTL_REORDER_MASK,
    FIELD_PREP(NUVOTON_QSPI_CTL_DWIDTH_MASK, bpw));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nuvoton_qspi_setup_transfer(spi: *mut spi_device, bpw: u8) -> c_int {
    static int nuvoton_qspi_setup_transfer(struct spi_device *spi, u8 bpw)
    {
    struct nuvoton_qspi *qspi = spi_controller_get_devdata(spi.controller);
    let mut mode: u32 = spi.mode & SPI_MODE_X_MASK;
    let mut ctl: u32 = 0;
    int ret;
    if (!bpw)
    bpw = NUVOTON_QSPI_DEFAULT_BPW;
    ret = nuvoton_qspi_set_bits_per_word(qspi, bpw);
    if (ret)
    return ret;
    if (mode == SPI_MODE_0 || mode == SPI_MODE_3)
    ctl |= NUVOTON_QSPI_CTL_TXNEG_MASK;
    else
    ctl |= NUVOTON_QSPI_CTL_RXNEG_MASK;
    if (spi.mode & SPI_CPOL)
    ctl |= NUVOTON_QSPI_CTL_CLKPOL_MASK;
    if (spi.mode & SPI_LSB_FIRST)
    ctl |= NUVOTON_QSPI_CTL_LSB_MASK;
    nuvoton_qspi_update_bits(qspi, NUVOTON_QSPI_CTL_OFFSET,
    NUVOTON_QSPI_CTL_TXNEG_MASK |
    NUVOTON_QSPI_CTL_RXNEG_MASK |
    NUVOTON_QSPI_CTL_CLKPOL_MASK |
    NUVOTON_QSPI_CTL_LSB_MASK, ctl);
    return 0;
    }
    static int nuvoton_qspi_configure_bus(struct spi_device *spi,
    unsigned int buswidth,
    enum spi_mem_data_dir dir,
    u32 speed_hz, bool dtr)
    {
    struct nuvoton_qspi *qspi = spi_controller_get_devdata(spi.controller);
    let mut ctl: u32 = 0;
    int ret;
    ret = nuvoton_qspi_set_speed(spi, speed_hz, dtr);
    if (ret)
    return ret;
    if (dtr)
    ctl |= NUVOTON_QSPI_CTL_DTREN_MASK;
    if (buswidth == 4)
    ctl |= NUVOTON_QSPI_CTL_QUADIOEN_MASK;
#[no_mangle]
pub unsafe extern "C" fn if(2: buswidth ==) -> else {
    else if (buswidth == 2)
    ctl |= NUVOTON_QSPI_CTL_DUALIOEN_MASK;
    if (buswidth > 1 && dir == SPI_MEM_DATA_OUT)
    ctl |= NUVOTON_QSPI_CTL_DATDIR_MASK;
    nuvoton_qspi_update_bits(qspi, NUVOTON_QSPI_CTL_OFFSET,
    NUVOTON_QSPI_CTL_DTREN_MASK |
    NUVOTON_QSPI_CTL_QUADIOEN_MASK |
    NUVOTON_QSPI_CTL_DUALIOEN_MASK |
    NUVOTON_QSPI_CTL_DATDIR_MASK, ctl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nuvoton_qspi_tx_byte(txbuf: *const c_void, idx: c_uint) -> u32 {
    static u32 nuvoton_qspi_tx_byte(const void *txbuf, unsigned int idx)
    {
    if (!txbuf)
    return 0;
    return ((const u8 *)txbuf)[idx];
    }
#[no_mangle]
unsafe extern "C" fn nuvoton_qspi_rx_byte(rxbuf: *mut c_void, idx: c_uint, val: u32) {
    static void nuvoton_qspi_rx_byte(void *rxbuf, unsigned int idx, u32 val)
    {
    if (rxbuf)
    ((u8 *)rxbuf)[idx] = val;
    }
#[no_mangle]
unsafe extern "C" fn nuvoton_qspi_wait_tx_not_full(qspi: *mut nuvoton_qspi) -> c_int {
    static int nuvoton_qspi_wait_tx_not_full(struct nuvoton_qspi *qspi)
    {
    u32 val;
    return readl_poll_timeout_atomic(qspi.regs + NUVOTON_QSPI_STATUS_OFFSET,
    val,
    !(val & NUVOTON_QSPI_STATUS_TXFULL_MASK),
    0, NUVOTON_QSPI_TIMEOUT_US);
    }
#[no_mangle]
unsafe extern "C" fn nuvoton_qspi_wait_rx_not_empty(qspi: *mut nuvoton_qspi) -> c_int {
    static int nuvoton_qspi_wait_rx_not_empty(struct nuvoton_qspi *qspi)
    {
    u32 val;
    return readl_poll_timeout_atomic(qspi.regs + NUVOTON_QSPI_STATUS_OFFSET,
    val,
    !(val & NUVOTON_QSPI_STATUS_RXEMPTY_MASK),
    0, NUVOTON_QSPI_TIMEOUT_US);
    }
    static int nuvoton_qspi_txrx(struct nuvoton_qspi *qspi, const void *txbuf,
    void *rxbuf, unsigned int len)
    {
    unsigned int i;
    u32 val;
    int ret;
    if (!len)
    return 0;
    if (len > NUVOTON_QSPI_MAX_TRANSFER_SIZE)
    return -EMSGSIZE;
    ret = nuvoton_qspi_reset_fifo(qspi);
    if (ret) {
    dev_err(qspi.dev, "FIFO reset timed out\n");
    return ret;
    }
//
// Use conservative byte-by-byte PIO access. This keeps the initial driver
// simple and avoids relying on FIFO threshold interrupts or DMA support.
//
// The MA35D1 QSPI controller pushes one RX FIFO entry for each TX byte in
// single, dual-output and quad-output modes. Drain RX after every TX byte
// and discard the value for TX-only transfers to avoid RX FIFO overflow.
//
    for (i = 0; i < len; i++) {
    ret = nuvoton_qspi_wait_tx_not_full(qspi);
    if (ret) {
    dev_err(qspi.dev, "TX FIFO full timeout\n");
    return ret;
    }
    nuvoton_qspi_write(qspi, nuvoton_qspi_tx_byte(txbuf, i),
    NUVOTON_QSPI_TX_OFFSET);
    ret = nuvoton_qspi_wait_rx_not_empty(qspi);
    if (ret) {
    dev_err(qspi.dev, "RX FIFO empty timeout\n");
    return ret;
    }
    val = nuvoton_qspi_read(qspi, NUVOTON_QSPI_RX_OFFSET);
    if (rxbuf)
    nuvoton_qspi_rx_byte(rxbuf, i, val);
    }
    ret = nuvoton_qspi_wait_ready(qspi);
    if (ret)
    dev_err(qspi.dev, "controller busy timeout\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nuvoton_qspi_hw_init(qspi: *mut nuvoton_qspi) -> c_int {
    static int nuvoton_qspi_hw_init(struct nuvoton_qspi *qspi)
    {
    u32 val;
    int ret;
    ret = nuvoton_qspi_set_bits_per_word(qspi, NUVOTON_QSPI_DEFAULT_BPW);
    if (ret)
    return ret;
    nuvoton_qspi_update_bits(qspi, NUVOTON_QSPI_CTL_OFFSET,
    NUVOTON_QSPI_CTL_SUSPITV_MASK |
    NUVOTON_QSPI_CTL_TXNEG_MASK |
    NUVOTON_QSPI_CTL_RXNEG_MASK |
    NUVOTON_QSPI_CTL_CLKPOL_MASK |
    NUVOTON_QSPI_CTL_LSB_MASK,
    NUVOTON_QSPI_CTL_TXNEG_MASK);
    nuvoton_qspi_update_bits(qspi, NUVOTON_QSPI_CTL_OFFSET,
    NUVOTON_QSPI_CTL_SPIEN_MASK,
    NUVOTON_QSPI_CTL_SPIEN_MASK);
    ret = readl_poll_timeout(qspi.regs + NUVOTON_QSPI_STATUS_OFFSET, val,
    (val & NUVOTON_QSPI_STATUS_SPIENSTS_MASK),
    1, NUVOTON_QSPI_TIMEOUT_US);
    if (ret) {
    dev_err(qspi.dev, "failed to enable controller\n");
    return ret;
    }
    ret = nuvoton_qspi_reset_fifo(qspi);
    if (ret)
    dev_err(qspi.dev, "FIFO reset timed out\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nuvoton_qspi_max_transfer_size(spi: *mut spi_device) -> usize {
    static size_t nuvoton_qspi_max_transfer_size(struct spi_device *spi)
    {
    return NUVOTON_QSPI_MAX_TRANSFER_SIZE;
    }
#[no_mangle]
unsafe extern "C" fn nuvoton_qspi_max_message_size(spi: *mut spi_device) -> usize {
    static size_t nuvoton_qspi_max_message_size(struct spi_device *spi)
    {
    return NUVOTON_QSPI_MAX_MESSAGE_SIZE;
    }
    static int nuvoton_qspi_mem_adjust_op_size(struct spi_mem *mem,
    struct spi_mem_op *op)
    {
    if (op.data.nbytes > NUVOTON_QSPI_MAX_TRANSFER_SIZE)
    op.data.nbytes = NUVOTON_QSPI_MAX_TRANSFER_SIZE;
    return 0;
    }
    static bool nuvoton_qspi_mem_supports_op(struct spi_mem *mem,
    const struct spi_mem_op *op)
    {
    if (!spi_mem_default_supports_op(mem, op))
    return false;
    if (op.cmd.buswidth > 4 || op.addr.buswidth > 4 ||
    op.dummy.buswidth > 4 || op.data.buswidth > 4)
    return false;
    if (op.addr.nbytes > 4)
    return false;
    return true;
    }
    static void nuvoton_qspi_set_cs_level(struct nuvoton_qspi *qspi,
    unsigned int cs, bool assert)
    {
    unsigned long flags;
    u32 mask;
    u32 val;
    switch (cs) {
    case 0:
    mask = NUVOTON_QSPI_SSCTL_SS0_MASK;
    break;
    case 1:
    mask = NUVOTON_QSPI_SSCTL_SS1_MASK;
    break;
    default:
    dev_warn(qspi.dev, "invalid chip select %u\n", cs);
    return;
    }
    spin_lock_irqsave(&qspi.ssctl_lock, flags);
    val = nuvoton_qspi_read(qspi, NUVOTON_QSPI_SSCTL_OFFSET);
    if (assert)
    val |= mask;
    else
    val &= ~mask;
    nuvoton_qspi_write(qspi, val, NUVOTON_QSPI_SSCTL_OFFSET);
    spin_unlock_irqrestore(&qspi.ssctl_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn nuvoton_qspi_set_cs(spi: *mut spi_device, level: bool) {
    static void nuvoton_qspi_set_cs(struct spi_device *spi, bool level)
    {
    struct nuvoton_qspi *qspi = spi_controller_get_devdata(spi.controller);
//
// The SPI core passes the physical CS level to ->set_cs(). This
// initial driver only supports active-low native chip selects.
//
    nuvoton_qspi_set_cs_level(qspi, spi_get_chipselect(spi, 0), !level);
    }
#[no_mangle]
unsafe extern "C" fn nuvoton_qspi_mem_set_cs(spi: *mut spi_device, assert: bool) {
    static void nuvoton_qspi_mem_set_cs(struct spi_device *spi, bool assert)
    {
    struct nuvoton_qspi *qspi = spi_controller_get_devdata(spi.controller);
// The direct spi-mem path passes a logical assertion state.
    nuvoton_qspi_set_cs_level(qspi, spi_get_chipselect(spi, 0), assert);
    }
    static int nuvoton_qspi_mem_exec_op(struct spi_mem *mem,
    const struct spi_mem_op *op)
    {
    struct spi_device *spi = mem.spi;
    struct nuvoton_qspi *qspi = spi_controller_get_devdata(spi.controller);
    u8 cmd[2], addr[4];
    int ret;
    int i;
    ret = nuvoton_qspi_setup_transfer(spi, NUVOTON_QSPI_DEFAULT_BPW);
    if (ret)
    return ret;
    nuvoton_qspi_mem_set_cs(spi, true);
    for (i = 0; i < op.cmd.nbytes; i++)
    cmd[i] = op.cmd.opcode >> (8 * (op.cmd.nbytes - i - 1));
    ret = nuvoton_qspi_configure_bus(spi, op.cmd.buswidth, SPI_MEM_DATA_OUT,
    op.max_freq, op.cmd.dtr);
    if (ret)
    goto out_deassert_cs;
    ret = nuvoton_qspi_txrx(qspi, cmd, core::ptr::null_mut(), op.cmd.nbytes);
    if (ret)
    goto out_deassert_cs;
    if (op.addr.nbytes) {
    for (i = 0; i < op.addr.nbytes; i++)
    addr[i] = op.addr.val >> (8 * (op.addr.nbytes - i - 1));
    ret = nuvoton_qspi_configure_bus(spi, op.addr.buswidth, SPI_MEM_DATA_OUT,
    op.max_freq, op.addr.dtr);
    if (ret)
    goto out_deassert_cs;
    ret = nuvoton_qspi_txrx(qspi, addr, core::ptr::null_mut(), op.addr.nbytes);
    if (ret)
    goto out_deassert_cs;
    }
    if (op.dummy.nbytes) {
    ret = nuvoton_qspi_configure_bus(spi, op.dummy.buswidth, SPI_MEM_DATA_OUT,
    op.max_freq, op.dummy.dtr);
    if (ret)
    goto out_deassert_cs;
    ret = nuvoton_qspi_txrx(qspi, core::ptr::null_mut(), core::ptr::null_mut(), op.dummy.nbytes);
    if (ret)
    goto out_deassert_cs;
    }
    if (op.data.nbytes) {
    ret = nuvoton_qspi_configure_bus(spi, op.data.buswidth, op.data.dir,
    op.max_freq, op.data.dtr);
    if (ret)
    goto out_deassert_cs;
    ret = nuvoton_qspi_txrx(qspi,
    op.data.dir == SPI_MEM_DATA_OUT ?
    op.data.buf.out : core::ptr::null_mut(),
    op.data.dir == SPI_MEM_DATA_IN ?
    op.data.buf.in : core::ptr::null_mut(),
    op.data.nbytes);
    }
    out_deassert_cs:
    nuvoton_qspi_mem_set_cs(spi, false);
    return ret;
    }
    static const struct spi_controller_mem_ops nuvoton_qspi_mem_ops = {
    .adjust_op_size = nuvoton_qspi_mem_adjust_op_size,
    .supports_op = nuvoton_qspi_mem_supports_op,
    .exec_op = nuvoton_qspi_mem_exec_op,
    };
    static const struct spi_controller_mem_caps nuvoton_qspi_mem_caps = {
    .per_op_freq = true,
    .dtr = true,
    };
    static int nuvoton_qspi_transfer_one(struct spi_controller *ctlr,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct nuvoton_qspi *qspi = spi_controller_get_devdata(ctlr);
    let mut tx_nbits: c_uint = xfer.tx_nbits ?: SPI_NBITS_SINGLE;
    let mut rx_nbits: c_uint = xfer.rx_nbits ?: SPI_NBITS_SINGLE;
    let mut dir: enum spi_mem_data_dir = SPI_MEM_DATA_IN;
    let mut buswidth: c_uint = 1;
    int ret;
    ret = nuvoton_qspi_setup_transfer(spi, xfer.bits_per_word);
    if (ret)
    return ret;
    if (xfer.tx_buf && xfer.rx_buf &&
    (tx_nbits != SPI_NBITS_SINGLE ||
    rx_nbits != SPI_NBITS_SINGLE))
    return -EOPNOTSUPP;
    if (xfer.tx_buf) {
    dir = SPI_MEM_DATA_OUT;
    if (tx_nbits == SPI_NBITS_QUAD)
    buswidth = 4;
#[no_mangle]
pub unsafe extern "C" fn if(SPI_NBITS_DUAL: tx_nbits ==) -> else {
    else if (tx_nbits == SPI_NBITS_DUAL)
    buswidth = 2;
    } else if (xfer.rx_buf) {
    if (rx_nbits == SPI_NBITS_QUAD)
    buswidth = 4;
#[no_mangle]
pub unsafe extern "C" fn if(SPI_NBITS_DUAL: rx_nbits ==) -> else {
    else if (rx_nbits == SPI_NBITS_DUAL)
    buswidth = 2;
    }
    ret = nuvoton_qspi_configure_bus(spi, buswidth, dir, xfer.speed_hz,
    xfer.dtr_mode);
    if (ret)
    return ret;
    ret = nuvoton_qspi_txrx(qspi, xfer.tx_buf, xfer.rx_buf,
    xfer.len);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nuvoton_qspi_probe(pdev: *mut platform_device) -> c_int {
    static int nuvoton_qspi_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct spi_controller *ctlr;
    struct nuvoton_qspi *qspi;
    struct reset_control *rst;
    let mut num_cs: u32 = NUVOTON_QSPI_DEFAULT_NUM_CS;
    int ret;
    ctlr = devm_spi_alloc_host(dev, sizeof(*qspi));
    if (!ctlr)
    return -ENOMEM;
    platform_set_drvdata(pdev, ctlr);
    qspi = spi_controller_get_devdata(ctlr);
    qspi.dev = dev;
    spin_lock_init(&qspi.ssctl_lock);
    qspi.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(qspi.regs))
    return PTR_ERR(qspi.regs);
    rst = devm_reset_control_get_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(rst))
    return dev_err_probe(dev, PTR_ERR(rst),
    "failed to get reset\n");
    qspi.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(qspi.clk))
    return dev_err_probe(dev, PTR_ERR(qspi.clk),
    "failed to get and enable clock\n");
    ret = reset_control_assert(rst);
    if (ret)
    return dev_err_probe(dev, ret, "failed to assert reset\n");
    udelay(2);
    ret = reset_control_deassert(rst);
    if (ret)
    return dev_err_probe(dev, ret, "failed to deassert reset\n");
    ret = device_property_read_u32(dev, "num-cs", &num_cs);
    if (ret && ret != -EINVAL)
    return dev_err_probe(dev, ret, "failed to read num-cs\n");
    if (!num_cs || num_cs > NUVOTON_QSPI_MAX_NUM_CS)
    return dev_err_probe(dev, -EINVAL, "invalid num-cs %u\n",
    num_cs);
    ctlr.num_chipselect = num_cs;
    ctlr.max_transfer_size = nuvoton_qspi_max_transfer_size;
    ctlr.max_message_size = nuvoton_qspi_max_message_size;
    ctlr.mem_ops = &nuvoton_qspi_mem_ops;
    ctlr.mem_caps = &nuvoton_qspi_mem_caps;
    ctlr.set_cs = nuvoton_qspi_set_cs;
    ctlr.transfer_one = nuvoton_qspi_transfer_one;
    ctlr.bits_per_word_mask = SPI_BPW_MASK(8);
    ctlr.mode_bits = SPI_CPOL | SPI_CPHA | SPI_LSB_FIRST |
    SPI_RX_DUAL | SPI_TX_DUAL |
    SPI_RX_QUAD | SPI_TX_QUAD;
    ctlr.dev.of_node = dev.of_node;
    ret = nuvoton_qspi_hw_init(qspi);
    if (ret)
    return ret;
    ret = devm_spi_register_controller(dev, ctlr);
    if (ret)
    return dev_err_probe(dev, ret,
    "failed to register spi controller\n");
    return 0;
    }
    static const struct of_device_id nuvoton_qspi_of_match[] = {
    { .compatible = "nuvoton,ma35d1-qspi" },
    { }
    };
    MODULE_DEVICE_TABLE(of, nuvoton_qspi_of_match);
    static struct platform_driver nuvoton_qspi_driver = {
    .driver = {
    .name = "ma35d1-qspi",
    .of_match_table = nuvoton_qspi_of_match,
    },
    .probe = nuvoton_qspi_probe,
    };
    module_platform_driver(nuvoton_qspi_driver);
    MODULE_DESCRIPTION("Nuvoton MA35D1 QSPI controller driver");
    MODULE_AUTHOR("Chi-Wen Weng <cwweng@nuvoton.com>");
    MODULE_LICENSE("GPL");
