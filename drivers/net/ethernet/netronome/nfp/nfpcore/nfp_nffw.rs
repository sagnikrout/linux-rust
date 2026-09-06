//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nfpcore/nfp_nffw.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2015-2018 Netronome Systems, Inc.
//
// nfp_nffw.h
// Authors: Jason McMullan <jason.mcmullan@netronome.com>
// Francois H. Theron <francois.theron@netronome.com>
//
// Implemented in nfp_nffw.c
extern "C" {
    pub fn nfp_nffw_info_close(state: *mut nfp_nffw_info);
}
extern "C" {
    pub fn nfp_nffw_info_mip_first(state: *mut nfp_nffw_info, cpp_id: *mut u32, off: *mut u64) -> c_int;
}
// Implemented in nfp_mip.c
extern "C" {
    pub fn nfp_mip_close(mip: *const nfp_mip);
}
extern "C" {
    pub fn nfp_mip_symtab(mip: *const nfp_mip, addr: *mut u32, size: *mut u32);
}
extern "C" {
    pub fn nfp_mip_strtab(mip: *const nfp_mip, addr: *mut u32, size: *mut u32);
}
// Implemented in nfp_rtsym.c
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_rtsym_type {
    NFP_RTSYM_TYPE_NONE	= 0,
    NFP_RTSYM_TYPE_OBJECT	= 1,
    NFP_RTSYM_TYPE_FUNCTION	= 2,
    NFP_RTSYM_TYPE_ABS	= 3,
}

pub const NFP_RTSYM_TARGET_NONE: c_int = 0;

//
// struct nfp_rtsym - RTSYM descriptor
// @name:	Symbol name
// @addr:	Address in the domain/target's address space
// @size:	Size (in bytes) of the symbol
// @type:	NFP_RTSYM_TYPE_* of the symbol
// @target:	CPP Target identifier, or NFP_RTSYM_TARGET_
// @domain:	CPP Target Domain (island)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_rtsym {
    pub name: *const c_char,
    pub addr: u64,
    pub size: u64,
    pub type: nfp_rtsym_type,
    pub target: c_int,
    pub domain: c_int,
}

extern "C" {
    pub fn nfp_rtsym_count(rtbl: *mut nfp_rtsym_table) -> c_int;
}
extern "C" {
    pub fn nfp_rtsym_size(rtsym: *const nfp_rtsym) -> u64;
}
