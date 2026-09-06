//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/atm.h
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
// atm.h - general ATM declarations
// Written 1995-2000 by Werner Almesberger, EPFL LRC/ICA
//
// WARNING: User-space programs should not #include <linux/atm.h> directly.
// Instead, #include <atm.h>
//
// BEGIN_xx and END_xx markers are used for automatic generation of
// documentation. Do not change them.
//

// general ATM constants

// "protcol" values for the socket system call

//
// socket option name coding functions
//
// Note that __SO_ENCODE and __SO_LEVEL are somewhat a hack since the
// << 22 only reserves 9 bits for the level.  On some architectures
// SOL_SOCKET is 0xFFFF, so that's a bit of a problem
//

//
// ATM layer
//

// set CLP bit value - TODO

// connection identifier range; socket must be

// Quality of Service setting

// Service Access Point

// "PVC" address (also for SVCs); get only

// make this vc a p2mp
//
// Note @@@: since the socket layers don't really distinguish the control and
// the data plane but generally seems to be data plane-centric, any layer is
// about equally wrong for the SAP. If you have a better idea about this,
// please speak up ...
//
// ATM cell header (for AAL0)
// BEGIN_CH
pub const ATM_HDR_GFC_MASK: c_uint = 0xf0000000;
pub const ATM_HDR_GFC_SHIFT: c_int = 28;
pub const ATM_HDR_VPI_MASK: c_uint = 0x0ff00000;
pub const ATM_HDR_VPI_SHIFT: c_int = 20;
pub const ATM_HDR_VCI_MASK: c_uint = 0x000ffff0;
pub const ATM_HDR_VCI_SHIFT: c_int = 4;
pub const ATM_HDR_PTI_MASK: c_uint = 0x0000000e;
pub const ATM_HDR_PTI_SHIFT: c_int = 1;
pub const ATM_HDR_CLP: c_uint = 0x00000001;
// END_CH
// PTI codings
// BEGIN_PTI

// END_PTI
//
// The following items should stay in linux/atm.h, which should be linked to
// netatm/atm.h
//
// Traffic description

pub const ATM_UBR: c_int = 1;
pub const ATM_CBR: c_int = 2;
pub const ATM_VBR: c_int = 3;
pub const ATM_ABR: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_trafprm {
    pub /: *mut *mut unsigned char traffic_class; / traffic class (ATM_UBR, ...),
    pub /: *mut *mut int max_pcr; / maximum PCR in cells per second,
    pub /: *mut *mut int pcr; / desired PCR in cells per second,
    pub /: *mut *mut int min_pcr; / minimum PCR in cells per second,
    pub /: *mut *mut int max_cdv; / maximum CDV in microseconds,
    pub /: *mut *mut int max_sdu; / maximum SDU in bytes,
// extra params for ABR
    pub /: *mut *mut unsigned int icr; / Initial Cell Rate (24-bit),
    pub /: *mut *mut unsigned int tbe; / Transient Buffer Exposure (24-bit),
    pub /: *mut *mut unsigned int frtt : 24; / Fixed Round Trip Time (24-bit),
    pub /: *mut *mut unsigned int rif : 4; / Rate Increment Factor (4-bit),
    pub /: *mut *mut unsigned int rdf : 4; / Rate Decrease Factor (4-bit),
    pub /: *mut *mut unsigned int nrm_pres :1; / nrm present bit,
    pub /: *mut *mut unsigned int trm_pres :1; / rm present bit,
    pub /: *mut *mut unsigned int adtf_pres :1; / adtf present bit,
    pub bit*/: *mut *mut unsigned int cdf_pres :1; / cdf present,
    pub /: *mut *mut unsigned int nrm :3; / Max # of Cells for each forward RM cell (3-bit),
    pub /: *mut *mut unsigned int trm :3; / Time between forward RM cells (3-bit),
    pub /: *mut *mut unsigned int adtf :10; / ACR Decrease Time Factor (10-bit),
    pub /: *mut *mut unsigned int cdf :3; / Cutoff Decrease Factor (3-bit),
    pub /: *mut *mut unsigned int spare :9; / spare bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_qos {
    pub /: *mut *mut atm_trafprm txtp; / parameters in TX direction,
    pub __ATM_API_ALIGN: atm_trafprm rxtp,
// parameters in RX direction
    pub __ATM_API_ALIGN: unsigned char aal,
}

// PVC addressing

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_atmpvc {
    pub /: *mut *mut unsigned short sap_family; / address family, AF_ATMPVC,
    pub /: *mut *mut short itf; / ATM interface,
    pub /: *mut *mut short vpi; / VPI (only 8 bits at UNI),
    pub /: *mut *mut int vci; / VCI (only 16 bits at UNI),
    pub /: *mut *mut } sap_addr __ATM_API_ALIGN; / PVC address,
}

// SVC addressing

pub const ATM_AFI_DCC: c_uint = 0x39		/* DCC ATM Format */;
pub const ATM_AFI_ICD: c_uint = 0x47		/* ICD ATM Format */;
pub const ATM_AFI_E164: c_uint = 0x45		/* E.164 ATM Format */;
pub const ATM_AFI_LOCAL: c_uint = 0x49		/* Local ATM Format */;
pub const ATM_AFI_DCC_GROUP: c_uint = 0xBD	/* DCC ATM Group Format */;
pub const ATM_AFI_ICD_GROUP: c_uint = 0xC5	/* ICD ATM Group Format */;
pub const ATM_AFI_E164_GROUP: c_uint = 0xC3	/* E.164 ATM Group Format */;
pub const ATM_AFI_LOCAL_GROUP: c_uint = 0xC7	/* Local ATM Group Format */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_atmsvc {
    pub /: *mut *mut unsigned short sas_family; / address family, AF_ATMSVC,
    pub /: *mut *mut unsigned char prv[ATM_ESA_LEN];/ private ATM address,
    pub /: *mut *mut char pub[ATM_E164_LEN+1]; / public address (E.164),
// unused addresses must be bzero'ed
    pub /: *mut *mut *mut char lij_type; / role in LIJ call; one of ATM_LIJ,
    pub /: *mut *mut __u32 lij_id; / LIJ call identifier,
    pub /: *mut *mut } sas_addr __ATM_API_ALIGN; / SVC address,
}

//
// Some stuff for linux/sockios.h
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmif_sioc {
    pub number: c_int,
    pub length: c_int,
    pub arg: *mut void __user,
}

pub type atm_backend_t = c_ushort;
