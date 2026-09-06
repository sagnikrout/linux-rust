//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-airoha-snfi.c
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
// Copyright (c) 2024 AIROHA Inc
// Author: Lorenzo Bianconi <lorenzo@kernel.org>
// Author: Ray Liu <ray.liu@airoha.com>
//

// SPI
pub const REG_SPI_CTRL_BASE: c_uint = 0x1FA10000;
pub const REG_SPI_CTRL_READ_MODE: c_uint = 0x0000;
pub const REG_SPI_CTRL_READ_IDLE_EN: c_uint = 0x0004;
pub const REG_SPI_CTRL_SIDLY: c_uint = 0x0008;
pub const REG_SPI_CTRL_CSHEXT: c_uint = 0x000c;
pub const REG_SPI_CTRL_CSLEXT: c_uint = 0x0010;
pub const REG_SPI_CTRL_MTX_MODE_TOG: c_uint = 0x0014;

pub const REG_SPI_CTRL_RDCTL_FSM: c_uint = 0x0018;

pub const REG_SPI_CTRL_MACMUX_SEL: c_uint = 0x001c;
pub const REG_SPI_CTRL_MANUAL_EN: c_uint = 0x0020;

pub const REG_SPI_CTRL_OPFIFO_EMPTY: c_uint = 0x0024;

pub const REG_SPI_CTRL_OPFIFO_WDATA: c_uint = 0x0028;

pub const REG_SPI_CTRL_OPFIFO_FULL: c_uint = 0x002c;

pub const REG_SPI_CTRL_OPFIFO_WR: c_uint = 0x0030;

pub const REG_SPI_CTRL_DFIFO_FULL: c_uint = 0x0034;

pub const REG_SPI_CTRL_DFIFO_WDATA: c_uint = 0x0038;

pub const REG_SPI_CTRL_DFIFO_EMPTY: c_uint = 0x003c;

pub const REG_SPI_CTRL_DFIFO_RD: c_uint = 0x0040;

pub const REG_SPI_CTRL_DFIFO_RDATA: c_uint = 0x0044;

pub const REG_SPI_CTRL_DUMMY: c_uint = 0x0080;

pub const REG_SPI_CTRL_PROBE_SEL: c_uint = 0x0088;
pub const REG_SPI_CTRL_INTERRUPT: c_uint = 0x0090;
pub const REG_SPI_CTRL_INTERRUPT_EN: c_uint = 0x0094;
pub const REG_SPI_CTRL_SI_CK_SEL: c_uint = 0x009c;
pub const REG_SPI_CTRL_SW_CFGNANDADDR_VAL: c_uint = 0x010c;
pub const REG_SPI_CTRL_SW_CFGNANDADDR_EN: c_uint = 0x0110;
pub const REG_SPI_CTRL_SFC_STRAP: c_uint = 0x0114;
pub const REG_SPI_CTRL_NFI2SPI_EN: c_uint = 0x0130;

// NFI2SPI
pub const REG_SPI_NFI_CNFG: c_uint = 0x0000;

pub const REG_SPI_NFI_PAGEFMT: c_uint = 0x0004;

pub const REG_SPI_NFI_CON: c_uint = 0x0008;

pub const REG_SPI_NFI_INTR_EN: c_uint = 0x0010;

    (SPI_NFI_RD_DONE_EN | SPI_NFI_WR_DONE_EN |		\
    SPI_NFI_RST_DONE_EN | SPI_NFI_ERASE_DONE_EN |		\
    SPI_NFI_BUSY_RETURN_EN | SPI_NFI_ACCESS_LOCK_EN |	\
    SPI_NFI_AHB_DONE_EN)
pub const REG_SPI_NFI_INTR: c_uint = 0x0014;

pub const REG_SPI_NFI_CMD: c_uint = 0x0020;
pub const REG_SPI_NFI_ADDR_NOB: c_uint = 0x0030;

pub const REG_SPI_NFI_STA: c_uint = 0x0060;
pub const REG_SPI_NFI_FIFOSTA: c_uint = 0x0064;
pub const REG_SPI_NFI_STRADDR: c_uint = 0x0080;
pub const REG_SPI_NFI_FDM0L: c_uint = 0x00a0;
pub const REG_SPI_NFI_FDM0M: c_uint = 0x00a4;
pub const REG_SPI_NFI_FDM7L: c_uint = 0x00d8;
pub const REG_SPI_NFI_FDM7M: c_uint = 0x00dc;
pub const REG_SPI_NFI_FIFODATA0: c_uint = 0x0190;
pub const REG_SPI_NFI_FIFODATA1: c_uint = 0x0194;
pub const REG_SPI_NFI_FIFODATA2: c_uint = 0x0198;
pub const REG_SPI_NFI_FIFODATA3: c_uint = 0x019c;
pub const REG_SPI_NFI_MASTERSTA: c_uint = 0x0224;
pub const REG_SPI_NFI_SECCUS_SIZE: c_uint = 0x022c;

pub const REG_SPI_NFI_RD_CTL2: c_uint = 0x0510;

pub const REG_SPI_NFI_RD_CTL3: c_uint = 0x0514;
pub const REG_SPI_NFI_PG_CTL1: c_uint = 0x0524;

pub const REG_SPI_NFI_PG_CTL2: c_uint = 0x0528;
pub const REG_SPI_NFI_NOR_PROG_ADDR: c_uint = 0x052c;
pub const REG_SPI_NFI_NOR_RD_ADDR: c_uint = 0x0534;
pub const REG_SPI_NFI_SNF_MISC_CTL: c_uint = 0x0538;

pub const REG_SPI_NFI_SNF_MISC_CTL2: c_uint = 0x053c;

pub const REG_SPI_NFI_SNF_STA_CTL1: c_uint = 0x0550;

pub const REG_SPI_NFI_SNF_STA_CTL2: c_uint = 0x0554;
pub const REG_SPI_NFI_SNF_NFI_CNFG: c_uint = 0x055c;

// SPI NAND Protocol OP
pub const SPI_NAND_OP_GET_FEATURE: c_uint = 0x0f;
pub const SPI_NAND_OP_SET_FEATURE: c_uint = 0x1f;
pub const SPI_NAND_OP_PAGE_READ: c_uint = 0x13;
pub const SPI_NAND_OP_READ_FROM_CACHE_SINGLE: c_uint = 0x03;
pub const SPI_NAND_OP_READ_FROM_CACHE_SINGLE_FAST: c_uint = 0x0b;
pub const SPI_NAND_OP_READ_FROM_CACHE_DUAL: c_uint = 0x3b;
pub const SPI_NAND_OP_READ_FROM_CACHE_DUALIO: c_uint = 0xbb;
pub const SPI_NAND_OP_READ_FROM_CACHE_QUAD: c_uint = 0x6b;
pub const SPI_NAND_OP_READ_FROM_CACHE_QUADIO: c_uint = 0xeb;
pub const SPI_NAND_OP_WRITE_ENABLE: c_uint = 0x06;
pub const SPI_NAND_OP_WRITE_DISABLE: c_uint = 0x04;
pub const SPI_NAND_OP_PROGRAM_LOAD_SINGLE: c_uint = 0x02;
pub const SPI_NAND_OP_PROGRAM_LOAD_QUAD: c_uint = 0x32;
pub const SPI_NAND_OP_PROGRAM_LOAD_RAMDOM_SINGLE: c_uint = 0x84;
pub const SPI_NAND_OP_PROGRAM_LOAD_RAMDON_QUAD: c_uint = 0x34;
pub const SPI_NAND_OP_PROGRAM_EXECUTE: c_uint = 0x10;
pub const SPI_NAND_OP_READ_ID: c_uint = 0x9f;
pub const SPI_NAND_OP_BLOCK_ERASE: c_uint = 0xd8;
pub const SPI_NAND_OP_RESET: c_uint = 0xff;
pub const SPI_NAND_OP_DIE_SELECT: c_uint = 0xc2;
// SNAND FIFO commands
pub const SNAND_FIFO_TX_BUSWIDTH_SINGLE: c_uint = 0x08;
pub const SNAND_FIFO_TX_BUSWIDTH_DUAL: c_uint = 0x09;
pub const SNAND_FIFO_TX_BUSWIDTH_QUAD: c_uint = 0x0a;
pub const SNAND_FIFO_RX_BUSWIDTH_SINGLE: c_uint = 0x0c;
pub const SNAND_FIFO_RX_BUSWIDTH_DUAL: c_uint = 0x0e;
pub const SNAND_FIFO_RX_BUSWIDTH_QUAD: c_uint = 0x0f;

pub const SPI_MAX_TRANSFER_SIZE: c_int = 511;
    enum airoha_snand_mode {
    SPI_MODE_AUTO,
    SPI_MODE_MANUAL,
    SPI_MODE_DMA,
    };
    enum airoha_snand_cs {
    SPI_CHIP_SEL_HIGH,
    SPI_CHIP_SEL_LOW,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_snand_ctrl {
    pub dev: *mut device,
    pub regmap_ctrl: *mut regmap,
    pub regmap_nfi: *mut regmap,
    pub spi_clk: *mut clk,
}

    static int airoha_snand_set_fifo_op(struct airoha_snand_ctrl *as_ctrl,
    u8 op_cmd, int op_len)
    {
    int err;
    u32 val;
    err = regmap_write(as_ctrl.regmap_ctrl, REG_SPI_CTRL_OPFIFO_WDATA,
    FIELD_PREP(SPI_CTRL_OPFIFO_LEN, op_len) |
    FIELD_PREP(SPI_CTRL_OPFIFO_OP, op_cmd));
    if (err)
    return err;
    err = regmap_read_poll_timeout(as_ctrl.regmap_ctrl,
    REG_SPI_CTRL_OPFIFO_FULL,
    val, !(val & SPI_CTRL_OPFIFO_FULL),
    0, 250 * USEC_PER_MSEC);
    if (err)
    return err;
    err = regmap_write(as_ctrl.regmap_ctrl, REG_SPI_CTRL_OPFIFO_WR,
    SPI_CTRL_OPFIFO_WR);
    if (err)
    return err;
    return regmap_read_poll_timeout(as_ctrl.regmap_ctrl,
    REG_SPI_CTRL_OPFIFO_EMPTY,
    val, (val & SPI_CTRL_OPFIFO_EMPTY),
    0, 250 * USEC_PER_MSEC);
    }
#[no_mangle]
unsafe extern "C" fn airoha_snand_set_cs(as_ctrl: *mut airoha_snand_ctrl, cs: u8) -> c_int {
    static int airoha_snand_set_cs(struct airoha_snand_ctrl *as_ctrl, u8 cs)
    {
    return airoha_snand_set_fifo_op(as_ctrl, cs, sizeof(cs));
    }
    static int airoha_snand_write_data_to_fifo(struct airoha_snand_ctrl *as_ctrl,
    const u8 *data, int len)
    {
    int i;
    for (i = 0; i < len; i++) {
    int err;
    u32 val;
// 1. Wait until dfifo is not full
    err = regmap_read_poll_timeout(as_ctrl.regmap_ctrl,
    REG_SPI_CTRL_DFIFO_FULL, val,
    !(val & SPI_CTRL_DFIFO_FULL),
    0, 250 * USEC_PER_MSEC);
    if (err)
    return err;
// 2. Write data to register DFIFO_WDATA
    err = regmap_write(as_ctrl.regmap_ctrl,
    REG_SPI_CTRL_DFIFO_WDATA,
    FIELD_PREP(SPI_CTRL_DFIFO_WDATA, data[i]));
    if (err)
    return err;
// 3. Wait until dfifo is not full
    err = regmap_read_poll_timeout(as_ctrl.regmap_ctrl,
    REG_SPI_CTRL_DFIFO_FULL, val,
    !(val & SPI_CTRL_DFIFO_FULL),
    0, 250 * USEC_PER_MSEC);
    if (err)
    return err;
    }
    return 0;
    }
    static int airoha_snand_read_data_from_fifo(struct airoha_snand_ctrl *as_ctrl,
    u8 *ptr, int len)
    {
    int i;
    for (i = 0; i < len; i++) {
    int err;
    u32 val;
// 1. wait until dfifo is not empty
    err = regmap_read_poll_timeout(as_ctrl.regmap_ctrl,
    REG_SPI_CTRL_DFIFO_EMPTY, val,
    !(val & SPI_CTRL_DFIFO_EMPTY),
    0, 250 * USEC_PER_MSEC);
    if (err)
    return err;
// 2. read from dfifo to register DFIFO_RDATA
    err = regmap_read(as_ctrl.regmap_ctrl,
    REG_SPI_CTRL_DFIFO_RDATA, &val);
    if (err)
    return err;
    ptr[i] = FIELD_GET(SPI_CTRL_DFIFO_RDATA, val);
// 3. enable register DFIFO_RD to read next byte
    err = regmap_write(as_ctrl.regmap_ctrl,
    REG_SPI_CTRL_DFIFO_RD, SPI_CTRL_DFIFO_RD);
    if (err)
    return err;
    }
    return 0;
    }
    static int airoha_snand_set_mode(struct airoha_snand_ctrl *as_ctrl,
    enum airoha_snand_mode mode)
    {
    int err;
    switch (mode) {
    case SPI_MODE_MANUAL: {
    u32 val;
    err = regmap_write(as_ctrl.regmap_ctrl,
    REG_SPI_CTRL_NFI2SPI_EN, 0);
    if (err)
    return err;
    err = regmap_write(as_ctrl.regmap_ctrl,
    REG_SPI_CTRL_READ_IDLE_EN, 0);
    if (err)
    return err;
    err = regmap_read_poll_timeout(as_ctrl.regmap_ctrl,
    REG_SPI_CTRL_RDCTL_FSM, val,
    !(val & SPI_CTRL_RDCTL_FSM),
    0, 250 * USEC_PER_MSEC);
    if (err)
    return err;
    err = regmap_write(as_ctrl.regmap_ctrl,
    REG_SPI_CTRL_MTX_MODE_TOG, 9);
    if (err)
    return err;
    err = regmap_write(as_ctrl.regmap_ctrl,
    REG_SPI_CTRL_MANUAL_EN, SPI_CTRL_MANUAL_EN);
    if (err)
    return err;
    break;
    }
    case SPI_MODE_DMA:
    err = regmap_write(as_ctrl.regmap_ctrl,
    REG_SPI_CTRL_NFI2SPI_EN,
    SPI_CTRL_MANUAL_EN);
    if (err < 0)
    return err;
    err = regmap_write(as_ctrl.regmap_ctrl,
    REG_SPI_CTRL_MTX_MODE_TOG, 0x0);
    if (err < 0)
    return err;
    err = regmap_write(as_ctrl.regmap_ctrl,
    REG_SPI_CTRL_MANUAL_EN, 0x0);
    if (err < 0)
    return err;
    break;
    case SPI_MODE_AUTO:
    default:
    break;
    }
    return regmap_write(as_ctrl.regmap_ctrl, REG_SPI_CTRL_DUMMY, 0);
    }
    static int airoha_snand_write_data(struct airoha_snand_ctrl *as_ctrl,
    const u8 *data, int len, int buswidth)
    {
    int i, data_len;
    u8 cmd;
    switch (buswidth) {
    case 0:
    case 1:
    cmd = SNAND_FIFO_TX_BUSWIDTH_SINGLE;
    break;
    case 2:
    cmd = SNAND_FIFO_TX_BUSWIDTH_DUAL;
    break;
    case 4:
    cmd = SNAND_FIFO_TX_BUSWIDTH_QUAD;
    break;
    default:
    return -EINVAL;
    }
    for (i = 0; i < len; i += data_len) {
    int err;
    data_len = min(len - i, SPI_MAX_TRANSFER_SIZE);
    err = airoha_snand_set_fifo_op(as_ctrl, cmd, data_len);
    if (err)
    return err;
    err = airoha_snand_write_data_to_fifo(as_ctrl, &data[i],
    data_len);
    if (err < 0)
    return err;
    }
    return 0;
    }
    static int airoha_snand_read_data(struct airoha_snand_ctrl *as_ctrl,
    u8 *data, int len, int buswidth)
    {
    int i, data_len;
    u8 cmd;
    switch (buswidth) {
    case 0:
    case 1:
    cmd = SNAND_FIFO_RX_BUSWIDTH_SINGLE;
    break;
    case 2:
    cmd = SNAND_FIFO_RX_BUSWIDTH_DUAL;
    break;
    case 4:
    cmd = SNAND_FIFO_RX_BUSWIDTH_QUAD;
    break;
    default:
    return -EINVAL;
    }
    for (i = 0; i < len; i += data_len) {
    int err;
    data_len = min(len - i, SPI_MAX_TRANSFER_SIZE);
    err = airoha_snand_set_fifo_op(as_ctrl, cmd, data_len);
    if (err)
    return err;
    err = airoha_snand_read_data_from_fifo(as_ctrl, &data[i],
    data_len);
    if (err < 0)
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn airoha_snand_nfi_init(as_ctrl: *mut airoha_snand_ctrl) -> c_int {
    static int airoha_snand_nfi_init(struct airoha_snand_ctrl *as_ctrl)
    {
    int err;
// switch to SNFI mode
    err = regmap_write(as_ctrl.regmap_nfi, REG_SPI_NFI_SNF_NFI_CNFG,
    SPI_NFI_SPI_MODE);
    if (err)
    return err;
// Enable DMA
    return regmap_update_bits(as_ctrl.regmap_nfi, REG_SPI_NFI_INTR_EN,
    SPI_NFI_ALL_IRQ_EN, SPI_NFI_AHB_DONE_EN);
    }
#[no_mangle]
unsafe extern "C" fn airoha_snand_is_page_ops(op: *const spi_mem_op) -> bool {
    static bool airoha_snand_is_page_ops(const struct spi_mem_op *op)
    {
    if (op.addr.nbytes != 2)
    return false;
    if (op.addr.buswidth != 1 && op.addr.buswidth != 2 &&
    op.addr.buswidth != 4)
    return false;
    switch (op.data.dir) {
    case SPI_MEM_DATA_IN:
    if (op.dummy.nbytes * BITS_PER_BYTE / op.dummy.buswidth > 0xf)
    return false;
// quad in / quad out
    if (op.addr.buswidth == 4)
    return op.data.buswidth == 4;
    if (op.addr.buswidth == 2)
    return op.data.buswidth == 2;
// standard spi
    return op.data.buswidth == 4 || op.data.buswidth == 2 ||
    op.data.buswidth == 1;
    case SPI_MEM_DATA_OUT:
    return !op.dummy.nbytes && op.addr.buswidth == 1 &&
    (op.data.buswidth == 4 || op.data.buswidth == 1);
    default:
    return false;
    }
    }
    static bool airoha_snand_supports_op(struct spi_mem *mem,
    const struct spi_mem_op *op)
    {
    if (!spi_mem_default_supports_op(mem, op))
    return false;
    if (op.cmd.buswidth != 1)
    return false;
    if (airoha_snand_is_page_ops(op))
    return true;
    return (!op.addr.nbytes || op.addr.buswidth == 1) &&
    (!op.dummy.nbytes || op.dummy.buswidth == 1) &&
    (!op.data.nbytes || op.data.buswidth == 1);
    }
#[no_mangle]
unsafe extern "C" fn airoha_snand_dirmap_create(desc: *mut spi_mem_dirmap_desc) -> c_int {
    static int airoha_snand_dirmap_create(struct spi_mem_dirmap_desc *desc)
    {
    u8 *txrx_buf = spi_get_ctldata(desc.mem.spi);
    if (!txrx_buf)
    return -EINVAL;
    if (desc.info.offset + desc.info.length > U32_MAX)
    return -EINVAL;
// continuous reading is not supported
    if (desc.info.length > SPI_NAND_CACHE_SIZE)
    return -E2BIG;
    if (!airoha_snand_supports_op(desc.mem, desc.info.op_tmpl))
    return -EOPNOTSUPP;
    return 0;
    }
    static ssize_t airoha_snand_dirmap_read(struct spi_mem_dirmap_desc *desc,
    u64 offs, size_t len, void *buf)
    {
    struct spi_device *spi = desc.mem.spi;
    struct airoha_snand_ctrl *as_ctrl;
    u8 *txrx_buf = spi_get_ctldata(spi);
    dma_addr_t dma_addr;
    u32 val, rd_mode, opcode;
    size_t bytes;
    int err;
    as_ctrl = spi_controller_get_devdata(spi.controller);
// minimum oob size is 64
    bytes = round_up(offs + len, 64);
//
// DUALIO and QUADIO opcodes are not supported by the spi controller,
// replace them with supported opcodes.
//
    opcode = desc.info.op_tmpl.cmd.opcode;
    switch (opcode) {
    case SPI_NAND_OP_READ_FROM_CACHE_SINGLE:
    case SPI_NAND_OP_READ_FROM_CACHE_SINGLE_FAST:
    rd_mode = 0;
    break;
    case SPI_NAND_OP_READ_FROM_CACHE_DUAL:
    case SPI_NAND_OP_READ_FROM_CACHE_DUALIO:
    opcode = SPI_NAND_OP_READ_FROM_CACHE_DUAL;
    rd_mode = 1;
    break;
    case SPI_NAND_OP_READ_FROM_CACHE_QUAD:
    case SPI_NAND_OP_READ_FROM_CACHE_QUADIO:
    opcode = SPI_NAND_OP_READ_FROM_CACHE_QUAD;
    rd_mode = 2;
    break;
    default:
// unknown opcode
    return -EOPNOTSUPP;
    }
    err = airoha_snand_set_mode(as_ctrl, SPI_MODE_DMA);
    if (err < 0)
    return err;
// NFI reset
    err = regmap_write(as_ctrl.regmap_nfi, REG_SPI_NFI_CON,
    SPI_NFI_FIFO_FLUSH | SPI_NFI_RST);
    if (err)
    goto error_dma_mode_off;
// NFI configure:
// - No AutoFDM (custom sector size (SECCUS) register will be used)
// - No SoC's hardware ECC (flash internal ECC will be used)
// - Use burst mode (faster, but requires 16 byte alignment for addresses)
// - Setup for reading (SPI_NFI_READ_MODE)
// - Setup reading command: FIELD_PREP(SPI_NFI_OPMODE, 6)
// - Use DMA instead of PIO for data reading
//
    err = regmap_update_bits(as_ctrl.regmap_nfi, REG_SPI_NFI_CNFG,
    SPI_NFI_DMA_MODE |
    SPI_NFI_READ_MODE |
    SPI_NFI_DMA_BURST_EN |
    SPI_NFI_HW_ECC_EN |
    SPI_NFI_AUTO_FDM_EN |
    SPI_NFI_OPMODE,
    SPI_NFI_DMA_MODE |
    SPI_NFI_READ_MODE |
    SPI_NFI_DMA_BURST_EN |
    FIELD_PREP(SPI_NFI_OPMODE, 6));
    if (err)
    goto error_dma_mode_off;
// Set number of sector will be read
    err = regmap_update_bits(as_ctrl.regmap_nfi, REG_SPI_NFI_CON,
    SPI_NFI_SEC_NUM,
    FIELD_PREP(SPI_NFI_SEC_NUM, 1));
    if (err)
    goto error_dma_mode_off;
// Set custom sector size
    err = regmap_update_bits(as_ctrl.regmap_nfi, REG_SPI_NFI_SECCUS_SIZE,
    SPI_NFI_CUS_SEC_SIZE |
    SPI_NFI_CUS_SEC_SIZE_EN,
    FIELD_PREP(SPI_NFI_CUS_SEC_SIZE, bytes) |
    SPI_NFI_CUS_SEC_SIZE_EN);
    if (err)
    goto error_dma_mode_off;
    dma_addr = dma_map_single(as_ctrl.dev, txrx_buf, SPI_NAND_CACHE_SIZE,
    DMA_FROM_DEVICE);
    err = dma_mapping_error(as_ctrl.dev, dma_addr);
    if (err)
    goto error_dma_mode_off;
// set dma addr
    err = regmap_write(as_ctrl.regmap_nfi, REG_SPI_NFI_STRADDR,
    dma_addr);
    if (err)
    goto error_dma_unmap;
//
// Setup transfer length
// ---------------------
// The following rule MUST be met:
// transfer_length =
// = NFI_SNF_MISC_CTL2.read_data_byte_number =
// = NFI_CON.sector_number * NFI_SECCUS.custom_sector_size
//
    err = regmap_update_bits(as_ctrl.regmap_nfi,
    REG_SPI_NFI_SNF_MISC_CTL2,
    SPI_NFI_READ_DATA_BYTE_NUM,
    FIELD_PREP(SPI_NFI_READ_DATA_BYTE_NUM, bytes));
    if (err)
    goto error_dma_unmap;
// set read command
    err = regmap_write(as_ctrl.regmap_nfi, REG_SPI_NFI_RD_CTL2,
    FIELD_PREP(SPI_NFI_DATA_READ_CMD, opcode));
    if (err)
    goto error_dma_unmap;
// set read mode
    err = regmap_write(as_ctrl.regmap_nfi, REG_SPI_NFI_SNF_MISC_CTL,
    FIELD_PREP(SPI_NFI_DATA_READ_WR_MODE, rd_mode));
    if (err)
    goto error_dma_unmap;
// set read addr: zero page offset + descriptor read offset
    err = regmap_write(as_ctrl.regmap_nfi, REG_SPI_NFI_RD_CTL3,
    desc.info.offset);
    if (err)
    goto error_dma_unmap;
    err = regmap_write(as_ctrl.regmap_nfi, REG_SPI_NFI_CMD, 0x0);
    if (err)
    goto error_dma_unmap;
// trigger dma reading
    err = regmap_clear_bits(as_ctrl.regmap_nfi, REG_SPI_NFI_CON,
    SPI_NFI_RD_TRIG);
    if (err)
    goto error_dma_unmap;
    err = regmap_set_bits(as_ctrl.regmap_nfi, REG_SPI_NFI_CON,
    SPI_NFI_RD_TRIG);
    if (err)
    goto error_dma_unmap;
    err = regmap_read_poll_timeout(as_ctrl.regmap_nfi,
    REG_SPI_NFI_SNF_STA_CTL1, val,
    (val & SPI_NFI_READ_FROM_CACHE_DONE),
    0, 1 * USEC_PER_SEC);
    if (err)
    goto error_dma_unmap;
//
// SPI_NFI_READ_FROM_CACHE_DONE bit must be written at the end
// of dirmap_read operation even if it is already set.
//
    err = regmap_write_bits(as_ctrl.regmap_nfi, REG_SPI_NFI_SNF_STA_CTL1,
    SPI_NFI_READ_FROM_CACHE_DONE,
    SPI_NFI_READ_FROM_CACHE_DONE);
    if (err)
    goto error_dma_unmap;
    err = regmap_read_poll_timeout(as_ctrl.regmap_nfi, REG_SPI_NFI_INTR,
    val, (val & SPI_NFI_AHB_DONE), 0,
    1 * USEC_PER_SEC);
    if (err)
    goto error_dma_unmap;
// DMA read need delay for data ready from controller to DRAM
    udelay(1);
    dma_unmap_single(as_ctrl.dev, dma_addr, SPI_NAND_CACHE_SIZE,
    DMA_FROM_DEVICE);
    err = airoha_snand_set_mode(as_ctrl, SPI_MODE_MANUAL);
    if (err < 0)
    return err;
    memcpy(buf, txrx_buf + offs, len);
    return len;
    error_dma_unmap:
    dma_unmap_single(as_ctrl.dev, dma_addr, SPI_NAND_CACHE_SIZE,
    DMA_FROM_DEVICE);
    error_dma_mode_off:
    airoha_snand_set_mode(as_ctrl, SPI_MODE_MANUAL);
    return err;
    }
    static ssize_t airoha_snand_dirmap_write(struct spi_mem_dirmap_desc *desc,
    u64 offs, size_t len, const void *buf)
    {
    struct spi_device *spi = desc.mem.spi;
    u8 *txrx_buf = spi_get_ctldata(spi);
    struct airoha_snand_ctrl *as_ctrl;
    dma_addr_t dma_addr;
    u32 wr_mode, val, opcode;
    size_t bytes;
    int err;
    as_ctrl = spi_controller_get_devdata(spi.controller);
// minimum oob size is 64
    bytes = round_up(offs + len, 64);
    opcode = desc.info.op_tmpl.cmd.opcode;
    switch (opcode) {
    case SPI_NAND_OP_PROGRAM_LOAD_SINGLE:
    case SPI_NAND_OP_PROGRAM_LOAD_RAMDOM_SINGLE:
    wr_mode = 0;
    break;
    case SPI_NAND_OP_PROGRAM_LOAD_QUAD:
    case SPI_NAND_OP_PROGRAM_LOAD_RAMDON_QUAD:
    wr_mode = 2;
    break;
    default:
// unknown opcode
    return -EOPNOTSUPP;
    }
    if (offs > 0)
    memset(txrx_buf, 0xff, offs);
    memcpy(txrx_buf + offs, buf, len);
    if (bytes > offs + len)
    memset(txrx_buf + offs + len, 0xff, bytes - offs - len);
    err = airoha_snand_set_mode(as_ctrl, SPI_MODE_DMA);
    if (err < 0)
    return err;
// NFI reset
    err = regmap_write(as_ctrl.regmap_nfi, REG_SPI_NFI_CON,
    SPI_NFI_FIFO_FLUSH | SPI_NFI_RST);
    if (err)
    goto error_dma_mode_off;
//
// NFI configure:
// - No AutoFDM (custom sector size (SECCUS) register will be used)
// - No SoC's hardware ECC (flash internal ECC will be used)
// - Use burst mode (faster, but requires 16 byte alignment for addresses)
// - Setup for writing (SPI_NFI_READ_MODE bit is cleared)
// - Setup writing command: FIELD_PREP(SPI_NFI_OPMODE, 3)
// - Use DMA instead of PIO for data writing
//
    err = regmap_update_bits(as_ctrl.regmap_nfi, REG_SPI_NFI_CNFG,
    SPI_NFI_DMA_MODE |
    SPI_NFI_READ_MODE |
    SPI_NFI_DMA_BURST_EN |
    SPI_NFI_HW_ECC_EN |
    SPI_NFI_AUTO_FDM_EN |
    SPI_NFI_OPMODE,
    SPI_NFI_DMA_MODE |
    SPI_NFI_DMA_BURST_EN |
    FIELD_PREP(SPI_NFI_OPMODE, 3));
    if (err)
    goto error_dma_mode_off;
// Set number of sector will be written
    err = regmap_update_bits(as_ctrl.regmap_nfi, REG_SPI_NFI_CON,
    SPI_NFI_SEC_NUM,
    FIELD_PREP(SPI_NFI_SEC_NUM, 1));
    if (err)
    goto error_dma_mode_off;
// Set custom sector size
    err = regmap_update_bits(as_ctrl.regmap_nfi, REG_SPI_NFI_SECCUS_SIZE,
    SPI_NFI_CUS_SEC_SIZE |
    SPI_NFI_CUS_SEC_SIZE_EN,
    FIELD_PREP(SPI_NFI_CUS_SEC_SIZE, bytes) |
    SPI_NFI_CUS_SEC_SIZE_EN);
    if (err)
    goto error_dma_mode_off;
    dma_addr = dma_map_single(as_ctrl.dev, txrx_buf, SPI_NAND_CACHE_SIZE,
    DMA_TO_DEVICE);
    err = dma_mapping_error(as_ctrl.dev, dma_addr);
    if (err)
    goto error_dma_mode_off;
// set dma addr
    err = regmap_write(as_ctrl.regmap_nfi, REG_SPI_NFI_STRADDR,
    dma_addr);
    if (err)
    goto error_dma_unmap;
//
// Setup transfer length
// ---------------------
// The following rule MUST be met:
// transfer_length =
// = NFI_SNF_MISC_CTL2.write_data_byte_number =
// = NFI_CON.sector_number * NFI_SECCUS.custom_sector_size
//
    err = regmap_update_bits(as_ctrl.regmap_nfi,
    REG_SPI_NFI_SNF_MISC_CTL2,
    SPI_NFI_PROG_LOAD_BYTE_NUM,
    FIELD_PREP(SPI_NFI_PROG_LOAD_BYTE_NUM, bytes));
    if (err)
    goto error_dma_unmap;
// set write command
    err = regmap_write(as_ctrl.regmap_nfi, REG_SPI_NFI_PG_CTL1,
    FIELD_PREP(SPI_NFI_PG_LOAD_CMD, opcode));
    if (err)
    goto error_dma_unmap;
// set write mode
    err = regmap_write(as_ctrl.regmap_nfi, REG_SPI_NFI_SNF_MISC_CTL,
    FIELD_PREP(SPI_NFI_DATA_READ_WR_MODE, wr_mode));
    if (err)
    goto error_dma_unmap;
// set write addr: zero page offset + descriptor write offset
    err = regmap_write(as_ctrl.regmap_nfi, REG_SPI_NFI_PG_CTL2,
    desc.info.offset);
    if (err)
    goto error_dma_unmap;
    err = regmap_write(as_ctrl.regmap_nfi, REG_SPI_NFI_CMD, 0x80);
    if (err)
    goto error_dma_unmap;
// trigger dma writing
    err = regmap_clear_bits(as_ctrl.regmap_nfi, REG_SPI_NFI_CON,
    SPI_NFI_WR_TRIG);
    if (err)
    goto error_dma_unmap;
    err = regmap_set_bits(as_ctrl.regmap_nfi, REG_SPI_NFI_CON,
    SPI_NFI_WR_TRIG);
    if (err)
    goto error_dma_unmap;
    err = regmap_read_poll_timeout(as_ctrl.regmap_nfi, REG_SPI_NFI_INTR,
    val, (val & SPI_NFI_AHB_DONE), 0,
    1 * USEC_PER_SEC);
    if (err)
    goto error_dma_unmap;
    err = regmap_read_poll_timeout(as_ctrl.regmap_nfi,
    REG_SPI_NFI_SNF_STA_CTL1, val,
    (val & SPI_NFI_LOAD_TO_CACHE_DONE),
    0, 1 * USEC_PER_SEC);
    if (err)
    goto error_dma_unmap;
//
// SPI_NFI_LOAD_TO_CACHE_DONE bit must be written at the end
// of dirmap_write operation even if it is already set.
//
    err = regmap_write_bits(as_ctrl.regmap_nfi, REG_SPI_NFI_SNF_STA_CTL1,
    SPI_NFI_LOAD_TO_CACHE_DONE,
    SPI_NFI_LOAD_TO_CACHE_DONE);
    if (err)
    goto error_dma_unmap;
    dma_unmap_single(as_ctrl.dev, dma_addr, SPI_NAND_CACHE_SIZE,
    DMA_TO_DEVICE);
    err = airoha_snand_set_mode(as_ctrl, SPI_MODE_MANUAL);
    if (err < 0)
    return err;
    return len;
    error_dma_unmap:
    dma_unmap_single(as_ctrl.dev, dma_addr, SPI_NAND_CACHE_SIZE,
    DMA_TO_DEVICE);
    error_dma_mode_off:
    airoha_snand_set_mode(as_ctrl, SPI_MODE_MANUAL);
    return err;
    }
    static int airoha_snand_exec_op(struct spi_mem *mem,
    const struct spi_mem_op *op)
    {
    struct airoha_snand_ctrl *as_ctrl;
    int op_len, addr_len, dummy_len;
    u8 buf[20], *data;
    int i, err;
    as_ctrl = spi_controller_get_devdata(mem.spi.controller);
    op_len = op.cmd.nbytes;
    addr_len = op.addr.nbytes;
    dummy_len = op.dummy.nbytes;
    if (op_len + dummy_len + addr_len > sizeof(buf))
    return -EIO;
    data = buf;
    for (i = 0; i < op_len; i++)
// data++ = op->cmd.opcode >> (8 * (op_len - i - 1));
    for (i = 0; i < addr_len; i++)
// data++ = op->addr.val >> (8 * (addr_len - i - 1));
    for (i = 0; i < dummy_len; i++)
// data++ = 0xff;
// switch to manual mode
    err = airoha_snand_set_mode(as_ctrl, SPI_MODE_MANUAL);
    if (err < 0)
    return err;
    err = airoha_snand_set_cs(as_ctrl, SPI_CHIP_SEL_LOW);
    if (err < 0)
    return err;
// opcode
    data = buf;
    err = airoha_snand_write_data(as_ctrl, data, op_len,
    op.cmd.buswidth);
    if (err)
    return err;
// addr part
    data += op_len;
    if (addr_len) {
    err = airoha_snand_write_data(as_ctrl, data, addr_len,
    op.addr.buswidth);
    if (err)
    return err;
    }
// dummy
    data += addr_len;
    if (dummy_len) {
    err = airoha_snand_write_data(as_ctrl, data, dummy_len,
    op.dummy.buswidth);
    if (err)
    return err;
    }
// data
    if (op.data.nbytes) {
    if (op.data.dir == SPI_MEM_DATA_IN)
    err = airoha_snand_read_data(as_ctrl, op.data.buf.in,
    op.data.nbytes,
    op.data.buswidth);
    else
    err = airoha_snand_write_data(as_ctrl, op.data.buf.out,
    op.data.nbytes,
    op.data.buswidth);
    if (err)
    return err;
    }
    return airoha_snand_set_cs(as_ctrl, SPI_CHIP_SEL_HIGH);
    }
    static const struct spi_controller_mem_ops airoha_snand_mem_ops = {
    .supports_op = airoha_snand_supports_op,
    .exec_op = airoha_snand_exec_op,
    .dirmap_create = airoha_snand_dirmap_create,
    .dirmap_read = airoha_snand_dirmap_read,
    .dirmap_write = airoha_snand_dirmap_write,
    };
    static const struct spi_controller_mem_ops airoha_snand_nodma_mem_ops = {
    .supports_op = airoha_snand_supports_op,
    .exec_op = airoha_snand_exec_op,
    };
#[no_mangle]
unsafe extern "C" fn airoha_snand_setup(spi: *mut spi_device) -> c_int {
    static int airoha_snand_setup(struct spi_device *spi)
    {
    struct airoha_snand_ctrl *as_ctrl;
    u8 *txrx_buf;
// prepare device buffer
    as_ctrl = spi_controller_get_devdata(spi.controller);
    txrx_buf = devm_kzalloc(as_ctrl.dev, SPI_NAND_CACHE_SIZE,
    GFP_KERNEL);
    if (!txrx_buf)
    return -ENOMEM;
    spi_set_ctldata(spi, txrx_buf);
    return 0;
    }
    static const struct regmap_config spi_ctrl_regmap_config = {
    .name		= "ctrl",
    .reg_bits	= 32,
    .val_bits	= 32,
    .reg_stride	= 4,
    .max_register	= REG_SPI_CTRL_NFI2SPI_EN,
    };
    static const struct regmap_config spi_nfi_regmap_config = {
    .name		= "nfi",
    .reg_bits	= 32,
    .val_bits	= 32,
    .reg_stride	= 4,
    .max_register	= REG_SPI_NFI_SNF_NFI_CNFG,
    };
    static const struct of_device_id airoha_snand_ids[] = {
    { .compatible	= "airoha,en7581-snand" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, airoha_snand_ids);
#[no_mangle]
unsafe extern "C" fn airoha_snand_probe(pdev: *mut platform_device) -> c_int {
    static int airoha_snand_probe(struct platform_device *pdev)
    {
    struct airoha_snand_ctrl *as_ctrl;
    struct device *dev = &pdev.dev;
    struct spi_controller *ctrl;
    let mut dma_enable: bool = true;
    void __iomem *base;
    u32 sfc_strap;
    int err;
    ctrl = devm_spi_alloc_host(dev, sizeof(*as_ctrl));
    if (!ctrl)
    return -ENOMEM;
    as_ctrl = spi_controller_get_devdata(ctrl);
    as_ctrl.dev = dev;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    as_ctrl.regmap_ctrl = devm_regmap_init_mmio(dev, base,
    &spi_ctrl_regmap_config);
    if (IS_ERR(as_ctrl.regmap_ctrl))
    return dev_err_probe(dev, PTR_ERR(as_ctrl.regmap_ctrl),
    "failed to init spi ctrl regmap\n");
    base = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(base))
    return PTR_ERR(base);
    as_ctrl.regmap_nfi = devm_regmap_init_mmio(dev, base,
    &spi_nfi_regmap_config);
    if (IS_ERR(as_ctrl.regmap_nfi))
    return dev_err_probe(dev, PTR_ERR(as_ctrl.regmap_nfi),
    "failed to init spi nfi regmap\n");
    as_ctrl.spi_clk = devm_clk_get_enabled(dev, "spi");
    if (IS_ERR(as_ctrl.spi_clk))
    return dev_err_probe(dev, PTR_ERR(as_ctrl.spi_clk),
    "unable to get spi clk\n");
    if (device_is_compatible(dev, "airoha,en7523-snand")) {
    err = regmap_read(as_ctrl.regmap_ctrl,
    REG_SPI_CTRL_SFC_STRAP, &sfc_strap);
    if (err)
    return err;
    if (!(sfc_strap & 0x04)) {
    dma_enable = false;
    dev_warn(dev, "Detected booting in RESERVED mode (UART_TXD was short to GND).\n");
    dev_warn(dev, "This mode is known for incorrect DMA reading of some flashes.\n");
    dev_warn(dev, "Much slower PIO mode will be used to prevent flash data damage.\n");
    dev_warn(dev, "Unplug UART cable and power cycle board to get full performance.\n");
    }
    }
    err = dma_set_mask(as_ctrl.dev, DMA_BIT_MASK(32));
    if (err)
    return err;
    ctrl.num_chipselect = 2;
    ctrl.mem_ops = dma_enable ? &airoha_snand_mem_ops
    : &airoha_snand_nodma_mem_ops;
    ctrl.bits_per_word_mask = SPI_BPW_MASK(8);
    ctrl.mode_bits = SPI_RX_DUAL;
    ctrl.setup = airoha_snand_setup;
    err = airoha_snand_nfi_init(as_ctrl);
    if (err)
    return err;
    return devm_spi_register_controller(dev, ctrl);
    }
    static struct platform_driver airoha_snand_driver = {
    .driver = {
    .name = "airoha-spi",
    .of_match_table = airoha_snand_ids,
    },
    .probe = airoha_snand_probe,
    };
    module_platform_driver(airoha_snand_driver);
    MODULE_DESCRIPTION("Airoha SPI-NAND Flash Controller Driver");
    MODULE_AUTHOR("Lorenzo Bianconi <lorenzo@kernel.org>");
    MODULE_AUTHOR("Ray Liu <ray.liu@airoha.com>");
    MODULE_LICENSE("GPL");
