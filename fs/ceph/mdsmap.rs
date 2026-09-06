//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ceph/mdsmap.h
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
// mds map - describe servers in the mds cluster.
//
// we limit fields to those the client actually xcares about
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_info {
    pub global_id: u64,
    pub addr: ceph_entity_addr,
    pub state: i32,
    pub num_export_targets: c_int,
    pub laggy: bool,
    pub export_targets: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mdsmap {
    pub m_last_failure: u32 m_epoch, m_client_epoch,,
    pub m_root: u32,
    pub /: *mut *mut u32 m_session_timeout; / seconds,
    pub /: *mut *mut u32 m_session_autoclose; / seconds,
    pub m_max_file_size: u64,
//
// maximum size for xattrs blob.
// Zeroed by default to force the usage of the (sync) SETXATTR Op.
//
    pub m_max_xattr_size: u64,
    pub /: *mut *mut u32 m_max_mds; / expected up:active mds number,
    pub /: *mut *mut u32 m_num_active_mds; / actual up:active mds number,
    pub /: *mut *mut u32 possible_max_rank; / possible max rank index,
    pub m_info: *mut ceph_mds_info,
// which object pools file data can be stored in
    pub m_num_data_pg_pools: c_int,
    pub m_data_pg_pools: *mut u64,
    pub m_cas_pg_pool: u64,
    pub m_enabled: bool,
    pub m_damaged: bool,
    pub m_num_laggy: c_int,
    pub m_fs_name: *mut c_char,
}

extern "C" {
    pub fn ceph_mdsmap_get_random_mds(m: *mut ceph_mdsmap) -> c_int;
}
extern "C" {
    pub fn ceph_mdsmap_destroy(m: *mut ceph_mdsmap);
}
extern "C" {
    pub fn ceph_mdsmap_is_cluster_available(m: *mut ceph_mdsmap) -> bool;
}
