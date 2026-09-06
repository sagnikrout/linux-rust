//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/megaraid/megaraid_sas.h
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
// Linux MegaRAID driver for SAS based RAID controllers
//
// Copyright (c) 2003-2013  LSI Corporation
// Copyright (c) 2013-2016  Avago Technologies
// Copyright (c) 2016-2018  Broadcom Inc.
//
// FILE: megaraid_sas.h
//
// Authors: Broadcom Inc.
// Kashyap Desai <kashyap.desai@broadcom.com>
// Sumit Saxena <sumit.saxena@broadcom.com>
//
// Send feedback to: megaraidlinux.pdl@broadcom.com
//

//
// MegaRAID SAS Driver meta data
//

pub const MEGASAS_MSIX_NAME_LEN: c_int = 32;
//
// Device IDs
//
pub const PCI_DEVICE_ID_LSI_SAS1078R: c_uint = 0x0060;
pub const PCI_DEVICE_ID_LSI_SAS1078DE: c_uint = 0x007C;
pub const PCI_DEVICE_ID_LSI_VERDE_ZCR: c_uint = 0x0413;
pub const PCI_DEVICE_ID_LSI_SAS1078GEN2: c_uint = 0x0078;
pub const PCI_DEVICE_ID_LSI_SAS0079GEN2: c_uint = 0x0079;
pub const PCI_DEVICE_ID_LSI_SAS0073SKINNY: c_uint = 0x0073;
pub const PCI_DEVICE_ID_LSI_SAS0071SKINNY: c_uint = 0x0071;
pub const PCI_DEVICE_ID_LSI_FUSION: c_uint = 0x005b;
pub const PCI_DEVICE_ID_LSI_PLASMA: c_uint = 0x002f;
pub const PCI_DEVICE_ID_LSI_INVADER: c_uint = 0x005d;
pub const PCI_DEVICE_ID_LSI_FURY: c_uint = 0x005f;
pub const PCI_DEVICE_ID_LSI_INTRUDER: c_uint = 0x00ce;
pub const PCI_DEVICE_ID_LSI_INTRUDER_24: c_uint = 0x00cf;
pub const PCI_DEVICE_ID_LSI_CUTLASS_52: c_uint = 0x0052;
pub const PCI_DEVICE_ID_LSI_CUTLASS_53: c_uint = 0x0053;
pub const PCI_DEVICE_ID_LSI_VENTURA: c_uint = 0x0014;
pub const PCI_DEVICE_ID_LSI_CRUSADER: c_uint = 0x0015;
pub const PCI_DEVICE_ID_LSI_HARPOON: c_uint = 0x0016;
pub const PCI_DEVICE_ID_LSI_TOMCAT: c_uint = 0x0017;
pub const PCI_DEVICE_ID_LSI_VENTURA_4PORT: c_uint = 0x001B;
pub const PCI_DEVICE_ID_LSI_CRUSADER_4PORT: c_uint = 0x001C;
pub const PCI_DEVICE_ID_LSI_AERO_10E1: c_uint = 0x10e1;
pub const PCI_DEVICE_ID_LSI_AERO_10E2: c_uint = 0x10e2;
pub const PCI_DEVICE_ID_LSI_AERO_10E5: c_uint = 0x10e5;
pub const PCI_DEVICE_ID_LSI_AERO_10E6: c_uint = 0x10e6;
pub const PCI_DEVICE_ID_LSI_AERO_10E0: c_uint = 0x10e0;
pub const PCI_DEVICE_ID_LSI_AERO_10E3: c_uint = 0x10e3;
pub const PCI_DEVICE_ID_LSI_AERO_10E4: c_uint = 0x10e4;
pub const PCI_DEVICE_ID_LSI_AERO_10E7: c_uint = 0x10e7;
//
// Intel HBA SSDIDs
//
pub const MEGARAID_INTEL_RS3DC080_SSDID: c_uint = 0x9360;
pub const MEGARAID_INTEL_RS3DC040_SSDID: c_uint = 0x9362;
pub const MEGARAID_INTEL_RS3SC008_SSDID: c_uint = 0x9380;
pub const MEGARAID_INTEL_RS3MC044_SSDID: c_uint = 0x9381;
pub const MEGARAID_INTEL_RS3WC080_SSDID: c_uint = 0x9341;
pub const MEGARAID_INTEL_RS3WC040_SSDID: c_uint = 0x9343;
pub const MEGARAID_INTEL_RMS3BC160_SSDID: c_uint = 0x352B;
//
// Intruder HBA SSDIDs
//
pub const MEGARAID_INTRUDER_SSDID1: c_uint = 0x9371;
pub const MEGARAID_INTRUDER_SSDID2: c_uint = 0x9390;
pub const MEGARAID_INTRUDER_SSDID3: c_uint = 0x9370;
//
// Intel HBA branding
//

//
// =====================================
// MegaRAID SAS MFI firmware definitions
// =====================================
//
// MFI stands for  MegaRAID SAS FW Interface. This is just a moniker for
// protocol between the software and firmware. Commands are issued using
// "message frames"
//
// FW posts its state in upper 4 bits of outbound_msg_0 register
//
pub const MFI_STATE_MASK: c_uint = 0xF0000000;
pub const MFI_STATE_UNDEFINED: c_uint = 0x00000000;
pub const MFI_STATE_BB_INIT: c_uint = 0x10000000;
pub const MFI_STATE_FW_INIT: c_uint = 0x40000000;
pub const MFI_STATE_WAIT_HANDSHAKE: c_uint = 0x60000000;
pub const MFI_STATE_FW_INIT_2: c_uint = 0x70000000;
pub const MFI_STATE_DEVICE_SCAN: c_uint = 0x80000000;
pub const MFI_STATE_BOOT_MESSAGE_PENDING: c_uint = 0x90000000;
pub const MFI_STATE_FLUSH_CACHE: c_uint = 0xA0000000;
pub const MFI_STATE_READY: c_uint = 0xB0000000;
pub const MFI_STATE_OPERATIONAL: c_uint = 0xC0000000;
pub const MFI_STATE_FAULT: c_uint = 0xF0000000;
pub const MFI_STATE_FORCE_OCR: c_uint = 0x00000080;
pub const MFI_STATE_DMADONE: c_uint = 0x00000008;
pub const MFI_STATE_CRASH_DUMP_DONE: c_uint = 0x00000004;
pub const MFI_RESET_REQUIRED: c_uint = 0x00000001;
pub const MFI_RESET_ADAPTER: c_uint = 0x00000002;
pub const MEGAMFI_FRAME_SIZE: c_int = 64;
pub const MFI_STATE_FAULT_CODE: c_uint = 0x0FFF0000;
pub const MFI_STATE_FAULT_SUBCODE: c_uint = 0x0000FF00;
//
// During FW init, clear pending cmds & reset state using inbound_msg_0
//
// ABORT	: Abort all pending cmds
// READY	: Move from OPERATIONAL to READY state; discard queue info
// MFIMODE	: Discard (possible) low MFA posted in 64-bit mode (??)
// CLR_HANDSHAKE: FW is waiting for HANDSHAKE from BIOS or Driver
// HOTPLUG	: Resume from Hotplug
// MFI_STOP_ADP	: Send signal to FW to stop processing
// MFI_ADP_TRIGGER_SNAP_DUMP: Inform firmware to initiate snap dump
//

pub const MFI_ADP_RESET: c_uint = 0x00000040;
pub const MFI_INIT_ABORT: c_uint = 0x00000001;
pub const MFI_INIT_READY: c_uint = 0x00000002;
pub const MFI_INIT_MFIMODE: c_uint = 0x00000004;
pub const MFI_INIT_CLEAR_HANDSHAKE: c_uint = 0x00000008;
pub const MFI_INIT_HOTPLUG: c_uint = 0x00000010;
pub const MFI_STOP_ADP: c_uint = 0x00000020;

pub const MFI_ADP_TRIGGER_SNAP_DUMP: c_uint = 0x00000100;

//
// MFI frame flags
//
pub const MFI_FRAME_POST_IN_REPLY_QUEUE: c_uint = 0x0000;
pub const MFI_FRAME_DONT_POST_IN_REPLY_QUEUE: c_uint = 0x0001;
pub const MFI_FRAME_SGL32: c_uint = 0x0000;
pub const MFI_FRAME_SGL64: c_uint = 0x0002;
pub const MFI_FRAME_SENSE32: c_uint = 0x0000;
pub const MFI_FRAME_SENSE64: c_uint = 0x0004;
pub const MFI_FRAME_DIR_NONE: c_uint = 0x0000;
pub const MFI_FRAME_DIR_WRITE: c_uint = 0x0008;
pub const MFI_FRAME_DIR_READ: c_uint = 0x0010;
pub const MFI_FRAME_DIR_BOTH: c_uint = 0x0018;
pub const MFI_FRAME_IEEE: c_uint = 0x0020;
// Driver internal
pub const DRV_DCMD_POLLED_MODE: c_uint = 0x1;
pub const DRV_DCMD_SKIP_REFIRE: c_uint = 0x2;
//
// Definition for cmd_status
//
pub const MFI_CMD_STATUS_POLL_MODE: c_uint = 0xFF;
//
// MFI command opcodes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MFI_CMD_OP {
    MFI_CMD_INIT		= 0x0,
    MFI_CMD_LD_READ		= 0x1,
    MFI_CMD_LD_WRITE	= 0x2,
    MFI_CMD_LD_SCSI_IO	= 0x3,
    MFI_CMD_PD_SCSI_IO	= 0x4,
    MFI_CMD_DCMD		= 0x5,
    MFI_CMD_ABORT		= 0x6,
    MFI_CMD_SMP		= 0x7,
    MFI_CMD_STP		= 0x8,
    MFI_CMD_NVME		= 0x9,
    MFI_CMD_TOOLBOX		= 0xa,
    MFI_CMD_OP_COUNT,
    MFI_CMD_INVALID		= 0xff
}

pub const MR_DCMD_CTRL_GET_INFO: c_uint = 0x01010000;
pub const MR_DCMD_LD_GET_LIST: c_uint = 0x03010000;
pub const MR_DCMD_LD_LIST_QUERY: c_uint = 0x03010100;
pub const MR_DCMD_CTRL_CACHE_FLUSH: c_uint = 0x01101000;
pub const MR_FLUSH_CTRL_CACHE: c_uint = 0x01;
pub const MR_FLUSH_DISK_CACHE: c_uint = 0x02;
pub const MR_DCMD_CTRL_SHUTDOWN: c_uint = 0x01050000;
pub const MR_DCMD_HIBERNATE_SHUTDOWN: c_uint = 0x01060000;
pub const MR_ENABLE_DRIVE_SPINDOWN: c_uint = 0x01;
pub const MR_DCMD_CTRL_EVENT_GET_INFO: c_uint = 0x01040100;
pub const MR_DCMD_CTRL_EVENT_GET: c_uint = 0x01040300;
pub const MR_DCMD_CTRL_EVENT_WAIT: c_uint = 0x01040500;
pub const MR_DCMD_LD_GET_PROPERTIES: c_uint = 0x03030000;
pub const MR_DCMD_CLUSTER: c_uint = 0x08000000;
pub const MR_DCMD_CLUSTER_RESET_ALL: c_uint = 0x08010100;
pub const MR_DCMD_CLUSTER_RESET_LD: c_uint = 0x08010200;
pub const MR_DCMD_PD_LIST_QUERY: c_uint = 0x02010100;
pub const MR_DCMD_CTRL_SET_CRASH_DUMP_PARAMS: c_uint = 0x01190100;

pub const MR_DCMD_PD_GET_INFO: c_uint = 0x02020000;
//
// Global functions
//
extern "C" {
    pub fn MR_ValidateMapInfo(instance: *mut megasas_instance, map_id: u64) -> u8;
}
//
// MFI command completion codes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MFI_STAT {
    MFI_STAT_OK = 0x00,
    MFI_STAT_INVALID_CMD = 0x01,
    MFI_STAT_INVALID_DCMD = 0x02,
    MFI_STAT_INVALID_PARAMETER = 0x03,
    MFI_STAT_INVALID_SEQUENCE_NUMBER = 0x04,
    MFI_STAT_ABORT_NOT_POSSIBLE = 0x05,
    MFI_STAT_APP_HOST_CODE_NOT_FOUND = 0x06,
    MFI_STAT_APP_IN_USE = 0x07,
    MFI_STAT_APP_NOT_INITIALIZED = 0x08,
    MFI_STAT_ARRAY_INDEX_INVALID = 0x09,
    MFI_STAT_ARRAY_ROW_NOT_EMPTY = 0x0a,
    MFI_STAT_CONFIG_RESOURCE_CONFLICT = 0x0b,
    MFI_STAT_DEVICE_NOT_FOUND = 0x0c,
    MFI_STAT_DRIVE_TOO_SMALL = 0x0d,
    MFI_STAT_FLASH_ALLOC_FAIL = 0x0e,
    MFI_STAT_FLASH_BUSY = 0x0f,
    MFI_STAT_FLASH_ERROR = 0x10,
    MFI_STAT_FLASH_IMAGE_BAD = 0x11,
    MFI_STAT_FLASH_IMAGE_INCOMPLETE = 0x12,
    MFI_STAT_FLASH_NOT_OPEN = 0x13,
    MFI_STAT_FLASH_NOT_STARTED = 0x14,
    MFI_STAT_FLUSH_FAILED = 0x15,
    MFI_STAT_HOST_CODE_NOT_FOUNT = 0x16,
    MFI_STAT_LD_CC_IN_PROGRESS = 0x17,
    MFI_STAT_LD_INIT_IN_PROGRESS = 0x18,
    MFI_STAT_LD_LBA_OUT_OF_RANGE = 0x19,
    MFI_STAT_LD_MAX_CONFIGURED = 0x1a,
    MFI_STAT_LD_NOT_OPTIMAL = 0x1b,
    MFI_STAT_LD_RBLD_IN_PROGRESS = 0x1c,
    MFI_STAT_LD_RECON_IN_PROGRESS = 0x1d,
    MFI_STAT_LD_WRONG_RAID_LEVEL = 0x1e,
    MFI_STAT_MAX_SPARES_EXCEEDED = 0x1f,
    MFI_STAT_MEMORY_NOT_AVAILABLE = 0x20,
    MFI_STAT_MFC_HW_ERROR = 0x21,
    MFI_STAT_NO_HW_PRESENT = 0x22,
    MFI_STAT_NOT_FOUND = 0x23,
    MFI_STAT_NOT_IN_ENCL = 0x24,
    MFI_STAT_PD_CLEAR_IN_PROGRESS = 0x25,
    MFI_STAT_PD_TYPE_WRONG = 0x26,
    MFI_STAT_PR_DISABLED = 0x27,
    MFI_STAT_ROW_INDEX_INVALID = 0x28,
    MFI_STAT_SAS_CONFIG_INVALID_ACTION = 0x29,
    MFI_STAT_SAS_CONFIG_INVALID_DATA = 0x2a,
    MFI_STAT_SAS_CONFIG_INVALID_PAGE = 0x2b,
    MFI_STAT_SAS_CONFIG_INVALID_TYPE = 0x2c,
    MFI_STAT_SCSI_DONE_WITH_ERROR = 0x2d,
    MFI_STAT_SCSI_IO_FAILED = 0x2e,
    MFI_STAT_SCSI_RESERVATION_CONFLICT = 0x2f,
    MFI_STAT_SHUTDOWN_FAILED = 0x30,
    MFI_STAT_TIME_NOT_SET = 0x31,
    MFI_STAT_WRONG_STATE = 0x32,
    MFI_STAT_LD_OFFLINE = 0x33,
    MFI_STAT_PEER_NOTIFICATION_REJECTED = 0x34,
    MFI_STAT_PEER_NOTIFICATION_FAILED = 0x35,
    MFI_STAT_RESERVATION_IN_PROGRESS = 0x36,
    MFI_STAT_I2C_ERRORS_DETECTED = 0x37,
    MFI_STAT_PCI_ERRORS_DETECTED = 0x38,
    MFI_STAT_CONFIG_SEQ_MISMATCH = 0x67,

    MFI_STAT_INVALID_STATUS = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mfi_evt_class {
    MFI_EVT_CLASS_DEBUG =		-2,
    MFI_EVT_CLASS_PROGRESS =	-1,
    MFI_EVT_CLASS_INFO =		0,
    MFI_EVT_CLASS_WARNING =		1,
    MFI_EVT_CLASS_CRITICAL =	2,
    MFI_EVT_CLASS_FATAL =		3,
    MFI_EVT_CLASS_DEAD =		4
}

//
// Crash dump related defines
//
pub const MAX_CRASH_DUMP_SIZE: c_int = 512;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MR_FW_CRASH_DUMP_STATE {
    UNAVAILABLE = 0,
    AVAILABLE = 1,
    COPYING = 2,
    COPIED = 3,
    COPY_ERROR = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _MR_CRASH_BUF_STATUS {
    MR_CRASH_BUF_TURN_OFF = 0,
    MR_CRASH_BUF_TURN_ON = 1,
}

//
// Number of mailbox bytes in DCMD message frame
//
pub const MFI_MBOX_SIZE: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MR_EVT_CLASS {

    MR_EVT_CLASS_DEBUG = -2,
    MR_EVT_CLASS_PROGRESS = -1,
    MR_EVT_CLASS_INFO = 0,
    MR_EVT_CLASS_WARNING = 1,
    MR_EVT_CLASS_CRITICAL = 2,
    MR_EVT_CLASS_FATAL = 3,
    MR_EVT_CLASS_DEAD = 4,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MR_EVT_LOCALE {

    MR_EVT_LOCALE_LD = 0x0001,
    MR_EVT_LOCALE_PD = 0x0002,
    MR_EVT_LOCALE_ENCL = 0x0004,
    MR_EVT_LOCALE_BBU = 0x0008,
    MR_EVT_LOCALE_SAS = 0x0010,
    MR_EVT_LOCALE_CTRL = 0x0020,
    MR_EVT_LOCALE_CONFIG = 0x0040,
    MR_EVT_LOCALE_CLUSTER = 0x0080,
    MR_EVT_LOCALE_ALL = 0xffff,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MR_EVT_ARGS {

    MR_EVT_ARGS_NONE,
    MR_EVT_ARGS_CDB_SENSE,
    MR_EVT_ARGS_LD,
    MR_EVT_ARGS_LD_COUNT,
    MR_EVT_ARGS_LD_LBA,
    MR_EVT_ARGS_LD_OWNER,
    MR_EVT_ARGS_LD_LBA_PD_LBA,
    MR_EVT_ARGS_LD_PROG,
    MR_EVT_ARGS_LD_STATE,
    MR_EVT_ARGS_LD_STRIP,
    MR_EVT_ARGS_PD,
    MR_EVT_ARGS_PD_ERR,
    MR_EVT_ARGS_PD_LBA,
    MR_EVT_ARGS_PD_LBA_LD,
    MR_EVT_ARGS_PD_PROG,
    MR_EVT_ARGS_PD_STATE,
    MR_EVT_ARGS_PCI,
    MR_EVT_ARGS_RATE,
    MR_EVT_ARGS_STR,
    MR_EVT_ARGS_TIME,
    MR_EVT_ARGS_ECC,
    MR_EVT_ARGS_LD_PROP,
    MR_EVT_ARGS_PD_SPARE,
    MR_EVT_ARGS_PD_INDEX,
    MR_EVT_ARGS_DIAG_PASS,
    MR_EVT_ARGS_DIAG_FAIL,
    MR_EVT_ARGS_PD_LBA_LBA,
    MR_EVT_ARGS_PORT_PHY,
    MR_EVT_ARGS_PD_MISSING,
    MR_EVT_ARGS_PD_ADDRESS,
    MR_EVT_ARGS_BITMAP,
    MR_EVT_ARGS_CONNECTOR,
    MR_EVT_ARGS_PD_PD,
    MR_EVT_ARGS_PD_FRU,
    MR_EVT_ARGS_PD_PATHINFO,
    MR_EVT_ARGS_PD_POWER_STATE,
    MR_EVT_ARGS_GENERIC,
}

pub const SGE_BUFFER_SIZE: c_int = 4096;
pub const MEGASAS_CLUSTER_ID_SIZE: c_int = 16;
//
// define constants for device list query options
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MR_PD_QUERY_TYPE {
    MR_PD_QUERY_TYPE_ALL                = 0,
    MR_PD_QUERY_TYPE_STATE              = 1,
    MR_PD_QUERY_TYPE_POWER_STATE        = 2,
    MR_PD_QUERY_TYPE_MEDIA_TYPE         = 3,
    MR_PD_QUERY_TYPE_SPEED              = 4,
    MR_PD_QUERY_TYPE_EXPOSED_TO_HOST    = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MR_LD_QUERY_TYPE {
    MR_LD_QUERY_TYPE_ALL	         = 0,
    MR_LD_QUERY_TYPE_EXPOSED_TO_HOST = 1,
    MR_LD_QUERY_TYPE_USED_TGT_IDS    = 2,
    MR_LD_QUERY_TYPE_CLUSTER_ACCESS  = 3,
    MR_LD_QUERY_TYPE_CLUSTER_LOCALE  = 4,
}

pub const MR_EVT_CFG_CLEARED: c_uint = 0x0004;
pub const MR_EVT_LD_STATE_CHANGE: c_uint = 0x0051;
pub const MR_EVT_PD_INSERTED: c_uint = 0x005b;
pub const MR_EVT_PD_REMOVED: c_uint = 0x0070;
pub const MR_EVT_LD_CREATED: c_uint = 0x008a;
pub const MR_EVT_LD_DELETED: c_uint = 0x008b;
pub const MR_EVT_FOREIGN_CFG_IMPORTED: c_uint = 0x00db;
pub const MR_EVT_LD_OFFLINE: c_uint = 0x00fc;
pub const MR_EVT_CTRL_HOST_BUS_SCAN_REQUESTED: c_uint = 0x0152;
pub const MR_EVT_CTRL_PROP_CHANGED: c_uint = 0x012f;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MR_PD_STATE {
    MR_PD_STATE_UNCONFIGURED_GOOD   = 0x00,
    MR_PD_STATE_UNCONFIGURED_BAD    = 0x01,
    MR_PD_STATE_HOT_SPARE           = 0x02,
    MR_PD_STATE_OFFLINE             = 0x10,
    MR_PD_STATE_FAILED              = 0x11,
    MR_PD_STATE_REBUILD             = 0x14,
    MR_PD_STATE_ONLINE              = 0x18,
    MR_PD_STATE_COPYBACK            = 0x20,
    MR_PD_STATE_SYSTEM              = 0x40,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MR_PD_REF {
    pub deviceId: u16,
    pub seqNum: u16,
    pub mrPdRef: },
    pub ref: u32,
}

//
// define the DDF Type bit structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union MR_PD_DDF_TYPE {
    pub forcedPDGUID:1: u16,
    pub inVD:1: u16,
    pub isGlobalSpare:1: u16,
    pub isSpare:1: u16,
    pub isForeign:1: u16,
    pub reserved:7: u16,
    pub intf:4: u16,

    pub intf:4: u16,
    pub reserved:7: u16,
    pub isForeign:1: u16,
    pub isSpare:1: u16,
    pub isGlobalSpare:1: u16,
    pub inVD:1: u16,
    pub forcedPDGUID:1: u16,

    pub pdType: },
    pub type: u16,
}

//
// defines the progress structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union MR_PROGRESS {
    pub progress: u16,
    pub elapsedSecs: u16,
    pub elapsedSecsForLastPercent: u16,
}

//
// defines the physical drive progress structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_PD_PROGRESS {
    pub rbld:1: u32,
    pub patrol:1: u32,
    pub clear:1: u32,
    pub copyBack:1: u32,
    pub erase:1: u32,
    pub locate:1: u32,
    pub reserved:26: u32,

    pub reserved:26: u32,
    pub locate:1: u32,
    pub erase:1: u32,
    pub copyBack:1: u32,
    pub clear:1: u32,
    pub patrol:1: u32,
    pub rbld:1: u32,

    pub active: },
    pub rbld: MR_PROGRESS,
    pub patrol: MR_PROGRESS,
    pub clear: MR_PROGRESS,
    pub erase: MR_PROGRESS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_PD_INFO {
    pub ref: MR_PD_REF,
    pub inquiryData: [u8; 96],
    pub vpdPage83: [u8; 64],
    pub notSupported: u8,
    pub scsiDevType: u8,
    pub connectedPortBitmap: u8,
    pub connectedPortNumbers: u8,
}

//
// Definition of structure used to expose attributes of VD or JBOD
// (this structure is to be filled by firmware when MR_DCMD_DRV_GET_TARGET_PROP
// is fired by driver)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_TARGET_PROPERTIES {
    pub max_io_size_kb: u32,
    pub device_qdepth: u32,
    pub sector_size: u32,
    pub reset_tmo: u8,
    pub reserved: [u8; 499],
    pub __packed: },
//
// defines the physical drive address structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_PD_ADDRESS {
    pub deviceId: __le16,
    pub enclDeviceId: u16,
    pub enclIndex: u8,
    pub slotNumber: u8,
    pub mrPdAddress: },
    pub enclPosition: u8,
    pub enclConnectorIndex: u8,
    pub mrEnclAddress: },
}

//
// defines the physical drive list structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_PD_LIST {
    pub size: __le32,
    pub count: __le32,
    pub addr: [MR_PD_ADDRESS; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_pd_list {
    pub tid: u16,
    pub driveType: u8,
    pub driveState: u8,
    pub __packed: },
//
// defines the logical drive reference structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union MR_LD_REF {
    pub targetId: u8,
    pub reserved: u8,
    pub seqNum: __le16,
}

//
// defines the logical drive list structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_LD_LIST {
    pub ldCount: __le32,
    pub reserved: __le32,
    pub ref: MR_LD_REF,
    pub state: u8,
    pub reserved: [u8; 3],
    pub size: __le64,
    pub ldList: [}; MAX_LOGICAL_DRIVES_EXT],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_LD_TARGETID_LIST {
    pub size: __le32,
    pub count: __le32,
    pub pad: [u8; 3],
    pub targetId: [u8; MAX_LOGICAL_DRIVES_EXT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_HOST_DEVICE_LIST_ENTRY {

    pub reserved:7: u8,
    pub is_sys_pd:1: u8,

    pub is_sys_pd:1: u8,
    pub reserved:7: u8,

    pub bits: },
    pub byte: u8,
    pub u: },
    pub flags: },
    pub scsi_type: u8,
    pub target_id: __le16,
    pub reserved: [u8; 4],
    pub sas_addr: [__le64; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_HOST_DEVICE_LIST {
    pub size: __le32,
    pub count: __le32,
    pub reserved: [__le32; 2],
    pub __counted_by_le(count): MR_HOST_DEVICE_LIST_ENTRY host_device_list[],
    pub __packed: },

//
// SAS controller properties
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_ctrl_prop {
    pub seq_num: u16,
    pub pred_fail_poll_interval: u16,
    pub intr_throttle_count: u16,
    pub intr_throttle_timeouts: u16,
    pub rebuild_rate: u8,
    pub patrol_read_rate: u8,
    pub bgi_rate: u8,
    pub cc_rate: u8,
    pub recon_rate: u8,
    pub cache_flush_interval: u8,
    pub spinup_drv_count: u8,
    pub spinup_delay: u8,
    pub cluster_enable: u8,
    pub coercion_mode: u8,
    pub alarm_enable: u8,
    pub disable_auto_rebuild: u8,
    pub disable_battery_warn: u8,
    pub ecc_bucket_size: u8,
    pub ecc_bucket_leak_rate: u16,
    pub restore_hotspare_on_insertion: u8,
    pub expose_encl_devices: u8,
    pub maintainPdFailHistory: u8,
    pub disallowHostRequestReordering: u8,
    pub abortCCOnError: u8,
    pub loadBalanceMode: u8,
    pub disableAutoDetectBackplane: u8,
    pub snapVDSpace: u8,
//
// Add properties that can be controlled by
// a bit in the following structure.
//

    pub reserved:18: u32,
    pub enableJBOD:1: u32,
    pub disableSpinDownHS:1: u32,
    pub allowBootWithPinnedCache:1: u32,
    pub disableOnlineCtrlReset:1: u32,
    pub enableSecretKeyControl:1: u32,
    pub autoEnhancedImport:1: u32,
    pub enableSpinDownUnconfigured:1: u32,
    pub SSDPatrolReadEnabled:1: u32,
    pub SSDSMARTerEnabled:1: u32,
    pub disableNCQ:1: u32,
    pub useFdeOnly:1: u32,
    pub prCorrectUnconfiguredAreas:1: u32,
    pub SMARTerEnabled:1: u32,
    pub copyBackDisabled:1: u32,

    pub copyBackDisabled:1: u32,
    pub SMARTerEnabled:1: u32,
    pub prCorrectUnconfiguredAreas:1: u32,
    pub useFdeOnly:1: u32,
    pub disableNCQ:1: u32,
    pub SSDSMARTerEnabled:1: u32,
    pub SSDPatrolReadEnabled:1: u32,
    pub enableSpinDownUnconfigured:1: u32,
    pub autoEnhancedImport:1: u32,
    pub enableSecretKeyControl:1: u32,
    pub disableOnlineCtrlReset:1: u32,
    pub allowBootWithPinnedCache:1: u32,
    pub disableSpinDownHS:1: u32,
    pub enableJBOD:1: u32,
    pub reserved:18: u32,

    pub OnOffProperties: },
    pub autoSnapVDSpace: u8,
    pub viewSpace: u8,

    pub reserved3:9: u16,
    pub enable_fw_dev_list:1: u16,
    pub reserved2:1: u16,
    pub enable_snap_dump:1: u16,
    pub reserved1:4: u16,

    pub reserved1:4: u16,
    pub enable_snap_dump:1: u16,
    pub reserved2:1: u16,
    pub enable_fw_dev_list:1: u16,
    pub reserved3:9: u16,

    pub on_off_properties2: },
}

//
// SAS controller information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_ctrl_info {
//
// PCI device information
//
    pub vendor_id: __le16,
    pub device_id: __le16,
    pub sub_vendor_id: __le16,
    pub sub_device_id: __le16,
    pub reserved: [u8; 24],
// C attribute field omitted
//
// Host interface information
//
    pub PCIX:1: u8,
    pub PCIE:1: u8,
    pub iSCSI:1: u8,
    pub SAS_3G:1: u8,
    pub SRIOV:1: u8,
    pub reserved_0:3: u8,
    pub reserved_1: [u8; 6],
    pub port_count: u8,
    pub port_addr: [u64; 8],
// C attribute field omitted
//
// Device (backend) interface information
//
    pub SPI:1: u8,
    pub SAS_3G:1: u8,
    pub SATA_1_5G:1: u8,
    pub SATA_3G:1: u8,
    pub reserved_0:4: u8,
    pub reserved_1: [u8; 6],
    pub port_count: u8,
    pub port_addr: [u64; 8],
// C attribute field omitted
//
// List of components residing in flash. All str are null terminated
//
    pub image_check_word: __le32,
    pub image_component_count: __le32,
    pub name: [c_char; 8],
    pub version: [c_char; 32],
    pub build_date: [c_char; 16],
    pub built_time: [c_char; 16],
    pub image_component: [} __attribute__ ((packed)); 8],
//
// List of flash components that have been flashed on the card, but
// are not in use, pending reset of the adapter. This list will be
// empty if a flash operation has not occurred. All stings are null
// terminated
//
    pub pending_image_component_count: __le32,
    pub name: [c_char; 8],
    pub version: [c_char; 32],
    pub build_date: [c_char; 16],
    pub build_time: [c_char; 16],
    pub pending_image_component: [} __attribute__ ((packed)); 8],
    pub max_arms: u8,
    pub max_spans: u8,
    pub max_arrays: u8,
    pub max_lds: u8,
    pub product_name: [c_char; 80],
    pub serial_no: [c_char; 32],
//
// Other physical/controller/operation information. Indicates the
// presence of the hardware
//
    pub bbu:1: u32,
    pub alarm:1: u32,
    pub nvram:1: u32,
    pub uart:1: u32,
    pub reserved:28: u32,
// C attribute field omitted
    pub current_fw_time: __le32,
//
// Maximum data transfer sizes
//
    pub max_concurrent_cmds: __le16,
    pub max_sge_count: __le16,
    pub max_request_size: __le32,
//
// Logical and physical device counts
//
    pub ld_present_count: __le16,
    pub ld_degraded_count: __le16,
    pub ld_offline_count: __le16,
    pub pd_present_count: __le16,
    pub pd_disk_present_count: __le16,
    pub pd_disk_pred_failure_count: __le16,
    pub pd_disk_failed_count: __le16,
//
// Memory size information
//
    pub nvram_size: __le16,
    pub memory_size: __le16,
    pub flash_size: __le16,
//
// Error counters
//
    pub mem_correctable_error_count: __le16,
    pub mem_uncorrectable_error_count: __le16,
//
// Cluster information
//
    pub cluster_permitted: u8,
    pub cluster_active: u8,
//
// Additional max data transfer sizes
//
    pub max_strips_per_io: __le16,
//
// Controller capabilities structures
//
    pub raid_level_0:1: u32,
    pub raid_level_1:1: u32,
    pub raid_level_5:1: u32,
    pub raid_level_1E:1: u32,
    pub raid_level_6:1: u32,
    pub reserved:27: u32,
// C attribute field omitted
    pub rbld_rate:1: u32,
    pub cc_rate:1: u32,
    pub bgi_rate:1: u32,
    pub recon_rate:1: u32,
    pub patrol_rate:1: u32,
    pub alarm_control:1: u32,
    pub cluster_supported:1: u32,
    pub bbu:1: u32,
    pub spanning_allowed:1: u32,
    pub dedicated_hotspares:1: u32,
    pub revertible_hotspares:1: u32,
    pub foreign_config_import:1: u32,
    pub self_diagnostic:1: u32,
    pub mixed_redundancy_arr:1: u32,
    pub global_hot_spares:1: u32,
    pub reserved:17: u32,
// C attribute field omitted
    pub read_policy:1: u32,
    pub write_policy:1: u32,
    pub io_policy:1: u32,
    pub access_policy:1: u32,
    pub disk_cache_policy:1: u32,
    pub reserved:27: u32,
// C attribute field omitted
    pub min: u8,
    pub max: u8,
    pub reserved: [u8; 2],
// C attribute field omitted
    pub force_online:1: u32,
    pub force_offline:1: u32,
    pub force_rebuild:1: u32,
    pub reserved:29: u32,
// C attribute field omitted
    pub ctrl_supports_sas:1: u32,
    pub ctrl_supports_sata:1: u32,
    pub allow_mix_in_encl:1: u32,
    pub allow_mix_in_ld:1: u32,
    pub allow_sata_in_cluster:1: u32,
    pub reserved:27: u32,
// C attribute field omitted
//
// Define ECC single-bit-error bucket information
//
    pub ecc_bucket_count: u8,
    pub reserved_2: [u8; 11],
//
// Include the controller properties (changeable items)
//
    pub properties: megasas_ctrl_prop,
//
// Define FW pkg version (set in envt v'bles on OEM basis)
//
    pub package_version: [c_char; 0x60],
//
// If adapterOperations.supportMoreThan8Phys is set,
// and deviceInterface.portCount is greater than 8,
// SAS Addrs for first 8 ports shall be populated in
// deviceInterface.portAddr, and the rest shall be
// populated in deviceInterfacePortAddr2.
//
    pub /: *mut *mut __le64 deviceInterfacePortAddr2[8]; /6a0h,
    pub /: *mut *mut u8 reserved3[128]; /6e0h,
    pub minPdRaidLevel_0:4: u16,
    pub maxPdRaidLevel_0:12: u16,
    pub minPdRaidLevel_1:4: u16,
    pub maxPdRaidLevel_1:12: u16,
    pub minPdRaidLevel_5:4: u16,
    pub maxPdRaidLevel_5:12: u16,
    pub minPdRaidLevel_1E:4: u16,
    pub maxPdRaidLevel_1E:12: u16,
    pub minPdRaidLevel_6:4: u16,
    pub maxPdRaidLevel_6:12: u16,
    pub minPdRaidLevel_10:4: u16,
    pub maxPdRaidLevel_10:12: u16,
    pub minPdRaidLevel_50:4: u16,
    pub maxPdRaidLevel_50:12: u16,
    pub minPdRaidLevel_60:4: u16,
    pub maxPdRaidLevel_60:12: u16,
    pub minPdRaidLevel_1E_RLQ0:4: u16,
    pub maxPdRaidLevel_1E_RLQ0:12: u16,
    pub minPdRaidLevel_1E0_RLQ0:4: u16,
    pub maxPdRaidLevel_1E0_RLQ0:12: u16,
    pub reserved: [u16; 6],
    pub pdsForRaidLevels: },
    pub /: *mut *mut __le16 maxPds; /780h,
    pub /: *mut *mut __le16 maxDedHSPs; /782h,
    pub /: *mut *mut __le16 maxGlobalHSP; /784h,
    pub /: *mut *mut __le16 ddfSize; /786h,
    pub /: *mut *mut u8 maxLdsPerArray; /788h,
    pub /: *mut *mut u8 partitionsInDDF; /789h,
    pub /: *mut *mut u8 lockKeyBinding; /78ah,
    pub /: *mut *mut u8 maxPITsPerLd; /78bh,
    pub /: *mut *mut u8 maxViewsPerLd; /78ch,
    pub /: *mut *mut u8 maxTargetId; /78dh,
    pub /: *mut *mut __le16 maxBvlVdSize; /78eh,
    pub /: *mut *mut __le16 maxConfigurableSSCSize; /790h,
    pub /: *mut *mut __le16 currentSSCsize; /792h,
    pub /: *mut *mut char expanderFwVersion[12]; /794h,
    pub /: *mut *mut __le16 PFKTrialTimeRemaining; /7A0h,
    pub /: *mut *mut __le16 cacheMemorySize; /7A2h,

    pub reserved:5: u32,
    pub activePassive:2: u32,
    pub supportConfigAutoBalance:1: u32,
    pub mpio:1: u32,
    pub supportDataLDonSSCArray:1: u32,
    pub supportPointInTimeProgress:1: u32,
    pub supportUnevenSpans:1: u32,
    pub dedicatedHotSparesLimited:1: u32,
    pub headlessMode:1: u32,
    pub supportEmulatedDrives:1: u32,
    pub supportResetNow:1: u32,
    pub realTimeScheduler:1: u32,
    pub supportSSDPatrolRead:1: u32,
    pub supportPerfTuning:1: u32,
    pub disableOnlinePFKChange:1: u32,
    pub supportJBOD:1: u32,
    pub supportBootTimePFKChange:1: u32,
    pub supportSetLinkSpeed:1: u32,
    pub supportEmergencySpares:1: u32,
    pub supportSuspendResumeBGops:1: u32,
    pub blockSSDWriteCacheChange:1: u32,
    pub supportShieldState:1: u32,
    pub supportLdBBMInfo:1: u32,
    pub supportLdPIType3:1: u32,
    pub supportLdPIType2:1: u32,
    pub supportLdPIType1:1: u32,
    pub supportPIcontroller:1: u32,

    pub supportPIcontroller:1: u32,
    pub supportLdPIType1:1: u32,
    pub supportLdPIType2:1: u32,
    pub supportLdPIType3:1: u32,
    pub supportLdBBMInfo:1: u32,
    pub supportShieldState:1: u32,
    pub blockSSDWriteCacheChange:1: u32,
    pub supportSuspendResumeBGops:1: u32,
    pub supportEmergencySpares:1: u32,
    pub supportSetLinkSpeed:1: u32,
    pub supportBootTimePFKChange:1: u32,
    pub supportJBOD:1: u32,
    pub disableOnlinePFKChange:1: u32,
    pub supportPerfTuning:1: u32,
    pub supportSSDPatrolRead:1: u32,
    pub realTimeScheduler:1: u32,
    pub supportResetNow:1: u32,
    pub supportEmulatedDrives:1: u32,
    pub headlessMode:1: u32,
    pub dedicatedHotSparesLimited:1: u32,
    pub supportUnevenSpans:1: u32,
    pub supportPointInTimeProgress:1: u32,
    pub supportDataLDonSSCArray:1: u32,
    pub mpio:1: u32,
    pub supportConfigAutoBalance:1: u32,
    pub activePassive:2: u32,
    pub reserved:5: u32,

    pub adapterOperations2: },
    pub /: *mut *mut u8 driverVersion[32]; /7A8h,
    pub /: *mut *mut u8 maxDAPdCountSpinup60; /7C8h,
    pub /: *mut *mut u8 temperatureROC; /7C9h,
    pub /: *mut *mut u8 temperatureCtrl; /7CAh,
    pub /: *mut *mut u8 reserved4; /7CBh,
    pub /: *mut *mut __le16 maxConfigurablePds; /7CCh,
    pub /: *mut *mut u8 reserved5[2]; /0x7CDh,
//
// HA cluster information
//

    pub reserved:25: u32,
    pub passive:1: u32,
    pub premiumFeatureMismatch:1: u32,
    pub ctrlPropIncompatible:1: u32,
    pub fwVersionMismatch:1: u32,
    pub hwIncompatible:1: u32,
    pub peerIsIncompatible:1: u32,
    pub peerIsPresent:1: u32,

    pub peerIsPresent:1: u32,
    pub peerIsIncompatible:1: u32,
    pub hwIncompatible:1: u32,
    pub fwVersionMismatch:1: u32,
    pub ctrlPropIncompatible:1: u32,
    pub premiumFeatureMismatch:1: u32,
    pub passive:1: u32,
    pub reserved:25: u32,

    pub cluster: },
    pub /: *mut *mut char clusterId[MEGASAS_CLUSTER_ID_SIZE]; /0x7D4,
    pub /*0x7E4*/: *mut u8 maxVFsSupported;,
    pub /*0x7E5*/: *mut u8 numVFsEnabled;,
    pub 2:VF2*/: *mut *mut u8 requestorId; /0x7E6 0:PF, 1:VF1,,
    pub /*0x7E7*/: *mut u8 reserved;,
    pub iov: },

    pub reserved:7: u32,
    pub useSeqNumJbodFP:1: u32,
    pub supportExtendedSSCSize:1: u32,
    pub supportDiskCacheSettingForSysPDs:1: u32,
    pub supportCPLDUpdate:1: u32,
    pub supportTTYLogCompression:1: u32,
    pub discardCacheDuringLDDelete:1: u32,
    pub supportSecurityonJBOD:1: u32,
    pub supportCacheBypassModes:1: u32,
    pub supportDisableSESMonitoring:1: u32,
    pub supportForceFlash:1: u32,
    pub supportNVDRAM:1: u32,
    pub supportDrvActivityLEDSetting:1: u32,
    pub supportAllowedOpsforDrvRemoval:1: u32,
    pub supportHOQRebuild:1: u32,
    pub supportForceTo512e:1: u32,
    pub supportNVCacheErase:1: u32,
    pub supportDebugQueue:1: u32,
    pub supportSwZone:1: u32,
    pub supportCrashDump:1: u32,
    pub supportMaxExtLDs:1: u32,
    pub supportT10RebuildAssist:1: u32,
    pub supportDisableImmediateIO:1: u32,
    pub supportThermalPollInterval:1: u32,
    pub supportPersonalityChange:2: u32,

    pub supportPersonalityChange:2: u32,
    pub supportThermalPollInterval:1: u32,
    pub supportDisableImmediateIO:1: u32,
    pub supportT10RebuildAssist:1: u32,
    pub supportMaxExtLDs:1: u32,
    pub supportCrashDump:1: u32,
    pub supportSwZone:1: u32,
    pub supportDebugQueue:1: u32,
    pub supportNVCacheErase:1: u32,
    pub supportForceTo512e:1: u32,
    pub supportHOQRebuild:1: u32,
    pub supportAllowedOpsforDrvRemoval:1: u32,
    pub supportDrvActivityLEDSetting:1: u32,
    pub supportNVDRAM:1: u32,
    pub supportForceFlash:1: u32,
    pub supportDisableSESMonitoring:1: u32,
    pub supportCacheBypassModes:1: u32,
    pub supportSecurityonJBOD:1: u32,
    pub discardCacheDuringLDDelete:1: u32,
    pub supportTTYLogCompression:1: u32,
    pub supportCPLDUpdate:1: u32,
    pub supportDiskCacheSettingForSysPDs:1: u32,
    pub supportExtendedSSCSize:1: u32,
    pub useSeqNumJbodFP:1: u32,
    pub reserved:7: u32,

    pub adapterOperations3: },

    pub reserved:7: u8,
// Indicates whether the CPLD image is part of
// the package and stored in flash
//
    pub cpld_in_flash:1: u8,

    pub cpld_in_flash:1: u8,
    pub reserved:7: u8,
    pub reserved1: [u8; 3],
// Null terminated string. Has the version
// information if cpld_in_flash = FALSE
//
    pub userCodeDefinition: [u8; 12],
    pub /: *mut *mut } cpld; / Valid only if upgradableCPLD is TRUE,

    pub reserved:2: u16,
    pub support_nvme_passthru:1: u16,
    pub support_pl_debug_info:1: u16,
    pub support_flash_comp_info:1: u16,
    pub support_host_info:1: u16,
    pub support_dual_fw_update:1: u16,
    pub support_ssc_rev3:1: u16,
    pub fw_swaps_bbu_vpd_info:1: u16,
    pub support_pd_map_target_id:1: u16,
    pub support_ses_ctrl_in_multipathcfg:1: u16,
    pub image_upload_supported:1: u16,
    pub support_encrypted_mfc:1: u16,
    pub supported_enc_algo:1: u16,
    pub support_ibutton_less:1: u16,
    pub ctrl_info_ext_supported:1: u16,

    pub ctrl_info_ext_supported:1: u16,
    pub support_ibutton_less:1: u16,
    pub supported_enc_algo:1: u16,
    pub support_encrypted_mfc:1: u16,
    pub image_upload_supported:1: u16,
// FW supports LUN based association and target port based
    pub support_ses_ctrl_in_multipathcfg:1: u16,
// association for the SES device connected in multipath mode
// FW defines Jbod target Id within MR_PD_CFG_SEQ
    pub support_pd_map_target_id:1: u16,
// FW swaps relevant fields in MR_BBU_VPD_INFO_FIXED to
// provide the data in little endian order
//
    pub fw_swaps_bbu_vpd_info:1: u16,
    pub support_ssc_rev3:1: u16,
// FW supports CacheCade 3.0, only one SSCD creation allowed
    pub support_dual_fw_update:1: u16,
// FW supports dual firmware update feature
    pub support_host_info:1: u16,
// FW supports MR_DCMD_CTRL_HOST_INFO_SET/GET
    pub support_flash_comp_info:1: u16,
// FW supports MR_DCMD_CTRL_FLASH_COMP_INFO_GET
    pub support_pl_debug_info:1: u16,
// FW supports retrieval of PL debug information through apps
    pub support_nvme_passthru:1: u16,
// FW supports NVMe passthru commands
    pub reserved:2: u16,

    pub adapter_operations4: },
    pub /: *mut *mut u8 pad[0x800 - 0x7FE]; / 0x7FE pad to 2K for expansion,
    pub size: u32,
    pub pad1: u32,
    pub reserved6: [u8; 64],
    pub reserved:19: u32,
    pub 1: u32 support_pci_lane_margining:,
    pub support_psoc_update:1: u32,
    pub support_force_personality_change:1: u32,
    pub support_fde_type_mix:1: u32,
    pub support_snap_dump:1: u32,
    pub support_nvme_tm:1: u32,
    pub support_oce_only:1: u32,
    pub support_ext_mfg_vpd:1: u32,
    pub support_pcie:1: u32,
    pub support_cvhealth_info:1: u32,
    pub support_profile_change:2: u32,
    pub mr_config_ext2_supported:1: u32,

    pub mr_config_ext2_supported:1: u32,
    pub support_profile_change:2: u32,
    pub support_cvhealth_info:1: u32,
    pub support_pcie:1: u32,
    pub support_ext_mfg_vpd:1: u32,
    pub support_oce_only:1: u32,
    pub support_nvme_tm:1: u32,
    pub support_snap_dump:1: u32,
    pub support_fde_type_mix:1: u32,
    pub support_force_personality_change:1: u32,
    pub support_psoc_update:1: u32,
    pub 1: u32 support_pci_lane_margining:,
    pub reserved:19: u32,

    pub adapter_operations5: },
    pub rsvdForAdptOp: [u32; 63],
    pub reserved7: [u8; 3],
    pub /: *mut *mut u8 TaskAbortTO; / Timeout value in seconds used by Abort Task TM,
    pub /: *mut *mut u8 MaxResetTO; / Max Supported Reset timeout in seconds.,
    pub reserved8: [u8; 3],
    pub __packed: },
//
// ===============================
// MegaRAID SAS driver definitions
// ===============================
//
pub const MEGASAS_MAX_PD_CHANNELS: c_int = 2;
pub const MEGASAS_MAX_LD_CHANNELS: c_int = 2;

pub const MEGASAS_MAX_DEV_PER_CHANNEL: c_int = 128;

pub const MEGASAS_MAX_LUN: c_int = 8;
pub const MEGASAS_DEFAULT_CMD_PER_LUN: c_int = 256;

pub const MEGASAS_MAX_SUPPORTED_LD_IDS: c_int = 240;

pub const MEGASAS_DBG_LVL: c_int = 1;
pub const MEGASAS_FW_BUSY: c_int = 1;
// Driver's internal Logging levels

pub const SCAN_PD_CHANNEL: c_uint = 0x1;
pub const SCAN_VD_CHANNEL: c_uint = 0x2;
pub const MEGASAS_KDUMP_QUEUE_DEPTH: c_int = 100;

pub const MR_R1_LDIO_PIGGYBACK_DEFAULT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MR_SCSI_CMD_TYPE {
    READ_WRITE_LDIO = 0,
    NON_READ_WRITE_LDIO = 1,
    READ_WRITE_SYSPDIO = 2,
    NON_READ_WRITE_SYSPDIO = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DCMD_TIMEOUT_ACTION {
    INITIATE_OCR = 0,
    KILL_ADAPTER = 1,
    IGNORE_TIMEOUT = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FW_BOOT_CONTEXT {
    PROBE_CONTEXT = 0,
    OCR_CONTEXT = 1,
}

// Frame Type
pub const IO_FRAME: c_int = 0;
pub const PTHRU_FRAME: c_int = 1;
//
// When SCSI mid-layer calls driver's reset routine, driver waits for
// MEGASAS_RESET_WAIT_TIME seconds for all outstanding IO to complete. Note
// that the driver cannot _actually_ abort or reset pending commands. While
// it is waiting for the commands to complete, it prints a diagnostic message
// every MEGASAS_RESET_NOTICE_INTERVAL seconds
//
pub const MEGASAS_RESET_WAIT_TIME: c_int = 180;
pub const MEGASAS_INTERNAL_CMD_WAIT_TIME: c_int = 180;
pub const MEGASAS_RESET_NOTICE_INTERVAL: c_int = 5;
pub const MEGASAS_IOCTL_CMD: c_int = 0;
pub const MEGASAS_DEFAULT_CMD_TIMEOUT: c_int = 90;
pub const MEGASAS_THROTTLE_QUEUE_DEPTH: c_int = 16;
pub const MEGASAS_DEFAULT_TM_TIMEOUT: c_int = 50;
//
// FW reports the maximum of number of commands that it can accept (maximum
// commands that can be outstanding) at any time. The driver must report a
// lower number to the mid layer because it can issue a few internal commands
// itself (E.g, AEN, abort cmd, IOCTLs etc). The number of commands it needs
// is shown below
//
pub const MEGASAS_INT_CMDS: c_int = 32;
pub const MEGASAS_SKINNY_INT_CMDS: c_int = 5;
pub const MEGASAS_FUSION_INTERNAL_CMDS: c_int = 8;
pub const MEGASAS_FUSION_IOCTL_CMDS: c_int = 3;
pub const MEGASAS_MFI_IOCTL_CMDS: c_int = 27;
pub const MEGASAS_MAX_MSIX_QUEUES: c_int = 128;
//
// FW can accept both 32 and 64 bit SGLs. We want to allocate 32/64 bit
// SGLs based on the size of dma_addr_t
//

pub const MFI_XSCALE_OMR0_CHANGE_INTERRUPT: c_uint = 0x00000001;
pub const MFI_INTR_FLAG_REPLY_MESSAGE: c_uint = 0x00000001;
pub const MFI_INTR_FLAG_FIRMWARE_STATE_CHANGE: c_uint = 0x00000002;
pub const MFI_G2_OUTBOUND_DOORBELL_CHANGE_INTERRUPT: c_uint = 0x00000004;
pub const MFI_OB_INTR_STATUS_MASK: c_uint = 0x00000002;
pub const MFI_POLL_TIMEOUT_SECS: c_int = 60;
pub const MFI_IO_TIMEOUT_SECS: c_int = 180;

pub const MEGASAS_SRIOV_MAX_RESET_TRIES_VF: c_int = 1;
pub const MEGASAS_ROUTINE_WAIT_TIME_VF: c_int = 300;
pub const MFI_REPLY_1078_MESSAGE_INTERRUPT: c_uint = 0x80000000;
pub const MFI_REPLY_GEN2_MESSAGE_INTERRUPT: c_uint = 0x00000001;

pub const MFI_REPLY_SKINNY_MESSAGE_INTERRUPT: c_uint = 0x40000000;

pub const MFI_1068_PCSR_OFFSET: c_uint = 0x84;
pub const MFI_1068_FW_HANDSHAKE_OFFSET: c_uint = 0x64;
pub const MFI_1068_FW_READY: c_uint = 0xDDDD0000;

pub const MR_MAX_REPLY_QUEUES_EXT_OFFSET_SHIFT: c_int = 14;
pub const MR_MAX_MSIX_REG_ARRAY: c_int = 16;

pub const MR_MAX_RAID_MAP_SIZE_OFFSET_SHIFT: c_int = 16;
pub const MR_MAX_RAID_MAP_SIZE_MASK: c_uint = 0x1FF;
pub const MR_MIN_MAP_SIZE: c_uint = 0x10000;
// 64k

pub const MEGASAS_WATCHDOG_THREAD_INTERVAL: c_int = 1000;
pub const MEGASAS_WAIT_FOR_NEXT_DMA_MSECS: c_int = 20;
pub const MEGASAS_WATCHDOG_WAIT_COUNT: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MR_ADAPTER_TYPE {
    MFI_SERIES = 1,
    THUNDERBOLT_SERIES = 2,
    INVADER_SERIES = 3,
    VENTURA_SERIES = 4,
    AERO_SERIES = 5,
}

//
// register set for both 1068 and 1078 controllers
// structure extended for 1078 registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_register_set {
    pub /*0000h*/: *mut u32 doorbell;,
    pub /*0004h*/: *mut u32 fusion_seq_offset;,
    pub /*0008h*/: *mut u32 fusion_host_diag;,
    pub /*000Ch*/: *mut u32 reserved_01;,
    pub /*0010h*/: *mut u32 inbound_msg_0;,
    pub /*0014h*/: *mut u32 inbound_msg_1;,
    pub /*0018h*/: *mut u32 outbound_msg_0;,
    pub /*001Ch*/: *mut u32 outbound_msg_1;,
    pub /*0020h*/: *mut u32 inbound_doorbell;,
    pub /*0024h*/: *mut u32 inbound_intr_status;,
    pub /*0028h*/: *mut u32 inbound_intr_mask;,
    pub /*002Ch*/: *mut u32 outbound_doorbell;,
    pub /*0030h*/: *mut u32 outbound_intr_status;,
    pub /*0034h*/: *mut u32 outbound_intr_mask;,
    pub /*0038h*/: *mut u32 reserved_1[2];,
    pub /*0040h*/: *mut u32 inbound_queue_port;,
    pub /*0044h*/: *mut u32 outbound_queue_port;,
    pub /*0048h*/: *mut u32 reserved_2[9];,
    pub /*006Ch*/: *mut u32 reply_post_host_index;,
    pub /*0070h*/: *mut u32 reserved_2_2[12];,
    pub /*00A0h*/: *mut u32 outbound_doorbell_clear;,
    pub /*00A4h*/: *mut u32 reserved_3[3];,
    pub /*00B0h*/: *mut u32 outbound_scratch_pad_0;,
    pub /*00B4h*/: *mut u32 outbound_scratch_pad_1;,
    pub /*00B8h*/: *mut u32 outbound_scratch_pad_2;,
    pub /*00BCh*/: *mut u32 outbound_scratch_pad_3;,
    pub /*00C0h*/: *mut u32 inbound_low_queue_port ;,
    pub /*00C4h*/: *mut u32 inbound_high_queue_port ;,
    pub /*00C8h*/: *mut u32 inbound_single_queue_port;,
    pub /*CCh*/: *mut u32 res_6[11];,
    pub host_diag: u32,
    pub seq_offset: u32,
    pub /*00CCh*/: *mut u32 index_registers[807];,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_sge32 {
    pub phys_addr: __le32,
    pub length: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_sge64 {
    pub phys_addr: __le64,
    pub length: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_sge_skinny {
    pub phys_addr: __le64,
    pub length: __le32,
    pub flag: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union megasas_sgl {
    pub sge32): DECLARE_FLEX_ARRAY(struct megasas_sge32,,
    pub sge64): DECLARE_FLEX_ARRAY(struct megasas_sge64,,
    pub sge_skinny): DECLARE_FLEX_ARRAY(struct megasas_sge_skinny,,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_header {
    pub /: *mut *mut u8 cmd; /00h,
    pub /: *mut *mut u8 sense_len; /01h,
    pub /: *mut *mut u8 cmd_status; /02h,
    pub /: *mut *mut u8 scsi_status; /03h,
    pub /: *mut *mut u8 target_id; /04h,
    pub /: *mut *mut u8 lun; /05h,
    pub /: *mut *mut u8 cdb_len; /06h,
    pub /: *mut *mut u8 sge_count; /07h,
    pub /: *mut *mut __le32 context; /08h,
    pub /: *mut *mut __le32 pad_0; /0Ch,
    pub /: *mut *mut __le16 flags; /10h,
    pub /: *mut *mut __le16 timeout; /12h,
    pub /: *mut *mut __le32 data_xferlen; /14h,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub union megasas_sgl_frame {
    pub sge32: [megasas_sge32; 8],
    pub sge64: [megasas_sge64; 5],
// C attribute field omitted

    pub reserved:15: u32,
    pub support_memdump:1: u32,
    pub support_fw_exposed_dev_list:1: u32,
    pub support_nvme_passthru:1: u32,
    pub support_64bit_mode:1: u32,
    pub support_pd_map_target_id:1: u32,
    pub support_qd_throttling:1: u32,
    pub support_fp_rlbypass:1: u32,
    pub support_vfid_in_ioframe:1: u32,
    pub support_ext_io_size:1: u32,
    pub support_ext_queue_depth:1: u32,
    pub security_protocol_cmds_fw:1: u32,
    pub support_core_affinity:1: u32,
    pub support_ndrive_r1_lb:1: u32,
    pub support_max_255lds:1: u32,
    pub support_fastpath_wb:1: u32,
    pub support_additional_msix:1: u32,
    pub support_fp_remote_lun:1: u32,

    pub support_fp_remote_lun:1: u32,
    pub support_additional_msix:1: u32,
    pub support_fastpath_wb:1: u32,
    pub support_max_255lds:1: u32,
    pub support_ndrive_r1_lb:1: u32,
    pub support_core_affinity:1: u32,
    pub security_protocol_cmds_fw:1: u32,
    pub support_ext_queue_depth:1: u32,
    pub support_ext_io_size:1: u32,
    pub support_vfid_in_ioframe:1: u32,
    pub support_fp_rlbypass:1: u32,
    pub support_qd_throttling:1: u32,
    pub support_pd_map_target_id:1: u32,
    pub support_64bit_mode:1: u32,
    pub support_nvme_passthru:1: u32,
    pub support_fw_exposed_dev_list:1: u32,
    pub support_memdump:1: u32,
    pub reserved:15: u32,

    pub mfi_capabilities: },
    pub reg: __le32,
    pub MFI_CAPABILITIES: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_init_frame {
    pub /: *mut *mut u8 cmd; /00h,
    pub /: *mut *mut u8 reserved_0; /01h,
    pub /: *mut *mut u8 cmd_status; /02h,
    pub /: *mut *mut u8 reserved_1; /03h,
    pub /*04h*/: *mut MFI_CAPABILITIES driver_operations;,
    pub /: *mut *mut __le32 context; /08h,
    pub /: *mut *mut __le32 pad_0; /0Ch,
    pub /: *mut *mut __le16 flags; /10h,
    pub /: *mut *mut __le16 replyqueue_mask; /12h,
    pub /: *mut *mut __le32 data_xfer_len; /14h,
    pub /: *mut *mut __le32 queue_info_new_phys_addr_lo; /18h,
    pub /: *mut *mut __le32 queue_info_new_phys_addr_hi; /1Ch,
    pub /: *mut *mut __le32 queue_info_old_phys_addr_lo; /20h,
    pub /: *mut *mut __le32 queue_info_old_phys_addr_hi; /24h,
    pub /: *mut *mut __le32 reserved_4[2]; /28h,
    pub /: *mut *mut __le32 system_info_lo; /30h,
    pub /: *mut *mut __le32 system_info_hi; /34h,
    pub /: *mut *mut __le32 reserved_5[2]; /38h,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_init_queue_info {
    pub /: *mut *mut __le32 init_flags; /00h,
    pub /: *mut *mut __le32 reply_queue_entries; /04h,
    pub /: *mut *mut __le32 reply_queue_start_phys_addr_lo; /08h,
    pub /: *mut *mut __le32 reply_queue_start_phys_addr_hi; /0Ch,
    pub /: *mut *mut __le32 producer_index_phys_addr_lo; /10h,
    pub /: *mut *mut __le32 producer_index_phys_addr_hi; /14h,
    pub /: *mut *mut __le32 consumer_index_phys_addr_lo; /18h,
    pub /: *mut *mut __le32 consumer_index_phys_addr_hi; /1Ch,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_io_frame {
    pub /: *mut *mut u8 cmd; /00h,
    pub /: *mut *mut u8 sense_len; /01h,
    pub /: *mut *mut u8 cmd_status; /02h,
    pub /: *mut *mut u8 scsi_status; /03h,
    pub /: *mut *mut u8 target_id; /04h,
    pub /: *mut *mut u8 access_byte; /05h,
    pub /: *mut *mut u8 reserved_0; /06h,
    pub /: *mut *mut u8 sge_count; /07h,
    pub /: *mut *mut __le32 context; /08h,
    pub /: *mut *mut __le32 pad_0; /0Ch,
    pub /: *mut *mut __le16 flags; /10h,
    pub /: *mut *mut __le16 timeout; /12h,
    pub /: *mut *mut __le32 lba_count; /14h,
    pub /: *mut *mut __le32 sense_buf_phys_addr_lo; /18h,
    pub /: *mut *mut __le32 sense_buf_phys_addr_hi; /1Ch,
    pub /: *mut *mut __le32 start_lba_lo; /20h,
    pub /: *mut *mut __le32 start_lba_hi; /24h,
    pub /: *mut *mut megasas_sgl sgl; /28h,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_pthru_frame {
    pub /: *mut *mut u8 cmd; /00h,
    pub /: *mut *mut u8 sense_len; /01h,
    pub /: *mut *mut u8 cmd_status; /02h,
    pub /: *mut *mut u8 scsi_status; /03h,
    pub /: *mut *mut u8 target_id; /04h,
    pub /: *mut *mut u8 lun; /05h,
    pub /: *mut *mut u8 cdb_len; /06h,
    pub /: *mut *mut u8 sge_count; /07h,
    pub /: *mut *mut __le32 context; /08h,
    pub /: *mut *mut __le32 pad_0; /0Ch,
    pub /: *mut *mut __le16 flags; /10h,
    pub /: *mut *mut __le16 timeout; /12h,
    pub /: *mut *mut __le32 data_xfer_len; /14h,
    pub /: *mut *mut __le32 sense_buf_phys_addr_lo; /18h,
    pub /: *mut *mut __le32 sense_buf_phys_addr_hi; /1Ch,
    pub /: *mut *mut u8 cdb[16]; /20h,
    pub /: *mut *mut megasas_sgl sgl; /30h,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_dcmd_frame {
    pub /: *mut *mut u8 cmd; /00h,
    pub /: *mut *mut u8 reserved_0; /01h,
    pub /: *mut *mut u8 cmd_status; /02h,
    pub /: *mut *mut u8 reserved_1[4]; /03h,
    pub /: *mut *mut u8 sge_count; /07h,
    pub /: *mut *mut __le32 context; /08h,
    pub /: *mut *mut __le32 pad_0; /0Ch,
    pub /: *mut *mut __le16 flags; /10h,
    pub /: *mut *mut __le16 timeout; /12h,
    pub /: *mut *mut __le32 data_xfer_len; /14h,
    pub /: *mut *mut __le32 opcode; /18h,
    pub b: [u8; 12],
    pub s: [__le16; 6],
    pub w: [__le32; 3],
    pub mbox: },
    pub /: *mut *mut megasas_sgl sgl; /28h,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_abort_frame {
    pub /: *mut *mut u8 cmd; /00h,
    pub /: *mut *mut u8 reserved_0; /01h,
    pub /: *mut *mut u8 cmd_status; /02h,
    pub /: *mut *mut u8 reserved_1; /03h,
    pub /: *mut *mut __le32 reserved_2; /04h,
    pub /: *mut *mut __le32 context; /08h,
    pub /: *mut *mut __le32 pad_0; /0Ch,
    pub /: *mut *mut __le16 flags; /10h,
    pub /: *mut *mut __le16 reserved_3; /12h,
    pub /: *mut *mut __le32 reserved_4; /14h,
    pub /: *mut *mut __le32 abort_context; /18h,
    pub /: *mut *mut __le32 pad_1; /1Ch,
    pub /: *mut *mut __le32 abort_mfi_phys_addr_lo; /20h,
    pub /: *mut *mut __le32 abort_mfi_phys_addr_hi; /24h,
    pub /: *mut *mut __le32 reserved_5[6]; /28h,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_smp_frame {
    pub /: *mut *mut u8 cmd; /00h,
    pub /: *mut *mut u8 reserved_1; /01h,
    pub /: *mut *mut u8 cmd_status; /02h,
    pub /: *mut *mut u8 connection_status; /03h,
    pub /: *mut *mut u8 reserved_2[3]; /04h,
    pub /: *mut *mut u8 sge_count; /07h,
    pub /: *mut *mut __le32 context; /08h,
    pub /: *mut *mut __le32 pad_0; /0Ch,
    pub /: *mut *mut __le16 flags; /10h,
    pub /: *mut *mut __le16 timeout; /12h,
    pub /: *mut *mut __le32 data_xfer_len; /14h,
    pub /: *mut *mut __le64 sas_addr; /18h,
    pub /: *mut *mut megasas_sge32 sge32[2]; / [0]: resp [1]: req,
    pub /: *mut *mut megasas_sge64 sge64[2]; / [0]: resp [1]: req,
    pub sgl: },
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_stp_frame {
    pub /: *mut *mut u8 cmd; /00h,
    pub /: *mut *mut u8 reserved_1; /01h,
    pub /: *mut *mut u8 cmd_status; /02h,
    pub /: *mut *mut u8 reserved_2; /03h,
    pub /: *mut *mut u8 target_id; /04h,
    pub /: *mut *mut u8 reserved_3[2]; /05h,
    pub /: *mut *mut u8 sge_count; /07h,
    pub /: *mut *mut __le32 context; /08h,
    pub /: *mut *mut __le32 pad_0; /0Ch,
    pub /: *mut *mut __le16 flags; /10h,
    pub /: *mut *mut __le16 timeout; /12h,
    pub /: *mut *mut __le32 data_xfer_len; /14h,
    pub /: *mut *mut __le16 fis[10]; /18h,
    pub stp_flags: __le32,
    pub /: *mut *mut megasas_sge32 sge32[2]; / [0]: resp [1]: data,
    pub /: *mut *mut megasas_sge64 sge64[2]; / [0]: resp [1]: data,
    pub sgl: },
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub union megasas_frame {
    pub hdr: megasas_header,
    pub init: megasas_init_frame,
    pub io: megasas_io_frame,
    pub pthru: megasas_pthru_frame,
    pub dcmd: megasas_dcmd_frame,
    pub abort: megasas_abort_frame,
    pub smp: megasas_smp_frame,
    pub stp: megasas_stp_frame,
    pub raw_bytes: [u8; 64],
}

//
// struct MR_PRIV_DEVICE - sdev private hostdata
// @is_tm_capable: firmware managed tm_capable flag
// @tm_busy: TM request is in progress
// @sdev_priv_busy: pending command per sdev
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_PRIV_DEVICE {
    pub is_tm_capable: bool,
    pub tm_busy: bool,
    pub sdev_priv_busy: core::sync::atomic::AtomicI32,
    pub r1_ldio_hint: core::sync::atomic::AtomicI32,
    pub interface_type: u8,
    pub task_abort_tmo: u8,
    pub target_reset_tmo: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union megasas_evt_class_locale {
    pub locale: u16,
    pub reserved: u8,
    pub class: i8,

    pub class: i8,
    pub reserved: u8,
    pub locale: u16,
// C attribute field omitted
    pub word: u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_evt_log_info {
    pub newest_seq_num: __le32,
    pub oldest_seq_num: __le32,
    pub clear_seq_num: __le32,
    pub shutdown_seq_num: __le32,
    pub boot_seq_num: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_progress {
    pub progress: __le16,
    pub elapsed_seconds: __le16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_evtarg_ld {
    pub target_id: u16,
    pub ld_index: u8,
    pub reserved: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_evtarg_pd {
    pub device_id: u16,
    pub encl_index: u8,
    pub slot_number: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_evt_detail {
    pub seq_num: __le32,
    pub time_stamp: __le32,
    pub code: __le32,
    pub cl: megasas_evt_class_locale,
    pub arg_type: u8,
    pub reserved1: [u8; 15],
    pub pd: megasas_evtarg_pd,
    pub cdb_length: u8,
    pub sense_length: u8,
    pub reserved: [u8; 2],
    pub cdb: [u8; 16],
    pub sense: [u8; 64],
// C attribute field omitted
    pub ld: megasas_evtarg_ld,
    pub ld: megasas_evtarg_ld,
    pub count: __le64,
// C attribute field omitted
    pub lba: __le64,
    pub ld: megasas_evtarg_ld,
// C attribute field omitted
    pub ld: megasas_evtarg_ld,
    pub prevOwner: __le32,
    pub newOwner: __le32,
// C attribute field omitted
    pub ld_lba: u64,
    pub pd_lba: u64,
    pub ld: megasas_evtarg_ld,
    pub pd: megasas_evtarg_pd,
// C attribute field omitted
    pub ld: megasas_evtarg_ld,
    pub prog: megasas_progress,
// C attribute field omitted
    pub ld: megasas_evtarg_ld,
    pub prev_state: u32,
    pub new_state: u32,
// C attribute field omitted
    pub strip: u64,
    pub ld: megasas_evtarg_ld,
// C attribute field omitted
    pub pd: megasas_evtarg_pd,
    pub pd: megasas_evtarg_pd,
    pub err: u32,
// C attribute field omitted
    pub lba: u64,
    pub pd: megasas_evtarg_pd,
// C attribute field omitted
    pub lba: u64,
    pub pd: megasas_evtarg_pd,
    pub ld: megasas_evtarg_ld,
// C attribute field omitted
    pub pd: megasas_evtarg_pd,
    pub prog: megasas_progress,
// C attribute field omitted
    pub pd: megasas_evtarg_pd,
    pub prevState: u32,
    pub newState: u32,
// C attribute field omitted
    pub vendorId: u16,
    pub deviceId: __le16,
    pub subVendorId: u16,
    pub subDeviceId: u16,
// C attribute field omitted
    pub rate: u32,
    pub str: [c_char; 96],
    pub rtc: u32,
    pub elapsedSeconds: u32,
// C attribute field omitted
    pub ecar: u32,
    pub elog: u32,
    pub str: [c_char; 64],
// C attribute field omitted
    pub b: [u8; 96],
    pub s: [__le16; 48],
    pub w: [__le32; 24],
    pub d: [__le64; 12],
    pub args: },
    pub description: [c_char; 128],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_aen_event {
    pub hotplug_work: delayed_work,
    pub instance: *mut megasas_instance,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_irq_context {
    pub name: [c_char; MEGASAS_MSIX_NAME_LEN],
    pub instance: *mut megasas_instance,
    pub MSIxIndex: u32,
    pub os_irq: u32,
    pub irqpoll: irq_poll,
    pub irq_poll_scheduled: bool,
    pub irq_line_enable: bool,
    pub in_used: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_DRV_SYSTEM_INFO {
    pub infoVersion: u8,
    pub systemIdLength: u8,
    pub reserved0: u16,
    pub systemId: [u8; 64],
    pub reserved: [u8; 1980],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MR_PD_TYPE {
    UNKNOWN_DRIVE = 0,
    PARALLEL_SCSI = 1,
    SAS_PD = 2,
    SATA_PD = 3,
    FC_PD = 4,
    NVME_PD = 5,
}

// JBOD Queue depth definitions
pub const MEGASAS_SATA_QD: c_int = 32;
pub const MEGASAS_SAS_QD: c_int = 256;
pub const MEGASAS_DEFAULT_PD_QD: c_int = 64;
pub const MEGASAS_NVME_QD: c_int = 64;
pub const MR_DEFAULT_NVME_PAGE_SIZE: c_int = 4096;
pub const MR_DEFAULT_NVME_PAGE_SHIFT: c_int = 12;
pub const MR_DEFAULT_NVME_MDTS_KB: c_int = 128;
pub const MR_NVME_PAGE_SIZE_MASK: c_uint = 0x000000FF;
// Aero performance parameters
pub const MR_HIGH_IOPS_QUEUE_COUNT: c_int = 8;
pub const MR_DEVICE_HIGH_IOPS_DEPTH: c_int = 8;
pub const MR_HIGH_IOPS_BATCH_COUNT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MR_PERF_MODE {
    MR_BALANCED_PERF_MODE		= 0,
    MR_IOPS_PERF_MODE		= 1,
    MR_LATENCY_PERF_MODE		= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MEGASAS_LD_TARGET_ID_STATUS {
    LD_TARGET_ID_INITIAL,
    LD_TARGET_ID_ACTIVE,
    LD_TARGET_ID_DELETED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_instance {
    pub reply_map: *mut c_uint,
    pub producer: *mut __le32,
    pub producer_h: dma_addr_t,
    pub consumer: *mut __le32,
    pub consumer_h: dma_addr_t,
    pub system_info_buf: *mut MR_DRV_SYSTEM_INFO,
    pub system_info_h: dma_addr_t,
    pub vf_affiliation: *mut MR_LD_VF_AFFILIATION,
    pub vf_affiliation_h: dma_addr_t,
    pub vf_affiliation_111: *mut MR_LD_VF_AFFILIATION_111,
    pub vf_affiliation_111_h: dma_addr_t,
    pub hb_host_mem: *mut MR_CTRL_HB_HOST_MEM,
    pub hb_host_mem_h: dma_addr_t,
    pub pd_info: *mut MR_PD_INFO,
    pub pd_info_h: dma_addr_t,
    pub tgt_prop: *mut MR_TARGET_PROPERTIES,
    pub tgt_prop_h: dma_addr_t,
    pub reply_queue: *mut __le32,
    pub reply_queue_h: dma_addr_t,
    pub crash_dump_buf: *mut u32,
    pub crash_dump_h: dma_addr_t,
    pub pd_list_buf: *mut MR_PD_LIST,
    pub pd_list_buf_h: dma_addr_t,
    pub ctrl_info_buf: *mut megasas_ctrl_info,
    pub ctrl_info_buf_h: dma_addr_t,
    pub ld_list_buf: *mut MR_LD_LIST,
    pub ld_list_buf_h: dma_addr_t,
    pub ld_targetid_list_buf: *mut MR_LD_TARGETID_LIST,
    pub ld_targetid_list_buf_h: dma_addr_t,
    pub host_device_list_buf: *mut MR_HOST_DEVICE_LIST,
    pub host_device_list_buf_h: dma_addr_t,
    pub snapdump_prop: *mut MR_SNAPDUMP_PROPERTIES,
    pub snapdump_prop_h: dma_addr_t,
    pub crash_buf: [*mut c_void; MAX_CRASH_DUMP_SIZE],
    pub fw_crash_buffer_size: c_uint,
    pub fw_crash_state: c_uint,
    pub fw_crash_buffer_offset: c_uint,
    pub drv_buf_index: u32,
    pub drv_buf_alloc: u32,
    pub crash_dump_fw_support: u32,
    pub crash_dump_drv_support: u32,
    pub crash_dump_app_support: u32,
    pub secure_jbod_support: u32,
    pub /: *mut *mut u32 support_morethan256jbod; / FW support for more than 256 PD/JBOD,
    pub /: *mut *mut bool use_seqnum_jbod_fp; / Added for PD sequence,
    pub smp_affinity_enable: bool,
    pub crashdump_lock: mutex,
    pub reg_set: *mut megasas_register_set __iomem,
    pub reply_post_host_index_addr: [*mut u32 __iomem; MR_MAX_MSIX_REG_ARRAY],
    pub pd_list: [megasas_pd_list; MEGASAS_MAX_PD],
    pub local_pd_list: [megasas_pd_list; MEGASAS_MAX_PD],
    pub ld_ids: [u8; MEGASAS_MAX_LD_IDS],
    pub ld_tgtid_status: [u8; MEGASAS_MAX_LD_IDS],
    pub ld_ids_prev: [u8; MEGASAS_MAX_LD_IDS],
    pub ld_ids_from_raidmap: [u8; MEGASAS_MAX_LD_IDS],
    pub init_id: i8,
    pub max_num_sge: u16,
    pub max_fw_cmds: u16,
    pub max_mpt_cmds: u16,
    pub max_mfi_cmds: u16,
    pub max_scsi_cmds: u16,
    pub ldio_threshold: u16,
    pub cur_can_queue: u16,
    pub max_sectors_per_req: u32,
    pub msix_load_balance: bool,
    pub ev: *mut megasas_aen_event,
    pub cmd_list: *mut megasas_cmd,
    pub cmd_pool: list_head,
// used to sync fire the cmd to fw
    pub mfi_pool_lock: spinlock_t,
// used to sync fire the cmd to fw
    pub hba_lock: spinlock_t,
// used to synch producer, consumer ptrs in dpc
    pub stream_lock: spinlock_t,
    pub completion_lock: spinlock_t,
    pub frame_dma_pool: *mut dma_pool,
    pub sense_dma_pool: *mut dma_pool,
    pub evt_detail: *mut megasas_evt_detail,
    pub evt_detail_h: dma_addr_t,
    pub aen_cmd: *mut megasas_cmd,
    pub ioctl_sem: semaphore,
    pub host: *mut Scsi_Host,
    pub int_cmd_wait_q: wait_queue_head_t,
    pub abort_cmd_wait_q: wait_queue_head_t,
    pub pdev: *mut pci_dev,
    pub unique_id: u32,
    pub fw_support_ieee: u32,
    pub threshold_reply_count: u32,
    pub fw_outstanding: core::sync::atomic::AtomicI32,
    pub ldio_outstanding: core::sync::atomic::AtomicI32,
    pub fw_reset_no_pci_access: core::sync::atomic::AtomicI32,
    pub total_io_count: core::sync::atomic::AtomicI64,
    pub high_iops_outstanding: core::sync::atomic::AtomicI64,
    pub instancet: *mut megasas_instance_template,
    pub isr_tasklet: tasklet_struct,
    pub work_init: work_struct,
    pub fw_fault_work: delayed_work,
    pub fw_fault_work_q: *mut workqueue_struct,
    pub fault_handler_work_q_name: [c_char; 48],
    pub flag: u8,
    pub unload: u8,
    pub flag_ieee: u8,
    pub issuepend_done: u8,
    pub disableOnlineCtrlReset: u8,
    pub UnevenSpanSupport: u8,
    pub supportmax256vd: u8,
    pub pd_list_not_supported: u8,
    pub fw_supported_vd_count: u16,
    pub fw_supported_pd_count: u16,
    pub drv_supported_vd_count: u16,
    pub drv_supported_pd_count: u16,
    pub adprecovery: core::sync::atomic::AtomicI32,
    pub last_time: c_ulong,
    pub mfiStatus: u32,
    pub last_seq_num: u32,
    pub internal_reset_pending_q: list_head,
// Ptr to hba specific information
    pub ctrl_context: *mut c_void,
    pub msix_vectors: c_uint,
    pub irq_context: [megasas_irq_context; MEGASAS_MAX_MSIX_QUEUES],
    pub map_id: u64,
    pub pd_seq_map_id: u64,
    pub map_update_cmd: *mut megasas_cmd,
    pub jbod_seq_cmd: *mut megasas_cmd,
    pub bar: c_ulong,
    pub reset_flags: c_long,
    pub reset_mutex: mutex,
    pub sriov_heartbeat_timer: timer_list,
    pub skip_heartbeat_timer_del: c_char,
    pub requestorId: u8,
    pub PlasmaFW111: c_char,
    pub clusterId: [c_char; MEGASAS_CLUSTER_ID_SIZE],
    pub peerIsPresent: u8,
    pub passive: u8,
    pub throttlequeuedepth: u16,
    pub mask_interrupts: u8,
    pub max_chain_frame_sz: u16,
    pub is_imr: u8,
    pub is_rdpq: u8,
    pub dev_handle: bool,
    pub fw_sync_cache_support: bool,
    pub mfi_frame_size: u32,
    pub msix_combined: bool,
    pub max_raid_mapsize: u16,
// preffered count to send as LDIO irrspective of FP capable.
    pub r1_ldio_hint_default: u8,
    pub nvme_page_size: u32,
    pub adapter_type: u8,
    pub consistent_mask_64bit: bool,
    pub support_nvme_passthru: bool,
    pub enable_sdev_max_qd: bool,
    pub task_abort_tmo: u8,
    pub max_reset_tmo: u8,
    pub snapdump_wait_time: u8,

    pub debugfs_root: *mut dentry,
    pub raidmap_dump: *mut dentry,

    pub enable_fw_dev_list: u8,
    pub atomic_desc_support: bool,
    pub support_seqnum_jbod_fp: bool,
    pub support_pci_lane_margining: bool,
    pub low_latency_index_start: u8,
    pub perf_mode: c_int,
    pub iopoll_q_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_LD_VF_MAP {
    pub size: u32,
    pub ref: MR_LD_REF,
    pub ldVfCount: u8,
    pub reserved: [u8; 6],
    pub policy: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_LD_VF_AFFILIATION {
    pub size: u32,
    pub ldCount: u8,
    pub vfCount: u8,
    pub thisVf: u8,
    pub reserved: [u8; 9],
    pub map: [MR_LD_VF_MAP; 1],
}

// Plasma 1.11 FW backward compatibility structures
pub const IOV_111_OFFSET: c_uint = 0x7CE;
pub const MAX_VIRTUAL_FUNCTIONS: c_int = 8;
pub const MR_LD_ACCESS_HIDDEN: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IOV_111 {
    pub maxVFsSupported: u8,
    pub numVFsEnabled: u8,
    pub requestorId: u8,
    pub reserved: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_LD_VF_MAP_111 {
    pub targetId: u8,
    pub reserved: [u8; 3],
    pub policy: [u8; MAX_VIRTUAL_FUNCTIONS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_LD_VF_AFFILIATION_111 {
    pub vdCount: u8,
    pub vfCount: u8,
    pub thisVf: u8,
    pub reserved: [u8; 5],
    pub map: [MR_LD_VF_MAP_111; MAX_LOGICAL_DRIVES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_CTRL_HB_HOST_MEM {
    pub /: *mut *mut u32 fwCounter; / Firmware heart beat counter,
    pub mode.: *mut *mut u32 debugmode:1; / 1=Firmware is in debug,
    pub reserved:31: u32,
    pub debug: },
    pub reserved_fw: [u32; 6],
    pub /: *mut *mut u32 driverCounter; / Driver heart beat counter. 0x20,
    pub reserved_driver: [u32; 7],
    pub HB: },
    pub pad: [u8; 0x400-0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_instance_template {
    pub ): *mut u32, struct megasas_register_set __iomem,
    pub ): *mut *mut void (enable_intr)(struct megasas_instance,
    pub ): *mut *mut void (disable_intr)(struct megasas_instance,
    pub ): *mut *mut int (clear_intr)(struct megasas_instance,
    pub ): *mut *mut u32 (read_fw_status_reg)(struct megasas_instance,
    pub ): *mut megasas_register_set __iomem,
    pub ): *mut megasas_register_set __iomem,
    pub devp): *mut *mut irqreturn_t (service_isr)(int irq, void,
    pub long): *mut *mut void (tasklet)(unsigned,
    pub ): *mut *mut u32 (init_adapter)(struct megasas_instance,
    pub ): *mut scsi_cmnd,
    pub cmd): *mut megasas_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_cmd {
    pub frame: *mut megasas_frame,
    pub frame_phys_addr: dma_addr_t,
    pub sense: *mut u8,
    pub sense_phys_addr: dma_addr_t,
    pub index: u32,
    pub sync_cmd: u8,
    pub cmd_status_drv: u8,
    pub abort_aen: u8,
    pub retry_for_fw_reset: u8,
    pub list: list_head,
    pub scmd: *mut scsi_cmnd,
    pub flags: u8,
    pub instance: *mut megasas_instance,
    pub smid: u16,
    pub resvd: u16,
    pub context: },
    pub frame_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_cmd_priv {
    pub cmd_priv: *mut c_void,
    pub status: u8,
}

extern "C" {
    pub fn scsi_cmd_priv(_arg: cmd) -> return;
}
pub const MAX_MGMT_ADAPTERS: c_int = 1024;
pub const MAX_IOCTL_SGE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_iocpacket {
    pub host_no: u16,
    pub __pad1: u16,
    pub sgl_off: u32,
    pub sge_count: u32,
    pub sense_off: u32,
    pub sense_len: u32,
    pub raw: [u8; 128],
    pub hdr: megasas_header,
    pub frame: },
    pub sgl: [iovec; MAX_IOCTL_SGE],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_aen {
    pub host_no: u16,
    pub __pad1: u16,
    pub seq_num: u32,
    pub class_locale_word: u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_megasas_iocpacket {
    pub host_no: u16,
    pub __pad1: u16,
    pub sgl_off: u32,
    pub sge_count: u32,
    pub sense_off: u32,
    pub sense_len: u32,
    pub raw: [u8; 128],
    pub hdr: megasas_header,
    pub frame: },
    pub sgl: [compat_iovec; MAX_IOCTL_SGE],
// C attribute field omitted

#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_mgmt_info {
    pub count: u16,
    pub instance: [*mut megasas_instance; MAX_MGMT_ADAPTERS],
    pub max_index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MEGASAS_OCR_CAUSE {
    FW_FAULT_OCR			= 0,
    SCSIIO_TIMEOUT_OCR		= 1,
    MFI_IO_TIMEOUT_OCR		= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DCMD_RETURN_STATUS {
    DCMD_SUCCESS    = 0x00,
    DCMD_TIMEOUT    = 0x01,
    DCMD_FAILED     = 0x02,
    DCMD_BUSY       = 0x03,
    DCMD_INIT       = 0xff,
}

extern "C" {
    pub fn MR_TargetIdToLdGet(ldTgtId: u32, map: *mut MR_DRV_RAID_MAP_ALL) -> u16;
}
extern "C" {
    pub fn MR_ArPdGet(ar: u32, arm: u32, map: *mut MR_DRV_RAID_MAP_ALL) -> u16;
}
extern "C" {
    pub fn MR_LdSpanArrayGet(ld: u32, span: u32, map: *mut MR_DRV_RAID_MAP_ALL) -> u16;
}
extern "C" {
    pub fn MR_PdDevHandleGet(pd: u32, map: *mut MR_DRV_RAID_MAP_ALL) -> __le16;
}
extern "C" {
    pub fn MR_GetLDTgtId(ld: u32, map: *mut MR_DRV_RAID_MAP_ALL) -> u16;
}
extern "C" {
    pub fn megasas_get_ctrl_info(instance: *mut megasas_instance) -> c_int;
}
// PD sequence
extern "C" {
    pub fn megasas_get_snapdump_properties(instance: *mut megasas_instance);
}
extern "C" {
    pub fn megasas_free_host_crash_buffer(instance: *mut megasas_instance);
}
extern "C" {
    pub fn megasas_cmd_type(cmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn megasas_setup_jbod_map(instance: *mut megasas_instance);
}
extern "C" {
    pub fn megasas_update_sdev_properties(sdev: *mut scsi_device);
}
extern "C" {
    pub fn megasas_reset_fusion(shost: *mut Scsi_Host, reason: c_int) -> c_int;
}
extern "C" {
    pub fn megasas_task_abort_fusion(scmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn megasas_reset_target_fusion(scmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn mega_mod64(dividend: u64, divisor: u32) -> u32;
}
extern "C" {
    pub fn megasas_alloc_fusion_context(instance: *mut megasas_instance) -> c_int;
}
extern "C" {
    pub fn megasas_free_fusion_context(instance: *mut megasas_instance);
}
extern "C" {
    pub fn megasas_fusion_start_watchdog(instance: *mut megasas_instance) -> c_int;
}
extern "C" {
    pub fn megasas_fusion_stop_watchdog(instance: *mut megasas_instance);
}
extern "C" {
    pub fn megasas_irqpoll(irqpoll: *mut irq_poll, budget: c_int) -> c_int;
}
extern "C" {
    pub fn megasas_dump_fusion_io(scmd: *mut scsi_cmnd);
}
extern "C" {
    pub fn megaraid_sas_kill_hba(instance: *mut megasas_instance);
}
extern "C" {
    pub fn megasas_check_and_restore_queue_depth(instance: *mut megasas_instance);
}
extern "C" {
    pub fn megasas_start_timer(instance: *mut megasas_instance);
}
extern "C" {
    pub fn megasas_alloc_cmds(instance: *mut megasas_instance) -> c_int;
}
extern "C" {
    pub fn megasas_free_cmds(instance: *mut megasas_instance);
}
extern "C" {
    pub fn megasas_init_debugfs();
}
extern "C" {
    pub fn megasas_exit_debugfs();
}
extern "C" {
    pub fn megasas_setup_debugfs(instance: *mut megasas_instance);
}
extern "C" {
    pub fn megasas_destroy_debugfs(instance: *mut megasas_instance);
}
extern "C" {
    pub fn megasas_blk_mq_poll(shost: *mut Scsi_Host, queue_num: c_uint) -> c_int;
}
