//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cdrom.h
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
// -- <linux/cdrom.h>
// General header file for linux CD-ROM drivers
// Copyright (C) 1992         David Giller, rafetmad@oxy.edu
// 1994, 1995   Eberhard Mönkeberg, emoenke@gwdg.de
// 1996         David van Leeuwen, david@tm.tno.nl
// 1997, 1998   Erik Andersen, andersee@debian.org
// 1998-2002    Jens Axboe, axboe@suse.de
//

//
// _OLD will use PIO transfer on atapi devices, _BPC_* will use DMA
//

// Uniform cdrom data structures for cdrom.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdrom_device_info {
    pub /: *const *const *const cdrom_device_ops ops; / link to device_ops,
    pub /: *mut *mut list_head list; / linked list of all device_info,
    pub /: *mut *mut *mut gendisk disk; / matching block layer disk,
    pub /: *mut *mut *mut void handle; / driver-dependent data,
// specifications
    pub /: *mut *mut int mask; / mask of capability: disables them,
    pub /: *mut *mut int speed; / maximum speed for reading data,
    pub /: *mut *mut int capacity; / number of discs in jukebox,
// device-related storage
    pub /: *mut *mut unsigned int options : 30; / options flags,
    pub /: *mut *mut unsigned mc_flags : 2; / media change buffer flags,
    pub /: *mut *mut unsigned int vfs_events; / cached events for vfs path,
    pub /: *mut *mut unsigned int ioctl_events; / cached events for ioctl path,
    pub /: *mut *mut int use_count; / number of times device opened,
    pub /: *mut *mut char name[20]; / name of the device type,
// per-device flags
    pub /: *mut *mut __u8 sanyo_slot : 2; / Sanyo 3 CD changer support,
    pub /: *mut *mut __u8 keeplocked : 1; / CDROM_LOCKDOOR status,
    pub /: *mut *mut __u8 reserved : 5; / not used yet,
    pub /: *mut *mut int cdda_method; / see flags,
    pub last_sense: __u8,
    pub /: *mut *mut __u8 media_written; / dirty flag, DVD+RW bookkeeping,
    pub /: *mut *mut unsigned short mmc3_profile; / current MMC3 profile,
    pub mrw_mode_page: c_int,
    pub opened_for_data: bool,
    pub last_media_change_ms: __s64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdrom_device_ops {
// routines
    pub int): *mut *mut *mut int (open) (struct cdrom_device_info ,,
    pub ): *mut *mut void (release) (struct cdrom_device_info,
    pub int): *mut *mut *mut int (drive_status) (struct cdrom_device_info ,,
    pub slot): unsigned int clearing, int,
    pub int): *mut *mut *mut int (tray_move) (struct cdrom_device_info ,,
    pub int): *mut *mut *mut int (lock_door) (struct cdrom_device_info ,,
    pub long): *mut *mut *mut int (select_speed) (struct cdrom_device_info , unsigned,
    pub ): *mut cdrom_multisession,
    pub ): *mut cdrom_mcn,
// hard reset device
    pub ): *mut *mut int (reset) (struct cdrom_device_info,
// play stuff
    pub ): *mut *mut *mut int (audio_ioctl) (struct cdrom_device_info ,unsigned int, void,
// handle uniform packets for scsi type devices (scsi,atapi)
    pub ): *mut packet_command,
    pub last_sense): *mut u32 lba, u32 nframes, u8,
// driver specifications
    pub /: *const *const int capability; / capability flags,
}

// the general block_device operations structure:
extern "C" {
    pub fn cdrom_open(cdi: *mut cdrom_device_info, mode: blk_mode_t) -> c_int;
}
extern "C" {
    pub fn cdrom_release(cdi: *mut cdrom_device_info);
}
extern "C" {
    pub fn cdrom_probe_write_features(cdi: *mut cdrom_device_info);
}
extern "C" {
    pub fn register_cdrom(disk: *mut gendisk, cdi: *mut cdrom_device_info) -> c_int;
}
extern "C" {
    pub fn unregister_cdrom(cdi: *mut cdrom_device_info);
}
extern "C" {
    pub fn cdrom_get_last_written(cdi: *mut cdrom_device_info, last_written: *mut c_long) -> c_int;
}
extern "C" {
    pub fn cdrom_number_of_slots(cdi: *mut cdrom_device_info) -> c_int;
}
// The SCSI spec says there could be 256 slots.
pub const CDROM_MAX_SLOTS: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdrom_mechstat_header {

    pub 1: __u8 fault :,
    pub 2: __u8 changer_state :,
    pub 5: __u8 curslot :,
    pub 3: __u8 mech_state :,
    pub 1: __u8 door_open :,
    pub 4: __u8 reserved1 :,

    pub 5: __u8 curslot :,
    pub 2: __u8 changer_state :,
    pub 1: __u8 fault :,
    pub 4: __u8 reserved1 :,
    pub 1: __u8 door_open :,
    pub 3: __u8 mech_state :,
    pub curlba: [__u8; 3],
    pub nslots: __u8,
    pub slot_tablelen: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdrom_slot {

    pub 1: __u8 disc_present :,
    pub 6: __u8 reserved1 :,
    pub 1: __u8 change :,

    pub 1: __u8 change :,
    pub 6: __u8 reserved1 :,
    pub 1: __u8 disc_present :,
    pub reserved2: [__u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdrom_changer_info {
    pub hdr: cdrom_mechstat_header,
    pub slots: [cdrom_slot; CDROM_MAX_SLOTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_header {
    pub data_len: __be16,

    pub 1: __u8 nea :,
    pub 4: __u8 reserved1 :,
    pub 3: __u8 notification_class :,

    pub 3: __u8 notification_class :,
    pub 4: __u8 reserved1 :,
    pub 1: __u8 nea :,

    pub supp_event_class: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_event_desc {

    pub 4: __u8 reserved1 :,
    pub 4: __u8 media_event_code :,
    pub 6: __u8 reserved2 :,
    pub 1: __u8 media_present :,
    pub 1: __u8 door_open :,

    pub 4: __u8 media_event_code :,
    pub 4: __u8 reserved1 :,
    pub 1: __u8 door_open :,
    pub 1: __u8 media_present :,
    pub 6: __u8 reserved2 :,

    pub start_slot: __u8,
    pub end_slot: __u8,
}

extern "C" {
    pub fn cdrom_get_media_event(cdi: *mut cdrom_device_info, med: *mut media_event_desc) -> c_int;
}
// m = lba / (CD_SECS * CD_FRAMES);
// s = lba / CD_FRAMES;
// f = lba % CD_FRAMES;
