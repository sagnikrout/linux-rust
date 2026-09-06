//! Automatically rewritten from C to Rust
//! Source: fs/fs_pin.c
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

    static DEFINE_SPINLOCK(pin_lock);
#[no_mangle]
pub unsafe extern "C" fn pin_remove(pin: *mut fs_pin) {
    void pin_remove(struct fs_pin *pin)
    {
    spin_lock(&pin_lock);
    hlist_del_init(&pin.m_list);
    hlist_del_init(&pin.s_list);
    spin_unlock(&pin_lock);
    spin_lock_irq(&pin.wait.lock);
    pin.done = 1;
    wake_up_locked(&pin.wait);
    spin_unlock_irq(&pin.wait.lock);
    }
#[no_mangle]
pub unsafe extern "C" fn pin_insert(pin: *mut fs_pin, m: *mut vfsmount) {
    void pin_insert(struct fs_pin *pin, struct vfsmount *m)
    {
    spin_lock(&pin_lock);
    hlist_add_head(&pin.s_list, &m.mnt_sb.s_pins);
    hlist_add_head(&pin.m_list, &real_mount(m).mnt_pins);
    spin_unlock(&pin_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn pin_kill(p: *mut fs_pin) {
    void pin_kill(struct fs_pin *p)
    {
    wait_queue_entry_t wait;
    if (!p) {
    rcu_read_unlock();
    return;
    }
    init_wait(&wait);
    spin_lock_irq(&p.wait.lock);
    if (likely(!p.done)) {
    p.done = -1;
    spin_unlock_irq(&p.wait.lock);
    rcu_read_unlock();
    p.kill(p);
    return;
    }
    if (p.done > 0) {
    spin_unlock_irq(&p.wait.lock);
    rcu_read_unlock();
    return;
    }
    __add_wait_queue(&p.wait, &wait);
    while (1) {
    set_current_state(TASK_UNINTERRUPTIBLE);
    spin_unlock_irq(&p.wait.lock);
    rcu_read_unlock();
    schedule();
    rcu_read_lock();
    if (likely(list_empty(&wait.entry)))
    break;
// OK, we know p couldn't have been freed yet
    spin_lock_irq(&p.wait.lock);
    if (p.done > 0) {
    spin_unlock_irq(&p.wait.lock);
    break;
    }
    }
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn mnt_pin_kill(m: *mut mount) {
    void mnt_pin_kill(struct mount *m)
    {
    while (1) {
    struct hlist_node *p;
    rcu_read_lock();
    p = READ_ONCE(m.mnt_pins.first);
    if (!p) {
    rcu_read_unlock();
    break;
    }
    pin_kill(hlist_entry(p, struct fs_pin, m_list));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn group_pin_kill(p: *mut hlist_head) {
    void group_pin_kill(struct hlist_head *p)
    {
    while (1) {
    struct hlist_node *q;
    rcu_read_lock();
    q = READ_ONCE(p.first);
    if (!q) {
    rcu_read_unlock();
    break;
    }
    pin_kill(hlist_entry(q, struct fs_pin, s_list));
    }
    }
