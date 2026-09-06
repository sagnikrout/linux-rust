//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nilfs2/sysfs.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Sysfs support declarations.
//
// Copyright (C) 2005-2014 Nippon Telegraph and Telephone Corporation.
// Copyright (C) 2014 HGST, Inc., a Western Digital Company.
//
// Written by Vyacheslav Dubeyko <Vyacheslav.Dubeyko@hgst.com>
//

//
// struct nilfs_sysfs_dev_subgroups - device subgroup kernel objects
// @sg_superblock_kobj: /sys/fs/<nilfs>/<device>/superblock
// @sg_superblock_kobj_unregister: completion state
// @sg_segctor_kobj: /sys/fs/<nilfs>/<device>/segctor
// @sg_segctor_kobj_unregister: completion state
// @sg_mounted_snapshots_kobj: /sys/fs/<nilfs>/<device>/mounted_snapshots
// @sg_mounted_snapshots_kobj_unregister: completion state
// @sg_checkpoints_kobj: /sys/fs/<nilfs>/<device>/checkpoints
// @sg_checkpoints_kobj_unregister: completion state
// @sg_segments_kobj: /sys/fs/<nilfs>/<device>/segments
// @sg_segments_kobj_unregister: completion state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_sysfs_dev_subgroups {
// /sys/fs/<nilfs>/<device>/superblock
    pub sg_superblock_kobj: kobject,
    pub sg_superblock_kobj_unregister: completion,
// /sys/fs/<nilfs>/<device>/segctor
    pub sg_segctor_kobj: kobject,
    pub sg_segctor_kobj_unregister: completion,
// /sys/fs/<nilfs>/<device>/mounted_snapshots
    pub sg_mounted_snapshots_kobj: kobject,
    pub sg_mounted_snapshots_kobj_unregister: completion,
// /sys/fs/<nilfs>/<device>/checkpoints
    pub sg_checkpoints_kobj: kobject,
    pub sg_checkpoints_kobj_unregister: completion,
// /sys/fs/<nilfs>/<device>/segments
    pub sg_segments_kobj: kobject,
    pub sg_segments_kobj_unregister: completion,
}

