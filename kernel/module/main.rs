//! Automatically rewritten from C to Rust
//! Source: kernel/module/main.c
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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2002 Richard Henderson
// Copyright (C) 2001 Rusty Russell, 2002, 2010 Rusty Russell IBM.
// Copyright (C) 2023 Luis Chamberlain <mcgrof@kernel.org>
//
// Macro flag: #define INCLUDE_VERMAGIC

// Macro flag: #define CREATE_TRACE_POINTS

//
// Mutex protects:
// 1) List of modules (also safely readable within RCU read section),
// 2) module_use links,
// 3) mod_tree.addr_min/mod_tree.addr_max.
// (delete and add uses RCU list operations).
//
pub static mut module_mutex: usize = 0;
pub static mut modules: usize = 0;
// Work queue for freeing init sections in success case
// forward_decl: do_free_init;
pub static mut init_free_wq: usize = 0;
pub static mut init_free_list: usize = 0;
    struct mod_tree_root mod_tree __cacheline_aligned = {
    .addr_min = -1UL,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct symsearch {
    pub stop: *const *const kernel_symbol start,,
    pub crcs: *const u32,
    pub flagstab: *const u8,
}

//
// Bounds of module memory, for speeding up __module_address.
// Protected by module_mutex.
//
#[no_mangle]
pub unsafe extern "C" fn __mod_update_bounds(__maybe_unused: mod_mem_type type, base: *mut c_void, size: c_uint, tree: *mut mod_tree_root) {
pub static mut min: c_ulong = 0;
pub static mut max: c_ulong = 0;

    if (mod_mem_type_is_core_data(type)) {
    if (min < tree.data_addr_min) {
    tree.data_addr_min = min;
    }
    if (max > tree.data_addr_max) {
    tree.data_addr_max = max;
    }
    return;
    }

    if (min < tree.addr_min) {
    tree.addr_min = min;
    }
    if (max > tree.addr_max) {
    tree.addr_max = max;
    }
    }
#[no_mangle]
unsafe extern "C" fn mod_update_bounds(mod: *mut module) {
    for_each_mod_mem_type(type) {
    let mut mod_mem = &mod.mem[type];
    if (mod_mem.size) {
    __mod_update_bounds(type, mod_mem.base, mod_mem.size, &mod_tree);
    }
    }
    }
// Block module loading/unloading?
    static int modules_disabled;
    core_param!(nomodule, modules_disabled, bint, 0);
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn init_module_sysctl() -> c_int {
    register_sysctl_init("kernel", module_sysctl_table);
    return 0;
    }
    subsys_initcall!(init_module_sysctl);
// Waiting for a module to finish initializing?
pub static mut module_wq: usize = 0;
    static BLOCKING_NOTIFIER_HEAD(module_notify_list);
#[no_mangle]
pub unsafe extern "C" fn register_module_notifier(nb: *mut notifier_block) -> c_int {
    return blocking_notifier_chain_register(&module_notify_list, nb);
    }
    EXPORT_SYMBOL(register_module_notifier);
#[no_mangle]
pub unsafe extern "C" fn unregister_module_notifier(nb: *mut notifier_block) -> c_int {
    return blocking_notifier_chain_unregister(&module_notify_list, nb);
    }
    EXPORT_SYMBOL(unregister_module_notifier);
//
// We require a truly strong try_module_get(): 0 means success.
// Otherwise an error is returned due to ongoing or failed
// initialization etc.
//
#[no_mangle]
pub unsafe extern "C" fn strong_try_module_get(mod: *mut module) -> c_int {
    BUG_ON!(mod && mod.state == MODULE_STATE_UNFORMED);
    if (mod && mod.state == MODULE_STATE_COMING) {
    return -EBUSY;
    }
    if (try_module_get(mod)) {
    return 0;
    }
    else {
    return -ENOENT;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn add_taint_module(mod: *mut module, flag: c_uint, lockdep_ok: lockdep_ok) {
    add_taint(flag, lockdep_ok);
    set_bit(flag, &mod.taints);
    }
//
// Like strncmp(), except s/-/_/g as per scripts/Makefile.lib:name-fix-token rule.
//
#[no_mangle]
unsafe extern "C" fn mod_strncmp(str_a: *const c_char, str_b: *const c_char, n: usize) -> c_int {
    while (i < n) {
pub static mut a: c_char = 0;
pub static mut b: c_char = 0;
    let mut d = 0;
    if (a == '-') a = '_'; {
    if (b == '-') b = '_';
    }
    d = a - b;
    if (d) {
    return d;
    }
    if (!a) {
    break;
    }
    }
    return 0;
    }
//
// A thread that wants to hold a reference to a module only while it
// is running can call this to safely exit.
//
#[no_mangle]
pub unsafe extern "C" fn __module_put_and_kthread_exit(mod: *mut module, code: c_long) -> void __noreturn {
    module_put!(mod);
    kthread_exit(code);
    }
    EXPORT_SYMBOL(__module_put_and_kthread_exit);
// Find a module section: 0 means not found.
#[no_mangle]
unsafe extern "C" fn find_sec(info: *const load_info, name: *const c_char) -> c_uint {
    let mut i = 0;
    while (i < info.hdr.e_shnum) {
    let mut shdr = &info.sechdrs[i];
// Alloc bit cleared means "ignore it."
    if ((shdr.sh_flags & SHF_ALLOC)
    && strcmp(info.secstrings + shdr.sh_name, name) == 0) {
    return i;
    }
    }
    return 0;
    }
//
// find_any_unique_sec() - Find a unique section index by name
// @info: Load info for the module to scan
// @name: Name of the section we're looking for
//
// Locates a unique section by name. Ignores SHF_ALLOC.
//
// Return: Section index if found uniquely, zero if absent, negative count
// of total instances if multiple were found.
//
#[no_mangle]
unsafe extern "C" fn find_any_unique_sec(info: *const load_info, name: *const c_char) -> c_int {
    let mut idx = 0;
pub static mut count: c_uint = 0;
    let mut i = 0;
    while (i < info.hdr.e_shnum) {
    if (strcmp(info.secstrings + info.sechdrs[i].sh_name,
    name) == 0) {
    count += 1;
    idx = i;
    }
    }
    if (count == 1) {
    return idx;
    } else if (count == 0) {
    return 0;
    } else {
    return -count;
    }
    }
// Find a module section, or NULL.
#[no_mangle]
pub unsafe extern "C" fn section_addr(info: *mut load_info, name: *mut c_char) -> *mut c_void {
// Section 0 has sh_addr 0.
    return info.sechdrs[find_sec(info, name)].sh_addr;
    }
// Find a module section, or NULL.  Fill in number of "objects" in section.
#[no_mangle]
pub unsafe extern "C" fn section_objs(info: *mut load_info, name: *mut c_char, object_size: size_t, num: *mut c_uint) -> *mut c_void {
pub static mut sec: c_uint = 0;
// Section 0 has sh_addr 0 and sh_size 0.
// num = info->sechdrs[sec].sh_size / object_size;
    return info.sechdrs[sec].sh_addr;
    }
// Find a module section: 0 means not found. Ignores SHF_ALLOC flag.
#[no_mangle]
unsafe extern "C" fn find_any_sec(info: *const load_info, name: *const c_char) -> c_uint {
    let mut i = 0;
    while (i < info.hdr.e_shnum) {
    let mut shdr = &info.sechdrs[i];
    if (strcmp(info.secstrings + shdr.sh_name, name) == 0) {
    return i;
    }
    }
    return 0;
    }
//
// Find a module section, or NULL. Fill in number of "objects" in section.
// Ignores SHF_ALLOC flag.
//
    static __maybe_unused void *any_section_objs(const struct load_info *info,
    const char *name,
    size_t object_size,
    unsigned int *num)
    {
pub static mut sec: c_uint = 0;
// Section 0 has sh_addr 0 and sh_size 0.
// num = info->sechdrs[sec].sh_size / object_size;
    return info.sechdrs[sec].sh_addr;
    }

    static const char *kernel_symbol_name(const struct kernel_symbol *sym)
    {

    return offset_to_ptr(&sym.name_offset);

    return sym.name;

    }
    static const char *kernel_symbol_namespace(const struct kernel_symbol *sym)
    {

    if (!sym.namespace_offset) {
    return core::ptr::null_mut();
    }
    return offset_to_ptr(&sym.namespace_offset);

    return sym.namespace;

    }
#[no_mangle]
pub unsafe extern "C" fn cmp_name(name: *const c_void, sym: *const c_void) -> c_int {
    return strcmp(name, kernel_symbol_name(sym));
    }
#[no_mangle]
pub unsafe extern "C" fn find_exported_symbol_in_section(syms: *mut symsearch, owner: *mut module, fsa: *mut find_symbol_arg) -> bool {
pub static mut sym: *mut c_void = core::ptr::null_mut();
    let mut sym_flags = 0;
    sym = bsearch(fsa.name, syms.start, syms.stop - syms.start,
    sizeof!(kernel_symbol), cmp_name);
    if (!sym) {
    return false;
    }
    sym_flags = *(syms.flagstab + (sym - syms.start));
    if (!fsa.gplok && (sym_flags & KSYM_FLAG_GPL_ONLY)) {
    return false;
    }
    fsa.owner = owner;
    fsa.crc = symversion(syms.crcs, sym - syms.start);
    fsa.sym = sym;
    fsa.license = (sym_flags & KSYM_FLAG_GPL_ONLY) ? GPL_ONLY : NOT_GPL_ONLY;
    return true;
    }
//
// Find an exported symbol and return it, along with, (optional) crc and
// (optional) module which owns it. Needs RCU or module_mutex.
//
#[no_mangle]
pub unsafe extern "C" fn find_symbol(fsa: *mut find_symbol_arg) -> bool {
pub static mut symsearch: usize = 0;
pub static mut mod: *mut c_void = core::ptr::null_mut();
    if (find_exported_symbol_in_section(&syms, core::ptr::null_mut(), fsa)) {
    return true;
    }
    list_for_each_entry_rcu(mod, &modules, list,
    lockdep_is_held(&module_mutex)) {
pub static mut symsearch: usize = 0;
    if (mod.state == MODULE_STATE_UNFORMED) {
    continue;
    }
    if (find_exported_symbol_in_section(&syms, mod, fsa)) {
    return true;
    }
    }
    pr_debug!("Failed to find symbol %s\n", fsa.name);
    return false;
    }
//
// Search for module by name: must hold module_mutex!(or RCU for read-only
// access).
//
#[no_mangle]
pub unsafe extern "C" fn find_module_all(name: *mut c_char, len: size_t, even_unformed: bool) -> *mut c_void {
pub static mut mod: *mut c_void = core::ptr::null_mut();
    list_for_each_entry_rcu(mod, &modules, list,
    lockdep_is_held(&module_mutex)) {
    if (!even_unformed && mod.state == MODULE_STATE_UNFORMED) {
    continue;
    }
    if (strlen(mod.name) == len && !memcmp(mod.name, name, len)) {
    return mod;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn find_module(name: *mut c_char) -> *mut c_void {
    return find_module_all(name, strlen(name), false);
    }

    static inline void  *mod_percpu(module *mod)
    {
    return mod.percpu;
    }
#[no_mangle]
unsafe extern "C" fn percpu_modalloc(mod: *mut module, info: *mut load_info) -> c_int {
    let mut pcpusec = &info.sechdrs[info.index.pcpu];
pub static mut align: c_ulong = 0;
    if (!pcpusec.sh_size) {
    return 0;
    }
    if (align > PAGE_SIZE) {
    pr_warn!("%s: per-cpu alignment %li > %li\n",
    mod.name, align, PAGE_SIZE);
    align = PAGE_SIZE;
    }
    mod.percpu = __alloc_reserved_percpu(pcpusec.sh_size, align);
    if (!mod.percpu) {
    pr_warn!("%s: Could not allocate %lu bytes percpu data\n",
    mod.name, (unsigned long)pcpusec.sh_size);
    return -ENOMEM;
    }
    mod.percpu_size = pcpusec.sh_size;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn percpu_modfree(mod: *mut module) {
    free_percpu(mod.percpu);
    }
#[no_mangle]
unsafe extern "C" fn find_pcpusec(info: *mut load_info) -> c_uint {
    return find_sec(info, ".data..percpu");
    }
#[no_mangle]
pub unsafe extern "C" fn percpu_modcopy(mod: *mut module, from: *mut c_void, size: c_ulong) {
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    memcpy(per_cpu_ptr(mod.percpu, cpu), from, size);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __is_module_percpu_address(addr: c_ulong, can_addr: *mut c_ulong) -> bool {
pub static mut mod: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    guard(rcu)();
    list_for_each_entry_rcu(mod, &modules, list) {
    if (mod.state == MODULE_STATE_UNFORMED) {
    continue;
    }
    if (!mod.percpu_size) {
    continue;
    }
    for_each_possible_cpu(cpu) {
    let mut start = per_cpu_ptr(mod.percpu, cpu);
    let mut va = addr;
    if (va >= start && va < start + mod.percpu_size) {
    if (can_addr) {
// can_addr = (unsigned long) (va - start);
// can_addr += (unsigned long)
    per_cpu_ptr(mod.percpu,
    get_boot_cpu_id());
    }
    return true;
    }
    }
    }
    return false;
    }
//
// is_module_percpu_address() - test whether address is from module static percpu
// @addr: address to test
//
// Test whether @addr belongs to module static percpu area.
//
// Return: %true if @addr is from module static percpu area
//
#[no_mangle]
pub unsafe extern "C" fn is_module_percpu_address(addr: c_ulong) -> bool {
    return __is_module_percpu_address(addr, core::ptr::null_mut());
    }

    static inline void  *mod_percpu(module *mod)
    {
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn percpu_modalloc(mod: *mut module, info: *mut load_info) -> c_int {
// UP modules shouldn't have this section: ENOMEM isn't quite right
    if (info.sechdrs[info.index.pcpu].sh_size != 0) {
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn percpu_modfree(mod: *mut module) {
    }
#[no_mangle]
unsafe extern "C" fn find_pcpusec(info: *mut load_info) -> c_uint {
    return 0;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: percpu_modcopy
pub unsafe extern "C" fn percpu_modcopy_dup(mod: *mut module, from: *mut c_void, size: c_ulong) {
// pcpusec should be 0, and size of that section should be 0.
    BUG_ON!(size != 0);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: is_module_percpu_address
pub unsafe extern "C" fn is_module_percpu_address_dup(addr: c_ulong) -> bool {
    return false;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: __is_module_percpu_address
pub unsafe extern "C" fn __is_module_percpu_address_dup(addr: c_ulong, can_addr: *mut c_ulong) -> bool {
    return false;
    }

    static void setup_modinfo_##field(module *mod, const char *s)  
    {                                                                     
    mod.field = kstrdup(s, GFP_KERNEL);                          
    }                                                                     
    static ssize_t show_modinfo_##field(const struct module_attribute *mattr, module_kobject *mk, char *buffer)      
    {                                                                     
    return scnprintf(buffer, PAGE_SIZE, "%s\n", mk.mod.field);  
    }                                                                     
    static int modinfo_##field##_exists(module *mod)               
    {                                                                     
    return mod.field != core::ptr::null_mut();                                    
    }                                                                     
    static void free_modinfo_##field(module *mod)                  
    {                                                                     
    kfree(mod.field);                                            
    mod.field = core::ptr::null_mut();                                            
    }                                                                     
    static const struct module_attribute modinfo_##field = {              
    .attr = { .name = __stringify(field), .mode = 0444 },         
    .show = show_modinfo_##field,                                 
    .setup = setup_modinfo_##field,                               
    .test = modinfo_##field##_exists,                             
    .free = free_modinfo_##field,                                 
    };
    MODINFO_ATTR(version);
    MODINFO_ATTR(srcversion);
#[no_mangle]
unsafe extern "C" fn setup_modinfo_import_ns(mod: *mut module, s: *const c_char) {
    mod.imported_namespaces = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn show_modinfo_import_ns(mattr: *mut module_attribute, mk: *mut module_kobject, buffer: *mut c_char) -> ssize_t {
    return sysfs_emit(buffer, "%s\n", mk.mod.imported_namespaces);
    }
#[no_mangle]
unsafe extern "C" fn modinfo_import_ns_exists(mod: *mut module) -> c_int {
    return mod.imported_namespaces != core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn free_modinfo_import_ns(mod: *mut module) {
    kfree(mod.imported_namespaces);
    mod.imported_namespaces = core::ptr::null_mut();
    }
pub static mut module_attribute: usize = 0;
    static struct {
    char name[MODULE_NAME_LEN];
    char taints[MODULE_FLAGS_BUF_SIZE];
    } last_unloaded_module;

    EXPORT_TRACEPOINT_SYMBOL(module_get);
// MODULE_REF_BASE is the base reference count by kmodule loader.
pub const MODULE_REF_BASE: c_int = 1;
// Init the unload section of the module.
#[no_mangle]
unsafe extern "C" fn module_unload_init!(mod: *mut module) -> c_int {
//
// Initialize reference counter to MODULE_REF_BASE.
// refcnt == 0 means module is going.
//
    atomic_set(&mod.refcnt, MODULE_REF_BASE);
    INIT_LIST_HEAD(&mod.source_list);
    INIT_LIST_HEAD(&mod.target_list);
// Hold reference count during initialization.
    atomic_inc(&mod.refcnt);
    return 0;
    }
// Does a already use b?
#[no_mangle]
unsafe extern "C" fn already_uses(a: *mut module, b: *mut module) -> c_int {
pub static mut use: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(use, &b.source_list, source_list) {
    if (use.source == a) {
    return 1;
    }
    }
    pr_debug!("%s does not use %s!\n", a.name, b.name);
    return 0;
    }
//
// Module a uses b
// - we add 'a' as a "source", 'b' as a "target" of module use
// - the module_use is added to the list of 'b' sources (so
// 'b' can walk the list to see who sourced them), and of 'a'
// targets (so 'a' can see what modules it targets).
//
#[no_mangle]
unsafe extern "C" fn add_module_usage(a: *mut module, b: *mut module) -> c_int {
pub static mut use: *mut c_void = core::ptr::null_mut();
    pr_debug!("Allocating new usage for %s.\n", a.name);
    use = kmalloc_obj(*use, GFP_ATOMIC);
    if (!use) {
    return -ENOMEM;
    }
    use.source = a;
    use.target = b;
    list_add(&use.source_list, &b.source_list);
    list_add(&use.target_list, &a.target_list);
    return 0;
    }
// Module a uses b: caller needs module_mutex!()
#[no_mangle]
unsafe extern "C" fn ref_module(a: *mut module, b: *mut module) -> c_int {
    let mut err = 0;
    if (b == core::ptr::null_mut() || already_uses(a, b)) {
    return 0;
    }
// If module isn't available, we fail.
    err = strong_try_module_get(b);
    if (err) {
    return err;
    }
    err = add_module_usage(a, b);
    if (err) {
    module_put!(b);
    return err;
    }
    return 0;
    }
// Clear the unload stuff of the module.
#[no_mangle]
unsafe extern "C" fn module_unload_free!(mod: *mut module) {
    let mut use = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    mutex_lock(&module_mutex);
    list_for_each_entry_safe(use, tmp, &mod.target_list, target_list) {
    let mut i = use.target;
    pr_debug!("%s unusing %s\n", mod.name, i.name);
    module_put!(i);
    list_del(&use.source_list);
    list_del(&use.target_list);
    kfree(use);
    }
    mutex_unlock(&module_mutex);
    }

#[no_mangle]
pub unsafe extern "C" fn try_force_unload(flags: c_uint) -> c_int {
pub static mut ret: c_int = 0;
    if (ret) {
    add_taint(TAINT_FORCED_RMMOD, LOCKDEP_NOW_UNRELIABLE);
    }
    return ret;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: try_force_unload
pub unsafe extern "C" fn try_force_unload_dup(flags: c_uint) -> c_int {
    return 0;
    }

// Try to release refcount of module, 0 means success.
#[no_mangle]
unsafe extern "C" fn try_release_module_ref(mod: *mut module) -> c_int {
    let mut ret = 0;
// Try to decrement refcnt which we set at loading
    ret = atomic_sub_return(MODULE_REF_BASE, &mod.refcnt);
    BUG_ON!(ret < 0);
    if (ret) {
// Someone can put this right now, recover with checking
    ret = atomic_add_unless(&mod.refcnt, MODULE_REF_BASE, 0);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn try_stop_module(mod: *mut module, flags: c_int, forced: *mut c_int) -> c_int {
// If it's not unused, quit unless we're forcing.
    if (try_release_module_ref(mod) != 0) {
// forced = try_force_unload(flags);
    if (!(*forced)) {
    return -EWOULDBLOCK;
    }
    }
// Mark it as dying.
    mod.state = MODULE_STATE_GOING;
    return 0;
    }
//
// module_refcount!() - return the refcount or -1 if unloading
// @mod:	the module we're checking
//
// Return:
// -1 if the module is in the process of unloading
// otherwise the number of references in the kernel to the module
//
#[no_mangle]
pub unsafe extern "C" fn module_refcount!(mod: *mut module) -> c_int {
    return atomic_read(&mod.refcnt) - MODULE_REF_BASE;
    }
    EXPORT_SYMBOL(module_refcount);
// This exists whether we can unload or not
// forward_decl: free_module;
#[no_mangle]
pub unsafe extern "C" fn sys_delete_module(name_user: usize, flags: usize) -> c_long {
pub static mut mod: *mut c_void = core::ptr::null_mut();
    char name[MODULE_NAME_LEN];
    char buf[MODULE_FLAGS_BUF_SIZE];
    int ret, len, forced = 0;
    if (!capable(CAP_SYS_MODULE) || modules_disabled) {
    return -EPERM;
    }
    len = strncpy_from_user(name, name_user, MODULE_NAME_LEN);
    if (len == 0 || len == MODULE_NAME_LEN) {
    return -ENOENT;
    }
    if (len < 0) {
    return len;
    }
    audit_log_kern_module(name);
    if (mutex_lock_interruptible(&module_mutex) != 0) {
    return -EINTR;
    }
    mod = find_module(name);
    if (!mod) {
    ret = -ENOENT;
// goto;
    }
    if (!list_empty(&mod.source_list)) {
// Other modules depend on us: get rid of them first.
    ret = -EWOULDBLOCK;
// goto;
    }
// Doing init or already dying?
    if (mod.state != MODULE_STATE_LIVE) {
// FIXME: if (force), slam module count damn the torpedoes
    pr_debug!("%s already dying\n", mod.name);
    ret = -EBUSY;
// goto;
    }
// If it has an init func, it must have an exit func to unload
    if (mod.init && !mod.exit) {
    forced = try_force_unload(flags);
    if (!forced) {
// This module can't be removed
    ret = -EBUSY;
// goto;
    }
    }
    ret = try_stop_module(mod, flags, &forced);
    if (ret != 0) {
// goto;
    }
    mutex_unlock(&module_mutex);
// Final destruction now no one is using it.
    if (mod.exit != core::ptr::null_mut()) {
    mod.exit();
    }
    blocking_notifier_call_chain(&module_notify_list,
    MODULE_STATE_GOING, mod);
    klp_module_going(mod);
    ftrace_release_mod(mod);
    async_synchronize_full();
// Store the name and taints of the last unloaded module for diagnostic purposes
    strscpy(last_unloaded_module.name, mod.name);
    strscpy(last_unloaded_module.taints, module_flags!(mod, buf, false));
    free_module(mod);
// someone could wait for the module in add_unformed_module()
    wake_up_all(&module_wq);
    return 0;
// label;
    mutex_unlock(&module_mutex);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __symbol_put(symbol: *const c_char) {
pub static mut find_symbol_arg: usize = 0;
    guard(rcu)();
    BUG_ON!(!find_symbol(&fsa));
    module_put!(fsa.owner);
    }
    EXPORT_SYMBOL(__symbol_put);
// Note this assumes addr is a function, which it currently always is.
#[no_mangle]
pub unsafe extern "C" fn symbol_put_addr(addr: *mut c_void) {
pub static mut modaddr: *mut c_void = core::ptr::null_mut();
pub static mut a: c_ulong = 0;
    if (core_kernel_text(a)) {
    return;
    }
//
// Even though we hold a reference on the module; we still need to
// RCU read section in order to safely traverse the data structure.
//
    guard(rcu)();
    modaddr = __module_text_address(a);
    BUG_ON!(!modaddr);
    module_put!(modaddr);
    }
    EXPORT_SYMBOL_GPL(symbol_put_addr);
#[no_mangle]
pub unsafe extern "C" fn show_refcnt(mattr: *mut module_attribute, mk: *mut module_kobject, buffer: *mut c_char) -> ssize_t {
    return sprintf(buffer, "%i\n", module_refcount!(mk.mod));
    }
    static const struct module_attribute modinfo_refcnt =
    __ATTR(refcnt, 0444, show_refcnt, core::ptr::null_mut());
#[no_mangle]
pub unsafe extern "C" fn __module_get(module: *mut module) {
    if (module) {
    atomic_inc(&module.refcnt);
    trace_module_get(module, _RET_IP_);
    }
    }
    EXPORT_SYMBOL(__module_get);
#[no_mangle]
pub unsafe extern "C" fn try_module_get(module: *mut module) -> bool {
pub static mut ret: bool = true;
    if (module) {
// Note: here, we can fail to get a reference
    if (likely(module_is_live!(module) &&
    atomic_inc_not_zero(&module.refcnt) != 0)) {
    trace_module_get(module, _RET_IP_);
    }
    else {
    ret = false;
    }
    }
    return ret;
    }
    EXPORT_SYMBOL(try_module_get);
#[no_mangle]
pub unsafe extern "C" fn module_put!(module: *mut module) {
    let mut ret = 0;
    if (module) {
    ret = atomic_dec_if_positive(&module.refcnt);
    WARN_ON!(ret < 0);	/* Failed to put refcount */
    trace_module_put(module, _RET_IP_);
    }
    }
    EXPORT_SYMBOL(module_put);

#[no_mangle]
pub unsafe extern "C" fn module_unload_free!(mod: *mut module) {
    }
#[no_mangle]
unsafe extern "C" fn ref_module(a: *mut module, b: *mut module) -> c_int {
    return strong_try_module_get(b);
    }
#[no_mangle]
pub unsafe extern "C" fn module_unload_init!(mod: *mut module) -> c_int {
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn module_flags_taint!(taints: c_ulong, buf: *mut c_char) -> usize {
pub static mut l: usize = 0;
    let mut i = 0;
    while (i < TAINT_FLAGS_COUNT) {
    if (test_bit(i, &taints)) {
    buf[l++] = taint_flags[i].c_true;
    }
    }
    return l;
    }
#[no_mangle]
pub unsafe extern "C" fn show_initstate(mattr: *mut module_attribute, mk: *mut module_kobject, buffer: *mut c_char) -> ssize_t {
    let mut state = "unknown";
    match (mk.mod.state) {
    MODULE_STATE_LIVE => {
    state = "live";
    // break;
    }
    MODULE_STATE_COMING => {
    state = "coming";
    // break;
    }
    MODULE_STATE_GOING => {
    state = "going";
    // break;
    }
    _ => {
    BUG();
    }
    }
    return sprintf(buffer, "%s\n", state);
    }
    static const struct module_attribute modinfo_initstate =
    __ATTR(initstate, 0444, show_initstate, core::ptr::null_mut());
#[no_mangle]
pub unsafe extern "C" fn store_uevent(mattr: *mut module_attribute, mk: *mut module_kobject, buffer: *mut c_char, count: size_t) -> ssize_t {
    let mut rc = 0;
    rc = kobject_synth_uevent(&mk.kobj, buffer, count);
    return rc ? rc : count;
    }
    const struct module_attribute module_uevent =
    __ATTR(uevent, 0200, core::ptr::null_mut(), store_uevent);
#[no_mangle]
pub unsafe extern "C" fn show_coresize(mattr: *mut module_attribute, mk: *mut module_kobject, buffer: *mut c_char) -> ssize_t {
pub static mut size: c_uint = 0;
    if (!IS_ENABLED!(CONFIG_ARCH_WANTS_MODULES_DATA_IN_VMALLOC)) {
    for_class_mod_mem_type(type, core_data)
    size += mk.mod.mem[type].size;
    }
    return sprintf(buffer, "%u\n", size);
    }
    static const struct module_attribute modinfo_coresize =
    __ATTR(coresize, 0444, show_coresize, core::ptr::null_mut());

#[no_mangle]
pub unsafe extern "C" fn show_datasize(mattr: *mut module_attribute, mk: *mut module_kobject, buffer: *mut c_char) -> ssize_t {
pub static mut size: c_uint = 0;
    for_class_mod_mem_type(type, core_data)
    size += mk.mod.mem[type].size;
    return sprintf(buffer, "%u\n", size);
    }
    static const struct module_attribute modinfo_datasize =
    __ATTR(datasize, 0444, show_datasize, core::ptr::null_mut());

#[no_mangle]
pub unsafe extern "C" fn show_initsize(mattr: *mut module_attribute, mk: *mut module_kobject, buffer: *mut c_char) -> ssize_t {
pub static mut size: c_uint = 0;
    for_class_mod_mem_type(type, init)
    size += mk.mod.mem[type].size;
    return sprintf(buffer, "%u\n", size);
    }
    static const struct module_attribute modinfo_initsize =
    __ATTR(initsize, 0444, show_initsize, core::ptr::null_mut());
#[no_mangle]
pub unsafe extern "C" fn show_taint(mattr: *mut module_attribute, mk: *mut module_kobject, buffer: *mut c_char) -> ssize_t {
    let mut l = 0;
    l = module_flags_taint!(mk.mod.taints, buffer);
    buffer[l++] = '\n';
    return l;
    }
    static const struct module_attribute modinfo_taint =
    __ATTR(taint, 0444, show_taint, core::ptr::null_mut());
    const struct module_attribute *const modinfo_attrs[] = {
    &module_uevent,
    &modinfo_version,
    &modinfo_srcversion,
    &modinfo_import_ns,
    &modinfo_initstate,
    &modinfo_coresize,

    &modinfo_datasize,

    &modinfo_initsize,
    &modinfo_taint,

    &modinfo_refcnt,

    core::ptr::null_mut(),
    };
pub static mut modinfo_attrs_count: usize = 0;
    static const char vermagic[] = VERMAGIC_STRING;
#[no_mangle]
pub unsafe extern "C" fn try_to_force_load(mod: *mut module, reason: *const c_char) -> c_int {

    if (!test_taint(TAINT_FORCED_MODULE)) {
    pr_warn!("%s: %s: kernel tainted.\n", mod.name, reason);
    }
    add_taint_module(mod, TAINT_FORCED_MODULE, LOCKDEP_NOW_UNRELIABLE);
    return 0;

    return -ENOEXEC;

    }
// Parse tag=value strings from .modinfo section
    char *module_next_tag_pair!(char *string, unsigned long *secsize)
    {
// Skip non-zero chars
    while (string[0]) {
    string += 1;
    if ((*secsize)-- <= 1) {
    return core::ptr::null_mut();
    }
    }
// Skip any zero padding.
    while (!string[0]) {
    string += 1;
    if ((*secsize)-- <= 1) {
    return core::ptr::null_mut();
    }
    }
    return string;
    }
#[no_mangle]
pub unsafe extern "C" fn get_next_modinfo(info: *mut load_info, tag: *mut c_char, prev: *mut c_char) -> *mut c_void {
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut taglen: c_uint = 0;
    let mut infosec = &info.sechdrs[info.index.info];
pub static mut size: c_ulong = 0;
//
// get_modinfo() calls made before rewrite_section_headers()
// must use sh_offset, as sh_addr isn't set!
//
    let mut modinfo = info.hdr + infosec.sh_offset;
    if (prev) {
    size -= prev - modinfo;
    modinfo = module_next_tag_pair!(prev, &size);
    }
    for (p = modinfo; p; p = module_next_tag_pair!(p, &size)) {
    if (strncmp(p, tag, taglen) == 0 && p[taglen] == '=') {
    return p + taglen + 1;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn get_modinfo(info: *mut load_info, tag: *mut c_char) -> *mut c_void {
    return get_next_modinfo(info, tag, core::ptr::null_mut());
    }
//
// verify_module_namespace() - does @modname have access to this symbol's @namespace
// @namespace: export symbol namespace
// @modname: module name
//
// If @namespace is prefixed with "module:" to indicate it is a module namespace
// then test if @modname matches any of the comma separated patterns.
//
// The patterns only support tail-glob.
//
#[no_mangle]
unsafe extern "C" fn verify_module_namespace(namespace: *const c_char, modname: *const c_char) -> bool {
    size_t len, modlen = strlen(modname);
    let mut prefix = "module:";
pub static mut sep: *mut c_void = core::ptr::null_mut();
    let mut glob = 0;
    if (!strstarts(namespace, prefix)) {
    return false;
    }
    while (*namespace) {
    sep = strchrnul(namespace, ',');
    len = sep - namespace;
    glob = false;
    if (sep[-1] == '*') {
    len -= 1;
    glob = true;
    }
    if (*sep) {
    sep += 1;
    }
    if (mod_strncmp(namespace, modname, len) == 0 && (glob || len == modlen)) {
    return true;
    }
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn verify_namespace_is_imported(info: *mut load_info, sym: *mut kernel_symbol, mod: *mut module) -> c_int {
pub static mut namespace: *mut c_void = core::ptr::null_mut();
pub static mut imported_namespace: *mut c_void = core::ptr::null_mut();
    namespace = kernel_symbol_namespace(sym);
    if (namespace && namespace[0]) {
    if (verify_module_namespace(namespace, mod.name)) {
    return 0;
    }
    for_each_modinfo_entry(imported_namespace, info, "import_ns") {
    if (strcmp(namespace, imported_namespace) == 0) {
    return 0;
    }
    }

    pr_warn!(

    pr_err!(

    "%s: module uses symbol (%s) from namespace %s, but does not import it.\n",
    mod.name, kernel_symbol_name(sym), namespace);

    return -EINVAL;

    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn inherit_taint(mod: *mut module, owner: *mut module, name: *const c_char) -> bool {
    if (!owner || !test_bit(TAINT_PROPRIETARY_MODULE, &owner.taints)) {
    return true;
    }
    if (mod.using_gplonly_symbols) {
    pr_err!("%s: module using GPL-only symbols uses symbols %s from proprietary module %s.\n",
    mod.name, name, owner.name);
    return false;
    }
    if (!test_bit(TAINT_PROPRIETARY_MODULE, &mod.taints)) {
    pr_warn!("%s: module uses symbols %s from proprietary module %s, inheriting taint.\n",
    mod.name, name, owner.name);
    set_bit(TAINT_PROPRIETARY_MODULE, &mod.taints);
    }
    return true;
    }
// Resolve a symbol for this module.  I.e. if we find one, record usage.
    static const struct kernel_symbol *resolve_symbol(module *mod,
    const struct load_info *info,
    const char *name,
    char ownername[])
    {
pub static mut find_symbol_arg: usize = 0;
    let mut err = 0;
//
// The module_mutex should not be a heavily contended lock;
// if we get the occasional sleep here, we'll go an extra iteration
// in the wait_event_interruptible(), which is harmless.
//
    sched_annotate_sleep();
    mutex_lock(&module_mutex);
    if (!find_symbol(&fsa)) {
// goto;
    }
    if (fsa.license == GPL_ONLY) {
    mod.using_gplonly_symbols = true;
    }
    if (!inherit_taint(mod, fsa.owner, name)) {
    fsa.sym = core::ptr::null_mut();
// goto;
    }
    if (!check_version(info, name, mod, fsa.crc)) {
    fsa.sym = ERR_PTR(-EINVAL);
// goto;
    }
    err = verify_namespace_is_imported(info, fsa.sym, mod);
    if (err) {
    fsa.sym = ERR_PTR(err);
// goto;
    }
    err = ref_module(mod, fsa.owner);
    if (err) {
    fsa.sym = ERR_PTR(err);
// goto;
    }
// label;
// We must make copy under the lock if we failed to get ref.
    strscpy(ownername, module_name!(fsa.owner), MODULE_NAME_LEN);
// label;
    mutex_unlock(&module_mutex);
    return fsa.sym;
    }
    static const struct kernel_symbol *
    resolve_symbol_wait(module *mod,
    const struct load_info *info,
    const char *name)
    {
pub static mut ksym: *mut c_void = core::ptr::null_mut();
    char owner[MODULE_NAME_LEN];
    if (wait_event_interruptible_timeout(module_wq,
    !IS_ERR(ksym = resolve_symbol(mod, info, name, owner))
    || PTR_ERR(ksym) != -EBUSY,
    30 * HZ) <= 0) {
    pr_warn!("%s: gave up waiting for init of module %s.\n",
    mod.name, owner);
    }
    return ksym;
    }
#[no_mangle]
pub unsafe extern "C" fn module_arch_cleanup!(mod: *mut module) -> void __weak {
    }
#[no_mangle]
pub unsafe extern "C" fn module_arch_freeing_init!(mod: *mut module) -> void __weak {
    }
#[no_mangle]
unsafe extern "C" fn module_memory_alloc!(mod: *mut module, type: mod_mem_type) -> c_int {
pub static mut size: c_uint = 0;
    enum execmem_type execmem_type;
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    mod.mem[type].size = size;
    if (mod_mem_type_is_data(type)) {
    execmem_type = EXECMEM_MODULE_DATA;
    }
    else {
    execmem_type = EXECMEM_MODULE_TEXT;
    }
    ptr = execmem_alloc_rw(execmem_type, size);
    if (!ptr) {
    return -ENOMEM;
    }
    mod.mem[type].is_rox = execmem_is_rox(execmem_type);
//
// The pointer to these blocks of memory are stored on the module
// structure and we keep that around so long as the module is
// around. We only free that memory when we unload the module.
// Just mark them as not being a leak then. The .init* ELF
// sections *do* get freed after boot so we *could* treat them
// slightly differently with kmemleak_ignore() and only grey
// them out as they work as typical memory allocations which
// *do* eventually get freed, but let's just keep things simple
// and avoid *any* false positives.
//
    if (!mod.mem[type].is_rox) {
    kmemleak_not_leak(ptr);
    }
    memset(ptr, 0, size);
    mod.mem[type].base = ptr;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn module_memory_restore_rox!(mod: *mut module) {
    for_class_mod_mem_type(type, text) {
    let mut mem = &mod.mem[type];
    if (mem.is_rox) {
    execmem_restore_rox(mem.base, mem.size);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn module_memory_free!(mod: *mut module, type: mod_mem_type) {
    let mut mem = &mod.mem[type];
    execmem_free(mem.base);
    }
#[no_mangle]
unsafe extern "C" fn free_mod_mem(mod: *mut module) {
    for_each_mod_mem_type(type) {
    let mut mod_mem = &mod.mem[type];
    if (type == MOD_DATA) {
    continue;
    }
// Free lock-classes; relies on the preceding sync_rcu().
    lockdep_free_key_range(mod_mem.base, mod_mem.size);
    if (mod_mem.size) {
    module_memory_free!(mod, type);
    }
    }
// MOD_DATA hosts mod, so free it at last
    lockdep_free_key_range(mod.mem[MOD_DATA].base, mod.mem[MOD_DATA].size);
    module_memory_free!(mod, MOD_DATA);
    }
// Free a module, remove from lists, etc.
#[no_mangle]
unsafe extern "C" fn free_module(mod: *mut module) {
    trace_module_free(mod);
    codetag_unload_module(mod);
    mod_sysfs_teardown(mod);
//
// We leave it in list to prevent duplicate loads, but make sure
// that noone uses it while it's being deconstructed.
//
    mutex_lock(&module_mutex);
    mod.state = MODULE_STATE_UNFORMED;
    mutex_unlock(&module_mutex);
// Arch-specific cleanup.
    module_arch_cleanup!(mod);
// Module unload stuff
    module_unload_free!(mod);
// Free any allocated parameters.
    module_destroy_params!(mod.kp, mod.num_kp);
    if (is_livepatch_module(mod)) {
    free_module_elf(mod);
    }
// Now we can delete it from the lists
    mutex_lock(&module_mutex);
// Unlink carefully: kallsyms could be walking list.
    list_del_rcu(&mod.list);
    mod_tree_remove(mod);
// Remove this module from bug list, this uses list_del_rcu
    module_bug_cleanup!(mod);
// Wait for RCU synchronizing before releasing mod->list and buglist.
    synchronize_rcu();
    if (try_add_tainted_module(mod)) {
    pr_err!("%s: adding tainted module to the unloaded tainted modules list failed.\n",
    mod.name);
    }
    mutex_unlock(&module_mutex);
// This may be empty, but that's OK
    module_arch_freeing_init!(mod);
    percpu_modfree(mod);
    free_mod_mem(mod);
    }
#[no_mangle]
pub unsafe extern "C" fn __symbol_get(symbol: *mut c_char) -> *mut c_void {
pub static mut find_symbol_arg: usize = 0;
    scoped_guard(rcu) {
    if (!find_symbol(&fsa)) {
    return core::ptr::null_mut();
    }
    if (fsa.license != GPL_ONLY) {
    pr_warn!("failing symbol_get of non-GPLONLY symbol %s.\n",
    symbol);
    return core::ptr::null_mut();
    }
    if (strong_try_module_get(fsa.owner)) {
    return core::ptr::null_mut();
    }
    }
    return kernel_symbol_value(fsa.sym);
    }
    EXPORT_SYMBOL_GPL(__symbol_get);
//
// Ensure that an exported symbol [global namespace] does not already exist
// in the kernel or in some other module's exported symbol table.
//
// You must hold the module_mutex.
//
#[no_mangle]
unsafe extern "C" fn verify_exported_symbols(mod: *mut module) -> c_int {
pub static mut s: *mut c_void = core::ptr::null_mut();
    while (s < mod.syms + mod.num_syms) {
pub static mut find_symbol_arg: usize = 0;
    if (find_symbol(&fsa)) {
    pr_err!("%s: exports duplicate symbol %s (owned by %s)\n",
    mod.name, kernel_symbol_name(s),
    module_name!(fsa.owner));
    return -ENOEXEC;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ignore_undef_symbol(emachine: Elf_Half, name: *const c_char) -> bool {
//
// On x86, PIC code and Clang non-PIC code may have call foo@PLT. GNU as
// before 2.37 produces an unreferenced _GLOBAL_OFFSET_TABLE_ on x86-64.
// i386 has a similar problem but may not deserve a fix.
//
// If we ever have to ignore many symbols, consider refactoring the code to
// only warn if referenced by a relocation.
//
    if (emachine == EM_386 || emachine == EM_X86_64) {
    return !strcmp(name, "_GLOBAL_OFFSET_TABLE_");
    }
    return false;
    }
// Change all symbols so that st_value encodes the pointer directly.
#[no_mangle]
unsafe extern "C" fn simplify_symbols(mod: *mut module, info: *const load_info) -> c_int {
    let mut symsec = &info.sechdrs[info.index.sym];
    let mut sym = symsec.sh_addr;
    let mut secbase = 0;
    let mut i = 0;
pub static mut ret: c_int = 0;
pub static mut ksym: *mut c_void = core::ptr::null_mut();
    while (i < symsec.sh_size / sizeof!(Elf_Sym)) {
    let mut name = info.strtab + sym[i].st_name;
    match (sym[i].st_shndx) {
    SHN_COMMON => {
// Ignore common symbols
    if (!strncmp(name, "__gnu_lto", 9)) {
    // break;
    }
//
// We compiled with -fno-common.  These are not
// supposed to happen.
//
    pr_debug!("Common symbol: %s\n", name);
    pr_warn!("%s: please compile with -fno-common\n",
    mod.name);
    ret = -ENOEXEC;
    // break;
    }
    SHN_ABS => {
// Don't need to do anything
    pr_debug!("Absolute symbol: 0x%08lx %s\n",
    (long)sym[i].st_value, name);
    // break;
    }
    SHN_LIVEPATCH => {
// Livepatch symbols are resolved by livepatch
    // break;
    }
    SHN_UNDEF => {
    ksym = resolve_symbol_wait(mod, info, name);
// Ok if resolved.
    if (ksym && !IS_ERR(ksym)) {
    sym[i].st_value = kernel_symbol_value(ksym);
    // break;
    }
// Ok if weak or ignored.
    if (!ksym &&
    (ELF_ST_BIND(sym[i].st_info) == STB_WEAK ||
    ignore_undef_symbol(info.hdr.e_machine, name))) {
    // break;
    }
    ret = PTR_ERR(ksym) ?: -ENOENT;
    pr_warn!("%s: Unknown symbol %s (err %d)\n",
    mod.name, name, ret);
    // break;
    }
    _ => {
    if (sym[i].st_shndx >= info.hdr.e_shnum) {
    pr_err!("%s: Symbol %s has an invalid section index %u (max %u)\n",
    mod.name, name, sym[i].st_shndx, info.hdr.e_shnum - 1);
    ret = -ENOEXEC;
    // break;
    }
// Divert to percpu allocation if a percpu var.
    if (sym[i].st_shndx == info.index.pcpu) {
    secbase = (unsigned long)mod_percpu(mod);
    }
    else {
    secbase = info.sechdrs[sym[i].st_shndx].sh_addr;
    }
    sym[i].st_value += secbase;
    // break;
    }
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn apply_relocations(mod: *mut module, info: *const load_info) -> c_int {
    let mut i = 0;
pub static mut err: c_int = 0;
// Now do relocations.
    while (i < info.hdr.e_shnum) {
pub static mut infosec: c_uint = 0;
// Not a valid relocation section?
    if (infosec >= info.hdr.e_shnum) {
    continue;
    }
//
// Don't bother with non-allocated sections.
// An exception is the percpu section, which has separate allocations
// for individual CPUs. We relocate the percpu section in the initial
// ELF template and subsequently copy it to the per-CPU destinations.
//
    if (!(info.sechdrs[infosec].sh_flags & SHF_ALLOC) &&
    (!infosec || infosec != info.index.pcpu)) {
    continue;
    }
    if (info.sechdrs[i].sh_flags & SHF_RELA_LIVEPATCH) {
    err = klp_apply_section_relocs(mod, info.sechdrs,
    info.secstrings,
    info.strtab,
    info.index.sym, i,
    core::ptr::null_mut());
    }

    else if (info.sechdrs[i].sh_type == SHT_REL) {
    err = apply_relocate(info.sechdrs, info.strtab,
    info.index.sym, i, mod);
    }

    else if (info.sechdrs[i].sh_type == SHT_RELA) {
    err = apply_relocate_add(info.sechdrs, info.strtab,
    info.index.sym, i, mod);
    }
    if (err < 0) {
    break;
    }
    }
    return err;
    }
// Additional bytes needed by arch in front of individual sections
    unsigned int __weak arch_mod_section_prepend(module *mod,
    unsigned int section)
    {
// default implementation just returns zero
    return 0;
    }
    long module_get_offset_and_type!(module *mod, enum mod_mem_type type,
    Elf_Shdr *sechdr, unsigned int section)
    {
    let mut offset = 0;
pub static mut mask: c_long = 0;
    mod.mem[type].size += arch_mod_section_prepend(mod, section);
    offset = ALIGN(mod.mem[type].size, sechdr.sh_addralign ?: 1);
    mod.mem[type].size = offset + sechdr.sh_size;
    WARN_ON_ONCE!(offset & mask);
    return offset | mask;
    }
#[no_mangle]
pub unsafe extern "C" fn module_init_layout_section!(sname: *const c_char) -> bool {

    if (module_exit_section!(sname)) {
    return true;
    }

    return module_init_section!(sname);
    }
#[no_mangle]
unsafe extern "C" fn __layout_sections(mod: *mut module, info: *mut load_info, is_init: bool) {
    let mut m = 0;
    let mut i = 0;
//
// { Mask of required section header flags,
// Mask of excluded section header flags }
//
    static const unsigned long masks[][2] = {
    { SHF_EXECINSTR | SHF_ALLOC, ARCH_SHF_SMALL },
    { SHF_ALLOC, SHF_WRITE | ARCH_SHF_SMALL },
    { SHF_RO_AFTER_INIT | SHF_ALLOC, ARCH_SHF_SMALL },
    { SHF_WRITE | SHF_ALLOC, ARCH_SHF_SMALL },
    { ARCH_SHF_SMALL | SHF_ALLOC, 0 }
    };
    static const int core_m_to_mem_type[] = {
    MOD_TEXT,
    MOD_RODATA,
    MOD_RO_AFTER_INIT,
    MOD_DATA,
    MOD_DATA,
    };
    static const int init_m_to_mem_type[] = {
    MOD_INIT_TEXT,
    MOD_INIT_RODATA,
    MOD_INVALID,
    MOD_INIT_DATA,
    MOD_INIT_DATA,
    };
    while (m < ARRAY_SIZE!(masks)) {
pub static mut type: mod_mem_type = 0;
    while (i < info.hdr.e_shnum) {
    let mut s = &info.sechdrs[i];
    let mut sname = info.secstrings + s.sh_name;
    if ((s.sh_flags & masks[m][0]) != masks[m][0]
    || (s.sh_flags & masks[m][1])
    || s.sh_entsize != ~0UL
    || is_init != module_init_layout_section!(sname)) {
    continue;
    }
    if (WARN_ON_ONCE!(type == MOD_INVALID)) {
    continue;
    }
//
// Do not allocate codetag memory as we load it into
// preallocated contiguous memory.
//
    if (codetag_needs_module_section(mod, sname, s.sh_size)) {
//
// s->sh_entsize won't be used but populate the
// type field to avoid confusion.
//
    s.sh_entsize = ((unsigned long)(type) & SH_ENTSIZE_TYPE_MASK)
    << SH_ENTSIZE_TYPE_SHIFT;
    continue;
    }
    s.sh_entsize = module_get_offset_and_type!(mod, type, s, i);
    pr_debug!("\t%s\n", sname);
    }
    }
    }
//
// Lay out the SHF_ALLOC sections in a way not dissimilar to how ld
// might -- code, read-only data, read-write data, small data.  Tally
// sizes, and place the offsets into sh_entsize fields: high bit means it
// belongs in init.
//
#[no_mangle]
unsafe extern "C" fn layout_sections(mod: *mut module, info: *mut load_info) {
    let mut i = 0;
    for (i = 0; i < info.hdr.e_shnum; i++) {
    info.sechdrs[i].sh_entsize = ~0UL;
    }
    pr_debug!("Core section allocation order for %s:\n", mod.name);
    __layout_sections(mod, info, false);
    pr_debug!("Init section allocation order for %s:\n", mod.name);
    __layout_sections(mod, info, true);
    }
#[no_mangle]
unsafe extern "C" fn module_license_taint_check!(mod: *mut module, license: *const c_char) {
    if (!license) {
    license = "unspecified";
    }
    if (!license_is_gpl_compatible(license)) {
    if (!test_taint(TAINT_PROPRIETARY_MODULE)) {
    pr_warn!("%s: module license '%s' taints kernel.\n",
    mod.name, license);
    }
    add_taint_module(mod, TAINT_PROPRIETARY_MODULE,
    LOCKDEP_NOW_UNRELIABLE);
    }
    }
#[no_mangle]
unsafe extern "C" fn copy_modinfo_import_ns(mod: *mut module, info: *mut load_info) -> c_int {
pub static mut ns: *mut c_void = core::ptr::null_mut();
    size_t len, total_len = 0;
    let mut buf = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    for_each_modinfo_entry(ns, info, "import_ns") {
    total_len += strlen(ns) + 1;
    }
    if (!total_len) {
    mod.imported_namespaces = core::ptr::null_mut();
    return 0;
    }
    buf = kmalloc(total_len, GFP_KERNEL);
    if (!buf) {
    return -ENOMEM;
    }
    p = buf;
    for_each_modinfo_entry(ns, info, "import_ns") {
    len = strlen(ns);
    memcpy(p, ns, len);
    p += len;
// p++ = '\n';
    }
// Replace trailing newline with null terminator.
// (p - 1) = '\0';
    mod.imported_namespaces = buf;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn setup_modinfo(mod: *mut module, info: *mut load_info) -> c_int {
pub static mut attr: *mut c_void = core::ptr::null_mut();
pub static mut imported_namespace: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut err = 0;
    while ((attr = modinfo_attrs[i])) {
    if (attr.setup) {
    attr.setup(mod, get_modinfo(info, attr.attr.name));
    }
    }
    for_each_modinfo_entry(imported_namespace, info, "import_ns") {
//
// 'module:' prefixed namespaces are implicit, disallow
// explicit imports.
//
    if (strstarts(imported_namespace, "module:")) {
    pr_err!("%s: module tries to import module namespace: %s\n",
    mod.name, imported_namespace);
    return -EPERM;
    }
    }
    err = copy_modinfo_import_ns(mod, info);
    if (err) {
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_modinfo(mod: *mut module) {
pub static mut attr: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while ((attr = modinfo_attrs[i])) {
    if (attr.free) {
    attr.free(mod);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn module_init_section!(name: *const c_char) -> bool __weak {
    return strstarts(name, ".init");
    }
#[no_mangle]
pub unsafe extern "C" fn module_exit_section!(name: *const c_char) -> bool __weak {
    return strstarts(name, ".exit");
    }
#[no_mangle]
unsafe extern "C" fn validate_section_offset(info: *const load_info, shdr: *mut Elf_Shdr) -> c_int {

    unsigned long long secend;

    let mut secend = 0;

//
// Check for both overflow and offset/size being
// too large.
//
    secend = shdr.sh_offset + shdr.sh_size;
    if (secend < shdr.sh_offset || secend > info.len) {
    return -ENOEXEC;
    }
    return 0;
    }
//
// elf_validity_ehdr() - Checks an ELF header for module validity
// @info: Load info containing the ELF header to check
//
// Checks whether an ELF header could belong to a valid module. Checks:
//
// * ELF header is within the data the user provided
// * ELF magic is present
// * It is relocatable (not final linked, not core file, etc.)
// * The header's machine type matches what the architecture expects.
// * Optional arch-specific hook for other properties
// - module_elf_check_arch!() is currently only used by PPC to check
// ELF ABI version, but may be used by others in the future.
//
// Return: %0 if valid, %-ENOEXEC on failure.
//
#[no_mangle]
unsafe extern "C" fn elf_validity_ehdr(info: *const load_info) -> c_int {
    if (info.len < sizeof!(*(info.hdr))) {
    pr_err!("Invalid ELF header len %lu\n", info.len);
    return -ENOEXEC;
    }
    if (memcmp(info.hdr.e_ident, ELFMAG, SELFMAG) != 0) {
    pr_err!("Invalid ELF header magic: != %s\n", ELFMAG);
    return -ENOEXEC;
    }
    if (info.hdr.e_type != ET_REL) {
    pr_err!("Invalid ELF header type: %u != %u\n",
    info.hdr.e_type, ET_REL);
    return -ENOEXEC;
    }
    if (!elf_check_arch(info.hdr)) {
    pr_err!("Invalid architecture in ELF header: %u\n",
    info.hdr.e_machine);
    return -ENOEXEC;
    }
    if (!module_elf_check_arch!(info.hdr)) {
    pr_err!("Invalid module architecture in ELF header: %u\n",
    info.hdr.e_machine);
    return -ENOEXEC;
    }
    return 0;
    }
//
// elf_validity_cache_sechdrs() - Cache section headers if valid
// @info: Load info to compute section headers from
//
// Checks:
//
// * ELF header is valid (see elf_validity_ehdr())
// * Section headers are the size we expect
// * Section array fits in the user provided data
// * Section index 0 is NULL
// * Section contents are inbounds
//
// Then updates @info with a &load_info->sechdrs pointer if valid.
//
// Return: %0 if valid, negative error code if validation failed.
//
#[no_mangle]
unsafe extern "C" fn elf_validity_cache_sechdrs(info: *mut load_info) -> c_int {
pub static mut sechdrs: *mut c_void = core::ptr::null_mut();
pub static mut shdr: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut err = 0;
    err = elf_validity_ehdr(info);
    if (err < 0) {
    return err;
    }
    if (info.hdr.e_shentsize != sizeof!(Elf_Shdr)) {
    pr_err!("Invalid ELF section header size\n");
    return -ENOEXEC;
    }
//
// e_shnum is 16 bits, and sizeof!(Elf_Shdr) is
// known and small. So e_shnum * sizeof!(Elf_Shdr)
// will not overflow unsigned long on any platform.
//
    if (info.hdr.e_shoff >= info.len
    || (info.hdr.e_shnum * sizeof!(Elf_Shdr) >
    info.len - info.hdr.e_shoff)) {
    pr_err!("Invalid ELF section header overflow\n");
    return -ENOEXEC;
    }
    sechdrs = info.hdr + info.hdr.e_shoff;
//
// The code assumes that section 0 has a length of zero and
// an addr of zero, so check for it.
//
    if (sechdrs[0].sh_type != SHT_NULL
    || sechdrs[0].sh_size != 0
    || sechdrs[0].sh_addr != 0) {
    pr_err!("ELF Spec violation: section 0 type(%d)!=SH_NULL or non-zero len or addr\n",
    sechdrs[0].sh_type);
    return -ENOEXEC;
    }
// Validate contents are inbounds
    while (i < info.hdr.e_shnum) {
    shdr = &sechdrs[i];
    match (shdr.sh_type) {
    SHT_NULL => {
    }
    SHT_NOBITS => {
// No contents, offset/size don't mean anything
    continue;
    }
    _ => {
    err = validate_section_offset(info, shdr);
    if (err < 0) {
    pr_err!("Invalid ELF section in module (section %u type %u)\n",
    i, shdr.sh_type);
    return err;
    }
    }
    }
    }
    info.sechdrs = sechdrs;
    return 0;
    }
//
// elf_validity_cache_secstrings() - Caches section names if valid
// @info: Load info to cache section names from. Must have valid sechdrs.
//
// Specifically checks:
//
// * Section name table index is inbounds of section headers
// * Section name table type is SHT_STRTAB
// * Section name table is not empty
// * Section name table is NUL terminated
// * All section name offsets are inbounds of the section
//
// Then updates @info with a &load_info->secstrings pointer if valid.
//
// Return: %0 if valid, negative error code if validation failed.
//
#[no_mangle]
unsafe extern "C" fn elf_validity_cache_secstrings(info: *mut load_info) -> c_int {
    let mut strhdr = core::ptr::null_mut();
    let mut shdr = core::ptr::null_mut();
pub static mut secstrings: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
//
// Verify if the section name table index is valid.
//
    if (info.hdr.e_shstrndx == SHN_UNDEF
    || info.hdr.e_shstrndx >= info.hdr.e_shnum) {
    pr_err!("Invalid ELF section name index: %d || e_shstrndx (%d) >= e_shnum (%d)\n",
    info.hdr.e_shstrndx, info.hdr.e_shstrndx,
    info.hdr.e_shnum);
    return -ENOEXEC;
    }
    strhdr = &info.sechdrs[info.hdr.e_shstrndx];
    if (strhdr.sh_type != SHT_STRTAB) {
    pr_err!("Invalid ELF section name table type: %u\n", strhdr.sh_type);
    return -ENOEXEC;
    }
//
// The section name table must be NUL-terminated, as required
// by the spec. This makes strcmp and pr_* calls that access
// strings in the section safe.
//
    secstrings = info.hdr + strhdr.sh_offset;
    if (strhdr.sh_size == 0) {
    pr_err!("empty section name table\n");
    return -ENOEXEC;
    }
    if (secstrings[strhdr.sh_size - 1] != '\0') {
    pr_err!("ELF Spec violation: section name table isn't null terminated\n");
    return -ENOEXEC;
    }
    while (i < info.hdr.e_shnum) {
    shdr = &info.sechdrs[i];
// SHT_NULL means sh_name has an undefined value
    if (shdr.sh_type == SHT_NULL) {
    continue;
    }
    if (shdr.sh_name >= strhdr.sh_size) {
    pr_err!("Invalid ELF section name in module (section %u type %u)\n",
    i, shdr.sh_type);
    return -ENOEXEC;
    }
    }
    info.secstrings = secstrings;
    return 0;
    }
//
// elf_validity_cache_index_info() - Validate and cache modinfo section
// @info: Load info to populate the modinfo index on.
// Must have &load_info->sechdrs and &load_info->secstrings populated
//
// Checks that if there is a .modinfo section, it is unique.
// Then, it caches its index in &load_info->index.info.
// Finally, it tries to populate the name to improve error messages.
//
// Return: %0 if valid, %-ENOEXEC if multiple modinfo sections were found.
//
#[no_mangle]
unsafe extern "C" fn elf_validity_cache_index_info(info: *mut load_info) -> c_int {
    let mut info_idx = 0;
    info_idx = find_any_unique_sec(info, ".modinfo");
    if (info_idx == 0) {
// Early return, no .modinfo
    return 0;
    }
    if (info_idx < 0) {
    pr_err!("Only one .modinfo section must exist.\n");
    return -ENOEXEC;
    }
    info.index.info = info_idx;
// Try to find a name early so we can log errors with a module name
    info.name = get_modinfo(info, "name");
    return 0;
    }
//
// elf_validity_cache_index_mod() - Validates and caches this_module section
// @info: Load info to cache this_module on.
// Must have &load_info->sechdrs and &load_info->secstrings populated
//
// The ".gnu.linkonce.this_module" ELF section is special. It is what modpost
// uses to refer to __this_module and let's use rely on THIS_MODULE to point
// to &__this_module properly. The kernel's modpost declares it on each
// modules's *.mod.c file. If the struct module of the kernel changes a full
// kernel rebuild is required.
//
// We have a few expectations for this special section, this function
// validates all this for us:
//
// * The section has contents
// * The section is unique
// * We expect the kernel to always have to allocate it: SHF_ALLOC
// * The section size must match the kernel's run time's struct module
// size
//
// If all checks pass, the index will be cached in &load_info->index.mod
//
// Return: %0 on validation success, %-ENOEXEC on failure
//
#[no_mangle]
unsafe extern "C" fn elf_validity_cache_index_mod(info: *mut load_info) -> c_int {
pub static mut shdr: *mut c_void = core::ptr::null_mut();
    let mut mod_idx = 0;
    mod_idx = find_any_unique_sec(info, ".gnu.linkonce.this_module");
    if (mod_idx <= 0) {
    pr_err!("module %s: Exactly one .gnu.linkonce.this_module section must exist.\n",
    info.name ?: "(missing .modinfo section or name field)");
    return -ENOEXEC;
    }
    shdr = &info.sechdrs[mod_idx];
    if (shdr.sh_type == SHT_NOBITS) {
    pr_err!("module %s: .gnu.linkonce.this_module section must have a size set\n",
    info.name ?: "(missing .modinfo section or name field)");
    return -ENOEXEC;
    }
    if (!(shdr.sh_flags & SHF_ALLOC)) {
    pr_err!("module %s: .gnu.linkonce.this_module must occupy memory during process execution\n",
    info.name ?: "(missing .modinfo section or name field)");
    return -ENOEXEC;
    }
    if (shdr.sh_size != sizeof!(module)) {
    pr_err!("module %s: .gnu.linkonce.this_module section size must match the kernel's built struct module size at run time\n",
    info.name ?: "(missing .modinfo section or name field)");
    return -ENOEXEC;
    }
    info.index.mod = mod_idx;
    return 0;
    }
//
// elf_validity_cache_index_sym() - Validate and cache symtab index
// @info: Load info to cache symtab index in.
// Must have &load_info->sechdrs and &load_info->secstrings populated.
//
// Checks that there is exactly one symbol table, then caches its index in
// &load_info->index.sym.
//
// Return: %0 if valid, %-ENOEXEC on failure.
//
#[no_mangle]
unsafe extern "C" fn elf_validity_cache_index_sym(info: *mut load_info) -> c_int {
    let mut sym_idx = 0;
pub static mut num_sym_secs: c_uint = 0;
    let mut i = 0;
    while (i < info.hdr.e_shnum) {
    if (info.sechdrs[i].sh_type == SHT_SYMTAB) {
    num_sym_secs += 1;
    sym_idx = i;
    }
    }
    if (num_sym_secs != 1) {
    pr_warn!("%s: module has no symbols (stripped?)\n",
    info.name ?: "(missing .modinfo section or name field)");
    return -ENOEXEC;
    }
    info.index.sym = sym_idx;
    return 0;
    }
//
// elf_validity_cache_index_str() - Validate and cache strtab index
// @info: Load info to cache strtab index in.
// Must have &load_info->sechdrs and &load_info->secstrings populated.
// Must have &load_info->index.sym populated.
//
// Looks at the symbol table's associated string table, makes sure it is
// in-bounds and of type SHT_STRTAB, and caches it.
//
// Return: %0 if valid, %-ENOEXEC on failure.
//
#[no_mangle]
unsafe extern "C" fn elf_validity_cache_index_str(info: *mut load_info) -> c_int {
pub static mut str_idx: c_uint = 0;
    if (str_idx == SHN_UNDEF || str_idx >= info.hdr.e_shnum) {
    pr_err!("Invalid ELF sh_link!=SHN_UNDEF(%d) or (sh_link(%d) >= hdr.e_shnum(%d)\n",
    str_idx, str_idx, info.hdr.e_shnum);
    return -ENOEXEC;
    }
    if (info.sechdrs[str_idx].sh_type != SHT_STRTAB) {
    pr_err!("Invalid ELF symbol string table type: %u\n",
    info.sechdrs[str_idx].sh_type);
    return -ENOEXEC;
    }
    info.index.str = str_idx;
    return 0;
    }
//
// elf_validity_cache_index_versions() - Validate and cache version indices
// @info:  Load info to cache version indices in.
// Must have &load_info->sechdrs and &load_info->secstrings populated.
// @flags: Load flags, relevant to suppress version loading, see
// uapi/linux/module.h
//
// If we're ignoring modversions based on @flags, zero all version indices
// and return validity. Othewrise check:
//
// * If "__version_ext_crcs" is present, "__version_ext_names" is present
// * There is a name present for every crc
//
// Then populate:
//
// * &load_info->index.vers
// * &load_info->index.vers_ext_crc
// * &load_info->index.vers_ext_names
//
// if present.
//
// Return: %0 if valid, %-ENOEXEC on failure.
//
#[no_mangle]
unsafe extern "C" fn elf_validity_cache_index_versions(info: *mut load_info, flags: c_int) -> c_int {
    let mut vers_ext_crc = 0;
    let mut vers_ext_name = 0;
    let mut crc_count = 0;
    let mut remaining_len = 0;
    let mut name_size = 0;
pub static mut name: *mut c_void = core::ptr::null_mut();
// If modversions were suppressed, pretend we didn't find any
    if (flags & MODULE_INIT_IGNORE_MODVERSIONS) {
    info.index.vers = 0;
    info.index.vers_ext_crc = 0;
    info.index.vers_ext_name = 0;
    return 0;
    }
    vers_ext_crc = find_sec(info, "__version_ext_crcs");
    vers_ext_name = find_sec(info, "__version_ext_names");
// If we have one field, we must have the other
    if (!!vers_ext_crc != !!vers_ext_name) {
    pr_err!("extended version crc+name presence does not match");
    return -ENOEXEC;
    }
//
// If we have extended version information, we should have the same
// number of entries in every section.
//
    if (vers_ext_crc) {
    crc_count = info.sechdrs[vers_ext_crc].sh_size / sizeof!(u32);
    name = info.hdr +
    info.sechdrs[vers_ext_name].sh_offset;
    remaining_len = info.sechdrs[vers_ext_name].sh_size;
    while (crc_count--) {
    name_size = strnlen(name, remaining_len) + 1;
    if (name_size > remaining_len) {
    pr_err!("more extended version crcs than names");
    return -ENOEXEC;
    }
    remaining_len -= name_size;
    name += name_size;
    }
    }
    info.index.vers = find_sec(info, "__versions");
    info.index.vers_ext_crc = vers_ext_crc;
    info.index.vers_ext_name = vers_ext_name;
    return 0;
    }
//
// elf_validity_cache_index() - Resolve, validate, cache section indices
// @info:  Load info to read from and update.
// &load_info->sechdrs and &load_info->secstrings must be populated.
// @flags: Load flags, relevant to suppress version loading, see
// uapi/linux/module.h
//
// Populates &load_info->index, validating as it goes.
// See child functions for per-field validation:
//
// * elf_validity_cache_index_info()
// * elf_validity_cache_index_mod()
// * elf_validity_cache_index_sym()
// * elf_validity_cache_index_str()
// * elf_validity_cache_index_versions()
//
// If CONFIG_SMP is enabled, load the percpu section by name with no
// validation.
//
// Return: 0 on success, negative error code if an index failed validation.
//
#[no_mangle]
unsafe extern "C" fn elf_validity_cache_index(info: *mut load_info, flags: c_int) -> c_int {
    let mut err = 0;
    err = elf_validity_cache_index_info(info);
    if (err < 0) {
    return err;
    }
    err = elf_validity_cache_index_mod(info);
    if (err < 0) {
    return err;
    }
    err = elf_validity_cache_index_sym(info);
    if (err < 0) {
    return err;
    }
    err = elf_validity_cache_index_str(info);
    if (err < 0) {
    return err;
    }
    err = elf_validity_cache_index_versions(info, flags);
    if (err < 0) {
    return err;
    }
    info.index.pcpu = find_pcpusec(info);
    return 0;
    }
//
// elf_validity_cache_strtab() - Validate and cache symbol string table
// @info: Load info to read from and update.
// Must have &load_info->sechdrs and &load_info->secstrings populated.
// Must have &load_info->index populated.
//
// Checks:
//
// * The string table is not empty.
// * The string table starts and ends with NUL (required by ELF spec).
// * Every &Elf_Sym->st_name offset in the symbol table is inbounds of the
// string table.
//
// And caches the pointer as &load_info->strtab in @info.
//
// Return: 0 on success, negative error code if a check failed.
//
#[no_mangle]
unsafe extern "C" fn elf_validity_cache_strtab(info: *mut load_info) -> c_int {
    let mut str_shdr = &info.sechdrs[info.index.str];
    let mut sym_shdr = &info.sechdrs[info.index.sym];
    let mut strtab = info.hdr + str_shdr.sh_offset;
    let mut syms = info.hdr + sym_shdr.sh_offset;
    let mut i = 0;
    if (str_shdr.sh_size == 0) {
    pr_err!("empty symbol string table\n");
    return -ENOEXEC;
    }
    if (strtab[0] != '\0') {
    pr_err!("symbol string table missing leading NUL\n");
    return -ENOEXEC;
    }
    if (strtab[str_shdr.sh_size - 1] != '\0') {
    pr_err!("symbol string table isn't NUL terminated\n");
    return -ENOEXEC;
    }
//
// Now that we know strtab is correctly structured, check symbol
// starts are inbounds before they're used later.
//
    while (i < sym_shdr.sh_size / sizeof!(*syms)) {
    if (syms[i].st_name >= str_shdr.sh_size) {
    pr_err!("symbol name out of bounds in string table");
    return -ENOEXEC;
    }
    }
    info.strtab = strtab;
    return 0;
    }
//
// Check userspace passed ELF module against our expectations, and cache
// useful variables for further processing as we go.
//
// This does basic validity checks against section offsets and sizes, the
// section name string table, and the indices used for it (sh_name).
//
// As a last step, since we're already checking the ELF sections we cache
// useful variables which will be used later for our convenience:
//
// o pointers to section headers
// o cache the modinfo symbol section
// o cache the string symbol section
// o cache the module section
//
// As a last step we set info->mod to the temporary copy of the module in
// info->hdr. The final one will be allocated in move_module(). Any
// modifications we make to our copy of the module will be carried over
// to the final minted module.
//
#[no_mangle]
unsafe extern "C" fn elf_validity_cache_copy(info: *mut load_info, flags: c_int) -> c_int {
    let mut err = 0;
    err = elf_validity_cache_sechdrs(info);
    if (err < 0) {
    return err;
    }
    err = elf_validity_cache_secstrings(info);
    if (err < 0) {
    return err;
    }
    err = elf_validity_cache_index(info, flags);
    if (err < 0) {
    return err;
    }
    err = elf_validity_cache_strtab(info);
    if (err < 0) {
    return err;
    }
// This is temporary: point mod into copy of data.
    info.mod = info.hdr + info.sechdrs[info.index.mod].sh_offset;
//
// If we didn't load the .modinfo 'name' field earlier, fall back to
// on-disk struct mod 'name' field.
//
    if (!info.name) {
    info.name = info.mod.name;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn copy_chunked_from_user(dst: *mut c_void, usrc: *const c_void , len: c_ulong) -> c_int {
    do {
pub static mut n: c_ulong = 0;
    if (copy_from_user(dst, usrc, n) != 0) {
    return -EFAULT;
    }
    cond_resched();
    dst += n;
    usrc += n;
    len -= n;
    } while (len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_modinfo_livepatch(mod: *mut module, info: *mut load_info) -> c_int {
    if (!get_modinfo(info, "livepatch")) {
// Nothing more to do
    return 0;
    }
    if (set_livepatch_module(mod)) {
    return 0;
    }
    pr_err!("%s: module is marked as livepatch module, but livepatch support is disabled",
    mod.name);
    return -ENOEXEC;
    }
#[no_mangle]
unsafe extern "C" fn check_modinfo_retpoline(mod: *mut module, info: *mut load_info) {
    if (retpoline_module_ok(get_modinfo(info, "retpoline"))) {
    return;
    }
    pr_warn!("%s: loading module not compiled with retpoline compiler.\n",
    mod.name);
    }
// Sets info->hdr and info->len.
#[no_mangle]
pub unsafe extern "C" fn copy_module_from_user(umod: *mut c_void, len: c_ulong, info: *mut load_info) -> c_int {
    let mut err = 0;
    info.len = len;
    if (info.len < sizeof!(*(info.hdr))) {
    return -ENOEXEC;
    }
    err = security_kernel_load_data(LOADING_MODULE, true);
    if (err) {
    return err;
    }
// Suck in entire file: we'll want most of it.
    info.hdr = __vmalloc(info.len, GFP_KERNEL | __GFP_NOWARN);
    if (!info.hdr) {
    return -ENOMEM;
    }
    if (copy_chunked_from_user(info.hdr, umod, info.len) != 0) {
    err = -EFAULT;
// goto;
    }
    err = security_kernel_post_load_data(info.hdr, info.len,
    LOADING_MODULE, "init_module");
// label;
    if (err) {
    vfree(info.hdr);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn free_copy(info: *mut load_info, flags: c_int) {
    if (flags & MODULE_INIT_COMPRESSED_FILE) {
    module_decompress_cleanup!(info);
    }
    else {
    vfree(info.hdr);
    }
    }
#[no_mangle]
unsafe extern "C" fn rewrite_section_headers(info: *mut load_info, flags: c_int) -> c_int {
    let mut i = 0;
// This should always be true, but let's be sure.
    info.sechdrs[0].sh_addr = 0;
    while (i < info.hdr.e_shnum) {
    let mut shdr = &info.sechdrs[i];
//
// Mark all sections sh_addr with their address in the
// temporary image.
//
    shdr.sh_addr = (size_t)info.hdr + shdr.sh_offset;
    }
// Track but don't keep modinfo and version sections.
    info.sechdrs[info.index.vers].sh_flags &= ~(unsigned long)SHF_ALLOC;
    info.sechdrs[info.index.vers_ext_crc].sh_flags &=
    ~(unsigned long)SHF_ALLOC;
    info.sechdrs[info.index.vers_ext_name].sh_flags &=
    ~(unsigned long)SHF_ALLOC;
    info.sechdrs[info.index.info].sh_flags &= ~(unsigned long)SHF_ALLOC;
    return 0;
    }
    static const char *const module_license_offenders[] = {
// driverloader was caught wrongly pretending to be under GPL
    "driverloader",
// lve claims to be GPL but upstream won't provide source
    "lve",
    };
//
// These calls taint the kernel depending certain module circumstances
#[no_mangle]
unsafe extern "C" fn module_augment_kernel_taints!(mod: *mut module, info: *mut load_info) {
pub static mut prev_taint: c_int = 0;
    let mut i = 0;
    if (!get_modinfo(info, "intree")) {
    if (!test_taint(TAINT_OOT_MODULE)) {
    pr_warn!("%s: loading out-of-tree module taints kernel.\n",
    mod.name);
    }
    add_taint_module(mod, TAINT_OOT_MODULE, LOCKDEP_STILL_OK);
    }
    check_modinfo_retpoline(mod, info);
    if (get_modinfo(info, "staging")) {
    add_taint_module(mod, TAINT_CRAP, LOCKDEP_STILL_OK);
    pr_warn!("%s: module is from the staging directory, the quality "
    "is unknown, you have been warned.\n", mod.name);
    }
    if (is_livepatch_module(mod)) {
    add_taint_module(mod, TAINT_LIVEPATCH, LOCKDEP_STILL_OK);
    pr_notice_once("%s: tainting kernel with TAINT_LIVEPATCH\n",
    mod.name);
    }
    module_license_taint_check!(mod, get_modinfo(info, "license"));
    if (get_modinfo(info, "test")) {
    if (!test_taint(TAINT_TEST)) {
    pr_warn!("%s: loading test module taints kernel.\n",
    mod.name);
    }
    add_taint_module(mod, TAINT_TEST, LOCKDEP_STILL_OK);
    }

    mod.sig_ok = info.sig_ok;
    if (!mod.sig_ok) {
    pr_notice_once("%s: module verification failed: signature "
    "and/or required key missing - tainting "
    "kernel\n", mod.name);
    add_taint_module(mod, TAINT_UNSIGNED_MODULE, LOCKDEP_STILL_OK);
    }

//
// ndiswrapper is under GPL by itself, but loads proprietary modules.
// Don't use add_taint_module(), as it would prevent ndiswrapper from
// using GPL-only symbols it needs.
//
    if (strcmp(mod.name, "ndiswrapper") == 0) {
    add_taint(TAINT_PROPRIETARY_MODULE, LOCKDEP_NOW_UNRELIABLE);
    }
    while (i < ARRAY_SIZE!(module_license_offenders)) {
    if (strcmp(mod.name, module_license_offenders[i]) == 0) {
    add_taint_module(mod, TAINT_PROPRIETARY_MODULE,
    LOCKDEP_NOW_UNRELIABLE);
    }
    }
    if (!prev_taint && test_taint(TAINT_PROPRIETARY_MODULE)) {
    pr_warn!("%s: module license taints kernel.\n", mod.name);
    }
    }
#[no_mangle]
unsafe extern "C" fn check_modinfo(mod: *mut module, info: *mut load_info, flags: c_int) -> c_int {
    let mut modmagic = get_modinfo(info, "vermagic");
    let mut err = 0;
    if (flags & MODULE_INIT_IGNORE_VERMAGIC) {
    modmagic = core::ptr::null_mut();
    }
// This is allowed: modprobe --force will invalidate it.
    if (!modmagic) {
    err = try_to_force_load(mod, "bad vermagic");
    if (err) {
    return err;
    }
    } else if (!same_magic(modmagic, vermagic, info.index.vers)) {
    pr_err!("%s: version magic '%s' should be '%s'\n",
    info.name, modmagic, vermagic);
    return -ENOEXEC;
    }
    err = check_modinfo_livepatch(mod, info);
    if (err) {
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn find_module_sections(mod: *mut module, info: *mut load_info) -> c_int {
    mod.kp = section_objs(info, "__param",
    sizeof!(*mod.kp), &mod.num_kp);
    mod.syms = section_objs(info, "__ksymtab",
    sizeof!(*mod.syms), &mod.num_syms);
    mod.crcs = section_addr(info, "__kcrctab");
    mod.flagstab = section_addr(info, "__kflagstab");
    if (section_addr(info, "__ksymtab_gpl")) {
    pr_warn!("%s: ignoring obsolete section __ksymtab_gpl\n",
    mod.name);
    }
    if (section_addr(info, "__kcrctab_gpl")) {
    pr_warn!("%s: ignoring obsolete section __kcrctab_gpl\n",
    mod.name);
    }

    mod.ctors = section_objs(info, ".ctors",
    sizeof!(*mod.ctors), &mod.num_ctors);
    if (!mod.ctors) {
    mod.ctors = section_objs(info, ".init_array",
    sizeof!(*mod.ctors), &mod.num_ctors);
    }
if true {
//
// This shouldn't happen with same compiler and binutils
// building all parts of the module.
//
    pr_warn!("%s: has both .ctors and .init_array.\n",
    mod.name);
    return -EINVAL;
    }

    mod.noinstr_text_start = section_objs(info, ".noinstr.text", 1,
    &mod.noinstr_text_size);

    mod.tracepoints_ptrs = section_objs(info, "__tracepoints_ptrs",
    sizeof!(*mod.tracepoints_ptrs),
    &mod.num_tracepoints);

    mod.srcu_struct_ptrs = section_objs(info, "___srcu_struct_ptrs",
    sizeof!(*mod.srcu_struct_ptrs),
    &mod.num_srcu_structs);

    mod.bpf_raw_events = section_objs(info, "__bpf_raw_tp_map",
    sizeof!(*mod.bpf_raw_events),
    &mod.num_bpf_raw_events);

    mod.btf_data = any_section_objs(info, ".BTF", 1, &mod.btf_data_size);
    mod.btf_base_data = any_section_objs(info, ".BTF.base", 1,
    &mod.btf_base_data_size);

    mod.jump_entries = section_objs(info, "__jump_table",
    sizeof!(*mod.jump_entries),
    &mod.num_jump_entries);

    mod.trace_events = section_objs(info, "_ftrace_events",
    sizeof!(*mod.trace_events),
    &mod.num_trace_events);
    mod.trace_evals = section_objs(info, "_ftrace_eval_map",
    sizeof!(*mod.trace_evals),
    &mod.num_trace_evals);

    mod.trace_bprintk_fmt_start = section_objs(info, "__trace_printk_fmt",
    sizeof!(*mod.trace_bprintk_fmt_start),
    &mod.num_trace_bprintk_fmt);

// sechdrs[0].sh_size is always zero
    mod.ftrace_callsites = section_objs(info, FTRACE_CALLSITE_SECTION,
    sizeof!(*mod.ftrace_callsites),
    &mod.num_ftrace_callsites);

    mod.ei_funcs = section_objs(info, "_error_injection_whitelist",
    sizeof!(*mod.ei_funcs),
    &mod.num_ei_funcs);

    mod.kprobes_text_start = section_objs(info, ".kprobes.text", 1,
    &mod.kprobes_text_size);
    mod.kprobe_blacklist = section_objs(info, "_kprobe_blacklist",
    sizeof!(unsigned long),
    &mod.num_kprobe_blacklist);

    mod.printk_index_start = section_objs(info, ".printk_index",
    sizeof!(*mod.printk_index_start),
    &mod.printk_index_size);

    mod.static_call_sites = section_objs(info, ".static_call_sites",
    sizeof!(*mod.static_call_sites),
    &mod.num_static_call_sites);

    mod.kunit_suites = section_objs(info, ".kunit_test_suites",
    sizeof!(*mod.kunit_suites),
    &mod.num_kunit_suites);
    mod.kunit_init_suites = section_objs(info, ".kunit_init_test_suites",
    sizeof!(*mod.kunit_init_suites),
    &mod.num_kunit_init_suites);

    mod.extable = section_objs(info, "__ex_table",
    sizeof!(*mod.extable), &mod.num_exentries);
    if (section_addr(info, "__obsparm")) {
    pr_warn!("%s: Ignoring obsolete parameters\n", mod.name);
    }

    mod.dyndbg_info.descs = section_objs(info, "__dyndbg",
    sizeof!(*mod.dyndbg_info.descs),
    &mod.dyndbg_info.num_descs);
    mod.dyndbg_info.classes = section_objs(info, "__dyndbg_classes",
    sizeof!(*mod.dyndbg_info.classes),
    &mod.dyndbg_info.num_classes);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn move_module(mod: *mut module, info: *mut load_info) -> c_int {
    let mut i = 0;
    let mut ret = 0;
pub static mut t: mod_mem_type = 0;
pub static mut codetag_section_found: bool = false;
    for_each_mod_mem_type(type) {
    if (!mod.mem[type].size) {
    mod.mem[type].base = core::ptr::null_mut();
    continue;
    }
    ret = module_memory_alloc!(mod, type);
    if (ret) {
    t = type;
// goto;
    }
    }
// Transfer each section which specifies SHF_ALLOC
    pr_debug!("Final section addresses for %s:\n", mod.name);
    while (i < info.hdr.e_shnum) {
pub static mut dest: *mut c_void = core::ptr::null_mut();
    let mut shdr = &info.sechdrs[i];
pub static mut sname: *mut c_void = core::ptr::null_mut();
    if (!(shdr.sh_flags & SHF_ALLOC)) {
    continue;
    }
    sname = info.secstrings + shdr.sh_name;
//
// Load codetag sections separately as they might still be used
// after module unload.
//
    if (codetag_needs_module_section(mod, sname, shdr.sh_size)) {
    dest = codetag_alloc_module_section(mod, sname, shdr.sh_size,
    arch_mod_section_prepend(mod, i), shdr.sh_addralign);
    if (WARN_ON!(!dest)) {
    ret = -EINVAL;
// goto;
    }
    if (IS_ERR(dest)) {
    ret = PTR_ERR(dest);
// goto;
    }
    codetag_section_found = true;
    } else {
pub static mut type: mod_mem_type = 0;
pub static mut offset: c_ulong = 0;
    dest = mod.mem[type].base + offset;
    }
    if (shdr.sh_type != SHT_NOBITS) {
//
// Our ELF checker already validated this, but let's
// be pedantic and make the goal clearer. We actually
// end up copying over all modifications made to the
// userspace copy of the entire struct module.
//
    if (i == info.index.mod &&
    (WARN_ON_ONCE!(shdr.sh_size != sizeof!(module)))) {
    ret = -ENOEXEC;
// goto;
    }
    memcpy(dest, shdr.sh_addr, shdr.sh_size);
    }
//
// Update the userspace copy's ELF section address to point to
// our newly allocated memory as a pure convenience so that
// users of info can keep taking advantage and using the newly
// minted official memory area.
//
    shdr.sh_addr = (unsigned long)dest;
    pr_debug!("\t0x%lx 0x%.8lx %s\n", (long)shdr.sh_addr,
    (long)shdr.sh_size, info.secstrings + shdr.sh_name);
    }
    return 0;
// label;
    module_memory_restore_rox!(mod);
    while (t--) {
    module_memory_free!(mod, t);
    }
    if (codetag_section_found) {
    codetag_free_module_sections(mod);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn check_export_symbol_sections(mod: *mut module) -> c_int {
    if (mod.num_syms && !mod.flagstab) {
    pr_err!("%s: no flags for exported symbols\n", mod.name);
    return -ENOEXEC;
    }

    if (mod.num_syms && !mod.crcs) {
    return try_to_force_load(mod,
    "no versions for exported symbols");
    }

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn flush_module_icache(mod: *const module) {
//
// Flush the instruction cache, since we've played with text.
// Do it before processing of module parameters, so the module
// can provide parameter accessor functions of its own.
//
    for_each_mod_mem_type(type) {
    let mut mod_mem = &mod.mem[type];
    if (mod_mem.size) {
    flush_icache_range((unsigned long)mod_mem.base,
    (unsigned long)mod_mem.base + mod_mem.size);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn module_elf_check_arch!(hdr: *mut Elf_Ehdr) -> bool __weak {
    return true;
    }
    int __weak module_frob_arch_sections!(Elf_Ehdr *hdr,
    Elf_Shdr *sechdrs,
    char *secstrings, module *mod)
    {
    return 0;
    }
// module_blacklist is a comma-separated list of module names
pub static mut module_blacklist: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn blacklisted(module_name: *const c_char) -> bool {
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    if (!module_blacklist) {
    return false;
    }
    while (*p) {
    len = strcspn(p, ",");
    if (strlen(module_name) == len && !memcmp(module_name, p, len)) {
    return true;
    }
    if (p[len] == ',') {
    len += 1;
    }
    }
    return false;
    }
    core_param!(module_blacklist, module_blacklist, charp, 0400);
#[no_mangle]
pub unsafe extern "C" fn layout_and_allocate(info: *mut load_info, flags: c_int) -> *mut c_void {
pub static mut mod: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
// Allow arches to frob section contents and sizes.
    err = module_frob_arch_sections!(info.hdr, info.sechdrs,
    info.secstrings, info.mod);
    if (err < 0) {
    return ERR_PTR(err);
    }
    err = module_enforce_rwx_sections!(info.hdr, info.sechdrs,
    info.secstrings, info.mod);
    if (err < 0) {
    return ERR_PTR(err);
    }
// We will do a special allocation for per-cpu sections later.
    info.sechdrs[info.index.pcpu].sh_flags &= ~(unsigned long)SHF_ALLOC;
//
// Mark relevant sections as SHF_RO_AFTER_INIT so layout_sections() can
// put them in the right place.
// Note: ro_after_init sections also have SHF_{WRITE,ALLOC} set.
//
    module_mark_ro_after_init!(info.hdr, info.sechdrs, info.secstrings);
//
// Determine total sizes, and put offsets in sh_entsize.  For now
// this is done generically; there doesn't appear to be any
// special cases for the architectures.
//
    layout_sections(info.mod, info);
    layout_symtab(info.mod, info);
// Allocate and move to the final place
    err = move_module(info.mod, info);
    if (err) {
    return ERR_PTR(err);
    }
// Module has been copied to its final place now: return it.
    mod = info.sechdrs[info.index.mod].sh_addr;
    kmemleak_load_module(mod, info);
    codetag_module_replaced(info.mod, mod);
    return mod;
    }
// mod is no longer valid after this!
#[no_mangle]
unsafe extern "C" fn module_deallocate!(mod: *mut module, info: *mut load_info) {
    percpu_modfree(mod);
    module_arch_freeing_init!(mod);
    codetag_free_module_sections(mod);
    free_mod_mem(mod);
    }
    int __weak module_finalize!(const Elf_Ehdr *hdr,
    const Elf_Shdr *sechdrs, module *me)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn post_relocation(mod: *mut module, info: *const load_info) -> c_int {
// Sort exception table now relocations are done.
    sort_extable(mod.extable, mod.extable + mod.num_exentries);
// Copy relocated percpu area over.
    percpu_modcopy(mod, info.sechdrs[info.index.pcpu].sh_addr,
    info.sechdrs[info.index.pcpu].sh_size);
// Setup kallsyms-specific fields.
    add_kallsyms(mod, info);
// Arch-specific module finalizing.
    return module_finalize!(info.hdr, info.sechdrs, mod);
    }
// Call module constructors.
#[no_mangle]
unsafe extern "C" fn do_mod_ctors(mod: *mut module) {

    let mut i = 0;
    for (i = 0; i < mod.num_ctors; i++) {
    mod.ctors[i]();
    }

    }
// For freeing module_init on success, in case kallsyms traversing
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_initfree {
    pub node: llist_node,
    pub init_text: *mut c_void,
    pub init_data: *mut c_void,
    pub init_rodata: *mut c_void,
}

#[no_mangle]
unsafe extern "C" fn do_free_init(w: *mut work_struct) {
    let mut pos = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
    let mut list = core::ptr::null_mut();
pub static mut initfree: *mut c_void = core::ptr::null_mut();
    list = llist_del_all(&init_free_list);
    synchronize_rcu();
    llist_for_each_safe(pos, n, list) {
    initfree = container_of!(pos, mod_initfree, node);
    execmem_free(initfree.init_text);
    execmem_free(initfree.init_data);
    execmem_free(initfree.init_rodata);
    kfree(initfree);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn flush_module_init_free_work() {
    flush_work(&init_free_wq);
    }

// Default value for module->async_probe_requested
    static bool async_probe;
    module_param!(async_probe, bool, 0644);
//
// This is where the real work happens.
//
// Keep it uninlined to provide a reliable breakpoint target, e.g. for the gdb
// helper command 'lx-symbols'.
//
#[no_mangle]
unsafe extern "C" fn do_init_module(mod: *mut module) -> noinline int {
pub static mut ret: c_int = 0;
pub static mut freeinit: *mut c_void = core::ptr::null_mut();

pub static mut text_size: c_uint = 0;
    for_each_mod_mem_type(type) {
    let mut mod_mem = &mod.mem[type];
    if (mod_mem.size) {
    total_size += mod_mem.size;
    if (type == MOD_TEXT || type == MOD_INIT_TEXT) {
    text_size += mod_mem.size;
    }
    }
    }

    freeinit = kmalloc_obj(*freeinit);
    if (!freeinit) {
    ret = -ENOMEM;
// goto;
    }
    freeinit.init_text = mod.mem[MOD_INIT_TEXT].base;
    freeinit.init_data = mod.mem[MOD_INIT_DATA].base;
    freeinit.init_rodata = mod.mem[MOD_INIT_RODATA].base;
    do_mod_ctors(mod);
// Start the module
    if (mod.init != core::ptr::null_mut()) {
    ret = do_one_initcall!(mod.init);
    }
    if (ret < 0) {
//
// -EEXIST is reserved by [f]init_module() to signal to userspace that
// a module with this name is already loaded. Use something else if the {
// module itself is returning that.
//
    if (ret == -EEXIST)
    ret = -EBUSY;
}
// goto;
    }
    if (ret > 0) {
    pr_warn!("%s: init suspiciously returned %d, it should follow 0/-E convention\n",
    mod.name, ret);
    }
// Now it's a first class citizen!
    mod.state = MODULE_STATE_LIVE;
    blocking_notifier_call_chain(&module_notify_list,
    MODULE_STATE_LIVE, mod);
// Delay uevent until module has finished its init routine
    kobject_uevent(&mod.mkobj.kobj, KOBJ_ADD);
//
// We need to finish all async code before the module init sequence
// is done. This has potential to deadlock if synchronous module
// loading is requested from async (which is not allowed!).
//
// See commit 0fdff3ec6d87 ("async, kmod: warn on synchronous
// request_module() from async workers") for more details.
//
    if (!mod.async_probe_requested) {
    async_synchronize_full();
    }
    ftrace_free_mem(mod, mod.mem[MOD_INIT_TEXT].base,
    mod.mem[MOD_INIT_TEXT].base + mod.mem[MOD_INIT_TEXT].size);
    mutex_lock(&module_mutex);
// Drop initial reference.
    module_put!(mod);
    trim_init_extable(mod);

// Switch to core kallsyms now init is done: kallsyms may be walking!
    rcu_assign_pointer(mod.kallsyms, &mod.core_kallsyms);

    ret = module_enable_rodata_ro_after_init!(mod);
    if (ret) {
    pr_warn!("%s: module_enable_rodata_ro_after_init!() returned %d, "
    "ro_after_init data might still be writable\n",
    mod.name, ret);
    }
    mod_tree_remove_init(mod);
    module_arch_freeing_init!(mod);
    for_class_mod_mem_type(type, init) {
    mod.mem[type].base = core::ptr::null_mut();
    mod.mem[type].size = 0;
    }

// .BTF is not SHF_ALLOC and will get removed, so sanitize pointers
    mod.btf_data = core::ptr::null_mut();
    mod.btf_base_data = core::ptr::null_mut();

//
// We want to free module_init, but be aware that kallsyms may be
// walking this within an RCU read section. In all the failure paths, we
// call synchronize_rcu(), but we don't want to slow down the success
// path. execmem_free() cannot be called in an interrupt, so do the
// work and call synchronize_rcu() in a work queue.
//
// Note that execmem_alloc() on most architectures creates W+X page
// mappings which won't be cleaned up until do_free_init() runs.  Any
// code such as mark_rodata_ro() which depends on those mappings to
// be cleaned up needs to sync with the queued work by invoking
// flush_module_init_free_work().
//
    if (llist_add(&freeinit.node, &init_free_list)) {
    schedule_work(&init_free_wq);
    }
    mutex_unlock(&module_mutex);
    wake_up_all(&module_wq);
    mod_stat_add_long(text_size, &total_text_size);
    mod_stat_add_long(total_size, &total_mod_size);
    mod_stat_inc(&modcount);
    return 0;
// label;
    kfree(freeinit);
// label;
// Try to protect us from buggy refcounters.
    mod.state = MODULE_STATE_GOING;
    synchronize_rcu();
    module_put!(mod);
    blocking_notifier_call_chain(&module_notify_list,
    MODULE_STATE_GOING, mod);
    klp_module_going(mod);
    ftrace_release_mod(mod);
    free_module(mod);
    wake_up_all(&module_wq);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn may_init_module() -> c_int {
    if (!capable(CAP_SYS_MODULE) || modules_disabled) {
    return -EPERM;
    }
    return 0;
    }
// Is this module of this name done loading?  No locks held.
#[no_mangle]
unsafe extern "C" fn finished_loading(name: *const c_char) -> bool {
pub static mut mod: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
//
// The module_mutex should not be a heavily contended lock;
// if we get the occasional sleep here, we'll go an extra iteration
// in the wait_event_interruptible(), which is harmless.
//
    sched_annotate_sleep();
    mutex_lock(&module_mutex);
    mod = find_module_all(name, strlen(name), true);
    ret = !mod || mod.state == MODULE_STATE_LIVE
    || mod.state == MODULE_STATE_GOING;
    mutex_unlock(&module_mutex);
    return ret;
    }
// Must be called with module_mutex held
    static int module_patient_check_exists!(const char *name,
    enum fail_dup_mod_reason reason)
    {
pub static mut old: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    old = find_module_all(name, strlen(name), true);
    if (old == core::ptr::null_mut()) {
    return 0;
    }
    if (old.state == MODULE_STATE_COMING ||
    old.state == MODULE_STATE_UNFORMED) {
// Wait in case it fails to load.
    mutex_unlock(&module_mutex);
    err = wait_event_interruptible(module_wq,
    finished_loading(name));
    mutex_lock(&module_mutex);
    if (err) {
    return err;
    }
// The module might have gone in the meantime.
    old = find_module_all(name, strlen(name), true);
    }
    if (try_add_failed_module(name, reason)) {
    pr_warn!("Could not add fail-tracking for module: %s\n", name);
    }
//
// We are here only when the same module was being loaded. Do
// not try to load it again right now. It prevents long delays
// caused by serialized module load failures. It might happen
// when more devices of the same type trigger load of
// a particular module.
//
    if (old && old.state == MODULE_STATE_LIVE) {
    return -EEXIST;
    }
    return -EBUSY;
    }
//
// We try to place it in the list now to make sure it's unique before
// we dedicate too many resources.  In particular, temporary percpu
// memory exhaustion.
//
#[no_mangle]
unsafe extern "C" fn add_unformed_module(mod: *mut module) -> c_int {
    let mut err = 0;
    mod.state = MODULE_STATE_UNFORMED;
    mutex_lock(&module_mutex);
    err = module_patient_check_exists!(mod.name, FAIL_DUP_MOD_LOAD);
    if (err) {
// goto;
    }
    mod_update_bounds(mod);
    list_add_rcu(&mod.list, &modules);
    mod_tree_insert(mod);
    err = 0;
// label;
    mutex_unlock(&module_mutex);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn complete_formation(mod: *mut module, info: *mut load_info) -> c_int {
    let mut err = 0;
    mutex_lock(&module_mutex);
// Find duplicate symbols (must be called under lock).
    err = verify_exported_symbols(mod);
    if (err < 0) {
// goto;
    }
// These rely on module_mutex for list integrity.
    module_bug_finalize!(info.hdr, info.sechdrs, mod);
    module_cfi_finalize!(info.hdr, info.sechdrs, mod);
    err = module_enable_rodata_ro!(mod);
    if (err) {
// goto;
    }
    err = module_enable_data_nx!(mod);
    if (err) {
// goto;
    }
    err = module_enable_text_rox!(mod);
    if (err) {
// goto;
    }
//
// Mark state as coming so strong_try_module_get() ignores us,
// but kallsyms etc. can see us.
//
    mod.state = MODULE_STATE_COMING;
    mutex_unlock(&module_mutex);
    return 0;
// label;
    module_bug_cleanup!(mod);
// label;
    mutex_unlock(&module_mutex);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn prepare_coming_module(mod: *mut module) -> c_int {
    let mut err = 0;
    ftrace_module_enable(mod);
    err = klp_module_coming(mod);
    if (err) {
    return err;
    }
    err = blocking_notifier_call_chain_robust(&module_notify_list,
    MODULE_STATE_COMING, MODULE_STATE_GOING, mod);
    err = notifier_to_errno(err);
    if (err) {
    klp_module_going(mod);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn unknown_module_param_cb(param: *mut c_char, val: *mut c_char, modname: *mut c_char, arg: *mut c_void) -> c_int {
    let mut mod = arg;
    let mut ret = 0;
    if (strcmp(param, "async_probe") == 0) {
    if (kstrtobool(val, &mod.async_probe_requested)) {
    mod.async_probe_requested = true;
    }
    return 0;
    }
// Check for magic 'dyndbg' arg
    ret = ddebug_dyndbg_module_param_cb(param, val, modname);
    if (ret != 0) {
    pr_warn!("%s: unknown parameter '%s' ignored\n", modname, param);
    }
    return 0;
    }
// Module within temporary copy, this doesn't do any allocation
#[no_mangle]
unsafe extern "C" fn early_mod_check(info: *mut load_info, flags: c_int) -> c_int {
    let mut err = 0;
//
// Now that we know we have the correct module name, check
// if it's blacklisted.
//
    if (blacklisted(info.name)) {
    pr_err!("Module %s is blacklisted\n", info.name);
    return -EPERM;
    }
    err = rewrite_section_headers(info, flags);
    if (err) {
    return err;
    }
// Check module struct version now, before we try to use module.
    if (!check_modstruct_version(info, info.mod)) {
    return -ENOEXEC;
    }
    err = check_modinfo(info.mod, info, flags);
    if (err) {
    return err;
    }
    mutex_lock(&module_mutex);
    err = module_patient_check_exists!(info.mod.name, FAIL_DUP_MOD_BECOMING);
    mutex_unlock(&module_mutex);
    return err;
    }
//
// Allocate and load the module: note that size of section 0 is always
// zero, and we rely on this for optional sections.
//
#[no_mangle]
pub unsafe extern "C" fn load_module(info: *mut load_info, uargs: *mut c_char, flags: c_int) -> c_int {
pub static mut mod: *mut c_void = core::ptr::null_mut();
pub static mut module_allocated: bool = false;
pub static mut err: c_long = 0;
    let mut args = core::ptr::null_mut(), *after_dashes;
//
// Do the signature check (if any) first. All that
// the signature check needs is info->len, it does
// not need any of the section info. That can be
// set up later. This will minimize the chances
// of a corrupt module causing problems before
// we even get to the signature check.
//
// The check will also adjust info->len by stripping
// off the sig length at the end of the module, making
// checks against info->len more correct.
//
    err = module_sig_check!(info, flags);
    if (err) {
// goto;
    }
//
// Do basic sanity checks against the ELF header and
// sections. Cache useful sections and set the
// info->mod to the userspace passed struct module.
//
    err = elf_validity_cache_copy(info, flags);
    if (err) {
// goto;
    }
    err = early_mod_check(info, flags);
    if (err) {
// goto;
    }
// Figure out module layout, and allocate all the memory.
    mod = layout_and_allocate(info, flags);
    if (IS_ERR(mod)) {
    err = PTR_ERR(mod);
// goto;
    }
    module_allocated = true;
    audit_log_kern_module(info.name);
// Reserve our place in the list.
    err = add_unformed_module(mod);
    if (err) {
// goto;
    }
//
// We are tainting your kernel if your module gets into
// the modules linked list somehow.
//
    module_augment_kernel_taints!(mod, info);
// To avoid stressing percpu allocator, do this once we're unique.
    err = percpu_modalloc(mod, info);
    if (err) {
// goto;
    }
// Now module is in final location, initialize linked lists, etc.
    err = module_unload_init!(mod);
    if (err) {
// goto;
    }
    init_param_lock(mod);
//
// Now we've got everything in the final locations, we can
// find optional sections.
//
    err = find_module_sections(mod, info);
    if (err) {
// goto;
    }
    err = check_export_symbol_sections(mod);
    if (err) {
// goto;
    }
// Set up MODINFO_ATTR fields
    err = setup_modinfo(mod, info);
    if (err) {
// goto;
    }
// Fix up syms, so that st_value is a pointer to location.
    err = simplify_symbols(mod, info);
    if (err < 0) {
// goto;
    }
    err = apply_relocations(mod, info);
    if (err < 0) {
// goto;
    }
    err = post_relocation(mod, info);
    if (err < 0) {
// goto;
    }
    flush_module_icache(mod);
// Now copy in args
    args = strndup_user(uargs, ~0UL >> 1);
    if (IS_ERR(args)) {
    err = PTR_ERR(args);
// goto;
    }
    init_build_id(mod, info);
// Ftrace init must be called in the MODULE_STATE_UNFORMED state
    ftrace_module_init(mod);
// Finally it's fully formed, ready to start executing.
    err = complete_formation(mod, info);
    if (err) {
// goto;
    }
    err = prepare_coming_module(mod);
    if (err) {
// goto;
    }
    mod.async_probe_requested = async_probe;
// Module is ready to execute: parsing args may do that.
    after_dashes = parse_args(mod.name, args, mod.kp, mod.num_kp,
    -32768, 32767, mod,
    unknown_module_param_cb);
    if (IS_ERR(after_dashes)) {
    err = PTR_ERR(after_dashes);
// goto;
    } else if (after_dashes) {
    pr_warn!("%s: parameters '%s' after `--' ignored\n",
    mod.name, after_dashes);
    }
    kfree(args);
    args = core::ptr::null_mut();
// Link in to sysfs.
    err = mod_sysfs_setup(mod, info, mod.kp, mod.num_kp);
    if (err < 0) {
// goto;
    }
    if (is_livepatch_module(mod)) {
    err = copy_module_elf(mod, info);
    if (err < 0) {
// goto;
    }
    }
    if (codetag_load_module(mod)) {
// goto;
    }
// Get rid of temporary copy.
    free_copy(info, flags);
// Done!
    trace_module_load(mod);
    return do_init_module(mod);
// label;
    mod_sysfs_teardown(mod);
// label;
    mod.state = MODULE_STATE_GOING;
    module_destroy_params!(mod.kp, mod.num_kp);
    blocking_notifier_call_chain(&module_notify_list,
    MODULE_STATE_GOING, mod);
    klp_module_going(mod);
// label;
    mod.state = MODULE_STATE_GOING;
// module_bug_cleanup needs module_mutex protection
    mutex_lock(&module_mutex);
    module_bug_cleanup!(mod);
    mutex_unlock(&module_mutex);
// label;
    ftrace_release_mod(mod);
    synchronize_rcu();
    kfree(args);
// label;
    module_arch_cleanup!(mod);
// label;
    free_modinfo(mod);
// label;
    module_unload_free!(mod);
// label;
    mutex_lock(&module_mutex);
// Unlink carefully: kallsyms could be walking list.
    list_del_rcu(&mod.list);
    mod_tree_remove(mod);
    wake_up_all(&module_wq);
// Wait for RCU-sched synchronizing before releasing mod->list.
    synchronize_rcu();
    mutex_unlock(&module_mutex);
// label;
    mod_stat_bump_invalid(info, flags);
    module_memory_restore_rox!(mod);
    module_deallocate!(mod, info);
// label;
//
// The info->len is always set. We distinguish between
// failures once the proper module was allocated and
// before that.
//
    if (!module_allocated) {
    audit_log_kern_module(info.name ? info.name : "?");
    mod_stat_bump_becoming(info, flags);
    }
    free_copy(info, flags);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_init_module(umod: usize, len: usize, uargs: usize) -> c_long {
    let mut err = 0;
pub static mut info: load_info = 0;
    err = may_init_module();
    if (err) {
    return err;
    }
    pr_debug!("init_module: umod=%p, len=%lu, uargs=%p\n",
    umod, len, uargs);
    err = copy_module_from_user(umod, len, &info);
    if (err) {
    mod_stat_inc(&failed_kreads);
    mod_stat_add_long(len, &invalid_kread_bytes);
    return err;
    }
    return load_module(&info, uargs, 0);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idempotent {
    pub cookie: *const c_void,
    pub entry: hlist_node,
    pub complete: completion,
    pub ret: c_int,
}

pub const IDEM_HASH_BITS: c_int = 8;
    static struct hlist_head idem_hash[1 << IDEM_HASH_BITS];
pub static mut idem_lock: usize = 0;
#[no_mangle]
unsafe extern "C" fn idempotent(u: *mut idempotent, cookie: *const c_void) -> bool {
pub static mut hash: c_int = 0;
    let mut head = idem_hash + hash;
pub static mut existing: *mut c_void = core::ptr::null_mut();
    let mut first = 0;
    u.ret = -EINTR;
    u.cookie = cookie;
    init_completion(&u.complete);
    spin_lock(&idem_lock);
    first = true;
    hlist_for_each_entry(existing, head, entry) {
    if (existing.cookie != cookie) {
    continue;
    }
    first = false;
    break;
    }
    hlist_add_head(&u.entry, idem_hash + hash);
    spin_unlock(&idem_lock);
    return !first;
    }
//
// We were the first one with 'cookie' on the list, and we ended
// up completing the operation. We now need to walk the list,
// remove everybody - which includes ourselves - fill in the return
// value, and then complete the operation.
//
#[no_mangle]
unsafe extern "C" fn idempotent_complete(u: *mut idempotent, ret: c_int) -> c_int {
    let mut cookie = u.cookie;
pub static mut hash: c_int = 0;
    let mut head = idem_hash + hash;
pub static mut next: *mut c_void = core::ptr::null_mut();
pub static mut pos: *mut c_void = core::ptr::null_mut();
    spin_lock(&idem_lock);
    hlist_for_each_entry_safe(pos, next, head, entry) {
    if (pos.cookie != cookie) {
    continue;
    }
    hlist_del_init(&pos.entry);
    pos.ret = ret;
    complete(&pos.complete);
    }
    spin_unlock(&idem_lock);
    return ret;
    }
//
// Wait for the idempotent worker.
//
// If we get interrupted, we need to remove ourselves from the
// the idempotent list, and the completion may still come in.
//
// The 'idem_lock' protects against the race, and 'idem.ret' was
// initialized to -EINTR and is thus always the right return
// value even if the idempotent work then completes between
// the wait_for_completion and the cleanup.
//
#[no_mangle]
unsafe extern "C" fn idempotent_wait_for_completion(u: *mut idempotent) -> c_int {
    if (wait_for_completion_interruptible(&u.complete)) {
    spin_lock(&idem_lock);
    if (!hlist_unhashed(&u.entry)) {
    hlist_del(&u.entry);
    }
    spin_unlock(&idem_lock);
    }
    return u.ret;
    }
#[no_mangle]
unsafe extern "C" fn init_module_from_file(f: *mut file, uargs: *const *const char , flags: c_int) -> c_int {
pub static mut compressed: bool = false;
pub static mut info: load_info = 0;
    let mut buf = core::ptr::null_mut();
    let mut len = 0;
    let mut err = 0;
    len = kernel_read_file(f, 0, &buf, INT_MAX, core::ptr::null_mut(),
    compressed ? READING_MODULE_COMPRESSED :
    READING_MODULE);
    if (len < 0) {
    mod_stat_inc(&failed_kreads);
    return len;
    }
    if (compressed) {
    err = module_decompress!(&info, buf, len);
    vfree(buf); /* compressed data is no longer needed */
    if (err) {
    mod_stat_inc(&failed_decompress);
    mod_stat_add_long(len, &invalid_decompress_bytes);
    return err;
    }
    err = security_kernel_post_read_file(f, info.hdr, info.len,
    READING_MODULE);
    if (err) {
    mod_stat_inc(&failed_kreads);
    free_copy(&info, flags);
    return err;
    }
    } else {
    info.hdr = buf;
    info.len = len;
    }
    return load_module(&info, uargs, flags);
    }
#[no_mangle]
unsafe extern "C" fn idempotent_init_module(f: *mut file, uargs: *const *const char , flags: c_int) -> c_int {
pub static mut idem: usize = 0;
    if (!(f.f_mode & FMODE_READ)) {
    return -EBADF;
    }
// Are we the winners of the race and get to do this?
    if (!idempotent(&idem, file_inode(f))) {
pub static mut ret: c_int = 0;
    return idempotent_complete(&idem, ret);
    }
//
// Somebody else won the race and is loading the module.
//
    return idempotent_wait_for_completion(&idem);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_finit_module(fd: usize, uargs: usize, flags: usize) -> c_long {
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    pr_debug!("finit_module: fd=%d, uargs=%p, flags=%i\n", fd, uargs, flags);
    if (flags & ~(MODULE_INIT_IGNORE_MODVERSIONS
    |MODULE_INIT_IGNORE_VERMAGIC
    |MODULE_INIT_COMPRESSED_FILE)) {
    return -EINVAL;
    }
    CLASS(fd, f)(fd);
    if (fd_empty(f)) {
    return -EBADF;
    }
    return idempotent_init_module(fd_file(f), uargs, flags);
    }
// Keep in sync with MODULE_FLAGS_BUF_SIZE !!!
    char *module_flags!(module *mod, char *buf, bool show_state)
    {
pub static mut bx: c_int = 0;
    BUG_ON!(mod.state == MODULE_STATE_UNFORMED);
    if (!mod.taints && !show_state) {
// goto;
    }
    if (mod.taints ||
    mod.state == MODULE_STATE_GOING ||
    mod.state == MODULE_STATE_COMING) {
    buf[bx++] = '(';
    bx += module_flags_taint!(mod.taints, buf + bx);
// Show a - for module-is-being-unloaded
    if (mod.state == MODULE_STATE_GOING && show_state) {
    buf[bx++] = '-';
    }
// Show a + for module-is-being-loaded
    if (mod.state == MODULE_STATE_COMING && show_state) {
    buf[bx++] = '+';
    }
    buf[bx++] = ')';
    }
// label;
    buf[bx] = '\0';
    return buf;
    }
// Given an address, look for it in the module exception tables.
    const struct exception_table_entry *search_module_extables(unsigned long addr)
    {
pub static mut mod: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    mod = __module_address(addr);
    if (!mod) {
    return core::ptr::null_mut();
    }
    if (!mod.num_exentries) {
    return core::ptr::null_mut();
    }
//
// The address passed here belongs to a module that is currently
// invoked (we are running inside it). Therefore its module::refcnt
// needs already be >0 to ensure that it is not removed at this stage.
// All other user need to invoke this function within a RCU read
// section.
//
    return search_extable(mod.extable, mod.num_exentries, addr);
    }
//
// is_module_address() - is this address inside a module?
// @addr: the address to check.
//
// See is_module_text_address() if you simply want to see if the address
// is code (not data).
//
#[no_mangle]
pub unsafe extern "C" fn is_module_address(addr: c_ulong) -> bool {
    guard(rcu)();
    return __module_address(addr) != core::ptr::null_mut();
    }
//
// __module_address() - get the module which contains an address.
// @addr: the address.
//
// Must be called within RCU read section or module mutex held so that
// module doesn't get freed during this.
//
#[no_mangle]
pub unsafe extern "C" fn __module_address(addr: c_ulong) -> *mut c_void {
pub static mut mod: *mut c_void = core::ptr::null_mut();
    if (addr >= mod_tree.addr_min && addr <= mod_tree.addr_max) {
// goto;
    }

    if (addr >= mod_tree.data_addr_min && addr <= mod_tree.data_addr_max) {
// goto;
    }

    return core::ptr::null_mut();
// label;
    mod = mod_find(addr, &mod_tree);
    if (mod) {
    BUG_ON!(!within_module(addr, mod));
    if (mod.state == MODULE_STATE_UNFORMED) {
    mod = core::ptr::null_mut();
    }
    }
    return mod;
    }
//
// is_module_text_address() - is this address inside module code?
// @addr: the address to check.
//
// See is_module_address() if you simply want to see if the address is
// anywhere in a module.  See kernel_text_address() for testing if an
// address corresponds to kernel or module code.
//
#[no_mangle]
pub unsafe extern "C" fn is_module_text_address(addr: c_ulong) -> bool {
    guard(rcu)();
    return __module_text_address(addr) != core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn module_for_each_mod!(mod: *mut *mut int(func)(module, data): *mut c_void, data: *mut c_void) {
    void module_for_each_mod!(int(*func)(module *mod, void *data), void *data)
    {
pub static mut mod: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    list_for_each_entry_rcu(mod, &modules, list) {
    if (mod.state == MODULE_STATE_UNFORMED) {
    continue;
    }
    if (func(mod, data)) {
    break;
    }
    }
    }
//
// __module_text_address() - get the module whose code contains an address.
// @addr: the address.
//
// Must be called within RCU read section or module mutex held so that
// module doesn't get freed during this.
//
#[no_mangle]
pub unsafe extern "C" fn __module_text_address(addr: c_ulong) -> *mut c_void {
    let mut mod = __module_address(addr);
    if (mod) {
// Make sure it's within the text section.
    if (!within_module_mem_type(addr, mod, MOD_TEXT) &&
    !within_module_mem_type(addr, mod, MOD_INIT_TEXT)) {
    mod = core::ptr::null_mut();
    }
    }
    return mod;
    }
// Don't grab lock, we're oopsing.
#[no_mangle]
pub unsafe extern "C" fn print_modules() {
pub static mut mod: *mut c_void = core::ptr::null_mut();
    char buf[MODULE_FLAGS_BUF_SIZE];
    printk("Modules linked in:");
// Most callers should already have preempt disabled, but make sure
    guard(rcu)();
    list_for_each_entry_rcu(mod, &modules, list) {
    if (mod.state == MODULE_STATE_UNFORMED) {
    continue;
    }
    pr_cont(" %s%s", mod.name, module_flags!(mod, buf, true));
    }
    print_unloaded_tainted_modules();
    if (last_unloaded_module.name[0]) {
    pr_cont(" [last unloaded: %s%s]", last_unloaded_module.name,
    last_unloaded_module.taints);
    }
    pr_cont("\n");
    }

pub static mut mod_debugfs_root: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn module_debugfs_init!() -> c_int {
    mod_debugfs_root = debugfs_create_dir("modules", core::ptr::null_mut());
    return 0;
    }
    module_init!(module_debugfs_init);
}
