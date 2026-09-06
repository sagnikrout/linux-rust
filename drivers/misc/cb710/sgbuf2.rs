//! Automatically rewritten from C to Rust
//! Source: drivers/misc/cb710/sgbuf2.c
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
// cb710/sgbuf2.c
//
// Copyright by Michał Mirosław, 2008-2009
//

#[no_mangle]
unsafe extern "C" fn sg_dwiter_next(miter: *mut sg_mapping_iter) -> bool {
    static bool sg_dwiter_next(struct sg_mapping_iter *miter)
    {
    if (sg_miter_next(miter)) {
    miter.consumed = 0;
    return true;
    } else
    return false;
    }
#[no_mangle]
unsafe extern "C" fn sg_dwiter_is_at_end(miter: *mut sg_mapping_iter) -> bool {
    static bool sg_dwiter_is_at_end(struct sg_mapping_iter *miter)
    {
    return miter.length == miter.consumed && !sg_dwiter_next(miter);
    }
#[no_mangle]
unsafe extern "C" fn sg_dwiter_read_buffer(miter: *mut sg_mapping_iter) -> u32 {
    static uint32_t sg_dwiter_read_buffer(struct sg_mapping_iter *miter)
    {
    size_t len, left = 4;
    uint32_t data;
    void *addr = &data;
    do {
    len = min(miter.length - miter.consumed, left);
    memcpy(addr, miter.addr + miter.consumed, len);
    miter.consumed += len;
    left -= len;
    if (!left)
    return data;
    addr += len;
    } while (sg_dwiter_next(miter));
    memset(addr, 0, left);
    return data;
    }
#[no_mangle]
pub unsafe extern "C" fn needs_unaligned_copy(ptr: *const c_void) -> bool {
    static inline bool needs_unaligned_copy(const void *ptr)
    {

    return false;

    return ((uintptr_t)ptr & 3) != 0;

    }
#[no_mangle]
unsafe extern "C" fn sg_dwiter_get_next_block(miter: *mut sg_mapping_iter, ptr: *mut u32) -> bool {
    static bool sg_dwiter_get_next_block(struct sg_mapping_iter *miter, uint32_t **ptr)
    {
    size_t len;
    if (sg_dwiter_is_at_end(miter))
    return true;
    len = miter.length - miter.consumed;
    if (likely(len >= 4 && !needs_unaligned_copy(
    miter.addr + miter.consumed))) {
// ptr = miter->addr + miter->consumed;
    miter.consumed += 4;
    return true;
    }
    return false;
    }
//
// cb710_sg_dwiter_read_next_block() - get next 32-bit word from sg buffer
// @miter: sg mapping iterator used for reading
//
// Description:
// Returns 32-bit word starting at byte pointed to by @miter@
// handling any alignment issues.  Bytes past the buffer's end
// are not accessed (read) but are returned as zeroes.  @miter@
// is advanced by 4 bytes or to the end of buffer whichever is
// closer.
//
// Context:
// Same requirements as in sg_miter_next().
//
// Returns:
// 32-bit word just read.
//
#[no_mangle]
pub unsafe extern "C" fn cb710_sg_dwiter_read_next_block(miter: *mut sg_mapping_iter) -> u32 {
    uint32_t cb710_sg_dwiter_read_next_block(struct sg_mapping_iter *miter)
    {
    uint32_t *ptr = core::ptr::null_mut();
    if (likely(sg_dwiter_get_next_block(miter, &ptr)))
    return ptr ? *ptr : 0;
    return sg_dwiter_read_buffer(miter);
    }
    EXPORT_SYMBOL_GPL(cb710_sg_dwiter_read_next_block);
#[no_mangle]
unsafe extern "C" fn sg_dwiter_write_slow(miter: *mut sg_mapping_iter, data: u32) {
    static void sg_dwiter_write_slow(struct sg_mapping_iter *miter, uint32_t data)
    {
    size_t len, left = 4;
    void *addr = &data;
    do {
    len = min(miter.length - miter.consumed, left);
    memcpy(miter.addr, addr, len);
    miter.consumed += len;
    left -= len;
    if (!left)
    return;
    addr += len;
    } while (sg_dwiter_next(miter));
    }
//
// cb710_sg_dwiter_write_next_block() - write next 32-bit word to sg buffer
// @miter: sg mapping iterator used for writing
// @data: data to write to sg buffer
//
// Description:
// Writes 32-bit word starting at byte pointed to by @miter@
// handling any alignment issues.  Bytes which would be written
// past the buffer's end are silently discarded. @miter@ is
// advanced by 4 bytes or to the end of buffer whichever is closer.
//
// Context:
// Same requirements as in sg_miter_next().
//
#[no_mangle]
pub unsafe extern "C" fn cb710_sg_dwiter_write_next_block(miter: *mut sg_mapping_iter, data: u32) {
    void cb710_sg_dwiter_write_next_block(struct sg_mapping_iter *miter, uint32_t data)
    {
    uint32_t *ptr = core::ptr::null_mut();
    if (likely(sg_dwiter_get_next_block(miter, &ptr))) {
    if (ptr)
// ptr = data;
    else
    return;
    } else
    sg_dwiter_write_slow(miter, data);
    }
    EXPORT_SYMBOL_GPL(cb710_sg_dwiter_write_next_block);
