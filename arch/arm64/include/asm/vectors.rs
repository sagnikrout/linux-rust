//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/vectors.h
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
// Copyright (C) 2022 ARM Ltd.
//

//
// Note: the order of this enum corresponds to two arrays in entry.S:
// tramp_vecs and __bp_harden_el1_vectors. By default the canonical
// 'full fat' vectors are used directly.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm64_bp_harden_el1_vectors {

//
// Perform the BHB loop mitigation, before branching to the canonical
// vectors.
//
    EL1_VECTOR_BHB_LOOP,

//
// Make the SMC call for firmware mitigation, before branching to the
// canonical vectors.
//
    EL1_VECTOR_BHB_FW,

//
// Use the ClearBHB instruction, before branching to the canonical
// vectors.
//
    EL1_VECTOR_BHB_CLEAR_INSN,

//
// Remap the kernel before branching to the canonical vectors.
//
    EL1_VECTOR_KPTI,
}

// The vectors to use on return from EL0. e.g. to remap the kernel

