//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/wd33c93.h
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
// wd33c93.h -  Linux device driver definitions for the
// Commodore Amiga A2091/590 SCSI controller card
//
// IMPORTANT: This file is for version 1.25 - 09/Jul/1997
//
// Copyright (c) 1996 John Shifflett, GeoLog Consulting
// john@geolog.com
// jshiffle@netcom.com
//

// Macro flag: #define DB(f,a)

// wd register names
pub const WD_OWN_ID: c_uint = 0x00;
pub const WD_CONTROL: c_uint = 0x01;
pub const WD_TIMEOUT_PERIOD: c_uint = 0x02;
pub const WD_CDB_1: c_uint = 0x03;
pub const WD_CDB_2: c_uint = 0x04;
pub const WD_CDB_3: c_uint = 0x05;
pub const WD_CDB_4: c_uint = 0x06;
pub const WD_CDB_5: c_uint = 0x07;
pub const WD_CDB_6: c_uint = 0x08;
pub const WD_CDB_7: c_uint = 0x09;
pub const WD_CDB_8: c_uint = 0x0a;
pub const WD_CDB_9: c_uint = 0x0b;
pub const WD_CDB_10: c_uint = 0x0c;
pub const WD_CDB_11: c_uint = 0x0d;
pub const WD_CDB_12: c_uint = 0x0e;
pub const WD_TARGET_LUN: c_uint = 0x0f;
pub const WD_COMMAND_PHASE: c_uint = 0x10;
pub const WD_SYNCHRONOUS_TRANSFER: c_uint = 0x11;
pub const WD_TRANSFER_COUNT_MSB: c_uint = 0x12;
pub const WD_TRANSFER_COUNT: c_uint = 0x13;
pub const WD_TRANSFER_COUNT_LSB: c_uint = 0x14;
pub const WD_DESTINATION_ID: c_uint = 0x15;
pub const WD_SOURCE_ID: c_uint = 0x16;
pub const WD_SCSI_STATUS: c_uint = 0x17;
pub const WD_COMMAND: c_uint = 0x18;
pub const WD_DATA: c_uint = 0x19;
pub const WD_QUEUE_TAG: c_uint = 0x1a;
pub const WD_AUXILIARY_STATUS: c_uint = 0x1f;
// WD commands
pub const WD_CMD_RESET: c_uint = 0x00;
pub const WD_CMD_ABORT: c_uint = 0x01;
pub const WD_CMD_ASSERT_ATN: c_uint = 0x02;
pub const WD_CMD_NEGATE_ACK: c_uint = 0x03;
pub const WD_CMD_DISCONNECT: c_uint = 0x04;
pub const WD_CMD_RESELECT: c_uint = 0x05;
pub const WD_CMD_SEL_ATN: c_uint = 0x06;
pub const WD_CMD_SEL: c_uint = 0x07;
pub const WD_CMD_SEL_ATN_XFER: c_uint = 0x08;
pub const WD_CMD_SEL_XFER: c_uint = 0x09;
pub const WD_CMD_RESEL_RECEIVE: c_uint = 0x0a;
pub const WD_CMD_RESEL_SEND: c_uint = 0x0b;
pub const WD_CMD_WAIT_SEL_RECEIVE: c_uint = 0x0c;
pub const WD_CMD_TRANS_ADDR: c_uint = 0x18;
pub const WD_CMD_TRANS_INFO: c_uint = 0x20;
pub const WD_CMD_TRANSFER_PAD: c_uint = 0x21;
pub const WD_CMD_SBT_MODE: c_uint = 0x80;
// ASR register

// SCSI Bus Phases
pub const PHS_DATA_OUT: c_uint = 0x00;
pub const PHS_DATA_IN: c_uint = 0x01;
pub const PHS_COMMAND: c_uint = 0x02;
pub const PHS_STATUS: c_uint = 0x03;
pub const PHS_MESS_OUT: c_uint = 0x06;
pub const PHS_MESS_IN: c_uint = 0x07;
// Command Status Register definitions
// reset state interrupts
pub const CSR_RESET: c_uint = 0x00;
pub const CSR_RESET_AF: c_uint = 0x01;
// successful completion interrupts
pub const CSR_RESELECT: c_uint = 0x10;
pub const CSR_SELECT: c_uint = 0x11;
pub const CSR_SEL_XFER_DONE: c_uint = 0x16;
pub const CSR_XFER_DONE: c_uint = 0x18;
// paused or aborted interrupts
pub const CSR_MSGIN: c_uint = 0x20;
pub const CSR_SDP: c_uint = 0x21;
pub const CSR_SEL_ABORT: c_uint = 0x22;
pub const CSR_RESEL_ABORT: c_uint = 0x25;
pub const CSR_RESEL_ABORT_AM: c_uint = 0x27;
pub const CSR_ABORT: c_uint = 0x28;
// terminated interrupts
pub const CSR_INVALID: c_uint = 0x40;
pub const CSR_UNEXP_DISC: c_uint = 0x41;
pub const CSR_TIMEOUT: c_uint = 0x42;
pub const CSR_PARITY: c_uint = 0x43;
pub const CSR_PARITY_ATN: c_uint = 0x44;
pub const CSR_BAD_STATUS: c_uint = 0x45;
pub const CSR_UNEXP: c_uint = 0x48;
// service required interrupts
pub const CSR_RESEL: c_uint = 0x80;
pub const CSR_RESEL_AM: c_uint = 0x81;
pub const CSR_DISC: c_uint = 0x85;
pub const CSR_SRV_REQ: c_uint = 0x88;
// Own ID/CDB Size register
pub const OWNID_EAF: c_uint = 0x08;
pub const OWNID_EHP: c_uint = 0x10;
pub const OWNID_RAF: c_uint = 0x20;
pub const OWNID_FS_8: c_uint = 0x00;
pub const OWNID_FS_12: c_uint = 0x40;
pub const OWNID_FS_16: c_uint = 0x80;
// define these so we don't have to change a2091.c, etc.

// pass input-clock explicitly. accepted mhz values are 8-10,12-20

// Control register
pub const CTRL_HSP: c_uint = 0x01;
pub const CTRL_HA: c_uint = 0x02;
pub const CTRL_IDI: c_uint = 0x04;
pub const CTRL_EDI: c_uint = 0x08;
pub const CTRL_HHP: c_uint = 0x10;
pub const CTRL_POLLED: c_uint = 0x00;
pub const CTRL_BURST: c_uint = 0x20;
pub const CTRL_BUS: c_uint = 0x40;
pub const CTRL_DMA: c_uint = 0x80;
// Timeout Period register

// Synchronous Transfer Register
pub const STR_FSS: c_uint = 0x80;
// Destination ID register
pub const DSTID_DPD: c_uint = 0x40;
pub const DATA_OUT_DIR: c_int = 0;
pub const DATA_IN_DIR: c_int = 1;
pub const DSTID_SCC: c_uint = 0x80;
// Source ID register
pub const SRCID_MASK: c_uint = 0x07;
pub const SRCID_SIV: c_uint = 0x08;
pub const SRCID_DSP: c_uint = 0x20;
pub const SRCID_ES: c_uint = 0x40;
pub const SRCID_ER: c_uint = 0x80;
// This is what the 3393 chip looks like to us
extern "C" {
    pub fn int(SCpnt: *mut *mut dma_setup_t) (struct scsi_cmnd, dir_in: c_int) -> typedef;
}
pub const ILLEGAL_STATUS_BYTE: c_uint = 0xff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sx_period {
    pub period_ns: c_uint,
    pub reg_value: c_uchar,
}

// FEF: defines for hostdata->dma_buffer_pool
pub const BUF_CHIP_ALLOCED: c_int = 0;
pub const BUF_SCSI_ALLOCED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WD33C93_hostdata {
    pub next: *mut Scsi_Host,
    pub regs: wd33c93_regs,
    pub lock: spinlock_t,
    pub clock_freq: c_uchar,
    pub /: *mut *mut uchar chip; / what kind of wd33c93?,
    pub /: *mut *mut uchar microcode; / microcode rev,
    pub /: *mut *mut uchar dma_buffer_pool; / FEF: buffer from chip_ram?,
    pub /: *mut *mut int dma_dir; / data transfer dir.,
    pub dma_setup: dma_setup_t,
    pub dma_stop: dma_stop_t,
    pub dma_xfer_mask: c_uint,
    pub dma_bounce_buffer: *mut c_uchar,
    pub dma_bounce_len: c_uint,
    pub /: *mut *mut volatile uchar busy[8]; / index = target, bit = lun,
    pub /: *mut *mut *mut volatile struct scsi_cmnd input_Q; / commands waiting to be started,
    pub /: *mut *mut *mut volatile struct scsi_cmnd selecting; / trying to select this command,
    pub /: *mut *mut *mut volatile struct scsi_cmnd connected; / currently connected command,
    pub /: *mut *mut *mut volatile struct scsi_cmnd disconnected_Q;/ commands waiting for reconnect,
    pub /: *mut *mut uchar state; / what we are currently doing,
    pub /: *mut *mut uchar dma; / current state of DMA (on/off),
    pub /: *mut *mut uchar level2; / extent to which Level-2 commands are used,
    pub /: *mut *mut uchar disconnect; / disconnect/reselect policy,
    pub /: *mut *mut unsigned int args; / set from command-line argument,
    pub /: *mut *mut uchar incoming_msg[8]; / filled during message_in phase,
    pub /: *mut *mut int incoming_ptr; / mainly used with EXTENDED messages,
    pub /: *mut *mut uchar outgoing_msg[8]; / send this during next message_out,
    pub /: *mut *mut int outgoing_len; / length of outgoing message,
    pub /: *mut *mut unsigned int default_sx_per; / default transfer period for SCSI bus,
    pub /: *mut *mut uchar sync_xfer[8]; / sync_xfer reg settings per target,
    pub /: *mut *mut uchar sync_stat[8]; / status of sync negotiation per target,
    pub /: *mut *mut uchar no_sync; / bitmask: don't do sync on these targets,
    pub /: *mut *mut uchar no_dma; / set this flag to disable DMA,
    pub /: *mut *mut uchar dma_mode; / DMA Burst Mode or Single Byte DMA,
    pub /: *mut *mut uchar fast; / set this flag to enable Fast SCSI,
    pub /: *mut *mut sx_period sx_table[9]; / transfer periods for actual DTC-setting,

    pub /: *mut *mut uchar proc; / bitmask: what's in proc output,

    pub /: *mut *mut unsigned long cmd_cnt[8]; / # of commands issued per target,
    pub /: *mut *mut unsigned long int_cnt; / # of interrupts serviced,
    pub /: *mut *mut unsigned long pio_cnt; / # of pio data transfers,
    pub /: *mut *mut unsigned long dma_cnt; / # of DMA data transfers,
    pub /: *mut *mut unsigned long disc_allowed_cnt[8]; / # of disconnects allowed per target,
    pub target*/: *mut *mut unsigned long disc_done_cnt[8]; / # of disconnects done per,

}

extern "C" {
    pub fn scsi_cmd_priv(_arg: cmd) -> return;
}
// defines for hostdata->chip
pub const C_WD33C93: c_int = 0;
pub const C_WD33C93A: c_int = 1;
pub const C_WD33C93B: c_int = 2;
pub const C_UNKNOWN_CHIP: c_int = 100;
// defines for hostdata->state
pub const S_UNCONNECTED: c_int = 0;
pub const S_SELECTING: c_int = 1;
pub const S_RUNNING_LEVEL2: c_int = 2;
pub const S_CONNECTED: c_int = 3;
pub const S_PRE_TMP_DISC: c_int = 4;
pub const S_PRE_CMP_DISC: c_int = 5;
// defines for hostdata->dma
pub const D_DMA_OFF: c_int = 0;
pub const D_DMA_RUNNING: c_int = 1;
// defines for hostdata->level2
// NOTE: only the first 3 are implemented so far

// defines for hostdata->disconnect
pub const DIS_NEVER: c_int = 0;
pub const DIS_ADAPTIVE: c_int = 1;
pub const DIS_ALWAYS: c_int = 2;
// defines for hostdata->args

pub const DB_MASK: c_uint = 0x3f;
// defines for hostdata->sync_stat[]
pub const SS_UNSET: c_int = 0;
pub const SS_FIRST: c_int = 1;
pub const SS_WAITING: c_int = 2;
pub const SS_SET: c_int = 3;
// defines for hostdata->proc

extern "C" {
    pub fn wd33c93_abort(cmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn wd33c93_intr(instance: *mut Scsi_Host);
}
extern "C" {
    pub fn wd33c93_show_info(: *mut seq_file, : *mut Scsi_Host) -> c_int;
}
extern "C" {
    pub fn wd33c93_write_info(: *mut Scsi_Host, : *mut c_char, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn wd33c93_host_reset(: *mut scsi_cmnd) -> c_int;
}
