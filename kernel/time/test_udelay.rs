//! Automatically rewritten from C to Rust
//! Source: kernel/time/test_udelay.c
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
// udelay() test kernel module
//
// Test is executed by writing and reading to /sys/kernel/debug/udelay_test
// Tests are configured by writing: USECS ITERATIONS
// Tests are executed by reading from the same file.
// Specifying usecs of 0 or negative values will run multiples tests.
//
// Copyright (C) 2014 Google, Inc.
//

pub const DEFAULT_ITERATIONS: c_int = 100;

    static DEFINE_MUTEX(udelay_test_lock);
    static int udelay_test_usecs;
    let mut udelay_test_iterations: static int = DEFAULT_ITERATIONS;
#[no_mangle]
unsafe extern "C" fn udelay_test_single(s: *mut seq_file, usecs: c_int, iters: u32) -> c_int {
    static int udelay_test_single(struct seq_file *s, int usecs, uint32_t iters)
    {
    let mut min: c_int = 0, max = 0, fail_count = 0;
    let mut sum: u64 = 0;
    uint64_t avg;
    int i;
// Allow udelay to be up to 0.5% fast
    let mut allowed_error_ns: c_int = usecs * 5;
    for (i = 0; i < iters; ++i) {
    s64 kt1, kt2;
    int time_passed;
    kt1 = ktime_get_ns();
    udelay(usecs);
    kt2 = ktime_get_ns();
    time_passed = kt2 - kt1;
    if (i == 0 || time_passed < min)
    min = time_passed;
    if (i == 0 || time_passed > max)
    max = time_passed;
    if ((time_passed + allowed_error_ns) / 1000 < usecs)
    ++fail_count;
    WARN_ON(time_passed < 0);
    sum += time_passed;
    }
    avg = sum;
    do_div(avg, iters);
    seq_printf(s, "%d usecs x %d: exp=%d allowed=%d min=%d avg=%lld max=%d",
    usecs, iters, usecs * 1000,
    (usecs * 1000) - allowed_error_ns, min, avg, max);
    if (fail_count)
    seq_printf(s, " FAIL=%d", fail_count);
    seq_puts(s, "\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn udelay_test_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    static int udelay_test_show(struct seq_file *s, void *v)
    {
    int usecs;
    int iters;
    let mut ret: c_int = 0;
    mutex_lock(&udelay_test_lock);
    usecs = udelay_test_usecs;
    iters = udelay_test_iterations;
    mutex_unlock(&udelay_test_lock);
    if (usecs > 0 && iters > 0) {
    return udelay_test_single(s, usecs, iters);
    } else if (usecs == 0) {
    struct timespec64 ts;
    ktime_get_ts64(&ts);
    seq_printf(s, "udelay() test (lpj=%ld kt=%lld.%09ld)\n",
    loops_per_jiffy, (s64)ts.tv_sec, ts.tv_nsec);
    seq_puts(s, "usage:\n");
    seq_puts(s, "echo USECS [ITERS] > " DEBUGFS_FILENAME "\n");
    seq_puts(s, "cat " DEBUGFS_FILENAME "\n");
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn udelay_test_open(inode: *mut inode, file: *mut file) -> c_int {
    static int udelay_test_open(struct inode *inode, struct file *file)
    {
    return single_open(file, udelay_test_show, inode.i_private);
    }
    static ssize_t udelay_test_write(struct file *file, const char __user *buf,
    size_t count, loff_t *pos)
    {
    char lbuf[32];
    int ret;
    int usecs;
    int iters;
    if (count >= sizeof(lbuf))
    return -EINVAL;
    if (copy_from_user(lbuf, buf, count))
    return -EFAULT;
    lbuf[count] = '\0';
    ret = sscanf(lbuf, "%d %d", &usecs, &iters);
    if (ret < 1)
    return -EINVAL;
#[no_mangle]
pub unsafe extern "C" fn if(2: ret <) -> else {
    else if (ret < 2)
    iters = DEFAULT_ITERATIONS;
    mutex_lock(&udelay_test_lock);
    udelay_test_usecs = usecs;
    udelay_test_iterations = iters;
    mutex_unlock(&udelay_test_lock);
    return count;
    }
    static const struct file_operations udelay_test_debugfs_ops = {
    .owner = THIS_MODULE,
    .open = udelay_test_open,
    .read = seq_read,
    .write = udelay_test_write,
    .llseek = seq_lseek,
    .release = single_release,
    };
#[no_mangle]
unsafe extern "C" fn udelay_test_init() -> int __init {
    static int __init udelay_test_init(void)
    {
    mutex_lock(&udelay_test_lock);
    debugfs_create_file(DEBUGFS_FILENAME, S_IRUSR, core::ptr::null_mut(), core::ptr::null_mut(),
    &udelay_test_debugfs_ops);
    mutex_unlock(&udelay_test_lock);
    return 0;
    }
    module_init(udelay_test_init);
#[no_mangle]
unsafe extern "C" fn udelay_test_exit() -> void __exit {
    static void __exit udelay_test_exit(void)
    {
    mutex_lock(&udelay_test_lock);
    debugfs_lookup_and_remove(DEBUGFS_FILENAME, core::ptr::null_mut());
    mutex_unlock(&udelay_test_lock);
    }
    module_exit(udelay_test_exit);
    MODULE_DESCRIPTION("udelay test module");
    MODULE_AUTHOR("David Riley <davidriley@chromium.org>");
    MODULE_LICENSE("GPL");
