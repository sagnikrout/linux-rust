//! Automatically rewritten from C to Rust
//! Source: kernel/module/procfs.c
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
// Module proc support
//
// Copyright (C) 2008 Alexey Dobriyan
//

#[no_mangle]
pub unsafe extern "C" fn print_unload_info(m: *mut seq_file, mod: *mut module) {
    static inline void print_unload_info(struct seq_file *m, struct module *mod)
    {
    struct module_use *use;
    let mut printed_something: c_int = 0;
    seq_printf(m, " %i ", module_refcount(mod));
//
// Always include a trailing , so userspace can differentiate
// between this and the old multi-field proc format.
//
    list_for_each_entry(use, &mod.source_list, source_list) {
    printed_something = 1;
    seq_printf(m, "%s,", use.source.name);
    }
    if (mod.init && !mod.exit) {
    printed_something = 1;
    seq_puts(m, "[permanent],");
    }
    if (!printed_something)
    seq_puts(m, "-");
    }

#[no_mangle]
pub unsafe extern "C" fn print_unload_info(m: *mut seq_file, mod: *mut module) {
    static inline void print_unload_info(struct seq_file *m, struct module *mod)
    {
// We don't know the usage count, or what modules are using.
    seq_puts(m, " - -");
    }

// Called by the /proc file system to return a list of modules.
    static void *m_start(struct seq_file *m, loff_t *pos)
    {
    mutex_lock(&module_mutex);
    return seq_list_start(&modules, *pos);
    }
    static void *m_next(struct seq_file *m, void *p, loff_t *pos)
    {
    return seq_list_next(p, &modules, pos);
    }
#[no_mangle]
unsafe extern "C" fn m_stop(m: *mut seq_file, p: *mut c_void) {
    static void m_stop(struct seq_file *m, void *p)
    {
    mutex_unlock(&module_mutex);
    }
#[no_mangle]
unsafe extern "C" fn module_total_size(mod: *mut module) -> c_uint {
    static unsigned int module_total_size(struct module *mod)
    {
    let mut size: c_uint = 0;
    for_each_mod_mem_type(type)
    size += mod.mem[type].size;
    return size;
    }
#[no_mangle]
unsafe extern "C" fn m_show(m: *mut seq_file, p: *mut c_void) -> c_int {
    static int m_show(struct seq_file *m, void *p)
    {
    struct module *mod = list_entry(p, struct module, list);
    char buf[MODULE_FLAGS_BUF_SIZE];
    void *value;
    unsigned int size;
// We always ignore unformed modules.
    if (mod.state == MODULE_STATE_UNFORMED)
    return 0;
    size = module_total_size(mod);
    seq_printf(m, "%s %u", mod.name, size);
    print_unload_info(m, mod);
// Informative for users.
    seq_printf(m, " %s",
    mod.state == MODULE_STATE_GOING ? "Unloading" :
    mod.state == MODULE_STATE_COMING ? "Loading" :
    "Live");
// Used by oprofile and other similar tools.
    value = m.private ? core::ptr::null_mut() : mod.mem[MOD_TEXT].base;
    seq_printf(m, " 0x%px", value);
// Taints info
    if (mod.taints)
    seq_printf(m, " %s", module_flags(mod, buf, true));
    seq_puts(m, "\n");
    return 0;
    }
//
// Format: modulename size refcount deps address
//
// Where refcount is a number or -, and deps is a comma-separated list
// of depends or -.
//
    static const struct seq_operations modules_op = {
    .start	= m_start,
    .next	= m_next,
    .stop	= m_stop,
    .show	= m_show
    };
//
// This also sets the "private" pointer to non-NULL if the
// kernel pointers should be hidden (so you can just test
// "m->private" to see if you should keep the values private).
//
// We use the same logic as for /proc/kallsyms.
//
#[no_mangle]
unsafe extern "C" fn modules_open(inode: *mut inode, file: *mut file) -> c_int {
    static int modules_open(struct inode *inode, struct file *file)
    {
    let mut err: c_int = seq_open(file, &modules_op);
    if (!err) {
    struct seq_file *m = file.private_data;
    m.private = kallsyms_show_value(file.f_cred) ? core::ptr::null_mut() : (void *)8ul;
    }
    return err;
    }
    static const struct proc_ops modules_proc_ops = {
    .proc_flags	= PROC_ENTRY_PERMANENT,
    .proc_open	= modules_open,
    .proc_read	= seq_read,
    .proc_lseek	= seq_lseek,
    .proc_release	= seq_release,
    };
#[no_mangle]
unsafe extern "C" fn proc_modules_init() -> int __init {
    static int __init proc_modules_init(void)
    {
    proc_create("modules", 0, core::ptr::null_mut(), &modules_proc_ops);
    return 0;
    }
    module_init(proc_modules_init);
