//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/asn1.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// ASN.1 BER/DER/CER encoding definitions
//
// Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
// Class
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum asn1_class {
    ASN1_UNIV	= 0,	/* Universal */
    ASN1_APPL	= 1,	/* Application */
    ASN1_CONT	= 2,	/* Context */
    ASN1_PRIV	= 3	/* Private */
}

pub const ASN1_CLASS_BITS: c_uint = 0xc0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum asn1_method {
    ASN1_PRIM	= 0,	/* Primitive */
    ASN1_CONS	= 1	/* Constructed */
}

pub const ASN1_CONS_BIT: c_uint = 0x20;
// Tag
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum asn1_tag {
    ASN1_EOC	= 0,	/* End Of Contents or N/A */
    ASN1_BOOL	= 1,	/* Boolean */
    ASN1_INT	= 2,	/* Integer */
    ASN1_BTS	= 3,	/* Bit String */
    ASN1_OTS	= 4,	/* Octet String */
    ASN1_NULL	= 5,	/* Null */
    ASN1_OID	= 6,	/* Object Identifier  */
    ASN1_ODE	= 7,	/* Object Description */
    ASN1_EXT	= 8,	/* External */
    ASN1_REAL	= 9,	/* Real float */
    ASN1_ENUM	= 10,	/* Enumerated */
    ASN1_EPDV	= 11,	/* Embedded PDV */
    ASN1_UTF8STR	= 12,	/* UTF8 String */
    ASN1_RELOID	= 13,	/* Relative OID */
// 14 - Reserved
// 15 - Reserved
    ASN1_SEQ	= 16,	/* Sequence and Sequence of */
    ASN1_SET	= 17,	/* Set and Set of */
    ASN1_NUMSTR	= 18,	/* Numerical String */
    ASN1_PRNSTR	= 19,	/* Printable String */
    ASN1_TEXSTR	= 20,	/* T61 String / Teletext String */
    ASN1_VIDSTR	= 21,	/* Videotex String */
    ASN1_IA5STR	= 22,	/* IA5 String */
    ASN1_UNITIM	= 23,	/* Universal Time */
    ASN1_GENTIM	= 24,	/* General Time */
    ASN1_GRASTR	= 25,	/* Graphic String */
    ASN1_VISSTR	= 26,	/* Visible String */
    ASN1_GENSTR	= 27,	/* General String */
    ASN1_UNISTR	= 28,	/* Universal String */
    ASN1_CHRSTR	= 29,	/* Character String */
    ASN1_BMPSTR	= 30,	/* BMP String */
    ASN1_LONG_TAG	= 31	/* Long form tag */
}

pub const ASN1_INDEFINITE_LENGTH: c_uint = 0x80;
