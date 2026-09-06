//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/vmwgfx_validation.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright © 2018 - 2022 VMware, Inc., Palo Alto, CA., USA
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//

pub const VMW_RES_DIRTY_NONE: c_int = 0;

//
// struct vmw_validation_context - Per command submission validation context
// @ht: Hash table used to find resource- or buffer object duplicates
// @resource_list: List head for resource validation metadata
// @resource_ctx_list: List head for resource validation metadata for
// resources that need to be validated before those in @resource_list
// @bo_list: List head for buffer objects
// @page_list: List of pages used by the memory allocator
// @ticket: Ticked used for ww mutex locking
// @res_mutex: Pointer to mutex used for resource reserving
// @merge_dups: Whether to merge metadata for duplicate resources or
// buffer objects
// @mem_size_left: Free memory left in the last page in @page_list
// @page_address: Kernel virtual address of the last page in @page_list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_validation_context {
    pub sw_context: *mut vmw_sw_context,
    pub resource_list: list_head,
    pub resource_ctx_list: list_head,
    pub bo_list: list_head,
    pub page_list: list_head,
    pub ticket: ww_acquire_ctx,
    pub res_mutex: *mut mutex,
    pub merge_dups: c_uint,
    pub mem_size_left: c_uint,
    pub page_address: *mut u8,
}

//
// DECLARE_VAL_CONTEXT - Declare a validation context with initialization
// @_name: The name of the variable
// @_sw_context: Contains the hash table used to find dups or NULL if none
// @_merge_dups: Whether to merge duplicate buffer object- or resource
// entries. If set to true, ideally a hash table pointer should be supplied
// as well unless the number of resources and buffer objects per validation
// is known to be very small
//

//
// vmw_validation_has_bos - return whether the validation context has
// any buffer objects registered.
//
// @ctx: The validation context
// Returns: Whether any buffer objects are registered
//
// vmw_validation_bo_reserve - Reserve buffer objects registered with a
// validation context
// @ctx: The validation context
// @intr: Perform waits interruptible
//
// Return: Zero on success, -ERESTARTSYS when interrupted, negative error
// code on failure
//
// vmw_validation_bo_fence - Unreserve and fence buffer objects registered
// with a validation context
// @ctx: The validation context
//
// This function unreserves the buffer objects previously reserved using
// vmw_validation_bo_reserve, and fences them with a fence object.
//
// vmw_validation_align - Align a validation memory allocation
// @val: The size to be aligned
//
// Returns: @val aligned to the granularity used by the validation memory
// allocator.
//
extern "C" {
    pub fn ALIGN(_arg: val, _arg: sizeof(long)) -> return;
}
extern "C" {
    pub fn vmw_validation_bo_validate(ctx: *mut vmw_validation_context, intr: bool) -> c_int;
}
extern "C" {
    pub fn vmw_validation_unref_lists(ctx: *mut vmw_validation_context);
}
extern "C" {
    pub fn vmw_validation_drop_ht(ctx: *mut vmw_validation_context);
}
extern "C" {
    pub fn vmw_validation_res_validate(ctx: *mut vmw_validation_context, intr: bool) -> c_int;
}
extern "C" {
    pub fn vmw_validation_revert(ctx: *mut vmw_validation_context);
}
extern "C" {
    pub fn vmw_validation_preload_bo(ctx: *mut vmw_validation_context) -> c_int;
}
extern "C" {
    pub fn vmw_validation_bo_backoff(ctx: *mut vmw_validation_context);
}
