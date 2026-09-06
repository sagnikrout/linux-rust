//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/dvb_ca_en50221.h
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
// dvb_ca.h: generic DVB functions for EN50221 CA interfaces
//
// Copyright (C) 2004 Andrew de Quincey
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation; either version 2.1
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the
// GNU General Public License for more details.
//

pub const DVB_CA_EN50221_POLL_CAM_PRESENT: c_int = 1;
pub const DVB_CA_EN50221_POLL_CAM_CHANGED: c_int = 2;
pub const DVB_CA_EN50221_POLL_CAM_READY: c_int = 4;
pub const DVB_CA_EN50221_FLAG_IRQ_CAMCHANGE: c_int = 1;
pub const DVB_CA_EN50221_FLAG_IRQ_FR: c_int = 2;
pub const DVB_CA_EN50221_FLAG_IRQ_DA: c_int = 4;
pub const DVB_CA_EN50221_CAMCHANGE_REMOVED: c_int = 0;
pub const DVB_CA_EN50221_CAMCHANGE_INSERTED: c_int = 1;
//
// struct dvb_ca_en50221- Structure describing a CA interface
//
// @owner:		the module owning this structure
// @read_attribute_mem:	function for reading attribute memory on the CAM
// @write_attribute_mem: function for writing attribute memory on the CAM
// @read_cam_control:	function for reading the control interface on the CAM
// @write_cam_control:	function for reading the control interface on the CAM
// @read_data:		function for reading data (block mode)
// @write_data:		function for writing data (block mode)
// @slot_reset:		function to reset the CAM slot
// @slot_shutdown:	function to shutdown a CAM slot
// @slot_ts_enable:	function to enable the Transport Stream on a CAM slot
// @poll_slot_status:	function to poll slot status. Only necessary if
// DVB_CA_FLAG_EN50221_IRQ_CAMCHANGE is not set.
// @data:		private data, used by caller.
// @private:		Opaque data used by the dvb_ca core. Do not modify!
//
// NOTE: the read_*, write_* and poll_slot_status functions will be
// called for different slots concurrently and need to use locks where
// and if appropriate. There will be no concurrent access to one slot.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_ca_en50221 {
    pub owner: *mut module,
    pub address): int slot, int,
    pub value): int slot, int address, u8,
    pub address): int slot, u8,
    pub value): int slot, u8 address, u8,
    pub ecount): *mut *mut int slot, u8 ebuf, int,
    pub ecount): *mut *mut int slot, u8 ebuf, int,
    pub slot): *mut *mut *mut int (slot_reset)(struct dvb_ca_en50221 ca, int,
    pub slot): *mut *mut *mut int (slot_shutdown)(struct dvb_ca_en50221 ca, int,
    pub slot): *mut *mut *mut int (slot_ts_enable)(struct dvb_ca_en50221 ca, int,
    pub open): *mut *mut *mut int (poll_slot_status)(struct dvb_ca_en50221 ca, int slot, int,
    pub data: *mut c_void,
    pub private: *mut c_void,
}

//
// Functions for reporting IRQ events
//
// dvb_ca_en50221_camchange_irq - A CAMCHANGE IRQ has occurred.
//
// @pubca: CA instance.
// @slot: Slot concerned.
// @change_type: One of the DVB_CA_CAMCHANGE_* values
//
// dvb_ca_en50221_camready_irq - A CAMREADY IRQ has occurred.
//
// @pubca: CA instance.
// @slot: Slot concerned.
//
extern "C" {
    pub fn dvb_ca_en50221_camready_irq(pubca: *mut dvb_ca_en50221, slot: c_int);
}
//
// dvb_ca_en50221_frda_irq - An FR or a DA IRQ has occurred.
//
// @ca: CA instance.
// @slot: Slot concerned.
//
extern "C" {
    pub fn dvb_ca_en50221_frda_irq(ca: *mut dvb_ca_en50221, slot: c_int);
}
//
// Initialisation/shutdown functions
//
// dvb_ca_en50221_init - Initialise a new DVB CA device.
//
// @dvb_adapter: DVB adapter to attach the new CA device to.
// @ca: The dvb_ca instance.
// @flags: Flags describing the CA device (DVB_CA_EN50221_FLAG_*).
// @slot_count: Number of slots supported.
//
// @return 0 on success, nonzero on failure
//
// dvb_ca_en50221_release - Release a DVB CA device.
//
// @ca: The associated dvb_ca instance.
//
extern "C" {
    pub fn dvb_ca_en50221_release(ca: *mut dvb_ca_en50221);
}
