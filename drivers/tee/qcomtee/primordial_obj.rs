//! Automatically rewritten from C to Rust
//! Source: drivers/tee/qcomtee/primordial_obj.c
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

//
// DOC: Primordial Object
//
// After boot, the kernel provides a static object of type
// %QCOMTEE_OBJECT_TYPE_CB called the primordial object. This object is used
// for native kernel services or privileged operations.
//
// We support:
// - %QCOMTEE_OBJECT_OP_MAP_REGION to map a memory object and return mapping
// object and mapping information (see qcomtee_mem_object_map()).
// - %QCOMTEE_OBJECT_OP_YIELD to yield by the thread running in QTEE.
// - %QCOMTEE_OBJECT_OP_SLEEP to wait for a period of time.
//
pub const QCOMTEE_OBJECT_OP_MAP_REGION: c_int = 0;
pub const QCOMTEE_OBJECT_OP_YIELD: c_int = 1;
pub const QCOMTEE_OBJECT_OP_SLEEP: c_int = 2;
// Mapping information format as expected by QTEE.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcomtee_mapping_info {
    pub paddr: u64,
    pub len: u64,
    pub perms: u32,
    pub __packed: },
    static int
    qcomtee_primordial_obj_dispatch(struct qcomtee_object_invoke_ctx *oic,
    struct qcomtee_object *primordial_object_unused,
    u32 op, struct qcomtee_arg *args)
    {
    pub map_info: *mut qcomtee_mapping_info,
    pub mem_object: *mut qcomtee_object,
    pub map_object: *mut qcomtee_object,
    pub 0: int err =,
    switch (op) {
    case QCOMTEE_OBJECT_OP_YIELD:
// No output object.
    pub NULL: oic->data =,
    case QCOMTEE_OBJECT_OP_SLEEP:
// Check message format matched QCOMTEE_OBJECT_OP_SLEEP op.
    if (qcomtee_args_len(args) != 1 ||
    args[0].type != QCOMTEE_ARG_TYPE_IB ||
    args[0].b.size < sizeof(u32))
    pub -EINVAL: return,
    pub )(args[0].b.addr)): *mut *mut msleep((u32,
// No output object.
    pub NULL: oic->data =,
    case QCOMTEE_OBJECT_OP_MAP_REGION:
    if (qcomtee_args_len(args) != 3 ||
    args[0].type != QCOMTEE_ARG_TYPE_OB ||
    args[1].type != QCOMTEE_ARG_TYPE_IO ||
    args[2].type != QCOMTEE_ARG_TYPE_OO ||
    args[0].b.size < sizeof(struct qcomtee_mapping_info))
    pub -EINVAL: return,
    pub args[0].b.addr: map_info =,
    pub args[1].o: mem_object =,
    qcomtee_mem_object_map(mem_object, &map_object,
    &map_info.paddr, &map_info.len,
    pub map_object: args[2].o =,
// One output object; pass it for cleanup to notify.
    pub map_object: oic->data =,
    default:
    pub -EINVAL: err =,
    }
    pub err: return,
    }
// Called after submitting the callback response.
    static void qcomtee_primordial_obj_notify(struct qcomtee_object_invoke_ctx *oic,
    struct qcomtee_object *unused,
    int err)
    {
    pub oic->data: *mut *mut qcomtee_object object =,
// If err, QTEE did not obtain mapping object. Drop it.
    if (object && err)
    }
    static struct qcomtee_object_operations qcomtee_primordial_obj_ops = {
    .dispatch = qcomtee_primordial_obj_dispatch,
    .notify = qcomtee_primordial_obj_notify,
}

    struct qcomtee_object qcomtee_primordial_object = {
    .name = "primordial",
    .object_type = QCOMTEE_OBJECT_TYPE_CB,
    .ops = &qcomtee_primordial_obj_ops
    };
