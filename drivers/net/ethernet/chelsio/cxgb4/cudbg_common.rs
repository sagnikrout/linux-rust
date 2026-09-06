//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/chelsio/cxgb4/cudbg_common.c
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
// Copyright (C) 2017 Chelsio Communications.  All rights reserved.
//

    int cudbg_get_buff(struct cudbg_init *pdbg_init,
    struct cudbg_buffer *pdbg_buff, u32 size,
    struct cudbg_buffer *pin_buff)
    {
    u32 offset;
    offset = pdbg_buff.offset;
    if (offset + size > pdbg_buff.size)
    return CUDBG_STATUS_NO_MEM;
    if (pdbg_init.compress_type != CUDBG_COMPRESSION_NONE) {
    if (size > pdbg_init.compress_buff_size)
    return CUDBG_STATUS_NO_MEM;
    pin_buff.data = (char *)pdbg_init.compress_buff;
    pin_buff.offset = 0;
    pin_buff.size = size;
    return 0;
    }
    pin_buff.data = (char *)pdbg_buff.data + offset;
    pin_buff.offset = offset;
    pin_buff.size = size;
    return 0;
    }
    void cudbg_put_buff(struct cudbg_init *pdbg_init,
    struct cudbg_buffer *pin_buff)
    {
// Clear compression buffer for re-use
    if (pdbg_init.compress_type != CUDBG_COMPRESSION_NONE)
    memset(pdbg_init.compress_buff, 0,
    pdbg_init.compress_buff_size);
    pin_buff.data = core::ptr::null_mut();
    pin_buff.offset = 0;
    pin_buff.size = 0;
    }
    void cudbg_update_buff(struct cudbg_buffer *pin_buff,
    struct cudbg_buffer *pout_buff)
    {
// We already write to buffer provided by ethool, so just
// increment offset to next free space.
//
    pout_buff.offset += pin_buff.size;
    }
