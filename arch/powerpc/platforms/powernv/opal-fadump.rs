//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/powernv/opal-fadump.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Firmware-Assisted Dump support on POWER platform (OPAL).
//
// Copyright 2019, Hari Bathini, IBM Corporation.
//

//
// With kernel & initrd loaded at 512MB (with 256MB size), enforce a minimum
// boot memory size of 768MB to ensure f/w loading kernel and initrd doesn't
// mess with crash'ed kernel's memory during MPIPL.
//

//
// OPAL FADump metadata structure format version
//
// OPAL FADump kernel metadata structure stores kernel metadata needed to
// register-for/process crash dump. Format version is used to keep a tab on
// the changes in the structure format. The changes, if any, to the format
// are expected to be minimal and backward compatible.
//
pub const OPAL_FADUMP_VERSION: c_uint = 0x1;
//
// OPAL FADump kernel metadata
//
// The address of this structure will be registered with f/w for retrieving
// in the capture kernel to process the crash dump.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_fadump_mem_struct {
    pub version: u8,
    pub reserved: [u8; 3],
    pub /: *mut *mut __be16 region_cnt; / number of regions,
    pub /: *mut *mut __be16 registered_regions; / Regions registered for MPIPL,
    pub fadumphdr_addr: __be64,
    pub rgn: [opal_mpipl_region; FADUMP_MAX_MEM_REGS],
    pub __packed: },
//
// CPU state data
//
// CPU state data information is provided by f/w. The format for this data
// is defined in the HDAT spec. Version is used to keep a tab on the changes
// in this CPU state data format. Changes to this format are unlikely, but
// if there are any changes, please refer to latest HDAT specification.
//
pub const HDAT_FADUMP_CPU_DATA_VER: c_int = 1;

// HDAT thread header for register entries
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdat_fadump_thread_hdr {
    pub pir: __be32,
// 0x00 - 0x0F - The corresponding stop state of the core
    pub core_state: u8,
    pub reserved: [u8; 3],
    pub /: *mut *mut __be32 offset; / Offset to Register Entries array,
    pub /: *mut *mut __be32 ecnt; / Number of entries,
    pub /: *mut *mut __be32 esize; / Alloc size of each array entry in bytes,
    pub /: *mut *mut __be32 eactsz; / Actual size of each array entry in bytes,
    pub __packed: },
// Register types populated by f/w
pub const HDAT_FADUMP_REG_TYPE_GPR: c_uint = 0x01;
pub const HDAT_FADUMP_REG_TYPE_SPR: c_uint = 0x02;
// ID numbers used by f/w while populating certain registers
pub const HDAT_FADUMP_REG_ID_NIP: c_uint = 0x7D0;
pub const HDAT_FADUMP_REG_ID_MSR: c_uint = 0x7D1;
pub const HDAT_FADUMP_REG_ID_CCR: c_uint = 0x7D2;
// HDAT register entry.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdat_fadump_reg_entry {
    pub reg_type: __be32,
    pub reg_num: __be32,
    pub reg_val: __be64,
    pub __packed: },
    pub reg_val: regs->gpr[reg_num] =,
    pub reg_val: regs->ctr =,
    pub reg_val: regs->link =,
    pub reg_val: regs->xer =,
    pub reg_val: regs->dar =,
    pub reg_val: regs->dsisr =,
    pub reg_val: regs->nip =,
    pub reg_val: regs->msr =,
    pub reg_val: regs->ccr =,
    pub reg_entry: *mut hdat_fadump_reg_entry,
    pub val: u64,
    pub i: c_int,
    pub pt_regs)): memset(regs, 0, sizeof(struct,
    pub {: for (i = 0; i < regs_cnt; i++, bufp += reg_entry_size),
    pub )bufp: *mut reg_entry = (struct hdat_fadump_reg_entry,
    pub )(reg_entry->reg_val)): (u64,
