//! Automatically rewritten from C to Rust
//! Source: kernel/kcsan/debugfs.c
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
// KCSAN debugfs interface.
//
// Copyright (C) 2019, Google LLC.
//

    atomic_long_t kcsan_counters[KCSAN_COUNTER_COUNT];
    static const char *const counter_names[] = {
    [KCSAN_COUNTER_USED_WATCHPOINTS]		= "used_watchpoints",
    [KCSAN_COUNTER_SETUP_WATCHPOINTS]		= "setup_watchpoints",
    [KCSAN_COUNTER_DATA_RACES]			= "data_races",
    [KCSAN_COUNTER_ASSERT_FAILURES]			= "assert_failures",
    [KCSAN_COUNTER_NO_CAPACITY]			= "no_capacity",
    [KCSAN_COUNTER_REPORT_RACES]			= "report_races",
    [KCSAN_COUNTER_RACES_UNKNOWN_ORIGIN]		= "races_unknown_origin",
    [KCSAN_COUNTER_UNENCODABLE_ACCESSES]		= "unencodable_accesses",
    [KCSAN_COUNTER_ENCODING_FALSE_POSITIVES]	= "encoding_false_positives",
    };
    static_assert(ARRAY_SIZE(counter_names) == KCSAN_COUNTER_COUNT);
//
// Addresses for filtering functions from reporting. This list can be used as a
// whitelist or blacklist.
//
    static struct {
    unsigned long	*addrs;		/* array of addresses */
    size_t		size;		/* current size */
    int		used;		/* number of elements used */
    bool		sorted;		/* if elements are sorted */
    bool		whitelist;	/* if list is a blacklist or whitelist */
    } report_filterlist;
    static DEFINE_RAW_SPINLOCK(report_filterlist_lock);
//
// The microbenchmark allows benchmarking KCSAN core runtime only. To run
// multiple threads, pipe 'microbench=<iters>' from multiple tasks into the
// debugfs file. This will not generate any conflicts, and tests fast-path only.
//
#[no_mangle]
unsafe extern "C" fn microbenchmark(iters: c_ulong) -> noinline void {
    static noinline void microbenchmark(unsigned long iters)
    {
    let mut ctx_save: kcsan_ctx = current.kcsan_ctx;
    let mut was_enabled: bool = READ_ONCE(kcsan_enabled);
    u64 cycles;
// We may have been called from an atomic region; reset context.
    memset(&current.kcsan_ctx, 0, sizeof(current.kcsan_ctx));
//
// Disable to benchmark fast-path for all accesses, and (expected
// negligible) call into slow-path, but never set up watchpoints.
//
    WRITE_ONCE(kcsan_enabled, false);
    pr_info("%s begin | iters: %lu\n", __func__, iters);
    cycles = get_cycles();
    while (iters--) {
    let mut addr: c_ulong = iters & ((PAGE_SIZE << 8) - 1);
    int type = !(iters & 0x7f) ? KCSAN_ACCESS_ATOMIC :
    (!(iters & 0xf) ? KCSAN_ACCESS_WRITE : 0);
    __kcsan_check_access((void *)addr, sizeof(long), type);
    }
    cycles = get_cycles() - cycles;
    pr_info("%s end   | cycles: %llu\n", __func__, cycles);
    WRITE_ONCE(kcsan_enabled, was_enabled);
// restore context
    current.kcsan_ctx = ctx_save;
    }
#[no_mangle]
unsafe extern "C" fn cmp_filterlist_addrs(rhs: *const c_void, lhs: *const c_void) -> c_int {
    static int cmp_filterlist_addrs(const void *rhs, const void *lhs)
    {
    let mut a: c_ulong = *(const unsigned long *)rhs;
    let mut b: c_ulong = *(const unsigned long *)lhs;
    return a < b ? -1 : a == b ? 0 : 1;
    }
#[no_mangle]
pub unsafe extern "C" fn kcsan_skip_report_debugfs(func_addr: c_ulong) -> bool {
    bool kcsan_skip_report_debugfs(unsigned long func_addr)
    {
    unsigned long symbolsize, offset;
    unsigned long flags;
    let mut ret: bool = false;
    if (!kallsyms_lookup_size_offset(func_addr, &symbolsize, &offset))
    return false;
    func_addr -= offset; /* Get function start */
    raw_spin_lock_irqsave(&report_filterlist_lock, flags);
    if (report_filterlist.used == 0)
    goto out;
// Sort array if it is unsorted, and then do a binary search.
    if (!report_filterlist.sorted) {
    sort(report_filterlist.addrs, report_filterlist.used,
    sizeof(unsigned long), cmp_filterlist_addrs, core::ptr::null_mut());
    report_filterlist.sorted = true;
    }
    ret = !!bsearch(&func_addr, report_filterlist.addrs,
    report_filterlist.used, sizeof(unsigned long),
    cmp_filterlist_addrs);
    if (report_filterlist.whitelist)
    ret = !ret;
    out:
    raw_spin_unlock_irqrestore(&report_filterlist_lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn set_report_filterlist_whitelist(whitelist: bool) {
    static void set_report_filterlist_whitelist(bool whitelist)
    {
    unsigned long flags;
    raw_spin_lock_irqsave(&report_filterlist_lock, flags);
    report_filterlist.whitelist = whitelist;
    raw_spin_unlock_irqrestore(&report_filterlist_lock, flags);
    }
// Returns 0 on success, error-code otherwise.
#[no_mangle]
unsafe extern "C" fn insert_report_filterlist(func: *const c_char) -> isize {
    static ssize_t insert_report_filterlist(const char *func)
    {
    unsigned long flags;
    let mut addr: c_ulong = kallsyms_lookup_name(func);
    unsigned long *delay_free = core::ptr::null_mut();
    unsigned long *new_addrs = core::ptr::null_mut();
    let mut new_size: usize = 0;
    let mut ret: isize = 0;
    if (!addr) {
    pr_err("could not find function: '%s'\n", func);
    return -ENOENT;
    }
    retry_alloc:
//
// Check if we need an allocation, and re-validate under the lock. Since
// the report_filterlist_lock is a raw, cannot allocate under the lock.
//
    if (data_race(report_filterlist.used == report_filterlist.size)) {
    new_size = (report_filterlist.size ?: 4) * 2;
    delay_free = new_addrs = kmalloc_array(new_size, sizeof(unsigned long), GFP_KERNEL);
    if (!new_addrs)
    return -ENOMEM;
    }
    raw_spin_lock_irqsave(&report_filterlist_lock, flags);
    if (report_filterlist.used == report_filterlist.size) {
// Check we pre-allocated enough, and retry if not.
    if (report_filterlist.used >= new_size) {
    raw_spin_unlock_irqrestore(&report_filterlist_lock, flags);
    kfree(new_addrs); /* kfree(core::ptr::null_mut()) is safe */
    delay_free = new_addrs = core::ptr::null_mut();
    goto retry_alloc;
    }
    if (report_filterlist.used)
    memcpy(new_addrs, report_filterlist.addrs, report_filterlist.used * sizeof(unsigned long));
    delay_free = report_filterlist.addrs; /* free the old list */
    report_filterlist.addrs = new_addrs;  /* switch to the new list */
    report_filterlist.size = new_size;
    }
// Note: deduplicating should be done in userspace.
    report_filterlist.addrs[report_filterlist.used++] = addr;
    report_filterlist.sorted = false;
    raw_spin_unlock_irqrestore(&report_filterlist_lock, flags);
    kfree(delay_free);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn show_info(file: *mut seq_file, v: *mut c_void) -> c_int {
    static int show_info(struct seq_file *file, void *v)
    {
    int i;
    unsigned long flags;
// show stats
    seq_printf(file, "enabled: %i\n", READ_ONCE(kcsan_enabled));
    for (i = 0; i < KCSAN_COUNTER_COUNT; ++i) {
    seq_printf(file, "%s: %ld\n", counter_names[i],
    atomic_long_read(&kcsan_counters[i]));
    }
// show filter functions, and filter type
    raw_spin_lock_irqsave(&report_filterlist_lock, flags);
    seq_printf(file, "\n%s functions: %s\n",
    report_filterlist.whitelist ? "whitelisted" : "blacklisted",
    report_filterlist.used == 0 ? "none" : "");
    for (i = 0; i < report_filterlist.used; ++i)
    seq_printf(file, " %ps\n", (void *)report_filterlist.addrs[i]);
    raw_spin_unlock_irqrestore(&report_filterlist_lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn debugfs_open(inode: *mut inode, file: *mut file) -> c_int {
    static int debugfs_open(struct inode *inode, struct file *file)
    {
    return single_open(file, show_info, core::ptr::null_mut());
    }
    static ssize_t
    debugfs_write(struct file *file, const char __user *buf, size_t count, loff_t *off)
    {
    char kbuf[KSYM_NAME_LEN];
    char *arg;
    let mut read_len: usize = min(count, sizeof(kbuf) - 1);
    if (copy_from_user(kbuf, buf, read_len))
    return -EFAULT;
    kbuf[read_len] = '\0';
    arg = strstrip(kbuf);
    if (!strcmp(arg, "on")) {
    WRITE_ONCE(kcsan_enabled, true);
    } else if (!strcmp(arg, "off")) {
    WRITE_ONCE(kcsan_enabled, false);
    } else if (str_has_prefix(arg, "microbench=")) {
    unsigned long iters;
    if (kstrtoul(&arg[strlen("microbench=")], 0, &iters))
    return -EINVAL;
    microbenchmark(iters);
    } else if (!strcmp(arg, "whitelist")) {
    set_report_filterlist_whitelist(true);
    } else if (!strcmp(arg, "blacklist")) {
    set_report_filterlist_whitelist(false);
    } else if (arg[0] == '!') {
    let mut ret: isize = insert_report_filterlist(&arg[1]);
    if (ret < 0)
    return ret;
    } else {
    return -EINVAL;
    }
    return count;
    }
    static const struct file_operations debugfs_ops =
    {
    .read	 = seq_read,
    .open	 = debugfs_open,
    .write	 = debugfs_write,
    .release = single_release
    };
#[no_mangle]
unsafe extern "C" fn kcsan_debugfs_init() -> int __init {
    static int __init kcsan_debugfs_init(void)
    {
    debugfs_create_file("kcsan", 0644, core::ptr::null_mut(), core::ptr::null_mut(), &debugfs_ops);
    return 0;
    }
    late_initcall(kcsan_debugfs_init);
