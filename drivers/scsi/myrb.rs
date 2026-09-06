//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/myrb.h
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
// Linux Driver for Mylex DAC960/AcceleRAID/eXtremeRAID PCI RAID Controllers
//
// Copyright 2017 Hannes Reinecke, SUSE Linux GmbH <hare@suse.com>
//
// Based on the original DAC960 driver,
// Copyright 1998-2001 by Leonard N. Zubkoff <lnz@dandelion.com>
// Portions Copyright 2002 by Mylex (An IBM Business Unit)
//
pub const MYRB_MAX_LDEVS: c_int = 32;
pub const MYRB_MAX_CHANNELS: c_int = 3;
pub const MYRB_MAX_TARGETS: c_int = 16;
pub const MYRB_MAX_PHYSICAL_DEVICES: c_int = 45;
pub const MYRB_SCATTER_GATHER_LIMIT: c_int = 32;
pub const MYRB_CMD_MBOX_COUNT: c_int = 256;
pub const MYRB_STAT_MBOX_COUNT: c_int = 1024;
pub const MYRB_BLKSIZE_BITS: c_int = 9;
pub const MYRB_MAILBOX_TIMEOUT: c_int = 1000000;
pub const MYRB_DCMD_TAG: c_int = 1;
pub const MYRB_MCMD_TAG: c_int = 2;

//
// DAC960 V1 Firmware Command Opcodes.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum myrb_cmd_opcode {
// I/O Commands
    MYRB_CMD_READ_EXTENDED =	0x33,
    MYRB_CMD_WRITE_EXTENDED =	0x34,
    MYRB_CMD_READAHEAD_EXTENDED =	0x35,
    MYRB_CMD_READ_EXTENDED_SG =	0xB3,
    MYRB_CMD_WRITE_EXTENDED_SG =	0xB4,
    MYRB_CMD_READ =			0x36,
    MYRB_CMD_READ_SG =		0xB6,
    MYRB_CMD_WRITE =		0x37,
    MYRB_CMD_WRITE_SG =		0xB7,
    MYRB_CMD_DCDB =			0x04,
    MYRB_CMD_DCDB_SG =		0x84,
    MYRB_CMD_FLUSH =		0x0A,
// Controller Status Related Commands
    MYRB_CMD_ENQUIRY =		0x53,
    MYRB_CMD_ENQUIRY2 =		0x1C,
    MYRB_CMD_GET_LDRV_ELEMENT =	0x55,
    MYRB_CMD_GET_LDEV_INFO =	0x19,
    MYRB_CMD_IOPORTREAD =		0x39,
    MYRB_CMD_IOPORTWRITE =		0x3A,
    MYRB_CMD_GET_SD_STATS =		0x3E,
    MYRB_CMD_GET_PD_STATS =		0x3F,
    MYRB_CMD_EVENT_LOG_OPERATION =	0x72,
// Device Related Commands
    MYRB_CMD_START_DEVICE =		0x10,
    MYRB_CMD_GET_DEVICE_STATE =	0x50,
    MYRB_CMD_STOP_CHANNEL =		0x13,
    MYRB_CMD_START_CHANNEL =	0x12,
    MYRB_CMD_RESET_CHANNEL =	0x1A,
// Commands Associated with Data Consistency and Errors
    MYRB_CMD_REBUILD =		0x09,
    MYRB_CMD_REBUILD_ASYNC =	0x16,
    MYRB_CMD_CHECK_CONSISTENCY =	0x0F,
    MYRB_CMD_CHECK_CONSISTENCY_ASYNC = 0x1E,
    MYRB_CMD_REBUILD_STAT =		0x0C,
    MYRB_CMD_GET_REBUILD_PROGRESS =	0x27,
    MYRB_CMD_REBUILD_CONTROL =	0x1F,
    MYRB_CMD_READ_BADBLOCK_TABLE =	0x0B,
    MYRB_CMD_READ_BADDATA_TABLE =	0x25,
    MYRB_CMD_CLEAR_BADDATA_TABLE =	0x26,
    MYRB_CMD_GET_ERROR_TABLE =	0x17,
    MYRB_CMD_ADD_CAPACITY_ASYNC =	0x2A,
    MYRB_CMD_BGI_CONTROL =		0x2B,
// Configuration Related Commands
    MYRB_CMD_READ_CONFIG2 =		0x3D,
    MYRB_CMD_WRITE_CONFIG2 =	0x3C,
    MYRB_CMD_READ_CONFIG_ONDISK =	0x4A,
    MYRB_CMD_WRITE_CONFIG_ONDISK =	0x4B,
    MYRB_CMD_READ_CONFIG =		0x4E,
    MYRB_CMD_READ_BACKUP_CONFIG =	0x4D,
    MYRB_CMD_WRITE_CONFIG =		0x4F,
    MYRB_CMD_ADD_CONFIG =		0x4C,
    MYRB_CMD_READ_CONFIG_LABEL =	0x48,
    MYRB_CMD_WRITE_CONFIG_LABEL =	0x49,
// Firmware Upgrade Related Commands
    MYRB_CMD_LOAD_IMAGE =		0x20,
    MYRB_CMD_STORE_IMAGE =		0x21,
    MYRB_CMD_PROGRAM_IMAGE =	0x22,
// Diagnostic Commands
    MYRB_CMD_SET_DIAGNOSTIC_MODE =	0x31,
    MYRB_CMD_RUN_DIAGNOSTIC =	0x32,
// Subsystem Service Commands
    MYRB_CMD_GET_SUBSYS_DATA =	0x70,
    MYRB_CMD_SET_SUBSYS_PARAM =	0x71,
// Version 2.xx Firmware Commands
    MYRB_CMD_ENQUIRY_OLD =		0x05,
    MYRB_CMD_GET_DEVICE_STATE_OLD =	0x14,
    MYRB_CMD_READ_OLD =		0x02,
    MYRB_CMD_WRITE_OLD =		0x03,
    MYRB_CMD_READ_SG_OLD =		0x82,
    MYRB_CMD_WRITE_SG_OLD =		0x83
    } __packed;

//
// DAC960 V1 Firmware Command Status Codes.
//
pub const MYRB_STATUS_SUCCESS: c_uint = 0x0000	/* Common */;
pub const MYRB_STATUS_CHECK_CONDITION: c_uint = 0x0002	/* Common */;
pub const MYRB_STATUS_NO_DEVICE: c_uint = 0x0102	/* Common */;
pub const MYRB_STATUS_INVALID_ADDRESS: c_uint = 0x0105	/* Common */;
pub const MYRB_STATUS_INVALID_PARAM: c_uint = 0x0105	/* Common */;
pub const MYRB_STATUS_IRRECOVERABLE_DATA_ERROR: c_uint = 0x0001	/* I/O */;
pub const MYRB_STATUS_LDRV_NONEXISTENT_OR_OFFLINE: c_uint = 0x0002	/* I/O */;
pub const MYRB_STATUS_ACCESS_BEYOND_END_OF_LDRV: c_uint = 0x0105	/* I/O */;
pub const MYRB_STATUS_BAD_DATA: c_uint = 0x010C	/* I/O */;
pub const MYRB_STATUS_DEVICE_BUSY: c_uint = 0x0008	/* DCDB */;
pub const MYRB_STATUS_DEVICE_NONRESPONSIVE: c_uint = 0x000E	/* DCDB */;
pub const MYRB_STATUS_COMMAND_TERMINATED: c_uint = 0x000F	/* DCDB */;
pub const MYRB_STATUS_START_DEVICE_FAILED: c_uint = 0x0002	/* Device */;
pub const MYRB_STATUS_INVALID_CHANNEL_OR_TARGET: c_uint = 0x0105	/* Device */;
pub const MYRB_STATUS_CHANNEL_BUSY: c_uint = 0x0106	/* Device */;
pub const MYRB_STATUS_OUT_OF_MEMORY: c_uint = 0x0107	/* Device */;
pub const MYRB_STATUS_CHANNEL_NOT_STOPPED: c_uint = 0x0002	/* Device */;
pub const MYRB_STATUS_ATTEMPT_TO_RBLD_ONLINE_DRIVE: c_uint = 0x0002	/* Consistency */;
pub const MYRB_STATUS_RBLD_BADBLOCKS: c_uint = 0x0003	/* Consistency */;
pub const MYRB_STATUS_RBLD_NEW_DISK_FAILED: c_uint = 0x0004	/* Consistency */;
pub const MYRB_STATUS_RBLD_OR_CHECK_INPROGRESS: c_uint = 0x0106	/* Consistency */;
pub const MYRB_STATUS_DEPENDENT_DISK_DEAD: c_uint = 0x0002	/* Consistency */;
pub const MYRB_STATUS_INCONSISTENT_BLOCKS: c_uint = 0x0003	/* Consistency */;
pub const MYRB_STATUS_INVALID_OR_NONREDUNDANT_LDRV: c_uint = 0x0105 /* Consistency */;
pub const MYRB_STATUS_NO_RBLD_OR_CHECK_INPROGRESS: c_uint = 0x0105	/* Consistency */;
pub const MYRB_STATUS_RBLD_IN_PROGRESS_DATA_VALID: c_uint = 0x0000	/* Consistency */;
pub const MYRB_STATUS_RBLD_FAILED_LDEV_FAILURE: c_uint = 0x0002	/* Consistency */;
pub const MYRB_STATUS_RBLD_FAILED_BADBLOCKS: c_uint = 0x0003	/* Consistency */;
pub const MYRB_STATUS_RBLD_FAILED_NEW_DRIVE_FAILED: c_uint = 0x0004	/* Consistency */;
pub const MYRB_STATUS_RBLD_SUCCESS: c_uint = 0x0100	/* Consistency */;
pub const MYRB_STATUS_RBLD_SUCCESS_TERMINATED: c_uint = 0x0107	/* Consistency */;
pub const MYRB_STATUS_RBLD_NOT_CHECKED: c_uint = 0x0108	/* Consistency */;
pub const MYRB_STATUS_BGI_SUCCESS: c_uint = 0x0100	/* Consistency */;
pub const MYRB_STATUS_BGI_ABORTED: c_uint = 0x0005	/* Consistency */;
pub const MYRB_STATUS_NO_BGI_INPROGRESS: c_uint = 0x0105	/* Consistency */;
pub const MYRB_STATUS_ADD_CAPACITY_INPROGRESS: c_uint = 0x0004	/* Consistency */;
pub const MYRB_STATUS_ADD_CAPACITY_FAILED_OR_SUSPENDED: c_uint = 0x00F4 /* Consistency */;
pub const MYRB_STATUS_CONFIG2_CSUM_ERROR: c_uint = 0x0002	/* Configuration */;
pub const MYRB_STATUS_CONFIGURATION_SUSPENDED: c_uint = 0x0106	/* Configuration */;
pub const MYRB_STATUS_FAILED_TO_CONFIGURE_NVRAM: c_uint = 0x0105	/* Configuration */;
pub const MYRB_STATUS_CONFIGURATION_NOT_SAVED: c_uint = 0x0106	/* Configuration */;
pub const MYRB_STATUS_SUBSYS_NOTINSTALLED: c_uint = 0x0001	/* Subsystem */;
pub const MYRB_STATUS_SUBSYS_FAILED: c_uint = 0x0002	/* Subsystem */;
pub const MYRB_STATUS_SUBSYS_BUSY: c_uint = 0x0106	/* Subsystem */;
pub const MYRB_STATUS_SUBSYS_TIMEOUT: c_uint = 0x0108	/* Subsystem */;

//
// DAC960 V1 Firmware Enquiry Command reply structure.
//
    struct myrb_enquiry {
    unsigned char ldev_count;			/* Byte 0 */
    unsigned int rsvd1:24;				/* Bytes 1-3 */
    unsigned int ldev_sizes[32];			/* Bytes 4-131 */
    unsigned short flash_age;			/* Bytes 132-133 */
    struct {
    unsigned char deferred:1;		/* Byte 134 Bit 0 */
    unsigned char low_bat:1;		/* Byte 134 Bit 1 */
    unsigned char rsvd2:6;			/* Byte 134 Bits 2-7 */
    } status;
    unsigned char rsvd3:8;				/* Byte 135 */
    unsigned char fw_minor_version;			/* Byte 136 */
    unsigned char fw_major_version;			/* Byte 137 */
    enum {
    MYRB_NO_STDBY_RBLD_OR_CHECK_IN_PROGRESS =	0x00,
    MYRB_STDBY_RBLD_IN_PROGRESS =			0x01,
    MYRB_BG_RBLD_IN_PROGRESS =			0x02,
    MYRB_BG_CHECK_IN_PROGRESS =			0x03,
    MYRB_STDBY_RBLD_COMPLETED_WITH_ERROR =		0xFF,
    MYRB_BG_RBLD_OR_CHECK_FAILED_DRIVE_FAILED =	0xF0,
    MYRB_BG_RBLD_OR_CHECK_FAILED_LDEV_FAILED =	0xF1,
    MYRB_BG_RBLD_OR_CHECK_FAILED_OTHER =		0xF2,
    MYRB_BG_RBLD_OR_CHECK_SUCCESS_TERMINATED =	0xF3
    } __packed rbld;		/* Byte 138 */
    unsigned char max_tcq;				/* Byte 139 */
    unsigned char ldev_offline;			/* Byte 140 */
    unsigned char rsvd4:8;				/* Byte 141 */
    unsigned short ev_seq;				/* Bytes 142-143 */
    unsigned char ldev_critical;			/* Byte 144 */
    unsigned int rsvd5:24;				/* Bytes 145-147 */
    unsigned char pdev_dead;			/* Byte 148 */
    unsigned char rsvd6:8;				/* Byte 149 */
    unsigned char rbld_count;			/* Byte 150 */
    struct {
    unsigned char rsvd7:3;			/* Byte 151 Bits 0-2 */
    unsigned char bbu_present:1;		/* Byte 151 Bit 3 */
    unsigned char rsvd8:4;			/* Byte 151 Bits 4-7 */
    } misc;
    struct {
    unsigned char target;
    unsigned char channel;
    } dead_drives[21];				/* Bytes 152-194 */
    unsigned char rsvd9[62];			/* Bytes 195-255 */
    } __packed;

//
// DAC960 V1 Firmware Enquiry2 Command reply structure.
//
    struct myrb_enquiry2 {
    struct {
    enum {
    DAC960_V1_P_PD_PU =			0x01,
    DAC960_V1_PL =				0x02,
    DAC960_V1_PG =				0x10,
    DAC960_V1_PJ =				0x11,
    DAC960_V1_PR =				0x12,
    DAC960_V1_PT =				0x13,
    DAC960_V1_PTL0 =			0x14,
    DAC960_V1_PRL =				0x15,
    DAC960_V1_PTL1 =			0x16,
    DAC960_V1_1164P =			0x20
    } __packed sub_model;		/* Byte 0 */
    unsigned char actual_channels;			/* Byte 1 */
    enum {
    MYRB_5_CHANNEL_BOARD =		0x01,
    MYRB_3_CHANNEL_BOARD =		0x02,
    MYRB_2_CHANNEL_BOARD =		0x03,
    MYRB_3_CHANNEL_ASIC_DAC =	0x04
    } __packed model;		/* Byte 2 */
    enum {
    MYRB_EISA_CONTROLLER =		0x01,
    MYRB_MCA_CONTROLLER =		0x02,
    MYRB_PCI_CONTROLLER =		0x03,
    MYRB_SCSI_TO_SCSI =		0x08
    } __packed controller;	/* Byte 3 */
    } hw;						/* Bytes 0-3 */
// MajorVersion.MinorVersion-FirmwareType-TurnID
    struct {
    unsigned char major_version;		/* Byte 4 */
    unsigned char minor_version;		/* Byte 5 */
    unsigned char turn_id;			/* Byte 6 */
    char firmware_type;			/* Byte 7 */
    } fw;						/* Bytes 4-7 */
    unsigned int rsvd1;				/* Byte 8-11 */
    unsigned char cfg_chan;				/* Byte 12 */
    unsigned char cur_chan;				/* Byte 13 */
    unsigned char max_targets;			/* Byte 14 */
    unsigned char max_tcq;				/* Byte 15 */
    unsigned char max_ldev;				/* Byte 16 */
    unsigned char max_arms;				/* Byte 17 */
    unsigned char max_spans;			/* Byte 18 */
    unsigned char rsvd2;				/* Byte 19 */
    unsigned int rsvd3;				/* Bytes 20-23 */
    unsigned int mem_size;				/* Bytes 24-27 */
    unsigned int cache_size;			/* Bytes 28-31 */
    unsigned int flash_size;			/* Bytes 32-35 */
    unsigned int nvram_size;			/* Bytes 36-39 */
    struct {
    enum {
    MYRB_RAM_TYPE_DRAM =		0x0,
    MYRB_RAM_TYPE_EDO =			0x1,
    MYRB_RAM_TYPE_SDRAM =		0x2,
    MYRB_RAM_TYPE_Last =		0x7
    } __packed ram:3;	/* Byte 40 Bits 0-2 */
    enum {
    MYRB_ERR_CORR_None =	0x0,
    MYRB_ERR_CORR_Parity =	0x1,
    MYRB_ERR_CORR_ECC =		0x2,
    MYRB_ERR_CORR_Last =	0x7
    } __packed ec:3;	/* Byte 40 Bits 3-5 */
    unsigned char fast_page:1;		/* Byte 40 Bit 6 */
    unsigned char low_power:1;		/* Byte 40 Bit 7 */
    unsigned char rsvd4;			/* Bytes 41 */
    } mem_type;
    unsigned short clock_speed;			/* Bytes 42-43 */
    unsigned short mem_speed;			/* Bytes 44-45 */
    unsigned short hw_speed;			/* Bytes 46-47 */
    unsigned char rsvd5[12];			/* Bytes 48-59 */
    unsigned short max_cmds;			/* Bytes 60-61 */
    unsigned short max_sge;				/* Bytes 62-63 */
    unsigned short max_drv_cmds;			/* Bytes 64-65 */
    unsigned short max_io_desc;			/* Bytes 66-67 */
    unsigned short max_sectors;			/* Bytes 68-69 */
    unsigned char latency;				/* Byte 70 */
    unsigned char rsvd6;				/* Byte 71 */
    unsigned char scsi_tmo;				/* Byte 72 */
    unsigned char rsvd7;				/* Byte 73 */
    unsigned short min_freelines;			/* Bytes 74-75 */
    unsigned char rsvd8[8];				/* Bytes 76-83 */
    unsigned char rbld_rate_const;			/* Byte 84 */
    unsigned char rsvd9[11];			/* Byte 85-95 */
    unsigned short pdrv_block_size;			/* Bytes 96-97 */
    unsigned short ldev_block_size;			/* Bytes 98-99 */
    unsigned short max_blocks_per_cmd;		/* Bytes 100-101 */
    unsigned short block_factor;			/* Bytes 102-103 */
    unsigned short cacheline_size;			/* Bytes 104-105 */
    struct {
    enum {
    MYRB_WIDTH_NARROW_8BIT =		0x0,
    MYRB_WIDTH_WIDE_16BIT =			0x1,
    MYRB_WIDTH_WIDE_32BIT =			0x2
    } __packed bus_width:2;	/* Byte 106 Bits 0-1 */
    enum {
    MYRB_SCSI_SPEED_FAST =			0x0,
    MYRB_SCSI_SPEED_ULTRA =			0x1,
    MYRB_SCSI_SPEED_ULTRA2 =		0x2
    } __packed bus_speed:2;	/* Byte 106 Bits 2-3 */
    unsigned char differential:1;		/* Byte 106 Bit 4 */
    unsigned char rsvd10:3;			/* Byte 106 Bits 5-7 */
    } scsi_cap;
    unsigned char rsvd11[5];			/* Byte 107-111 */
    unsigned short fw_build;			/* Bytes 112-113 */
    enum {
    MYRB_FAULT_AEMI =				0x01,
    MYRB_FAULT_OEM1 =				0x02,
    MYRB_FAULT_OEM2 =				0x04,
    MYRB_FAULT_OEM3 =				0x08,
    MYRB_FAULT_CONNER =				0x10,
    MYRB_FAULT_SAFTE =				0x20
    } __packed fault_mgmt;		/* Byte 114 */
    unsigned char rsvd12;				/* Byte 115 */
    struct {
    unsigned int clustering:1;		/* Byte 116 Bit 0 */
    unsigned int online_RAID_expansion:1;	/* Byte 116 Bit 1 */
    unsigned int readahead:1;		/* Byte 116 Bit 2 */
    unsigned int bgi:1;			/* Byte 116 Bit 3 */
    unsigned int rsvd13:28;			/* Bytes 116-119 */
    } fw_features;
    unsigned char rsvd14[8];			/* Bytes 120-127 */
    } __packed;

//
// DAC960 V1 Firmware Logical Drive State type.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum myrb_devstate {
    MYRB_DEVICE_DEAD =		0x00,
    MYRB_DEVICE_WO =		0x02,
    MYRB_DEVICE_ONLINE =		0x03,
    MYRB_DEVICE_CRITICAL =		0x04,
    MYRB_DEVICE_STANDBY =		0x10,
    MYRB_DEVICE_OFFLINE =		0xFF
    } __packed;

//
// DAC960 V1 RAID Levels
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum myrb_raidlevel {
    MYRB_RAID_LEVEL0 =		0x0,     /* RAID 0 */
    MYRB_RAID_LEVEL1 =		0x1,     /* RAID 1 */
    MYRB_RAID_LEVEL3 =		0x3,     /* RAID 3 */
    MYRB_RAID_LEVEL5 =		0x5,     /* RAID 5 */
    MYRB_RAID_LEVEL6 =		0x6,     /* RAID 6 */
    MYRB_RAID_JBOD =		0x7,     /* RAID 7 (JBOD) */
    } __packed;

//
// DAC960 V1 Firmware Logical Drive Information structure.
//
    struct myrb_ldev_info {
    unsigned int size;				/* Bytes 0-3 */
    enum myrb_devstate state;			/* Byte 4 */
    unsigned int raid_level:7;			/* Byte 5 Bits 0-6 */
    unsigned int wb_enabled:1;			/* Byte 5 Bit 7 */
    unsigned int rsvd:16;				/* Bytes 6-7 */
}

//
// DAC960 V1 Firmware Perform Event Log Operation Types.
//
pub const DAC960_V1_GetEventLogEntry: c_uint = 0x00;
//
// DAC960 V1 Firmware Get Event Log Entry Command reply structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct myrb_log_entry {
    pub /: *mut *mut unsigned char msg_type; / Byte 0,
    pub /: *mut *mut unsigned char msg_len; / Byte 1,
    pub /: *mut *mut unsigned char target:5; / Byte 2 Bits 0-4,
    pub /: *mut *mut unsigned char channel:3; / Byte 2 Bits 5-7,
    pub /: *mut *mut unsigned char lun:6; / Byte 3 Bits 0-5,
    pub /: *mut *mut unsigned char rsvd1:2; / Byte 3 Bits 6-7,
    pub /: *mut *mut unsigned short seq_num; / Bytes 4-5,
    pub /: *mut *mut unsigned char sense[26]; / Bytes 6-31,
}

//
// DAC960 V1 Firmware Get Device State Command reply structure.
// The structure is padded by 2 bytes for compatibility with Version 2.xx
// Firmware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct myrb_pdev_state {
    pub /: *mut *mut unsigned int present:1; / Byte 0 Bit 0,
    pub /: *mut *mut unsigned int :7; / Byte 0 Bits 1-7,
    pub /: *mut *mut } __packed devtype:2; / Byte 1 Bits 0-1,
    pub /: *mut *mut unsigned int rsvd1:1; / Byte 1 Bit 2,
    pub /: *mut *mut unsigned int fast20:1; / Byte 1 Bit 3,
    pub /: *mut *mut unsigned int sync:1; / Byte 1 Bit 4,
    pub /: *mut *mut unsigned int fast:1; / Byte 1 Bit 5,
    pub /: *mut *mut unsigned int wide:1; / Byte 1 Bit 6,
    pub /: *mut *mut unsigned int tcq_supported:1; / Byte 1 Bit 7,
    pub /: *mut *mut myrb_devstate state; / Byte 2,
    pub /: *mut *mut unsigned int rsvd2:8; / Byte 3,
    pub /: *mut *mut unsigned int sync_multiplier; / Byte 4,
    pub /: *mut *mut unsigned int sync_offset:5; / Byte 5 Bits 0-4,
    pub /: *mut *mut unsigned int rsvd3:3; / Byte 5 Bits 5-7,
    pub /: *mut *mut unsigned int size; / Bytes 6-9,
    pub /: *mut *mut unsigned int rsvd4:16; / Bytes 10-11,
    pub __packed: },
//
// DAC960 V1 Firmware Get Rebuild Progress Command reply structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct myrb_rbld_progress {
    pub /: *mut *mut unsigned int ldev_num; / Bytes 0-3,
    pub /: *mut *mut unsigned int ldev_size; / Bytes 4-7,
    pub /: *mut *mut unsigned int blocks_left; / Bytes 8-11,
}

//
// DAC960 V1 Firmware Background Initialization Status Command reply structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct myrb_bgi_status {
    pub /: *mut *mut unsigned int ldev_size; / Bytes 0-3,
    pub /: *mut *mut unsigned int blocks_done; / Bytes 4-7,
    pub /: *mut *mut unsigned char rsvd1[12]; / Bytes 8-19,
    pub /: *mut *mut unsigned int ldev_num; / Bytes 20-23,
    pub /: *mut *mut unsigned char raid_level; / Byte 24,
    pub /: *mut *mut } __packed status; / Byte 25,
    pub /: *mut *mut unsigned char rsvd2[6]; / Bytes 26-31,
}

//
// DAC960 V1 Firmware Error Table Entry structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct myrb_error_entry {
    pub /: *mut *mut unsigned char parity_err; / Byte 0,
    pub /: *mut *mut unsigned char soft_err; / Byte 1,
    pub /: *mut *mut unsigned char hard_err; / Byte 2,
    pub /: *mut *mut unsigned char misc_err; / Byte 3,
}

//
// DAC960 V1 Firmware Read Config2 Command reply structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct myrb_config2 {
    pub /: *mut *mut unsigned rsvd1:1; / Byte 0 Bit 0,
    pub /: *mut *mut unsigned active_negation:1; / Byte 0 Bit 1,
    pub /: *mut *mut unsigned rsvd2:5; / Byte 0 Bits 2-6,
    pub /: *mut *mut unsigned no_rescan_on_reset_during_scan:1; / Byte 0 Bit 7,
    pub /: *mut *mut unsigned StorageWorks_support:1; / Byte 1 Bit 0,
    pub /: *mut *mut unsigned HewlettPackard_support:1; / Byte 1 Bit 1,
    pub /: *mut *mut unsigned no_disconnect_on_first_command:1; / Byte 1 Bit 2,
    pub /: *mut *mut unsigned rsvd3:2; / Byte 1 Bits 3-4,
    pub /: *mut *mut unsigned AEMI_ARM:1; / Byte 1 Bit 5,
    pub /: *mut *mut unsigned AEMI_OFM:1; / Byte 1 Bit 6,
    pub /: *mut *mut unsigned rsvd4:1; / Byte 1 Bit 7,
    pub /: *mut *mut } __packed OEMID; / Byte 2,
    pub /: *mut *mut unsigned char oem_model_number; / Byte 3,
    pub /: *mut *mut unsigned char physical_sector; / Byte 4,
    pub /: *mut *mut unsigned char logical_sector; / Byte 5,
    pub /: *mut *mut unsigned char block_factor; / Byte 6,
    pub /: *mut *mut unsigned readahead_enabled:1; / Byte 7 Bit 0,
    pub /: *mut *mut unsigned low_BIOS_delay:1; / Byte 7 Bit 1,
    pub /: *mut *mut unsigned rsvd5:2; / Byte 7 Bits 2-3,
    pub /: *mut *mut unsigned restrict_reassign_to_one_sector:1; / Byte 7 Bit 4,
    pub /: *mut *mut unsigned rsvd6:1; / Byte 7 Bit 5,
    pub /: *mut *mut unsigned FUA_during_write_recovery:1; / Byte 7 Bit 6,
    pub /: *mut *mut unsigned enable_LeftSymmetricRAID5Algorithm:1; / Byte 7 Bit 7,
    pub /: *mut *mut unsigned char default_rebuild_rate; / Byte 8,
    pub /: *mut *mut unsigned char rsvd7; / Byte 9,
    pub /: *mut *mut unsigned char blocks_per_cacheline; / Byte 10,
    pub /: *mut *mut unsigned char blocks_per_stripe; / Byte 11,
    pub /: *mut *mut } __packed speed:2; / Byte 11 Bits 0-1,
    pub /: *mut *mut unsigned force_8bit:1; / Byte 11 Bit 2,
    pub /: *mut *mut unsigned disable_fast20:1; / Byte 11 Bit 3,
    pub /: *mut *mut unsigned rsvd8:3; / Byte 11 Bits 4-6,
    pub /: *mut *mut unsigned enable_tcq:1; / Byte 11 Bit 7,
    pub /: *mut *mut } __packed channelparam[6]; / Bytes 12-17,
    pub /: *mut *mut unsigned char SCSIInitiatorID; / Byte 18,
    pub /: *mut *mut unsigned char rsvd9; / Byte 19,
    pub /: *mut *mut } __packed startup; / Byte 20,
    pub /: *mut *mut unsigned char simultaneous_device_spinup_count; / Byte 21,
    pub /: *mut *mut unsigned char seconds_delay_between_spinups; / Byte 22,
    pub /: *mut *mut unsigned char rsvd10[29]; / Bytes 23-51,
    pub /: *mut *mut unsigned BIOS_disabled:1; / Byte 52 Bit 0,
    pub /: *mut *mut unsigned CDROM_boot_enabled:1; / Byte 52 Bit 1,
    pub /: *mut *mut unsigned rsvd11:3; / Byte 52 Bits 2-4,
    pub /: *mut *mut } __packed drive_geometry:2; / Byte 52 Bits 5-6,
    pub /: *mut *mut unsigned rsvd12:1; / Byte 52 Bit 7,
    pub /: *mut *mut unsigned char rsvd13[9]; / Bytes 53-61,
    pub /: *mut *mut unsigned short csum; / Bytes 62-63,
}

//
// DAC960 V1 Firmware DCDB request structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct myrb_dcdb {
    pub /: *mut *mut unsigned target:4; / Byte 0 Bits 0-3,
    pub /: *mut *mut unsigned channel:4; / Byte 0 Bits 4-7,
    pub /: *mut *mut } __packed data_xfer:2; / Byte 1 Bits 0-1,
    pub /: *mut *mut unsigned early_status:1; / Byte 1 Bit 2,
    pub /: *mut *mut unsigned rsvd1:1; / Byte 1 Bit 3,
    pub /: *mut *mut } __packed timeout:2; / Byte 1 Bits 4-5,
    pub /: *mut *mut unsigned no_autosense:1; / Byte 1 Bit 6,
    pub /: *mut *mut unsigned allow_disconnect:1; / Byte 1 Bit 7,
    pub /: *mut *mut unsigned short xfer_len_lo; / Bytes 2-3,
    pub /: *mut *mut u32 dma_addr; / Bytes 4-7,
    pub /: *mut *mut unsigned char cdb_len:4; / Byte 8 Bits 0-3,
    pub /: *mut *mut unsigned char xfer_len_hi4:4; / Byte 8 Bits 4-7,
    pub /: *mut *mut unsigned char sense_len; / Byte 9,
    pub /: *mut *mut unsigned char cdb[12]; / Bytes 10-21,
    pub /: *mut *mut unsigned char sense[64]; / Bytes 22-85,
    pub /: *mut *mut unsigned char status; / Byte 86,
    pub /: *mut *mut unsigned char rsvd2; / Byte 87,
}

//
// DAC960 V1 Firmware Scatter/Gather List Type 1 32 Bit Address
// 32 Bit Byte Count structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct myrb_sge {
    pub /: *mut *mut u32 sge_addr; / Bytes 0-3,
    pub /: *mut *mut u32 sge_count; / Bytes 4-7,
}

//
// 13 Byte DAC960 V1 Firmware Command Mailbox structure.
// Bytes 13-15 are not used.  The structure is padded to 16 bytes for
// efficient access.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union myrb_cmd_mbox {
    pub /: *mut *mut unsigned int words[4]; / Words 0-3,
    pub /: *mut *mut unsigned char bytes[16]; / Bytes 0-15,
    pub /: *mut *mut myrb_cmd_opcode opcode; / Byte 0,
    pub /: *mut *mut unsigned char id; / Byte 1,
    pub /: *mut *mut unsigned char rsvd[14]; / Bytes 2-15,
    pub common: } __packed,
    pub /: *mut *mut myrb_cmd_opcode opcode; / Byte 0,
    pub /: *mut *mut unsigned char id; / Byte 1,
    pub /: *mut *mut unsigned char rsvd1[6]; / Bytes 2-7,
    pub /: *mut *mut u32 addr; / Bytes 8-11,
    pub /: *mut *mut unsigned char rsvd2[4]; / Bytes 12-15,
    pub type3: } __packed,
    pub /: *mut *mut myrb_cmd_opcode opcode; / Byte 0,
    pub /: *mut *mut unsigned char id; / Byte 1,
    pub /: *mut *mut unsigned char optype; / Byte 2,
    pub /: *mut *mut unsigned char rsvd1[5]; / Bytes 3-7,
    pub /: *mut *mut u32 addr; / Bytes 8-11,
    pub /: *mut *mut unsigned char rsvd2[4]; / Bytes 12-15,
    pub type3B: } __packed,
    pub /: *mut *mut myrb_cmd_opcode opcode; / Byte 0,
    pub /: *mut *mut unsigned char id; / Byte 1,
    pub /: *mut *mut unsigned char rsvd1[5]; / Bytes 2-6,
    pub /: *mut *mut unsigned char ldev_num:6; / Byte 7 Bits 0-6,
    pub /: *mut *mut unsigned char auto_restore:1; / Byte 7 Bit 7,
    pub /: *mut *mut unsigned char rsvd2[8]; / Bytes 8-15,
    pub type3C: } __packed,
    pub /: *mut *mut myrb_cmd_opcode opcode; / Byte 0,
    pub /: *mut *mut unsigned char id; / Byte 1,
    pub /: *mut *mut unsigned char channel; / Byte 2,
    pub /: *mut *mut unsigned char target; / Byte 3,
    pub /: *mut *mut myrb_devstate state; / Byte 4,
    pub /: *mut *mut unsigned char rsvd1[3]; / Bytes 5-7,
    pub /: *mut *mut u32 addr; / Bytes 8-11,
    pub /: *mut *mut unsigned char rsvd2[4]; / Bytes 12-15,
    pub type3D: } __packed,
    pub /: *mut *mut myrb_cmd_opcode opcode; / Byte 0,
    pub /: *mut *mut unsigned char id; / Byte 1,
    pub /: *mut *mut unsigned char optype; / Byte 2,
    pub /: *mut *mut unsigned char opqual; / Byte 3,
    pub /: *mut *mut unsigned short ev_seq; / Bytes 4-5,
    pub /: *mut *mut unsigned char rsvd1[2]; / Bytes 6-7,
    pub /: *mut *mut u32 addr; / Bytes 8-11,
    pub /: *mut *mut unsigned char rsvd2[4]; / Bytes 12-15,
    pub type3E: } __packed,
    pub /: *mut *mut myrb_cmd_opcode opcode; / Byte 0,
    pub /: *mut *mut unsigned char id; / Byte 1,
    pub /: *mut *mut unsigned char rsvd1[2]; / Bytes 2-3,
    pub /: *mut *mut unsigned char rbld_rate; / Byte 4,
    pub /: *mut *mut unsigned char rsvd2[3]; / Bytes 5-7,
    pub /: *mut *mut u32 addr; / Bytes 8-11,
    pub /: *mut *mut unsigned char rsvd3[4]; / Bytes 12-15,
    pub type3R: } __packed,
    pub /: *mut *mut myrb_cmd_opcode opcode; / Byte 0,
    pub /: *mut *mut unsigned char id; / Byte 1,
    pub /: *mut *mut unsigned short xfer_len; / Bytes 2-3,
    pub /: *mut *mut unsigned int lba; / Bytes 4-7,
    pub /: *mut *mut u32 addr; / Bytes 8-11,
    pub /: *mut *mut unsigned char ldev_num; / Byte 12,
    pub /: *mut *mut unsigned char rsvd[3]; / Bytes 13-15,
    pub type4: } __packed,
    pub /: *mut *mut myrb_cmd_opcode opcode; / Byte 0,
    pub /: *mut *mut unsigned char id; / Byte 1,
    pub /: *mut *mut unsigned short xfer_len:11; / Bytes 2-3,
    pub /: *mut *mut unsigned char ldev_num:5; / Byte 3 Bits 3-7,
    pub ld: } __packed,
    pub /: *mut *mut unsigned int lba; / Bytes 4-7,
    pub /: *mut *mut u32 addr; / Bytes 8-11,
    pub /: *mut *mut unsigned char sg_count:6; / Byte 12 Bits 0-5,
    pub /: *mut *mut } __packed sg_type:2; / Byte 12 Bits 6-7,
    pub /: *mut *mut unsigned char rsvd[3]; / Bytes 13-15,
    pub type5: } __packed,
    pub /: *mut *mut myrb_cmd_opcode opcode; / Byte 0,
    pub /: *mut *mut unsigned char id; / Byte 1,
    pub /: *mut *mut unsigned char opcode2; / Byte 2,
    pub /: *mut *mut unsigned char rsvd1:8; / Byte 3,
    pub /: *mut *mut u32 cmd_mbox_addr; / Bytes 4-7,
    pub /: *mut *mut u32 stat_mbox_addr; / Bytes 8-11,
    pub /: *mut *mut unsigned char rsvd2[4]; / Bytes 12-15,
    pub typeX: } __packed,
}

//
// DAC960 V1 Firmware Controller Status Mailbox structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct myrb_stat_mbox {
    pub /: *mut *mut unsigned char id; / Byte 0,
    pub /: *mut *mut unsigned char rsvd:7; / Byte 1 Bits 0-6,
    pub /: *mut *mut unsigned char valid:1; / Byte 1 Bit 7,
    pub /: *mut *mut unsigned short status; / Bytes 2-3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct myrb_cmdblk {
    pub mbox: myrb_cmd_mbox,
    pub status: c_ushort,
    pub completion: *mut completion,
    pub dcdb: *mut myrb_dcdb,
    pub dcdb_addr: dma_addr_t,
    pub sgl: *mut myrb_sge,
    pub sgl_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct myrb_hba {
    pub ldev_block_size: c_uint,
    pub ldev_geom_heads: c_uchar,
    pub ldev_geom_sectors: c_uchar,
    pub bus_width: c_uchar,
    pub stripe_size: c_ushort,
    pub segment_size: c_ushort,
    pub new_ev_seq: c_ushort,
    pub old_ev_seq: c_ushort,
    pub dual_mode_interface: bool,
    pub bgi_status_supported: bool,
    pub safte_enabled: bool,
    pub need_ldev_info: bool,
    pub need_err_info: bool,
    pub need_rbld: bool,
    pub need_cc_status: bool,
    pub need_bgi_status: bool,
    pub rbld_first: bool,
    pub pdev: *mut pci_dev,
    pub host: *mut Scsi_Host,
    pub work_q: *mut workqueue_struct,
    pub monitor_work: delayed_work,
    pub primary_monitor_time: c_ulong,
    pub secondary_monitor_time: c_ulong,
    pub sg_pool: *mut dma_pool,
    pub dcdb_pool: *mut dma_pool,
    pub queue_lock: spinlock_t,
    pub cmd_blk): *mut *mut *mut void (qcmd)(struct myrb_hba cs, struct myrb_cmdblk,
    pub cmd_mbox): *mut myrb_cmd_mbox,
    pub base): *mut *mut void (get_cmd_mbox)(void __iomem,
    pub base): *mut *mut void (disable_intr)(void __iomem,
    pub base): *mut *mut void (reset)(void __iomem,
    pub ctlr_num: c_uint,
    pub model_name: [c_uchar; 20],
    pub fw_version: [c_uchar; 12],
    pub irq: c_uint,
    pub io_addr: phys_addr_t,
    pub pci_addr: phys_addr_t,
    pub io_base: *mut void __iomem,
    pub mmio_base: *mut void __iomem,
    pub cmd_mbox_size: usize,
    pub cmd_mbox_addr: dma_addr_t,
    pub first_cmd_mbox: *mut myrb_cmd_mbox,
    pub last_cmd_mbox: *mut myrb_cmd_mbox,
    pub next_cmd_mbox: *mut myrb_cmd_mbox,
    pub prev_cmd_mbox1: *mut myrb_cmd_mbox,
    pub prev_cmd_mbox2: *mut myrb_cmd_mbox,
    pub stat_mbox_size: usize,
    pub stat_mbox_addr: dma_addr_t,
    pub first_stat_mbox: *mut myrb_stat_mbox,
    pub last_stat_mbox: *mut myrb_stat_mbox,
    pub next_stat_mbox: *mut myrb_stat_mbox,
    pub dcmd_blk: myrb_cmdblk,
    pub mcmd_blk: myrb_cmdblk,
    pub dcmd_mutex: mutex,
    pub enquiry: *mut myrb_enquiry,
    pub enquiry_addr: dma_addr_t,
    pub err_table: *mut myrb_error_entry,
    pub err_table_addr: dma_addr_t,
    pub last_rbld_status: c_ushort,
    pub ldev_info_buf: *mut myrb_ldev_info,
    pub ldev_info_addr: dma_addr_t,
    pub bgi_status: myrb_bgi_status,
    pub dma_mutex: mutex,
}

//
// DAC960 LA Series Controller Interface Register Offsets.
//
pub const DAC960_LA_mmio_size: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DAC960_LA_reg_offset {
    DAC960_LA_IRQMASK_OFFSET	= 0x34,
    DAC960_LA_CMDOP_OFFSET		= 0x50,
    DAC960_LA_CMDID_OFFSET		= 0x51,
    DAC960_LA_MBOX2_OFFSET		= 0x52,
    DAC960_LA_MBOX3_OFFSET		= 0x53,
    DAC960_LA_MBOX4_OFFSET		= 0x54,
    DAC960_LA_MBOX5_OFFSET		= 0x55,
    DAC960_LA_MBOX6_OFFSET		= 0x56,
    DAC960_LA_MBOX7_OFFSET		= 0x57,
    DAC960_LA_MBOX8_OFFSET		= 0x58,
    DAC960_LA_MBOX9_OFFSET		= 0x59,
    DAC960_LA_MBOX10_OFFSET		= 0x5A,
    DAC960_LA_MBOX11_OFFSET		= 0x5B,
    DAC960_LA_MBOX12_OFFSET		= 0x5C,
    DAC960_LA_STSID_OFFSET		= 0x5D,
    DAC960_LA_STS_OFFSET		= 0x5E,
    DAC960_LA_IDB_OFFSET		= 0x60,
    DAC960_LA_ODB_OFFSET		= 0x61,
    DAC960_LA_ERRSTS_OFFSET		= 0x63,
}

//
// DAC960 LA Series Inbound Door Bell Register.
//
pub const DAC960_LA_IDB_HWMBOX_NEW_CMD: c_uint = 0x01;
pub const DAC960_LA_IDB_HWMBOX_ACK_STS: c_uint = 0x02;
pub const DAC960_LA_IDB_GEN_IRQ: c_uint = 0x04;
pub const DAC960_LA_IDB_CTRL_RESET: c_uint = 0x08;
pub const DAC960_LA_IDB_MMBOX_NEW_CMD: c_uint = 0x10;
pub const DAC960_LA_IDB_HWMBOX_EMPTY: c_uint = 0x01;
pub const DAC960_LA_IDB_INIT_DONE: c_uint = 0x02;
//
// DAC960 LA Series Outbound Door Bell Register.
//
pub const DAC960_LA_ODB_HWMBOX_ACK_IRQ: c_uint = 0x01;
pub const DAC960_LA_ODB_MMBOX_ACK_IRQ: c_uint = 0x02;
pub const DAC960_LA_ODB_HWMBOX_STS_AVAIL: c_uint = 0x01;
pub const DAC960_LA_ODB_MMBOX_STS_AVAIL: c_uint = 0x02;
//
// DAC960 LA Series Interrupt Mask Register.
//
pub const DAC960_LA_IRQMASK_DISABLE_IRQ: c_uint = 0x04;
//
// DAC960 LA Series Error Status Register.
//
pub const DAC960_LA_ERRSTS_PENDING: c_uint = 0x02;
//
// DAC960 PG Series Controller Interface Register Offsets.
//
pub const DAC960_PG_mmio_size: c_uint = 0x2000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DAC960_PG_reg_offset {
    DAC960_PG_IDB_OFFSET		= 0x0020,
    DAC960_PG_ODB_OFFSET		= 0x002C,
    DAC960_PG_IRQMASK_OFFSET	= 0x0034,
    DAC960_PG_CMDOP_OFFSET		= 0x1000,
    DAC960_PG_CMDID_OFFSET		= 0x1001,
    DAC960_PG_MBOX2_OFFSET		= 0x1002,
    DAC960_PG_MBOX3_OFFSET		= 0x1003,
    DAC960_PG_MBOX4_OFFSET		= 0x1004,
    DAC960_PG_MBOX5_OFFSET		= 0x1005,
    DAC960_PG_MBOX6_OFFSET		= 0x1006,
    DAC960_PG_MBOX7_OFFSET		= 0x1007,
    DAC960_PG_MBOX8_OFFSET		= 0x1008,
    DAC960_PG_MBOX9_OFFSET		= 0x1009,
    DAC960_PG_MBOX10_OFFSET		= 0x100A,
    DAC960_PG_MBOX11_OFFSET		= 0x100B,
    DAC960_PG_MBOX12_OFFSET		= 0x100C,
    DAC960_PG_STSID_OFFSET		= 0x1018,
    DAC960_PG_STS_OFFSET		= 0x101A,
    DAC960_PG_ERRSTS_OFFSET		= 0x103F,
}

//
// DAC960 PG Series Inbound Door Bell Register.
//
pub const DAC960_PG_IDB_HWMBOX_NEW_CMD: c_uint = 0x01;
pub const DAC960_PG_IDB_HWMBOX_ACK_STS: c_uint = 0x02;
pub const DAC960_PG_IDB_GEN_IRQ: c_uint = 0x04;
pub const DAC960_PG_IDB_CTRL_RESET: c_uint = 0x08;
pub const DAC960_PG_IDB_MMBOX_NEW_CMD: c_uint = 0x10;
pub const DAC960_PG_IDB_HWMBOX_FULL: c_uint = 0x01;
pub const DAC960_PG_IDB_INIT_IN_PROGRESS: c_uint = 0x02;
//
// DAC960 PG Series Outbound Door Bell Register.
//
pub const DAC960_PG_ODB_HWMBOX_ACK_IRQ: c_uint = 0x01;
pub const DAC960_PG_ODB_MMBOX_ACK_IRQ: c_uint = 0x02;
pub const DAC960_PG_ODB_HWMBOX_STS_AVAIL: c_uint = 0x01;
pub const DAC960_PG_ODB_MMBOX_STS_AVAIL: c_uint = 0x02;
//
// DAC960 PG Series Interrupt Mask Register.
//
pub const DAC960_PG_IRQMASK_MSI_MASK1: c_uint = 0x03;
pub const DAC960_PG_IRQMASK_DISABLE_IRQ: c_uint = 0x04;
pub const DAC960_PG_IRQMASK_MSI_MASK2: c_uint = 0xF8;
//
// DAC960 PG Series Error Status Register.
//
pub const DAC960_PG_ERRSTS_PENDING: c_uint = 0x04;
//
// DAC960 PD Series Controller Interface Register Offsets.
//
pub const DAC960_PD_mmio_size: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DAC960_PD_reg_offset {
    DAC960_PD_CMDOP_OFFSET		= 0x00,
    DAC960_PD_CMDID_OFFSET		= 0x01,
    DAC960_PD_MBOX2_OFFSET		= 0x02,
    DAC960_PD_MBOX3_OFFSET		= 0x03,
    DAC960_PD_MBOX4_OFFSET		= 0x04,
    DAC960_PD_MBOX5_OFFSET		= 0x05,
    DAC960_PD_MBOX6_OFFSET		= 0x06,
    DAC960_PD_MBOX7_OFFSET		= 0x07,
    DAC960_PD_MBOX8_OFFSET		= 0x08,
    DAC960_PD_MBOX9_OFFSET		= 0x09,
    DAC960_PD_MBOX10_OFFSET		= 0x0A,
    DAC960_PD_MBOX11_OFFSET		= 0x0B,
    DAC960_PD_MBOX12_OFFSET		= 0x0C,
    DAC960_PD_STSID_OFFSET		= 0x0D,
    DAC960_PD_STS_OFFSET		= 0x0E,
    DAC960_PD_ERRSTS_OFFSET		= 0x3F,
    DAC960_PD_IDB_OFFSET		= 0x40,
    DAC960_PD_ODB_OFFSET		= 0x41,
    DAC960_PD_IRQEN_OFFSET		= 0x43,
}

//
// DAC960 PD Series Inbound Door Bell Register.
//
pub const DAC960_PD_IDB_HWMBOX_NEW_CMD: c_uint = 0x01;
pub const DAC960_PD_IDB_HWMBOX_ACK_STS: c_uint = 0x02;
pub const DAC960_PD_IDB_GEN_IRQ: c_uint = 0x04;
pub const DAC960_PD_IDB_CTRL_RESET: c_uint = 0x08;
pub const DAC960_PD_IDB_HWMBOX_FULL: c_uint = 0x01;
pub const DAC960_PD_IDB_INIT_IN_PROGRESS: c_uint = 0x02;
//
// DAC960 PD Series Outbound Door Bell Register.
//
pub const DAC960_PD_ODB_HWMBOX_ACK_IRQ: c_uint = 0x01;
pub const DAC960_PD_ODB_HWMBOX_STS_AVAIL: c_uint = 0x01;
//
// DAC960 PD Series Interrupt Enable Register.
//
pub const DAC960_PD_IRQMASK_ENABLE_IRQ: c_uint = 0x01;
//
// DAC960 PD Series Error Status Register.
//
pub const DAC960_PD_ERRSTS_PENDING: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct myrb_privdata {
    pub hw_init: myrb_hw_init_t,
    pub irq_handler: irq_handler_t,
    pub mmio_size: c_uint,
}
