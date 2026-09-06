//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/tests/readtest.c
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
// Copyright (C) 2006-2008 Nokia Corporation
//
// Check MTD device read.
//
// Author: Adrian Hunter <ext-adrian.hunter@nokia.com>
//

    let mut dev: static int = -EINVAL;
    module_param(dev, int, S_IRUGO);
    MODULE_PARM_DESC(dev, "MTD device number to use");
    static struct mtd_info *mtd;
    static unsigned char *iobuf;
    static unsigned char *iobuf1;
    static unsigned char *bbt;
    static int pgsize;
    static int ebcnt;
    static int pgcnt;
#[no_mangle]
unsafe extern "C" fn read_eraseblock_by_page(ebnum: c_int) -> c_int {
    static int read_eraseblock_by_page(int ebnum)
    {
    int i, ret, err = 0;
    let mut addr: loff_t = (loff_t)ebnum * mtd.erasesize;
    void *buf = iobuf;
    void *oobbuf = iobuf1;
    for (i = 0; i < pgcnt; i++) {
    memset(buf, 0 , pgsize);
    ret = mtdtest_read(mtd, addr, pgsize, buf);
    if (ret) {
    if (!err)
    err = ret;
    }
    if (mtd.oobsize) {
    let mut ops: mtd_oob_ops = { };
    ops.mode      = MTD_OPS_PLACE_OOB;
    ops.len       = 0;
    ops.retlen    = 0;
    ops.ooblen    = mtd.oobsize;
    ops.oobretlen = 0;
    ops.ooboffs   = 0;
    ops.datbuf    = core::ptr::null_mut();
    ops.oobbuf    = oobbuf;
    ret = mtd_read_oob(mtd, addr, &ops);
    if ((ret && !mtd_is_bitflip(ret)) ||
    ops.oobretlen != mtd.oobsize) {
    pr_err("error: read oob failed at "
    "%#llx\n", (long long)addr);
    if (!err)
    err = ret;
    if (!err)
    err = -EINVAL;
    }
    oobbuf += mtd.oobsize;
    }
    addr += pgsize;
    buf += pgsize;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn dump_eraseblock(ebnum: c_int) {
    static void dump_eraseblock(int ebnum)
    {
    int i, j, n;
    char line[128];
    int pg, oob;
    pr_info("dumping eraseblock %d\n", ebnum);
    n = mtd.erasesize;
    for (i = 0; i < n;) {
    char *p = line;
    p += sprintf(p, "%05x: ", i);
    for (j = 0; j < 32 && i < n; j++, i++)
    p += sprintf(p, "%02x", (unsigned int)iobuf[i]);
    printk(KERN_CRIT "%s\n", line);
    cond_resched();
    }
    if (!mtd.oobsize)
    return;
    pr_info("dumping oob from eraseblock %d\n", ebnum);
    n = mtd.oobsize;
    for (pg = 0, i = 0; pg < pgcnt; pg++)
    for (oob = 0; oob < n;) {
    char *p = line;
    p += sprintf(p, "%05x: ", i);
    for (j = 0; j < 32 && oob < n; j++, oob++, i++)
    p += sprintf(p, "%02x",
    (unsigned int)iobuf1[i]);
    printk(KERN_CRIT "%s\n", line);
    cond_resched();
    }
    }
#[no_mangle]
unsafe extern "C" fn mtd_readtest_init() -> int __init {
    static int __init mtd_readtest_init(void)
    {
    uint64_t tmp;
    int err, i;
    printk(KERN_INFO "\n");
    printk(KERN_INFO "=================================================\n");
    if (dev < 0) {
    pr_info("Please specify a valid mtd-device via module parameter\n");
    return -EINVAL;
    }
    pr_info("MTD device: %d\n", dev);
    mtd = get_mtd_device(core::ptr::null_mut(), dev);
    if (IS_ERR(mtd)) {
    err = PTR_ERR(mtd);
    pr_err("error: Cannot get MTD device\n");
    return err;
    }
    if (mtd.writesize == 1) {
    pr_info("not NAND flash, assume page size is 512 "
    "bytes.\n");
    pgsize = 512;
    } else
    pgsize = mtd.writesize;
    tmp = mtd.size;
    do_div(tmp, mtd.erasesize);
    ebcnt = tmp;
    pgcnt = mtd.erasesize / pgsize;
    pr_info("MTD device size %llu, eraseblock size %u, "
    "page size %u, count of eraseblocks %u, pages per "
    "eraseblock %u, OOB size %u\n",
    (unsigned long long)mtd.size, mtd.erasesize,
    pgsize, ebcnt, pgcnt, mtd.oobsize);
    err = -ENOMEM;
    iobuf = kmalloc(mtd.erasesize, GFP_KERNEL);
    if (!iobuf)
    goto out;
    iobuf1 = kmalloc(mtd.erasesize, GFP_KERNEL);
    if (!iobuf1)
    goto out;
    bbt = kzalloc(ebcnt, GFP_KERNEL);
    if (!bbt)
    goto out;
    err = mtdtest_scan_for_bad_eraseblocks(mtd, bbt, 0, ebcnt);
    if (err)
    goto out;
// Read all eraseblocks 1 page at a time
    pr_info("testing page read\n");
    for (i = 0; i < ebcnt; ++i) {
    int ret;
    if (bbt[i])
    continue;
    ret = read_eraseblock_by_page(i);
    if (ret) {
    dump_eraseblock(i);
    if (!err)
    err = ret;
    }
    ret = mtdtest_relax();
    if (ret) {
    err = ret;
    goto out;
    }
    }
    if (err)
    pr_info("finished with errors\n");
    else
    pr_info("finished\n");
    out:
    kfree(iobuf);
    kfree(iobuf1);
    kfree(bbt);
    put_mtd_device(mtd);
    if (err)
    pr_info("error %d occurred\n", err);
    printk(KERN_INFO "=================================================\n");
    return err;
    }
    module_init(mtd_readtest_init);
#[no_mangle]
unsafe extern "C" fn mtd_readtest_exit() -> void __exit {
    static void __exit mtd_readtest_exit(void)
    {
    return;
    }
    module_exit(mtd_readtest_exit);
    MODULE_DESCRIPTION("Read test module");
    MODULE_AUTHOR("Adrian Hunter");
    MODULE_LICENSE("GPL");
