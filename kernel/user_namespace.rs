//! Automatically rewritten from C to Rust
//! Source: kernel/user_namespace.c
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


// SPDX-License-Identifier: GPL-2.0-only

pub static mut user_ns_cachep: *mut c_void = core::ptr::null_mut();
pub static mut userns_state_mutex: usize = 0;
// forward_decl: new_idmap_permitted;
// forward_decl: free_user_ns;
#[no_mangle]
pub unsafe extern "C" fn inc_user_namespaces(ns: *mut user_namespace, uid: kuid_t) -> *mut c_void {
    return inc_ucount(ns, uid, UCOUNT_USER_NAMESPACES);
    }
#[no_mangle]
unsafe extern "C" fn dec_user_namespaces(ucounts: *mut ucounts) {
    return dec_ucount(ucounts, UCOUNT_USER_NAMESPACES);
    }
#[no_mangle]
unsafe extern "C" fn set_cred_user_ns(cred: *mut cred, user_ns: *mut user_namespace) {
// Start with the same capabilities as init but useless for doing
// anything as the capabilities are bound to the new user namespace.
//
    cred.securebits = SECUREBITS_DEFAULT;
    cred.cap_inheritable = CAP_EMPTY_SET;
    cred.cap_permitted = CAP_FULL_SET;
    cred.cap_effective = CAP_FULL_SET;
    cred.cap_ambient = CAP_EMPTY_SET;
    cred.cap_bset = CAP_FULL_SET;

    key_put(cred.request_key_auth);
    cred.request_key_auth = core::ptr::null_mut();

// tgcred will be cleared in our caller bc CLONE_THREAD won't be set
    cred.user_ns = user_ns;
    }
#[no_mangle]
unsafe extern "C" fn enforced_nproc_rlimit() -> c_ulong {
pub static mut limit: c_ulong = 0;
// Is RLIMIT_NPROC currently enforced?
    if (!uid_eq(current_uid(), GLOBAL_ROOT_UID) ||
    (current_user_ns() != &init_user_ns)) {
    limit = rlimit(RLIMIT_NPROC);
    }
    return limit;
    }
//
// Create a new user namespace, deriving the creator from the user in the
// passed credentials, and replacing that user with the new root user for the
// new namespace.
//
// This is called by copy_creds(), which will finish setting the target task's
// credentials.
//
#[no_mangle]
pub unsafe extern "C" fn create_user_ns(new: *mut cred) -> c_int {
    struct user_namespace *ns, *parent_ns = new.user_ns;
pub static mut owner: kuid_t = 0;
pub static mut group: kgid_t = 0;
pub static mut ucounts: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut i = 0;
    ret = -ENOSPC;
    if (parent_ns.level > 32) {
// goto;
    }
    ucounts = inc_user_namespaces(parent_ns, owner);
    if (!ucounts) {
// goto;
    }
//
// Verify that we can not violate the policy of which files
// may be accessed that is specified by the root directory,
// by verifying that the root directory is at the root of the
// mount namespace which allows all files to be accessed.
//
    ret = -EPERM;
    if (current_chrooted()) {
// goto;
    }
// The creator needs a mapping in the parent user namespace
// or else we won't be able to reasonably tell userspace who
// created a user_namespace.
//
    ret = -EPERM;
    if (!kuid_has_mapping(parent_ns, owner) ||
    !kgid_has_mapping(parent_ns, group)) {
// goto;
    }
    ret = security_create_user_ns(new);
    if (ret < 0) {
// goto;
    }
    ret = -ENOMEM;
    ns = kmem_cache_zalloc(user_ns_cachep, GFP_KERNEL);
    if (!ns) {
// goto;
    }
    ns.parent_could_setfcap = cap_raised(new.cap_effective, CAP_SETFCAP);
    ret = ns_common_init(ns);
    if (ret) {
// goto;
    }
// Leave the new->user_ns reference with the new user namespace.
    ns.parent = parent_ns;
    ns.level = parent_ns.level + 1;
    ns.owner = owner;
    ns.group = group;
    INIT_WORK(&ns.work, free_user_ns);
    while (i < UCOUNT_COUNTS) {
    ns.ucount_max[i] = INT_MAX;
    }
    set_userns_rlimit_max(ns, UCOUNT_RLIMIT_NPROC, enforced_nproc_rlimit());
    set_userns_rlimit_max(ns, UCOUNT_RLIMIT_MSGQUEUE, rlimit(RLIMIT_MSGQUEUE));
    set_userns_rlimit_max(ns, UCOUNT_RLIMIT_SIGPENDING, rlimit(RLIMIT_SIGPENDING));
    set_userns_rlimit_max(ns, UCOUNT_RLIMIT_MEMLOCK, rlimit(RLIMIT_MEMLOCK));
    ns.ucounts = ucounts;
// Inherit USERNS_SETGROUPS_ALLOWED from our parent
    mutex_lock(&userns_state_mutex);
    ns.flags = parent_ns.flags;
    mutex_unlock(&userns_state_mutex);

    INIT_LIST_HEAD(&ns.keyring_name_list);
    init_rwsem(&ns.keyring_sem);

    ret = -ENOMEM;
    if (!setup_userns_sysctls(ns)) {
// goto;
    }
    set_cred_user_ns(new, ns);
    ns_tree_add(ns);
    return 0;
// label;
    key_put(ns.persistent_keyring_register);

    ns_common_free(ns);
// label;
    kmem_cache_free(user_ns_cachep, ns);
// label;
    dec_user_namespaces(ucounts);
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn unshare_userns(unshare_flags: c_ulong, new_cred: *mut cred) -> c_int {
pub static mut cred: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    if (!(unshare_flags & CLONE_NEWUSER)) {
    return 0;
    }
    cred = prepare_creds();
    if (cred) {
    err = create_user_ns(cred);
    if (err) {
    put_cred(cred);
    }
    else {
// new_cred = cred;
    }
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn free_user_ns(work: *mut work_struct) {
    struct user_namespace *parent, *ns =
    container_of!(work, user_namespace, work);
    do {
    let mut ucounts = ns.ucounts;
    parent = ns.parent;
    ns_tree_remove(ns);
    if (ns.gid_map.nr_extents > UID_GID_MAP_MAX_BASE_EXTENTS) {
    kfree(ns.gid_map.forward);
    kfree(ns.gid_map.reverse);
    }
    if (ns.uid_map.nr_extents > UID_GID_MAP_MAX_BASE_EXTENTS) {
    kfree(ns.uid_map.forward);
    kfree(ns.uid_map.reverse);
    }
    if (ns.projid_map.nr_extents > UID_GID_MAP_MAX_BASE_EXTENTS) {
    kfree(ns.projid_map.forward);
    kfree(ns.projid_map.reverse);
    }

    kfree(ns.binfmt_misc);

    retire_userns_sysctls(ns);
    key_free_user_ns(ns);
    ns_common_free(ns);
// Concurrent nstree traversal depends on a grace period.
    kfree_rcu(ns, ns.ns_rcu);
    dec_user_namespaces(ucounts);
    ns = parent;
    } while (ns_ref_put(parent));
    }
#[no_mangle]
pub unsafe extern "C" fn __put_user_ns(ns: *mut user_namespace) {
    schedule_work(&ns.work);
    }
    EXPORT_SYMBOL(__put_user_ns);
//
// struct idmap_key - holds the information necessary to find an idmapping in a
// sorted idmap array. It is passed to cmp_map_id() as first argument.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idmap_key {
//     pub /: *mut *mut bool map_up; / true -> id from kid; false -> kid from id,
//     pub /: *mut *mut u32 id; / id to find,
    pub count: u32,
}

//
// cmp_map_id - Function to be passed to bsearch() to find the requested
// idmapping. Expects struct idmap_key to be passed via @k.
//
#[no_mangle]
unsafe extern "C" fn cmp_map_id(k: *const c_void, e: *const c_void) -> c_int {
    u32 first, last, id2;
    let mut key = k;
    let mut el = e;
    id2 = key.id + key.count - 1;
// handle map_id_{down,up}()
    if (key.map_up) {
    first = el.lower_first;
    }
    else {
    first = el.first;
    }
    last = first + el.count - 1;
    if (key.id >= first && key.id <= last &&
    (id2 >= first && id2 <= last)) {
    return 0;
    }
    if (key.id < first || id2 < first) {
    return -1;
    }
    return 1;
    }
//
// map_id_range_down_max - Find idmap via binary search in ordered idmap array.
// Can only be called if number of mappings exceeds UID_GID_MAP_MAX_BASE_EXTENTS.
//
#[no_mangle]
pub unsafe extern "C" fn map_id_range_down_max(extents: c_uint, map: *mut uid_gid_map, id: u32, count: u32) -> *mut c_void {
pub static mut key: usize = 0;
    key.map_up = false;
    key.count = count;
    key.id = id;
    return bsearch(&key, map.forward, extents,
    sizeof!(uid_gid_extent), cmp_map_id);
    }
//
// map_id_range_down_base - Find idmap via binary search in static extent array.
// Can only be called if number of mappings is equal or less than
// UID_GID_MAP_MAX_BASE_EXTENTS.
//
#[no_mangle]
pub unsafe extern "C" fn map_id_range_down_base(extents: c_uint, map: *mut uid_gid_map, id: u32, count: u32) -> *mut c_void {
    let mut idx: c_uint = 0;
    u32 first, last, id2;
    id2 = id + count - 1;
// Find the matching extent
    while (idx < extents) {
    first = map.extent[idx].first;
    last = first + map.extent[idx].count - 1;
    if (id >= first && id <= last &&
    (id2 >= first && id2 <= last)) {
    return &map.extent[idx];
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn map_id_range_down(map: *mut uid_gid_map, id: u32, count: u32) -> u32 {
pub static mut extent: *mut c_void = core::ptr::null_mut();
pub static mut extents: unsigned = 0;
    smp_rmb();
    if (extents <= UID_GID_MAP_MAX_BASE_EXTENTS) {
    extent = map_id_range_down_base(extents, map, id, count);
    }
    else {
    extent = map_id_range_down_max(extents, map, id, count);
    }
// Map the id or note failure
    if (extent) {
    id = (id - extent.first) + extent.lower_first;
    }
    else {
    id = (u32) -1;
    }
    return id;
    }
#[no_mangle]
pub unsafe extern "C" fn map_id_down(map: *mut uid_gid_map, id: u32) -> u32 {
    return map_id_range_down(map, id, 1);
    }
//
// map_id_up_base - Find idmap via binary search in static extent array.
// Can only be called if number of mappings is equal or less than
// UID_GID_MAP_MAX_BASE_EXTENTS.
//
#[no_mangle]
pub unsafe extern "C" fn map_id_range_up_base(extents: c_uint, map: *mut uid_gid_map, id: u32, count: u32) -> *mut c_void {
    let mut idx: c_uint = 0;
    u32 first, last, id2;
    id2 = id + count - 1;
// Find the matching extent
    while (idx < extents) {
    first = map.extent[idx].lower_first;
    last = first + map.extent[idx].count - 1;
    if (id >= first && id <= last &&
    (id2 >= first && id2 <= last)) {
    return &map.extent[idx];
    }
    }
    return core::ptr::null_mut();
    }
//
// map_id_up_max - Find idmap via binary search in ordered idmap array.
// Can only be called if number of mappings exceeds UID_GID_MAP_MAX_BASE_EXTENTS.
//
#[no_mangle]
pub unsafe extern "C" fn map_id_range_up_max(extents: c_uint, map: *mut uid_gid_map, id: u32, count: u32) -> *mut c_void {
pub static mut key: usize = 0;
    key.map_up = true;
    key.count = count;
    key.id = id;
    return bsearch(&key, map.reverse, extents,
    sizeof!(uid_gid_extent), cmp_map_id);
    }
#[no_mangle]
pub unsafe extern "C" fn map_id_range_up(map: *mut uid_gid_map, id: u32, count: u32) -> u32 {
pub static mut extent: *mut c_void = core::ptr::null_mut();
pub static mut extents: unsigned = 0;
    smp_rmb();
    if (extents <= UID_GID_MAP_MAX_BASE_EXTENTS) {
    extent = map_id_range_up_base(extents, map, id, count);
    }
    else {
    extent = map_id_range_up_max(extents, map, id, count);
    }
// Map the id or note failure
    if (extent) {
    id = (id - extent.lower_first) + extent.first;
    }
    else {
    id = (u32) -1;
    }
    return id;
    }
#[no_mangle]
pub unsafe extern "C" fn map_id_up(map: *mut uid_gid_map, id: u32) -> u32 {
    return map_id_range_up(map, id, 1);
    }
//
// make_kuid - Map a user-namespace uid pair into a kuid.
// @ns:  User namespace that the uid is in
// @uid: User identifier
//
// Maps a user-namespace uid pair into a kernel internal kuid,
// and returns that kuid.
//
// When there is no mapping defined for the user-namespace uid
// pair INVALID_UID is returned.  Callers are expected to test
// for and handle INVALID_UID being returned.  INVALID_UID
// may be tested for using uid_valid().
//
#[no_mangle]
pub unsafe extern "C" fn make_kuid(ns: *mut user_namespace, uid: uid_t) -> kuid_t {
// Map the uid to a global kernel uid
    return KUIDT_INIT(map_id_down(&ns.uid_map, uid));
    }
    EXPORT_SYMBOL(make_kuid);
//
// from_kuid - Create a uid from a kuid user-namespace pair.
// @targ: The user namespace we want a uid in.
// @kuid: The kernel internal uid to start with.
//
// Map @kuid into the user-namespace specified by @targ and
// return the resulting uid.
//
// There is always a mapping into the initial user_namespace.
//
// If @kuid has no mapping in @targ (uid_t)-1 is returned.
//
#[no_mangle]
pub unsafe extern "C" fn from_kuid(targ: *mut user_namespace, kuid: kuid_t) -> uid_t {
// Map the uid from a global kernel uid
    return map_id_up(&targ.uid_map, __kuid_val(kuid));
    }
    EXPORT_SYMBOL(from_kuid);
//
// from_kuid_munged - Create a uid from a kuid user-namespace pair.
// @targ: The user namespace we want a uid in.
// @kuid: The kernel internal uid to start with.
//
// Map @kuid into the user-namespace specified by @targ and
// return the resulting uid.
//
// There is always a mapping into the initial user_namespace.
//
// Unlike from_kuid from_kuid_munged never fails and always
// returns a valid uid.  This makes from_kuid_munged appropriate
// for use in syscalls like stat and getuid where failing the
// system call and failing to provide a valid uid are not an
// options.
//
// If @kuid has no mapping in @targ overflowuid is returned.
//
#[no_mangle]
pub unsafe extern "C" fn from_kuid_munged(targ: *mut user_namespace, kuid: kuid_t) -> uid_t {
    let mut uid = 0;
    uid = from_kuid(targ, kuid);
    if (uid == (uid_t) -1) {
    uid = overflowuid;
    }
    return uid;
    }
    EXPORT_SYMBOL(from_kuid_munged);
//
// make_kgid - Map a user-namespace gid pair into a kgid.
// @ns:  User namespace that the gid is in
// @gid: group identifier
//
// Maps a user-namespace gid pair into a kernel internal kgid,
// and returns that kgid.
//
// When there is no mapping defined for the user-namespace gid
// pair INVALID_GID is returned.  Callers are expected to test
// for and handle INVALID_GID being returned.  INVALID_GID may be
// tested for using gid_valid().
//
#[no_mangle]
pub unsafe extern "C" fn make_kgid(ns: *mut user_namespace, gid: gid_t) -> kgid_t {
// Map the gid to a global kernel gid
    return KGIDT_INIT(map_id_down(&ns.gid_map, gid));
    }
    EXPORT_SYMBOL(make_kgid);
//
// from_kgid - Create a gid from a kgid user-namespace pair.
// @targ: The user namespace we want a gid in.
// @kgid: The kernel internal gid to start with.
//
// Map @kgid into the user-namespace specified by @targ and
// return the resulting gid.
//
// There is always a mapping into the initial user_namespace.
//
// If @kgid has no mapping in @targ (gid_t)-1 is returned.
//
#[no_mangle]
pub unsafe extern "C" fn from_kgid(targ: *mut user_namespace, kgid: kgid_t) -> gid_t {
// Map the gid from a global kernel gid
    return map_id_up(&targ.gid_map, __kgid_val(kgid));
    }
    EXPORT_SYMBOL(from_kgid);
//
// from_kgid_munged - Create a gid from a kgid user-namespace pair.
// @targ: The user namespace we want a gid in.
// @kgid: The kernel internal gid to start with.
//
// Map @kgid into the user-namespace specified by @targ and
// return the resulting gid.
//
// There is always a mapping into the initial user_namespace.
//
// Unlike from_kgid from_kgid_munged never fails and always
// returns a valid gid.  This makes from_kgid_munged appropriate
// for use in syscalls like stat and getgid where failing the
// system call and failing to provide a valid gid are not options.
//
// If @kgid has no mapping in @targ overflowgid is returned.
//
#[no_mangle]
pub unsafe extern "C" fn from_kgid_munged(targ: *mut user_namespace, kgid: kgid_t) -> gid_t {
    let mut gid = 0;
    gid = from_kgid(targ, kgid);
    if (gid == (gid_t) -1) {
    gid = overflowgid;
    }
    return gid;
    }
    EXPORT_SYMBOL(from_kgid_munged);
//
// make_kprojid - Map a user-namespace projid pair into a kprojid.
// @ns:  User namespace that the projid is in
// @projid: Project identifier
//
// Maps a user-namespace uid pair into a kernel internal kuid,
// and returns that kuid.
//
// When there is no mapping defined for the user-namespace projid
// pair INVALID_PROJID is returned.  Callers are expected to test
// for and handle INVALID_PROJID being returned.  INVALID_PROJID
// may be tested for using projid_valid().
//
#[no_mangle]
pub unsafe extern "C" fn make_kprojid(ns: *mut user_namespace, projid: projid_t) -> kprojid_t {
// Map the uid to a global kernel uid
    return KPROJIDT_INIT(map_id_down(&ns.projid_map, projid));
    }
    EXPORT_SYMBOL(make_kprojid);
//
// from_kprojid - Create a projid from a kprojid user-namespace pair.
// @targ: The user namespace we want a projid in.
// @kprojid: The kernel internal project identifier to start with.
//
// Map @kprojid into the user-namespace specified by @targ and
// return the resulting projid.
//
// There is always a mapping into the initial user_namespace.
//
// If @kprojid has no mapping in @targ (projid_t)-1 is returned.
//
#[no_mangle]
pub unsafe extern "C" fn from_kprojid(targ: *mut user_namespace, kprojid: kprojid_t) -> projid_t {
// Map the uid from a global kernel uid
    return map_id_up(&targ.projid_map, __kprojid_val(kprojid));
    }
    EXPORT_SYMBOL(from_kprojid);
//
// from_kprojid_munged - Create a projiid from a kprojid user-namespace pair.
// @targ: The user namespace we want a projid in.
// @kprojid: The kernel internal projid to start with.
//
// Map @kprojid into the user-namespace specified by @targ and
// return the resulting projid.
//
// There is always a mapping into the initial user_namespace.
//
// Unlike from_kprojid from_kprojid_munged never fails and always
// returns a valid projid.  This makes from_kprojid_munged
// appropriate for use in syscalls like stat and where
// failing the system call and failing to provide a valid projid are
// not an options.
//
// If @kprojid has no mapping in @targ OVERFLOW_PROJID is returned.
//
#[no_mangle]
pub unsafe extern "C" fn from_kprojid_munged(targ: *mut user_namespace, kprojid: kprojid_t) -> projid_t {
    let mut projid;
    projid = from_kprojid(targ, kprojid);
    if (projid == (projid_t) -1) {
    projid = OVERFLOW_PROJID;
    }
    return projid;
    }
    EXPORT_SYMBOL(from_kprojid_munged);
#[no_mangle]
unsafe extern "C" fn uid_m_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let mut ns = seq.private;
    let mut extent = v;
pub static mut lower_ns: *mut c_void = core::ptr::null_mut();
    let mut lower = 0;
    lower_ns = seq_user_ns(seq);
    if ((lower_ns == ns) && lower_ns.parent) {
    lower_ns = lower_ns.parent;
    }
    lower = from_kuid(lower_ns, KUIDT_INIT(extent.lower_first));
    seq_printf(seq, "%10u %10u %10u\n",
    extent.first,
    lower,
    extent.count);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gid_m_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let mut ns = seq.private;
    let mut extent = v;
pub static mut lower_ns: *mut c_void = core::ptr::null_mut();
    let mut lower = 0;
    lower_ns = seq_user_ns(seq);
    if ((lower_ns == ns) && lower_ns.parent) {
    lower_ns = lower_ns.parent;
    }
    lower = from_kgid(lower_ns, KGIDT_INIT(extent.lower_first));
    seq_printf(seq, "%10u %10u %10u\n",
    extent.first,
    lower,
    extent.count);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn projid_m_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let mut ns = seq.private;
    let mut extent = v;
pub static mut lower_ns: *mut c_void = core::ptr::null_mut();
    let mut lower;
    lower_ns = seq_user_ns(seq);
    if ((lower_ns == ns) && lower_ns.parent) {
    lower_ns = lower_ns.parent;
    }
    lower = from_kprojid(lower_ns, KPROJIDT_INIT(extent.lower_first));
    seq_printf(seq, "%10u %10u %10u\n",
    extent.first,
    lower,
    extent.count);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn m_start(seq: *mut seq_file, ppos: *mut loff_t, map: *mut uid_gid_map) -> *mut c_void {
pub static mut pos: loff_t = 0;
pub static mut extents: unsigned = 0;
    smp_rmb();
    if (pos >= extents) {
    return core::ptr::null_mut();
    }
    if (extents <= UID_GID_MAP_MAX_BASE_EXTENTS) {
    return &map.extent[pos];
    }
    return &map.forward[pos];
    }
#[no_mangle]
pub unsafe extern "C" fn uid_m_start(seq: *mut seq_file, ppos: *mut loff_t) -> *mut c_void {
    let mut ns = seq.private;
    return m_start(seq, ppos, &ns.uid_map);
    }
#[no_mangle]
pub unsafe extern "C" fn gid_m_start(seq: *mut seq_file, ppos: *mut loff_t) -> *mut c_void {
    let mut ns = seq.private;
    return m_start(seq, ppos, &ns.gid_map);
    }
#[no_mangle]
pub unsafe extern "C" fn projid_m_start(seq: *mut seq_file, ppos: *mut loff_t) -> *mut c_void {
    let mut ns = seq.private;
    return m_start(seq, ppos, &ns.projid_map);
    }
#[no_mangle]
pub unsafe extern "C" fn m_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    (*pos)++;
    return seq.op.start(seq, pos);
    }
#[no_mangle]
unsafe extern "C" fn m_stop(seq: *mut seq_file, v: *mut c_void) {
    return;
    }
pub static mut seq_operations: usize = 0;
pub static mut seq_operations: usize = 0;
pub static mut seq_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn mappings_overlap(new_map: *mut uid_gid_map, extent: *mut uid_gid_extent) -> bool {
    u32 upper_first, lower_first, upper_last, lower_last;
    let mut idx: c_uint = 0;
    upper_first = extent.first;
    lower_first = extent.lower_first;
    upper_last = upper_first + extent.count - 1;
    lower_last = lower_first + extent.count - 1;
    while (idx < new_map.nr_extents) {
    u32 prev_upper_first, prev_lower_first;
    u32 prev_upper_last, prev_lower_last;
pub static mut prev: *mut c_void = core::ptr::null_mut();
    if (new_map.nr_extents <= UID_GID_MAP_MAX_BASE_EXTENTS) {
    prev = &new_map.extent[idx];
    }
    else {
    prev = &new_map.forward[idx];
    }
    prev_upper_first = prev.first;
    prev_lower_first = prev.lower_first;
    prev_upper_last = prev_upper_first + prev.count - 1;
    prev_lower_last = prev_lower_first + prev.count - 1;
// Does the upper range intersect a previous extent?
    if ((prev_upper_first <= upper_last) &&
    (prev_upper_last >= upper_first)) {
    return true;
    }
// Does the lower range intersect a previous extent?
    if ((prev_lower_first <= lower_last) &&
    (prev_lower_last >= lower_first)) {
    return true;
    }
    }
    return false;
    }
//
// insert_extent - Safely insert a new idmap extent into struct uid_gid_map.
// Takes care to allocate a 4K block of memory if the number of mappings exceeds
// UID_GID_MAP_MAX_BASE_EXTENTS.
//
#[no_mangle]
unsafe extern "C" fn insert_extent(map: *mut uid_gid_map, extent: *mut uid_gid_extent) -> c_int {
pub static mut dest: *mut c_void = core::ptr::null_mut();
    if (map.nr_extents == UID_GID_MAP_MAX_BASE_EXTENTS) {
pub static mut forward: *mut c_void = core::ptr::null_mut();
// Allocate memory for 340 mappings.
    forward = kmalloc_objs(uid_gid_extent,
    UID_GID_MAP_MAX_EXTENTS);
    if (!forward) {
    return -ENOMEM;
    }
// Copy over memory. Only set up memory for the forward pointer.
// Defer the memory setup for the reverse pointer.
//
    memcpy(forward, map.extent,
    map.nr_extents * sizeof!(map.extent[0]));
    map.forward = forward;
    map.reverse = core::ptr::null_mut();
    }
    if (map.nr_extents < UID_GID_MAP_MAX_BASE_EXTENTS) {
    dest = &map.extent[map.nr_extents];
    }
    else {
    dest = &map.forward[map.nr_extents];
    }
// dest = *extent;
    map.nr_extents += 1;
    return 0;
    }
// cmp function to sort() forward mappings
#[no_mangle]
unsafe extern "C" fn cmp_extents_forward(a: *const c_void, b: *const c_void) -> c_int {
    let mut e1 = a;
    let mut e2 = b;
    if (e1.first < e2.first) {
    return -1;
    }
    if (e1.first > e2.first) {
    return 1;
    }
    return 0;
    }
// cmp function to sort() reverse mappings
#[no_mangle]
unsafe extern "C" fn cmp_extents_reverse(a: *const c_void, b: *const c_void) -> c_int {
    let mut e1 = a;
    let mut e2 = b;
    if (e1.lower_first < e2.lower_first) {
    return -1;
    }
    if (e1.lower_first > e2.lower_first) {
    return 1;
    }
    return 0;
    }
//
// sort_idmaps - Sorts an array of idmap entries.
// Can only be called if number of mappings exceeds UID_GID_MAP_MAX_BASE_EXTENTS.
//
#[no_mangle]
unsafe extern "C" fn sort_idmaps(map: *mut uid_gid_map) -> c_int {
    if (map.nr_extents <= UID_GID_MAP_MAX_BASE_EXTENTS) {
    return 0;
    }
// Sort forward array.
    sort(map.forward, map.nr_extents, sizeof!(uid_gid_extent),
    cmp_extents_forward, core::ptr::null_mut());
// Only copy the memory from forward we actually need.
    map.reverse = kmemdup_array(map.forward, map.nr_extents,
    sizeof!(uid_gid_extent), GFP_KERNEL);
    if (!map.reverse) {
    return -ENOMEM;
    }
// Sort reverse array.
    sort(map.reverse, map.nr_extents, sizeof!(uid_gid_extent),
    cmp_extents_reverse, core::ptr::null_mut());
    return 0;
    }
//
// verify_root_map() - check the uid 0 mapping
// @file: idmapping file
// @map_ns: user namespace of the target process
// @new_map: requested idmap
//
// If a process requests mapping parent uid 0 into the new ns, verify that the
// process writing the map had the CAP_SETFCAP capability as the target process
// will be able to write fscaps that are valid in ancestor user namespaces.
//
// Return: true if the mapping is allowed, false if not.
//
#[no_mangle]
pub unsafe extern "C" fn verify_root_map(file: *mut file, map_ns: *mut user_namespace, new_map: *mut uid_gid_map) -> bool {
    let mut idx = 0;
    let mut file_ns = file.f_cred.user_ns;
    let mut extent0 = core::ptr::null_mut();
    while (idx < new_map.nr_extents) {
    if (new_map.nr_extents <= UID_GID_MAP_MAX_BASE_EXTENTS) {
    extent0 = &new_map.extent[idx];
    }
    else {
    extent0 = &new_map.forward[idx];
    }
    if (extent0.lower_first == 0) {
    break;
    }
    extent0 = core::ptr::null_mut();
    }
    if (!extent0) {
    return true;
    }
    if (map_ns == file_ns) {
// The process unshared its ns and is writing to its own
// /proc/self/uid_map.  User already has full capabilites in
// the new namespace.  Verify that the parent had CAP_SETFCAP
// when it unshared.
//
    if (!file_ns.parent_could_setfcap) {
    return false;
    }
    } else {
// Process p1 is writing to uid_map of p2, who is in a child
// user namespace to p1's.  Verify that the opener of the map
// file has CAP_SETFCAP against the parent of the new map
// namespace
    if (!file_ns_capable(file, map_ns.parent, CAP_SETFCAP)) {
    return false;
    }
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn map_write(file: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t, cap_setid: c_int, map: *mut uid_gid_map, parent_map: *mut uid_gid_map) -> ssize_t {
    let mut seq = file.private_data;
    let mut map_ns = seq.private;
pub static mut new_map: usize = 0;
    let mut idx: c_uint = 0;
pub static mut extent: usize = 0;
    let mut kbuf = core::ptr::null_mut();
    let mut pos = core::ptr::null_mut();
    let mut next_line = core::ptr::null_mut();
    let mut ret = 0;
// Only allow < page size writes at the beginning of the file
    if ((*ppos != 0) || (count >= PAGE_SIZE)) {
    return -EINVAL;
    }
// Slurp in the user data
    kbuf = memdup_user_nul(buf, count);
    if (IS_ERR(kbuf)) {
    return PTR_ERR(kbuf);
    }
//
// The userns_state_mutex serializes all writes to any given map.
//
// Any map is only ever written once.
//
// An id map fits within 1 cache line on most architectures.
//
// On read nothing needs to be done unless you are on an
// architecture with a crazy cache coherency model like alpha.
//
// There is a one time data dependency between reading the
// count of the extents and the values of the extents.  The
// desired behavior is to see the values of the extents that
// were written before the count of the extents.
//
// To achieve this smp_wmb() is used on guarantee the write
// order and smp_rmb() is guaranteed that we don't have crazy
// architectures returning stale data.
//
    mutex_lock(&userns_state_mutex);
    memset(&new_map, 0, sizeof!(uid_gid_map));
    ret = -EPERM;
// Only allow one successful write to the map
    if (map.nr_extents != 0) {
// goto;
    }
//
// Adjusting namespace settings requires capabilities on the target.
//
    if (cap_valid(cap_setid) && !file_ns_capable(file, map_ns, CAP_SYS_ADMIN)) {
// goto;
    }
// Parse the user data
    ret = -EINVAL;
    pos = kbuf;
    while (pos) {
// Find the end of line and ensure I don't look past it
    next_line = strchr(pos, '\n');
    if (next_line) {
// next_line = '\0';
    next_line += 1;
    if (*next_line == '\0') {
    next_line = core::ptr::null_mut();
    }
    }
    pos = skip_spaces(pos);
    extent.first = simple_strtoul(pos, &pos, 10);
    if (!isspace(*pos)) {
// goto;
    }
    pos = skip_spaces(pos);
    extent.lower_first = simple_strtoul(pos, &pos, 10);
    if (!isspace(*pos)) {
// goto;
    }
    pos = skip_spaces(pos);
    extent.count = simple_strtoul(pos, &pos, 10);
    if (*pos && !isspace(*pos)) {
// goto;
    }
// Verify there is not trailing junk on the line
    pos = skip_spaces(pos);
    if (*pos != '\0') {
// goto;
    }
// Verify we have been given valid starting values
    if ((extent.first == (u32) -1) ||
    (extent.lower_first == (u32) -1)) {
// goto;
    }
// Verify count is not zero and does not cause the
// extent to wrap
//
    if ((extent.first + extent.count) <= extent.first) {
// goto;
    }
    if ((extent.lower_first + extent.count) <=
    extent.lower_first) {
// goto;
    }
// Do the ranges in extent overlap any previous extents?
    if (mappings_overlap(&new_map, &extent)) {
// goto;
    }
    if ((new_map.nr_extents + 1) == UID_GID_MAP_MAX_EXTENTS &&
    (next_line != core::ptr::null_mut())) {
// goto;
    }
    ret = insert_extent(&new_map, &extent);
    if (ret < 0) {
// goto;
    }
    ret = -EINVAL;
    }
// Be very certain the new map actually exists
    if (new_map.nr_extents == 0) {
// goto;
    }
    ret = -EPERM;
// Validate the user is allowed to use user id's mapped to.
    if (!new_idmap_permitted(file, map_ns, cap_setid, &new_map)) {
// goto;
    }
    ret = -EPERM;
// Map the lower ids from the parent user namespace to the
// kernel global id space.
//
    while (idx < new_map.nr_extents) {
pub static mut e: *mut c_void = core::ptr::null_mut();
    let mut lower_first = 0;
    if (new_map.nr_extents <= UID_GID_MAP_MAX_BASE_EXTENTS) {
    e = &new_map.extent[idx];
    }
    else {
    e = &new_map.forward[idx];
    }
    lower_first = map_id_range_down(parent_map,
    e.lower_first,
    e.count);
// Fail if we can not map the specified extent to
// the kernel global id space.
//
    if (lower_first == (u32) -1) {
// goto;
    }
    e.lower_first = lower_first;
    }
//
// If we want to use binary search for lookup, this clones the extent
// array and sorts both copies.
//
    ret = sort_idmaps(&new_map);
    if (ret < 0) {
// goto;
    }
// Install the map
    if (new_map.nr_extents <= UID_GID_MAP_MAX_BASE_EXTENTS) {
    memcpy(map.extent, new_map.extent,
    new_map.nr_extents * sizeof!(new_map.extent[0]));
    } else {
    map.forward = new_map.forward;
    map.reverse = new_map.reverse;
    }
    smp_wmb();
    map.nr_extents = new_map.nr_extents;
// ppos = count;
    ret = count;
// label;
    if (ret < 0 && new_map.nr_extents > UID_GID_MAP_MAX_BASE_EXTENTS) {
    kfree(new_map.forward);
    kfree(new_map.reverse);
    map.forward = core::ptr::null_mut();
    map.reverse = core::ptr::null_mut();
    map.nr_extents = 0;
    }
    mutex_unlock(&userns_state_mutex);
    kfree(kbuf);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn proc_uid_map_write(file: *mut file, buf: *mut c_char, size: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut seq = file.private_data;
    let mut ns = seq.private;
    let mut seq_ns = seq_user_ns(seq);
    if (!ns.parent) {
    return -EPERM;
    }
    if ((seq_ns != ns) && (seq_ns != ns.parent)) {
    return -EPERM;
    }
    return map_write(file, buf, size, ppos, CAP_SETUID,
    &ns.uid_map, &ns.parent.uid_map);
    }
#[no_mangle]
pub unsafe extern "C" fn proc_gid_map_write(file: *mut file, buf: *mut c_char, size: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut seq = file.private_data;
    let mut ns = seq.private;
    let mut seq_ns = seq_user_ns(seq);
    if (!ns.parent) {
    return -EPERM;
    }
    if ((seq_ns != ns) && (seq_ns != ns.parent)) {
    return -EPERM;
    }
    return map_write(file, buf, size, ppos, CAP_SETGID,
    &ns.gid_map, &ns.parent.gid_map);
    }
#[no_mangle]
pub unsafe extern "C" fn proc_projid_map_write(file: *mut file, buf: *mut c_char, size: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut seq = file.private_data;
    let mut ns = seq.private;
    let mut seq_ns = seq_user_ns(seq);
    if (!ns.parent) {
    return -EPERM;
    }
    if ((seq_ns != ns) && (seq_ns != ns.parent)) {
    return -EPERM;
    }
// Anyone can set any valid project id no capability needed
    return map_write(file, buf, size, ppos, -1,
    &ns.projid_map, &ns.parent.projid_map);
    }
#[no_mangle]
pub unsafe extern "C" fn new_idmap_permitted(file: *mut file, ns: *mut user_namespace, cap_setid: c_int, new_map: *mut uid_gid_map) -> bool {
    let mut cred = file.f_cred;
    if (cap_setid == CAP_SETUID && !verify_root_map(file, ns, new_map)) {
    return false;
    }
// Don't allow mappings that would allow anything that wouldn't
// be allowed without the establishment of unprivileged mappings.
//
    if ((new_map.nr_extents == 1) && (new_map.extent[0].count == 1) &&
    uid_eq(ns.owner, cred.euid)) {
pub static mut id: u32 = 0;
    if (cap_setid == CAP_SETUID) {
pub static mut uid: kuid_t = 0;
    if (uid_eq(uid, cred.euid)) {
    return true;
    }
    } else if (cap_setid == CAP_SETGID) {
pub static mut gid: kgid_t = 0;
    if (!(ns.flags & USERNS_SETGROUPS_ALLOWED) &&
    gid_eq(gid, cred.egid)) {
    return true;
    }
    }
    }
// Allow anyone to set a mapping that doesn't require privilege
    if (!cap_valid(cap_setid)) {
    return true;
    }
// Allow the specified ids if we have the appropriate capability
// (CAP_SETUID or CAP_SETGID) over the parent user namespace.
// And the opener of the id file also has the appropriate capability.
//
    if (ns_capable(ns.parent, cap_setid) &&
    file_ns_capable(file, ns.parent, cap_setid)) {
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn proc_setgroups_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let mut ns = seq.private;
pub static mut userns_flags: c_ulong = 0;
    seq_printf(seq, "%s\n",
    (userns_flags & USERNS_SETGROUPS_ALLOWED) ?
    "allow" : "deny");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn proc_setgroups_write(file: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut seq = file.private_data;
    let mut ns = seq.private;
    char kbuf[8], *pos;
    let mut setgroups_allowed = 0;
    let mut ret = 0;
// Only allow a very narrow range of strings to be written
    ret = -EINVAL;
    if ((*ppos != 0) || (count >= sizeof!(kbuf))) {
// goto;
    }
// What was written?
    ret = -EFAULT;
    if (copy_from_user(kbuf, buf, count)) {
// goto;
    }
    kbuf[count] = '\0';
    pos = kbuf;
// What is being requested?
    ret = -EINVAL;
    if (strncmp(pos, "allow", 5) == 0) {
    pos += 5;
    setgroups_allowed = true;
    }
if true {
    pos += 4;
    setgroups_allowed = false;
    }
    else {
// goto;
    }
// Verify there is not trailing junk on the line
    pos = skip_spaces(pos);
    if (*pos != '\0') {
// goto;
    }
    ret = -EPERM;
    mutex_lock(&userns_state_mutex);
    if (setgroups_allowed) {
// Enabling setgroups after setgroups has been disabled
// is not allowed.
//
    if (!(ns.flags & USERNS_SETGROUPS_ALLOWED)) {
// goto;
    }
    } else {
// Permanently disabling setgroups after setgroups has
// been enabled by writing the gid_map is not allowed.
//
    if (ns.gid_map.nr_extents != 0) {
// goto;
    }
    ns.flags &= ~USERNS_SETGROUPS_ALLOWED;
    }
    mutex_unlock(&userns_state_mutex);
// Report a successful write
// ppos = count;
    ret = count;
// label;
    return ret;
// label;
    mutex_unlock(&userns_state_mutex);
// goto;
    }
#[no_mangle]
pub unsafe extern "C" fn userns_may_setgroups(ns: *const user_namespace) -> bool {
    let mut allowed = 0;
    mutex_lock(&userns_state_mutex);
// It is not safe to use setgroups until a gid mapping in
// the user namespace has been established.
//
    allowed = ns.gid_map.nr_extents != 0;
// Is setgroups allowed?
    allowed = allowed && (ns.flags & USERNS_SETGROUPS_ALLOWED);
    mutex_unlock(&userns_state_mutex);
    return allowed;
    }
//
// Returns true if @child is the same namespace or a descendant of
// @ancestor.
//
#[no_mangle]
pub unsafe extern "C" fn in_userns(ancestor: *mut user_namespace, child: *mut user_namespace) -> bool {
pub static mut ns: *mut c_void = core::ptr::null_mut();
    for (ns = child; ns.level > ancestor.level; ns = ns.parent) {
    ;
    }
    return (ns == ancestor);
    }
#[no_mangle]
pub unsafe extern "C" fn current_in_userns(target_ns: *const user_namespace) -> bool {
    return in_userns(target_ns, current_user_ns());
    }
    EXPORT_SYMBOL(current_in_userns);
#[no_mangle]
pub unsafe extern "C" fn userns_get(task: *mut task_struct) -> *mut c_void {
pub static mut user_ns: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    user_ns = get_user_ns(__task_cred(task).user_ns);
    rcu_read_unlock();
    return user_ns ? &user_ns.ns : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn userns_put(ns: *mut ns_common) {
    put_user_ns(to_user_ns(ns));
    }
#[no_mangle]
unsafe extern "C" fn userns_install(nsset: *mut nsset, ns: *mut ns_common) -> c_int {
    let mut user_ns = to_user_ns(ns);
pub static mut cred: *mut c_void = core::ptr::null_mut();
// Don't allow gaining capabilities by reentering
// the same user namespace.
//
    if (user_ns == current_user_ns()) {
    return -EINVAL;
    }
// Tasks that share a thread group must share a user namespace
    if (!thread_group_empty(current)) {
    return -EINVAL;
    }
    if (current.fs.users != 1) {
    return -EINVAL;
    }
    if (!ns_capable(user_ns, CAP_SYS_ADMIN)) {
    return -EPERM;
    }
    cred = nsset_cred(nsset);
    if (!cred) {
    return -EINVAL;
    }
    put_user_ns(cred.user_ns);
    set_cred_user_ns(cred, get_user_ns(user_ns));
    if (set_cred_ucounts(cred) < 0) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ns_get_owner(ns: *mut ns_common) -> *mut c_void {
    let mut my_user_ns = current_user_ns();
    let mut owner = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
// See if the owner is in the current user namespace
    owner = p = ns.ops.owner(ns);
    for (;;) {
    if (!p) {
    return ERR_PTR(-EPERM);
    }
    if (p == my_user_ns) {
    break;
    }
    p = p.parent;
    }
    return &get_user_ns(owner).ns;
    }
#[no_mangle]
pub unsafe extern "C" fn userns_owner(ns: *mut ns_common) -> *mut c_void {
    return to_user_ns(ns).parent;
    }
pub static mut proc_ns_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn user_namespaces_init() -> __init int {
    user_ns_cachep = KMEM_CACHE(user_namespace, SLAB_PANIC | SLAB_ACCOUNT);
    ns_tree_add(&init_user_ns);
    return 0;
    }
    subsys_initcall!(user_namespaces_init);