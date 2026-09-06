//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-npcm-fiu.c
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
// Copyright (c) 2019 Nuvoton Technology corporation.

// NPCM7xx GCR module
pub const NPCM7XX_INTCR3_OFFSET: c_uint = 0x9C;

// Flash Interface Unit (FIU) Registers
pub const NPCM_FIU_DRD_CFG: c_uint = 0x00;
pub const NPCM_FIU_DWR_CFG: c_uint = 0x04;
pub const NPCM_FIU_UMA_CFG: c_uint = 0x08;
pub const NPCM_FIU_UMA_CTS: c_uint = 0x0C;
pub const NPCM_FIU_UMA_CMD: c_uint = 0x10;
pub const NPCM_FIU_UMA_ADDR: c_uint = 0x14;
pub const NPCM_FIU_PRT_CFG: c_uint = 0x18;
pub const NPCM_FIU_UMA_DW0: c_uint = 0x20;
pub const NPCM_FIU_UMA_DW1: c_uint = 0x24;
pub const NPCM_FIU_UMA_DW2: c_uint = 0x28;
pub const NPCM_FIU_UMA_DW3: c_uint = 0x2C;
pub const NPCM_FIU_UMA_DR0: c_uint = 0x30;
pub const NPCM_FIU_UMA_DR1: c_uint = 0x34;
pub const NPCM_FIU_UMA_DR2: c_uint = 0x38;
pub const NPCM_FIU_UMA_DR3: c_uint = 0x3C;
pub const NPCM_FIU_CFG: c_uint = 0x78;
pub const NPCM_FIU_MAX_REG_LIMIT: c_uint = 0x80;
// FIU Direct Read Configuration Register

pub const NPCM_FIU_DRD_ADDSIZ_SHIFT: c_int = 16;
pub const NPCM_FIU_DRD_DBW_SHIFT: c_int = 12;
pub const NPCM_FIU_DRD_ACCTYPE_SHIFT: c_int = 8;
// FIU Direct Write Configuration Register

pub const NPCM_FIU_DWR_ADDSIZ_SHIFT: c_int = 16;
pub const NPCM_FIU_DWR_ABPCK_SHIFT: c_int = 10;
pub const NPCM_FIU_DWR_DBPCK_SHIFT: c_int = 8;
// FIU UMA Configuration Register

pub const NPCM_FIU_UMA_CFG_ADBPCK_SHIFT: c_int = 2;
pub const NPCM_FIU_UMA_CFG_WDBPCK_SHIFT: c_int = 4;
pub const NPCM_FIU_UMA_CFG_DBPCK_SHIFT: c_int = 6;
pub const NPCM_FIU_UMA_CFG_RDBPCK_SHIFT: c_int = 8;
pub const NPCM_FIU_UMA_CFG_ADDSIZ_SHIFT: c_int = 11;
pub const NPCM_FIU_UMA_CFG_WDATSIZ_SHIFT: c_int = 16;
pub const NPCM_FIU_UMA_CFG_DBSIZ_SHIFT: c_int = 21;
pub const NPCM_FIU_UMA_CFG_RDATSIZ_SHIFT: c_int = 24;
// FIU UMA Control and Status Register

pub const NPCM_FIU_UMA_CTS_DEV_NUM_SHIFT: c_int = 8;
// FIU UMA Command Register

// FIU UMA Address Register

// FIU UMA Write Data Bytes 0-3 Register

// FIU UMA Write Data Bytes 4-7 Register

// FIU UMA Write Data Bytes 8-11 Register

// FIU UMA Write Data Bytes 12-15 Register

// FIU UMA Read Data Bytes 0-3 Register

// FIU UMA Read Data Bytes 4-7 Register

// FIU UMA Read Data Bytes 8-11 Register

// FIU UMA Read Data Bytes 12-15 Register

// FIU Configuration Register

// FIU Read Mode
    enum {
    DRD_SINGLE_WIRE_MODE	= 0,
    DRD_DUAL_IO_MODE	= 1,
    DRD_QUAD_IO_MODE	= 2,
    DRD_SPI_X_MODE		= 3,
    };
    enum {
    DWR_ABPCK_BIT_PER_CLK	= 0,
    DWR_ABPCK_2_BIT_PER_CLK	= 1,
    DWR_ABPCK_4_BIT_PER_CLK	= 2,
    };
    enum {
    DWR_DBPCK_BIT_PER_CLK	= 0,
    DWR_DBPCK_2_BIT_PER_CLK	= 1,
    DWR_DBPCK_4_BIT_PER_CLK	= 2,
    };
pub const NPCM_FIU_DRD_16_BYTE_BURST: c_uint = 0x3000000;
pub const NPCM_FIU_DWR_16_BYTE_BURST: c_uint = 0x3000000;
pub const MAP_SIZE_128MB: c_uint = 0x8000000;
pub const MAP_SIZE_16MB: c_uint = 0x1000000;
pub const MAP_SIZE_8MB: c_uint = 0x800000;
pub const FIU_DRD_MAX_DUMMY_NUMBER: c_int = 3;
pub const NPCM_MAX_CHIP_NUM: c_int = 4;
pub const CHUNK_SIZE: c_int = 16;
pub const UMA_MICRO_SEC_TIMEOUT: c_int = 150;
    enum {
    FIU0 = 0,
    FIU3,
    FIUX,
    FIU1,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npcm_fiu_info {
    pub name: *mut c_char,
    pub fiu_id: u32,
    pub max_map_size: u32,
    pub max_cs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fiu_data {
    pub npcm_fiu_data_info: *const npcm_fiu_info,
    pub fiu_max: c_int,
}

    static const struct npcm_fiu_info npcm7xx_fiu_info[] = {
    {.name = "FIU0", .fiu_id = FIU0,
    .max_map_size = MAP_SIZE_128MB, .max_cs = 2},
    {.name = "FIU3", .fiu_id = FIU3,
    .max_map_size = MAP_SIZE_128MB, .max_cs = 4},
    {.name = "FIUX", .fiu_id = FIUX,
    .max_map_size = MAP_SIZE_16MB, .max_cs = 2} };
    static const struct fiu_data npcm7xx_fiu_data = {
    .npcm_fiu_data_info = npcm7xx_fiu_info,
    .fiu_max = 3,
    };
    static const struct npcm_fiu_info npxm8xx_fiu_info[] = {
    {.name = "FIU0", .fiu_id = FIU0,
    .max_map_size = MAP_SIZE_128MB, .max_cs = 2},
    {.name = "FIU3", .fiu_id = FIU3,
    .max_map_size = MAP_SIZE_128MB, .max_cs = 4},
    {.name = "FIUX", .fiu_id = FIUX,
    .max_map_size = MAP_SIZE_16MB, .max_cs = 2},
    {.name = "FIU1", .fiu_id = FIU1,
    .max_map_size = MAP_SIZE_16MB, .max_cs = 4} };
    static const struct fiu_data npxm8xx_fiu_data = {
    .npcm_fiu_data_info = npxm8xx_fiu_info,
    .fiu_max = 4,
    };
    struct npcm_fiu_spi;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npcm_fiu_chip {
    pub flash_region_mapped_ptr: *mut void __iomem,
    pub fiu: *mut npcm_fiu_spi,
    pub clkrate: c_ulong,
    pub chipselect: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npcm_fiu_spi {
    pub chip: [npcm_fiu_chip; NPCM_MAX_CHIP_NUM],
    pub info: *const npcm_fiu_info,
    pub drd_op: spi_mem_op,
    pub res_mem: *mut resource,
    pub regmap: *mut regmap,
    pub clkrate: c_ulong,
    pub dev: *mut device,
    pub clk: *mut clk,
    pub spix_mode: bool,
}

    static const struct regmap_config npcm_mtd_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = NPCM_FIU_MAX_REG_LIMIT,
    };
    static void npcm_fiu_set_drd(struct npcm_fiu_spi *fiu,
    const struct spi_mem_op *op)
    {
    regmap_update_bits(fiu.regmap, NPCM_FIU_DRD_CFG,
    NPCM_FIU_DRD_CFG_ACCTYPE,
    ilog2(op.addr.buswidth) <<
    NPCM_FIU_DRD_ACCTYPE_SHIFT);
    fiu.drd_op.addr.buswidth = op.addr.buswidth;
    regmap_update_bits(fiu.regmap, NPCM_FIU_DRD_CFG,
    NPCM_FIU_DRD_CFG_DBW,
    op.dummy.nbytes << NPCM_FIU_DRD_DBW_SHIFT);
    fiu.drd_op.dummy.nbytes = op.dummy.nbytes;
    regmap_update_bits(fiu.regmap, NPCM_FIU_DRD_CFG,
    NPCM_FIU_DRD_CFG_RDCMD, op.cmd.opcode);
    fiu.drd_op.cmd.opcode = op.cmd.opcode;
    regmap_update_bits(fiu.regmap, NPCM_FIU_DRD_CFG,
    NPCM_FIU_DRD_CFG_ADDSIZ,
    (op.addr.nbytes - 3) << NPCM_FIU_DRD_ADDSIZ_SHIFT);
    fiu.drd_op.addr.nbytes = op.addr.nbytes;
    }
    static ssize_t npcm_fiu_direct_read(struct spi_mem_dirmap_desc *desc,
    u64 offs, size_t len, void *buf)
    {
    struct npcm_fiu_spi *fiu =
    spi_controller_get_devdata(desc.mem.spi.controller);
    struct npcm_fiu_chip *chip = &fiu.chip[spi_get_chipselect(desc.mem.spi, 0)];
    void __iomem *src = (void __iomem *)(chip.flash_region_mapped_ptr +
    offs);
    u8 *buf_rx = buf;
    u32 i;
    if (fiu.spix_mode) {
    for (i = 0 ; i < len ; i++)
// (buf_rx + i) = ioread8(src + i);
    } else {
    if (desc.info.op_tmpl.addr.buswidth != fiu.drd_op.addr.buswidth ||
    desc.info.op_tmpl.dummy.nbytes != fiu.drd_op.dummy.nbytes ||
    desc.info.op_tmpl.cmd.opcode != fiu.drd_op.cmd.opcode ||
    desc.info.op_tmpl.addr.nbytes != fiu.drd_op.addr.nbytes)
    npcm_fiu_set_drd(fiu, desc.info.op_tmpl);
    memcpy_fromio(buf_rx, src, len);
    }
    return len;
    }
    static ssize_t npcm_fiu_direct_write(struct spi_mem_dirmap_desc *desc,
    u64 offs, size_t len, const void *buf)
    {
    struct npcm_fiu_spi *fiu =
    spi_controller_get_devdata(desc.mem.spi.controller);
    struct npcm_fiu_chip *chip = &fiu.chip[spi_get_chipselect(desc.mem.spi, 0)];
    void __iomem *dst = (void __iomem *)(chip.flash_region_mapped_ptr +
    offs);
    const u8 *buf_tx = buf;
    u32 i;
    if (fiu.spix_mode)
    for (i = 0 ; i < len ; i++)
    iowrite8(*(buf_tx + i), dst + i);
    else
    memcpy_toio(dst, buf_tx, len);
    return len;
    }
    static int npcm_fiu_uma_read(struct spi_mem *mem,
    const struct spi_mem_op *op, u32 addr,
    bool is_address_size, u8 *data, u32 data_size)
    {
    struct npcm_fiu_spi *fiu =
    spi_controller_get_devdata(mem.spi.controller);
    let mut uma_cfg: u32 = BIT(10);
    u32 data_reg[4];
    int ret;
    u32 val;
    u32 i;
    regmap_update_bits(fiu.regmap, NPCM_FIU_UMA_CTS,
    NPCM_FIU_UMA_CTS_DEV_NUM,
    (spi_get_chipselect(mem.spi, 0) <<
    NPCM_FIU_UMA_CTS_DEV_NUM_SHIFT));
    regmap_update_bits(fiu.regmap, NPCM_FIU_UMA_CMD,
    NPCM_FIU_UMA_CMD_CMD, op.cmd.opcode);
    if (is_address_size) {
    uma_cfg |= ilog2(op.cmd.buswidth);
    uma_cfg |= ilog2(op.addr.buswidth)
    << NPCM_FIU_UMA_CFG_ADBPCK_SHIFT;
    if (op.dummy.nbytes)
    uma_cfg |= ilog2(op.dummy.buswidth)
    << NPCM_FIU_UMA_CFG_DBPCK_SHIFT;
    uma_cfg |= ilog2(op.data.buswidth)
    << NPCM_FIU_UMA_CFG_RDBPCK_SHIFT;
    uma_cfg |= op.dummy.nbytes << NPCM_FIU_UMA_CFG_DBSIZ_SHIFT;
    uma_cfg |= op.addr.nbytes << NPCM_FIU_UMA_CFG_ADDSIZ_SHIFT;
    regmap_write(fiu.regmap, NPCM_FIU_UMA_ADDR, addr);
    } else {
    regmap_write(fiu.regmap, NPCM_FIU_UMA_ADDR, 0x0);
    }
    uma_cfg |= data_size << NPCM_FIU_UMA_CFG_RDATSIZ_SHIFT;
    regmap_write(fiu.regmap, NPCM_FIU_UMA_CFG, uma_cfg);
    regmap_write_bits(fiu.regmap, NPCM_FIU_UMA_CTS,
    NPCM_FIU_UMA_CTS_EXEC_DONE,
    NPCM_FIU_UMA_CTS_EXEC_DONE);
    ret = regmap_read_poll_timeout(fiu.regmap, NPCM_FIU_UMA_CTS, val,
    (!(val & NPCM_FIU_UMA_CTS_EXEC_DONE)), 0,
    UMA_MICRO_SEC_TIMEOUT);
    if (ret)
    return ret;
    if (data_size) {
    for (i = 0; i < DIV_ROUND_UP(data_size, 4); i++)
    regmap_read(fiu.regmap, NPCM_FIU_UMA_DR0 + (i * 4),
    &data_reg[i]);
    memcpy(data, data_reg, data_size);
    }
    return 0;
    }
    static int npcm_fiu_uma_write(struct spi_mem *mem,
    const struct spi_mem_op *op, u8 cmd,
    bool is_address_size, u8 *data, u32 data_size)
    {
    struct npcm_fiu_spi *fiu =
    spi_controller_get_devdata(mem.spi.controller);
    let mut uma_cfg: u32 = cmd ? BIT(10) : 0;
    u32 data_reg[4] = {0};
    u32 val;
    u32 i;
    regmap_update_bits(fiu.regmap, NPCM_FIU_UMA_CTS,
    NPCM_FIU_UMA_CTS_DEV_NUM,
    (spi_get_chipselect(mem.spi, 0) <<
    NPCM_FIU_UMA_CTS_DEV_NUM_SHIFT));
    if (cmd)
    regmap_update_bits(fiu.regmap, NPCM_FIU_UMA_CMD,
    NPCM_FIU_UMA_CMD_CMD, cmd);
    else
    uma_cfg |= ilog2(op.data.buswidth) << NPCM_FIU_UMA_CFG_WDBPCK_SHIFT;
    if (data_size) {
    memcpy(data_reg, data, data_size);
    for (i = 0; i < DIV_ROUND_UP(data_size, 4); i++)
    regmap_write(fiu.regmap, NPCM_FIU_UMA_DW0 + (i * 4),
    data_reg[i]);
    }
    if (is_address_size) {
    uma_cfg |= ilog2(op.cmd.buswidth);
    uma_cfg |= ilog2(op.addr.buswidth) <<
    NPCM_FIU_UMA_CFG_ADBPCK_SHIFT;
    uma_cfg |= ilog2(op.data.buswidth) <<
    NPCM_FIU_UMA_CFG_WDBPCK_SHIFT;
    uma_cfg |= op.addr.nbytes << NPCM_FIU_UMA_CFG_ADDSIZ_SHIFT;
    regmap_write(fiu.regmap, NPCM_FIU_UMA_ADDR, op.addr.val);
    } else {
    regmap_write(fiu.regmap, NPCM_FIU_UMA_ADDR, 0x0);
    }
    uma_cfg |= (data_size << NPCM_FIU_UMA_CFG_WDATSIZ_SHIFT);
    regmap_write(fiu.regmap, NPCM_FIU_UMA_CFG, uma_cfg);
    regmap_write_bits(fiu.regmap, NPCM_FIU_UMA_CTS,
    NPCM_FIU_UMA_CTS_EXEC_DONE,
    NPCM_FIU_UMA_CTS_EXEC_DONE);
    return regmap_read_poll_timeout(fiu.regmap, NPCM_FIU_UMA_CTS, val,
    (!(val & NPCM_FIU_UMA_CTS_EXEC_DONE)), 0,
    UMA_MICRO_SEC_TIMEOUT);
    }
    static int npcm_fiu_manualwrite(struct spi_mem *mem,
    const struct spi_mem_op *op)
    {
    struct npcm_fiu_spi *fiu =
    spi_controller_get_devdata(mem.spi.controller);
    u8 *data = (u8 *)op.data.buf.out;
    u32 num_data_chunks;
    u32 remain_data;
    let mut idx: u32 = 0;
    int ret;
    num_data_chunks  = op.data.nbytes / CHUNK_SIZE;
    remain_data  = op.data.nbytes % CHUNK_SIZE;
    regmap_update_bits(fiu.regmap, NPCM_FIU_UMA_CTS,
    NPCM_FIU_UMA_CTS_DEV_NUM,
    (spi_get_chipselect(mem.spi, 0) <<
    NPCM_FIU_UMA_CTS_DEV_NUM_SHIFT));
    regmap_update_bits(fiu.regmap, NPCM_FIU_UMA_CTS,
    NPCM_FIU_UMA_CTS_SW_CS, 0);
    ret = npcm_fiu_uma_write(mem, op, op.cmd.opcode, true, core::ptr::null_mut(), 0);
    if (ret)
    return ret;
// Starting the data writing loop in multiples of 8
    for (idx = 0; idx < num_data_chunks; ++idx) {
    ret = npcm_fiu_uma_write(mem, op, 0, false, &data[0], CHUNK_SIZE);
    if (ret)
    return ret;
    data += CHUNK_SIZE;
    }
// Handling chunk remains
    if (remain_data > 0) {
    ret = npcm_fiu_uma_write(mem, op, 0, false, &data[0], remain_data);
    if (ret)
    return ret;
    }
    regmap_update_bits(fiu.regmap, NPCM_FIU_UMA_CTS,
    NPCM_FIU_UMA_CTS_SW_CS, NPCM_FIU_UMA_CTS_SW_CS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn npcm_fiu_read(mem: *mut spi_mem, op: *const spi_mem_op) -> c_int {
    static int npcm_fiu_read(struct spi_mem *mem, const struct spi_mem_op *op)
    {
    u8 *data = op.data.buf.in;
    int i, readlen, currlen;
    u8 *buf_ptr;
    u32 addr;
    int ret;
    i = 0;
    currlen = op.data.nbytes;
    do {
    addr = ((u32)op.addr.val + i);
    readlen = min_t(int, currlen, 16);
    buf_ptr = data + i;
    ret = npcm_fiu_uma_read(mem, op, addr, true, buf_ptr,
    readlen);
    if (ret)
    return ret;
    i += readlen;
    currlen -= 16;
    } while (currlen > 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn npcm_fiux_set_direct_wr(fiu: *mut npcm_fiu_spi) {
    static void npcm_fiux_set_direct_wr(struct npcm_fiu_spi *fiu)
    {
    regmap_write(fiu.regmap, NPCM_FIU_DWR_CFG,
    NPCM_FIU_DWR_16_BYTE_BURST);
    regmap_update_bits(fiu.regmap, NPCM_FIU_DWR_CFG,
    NPCM_FIU_DWR_CFG_ABPCK,
    DWR_ABPCK_4_BIT_PER_CLK << NPCM_FIU_DWR_ABPCK_SHIFT);
    regmap_update_bits(fiu.regmap, NPCM_FIU_DWR_CFG,
    NPCM_FIU_DWR_CFG_DBPCK,
    DWR_DBPCK_4_BIT_PER_CLK << NPCM_FIU_DWR_DBPCK_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn npcm_fiux_set_direct_rd(fiu: *mut npcm_fiu_spi) {
    static void npcm_fiux_set_direct_rd(struct npcm_fiu_spi *fiu)
    {
    let mut rx_dummy: u32 = 0;
    regmap_write(fiu.regmap, NPCM_FIU_DRD_CFG,
    NPCM_FIU_DRD_16_BYTE_BURST);
    regmap_update_bits(fiu.regmap, NPCM_FIU_DRD_CFG,
    NPCM_FIU_DRD_CFG_ACCTYPE,
    DRD_SPI_X_MODE << NPCM_FIU_DRD_ACCTYPE_SHIFT);
    regmap_update_bits(fiu.regmap, NPCM_FIU_DRD_CFG,
    NPCM_FIU_DRD_CFG_DBW,
    rx_dummy << NPCM_FIU_DRD_DBW_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn npcm_fiu_exec_op(mem: *mut spi_mem, op: *const spi_mem_op) -> c_int {
    static int npcm_fiu_exec_op(struct spi_mem *mem, const struct spi_mem_op *op)
    {
    struct npcm_fiu_spi *fiu =
    spi_controller_get_devdata(mem.spi.controller);
    struct npcm_fiu_chip *chip = &fiu.chip[spi_get_chipselect(mem.spi, 0)];
    let mut ret: c_int = 0;
    u8 *buf;
    if (fiu.spix_mode || op.addr.nbytes > 4)
    return -EOPNOTSUPP;
    if (fiu.clkrate != chip.clkrate) {
    ret = clk_set_rate(fiu.clk, chip.clkrate);
    if (ret < 0)
    dev_warn(fiu.dev, "Failed setting %lu frequency, stay at %lu frequency\n",
    chip.clkrate, fiu.clkrate);
    else
    fiu.clkrate = chip.clkrate;
    }
    if (op.data.dir == SPI_MEM_DATA_IN) {
    if (!op.addr.nbytes) {
    buf = op.data.buf.in;
    ret = npcm_fiu_uma_read(mem, op, op.addr.val, false,
    buf, op.data.nbytes);
    } else {
    ret = npcm_fiu_read(mem, op);
    }
    } else  {
    if (!op.addr.nbytes && !op.data.nbytes)
    ret = npcm_fiu_uma_write(mem, op, op.cmd.opcode, false,
    core::ptr::null_mut(), 0);
    if (op.addr.nbytes && !op.data.nbytes) {
    int i;
    u8 buf_addr[4];
    let mut addr: u32 = op.addr.val;
    for (i = op.addr.nbytes - 1; i >= 0; i--) {
    buf_addr[i] = addr & 0xff;
    addr >>= 8;
    }
    ret = npcm_fiu_uma_write(mem, op, op.cmd.opcode, false,
    buf_addr, op.addr.nbytes);
    }
    if (!op.addr.nbytes && op.data.nbytes)
    ret = npcm_fiu_uma_write(mem, op, op.cmd.opcode, false,
    (u8 *)op.data.buf.out,
    op.data.nbytes);
    if (op.addr.nbytes && op.data.nbytes)
    ret = npcm_fiu_manualwrite(mem, op);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn npcm_fiu_dirmap_create(desc: *mut spi_mem_dirmap_desc) -> c_int {
    static int npcm_fiu_dirmap_create(struct spi_mem_dirmap_desc *desc)
    {
    struct npcm_fiu_spi *fiu =
    spi_controller_get_devdata(desc.mem.spi.controller);
    struct npcm_fiu_chip *chip = &fiu.chip[spi_get_chipselect(desc.mem.spi, 0)];
    struct regmap *gcr_regmap;
    if (!fiu.res_mem) {
    dev_warn(fiu.dev, "Reserved memory not defined, direct read disabled\n");
    desc.nodirmap = true;
    return 0;
    }
    if (!fiu.spix_mode &&
    desc.info.op_tmpl.data.dir == SPI_MEM_DATA_OUT) {
    desc.nodirmap = true;
    return 0;
    }
    if (!chip.flash_region_mapped_ptr) {
    chip.flash_region_mapped_ptr =
    devm_ioremap(fiu.dev, (fiu.res_mem.start +
    (fiu.info.max_map_size *
    spi_get_chipselect(desc.mem.spi, 0))),
    (u32)desc.info.length);
    if (!chip.flash_region_mapped_ptr) {
    dev_warn(fiu.dev, "Error mapping memory region, direct read disabled\n");
    desc.nodirmap = true;
    return 0;
    }
    }
    if (of_device_is_compatible(fiu.dev.of_node, "nuvoton,npcm750-fiu")) {
    gcr_regmap =
    syscon_regmap_lookup_by_compatible("nuvoton,npcm750-gcr");
    if (IS_ERR(gcr_regmap)) {
    dev_warn(fiu.dev, "Didn't find nuvoton,npcm750-gcr, direct read disabled\n");
    desc.nodirmap = true;
    return 0;
    }
    regmap_update_bits(gcr_regmap, NPCM7XX_INTCR3_OFFSET,
    NPCM7XX_INTCR3_FIU_FIX,
    NPCM7XX_INTCR3_FIU_FIX);
    } else {
    regmap_update_bits(fiu.regmap, NPCM_FIU_CFG,
    NPCM_FIU_CFG_FIU_FIX,
    NPCM_FIU_CFG_FIU_FIX);
    }
    if (desc.info.op_tmpl.data.dir == SPI_MEM_DATA_IN) {
    if (!fiu.spix_mode)
    npcm_fiu_set_drd(fiu, desc.info.op_tmpl);
    else
    npcm_fiux_set_direct_rd(fiu);
    } else {
    npcm_fiux_set_direct_wr(fiu);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn npcm_fiu_setup(spi: *mut spi_device) -> c_int {
    static int npcm_fiu_setup(struct spi_device *spi)
    {
    struct spi_controller *ctrl = spi.controller;
    struct npcm_fiu_spi *fiu = spi_controller_get_devdata(ctrl);
    struct npcm_fiu_chip *chip;
    chip = &fiu.chip[spi_get_chipselect(spi, 0)];
    chip.fiu = fiu;
    chip.chipselect = spi_get_chipselect(spi, 0);
    chip.clkrate = spi.max_speed_hz;
    fiu.clkrate = clk_get_rate(fiu.clk);
    return 0;
    }
    static const struct spi_controller_mem_ops npcm_fiu_mem_ops = {
    .exec_op = npcm_fiu_exec_op,
    .dirmap_create = npcm_fiu_dirmap_create,
    .dirmap_read = npcm_fiu_direct_read,
    .dirmap_write = npcm_fiu_direct_write,
    };
    static const struct of_device_id npcm_fiu_dt_ids[] = {
    { .compatible = "nuvoton,npcm750-fiu", .data = &npcm7xx_fiu_data  },
    { .compatible = "nuvoton,npcm845-fiu", .data = &npxm8xx_fiu_data  },
    { /* sentinel */ }
    };
#[no_mangle]
unsafe extern "C" fn npcm_fiu_probe(pdev: *mut platform_device) -> c_int {
    static int npcm_fiu_probe(struct platform_device *pdev)
    {
    const struct fiu_data *fiu_data_match;
    struct device *dev = &pdev.dev;
    struct spi_controller *ctrl;
    struct npcm_fiu_spi *fiu;
    void __iomem *regbase;
    int id;
    ctrl = devm_spi_alloc_host(dev, sizeof(*fiu));
    if (!ctrl)
    return -ENOMEM;
    fiu = spi_controller_get_devdata(ctrl);
    fiu_data_match = of_device_get_match_data(dev);
    if (!fiu_data_match) {
    dev_err(dev, "No compatible OF match\n");
    return -ENODEV;
    }
    id = of_alias_get_id(dev.of_node, "fiu");
    if (id < 0 || id >= fiu_data_match.fiu_max) {
    dev_err(dev, "Invalid platform device id: %d\n", id);
    return -EINVAL;
    }
    fiu.info = &fiu_data_match.npcm_fiu_data_info[id];
    fiu.dev = dev;
    regbase = devm_platform_ioremap_resource_byname(pdev, "control");
    if (IS_ERR(regbase))
    return PTR_ERR(regbase);
    fiu.regmap = devm_regmap_init_mmio(dev, regbase,
    &npcm_mtd_regmap_config);
    if (IS_ERR(fiu.regmap)) {
    dev_err(dev, "Failed to create regmap\n");
    return PTR_ERR(fiu.regmap);
    }
    fiu.res_mem = platform_get_resource_byname(pdev, IORESOURCE_MEM,
    "memory");
    fiu.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(fiu.clk))
    return PTR_ERR(fiu.clk);
    fiu.spix_mode = of_property_read_bool(dev.of_node,
    "nuvoton,spix-mode");
    ctrl.mode_bits = SPI_RX_DUAL | SPI_RX_QUAD
    | SPI_TX_DUAL | SPI_TX_QUAD;
    ctrl.setup = npcm_fiu_setup;
    ctrl.bus_num = -1;
    ctrl.mem_ops = &npcm_fiu_mem_ops;
    ctrl.num_chipselect = fiu.info.max_cs;
    return devm_spi_register_controller(dev, ctrl);
    }
    MODULE_DEVICE_TABLE(of, npcm_fiu_dt_ids);
    static struct platform_driver npcm_fiu_driver = {
    .driver = {
    .name = "NPCM-FIU",
    .bus = &platform_bus_type,
    .of_match_table = npcm_fiu_dt_ids,
    },
    .probe = npcm_fiu_probe,
    };
    module_platform_driver(npcm_fiu_driver);
    MODULE_DESCRIPTION("Nuvoton FLASH Interface Unit SPI Controller Driver");
    MODULE_AUTHOR("Tomer Maimon <tomer.maimon@nuvoton.com>");
    MODULE_LICENSE("GPL v2");
