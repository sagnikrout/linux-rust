//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/3w-xxxx.h
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

// AEN strings
//
// Codes for newer firmware
// ATA Error                    SCSI Error
// Codes for older firmware
// 3ware Error                  SCSI Error
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
pub const TW_CONTROL_CLEAR_SBUF_WRITE_ERROR: c_uint = 0x00000008;
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
pub const TW_STATUS_ALL_INTERRUPTS: c_uint = 0x000F0000;
pub const TW_STATUS_CLEARABLE_BITS: c_uint = 0x00D00000;
pub const TW_STATUS_EXPECTED_BITS: c_uint = 0x00002000;
pub const TW_STATUS_UNEXPECTED_BITS: c_uint = 0x00F00008;
pub const TW_STATUS_SBUF_WRITE_ERROR: c_uint = 0x00000008;
pub const TW_STATUS_VALID_INTERRUPT: c_uint = 0x00DF0008;
// RESPONSE QUEUE BIT DEFINITIONS
pub const TW_RESPONSE_ID_MASK: c_uint = 0x00000FF0;
// PCI related defines
pub const TW_IO_ADDRESS_RANGE: c_uint = 0x10;

pub const TW_NUMDEVICES: c_int = 2;
pub const TW_PCI_CLEAR_PARITY_ERRORS: c_uint = 0xc100;
pub const TW_PCI_CLEAR_PCI_ABORT: c_uint = 0x2000;
// Command packet opcodes
pub const TW_OP_NOP: c_uint = 0x0;
pub const TW_OP_INIT_CONNECTION: c_uint = 0x1;
pub const TW_OP_READ: c_uint = 0x2;
pub const TW_OP_WRITE: c_uint = 0x3;
pub const TW_OP_VERIFY: c_uint = 0x4;
pub const TW_OP_GET_PARAM: c_uint = 0x12;
pub const TW_OP_SET_PARAM: c_uint = 0x13;
pub const TW_OP_SECTOR_INFO: c_uint = 0x1a;
pub const TW_OP_AEN_LISTEN: c_uint = 0x1c;
pub const TW_OP_FLUSH_CACHE: c_uint = 0x0e;
pub const TW_CMD_PACKET: c_uint = 0x1d;
pub const TW_CMD_PACKET_WITH_DATA: c_uint = 0x1f;
// Asynchronous Event Notification (AEN) Codes
pub const TW_AEN_QUEUE_EMPTY: c_uint = 0x0000;
pub const TW_AEN_SOFT_RESET: c_uint = 0x0001;
pub const TW_AEN_DEGRADED_MIRROR: c_uint = 0x0002;
pub const TW_AEN_CONTROLLER_ERROR: c_uint = 0x0003;
pub const TW_AEN_REBUILD_FAIL: c_uint = 0x0004;
pub const TW_AEN_REBUILD_DONE: c_uint = 0x0005;
pub const TW_AEN_QUEUE_FULL: c_uint = 0x00ff;
pub const TW_AEN_TABLE_UNDEFINED: c_uint = 0x15;
pub const TW_AEN_APORT_TIMEOUT: c_uint = 0x0009;
pub const TW_AEN_DRIVE_ERROR: c_uint = 0x000A;
pub const TW_AEN_SMART_FAIL: c_uint = 0x000F;
pub const TW_AEN_SBUF_FAIL: c_uint = 0x0024;
// Misc defines

pub const TW_MAX_UNITS: c_int = 16;
pub const TW_COMMAND_ALIGNMENT_MASK: c_uint = 0x1ff;
pub const TW_INIT_MESSAGE_CREDITS: c_uint = 0x100;
pub const TW_INIT_COMMAND_PACKET_SIZE: c_uint = 0x3;
pub const TW_POLL_MAX_RETRIES: c_int = 20000;
pub const TW_MAX_SGL_LENGTH: c_int = 62;
pub const TW_ATA_PASS_SGL_MAX: c_int = 60;
pub const TW_Q_LENGTH: c_int = 256;
pub const TW_Q_START: c_int = 0;
pub const TW_MAX_SLOT: c_int = 32;
pub const TW_MAX_PCI_BUSES: c_int = 255;
pub const TW_MAX_RESET_TRIES: c_int = 3;
pub const TW_UNIT_INFORMATION_TABLE_BASE: c_uint = 0x300;

pub const TW_BLOCK_SIZE: c_uint = 0x200 /* 512-byte blocks */;
pub const TW_IOCTL: c_uint = 0x80;
pub const TW_UNIT_ONLINE: c_int = 1;
pub const TW_IN_INTR: c_int = 1;
pub const TW_IN_RESET: c_int = 2;
pub const TW_IN_CHRDEV_IOCTL: c_int = 3;
pub const TW_MAX_SECTORS: c_int = 256;
pub const TW_MAX_IOCTL_SECTORS: c_int = 512;
pub const TW_AEN_WAIT_TIME: c_int = 1000;

pub const TW_ISR_DONT_COMPLETE: c_int = 2;
pub const TW_ISR_DONT_RESULT: c_int = 3;

pub const TW_MAX_CDB_LEN: c_int = 16;
// Bitmask macros to eliminate bitfields
// opcode: 5, sgloffset: 3

// reserved_1: 4, response_id: 8, reserved_2: 20

// unit: 4, host_id: 4

// Macros

// Scatter Gather List Entry
// Command Packet
// Second DWORD

// Structure for new chardev ioctls
// GetParam descriptor
// Response queue
pub type TW_Cmd_State = c_int;
pub const TW_S_INITIAL: c_uint = 0x1  /* Initial state */;
pub const TW_S_STARTED: c_uint = 0x2  /* Id in use */;
pub const TW_S_POSTED: c_uint = 0x4  /* Posted to the controller */;
pub const TW_S_PENDING: c_uint = 0x8  /* Waiting to be posted in isr */;
pub const TW_S_COMPLETED: c_uint = 0x10 /* Completed by isr */;
pub const TW_S_FINISHED: c_uint = 0x20 /* I/O completely done */;

// Command header for ATA pass-thru

