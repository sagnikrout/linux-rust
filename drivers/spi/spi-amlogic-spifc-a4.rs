//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-amlogic-spifc-a4.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
//
// Copyright (C) 2025 Amlogic, Inc. All rights reserved
//
// Driver for the SPI Mode of Amlogic Flash Controller
// Authors:
// Liang Yang <liang.yang@amlogic.com>
// Feng Chen <feng.chen@amlogic.com>
// Xianwei Zhao <xianwei.zhao@amlogic.com>
//

pub const SFC_CMD: c_uint = 0x00;
pub const SFC_CFG: c_uint = 0x04;
pub const SFC_DADR: c_uint = 0x08;
pub const SFC_IADR: c_uint = 0x0c;
pub const SFC_BUF: c_uint = 0x10;
pub const SFC_INFO: c_uint = 0x14;
pub const SFC_DC: c_uint = 0x18;
pub const SFC_ADR: c_uint = 0x1c;
pub const SFC_DL: c_uint = 0x20;
pub const SFC_DH: c_uint = 0x24;
pub const SFC_CADR: c_uint = 0x28;
pub const SFC_SADR: c_uint = 0x2c;
pub const SFC_RX_IDX: c_uint = 0x34;
pub const SFC_RX_DAT: c_uint = 0x38;
pub const SFC_SPI_CFG: c_uint = 0x40;
// settings in SFC_CMD
// 4 bits support 4 chip select, high false, low select but spi support 2

pub const CS_NONE: c_uint = 0xf;
pub const CS_0: c_uint = 0xe;
pub const CS_1: c_uint = 0xd;

pub const DEFAULT_PULLUP_CYCLE: c_int = 2;
pub const CS_SETUP_CYCLE: c_int = 1;
pub const CS_HOLD_CYCLE: c_int = 2;
pub const DEFAULT_BUS_CYCLE: c_int = 4;

pub const RAW_SIZE_BW: c_int = 14;
pub const DMA_ADDR_ALIGN: c_int = 8;
// Bit fields in SFC_SPI_CFG

pub const LANE_MAX: c_uint = 0x3;
// raw ext size[25:14] + raw size[13:0]

// Ecc fields

pub const ECC_UNCORRECTABLE: c_uint = 0x3f;

pub const ECC_BCH8_512: c_int = 1;
pub const ECC_BCH8_1K: c_int = 2;
pub const ECC_BCH8_PARITY_BYTES: c_int = 14;
pub const ECC_BCH8_USER_BYTES: c_int = 2;

pub const ECC_BCH8_STRENGTH: c_int = 8;
pub const ECC_BCH8_DEFAULT_STEP: c_int = 512;

pub const ECC_PER_INFO_BYTE: c_int = 8;
pub const ECC_PATTERN: c_uint = 0x5a;
pub const ECC_BCH_MAX_SECT_SIZE: c_int = 63;
// soft flags for sfc

pub const SFC_DATABUF_SIZE: c_int = 8192;
pub const SFC_INFOBUF_SIZE: c_int = 256;

// !!! PCB and SPI-NAND chip limitations

pub const SFC_BUS_DEFAULT_CLK: c_int = 40000000;
pub const SFC_MAX_CS_NUM: c_int = 2;
// SPI-FLASH R/W operation cmd
pub const SPIFLASH_RD_OCTALIO: c_uint = 0xcb;
pub const SPIFLASH_RD_OCTAL: c_uint = 0x8b;
pub const SPIFLASH_RD_QUADIO: c_uint = 0xeb;
pub const SPIFLASH_RD_QUAD: c_uint = 0x6b;
pub const SPIFLASH_RD_DUALIO: c_uint = 0xbb;
pub const SPIFLASH_RD_DUAL: c_uint = 0x3b;
pub const SPIFLASH_RD_FAST: c_uint = 0x0b;
pub const SPIFLASH_RD: c_uint = 0x03;
pub const SPIFLASH_WR_OCTALIO: c_uint = 0xC2;
pub const SPIFLASH_WR_OCTAL: c_uint = 0x82;
pub const SPIFLASH_WR_QUAD: c_uint = 0x32;
pub const SPIFLASH_WR: c_uint = 0x02;
pub const SPIFLASH_UP_QUAD: c_uint = 0x34;
pub const SPIFLASH_UP: c_uint = 0x84;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_sfc_ecc_cfg {
    pub stepsize: u32,
    pub nsteps: u32,
    pub strength: u32,
    pub oobsize: u32,
    pub bch: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_ecc_stats {
    pub corrected: u32,
    pub bitflips: u32,
    pub failed: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_sfc_caps {
    pub ecc_caps: *mut aml_sfc_ecc_cfg,
    pub num_ecc_caps: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_sfc {
    pub dev: *mut device,
    pub gate_clk: *mut clk,
    pub core_clk: *mut clk,
    pub ctrl: *mut spi_controller,
    pub regmap_base: *mut regmap,
    pub caps: *const aml_sfc_caps,
    pub ecc_eng: nand_ecc_engine,
    pub ecc_stats: aml_ecc_stats,
    pub daddr: dma_addr_t,
    pub iaddr: dma_addr_t,
    pub info_bytes: u32,
    pub bus_rate: u32,
    pub flags: u32,
    pub rx_adj: u32,
    pub cs_sel: u32,
    pub data_buf: *mut u8,
    pub info_buf: *mut __le64,
    pub priv: *mut u8,
}

    static struct aml_sfc_ecc_cfg aml_a113l2_ecc_caps[] = {
    AML_ECC_DATA(512, 8, ECC_BCH8_512),
    AML_ECC_DATA(1024, 8, ECC_BCH8_1K),
    };
    static const struct aml_sfc_caps aml_a113l2_sfc_caps = {
    .ecc_caps = aml_a113l2_ecc_caps,
    .num_ecc_caps = ARRAY_SIZE(aml_a113l2_ecc_caps)
    };
    static struct aml_sfc *nand_to_aml_sfc(struct nand_device *nand)
    {
    struct nand_ecc_engine *eng = nand.ecc.engine;
    return container_of(eng, struct aml_sfc, ecc_eng);
    }
    static inline void *aml_sfc_to_ecc_ctx(struct aml_sfc *sfc)
    {
    return sfc.priv;
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_wait_cmd_finish(sfc: *mut aml_sfc, timeout_ms: u64) -> c_int {
    static int aml_sfc_wait_cmd_finish(struct aml_sfc *sfc, u64 timeout_ms)
    {
    let mut cmd_size: u32 = 0;
    int ret;
//
// The SPINAND flash controller employs a two-stage pipeline:
// 1) command prefetch; 2) command execution.
//
// All commands are stored in the FIFO, with one prefetched for execution.
//
// There are cases where the FIFO is detected as empty, yet a command may
// still be in execution and a prefetched command pending execution.
//
// So, send two idle commands to ensure all previous commands have
// been executed.
//
    regmap_write(sfc.regmap_base, SFC_CMD, CMD_IDLE(sfc.cs_sel, 0));
    regmap_write(sfc.regmap_base, SFC_CMD, CMD_IDLE(sfc.cs_sel, 0));
// Wait for the FIFO to empty.
    ret = regmap_read_poll_timeout(sfc.regmap_base, SFC_CMD, cmd_size,
    !GET_CMD_SIZE(cmd_size),
    10, timeout_ms * 1000);
    if (ret)
    dev_err(sfc.dev, "wait for empty CMD FIFO time out\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_pre_transfer(sfc: *mut aml_sfc, idle_cycle: u32, cs2clk_cycle: u32) -> c_int {
    static int aml_sfc_pre_transfer(struct aml_sfc *sfc, u32 idle_cycle, u32 cs2clk_cycle)
    {
    int ret;
    ret = regmap_write(sfc.regmap_base, SFC_CMD, CMD_IDLE(CS_NONE, idle_cycle));
    if (ret)
    return ret;
    return regmap_write(sfc.regmap_base, SFC_CMD, CMD_IDLE(sfc.cs_sel, cs2clk_cycle));
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_end_transfer(sfc: *mut aml_sfc, clk2cs_cycle: u32) -> c_int {
    static int aml_sfc_end_transfer(struct aml_sfc *sfc, u32 clk2cs_cycle)
    {
    int ret;
    ret = regmap_write(sfc.regmap_base, SFC_CMD, CMD_IDLE(sfc.cs_sel, clk2cs_cycle));
    if (ret)
    return ret;
    return aml_sfc_wait_cmd_finish(sfc, 0);
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_set_bus_width(sfc: *mut aml_sfc, buswidth: u8, mask: u32) -> c_int {
    static int aml_sfc_set_bus_width(struct aml_sfc *sfc, u8 buswidth, u32 mask)
    {
    int i;
    let mut conf: u32 = 0;
    for (i = 0; i <= LANE_MAX; i++) {
    if (buswidth == 1 << i) {
    conf = i << __ffs(mask);
    return regmap_update_bits(sfc.regmap_base, SFC_SPI_CFG,
    mask, conf);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_send_cmd(sfc: *mut aml_sfc, op: *const spi_mem_op) -> c_int {
    static int aml_sfc_send_cmd(struct aml_sfc *sfc, const struct spi_mem_op *op)
    {
    int i, ret;
    u8 val;
    ret = aml_sfc_set_bus_width(sfc, op.cmd.buswidth, CMD_LANE);
    if (ret)
    return ret;
    for (i = 0; i < op.cmd.nbytes; i++) {
    val = (op.cmd.opcode >> ((op.cmd.nbytes - i - 1) * 8)) & 0xff;
    ret = regmap_write(sfc.regmap_base, SFC_CMD, CMD_COMMAND(sfc.cs_sel, val));
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_send_addr(sfc: *mut aml_sfc, op: *const spi_mem_op) -> c_int {
    static int aml_sfc_send_addr(struct aml_sfc *sfc, const struct spi_mem_op *op)
    {
    int i, ret;
    u8 val;
    ret = aml_sfc_set_bus_width(sfc, op.addr.buswidth, ADDR_LANE);
    if (ret)
    return ret;
    for (i = 0; i < op.addr.nbytes; i++) {
    val = (op.addr.val >> ((op.addr.nbytes - i - 1) * 8)) & 0xff;
    ret = regmap_write(sfc.regmap_base, SFC_CMD, CMD_ADDR(sfc.cs_sel, val));
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_is_xio_op(op: *const spi_mem_op) -> bool {
    static bool aml_sfc_is_xio_op(const struct spi_mem_op *op)
    {
    switch (op.cmd.opcode) {
    case SPIFLASH_RD_OCTALIO:
    case SPIFLASH_RD_QUADIO:
    case SPIFLASH_RD_DUALIO:
    return true;
    default:
    break;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_send_cmd_addr_dummy(sfc: *mut aml_sfc, op: *const spi_mem_op) -> c_int {
    static int aml_sfc_send_cmd_addr_dummy(struct aml_sfc *sfc, const struct spi_mem_op *op)
    {
    u32 dummy_cycle, cmd;
    int ret;
    ret = aml_sfc_send_cmd(sfc, op);
    if (ret)
    return ret;
    ret = aml_sfc_send_addr(sfc, op);
    if (ret)
    return ret;
    if (op.dummy.nbytes) {
// Dummy buswidth configuration is not supported
    if (aml_sfc_is_xio_op(op))
    dummy_cycle = op.dummy.nbytes * 8 / op.data.buswidth;
    else
    dummy_cycle = op.dummy.nbytes * 8;
    cmd = CMD_DUMMY(sfc.cs_sel, dummy_cycle - 1);
    return regmap_write(sfc.regmap_base, SFC_CMD, cmd);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_is_snand_hwecc_page_op(sfc: *mut aml_sfc, op: *const spi_mem_op) -> bool {
    static bool aml_sfc_is_snand_hwecc_page_op(struct aml_sfc *sfc, const struct spi_mem_op *op)
    {
    switch (op.cmd.opcode) {
// SPINAND read from cache cmd
    case SPIFLASH_RD_QUADIO:
    case SPIFLASH_RD_QUAD:
    case SPIFLASH_RD_DUALIO:
    case SPIFLASH_RD_DUAL:
    case SPIFLASH_RD_FAST:
    case SPIFLASH_RD:
// SPINAND write to cache cmd
    case SPIFLASH_WR_QUAD:
    case SPIFLASH_WR:
    case SPIFLASH_UP_QUAD:
    case SPIFLASH_UP:
    if (sfc.flags & SFC_HWECC)
    return true;
    else
    return false;
    default:
    break;
    }
    return false;
    }
    static int aml_sfc_dma_buffer_setup(struct aml_sfc *sfc, void *databuf,
    int datalen, void *infobuf, int infolen,
    enum dma_data_direction dir)
    {
    let mut cmd: u32 = 0;
    int ret;
    sfc.daddr = dma_map_single(sfc.dev, databuf, datalen, dir);
    ret = dma_mapping_error(sfc.dev, sfc.daddr);
    if (ret) {
    dev_err(sfc.dev, "DMA mapping error\n");
    return ret;
    }
    cmd = CMD_DATA_ADDRL(sfc.daddr);
    ret = regmap_write(sfc.regmap_base, SFC_CMD, cmd);
    if (ret)
    goto out_map_data;
    cmd = CMD_DATA_ADDRH(sfc.daddr);
    ret = regmap_write(sfc.regmap_base, SFC_CMD, cmd);
    if (ret)
    goto out_map_data;
    if (infobuf) {
    sfc.iaddr = dma_map_single(sfc.dev, infobuf, infolen, dir);
    ret = dma_mapping_error(sfc.dev, sfc.iaddr);
    if (ret) {
    dev_err(sfc.dev, "DMA mapping error\n");
    goto out_map_data;
    }
    sfc.info_bytes = infolen;
    cmd = CMD_INFO_ADDRL(sfc.iaddr);
    ret = regmap_write(sfc.regmap_base, SFC_CMD, cmd);
    if (ret)
    goto out_map_info;
    cmd = CMD_INFO_ADDRH(sfc.iaddr);
    ret = regmap_write(sfc.regmap_base, SFC_CMD, cmd);
    if (ret)
    goto out_map_info;
    }
    return 0;
    out_map_info:
    dma_unmap_single(sfc.dev, sfc.iaddr, infolen, dir);
    out_map_data:
    dma_unmap_single(sfc.dev, sfc.daddr, datalen, dir);
    return ret;
    }
    static void aml_sfc_dma_buffer_release(struct aml_sfc *sfc,
    int datalen, int infolen,
    enum dma_data_direction dir)
    {
    dma_unmap_single(sfc.dev, sfc.daddr, datalen, dir);
    if (infolen) {
    dma_unmap_single(sfc.dev, sfc.iaddr, infolen, dir);
    sfc.info_bytes = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_dma_buffer_is_safe(buffer: *const c_void) -> bool {
    static bool aml_sfc_dma_buffer_is_safe(const void *buffer)
    {
    if ((uintptr_t)buffer % DMA_ADDR_ALIGN)
    return false;
    if (virt_addr_valid(buffer))
    return true;
    return false;
    }
    static void *aml_get_dma_safe_input_buf(const struct spi_mem_op *op)
    {
    if (aml_sfc_dma_buffer_is_safe(op.data.buf.in))
    return op.data.buf.in;
    return kzalloc(op.data.nbytes, GFP_KERNEL);
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_put_dma_safe_input_buf(op: *const spi_mem_op, buf: *mut c_void) {
    static void aml_sfc_put_dma_safe_input_buf(const struct spi_mem_op *op, void *buf)
    {
    if (WARN_ON(op.data.dir != SPI_MEM_DATA_IN) || WARN_ON(!buf))
    return;
    if (buf == op.data.buf.in)
    return;
    memcpy(op.data.buf.in, buf, op.data.nbytes);
    kfree(buf);
    }
    static void *aml_sfc_get_dma_safe_output_buf(const struct spi_mem_op *op)
    {
    if (aml_sfc_dma_buffer_is_safe(op.data.buf.out))
    return (void *)op.data.buf.out;
    return kmemdup(op.data.buf.out, op.data.nbytes, GFP_KERNEL);
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_put_dma_safe_output_buf(op: *const spi_mem_op, buf: *const c_void) {
    static void aml_sfc_put_dma_safe_output_buf(const struct spi_mem_op *op, const void *buf)
    {
    if (WARN_ON(op.data.dir != SPI_MEM_DATA_OUT) || WARN_ON(!buf))
    return;
    if (buf != op.data.buf.out)
    kfree(buf);
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_cal_timeout_cycle(sfc: *mut aml_sfc, op: *const spi_mem_op) -> u64 {
    static u64 aml_sfc_cal_timeout_cycle(struct aml_sfc *sfc, const struct spi_mem_op *op)
    {
    u64 ms;
// For each byte we wait for (8 cycles / buswidth) of the SPI clock.
    ms = 8 * MSEC_PER_SEC * op.data.nbytes / op.data.buswidth;
    do_div(ms, sfc.bus_rate / DEFAULT_BUS_CYCLE);
//
// Double the value and add a 200 ms tolerance to compensate for
// the impact of specific CS hold time, CS setup time sequences,
// controller burst gaps, and other related timing variations.
//
    ms += ms + 200;
    if (ms > UINT_MAX)
    ms = UINT_MAX;
    return ms;
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_check_ecc_pages_valid(sfc: *mut aml_sfc, raw: bool) {
    static void aml_sfc_check_ecc_pages_valid(struct aml_sfc *sfc, bool raw)
    {
    struct aml_sfc_ecc_cfg *ecc_cfg;
    __le64 *info;
    int ret;
    info = sfc.info_buf;
    ecc_cfg = aml_sfc_to_ecc_ctx(sfc);
    info += raw ? 0 : ecc_cfg.nsteps - 1;
    do {
    usleep_range(10, 15);
// info is updated by nfc dma engine
    smp_rmb();
    dma_sync_single_for_cpu(sfc.dev, sfc.iaddr, sfc.info_bytes,
    DMA_FROM_DEVICE);
    ret = le64_to_cpu(*info) & ECC_COMPLETE;
    } while (!ret);
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_raw_io_op(sfc: *mut aml_sfc, op: *const spi_mem_op) -> c_int {
    static int aml_sfc_raw_io_op(struct aml_sfc *sfc, const struct spi_mem_op *op)
    {
    void *buf = core::ptr::null_mut();
    int ret;
    let mut is_datain: bool = false;
    let mut cmd: u32 = 0, conf;
    u64 timeout_ms;
    if (!op.data.nbytes)
    goto end_xfer;
    conf = (op.data.nbytes >> RAW_SIZE_BW) << __ffs(RAW_EXT_SIZE);
    ret = regmap_update_bits(sfc.regmap_base, SFC_SPI_CFG, RAW_EXT_SIZE, conf);
    if (ret)
    goto err_out;
    if (op.data.dir == SPI_MEM_DATA_IN) {
    is_datain = true;
    buf = aml_get_dma_safe_input_buf(op);
    if (!buf) {
    ret = -ENOMEM;
    goto err_out;
    }
    cmd |= CMD_NAND2MEM(0, (op.data.nbytes & RAW_SIZE));
    } else if (op.data.dir == SPI_MEM_DATA_OUT) {
    is_datain = false;
    buf = aml_sfc_get_dma_safe_output_buf(op);
    if (!buf) {
    ret = -ENOMEM;
    goto err_out;
    }
    cmd |= CMD_MEM2NAND(0, (op.data.nbytes & RAW_SIZE));
    } else {
    goto end_xfer;
    }
    ret = aml_sfc_dma_buffer_setup(sfc, buf, op.data.nbytes,
    is_datain ? sfc.info_buf : core::ptr::null_mut(),
    is_datain ? ECC_PER_INFO_BYTE : 0,
    is_datain ? DMA_FROM_DEVICE : DMA_TO_DEVICE);
    if (ret)
    goto err_out;
    ret = regmap_write(sfc.regmap_base, SFC_CMD, cmd);
    if (ret)
    goto err_out;
    timeout_ms = aml_sfc_cal_timeout_cycle(sfc, op);
    ret = aml_sfc_wait_cmd_finish(sfc, timeout_ms);
    if (ret)
    goto err_out;
    if (is_datain)
    aml_sfc_check_ecc_pages_valid(sfc, 1);
    if (op.data.dir == SPI_MEM_DATA_IN)
    aml_sfc_put_dma_safe_input_buf(op, buf);
#[no_mangle]
pub unsafe extern "C" fn if(SPI_MEM_DATA_OUT: op->data.dir ==) -> else {
    else if (op.data.dir == SPI_MEM_DATA_OUT)
    aml_sfc_put_dma_safe_output_buf(op, buf);
    aml_sfc_dma_buffer_release(sfc, op.data.nbytes,
    is_datain ? ECC_PER_INFO_BYTE : 0,
    is_datain ? DMA_FROM_DEVICE : DMA_TO_DEVICE);
    end_xfer:
    return aml_sfc_end_transfer(sfc, CS_HOLD_CYCLE);
    err_out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_set_user_byte(sfc: *mut aml_sfc, info_buf: *mut __le64, oob_buf: *mut u8, auto_oob: bool) {
    static void aml_sfc_set_user_byte(struct aml_sfc *sfc, __le64 *info_buf, u8 *oob_buf, bool auto_oob)
    {
    struct aml_sfc_ecc_cfg *ecc_cfg;
    __le64 *info;
    int i, count, step_size;
    ecc_cfg = aml_sfc_to_ecc_ctx(sfc);
    step_size = auto_oob ? ECC_BCH8_INFO_BYTES : ECC_BCH8_USER_BYTES;
    for (i = 0, count = 0; i < ecc_cfg.nsteps; i++, count += step_size) {
    info = &info_buf[i];
// info &= cpu_to_le64(~0xffff);
// info |= cpu_to_le64((oob_buf[count + 1] << 8) + oob_buf[count]);
    }
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_get_user_byte(sfc: *mut aml_sfc, info_buf: *mut __le64, oob_buf: *mut u8) {
    static void aml_sfc_get_user_byte(struct aml_sfc *sfc, __le64 *info_buf, u8 *oob_buf)
    {
    struct aml_sfc_ecc_cfg *ecc_cfg;
    __le64 *info;
    int i, count;
    ecc_cfg = aml_sfc_to_ecc_ctx(sfc);
    for (i = 0, count = 0; i < ecc_cfg.nsteps; i++, count += ECC_BCH8_INFO_BYTES) {
    info = &info_buf[i];
    oob_buf[count] = le64_to_cpu(*info);
    oob_buf[count + 1] = le64_to_cpu(*info) >> 8;
    }
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_check_hwecc_status(sfc: *mut aml_sfc, info_buf: *mut __le64) -> c_int {
    static int aml_sfc_check_hwecc_status(struct aml_sfc *sfc, __le64 *info_buf)
    {
    struct aml_sfc_ecc_cfg *ecc_cfg;
    __le64 *info;
    u32 i, max_bitflips = 0, per_sector_bitflips = 0;
    ecc_cfg = aml_sfc_to_ecc_ctx(sfc);
    sfc.ecc_stats.failed = 0;
    sfc.ecc_stats.bitflips = 0;
    sfc.ecc_stats.corrected = 0;
    for (i = 0, info = info_buf; i < ecc_cfg.nsteps; i++, info++) {
    if (ECC_ERR_CNT(le64_to_cpu(*info)) != ECC_UNCORRECTABLE) {
    per_sector_bitflips = ECC_ERR_CNT(le64_to_cpu(*info));
    max_bitflips = max_t(u32, max_bitflips, per_sector_bitflips);
    sfc.ecc_stats.corrected += per_sector_bitflips;
    continue;
    }
    return -EBADMSG;
    }
    return max_bitflips;
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_read_page_hwecc(sfc: *mut aml_sfc, op: *const spi_mem_op) -> c_int {
    static int aml_sfc_read_page_hwecc(struct aml_sfc *sfc, const struct spi_mem_op *op)
    {
    struct aml_sfc_ecc_cfg *ecc_cfg;
    int ret, data_len, info_len;
    u32 page_size, cmd = 0;
    u64 timeout_ms;
    ecc_cfg = aml_sfc_to_ecc_ctx(sfc);
    page_size = ecc_cfg.stepsize * ecc_cfg.nsteps;
    data_len = page_size + ecc_cfg.oobsize;
    info_len = ecc_cfg.nsteps * ECC_PER_INFO_BYTE;
    ret = aml_sfc_dma_buffer_setup(sfc, sfc.data_buf, data_len,
    sfc.info_buf, info_len, DMA_FROM_DEVICE);
    if (ret)
    goto err_out;
    cmd |= CMD_NAND2MEM(ecc_cfg.bch, ecc_cfg.nsteps);
    ret = regmap_write(sfc.regmap_base, SFC_CMD, cmd);
    if (ret)
    goto err_out;
    timeout_ms = aml_sfc_cal_timeout_cycle(sfc, op);
    ret = aml_sfc_wait_cmd_finish(sfc, timeout_ms);
    if (ret)
    goto err_out;
    aml_sfc_check_ecc_pages_valid(sfc, 0);
    aml_sfc_dma_buffer_release(sfc, data_len, info_len, DMA_FROM_DEVICE);
// check ecc status here
    ret = aml_sfc_check_hwecc_status(sfc, sfc.info_buf);
    if (ret < 0)
    sfc.ecc_stats.failed++;
    else
    sfc.ecc_stats.bitflips = ret;
    if (sfc.flags & SFC_DATA_ONLY) {
    memcpy(op.data.buf.in, sfc.data_buf, page_size);
    } else if (sfc.flags & SFC_OOB_ONLY) {
    aml_sfc_get_user_byte(sfc, sfc.info_buf, op.data.buf.in);
    } else if (sfc.flags & SFC_DATA_OOB) {
    memcpy(op.data.buf.in, sfc.data_buf, page_size);
    aml_sfc_get_user_byte(sfc, sfc.info_buf, op.data.buf.in + page_size);
    }
    return aml_sfc_end_transfer(sfc, CS_HOLD_CYCLE);
    err_out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_write_page_hwecc(sfc: *mut aml_sfc, op: *const spi_mem_op) -> c_int {
    static int aml_sfc_write_page_hwecc(struct aml_sfc *sfc, const struct spi_mem_op *op)
    {
    struct aml_sfc_ecc_cfg *ecc_cfg;
    int ret, data_len, info_len;
    u32 page_size, cmd = 0;
    u64 timeout_ms;
    ecc_cfg = aml_sfc_to_ecc_ctx(sfc);
    page_size = ecc_cfg.stepsize * ecc_cfg.nsteps;
    data_len = page_size + ecc_cfg.oobsize;
    info_len = ecc_cfg.nsteps * ECC_PER_INFO_BYTE;
    memset(sfc.info_buf, ECC_PATTERN, ecc_cfg.oobsize);
    memcpy(sfc.data_buf, op.data.buf.out, page_size);
    if (!(sfc.flags & SFC_DATA_ONLY)) {
    if (sfc.flags & SFC_AUTO_OOB)
    aml_sfc_set_user_byte(sfc, sfc.info_buf,
    (u8 *)op.data.buf.out + page_size, 1);
    else
    aml_sfc_set_user_byte(sfc, sfc.info_buf,
    (u8 *)op.data.buf.out + page_size, 0);
    }
    ret = aml_sfc_dma_buffer_setup(sfc, sfc.data_buf, data_len,
    sfc.info_buf, info_len, DMA_TO_DEVICE);
    if (ret)
    goto err_out;
    cmd |= CMD_MEM2NAND(ecc_cfg.bch, ecc_cfg.nsteps);
    ret = regmap_write(sfc.regmap_base, SFC_CMD, cmd);
    if (ret)
    goto err_out;
    timeout_ms = aml_sfc_cal_timeout_cycle(sfc, op);
    ret = aml_sfc_wait_cmd_finish(sfc, timeout_ms);
    if (ret)
    goto err_out;
    aml_sfc_dma_buffer_release(sfc, data_len, info_len, DMA_TO_DEVICE);
    return  aml_sfc_end_transfer(sfc, CS_HOLD_CYCLE);
    err_out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_exec_op(mem: *mut spi_mem, op: *const spi_mem_op) -> c_int {
    static int aml_sfc_exec_op(struct spi_mem *mem, const struct spi_mem_op *op)
    {
    struct aml_sfc *sfc;
    struct spi_device *spi;
    struct aml_sfc_ecc_cfg *ecc_cfg;
    int ret;
    sfc = spi_controller_get_devdata(mem.spi.controller);
    ecc_cfg = aml_sfc_to_ecc_ctx(sfc);
    spi = mem.spi;
    sfc.cs_sel = spi.chip_select[0] ? CS_1 : CS_0;
    dev_dbg(sfc.dev, "cmd:0x%02x - addr:%08llX@%d:%u - dummy:%d:%u - data:%d:%u",
    op.cmd.opcode, op.addr.val, op.addr.buswidth, op.addr.nbytes,
    op.dummy.buswidth, op.dummy.nbytes, op.data.buswidth, op.data.nbytes);
    ret = aml_sfc_pre_transfer(sfc, DEFAULT_PULLUP_CYCLE, CS_SETUP_CYCLE);
    if (ret)
    return ret;
    ret = aml_sfc_send_cmd_addr_dummy(sfc, op);
    if (ret)
    return ret;
    ret = aml_sfc_set_bus_width(sfc, op.data.buswidth, DATA_LANE);
    if (ret)
    return ret;
    if (aml_sfc_is_snand_hwecc_page_op(sfc, op) &&
    ecc_cfg && !(sfc.flags & SFC_RAW_RW)) {
    if (op.data.dir == SPI_MEM_DATA_IN)
    return aml_sfc_read_page_hwecc(sfc, op);
    else
    return aml_sfc_write_page_hwecc(sfc, op);
    }
    return aml_sfc_raw_io_op(sfc, op);
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_adjust_op_size(mem: *mut spi_mem, op: *mut spi_mem_op) -> c_int {
    static int aml_sfc_adjust_op_size(struct spi_mem *mem, struct spi_mem_op *op)
    {
    struct aml_sfc *sfc;
    struct aml_sfc_ecc_cfg *ecc_cfg;
    sfc = spi_controller_get_devdata(mem.spi.controller);
    ecc_cfg = aml_sfc_to_ecc_ctx(sfc);
    if (aml_sfc_is_snand_hwecc_page_op(sfc, op) && ecc_cfg) {
    if (op.data.nbytes > ecc_cfg.stepsize * ECC_BCH_MAX_SECT_SIZE)
    return -EOPNOTSUPP;
    } else if (op.data.nbytes & ~RAW_MAX_RW_SIZE_MASK) {
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static const struct spi_controller_mem_ops aml_sfc_mem_ops = {
    .adjust_op_size = aml_sfc_adjust_op_size,
    .exec_op = aml_sfc_exec_op,
    };
    static int aml_sfc_layout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *oobregion)
    {
    struct nand_device *nand = mtd_to_nanddev(mtd);
    if (section >= nand.ecc.ctx.nsteps)
    return -ERANGE;
    oobregion.offset =  ECC_BCH8_USER_BYTES + (section * ECC_BCH8_INFO_BYTES);
    oobregion.length = ECC_BCH8_PARITY_BYTES;
    return 0;
    }
    static int aml_sfc_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *oobregion)
    {
    struct nand_device *nand = mtd_to_nanddev(mtd);
    if (section >= nand.ecc.ctx.nsteps)
    return -ERANGE;
    oobregion.offset = section * ECC_BCH8_INFO_BYTES;
    oobregion.length = ECC_BCH8_USER_BYTES;
    return 0;
    }
    static const struct mtd_ooblayout_ops aml_sfc_ooblayout_ops = {
    .ecc = aml_sfc_layout_ecc,
    .free = aml_sfc_ooblayout_free,
    };
#[no_mangle]
unsafe extern "C" fn aml_spi_settings(sfc: *mut aml_sfc, spi: *mut spi_device) -> c_int {
    static int aml_spi_settings(struct aml_sfc *sfc, struct spi_device *spi)
    {
    let mut conf: u32 = 0;
    if (spi.mode & SPI_CPHA)
    conf |= CPHA;
    if (spi.mode & SPI_CPOL)
    conf |= CPOL;
    conf |= FIELD_PREP(RXADJ, sfc.rx_adj);
    conf |= EN_HOLD | EN_WP;
    return regmap_update_bits(sfc.regmap_base, SFC_SPI_CFG,
    CPHA | CPOL | RXADJ |
    EN_HOLD | EN_WP, conf);
    }
#[no_mangle]
unsafe extern "C" fn aml_set_spi_clk(sfc: *mut aml_sfc, spi: *mut spi_device) -> c_int {
    static int aml_set_spi_clk(struct aml_sfc *sfc, struct spi_device *spi)
    {
    u32 speed_hz;
    int ret;
    if (spi.max_speed_hz > SFC_MAX_FREQUENCY)
    speed_hz = SFC_MAX_FREQUENCY;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !spi->max_speed_hz) -> else {
    else if (!spi.max_speed_hz)
    speed_hz = SFC_BUS_DEFAULT_CLK;
#[no_mangle]
pub unsafe extern "C" fn if(SFC_MIN_FREQUENCY: spi->max_speed_hz <) -> else {
    else if (spi.max_speed_hz < SFC_MIN_FREQUENCY)
    speed_hz = SFC_MIN_FREQUENCY;
    else
    speed_hz = spi.max_speed_hz;
// The SPI clock is generated by dividing the bus clock by four by default.
    ret = regmap_write(sfc.regmap_base, SFC_CFG, (DEFAULT_BUS_CYCLE - 1));
    if (ret) {
    dev_err(sfc.dev, "failed to set bus cycle\n");
    return ret;
    }
    return clk_set_rate(sfc.core_clk, speed_hz * DEFAULT_BUS_CYCLE);
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_setup(spi: *mut spi_device) -> c_int {
    static int aml_sfc_setup(struct spi_device *spi)
    {
    struct aml_sfc *sfc;
    int ret;
    sfc = spi_controller_get_devdata(spi.controller);
    ret = aml_spi_settings(sfc, spi);
    if (ret)
    return ret;
    ret = aml_set_spi_clk(sfc, spi);
    if (ret)
    return ret;
    sfc.bus_rate = clk_get_rate(sfc.core_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_ecc_init_ctx(nand: *mut nand_device) -> c_int {
    static int aml_sfc_ecc_init_ctx(struct nand_device *nand)
    {
    struct mtd_info *mtd = nanddev_to_mtd(nand);
    struct aml_sfc *sfc = nand_to_aml_sfc(nand);
    struct aml_sfc_ecc_cfg *ecc_cfg;
    const struct aml_sfc_caps *caps = sfc.caps;
    struct aml_sfc_ecc_cfg *ecc_caps = caps.ecc_caps;
    int i, ecc_strength, ecc_step_size;
    ecc_step_size = nand.ecc.user_conf.step_size;
    ecc_strength = nand.ecc.user_conf.strength;
    for (i = 0; i < caps.num_ecc_caps; i++) {
    if (ecc_caps[i].stepsize == ecc_step_size) {
    nand.ecc.ctx.conf.step_size = ecc_step_size;
    nand.ecc.ctx.conf.flags |= BIT(ecc_caps[i].bch);
    }
    if (ecc_caps[i].strength == ecc_strength)
    nand.ecc.ctx.conf.strength = ecc_strength;
    }
    if (!nand.ecc.ctx.conf.step_size) {
    nand.ecc.ctx.conf.step_size = ECC_BCH8_DEFAULT_STEP;
    nand.ecc.ctx.conf.flags |= BIT(ECC_DEFAULT_BCH_MODE);
    }
    if (!nand.ecc.ctx.conf.strength)
    nand.ecc.ctx.conf.strength = ECC_BCH8_STRENGTH;
    nand.ecc.ctx.nsteps = nand.memorg.pagesize / nand.ecc.ctx.conf.step_size;
    nand.ecc.ctx.total = nand.ecc.ctx.nsteps * ECC_BCH8_PARITY_BYTES;
// Verify the page size and OOB size against the SFC requirements.
    if ((nand.memorg.pagesize % nand.ecc.ctx.conf.step_size) ||
    (nand.memorg.oobsize < (nand.ecc.ctx.total +
    nand.ecc.ctx.nsteps * ECC_BCH8_USER_BYTES)))
    return -EOPNOTSUPP;
    nand.ecc.ctx.conf.engine_type = NAND_ECC_ENGINE_TYPE_ON_HOST;
    ecc_cfg = kzalloc_obj(*ecc_cfg);
    if (!ecc_cfg)
    return -ENOMEM;
    ecc_cfg.stepsize = nand.ecc.ctx.conf.step_size;
    ecc_cfg.nsteps = nand.ecc.ctx.nsteps;
    ecc_cfg.strength = nand.ecc.ctx.conf.strength;
    ecc_cfg.oobsize = nand.memorg.oobsize;
    ecc_cfg.bch = nand.ecc.ctx.conf.flags & BIT(ECC_DEFAULT_BCH_MODE) ? 1 : 2;
    nand.ecc.ctx.priv = ecc_cfg;
    sfc.priv = (void *)ecc_cfg;
    mtd_set_ooblayout(mtd, &aml_sfc_ooblayout_ops);
    sfc.flags |= SFC_HWECC;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_ecc_cleanup_ctx(nand: *mut nand_device) {
    static void aml_sfc_ecc_cleanup_ctx(struct nand_device *nand)
    {
    struct aml_sfc *sfc = nand_to_aml_sfc(nand);
    sfc.flags &= ~(SFC_HWECC);
    kfree(nand.ecc.ctx.priv);
    sfc.priv = core::ptr::null_mut();
    }
    static int aml_sfc_ecc_prepare_io_req(struct nand_device *nand,
    struct nand_page_io_req *req)
    {
    struct aml_sfc *sfc = nand_to_aml_sfc(nand);
    struct spinand_device *spinand = nand_to_spinand(nand);
    sfc.flags &= ~SFC_XFER_MDOE_MASK;
    if (req.datalen && !req.ooblen)
    sfc.flags |= SFC_DATA_ONLY;
#[no_mangle]
pub unsafe extern "C" fn if(req->ooblen: !req->datalen &&) -> else {
    else if (!req.datalen && req.ooblen)
    sfc.flags |= SFC_OOB_ONLY;
#[no_mangle]
pub unsafe extern "C" fn if(req->ooblen: req->datalen &&) -> else {
    else if (req.datalen && req.ooblen)
    sfc.flags |= SFC_DATA_OOB;
    if (req.mode == MTD_OPS_RAW)
    sfc.flags |= SFC_RAW_RW;
#[no_mangle]
pub unsafe extern "C" fn if(MTD_OPS_AUTO_OOB: req->mode ==) -> else {
    else if (req.mode == MTD_OPS_AUTO_OOB)
    sfc.flags |= SFC_AUTO_OOB;
    memset(spinand.oobbuf, 0xff, nanddev_per_page_oobsize(nand));
    return 0;
    }
    static int aml_sfc_ecc_finish_io_req(struct nand_device *nand,
    struct nand_page_io_req *req)
    {
    struct aml_sfc *sfc = nand_to_aml_sfc(nand);
    struct mtd_info *mtd = nanddev_to_mtd(nand);
    if (req.mode == MTD_OPS_RAW || req.type == NAND_PAGE_WRITE)
    return 0;
    if (sfc.ecc_stats.failed)
    mtd.ecc_stats.failed++;
    mtd.ecc_stats.corrected += sfc.ecc_stats.corrected;
    return sfc.ecc_stats.failed ? -EBADMSG : sfc.ecc_stats.bitflips;
    }
    static const struct spi_controller_mem_caps aml_sfc_mem_caps = {
    .ecc = true,
    };
    static const struct nand_ecc_engine_ops aml_sfc_ecc_engine_ops = {
    .init_ctx = aml_sfc_ecc_init_ctx,
    .cleanup_ctx = aml_sfc_ecc_cleanup_ctx,
    .prepare_io_req = aml_sfc_ecc_prepare_io_req,
    .finish_io_req = aml_sfc_ecc_finish_io_req,
    };
#[no_mangle]
unsafe extern "C" fn aml_sfc_unregister_ecc_engine(data: *mut c_void) {
    static void aml_sfc_unregister_ecc_engine(void *data)
    {
    struct nand_ecc_engine *eng = data;
    nand_ecc_unregister_on_host_hw_engine(eng);
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_clk_init(sfc: *mut aml_sfc) -> c_int {
    static int aml_sfc_clk_init(struct aml_sfc *sfc)
    {
    sfc.gate_clk = devm_clk_get_enabled(sfc.dev, "gate");
    if (IS_ERR(sfc.gate_clk)) {
    dev_err(sfc.dev, "unable to enable gate clk\n");
    return PTR_ERR(sfc.gate_clk);
    }
    sfc.core_clk = devm_clk_get_enabled(sfc.dev, "core");
    if (IS_ERR(sfc.core_clk)) {
    dev_err(sfc.dev, "unable to enable core clk\n");
    return PTR_ERR(sfc.core_clk);
    }
    return clk_set_rate(sfc.core_clk, SFC_BUS_DEFAULT_CLK);
    }
#[no_mangle]
unsafe extern "C" fn aml_sfc_probe(pdev: *mut platform_device) -> c_int {
    static int aml_sfc_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    struct spi_controller *ctrl;
    struct aml_sfc *sfc;
    void __iomem *reg_base;
    int ret;
    let mut val: u32 = 0;
    const struct regmap_config core_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = SFC_SPI_CFG,
    };
    ctrl = devm_spi_alloc_host(dev, sizeof(*sfc));
    if (!ctrl)
    return -ENOMEM;
    platform_set_drvdata(pdev, ctrl);
    sfc = spi_controller_get_devdata(ctrl);
    sfc.dev = dev;
    sfc.ctrl = ctrl;
    sfc.caps = of_device_get_match_data(dev);
    if (!sfc.caps)
    return dev_err_probe(dev, -ENODEV, "failed to get device data\n");
    reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(reg_base))
    return PTR_ERR(reg_base);
    sfc.regmap_base = devm_regmap_init_mmio(dev, reg_base, &core_config);
    if (IS_ERR(sfc.regmap_base))
    return dev_err_probe(dev, PTR_ERR(sfc.regmap_base),
    "failed to init sfc base regmap\n");
    sfc.data_buf = devm_kzalloc(dev, SFC_BUF_SIZE, GFP_KERNEL);
    if (!sfc.data_buf)
    return -ENOMEM;
    sfc.info_buf = (__le64 *)(sfc.data_buf + SFC_DATABUF_SIZE);
    ret = aml_sfc_clk_init(sfc);
    if (ret)
    return dev_err_probe(dev, ret, "failed to initialize SFC clock\n");
// Enable Amlogic flash controller spi mode
    ret = regmap_write(sfc.regmap_base, SFC_SPI_CFG, SPI_MODE_EN);
    if (ret)
    return dev_err_probe(dev, ret, "failed to enable SPI mode\n");
    ret = dma_set_mask(sfc.dev, DMA_BIT_MASK(32));
    if (ret)
    return dev_err_probe(sfc.dev, ret, "failed to set dma mask\n");
    sfc.ecc_eng.dev = &pdev.dev;
    sfc.ecc_eng.integration = NAND_ECC_ENGINE_INTEGRATION_PIPELINED;
    sfc.ecc_eng.ops = &aml_sfc_ecc_engine_ops;
    sfc.ecc_eng.priv = sfc;
    ret = nand_ecc_register_on_host_hw_engine(&sfc.ecc_eng);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "failed to register Aml host ecc engine.\n");
    ret = devm_add_action_or_reset(dev, aml_sfc_unregister_ecc_engine,
    &sfc.ecc_eng);
    if (ret)
    return dev_err_probe(dev, ret, "failed to add ECC unregister action\n");
    ret = of_property_read_u32(np, "amlogic,rx-adj", &val);
    if (!ret)
    sfc.rx_adj = val;
    ctrl.dev.of_node = np;
    ctrl.mem_ops = &aml_sfc_mem_ops;
    ctrl.mem_caps = &aml_sfc_mem_caps;
    ctrl.setup = aml_sfc_setup;
    ctrl.mode_bits = SPI_TX_QUAD | SPI_TX_DUAL | SPI_RX_QUAD |
    SPI_RX_DUAL | SPI_TX_OCTAL | SPI_RX_OCTAL;
    ctrl.max_speed_hz = SFC_MAX_FREQUENCY;
    ctrl.min_speed_hz = SFC_MIN_FREQUENCY;
    ctrl.num_chipselect = SFC_MAX_CS_NUM;
    return devm_spi_register_controller(dev, ctrl);
    }
    static const struct of_device_id aml_sfc_of_match[] = {
    {
    .compatible = "amlogic,a4-spifc",
    .data = &aml_a113l2_sfc_caps
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, aml_sfc_of_match);
    static struct platform_driver aml_sfc_driver = {
    .driver = {
    .name = "aml_sfc",
    .of_match_table = aml_sfc_of_match,
    },
    .probe = aml_sfc_probe,
    };
    module_platform_driver(aml_sfc_driver);
    MODULE_DESCRIPTION("Amlogic SPI Flash Controller driver");
    MODULE_AUTHOR("Feng Chen <feng.chen@amlogic.com>");
    MODULE_LICENSE("Dual MIT/GPL");
