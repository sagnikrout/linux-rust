//! Automatically rewritten from C to Rust
//! Source: mm/hugetlb_cgroup.c
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


// SPDX-License-Identifier: LGPL-2.1
//
// Copyright IBM Corporation, 2012
// Author Aneesh Kumar K.V <aneesh.kumar@linux.vnet.ibm.com>
//
// Cgroup v2
// Copyright (C) 2019 Red Hat, Inc.
// Author: Giuseppe Scrivano <gscrivan@redhat.com>
//

// Use t->m[0] to encode the offset

pub static mut root_h_cgroup: *mut c_void = core::ptr::null_mut();
pub static mut dfl_files: *mut c_void = core::ptr::null_mut();
pub static mut legacy_files: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn __hugetlb_cgroup_counter_from_cgroup(h_cg: *mut hugetlb_cgroup, idx: c_int, rsvd: bool) -> *mut c_void {
    if (rsvd) {
    return &h_cg.rsvd_hugepage[idx];
    }
    return &h_cg.hugepage[idx];
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_counter_from_cgroup(h_cg: *mut hugetlb_cgroup, idx: c_int) -> *mut c_void {
    return __hugetlb_cgroup_counter_from_cgroup(h_cg, idx, false);
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_counter_from_cgroup_rsvd(h_cg: *mut hugetlb_cgroup, idx: c_int) -> *mut c_void {
    return __hugetlb_cgroup_counter_from_cgroup(h_cg, idx, true);
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_from_css(s: *mut cgroup_subsys_state) -> *mut c_void {
    return s ? container_of!(s, hugetlb_cgroup, css) : core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_from_task(task: *mut task_struct) -> *mut c_void {
    return hugetlb_cgroup_from_css(task_css(task, hugetlb_cgrp_id));
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_is_root(h_cg: *mut hugetlb_cgroup) -> bool {
    return (h_cg == root_h_cgroup);
    }
#[no_mangle]
pub unsafe extern "C" fn parent_hugetlb_cgroup(h_cg: *mut hugetlb_cgroup) -> *mut c_void {
    return hugetlb_cgroup_from_css(h_cg.css.parent);
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_have_usage(h_cg: *mut hugetlb_cgroup) -> bool {
pub static mut h: *mut c_void = core::ptr::null_mut();
    for_each_hstate(h) {
    if (page_counter_read(
    hugetlb_cgroup_counter_from_cgroup(h_cg, hstate_index(h)))) {
    return true;
    }
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_init(h_cgroup: *mut hugetlb_cgroup, parent_h_cgroup: *mut hugetlb_cgroup) {
    let mut idx = 0;
    while (idx < HUGE_MAX_HSTATE) {
    struct page_counter *fault, *fault_parent = core::ptr::null_mut();
    struct page_counter *rsvd, *rsvd_parent = core::ptr::null_mut();
    let mut limit = 0;
    let mut ret = 0;
    if (parent_h_cgroup) {
    fault_parent = hugetlb_cgroup_counter_from_cgroup(
    parent_h_cgroup, idx);
    rsvd_parent = hugetlb_cgroup_counter_from_cgroup_rsvd(
    parent_h_cgroup, idx);
    }
    fault = hugetlb_cgroup_counter_from_cgroup(h_cgroup, idx);
    rsvd = hugetlb_cgroup_counter_from_cgroup_rsvd(h_cgroup, idx);
    page_counter_init(fault, fault_parent, false);
    page_counter_init(rsvd, rsvd_parent, false);
    if (!cgroup_subsys_on_dfl(hugetlb_cgrp_subsys)) {
    fault.track_failcnt = true;
    rsvd.track_failcnt = true;
    }
    limit = round_down(PAGE_COUNTER_MAX,
    pages_per_huge_page(&hstates[idx]));
    ret = page_counter_set_max(fault, limit);
    VM_WARN_ON_ONCE(ret);
    ret = page_counter_set_max(rsvd, limit);
    VM_WARN_ON_ONCE(ret);
    }
    }
#[no_mangle]
unsafe extern "C" fn hugetlb_cgroup_free(h_cgroup: *mut hugetlb_cgroup) {
    let mut node = 0;
    for_each_node(node) {
    kfree(h_cgroup.nodeinfo[node]);
    }
    kfree(h_cgroup);
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_css_alloc(parent_css: *mut cgroup_subsys_state) -> *mut c_void {
    let mut parent_h_cgroup = hugetlb_cgroup_from_css(parent_css);
pub static mut h_cgroup: *mut c_void = core::ptr::null_mut();
    let mut node = 0;
    h_cgroup = kzalloc_flex(*h_cgroup, nodeinfo, nr_node_ids);
    if (!h_cgroup) {
    return ERR_PTR(-ENOMEM);
    }
    if (!parent_h_cgroup) {
    root_h_cgroup = h_cgroup;
    }
//
// TODO: this routine can waste much memory for nodes which will
// never be onlined. It's better to use memory hotplug callback
// function.
//
    for_each_node(node) {
// Set node_to_alloc to NUMA_NO_NODE for offline nodes.
    let mut node_to_alloc = node_state(node, N_NORMAL_MEMORY) ? node : NUMA_NO_NODE;
    h_cgroup.nodeinfo[node] =
    kzalloc_node(sizeof!(hugetlb_cgroup_per_node),
    GFP_KERNEL, node_to_alloc);
    if (!h_cgroup.nodeinfo[node]) {
// goto;
    }
    }
    hugetlb_cgroup_init(h_cgroup, parent_h_cgroup);
    return &h_cgroup.css;
// label;
    hugetlb_cgroup_free(h_cgroup);
    return ERR_PTR(-ENOMEM);
    }
#[no_mangle]
unsafe extern "C" fn hugetlb_cgroup_css_free(css: *mut cgroup_subsys_state) {
    hugetlb_cgroup_free(hugetlb_cgroup_from_css(css));
    }
//
// Should be called with hugetlb_lock held.
// Since we are holding hugetlb_lock, pages cannot get moved from
// active list or uncharged from the cgroup, So no need to get
// page reference and test for page active here. This function
// cannot fail.
//
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_move_parent(idx: c_int, h_cg: *mut hugetlb_cgroup, folio: *mut folio) {
    let mut nr_pages = 0;
pub static mut counter: *mut c_void = core::ptr::null_mut();
pub static mut hcg: *mut c_void = core::ptr::null_mut();
    let mut parent = parent_hugetlb_cgroup(h_cg);
    hcg = hugetlb_cgroup_from_folio(folio);
//
// We can have pages in active list without any cgroup
// ie, hugepage with less than 3 pages. We can safely
// ignore those pages.
//
    if (!hcg || hcg != h_cg) {
// goto;
    }
    nr_pages = folio_nr_pages(folio);
    if (!parent) {
    parent = root_h_cgroup;
// root has no limit
    page_counter_charge(&parent.hugepage[idx], nr_pages);
    }
    counter = &h_cg.hugepage[idx];
// Take the pages off the local counter
    page_counter_cancel(counter, nr_pages);
    set_hugetlb_cgroup(folio, parent);
// label;
    return;
    }
//
// Force the hugetlb cgroup to empty the hugetlb resources by moving them to
// the parent cgroup.
//
#[no_mangle]
unsafe extern "C" fn hugetlb_cgroup_css_offline(css: *mut cgroup_subsys_state) {
    let mut h_cg = hugetlb_cgroup_from_css(css);
pub static mut h: *mut c_void = core::ptr::null_mut();
pub static mut folio: *mut c_void = core::ptr::null_mut();
    do {
    for_each_hstate(h) {
    spin_lock_irq(&hugetlb_lock);
    list_for_each_entry(folio, &h.hugepage_activelist, lru) {
    hugetlb_cgroup_move_parent(hstate_index(h), h_cg, folio);
    }
    spin_unlock_irq(&hugetlb_lock);
    }
    cond_resched();
    } while (hugetlb_cgroup_have_usage(h_cg));
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_event(hugetlb: *mut hugetlb_cgroup, idx: c_int, event: hugetlb_memory_event) {
    atomic_long_inc(&hugetlb.events_local[idx][event]);
    cgroup_file_notify(&hugetlb.events_local_file[idx]);
    do {
    atomic_long_inc(&hugetlb.events[idx][event]);
    cgroup_file_notify(&hugetlb.events_file[idx]);
    } while ((hugetlb = parent_hugetlb_cgroup(hugetlb)) &&
    !hugetlb_cgroup_is_root(hugetlb));
    }
#[no_mangle]
pub unsafe extern "C" fn __hugetlb_cgroup_charge_cgroup(idx: c_int, nr_pages: c_ulong, ptr: *mut *mut hugetlb_cgroup, rsvd: bool) -> c_int {
pub static mut ret: c_int = 0;
pub static mut counter: *mut c_void = core::ptr::null_mut();
    let mut h_cg = core::ptr::null_mut();
    if (hugetlb_cgroup_disabled()) {
// goto;
    }
// label;
    rcu_read_lock();
    h_cg = hugetlb_cgroup_from_task(current);
    if (!css_tryget(&h_cg.css)) {
    rcu_read_unlock();
// goto;
    }
    rcu_read_unlock();
    if (!page_counter_try_charge(
    __hugetlb_cgroup_counter_from_cgroup(h_cg, idx, rsvd),
    nr_pages, &counter)) {
    ret = -ENOMEM;
    hugetlb_event(h_cg, idx, HUGETLB_MAX);
    css_put(&h_cg.css);
// goto;
    }
// Reservations take a reference to the css because they do not get
// reparented.
//
    if (!rsvd) {
    css_put(&h_cg.css);
    }
// label;
// ptr = h_cg;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_charge_cgroup(idx: c_int, nr_pages: c_ulong, ptr: *mut *mut hugetlb_cgroup) -> c_int {
    return __hugetlb_cgroup_charge_cgroup(idx, nr_pages, ptr, false);
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_charge_cgroup_rsvd(idx: c_int, nr_pages: c_ulong, ptr: *mut *mut hugetlb_cgroup) -> c_int {
    return __hugetlb_cgroup_charge_cgroup(idx, nr_pages, ptr, true);
    }
// Should be called with hugetlb_lock held
#[no_mangle]
pub unsafe extern "C" fn __hugetlb_cgroup_commit_charge(idx: c_int, nr_pages: c_ulong, h_cg: *mut hugetlb_cgroup, folio: *mut folio, rsvd: bool) {
    if (hugetlb_cgroup_disabled() || !h_cg) {
    return;
    }
    lockdep_assert_held(&hugetlb_lock);
    __set_hugetlb_cgroup(folio, h_cg, rsvd);
    if (!rsvd) {
    let mut usage = h_cg.nodeinfo[folio_nid(folio)].usage[idx];
//
// This write is not atomic due to fetching usage and writing
// to it, but that's fine because we call this with
// hugetlb_lock held anyway.
//
    WRITE_ONCE(h_cg.nodeinfo[folio_nid(folio)].usage[idx],
    usage + nr_pages);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_commit_charge(idx: c_int, nr_pages: c_ulong, h_cg: *mut hugetlb_cgroup, folio: *mut folio) {
    __hugetlb_cgroup_commit_charge(idx, nr_pages, h_cg, folio, false);
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_commit_charge_rsvd(idx: c_int, nr_pages: c_ulong, h_cg: *mut hugetlb_cgroup, folio: *mut folio) {
    __hugetlb_cgroup_commit_charge(idx, nr_pages, h_cg, folio, true);
    }
//
// Should be called with hugetlb_lock held
//
#[no_mangle]
pub unsafe extern "C" fn __hugetlb_cgroup_uncharge_folio(idx: c_int, nr_pages: c_ulong, folio: *mut folio, rsvd: bool) {
pub static mut h_cg: *mut c_void = core::ptr::null_mut();
    if (hugetlb_cgroup_disabled()) {
    return;
    }
    lockdep_assert_held(&hugetlb_lock);
    h_cg = __hugetlb_cgroup_from_folio(folio, rsvd);
    if (unlikely(!h_cg)) {
    return;
    }
    __set_hugetlb_cgroup(folio, core::ptr::null_mut(), rsvd);
    page_counter_uncharge(__hugetlb_cgroup_counter_from_cgroup(h_cg, idx,
    rsvd),
    nr_pages);
    if (rsvd) {
    css_put(&h_cg.css);
    }
    else {
    let mut usage = h_cg.nodeinfo[folio_nid(folio)].usage[idx];
//
// This write is not atomic due to fetching usage and writing
// to it, but that's fine because we call this with
// hugetlb_lock held anyway.
//
    WRITE_ONCE(h_cg.nodeinfo[folio_nid(folio)].usage[idx],
    usage - nr_pages);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_uncharge_folio(idx: c_int, nr_pages: c_ulong, folio: *mut folio) {
    __hugetlb_cgroup_uncharge_folio(idx, nr_pages, folio, false);
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_uncharge_folio_rsvd(idx: c_int, nr_pages: c_ulong, folio: *mut folio) {
    __hugetlb_cgroup_uncharge_folio(idx, nr_pages, folio, true);
    }
#[no_mangle]
pub unsafe extern "C" fn __hugetlb_cgroup_uncharge_cgroup(idx: c_int, nr_pages: c_ulong, h_cg: *mut hugetlb_cgroup, rsvd: bool) {
    if (hugetlb_cgroup_disabled() || !h_cg) {
    return;
    }
    page_counter_uncharge(__hugetlb_cgroup_counter_from_cgroup(h_cg, idx,
    rsvd),
    nr_pages);
    if (rsvd) {
    css_put(&h_cg.css);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_uncharge_cgroup(idx: c_int, nr_pages: c_ulong, h_cg: *mut hugetlb_cgroup) {
    __hugetlb_cgroup_uncharge_cgroup(idx, nr_pages, h_cg, false);
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_uncharge_cgroup_rsvd(idx: c_int, nr_pages: c_ulong, h_cg: *mut hugetlb_cgroup) {
    __hugetlb_cgroup_uncharge_cgroup(idx, nr_pages, h_cg, true);
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_uncharge_counter(resv: *mut resv_map, start: c_ulong, end: c_ulong) {
    if (hugetlb_cgroup_disabled() || !resv || !resv.reservation_counter ||
    !resv.css) {
    return;
    }
    page_counter_uncharge(resv.reservation_counter,
    (end - start) * resv.pages_per_hpage);
    css_put(resv.css);
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_uncharge_file_region(resv: *mut resv_map, rg: *mut file_region, nr_pages: c_ulong, region_del: bool) {
    if (hugetlb_cgroup_disabled() || !resv || !rg || !nr_pages) {
    return;
    }
    if (rg.reservation_counter && resv.pages_per_hpage &&
    !resv.reservation_counter) {
    page_counter_uncharge(rg.reservation_counter,
    nr_pages * resv.pages_per_hpage);
//
// Only do css_put(rg->css) when we delete the entire region
// because one file_region must hold exactly one css reference.
//
    if (region_del) {
    css_put(rg.css);
    }
    }
    }
    enum {
    RES_USAGE,
    RES_RSVD_USAGE,
    RES_LIMIT,
    RES_RSVD_LIMIT,
    RES_MAX_USAGE,
    RES_RSVD_MAX_USAGE,
    RES_FAILCNT,
    RES_RSVD_FAILCNT,
    };
#[no_mangle]
unsafe extern "C" fn hugetlb_cgroup_read_numa_stat(seq: *mut seq_file, dummy: *mut c_void) -> c_int {
    let mut nid = 0;
    let mut cft = seq_cft(seq);
pub static mut idx: c_int = 0;
pub static mut legacy: bool = false;
    let mut h_cg = hugetlb_cgroup_from_css(seq_css(seq));
pub static mut css: *mut c_void = core::ptr::null_mut();
    let mut usage = 0;
    if (legacy) {
// Add up usage across all nodes for the non-hierarchical total.
    usage = 0;
    for_each_node_state(nid, N_MEMORY) {
    usage += READ_ONCE(h_cg.nodeinfo[nid].usage[idx]);
    }
    seq_printf(seq, "total=%lu", usage * PAGE_SIZE);
// Simply print the per-node usage for the non-hierarchical total.
    for_each_node_state(nid, N_MEMORY) {
    seq_printf(seq, " N%d=%lu", nid,
    READ_ONCE(h_cg.nodeinfo[nid].usage[idx]) *
    PAGE_SIZE);
    }
    seq_putc(seq, '\n');
    }
//
// The hierarchical total is pretty much the value recorded by the
// counter, so use that.
//
    seq_printf(seq, "%stotal=%lu", legacy ? "hierarchical_" : "",
    page_counter_read(&h_cg.hugepage[idx]) * PAGE_SIZE);
//
// For each node, transverse the css tree to obtain the hierarchical
// node usage.
//
    for_each_node_state(nid, N_MEMORY) {
    usage = 0;
    rcu_read_lock();
    css_for_each_descendant_pre(css, &h_cg.css) {
    usage += READ_ONCE(hugetlb_cgroup_from_css(css)
    .nodeinfo[nid]
    .usage[idx]);
    }
    rcu_read_unlock();
    seq_printf(seq, " N%d=%lu", nid, usage * PAGE_SIZE);
    }
    seq_putc(seq, '\n');
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_read_u64(css: *mut cgroup_subsys_state, cft: *mut cftype) -> u64 {
pub static mut counter: *mut c_void = core::ptr::null_mut();
pub static mut rsvd_counter: *mut c_void = core::ptr::null_mut();
    let mut h_cg = hugetlb_cgroup_from_css(css);
    counter = &h_cg.hugepage[MEMFILE_IDX(cft.private)];
    rsvd_counter = &h_cg.rsvd_hugepage[MEMFILE_IDX(cft.private)];
    switch (MEMFILE_ATTR(cft.private)) {
    case RES_USAGE:
    return (u64)page_counter_read(counter) * PAGE_SIZE;
    case RES_RSVD_USAGE:
    return (u64)page_counter_read(rsvd_counter) * PAGE_SIZE;
    case RES_LIMIT:
    return (u64)counter.max * PAGE_SIZE;
    case RES_RSVD_LIMIT:
    return (u64)rsvd_counter.max * PAGE_SIZE;
    case RES_MAX_USAGE:
    return (u64)counter.watermark * PAGE_SIZE;
    case RES_RSVD_MAX_USAGE:
    return (u64)rsvd_counter.watermark * PAGE_SIZE;
    case RES_FAILCNT:
    return counter.failcnt;
    case RES_RSVD_FAILCNT:
    return rsvd_counter.failcnt;
// label;
    BUG();
    }
    }
#[no_mangle]
unsafe extern "C" fn hugetlb_cgroup_read_u64_max(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let mut idx = 0;
    let mut val = 0;
    let mut cft = seq_cft(seq);
    let mut limit = 0;
pub static mut counter: *mut c_void = core::ptr::null_mut();
    let mut h_cg = hugetlb_cgroup_from_css(seq_css(seq));
    idx = MEMFILE_IDX(cft.private);
    counter = &h_cg.hugepage[idx];
    limit = round_down(PAGE_COUNTER_MAX,
    pages_per_huge_page(&hstates[idx]));
    switch (MEMFILE_ATTR(cft.private)) {
    case RES_RSVD_USAGE:
    counter = &h_cg.rsvd_hugepage[idx];
    fallthrough;
    case RES_USAGE:
    val = (u64)page_counter_read(counter);
    seq_printf(seq, "%llu\n", val * PAGE_SIZE);
    break;
    case RES_RSVD_LIMIT:
    counter = &h_cg.rsvd_hugepage[idx];
    fallthrough;
    case RES_LIMIT:
    val = (u64)counter.max;
    if (val == limit) {
    seq_puts(seq, "max\n");
    }
    else {
    seq_printf(seq, "%llu\n", val * PAGE_SIZE);
    }
    break;
// label;
    BUG();
    }
    return 0;
    }
pub static mut hugetlb_limit_mutex: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t, max: *mut c_char) -> ssize_t {
    let mut ret = 0;
    let mut idx = 0;
    let mut nr_pages = 0;
    let mut h_cg = hugetlb_cgroup_from_css(of_css(of));
pub static mut rsvd: bool = false;
    if (hugetlb_cgroup_is_root(h_cg)) /* Can't set limit on root */ {
    return -EINVAL;
    }
    buf = strstrip(buf);
    ret = page_counter_memparse(buf, max, &nr_pages);
    if (ret) {
    return ret;
    }
    idx = MEMFILE_IDX(of_cft(of).private);
    nr_pages = round_down(nr_pages, pages_per_huge_page(&hstates[idx]));
    switch (MEMFILE_ATTR(of_cft(of).private)) {
    case RES_RSVD_LIMIT:
    rsvd = true;
    fallthrough;
    case RES_LIMIT:
    mutex_lock(&hugetlb_limit_mutex);
    ret = page_counter_set_max(
    __hugetlb_cgroup_counter_from_cgroup(h_cg, idx, rsvd),
    nr_pages);
    mutex_unlock(&hugetlb_limit_mutex);
    break;
// label;
    ret = -EINVAL;
    break;
    }
    return ret ?: nbytes;
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_write_legacy(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t {
    return hugetlb_cgroup_write(of, buf, nbytes, off, "-1");
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_write_dfl(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t {
    return hugetlb_cgroup_write(of, buf, nbytes, off, "max");
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_reset(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t {
pub static mut ret: c_int = 0;
    let mut counter = core::ptr::null_mut();
    let mut rsvd_counter = core::ptr::null_mut();
    let mut h_cg = hugetlb_cgroup_from_css(of_css(of));
    counter = &h_cg.hugepage[MEMFILE_IDX(of_cft(of).private)];
    rsvd_counter = &h_cg.rsvd_hugepage[MEMFILE_IDX(of_cft(of).private)];
    switch (MEMFILE_ATTR(of_cft(of).private)) {
    case RES_MAX_USAGE:
    page_counter_reset_watermark(counter);
    break;
    case RES_RSVD_MAX_USAGE:
    page_counter_reset_watermark(rsvd_counter);
    break;
    case RES_FAILCNT:
    counter.failcnt = 0;
    break;
    case RES_RSVD_FAILCNT:
    rsvd_counter.failcnt = 0;
    break;
// label;
    ret = -EINVAL;
    break;
    }
    return ret ?: nbytes;
    }
#[no_mangle]
pub unsafe extern "C" fn mem_fmt(buf: *mut c_char, size: c_int, hsize: c_ulong) -> *mut c_void {
    if (hsize >= SZ_1G) {
    snprintf(buf, size, "%luGB", hsize / SZ_1G);
    }

    else if (hsize >= SZ_1M) {
    snprintf(buf, size, "%luMB", hsize / SZ_1M);
    }
    else {
    snprintf(buf, size, "%luKB", hsize / SZ_1K);
    }
    return buf;
    }
#[no_mangle]
unsafe extern "C" fn __hugetlb_events_show(seq: *mut seq_file, local: bool) -> c_int {
    let mut idx = 0;
    let mut max = 0;
    let mut cft = seq_cft(seq);
    let mut h_cg = hugetlb_cgroup_from_css(seq_css(seq));
    idx = MEMFILE_IDX(cft.private);
    if (local) {
    max = atomic_long_read(&h_cg.events_local[idx][HUGETLB_MAX]);
    }
    else {
    max = atomic_long_read(&h_cg.events[idx][HUGETLB_MAX]);
    }
    seq_printf(seq, "max %lu\n", max);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hugetlb_events_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    return __hugetlb_events_show(seq, false);
    }
#[no_mangle]
unsafe extern "C" fn hugetlb_events_local_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    return __hugetlb_events_show(seq, true);
    }
pub static mut cftype: usize = 0;
pub static mut cftype: usize = 0;
    static void __init
    hugetlb_cgroup_cfttypes_init(hstate *h, cftype *cft, cftype *tmpl, int tmpl_size)
    {
    char buf[32];
    int i, idx = hstate_index(h);
// format the size
    mem_fmt(buf, sizeof!(buf), huge_page_size(h));
    while (i < tmpl_size) {
// cft = *tmpl;
// rebuild the name
    scnprintf(cft.name, MAX_CFTYPE_NAME, "%s.%s", buf, tmpl.name);
// rebuild the private
    cft.private = MEMFILE_PRIVATE(idx, tmpl.private);
// rebuild the file_offset
    if (tmpl.file_offset) {
pub static mut offset: c_uint = 0;
    cft.file_offset = MEMFILE_OFFSET0(offset) +
    MEMFILE_FIELD_SIZE(offset) * idx;
    }
    lockdep_register_key(&cft.lockdep_key);
    }
    }
#[no_mangle]
unsafe extern "C" fn __hugetlb_cgroup_file_dfl_init(h: *mut hstate)  {
pub static mut idx: c_int = 0;
    hugetlb_cgroup_cfttypes_init(h, dfl_files + idx * DFL_TMPL_SIZE,
    hugetlb_dfl_tmpl, DFL_TMPL_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn __hugetlb_cgroup_file_legacy_init(h: *mut hstate)  {
pub static mut idx: c_int = 0;
    hugetlb_cgroup_cfttypes_init(h, legacy_files + idx * LEGACY_TMPL_SIZE,
    hugetlb_legacy_tmpl, LEGACY_TMPL_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn __hugetlb_cgroup_file_init(h: *mut hstate)  {
    __hugetlb_cgroup_file_dfl_init(h);
    __hugetlb_cgroup_file_legacy_init(h);
    }
#[no_mangle]
unsafe extern "C" fn __hugetlb_cgroup_file_pre_init()  {
    let mut cft_count = 0;
    cft_count = hugetlb_max_hstate * DFL_TMPL_SIZE + 1; /* add terminator */
    dfl_files = kzalloc_objs(cftype, cft_count);
    BUG_ON!(!dfl_files);
    cft_count = hugetlb_max_hstate * LEGACY_TMPL_SIZE + 1; /* add terminator */
    legacy_files = kzalloc_objs(cftype, cft_count);
    BUG_ON!(!legacy_files);
    }
#[no_mangle]
unsafe extern "C" fn __hugetlb_cgroup_file_post_init()  {
    WARN_ON!(cgroup_add_dfl_cftypes(&hugetlb_cgrp_subsys,
    dfl_files));
    WARN_ON!(cgroup_add_legacy_cftypes(&hugetlb_cgrp_subsys,
    legacy_files));
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_file_init()  {
pub static mut h: *mut c_void = core::ptr::null_mut();
    __hugetlb_cgroup_file_pre_init();
    for_each_hstate(h) {
    __hugetlb_cgroup_file_init(h);
    }
    __hugetlb_cgroup_file_post_init();
    }
//
// hugetlb_lock will make sure a parallel cgroup rmdir won't happen
// when we migrate hugepages
//
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_migrate(old_folio: *mut folio, new_folio: *mut folio) {
pub static mut h_cg: *mut c_void = core::ptr::null_mut();
pub static mut h_cg_rsvd: *mut c_void = core::ptr::null_mut();
    let mut h = folio_hstate(old_folio);
    if (hugetlb_cgroup_disabled()) {
    return;
    }
    spin_lock_irq(&hugetlb_lock);
    h_cg = hugetlb_cgroup_from_folio(old_folio);
    h_cg_rsvd = hugetlb_cgroup_from_folio_rsvd(old_folio);
    set_hugetlb_cgroup(old_folio, core::ptr::null_mut());
    set_hugetlb_cgroup_rsvd(old_folio, core::ptr::null_mut());
// move the h_cg details to new cgroup
    set_hugetlb_cgroup(new_folio, h_cg);
    set_hugetlb_cgroup_rsvd(new_folio, h_cg_rsvd);
    list_move(&new_folio.lru, &h.hugepage_activelist);
    spin_unlock_irq(&hugetlb_lock);
    }
pub static mut cftype: usize = 0;
pub static mut cgroup_subsys: usize = 0;