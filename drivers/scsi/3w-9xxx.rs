//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/3w-9xxx.h
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
// AEN string type
// AEN strings
// AEN severity table
// Error strings
// Control register bit definitions
pub const TW_CONTROL_CLEAR_HOST_INTERRUPT: c_uint = 0x00080000;
pub const TW_CONTROL_CLEAR_ATTENTION_INTERRUPT: c_uint = 0x00040000;
pub const TW_CONTROL_MASK_COMMAND_INTERRUPT: c_uint = 0x00020000;
pub const TW_CONTROL_MASK_RESPONSE_INTERRUPT: c_uint = 0x00010000;
pub const TW_CONTROL_UNMASK_COMMAND_INTERRUPT: c_uint = 0x00008000;
pub const TW_CONTROL_UNMASK_RESPONSE_INTERRUPT: c_uint = 0x00004000;
pub const TW_CONTROL_CLEAR_ERROR_STATUS: c_uint = 0x00000200;
pub const TW_CONTROL_ISSUE_SOFT_RESET: c_uint = 0x00000100;
pub const TW_CONTROL_ENABLE_INTERRUPTS: c_uint = 0x00000080;
pub const TW_CONTROL_DISABLE_INTERRUPTS: c_uint = 0x00000040;
pub const TW_CONTROL_ISSUE_HOST_INTERRUPT: c_uint = 0x00000020;
pub const TW_CONTROL_CLEAR_PARITY_ERROR: c_uint = 0x00800000;
pub const TW_CONTROL_CLEAR_QUEUE_ERROR: c_uint = 0x00400000;
pub const TW_CONTROL_CLEAR_PCI_ABORT: c_uint = 0x00100000;
// Status register bit definitions
pub const TW_STATUS_MAJOR_VERSION_MASK: c_uint = 0xF0000000;
pub const TW_STATUS_MINOR_VERSION_MASK: c_uint = 0x0F000000;
pub const TW_STATUS_PCI_PARITY_ERROR: c_uint = 0x00800000;
pub const TW_STATUS_QUEUE_ERROR: c_uint = 0x00400000;
pub const TW_STATUS_MICROCONTROLLER_ERROR: c_uint = 0x00200000;
pub const TW_STATUS_PCI_ABORT: c_uint = 0x00100000;
pub const TW_STATUS_HOST_INTERRUPT: c_uint = 0x00080000;
pub const TW_STATUS_ATTENTION_INTERRUPT: c_uint = 0x00040000;
pub const TW_STATUS_COMMAND_INTERRUPT: c_uint = 0x00020000;
pub const TW_STATUS_RESPONSE_INTERRUPT: c_uint = 0x00010000;
pub const TW_STATUS_COMMAND_QUEUE_FULL: c_uint = 0x00008000;
pub const TW_STATUS_RESPONSE_QUEUE_EMPTY: c_uint = 0x00004000;
pub const TW_STATUS_MICROCONTROLLER_READY: c_uint = 0x00002000;
pub const TW_STATUS_COMMAND_QUEUE_EMPTY: c_uint = 0x00001000;
pub const TW_STATUS_EXPECTED_BITS: c_uint = 0x00002000;
pub const TW_STATUS_UNEXPECTED_BITS: c_uint = 0x00F00000;
pub const TW_STATUS_VALID_INTERRUPT: c_uint = 0x00DF0000;
// PCI related defines
pub const TW_PCI_CLEAR_PARITY_ERRORS: c_uint = 0xc100;
pub const TW_PCI_CLEAR_PCI_ABORT: c_uint = 0x2000;
// Command packet opcodes used by the driver
pub const TW_OP_INIT_CONNECTION: c_uint = 0x1;
pub const TW_OP_GET_PARAM: c_uint = 0x12;
pub const TW_OP_SET_PARAM: c_uint = 0x13;
pub const TW_OP_EXECUTE_SCSI: c_uint = 0x10;
pub const TW_OP_DOWNLOAD_FIRMWARE: c_uint = 0x16;
pub const TW_OP_RESET: c_uint = 0x1C;
// Asynchronous Event Notification (AEN) codes used by the driver
pub const TW_AEN_QUEUE_EMPTY: c_uint = 0x0000;
pub const TW_AEN_SOFT_RESET: c_uint = 0x0001;
pub const TW_AEN_SYNC_TIME_WITH_HOST: c_uint = 0x031;
pub const TW_AEN_SEVERITY_ERROR: c_uint = 0x1;
pub const TW_AEN_SEVERITY_DEBUG: c_uint = 0x4;
pub const TW_AEN_NOT_RETRIEVED: c_uint = 0x1;
pub const TW_AEN_RETRIEVED: c_uint = 0x2;
// Command state defines
pub const TW_S_INITIAL: c_uint = 0x1  /* Initial state */;
pub const TW_S_STARTED: c_uint = 0x2  /* Id in use */;
pub const TW_S_POSTED: c_uint = 0x4  /* Posted to the controller */;
pub const TW_S_PENDING: c_uint = 0x8  /* Waiting to be posted in isr */;
pub const TW_S_COMPLETED: c_uint = 0x10 /* Completed by isr */;
pub const TW_S_FINISHED: c_uint = 0x20 /* I/O completely done */;
// Compatibility defines
pub const TW_9000_ARCH_ID: c_uint = 0x5;
pub const TW_CURRENT_DRIVER_SRL: c_int = 35;
pub const TW_CURRENT_DRIVER_BUILD: c_int = 0;
pub const TW_CURRENT_DRIVER_BRANCH: c_int = 0;
// Misc defines
pub const TW_9550SX_DRAIN_COMPLETED: c_uint = 0xFFFF;
pub const TW_SECTOR_SIZE: c_int = 512;

pub const TW_ALIGNMENT_9000_SGL: c_uint = 0x3;
pub const TW_MAX_UNITS: c_int = 16;
pub const TW_MAX_UNITS_9650SE: c_int = 32;
pub const TW_INIT_MESSAGE_CREDITS: c_uint = 0x100;
pub const TW_INIT_COMMAND_PACKET_SIZE: c_uint = 0x3;
pub const TW_INIT_COMMAND_PACKET_SIZE_EXTENDED: c_uint = 0x6;
pub const TW_EXTENDED_INIT_CONNECT: c_uint = 0x2;
pub const TW_BUNDLED_FW_SAFE_TO_FLASH: c_uint = 0x4;
pub const TW_CTLR_FW_RECOMMENDS_FLASH: c_uint = 0x8;
pub const TW_CTLR_FW_COMPATIBLE: c_uint = 0x2;
pub const TW_BASE_FW_SRL: c_int = 24;
pub const TW_BASE_FW_BRANCH: c_int = 0;
pub const TW_BASE_FW_BUILD: c_int = 1;
pub const TW_FW_SRL_LUNS_SUPPORTED: c_int = 28;
pub const TW_Q_LENGTH: c_int = 256;
pub const TW_Q_START: c_int = 0;
pub const TW_MAX_SLOT: c_int = 32;
pub const TW_MAX_RESET_TRIES: c_int = 2;
pub const TW_MAX_CMDS_PER_LUN: c_int = 254;
pub const TW_MAX_RESPONSE_DRAIN: c_int = 256;
pub const TW_MAX_AEN_DRAIN: c_int = 255;
pub const TW_IN_RESET: c_int = 2;
pub const TW_USING_MSI: c_int = 3;
pub const TW_IN_ATTENTION_LOOP: c_int = 4;
pub const TW_MAX_SECTORS: c_int = 256;
pub const TW_AEN_WAIT_TIME: c_int = 1000;

pub const TW_MAX_CDB_LEN: c_int = 16;
pub const TW_ISR_DONT_COMPLETE: c_int = 2;
pub const TW_ISR_DONT_RESULT: c_int = 3;

pub const TW_VERSION_TABLE: c_uint = 0x0402;
pub const TW_TIMEKEEP_TABLE: c_uint = 0x040A;
pub const TW_INFORMATION_TABLE: c_uint = 0x0403;
pub const TW_PARAM_FWVER: c_int = 3;
pub const TW_PARAM_FWVER_LENGTH: c_int = 16;
pub const TW_PARAM_BIOSVER: c_int = 4;
pub const TW_PARAM_BIOSVER_LENGTH: c_int = 16;
pub const TW_PARAM_PORTCOUNT: c_int = 3;
pub const TW_PARAM_PORTCOUNT_LENGTH: c_int = 1;
pub const TW_MIN_SGL_LENGTH: c_uint = 0x200 /* 512 bytes */;
pub const TW_MAX_SENSE_LENGTH: c_int = 256;
pub const TW_EVENT_SOURCE_AEN: c_uint = 0x1000;
pub const TW_EVENT_SOURCE_COMMAND: c_uint = 0x1001;
pub const TW_EVENT_SOURCE_PCHIP: c_uint = 0x1002;
pub const TW_EVENT_SOURCE_DRIVER: c_uint = 0x1003;
pub const TW_IOCTL_GET_COMPATIBILITY_INFO: c_uint = 0x101;
pub const TW_IOCTL_GET_LAST_EVENT: c_uint = 0x102;
pub const TW_IOCTL_GET_FIRST_EVENT: c_uint = 0x103;
pub const TW_IOCTL_GET_NEXT_EVENT: c_uint = 0x104;
pub const TW_IOCTL_GET_PREVIOUS_EVENT: c_uint = 0x105;
pub const TW_IOCTL_GET_LOCK: c_uint = 0x106;
pub const TW_IOCTL_RELEASE_LOCK: c_uint = 0x107;
pub const TW_IOCTL_FIRMWARE_PASS_THROUGH: c_uint = 0x108;
pub const TW_IOCTL_ERROR_STATUS_NOT_LOCKED: c_uint = 0x1001 // Not locked;
pub const TW_IOCTL_ERROR_STATUS_LOCKED: c_uint = 0x1002 // Already locked;
pub const TW_IOCTL_ERROR_STATUS_NO_MORE_EVENTS: c_uint = 0x1003 // No more events;
pub const TW_IOCTL_ERROR_STATUS_AEN_CLOBBER: c_uint = 0x1004 // AEN clobber occurred;

pub const TW_ALLOCATION_LENGTH: c_int = 128;
pub const TW_SENSE_DATA_LENGTH: c_int = 18;
pub const TW_STATUS_CHECK_CONDITION: c_int = 2;
pub const TW_ERROR_LOGICAL_UNIT_NOT_SUPPORTED: c_uint = 0x10a;
pub const TW_ERROR_UNIT_OFFLINE: c_uint = 0x128;
pub const TW_MESSAGE_SOURCE_CONTROLLER_ERROR: c_int = 3;
pub const TW_MESSAGE_SOURCE_CONTROLLER_EVENT: c_int = 4;
pub const TW_MESSAGE_SOURCE_LINUX_DRIVER: c_int = 6;

pub const TW_MESSAGE_SOURCE_LINUX_OS: c_int = 9;

pub const PCI_DEVICE_ID_3WARE_9000: c_uint = 0x1002;

pub const PCI_DEVICE_ID_3WARE_9550SX: c_uint = 0x1003;

pub const PCI_DEVICE_ID_3WARE_9650SE: c_uint = 0x1004;

pub const PCI_DEVICE_ID_3WARE_9690SA: c_uint = 0x1005;

// Bitmask macros to eliminate bitfields
// opcode: 5, reserved: 3

// opcode: 5, sgloffset: 3

// severity: 3, reserved: 5

// reserved_1: 4, response_id: 8, reserved_2: 20

// request_id: 12, lun: 4

// Macros

pub type twa_addr_t = __le64;

pub type twa_addr_t = __le32;

// Scatter Gather List Entry
// Command Packet
// Second DWORD
// Command Packet for 9000+ controllers
// New command packet header
// This struct is a union of the 2 command packets
// Initconnection structure
// Event info structure
// Lock structure for ioctl get/release lock
// GetParam descriptor
// Response queue
// Compatibility information structure
