//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla2xxx/qla_bsg.h
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
// QLogic Fibre Channel HBA Driver
// Copyright (c)  2003-2014 QLogic Corporation
//
// BSG Vendor specific commands
pub const QL_VND_LOOPBACK: c_uint = 0x01;
pub const QL_VND_A84_RESET: c_uint = 0x02;
pub const QL_VND_A84_UPDATE_FW: c_uint = 0x03;
pub const QL_VND_A84_MGMT_CMD: c_uint = 0x04;
pub const QL_VND_IIDMA: c_uint = 0x05;
pub const QL_VND_FCP_PRIO_CFG_CMD: c_uint = 0x06;
pub const QL_VND_READ_FLASH: c_uint = 0x07;
pub const QL_VND_UPDATE_FLASH: c_uint = 0x08;
pub const QL_VND_SET_FRU_VERSION: c_uint = 0x0B;
pub const QL_VND_READ_FRU_STATUS: c_uint = 0x0C;
pub const QL_VND_WRITE_FRU_STATUS: c_uint = 0x0D;
pub const QL_VND_DIAG_IO_CMD: c_uint = 0x0A;
pub const QL_VND_WRITE_I2C: c_uint = 0x10;
pub const QL_VND_READ_I2C: c_uint = 0x11;
pub const QL_VND_FX00_MGMT_CMD: c_uint = 0x12;
pub const QL_VND_SERDES_OP: c_uint = 0x13;
pub const QL_VND_SERDES_OP_EX: c_uint = 0x14;
pub const QL_VND_GET_FLASH_UPDATE_CAPS: c_uint = 0x15;
pub const QL_VND_SET_FLASH_UPDATE_CAPS: c_uint = 0x16;
pub const QL_VND_GET_BBCR_DATA: c_uint = 0x17;
pub const QL_VND_GET_PRIV_STATS: c_uint = 0x18;
pub const QL_VND_DPORT_DIAGNOSTICS: c_uint = 0x19;
pub const QL_VND_GET_PRIV_STATS_EX: c_uint = 0x1A;
pub const QL_VND_SS_GET_FLASH_IMAGE_STATUS: c_uint = 0x1E;

pub const QL_VND_GET_DRV_ATTR: c_uint = 0x22;
pub const QL_VND_MANAGE_HOST_STATS: c_uint = 0x23;
pub const QL_VND_GET_HOST_STATS: c_uint = 0x24;
pub const QL_VND_GET_TGT_STATS: c_uint = 0x25;
pub const QL_VND_MANAGE_HOST_PORT: c_uint = 0x26;
pub const QL_VND_MBX_PASSTHRU: c_uint = 0x2B;
pub const QL_VND_DPORT_DIAGNOSTICS_V2: c_uint = 0x2C;
pub const QL_VND_IMG_SET_VALID: c_uint = 0x30;
pub const QL_VND_READ_FLASH_BLOCK: c_uint = 0x33;
pub const QL_VND_WRITE_FLASH_BLOCK: c_uint = 0x34;
pub const QL_VND_LOAD_MPI: c_uint = 0x35;
pub const QL_VND_DUMP_MPI: c_uint = 0x36;
// BSG Vendor specific subcode returns
pub const EXT_STATUS_OK: c_int = 0;
pub const EXT_STATUS_ERR: c_int = 1;
pub const EXT_STATUS_BUSY: c_int = 2;
pub const EXT_STATUS_INVALID_PARAM: c_int = 6;
pub const EXT_STATUS_DATA_OVERRUN: c_int = 7;
pub const EXT_STATUS_DATA_UNDERRUN: c_int = 8;
pub const EXT_STATUS_MAILBOX: c_int = 11;
pub const EXT_STATUS_BUFFER_TOO_SMALL: c_int = 16;
pub const EXT_STATUS_NO_MEMORY: c_int = 17;
pub const EXT_STATUS_DEVICE_OFFLINE: c_int = 22;
pub const EXT_STATUS_IMG_SET_VALID_ERR: c_int = 47;
pub const EXT_STATUS_IMG_SET_CONFIG_ERR: c_int = 48;
//
// To support bidirectional iocb
// BSG Vendor specific returns
//
pub const EXT_STATUS_NOT_SUPPORTED: c_int = 27;
pub const EXT_STATUS_INVALID_CFG: c_int = 28;
pub const EXT_STATUS_DMA_ERR: c_int = 29;
pub const EXT_STATUS_TIMEOUT: c_int = 30;
pub const EXT_STATUS_THREAD_FAILED: c_int = 31;
pub const EXT_STATUS_DATA_CMP_FAILED: c_int = 32;
pub const EXT_STATUS_DPORT_DIAG_ERR: c_int = 40;
pub const EXT_STATUS_DPORT_DIAG_IN_PROCESS: c_int = 41;
pub const EXT_STATUS_DPORT_DIAG_NOT_RUNNING: c_int = 42;
// BSG definations for interpreting CommandSent field
pub const INT_DEF_LB_LOOPBACK_CMD: c_int = 0;
pub const INT_DEF_LB_ECHO_CMD: c_int = 1;
// Loopback related definations
pub const INTERNAL_LOOPBACK: c_uint = 0xF1;
pub const EXTERNAL_LOOPBACK: c_uint = 0xF2;
pub const ENABLE_INTERNAL_LOOPBACK: c_uint = 0x02;
pub const ENABLE_EXTERNAL_LOOPBACK: c_uint = 0x04;
pub const INTERNAL_LOOPBACK_MASK: c_uint = 0x000E;
pub const MAX_ELS_FRAME_PAYLOAD: c_int = 252;
pub const ELS_OPCODE_BYTE: c_uint = 0x10;
// BSG Vendor specific definations
pub const QLA_IS_TIM: c_uint = 0x1;
pub const QLA_IS_SECURE: c_uint = 0x2;
pub const QLA_UPDATE_MBR: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_block_rw {
    pub region: u32,
    pub rw_length: u32,
    pub options: u32,
    pub region_offset: u32,
    pub chunk_length: u32,
    pub reserved: [u8; 44],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_load_dump_mpi {
    pub mpi_address: u32,
    pub length: u32,
    pub options: u32,
pub const QLA_LDM_SECURE_ENABLE: c_uint = 0x1;
pub const QLA_LDM_OTP_PROV: c_uint = 0x2;
pub const QLA_LDM_DEV_CSR: c_uint = 0x4;
pub const QLA_LDM_AUTH_CMD_BIN: c_uint = 0x8;
pub const QLA_LDM_SHADOW_REGS: c_uint = 0x10;
pub const QLA_LDM_CA_CSR: c_uint = 0x20;
pub const QLA_LDM_MLDSA_ALGO: c_uint = 0x40;
pub const QLA_LDM_MLDSA_SIGNATURE: c_uint = 0x80;
    pub reserved: [u8; 52],
    pub __packed: },
pub const A84_ISSUE_WRITE_TYPE_CMD: c_int = 0;
pub const A84_ISSUE_READ_TYPE_CMD: c_int = 1;
pub const A84_CLEANUP_CMD: c_int = 2;
pub const A84_ISSUE_RESET_OP_FW: c_int = 3;
pub const A84_ISSUE_RESET_DIAG_FW: c_int = 4;
pub const A84_ISSUE_UPDATE_OPFW_CMD: c_int = 5;
pub const A84_ISSUE_UPDATE_DIAGFW_CMD: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla84_mgmt_param {
    pub start_addr: u32,
    pub /: *mut *mut } mem; / for QLA84_MGMT_READ/WRITE_MEM,
    pub id: u32,
pub const QLA84_MGMT_CONFIG_ID_UIF: c_int = 1;
pub const QLA84_MGMT_CONFIG_ID_FCOE_COS: c_int = 2;
pub const QLA84_MGMT_CONFIG_ID_PAUSE: c_int = 3;
pub const QLA84_MGMT_CONFIG_ID_TIMEOUTS: c_int = 4;
    pub param0: u32,
    pub param1: u32,
    pub /: *mut *mut } config; / for QLA84_MGMT_CHNG_CONFIG,
    pub type: u32,

    pub context: u32,
//
// context definitions for QLA84_MGMT_INFO_CONFIG_LOG_DATA
//
pub const IC_LOG_DATA_LOG_ID_DEBUG_LOG: c_int = 0;
pub const IC_LOG_DATA_LOG_ID_LEARN_LOG: c_int = 1;
pub const IC_LOG_DATA_LOG_ID_FC_ACL_INGRESS_LOG: c_int = 2;
pub const IC_LOG_DATA_LOG_ID_FC_ACL_EGRESS_LOG: c_int = 3;
pub const IC_LOG_DATA_LOG_ID_ETHERNET_ACL_INGRESS_LOG: c_int = 4;
pub const IC_LOG_DATA_LOG_ID_ETHERNET_ACL_EGRESS_LOG: c_int = 5;
pub const IC_LOG_DATA_LOG_ID_MESSAGE_TRANSMIT_LOG: c_int = 6;
pub const IC_LOG_DATA_LOG_ID_MESSAGE_RECEIVE_LOG: c_int = 7;
pub const IC_LOG_DATA_LOG_ID_LINK_EVENT_LOG: c_int = 8;
pub const IC_LOG_DATA_LOG_ID_DCX_LOG: c_int = 9;
//
// context definitions for QLA84_MGMT_INFO_PORT_STAT
//
pub const IC_PORT_STATISTICS_PORT_NUMBER_ETHERNET_PORT0: c_int = 0;
pub const IC_PORT_STATISTICS_PORT_NUMBER_ETHERNET_PORT1: c_int = 1;
pub const IC_PORT_STATISTICS_PORT_NUMBER_NSL_PORT0: c_int = 2;
pub const IC_PORT_STATISTICS_PORT_NUMBER_NSL_PORT1: c_int = 3;
pub const IC_PORT_STATISTICS_PORT_NUMBER_FC_PORT0: c_int = 4;
pub const IC_PORT_STATISTICS_PORT_NUMBER_FC_PORT1: c_int = 5;
//
// context definitions for QLA84_MGMT_INFO_LIF_STAT
//
pub const IC_LIF_STATISTICS_LIF_NUMBER_ETHERNET_PORT0: c_int = 0;
pub const IC_LIF_STATISTICS_LIF_NUMBER_ETHERNET_PORT1: c_int = 1;
pub const IC_LIF_STATISTICS_LIF_NUMBER_FC_PORT0: c_int = 2;
pub const IC_LIF_STATISTICS_LIF_NUMBER_FC_PORT1: c_int = 3;
pub const IC_LIF_STATISTICS_LIF_NUMBER_CPU: c_int = 6;
    pub /: *mut *mut } info; / for QLA84_MGMT_GET_INFO,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla84_msg_mgmt {
    pub cmd: u16,
pub const QLA84_MGMT_READ_MEM: c_uint = 0x00;
pub const QLA84_MGMT_WRITE_MEM: c_uint = 0x01;
pub const QLA84_MGMT_CHNG_CONFIG: c_uint = 0x02;
pub const QLA84_MGMT_GET_INFO: c_uint = 0x03;
    pub rsrvd: u16,
    pub /: *mut *mut qla84_mgmt_param mgmtp;/ parameters for cmd,
    pub /: *mut *mut uint32_t len; / bytes in payload following this struct,
    pub /: *mut *mut uint8_t payload[]; / payload for cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_bsg_a84_mgmt {
    pub mgmt: qla84_msg_mgmt,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_scsi_addr {
    pub bus: u16,
    pub target: u16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_ext_dest_addr {
    pub wwnn: [u8; 8],
    pub wwpn: [u8; 8],
    pub id: [u8; 4],
    pub scsi_addr: qla_scsi_addr,
    pub dest_addr: },
    pub dest_type: u16,
pub const EXT_DEF_TYPE_WWPN: c_int = 2;
    pub lun: u16,
    pub padding: [u16; 2],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_port_param {
    pub fc_scsi_addr: qla_ext_dest_addr,
    pub mode: u16,
    pub speed: u16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_mbx_passthru {
    pub reserved1: [u16; 2],
    pub mbx_in: [u16; 32],
    pub mbx_out: [u16; 32],
    pub reserved2: [u32; 16],
    pub __packed: },
// FRU VPD
pub const MAX_FRU_SIZE: c_int = 36;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_field_address {
    pub offset: u16,
    pub device: u16,
    pub option: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_field_info {
    pub version: [u8; MAX_FRU_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_image_version {
    pub field_address: qla_field_address,
    pub field_info: qla_field_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_image_version_list {
    pub count: u32,
    pub version: [qla_image_version; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_status_reg {
    pub field_address: qla_field_address,
    pub status_reg: u8,
    pub reserved: [u8; 7],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_i2c_access {
    pub device: u16,
    pub offset: u16,
    pub option: u16,
    pub length: u16,
    pub buffer: [u8; 0x40],
    pub __packed: },
// 26xx serdes register interface
// serdes reg commands
pub const INT_SC_SERDES_READ_REG: c_int = 1;
pub const INT_SC_SERDES_WRITE_REG: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_serdes_reg {
    pub cmd: u16,
    pub addr: u16,
    pub val: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_serdes_reg_ex {
    pub cmd: u16,
    pub addr: u32,
    pub val: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_flash_update_caps {
    pub capabilities: u64,
    pub outage_duration: u32,
    pub reserved: [u8; 20],
    pub __packed: },
// BB_CR Status
pub const QLA_BBCR_STATUS_DISABLED: c_int = 0;
pub const QLA_BBCR_STATUS_ENABLED: c_int = 1;
pub const QLA_BBCR_STATUS_UNKNOWN: c_int = 2;
// BB_CR State
pub const QLA_BBCR_STATE_OFFLINE: c_int = 0;
pub const QLA_BBCR_STATE_ONLINE: c_int = 1;
// BB_CR Offline Reason Code
pub const QLA_BBCR_REASON_PORT_SPEED: c_int = 1;
pub const QLA_BBCR_REASON_PEER_PORT: c_int = 2;
pub const QLA_BBCR_REASON_SWITCH: c_int = 3;
pub const QLA_BBCR_REASON_LOGIN_REJECT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_bbcr_data {
    pub /: *mut *mut uint8_t status; / 1 - enabled, 0 - Disabled,
    pub /: *mut *mut uint8_t state; / 1 - online, 0 - offline,
    pub /: *mut *mut uint8_t configured_bbscn; / 0-15,
    pub /: *mut *mut uint8_t negotiated_bbscn; / 0-15,
    pub offline_reason_code: u8,
    pub /: *mut *mut uint16_t mbx1; / Port state,
    pub reserved: [u8; 9],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_dport_diag {
    pub options: u16,
    pub buf: [u32; 16],
    pub unused: [u8; 62],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_dport_diag_v2 {
    pub options: u16,
    pub mbx1: u16,
    pub mbx2: u16,
    pub unused: [u8; 58],
    pub /: *mut *mut uint8_t buf[1024]; / Test Result,
    pub __packed: },
// D_Port options
pub const QLA_DPORT_RESULT: c_uint = 0x0;
pub const QLA_DPORT_START: c_uint = 0x2;
// active images in flash
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_active_regions {
    pub global_image: u8,
    pub board_config: u8,
    pub vpd_nvram: u8,
    pub npiv_config_0_1: u8,
    pub npiv_config_2_3: u8,
    pub nvme_params: u8,
    pub reserved: [u8; 31],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_drv_attr {
    pub attributes: u32,
    pub ext_attributes: u32,

    pub status_flags: u32,
    pub reserved: [u8; 20],
    pub __packed: },

