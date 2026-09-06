//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic94xx/aic94xx_seq.h
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
// Aic94xx SAS/SATA driver sequencer interface header file.
//
// Copyright (C) 2005 Adaptec, Inc.  All rights reserved.
// Copyright (C) 2005 Luben Tuikov <luben_tuikov@adaptec.com>
//
pub const CSEQ_NUM_VECS: c_int = 3;
pub const LSEQ_NUM_VECS: c_int = 11;

pub const SAS_RAZOR_SEQUENCER_FW_MAJOR: c_int = 1;
// Note:  All quantites in the sequencer file are little endian
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sequencer_file_header {
// Checksum of the entire contents of the sequencer excluding
// these four bytes
    pub csum: u32,
// numeric major version
    pub major: u32,
// numeric minor version
    pub minor: u32,
// version string printed by driver
    pub version: [c_char; 16],
    pub cseq_table_offset: u32,
    pub cseq_table_size: u32,
    pub lseq_table_offset: u32,
    pub lseq_table_size: u32,
    pub cseq_code_offset: u32,
    pub cseq_code_size: u32,
    pub lseq_code_offset: u32,
    pub lseq_code_size: u32,
    pub mode2_task: u16,
    pub cseq_idle_loop: u16,
    pub lseq_idle_loop: u16,
    pub __attribute__((packed)): },

    pub asd_ha): *mut int asd_init_seqs(struct asd_ha_struct,
    pub asd_ha): *mut int asd_start_seqs(struct asd_ha_struct,
    pub asd_release_firmware(void): c_int,
    pub phy): *mut *mut void asd_update_port_links(struct asd_ha_struct asd_ha, struct asd_phy,

