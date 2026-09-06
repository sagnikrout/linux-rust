//! Automatically rewritten from C to Rust
//! Source: kernel/module/tracking.c
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
// Module taint unload tracking support
//
// Copyright (C) 2022 Aaron Tomlin
//

    static LIST_HEAD(unloaded_tainted_modules);
    extern struct dentry *mod_debugfs_root;
#[no_mangle]
pub unsafe extern "C" fn try_add_tainted_module(mod: *mut module) -> c_int {
    int try_add_tainted_module(struct module *mod)
    {
    struct mod_unload_taint *mod_taint;
    if (!mod.taints)
    goto out;
    list_for_each_entry_rcu(mod_taint, &unloaded_tainted_modules, list,
    lockdep_is_held(&module_mutex)) {
    if (!strcmp(mod_taint.name, mod.name) &&
    mod_taint.taints & mod.taints) {
    mod_taint.count++;
    goto out;
    }
    }
    mod_taint = kmalloc_obj(*mod_taint);
    if (unlikely(!mod_taint))
    return -ENOMEM;
    strscpy(mod_taint.name, mod.name, MODULE_NAME_LEN);
    mod_taint.taints = mod.taints;
    list_add_rcu(&mod_taint.list, &unloaded_tainted_modules);
    mod_taint.count = 1;
    out:
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn print_unloaded_tainted_modules() {
    void print_unloaded_tainted_modules(void)
    {
    struct mod_unload_taint *mod_taint;
    char buf[MODULE_FLAGS_BUF_SIZE];
    if (!list_empty(&unloaded_tainted_modules)) {
    printk(KERN_DEFAULT "Unloaded tainted modules:");
    list_for_each_entry_rcu(mod_taint, &unloaded_tainted_modules,
    list) {
    size_t l;
    l = module_flags_taint(mod_taint.taints, buf);
    buf[l++] = '\0';
    pr_cont(" %s(%s):%llu", mod_taint.name, buf,
    mod_taint.count);
    }
    }
    }

    static void *unloaded_tainted_modules_seq_start(struct seq_file *m, loff_t *pos)
    __acquires(rcu)
    {
    rcu_read_lock();
    return seq_list_start_rcu(&unloaded_tainted_modules, *pos);
    }
    static void *unloaded_tainted_modules_seq_next(struct seq_file *m, void *p, loff_t *pos)
    {
    return seq_list_next_rcu(p, &unloaded_tainted_modules, pos);
    }
#[no_mangle]
unsafe extern "C" fn unloaded_tainted_modules_seq_stop(m: *mut seq_file, p: *mut c_void) {
    static void unloaded_tainted_modules_seq_stop(struct seq_file *m, void *p)
    __releases(rcu)
    {
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn unloaded_tainted_modules_seq_show(m: *mut seq_file, p: *mut c_void) -> c_int {
    static int unloaded_tainted_modules_seq_show(struct seq_file *m, void *p)
    {
    struct mod_unload_taint *mod_taint;
    char buf[MODULE_FLAGS_BUF_SIZE];
    size_t l;
    mod_taint = list_entry(p, struct mod_unload_taint, list);
    l = module_flags_taint(mod_taint.taints, buf);
    buf[l++] = '\0';
    seq_printf(m, "%s (%s) %llu", mod_taint.name, buf, mod_taint.count);
    seq_puts(m, "\n");
    return 0;
    }
    static const struct seq_operations unloaded_tainted_modules_seq_ops = {
    .start = unloaded_tainted_modules_seq_start,
    .next  = unloaded_tainted_modules_seq_next,
    .stop  = unloaded_tainted_modules_seq_stop,
    .show  = unloaded_tainted_modules_seq_show,
    };
#[no_mangle]
unsafe extern "C" fn unloaded_tainted_modules_open(inode: *mut inode, file: *mut file) -> c_int {
    static int unloaded_tainted_modules_open(struct inode *inode, struct file *file)
    {
    return seq_open(file, &unloaded_tainted_modules_seq_ops);
    }
    static const struct file_operations unloaded_tainted_modules_fops = {
    .open = unloaded_tainted_modules_open,
    .read = seq_read,
    .llseek = seq_lseek,
    .release = seq_release,
    };
#[no_mangle]
unsafe extern "C" fn unloaded_tainted_modules_init() -> int __init {
    static int __init unloaded_tainted_modules_init(void)
    {
    debugfs_create_file("unloaded_tainted", 0444, mod_debugfs_root, core::ptr::null_mut(),
    &unloaded_tainted_modules_fops);
    return 0;
    }
    module_init(unloaded_tainted_modules_init);
