//! Automatically rewritten from C to Rust
//! Source: fs/udf/symlink.c
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
// symlink.c
//
// PURPOSE
// Symlink handling routines for the OSTA-UDF(tm) filesystem.
//
// COPYRIGHT
// (C) 1998-2001 Ben Fennema
// (C) 1999 Stelias Computing Inc
//
// HISTORY
//
// 04/16/99 blf  Created.
//

    static int udf_pc_to_char(struct super_block *sb, unsigned char *from,
    int fromlen, unsigned char *to, int tolen)
    {
    struct pathComponent *pc;
    let mut elen: c_int = 0;
    int comp_len;
    unsigned char *p = to;
// Reserve one byte for terminating \0
    tolen--;
    while (elen < fromlen) {
    if (fromlen - elen < sizeof(struct pathComponent))
    return -EIO;
    pc = (struct pathComponent *)(from + elen);
    elen += sizeof(struct pathComponent);
    switch (pc.componentType) {
    case 1:
//
// Symlink points to some place which should be agreed
// upon between originator and receiver of the media. Ignore.
//
    if (pc.lengthComponentIdent > 0) {
    elen += pc.lengthComponentIdent;
    break;
    }
    fallthrough;
    case 2:
    if (tolen == 0)
    return -ENAMETOOLONG;
    p = to;
// p++ = '/';
    tolen--;
    break;
    case 3:
    if (tolen < 3)
    return -ENAMETOOLONG;
    memcpy(p, "../", 3);
    p += 3;
    tolen -= 3;
    break;
    case 4:
    if (tolen < 2)
    return -ENAMETOOLONG;
    memcpy(p, "./", 2);
    p += 2;
    tolen -= 2;
// that would be . - just ignore
    break;
    case 5:
    elen += pc.lengthComponentIdent;
    if (elen > fromlen)
    return -EIO;
    comp_len = udf_get_filename(sb, pc.componentIdent,
    pc.lengthComponentIdent,
    p, tolen);
    if (comp_len < 0)
    return comp_len;
    p += comp_len;
    tolen -= comp_len;
    if (tolen == 0)
    return -ENAMETOOLONG;
// p++ = '/';
    tolen--;
    break;
    }
    }
    if (p > to + 1)
    p[-1] = '\0';
    else
    p[0] = '\0';
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn udf_symlink_filler(file: *mut file, folio: *mut folio) -> c_int {
    static int udf_symlink_filler(struct file *file, struct folio *folio)
    {
    struct inode *inode = folio.mapping.host;
    struct buffer_head *bh = core::ptr::null_mut();
    unsigned char *symlink;
    let mut err: c_int = 0;
    unsigned char *p = folio_address(folio);
    struct udf_inode_info *iinfo = UDF_I(inode);
// We don't support symlinks longer than one block
    if (inode.i_size > inode.i_sb.s_blocksize) {
    err = -ENAMETOOLONG;
    goto out;
    }
    if (iinfo.i_alloc_type == ICBTAG_FLAG_AD_IN_ICB) {
    symlink = iinfo.i_data + iinfo.i_lenEAttr;
    } else {
    bh = udf_bread(inode, 0, 0, &err);
    if (!bh) {
    if (!err)
    err = -EFSCORRUPTED;
    goto out;
    }
    symlink = bh.b_data;
    }
    err = udf_pc_to_char(inode.i_sb, symlink, inode.i_size, p, PAGE_SIZE);
    brelse(bh);
    out:
    folio_end_read(folio, err == 0);
    return err;
    }
    static int udf_symlink_getattr(struct mnt_idmap *idmap,
    const struct path *path, struct kstat *stat,
    u32 request_mask, unsigned int flags)
    {
    struct dentry *dentry = path.dentry;
    struct inode *inode = d_backing_inode(dentry);
    struct folio *folio;
    generic_fillattr(&nop_mnt_idmap, request_mask, inode, stat);
    folio = read_mapping_folio(inode.i_mapping, 0, core::ptr::null_mut());
    if (IS_ERR(folio))
    return PTR_ERR(folio);
//
// UDF uses non-trivial encoding of symlinks so i_size does not match
// number of characters reported by readlink(2) which apparently some
// applications expect. Also POSIX says that "The value returned in the
// st_size field shall be the length of the contents of the symbolic
// link, and shall not count a trailing null if one is present." So
// let's report the length of string returned by readlink(2) for
// st_size.
//
    stat.size = strlen(folio_address(folio));
    folio_put(folio);
    return 0;
    }
//
// symlinks can't do much...
//
    const struct address_space_operations udf_symlink_aops = {
    .read_folio		= udf_symlink_filler,
    };
    const struct inode_operations udf_symlink_inode_operations = {
    .get_link	= page_get_link,
    .getattr	= udf_symlink_getattr,
    };
