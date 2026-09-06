//! Automatically rewritten from C to Rust
//! Source: net/ieee802154/nl_policy.c
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
// nl802154.h
//
// Copyright (C) 2007, 2008 Siemens AG
//

    const struct nla_policy ieee802154_policy[IEEE802154_ATTR_MAX + 1] = {
    [IEEE802154_ATTR_DEV_NAME] = { .type = NLA_STRING, },
    [IEEE802154_ATTR_DEV_INDEX] = { .type = NLA_U32, },
    [IEEE802154_ATTR_PHY_NAME] = { .type = NLA_STRING, },
    [IEEE802154_ATTR_STATUS] = { .type = NLA_U8, },
    [IEEE802154_ATTR_SHORT_ADDR] = { .type = NLA_U16, },
    [IEEE802154_ATTR_HW_ADDR] = { .type = NLA_HW_ADDR, },
    [IEEE802154_ATTR_PAN_ID] = { .type = NLA_U16, },
    [IEEE802154_ATTR_CHANNEL] = { .type = NLA_U8, },
    [IEEE802154_ATTR_BCN_ORD] = { .type = NLA_U8, },
    [IEEE802154_ATTR_SF_ORD] = { .type = NLA_U8, },
    [IEEE802154_ATTR_PAN_COORD] = { .type = NLA_U8, },
    [IEEE802154_ATTR_BAT_EXT] = { .type = NLA_U8, },
    [IEEE802154_ATTR_COORD_REALIGN] = { .type = NLA_U8, },
    [IEEE802154_ATTR_PAGE] = { .type = NLA_U8, },
    [IEEE802154_ATTR_DEV_TYPE] = { .type = NLA_U8, },
    [IEEE802154_ATTR_COORD_SHORT_ADDR] = { .type = NLA_U16, },
    [IEEE802154_ATTR_COORD_HW_ADDR] = { .type = NLA_HW_ADDR, },
    [IEEE802154_ATTR_COORD_PAN_ID] = { .type = NLA_U16, },
    [IEEE802154_ATTR_SRC_SHORT_ADDR] = { .type = NLA_U16, },
    [IEEE802154_ATTR_SRC_HW_ADDR] = { .type = NLA_HW_ADDR, },
    [IEEE802154_ATTR_SRC_PAN_ID] = { .type = NLA_U16, },
    [IEEE802154_ATTR_DEST_SHORT_ADDR] = { .type = NLA_U16, },
    [IEEE802154_ATTR_DEST_HW_ADDR] = { .type = NLA_HW_ADDR, },
    [IEEE802154_ATTR_DEST_PAN_ID] = { .type = NLA_U16, },
    [IEEE802154_ATTR_CAPABILITY] = { .type = NLA_U8, },
    [IEEE802154_ATTR_REASON] = { .type = NLA_U8, },
    [IEEE802154_ATTR_SCAN_TYPE] = { .type = NLA_U8, },
    [IEEE802154_ATTR_CHANNELS] = { .type = NLA_U32, },
    [IEEE802154_ATTR_DURATION] = { .type = NLA_U8, },
    [IEEE802154_ATTR_ED_LIST] = { .len = 27 },
    [IEEE802154_ATTR_CHANNEL_PAGE_LIST] = { .len = 32 * 4, },
    [IEEE802154_ATTR_TXPOWER] = { .type = NLA_S8, },
    [IEEE802154_ATTR_LBT_ENABLED] = { .type = NLA_U8, },
    [IEEE802154_ATTR_CCA_MODE] = { .type = NLA_U8, },
    [IEEE802154_ATTR_CCA_ED_LEVEL] = { .type = NLA_S32, },
    [IEEE802154_ATTR_CSMA_RETRIES] = { .type = NLA_U8, },
    [IEEE802154_ATTR_CSMA_MIN_BE] = { .type = NLA_U8, },
    [IEEE802154_ATTR_CSMA_MAX_BE] = { .type = NLA_U8, },
    [IEEE802154_ATTR_FRAME_RETRIES] = { .type = NLA_S8, },
    [IEEE802154_ATTR_LLSEC_ENABLED] = { .type = NLA_U8, },
    [IEEE802154_ATTR_LLSEC_SECLEVEL] = { .type = NLA_U8, },
    [IEEE802154_ATTR_LLSEC_KEY_MODE] = { .type = NLA_U8, },
    [IEEE802154_ATTR_LLSEC_KEY_SOURCE_SHORT] = { .type = NLA_U32, },
    [IEEE802154_ATTR_LLSEC_KEY_SOURCE_EXTENDED] = { .type = NLA_HW_ADDR, },
    [IEEE802154_ATTR_LLSEC_KEY_ID] = { .type = NLA_U8, },
    [IEEE802154_ATTR_LLSEC_FRAME_COUNTER] = { .type = NLA_U32 },
    [IEEE802154_ATTR_LLSEC_KEY_BYTES] = { .len = 16, },
    [IEEE802154_ATTR_LLSEC_KEY_USAGE_FRAME_TYPES] = { .type = NLA_U8, },
    [IEEE802154_ATTR_LLSEC_KEY_USAGE_COMMANDS] = { .len = 258 / 8 },
    [IEEE802154_ATTR_LLSEC_FRAME_TYPE] = { .type = NLA_U8, },
    [IEEE802154_ATTR_LLSEC_CMD_FRAME_ID] = { .type = NLA_U8, },
    [IEEE802154_ATTR_LLSEC_SECLEVELS] = { .type = NLA_U8, },
    [IEEE802154_ATTR_LLSEC_DEV_OVERRIDE] = { .type = NLA_U8, },
    [IEEE802154_ATTR_LLSEC_DEV_KEY_MODE] = { .type = NLA_U8, },
    };
