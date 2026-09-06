//! Automatically rewritten from C to Rust
//! Source: fs/erofs/dir.c
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
// Copyright (C) 2017-2018 HUAWEI, Inc.
// https://www.huawei.com
// Copyright (C) 2022, Alibaba Cloud
//

    static int erofs_fill_dentries(struct inode *dir, struct dir_context *ctx,
    void *dentry_blk, struct erofs_dirent *de,
    unsigned int nameoff0, unsigned int maxsize)
    {
    const struct erofs_dirent *end = dentry_blk + nameoff0;
    while (de < end) {
    let mut d_type: c_uchar = fs_ftype_to_dtype(de.file_type);
    let mut nameoff: c_uint = le16_to_cpu(de.nameoff);
    const char *de_name = (char *)dentry_blk + nameoff;
    unsigned int de_namelen;
// non-trailing dirent in the directory block?
    if (de + 1 < end)
    de_namelen = le16_to_cpu(de[1].nameoff) - nameoff;
#[no_mangle]
pub unsafe extern "C" fn if(nameoff: maxsize <=) -> else {
    else if (maxsize <= nameoff)
    goto err_bogus;
    else
    de_namelen = strnlen(de_name, maxsize - nameoff);
// a corrupted entry is found (including negative namelen)
    if (!in_range32(de_namelen, 1, EROFS_NAME_LEN) ||
    nameoff + de_namelen > maxsize)
    goto err_bogus;
    if (!dir_emit(ctx, de_name, de_namelen,
    erofs_nid_to_ino64(EROFS_SB(dir.i_sb),
    le64_to_cpu(de.nid)), d_type))
    return 1;
    ++de;
    ctx.pos += sizeof(struct erofs_dirent);
    }
    return 0;
    err_bogus:
    erofs_err(dir.i_sb, "bogus dirent @ nid %llu", EROFS_I(dir).nid);
    DBG_BUGON(1);
    return -EFSCORRUPTED;
    }
#[no_mangle]
unsafe extern "C" fn erofs_readdir(f: *mut file, ctx: *mut dir_context) -> c_int {
    static int erofs_readdir(struct file *f, struct dir_context *ctx)
    {
    struct inode *dir = file_inode(f);
    let mut buf: erofs_buf = __EROFS_BUF_INITIALIZER;
    struct super_block *sb = dir.i_sb;
    struct file_ra_state *ra = &f.f_ra;
    let mut bsz: c_ulong = sb.s_blocksize;
    let mut ofs: c_uint = erofs_blkoff(sb, ctx.pos);
    pgoff_t ra_pages = DIV_ROUND_UP_POW2(
    EROFS_I_SB(dir).dir_ra_bytes, PAGE_SIZE);
    let mut nr_pages: pgoff_t = DIV_ROUND_UP_POW2(dir.i_size, PAGE_SIZE);
    let mut err: c_int = 0;
    let mut initial: bool = true;
    buf.mapping = dir.i_mapping;
    while (ctx.pos < dir.i_size) {
    let mut dbstart: erofs_off_t = ctx.pos - ofs;
    struct erofs_dirent *de;
    unsigned int nameoff, maxsize;
    if (fatal_signal_pending(current)) {
    err = -ERESTARTSYS;
    break;
    }
// readahead blocks to enhance performance for large directories
    if (ra_pages) {
    let mut idx: pgoff_t = DIV_ROUND_UP_POW2(ctx.pos, PAGE_SIZE);
    let mut pages: pgoff_t = min(nr_pages - idx, ra_pages);
    if (pages > 1 && !ra_has_index(ra, idx))
    page_cache_sync_readahead(dir.i_mapping, ra,
    f, idx, pages);
    }
    de = erofs_bread(&buf, dbstart, true);
    if (IS_ERR(de)) {
    erofs_err(sb, "failed to readdir of logical block %llu of nid %llu",
    erofs_blknr(sb, dbstart), EROFS_I(dir).nid);
    err = PTR_ERR(de);
    break;
    }
    nameoff = le16_to_cpu(de.nameoff);
    if (!nameoff || nameoff >= bsz || (nameoff % sizeof(*de))) {
    erofs_err(sb, "invalid de[0].nameoff %u @ nid %llu",
    nameoff, EROFS_I(dir).nid);
    err = -EFSCORRUPTED;
    break;
    }
    maxsize = min_t(unsigned int, dir.i_size - dbstart, bsz);
// search dirents at the arbitrary position
    if (initial) {
    initial = false;
    ofs = roundup(ofs, sizeof(struct erofs_dirent));
    ctx.pos = dbstart + ofs;
    }
    err = erofs_fill_dentries(dir, ctx, de, (void *)de + ofs,
    nameoff, maxsize);
    if (err)
    break;
    ctx.pos = dbstart + maxsize;
    ofs = 0;
    cond_resched();
    }
    erofs_put_metabuf(&buf);
    if (EROFS_I(dir).dot_omitted && ctx.pos == dir.i_size) {
    if (!dir_emit_dot(f, ctx))
    return 0;
    ++ctx.pos;
    }
    return err < 0 ? err : 0;
    }
    const struct file_operations erofs_dir_fops = {
    .llseek		= generic_file_llseek,
    .read		= generic_read_dir,
    .iterate_shared	= erofs_readdir,
    .unlocked_ioctl = erofs_ioctl,

    .compat_ioctl   = erofs_compat_ioctl,

    .setlease	= generic_setlease,
    };
