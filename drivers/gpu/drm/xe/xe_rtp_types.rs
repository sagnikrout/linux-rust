//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_rtp_types.h
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
//
// Copyright © 2022 Intel Corporation
//

//
// struct xe_rtp_action - action to take for any matching rule
//
// This struct records what action should be taken in a register that has a
// matching rule. Example of actions: set/clear bits.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_rtp_action {
// @reg: Register
    pub reg: xe_reg,
//
// @clr_bits: bits to clear when updating register. It's always a
// superset of bits being modified
//
    pub clr_bits: u32,
// @set_bits: bits to set when updating register
    pub set_bits: u32,
// @set_func: function to provide bits to set when updating register
    pub hwe): *mut xe_hw_engine,
}

// @read_mask: mask for bits to consider when reading value back

// @flags: flags to apply on rule evaluation or action
//
// @use_func:
// Internal flag indicating @set_func should be called instead of
// using @set_bits.
//
// struct xe_rtp_rule - match rule for processing entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_rtp_rule {
    pub match_type: u8,
// match filters
// MATCH_PLATFORM / MATCH_SUBPLATFORM
    pub platform: u8,
    pub subplatform: u8,
}

//
// MATCH_GRAPHICS_VERSION / XE_RTP_MATCH_GRAPHICS_VERSION_RANGE
// MATCH_MEDIA_VERSION  / XE_RTP_MATCH_MEDIA_VERSION_RANGE
//

// MATCH_STEP
// MATCH_ENGINE_CLASS / MATCH_NOT_ENGINE_CLASS
// MATCH_FUNC
// struct xe_rtp_entry_sr - Entry in an rtp table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_rtp_entry_sr {
    pub name: *const c_char,
    pub actions: *const xe_rtp_action,
    pub rules: *const xe_rtp_rule,
    pub n_rules: u8,
    pub n_actions: u8,

    pub flags: u8,
}

// struct xe_rtp_entry - Entry in an rtp table, with no action associated
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_rtp_entry {
    pub name: *const c_char,
    pub rules: *const xe_rtp_rule,
    pub n_rules: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_rtp_table_sr {
    pub entries: *const xe_rtp_entry_sr,
    pub n_entries: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_rtp_table {
    pub entries: *const xe_rtp_entry,
    pub n_entries: usize,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_rtp_process_type {
    XE_RTP_PROCESS_TYPE_DEVICE,
    XE_RTP_PROCESS_TYPE_GT,
    XE_RTP_PROCESS_TYPE_ENGINE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_rtp_process_ctx {
    pub xe: *mut xe_device,
    pub gt: *mut xe_gt,
    pub hwe: *mut xe_hw_engine,
}
