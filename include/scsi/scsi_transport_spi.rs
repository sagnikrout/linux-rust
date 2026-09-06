//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/scsi_transport_spi.h
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
// Parallel SCSI (SPI) transport specific attributes exported to sysfs.
//
// Copyright (c) 2003 Silicon Graphics, Inc.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_transport_attrs {
    pub /: *mut *mut int period; / value in the PPR/SDTR command,
    pub min_period: c_int,
    pub offset: c_int,
    pub max_offset: c_int,
    pub /: *mut *mut unsigned int width:1; / 0 - narrow, 1 - wide,
    pub max_width:1: c_uint,
    pub /: *mut *mut unsigned int iu:1; / Information Units enabled,
    pub max_iu:1: c_uint,
    pub /: *mut *mut unsigned int dt:1; / DT clocking enabled,
    pub /: *mut *mut unsigned int qas:1; / Quick Arbitration and Selection enabled,
    pub max_qas:1: c_uint,
    pub /: *mut *mut unsigned int wr_flow:1; / Write Flow control enabled,
    pub /: *mut *mut unsigned int rd_strm:1; / Read streaming enabled,
    pub /: *mut *mut unsigned int rti:1; / Retain Training Information,
    pub /: *mut *mut unsigned int pcomp_en:1;/ Precompensation enabled,
    pub /: *mut *mut unsigned int hold_mcs:1;/ Hold Margin Control Settings,
    pub /: *mut *mut unsigned int initial_dv:1; / DV done to this target yet,
    pub /: *mut *mut unsigned long flags; / flags field for drivers to use,
// Device Properties fields
    pub /: *mut *mut unsigned int support_sync:1; / synchronous support,
    pub /: *mut *mut unsigned int support_wide:1; / wide support,
    pub /: *mut *mut unsigned int support_dt:1; / allows DT phases,
    pub /: *mut *mut unsigned int support_dt_only; / disallows ST phases,
    pub /: *mut *mut unsigned int support_ius; / support Information Units,
    pub /: *mut *mut unsigned int support_qas; / supports quick arbitration and selection,
// Private Fields
    pub /: *mut *mut unsigned int dv_pending:1; / Internal flag: DV Requested,
    pub /: *mut *mut unsigned int dv_in_progress:1; / Internal: DV started,
    pub /: *mut *mut mutex dv_mutex; / semaphore to serialise dv,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spi_signal_type {
    SPI_SIGNAL_UNKNOWN = 1,
    SPI_SIGNAL_SE,
    SPI_SIGNAL_LVD,
    SPI_SIGNAL_HVD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_host_attrs {
    pub signalling: spi_signal_type,
}

// accessor functions

// The functions by which the transport class and the driver communicate
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_function_template {
    pub ): *mut *mut void (get_period)(struct scsi_target,
    pub int): *mut *mut *mut void (set_period)(struct scsi_target ,,
    pub ): *mut *mut void (get_offset)(struct scsi_target,
    pub int): *mut *mut *mut void (set_offset)(struct scsi_target ,,
    pub ): *mut *mut void (get_width)(struct scsi_target,
    pub int): *mut *mut *mut void (set_width)(struct scsi_target ,,
    pub ): *mut *mut void (get_iu)(struct scsi_target,
    pub int): *mut *mut *mut void (set_iu)(struct scsi_target ,,
    pub ): *mut *mut void (get_dt)(struct scsi_target,
    pub int): *mut *mut *mut void (set_dt)(struct scsi_target ,,
    pub ): *mut *mut void (get_qas)(struct scsi_target,
    pub int): *mut *mut *mut void (set_qas)(struct scsi_target ,,
    pub ): *mut *mut void (get_wr_flow)(struct scsi_target,
    pub int): *mut *mut *mut void (set_wr_flow)(struct scsi_target ,,
    pub ): *mut *mut void (get_rd_strm)(struct scsi_target,
    pub int): *mut *mut *mut void (set_rd_strm)(struct scsi_target ,,
    pub ): *mut *mut void (get_rti)(struct scsi_target,
    pub int): *mut *mut *mut void (set_rti)(struct scsi_target ,,
    pub ): *mut *mut void (get_pcomp_en)(struct scsi_target,
    pub int): *mut *mut *mut void (set_pcomp_en)(struct scsi_target ,,
    pub ): *mut *mut void (get_hold_mcs)(struct scsi_target,
    pub int): *mut *mut *mut void (set_hold_mcs)(struct scsi_target ,,
    pub ): *mut *mut void (get_signalling)(struct Scsi_Host,
    pub spi_signal_type): *mut *mut *mut void (set_signalling)(struct Scsi_Host , enum,
    pub ): *mut *mut int (deny_binding)(struct scsi_target,
// The driver sets these to tell the transport class it
// wants the attributes displayed in sysfs.  If the show_ flag
// is not set, the attribute will be private to the transport
// class
    pub show_period:1: c_ulong,
    pub show_offset:1: c_ulong,
    pub show_width:1: c_ulong,
    pub show_iu:1: c_ulong,
    pub show_dt:1: c_ulong,
    pub show_qas:1: c_ulong,
    pub show_wr_flow:1: c_ulong,
    pub show_rd_strm:1: c_ulong,
    pub show_rti:1: c_ulong,
    pub show_pcomp_en:1: c_ulong,
    pub show_hold_mcs:1: c_ulong,
}

extern "C" {
    pub fn spi_release_transport(: *mut scsi_transport_template);
}
extern "C" {
    pub fn spi_schedule_dv_device(: *mut scsi_device);
}
extern "C" {
    pub fn spi_dv_device(: *mut scsi_device);
}
extern "C" {
    pub fn spi_display_xfer_agreement(: *mut scsi_target);
}
extern "C" {
    pub fn spi_print_msg(: *const c_uchar) -> c_int;
}
extern "C" {
    pub fn spi_populate_width_msg(msg: *mut c_uchar, width: c_int) -> c_int;
}
extern "C" {
    pub fn spi_populate_sync_msg(msg: *mut c_uchar, period: c_int, offset: c_int) -> c_int;
}
extern "C" {
    pub fn spi_populate_tag_msg(msg: *mut c_uchar, cmd: *mut scsi_cmnd) -> c_int;
}
