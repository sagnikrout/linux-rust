//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/scsi_proto.h
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
// This header file contains public constants and structures used by
// both the SCSI initiator and the SCSI target code.
//
// For documentation on the OPCODES, MESSAGES, and SENSE values,
// please consult the SCSI standard.
//

//
// SCSI opcodes
//
pub const TEST_UNIT_READY: c_uint = 0x00;
pub const REZERO_UNIT: c_uint = 0x01;
pub const REQUEST_SENSE: c_uint = 0x03;
pub const FORMAT_UNIT: c_uint = 0x04;
pub const READ_BLOCK_LIMITS: c_uint = 0x05;
pub const REASSIGN_BLOCKS: c_uint = 0x07;
pub const INITIALIZE_ELEMENT_STATUS: c_uint = 0x07;
pub const READ_6: c_uint = 0x08;
pub const WRITE_6: c_uint = 0x0a;
pub const SEEK_6: c_uint = 0x0b;
pub const READ_REVERSE: c_uint = 0x0f;
pub const WRITE_FILEMARKS: c_uint = 0x10;
pub const SPACE: c_uint = 0x11;
pub const INQUIRY: c_uint = 0x12;
pub const RECOVER_BUFFERED_DATA: c_uint = 0x14;
pub const MODE_SELECT: c_uint = 0x15;
pub const RESERVE_6: c_uint = 0x16;
pub const RELEASE_6: c_uint = 0x17;
pub const COPY: c_uint = 0x18;
pub const ERASE: c_uint = 0x19;
pub const MODE_SENSE: c_uint = 0x1a;
pub const START_STOP: c_uint = 0x1b;
pub const RECEIVE_DIAGNOSTIC: c_uint = 0x1c;
pub const SEND_DIAGNOSTIC: c_uint = 0x1d;
pub const ALLOW_MEDIUM_REMOVAL: c_uint = 0x1e;
pub const READ_FORMAT_CAPACITIES: c_uint = 0x23;
pub const SET_WINDOW: c_uint = 0x24;
pub const READ_CAPACITY: c_uint = 0x25;
pub const READ_10: c_uint = 0x28;
pub const WRITE_10: c_uint = 0x2a;
pub const SEEK_10: c_uint = 0x2b;
pub const POSITION_TO_ELEMENT: c_uint = 0x2b;
pub const WRITE_VERIFY: c_uint = 0x2e;
pub const VERIFY: c_uint = 0x2f;
pub const SEARCH_HIGH: c_uint = 0x30;
pub const SEARCH_EQUAL: c_uint = 0x31;
pub const SEARCH_LOW: c_uint = 0x32;
pub const SET_LIMITS: c_uint = 0x33;
pub const PRE_FETCH: c_uint = 0x34;
pub const READ_POSITION: c_uint = 0x34;
pub const SYNCHRONIZE_CACHE: c_uint = 0x35;
pub const LOCK_UNLOCK_CACHE: c_uint = 0x36;
pub const READ_DEFECT_DATA: c_uint = 0x37;
pub const MEDIUM_SCAN: c_uint = 0x38;
pub const COMPARE: c_uint = 0x39;
pub const COPY_VERIFY: c_uint = 0x3a;
pub const WRITE_BUFFER: c_uint = 0x3b;
pub const READ_BUFFER: c_uint = 0x3c;
pub const UPDATE_BLOCK: c_uint = 0x3d;
pub const READ_LONG: c_uint = 0x3e;
pub const WRITE_LONG: c_uint = 0x3f;
pub const CHANGE_DEFINITION: c_uint = 0x40;
pub const WRITE_SAME: c_uint = 0x41;
pub const UNMAP: c_uint = 0x42;
pub const READ_TOC: c_uint = 0x43;
pub const READ_HEADER: c_uint = 0x44;
pub const GET_EVENT_STATUS_NOTIFICATION: c_uint = 0x4a;
pub const LOG_SELECT: c_uint = 0x4c;
pub const LOG_SENSE: c_uint = 0x4d;
pub const XDWRITEREAD_10: c_uint = 0x53;
pub const MODE_SELECT_10: c_uint = 0x55;
pub const RESERVE_10: c_uint = 0x56;
pub const RELEASE_10: c_uint = 0x57;
pub const MODE_SENSE_10: c_uint = 0x5a;
pub const PERSISTENT_RESERVE_IN: c_uint = 0x5e;
pub const PERSISTENT_RESERVE_OUT: c_uint = 0x5f;
pub const VARIABLE_LENGTH_CMD: c_uint = 0x7f;
pub const REPORT_LUNS: c_uint = 0xa0;
pub const SECURITY_PROTOCOL_IN: c_uint = 0xa2;
pub const MAINTENANCE_IN: c_uint = 0xa3;
pub const MAINTENANCE_OUT: c_uint = 0xa4;
pub const MOVE_MEDIUM: c_uint = 0xa5;
pub const EXCHANGE_MEDIUM: c_uint = 0xa6;
pub const READ_12: c_uint = 0xa8;
pub const SERVICE_ACTION_OUT_12: c_uint = 0xa9;
pub const WRITE_12: c_uint = 0xaa;
pub const READ_MEDIA_SERIAL_NUMBER: c_uint = 0xab /* Obsolete with SPC-2 */;
pub const SERVICE_ACTION_IN_12: c_uint = 0xab;
pub const WRITE_VERIFY_12: c_uint = 0xae;
pub const VERIFY_12: c_uint = 0xaf;
pub const SEARCH_HIGH_12: c_uint = 0xb0;
pub const SEARCH_EQUAL_12: c_uint = 0xb1;
pub const SEARCH_LOW_12: c_uint = 0xb2;
pub const SECURITY_PROTOCOL_OUT: c_uint = 0xb5;
pub const READ_ELEMENT_STATUS: c_uint = 0xb8;
pub const SEND_VOLUME_TAG: c_uint = 0xb6;
pub const WRITE_LONG_2: c_uint = 0xea;
pub const EXTENDED_COPY: c_uint = 0x83;
pub const RECEIVE_COPY_RESULTS: c_uint = 0x84;
pub const ACCESS_CONTROL_IN: c_uint = 0x86;
pub const ACCESS_CONTROL_OUT: c_uint = 0x87;
pub const READ_16: c_uint = 0x88;
pub const COMPARE_AND_WRITE: c_uint = 0x89;
pub const WRITE_16: c_uint = 0x8a;
pub const READ_ATTRIBUTE: c_uint = 0x8c;
pub const WRITE_ATTRIBUTE: c_uint = 0x8d;
pub const WRITE_VERIFY_16: c_uint = 0x8e;
pub const VERIFY_16: c_uint = 0x8f;
pub const SYNCHRONIZE_CACHE_16: c_uint = 0x91;
pub const WRITE_SAME_16: c_uint = 0x93;
pub const ZBC_OUT: c_uint = 0x94;
pub const ZBC_IN: c_uint = 0x95;
pub const WRITE_ATOMIC_16: c_uint = 0x9c;
pub const SERVICE_ACTION_BIDIRECTIONAL: c_uint = 0x9d;
pub const SERVICE_ACTION_IN_16: c_uint = 0x9e;
pub const SERVICE_ACTION_OUT_16: c_uint = 0x9f;
// values for service action in
pub const SAI_READ_CAPACITY_16: c_uint = 0x10;
pub const SAI_GET_LBA_STATUS: c_uint = 0x12;
pub const SAI_REPORT_REFERRALS: c_uint = 0x13;
pub const SAI_GET_STREAM_STATUS: c_uint = 0x16;
pub const SAI_GET_PHYSICAL_ELEMENT_STATUS: c_uint = 0x17;
pub const SAI_REMOVE_ELEMENT_AND_TRUNCATE: c_uint = 0x18;
pub const SAI_RESTORE_ELEMENTS_AND_REBUILD: c_uint = 0x19;
pub const SAI_REMOVE_ELEMENT_AND_MODIFY_ZONES: c_uint = 0x1a;
// values for maintenance in
pub const MI_REPORT_IDENTIFYING_INFORMATION: c_uint = 0x05;
pub const MI_REPORT_TARGET_PGS: c_uint = 0x0a;
pub const MI_REPORT_ALIASES: c_uint = 0x0b;
pub const MI_REPORT_SUPPORTED_OPERATION_CODES: c_uint = 0x0c;
pub const MI_REPORT_SUPPORTED_TASK_MANAGEMENT_FUNCTIONS: c_uint = 0x0d;
pub const MI_REPORT_PRIORITY: c_uint = 0x0e;
pub const MI_REPORT_TIMESTAMP: c_uint = 0x0f;
pub const MI_MANAGEMENT_PROTOCOL_IN: c_uint = 0x10;
// value for MI_REPORT_TARGET_PGS ext header
pub const MI_EXT_HDR_PARAM_FMT: c_uint = 0x20;
// values for maintenance out
pub const MO_SET_IDENTIFYING_INFORMATION: c_uint = 0x06;
pub const MO_SET_TARGET_PGS: c_uint = 0x0a;
pub const MO_CHANGE_ALIASES: c_uint = 0x0b;
pub const MO_SET_PRIORITY: c_uint = 0x0e;
pub const MO_SET_TIMESTAMP: c_uint = 0x0f;
pub const MO_MANAGEMENT_PROTOCOL_OUT: c_uint = 0x10;
// values for ZBC_IN
pub const ZI_REPORT_ZONES: c_uint = 0x00;
// values for ZBC_OUT
pub const ZO_CLOSE_ZONE: c_uint = 0x01;
pub const ZO_FINISH_ZONE: c_uint = 0x02;
pub const ZO_OPEN_ZONE: c_uint = 0x03;
pub const ZO_RESET_WRITE_POINTER: c_uint = 0x04;
// values for PR in service action
pub const READ_KEYS: c_uint = 0x00;
pub const READ_RESERVATION: c_uint = 0x01;
pub const REPORT_CAPABILITES: c_uint = 0x02;
pub const READ_FULL_STATUS: c_uint = 0x03;
// values for variable length command
pub const XDREAD_32: c_uint = 0x03;
pub const XDWRITE_32: c_uint = 0x04;
pub const XPWRITE_32: c_uint = 0x06;
pub const XDWRITEREAD_32: c_uint = 0x07;
pub const READ_32: c_uint = 0x09;
pub const VERIFY_32: c_uint = 0x0a;
pub const WRITE_32: c_uint = 0x0b;
pub const WRITE_VERIFY_32: c_uint = 0x0c;
pub const WRITE_SAME_32: c_uint = 0x0d;
pub const ATA_32: c_uint = 0x1ff0;
// Values for T10/04-262r7
pub const ATA_16: c_uint = 0x85	/* 16-byte pass-thru */;
pub const ATA_12: c_uint = 0xa1	/* 12-byte pass-thru */;
// Vendor specific CDBs start here
pub const VENDOR_SPECIFIC_CDB: c_uint = 0xc0;
//
// SCSI command lengths
//
pub const SCSI_MAX_VARLEN_CDB_SIZE: c_int = 260;
// defined in T10 SCSI Primary Commands-2 (SPC2)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_varlen_cdb_hdr {
    pub /: *mut *mut __u8 opcode; / opcode always == VARIABLE_LENGTH_CMD,
    pub control: __u8,
    pub misc: [__u8; 5],
    pub /: *mut *mut __u8 additional_cdb_length; / total cdb length - 8,
    pub service_action: __be16,
// service specific data follows
}

//
// SCSI Architecture Model (SAM) Status codes. Taken from SAM-3 draft
// T10/1561-D Revision 4 Draft dated 7th November 2002.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sam_status {
    SAM_STAT_GOOD				= 0x00,
    SAM_STAT_CHECK_CONDITION		= 0x02,
    SAM_STAT_CONDITION_MET			= 0x04,
    SAM_STAT_BUSY				= 0x08,
    SAM_STAT_INTERMEDIATE			= 0x10,
    SAM_STAT_INTERMEDIATE_CONDITION_MET	= 0x14,
    SAM_STAT_RESERVATION_CONFLICT		= 0x18,
    SAM_STAT_COMMAND_TERMINATED		= 0x22,	/* obsolete in SAM-3 */
    SAM_STAT_TASK_SET_FULL			= 0x28,
    SAM_STAT_ACA_ACTIVE			= 0x30,
    SAM_STAT_TASK_ABORTED			= 0x40,
}

pub const STATUS_MASK: c_uint = 0xfe;
//
// SENSE KEYS
//
pub const NO_SENSE: c_uint = 0x00;
pub const RECOVERED_ERROR: c_uint = 0x01;
pub const NOT_READY: c_uint = 0x02;
pub const MEDIUM_ERROR: c_uint = 0x03;
pub const HARDWARE_ERROR: c_uint = 0x04;
pub const ILLEGAL_REQUEST: c_uint = 0x05;
pub const UNIT_ATTENTION: c_uint = 0x06;
pub const DATA_PROTECT: c_uint = 0x07;
pub const BLANK_CHECK: c_uint = 0x08;
pub const VENDOR_SPECIFIC: c_uint = 0x09;
pub const COPY_ABORTED: c_uint = 0x0a;
pub const ABORTED_COMMAND: c_uint = 0x0b;
pub const VOLUME_OVERFLOW: c_uint = 0x0d;
pub const MISCOMPARE: c_uint = 0x0e;
pub const COMPLETED: c_uint = 0x0f;
//
// Additional Sense Codes (ASC).
//
pub const NO_ADDITIONAL_SENSE: c_uint = 0x00;
pub const OVERLAP_ATOMIC_COMMAND_ASC: c_uint = 0x00;
pub const LOGICAL_UNIT_NOT_READY: c_uint = 0x04;
pub const LOGICAL_UNIT_COMMUNICATION_FAILURE: c_uint = 0x8;
pub const WRITE_ERROR_ASC: c_uint = 0x0c;
pub const UNRECOVERED_READ_ERR: c_uint = 0x11;
pub const PARAMETER_LIST_LENGTH_ERR: c_uint = 0x1a;
pub const MISCOMPARE_VERIFY_ASC: c_uint = 0x1d;
pub const INVALID_OPCODE: c_uint = 0x20;
pub const LBA_OUT_OF_RANGE: c_uint = 0x21;
pub const INVALID_FIELD_IN_CDB: c_uint = 0x24;
pub const INVALID_FIELD_IN_PARAM_LIST: c_uint = 0x26;
pub const WRITE_PROTECTED: c_uint = 0x27;
pub const UA_READY_ASC: c_uint = 0x28;
pub const UA_RESET_ASC: c_uint = 0x29;
pub const UA_CHANGED_ASC: c_uint = 0x2a;
pub const TOO_MANY_IN_PARTITION_ASC: c_uint = 0x3b;
pub const TARGET_CHANGED_ASC: c_uint = 0x3f;
pub const SAVING_PARAMS_UNSUP: c_uint = 0x39;
pub const TRANSPORT_PROBLEM: c_uint = 0x4b;
pub const INSUFF_RES_ASC: c_uint = 0x55;
pub const LOW_POWER_COND_ON: c_uint = 0x5e;
pub const THRESHOLD_EXCEEDED: c_uint = 0x5d;
//
// Additional Sense Code Qualifiers (ASCQ).
//
pub const POWER_ON_RESET_ASCQ: c_uint = 0x00;
pub const MODE_CHANGED_ASCQ: c_uint = 0x01	/* mode parameters changed */;
pub const FILEMARK_DETECTED_ASCQ: c_uint = 0x01;
pub const POWER_ON_OCCURRED_ASCQ: c_uint = 0x01;
pub const MICROCODE_CHANGED_ASCQ: c_uint = 0x01	/* with TARGET_CHANGED_ASC */;
pub const BUS_RESET_ASCQ: c_uint = 0x02	/* scsi bus reset occurred */;
pub const EOP_EOM_DETECTED_ASCQ: c_uint = 0x02;
pub const INSUFF_RES_ASCQ: c_uint = 0x03;
pub const BEGINNING_OF_P_M_DETECTED_ASCQ: c_uint = 0x04;
pub const UNALIGNED_WRITE_ASCQ: c_uint = 0x04;
pub const EOD_DETECTED_ASCQ: c_uint = 0x05;
pub const WRITE_BOUNDARY_ASCQ: c_uint = 0x05;
pub const READ_INVDATA_ASCQ: c_uint = 0x06;
pub const READ_BOUNDARY_ASCQ: c_uint = 0x07;
pub const CAPACITY_CHANGED_ASCQ: c_uint = 0x09;
pub const ATTEMPT_ACCESS_GAP: c_uint = 0x09;
pub const LUNS_CHANGED_ASCQ: c_uint = 0x0e;
pub const INSUFF_ZONE_ASCQ: c_uint = 0x0e;
pub const MICROCODE_CHANGED_WO_RESET_ASCQ: c_uint = 0x16;
pub const OVERLAP_ATOMIC_COMMAND_ASCQ: c_uint = 0x23;
//
// DEVICE TYPES
// Please keep them in 0x%02x format for $MODALIAS to work
//
pub const TYPE_DISK: c_uint = 0x00;
pub const TYPE_TAPE: c_uint = 0x01;
pub const TYPE_PRINTER: c_uint = 0x02;
pub const TYPE_PROCESSOR: c_uint = 0x03    /* HP scanners use this */;
pub const TYPE_WORM: c_uint = 0x04    /* Treated as ROM by our system */;
pub const TYPE_ROM: c_uint = 0x05;
pub const TYPE_SCANNER: c_uint = 0x06;
pub const TYPE_MOD: c_uint = 0x07    /* Magneto-optical disk -;
// - treated as TYPE_DISK
pub const TYPE_MEDIUM_CHANGER: c_uint = 0x08;
pub const TYPE_COMM: c_uint = 0x09    /* Communications device */;
pub const TYPE_RAID: c_uint = 0x0c;
pub const TYPE_ENCLOSURE: c_uint = 0x0d    /* Enclosure Services Device */;
pub const TYPE_RBC: c_uint = 0x0e;
pub const TYPE_OSD: c_uint = 0x11;
pub const TYPE_ZBC: c_uint = 0x14;
pub const TYPE_WLUN: c_uint = 0x1e    /* well-known logical unit */;
pub const TYPE_NO_LUN: c_uint = 0x7f;
// SCSI protocols; these are taken from SPC-3 section 7.5
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_protocol {
    SCSI_PROTOCOL_FCP = 0,	/* Fibre Channel */
    SCSI_PROTOCOL_SPI = 1,	/* parallel SCSI */
    SCSI_PROTOCOL_SSA = 2,	/* Serial Storage Architecture - Obsolete */
    SCSI_PROTOCOL_SBP = 3,	/* firewire */
    SCSI_PROTOCOL_SRP = 4,	/* Infiniband RDMA */
    SCSI_PROTOCOL_ISCSI = 5,
    SCSI_PROTOCOL_SAS = 6,
    SCSI_PROTOCOL_ADT = 7,	/* Media Changers */
    SCSI_PROTOCOL_ATA = 8,
    SCSI_PROTOCOL_UNSPEC = 0xf, /* No specific protocol */
}

//
// ScsiLun: 8 byte LUN.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_lun {
    pub scsi_lun: [__u8; 8],
}

// SBC-5 IO advice hints group descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_io_group_descriptor {

    pub 2: u8 io_advice_hints_mode:,
    pub 3: u8 reserved1:,
    pub 1: u8 st_enble:,
    pub 1: u8 cs_enble:,
    pub 1: u8 ic_enable:,

    pub 1: u8 ic_enable:,
    pub 1: u8 cs_enble:,
    pub 1: u8 st_enble:,
    pub 3: u8 reserved1:,
    pub 2: u8 io_advice_hints_mode:,
    pub reserved2: [u8; 3],
// Logical block markup descriptor

    pub 1: u8 acdlu:,
    pub 1: u8 reserved3:,
    pub 2: u8 rlbsr:,
    pub 4: u8 lbm_descriptor_type:,

    pub 4: u8 lbm_descriptor_type:,
    pub 2: u8 rlbsr:,
    pub 1: u8 reserved3:,
    pub 1: u8 acdlu:,
    pub params: [u8; 2],
    pub reserved4: u8,
    pub reserved5: [u8; 8],
}

// SCSI stream status descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_stream_status {

    pub 1: u8 perm:,
    pub 7: u8 reserved1:,

    pub 7: u8 reserved1:,
    pub 1: u8 perm:,

    pub reserved2: u8,
    pub stream_identifier: __be16,

    pub 2: u8 reserved3:,
    pub 6: u8 rel_lifetime:,

    pub 6: u8 rel_lifetime:,
    pub 2: u8 reserved3:,
    pub reserved4: [u8; 3],
}

// GET STREAM STATUS parameter data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_stream_status_header {
    pub /: *mut *mut __be32 len; / length in bytes of following payload,
    pub reserved: u16,
    pub number_of_open_streams: __be16,
}

// SPC asymmetric access states
pub const SCSI_ACCESS_STATE_OPTIMAL: c_uint = 0x00;
pub const SCSI_ACCESS_STATE_ACTIVE: c_uint = 0x01;
pub const SCSI_ACCESS_STATE_STANDBY: c_uint = 0x02;
pub const SCSI_ACCESS_STATE_UNAVAILABLE: c_uint = 0x03;
pub const SCSI_ACCESS_STATE_LBA: c_uint = 0x04;
pub const SCSI_ACCESS_STATE_OFFLINE: c_uint = 0x0e;
pub const SCSI_ACCESS_STATE_TRANSITIONING: c_uint = 0x0f;
// Values for REPORT TARGET GROUP STATES
pub const SCSI_ACCESS_STATE_MASK: c_uint = 0x0f;
pub const SCSI_ACCESS_STATE_PREFERRED: c_uint = 0x80;
// Reporting options for REPORT ZONES
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zbc_zone_reporting_options {
    ZBC_ZONE_REPORTING_OPTION_ALL		= 0x00,
    ZBC_ZONE_REPORTING_OPTION_EMPTY		= 0x01,
    ZBC_ZONE_REPORTING_OPTION_IMPLICIT_OPEN	= 0x02,
    ZBC_ZONE_REPORTING_OPTION_EXPLICIT_OPEN	= 0x03,
    ZBC_ZONE_REPORTING_OPTION_CLOSED	= 0x04,
    ZBC_ZONE_REPORTING_OPTION_FULL		= 0x05,
    ZBC_ZONE_REPORTING_OPTION_READONLY	= 0x06,
    ZBC_ZONE_REPORTING_OPTION_OFFLINE	= 0x07,
// 0x08 to 0x0f are reserved
    ZBC_ZONE_REPORTING_OPTION_NEED_RESET_WP	= 0x10,
    ZBC_ZONE_REPORTING_OPTION_NON_SEQWRITE	= 0x11,
// 0x12 to 0x3e are reserved
    ZBC_ZONE_REPORTING_OPTION_NON_WP	= 0x3f,
}

pub const ZBC_REPORT_ZONE_PARTIAL: c_uint = 0x80;
// Zone types of REPORT ZONES zone descriptors
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zbc_zone_type {
    ZBC_ZONE_TYPE_CONV		= 0x1,
    ZBC_ZONE_TYPE_SEQWRITE_REQ	= 0x2,
    ZBC_ZONE_TYPE_SEQWRITE_PREF	= 0x3,
    ZBC_ZONE_TYPE_SEQ_OR_BEFORE_REQ	= 0x4,
    ZBC_ZONE_TYPE_GAP		= 0x5,
// 0x6 to 0xf are reserved
}

// Zone conditions of REPORT ZONES zone descriptors
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zbc_zone_cond {
    ZBC_ZONE_COND_NO_WP		= 0x0,
    ZBC_ZONE_COND_EMPTY		= 0x1,
    ZBC_ZONE_COND_IMP_OPEN		= 0x2,
    ZBC_ZONE_COND_EXP_OPEN		= 0x3,
    ZBC_ZONE_COND_CLOSED		= 0x4,
// 0x5 to 0xc are reserved
    ZBC_ZONE_COND_READONLY		= 0xd,
    ZBC_ZONE_COND_FULL		= 0xe,
    ZBC_ZONE_COND_OFFLINE		= 0xf,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zbc_zone_alignment_method {
    ZBC_CONSTANT_ZONE_LENGTH	= 0x1,
    ZBC_CONSTANT_ZONE_START_OFFSET	= 0x8,
}

// SCSI physical element types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_phys_element_type {
    SCSI_PHYS_ELEM_TYPE_ALL_ACCESS_STORAGE	= 0x1,
    SCSI_PHYS_ELEM_TYPE_FRAC_ACCESS_STORAGE	= 0x2,
}

// SCSI physical element health.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_phys_element_health {
    SCSI_PHYS_ELEM_HEALTH_NOT_REPORTED		= 0x00,
    SCSI_PHYS_ELEM_HEALTH_WITHIN_SPEC_LIMITS	= 0x01,
    SCSI_PHYS_ELEM_HEALTH_AT_SPEC_LIMITS		= 0x64,
    SCSI_PHYS_ELEM_HEALTH_OUTSIDE_SPEC_LIMITS	= 0x65,
    SCSI_PHYS_ELEM_HEALTH_DEPOP_REVOKE_ERR		= 0xFB,
    SCSI_PHYS_ELEM_HEALTH_DEPOP_REVOKE_IN_PROGRESS	= 0xFC,
    SCSI_PHYS_ELEM_HEALTH_DEPOP_ERR			= 0xFD,
    SCSI_PHYS_ELEM_HEALTH_DEPOP_IN_PROGRESS		= 0xFE,
    SCSI_PHYS_ELEM_HEALTH_DEPOP_OK			= 0xFF,
}

// Version descriptor values for INQUIRY
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_version_descriptor {
    SCSI_VERSION_DESCRIPTOR_FCP4	= 0x0a40,
    SCSI_VERSION_DESCRIPTOR_ISCSI	= 0x0960,
    SCSI_VERSION_DESCRIPTOR_SAM5	= 0x00a0,
    SCSI_VERSION_DESCRIPTOR_SAS3	= 0x0c60,
    SCSI_VERSION_DESCRIPTOR_SBC3	= 0x04c0,
    SCSI_VERSION_DESCRIPTOR_SBP3	= 0x0980,
    SCSI_VERSION_DESCRIPTOR_SPC4	= 0x0460,
    SCSI_VERSION_DESCRIPTOR_SRP	= 0x0940
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_support_opcode {
    SCSI_SUPPORT_NO_INFO		= 0,
    SCSI_SUPPORT_NOT_SUPPORTED	= 1,
    SCSI_SUPPORT_FULL		= 3,
    SCSI_SUPPORT_VENDOR		= 5,
}

pub const SCSI_CONTROL_MASK: c_int = 0;
pub const SCSI_GROUP_NUMBER_MASK: c_int = 0;
