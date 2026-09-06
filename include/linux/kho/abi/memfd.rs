//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kho/abi/memfd.h
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
// Copyright (c) 2025, Google LLC.
// Pasha Tatashin <pasha.tatashin@soleen.com>
//
// Copyright (C) 2025 Amazon.com Inc. or its affiliates.
// Pratyush Yadav <ptyadav@amazon.de>
//

//
// DOC: memfd Live Update ABI
//
// memfd uses the ABI defined below for preserving its state across a kexec
// reboot using the LUO.
//
// The state is serialized into a packed structure `struct memfd_luo_ser`
// which is handed over to the next kernel via the KHO mechanism.
//
// This interface is a contract. Any modification to the structure layout
// constitutes a breaking change. Such changes require incrementing the
// version number in the MEMFD_LUO_FH_COMPATIBLE string.
//
// MEMFD_LUO_FOLIO_DIRTY - The folio is dirty.
//
// This flag indicates the folio contains data from user. A non-dirty folio is
// one that was allocated (say using fallocate(2)) but not written to.
//

//
// MEMFD_LUO_FOLIO_UPTODATE - The folio is up-to-date.
//
// An up-to-date folio has been zeroed out. shmem zeroes out folios on first
// use. This flag tracks which folios need zeroing.
//

//
// struct memfd_luo_folio_ser - Serialized state of a single folio.
// @pfn:       The page frame number of the folio.
// @flags:     Flags to describe the state of the folio.
// @index:     The page offset (pgoff_t) of the folio within the original file.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memfd_luo_folio_ser {
    pub pfn:52: u64,
    pub flags:12: u64,
    pub index: u64,
    pub __packed: },
//
// The set of seals this version supports preserving. If support for any new
// seals is needed, add it here and bump version.
//

//
// struct memfd_luo_ser - Main serialization structure for a memfd.
// @pos:       The file's current position (f_pos).
// @size:      The total size of the file in bytes (i_size).
// @seals:     The seals present on the memfd. The seals are uABI so it is safe
// to directly use them in the ABI.
// @flags:     Flags for the file. Unused flag bits must be set to 0.
// @nr_folios: Number of folios in the folios array.
// @folios:    KHO vmalloc descriptor pointing to the array of
// struct memfd_luo_folio_ser.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memfd_luo_ser {
    pub pos: u64,
    pub size: u64,
    pub seals: u32,
    pub flags: u32,
    pub nr_folios: u64,
    pub folios: kho_vmalloc,
    pub __packed: },
// The compatibility string for memfd file handler

