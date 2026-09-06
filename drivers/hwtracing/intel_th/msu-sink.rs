//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/intel_th/msu-sink.c
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


// SPDX-License-Identifier: GPL-2.0
//
// An example software sink buffer for Intel TH MSU.
//
// Copyright (C) 2019 Intel Corporation.
//

pub const MAX_SGTS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msu_sink_private {
    pub dev: *mut device,
    pub sgts: *mut sg_table,
    pub nr_sgts: c_uint,
}

    static void *msu_sink_assign(struct device *dev, int *mode)
    {
    struct msu_sink_private *priv;
    priv = kzalloc_obj(*priv);
    if (!priv)
    return core::ptr::null_mut();
    priv.sgts = kcalloc(MAX_SGTS, sizeof(void *), GFP_KERNEL);
    if (!priv.sgts) {
    kfree(priv);
    return core::ptr::null_mut();
    }
    priv.dev = dev;
// mode = MSC_MODE_MULTI;
    return priv;
    }
#[no_mangle]
unsafe extern "C" fn msu_sink_unassign(data: *mut c_void) {
    static void msu_sink_unassign(void *data)
    {
    struct msu_sink_private *priv = data;
    kfree(priv.sgts);
    kfree(priv);
    }
// See also: msc.c: __msc_buffer_win_alloc()
#[no_mangle]
unsafe extern "C" fn msu_sink_alloc_window(data: *mut c_void, sgt: *mut sg_table, size: usize) -> c_int {
    static int msu_sink_alloc_window(void *data, struct sg_table **sgt, size_t size)
    {
    struct msu_sink_private *priv = data;
    unsigned int nents;
    struct scatterlist *sg_ptr;
    void *block;
    int ret, i;
    if (priv.nr_sgts == MAX_SGTS)
    return -ENOMEM;
    nents = DIV_ROUND_UP(size, PAGE_SIZE);
    ret = sg_alloc_table(*sgt, nents, GFP_KERNEL);
    if (ret)
    return -ENOMEM;
    priv.sgts[priv.nr_sgts++] = *sgt;
    for_each_sg((*sgt).sgl, sg_ptr, nents, i) {
    block = dma_alloc_coherent(priv.dev.parent.parent,
    PAGE_SIZE, &sg_dma_address(sg_ptr),
    GFP_KERNEL);
    if (!block)
    return -ENOMEM;
    sg_set_buf(sg_ptr, block, PAGE_SIZE);
    }
    return nents;
    }
// See also: msc.c: __msc_buffer_win_free()
#[no_mangle]
unsafe extern "C" fn msu_sink_free_window(data: *mut c_void, sgt: *mut sg_table) {
    static void msu_sink_free_window(void *data, struct sg_table *sgt)
    {
    struct msu_sink_private *priv = data;
    struct scatterlist *sg_ptr;
    int i;
    for_each_sg(sgt.sgl, sg_ptr, sgt.nents, i) {
    dma_free_coherent(priv.dev.parent.parent, PAGE_SIZE,
    sg_virt(sg_ptr), sg_dma_address(sg_ptr));
    }
    sg_free_table(sgt);
    priv.nr_sgts--;
    }
#[no_mangle]
unsafe extern "C" fn msu_sink_ready(data: *mut c_void, sgt: *mut sg_table, bytes: usize) -> c_int {
    static int msu_sink_ready(void *data, struct sg_table *sgt, size_t bytes)
    {
    struct msu_sink_private *priv = data;
    intel_th_msc_window_unlock(priv.dev, sgt);
    return 0;
    }
    static const struct msu_buffer sink_mbuf = {
    .name		= "sink",
    .assign		= msu_sink_assign,
    .unassign	= msu_sink_unassign,
    .alloc_window	= msu_sink_alloc_window,
    .free_window	= msu_sink_free_window,
    .ready		= msu_sink_ready,
    };
    module_intel_th_msu_buffer(sink_mbuf);
    MODULE_DESCRIPTION("example software sink buffer for Intel TH MSU");
    MODULE_LICENSE("GPL v2");
