//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/ocfs1_fs_compat.h
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
// ocfs1_fs_compat.h
//
// OCFS1 volume header definitions.  OCFS2 creates valid but unmountable
// OCFS1 volume headers on the first two sectors of an OCFS2 volume.
// This allows an OCFS1 volume to see the partition and cleanly fail to
// mount it.
//
// Copyright (C) 2002, 2004 Oracle.  All rights reserved.
//
pub const OCFS1_MAX_VOL_SIGNATURE_LEN: c_int = 128;
pub const OCFS1_MAX_MOUNT_POINT_LEN: c_int = 128;
pub const OCFS1_MAX_VOL_ID_LENGTH: c_int = 16;
pub const OCFS1_MAX_VOL_LABEL_LEN: c_int = 64;
pub const OCFS1_MAX_CLUSTER_NAME_LEN: c_int = 64;

//
// OCFS1 superblock.  Lives at sector 0.
//
// 00*/	__u32 minor_version;
// 08*/	__u8 signature[OCFS1_MAX_VOL_SIGNATURE_LEN];
// 88*/	__u8 mount_point[OCFS1_MAX_MOUNT_POINT_LEN];
// 108*/	__u64 serial_num;
// 110*/	__u64 device_size;
// 120*/	__u64 bitmap_off;
// 130*/	__u64 vote_off;
// 140*/	__u64 data_start_off;
// 150*/	__u64 root_off;
// 160*/	__u64 cluster_size;
// 170*/	__u64 num_clusters;
// 180*/	__u64 file_node_size;
// 190*/	__u64 node_cfg_off;
// 1A0*/	__u64 new_cfg_off;
// 1B0
// 00*/	__u32 curr_master;
// 10*/	__u64 last_read_time;
// 20*/	__u64 oin_node_map;
// 30
//
// OCFS1 volume label.  Lives at sector 1.
//
// 00*/	struct ocfs1_disk_lock disk_lock;
// 30*/	__u8 label[OCFS1_MAX_VOL_LABEL_LEN];
// 70*/	__u16 label_len;
// 72*/	__u8 vol_id[OCFS1_MAX_VOL_ID_LENGTH];
// 82*/	__u16 vol_id_len;
// 84*/	__u8 cluster_name[OCFS1_MAX_CLUSTER_NAME_LEN];
// A4*/	__u16 cluster_name_len;
// A6
