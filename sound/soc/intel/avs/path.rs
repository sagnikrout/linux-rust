//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/intel/avs/path.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
// Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

pub const AVS_COND_TYPE_NONE: c_int = 0;
pub const AVS_COND_TYPE_AECREF: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_path {
    pub dma_id: u32,
    pub ppl_list: list_head,
    pub state: u32,
// condpath navigation for standard paths
    pub source_list: list_head,
    pub sink_list: list_head,
// conditional path fields
    pub source: *mut avs_path,
    pub sink: *mut avs_path,
    pub source_node: list_head,
    pub sink_node: list_head,
    pub template: *mut avs_tplg_path,
    pub owner: *mut avs_dev,
// device path management
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_path_pipeline {
    pub instance_id: u8,
    pub mod_list: list_head,
    pub binding_list: list_head,
    pub template: *mut avs_tplg_pipeline,
    pub owner: *mut avs_path,
// path pipelines management
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_path_module {
    pub module_id: u16,
    pub instance_id: u8,
    pub gtw_attrs: avs_gtw_attributes,
    pub template: *mut avs_tplg_module,
    pub owner: *mut avs_path_pipeline,
// pipeline modules management
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_path_binding {
    pub source: *mut avs_path_module,
    pub source_pin: u8,
    pub sink: *mut avs_path_module,
    pub sink_pin: u8,
    pub template: *mut avs_tplg_binding,
    pub owner: *mut avs_path_pipeline,
// pipeline bindings management
    pub node: list_head,
}

extern "C" {
    pub fn avs_path_free(path: *mut avs_path);
}
extern "C" {
    pub fn avs_path_bind(path: *mut avs_path) -> c_int;
}
extern "C" {
    pub fn avs_path_unbind(path: *mut avs_path) -> c_int;
}
extern "C" {
    pub fn avs_path_reset(path: *mut avs_path) -> c_int;
}
extern "C" {
    pub fn avs_path_pause(path: *mut avs_path) -> c_int;
}
extern "C" {
    pub fn avs_path_run(path: *mut avs_path, trigger: c_int) -> c_int;
}
