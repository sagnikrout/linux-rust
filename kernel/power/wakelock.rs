//! Automatically rewritten from C to Rust
//! Source: kernel/power/wakelock.c
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
//
// kernel/power/wakelock.c
//
// User space wakeup sources support.
//
// Copyright (C) 2012 Rafael J. Wysocki <rjw@sisk.pl>
//
// This code is based on the analogous interface allowing user space to
// manipulate wakelocks on Android.
//

    static DEFINE_MUTEX(wakelocks_lock);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wakelock {
    pub name: *mut c_char,
    pub node: rb_node,
    pub ws: *mut wakeup_source,

    pub lru: list_head,

}

    let mut wakelocks_tree: static struct rb_root = RB_ROOT;
#[no_mangle]
pub unsafe extern "C" fn pm_show_wakelocks(buf: *mut c_char, show_active: bool) -> isize {
    ssize_t pm_show_wakelocks(char *buf, bool show_active)
    {
    struct rb_node *node;
    struct wakelock *wl;
    let mut len: c_int = 0;
    mutex_lock(&wakelocks_lock);
    for (node = rb_first(&wakelocks_tree); node; node = rb_next(node)) {
    wl = rb_entry(node, struct wakelock, node);
    if (wl.ws.active == show_active)
    len += sysfs_emit_at(buf, len, "%s ", wl.name);
    }
    if (len > 0)
    --len;
    len += sysfs_emit_at(buf, len, "\n");
    mutex_unlock(&wakelocks_lock);
    return len;
    }

    static unsigned int number_of_wakelocks;
#[no_mangle]
pub unsafe extern "C" fn wakelocks_limit_exceeded() -> bool {
    static inline bool wakelocks_limit_exceeded(void)
    {
    return number_of_wakelocks >= CONFIG_PM_WAKELOCKS_LIMIT;
    }
#[no_mangle]
pub unsafe extern "C" fn increment_wakelocks_number() {
    static inline void increment_wakelocks_number(void)
    {
    number_of_wakelocks++;
    }
#[no_mangle]
pub unsafe extern "C" fn decrement_wakelocks_number() {
    static inline void decrement_wakelocks_number(void)
    {
    number_of_wakelocks--;
    }

    static inline bool wakelocks_limit_exceeded(void) { return false; }
    static inline void increment_wakelocks_number(void) {}
    static inline void decrement_wakelocks_number(void) {}

pub const WL_GC_COUNT_MAX: c_int = 100;
pub const WL_GC_TIME_SEC: c_int = 300;
    static void __wakelocks_gc(struct work_struct *work);
    static LIST_HEAD(wakelocks_lru_list);
    static DECLARE_WORK(wakelock_work, __wakelocks_gc);
    static unsigned int wakelocks_gc_count;
#[no_mangle]
pub unsafe extern "C" fn wakelocks_lru_add(wl: *mut wakelock) {
    static inline void wakelocks_lru_add(struct wakelock *wl)
    {
    list_add(&wl.lru, &wakelocks_lru_list);
    }
#[no_mangle]
pub unsafe extern "C" fn wakelocks_lru_most_recent(wl: *mut wakelock) {
    static inline void wakelocks_lru_most_recent(struct wakelock *wl)
    {
    list_move(&wl.lru, &wakelocks_lru_list);
    }
#[no_mangle]
unsafe extern "C" fn __wakelocks_gc(work: *mut work_struct) {
    static void __wakelocks_gc(struct work_struct *work)
    {
    struct wakelock *wl, *aux;
    ktime_t now;
    mutex_lock(&wakelocks_lock);
    now = ktime_get();
    list_for_each_entry_safe_reverse(wl, aux, &wakelocks_lru_list, lru) {
    u64 idle_time_ns;
    bool active;
    spin_lock_irq(&wl.ws.lock);
    idle_time_ns = ktime_to_ns(ktime_sub(now, wl.ws.last_time));
    active = wl.ws.active;
    spin_unlock_irq(&wl.ws.lock);
    if (idle_time_ns < ((u64)WL_GC_TIME_SEC * NSEC_PER_SEC))
    break;
    if (!active) {
    wakeup_source_unregister(wl.ws);
    rb_erase(&wl.node, &wakelocks_tree);
    list_del(&wl.lru);
    kfree(wl.name);
    kfree(wl);
    decrement_wakelocks_number();
    }
    }
    wakelocks_gc_count = 0;
    mutex_unlock(&wakelocks_lock);
    }
#[no_mangle]
unsafe extern "C" fn wakelocks_gc() {
    static void wakelocks_gc(void)
    {
    if (++wakelocks_gc_count <= WL_GC_COUNT_MAX)
    return;
    schedule_work(&wakelock_work);
    }

    static inline void wakelocks_lru_add(struct wakelock *wl) {}
    static inline void wakelocks_lru_most_recent(struct wakelock *wl) {}
    static inline void wakelocks_gc(void) {}

    static struct wakelock *wakelock_lookup_add(const char *name, size_t len,
    bool add_if_not_found)
    {
    struct rb_node **node = &wakelocks_tree.rb_node;
    struct rb_node *parent = *node;
    struct wakelock *wl;
    while (*node) {
    int diff;
    parent = *node;
    wl = rb_entry(*node, struct wakelock, node);
    diff = strncmp(name, wl.name, len);
    if (diff == 0) {
    if (wl.name[len])
    diff = -1;
    else
    return wl;
    }
    if (diff < 0)
    node = &(*node).rb_left;
    else
    node = &(*node).rb_right;
    }
    if (!add_if_not_found)
    return ERR_PTR(-EINVAL);
    if (wakelocks_limit_exceeded())
    return ERR_PTR(-ENOSPC);
// Not found, we have to add a new one.
    wl = kzalloc_obj(*wl);
    if (!wl)
    return ERR_PTR(-ENOMEM);
    wl.name = kstrndup(name, len, GFP_KERNEL);
    if (!wl.name) {
    kfree(wl);
    return ERR_PTR(-ENOMEM);
    }
    wl.ws = wakeup_source_register(core::ptr::null_mut(), wl.name);
    if (!wl.ws) {
    kfree(wl.name);
    kfree(wl);
    return ERR_PTR(-ENOMEM);
    }
    wl.ws.last_time = ktime_get();
    rb_link_node(&wl.node, parent, node);
    rb_insert_color(&wl.node, &wakelocks_tree);
    wakelocks_lru_add(wl);
    increment_wakelocks_number();
    return wl;
    }
#[no_mangle]
pub unsafe extern "C" fn pm_wake_lock(buf: *const c_char) -> c_int {
    int pm_wake_lock(const char *buf)
    {
    const char *str = buf;
    struct wakelock *wl;
    let mut timeout_ns: u64 = 0;
    size_t len;
    let mut ret: c_int = 0;
    if (!capable(CAP_BLOCK_SUSPEND))
    return -EPERM;
    while (*str && !isspace(*str))
    str++;
    len = str - buf;
    if (!len)
    return -EINVAL;
    if (*str && *str != '\n') {
// Find out if there's a valid timeout string appended.
    ret = kstrtou64(skip_spaces(str), 10, &timeout_ns);
    if (ret)
    return -EINVAL;
    }
    mutex_lock(&wakelocks_lock);
    wl = wakelock_lookup_add(buf, len, true);
    if (IS_ERR(wl)) {
    ret = PTR_ERR(wl);
    goto out;
    }
    if (timeout_ns) {
    let mut timeout_ms: u64 = timeout_ns + NSEC_PER_MSEC - 1;
    do_div(timeout_ms, NSEC_PER_MSEC);
    __pm_wakeup_event(wl.ws, timeout_ms);
    } else {
    __pm_stay_awake(wl.ws);
    }
    wakelocks_lru_most_recent(wl);
    out:
    mutex_unlock(&wakelocks_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn pm_wake_unlock(buf: *const c_char) -> c_int {
    int pm_wake_unlock(const char *buf)
    {
    struct wakelock *wl;
    size_t len;
    let mut ret: c_int = 0;
    if (!capable(CAP_BLOCK_SUSPEND))
    return -EPERM;
    len = strlen(buf);
    if (!len)
    return -EINVAL;
    if (buf[len-1] == '\n')
    len--;
    if (!len)
    return -EINVAL;
    mutex_lock(&wakelocks_lock);
    wl = wakelock_lookup_add(buf, len, false);
    if (IS_ERR(wl)) {
    ret = PTR_ERR(wl);
    goto out;
    }
    __pm_relax(wl.ws);
    wakelocks_lru_most_recent(wl);
    wakelocks_gc();
    out:
    mutex_unlock(&wakelocks_lock);
    return ret;
    }
