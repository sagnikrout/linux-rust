//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/fcx.h
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
//
// Functions for assembling fcx enabled I/O control blocks.
//
// Copyright IBM Corp. 2008
// Author(s): Peter Oberparleiter <peter.oberparleiter@de.ibm.com>
//

pub const TCW_FORMAT_DEFAULT: c_int = 0;
pub const TCW_TIDAW_FORMAT_DEFAULT: c_int = 0;

//
// struct tcw - Transport Control Word (TCW)
// @format: TCW format
// @flags: TCW flags
// @tccbl: Transport-Command-Control-Block Length
// @r: Read Operations
// @w: Write Operations
// @output: Output-Data Address
// @input: Input-Data Address
// @tsb: Transport-Status-Block Address
// @tccb: Transport-Command-Control-Block Address
// @output_count: Output Count
// @input_count: Input Count
// @intrg: Interrogate TCW Address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcw {
    pub format:2: u32,
    pub :6: u32,
    pub flags:24: u32,
    pub :8: u32,
    pub tccbl:6: u32,
    pub r:1: u32,
    pub w:1: u32,
    pub :16: u32,
    pub output: dma64_t,
    pub input: dma64_t,
    pub tsb: dma64_t,
    pub tccb: dma64_t,
    pub output_count: u32,
    pub input_count: u32,
    pub :32: u32,
    pub :32: u32,
    pub :32: u32,
    pub intrg: dma32_t,
// C attribute field omitted

//
// struct tidaw - Transport-Indirect-Addressing Word (TIDAW)
// @flags: TIDAW flags. Can be an arithmetic OR of the following constants:
// %TIDAW_FLAGS_LAST, %TIDAW_FLAGS_SKIP, %TIDAW_FLAGS_DATA_INT,
// %TIDAW_FLAGS_TTIC, %TIDAW_FLAGS_INSERT_CBC
// @count: Count
// @addr: Address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tidaw {
    pub flags:8: u32,
    pub :24: u32,
    pub count: u32,
    pub addr: dma64_t,
// C attribute field omitted
//
// struct tsa_iostat - I/O-Status Transport-Status Area (IO-Stat TSA)
// @dev_time: Device Time
// @def_time: Defer Time
// @queue_time: Queue Time
// @dev_busy_time: Device-Busy Time
// @dev_act_time: Device-Active-Only Time
// @sense: Sense Data (if present)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsa_iostat {
    pub dev_time: u32,
    pub def_time: u32,
    pub queue_time: u32,
    pub dev_busy_time: u32,
    pub dev_act_time: u32,
    pub sense: [u8; 32],
// C attribute field omitted
//
// struct tsa_ddpcs - Device-Detected-Program-Check Transport-Status Area (DDPC TSA)
// @rc: Reason Code
// @rcq: Reason Code Qualifier
// @sense: Sense Data (if present)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsa_ddpc {
    pub :24: u32,
    pub rc:8: u32,
    pub rcq: [u8; 16],
    pub sense: [u8; 32],
// C attribute field omitted

//
// struct tsa_intrg - Interrogate Transport-Status Area (Intrg. TSA)
// @format: Format
// @flags: Flags. Can be an arithmetic OR of the following constants:
// %TSA_INTRG_FLAGS_CU_STATE_VALID, %TSA_INTRG_FLAGS_DEV_STATE_VALID,
// %TSA_INTRG_FLAGS_OP_STATE_VALID
// @cu_state: Controle-Unit State
// @dev_state: Device State
// @op_state: Operation State
// @sd_info: State-Dependent Information
// @dl_id: Device-Level Identifier
// @dd_data: Device-Dependent Data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsa_intrg {
    pub format:8: u32,
    pub flags:8: u32,
    pub cu_state:8: u32,
    pub dev_state:8: u32,
    pub op_state:8: u32,
    pub :24: u32,
    pub sd_info: [u8; 12],
    pub dl_id: u32,
    pub dd_data: [u8; 28],
// C attribute field omitted
pub const TSB_FORMAT_NONE: c_int = 0;
pub const TSB_FORMAT_IOSTAT: c_int = 1;
pub const TSB_FORMAT_DDPC: c_int = 2;
pub const TSB_FORMAT_INTRG: c_int = 3;

//
// struct tsb - Transport-Status Block (TSB)
// @length: Length
// @flags: Flags. Can be an arithmetic OR of the following constants:
// %TSB_FLAGS_DCW_OFFSET_VALID, %TSB_FLAGS_COUNT_VALID, %TSB_FLAGS_CACHE_MISS,
// %TSB_FLAGS_TIME_VALID
// @dcw_offset: DCW Offset
// @count: Count
// @tsa: Transport-Status-Area
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsb {
    pub length:8: u32,
    pub flags:8: u32,
    pub dcw_offset:16: u32,
    pub count: u32,
    pub :32: u32,
    pub iostat: tsa_iostat,
    pub ddpc: tsa_ddpc,
    pub intrg: tsa_intrg,
// C attribute field omitted
// C attribute field omitted
pub const DCW_INTRG_FORMAT_DEFAULT: c_int = 0;
pub const DCW_INTRG_RC_UNSPECIFIED: c_int = 0;
pub const DCW_INTRG_RC_TIMEOUT: c_int = 1;
pub const DCW_INTRG_RCQ_UNSPECIFIED: c_int = 0;
pub const DCW_INTRG_RCQ_PRIMARY: c_int = 1;
pub const DCW_INTRG_RCQ_SECONDARY: c_int = 2;

//
// struct dcw_intrg_data - Interrogate DCW data
// @format: Format. Should be %DCW_INTRG_FORMAT_DEFAULT
// @rc: Reason Code. Can be one of %DCW_INTRG_RC_UNSPECIFIED,
// %DCW_INTRG_RC_TIMEOUT
// @rcq: Reason Code Qualifier: Can be one of %DCW_INTRG_RCQ_UNSPECIFIED,
// %DCW_INTRG_RCQ_PRIMARY, %DCW_INTRG_RCQ_SECONDARY
// @lpm: Logical-Path Mask
// @pam: Path-Available Mask
// @pim: Path-Installed Mask
// @timeout: Timeout
// @flags: Flags. Can be an arithmetic OR of %DCW_INTRG_FLAGS_MPM,
// %DCW_INTRG_FLAGS_PPR, %DCW_INTRG_FLAGS_CRIT
// @time: Time
// @prog_id: Program Identifier
// @prog_data: Program-Dependent Data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcw_intrg_data {
    pub format:8: u32,
    pub rc:8: u32,
    pub rcq:8: u32,
    pub lpm:8: u32,
    pub pam:8: u32,
    pub pim:8: u32,
    pub timeout:16: u32,
    pub flags:8: u32,
    pub :24: u32,
    pub :32: u32,
    pub time: u64,
    pub prog_id: u64,
    pub prog_data: [u8; ],
// C attribute field omitted

pub const DCW_CMD_WRITE: c_uint = 0x01;
pub const DCW_CMD_READ: c_uint = 0x02;
pub const DCW_CMD_CONTROL: c_uint = 0x03;
pub const DCW_CMD_SENSE: c_uint = 0x04;
pub const DCW_CMD_SENSE_ID: c_uint = 0xe4;
pub const DCW_CMD_INTRG: c_uint = 0x40;
//
// struct dcw - Device-Command Word (DCW)
// @cmd: Command Code. Can be one of %DCW_CMD_WRITE, %DCW_CMD_READ,
// %DCW_CMD_CONTROL, %DCW_CMD_SENSE, %DCW_CMD_SENSE_ID, %DCW_CMD_INTRG
// @flags: Flags. Can be an arithmetic OR of %DCW_FLAGS_CC
// @cd_count: Control-Data Count
// @count: Count
// @cd: Control Data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcw {
    pub cmd:8: u32,
    pub flags:8: u32,
    pub :8: u32,
    pub cd_count:8: u32,
    pub count: u32,
    pub cd: [u8; ],
// C attribute field omitted
pub const TCCB_FORMAT_DEFAULT: c_uint = 0x7f;
pub const TCCB_MAX_DCW: c_int = 30;

pub const TCCB_SAC_DEFAULT: c_uint = 0x1ffe;
pub const TCCB_SAC_INTRG: c_uint = 0x1fff;
//
// struct tccb_tcah - Transport-Command-Area Header (TCAH)
// @format: Format. Should be %TCCB_FORMAT_DEFAULT
// @tcal: Transport-Command-Area Length
// @sac: Service-Action Code. Can be one of %TCCB_SAC_DEFAULT, %TCCB_SAC_INTRG
// @prio: Priority
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tccb_tcah {
    pub format:8: u32,
    pub :24: u32,
    pub :24: u32,
    pub tcal:8: u32,
    pub sac:16: u32,
    pub :8: u32,
    pub prio:8: u32,
    pub :32: u32,
// C attribute field omitted
//
// struct tccb_tcat - Transport-Command-Area Trailer (TCAT)
// @count: Transport Count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tccb_tcat {
    pub :32: u32,
    pub count: u32,
// C attribute field omitted
//
// struct tccb - (partial) Transport-Command-Control Block (TCCB)
// @tcah: TCAH
// @tca: Transport-Command Area
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tccb {
    pub tcah: tccb_tcah,
    pub tca: [u8; ],
// C attribute field omitted
    pub tcw): *mut *mut tcw tcw_get_intrg(tcw,
    pub tcw): *mut *mut void tcw_get_data(struct tcw,
    pub tcw): *mut *mut tccb tcw_get_tccb(tcw,
    pub tcw): *mut *mut tsb tcw_get_tsb(tcw,
    pub w): *mut *mut void tcw_init(struct tcw tcw, int r, int,
    pub num_tidaws): *mut *mut void tcw_finalize(struct tcw tcw, int,
    pub intrg_tcw): *mut *mut void tcw_set_intrg(struct tcw tcw, struct tcw,
    pub use_tidal): *mut *mut *mut void tcw_set_data(struct tcw tcw, void data, int,
    pub tccb): *mut *mut void tcw_set_tccb(struct tcw tcw, struct tccb,
    pub tsb): *mut *mut void tcw_set_tsb(struct tcw tcw, struct tsb,
    pub sac): *mut *mut void tccb_init(struct tccb tccb, size_t tccb_size, u32,
    pub tsb): *mut void tsb_init(struct tsb,
    pub count): *mut *mut void cd, u8 cd_count, u32,
    pub count): *mut *mut void addr, u32,
