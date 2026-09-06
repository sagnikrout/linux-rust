//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/mtd/inftl-user.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Parts of INFTL headers shared with userspace
//

pub const OSAK_VERSION: c_uint = 0x5120;
pub const PERCENTUSED: c_int = 98;
pub const SECTORSIZE: c_int = 512;
// Block Control Information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inftl_bci {
    pub ECCsig: [__u8; 6],
    pub Status: __u8,
    pub Status1: __u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inftl_unithead1 {
    pub virtualUnitNo: __u16,
    pub prevUnitNo: __u16,
    pub ANAC: __u8,
    pub NACs: __u8,
    pub parityPerField: __u8,
    pub discarded: __u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inftl_unithead2 {
    pub parityPerField: __u8,
    pub ANAC: __u8,
    pub prevUnitNo: __u16,
    pub virtualUnitNo: __u16,
    pub NACs: __u8,
    pub discarded: __u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inftl_unittail {
    pub Reserved: [__u8; 4],
    pub EraseMark: __u16,
    pub EraseMark1: __u16,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub union inftl_uci {
    pub a: inftl_unithead1,
    pub b: inftl_unithead2,
    pub c: inftl_unittail,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inftl_oob {
    pub b: inftl_bci,
    pub u: inftl_uci,
}

// INFTL Media Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct INFTLPartition {
    pub virtualUnits: __u32,
    pub firstUnit: __u32,
    pub lastUnit: __u32,
    pub flags: __u32,
    pub spareUnits: __u32,
    pub Reserved0: __u32,
    pub Reserved1: __u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct INFTLMediaHeader {
    pub bootRecordID: [c_char; 8],
    pub NoOfBootImageBlocks: __u32,
    pub NoOfBinaryPartitions: __u32,
    pub NoOfBDTLPartitions: __u32,
    pub BlockMultiplierBits: __u32,
    pub FormatFlags: __u32,
    pub OsakVersion: __u32,
    pub PercentUsed: __u32,
    pub Partitions: [INFTLPartition; 4],
    pub __attribute__((packed)): },
// Partition flag types
pub const INFTL_BINARY: c_uint = 0x20000000;
pub const INFTL_BDTL: c_uint = 0x40000000;
pub const INFTL_LAST: c_uint = 0x80000000;
