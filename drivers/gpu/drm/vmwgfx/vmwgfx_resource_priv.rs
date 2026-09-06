//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/vmwgfx_resource_priv.h
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
// Copyright 2012-2014 VMware, Inc., Palo Alto, CA., USA
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

//
// Extra memory required by the resource id's ida storage, which is allocated
// separately from the base object itself. We estimate an on-average 128 bytes
// per ida.
//
pub const VMW_IDA_ACC_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmw_cmdbuf_res_state {
    VMW_CMDBUF_RES_COMMITTED,
    VMW_CMDBUF_RES_ADD,
    VMW_CMDBUF_RES_DEL
}

//
// struct vmw_user_resource_conv - Identify a derived user-exported resource
// type and provide a function to convert its ttm_base_object pointer to
// a struct vmw_resource
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_user_resource_conv {
    pub object_type: ttm_object_type,
    pub base): *mut *mut *mut vmw_resource (base_obj_to_res)(ttm_base_object,
    pub res): *mut *mut void (res_free) (struct vmw_resource,
}

//
// struct vmw_res_func - members and functions common for a resource type
//
// @res_type:          Enum that identifies the lru list to use for eviction.
// @needs_guest_memory:Whether the resource is guest-backed and needs
// persistent buffer storage.
// @type_name:         String that identifies the resource type.
// @domain:            TTM placement for guest memory buffers.
// @busy_domain:       TTM busy placement for guest memory buffers.
// @may_evict          Whether the resource may be evicted.
// @create:            Create a hardware resource.
// @destroy:           Destroy a hardware resource.
// @bind:              Bind a hardware resource to persistent buffer storage.
// @unbind:            Unbind a hardware resource from persistent
// buffer storage.
// @commit_notify:     If the resource is a command buffer managed resource,
// callback to notify that a define or remove command
// has been committed to the device.
// @dirty_alloc:       Allocate a dirty tracker. NULL if dirty-tracking is not
// supported.
// @dirty_free:        Free the dirty tracker.
// @dirty_sync:        Upload the dirty mob contents to the resource.
// @dirty_add_range:   Add a sequential dirty range to the resource
// dirty tracker.
// @clean:             Clean the resource.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_res_func {
    pub res_type: vmw_res_type,
    pub needs_guest_memory: bool,
    pub type_name: *const c_char,
    pub domain: u32,
    pub busy_domain: u32,
    pub may_evict: bool,
    pub prio: u32,
    pub dirty_prio: u32,
    pub res): *mut *mut int (create) (struct vmw_resource,
    pub res): *mut *mut int (destroy) (struct vmw_resource,
    pub val_buf): *mut ttm_validate_buffer,
    pub val_buf): *mut ttm_validate_buffer,
    pub state): vmw_cmdbuf_res_state,
    pub res): *mut *mut int (dirty_alloc)(struct vmw_resource,
    pub res): *mut *mut void (dirty_free)(struct vmw_resource,
    pub res): *mut *mut int (dirty_sync)(struct vmw_resource,
    pub end): usize,
    pub res): *mut *mut int (clean)(struct vmw_resource,
}

//
// struct vmw_simple_resource_func - members and functions common for the
// simple resource helpers.
// @res_func:  struct vmw_res_func as described above.
// @ttm_res_type:  TTM resource type used for handle recognition.
// @size:  Size of the simple resource information struct.
// @init:  Initialize the simple resource information.
// @hw_destroy:  A resource hw_destroy function.
// @set_arg_handle:  Set the handle output argument of the ioctl create struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_simple_resource_func {
    pub res_func: vmw_res_func,
    pub ttm_res_type: c_int,
    pub size: usize,
    pub data): *mut *mut *mut int (init)(struct vmw_resource res, void,
    pub res): *mut *mut void (hw_destroy)(struct vmw_resource,
    pub handle): *mut *mut *mut void (set_arg_handle)(void data, u32,
}

//
// struct vmw_simple_resource - Kernel only side simple resource
// @res: The resource we derive from.
// @func: The method and member virtual table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_simple_resource {
    pub res: vmw_resource,
    pub func: *const vmw_simple_resource_func,
}

extern "C" {
    pub fn vmw_resource_alloc_id(res: *mut vmw_resource) -> c_int;
}
extern "C" {
    pub fn vmw_resource_release_id(res: *mut vmw_resource);
}
