//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ieee802154/at86rf230.h
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
// AT86RF230/RF231 driver
//
// Copyright (C) 2009-2012 Siemens AG
//
// Written by:
// Dmitry Eremin-Solenikov <dbaryshkov@gmail.com>
// Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
//

pub const SR_TRX_STATUS: c_uint = 0x01, 0x1f, 0;
pub const SR_RESERVED_01_3: c_uint = 0x01, 0x20, 5;
pub const SR_CCA_STATUS: c_uint = 0x01, 0x40, 6;
pub const SR_CCA_DONE: c_uint = 0x01, 0x80, 7;

pub const SR_TRX_CMD: c_uint = 0x02, 0x1f, 0;
pub const SR_TRAC_STATUS: c_uint = 0x02, 0xe0, 5;

pub const SR_CLKM_CTRL: c_uint = 0x03, 0x07, 0;
pub const SR_CLKM_SHA_SEL: c_uint = 0x03, 0x08, 3;
pub const SR_PAD_IO_CLKM: c_uint = 0x03, 0x30, 4;
pub const SR_PAD_IO: c_uint = 0x03, 0xc0, 6;

pub const SR_IRQ_POLARITY: c_uint = 0x04, 0x01, 0;
pub const SR_IRQ_MASK_MODE: c_uint = 0x04, 0x02, 1;
pub const SR_SPI_CMD_MODE: c_uint = 0x04, 0x0c, 2;
pub const SR_RX_BL_CTRL: c_uint = 0x04, 0x10, 4;
pub const SR_TX_AUTO_CRC_ON: c_uint = 0x04, 0x20, 5;
pub const SR_IRQ_2_EXT_EN: c_uint = 0x04, 0x40, 6;
pub const SR_PA_EXT_EN: c_uint = 0x04, 0x80, 7;

pub const SR_TX_PWR_23X: c_uint = 0x05, 0x0f, 0;
pub const SR_PA_LT_230: c_uint = 0x05, 0x30, 4;
pub const SR_PA_BUF_LT_230: c_uint = 0x05, 0xc0, 6;
pub const SR_TX_PWR_212: c_uint = 0x05, 0x1f, 0;
pub const SR_GC_PA_212: c_uint = 0x05, 0x60, 5;
pub const SR_PA_BOOST_LT_212: c_uint = 0x05, 0x80, 7;

pub const SR_RSSI: c_uint = 0x06, 0x1f, 0;
pub const SR_RND_VALUE: c_uint = 0x06, 0x60, 5;
pub const SR_RX_CRC_VALID: c_uint = 0x06, 0x80, 7;

pub const SR_ED_LEVEL: c_uint = 0x07, 0xff, 0;

pub const SR_CHANNEL: c_uint = 0x08, 0x1f, 0;
pub const SR_CCA_MODE: c_uint = 0x08, 0x60, 5;
pub const SR_CCA_REQUEST: c_uint = 0x08, 0x80, 7;

pub const SR_CCA_ED_THRES: c_uint = 0x09, 0x0f, 0;
pub const SR_RESERVED_09_1: c_uint = 0x09, 0xf0, 4;

pub const SR_PDT_THRES: c_uint = 0x0a, 0x0f, 0;
pub const SR_RESERVED_0a_1: c_uint = 0x0a, 0xf0, 4;

pub const SR_SFD_VALUE: c_uint = 0x0b, 0xff, 0;

pub const SR_OQPSK_DATA_RATE: c_uint = 0x0c, 0x03, 0;
pub const SR_SUB_MODE: c_uint = 0x0c, 0x04, 2;
pub const SR_BPSK_QPSK: c_uint = 0x0c, 0x08, 3;
pub const SR_OQPSK_SUB1_RC_EN: c_uint = 0x0c, 0x10, 4;
pub const SR_RESERVED_0c_5: c_uint = 0x0c, 0x60, 5;
pub const SR_RX_SAFE_MODE: c_uint = 0x0c, 0x80, 7;

pub const SR_ANT_CTRL: c_uint = 0x0d, 0x03, 0;
pub const SR_ANT_EXT_SW_EN: c_uint = 0x0d, 0x04, 2;
pub const SR_ANT_DIV_EN: c_uint = 0x0d, 0x08, 3;
pub const SR_RESERVED_0d_2: c_uint = 0x0d, 0x70, 4;
pub const SR_ANT_SEL: c_uint = 0x0d, 0x80, 7;

pub const SR_IRQ_MASK: c_uint = 0x0e, 0xff, 0;

pub const SR_IRQ_0_PLL_LOCK: c_uint = 0x0f, 0x01, 0;
pub const SR_IRQ_1_PLL_UNLOCK: c_uint = 0x0f, 0x02, 1;
pub const SR_IRQ_2_RX_START: c_uint = 0x0f, 0x04, 2;
pub const SR_IRQ_3_TRX_END: c_uint = 0x0f, 0x08, 3;
pub const SR_IRQ_4_CCA_ED_DONE: c_uint = 0x0f, 0x10, 4;
pub const SR_IRQ_5_AMI: c_uint = 0x0f, 0x20, 5;
pub const SR_IRQ_6_TRX_UR: c_uint = 0x0f, 0x40, 6;
pub const SR_IRQ_7_BAT_LOW: c_uint = 0x0f, 0x80, 7;

pub const SR_RESERVED_10_6: c_uint = 0x10, 0x03, 0;
pub const SR_DVDD_OK: c_uint = 0x10, 0x04, 2;
pub const SR_DVREG_EXT: c_uint = 0x10, 0x08, 3;
pub const SR_RESERVED_10_3: c_uint = 0x10, 0x30, 4;
pub const SR_AVDD_OK: c_uint = 0x10, 0x40, 6;
pub const SR_AVREG_EXT: c_uint = 0x10, 0x80, 7;

pub const SR_BATMON_VTH: c_uint = 0x11, 0x0f, 0;
pub const SR_BATMON_HR: c_uint = 0x11, 0x10, 4;
pub const SR_BATMON_OK: c_uint = 0x11, 0x20, 5;
pub const SR_RESERVED_11_1: c_uint = 0x11, 0xc0, 6;

pub const SR_XTAL_TRIM: c_uint = 0x12, 0x0f, 0;
pub const SR_XTAL_MODE: c_uint = 0x12, 0xf0, 4;

pub const SR_RX_PDT_LEVEL: c_uint = 0x15, 0x0f, 0;
pub const SR_RESERVED_15_2: c_uint = 0x15, 0x70, 4;
pub const SR_RX_PDT_DIS: c_uint = 0x15, 0x80, 7;

pub const SR_RESERVED_17_8: c_uint = 0x17, 0x01, 0;
pub const SR_AACK_PROM_MODE: c_uint = 0x17, 0x02, 1;
pub const SR_AACK_ACK_TIME: c_uint = 0x17, 0x04, 2;
pub const SR_RESERVED_17_5: c_uint = 0x17, 0x08, 3;
pub const SR_AACK_UPLD_RES_FT: c_uint = 0x17, 0x10, 4;
pub const SR_AACK_FLTR_RES_FT: c_uint = 0x17, 0x20, 5;
pub const SR_CSMA_LBT_MODE: c_uint = 0x17, 0x40, 6;
pub const SR_RESERVED_17_1: c_uint = 0x17, 0x80, 7;

pub const SR_RESERVED_18_2: c_uint = 0x18, 0x7f, 0;
pub const SR_FTN_START: c_uint = 0x18, 0x80, 7;

pub const SR_RESERVED_1a_2: c_uint = 0x1a, 0x7f, 0;
pub const SR_PLL_CF_START: c_uint = 0x1a, 0x80, 7;

pub const SR_RESERVED_1b_3: c_uint = 0x1b, 0x3f, 0;
pub const SR_RESERVED_1b_2: c_uint = 0x1b, 0x40, 6;
pub const SR_PLL_DCU_START: c_uint = 0x1b, 0x80, 7;

pub const SR_PART_NUM: c_uint = 0x1c, 0xff, 0;

pub const SR_VERSION_NUM: c_uint = 0x1d, 0xff, 0;

pub const SR_MAN_ID_0: c_uint = 0x1e, 0xff, 0;

pub const SR_MAN_ID_1: c_uint = 0x1f, 0xff, 0;

pub const SR_SHORT_ADDR_0: c_uint = 0x20, 0xff, 0;

pub const SR_SHORT_ADDR_1: c_uint = 0x21, 0xff, 0;

pub const SR_PAN_ID_0: c_uint = 0x22, 0xff, 0;

pub const SR_PAN_ID_1: c_uint = 0x23, 0xff, 0;

pub const SR_IEEE_ADDR_0: c_uint = 0x24, 0xff, 0;

pub const SR_IEEE_ADDR_1: c_uint = 0x25, 0xff, 0;

pub const SR_IEEE_ADDR_2: c_uint = 0x26, 0xff, 0;

pub const SR_IEEE_ADDR_3: c_uint = 0x27, 0xff, 0;

pub const SR_IEEE_ADDR_4: c_uint = 0x28, 0xff, 0;

pub const SR_IEEE_ADDR_5: c_uint = 0x29, 0xff, 0;

pub const SR_IEEE_ADDR_6: c_uint = 0x2a, 0xff, 0;

pub const SR_IEEE_ADDR_7: c_uint = 0x2b, 0xff, 0;

pub const SR_SLOTTED_OPERATION: c_uint = 0x2c, 0x01, 0;
pub const SR_MAX_CSMA_RETRIES: c_uint = 0x2c, 0x0e, 1;
pub const SR_MAX_FRAME_RETRIES: c_uint = 0x2c, 0xf0, 4;

pub const SR_CSMA_SEED_0: c_uint = 0x2d, 0xff, 0;

pub const SR_CSMA_SEED_1: c_uint = 0x2e, 0x07, 0;
pub const SR_AACK_I_AM_COORD: c_uint = 0x2e, 0x08, 3;
pub const SR_AACK_DIS_ACK: c_uint = 0x2e, 0x10, 4;
pub const SR_AACK_SET_PD: c_uint = 0x2e, 0x20, 5;
pub const SR_AACK_FVN_MODE: c_uint = 0x2e, 0xc0, 6;

pub const SR_MIN_BE: c_uint = 0x2f, 0x0f, 0;
pub const SR_MAX_BE: c_uint = 0x2f, 0xf0, 4;
pub const CMD_REG: c_uint = 0x80;
pub const CMD_REG_MASK: c_uint = 0x3f;
pub const CMD_WRITE: c_uint = 0x40;
pub const CMD_FB: c_uint = 0x20;

pub const IRQ_ACTIVE_HIGH: c_int = 0;
pub const IRQ_ACTIVE_LOW: c_int = 1;
pub const STATE_P_ON: c_uint = 0x00	/* BUSY */;
pub const STATE_BUSY_RX: c_uint = 0x01;
pub const STATE_BUSY_TX: c_uint = 0x02;
pub const STATE_FORCE_TRX_OFF: c_uint = 0x03;
pub const STATE_FORCE_TX_ON: c_uint = 0x04	/* IDLE */;
// 0x05 */				/* INVALID_PARAMETER
pub const STATE_RX_ON: c_uint = 0x06;
// 0x07 */				/* SUCCESS
pub const STATE_TRX_OFF: c_uint = 0x08;
pub const STATE_TX_ON: c_uint = 0x09;
// 0x0a - 0x0e */			/* 0x0a - UNSUPPORTED_ATTRIBUTE
pub const STATE_SLEEP: c_uint = 0x0F;
pub const STATE_PREP_DEEP_SLEEP: c_uint = 0x10;
pub const STATE_BUSY_RX_AACK: c_uint = 0x11;
pub const STATE_BUSY_TX_ARET: c_uint = 0x12;
pub const STATE_RX_AACK_ON: c_uint = 0x16;
pub const STATE_TX_ARET_ON: c_uint = 0x19;
pub const STATE_RX_ON_NOCLK: c_uint = 0x1C;
pub const STATE_RX_AACK_ON_NOCLK: c_uint = 0x1D;
pub const STATE_BUSY_RX_AACK_NOCLK: c_uint = 0x1E;
pub const STATE_TRANSITION_IN_PROGRESS: c_uint = 0x1F;

pub const TRAC_SUCCESS: c_int = 0;
pub const TRAC_SUCCESS_DATA_PENDING: c_int = 1;
pub const TRAC_SUCCESS_WAIT_FOR_ACK: c_int = 2;
pub const TRAC_CHANNEL_ACCESS_FAILURE: c_int = 3;
pub const TRAC_NO_ACK: c_int = 5;
pub const TRAC_INVALID: c_int = 7;
