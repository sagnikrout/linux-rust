//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla2xxx/qla_edif_bsg.h
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
// Marvell Fibre Channel HBA Driver
// Copyright (C)  2018-	    Marvell
//
pub const EDIF_VERSION1: c_int = 1;
// BSG Vendor specific commands
pub const ELS_MAX_PAYLOAD: c_int = 2112;

pub const WWN_SIZE: c_int = 8;

pub const VND_CMD_APP_RESERVED_SIZE: c_int = 28;
pub const VND_CMD_PAD_SIZE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum auth_els_sub_cmd {
    SEND_ELS = 0,
    SEND_ELS_REPLY,
    PULL_ELS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct extra_auth_els {
    pub sub_cmd: auth_els_sub_cmd,
    pub extra_rx_xchg_address: u32,
    pub extra_control_flags: u8,
pub const BSG_CTL_FLAG_INIT: c_int = 0;
pub const BSG_CTL_FLAG_LS_ACC: c_int = 1;
pub const BSG_CTL_FLAG_LS_RJT: c_int = 2;
pub const BSG_CTL_FLAG_TRM: c_int = 3;
    pub version: u8,
    pub pad: [u8; 2],
    pub reserved: [u8; VND_CMD_APP_RESERVED_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_bsg_auth_els_request {
    pub r: fc_bsg_request,
    pub e: extra_auth_els,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_bsg_auth_els_reply {
    pub r: fc_bsg_reply,
    pub rx_xchg_address: u32,
    pub version: u8,
    pub pad: [u8; VND_CMD_PAD_SIZE],
    pub reserved: [u8; VND_CMD_APP_RESERVED_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct app_id {
    pub app_vid: c_int,
    pub version: u8,
    pub pad: [u8; VND_CMD_PAD_SIZE],
    pub reserved: [u8; VND_CMD_APP_RESERVED_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct app_start_reply {
    pub host_support_edif: u32,
    pub edif_enode_active: u32,
    pub edif_edb_active: u32,
    pub version: u8,
    pub pad: [u8; VND_CMD_PAD_SIZE],
    pub reserved: [u8; VND_CMD_APP_RESERVED_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct app_start {
    pub app_info: app_id,
    pub app_start_flags: u8,
    pub version: u8,
    pub pad: [u8; 2],
    pub reserved: [u8; VND_CMD_APP_RESERVED_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct app_stop {
    pub app_info: app_id,
    pub version: u8,
    pub pad: [u8; VND_CMD_PAD_SIZE],
    pub reserved: [u8; VND_CMD_APP_RESERVED_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct app_plogi_reply {
    pub prli_status: u32,
    pub version: u8,
    pub pad: [u8; VND_CMD_PAD_SIZE],
    pub reserved: [u8; VND_CMD_APP_RESERVED_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct app_pinfo_req {
    pub app_info: app_id,
    pub num_ports: u8,

    pub domain: u8,
    pub area: u8,
    pub al_pa: u8,

    pub al_pa: u8,
    pub area: u8,
    pub domain: u8,

    pub rsvd_1: u8,
    pub remote_pid: },
    pub version: u8,
    pub pad: [u8; VND_CMD_PAD_SIZE],
    pub reserved: [u8; VND_CMD_APP_RESERVED_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct app_pinfo {
    pub remote_pid: port_id_t,
    pub remote_wwpn: [u8; WWN_SIZE],
    pub remote_type: u8,
pub const VND_CMD_RTYPE_UNKNOWN: c_int = 0;
pub const VND_CMD_RTYPE_TARGET: c_int = 1;
pub const VND_CMD_RTYPE_INITIATOR: c_int = 2;
    pub remote_state: u8,
    pub auth_state: u8,
    pub version: u8,
    pub pad: [u8; VND_CMD_PAD_SIZE],
    pub reserved: [u8; VND_CMD_APP_RESERVED_SIZE],
    pub __packed: },
// AUTH States
pub const VND_CMD_AUTH_STATE_UNDEF: c_int = 0;
pub const VND_CMD_AUTH_STATE_SESSION_SHUTDOWN: c_int = 1;
pub const VND_CMD_AUTH_STATE_NEEDED: c_int = 2;
pub const VND_CMD_AUTH_STATE_ELS_RCVD: c_int = 3;
pub const VND_CMD_AUTH_STATE_SAUPDATE_COMPL: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct app_pinfo_reply {
    pub port_count: u8,
    pub version: u8,
    pub pad: [u8; VND_CMD_PAD_SIZE],
    pub reserved: [u8; VND_CMD_APP_RESERVED_SIZE],
    pub ports: [app_pinfo; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct app_sinfo_req {
    pub app_info: app_id,
    pub num_ports: u8,
    pub version: u8,
    pub pad: [u8; VND_CMD_PAD_SIZE],
    pub reserved: [u8; VND_CMD_APP_RESERVED_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct app_sinfo {
    pub remote_wwpn: [u8; WWN_SIZE],
    pub rekey_count: i64,
    pub rekey_mode: u8,
    pub tx_bytes: i64,
    pub rx_bytes: i64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct app_stats_reply {
    pub elem_count: u8,
    pub version: u8,
    pub pad: [u8; VND_CMD_PAD_SIZE],
    pub reserved: [u8; VND_CMD_APP_RESERVED_SIZE],
    pub elem: [app_sinfo; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_sa_update_frame {
    pub app_info: app_id,
    pub flags: u16,
pub const SAU_FLG_INV: c_uint = 0x01	/* delete key */;
pub const SAU_FLG_TX: c_uint = 0x02	/* 1=tx, 0 = rx */;
pub const SAU_FLG_FORCE_DELETE: c_uint = 0x08;
pub const SAU_FLG_GMAC_MODE: c_uint = 0x20	/*;
// GMAC mode is cleartext for the IO
// (i.e. NULL encryption)
//
pub const SAU_FLG_KEY128: c_uint = 0x40;
pub const SAU_FLG_KEY256: c_uint = 0x80;
    pub salt: u32,
    pub spi: u32,
    pub sa_key: [u8; 32],
    pub node_name: [u8; WWN_SIZE],
    pub port_name: [u8; WWN_SIZE],
    pub port_id: port_id_t,
    pub version: u8,
    pub pad: [u8; VND_CMD_PAD_SIZE],
    pub reserved2: [u8; VND_CMD_APP_RESERVED_SIZE],
    pub __packed: },
pub const QL_VND_SC_UNDEF: c_int = 0;
pub const QL_VND_SC_SA_UPDATE: c_int = 1;
pub const QL_VND_SC_APP_START: c_int = 2;
pub const QL_VND_SC_APP_STOP: c_int = 3;
pub const QL_VND_SC_AUTH_OK: c_int = 4;
pub const QL_VND_SC_AUTH_FAIL: c_int = 5;
pub const QL_VND_SC_REKEY_CONFIG: c_int = 6;
pub const QL_VND_SC_GET_FCINFO: c_int = 7;
pub const QL_VND_SC_GET_STATS: c_int = 8;
pub const QL_VND_SC_AEN_COMPLETE: c_int = 9;
pub const QL_VND_SC_READ_DBELL: c_int = 10;
//
// bsg caller to provide empty buffer for doorbell events.
//
// sg_io_v4.din_xferp  = empty buffer for door bell events
// sg_io_v4.dout_xferp = struct edif_read_dbell *buf
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edif_read_dbell {
    pub app_info: app_id,
    pub version: u8,
    pub pad: [u8; VND_CMD_PAD_SIZE],
    pub reserved: [u8; VND_CMD_APP_RESERVED_SIZE],
}

// Application interface data structure for rtn data
pub const EXT_DEF_EVENT_DATA_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edif_app_dbell {
    pub event_code: u32,
    pub event_data_size: u32,
    pub port_id: port_id_t,
    pub event_data: [u8; EXT_DEF_EVENT_DATA_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct edif_sa_update_aen {
    pub port_id: port_id_t,
    pub /: *mut *mut uint32_t key_type; / Tx (1) or RX (2),
    pub /: *mut *mut uint32_t status; / 0 succes, 1 failed, 2 timeout , 3 error,
    pub version: u8,
    pub pad: [u8; VND_CMD_PAD_SIZE],
    pub reserved: [u8; VND_CMD_APP_RESERVED_SIZE],
    pub __packed: },
pub const QL_VND_SA_STAT_SUCCESS: c_int = 0;
pub const QL_VND_SA_STAT_FAILED: c_int = 1;
pub const QL_VND_SA_STAT_TIMEOUT: c_int = 2;
pub const QL_VND_SA_STAT_ERROR: c_int = 3;
pub const QL_VND_RX_SA_KEY: c_int = 1;
pub const QL_VND_TX_SA_KEY: c_int = 2;
// App defines for plogi auth'd ok and plogi auth bad requests
#[repr(C)]
#[derive(Copy, Clone)]
pub struct auth_complete_cmd {
    pub app_info: app_id,
pub const PL_TYPE_WWPN: c_int = 1;
pub const PL_TYPE_DID: c_int = 2;
    pub type: u32,
    pub wwpn: [u8; WWN_SIZE],
    pub d_id: port_id_t,
    pub u: },
    pub version: u8,
    pub pad: [u8; VND_CMD_PAD_SIZE],
    pub reserved: [u8; VND_CMD_APP_RESERVED_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aen_complete_cmd {
    pub app_info: app_id,
    pub port_id: port_id_t,
    pub event_code: u32,
    pub version: u8,
    pub pad: [u8; VND_CMD_PAD_SIZE],
    pub reserved: [u8; VND_CMD_APP_RESERVED_SIZE],
    pub __packed: },
pub const RX_DELAY_DELETE_TIMEOUT: c_int = 20;
pub const FCH_EVT_VENDOR_UNIQUE_VPORT_DOWN: c_int = 1;
