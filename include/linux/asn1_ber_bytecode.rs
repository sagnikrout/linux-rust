//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/asn1_ber_bytecode.h
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
// ASN.1 BER/DER/CER parsing state machine internal definitions
//
// Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asn1_decoder {
    pub machine: *const c_uchar,
    pub machlen: usize,
    pub actions: *const asn1_action_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum asn1_opcode {
// The tag-matching ops come first and the odd-numbered slots
// are for OR_SKIP ops.
//
pub const ASN1_OP_MATCH__SKIP: c_uint = 0x01;
pub const ASN1_OP_MATCH__ACT: c_uint = 0x02;
pub const ASN1_OP_MATCH__JUMP: c_uint = 0x04;
pub const ASN1_OP_MATCH__ANY: c_uint = 0x08;
pub const ASN1_OP_MATCH__COND: c_uint = 0x10;

    ASN1_OP_MATCH			= 0x00,
    ASN1_OP_MATCH_OR_SKIP		= 0x01,
    ASN1_OP_MATCH_ACT		= 0x02,
    ASN1_OP_MATCH_ACT_OR_SKIP	= 0x03,
    ASN1_OP_MATCH_JUMP		= 0x04,
    ASN1_OP_MATCH_JUMP_OR_SKIP	= 0x05,
    ASN1_OP_MATCH_ANY		= 0x08,
    ASN1_OP_MATCH_ANY_OR_SKIP	= 0x09,
    ASN1_OP_MATCH_ANY_ACT		= 0x0a,
    ASN1_OP_MATCH_ANY_ACT_OR_SKIP	= 0x0b,
// Everything before here matches unconditionally

    ASN1_OP_COND_MATCH_OR_SKIP	= 0x11,
    ASN1_OP_COND_MATCH_ACT_OR_SKIP	= 0x13,
    ASN1_OP_COND_MATCH_JUMP_OR_SKIP	= 0x15,
    ASN1_OP_COND_MATCH_ANY		= 0x18,
    ASN1_OP_COND_MATCH_ANY_OR_SKIP	= 0x19,
    ASN1_OP_COND_MATCH_ANY_ACT	= 0x1a,
    ASN1_OP_COND_MATCH_ANY_ACT_OR_SKIP = 0x1b,

// Everything before here will want a tag from the data

// These are here to help fill up space
    ASN1_OP_COND_FAIL		= 0x1c,
    ASN1_OP_COMPLETE		= 0x1d,
    ASN1_OP_ACT			= 0x1e,
    ASN1_OP_MAYBE_ACT		= 0x1f,

// The following eight have bit 0 -> SET, 1 -> OF, 2 -> ACT
    ASN1_OP_END_SEQ			= 0x20,
    ASN1_OP_END_SET			= 0x21,
    ASN1_OP_END_SEQ_OF		= 0x22,
    ASN1_OP_END_SET_OF		= 0x23,
    ASN1_OP_END_SEQ_ACT		= 0x24,
    ASN1_OP_END_SET_ACT		= 0x25,
    ASN1_OP_END_SEQ_OF_ACT		= 0x26,
    ASN1_OP_END_SET_OF_ACT		= 0x27,
pub const ASN1_OP_END__SET: c_uint = 0x01;
pub const ASN1_OP_END__OF: c_uint = 0x02;
pub const ASN1_OP_END__ACT: c_uint = 0x04;

    ASN1_OP_RETURN			= 0x28,

    ASN1_OP__NR
}

