//! Automatically rewritten from C to Rust
//! Source: net/wireless/ocb.c
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
// OCB mode implementation
//
// Copyright: (c) 2014 Czech Technical University in Prague
// (c) 2014 Volkswagen Group Research
// Copyright (C) 2022-2023 Intel Corporation
// Author:    Rostislav Lisovy <rostislav.lisovy@fel.cvut.cz>
// Funded by: Volkswagen Group Research
//

    int cfg80211_join_ocb(struct cfg80211_registered_device *rdev,
    struct net_device *dev,
    struct ocb_setup *setup)
    {
    struct wireless_dev *wdev = dev.ieee80211_ptr;
    int err;
    lockdep_assert_wiphy(wdev.wiphy);
    if (dev.ieee80211_ptr.iftype != NL80211_IFTYPE_OCB)
    return -EOPNOTSUPP;
    if (!rdev.ops.join_ocb)
    return -EOPNOTSUPP;
    if (WARN_ON(!setup.chandef.chan))
    return -EINVAL;
    err = rdev_join_ocb(rdev, dev, setup);
    if (!err)
    wdev.u.ocb.chandef = setup.chandef;
    return err;
    }
    int cfg80211_leave_ocb(struct cfg80211_registered_device *rdev,
    struct net_device *dev)
    {
    struct wireless_dev *wdev = dev.ieee80211_ptr;
    int err;
    lockdep_assert_wiphy(wdev.wiphy);
    if (dev.ieee80211_ptr.iftype != NL80211_IFTYPE_OCB)
    return -EOPNOTSUPP;
    if (!rdev.ops.leave_ocb)
    return -EOPNOTSUPP;
    if (!wdev.u.ocb.chandef.chan)
    return -ENOTCONN;
    err = rdev_leave_ocb(rdev, dev);
    if (!err)
    memset(&wdev.u.ocb.chandef, 0, sizeof(wdev.u.ocb.chandef));
    return err;
    }
