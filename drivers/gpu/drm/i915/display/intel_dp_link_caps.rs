//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_dp_link_caps.h
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


// SPDX-License-Identifier: MIT
// Copyright © 2026 Intel Corporation

//
// enum intel_dp_link_caps_order_key - key used to order configurations
// @INTEL_DP_LINK_CAPS_ORDER_KEY_BW:
// Order configurations by bandwidth, then by link rate.
// @INTEL_DP_LINK_CAPS_ORDER_KEY_RATE_LANE:
// Order configurations by link rate, then by lane count.
// @INTEL_DP_LINK_CAPS_ORDER_KEY_LANE_RATE:
// Order configurations by lane count, then by link rate.
// @INTEL_DP_LINK_CAPS_ORDER_KEY_NUM:
// Number of ordering keys.
//
// Selects how a caller wants the configuration table to be ordered,
// together with an &enum intel_dp_link_caps_order_direction, for
// iteration queries.
//
// See also:
// - &struct intel_dp_link_caps_order
// - intel_dp_link_caps_get_max_config()
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_dp_link_caps_order_key {
    INTEL_DP_LINK_CAPS_ORDER_KEY_BW,
    INTEL_DP_LINK_CAPS_ORDER_KEY_RATE_LANE,
    INTEL_DP_LINK_CAPS_ORDER_KEY_LANE_RATE,

    INTEL_DP_LINK_CAPS_ORDER_KEY_NUM
}

//
// enum intel_dp_link_caps_order_direction - iteration direction
// @INTEL_DP_LINK_CAPS_ORDER_DIR_ASC:
// Iterate in ascending order according to the selected ordering key.
// @INTEL_DP_LINK_CAPS_ORDER_DIR_DESC:
// Iterate in descending order according to the selected ordering key.
// @INTEL_DP_LINK_CAPS_ORDER_DIR_NUM:
// Number of ordering directions.
//
// Selects the direction associated with an
// &enum intel_dp_link_caps_order_key for iteration queries.
//
// See also:
// - &struct intel_dp_link_caps_order
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_dp_link_caps_order_direction {
    INTEL_DP_LINK_CAPS_ORDER_DIR_ASC,
    INTEL_DP_LINK_CAPS_ORDER_DIR_DESC,

    INTEL_DP_LINK_CAPS_ORDER_DIR_NUM
}

//
// struct intel_dp_link_caps_order - configuration ordering
// @key:
// Key used to order configurations.
// @dir:
// Direction of the selected ordering.
//
// Describes an iteration order for link configurations.
//
// See also:
// - for_each_dp_link_config()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dp_link_caps_order {
    pub key: intel_dp_link_caps_order_key,
    pub dir: intel_dp_link_caps_order_direction,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dp_link_caps_filter {
    pub config_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dp_link_caps_iter {
    pub link_caps: *mut intel_dp_link_caps,
    pub pos: c_int,
    pub order: intel_dp_link_caps_order,
    pub filter: intel_dp_link_caps_filter,
    pub config): *mut intel_dp_link_config,
}

//
// for_each_dp_link_config - iterate allowed link configurations
// @__iter:
// &struct intel_dp_link_caps_iter being iterated
// @__config:
// pointer to &struct intel_dp_link_config filled for each match
//

extern "C" {
    pub fn intel_dp_link_caps_iter_end(iter: *mut intel_dp_link_caps_iter);
}
extern "C" {
    pub fn intel_dp_link_caps_print_common_rates(link_caps: *mut intel_dp_link_caps);
}
extern "C" {
    pub fn intel_dp_link_caps_reset_max_limits(link_caps: *mut intel_dp_link_caps);
}
extern "C" {
    pub fn intel_dp_link_caps_reset(link_caps: *mut intel_dp_link_caps);
}
extern "C" {
    pub fn intel_dp_link_caps_debugfs_add(connector: *mut intel_connector);
}
extern "C" {
    pub fn intel_dp_link_caps_cleanup(link_caps: *mut intel_dp_link_caps);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dp_link_caps_test_ops {
}

