//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ipa/ipa_mem.h
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
// Copyright (c) 2012-2018, The Linux Foundation. All rights reserved.
// Copyright (C) 2019-2024 Linaro Ltd.
//

//
// DOC: IPA Local Memory
//
// The IPA has a block of shared memory, divided into regions used for
// specific purposes.
//
// The regions within the shared block are bounded by an offset (relative to
// the "ipa-shared" memory range) and size found in the IPA_SHARED_MEM_SIZE
// register.
//
// Each region is optionally preceded by one or more 32-bit "canary" values.
// These are meant to detect out-of-range writes (if they become corrupted).
// A given region (such as a filter or routing table) has the same number
// of canaries for all IPA hardware versions.  Still, the number used is
// defined in the config data, allowing for generic handling of regions.
//
// The set of memory regions is defined in configuration data.  They are
// subject to these constraints:
// - a zero offset and zero size represents and undefined region
// - a region's size does not include space for its "canary" values
// - a region's offset is defined to be *past* all "canary" values
// - offset must be large enough to account for all canaries
// - a region's size may be zero, but may still have canaries
// - all offsets must be 8-byte aligned
// - most sizes must be a multiple of 8
// - modem memory size must be a multiple of 4
// - the microcontroller ring offset must be a multiple of 1024
//
// The maximum allowed size for any memory region

// IPA-resident memory region ids
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipa_mem_id {
    IPA_MEM_UC_SHARED,		/* 0 canaries */
    IPA_MEM_UC_INFO,		/* 0 canaries */
    IPA_MEM_V4_FILTER_HASHED,	/* 2 canaries */
    IPA_MEM_V4_FILTER,		/* 2 canaries */
    IPA_MEM_V6_FILTER_HASHED,	/* 2 canaries */
    IPA_MEM_V6_FILTER,		/* 2 canaries */
    IPA_MEM_V4_ROUTE_HASHED,	/* 2 canaries */
    IPA_MEM_V4_ROUTE,		/* 2 canaries */
    IPA_MEM_V6_ROUTE_HASHED,	/* 2 canaries */
    IPA_MEM_V6_ROUTE,		/* 2 canaries */
    IPA_MEM_MODEM_HEADER,		/* 2 canaries */
    IPA_MEM_AP_HEADER,		/* 0 canaries, optional */
    IPA_MEM_MODEM_PROC_CTX,		/* 2 canaries */
    IPA_MEM_AP_PROC_CTX,		/* 0 canaries */
    IPA_MEM_MODEM,			/* 0/2 canaries */
    IPA_MEM_UC_EVENT_RING,		/* 1 canary, optional */
    IPA_MEM_PDN_CONFIG,		/* 0/2 canaries (IPA v4.0+) */
    IPA_MEM_STATS_QUOTA_MODEM,	/* 2/4 canaries (IPA v4.0+) */
    IPA_MEM_STATS_QUOTA_AP,		/* 0 canaries, optional (IPA v4.0+) */
    IPA_MEM_STATS_TETHERING,	/* 0 canaries, optional (IPA v4.0+) */
    IPA_MEM_STATS_DROP,		/* 0 canaries, optional (IPA v4.0+) */
// The next 7 filter and route statistics regions are optional
    IPA_MEM_STATS_V4_FILTER,	/* 0 canaries (IPA v4.0-v4.2) */
    IPA_MEM_STATS_V6_FILTER,	/* 0 canaries (IPA v4.0-v4.2) */
    IPA_MEM_STATS_V4_ROUTE,		/* 0 canaries (IPA v4.0-v4.2) */
    IPA_MEM_STATS_V6_ROUTE,		/* 0 canaries (IPA v4.0-v4.2) */
    IPA_MEM_AP_V4_FILTER,		/* 2 canaries (IPA v5.0) */
    IPA_MEM_AP_V6_FILTER,		/* 0 canaries (IPA v5.0) */
    IPA_MEM_STATS_FILTER_ROUTE,	/* 0 canaries (IPA v4.5+) */
    IPA_MEM_NAT_TABLE,		/* 4 canaries, optional (IPA v4.5+) */
    IPA_MEM_END_MARKER,		/* 1 canary (not a real region) */
    IPA_MEM_COUNT,			/* Number of regions (not an index) */
}

//
// struct ipa_mem - IPA local memory region description
// @id:			memory region identifier
// @offset:		offset in IPA memory space to base of the region
// @size:		size in bytes base of the region
// @canary_count:	Number of 32-bit "canary" values that precede region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_mem {
    pub id: ipa_mem_id,
    pub offset: u32,
    pub size: u16,
    pub canary_count: u16,
}

extern "C" {
    pub fn ipa_mem_config(ipa: *mut ipa) -> c_int;
}
extern "C" {
    pub fn ipa_mem_deconfig(ipa: *mut ipa);
}
extern "C" {
    pub fn ipa_mem_zero_modem(ipa: *mut ipa) -> c_int;
}
extern "C" {
    pub fn ipa_mem_exit(ipa: *mut ipa);
}
