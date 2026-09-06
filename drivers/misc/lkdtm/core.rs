//! Automatically rewritten from C to Rust
//! Source: drivers/misc/lkdtm/core.c
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
// Linux Kernel Dump Test Module for testing kernel crashes conditions:
// induces system failures at predefined crashpoints and under predefined
// operational conditions in order to evaluate the reliability of kernel
// sanity checking and crash dumps obtained using different dumping
// solutions.
//
// Copyright (C) IBM Corporation, 2006
//
// Author: Ankita Garg <ankita@in.ibm.com>
//
// It is adapted from the Linux Kernel Dump Test Tool by
// Fernando Luis Vazquez Cao <http://lkdtt.sourceforge.net>
//
// Debugfs support added by Simon Kagstrom <simon.kagstrom@netinsight.net>
//
// See Documentation/fault-injection/provoke-crashes.rst for instructions
//

pub const DEFAULT_COUNT: c_int = 10;
    static int lkdtm_debugfs_open(struct inode *inode, struct file *file);
    static ssize_t lkdtm_debugfs_read(struct file *f, char __user *user_buf,
    size_t count, loff_t *off);
    static ssize_t direct_entry(struct file *f, const char __user *user_buf,
    size_t count, loff_t *off);

    static int lkdtm_kprobe_handler(struct kprobe *kp, struct pt_regs *regs);
    static ssize_t lkdtm_debugfs_entry(struct file *f,
    const char __user *user_buf,
    size_t count, loff_t *off);

    .kprobe = {					\
    .symbol_name = (_symbol),		\
    .pre_handler = lkdtm_kprobe_handler,	\
    },

    (_symbol) ? lkdtm_debugfs_entry : direct_entry

// Crash points
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crashpoint {
    pub name: *const c_char,
    pub fops: file_operations,
    pub kprobe: kprobe,
}

    {							\
    .name = _name,					\
    .fops = {					\
    .read	= lkdtm_debugfs_read,		\
    .llseek	= generic_file_llseek,		\
    .open	= lkdtm_debugfs_open,		\
    .write	= CRASHPOINT_WRITE(_symbol)	\
    },						\
    CRASHPOINT_KPROBE(_symbol)			\
    }
// Define the possible places where we can trigger a crash point.
    static struct crashpoint crashpoints[] = {
    CRASHPOINT("DIRECT",		 core::ptr::null_mut()),

    CRASHPOINT("INT_HARDWARE_ENTRY", "do_IRQ"),
    CRASHPOINT("INT_HW_IRQ_EN",	 "handle_irq_event"),
    CRASHPOINT("INT_TASKLET_ENTRY",	 "tasklet_action"),
    CRASHPOINT("FS_SUBMIT_BH",		 "submit_bh"),
    CRASHPOINT("MEM_SWAPOUT",	 "shrink_inactive_list"),
    CRASHPOINT("TIMERADD",		 "hrtimer_start"),
    CRASHPOINT("SCSI_QUEUE_RQ",	 "scsi_queue_rq"),

    };
// List of possible types for crashes that can be triggered.
    static const struct crashtype_category *crashtype_categories[] = {
    &bugs_crashtypes,
    &heap_crashtypes,
    &perms_crashtypes,
    &refcount_crashtypes,
    &usercopy_crashtypes,
    &stackleak_crashtypes,
    &cfi_crashtypes,
    &fortify_crashtypes,

    &powerpc_crashtypes,

    };
// Global kprobe entry and crashtype.
    static struct kprobe *lkdtm_kprobe;
    static struct crashpoint *lkdtm_crashpoint;
    static const struct crashtype *lkdtm_crashtype;
// Module parameters
    let mut recur_count: static int = -1;
    module_param(recur_count, int, 0644);
    MODULE_PARM_DESC(recur_count, " Recursion level for the stack overflow test");
    static char* cpoint_name;
    module_param(cpoint_name, charp, 0444);
    MODULE_PARM_DESC(cpoint_name, " Crash Point, where kernel is to be crashed");
    static char* cpoint_type;
    module_param(cpoint_type, charp, 0444);
    MODULE_PARM_DESC(cpoint_type, " Crash Point Type, action to be taken on "\
    "hitting the crash point");
    let mut cpoint_count: static int = DEFAULT_COUNT;
    module_param(cpoint_count, int, 0644);
    MODULE_PARM_DESC(cpoint_count, " Crash Point Count, number of times the "\
    "crash point is to be hit to trigger action");
//
// For test debug reporting when CI systems provide terse summaries.
// TODO: Remove this once reasonable reporting exists in most CI systems:
// https://lore.kernel.org/lkml/CAHk-=wiFvfkoFixTapvvyPMN9pq5G-+Dys2eSyBa1vzDGAO5+A@mail.gmail.com
//
    char *lkdtm_kernel_info;
// Return the crashtype number or NULL if the name is invalid
    static const struct crashtype *find_crashtype(const char *name)
    {
    int cat, idx;
    for (cat = 0; cat < ARRAY_SIZE(crashtype_categories); cat++) {
    for (idx = 0; idx < crashtype_categories[cat].len; idx++) {
    struct crashtype *crashtype;
    crashtype = &crashtype_categories[cat].crashtypes[idx];
    if (!strcmp(name, crashtype.name))
    return crashtype;
    }
    }
    return core::ptr::null_mut();
    }
//
// This is forced noinline just so it distinctly shows up in the stackdump
// which makes validation of expected lkdtm crashes easier.
//
// NOTE: having a valid return value helps prevent the compiler from doing
// tail call optimizations and taking this out of the stack trace.
//
#[no_mangle]
unsafe extern "C" fn lkdtm_do_action(crashtype: *const crashtype) -> noinline int {
    static noinline int lkdtm_do_action(const struct crashtype *crashtype)
    {
    if (WARN_ON(!crashtype || !crashtype.func))
    return -EINVAL;
    crashtype.func();
    return 0;
    }
    static int lkdtm_register_cpoint(struct crashpoint *crashpoint,
    const struct crashtype *crashtype)
    {
    int ret;
// If this doesn't have a symbol, just call immediately.
    if (!crashpoint.kprobe.symbol_name)
    return lkdtm_do_action(crashtype);
    if (lkdtm_kprobe != core::ptr::null_mut())
    unregister_kprobe(lkdtm_kprobe);
    lkdtm_crashpoint = crashpoint;
    lkdtm_crashtype = crashtype;
    lkdtm_kprobe = &crashpoint.kprobe;
    ret = register_kprobe(lkdtm_kprobe);
    if (ret < 0) {
    pr_info("Couldn't register kprobe %s\n",
    crashpoint.kprobe.symbol_name);
    lkdtm_kprobe = core::ptr::null_mut();
    lkdtm_crashpoint = core::ptr::null_mut();
    lkdtm_crashtype = core::ptr::null_mut();
    }
    return ret;
    }

// Global crash counter and spinlock.
    let mut crash_count: static int = DEFAULT_COUNT;
    static DEFINE_SPINLOCK(crash_count_lock);
// Called by kprobe entry points.
#[no_mangle]
unsafe extern "C" fn lkdtm_kprobe_handler(kp: *mut kprobe, regs: *mut pt_regs) -> c_int {
    static int lkdtm_kprobe_handler(struct kprobe *kp, struct pt_regs *regs)
    {
    unsigned long flags;
    let mut do_it: bool = false;
    if (WARN_ON(!lkdtm_crashpoint || !lkdtm_crashtype))
    return 0;
    spin_lock_irqsave(&crash_count_lock, flags);
    crash_count--;
    pr_info("Crash point %s of type %s hit, trigger in %d rounds\n",
    lkdtm_crashpoint.name, lkdtm_crashtype.name, crash_count);
    if (crash_count == 0) {
    do_it = true;
    crash_count = cpoint_count;
    }
    spin_unlock_irqrestore(&crash_count_lock, flags);
    if (do_it)
    return lkdtm_do_action(lkdtm_crashtype);
    return 0;
    }
    static ssize_t lkdtm_debugfs_entry(struct file *f,
    const char __user *user_buf,
    size_t count, loff_t *off)
    {
    struct crashpoint *crashpoint = file_inode(f).i_private;
    const struct crashtype *crashtype = core::ptr::null_mut();
    char *buf;
    int err;
    if (count >= PAGE_SIZE)
    return -EINVAL;
    buf = (char *)__get_free_page(GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    if (copy_from_user(buf, user_buf, count)) {
    free_page((unsigned long) buf);
    return -EFAULT;
    }
// NULL-terminate and remove enter
    buf[count] = '\0';
    strim(buf);
    crashtype = find_crashtype(buf);
    free_page((unsigned long)buf);
    if (!crashtype)
    return -EINVAL;
    err = lkdtm_register_cpoint(crashpoint, crashtype);
    if (err < 0)
    return err;
// off += count;
    return count;
    }

// Generic read callback that just prints out the available crash types
    static ssize_t lkdtm_debugfs_read(struct file *f, char __user *user_buf,
    size_t count, loff_t *off)
    {
    int n, cat, idx;
    ssize_t out;
    char *buf;
    buf = (char *)__get_free_page(GFP_KERNEL);
    if (buf == core::ptr::null_mut())
    return -ENOMEM;
    n = scnprintf(buf, PAGE_SIZE, "Available crash types:\n");
    for (cat = 0; cat < ARRAY_SIZE(crashtype_categories); cat++) {
    for (idx = 0; idx < crashtype_categories[cat].len; idx++) {
    struct crashtype *crashtype;
    crashtype = &crashtype_categories[cat].crashtypes[idx];
    n += scnprintf(buf + n, PAGE_SIZE - n, "%s\n",
    crashtype.name);
    }
    }
    buf[n] = '\0';
    out = simple_read_from_buffer(user_buf, count, off,
    buf, n);
    free_page((unsigned long) buf);
    return out;
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_debugfs_open(inode: *mut inode, file: *mut file) -> c_int {
    static int lkdtm_debugfs_open(struct inode *inode, struct file *file)
    {
    return 0;
    }
// Special entry to just crash directly. Available without KPROBEs
    static ssize_t direct_entry(struct file *f, const char __user *user_buf,
    size_t count, loff_t *off)
    {
    const struct crashtype *crashtype;
    char *buf;
    int err;
    if (count >= PAGE_SIZE)
    return -EINVAL;
    if (count < 1)
    return -EINVAL;
    buf = (char *)__get_free_page(GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    if (copy_from_user(buf, user_buf, count)) {
    free_page((unsigned long) buf);
    return -EFAULT;
    }
// NULL-terminate and remove enter
    buf[count] = '\0';
    strim(buf);
    crashtype = find_crashtype(buf);
    free_page((unsigned long) buf);
    if (!crashtype)
    return -EINVAL;
    pr_info("Performing direct entry %s\n", crashtype.name);
    err = lkdtm_do_action(crashtype);
// off += count;
    if (err)
    return err;
    return count;
    }

//
// To avoid needing to export parse_args(), just don't use this code
// when LKDTM is built as a module.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct check_cmdline_args {
    pub param: *const c_char,
    pub value: c_int,
}

    static int lkdtm_parse_one(char *param, char *val,
    const char *unused, void *arg)
    {
    struct check_cmdline_args *args = arg;
// short circuit if we already found a value.
    if (args.value != -ESRCH)
    return 0;
    if (strncmp(param, args.param, strlen(args.param)) == 0) {
    bool bool_result;
    int ret;
    ret = kstrtobool(val, &bool_result);
    if (ret == 0)
    args.value = bool_result;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn lkdtm_check_bool_cmdline(param: *const c_char) -> c_int {
    int lkdtm_check_bool_cmdline(const char *param)
    {
    char *command_line;
    struct check_cmdline_args args = {
    .param = param,
    .value = -ESRCH,
    };
    command_line = kstrdup(saved_command_line, GFP_KERNEL);
    if (!command_line)
    return -ENOMEM;
    parse_args("Setting sysctl args", command_line,
    core::ptr::null_mut(), 0, -1, -1, &args, lkdtm_parse_one);
    kfree(command_line);
    return args.value;
    }

    static struct dentry *lkdtm_debugfs_root;
#[no_mangle]
unsafe extern "C" fn lkdtm_module_init() -> int __init {
    static int __init lkdtm_module_init(void)
    {
    struct crashpoint *crashpoint = core::ptr::null_mut();
    const struct crashtype *crashtype = core::ptr::null_mut();
    int ret;
    int i;
// Neither or both of these need to be set
    if ((cpoint_type || cpoint_name) && !(cpoint_type && cpoint_name)) {
    pr_err("Need both cpoint_type and cpoint_name or neither\n");
    return -EINVAL;
    }
    if (cpoint_type) {
    crashtype = find_crashtype(cpoint_type);
    if (!crashtype) {
    pr_err("Unknown crashtype '%s'\n", cpoint_type);
    return -EINVAL;
    }
    }
    if (cpoint_name) {
    for (i = 0; i < ARRAY_SIZE(crashpoints); i++) {
    if (!strcmp(cpoint_name, crashpoints[i].name))
    crashpoint = &crashpoints[i];
    }
// Refuse unknown crashpoints.
    if (!crashpoint) {
    pr_err("Invalid crashpoint %s\n", cpoint_name);
    return -EINVAL;
    }
    }

// Set crash count.
    crash_count = cpoint_count;

// Common initialization.
    lkdtm_kernel_info = kasprintf(GFP_KERNEL, "kernel (%s %s)",
    init_uts_ns.name.release,
    init_uts_ns.name.machine);
// Handle test-specific initialization.
    lkdtm_bugs_init(&recur_count);
    lkdtm_perms_init();
    lkdtm_usercopy_init();
    lkdtm_heap_init();
// Register debugfs interface
    lkdtm_debugfs_root = debugfs_create_dir("provoke-crash", core::ptr::null_mut());
// Install debugfs trigger files.
    for (i = 0; i < ARRAY_SIZE(crashpoints); i++) {
    struct crashpoint *cur = &crashpoints[i];
    debugfs_create_file(cur.name, 0644, lkdtm_debugfs_root, cur,
    &cur.fops);
    }
// Install crashpoint if one was selected.
    if (crashpoint) {
    ret = lkdtm_register_cpoint(crashpoint, crashtype);
    if (ret < 0) {
    pr_info("Invalid crashpoint %s\n", crashpoint.name);
    goto out_err;
    }
    pr_info("Crash point %s of type %s registered\n",
    crashpoint.name, cpoint_type);
    } else {
    pr_info("No crash points registered, enable through debugfs\n");
    }
    return 0;
    out_err:
    debugfs_remove_recursive(lkdtm_debugfs_root);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_module_exit() -> void __exit {
    static void __exit lkdtm_module_exit(void)
    {
    debugfs_remove_recursive(lkdtm_debugfs_root);
// Handle test-specific clean-up.
    lkdtm_heap_exit();
    lkdtm_usercopy_exit();
    if (lkdtm_kprobe != core::ptr::null_mut())
    unregister_kprobe(lkdtm_kprobe);
    kfree(lkdtm_kernel_info);
    pr_info("Crash point unregistered\n");
    }
    module_init(lkdtm_module_init);
    module_exit(lkdtm_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Kernel crash testing module");
