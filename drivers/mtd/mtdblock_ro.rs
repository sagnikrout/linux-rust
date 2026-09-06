//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/mtdblock_ro.c
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
// Simple read-only (writable only for RAM) mtdblock driver
//
// Copyright © 2001-2010 David Woodhouse <dwmw2@infradead.org>
//

    static int mtdblock_readsect(struct mtd_blktrans_dev *dev,
    unsigned long block, char *buf)
    {
    size_t retlen;
    int err;
    err = mtd_read(dev.mtd, (block * 512), 512, &retlen, buf);
    if (err && !mtd_is_bitflip(err))
    return 1;
    return 0;
    }
    static int mtdblock_writesect(struct mtd_blktrans_dev *dev,
    unsigned long block, char *buf)
    {
    size_t retlen;
    if (mtd_write(dev.mtd, (block * 512), 512, &retlen, buf))
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtdblock_add_mtd(tr: *mut mtd_blktrans_ops, mtd: *mut mtd_info) {
    static void mtdblock_add_mtd(struct mtd_blktrans_ops *tr, struct mtd_info *mtd)
    {
    struct mtd_blktrans_dev *dev = kzalloc_obj(*dev);
    if (!dev)
    return;
    dev.mtd = mtd;
    dev.devnum = mtd.index;
    dev.size = mtd.size >> 9;
    dev.tr = tr;
    dev.readonly = 1;
    if (mtd_type_is_nand(mtd))
    pr_warn_ratelimited("%s: MTD device '%s' is NAND, please consider using UBI block devices instead.\n",
    tr.name, mtd.name);
    if (add_mtd_blktrans_dev(dev))
    kfree(dev);
    }
#[no_mangle]
unsafe extern "C" fn mtdblock_remove_dev(dev: *mut mtd_blktrans_dev) {
    static void mtdblock_remove_dev(struct mtd_blktrans_dev *dev)
    {
    del_mtd_blktrans_dev(dev);
    }
    static struct mtd_blktrans_ops mtdblock_tr = {
    .name		= "mtdblock",
    .major		= MTD_BLOCK_MAJOR,
    .part_bits	= 0,
    .blksize 	= 512,
    .readsect	= mtdblock_readsect,
    .writesect	= mtdblock_writesect,
    .add_mtd	= mtdblock_add_mtd,
    .remove_dev	= mtdblock_remove_dev,
    .owner		= THIS_MODULE,
    };
    module_mtd_blktrans(mtdblock_tr);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("David Woodhouse <dwmw2@infradead.org>");
    MODULE_DESCRIPTION("Simple read-only block device emulation access to MTD devices");
