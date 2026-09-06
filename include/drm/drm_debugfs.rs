//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_debugfs.h
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


//
// Internal Header for the Direct Rendering Manager
//
// Copyright 1999 Precision Insight, Inc., Cedar Park, Texas.
// Copyright 2000 VA Linux Systems, Inc., Sunnyvale, California.
// Copyright (c) 2009-2010, Code Aurora Forum.
// All rights reserved.
//
// Author: Rickard E. (Rik) Faith <faith@valinux.com>
// Author: Gareth Hughes <gareth@valinux.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// VA LINUX SYSTEMS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

//
// DRM_DEBUGFS_GPUVA_INFO - &drm_info_list entry to dump a GPU VA space
// @show: the &drm_info_list's show callback
// @data: driver private data
//
// Drivers should use this macro to define a &drm_info_list entry to provide a
// debugfs file for dumping the GPU VA space regions and mappings.
//
// For each DRM GPU VA space drivers should call drm_debugfs_gpuva_info() from
// their @show callback.
//

//
// struct drm_info_list - debugfs info list entry
//
// This structure represents a debugfs file to be created by the drm
// core.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_info_list {
// @name: file name
    pub name: *const c_char,
//
// @show:
//
// Show callback. &seq_file->private will be set to the &struct
// drm_info_node corresponding to the instance of this info on a given
// &struct drm_minor.
//
    pub void*): *mut *mut *mut int (show)(struct seq_file,,
// @driver_features: Required driver features for this entry
    pub driver_features: u32,
// @data: Driver-private data, should not be device-specific.
    pub data: *mut c_void,
}

//
// struct drm_info_node - Per-minor debugfs node structure
//
// This structure represents a debugfs file, as an instantiation of a &struct
// drm_info_list on a &struct drm_minor.
//
// FIXME:
//
// No it doesn't make a hole lot of sense that we duplicate debugfs entries for
// both the render and the primary nodes, but that's how this has organically
// grown. It should probably be fixed, with a compatibility link, if needed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_info_node {
// @minor: &struct drm_minor for this node.
    pub minor: *mut drm_minor,
// @info_ent: template for this node.
    pub info_ent: *const drm_info_list,
// private:
    pub list: list_head,
    pub dent: *mut dentry,
}

//
// struct drm_debugfs_info - debugfs info list entry
//
// This structure represents a debugfs file to be created by the drm
// core.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_debugfs_info {
// @name: File name
    pub name: *const c_char,
//
// @show:
//
// Show callback. &seq_file->private will be set to the &struct
// drm_debugfs_entry corresponding to the instance of this info
// on a given &struct drm_device.
//
    pub void*): *mut *mut *mut int (show)(struct seq_file,,
// @driver_features: Required driver features for this entry.
    pub driver_features: u32,
// @data: Driver-private data, should not be device-specific.
    pub data: *mut c_void,
}

//
// struct drm_debugfs_entry - Per-device debugfs node structure
//
// This structure represents a debugfs file, as an instantiation of a &struct
// drm_debugfs_info on a &struct drm_device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_debugfs_entry {
// @dev: &struct drm_device for this node.
    pub dev: *mut drm_device,
// @file: Template for this node.
    pub file: drm_debugfs_info,
// @list: Linked list of all device nodes.
    pub list: list_head,
}

extern "C" {
    pub fn drm_debugfs_clients_add(file: *mut drm_file);
}
extern "C" {
    pub fn drm_debugfs_clients_remove(file: *mut drm_file);
}

