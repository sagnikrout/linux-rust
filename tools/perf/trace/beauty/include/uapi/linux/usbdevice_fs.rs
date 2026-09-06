//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/trace/beauty/include/uapi/linux/usbdevice_fs.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// usbdevice_fs.h  --  USB device file system.
//
// Copyright (C) 2000
// Thomas Sailer (sailer@ife.ee.ethz.ch)
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
//
// History:
// 0.1  04.01.2000  Created
//

// ---------------------------------------------------------------------
// usbdevfs ioctl codes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbdevfs_ctrltransfer {
    pub bRequestType: __u8,
    pub bRequest: __u8,
    pub wValue: __u16,
    pub wIndex: __u16,
    pub wLength: __u16,
    pub /: *mut *mut __u32 timeout; / in milliseconds,
    pub data: *mut void __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbdevfs_bulktransfer {
    pub ep: c_uint,
    pub len: c_uint,
    pub /: *mut *mut unsigned int timeout; / in milliseconds,
    pub data: *mut void __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbdevfs_setinterface {
    pub interface: c_uint,
    pub altsetting: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbdevfs_disconnectsignal {
    pub signr: c_uint,
    pub context: *mut void __user,
}

pub const USBDEVFS_MAXDRIVERNAME: c_int = 255;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbdevfs_getdriver {
    pub interface: c_uint,
    pub 1]: char driver[USBDEVFS_MAXDRIVERNAME +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbdevfs_connectinfo {
    pub devnum: c_uint,
    pub slow: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbdevfs_conninfo_ex {
    pub /: *mut *mut __u32 size; / Size of the structure from the kernel's,
// point of view. Can be used by userspace
// to determine how much data can be
// used/trusted.
    pub /: *mut *mut __u32 busnum; / USB bus number, as enumerated by the,
// kernel, the device is connected to.
    pub /: *mut *mut __u32 devnum; / Device address on the bus.,
    pub /: *mut *mut *mut __u32 speed; / USB_SPEED_ constants from ch9.h,
    pub /: *mut *mut __u8 num_ports; / Number of ports the device is connected,
// to on the way to the root hub. It may
// be bigger than size of 'ports' array so
// userspace can detect overflows.
    pub /: *mut *mut __u8 ports[7]; / List of ports on the way from the root,
// hub to the device. Current limit in
// USB specification is 7 tiers (root hub,
// 5 intermediate hubs, device), which
// gives at most 6 port entries.
}

pub const USBDEVFS_URB_SHORT_NOT_OK: c_uint = 0x01;
pub const USBDEVFS_URB_ISO_ASAP: c_uint = 0x02;
pub const USBDEVFS_URB_BULK_CONTINUATION: c_uint = 0x04;
pub const USBDEVFS_URB_NO_FSBR: c_uint = 0x20	/* Not used */;
pub const USBDEVFS_URB_ZERO_PACKET: c_uint = 0x40;
pub const USBDEVFS_URB_NO_INTERRUPT: c_uint = 0x80;
pub const USBDEVFS_URB_TYPE_ISO: c_int = 0;
pub const USBDEVFS_URB_TYPE_INTERRUPT: c_int = 1;
pub const USBDEVFS_URB_TYPE_CONTROL: c_int = 2;
pub const USBDEVFS_URB_TYPE_BULK: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbdevfs_iso_packet_desc {
    pub length: c_uint,
    pub actual_length: c_uint,
    pub status: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbdevfs_urb {
    pub type: c_uchar,
    pub endpoint: c_uchar,
    pub status: c_int,
    pub flags: c_uint,
    pub buffer: *mut void __user,
    pub buffer_length: c_int,
    pub actual_length: c_int,
    pub start_frame: c_int,
    pub /: *mut *mut int number_of_packets; / Only used for isoc urbs,
    pub /: *mut *mut unsigned int stream_id; / Only used with bulk streams,
}

// ioctls for talking directly to drivers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbdevfs_ioctl {
    pub /: *mut *mut int ifno; / interface 0..N ; negative numbers reserved,
    pub the: *mut *mut int ioctl_code; / MUST encode size + direction of data so,
// macros in <asm/ioctl.h> give correct values
    pub /: *mut *mut *mut void __user data; / param buffer (in, or out),
}

// You can do most things with hubs just through control messages,
// except find out what device connects to what port.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbdevfs_hub_portinfo {
    pub /: *mut *mut char nports; / number of downstream ports in this hub,
    pub /: *mut *mut char port [127]; / e.g. port 3 connects to device 27,
}

// System and bus capability flags
pub const USBDEVFS_CAP_ZERO_PACKET: c_uint = 0x01;
pub const USBDEVFS_CAP_BULK_CONTINUATION: c_uint = 0x02;
pub const USBDEVFS_CAP_NO_PACKET_SIZE_LIM: c_uint = 0x04;
pub const USBDEVFS_CAP_BULK_SCATTER_GATHER: c_uint = 0x08;
pub const USBDEVFS_CAP_REAP_AFTER_DISCONNECT: c_uint = 0x10;
pub const USBDEVFS_CAP_MMAP: c_uint = 0x20;
pub const USBDEVFS_CAP_DROP_PRIVILEGES: c_uint = 0x40;
pub const USBDEVFS_CAP_CONNINFO_EX: c_uint = 0x80;
pub const USBDEVFS_CAP_SUSPEND: c_uint = 0x100;
// USBDEVFS_DISCONNECT_CLAIM flags & struct
// disconnect-and-claim if the driver matches the driver field
pub const USBDEVFS_DISCONNECT_CLAIM_IF_DRIVER: c_uint = 0x01;
// disconnect-and-claim except when the driver matches the driver field
pub const USBDEVFS_DISCONNECT_CLAIM_EXCEPT_DRIVER: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbdevfs_disconnect_claim {
    pub interface: c_uint,
    pub flags: c_uint,
    pub 1]: char driver[USBDEVFS_MAXDRIVERNAME +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbdevfs_streams {
    pub /: *mut *mut unsigned int num_streams; / Not used by USBDEVFS_FREE_STREAMS,
    pub num_eps: c_uint,
    pub eps: [c_uchar; ],
}

//
// USB_SPEED_* values returned by USBDEVFS_GET_SPEED are defined in
// linux/usb/ch9.h
//

//
// Returns struct usbdevfs_conninfo_ex; length is variable to allow
// extending size of the data returned.
//

