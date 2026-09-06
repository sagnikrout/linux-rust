//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/dev_energymodel.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
// Do not edit directly, auto-generated from:
// Documentation/netlink/specs/dev-energymodel.yaml
// YNL-GEN uapi header
// To regenerate run: tools/net/ynl/ynl-regen.sh

pub const DEV_ENERGYMODEL_FAMILY_VERSION: c_int = 1;
//
// enum dev_energymodel_perf_state_flags
// @DEV_ENERGYMODEL_PERF_STATE_FLAGS_PERF_STATE_INEFFICIENT: The performance
// state is inefficient. There is in this perf-domain, another performance
// state with a higher frequency but a lower or equal power cost.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_energymodel_perf_state_flags {
    DEV_ENERGYMODEL_PERF_STATE_FLAGS_PERF_STATE_INEFFICIENT = 1,
}

//
// enum dev_energymodel_perf_domain_flags
// @DEV_ENERGYMODEL_PERF_DOMAIN_FLAGS_PERF_DOMAIN_MICROWATTS: The power values
// are in micro-Watts or some other scale.
// @DEV_ENERGYMODEL_PERF_DOMAIN_FLAGS_PERF_DOMAIN_SKIP_INEFFICIENCIES: Skip
// inefficient states when estimating energy consumption.
// @DEV_ENERGYMODEL_PERF_DOMAIN_FLAGS_PERF_DOMAIN_ARTIFICIAL: The power values
// are artificial and might be created by platform missing real power
// information.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_energymodel_perf_domain_flags {
    DEV_ENERGYMODEL_PERF_DOMAIN_FLAGS_PERF_DOMAIN_MICROWATTS = 1,
    DEV_ENERGYMODEL_PERF_DOMAIN_FLAGS_PERF_DOMAIN_SKIP_INEFFICIENCIES = 2,
    DEV_ENERGYMODEL_PERF_DOMAIN_FLAGS_PERF_DOMAIN_ARTIFICIAL = 4,
}

