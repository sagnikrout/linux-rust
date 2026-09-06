//! Automatically rewritten from C to Rust
//! Source: kernel/user.c
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



// SPDX-License-Identifier: GPL-2.0-only
//
// The "user cache".
//
// (C) Copyright 1991-2000 Linus Torvalds
//
// We have a per-user structure to keep track of how many
// processes, files etc the user has claimed, in order to be
// able to have per-user limits for system resources.
//

    struct binfmt_misc init_binfmt_misc = {
    .entries = HLIST_HEAD_INIT,
    .enabled = true,
    .entries_lock = __SPIN_LOCK_UNLOCKED(init_binfmt_misc.entries_lock),
    };
    EXPORT_SYMBOL_GPL(init_binfmt_misc);

//
// userns count is 1 for root user, 1 for init_uts_ns,
// and 1 for... ?
//
    struct user_namespace init_user_ns = {
    .ns = NS_COMMON_INIT(init_user_ns),
    .uid_map = {
    {
    .extent[0] = {
    .first = 0,
    .lower_first = 0,
    .count = 4294967295U,
    },
    .nr_extents = 1,
    },
    },
    .gid_map = {
    {
    .extent[0] = {
    .first = 0,
    .lower_first = 0,
    .count = 4294967295U,
    },
    .nr_extents = 1,
    },
    },
    .projid_map = {
    {
    .extent[0] = {
    .first = 0,
    .lower_first = 0,
    .count = 4294967295U,
    },
    .nr_extents = 1,
    },
    },
    .owner = GLOBAL_ROOT_UID,
    .group = GLOBAL_ROOT_GID,
    .flags = USERNS_INIT_FLAGS,

    .keyring_name_list = LIST_HEAD_INIT(init_user_ns.keyring_name_list),
    .keyring_sem = __RWSEM_INITIALIZER(init_user_ns.keyring_sem),

    .binfmt_misc = &init_binfmt_misc,

    };
    EXPORT_SYMBOL_GPL(init_user_ns);
//
// UID task count cache, to get fast user lookup in "alloc_uid"
// when changing user ID's (ie setuid() and friends).
//

    static struct kmem_cache *uid_cachep;
    static struct hlist_head uidhash_table[UIDHASH_SZ];
//
// The uidhash_lock is mostly taken from process context, but it is
// occasionally also taken from softirq/tasklet context, when
// task-structs get RCU-freed. Hence all locking must be softirq-safe.
// But free_uid() is also called with local interrupts disabled, and running
// local_bh_enable() with local interrupts disabled is an error - we'll run
// softirq callbacks, and they can unconditionally enable interrupts, and
// the caller of free_uid() didn't expect that..
//
// static DEFINE_SPINLOCK(uidhash_lock);
// root_user.__count is 1, for init task cred
    struct user_struct root_user = {
    .__count	= REFCOUNT_INIT(1),
    .uid		= GLOBAL_ROOT_UID,
    .ratelimit	= RATELIMIT_STATE_INIT(root_user.ratelimit, 0, 0),
    };
//
// These routines must be called with the uidhash spinlock held!
//
#[no_mangle]
unsafe extern "C" fn uid_hash_insert(up: *mut user_struct, hashent: *mut hlist_head) {
    hlist_add_head(&up.uidhash_node, hashent);
    }
#[no_mangle]
unsafe extern "C" fn uid_hash_remove(up: *mut user_struct) {
    hlist_del_init(&up.uidhash_node);
    }
    static struct user_struct *uid_hash_find(kuid_t uid, struct hlist_head *hashent)
    {
    struct user_struct *user;
    hlist_for_each_entry(user, hashent, uidhash_node) {
    if (uid_eq(user.uid, uid)) {
    refcount_inc(&user.__count);
    return user;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn user_epoll_alloc(up: *mut user_struct) -> c_int {

    return percpu_counter_init(&up.epoll_watches, 0, GFP_KERNEL);

    return 0;

    }
#[no_mangle]
unsafe extern "C" fn user_epoll_free(up: *mut user_struct) {

    percpu_counter_destroy(&up.epoll_watches);

    }
// IRQs are disabled and uidhash_lock is held upon function entry.
// IRQ state (as stored in flags) is restored and uidhash_lock released
// upon function exit.
//
#[no_mangle]
unsafe extern "C" fn free_user(up: *mut user_struct, flags: c_ulong) {
    uid_hash_remove(up);
    spin_unlock_irqrestore(&uidhash_lock, flags);
    user_epoll_free(up);
    kmem_cache_free(uid_cachep, up);
    }
//
// Locate the user_struct for the passed UID.  If found, take a ref on it.  The
// caller must undo that ref with free_uid().
//
// If the user_struct could not be found, return NULL.
//
    struct user_struct *find_user(kuid_t uid)
    {
    struct user_struct *ret;
    unsigned long flags;
    spin_lock_irqsave(&uidhash_lock, flags);
    ret = uid_hash_find(uid, uidhashentry(uid));
    spin_unlock_irqrestore(&uidhash_lock, flags);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn free_uid(up: *mut user_struct) {
    unsigned long flags;
    if (!up)
    return;
    if (refcount_dec_and_lock_irqsave(&up.__count, &uidhash_lock, &flags))
    free_user(up, flags);
    }
    EXPORT_SYMBOL_GPL(free_uid);
    struct user_struct *alloc_uid(kuid_t uid)
    {
    struct hlist_head *hashent = uidhashentry(uid);
    struct user_struct *up, *new;
    spin_lock_irq(&uidhash_lock);
    up = uid_hash_find(uid, hashent);
    spin_unlock_irq(&uidhash_lock);
    if (!up) {
    new = kmem_cache_zalloc(uid_cachep, GFP_KERNEL);
    if (!new)
    return core::ptr::null_mut();
    new.uid = uid;
    refcount_set(&new.__count, 1);
    if (user_epoll_alloc(new)) {
    kmem_cache_free(uid_cachep, new);
    return core::ptr::null_mut();
    }
    ratelimit_state_init(&new.ratelimit, HZ, 100);
    ratelimit_set_flags(&new.ratelimit, RATELIMIT_MSG_ON_RELEASE);
//
// Before adding this, check whether we raced
// on adding the same user already..
//
    spin_lock_irq(&uidhash_lock);
    up = uid_hash_find(uid, hashent);
    if (up) {
    user_epoll_free(new);
    kmem_cache_free(uid_cachep, new);
    } else {
    uid_hash_insert(new, hashent);
    up = new;
    }
    spin_unlock_irq(&uidhash_lock);
    }
    return up;
    }
#[no_mangle]
unsafe extern "C" fn uid_cache_init() -> c_int {
    int n;
    uid_cachep = kmem_cache_create("uid_cache", sizeof(struct user_struct),
    0, SLAB_HWCACHE_ALIGN|SLAB_PANIC, core::ptr::null_mut());
    for(n = 0; n < UIDHASH_SZ; ++n)
    INIT_HLIST_HEAD(uidhash_table + n);
    if (user_epoll_alloc(&root_user))
    panic("root_user epoll percpu counter alloc failed");
// Insert the root user immediately (init already runs as root)
    spin_lock_irq(&uidhash_lock);
    uid_hash_insert(&root_user, uidhashentry(GLOBAL_ROOT_UID));
    spin_unlock_irq(&uidhash_lock);
    return 0;
    }
    subsys_initcall(uid_cache_init);
