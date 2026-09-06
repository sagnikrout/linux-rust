//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/macsec/macsec_api.h
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
// Atlantic Network Driver
// Copyright (C) 2020 Marvell International Ltd.
//

pub const NUMROWS_INGRESSPRECTLFRECORD: c_int = 24;
pub const ROWOFFSET_INGRESSPRECTLFRECORD: c_int = 0;
pub const NUMROWS_INGRESSPRECLASSRECORD: c_int = 48;
pub const ROWOFFSET_INGRESSPRECLASSRECORD: c_int = 0;
pub const NUMROWS_INGRESSPOSTCLASSRECORD: c_int = 48;
pub const ROWOFFSET_INGRESSPOSTCLASSRECORD: c_int = 0;
pub const NUMROWS_INGRESSSCRECORD: c_int = 32;
pub const ROWOFFSET_INGRESSSCRECORD: c_int = 0;
pub const NUMROWS_INGRESSSARECORD: c_int = 32;
pub const ROWOFFSET_INGRESSSARECORD: c_int = 32;
pub const NUMROWS_INGRESSSAKEYRECORD: c_int = 32;
pub const ROWOFFSET_INGRESSSAKEYRECORD: c_int = 0;
pub const NUMROWS_INGRESSPOSTCTLFRECORD: c_int = 24;
pub const ROWOFFSET_INGRESSPOSTCTLFRECORD: c_int = 0;
pub const NUMROWS_EGRESSCTLFRECORD: c_int = 24;
pub const ROWOFFSET_EGRESSCTLFRECORD: c_int = 0;
pub const NUMROWS_EGRESSCLASSRECORD: c_int = 48;
pub const ROWOFFSET_EGRESSCLASSRECORD: c_int = 0;
pub const NUMROWS_EGRESSSCRECORD: c_int = 32;
pub const ROWOFFSET_EGRESSSCRECORD: c_int = 0;
pub const NUMROWS_EGRESSSARECORD: c_int = 32;
pub const ROWOFFSET_EGRESSSARECORD: c_int = 32;
pub const NUMROWS_EGRESSSAKEYRECORD: c_int = 32;
pub const ROWOFFSET_EGRESSSAKEYRECORD: c_int = 96;
// !  Read the raw table data from the specified row of the Egress CTL
// Filter table, and unpack it into the fields of rec.
// rec - [OUT] The raw table row data will be unpacked into the fields of rec.
// table_index - The table row to read (max 23).
//
// !  Pack the fields of rec, and write the packed data into the
// specified row of the Egress CTL Filter table.
// rec - [IN] The bitfield values to write to the table row.
// table_index - The table row to write(max 23).
//
// !  Read the raw table data from the specified row of the Egress
// Packet Classifier table, and unpack it into the fields of rec.
// rec - [OUT] The raw table row data will be unpacked into the fields of rec.
// table_index - The table row to read (max 47).
//
// !  Pack the fields of rec, and write the packed data into the
// specified row of the Egress Packet Classifier table.
// rec - [IN] The bitfield values to write to the table row.
// table_index - The table row to write (max 47).
//
// !  Read the raw table data from the specified row of the Egress SC
// Lookup table, and unpack it into the fields of rec.
// rec - [OUT] The raw table row data will be unpacked into the fields of rec.
// table_index - The table row to read (max 31).
//
// !  Pack the fields of rec, and write the packed data into the
// specified row of the Egress SC Lookup table.
// rec - [IN] The bitfield values to write to the table row.
// table_index - The table row to write (max 31).
//
// !  Read the raw table data from the specified row of the Egress SA
// Lookup table, and unpack it into the fields of rec.
// rec - [OUT] The raw table row data will be unpacked into the fields of rec.
// table_index - The table row to read (max 31).
//
// !  Pack the fields of rec, and write the packed data into the
// specified row of the Egress SA Lookup table.
// rec  - [IN] The bitfield values to write to the table row.
// table_index - The table row to write (max 31).
//
// !  Read the raw table data from the specified row of the Egress SA
// Key Lookup table, and unpack it into the fields of rec.
// rec - [OUT] The raw table row data will be unpacked into the fields of rec.
// table_index - The table row to read (max 31).
//
// !  Pack the fields of rec, and write the packed data into the
// specified row of the Egress SA Key Lookup table.
// rec - [IN] The bitfield values to write to the table row.
// table_index - The table row to write (max 31).
//
// !  Read the raw table data from the specified row of the Ingress
// Pre-MACSec CTL Filter table, and unpack it into the fields of rec.
// rec - [OUT] The raw table row data will be unpacked into the fields of rec.
// table_index - The table row to read (max 23).
//
// !  Pack the fields of rec, and write the packed data into the
// specified row of the Ingress Pre-MACSec CTL Filter table.
// rec - [IN] The bitfield values to write to the table row.
// table_index - The table row to write(max 23).
//
// !  Read the raw table data from the specified row of the Ingress
// Pre-MACSec Packet Classifier table, and unpack it into the fields of rec.
// rec - [OUT] The raw table row data will be unpacked into the fields of rec.
// table_index - The table row to read (max 47).
//
// !  Pack the fields of rec, and write the packed data into the
// specified row of the Ingress Pre-MACSec Packet Classifier table.
// rec - [IN] The bitfield values to write to the table row.
// table_index - The table row to write(max 47).
//
// !  Read the raw table data from the specified row of the Ingress SC
// Lookup table, and unpack it into the fields of rec.
// rec - [OUT] The raw table row data will be unpacked into the fields of rec.
// table_index - The table row to read (max 31).
//
// !  Pack the fields of rec, and write the packed data into the
// specified row of the Ingress SC Lookup table.
// rec - [IN] The bitfield values to write to the table row.
// table_index - The table row to write(max 31).
//
// !  Read the raw table data from the specified row of the Ingress SA
// Lookup table, and unpack it into the fields of rec.
// rec - [OUT] The raw table row data will be unpacked into the fields of rec.
// table_index - The table row to read (max 31).
//
// !  Pack the fields of rec, and write the packed data into the
// specified row of the Ingress SA Lookup table.
// rec - [IN] The bitfield values to write to the table row.
// table_index - The table row to write(max 31).
//
// !  Read the raw table data from the specified row of the Ingress SA
// Key Lookup table, and unpack it into the fields of rec.
// rec - [OUT] The raw table row data will be unpacked into the fields of rec.
// table_index - The table row to read (max 31).
//
// !  Pack the fields of rec, and write the packed data into the
// specified row of the Ingress SA Key Lookup table.
// rec - [IN] The bitfield values to write to the table row.
// table_index - The table row to write(max 31).
//
// !  Read the raw table data from the specified row of the Ingress
// Post-MACSec Packet Classifier table, and unpack it into the
// fields of rec.
// rec - [OUT] The raw table row data will be unpacked into the fields of rec.
// table_index - The table row to read (max 48).
//
// !  Pack the fields of rec, and write the packed data into the
// specified row of the Ingress Post-MACSec Packet Classifier table.
// rec - [IN] The bitfield values to write to the table row.
// table_index - The table row to write(max 48).
//
// !  Read the raw table data from the specified row of the Ingress
// Post-MACSec CTL Filter table, and unpack it into the fields of rec.
// rec - [OUT] The raw table row data will be unpacked into the fields of rec.
// table_index - The table row to read (max 23).
//
// !  Pack the fields of rec, and write the packed data into the
// specified row of the Ingress Post-MACSec CTL Filter table.
// rec - [IN] The bitfield values to write to the table row.
// table_index - The table row to write(max 23).
//
// !  Read the counters for the specified SC, and unpack them into the
// fields of counters.
// counters - [OUT] The raw table row data will be unpacked here.
// sc_index - The table row to read (max 31).
//
// !  Read the counters for the specified SA, and unpack them into the
// fields of counters.
// counters - [OUT] The raw table row data will be unpacked here.
// sa_index - The table row to read (max 31).
//
// !  Read the counters for the common egress counters, and unpack them
// into the fields of counters.
// counters - [OUT] The raw table row data will be unpacked here.
//
// !  Clear all Egress counters to 0.
extern "C" {
    pub fn aq_mss_clear_egress_counters(hw: *mut aq_hw_s) -> c_int;
}
// !  Read the counters for the specified SA, and unpack them into the
// fields of counters.
// counters - [OUT] The raw table row data will be unpacked here.
// sa_index - The table row to read (max 31).
//
// !  Read the counters for the common ingress counters, and unpack them
// into the fields of counters.
// counters - [OUT] The raw table row data will be unpacked here.
//
// !  Clear all Ingress counters to 0.
extern "C" {
    pub fn aq_mss_clear_ingress_counters(hw: *mut aq_hw_s) -> c_int;
}
// !  Get Egress SA expired.
extern "C" {
    pub fn aq_mss_get_egress_sa_expired(hw: *mut aq_hw_s, expired: *mut u32) -> c_int;
}
// !  Get Egress SA threshold expired.
// !  Set Egress SA expired.
extern "C" {
    pub fn aq_mss_set_egress_sa_expired(hw: *mut aq_hw_s, expired: u32) -> c_int;
}
// !  Set Egress SA threshold expired.
