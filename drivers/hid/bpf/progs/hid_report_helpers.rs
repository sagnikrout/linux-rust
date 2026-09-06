//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/bpf/progs/hid_report_helpers.h
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
// Copyright (c) 2024 Red Hat, Inc
//
// THIS FILE IS GENERATED, DO NOT EDIT

// Macros for composing HID reports.
//
// HID Fields are added manually to the template, please add to it as needed
// for any individual device. The Usage Pages and Usages are generated.
//
// Some macros have a _i8, _i16, or _i32 suffix. Pick the
// right suffix given the passed-in value.
//
// This macro behaves like static_assert(), failing to
// compile if its argument is not true.  However, it always
// returns 0, which allows using it everywhere an expression
// can be used.
//

// Ensure the given value fits within 8/16/32 bits

// Split a value across multiple bytes in LE order

// Collections require two items in the report descriptor, the start
// of the collection (0xa?) and the EndCollection item (0xc?).
// This macro provides both, use like this:
//
// static const __u8 fixed_rdesc[] = {
// UsagePage_Generic_Desktop
// Usage_GD_Keyboard
// CollectionApplication(     ← Open the collection
// ReportId(3)
// LogicalMinimum_i8(0)
// LogicalMaximum_i8(1)
// // other fields
// )                          ← End EndCollection
//
// Collections may be nested.
//

// See Collections, this macro provides Push and Pop with
// elements in between
//
pub const PushPop(...): c_uint = 0xa4, __VA_ARGS__ 0xb4,;
// Arguments to use in bitwise-or for Input, Output, Feature
pub const Const: c_uint = 0x1;
pub const Var: c_uint = 0x2;
pub const Arr: c_uint = 0x0;
pub const Abs: c_uint = 0x0;
pub const Rel: c_uint = 0x4;
pub const Null: c_uint = 0x40;
pub const Buff: c_uint = 0x0100;
// Use like this: Input(Var|Abs)
pub const Input(i_): c_uint = 0x081, i8(i_),;
pub const Output(i_): c_uint = 0x091, i8(i_),;
pub const Feature(i_): c_uint = 0x0b1, i8(i_),;
pub const Input_i16(i_): c_uint = 0x082, LE16(i_),;
pub const Output_i16(i_): c_uint = 0x092, LE16(i_),;
pub const Feature_i16(i_): c_uint = 0x0b2, LE16(i_),;
pub const ReportId(id_): c_uint = 0x85, i8(id_),;
pub const ReportSize(sz_): c_uint = 0x75, i8(sz_),;
pub const ReportCount(cnt_): c_uint = 0x95, i8(cnt_),;
pub const LogicalMinimum_i8(min_): c_uint = 0x15, i8(min_),;
pub const LogicalMinimum_i16(min_): c_uint = 0x16, LE16(min_),;
pub const LogicalMinimum_i32(min_): c_uint = 0x17, LE32(min_),;
pub const LogicalMaximum_i8(max_): c_uint = 0x25, i8(max_),;
pub const LogicalMaximum_i16(max_): c_uint = 0x26, LE16(max_),;
pub const LogicalMaximum_i32(max_): c_uint = 0x27, LE32(max_),;
pub const PhysicalMinimum_i8(min_): c_uint = 0x35, i8(min_),;
pub const PhysicalMinimum_i16(min_): c_uint = 0x36, LE16(min_),;
pub const PhysicalMinimum_i32(min_): c_uint = 0x37, LE32(min_),;
pub const PhysicalMaximum_i8(max_): c_uint = 0x45, i8(max_),;
pub const PhysicalMaximum_i16(max_): c_uint = 0x46, LE16(max_),;
pub const PhysicalMaximum_i32(max_): c_uint = 0x47, LE32(max_),;
pub const UsageMinimum_i8(min_): c_uint = 0x19, i8(min_),;
pub const UsageMinimum_i16(min_): c_uint = 0x1a, LE16(min_),;
pub const UsageMaximum_i8(max_): c_uint = 0x29, i8(max_),;
pub const UsageMaximum_i16(max_): c_uint = 0x2a, LE16(max_),;
pub const UsagePage_i8(p_): c_uint = 0x05, i8(p_),;
pub const UsagePage_i16(p_): c_uint = 0x06, LE16(p_),;
pub const Usage_i8(u_): c_uint = 0x09, i8(u_),;
pub const Usage_i16(u_): c_uint = 0x0a, LE16(u_),;
pub const Usage_i32(u_): c_uint = 0x0b, LE32(u_),;
pub const SILinear: c_uint = 0x1;
pub const SIRotation: c_uint = 0x2;
pub const EnglishLinear: c_uint = 0x3;
pub const EnglishRotation: c_uint = 0x4;

// Use as Unit(cm) or Unit(rad) or similar.
// This macro currently defaults to exponent 1 only, so no
// cm^2 or others
//

pub const Unit_i8(u_): c_uint = 0x65, i8(u_),;
pub const Unit_i16(u_): c_uint = 0x66, i16(u_),;
pub const Unit_i32(u_): c_uint = 0x67, i32(u_),;
pub const UnitExponent(u_): c_uint = 0x55, i4(u_),;
// A macro to generate a vendor-specific padding-only
// report with Report ID 0xac of the given size in bytes.
// The size is inclusive of the 1 byte Report ID prefix.
//
// The kernel discards any HID reports that are larger
// than the largest report in a HID report descriptor.
// Thus at least one report must have (at least)
// the same size as the largest original report from
// the device.
// The easy way to ensure that is to add this
// macro as the last element of your CollectionApplication
// other reports can be of any size less than this.
//
// e.g.
// static __u8 fixed_rdesc = [
// UsagePage_Generic_Desktop
// Usage_GD_Keyboard
// CollectionApplication(
// ... intended rdesc items go here ...
// FixedSizeVendorReport(12)
// )
// ];
//
// If the FixedSizeVendorReport is placed outside
// a CollectionApplication it will result in
// an extra useless evdev node being created.
//

// ----- Generated Usage Pages and Usages ------

