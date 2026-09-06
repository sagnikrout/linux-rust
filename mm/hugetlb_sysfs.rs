//! Automatically rewritten from C to Rust
//! Source: mm/hugetlb_sysfs.c
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
//
// HugeTLB sysfs interfaces.
// (C) Nadia Yvette Chambers, April 2004
//

    static struct kobj_attribute _name##_attr = __ATTR_RO(_name)

    static struct kobj_attribute _name##_attr = __ATTR_WO(_name)

    static struct kobj_attribute _name##_attr = __ATTR_RW(_name)
pub static mut hugepages_kobj: *mut c_void = core::ptr::null_mut();
    static struct kobject *hstate_kobjs[HUGE_MAX_HSTATE];
// forward_decl: kobj_to_node_hstate;
#[no_mangle]
pub unsafe extern "C" fn kobj_to_hstate(kobj: *mut kobject, nidp: *mut c_int) -> *mut c_void {
    let mut i = 0;
    for (i = 0; i < HUGE_MAX_HSTATE; i++) {
    if (hstate_kobjs[i] == kobj) {
    }
    if (nidp) {
// nidp = NUMA_NO_NODE;
    }
    return &hstates[i];
    }
    return kobj_to_node_hstate(kobj, nidp);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_hugepages_show_common(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
pub static mut h: *mut c_void = core::ptr::null_mut();
    let mut nr_huge_pages = 0;
    let mut nid = 0;
    h = kobj_to_hstate(kobj, &nid);
    if (nid == NUMA_NO_NODE) {
    nr_huge_pages = h.nr_huge_pages;
    }
    else {
    nr_huge_pages = h.nr_huge_pages_node[nid];
    }
    return sysfs_emit(buf, "%lu\n", nr_huge_pages);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_hugepages_store_common(obey_mempolicy: bool, kobj: *mut kobject, buf: *mut c_char, len: size_t) -> ssize_t {
pub static mut h: *mut c_void = core::ptr::null_mut();
    let mut count = 0;
    let mut nid = 0;
    let mut err = 0;
    err = kstrtoul(buf, 10, &count);
    if (err) {
    return err;
    }
    h = kobj_to_hstate(kobj, &nid);
    return __nr_hugepages_store_common(obey_mempolicy, h, nid, count, len);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_hugepages_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return nr_hugepages_show_common(kobj, attr, buf);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_hugepages_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, len: size_t) -> ssize_t {
    return nr_hugepages_store_common(false, kobj, buf, len);
    }
    HSTATE_ATTR(nr_hugepages);

//
// hstate attribute for optionally mempolicy-based constraint on persistent
// huge page alloc/free.
//
#[no_mangle]
pub unsafe extern "C" fn nr_hugepages_mempolicy_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return nr_hugepages_show_common(kobj, attr, buf);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_hugepages_mempolicy_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, len: size_t) -> ssize_t {
    return nr_hugepages_store_common(true, kobj, buf, len);
    }
    HSTATE_ATTR(nr_hugepages_mempolicy);

#[no_mangle]
pub unsafe extern "C" fn nr_overcommit_hugepages_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut h = kobj_to_hstate(kobj, core::ptr::null_mut());
    return sysfs_emit(buf, "%lu\n", h.nr_overcommit_huge_pages);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_overcommit_hugepages_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut err = 0;
    let mut input = 0;
    let mut h = kobj_to_hstate(kobj, core::ptr::null_mut());
    if (hstate_is_gigantic_no_runtime(h)) {
    return -EINVAL;
    }
    err = kstrtoul(buf, 10, &input);
    if (err) {
    return err;
    }
    spin_lock_irq(&hugetlb_lock);
    h.nr_overcommit_huge_pages = input;
    spin_unlock_irq(&hugetlb_lock);
    return count;
    }
    HSTATE_ATTR(nr_overcommit_hugepages);
#[no_mangle]
pub unsafe extern "C" fn free_hugepages_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
pub static mut h: *mut c_void = core::ptr::null_mut();
    let mut free_huge_pages = 0;
    let mut nid = 0;
    h = kobj_to_hstate(kobj, &nid);
    if (nid == NUMA_NO_NODE) {
    free_huge_pages = h.free_huge_pages;
    }
    else {
    free_huge_pages = h.free_huge_pages_node[nid];
    }
    return sysfs_emit(buf, "%lu\n", free_huge_pages);
    }
    HSTATE_ATTR_RO(free_hugepages);
#[no_mangle]
pub unsafe extern "C" fn resv_hugepages_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut h = kobj_to_hstate(kobj, core::ptr::null_mut());
    return sysfs_emit(buf, "%lu\n", h.resv_huge_pages);
    }
    HSTATE_ATTR_RO(resv_hugepages);
#[no_mangle]
pub unsafe extern "C" fn surplus_hugepages_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
pub static mut h: *mut c_void = core::ptr::null_mut();
    let mut surplus_huge_pages = 0;
    let mut nid = 0;
    h = kobj_to_hstate(kobj, &nid);
    if (nid == NUMA_NO_NODE) {
    surplus_huge_pages = h.surplus_huge_pages;
    }
    else {
    surplus_huge_pages = h.surplus_huge_pages_node[nid];
    }
    return sysfs_emit(buf, "%lu\n", surplus_huge_pages);
    }
    HSTATE_ATTR_RO(surplus_hugepages);
#[no_mangle]
pub unsafe extern "C" fn demote_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, len: size_t) -> ssize_t {
    let mut nr_demote = 0;
    let mut nr_available = 0;
    nodemask_t nodes_allowed, *n_mask;
pub static mut h: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    let mut nid = 0;
    err = kstrtoul(buf, 10, &nr_demote);
    if (err) {
    return err;
    }
    h = kobj_to_hstate(kobj, &nid);
    if (nid != NUMA_NO_NODE) {
    init_nodemask_of_node(&nodes_allowed, nid);
    n_mask = &nodes_allowed;
    } else {
    n_mask = &node_states[N_MEMORY];
    }
// Synchronize with other sysfs operations modifying huge pages
    mutex_lock(&h.resize_lock);
    spin_lock_irq(&hugetlb_lock);
    while (nr_demote) {
    let mut rc = 0;
//
// Check for available pages to demote each time thorough the
// loop as demote_pool_huge_page will drop hugetlb_lock.
//
    if (nid != NUMA_NO_NODE) {
    nr_available = h.free_huge_pages_node[nid];
    }
    else {
    nr_available = h.free_huge_pages;
    }
    nr_available -= h.resv_huge_pages;
    if (!nr_available) {
    break;
    }
    rc = demote_pool_huge_page(h, n_mask, nr_demote);
    if (rc < 0) {
    err = rc;
    break;
    }
    nr_demote -= rc;
    }
    spin_unlock_irq(&hugetlb_lock);
    mutex_unlock(&h.resize_lock);
    if (err) {
    return err;
    }
    return len;
    }
    HSTATE_ATTR_WO(demote);
#[no_mangle]
pub unsafe extern "C" fn demote_size_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut h = kobj_to_hstate(kobj, core::ptr::null_mut());
pub static mut demote_size: c_ulong = 0;
    return sysfs_emit(buf, "%lukB\n", demote_size);
    }
#[no_mangle]
pub unsafe extern "C" fn demote_size_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut h = core::ptr::null_mut();
    let mut demote_hstate = core::ptr::null_mut();
    let mut demote_size = 0;
    let mut demote_order = 0;
    demote_size = (unsigned long)memparse(buf, core::ptr::null_mut());
    demote_hstate = size_to_hstate(demote_size);
    if (!demote_hstate) {
    return -EINVAL;
    }
    demote_order = demote_hstate.order;
    if (demote_order < HUGETLB_PAGE_ORDER) {
    return -EINVAL;
    }
// demote order must be smaller than hstate order
    h = kobj_to_hstate(kobj, core::ptr::null_mut());
    if (demote_order >= h.order) {
    return -EINVAL;
    }
// resize_lock synchronizes access to demote size and writes
    mutex_lock(&h.resize_lock);
    h.demote_order = demote_order;
    mutex_unlock(&h.resize_lock);
    return count;
    }
    HSTATE_ATTR(demote_size);
    static struct attribute *hstate_attrs[] = {
    &nr_hugepages_attr.attr,
    &nr_overcommit_hugepages_attr.attr,
    &free_hugepages_attr.attr,
    &resv_hugepages_attr.attr,
    &surplus_hugepages_attr.attr,

    &nr_hugepages_mempolicy_attr.attr,

    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
    static struct attribute *hstate_demote_attrs[] = {
    &demote_size_attr.attr,
    &demote_attr.attr,
    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn hugetlb_sysfs_add_hstate(h: *mut hstate, parent: *mut kobject, hstate_kobjs: *mut *mut kobject, hstate_attr_group: *mut attribute_group) -> c_int {
    let mut retval = 0;
pub static mut hi: c_int = 0;
    hstate_kobjs[hi] = kobject_create_and_add(h.name, parent);
    if (!hstate_kobjs[hi]) {
    return -ENOMEM;
    }
    retval = sysfs_create_group(hstate_kobjs[hi], hstate_attr_group);
    if (retval) {
    kobject_put(hstate_kobjs[hi]);
    hstate_kobjs[hi] = core::ptr::null_mut();
    return retval;
    }
    if (h.demote_order) {
    retval = sysfs_create_group(hstate_kobjs[hi],
    &hstate_demote_attr_group);
    if (retval) {
    pr_warn!("HugeTLB unable to create demote interfaces for %s\n", h.name);
    sysfs_remove_group(hstate_kobjs[hi], hstate_attr_group);
    kobject_put(hstate_kobjs[hi]);
    hstate_kobjs[hi] = core::ptr::null_mut();
    return retval;
    }
    }
    return 0;
    }

    static bool hugetlb_sysfs_initialized __ro_after_init;
//
// node_hstate/s - associate per node hstate attributes, via their kobjects,
// with node devices in node_devices[] using a parallel array.  The array
// index of a node device or _hstate == node id.
// This is here to avoid any static dependency of the node device driver, in
// the base kernel, on the hugetlb module.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_hstate {
    pub hugepages_kobj: *mut kobject,
    pub hstate_kobjs: [*mut kobject; HUGE_MAX_HSTATE],
}

    static struct node_hstate node_hstates[MAX_NUMNODES];
//
// A subset of global hstate attributes for node devices
//
    static struct attribute *per_node_hstate_attrs[] = {
    &nr_hugepages_attr.attr,
    &free_hugepages_attr.attr,
    &surplus_hugepages_attr.attr,
    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
//
// kobj_to_node_hstate - lookup global hstate for node device hstate attr kobj.
// Returns node id via non-NULL nidp.
//
#[no_mangle]
pub unsafe extern "C" fn kobj_to_node_hstate(kobj: *mut kobject, nidp: *mut c_int) -> *mut c_void {
    let mut nid = 0;
    while (nid < nr_node_ids) {
    let mut nhs = &node_hstates[nid];
    let mut i = 0;
    for (i = 0; i < HUGE_MAX_HSTATE; i++) {
    if (nhs.hstate_kobjs[i] == kobj) {
    }
    if (nidp) {
// nidp = nid;
    }
    return &hstates[i];
    }
    }
    BUG();
    return core::ptr::null_mut();
    }
//
// Unregister hstate attributes from a single node device.
// No-op if no hstate attributes attached.
//
#[no_mangle]
pub unsafe extern "C" fn hugetlb_unregister_node(node: *mut node) {
pub static mut h: *mut c_void = core::ptr::null_mut();
    let mut nhs = &node_hstates[node.dev.id];
    if (!nhs.hugepages_kobj) {
    return;		/* no hstate attributes */
    }
    for_each_hstate(h) {
pub static mut idx: c_int = 0;
    let mut hstate_kobj = nhs.hstate_kobjs[idx];
    if (!hstate_kobj) {
    continue;
    }
    if (h.demote_order) {
    sysfs_remove_group(hstate_kobj, &hstate_demote_attr_group);
    }
    sysfs_remove_group(hstate_kobj, &per_node_hstate_attr_group);
    kobject_put(hstate_kobj);
    nhs.hstate_kobjs[idx] = core::ptr::null_mut();
    }
    kobject_put(nhs.hugepages_kobj);
    nhs.hugepages_kobj = core::ptr::null_mut();
    }
//
// Register hstate attributes for a single node device.
// No-op if attributes already registered.
//
#[no_mangle]
pub unsafe extern "C" fn hugetlb_register_node(node: *mut node) {
pub static mut h: *mut c_void = core::ptr::null_mut();
    let mut nhs = &node_hstates[node.dev.id];
    let mut err = 0;
    if (!hugetlb_sysfs_initialized) {
    return;
    }
    if (nhs.hugepages_kobj) {
    return;		/* already allocated */
    }
    nhs.hugepages_kobj = kobject_create_and_add("hugepages",
    &node.dev.kobj);
    if (!nhs.hugepages_kobj) {
    return;
    }
    for_each_hstate(h) {
    err = hugetlb_sysfs_add_hstate(h, nhs.hugepages_kobj,
    nhs.hstate_kobjs,
    &per_node_hstate_attr_group);
    if (err) {
    pr_err!("HugeTLB: Unable to add hstate %s for node %d\n",
    h.name, node.dev.id);
    hugetlb_unregister_node(node);
    break;
    }
    }
    }
//
// hugetlb init time:  register hstate attributes for all registered node
// devices of nodes that have memory.  All on-line nodes should have
// registered their associated device by this time.
//
#[no_mangle]
unsafe extern "C" fn hugetlb_register_all_nodes()  {
    let mut nid = 0;
    for_each_online_node(nid) {
    hugetlb_register_node(node_devices[nid]);
    }
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: kobj_to_node_hstate
pub unsafe extern "C" fn kobj_to_node_hstate_dup(kobj: *mut kobject, nidp: *mut c_int) -> *mut c_void {
    BUG();
    if (nidp) {
// nidp = -1;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_register_all_nodes() { }

#[no_mangle]
pub unsafe extern "C" fn hugetlb_sysfs_init()  {
pub static mut h: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    hugepages_kobj = kobject_create_and_add("hugepages", mm_kobj);
    if (!hugepages_kobj) {
    return;
    }
    for_each_hstate(h) {
    err = hugetlb_sysfs_add_hstate(h, hugepages_kobj,
    hstate_kobjs, &hstate_attr_group);
    if (err) {
    pr_err!("HugeTLB: Unable to add hstate %s\n", h.name);
    }
    }

    hugetlb_sysfs_initialized = true;

    hugetlb_register_all_nodes();
    }