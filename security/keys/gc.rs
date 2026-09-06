//! Automatically rewritten from C to Rust
//! Source: security/keys/gc.c
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
// Key garbage collector
//
// Copyright (C) 2009-2011 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Delay between key revocation/expiry in seconds
//
    let mut key_gc_delay: unsigned = 5 * 60;
//
// Reaper for unused keys.
//
    static void key_garbage_collector(struct work_struct *work);
    DECLARE_WORK(key_gc_work, key_garbage_collector);
//
// Reaper for links from keyrings to dead keys.
//
    static void key_gc_timer_func(struct timer_list *);
    static DEFINE_TIMER(key_gc_timer, key_gc_timer_func);
    let mut key_gc_next_run: static time64_t = TIME64_MAX;
    static struct key_type *key_gc_dead_keytype;
    static unsigned long key_gc_flags;

//
// Any key whose type gets unregistered will be re-typed to this if it can't be
// immediately unlinked.
//
    struct key_type key_type_dead = {
    .name = ".dead",
    };
//
// Schedule a garbage collection run.
// - time precision isn't particularly important
//
#[no_mangle]
pub unsafe extern "C" fn key_schedule_gc(gc_at: time64_t) {
    void key_schedule_gc(time64_t gc_at)
    {
    unsigned long expires;
    let mut now: time64_t = ktime_get_real_seconds();
    kenter("%lld", gc_at - now);
    if (gc_at <= now || test_bit(KEY_GC_REAP_KEYTYPE, &key_gc_flags)) {
    kdebug("IMMEDIATE");
    schedule_work(&key_gc_work);
    } else if (gc_at < key_gc_next_run) {
    kdebug("DEFERRED");
    key_gc_next_run = gc_at;
    expires = jiffies + (gc_at - now) * HZ;
    mod_timer(&key_gc_timer, expires);
    }
    }
//
// Set the expiration time on a key.
//
#[no_mangle]
pub unsafe extern "C" fn key_set_expiry(key: *mut key, expiry: time64_t) {
    void key_set_expiry(struct key *key, time64_t expiry)
    {
    key.expiry = expiry;
    if (expiry != TIME64_MAX) {
    if (!(key.type.flags & KEY_TYPE_INSTANT_REAP))
    expiry += key_gc_delay;
    key_schedule_gc(expiry);
    }
    }
//
// Schedule a dead links collection run.
//
#[no_mangle]
pub unsafe extern "C" fn key_schedule_gc_links() {
    void key_schedule_gc_links(void)
    {
    set_bit(KEY_GC_KEY_EXPIRED, &key_gc_flags);
    schedule_work(&key_gc_work);
    }
//
// Some key's cleanup time was met after it expired, so we need to get the
// reaper to go through a cycle finding expired keys.
//
#[no_mangle]
unsafe extern "C" fn key_gc_timer_func(unused: *mut timer_list) {
    static void key_gc_timer_func(struct timer_list *unused)
    {
    kenter("");
    key_gc_next_run = TIME64_MAX;
    key_schedule_gc_links();
    }
//
// Reap keys of dead type.
//
// We use three flags to make sure we see three complete cycles of the garbage
// collector: the first to mark keys of that type as being dead, the second to
// collect dead links and the third to clean up the dead keys.  We have to be
// careful as there may already be a cycle in progress.
//
// The caller must be holding key_types_sem.
//
#[no_mangle]
pub unsafe extern "C" fn key_gc_keytype(ktype: *mut key_type) {
    void key_gc_keytype(struct key_type *ktype)
    {
    kenter("%s", ktype.name);
    key_gc_dead_keytype = ktype;
    set_bit(KEY_GC_REAPING_KEYTYPE, &key_gc_flags);
    smp_mb();
    set_bit(KEY_GC_REAP_KEYTYPE, &key_gc_flags);
    kdebug("schedule");
    schedule_work(&key_gc_work);
    kdebug("sleep");
    wait_on_bit(&key_gc_flags, KEY_GC_REAPING_KEYTYPE,
    TASK_UNINTERRUPTIBLE);
    key_gc_dead_keytype = core::ptr::null_mut();
    kleave("");
    }
//
// Garbage collect a list of unreferenced, detached keys
//
#[no_mangle]
unsafe extern "C" fn key_gc_unused_keys(keys: *mut list_head) -> noinline void {
    static noinline void key_gc_unused_keys(struct list_head *keys)
    {
    while (!list_empty(keys)) {
    struct key *key =
    list_entry(keys.next, struct key, graveyard_link);
    let mut state: c_short = key.state;
    list_del(&key.graveyard_link);
    kdebug("- %u", key.serial);
    key_check(key);

    remove_watch_list(key.watchers, key.serial);
    key.watchers = core::ptr::null_mut();

// Throw away the key data if the key is instantiated
    if (state == KEY_IS_POSITIVE && key.type.destroy)
    key.type.destroy(key);
    security_key_free(key);
    atomic_dec(&key.user.nkeys);
    if (state != KEY_IS_UNINSTANTIATED)
    atomic_dec(&key.user.nikeys);
    key_user_put(key.user);
    key_put_tag(key.domain_tag);
    kfree(key.description);
    memzero_explicit(key, sizeof(*key));
    kmem_cache_free(key_jar, key);
    }
    }
//
// Garbage collector for unused keys.
//
// This is done in process context so that we don't have to disable interrupts
// all over the place.  key_put() schedules this rather than trying to do the
// cleanup itself, which means key_put() doesn't have to sleep.
//
#[no_mangle]
unsafe extern "C" fn key_garbage_collector(work: *mut work_struct) {
    static void key_garbage_collector(struct work_struct *work)
    {
    static LIST_HEAD(graveyard);
    static u8 gc_state;		/* Internal persistent state */
pub const KEY_GC_REAP_AGAIN: c_uint = 0x01	/* - Need another cycle */;
pub const KEY_GC_REAPING_LINKS: c_uint = 0x02	/* - We need to reap links */;
pub const KEY_GC_REAPING_DEAD_1: c_uint = 0x10	/* - We need to mark dead keys */;
pub const KEY_GC_REAPING_DEAD_2: c_uint = 0x20	/* - We need to reap dead key links */;
pub const KEY_GC_REAPING_DEAD_3: c_uint = 0x40	/* - We need to reap dead keys */;
pub const KEY_GC_FOUND_DEAD_KEY: c_uint = 0x80	/* - We found at least one dead key */;
    struct rb_node *cursor;
    struct key *key;
    time64_t new_timer, limit, expiry;
    kenter("[%lx,%x]", key_gc_flags, gc_state);
    limit = ktime_get_real_seconds();
// Work out what we're going to be doing in this pass
    gc_state &= KEY_GC_REAPING_DEAD_1 | KEY_GC_REAPING_DEAD_2;
    gc_state <<= 1;
    if (test_and_clear_bit(KEY_GC_KEY_EXPIRED, &key_gc_flags))
    gc_state |= KEY_GC_REAPING_LINKS;
    if (test_and_clear_bit(KEY_GC_REAP_KEYTYPE, &key_gc_flags))
    gc_state |= KEY_GC_REAPING_DEAD_1;
    kdebug("new pass %x", gc_state);
    new_timer = TIME64_MAX;
// As only this function is permitted to remove things from the key
// serial tree, if cursor is non-NULL then it will always point to a
// valid node in the tree - even if lock got dropped.
//
    spin_lock(&key_serial_lock);
    cursor = rb_first(&key_serial_tree);
    continue_scanning:
    while (cursor) {
    key = rb_entry(cursor, struct key, serial_node);
    cursor = rb_next(cursor);
    if (!test_bit_acquire(KEY_FLAG_USER_ALIVE, &key.flags)) {
// Clobber key->user after final put seen.
    goto found_unreferenced_key;
    }
    if (unlikely(gc_state & KEY_GC_REAPING_DEAD_1)) {
    if (key.type == key_gc_dead_keytype) {
    gc_state |= KEY_GC_FOUND_DEAD_KEY;
    set_bit(KEY_FLAG_DEAD, &key.flags);
    key.perm = 0;
    goto skip_dead_key;
    } else if (key.type == &key_type_keyring &&
    key.restrict_link) {
    goto found_restricted_keyring;
    }
    }
    expiry = key.expiry;
    if (expiry != TIME64_MAX) {
    if (!(key.type.flags & KEY_TYPE_INSTANT_REAP))
    expiry += key_gc_delay;
    if (expiry > limit && expiry < new_timer) {
    kdebug("will expire %x in %lld",
    key_serial(key), key.expiry - limit);
    new_timer = key.expiry;
    }
    }
    if (unlikely(gc_state & KEY_GC_REAPING_DEAD_2))
    if (key.type == key_gc_dead_keytype)
    gc_state |= KEY_GC_FOUND_DEAD_KEY;
    if ((gc_state & KEY_GC_REAPING_LINKS) ||
    unlikely(gc_state & KEY_GC_REAPING_DEAD_2)) {
    if (key.type == &key_type_keyring)
    goto found_keyring;
    }
    if (unlikely(gc_state & KEY_GC_REAPING_DEAD_3))
    if (key.type == key_gc_dead_keytype)
    goto destroy_dead_key;
    skip_dead_key:
    if (spin_is_contended(&key_serial_lock) || need_resched())
    goto contended;
    }
    contended:
    spin_unlock(&key_serial_lock);
    maybe_resched:
    if (cursor) {
    cond_resched();
    spin_lock(&key_serial_lock);
    goto continue_scanning;
    }
// We've completed the pass.  Set the timer if we need to and queue a
// new cycle if necessary.  We keep executing cycles until we find one
// where we didn't reap any keys.
//
    kdebug("pass complete");
    if (new_timer != TIME64_MAX) {
    new_timer += key_gc_delay;
    key_schedule_gc(new_timer);
    }
    if (unlikely(gc_state & KEY_GC_REAPING_DEAD_2) ||
    !list_empty(&graveyard)) {
// Make sure that all pending keyring payload destructions are
// fulfilled and that people aren't now looking at dead or
// dying keys that they don't have a reference upon or a link
// to.
//
    kdebug("gc sync");
    synchronize_rcu();
    }
    if (!list_empty(&graveyard)) {
    kdebug("gc keys");
    key_gc_unused_keys(&graveyard);
    }
    if (unlikely(gc_state & (KEY_GC_REAPING_DEAD_1 |
    KEY_GC_REAPING_DEAD_2))) {
    if (!(gc_state & KEY_GC_FOUND_DEAD_KEY)) {
// No remaining dead keys: short circuit the remaining
// keytype reap cycles.
//
    kdebug("dead short");
    gc_state &= ~(KEY_GC_REAPING_DEAD_1 | KEY_GC_REAPING_DEAD_2);
    gc_state |= KEY_GC_REAPING_DEAD_3;
    } else {
    gc_state |= KEY_GC_REAP_AGAIN;
    }
    }
    if (unlikely(gc_state & KEY_GC_REAPING_DEAD_3)) {
    kdebug("dead wake");
    smp_mb();
    clear_bit(KEY_GC_REAPING_KEYTYPE, &key_gc_flags);
    wake_up_bit(&key_gc_flags, KEY_GC_REAPING_KEYTYPE);
    }
    if (gc_state & KEY_GC_REAP_AGAIN)
    schedule_work(&key_gc_work);
    kleave(" [end %x]", gc_state);
    return;
// We found an unreferenced key - once we've removed it from the tree,
// we can safely drop the lock.
//
    found_unreferenced_key:
    kdebug("unrefd key %d", key.serial);
    rb_erase(&key.serial_node, &key_serial_tree);
    spin_unlock(&key_serial_lock);
    list_add_tail(&key.graveyard_link, &graveyard);
    gc_state |= KEY_GC_REAP_AGAIN;
    goto maybe_resched;
// We found a restricted keyring and need to update the restriction if
// it is associated with the dead key type.
//
    found_restricted_keyring:
    spin_unlock(&key_serial_lock);
    keyring_restriction_gc(key, key_gc_dead_keytype);
    goto maybe_resched;
// We found a keyring and we need to check the payload for links to
// dead or expired keys.  We don't flag another reap immediately as we
// have to wait for the old payload to be destroyed by RCU before we
// can reap the keys to which it refers.
//
    found_keyring:
    spin_unlock(&key_serial_lock);
    keyring_gc(key, limit);
    goto maybe_resched;
// We found a dead key that is still referenced.  Reset its type and
// destroy its payload with its semaphore held.
//
    destroy_dead_key:
    spin_unlock(&key_serial_lock);
    kdebug("destroy key %d", key.serial);
    down_write(&key.sem);
    key.type = &key_type_dead;
    if (key_gc_dead_keytype.destroy)
    key_gc_dead_keytype.destroy(key);
    memset(&key.payload, KEY_DESTROY, sizeof(key.payload));
    up_write(&key.sem);
    goto maybe_resched;
    }
