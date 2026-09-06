//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-cadence-xspi.c
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
// Cadence XSPI flash controller driver
// Copyright (C) 2020-21 Cadence

pub const CDNS_XSPI_MAGIC_NUM_VALUE: c_uint = 0x6522;
pub const CDNS_XSPI_MAX_BANKS: c_int = 8;

//
// Note: below are additional auxiliary registers to
// configure XSPI controller pin-strap settings
//
// PHY DQ timing register
pub const CDNS_XSPI_CCP_PHY_DQ_TIMING: c_uint = 0x0000;
// PHY DQS timing register
pub const CDNS_XSPI_CCP_PHY_DQS_TIMING: c_uint = 0x0004;
// PHY gate loopback control register
pub const CDNS_XSPI_CCP_PHY_GATE_LPBCK_CTRL: c_uint = 0x0008;
// PHY DLL slave control register
pub const CDNS_XSPI_CCP_PHY_DLL_SLAVE_CTRL: c_uint = 0x0010;
// DLL PHY control register
pub const CDNS_XSPI_DLL_PHY_CTRL: c_uint = 0x1034;
// Command registers
pub const CDNS_XSPI_CMD_REG_0: c_uint = 0x0000;
pub const CDNS_XSPI_CMD_REG_1: c_uint = 0x0004;
pub const CDNS_XSPI_CMD_REG_2: c_uint = 0x0008;
pub const CDNS_XSPI_CMD_REG_3: c_uint = 0x000C;
pub const CDNS_XSPI_CMD_REG_4: c_uint = 0x0010;
pub const CDNS_XSPI_CMD_REG_5: c_uint = 0x0014;
// Command status registers
pub const CDNS_XSPI_CMD_STATUS_REG: c_uint = 0x0044;
// Controller status register
pub const CDNS_XSPI_CTRL_STATUS_REG: c_uint = 0x0100;

// Controller interrupt status register
pub const CDNS_XSPI_INTR_STATUS_REG: c_uint = 0x0110;

pub const CDNS_XSPI_TRD_COMP_INTR_STATUS: c_uint = 0x0120;
pub const CDNS_XSPI_TRD_ERR_INTR_STATUS: c_uint = 0x0130;
pub const CDNS_XSPI_TRD_ERR_INTR_EN: c_uint = 0x0134;
// Controller interrupt enable register
pub const CDNS_XSPI_INTR_ENABLE_REG: c_uint = 0x0114;

    CDNS_XSPI_STIG_DONE_EN  | \
    CDNS_XSPI_SDMA_ERROR_EN | \
    CDNS_XSPI_SDMA_TRIGGER_EN)
// Controller config register
pub const CDNS_XSPI_CTRL_CONFIG_REG: c_uint = 0x0230;

pub const CDNS_XSPI_WORK_MODE_DIRECT: c_int = 0;
pub const CDNS_XSPI_WORK_MODE_STIG: c_int = 1;
pub const CDNS_XSPI_WORK_MODE_ACMD: c_int = 3;
// SDMA trigger transaction registers
pub const CDNS_XSPI_SDMA_SIZE_REG: c_uint = 0x0240;
pub const CDNS_XSPI_SDMA_TRD_INFO_REG: c_uint = 0x0244;

// Controller features register
pub const CDNS_XSPI_CTRL_FEATURES_REG: c_uint = 0x0F04;

// Controller version register
pub const CDNS_XSPI_CTRL_VERSION_REG: c_uint = 0x0F00;

// STIG Profile 1.0 instruction fields (split into registers)

// STIG data sequence instruction fields (split into registers)

// STIG command status fields

pub const CDNS_XSPI_TRD_STATUS: c_uint = 0x0104;

pub const MODEBYTES_COUNT: c_int = 1;
// Helper macros for filling command registers

    FIELD_PREP(CDNS_XSPI_CMD_INSTR_TYPE, (data_phase) ? \
    CDNS_XSPI_STIG_INSTR_TYPE_1 : CDNS_XSPI_STIG_INSTR_TYPE_0) | \
    FIELD_PREP(CDNS_XSPI_CMD_P1_R1_ADDR0, (op).addr.val & 0xff))

    FIELD_PREP(CDNS_XSPI_CMD_P1_R2_ADDR1, ((op).addr.val >> 8)  & 0xFF) | \
    FIELD_PREP(CDNS_XSPI_CMD_P1_R2_ADDR2, ((op).addr.val >> 16) & 0xFF) | \
    FIELD_PREP(CDNS_XSPI_CMD_P1_R2_ADDR3, ((op).addr.val >> 24) & 0xFF) | \
    FIELD_PREP(CDNS_XSPI_CMD_P1_R2_ADDR4, ((op).addr.val >> 32) & 0xFF))

    FIELD_PREP(CDNS_XSPI_CMD_P1_R3_ADDR5, ((op).addr.val >> 40) & 0xFF) | \
    FIELD_PREP(CDNS_XSPI_CMD_P1_R3_CMD, (op).cmd.opcode) | \
    FIELD_PREP(MODE_NO_OF_BYTES, modebytes) | \
    FIELD_PREP(CDNS_XSPI_CMD_P1_R3_NUM_ADDR_BYTES, (op).addr.nbytes))

    FIELD_PREP(CDNS_XSPI_CMD_P1_R4_ADDR_IOS, ilog2((op).addr.buswidth)) | \
    FIELD_PREP(CDNS_XSPI_CMD_P1_R4_CMD_IOS, ilog2((op).cmd.buswidth)) | \
    FIELD_PREP(CDNS_XSPI_CMD_P1_R4_BANK, chipsel))

    FIELD_PREP(CDNS_XSPI_CMD_INSTR_TYPE, CDNS_XSPI_STIG_INSTR_TYPE_DATA_SEQ)

    FIELD_PREP(CDNS_XSPI_CMD_DSEQ_R2_DCNT_L, (op).data.nbytes & 0xFFFF)

    FIELD_PREP(CDNS_XSPI_CMD_DSEQ_R3_DCNT_H, \
    ((op).data.nbytes >> 16) & 0xffff) | \
    FIELD_PREP(CDNS_XSPI_CMD_DSEQ_R3_NUM_OF_DUMMY, \
    (op).dummy.buswidth != 0 ? \
    (((dummybytes) * 8) / (op).dummy.buswidth) : \
    0))

    FIELD_PREP(CDNS_XSPI_CMD_DSEQ_R4_BANK, chipsel) | \
    FIELD_PREP(CDNS_XSPI_CMD_DSEQ_R4_DATA_IOS, \
    ilog2((op).data.buswidth)) | \
    FIELD_PREP(CDNS_XSPI_CMD_DSEQ_R4_DIR, \
    ((op).data.dir == SPI_MEM_DATA_IN) ? \
    CDNS_XSPI_STIG_CMD_DIR_READ : CDNS_XSPI_STIG_CMD_DIR_WRITE))
// Helper macros for GENERIC and GENERIC-DSEQ instruction type

pub const INSTRUCTION_TYPE_GENERIC: c_int = 96;

    FIELD_PREP(CDNS_XSPI_CMD_INSTR_TYPE, INSTRUCTION_TYPE_GENERIC))

    FIELD_PREP(GENERIC_NUM_OF_BYTES, len))

    FIELD_PREP(GENERIC_BANK_NUM, cs) | FIELD_PREP(GENERIC_GLUE_CMD, glue))

    FIELD_PREP(CDNS_XSPI_CMD_INSTR_TYPE, CDNS_XSPI_STIG_INSTR_TYPE_DATA_SEQ))

    FIELD_PREP(CDNS_XSPI_CMD_DSEQ_R2_DCNT_L, nbytes & 0xffff))

    FIELD_PREP(CDNS_XSPI_CMD_DSEQ_R3_DCNT_H, (nbytes >> 16) & 0xffff))

    FIELD_PREP(CDNS_XSPI_CMD_DSEQ_R4_BANK, chipsel) | \
    FIELD_PREP(CDNS_XSPI_CMD_DSEQ_R4_DIR, dir))
// Marvell PHY default values
pub const MARVELL_REGS_DLL_PHY_CTRL: c_uint = 0x00000707;
pub const MARVELL_CTB_RFILE_PHY_CTRL: c_uint = 0x00004000;
pub const MARVELL_RFILE_PHY_TSEL: c_uint = 0x00000000;
pub const MARVELL_RFILE_PHY_DQ_TIMING: c_uint = 0x00000101;
pub const MARVELL_RFILE_PHY_DQS_TIMING: c_uint = 0x00700404;
pub const MARVELL_RFILE_PHY_GATE_LPBK_CTRL: c_uint = 0x00200030;
pub const MARVELL_RFILE_PHY_DLL_MASTER_CTRL: c_uint = 0x00800000;
pub const MARVELL_RFILE_PHY_DLL_SLAVE_CTRL: c_uint = 0x0000ff01;
// PHY config registers
pub const CDNS_XSPI_RF_MINICTRL_REGS_DLL_PHY_CTRL: c_uint = 0x1034;
pub const CDNS_XSPI_PHY_CTB_RFILE_PHY_CTRL: c_uint = 0x0080;
pub const CDNS_XSPI_PHY_CTB_RFILE_PHY_TSEL: c_uint = 0x0084;
pub const CDNS_XSPI_PHY_DATASLICE_RFILE_PHY_DQ_TIMING: c_uint = 0x0000;
pub const CDNS_XSPI_PHY_DATASLICE_RFILE_PHY_DQS_TIMING: c_uint = 0x0004;
pub const CDNS_XSPI_PHY_DATASLICE_RFILE_PHY_GATE_LPBK_CTRL: c_uint = 0x0008;
pub const CDNS_XSPI_PHY_DATASLICE_RFILE_PHY_DLL_MASTER_CTRL: c_uint = 0x000c;
pub const CDNS_XSPI_PHY_DATASLICE_RFILE_PHY_DLL_SLAVE_CTRL: c_uint = 0x0010;
pub const CDNS_XSPI_DATASLICE_RFILE_PHY_DLL_OBS_REG_0: c_uint = 0x001c;

// Marvell overlay registers - clock
pub const MRVL_XSPI_CLK_CTRL_AUX_REG: c_uint = 0x2020;

pub const MRVL_XSPI_CLOCK_IO_HZ: c_int = 800000000;

pub const MRVL_DEFAULT_CLK: c_int = 25000000;
// Marvell overlay registers - xfer
pub const MRVL_XFER_FUNC_CTRL: c_uint = 0x210;

pub const MRVL_XFER_QWORD_COUNT: c_int = 32;
pub const MRVL_XFER_QWORD_BYTECOUNT: c_int = 8;
pub const MRVL_XSPI_POLL_TIMEOUT_US: c_int = 1000;
pub const MRVL_XSPI_POLL_DELAY_US: c_int = 10;
// Macros for calculating data bits in generic command
// Up to 10 bytes can be fit into cmd_registers
// least significant is placed in cmd_reg[1]
// Other bits are inserted after it in cmd_reg[1,2,3] register
//

    enum cdns_xspi_stig_instr_type {
    CDNS_XSPI_STIG_INSTR_TYPE_0,
    CDNS_XSPI_STIG_INSTR_TYPE_1,
    CDNS_XSPI_STIG_INSTR_TYPE_DATA_SEQ = 127,
    };
    enum cdns_xspi_sdma_dir {
    CDNS_XSPI_SDMA_DIR_READ,
    CDNS_XSPI_SDMA_DIR_WRITE,
    };
    enum cdns_xspi_stig_cmd_dir {
    CDNS_XSPI_STIG_CMD_DIR_READ,
    CDNS_XSPI_STIG_CMD_DIR_WRITE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_xspi_driver_data {
    pub mrvl_hw_overlay: bool,
    pub dll_phy_ctrl: u32,
    pub ctb_rfile_phy_ctrl: u32,
    pub rfile_phy_tsel: u32,
    pub rfile_phy_dq_timing: u32,
    pub rfile_phy_dqs_timing: u32,
    pub rfile_phy_gate_lpbk_ctrl: u32,
    pub rfile_phy_dll_master_ctrl: u32,
    pub rfile_phy_dll_slave_ctrl: u32,
}

    static struct cdns_xspi_driver_data cdns_driver_data = {
    .mrvl_hw_overlay = false,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_xspi_dev {
    pub pdev: *mut platform_device,
    pub host: *mut spi_controller,
    pub dev: *mut device,
    pub iobase: *mut void __iomem,
    pub auxbase: *mut void __iomem,
    pub sdmabase: *mut void __iomem,
    pub xferbase: *mut void __iomem,
    pub irq: c_int,
    pub cur_cs: c_int,
    pub sdmasize: c_uint,
    pub cmd_complete: completion,
    pub auto_cmd_complete: completion,
    pub sdma_complete: completion,
    pub sdma_error: bool,
    pub in_buffer: *mut c_void,
    pub out_buffer: *const c_void,
// Slave DMA data width in bytes (4 or 8).
    pub dma_data_width: u8,
    pub hw_num_banks: u8,
    pub driver_data: *const cdns_xspi_driver_data,
    pub cdns_xspi): *mut *mut void (sdma_handler)(struct cdns_xspi_dev,
    pub enabled): *mut *mut *mut void (set_interrupts_handler)(struct cdns_xspi_dev cdns_xspi, bool,
    pub xfer_in_progress: bool,
    pub current_xfer_qword: c_int,
}

#[no_mangle]
unsafe extern "C" fn cdns_xspi_wait_for_controller_idle(cdns_xspi: *mut cdns_xspi_dev) -> c_int {
    static int cdns_xspi_wait_for_controller_idle(struct cdns_xspi_dev *cdns_xspi)
    {
    u32 ctrl_stat;
    return readl_relaxed_poll_timeout(cdns_xspi.iobase +
    CDNS_XSPI_CTRL_STATUS_REG,
    ctrl_stat,
    ((ctrl_stat &
    CDNS_XSPI_CTRL_BUSY) == 0),
    100, 1000);
    }
    static void cdns_xspi_trigger_command(struct cdns_xspi_dev *cdns_xspi,
    u32 cmd_regs[6])
    {
    writel(cmd_regs[5], cdns_xspi.iobase + CDNS_XSPI_CMD_REG_5);
    writel(cmd_regs[4], cdns_xspi.iobase + CDNS_XSPI_CMD_REG_4);
    writel(cmd_regs[3], cdns_xspi.iobase + CDNS_XSPI_CMD_REG_3);
    writel(cmd_regs[2], cdns_xspi.iobase + CDNS_XSPI_CMD_REG_2);
    writel(cmd_regs[1], cdns_xspi.iobase + CDNS_XSPI_CMD_REG_1);
    writel(cmd_regs[0], cdns_xspi.iobase + CDNS_XSPI_CMD_REG_0);
    }
#[no_mangle]
unsafe extern "C" fn cdns_xspi_check_command_status(cdns_xspi: *mut cdns_xspi_dev) -> c_int {
    static int cdns_xspi_check_command_status(struct cdns_xspi_dev *cdns_xspi)
    {
    let mut ret: c_int = 0;
    let mut cmd_status: u32 = readl(cdns_xspi.iobase + CDNS_XSPI_CMD_STATUS_REG);
    if (cmd_status & CDNS_XSPI_CMD_STATUS_COMPLETED) {
    if ((cmd_status & CDNS_XSPI_CMD_STATUS_FAILED) != 0) {
    if (cmd_status & CDNS_XSPI_CMD_STATUS_DQS_ERROR) {
    dev_err(cdns_xspi.dev,
    "Incorrect DQS pulses detected\n");
    ret = -EPROTO;
    }
    if (cmd_status & CDNS_XSPI_CMD_STATUS_CRC_ERROR) {
    dev_err(cdns_xspi.dev,
    "CRC error received\n");
    ret = -EPROTO;
    }
    if (cmd_status & CDNS_XSPI_CMD_STATUS_BUS_ERROR) {
    dev_err(cdns_xspi.dev,
    "Error resp on system DMA interface\n");
    ret = -EPROTO;
    }
    if (cmd_status & CDNS_XSPI_CMD_STATUS_INV_SEQ_ERROR) {
    dev_err(cdns_xspi.dev,
    "Invalid command sequence detected\n");
    ret = -EPROTO;
    }
    }
    } else {
    dev_err(cdns_xspi.dev, "Fatal err - command not completed\n");
    ret = -EPROTO;
    }
    return ret;
    }
    static void cdns_xspi_set_interrupts(struct cdns_xspi_dev *cdns_xspi,
    bool enabled)
    {
    u32 intr_enable;
    intr_enable = readl(cdns_xspi.iobase + CDNS_XSPI_INTR_ENABLE_REG);
    if (enabled)
    intr_enable |= CDNS_XSPI_INTR_MASK;
    else
    intr_enable &= ~CDNS_XSPI_INTR_MASK;
    writel(intr_enable, cdns_xspi.iobase + CDNS_XSPI_INTR_ENABLE_REG);
    }
#[no_mangle]
unsafe extern "C" fn cdns_xspi_controller_init(cdns_xspi: *mut cdns_xspi_dev) -> c_int {
    static int cdns_xspi_controller_init(struct cdns_xspi_dev *cdns_xspi)
    {
    u32 ctrl_ver;
    u32 ctrl_features;
    u16 hw_magic_num;
    ctrl_ver = readl(cdns_xspi.iobase + CDNS_XSPI_CTRL_VERSION_REG);
    hw_magic_num = FIELD_GET(CDNS_XSPI_MAGIC_NUM, ctrl_ver);
    if (hw_magic_num != CDNS_XSPI_MAGIC_NUM_VALUE) {
    dev_err(cdns_xspi.dev,
    "Incorrect XSPI magic number: %x, expected: %x\n",
    hw_magic_num, CDNS_XSPI_MAGIC_NUM_VALUE);
    return -EIO;
    }
    ctrl_features = readl(cdns_xspi.iobase + CDNS_XSPI_CTRL_FEATURES_REG);
    cdns_xspi.hw_num_banks = FIELD_GET(CDNS_XSPI_NUM_BANKS, ctrl_features);
    cdns_xspi.dma_data_width = (ctrl_features & CDNS_XSPI_DMA_DATA_WIDTH) ? 8 : 4;
    cdns_xspi.set_interrupts_handler(cdns_xspi, false);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cdns_xspi_sdma_read(cdns_xspi: *mut cdns_xspi_dev, len: usize) {
    static inline void cdns_xspi_sdma_read(struct cdns_xspi_dev *cdns_xspi, size_t len)
    {
    void __iomem *src = cdns_xspi.sdmabase;
    void *buf = cdns_xspi.in_buffer;
    let mut offset: usize = 0;
    if (!IS_ENABLED(CONFIG_64BIT) || cdns_xspi.dma_data_width == 4) {
    if (IS_ALIGNED((uintptr_t)src, 4) && IS_ALIGNED((uintptr_t)buf, 4)) {
    ioread32_rep(src, buf, len >> 2);
    offset = len & ~0x3;
    len -= offset;
    }

    } else {
    if (IS_ALIGNED((uintptr_t)src, 8) && IS_ALIGNED((uintptr_t)buf, 8)) {
    readsq(src, buf, len >> 3);
    offset = len & ~0x7;
    len -= offset;
    }

    }
    ioread8_rep(src, (u8 *)buf + offset, len);
    }
#[no_mangle]
pub unsafe extern "C" fn cdns_xspi_sdma_write(cdns_xspi: *mut cdns_xspi_dev, len: usize) {
    static inline void cdns_xspi_sdma_write(struct cdns_xspi_dev *cdns_xspi, size_t len)
    {
    void __iomem *dst = cdns_xspi.sdmabase;
    const void *buf = cdns_xspi.out_buffer;
    let mut offset: usize = 0;
    if (!IS_ENABLED(CONFIG_64BIT) || cdns_xspi.dma_data_width == 4) {
    if (IS_ALIGNED((uintptr_t)dst, 4) && IS_ALIGNED((uintptr_t)buf, 4)) {
    iowrite32_rep(dst, buf, len >> 2);
    offset = len & ~0x3;
    len -= offset;
    }

    } else {
    if (IS_ALIGNED((uintptr_t)dst, 8) && IS_ALIGNED((uintptr_t)buf, 8)) {
    writesq(dst, buf, len >> 3);
    offset = len & ~0x7;
    len -= offset;
    }

    }
    iowrite8_rep(dst, (const u8 *)buf + offset, len);
    }
#[no_mangle]
unsafe extern "C" fn cdns_xspi_sdma_handle(cdns_xspi: *mut cdns_xspi_dev) {
    static void cdns_xspi_sdma_handle(struct cdns_xspi_dev *cdns_xspi)
    {
    u32 sdma_size, sdma_trd_info;
    u8 sdma_dir;
    sdma_size = readl(cdns_xspi.iobase + CDNS_XSPI_SDMA_SIZE_REG);
    sdma_trd_info = readl(cdns_xspi.iobase + CDNS_XSPI_SDMA_TRD_INFO_REG);
    sdma_dir = FIELD_GET(CDNS_XSPI_SDMA_DIR, sdma_trd_info);
    switch (sdma_dir) {
    case CDNS_XSPI_SDMA_DIR_READ:
    cdns_xspi_sdma_read(cdns_xspi, sdma_size);
    break;
    case CDNS_XSPI_SDMA_DIR_WRITE:
    cdns_xspi_sdma_write(cdns_xspi, sdma_size);
    break;
    }
    }
    static int cdns_xspi_send_stig_command(struct cdns_xspi_dev *cdns_xspi,
    const struct spi_mem_op *op,
    bool data_phase)
    {
    u32 cmd_regs[6];
    u32 cmd_status;
    int ret;
    let mut dummybytes: c_int = op.dummy.nbytes;
    ret = cdns_xspi_wait_for_controller_idle(cdns_xspi);
    if (ret < 0)
    return -EIO;
    writel(FIELD_PREP(CDNS_XSPI_CTRL_WORK_MODE, CDNS_XSPI_WORK_MODE_STIG),
    cdns_xspi.iobase + CDNS_XSPI_CTRL_CONFIG_REG);
    cdns_xspi.set_interrupts_handler(cdns_xspi, true);
    cdns_xspi.sdma_error = false;
    memset(cmd_regs, 0, sizeof(cmd_regs));
    cmd_regs[1] = CDNS_XSPI_CMD_FLD_P1_INSTR_CMD_1(op, data_phase);
    cmd_regs[2] = CDNS_XSPI_CMD_FLD_P1_INSTR_CMD_2(op);
    if (dummybytes != 0) {
    cmd_regs[3] = CDNS_XSPI_CMD_FLD_P1_INSTR_CMD_3(op, 1);
    dummybytes--;
    } else {
    cmd_regs[3] = CDNS_XSPI_CMD_FLD_P1_INSTR_CMD_3(op, 0);
    }
    cmd_regs[4] = CDNS_XSPI_CMD_FLD_P1_INSTR_CMD_4(op,
    cdns_xspi.cur_cs);
    cdns_xspi_trigger_command(cdns_xspi, cmd_regs);
    if (data_phase) {
    cmd_regs[0] = CDNS_XSPI_STIG_DONE_FLAG;
    cmd_regs[1] = CDNS_XSPI_CMD_FLD_DSEQ_CMD_1(op);
    cmd_regs[2] = CDNS_XSPI_CMD_FLD_DSEQ_CMD_2(op);
    cmd_regs[3] = CDNS_XSPI_CMD_FLD_DSEQ_CMD_3(op, dummybytes);
    cmd_regs[4] = CDNS_XSPI_CMD_FLD_DSEQ_CMD_4(op,
    cdns_xspi.cur_cs);
    cdns_xspi.in_buffer = op.data.buf.in;
    cdns_xspi.out_buffer = op.data.buf.out;
    cdns_xspi_trigger_command(cdns_xspi, cmd_regs);
    wait_for_completion(&cdns_xspi.sdma_complete);
    if (cdns_xspi.sdma_error) {
    cdns_xspi.set_interrupts_handler(cdns_xspi, false);
    return -EIO;
    }
    cdns_xspi.sdma_handler(cdns_xspi);
    }
    wait_for_completion(&cdns_xspi.cmd_complete);
    cdns_xspi.set_interrupts_handler(cdns_xspi, false);
    cmd_status = cdns_xspi_check_command_status(cdns_xspi);
    if (cmd_status)
    return -EPROTO;
    return 0;
    }
    static int cdns_xspi_mem_op(struct cdns_xspi_dev *cdns_xspi,
    struct spi_mem *mem,
    const struct spi_mem_op *op)
    {
    let mut dir: enum spi_mem_data_dir = op.data.dir;
    if (cdns_xspi.cur_cs != spi_get_chipselect(mem.spi, 0))
    cdns_xspi.cur_cs = spi_get_chipselect(mem.spi, 0);
    return cdns_xspi_send_stig_command(cdns_xspi, op,
    (dir != SPI_MEM_NO_DATA));
    }
    static int cdns_xspi_mem_op_execute(struct spi_mem *mem,
    const struct spi_mem_op *op)
    {
    struct cdns_xspi_dev *cdns_xspi =
    spi_controller_get_devdata(mem.spi.controller);
    let mut ret: c_int = 0;
    ret = cdns_xspi_mem_op(cdns_xspi, mem, op);
    return ret;
    }
    static bool cdns_xspi_supports_op(struct spi_mem *mem,
    const struct spi_mem_op *op)
    {
    struct spi_device *spi = mem.spi;
    struct device *dev = &spi.dev;
    u32 value;
    if (!device_property_read_u32(dev, "spi-tx-bus-width", &value)) {
    switch (value) {
    case 1:
    break;
    case 2:
    spi.mode |= SPI_TX_DUAL;
    break;
    case 4:
    spi.mode |= SPI_TX_QUAD;
    break;
    case 8:
    spi.mode |= SPI_TX_OCTAL;
    break;
    default:
    dev_warn(dev, "spi-tx-bus-width %u not supported\n", value);
    break;
    }
    }
    if (!device_property_read_u32(dev, "spi-rx-bus-width", &value)) {
    switch (value) {
    case 1:
    break;
    case 2:
    spi.mode |= SPI_RX_DUAL;
    break;
    case 4:
    spi.mode |= SPI_RX_QUAD;
    break;
    case 8:
    spi.mode |= SPI_RX_OCTAL;
    break;
    default:
    dev_warn(dev, "spi-rx-bus-width %u not supported\n", value);
    break;
    }
    }
    if (!spi_mem_default_supports_op(mem, op))
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn cdns_xspi_adjust_mem_op_size(mem: *mut spi_mem, op: *mut spi_mem_op) -> c_int {
    static int cdns_xspi_adjust_mem_op_size(struct spi_mem *mem, struct spi_mem_op *op)
    {
    struct cdns_xspi_dev *cdns_xspi =
    spi_controller_get_devdata(mem.spi.controller);
    op.data.nbytes = clamp_val(op.data.nbytes, 0, cdns_xspi.sdmasize);
    return 0;
    }
    static const struct spi_controller_mem_ops cadence_xspi_mem_ops = {
    .supports_op = PTR_IF(IS_ENABLED(CONFIG_ACPI), cdns_xspi_supports_op),
    .exec_op = cdns_xspi_mem_op_execute,
    .adjust_op_size = cdns_xspi_adjust_mem_op_size,
    };
#[no_mangle]
unsafe extern "C" fn cdns_xspi_irq_handler(this_irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t cdns_xspi_irq_handler(int this_irq, void *dev)
    {
    struct cdns_xspi_dev *cdns_xspi = dev;
    u32 irq_status;
    let mut result: irqreturn_t = IRQ_NONE;
    irq_status = readl(cdns_xspi.iobase + CDNS_XSPI_INTR_STATUS_REG);
    writel(irq_status, cdns_xspi.iobase + CDNS_XSPI_INTR_STATUS_REG);
    if (irq_status &
    (CDNS_XSPI_SDMA_ERROR | CDNS_XSPI_SDMA_TRIGGER |
    CDNS_XSPI_STIG_DONE)) {
    if (irq_status & CDNS_XSPI_SDMA_ERROR) {
    dev_err(cdns_xspi.dev,
    "Slave DMA transaction error\n");
    cdns_xspi.sdma_error = true;
    complete(&cdns_xspi.sdma_complete);
    }
    if (irq_status & CDNS_XSPI_SDMA_TRIGGER)
    complete(&cdns_xspi.sdma_complete);
    if (irq_status & CDNS_XSPI_STIG_DONE)
    complete(&cdns_xspi.cmd_complete);
    result = IRQ_HANDLED;
    }
    irq_status = readl(cdns_xspi.iobase + CDNS_XSPI_TRD_COMP_INTR_STATUS);
    if (irq_status) {
    writel(irq_status,
    cdns_xspi.iobase + CDNS_XSPI_TRD_COMP_INTR_STATUS);
    complete(&cdns_xspi.auto_cmd_complete);
    result = IRQ_HANDLED;
    }
    return result;
    }
#[no_mangle]
unsafe extern "C" fn cdns_xspi_of_get_plat_data(pdev: *mut platform_device) -> c_int {
    static int cdns_xspi_of_get_plat_data(struct platform_device *pdev)
    {
    struct fwnode_handle *fwnode_child;
    unsigned int cs;
    device_for_each_child_node(&pdev.dev, fwnode_child) {
    if (!fwnode_device_is_available(fwnode_child))
    continue;
    if (fwnode_property_read_u32(fwnode_child, "reg", &cs)) {
    dev_err(&pdev.dev, "Couldn't get memory chip select\n");
    fwnode_handle_put(fwnode_child);
    return -ENXIO;
    } else if (cs >= CDNS_XSPI_MAX_BANKS) {
    dev_err(&pdev.dev, "reg (cs) parameter value too large\n");
    fwnode_handle_put(fwnode_child);
    return -ENXIO;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_xspi_print_phy_config(cdns_xspi: *mut cdns_xspi_dev) {
    static void cdns_xspi_print_phy_config(struct cdns_xspi_dev *cdns_xspi)
    {
    struct device *dev = cdns_xspi.dev;
    dev_info(dev, "PHY configuration\n");
    dev_info(dev, "   * xspi_dll_phy_ctrl: %08x\n",
    readl(cdns_xspi.iobase + CDNS_XSPI_DLL_PHY_CTRL));
    dev_info(dev, "   * phy_dq_timing: %08x\n",
    readl(cdns_xspi.auxbase + CDNS_XSPI_CCP_PHY_DQ_TIMING));
    dev_info(dev, "   * phy_dqs_timing: %08x\n",
    readl(cdns_xspi.auxbase + CDNS_XSPI_CCP_PHY_DQS_TIMING));
    dev_info(dev, "   * phy_gate_loopback_ctrl: %08x\n",
    readl(cdns_xspi.auxbase + CDNS_XSPI_CCP_PHY_GATE_LPBCK_CTRL));
    dev_info(dev, "   * phy_dll_slave_ctrl: %08x\n",
    readl(cdns_xspi.auxbase + CDNS_XSPI_CCP_PHY_DLL_SLAVE_CTRL));
    }

    static struct cdns_xspi_driver_data marvell_driver_data = {
    .mrvl_hw_overlay = true,
    .dll_phy_ctrl = MARVELL_REGS_DLL_PHY_CTRL,
    .ctb_rfile_phy_ctrl = MARVELL_CTB_RFILE_PHY_CTRL,
    .rfile_phy_tsel = MARVELL_RFILE_PHY_TSEL,
    .rfile_phy_dq_timing = MARVELL_RFILE_PHY_DQ_TIMING,
    .rfile_phy_dqs_timing = MARVELL_RFILE_PHY_DQS_TIMING,
    .rfile_phy_gate_lpbk_ctrl = MARVELL_RFILE_PHY_GATE_LPBK_CTRL,
    .rfile_phy_dll_master_ctrl = MARVELL_RFILE_PHY_DLL_MASTER_CTRL,
    .rfile_phy_dll_slave_ctrl = MARVELL_RFILE_PHY_DLL_SLAVE_CTRL,
    };
    static const int cdns_mrvl_xspi_clk_div_list[] = {
    4,	//0x0 = Divide by 4.   SPI clock is 200 MHz.
    6,	//0x1 = Divide by 6.   SPI clock is 133.33 MHz.
    8,	//0x2 = Divide by 8.   SPI clock is 100 MHz.
    10,	//0x3 = Divide by 10.  SPI clock is 80 MHz.
    12,	//0x4 = Divide by 12.  SPI clock is 66.666 MHz.
    16,	//0x5 = Divide by 16.  SPI clock is 50 MHz.
    18,	//0x6 = Divide by 18.  SPI clock is 44.44 MHz.
    20,	//0x7 = Divide by 20.  SPI clock is 40 MHz.
    24,	//0x8 = Divide by 24.  SPI clock is 33.33 MHz.
    32,	//0x9 = Divide by 32.  SPI clock is 25 MHz.
    40,	//0xA = Divide by 40.  SPI clock is 20 MHz.
    50,	//0xB = Divide by 50.  SPI clock is 16 MHz.
    64,	//0xC = Divide by 64.  SPI clock is 12.5 MHz.
    128	//0xD = Divide by 128. SPI clock is 6.25 MHz.
    };
#[no_mangle]
unsafe extern "C" fn cdns_xspi_reset_dll(cdns_xspi: *mut cdns_xspi_dev) {
    static void cdns_xspi_reset_dll(struct cdns_xspi_dev *cdns_xspi)
    {
    u32 dll_cntrl = readl(cdns_xspi.iobase +
    CDNS_XSPI_RF_MINICTRL_REGS_DLL_PHY_CTRL);
// Reset DLL
    dll_cntrl |= CDNS_XSPI_DLL_RST_N;
    writel(dll_cntrl, cdns_xspi.iobase +
    CDNS_XSPI_RF_MINICTRL_REGS_DLL_PHY_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn cdns_xspi_is_dll_locked(cdns_xspi: *mut cdns_xspi_dev) -> bool {
    static bool cdns_xspi_is_dll_locked(struct cdns_xspi_dev *cdns_xspi)
    {
    u32 dll_lock;
    return !readl_relaxed_poll_timeout(cdns_xspi.iobase +
    CDNS_XSPI_INTR_STATUS_REG,
    dll_lock, ((dll_lock & CDNS_XSPI_DLL_LOCK) == 1), 10, 10000);
    }
// Static configuration of PHY
#[no_mangle]
unsafe extern "C" fn cdns_xspi_configure_phy(cdns_xspi: *mut cdns_xspi_dev) -> bool {
    static bool cdns_xspi_configure_phy(struct cdns_xspi_dev *cdns_xspi)
    {
    writel(cdns_xspi.driver_data.dll_phy_ctrl,
    cdns_xspi.iobase + CDNS_XSPI_RF_MINICTRL_REGS_DLL_PHY_CTRL);
    writel(cdns_xspi.driver_data.ctb_rfile_phy_ctrl,
    cdns_xspi.auxbase + CDNS_XSPI_PHY_CTB_RFILE_PHY_CTRL);
    writel(cdns_xspi.driver_data.rfile_phy_tsel,
    cdns_xspi.auxbase + CDNS_XSPI_PHY_CTB_RFILE_PHY_TSEL);
    writel(cdns_xspi.driver_data.rfile_phy_dq_timing,
    cdns_xspi.auxbase + CDNS_XSPI_PHY_DATASLICE_RFILE_PHY_DQ_TIMING);
    writel(cdns_xspi.driver_data.rfile_phy_dqs_timing,
    cdns_xspi.auxbase + CDNS_XSPI_PHY_DATASLICE_RFILE_PHY_DQS_TIMING);
    writel(cdns_xspi.driver_data.rfile_phy_gate_lpbk_ctrl,
    cdns_xspi.auxbase + CDNS_XSPI_PHY_DATASLICE_RFILE_PHY_GATE_LPBK_CTRL);
    writel(cdns_xspi.driver_data.rfile_phy_dll_master_ctrl,
    cdns_xspi.auxbase + CDNS_XSPI_PHY_DATASLICE_RFILE_PHY_DLL_MASTER_CTRL);
    writel(cdns_xspi.driver_data.rfile_phy_dll_slave_ctrl,
    cdns_xspi.auxbase + CDNS_XSPI_PHY_DATASLICE_RFILE_PHY_DLL_SLAVE_CTRL);
    cdns_xspi_reset_dll(cdns_xspi);
    return cdns_xspi_is_dll_locked(cdns_xspi);
    }
    static bool cdns_mrvl_xspi_setup_clock(struct cdns_xspi_dev *cdns_xspi,
    int requested_clk)
    {
    let mut i: c_int = 0;
    int clk_val;
    u32 clk_reg;
    let mut update_clk: bool = false;
    while (i < (ARRAY_SIZE(cdns_mrvl_xspi_clk_div_list) - 1)) {
    clk_val = MRVL_XSPI_CLOCK_DIVIDED(
    cdns_mrvl_xspi_clk_div_list[i]);
    if (clk_val <= requested_clk)
    break;
    i++;
    }
    dev_dbg(cdns_xspi.dev, "Found clk div: %d, clk val: %d\n",
    cdns_mrvl_xspi_clk_div_list[i],
    MRVL_XSPI_CLOCK_DIVIDED(
    cdns_mrvl_xspi_clk_div_list[i]));
    clk_reg = readl(cdns_xspi.auxbase + MRVL_XSPI_CLK_CTRL_AUX_REG);
    if (FIELD_GET(MRVL_XSPI_CLK_DIV, clk_reg) != i) {
    clk_reg &= ~MRVL_XSPI_CLK_ENABLE;
    writel(clk_reg,
    cdns_xspi.auxbase + MRVL_XSPI_CLK_CTRL_AUX_REG);
    clk_reg = FIELD_PREP(MRVL_XSPI_CLK_DIV, i);
    FIELD_MODIFY(MRVL_XSPI_CLK_DIV, &clk_reg, i);
    clk_reg |= MRVL_XSPI_CLK_ENABLE;
    clk_reg |= MRVL_XSPI_IRQ_ENABLE;
    update_clk = true;
    }
    if (update_clk)
    writel(clk_reg,
    cdns_xspi.auxbase + MRVL_XSPI_CLK_CTRL_AUX_REG);
    return update_clk;
    }
    static void marvell_xspi_set_interrupts(struct cdns_xspi_dev *cdns_xspi,
    bool enabled)
    {
    u32 intr_enable;
    u32 irq_status;
    irq_status = readl(cdns_xspi.iobase + CDNS_XSPI_INTR_STATUS_REG);
    writel(irq_status, cdns_xspi.iobase + CDNS_XSPI_INTR_STATUS_REG);
    intr_enable = readl(cdns_xspi.iobase + CDNS_XSPI_INTR_ENABLE_REG);
    if (enabled)
    intr_enable |= CDNS_XSPI_INTR_MASK;
    else
    intr_enable &= ~CDNS_XSPI_INTR_MASK;
    writel(intr_enable, cdns_xspi.iobase + CDNS_XSPI_INTR_ENABLE_REG);
    }
    static int marvell_xspi_mem_op_execute(struct spi_mem *mem,
    const struct spi_mem_op *op)
    {
    struct cdns_xspi_dev *cdns_xspi =
    spi_controller_get_devdata(mem.spi.controller);
    let mut ret: c_int = 0;
    cdns_mrvl_xspi_setup_clock(cdns_xspi, mem.spi.max_speed_hz);
    ret = cdns_xspi_mem_op(cdns_xspi, mem, op);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn m_ioreadq(addr: *mut void __iomem, buf: *mut c_void, len: c_int) {
    static void m_ioreadq(void __iomem  *addr, void *buf, int len)
    {
    if (IS_ALIGNED((long)buf, 8) && len >= 8) {
    let mut full_ops: u64 = len / 8;
    u64 *buffer = buf;
    len -= full_ops * 8;
    buf += full_ops * 8;
    do {
    let mut b: u64 = readq(addr);
// buffer++ = b;
    } while (--full_ops);
    }
    while (len) {
    u64 tmp_buf;
    tmp_buf = readq(addr);
    memcpy(buf, &tmp_buf, min(len, 8));
    len = len > 8 ? len - 8 : 0;
    buf += 8;
    }
    }
#[no_mangle]
unsafe extern "C" fn m_iowriteq(addr: *mut void __iomem, buf: *const c_void, len: c_int) {
    static void m_iowriteq(void __iomem *addr, const void *buf, int len)
    {
    if (IS_ALIGNED((long)buf, 8) && len >= 8) {
    let mut full_ops: u64 = len / 8;
    const u64 *buffer = buf;
    len -= full_ops * 8;
    buf += full_ops * 8;
    do {
    writeq(*buffer++, addr);
    } while (--full_ops);
    }
    while (len) {
    u64 tmp_buf;
    memcpy(&tmp_buf, buf, min(len, 8));
    writeq(tmp_buf, addr);
    len = len > 8 ? len - 8 : 0;
    buf += 8;
    }
    }
#[no_mangle]
unsafe extern "C" fn marvell_xspi_sdma_handle(cdns_xspi: *mut cdns_xspi_dev) {
    static void marvell_xspi_sdma_handle(struct cdns_xspi_dev *cdns_xspi)
    {
    u32 sdma_size, sdma_trd_info;
    u8 sdma_dir;
    sdma_size = readl(cdns_xspi.iobase + CDNS_XSPI_SDMA_SIZE_REG);
    sdma_trd_info = readl(cdns_xspi.iobase + CDNS_XSPI_SDMA_TRD_INFO_REG);
    sdma_dir = FIELD_GET(CDNS_XSPI_SDMA_DIR, sdma_trd_info);
    switch (sdma_dir) {
    case CDNS_XSPI_SDMA_DIR_READ:
    m_ioreadq(cdns_xspi.sdmabase,
    cdns_xspi.in_buffer, sdma_size);
    break;
    case CDNS_XSPI_SDMA_DIR_WRITE:
    m_iowriteq(cdns_xspi.sdmabase,
    cdns_xspi.out_buffer, sdma_size);
    break;
    }
    }
    static const struct spi_controller_mem_ops marvell_xspi_mem_ops = {
    .supports_op = PTR_IF(IS_ENABLED(CONFIG_ACPI), cdns_xspi_supports_op),
    .exec_op = marvell_xspi_mem_op_execute,
    .adjust_op_size = cdns_xspi_adjust_mem_op_size,
    };
#[no_mangle]
unsafe extern "C" fn cdns_xspi_prepare_generic(cs: c_int, dout: *const c_void, len: c_int, glue: c_int, cmd_regs: *mut u32) -> c_int {
    static int cdns_xspi_prepare_generic(int cs, const void *dout, int len, int glue, u32 *cmd_regs)
    {
    u8 *data = (u8 *)dout;
    int i;
    let mut data_counter: c_int = 0;
    memset(cmd_regs, 0x00, CMD_REG_LEN);
    if (GENERIC_CMD_REG_3_NEEDED(len)) {
    for (i = GENERIC_CMD_DATA_REG_3_COUNT(len); i >= 0 ; i--)
    cmd_regs[3] |= GENERIC_CMD_DATA_INSERT(data[data_counter++],
    GENERIC_CMD_DATA_3_OFFSET(i));
    }
    if (GENERIC_CMD_REG_2_NEEDED(len)) {
    for (i = GENERIC_CMD_DATA_REG_2_COUNT(len); i >= 0; i--)
    cmd_regs[2] |= GENERIC_CMD_DATA_INSERT(data[data_counter++],
    GENERIC_CMD_DATA_2_OFFSET(i));
    }
    for (i = GENERIC_CMD_DATA_REG_1_COUNT(len); i >= 0 ; i--)
    cmd_regs[1] |= GENERIC_CMD_DATA_INSERT(data[data_counter++],
    GENERIC_CMD_DATA_1_OFFSET(i));
    cmd_regs[1] |= CDNS_XSPI_CMD_FLD_P1_GENERIC_CMD;
    cmd_regs[3] |= CDNS_XSPI_CMD_FLD_P3_GENERIC_CMD(len);
    cmd_regs[4] |= CDNS_XSPI_CMD_FLD_P4_GENERIC_CMD(cs, glue);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn marvell_xspi_read_single_qword(cdns_xspi: *mut cdns_xspi_dev, buffer: *mut u8) {
    static void marvell_xspi_read_single_qword(struct cdns_xspi_dev *cdns_xspi, u8 **buffer)
    {
    u64 d = readq(cdns_xspi.xferbase +
    MRVL_XFER_FUNC_CTRL_READ_DATA(cdns_xspi.current_xfer_qword));
    u8 *ptr = (u8 *)&d;
    int k;
    for (k = 0; k < 8; k++) {
    let mut val: u8 = bitrev8((ptr[k]));
// buffer = val;
// buffer = *buffer + 1;
    }
    cdns_xspi.current_xfer_qword++;
    cdns_xspi.current_xfer_qword %= MRVL_XFER_QWORD_COUNT;
    }
#[no_mangle]
unsafe extern "C" fn cdns_xspi_finish_read(cdns_xspi: *mut cdns_xspi_dev, buffer: *mut u8, data_count: u32) {
    static void cdns_xspi_finish_read(struct cdns_xspi_dev *cdns_xspi, u8 **buffer, u32 data_count)
    {
    u64 d = readq(cdns_xspi.xferbase +
    MRVL_XFER_FUNC_CTRL_READ_DATA(cdns_xspi.current_xfer_qword));
    u8 *ptr = (u8 *)&d;
    int k;
    for (k = 0; k < data_count % MRVL_XFER_QWORD_BYTECOUNT; k++) {
    let mut val: u8 = bitrev8((ptr[k]));
// buffer = val;
// buffer = *buffer + 1;
    }
    cdns_xspi.current_xfer_qword++;
    cdns_xspi.current_xfer_qword %= MRVL_XFER_QWORD_COUNT;
    }
#[no_mangle]
unsafe extern "C" fn cdns_xspi_prepare_transfer(cs: c_int, dir: c_int, len: c_int, cmd_regs: *mut u32) -> c_int {
    static int cdns_xspi_prepare_transfer(int cs, int dir, int len, u32 *cmd_regs)
    {
    memset(cmd_regs, 0x00, CMD_REG_LEN);
    cmd_regs[1] |= CDNS_XSPI_CMD_FLD_GENERIC_DSEQ_CMD_1;
    cmd_regs[2] |= CDNS_XSPI_CMD_FLD_GENERIC_DSEQ_CMD_2(len);
    cmd_regs[4] |= CDNS_XSPI_CMD_FLD_GENERIC_DSEQ_CMD_4(dir, cs);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_xspi_is_stig_ready(cdns_xspi: *mut cdns_xspi_dev, sleep: bool) -> bool {
    static bool cdns_xspi_is_stig_ready(struct cdns_xspi_dev *cdns_xspi, bool sleep)
    {
    u32 ctrl_stat;
    return !readl_relaxed_poll_timeout
    (cdns_xspi.iobase + CDNS_XSPI_CTRL_STATUS_REG,
    ctrl_stat,
    ((ctrl_stat & BIT(3)) == 0),
    sleep ? MRVL_XSPI_POLL_DELAY_US : 0,
    sleep ? MRVL_XSPI_POLL_TIMEOUT_US : 0);
    }
#[no_mangle]
unsafe extern "C" fn cdns_xspi_is_sdma_ready(cdns_xspi: *mut cdns_xspi_dev, sleep: bool) -> bool {
    static bool cdns_xspi_is_sdma_ready(struct cdns_xspi_dev *cdns_xspi, bool sleep)
    {
    u32 ctrl_stat;
    return !readl_relaxed_poll_timeout
    (cdns_xspi.iobase + CDNS_XSPI_INTR_STATUS_REG,
    ctrl_stat,
    (ctrl_stat & CDNS_XSPI_SDMA_TRIGGER),
    sleep ? MRVL_XSPI_POLL_DELAY_US : 0,
    sleep ? MRVL_XSPI_POLL_TIMEOUT_US : 0);
    }
    static int cdns_xspi_transfer_one_message_b0(struct spi_controller *controller,
    struct spi_message *m)
    {
    struct cdns_xspi_dev *cdns_xspi = spi_controller_get_devdata(controller);
    struct spi_device *spi = m.spi;
    struct spi_transfer *t = core::ptr::null_mut();
    let mut max_len: c_uint = MRVL_XFER_QWORD_BYTECOUNT * MRVL_XFER_QWORD_COUNT;
    int current_transfer_len;
    let mut cs: c_int = spi_get_chipselect(spi, 0);
    let mut cs_change: c_int = 0;
// Enable xfer state machine
    if (!cdns_xspi.xfer_in_progress) {
    let mut xfer_control: u32 = readl(cdns_xspi.xferbase + MRVL_XFER_FUNC_CTRL);
    cdns_xspi.current_xfer_qword = 0;
    cdns_xspi.xfer_in_progress = true;
    xfer_control |= (MRVL_XFER_RECEIVE_ENABLE |
    MRVL_XFER_CLK_CAPTURE_POL |
    MRVL_XFER_FUNC_START |
    MRVL_XFER_SOFT_RESET |
    FIELD_PREP(MRVL_XFER_CS_N_HOLD, (1 << cs)));
    xfer_control &= ~(MRVL_XFER_FUNC_ENABLE | MRVL_XFER_CLK_DRIVE_POL);
    writel(xfer_control, cdns_xspi.xferbase + MRVL_XFER_FUNC_CTRL);
    }
    list_for_each_entry(t, &m.transfers, transfer_list) {
    u8 *txd = (u8 *) t.tx_buf;
    u8 *rxd = (u8 *) t.rx_buf;
    u8 data[10];
    u32 cmd_regs[6];
    if (!txd)
    txd = data;
    cdns_xspi.in_buffer = txd + 1;
    cdns_xspi.out_buffer = txd + 1;
    while (t.len) {
    current_transfer_len = min(max_len, t.len);
    if (current_transfer_len < 10) {
    cdns_xspi_prepare_generic(cs, txd, current_transfer_len,
    false, cmd_regs);
    cdns_xspi_trigger_command(cdns_xspi, cmd_regs);
    if (!cdns_xspi_is_stig_ready(cdns_xspi, true))
    return -EIO;
    } else {
    cdns_xspi_prepare_generic(cs, txd, 1, true, cmd_regs);
    cdns_xspi_trigger_command(cdns_xspi, cmd_regs);
    cdns_xspi_prepare_transfer(cs, 1, current_transfer_len - 1,
    cmd_regs);
    cdns_xspi_trigger_command(cdns_xspi, cmd_regs);
    if (!cdns_xspi_is_sdma_ready(cdns_xspi, true))
    return -EIO;
    cdns_xspi.sdma_handler(cdns_xspi);
    if (!cdns_xspi_is_stig_ready(cdns_xspi, true))
    return -EIO;
    cdns_xspi.in_buffer += current_transfer_len;
    cdns_xspi.out_buffer += current_transfer_len;
    }
    if (rxd) {
    int j;
    for (j = 0; j < current_transfer_len / 8; j++)
    marvell_xspi_read_single_qword(cdns_xspi, &rxd);
    cdns_xspi_finish_read(cdns_xspi, &rxd, current_transfer_len);
    } else {
    cdns_xspi.current_xfer_qword += current_transfer_len /
    MRVL_XFER_QWORD_BYTECOUNT;
    if (current_transfer_len % MRVL_XFER_QWORD_BYTECOUNT)
    cdns_xspi.current_xfer_qword++;
    cdns_xspi.current_xfer_qword %= MRVL_XFER_QWORD_COUNT;
    }
    cs_change = t.cs_change;
    t.len -= current_transfer_len;
    }
    spi_transfer_delay_exec(t);
    }
    if (!cs_change) {
    let mut xfer_control: u32 = readl(cdns_xspi.xferbase + MRVL_XFER_FUNC_CTRL);
    xfer_control &= ~(MRVL_XFER_RECEIVE_ENABLE |
    MRVL_XFER_SOFT_RESET);
    writel(xfer_control, cdns_xspi.xferbase + MRVL_XFER_FUNC_CTRL);
    cdns_xspi.xfer_in_progress = false;
    }
    m.status = 0;
    spi_finalize_current_message(controller);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn cdns_xspi_probe(pdev: *mut platform_device) -> c_int {
    static int cdns_xspi_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct spi_controller *host = core::ptr::null_mut();
    struct cdns_xspi_dev *cdns_xspi = core::ptr::null_mut();
    struct resource *res;
    int ret;
    host = devm_spi_alloc_host(dev, sizeof(*cdns_xspi));
    if (!host)
    return -ENOMEM;
    host.mode_bits = SPI_3WIRE | SPI_TX_DUAL  | SPI_TX_QUAD  |
    SPI_RX_DUAL | SPI_RX_QUAD | SPI_TX_OCTAL | SPI_RX_OCTAL |
    SPI_MODE_0  | SPI_MODE_3;
    cdns_xspi = spi_controller_get_devdata(host);
    cdns_xspi.driver_data = device_get_match_data(dev);
    if (!cdns_xspi.driver_data)
    return -ENODEV;
    host.mem_ops = &cadence_xspi_mem_ops;
    cdns_xspi.sdma_handler = &cdns_xspi_sdma_handle;
    cdns_xspi.set_interrupts_handler = &cdns_xspi_set_interrupts;

    if (cdns_xspi.driver_data.mrvl_hw_overlay) {
    host.mem_ops = &marvell_xspi_mem_ops;
    host.transfer_one_message = cdns_xspi_transfer_one_message_b0;
    cdns_xspi.sdma_handler = &marvell_xspi_sdma_handle;
    cdns_xspi.set_interrupts_handler = &marvell_xspi_set_interrupts;
    }

    host.bus_num = -1;
    platform_set_drvdata(pdev, cdns_xspi);
    cdns_xspi.pdev = pdev;
    cdns_xspi.host = host;
    cdns_xspi.dev = &pdev.dev;
    cdns_xspi.cur_cs = 0;
    init_completion(&cdns_xspi.cmd_complete);
    init_completion(&cdns_xspi.auto_cmd_complete);
    init_completion(&cdns_xspi.sdma_complete);
    ret = cdns_xspi_of_get_plat_data(pdev);
    if (ret)
    return -ENODEV;
    cdns_xspi.iobase = devm_platform_ioremap_resource_byname(pdev, "io");
    if (IS_ERR(cdns_xspi.iobase)) {
    cdns_xspi.iobase = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(cdns_xspi.iobase)) {
    dev_err(dev, "Failed to remap controller base address\n");
    return PTR_ERR(cdns_xspi.iobase);
    }
    }
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "sdma");
    cdns_xspi.sdmabase = devm_ioremap_resource(dev, res);
    if (IS_ERR(cdns_xspi.sdmabase)) {
    res = platform_get_resource(pdev, IORESOURCE_MEM, 1);
    cdns_xspi.sdmabase = devm_ioremap_resource(dev, res);
    if (IS_ERR(cdns_xspi.sdmabase))
    return PTR_ERR(cdns_xspi.sdmabase);
    }
    cdns_xspi.sdmasize = resource_size(res);
    cdns_xspi.auxbase = devm_platform_ioremap_resource_byname(pdev, "aux");
    if (IS_ERR(cdns_xspi.auxbase)) {
    cdns_xspi.auxbase = devm_platform_ioremap_resource(pdev, 2);
    if (IS_ERR(cdns_xspi.auxbase)) {
    dev_err(dev, "Failed to remap AUX address\n");
    return PTR_ERR(cdns_xspi.auxbase);
    }
    }

    if (cdns_xspi.driver_data.mrvl_hw_overlay) {
    cdns_xspi.xferbase = devm_platform_ioremap_resource_byname(pdev, "xfer");
    if (IS_ERR(cdns_xspi.xferbase)) {
    cdns_xspi.xferbase = devm_platform_ioremap_resource(pdev, 3);
    if (IS_ERR(cdns_xspi.xferbase)) {
    dev_info(dev, "XFER register base not found, set it\n");
// For compatibility with older firmware
    cdns_xspi.xferbase = cdns_xspi.iobase + 0x8000;
    }
    }
    }

    cdns_xspi.irq = platform_get_irq(pdev, 0);
    if (cdns_xspi.irq < 0)
    return -ENXIO;
    ret = devm_request_irq(dev, cdns_xspi.irq, cdns_xspi_irq_handler,
    IRQF_SHARED, pdev.name, cdns_xspi);
    if (ret) {
    dev_err(dev, "Failed to request IRQ: %d\n", cdns_xspi.irq);
    return ret;
    }

    if (cdns_xspi.driver_data.mrvl_hw_overlay) {
    cdns_mrvl_xspi_setup_clock(cdns_xspi, MRVL_DEFAULT_CLK);
    cdns_xspi_configure_phy(cdns_xspi);
    }

    cdns_xspi_print_phy_config(cdns_xspi);
    ret = cdns_xspi_controller_init(cdns_xspi);
    if (ret) {
    dev_err(dev, "Failed to initialize controller\n");
    return ret;
    }
    host.num_chipselect = 1 << cdns_xspi.hw_num_banks;
    ret = devm_spi_register_controller(dev, host);
    if (ret) {
    dev_err(dev, "Failed to register SPI host\n");
    return ret;
    }
    dev_info(dev, "Successfully registered SPI host\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_xspi_suspend(dev: *mut device) -> c_int {
    static int cdns_xspi_suspend(struct device *dev)
    {
    struct cdns_xspi_dev *cdns_xspi = dev_get_drvdata(dev);
    return spi_controller_suspend(cdns_xspi.host);
    }
#[no_mangle]
unsafe extern "C" fn cdns_xspi_resume(dev: *mut device) -> c_int {
    static int cdns_xspi_resume(struct device *dev)
    {
    struct cdns_xspi_dev *cdns_xspi = dev_get_drvdata(dev);

    if (cdns_xspi.driver_data.mrvl_hw_overlay) {
    cdns_mrvl_xspi_setup_clock(cdns_xspi, MRVL_DEFAULT_CLK);
    cdns_xspi_configure_phy(cdns_xspi);
    }

    cdns_xspi.set_interrupts_handler(cdns_xspi, false);
    return spi_controller_resume(cdns_xspi.host);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(cdns_xspi_pm_ops,
    cdns_xspi_suspend, cdns_xspi_resume);
    static const struct of_device_id cdns_xspi_of_match[] = {
    {
    .compatible = "cdns,xspi-nor",
    .data = &cdns_driver_data,
    },

    {
    .compatible = "marvell,cn10-xspi-nor",
    .data = &marvell_driver_data,
    },

    { /* end of table */}
    };
    MODULE_DEVICE_TABLE(of, cdns_xspi_of_match);
    static struct platform_driver cdns_xspi_platform_driver = {
    .probe          = cdns_xspi_probe,
    .driver = {
    .name = CDNS_XSPI_NAME,
    .of_match_table = cdns_xspi_of_match,
    .pm = pm_sleep_ptr(&cdns_xspi_pm_ops),
    },
    };
    module_platform_driver(cdns_xspi_platform_driver);
    MODULE_DESCRIPTION("Cadence XSPI Controller Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:" CDNS_XSPI_NAME);
    MODULE_AUTHOR("Konrad Kociolek <konrad@cadence.com>");
    MODULE_AUTHOR("Jayshri Pawar <jpawar@cadence.com>");
    MODULE_AUTHOR("Parshuram Thombare <pthombar@cadence.com>");
