//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/microchip/sparx5/sparx5_tc.h
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


// SPDX-License-Identifier: GPL-2.0+
// Microchip Sparx5 Switch driver
//
// Copyright (c) 2022 Microchip Technology Inc. and its subsidiaries.
//

// Controls how PORT_MASK is applied
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SPX5_PORT_MASK_MODE {
    SPX5_PMM_OR_DSTMASK,
    SPX5_PMM_AND_VLANMASK,
    SPX5_PMM_REPLACE_PGID,
    SPX5_PMM_REPLACE_ALL,
    SPX5_PMM_REDIR_PGID,
    SPX5_PMM_OR_PGID_MASK,
}

// Controls ES0 forwarding
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SPX5_FORWARDING_SEL {
    SPX5_FWSEL_NO_ACTION,
    SPX5_FWSEL_COPY_TO_LOOPBACK,
    SPX5_FWSEL_REDIRECT_TO_LOOPBACK,
    SPX5_FWSEL_DISCARD,
}

// Controls tag A (outer tagging)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SPX5_OUTER_TAG_SEL {
    SPX5_OTAG_PORT,
    SPX5_OTAG_TAG_A,
    SPX5_OTAG_FORCED_PORT,
    SPX5_OTAG_UNTAG,
}

// Selects TPID for ES0 tag A
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SPX5_TPID_A_SEL {
    SPX5_TPID_A_8100,
    SPX5_TPID_A_88A8,
    SPX5_TPID_A_CUST1,
    SPX5_TPID_A_CUST2,
    SPX5_TPID_A_CUST3,
    SPX5_TPID_A_CLASSIFIED,
}

// Selects VID for ES0 tag A
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SPX5_VID_A_SEL {
    SPX5_VID_A_CLASSIFIED,
    SPX5_VID_A_VAL,
    SPX5_VID_A_IFH,
    SPX5_VID_A_RESERVED,
}

// Select PCP source for ES0 tag A
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SPX5_PCP_A_SEL {
    SPX5_PCP_A_CLASSIFIED,
    SPX5_PCP_A_VAL,
    SPX5_PCP_A_RESERVED,
    SPX5_PCP_A_POPPED,
    SPX5_PCP_A_MAPPED_0,
    SPX5_PCP_A_MAPPED_1,
    SPX5_PCP_A_MAPPED_2,
    SPX5_PCP_A_MAPPED_3,
}

// Select DEI source for ES0 tag A
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SPX5_DEI_A_SEL {
    SPX5_DEI_A_CLASSIFIED,
    SPX5_DEI_A_VAL,
    SPX5_DEI_A_REW,
    SPX5_DEI_A_POPPED,
    SPX5_DEI_A_MAPPED_0,
    SPX5_DEI_A_MAPPED_1,
    SPX5_DEI_A_MAPPED_2,
    SPX5_DEI_A_MAPPED_3,
}

// Controls tag B (inner tagging)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SPX5_INNER_TAG_SEL {
    SPX5_ITAG_NO_PUSH,
    SPX5_ITAG_PUSH_B_TAG,
}

// Selects TPID for ES0 tag B.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SPX5_TPID_B_SEL {
    SPX5_TPID_B_8100,
    SPX5_TPID_B_88A8,
    SPX5_TPID_B_CUST1,
    SPX5_TPID_B_CUST2,
    SPX5_TPID_B_CUST3,
    SPX5_TPID_B_CLASSIFIED,
}
