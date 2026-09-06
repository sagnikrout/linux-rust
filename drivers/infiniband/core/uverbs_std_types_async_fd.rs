//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/core/uverbs_std_types_async_fd.c
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2019, Mellanox Technologies inc.  All rights reserved.
//

    static int UVERBS_HANDLER(UVERBS_METHOD_ASYNC_EVENT_ALLOC)(
    struct uverbs_attr_bundle *attrs)
    {
    struct ib_uobject *uobj =
    uverbs_attr_get_uobject(attrs, UVERBS_METHOD_ASYNC_EVENT_ALLOC);
    ib_uverbs_init_async_event_file(
    container_of(uobj, struct ib_uverbs_async_event_file, uobj));
    return 0;
    }
    static void uverbs_async_event_destroy_uobj(struct ib_uobject *uobj,
    enum rdma_remove_reason why)
    {
    struct ib_uverbs_async_event_file *event_file =
    container_of(uobj, struct ib_uverbs_async_event_file, uobj);
    ib_unregister_event_handler(&event_file.event_handler);
    if (why == RDMA_REMOVE_DRIVER_REMOVE)
    ib_uverbs_async_handler(event_file, 0, IB_EVENT_DEVICE_FATAL,
    core::ptr::null_mut(), core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn uverbs_async_event_free_event_queue(uobj: *mut ib_uobject) {
    static void uverbs_async_event_free_event_queue(struct ib_uobject *uobj)
    {
    struct ib_uverbs_async_event_file *event_file;
    event_file =
    container_of(uobj, struct ib_uverbs_async_event_file, uobj);
//
// The async event FD has to deliver IB_EVENT_DEVICE_FATAL even after
// disassociation, so cleaning the event list must only happen after
// release. The user knows it has reached the end of the event stream
// when it sees IB_EVENT_DEVICE_FATAL.
//
    ib_uverbs_free_event_queue(&event_file.ev_queue);
    }
    DECLARE_UVERBS_NAMED_METHOD(
    UVERBS_METHOD_ASYNC_EVENT_ALLOC,
    UVERBS_ATTR_FD(UVERBS_ATTR_ASYNC_EVENT_ALLOC_FD_HANDLE,
    UVERBS_OBJECT_ASYNC_EVENT,
    UVERBS_ACCESS_NEW,
    UA_MANDATORY));
    DECLARE_UVERBS_NAMED_OBJECT(
    UVERBS_OBJECT_ASYNC_EVENT,
    UVERBS_TYPE_ALLOC_FD_RELEASE(sizeof(struct ib_uverbs_async_event_file),
    uverbs_async_event_destroy_uobj,
    uverbs_async_event_free_event_queue,
    &uverbs_async_event_fops,
    "[infinibandevent]",
    O_RDONLY),
    &UVERBS_METHOD(UVERBS_METHOD_ASYNC_EVENT_ALLOC));
    const struct uapi_definition uverbs_def_obj_async_fd[] = {
    UAPI_DEF_CHAIN_OBJ_TREE_NAMED(UVERBS_OBJECT_ASYNC_EVENT),
    {}
    };
