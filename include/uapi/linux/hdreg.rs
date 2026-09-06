//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/hdreg.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// Command Header sizes for IOCTL commands
//

pub const IDE_DRIVE_TASK_NO_DATA: c_int = 0;

pub const IDE_DRIVE_TASK_SET_XFER: c_int = 1;
pub const IDE_DRIVE_TASK_IN: c_int = 2;
pub const IDE_DRIVE_TASK_OUT: c_int = 3;

pub const IDE_DRIVE_TASK_RAW_WRITE: c_int = 4;
//
// Define standard taskfile in/out register
//
pub const IDE_TASKFILE_STD_IN_FLAGS: c_uint = 0xFE;
pub const IDE_HOB_STD_IN_FLAGS: c_uint = 0x3C;
pub const IDE_TASKFILE_STD_OUT_FLAGS: c_uint = 0xFE;
pub const IDE_HOB_STD_OUT_FLAGS: c_uint = 0x3C;
pub type task_ioreg_t = c_uchar;
pub type sata_ioreg_t = c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hd_drive_cmd_hdr {
    pub command: __u8,
    pub sector_number: __u8,
    pub feature: __u8,
    pub sector_count: __u8,
}

pub const TASKFILE_NO_DATA: c_uint = 0x0000;
pub const TASKFILE_IN: c_uint = 0x0001;
pub const TASKFILE_MULTI_IN: c_uint = 0x0002;
pub const TASKFILE_OUT: c_uint = 0x0004;
pub const TASKFILE_MULTI_OUT: c_uint = 0x0008;
pub const TASKFILE_IN_OUT: c_uint = 0x0010;
pub const TASKFILE_IN_DMA: c_uint = 0x0020;
pub const TASKFILE_OUT_DMA: c_uint = 0x0040;
pub const TASKFILE_IN_DMAQ: c_uint = 0x0080;
pub const TASKFILE_OUT_DMAQ: c_uint = 0x0100;
pub const TASKFILE_P_IN: c_uint = 0x0200;
pub const TASKFILE_P_OUT: c_uint = 0x0400;
pub const TASKFILE_P_IN_DMA: c_uint = 0x0800;
pub const TASKFILE_P_OUT_DMA: c_uint = 0x1000;
pub const TASKFILE_P_IN_DMAQ: c_uint = 0x2000;
pub const TASKFILE_P_OUT_DMAQ: c_uint = 0x4000;
pub const TASKFILE_48: c_uint = 0x8000;
pub const TASKFILE_INVALID: c_uint = 0x7fff;

// ATA/ATAPI Commands pre T13 Spec
pub const WIN_NOP: c_uint = 0x00;
//
// 0x01->0x02 Reserved
//
pub const CFA_REQ_EXT_ERROR_CODE: c_uint = 0x03 /* CFA Request Extended Error Code */;
//
// 0x04->0x07 Reserved
//
pub const WIN_SRST: c_uint = 0x08 /* ATAPI soft reset command */;
pub const WIN_DEVICE_RESET: c_uint = 0x08;
//
// 0x09->0x0F Reserved
//
pub const WIN_RECAL: c_uint = 0x10;

//
// 0x10->0x1F Reserved
//
pub const WIN_READ: c_uint = 0x20 /* 28-Bit */;
pub const WIN_READ_ONCE: c_uint = 0x21 /* 28-Bit without retries */;
pub const WIN_READ_LONG: c_uint = 0x22 /* 28-Bit */;
pub const WIN_READ_LONG_ONCE: c_uint = 0x23 /* 28-Bit without retries */;
pub const WIN_READ_EXT: c_uint = 0x24 /* 48-Bit */;
pub const WIN_READDMA_EXT: c_uint = 0x25 /* 48-Bit */;
pub const WIN_READDMA_QUEUED_EXT: c_uint = 0x26 /* 48-Bit */;
pub const WIN_READ_NATIVE_MAX_EXT: c_uint = 0x27 /* 48-Bit */;
//
// 0x28
//
pub const WIN_MULTREAD_EXT: c_uint = 0x29 /* 48-Bit */;
//
// 0x2A->0x2F Reserved
//
pub const WIN_WRITE: c_uint = 0x30 /* 28-Bit */;
pub const WIN_WRITE_ONCE: c_uint = 0x31 /* 28-Bit without retries */;
pub const WIN_WRITE_LONG: c_uint = 0x32 /* 28-Bit */;
pub const WIN_WRITE_LONG_ONCE: c_uint = 0x33 /* 28-Bit without retries */;
pub const WIN_WRITE_EXT: c_uint = 0x34 /* 48-Bit */;
pub const WIN_WRITEDMA_EXT: c_uint = 0x35 /* 48-Bit */;
pub const WIN_WRITEDMA_QUEUED_EXT: c_uint = 0x36 /* 48-Bit */;
pub const WIN_SET_MAX_EXT: c_uint = 0x37 /* 48-Bit */;
pub const CFA_WRITE_SECT_WO_ERASE: c_uint = 0x38 /* CFA Write Sectors without erase */;
pub const WIN_MULTWRITE_EXT: c_uint = 0x39 /* 48-Bit */;
//
// 0x3A->0x3B Reserved
//
pub const WIN_WRITE_VERIFY: c_uint = 0x3C /* 28-Bit */;
//
// 0x3D->0x3F Reserved
//
pub const WIN_VERIFY: c_uint = 0x40 /* 28-Bit - Read Verify Sectors */;
pub const WIN_VERIFY_ONCE: c_uint = 0x41 /* 28-Bit - without retries */;
pub const WIN_VERIFY_EXT: c_uint = 0x42 /* 48-Bit */;
//
// 0x43->0x4F Reserved
//
pub const WIN_FORMAT: c_uint = 0x50;
//
// 0x51->0x5F Reserved
//
pub const WIN_INIT: c_uint = 0x60;
//
// 0x61->0x5F Reserved
//
pub const WIN_SEEK: c_uint = 0x70 /* 0x70-0x7F Reserved */;
pub const CFA_TRANSLATE_SECTOR: c_uint = 0x87 /* CFA Translate Sector */;
pub const WIN_DIAGNOSE: c_uint = 0x90;
pub const WIN_SPECIFY: c_uint = 0x91 /* set drive geometry translation */;
pub const WIN_DOWNLOAD_MICROCODE: c_uint = 0x92;
pub const WIN_STANDBYNOW2: c_uint = 0x94;
pub const WIN_STANDBY2: c_uint = 0x96;
pub const WIN_SETIDLE2: c_uint = 0x97;
pub const WIN_CHECKPOWERMODE2: c_uint = 0x98;
pub const WIN_SLEEPNOW2: c_uint = 0x99;
//
// 0x9A VENDOR
//
pub const WIN_PACKETCMD: c_uint = 0xA0 /* Send a packet command. */;
pub const WIN_PIDENTIFY: c_uint = 0xA1 /* identify ATAPI device	*/;
pub const WIN_QUEUED_SERVICE: c_uint = 0xA2;
pub const WIN_SMART: c_uint = 0xB0 /* self-monitoring and reporting */;
pub const CFA_ERASE_SECTORS: c_uint = 0xC0;
pub const WIN_MULTREAD: c_uint = 0xC4 /* read sectors using multiple mode*/;
pub const WIN_MULTWRITE: c_uint = 0xC5 /* write sectors using multiple mode */;
pub const WIN_SETMULT: c_uint = 0xC6 /* enable/disable multiple mode */;
pub const WIN_READDMA_QUEUED: c_uint = 0xC7 /* read sectors using Queued DMA transfers */;
pub const WIN_READDMA: c_uint = 0xC8 /* read sectors using DMA transfers */;
pub const WIN_READDMA_ONCE: c_uint = 0xC9 /* 28-Bit - without retries */;
pub const WIN_WRITEDMA: c_uint = 0xCA /* write sectors using DMA transfers */;
pub const WIN_WRITEDMA_ONCE: c_uint = 0xCB /* 28-Bit - without retries */;
pub const WIN_WRITEDMA_QUEUED: c_uint = 0xCC /* write sectors using Queued DMA transfers */;
pub const CFA_WRITE_MULTI_WO_ERASE: c_uint = 0xCD /* CFA Write multiple without erase */;
pub const WIN_GETMEDIASTATUS: c_uint = 0xDA;
pub const WIN_ACKMEDIACHANGE: c_uint = 0xDB /* ATA-1, ATA-2 vendor */;
pub const WIN_POSTBOOT: c_uint = 0xDC;
pub const WIN_PREBOOT: c_uint = 0xDD;
pub const WIN_DOORLOCK: c_uint = 0xDE /* lock door on removable drives */;
pub const WIN_DOORUNLOCK: c_uint = 0xDF /* unlock door on removable drives */;
pub const WIN_STANDBYNOW1: c_uint = 0xE0;
pub const WIN_IDLEIMMEDIATE: c_uint = 0xE1 /* force drive to become "ready" */;
pub const WIN_STANDBY: c_uint = 0xE2 /* Set device in Standby Mode */;
pub const WIN_SETIDLE1: c_uint = 0xE3;
pub const WIN_READ_BUFFER: c_uint = 0xE4 /* force read only 1 sector */;
pub const WIN_CHECKPOWERMODE1: c_uint = 0xE5;
pub const WIN_SLEEPNOW1: c_uint = 0xE6;
pub const WIN_FLUSH_CACHE: c_uint = 0xE7;
pub const WIN_WRITE_BUFFER: c_uint = 0xE8 /* force write only 1 sector */;
pub const WIN_WRITE_SAME: c_uint = 0xE9 /* read ata-2 to use */;
// SET_FEATURES 0x22 or 0xDD
pub const WIN_FLUSH_CACHE_EXT: c_uint = 0xEA /* 48-Bit */;
pub const WIN_IDENTIFY: c_uint = 0xEC /* ask drive to identify itself	*/;
pub const WIN_MEDIAEJECT: c_uint = 0xED;
pub const WIN_IDENTIFY_DMA: c_uint = 0xEE /* same as WIN_IDENTIFY, but DMA */;
pub const WIN_SETFEATURES: c_uint = 0xEF /* set special drive features */;
pub const EXABYTE_ENABLE_NEST: c_uint = 0xF0;
pub const WIN_SECURITY_SET_PASS: c_uint = 0xF1;
pub const WIN_SECURITY_UNLOCK: c_uint = 0xF2;
pub const WIN_SECURITY_ERASE_PREPARE: c_uint = 0xF3;
pub const WIN_SECURITY_ERASE_UNIT: c_uint = 0xF4;
pub const WIN_SECURITY_FREEZE_LOCK: c_uint = 0xF5;
pub const WIN_SECURITY_DISABLE: c_uint = 0xF6;
pub const WIN_READ_NATIVE_MAX: c_uint = 0xF8 /* return the native maximum address */;
pub const WIN_SET_MAX: c_uint = 0xF9;
pub const DISABLE_SEAGATE: c_uint = 0xFB;
// WIN_SMART sub-commands
pub const SMART_READ_VALUES: c_uint = 0xD0;
pub const SMART_READ_THRESHOLDS: c_uint = 0xD1;
pub const SMART_AUTOSAVE: c_uint = 0xD2;
pub const SMART_SAVE: c_uint = 0xD3;
pub const SMART_IMMEDIATE_OFFLINE: c_uint = 0xD4;
pub const SMART_READ_LOG_SECTOR: c_uint = 0xD5;
pub const SMART_WRITE_LOG_SECTOR: c_uint = 0xD6;
pub const SMART_WRITE_THRESHOLDS: c_uint = 0xD7;
pub const SMART_ENABLE: c_uint = 0xD8;
pub const SMART_DISABLE: c_uint = 0xD9;
pub const SMART_STATUS: c_uint = 0xDA;
pub const SMART_AUTO_OFFLINE: c_uint = 0xDB;
// Password used in TF4 & TF5 executing SMART commands
pub const SMART_LCYL_PASS: c_uint = 0x4F;
pub const SMART_HCYL_PASS: c_uint = 0xC2;
// WIN_SETFEATURES sub-commands
pub const SETFEATURES_EN_8BIT: c_uint = 0x01	/* Enable 8-Bit Transfers */;
pub const SETFEATURES_EN_WCACHE: c_uint = 0x02	/* Enable write cache */;
pub const SETFEATURES_DIS_DEFECT: c_uint = 0x04	/* Disable Defect Management */;
pub const SETFEATURES_EN_APM: c_uint = 0x05	/* Enable advanced power management */;
pub const SETFEATURES_EN_SAME_R: c_uint = 0x22	/* for a region ATA-1 */;
pub const SETFEATURES_DIS_MSN: c_uint = 0x31	/* Disable Media Status Notification */;
pub const SETFEATURES_DIS_RETRY: c_uint = 0x33	/* Disable Retry */;
pub const SETFEATURES_EN_AAM: c_uint = 0x42	/* Enable Automatic Acoustic Management */;
pub const SETFEATURES_RW_LONG: c_uint = 0x44	/* Set Length of VS bytes */;
pub const SETFEATURES_SET_CACHE: c_uint = 0x54	/* Set Cache segments to SC Reg. Val */;
pub const SETFEATURES_DIS_RLA: c_uint = 0x55	/* Disable read look-ahead feature */;
pub const SETFEATURES_EN_RI: c_uint = 0x5D	/* Enable release interrupt */;
pub const SETFEATURES_EN_SI: c_uint = 0x5E	/* Enable SERVICE interrupt */;
pub const SETFEATURES_DIS_RPOD: c_uint = 0x66	/* Disable reverting to power on defaults */;
pub const SETFEATURES_DIS_ECC: c_uint = 0x77	/* Disable ECC byte count */;
pub const SETFEATURES_DIS_8BIT: c_uint = 0x81	/* Disable 8-Bit Transfers */;
pub const SETFEATURES_DIS_WCACHE: c_uint = 0x82	/* Disable write cache */;
pub const SETFEATURES_EN_DEFECT: c_uint = 0x84	/* Enable Defect Management */;
pub const SETFEATURES_DIS_APM: c_uint = 0x85	/* Disable advanced power management */;
pub const SETFEATURES_EN_ECC: c_uint = 0x88	/* Enable ECC byte count */;
pub const SETFEATURES_EN_MSN: c_uint = 0x95	/* Enable Media Status Notification */;
pub const SETFEATURES_EN_RETRY: c_uint = 0x99	/* Enable Retry */;
pub const SETFEATURES_EN_RLA: c_uint = 0xAA	/* Enable read look-ahead feature */;
pub const SETFEATURES_PREFETCH: c_uint = 0xAB	/* Sets drive prefetch value */;
pub const SETFEATURES_EN_REST: c_uint = 0xAC	/* ATA-1 */;
pub const SETFEATURES_4B_RW_LONG: c_uint = 0xBB	/* Set Length of 4 bytes */;
pub const SETFEATURES_DIS_AAM: c_uint = 0xC2	/* Disable Automatic Acoustic Management */;
pub const SETFEATURES_EN_RPOD: c_uint = 0xCC	/* Enable reverting to power on defaults */;
pub const SETFEATURES_DIS_RI: c_uint = 0xDD	/* Disable release interrupt ATAPI */;
pub const SETFEATURES_EN_SAME_M: c_uint = 0xDD	/* for a entire device ATA-1 */;
pub const SETFEATURES_DIS_SI: c_uint = 0xDE	/* Disable SERVICE interrupt ATAPI */;
// WIN_SECURITY sub-commands
pub const SECURITY_SET_PASSWORD: c_uint = 0xBA;
pub const SECURITY_UNLOCK: c_uint = 0xBB;
pub const SECURITY_ERASE_PREPARE: c_uint = 0xBC;
pub const SECURITY_ERASE_UNIT: c_uint = 0xBD;
pub const SECURITY_FREEZE_LOCK: c_uint = 0xBE;
pub const SECURITY_DISABLE_PASSWORD: c_uint = 0xBF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hd_geometry {
    pub heads: c_uchar,
    pub sectors: c_uchar,
    pub cylinders: c_ushort,
    pub start: c_ulong,
}

// hd/ide ctl's that pass (arg) ptrs to user space are numbered 0x030n/0x031n
pub const HDIO_GETGEO: c_uint = 0x0301	/* get device geometry */;
pub const HDIO_GET_UNMASKINTR: c_uint = 0x0302	/* get current unmask setting */;
pub const HDIO_GET_MULTCOUNT: c_uint = 0x0304	/* get current IDE blockmode setting */;
pub const HDIO_GET_QDMA: c_uint = 0x0305	/* get use-qdma flag */;
pub const HDIO_SET_XFER: c_uint = 0x0306  /* set transfer rate via proc */;
pub const HDIO_OBSOLETE_IDENTITY: c_uint = 0x0307	/* OBSOLETE, DO NOT USE: returns 142 bytes */;
pub const HDIO_GET_KEEPSETTINGS: c_uint = 0x0308	/* get keep-settings-on-reset flag */;
pub const HDIO_GET_32BIT: c_uint = 0x0309	/* get current io_32bit setting */;
pub const HDIO_GET_NOWERR: c_uint = 0x030a	/* get ignore-write-error flag */;
pub const HDIO_GET_DMA: c_uint = 0x030b	/* get use-dma flag */;
pub const HDIO_GET_NICE: c_uint = 0x030c	/* get nice flags */;
pub const HDIO_GET_IDENTITY: c_uint = 0x030d	/* get IDE identification info */;
pub const HDIO_GET_WCACHE: c_uint = 0x030e	/* get write cache mode on|off */;
pub const HDIO_GET_ACOUSTIC: c_uint = 0x030f	/* get acoustic value */;
pub const HDIO_GET_ADDRESS: c_uint = 0x0310	/* */;
pub const HDIO_GET_BUSSTATE: c_uint = 0x031a	/* get the bus state of the hwif */;
pub const HDIO_TRISTATE_HWIF: c_uint = 0x031b	/* execute a channel tristate */;
pub const HDIO_DRIVE_RESET: c_uint = 0x031c	/* execute a device reset */;
pub const HDIO_DRIVE_TASKFILE: c_uint = 0x031d	/* execute raw taskfile */;
pub const HDIO_DRIVE_TASK: c_uint = 0x031e	/* execute task and special drive command */;
pub const HDIO_DRIVE_CMD: c_uint = 0x031f	/* execute a special drive command */;

// hd/ide ctl's that pass (arg) non-ptr values are numbered 0x032n/0x033n
pub const HDIO_SET_MULTCOUNT: c_uint = 0x0321	/* change IDE blockmode */;
pub const HDIO_SET_UNMASKINTR: c_uint = 0x0322	/* permit other irqs during I/O */;
pub const HDIO_SET_KEEPSETTINGS: c_uint = 0x0323	/* keep ioctl settings on reset */;
pub const HDIO_SET_32BIT: c_uint = 0x0324	/* change io_32bit flags */;
pub const HDIO_SET_NOWERR: c_uint = 0x0325	/* change ignore-write-error flag */;
pub const HDIO_SET_DMA: c_uint = 0x0326	/* change use-dma flag */;
pub const HDIO_SET_PIO_MODE: c_uint = 0x0327	/* reconfig interface to new speed */;
pub const HDIO_SCAN_HWIF: c_uint = 0x0328	/* register and (re)scan interface */;
pub const HDIO_UNREGISTER_HWIF: c_uint = 0x032a  /* unregister interface */;

pub const HDIO_SET_NICE: c_uint = 0x0329	/* set nice flags */;
pub const HDIO_SET_WCACHE: c_uint = 0x032b	/* change write cache enable-disable */;
pub const HDIO_SET_ACOUSTIC: c_uint = 0x032c	/* change acoustic behavior */;
pub const HDIO_SET_BUSSTATE: c_uint = 0x032d	/* set the bus state of the hwif */;
pub const HDIO_SET_QDMA: c_uint = 0x032e	/* change use-qdma flag */;
pub const HDIO_SET_ADDRESS: c_uint = 0x032f	/* change lba addressing modes */;
// bus states
// hd/ide ctl's that pass (arg) ptrs to user space are numbered 0x033n/0x033n
// 0x330 is reserved - used to be HDIO_GETGEO_BIG
// 0x331 is reserved - used to be HDIO_GETGEO_BIG_RAW
// 0x338 is reserved - used to be HDIO_SET_IDE_SCSI
// 0x339 is reserved - used to be HDIO_SET_SCSI_IDE
//
// Structure returned by HDIO_GET_IDENTITY, as per ANSI NCITS ATA6 rev.1b spec.
//
// If you change something here, please remember to update fix_driveid() in
// ide/probe.c.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hd_driveid {
    pub /: *mut *mut unsigned short config; / lots of obsolete bit flags,
    pub /: *mut *mut unsigned short cyls; / Obsolete, "physical" cyls,
    pub /: *mut *mut unsigned short reserved2; / reserved (word 2),
    pub /: *mut *mut unsigned short heads; / Obsolete, "physical" heads,
    pub /: *mut *mut unsigned short track_bytes; / unformatted bytes per track,
    pub /: *mut *mut unsigned short sector_bytes; / unformatted bytes per sector,
    pub /: *mut *mut unsigned short sectors; / Obsolete, "physical" sectors per track,
    pub /: *mut *mut unsigned short vendor0; / vendor unique,
    pub /: *mut *mut unsigned short vendor1; / vendor unique,
    pub /: *mut *mut unsigned short vendor2; / Retired vendor unique,
    pub /: *mut *mut unsigned char serial_no[20]; / 0 = not_specified,
    pub /: *mut *mut unsigned short buf_type; / Retired,
    pub increments: *mut *mut unsigned short buf_size; / Retired, 512 byte,
// 0 = not_specified
//
    pub /: *mut *mut unsigned short ecc_bytes; / for r/w long cmds; 0 = not_specified,
    pub /: *mut *mut unsigned char fw_rev[8]; / 0 = not_specified,
    pub /: *mut *mut unsigned char model[40]; / 0 = not_specified,
    pub /: *mut *mut unsigned char max_multsect; / 0=not_implemented,
    pub /: *mut *mut unsigned char vendor3; / vendor unique,
    pub /: *mut *mut unsigned short dword_io; / 0=not_implemented; 1=implemented,
    pub /: *mut *mut unsigned char vendor4; / vendor unique,
    pub 49): *mut *mut unsigned char capability; / (upper byte of word,
// 3:	IORDYsup
// 2:	IORDYsw
// 1:	LBA
// 0:	DMA
//
    pub /: *mut *mut unsigned short reserved50; / reserved (word 50),
    pub /: *mut *mut unsigned char vendor5; / Obsolete, vendor unique,
    pub /: *mut *mut unsigned char tPIO; / Obsolete, 0=slow, 1=medium, 2=fast,
    pub /: *mut *mut unsigned char vendor6; / Obsolete, vendor unique,
    pub /: *mut *mut unsigned char tDMA; / Obsolete, 0=slow, 1=medium, 2=fast,
    pub 53): *mut *mut unsigned short field_valid; / (word,
// 2:	ultra_ok	word  88
// 1:	eide_ok		words 64-70
// 0:	cur_ok		words 54-58
//
    pub /: *mut *mut unsigned short cur_cyls; / Obsolete, logical cylinders,
    pub /: *mut *mut unsigned short cur_heads; / Obsolete, l heads,
    pub /: *mut *mut unsigned short cur_sectors; / Obsolete, l sectors per track,
    pub /: *mut *mut unsigned short cur_capacity0; / Obsolete, l total sectors on drive,
    pub /: *mut *mut unsigned short cur_capacity1; / Obsolete, (2 words, misaligned int),
    pub /: *mut *mut unsigned char multsect; / current multiple sector count,
    pub /: *mut *mut unsigned char multsect_valid; / when (bit0==1) multsect is ok,
    pub /: *mut *mut unsigned int lba_capacity; / Obsolete, total number of sectors,
    pub /: *mut *mut unsigned short dma_1word; / Obsolete, single-word dma info,
    pub /: *mut *mut unsigned short dma_mword; / multiple-word dma info,
    pub /: *mut *mut unsigned short eide_pio_modes; / bits 0:mode3 1:mode4,
    pub /: *mut *mut unsigned short eide_dma_min; / min mword dma cycle time (ns),
    pub /: *mut *mut unsigned short eide_dma_time; / recommended mword dma cycle time (ns),
    pub /: *mut *mut unsigned short eide_pio; / min cycle time (ns), no IORDY,
    pub /: *mut *mut unsigned short eide_pio_iordy; / min cycle time (ns), with IORDY,
    pub 69-70: *mut *mut unsigned short words69_70[2]; / reserved words,
// future command overlap and queuing
//
    pub 71-74: *mut *mut unsigned short words71_74[4]; / reserved words,
// for IDENTIFY PACKET DEVICE command
//
    pub 75): *mut *mut unsigned short queue_depth; / (word,
// 15:5	reserved
// 4:0	Maximum queue depth -1
//
    pub /: *mut *mut unsigned short words76_79[4]; / reserved words 76-79,
    pub /: *mut *mut unsigned short major_rev_num; / (word 80),
    pub /: *mut *mut unsigned short minor_rev_num; / (word 81),
    pub supported: *mut *mut unsigned short command_set_1; / (word 82),
// 15:	Obsolete
// 14:	NOP command
// 13:	READ_BUFFER
// 12:	WRITE_BUFFER
// 11:	Obsolete
// 10:	Host Protected Area
// 9:	DEVICE Reset
// 8:	SERVICE Interrupt
// 7:	Release Interrupt
// 6:	look-ahead
// 5:	write cache
// 4:	PACKET Command
// 3:	Power Management Feature Set
// 2:	Removable Feature Set
// 1:	Security Feature Set
// 0:	SMART Feature Set
//
    pub 83): *mut *mut unsigned short command_set_2; / (word,
// 15:	Shall be ZERO
// 14:	Shall be ONE
// 13:	FLUSH CACHE EXT
// 12:	FLUSH CACHE
// 11:	Device Configuration Overlay
// 10:	48-bit Address Feature Set
// 9:	Automatic Acoustic Management
// 8:	SET MAX security
// 7:	reserved 1407DT PARTIES
// 6:	SetF sub-command Power-Up
// 5:	Power-Up in Standby Feature Set
// 4:	Removable Media Notification
// 3:	APM Feature Set
// 2:	CFA Feature Set
// 1:	READ/WRITE DMA QUEUED
// 0:	Download MicroCode
//
    pub 84): *mut *mut unsigned short cfsse; / (word,
// cmd set-feature supported extensions
// 15:	Shall be ZERO
// 14:	Shall be ONE
// 13:6	reserved
// 5:	General Purpose Logging
// 4:	Streaming Feature Set
// 3:	Media Card Pass Through
// 2:	Media Serial Number Valid
// 1:	SMART selt-test supported
// 0:	SMART error logging
//
    pub 85): *mut *mut unsigned short cfs_enable_1; / (word,
// command set-feature enabled
// 15:	Obsolete
// 14:	NOP command
// 13:	READ_BUFFER
// 12:	WRITE_BUFFER
// 11:	Obsolete
// 10:	Host Protected Area
// 9:	DEVICE Reset
// 8:	SERVICE Interrupt
// 7:	Release Interrupt
// 6:	look-ahead
// 5:	write cache
// 4:	PACKET Command
// 3:	Power Management Feature Set
// 2:	Removable Feature Set
// 1:	Security Feature Set
// 0:	SMART Feature Set
//
    pub 86): *mut *mut unsigned short cfs_enable_2; / (word,
// command set-feature enabled
// 15:	Shall be ZERO
// 14:	Shall be ONE
// 13:	FLUSH CACHE EXT
// 12:	FLUSH CACHE
// 11:	Device Configuration Overlay
// 10:	48-bit Address Feature Set
// 9:	Automatic Acoustic Management
// 8:	SET MAX security
// 7:	reserved 1407DT PARTIES
// 6:	SetF sub-command Power-Up
// 5:	Power-Up in Standby Feature Set
// 4:	Removable Media Notification
// 3:	APM Feature Set
// 2:	CFA Feature Set
// 1:	READ/WRITE DMA QUEUED
// 0:	Download MicroCode
//
    pub 87): *mut *mut unsigned short csf_default; / (word,
// command set-feature default
// 15:	Shall be ZERO
// 14:	Shall be ONE
// 13:6	reserved
// 5:	General Purpose Logging enabled
// 4:	Valid CONFIGURE STREAM executed
// 3:	Media Card Pass Through enabled
// 2:	Media Serial Number Valid
// 1:	SMART selt-test supported
// 0:	SMART error logging
//
    pub /: *mut *mut unsigned short dma_ultra; / (word 88),
    pub /: *mut *mut unsigned short trseuc; / time required for security erase,
    pub /: *mut *mut unsigned short trsEuc; / time required for enhanced erase,
    pub /: *mut *mut unsigned short CurAPMvalues; / current APM values,
    pub /: *mut *mut unsigned short mprc; / master password revision code,
    pub 93): *mut *mut unsigned short hw_config; / hardware config (word,
// 15:	Shall be ZERO
// 14:	Shall be ONE
// 13:
// 12:
// 11:
// 10:
// 9:
// 8:
// 7:
// 6:
// 5:
// 4:
// 3:
// 2:
// 1:
// 0:	Shall be ONE
//
    pub 94): *mut *mut unsigned short acoustic; / (word,
// 15:8	Vendor's recommended value
// 7:0	current value
//
    pub /: *mut *mut unsigned short msrqs; / min stream request size,
    pub /: *mut *mut unsigned short sxfert; / stream transfer time,
    pub /: *mut *mut unsigned short sal; / stream access latency,
    pub /: *mut *mut unsigned int spg; / stream performance granularity,
    pub /: *mut *mut unsigned long long lba_capacity_2;/ 48-bit total number of sectors,
    pub /: *mut *mut unsigned short words104_125[22];/ reserved words 104-125,
    pub /: *mut *mut unsigned short last_lun; / (word 126),
    pub Set: *mut *mut unsigned short word127; / (word 127) Feature,
// Removable Media Notification
// 15:2	reserved
// 1:0	00 = not supported
// 01 = supported
// 10 = reserved
// 11 = reserved
//
    pub 128): *mut *mut unsigned short dlf; / (word,
// device lock function
// 15:9	reserved
// 8	security level 1:max 0:high
// 7:6	reserved
// 5	enhanced erase
// 4	expire
// 3	frozen
// 2	locked
// 1	en/disabled
// 0	capability
//
    pub 129): *mut *mut unsigned short csfo; / (word,
// current set features options
// 15:4	reserved
// 3:	auto reassign
// 2:	reverting
// 1:	read-look-ahead
// 0:	write cache
//
    pub /: *mut *mut unsigned short words130_155[26];/ reserved vendor words 130-155,
    pub /: *mut *mut unsigned short word156; / reserved vendor word 156,
    pub /: *mut *mut unsigned short words157_159[3];/ reserved vendor words 157-159,
    pub Mode: *mut *mut unsigned short cfa_power; / (word 160) CFA Power,
// 15 word 160 supported
// 14 reserved
// 13
// 12
// 11:0
//
    pub /: *mut *mut unsigned short words161_175[15];/ Reserved for CFA,
    pub /: *mut *mut unsigned short words176_205[30];/ Current Media Serial Number,
    pub /: *mut *mut unsigned short words206_254[49];/ reserved words 206-254,
    pub 255): *mut *mut unsigned short integrity_word; / (word,
// 15:8 Checksum
// 7:0 Signature
//
}

//
// IDE "nice" flags. These are used on a per drive basis to determine
// when to be nice and give more bandwidth to the other devices which
// share the same IDE bus.
//

