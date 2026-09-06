//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/fc/fc_encaps.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2007 Intel Corporation. All rights reserved.
//
// Maintained at www.Open-FCoE.org
//
// Protocol definitions from RFC 3643 - Fibre Channel Frame Encapsulation.
//
// Note:  The frame length field is the number of 32-bit words in
// the encapsulation including the fcip_encaps_header, CRC and EOF words.
// The minimum frame length value in bytes is (32 + 24 + 4 + 4) * 4 = 64.
// The maximum frame length value in bytes is (32 + 24 + 2112 + 4 + 4) = 2172.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_encaps_hdr {
    pub /: *mut *mut __u8 fc_proto; / protocol number,
    pub /: *mut *mut __u8 fc_ver; / version of encapsulation,
    pub /: *mut *mut __u8 fc_proto_n; / ones complement of protocol,
    pub /: *mut *mut __u8 fc_ver_n; / ones complement of version,
    pub /: *mut *mut unsigned char fc_proto_data[8]; / protocol specific data,
    pub /: *mut *mut __be16 fc_len_flags; / 10-bit length/4 w/ 6 flag bits,
    pub /: *mut *mut __be16 fc_len_flags_n; / ones complement of length / flags,
//
// Offset 0x10
//
    pub /: *mut *mut __be32 fc_time[2]; / time stamp: seconds and fraction,
    pub /: *mut *mut __be32 fc_crc; / CRC,
    pub /: *mut *mut __be32 fc_sof; / start of frame (see FC_SOF below),
// 0x20 - FC frame content followed by EOF word
}

pub const FCIP_ENCAPS_HDR_LEN: c_uint = 0x20	/* expected length for asserts */;
//
// Macro's for making redundant copies of EOF and SOF.
//

//
// SOF / EOF bytes.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_sof {
    FC_SOF_F =	0x28,	/* fabric */
    FC_SOF_I4 =	0x29,	/* initiate class 4 */
    FC_SOF_I2 =	0x2d,	/* initiate class 2 */
    FC_SOF_I3 =	0x2e,	/* initiate class 3 */
    FC_SOF_N4 =	0x31,	/* normal class 4 */
    FC_SOF_N2 =	0x35,	/* normal class 2 */
    FC_SOF_N3 =	0x36,	/* normal class 3 */
    FC_SOF_C4 =	0x39,	/* activate class 4 */
    } __attribute__((packed));

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_eof {
    FC_EOF_N =	0x41,	/* normal (not last frame of seq) */
    FC_EOF_T =	0x42,	/* terminate (last frame of sequence) */
    FC_EOF_RT =	0x44,
    FC_EOF_DT =	0x46,	/* disconnect-terminate class-1 */
    FC_EOF_NI =	0x49,	/* normal-invalid */
    FC_EOF_DTI =	0x4e,	/* disconnect-terminate-invalid */
    FC_EOF_RTI =	0x4f,
    FC_EOF_A =	0x50,	/* abort */
    } __attribute__((packed));

pub const FC_SOF_CLASS_MASK: c_uint = 0x06	/* mask for class of service in SOF */;

//
// Define classes in terms of the SOF code (initial).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_class {
    FC_CLASS_NONE = 0,	/* software value indicating no class */
    FC_CLASS_2 =	FC_SOF_I2,
    FC_CLASS_3 =	FC_SOF_I3,
    FC_CLASS_4 =	FC_SOF_I4,
    FC_CLASS_F =	FC_SOF_F,
}

//
// Determine whether SOF code indicates the need for a BLS ACK.
//
// Given an fc_class, return the normal (non-initial) SOF value.
//
// Compute class from SOF value.
//
// Determine whether SOF is for the initial frame of a sequence.
//
