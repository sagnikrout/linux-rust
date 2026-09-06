//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/misc/sisusbvga/sisusb.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// sisusb - usb kernel driver for Net2280/SiS315 based USB2VGA dongles
//
// Copyright (C) 2005 by Thomas Winischhofer, Vienna, Austria
//
// If distributed as part of the Linux kernel, this code is licensed under the
// terms of the GPL v2.
//
// Otherwise, the following license terms apply:
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1) Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2) Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3) The name of the author may not be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESSED OR
// IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
// OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
// IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
// INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
// NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
// THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Author:	Thomas Winischhofer <thomas@winischhofer.net>
//

// Version Information
pub const SISUSB_VERSION: c_int = 0;
pub const SISUSB_REVISION: c_int = 0;
pub const SISUSB_PATCHLEVEL: c_int = 8;
// Include console and mode switching code?

// USB related

// Size of the sisusb input/output buffers
pub const SISUSB_IBUF_SIZE: c_uint = 0x01000;
pub const SISUSB_OBUF_SIZE: c_uint = 0x10000	/* fixed */;

// About endianness:
//
// 1) I/O ports, PCI config registers. The read/write()
// calls emulate inX/outX. Hence, the data is
// expected/delivered in machine endiannes by this
// driver.
// 2) Video memory. The data is copied 1:1. There is
// no swapping. Ever. This means for userland that
// the data has to be prepared properly. (Hint:
// think graphics data format, command queue,
// hardware cursor.)
// 3) MMIO. Data is copied 1:1. MMIO must be swapped
// properly by userland.
//

// Macro flag: #define SISUSB_CORRECT_ENDIANNESS_PACKET(p)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sisusb_urb_context {
    pub sisusb: *mut sisusb_usb_data,
    pub urbindex: c_int,
    pub actual_length: *mut c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sisusb_usb_data {
    pub sisusb_dev: *mut usb_device,
    pub interface: *mut usb_interface,
    pub kref: kref,
    pub /: *mut *mut wait_queue_head_t wait_q; / for syncind and timeouts,
    pub /: *mut *mut mutex lock; / general race avoidance,
    pub /: *mut *mut unsigned int ifnum; / interface number of the USB device,
    pub /: *mut *mut int minor; / minor (for logging clarity),
    pub /: *mut *mut int isopen; / !=0 if open,
    pub /: *mut *mut int present; / !=0 if device is present on the bus,
    pub /: *mut *mut int ready; / !=0 if device is ready for userland,
    pub /: *mut *mut int numobufs; / number of obufs = number of out urbs,
    pub /: *mut *mut *mut *mut char obuf[NUMOBUFS], ibuf; / transfer buffers,
    pub ibufsize: int obufsize,,
    pub sisurbout: [*mut urb; NUMOBUFS],
    pub sisurbin: *mut urb,
    pub urbstatus: [c_uchar; NUMOBUFS],
    pub completein: c_uchar,
    pub urbout_context: [sisusb_urb_context; NUMOBUFS],
    pub flagb0: c_ulong,
    pub /: *mut *mut unsigned long vrambase; / framebuffer base,
    pub /: *mut *mut unsigned int vramsize; / framebuffer size (bytes),
    pub mmiobase: c_ulong,
    pub mmiosize: c_uint,
    pub ioportbase: c_ulong,
    pub /: *mut *mut unsigned char devinit; / device initialized?,
    pub /: *mut *mut unsigned char gfxinit; / graphics core initialized?,
    pub chipvendor: unsigned short chipid,,
    pub chiprevision: c_ushort,
}

// USB transport related
// urbstatus
pub const SU_URB_BUSY: c_int = 1;
pub const SU_URB_ALLOC: c_int = 2;
// Endpoints
pub const SISUSB_EP_GFX_IN: c_uint = 0x0e	/* gfx std packet out(0e)/in(8e) */;
pub const SISUSB_EP_GFX_OUT: c_uint = 0x0e;
pub const SISUSB_EP_GFX_BULK_OUT: c_uint = 0x01	/* gfx mem bulk out/in */;
pub const SISUSB_EP_GFX_BULK_IN: c_uint = 0x02	/* ? 2 is "OUT" ? */;
pub const SISUSB_EP_GFX_LBULK_OUT: c_uint = 0x03	/* gfx large mem bulk out */;
pub const SISUSB_EP_UNKNOWN_04: c_uint = 0x04	/* ? 4 is "OUT" ? - unused */;
pub const SISUSB_EP_BRIDGE_IN: c_uint = 0x0d	/* Net2280 out(0d)/in(8d) */;
pub const SISUSB_EP_BRIDGE_OUT: c_uint = 0x0d;
pub const SISUSB_TYPE_MEM: c_int = 0;
pub const SISUSB_TYPE_IO: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sisusb_packet {
    pub header: c_ushort,
    pub address: u32,
    pub data: u32,
// C attribute field omitted

// PCI bridge related
pub const SISUSB_PCI_MEMBASE: c_uint = 0xd0000000;
pub const SISUSB_PCI_MMIOBASE: c_uint = 0xe4000000;
pub const SISUSB_PCI_IOPORTBASE: c_uint = 0x0000d000;
pub const SISUSB_PCI_PSEUDO_MEMBASE: c_uint = 0x10000000;
pub const SISUSB_PCI_PSEUDO_MMIOBASE: c_uint = 0x20000000;
pub const SISUSB_PCI_PSEUDO_IOPORTBASE: c_uint = 0x0000d000;
pub const SISUSB_PCI_PSEUDO_PCIBASE: c_uint = 0x00010000;

pub const SISUSB_PCI_PCONFSIZE: c_uint = 0x5c;
// graphics core related
pub const AROFFSET: c_uint = 0x40;
pub const ARROFFSET: c_uint = 0x41;
pub const GROFFSET: c_uint = 0x4e;
pub const SROFFSET: c_uint = 0x44;
pub const CROFFSET: c_uint = 0x54;
pub const MISCROFFSET: c_uint = 0x4c;
pub const MISCWOFFSET: c_uint = 0x42;
pub const INPUTSTATOFFSET: c_uint = 0x5A;
pub const PART1OFFSET: c_uint = 0x04;
pub const PART2OFFSET: c_uint = 0x10;
pub const PART3OFFSET: c_uint = 0x12;
pub const PART4OFFSET: c_uint = 0x14;
pub const PART5OFFSET: c_uint = 0x16;
pub const CAPTUREOFFSET: c_uint = 0x00;
pub const VIDEOOFFSET: c_uint = 0x02;
pub const COLREGOFFSET: c_uint = 0x48;
pub const PELMASKOFFSET: c_uint = 0x46;
pub const VGAENABLE: c_uint = 0x43;

// ioctl related
// Structure argument for SISUSB_GET_INFO ioctl
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sisusb_info {
    pub /: *mut *mut __u32 sisusb_id; / for identifying sisusb,
pub const SISUSB_ID: c_uint = 0x53495355	/* Identify myself with 'SISU' */;
    pub sisusb_version: __u8,
    pub sisusb_revision: __u8,
    pub sisusb_patchlevel: __u8,
    pub /: *mut *mut __u8 sisusb_gfxinit; / graphics core initialized?,
    pub sisusb_vrambase: __u32,
    pub sisusb_mmiobase: __u32,
    pub sisusb_iobase: __u32,
    pub sisusb_pcibase: __u32,
    pub /: *mut *mut __u32 sisusb_vramsize; / framebuffer size in bytes,
    pub sisusb_minor: __u32,
    pub /: *mut *mut __u32 sisusb_fbdevactive; / != 0 if framebuffer device active,
    pub /: *mut *mut __u32 sisusb_conactive; / != 0 if console driver active,
    pub /: *mut *mut __u8 sisusb_reserved[28]; / for future use,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sisusb_command {
    pub /: *mut *mut __u8 operation; / see below,
    pub /: *mut *mut __u8 data0; / operation dependent,
    pub /: *mut *mut __u8 data1; / operation dependent,
    pub /: *mut *mut __u8 data2; / operation dependent,
    pub /: *mut *mut __u32 data3; / operation dependent,
    pub /: *mut *mut __u32 data4; / for future use,
}

pub const SUCMD_GET: c_uint = 0x01	/* for all: data0 = index, data3 = port */;
pub const SUCMD_SET: c_uint = 0x02	/* data1 = value */;
pub const SUCMD_SETOR: c_uint = 0x03	/* data1 = or */;
pub const SUCMD_SETAND: c_uint = 0x04	/* data1 = and */;
pub const SUCMD_SETANDOR: c_uint = 0x05	/* data1 = and, data2 = or */;
pub const SUCMD_SETMASK: c_uint = 0x06	/* data1 = data, data2 = mask */;
pub const SUCMD_CLRSCR: c_uint = 0x07	/* data0:1:2 = length, data3 = address */;
pub const SUCMD_HANDLETEXTMODE: c_uint = 0x08	/* Reset/destroy text mode */;
pub const SUCMD_SETMODE: c_uint = 0x09	/* Set a display mode (data3 = SiS mode) */;
pub const SUCMD_SETVESAMODE: c_uint = 0x0a	/* Set a display mode (data3 = VESA mode) */;

