//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/dvb_net.h
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
// dvb_net.h
//
// Copyright (C) 2001 Ralph Metzler for convergence integrated media GmbH
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation; either version 2.1
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

pub const DVB_NET_DEVICES_MAX: c_int = 10;

//
// struct dvb_net - describes a DVB network interface
//
// @dvbdev:		pointer to &struct dvb_device.
// @device:		array of pointers to &struct net_device.
// @state:		array of integers to each net device. A value
// different than zero means that the interface is
// in usage.
// @exit:		flag to indicate when the device is being removed.
// @demux:		pointer to &struct dmx_demux.
// @ioctl_mutex:	protect access to this struct.
// @remove_mutex:	mutex that avoids a race condition between a callback
// called when the hardware is disconnected and the
// file_operations of dvb_net.
//
// Currently, the core supports up to %DVB_NET_DEVICES_MAX (10) network
// devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_net {
    pub dvbdev: *mut dvb_device,
    pub device: [*mut net_device; DVB_NET_DEVICES_MAX],
    pub state: [c_int; DVB_NET_DEVICES_MAX],
    pub exit:1: c_uint,
    pub demux: *mut dmx_demux,
    pub ioctl_mutex: mutex,
    pub remove_mutex: mutex,
}

//
// dvb_net_init - nitializes a digital TV network device and registers it.
//
// @adap:	pointer to &struct dvb_adapter.
// @dvbnet:	pointer to &struct dvb_net.
// @dmxdemux:	pointer to &struct dmx_demux.
//
// dvb_net_release - releases a digital TV network device and unregisters it.
//
// @dvbnet:	pointer to &struct dvb_net.
//
extern "C" {
    pub fn dvb_net_release(dvbnet: *mut dvb_net);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_net {
    pub dvbdev: *mut dvb_device,
}

