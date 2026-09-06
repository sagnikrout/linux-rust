//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/ipc4-telemetry.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2023 Intel Corporation
//
// Target code
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_coredump_tgt_code {
    COREDUMP_TGT_UNKNOWN = 0,
    COREDUMP_TGT_X86,
    COREDUMP_TGT_X86_64,
    COREDUMP_TGT_ARM_CORTEX_M,
    COREDUMP_TGT_RISC_V,
    COREDUMP_TGT_XTENSA,
}

pub const XTENSA_BLOCK_HDR_VER: c_int = 2;
pub const XTENSA_CORE_DUMP_SEPARATOR: c_uint = 0x0DEC0DEB;
pub const XTENSA_CORE_AR_REGS_COUNT: c_int = 16;
pub const XTENSA_SOC_INTEL_ADSP: c_int = 3;
pub const XTENSA_TOOL_CHAIN_ZEPHYR: c_int = 1;
pub const XTENSA_TOOL_CHAIN_XCC: c_int = 2;
// Coredump header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_coredump_hdr {
// 'Z', 'E' as identifier of file
    pub id: [c_char; 2],
// Identify the version of the header
    pub hdr_version: u16,
// Indicate which target (e.g. architecture or SoC)
    pub tgt_code: u16,
// Size of uintptr_t in power of 2. (e.g. 5 for 32-bit, 6 for 64-bit)
    pub ptr_size_bits: u8,
    pub flag: u8,
// Reason for the fatal error
    pub reason: u32,
    pub __packed: },
// Architecture-specific block header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_coredump_arch_hdr {
// COREDUMP_ARCH_HDR_ID to indicate this is a architecture-specific block
    pub id: c_char,
// Identify the version of this block
    pub hdr_version: u16,
// Number of bytes following the header
    pub num_bytes: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_telemetry_slot_data {
    pub separator: u32,
    pub hdr: sof_ipc4_coredump_hdr,
    pub arch_hdr: sof_ipc4_coredump_arch_hdr,
    pub arch_data: [u32; ],
    pub __packed: },
    pub sdev): *mut void sof_ipc4_create_exception_debugfs_node(struct snd_sof_dev,
