//! Automatically rewritten from C Header to Rust Module
//! Source: security/landlock/access.h
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
// Landlock - Access types and helpers
//
// Copyright © 2016-2020 Mickaël Salaün <mic@digikod.net>
// Copyright © 2018-2020 ANSSI
// Copyright © 2024-2025 Microsoft Corporation
//

//
// All access rights that are denied by default whether they are handled or not
// by a ruleset/layer.  This must be ORed with all domain->handled_masks[]
// entries when we need to get the absolute handled access masks, see
// landlock_upgrade_handled_access_masks().
//
// clang-format off

// clang-format on
// clang-format off

// clang-format on
pub type access_mask_t = u32;
// Makes sure all filesystem access rights can be stored.
// Makes sure all network access rights can be stored.
// Makes sure all scoped rights can be stored.
// Makes sure for_each_set_bit() and for_each_clear_bit() calls are OK.
// Ruleset access masks.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct access_masks {
    pub LANDLOCK_NUM_ACCESS_FS: access_mask_t fs :,
    pub LANDLOCK_NUM_ACCESS_NET: access_mask_t net :,
    pub LANDLOCK_NUM_SCOPE: access_mask_t scope :,
    pub __aligned(sizeof(u32)): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub union access_masks_all {
    pub masks: access_masks,
    pub all: u32,
}

// Makes sure all fields are covered.
//
// struct layer_mask - The access rights and rule flags for a layer.
//
// This has a bit for each access rights and rule flags.  During access checks,
// it is used to represent the access rights for each layer which still need to
// be fulfilled.  When all bits are 0, the access request is considered to be
// fulfilled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct layer_mask {
//
// @access: The unfulfilled access rights for this layer.
//
    pub LANDLOCK_NUM_ACCESS_MAX: access_mask_t access :,

//
// @quiet: Whether we have encountered a rule with the quiet flag for
// this layer.  Used to control logging.
//
    pub 1: access_mask_t quiet :,

    pub __aligned(sizeof(access_mask_t)): } __packed,
//
// Make sure that we don't increase the size of struct layer_mask when storing
// rule flags.
//
    pub sizeof(access_mask_t)): static_assert(sizeof(struct layer_mask) ==,
//
// struct layer_masks - An array of struct layer_mask, one per layer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct layer_masks {
//
// @layers: The unfulfilled access rights for each layer.
//
    pub layers: [layer_mask; LANDLOCK_MAX_NUM_LAYERS],
}

//
// Tracks domains responsible of a denied access.  This avoids storing in each
// object the full matrix of per-layer unfulfilled access rights, which is
// required by update_request().
//
// Each nibble represents the layer index of the newest layer which denied a
// certain access right.  For file system access rights, the upper four bits are
// the index of the layer which denies LANDLOCK_ACCESS_FS_IOCTL_DEV and the
// lower nibble represents LANDLOCK_ACCESS_FS_TRUNCATE.
//
pub type deny_masks_t = u8;
//
// Makes sure all optional access rights can be tied to a layer index (cf.
// get_deny_mask).
//
// LANDLOCK_MAX_NUM_LAYERS must be a power of two (cf. deny_masks_t assert).
// Upgrades with all initially denied by default access rights.
//
// All access rights that are denied by default whether they are
// explicitly handled or not.
//
// Checks the subset relation between access masks.
// A bitmask that is large enough to hold set of optional accesses.
pub type optional_access_t = u8;
