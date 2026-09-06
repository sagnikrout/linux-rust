//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/cal/cal_regs.h
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
// TI CAL camera interface driver
//
// Copyright (c) 2015 Texas Instruments Inc.
//
// Benoit Parrot, <bparrot@ti.com>
//
// struct cal_dev.flags possibilities
//
// DRA72_CAL_PRE_ES2_LDO_DISABLE:
// Errata i913: CSI2 LDO Needs to be disabled when module is powered on
//
// Enabling CSI2 LDO shorts it to core supply. It is crucial the 2 CSI2
// LDOs on the device are disabled if CSI-2 module is powered on
// (0x4845 B304 | 0x4845 B384 [28:27] = 0x1) or in ULPS (0x4845 B304
// | 0x4845 B384 [28:27] = 0x2) mode. Common concerns include: high
// current draw on the module supply in active mode.
//
// Errata does not apply when CSI-2 module is powered off
// (0x4845 B304 | 0x4845 B384 [28:27] = 0x0).
//
// SW Workaround:
// Set the following register bits to disable the LDO,
// which is essentially CSI2 REG10 bit 6:
//
// Core 0:  0x4845 B828 = 0x0000 0040
// Core 1:  0x4845 B928 = 0x0000 0040
//

// CAL register offsets
pub const CAL_HL_REVISION: c_uint = 0x0000;
pub const CAL_HL_HWINFO: c_uint = 0x0004;
pub const CAL_HL_SYSCONFIG: c_uint = 0x0010;
pub const CAL_HL_IRQ_EOI: c_uint = 0x001c;

pub const CAL_CTRL: c_uint = 0x100;
pub const CAL_CTRL1: c_uint = 0x104;
pub const CAL_LINE_NUMBER_EVT: c_uint = 0x108;
pub const CAL_VPORT_CTRL1: c_uint = 0x120;
pub const CAL_VPORT_CTRL2: c_uint = 0x124;
pub const CAL_BYS_CTRL1: c_uint = 0x130;
pub const CAL_BYS_CTRL2: c_uint = 0x134;
pub const CAL_RD_DMA_CTRL: c_uint = 0x140;
pub const CAL_RD_DMA_PIX_ADDR: c_uint = 0x144;
pub const CAL_RD_DMA_PIX_OFST: c_uint = 0x148;
pub const CAL_RD_DMA_XSIZE: c_uint = 0x14c;
pub const CAL_RD_DMA_YSIZE: c_uint = 0x150;
pub const CAL_RD_DMA_INIT_ADDR: c_uint = 0x154;
pub const CAL_RD_DMA_INIT_OFST: c_uint = 0x168;
pub const CAL_RD_DMA_CTRL2: c_uint = 0x16c;

// CAL CSI2 PHY register offsets
pub const CAL_CSI2_PHY_REG0: c_uint = 0x000;
pub const CAL_CSI2_PHY_REG1: c_uint = 0x004;
pub const CAL_CSI2_PHY_REG2: c_uint = 0x008;
pub const CAL_CSI2_PHY_REG10: c_uint = 0x028;
// CAL Control Module Core Camerrx Control register offsets
pub const CM_CTRL_CORE_CAMERRX_CONTROL: c_uint = 0x000;
//
// Field Definition Macros
//

pub const CAL_HL_REVISION_SCHEME_H08: c_int = 1;
pub const CAL_HL_REVISION_SCHEME_LEGACY: c_int = 0;

pub const CAL_HL_HWINFO_NPPI_CONTEXTS_ZERO: c_int = 0;
pub const CAL_HL_HWINFO_NPPI_CONTEXTS_FOUR: c_int = 1;
pub const CAL_HL_HWINFO_NPPI_CONTEXTS_EIGHT: c_int = 2;
pub const CAL_HL_HWINFO_NPPI_CONTEXTS_RESERVED: c_int = 3;

pub const CAL_HL_SYSCONFIG_SOFTRESET_DONE: c_uint = 0x0;
pub const CAL_HL_SYSCONFIG_SOFTRESET_PENDING: c_uint = 0x1;
pub const CAL_HL_SYSCONFIG_SOFTRESET_NOACTION: c_uint = 0x0;
pub const CAL_HL_SYSCONFIG_SOFTRESET_RESET: c_uint = 0x1;

pub const CAL_HL_SYSCONFIG_IDLEMODE_FORCE: c_int = 0;
pub const CAL_HL_SYSCONFIG_IDLEMODE_NO: c_int = 1;
pub const CAL_HL_SYSCONFIG_IDLEMODE_SMART1: c_int = 2;
pub const CAL_HL_SYSCONFIG_IDLEMODE_SMART2: c_int = 3;

pub const CAL_HL_IRQ_EOI_LINE_NUMBER_READ0: c_int = 0;
pub const CAL_HL_IRQ_EOI_LINE_NUMBER_EOI0: c_int = 0;

pub const CAL_PIX_PROC_EXTRACT_B6: c_uint = 0x0;
pub const CAL_PIX_PROC_EXTRACT_B7: c_uint = 0x1;
pub const CAL_PIX_PROC_EXTRACT_B8: c_uint = 0x2;
pub const CAL_PIX_PROC_EXTRACT_B10: c_uint = 0x3;
pub const CAL_PIX_PROC_EXTRACT_B10_MIPI: c_uint = 0x4;
pub const CAL_PIX_PROC_EXTRACT_B12: c_uint = 0x5;
pub const CAL_PIX_PROC_EXTRACT_B12_MIPI: c_uint = 0x6;
pub const CAL_PIX_PROC_EXTRACT_B14: c_uint = 0x7;
pub const CAL_PIX_PROC_EXTRACT_B14_MIPI: c_uint = 0x8;
pub const CAL_PIX_PROC_EXTRACT_B16_BE: c_uint = 0x9;
pub const CAL_PIX_PROC_EXTRACT_B16_LE: c_uint = 0xa;

pub const CAL_PIX_PROC_DPCMD_BYPASS: c_uint = 0x0;
pub const CAL_PIX_PROC_DPCMD_DPCM_10_8_1: c_uint = 0x2;
pub const CAL_PIX_PROC_DPCMD_DPCM_12_8_1: c_uint = 0x8;
pub const CAL_PIX_PROC_DPCMD_DPCM_10_7_1: c_uint = 0x4;
pub const CAL_PIX_PROC_DPCMD_DPCM_10_7_2: c_uint = 0x5;
pub const CAL_PIX_PROC_DPCMD_DPCM_10_6_1: c_uint = 0x6;
pub const CAL_PIX_PROC_DPCMD_DPCM_10_6_2: c_uint = 0x7;
pub const CAL_PIX_PROC_DPCMD_DPCM_12_7_1: c_uint = 0xa;
pub const CAL_PIX_PROC_DPCMD_DPCM_12_6_1: c_uint = 0xc;
pub const CAL_PIX_PROC_DPCMD_DPCM_14_10: c_uint = 0xe;
pub const CAL_PIX_PROC_DPCMD_DPCM_14_8_1: c_uint = 0x10;
pub const CAL_PIX_PROC_DPCMD_DPCM_16_12_1: c_uint = 0x12;
pub const CAL_PIX_PROC_DPCMD_DPCM_16_10_1: c_uint = 0x14;
pub const CAL_PIX_PROC_DPCMD_DPCM_16_8_1: c_uint = 0x16;

pub const CAL_PIX_PROC_DPCME_BYPASS: c_uint = 0x0;
pub const CAL_PIX_PROC_DPCME_DPCM_10_8_1: c_uint = 0x2;
pub const CAL_PIX_PROC_DPCME_DPCM_12_8_1: c_uint = 0x8;
pub const CAL_PIX_PROC_DPCME_DPCM_14_10: c_uint = 0xe;
pub const CAL_PIX_PROC_DPCME_DPCM_14_8_1: c_uint = 0x10;
pub const CAL_PIX_PROC_DPCME_DPCM_16_12_1: c_uint = 0x12;
pub const CAL_PIX_PROC_DPCME_DPCM_16_10_1: c_uint = 0x14;
pub const CAL_PIX_PROC_DPCME_DPCM_16_8_1: c_uint = 0x16;

pub const CAL_PIX_PROC_PACK_B8: c_uint = 0x0;
pub const CAL_PIX_PROC_PACK_B10_MIPI: c_uint = 0x2;
pub const CAL_PIX_PROC_PACK_B12: c_uint = 0x3;
pub const CAL_PIX_PROC_PACK_B12_MIPI: c_uint = 0x4;
pub const CAL_PIX_PROC_PACK_B16: c_uint = 0x5;
pub const CAL_PIX_PROC_PACK_ARGB: c_uint = 0x6;

pub const CAL_CTRL_POSTED_WRITES_NONPOSTED: c_int = 0;
pub const CAL_CTRL_POSTED_WRITES: c_int = 1;

pub const CAL_CTRL_BURSTSIZE_BURST16: c_uint = 0x0;
pub const CAL_CTRL_BURSTSIZE_BURST32: c_uint = 0x1;
pub const CAL_CTRL_BURSTSIZE_BURST64: c_uint = 0x2;
pub const CAL_CTRL_BURSTSIZE_BURST128: c_uint = 0x3;

pub const CAL_CTRL_PWRSCPCLK_AUTO: c_int = 0;
pub const CAL_CTRL_PWRSCPCLK_FORCE: c_int = 1;

pub const CAL_CTRL1_PPI_GROUPING_DISABLED: c_int = 0;
pub const CAL_CTRL1_PPI_GROUPING_RESERVED: c_int = 1;
pub const CAL_CTRL1_PPI_GROUPING_0: c_int = 2;
pub const CAL_CTRL1_PPI_GROUPING_1: c_int = 3;

pub const CAL_CTRL1_INTERLEAVE01_DISABLED: c_int = 0;
pub const CAL_CTRL1_INTERLEAVE01_PIX1: c_int = 1;
pub const CAL_CTRL1_INTERLEAVE01_PIX4: c_int = 2;
pub const CAL_CTRL1_INTERLEAVE01_RESERVED: c_int = 3;

pub const CAL_CTRL1_INTERLEAVE23_DISABLED: c_int = 0;
pub const CAL_CTRL1_INTERLEAVE23_PIX1: c_int = 1;
pub const CAL_CTRL1_INTERLEAVE23_PIX4: c_int = 2;
pub const CAL_CTRL1_INTERLEAVE23_RESERVED: c_int = 3;

pub const CAL_VPORT_CTRL1_WIDTH_ONE: c_int = 0;
pub const CAL_VPORT_CTRL1_WIDTH_TWO: c_int = 1;

pub const CAL_VPORT_CTRL2_FREERUNNING_GATED: c_int = 0;
pub const CAL_VPORT_CTRL2_FREERUNNING_FREE: c_int = 1;

pub const CAL_VPORT_CTRL2_FS_RESETS_NO: c_int = 0;
pub const CAL_VPORT_CTRL2_FS_RESETS_YES: c_int = 1;

pub const CAL_VPORT_CTRL2_FSM_RESET_NOEFFECT: c_int = 0;
pub const CAL_VPORT_CTRL2_FSM_RESET: c_int = 1;

pub const CAL_BYS_CTRL2_DUPLICATEDDATA_NO: c_int = 0;
pub const CAL_BYS_CTRL2_DUPLICATEDDATA_YES: c_int = 1;

pub const CAL_BYS_CTRL2_FREERUNNING_NO: c_int = 0;
pub const CAL_BYS_CTRL2_FREERUNNING_YES: c_int = 1;

pub const CAL_RD_DMA_CTRL_GO_DIS: c_int = 0;
pub const CAL_RD_DMA_CTRL_GO_EN: c_int = 1;
pub const CAL_RD_DMA_CTRL_GO_IDLE: c_int = 0;
pub const CAL_RD_DMA_CTRL_GO_BUSY: c_int = 1;

pub const CAL_RD_DMA_CTRL2_CIRC_MODE_DIS: c_int = 0;
pub const CAL_RD_DMA_CTRL2_CIRC_MODE_ONE: c_int = 1;
pub const CAL_RD_DMA_CTRL2_CIRC_MODE_FOUR: c_int = 2;
pub const CAL_RD_DMA_CTRL2_CIRC_MODE_SIXTEEN: c_int = 3;
pub const CAL_RD_DMA_CTRL2_CIRC_MODE_SIXTYFOUR: c_int = 4;
pub const CAL_RD_DMA_CTRL2_CIRC_MODE_RESERVED: c_int = 5;

pub const CAL_RD_DMA_CTRL2_PATTERN_LINEAR: c_int = 0;
pub const CAL_RD_DMA_CTRL2_PATTERN_YUV420: c_int = 1;
pub const CAL_RD_DMA_CTRL2_PATTERN_RD2SKIP2: c_int = 2;
pub const CAL_RD_DMA_CTRL2_PATTERN_RD2SKIP4: c_int = 3;

pub const CAL_RD_DMA_CTRL2_BYSOUT_LE_WAIT_FREERUNNING: c_int = 0;
pub const CAL_RD_DMA_CTRL2_BYSOUT_LE_WAIT_WAITFORBYSOUT: c_int = 1;

pub const CAL_WR_DMA_CTRL_MODE_DIS: c_int = 0;
pub const CAL_WR_DMA_CTRL_MODE_SHD: c_int = 1;
pub const CAL_WR_DMA_CTRL_MODE_CNT: c_int = 2;
pub const CAL_WR_DMA_CTRL_MODE_CNT_INIT: c_int = 3;
pub const CAL_WR_DMA_CTRL_MODE_CONST: c_int = 4;
pub const CAL_WR_DMA_CTRL_MODE_RESERVED: c_int = 5;

pub const CAL_WR_DMA_CTRL_PATTERN_LINEAR: c_int = 0;
pub const CAL_WR_DMA_CTRL_PATTERN_WR2SKIP2: c_int = 2;
pub const CAL_WR_DMA_CTRL_PATTERN_WR2SKIP4: c_int = 3;
pub const CAL_WR_DMA_CTRL_PATTERN_RESERVED: c_int = 1;

pub const CAL_WR_DMA_CTRL_DTAG_ATT_HDR: c_int = 0;
pub const CAL_WR_DMA_CTRL_DTAG_ATT_DAT: c_int = 1;
pub const CAL_WR_DMA_CTRL_DTAG: c_int = 2;
pub const CAL_WR_DMA_CTRL_DTAG_PIX_HDR: c_int = 3;
pub const CAL_WR_DMA_CTRL_DTAG_PIX_DAT: c_int = 4;
pub const CAL_WR_DMA_CTRL_DTAG_D5: c_int = 5;
pub const CAL_WR_DMA_CTRL_DTAG_D6: c_int = 6;
pub const CAL_WR_DMA_CTRL_DTAG_D7: c_int = 7;

pub const CAL_WR_DMA_OFST_CIRC_MODE_ONE: c_int = 1;
pub const CAL_WR_DMA_OFST_CIRC_MODE_FOUR: c_int = 2;
pub const CAL_WR_DMA_OFST_CIRC_MODE_SIXTYFOUR: c_int = 3;
pub const CAL_WR_DMA_OFST_CIRC_MODE_DISABLED: c_int = 0;

pub const CAL_CSI2_PPI_CTRL_FRAME_IMMEDIATE: c_int = 0;
pub const CAL_CSI2_PPI_CTRL_FRAME: c_int = 1;

pub const CAL_CSI2_COMPLEXIO_CFG_POSITION_5: c_int = 5;
pub const CAL_CSI2_COMPLEXIO_CFG_POSITION_4: c_int = 4;
pub const CAL_CSI2_COMPLEXIO_CFG_POSITION_3: c_int = 3;
pub const CAL_CSI2_COMPLEXIO_CFG_POSITION_2: c_int = 2;
pub const CAL_CSI2_COMPLEXIO_CFG_POSITION_1: c_int = 1;
pub const CAL_CSI2_COMPLEXIO_CFG_POSITION_NOT_USED: c_int = 0;

pub const CAL_CSI2_COMPLEXIO_CFG_POL_PLUSMINUS: c_int = 0;
pub const CAL_CSI2_COMPLEXIO_CFG_POL_MINUSPLUS: c_int = 1;

pub const CAL_CSI2_COMPLEXIO_CFG_PWR_STATUS_STATE_OFF: c_int = 0;
pub const CAL_CSI2_COMPLEXIO_CFG_PWR_STATUS_STATE_ON: c_int = 1;
pub const CAL_CSI2_COMPLEXIO_CFG_PWR_STATUS_STATE_ULP: c_int = 2;

pub const CAL_CSI2_COMPLEXIO_CFG_PWR_CMD_STATE_OFF: c_int = 0;
pub const CAL_CSI2_COMPLEXIO_CFG_PWR_CMD_STATE_ON: c_int = 1;
pub const CAL_CSI2_COMPLEXIO_CFG_PWR_CMD_STATE_ULP: c_int = 2;

pub const CAL_CSI2_COMPLEXIO_CFG_RESET_DONE_RESETCOMPLETED: c_int = 1;
pub const CAL_CSI2_COMPLEXIO_CFG_RESET_DONE_RESETONGOING: c_int = 0;

pub const CAL_CSI2_COMPLEXIO_CFG_RESET_CTRL: c_int = 0;
pub const CAL_CSI2_COMPLEXIO_CFG_RESET_CTRL_OPERATIONAL: c_int = 1;

pub const CAL_CSI2_CTX_DT_DISABLED: c_int = 0;
pub const CAL_CSI2_CTX_DT_ANY: c_int = 1;

pub const CAL_CSI2_CTX_ATT_PIX: c_int = 0;
pub const CAL_CSI2_CTX_ATT: c_int = 1;

pub const CAL_CSI2_CTX_PACK_MODE_LINE: c_int = 0;
pub const CAL_CSI2_CTX_PACK_MODE_FRAME: c_int = 1;

pub const CAL_CSI2_PHY_REG0_HSCLOCKCONFIG_DISABLE: c_int = 1;
pub const CAL_CSI2_PHY_REG0_HSCLOCKCONFIG_ENABLE: c_int = 0;

pub const CAL_CSI2_PHY_REG1_CLOCK_MISS_DETECTOR_STATUS_ERROR: c_int = 1;
pub const CAL_CSI2_PHY_REG1_CLOCK_MISS_DETECTOR_STATUS_SUCCESS: c_int = 0;

