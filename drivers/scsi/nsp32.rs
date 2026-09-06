//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/nsp32.h
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
// Workbit NinjaSCSI-32Bi/UDE PCI/CardBus SCSI Host Bus Adapter driver
// Basic data header
//
// #define NSP32_DEBUG 9
//
// VENDOR/DEVICE ID
//
pub const PCI_VENDOR_ID_IODATA: c_uint = 0x10fc;
pub const PCI_VENDOR_ID_WORKBIT: c_uint = 0x1145;
pub const PCI_DEVICE_ID_NINJASCSI_32BI_CBSC_II: c_uint = 0x0005;
pub const PCI_DEVICE_ID_NINJASCSI_32BI_KME: c_uint = 0xf007;
pub const PCI_DEVICE_ID_NINJASCSI_32BI_WBT: c_uint = 0x8007;
pub const PCI_DEVICE_ID_WORKBIT_STANDARD: c_uint = 0xf010;
pub const PCI_DEVICE_ID_WORKBIT_DUALEDGE: c_uint = 0xf011;
pub const PCI_DEVICE_ID_NINJASCSI_32BI_LOGITEC: c_uint = 0xf012;
pub const PCI_DEVICE_ID_NINJASCSI_32BIB_LOGITEC: c_uint = 0xf013;
pub const PCI_DEVICE_ID_NINJASCSI_32UDE_MELCO: c_uint = 0xf015;
pub const PCI_DEVICE_ID_NINJASCSI_32UDE_MELCO_II: c_uint = 0x8009;
//
// MODEL
//
// SCSI Generic Definitions
//
pub const EXTENDED_SDTR_LEN: c_uint = 0x03;
// Little Endian
pub type u32_le = u32;
pub type u16_le = u16;
//
// BASIC Definitions
//

pub const ASSERT: c_int = 1;
pub const NEGATE: c_int = 0;
//
// normal register
//
// Don't access below register with Double Word:
// +00, +04, +08, +0c, +64, +80, +84, +88, +90, +c4, +c8, +cc, +d0.
//
pub const IRQ_CONTROL: c_uint = 0x00	/* BASE+00, W, W */;
pub const IRQ_STATUS: c_uint = 0x00	/* BASE+00, W, R */;

pub const TRANSFER_CONTROL: c_uint = 0x02	/* BASE+02, W, W */;
pub const TRANSFER_STATUS: c_uint = 0x02	/* BASE+02, W, R */;

pub const INDEX_REG: c_uint = 0x04	/* BASE+04, Byte(R/W), Word(R) */;
pub const TIMER_SET: c_uint = 0x06	/* BASE+06, W, R/W */;

pub const DATA_REG_LOW: c_uint = 0x08	/* BASE+08, LowW, R/W */;
pub const DATA_REG_HI: c_uint = 0x0a	/* BASE+0a, Hi-W, R/W */;
pub const FIFO_REST_CNT: c_uint = 0x0c	/* BASE+0c, W, R/W */;

pub const SREQ_SMPL_RATE: c_uint = 0x0f	/* BASE+0f, B, R/W */;

pub const SCSI_BUS_CONTROL: c_uint = 0x10	/* BASE+10, B, R/W */;

pub const CLR_COUNTER: c_uint = 0x12	/* BASE+12, B, W */;

pub const SCSI_BUS_MONITOR: c_uint = 0x12	/* BASE+12, B, R */;

pub const COMMAND_DATA: c_uint = 0x14	/* BASE+14, B, R/W */;
pub const PARITY_CONTROL: c_uint = 0x16	/* BASE+16, B, W */;

pub const PARITY_STATUS: c_uint = 0x16	/* BASE+16, B, R */;
// # define PARITY_CHECK_ENABLE BIT(0)

pub const RESELECT_ID: c_uint = 0x18	/* BASE+18, B, R */;
pub const COMMAND_CONTROL: c_uint = 0x18	/* BASE+18, W, W */;

pub const SET_ARBIT: c_uint = 0x1a	/* BASE+1a, B, W */;

pub const ARBIT_STATUS: c_uint = 0x1a	/* BASE+1a, B, R */;
// # define ARBIT_GO             BIT(0)

pub const SYNC_REG: c_uint = 0x1c	/* BASE+1c, B, R/W */;
pub const ACK_WIDTH: c_uint = 0x1d	/* BASE+1d, B, R/W */;
pub const SCSI_DATA_WITH_ACK: c_uint = 0x20	/* BASE+20, B, R/W */;
pub const SCSI_OUT_LATCH_TARGET_ID: c_uint = 0x22	/* BASE+22, B, W */;
pub const SCSI_DATA_IN: c_uint = 0x22	/* BASE+22, B, R */;
pub const SCAM_CONTROL: c_uint = 0x24	/* BASE+24, B, W */;
pub const SCAM_STATUS: c_uint = 0x24	/* BASE+24, B, R */;

pub const SCAM_DATA: c_uint = 0x26	/* BASE+26, B, R/W */;

pub const SACK_CNT: c_uint = 0x28	/* BASE+28, DW, R/W */;
pub const SREQ_CNT: c_uint = 0x2c	/* BASE+2c, DW, R/W */;
pub const FIFO_DATA_LOW: c_uint = 0x30	/* BASE+30, B/W/DW, R/W */;
pub const FIFO_DATA_HIGH: c_uint = 0x32	/* BASE+32, B/W, R/W */;
pub const BM_START_ADR: c_uint = 0x34	/* BASE+34, DW, R/W */;
pub const BM_CNT: c_uint = 0x38	/* BASE+38, DW, R/W */;

pub const SGT_ADR: c_uint = 0x3c	/* BASE+3c, DW, R/W */;
pub const WAIT_REG: c_uint = 0x40	/* Bi only */;
pub const SCSI_EXECUTE_PHASE: c_uint = 0x40	/* BASE+40, W, R */;

pub const SCSI_CSB_IN: c_uint = 0x42	/* BASE+42, B, R */;
pub const SCSI_MSG_OUT: c_uint = 0x44	/* BASE+44, DW, R/W */;

pub const SEL_TIME_OUT: c_uint = 0x48	/* BASE+48, W, R/W */;
pub const SAVED_SACK_CNT: c_uint = 0x4c	/* BASE+4c, DW, R */;
pub const HTOSDATADELAY: c_uint = 0x50	/* BASE+50, B, R/W */;
pub const STOHDATADELAY: c_uint = 0x54	/* BASE+54, B, R/W */;
pub const ACKSUMCHECKRD: c_uint = 0x58	/* BASE+58, W, R */;
pub const REQSUMCHECKRD: c_uint = 0x5c	/* BASE+5c, W, R */;
//
// indexed register
//
pub const CLOCK_DIV: c_uint = 0x00	/* BASE+08, IDX+00, B, R/W */;

pub const TERM_PWR_CONTROL: c_uint = 0x01	/* BASE+08, IDX+01, B, R/W */;

pub const EXT_PORT_DDR: c_uint = 0x02	/* BASE+08, IDX+02, B, R/W */;
pub const EXT_PORT: c_uint = 0x03	/* BASE+08, IDX+03, B, R/W */;

pub const IRQ_SELECT: c_uint = 0x04	/* BASE+08, IDX+04, W, R/W */;

pub const OLD_SCSI_PHASE: c_uint = 0x05	/* BASE+08, IDX+05, B, R */;

pub const FIFO_FULL_SHLD_COUNT: c_uint = 0x06	/* BASE+08, IDX+06, B, R/W */;
pub const FIFO_EMPTY_SHLD_COUNT: c_uint = 0x07	/* BASE+08, IDX+07, B, R/W */;
pub const EXP_ROM_CONTROL: c_uint = 0x08	/* BASE+08, IDX+08, B, R/W */ /* external ROM control */;

pub const EXP_ROM_ADR: c_uint = 0x09	/* BASE+08, IDX+09, W, R/W */;
pub const EXP_ROM_DATA: c_uint = 0x0a	/* BASE+08, IDX+0a, B, R/W */;
pub const CHIP_MODE: c_uint = 0x0b	/* BASE+08, IDX+0b, B, R   */ /* NinjaSCSI-32Bi only */;

pub const MISC_WR: c_uint = 0x0c	/* BASE+08, IDX+0c, W, R/W */;
pub const MISC_RD: c_uint = 0x0c;

pub const BM_CYCLE: c_uint = 0x0d	/* BASE+08, IDX+0d, B, R/W */;

pub const SREQ_EDGH: c_uint = 0x0e	/* BASE+08, IDX+0e, B, W */;

pub const UP_CNT: c_uint = 0x0f	/* BASE+08, IDX+0f, B, W */;

pub const CFG_CMD_STR: c_uint = 0x10	/* BASE+08, IDX+10, W, R */;
pub const CFG_LATE_CACHE: c_uint = 0x11	/* BASE+08, IDX+11, W, R/W */;
pub const CFG_BASE_ADR_1: c_uint = 0x12	/* BASE+08, IDX+12, W, R */;
pub const CFG_BASE_ADR_2: c_uint = 0x13	/* BASE+08, IDX+13, W, R */;
pub const CFG_INLINE: c_uint = 0x14	/* BASE+08, IDX+14, W, R */;
pub const SERIAL_ROM_CTL: c_uint = 0x15	/* BASE+08, IDX+15, B, R */;

pub const FIFO_HST_POINTER: c_uint = 0x16	/* BASE+08, IDX+16, B, R/W */;
pub const SREQ_DELAY: c_uint = 0x17	/* BASE+08, IDX+17, B, R/W */;
pub const SACK_DELAY: c_uint = 0x18	/* BASE+08, IDX+18, B, R/W */;
pub const SREQ_NOISE_CANCEL: c_uint = 0x19	/* BASE+08, IDX+19, B, R/W */;
pub const SDP_NOISE_CANCEL: c_uint = 0x1a	/* BASE+08, IDX+1a, B, R/W */;
pub const DELAY_TEST: c_uint = 0x1b	/* BASE+08, IDX+1b, B, R/W */;
pub const SD0_NOISE_CANCEL: c_uint = 0x20	/* BASE+08, IDX+20, B, R/W */;
pub const SD1_NOISE_CANCEL: c_uint = 0x21	/* BASE+08, IDX+21, B, R/W */;
pub const SD2_NOISE_CANCEL: c_uint = 0x22	/* BASE+08, IDX+22, B, R/W */;
pub const SD3_NOISE_CANCEL: c_uint = 0x23	/* BASE+08, IDX+23, B, R/W */;
pub const SD4_NOISE_CANCEL: c_uint = 0x24	/* BASE+08, IDX+24, B, R/W */;
pub const SD5_NOISE_CANCEL: c_uint = 0x25	/* BASE+08, IDX+25, B, R/W */;
pub const SD6_NOISE_CANCEL: c_uint = 0x26	/* BASE+08, IDX+26, B, R/W */;
pub const SD7_NOISE_CANCEL: c_uint = 0x27	/* BASE+08, IDX+27, B, R/W */;
//
// Useful Bus Monitor status combinations.
//
pub const BUSMON_BUS_FREE: c_int = 0;

//
// structure for DMA/Scatter Gather list
//

// values must be little endian

// Auto parameter mode memory map.
// All values must be little endian.
//
// host data structure
//
// message in/out buffer
pub const MSGOUTBUF_MAX: c_int = 20;
pub const MSGINBUF_MAX: c_int = 20;
// flag for trans_method

//
// structure for connected LUN dynamic data
//
// Note: Currently tagged queuing is disabled, each nsp32_lunt holds
// one SCSI command and one state.
//

//
// SCSI TARGET/LUN definition
//

pub const MAX_TARGET: c_int = 8;

//
// structure for target device static data
//
// flag for nsp32_target.sync_flag

// syncronous period value for nsp32_target.config_max
pub const FAST5M: c_uint = 0x32;
pub const FAST10M: c_uint = 0x19;
pub const ULTRA20M: c_uint = 0x0c;
// flag for nsp32_target.{sync_offset}, period

pub const SYNC_OFFSET: c_uint = 0xf	/* synchronous transfer max offset */;
// syncreg:

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsp32_cmd_priv {
    pub status: sam_status,
}

extern "C" {
    pub fn scsi_cmd_priv(_arg: cmd) -> return;
}
pub const NSP32_MMIO_OFFSET: c_uint = 0x0800;
// allocated memory region
// target/LUN
// behavior setting parameters
// message buffer
//
// TIME definition
//

// end
