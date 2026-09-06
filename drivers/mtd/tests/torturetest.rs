//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/tests/torturetest.c
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
// Copyright (C) 2006-2008 Artem Bityutskiy
// Copyright (C) 2006-2008 Jarkko Lavinen
// Copyright (C) 2006-2008 Adrian Hunter
//
// Authors: Artem Bityutskiy, Jarkko Lavinen, Adria Hunter
//
// WARNING: this test program may kill your flash and your device. Do not
// use it unless you know what you do. Authors are not responsible for any
// damage caused by this program.
//

pub const RETRIES: c_int = 3;
    let mut eb: static int = 8;
    module_param(eb, int, S_IRUGO);
    MODULE_PARM_DESC(eb, "eraseblock number within the selected MTD device");
    let mut ebcnt: static int = 32;
    module_param(ebcnt, int, S_IRUGO);
    MODULE_PARM_DESC(ebcnt, "number of consecutive eraseblocks to torture");
    static int pgcnt;
    module_param(pgcnt, int, S_IRUGO);
    MODULE_PARM_DESC(pgcnt, "number of pages per eraseblock to torture (0 => all)");
    let mut dev: static int = -EINVAL;
    module_param(dev, int, S_IRUGO);
    MODULE_PARM_DESC(dev, "MTD device number to use");
    let mut gran: static int = 512;
    module_param(gran, int, S_IRUGO);
    MODULE_PARM_DESC(gran, "how often the status information should be printed");
    let mut check: static int = 1;
    module_param(check, int, S_IRUGO);
    MODULE_PARM_DESC(check, "if the written data should be checked");
    static unsigned int cycles_count;
    module_param(cycles_count, uint, S_IRUGO);
    MODULE_PARM_DESC(cycles_count, "how many erase cycles to do "
    "(infinite by default)");
    static struct mtd_info *mtd;
// This buffer contains 0x555555...0xAAAAAA... pattern
    static unsigned char *patt_5A5;
// This buffer contains 0xAAAAAA...0x555555... pattern
    static unsigned char *patt_A5A;
// This buffer contains all 0xFF bytes
    static unsigned char *patt_FF;
// This a temporary buffer is use when checking data
    static unsigned char *check_buf;
// How many erase cycles were done
    static unsigned int erase_cycles;
    static int pgsize;
    static ktime_t start, finish;
    static void report_corrupt(unsigned char *read, unsigned char *written);
#[no_mangle]
pub unsafe extern "C" fn start_timing() {
    static inline void start_timing(void)
    {
    start = ktime_get();
    }
#[no_mangle]
pub unsafe extern "C" fn stop_timing() {
    static inline void stop_timing(void)
    {
    finish = ktime_get();
    }
//
// Check that the contents of eraseblock number @enbum is equivalent to the
// @buf buffer.
//
#[no_mangle]
pub unsafe extern "C" fn check_eraseblock(ebnum: c_int, buf: *mut c_uchar) -> c_int {
    static inline int check_eraseblock(int ebnum, unsigned char *buf)
    {
    int err, retries = 0;
    size_t read;
    let mut addr: loff_t = (loff_t)ebnum * mtd.erasesize;
    let mut len: usize = mtd.erasesize;
    if (pgcnt) {
    addr = (loff_t)(ebnum + 1) * mtd.erasesize - pgcnt * pgsize;
    len = pgcnt * pgsize;
    }
    retry:
    err = mtd_read(mtd, addr, len, &read, check_buf);
    if (mtd_is_bitflip(err))
    pr_err("single bit flip occurred at EB %d "
    "MTD reported that it was fixed.\n", ebnum);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: err) -> else {
    pr_err("error %d while reading EB %d, "
    "read %zd\n", err, ebnum, read);
    return err;
    }
    if (read != len) {
    pr_err("failed to read %zd bytes from EB %d, "
    "read only %zd, but no error reported\n",
    len, ebnum, read);
    return -EIO;
    }
    if (memcmp(buf, check_buf, len)) {
    pr_err("read wrong data from EB %d\n", ebnum);
    report_corrupt(check_buf, buf);
    if (retries++ < RETRIES) {
// Try read again
    yield();
    pr_info("re-try reading data from EB %d\n",
    ebnum);
    goto retry;
    } else {
    pr_info("retried %d times, still errors, "
    "give-up\n", RETRIES);
    return -EINVAL;
    }
    }
    if (retries != 0)
    pr_info("only attempt number %d was OK (!!!)\n",
    retries);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn write_pattern(ebnum: c_int, buf: *mut c_void) -> c_int {
    static inline int write_pattern(int ebnum, void *buf)
    {
    int err;
    size_t written;
    let mut addr: loff_t = (loff_t)ebnum * mtd.erasesize;
    let mut len: usize = mtd.erasesize;
    if (pgcnt) {
    addr = (loff_t)(ebnum + 1) * mtd.erasesize - pgcnt * pgsize;
    len = pgcnt * pgsize;
    }
    err = mtd_write(mtd, addr, len, &written, buf);
    if (err) {
    pr_err("error %d while writing EB %d, written %zd"
    " bytes\n", err, ebnum, written);
    return err;
    }
    if (written != len) {
    pr_info("written only %zd bytes of %zd, but no error"
    " reported\n", written, len);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tort_init() -> int __init {
    static int __init tort_init(void)
    {
    let mut err: c_int = 0, i, infinite = !cycles_count;
    unsigned char *bad_ebs;
    printk(KERN_INFO "\n");
    printk(KERN_INFO "=================================================\n");
    pr_info("Warning: this program is trying to wear out your "
    "flash, stop it if this is not wanted.\n");
    if (dev < 0) {
    pr_info("Please specify a valid mtd-device via module parameter\n");
    pr_crit("CAREFUL: This test wipes all data on the specified MTD device!\n");
    return -EINVAL;
    }
    pr_info("MTD device: %d\n", dev);
    pr_info("torture %d eraseblocks (%d-%d) of mtd%d\n",
    ebcnt, eb, eb + ebcnt - 1, dev);
    if (pgcnt)
    pr_info("torturing just %d pages per eraseblock\n",
    pgcnt);
    pr_info("write verify %s\n", check ? "enabled" : "disabled");
    mtd = get_mtd_device(core::ptr::null_mut(), dev);
    if (IS_ERR(mtd)) {
    err = PTR_ERR(mtd);
    pr_err("error: cannot get MTD device\n");
    return err;
    }
    if (mtd.writesize == 1) {
    pr_info("not NAND flash, assume page size is 512 "
    "bytes.\n");
    pgsize = 512;
    } else
    pgsize = mtd.writesize;
    if (pgcnt && (pgcnt > mtd.erasesize / pgsize || pgcnt < 0)) {
    pr_err("error: invalid pgcnt value %d\n", pgcnt);
    goto out_mtd;
    }
    err = -ENOMEM;
    patt_5A5 = kmalloc(mtd.erasesize, GFP_KERNEL);
    if (!patt_5A5)
    goto out_mtd;
    patt_A5A = kmalloc(mtd.erasesize, GFP_KERNEL);
    if (!patt_A5A)
    goto out_patt_5A5;
    patt_FF = kmalloc(mtd.erasesize, GFP_KERNEL);
    if (!patt_FF)
    goto out_patt_A5A;
    check_buf = kmalloc(mtd.erasesize, GFP_KERNEL);
    if (!check_buf)
    goto out_patt_FF;
    bad_ebs = kzalloc(ebcnt, GFP_KERNEL);
    if (!bad_ebs)
    goto out_check_buf;
// Initialize patterns
    memset(patt_FF, 0xFF, mtd.erasesize);
    for (i = 0; i < mtd.erasesize / pgsize; i++) {
    if (!(i & 1)) {
    memset(patt_5A5 + i * pgsize, 0x55, pgsize);
    memset(patt_A5A + i * pgsize, 0xAA, pgsize);
    } else {
    memset(patt_5A5 + i * pgsize, 0xAA, pgsize);
    memset(patt_A5A + i * pgsize, 0x55, pgsize);
    }
    }
    err = mtdtest_scan_for_bad_eraseblocks(mtd, bad_ebs, eb, ebcnt);
    if (err)
    goto out;
    start_timing();
    while (1) {
    int i;
    void *patt;
    err = mtdtest_erase_good_eraseblocks(mtd, bad_ebs, eb, ebcnt);
    if (err)
    goto out;
// Check if the eraseblocks contain only 0xFF bytes
    if (check) {
    for (i = eb; i < eb + ebcnt; i++) {
    if (bad_ebs[i - eb])
    continue;
    err = check_eraseblock(i, patt_FF);
    if (err) {
    pr_info("verify failed"
    " for 0xFF... pattern\n");
    goto out;
    }
    err = mtdtest_relax();
    if (err)
    goto out;
    }
    }
// Write the pattern
    for (i = eb; i < eb + ebcnt; i++) {
    if (bad_ebs[i - eb])
    continue;
    if ((eb + erase_cycles) & 1)
    patt = patt_5A5;
    else
    patt = patt_A5A;
    err = write_pattern(i, patt);
    if (err)
    goto out;
    err = mtdtest_relax();
    if (err)
    goto out;
    }
// Verify what we wrote
    if (check) {
    for (i = eb; i < eb + ebcnt; i++) {
    if (bad_ebs[i - eb])
    continue;
    if ((eb + erase_cycles) & 1)
    patt = patt_5A5;
    else
    patt = patt_A5A;
    err = check_eraseblock(i, patt);
    if (err) {
    pr_info("verify failed for %s"
    " pattern\n",
    ((eb + erase_cycles) & 1) ?
    "0x55AA55..." : "0xAA55AA...");
    goto out;
    }
    err = mtdtest_relax();
    if (err)
    goto out;
    }
    }
    erase_cycles += 1;
    if (erase_cycles % gran == 0) {
    long ms;
    stop_timing();
    ms = ktime_ms_delta(finish, start);
    pr_info("%08u erase cycles done, took %lu "
    "milliseconds (%lu seconds)\n",
    erase_cycles, ms, ms / 1000);
    start_timing();
    }
    if (!infinite && --cycles_count == 0)
    break;
    }
    out:
    pr_info("finished after %u erase cycles\n",
    erase_cycles);
    kfree(bad_ebs);
    out_check_buf:
    kfree(check_buf);
    out_patt_FF:
    kfree(patt_FF);
    out_patt_A5A:
    kfree(patt_A5A);
    out_patt_5A5:
    kfree(patt_5A5);
    out_mtd:
    put_mtd_device(mtd);
    if (err)
    pr_info("error %d occurred during torturing\n", err);
    printk(KERN_INFO "=================================================\n");
    return err;
    }
    module_init(tort_init);
#[no_mangle]
unsafe extern "C" fn tort_exit() -> void __exit {
    static void __exit tort_exit(void)
    {
    return;
    }
    module_exit(tort_exit);
    static int countdiffs(unsigned char *buf, unsigned char *check_buf,
    unsigned offset, unsigned len, unsigned *bytesp,
    unsigned *bitsp);
    static void print_bufs(unsigned char *read, unsigned char *written, int start,
    int len);
//
// Report the detailed information about how the read EB differs from what was
// written.
//
#[no_mangle]
unsafe extern "C" fn report_corrupt(read: *mut c_uchar, written: *mut c_uchar) {
    static void report_corrupt(unsigned char *read, unsigned char *written)
    {
    int i;
    int bytes, bits, pages, first;
    int offset, len;
    let mut check_len: usize = mtd.erasesize;
    if (pgcnt)
    check_len = pgcnt * pgsize;
    bytes = bits = pages = 0;
    for (i = 0; i < check_len; i += pgsize)
    if (countdiffs(written, read, i, pgsize, &bytes,
    &bits) >= 0)
    pages++;
    pr_info("verify fails on %d pages, %d bytes/%d bits\n",
    pages, bytes, bits);
    pr_info("The following is a list of all differences between"
    " what was read from flash and what was expected\n");
    for (i = 0; i < check_len; i += pgsize) {
    cond_resched();
    bytes = bits = 0;
    first = countdiffs(written, read, i, pgsize, &bytes,
    &bits);
    if (first < 0)
    continue;
    printk("-------------------------------------------------------"
    "----------------------------------\n");
    pr_info("Page %zd has %d bytes/%d bits failing verify,"
    " starting at offset 0x%x\n",
    (mtd.erasesize - check_len + i) / pgsize,
    bytes, bits, first);
    offset = first & ~0x7;
    len = ((first + bytes) | 0x7) + 1 - offset;
    print_bufs(read, written, offset, len);
    }
    }
    static void print_bufs(unsigned char *read, unsigned char *written, int start,
    int len)
    {
    let mut i: c_int = 0, j1, j2;
    char *diff;
    printk("Offset       Read                          Written\n");
    while (i < len) {
    printk("0x%08x: ", start + i);
    diff = "   ";
    for (j1 = 0; j1 < 8 && i + j1 < len; j1++) {
    printk(" %02x", read[start + i + j1]);
    if (read[start + i + j1] != written[start + i + j1])
    diff = "***";
    }
    while (j1 < 8) {
    printk(" ");
    j1 += 1;
    }
    printk("  %s ", diff);
    for (j2 = 0; j2 < 8 && i + j2 < len; j2++)
    printk(" %02x", written[start + i + j2]);
    printk("\n");
    i += 8;
    }
    }
//
// Count the number of differing bytes and bits and return the first differing
// offset.
//
    static int countdiffs(unsigned char *buf, unsigned char *check_buf,
    unsigned offset, unsigned len, unsigned *bytesp,
    unsigned *bitsp)
    {
    unsigned i, bit;
    let mut first: c_int = -1;
    for (i = offset; i < offset + len; i++)
    if (buf[i] != check_buf[i]) {
    first = i;
    break;
    }
    while (i < offset + len) {
    if (buf[i] != check_buf[i]) {
    (*bytesp)++;
    bit = 1;
    while (bit < 256) {
    if ((buf[i] & bit) != (check_buf[i] & bit))
    (*bitsp)++;
    bit <<= 1;
    }
    }
    i++;
    }
    return first;
    }
    MODULE_DESCRIPTION("Eraseblock torturing module");
    MODULE_AUTHOR("Artem Bityutskiy, Jarkko Lavinen, Adrian Hunter");
    MODULE_LICENSE("GPL");
