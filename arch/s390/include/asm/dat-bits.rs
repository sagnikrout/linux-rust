//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/dat-bits.h
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
// DAT table and related structures
//
// Copyright IBM Corp. 2024
//
// vaddress union in order to easily decode a virtual address into its
// region first index, region second index etc. parts.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union vaddress {
    pub addr: c_ulong,
    pub 11: unsigned long rfx :,
    pub 11: unsigned long rsx :,
    pub 11: unsigned long rtx :,
    pub 11: unsigned long sx :,
    pub 8: unsigned long px :,
    pub 12: unsigned long bx :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union asce {
    pub val: c_ulong,
    pub /: *mut *mut unsigned long rsto: 52;/ Region- or Segment-Table Origin,
    pub 2: unsigned long :,
    pub /: *mut *mut unsigned long g : 1; / Subspace Group control,
    pub /: *mut *mut unsigned long p : 1; / Private Space control,
    pub /: *mut *mut unsigned long s : 1; / Storage-Alteration-Event control,
    pub /: *mut *mut unsigned long x : 1; / Space-Switch-Event control,
    pub /: *mut *mut unsigned long r : 1; / Real-Space control,
    pub 1: unsigned long :,
    pub /: *mut *mut unsigned long dt : 2; / Designation-Type control,
    pub /: *mut *mut unsigned long tl : 2; / Region- or Segment-Table Length,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union region1_table_entry {
    pub val: c_ulong,
    pub /: *mut *mut unsigned long rto: 52;/ Region-Table Origin,
    pub 2: unsigned long :,
    pub /: *mut *mut unsigned long p : 1; / DAT-Protection Bit,
    pub 1: unsigned long :,
    pub /: *mut *mut unsigned long tf : 2; / Region-Second-Table Offset,
    pub /: *mut *mut unsigned long i : 1; / Region-Invalid Bit,
    pub 1: unsigned long :,
    pub /: *mut *mut unsigned long tt : 2; / Table-Type Bits,
    pub /: *mut *mut unsigned long tl : 2; / Region-Second-Table Length,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union region2_table_entry {
    pub val: c_ulong,
    pub /: *mut *mut unsigned long rto: 52;/ Region-Table Origin,
    pub 2: unsigned long :,
    pub /: *mut *mut unsigned long p : 1; / DAT-Protection Bit,
    pub 1: unsigned long :,
    pub /: *mut *mut unsigned long tf : 2; / Region-Third-Table Offset,
    pub /: *mut *mut unsigned long i : 1; / Region-Invalid Bit,
    pub 1: unsigned long :,
    pub /: *mut *mut unsigned long tt : 2; / Table-Type Bits,
    pub /: *mut *mut unsigned long tl : 2; / Region-Third-Table Length,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct region3_table_entry_fc0 {
    pub /: *mut *mut unsigned long sto: 52;/ Segment-Table Origin,
    pub 1: unsigned long :,
    pub /: *mut *mut unsigned long fc : 1; / Format-Control,
    pub /: *mut *mut unsigned long p : 1; / DAT-Protection Bit,
    pub 1: unsigned long :,
    pub /: *mut *mut unsigned long tf : 2; / Segment-Table Offset,
    pub /: *mut *mut unsigned long i : 1; / Region-Invalid Bit,
    pub /: *mut *mut unsigned long cr : 1; / Common-Region Bit,
    pub /: *mut *mut unsigned long tt : 2; / Table-Type Bits,
    pub /: *mut *mut unsigned long tl : 2; / Segment-Table Length,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct region3_table_entry_fc1 {
    pub /: *mut *mut unsigned long rfaa: 33;/ Region-Frame Absolute Address,
    pub 14: unsigned long :,
    pub /: *mut *mut unsigned long av : 1; / ACCF-Validity Control,
    pub /: *mut *mut unsigned long acc : 4; / Access-Control Bits,
    pub /: *mut *mut unsigned long f : 1; / Fetch-Protection Bit,
    pub /: *mut *mut unsigned long fc : 1; / Format-Control,
    pub /: *mut *mut unsigned long p : 1; / DAT-Protection Bit,
    pub /: *mut *mut unsigned long iep : 1; / Instruction-Execution-Protection,
    pub 2: unsigned long :,
    pub /: *mut *mut unsigned long i : 1; / Region-Invalid Bit,
    pub /: *mut *mut unsigned long cr : 1; / Common-Region Bit,
    pub /: *mut *mut unsigned long tt : 2; / Table-Type Bits,
    pub 2: unsigned long :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union region3_table_entry {
    pub val: c_ulong,
    pub fc0: region3_table_entry_fc0,
    pub fc1: region3_table_entry_fc1,
    pub 53: unsigned long :,
    pub /: *mut *mut unsigned long fc: 1; / Format-Control,
    pub /: *mut *mut unsigned long p : 1; / DAT-Protection Bit,
    pub 3: unsigned long :,
    pub /: *mut *mut unsigned long i : 1; / Region-Invalid Bit,
    pub /: *mut *mut unsigned long cr: 1; / Common-Region Bit,
    pub /: *mut *mut unsigned long tt: 2; / Table-Type Bits,
    pub 2: unsigned long :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct segment_table_entry_fc0 {
    pub /: *mut *mut unsigned long pto: 53;/ Page-Table Origin,
    pub /: *mut *mut unsigned long fc : 1; / Format-Control,
    pub /: *mut *mut unsigned long p : 1; / DAT-Protection Bit,
    pub 3: unsigned long :,
    pub /: *mut *mut unsigned long i : 1; / Segment-Invalid Bit,
    pub /: *mut *mut unsigned long cs : 1; / Common-Segment Bit,
    pub /: *mut *mut unsigned long tt : 2; / Table-Type Bits,
    pub 2: unsigned long :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct segment_table_entry_fc1 {
    pub /: *mut *mut unsigned long sfaa: 44;/ Segment-Frame Absolute Address,
    pub 3: unsigned long :,
    pub /: *mut *mut unsigned long av : 1; / ACCF-Validity Control,
    pub /: *mut *mut unsigned long acc : 4; / Access-Control Bits,
    pub /: *mut *mut unsigned long f : 1; / Fetch-Protection Bit,
    pub /: *mut *mut unsigned long fc : 1; / Format-Control,
    pub /: *mut *mut unsigned long p : 1; / DAT-Protection Bit,
    pub /: *mut *mut unsigned long iep : 1; / Instruction-Execution-Protection,
    pub 2: unsigned long :,
    pub /: *mut *mut unsigned long i : 1; / Segment-Invalid Bit,
    pub /: *mut *mut unsigned long cs : 1; / Common-Segment Bit,
    pub /: *mut *mut unsigned long tt : 2; / Table-Type Bits,
    pub 2: unsigned long :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union segment_table_entry {
    pub val: c_ulong,
    pub fc0: segment_table_entry_fc0,
    pub fc1: segment_table_entry_fc1,
    pub 53: unsigned long :,
    pub /: *mut *mut unsigned long fc: 1; / Format-Control,
    pub /: *mut *mut unsigned long p : 1; / DAT-Protection Bit,
    pub 3: unsigned long :,
    pub /: *mut *mut unsigned long i : 1; / Segment-Invalid Bit,
    pub /: *mut *mut unsigned long cs: 1; / Common-Segment Bit,
    pub /: *mut *mut unsigned long tt: 2; / Table-Type Bits,
    pub 2: unsigned long :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union page_table_entry {
    pub val: c_ulong,
    pub /: *mut *mut unsigned long pfra: 52;/ Page-Frame Real Address,
    pub /: *mut *mut unsigned long z : 1; / Zero Bit,
    pub /: *mut *mut unsigned long i : 1; / Page-Invalid Bit,
    pub /: *mut *mut unsigned long p : 1; / DAT-Protection Bit,
    pub /: *mut *mut unsigned long iep : 1; / Instruction-Execution-Protection,
    pub 8: unsigned long :,
}
