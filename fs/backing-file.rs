//! Automatically rewritten from C to Rust
//! Source: fs/backing-file.c
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
// Common helpers for stackable filesystems and backing files.
//
// Forked from fs/overlayfs/file.c.
//
// Copyright (C) 2017 Red Hat, Inc.
// Copyright (C) 2023 CTERA Networks.
//

//
// backing_file_open - open a backing file for kernel internal use
// @user_file:  file the user requested to open
// @flags:	open flags
// @real_path:	path of the backing file
// @cred:	credentials for open
//
// Open a backing file for a stackable filesystem (e.g., overlayfs).
// @user_file->f_path may be on the stackable filesystem and @real_path
// on the underlying filesystem. In this case, we want to be able to
// return the path of the stackable filesystem. This is done by
// embedding the returned file into a container structure that also
// stores the stacked file's path, which can be retrieved using
// backing_file_user_path().
//
    struct file *backing_file_open(const struct file *user_file, int flags,
    const struct path *real_path,
    const struct cred *cred)
    {
    const struct path *user_path = file_user_path(user_file);
    struct file *f;
    int error;
    f = alloc_empty_backing_file(flags, cred, user_file);
    if (IS_ERR(f))
    return f;
    path_get(user_path);
    backing_file_set_user_path(f, user_path);
    error = vfs_open(real_path, f);
    if (error) {
    fput(f);
    f = ERR_PTR(error);
    }
    return f;
    }
    EXPORT_SYMBOL_GPL(backing_file_open);
    struct file *backing_tmpfile_open(const struct file *user_file, int flags,
    const struct path *real_parentpath,
    umode_t mode, const struct cred *cred)
    {
    struct mnt_idmap *real_idmap = mnt_idmap(real_parentpath.mnt);
    const struct path *user_path = &user_file.f_path;
    struct file *f;
    int error;
    f = alloc_empty_backing_file(flags, cred, user_file);
    if (IS_ERR(f))
    return f;
    path_get(user_path);
    backing_file_set_user_path(f, user_path);
    error = vfs_tmpfile(real_idmap, real_parentpath, f, mode);
    if (error) {
    fput(f);
    f = ERR_PTR(error);
    }
    return f;
    }
    EXPORT_SYMBOL(backing_tmpfile_open);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct backing_aio {
    pub iocb: kiocb,
    pub ref: refcount_t,
    pub orig_iocb: *mut kiocb,
// used for aio completion
    pub ssize_t): *mut *mut *mut void (end_write)(struct kiocb iocb,,
    pub work: work_struct,
    pub res: c_long,
}

    static struct kmem_cache *backing_aio_cachep;

    (IOCB_NOWAIT | IOCB_HIPRI | IOCB_DSYNC | IOCB_SYNC | IOCB_APPEND)
#[no_mangle]
unsafe extern "C" fn iocb_to_rw_flags(flags: c_int) -> rwf_t {
    static rwf_t iocb_to_rw_flags(int flags)
    {
    return ( rwf_t)(flags & BACKING_IOCB_MASK);
    }
#[no_mangle]
unsafe extern "C" fn backing_aio_put(aio: *mut backing_aio) {
    static void backing_aio_put(struct backing_aio *aio)
    {
    if (refcount_dec_and_test(&aio.ref)) {
    fput(aio.iocb.ki_filp);
    kmem_cache_free(backing_aio_cachep, aio);
    }
    }
#[no_mangle]
unsafe extern "C" fn backing_aio_cleanup(aio: *mut backing_aio, res: c_long) {
    static void backing_aio_cleanup(struct backing_aio *aio, long res)
    {
    struct kiocb *iocb = &aio.iocb;
    struct kiocb *orig_iocb = aio.orig_iocb;
    orig_iocb.ki_pos = iocb.ki_pos;
    if (aio.end_write)
    aio.end_write(orig_iocb, res);
    backing_aio_put(aio);
    }
#[no_mangle]
unsafe extern "C" fn backing_aio_rw_complete(iocb: *mut kiocb, res: c_long) {
    static void backing_aio_rw_complete(struct kiocb *iocb, long res)
    {
    struct backing_aio *aio = container_of(iocb, struct backing_aio, iocb);
    struct kiocb *orig_iocb = aio.orig_iocb;
    if (iocb.ki_flags & IOCB_WRITE)
    kiocb_end_write(iocb);
    backing_aio_cleanup(aio, res);
    orig_iocb.ki_complete(orig_iocb, res);
    }
#[no_mangle]
unsafe extern "C" fn backing_aio_complete_work(work: *mut work_struct) {
    static void backing_aio_complete_work(struct work_struct *work)
    {
    struct backing_aio *aio = container_of(work, struct backing_aio, work);
    backing_aio_rw_complete(&aio.iocb, aio.res);
    }
#[no_mangle]
unsafe extern "C" fn backing_aio_queue_completion(iocb: *mut kiocb, res: c_long) {
    static void backing_aio_queue_completion(struct kiocb *iocb, long res)
    {
    struct backing_aio *aio = container_of(iocb, struct backing_aio, iocb);
//
// Punt to a work queue to serialize updates of mtime/size.
//
    aio.res = res;
    INIT_WORK(&aio.work, backing_aio_complete_work);
    queue_work(file_inode(aio.orig_iocb.ki_filp).i_sb.s_dio_done_wq,
    &aio.work);
    }
#[no_mangle]
unsafe extern "C" fn backing_aio_init_wq(iocb: *mut kiocb) -> c_int {
    static int backing_aio_init_wq(struct kiocb *iocb)
    {
    struct super_block *sb = file_inode(iocb.ki_filp).i_sb;
    if (sb.s_dio_done_wq)
    return 0;
    return sb_init_dio_done_wq(sb);
    }
    static int do_backing_file_read_iter(struct file *file, struct iov_iter *iter,
    struct kiocb *iocb, int flags)
    {
    struct backing_aio *aio = core::ptr::null_mut();
    int ret;
    if (is_sync_kiocb(iocb)) {
    let mut rwf: rwf_t = iocb_to_rw_flags(flags);
    return vfs_iter_read(file, iter, &iocb.ki_pos, rwf);
    }
    aio = kmem_cache_zalloc(backing_aio_cachep, GFP_KERNEL);
    if (!aio)
    return -ENOMEM;
    aio.orig_iocb = iocb;
    kiocb_clone(&aio.iocb, iocb, get_file(file));
    aio.iocb.ki_complete = backing_aio_rw_complete;
    refcount_set(&aio.ref, 2);
    ret = vfs_iocb_iter_read(file, &aio.iocb, iter);
    backing_aio_put(aio);
    if (ret != -EIOCBQUEUED)
    backing_aio_cleanup(aio, ret);
    return ret;
    }
    ssize_t backing_file_read_iter(struct file *file, struct iov_iter *iter,
    struct kiocb *iocb, int flags,
    struct backing_file_ctx *ctx)
    {
    ssize_t ret;
    if (WARN_ON_ONCE(!(file.f_mode & FMODE_BACKING)))
    return -EIO;
    if (!iov_iter_count(iter))
    return 0;
    if (iocb.ki_flags & IOCB_DIRECT &&
    !(file.f_mode & FMODE_CAN_ODIRECT))
    return -EINVAL;
    scoped_with_creds(ctx.cred)
    ret = do_backing_file_read_iter(file, iter, iocb, flags);
    if (ctx.accessed)
    ctx.accessed(iocb.ki_filp);
    return ret;
    }
    EXPORT_SYMBOL_GPL(backing_file_read_iter);
    static int do_backing_file_write_iter(struct file *file, struct iov_iter *iter,
    struct kiocb *iocb, int flags,
    void (*end_write)(struct kiocb *, ssize_t))
    {
    struct backing_aio *aio;
    int ret;
    if (is_sync_kiocb(iocb)) {
    let mut rwf: rwf_t = iocb_to_rw_flags(flags);
    ret = vfs_iter_write(file, iter, &iocb.ki_pos, rwf);
    if (end_write)
    end_write(iocb, ret);
    return ret;
    }
    ret = backing_aio_init_wq(iocb);
    if (ret)
    return ret;
    aio = kmem_cache_zalloc(backing_aio_cachep, GFP_KERNEL);
    if (!aio)
    return -ENOMEM;
    aio.orig_iocb = iocb;
    aio.end_write = end_write;
    kiocb_clone(&aio.iocb, iocb, get_file(file));
    aio.iocb.ki_flags = flags;
    aio.iocb.ki_complete = backing_aio_queue_completion;
    refcount_set(&aio.ref, 2);
    ret = vfs_iocb_iter_write(file, &aio.iocb, iter);
    backing_aio_put(aio);
    if (ret != -EIOCBQUEUED)
    backing_aio_cleanup(aio, ret);
    return ret;
    }
    ssize_t backing_file_write_iter(struct file *file, struct iov_iter *iter,
    struct kiocb *iocb, int flags,
    struct backing_file_ctx *ctx)
    {
    ssize_t ret;
    if (WARN_ON_ONCE(!(file.f_mode & FMODE_BACKING)))
    return -EIO;
    if (!iov_iter_count(iter))
    return 0;
    ret = file_remove_privs(iocb.ki_filp);
    if (ret)
    return ret;
    if (iocb.ki_flags & IOCB_DIRECT &&
    !(file.f_mode & FMODE_CAN_ODIRECT))
    return -EINVAL;
    scoped_with_creds(ctx.cred)
    return do_backing_file_write_iter(file, iter, iocb, flags, ctx.end_write);
    }
    EXPORT_SYMBOL_GPL(backing_file_write_iter);
    ssize_t backing_file_splice_read(struct file *in, struct kiocb *iocb,
    struct pipe_inode_info *pipe, size_t len,
    unsigned int flags,
    struct backing_file_ctx *ctx)
    {
    ssize_t ret;
    if (WARN_ON_ONCE(!(in.f_mode & FMODE_BACKING)))
    return -EIO;
    scoped_with_creds(ctx.cred)
    ret = vfs_splice_read(in, &iocb.ki_pos, pipe, len, flags);
    if (ctx.accessed)
    ctx.accessed(iocb.ki_filp);
    return ret;
    }
    EXPORT_SYMBOL_GPL(backing_file_splice_read);
    ssize_t backing_file_splice_write(struct pipe_inode_info *pipe,
    struct file *out, struct kiocb *iocb,
    size_t len, unsigned int flags,
    struct backing_file_ctx *ctx)
    {
    ssize_t ret;
    if (WARN_ON_ONCE(!(out.f_mode & FMODE_BACKING)))
    return -EIO;
    if (!out.f_op.splice_write)
    return -EINVAL;
    ret = file_remove_privs(iocb.ki_filp);
    if (ret)
    return ret;
    scoped_with_creds(ctx.cred) {
    file_start_write(out);
    ret = out.f_op.splice_write(pipe, out, &iocb.ki_pos, len, flags);
    file_end_write(out);
    }
    if (ctx.end_write)
    ctx.end_write(iocb, ret);
    return ret;
    }
    EXPORT_SYMBOL_GPL(backing_file_splice_write);
    int backing_file_mmap(struct file *file, struct vm_area_struct *vma,
    struct backing_file_ctx *ctx)
    {
    struct file *user_file = vma.vm_file;
    int ret;
    if (WARN_ON_ONCE(!(file.f_mode & FMODE_BACKING)))
    return -EIO;
    if (!can_mmap_file(file))
    return -ENODEV;
    vma_set_file(vma, file);
    scoped_with_creds(ctx.cred) {
    ret = security_mmap_backing_file(vma, file, user_file);
    if (ret)
    return ret;
    ret = vfs_mmap(vma.vm_file, vma);
    }
    if (ctx.accessed)
    ctx.accessed(user_file);
    return ret;
    }
    EXPORT_SYMBOL_GPL(backing_file_mmap);
#[no_mangle]
unsafe extern "C" fn backing_aio_init() -> int __init {
    static int __init backing_aio_init(void)
    {
    backing_aio_cachep = KMEM_CACHE(backing_aio, SLAB_HWCACHE_ALIGN);
    if (!backing_aio_cachep)
    return -ENOMEM;
    return 0;
    }
    fs_initcall(backing_aio_init);
