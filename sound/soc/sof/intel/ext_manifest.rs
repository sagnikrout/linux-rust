//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/intel/ext_manifest.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2020 Intel Corporation
//
// Intel extended manifest is a extra place to store Intel cavs specific
// metadata about firmware, for example LPRO/HPRO configuration is
// Intel cavs specific. This part of output binary is not signed.
//

// EXT_MAN_ELEM_PLATFORM_CONFIG_DATA elements identificators
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_cavs_config_elem_type {
    SOF_EXT_MAN_CAVS_CONFIG_EMPTY		= 0,
    SOF_EXT_MAN_CAVS_CONFIG_CAVS_LPRO	= 1,
    SOF_EXT_MAN_CAVS_CONFIG_OUTBOX_SIZE	= 2,
    SOF_EXT_MAN_CAVS_CONFIG_INBOX_SIZE	= 3,
}

// EXT_MAN_ELEM_PLATFORM_CONFIG_DATA elements
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ext_man_cavs_config_data {
    pub hdr: sof_ext_man_elem_header,
    pub elems: [sof_config_elem; ],
    pub __packed: },
