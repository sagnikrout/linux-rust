//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/dev/length.c
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
// Copyright (C) 2012, 2020 Oliver Hartkopp <socketcan@hartkopp.net>
//

// CAN DLC to real data length conversion helpers
    static const u8 dlc2len[] = {
    0, 1, 2, 3, 4, 5, 6, 7,
    8, 12, 16, 20, 24, 32, 48, 64
    };
// get data length from raw data length code (DLC)
#[no_mangle]
pub unsafe extern "C" fn can_fd_dlc2len(dlc: u8) -> u8 {
    u8 can_fd_dlc2len(u8 dlc)
    {
    return dlc2len[dlc & 0x0F];
    }
    EXPORT_SYMBOL_GPL(can_fd_dlc2len);
    static const u8 len2dlc[] = {
    0, 1, 2, 3, 4, 5, 6, 7, 8,	/* 0 - 8 */
    9, 9, 9, 9,			/* 9 - 12 */
    10, 10, 10, 10,			/* 13 - 16 */
    11, 11, 11, 11,			/* 17 - 20 */
    12, 12, 12, 12,			/* 21 - 24 */
    13, 13, 13, 13, 13, 13, 13, 13,	/* 25 - 32 */
    14, 14, 14, 14, 14, 14, 14, 14,	/* 33 - 40 */
    14, 14, 14, 14, 14, 14, 14, 14,	/* 41 - 48 */
    15, 15, 15, 15, 15, 15, 15, 15,	/* 49 - 56 */
    15, 15, 15, 15, 15, 15, 15, 15	/* 57 - 64 */
    };
// map the sanitized data length to an appropriate data length code
#[no_mangle]
pub unsafe extern "C" fn can_fd_len2dlc(len: u8) -> u8 {
    u8 can_fd_len2dlc(u8 len)
    {
// check for length mapping table size at build time
    BUILD_BUG_ON(ARRAY_SIZE(len2dlc) != CANFD_MAX_DLEN + 1);
    if (unlikely(len > CANFD_MAX_DLEN))
    return CANFD_MAX_DLC;
    return len2dlc[len];
    }
    EXPORT_SYMBOL_GPL(can_fd_len2dlc);
//
// can_skb_get_frame_len() - Calculate the CAN Frame length in bytes
// of a given skb.
// @skb: socket buffer of a CAN message.
//
// Do a rough calculation: bit stuffing is ignored and length in bits
// is rounded up to a length in bytes.
//
// Rationale: this function is to be used for the BQL functions
// (netdev_sent_queue() and netdev_completed_queue()) which expect a
// value in bytes. Just using skb->len is insufficient because it will
// return the constant value of CAN(FD)_MTU. Doing the bit stuffing
// calculation would be too expensive in term of computing resources
// for no noticeable gain.
//
// Remarks: The payload of CAN FD frames with BRS flag are sent at a
// different bitrate. Currently, the can-utils canbusload tool does
// not support CAN-FD yet and so we could not run any benchmark to
// measure the impact. There might be possible improvement here.
//
// Return: length in bytes.
//
#[no_mangle]
pub unsafe extern "C" fn can_skb_get_frame_len(skb: *const sk_buff) -> c_uint {
    unsigned int can_skb_get_frame_len(const struct sk_buff *skb)
    {
    const struct canfd_frame *cf = (const struct canfd_frame *)skb.data;
    u8 len;
    if (can_is_canfd_skb(skb))
    len = canfd_sanitize_len(cf.len);
#[no_mangle]
pub unsafe extern "C" fn if(CAN_RTR_FLAG: cf->can_id &) -> else {
    else if (cf.can_id & CAN_RTR_FLAG)
    len = 0;
    else
    len = cf.len;
    return can_frame_bytes(can_is_canfd_skb(skb), cf.can_id & CAN_EFF_FLAG,
    false, len);
    }
    EXPORT_SYMBOL_GPL(can_skb_get_frame_len);
