//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/pci_dma.h
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
// I/O Translation Anchor (IOTA)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zpci_ioat_dtype {
    ZPCI_IOTA_STO = 0,
    ZPCI_IOTA_RTTO = 1,
    ZPCI_IOTA_RSTO = 2,
    ZPCI_IOTA_RFTO = 3,
    ZPCI_IOTA_PFAA = 4,
    ZPCI_IOTA_IOPFAA = 5,
    ZPCI_IOTA_IOPTO = 7
}

pub const ZPCI_IOTA_IOT_ENABLED: c_uint = 0x800UL;

pub const ZPCI_IOTA_FS_4K: c_int = 0;
pub const ZPCI_IOTA_FS_1M: c_int = 1;
pub const ZPCI_IOTA_FS_2G: c_int = 2;

// I/O Region and segment tables
pub const ZPCI_INDEX_MASK: c_uint = 0x7ffUL;
pub const ZPCI_TABLE_TYPE_MASK: c_uint = 0xc;
pub const ZPCI_TABLE_TYPE_RFX: c_uint = 0xc;
pub const ZPCI_TABLE_TYPE_RSX: c_uint = 0x8;
pub const ZPCI_TABLE_TYPE_RTX: c_uint = 0x4;
pub const ZPCI_TABLE_TYPE_SX: c_uint = 0x0;
pub const ZPCI_TABLE_LEN_RFX: c_uint = 0x3;
pub const ZPCI_TABLE_LEN_RSX: c_uint = 0x3;
pub const ZPCI_TABLE_LEN_RTX: c_uint = 0x3;
pub const ZPCI_TABLE_OFFSET_MASK: c_uint = 0xc0;
pub const ZPCI_TABLE_SIZE: c_uint = 0x4000;

pub const ZPCI_TABLE_BITS: c_int = 11;
pub const ZPCI_PT_BITS: c_int = 8;

pub const ZPCI_RTE_FLAG_MASK: c_uint = 0x3fffUL;

pub const ZPCI_STE_FLAG_MASK: c_uint = 0x7ffUL;

// I/O Page tables
pub const ZPCI_PTE_VALID_MASK: c_uint = 0x400;
pub const ZPCI_PTE_INVALID: c_uint = 0x400;
pub const ZPCI_PTE_VALID: c_uint = 0x000;
pub const ZPCI_PT_SIZE: c_uint = 0x800;

pub const ZPCI_PTE_FLAG_MASK: c_uint = 0xfffUL;

// Shared bits
pub const ZPCI_TABLE_VALID: c_uint = 0x00;
pub const ZPCI_TABLE_INVALID: c_uint = 0x20;
pub const ZPCI_TABLE_PROTECTED: c_uint = 0x200;
pub const ZPCI_TABLE_UNPROTECTED: c_uint = 0x000;
pub const ZPCI_TABLE_VALID_MASK: c_uint = 0x20;
pub const ZPCI_TABLE_PROT_MASK: c_uint = 0x200;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_iommu_ctrs {
    pub mapped_pages: core::sync::atomic::AtomicI64,
    pub unmapped_pages: core::sync::atomic::AtomicI64,
    pub global_rpcits: core::sync::atomic::AtomicI64,
    pub sync_map_rpcits: core::sync::atomic::AtomicI64,
    pub sync_rpcits: core::sync::atomic::AtomicI64,
}
