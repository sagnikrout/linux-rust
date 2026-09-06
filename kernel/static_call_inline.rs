//! Automatically rewritten from C to Rust
//! Source: kernel/static_call_inline.c
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



// SPDX-License-Identifier: GPL-2.0

    extern struct static_call_site __start_static_call_sites[],
    __stop_static_call_sites[];
    extern struct static_call_tramp_key __start_static_call_tramp_key[],
    __stop_static_call_tramp_key[];
    let mut static_call_initialized = 0;
//
// Must be called before early_initcall!() to be effective.
//
#[no_mangle]
pub unsafe extern "C" fn static_call_force_reinit() {
    if (WARN_ON_ONCE!(!static_call_initialized)) {
    return;
    }
    static_call_initialized += 1;
    }
// mutex to protect key modules/sites
// static DEFINE_MUTEX(static_call_mutex);
#[no_mangle]
unsafe extern "C" fn static_call_lock() {
    mutex_lock(&static_call_mutex);
    }
#[no_mangle]
unsafe extern "C" fn static_call_unlock() {
    mutex_unlock(&static_call_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn static_call_addr(site: *mut static_call_site) -> *mut c_void {
    return ((long)site.addr + (long)&site.addr);
    }
#[no_mangle]
pub unsafe extern "C" fn __static_call_key(site: *const static_call_site) -> c_ulong {
    return (long)site.key + (long)&site.key;
    }
#[no_mangle]
pub unsafe extern "C" fn static_call_key(site: *mut static_call_site) -> *mut c_void {
    return (__static_call_key(site) & ~STATIC_CALL_SITE_FLAGS);
    }
// These assume the key is word-aligned.
#[no_mangle]
pub unsafe extern "C" fn static_call_is_init(site: *mut static_call_site) -> bool {
    return __static_call_key(site) & STATIC_CALL_SITE_INIT;
    }
#[no_mangle]
pub unsafe extern "C" fn static_call_is_tail(site: *mut static_call_site) -> bool {
    return __static_call_key(site) & STATIC_CALL_SITE_TAIL;
    }
#[no_mangle]
pub unsafe extern "C" fn static_call_set_init(site: *mut static_call_site) {
    site.key = (__static_call_key(site) | STATIC_CALL_SITE_INIT) -
    (long)&site.key;
    }
#[no_mangle]
unsafe extern "C" fn static_call_site_cmp(_a: *const c_void, _b: *const c_void) -> c_int {
    let mut a = _a;
    let mut b = _b;
    let mut key_a = static_call_key(a);
    let mut key_b = static_call_key(b);
    if (key_a < key_b) {
    return -1;
    }
    if (key_a > key_b) {
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn static_call_site_swap(_a: *mut c_void, _b: *mut c_void, size: c_int) {
pub static mut delta: c_long = 0;
    let mut a = _a;
    let mut b = _b;
pub static mut tmp: static_call_site = 0;
    a.addr = b.addr  - delta;
    a.key  = b.key   - delta;
    b.addr = tmp.addr + delta;
    b.key  = tmp.key  + delta;
    }
#[no_mangle]
pub unsafe extern "C" fn static_call_sort_entries(start: *mut static_call_site, stop: *mut static_call_site) {
    sort(start, stop - start, sizeof!(static_call_site),
    static_call_site_cmp, static_call_site_swap);
    }
#[no_mangle]
pub unsafe extern "C" fn static_call_key_has_mods(key: *mut static_call_key) -> bool {
    return !(key.type & 1);
    }
#[no_mangle]
pub unsafe extern "C" fn static_call_key_next(key: *mut static_call_key) -> *mut c_void {
    if (!static_call_key_has_mods(key)) {
    return core::ptr::null_mut();
    }
    return key.mods;
    }
#[no_mangle]
pub unsafe extern "C" fn static_call_key_sites(key: *mut static_call_key) -> *mut c_void {
    if (static_call_key_has_mods(key)) {
    return core::ptr::null_mut();
    }
    return (key.type & ~1);
    }
#[no_mangle]
pub unsafe extern "C" fn __static_call_update(key: *mut static_call_key, tramp: *mut c_void, func: *mut c_void) {
    let mut site = core::ptr::null_mut();
    let mut stop = core::ptr::null_mut();
    struct static_call_mod *site_mod, first;
    cpus_read_lock();
    static_call_lock();
    if (key.func == func) {
// goto;
    }
    key.func = func;
    arch_static_call_transform(core::ptr::null_mut(), tramp, func, false);
//
// If uninitialized, we'll not update the callsites, but they still
// point to the trampoline and we just patched that.
//
    if (WARN_ON_ONCE!(!static_call_initialized)) {
// goto;
    }
    first = (static_call_mod){
    .next = static_call_key_next(key),
    .mod = core::ptr::null_mut(),
    .sites = static_call_key_sites(key),
    };
    while (site_mod) {
pub static mut init: bool = false;
    let mut mod = site_mod.mod;
    if (!site_mod.sites) {
//
// This can happen if the static call key is defined in
// a module which doesn't use it.
//
// It also happens in the has_mods case, where the
// 'first' entry has no sites associated with it.
//
    continue;
    }
    stop = __stop_static_call_sites;
    if (mod) {

    stop = mod.static_call_sites +
    mod.num_static_call_sites;
    init = mod.state == MODULE_STATE_COMING;

    }
    while (site < stop && static_call_key(site) == key) {
    let mut site_addr = static_call_addr(site);
    if (!init && static_call_is_init(site)) {
    continue;
    }
    if (!kernel_text_address((unsigned long)site_addr)) {
//
// This skips patching built-in __exit, which
// is part of init_section_contains() but is
// not part of kernel_text_address().
//
// Skipping built-in __exit is fine since it
// will never be executed.
//
    WARN_ONCE(!static_call_is_init(site),
    "can't patch static call site at %pS",
    site_addr);
    continue;
    }
    arch_static_call_transform(site_addr, tramp, func,
    static_call_is_tail(site));
    }
    }
// label;
    static_call_unlock();
    cpus_read_unlock();
    }
    EXPORT_SYMBOL_GPL(__static_call_update);
#[no_mangle]
pub unsafe extern "C" fn __static_call_init(mod: *mut module, start: *mut static_call_site, stop: *mut static_call_site) -> c_int {
pub static mut site: *mut c_void = core::ptr::null_mut();
    struct static_call_key *key, *prev_key = core::ptr::null_mut();
pub static mut site_mod: *mut c_void = core::ptr::null_mut();
    if (start == stop) {
    return 0;
    }
    static_call_sort_entries(start, stop);
    while (site < stop) {
    let mut site_addr = static_call_addr(site);
    if ((mod && within_module_init((unsigned long)site_addr, mod)) ||
    (!mod && init_section_contains(site_addr, 1))) {
    static_call_set_init(site);
    }
    key = static_call_key(site);
    if (key != prev_key) {
    prev_key = key;
//
// For vmlinux (!mod) avoid the allocation by storing
// the sites pointer in the key itself. Also see
// __static_call_update()'s @first.
//
// This allows architectures (eg. x86) to call
// static_call_init() before memory allocation works.
//
    if (!mod) {
    key.sites = site;
    key.type |= 1;
// goto;
    }
    site_mod = kzalloc_obj(*site_mod);
    if (!site_mod) {
    return -ENOMEM;
    }
//
// When the key has a direct sites pointer, extract
// that into an explicit struct static_call_mod, so we
// can have a list of modules.
//
    if (static_call_key_sites(key)) {
    site_mod.mod = core::ptr::null_mut();
    site_mod.next = core::ptr::null_mut();
    site_mod.sites = static_call_key_sites(key);
    key.mods = site_mod;
    site_mod = kzalloc_obj(*site_mod);
    if (!site_mod) {
    return -ENOMEM;
    }
    }
    site_mod.mod = mod;
    site_mod.sites = site;
    site_mod.next = static_call_key_next(key);
    key.mods = site_mod;
    }
// label;
    arch_static_call_transform(site_addr, core::ptr::null_mut(), key.func,
    static_call_is_tail(site));
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn addr_conflict(site: *mut static_call_site, start: *mut c_void, end: *mut c_void) -> c_int {
pub static mut addr: c_ulong = 0;
    if (addr <= (unsigned long)end &&
    addr + CALL_INSN_SIZE > (unsigned long)start) {
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __static_call_text_reserved(iter_start: *mut static_call_site, iter_stop: *mut static_call_site, start: *mut c_void, end: *mut c_void, init: bool) -> c_int {
    let mut iter = iter_start;
    while (iter < iter_stop) {
    if (init || !static_call_is_init(iter)) {
    if (addr_conflict(iter, start, end)) {
    return 1;
    }
    }
    iter += 1;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn __static_call_mod_text_reserved(start: *mut c_void, end: *mut c_void) -> c_int {
pub static mut mod: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    scoped_guard(rcu) {
    mod = __module_text_address((unsigned long)start);
    WARN_ON_ONCE!(__module_text_address((unsigned long)end) != mod);
    if (!try_module_get(mod)) {
    mod = core::ptr::null_mut();
    }
    }
    if (!mod) {
    return 0;
    }
    ret = __static_call_text_reserved(mod.static_call_sites,
    mod.static_call_sites + mod.num_static_call_sites,
    start, end, mod.state == MODULE_STATE_COMING);
    module_put!(mod);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tramp_key_lookup(addr: c_ulong) -> c_ulong {
    let mut start = __start_static_call_tramp_key;
    let mut stop = __stop_static_call_tramp_key;
pub static mut tramp_key: *mut c_void = core::ptr::null_mut();
    while (tramp_key != stop) {
    let mut tramp = 0;
    tramp = (long)tramp_key.tramp + (long)&tramp_key.tramp;
    if (tramp == addr) {
    return (long)tramp_key.key + (long)&tramp_key.key;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn static_call_add_module(mod: *mut module) -> c_int {
    let mut start = mod.static_call_sites;
    let mut stop = start + mod.num_static_call_sites;
pub static mut site: *mut c_void = core::ptr::null_mut();
    while (site != stop) {
pub static mut s_key: c_ulong = 0;
pub static mut addr: c_ulong = 0;
    let mut key = 0;
//
// Is the key is exported, 'addr' points to the key, which
// means modules are allowed to call static_call_update() on
// it.
//
// Otherwise, the key isn't exported, and 'addr' points to the
// trampoline so we need to lookup the key.
//
// We go through this dance to prevent crazy modules from
// abusing sensitive static calls.
//
    if (!kernel_text_address(addr)) {
    continue;
    }
    key = tramp_key_lookup(addr);
    if (!key) {
    pr_warn!("Failed to fixup __raw_static_call() usage at: %ps\n",
    static_call_addr(site));
    return -EINVAL;
    }
    key |= s_key & STATIC_CALL_SITE_FLAGS;
    site.key = key - (long)&site.key;
    }
    return __static_call_init(mod, start, stop);
    }
#[no_mangle]
unsafe extern "C" fn static_call_del_module(mod: *mut module) {
    let mut start = mod.static_call_sites;
    let mut stop = mod.static_call_sites +
    mod.num_static_call_sites;
    struct static_call_key *key, *prev_key = core::ptr::null_mut();
    let mut site_mod = core::ptr::null_mut();
    let mut prev = core::ptr::null_mut();
pub static mut site: *mut c_void = core::ptr::null_mut();
    while (site < stop) {
    key = static_call_key(site);
//
// If the key was not updated due to a memory allocation
// failure in __static_call_init() then treating key::sites
// as key::mods in the code below would cause random memory
// access and #GP. In that case all subsequent sites have
// not been touched either, so stop iterating.
//
    if (!static_call_key_has_mods(key)) {
    break;
    }
    if (key == prev_key) {
    continue;
    }
    prev_key = key;
    for (prev = &key.mods, site_mod = key.mods;
    site_mod && site_mod.mod != mod;
    prev = &site_mod.next, site_mod = site_mod.next) {
    ;
    }
    if (!site_mod) {
    continue;
    }
// prev = site_mod->next;
    kfree(site_mod);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn static_call_module_notify(nb: *mut notifier_block, val: c_ulong, data: *mut c_void) -> c_int {
    let mut mod = data;
pub static mut ret: c_int = 0;
    cpus_read_lock();
    static_call_lock();
    match (val) {
    MODULE_STATE_COMING => {
    ret = static_call_add_module(mod);
    if (ret) {
    pr_warn!("Failed to allocate memory for static calls\n");
    static_call_del_module(mod);
    }
    // break;
    }
    MODULE_STATE_GOING => {
    static_call_del_module(mod);
    // break;
    }
    }
    static_call_unlock();
    cpus_read_unlock();
    return notifier_from_errno(ret);
    }
pub static mut notifier_block: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn __static_call_mod_text_reserved(start: *mut c_void, end: *mut c_void) -> c_int {
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn static_call_text_reserved(start: *mut c_void, end: *mut c_void) -> c_int {
pub static mut init: bool = false;
    let mut ret = __static_call_text_reserved(__start_static_call_sites,
    __stop_static_call_sites, start, end, init);
    if (ret) {
    return ret;
    }
    return __static_call_mod_text_reserved(start, end);
    }
#[no_mangle]
pub unsafe extern "C" fn static_call_init() -> c_int {
    let mut ret = 0;
// See static_call_force_reinit().
    if (static_call_initialized == 1) {
    return 0;
    }
    cpus_read_lock();
    static_call_lock();
    ret = __static_call_init(core::ptr::null_mut(), __start_static_call_sites,
    __stop_static_call_sites);
    static_call_unlock();
    cpus_read_unlock();
    if (ret) {
    pr_err!("Failed to allocate memory for static_call!\n");
    BUG();
    }

    if (!static_call_initialized) {
    register_module_notifier(&static_call_module_nb);
    }

    static_call_initialized = 1;
    return 0;
    }
    early_initcall!(static_call_init);

#[no_mangle]
unsafe extern "C" fn func_a(x: c_int) -> c_int {
    return x+1;
    }
#[no_mangle]
unsafe extern "C" fn func_b(x: c_int) -> c_int {
    return x+2;
    }
pub static mut sc_selftest: usize = 0;
    static struct static_call_data {
    int (*func)(int);
    let mut val = 0;
    let mut expect = 0;
    } static_call_data [] __initdata = {
    { core::ptr::null_mut(),   2, 3 },
    { func_b, 2, 4 },
    { func_a, 2, 3 }
    };
#[no_mangle]
unsafe extern "C" fn test_static_call_init() -> c_int {
    let mut i = 0;
    while (i < ARRAY_SIZE!(static_call_data)) {
    let mut scd = &static_call_data[i];
    if (scd.func) {
    static_call_update(sc_selftest, scd.func);
    }
    WARN_ON!(static_call(sc_selftest)(scd.val) != scd.expect);
    }
    return 0;
    }
    early_initcall!(test_static_call_init);