//! Automatically rewritten from C to Rust
//! Source: net/ceph/buffer.c
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

    struct ceph_buffer *ceph_buffer_new(size_t len, gfp_t gfp)
    {
    struct ceph_buffer *b;
    b = kmalloc_obj(*b, gfp);
    if (!b)
    return core::ptr::null_mut();
    b.vec.iov_base = kvmalloc(len, gfp);
    if (!b.vec.iov_base) {
    kfree(b);
    return core::ptr::null_mut();
    }
    kref_init(&b.kref);
    b.alloc_len = len;
    b.vec.iov_len = len;
    dout("buffer_new %p\n", b);
    return b;
    }
    EXPORT_SYMBOL(ceph_buffer_new);
#[no_mangle]
pub unsafe extern "C" fn ceph_buffer_release(kref: *mut kref) {
    void ceph_buffer_release(struct kref *kref)
    {
    struct ceph_buffer *b = container_of(kref, struct ceph_buffer, kref);
    dout("buffer_release %p\n", b);
    kvfree(b.vec.iov_base);
    kfree(b);
    }
    EXPORT_SYMBOL(ceph_buffer_release);
#[no_mangle]
pub unsafe extern "C" fn ceph_decode_buffer(b: *mut ceph_buffer, p: *mut c_void, end: *mut c_void) -> c_int {
    int ceph_decode_buffer(struct ceph_buffer **b, void **p, void *end)
    {
    size_t len;
    ceph_decode_need(p, end, sizeof(u32), bad);
    len = ceph_decode_32(p);
    dout("decode_buffer len %d\n", (int)len);
    ceph_decode_need(p, end, len, bad);
// b = ceph_buffer_new(len, GFP_NOFS);
    if (!*b)
    return -ENOMEM;
    ceph_decode_copy(p, (*b).vec.iov_base, len);
    return 0;
    bad:
    return -EINVAL;
    }
