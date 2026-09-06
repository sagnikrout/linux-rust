//! Automatically rewritten from C to Rust
//! Source: lib/error-inject.c
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
// error-inject.c: Function-level error injection table

// Whitelist of symbols that can be overridden for error injection.
    static LIST_HEAD(error_injection_list);
    static DEFINE_MUTEX(ei_mutex);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ei_entry {
    pub list: list_head,
    pub start_addr: c_ulong,
    pub end_addr: c_ulong,
    pub etype: c_int,
    pub priv: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn within_error_injection_list(addr: c_ulong) -> bool {
    bool within_error_injection_list(unsigned long addr)
    {
    struct ei_entry *ent;
    let mut ret: bool = false;
    mutex_lock(&ei_mutex);
    list_for_each_entry(ent, &error_injection_list, list) {
    if (addr >= ent.start_addr && addr < ent.end_addr) {
    ret = true;
    break;
    }
    }
    mutex_unlock(&ei_mutex);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn get_injectable_error_type(addr: c_ulong) -> c_int {
    int get_injectable_error_type(unsigned long addr)
    {
    struct ei_entry *ent;
    let mut ei_type: c_int = -EINVAL;
    mutex_lock(&ei_mutex);
    list_for_each_entry(ent, &error_injection_list, list) {
    if (addr >= ent.start_addr && addr < ent.end_addr) {
    ei_type = ent.etype;
    break;
    }
    }
    mutex_unlock(&ei_mutex);
    return ei_type;
    }
//
// Lookup and populate the error_injection_list.
//
// For safety reasons we only allow certain functions to be overridden with
// bpf_error_injection, so we need to populate the list of the symbols that have
// been marked as safe for overriding.
//
    static void populate_error_injection_list(struct error_injection_entry *start,
    struct error_injection_entry *end,
    void *priv)
    {
    struct error_injection_entry *iter;
    struct ei_entry *ent;
    unsigned long entry, offset = 0, size = 0;
    mutex_lock(&ei_mutex);
    for (iter = start; iter < end; iter++) {
    entry = (unsigned long)dereference_symbol_descriptor((void *)iter.addr);
    if (!kernel_text_address(entry) ||
    !kallsyms_lookup_size_offset(entry, &size, &offset)) {
    pr_err("Failed to find error inject entry at %p\n",
    (void *)entry);
    continue;
    }
    ent = kmalloc_obj(*ent);
    if (!ent)
    break;
    ent.start_addr = entry;
    ent.end_addr = entry + size;
    ent.etype = iter.etype;
    ent.priv = priv;
    INIT_LIST_HEAD(&ent.list);
    list_add_tail(&ent.list, &error_injection_list);
    }
    mutex_unlock(&ei_mutex);
    }
// Markers of the _error_inject_whitelist section
    extern struct error_injection_entry __start_error_injection_whitelist[];
    extern struct error_injection_entry __stop_error_injection_whitelist[];
#[no_mangle]
unsafe extern "C" fn populate_kernel_ei_list() -> void __init {
    static void __init populate_kernel_ei_list(void)
    {
    populate_error_injection_list(__start_error_injection_whitelist,
    __stop_error_injection_whitelist,
    core::ptr::null_mut());
    }

#[no_mangle]
unsafe extern "C" fn module_load_ei_list(mod: *mut module) {
    static void module_load_ei_list(struct module *mod)
    {
    if (!mod.num_ei_funcs)
    return;
    populate_error_injection_list(mod.ei_funcs,
    mod.ei_funcs + mod.num_ei_funcs, mod);
    }
#[no_mangle]
unsafe extern "C" fn module_unload_ei_list(mod: *mut module) {
    static void module_unload_ei_list(struct module *mod)
    {
    struct ei_entry *ent, *n;
    if (!mod.num_ei_funcs)
    return;
    mutex_lock(&ei_mutex);
    list_for_each_entry_safe(ent, n, &error_injection_list, list) {
    if (ent.priv == mod) {
    list_del_init(&ent.list);
    kfree(ent);
    }
    }
    mutex_unlock(&ei_mutex);
    }
// Module notifier call back, checking error injection table on the module
    static int ei_module_callback(struct notifier_block *nb,
    unsigned long val, void *data)
    {
    struct module *mod = data;
    if (val == MODULE_STATE_COMING)
    module_load_ei_list(mod);
#[no_mangle]
pub unsafe extern "C" fn if(MODULE_STATE_GOING: val ==) -> else {
    else if (val == MODULE_STATE_GOING)
    module_unload_ei_list(mod);
    return NOTIFY_DONE;
    }
    static struct notifier_block ei_module_nb = {
    .notifier_call = ei_module_callback,
    .priority = 0
    };
#[no_mangle]
unsafe extern "C" fn module_ei_init() -> __init int {
    static __init int module_ei_init(void)
    {
    return register_module_notifier(&ei_module_nb);
    }

//
// error_injection/whitelist -- shows which functions can be overridden for
// error injection.
//
    static void *ei_seq_start(struct seq_file *m, loff_t *pos)
    {
    mutex_lock(&ei_mutex);
    return seq_list_start(&error_injection_list, *pos);
    }
#[no_mangle]
unsafe extern "C" fn ei_seq_stop(m: *mut seq_file, v: *mut c_void) {
    static void ei_seq_stop(struct seq_file *m, void *v)
    {
    mutex_unlock(&ei_mutex);
    }
    static void *ei_seq_next(struct seq_file *m, void *v, loff_t *pos)
    {
    return seq_list_next(v, &error_injection_list, pos);
    }
    static const char *error_type_string(int etype)
    {
    switch (etype) {
    case EI_ETYPE_NULL:
    return "core::ptr::null_mut()";
    case EI_ETYPE_ERRNO:
    return "ERRNO";
    case EI_ETYPE_ERRNO_NULL:
    return "ERRNO_NULL";
    case EI_ETYPE_TRUE:
    return "TRUE";
    default:
    return "(unknown)";
    }
    }
#[no_mangle]
unsafe extern "C" fn ei_seq_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int ei_seq_show(struct seq_file *m, void *v)
    {
    struct ei_entry *ent = list_entry(v, struct ei_entry, list);
    seq_printf(m, "%ps\t%s\n", (void *)ent.start_addr,
    error_type_string(ent.etype));
    return 0;
    }
    static const struct seq_operations ei_sops = {
    .start = ei_seq_start,
    .next  = ei_seq_next,
    .stop  = ei_seq_stop,
    .show  = ei_seq_show,
    };
    DEFINE_SEQ_ATTRIBUTE(ei);
#[no_mangle]
unsafe extern "C" fn ei_debugfs_init() -> int __init {
    static int __init ei_debugfs_init(void)
    {
    struct dentry *dir, *file;
    dir = debugfs_create_dir("error_injection", core::ptr::null_mut());
    file = debugfs_create_file("list", 0444, dir, core::ptr::null_mut(), &ei_fops);
    if (IS_ERR(file)) {
    debugfs_remove(dir);
    return PTR_ERR(file);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_error_injection() -> int __init {
    static int __init init_error_injection(void)
    {
    populate_kernel_ei_list();
    if (!module_ei_init())
    ei_debugfs_init();
    return 0;
    }
    late_initcall(init_error_injection);
