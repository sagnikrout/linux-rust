//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/net/intel/libie/adminq.h
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
// Copyright (C) 2025 Intel Corporation

pub const LIBIE_AQ_MAX_BUF_LEN: c_int = 4096;
//
// struct libie_aqc_generic - Generic structure used in adminq communication
// @param0: generic parameter high 32bit
// @param1: generic parameter lower 32bit
// @addr_high: generic address high 32bit
// @addr_low: generic address lower 32bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_aqc_generic {
    pub param0: __le32,
    pub param1: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

//
// struct libie_aqc_get_ver -  Used in command get version (direct 0x0001)
// @rom_ver: rom version
// @fw_build: number coressponding to firmware build
// @fw_branch: branch identifier of firmware version
// @fw_major: major number of firmware version
// @fw_minor: minor number of firmware version
// @fw_patch: patch of firmware version
// @api_branch: brancch identifier of API version
// @api_major: major number of API version
// @api_minor: minor number of API version
// @api_patch: patch of API version
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_aqc_get_ver {
    pub rom_ver: __le32,
    pub fw_build: __le32,
    pub fw_branch: u8,
    pub fw_major: u8,
    pub fw_minor: u8,
    pub fw_patch: u8,
    pub api_branch: u8,
    pub api_major: u8,
    pub api_minor: u8,
    pub api_patch: u8,
}

//
// struct libie_aqc_driver_ver - Used in command send driver version
// (indirect 0x0002)
// @major_ver: driver major version
// @minor_ver: driver minor version
// @build_ver: driver build version
// @subbuild_ver: driver subbuild version
// @reserved: for feature use
// @addr_high: high part of response address buff
// @addr_low: low part of response address buff
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_aqc_driver_ver {
    pub major_ver: u8,
    pub minor_ver: u8,
    pub build_ver: u8,
    pub subbuild_ver: u8,
    pub reserved: [u8; 4],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libie_aq_res_id {
    LIBIE_AQC_RES_ID_NVM				= 1,
    LIBIE_AQC_RES_ID_SDP				= 2,
    LIBIE_AQC_RES_ID_CHNG_LOCK			= 3,
    LIBIE_AQC_RES_ID_GLBL_LOCK			= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libie_aq_res_access_type {
    LIBIE_AQC_RES_ACCESS_READ			= 1,
    LIBIE_AQC_RES_ACCESS_WRITE			= 2,
}

pub const LIBIE_AQ_RES_NVM_READ_DFLT_TIMEOUT_MS: c_int = 3000;
pub const LIBIE_AQ_RES_NVM_WRITE_DFLT_TIMEOUT_MS: c_int = 180000;
pub const LIBIE_AQ_RES_CHNG_LOCK_DFLT_TIMEOUT_MS: c_int = 1000;
pub const LIBIE_AQ_RES_GLBL_LOCK_DFLT_TIMEOUT_MS: c_int = 3000;
pub const LIBIE_AQ_RES_GLBL_SUCCESS: c_int = 0;
pub const LIBIE_AQ_RES_GLBL_IN_PROG: c_int = 1;
pub const LIBIE_AQ_RES_GLBL_DONE: c_int = 2;
//
// struct libie_aqc_req_res - Request resource ownership
// @res_id: resource ID (look at enum definition above)
// @access_type: read or write (enum definition above)
// @timeout: Upon successful completion, FW writes this value and driver is
// expected to release resource before timeout. This value is provided in
// milliseconds.
// @res_number: for SDP, this is the pin ID of the SDP
// @status: status only used for LIBIE_AQC_RES_ID_GLBL_LOCK, for others reserved
// @reserved: reserved for future use
//
// Used in commands:
// request resource ownership (direct 0x0008)
// request resource ownership (direct 0x0009)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_aqc_req_res {
    pub res_id: __le16,
    pub access_type: __le16,
    pub timeout: __le32,
    pub res_number: __le32,
    pub status: __le16,
    pub reserved: [u8; 2],
}

//
// struct libie_aqc_list_caps - Getting capabilities
// @cmd_flags: command flags
// @pf_index: index of PF to get caps from
// @reserved: reserved for future use
// @count: number of capabilities records
// @addr_high: high part of response address buff
// @addr_low: low part of response address buff
//
// Used in commands:
// get function capabilities (indirect 0x000A)
// get device capabilities (indirect 0x000B)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_aqc_list_caps {
    pub cmd_flags: u8,
    pub pf_index: u8,
    pub reserved: [u8; 2],
    pub count: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Device/Function buffer entry, repeated per reported capability
pub const LIBIE_AQC_CAPS_SWITCH_MODE: c_uint = 0x0001;
pub const LIBIE_AQC_CAPS_MNG_MODE: c_uint = 0x0002;
pub const LIBIE_AQC_CAPS_NPAR_ACTIVE: c_uint = 0x0003;
pub const LIBIE_AQC_CAPS_OS2BMC_CAP: c_uint = 0x0004;
pub const LIBIE_AQC_CAPS_VALID_FUNCTIONS: c_uint = 0x0005;
pub const LIBIE_AQC_MAX_VALID_FUNCTIONS: c_uint = 0x8;
pub const LIBIE_AQC_CAPS_SRIOV: c_uint = 0x0012;
pub const LIBIE_AQC_CAPS_VF: c_uint = 0x0013;
pub const LIBIE_AQC_CAPS_VMDQ: c_uint = 0x0014;
pub const LIBIE_AQC_CAPS_8021QBG: c_uint = 0x0015;
pub const LIBIE_AQC_CAPS_8021QBR: c_uint = 0x0016;
pub const LIBIE_AQC_CAPS_VSI: c_uint = 0x0017;
pub const LIBIE_AQC_CAPS_DCB: c_uint = 0x0018;
pub const LIBIE_AQC_CAPS_FCOE: c_uint = 0x0021;
pub const LIBIE_AQC_CAPS_ISCSI: c_uint = 0x0022;
pub const LIBIE_AQC_CAPS_RSS: c_uint = 0x0040;
pub const LIBIE_AQC_CAPS_RXQS: c_uint = 0x0041;
pub const LIBIE_AQC_CAPS_TXQS: c_uint = 0x0042;
pub const LIBIE_AQC_CAPS_MSIX: c_uint = 0x0043;
pub const LIBIE_AQC_CAPS_VF_MSIX: c_uint = 0x0044;
pub const LIBIE_AQC_CAPS_FD: c_uint = 0x0045;
pub const LIBIE_AQC_CAPS_1588: c_uint = 0x0046;
pub const LIBIE_AQC_CAPS_MAX_MTU: c_uint = 0x0047;
pub const LIBIE_AQC_CAPS_NVM_VER: c_uint = 0x0048;
pub const LIBIE_AQC_CAPS_PENDING_NVM_VER: c_uint = 0x0049;
pub const LIBIE_AQC_CAPS_OROM_VER: c_uint = 0x004A;
pub const LIBIE_AQC_CAPS_PENDING_OROM_VER: c_uint = 0x004B;
pub const LIBIE_AQC_CAPS_NET_VER: c_uint = 0x004C;
pub const LIBIE_AQC_CAPS_PENDING_NET_VER: c_uint = 0x004D;
pub const LIBIE_AQC_CAPS_RDMA: c_uint = 0x0051;
pub const LIBIE_AQC_CAPS_LED: c_uint = 0x0061;
pub const LIBIE_AQC_CAPS_SDP: c_uint = 0x0062;
pub const LIBIE_AQC_CAPS_MDIO: c_uint = 0x0063;
pub const LIBIE_AQC_CAPS_WSR_PROT: c_uint = 0x0064;
pub const LIBIE_AQC_CAPS_SENSOR_READING: c_uint = 0x0067;
pub const LIBIE_AQC_INLINE_IPSEC: c_uint = 0x0070;
pub const LIBIE_AQC_CAPS_NUM_ENABLED_PORTS: c_uint = 0x0072;
pub const LIBIE_AQC_CAPS_PCIE_RESET_AVOIDANCE: c_uint = 0x0076;
pub const LIBIE_AQC_CAPS_POST_UPDATE_RESET_RESTRICT: c_uint = 0x0077;
pub const LIBIE_AQC_CAPS_NVM_MGMT: c_uint = 0x0080;
pub const LIBIE_AQC_CAPS_EXT_TOPO_DEV_IMG0: c_uint = 0x0081;
pub const LIBIE_AQC_CAPS_EXT_TOPO_DEV_IMG1: c_uint = 0x0082;
pub const LIBIE_AQC_CAPS_EXT_TOPO_DEV_IMG2: c_uint = 0x0083;
pub const LIBIE_AQC_CAPS_EXT_TOPO_DEV_IMG3: c_uint = 0x0084;
pub const LIBIE_AQC_CAPS_TX_SCHED_TOPO_COMP_MODE: c_uint = 0x0085;
pub const LIBIE_AQC_CAPS_NAC_TOPOLOGY: c_uint = 0x0087;
pub const LIBIE_AQC_CAPS_FW_LAG_SUPPORT: c_uint = 0x0092;

pub const LIBIE_AQC_CAPS_EEE: c_uint = 0x009B;
pub const LIBIE_AQC_CAPS_FLEX10: c_uint = 0x00F1;
pub const LIBIE_AQC_CAPS_CEM: c_uint = 0x00F2;
//
// struct libie_aqc_list_caps_elem - Getting list of caps elements
// @cap: one from the defines list above
// @major_ver: major version
// @minor_ver: minor version
// @number: number of resources described by this capability
// @logical_id: logical ID, only meaningful for some types of resources
// @phys_id: physical ID, only meaningful for some types of resources
// @rsvd1: reserved for future use
// @rsvd2: reserved for future use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_aqc_list_caps_elem {
    pub cap: __le16,
    pub major_ver: u8,
    pub minor_ver: u8,
    pub number: __le32,
    pub logical_id: __le32,
    pub phys_id: __le32,
    pub rsvd1: __le64,
    pub rsvd2: __le64,
}

// Admin Queue command opcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libie_adminq_opc {
// FW Logging Commands
    libie_aqc_opc_fw_logs_config			= 0xFF30,
    libie_aqc_opc_fw_logs_register			= 0xFF31,
    libie_aqc_opc_fw_logs_query			= 0xFF32,
    libie_aqc_opc_fw_logs_event			= 0xFF33,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libie_aqc_fw_logging_mod {
    LIBIE_AQC_FW_LOG_ID_GENERAL = 0,
    LIBIE_AQC_FW_LOG_ID_CTRL,
    LIBIE_AQC_FW_LOG_ID_LINK,
    LIBIE_AQC_FW_LOG_ID_LINK_TOPO,
    LIBIE_AQC_FW_LOG_ID_DNL,
    LIBIE_AQC_FW_LOG_ID_I2C,
    LIBIE_AQC_FW_LOG_ID_SDP,
    LIBIE_AQC_FW_LOG_ID_MDIO,
    LIBIE_AQC_FW_LOG_ID_ADMINQ,
    LIBIE_AQC_FW_LOG_ID_HDMA,
    LIBIE_AQC_FW_LOG_ID_LLDP,
    LIBIE_AQC_FW_LOG_ID_DCBX,
    LIBIE_AQC_FW_LOG_ID_DCB,
    LIBIE_AQC_FW_LOG_ID_XLR,
    LIBIE_AQC_FW_LOG_ID_NVM,
    LIBIE_AQC_FW_LOG_ID_AUTH,
    LIBIE_AQC_FW_LOG_ID_VPD,
    LIBIE_AQC_FW_LOG_ID_IOSF,
    LIBIE_AQC_FW_LOG_ID_PARSER,
    LIBIE_AQC_FW_LOG_ID_SW,
    LIBIE_AQC_FW_LOG_ID_SCHEDULER,
    LIBIE_AQC_FW_LOG_ID_TXQ,
    LIBIE_AQC_FW_LOG_ID_RSVD,
    LIBIE_AQC_FW_LOG_ID_POST,
    LIBIE_AQC_FW_LOG_ID_WATCHDOG,
    LIBIE_AQC_FW_LOG_ID_TASK_DISPATCH,
    LIBIE_AQC_FW_LOG_ID_MNG,
    LIBIE_AQC_FW_LOG_ID_SYNCE,
    LIBIE_AQC_FW_LOG_ID_HEALTH,
    LIBIE_AQC_FW_LOG_ID_TSDRV,
    LIBIE_AQC_FW_LOG_ID_PFREG,
    LIBIE_AQC_FW_LOG_ID_MDLVER,
    LIBIE_AQC_FW_LOG_ID_MAX
}

// Set FW Logging configuration (indirect 0xFF30)
// Register for FW Logging (indirect 0xFF31)
// Query FW Logging (indirect 0xFF32)
// FW Log Event (indirect 0xFF33)
//

pub const LIBIE_AQC_FW_LOG_MIN_RESOLUTION: c_int = 1;
pub const LIBIE_AQC_FW_LOG_MAX_RESOLUTION: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_aqc_fw_log {
    pub cmd_flags: u8,
    pub rsp_flag: u8,
    pub fw_rt_msb: __le16,
    pub fw_rt_lsb: __le32,
    pub sync: },
    pub log_resolution: __le16,
    pub mdl_cnt: __le16,
    pub cfg: },
    pub ops: },
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Response Buffer for:
// Set Firmware Logging Configuration (0xFF30)
// Query FW Logging (0xFF32)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_aqc_fw_log_cfg_resp {
    pub module_identifier: __le16,
    pub log_level: u8,
    pub rsvd0: u8,
}

//
// struct libie_aq_desc - Admin Queue (AQ) descriptor
// @flags: LIBIE_AQ_FLAG_* flags
// @opcode: AQ command opcode
// @datalen: length in bytes of indirect/external data buffer
// @retval: return value from firmware
// @cookie_high: opaque data high-half
// @cookie_low: opaque data low-half
// @params: command-specific parameters
//
// Descriptor format for commands the driver posts on the Admin Transmit Queue
// (ATQ). The firmware writes back onto the command descriptor and returns
// the result of the command. Asynchronous events that are not an immediate
// result of the command are written to the Admin Receive Queue (ARQ) using
// the same descriptor format. Descriptors are in little-endian notation with
// 32-bit words.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_aq_desc {
    pub flags: __le16,
    pub opcode: __le16,
    pub datalen: __le16,
    pub retval: __le16,
    pub cookie_high: __le32,
    pub cookie_low: __le32,
    pub raw: [u8; 16],
    pub generic: libie_aqc_generic,
    pub get_ver: libie_aqc_get_ver,
    pub driver_ver: libie_aqc_driver_ver,
    pub res_owner: libie_aqc_req_res,
    pub get_cap: libie_aqc_list_caps,
    pub fw_log: libie_aqc_fw_log,
    pub params: },
}

// FW defined boundary for a large buffer, 4k >= Large buffer > 512 bytes
pub const LIBIE_AQ_LG_BUF: c_int = 512;
// Flags sub-structure
// |0  |1  |2  |3  |4  |5  |6  |7  |8  |9  |10 |11 |12 |13 |14 |15 |
// |DD |CMP|ERR|VFE| * *  RESERVED * * |LB |RD |VFC|BUF|SI |EI |FE |
//

// error codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libie_aq_err {
    LIBIE_AQ_RC_OK		= 0,  /* Success */
    LIBIE_AQ_RC_EPERM	= 1,  /* Operation not permitted */
    LIBIE_AQ_RC_ENOENT	= 2,  /* No such element */
    LIBIE_AQ_RC_ESRCH	= 3,  /* Bad opcode */
    LIBIE_AQ_RC_EIO		= 5,  /* I/O error */
    LIBIE_AQ_RC_EAGAIN	= 8,  /* Try again */
    LIBIE_AQ_RC_ENOMEM	= 9,  /* Out of memory */
    LIBIE_AQ_RC_EACCES	= 10, /* Permission denied */
    LIBIE_AQ_RC_EBUSY	= 12, /* Device or resource busy */
    LIBIE_AQ_RC_EEXIST	= 13, /* Object already exists */
    LIBIE_AQ_RC_EINVAL	= 14, /* Invalid argument */
    LIBIE_AQ_RC_ENOSPC	= 16, /* No space left or allocation failure */
    LIBIE_AQ_RC_ENOSYS	= 17, /* Function not implemented */
    LIBIE_AQ_RC_EMODE	= 21, /* Op not allowed in current dev mode */
    LIBIE_AQ_RC_ENOSEC	= 24, /* Missing security manifest */
    LIBIE_AQ_RC_EBADSIG	= 25, /* Bad RSA signature */
    LIBIE_AQ_RC_ESVN	= 26, /* SVN number prohibits this package */
    LIBIE_AQ_RC_EBADMAN	= 27, /* Manifest hash mismatch */
    LIBIE_AQ_RC_EBADBUF	= 28, /* Buffer hash mismatches manifest */
}
