//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/3w-sas.h
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
// AEN severity table
// Liberator register offsets
pub const TWL_STATUS: c_uint = 0x0  /* Status */;
pub const TWL_HIBDB: c_uint = 0x20 /* Inbound doorbell */;
pub const TWL_HISTAT: c_uint = 0x30 /* Host interrupt status */;
pub const TWL_HIMASK: c_uint = 0x34 /* Host interrupt mask */;
pub const TWL_HOBDB: c_uint = 0x9C /* Outbound doorbell */;
pub const TWL_HOBDBC: c_uint = 0xA0 /* Outbound doorbell clear */;
pub const TWL_SCRPD3: c_uint = 0xBC /* Scratchpad */;
pub const TWL_HIBQPL: c_uint = 0xC0 /* Host inbound Q low */;
pub const TWL_HIBQPH: c_uint = 0xC4 /* Host inbound Q high */;
pub const TWL_HOBQPL: c_uint = 0xC8 /* Host outbound Q low */;
pub const TWL_HOBQPH: c_uint = 0xCC /* Host outbound Q high */;
pub const TWL_HISTATUS_VALID_INTERRUPT: c_uint = 0xC;
pub const TWL_HISTATUS_ATTENTION_INTERRUPT: c_uint = 0x4;
pub const TWL_HISTATUS_RESPONSE_INTERRUPT: c_uint = 0x8;
pub const TWL_STATUS_OVERRUN_SUBMIT: c_uint = 0x2000;
pub const TWL_ISSUE_SOFT_RESET: c_uint = 0x100;
pub const TWL_CONTROLLER_READY: c_uint = 0x2000;
pub const TWL_DOORBELL_CONTROLLER_ERROR: c_uint = 0x200000;
pub const TWL_DOORBELL_ATTENTION_INTERRUPT: c_uint = 0x40000;
pub const TWL_PULL_MODE: c_uint = 0x1;
// Command packet opcodes used by the driver
pub const TW_OP_INIT_CONNECTION: c_uint = 0x1;
pub const TW_OP_GET_PARAM: c_uint = 0x12;
pub const TW_OP_SET_PARAM: c_uint = 0x13;
pub const TW_OP_EXECUTE_SCSI: c_uint = 0x10;
// Asynchronous Event Notification (AEN) codes used by the driver
pub const TW_AEN_QUEUE_EMPTY: c_uint = 0x0000;
pub const TW_AEN_SOFT_RESET: c_uint = 0x0001;
pub const TW_AEN_SYNC_TIME_WITH_HOST: c_uint = 0x031;
pub const TW_AEN_SEVERITY_ERROR: c_uint = 0x1;
pub const TW_AEN_SEVERITY_DEBUG: c_uint = 0x4;
pub const TW_AEN_NOT_RETRIEVED: c_uint = 0x1;
// Command state defines
pub const TW_S_INITIAL: c_uint = 0x1  /* Initial state */;
pub const TW_S_STARTED: c_uint = 0x2  /* Id in use */;
pub const TW_S_POSTED: c_uint = 0x4  /* Posted to the controller */;
pub const TW_S_COMPLETED: c_uint = 0x8  /* Completed by isr */;
pub const TW_S_FINISHED: c_uint = 0x10 /* I/O completely done */;
// Compatibility defines
pub const TW_9750_ARCH_ID: c_int = 10;
pub const TW_CURRENT_DRIVER_SRL: c_int = 40;
pub const TW_CURRENT_DRIVER_BUILD: c_int = 0;
pub const TW_CURRENT_DRIVER_BRANCH: c_int = 0;
// Misc defines
pub const TW_SECTOR_SIZE: c_int = 512;
pub const TW_MAX_UNITS: c_int = 32;
pub const TW_INIT_MESSAGE_CREDITS: c_uint = 0x100;
pub const TW_INIT_COMMAND_PACKET_SIZE: c_uint = 0x3;
pub const TW_INIT_COMMAND_PACKET_SIZE_EXTENDED: c_uint = 0x6;
pub const TW_EXTENDED_INIT_CONNECT: c_uint = 0x2;
pub const TW_BASE_FW_SRL: c_int = 24;
pub const TW_BASE_FW_BRANCH: c_int = 0;
pub const TW_BASE_FW_BUILD: c_int = 1;
pub const TW_Q_LENGTH: c_int = 256;
pub const TW_Q_START: c_int = 0;
pub const TW_MAX_SLOT: c_int = 32;
pub const TW_MAX_RESET_TRIES: c_int = 2;
pub const TW_MAX_CMDS_PER_LUN: c_int = 254;
pub const TW_MAX_AEN_DRAIN: c_int = 255;
pub const TW_IN_RESET: c_int = 2;
pub const TW_USING_MSI: c_int = 3;
pub const TW_IN_ATTENTION_LOOP: c_int = 4;
pub const TW_MAX_SECTORS: c_int = 256;
pub const TW_MAX_CDB_LEN: c_int = 16;

pub const TW_VERSION_TABLE: c_uint = 0x0402;
pub const TW_TIMEKEEP_TABLE: c_uint = 0x040A;
pub const TW_INFORMATION_TABLE: c_uint = 0x0403;
pub const TW_PARAM_FWVER: c_int = 3;
pub const TW_PARAM_FWVER_LENGTH: c_int = 16;
pub const TW_PARAM_BIOSVER: c_int = 4;
pub const TW_PARAM_BIOSVER_LENGTH: c_int = 16;
pub const TW_PARAM_MODEL: c_int = 8;
pub const TW_PARAM_MODEL_LENGTH: c_int = 16;
pub const TW_PARAM_PHY_SUMMARY_TABLE: c_int = 1;
pub const TW_PARAM_PHYCOUNT: c_int = 2;
pub const TW_PARAM_PHYCOUNT_LENGTH: c_int = 1;
pub const TW_IOCTL_FIRMWARE_PASS_THROUGH: c_uint = 0x108  // Used by smartmontools;
pub const TW_ALLOCATION_LENGTH: c_int = 128;
pub const TW_SENSE_DATA_LENGTH: c_int = 18;
pub const TW_ERROR_LOGICAL_UNIT_NOT_SUPPORTED: c_uint = 0x10a;
pub const TW_ERROR_INVALID_FIELD_IN_CDB: c_uint = 0x10d;
pub const TW_ERROR_UNIT_OFFLINE: c_uint = 0x128;
pub const TW_MESSAGE_SOURCE_CONTROLLER_ERROR: c_int = 3;
pub const TW_MESSAGE_SOURCE_CONTROLLER_EVENT: c_int = 4;
pub const TW_DRIVER: c_int = 6;

pub const PCI_DEVICE_ID_3WARE_9750: c_uint = 0x1010;

// Bitmask macros to eliminate bitfields
// opcode: 5, reserved: 3

// opcode: 5, sgloffset: 3

// severity: 3, reserved: 5

// not_mfa: 1, reserved: 7, status: 8, request_id: 16

// request_id: 12, lun: 4

// Register access macros

// Macros

pub const TW_MAX_LUNS: c_int = 16;

pub const TW_PADDING_LENGTH_LIBERATOR: c_int = 136;
pub const TW_PADDING_LENGTH_LIBERATOR_OLD: c_int = 132;

// SGL entry
// Old Command Packet with ISO SGL
// Second DWORD
// New Command Packet with ISO SGL
// New command packet header
// This struct is a union of the 2 command packets
// Initconnection structure
// Event info structure
// GetParam descriptor
// Compatibility information structure

