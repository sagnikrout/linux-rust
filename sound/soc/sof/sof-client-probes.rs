//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/sof-client-probes.h
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
// Callbacks used on platforms where the control for audio is split between
// DSP and host, like HDA.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_probes_host_ops {
    pub stream_id): *mut *mut snd_soc_dai dai, u32,
    pub dai): *mut snd_soc_dai,
    pub dai): *mut snd_soc_dai,
    pub dai): *mut int cmd, struct snd_soc_dai,
    pub dai): *mut snd_soc_dai,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_probe_point_desc {
    pub buffer_id: c_uint,
    pub purpose: c_uint,
    pub stream_tag: c_uint,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_probe_info_type {
    PROBES_INFO_ACTIVE_PROBES,
    PROBES_INFO_AVAILABE_PROBES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_probes_ipc_ops {
    pub buffer_size): usize,
    pub cdev): *mut *mut int (deinit)(struct sof_client_dev,
    pub type): *mut *mut size_t num_desc, enum sof_probe_info_type,
    pub desc): *mut sof_probe_point_desc,
    pub num_desc): usize,
    pub num_buffer_id): *mut *mut unsigned int buffer_id, size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_probes_priv {
    pub dfs_points: *mut dentry,
    pub dfs_points_remove: *mut dentry,
    pub extractor_stream_tag: u32,
    pub card: snd_soc_card,
    pub ipc_priv: *mut c_void,
    pub host_ops: *const sof_probes_host_ops,
    pub ipc_ops: *const sof_probes_ipc_ops,
}
