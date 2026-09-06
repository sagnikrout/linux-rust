//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/cell/spufs/gang.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// SPU file system
//
// (C) Copyright IBM Deutschland Entwicklung GmbH 2005
//
// Author: Arnd Bergmann <arndb@de.ibm.com>
//

    struct spu_gang *alloc_spu_gang(void)
    {
    struct spu_gang *gang;
    gang = kzalloc_obj(*gang);
    if (!gang)
    goto out;
    kref_init(&gang.kref);
    mutex_init(&gang.mutex);
    mutex_init(&gang.aff_mutex);
    INIT_LIST_HEAD(&gang.list);
    INIT_LIST_HEAD(&gang.aff_list_head);
    gang.alive = 1;
    out:
    return gang;
    }
#[no_mangle]
unsafe extern "C" fn destroy_spu_gang(kref: *mut kref) {
    static void destroy_spu_gang(struct kref *kref)
    {
    struct spu_gang *gang;
    gang = container_of(kref, struct spu_gang, kref);
    WARN_ON(gang.contexts || !list_empty(&gang.list));
    kfree(gang);
    }
    struct spu_gang *get_spu_gang(struct spu_gang *gang)
    {
    kref_get(&gang.kref);
    return gang;
    }
#[no_mangle]
pub unsafe extern "C" fn put_spu_gang(gang: *mut spu_gang) -> c_int {
    int put_spu_gang(struct spu_gang *gang)
    {
    return kref_put(&gang.kref, &destroy_spu_gang);
    }
#[no_mangle]
pub unsafe extern "C" fn spu_gang_add_ctx(gang: *mut spu_gang, ctx: *mut spu_context) {
    void spu_gang_add_ctx(struct spu_gang *gang, struct spu_context *ctx)
    {
    mutex_lock(&gang.mutex);
    ctx.gang = get_spu_gang(gang);
    list_add(&ctx.gang_list, &gang.list);
    gang.contexts++;
    mutex_unlock(&gang.mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn spu_gang_remove_ctx(gang: *mut spu_gang, ctx: *mut spu_context) {
    void spu_gang_remove_ctx(struct spu_gang *gang, struct spu_context *ctx)
    {
    mutex_lock(&gang.mutex);
    WARN_ON(ctx.gang != gang);
    if (!list_empty(&ctx.aff_list)) {
    list_del_init(&ctx.aff_list);
    gang.aff_flags &= ~AFF_OFFSETS_SET;
    }
    list_del_init(&ctx.gang_list);
    gang.contexts--;
    mutex_unlock(&gang.mutex);
    put_spu_gang(gang);
    }
