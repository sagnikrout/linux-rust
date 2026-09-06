//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/atmsap.h
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
// atmsap.h - ATM Service Access Point addressing definitions
// Written 1995-1999 by Werner Almesberger, EPFL LRC/ICA

//
// BEGIN_xx and END_xx markers are used for automatic generation of
// documentation. Do not change them.
//
// Layer 2 protocol identifiers
//
// BEGIN_L2

pub const ATM_L2_ISO1745: c_uint = 0x01	/* Basic mode ISO 1745 */;
pub const ATM_L2_Q291: c_uint = 0x02	/* ITU-T Q.291 (Rec. I.441) */;
pub const ATM_L2_X25_LL: c_uint = 0x06	/* ITU-T X.25, link layer */;
pub const ATM_L2_X25_ML: c_uint = 0x07	/* ITU-T X.25, multilink */;
pub const ATM_L2_LAPB: c_uint = 0x08	/* Extended LAPB, half-duplex (Rec. T.71) */;
pub const ATM_L2_HDLC_ARM: c_uint = 0x09	/* HDLC ARM (ISO/IEC 4335) */;
pub const ATM_L2_HDLC_NRM: c_uint = 0x0a	/* HDLC NRM (ISO/IEC 4335) */;
pub const ATM_L2_HDLC_ABM: c_uint = 0x0b	/* HDLC ABM (ISO/IEC 4335) */;
pub const ATM_L2_ISO8802: c_uint = 0x0c	/* LAN LLC (ISO/IEC 8802/2) */;
pub const ATM_L2_X75: c_uint = 0x0d	/* ITU-T X.75, SLP */;
pub const ATM_L2_Q922: c_uint = 0x0e	/* ITU-T Q.922 */;
pub const ATM_L2_USER: c_uint = 0x10	/* user-specified */;
pub const ATM_L2_ISO7776: c_uint = 0x11	/* ISO 7776 DTE-DTE */;
// END_L2
//
// Layer 3 protocol identifiers
//
// BEGIN_L3

pub const ATM_L3_X25: c_uint = 0x06	/* ITU-T X.25, packet layer */;
pub const ATM_L3_ISO8208: c_uint = 0x07	/* ISO/IEC 8208 */;
pub const ATM_L3_X223: c_uint = 0x08	/* ITU-T X.223 | ISO/IEC 8878 */;
pub const ATM_L3_ISO8473: c_uint = 0x09	/* ITU-T X.233 | ISO/IEC 8473 */;
pub const ATM_L3_T70: c_uint = 0x0a	/* ITU-T T.70 minimum network layer */;
pub const ATM_L3_TR9577: c_uint = 0x0b	/* ISO/IEC TR 9577 */;
pub const ATM_L3_H310: c_uint = 0x0c	/* ITU-T Recommendation H.310 */;
pub const ATM_L3_H321: c_uint = 0x0d	/* ITU-T Recommendation H.321 */;
pub const ATM_L3_USER: c_uint = 0x10	/* user-specified */;
// END_L3
//
// High layer identifiers
//
// BEGIN_HL

pub const ATM_HL_ISO: c_uint = 0x01	/* ISO */;
pub const ATM_HL_USER: c_uint = 0x02	/* user-specific */;
pub const ATM_HL_HLP: c_uint = 0x03	/* high layer profile - UNI 3.0 only */;
pub const ATM_HL_VENDOR: c_uint = 0x04	/* vendor-specific application identifier */;
// END_HL
//
// ITU-T coded mode of operation
//
// BEGIN_IMD

// END_IMD
//
// H.310 code points
//

//
// SAP structures
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_blli {
    pub /: *mut *mut unsigned char l2_proto; / layer 2 protocol,
    pub /: *mut *mut unsigned char mode; / mode of operation (ATM_IMD_xxx), 0 if,
// absent
    pub /: *mut *mut unsigned char window; / window size (k), 1-127 (0 to omit),
    pub /: *mut *mut } itu; / ITU-T encoding,
    pub /: *mut *mut unsigned char user; / user-specified l2 information,
    pub l2: },
    pub /: *mut *mut unsigned char l3_proto; / layer 3 protocol,
    pub /: *mut *mut unsigned char mode; / mode of operation (ATM_IMD_xxx), 0 if,
// absent
    pub /: *mut *mut unsigned char def_size; / default packet size (log2), 4-12 (0 to,
// omit)
    pub /: *mut *mut unsigned char window;/ packet window size, 1-127 (0 to omit),
    pub /: *mut *mut } itu; / ITU-T encoding,
    pub /: *mut *mut unsigned char user; / user specified l3 information,
    pub /: *mut *mut unsigned char term_type; / terminal type,
    pub /: *mut *mut unsigned char fw_mpx_cap; / forward multiplexing capability,
// only if term_type != ATM_TT_NONE
    pub /: *mut *mut unsigned char bw_mpx_cap; / backward multiplexing capability,
// only if term_type != ATM_TT_NONE
    pub h310: },
    pub /: *mut *mut unsigned char ipi; / initial protocol id,
    pub /: *mut *mut unsigned char snap[5];/ IEEE 802.1 SNAP identifier,
// (only if ipi == NLPID_IEEE802_1_SNAP)
    pub tr9577: },
    pub l3: },
    pub __ATM_API_ALIGN: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_bhli {
    pub /: *mut *mut unsigned char hl_type; / high layer information type,
    pub /: *mut *mut unsigned char hl_length; / length (only if hl_type == ATM_HL_USER ||,
// hl_type == ATM_HL_ISO)
    pub /: *mut *mut unsigned char hl_info[ATM_MAX_HLI];/ high layer information,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_sap {
    pub /: *mut *mut atm_bhli bhli; / local SAP, high-layer information,
    pub __ATM_API_ALIGN: atm_blli blli[ATM_MAX_BLLI],
// local SAP, low-layer info
}
