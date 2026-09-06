//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnx2x/bnx2x_fw_file_hdr.h
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


// bnx2x_fw_file_hdr.h: FW binary file header structure.
//
// Copyright (c) 2007-2013 Broadcom Corporation
// Copyright (c) 2014 QLogic Corporation
// All rights reserved
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// Maintained by: Ariel Elior <ariel.elior@qlogic.com>
// Written by: Vladislav Zolotarov
// Based on the original idea of John Wright <john.wright@hp.com>.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_fw_file_section {
    pub len: __be32,
    pub offset: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_fw_file_hdr {
    pub init_ops: bnx2x_fw_file_section,
    pub init_ops_offsets: bnx2x_fw_file_section,
    pub init_data: bnx2x_fw_file_section,
    pub tsem_int_table_data: bnx2x_fw_file_section,
    pub tsem_pram_data: bnx2x_fw_file_section,
    pub usem_int_table_data: bnx2x_fw_file_section,
    pub usem_pram_data: bnx2x_fw_file_section,
    pub csem_int_table_data: bnx2x_fw_file_section,
    pub csem_pram_data: bnx2x_fw_file_section,
    pub xsem_int_table_data: bnx2x_fw_file_section,
    pub xsem_pram_data: bnx2x_fw_file_section,
    pub iro_arr: bnx2x_fw_file_section,
    pub fw_version: bnx2x_fw_file_section,
}
