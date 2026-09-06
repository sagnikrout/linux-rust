//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla2xxx/qla_mr.h
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

//
// The PCI VendorID and DeviceID for our board.
//
pub const PCI_DEVICE_ID_QLOGIC_ISPF001: c_uint = 0xF001;
// FX00 specific definitions
pub const FX00_COMMAND_TYPE_7: c_uint = 0x07	/* Command Type 7 entry for 7XXX */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_type_7_fx00 {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub reserved_0: u8,
    pub port_path_ctrl: u8,
    pub reserved_1: u16,
    pub /: *mut *mut __le16 tgt_idx; / Target Idx.,
    pub /: *mut *mut uint16_t timeout; / Command timeout.,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub scsi_rsp_dsd_len: u8,
    pub reserved_2: u8,
    pub /: *mut *mut scsi_lun lun; / LUN (LE).,
    pub cntrl_flags: u8,
    pub /: *mut *mut uint8_t task_mgmt_flags; / Task management flags.,
    pub task: u8,
    pub crn: u8,
    pub /: *mut *mut uint8_t fcp_cdb[MAX_CMDSZ]; / SCSI command words.,
    pub /: *mut *mut __le32 byte_count; / Total byte count.,
    pub dsd: dsd64,
}

pub const STATUS_TYPE_FX00: c_uint = 0x01		/* Status entry. */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sts_entry_fx00 {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut uint32_t reserved_3; / System handle.,
    pub /: *mut *mut __le16 comp_status; / Completion status.,
    pub /: *mut *mut uint16_t reserved_0; / OX_ID used by the firmware.,
    pub /: *mut *mut __le32 residual_len; / FW calc residual transfer length.,
    pub reserved_1: u16,
    pub /: *mut *mut uint16_t state_flags; / State flags.,
    pub reserved_2: u16,
    pub /: *mut *mut __le16 scsi_status; / SCSI status.,
    pub /: *mut *mut uint32_t sense_len; / FCP SENSE length.,
    pub /: *mut *mut uint8_t data[32]; / FCP response/sense information.,
}

pub const MAX_HANDLE_COUNT: c_int = 15;
pub const MULTI_STATUS_TYPE_FX00: c_uint = 0x0D;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct multi_sts_entry_fx00 {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub handle_count: u8,
    pub entry_status: u8,
    pub handles: [__le32; MAX_HANDLE_COUNT],
}

pub const TSK_MGMT_IOCB_TYPE_FX00: c_uint = 0x05;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsk_mgmt_entry_fx00 {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub sys_define: u8,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub reserved_0: u32,
    pub /: *mut *mut __le16 tgt_id; / Target Idx.,
    pub reserved_1: u16,
    pub reserved_3: u16,
    pub reserved_4: u16,
    pub /: *mut *mut scsi_lun lun; / LUN (LE).,
    pub /: *mut *mut __le32 control_flags; / Control Flags.,
    pub reserved_2: [u8; 32],
}

pub const ABORT_IOCB_TYPE_FX00: c_uint = 0x08		/* Abort IOCB status. */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct abort_iocb_entry_fx00 {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub reserved_0: __le32,
    pub /: *mut *mut __le16 tgt_id_sts; / Completion status.,
    pub options: __le16,
    pub /: *mut *mut uint32_t abort_handle; / System handle.,
    pub reserved_2: __le32,
    pub req_que_no: __le16,
    pub reserved_1: [u8; 38],
}

pub const IOCTL_IOSB_TYPE_FX00: c_uint = 0x0C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioctl_iocb_entry_fx00 {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut uint32_t reserved_0; / System handle.,
    pub comp_func_num: u16,
    pub fw_iotcl_flags: __le16,
    pub /: *mut *mut __le32 dataword_r; / Data word returned,
    pub /: *mut *mut uint32_t adapid; / Adapter ID,
    pub dataword_r_extra: u32,
    pub seq_no: __le32,
    pub reserved_2: [u8; 20],
    pub residuallen: u32,
    pub status: __le32,
}

pub const STATUS_CONT_TYPE_FX00: c_uint = 0x04;
pub const FX00_IOCB_TYPE: c_uint = 0x0B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fxdisc_entry_fx00 {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System Defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le32 reserved_0; / System handle.,
    pub func_num: __le16,
    pub req_xfrcnt: __le16,
    pub req_dsdcnt: __le16,
    pub rsp_xfrcnt: __le16,
    pub rsp_dsdcnt: __le16,
    pub flags: u8,
    pub reserved_1: u8,
//
// Use array size 1 below to prevent that Coverity complains about
// the append_dsd64() calls for the two arrays below.
//
    pub dseg_rq: [dsd64; 1],
    pub dseg_rsp: [dsd64; 1],
    pub dataword: __le32,
    pub adapid: __le32,
    pub adapid_hi: __le32,
    pub dataword_extra: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlafx00_tgt_node_info {
    pub tgt_node_wwpn: [u8; WWN_SIZE],
    pub tgt_node_wwnn: [u8; WWN_SIZE],
    pub tgt_node_state: u32,
    pub reserved: [u8; 128],
    pub reserved_1: [u32; 8],
    pub reserved_2: [u64; 4],
    pub __packed: },

pub const QLAFX00_LINK_STATUS_DOWN: c_uint = 0x10;
pub const QLAFX00_LINK_STATUS_UP: c_uint = 0x11;
pub const QLAFX00_PORT_SPEED_2G: c_uint = 0x2;
pub const QLAFX00_PORT_SPEED_4G: c_uint = 0x4;
pub const QLAFX00_PORT_SPEED_8G: c_uint = 0x8;
pub const QLAFX00_PORT_SPEED_10G: c_uint = 0xa;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_info_data {
    pub port_state: u8,
    pub port_type: u8,
    pub port_identifier: u16,
    pub up_port_state: u32,
    pub fw_ver_num: [u8; 32],
    pub portal_attrib: u8,
    pub host_option: u16,
    pub reset_delay: u8,
    pub pdwn_retry_cnt: u8,
    pub max_luns2tgt: u16,
    pub risc_ver: u8,
    pub pconn_option: u8,
    pub risc_option: u16,
    pub max_frame_len: u16,
    pub max_iocb_alloc: u16,
    pub exec_throttle: u16,
    pub retry_cnt: u8,
    pub retry_delay: u8,
    pub port_name: [u8; 8],
    pub port_id: [u8; 3],
    pub link_status: u8,
    pub plink_rate: u8,
    pub link_config: u32,
    pub adap_haddr: u16,
    pub tgt_disc: u8,
    pub log_tout: u8,
    pub node_name: [u8; 8],
    pub erisc_opt1: u16,
    pub resp_acc_tmr: u8,
    pub intr_del_tmr: u8,
    pub erisc_opt2: u8,
    pub alt_port_name: [u8; 8],
    pub alt_node_name: [u8; 8],
    pub link_down_tout: u8,
    pub conn_type: u8,
    pub fc_fw_mode: u8,
    pub uiReserved: [u32; 48],
    pub __packed: },
// OS Type Designations
pub const OS_TYPE_UNKNOWN: c_int = 0;
pub const OS_TYPE_LINUX: c_int = 2;
// Linux Info
pub const SYSNAME_LENGTH: c_int = 128;
pub const NODENAME_LENGTH: c_int = 64;
pub const RELEASE_LENGTH: c_int = 64;
pub const VERSION_LENGTH: c_int = 64;
pub const MACHINE_LENGTH: c_int = 64;
pub const DOMNAME_LENGTH: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_system_info {
    pub os_type: u32,
    pub sysname: [c_char; SYSNAME_LENGTH],
    pub nodename: [c_char; NODENAME_LENGTH],
    pub release: [c_char; RELEASE_LENGTH],
    pub version: [c_char; VERSION_LENGTH],
    pub machine: [c_char; MACHINE_LENGTH],
    pub domainname: [c_char; DOMNAME_LENGTH],
    pub hostdriver: [c_char; VERSION_LENGTH],
    pub reserved: [u32; 64],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct register_host_info {
    pub /: *mut *mut host_system_info hsi; / host system info,
    pub /: *mut *mut uint64_t utc; / UTC (system time),
    pub /: *mut *mut uint32_t reserved[64]; / future additions,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct config_info_data {
    pub __nonstring: uint8_t model_num[16],
    pub __nonstring: uint8_t model_description[80],
    pub reserved0: [u8; 160],
    pub symbolic_name: [u8; 64],
    pub serial_num: [u8; 32],
    pub hw_version: [u8; 16],
    pub fw_version: [u8; 16],
    pub uboot_version: [u8; 16],
    pub fru_serial_num: [u8; 32],
    pub fc_port_count: u8,
    pub iscsi_port_count: u8,
    pub reserved1: [u8; 2],
    pub mode: u8,
    pub log_level: u8,
    pub reserved2: [u8; 2],
    pub log_size: u32,
    pub tgt_pres_mode: u8,
    pub iqn_flags: u8,
    pub lun_mapping: u8,
    pub adapter_id: u64,
    pub cluster_key_len: u32,
    pub cluster_key: [u8; 16],
    pub cluster_master_id: u64,
    pub cluster_slave_id: u64,
    pub cluster_flags: u8,
    pub enabled_capabilities: u32,
    pub nominal_temp_value: u32,
    pub __packed: },
pub const FXDISC_GET_CONFIG_INFO: c_uint = 0x01;
pub const FXDISC_GET_PORT_INFO: c_uint = 0x02;
pub const FXDISC_GET_TGT_NODE_INFO: c_uint = 0x80;
pub const FXDISC_GET_TGT_NODE_LIST: c_uint = 0x81;
pub const FXDISC_REG_HOST_INFO: c_uint = 0x99;
pub const FXDISC_ABORT_IOCTL: c_uint = 0xff;
pub const QLAFX00_HBA_ICNTRL_REG: c_uint = 0x20B08;
pub const QLAFX00_ICR_ENB_MASK: c_uint = 0x80000000;
pub const QLAFX00_ICR_DIS_MASK: c_uint = 0x7fffffff;
pub const QLAFX00_HST_RST_REG: c_uint = 0x18264;
pub const QLAFX00_SOC_TEMP_REG: c_uint = 0x184C4;
pub const QLAFX00_HST_TO_HBA_REG: c_uint = 0x20A04;
pub const QLAFX00_HBA_TO_HOST_REG: c_uint = 0x21B70;
pub const QLAFX00_HST_INT_STS_BITS: c_uint = 0x7;
pub const QLAFX00_BAR1_BASE_ADDR_REG: c_uint = 0x40018;
pub const QLAFX00_PEX0_WIN0_BASE_ADDR_REG: c_uint = 0x41824;
pub const QLAFX00_INTR_MB_CMPLT: c_uint = 0x1;
pub const QLAFX00_INTR_RSP_CMPLT: c_uint = 0x2;
pub const QLAFX00_INTR_ASYNC_CMPLT: c_uint = 0x4;
pub const QLAFX00_MBA_SYSTEM_ERR: c_uint = 0x8002;
pub const QLAFX00_MBA_TEMP_OVER: c_uint = 0x8005;
pub const QLAFX00_MBA_TEMP_NORM: c_uint = 0x8006;
pub const QLAFX00_MBA_TEMP_CRIT: c_uint = 0x8007;
pub const QLAFX00_MBA_LINK_UP: c_uint = 0x8011;
pub const QLAFX00_MBA_LINK_DOWN: c_uint = 0x8012;
pub const QLAFX00_MBA_PORT_UPDATE: c_uint = 0x8014;
pub const QLAFX00_MBA_SHUTDOWN_RQSTD: c_uint = 0x8062;
pub const SOC_SW_RST_CONTROL_REG_CORE0: c_uint = 0x0020800;
pub const SOC_FABRIC_RST_CONTROL_REG: c_uint = 0x0020840;
pub const SOC_FABRIC_CONTROL_REG: c_uint = 0x0020200;
pub const SOC_FABRIC_CONFIG_REG: c_uint = 0x0020204;
pub const SOC_PWR_MANAGEMENT_PWR_DOWN_REG: c_uint = 0x001820C;
pub const SOC_INTERRUPT_SOURCE_I_CONTROL_REG: c_uint = 0x0020B00;
pub const SOC_CORE_TIMER_REG: c_uint = 0x0021850;
pub const SOC_IRQ_ACK_REG: c_uint = 0x00218b4;
pub const CONTINUE_A64_TYPE_FX00: c_uint = 0x03	/* Continuation entry. */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_mt_iocb_rqst_fx00 {
    pub reserved_0: __le32,
    pub func_type: __le16,
    pub flags: u8,
    pub reserved_1: u8,
    pub dataword: __le32,
    pub adapid: __le32,
    pub adapid_hi: __le32,
    pub dataword_extra: __le32,
    pub req_len: __le16,
    pub reserved_2: __le16,
    pub rsp_len: __le16,
    pub reserved_3: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_mt_iocb_rsp_fx00 {
    pub reserved_1: u32,
    pub func_type: u16,
    pub ioctl_flags: __le16,
    pub ioctl_data: __le32,
    pub adapid: u32,
    pub adapid_hi: u32,
    pub reserved_2: u32,
    pub seq_number: __le32,
    pub reserved_3: [u8; 20],
    pub res_count: i32,
    pub status: __le32,
}

pub const MAILBOX_REGISTER_COUNT_FX00: c_int = 16;
pub const AEN_MAILBOX_REGISTER_COUNT_FX00: c_int = 8;
pub const MAX_FIBRE_DEVICES_FX00: c_int = 512;
pub const MAX_LUNS_FX00: c_uint = 0x1024;

//
// Firmware state codes for QLAFX00 adapters
//
pub const FSTATE_FX00_CONFIG_WAIT: c_uint = 0x0000	/* Waiting for driver to issue;
// Initialize FW Mbox cmd
//
pub const FSTATE_FX00_INITIALIZED: c_uint = 0x1000	/* FW has been initialized by;
// the driver
//
pub const FX00_DEF_RATOV: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mr_data_fx00 {
    pub symbolic_name: [u8; 64],
    pub serial_num: [u8; 32],
    pub hw_version: [u8; 16],
    pub fw_version: [u8; 16],
    pub uboot_version: [u8; 16],
    pub fru_serial_num: [u8; 32],
    pub requests: *mut *mut fc_port_t fcport; / fcport used for,
// that are not linked
// to a particular target
//
    pub fw_hbt_en: u8,
    pub fw_hbt_cnt: u8,
    pub fw_hbt_miss_cnt: u8,
    pub old_fw_hbt_cnt: u32,
    pub fw_reset_timer_tick: u16,
    pub fw_reset_timer_exp: u8,
    pub fw_critemp_timer_tick: u16,
    pub old_aenmbx0_state: u32,
    pub critical_temperature: u32,
    pub extended_io_enabled: bool,
    pub host_info_resend: bool,
    pub hinfo_resend_timer_tick: u8,
}

pub const QLAFX00_EXTENDED_IO_EN_MASK: c_uint = 0x20;
//
// SoC Junction Temperature is stored in
// bits 9:1 of SoC Junction Temperature Register
// in a firmware specific format format.
// To get the temperature in Celsius degrees
// the value from this bitfiled should be converted
// using this formula:
// Temperature (degrees C) = ((3,153,000 - (10,000 * X)) / 13,825)
// where X is the bit field value
// this macro reads the register, extracts the bitfield value,
// performs the calcualtions and returns temperature in Celsius
//

// Max conncurrent IOs that can be queued
pub const QLAFX00_MAX_CANQUEUE: c_int = 1024;
// IOCTL IOCB abort success
pub const QLAFX00_IOCTL_ICOB_ABORT_SUCCESS: c_uint = 0x68;
