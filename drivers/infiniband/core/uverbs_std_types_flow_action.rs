//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/core/uverbs_std_types_flow_action.c
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
// Copyright (c) 2018, Mellanox Technologies inc.  All rights reserved.
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

    static int uverbs_free_flow_action(struct ib_uobject *uobject,
    enum rdma_remove_reason why,
    struct uverbs_attr_bundle *attrs)
    {
    struct ib_flow_action *action = uobject.object;
    if (atomic_read(&action.usecnt))
    return -EBUSY;
    return action.device.ops.destroy_flow_action(action);
    }
    DECLARE_UVERBS_NAMED_METHOD_DESTROY(
    UVERBS_METHOD_FLOW_ACTION_DESTROY,
    UVERBS_ATTR_IDR(UVERBS_ATTR_DESTROY_FLOW_ACTION_HANDLE,
    UVERBS_OBJECT_FLOW_ACTION,
    UVERBS_ACCESS_DESTROY,
    UA_MANDATORY));
    DECLARE_UVERBS_NAMED_OBJECT(
    UVERBS_OBJECT_FLOW_ACTION,
    UVERBS_TYPE_ALLOC_IDR(uverbs_free_flow_action),
    &UVERBS_METHOD(UVERBS_METHOD_FLOW_ACTION_DESTROY));
    const struct uapi_definition uverbs_def_obj_flow_action[] = {
    UAPI_DEF_CHAIN_OBJ_TREE_NAMED(
    UVERBS_OBJECT_FLOW_ACTION,
    UAPI_DEF_OBJ_NEEDS_FN(destroy_flow_action)),
    {}
    };
