//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/kvm_host.h
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

pub const PGM_OPERATION: c_uint = 0x01;
pub const PGM_PRIVILEGED_OP: c_uint = 0x02;
pub const PGM_EXECUTE: c_uint = 0x03;
pub const PGM_PROTECTION: c_uint = 0x04;
pub const PGM_ADDRESSING: c_uint = 0x05;
pub const PGM_SPECIFICATION: c_uint = 0x06;
pub const PGM_DATA: c_uint = 0x07;
pub const PGM_FIXED_POINT_OVERFLOW: c_uint = 0x08;
pub const PGM_FIXED_POINT_DIVIDE: c_uint = 0x09;
pub const PGM_DECIMAL_OVERFLOW: c_uint = 0x0a;
pub const PGM_DECIMAL_DIVIDE: c_uint = 0x0b;
pub const PGM_HFP_EXPONENT_OVERFLOW: c_uint = 0x0c;
pub const PGM_HFP_EXPONENT_UNDERFLOW: c_uint = 0x0d;
pub const PGM_HFP_SIGNIFICANCE: c_uint = 0x0e;
pub const PGM_HFP_DIVIDE: c_uint = 0x0f;
pub const PGM_SEGMENT_TRANSLATION: c_uint = 0x10;
pub const PGM_PAGE_TRANSLATION: c_uint = 0x11;
pub const PGM_TRANSLATION_SPEC: c_uint = 0x12;
pub const PGM_SPECIAL_OPERATION: c_uint = 0x13;
pub const PGM_OPERAND: c_uint = 0x15;
pub const PGM_TRACE_TABEL: c_uint = 0x16;
pub const PGM_VECTOR_PROCESSING: c_uint = 0x1b;
pub const PGM_SPACE_SWITCH: c_uint = 0x1c;
pub const PGM_HFP_SQUARE_ROOT: c_uint = 0x1d;
pub const PGM_PC_TRANSLATION_SPEC: c_uint = 0x1f;
pub const PGM_AFX_TRANSLATION: c_uint = 0x20;
pub const PGM_ASX_TRANSLATION: c_uint = 0x21;
pub const PGM_LX_TRANSLATION: c_uint = 0x22;
pub const PGM_EX_TRANSLATION: c_uint = 0x23;
pub const PGM_PRIMARY_AUTHORITY: c_uint = 0x24;
pub const PGM_SECONDARY_AUTHORITY: c_uint = 0x25;
pub const PGM_LFX_TRANSLATION: c_uint = 0x26;
pub const PGM_LSX_TRANSLATION: c_uint = 0x27;
pub const PGM_ALET_SPECIFICATION: c_uint = 0x28;
pub const PGM_ALEN_TRANSLATION: c_uint = 0x29;
pub const PGM_ALE_SEQUENCE: c_uint = 0x2a;
pub const PGM_ASTE_VALIDITY: c_uint = 0x2b;
pub const PGM_ASTE_SEQUENCE: c_uint = 0x2c;
pub const PGM_EXTENDED_AUTHORITY: c_uint = 0x2d;
pub const PGM_LSTE_SEQUENCE: c_uint = 0x2e;
pub const PGM_ASTE_INSTANCE: c_uint = 0x2f;
pub const PGM_STACK_FULL: c_uint = 0x30;
pub const PGM_STACK_EMPTY: c_uint = 0x31;
pub const PGM_STACK_SPECIFICATION: c_uint = 0x32;
pub const PGM_STACK_TYPE: c_uint = 0x33;
pub const PGM_STACK_OPERATION: c_uint = 0x34;
pub const PGM_ASCE_TYPE: c_uint = 0x38;
pub const PGM_REGION_FIRST_TRANS: c_uint = 0x39;
pub const PGM_REGION_SECOND_TRANS: c_uint = 0x3a;
pub const PGM_REGION_THIRD_TRANS: c_uint = 0x3b;
pub const PGM_SECURE_STORAGE_ACCESS: c_uint = 0x3d;
pub const PGM_NON_SECURE_STORAGE_ACCESS: c_uint = 0x3e;
pub const PGM_SECURE_STORAGE_VIOLATION: c_uint = 0x3f;
pub const PGM_MONITOR: c_uint = 0x40;
pub const PGM_PER: c_uint = 0x80;
pub const PGM_CRYPTO_OPERATION: c_uint = 0x119;
