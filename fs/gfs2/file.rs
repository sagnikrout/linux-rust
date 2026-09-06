//! Automatically rewritten from C to Rust
//! Source: fs/gfs2/file.c
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
// Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
// Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
//

//
// gfs2_llseek - seek to a location in a file
// @file: the file
// @offset: the offset
// @whence: Where to seek from (SEEK_SET, SEEK_CUR, or SEEK_END)
//
// SEEK_END requires the glock for the file because it references the
// file's size.
//
// Returns: The new offset, or errno
//
#[no_mangle]
unsafe extern "C" fn gfs2_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t {
    static loff_t gfs2_llseek(struct file *file, loff_t offset, int whence)
    {
    struct gfs2_inode *ip = GFS2_I(file.f_mapping.host);
    struct gfs2_holder i_gh;
    loff_t error;
    switch (whence) {
    case SEEK_END:
    error = gfs2_glock_nq_init(ip.i_gl, LM_ST_SHARED, LM_FLAG_ANY,
    &i_gh);
    if (!error) {
    error = generic_file_llseek(file, offset, whence);
    gfs2_glock_dq_uninit(&i_gh);
    }
    break;
    case SEEK_DATA:
    error = gfs2_seek_data(file, offset);
    break;
    case SEEK_HOLE:
    error = gfs2_seek_hole(file, offset);
    break;
    case SEEK_CUR:
    case SEEK_SET:
//
// These don't reference inode->i_size and don't depend on the
// block mapping, so we don't need the glock.
//
    error = generic_file_llseek(file, offset, whence);
    break;
    default:
    error = -EINVAL;
    }
    return error;
    }
//
// gfs2_readdir - Iterator for a directory
// @file: The directory to read from
// @ctx: What to feed directory entries to
//
// Returns: errno
//
#[no_mangle]
unsafe extern "C" fn gfs2_readdir(file: *mut file, ctx: *mut dir_context) -> c_int {
    static int gfs2_readdir(struct file *file, struct dir_context *ctx)
    {
    struct inode *dir = file.f_mapping.host;
    struct gfs2_inode *dip = GFS2_I(dir);
    struct gfs2_holder d_gh;
    int error;
    error = gfs2_glock_nq_init(dip.i_gl, LM_ST_SHARED, 0, &d_gh);
    if (error)
    return error;
    error = gfs2_dir_read(dir, ctx, &file.f_ra);
    gfs2_glock_dq_uninit(&d_gh);
    return error;
    }
//
// struct fsflag_gfs2flag
//
// The FS_JOURNAL_DATA_FL flag maps to GFS2_DIF_INHERIT_JDATA for directories,
// and to GFS2_DIF_JDATA for non-directories.
//
    static struct {
    u32 fsflag;
    u32 gfsflag;
    } fsflag_gfs2flag[] = {
    {FS_SYNC_FL, GFS2_DIF_SYNC},
    {FS_IMMUTABLE_FL, GFS2_DIF_IMMUTABLE},
    {FS_APPEND_FL, GFS2_DIF_APPENDONLY},
    {FS_NOATIME_FL, GFS2_DIF_NOATIME},
    {FS_INDEX_FL, GFS2_DIF_EXHASH},
    {FS_TOPDIR_FL, GFS2_DIF_TOPDIR},
    {FS_JOURNAL_DATA_FL, GFS2_DIF_JDATA | GFS2_DIF_INHERIT_JDATA},
    };
#[no_mangle]
pub unsafe extern "C" fn gfs2_gfsflags_to_fsflags(inode: *mut inode, gfsflags: u32) -> u32 {
    static inline u32 gfs2_gfsflags_to_fsflags(struct inode *inode, u32 gfsflags)
    {
    int i;
    let mut fsflags: u32 = 0;
    if (S_ISDIR(inode.i_mode))
    gfsflags &= ~GFS2_DIF_JDATA;
    else
    gfsflags &= ~GFS2_DIF_INHERIT_JDATA;
    for (i = 0; i < ARRAY_SIZE(fsflag_gfs2flag); i++)
    if (gfsflags & fsflag_gfs2flag[i].gfsflag)
    fsflags |= fsflag_gfs2flag[i].fsflag;
    return fsflags;
    }
#[no_mangle]
pub unsafe extern "C" fn gfs2_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int {
    int gfs2_fileattr_get(struct dentry *dentry, struct file_kattr *fa)
    {
    struct inode *inode = d_inode(dentry);
    struct gfs2_inode *ip = GFS2_I(inode);
    struct gfs2_holder gh;
    int error;
    u32 fsflags;
    if (d_is_special(dentry))
    return -ENOTTY;
    gfs2_holder_init(ip.i_gl, LM_ST_SHARED, 0, &gh);
    error = gfs2_glock_nq(&gh);
    if (error)
    goto out_uninit;
    fsflags = gfs2_gfsflags_to_fsflags(inode, ip.i_diskflags);
    fileattr_fill_flags(fa, fsflags);
    gfs2_glock_dq(&gh);
    out_uninit:
    gfs2_holder_uninit(&gh);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn gfs2_set_inode_flags(inode: *mut inode) {
    void gfs2_set_inode_flags(struct inode *inode)
    {
    struct gfs2_inode *ip = GFS2_I(inode);
    let mut flags: c_uint = inode.i_flags;
    flags &= ~(S_SYNC|S_APPEND|S_IMMUTABLE|S_NOATIME|S_DIRSYNC|S_NOSEC);
    if ((ip.i_eattr == 0) && !is_sxid(inode.i_mode))
    flags |= S_NOSEC;
    if (ip.i_diskflags & GFS2_DIF_IMMUTABLE)
    flags |= S_IMMUTABLE;
    if (ip.i_diskflags & GFS2_DIF_APPENDONLY)
    flags |= S_APPEND;
    if (ip.i_diskflags & GFS2_DIF_NOATIME)
    flags |= S_NOATIME;
    if (ip.i_diskflags & GFS2_DIF_SYNC)
    flags |= S_SYNC;
    inode.i_flags = flags;
    }
// Flags that can be set by user space

    GFS2_DIF_IMMUTABLE|		\
    GFS2_DIF_APPENDONLY|		\
    GFS2_DIF_NOATIME|			\
    GFS2_DIF_SYNC|			\
    GFS2_DIF_TOPDIR|			\
    GFS2_DIF_INHERIT_JDATA)
//
// do_gfs2_set_flags - set flags on an inode
// @inode: The inode
// @reqflags: The flags to set
// @mask: Indicates which flags are valid
//
#[no_mangle]
unsafe extern "C" fn do_gfs2_set_flags(inode: *mut inode, reqflags: u32, mask: u32) -> c_int {
    static int do_gfs2_set_flags(struct inode *inode, u32 reqflags, u32 mask)
    {
    struct gfs2_inode *ip = GFS2_I(inode);
    struct gfs2_sbd *sdp = GFS2_SB(inode);
    struct buffer_head *bh;
    struct gfs2_holder gh;
    int error;
    u32 new_flags, flags;
    error = gfs2_glock_nq_init(ip.i_gl, LM_ST_EXCLUSIVE, 0, &gh);
    if (error)
    return error;
    error = 0;
    flags = ip.i_diskflags;
    new_flags = (flags & ~mask) | (reqflags & mask);
    if ((new_flags ^ flags) == 0)
    goto out;
    if (!IS_IMMUTABLE(inode)) {
    error = gfs2_permission(&nop_mnt_idmap, inode, MAY_WRITE);
    if (error)
    goto out;
    }
    if ((flags ^ new_flags) & GFS2_DIF_JDATA) {
    if (new_flags & GFS2_DIF_JDATA)
    gfs2_log_flush(sdp, ip.i_gl,
    GFS2_LOG_HEAD_FLUSH_NORMAL |
    GFS2_LFC_SET_FLAGS);
    error = filemap_fdatawrite(inode.i_mapping);
    if (error)
    goto out;
    error = filemap_fdatawait(inode.i_mapping);
    if (error)
    goto out;
    truncate_inode_pages(inode.i_mapping, 0);
    if (new_flags & GFS2_DIF_JDATA)
    gfs2_ordered_del_inode(ip);
    }
    error = gfs2_trans_begin(sdp, RES_DINODE, 0);
    if (error)
    goto out;
    error = gfs2_meta_inode_buffer(ip, &bh);
    if (error)
    goto out_trans_end;
    inode_set_ctime_current(inode);
    gfs2_trans_add_meta(ip.i_gl, bh);
    ip.i_diskflags = new_flags;
    gfs2_dinode_out(ip, bh.b_data);
    brelse(bh);
    gfs2_set_inode_flags(inode);
    gfs2_set_aops(inode);
    out_trans_end:
    gfs2_trans_end(sdp);
    out:
    gfs2_glock_dq_uninit(&gh);
    return error;
    }
    int gfs2_fileattr_set(struct mnt_idmap *idmap,
    struct dentry *dentry, struct file_kattr *fa)
    {
    struct inode *inode = d_inode(dentry);
    let mut fsflags: u32 = fa.flags, gfsflags = 0;
    u32 mask;
    int i;
    if (d_is_special(dentry))
    return -ENOTTY;
    if (fileattr_has_fsx(fa))
    return -EOPNOTSUPP;
    for (i = 0; i < ARRAY_SIZE(fsflag_gfs2flag); i++) {
    if (fsflags & fsflag_gfs2flag[i].fsflag) {
    fsflags &= ~fsflag_gfs2flag[i].fsflag;
    gfsflags |= fsflag_gfs2flag[i].gfsflag;
    }
    }
    if (fsflags || gfsflags & ~GFS2_FLAGS_USER_SET)
    return -EINVAL;
    mask = GFS2_FLAGS_USER_SET;
    if (S_ISDIR(inode.i_mode)) {
    mask &= ~GFS2_DIF_JDATA;
    } else {
// The GFS2_DIF_TOPDIR flag is only valid for directories.
    if (gfsflags & GFS2_DIF_TOPDIR)
    return -EINVAL;
    mask &= ~(GFS2_DIF_TOPDIR | GFS2_DIF_INHERIT_JDATA);
    }
    return do_gfs2_set_flags(inode, gfsflags, mask);
    }
#[no_mangle]
unsafe extern "C" fn gfs2_getlabel(filp: *mut file, label: *mut char __user) -> c_int {
    static int gfs2_getlabel(struct file *filp, char __user *label)
    {
    struct inode *inode = file_inode(filp);
    struct gfs2_sbd *sdp = GFS2_SB(inode);
    if (copy_to_user(label, sdp.sd_sb.sb_locktable, GFS2_LOCKNAME_LEN))
    return -EFAULT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gfs2_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long gfs2_ioctl(struct file *filp, unsigned int cmd, unsigned long arg)
    {
    switch(cmd) {
    case FITRIM:
    return gfs2_fitrim(filp, (void __user *)arg);
    case FS_IOC_GETFSLABEL:
    return gfs2_getlabel(filp, (char __user *)arg);
    }
    return -ENOTTY;
    }

#[no_mangle]
unsafe extern "C" fn gfs2_compat_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long gfs2_compat_ioctl(struct file *filp, unsigned int cmd, unsigned long arg)
    {
    switch(cmd) {
// Keep this list in sync with gfs2_ioctl
    case FITRIM:
    case FS_IOC_GETFSLABEL:
    break;
    default:
    return -ENOIOCTLCMD;
    }
    return gfs2_ioctl(filp, cmd, (unsigned long)compat_ptr(arg));
    }

//
// gfs2_size_hint - Give a hint to the size of a write request
// @filep: The struct file
// @offset: The file offset of the write
// @size: The length of the write
//
// When we are about to do a write, this function records the total
// write size in order to provide a suitable hint to the lower layers
// about how many blocks will be required.
//
#[no_mangle]
unsafe extern "C" fn gfs2_size_hint(filep: *mut file, offset: loff_t, size: usize) {
    static void gfs2_size_hint(struct file *filep, loff_t offset, size_t size)
    {
    struct inode *inode = file_inode(filep);
    struct gfs2_sbd *sdp = GFS2_SB(inode);
    struct gfs2_inode *ip = GFS2_I(inode);
    let mut blks: usize = (size + sdp.sd_sb.sb_bsize - 1) >> sdp.sd_sb.sb_bsize_shift;
    let mut hint: c_int = min_t(size_t, INT_MAX, blks);
    if (hint > atomic_read(&ip.i_sizehint))
    atomic_set(&ip.i_sizehint, hint);
    }
//
// gfs2_allocate_folio_backing - Allocate blocks for a write fault
// @folio: The (locked) folio to allocate backing for
// @length: Size of the allocation
//
// We try to allocate all the blocks required for the folio in one go.  This
// might fail for various reasons, so we keep trying until all the blocks to
// back this folio are allocated.  If some of the blocks are already allocated,
// that is ok too.
//
#[no_mangle]
unsafe extern "C" fn gfs2_allocate_folio_backing(folio: *mut folio, length: usize) -> c_int {
    static int gfs2_allocate_folio_backing(struct folio *folio, size_t length)
    {
    let mut pos: u64 = folio_pos(folio);
    do {
    let mut iomap: iomap = { };
    if (gfs2_iomap_alloc(folio.mapping.host, pos, length, &iomap))
    return -EIO;
    if (length < iomap.length)
    iomap.length = length;
    length -= iomap.length;
    pos += iomap.length;
    } while (length > 0);
    return 0;
    }
//
// gfs2_page_mkwrite - Make a shared, mmap()ed, page writable
// @vmf: The virtual memory fault containing the page to become writable
//
// When the page becomes writable, we need to ensure that we have
// blocks allocated on disk to back that page.
//
#[no_mangle]
unsafe extern "C" fn gfs2_page_mkwrite(vmf: *mut vm_fault) -> vm_fault_t {
    static vm_fault_t gfs2_page_mkwrite(struct vm_fault *vmf)
    {
    struct folio *folio = page_folio(vmf.page);
    struct inode *inode = file_inode(vmf.vma.vm_file);
    struct gfs2_inode *ip = GFS2_I(inode);
    struct gfs2_sbd *sdp = GFS2_SB(inode);
    let mut ap: gfs2_alloc_parms = {};
    let mut pos: u64 = folio_pos(folio);
    unsigned int data_blocks, ind_blocks, rblocks;
    let mut ret: vm_fault_t = VM_FAULT_LOCKED;
    struct gfs2_holder gh;
    size_t length;
    loff_t size;
    int err;
    sb_start_pagefault(inode.i_sb);
    gfs2_holder_init(ip.i_gl, LM_ST_EXCLUSIVE, 0, &gh);
    err = gfs2_glock_nq(&gh);
    if (err) {
    ret = vmf_fs_error(err);
    goto out_uninit;
    }
// Check folio index against inode size
    size = i_size_read(inode);
    if (pos >= size) {
    ret = VM_FAULT_SIGBUS;
    goto out_unlock;
    }
// Update file times before taking folio lock
    file_update_time(vmf.vma.vm_file);
// folio is wholly or partially inside EOF
    if (size - pos < folio_size(folio))
    length = size - pos;
    else
    length = folio_size(folio);
    gfs2_size_hint(vmf.vma.vm_file, pos, length);
    set_bit(GLF_DIRTY, &ip.i_gl.gl_flags);
    set_bit(GIF_SW_PAGED, &ip.i_flags);
//
// iomap_writepage / iomap_writepages currently don't support inline
// files, so always unstuff here.
//
    if (!gfs2_is_stuffed(ip) &&
    !gfs2_write_alloc_required(ip, pos, length)) {
    folio_lock(folio);
    if (!folio_test_uptodate(folio) ||
    folio.mapping != inode.i_mapping) {
    ret = VM_FAULT_NOPAGE;
    folio_unlock(folio);
    }
    goto out_unlock;
    }
    err = gfs2_rindex_update(sdp);
    if (err) {
    ret = vmf_fs_error(err);
    goto out_unlock;
    }
    gfs2_write_calc_reserv(ip, length, &data_blocks, &ind_blocks);
    ap.target = data_blocks + ind_blocks;
    err = gfs2_quota_lock_check(ip, &ap);
    if (err) {
    ret = vmf_fs_error(err);
    goto out_unlock;
    }
    err = gfs2_inplace_reserve(ip, &ap);
    if (err) {
    ret = vmf_fs_error(err);
    goto out_quota_unlock;
    }
    rblocks = RES_DINODE + ind_blocks;
    if (gfs2_is_jdata(ip))
    rblocks += data_blocks ? data_blocks : 1;
    if (ind_blocks || data_blocks) {
    rblocks += RES_STATFS + RES_QUOTA;
    rblocks += gfs2_rg_blocks(ip, data_blocks + ind_blocks);
    }
    err = gfs2_trans_begin(sdp, rblocks, 0);
    if (err) {
    ret = vmf_fs_error(err);
    goto out_trans_fail;
    }
// Unstuff, if required, and allocate backing blocks for folio
    if (gfs2_is_stuffed(ip)) {
    err = gfs2_unstuff_dinode(ip);
    if (err) {
    ret = vmf_fs_error(err);
    goto out_trans_end;
    }
    }
    folio_lock(folio);
// If truncated, we must retry the operation, we may have raced
// with the glock demotion code.
//
    if (!folio_test_uptodate(folio) || folio.mapping != inode.i_mapping) {
    ret = VM_FAULT_NOPAGE;
    goto out_page_locked;
    }
    err = gfs2_allocate_folio_backing(folio, length);
    if (err)
    ret = vmf_fs_error(err);
    out_page_locked:
    if (ret != VM_FAULT_LOCKED)
    folio_unlock(folio);
    out_trans_end:
    gfs2_trans_end(sdp);
    out_trans_fail:
    gfs2_inplace_release(ip);
    out_quota_unlock:
    gfs2_quota_unlock(ip);
    out_unlock:
    gfs2_glock_dq(&gh);
    out_uninit:
    gfs2_holder_uninit(&gh);
    if (ret == VM_FAULT_LOCKED) {
    folio_mark_dirty(folio);
    folio_wait_stable(folio);
    }
    sb_end_pagefault(inode.i_sb);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gfs2_fault(vmf: *mut vm_fault) -> vm_fault_t {
    static vm_fault_t gfs2_fault(struct vm_fault *vmf)
    {
    struct inode *inode = file_inode(vmf.vma.vm_file);
    struct gfs2_inode *ip = GFS2_I(inode);
    struct gfs2_holder gh;
    vm_fault_t ret;
    int err;
    gfs2_holder_init(ip.i_gl, LM_ST_SHARED, 0, &gh);
    err = gfs2_glock_nq(&gh);
    if (err) {
    ret = vmf_fs_error(err);
    goto out_uninit;
    }
    ret = filemap_fault(vmf);
    gfs2_glock_dq(&gh);
    out_uninit:
    gfs2_holder_uninit(&gh);
    return ret;
    }
    static const struct vm_operations_struct gfs2_vm_ops = {
    .fault = gfs2_fault,
    .map_pages = filemap_map_pages,
    .page_mkwrite = gfs2_page_mkwrite,
    };
//
// gfs2_mmap
// @file: The file to map
// @vma: The VMA which described the mapping
//
// There is no need to get a lock here unless we should be updating
// atime. We ignore any locking errors since the only consequence is
// a missed atime update (which will just be deferred until later).
//
// Returns: 0
//
#[no_mangle]
unsafe extern "C" fn gfs2_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    static int gfs2_mmap(struct file *file, struct vm_area_struct *vma)
    {
    struct gfs2_inode *ip = GFS2_I(file.f_mapping.host);
    if (!(file.f_flags & O_NOATIME) &&
    !IS_NOATIME(&ip.i_inode)) {
    struct gfs2_holder i_gh;
    int error;
    error = gfs2_glock_nq_init(ip.i_gl, LM_ST_SHARED, LM_FLAG_ANY,
    &i_gh);
    if (error)
    return error;
// grab lock to update inode
    gfs2_glock_dq_uninit(&i_gh);
    file_accessed(file);
    }
    vma.vm_ops = &gfs2_vm_ops;
    return 0;
    }
//
// gfs2_open_common - This is common to open and atomic_open
// @inode: The inode being opened
// @file: The file being opened
//
// This maybe called under a glock or not depending upon how it has
// been called. We must always be called under a glock for regular
// files, however. For other file types, it does not matter whether
// we hold the glock or not.
//
// Returns: Error code or 0 for success
//
#[no_mangle]
pub unsafe extern "C" fn gfs2_open_common(inode: *mut inode, file: *mut file) -> c_int {
    int gfs2_open_common(struct inode *inode, struct file *file)
    {
    struct gfs2_file *fp;
    int ret;
    if (S_ISREG(inode.i_mode)) {
    ret = generic_file_open(inode, file);
    if (ret)
    return ret;
    if (!gfs2_is_jdata(GFS2_I(inode)))
    file.f_mode |= FMODE_CAN_ODIRECT;
    }
    fp = kzalloc_obj(struct gfs2_file, GFP_NOFS);
    if (!fp)
    return -ENOMEM;
    mutex_init(&fp.f_fl_mutex);
    gfs2_assert_warn(GFS2_SB(inode), !file.private_data);
    file.private_data = fp;
    if (file.f_mode & FMODE_WRITE) {
    ret = gfs2_qa_get(GFS2_I(inode));
    if (ret)
    goto fail;
    }
    return 0;
    fail:
    kfree(file.private_data);
    file.private_data = core::ptr::null_mut();
    return ret;
    }
//
// gfs2_open - open a file
// @inode: the inode to open
// @file: the struct file for this opening
//
// After atomic_open, this function is only used for opening files
// which are already cached. We must still get the glock for regular
// files to ensure that we have the file size uptodate for the large
// file check which is in the common code. That is only an issue for
// regular files though.
//
// Returns: errno
//
#[no_mangle]
unsafe extern "C" fn gfs2_open(inode: *mut inode, file: *mut file) -> c_int {
    static int gfs2_open(struct inode *inode, struct file *file)
    {
    struct gfs2_inode *ip = GFS2_I(inode);
    struct gfs2_holder i_gh;
    int error;
    let mut need_unlock: bool = false;
    if (S_ISREG(ip.i_inode.i_mode)) {
    error = gfs2_glock_nq_init(ip.i_gl, LM_ST_SHARED, LM_FLAG_ANY,
    &i_gh);
    if (error)
    return error;
    need_unlock = true;
    }
    error = gfs2_open_common(inode, file);
    if (need_unlock)
    gfs2_glock_dq_uninit(&i_gh);
    return error;
    }
//
// gfs2_release - called to close a struct file
// @inode: the inode the struct file belongs to
// @file: the struct file being closed
//
// Returns: errno
//
#[no_mangle]
unsafe extern "C" fn gfs2_release(inode: *mut inode, file: *mut file) -> c_int {
    static int gfs2_release(struct inode *inode, struct file *file)
    {
    struct gfs2_inode *ip = GFS2_I(inode);
    kfree(file.private_data);
    file.private_data = core::ptr::null_mut();
    if (file.f_mode & FMODE_WRITE) {
    if (gfs2_rs_active(&ip.i_res))
    gfs2_rs_delete(ip);
    gfs2_qa_put(ip);
    }
    return 0;
    }
//
// gfs2_fsync - sync the dirty data for a file (across the cluster)
// @file: the file that points to the dentry
// @start: the start position in the file to sync
// @end: the end position in the file to sync
// @datasync: set if we can ignore timestamp changes
//
// We split the data flushing here so that we don't wait for the data
// until after we've also sent the metadata to disk. Note that for
// data=ordered, we will write & wait for the data at the log flush
// stage anyway, so this is unlikely to make much of a difference
// except in the data=writeback case.
//
// If the fdatawrite fails due to any reason except -EIO, we will
// continue the remainder of the fsync, although we'll still report
// the error at the end. This is to match filemap_write_and_wait_range()
// behaviour.
//
// Returns: errno
//
    static int gfs2_fsync(struct file *file, loff_t start, loff_t end,
    int datasync)
    {
    struct address_space *mapping = file.f_mapping;
    struct inode *inode = mapping.host;
    let mut sync_state: c_int = inode_state_read_once(inode) & I_DIRTY;
    struct gfs2_inode *ip = GFS2_I(inode);
    let mut ret: c_int = 0, ret1 = 0;
    if (mapping.nrpages) {
    ret1 = filemap_fdatawrite_range(mapping, start, end);
    if (ret1 == -EIO)
    return ret1;
    }
    if (!gfs2_is_jdata(ip))
    sync_state &= ~I_DIRTY_PAGES;
    if (datasync)
    sync_state &= ~I_DIRTY_SYNC;
    if (sync_state) {
    ret = sync_inode_metadata(inode, 1);
    if (ret)
    return ret;
    if (gfs2_is_jdata(ip))
    ret = file_write_and_wait(file);
    if (ret)
    return ret;
    gfs2_ail_flush(ip.i_gl, 1);
    }
    if (mapping.nrpages)
    ret = file_fdatawait_range(file, start, end);
    return ret ? ret : ret1;
    }
    static inline bool should_fault_in_pages(struct iov_iter *i,
    struct kiocb *iocb,
    size_t *prev_count,
    size_t *window_size)
    {
    let mut count: usize = iov_iter_count(i);
    size_t size, offs;
    if (!count)
    return false;
    if (!user_backed_iter(i))
    return false;
//
// Try to fault in multiple pages initially.  When that doesn't result
// in any progress, fall back to a single page.
//
    size = PAGE_SIZE;
    offs = offset_in_page(iocb.ki_pos);
    if (*prev_count != count) {
    size_t nr_dirtied;
    nr_dirtied = max(current.nr_dirtied_pause -
    current.nr_dirtied, 8);
    size = min_t(size_t, SZ_1M, nr_dirtied << PAGE_SHIFT);
    }
// prev_count = count;
// window_size = size - offs;
    return true;
    }
    static ssize_t gfs2_file_direct_read(struct kiocb *iocb, struct iov_iter *to,
    struct gfs2_holder *gh)
    {
    struct file *file = iocb.ki_filp;
    struct gfs2_inode *ip = GFS2_I(file.f_mapping.host);
    let mut prev_count: usize = 0, window_size = 0;
    let mut read: usize = 0;
    ssize_t ret;
//
// In this function, we disable page faults when we're holding the
// inode glock while doing I/O.  If a page fault occurs, we indicate
// that the inode glock should be dropped, fault in the pages manually,
// and retry.
//
// Unlike generic_file_read_iter, for reads, iomap_dio_rw can trigger
// physical as well as manual page faults, and we need to disable both
// kinds.
//
// For direct I/O, gfs2 takes the inode glock in deferred mode.  This
// locking mode is compatible with other deferred holders, so multiple
// processes and nodes can do direct I/O to a file at the same time.
// There's no guarantee that reads or writes will be atomic.  Any
// coordination among readers and writers needs to happen externally.
//
    if (!iov_iter_count(to))
    return 0; /* skip atime */
    gfs2_holder_init(ip.i_gl, LM_ST_DEFERRED, 0, gh);
    retry:
    ret = gfs2_glock_nq(gh);
    if (ret)
    goto out_uninit;
    pagefault_disable();
    to.nofault = true;
    ret = iomap_dio_rw(iocb, to, &gfs2_iomap_ops, core::ptr::null_mut(),
    IOMAP_DIO_PARTIAL, core::ptr::null_mut(), read);
    to.nofault = false;
    pagefault_enable();
    if (ret <= 0 && ret != -EFAULT)
    goto out_unlock;
// No increment (+=) because iomap_dio_rw returns a cumulative value.
    if (ret > 0)
    read = ret;
    if (should_fault_in_pages(to, iocb, &prev_count, &window_size)) {
    gfs2_glock_dq(gh);
    window_size -= fault_in_iov_iter_writeable(to, window_size);
    if (window_size)
    goto retry;
    }
    out_unlock:
    if (gfs2_holder_queued(gh))
    gfs2_glock_dq(gh);
    out_uninit:
    gfs2_holder_uninit(gh);
// User space doesn't expect partial success.
    if (ret < 0)
    return ret;
    return read;
    }
    static ssize_t gfs2_file_direct_write(struct kiocb *iocb, struct iov_iter *from,
    struct gfs2_holder *gh)
    {
    struct file *file = iocb.ki_filp;
    struct inode *inode = file.f_mapping.host;
    struct gfs2_inode *ip = GFS2_I(inode);
    let mut prev_count: usize = 0, window_size = 0;
    let mut written: usize = 0;
    bool enough_retries;
    ssize_t ret;
//
// In this function, we disable page faults when we're holding the
// inode glock while doing I/O.  If a page fault occurs, we indicate
// that the inode glock should be dropped, fault in the pages manually,
// and retry.
//
// For writes, iomap_dio_rw only triggers manual page faults, so we
// don't need to disable physical ones.
//
// Deferred lock, even if its a write, since we do no allocation on
// this path. All we need to change is the atime, and this lock mode
// ensures that other nodes have flushed their buffered read caches
// (i.e. their page cache entries for this inode). We do not,
// unfortunately, have the option of only flushing a range like the
// VFS does.
//
    gfs2_holder_init(ip.i_gl, LM_ST_DEFERRED, 0, gh);
    retry:
    ret = gfs2_glock_nq(gh);
    if (ret)
    goto out_uninit;
// Silently fall back to buffered I/O when writing beyond EOF
    if (iocb.ki_pos + iov_iter_count(from) > i_size_read(&ip.i_inode))
    goto out_unlock;
    from.nofault = true;
    ret = iomap_dio_rw(iocb, from, &gfs2_iomap_ops, core::ptr::null_mut(),
    IOMAP_DIO_PARTIAL, core::ptr::null_mut(), written);
    from.nofault = false;
    if (ret <= 0) {
    if (ret == -ENOTBLK)
    ret = 0;
    if (ret != -EFAULT)
    goto out_unlock;
    }
// No increment (+=) because iomap_dio_rw returns a cumulative value.
    if (ret > 0)
    written = ret;
    enough_retries = prev_count == iov_iter_count(from) &&
    window_size <= PAGE_SIZE;
    if (should_fault_in_pages(from, iocb, &prev_count, &window_size)) {
    gfs2_glock_dq(gh);
    window_size -= fault_in_iov_iter_readable(from, window_size);
    if (window_size) {
    if (!enough_retries)
    goto retry;
// fall back to buffered I/O
    ret = 0;
    }
    }
    out_unlock:
    if (gfs2_holder_queued(gh))
    gfs2_glock_dq(gh);
    out_uninit:
    gfs2_holder_uninit(gh);
// User space doesn't expect partial success.
    if (ret < 0)
    return ret;
    return written;
    }
#[no_mangle]
unsafe extern "C" fn gfs2_file_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> isize {
    static ssize_t gfs2_file_read_iter(struct kiocb *iocb, struct iov_iter *to)
    {
    struct gfs2_inode *ip;
    struct gfs2_holder gh;
    let mut prev_count: usize = 0, window_size = 0;
    let mut read: usize = 0;
    ssize_t ret;
//
// In this function, we disable page faults when we're holding the
// inode glock while doing I/O.  If a page fault occurs, we indicate
// that the inode glock should be dropped, fault in the pages manually,
// and retry.
//
    if (iocb.ki_flags & IOCB_DIRECT)
    return gfs2_file_direct_read(iocb, to, &gh);
    pagefault_disable();
    iocb.ki_flags |= IOCB_NOIO;
    ret = generic_file_read_iter(iocb, to);
    iocb.ki_flags &= ~IOCB_NOIO;
    pagefault_enable();
    if (ret >= 0) {
    if (!iov_iter_count(to))
    return ret;
    read = ret;
    } else if (ret != -EFAULT) {
    if (ret != -EAGAIN)
    return ret;
    if (iocb.ki_flags & IOCB_NOWAIT)
    return ret;
    }
    ip = GFS2_I(iocb.ki_filp.f_mapping.host);
    gfs2_holder_init(ip.i_gl, LM_ST_SHARED, 0, &gh);
    retry:
    ret = gfs2_glock_nq(&gh);
    if (ret)
    goto out_uninit;
    pagefault_disable();
    ret = generic_file_read_iter(iocb, to);
    pagefault_enable();
    if (ret <= 0 && ret != -EFAULT)
    goto out_unlock;
    if (ret > 0)
    read += ret;
    if (should_fault_in_pages(to, iocb, &prev_count, &window_size)) {
    gfs2_glock_dq(&gh);
    window_size -= fault_in_iov_iter_writeable(to, window_size);
    if (window_size)
    goto retry;
    }
    out_unlock:
    if (gfs2_holder_queued(&gh))
    gfs2_glock_dq(&gh);
    out_uninit:
    gfs2_holder_uninit(&gh);
    return read ? read : ret;
    }
    static ssize_t gfs2_file_buffered_write(struct kiocb *iocb,
    struct iov_iter *from,
    struct gfs2_holder *gh)
    {
    struct file *file = iocb.ki_filp;
    struct inode *inode = file_inode(file);
    struct gfs2_inode *ip = GFS2_I(inode);
    struct gfs2_sbd *sdp = GFS2_SB(inode);
    struct gfs2_holder *statfs_gh = core::ptr::null_mut();
    let mut prev_count: usize = 0, window_size = 0;
    let mut orig_count: usize = iov_iter_count(from);
    let mut written: usize = 0;
    ssize_t ret;
//
// In this function, we disable page faults when we're holding the
// inode glock while doing I/O.  If a page fault occurs, we indicate
// that the inode glock should be dropped, fault in the pages manually,
// and retry.
//
    if (inode == sdp.sd_rindex) {
    statfs_gh = kmalloc_obj(*statfs_gh, GFP_NOFS);
    if (!statfs_gh)
    return -ENOMEM;
    }
    gfs2_holder_init(ip.i_gl, LM_ST_EXCLUSIVE, 0, gh);
    if (should_fault_in_pages(from, iocb, &prev_count, &window_size)) {
    retry:
    window_size -= fault_in_iov_iter_readable(from, window_size);
    if (!window_size) {
    ret = -EFAULT;
    goto out_uninit;
    }
    from.count = min(from.count, window_size);
    }
    ret = gfs2_glock_nq(gh);
    if (ret)
    goto out_uninit;
    if (inode == sdp.sd_rindex) {
    struct gfs2_inode *m_ip = GFS2_I(sdp.sd_statfs_inode);
    ret = gfs2_glock_nq_init(m_ip.i_gl, LM_ST_EXCLUSIVE,
    GL_NOCACHE, statfs_gh);
    if (ret)
    goto out_unlock;
    }
    ret = gfs2_clear_beyond_eof(inode, iocb.ki_pos);
    if (ret)
    goto out_unlock;
    pagefault_disable();
    ret = iomap_file_buffered_write(iocb, from, &gfs2_iomap_ops,
    &gfs2_iomap_write_ops, core::ptr::null_mut());
    pagefault_enable();
    if (ret > 0)
    written += ret;
    if (inode == sdp.sd_rindex)
    gfs2_glock_dq_uninit(statfs_gh);
    if (ret <= 0 && ret != -EFAULT)
    goto out_unlock;
    from.count = orig_count - written;
    if (should_fault_in_pages(from, iocb, &prev_count, &window_size)) {
    gfs2_glock_dq(gh);
    goto retry;
    }
    out_unlock:
    if (gfs2_holder_queued(gh))
    gfs2_glock_dq(gh);
    out_uninit:
    gfs2_holder_uninit(gh);
    kfree(statfs_gh);
    from.count = orig_count - written;
    return written ? written : ret;
    }
//
// gfs2_file_write_iter - Perform a write to a file
// @iocb: The io context
// @from: The data to write
//
// We have to do a lock/unlock here to refresh the inode size for
// O_APPEND writes, otherwise we can land up writing at the wrong
// offset. There is still a race, but provided the app is using its
// own file locking, this will make O_APPEND work as expected.
//
#[no_mangle]
unsafe extern "C" fn gfs2_file_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize {
    static ssize_t gfs2_file_write_iter(struct kiocb *iocb, struct iov_iter *from)
    {
    struct file *file = iocb.ki_filp;
    struct inode *inode = file_inode(file);
    struct gfs2_inode *ip = GFS2_I(inode);
    struct gfs2_holder gh;
    ssize_t ret;
    gfs2_size_hint(file, iocb.ki_pos, iov_iter_count(from));
    if (iocb.ki_flags & IOCB_APPEND) {
    ret = gfs2_glock_nq_init(ip.i_gl, LM_ST_SHARED, 0, &gh);
    if (ret)
    return ret;
    gfs2_glock_dq_uninit(&gh);
    }
    inode_lock(inode);
    ret = generic_write_checks(iocb, from);
    if (ret <= 0)
    goto out_unlock;
    ret = file_remove_privs(file);
    if (ret)
    goto out_unlock;
    if (iocb.ki_flags & IOCB_DIRECT) {
    struct address_space *mapping = file.f_mapping;
    ssize_t buffered, ret2;
//
// Note that under direct I/O, we don't allow and inode
// timestamp updates, so we're not calling file_update_time()
// here.
//
    ret = gfs2_file_direct_write(iocb, from, &gh);
    if (ret < 0 || !iov_iter_count(from))
    goto out_unlock;
    iocb.ki_flags |= IOCB_DSYNC;
    buffered = gfs2_file_buffered_write(iocb, from, &gh);
    if (unlikely(buffered <= 0)) {
    if (!ret)
    ret = buffered;
    goto out_unlock;
    }
//
// We need to ensure that the page cache pages are written to
// disk and invalidated to preserve the expected O_DIRECT
// semantics.  If the writeback or invalidate fails, only report
// the direct I/O range as we don't know if the buffered pages
// made it to disk.
//
    ret2 = generic_write_sync(iocb, buffered);
    invalidate_mapping_pages(mapping,
    (iocb.ki_pos - buffered) >> PAGE_SHIFT,
    (iocb.ki_pos - 1) >> PAGE_SHIFT);
    if (!ret || ret2 > 0)
    ret += ret2;
    } else {
    ret = file_update_time(file);
    if (ret)
    goto out_unlock;
    ret = gfs2_file_buffered_write(iocb, from, &gh);
    if (likely(ret > 0))
    ret = generic_write_sync(iocb, ret);
    }
    out_unlock:
    inode_unlock(inode);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fallocate_chunk(inode: *mut inode, offset: loff_t, len: loff_t) -> c_int {
    static int fallocate_chunk(struct inode *inode, loff_t offset, loff_t len)
    {
    struct super_block *sb = inode.i_sb;
    struct gfs2_inode *ip = GFS2_I(inode);
    let mut end: loff_t = offset + len;
    struct buffer_head *dibh;
    int error;
    error = gfs2_meta_inode_buffer(ip, &dibh);
    if (unlikely(error))
    return error;
    gfs2_trans_add_meta(ip.i_gl, dibh);
    if (gfs2_is_stuffed(ip)) {
    error = gfs2_unstuff_dinode(ip);
    if (unlikely(error))
    goto out;
    }
    while (offset < end) {
    let mut iomap: iomap = { };
    error = gfs2_iomap_alloc(inode, offset, end - offset, &iomap);
    if (error)
    goto out;
    offset = iomap.offset + iomap.length;
    if (!(iomap.flags & IOMAP_F_NEW))
    continue;
    error = sb_issue_zeroout(sb, iomap.addr >> inode.i_blkbits,
    iomap.length >> inode.i_blkbits,
    GFP_NOFS);
    if (error) {
    fs_err(GFS2_SB(inode), "Failed to zero data buffers\n");
    goto out;
    }
    }
    out:
    brelse(dibh);
    return error;
    }
//
// calc_max_reserv() - Reverse of write_calc_reserv. Given a number of
// blocks, determine how many bytes can be written.
// @ip:          The inode in question.
// @len:         Max cap of bytes. What we return in *len must be <= this.
// @data_blocks: Compute and return the number of data blocks needed
// @ind_blocks:  Compute and return the number of indirect blocks needed
// @max_blocks:  The total blocks available to work with.
//
// Returns: void, but @len, @data_blocks and @ind_blocks are filled in.
//
    static void calc_max_reserv(struct gfs2_inode *ip, loff_t *len,
    unsigned int *data_blocks, unsigned int *ind_blocks,
    unsigned int max_blocks)
    {
    let mut max: loff_t = *len;
    const struct gfs2_sbd *sdp = GFS2_SB(&ip.i_inode);
    unsigned int tmp, max_data = max_blocks - 3 * (sdp.sd_max_height - 1);
    for (tmp = max_data; tmp > sdp.sd_diptrs;) {
    tmp = DIV_ROUND_UP(tmp, sdp.sd_inptrs);
    max_data -= tmp;
    }
// data_blocks = max_data;
// ind_blocks = max_blocks - max_data;
// len = ((loff_t)max_data - 3) << sdp->sd_sb.sb_bsize_shift;
    if (*len > max) {
// len = max;
    gfs2_write_calc_reserv(ip, max, data_blocks, ind_blocks);
    }
    }
#[no_mangle]
unsafe extern "C" fn __gfs2_fallocate(file: *mut file, mode: c_int, offset: loff_t, len: loff_t) -> c_long {
    static long __gfs2_fallocate(struct file *file, int mode, loff_t offset, loff_t len)
    {
    struct inode *inode = file_inode(file);
    struct gfs2_sbd *sdp = GFS2_SB(inode);
    struct gfs2_inode *ip = GFS2_I(inode);
    let mut ap: gfs2_alloc_parms = {};
    let mut data_blocks: c_uint = 0, ind_blocks = 0, rblocks;
    loff_t bytes, max_bytes, max_blks;
    int error;
    let mut pos: loff_t = offset;
    let mut count: loff_t = len;
    let mut bsize_mask: loff_t = ~((loff_t)sdp.sd_sb.sb_bsize - 1);
    let mut next: loff_t = (offset + len - 1) >> sdp.sd_sb.sb_bsize_shift;
    let mut max_chunk_size: loff_t = UINT_MAX & bsize_mask;
    next = (next + 1) << sdp.sd_sb.sb_bsize_shift;
    if (!(mode & FALLOC_FL_KEEP_SIZE)) {
    error = gfs2_clear_beyond_eof(inode, offset + len);
    if (error)
    return error;
    }
    offset &= bsize_mask;
    len = next - offset;
    bytes = sdp.sd_max_rg_data * sdp.sd_sb.sb_bsize / 2;
    if (!bytes)
    bytes = UINT_MAX;
    bytes &= bsize_mask;
    if (bytes == 0)
    bytes = sdp.sd_sb.sb_bsize;
    gfs2_size_hint(file, offset, len);
    gfs2_write_calc_reserv(ip, PAGE_SIZE, &data_blocks, &ind_blocks);
    ap.min_target = data_blocks + ind_blocks;
    while (len > 0) {
    if (len < bytes)
    bytes = len;
    if (!gfs2_write_alloc_required(ip, offset, bytes)) {
    len -= bytes;
    offset += bytes;
    continue;
    }
// We need to determine how many bytes we can actually
// fallocate without exceeding quota or going over the
// end of the fs. We start off optimistically by assuming
// we can write max_bytes
    max_bytes = (len > max_chunk_size) ? max_chunk_size : len;
// Since max_bytes is most likely a theoretical max, we
// calculate a more realistic 'bytes' to serve as a good
// starting point for the number of bytes we may be able
// to write
    gfs2_write_calc_reserv(ip, bytes, &data_blocks, &ind_blocks);
    ap.target = data_blocks + ind_blocks;
    error = gfs2_quota_lock_check(ip, &ap);
    if (error)
    return error;
// ap.allowed tells us how many blocks quota will allow
// us to write. Check if this reduces max_blks
    max_blks = UINT_MAX;
    if (ap.allowed)
    max_blks = ap.allowed;
    error = gfs2_inplace_reserve(ip, &ap);
    if (error)
    goto out_qunlock;
// check if the selected rgrp limits our max_blks further
    if (ip.i_res.rs_reserved < max_blks)
    max_blks = ip.i_res.rs_reserved;
// Almost done. Calculate bytes that can be written using
// max_blks. We also recompute max_bytes, data_blocks and
// ind_blocks
    calc_max_reserv(ip, &max_bytes, &data_blocks,
    &ind_blocks, max_blks);
    rblocks = RES_DINODE + ind_blocks + RES_STATFS + RES_QUOTA +
    RES_RG_HDR + gfs2_rg_blocks(ip, data_blocks + ind_blocks);
    if (gfs2_is_jdata(ip))
    rblocks += data_blocks ? data_blocks : 1;
    error = gfs2_trans_begin(sdp, rblocks,
    PAGE_SIZE >> inode.i_blkbits);
    if (error)
    goto out_trans_fail;
    error = fallocate_chunk(inode, offset, max_bytes);
    gfs2_trans_end(sdp);
    if (error)
    goto out_trans_fail;
    len -= max_bytes;
    offset += max_bytes;
    gfs2_inplace_release(ip);
    gfs2_quota_unlock(ip);
    }
    if (!(mode & FALLOC_FL_KEEP_SIZE) && (pos + count) > inode.i_size)
    i_size_write(inode, pos + count);
    file_update_time(file);
    mark_inode_dirty(inode);
    if ((file.f_flags & O_DSYNC) || IS_SYNC(file.f_mapping.host))
    return vfs_fsync_range(file, pos, pos + count - 1,
    (file.f_flags & __O_SYNC) ? 0 : 1);
    return 0;
    out_trans_fail:
    gfs2_inplace_release(ip);
    out_qunlock:
    gfs2_quota_unlock(ip);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn gfs2_fallocate(file: *mut file, mode: c_int, offset: loff_t, len: loff_t) -> c_long {
    static long gfs2_fallocate(struct file *file, int mode, loff_t offset, loff_t len)
    {
    struct inode *inode = file_inode(file);
    struct gfs2_sbd *sdp = GFS2_SB(inode);
    struct gfs2_inode *ip = GFS2_I(inode);
    struct gfs2_holder gh;
    int ret;
    if (mode & ~(FALLOC_FL_PUNCH_HOLE | FALLOC_FL_KEEP_SIZE))
    return -EOPNOTSUPP;
// fallocate is needed by gfs2_grow to reserve space in the rindex
    if (gfs2_is_jdata(ip) && inode != sdp.sd_rindex)
    return -EOPNOTSUPP;
    inode_lock(inode);
    gfs2_holder_init(ip.i_gl, LM_ST_EXCLUSIVE, 0, &gh);
    ret = gfs2_glock_nq(&gh);
    if (ret)
    goto out_uninit;
    if (!(mode & FALLOC_FL_KEEP_SIZE) &&
    (offset + len) > inode.i_size) {
    ret = inode_newsize_ok(inode, offset + len);
    if (ret)
    goto out_unlock;
    }
    ret = get_write_access(inode);
    if (ret)
    goto out_unlock;
    if (mode & FALLOC_FL_PUNCH_HOLE) {
    ret = __gfs2_punch_hole(file, offset, len);
    } else {
    ret = __gfs2_fallocate(file, mode, offset, len);
    if (ret)
    gfs2_rs_deltree(&ip.i_res);
    }
    put_write_access(inode);
    out_unlock:
    gfs2_glock_dq(&gh);
    out_uninit:
    gfs2_holder_uninit(&gh);
    inode_unlock(inode);
    return ret;
    }
    static ssize_t gfs2_file_splice_write(struct pipe_inode_info *pipe,
    struct file *out, loff_t *ppos,
    size_t len, unsigned int flags)
    {
    ssize_t ret;
    gfs2_size_hint(out, *ppos, len);
    ret = iter_file_splice_write(pipe, out, ppos, len, flags);
    return ret;
    }

//
// gfs2_lock - acquire/release a posix lock on a file
// @file: the file pointer
// @cmd: either modify or retrieve lock state, possibly wait
// @fl: type and range of lock
//
// Returns: errno
//
#[no_mangle]
unsafe extern "C" fn gfs2_lock(file: *mut file, cmd: c_int, fl: *mut file_lock) -> c_int {
    static int gfs2_lock(struct file *file, int cmd, struct file_lock *fl)
    {
    struct gfs2_inode *ip = GFS2_I(file.f_mapping.host);
    struct gfs2_sbd *sdp = GFS2_SB(file.f_mapping.host);
    struct lm_lockstruct *ls = &sdp.sd_lockstruct;
    int ret;
    if (!(fl.c.flc_flags & FL_POSIX))
    return -ENOLCK;
    if (gfs2_withdrawn(sdp)) {
    if (lock_is_unlock(fl))
    locks_lock_file_wait(file, fl);
    return -EIO;
    }
    down_read(&ls.ls_sem);
    ret = -ENODEV;
    if (likely(ls.ls_dlm != core::ptr::null_mut())) {
    if (cmd == F_CANCELLK)
    ret = dlm_posix_cancel(ls.ls_dlm, ip.i_no_addr, file, fl);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: IS_GETLK(cmd)) -> else {
    else if (IS_GETLK(cmd))
    ret = dlm_posix_get(ls.ls_dlm, ip.i_no_addr, file, fl);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: lock_is_unlock(fl)) -> else {
    else if (lock_is_unlock(fl))
    ret = dlm_posix_unlock(ls.ls_dlm, ip.i_no_addr, file, fl);
    else
    ret = dlm_posix_lock(ls.ls_dlm, ip.i_no_addr, file, cmd, fl);
    }
    up_read(&ls.ls_sem);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __flock_holder_uninit(file: *mut file, fl_gh: *mut gfs2_holder) {
    static void __flock_holder_uninit(struct file *file, struct gfs2_holder *fl_gh)
    {
    struct gfs2_glock *gl = gfs2_glock_hold(fl_gh.gh_gl);
//
// Make sure gfs2_glock_put() won't sleep under the file->f_lock
// spinlock.
//
    spin_lock(&file.f_lock);
    gfs2_holder_uninit(fl_gh);
    spin_unlock(&file.f_lock);
    gfs2_glock_put(gl);
    }
#[no_mangle]
unsafe extern "C" fn do_flock(file: *mut file, cmd: c_int, fl: *mut file_lock) -> c_int {
    static int do_flock(struct file *file, int cmd, struct file_lock *fl)
    {
    struct gfs2_file *fp = file.private_data;
    struct gfs2_holder *fl_gh = &fp.f_fl_gh;
    struct gfs2_inode *ip = GFS2_I(file_inode(file));
    struct gfs2_glock *gl;
    unsigned int state;
    u16 flags;
    let mut error: c_int = 0;
    int sleeptime;
    state = lock_is_write(fl) ? LM_ST_EXCLUSIVE : LM_ST_SHARED;
    flags = GL_EXACT | GL_NOPID;
    if (!IS_SETLKW(cmd))
    flags |= LM_FLAG_TRY_1CB;
    mutex_lock(&fp.f_fl_mutex);
    if (gfs2_holder_initialized(fl_gh)) {
    struct file_lock request;
    if (fl_gh.gh_state == state)
    goto out;
    locks_init_lock(&request);
    request.c.flc_type = F_UNLCK;
    request.c.flc_flags = FL_FLOCK;
    locks_lock_file_wait(file, &request);
    gfs2_glock_dq(fl_gh);
    gfs2_holder_reinit(state, flags, fl_gh);
    } else {
    error = gfs2_glock_get(GFS2_SB(&ip.i_inode), ip.i_no_addr,
    &gfs2_flock_glops, CREATE, &gl);
    if (error)
    goto out;
    spin_lock(&file.f_lock);
    gfs2_holder_init(gl, state, flags, fl_gh);
    spin_unlock(&file.f_lock);
    gfs2_glock_put(gl);
    }
    for (sleeptime = 1; sleeptime <= 4; sleeptime <<= 1) {
    error = gfs2_glock_nq(fl_gh);
    if (error != GLR_TRYFAILED)
    break;
    fl_gh.gh_flags &= ~LM_FLAG_TRY_1CB;
    fl_gh.gh_flags |= LM_FLAG_TRY;
    msleep(sleeptime);
    }
    if (error) {
    __flock_holder_uninit(file, fl_gh);
    if (error == GLR_TRYFAILED)
    error = -EAGAIN;
    } else {
    error = locks_lock_file_wait(file, fl);
    gfs2_assert_warn(GFS2_SB(&ip.i_inode), !error);
    }
    out:
    mutex_unlock(&fp.f_fl_mutex);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn do_unflock(file: *mut file, fl: *mut file_lock) {
    static void do_unflock(struct file *file, struct file_lock *fl)
    {
    struct gfs2_file *fp = file.private_data;
    struct gfs2_holder *fl_gh = &fp.f_fl_gh;
    mutex_lock(&fp.f_fl_mutex);
    locks_lock_file_wait(file, fl);
    if (gfs2_holder_initialized(fl_gh)) {
    gfs2_glock_dq(fl_gh);
    __flock_holder_uninit(file, fl_gh);
    }
    mutex_unlock(&fp.f_fl_mutex);
    }
//
// gfs2_flock - acquire/release a flock lock on a file
// @file: the file pointer
// @cmd: either modify or retrieve lock state, possibly wait
// @fl: type and range of lock
//
// Returns: errno
//
#[no_mangle]
unsafe extern "C" fn gfs2_flock(file: *mut file, cmd: c_int, fl: *mut file_lock) -> c_int {
    static int gfs2_flock(struct file *file, int cmd, struct file_lock *fl)
    {
    if (!(fl.c.flc_flags & FL_FLOCK))
    return -ENOLCK;
    if (lock_is_unlock(fl)) {
    do_unflock(file, fl);
    return 0;
    } else {
    return do_flock(file, cmd, fl);
    }
    }
    const struct file_operations gfs2_file_fops = {
    .llseek		= gfs2_llseek,
    .read_iter	= gfs2_file_read_iter,
    .write_iter	= gfs2_file_write_iter,
    .iopoll		= iocb_bio_iopoll,
    .unlocked_ioctl	= gfs2_ioctl,
    .compat_ioctl	= gfs2_compat_ioctl,
    .mmap		= gfs2_mmap,
    .open		= gfs2_open,
    .release	= gfs2_release,
    .fsync		= gfs2_fsync,
    .lock		= gfs2_lock,
    .flock		= gfs2_flock,
    .splice_read	= copy_splice_read,
    .splice_write	= gfs2_file_splice_write,
    .fallocate	= gfs2_fallocate,
    .fop_flags	= FOP_ASYNC_LOCK,
    };
    const struct file_operations gfs2_dir_fops = {
    .iterate_shared	= gfs2_readdir,
    .unlocked_ioctl	= gfs2_ioctl,
    .compat_ioctl	= gfs2_compat_ioctl,
    .open		= gfs2_open,
    .release	= gfs2_release,
    .fsync		= gfs2_fsync,
    .lock		= gfs2_lock,
    .flock		= gfs2_flock,
    .llseek		= default_llseek,
    .fop_flags	= FOP_ASYNC_LOCK,
    };

    const struct file_operations gfs2_file_fops_nolock = {
    .llseek		= gfs2_llseek,
    .read_iter	= gfs2_file_read_iter,
    .write_iter	= gfs2_file_write_iter,
    .iopoll		= iocb_bio_iopoll,
    .unlocked_ioctl	= gfs2_ioctl,
    .compat_ioctl	= gfs2_compat_ioctl,
    .mmap		= gfs2_mmap,
    .open		= gfs2_open,
    .release	= gfs2_release,
    .fsync		= gfs2_fsync,
    .splice_read	= copy_splice_read,
    .splice_write	= gfs2_file_splice_write,
    .setlease	= generic_setlease,
    .fallocate	= gfs2_fallocate,
    };
    const struct file_operations gfs2_dir_fops_nolock = {
    .iterate_shared	= gfs2_readdir,
    .unlocked_ioctl	= gfs2_ioctl,
    .compat_ioctl	= gfs2_compat_ioctl,
    .open		= gfs2_open,
    .release	= gfs2_release,
    .fsync		= gfs2_fsync,
    .llseek		= default_llseek,
    .setlease	= generic_setlease,
    };
