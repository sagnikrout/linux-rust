//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/cxgb4/id_table.c
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


//
// Copyright (c) 2011 Chelsio Communications.  All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

pub const RANDOM_SKIP: c_int = 16;
//
// Trivial bitmap-based allocator. If the random flag is set, the
// allocator is designed to:
// - pseudo-randomize the id returned such that it is not trivially predictable.
// - avoid reuse of recently used id (at the expense of predictability)
//
#[no_mangle]
pub unsafe extern "C" fn c4iw_id_alloc(alloc: *mut c4iw_id_table) -> u32 {
    u32 c4iw_id_alloc(struct c4iw_id_table *alloc)
    {
    unsigned long flags;
    u32 obj;
    spin_lock_irqsave(&alloc.lock, flags);
    obj = find_next_zero_bit(alloc.table, alloc.max, alloc.last);
    if (obj >= alloc.max)
    obj = find_first_zero_bit(alloc.table, alloc.max);
    if (obj < alloc.max) {
    if (alloc.flags & C4IW_ID_TABLE_F_RANDOM)
    alloc.last += get_random_u32_below(RANDOM_SKIP);
    else
    alloc.last = obj + 1;
    if (alloc.last >= alloc.max)
    alloc.last = 0;
    __set_bit(obj, alloc.table);
    obj += alloc.start;
    } else
    obj = -1;
    spin_unlock_irqrestore(&alloc.lock, flags);
    return obj;
    }
#[no_mangle]
pub unsafe extern "C" fn c4iw_id_free(alloc: *mut c4iw_id_table, obj: u32) {
    void c4iw_id_free(struct c4iw_id_table *alloc, u32 obj)
    {
    unsigned long flags;
    obj -= alloc.start;
    spin_lock_irqsave(&alloc.lock, flags);
    __clear_bit(obj, alloc.table);
    spin_unlock_irqrestore(&alloc.lock, flags);
    }
    int c4iw_id_table_alloc(struct c4iw_id_table *alloc, u32 start, u32 num,
    u32 reserved, u32 flags)
    {
    alloc.start = start;
    alloc.flags = flags;
    if (flags & C4IW_ID_TABLE_F_RANDOM)
    alloc.last = get_random_u32_below(RANDOM_SKIP);
    else
    alloc.last = 0;
    alloc.max = num;
    spin_lock_init(&alloc.lock);
    alloc.table = bitmap_zalloc(num, GFP_KERNEL);
    if (!alloc.table)
    return -ENOMEM;
    if (!(alloc.flags & C4IW_ID_TABLE_F_EMPTY))
    bitmap_set(alloc.table, 0, reserved);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn c4iw_id_table_free(alloc: *mut c4iw_id_table) {
    void c4iw_id_table_free(struct c4iw_id_table *alloc)
    {
    bitmap_free(alloc.table);
    }
