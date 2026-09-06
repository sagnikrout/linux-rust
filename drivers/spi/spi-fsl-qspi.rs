//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-fsl-qspi.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Freescale QuadSPI driver.
//
// Copyright (C) 2013 Freescale Semiconductor, Inc.
// Copyright (C) 2018 Bootlin
// Copyright (C) 2018 exceet electronics GmbH
// Copyright (C) 2018 Kontron Electronics GmbH
//
// Transition to SPI MEM interface:
// Authors:
// Boris Brezillon <bbrezillon@kernel.org>
// Frieder Schrempf <frieder.schrempf@kontron.de>
// Yogesh Gaur <yogeshnarayan.gaur@nxp.com>
// Suresh Gupta <suresh.gupta@nxp.com>
//
// Based on the original fsl-quadspi.c SPI NOR driver:
// Author: Freescale Semiconductor, Inc.
//

//
// The driver only uses one single LUT entry, that is updated on
// each call of exec_op(). Index 0 is preset at boot with a basic
// read operation, so let's use the last entry (15).
//
pub const SEQID_LUT: c_int = 15;
// Registers used by the driver
pub const QUADSPI_MCR: c_uint = 0x00;

pub const QUADSPI_IPCR: c_uint = 0x08;

pub const QUADSPI_FLSHCR: c_uint = 0x0c;

pub const QUADSPI_BUF0CR: c_uint = 0x10;
pub const QUADSPI_BUF1CR: c_uint = 0x14;
pub const QUADSPI_BUF2CR: c_uint = 0x18;
pub const QUADSPI_BUFXCR_INVALID_MSTRID: c_uint = 0xe;
pub const QUADSPI_BUF3CR: c_uint = 0x1c;

pub const QUADSPI_BFGENCR: c_uint = 0x20;

pub const QUADSPI_BUF0IND: c_uint = 0x30;
pub const QUADSPI_BUF1IND: c_uint = 0x34;
pub const QUADSPI_BUF2IND: c_uint = 0x38;
pub const QUADSPI_SFAR: c_uint = 0x100;
pub const QUADSPI_SMPR: c_uint = 0x108;

pub const QUADSPI_RBCT: c_uint = 0x110;

pub const QUADSPI_TBDR: c_uint = 0x154;
pub const QUADSPI_SR: c_uint = 0x15c;

pub const QUADSPI_FR: c_uint = 0x160;

pub const QUADSPI_RSER: c_uint = 0x164;

pub const QUADSPI_SPTRCLR: c_uint = 0x16c;

pub const QUADSPI_SFA1AD: c_uint = 0x180;
pub const QUADSPI_SFA2AD: c_uint = 0x184;
pub const QUADSPI_SFB1AD: c_uint = 0x188;
pub const QUADSPI_SFB2AD: c_uint = 0x18c;

pub const QUADSPI_LUTKEY: c_uint = 0x300;
pub const QUADSPI_LUTKEY_VALUE: c_uint = 0x5AF05AF0;
pub const QUADSPI_LCKCR: c_uint = 0x304;

pub const QUADSPI_LUT_BASE: c_uint = 0x310;

    (QUADSPI_LUT_BASE + QUADSPI_LUT_OFFSET + (idx) * 4)
// Instruction set for the LUT register
pub const LUT_STOP: c_int = 0;
pub const LUT_CMD: c_int = 1;
pub const LUT_ADDR: c_int = 2;
pub const LUT_DUMMY: c_int = 3;
pub const LUT_MODE: c_int = 4;
pub const LUT_MODE2: c_int = 5;
pub const LUT_MODE4: c_int = 6;
pub const LUT_FSL_READ: c_int = 7;
pub const LUT_FSL_WRITE: c_int = 8;
pub const LUT_JMP_ON_CS: c_int = 9;
pub const LUT_ADDR_DDR: c_int = 10;
pub const LUT_MODE_DDR: c_int = 11;
pub const LUT_MODE2_DDR: c_int = 12;
pub const LUT_MODE4_DDR: c_int = 13;
pub const LUT_FSL_READ_DDR: c_int = 14;
pub const LUT_FSL_WRITE_DDR: c_int = 15;
pub const LUT_DATA_LEARN: c_int = 16;
//
// The PAD definitions for LUT register.
//
// The pad stands for the number of IO lines [0:3].
// For example, the quad read needs four IO lines,
// so you should use LUT_PAD(4).
//

//
// Macro for constructing the LUT entries with the following
// register layout:
//
// ---------------------------------------------------
// | INSTR1 | PAD1 | OPRND1 | INSTR0 | PAD0 | OPRND0 |
// ---------------------------------------------------
//

    ((((ins) << 10) | ((pad) << 8) | (opr)) << (((idx) % 2) * 16))
// Controller needs driver to swap endianness

// Controller needs 4x internal clock

//
// TKT253890, the controller needs the driver to fill the txfifo with
// 16 bytes at least to trigger a data transfer, even though the extra
// data won't be transferred.
//

// TKT245618, the controller cannot wake up from wait mode

//
// Controller adds QSPI_AMBA_BASE (base address of the mapped memory)
// internally. No need to add it when setting SFXXAD and SFAR registers
//

//
// Controller uses TDH bits in register QUADSPI_FLSHCR.
// They need to be set in accordance with the DDR/SDR mode.
//

//
// Do not disable the "qspi" clock when changing its rate.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_qspi_devtype_data {
    pub rxfifo: c_uint,
    pub txfifo: c_uint,
    pub invalid_mstrid: c_int,
    pub ahb_buf_size: c_uint,
    pub sfa_size: c_uint,
    pub quirks: c_uint,
    pub little_endian: bool,
}

    static const struct fsl_qspi_devtype_data vybrid_data = {
    .rxfifo = SZ_128,
    .txfifo = SZ_64,
    .invalid_mstrid = QUADSPI_BUFXCR_INVALID_MSTRID,
    .ahb_buf_size = SZ_1K,
    .quirks = QUADSPI_QUIRK_SWAP_ENDIAN,
    .little_endian = true,
    };
    static const struct fsl_qspi_devtype_data imx6sx_data = {
    .rxfifo = SZ_128,
    .txfifo = SZ_512,
    .invalid_mstrid = QUADSPI_BUFXCR_INVALID_MSTRID,
    .ahb_buf_size = SZ_1K,
    .quirks = QUADSPI_QUIRK_4X_INT_CLK | QUADSPI_QUIRK_TKT245618,
    .little_endian = true,
    };
    static const struct fsl_qspi_devtype_data imx7d_data = {
    .rxfifo = SZ_128,
    .txfifo = SZ_512,
    .invalid_mstrid = QUADSPI_BUFXCR_INVALID_MSTRID,
    .ahb_buf_size = SZ_1K,
    .quirks = QUADSPI_QUIRK_TKT253890 | QUADSPI_QUIRK_4X_INT_CLK |
    QUADSPI_QUIRK_USE_TDH_SETTING,
    .little_endian = true,
    };
    static const struct fsl_qspi_devtype_data imx6ul_data = {
    .rxfifo = SZ_128,
    .txfifo = SZ_512,
    .invalid_mstrid = QUADSPI_BUFXCR_INVALID_MSTRID,
    .ahb_buf_size = SZ_1K,
    .quirks = QUADSPI_QUIRK_TKT253890 | QUADSPI_QUIRK_4X_INT_CLK |
    QUADSPI_QUIRK_USE_TDH_SETTING,
    .little_endian = true,
    };
    static const struct fsl_qspi_devtype_data ls1021a_data = {
    .rxfifo = SZ_128,
    .txfifo = SZ_64,
    .invalid_mstrid = QUADSPI_BUFXCR_INVALID_MSTRID,
    .ahb_buf_size = SZ_1K,
    .quirks = 0,
    .little_endian = false,
    };
    static const struct fsl_qspi_devtype_data ls2080a_data = {
    .rxfifo = SZ_128,
    .txfifo = SZ_64,
    .ahb_buf_size = SZ_1K,
    .invalid_mstrid = 0x0,
    .quirks = QUADSPI_QUIRK_TKT253890 | QUADSPI_QUIRK_BASE_INTERNAL,
    .little_endian = true,
    };
    static const struct fsl_qspi_devtype_data spacemit_k1_data = {
    .rxfifo = SZ_128,
    .txfifo = SZ_256,
    .ahb_buf_size = SZ_512,
    .sfa_size = SZ_1K,
    .invalid_mstrid = QUADSPI_BUFXCR_INVALID_MSTRID,
    .quirks = QUADSPI_QUIRK_TKT253890 | QUADSPI_QUIRK_SKIP_CLK_DISABLE,
    .little_endian = true,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_qspi {
    pub iobase: *mut void __iomem,
    pub ahb_addr: *mut void __iomem,
    pub devtype_data: *const fsl_qspi_devtype_data,
    pub lock: mutex,
    pub c: completion,
    pub resets: *mut reset_control,
    pub clk_en: *mut *mut clk clk,,
    pub pm_qos_req: pm_qos_request,
    pub dev: *mut device,
    pub selected: c_int,
    pub memmap_phy: u32,
}

#[no_mangle]
unsafe extern "C" fn needs_swap_endian(q: *mut fsl_qspi) -> bool {
    static bool needs_swap_endian(struct fsl_qspi *q)
    {
    return !!(q.devtype_data.quirks & QUADSPI_QUIRK_SWAP_ENDIAN);
    }
#[no_mangle]
unsafe extern "C" fn needs_4x_clock(q: *mut fsl_qspi) -> bool {
    static bool needs_4x_clock(struct fsl_qspi *q)
    {
    return !!(q.devtype_data.quirks & QUADSPI_QUIRK_4X_INT_CLK);
    }
#[no_mangle]
unsafe extern "C" fn needs_fill_txfifo(q: *mut fsl_qspi) -> bool {
    static bool needs_fill_txfifo(struct fsl_qspi *q)
    {
    return !!(q.devtype_data.quirks & QUADSPI_QUIRK_TKT253890);
    }
#[no_mangle]
unsafe extern "C" fn needs_wakeup_wait_mode(q: *mut fsl_qspi) -> bool {
    static bool needs_wakeup_wait_mode(struct fsl_qspi *q)
    {
    return !!(q.devtype_data.quirks & QUADSPI_QUIRK_TKT245618);
    }
#[no_mangle]
unsafe extern "C" fn needs_amba_base_offset(q: *mut fsl_qspi) -> bool {
    static bool needs_amba_base_offset(struct fsl_qspi *q)
    {
    return !(q.devtype_data.quirks & QUADSPI_QUIRK_BASE_INTERNAL);
    }
#[no_mangle]
unsafe extern "C" fn needs_tdh_setting(q: *mut fsl_qspi) -> bool {
    static bool needs_tdh_setting(struct fsl_qspi *q)
    {
    return !!(q.devtype_data.quirks & QUADSPI_QUIRK_USE_TDH_SETTING);
    }
#[no_mangle]
unsafe extern "C" fn needs_clk_disable(q: *mut fsl_qspi) -> bool {
    static bool needs_clk_disable(struct fsl_qspi *q)
    {
    return !(q.devtype_data.quirks & QUADSPI_QUIRK_SKIP_CLK_DISABLE);
    }
//
// An IC bug makes it necessary to rearrange the 32-bit data.
// Later chips, such as IMX6SLX, have fixed this bug.
//
#[no_mangle]
pub unsafe extern "C" fn fsl_qspi_endian_xchg(q: *mut fsl_qspi, a: u32) -> u32 {
    static inline u32 fsl_qspi_endian_xchg(struct fsl_qspi *q, u32 a)
    {
    return needs_swap_endian(q) ? __swab32(a) : a;
    }
//
// R/W functions for big- or little-endian registers:
// The QSPI controller's endianness is independent of
// the CPU core's endianness. So far, although the CPU
// core is little-endian the QSPI controller can use
// big-endian or little-endian.
//
#[no_mangle]
unsafe extern "C" fn qspi_writel(q: *mut fsl_qspi, val: u32, addr: *mut void __iomem) {
    static void qspi_writel(struct fsl_qspi *q, u32 val, void __iomem *addr)
    {
    if (q.devtype_data.little_endian)
    iowrite32(val, addr);
    else
    iowrite32be(val, addr);
    }
#[no_mangle]
unsafe extern "C" fn qspi_readl(q: *mut fsl_qspi, addr: *mut void __iomem) -> u32 {
    static u32 qspi_readl(struct fsl_qspi *q, void __iomem *addr)
    {
    if (q.devtype_data.little_endian)
    return ioread32(addr);
    return ioread32be(addr);
    }
#[no_mangle]
unsafe extern "C" fn fsl_qspi_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t fsl_qspi_irq_handler(int irq, void *dev_id)
    {
    struct fsl_qspi *q = dev_id;
    u32 reg;
// clear interrupt
    reg = qspi_readl(q, q.iobase + QUADSPI_FR);
    qspi_writel(q, reg, q.iobase + QUADSPI_FR);
    if (reg & QUADSPI_FR_TFF_MASK)
    complete(&q.c);
    dev_dbg(q.dev, "QUADSPI_FR : 0x%.8x:0x%.8x\n", 0, reg);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn fsl_qspi_check_buswidth(q: *mut fsl_qspi, width: u8) -> c_int {
    static int fsl_qspi_check_buswidth(struct fsl_qspi *q, u8 width)
    {
    switch (width) {
    case 1:
    case 2:
    case 4:
    return 0;
    }
    return -ENOTSUPP;
    }
    static bool fsl_qspi_supports_op(struct spi_mem *mem,
    const struct spi_mem_op *op)
    {
    struct fsl_qspi *q = spi_controller_get_devdata(mem.spi.controller);
    int ret;
    ret = fsl_qspi_check_buswidth(q, op.cmd.buswidth);
    if (op.addr.nbytes)
    ret |= fsl_qspi_check_buswidth(q, op.addr.buswidth);
    if (op.dummy.nbytes)
    ret |= fsl_qspi_check_buswidth(q, op.dummy.buswidth);
    if (op.data.nbytes)
    ret |= fsl_qspi_check_buswidth(q, op.data.buswidth);
    if (ret)
    return false;
//
// The number of instructions needed for the op, needs
// to fit into a single LUT entry.
//
    if (op.addr.nbytes +
    (op.dummy.nbytes ? 1:0) +
    (op.data.nbytes ? 1:0) > 6)
    return false;
// Max 64 dummy clock cycles supported
    if (op.dummy.nbytes &&
    (op.dummy.nbytes * 8 / op.dummy.buswidth > 64))
    return false;
// Max data length, check controller limits and alignment
    if (op.data.dir == SPI_MEM_DATA_IN &&
    (op.data.nbytes > q.devtype_data.ahb_buf_size ||
    (op.data.nbytes > q.devtype_data.rxfifo - 4 &&
    !IS_ALIGNED(op.data.nbytes, 8))))
    return false;
    if (op.data.dir == SPI_MEM_DATA_OUT &&
    op.data.nbytes > q.devtype_data.txfifo)
    return false;
    return spi_mem_default_supports_op(mem, op);
    }
    static void fsl_qspi_prepare_lut(struct fsl_qspi *q,
    const struct spi_mem_op *op)
    {
    void __iomem *base = q.iobase;
    u32 lutval[4] = {};
    let mut lutidx: c_int = 1, i;
    lutval[0] |= LUT_DEF(0, LUT_CMD, LUT_PAD(op.cmd.buswidth),
    op.cmd.opcode);
//
// For some unknown reason, using LUT_ADDR doesn't work in some
// cases (at least with only one byte long addresses), so
// let's use LUT_MODE to write the address bytes one by one
//
    for (i = 0; i < op.addr.nbytes; i++) {
    let mut addrbyte: u8 = op.addr.val >> (8 * (op.addr.nbytes - i - 1));
    lutval[lutidx / 2] |= LUT_DEF(lutidx, LUT_MODE,
    LUT_PAD(op.addr.buswidth),
    addrbyte);
    lutidx++;
    }
    if (op.dummy.nbytes) {
    lutval[lutidx / 2] |= LUT_DEF(lutidx, LUT_DUMMY,
    LUT_PAD(op.dummy.buswidth),
    op.dummy.nbytes * 8 /
    op.dummy.buswidth);
    lutidx++;
    }
    if (op.data.nbytes) {
    lutval[lutidx / 2] |= LUT_DEF(lutidx,
    op.data.dir == SPI_MEM_DATA_IN ?
    LUT_FSL_READ : LUT_FSL_WRITE,
    LUT_PAD(op.data.buswidth),
    0);
    lutidx++;
    }
    lutval[lutidx / 2] |= LUT_DEF(lutidx, LUT_STOP, 0, 0);
// unlock LUT
    qspi_writel(q, QUADSPI_LUTKEY_VALUE, q.iobase + QUADSPI_LUTKEY);
    qspi_writel(q, QUADSPI_LCKER_UNLOCK, q.iobase + QUADSPI_LCKCR);
// fill LUT
    for (i = 0; i < ARRAY_SIZE(lutval); i++)
    qspi_writel(q, lutval[i], base + QUADSPI_LUT_REG(i));
// lock LUT
    qspi_writel(q, QUADSPI_LUTKEY_VALUE, q.iobase + QUADSPI_LUTKEY);
    qspi_writel(q, QUADSPI_LCKER_LOCK, q.iobase + QUADSPI_LCKCR);
    }
#[no_mangle]
unsafe extern "C" fn fsl_qspi_clk_prep_enable(q: *mut fsl_qspi) -> c_int {
    static int fsl_qspi_clk_prep_enable(struct fsl_qspi *q)
    {
    int ret;
    ret = clk_prepare_enable(q.clk_en);
    if (ret)
    return ret;
    ret = clk_prepare_enable(q.clk);
    if (ret) {
    clk_disable_unprepare(q.clk_en);
    return ret;
    }
    if (needs_wakeup_wait_mode(q))
    cpu_latency_qos_add_request(&q.pm_qos_req, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsl_qspi_clk_disable_unprep(q: *mut fsl_qspi) {
    static void fsl_qspi_clk_disable_unprep(struct fsl_qspi *q)
    {
    if (needs_wakeup_wait_mode(q))
    cpu_latency_qos_remove_request(&q.pm_qos_req);
    clk_disable_unprepare(q.clk);
    clk_disable_unprepare(q.clk_en);
    }
//
// If we have changed the content of the flash by writing or erasing, or if we
// read from flash with a different offset into the page buffer, we need to
// invalidate the AHB buffer. If we do not do so, we may read out the wrong
// data. The spec tells us reset the AHB domain and Serial Flash domain at
// the same time.
//
#[no_mangle]
unsafe extern "C" fn fsl_qspi_invalidate(q: *mut fsl_qspi) {
    static void fsl_qspi_invalidate(struct fsl_qspi *q)
    {
    u32 reg;
    reg = qspi_readl(q, q.iobase + QUADSPI_MCR);
    reg |= QUADSPI_MCR_SWRSTHD_MASK | QUADSPI_MCR_SWRSTSD_MASK;
    qspi_writel(q, reg, q.iobase + QUADSPI_MCR);
//
// The minimum delay : 1 AHB + 2 SFCK clocks.
// Delay 1 us is enough.
//
    udelay(1);
    reg &= ~(QUADSPI_MCR_SWRSTHD_MASK | QUADSPI_MCR_SWRSTSD_MASK);
    qspi_writel(q, reg, q.iobase + QUADSPI_MCR);
    }
    static void fsl_qspi_select_mem(struct fsl_qspi *q, struct spi_device *spi,
    const struct spi_mem_op *op)
    {
    let mut rate: c_ulong = op.max_freq;
    int ret;
    if (q.selected == spi_get_chipselect(spi, 0))
    return;
    if (needs_4x_clock(q))
    rate *= 4;
    if (needs_clk_disable(q))
    fsl_qspi_clk_disable_unprep(q);
    ret = clk_set_rate(q.clk, rate);
    if (ret)
    return;
    if (needs_clk_disable(q)) {
    ret = fsl_qspi_clk_prep_enable(q);
    if (ret)
    return;
    }
    q.selected = spi_get_chipselect(spi, 0);
    fsl_qspi_invalidate(q);
    }
#[no_mangle]
unsafe extern "C" fn fsl_qspi_read_ahb(q: *mut fsl_qspi, op: *const spi_mem_op) {
    static void fsl_qspi_read_ahb(struct fsl_qspi *q, const struct spi_mem_op *op)
    {
    memcpy_fromio(op.data.buf.in,
    q.ahb_addr + q.selected * q.devtype_data.ahb_buf_size,
    op.data.nbytes);
    }
    static void fsl_qspi_fill_txfifo(struct fsl_qspi *q,
    const struct spi_mem_op *op)
    {
    void __iomem *base = q.iobase;
    int i;
    u32 val;
    for (i = 0; i < ALIGN_DOWN(op.data.nbytes, 4); i += 4) {
    memcpy(&val, op.data.buf.out + i, 4);
    val = fsl_qspi_endian_xchg(q, val);
    qspi_writel(q, val, base + QUADSPI_TBDR);
    }
    if (i < op.data.nbytes) {
    memcpy(&val, op.data.buf.out + i, op.data.nbytes - i);
    val = fsl_qspi_endian_xchg(q, val);
    qspi_writel(q, val, base + QUADSPI_TBDR);
    }
    if (needs_fill_txfifo(q)) {
    for (i = op.data.nbytes; i < 16; i += 4)
    qspi_writel(q, 0, base + QUADSPI_TBDR);
    }
    }
    static void fsl_qspi_read_rxfifo(struct fsl_qspi *q,
    const struct spi_mem_op *op)
    {
    void __iomem *base = q.iobase;
    int i;
    u8 *buf = op.data.buf.in;
    u32 val;
    for (i = 0; i < ALIGN_DOWN(op.data.nbytes, 4); i += 4) {
    val = qspi_readl(q, base + QUADSPI_RBDR(i / 4));
    val = fsl_qspi_endian_xchg(q, val);
    memcpy(buf + i, &val, 4);
    }
    if (i < op.data.nbytes) {
    val = qspi_readl(q, base + QUADSPI_RBDR(i / 4));
    val = fsl_qspi_endian_xchg(q, val);
    memcpy(buf + i, &val, op.data.nbytes - i);
    }
    }
#[no_mangle]
unsafe extern "C" fn fsl_qspi_do_op(q: *mut fsl_qspi, op: *const spi_mem_op) -> c_int {
    static int fsl_qspi_do_op(struct fsl_qspi *q, const struct spi_mem_op *op)
    {
    void __iomem *base = q.iobase;
    let mut err: c_int = 0;
    reinit_completion(&q.c);
//
// Always start the sequence at the same index since we update
// the LUT at each exec_op() call. And also specify the DATA
// length, since it's has not been specified in the LUT.
//
    qspi_writel(q, op.data.nbytes | QUADSPI_IPCR_SEQID(SEQID_LUT),
    base + QUADSPI_IPCR);
// Wait for the interrupt.
    if (!wait_for_completion_timeout(&q.c, msecs_to_jiffies(1000)))
    err = -ETIMEDOUT;
    if (!err && op.data.nbytes && op.data.dir == SPI_MEM_DATA_IN)
    fsl_qspi_read_rxfifo(q, op);
    return err;
    }
    static int fsl_qspi_readl_poll_tout(struct fsl_qspi *q, void __iomem *base,
    u32 mask, u32 delay_us, u32 timeout_us)
    {
    u32 reg;
    if (!q.devtype_data.little_endian)
    mask = (u32)cpu_to_be32(mask);
    return readl_poll_timeout(base, reg, !(reg & mask), delay_us,
    timeout_us);
    }
#[no_mangle]
unsafe extern "C" fn fsl_qspi_exec_op(mem: *mut spi_mem, op: *const spi_mem_op) -> c_int {
    static int fsl_qspi_exec_op(struct spi_mem *mem, const struct spi_mem_op *op)
    {
    struct fsl_qspi *q = spi_controller_get_devdata(mem.spi.controller);
    void __iomem *base = q.iobase;
    let mut addr_offset: u32 = 0;
    let mut err: c_int = 0;
    let mut invalid_mstrid: c_int = q.devtype_data.invalid_mstrid;
    mutex_lock(&q.lock);
// wait for the controller being ready
    fsl_qspi_readl_poll_tout(q, base + QUADSPI_SR, (QUADSPI_SR_IP_ACC_MASK |
    QUADSPI_SR_AHB_ACC_MASK), 10, 1000);
    fsl_qspi_select_mem(q, mem.spi, op);
    if (needs_amba_base_offset(q))
    addr_offset = q.memmap_phy;
    qspi_writel(q,
    q.selected * q.devtype_data.ahb_buf_size + addr_offset,
    base + QUADSPI_SFAR);
    qspi_writel(q, qspi_readl(q, base + QUADSPI_MCR) |
    QUADSPI_MCR_CLR_RXF_MASK | QUADSPI_MCR_CLR_TXF_MASK,
    base + QUADSPI_MCR);
    qspi_writel(q, QUADSPI_SPTRCLR_BFPTRC | QUADSPI_SPTRCLR_IPPTRC,
    base + QUADSPI_SPTRCLR);
    qspi_writel(q, invalid_mstrid, base + QUADSPI_BUF0CR);
    qspi_writel(q, invalid_mstrid, base + QUADSPI_BUF1CR);
    qspi_writel(q, invalid_mstrid, base + QUADSPI_BUF2CR);
    fsl_qspi_prepare_lut(q, op);
//
// If we have large chunks of data, we read them through the AHB bus
// by accessing the mapped memory. In all other cases we use
// IP commands to access the flash.
//
    if (op.data.nbytes > (q.devtype_data.rxfifo - 4) &&
    op.data.dir == SPI_MEM_DATA_IN) {
    fsl_qspi_read_ahb(q, op);
    } else {
    qspi_writel(q, QUADSPI_RBCT_WMRK_MASK |
    QUADSPI_RBCT_RXBRD_USEIPS, base + QUADSPI_RBCT);
    if (op.data.nbytes && op.data.dir == SPI_MEM_DATA_OUT)
    fsl_qspi_fill_txfifo(q, op);
    err = fsl_qspi_do_op(q, op);
    }
// Invalidate the data in the AHB buffer.
    fsl_qspi_invalidate(q);
    mutex_unlock(&q.lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn fsl_qspi_adjust_op_size(mem: *mut spi_mem, op: *mut spi_mem_op) -> c_int {
    static int fsl_qspi_adjust_op_size(struct spi_mem *mem, struct spi_mem_op *op)
    {
    struct fsl_qspi *q = spi_controller_get_devdata(mem.spi.controller);
    if (op.data.dir == SPI_MEM_DATA_OUT) {
    if (op.data.nbytes > q.devtype_data.txfifo)
    op.data.nbytes = q.devtype_data.txfifo;
    } else {
    if (op.data.nbytes > q.devtype_data.ahb_buf_size)
    op.data.nbytes = q.devtype_data.ahb_buf_size;
#[no_mangle]
pub unsafe extern "C" fn if(4): op->data.nbytes > (q->devtype_data->rxfifo -) -> else {
    else if (op.data.nbytes > (q.devtype_data.rxfifo - 4))
    op.data.nbytes = ALIGN_DOWN(op.data.nbytes, 8);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsl_qspi_default_setup(q: *mut fsl_qspi) -> c_int {
    static int fsl_qspi_default_setup(struct fsl_qspi *q)
    {
    void __iomem *base = q.iobase;
    u32 reg, addr_offset = 0;
    u32 sfa_size;
    int ret;
// disable and unprepare clock to avoid glitch pass to controller
    fsl_qspi_clk_disable_unprep(q);
// the default frequency, we will change it later if necessary.
    ret = clk_set_rate(q.clk, 66000000);
    if (ret)
    return ret;
    ret = fsl_qspi_clk_prep_enable(q);
    if (ret)
    return ret;
// Reset the module
    qspi_writel(q, QUADSPI_MCR_SWRSTSD_MASK | QUADSPI_MCR_SWRSTHD_MASK,
    base + QUADSPI_MCR);
    udelay(1);
// Disable the module
    qspi_writel(q, QUADSPI_MCR_MDIS_MASK | QUADSPI_MCR_RESERVED_MASK,
    base + QUADSPI_MCR);
//
// Previous boot stages (BootROM, bootloader) might have used DDR
// mode and did not clear the TDH bits. As we currently use SDR mode
// only, clear the TDH bits if necessary.
//
    if (needs_tdh_setting(q))
    qspi_writel(q, qspi_readl(q, base + QUADSPI_FLSHCR) &
    ~QUADSPI_FLSHCR_TDH_MASK,
    base + QUADSPI_FLSHCR);
    reg = qspi_readl(q, base + QUADSPI_SMPR);
    qspi_writel(q, reg & ~(QUADSPI_SMPR_FSDLY_MASK
    | QUADSPI_SMPR_FSPHS_MASK
    | QUADSPI_SMPR_HSENA_MASK
    | QUADSPI_SMPR_DDRSMP_MASK), base + QUADSPI_SMPR);
// We only use the buffer3 for AHB read
    qspi_writel(q, 0, base + QUADSPI_BUF0IND);
    qspi_writel(q, 0, base + QUADSPI_BUF1IND);
    qspi_writel(q, 0, base + QUADSPI_BUF2IND);
    qspi_writel(q, QUADSPI_BFGENCR_SEQID(SEQID_LUT),
    q.iobase + QUADSPI_BFGENCR);
    qspi_writel(q, QUADSPI_RBCT_WMRK_MASK, base + QUADSPI_RBCT);
    qspi_writel(q, QUADSPI_BUF3CR_ALLMST_MASK |
    QUADSPI_BUF3CR_ADATSZ(q.devtype_data.ahb_buf_size / 8),
    base + QUADSPI_BUF3CR);
    if (needs_amba_base_offset(q))
    addr_offset = q.memmap_phy;
//
// In HW there can be a maximum of four chips on two buses with
// two chip selects on each bus. We use four chip selects in SW
// to differentiate between the four chips.
//
// By default we write the AHB buffer size to each chip, but
// a different size can be specified with devtype_data->sfa_size.
// The SFA1AD, SFA2AD, SFB1AD, and SFB2AD registers define the
// top (end) of these four regions.
//
    sfa_size = q.devtype_data.sfa_size ? : q.devtype_data.ahb_buf_size;
    qspi_writel(q, addr_offset + 1 * sfa_size, base + QUADSPI_SFA1AD);
    qspi_writel(q, addr_offset + 2 * sfa_size, base + QUADSPI_SFA2AD);
    qspi_writel(q, addr_offset + 3 * sfa_size, base + QUADSPI_SFB1AD);
    qspi_writel(q, addr_offset + 4 * sfa_size, base + QUADSPI_SFB2AD);
    q.selected = -1;
// Enable the module
    qspi_writel(q, QUADSPI_MCR_RESERVED_MASK | QUADSPI_MCR_END_CFG_MASK,
    base + QUADSPI_MCR);
// clear all interrupt status
    qspi_writel(q, 0xffffffff, q.iobase + QUADSPI_FR);
// enable the interrupt
    qspi_writel(q, QUADSPI_RSER_TFIE, q.iobase + QUADSPI_RSER);
    return 0;
    }
    static const char *fsl_qspi_get_name(struct spi_mem *mem)
    {
    struct fsl_qspi *q = spi_controller_get_devdata(mem.spi.controller);
    struct device *dev = &mem.spi.dev;
    const char *name;
//
// In order to keep mtdparts compatible with the old MTD driver at
// mtd/spi-nor/fsl-quadspi.c, we set a custom name derived from the
// platform_device of the controller.
//
    if (of_get_available_child_count(q.dev.of_node) == 1)
    return dev_name(q.dev);
    name = devm_kasprintf(dev, GFP_KERNEL,
    "%s-%d", dev_name(q.dev),
    spi_get_chipselect(mem.spi, 0));
    if (!name) {
    dev_err(dev, "failed to get memory for custom flash name\n");
    return ERR_PTR(-ENOMEM);
    }
    return name;
    }
    static const struct spi_controller_mem_ops fsl_qspi_mem_ops = {
    .adjust_op_size = fsl_qspi_adjust_op_size,
    .supports_op = fsl_qspi_supports_op,
    .exec_op = fsl_qspi_exec_op,
    .get_name = fsl_qspi_get_name,
    };
    static const struct spi_controller_mem_caps fsl_qspi_mem_caps = {
    .per_op_freq = true,
    };
#[no_mangle]
unsafe extern "C" fn fsl_qspi_disable(data: *mut c_void) {
    static void fsl_qspi_disable(void *data)
    {
    struct fsl_qspi *q = data;
// disable the hardware
    qspi_writel(q, QUADSPI_MCR_MDIS_MASK, q.iobase + QUADSPI_MCR);
    qspi_writel(q, 0x0, q.iobase + QUADSPI_RSER);
    }
#[no_mangle]
unsafe extern "C" fn fsl_qspi_cleanup(data: *mut c_void) {
    static void fsl_qspi_cleanup(void *data)
    {
    struct fsl_qspi *q = data;
    reset_control_assert(q.resets);
    fsl_qspi_clk_disable_unprep(q);
    mutex_destroy(&q.lock);
    }
#[no_mangle]
unsafe extern "C" fn fsl_qspi_probe(pdev: *mut platform_device) -> c_int {
    static int fsl_qspi_probe(struct platform_device *pdev)
    {
    struct spi_controller *ctlr;
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct resource *res;
    struct fsl_qspi *q;
    int ret;
    ctlr = devm_spi_alloc_host(&pdev.dev, sizeof(*q));
    if (!ctlr)
    return -ENOMEM;
    ctlr.mode_bits = SPI_RX_DUAL | SPI_RX_QUAD |
    SPI_TX_DUAL | SPI_TX_QUAD;
    q = spi_controller_get_devdata(ctlr);
    q.dev = dev;
    q.devtype_data = of_device_get_match_data(dev);
    if (!q.devtype_data)
    return -ENODEV;
    platform_set_drvdata(pdev, q);
// find the resources
    q.iobase = devm_platform_ioremap_resource_byname(pdev, "QuadSPI");
    if (IS_ERR(q.iobase))
    return PTR_ERR(q.iobase);
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM,
    "QuadSPI-memory");
    if (!res)
    return -EINVAL;
    q.memmap_phy = res.start;
// Since there are 4 cs, map size required is 4 times ahb_buf_size
    q.ahb_addr = devm_ioremap(dev, q.memmap_phy,
    (q.devtype_data.ahb_buf_size * 4));
    if (!q.ahb_addr)
    return -ENOMEM;
    q.resets = devm_reset_control_array_get_optional_exclusive(dev);
    if (IS_ERR(q.resets))
    return PTR_ERR(q.resets);
// find the clocks
    q.clk_en = devm_clk_get(dev, "qspi_en");
    if (IS_ERR(q.clk_en))
    return PTR_ERR(q.clk_en);
    q.clk = devm_clk_get(dev, "qspi");
    if (IS_ERR(q.clk))
    return PTR_ERR(q.clk);
    mutex_init(&q.lock);
    ret = fsl_qspi_clk_prep_enable(q);
    if (ret) {
    dev_err(dev, "can not enable the clock\n");
    return ret;
    }
    ret = devm_add_action_or_reset(dev, fsl_qspi_cleanup, q);
    if (ret)
    return ret;
    ret = reset_control_deassert(q.resets);
    if (ret)
    return ret;
// find the irq
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    return ret;
    init_completion(&q.c);
    ret = devm_request_irq(dev, ret,
    fsl_qspi_irq_handler, 0, pdev.name, q);
    if (ret) {
    dev_err(dev, "failed to request irq: %d\n", ret);
    return ret;
    }
    ctlr.bus_num = -1;
    ctlr.num_chipselect = 4;
    ctlr.mem_ops = &fsl_qspi_mem_ops;
    ctlr.mem_caps = &fsl_qspi_mem_caps;
    fsl_qspi_default_setup(q);
    ctlr.dev.of_node = np;
    ret = devm_add_action_or_reset(dev, fsl_qspi_disable, q);
    if (ret)
    return ret;
    ret = devm_spi_register_controller(dev, ctlr);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsl_qspi_suspend(dev: *mut device) -> c_int {
    static int fsl_qspi_suspend(struct device *dev)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsl_qspi_resume(dev: *mut device) -> c_int {
    static int fsl_qspi_resume(struct device *dev)
    {
    struct fsl_qspi *q = dev_get_drvdata(dev);
    fsl_qspi_default_setup(q);
    return 0;
    }
    static const struct of_device_id fsl_qspi_dt_ids[] = {
    { .compatible = "fsl,vf610-qspi", .data = &vybrid_data, },
    { .compatible = "fsl,imx6sx-qspi", .data = &imx6sx_data, },
    { .compatible = "fsl,imx7d-qspi", .data = &imx7d_data, },
    { .compatible = "fsl,imx6ul-qspi", .data = &imx6ul_data, },
    { .compatible = "fsl,ls1021a-qspi", .data = &ls1021a_data, },
    { .compatible = "fsl,ls2080a-qspi", .data = &ls2080a_data, },
    { .compatible = "spacemit,k1-qspi", .data = &spacemit_k1_data, },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, fsl_qspi_dt_ids);
    static const struct dev_pm_ops fsl_qspi_pm_ops = {
    .suspend	= fsl_qspi_suspend,
    .resume		= fsl_qspi_resume,
    };
    static struct platform_driver fsl_qspi_driver = {
    .driver = {
    .name	= "fsl-quadspi",
    .of_match_table = fsl_qspi_dt_ids,
    .pm =   &fsl_qspi_pm_ops,
    },
    .probe          = fsl_qspi_probe,
    };
    module_platform_driver(fsl_qspi_driver);
    MODULE_DESCRIPTION("Freescale QuadSPI Controller Driver");
    MODULE_AUTHOR("Freescale Semiconductor Inc.");
    MODULE_AUTHOR("Boris Brezillon <bbrezillon@kernel.org>");
    MODULE_AUTHOR("Frieder Schrempf <frieder.schrempf@kontron.de>");
    MODULE_AUTHOR("Yogesh Gaur <yogeshnarayan.gaur@nxp.com>");
    MODULE_AUTHOR("Suresh Gupta <suresh.gupta@nxp.com>");
    MODULE_LICENSE("GPL v2");
