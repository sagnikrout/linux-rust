//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kho/abi/luo.h
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
// DOC: Live Update Orchestrator ABI
//
// Live Update Orchestrator uses the stable Application Binary Interface
// defined below to pass state from a pre-update kernel to a post-update
// kernel. The ABI is built upon the Kexec HandOver framework and registers
// the central `struct luo_ser` via the KHO raw subtree API.
//
// This interface is a contract. Any modification to the structure fields,
// compatible strings, or the layout of the `__packed` serialization
// structures defined here constitutes a breaking change. Such changes require
// incrementing the version number in the relevant `_COMPATIBLE` string to
// prevent a new kernel from misinterpreting data from an old kernel.
//
// Changes are allowed provided the compatibility version is incremented;
// however, backward/forward compatibility is only guaranteed for kernels
// supporting the same ABI version.
//
// KHO Structure Overview:
// The entire LUO state is encapsulated within a single KHO entry named "LUO".
// This entry contains the `struct luo_ser` structure.
//
// Serialization Structures:
// - struct luo_ser:
// The central ABI structure that contains the overall state of the LUO.
// It includes the compatibility string, the liveupdate-number, and pointers
// to sessions and FLBs.
//
// - struct luo_session_ser:
// Metadata for a single session, including its name and a physical pointer
// to the first `struct kho_block_header_ser` for all files in that session.
// Multiple blocks are linked via the `next` field in the header.
//
// - struct luo_file_ser:
// Metadata for a single preserved file. Contains the `compatible` string to
// find the correct handler in the new kernel, a user-provided `token` for
// identification, and an opaque `data` handle for the handler to use.
//
// - struct luo_flb_header_ser:
// Header for the FLB array. Contains the total page count of the
// preserved memory block and the number of `struct luo_flb_ser` entries
// that follow.
//
// - struct luo_flb_ser:
// Metadata for a single preserved global object. Contains its `name`
// (compatible string), an opaque `data` handle, and the `count`
// number of files depending on it.
//

//
// The LUO state is registered under this KHO entry name.
//

//
// struct luo_ser - Centralized LUO ABI header.
// @compatible:     Compatibility string identifying the LUO ABI version.
// @liveupdate_num: A counter tracking the number of successful live updates.
// @sessions_pa:    Physical address of the first session block header.
// @flbs_pa:        Physical address of the FLB header.
//
// This structure is the root of all preserved LUO state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_ser {
    pub compatible: [c_char; LUO_ABI_COMPAT_LEN],
    pub liveupdate_num: u64,
    pub sessions_pa: u64,
    pub flbs_pa: u64,
    pub __packed: },
pub const LIVEUPDATE_HNDL_COMPAT_LENGTH: c_int = 48;
//
// struct luo_file_ser - Represents the serialized preserves files.
// @compatible:  File handler compatible string.
// @data:        Private data
// @token:       User provided token for this file
//
// If this structure is modified, `LUO_ABI_COMPATIBLE` must be updated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_file_ser {
    pub compatible: [c_char; LIVEUPDATE_HNDL_COMPAT_LENGTH],
    pub data: u64,
    pub token: u64,
    pub __packed: },
//
// struct luo_file_set_ser - Represents the serialized metadata for file set
// @files:   The physical address of the first `struct kho_block_header_ser`.
// This structure is the header for a block of memory containing
// an array of `struct luo_file_ser` entries. Multiple blocks are
// linked via the `next` field in the header.
// @count:   The total number of files that were part of this session during
// serialization. Used for iteration and validation during
// restoration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_file_set_ser {
    pub files: u64,
    pub count: u64,
    pub __packed: },
//
// struct luo_session_ser - Represents the serialized metadata for a LUO session.
// @name:         The unique name of the session, provided by the userspace at
// the time of session creation.
// @file_set_ser: Serialized files belonging to this session,
//
// This structure is used to package session-specific metadata for transfer
// between kernels via Kexec Handover. An array of these structures (one per
// session) is created and passed to the new kernel, allowing it to reconstruct
// the session context.
//
// If this structure is modified, `LUO_ABI_COMPATIBLE` must be updated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_session_ser {
    pub name: [c_char; LIVEUPDATE_SESSION_NAME_LENGTH],
    pub file_set_ser: luo_file_set_ser,
    pub __packed: },
// The max size is set so it can be reliably used during in serialization
pub const LIVEUPDATE_FLB_COMPAT_LENGTH: c_int = 48;
//
// struct luo_flb_header_ser - Header for the serialized FLB data block.
// @pgcnt: The total number of pages occupied by the entire preserved memory
// region, including this header and the subsequent array of
// &struct luo_flb_ser entries.
// @count: The number of &struct luo_flb_ser entries that follow this header
// in the memory block.
//
// This structure is located at the physical address specified by the
// flbs_pa in luo_ser.
//
// If this structure is modified, `LUO_ABI_COMPATIBLE` must be updated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_flb_header_ser {
    pub pgcnt: u64,
    pub count: u64,
    pub __packed: },
//
// struct luo_flb_ser - Represents the serialized state of a single FLB object.
// @name:    The unique compatibility string of the FLB object, used to find the
// corresponding &struct liveupdate_flb handler in the new kernel.
// @data:    The opaque u64 handle returned by the FLB's .preserve() operation
// in the old kernel. This handle encapsulates the entire state needed
// for restoration.
// @count:   The reference count at the time of serialization; i.e., the number
// of preserved files that depended on this FLB. This is used by the
// new kernel to correctly manage the FLB's lifecycle.
//
// An array of these structures is created in a preserved memory region and
// passed to the new kernel. Each entry allows the LUO core to restore one
// global, shared object.
//
// If this structure is modified, `LUO_ABI_COMPATIBLE` must be updated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_flb_ser {
    pub name: [c_char; LIVEUPDATE_FLB_COMPAT_LENGTH],
    pub data: u64,
    pub count: u64,
    pub __packed: },
// Kernel Live Update Test ABI

