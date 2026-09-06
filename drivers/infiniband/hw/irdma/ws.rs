//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/ws.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2015 - 2020 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_ws_node_type {
    WS_NODE_TYPE_PARENT,
    WS_NODE_TYPE_LEAF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_ws_match_type {
    WS_MATCH_TYPE_VSI,
    WS_MATCH_TYPE_TC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_ws_node {
    pub siblings: list_head,
    pub child_list_head: list_head,
    pub parent: *mut irdma_ws_node,
    pub /: *mut *mut u64 lan_qs_handle; / opaque handle used by LAN,
    pub l2_sched_node_id: u32,
    pub index: u16,
    pub qs_handle: u16,
    pub vsi_index: u16,
    pub traffic_class: u8,
    pub user_pri: u8,
    pub rel_bw: u8,
    pub /: *mut *mut u8 abstraction_layer; / used for splitting a TC,
    pub prio_type: u8,
    pub type_leaf:1: bool,
    pub enable:1: bool,
}

extern "C" {
    pub fn irdma_ws_add(vsi: *mut irdma_sc_vsi, user_pri: u8) -> c_int;
}
extern "C" {
    pub fn irdma_ws_remove(vsi: *mut irdma_sc_vsi, user_pri: u8);
}
extern "C" {
    pub fn irdma_ws_reset(vsi: *mut irdma_sc_vsi);
}
