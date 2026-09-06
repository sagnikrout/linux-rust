//! Automatically rewritten from C to Rust
//! Source: kernel/audit_tree.c
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
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

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
macro_rules! DEFINE { ($($tt:tt)*) => {}; }

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
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0

    let mut audit_tree;
    let mut audit_chunk;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_tree {
    pub count: refcount_t,
    pub goner: c_int,
    pub root: *mut audit_chunk,
    pub chunks: list_head,
    pub rules: list_head,
    pub list: list_head,
    pub same_root: list_head,
    pub head: rcu_head,
    pub pathname: [c_char; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_chunk {
    pub hash: list_head,
    pub key: c_ulong,
    pub mark: *mut fsnotify_mark,
//     pub /: *mut *mut list_head trees; / with root here,
    pub count: c_int,
    pub refs: atomic_long_t,
    pub head: rcu_head,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_node {
    pub list: list_head,
    pub owner: *mut audit_tree,
//     pub /: *mut *mut unsigned int index; / index; upper bit indicates 'will prune',
    pub __counted_by(count): } owners[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_tree_mark {
    pub mark: fsnotify_mark,
    pub chunk: *mut audit_chunk,
}
// static LIST_HEAD(tree_list);
// static LIST_HEAD(prune_list);
pub static mut prune_thread: *mut c_void = core::ptr::null_mut();
//
// One struct chunk is attached to each inode of interest through
// audit_tree_mark (fsnotify mark). We replace struct chunk on tagging
// untagging, the mark is stable as long as there is chunk attached. The
// association between mark and chunk is protected by hash_lock and
// audit_tree_group->mark_mutex. Thus as long as we hold
// audit_tree_group->mark_mutex and check that the mark is alive by
// FSNOTIFY_MARK_FLAG_ATTACHED flag check, we are sure the mark points to
// the current chunk.
//
// Rules have pointer to struct audit_tree.
// Rules have struct list_head rlist forming a list of rules over
// the same tree.
// References to struct chunk are collected at audit_inode{,_child}()
// time and used in AUDIT_TREE rule matching.
// These references are dropped at the same time we are calling
// audit_free_names(), etc.
//
// Cyclic lists galore:
// tree.chunks anchors chunk.owners[].list			hash_lock
// tree.rules anchors rule.rlist				audit_filter_mutex
// chunk.trees anchors tree.same_root				hash_lock
// chunk.hash is a hash with middle bits of watch.inode as
// a hash function.						RCU, hash_lock
//
// tree is refcounted; one reference for "some rules on rules_list refer to
// it", one for each chunk with pointer to it.
//
// chunk is refcounted by embedded .refs. Mark associated with the chunk holds
// one chunk reference. This reference is dropped either when a mark is going
// to be freed (corresponding inode goes away) or when chunk attached to the
// mark gets replaced. This reference must be dropped using
// audit_mark_put_chunk() to make sure the reference is dropped only after RCU
// grace period as it protects RCU readers of the hash table.
//
// node.index allows to get from node.list to containing chunk.
// MSB of that sucker is stolen to mark taggings that we might have to
// revert - several operations have very unpleasant cleanup logics and
// that makes a difference.  Some.
//
pub static mut audit_tree_group: *mut c_void = core::ptr::null_mut();
pub static mut audit_tree_mark_cachep: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn alloc_tree() {
    let mut tree = core::ptr::null_mut();
    let mut sz = 0;
    sz = strlen(s) + 1;
    tree = kmalloc_flex(*tree, pathname, sz);
    if (tree) {
    refcount_set(&tree.count, 1);
    tree.goner = 0;
// INIT_LIST_HEAD;
// INIT_LIST_HEAD;
// INIT_LIST_HEAD;
// INIT_LIST_HEAD;
    tree.root = core::ptr::null_mut();
    strscpy(tree.pathname, s, sz);
    }
    return tree;
    }
#[no_mangle]
pub unsafe extern "C" fn get_tree(tree: *mut audit_tree) {
    refcount_inc(&tree.count);
    }
#[no_mangle]
pub unsafe extern "C" fn put_tree(tree: *mut audit_tree) {
    if (refcount_dec_and_test(&tree.count)) {
    kfree_rcu(tree, head);
    }
    }
// to avoid bringing the entire thing in audit.h
    const char *audit_tree_path(audit_tree *tree)
    {
    return tree.pathname;
    }
#[no_mangle]
unsafe extern "C" fn free_chunk(chunk: *mut audit_chunk) {
    let mut i = 0;
    while (i < chunk.count) {
    if (chunk.owners[i].owner) {
    put_tree(chunk.owners[i].owner);
    }
    }
    kfree(chunk);
    }
#[no_mangle]
pub unsafe extern "C" fn audit_put_chunk(chunk: *mut audit_chunk) {
    if (atomic_long_dec_and_test(&chunk.refs)) {
    free_chunk(chunk);
    }
    }
#[no_mangle]
unsafe extern "C" fn __put_chunk(rcu: *mut rcu_head) {
    let mut chunk = container_of!(rcu, audit_chunk, head);
    audit_put_chunk(chunk);
    }
//
// Drop reference to the chunk that was held by the mark. This is the reference
// that gets dropped after we've removed the chunk from the hash table and we
// use it to make sure chunk cannot be freed before RCU grace period expires.
//
#[no_mangle]
unsafe extern "C" fn audit_mark_put_chunk(chunk: *mut audit_chunk) {
    call_rcu(&chunk.head, __put_chunk);
    }
#[no_mangle]
pub unsafe extern "C" fn audit_mark() {
    return container_of!(mark, audit_tree_mark, mark);
    }
#[no_mangle]
pub unsafe extern "C" fn mark_chunk() {
    return audit_mark(mark).chunk;
    }
#[no_mangle]
unsafe extern "C" fn audit_tree_destroy_watch(mark: *mut fsnotify_mark) {
    kmem_cache_free(audit_tree_mark_cachep, audit_mark(mark));
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_mark() {
    let mut amark = core::ptr::null_mut();
    amark = kmem_cache_zalloc(audit_tree_mark_cachep, GFP_KERNEL);
    if (!amark) {
    return core::ptr::null_mut();
    }
    fsnotify_init_mark(&amark.mark, audit_tree_group);
    amark.mark.mask = FS_IN_IGNORED;
    return &amark.mark;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_chunk() {
    let mut chunk = core::ptr::null_mut();
    let mut i = 0;
    chunk = kzalloc_flex(*chunk, owners, count);
    if (!chunk) {
    return core::ptr::null_mut();
    }
// INIT_LIST_HEAD;
// INIT_LIST_HEAD;
    chunk.count = count;
    atomic_long_set(&chunk.refs, 1);
    while (i < count) {
// INIT_LIST_HEAD;
    chunk.owners[i].index = i;
    }
    return chunk;
    }
    enum {HASH_SIZE = 128};
    static struct list_head chunk_hash_heads[HASH_SIZE];
    static __cacheline_aligned_in_smp DEFINE_SPINLOCK(hash_lock);
// Function to return search key in our hash from inode.
#[no_mangle]
unsafe extern "C" fn inode_to_key(inode: *const inode) -> c_ulong {
// Use address pointed to by connector->obj as the key
    return (unsigned long)&inode.i_fsnotify_marks;
    }
#[no_mangle]
pub unsafe extern "C" fn chunk_hash() {
pub static mut n: c_ulong = 0;
    return chunk_hash_heads + n % HASH_SIZE;
    }
// hash_lock & mark->group->mark_mutex is held by caller
#[no_mangle]
unsafe extern "C" fn insert_hash(chunk: *mut audit_chunk) {
    let mut list = core::ptr::null_mut();
//
// Make sure chunk is fully initialized before making it visible in the
// hash. Pairs with a data dependency barrier in READ_ONCE() in
// audit_tree_lookup().
//
    smp_wmb();
// WARN_ON_ONCE;
    list = chunk_hash(chunk.key);
    list_add_rcu(&chunk.hash, list);
    }
// called under rcu_read_lock
#[no_mangle]
pub unsafe extern "C" fn audit_tree_lookup() {
pub static mut key: c_ulong = 0;
    let mut list = chunk_hash(key);
    let mut p = core::ptr::null_mut();
    list_for_each_entry_rcu(p, list, hash) {
//
// We use a data dependency barrier in READ_ONCE() to make sure
// the chunk we see is fully initialized.
//
    if (READ_ONCE(p.key) == key) {
    atomic_long_inc(&p.refs);
    return p;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn audit_tree_match(chunk: *mut audit_chunk, tree: *mut audit_tree) -> bool {
    let mut n = 0;
    for (n = 0; n < chunk.count; n++) {
    if (chunk.owners[n].owner == tree) {
    }
    return true;
    }
    return false;
    }
// tagging and untagging inodes with trees
#[no_mangle]
pub unsafe extern "C" fn find_chunk() {
pub static mut index: c_int = 0;
    p -= index;
    return container_of!(p, audit_chunk, owners[0]);
    }
#[no_mangle]
pub unsafe extern "C" fn replace_mark_chunk() {
    let mut old = core::ptr::null_mut();
    assert_spin_locked(&hash_lock);
    old = mark_chunk(mark);
    audit_mark(mark).chunk = chunk;
    if (chunk) {
    chunk.mark = mark;
    }
    if (old) {
    old.mark = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn replace_chunk(new: *mut audit_chunk, old: *mut audit_chunk) {
    let mut owner = core::ptr::null_mut();
    let mut i = 0;
    let mut j = 0;
    new.key = old.key;
    list_splice_init(&old.trees, &new.trees);
    list_for_each_entry(owner, &new.trees, same_root) {
    owner.root = new;
    }
    while (j < old.count) {
    if (!old.owners[j].owner) {
    i -= 1;
    continue;
    }
    owner = old.owners[j].owner;
    new.owners[i].owner = owner;
    new.owners[i].index = old.owners[j].index - j + i;
    if (!owner) /* result of earlier fallback */ {
    continue;
    }
    get_tree(owner);
    list_replace_init(&old.owners[j].list, &new.owners[i].list);
    }
    replace_mark_chunk(old.mark, new);
//
// Make sure chunk is fully initialized before making it visible in the
// hash. Pairs with a data dependency barrier in READ_ONCE() in
// audit_tree_lookup().
//
    smp_wmb();
    list_replace_rcu(&old.hash, &new.hash);
    }
#[no_mangle]
unsafe extern "C" fn remove_chunk_node(chunk: *mut audit_chunk, p: *mut audit_node) {
    let mut owner = p.owner;
    if (owner.root == chunk) {
    list_del_init(&owner.same_root);
    owner.root = core::ptr::null_mut();
    }
    list_del_init(&p.list);
    p.owner = core::ptr::null_mut();
    put_tree(owner);
    }
#[no_mangle]
unsafe extern "C" fn chunk_count_trees(chunk: *mut audit_chunk) -> c_int {
    let mut i = 0;
pub static mut ret: c_int = 0;
    for (i = 0; i < chunk.count; i++) {
    if (chunk.owners[i].owner) {
    }
    ret += 1;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn untag_chunk(chunk: *mut audit_chunk, mark: *mut fsnotify_mark) {
    let mut new = core::ptr::null_mut();
    let mut size = 0;
    fsnotify_group_lock(audit_tree_group);
//
// mark_mutex stabilizes chunk attached to the mark so we can check
// whether it didn't change while we've dropped hash_lock.
//
    if (!(mark.flags & FSNOTIFY_MARK_FLAG_ATTACHED) ||
    mark_chunk(mark) != chunk) {
// goto;
    }
    size = chunk_count_trees(chunk);
    if (!size) {
    spin_lock(&hash_lock);
    list_del_init(&chunk.trees);
    list_del_rcu(&chunk.hash);
    replace_mark_chunk(mark, core::ptr::null_mut());
    spin_unlock(&hash_lock);
    fsnotify_detach_mark(mark);
    fsnotify_group_unlock(audit_tree_group);
    audit_mark_put_chunk(chunk);
    fsnotify_free_mark(mark);
    return;
    }
    new = alloc_chunk(size);
    if (!new) {
// goto;
    }
    spin_lock(&hash_lock);
//
// This has to go last when updating chunk as once replace_chunk() is
// called, new RCU readers can see the new chunk.
//
    replace_chunk(new, chunk);
    spin_unlock(&hash_lock);
    fsnotify_group_unlock(audit_tree_group);
    audit_mark_put_chunk(chunk);
    return;
// label;
    fsnotify_group_unlock(audit_tree_group);
    }
// Call with group->mark_mutex held, releases it
#[no_mangle]
unsafe extern "C" fn create_chunk(inode: *mut inode, tree: *mut audit_tree) -> c_int {
    let mut mark = core::ptr::null_mut();
    let mut chunk = alloc_chunk(1);
    if (!chunk) {
    fsnotify_group_unlock(audit_tree_group);
    return -ENOMEM;
    }
    mark = alloc_mark();
    if (!mark) {
    fsnotify_group_unlock(audit_tree_group);
    kfree(chunk);
    return -ENOMEM;
    }
    if (fsnotify_add_inode_mark_locked(mark, inode, 0)) {
    fsnotify_group_unlock(audit_tree_group);
    fsnotify_put_mark(mark);
    kfree(chunk);
    return -ENOSPC;
    }
    spin_lock(&hash_lock);
    if (tree.goner) {
    spin_unlock(&hash_lock);
    fsnotify_detach_mark(mark);
    fsnotify_group_unlock(audit_tree_group);
    fsnotify_free_mark(mark);
    fsnotify_put_mark(mark);
    kfree(chunk);
    return 0;
    }
    replace_mark_chunk(mark, chunk);
    chunk.owners[0].index = (1U << 31);
    chunk.owners[0].owner = tree;
    get_tree(tree);
    list_add(&chunk.owners[0].list, &tree.chunks);
    if (!tree.root) {
    tree.root = chunk;
    list_add(&tree.same_root, &chunk.trees);
    }
    chunk.key = inode_to_key(inode);
//
// Inserting into the hash table has to go last as once we do that RCU
// readers can see the chunk.
//
    insert_hash(chunk);
    spin_unlock(&hash_lock);
    fsnotify_group_unlock(audit_tree_group);
//
// Drop our initial reference. When mark we point to is getting freed,
// we get notification through ->freeing_mark callback and cleanup
// chunk pointing to this mark.
//
    fsnotify_put_mark(mark);
    return 0;
    }
// the first tagged inode becomes root of tree
#[no_mangle]
unsafe extern "C" fn tag_chunk(inode: *mut inode, tree: *mut audit_tree) -> c_int {
    let mut mark = core::ptr::null_mut();
    let mut chunk = core::ptr::null_mut();
    let mut old = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut n = 0;
    fsnotify_group_lock(audit_tree_group);
    mark = fsnotify_find_inode_mark(inode, audit_tree_group);
    if (!mark) {
    return create_chunk(inode, tree);
    }
//
// Found mark is guaranteed to be attached and mark_mutex protects mark
// from getting detached and thus it makes sure there is chunk attached
// to the mark.
//
// are we already there?
    spin_lock(&hash_lock);
    old = mark_chunk(mark);
    while (n < old.count) {
    if (old.owners[n].owner == tree) {
    spin_unlock(&hash_lock);
    fsnotify_group_unlock(audit_tree_group);
    fsnotify_put_mark(mark);
    return 0;
    }
    }
    spin_unlock(&hash_lock);
    chunk = alloc_chunk(old.count + 1);
    if (!chunk) {
    fsnotify_group_unlock(audit_tree_group);
    fsnotify_put_mark(mark);
    return -ENOMEM;
    }
    spin_lock(&hash_lock);
    if (tree.goner) {
    spin_unlock(&hash_lock);
    fsnotify_group_unlock(audit_tree_group);
    fsnotify_put_mark(mark);
    kfree(chunk);
    return 0;
    }
    p = &chunk.owners[chunk.count - 1];
    p.index = (chunk.count - 1) | (1U<<31);
    p.owner = tree;
    get_tree(tree);
    list_add(&p.list, &tree.chunks);
    if (!tree.root) {
    tree.root = chunk;
    list_add(&tree.same_root, &chunk.trees);
    }
//
// This has to go last when updating chunk as once replace_chunk() is
// called, new RCU readers can see the new chunk.
//
    replace_chunk(chunk, old);
    spin_unlock(&hash_lock);
    fsnotify_group_unlock(audit_tree_group);
    fsnotify_put_mark(mark); /* pair to fsnotify_find_mark */
    audit_mark_put_chunk(old);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_tree_log_remove_rule() {
    let mut ab = core::ptr::null_mut();
    if (!audit_enabled) {
    return;
    }
    ab = audit_log_start(context, GFP_KERNEL, AUDIT_CONFIG_CHANGE);
    if (unlikely(!ab)) {
    return;
    }
    audit_log_format(ab, "op=remove_rule dir=");
    audit_log_untrustedstring(ab, rule.tree.pathname);
    audit_log_key(ab, rule.filterkey);
    audit_log_format(ab, " list=%d res=1", rule.listnr);
    audit_log_end(ab);
    }
#[no_mangle]
unsafe extern "C" fn kill_rules(context: *mut audit_context, tree: *mut audit_tree) {
    let mut rule = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    let mut entry = core::ptr::null_mut();
    list_for_each_entry_safe(rule, next, &tree.rules, rlist) {
    entry = container_of!(rule, audit_entry, rule);
    list_del_init(&rule.rlist);
    if (rule.tree) {
// not a half-baked one
    audit_tree_log_remove_rule(context, rule);
    if (entry.rule.exe) {
    audit_remove_mark(entry.rule.exe);
    }
    rule.tree = core::ptr::null_mut();
    list_del_rcu(&entry.list);
    list_del(&entry.rule.list);
    call_rcu(&entry.rcu, audit_free_rule_rcu);
    }
    }
    }
//
// Remove tree from chunks. If 'tagged' is set, remove tree only from tagged
// chunks. The function expects tagged chunks are all at the beginning of the
// chunks list.
//
#[no_mangle]
unsafe extern "C" fn prune_tree_chunks(victim: *mut audit_tree, tagged: bool) {
    spin_lock(&hash_lock);
    while (!list_empty(&victim.chunks)) {
    let mut p = core::ptr::null_mut();
    let mut chunk = core::ptr::null_mut();
    let mut mark = core::ptr::null_mut();
    p = list_first_entry(&victim.chunks, audit_node, list);
// have we run out of marked?
    if (tagged && !(p.index & (1U<<31))) {
    break;
    }
    chunk = find_chunk(p);
    mark = chunk.mark;
    remove_chunk_node(chunk, p);
// Racing with audit_tree_freeing_mark()?
    if (!mark) {
    continue;
    }
    fsnotify_get_mark(mark);
    spin_unlock(&hash_lock);
    untag_chunk(chunk, mark);
    fsnotify_put_mark(mark);
    spin_lock(&hash_lock);
    }
    spin_unlock(&hash_lock);
    }
//
// finish killing struct audit_tree
//
#[no_mangle]
unsafe extern "C" fn prune_one(victim: *mut audit_tree) {
    prune_tree_chunks(victim, false);
    put_tree(victim);
    }
// trim the uncommitted chunks from tree
#[no_mangle]
unsafe extern "C" fn trim_marked(tree: *mut audit_tree) {
    let mut p = core::ptr::null_mut();
    let mut q = core::ptr::null_mut();
    spin_lock(&hash_lock);
    if (tree.goner) {
    spin_unlock(&hash_lock);
    return;
    }
// reorder
    while (p != &tree.chunks) {
    let mut node = list_entry(p, audit_node, list);
    q = p.next;
    if (node.index & (1U<<31)) {
    list_del_init(p);
    list_add(p, &tree.chunks);
    }
    }
    spin_unlock(&hash_lock);
    prune_tree_chunks(tree, true);
    spin_lock(&hash_lock);
    if (!tree.root && !tree.goner) {
    tree.goner = 1;
    spin_unlock(&hash_lock);
    mutex_lock(&audit_filter_mutex);
    kill_rules(audit_context(), tree);
    list_del_init(&tree.list);
    mutex_unlock(&audit_filter_mutex);
    prune_one(tree);
    } else {
    spin_unlock(&hash_lock);
    }
    }
// forward_decl: audit_schedule_prune;
// called with audit_filter_mutex
#[no_mangle]
pub unsafe extern "C" fn audit_remove_tree_rule(rule: *mut audit_krule) -> c_int {
    let mut tree = core::ptr::null_mut();
    tree = rule.tree;
    if (tree) {
    spin_lock(&hash_lock);
    list_del_init(&rule.rlist);
    if (list_empty(&tree.rules) && !tree.goner) {
    tree.root = core::ptr::null_mut();
    list_del_init(&tree.same_root);
    tree.goner = 1;
    list_move(&tree.list, &prune_list);
    rule.tree = core::ptr::null_mut();
    spin_unlock(&hash_lock);
    audit_schedule_prune();
    return 1;
    }
    rule.tree = core::ptr::null_mut();
    spin_unlock(&hash_lock);
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_trim_trees() {
    let mut cursor;
    mutex_lock(&audit_filter_mutex);
    list_add(&cursor, &tree_list);
    while (cursor.next != &tree_list) {
    let mut tree = core::ptr::null_mut();
    let mut path;
    let mut node = core::ptr::null_mut();
    let mut paths = core::ptr::null_mut();
    struct path array[16];
    let mut err = 0;
    tree = container_of!(cursor.next, audit_tree, list);
    get_tree(tree);
    list_move(&cursor, &tree.list);
    mutex_unlock(&audit_filter_mutex);
    err = kern_path(tree.pathname, 0, &path);
    if (err) {
// goto;
    }
    paths = collect_paths(&path, array, 16);
    path_put(&path);
    if (IS_ERR(paths)) {
// goto;
    }
    spin_lock(&hash_lock);
    list_for_each_entry(node, &tree.chunks, list) {
    let mut chunk = find_chunk(node);
// this could be NULL if the watch is dying else where...
    node.index |= 1U<<31;
    while (p.dentry) {
    let mut inode = p.dentry.d_inode;
    if (inode_to_key(inode) == chunk.key) {
    node.index &= ~(1U<<31);
    break;
    }
    }
    }
    spin_unlock(&hash_lock);
    trim_marked(tree);
    drop_collected_paths(paths, array);
// label;
    put_tree(tree);
    mutex_lock(&audit_filter_mutex);
    }
    list_del(&cursor);
    mutex_unlock(&audit_filter_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn audit_make_tree(rule: *mut audit_krule, pathname: *mut c_char, op: u32) -> c_int {
    if (pathname[0] != '/' ||
    (rule.listnr != AUDIT_FILTER_EXIT &&
    rule.listnr != AUDIT_FILTER_URING_EXIT) ||
    op != Audit_equal ||
    rule.inode_f || rule.watch || rule.tree) {
    return -EINVAL;
    }
    rule.tree = alloc_tree(pathname);
    if (!rule.tree) {
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_put_tree(tree: *mut audit_tree) {
    put_tree(tree);
    }
#[no_mangle]
unsafe extern "C" fn tag_mounts(paths: *const path, tree: *mut audit_tree) -> c_int {
    while (p.dentry) {
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    }
    return 0;
    }
//
// That gets run when evict_chunk() ends up needing to kill audit_tree.
// Runs from a separate thread.
//
#[no_mangle]
unsafe extern "C" fn prune_tree_thread(unused: *mut c_void) -> c_int {
    for (;;) {
    if (list_empty(&prune_list)) {
    set_current_state(TASK_INTERRUPTIBLE);
    schedule();
    }
    audit_ctl_lock();
    mutex_lock(&audit_filter_mutex);
    while (!list_empty(&prune_list)) {
    let mut victim = core::ptr::null_mut();
    victim = list_entry(prune_list.next, audit_tree, list);
    list_del_init(&victim.list);
    mutex_unlock(&audit_filter_mutex);
    prune_one(victim);
    mutex_lock(&audit_filter_mutex);
    }
    mutex_unlock(&audit_filter_mutex);
    audit_ctl_unlock();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn audit_launch_prune() -> c_int {
    if (prune_thread) {
    return 0;
    }
    prune_thread = kthread_run(prune_tree_thread, core::ptr::null_mut(),
    "audit_prune_tree");
    if (IS_ERR(prune_thread)) {
    pr_err!("cannot start thread audit_prune_tree");
    prune_thread = core::ptr::null_mut();
    return -ENOMEM;
    }
    return 0;
    }
// called with audit_filter_mutex
#[no_mangle]
pub unsafe extern "C" fn audit_add_tree_rule(rule: *mut audit_krule) -> c_int {
    let mut seed = rule.tree, *tree;
    let mut path;
    struct path array[16];
    let mut paths = core::ptr::null_mut();
    let mut err = 0;
    rule.tree = core::ptr::null_mut();
    list_for_each_entry(tree, &tree_list, list) {
    if (!strcmp(seed.pathname, tree.pathname)) {
    put_tree(seed);
    rule.tree = tree;
    list_add(&rule.rlist, &tree.rules);
    return 0;
    }
    }
    tree = seed;
    list_add(&tree.list, &tree_list);
    list_add(&rule.rlist, &tree.rules);
// do not set rule->tree yet
    mutex_unlock(&audit_filter_mutex);
    if (unlikely(!prune_thread)) {
    err = audit_launch_prune();
    if (err) {
// goto;
    }
    }
    err = kern_path(tree.pathname, 0, &path);
    if (err) {
// goto;
    }
    paths = collect_paths(&path, array, 16);
    path_put(&path);
    if (IS_ERR(paths)) {
    err = PTR_ERR(paths);
// goto;
    }
    get_tree(tree);
    err = tag_mounts(paths, tree);
    drop_collected_paths(paths, array);
    if (!err) {
    let mut node = core::ptr::null_mut();
    spin_lock(&hash_lock);
    list_for_each_entry(node, &tree.chunks, list) {
    node.index &= ~(1U<<31);
    }
    spin_unlock(&hash_lock);
    } else {
    trim_marked(tree);
// goto;
    }
    mutex_lock(&audit_filter_mutex);
    if (list_empty(&rule.rlist)) {
    put_tree(tree);
    return -ENOENT;
    }
    rule.tree = tree;
    put_tree(tree);
    return 0;
// label;
    mutex_lock(&audit_filter_mutex);
    list_del_init(&tree.list);
    list_del_init(&tree.rules);
    put_tree(tree);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_tag_tree(old: *mut c_char, new: *mut c_char) -> c_int {
    struct list_head cursor, barrier;
pub static mut failed: c_int = 0;
    struct path path1, path2;
    struct path array[16];
    let mut paths = core::ptr::null_mut();
    let mut err = 0;
    err = kern_path(new, 0, &path2);
    if (err) {
    return err;
    }
    paths = collect_paths(&path2, array, 16);
    path_put(&path2);
    if (IS_ERR(paths)) {
    return PTR_ERR(paths);
    }
    err = kern_path(old, 0, &path1);
    if (err) {
    drop_collected_paths(paths, array);
    return err;
    }
    mutex_lock(&audit_filter_mutex);
    list_add(&barrier, &tree_list);
    list_add(&cursor, &barrier);
    while (cursor.next != &tree_list) {
    let mut tree = core::ptr::null_mut();
pub static mut good_one: c_int = 0;
    tree = container_of!(cursor.next, audit_tree, list);
    get_tree(tree);
    list_move(&cursor, &tree.list);
    mutex_unlock(&audit_filter_mutex);
    err = kern_path(tree.pathname, 0, &path2);
    if (!err) {
    good_one = path_is_under(&path1, &path2);
    path_put(&path2);
    }
    if (!good_one) {
    put_tree(tree);
    mutex_lock(&audit_filter_mutex);
    continue;
    }
    failed = tag_mounts(paths, tree);
    if (failed) {
    put_tree(tree);
    mutex_lock(&audit_filter_mutex);
    break;
    }
    mutex_lock(&audit_filter_mutex);
    spin_lock(&hash_lock);
    if (!tree.goner) {
    list_move(&tree.list, &tree_list);
    }
    spin_unlock(&hash_lock);
    put_tree(tree);
    }
    while (barrier.prev != &tree_list) {
    let mut tree = core::ptr::null_mut();
    tree = container_of!(barrier.prev, audit_tree, list);
    get_tree(tree);
    list_move(&tree.list, &barrier);
    mutex_unlock(&audit_filter_mutex);
    if (!failed) {
    let mut node = core::ptr::null_mut();
    spin_lock(&hash_lock);
    list_for_each_entry(node, &tree.chunks, list) {
    node.index &= ~(1U<<31);
    }
    spin_unlock(&hash_lock);
    } else {
    trim_marked(tree);
    }
    put_tree(tree);
    mutex_lock(&audit_filter_mutex);
    }
    list_del(&barrier);
    list_del(&cursor);
    mutex_unlock(&audit_filter_mutex);
    path_put(&path1);
    drop_collected_paths(paths, array);
    return failed;
    }
#[no_mangle]
unsafe extern "C" fn audit_schedule_prune() {
    wake_up_process(prune_thread);
    }
//
// ... and that one is done if evict_chunk() decides to delay until the end
// of syscall.  Runs synchronously.
//
#[no_mangle]
pub unsafe extern "C" fn audit_kill_trees(context: *mut audit_context) {
    let mut list = &context.killed_trees;
    audit_ctl_lock();
    mutex_lock(&audit_filter_mutex);
    while (!list_empty(list)) {
    let mut victim = core::ptr::null_mut();
    victim = list_entry(list.next, audit_tree, list);
    kill_rules(context, victim);
    list_del_init(&victim.list);
    mutex_unlock(&audit_filter_mutex);
    prune_one(victim);
    mutex_lock(&audit_filter_mutex);
    }
    mutex_unlock(&audit_filter_mutex);
    audit_ctl_unlock();
    }
//
// Here comes the stuff asynchronous to auditctl operations
//
#[no_mangle]
unsafe extern "C" fn evict_chunk(chunk: *mut audit_chunk) {
    let mut owner = core::ptr::null_mut();
    let mut postponed = audit_killed_trees();
pub static mut need_prune: c_int = 0;
    let mut n = 0;
    mutex_lock(&audit_filter_mutex);
    spin_lock(&hash_lock);
    while (!list_empty(&chunk.trees)) {
    owner = list_entry(chunk.trees.next, audit_tree, same_root);
    owner.goner = 1;
    owner.root = core::ptr::null_mut();
    list_del_init(&owner.same_root);
    spin_unlock(&hash_lock);
    if (!postponed) {
    kill_rules(audit_context(), owner);
    list_move(&owner.list, &prune_list);
    need_prune = 1;
    } else {
    list_move(&owner.list, postponed);
    }
    spin_lock(&hash_lock);
    }
    list_del_rcu(&chunk.hash);
    for (n = 0; n < chunk.count; n++) {
    list_del_init(&chunk.owners[n].list);
    }
    spin_unlock(&hash_lock);
    mutex_unlock(&audit_filter_mutex);
    if (need_prune) {
    audit_schedule_prune();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn audit_tree_handle_event() {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_tree_freeing_mark() {
    let mut chunk = core::ptr::null_mut();
    fsnotify_group_lock(mark.group);
    spin_lock(&hash_lock);
    chunk = mark_chunk(mark);
    replace_mark_chunk(mark, core::ptr::null_mut());
    spin_unlock(&hash_lock);
    fsnotify_group_unlock(mark.group);
    if (chunk) {
    evict_chunk(chunk);
    audit_mark_put_chunk(chunk);
    }
//
// We are guaranteed to have at least one reference to the mark from
// either the inode or the caller of fsnotify_destroy_mark().
//
// BUG_ON;
    }
pub static mut fsnotify_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn audit_tree_init() -> c_int {
    let mut i = 0;
    audit_tree_mark_cachep = KMEM_CACHE(audit_tree_mark, SLAB_PANIC);
    audit_tree_group = fsnotify_alloc_group(&audit_tree_ops, 0);
    if (IS_ERR(audit_tree_group)) {
    audit_panic("cannot initialize fsnotify group for rectree watches");
    }
    for (i = 0; i < HASH_SIZE; i++) {
// INIT_LIST_HEAD;
    }
    return 0;
    }
// __initcall;