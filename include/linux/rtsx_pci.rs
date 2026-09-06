//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rtsx_pci.h
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
// Driver for Realtek PCI-Express card reader
//
// Copyright(c) 2009-2013 Realtek Semiconductor Corp. All rights reserved.
//
// Author:
// Wei WANG <wei_wang@realsil.com.cn>
//

pub const MAX_RW_REG_CNT: c_int = 1024;
pub const RTSX_HCBAR: c_uint = 0x00;
pub const RTSX_HCBCTLR: c_uint = 0x04;

pub const READ_REG_CMD: c_int = 0;
pub const WRITE_REG_CMD: c_int = 1;
pub const CHECK_REG_CMD: c_int = 2;
pub const RTSX_HDBAR: c_uint = 0x08;
pub const RTSX_SG_INT: c_uint = 0x04;
pub const RTSX_SG_END: c_uint = 0x02;
pub const RTSX_SG_VALID: c_uint = 0x01;
pub const RTSX_SG_NO_OP: c_uint = 0x00;

pub const RTSX_HDBCTLR: c_uint = 0x0C;
pub const SDMA_MODE: c_uint = 0x00;

pub const RTSX_HAIMR: c_uint = 0x10;

pub const HAIMR_READ: c_uint = 0x00;

pub const RTSX_BIPR: c_uint = 0x14;

pub const RTSX_BIER: c_uint = 0x18;

pub const RTSX_DUM_REG: c_uint = 0x1C;
//
// macros for easy use
//

pub const STATE_TRANS_NONE: c_int = 0;
pub const STATE_TRANS_CMD: c_int = 1;
pub const STATE_TRANS_BUF: c_int = 2;
pub const STATE_TRANS_SG: c_int = 3;
pub const TRANS_NOT_READY: c_int = 0;
pub const TRANS_RESULT_OK: c_int = 1;
pub const TRANS_RESULT_FAIL: c_int = 2;
pub const TRANS_NO_DEVICE: c_int = 3;
pub const RTSX_RESV_BUF_LEN: c_int = 4096;
pub const HOST_CMDS_BUF_LEN: c_int = 1024;

pub const MAX_SG_ITEM_LEN: c_uint = 0x80000;
pub const HOST_TO_DEVICE: c_int = 0;
pub const DEVICE_TO_HOST: c_int = 1;
pub const OUTPUT_3V3: c_int = 0;
pub const OUTPUT_1V8: c_int = 1;
pub const RTSX_PHASE_MAX: c_int = 32;
pub const RX_TUNING_CNT: c_int = 3;
pub const MS_CFG: c_uint = 0xFD40;
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
pub const MS_TPC: c_uint = 0xFD41;
pub const MS_TRANS_CFG: c_uint = 0xFD42;
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
pub const MS_TRANSFER: c_uint = 0xFD43;
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
pub const MS_INT_REG: c_uint = 0xFD44;
pub const MS_BYTE_CNT: c_uint = 0xFD45;
pub const MS_SECTOR_CNT_L: c_uint = 0xFD46;
pub const MS_SECTOR_CNT_H: c_uint = 0xFD47;
pub const MS_DBUS_H: c_uint = 0xFD48;
pub const SD_CFG1: c_uint = 0xFDA0;
pub const SD_CLK_DIVIDE_0: c_uint = 0x00;
pub const SD_CLK_DIVIDE_256: c_uint = 0xC0;
pub const SD_CLK_DIVIDE_128: c_uint = 0x80;
pub const SD_BUS_WIDTH_1BIT: c_uint = 0x00;
pub const SD_BUS_WIDTH_4BIT: c_uint = 0x01;
pub const SD_BUS_WIDTH_8BIT: c_uint = 0x02;
pub const SD_ASYNC_FIFO_NOT_RST: c_uint = 0x10;
pub const SD_20_MODE: c_uint = 0x00;
pub const SD_DDR_MODE: c_uint = 0x04;
pub const SD_30_MODE: c_uint = 0x08;
pub const SD_CLK_DIVIDE_MASK: c_uint = 0xC0;
pub const SD_MODE_SELECT_MASK: c_uint = 0x0C;
pub const SD_CFG2: c_uint = 0xFDA1;
pub const SD_CALCULATE_CRC7: c_uint = 0x00;
pub const SD_NO_CALCULATE_CRC7: c_uint = 0x80;
pub const SD_CHECK_CRC16: c_uint = 0x00;
pub const SD_NO_CHECK_CRC16: c_uint = 0x40;
pub const SD_NO_CHECK_WAIT_CRC_TO: c_uint = 0x20;
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
pub const SD_CFG3: c_uint = 0xFDA2;
pub const SD30_CLK_END_EN: c_uint = 0x10;
pub const SD_RSP_80CLK_TIMEOUT_EN: c_uint = 0x01;
pub const SD_STAT1: c_uint = 0xFDA3;
pub const SD_CRC7_ERR: c_uint = 0x80;
pub const SD_CRC16_ERR: c_uint = 0x40;
pub const SD_CRC_WRITE_ERR: c_uint = 0x20;
pub const SD_CRC_WRITE_ERR_MASK: c_uint = 0x1C;
pub const GET_CRC_TIME_OUT: c_uint = 0x02;
pub const SD_TUNING_COMPARE_ERR: c_uint = 0x01;
pub const SD_STAT2: c_uint = 0xFDA4;
pub const SD_RSP_80CLK_TIMEOUT: c_uint = 0x01;
pub const SD_BUS_STAT: c_uint = 0xFDA5;
pub const SD_CLK_TOGGLE_EN: c_uint = 0x80;
pub const SD_CLK_FORCE_STOP: c_uint = 0x40;
pub const SD_DAT3_STATUS: c_uint = 0x10;
pub const SD_DAT2_STATUS: c_uint = 0x08;
pub const SD_DAT1_STATUS: c_uint = 0x04;
pub const SD_DAT0_STATUS: c_uint = 0x02;
pub const SD_CMD_STATUS: c_uint = 0x01;
pub const SD_PAD_CTL: c_uint = 0xFDA6;
pub const SD_IO_USING_1V8: c_uint = 0x80;
pub const SD_IO_USING_3V3: c_uint = 0x7F;
pub const TYPE_A_DRIVING: c_uint = 0x00;
pub const TYPE_B_DRIVING: c_uint = 0x01;
pub const TYPE_C_DRIVING: c_uint = 0x02;
pub const TYPE_D_DRIVING: c_uint = 0x03;
pub const SD_SAMPLE_POINT_CTL: c_uint = 0xFDA7;
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
pub const SD_PUSH_POINT_CTL: c_uint = 0xFDA8;
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
pub const SD_CMD0: c_uint = 0xFDA9;
pub const SD_CMD_START: c_uint = 0x40;
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
pub const SD_CMD_STATE: c_uint = 0xFDB5;
pub const SD_CMD_IDLE: c_uint = 0x80;
pub const SD_DATA_STATE: c_uint = 0xFDB6;
pub const SD_DATA_IDLE: c_uint = 0x80;
pub const REG_SD_STOP_SDCLK_CFG: c_uint = 0xFDB8;
pub const SD30_CLK_STOP_CFG_EN: c_uint = 0x04;
pub const SD30_CLK_STOP_CFG1: c_uint = 0x02;
pub const SD30_CLK_STOP_CFG0: c_uint = 0x01;
pub const REG_PRE_RW_MODE: c_uint = 0xFD70;
pub const EN_INFINITE_MODE: c_uint = 0x01;
pub const REG_CRC_DUMMY_0: c_uint = 0xFD71;

pub const SRCTL: c_uint = 0xFC13;
pub const DCM_DRP_CTL: c_uint = 0xFC23;
pub const DCM_RESET: c_uint = 0x08;
pub const DCM_LOCKED: c_uint = 0x04;
pub const DCM_208M: c_uint = 0x00;
pub const DCM_TX: c_uint = 0x01;
pub const DCM_RX: c_uint = 0x02;
pub const DCM_DRP_TRIG: c_uint = 0xFC24;
pub const DRP_START: c_uint = 0x80;
pub const DRP_DONE: c_uint = 0x40;
pub const DCM_DRP_CFG: c_uint = 0xFC25;
pub const DRP_WRITE: c_uint = 0x80;
pub const DRP_READ: c_uint = 0x00;
pub const DCM_WRITE_ADDRESS_50: c_uint = 0x50;
pub const DCM_WRITE_ADDRESS_51: c_uint = 0x51;
pub const DCM_READ_ADDRESS_00: c_uint = 0x00;
pub const DCM_READ_ADDRESS_51: c_uint = 0x51;
pub const DCM_DRP_WR_DATA_L: c_uint = 0xFC26;
pub const DCM_DRP_WR_DATA_H: c_uint = 0xFC27;
pub const DCM_DRP_RD_DATA_L: c_uint = 0xFC28;
pub const DCM_DRP_RD_DATA_H: c_uint = 0xFC29;
pub const SD_VPCLK0_CTL: c_uint = 0xFC2A;
pub const SD_VPCLK1_CTL: c_uint = 0xFC2B;
pub const PHASE_SELECT_MASK: c_uint = 0x1F;
pub const SD_DCMPS0_CTL: c_uint = 0xFC2C;
pub const SD_DCMPS1_CTL: c_uint = 0xFC2D;

pub const PHASE_CHANGE: c_uint = 0x80;
pub const PHASE_NOT_RESET: c_uint = 0x40;

pub const DCMPS_CHANGE: c_uint = 0x80;
pub const DCMPS_CHANGE_DONE: c_uint = 0x40;
pub const DCMPS_ERROR: c_uint = 0x20;
pub const DCMPS_CURRENT_PHASE: c_uint = 0x1F;
pub const CARD_CLK_SOURCE: c_uint = 0xFC2E;

pub const CARD_PWR_CTL: c_uint = 0xFD50;
pub const PMOS_STRG_MASK: c_uint = 0x10;
pub const PMOS_STRG_800mA: c_uint = 0x10;
pub const PMOS_STRG_400mA: c_uint = 0x00;
pub const SD_POWER_OFF: c_uint = 0x03;
pub const SD_PARTIAL_POWER_ON: c_uint = 0x01;
pub const SD_POWER_ON: c_uint = 0x00;
pub const SD_POWER_MASK: c_uint = 0x03;
pub const MS_POWER_OFF: c_uint = 0x0C;
pub const MS_PARTIAL_POWER_ON: c_uint = 0x04;
pub const MS_POWER_ON: c_uint = 0x00;
pub const MS_POWER_MASK: c_uint = 0x0C;
pub const BPP_POWER_OFF: c_uint = 0x0F;
pub const BPP_POWER_5_PERCENT_ON: c_uint = 0x0E;
pub const BPP_POWER_10_PERCENT_ON: c_uint = 0x0C;
pub const BPP_POWER_15_PERCENT_ON: c_uint = 0x08;
pub const BPP_POWER_ON: c_uint = 0x00;
pub const BPP_POWER_MASK: c_uint = 0x0F;
pub const SD_VCC_PARTIAL_POWER_ON: c_uint = 0x02;
pub const SD_VCC_POWER_ON: c_uint = 0x00;
pub const CARD_CLK_SWITCH: c_uint = 0xFD51;
pub const RTL8411B_PACKAGE_MODE: c_uint = 0xFD51;
pub const CARD_SHARE_MODE: c_uint = 0xFD52;
pub const CARD_SHARE_MASK: c_uint = 0x0F;
pub const CARD_SHARE_MULTI_LUN: c_uint = 0x00;
pub const CARD_SHARE_NORMAL: c_uint = 0x00;
pub const CARD_SHARE_48_SD: c_uint = 0x04;
pub const CARD_SHARE_48_MS: c_uint = 0x08;
pub const CARD_SHARE_BAROSSA_SD: c_uint = 0x01;
pub const CARD_SHARE_BAROSSA_MS: c_uint = 0x02;
pub const CARD_DRIVE_SEL: c_uint = 0xFD53;

pub const GPIO_DRIVE_8mA: c_uint = 0x01;

pub const CARD_STOP: c_uint = 0xFD54;
pub const SPI_STOP: c_uint = 0x01;
pub const XD_STOP: c_uint = 0x02;
pub const SD_STOP: c_uint = 0x04;
pub const MS_STOP: c_uint = 0x08;
pub const SPI_CLR_ERR: c_uint = 0x10;
pub const XD_CLR_ERR: c_uint = 0x20;
pub const SD_CLR_ERR: c_uint = 0x40;
pub const MS_CLR_ERR: c_uint = 0x80;
pub const CARD_OE: c_uint = 0xFD55;
pub const SD_OUTPUT_EN: c_uint = 0x04;
pub const MS_OUTPUT_EN: c_uint = 0x08;
pub const CARD_AUTO_BLINK: c_uint = 0xFD56;
pub const CARD_GPIO_DIR: c_uint = 0xFD57;
pub const CARD_GPIO: c_uint = 0xFD58;
pub const CARD_DATA_SOURCE: c_uint = 0xFD5B;
pub const PINGPONG_BUFFER: c_uint = 0x01;
pub const RING_BUFFER: c_uint = 0x00;
pub const SD30_CLK_DRIVE_SEL: c_uint = 0xFD5A;
pub const DRIVER_TYPE_A: c_uint = 0x05;
pub const DRIVER_TYPE_B: c_uint = 0x03;
pub const DRIVER_TYPE_C: c_uint = 0x02;
pub const DRIVER_TYPE_D: c_uint = 0x01;
pub const CARD_SELECT: c_uint = 0xFD5C;
pub const SD_MOD_SEL: c_int = 2;
pub const MS_MOD_SEL: c_int = 3;
pub const SD30_DRIVE_SEL: c_uint = 0xFD5E;
pub const CFG_DRIVER_TYPE_A: c_uint = 0x02;
pub const CFG_DRIVER_TYPE_B: c_uint = 0x03;
pub const CFG_DRIVER_TYPE_C: c_uint = 0x01;
pub const CFG_DRIVER_TYPE_D: c_uint = 0x00;
pub const SD30_CMD_DRIVE_SEL: c_uint = 0xFD5E;
pub const SD30_DAT_DRIVE_SEL: c_uint = 0xFD5F;
pub const CARD_CLK_EN: c_uint = 0xFD69;
pub const SD_CLK_EN: c_uint = 0x04;
pub const MS_CLK_EN: c_uint = 0x08;
pub const SD40_CLK_EN: c_uint = 0x10;
pub const SDIO_CTRL: c_uint = 0xFD6B;
pub const CD_PAD_CTL: c_uint = 0xFD73;
pub const CD_DISABLE_MASK: c_uint = 0x07;
pub const MS_CD_DISABLE: c_uint = 0x04;
pub const SD_CD_DISABLE: c_uint = 0x02;
pub const XD_CD_DISABLE: c_uint = 0x01;
pub const CD_DISABLE: c_uint = 0x07;
pub const CD_ENABLE: c_uint = 0x00;
pub const MS_CD_EN_ONLY: c_uint = 0x03;
pub const SD_CD_EN_ONLY: c_uint = 0x05;
pub const XD_CD_EN_ONLY: c_uint = 0x06;
pub const FORCE_CD_LOW_MASK: c_uint = 0x38;
pub const FORCE_CD_XD_LOW: c_uint = 0x08;
pub const FORCE_CD_SD_LOW: c_uint = 0x10;
pub const FORCE_CD_MS_LOW: c_uint = 0x20;
pub const CD_AUTO_DISABLE: c_uint = 0x40;
pub const FPDCTL: c_uint = 0xFC00;
pub const SSC_POWER_DOWN: c_uint = 0x01;
pub const SD_OC_POWER_DOWN: c_uint = 0x02;
pub const ALL_POWER_DOWN: c_uint = 0x03;
pub const OC_POWER_DOWN: c_uint = 0x02;
pub const PDINFO: c_uint = 0xFC01;
pub const CLK_CTL: c_uint = 0xFC02;
pub const CHANGE_CLK: c_uint = 0x01;
pub const CLK_LOW_FREQ: c_uint = 0x01;
pub const CLK_DIV: c_uint = 0xFC03;
pub const CLK_DIV_1: c_uint = 0x01;
pub const CLK_DIV_2: c_uint = 0x02;
pub const CLK_DIV_4: c_uint = 0x03;
pub const CLK_DIV_8: c_uint = 0x04;
pub const CLK_SEL: c_uint = 0xFC04;
pub const SSC_DIV_N_0: c_uint = 0xFC0F;
pub const SSC_DIV_N_1: c_uint = 0xFC10;
pub const SSC_CTL1: c_uint = 0xFC11;
pub const SSC_RSTB: c_uint = 0x80;
pub const SSC_8X_EN: c_uint = 0x40;
pub const SSC_FIX_FRAC: c_uint = 0x20;
pub const SSC_SEL_1M: c_uint = 0x00;
pub const SSC_SEL_2M: c_uint = 0x08;
pub const SSC_SEL_4M: c_uint = 0x10;
pub const SSC_SEL_8M: c_uint = 0x18;
pub const SSC_CTL2: c_uint = 0xFC12;
pub const SSC_DEPTH_MASK: c_uint = 0x07;
pub const SSC_DEPTH_DISALBE: c_uint = 0x00;
pub const SSC_DEPTH_4M: c_uint = 0x01;
pub const SSC_DEPTH_2M: c_uint = 0x02;
pub const SSC_DEPTH_1M: c_uint = 0x03;
pub const SSC_DEPTH_500K: c_uint = 0x04;
pub const SSC_DEPTH_250K: c_uint = 0x05;
pub const RCCTL: c_uint = 0xFC14;
pub const FPGA_PULL_CTL: c_uint = 0xFC1D;
pub const OLT_LED_CTL: c_uint = 0xFC1E;
pub const LED_SHINE_MASK: c_uint = 0x08;
pub const LED_SHINE_EN: c_uint = 0x08;
pub const LED_SHINE_DISABLE: c_uint = 0x00;
pub const GPIO_CTL: c_uint = 0xFC1F;
pub const LDO_CTL: c_uint = 0xFC1E;
pub const BPP_ASIC_1V7: c_uint = 0x00;
pub const BPP_ASIC_1V8: c_uint = 0x01;
pub const BPP_ASIC_1V9: c_uint = 0x02;
pub const BPP_ASIC_2V0: c_uint = 0x03;
pub const BPP_ASIC_2V7: c_uint = 0x04;
pub const BPP_ASIC_2V8: c_uint = 0x05;
pub const BPP_ASIC_3V2: c_uint = 0x06;
pub const BPP_ASIC_3V3: c_uint = 0x07;
pub const BPP_REG_TUNED18: c_uint = 0x07;
pub const BPP_TUNED18_SHIFT_8402: c_int = 5;
pub const BPP_TUNED18_SHIFT_8411: c_int = 4;
pub const BPP_PAD_MASK: c_uint = 0x04;
pub const BPP_PAD_3V3: c_uint = 0x04;
pub const BPP_PAD_1V8: c_uint = 0x00;
pub const BPP_LDO_POWB: c_uint = 0x03;
pub const BPP_LDO_ON: c_uint = 0x00;
pub const BPP_LDO_SUSPEND: c_uint = 0x02;
pub const BPP_LDO_OFF: c_uint = 0x03;
pub const EFUSE_CTL: c_uint = 0xFC30;
pub const EFUSE_ADD: c_uint = 0xFC31;
pub const SYS_VER: c_uint = 0xFC32;
pub const EFUSE_DATAL: c_uint = 0xFC34;
pub const EFUSE_DATAH: c_uint = 0xFC35;
pub const CARD_PULL_CTL1: c_uint = 0xFD60;
pub const CARD_PULL_CTL2: c_uint = 0xFD61;
pub const CARD_PULL_CTL3: c_uint = 0xFD62;
pub const CARD_PULL_CTL4: c_uint = 0xFD63;
pub const CARD_PULL_CTL5: c_uint = 0xFD64;
pub const CARD_PULL_CTL6: c_uint = 0xFD65;
// PCI Express Related Registers
pub const IRQEN0: c_uint = 0xFE20;
pub const IRQSTAT0: c_uint = 0xFE21;
pub const DMA_DONE_INT: c_uint = 0x80;
pub const SUSPEND_INT: c_uint = 0x40;
pub const LINK_RDY_INT: c_uint = 0x20;
pub const LINK_DOWN_INT: c_uint = 0x10;
pub const IRQEN1: c_uint = 0xFE22;
pub const IRQSTAT1: c_uint = 0xFE23;
pub const TLPRIEN: c_uint = 0xFE24;
pub const TLPRISTAT: c_uint = 0xFE25;
pub const TLPTIEN: c_uint = 0xFE26;
pub const TLPTISTAT: c_uint = 0xFE27;
pub const DMATC0: c_uint = 0xFE28;
pub const DMATC1: c_uint = 0xFE29;
pub const DMATC2: c_uint = 0xFE2A;
pub const DMATC3: c_uint = 0xFE2B;
pub const DMACTL: c_uint = 0xFE2C;
pub const DMA_RST: c_uint = 0x80;
pub const DMA_BUSY: c_uint = 0x04;
pub const DMA_DIR_TO_CARD: c_uint = 0x00;
pub const DMA_DIR_FROM_CARD: c_uint = 0x02;
pub const DMA_EN: c_uint = 0x01;

pub const DMA_PACK_SIZE_MASK: c_uint = 0x30;
pub const BCTL: c_uint = 0xFE2D;
pub const RBBC0: c_uint = 0xFE2E;
pub const RBBC1: c_uint = 0xFE2F;
pub const RBDAT: c_uint = 0xFE30;
pub const RBCTL: c_uint = 0xFE34;
pub const U_AUTO_DMA_EN_MASK: c_uint = 0x20;
pub const U_AUTO_DMA_DISABLE: c_uint = 0x00;
pub const RB_FLUSH: c_uint = 0x80;
pub const CFGADDR0: c_uint = 0xFE35;
pub const CFGADDR1: c_uint = 0xFE36;
pub const CFGDATA0: c_uint = 0xFE37;
pub const CFGDATA1: c_uint = 0xFE38;
pub const CFGDATA2: c_uint = 0xFE39;
pub const CFGDATA3: c_uint = 0xFE3A;
pub const CFGRWCTL: c_uint = 0xFE3B;
pub const PHYRWCTL: c_uint = 0xFE3C;
pub const PHYDATA0: c_uint = 0xFE3D;
pub const PHYDATA1: c_uint = 0xFE3E;
pub const PHYADDR: c_uint = 0xFE3F;
pub const MSGRXDATA0: c_uint = 0xFE40;
pub const MSGRXDATA1: c_uint = 0xFE41;
pub const MSGRXDATA2: c_uint = 0xFE42;
pub const MSGRXDATA3: c_uint = 0xFE43;
pub const MSGTXDATA0: c_uint = 0xFE44;
pub const MSGTXDATA1: c_uint = 0xFE45;
pub const MSGTXDATA2: c_uint = 0xFE46;
pub const MSGTXDATA3: c_uint = 0xFE47;
pub const MSGTXCTL: c_uint = 0xFE48;
pub const LTR_CTL: c_uint = 0xFE4A;

pub const LTR_TX_EN_0: c_int = 0;

pub const LTR_LATENCY_MODE_HW: c_int = 0;

pub const OBFF_CFG: c_uint = 0xFE4C;
pub const OBFF_EN_MASK: c_uint = 0x03;
pub const OBFF_DISABLE: c_uint = 0x00;
pub const CDRESUMECTL: c_uint = 0xFE52;
pub const CDGW: c_uint = 0xFE53;
pub const WAKE_SEL_CTL: c_uint = 0xFE54;
pub const PCLK_CTL: c_uint = 0xFE55;
pub const PCLK_MODE_SEL: c_uint = 0x20;
pub const PME_FORCE_CTL: c_uint = 0xFE56;
pub const ASPM_FORCE_CTL: c_uint = 0xFE57;
pub const FORCE_ASPM_CTL0: c_uint = 0x10;
pub const FORCE_ASPM_CTL1: c_uint = 0x20;
pub const FORCE_ASPM_VAL_MASK: c_uint = 0x03;
pub const FORCE_ASPM_L1_EN: c_uint = 0x02;
pub const FORCE_ASPM_L0_EN: c_uint = 0x01;
pub const FORCE_ASPM_NO_ASPM: c_uint = 0x00;
pub const PM_CLK_FORCE_CTL: c_uint = 0xFE58;
pub const CLK_PM_EN: c_uint = 0x01;
pub const FUNC_FORCE_CTL: c_uint = 0xFE59;
pub const FUNC_FORCE_UPME_XMT_DBG: c_uint = 0x02;
pub const PERST_GLITCH_WIDTH: c_uint = 0xFE5C;
pub const CHANGE_LINK_STATE: c_uint = 0xFE5B;
pub const RESET_LOAD_REG: c_uint = 0xFE5E;
pub const EFUSE_CONTENT: c_uint = 0xFE5F;
pub const HOST_SLEEP_STATE: c_uint = 0xFE60;
pub const HOST_ENTER_S1: c_int = 1;
pub const HOST_ENTER_S3: c_int = 2;
pub const SDIO_CFG: c_uint = 0xFE70;
pub const PM_EVENT_DEBUG: c_uint = 0xFE71;
pub const PME_DEBUG_0: c_uint = 0x08;
pub const NFTS_TX_CTRL: c_uint = 0xFE72;
pub const PWR_GATE_CTRL: c_uint = 0xFE75;
pub const PWR_GATE_EN: c_uint = 0x01;
pub const LDO3318_PWR_MASK: c_uint = 0x06;
pub const LDO_ON: c_uint = 0x00;
pub const LDO_SUSPEND: c_uint = 0x04;
pub const LDO_OFF: c_uint = 0x06;
pub const PWD_SUSPEND_EN: c_uint = 0xFE76;
pub const LDO_PWR_SEL: c_uint = 0xFE78;
pub const L1SUB_CONFIG1: c_uint = 0xFE8D;
pub const AUX_CLK_ACTIVE_SEL_MASK: c_uint = 0x01;
pub const MAC_CKSW_DONE: c_uint = 0x00;
pub const L1SUB_CONFIG2: c_uint = 0xFE8E;
pub const L1SUB_AUTO_CFG: c_uint = 0x02;
pub const L1SUB_CONFIG3: c_uint = 0xFE8F;

pub const DUMMY_REG_RESET_0: c_uint = 0xFE90;
pub const IC_VERSION_MASK: c_uint = 0x0F;
pub const REG_VREF: c_uint = 0xFE97;
pub const PWD_SUSPND_EN: c_uint = 0x10;
pub const RTS5260_DMA_RST_CTL_0: c_uint = 0xFEBF;
pub const RTS5260_DMA_RST: c_uint = 0x80;
pub const RTS5260_ADMA3_RST: c_uint = 0x40;
pub const AUTOLOAD_CFG_BASE: c_uint = 0xFF00;
pub const RELINK_TIME_MASK: c_uint = 0x01;
pub const PETXCFG: c_uint = 0xFF03;

pub const FORCE_CLKREQ_LOW: c_uint = 0x80;
pub const FORCE_CLKREQ_HIGH: c_uint = 0x00;
pub const PM_CTRL1: c_uint = 0xFF44;
pub const CD_RESUME_EN_MASK: c_uint = 0xF0;
pub const PM_CTRL2: c_uint = 0xFF45;
pub const PM_CTRL3: c_uint = 0xFF46;
pub const SDIO_SEND_PME_EN: c_uint = 0x80;
pub const FORCE_RC_MODE_ON: c_uint = 0x40;
pub const FORCE_RX50_LINK_ON: c_uint = 0x20;
pub const D3_DELINK_MODE_EN: c_uint = 0x10;
pub const USE_PESRTB_CTL_DELINK: c_uint = 0x08;
pub const DELAY_PIN_WAKE: c_uint = 0x04;
pub const RESET_PIN_WAKE: c_uint = 0x02;
pub const PM_WAKE_EN: c_uint = 0x01;
pub const PM_CTRL4: c_uint = 0xFF47;
// FW config info register
pub const RTS5261_FW_CFG_INFO0: c_uint = 0xFF50;

pub const RTS5261_FW_CFG0: c_uint = 0xFF54;

pub const RTS5261_FW_CFG1: c_uint = 0xFF55;

pub const REG_CFG_OOBS_OFF_TIMER: c_uint = 0xFEA6;
pub const REG_CFG_OOBS_ON_TIMER: c_uint = 0xFEA7;
pub const REG_CFG_VCM_ON_TIMER: c_uint = 0xFEA8;
pub const REG_CFG_OOBS_POLLING: c_uint = 0xFEA9;
// Memory mapping
pub const SRAM_BASE: c_uint = 0xE600;
pub const RBUF_BASE: c_uint = 0xF400;
pub const PPBUF_BASE1: c_uint = 0xF800;
pub const PPBUF_BASE2: c_uint = 0xFA00;
pub const IMAGE_FLAG_ADDR0: c_uint = 0xCE80;
pub const IMAGE_FLAG_ADDR1: c_uint = 0xCE81;
pub const RREF_CFG: c_uint = 0xFF6C;
pub const RREF_VBGSEL_MASK: c_uint = 0x38;
pub const RREF_VBGSEL_1V25: c_uint = 0x28;
pub const OOBS_CONFIG: c_uint = 0xFF6E;
pub const OOBS_AUTOK_DIS: c_uint = 0x80;
pub const OOBS_VAL_MASK: c_uint = 0x1F;
pub const LDO_DV18_CFG: c_uint = 0xFF70;
pub const LDO_DV18_SR_MASK: c_uint = 0xC0;
pub const LDO_DV18_SR_DF: c_uint = 0x40;
pub const DV331812_MASK: c_uint = 0x70;
pub const DV331812_33: c_uint = 0x70;
pub const DV331812_17: c_uint = 0x30;
pub const LDO_CONFIG2: c_uint = 0xFF71;
pub const LDO_D3318_MASK: c_uint = 0x07;
pub const LDO_D3318_33V: c_uint = 0x07;
pub const LDO_D3318_18V: c_uint = 0x02;
pub const DV331812_VDD1: c_uint = 0x04;
pub const DV331812_POWERON: c_uint = 0x08;
pub const DV331812_POWEROFF: c_uint = 0x00;
pub const LDO_VCC_CFG0: c_uint = 0xFF72;
pub const LDO_VCC_LMTVTH_MASK: c_uint = 0x30;
pub const LDO_VCC_LMTVTH_2A: c_uint = 0x10;
// RTS5260
pub const RTS5260_DVCC_TUNE_MASK: c_uint = 0x70;
pub const RTS5260_DVCC_33: c_uint = 0x70;
// RTS5261
pub const RTS5261_LDO1_CFG0: c_uint = 0xFF72;

pub const LDO_VCC_CFG1: c_uint = 0xFF73;
pub const LDO_VCC_REF_TUNE_MASK: c_uint = 0x30;
pub const LDO_VCC_REF_1V2: c_uint = 0x20;
pub const LDO_VCC_TUNE_MASK: c_uint = 0x07;
pub const LDO_VCC_1V8: c_uint = 0x04;
pub const LDO_VCC_3V3: c_uint = 0x07;
pub const LDO_VCC_LMT_EN: c_uint = 0x08;
// RTS5260
pub const LDO_POW_SDVDD1_MASK: c_uint = 0x08;
pub const LDO_POW_SDVDD1_ON: c_uint = 0x08;
pub const LDO_POW_SDVDD1_OFF: c_uint = 0x00;
pub const LDO_VIO_CFG: c_uint = 0xFF75;
pub const LDO_VIO_SR_MASK: c_uint = 0xC0;
pub const LDO_VIO_SR_DF: c_uint = 0x40;
pub const LDO_VIO_REF_TUNE_MASK: c_uint = 0x30;
pub const LDO_VIO_REF_1V2: c_uint = 0x20;
pub const LDO_VIO_TUNE_MASK: c_uint = 0x07;
pub const LDO_VIO_1V7: c_uint = 0x03;
pub const LDO_VIO_1V8: c_uint = 0x04;
pub const LDO_VIO_3V3: c_uint = 0x07;
pub const LDO_DV12S_CFG: c_uint = 0xFF76;
pub const LDO_REF12_TUNE_MASK: c_uint = 0x18;
pub const LDO_REF12_TUNE_DF: c_uint = 0x10;
pub const LDO_D12_TUNE_MASK: c_uint = 0x07;
pub const LDO_D12_TUNE_DF: c_uint = 0x04;
pub const LDO_AV12S_CFG: c_uint = 0xFF77;
pub const LDO_AV12S_TUNE_MASK: c_uint = 0x07;
pub const LDO_AV12S_TUNE_DF: c_uint = 0x04;
pub const SD40_LDO_CTL1: c_uint = 0xFE7D;
pub const SD40_VIO_TUNE_MASK: c_uint = 0x70;
pub const SD40_VIO_TUNE_1V7: c_uint = 0x30;
pub const SD_VIO_LDO_1V8: c_uint = 0x40;
pub const SD_VIO_LDO_3V3: c_uint = 0x70;
pub const RTS5264_AUTOLOAD_CFG2: c_uint = 0xFF7D;

pub const RTS5260_AUTOLOAD_CFG4: c_uint = 0xFF7F;
pub const RTS5260_MIMO_DISABLE: c_uint = 0x8A;
// RTS5261

pub const RTS5260_REG_GPIO_CTL0: c_uint = 0xFC1A;
pub const RTS5260_REG_GPIO_MASK: c_uint = 0x01;
pub const RTS5260_REG_GPIO_ON: c_uint = 0x01;
pub const RTS5260_REG_GPIO_OFF: c_uint = 0x00;
pub const PWR_GLOBAL_CTRL: c_uint = 0xF200;
pub const PCIE_L1_2_EN: c_uint = 0x0C;
pub const PCIE_L1_1_EN: c_uint = 0x0A;
pub const PCIE_L1_0_EN: c_uint = 0x09;
pub const PWR_FE_CTL: c_uint = 0xF201;
pub const PCIE_L1_2_PD_FE_EN: c_uint = 0x0C;
pub const PCIE_L1_1_PD_FE_EN: c_uint = 0x0A;
pub const PCIE_L1_0_PD_FE_EN: c_uint = 0x09;
pub const CFG_PCIE_APHY_OFF_0: c_uint = 0xF204;
pub const CFG_PCIE_APHY_OFF_0_DEFAULT: c_uint = 0xBF;
pub const CFG_PCIE_APHY_OFF_1: c_uint = 0xF205;
pub const CFG_PCIE_APHY_OFF_1_DEFAULT: c_uint = 0xFF;
pub const CFG_PCIE_APHY_OFF_2: c_uint = 0xF206;
pub const CFG_PCIE_APHY_OFF_2_DEFAULT: c_uint = 0x01;
pub const CFG_PCIE_APHY_OFF_3: c_uint = 0xF207;
pub const CFG_PCIE_APHY_OFF_3_DEFAULT: c_uint = 0x00;
pub const CFG_L1_0_PCIE_MAC_RET_VALUE: c_uint = 0xF20C;
pub const CFG_L1_0_PCIE_DPHY_RET_VALUE: c_uint = 0xF20E;
pub const CFG_L1_0_SYS_RET_VALUE: c_uint = 0xF210;
pub const CFG_L1_0_CRC_MISC_RET_VALUE: c_uint = 0xF212;
pub const CFG_L1_0_CRC_SD30_RET_VALUE: c_uint = 0xF214;
pub const CFG_L1_0_CRC_SD40_RET_VALUE: c_uint = 0xF216;
pub const CFG_LP_FPWM_VALUE: c_uint = 0xF219;
pub const CFG_LP_FPWM_VALUE_DEFAULT: c_uint = 0x18;
pub const PWC_CDR: c_uint = 0xF253;
pub const PWC_CDR_DEFAULT: c_uint = 0x03;
pub const CFG_L1_0_RET_VALUE_DEFAULT: c_uint = 0x1B;
pub const CFG_L1_0_CRC_MISC_RET_VALUE_DEFAULT: c_uint = 0x0C;
// OCPCTL
pub const SD_DETECT_EN: c_uint = 0x08;
pub const SD_OCP_INT_EN: c_uint = 0x04;
pub const SD_OCP_INT_CLR: c_uint = 0x02;
pub const SD_OC_CLR: c_uint = 0x01;

// OCPSTAT
pub const SD_OCP_DETECT: c_uint = 0x08;
pub const SD_OC_NOW: c_uint = 0x04;
pub const SD_OC_EVER: c_uint = 0x02;

pub const REG_OCPCTL: c_uint = 0xFD6A;
pub const REG_OCPSTAT: c_uint = 0xFD6E;
pub const REG_OCPGLITCH: c_uint = 0xFD6C;
pub const REG_OCPPARA1: c_uint = 0xFD6B;
pub const REG_OCPPARA2: c_uint = 0xFD6D;
// rts5260 DV3318 OCP-related registers
pub const REG_DV3318_OCPCTL: c_uint = 0xFD89;
pub const DV3318_OCP_TIME_MASK: c_uint = 0xF0;
pub const DV3318_DETECT_EN: c_uint = 0x08;
pub const DV3318_OCP_INT_EN: c_uint = 0x04;
pub const DV3318_OCP_INT_CLR: c_uint = 0x02;
pub const DV3318_OCP_CLR: c_uint = 0x01;
pub const REG_DV3318_OCPSTAT: c_uint = 0xFD8A;
pub const DV3318_OCP_GlITCH_TIME_MASK: c_uint = 0xF0;
pub const DV3318_OCP_DETECT: c_uint = 0x08;
pub const DV3318_OCP_NOW: c_uint = 0x04;
pub const DV3318_OCP_EVER: c_uint = 0x02;
pub const SD_OCP_GLITCH_MASK: c_uint = 0x0F;
// OCPPARA1
pub const SDVIO_OCP_TIME_60: c_uint = 0x00;
pub const SDVIO_OCP_TIME_100: c_uint = 0x10;
pub const SDVIO_OCP_TIME_200: c_uint = 0x20;
pub const SDVIO_OCP_TIME_400: c_uint = 0x30;
pub const SDVIO_OCP_TIME_600: c_uint = 0x40;
pub const SDVIO_OCP_TIME_800: c_uint = 0x50;
pub const SDVIO_OCP_TIME_1100: c_uint = 0x60;
pub const SDVIO_OCP_TIME_MASK: c_uint = 0x70;
pub const SD_OCP_TIME_60: c_uint = 0x00;
pub const SD_OCP_TIME_100: c_uint = 0x01;
pub const SD_OCP_TIME_200: c_uint = 0x02;
pub const SD_OCP_TIME_400: c_uint = 0x03;
pub const SD_OCP_TIME_600: c_uint = 0x04;
pub const SD_OCP_TIME_800: c_uint = 0x05;
pub const SD_OCP_TIME_1100: c_uint = 0x06;
pub const SD_OCP_TIME_MASK: c_uint = 0x07;
// OCPPARA2
pub const SDVIO_OCP_THD_190: c_uint = 0x00;
pub const SDVIO_OCP_THD_250: c_uint = 0x10;
pub const SDVIO_OCP_THD_320: c_uint = 0x20;
pub const SDVIO_OCP_THD_380: c_uint = 0x30;
pub const SDVIO_OCP_THD_440: c_uint = 0x40;
pub const SDVIO_OCP_THD_500: c_uint = 0x50;
pub const SDVIO_OCP_THD_570: c_uint = 0x60;
pub const SDVIO_OCP_THD_630: c_uint = 0x70;
pub const SDVIO_OCP_THD_MASK: c_uint = 0x70;
pub const SD_OCP_THD_450: c_uint = 0x00;
pub const SD_OCP_THD_550: c_uint = 0x01;
pub const SD_OCP_THD_650: c_uint = 0x02;
pub const SD_OCP_THD_750: c_uint = 0x03;
pub const SD_OCP_THD_850: c_uint = 0x04;
pub const SD_OCP_THD_950: c_uint = 0x05;
pub const SD_OCP_THD_1050: c_uint = 0x06;
pub const SD_OCP_THD_1150: c_uint = 0x07;
pub const SD_OCP_THD_MASK: c_uint = 0x07;
pub const SDVIO_OCP_GLITCH_MASK: c_uint = 0xF0;
pub const SDVIO_OCP_GLITCH_NONE: c_uint = 0x00;
pub const SDVIO_OCP_GLITCH_50U: c_uint = 0x10;
pub const SDVIO_OCP_GLITCH_100U: c_uint = 0x20;
pub const SDVIO_OCP_GLITCH_200U: c_uint = 0x30;
pub const SDVIO_OCP_GLITCH_600U: c_uint = 0x40;
pub const SDVIO_OCP_GLITCH_800U: c_uint = 0x50;
pub const SDVIO_OCP_GLITCH_1M: c_uint = 0x60;
pub const SDVIO_OCP_GLITCH_2M: c_uint = 0x70;
pub const SDVIO_OCP_GLITCH_3M: c_uint = 0x80;
pub const SDVIO_OCP_GLITCH_4M: c_uint = 0x90;
pub const SDVIO_OCP_GLIVCH_5M: c_uint = 0xA0;
pub const SDVIO_OCP_GLITCH_6M: c_uint = 0xB0;
pub const SDVIO_OCP_GLITCH_7M: c_uint = 0xC0;
pub const SDVIO_OCP_GLITCH_8M: c_uint = 0xD0;
pub const SDVIO_OCP_GLITCH_9M: c_uint = 0xE0;
pub const SDVIO_OCP_GLITCH_10M: c_uint = 0xF0;
pub const SD_OCP_GLITCH_MASK: c_uint = 0x0F;
pub const SD_OCP_GLITCH_NONE: c_uint = 0x00;
pub const SD_OCP_GLITCH_50U: c_uint = 0x01;
pub const SD_OCP_GLITCH_100U: c_uint = 0x02;
pub const SD_OCP_GLITCH_200U: c_uint = 0x03;
pub const SD_OCP_GLITCH_600U: c_uint = 0x04;
pub const SD_OCP_GLITCH_800U: c_uint = 0x05;
pub const SD_OCP_GLITCH_1M: c_uint = 0x06;
pub const SD_OCP_GLITCH_2M: c_uint = 0x07;
pub const SD_OCP_GLITCH_3M: c_uint = 0x08;
pub const SD_OCP_GLITCH_4M: c_uint = 0x09;
pub const SD_OCP_GLIVCH_5M: c_uint = 0x0A;
pub const SD_OCP_GLITCH_6M: c_uint = 0x0B;
pub const SD_OCP_GLITCH_7M: c_uint = 0x0C;
pub const SD_OCP_GLITCH_8M: c_uint = 0x0D;
pub const SD_OCP_GLITCH_9M: c_uint = 0x0E;
pub const SD_OCP_GLITCH_10M: c_uint = 0x0F;
// Phy register
pub const PHY_PCR: c_uint = 0x00;
pub const PHY_PCR_FORCE_CODE: c_uint = 0xB000;
pub const PHY_PCR_OOBS_CALI_50: c_uint = 0x0800;
pub const PHY_PCR_OOBS_VCM_08: c_uint = 0x0200;
pub const PHY_PCR_OOBS_SEN_90: c_uint = 0x0040;
pub const PHY_PCR_RSSI_EN: c_uint = 0x0002;
pub const PHY_PCR_RX10K: c_uint = 0x0001;
pub const PHY_RCR0: c_uint = 0x01;
pub const PHY_RCR1: c_uint = 0x02;
pub const PHY_RCR1_ADP_TIME_4: c_uint = 0x0400;
pub const PHY_RCR1_VCO_COARSE: c_uint = 0x001F;
pub const PHY_RCR1_INIT_27S: c_uint = 0x0A1F;
pub const PHY_SSCCR2: c_uint = 0x02;
pub const PHY_SSCCR2_PLL_NCODE: c_uint = 0x0A00;
pub const PHY_SSCCR2_TIME0: c_uint = 0x001C;
pub const PHY_SSCCR2_TIME2_WIDTH: c_uint = 0x0003;
pub const PHY_RCR2: c_uint = 0x03;
pub const PHY_RCR2_EMPHASE_EN: c_uint = 0x8000;
pub const PHY_RCR2_NADJR: c_uint = 0x4000;
pub const PHY_RCR2_CDR_SR_2: c_uint = 0x0100;
pub const PHY_RCR2_FREQSEL_12: c_uint = 0x0040;
pub const PHY_RCR2_CDR_SC_12P: c_uint = 0x0010;
pub const PHY_RCR2_CALIB_LATE: c_uint = 0x0002;
pub const PHY_RCR2_INIT_27S: c_uint = 0xC152;
pub const PHY_SSCCR3: c_uint = 0x03;
pub const PHY_SSCCR3_STEP_IN: c_uint = 0x2740;
pub const PHY_SSCCR3_CHECK_DELAY: c_uint = 0x0008;
pub const _PHY_ANA03: c_uint = 0x03;
pub const _PHY_ANA03_TIMER_MAX: c_uint = 0x2700;
pub const _PHY_ANA03_OOBS_DEB_EN: c_uint = 0x0040;
pub const _PHY_CMU_DEBUG_EN: c_uint = 0x0008;
pub const PHY_RTCR: c_uint = 0x04;
pub const PHY_RDR: c_uint = 0x05;
pub const PHY_RDR_RXDSEL_1_9: c_uint = 0x4000;
pub const PHY_SSC_AUTO_PWD: c_uint = 0x0600;
pub const PHY_TCR0: c_uint = 0x06;
pub const PHY_TCR1: c_uint = 0x07;
pub const PHY_TUNE: c_uint = 0x08;
pub const PHY_TUNE_TUNEREF_1_0: c_uint = 0x4000;
pub const PHY_TUNE_VBGSEL_1252: c_uint = 0x0C00;
pub const PHY_TUNE_SDBUS_33: c_uint = 0x0200;
pub const PHY_TUNE_TUNED18: c_uint = 0x01C0;

pub const PHY_TUNE_TUNEA12: c_uint = 0x0004;
pub const PHY_TUNE_VOLTAGE_MASK: c_uint = 0xFC3F;
pub const PHY_TUNE_VOLTAGE_3V3: c_uint = 0x03C0;
pub const PHY_TUNE_D18_1V8: c_uint = 0x0100;
pub const PHY_TUNE_D18_1V7: c_uint = 0x0080;
pub const PHY_ANA08: c_uint = 0x08;
pub const PHY_ANA08_RX_EQ_DCGAIN: c_uint = 0x5000;
pub const PHY_ANA08_SEL_RX_EN: c_uint = 0x0400;
pub const PHY_ANA08_RX_EQ_VAL: c_uint = 0x03C0;
pub const PHY_ANA08_SCP: c_uint = 0x0020;
pub const PHY_ANA08_SEL_IPI: c_uint = 0x0004;
pub const PHY_IMR: c_uint = 0x09;
pub const PHY_BPCR: c_uint = 0x0A;
pub const PHY_BPCR_IBRXSEL: c_uint = 0x0400;
pub const PHY_BPCR_IBTXSEL: c_uint = 0x0100;
pub const PHY_BPCR_IB_FILTER: c_uint = 0x0080;
pub const PHY_BPCR_CMIRROR_EN: c_uint = 0x0040;
pub const PHY_BIST: c_uint = 0x0B;
pub const PHY_RAW_L: c_uint = 0x0C;
pub const PHY_RAW_H: c_uint = 0x0D;
pub const PHY_RAW_DATA: c_uint = 0x0E;
pub const PHY_HOST_CLK_CTRL: c_uint = 0x0F;
pub const PHY_DMR: c_uint = 0x10;
pub const PHY_BACR: c_uint = 0x11;
pub const PHY_BACR_BASIC_MASK: c_uint = 0xFFF3;
pub const PHY_IER: c_uint = 0x12;
pub const PHY_BCSR: c_uint = 0x13;
pub const PHY_BPR: c_uint = 0x14;
pub const PHY_BPNR2: c_uint = 0x15;
pub const PHY_BPNR: c_uint = 0x16;
pub const PHY_BRNR2: c_uint = 0x17;
pub const PHY_BENR: c_uint = 0x18;
pub const PHY_REV: c_uint = 0x19;
pub const PHY_REV_RESV: c_uint = 0xE000;
pub const PHY_REV_RXIDLE_LATCHED: c_uint = 0x1000;
pub const PHY_REV_P1_EN: c_uint = 0x0800;
pub const PHY_REV_RXIDLE_EN: c_uint = 0x0400;
pub const PHY_REV_CLKREQ_TX_EN: c_uint = 0x0200;
pub const PHY_REV_CLKREQ_RX_EN: c_uint = 0x0100;
pub const PHY_REV_CLKREQ_DT_1_0: c_uint = 0x0040;
pub const PHY_REV_STOP_CLKRD: c_uint = 0x0020;
pub const PHY_REV_RX_PWST: c_uint = 0x0008;
pub const PHY_REV_STOP_CLKWR: c_uint = 0x0004;
pub const _PHY_REV0: c_uint = 0x19;
pub const _PHY_REV0_FILTER_OUT: c_uint = 0x3800;
pub const _PHY_REV0_CDR_BYPASS_PFD: c_uint = 0x0100;
pub const _PHY_REV0_CDR_RX_IDLE_BYPASS: c_uint = 0x0002;
pub const PHY_FLD0: c_uint = 0x1A;
pub const PHY_ANA1A: c_uint = 0x1A;
pub const PHY_ANA1A_TXR_LOOPBACK: c_uint = 0x2000;
pub const PHY_ANA1A_RXT_BIST: c_uint = 0x0500;
pub const PHY_ANA1A_TXR_BIST: c_uint = 0x0040;
pub const PHY_ANA1A_REV: c_uint = 0x0006;
pub const PHY_FLD0_INIT_27S: c_uint = 0x2546;
pub const PHY_FLD1: c_uint = 0x1B;
pub const PHY_FLD2: c_uint = 0x1C;
pub const PHY_FLD3: c_uint = 0x1D;
pub const PHY_FLD3_TIMER_4: c_uint = 0x0800;
pub const PHY_FLD3_TIMER_6: c_uint = 0x0020;
pub const PHY_FLD3_RXDELINK: c_uint = 0x0004;
pub const PHY_FLD3_INIT_27S: c_uint = 0x0004;
pub const PHY_ANA1D: c_uint = 0x1D;
pub const PHY_ANA1D_DEBUG_ADDR: c_uint = 0x0004;
pub const _PHY_FLD0: c_uint = 0x1D;
pub const _PHY_FLD0_CLK_REQ_20C: c_uint = 0x8000;
pub const _PHY_FLD0_RX_IDLE_EN: c_uint = 0x1000;
pub const _PHY_FLD0_BIT_ERR_RSTN: c_uint = 0x0800;
pub const _PHY_FLD0_BER_COUNT: c_uint = 0x01E0;
pub const _PHY_FLD0_BER_TIMER: c_uint = 0x001E;
pub const _PHY_FLD0_CHECK_EN: c_uint = 0x0001;
pub const PHY_FLD4: c_uint = 0x1E;
pub const PHY_FLD4_FLDEN_SEL: c_uint = 0x4000;
pub const PHY_FLD4_REQ_REF: c_uint = 0x2000;
pub const PHY_FLD4_RXAMP_OFF: c_uint = 0x1000;
pub const PHY_FLD4_REQ_ADDA: c_uint = 0x0800;
pub const PHY_FLD4_BER_COUNT: c_uint = 0x00E0;
pub const PHY_FLD4_BER_TIMER: c_uint = 0x000A;
pub const PHY_FLD4_BER_CHK_EN: c_uint = 0x0001;
pub const PHY_FLD4_INIT_27S: c_uint = 0x5C7F;
pub const PHY_DIG1E: c_uint = 0x1E;
pub const PHY_DIG1E_REV: c_uint = 0x4000;
pub const PHY_DIG1E_D0_X_D1: c_uint = 0x1000;
pub const PHY_DIG1E_RX_ON_HOST: c_uint = 0x0800;
pub const PHY_DIG1E_RCLK_REF_HOST: c_uint = 0x0400;
pub const PHY_DIG1E_RCLK_TX_EN_KEEP: c_uint = 0x0040;
pub const PHY_DIG1E_RCLK_TX_TERM_KEEP: c_uint = 0x0020;
pub const PHY_DIG1E_RCLK_RX_EIDLE_ON: c_uint = 0x0010;
pub const PHY_DIG1E_TX_TERM_KEEP: c_uint = 0x0008;
pub const PHY_DIG1E_RX_TERM_KEEP: c_uint = 0x0004;
pub const PHY_DIG1E_TX_EN_KEEP: c_uint = 0x0002;
pub const PHY_DIG1E_RX_EN_KEEP: c_uint = 0x0001;
pub const PHY_DUM_REG: c_uint = 0x1F;
pub const PCR_SETTING_REG1: c_uint = 0x724;
pub const PCR_SETTING_REG2: c_uint = 0x814;
pub const PCR_SETTING_REG3: c_uint = 0x747;
pub const PCR_SETTING_REG4: c_uint = 0x818;
pub const PCR_SETTING_REG5: c_uint = 0x81C;

pub const RTS5227_DEVICE_ID: c_uint = 0x5227;
pub const RTS_MAX_TIMES_FREQ_REDUCTION: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcr_handle {
    pub pcr: *mut rtsx_pcr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcr_ops {
    pub val): *mut *mut *mut int (write_phy)(struct rtsx_pcr pcr, u8 addr, u16,
    pub val): *mut *mut *mut int (read_phy)(struct rtsx_pcr pcr, u8 addr, u16,
    pub pcr): *mut *mut int (extra_init_hw)(struct rtsx_pcr,
    pub pcr): *mut *mut int (optimize_phy)(struct rtsx_pcr,
    pub pcr): *mut *mut int (turn_on_led)(struct rtsx_pcr,
    pub pcr): *mut *mut int (turn_off_led)(struct rtsx_pcr,
    pub pcr): *mut *mut int (enable_auto_blink)(struct rtsx_pcr,
    pub pcr): *mut *mut int (disable_auto_blink)(struct rtsx_pcr,
    pub card): *mut *mut *mut int (card_power_on)(struct rtsx_pcr pcr, int,
    pub card): *mut *mut *mut int (card_power_off)(struct rtsx_pcr pcr, int,
    pub voltage): u8,
    pub pcr): *mut *mut unsigned int (cd_deglitch)(struct rtsx_pcr,
    pub dir): *mut *mut int (conv_clk_and_div_n)(int clk, int,
    pub pcr): *mut *mut void (fetch_vendor_settings)(struct rtsx_pcr,
    pub runtime): *mut *mut *mut void (force_power_down)(struct rtsx_pcr pcr, u8 pm_state, bool,
    pub pcr): *mut *mut void (stop_cmd)(struct rtsx_pcr,
    pub enable): *mut *mut *mut void (set_aspm)(struct rtsx_pcr pcr, bool,
    pub active): *mut *mut *mut void (set_l1off_cfg_sub_d0)(struct rtsx_pcr pcr, int,
    pub pcr): *mut *mut void (enable_ocp)(struct rtsx_pcr,
    pub pcr): *mut *mut void (disable_ocp)(struct rtsx_pcr,
    pub pcr): *mut *mut void (init_ocp)(struct rtsx_pcr,
    pub pcr): *mut *mut void (process_ocp)(struct rtsx_pcr,
    pub val): *mut *mut *mut int (get_ocpstat)(struct rtsx_pcr pcr, u8,
    pub pcr): *mut *mut void (clear_ocpstat)(struct rtsx_pcr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PDEV_STAT {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ASPM_MODE {

//
// struct rtsx_cr_option  - card reader option
// @dev_flags: device flags
// @force_clkreq_0: force clock request
// @ltr_en: enable ltr mode flag
// @ltr_enabled: ltr mode in configure space flag
// @ltr_active: ltr mode status
// @ltr_active_latency: ltr mode active latency
// @ltr_idle_latency: ltr mode idle latency
// @ltr_l1off_latency: ltr mode l1off latency
// @l1_snooze_delay: l1 snooze delay
// @ltr_l1off_sspwrgate: ltr l1off sspwrgate
// @ltr_l1off_snooze_sspwrgate: ltr l1off snooze sspwrgate
// @ocp_en: enable ocp flag
// @sd_400mA_ocp_thd: 400mA ocp thd
// @sd_800mA_ocp_thd: 800mA ocp thd
//
    struct rtsx_cr_option {
    u32 dev_flags;
    bool force_clkreq_0;
    bool ltr_en;
    bool ltr_enabled;
    bool ltr_active;
    u32 ltr_active_latency;
    u32 ltr_idle_latency;
    u32 ltr_l1off_latency;
    u32 l1_snooze_delay;
    u8 ltr_l1off_sspwrgate;
    u8 ltr_l1off_snooze_sspwrgate;
    bool ocp_en;
    u8 sd_400mA_ocp_thd;
    u8 sd_800mA_ocp_thd;
    u8 sd_cd_reverse_en;
    u8 sd_wp_reverse_en;
}

//
// struct rtsx_hw_param  - card reader hardware param
// @interrupt_en: indicate which interrutp enable
// @ocp_glitch: ocp glitch time
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtsx_hw_param {
    pub interrupt_en: u32,
    pub ocp_glitch: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtsx_pcr {
    pub pci: *mut pci_dev,
    pub id: c_uint,
    pub option: rtsx_cr_option,
    pub hw_param: rtsx_hw_param,
// pci resources
    pub addr: c_ulong,
    pub remap_addr: *mut void __iomem,
    pub irq: c_int,
// host reserved buffer
    pub rtsx_resv_buf: *mut c_void,
    pub rtsx_resv_buf_addr: dma_addr_t,
    pub host_cmds_ptr: *mut c_void,
    pub host_cmds_addr: dma_addr_t,
    pub ci: c_int,
    pub host_sg_tbl_ptr: *mut c_void,
    pub host_sg_tbl_addr: dma_addr_t,
    pub sgi: c_int,
    pub bier: u32,
    pub trans_result: c_char,
    pub card_inserted: c_uint,
    pub card_removed: c_uint,
    pub card_exist: c_uint,
    pub carddet_work: delayed_work,
    pub lock: spinlock_t,
    pub pcr_mutex: mutex,
    pub done: *mut completion,
    pub finish_me: *mut completion,
    pub cur_clock: c_uint,
    pub remove_pci: bool,
    pub msi_en: bool,

    pub extra_caps: u32,
pub const IC_VER_A: c_int = 0;
pub const IC_VER_B: c_int = 1;
pub const IC_VER_C: c_int = 2;
pub const IC_VER_D: c_int = 3;
    pub ic_version: u8,
    pub sd30_drive_sel_1v8: u8,
    pub sd30_drive_sel_3v3: u8,
    pub card_drive_sel: u8,
pub const ASPM_L1_EN: c_uint = 0x02;
    pub aspm_en: u8,
    pub aspm_mode: ASPM_MODE,
    pub aspm_enabled: bool,

    pub flags: u32,
    pub tx_initial_phase: u32,
    pub rx_initial_phase: u32,
    pub sd_pull_ctl_enable_tbl: *const u32,
    pub sd_pull_ctl_disable_tbl: *const u32,
    pub ms_pull_ctl_enable_tbl: *const u32,
    pub ms_pull_ctl_disable_tbl: *const u32,
    pub ops: *const pcr_ops,
    pub state: PDEV_STAT,
    pub reg_pm_ctrl3: u16,
    pub num_slots: c_int,
    pub slots: *mut rtsx_slot,
    pub dma_error_count: u8,
    pub ocp_stat: u8,
    pub ocp_stat2: u8,
    pub ovp_stat: u8,
    pub rtd3_en: u8,
}

pub const PID_524A: c_uint = 0x524A;
pub const PID_5249: c_uint = 0x5249;
pub const PID_5250: c_uint = 0x5250;
pub const PID_525A: c_uint = 0x525A;
pub const PID_5260: c_uint = 0x5260;
pub const PID_5261: c_uint = 0x5261;
pub const PID_5228: c_uint = 0x5228;
pub const PID_5264: c_uint = 0x5264;

extern "C" {
    pub fn rtsx_pci_start_run(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rtsx_pci_write_register(pcr: *mut rtsx_pcr, addr: u16, mask: u8, data: u8) -> c_int;
}
extern "C" {
    pub fn rtsx_pci_read_register(pcr: *mut rtsx_pcr, addr: u16, data: *mut u8) -> c_int;
}
extern "C" {
    pub fn rtsx_pci_write_phy_register(pcr: *mut rtsx_pcr, addr: u8, val: u16) -> c_int;
}
extern "C" {
    pub fn rtsx_pci_read_phy_register(pcr: *mut rtsx_pcr, addr: u8, val: *mut u16) -> c_int;
}
extern "C" {
    pub fn rtsx_pci_stop_cmd(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rtsx_pci_send_cmd_no_wait(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rtsx_pci_send_cmd(pcr: *mut rtsx_pcr, timeout: c_int) -> c_int;
}
extern "C" {
    pub fn rtsx_pci_read_ppbuf(pcr: *mut rtsx_pcr, buf: *mut u8, buf_len: c_int) -> c_int;
}
extern "C" {
    pub fn rtsx_pci_write_ppbuf(pcr: *mut rtsx_pcr, buf: *mut u8, buf_len: c_int) -> c_int;
}
extern "C" {
    pub fn rtsx_pci_card_pull_ctl_enable(pcr: *mut rtsx_pcr, card: c_int) -> c_int;
}
extern "C" {
    pub fn rtsx_pci_card_pull_ctl_disable(pcr: *mut rtsx_pcr, card: c_int) -> c_int;
}
extern "C" {
    pub fn rtsx_pci_card_power_on(pcr: *mut rtsx_pcr, card: c_int) -> c_int;
}
extern "C" {
    pub fn rtsx_pci_card_power_off(pcr: *mut rtsx_pcr, card: c_int) -> c_int;
}
extern "C" {
    pub fn rtsx_pci_card_exclusive_check(pcr: *mut rtsx_pcr, card: c_int) -> c_int;
}
extern "C" {
    pub fn rtsx_pci_switch_output_voltage(pcr: *mut rtsx_pcr, voltage: u8) -> c_int;
}
extern "C" {
    pub fn rtsx_pci_card_exist(pcr: *mut rtsx_pcr) -> c_uint;
}
extern "C" {
    pub fn rtsx_pci_complete_unfinished_transfer(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rtsx_pci_write_phy_register(_arg: pcr, _arg: addr, append: (val & mask) |) -> return;
}
