//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/gendwarfksyms/examples/kabi_ex.h
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
// kabi_ex.h
//
// Copyright (C) 2024 Google LLC
//
// Examples for kABI stability features with --stable.
//
// The comments below each example contain the expected gendwarfksyms
// output, which can be verified using LLVM's FileCheck tool:
//
// https://llvm.org/docs/CommandGuide/FileCheck.html
//
// Usage:
//
// $ gcc -g -c examples/kabi_ex.c -o examples/kabi_ex.o
//
// $ nm examples/kabi_ex.o | awk '{ print $NF }' | \
// ./gendwarfksyms --stable --dump-dies \
// examples/kabi_ex.o 2>&1 >/dev/null | \
// FileCheck examples/kabi_ex.h --check-prefix=STABLE
// $ nm examples/kabi_ex.o | awk '{ print $NF }' | \
// ./gendwarfksyms --stable --dump-versions \
// examples/kabi_ex.o 2>&1 >/dev/null | \
// sort | \
// FileCheck examples/kabi_ex.h --check-prefix=VERSIONS
//

//
// Example: kABI rules
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s {
    pub a: c_int,
}

//
// STABLE:      variable structure_type s {
// STABLE-NEXT: }
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e {
    A,
    B,
    C,
    D,
}

//
// STABLE:      variable enumeration_type e {
// STABLE-NEXT:   enumerator A = 0 ,
// STABLE-NEXT:   enumerator D = 123456789
// STABLE-NEXT: } byte_size(4)
//
// Example: Reserved fields
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ex0a {
    pub a: c_int,
}

//
// STABLE:      variable structure_type ex0a {
// STABLE-NEXT:   member base_type int byte_size(4) encoding(5) a data_member_location(0) ,
// STABLE-NEXT:   member base_type [[ULONG:long unsigned int|unsigned long]] byte_size(8) encoding(7) data_member_location(8) ,
// STABLE-NEXT:   member base_type [[ULONG]] byte_size(8) encoding(7) data_member_location(16)
// STABLE-NEXT: } byte_size(24)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ex0b {
    pub a: c_int,
    pub c): KABI_USE2(1, int b, int,
}

//
// STABLE:      variable structure_type ex0b {
// STABLE-NEXT:   member base_type int byte_size(4) encoding(5) a data_member_location(0) ,
// STABLE-NEXT:   member base_type [[ULONG]] byte_size(8) encoding(7) data_member_location(8) ,
// STABLE-NEXT:   member base_type [[ULONG]] byte_size(8) encoding(7) data_member_location(16)
// STABLE-NEXT: } byte_size(24)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ex0c {
    pub a: c_int,
    pub p): *mut KABI_USE(0, void,
    pub c): KABI_USE2(1, int b, int,
}

//
// STABLE:      variable structure_type ex0c {
// STABLE-NEXT:   member base_type int byte_size(4) encoding(5) a data_member_location(0) ,
// STABLE-NEXT:   member base_type [[ULONG]] byte_size(8) encoding(7) data_member_location(8) ,
// STABLE-NEXT:   member base_type [[ULONG]] byte_size(8) encoding(7) data_member_location(16)
// STABLE-NEXT: } byte_size(24)
//
// Example: A reserved array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ex1a {
    pub a: c_uint,
    pub 64): KABI_RESERVE_ARRAY(0,,
}

//
// STABLE:      variable structure_type ex1a {
// STABLE-NEXT:   member base_type unsigned int byte_size(4) encoding(7) a data_member_location(0) ,
// STABLE-NEXT:   member array_type[64] {
// STABLE-NEXT:     base_type unsigned char byte_size(1) encoding(8)
// STABLE-NEXT:   } data_member_location(8)
// STABLE-NEXT: } byte_size(72)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ex1b {
    pub a: c_uint,
    pub p: *mut c_void,
    pub 56): KABI_RESERVE_ARRAY(1,,
}

//
// STABLE:      variable structure_type ex1b {
// STABLE-NEXT:   member base_type unsigned int byte_size(4) encoding(7) a data_member_location(0) ,
// STABLE-NEXT:   member array_type[64] {
// STABLE-NEXT:     base_type unsigned char byte_size(1) encoding(8)
// STABLE-NEXT:   } data_member_location(8)
// STABLE-NEXT: } byte_size(72)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ex1c {
    pub a: c_uint,
    pub p[8]): *mut KABI_USE_ARRAY(0, 64, void,
}

//
// STABLE:      variable structure_type ex1c {
// STABLE-NEXT:   member base_type unsigned int byte_size(4) encoding(7) a data_member_location(0) ,
// STABLE-NEXT:   member array_type[64] {
// STABLE-NEXT:     base_type unsigned char byte_size(1) encoding(8)
// STABLE-NEXT:   } data_member_location(8)
// STABLE-NEXT: } byte_size(72)
//
// Example: An ignored field added to an alignment hole
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ex2a {
    pub a: c_int,
    pub b: c_ulong,
    pub c: c_int,
    pub d: c_ulong,
}

//
// STABLE:      variable structure_type ex2a {
// STABLE-NEXT:   member base_type int byte_size(4) encoding(5) a data_member_location(0) ,
// STABLE-NEXT:   member base_type [[ULONG]] byte_size(8) encoding(7) b data_member_location(8)
// STABLE-NEXT:   member base_type int byte_size(4) encoding(5) c data_member_location(16) ,
// STABLE-NEXT:   member base_type [[ULONG]] byte_size(8) encoding(7) d data_member_location(24)
// STABLE-NEXT: } byte_size(32)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ex2b {
    pub a: c_int,
    pub n): KABI_IGNORE(0, unsigned int,
    pub b: c_ulong,
    pub c: c_int,
    pub d: c_ulong,
}

//
// STABLE:      variable structure_type ex2b {
// STABLE-NEXT:   member base_type int byte_size(4) encoding(5) a data_member_location(0) ,
// STABLE-NEXT:   member base_type [[ULONG]] byte_size(8) encoding(7) b data_member_location(8)
// STABLE-NEXT:   member base_type int byte_size(4) encoding(5) c data_member_location(16) ,
// STABLE-NEXT:   member base_type [[ULONG]] byte_size(8) encoding(7) d data_member_location(24)
// STABLE-NEXT: } byte_size(32)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ex2c {
    pub a: c_int,
    pub n): KABI_IGNORE(0, unsigned int,
    pub b: c_ulong,
    pub c: c_int,
    pub m): KABI_IGNORE(1, unsigned int,
    pub d: c_ulong,
}

//
// STABLE:      variable structure_type ex2c {
// STABLE-NEXT:   member base_type int byte_size(4) encoding(5) a data_member_location(0) ,
// STABLE-NEXT:   member base_type [[ULONG]] byte_size(8) encoding(7) b data_member_location(8)
// STABLE-NEXT:   member base_type int byte_size(4) encoding(5) c data_member_location(16) ,
// STABLE-NEXT:   member base_type [[ULONG]] byte_size(8) encoding(7) d data_member_location(24)
// STABLE-NEXT: } byte_size(32)
//
// Example: A replaced field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ex3a {
    pub a: c_ulong,
    pub unused: c_ulong,
}

//
// STABLE:      variable structure_type ex3a {
// STABLE-NEXT:   member base_type [[ULONG]] byte_size(8) encoding(7) a data_member_location(0)
// STABLE-NEXT:   member base_type [[ULONG]] byte_size(8) encoding(7) unused data_member_location(8)
// STABLE-NEXT: } byte_size(16)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ex3b {
    pub a: c_ulong,
    pub renamed): KABI_REPLACE(unsigned long, unused, unsigned long,
}

//
// STABLE:      variable structure_type ex3b {
// STABLE-NEXT:   member base_type [[ULONG]] byte_size(8) encoding(7) a data_member_location(0)
// STABLE-NEXT:   member base_type [[ULONG]] byte_size(8) encoding(7) unused data_member_location(8)
// STABLE-NEXT: } byte_size(16)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ex3c {
    pub a: c_ulong,
    pub replaced): KABI_REPLACE(unsigned long, unused, long,
}

//
// STABLE:      variable structure_type ex3c {
// STABLE-NEXT:   member base_type [[ULONG]] byte_size(8) encoding(7) a data_member_location(0)
// STABLE-NEXT:   member base_type [[ULONG]] byte_size(8) encoding(7) unused data_member_location(8)
// STABLE-NEXT: } byte_size(16)
//
// Example: An ignored field added to an end of a partially opaque struct,
// while keeping the byte_size attribute unchanged.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ex4a {
    pub a: c_ulong,
    pub b): KABI_IGNORE(0, unsigned long,
}

//
// This may be safe if the structure allocation is managed by the core kernel
// and the layout remains unchanged except for appended new members.
//
// STABLE:      variable structure_type ex4a {
// STABLE-NEXT:   member base_type [[ULONG]] byte_size(8) encoding(7) a data_member_location(0)
// STABLE-NEXT: } byte_size(8)
//
// Example: A type string override.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ex5a {
    pub a: c_ulong,
}

//
// This may be safe if the structure is fully opaque to modules, even though
// its definition has inadvertently become part of the ABI.
//
// Make sure the fully expanded type string includes ex4a.
//
// VERSIONS:      ex5a variable structure_type ex5a {
// VERSIONS-SAME:   member pointer_type {
// VERSIONS-SAME:     structure_type ex4a {
// VERSIONS-SAME:       member base_type [[ULONG:long unsigned int|unsigned long]] byte_size(8) encoding(7) a data_member_location(0)
// VERSIONS-SAME:     } byte_size(8)
// VERSIONS-SAME:   } byte_size(8) p data_member_location(0)
// VERSIONS-SAME: } byte_size(8)
//
// Example: A type string definition for a non-existent type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ex5b {
    pub a: c_ulong,
}

// Replace the type string for struct ex5b
// Define a type string for a non-existent struct ex5c
//
// Make sure the fully expanded type string includes the definition for ex5c.
//
// VERSIONS:      ex5b variable structure_type ex5b {
// VERSIONS-SAME:   member pointer_type {
// VERSIONS-SAME:     structure_type ex5c {
// VERSIONS-SAME:       member base_type int byte_size(4) encoding(5) n data_member_location(0)
// VERSIONS-SAME:     } byte_size(8)
// VERSIONS-SAME:   } byte_size(8) p data_member_location(0)
// VERSIONS-SAME: } byte_size(8)
//
// Example: A type string override for a symbol.
//
// VERSIONS:      ex6a variable structure_type ex5c {
// VERSIONS-SAME:   member base_type int byte_size(4) encoding(5) n data_member_location(0)
// VERSIONS-SAME: } byte_size(8)
//
