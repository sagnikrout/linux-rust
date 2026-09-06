//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/ipr.h
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
// ipr.h -- driver for IBM Power Linux RAID adapters
//
// Written By: Brian King <brking@us.ibm.com>, IBM Corporation
//
// Copyright (C) 2003, 2004 IBM Corporation
//
// Alan Cox <alan@lxorguk.ukuu.org.uk> - Removed several careless u32/dma_addr_t errors
// that broke 64bit platforms.
//

//
// Literals
//

//
// IPR_MAX_CMD_PER_LUN: This defines the maximum number of outstanding
// ops per device for devices not running tagged command queuing.
// This can be adjusted at runtime through sysfs device attributes.
//
pub const IPR_MAX_CMD_PER_LUN: c_int = 6;
//
// IPR_NUM_BASE_CMD_BLKS: This defines the maximum number of
// ops the mid-layer can send to the adapter.
//

pub const PCI_DEVICE_ID_IBM_OBSIDIAN_E: c_uint = 0x0339;
pub const PCI_DEVICE_ID_IBM_CROC_FPGA_E2: c_uint = 0x033D;
pub const PCI_DEVICE_ID_IBM_CROCODILE: c_uint = 0x034A;
pub const PCI_DEVICE_ID_IBM_RATTLESNAKE: c_uint = 0x04DA;
pub const IPR_SUBS_DEV_ID_2780: c_uint = 0x0264;
pub const IPR_SUBS_DEV_ID_5702: c_uint = 0x0266;
pub const IPR_SUBS_DEV_ID_5703: c_uint = 0x0278;
pub const IPR_SUBS_DEV_ID_572E: c_uint = 0x028D;
pub const IPR_SUBS_DEV_ID_573E: c_uint = 0x02D3;
pub const IPR_SUBS_DEV_ID_573D: c_uint = 0x02D4;
pub const IPR_SUBS_DEV_ID_571A: c_uint = 0x02C0;
pub const IPR_SUBS_DEV_ID_571B: c_uint = 0x02BE;
pub const IPR_SUBS_DEV_ID_571E: c_uint = 0x02BF;
pub const IPR_SUBS_DEV_ID_571F: c_uint = 0x02D5;
pub const IPR_SUBS_DEV_ID_572A: c_uint = 0x02C1;
pub const IPR_SUBS_DEV_ID_572B: c_uint = 0x02C2;
pub const IPR_SUBS_DEV_ID_572F: c_uint = 0x02C3;
pub const IPR_SUBS_DEV_ID_574E: c_uint = 0x030A;
pub const IPR_SUBS_DEV_ID_575B: c_uint = 0x030D;
pub const IPR_SUBS_DEV_ID_575C: c_uint = 0x0338;
pub const IPR_SUBS_DEV_ID_57B3: c_uint = 0x033A;
pub const IPR_SUBS_DEV_ID_57B7: c_uint = 0x0360;
pub const IPR_SUBS_DEV_ID_57B8: c_uint = 0x02C2;
pub const IPR_SUBS_DEV_ID_57B4: c_uint = 0x033B;
pub const IPR_SUBS_DEV_ID_57B2: c_uint = 0x035F;
pub const IPR_SUBS_DEV_ID_57C0: c_uint = 0x0352;
pub const IPR_SUBS_DEV_ID_57C3: c_uint = 0x0353;
pub const IPR_SUBS_DEV_ID_57C4: c_uint = 0x0354;
pub const IPR_SUBS_DEV_ID_57C6: c_uint = 0x0357;
pub const IPR_SUBS_DEV_ID_57CC: c_uint = 0x035C;
pub const IPR_SUBS_DEV_ID_57B5: c_uint = 0x033C;
pub const IPR_SUBS_DEV_ID_57CE: c_uint = 0x035E;
pub const IPR_SUBS_DEV_ID_57B1: c_uint = 0x0355;
pub const IPR_SUBS_DEV_ID_574D: c_uint = 0x0356;
pub const IPR_SUBS_DEV_ID_57C8: c_uint = 0x035D;
pub const IPR_SUBS_DEV_ID_57D5: c_uint = 0x03FB;
pub const IPR_SUBS_DEV_ID_57D6: c_uint = 0x03FC;
pub const IPR_SUBS_DEV_ID_57D7: c_uint = 0x03FF;
pub const IPR_SUBS_DEV_ID_57D8: c_uint = 0x03FE;
pub const IPR_SUBS_DEV_ID_57D9: c_uint = 0x046D;
pub const IPR_SUBS_DEV_ID_57DA: c_uint = 0x04CA;
pub const IPR_SUBS_DEV_ID_57EB: c_uint = 0x0474;
pub const IPR_SUBS_DEV_ID_57EC: c_uint = 0x0475;
pub const IPR_SUBS_DEV_ID_57ED: c_uint = 0x0499;
pub const IPR_SUBS_DEV_ID_57EE: c_uint = 0x049A;
pub const IPR_SUBS_DEV_ID_57EF: c_uint = 0x049B;
pub const IPR_SUBS_DEV_ID_57F0: c_uint = 0x049C;
pub const IPR_SUBS_DEV_ID_2CCA: c_uint = 0x04C7;
pub const IPR_SUBS_DEV_ID_2CD2: c_uint = 0x04C8;
pub const IPR_SUBS_DEV_ID_2CCD: c_uint = 0x04C9;
pub const IPR_SUBS_DEV_ID_580A: c_uint = 0x04FC;
pub const IPR_SUBS_DEV_ID_580B: c_uint = 0x04FB;

//
// Return codes
//
pub const IPR_RC_JOB_CONTINUE: c_int = 1;
pub const IPR_RC_JOB_RETURN: c_int = 2;
//
// IOASCs
//
pub const IPR_IOASC_NR_INIT_CMD_REQUIRED: c_uint = 0x02040200;
pub const IPR_IOASC_NR_IOA_RESET_REQUIRED: c_uint = 0x02048000;
pub const IPR_IOASC_SYNC_REQUIRED: c_uint = 0x023f0000;
pub const IPR_IOASC_MED_DO_NOT_REALLOC: c_uint = 0x03110C00;
pub const IPR_IOASC_HW_SEL_TIMEOUT: c_uint = 0x04050000;
pub const IPR_IOASC_HW_DEV_BUS_STATUS: c_uint = 0x04448500;
pub const IPR_IOASC_IOASC_MASK: c_uint = 0xFFFFFF00;
pub const IPR_IOASC_SCSI_STATUS_MASK: c_uint = 0x000000FF;
pub const IPR_IOASC_HW_CMD_FAILED: c_uint = 0x046E0000;
pub const IPR_IOASC_IR_INVALID_REQ_TYPE_OR_PKT: c_uint = 0x05240000;
pub const IPR_IOASC_IR_RESOURCE_HANDLE: c_uint = 0x05250000;
pub const IPR_IOASC_IR_NO_CMDS_TO_2ND_IOA: c_uint = 0x05258100;
pub const IPR_IOASA_IR_DUAL_IOA_DISABLED: c_uint = 0x052C8000;
pub const IPR_IOASC_BUS_WAS_RESET: c_uint = 0x06290000;
pub const IPR_IOASC_BUS_WAS_RESET_BY_OTHER: c_uint = 0x06298000;
pub const IPR_IOASC_ABORTED_CMD_TERM_BY_HOST: c_uint = 0x0B5A0000;
pub const IPR_IOASC_IR_NON_OPTIMIZED: c_uint = 0x05258200;
pub const IPR_FIRST_DRIVER_IOASC: c_uint = 0x10000000;
pub const IPR_IOASC_IOA_WAS_RESET: c_uint = 0x10000001;
pub const IPR_IOASC_PCI_ACCESS_ERROR: c_uint = 0x10000002;
// Driver data flags
pub const IPR_USE_LONG_TRANSOP_TIMEOUT: c_uint = 0x00000001;
pub const IPR_USE_PCI_WARM_RESET: c_uint = 0x00000002;
pub const IPR_DEFAULT_MAX_ERROR_DUMP: c_int = 984;
pub const IPR_NUM_LOG_HCAMS: c_int = 2;
pub const IPR_NUM_CFG_CHG_HCAMS: c_int = 2;
pub const IPR_NUM_HCAM_QUEUE: c_int = 12;

pub const IPR_MAX_SIS64_TARGETS_PER_BUS: c_int = 1024;
pub const IPR_MAX_SIS64_LUNS_PER_TARGET: c_uint = 0xffffffff;
pub const IPR_MAX_NUM_TARGETS_PER_BUS: c_int = 256;
pub const IPR_MAX_NUM_LUNS_PER_TARGET: c_int = 256;
pub const IPR_VSET_BUS: c_uint = 0xff;
pub const IPR_IOA_BUS: c_uint = 0xff;
pub const IPR_IOA_TARGET: c_uint = 0xff;
pub const IPR_IOA_LUN: c_uint = 0xff;
pub const IPR_MAX_NUM_BUSES: c_int = 16;
pub const IPR_NUM_RESET_RELOAD_RETRIES: c_int = 3;
// We need resources for HCAMS, IOA reset, IOA bringdown, and ERP

pub const IPR_MAX_COMMANDS: c_int = 100;

pub const IPR_MAX_PHYSICAL_DEVS: c_int = 192;
pub const IPR_DEFAULT_SIS64_DEVS: c_int = 1024;
pub const IPR_MAX_SIS64_DEVS: c_int = 4096;
pub const IPR_MAX_SGLIST: c_int = 64;
pub const IPR_IOA_MAX_SECTORS: c_int = 32767;
pub const IPR_VSET_MAX_SECTORS: c_int = 512;
pub const IPR_MAX_CDB_LEN: c_int = 16;
pub const IPR_MAX_HRRQ_RETRIES: c_int = 3;
pub const IPR_DEFAULT_BUS_WIDTH: c_int = 16;

pub const IPR_IOA_RES_HANDLE: c_uint = 0xffffffff;
pub const IPR_INVALID_RES_HANDLE: c_int = 0;
pub const IPR_IOA_RES_ADDR: c_uint = 0x00ffffff;
//
// Adapter Commands
//
pub const IPR_CANCEL_REQUEST: c_uint = 0xC0;
pub const IPR_CANCEL_64BIT_IOARCB: c_uint = 0x01;
pub const IPR_QUERY_RSRC_STATE: c_uint = 0xC2;
pub const IPR_RESET_DEVICE: c_uint = 0xC3;
pub const IPR_RESET_TYPE_SELECT: c_uint = 0x80;
pub const IPR_LUN_RESET: c_uint = 0x40;
pub const IPR_TARGET_RESET: c_uint = 0x20;
pub const IPR_BUS_RESET: c_uint = 0x10;
pub const IPR_ID_HOST_RR_Q: c_uint = 0xC4;
pub const IPR_QUERY_IOA_CONFIG: c_uint = 0xC5;
pub const IPR_CANCEL_ALL_REQUESTS: c_uint = 0xCE;
pub const IPR_HOST_CONTROLLED_ASYNC: c_uint = 0xCF;
pub const IPR_HCAM_CDB_OP_CODE_CONFIG_CHANGE: c_uint = 0x01;
pub const IPR_HCAM_CDB_OP_CODE_LOG_DATA: c_uint = 0x02;
pub const IPR_SET_SUPPORTED_DEVICES: c_uint = 0xFB;
pub const IPR_SET_ALL_SUPPORTED_DEVICES: c_uint = 0x80;
pub const IPR_IOA_SHUTDOWN: c_uint = 0xF7;
pub const IPR_WR_BUF_DOWNLOAD_AND_SAVE: c_uint = 0x05;
pub const IPR_IOA_SERVICE_ACTION: c_uint = 0xD2;
// IOA Service Actions
pub const IPR_IOA_SA_CHANGE_CACHE_PARAMS: c_uint = 0x14;
//
// Timeouts
//

pub const IPR_DUMP_DELAY_SECONDS: c_int = 4;

//
// SCSI Literals
//
pub const IPR_VENDOR_ID_LEN: c_int = 8;
pub const IPR_PROD_ID_LEN: c_int = 16;
pub const IPR_SERIAL_NUM_LEN: c_int = 8;
//
// Hardware literals
//
pub const IPR_FMT2_MBX_ADDR_MASK: c_uint = 0x0fffffff;
pub const IPR_FMT2_MBX_BAR_SEL_MASK: c_uint = 0xf0000000;
pub const IPR_FMT2_MKR_BAR_SEL_SHIFT: c_int = 28;

pub const IPR_SDT_FMT2_BAR0_SEL: c_uint = 0x0;
pub const IPR_SDT_FMT2_BAR1_SEL: c_uint = 0x1;
pub const IPR_SDT_FMT2_BAR2_SEL: c_uint = 0x2;
pub const IPR_SDT_FMT2_BAR3_SEL: c_uint = 0x3;
pub const IPR_SDT_FMT2_BAR4_SEL: c_uint = 0x4;
pub const IPR_SDT_FMT2_BAR5_SEL: c_uint = 0x5;
pub const IPR_SDT_FMT2_EXP_ROM_SEL: c_uint = 0x8;
pub const IPR_FMT2_SDT_READY_TO_USE: c_uint = 0xC4D4E3F2;
pub const IPR_FMT3_SDT_READY_TO_USE: c_uint = 0xC4D4E3F3;
pub const IPR_DOORBELL: c_uint = 0x82800000;
pub const IPR_RUNTIME_RESET: c_uint = 0x40000000;
pub const IPR_IPL_INIT_MIN_STAGE_TIME: c_int = 5;
pub const IPR_IPL_INIT_DEFAULT_STAGE_TIME: c_int = 30;
pub const IPR_IPL_INIT_STAGE_UNKNOWN: c_uint = 0x0;
pub const IPR_IPL_INIT_STAGE_TRANSOP: c_uint = 0xB0000000;
pub const IPR_IPL_INIT_STAGE_MASK: c_uint = 0xff000000;
pub const IPR_IPL_INIT_STAGE_TIME_MASK: c_uint = 0x0000ffff;

//
// Dump literals
//

pub const IPR_FMT2_NUM_SDT_ENTRIES: c_int = 511;
pub const IPR_FMT3_NUM_SDT_ENTRIES: c_uint = 0xFFF;

//
// Misc literals
//

pub const IPR_MAX_MSIX_VECTORS: c_uint = 0x10;
pub const IPR_MAX_HRRQ_NUM: c_uint = 0x10;
pub const IPR_INIT_HRRQ: c_uint = 0x0;
//
// Adapter interface types
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_res_addr {
    pub reserved: u8,
    pub bus: u8,
    pub target: u8,
    pub lun: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_std_inq_vpids {
    pub vendor_id: [u8; IPR_VENDOR_ID_LEN],
    pub product_id: [u8; IPR_PROD_ID_LEN],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_vpd {
    pub vpids: ipr_std_inq_vpids,
    pub sn: [u8; IPR_SERIAL_NUM_LEN],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_ext_vpd {
    pub vpd: ipr_vpd,
    pub wwid: [__be32; 2],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_ext_vpd64 {
    pub vpd: ipr_vpd,
    pub wwid: [__be32; 4],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_std_inq_data {
    pub peri_qual_dev_type: u8,

    pub removeable_medium_rsvd: u8,
pub const IPR_STD_INQ_REMOVEABLE_MEDIUM: c_uint = 0x80;

    pub version: u8,
    pub aen_naca_fmt: u8,
    pub additional_len: u8,
    pub sccs_rsvd: u8,
    pub bq_enc_multi: u8,
    pub sync_cmdq_flags: u8,
    pub vpids: ipr_std_inq_vpids,
    pub ros_rsvd_ram_rsvd: [u8; 4],
    pub serial_num: [u8; IPR_SERIAL_NUM_LEN],
// C attribute field omitted
pub const IPR_RES_TYPE_AF_DASD: c_uint = 0x00;
pub const IPR_RES_TYPE_GENERIC_SCSI: c_uint = 0x01;
pub const IPR_RES_TYPE_VOLUME_SET: c_uint = 0x02;
pub const IPR_RES_TYPE_REMOTE_AF_DASD: c_uint = 0x03;
pub const IPR_RES_TYPE_GENERIC_ATA: c_uint = 0x04;
pub const IPR_RES_TYPE_ARRAY: c_uint = 0x05;
pub const IPR_RES_TYPE_IOAFP: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_config_table_entry {
    pub proto: u8,
pub const IPR_PROTO_SATA: c_uint = 0x02;
pub const IPR_PROTO_SATA_ATAPI: c_uint = 0x03;
pub const IPR_PROTO_SAS_STP: c_uint = 0x06;
pub const IPR_PROTO_SAS_STP_ATAPI: c_uint = 0x07;
    pub array_id: u8,
    pub flags: u8,
pub const IPR_IS_IOA_RESOURCE: c_uint = 0x80;
    pub rsvd_subtype: u8,

pub const IPR_QUEUE_FROZEN_MODEL: c_int = 0;
pub const IPR_QUEUE_NACA_MODEL: c_int = 1;
    pub res_addr: ipr_res_addr,
    pub res_handle: __be32,
    pub lun_wwn: [__be32; 2],
    pub std_inq_data: ipr_std_inq_data,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_config_table_entry64 {
    pub res_type: u8,
    pub proto: u8,
    pub vset_num: u8,
    pub array_id: u8,
    pub flags: __be16,
    pub res_flags: __be16,

    pub res_handle: __be32,
    pub dev_id_type: u8,
    pub reserved: [u8; 3],
    pub dev_id: __be64,
    pub lun: __be64,
    pub lun_wwn: [__be64; 2],
pub const IPR_MAX_RES_PATH_LENGTH: c_int = 48;
pub const IPR_RES_PATH_BYTES: c_int = 8;
    pub res_path: __be64,
    pub std_inq_data: ipr_std_inq_data,
    pub reserved2: [u8; 4],
    pub reserved3: [__be64; 2],
    pub reserved4: [u8; 8],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_config_table_hdr {
    pub num_entries: u8,
    pub flags: u8,
pub const IPR_UCODE_DOWNLOAD_REQ: c_uint = 0x10;
    pub reserved: __be16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_config_table_hdr64 {
    pub num_entries: __be16,
    pub reserved: __be16,
    pub flags: u8,
    pub reserved2: [u8; 11],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_config_table {
    pub hdr: ipr_config_table_hdr,
    pub dev: [ipr_config_table_entry; ],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_config_table64 {
    pub hdr64: ipr_config_table_hdr64,
    pub dev: [ipr_config_table_entry64; ],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_config_table_entry_wrapper {
    pub cfgte: *mut ipr_config_table_entry,
    pub cfgte64: *mut ipr_config_table_entry64,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_cfg_ch_not {
    pub cfgte: ipr_config_table_entry,
    pub cfgte64: ipr_config_table_entry64,
    pub u: },
    pub reserved: [u8; 936],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_supported_device {
    pub data_length: __be16,
    pub reserved: u8,
    pub num_records: u8,
    pub vpids: ipr_std_inq_vpids,
    pub reserved2: [u8; 16],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hrr_queue {
    pub ioa_cfg: *mut ipr_ioa_cfg,
    pub host_rrq: *mut __be32,
    pub host_rrq_dma: dma_addr_t,
pub const IPR_HRRQ_REQ_RESP_HANDLE_MASK: c_uint = 0xfffffffc;
pub const IPR_HRRQ_RESP_BIT_SET: c_uint = 0x00000002;
pub const IPR_HRRQ_TOGGLE_BIT: c_uint = 0x00000001;
pub const IPR_HRRQ_REQ_RESP_HANDLE_SHIFT: c_int = 2;
pub const IPR_ID_HRRQ_SELE_ENABLE: c_uint = 0x02;
    pub hrrq_start: *mut volatile __be32,
    pub hrrq_end: *mut volatile __be32,
    pub hrrq_curr: *mut volatile __be32,
    pub hrrq_free_q: list_head,
    pub hrrq_pending_q: list_head,
    pub _lock: spinlock_t,
    pub lock: *mut spinlock_t,
    pub toggle_bit: volatile u32,
    pub size: u32,
    pub min_cmd_id: u32,
    pub max_cmd_id: u32,
    pub allow_interrupts:1: u8,
    pub ioa_is_dead:1: u8,
    pub allow_cmds:1: u8,
    pub removing_ioa:1: u8,
    pub iopoll: irq_poll,
}

// Command packet structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_cmd_pkt {
    pub /: *mut *mut u8 reserved; / Reserved by IOA,
    pub hrrq_id: u8,
    pub request_type: u8,
pub const IPR_RQTYPE_SCSICDB: c_uint = 0x00;
pub const IPR_RQTYPE_IOACMD: c_uint = 0x01;
pub const IPR_RQTYPE_HCAM: c_uint = 0x02;
pub const IPR_RQTYPE_PIPE: c_uint = 0x05;
    pub reserved2: u8,
    pub flags_hi: u8,
pub const IPR_FLAGS_HI_WRITE_NOT_READ: c_uint = 0x80;
pub const IPR_FLAGS_HI_NO_ULEN_CHK: c_uint = 0x20;
pub const IPR_FLAGS_HI_SYNC_OVERRIDE: c_uint = 0x10;
pub const IPR_FLAGS_HI_SYNC_COMPLETE: c_uint = 0x08;
pub const IPR_FLAGS_HI_NO_LINK_DESC: c_uint = 0x04;
    pub flags_lo: u8,
pub const IPR_FLAGS_LO_ALIGNED_BFR: c_uint = 0x20;
pub const IPR_FLAGS_LO_DELAY_AFTER_RST: c_uint = 0x10;
pub const IPR_FLAGS_LO_UNTAGGED_TASK: c_uint = 0x00;
pub const IPR_FLAGS_LO_SIMPLE_TASK: c_uint = 0x02;
pub const IPR_FLAGS_LO_ORDERED_TASK: c_uint = 0x04;
pub const IPR_FLAGS_LO_HEAD_OF_Q_TASK: c_uint = 0x06;
pub const IPR_FLAGS_LO_ACA_TASK: c_uint = 0x08;
    pub cdb: [u8; 16],
    pub timeout: __be16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_ioadl_desc {
    pub flags_and_data_len: __be32,
pub const IPR_IOADL_FLAGS_MASK: c_uint = 0xff000000;

pub const IPR_IOADL_DATA_LEN_MASK: c_uint = 0x00ffffff;

pub const IPR_IOADL_FLAGS_READ: c_uint = 0x48000000;
pub const IPR_IOADL_FLAGS_READ_LAST: c_uint = 0x49000000;
pub const IPR_IOADL_FLAGS_WRITE: c_uint = 0x68000000;
pub const IPR_IOADL_FLAGS_WRITE_LAST: c_uint = 0x69000000;
pub const IPR_IOADL_FLAGS_LAST: c_uint = 0x01000000;
    pub address: __be32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_ioadl64_desc {
    pub flags: __be32,
    pub data_len: __be32,
    pub address: __be64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_ioarcb_add_data {
    pub ioadl: [ipr_ioadl_desc; 5],
    pub add_cmd_parms: [__be32; 10],
    pub u: },
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_ioarcb_sis64_add_addr_ecb {
    pub ioasa_host_pci_addr: __be64,
    pub data_ioadl_addr: __be64,
    pub reserved: __be64,
    pub ext_control_buf: [__be32; 4],
// C attribute field omitted
// IOA Request Control Block    128 bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_ioarcb {
    pub ioarcb_host_pci_addr: __be32,
    pub ioarcb_host_pci_addr64: __be64,
    pub a: },
    pub res_handle: __be32,
    pub host_response_handle: __be32,
    pub reserved1: __be32,
    pub reserved2: __be32,
    pub reserved3: __be32,
    pub data_transfer_length: __be32,
    pub read_data_transfer_length: __be32,
    pub write_ioadl_addr: __be32,
    pub ioadl_len: __be32,
    pub read_ioadl_addr: __be32,
    pub read_ioadl_len: __be32,
    pub ioasa_host_pci_addr: __be32,
    pub ioasa_len: __be16,
    pub reserved4: __be16,
    pub cmd_pkt: ipr_cmd_pkt,
    pub add_cmd_parms_offset: __be16,
    pub add_cmd_parms_len: __be16,
    pub add_data: ipr_ioarcb_add_data,
    pub sis64_addr_data: ipr_ioarcb_sis64_add_addr_ecb,
    pub u: },
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_ioasa_vset {
    pub failing_lba_hi: __be32,
    pub failing_lba_lo: __be32,
    pub reserved: __be32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_ioasa_af_dasd {
    pub failing_lba: __be32,
    pub reserved: [__be32; 2],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_ioasa_gpdd {
    pub end_state: u8,
    pub bus_phase: u8,
    pub reserved: __be16,
    pub ioa_data: [__be32; 2],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_auto_sense {
    pub auto_sense_len: __be16,
    pub ioa_data_len: __be16,
    pub data: [__be32; SCSI_SENSE_BUFFERSIZE/sizeof(__be32)],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_ioasa_hdr {
    pub ioasc: __be32,

    pub /: *mut *mut __be16 ret_stat_len; / Length of the returned IOASA,
    pub /: *mut *mut __be16 avail_stat_len; / Total Length of status available.,
    pub /: *mut *mut __be32 residual_data_len; / number of bytes in the host data,
// buffers that were not used by the IOARCB command.
    pub ilid: __be32,
pub const IPR_NO_ILID: c_int = 0;
pub const IPR_DRIVER_ILID: c_uint = 0xffffffff;
    pub fd_ioasc: __be32,
    pub fd_phys_locator: __be32,
    pub fd_res_handle: __be32,
    pub /: *mut *mut __be32 ioasc_specific; / status code specific field,
pub const IPR_ADDITIONAL_STATUS_FMT: c_uint = 0x80000000;
pub const IPR_AUTOSENSE_VALID: c_uint = 0x40000000;
pub const IPR_IOASC_SPECIFIC_MASK: c_uint = 0x00ffffff;

pub const IPR_FIELD_POINTER_MASK: c_uint = 0x0000ffff;
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_ioasa {
    pub hdr: ipr_ioasa_hdr,
    pub vset: ipr_ioasa_vset,
    pub dasd: ipr_ioasa_af_dasd,
    pub gpdd: ipr_ioasa_gpdd,
    pub u: },
    pub auto_sense: ipr_auto_sense,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_ioasa64 {
    pub hdr: ipr_ioasa_hdr,
    pub fd_res_path: [u8; 8],
    pub vset: ipr_ioasa_vset,
    pub dasd: ipr_ioasa_af_dasd,
    pub gpdd: ipr_ioasa_gpdd,
    pub u: },
    pub auto_sense: ipr_auto_sense,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_mode_parm_hdr {
    pub length: u8,
    pub medium_type: u8,
    pub device_spec_parms: u8,
    pub block_desc_len: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_mode_pages {
    pub hdr: ipr_mode_parm_hdr,
    pub ipr_mode_parm_hdr)]: u8 data[255 - sizeof(struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_mode_page_hdr {
    pub ps_page_code: u8,
pub const IPR_MODE_PAGE_PS: c_uint = 0x80;

    pub page_length: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_dev_bus_entry {
    pub res_addr: ipr_res_addr,
    pub flags: u8,
pub const IPR_SCSI_ATTR_ENABLE_QAS: c_uint = 0x80;
pub const IPR_SCSI_ATTR_DISABLE_QAS: c_uint = 0x40;
pub const IPR_SCSI_ATTR_QAS_MASK: c_uint = 0xC0;
pub const IPR_SCSI_ATTR_ENABLE_TM: c_uint = 0x20;
pub const IPR_SCSI_ATTR_NO_TERM_PWR: c_uint = 0x10;
pub const IPR_SCSI_ATTR_TM_SUPPORTED: c_uint = 0x08;
pub const IPR_SCSI_ATTR_LVD_TO_SE_NOT_ALLOWED: c_uint = 0x04;
    pub scsi_id: u8,
    pub bus_width: u8,
    pub extended_reset_delay: u8,
pub const IPR_EXTENDED_RESET_DELAY: c_int = 7;
    pub max_xfer_rate: __be32,
    pub spinup_delay: u8,
    pub reserved3: u8,
    pub reserved4: __be16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_mode_page28 {
    pub hdr: ipr_mode_page_hdr,
    pub num_entries: u8,
    pub entry_length: u8,
    pub bus: [ipr_dev_bus_entry; ],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_mode_page24 {
    pub hdr: ipr_mode_page_hdr,
    pub flags: u8,
pub const IPR_ENABLE_DUAL_IOA_AF: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_ioa_vpd {
    pub std_inq_data: ipr_std_inq_data,
    pub ascii_part_num: [u8; 12],
    pub reserved: [u8; 40],
    pub ascii_plant_code: [u8; 4],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_inquiry_page3 {
    pub peri_qual_dev_type: u8,
    pub page_code: u8,
    pub reserved1: u8,
    pub page_length: u8,
    pub ascii_len: u8,
    pub reserved2: [u8; 3],
    pub load_id: [u8; 4],
    pub major_release: u8,
    pub card_type: u8,
    pub minor_release: [u8; 2],
    pub ptf_number: [u8; 4],
    pub patch_number: [u8; 4],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_inquiry_cap {
    pub peri_qual_dev_type: u8,
    pub page_code: u8,
    pub reserved1: u8,
    pub page_length: u8,
    pub ascii_len: u8,
    pub reserved2: u8,
    pub sis_version: [u8; 2],
    pub cap: u8,
pub const IPR_CAP_DUAL_IOA_RAID: c_uint = 0x80;
    pub reserved3: [u8; 15],
pub const IPR_INQUIRY_PAGE0_ENTRIES: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_inquiry_page0 {
    pub peri_qual_dev_type: u8,
    pub page_code: u8,
    pub reserved1: u8,
    pub len: u8,
    pub page: [u8; IPR_INQUIRY_PAGE0_ENTRIES],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_inquiry_pageC4 {
    pub peri_qual_dev_type: u8,
    pub page_code: u8,
    pub reserved1: u8,
    pub len: u8,
    pub cache_cap: [u8; 4],
pub const IPR_CAP_SYNC_CACHE: c_uint = 0x08;
    pub reserved2: [u8; 20],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_device_data_entry {
    pub vpd: ipr_vpd,
    pub dev_res_addr: ipr_res_addr,
    pub new_vpd: ipr_vpd,
    pub ioa_last_with_dev_vpd: ipr_vpd,
    pub cfc_last_with_dev_vpd: ipr_vpd,
    pub ioa_data: [__be32; 5],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_device_data_entry_enhanced {
    pub vpd: ipr_ext_vpd,
    pub ccin: [u8; 4],
    pub dev_res_addr: ipr_res_addr,
    pub new_vpd: ipr_ext_vpd,
    pub new_ccin: [u8; 4],
    pub ioa_last_with_dev_vpd: ipr_ext_vpd,
    pub cfc_last_with_dev_vpd: ipr_ext_vpd,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb64_device_data_entry_enhanced {
    pub vpd: ipr_ext_vpd,
    pub ccin: [u8; 4],
    pub res_path: [u8; 8],
    pub new_vpd: ipr_ext_vpd,
    pub new_ccin: [u8; 4],
    pub ioa_last_with_dev_vpd: ipr_ext_vpd,
    pub cfc_last_with_dev_vpd: ipr_ext_vpd,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_array_data_entry {
    pub vpd: ipr_vpd,
    pub expected_dev_res_addr: ipr_res_addr,
    pub dev_res_addr: ipr_res_addr,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb64_array_data_entry {
    pub vpd: ipr_ext_vpd,
    pub ccin: [u8; 4],
    pub expected_res_path: [u8; 8],
    pub res_path: [u8; 8],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_array_data_entry_enhanced {
    pub vpd: ipr_ext_vpd,
    pub ccin: [u8; 4],
    pub expected_dev_res_addr: ipr_res_addr,
    pub dev_res_addr: ipr_res_addr,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_type_ff_error {
    pub ioa_data: [__be32; 758],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_type_01_error {
    pub seek_counter: __be32,
    pub read_counter: __be32,
    pub sense_data: [u8; 32],
    pub ioa_data: [__be32; 236],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_type_21_error {
    pub wwn: [__be32; 4],
    pub res_path: [u8; 8],
    pub primary_problem_desc: [u8; 32],
    pub second_problem_desc: [u8; 32],
    pub sense_data: [__be32; 8],
    pub cdb: [__be32; 4],
    pub residual_trans_length: __be32,
    pub length_of_error: __be32,
    pub ioa_data: [__be32; 236],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_type_02_error {
    pub ioa_vpd: ipr_vpd,
    pub cfc_vpd: ipr_vpd,
    pub ioa_last_attached_to_cfc_vpd: ipr_vpd,
    pub cfc_last_attached_to_ioa_vpd: ipr_vpd,
    pub ioa_data: [__be32; 3],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_type_12_error {
    pub ioa_vpd: ipr_ext_vpd,
    pub cfc_vpd: ipr_ext_vpd,
    pub ioa_last_attached_to_cfc_vpd: ipr_ext_vpd,
    pub cfc_last_attached_to_ioa_vpd: ipr_ext_vpd,
    pub ioa_data: [__be32; 3],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_type_03_error {
    pub ioa_vpd: ipr_vpd,
    pub cfc_vpd: ipr_vpd,
    pub errors_detected: __be32,
    pub errors_logged: __be32,
    pub ioa_data: [u8; 12],
    pub dev: [ipr_hostrcb_device_data_entry; 3],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_type_13_error {
    pub ioa_vpd: ipr_ext_vpd,
    pub cfc_vpd: ipr_ext_vpd,
    pub errors_detected: __be32,
    pub errors_logged: __be32,
    pub dev: [ipr_hostrcb_device_data_entry_enhanced; 3],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_type_23_error {
    pub ioa_vpd: ipr_ext_vpd,
    pub cfc_vpd: ipr_ext_vpd,
    pub errors_detected: __be32,
    pub errors_logged: __be32,
    pub dev: [ipr_hostrcb64_device_data_entry_enhanced; 3],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_type_04_error {
    pub ioa_vpd: ipr_vpd,
    pub cfc_vpd: ipr_vpd,
    pub ioa_data: [u8; 12],
    pub array_member: [ipr_hostrcb_array_data_entry; 10],
    pub exposed_mode_adn: __be32,
    pub array_id: __be32,
    pub incomp_dev_vpd: ipr_vpd,
    pub ioa_data2: __be32,
    pub array_member2: [ipr_hostrcb_array_data_entry; 8],
    pub last_func_vset_res_addr: ipr_res_addr,
    pub vset_serial_num: [u8; IPR_SERIAL_NUM_LEN],
    pub protection_level: [u8; 8],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_type_14_error {
    pub ioa_vpd: ipr_ext_vpd,
    pub cfc_vpd: ipr_ext_vpd,
    pub exposed_mode_adn: __be32,
    pub array_id: __be32,
    pub last_func_vset_res_addr: ipr_res_addr,
    pub vset_serial_num: [u8; IPR_SERIAL_NUM_LEN],
    pub protection_level: [u8; 8],
    pub num_entries: __be32,
    pub array_member: [ipr_hostrcb_array_data_entry_enhanced; 18],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_type_24_error {
    pub ioa_vpd: ipr_ext_vpd,
    pub cfc_vpd: ipr_ext_vpd,
    pub reserved: [u8; 2],
    pub exposed_mode_adn: u8,
pub const IPR_INVALID_ARRAY_DEV_NUM: c_uint = 0xff;
    pub array_id: u8,
    pub last_res_path: [u8; 8],
    pub protection_level: [u8; 8],
    pub array_vpd: ipr_ext_vpd64,
    pub description: [u8; 16],
    pub reserved2: [u8; 3],
    pub num_entries: u8,
    pub array_member: [ipr_hostrcb64_array_data_entry; 32],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_type_07_error {
    pub failure_reason: [u8; 64],
    pub vpd: ipr_vpd,
    pub data: [__be32; 222],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_type_17_error {
    pub failure_reason: [u8; 64],
    pub vpd: ipr_ext_vpd,
    pub data: [__be32; 476],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_config_element {
    pub type_status: u8,
pub const IPR_PATH_CFG_TYPE_MASK: c_uint = 0xF0;
pub const IPR_PATH_CFG_NOT_EXIST: c_uint = 0x00;
pub const IPR_PATH_CFG_IOA_PORT: c_uint = 0x10;
pub const IPR_PATH_CFG_EXP_PORT: c_uint = 0x20;
pub const IPR_PATH_CFG_DEVICE_PORT: c_uint = 0x30;
pub const IPR_PATH_CFG_DEVICE_LUN: c_uint = 0x40;
pub const IPR_PATH_CFG_STATUS_MASK: c_uint = 0x0F;
pub const IPR_PATH_CFG_NO_PROB: c_uint = 0x00;
pub const IPR_PATH_CFG_DEGRADED: c_uint = 0x01;
pub const IPR_PATH_CFG_FAILED: c_uint = 0x02;
pub const IPR_PATH_CFG_SUSPECT: c_uint = 0x03;
pub const IPR_PATH_NOT_DETECTED: c_uint = 0x04;
pub const IPR_PATH_INCORRECT_CONN: c_uint = 0x05;
    pub cascaded_expander: u8,
    pub phy: u8,
    pub link_rate: u8,
pub const IPR_PHY_LINK_RATE_MASK: c_uint = 0x0F;
    pub wwid: [__be32; 2],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb64_config_element {
    pub length: __be16,
    pub descriptor_id: u8,
pub const IPR_DESCRIPTOR_MASK: c_uint = 0xC0;
pub const IPR_DESCRIPTOR_SIS64: c_uint = 0x00;
    pub reserved: u8,
    pub type_status: u8,
    pub reserved2: [u8; 2],
    pub link_rate: u8,
    pub res_path: [u8; 8],
    pub wwid: [__be32; 2],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_fabric_desc {
    pub length: __be16,
    pub ioa_port: u8,
    pub cascaded_expander: u8,
    pub phy: u8,
    pub path_state: u8,
pub const IPR_PATH_ACTIVE_MASK: c_uint = 0xC0;
pub const IPR_PATH_NO_INFO: c_uint = 0x00;
pub const IPR_PATH_ACTIVE: c_uint = 0x40;
pub const IPR_PATH_NOT_ACTIVE: c_uint = 0x80;
pub const IPR_PATH_STATE_MASK: c_uint = 0x0F;
pub const IPR_PATH_STATE_NO_INFO: c_uint = 0x00;
pub const IPR_PATH_HEALTHY: c_uint = 0x01;
pub const IPR_PATH_DEGRADED: c_uint = 0x02;
pub const IPR_PATH_FAILED: c_uint = 0x03;
    pub num_entries: __be16,
    pub elem: [ipr_hostrcb_config_element; ],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb64_fabric_desc {
    pub length: __be16,
    pub descriptor_id: u8,
    pub reserved: [u8; 2],
    pub path_state: u8,
    pub reserved2: [u8; 2],
    pub res_path: [u8; 8],
    pub reserved3: [u8; 6],
    pub num_entries: __be16,
    pub elem: [ipr_hostrcb64_config_element; ],
// C attribute field omitted

    pub \: for (hrrq = (ioa_cfg)->hrrq;,
    pub hrrq++): hrrq < ((ioa_cfg)->hrrq + (ioa_cfg)->hrrq_num);,

    pub \: for (cfg = (fabric)->elem;,
    pub \: cfg < ((fabric)->elem + be16_to_cpu((fabric)->num_entries));,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_type_20_error {
    pub failure_reason: [u8; 64],
    pub reserved: [u8; 3],
    pub num_entries: u8,
    pub desc: [ipr_hostrcb_fabric_desc; 1],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_type_30_error {
    pub failure_reason: [u8; 64],
    pub reserved: [u8; 3],
    pub num_entries: u8,
    pub desc: [ipr_hostrcb64_fabric_desc; 1],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_type_41_error {
    pub failure_reason: [u8; 64],
    pub data: [__be32; 200],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_error {
    pub fd_ioasc: __be32,
    pub fd_res_addr: ipr_res_addr,
    pub fd_res_handle: __be32,
    pub prc: __be32,
    pub type_ff_error: ipr_hostrcb_type_ff_error,
    pub type_01_error: ipr_hostrcb_type_01_error,
    pub type_02_error: ipr_hostrcb_type_02_error,
    pub type_03_error: ipr_hostrcb_type_03_error,
    pub type_04_error: ipr_hostrcb_type_04_error,
    pub type_07_error: ipr_hostrcb_type_07_error,
    pub type_12_error: ipr_hostrcb_type_12_error,
    pub type_13_error: ipr_hostrcb_type_13_error,
    pub type_14_error: ipr_hostrcb_type_14_error,
    pub type_17_error: ipr_hostrcb_type_17_error,
    pub type_20_error: ipr_hostrcb_type_20_error,
    pub u: },
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb64_error {
    pub fd_ioasc: __be32,
    pub ioa_fw_level: __be32,
    pub fd_res_handle: __be32,
    pub prc: __be32,
    pub fd_dev_id: __be64,
    pub fd_lun: __be64,
    pub fd_res_path: [u8; 8],
    pub time_stamp: __be64,
    pub reserved: [u8; 16],
    pub type_ff_error: ipr_hostrcb_type_ff_error,
    pub type_12_error: ipr_hostrcb_type_12_error,
    pub type_17_error: ipr_hostrcb_type_17_error,
    pub type_21_error: ipr_hostrcb_type_21_error,
    pub type_23_error: ipr_hostrcb_type_23_error,
    pub type_24_error: ipr_hostrcb_type_24_error,
    pub type_30_error: ipr_hostrcb_type_30_error,
    pub type_41_error: ipr_hostrcb_type_41_error,
    pub u: },
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb_raw {
    pub ipr_hostrcb_error)/sizeof(__be32)]: __be32 data[sizeof(struct,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hcam {
    pub op_code: u8,
pub const IPR_HOST_RCB_OP_CODE_CONFIG_CHANGE: c_uint = 0xE1;
pub const IPR_HOST_RCB_OP_CODE_LOG_DATA: c_uint = 0xE2;
    pub notify_type: u8,
pub const IPR_HOST_RCB_NOTIF_TYPE_EXISTING_CHANGED: c_uint = 0x00;
pub const IPR_HOST_RCB_NOTIF_TYPE_NEW_ENTRY: c_uint = 0x01;
pub const IPR_HOST_RCB_NOTIF_TYPE_REM_ENTRY: c_uint = 0x02;
pub const IPR_HOST_RCB_NOTIF_TYPE_ERROR_LOG_ENTRY: c_uint = 0x10;
pub const IPR_HOST_RCB_NOTIF_TYPE_INFORMATION_ENTRY: c_uint = 0x11;
    pub notifications_lost: u8,
pub const IPR_HOST_RCB_NO_NOTIFICATIONS_LOST: c_int = 0;
pub const IPR_HOST_RCB_NOTIFICATIONS_LOST: c_uint = 0x80;
    pub flags: u8,
pub const IPR_HOSTRCB_INTERNAL_OPER: c_uint = 0x80;
pub const IPR_HOSTRCB_ERR_RESP_SENT: c_uint = 0x40;
    pub overlay_id: u8,
pub const IPR_HOST_RCB_OVERLAY_ID_1: c_uint = 0x01;
pub const IPR_HOST_RCB_OVERLAY_ID_2: c_uint = 0x02;
pub const IPR_HOST_RCB_OVERLAY_ID_3: c_uint = 0x03;
pub const IPR_HOST_RCB_OVERLAY_ID_4: c_uint = 0x04;
pub const IPR_HOST_RCB_OVERLAY_ID_6: c_uint = 0x06;
pub const IPR_HOST_RCB_OVERLAY_ID_7: c_uint = 0x07;
pub const IPR_HOST_RCB_OVERLAY_ID_12: c_uint = 0x12;
pub const IPR_HOST_RCB_OVERLAY_ID_13: c_uint = 0x13;
pub const IPR_HOST_RCB_OVERLAY_ID_14: c_uint = 0x14;
pub const IPR_HOST_RCB_OVERLAY_ID_16: c_uint = 0x16;
pub const IPR_HOST_RCB_OVERLAY_ID_17: c_uint = 0x17;
pub const IPR_HOST_RCB_OVERLAY_ID_20: c_uint = 0x20;
pub const IPR_HOST_RCB_OVERLAY_ID_21: c_uint = 0x21;
pub const IPR_HOST_RCB_OVERLAY_ID_23: c_uint = 0x23;
pub const IPR_HOST_RCB_OVERLAY_ID_24: c_uint = 0x24;
pub const IPR_HOST_RCB_OVERLAY_ID_26: c_uint = 0x26;
pub const IPR_HOST_RCB_OVERLAY_ID_30: c_uint = 0x30;
pub const IPR_HOST_RCB_OVERLAY_ID_41: c_uint = 0x41;
pub const IPR_HOST_RCB_OVERLAY_ID_DEFAULT: c_uint = 0xFF;
    pub reserved1: [u8; 3],
    pub ilid: __be32,
    pub time_since_last_ioa_reset: __be32,
    pub reserved2: __be32,
    pub length: __be32,
    pub error: ipr_hostrcb_error,
    pub error64: ipr_hostrcb64_error,
    pub ccn: ipr_hostrcb_cfg_ch_not,
    pub raw: ipr_hostrcb_raw,
    pub u: },
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_hostrcb {
    pub hcam: ipr_hcam,
    pub hostrcb_dma: dma_addr_t,
    pub queue: list_head,
    pub ioa_cfg: *mut ipr_ioa_cfg,
    pub rp_buffer: [c_char; IPR_MAX_RES_PATH_LENGTH],
}

// IPR smart dump table structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_sdt_entry {
    pub start_token: __be32,
    pub end_token: __be32,
    pub reserved: [u8; 4],
    pub flags: u8,
pub const IPR_SDT_ENDIAN: c_uint = 0x80;
pub const IPR_SDT_VALID_ENTRY: c_uint = 0x20;
    pub resv: u8,
    pub priority: __be16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_sdt_header {
    pub state: __be32,
    pub num_entries: __be32,
    pub num_entries_used: __be32,
    pub dump_size: __be32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_sdt {
    pub hdr: ipr_sdt_header,
    pub entry: [ipr_sdt_entry; IPR_FMT3_NUM_SDT_ENTRIES],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_uc_sdt {
    pub hdr: ipr_sdt_header,
    pub entry: [ipr_sdt_entry; 1],
// C attribute field omitted
//
// Driver types
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_bus_attributes {
    pub bus: u8,
    pub qas_enabled: u8,
    pub bus_width: u8,
    pub reserved: u8,
    pub max_xfer_rate: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_resource_entry {
    pub needs_sync_complete:1: u8,
    pub in_erp:1: u8,
    pub add_to_ml:1: u8,
    pub del_from_ml:1: u8,
    pub resetting_device:1: u8,
    pub reset_occurred:1: u8,
    pub raw_mode:1: u8,
    pub /: *mut *mut u32 bus; / AKA channel,
    pub /: *mut *mut u32 target; / AKA id,
    pub lun: u32,
pub const IPR_ARRAY_VIRTUAL_BUS: c_uint = 0x1;
pub const IPR_VSET_VIRTUAL_BUS: c_uint = 0x2;
pub const IPR_IOAFP_VIRTUAL_BUS: c_uint = 0x3;
pub const IPR_MAX_SIS64_BUSES: c_uint = 0x4;

    pub ata_class: u8,
    pub type: u8,
    pub flags: u16,
    pub res_flags: u16,
    pub qmodel: u8,
    pub std_inq_data: ipr_std_inq_data,
    pub res_handle: __be32,
    pub dev_id: __be64,
    pub lun_wwn: u64,
    pub dev_lun: scsi_lun,
    pub res_path: [u8; 8],
    pub ioa_cfg: *mut ipr_ioa_cfg,
    pub sdev: *mut scsi_device,
    pub queue: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_resource_hdr {
    pub num_entries: u16,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_misc_cbs {
    pub ioa_vpd: ipr_ioa_vpd,
    pub page0_data: ipr_inquiry_page0,
    pub page3_data: ipr_inquiry_page3,
    pub cap: ipr_inquiry_cap,
    pub pageC4_data: ipr_inquiry_pageC4,
    pub mode_pages: ipr_mode_pages,
    pub supp_dev: ipr_supported_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_interrupt_offsets {
    pub set_interrupt_mask_reg: c_ulong,
    pub clr_interrupt_mask_reg: c_ulong,
    pub clr_interrupt_mask_reg32: c_ulong,
    pub sense_interrupt_mask_reg: c_ulong,
    pub sense_interrupt_mask_reg32: c_ulong,
    pub clr_interrupt_reg: c_ulong,
    pub clr_interrupt_reg32: c_ulong,
    pub sense_interrupt_reg: c_ulong,
    pub sense_interrupt_reg32: c_ulong,
    pub ioarrin_reg: c_ulong,
    pub sense_uproc_interrupt_reg: c_ulong,
    pub sense_uproc_interrupt_reg32: c_ulong,
    pub set_uproc_interrupt_reg: c_ulong,
    pub set_uproc_interrupt_reg32: c_ulong,
    pub clr_uproc_interrupt_reg: c_ulong,
    pub clr_uproc_interrupt_reg32: c_ulong,
    pub init_feedback_reg: c_ulong,
    pub dump_addr_reg: c_ulong,
    pub dump_data_reg: c_ulong,
pub const IPR_ENDIAN_SWAP_KEY: c_uint = 0x00080800;
    pub endian_swap_reg: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_interrupts {
    pub set_interrupt_mask_reg: *mut void __iomem,
    pub clr_interrupt_mask_reg: *mut void __iomem,
    pub clr_interrupt_mask_reg32: *mut void __iomem,
    pub sense_interrupt_mask_reg: *mut void __iomem,
    pub sense_interrupt_mask_reg32: *mut void __iomem,
    pub clr_interrupt_reg: *mut void __iomem,
    pub clr_interrupt_reg32: *mut void __iomem,
    pub sense_interrupt_reg: *mut void __iomem,
    pub sense_interrupt_reg32: *mut void __iomem,
    pub ioarrin_reg: *mut void __iomem,
    pub sense_uproc_interrupt_reg: *mut void __iomem,
    pub sense_uproc_interrupt_reg32: *mut void __iomem,
    pub set_uproc_interrupt_reg: *mut void __iomem,
    pub set_uproc_interrupt_reg32: *mut void __iomem,
    pub clr_uproc_interrupt_reg: *mut void __iomem,
    pub clr_uproc_interrupt_reg32: *mut void __iomem,
    pub init_feedback_reg: *mut void __iomem,
    pub dump_addr_reg: *mut void __iomem,
    pub dump_data_reg: *mut void __iomem,
    pub endian_swap_reg: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_chip_cfg_t {
    pub mailbox: u32,
    pub max_cmds: u16,
    pub cache_line_size: u8,
    pub clear_isr: u8,
    pub iopoll_weight: u32,
    pub regs: ipr_interrupt_offsets,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_chip_t {
    pub vendor: u16,
    pub device: u16,
    pub has_msi: bool,
    pub sis_type: u16,
pub const IPR_SIS32: c_uint = 0x00;
pub const IPR_SIS64: c_uint = 0x01;
    pub bist_method: u16,
pub const IPR_PCI_CFG: c_uint = 0x00;
pub const IPR_MMIO: c_uint = 0x01;
    pub cfg: *const ipr_chip_cfg_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipr_shutdown_type {
    IPR_SHUTDOWN_NORMAL = 0x00,
    IPR_SHUTDOWN_PREPARE_FOR_NORMAL = 0x40,
    IPR_SHUTDOWN_ABBREV = 0x80,
    IPR_SHUTDOWN_NONE = 0x100,
    IPR_SHUTDOWN_QUIESCE = 0x101,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_trace_entry {
    pub time: u32,
    pub op_code: u8,
    pub ata_op_code: u8,
    pub type: u8,
pub const IPR_TRACE_START: c_uint = 0x00;
pub const IPR_TRACE_FINISH: c_uint = 0xff;
    pub cmd_index: u8,
    pub res_handle: __be32,
    pub ioasc: u32,
    pub add_data: u32,
    pub res_addr: u32,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_sglist {
    pub order: u32,
    pub num_sg: u32,
    pub num_dma_sg: u32,
    pub buffer_len: u32,
    pub scatterlist: *mut scatterlist,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipr_sdt_state {
    INACTIVE,
    WAIT_FOR_DUMP,
    GET_DUMP,
    READ_DUMP,
    ABORT_DUMP,
    DUMP_OBTAINED
}

// Per-controller data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_ioa_cfg {
    pub eye_catcher: [c_char; 8],
    pub queue: list_head,
    pub in_reset_reload:1: u8,
    pub in_ioa_bringdown:1: u8,
    pub ioa_unit_checked:1: u8,
    pub dump_taken:1: u8,
    pub scan_enabled:1: u8,
    pub scan_done:1: u8,
    pub needs_hard_reset:1: u8,
    pub dual_raid:1: u8,
    pub needs_warm_reset:1: u8,
    pub msi_received:1: u8,
    pub sis64:1: u8,
    pub dump_timeout:1: u8,
    pub cfg_locked:1: u8,
    pub clear_isr:1: u8,
    pub probe_done:1: u8,
    pub scsi_unblock:1: u8,
    pub scsi_blocked:1: u8,
    pub revid: u8,
//
// Bitmaps for SIS64 generated target values
//
    pub target_ids: [c_ulong; BITS_TO_LONGS(IPR_MAX_SIS64_DEVS)],
    pub array_ids: [c_ulong; BITS_TO_LONGS(IPR_MAX_SIS64_DEVS)],
    pub vset_ids: [c_ulong; BITS_TO_LONGS(IPR_MAX_SIS64_DEVS)],
    pub /: *mut *mut u16 type; / CCIN of the card,
    pub log_level: u8,
pub const IPR_MAX_LOG_LEVEL: c_int = 4;
pub const IPR_DEFAULT_LOG_LEVEL: c_int = 2;
pub const IPR_DEBUG_LOG_LEVEL: c_int = 3;
pub const IPR_NUM_TRACE_INDEX_BITS: c_int = 8;
    pub trace_start: [c_char; 8],
    pub trace: *mut ipr_trace_entry,
    pub trace_index: core::sync::atomic::AtomicI32,
    pub cfg_table_start: [c_char; 8],
    pub cfg_table: *mut ipr_config_table,
    pub cfg_table64: *mut ipr_config_table64,
    pub u: },
    pub cfg_table_dma: dma_addr_t,
    pub cfg_table_size: u32,
    pub max_devs_supported: u32,
    pub resource_table_label: [c_char; 8],
    pub res_entries: *mut ipr_resource_entry,
    pub free_res_q: list_head,
    pub used_res_q: list_head,
    pub ipr_hcam_label: [c_char; 8],    pub hostrcb: [*mut ipr_hostrcb; IPR_MAX_HCAMS],
    pub hostrcb_dma: [dma_addr_t; IPR_MAX_HCAMS],
    pub hostrcb_free_q: list_head,
    pub hostrcb_pending_q: list_head,
    pub hostrcb_report_q: list_head,
    pub hrrq: [ipr_hrr_queue; IPR_MAX_HRRQ_NUM],
    pub hrrq_num: u32,
    pub hrrq_index: core::sync::atomic::AtomicI32,
    pub identify_hrrq_index: u16,
    pub bus_attr: [ipr_bus_attributes; IPR_MAX_NUM_BUSES],
    pub transop_timeout: c_uint,
    pub chip_cfg: *const ipr_chip_cfg_t,
    pub ipr_chip: *const ipr_chip_t,
    pub /: *mut *mut *mut void __iomem hdw_dma_regs; / iomapped PCI memory space,
    pub /: *mut *mut unsigned long hdw_dma_regs_pci; / raw PCI memory space,
    pub ioa_mailbox: *mut void __iomem,
    pub regs: ipr_interrupts,
    pub saved_pcix_cmd_reg: u16,
    pub reset_retries: u16,
    pub errors_logged: u32,
    pub doorbell: u32,
    pub host: *mut Scsi_Host,
    pub pdev: *mut pci_dev,
    pub ucode_sglist: *mut ipr_sglist,
    pub saved_mode_page_len: u8,
    pub work_q: work_struct,
    pub scsi_add_work_q: work_struct,
    pub reset_work_q: *mut workqueue_struct,
    pub reset_wait_q: wait_queue_head_t,
    pub msi_wait_q: wait_queue_head_t,
    pub eeh_wait_q: wait_queue_head_t,
    pub dump: *mut ipr_dump,
    pub sdt_state: ipr_sdt_state,
    pub vpd_cbs: *mut ipr_misc_cbs,
    pub vpd_cbs_dma: dma_addr_t,
    pub ipr_cmd_pool: *mut dma_pool,
    pub reset_cmd: *mut ipr_cmnd,
    pub ): *mut *mut int (reset) (struct ipr_cmnd,
    pub ipr_cmd_label: [c_char; 8],
    pub max_cmds: u32,
    pub ipr_cmnd_list: *mut ipr_cmnd,
    pub ipr_cmnd_list_dma: *mut dma_addr_t,
    pub nvectors: c_uint,
    pub desc: [c_char; 22],
    pub vectors_info: [}; IPR_MAX_MSIX_VECTORS],
    pub iopoll_weight: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_cmnd {
    pub ioarcb: ipr_ioarcb,
    pub ioadl: [ipr_ioadl_desc; IPR_NUM_IOADL_ENTRIES],
    pub ioadl64: [ipr_ioadl64_desc; IPR_NUM_IOADL_ENTRIES],
    pub i: },
    pub ioasa: ipr_ioasa,
    pub ioasa64: ipr_ioasa64,
    pub s: },
    pub queue: list_head,
    pub scsi_cmd: *mut scsi_cmnd,
    pub completion: completion,
    pub timer: timer_list,
    pub work: work_struct,
    pub ): *mut *mut void (fast_done) (struct ipr_cmnd,
    pub ): *mut *mut void (done) (struct ipr_cmnd,
    pub ): *mut *mut int (job_step) (struct ipr_cmnd,
    pub ): *mut *mut int (job_step_failed) (struct ipr_cmnd,
    pub cmd_index: u16,
    pub sense_buffer: [u8; SCSI_SENSE_BUFFERSIZE],
    pub sense_buffer_dma: dma_addr_t,
    pub dma_use_sg: c_ushort,
    pub dma_addr: dma_addr_t,
    pub sibling: *mut ipr_cmnd,
    pub shutdown_type: ipr_shutdown_type,
    pub hostrcb: *mut ipr_hostrcb,
    pub time_left: c_ulong,
    pub scratch: c_ulong,
    pub res: *mut ipr_resource_entry,
    pub sdev: *mut scsi_device,
    pub u: },
    pub eh_comp: *mut completion,
    pub hrrq: *mut ipr_hrr_queue,
    pub ioa_cfg: *mut ipr_ioa_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_ses_table_entry {
    pub product_id: [c_char; 17],
    pub compare_product_id_byte: [c_char; 17],
    pub /: *mut *mut u32 max_bus_speed_limit; / MB/sec limit for this backplane,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_dump_header {
    pub eye_catcher: u32,
pub const IPR_DUMP_EYE_CATCHER: c_uint = 0xC5D4E3F2;
    pub len: u32,
    pub num_entries: u32,
    pub first_entry_offset: u32,
    pub status: u32,
pub const IPR_DUMP_STATUS_SUCCESS: c_int = 0;
pub const IPR_DUMP_STATUS_QUAL_SUCCESS: c_int = 2;
pub const IPR_DUMP_STATUS_FAILED: c_uint = 0xffffffff;
    pub os: u32,
pub const IPR_DUMP_OS_LINUX: c_uint = 0x4C4E5558;
    pub driver_name: u32,
pub const IPR_DUMP_DRIVER_NAME: c_uint = 0x49505232;
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_dump_entry_header {
    pub eye_catcher: u32,
pub const IPR_DUMP_EYE_CATCHER: c_uint = 0xC5D4E3F2;
    pub len: u32,
    pub num_elems: u32,
    pub offset: u32,
    pub data_type: u32,
pub const IPR_DUMP_DATA_TYPE_ASCII: c_uint = 0x41534349;
pub const IPR_DUMP_DATA_TYPE_BINARY: c_uint = 0x42494E41;
    pub id: u32,
pub const IPR_DUMP_IOA_DUMP_ID: c_uint = 0x494F4131;
pub const IPR_DUMP_LOCATION_ID: c_uint = 0x4C4F4341;
pub const IPR_DUMP_TRACE_ID: c_uint = 0x54524143;
pub const IPR_DUMP_DRIVER_VERSION_ID: c_uint = 0x44525652;
pub const IPR_DUMP_DRIVER_TYPE_ID: c_uint = 0x54595045;
pub const IPR_DUMP_IOA_CTRL_BLK: c_uint = 0x494F4342;
pub const IPR_DUMP_PEND_OPS: c_uint = 0x414F5053;
    pub status: u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_dump_location_entry {
    pub hdr: ipr_dump_entry_header,
    pub location: [u8; 20],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_dump_trace_entry {
    pub hdr: ipr_dump_entry_header,
    pub sizeof(u32)]: u32 trace[IPR_TRACE_SIZE /,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_dump_version_entry {
    pub hdr: ipr_dump_entry_header,
    pub version: [u8; sizeof(IPR_DRIVER_VERSION)],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_dump_ioa_type_entry {
    pub hdr: ipr_dump_entry_header,
    pub type: u32,
    pub fw_version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_driver_dump {
    pub hdr: ipr_dump_header,
    pub version_entry: ipr_dump_version_entry,
    pub location_entry: ipr_dump_location_entry,
    pub ioa_type_entry: ipr_dump_ioa_type_entry,
    pub trace_entry: ipr_dump_trace_entry,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_ioa_dump {
    pub hdr: ipr_dump_entry_header,
    pub sdt: ipr_sdt,
    pub ioa_data: *mut __be32,
    pub reserved: u32,
    pub next_page_index: u32,
    pub page_offset: u32,
    pub format: u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_dump {
    pub kref: kref,
    pub ioa_cfg: *mut ipr_ioa_cfg,
    pub driver_dump: ipr_driver_dump,
    pub ioa_dump: ipr_ioa_dump,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_error_table_t {
    pub ioasc: u32,
    pub log_ioasa: c_int,
    pub log_hcam: c_int,
    pub error: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_software_inq_lid_info {
    pub load_id: __be32,
    pub timestamp: [__be32; 3],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipr_ucode_image_header {
    pub header_length: __be32,
    pub lid_table_offset: __be32,
    pub major_release: u8,
    pub card_type: u8,
    pub minor_release: [u8; 2],
    pub reserved: [u8; 20],
    pub eyecatcher: [c_char; 16],
    pub num_lids: __be32,
    pub lid: [ipr_software_inq_lid_info; 1],
// C attribute field omitted
//
// Macros
//

//
// Error logging macros
//

    pub \: ipr_err(fmt": unknown\n", ##__VA_ARGS__);,

    pub \: (res).bus, (res).target, (res).lun);,

    pub \: __VA_ARGS__);,
    pub \: fmt, __VA_ARGS__);,
    pub \: dev_err(&(hostrcb)->ioa_cfg->pdev->dev, fmt, __VA_ARGS__);,

//
// Inlines
//
// ipr_is_ioa_resource - Determine if a resource is the IOA
// @res:	resource entry struct
//
// Return value:
// 1 if IOA / 0 if not IOA
//
    pub IPR_RES_TYPE_IOAFP: return res->type ==,
//
// ipr_is_af_dasd_device - Determine if a resource is an AF DASD
// @res:	resource entry struct
//
// Return value:
// 1 if AF DASD / 0 if not AF DASD
//
    pub IPR_RES_TYPE_REMOTE_AF_DASD: res->type ==,
//
// ipr_is_vset_device - Determine if a resource is a VSET
// @res:	resource entry struct
//
// Return value:
// 1 if VSET / 0 if not VSET
//
    pub IPR_RES_TYPE_VOLUME_SET: return res->type ==,
//
// ipr_is_gscsi - Determine if a resource is a generic scsi resource
// @res:	resource entry struct
//
// Return value:
// 1 if GSCSI / 0 if not GSCSI
//
    pub IPR_RES_TYPE_GENERIC_SCSI: return res->type ==,
//
// ipr_is_scsi_disk - Determine if a resource is a SCSI disk
// @res:	resource entry struct
//
// Return value:
// 1 if SCSI disk / 0 if not SCSI disk
//
    pub 1: return,
    pub 0: return,
//
// ipr_is_gata - Determine if a resource is a generic ATA resource
// @res:	resource entry struct
//
// Return value:
// 1 if GATA / 0 if not GATA
//
    pub IPR_RES_TYPE_GENERIC_ATA: return res->type ==,
//
// ipr_is_naca_model - Determine if a resource is using NACA queueing model
// @res:	resource entry struct
//
// Return value:
// 1 if NACA queueing model / 0 if not NACA queueing model
//
    pub 1: return,
    pub 0: return,
//
// ipr_is_device - Determine if the hostrcb structure is related to a device
// @hostrcb:	host resource control blocks struct
//
// Return value:
// 1 if AF / 0 if not AF
//
    pub res_addr: *mut ipr_res_addr,
    pub res_path: *mut u8,
    pub &hostrcb->hcam.u.error64.fd_res_path[0]: res_path =,
    pub 1: return,
    pub &hostrcb->hcam.u.error.fd_res_addr: res_addr =,
    pub 1: return,
    pub 0: return,
//
// ipr_sdt_is_fmt2 - Determine if a SDT address is in format 2
// @sdt_word:	SDT address
//
// Return value:
// 1 if format 2 / 0 if not
//
    pub IPR_GET_FMT2_BAR_SEL(sdt_word): u32 bar_sel =,
    pub 1: return,
}

