//! Automatically rewritten from C Header to Rust Module
//! Source: net/ethtool/cmis.h
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
pub const ETHTOOL_CMIS_CDB_LPL_MAX_PL_LENGTH: c_int = 120;
pub const ETHTOOL_CMIS_CDB_EPL_MAX_PL_LENGTH: c_int = 2048;
pub const ETHTOOL_CMIS_CDB_CMD_PAGE: c_uint = 0x9F;
pub const ETHTOOL_CMIS_CDB_PAGE_I2C_ADDR: c_uint = 0x50;
//
// struct ethtool_cmis_cdb - CDB commands parameters
// @cmis_rev: CMIS revision major.
// @read_write_len_ext: Allowable additional number of byte octets to the LPL
// in a READ or a WRITE CDB commands.
// @max_completion_time:  Maximum CDB command completion time in msec.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_cmis_cdb {
    pub cmis_rev: u8,
    pub read_write_len_ext: u8,
    pub max_completion_time: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ethtool_cmis_cdb_cmd_id {
    ETHTOOL_CMIS_CDB_CMD_QUERY_STATUS		= 0x0000,
    ETHTOOL_CMIS_CDB_CMD_MODULE_FEATURES		= 0x0040,
    ETHTOOL_CMIS_CDB_CMD_FW_MANAGMENT_FEATURES	= 0x0041,
    ETHTOOL_CMIS_CDB_CMD_START_FW_DOWNLOAD		= 0x0101,
    ETHTOOL_CMIS_CDB_CMD_WRITE_FW_BLOCK_LPL		= 0x0103,
    ETHTOOL_CMIS_CDB_CMD_WRITE_FW_BLOCK_EPL		= 0x0104,
    ETHTOOL_CMIS_CDB_CMD_COMPLETE_FW_DOWNLOAD	= 0x0107,
    ETHTOOL_CMIS_CDB_CMD_RUN_FW_IMAGE		= 0x0109,
    ETHTOOL_CMIS_CDB_CMD_COMMIT_FW_IMAGE		= 0x010A,
}

//
// struct ethtool_cmis_cdb_request - CDB commands request fields as decribed in
// the CMIS standard
// @id: Command ID.
// @epl_len: EPL memory length.
// @lpl_len: LPL memory length.
// @chk_code: Check code for the previous field and the payload.
// @resv1: Added to match the CMIS standard request continuity.
// @resv2: Added to match the CMIS standard request continuity.
// @payload: Payload for the CDB commands.
// @epl: Extended payload for the CDB commands.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_cmis_cdb_request {
    pub id: __be16,
    pub epl_len: __be16,
    pub lpl_len: u8,
    pub chk_code: u8,
    pub resv1: u8,
    pub resv2: u8,
    pub payload: [u8; ETHTOOL_CMIS_CDB_LPL_MAX_PL_LENGTH],
    pub /: *mut *mut *mut u8 epl; / Everything above this field checksummed.,
}

//
// struct ethtool_cmis_cdb_cmd_args - CDB commands execution arguments
// @req: CDB command fields as described in the CMIS standard.
// @max_duration: Maximum duration time for command completion in msec.
// @msleep_pre_rpl: Waiting time before checking reply in msec.
// @read_write_len_ext: Allowable additional number of byte octets to the LPL
// in a READ or a WRITE commands.
// @rpl_exp_len: Expected reply length in bytes.
// @flags: Validation flags for CDB commands.
// @err_msg: Error message to be sent to user space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_cmis_cdb_cmd_args {
    pub req: ethtool_cmis_cdb_request,
    pub max_duration: u16,
    pub msleep_pre_rpl: u16,
    pub read_write_len_ext: u8,
    pub rpl_exp_len: u8,
    pub flags: u8,
    pub err_msg: *mut c_char,
}

//
// struct ethtool_cmis_cdb_rpl_hdr - CDB commands reply header arguments
// @rpl_len: Reply length.
// @rpl_chk_code: Reply check code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_cmis_cdb_rpl_hdr {
    pub rpl_len: u8,
    pub rpl_chk_code: u8,
}

//
// struct ethtool_cmis_cdb_rpl - CDB commands reply arguments
// @hdr: CDB commands reply header arguments.
// @payload: Payload for the CDB commands reply.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_cmis_cdb_rpl {
    pub hdr: ethtool_cmis_cdb_rpl_hdr,
    pub payload: [u8; ETHTOOL_CMIS_CDB_LPL_MAX_PL_LENGTH],
}

extern "C" {
    pub fn ethtool_cmis_get_max_lpl_size(num_of_byte_octs: u8) -> u32;
}
extern "C" {
    pub fn ethtool_cmis_cdb_check_completion_flag(cmis_rev: u8, flags: *mut u8);
}
extern "C" {
    pub fn ethtool_cmis_cdb_fini(cdb: *mut ethtool_cmis_cdb);
}
