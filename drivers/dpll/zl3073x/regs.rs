//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dpll/zl3073x/regs.h
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
// Hardware limits for ZL3073x chip family
//
pub const ZL3073X_MAX_CHANNELS: c_int = 5;
pub const ZL3073X_NUM_REFS: c_int = 10;
pub const ZL3073X_NUM_OUTS: c_int = 10;
pub const ZL3073X_NUM_SYNTHS: c_int = 5;

//
// Register address structure:
// ===========================
// 25        19 18  16 15     7 6           0
// +------------------------------------------+
// | max_offset | size |  page  | page_offset |
// +------------------------------------------+
//
// page_offset ... <0x00..0x7F>
// page .......... HW page number
// size .......... register byte size (1, 2, 4 or 6)
// max_offset .... maximal offset for indexed registers
// (for non-indexed regs max_offset == page_offset)
//

//
// ZL_REG_IDX - define indexed register
// @_idx: index of register to access
// @_page: register page
// @_offset: register offset in page
// @_size: register byte size (1, 2, 4 or 6)
// @_items: number of register indices
// @_stride: stride between items in bytes
//
// All parameters except @_idx should be constant.
//

//
// ZL_REG - define simple (non-indexed) register
// @_page: register page
// @_offset: register offset in page
// @_size: register byte size (1, 2, 4 or 6)
//
// All parameters should be constant.
//

//
// Register Page 0, General
//

//
// Register Page 2, Status
//

pub const ZL_REF_MON_STATUS_OK: c_int = 0;

pub const ZL_DPLL_MON_STATUS_STATE_ACQUIRING: c_int = 0;
pub const ZL_DPLL_MON_STATUS_STATE_LOCK: c_int = 1;
pub const ZL_DPLL_MON_STATUS_STATE_HOLDOVER: c_int = 2;

pub const ZL_DPLL_REFSEL_STATUS_STATE_LOCK: c_int = 4;

//
// Register Page 4, Ref
//

pub const ZL_REF_FREQ_MEAS_CTRL_REF_FREQ: c_int = 1;
pub const ZL_REF_FREQ_MEAS_CTRL_REF_FREQ_OFF: c_int = 2;
pub const ZL_REF_FREQ_MEAS_CTRL_DPLL_FREQ_OFF: c_int = 3;

//
// Register Page 5, DPLL
//

pub const ZL_DPLL_MODE_REFSEL_MODE_FREERUN: c_int = 0;
pub const ZL_DPLL_MODE_REFSEL_MODE_HOLDOVER: c_int = 1;
pub const ZL_DPLL_MODE_REFSEL_MODE_REFLOCK: c_int = 2;
pub const ZL_DPLL_MODE_REFSEL_MODE_AUTO: c_int = 3;
pub const ZL_DPLL_MODE_REFSEL_MODE_NCO: c_int = 4;

pub const ZL_DPLL_DF_READ_CMD_ACC_I: c_int = 4;

pub const ZL_DPLL_TIE_CTRL_OP_WR: c_int = 4;

pub const ZL_DPLL_TOD_CTRL_CMD_WR_NEXT_1HZ: c_int = 1;
pub const ZL_DPLL_TOD_CTRL_CMD_RD_CURRENT: c_int = 8;
pub const ZL_DPLL_TOD_CTRL_CMD_RD_NEXT_1HZ: c_int = 9;

//
// Register Pages 6-7, DPLL Data
//
// Per-channel registers with stride 0x20. Channels 0-3 reside on page 6,
// channel 4 on page 7.
//

//
// Register Page 9, Synth and Output
//

pub const ZL_OUTPUT_PHASE_STEP_CTRL_OP_NONE: c_int = 0;
pub const ZL_OUTPUT_PHASE_STEP_CTRL_OP_RESET: c_int = 1;
pub const ZL_OUTPUT_PHASE_STEP_CTRL_OP_READ: c_int = 2;
pub const ZL_OUTPUT_PHASE_STEP_CTRL_OP_WRITE: c_int = 3;

//
// Register Page 10, Ref Mailbox
//

pub const ZL_REF_SYNC_CTRL_MODE_REFSYNC_PAIR_OFF: c_int = 0;
pub const ZL_REF_SYNC_CTRL_MODE_REFSYNC_PAIR: c_int = 1;
pub const ZL_REF_SYNC_CTRL_MODE_50_50_ESYNC_25_75: c_int = 2;

pub const ZL_REF_ESYNC_DIV_1HZ: c_int = 0;
//
// Register Page 12, DPLL Mailbox
//

pub const ZL_DPLL_REF_PRIO_MAX: c_int = 14;
pub const ZL_DPLL_REF_PRIO_NONE: c_int = 15;
//
// Register Page 13, Synth Mailbox
//

//
// Register Page 14, Output Mailbox
//

pub const ZL_OUTPUT_MODE_CLOCK_TYPE_NORMAL: c_int = 0;
pub const ZL_OUTPUT_MODE_CLOCK_TYPE_ESYNC: c_int = 1;

pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_DISABLED: c_int = 0;
pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_LVDS: c_int = 1;
pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_DIFF: c_int = 2;
pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_LOWVCM: c_int = 3;
pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_2: c_int = 4;
pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_1P: c_int = 5;
pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_1N: c_int = 6;
pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_2_INV: c_int = 7;
pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_2_NDIV: c_int = 12;
pub const ZL_OUTPUT_MODE_SIGNAL_FORMAT_2_NDIV_INV: c_int = 15;

//
// Register Page 255 - HW registers access
//

pub const ZL_HWREG_OP_WRITE: c_uint = 0x28;
pub const ZL_HWREG_OP_READ: c_uint = 0x29;

//
// Registers available in flash mode
//

pub const ZL_WRITE_FLASH_OP_DONE: c_uint = 0x0;
pub const ZL_WRITE_FLASH_OP_SECTORS: c_uint = 0x2;
pub const ZL_WRITE_FLASH_OP_PAGE: c_uint = 0x3;
pub const ZL_WRITE_FLASH_OP_COPY_PAGE: c_uint = 0x4;

pub const ZL_FLASH_INFO_SECTOR_4K: c_int = 0;
pub const ZL_FLASH_INFO_SECTOR_64K: c_int = 1;

pub const ZL_OP_STATE_NO_COMMAND: c_int = 0;
pub const ZL_OP_STATE_PENDING: c_int = 1;
pub const ZL_OP_STATE_DONE: c_int = 2;
