//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/chio.h
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
// ioctl interface for the scsi media changer driver
//
// changer element types

//
// CHIOGPARAMS
// query changer properties
//
// CHIOVGPARAMS
// query vendor-specific element types
//
// accessing elements works by specifing type and unit of the element.
// for example, storage elements are addressed with type = CHET_ST and
// unit = 0 .. cp_nslots-1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct changer_params {
    pub /: *mut *mut int cp_curpicker; / current transport element,
    pub /: *mut *mut int cp_npickers; / number of transport elements (CHET_MT),
    pub /: *mut *mut int cp_nslots; / number of storage elements (CHET_ST),
    pub /: *mut *mut int cp_nportals; / number of import/export elements (CHET_IE),
    pub /: *mut *mut int cp_ndrives; / number of data transfer elements (CHET_DT),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct changer_vendor_params {
    pub /: *mut *mut int cvp_n1; / number of vendor specific elems (CHET_V1),
    pub cvp_label1: [c_char; 16],
    pub /: *mut *mut int cvp_n2; / number of vendor specific elems (CHET_V2),
    pub cvp_label2: [c_char; 16],
    pub /: *mut *mut int cvp_n3; / number of vendor specific elems (CHET_V3),
    pub cvp_label3: [c_char; 16],
    pub /: *mut *mut int cvp_n4; / number of vendor specific elems (CHET_V4),
    pub cvp_label4: [c_char; 16],
    pub reserved: [c_int; 8],
}

//
// CHIOMOVE
// move a medium from one element to another
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct changer_move {
    pub /: *mut *mut int cm_fromtype; / type/unit of source element,
    pub cm_fromunit: c_int,
    pub /: *mut *mut int cm_totype; / type/unit of destination element,
    pub cm_tounit: c_int,
    pub cm_flags: c_int,
}

//
// CHIOEXCHANGE
// move one medium from element #1 to element #2,
// and another one from element #2 to element #3.
// element #1 and #3 are allowed to be identical.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct changer_exchange {
    pub /: *mut *mut int ce_srctype; / type/unit of element #1,
    pub ce_srcunit: c_int,
    pub /: *mut *mut int ce_fdsttype; / type/unit of element #2,
    pub ce_fdstunit: c_int,
    pub /: *mut *mut int ce_sdsttype; / type/unit of element #3,
    pub ce_sdstunit: c_int,
    pub ce_flags: c_int,
}

pub const CE_INVERT1: c_int = 1;
pub const CE_INVERT2: c_int = 2;
//
// CHIOPOSITION
// move the transport element (robot arm) to a specific element.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct changer_position {
    pub cp_type: c_int,
    pub cp_unit: c_int,
    pub cp_flags: c_int,
}

pub const CP_INVERT: c_int = 1;
//
// CHIOGSTATUS
// get element status for all elements of a specific type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct changer_element_status {
    pub ces_type: c_int,
    pub ces_data: *mut unsigned char __user,
}

pub const CESTATUS_FULL: c_uint = 0x01 /* full */;
pub const CESTATUS_IMPEXP: c_uint = 0x02	/* media was imported (inserted by sysop) */;
pub const CESTATUS_EXCEPT: c_uint = 0x04	/* error condition */;
pub const CESTATUS_ACCESS: c_uint = 0x08	/* access allowed */;
pub const CESTATUS_EXENAB: c_uint = 0x10	/* element can export media */;
pub const CESTATUS_INENAB: c_uint = 0x20	/* element can import media */;
//
// CHIOGELEM
// get more detailed status information for a single element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct changer_get_element {
    pub /: *mut *mut int cge_type; / type/unit,
    pub cge_unit: c_int,
    pub /: *mut *mut int cge_status; / status,
    pub /: *mut *mut int cge_errno; / errno,
    pub /: *mut *mut int cge_srctype; / source element of the last move/exchange,
    pub cge_srcunit: c_int,
    pub /: *mut *mut int cge_id; / scsi id (for data transfer elements),
    pub /: *mut *mut int cge_lun; / scsi lun (for data transfer elements),
    pub /: *mut *mut char cge_pvoltag[36]; / primary volume tag,
    pub /: *mut *mut char cge_avoltag[36]; / alternate volume tag,
    pub cge_flags: c_int,
}

// flags
pub const CGE_ERRNO: c_uint = 0x01       /* errno available       */;
pub const CGE_INVERT: c_uint = 0x02       /* media inverted        */;
pub const CGE_SRC: c_uint = 0x04       /* media src available   */;
pub const CGE_IDLUN: c_uint = 0x08       /* ID+LUN available      */;
pub const CGE_PVOLTAG: c_uint = 0x10       /* primary volume tag available */;
pub const CGE_AVOLTAG: c_uint = 0x20       /* alternate volume tag available */;
//
// CHIOSVOLTAG
// set volume tag
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct changer_set_voltag {
    pub /: *mut *mut int csv_type; / type/unit,
    pub csv_unit: c_int,
    pub /: *mut *mut char csv_voltag[36]; / volume tag,
    pub csv_flags: c_int,
}

pub const CSV_PVOLTAG: c_uint = 0x01       /* primary volume tag */;
pub const CSV_AVOLTAG: c_uint = 0x02       /* alternate volume tag */;
pub const CSV_CLEARTAG: c_uint = 0x04       /* clear volume tag */;
// ioctls

