//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/kdebugfs.c
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

    struct dentry *arch_debugfs_dir;
    EXPORT_SYMBOL(arch_debugfs_dir);
    static int sfb_state, tso_state;
#[no_mangle]
unsafe extern "C" fn set_sfb_state(info: *mut c_void) {
    static void set_sfb_state(void *info)
    {
    let mut val: c_int = *(int *)info << CSR_STFILL_SHIFT;
    csr_xchg32(val, CSR_STFILL, LOONGARCH_CSR_IMPCTL1);
    }
#[no_mangle]
unsafe extern "C" fn sfb_read(file: *mut file, buf: *mut char __user, count: usize, ppos: *mut loff_t) -> isize {
    static ssize_t sfb_read(struct file *file, char __user *buf, size_t count, loff_t *ppos)
    {
    int s, state;
    char str[32];
    state = (csr_read32(LOONGARCH_CSR_IMPCTL1) & CSR_STFILL) >> CSR_STFILL_SHIFT;
    s = snprintf(str, sizeof(str), "Boot State: %x\nCurrent State: %x\n", sfb_state, state);
    if (*ppos >= s)
    return 0;
    s -= *ppos;
    s = min_t(u32, s, count);
    if (copy_to_user(buf, &str[*ppos], s))
    return -EFAULT;
// ppos += s;
    return s;
    }
#[no_mangle]
unsafe extern "C" fn sfb_write(file: *mut file, buf: *const char __user, count: usize, ppos: *mut loff_t) -> isize {
    static ssize_t sfb_write(struct file *file, const char __user *buf, size_t count, loff_t *ppos)
    {
    int state;
    if (kstrtoint_from_user(buf, count, 10, &state))
    return -EFAULT;
    switch (state) {
    case 0: case 1:
    on_each_cpu(set_sfb_state, &state, 1);
    break;
    default:
    return -EINVAL;
    }
    return count;
    }
    static const struct file_operations sfb_fops = {
    .read = sfb_read,
    .write = sfb_write,
    .open = simple_open,
    .llseek = default_llseek
    };
pub const LDSTORDER_NLD_NST: c_uint = 0x0 /* 000 = No Load No Store */;
pub const LDSTORDER_ALD_NST: c_uint = 0x1 /* 001 = All Load No Store */;
pub const LDSTORDER_SLD_NST: c_uint = 0x3 /* 011 = Same Load No Store */;
pub const LDSTORDER_NLD_AST: c_uint = 0x4 /* 100 = No Load All Store */;
pub const LDSTORDER_ALD_AST: c_uint = 0x5 /* 101 = All Load All Store */;
pub const LDSTORDER_SLD_AST: c_uint = 0x7 /* 111 = Same Load All Store */;
    static char *tso_hints[] = {
    "No Load No Store",
    "All Load No Store",
    "Invalid Config",
    "Same Load No Store",
    "No Load All Store",
    "All Load All Store",
    "Invalid Config",
    "Same Load All Store"
    };
#[no_mangle]
unsafe extern "C" fn set_tso_state(info: *mut c_void) {
    static void set_tso_state(void *info)
    {
    let mut val: c_int = *(int *)info << CSR_LDSTORDER_SHIFT;
    csr_xchg32(val, CSR_LDSTORDER_MASK, LOONGARCH_CSR_IMPCTL1);
    }
#[no_mangle]
unsafe extern "C" fn tso_read(file: *mut file, buf: *mut char __user, count: usize, ppos: *mut loff_t) -> isize {
    static ssize_t tso_read(struct file *file, char __user *buf, size_t count, loff_t *ppos)
    {
    int s, state;
    char str[240];
    state = (csr_read32(LOONGARCH_CSR_IMPCTL1) & CSR_LDSTORDER_MASK) >> CSR_LDSTORDER_SHIFT;
    s = snprintf(str, sizeof(str), "Boot State: %d (%s)\n"
    "Current State: %d (%s)\n\n"
    "Available States:\n"
    "0 (%s)\t" "1 (%s)\t" "3 (%s)\n"
    "4 (%s)\t" "5 (%s)\t" "7 (%s)\n",
    tso_state, tso_hints[tso_state], state, tso_hints[state],
    tso_hints[0], tso_hints[1], tso_hints[3], tso_hints[4], tso_hints[5], tso_hints[7]);
    if (*ppos >= s)
    return 0;
    s -= *ppos;
    s = min_t(u32, s, count);
    if (copy_to_user(buf, &str[*ppos], s))
    return -EFAULT;
// ppos += s;
    return s;
    }
#[no_mangle]
unsafe extern "C" fn tso_write(file: *mut file, buf: *const char __user, count: usize, ppos: *mut loff_t) -> isize {
    static ssize_t tso_write(struct file *file, const char __user *buf, size_t count, loff_t *ppos)
    {
    int state;
    if (kstrtoint_from_user(buf, count, 10, &state))
    return -EFAULT;
    switch (state) {
    case 0: case 1: case 3:
    case 4: case 5: case 7:
    on_each_cpu(set_tso_state, &state, 1);
    break;
    default:
    return -EINVAL;
    }
    return count;
    }
    static const struct file_operations tso_fops = {
    .read = tso_read,
    .write = tso_write,
    .open = simple_open,
    .llseek = default_llseek
    };
#[no_mangle]
unsafe extern "C" fn arch_kdebugfs_init() -> int __init {
    static int __init arch_kdebugfs_init(void)
    {
    let mut config: c_uint = read_cpucfg(LOONGARCH_CPUCFG3);
    arch_debugfs_dir = debugfs_create_dir("loongarch", core::ptr::null_mut());
    if (config & CPUCFG3_SFB) {
    debugfs_create_file("sfb_state", S_IRUGO | S_IWUSR,
    arch_debugfs_dir, &sfb_state, &sfb_fops);
    sfb_state = (csr_read32(LOONGARCH_CSR_IMPCTL1) & CSR_STFILL) >> CSR_STFILL_SHIFT;
    }
    if (config & (CPUCFG3_ALDORDER_CAP | CPUCFG3_ASTORDER_CAP)) {
    debugfs_create_file("tso_state", S_IRUGO | S_IWUSR,
    arch_debugfs_dir, &tso_state, &tso_fops);
    tso_state = (csr_read32(LOONGARCH_CSR_IMPCTL1) & CSR_LDSTORDER_MASK) >> CSR_LDSTORDER_SHIFT;
    }
    return 0;
    }
    postcore_initcall(arch_kdebugfs_init);
