//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rtsx_usb.h
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
// Driver for Realtek RTS5139 USB card reader
//
// Copyright(c) 2009-2013 Realtek Semiconductor Corp. All rights reserved.
//
// Author:
// Roger Tseng <rogerable@realtek.com>
//

// related module names
pub const RTSX_USB_SD_CARD: c_int = 0;
pub const RTSX_USB_MS_CARD: c_int = 1;
// endpoint numbers
pub const EP_BULK_OUT: c_int = 1;
pub const EP_BULK_IN: c_int = 2;
pub const EP_INTR_IN: c_int = 3;
// USB vendor requests
pub const RTSX_USB_REQ_REG_OP: c_uint = 0x00;
pub const RTSX_USB_REQ_POLL: c_uint = 0x02;
// miscellaneous parameters
pub const MIN_DIV_N: c_int = 60;
pub const MAX_DIV_N: c_int = 120;
pub const MAX_PHASE: c_int = 15;
pub const RX_TUNING_CNT: c_int = 3;
pub const QFN24: c_int = 0;
pub const LQFP48: c_int = 1;

// data structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtsx_ucr {
    pub vendor_id: u16,
    pub product_id: u16,
    pub package: c_int,
    pub ic_version: u8,
    pub is_rts5179: bool,
    pub cur_clk: c_uint,
    pub cmd_buf: *mut u8,
    pub cmd_idx: c_uint,
    pub rsp_buf: *mut u8,
    pub pusb_dev: *mut usb_device,
    pub pusb_intf: *mut usb_interface,
    pub current_sg: usb_sg_request,
    pub sg_timer: timer_list,
    pub dev_mutex: mutex,
    pub card_status_cache: u16,
    pub card_status_valid: bool,
}

// buffer size
pub const IOBUF_SIZE: c_int = 1024;
// prototypes of exported functions
extern "C" {
    pub fn rtsx_usb_get_card_status(ucr: *mut rtsx_ucr, status: *mut u16) -> c_int;
}
extern "C" {
    pub fn rtsx_usb_read_register(ucr: *mut rtsx_ucr, addr: u16, data: *mut u8) -> c_int;
}
extern "C" {
    pub fn rtsx_usb_send_cmd(ucr: *mut rtsx_ucr, flag: u8, timeout: c_int) -> c_int;
}
extern "C" {
    pub fn rtsx_usb_get_rsp(ucr: *mut rtsx_ucr, rsp_len: c_int, timeout: c_int) -> c_int;
}
extern "C" {
    pub fn rtsx_usb_read_ppbuf(ucr: *mut rtsx_ucr, buf: *mut u8, buf_len: c_int) -> c_int;
}
extern "C" {
    pub fn rtsx_usb_write_ppbuf(ucr: *mut rtsx_ucr, buf: *mut u8, buf_len: c_int) -> c_int;
}
extern "C" {
    pub fn rtsx_usb_card_exclusive_check(ucr: *mut rtsx_ucr, card: c_int) -> c_int;
}
// card status
pub const SD_CD: c_uint = 0x01;
pub const MS_CD: c_uint = 0x02;
pub const XD_CD: c_uint = 0x04;

pub const SD_WP: c_uint = 0x08;
// OCPCTL
pub const MS_OCP_DETECT_EN: c_uint = 0x08;
pub const MS_OCP_INT_EN: c_uint = 0x04;
pub const MS_OCP_INT_CLR: c_uint = 0x02;
pub const MS_OCP_CLEAR: c_uint = 0x01;
// OCPSTAT
pub const MS_OCP_DETECT: c_uint = 0x80;
pub const MS_OCP_NOW: c_uint = 0x02;
pub const MS_OCP_EVER: c_uint = 0x01;
// reader command field offset & parameters
pub const READ_REG_CMD: c_int = 0;
pub const WRITE_REG_CMD: c_int = 1;
pub const CHECK_REG_CMD: c_int = 2;
pub const PACKET_TYPE: c_int = 4;
pub const CNT_H: c_int = 5;
pub const CNT_L: c_int = 6;
pub const STAGE_FLAG: c_int = 7;
pub const CMD_OFFSET: c_int = 8;
pub const SEQ_WRITE_DATA_OFFSET: c_int = 12;
pub const BATCH_CMD: c_int = 0;
pub const SEQ_READ: c_int = 1;
pub const SEQ_WRITE: c_int = 2;
pub const STAGE_R: c_uint = 0x01;
pub const STAGE_DI: c_uint = 0x02;
pub const STAGE_DO: c_uint = 0x04;
pub const STAGE_MS_STATUS: c_uint = 0x08;
pub const STAGE_XD_STATUS: c_uint = 0x10;
pub const MODE_C: c_uint = 0x00;

pub const EP0_OP_SHIFT: c_int = 14;
pub const EP0_READ_REG_CMD: c_int = 2;
pub const EP0_WRITE_REG_CMD: c_int = 3;

// internal register address
pub const FPDCTL: c_uint = 0xFC00;
pub const SSC_DIV_N_0: c_uint = 0xFC07;
pub const SSC_CTL1: c_uint = 0xFC09;
pub const SSC_CTL2: c_uint = 0xFC0A;
pub const CFG_MODE: c_uint = 0xFC0E;
pub const CFG_MODE_1: c_uint = 0xFC0F;
pub const RCCTL: c_uint = 0xFC14;
pub const SOF_WDOG: c_uint = 0xFC28;
pub const SYS_DUMMY0: c_uint = 0xFC30;
pub const MS_BLKEND: c_uint = 0xFD30;
pub const MS_READ_START: c_uint = 0xFD31;
pub const MS_READ_COUNT: c_uint = 0xFD32;
pub const MS_WRITE_START: c_uint = 0xFD33;
pub const MS_WRITE_COUNT: c_uint = 0xFD34;
pub const MS_COMMAND: c_uint = 0xFD35;
pub const MS_OLD_BLOCK_0: c_uint = 0xFD36;
pub const MS_OLD_BLOCK_1: c_uint = 0xFD37;
pub const MS_NEW_BLOCK_0: c_uint = 0xFD38;
pub const MS_NEW_BLOCK_1: c_uint = 0xFD39;
pub const MS_LOG_BLOCK_0: c_uint = 0xFD3A;
pub const MS_LOG_BLOCK_1: c_uint = 0xFD3B;
pub const MS_BUS_WIDTH: c_uint = 0xFD3C;
pub const MS_PAGE_START: c_uint = 0xFD3D;
pub const MS_PAGE_LENGTH: c_uint = 0xFD3E;
pub const MS_CFG: c_uint = 0xFD40;
pub const MS_TPC: c_uint = 0xFD41;
pub const MS_TRANS_CFG: c_uint = 0xFD42;
pub const MS_TRANSFER: c_uint = 0xFD43;
pub const MS_INT_REG: c_uint = 0xFD44;
pub const MS_BYTE_CNT: c_uint = 0xFD45;
pub const MS_SECTOR_CNT_L: c_uint = 0xFD46;
pub const MS_SECTOR_CNT_H: c_uint = 0xFD47;
pub const MS_DBUS_H: c_uint = 0xFD48;
pub const CARD_DMA1_CTL: c_uint = 0xFD5C;
pub const CARD_PULL_CTL1: c_uint = 0xFD60;
pub const CARD_PULL_CTL2: c_uint = 0xFD61;
pub const CARD_PULL_CTL3: c_uint = 0xFD62;
pub const CARD_PULL_CTL4: c_uint = 0xFD63;
pub const CARD_PULL_CTL5: c_uint = 0xFD64;
pub const CARD_PULL_CTL6: c_uint = 0xFD65;
pub const CARD_EXIST: c_uint = 0xFD6F;
pub const CARD_INT_PEND: c_uint = 0xFD71;
pub const LDO_POWER_CFG: c_uint = 0xFD7B;
pub const SD_CFG1: c_uint = 0xFDA0;
pub const SD_CFG2: c_uint = 0xFDA1;
pub const SD_CFG3: c_uint = 0xFDA2;
pub const SD_STAT1: c_uint = 0xFDA3;
pub const SD_STAT2: c_uint = 0xFDA4;
pub const SD_BUS_STAT: c_uint = 0xFDA5;
pub const SD_PAD_CTL: c_uint = 0xFDA6;
pub const SD_SAMPLE_POINT_CTL: c_uint = 0xFDA7;
pub const SD_PUSH_POINT_CTL: c_uint = 0xFDA8;
pub const SD_CMD0: c_uint = 0xFDA9;
pub const SD_CMD1: c_uint = 0xFDAA;
pub const SD_CMD2: c_uint = 0xFDAB;
pub const SD_CMD3: c_uint = 0xFDAC;
pub const SD_CMD4: c_uint = 0xFDAD;
pub const SD_CMD5: c_uint = 0xFDAE;
pub const SD_BYTE_CNT_L: c_uint = 0xFDAF;
pub const SD_BYTE_CNT_H: c_uint = 0xFDB0;
pub const SD_BLOCK_CNT_L: c_uint = 0xFDB1;
pub const SD_BLOCK_CNT_H: c_uint = 0xFDB2;
pub const SD_TRANSFER: c_uint = 0xFDB3;
pub const SD_CMD_STATE: c_uint = 0xFDB5;
pub const SD_DATA_STATE: c_uint = 0xFDB6;
pub const SD_VPCLK0_CTL: c_uint = 0xFC2A;
pub const SD_VPCLK1_CTL: c_uint = 0xFC2B;
pub const SD_DCMPS0_CTL: c_uint = 0xFC2C;
pub const SD_DCMPS1_CTL: c_uint = 0xFC2D;
pub const CARD_DMA1_CTL: c_uint = 0xFD5C;
pub const HW_VERSION: c_uint = 0xFC01;
pub const SSC_CLK_FPGA_SEL: c_uint = 0xFC02;
pub const CLK_DIV: c_uint = 0xFC03;
pub const SFSM_ED: c_uint = 0xFC04;
pub const CD_DEGLITCH_WIDTH: c_uint = 0xFC20;
pub const CD_DEGLITCH_EN: c_uint = 0xFC21;
pub const AUTO_DELINK_EN: c_uint = 0xFC23;
pub const FPGA_PULL_CTL: c_uint = 0xFC1D;
pub const CARD_CLK_SOURCE: c_uint = 0xFC2E;
pub const CARD_SHARE_MODE: c_uint = 0xFD51;
pub const CARD_DRIVE_SEL: c_uint = 0xFD52;
pub const CARD_STOP: c_uint = 0xFD53;
pub const CARD_OE: c_uint = 0xFD54;
pub const CARD_AUTO_BLINK: c_uint = 0xFD55;
pub const CARD_GPIO: c_uint = 0xFD56;
pub const SD30_DRIVE_SEL: c_uint = 0xFD57;
pub const CARD_DATA_SOURCE: c_uint = 0xFD5D;
pub const CARD_SELECT: c_uint = 0xFD5E;
pub const CARD_CLK_EN: c_uint = 0xFD79;
pub const CARD_PWR_CTL: c_uint = 0xFD7A;
pub const OCPCTL: c_uint = 0xFD80;
pub const OCPPARA1: c_uint = 0xFD81;
pub const OCPPARA2: c_uint = 0xFD82;
pub const OCPSTAT: c_uint = 0xFD83;
pub const HS_USB_STAT: c_uint = 0xFE01;
pub const HS_VCONTROL: c_uint = 0xFE26;
pub const HS_VSTAIN: c_uint = 0xFE27;
pub const HS_VLOADM: c_uint = 0xFE28;
pub const HS_VSTAOUT: c_uint = 0xFE29;
pub const MC_IRQ: c_uint = 0xFF00;
pub const MC_IRQEN: c_uint = 0xFF01;
pub const MC_FIFO_CTL: c_uint = 0xFF02;
pub const MC_FIFO_BC0: c_uint = 0xFF03;
pub const MC_FIFO_BC1: c_uint = 0xFF04;
pub const MC_FIFO_STAT: c_uint = 0xFF05;
pub const MC_FIFO_MODE: c_uint = 0xFF06;
pub const MC_FIFO_RD_PTR0: c_uint = 0xFF07;
pub const MC_FIFO_RD_PTR1: c_uint = 0xFF08;
pub const MC_DMA_CTL: c_uint = 0xFF10;
pub const MC_DMA_TC0: c_uint = 0xFF11;
pub const MC_DMA_TC1: c_uint = 0xFF12;
pub const MC_DMA_TC2: c_uint = 0xFF13;
pub const MC_DMA_TC3: c_uint = 0xFF14;
pub const MC_DMA_RST: c_uint = 0xFF15;
pub const RBUF_SIZE_MASK: c_uint = 0xFBFF;
pub const RBUF_BASE: c_uint = 0xF000;
pub const PPBUF_BASE1: c_uint = 0xF800;
pub const PPBUF_BASE2: c_uint = 0xFA00;
// internal register value macros
pub const POWER_OFF: c_uint = 0x03;
pub const PARTIAL_POWER_ON: c_uint = 0x02;
pub const POWER_ON: c_uint = 0x00;
pub const POWER_MASK: c_uint = 0x03;
pub const LDO3318_PWR_MASK: c_uint = 0x0C;
pub const LDO_ON: c_uint = 0x00;
pub const LDO_SUSPEND: c_uint = 0x08;
pub const LDO_OFF: c_uint = 0x0C;
pub const DV3318_AUTO_PWR_OFF: c_uint = 0x10;
pub const FORCE_LDO_POWERB: c_uint = 0x60;
// LDO_POWER_CFG
pub const TUNE_SD18_MASK: c_uint = 0x1C;
pub const TUNE_SD18_1V7: c_uint = 0x00;

// CLK_DIV
pub const CLK_CHANGE: c_uint = 0x80;
pub const CLK_DIV_1: c_uint = 0x00;
pub const CLK_DIV_2: c_uint = 0x01;
pub const CLK_DIV_4: c_uint = 0x02;
pub const CLK_DIV_8: c_uint = 0x03;
pub const SSC_POWER_MASK: c_uint = 0x01;
pub const SSC_POWER_DOWN: c_uint = 0x01;
pub const SSC_POWER_ON: c_uint = 0x00;
pub const FPGA_VER: c_uint = 0x80;
pub const HW_VER_MASK: c_uint = 0x0F;
pub const EXTEND_DMA1_ASYNC_SIGNAL: c_uint = 0x02;
// CFG_MODE
pub const XTAL_FREE: c_uint = 0x80;
pub const CLK_MODE_MASK: c_uint = 0x03;
pub const CLK_MODE_12M_XTAL: c_uint = 0x00;
pub const CLK_MODE_NON_XTAL: c_uint = 0x01;
pub const CLK_MODE_24M_OSC: c_uint = 0x02;
pub const CLK_MODE_48M_OSC: c_uint = 0x03;
// CFG_MODE_1
pub const RTS5179: c_uint = 0x02;
pub const NYET_EN: c_uint = 0x01;
pub const NYET_MSAK: c_uint = 0x01;
pub const SD30_DRIVE_MASK: c_uint = 0x07;
pub const SD20_DRIVE_MASK: c_uint = 0x03;
pub const DISABLE_SD_CD: c_uint = 0x08;
pub const DISABLE_MS_CD: c_uint = 0x10;
pub const DISABLE_XD_CD: c_uint = 0x20;
pub const SD_CD_DEGLITCH_EN: c_uint = 0x01;
pub const MS_CD_DEGLITCH_EN: c_uint = 0x02;
pub const XD_CD_DEGLITCH_EN: c_uint = 0x04;
pub const CARD_SHARE_LQFP48: c_uint = 0x04;
pub const CARD_SHARE_QFN24: c_uint = 0x00;
pub const CARD_SHARE_LQFP_SEL: c_uint = 0x04;
pub const CARD_SHARE_XD: c_uint = 0x00;
pub const CARD_SHARE_SD: c_uint = 0x01;
pub const CARD_SHARE_MS: c_uint = 0x02;
pub const CARD_SHARE_MASK: c_uint = 0x03;
// SD30_DRIVE_SEL
pub const DRIVER_TYPE_A: c_uint = 0x05;
pub const DRIVER_TYPE_B: c_uint = 0x03;
pub const DRIVER_TYPE_C: c_uint = 0x02;
pub const DRIVER_TYPE_D: c_uint = 0x01;
// SD_BUS_STAT
pub const SD_CLK_TOGGLE_EN: c_uint = 0x80;
pub const SD_CLK_FORCE_STOP: c_uint = 0x40;
pub const SD_DAT3_STATUS: c_uint = 0x10;
pub const SD_DAT2_STATUS: c_uint = 0x08;
pub const SD_DAT1_STATUS: c_uint = 0x04;
pub const SD_DAT0_STATUS: c_uint = 0x02;
pub const SD_CMD_STATUS: c_uint = 0x01;
// SD_PAD_CTL
pub const SD_IO_USING_1V8: c_uint = 0x80;
pub const SD_IO_USING_3V3: c_uint = 0x7F;
pub const TYPE_A_DRIVING: c_uint = 0x00;
pub const TYPE_B_DRIVING: c_uint = 0x01;
pub const TYPE_C_DRIVING: c_uint = 0x02;
pub const TYPE_D_DRIVING: c_uint = 0x03;
// CARD_CLK_EN
pub const SD_CLK_EN: c_uint = 0x04;
pub const MS_CLK_EN: c_uint = 0x08;
// CARD_SELECT
pub const SD_MOD_SEL: c_int = 2;
pub const MS_MOD_SEL: c_int = 3;
// CARD_SHARE_MODE
pub const CARD_SHARE_LQFP48: c_uint = 0x04;
pub const CARD_SHARE_QFN24: c_uint = 0x00;
pub const CARD_SHARE_LQFP_SEL: c_uint = 0x04;
pub const CARD_SHARE_XD: c_uint = 0x00;
pub const CARD_SHARE_SD: c_uint = 0x01;
pub const CARD_SHARE_MS: c_uint = 0x02;
pub const CARD_SHARE_MASK: c_uint = 0x03;
// SSC_CTL1
pub const SSC_RSTB: c_uint = 0x80;
pub const SSC_8X_EN: c_uint = 0x40;
pub const SSC_FIX_FRAC: c_uint = 0x20;
pub const SSC_SEL_1M: c_uint = 0x00;
pub const SSC_SEL_2M: c_uint = 0x08;
pub const SSC_SEL_4M: c_uint = 0x10;
pub const SSC_SEL_8M: c_uint = 0x18;
// SSC_CTL2
pub const SSC_DEPTH_MASK: c_uint = 0x03;
pub const SSC_DEPTH_DISALBE: c_uint = 0x00;
pub const SSC_DEPTH_2M: c_uint = 0x01;
pub const SSC_DEPTH_1M: c_uint = 0x02;
pub const SSC_DEPTH_512K: c_uint = 0x03;
// SD_VPCLK0_CTL
pub const PHASE_CHANGE: c_uint = 0x80;
pub const PHASE_NOT_RESET: c_uint = 0x40;
// SD_TRANSFER
pub const SD_TRANSFER_START: c_uint = 0x80;
pub const SD_TRANSFER_END: c_uint = 0x40;
pub const SD_STAT_IDLE: c_uint = 0x20;
pub const SD_TRANSFER_ERR: c_uint = 0x10;
pub const SD_TM_NORMAL_WRITE: c_uint = 0x00;
pub const SD_TM_AUTO_WRITE_3: c_uint = 0x01;
pub const SD_TM_AUTO_WRITE_4: c_uint = 0x02;
pub const SD_TM_AUTO_READ_3: c_uint = 0x05;
pub const SD_TM_AUTO_READ_4: c_uint = 0x06;
pub const SD_TM_CMD_RSP: c_uint = 0x08;
pub const SD_TM_AUTO_WRITE_1: c_uint = 0x09;
pub const SD_TM_AUTO_WRITE_2: c_uint = 0x0A;
pub const SD_TM_NORMAL_READ: c_uint = 0x0C;
pub const SD_TM_AUTO_READ_1: c_uint = 0x0D;
pub const SD_TM_AUTO_READ_2: c_uint = 0x0E;
pub const SD_TM_AUTO_TUNING: c_uint = 0x0F;
// SD_CFG1
pub const SD_CLK_DIVIDE_0: c_uint = 0x00;
pub const SD_CLK_DIVIDE_256: c_uint = 0xC0;
pub const SD_CLK_DIVIDE_128: c_uint = 0x80;
pub const SD_CLK_DIVIDE_MASK: c_uint = 0xC0;
pub const SD_BUS_WIDTH_1BIT: c_uint = 0x00;
pub const SD_BUS_WIDTH_4BIT: c_uint = 0x01;
pub const SD_BUS_WIDTH_8BIT: c_uint = 0x02;
pub const SD_ASYNC_FIFO_RST: c_uint = 0x10;
pub const SD_20_MODE: c_uint = 0x00;
pub const SD_DDR_MODE: c_uint = 0x04;
pub const SD_30_MODE: c_uint = 0x08;
// SD_CFG2
pub const SD_CALCULATE_CRC7: c_uint = 0x00;
pub const SD_NO_CALCULATE_CRC7: c_uint = 0x80;
pub const SD_CHECK_CRC16: c_uint = 0x00;
pub const SD_NO_CHECK_CRC16: c_uint = 0x40;
pub const SD_WAIT_CRC_TO_EN: c_uint = 0x20;
pub const SD_WAIT_BUSY_END: c_uint = 0x08;
pub const SD_NO_WAIT_BUSY_END: c_uint = 0x00;
pub const SD_CHECK_CRC7: c_uint = 0x00;
pub const SD_NO_CHECK_CRC7: c_uint = 0x04;
pub const SD_RSP_LEN_0: c_uint = 0x00;
pub const SD_RSP_LEN_6: c_uint = 0x01;
pub const SD_RSP_LEN_17: c_uint = 0x02;
pub const SD_RSP_TYPE_R0: c_uint = 0x04;
pub const SD_RSP_TYPE_R1: c_uint = 0x01;
pub const SD_RSP_TYPE_R1b: c_uint = 0x09;
pub const SD_RSP_TYPE_R2: c_uint = 0x02;
pub const SD_RSP_TYPE_R3: c_uint = 0x05;
pub const SD_RSP_TYPE_R4: c_uint = 0x05;
pub const SD_RSP_TYPE_R5: c_uint = 0x01;
pub const SD_RSP_TYPE_R6: c_uint = 0x01;
pub const SD_RSP_TYPE_R7: c_uint = 0x01;
// SD_STAT1
pub const SD_CRC7_ERR: c_uint = 0x80;
pub const SD_CRC16_ERR: c_uint = 0x40;
pub const SD_CRC_WRITE_ERR: c_uint = 0x20;
pub const SD_CRC_WRITE_ERR_MASK: c_uint = 0x1C;
pub const GET_CRC_TIME_OUT: c_uint = 0x02;
pub const SD_TUNING_COMPARE_ERR: c_uint = 0x01;
// SD_DATA_STATE
pub const SD_DATA_IDLE: c_uint = 0x80;
// CARD_DATA_SOURCE
pub const PINGPONG_BUFFER: c_uint = 0x01;
pub const RING_BUFFER: c_uint = 0x00;
// CARD_OE
pub const SD_OUTPUT_EN: c_uint = 0x04;
pub const MS_OUTPUT_EN: c_uint = 0x08;
// CARD_STOP
pub const SD_STOP: c_uint = 0x04;
pub const MS_STOP: c_uint = 0x08;
pub const SD_CLR_ERR: c_uint = 0x40;
pub const MS_CLR_ERR: c_uint = 0x80;
// CARD_CLK_SOURCE

// SD_SAMPLE_POINT_CTL
pub const DDR_FIX_RX_DAT: c_uint = 0x00;
pub const DDR_VAR_RX_DAT: c_uint = 0x80;
pub const DDR_FIX_RX_DAT_EDGE: c_uint = 0x00;
pub const DDR_FIX_RX_DAT_14_DELAY: c_uint = 0x40;
pub const DDR_FIX_RX_CMD: c_uint = 0x00;
pub const DDR_VAR_RX_CMD: c_uint = 0x20;
pub const DDR_FIX_RX_CMD_POS_EDGE: c_uint = 0x00;
pub const DDR_FIX_RX_CMD_14_DELAY: c_uint = 0x10;
pub const SD20_RX_POS_EDGE: c_uint = 0x00;
pub const SD20_RX_14_DELAY: c_uint = 0x08;
pub const SD20_RX_SEL_MASK: c_uint = 0x08;
// SD_PUSH_POINT_CTL
pub const DDR_FIX_TX_CMD_DAT: c_uint = 0x00;
pub const DDR_VAR_TX_CMD_DAT: c_uint = 0x80;
pub const DDR_FIX_TX_DAT_14_TSU: c_uint = 0x00;
pub const DDR_FIX_TX_DAT_12_TSU: c_uint = 0x40;
pub const DDR_FIX_TX_CMD_NEG_EDGE: c_uint = 0x00;
pub const DDR_FIX_TX_CMD_14_AHEAD: c_uint = 0x20;
pub const SD20_TX_NEG_EDGE: c_uint = 0x00;
pub const SD20_TX_14_AHEAD: c_uint = 0x10;
pub const SD20_TX_SEL_MASK: c_uint = 0x10;
pub const DDR_VAR_SDCLK_POL_SWAP: c_uint = 0x01;
// MS_CFG
pub const SAMPLE_TIME_RISING: c_uint = 0x00;
pub const SAMPLE_TIME_FALLING: c_uint = 0x80;
pub const PUSH_TIME_DEFAULT: c_uint = 0x00;
pub const PUSH_TIME_ODD: c_uint = 0x40;
pub const NO_EXTEND_TOGGLE: c_uint = 0x00;
pub const EXTEND_TOGGLE_CHK: c_uint = 0x20;
pub const MS_BUS_WIDTH_1: c_uint = 0x00;
pub const MS_BUS_WIDTH_4: c_uint = 0x10;
pub const MS_BUS_WIDTH_8: c_uint = 0x18;
pub const MS_2K_SECTOR_MODE: c_uint = 0x04;
pub const MS_512_SECTOR_MODE: c_uint = 0x00;
pub const MS_TOGGLE_TIMEOUT_EN: c_uint = 0x00;
pub const MS_TOGGLE_TIMEOUT_DISEN: c_uint = 0x01;
pub const MS_NO_CHECK_INT: c_uint = 0x02;
// MS_TRANS_CFG
pub const WAIT_INT: c_uint = 0x80;
pub const NO_WAIT_INT: c_uint = 0x00;
pub const NO_AUTO_READ_INT_REG: c_uint = 0x00;
pub const AUTO_READ_INT_REG: c_uint = 0x40;
pub const MS_CRC16_ERR: c_uint = 0x20;
pub const MS_RDY_TIMEOUT: c_uint = 0x10;
pub const MS_INT_CMDNK: c_uint = 0x08;
pub const MS_INT_BREQ: c_uint = 0x04;
pub const MS_INT_ERR: c_uint = 0x02;
pub const MS_INT_CED: c_uint = 0x01;
// MS_TRANSFER
pub const MS_TRANSFER_START: c_uint = 0x80;
pub const MS_TRANSFER_END: c_uint = 0x40;
pub const MS_TRANSFER_ERR: c_uint = 0x20;
pub const MS_BS_STATE: c_uint = 0x10;
pub const MS_TM_READ_BYTES: c_uint = 0x00;
pub const MS_TM_NORMAL_READ: c_uint = 0x01;
pub const MS_TM_WRITE_BYTES: c_uint = 0x04;
pub const MS_TM_NORMAL_WRITE: c_uint = 0x05;
pub const MS_TM_AUTO_READ: c_uint = 0x08;
pub const MS_TM_AUTO_WRITE: c_uint = 0x0C;
pub const MS_TM_SET_CMD: c_uint = 0x06;
pub const MS_TM_COPY_PAGE: c_uint = 0x07;
pub const MS_TM_MULTI_READ: c_uint = 0x02;
pub const MS_TM_MULTI_WRITE: c_uint = 0x03;
// MC_FIFO_CTL
pub const FIFO_FLUSH: c_uint = 0x01;
// MC_DMA_RST
pub const DMA_RESET: c_uint = 0x01;
// MC_DMA_CTL
pub const DMA_TC_EQ_0: c_uint = 0x80;
pub const DMA_DIR_TO_CARD: c_uint = 0x00;
pub const DMA_DIR_FROM_CARD: c_uint = 0x02;
pub const DMA_EN: c_uint = 0x01;

pub const DMA_PACK_SIZE_MASK: c_uint = 0x0C;
// CARD_INT_PEND
pub const XD_INT: c_uint = 0x10;
pub const MS_INT: c_uint = 0x08;
pub const SD_INT: c_uint = 0x04;
// LED operations
extern "C" {
    pub fn rtsx_usb_ep0_write_register(_arg: ucr, _arg: CARD_GPIO, _arg: 0x03, _arg: 0x02) -> return;
}
extern "C" {
    pub fn rtsx_usb_ep0_write_register(_arg: ucr, _arg: CARD_GPIO, _arg: 0x03, _arg: 0x03) -> return;
}
// HW error clearing
