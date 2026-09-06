//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/core/uverbs_flow.c
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

    struct ib_uflow_resources *flow_resources_alloc(size_t num_specs)
    {
    struct ib_uflow_resources *resources;
    resources = kzalloc_obj(*resources);
    if (!resources)
    return core::ptr::null_mut();
    if (!num_specs)
    goto out;
    resources.counters =
    kzalloc_objs(*resources.counters, num_specs);
    resources.collection =
    kzalloc_objs(*resources.collection, num_specs);
    if (!resources.counters || !resources.collection)
    goto err;
    out:
    resources.max = num_specs;
    return resources;
    err:
    kfree(resources.counters);
    kfree(resources);
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(flow_resources_alloc);
#[no_mangle]
pub unsafe extern "C" fn ib_uverbs_flow_resources_free(uflow_res: *mut ib_uflow_resources) {
    void ib_uverbs_flow_resources_free(struct ib_uflow_resources *uflow_res)
    {
    unsigned int i;
    if (!uflow_res)
    return;
    for (i = 0; i < uflow_res.collection_num; i++)
    atomic_dec(&uflow_res.collection[i].usecnt);
    for (i = 0; i < uflow_res.counters_num; i++)
    atomic_dec(&uflow_res.counters[i].usecnt);
    kfree(uflow_res.collection);
    kfree(uflow_res.counters);
    kfree(uflow_res);
    }
    EXPORT_SYMBOL(ib_uverbs_flow_resources_free);
    void flow_resources_add(struct ib_uflow_resources *uflow_res,
    enum ib_flow_spec_type type,
    void *ibobj)
    {
    WARN_ON(uflow_res.num >= uflow_res.max);
    switch (type) {
    case IB_FLOW_SPEC_ACTION_HANDLE:
    atomic_inc(&((struct ib_flow_action *)ibobj).usecnt);
    uflow_res.collection[uflow_res.collection_num++] =
    (struct ib_flow_action *)ibobj;
    break;
    case IB_FLOW_SPEC_ACTION_COUNT:
    atomic_inc(&((struct ib_counters *)ibobj).usecnt);
    uflow_res.counters[uflow_res.counters_num++] =
    (struct ib_counters *)ibobj;
    break;
    default:
    WARN_ON(1);
    }
    uflow_res.num++;
    }
    EXPORT_SYMBOL(flow_resources_add);
