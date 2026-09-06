//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sof/ext_manifest.h
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
// Extended manifest is a place to store metadata about firmware, known during
// compilation time - for example firmware version or used compiler.
// Given information are read on host side before firmware startup.
// This part of output binary is not signed.
//

// In ASCII `XMan`
pub const SOF_EXT_MAN_MAGIC_NUMBER: c_uint = 0x6e614d58;
// Build u32 number in format MMmmmppp

// check extended manifest version consistency

// used extended manifest header version

// extended manifest header, deleting any field breaks backward compatibility
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ext_man_header {
    pub /: *mut *mut uint32_t magic; /< identification number,,
// < EXT_MAN_MAGIC_NUMBER
    pub /: *mut *mut uint32_t full_size; /< [bytes] full size of ext_man,,
// < (header + content + padding)
    pub /: *mut *mut uint32_t header_size; /< [bytes] makes header extensionable,,
// < after append new field to ext_man header
// < then backward compatible won't be lost
    pub /: *mut *mut uint32_t header_version; /< value of EXT_MAN_VERSION,
// < not related with following content
// just after this header should be list of ext_man_elem_* elements
    pub __packed: },
// Now define extended manifest elements
// Extended manifest elements types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ext_man_elem_type {
    SOF_EXT_MAN_ELEM_FW_VERSION		= 0,
    SOF_EXT_MAN_ELEM_WINDOW			= 1,
    SOF_EXT_MAN_ELEM_CC_VERSION		= 2,
    SOF_EXT_MAN_ELEM_PROBE_INFO		= 3,
    SOF_EXT_MAN_ELEM_DBG_ABI		= 4,
    SOF_EXT_MAN_ELEM_CONFIG_DATA		= 5, /**< ABI3.17 */
    SOF_EXT_MAN_ELEM_PLATFORM_CONFIG_DATA   = 6,
}

// extended manifest element header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ext_man_elem_header {
    pub /: *mut *mut uint32_t type; /< SOF_EXT_MAN_ELEM_,
    pub /: *mut *mut uint32_t size; /< in bytes, including header size,
// just after this header should be type dependent content
    pub __packed: },
// FW version
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ext_man_fw_version {
    pub hdr: sof_ext_man_elem_header,
// use sof_ipc struct because of code re-use
    pub version: sof_ipc_fw_version,
    pub flags: u32,
    pub __packed: },
// extended data memory windows for IPC, trace and debug
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ext_man_window {
    pub hdr: sof_ext_man_elem_header,
// use sof_ipc struct because of code re-use
    pub ipc_window: sof_ipc_window,
    pub __packed: },
// Used C compiler description
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ext_man_cc_version {
    pub hdr: sof_ext_man_elem_header,
// use sof_ipc struct because of code re-use
    pub cc_version: sof_ipc_cc_version,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_man_dbg_abi {
    pub hdr: sof_ext_man_elem_header,
// use sof_ipc struct because of code re-use
    pub dbg_abi: sof_ipc_user_abi_version,
    pub __packed: },
// EXT_MAN_ELEM_CONFIG_DATA elements identificators, ABI3.17
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum config_elem_type {
    SOF_EXT_MAN_CONFIG_EMPTY		= 0,
    SOF_EXT_MAN_CONFIG_IPC_MSG_SIZE		= 1,
    SOF_EXT_MAN_CONFIG_MEMORY_USAGE_SCAN	= 2, /**< ABI 3.18 */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_config_elem {
    pub token: u32,
    pub value: u32,
    pub __packed: },
// firmware configuration information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ext_man_config_data {
    pub hdr: sof_ext_man_elem_header,
    pub elems: [sof_config_elem; ],
    pub __packed: },
