//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/elx/libefc/efc_node.h
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
// Copyright (C) 2021 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
//

//
// hold frames in pending frame list
//
// Unsolicited receive frames are held on the node pending frame list,
// rather than being processed.
//
// accept frames
//
// Unsolicited receive frames processed rather than being held on the node
// pending frame list.
//
// Node initiator/target enable defines
// All combinations of the SLI port (nport) initiator/target enable,
// and remote node initiator/target enable are enumerated.
// ex: EFC_NODE_ENABLE_T_TO_IT decodes to target mode is enabled on SLI port
// and I+T is enabled on remote node.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efc_node_enable {
    EFC_NODE_ENABLE_x_TO_x,
    EFC_NODE_ENABLE_x_TO_T,
    EFC_NODE_ENABLE_x_TO_I,
    EFC_NODE_ENABLE_x_TO_IT,
    EFC_NODE_ENABLE_T_TO_x,
    EFC_NODE_ENABLE_T_TO_T,
    EFC_NODE_ENABLE_T_TO_I,
    EFC_NODE_ENABLE_T_TO_IT,
    EFC_NODE_ENABLE_I_TO_x,
    EFC_NODE_ENABLE_I_TO_T,
    EFC_NODE_ENABLE_I_TO_I,
    EFC_NODE_ENABLE_I_TO_IT,
    EFC_NODE_ENABLE_IT_TO_x,
    EFC_NODE_ENABLE_IT_TO_T,
    EFC_NODE_ENABLE_IT_TO_I,
    EFC_NODE_ENABLE_IT_TO_IT,
}

extern "C" {
    pub fn efc_node_get_wwnn(node: *mut efc_node) -> u64;
}
