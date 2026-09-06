//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/s390-trng.c
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
// s390 TRNG device driver
//
// Driver for the TRNG (true random number generation) command
// available via CPACF extension MSA 7 on the s390 arch.
// Copyright IBM Corp. 2017
// Author(s): Harald Freudenberger <freude@de.ibm.com>
//

    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("IBM Corporation");
    MODULE_DESCRIPTION("s390 CPACF TRNG device driver");
// trng related debug feature things
    static debug_info_t *debug_info;

// trng helpers
    let mut trng_dev_counter: static atomic64_t = ATOMIC64_INIT(0);
    let mut trng_hwrng_counter: static atomic64_t = ATOMIC64_INIT(0);
// file io functions
#[no_mangle]
unsafe extern "C" fn trng_open(inode: *mut inode, file: *mut file) -> c_int {
    static int trng_open(struct inode *inode, struct file *file)
    {
    return nonseekable_open(inode, file);
    }
    static ssize_t trng_read(struct file *file, char __user *ubuf,
    size_t nbytes, loff_t *ppos)
    {
    u8 buf[32];
    u8 *p = buf;
    unsigned int n;
    let mut ret: isize = 0;
//
// use buf for requests <= sizeof(buf),
// otherwise allocate one page and fetch
// pagewise.
//
    if (nbytes > sizeof(buf)) {
    p = kmalloc(PAGE_SIZE, GFP_KERNEL);
    if (!p)
    return -ENOMEM;
    }
    while (nbytes) {
    if (need_resched()) {
    if (signal_pending(current)) {
    if (ret == 0)
    ret = -ERESTARTSYS;
    break;
    }
    schedule();
    }
    n = nbytes > PAGE_SIZE ? PAGE_SIZE : nbytes;
    cpacf_trng(core::ptr::null_mut(), 0, p, n);
    atomic64_add(n, &trng_dev_counter);
    if (copy_to_user(ubuf, p, n)) {
    ret = -EFAULT;
    break;
    }
    nbytes -= n;
    ubuf += n;
    ret += n;
    }
    if (p != buf)
    kfree(p);
    DEBUG_DBG("trng_read()=%zd\n", ret);
    return ret;
    }
// sysfs
    static ssize_t trng_counter_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut dev_counter: u64 = atomic64_read(&trng_dev_counter);
    let mut hwrng_counter: u64 = atomic64_read(&trng_hwrng_counter);
    let mut arch_counter: u64 = atomic64_read(&s390_arch_random_counter);
    return sysfs_emit(buf,
    "trng:  %llu\n"
    "hwrng: %llu\n"
    "arch:  %llu\n"
    "total: %llu\n",
    dev_counter, hwrng_counter, arch_counter,
    dev_counter + hwrng_counter + arch_counter);
    }
    static DEVICE_ATTR(byte_counter, 0444, trng_counter_show, core::ptr::null_mut());
    static struct attribute *trng_dev_attrs[] = {
    &dev_attr_byte_counter.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group trng_dev_attr_group = {
    .attrs = trng_dev_attrs
    };
    static const struct attribute_group *trng_dev_attr_groups[] = {
    &trng_dev_attr_group,
    core::ptr::null_mut()
    };
    static const struct file_operations trng_fops = {
    .owner		= THIS_MODULE,
    .open		= &trng_open,
    .release	= core::ptr::null_mut(),
    .read		= &trng_read,
    .llseek		= noop_llseek,
    };
    static struct miscdevice trng_dev = {
    .name	= "trng",
    .minor	= MISC_DYNAMIC_MINOR,
    .mode	= 0444,
    .fops	= &trng_fops,
    .groups = trng_dev_attr_groups,
    };
// hwrng_register
#[no_mangle]
pub unsafe extern "C" fn _trng_hwrng_read(buf: *mut u8, len: usize) {
    static inline void _trng_hwrng_read(u8 *buf, size_t len)
    {
    cpacf_trng(core::ptr::null_mut(), 0, buf, len);
    atomic64_add(len, &trng_hwrng_counter);
    }
#[no_mangle]
unsafe extern "C" fn trng_hwrng_data_read(rng: *mut hwrng, data: *mut u32) -> c_int {
    static int trng_hwrng_data_read(struct hwrng *rng, u32 *data)
    {
    let mut len: usize = sizeof(*data);
    _trng_hwrng_read((u8 *) data, len);
    DEBUG_DBG("trng_hwrng_data_read()=%zu\n", len);
    return len;
    }
#[no_mangle]
unsafe extern "C" fn trng_hwrng_read(rng: *mut hwrng, data: *mut c_void, max: usize, wait: bool) -> c_int {
    static int trng_hwrng_read(struct hwrng *rng, void *data, size_t max, bool wait)
    {
    let mut len: usize = max <= PAGE_SIZE ? max : PAGE_SIZE;
    _trng_hwrng_read((u8 *) data, len);
    DEBUG_DBG("trng_hwrng_read()=%zu\n", len);
    return len;
    }
//
// hwrng register struct
// The trng is supposed to have 100% entropy, and thus we register with a very
// high quality value. If we ever have a better driver in the future, we should
// change this value again when we merge this driver.
//
    static struct hwrng trng_hwrng_dev = {
    .name		= "s390-trng",
    .data_read	= trng_hwrng_data_read,
    .read		= trng_hwrng_read,
    };
// init and exit
#[no_mangle]
unsafe extern "C" fn trng_debug_init() -> void __init {
    static void __init trng_debug_init(void)
    {
    debug_info = debug_register("trng", 1, 1, 4 * sizeof(long));
    debug_register_view(debug_info, &debug_sprintf_view);
    debug_set_level(debug_info, 3);
    }
#[no_mangle]
unsafe extern "C" fn trng_debug_exit() {
    static void trng_debug_exit(void)
    {
    debug_unregister(debug_info);
    }
#[no_mangle]
unsafe extern "C" fn trng_init() -> int __init {
    static int __init trng_init(void)
    {
    int ret;
    trng_debug_init();
// check if subfunction CPACF_PRNO_TRNG is available
    if (!cpacf_query_func(CPACF_PRNO, CPACF_PRNO_TRNG)) {
    DEBUG_INFO("trng_init CPACF_PRNO_TRNG not available\n");
    ret = -ENODEV;
    goto out_dbg;
    }
    ret = misc_register(&trng_dev);
    if (ret) {
    DEBUG_WARN("trng_init misc_register() failed rc=%d\n", ret);
    goto out_dbg;
    }
    ret = hwrng_register(&trng_hwrng_dev);
    if (ret) {
    DEBUG_WARN("trng_init hwrng_register() failed rc=%d\n", ret);
    goto out_misc;
    }
    DEBUG_DBG("trng_init successful\n");
    return 0;
    out_misc:
    misc_deregister(&trng_dev);
    out_dbg:
    trng_debug_exit();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn trng_exit() -> void __exit {
    static void __exit trng_exit(void)
    {
    hwrng_unregister(&trng_hwrng_dev);
    misc_deregister(&trng_dev);
    trng_debug_exit();
    }
    module_cpu_feature_match(S390_CPU_FEATURE_MSA, trng_init);
    module_exit(trng_exit);
