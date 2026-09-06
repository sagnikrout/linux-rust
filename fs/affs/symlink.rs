//! Automatically rewritten from C to Rust
//! Source: fs/affs/symlink.c
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
// linux/fs/affs/symlink.c
//
// 1995  Hans-Joachim Widmaier - Modified for affs.
//
// Copyright (C) 1991, 1992  Linus Torvalds
//
// affs symlink handling code
//

#[no_mangle]
unsafe extern "C" fn affs_symlink_read_folio(file: *mut file, folio: *mut folio) -> c_int {
    static int affs_symlink_read_folio(struct file *file, struct folio *folio)
    {
    struct buffer_head *bh;
    struct inode *inode = folio.mapping.host;
    char *link = folio_address(folio);
    struct slink_front *lf;
    int			 i, j;
    char			 c;
    char			 lc;
    pr_debug("get_link(ino=%llu)\n", inode.i_ino);
    bh = affs_bread(inode.i_sb, inode.i_ino);
    if (!bh)
    goto fail;
    i  = 0;
    j  = 0;
    lf = (struct slink_front *)bh.b_data;
    lc = 0;
    if (strchr(lf.symname,':')) {	/* Handle assign or volume name */
    struct affs_sb_info *sbi = AFFS_SB(inode.i_sb);
    char *pf;
    spin_lock(&sbi.symlink_lock);
    pf = sbi.s_prefix ? sbi.s_prefix : "/";
    while (i < 1023 && (c = pf[i]))
    link[i++] = c;
    spin_unlock(&sbi.symlink_lock);
    while (i < 1023 && lf.symname[j] != ':')
    link[i++] = lf.symname[j++];
    if (i < 1023)
    link[i++] = '/';
    j++;
    lc = '/';
    }
    while (i < 1023 && (c = lf.symname[j])) {
    if (c == '/' && lc == '/' && i < 1020) {	/* parent dir */
    link[i++] = '.';
    link[i++] = '.';
    }
    link[i++] = c;
    lc = c;
    j++;
    }
    link[i] = '\0';
    affs_brelse(bh);
    folio_mark_uptodate(folio);
    folio_unlock(folio);
    return 0;
    fail:
    folio_unlock(folio);
    return -EIO;
    }
    const struct address_space_operations affs_symlink_aops = {
    .read_folio	= affs_symlink_read_folio,
    };
    const struct inode_operations affs_symlink_inode_operations = {
    .get_link	= page_get_link,
    .setattr	= affs_setattr,
    };
