//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dio.h
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
// header file for DIO boards for the HP300 architecture.
// Maybe this should handle DIO-II later?
// The general structure of this is vaguely based on how
// the Amiga port handles Zorro boards.
// Copyright (C) Peter Maydell 05/1998 <pmaydell@chiark.greenend.org.uk>
// Converted to driver model Jochen Friedrich <jochen@scram.de>
//
// The board IDs are from the NetBSD kernel, which for once provided
// helpful comments...
//
// This goes with drivers/dio/dio.c
//
// The DIO boards in a system are distinguished by 'select codes' which
// range from 0-63 (DIO) and 132-255 (DIO-II).
// The DIO board with select code sc is located at physical address
// 0x600000 + sc * 0x10000
// So DIO cards cover [0x600000-0x800000); the areas [0x200000-0x400000) and
// [0x800000-0x1000000) are for additional space required by things
// like framebuffers. [0x400000-0x600000) is for miscellaneous internal I/O.
// On Linux, this is currently all mapped into the virtual address space
// at 0xf0000000 on bootup.
// DIO-II boards are at 0x1000000 + (sc - 132) * 0x400000
// which is address range [0x1000000-0x20000000) -- too big to map completely,
// so currently we just don't handle DIO-II boards.  It wouldn't be hard to
// do with ioremap() though.
//

pub type dio_id = __u16;
//
// DIO devices
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dio_dev {
    pub bus: *mut dio_bus,
    pub id: dio_id,
    pub scode: c_int,
    pub /: *mut *mut *mut dio_driver driver; / which driver has allocated this device,
    pub /: *mut *mut device dev; / Generic device interface,
    pub ipl: u8,
    pub name: [c_char; 64],
    pub resource: resource,
}

//
// DIO bus
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dio_bus {
    pub /: *mut *mut list_head devices; / list of devices on this bus,
    pub /: *mut *mut unsigned int num_resources; / number of resources,
    pub /: *mut *mut resource resources[2]; / address space routed to this bus,
    pub dev: device,
    pub name: [c_char; 10],
}

//
// DIO device IDs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dio_device_id {
    pub /: *mut *mut dio_id id; / Device ID or DIO_WILDCARD,
    pub /: *mut *mut unsigned long driver_data; / Data private to the driver,
}

//
// DIO device drivers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dio_driver {
    pub node: list_head,
    pub name: *mut c_char,
    pub /: *const *const *const dio_device_id id_table; / NULL if wants all devices,
    pub id): *const *const *const int (probe)(struct dio_dev z, struct dio_device_id,
// New device inserted
    pub /: *mut *mut *mut *mut void (remove)(struct dio_dev z); / Device removed (NULL if not a hot-plug capable driver),
    pub driver: device_driver,
}

// DIO/DIO-II boards all have the following 8bit registers.
// These are offsets from the base of the device.
//
pub const DIO_IDOFF: c_uint = 0x01             /* primary device ID */;
pub const DIO_IPLOFF: c_uint = 0x03             /* interrupt priority level */;
pub const DIO_SECIDOFF: c_uint = 0x15             /* secondary device ID */;
pub const DIOII_SIZEOFF: c_uint = 0x101            /* device size, DIO-II only */;
pub const DIO_VIRADDRBASE: c_uint = 0xf0000000UL   /* vir addr where IOspace is mapped */;
pub const DIO_BASE: c_uint = 0x600000        /* start of DIO space */;
pub const DIO_END: c_uint = 0x1000000       /* end of DIO space */;
pub const DIO_DEVSIZE: c_uint = 0x10000         /* size of a DIO device */;
pub const DIOII_BASE: c_uint = 0x01000000      /* start of DIO-II space */;
pub const DIOII_END: c_uint = 0x20000000      /* end of DIO-II space */;
pub const DIOII_DEVSIZE: c_uint = 0x00400000      /* size of a DIO-II device */;
// highest valid select code

// macros to read device IDs, given base address

// extract the interrupt level

// find the size of a DIO-II board's address space.
// DIO boards are all fixed length.
//

// general purpose macro for both DIO and DIO-II

// The hardware has primary and secondary IDs; we encode these in a single
// int as PRIMARY ID & (SECONDARY ID << 8).
// In practice this is only important for framebuffers,
// and everybody else just sets ID fields equal to the DIO_ID_FOO value.
//

// macro to determine whether a given primary ID requires a secondary ID byte

pub const DIO_WILDCARD: c_uint = 0xff;
// Now a whole slew of macros giving device IDs and descriptive strings:
pub const DIO_ID_DCA0: c_uint = 0x02 /* 98644A serial */;

pub const DIO_ID_DCA0REM: c_uint = 0x82 /* 98644A serial */;

pub const DIO_ID_DCA1: c_uint = 0x42 /* 98644A serial */;

pub const DIO_ID_DCA1REM: c_uint = 0xc2 /* 98644A serial */;

pub const DIO_ID_DCM: c_uint = 0x05 /* 98642A serial MUX */;

pub const DIO_ID_DCMREM: c_uint = 0x85 /* 98642A serial MUX */;

pub const DIO_ID_LAN: c_uint = 0x15 /* 98643A LAN */;

pub const DIO_ID_FHPIB: c_uint = 0x08 /* 98625A/98625B fast HP-IB */;

pub const DIO_ID_NHPIB: c_uint = 0x01 /* 98624A HP-IB (normal ie slow) */;

pub const DIO_ID_SCSI0: c_uint = 0x07 /* 98265A SCSI */;

pub const DIO_ID_SCSI1: c_uint = 0x27 /* ditto */;

pub const DIO_ID_SCSI2: c_uint = 0x47 /* ditto */;

pub const DIO_ID_SCSI3: c_uint = 0x67 /* ditto */;

pub const DIO_ID_FBUFFER: c_uint = 0x39 /* framebuffer: flavour is distinguished by secondary ID */;

// the NetBSD kernel source is a bit unsure as to what these next IDs actually do :->
pub const DIO_ID_MISC0: c_uint = 0x03 /* 98622A */;

pub const DIO_ID_MISC1: c_uint = 0x04 /* 98623A */;

pub const DIO_ID_PARALLEL: c_uint = 0x06 /* internal parallel */;

pub const DIO_ID_MISC2: c_uint = 0x09 /* 98287A keyboard */;

pub const DIO_ID_MISC3: c_uint = 0x0a /* HP98635A FP accelerator */;

pub const DIO_ID_MISC4: c_uint = 0x0b /* timer */;

pub const DIO_ID_MISC5: c_uint = 0x12 /* 98640A */;

pub const DIO_ID_MISC6: c_uint = 0x16 /* 98659A */;

pub const DIO_ID_MISC7: c_uint = 0x19 /* 237 display */;

pub const DIO_ID_MISC8: c_uint = 0x1a /* quad-wide card */;

pub const DIO_ID_MISC9: c_uint = 0x1b /* 98253A */;

pub const DIO_ID_MISC10: c_uint = 0x1c /* 98627A */;

pub const DIO_ID_MISC11: c_uint = 0x1d /* 98633A */;

pub const DIO_ID_MISC12: c_uint = 0x1e /* 98259A */;

pub const DIO_ID_MISC13: c_uint = 0x1f /* 8741 */;

pub const DIO_ID_VME: c_uint = 0x31 /* 98577A VME adapter */;

pub const DIO_ID_DCL: c_uint = 0x34 /* 98628A serial */;

pub const DIO_ID_DCLREM: c_uint = 0xb4 /* 98628A serial */;

// These are the secondary IDs for the framebuffers
pub const DIO_ID2_GATORBOX: c_uint = 0x01 /* 98700/98710 "gatorbox" */;

pub const DIO_ID2_TOPCAT: c_uint = 0x02 /* 98544/98545/98547 "topcat" */;

pub const DIO_ID2_RENAISSANCE: c_uint = 0x04 /* 98720/98721 "renaissance" */;

pub const DIO_ID2_LRCATSEYE: c_uint = 0x05 /* lowres "catseye" */;

pub const DIO_ID2_HRCCATSEYE: c_uint = 0x06 /* highres colour "catseye" */;

pub const DIO_ID2_HRMCATSEYE: c_uint = 0x07 /* highres mono "catseye" */;

pub const DIO_ID2_DAVINCI: c_uint = 0x08 /* 98730/98731 "davinci" */;

pub const DIO_ID2_XXXCATSEYE: c_uint = 0x09 /* "catseye" */;

pub const DIO_ID2_HYPERION: c_uint = 0x0e /* A1096A "hyperion" */;

pub const DIO_ID2_XGENESIS: c_uint = 0x0b /* "x-genesis"; no NetBSD support */;

pub const DIO_ID2_TIGER: c_uint = 0x0c /* "tiger"; no NetBSD support */;

pub const DIO_ID2_YGENESIS: c_uint = 0x0d /* "y-genesis"; no NetBSD support */;

// if you add new IDs then you should tell dio.c about them so it can
// identify them...
//
extern "C" {
    pub fn dio_find(deviceid: c_int) -> c_int;
}
extern "C" {
    pub fn dio_scodetophysaddr(scode: c_int) -> c_ulong;
}
extern "C" {
    pub fn dio_create_sysfs_dev_files(: *mut dio_dev) -> c_int;
}
// New-style probing
extern "C" {
    pub fn dio_register_driver(: *mut dio_driver) -> c_int;
}
extern "C" {
    pub fn dio_unregister_driver(: *mut dio_driver);
}

// Similar to the helpers above, these manipulate per-dio_dev
// driver-specific data.  They are really just a wrapper around
// the generic device structure functions of these calls.
//
extern "C" {
    pub fn dev_get_drvdata(_arg: &d->dev) -> return;
}

