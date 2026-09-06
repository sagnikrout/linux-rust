//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/sr.h
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
// sr.h by David Giller
// CD-ROM disk driver header file
//
// adapted from:
// sd.h Copyright (C) 1992 Drew Eckhardt
// SCSI disk driver header file by
// Drew Eckhardt
//
// <drew@colorado.edu>
//
// Modified by Eric Youngdale eric@andante.org to
// add scatter-gather, multiple outstanding request, and other
// enhancements.
//

pub const MAX_RETRIES: c_int = 3;

// The CDROM is fairly slow, so we need a little extra time
// In fact, it is very slow if it has to spin up first

// GET_EVENT spurious event handling, blk layer guarantees exclusion

extern "C" {
    pub fn sr_do_ioctl(: *mut Scsi_CD, : *mut packet_command) -> c_int;
}
extern "C" {
    pub fn sr_lock_door(: *mut cdrom_device_info, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn sr_tray_move(: *mut cdrom_device_info, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn sr_drive_status(: *mut cdrom_device_info, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn sr_disk_status(: *mut cdrom_device_info) -> c_int;
}
extern "C" {
    pub fn sr_get_last_session(: *mut cdrom_device_info, : *mut cdrom_multisession) -> c_int;
}
extern "C" {
    pub fn sr_get_mcn(: *mut cdrom_device_info, : *mut cdrom_mcn) -> c_int;
}
extern "C" {
    pub fn sr_reset(: *mut cdrom_device_info) -> c_int;
}
extern "C" {
    pub fn sr_select_speed(cdi: *mut cdrom_device_info, speed: c_ulong) -> c_int;
}
extern "C" {
    pub fn sr_audio_ioctl(: *mut cdrom_device_info, int: unsigned, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn sr_is_xa(: *mut Scsi_CD) -> c_int;
}
// sr_vendor.c
extern "C" {
    pub fn sr_vendor_init(: *mut Scsi_CD);
}
extern "C" {
    pub fn sr_cd_check(: *mut cdrom_device_info) -> c_int;
}
extern "C" {
    pub fn sr_set_blocklength(: *mut Scsi_CD, blocklength: c_int) -> c_int;
}
