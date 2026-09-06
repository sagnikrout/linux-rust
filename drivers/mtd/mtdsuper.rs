//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/mtdsuper.c
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
// MTD-based superblock management
//
// Copyright © 2001-2007 Red Hat, Inc. All Rights Reserved.
// Copyright © 2001-2010 David Woodhouse <dwmw2@infradead.org>
//
// Written by:  David Howells <dhowells@redhat.com>
// David Woodhouse <dwmw2@infradead.org>
//

//
// get a superblock on an MTD-backed filesystem
//
    static int mtd_get_sb(struct fs_context *fc,
    struct mtd_info *mtd,
    int (*fill_super)(struct super_block *,
    struct fs_context *))
    {
    struct super_block *sb;
    int ret;
    sb = sget_dev(fc, MKDEV(MTD_BLOCK_MAJOR, mtd.index));
    if (IS_ERR(sb))
    return PTR_ERR(sb);
    if (sb.s_root) {
// new mountpoint for an already mounted superblock
    pr_debug("MTDSB: Device %d (\"%s\") is already mounted\n",
    mtd.index, mtd.name);
    put_mtd_device(mtd);
    } else {
// fresh new superblock
    pr_debug("MTDSB: New superblock for device %d (\"%s\")\n",
    mtd.index, mtd.name);
//
// Would usually have been set with @sb_lock held but in
// contrast to sb->s_bdev that's checked with only
// @sb_lock held, nothing checks sb->s_mtd without also
// holding sb->s_umount and we're holding sb->s_umount
// here.
//
    sb.s_mtd = mtd;
    sb.s_bdi = bdi_get(mtd_bdi);
    ret = fill_super(sb, fc);
    if (ret < 0)
    goto error_sb;
    sb.s_flags |= SB_ACTIVE;
    }
    BUG_ON(fc.root);
    fc.root = dget(sb.s_root);
    return 0;
    error_sb:
    deactivate_locked_super(sb);
    return ret;
    }
//
// get a superblock on an MTD-backed filesystem by MTD device number
//
    static int mtd_get_sb_by_nr(struct fs_context *fc, int mtdnr,
    int (*fill_super)(struct super_block *,
    struct fs_context *))
    {
    struct mtd_info *mtd;
    mtd = get_mtd_device(core::ptr::null_mut(), mtdnr);
    if (IS_ERR(mtd)) {
    errorf(fc, "MTDSB: Device #%u doesn't appear to exist\n", mtdnr);
    return PTR_ERR(mtd);
    }
    return mtd_get_sb(fc, mtd, fill_super);
    }
//
// get_tree_mtd - Get a superblock based on a single MTD device
// @fc: The filesystem context holding the parameters
// @fill_super: Helper to initialise a new superblock
//
    int get_tree_mtd(struct fs_context *fc,
    int (*fill_super)(struct super_block *sb,
    struct fs_context *fc))
    {

    dev_t dev;
    int ret;

    int mtdnr;
    if (!fc.source)
    return invalf(fc, "No source specified");
    pr_debug("MTDSB: dev_name \"%s\"\n", fc.source);
// the preferred way of mounting in future; especially when
// CONFIG_BLOCK=n - we specify the underlying MTD device by number or
// by name, so that we don't require block device support to be present
// in the kernel.
//
    if (fc.source[0] == 'm' &&
    fc.source[1] == 't' &&
    fc.source[2] == 'd') {
    if (fc.source[3] == ':') {
    struct mtd_info *mtd;
// mount by MTD device name
    pr_debug("MTDSB: mtd:%%s, name \"%s\"\n",
    fc.source + 4);
    mtd = get_mtd_device_nm(fc.source + 4);
    if (!IS_ERR(mtd))
    return mtd_get_sb(fc, mtd, fill_super);
    errorf(fc, "MTD: MTD device with name \"%s\" not found",
    fc.source + 4);
    } else if (isdigit(fc.source[3])) {
// mount by MTD device number name
    unsigned int mtdnr_val;
    if (kstrtouint(fc.source + 3, 0, &mtdnr_val) == 0) {
    mtdnr = mtdnr_val;
// It was a valid number
    pr_debug("MTDSB: mtd%%d, mtdnr %u\n", mtdnr_val);
    return mtd_get_sb_by_nr(fc, mtdnr, fill_super);
    }
    }
    }

// try the old way - the hack where we allowed users to mount
// /dev/mtdblock$(n) but didn't actually _use_ the blockdev
//
    ret = lookup_bdev(fc.source, &dev);
    if (ret) {
    errorf(fc, "MTD: Couldn't look up '%s': %d", fc.source, ret);
    return ret;
    }
    pr_debug("MTDSB: lookup_bdev() returned 0\n");
    if (MAJOR(dev) == MTD_BLOCK_MAJOR)
    return mtd_get_sb_by_nr(fc, MINOR(dev), fill_super);

    if (!(fc.sb_flags & SB_SILENT))
    errorf(fc, "MTD: Attempt to mount non-MTD device \"%s\"",
    fc.source);
    return -EINVAL;
    }
    EXPORT_SYMBOL_GPL(get_tree_mtd);
//
// destroy an MTD-based superblock
//
#[no_mangle]
pub unsafe extern "C" fn kill_mtd_super(sb: *mut super_block) {
    void kill_mtd_super(struct super_block *sb)
    {
    generic_shutdown_super(sb);
    put_mtd_device(sb.s_mtd);
    sb.s_mtd = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(kill_mtd_super);
