//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/xe/tests/xe_rtp.c
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


// SPDX-License-Identifier: GPL-2.0 AND MIT
//
// Copyright © 2026 Intel Corporation
//

//
// xe_rtp_rule_matches - Check if a set of RTP rule set match against the
// device/GT/hwe
// @xe: The xe device
// @gt: The GT struct (may be NULL)
// @hwe: The hw_engine  (may be NULL)
// @rules: The array of rules to match against
// @n_rules: Number of items in @rules
// @err: Pointer (may be NULL) to set error number.
//
// This parses the set of rules and check if they match against the passed
// parameters.
//
// If passed, @err is updated with a non-zero negative error number or zero if
// no errors were found during the parsing/evaluation of rules.
//
// Returns true if there is a match and false if there is no match or if an
// error was found.
//
    bool xe_rtp_rule_matches(const struct xe_device *xe,
    struct xe_gt *gt,
    struct xe_hw_engine *hwe,
    const struct xe_rtp_rule *rules,
    unsigned int n_rules,
    int *err)
    {
    return rule_matches_with_err(xe, gt, hwe, rules, n_rules, err);
    }
    EXPORT_SYMBOL_IF_KUNIT(xe_rtp_rule_matches);
