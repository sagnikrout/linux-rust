//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/mptscsih.h
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
// linux/drivers/message/fusion/mptscsih.h
// High performance SCSI / Fibre Channel SCSI Host device driver.
// For use with PCI chip/adapter(s):
// LSIFC9xx/LSI409xx Fibre Channel
// running LSI Fusion MPT (Message Passing Technology) firmware.
//
// Copyright (c) 1999-2008 LSI Corporation
// (mailto:DL-MPTFusionLinux@lsi.com)
//
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//

// Macro flag: #define SCSIHOST_H_INCLUDED
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//
// SCSI Public stuff...
//

pub const MPT_ICFLAG_BUF_CAP: c_uint = 0x01	/* ReadBuffer Read Capacity format */;
pub const MPT_ICFLAG_ECHO: c_uint = 0x02	/* ReadBuffer Echo buffer format */;
pub const MPT_ICFLAG_EBOS: c_uint = 0x04	/* ReadBuffer Echo buffer has EBOS */;
pub const MPT_ICFLAG_PHYS_DISK: c_uint = 0x08	/* Any SCSI IO but do Phys Disk Format */;
pub const MPT_ICFLAG_TAGGED_CMD: c_uint = 0x10	/* Do tagged IO */;
pub const MPT_ICFLAG_DID_RESET: c_uint = 0x20	/* Bus Reset occurred with this command */;
pub const MPT_ICFLAG_RESERVED: c_uint = 0x40	/* Reserved has been issued */;
pub const MPT_SCSI_CMD_PER_DEV_HIGH: c_int = 64;
pub const MPT_SCSI_CMD_PER_DEV_LOW: c_int = 32;
pub const MPT_SCSI_CMD_PER_LUN: c_int = 7;
pub const MPT_SCSI_MAX_SECTORS: c_int = 8192;
// SCSI driver setup structure. Settings can be overridden
// by command line options.
//
pub const MPTSCSIH_DOMAIN_VALIDATION: c_int = 1;
pub const MPTSCSIH_MAX_WIDTH: c_int = 1;
pub const MPTSCSIH_MIN_SYNC: c_uint = 0x08;
pub const MPTSCSIH_SAF_TE: c_int = 0;
pub const MPTSCSIH_PT_CLEAR: c_int = 0;

extern "C" {
    pub fn mptscsih_remove(: *mut pci_dev);
}
extern "C" {
    pub fn mptscsih_shutdown(: *mut pci_dev);
}

extern "C" {
    pub fn mptscsih_suspend(pdev: *mut pci_dev, state: pm_message_t) -> c_int;
}
extern "C" {
    pub fn mptscsih_resume(pdev: *mut pci_dev) -> c_int;
}

extern "C" {
    pub fn mptscsih_show_info(: *mut seq_file, : *mut Scsi_Host) -> c_int;
}
extern "C" {
    pub fn mptscsih_info(SChost: *mut Scsi_Host) -> *const c_char;
}
extern "C" {
    pub fn mptscsih_qcmd(SCpnt: *mut scsi_cmnd) -> scsi_qc_status;
}
extern "C" {
    pub fn mptscsih_sdev_destroy(device: *mut scsi_device);
}
extern "C" {
    pub fn mptscsih_abort(SCpnt: *mut *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn mptscsih_dev_reset(SCpnt: *mut *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn mptscsih_bus_reset(SCpnt: *mut *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn mptscsih_host_reset(SCpnt: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn mptscsih_bios_param(sdev: *mut *mut scsi_device, unused: *mut gendisk, capacity: sector_t, geom[]: c_int) -> c_int;
}
extern "C" {
    pub fn mptscsih_io_done(ioc: *mut MPT_ADAPTER, mf: *mut MPT_FRAME_HDR, r: *mut MPT_FRAME_HDR) -> c_int;
}
extern "C" {
    pub fn mptscsih_taskmgmt_complete(ioc: *mut MPT_ADAPTER, mf: *mut MPT_FRAME_HDR, r: *mut MPT_FRAME_HDR) -> c_int;
}
extern "C" {
    pub fn mptscsih_scandv_complete(ioc: *mut MPT_ADAPTER, mf: *mut MPT_FRAME_HDR, r: *mut MPT_FRAME_HDR) -> c_int;
}
extern "C" {
    pub fn mptscsih_event_process(ioc: *mut MPT_ADAPTER, pEvReply: *mut EventNotificationReply_t) -> c_int;
}
extern "C" {
    pub fn mptscsih_ioc_reset(ioc: *mut MPT_ADAPTER, post_reset: c_int) -> c_int;
}
extern "C" {
    pub fn mptscsih_change_queue_depth(sdev: *mut scsi_device, qdepth: c_int) -> c_int;
}
extern "C" {
    pub fn mptscsih_raid_id_to_num(ioc: *mut MPT_ADAPTER, channel: u8, id: u8) -> u8;
}
extern "C" {
    pub fn mptscsih_is_phys_disk(ioc: *mut MPT_ADAPTER, channel: u8, id: u8) -> c_int;
}
extern "C" {
    pub fn mptscsih_taskmgmt_response_code(ioc: *mut MPT_ADAPTER, response_code: u8);
}
extern "C" {
    pub fn mptscsih_flush_running_cmds(hd: *mut MPT_SCSI_HOST);
}
