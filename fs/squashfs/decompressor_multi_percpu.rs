//! Automatically rewritten from C to Rust
//! Source: fs/squashfs/decompressor_multi_percpu.c
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
// Copyright (c) 2013
// Phillip Lougher <phillip@squashfs.org.uk>
//

//
// This file implements multi-threaded decompression using percpu
// variables, one thread per cpu core.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_stream {
    pub stream: *mut c_void,
    pub lock: local_lock_t,
}

    static void *squashfs_decompressor_create(struct squashfs_sb_info *msblk,
    void *comp_opts)
    {
    struct squashfs_stream *stream;
    struct squashfs_stream __percpu *percpu;
    int err, cpu;
    percpu = alloc_percpu(struct squashfs_stream);
    if (percpu == core::ptr::null_mut())
    return ERR_PTR(-ENOMEM);
    for_each_possible_cpu(cpu) {
    stream = per_cpu_ptr(percpu, cpu);
    stream.stream = msblk.decompressor.init(msblk, comp_opts);
    if (IS_ERR(stream.stream)) {
    err = PTR_ERR(stream.stream);
    goto out;
    }
    local_lock_init(&stream.lock);
    }
    kfree(comp_opts);
    return (void *)( unsigned long) percpu;
    out:
    for_each_possible_cpu(cpu) {
    stream = per_cpu_ptr(percpu, cpu);
    if (!IS_ERR_OR_NULL(stream.stream))
    msblk.decompressor.free(stream.stream);
    }
    free_percpu(percpu);
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn squashfs_decompressor_destroy(msblk: *mut squashfs_sb_info) {
    static void squashfs_decompressor_destroy(struct squashfs_sb_info *msblk)
    {
    struct squashfs_stream __percpu *percpu =
    (void __percpu *)(unsigned long) msblk.stream;
    struct squashfs_stream *stream;
    int cpu;
    if (msblk.stream) {
    for_each_possible_cpu(cpu) {
    stream = per_cpu_ptr(percpu, cpu);
    msblk.decompressor.free(stream.stream);
    }
    free_percpu(percpu);
    }
    }
    static int squashfs_decompress(struct squashfs_sb_info *msblk, struct bio *bio,
    int offset, int length, struct squashfs_page_actor *output)
    {
    struct squashfs_stream *stream;
    struct squashfs_stream __percpu *percpu =
    (void __percpu *)(unsigned long) msblk.stream;
    int res;
    local_lock(&percpu.lock);
    stream = this_cpu_ptr(percpu);
    res = msblk.decompressor.decompress(msblk, stream.stream, bio,
    offset, length, output);
    local_unlock(&percpu.lock);
    if (res < 0)
    ERROR("%s decompression failed, data probably corrupt\n",
    msblk.decompressor.name);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn squashfs_max_decompressors() -> c_int {
    static int squashfs_max_decompressors(void)
    {
    return num_possible_cpus();
    }
    const struct squashfs_decompressor_thread_ops squashfs_decompressor_percpu = {
    .create = squashfs_decompressor_create,
    .destroy = squashfs_decompressor_destroy,
    .decompress = squashfs_decompress,
    .max_decompressors = squashfs_max_decompressors,
    };
