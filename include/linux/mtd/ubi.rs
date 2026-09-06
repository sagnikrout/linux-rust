//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/ubi.h
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
// Copyright (c) International Business Machines Corp., 2006
//
// Author: Artem Bityutskiy (Битюцкий Артём)
//

// All voumes/LEBs

//
// Maximum number of scatter gather list entries,
// we use only 64 to have a lower memory foot print.
//
pub const UBI_MAX_SG_COUNT: c_int = 64;
//
// enum ubi_open_mode - UBI volume open mode constants.
//
// UBI_READONLY: read-only mode
// UBI_READWRITE: read-write mode
// UBI_EXCLUSIVE: exclusive mode
// UBI_METAONLY: modify only the volume meta-data,
// i.e. the data stored in the volume table, but not in any of volume LEBs.
//
// struct ubi_volume_info - UBI volume description data structure.
// @vol_id: volume ID
// @ubi_num: UBI device number this volume belongs to
// @size: how many physical eraseblocks are reserved for this volume
// @used_bytes: how many bytes of data this volume contains
// @used_ebs: how many physical eraseblocks of this volume actually contain any
// data
// @vol_type: volume type (%UBI_DYNAMIC_VOLUME or %UBI_STATIC_VOLUME)
// @corrupted: non-zero if the volume is corrupted (static volumes only)
// @upd_marker: non-zero if the volume has update marker set
// @alignment: volume alignment
// @usable_leb_size: how many bytes are available in logical eraseblocks of
// this volume
// @name_len: volume name length
// @name: volume name
// @cdev: UBI volume character device major and minor numbers
//
// The @corrupted flag is only relevant to static volumes and is always zero
// for dynamic ones. This is because UBI does not care about dynamic volume
// data protection and only cares about protecting static volume data.
//
// The @upd_marker flag is set if the volume update operation was interrupted.
// Before touching the volume data during the update operation, UBI first sets
// the update marker flag for this volume. If the volume update operation was
// further interrupted, the update marker indicates this. If the update marker
// is set, the contents of the volume is certainly damaged and a new volume
// update operation has to be started.
//
// To put it differently, @corrupted and @upd_marker fields have different
// semantics:
// o the @corrupted flag means that this static volume is corrupted for some
// reasons, but not because an interrupted volume update
// o the @upd_marker field means that the volume is damaged because of an
// interrupted update operation.
//
// I.e., the @corrupted flag is never set if the @upd_marker flag is set.
//
// The @used_bytes and @used_ebs fields are only really needed for static
// volumes and contain the number of bytes stored in this static volume and how
// many eraseblock this data occupies. In case of dynamic volumes, the
// @used_bytes field is equivalent to @size*@usable_leb_size, and the @used_ebs
// field is equivalent to @size.
//
// In general, logical eraseblock size is a property of the UBI device, not
// of the UBI volume. Indeed, the logical eraseblock size depends on the
// physical eraseblock size and on how much bytes UBI headers consume. But
// because of the volume alignment (@alignment), the usable size of logical
// eraseblocks if a volume may be less. The following equation is true:
// @usable_leb_size = LEB size - (LEB size mod @alignment),
// where LEB size is the logical eraseblock size defined by the UBI device.
//
// The alignment is multiple to the minimal flash input/output unit size or %1
// if all the available space is used.
//
// To put this differently, alignment may be considered is a way to change
// volume logical eraseblock sizes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_volume_info {
    pub ubi_num: c_int,
    pub vol_id: c_int,
    pub size: c_int,
    pub used_bytes: c_longlong,
    pub used_ebs: c_int,
    pub vol_type: c_int,
    pub corrupted: c_int,
    pub upd_marker: c_int,
    pub alignment: c_int,
    pub usable_leb_size: c_int,
    pub name_len: c_int,
    pub name: *const c_char,
    pub cdev: dev_t,
    pub dev: *mut device,
}

//
// struct ubi_sgl - UBI scatter gather list data structure.
// @list_pos: current position in @sg[]
// @page_pos: current position in @sg[@list_pos]
// @sg: the scatter gather list itself
//
// ubi_sgl is a wrapper around a scatter list which keeps track of the
// current position in the list and the current list item such that
// it can be used across multiple ubi_leb_read_sg() calls.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_sgl {
    pub list_pos: c_int,
    pub page_pos: c_int,
    pub sg: [scatterlist; UBI_MAX_SG_COUNT],
}

//
// ubi_sgl_init - initialize an UBI scatter gather list data structure.
// @usgl: the UBI scatter gather struct itself
//
// Please note that you still have to use sg_init_table() or any adequate
// function to initialize the unterlaying struct scatterlist.
//
// struct ubi_device_info - UBI device description data structure.
// @ubi_num: ubi device number
// @leb_size: logical eraseblock size on this UBI device
// @leb_start: starting offset of logical eraseblocks within physical
// eraseblocks
// @min_io_size: minimal I/O unit size
// @max_write_size: maximum amount of bytes the underlying flash can write at a
// time (MTD write buffer size)
// @ro_mode: if this device is in read-only mode
// @cdev: UBI character device major and minor numbers
//
// Note, @leb_size is the logical eraseblock size offered by the UBI device.
// Volumes of this UBI device may have smaller logical eraseblock size if their
// alignment is not equivalent to %1.
//
// The @max_write_size field describes flash write maximum write unit. For
// example, NOR flash allows for changing individual bytes, so @min_io_size is
// %1. However, it does not mean than NOR flash has to write data byte-by-byte.
// Instead, CFI NOR flashes have a write-buffer of, e.g., 64 bytes, and when
// writing large chunks of data, they write 64-bytes at a time. Obviously, this
// improves write throughput.
//
// Also, the MTD device may have N interleaved (striped) flash chips
// underneath, in which case @min_io_size can be physical min. I/O size of
// single flash chip, while @max_write_size can be N * @min_io_size.
//
// The @max_write_size field is always greater or equivalent to @min_io_size.
// E.g., some NOR flashes may have (@min_io_size = 1, @max_write_size = 64). In
// contrast, NAND flashes usually have @min_io_size = @max_write_size = NAND
// page size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_device_info {
    pub ubi_num: c_int,
    pub leb_size: c_int,
    pub leb_start: c_int,
    pub min_io_size: c_int,
    pub max_write_size: c_int,
    pub ro_mode: c_int,
    pub cdev: dev_t,
}

//
// Volume notification types.
// @UBI_VOLUME_ADDED: a volume has been added (an UBI device was attached or a
// volume was created)
// @UBI_VOLUME_REMOVED: a volume has been removed (an UBI device was detached
// or a volume was removed)
// @UBI_VOLUME_RESIZED: a volume has been re-sized
// @UBI_VOLUME_RENAMED: a volume has been re-named
// @UBI_VOLUME_SHUTDOWN: a volume is going to removed, shutdown users
// @UBI_VOLUME_UPDATED: data has been written to a volume
//
// These constants define which type of event has happened when a volume
// notification function is invoked.
//
// struct ubi_notification - UBI notification description structure.
// @di: UBI device description object
// @vi: UBI volume description object
//
// UBI notifiers are called with a pointer to an object of this type. The
// object describes the notification. Namely, it provides a description of the
// UBI device and UBI volume the notification informs about.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_notification {
    pub di: ubi_device_info,
    pub vi: ubi_volume_info,
}

// UBI descriptor given to users when they open UBI volumes
extern "C" {
    pub fn ubi_get_device_info(ubi_num: c_int, di: *mut ubi_device_info) -> c_int;
}
extern "C" {
    pub fn ubi_unregister_volume_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn ubi_close_volume(desc: *mut ubi_volume_desc);
}
extern "C" {
    pub fn ubi_leb_erase(desc: *mut ubi_volume_desc, lnum: c_int) -> c_int;
}
extern "C" {
    pub fn ubi_leb_unmap(desc: *mut ubi_volume_desc, lnum: c_int) -> c_int;
}
extern "C" {
    pub fn ubi_leb_map(desc: *mut ubi_volume_desc, lnum: c_int) -> c_int;
}
extern "C" {
    pub fn ubi_is_mapped(desc: *mut ubi_volume_desc, lnum: c_int) -> c_int;
}
extern "C" {
    pub fn ubi_sync(ubi_num: c_int) -> c_int;
}
//
// This function is the same as the 'ubi_leb_read()' function, but it does not
// provide the checking capability.
//
extern "C" {
    pub fn ubi_leb_read(_arg: desc, _arg: lnum, _arg: buf, _arg: offset, _arg: len, _arg: 0) -> return;
}
//
// This function is the same as the 'ubi_leb_read_sg()' function, but it does
// not provide the checking capability.
//
extern "C" {
    pub fn ubi_leb_read_sg(_arg: desc, _arg: lnum, _arg: sgl, _arg: offset, _arg: len, _arg: 0) -> return;
}
