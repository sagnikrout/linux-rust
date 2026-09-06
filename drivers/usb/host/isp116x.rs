//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/host/isp116x.h
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
// ISP116x register declarations and HCD data structures
//
// Copyright (C) 2005 Olav Kongas <ok@artecdesign.ee>
// Portions:
// Copyright (C) 2004 Lothar Wassmann
// Copyright (C) 2004 Psion Teklogix
// Copyright (C) 2004 David Brownell
//
// us of 1ms frame
pub const MAX_LOAD_LIMIT: c_int = 850;
// Full speed: max # of bytes to transfer for a single urb
pub const MAX_TRANSFER_SIZE_FULLSPEED: c_int = 832;
// Low speed: there is no reason to schedule in very big
pub const MAX_TRANSFER_SIZE_LOWSPEED: c_int = 64;
// Bytetime (us), a rough indication of how much time it
pub const BYTE_TIME_FULLSPEED: c_int = 1;
pub const BYTE_TIME_LOWSPEED: c_int = 20;
// Buffer sizes
pub const ISP116x_BUF_SIZE: c_int = 4096;
pub const ISP116x_ITL_BUFSIZE: c_int = 0;

pub const ISP116x_WRITE_OFFSET: c_uint = 0x80;
// ------------ ISP116x registers/bits ------------
pub const HCREVISION: c_uint = 0x00;
pub const HCCONTROL: c_uint = 0x01;

pub const HCCMDSTAT: c_uint = 0x02;

pub const HCINTSTAT: c_uint = 0x03;

pub const HCINTENB: c_uint = 0x04;
pub const HCINTDIS: c_uint = 0x05;
pub const HCFMINTVL: c_uint = 0x0d;
pub const HCFMREM: c_uint = 0x0e;
pub const HCFMNUM: c_uint = 0x0f;
pub const HCLSTHRESH: c_uint = 0x11;
pub const HCRHDESCA: c_uint = 0x12;

pub const HCRHDESCB: c_uint = 0x13;

pub const HCRHSTATUS: c_uint = 0x14;

pub const HCRHPORT1: c_uint = 0x15;

pub const HCRHPORT2: c_uint = 0x16;
pub const HCHWCFG: c_uint = 0x20;

pub const HCDMACFG: c_uint = 0x21;

pub const HCXFERCTR: c_uint = 0x22;
pub const HCuPINT: c_uint = 0x24;

pub const HCuPINTENB: c_uint = 0x25;
pub const HCCHIPID: c_uint = 0x27;
pub const HCCHIPID_MASK: c_uint = 0xff00;
pub const HCCHIPID_MAGIC: c_uint = 0x6100;
pub const HCSCRATCH: c_uint = 0x28;
pub const HCSWRES: c_uint = 0x29;
pub const HCSWRES_MAGIC: c_uint = 0x00f6;
pub const HCITLBUFLEN: c_uint = 0x2a;
pub const HCATLBUFLEN: c_uint = 0x2b;
pub const HCBUFSTAT: c_uint = 0x2c;

pub const HCRDITL0LEN: c_uint = 0x2d;
pub const HCRDITL1LEN: c_uint = 0x2e;
pub const HCITLPORT: c_uint = 0x40;
pub const HCATLPORT: c_uint = 0x41;
// Philips transfer descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptd {
    pub count: u16,

    pub mps: u16,

    pub len: u16,

    pub faddr: u16,
// C attribute field omitted
// PTD accessor macros.

// Hardware transfer status codes -- CC from ptd->count
pub const TD_CC_NOERROR: c_uint = 0x00;
pub const TD_CC_CRC: c_uint = 0x01;
pub const TD_CC_BITSTUFFING: c_uint = 0x02;
pub const TD_CC_DATATOGGLEM: c_uint = 0x03;
pub const TD_CC_STALL: c_uint = 0x04;
pub const TD_DEVNOTRESP: c_uint = 0x05;
pub const TD_PIDCHECKFAIL: c_uint = 0x06;
pub const TD_UNEXPECTEDPID: c_uint = 0x07;
pub const TD_DATAOVERRUN: c_uint = 0x08;
pub const TD_DATAUNDERRUN: c_uint = 0x09;
// 0x0A, 0x0B reserved for hardware
pub const TD_BUFFEROVERRUN: c_uint = 0x0C;
pub const TD_BUFFERUNDERRUN: c_uint = 0x0D;
// 0x0E, 0x0F reserved for HCD
pub const TD_NOTACCESSED: c_uint = 0x0F;
// map PTD status codes (CC) to errno values
// No  Error  */ 0,
// CRC Error  */ -EILSEQ,
// Bit Stuff  */ -EPROTO,
// Data Togg  */ -EILSEQ,
// Stall      */ -EPIPE,
// DevNotResp */ -ETIME,
// PIDCheck   */ -EPROTO,
// UnExpPID   */ -EPROTO,
// DataOver   */ -EOVERFLOW,
// DataUnder  */ -EREMOTEIO,
// (for hw)   */ -EIO,
// BufferOver */ -ECOMM,
// BuffUnder  */ -ENOSR,
// (for HCD)  */ -EALREADY,
// (for HCD)  */ -EALREADY
}

// --------------------------------------------------------------

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp116x {
    pub lock: spinlock_t,
    pub addr_reg: *mut void __iomem,
    pub data_reg: *mut void __iomem,
    pub board: *mut isp116x_platform_data,
    pub stat16: unsigned long stat1, stat2, stat4, stat8,,
// HC registers
    pub /: *mut *mut u32 intenb; / "OHCI" interrupts,
    pub /: *mut *mut u16 irqenb; / uP interrupts,
// Root hub registers
    pub rhdesca: u32,
    pub rhdescb: u32,
    pub rhstatus: u32,
// async schedule: control, bulk
    pub async: list_head,
// periodic schedule: int
    pub load: [u16; PERIODIC_SIZE],
    pub periodic: [*mut isp116x_ep; PERIODIC_SIZE],
    pub periodic_count: unsigned,
    pub fmindex: u16,
// Schedule for the current frame
    pub atl_active: *mut isp116x_ep,
    pub atl_buflen: c_int,
    pub atl_bufshrt: c_int,
    pub atl_last_dir: c_int,
    pub atl_finishing: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn container_of()isp116x: *mut (void, usb_hcd: struct, _arg: hcd_priv) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp116x_ep {
    pub hep: *mut usb_host_endpoint,
    pub udev: *mut usb_device,
    pub ptd: ptd,
    pub maxpacket: u8,
    pub epnum: u8,
    pub nextpid: u8,
    pub error_count: u16,
    pub /: *mut *mut u16 length; / of current packet,
    pub /: *mut *mut *mut unsigned char data; / to databuf,
// queue of active EP's (the ones scheduled for the
    pub active: *mut isp116x_ep,
// periodic schedule
    pub period: u16,
    pub branch: u16,
    pub load: u16,
    pub next: *mut isp116x_ep,
// async schedule
    pub schedule: list_head,
}

// -------------------------------------------------------------------------

// -------------------------------------------------

pub const isp116x_check_platform_delay(h): c_int = 0;

pub const isp116x_check_platform_delay(h): c_int = 0;

// Let's keep register access functions out of line. Hint:
//
extern "C" {
    pub fn isp116x_read_data16(_arg: isp116x) -> return;
}
extern "C" {
    pub fn isp116x_read_data32(_arg: isp116x) -> return;
}

//

// print debug info about the URB

//

