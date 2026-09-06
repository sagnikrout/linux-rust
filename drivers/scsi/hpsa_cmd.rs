//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/hpsa_cmd.h
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


//
// Disk Array driver for HP Smart Array SAS controllers
// Copyright (c) 2019-2020 Microchip Technology Inc. and its subsidiaries
// Copyright 2016 Microsemi Corporation
// Copyright 2014-2015 PMC-Sierra, Inc.
// Copyright 2000,2009-2015 Hewlett-Packard Development Company, L.P.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; version 2 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE, GOOD TITLE or
// NON INFRINGEMENT.  See the GNU General Public License for more details.
//
// Questions/Comments/Bugfixes to esc.storagedev@microsemi.com
//

// general boundary defintions

pub const HPSA_SG_CHAIN: c_uint = 0x80000000;
pub const HPSA_SG_LAST: c_uint = 0x40000000;
pub const MAXREPLYQS: c_int = 256;
// Command Status value
pub const CMD_SUCCESS: c_uint = 0x0000;
pub const CMD_TARGET_STATUS: c_uint = 0x0001;
pub const CMD_DATA_UNDERRUN: c_uint = 0x0002;
pub const CMD_DATA_OVERRUN: c_uint = 0x0003;
pub const CMD_INVALID: c_uint = 0x0004;
pub const CMD_PROTOCOL_ERR: c_uint = 0x0005;
pub const CMD_HARDWARE_ERR: c_uint = 0x0006;
pub const CMD_CONNECTION_LOST: c_uint = 0x0007;
pub const CMD_ABORTED: c_uint = 0x0008;
pub const CMD_ABORT_FAILED: c_uint = 0x0009;
pub const CMD_UNSOLICITED_ABORT: c_uint = 0x000A;
pub const CMD_TIMEOUT: c_uint = 0x000B;
pub const CMD_UNABORTABLE: c_uint = 0x000C;
pub const CMD_TMF_STATUS: c_uint = 0x000D;
pub const CMD_IOACCEL_DISABLED: c_uint = 0x000E;
pub const CMD_CTLR_LOCKUP: c_uint = 0xffff;
// Note: CMD_CTLR_LOCKUP is not a value defined by the CISS spec
// it is a value defined by the driver that commands can be marked
// with when a controller lockup has been detected by the driver
//
// TMF function status values
pub const CISS_TMF_COMPLETE: c_uint = 0x00;
pub const CISS_TMF_INVALID_FRAME: c_uint = 0x02;
pub const CISS_TMF_NOT_SUPPORTED: c_uint = 0x04;
pub const CISS_TMF_FAILED: c_uint = 0x05;
pub const CISS_TMF_SUCCESS: c_uint = 0x08;
pub const CISS_TMF_WRONG_LUN: c_uint = 0x09;
pub const CISS_TMF_OVERLAPPED_TAG: c_uint = 0x0a;
// Unit Attentions ASC's as defined for the MSA2012sa
pub const POWER_OR_RESET: c_uint = 0x29;
pub const STATE_CHANGED: c_uint = 0x2a;
pub const UNIT_ATTENTION_CLEARED: c_uint = 0x2f;
pub const LUN_FAILED: c_uint = 0x3e;
pub const REPORT_LUNS_CHANGED: c_uint = 0x3f;
// Unit Attentions ASCQ's as defined for the MSA2012sa
// These ASCQ's defined for ASC = POWER_OR_RESET
pub const POWER_ON_RESET: c_uint = 0x00;
pub const POWER_ON_REBOOT: c_uint = 0x01;
pub const SCSI_BUS_RESET: c_uint = 0x02;
pub const MSA_TARGET_RESET: c_uint = 0x03;
pub const CONTROLLER_FAILOVER: c_uint = 0x04;
pub const TRANSCEIVER_SE: c_uint = 0x05;
pub const TRANSCEIVER_LVD: c_uint = 0x06;
// These ASCQ's defined for ASC = STATE_CHANGED
pub const RESERVATION_PREEMPTED: c_uint = 0x03;
pub const ASYM_ACCESS_CHANGED: c_uint = 0x06;
pub const LUN_CAPACITY_CHANGED: c_uint = 0x09;
// transfer direction
pub const XFER_NONE: c_uint = 0x00;
pub const XFER_WRITE: c_uint = 0x01;
pub const XFER_READ: c_uint = 0x02;
pub const XFER_RSVD: c_uint = 0x03;
// task attribute
pub const ATTR_UNTAGGED: c_uint = 0x00;
pub const ATTR_SIMPLE: c_uint = 0x04;
pub const ATTR_HEADOFQUEUE: c_uint = 0x05;
pub const ATTR_ORDERED: c_uint = 0x06;
pub const ATTR_ACA: c_uint = 0x07;
// cdb type
pub const TYPE_CMD: c_uint = 0x00;
pub const TYPE_MSG: c_uint = 0x01;
pub const TYPE_IOACCEL2_CMD: c_uint = 0x81 /* 0x81 is not used by hardware */;
// Message Types
pub const HPSA_TASK_MANAGEMENT: c_uint = 0x00;
pub const HPSA_RESET: c_uint = 0x01;
pub const HPSA_SCAN: c_uint = 0x02;
pub const HPSA_NOOP: c_uint = 0x03;
pub const HPSA_CTLR_RESET_TYPE: c_uint = 0x00;
pub const HPSA_BUS_RESET_TYPE: c_uint = 0x01;
pub const HPSA_TARGET_RESET_TYPE: c_uint = 0x03;
pub const HPSA_LUN_RESET_TYPE: c_uint = 0x04;
pub const HPSA_NEXUS_RESET_TYPE: c_uint = 0x05;
// Task Management Functions
pub const HPSA_TMF_ABORT_TASK: c_uint = 0x00;
pub const HPSA_TMF_ABORT_TASK_SET: c_uint = 0x01;
pub const HPSA_TMF_CLEAR_ACA: c_uint = 0x02;
pub const HPSA_TMF_CLEAR_TASK_SET: c_uint = 0x03;
pub const HPSA_TMF_QUERY_TASK: c_uint = 0x04;
pub const HPSA_TMF_QUERY_TASK_SET: c_uint = 0x05;
pub const HPSA_TMF_QUERY_ASYNCEVENT: c_uint = 0x06;
// config space register offsets
pub const CFG_VENDORID: c_uint = 0x00;
pub const CFG_DEVICEID: c_uint = 0x02;
pub const CFG_I2OBAR: c_uint = 0x10;
pub const CFG_MEM1BAR: c_uint = 0x14;
// i2o space register offsets
pub const I2O_IBDB_SET: c_uint = 0x20;
pub const I2O_IBDB_CLEAR: c_uint = 0x70;
pub const I2O_INT_STATUS: c_uint = 0x30;
pub const I2O_INT_MASK: c_uint = 0x34;
pub const I2O_IBPOST_Q: c_uint = 0x40;
pub const I2O_OBPOST_Q: c_uint = 0x44;
pub const I2O_DMA1_CFG: c_uint = 0x214;
// Configuration Table
pub const CFGTBL_ChangeReq: c_uint = 0x00000001l;
pub const CFGTBL_AccCmds: c_uint = 0x00000001l;
pub const DOORBELL_CTLR_RESET: c_uint = 0x00000004l;
pub const DOORBELL_CTLR_RESET2: c_uint = 0x00000020l;
pub const DOORBELL_CLEAR_EVENTS: c_uint = 0x00000040l;
pub const DOORBELL_GENERATE_CHKPT: c_uint = 0x00000080l;
pub const CFGTBL_Trans_Simple: c_uint = 0x00000002l;
pub const CFGTBL_Trans_Performant: c_uint = 0x00000004l;
pub const CFGTBL_Trans_io_accel1: c_uint = 0x00000080l;
pub const CFGTBL_Trans_io_accel2: c_uint = 0x00000100l;
pub const CFGTBL_Trans_use_short_tags: c_uint = 0x20000000l;

pub const CFGTBL_BusType_Ultra2: c_uint = 0x00000001l;
pub const CFGTBL_BusType_Ultra3: c_uint = 0x00000002l;
pub const CFGTBL_BusType_Fibre1G: c_uint = 0x00000100l;
pub const CFGTBL_BusType_Fibre2G: c_uint = 0x00000200l;
// VPD Inquiry types
pub const HPSA_INQUIRY_FAILED: c_uint = 0x02;
pub const HPSA_VPD_SUPPORTED_PAGES: c_uint = 0x00;
pub const HPSA_VPD_LV_DEVICE_ID: c_uint = 0x83;
pub const HPSA_VPD_LV_DEVICE_GEOMETRY: c_uint = 0xC1;
pub const HPSA_VPD_LV_IOACCEL_STATUS: c_uint = 0xC2;
pub const HPSA_VPD_LV_STATUS: c_uint = 0xC3;
pub const HPSA_VPD_HEADER_SZ: c_int = 4;
// Logical volume states
pub const HPSA_VPD_LV_STATUS_UNSUPPORTED: c_uint = 0xff;
pub const HPSA_LV_OK: c_uint = 0x0;
pub const HPSA_LV_FAILED: c_uint = 0x01;
pub const HPSA_LV_NOT_AVAILABLE: c_uint = 0x0b;
pub const HPSA_LV_UNDERGOING_ERASE: c_uint = 0x0F;
pub const HPSA_LV_UNDERGOING_RPI: c_uint = 0x12;
pub const HPSA_LV_PENDING_RPI: c_uint = 0x13;
pub const HPSA_LV_ENCRYPTED_NO_KEY: c_uint = 0x14;
pub const HPSA_LV_PLAINTEXT_IN_ENCRYPT_ONLY_CONTROLLER: c_uint = 0x15;
pub const HPSA_LV_UNDERGOING_ENCRYPTION: c_uint = 0x16;
pub const HPSA_LV_UNDERGOING_ENCRYPTION_REKEYING: c_uint = 0x17;
pub const HPSA_LV_ENCRYPTED_IN_NON_ENCRYPTED_CONTROLLER: c_uint = 0x18;
pub const HPSA_LV_PENDING_ENCRYPTION: c_uint = 0x19;
pub const HPSA_LV_PENDING_ENCRYPTION_REKEYING: c_uint = 0x1A;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vals32 {
    pub lower: u32,
    pub upper: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union u64bit {
    pub val32: vals32,
    pub val: u64,
}

// FIXME this is a per controller value (barf!)
pub const HPSA_MAX_LUN: c_int = 1024;
pub const HPSA_MAX_PHYS_LUN: c_int = 1024;
pub const MAX_EXT_TARGETS: c_int = 32;

// SCSI-3 Commands
pub const HPSA_INQUIRY: c_uint = 0x12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InquiryData {
    pub data_byte: [u8; 36],
    pub __packed: },
pub const HPSA_REPORT_LOG: c_uint = 0xc2    /* Report Logical LUNs */;
pub const HPSA_REPORT_PHYS: c_uint = 0xc3   /* Report Physical LUNs */;
pub const HPSA_REPORT_PHYS_EXTENDED: c_uint = 0x02;
pub const HPSA_CISS_READ: c_uint = 0xc0	/* CISS Read */;
pub const HPSA_GET_RAID_MAP: c_uint = 0xc8	/* CISS Get RAID Layout Map */;
pub const RAID_MAP_MAX_ENTRIES: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raid_map_disk_data {
    pub the: *mut *mut *mut u32 ioaccel_handle; /< Handle to access this disk via,
// I/O accelerator
    pub position,: *mut *mut *mut u8 xor_mult[2]; /< XOR multipliers for this,
// valid for data disks only
    pub reserved: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raid_map_data {
    pub /: *mut *mut __le32 structure_size; / Size of entire structure in bytes,
    pub /: *mut *mut __le32 volume_blk_size; / bytes / block in the volume,
    pub /: *mut *mut __le64 volume_blk_cnt; / logical blocks on the volume,
    pub between: *mut *mut u8 phys_blk_shift; / Shift factor to convert,
// units of logical blocks and physical
// disk blocks
    pub units: *mut *mut u8 parity_rotation_shift; / Shift factor to convert between,
// of logical stripes and physical
// stripes
    pub /: *mut *mut __le16 strip_size; / blocks used on each disk / stripe,
    pub /: *mut *mut __le64 disk_starting_blk; / First disk block used in volume,
    pub /: *mut *mut __le64 disk_blk_cnt; / disk blocks used by volume / disk,
    pub /: *mut *mut __le16 data_disks_per_row; / data disk entries / row in the map,
    pub row: *mut *mut __le16 metadata_disks_per_row;/ mirror/parity disk entries /,
// in the map
    pub /: *mut *mut __le16 row_cnt; / rows in each layout map,
    pub mirror/parity: *mut *mut __le16 layout_map_count; / layout maps (1 map per,
// group)
    pub /: *mut *mut __le16 flags; / Bit 0 set if encryption enabled,
pub const RAID_MAP_FLAG_ENCRYPT_ON: c_uint = 0x01;
    pub /: *mut *mut __le16 dekindex; / Data encryption key index.,
    pub reserved: [u8; 16],
    pub data: [raid_map_disk_data; RAID_MAP_MAX_ENTRIES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ReportLUNdata {
    pub LUNListLength: [u8; 4],
    pub extended_response_flag: u8,
    pub reserved: [u8; 3],
    pub LUN: [u8; HPSA_MAX_LUN][8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_report_lun_entry {
    pub lunid: [u8; 8],    pub wwid: [u8; 8],
    pub device_type: u8,
    pub device_flags: u8,
    pub /: *mut *mut u8 lun_count; / multi-lun device, how many luns,
    pub redundant_paths: u8,
    pub /: *mut *mut u32 ioaccel_handle; / ioaccel1 only uses lower 16 bits,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ReportExtendedLUNdata {
    pub LUNListLength: [u8; 4],
    pub extended_response_flag: u8,
    pub reserved: [u8; 3],
    pub LUN: [ext_report_lun_entry; HPSA_MAX_PHYS_LUN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SenseSubsystem_info {
    pub reserved: [u8; 36],
    pub portname: [u8; 8],
    pub reserved1: [u8; 1108],
    pub __packed: },
// BMIC commands
pub const BMIC_READ: c_uint = 0x26;
pub const BMIC_WRITE: c_uint = 0x27;
pub const BMIC_CACHE_FLUSH: c_uint = 0xc2;
pub const HPSA_CACHE_FLUSH: c_uint = 0x01	/* C2 was already being used by HPSA */;
pub const BMIC_FLASH_FIRMWARE: c_uint = 0xF7;
pub const BMIC_SENSE_CONTROLLER_PARAMETERS: c_uint = 0x64;
pub const BMIC_IDENTIFY_PHYSICAL_DEVICE: c_uint = 0x15;
pub const BMIC_IDENTIFY_CONTROLLER: c_uint = 0x11;
pub const BMIC_SET_DIAG_OPTIONS: c_uint = 0xF4;
pub const BMIC_SENSE_DIAG_OPTIONS: c_uint = 0xF5;
pub const HPSA_DIAG_OPTS_DISABLE_RLD_CACHING: c_uint = 0x80000000;
pub const BMIC_SENSE_SUBSYSTEM_INFORMATION: c_uint = 0x66;
pub const BMIC_SENSE_STORAGE_BOX_PARAMS: c_uint = 0x65;
// Command List Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub union SCSI3Addr {
    pub Dev: u8,
    pub Bus:6: u8,
    pub /: *mut *mut u8 Mode:2; / b00,
    pub PeripDev: },
    pub DevLSB: u8,
    pub DevMSB:6: u8,
    pub /: *mut *mut u8 Mode:2; / b01,
    pub LogDev: },
    pub Dev:5: u8,
    pub Bus:3: u8,
    pub Targ:6: u8,
    pub /: *mut *mut u8 Mode:2; / b10,
    pub LogUnit: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysDevAddr {
    pub TargetId:24: u32,
    pub Bus:6: u32,
    pub Mode:2: u32,
// 2 level target device addr
    pub Target: [SCSI3Addr; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LogDevAddr {
    pub VolId:30: u32,
    pub Mode:2: u32,
    pub reserved: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union LUNAddr {
    pub LunAddrBytes: [u8; 8],
    pub SCSI3Lun: [SCSI3Addr; 4],
    pub PhysDev: PhysDevAddr,
    pub LogDev: LogDevAddr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandListHeader {
    pub ReplyQueue: u8,
    pub SGList: u8,
    pub SGTotal: __le16,
    pub tag: __le64,
    pub LUN: LUNAddr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RequestBlock {
    pub CDBLen: u8,
//
// type_attr_dir:
// type: low 3 bits
// attr: middle 3 bits
// dir: high 2 bits
//
    pub type_attr_dir: u8,

    pub Timeout: u16,
    pub CDB: [u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ErrDescriptor {
    pub Addr: __le64,
    pub Len: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SGDescriptor {
    pub Addr: __le64,
    pub Len: __le32,
    pub Ext: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union MoreErrInfo {
    pub Reserved: [u8; 3],
    pub Type: u8,
    pub ErrorInfo: u32,
    pub Common_Info: },
    pub Reserved: [u8; 2],
    pub /: *mut *mut u8 offense_size; / size of offending entry,
    pub /: *mut *mut u8 offense_num; / byte # of offense 0-base,
    pub offense_value: u32,
    pub Invalid_Cmd: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ErrorInfo {
    pub ScsiStatus: u8,
    pub SenseLen: u8,
    pub CommandStatus: u16,
    pub ResidualCnt: u32,
    pub MoreErrInfo: MoreErrInfo,
    pub SenseInfo: [u8; SENSEINFOBYTES],
    pub __packed: },
// Command types
pub const CMD_IOCTL_PEND: c_uint = 0x01;
pub const CMD_SCSI: c_uint = 0x03;
pub const CMD_IOACCEL1: c_uint = 0x04;
pub const CMD_IOACCEL2: c_uint = 0x05;
pub const IOACCEL2_TMF: c_uint = 0x06;
pub const DIRECT_LOOKUP_SHIFT: c_int = 4;

pub const HPSA_ERROR_BIT: c_uint = 0x02;
    pub /: *mut *mut ctlr_info; / defined in hpsa.h,
// The size of this structure needs to be divisible by 128
// on all architectures.  The low 4 bits of the addresses
// are used as follows:
//
// bit 0: to device, used to indicate "performant mode" command
// from device, indidcates error status.
// bit 1-3: to device, indicates block fetch table entry for
// reducing DMA in fetching commands from host memory.
//
pub const COMMANDLIST_ALIGNMENT: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandList {
    pub Header: CommandListHeader,
    pub Request: RequestBlock,
    pub ErrDesc: ErrDescriptor,
    pub SG: [SGDescriptor; SG_ENTRIES_IN_CMD],
// information associated with the command
    pub /: *mut *mut u32 busaddr; / physical addr of this record,
    pub /: *mut *mut *mut ErrorInfo err_info; / pointer to the allocated mem,
    pub h: *mut ctlr_info,
    pub cmd_type: c_int,
    pub cmdindex: c_long,
    pub waiting: *mut completion,
    pub scsi_cmd: *mut scsi_cmnd,
    pub work: work_struct,
//
// For commands using either of the two "ioaccel" paths to
// bypass the RAID stack and go directly to the physical disk
// phys_disk is a pointer to the hpsa_scsi_dev_t to which the
// i/o is destined.  We need to store that here because the command
// may potentially encounter TASK SET FULL and need to be resubmitted
// For "normal" i/o's not using the "ioaccel" paths, phys_disk is
// not used.
//
    pub phys_disk: *mut hpsa_scsi_dev_t,
    pub retry_pending: bool,
    pub device: *mut hpsa_scsi_dev_t,
    pub /: *mut *mut atomic_t refcount; / Must be last to avoid memset in hpsa_cmd_init(),
    pub __aligned(COMMANDLIST_ALIGNMENT): },
//
// Make sure our embedded atomic variable is aligned. Otherwise we break atomic
// operations on architectures that don't support unaligned atomics like IA64.
//
// The assert guards against reintroductin against unwanted __packed to
// the struct CommandList.
//
    pub 0): static_assert(offsetof(struct CommandList, refcount) % __alignof__(atomic_t) ==,
// Max S/G elements in I/O accelerator command
pub const IOACCEL1_MAXSGENTRIES: c_int = 24;
pub const IOACCEL2_MAXSGENTRIES: c_int = 28;
//
// Structure for I/O accelerator (mode 1) commands.
// Note that this structure must be 128-byte aligned in size.
//
pub const IOACCEL1_COMMANDLIST_ALIGNMENT: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_accel1_cmd {
    pub /: *mut *mut __le16 dev_handle; / 0x00 - 0x01,
    pub /: *mut *mut u8 reserved1; / 0x02,
    pub /: *mut *mut u8 function; / 0x03,
    pub /: *mut *mut u8 reserved2[8]; / 0x04 - 0x0B,
    pub /: *mut *mut u32 err_info; / 0x0C - 0x0F,
    pub /: *mut *mut u8 reserved3[2]; / 0x10 - 0x11,
    pub /: *mut *mut u8 err_info_len; / 0x12,
    pub /: *mut *mut u8 reserved4; / 0x13,
    pub /: *mut *mut u8 sgl_offset; / 0x14,
    pub /: *mut *mut u8 reserved5[7]; / 0x15 - 0x1B,
    pub /: *mut *mut __le32 transfer_len; / 0x1C - 0x1F,
    pub /: *mut *mut u8 reserved6[4]; / 0x20 - 0x23,
    pub /: *mut *mut __le16 io_flags; / 0x24 - 0x25,
    pub /: *mut *mut u8 reserved7[14]; / 0x26 - 0x33,
    pub /: *mut *mut u8 LUN[8]; / 0x34 - 0x3B,
    pub /: *mut *mut __le32 control; / 0x3C - 0x3F,
    pub /: *mut *mut u8 CDB[16]; / 0x40 - 0x4F,
    pub /: *mut *mut u8 reserved8[16]; / 0x50 - 0x5F,
    pub /: *mut *mut __le16 host_context_flags; / 0x60 - 0x61,
    pub /: *mut *mut __le16 timeout_sec; / 0x62 - 0x63,
    pub /: *mut *mut u8 ReplyQueue; / 0x64,
    pub /: *mut *mut u8 reserved9[3]; / 0x65 - 0x67,
    pub /: *mut *mut __le64 tag; / 0x68 - 0x6F,
    pub /: *mut *mut __le64 host_addr; / 0x70 - 0x77,
    pub /: *mut *mut u8 CISS_LUN[8]; / 0x78 - 0x7F,
    pub SG: [SGDescriptor; IOACCEL1_MAXSGENTRIES],
    pub __aligned(IOACCEL1_COMMANDLIST_ALIGNMENT): } __packed,
pub const IOACCEL1_FUNCTION_SCSIIO: c_uint = 0x00;
pub const IOACCEL1_SGLOFFSET: c_int = 32;
pub const IOACCEL1_IOFLAGS_IO_REQ: c_uint = 0x4000;
pub const IOACCEL1_IOFLAGS_CDBLEN_MASK: c_uint = 0x001F;
pub const IOACCEL1_IOFLAGS_CDBLEN_MAX: c_int = 16;
pub const IOACCEL1_CONTROL_NODATAXFER: c_uint = 0x00000000;
pub const IOACCEL1_CONTROL_DATA_OUT: c_uint = 0x01000000;
pub const IOACCEL1_CONTROL_DATA_IN: c_uint = 0x02000000;
pub const IOACCEL1_CONTROL_TASKPRIO_MASK: c_uint = 0x00007800;
pub const IOACCEL1_CONTROL_TASKPRIO_SHIFT: c_int = 11;
pub const IOACCEL1_CONTROL_SIMPLEQUEUE: c_uint = 0x00000000;
pub const IOACCEL1_CONTROL_HEADOFQUEUE: c_uint = 0x00000100;
pub const IOACCEL1_CONTROL_ORDEREDQUEUE: c_uint = 0x00000200;
pub const IOACCEL1_CONTROL_ACA: c_uint = 0x00000400;
pub const IOACCEL1_HCFLAGS_CISS_FORMAT: c_uint = 0x0013;
pub const IOACCEL1_BUSADDR_CMDTYPE: c_uint = 0x00000060;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioaccel2_sg_element {
    pub address: __le64,
    pub length: __le32,
    pub reserved: [u8; 3],
    pub chain_indicator: u8,
pub const IOACCEL2_CHAIN: c_uint = 0x80;
pub const IOACCEL2_LAST_SG: c_uint = 0x40;
    pub __packed: },
//
// SCSI Response Format structure for IO Accelerator Mode 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_accel2_scsi_response {
    pub IU_type: u8,
pub const IOACCEL2_IU_TYPE_SRF: c_uint = 0x60;
    pub reserved1: [u8; 3],
    pub /: *mut *mut u8 req_id[4]; / request identifier,
    pub reserved2: [u8; 4],
    pub /: *mut *mut u8 serv_response; / service response,
pub const IOACCEL2_SERV_RESPONSE_COMPLETE: c_uint = 0x000;
pub const IOACCEL2_SERV_RESPONSE_FAILURE: c_uint = 0x001;
pub const IOACCEL2_SERV_RESPONSE_TMF_COMPLETE: c_uint = 0x002;
pub const IOACCEL2_SERV_RESPONSE_TMF_SUCCESS: c_uint = 0x003;
pub const IOACCEL2_SERV_RESPONSE_TMF_REJECTED: c_uint = 0x004;
pub const IOACCEL2_SERV_RESPONSE_TMF_WRONG_LUN: c_uint = 0x005;
    pub /: *mut *mut u8 status; / status,
pub const IOACCEL2_STATUS_SR_TASK_COMP_GOOD: c_uint = 0x00;
pub const IOACCEL2_STATUS_SR_TASK_COMP_CHK_COND: c_uint = 0x02;
pub const IOACCEL2_STATUS_SR_TASK_COMP_BUSY: c_uint = 0x08;
pub const IOACCEL2_STATUS_SR_TASK_COMP_RES_CON: c_uint = 0x18;
pub const IOACCEL2_STATUS_SR_TASK_COMP_SET_FULL: c_uint = 0x28;
pub const IOACCEL2_STATUS_SR_TASK_COMP_ABORTED: c_uint = 0x40;
pub const IOACCEL2_STATUS_SR_IOACCEL_DISABLED: c_uint = 0x0E;
pub const IOACCEL2_STATUS_SR_IO_ERROR: c_uint = 0x01;
pub const IOACCEL2_STATUS_SR_IO_ABORTED: c_uint = 0x02;
pub const IOACCEL2_STATUS_SR_NO_PATH_TO_DEVICE: c_uint = 0x03;
pub const IOACCEL2_STATUS_SR_INVALID_DEVICE: c_uint = 0x04;
pub const IOACCEL2_STATUS_SR_UNDERRUN: c_uint = 0x51;
pub const IOACCEL2_STATUS_SR_OVERRUN: c_uint = 0x75;
    pub /: *mut *mut u8 data_present; / low 2 bits,
pub const IOACCEL2_NO_DATAPRESENT: c_uint = 0x000;
pub const IOACCEL2_RESPONSE_DATAPRESENT: c_uint = 0x001;
pub const IOACCEL2_SENSE_DATA_PRESENT: c_uint = 0x002;
pub const IOACCEL2_RESERVED: c_uint = 0x003;
    pub /: *mut *mut u8 sense_data_len; / sense/response data length,
    pub /: *mut *mut u8 resid_cnt[4]; / residual count,
    pub /: *mut *mut u8 sense_data_buff[32]; / sense/response data buffer,
    pub __packed: },
//
// Structure for I/O accelerator (mode 2 or m2) commands.
// Note that this structure must be 128-byte aligned in size.
//
pub const IOACCEL2_COMMANDLIST_ALIGNMENT: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_accel2_cmd {
    pub /: *mut *mut u8 IU_type; / IU Type,
    pub /: *mut *mut u8 direction; / direction, memtype, and encryption,
pub const IOACCEL2_DIRECTION_MASK: c_uint = 0x03 /* bits 0,1: direction  */;
pub const IOACCEL2_DIRECTION_MEMTYPE_MASK: c_uint = 0x04 /* bit 2: memtype source/dest */;
// 0b=PCIe, 1b=DDR
pub const IOACCEL2_DIRECTION_ENCRYPT_MASK: c_uint = 0x08 /* bit 3: encryption flag */;
// 0=off, 1=on
    pub /: *mut *mut u8 reply_queue; / Reply Queue ID,
    pub /: *mut *mut u8 reserved1; / Reserved,
    pub /: *mut *mut __le32 scsi_nexus; / Device Handle,
    pub /: *mut *mut __le32 Tag; / cciss tag, lower 4 bytes only,
    pub /: *mut *mut __le32 tweak_lower; / Encryption tweak, lower 4 bytes,
    pub /: *mut *mut u8 cdb[16]; / SCSI Command Descriptor Block,
    pub /: *mut *mut u8 cciss_lun[8]; / 8 byte SCSI address,
    pub /: *mut *mut __le32 data_len; / Total bytes to transfer,
    pub /: *mut *mut u8 cmd_priority_task_attr; / priority and task attrs,
pub const IOACCEL2_PRIORITY_MASK: c_uint = 0x78;
pub const IOACCEL2_ATTR_MASK: c_uint = 0x07;
    pub /: *mut *mut u8 sg_count; / Number of sg elements,
    pub /: *mut *mut __le16 dekindex; / Data encryption key index,
    pub /: *mut *mut __le64 err_ptr; / Error Pointer,
    pub Length*/: *mut *mut __le32 err_len; / Error,
    pub /: *mut *mut __le32 tweak_upper; / Encryption tweak, upper 4 bytes,
    pub sg: [ioaccel2_sg_element; IOACCEL2_MAXSGENTRIES],
    pub error_data: io_accel2_scsi_response,
    pub __aligned(IOACCEL2_COMMANDLIST_ALIGNMENT): } __packed,
//
// defines for Mode 2 command struct
// FIXME: this can't be all I need mfm
//
pub const IOACCEL2_IU_TYPE: c_uint = 0x40;
pub const IOACCEL2_IU_TMF_TYPE: c_uint = 0x41;
pub const IOACCEL2_DIR_NO_DATA: c_uint = 0x00;
pub const IOACCEL2_DIR_DATA_IN: c_uint = 0x01;
pub const IOACCEL2_DIR_DATA_OUT: c_uint = 0x02;
pub const IOACCEL2_TMF_ABORT: c_uint = 0x01;
//
// SCSI Task Management Request format for Accelerator Mode 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpsa_tmf_struct {
    pub /: *mut *mut u8 iu_type; / Information Unit Type,
    pub /: *mut *mut u8 reply_queue; / Reply Queue ID,
    pub /: *mut *mut u8 tmf; / Task Management Function,
    pub /: *mut *mut u8 reserved1; / byte 3 Reserved,
    pub /: *mut *mut __le32 it_nexus; / SCSI I-T Nexus,
    pub /: *mut *mut u8 lun_id[8]; / LUN ID for TMF request,
    pub /: *mut *mut __le64 tag; / cciss tag associated w/ request,
    pub /: *mut *mut __le64 abort_tag; / cciss tag of SCSI cmd or TMF to abort,
    pub /: *mut *mut __le64 error_ptr; / Error Pointer,
    pub /: *mut *mut __le32 error_len; / Error Length,
    pub __aligned(IOACCEL2_COMMANDLIST_ALIGNMENT): } __packed,
// Configuration Table Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HostWrite {
    pub TransportRequest: __le32,
    pub command_pool_addr_hi: __le32,
    pub CoalIntDelay: __le32,
    pub CoalIntCount: __le32,
    pub __packed: },
pub const SIMPLE_MODE: c_uint = 0x02;
pub const PERFORMANT_MODE: c_uint = 0x04;
pub const MEMQ_MODE: c_uint = 0x08;
pub const IOACCEL_MODE_1: c_uint = 0x80;
pub const DRIVER_SUPPORT_UA_ENABLE: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CfgTable {
    pub Signature: [u8; 4],
    pub SpecValence: __le32,
    pub TransportSupport: __le32,
    pub TransportActive: __le32,
    pub HostWrite: HostWrite,
    pub CmdsOutMax: __le32,
    pub BusTypes: __le32,
    pub TransMethodOffset: __le32,
    pub ServerName: [u8; 16],
    pub HeartBeat: __le32,
    pub driver_support: __le32,
pub const ENABLE_SCSI_PREFETCH: c_uint = 0x100;
pub const ENABLE_UNIT_ATTN: c_uint = 0x01;
    pub MaxScatterGatherElements: __le32,
    pub MaxLogicalUnits: __le32,
    pub MaxPhysicalDevices: __le32,
    pub MaxPhysicalDrivesPerLogicalUnit: __le32,
    pub MaxPerformantModeCommands: __le32,
    pub MaxBlockFetch: __le32,
    pub PowerConservationSupport: __le32,
    pub PowerConservationEnable: __le32,
    pub TMFSupportFlags: __le32,
    pub TMFTagMask: [u8; 8],
    pub 0x70]: u8 reserved[0x78 -,
    pub /: *mut *mut __le32 misc_fw_support; / offset 0x78,
pub const MISC_FW_DOORBELL_RESET: c_uint = 0x02;
pub const MISC_FW_DOORBELL_RESET2: c_uint = 0x010;
pub const MISC_FW_RAID_OFFLOAD_BASIC: c_uint = 0x020;
pub const MISC_FW_EVENT_NOTIFY: c_uint = 0x080;
    pub driver_version: [u8; 32],
    pub max_cached_write_size: __le32,
    pub driver_scratchpad: [u8; 16],
    pub max_error_info_length: __le32,
    pub io_accel_max_embedded_sg_count: __le32,
    pub io_accel_request_size_offset: __le32,
    pub event_notify: __le32,

    pub clear_event_notify: __le32,
    pub __packed: },
pub const NUM_BLOCKFETCH_ENTRIES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TransTable_struct {
    pub BlockFetch: [__le32; NUM_BLOCKFETCH_ENTRIES],
    pub RepQSize: __le32,
    pub RepQCount: __le32,
    pub RepQCtrAddrLow32: __le32,
    pub RepQCtrAddrHigh32: __le32,
pub const MAX_REPLY_QUEUES: c_int = 64;
    pub RepQAddr: [vals32; MAX_REPLY_QUEUES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpsa_pci_info {
    pub bus: c_uchar,
    pub dev_fn: c_uchar,
    pub domain: c_ushort,
    pub board_id: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmic_identify_controller {
    pub /: *mut *mut u8 configured_logical_drive_count; / offset 0,
    pub pad1: [u8; 153],
    pub /: *mut *mut __le16 extended_logical_unit_count; / offset 154,
    pub pad2: [u8; 136],
    pub /: *mut *mut u8 controller_mode; / offset 292,
    pub pad3: [u8; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmic_identify_physical_device {
    pub /: *mut *mut u8 scsi_bus; / SCSI Bus number on controller,
    pub /: *mut *mut u8 scsi_id; / SCSI ID on this bus,
    pub /: *mut *mut __le16 block_size; / sector size in bytes,
    pub /: *mut *mut __le32 total_blocks; / number for sectors on drive,
    pub /: *mut *mut __le32 reserved_blocks; / controller reserved (RIS),
    pub /: *mut *mut u8 model[40]; / Physical Drive Model,
    pub /: *mut *mut u8 serial_number[40]; / Drive Serial Number,
    pub /: *mut *mut u8 firmware_revision[8]; / drive firmware revision,
    pub /: *mut *mut u8 scsi_inquiry_bits; / inquiry byte 7 bits,
    pub /: *mut *mut u8 compaq_drive_stamp; / 0 means drive not stamped,
    pub last_failure_reason: u8,
pub const BMIC_LAST_FAILURE_TOO_SMALL_IN_LOAD_CONFIG: c_uint = 0x01;
pub const BMIC_LAST_FAILURE_ERROR_ERASING_RIS: c_uint = 0x02;
pub const BMIC_LAST_FAILURE_ERROR_SAVING_RIS: c_uint = 0x03;
pub const BMIC_LAST_FAILURE_FAIL_DRIVE_COMMAND: c_uint = 0x04;
pub const BMIC_LAST_FAILURE_MARK_BAD_FAILED: c_uint = 0x05;
pub const BMIC_LAST_FAILURE_MARK_BAD_FAILED_IN_FINISH_REMAP: c_uint = 0x06;
pub const BMIC_LAST_FAILURE_TIMEOUT: c_uint = 0x07;
pub const BMIC_LAST_FAILURE_AUTOSENSE_FAILED: c_uint = 0x08;
pub const BMIC_LAST_FAILURE_MEDIUM_ERROR_1: c_uint = 0x09;
pub const BMIC_LAST_FAILURE_MEDIUM_ERROR_2: c_uint = 0x0a;
pub const BMIC_LAST_FAILURE_NOT_READY_BAD_SENSE: c_uint = 0x0b;
pub const BMIC_LAST_FAILURE_NOT_READY: c_uint = 0x0c;
pub const BMIC_LAST_FAILURE_HARDWARE_ERROR: c_uint = 0x0d;
pub const BMIC_LAST_FAILURE_ABORTED_COMMAND: c_uint = 0x0e;
pub const BMIC_LAST_FAILURE_WRITE_PROTECTED: c_uint = 0x0f;
pub const BMIC_LAST_FAILURE_SPIN_UP_FAILURE_IN_RECOVER: c_uint = 0x10;
pub const BMIC_LAST_FAILURE_REBUILD_WRITE_ERROR: c_uint = 0x11;
pub const BMIC_LAST_FAILURE_TOO_SMALL_IN_HOT_PLUG: c_uint = 0x12;
pub const BMIC_LAST_FAILURE_BUS_RESET_RECOVERY_ABORTED: c_uint = 0x13;
pub const BMIC_LAST_FAILURE_REMOVED_IN_HOT_PLUG: c_uint = 0x14;
pub const BMIC_LAST_FAILURE_INIT_REQUEST_SENSE_FAILED: c_uint = 0x15;
pub const BMIC_LAST_FAILURE_INIT_START_UNIT_FAILED: c_uint = 0x16;
pub const BMIC_LAST_FAILURE_INQUIRY_FAILED: c_uint = 0x17;
pub const BMIC_LAST_FAILURE_NON_DISK_DEVICE: c_uint = 0x18;
pub const BMIC_LAST_FAILURE_READ_CAPACITY_FAILED: c_uint = 0x19;
pub const BMIC_LAST_FAILURE_INVALID_BLOCK_SIZE: c_uint = 0x1a;
pub const BMIC_LAST_FAILURE_HOT_PLUG_REQUEST_SENSE_FAILED: c_uint = 0x1b;
pub const BMIC_LAST_FAILURE_HOT_PLUG_START_UNIT_FAILED: c_uint = 0x1c;
pub const BMIC_LAST_FAILURE_WRITE_ERROR_AFTER_REMAP: c_uint = 0x1d;
pub const BMIC_LAST_FAILURE_INIT_RESET_RECOVERY_ABORTED: c_uint = 0x1e;
pub const BMIC_LAST_FAILURE_DEFERRED_WRITE_ERROR: c_uint = 0x1f;
pub const BMIC_LAST_FAILURE_MISSING_IN_SAVE_RIS: c_uint = 0x20;
pub const BMIC_LAST_FAILURE_WRONG_REPLACE: c_uint = 0x21;
pub const BMIC_LAST_FAILURE_GDP_VPD_INQUIRY_FAILED: c_uint = 0x22;
pub const BMIC_LAST_FAILURE_GDP_MODE_SENSE_FAILED: c_uint = 0x23;
pub const BMIC_LAST_FAILURE_DRIVE_NOT_IN_48BIT_MODE: c_uint = 0x24;
pub const BMIC_LAST_FAILURE_DRIVE_TYPE_MIX_IN_HOT_PLUG: c_uint = 0x25;
pub const BMIC_LAST_FAILURE_DRIVE_TYPE_MIX_IN_LOAD_CFG: c_uint = 0x26;
pub const BMIC_LAST_FAILURE_PROTOCOL_ADAPTER_FAILED: c_uint = 0x27;
pub const BMIC_LAST_FAILURE_FAULTY_ID_BAY_EMPTY: c_uint = 0x28;
pub const BMIC_LAST_FAILURE_FAULTY_ID_BAY_OCCUPIED: c_uint = 0x29;
pub const BMIC_LAST_FAILURE_FAULTY_ID_INVALID_BAY: c_uint = 0x2a;
pub const BMIC_LAST_FAILURE_WRITE_RETRIES_FAILED: c_uint = 0x2b;
pub const BMIC_LAST_FAILURE_SMART_ERROR_REPORTED: c_uint = 0x37;
pub const BMIC_LAST_FAILURE_PHY_RESET_FAILED: c_uint = 0x38;
pub const BMIC_LAST_FAILURE_ONLY_ONE_CTLR_CAN_SEE_DRIVE: c_uint = 0x40;
pub const BMIC_LAST_FAILURE_KC_VOLUME_FAILED: c_uint = 0x41;
pub const BMIC_LAST_FAILURE_UNEXPECTED_REPLACEMENT: c_uint = 0x42;
pub const BMIC_LAST_FAILURE_OFFLINE_ERASE: c_uint = 0x80;
pub const BMIC_LAST_FAILURE_OFFLINE_TOO_SMALL: c_uint = 0x81;
pub const BMIC_LAST_FAILURE_OFFLINE_DRIVE_TYPE_MIX: c_uint = 0x82;
pub const BMIC_LAST_FAILURE_OFFLINE_ERASE_COMPLETE: c_uint = 0x83;
    pub flags: u8,
    pub more_flags: u8,
    pub /: *mut *mut u8 scsi_lun; / SCSI LUN for phys drive,
    pub yet_more_flags: u8,
    pub even_more_flags: u8,
    pub /: *mut *mut __le32 spi_speed_rules;/ SPI Speed data:Ultra disable diagnose,
    pub /: *mut *mut u8 phys_connector[2]; / connector number on controller,
    pub /: *mut *mut u8 phys_box_on_bus; / phys enclosure this drive resides,
    pub /: *mut *mut u8 phys_bay_in_box; / phys drv bay this drive resides,
    pub /: *mut *mut __le32 rpm; / Drive rotational speed in rpm,
    pub /: *mut *mut u8 device_type; / type of drive,
pub const BMIC_DEVICE_TYPE_CONTROLLER: c_uint = 0x07;
    pub /: *mut *mut u8 sata_version; / only valid when drive_type is SATA,
    pub big_total_block_count: __le64,
    pub ris_starting_lba: __le64,
    pub ris_size: __le32,
    pub wwid: [u8; 20],
    pub controller_phy_map: [u8; 32],
    pub phy_count: __le16,
    pub phy_connected_dev_type: [u8; 256],
    pub phy_to_drive_bay_num: [u8; 256],
    pub phy_to_attached_dev_index: [__le16; 256],
    pub box_index: u8,
    pub reserved: u8,
    pub extra_physical_drive_flags: __le16,
    pub negotiated_link_rate: [u8; 256],
    pub phy_to_phy_map: [u8; 256],
    pub redundant_path_present_map: u8,
    pub redundant_path_failure_map: u8,
    pub active_path_number: u8,
    pub alternate_paths_phys_connector: [__le16; 8],
    pub alternate_paths_phys_box_on_port: [u8; 8],
    pub multi_lun_device_lun_count: u8,
    pub minimum_good_fw_revision: [u8; 8],
    pub unique_inquiry_bytes: [u8; 20],
    pub current_temperature_degreesC: u8,
    pub temperature_threshold_degreesC: u8,
    pub max_temperature_degreesC: u8,
    pub /: *mut *mut *mut u8 logical_blocks_per_phys_block_exp; / phyblocksize = 5122^exp,
    pub current_queue_depth_limit: __le16,
    pub reserved_switch_stuff: [u8; 60],
    pub /: *mut *mut __le16 power_on_hours; / valid only if gas gauge supported,
    pub /: *mut *mut __le16 percent_endurance_used; / valid only if gas gauge supported.,

    pub drive_authentication: u8,

    pub smart_carrier_authentication: u8,

    pub smart_carrier_app_fw_version: u8,
    pub smart_carrier_bootloader_fw_version: u8,
    pub sanitize_support_flags: u8,
    pub drive_key_flags: u8,
    pub encryption_key_name: [u8; 64],
    pub misc_drive_flags: __le32,
    pub dek_index: __le16,
    pub hba_drive_encryption_flags: __le16,
    pub max_overwrite_time: __le16,
    pub max_block_erase_time: __le16,
    pub max_crypto_erase_time: __le16,
    pub device_connector_info: [u8; 5],
    pub connector_name: [u8; 8][8],
    pub page_83_id: [u8; 16],
    pub max_link_rate: [u8; 256],
    pub neg_phys_link_rate: [u8; 256],
    pub box_conn_name: [u8; 8],
    pub __attribute((aligned(512))): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmic_sense_subsystem_info {
    pub primary_slot_number: u8,
    pub reserved: [u8; 3],
    pub chasis_serial_number: [u8; 32],
    pub primary_world_wide_id: [u8; 8],
    pub /: *mut *mut u8 primary_array_serial_number[32]; / NULL terminated,
    pub /: *mut *mut u8 primary_cache_serial_number[32]; / NULL terminated,
    pub reserved_2: [u8; 8],
    pub secondary_array_serial_number: [u8; 32],
    pub secondary_cache_serial_number: [u8; 32],
    pub pad: [u8; 332],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmic_sense_storage_box_params {
    pub reserved: [u8; 36],
    pub inquiry_valid: u8,
    pub reserved_1: [u8; 68],
    pub phys_box_on_port: u8,
    pub reserved_2: [u8; 22],
    pub connection_info: u16,
    pub reserver_3: [u8; 84],
    pub phys_connector: [u8; 2],
    pub reserved_4: [u8; 296],
    pub __packed: },
