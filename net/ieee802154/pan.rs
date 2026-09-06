//! Automatically rewritten from C to Rust
//! Source: net/ieee802154/pan.c
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
// IEEE 802.15.4 PAN management
//
// Copyright (C) 2023 Qorvo US, Inc
// Authors:
// - David Girault <david.girault@qorvo.com>
// - Miquel Raynal <miquel.raynal@bootlin.com>
//

// Checks whether a device address matches one from the PAN list.
// This helper is meant to be used only during PAN management, when we expect
// extended addresses to be used.
//
    static bool cfg802154_pan_device_is_matching(struct ieee802154_pan_device *pan_dev,
    struct ieee802154_addr *ext_dev)
    {
    if (!pan_dev || !ext_dev)
    return false;
    if (ext_dev.mode == IEEE802154_ADDR_SHORT)
    return false;
    return pan_dev.extended_addr == ext_dev.extended_addr;
    }
#[no_mangle]
pub unsafe extern "C" fn cfg802154_device_is_associated(wpan_dev: *mut wpan_dev) -> bool {
    bool cfg802154_device_is_associated(struct wpan_dev *wpan_dev)
    {
    bool is_assoc;
    mutex_lock(&wpan_dev.association_lock);
    is_assoc = !list_empty(&wpan_dev.children) || wpan_dev.parent;
    mutex_unlock(&wpan_dev.association_lock);
    return is_assoc;
    }
    bool cfg802154_device_is_parent(struct wpan_dev *wpan_dev,
    struct ieee802154_addr *target)
    {
    lockdep_assert_held(&wpan_dev.association_lock);
    return cfg802154_pan_device_is_matching(wpan_dev.parent, target);
    }
    EXPORT_SYMBOL_GPL(cfg802154_device_is_parent);
    struct ieee802154_pan_device *
    cfg802154_device_is_child(struct wpan_dev *wpan_dev,
    struct ieee802154_addr *target)
    {
    struct ieee802154_pan_device *child;
    lockdep_assert_held(&wpan_dev.association_lock);
    list_for_each_entry(child, &wpan_dev.children, node)
    if (cfg802154_pan_device_is_matching(child, target))
    return child;
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(cfg802154_device_is_child);
#[no_mangle]
pub unsafe extern "C" fn cfg802154_get_free_short_addr(wpan_dev: *mut wpan_dev) -> __le16 {
    __le16 cfg802154_get_free_short_addr(struct wpan_dev *wpan_dev)
    {
    struct ieee802154_pan_device *child;
    __le16 addr;
    lockdep_assert_held(&wpan_dev.association_lock);
    do {
    get_random_bytes(&addr, 2);
    if (addr == cpu_to_le16(IEEE802154_ADDR_SHORT_BROADCAST) ||
    addr == cpu_to_le16(IEEE802154_ADDR_SHORT_UNSPEC))
    continue;
    if (wpan_dev.short_addr == addr)
    continue;
    if (wpan_dev.parent && wpan_dev.parent.short_addr == addr)
    continue;
    list_for_each_entry(child, &wpan_dev.children, node)
    if (child.short_addr == addr)
    continue;
    break;
    } while (1);
    return addr;
    }
    EXPORT_SYMBOL_GPL(cfg802154_get_free_short_addr);
    unsigned int cfg802154_set_max_associations(struct wpan_dev *wpan_dev,
    unsigned int max)
    {
    unsigned int old_max;
    lockdep_assert_held(&wpan_dev.association_lock);
    old_max = wpan_dev.max_associations;
    wpan_dev.max_associations = max;
    return old_max;
    }
    EXPORT_SYMBOL_GPL(cfg802154_set_max_associations);
