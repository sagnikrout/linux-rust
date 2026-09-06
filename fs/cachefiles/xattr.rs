//! Automatically rewritten from C to Rust
//! Source: fs/cachefiles/xattr.c
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
// CacheFiles extended attribute management
//
// Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

pub const CACHEFILES_COOKIE_TYPE_DATA: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cachefiles_xattr {
    pub /: *mut *mut __be64 object_size; / Actual size of the object,
    pub /: *mut *mut __be64 zero_point; / Size after which server has no data not written by us,
    pub /: *mut *mut __u8 type; / Type of object,
    pub /: *mut *mut __u8 content; / Content presence (enum cachefiles_content),
    pub /: *mut *mut __u8 data[]; / netfs coherency data,
    pub __packed: },
    static const char cachefiles_xattr_cache[] =
    pub "CacheFiles.cache": XATTR_USER_PREFIX,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cachefiles_vol_xattr {
    pub /: *mut *mut __be32 reserved; / Reserved, should be 0,
    pub /: *mut *mut __u8 data[]; / netfs volume coherency data,
    pub __packed: },
//
// set the state xattr on a cache file
//
#[no_mangle]
pub unsafe extern "C" fn cachefiles_set_object_xattr(object: *mut cachefiles_object) -> c_int {
    int cachefiles_set_object_xattr(struct cachefiles_object *object)
    {
    pub buf: *mut cachefiles_xattr,
    pub dentry: *mut dentry,
    pub object->file: *mut *mut file file =,
    pub object->cookie->aux_len: unsigned int len =,
    pub ret: c_int,
    if (!file)
    pub -ESTALE: return,
    pub file->f_path.dentry: dentry =,
    pub len): _enter("%x,#%d", object->debug_id,,
    pub GFP_KERNEL): buf = kmalloc(sizeof(struct cachefiles_xattr) + len,,
    if (!buf)
    pub -ENOMEM: return,
    pub cpu_to_be64(object->cookie->object_size): buf->object_size =,
    pub 0: buf->zero_point =,
    pub CACHEFILES_COOKIE_TYPE_DATA: buf->type =,
    pub object->content_info: buf->content =,
    if (test_bit(FSCACHE_COOKIE_LOCAL_WRITE, &object.cookie.flags))
    pub CACHEFILES_CONTENT_DIRTY: buf->content =,
    if (len > 0)
    pub len): memcpy(buf->data, fscache_get_aux(object->cookie),,
    pub cachefiles_inject_write_error(): ret =,
    if (ret == 0) {
    pub mnt_want_write_file(file): ret =,
    if (ret == 0) {
    ret = vfs_setxattr(&nop_mnt_idmap, dentry,
    cachefiles_xattr_cache, buf,
    pub 0): sizeof(struct cachefiles_xattr) + len,,
    }
    }
    if (ret < 0) {
    trace_cachefiles_vfs_error(object, file_inode(file), ret,
    trace_cachefiles_coherency(object, file_inode(file).i_ino,
    be64_to_cpup((__be64 *)buf.data),
    buf.content,
    if (ret != -ENOMEM)
    cachefiles_io_error_obj(
    object,
    pub ret): "Failed to set xattr with error %d",,
    } else {
    trace_cachefiles_coherency(object, file_inode(file).i_ino,
    be64_to_cpup((__be64 *)buf.data),
    buf.content,
    }
    pub ret): _leave(" = %d",,
    pub ret: return,
    }
//
// check the consistency between the backing cache and the FS-Cache cookie
//
#[no_mangle]
pub unsafe extern "C" fn cachefiles_check_auxdata(object: *mut cachefiles_object, file: *mut file) -> c_int {
    int cachefiles_check_auxdata(struct cachefiles_object *object, struct file *file)
    {
    pub buf: *mut cachefiles_xattr,
    pub file->f_path.dentry: *mut *mut dentry dentry =,
    pub tlen: unsigned int len = object->cookie->aux_len,,
    pub fscache_get_aux(object->cookie): *const *const void p =,
    pub why: enum cachefiles_coherency_trace,
    pub xlen: isize,
    pub -ESTALE: int ret =,
    pub len: tlen = sizeof(struct cachefiles_xattr) +,
    pub GFP_KERNEL): buf = kmalloc(tlen,,
    if (!buf)
    pub -ENOMEM: return,
    pub cachefiles_inject_read_error(): xlen =,
    if (xlen == 0)
    pub tlen): xlen = vfs_getxattr(&nop_mnt_idmap, dentry, cachefiles_xattr_cache, buf,,
    if (xlen != tlen) {
    if (xlen < 0) {
    pub xlen: ret =,
    trace_cachefiles_vfs_error(object, file_inode(file), xlen,
    }
    if (xlen == -EIO)
    cachefiles_io_error_obj(
    object,
    pub xlen): "Failed to read aux with error %zd",,
    pub cachefiles_coherency_check_xattr: why =,
    pub out: goto,
    }
    if (buf.type != CACHEFILES_COOKIE_TYPE_DATA) {
    pub cachefiles_coherency_check_type: why =,
    } else if (memcmp(buf.data, p, len) != 0) {
    pub cachefiles_coherency_check_aux: why =,
    } else if (be64_to_cpu(buf.object_size) != object.cookie.object_size) {
    pub cachefiles_coherency_check_objsize: why =,
    } else if (buf.content == CACHEFILES_CONTENT_DIRTY) {
// TODO: Begin conflict resolution
    pub cache\n"): pr_warn("Dirty object in,
    pub cachefiles_coherency_check_dirty: why =,
    } else {
    pub cachefiles_coherency_check_ok: why =,
    pub 0: ret =,
    }
    out:
    trace_cachefiles_coherency(object, file_inode(file).i_ino,
    be64_to_cpup((__be64 *)buf.data),
    pub why): buf->content,,
    pub ret: return,
    }
//
// remove the object's xattr to mark it stale
//
    int cachefiles_remove_object_xattr(struct cachefiles_cache *cache,
    struct cachefiles_object *object,
    struct dentry *dentry)
    {
    pub ret: c_int,
    pub cachefiles_inject_remove_error(): ret =,
    if (ret == 0) {
    pub mnt_want_write(cache->mnt): ret =,
    if (ret == 0) {
    ret = vfs_removexattr(&nop_mnt_idmap, dentry,
    }
    }
    if (ret < 0) {
    trace_cachefiles_vfs_error(object, d_inode(dentry), ret,
    if (ret == -ENOENT || ret == -ENODATA)
    pub 0: ret =,
#[no_mangle]
pub unsafe extern "C" fn if(-ENOMEM: ret !=) -> else {
    else if (ret != -ENOMEM)
    cachefiles_io_error(cache,
    "Can't remove xattr from %llu"
    " (error %d)",
    pub -ret): d_backing_inode(dentry)->i_ino,,
    }
    pub ret): _leave(" = %d",,
    pub ret: return,
    }
//
// Stick a marker on the cache object to indicate that it's dirty.
//
#[no_mangle]
pub unsafe extern "C" fn cachefiles_prepare_to_write(cookie: *mut fscache_cookie) {
    void cachefiles_prepare_to_write(struct fscache_cookie *cookie)
    {
    pub saved_cred: *const cred,
    pub cookie->cache_priv: *mut *mut cachefiles_object object =,
    pub object->volume->cache: *mut *mut cachefiles_cache cache =,
    pub object->cookie->debug_id): _enter("c=%08x",,
    if (!test_bit(CACHEFILES_OBJECT_USING_TMPFILE, &object.flags)) {
    pub &saved_cred): cachefiles_begin_secure(cache,,
    pub saved_cred): cachefiles_end_secure(cache,,
    }
    }
//
// Set the state xattr on a volume directory.
//
#[no_mangle]
pub unsafe extern "C" fn cachefiles_set_volume_xattr(volume: *mut cachefiles_volume) -> bool {
    bool cachefiles_set_volume_xattr(struct cachefiles_volume *volume)
    {
    pub buf: *mut cachefiles_vol_xattr,
    pub volume->vcookie->coherency_len: unsigned int len =,
    pub volume->vcookie->coherency: *const *const void p =,
    pub volume->dentry: *mut *mut dentry dentry =,
    pub ret: c_int,
    pub len): _enter("%x,#%d", volume->vcookie->debug_id,,
    pub sizeof(*buf): *mut len +=,
    pub GFP_KERNEL): buf = kmalloc(len,,
    if (!buf)
    pub false: return,
    pub cpu_to_be32(0): buf->reserved =,
    pub volume->vcookie->coherency_len): memcpy(buf->data, p,,
    pub cachefiles_inject_write_error(): ret =,
    if (ret == 0) {
    pub mnt_want_write(volume->cache->mnt): ret =,
    if (ret == 0) {
    ret = vfs_setxattr(&nop_mnt_idmap, dentry,
    cachefiles_xattr_cache,
    pub 0): buf, len,,
    }
    }
    if (ret < 0) {
    trace_cachefiles_vfs_error(core::ptr::null_mut(), d_inode(dentry), ret,
    trace_cachefiles_vol_coherency(volume, d_inode(dentry).i_ino,
    if (ret != -ENOMEM)
    cachefiles_io_error(
    pub ret): volume->cache, "Failed to set xattr with error %d",,
    } else {
    trace_cachefiles_vol_coherency(volume, d_inode(dentry).i_ino,
    }
    pub ret): _leave(" = %d",,
    pub 0: return ret ==,
    }
//
// Check the consistency between the backing cache and the volume cookie.
//
#[no_mangle]
pub unsafe extern "C" fn cachefiles_check_volume_xattr(volume: *mut cachefiles_volume) -> c_int {
    int cachefiles_check_volume_xattr(struct cachefiles_volume *volume)
    {
    pub buf: *mut cachefiles_vol_xattr,
    pub volume->dentry: *mut *mut dentry dentry =,
    pub volume->vcookie->coherency_len: unsigned int len =,
    pub volume->vcookie->coherency: *const *const void p =,
    pub why: enum cachefiles_coherency_trace,
    pub xlen: isize,
    pub -ESTALE: int ret =,
    pub sizeof(*buf): *mut len +=,
    pub GFP_KERNEL): buf = kmalloc(len,,
    if (!buf)
    pub -ENOMEM: return,
    pub cachefiles_inject_read_error(): xlen =,
    if (xlen == 0)
    pub len): xlen = vfs_getxattr(&nop_mnt_idmap, dentry, cachefiles_xattr_cache, buf,,
    if (xlen != len) {
    if (xlen < 0) {
    pub xlen: ret =,
    trace_cachefiles_vfs_error(core::ptr::null_mut(), d_inode(dentry), xlen,
    if (xlen == -EIO)
    cachefiles_io_error(
    volume.cache,
    pub xlen): "Failed to read xattr with error %zd",,
    }
    pub cachefiles_coherency_vol_check_xattr: why =,
    } else if (buf.reserved != cpu_to_be32(0)) {
    pub cachefiles_coherency_vol_check_resv: why =,
    } else if (memcmp(buf.data, p, len - sizeof(*buf)) != 0) {
    pub cachefiles_coherency_vol_check_cmp: why =,
    } else {
    pub cachefiles_coherency_vol_check_ok: why =,
    pub 0: ret =,
    }
    pub why): trace_cachefiles_vol_coherency(volume, d_inode(dentry)->i_ino,,
    pub ret): _leave(" = %d",,
    pub ret: return,
    }
