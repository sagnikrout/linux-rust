//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/media.h
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
// Multimedia device API
//
// Copyright (C) 2010 Nokia Corporation
//
// Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_device_info {
    pub driver: [c_char; 16],
    pub model: [c_char; 32],
    pub serial: [c_char; 40],
    pub bus_info: [c_char; 32],
    pub media_version: __u32,
    pub hw_revision: __u32,
    pub driver_version: __u32,
    pub reserved: [__u32; 31],
}

//
// Base number ranges for entity functions
//
// NOTE: Userspace should not rely on these ranges to identify a group
// of function types, as newer functions can be added with any name within
// the full u32 range.
//
// Some older functions use the MEDIA_ENT_F_OLD_*_BASE range. Do not
// change this, this is for backwards compatibility. When adding new
// functions always use MEDIA_ENT_F_BASE.
//
pub const MEDIA_ENT_F_BASE: c_uint = 0x00000000;
pub const MEDIA_ENT_F_OLD_BASE: c_uint = 0x00010000;
pub const MEDIA_ENT_F_OLD_SUBDEV_BASE: c_uint = 0x00020000;
//
// Initial value to be used when a new entity is created
// Drivers should change it to something useful.
//

//
// Subdevs are initialized with MEDIA_ENT_F_V4L2_SUBDEV_UNKNOWN in order
// to preserve backward compatibility. Drivers must change to the proper
// subdev type before registering the entity.
//

//
// DVB entity functions
//

//
// I/O entity functions
//

//
// Sensor functions
//

//
// Digital TV, analog TV, radio and/or software defined radio tuner functions.
//
// It is a responsibility of the master/bridge drivers to add connectors
// and links for MEDIA_ENT_F_TUNER. Please notice that some old tuners
// may require the usage of separate I2C chips to decode analog TV signals,
// when the master/bridge chipset doesn't have its own TV standard decoder.
// On such cases, the IF-PLL staging is mapped via one or two entities:
// MEDIA_ENT_F_IF_VID_DECODER and/or MEDIA_ENT_F_IF_AUD_DECODER.
//

//
// Analog TV IF-PLL decoder functions
//
// It is a responsibility of the master/bridge drivers to create links
// for MEDIA_ENT_F_IF_VID_DECODER and MEDIA_ENT_F_IF_AUD_DECODER.
//

//
// Audio entity functions
//

//
// Processing entity functions
//

//
// Switch and bridge entity functions
//

//
// Video decoder/encoder functions
//

// Entity flags

// OR with the entity id value to find the next entity

#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_entity_desc {
    pub id: __u32,
    pub name: [c_char; 32],
    pub type: __u32,
    pub revision: __u32,
    pub flags: __u32,
    pub group_id: __u32,
    pub pads: __u16,
    pub links: __u16,
    pub reserved: [__u32; 4],
// Node specifications
    pub major: __u32,
    pub minor: __u32,
    pub dev: },

//
// TODO: this shouldn't have been added without
// actual drivers that use this. When the first real driver
// appears that sets this information, special attention
// should be given whether this information is 1) enough, and
// 2) can deal with udev rules that rename devices. The struct
// dev would not be sufficient for this since that does not
// contain the subdevice information. In addition, struct dev
// can only refer to a single device, and not to multiple (e.g.
// pcm and mixer devices).
//
    pub card: __u32,
    pub device: __u32,
    pub subdevice: __u32,
    pub alsa: },
//
// DEPRECATED: previous node specifications. Kept just to
// avoid breaking compilation. Use media_entity_desc.dev
// instead.
//
    pub major: __u32,
    pub minor: __u32,
    pub v4l: },
    pub major: __u32,
    pub minor: __u32,
    pub fb: },
    pub dvb: c_int,

// Sub-device specifications
// Nothing needed yet
    pub raw: [__u8; 184],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_pad_desc {
    pub /: *mut *mut __u32 entity; / entity ID,
    pub /: *mut *mut __u16 index; / pad index,
    pub /: *mut *mut __u32 flags; / pad flags,
    pub reserved: [__u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_link_desc {
    pub source: media_pad_desc,
    pub sink: media_pad_desc,
    pub flags: __u32,
    pub reserved: [__u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_links_enum {
    pub entity: __u32,
// Should have enough room for pads elements
    pub pads: *mut media_pad_desc __user,
// Should have enough room for links elements
    pub links: *mut media_link_desc __user,
    pub reserved: [__u32; 4],
}

// Interface type ranges
pub const MEDIA_INTF_T_DVB_BASE: c_uint = 0x00000100;
pub const MEDIA_INTF_T_V4L_BASE: c_uint = 0x00000200;
// Interface types

pub const MEDIA_INTF_T_ALSA_BASE: c_uint = 0x00000300;

//
// Connector functions
//
// For now these should not be used in userspace, as some definitions may
// change.
//
// It is the responsibility of the entity drivers to add connectors and links.
//

//
// MC next gen API definitions
//
// Appeared in 4.19.0.
//
// The media_version argument comes from the media_version field in
// struct media_device_info.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_v2_entity {
    pub id: __u32,
    pub name: [c_char; 64],
    pub /: *mut *mut __u32 function; / Main function of the entity,
    pub flags: __u32,
    pub reserved: [__u32; 5],
// C attribute field omitted
// Should match the specific fields at media_intf_devnode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_v2_intf_devnode {
    pub major: __u32,
    pub minor: __u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_v2_interface {
    pub id: __u32,
    pub intf_type: __u32,
    pub flags: __u32,
    pub reserved: [__u32; 9],
    pub devnode: media_v2_intf_devnode,
    pub raw: [__u32; 16],
}

//
// Appeared in 4.19.0.
//
// The media_version argument comes from the media_version field in
// struct media_device_info.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_v2_pad {
    pub id: __u32,
    pub entity_id: __u32,
    pub flags: __u32,
    pub index: __u32,
    pub reserved: [__u32; 4],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_v2_link {
    pub id: __u32,
    pub source_id: __u32,
    pub sink_id: __u32,
    pub flags: __u32,
    pub reserved: [__u32; 6],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_v2_topology {
    pub topology_version: __u64,
    pub num_entities: __u32,
    pub reserved1: __u32,
    pub ptr_entities: __u64,
    pub num_interfaces: __u32,
    pub reserved2: __u32,
    pub ptr_interfaces: __u64,
    pub num_pads: __u32,
    pub reserved3: __u32,
    pub ptr_pads: __u64,
    pub num_links: __u32,
    pub reserved4: __u32,
    pub ptr_links: __u64,
// C attribute field omitted
// ioctls

//
// These ioctls are called on the request file descriptor as returned
// by MEDIA_IOC_REQUEST_ALLOC.
//

//
// Legacy symbols used to avoid userspace compilation breakages.
// Do not use any of this in new applications!
//
// Those symbols map the entity function into types and should be
// used only on legacy programs for legacy hardware. Don't rely
// on those for MEDIA_IOC_G_TOPOLOGY.
//
pub const MEDIA_ENT_TYPE_SHIFT: c_int = 16;
pub const MEDIA_ENT_TYPE_MASK: c_uint = 0x00ff0000;
pub const MEDIA_ENT_SUBTYPE_MASK: c_uint = 0x0000ffff;

//
// There is still no full ALSA support in the media controller. These
// defines should not have been added and we leave them here only
// in case some application tries to use these defines.
//
// The ALSA defines that are in use have been moved into __KERNEL__
// scope. As support gets added to these interface types, they should
// be moved into __KERNEL__ scope with the code that uses them.
//

// Obsolete symbol for media_version, no longer used in the kernel

