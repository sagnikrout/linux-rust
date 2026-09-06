//! Automatically rewritten from C to Rust
//! Source: drivers/pnp/isapnp/compat.c
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
// compat.c - A series of functions to make it easier to convert drivers that use
// the old isapnp APIs. If possible use the new APIs instead.
//
// Copyright 2002 Adam Belay <ambx1@neo.rr.com>
//

    static void pnp_convert_id(char *buf, unsigned short vendor,
    unsigned short device)
    {
    sprintf(buf, "%c%c%c%x%x%x%x",
    'A' + ((vendor >> 2) & 0x3f) - 1,
    'A' + (((vendor & 3) << 3) | ((vendor >> 13) & 7)) - 1,
    'A' + ((vendor >> 8) & 0x1f) - 1,
    (device >> 4) & 0x0f, device & 0x0f,
    (device >> 12) & 0x0f, (device >> 8) & 0x0f);
    }
    struct pnp_dev *pnp_find_dev(struct pnp_card *card, unsigned short vendor,
    unsigned short function, struct pnp_dev *from)
    {
    char id[8];
    char any[8];
    pnp_convert_id(id, vendor, function);
    pnp_convert_id(any, ISAPNP_ANY_ID, ISAPNP_ANY_ID);
    if (card == core::ptr::null_mut()) {	/* look for a logical device from all cards */
    struct list_head *list;
    list = pnp_global.next;
    if (from)
    list = from.global_list.next;
    while (list != &pnp_global) {
    struct pnp_dev *dev = global_to_pnp_dev(list);
    if (compare_pnp_id(dev.id, id) ||
    (memcmp(id, any, 7) == 0))
    return dev;
    list = list.next;
    }
    } else {
    struct list_head *list;
    list = card.devices.next;
    if (from) {
    list = from.card_list.next;
    if (from.card != card)	/* something is wrong */
    return core::ptr::null_mut();
    }
    while (list != &card.devices) {
    struct pnp_dev *dev = card_to_pnp_dev(list);
    if (compare_pnp_id(dev.id, id))
    return dev;
    list = list.next;
    }
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(pnp_find_dev);
