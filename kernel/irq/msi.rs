//! Automatically rewritten from C to Rust
//! Source: kernel/irq/msi.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2014 Intel Corp.
// Author: Jiang Liu <jiang.liu@linux.intel.com>
//
// This file is licensed under GPLv2.
//
// This file contains common code to support Message Signaled Interrupts for
// PCI compatible and non PCI compatible devices.
//

//
// struct msi_device_data - MSI per device data
// @properties:		MSI properties which are interesting to drivers
// @mutex:		Mutex protecting the MSI descriptor store
// @__domains:		Internal data for per device MSI domains
// @__iter_idx:		Index to search the next entry for iterators
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_device_data {
    pub properties: c_ulong,
    pub mutex: mutex,
    pub __domains: [msi_dev_domain; MSI_MAX_DEVICE_IRQDOMAINS],
    pub __iter_idx: c_ulong,
}

//
// struct msi_ctrl - MSI internal management control structure
// @domid:	ID of the domain on which management operations should be done
// @first:	First (hardware) slot index to operate on
// @last:	Last (hardware) slot index to operate on
// @nirqs:	The number of Linux interrupts to allocate. Can be larger
// than the range due to PCI/multi-MSI.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msi_ctrl {
    pub domid: c_uint,
    pub first: c_uint,
    pub last: c_uint,
    pub nirqs: c_uint,
}

// Invalid Xarray index which is outside of any searchable range

// The maximum domain size

// forward_decl: msi_domain_free_locked;
// forward_decl: msi_domain_get_hwsize;
// forward_decl: msi_sysfs_create_group;
// forward_decl: msi_domain_prepare_irqs;
//
// msi_alloc_desc - Allocate an initialized msi_desc
// @dev:	Pointer to the device for which this is allocated
// @nvec:	The number of vectors used in this entry
// @affinity:	Optional pointer to an affinity mask array size of @nvec
//
// If @affinity is not %NULL then an affinity array[@nvec] is allocated
// and the affinity masks and flags from @affinity are copied.
//
// Return: pointer to allocated &msi_desc on success or %NULL on failure
//
#[no_mangle]
pub unsafe extern "C" fn msi_alloc_desc(dev: *mut device, nvec: c_int, affinity: *mut irq_affinity_desc) -> *mut c_void {
    let mut desc = kzalloc_obj(*desc);
    if (!desc) {
    return core::ptr::null_mut();
    }
    desc.dev = dev;
    desc.nvec_used = nvec;
    if (affinity) {
    desc.affinity = kmemdup_array(affinity, nvec, sizeof!(*desc.affinity), GFP_KERNEL);
    if (!desc.affinity) {
    kfree(desc);
    return core::ptr::null_mut();
    }
    }
    return desc;
    }
#[no_mangle]
unsafe extern "C" fn msi_free_desc(desc: *mut msi_desc) {
    kfree(desc.affinity);
    kfree(desc);
    }
#[no_mangle]
pub unsafe extern "C" fn msi_insert_desc(dev: *mut device, desc: *mut msi_desc, domid: c_uint, index: c_uint) -> c_int {
    let mut md = dev.msi.data;
    let mut xa = &md.__domains[domid].store;
    let mut hwsize = 0;
    let mut ret = 0;
    hwsize = msi_domain_get_hwsize(dev, domid);
    if (index == MSI_ANY_INDEX) {
pub static mut limit: xa_limit = 0;
    let mut index = 0;
// Let the xarray allocate a free index within the limit
    ret = xa_alloc(xa, &index, desc, limit, GFP_KERNEL);
    if (ret) {
// goto;
    }
    desc.msi_index = index;
    return 0;
    } else {
    if (index >= hwsize) {
    ret = -ERANGE;
// goto;
    }
    desc.msi_index = index;
    ret = xa_insert(xa, index, desc, GFP_KERNEL);
    if (ret) {
// goto;
    }
    return 0;
    }
// label;
    msi_free_desc(desc);
    return ret;
    }
//
// msi_domain_insert_msi_desc - Allocate and initialize a MSI descriptor and
// insert it at @init_desc->msi_index
//
// @dev:	Pointer to the device for which the descriptor is allocated
// @domid:	The id of the interrupt domain to which the desriptor is added
// @init_desc:	Pointer to an MSI descriptor to initialize the new descriptor
//
// Return: 0 on success or an appropriate failure code.
//
#[no_mangle]
pub unsafe extern "C" fn msi_domain_insert_msi_desc(dev: *mut device, domid: c_uint, init_desc: *mut msi_desc) -> c_int {
pub static mut desc: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&dev.msi.data.mutex);
    desc = msi_alloc_desc(dev, init_desc.nvec_used, init_desc.affinity);
    if (!desc) {
    return -ENOMEM;
    }
// Copy type specific data to the new descriptor.
    desc.pci = init_desc.pci;
    return msi_insert_desc(dev, desc, domid, init_desc.msi_index);
    }
#[no_mangle]
unsafe extern "C" fn msi_desc_match(desc: *mut msi_desc, filter: msi_desc_filter) -> bool {
    match (filter) {
    MSI_DESC_ALL => {
    return true;
    }
    MSI_DESC_NOTASSOCIATED => {
    return !desc.irq;
    }
    MSI_DESC_ASSOCIATED => {
    return !!desc.irq;
    }
    }
    WARN_ON_ONCE!(1);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn msi_ctrl_valid(dev: *mut device, ctrl: *mut msi_ctrl) -> bool {
    let mut hwsize = 0;
    if (WARN_ON_ONCE!(ctrl.domid >= MSI_MAX_DEVICE_IRQDOMAINS ||
    (dev.msi.domain &&
    !dev.msi.data.__domains[ctrl.domid].domain))) {
    return false;
    }
    hwsize = msi_domain_get_hwsize(dev, ctrl.domid);
    if (WARN_ON_ONCE!(ctrl.first > ctrl.last ||
    ctrl.first >= hwsize ||
    ctrl.last >= hwsize)) {
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn msi_domain_free_descs(dev: *mut device, ctrl: *mut msi_ctrl) {
pub static mut desc: *mut c_void = core::ptr::null_mut();
pub static mut xa: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    lockdep_assert_held(&dev.msi.data.mutex);
    if (!msi_ctrl_valid(dev, ctrl)) {
    return;
    }
    xa = &dev.msi.data.__domains[ctrl.domid].store;
    xa_for_each_range(xa, idx, desc, ctrl.first, ctrl.last) {
    xa_erase(xa, idx);
// Leak the descriptor when it is still referenced
    if (WARN_ON_ONCE!(msi_desc_match(desc, MSI_DESC_ASSOCIATED))) {
    continue;
    }
    msi_free_desc(desc);
    }
    }
//
// msi_domain_free_msi_descs_range - Free a range of MSI descriptors of a device in an irqdomain
// @dev:	Device for which to free the descriptors
// @domid:	Id of the domain to operate on
// @first:	Index to start freeing from (inclusive)
// @last:	Last index to be freed (inclusive)
//
#[no_mangle]
pub unsafe extern "C" fn msi_domain_free_msi_descs_range(dev: *mut device, domid: c_uint, first: c_uint, last: c_uint) {
pub static mut msi_ctrl: usize = 0;
    msi_domain_free_descs(dev, &ctrl);
    }
//
// msi_domain_add_simple_msi_descs - Allocate and initialize MSI descriptors
// @dev:	Pointer to the device for which the descriptors are allocated
// @ctrl:	Allocation control struct
//
// Return: 0 on success or an appropriate failure code.
//
#[no_mangle]
unsafe extern "C" fn msi_domain_add_simple_msi_descs(dev: *mut device, ctrl: *mut msi_ctrl) -> c_int {
pub static mut desc: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    let mut ret = 0;
    lockdep_assert_held(&dev.msi.data.mutex);
    if (!msi_ctrl_valid(dev, ctrl)) {
    return -EINVAL;
    }
    while (idx <= ctrl.last) {
    desc = msi_alloc_desc(dev, 1, core::ptr::null_mut());
    if (!desc) {
// goto;
    }
    ret = msi_insert_desc(dev, desc, ctrl.domid, idx);
    if (ret) {
// goto;
    }
    }
    return 0;
// label;
    ret = -ENOMEM;
// label;
    msi_domain_free_descs(dev, ctrl);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __get_cached_msi_msg(entry: *mut msi_desc, msg: *mut msi_msg) {
// msg = entry->msg;
    }
#[no_mangle]
pub unsafe extern "C" fn get_cached_msi_msg(irq: c_uint, msg: *mut msi_msg) {
    let mut entry = irq_get_msi_desc(irq);
    __get_cached_msi_msg(entry, msg);
    }
    EXPORT_SYMBOL_GPL(get_cached_msi_msg);
#[no_mangle]
unsafe extern "C" fn msi_device_data_release(dev: *mut device, res: *mut c_void) {
    let mut md = res;
    let mut i = 0;
    while (i < MSI_MAX_DEVICE_IRQDOMAINS) {
    msi_remove_device_irq_domain(dev, i);
    WARN_ON_ONCE!(!xa_empty(&md.__domains[i].store));
    xa_destroy(&md.__domains[i].store);
    }
    dev.msi.data = core::ptr::null_mut();
    }
//
// msi_setup_device_data - Setup MSI device data
// @dev:	Device for which MSI device data should be set up
//
// Return: 0 on success, appropriate error code otherwise
//
// This can be called more than once for @dev. If the MSI device data is
// already allocated the call succeeds. The allocated memory is
// automatically released when the device is destroyed.
//
#[no_mangle]
pub unsafe extern "C" fn msi_setup_device_data(dev: *mut device) -> c_int {
pub static mut md: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut i = 0;
    if (dev.msi.data) {
    return 0;
    }
    md = devres_alloc(msi_device_data_release, sizeof!(*md), GFP_KERNEL);
    if (!md) {
    return -ENOMEM;
    }
    ret = msi_sysfs_create_group(dev);
    if (ret) {
    devres_free(md);
    return ret;
    }
    for (i = 0; i < MSI_MAX_DEVICE_IRQDOMAINS; i++) {
    xa_init_flags(&md.__domains[i].store, XA_FLAGS_ALLOC);
    }
//
// If @dev::msi::domain is set and is a global MSI domain, copy the
// pointer into the domain array so all code can operate on domain
// ids. The NULL pointer check is required to keep the legacy
// architecture specific PCI/MSI support working.
//
    if (dev.msi.domain && !irq_domain_is_msi_parent(dev.msi.domain)) {
    md.__domains[MSI_DEFAULT_DOMAIN].domain = dev.msi.domain;
    }
    mutex_init(&md.mutex);
    dev.msi.data = md;
    devres_add(dev, md);
    return 0;
    }
//
// __msi_lock_descs - Lock the MSI descriptor storage of a device
// @dev:	Device to operate on
//
// Internal function for guard(msi_descs_lock). Don't use in code.
//
#[no_mangle]
pub unsafe extern "C" fn __msi_lock_descs(dev: *mut device) {
    mutex_lock(&dev.msi.data.mutex);
    }
    EXPORT_SYMBOL_GPL(__msi_lock_descs);
//
// __msi_unlock_descs - Unlock the MSI descriptor storage of a device
// @dev:	Device to operate on
//
// Internal function for guard(msi_descs_lock). Don't use in code.
//
#[no_mangle]
pub unsafe extern "C" fn __msi_unlock_descs(dev: *mut device) {
// Invalidate the index which was cached by the iterator
    dev.msi.data.__iter_idx = MSI_XA_MAX_INDEX;
    mutex_unlock(&dev.msi.data.mutex);
    }
    EXPORT_SYMBOL_GPL(__msi_unlock_descs);
#[no_mangle]
pub unsafe extern "C" fn msi_find_desc(md: *mut msi_device_data, domid: c_uint, filter: msi_desc_filter) -> *mut c_void {
    let mut xa = &md.__domains[domid].store;
pub static mut desc: *mut c_void = core::ptr::null_mut();
    xa_for_each_start(xa, md.__iter_idx, desc, md.__iter_idx) {
    if (msi_desc_match(desc, filter)) {
    return desc;
    }
    }
    md.__iter_idx = MSI_XA_MAX_INDEX;
    return core::ptr::null_mut();
    }
//
// msi_domain_first_desc - Get the first MSI descriptor of an irqdomain associated to a device
// @dev:	Device to operate on
// @domid:	The id of the interrupt domain which should be walked.
// @filter:	Descriptor state filter
//
// Must be called with the MSI descriptor mutex held, i.e. msi_lock_descs()
// must be invoked before the call.
//
// Return: Pointer to the first MSI descriptor matching the search
// criteria, NULL if none found.
//
#[no_mangle]
pub unsafe extern "C" fn msi_domain_first_desc(dev: *mut device, domid: c_uint, filter: msi_desc_filter) -> *mut c_void {
    let mut md = dev.msi.data;
    if (WARN_ON_ONCE!(!md || domid >= MSI_MAX_DEVICE_IRQDOMAINS)) {
    return core::ptr::null_mut();
    }
    lockdep_assert_held(&md.mutex);
    md.__iter_idx = 0;
    return msi_find_desc(md, domid, filter);
    }
    EXPORT_SYMBOL_GPL(msi_domain_first_desc);
//
// msi_next_desc - Get the next MSI descriptor of a device
// @dev:	Device to operate on
// @domid:	The id of the interrupt domain which should be walked.
// @filter:	Descriptor state filter
//
// The first invocation of msi_next_desc() has to be preceeded by a
// successful invocation of __msi_first_desc(). Consecutive invocations are
// only valid if the previous one was successful. All these operations have
// to be done within the same MSI mutex held region.
//
// Return: Pointer to the next MSI descriptor matching the search
// criteria, NULL if none found.
//
#[no_mangle]
pub unsafe extern "C" fn msi_next_desc(dev: *mut device, domid: c_uint, filter: msi_desc_filter) -> *mut c_void {
    let mut md = dev.msi.data;
    if (WARN_ON_ONCE!(!md || domid >= MSI_MAX_DEVICE_IRQDOMAINS)) {
    return core::ptr::null_mut();
    }
    lockdep_assert_held(&md.mutex);
    if (md.__iter_idx >= (unsigned long)MSI_MAX_INDEX) {
    return core::ptr::null_mut();
    }
    md.__iter_idx += 1;
    return msi_find_desc(md, domid, filter);
    }
    EXPORT_SYMBOL_GPL(msi_next_desc);
//
// msi_domain_get_virq - Lookup the Linux interrupt number for a MSI index on a interrupt domain
// @dev:	Device to operate on
// @domid:	Domain ID of the interrupt domain associated to the device
// @index:	MSI interrupt index to look for (0-based)
//
// Return: The Linux interrupt number on success (> 0), 0 if not found
//
#[no_mangle]
pub unsafe extern "C" fn msi_domain_get_virq(dev: *mut device, domid: c_uint, index: c_uint) -> c_uint {
pub static mut desc: *mut c_void = core::ptr::null_mut();
pub static mut pcimsi: bool = false;
pub static mut xa: *mut c_void = core::ptr::null_mut();
    if (!dev.msi.data) {
    return 0;
    }
    if (WARN_ON_ONCE!(index > MSI_MAX_INDEX || domid >= MSI_MAX_DEVICE_IRQDOMAINS)) {
    return 0;
    }
// This check is only valid for the PCI default MSI domain
    if (dev_is_pci(dev) && domid == MSI_DEFAULT_DOMAIN) {
    pcimsi = to_pci_dev(dev).msi_enabled;
    }
    guard(msi_descs_lock)(dev);
    xa = &dev.msi.data.__domains[domid].store;
    desc = xa_load(xa, pcimsi ? 0 : index);
    if (desc && desc.irq) {
//
// PCI-MSI has only one descriptor for multiple interrupts.
// PCI-MSIX and platform MSI use a descriptor per
// interrupt.
//
    if (!pcimsi) {
    return desc.irq;
    }
    if (index < desc.nvec_used) {
    return desc.irq + index;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(msi_domain_get_virq);

    static struct attribute *msi_dev_attrs[] = {
    core::ptr::null_mut()
    };
pub static mut attribute_group: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn msi_sysfs_create_group(dev: *mut device) -> c_int {
    return devm_device_add_group(dev, &msi_irqs_group);
    }
#[no_mangle]
pub unsafe extern "C" fn msi_mode_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
// MSI vs. MSIX is per device not per interrupt
pub static mut is_msix: bool = false;
    return sysfs_emit(buf, "%s\n", is_msix ? "msix" : "msi");
    }
#[no_mangle]
unsafe extern "C" fn msi_sysfs_remove_desc(dev: *mut device, desc: *mut msi_desc) {
    let mut attrs = desc.sysfs_attrs;
    let mut i = 0;
    if (!attrs) {
    return;
    }
    desc.sysfs_attrs = core::ptr::null_mut();
    while (i < desc.nvec_used) {
    if (attrs[i].show) {
    sysfs_remove_file_from_group(&dev.kobj, &attrs[i].attr, msi_irqs_group.name);
    }
    kfree(attrs[i].attr.name);
    }
    kfree(attrs);
    }
#[no_mangle]
unsafe extern "C" fn msi_sysfs_populate_desc(dev: *mut device, desc: *mut msi_desc) -> c_int {
pub static mut attrs: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut i = 0;
    attrs = kzalloc_objs(*attrs, desc.nvec_used);
    if (!attrs) {
    return -ENOMEM;
    }
    desc.sysfs_attrs = attrs;
    while (i < desc.nvec_used) {
    sysfs_attr_init(&attrs[i].attr);
    attrs[i].attr.name = kasprintf(GFP_KERNEL, "%d", desc.irq + i);
    if (!attrs[i].attr.name) {
    ret = -ENOMEM;
// goto;
    }
    attrs[i].attr.mode = 0444;
    attrs[i].show = msi_mode_show;
    ret = sysfs_add_file_to_group(&dev.kobj, &attrs[i].attr, msi_irqs_group.name);
    if (ret) {
    attrs[i].show = core::ptr::null_mut();
// goto;
    }
    }
    return 0;
// label;
    msi_sysfs_remove_desc(dev, desc);
    return ret;
    }

//
// msi_device_populate_sysfs - Populate msi_irqs sysfs entries for a device
// @dev:	The device (PCI, platform etc) which will get sysfs entries
//
#[no_mangle]
pub unsafe extern "C" fn msi_device_populate_sysfs(dev: *mut device) -> c_int {
pub static mut desc: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    msi_for_each_desc(desc, dev, MSI_DESC_ASSOCIATED) {
    if (desc.sysfs_attrs) {
    continue;
    }
    ret = msi_sysfs_populate_desc(dev, desc);
    if (ret) {
    return ret;
    }
    }
    return 0;
    }
//
// msi_device_destroy_sysfs - Destroy msi_irqs sysfs entries for a device
// @dev:		The device (PCI, platform etc) for which to remove
// sysfs entries
//
#[no_mangle]
pub unsafe extern "C" fn msi_device_destroy_sysfs(dev: *mut device) {
pub static mut desc: *mut c_void = core::ptr::null_mut();
    msi_for_each_desc(desc, dev, MSI_DESC_ALL)
    msi_sysfs_remove_desc(dev, desc);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: msi_sysfs_create_group
pub unsafe extern "C" fn msi_sysfs_create_group_dup(dev: *mut device) -> c_int { return 0; }
#[no_mangle]
pub unsafe extern "C" fn msi_sysfs_populate_desc(dev: *mut device, desc: *mut msi_desc) -> c_int { return 0; }
#[no_mangle]
pub unsafe extern "C" fn msi_sysfs_remove_desc(dev: *mut device, desc: *mut msi_desc) { }

#[no_mangle]
pub unsafe extern "C" fn msi_get_device_domain(dev: *mut device, domid: c_uint) -> *mut c_void {
pub static mut domain: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&dev.msi.data.mutex);
    if (WARN_ON_ONCE!(domid >= MSI_MAX_DEVICE_IRQDOMAINS)) {
    return core::ptr::null_mut();
    }
    domain = dev.msi.data.__domains[domid].domain;
    if (!domain) {
    return core::ptr::null_mut();
    }
    if (WARN_ON_ONCE!(irq_domain_is_msi_parent(domain))) {
    return core::ptr::null_mut();
    }
    return domain;
    }
#[no_mangle]
unsafe extern "C" fn msi_domain_get_hwsize(dev: *mut device, domid: c_uint) -> c_uint {
pub static mut info: *mut c_void = core::ptr::null_mut();
pub static mut domain: *mut c_void = core::ptr::null_mut();
    domain = msi_get_device_domain(dev, domid);
    if (domain) {
    info = domain.host_data;
    return info.hwsize;
    }
// No domain, default to MSI_XA_DOMAIN_SIZE
    return MSI_XA_DOMAIN_SIZE;
    }
#[no_mangle]
pub unsafe extern "C" fn irq_chip_write_msi_msg(data: *mut irq_data, msg: *mut msi_msg) {
    data.chip.irq_write_msi_msg(data, msg);
    }
#[no_mangle]
unsafe extern "C" fn msi_check_level(domain: *mut irq_domain, msg: *mut msi_msg) {
    let mut info = domain.host_data;
//
// If the MSI provider has messed with the second message and
// not advertized that it is level-capable, signal the breakage.
//
    WARN_ON!(!((info.flags & MSI_FLAG_LEVEL_CAPABLE) &&
    (info.chip.flags & IRQCHIP_SUPPORTS_LEVEL_MSI)) &&
    (msg[1].address_lo || msg[1].address_hi || msg[1].data));
    }
//
// msi_domain_set_affinity - Generic affinity setter function for MSI domains
// @irq_data:	The irq data associated to the interrupt
// @mask:	The affinity mask to set
// @force:	Flag to enforce setting (disable online checks)
//
// Intended to be used by MSI interrupt controllers which are
// implemented with hierarchical domains.
//
// Return: IRQ_SET_MASK_* result code
//
#[no_mangle]
pub unsafe extern "C" fn msi_domain_set_affinity(irq_data: *mut irq_data, mask: *mut cpumask, force: bool) -> c_int {
    let mut parent = irq_data.parent_data;
pub static mut msi_msg: usize = 0;
    let mut ret = 0;
    ret = parent.chip.irq_set_affinity(parent, mask, force);
    if (ret >= 0 && ret != IRQ_SET_MASK_OK_DONE) {
    BUG_ON!(irq_chip_compose_msi_msg(irq_data, msg));
    msi_check_level(irq_data.domain, msg);
    irq_chip_write_msi_msg(irq_data, msg);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn msi_domain_activate(domain: *mut irq_domain, irq_data: *mut irq_data, early: bool) -> c_int {
pub static mut msi_msg: usize = 0;
    BUG_ON!(irq_chip_compose_msi_msg(irq_data, msg));
    msi_check_level(irq_data.domain, msg);
    irq_chip_write_msi_msg(irq_data, msg);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn msi_domain_deactivate(domain: *mut irq_domain, irq_data: *mut irq_data) {
    struct msi_msg msg[2];
    memset(msg, 0, sizeof!(msg));
    irq_chip_write_msi_msg(irq_data, msg);
    }
#[no_mangle]
pub unsafe extern "C" fn msi_domain_alloc(domain: *mut irq_domain, virq: c_uint, nr_irqs: c_uint, arg: *mut c_void) -> c_int {
    let mut info = domain.host_data;
    let mut ops = info.ops;
pub static mut hwirq: irq_hw_number_t = 0;
    let mut i = 0;
    let mut ret = 0;
    if (irq_resolve_mapping(domain, hwirq)) {
    return -EEXIST;
    }
    if (domain.parent) {
    ret = irq_domain_alloc_irqs_parent(domain, virq, nr_irqs, arg);
    if (ret < 0) {
    return ret;
    }
    }
    while (i < nr_irqs) {
    ret = ops.msi_init(domain, info, virq + i, hwirq + i, arg);
    if (ret < 0) {
    if (ops.msi_free) {
    for (i -= 1; i >= 0; i--) {
    ops.msi_free(domain, info, virq + i);
    }
    }
    irq_domain_free_irqs_top(domain, virq, nr_irqs);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn msi_domain_free(domain: *mut irq_domain, virq: c_uint, nr_irqs: c_uint) {
    let mut info = domain.host_data;
    let mut i = 0;
    if (info.ops.msi_free) {
    for (i = 0; i < nr_irqs; i++) {
    info.ops.msi_free(domain, info, virq + i);
    }
    }
    irq_domain_free_irqs_top(domain, virq, nr_irqs);
    }
#[no_mangle]
pub unsafe extern "C" fn msi_domain_translate(domain: *mut irq_domain, fwspec: *mut irq_fwspec, hwirq: *mut irq_hw_number_t, type: *mut c_uint) -> c_int {
    let mut info = domain.host_data;
//
// This will catch allocations through the regular irqdomain path except
// for MSI domains which really support this, e.g. MBIGEN.
//
    if (!info.ops.msi_translate) {
    return -ENOTSUPP;
    }
    return info.ops.msi_translate(domain, fwspec, hwirq, type);
    }

#[no_mangle]
pub unsafe extern "C" fn msi_domain_debug_show(m: *mut seq_file, d: *mut irq_domain, irqd: *mut irq_data, ind: c_int) {
    let mut desc = irqd ? irq_data_get_msi_desc(irqd) : core::ptr::null_mut();
    if (!desc) {
    return;
    }
    seq_printf(m, "\n%*saddress_hi: 0x%08x", ind + 1, "", desc.msg.address_hi);
    seq_printf(m, "\n%*saddress_lo: 0x%08x", ind + 1, "", desc.msg.address_lo);
    seq_printf(m, "\n%*smsg_data:   0x%08x\n", ind + 1, "", desc.msg.data);
    }

pub static mut irq_domain_ops: usize = 0;
    static irq_hw_number_t msi_domain_ops_get_hwirq(msi_domain_info *info,
    msi_alloc_info_t *arg)
    {
    return arg.hwirq;
    }
#[no_mangle]
pub unsafe extern "C" fn msi_domain_ops_prepare(domain: *mut irq_domain, dev: *mut device, nvec: c_int, arg: *mut msi_alloc_info_t) -> c_int {
    memset(arg, 0, sizeof!(*arg));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msi_domain_ops_teardown(domain: *mut irq_domain, arg: *mut msi_alloc_info_t) {
    }
#[no_mangle]
pub unsafe extern "C" fn msi_domain_ops_set_desc(arg: *mut msi_alloc_info_t, desc: *mut msi_desc) {
    arg.desc = desc;
    }
#[no_mangle]
pub unsafe extern "C" fn msi_domain_ops_init(domain: *mut irq_domain, info: *mut msi_domain_info, virq: c_uint, hwirq: irq_hw_number_t, arg: *mut msi_alloc_info_t) -> c_int {
    irq_domain_set_hwirq_and_chip(domain, virq, hwirq, info.chip,
    info.chip_data);
    if (info.handler && info.handler_name) {
    __irq_set_handler(virq, info.handler, 0, info.handler_name);
    if (info.handler_data) {
    irq_set_handler_data(virq, info.handler_data);
    }
    }
    return 0;
    }
pub static mut msi_domain_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn msi_domain_update_dom_ops(info: *mut msi_domain_info) {
    let mut ops = info.ops;
    if (ops == core::ptr::null_mut()) {
    info.ops = &msi_domain_ops_default;
    return;
    }
    if (!(info.flags & MSI_FLAG_USE_DEF_DOM_OPS)) {
    return;
    }
    if (ops.get_hwirq == core::ptr::null_mut()) {
    ops.get_hwirq = msi_domain_ops_default.get_hwirq;
    }
    if (ops.msi_init == core::ptr::null_mut()) {
    ops.msi_init = msi_domain_ops_default.msi_init;
    }
    if (ops.msi_prepare == core::ptr::null_mut()) {
    ops.msi_prepare = msi_domain_ops_default.msi_prepare;
    }
    if (ops.msi_teardown == core::ptr::null_mut()) {
    ops.msi_teardown = msi_domain_ops_default.msi_teardown;
    }
    if (ops.set_desc == core::ptr::null_mut()) {
    ops.set_desc = msi_domain_ops_default.set_desc;
    }
    }
#[no_mangle]
unsafe extern "C" fn msi_domain_update_chip_ops(info: *mut msi_domain_info) {
    let mut chip = info.chip;
    BUG_ON!(!chip || !chip.irq_mask || !chip.irq_unmask);
    if (!chip.irq_set_affinity && !(info.flags & MSI_FLAG_NO_AFFINITY)) {
    chip.irq_set_affinity = msi_domain_set_affinity;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __msi_create_irq_domain(fwnode: *mut fwnode_handle, info: *mut msi_domain_info, flags: c_uint, parent: *mut irq_domain) -> *mut c_void {
pub static mut domain: *mut c_void = core::ptr::null_mut();
    if (info.hwsize > MSI_XA_DOMAIN_SIZE) {
    return core::ptr::null_mut();
    }
//
// Hardware size 0 is valid for backwards compatibility and for
// domains which are not backed by a hardware table. Grant the
// maximum index space.
//
    if (!info.hwsize) {
    info.hwsize = MSI_XA_DOMAIN_SIZE;
    }
    msi_domain_update_dom_ops(info);
    if (info.flags & MSI_FLAG_USE_DEF_CHIP_OPS) {
    msi_domain_update_chip_ops(info);
    }
    domain = irq_domain_create_hierarchy(parent, flags | IRQ_DOMAIN_FLAG_MSI, 0,
    fwnode, &msi_domain_ops, info);
    if (domain) {
    irq_domain_update_bus_token(domain, info.bus_token);
    domain.dev = info.dev;
    if (info.flags & MSI_FLAG_PARENT_PM_DEV) {
    domain.pm_dev = parent.pm_dev;
    }
    }
    return domain;
    }
//
// msi_create_irq_domain - Create an MSI interrupt domain
// @fwnode:	Optional fwnode of the interrupt controller
// @info:	MSI domain info
// @parent:	Parent irq domain
//
// Return: pointer to the created &struct irq_domain or %NULL on failure
//
#[no_mangle]
pub unsafe extern "C" fn msi_create_irq_domain(fwnode: *mut fwnode_handle, info: *mut msi_domain_info, parent: *mut irq_domain) -> *mut c_void {
    return __msi_create_irq_domain(fwnode, info, 0, parent);
    }
//
// msi_create_parent_irq_domain - Create an MSI-parent interrupt domain
// @info:		MSI irqdomain creation info
// @msi_parent_ops:	MSI parent callbacks and configuration
//
// Return: pointer to the created &struct irq_domain or %NULL on failure
//
#[no_mangle]
pub unsafe extern "C" fn msi_create_parent_irq_domain(info: *mut irq_domain_info, msi_parent_ops: *mut msi_parent_ops) -> *mut c_void {
pub static mut d: *mut c_void = core::ptr::null_mut();
    info.hwirq_max		= max(info.hwirq_max, info.size);
    info.size		= info.hwirq_max;
    info.domain_flags	|= IRQ_DOMAIN_FLAG_MSI_PARENT;
    info.bus_token		= msi_parent_ops.bus_select_token;
    d = irq_domain_instantiate(info);
    if (IS_ERR(d)) {
    return core::ptr::null_mut();
    }
    d.msi_parent_ops = msi_parent_ops;
    return d;
    }
    EXPORT_SYMBOL_GPL(msi_create_parent_irq_domain);
//
// msi_parent_init_dev_msi_info - Delegate initialization of device MSI info down
// in the domain hierarchy
// @dev:		The device for which the domain should be created
// @domain:		The domain in the hierarchy this op is being called on
// @msi_parent_domain:	The IRQ_DOMAIN_FLAG_MSI_PARENT domain for the child to
// be created
// @msi_child_info:	The MSI domain info of the IRQ_DOMAIN_FLAG_MSI_DEVICE
// domain to be created
//
// Return: true on success, false otherwise
//
// This is the most complex problem of per device MSI domains and the
// underlying interrupt domain hierarchy:
//
// The device domain to be initialized requests the broadest feature set
// possible and the underlying domain hierarchy puts restrictions on it.
//
// That's trivial for a simple parent->child relationship, but it gets
// interesting with an intermediate domain: root->parent->child.  The
// intermediate 'parent' can expand the capabilities which the 'root'
// domain is providing. So that creates a classic hen and egg problem:
// Which entity is doing the restrictions/expansions?
//
// One solution is to let the root domain handle the initialization that's
// why there is the @domain and the @msi_parent_domain pointer.
//
#[no_mangle]
pub unsafe extern "C" fn msi_parent_init_dev_msi_info(dev: *mut device, domain: *mut irq_domain, msi_parent_domain: *mut irq_domain, msi_child_info: *mut msi_domain_info) -> bool {
    let mut parent = domain.parent;
    if (WARN_ON_ONCE!(!parent || !parent.msi_parent_ops ||
    !parent.msi_parent_ops.init_dev_msi_info)) {
    return false;
    }
    return parent.msi_parent_ops.init_dev_msi_info(dev, parent, msi_parent_domain,
    msi_child_info);
    }
//
// msi_create_device_irq_domain - Create a device MSI interrupt domain
// @dev:		Pointer to the device
// @domid:		Domain id
// @template:		MSI domain info bundle used as template
// @hwsize:		Maximum number of MSI table entries (0 if unknown or unlimited)
// @domain_data:	Optional pointer to domain specific data which is set in
// msi_domain_info::data
// @chip_data:		Optional pointer to chip specific data which is set in
// msi_domain_info::chip_data
//
// Return: True on success, false otherwise
//
// There is no firmware node required for this interface because the per
// device domains are software constructs which are actually closer to the
// hardware reality than any firmware can describe them.
//
// The domain name and the irq chip name for a MSI device domain are
// composed by: "$(PREFIX)$(CHIPNAME)-$(DEVNAME)"
//
// $PREFIX:   Optional prefix provided by the underlying MSI parent domain
// via msi_parent_ops::prefix. If that pointer is NULL the prefix
// is empty.
// $CHIPNAME: The name of the irq_chip in @template
// $DEVNAME:  The name of the device
//
// This results in understandable chip names and hardware interrupt numbers
// in e.g. /proc/interrupts
//
// PCI-MSI-0000:00:1c.0     0-edge  Parent domain has no prefix
// IR-PCI-MSI-0000:00:1c.4  0-edge  Same with interrupt remapping prefix 'IR-'
//
// IR-PCI-MSIX-0000:3d:00.0 0-edge  Hardware interrupt numbers reflect
// IR-PCI-MSIX-0000:3d:00.0 1-edge  the real MSI-X index on that device
// IR-PCI-MSIX-0000:3d:00.0 2-edge
//
// On IMS domains the hardware interrupt number is either a table entry
// index or a purely software managed index but it is guaranteed to be
// unique.
//
// The domain pointer is stored in @dev::msi::data::__irqdomains[]. All
// subsequent operations on the domain depend on the domain id.
//
// The domain is automatically freed when the device is removed via devres
// in the context of @dev::msi::data freeing, but it can also be
// independently removed via @msi_remove_device_irq_domain().
//
#[no_mangle]
pub unsafe extern "C" fn msi_create_device_irq_domain(dev: *mut device, domid: c_uint, template: *mut msi_domain_template, hwsize: c_uint, domain_data: *mut c_void, chip_data: *mut c_void) -> bool {
    struct irq_domain *domain, *parent = dev.msi.domain;
pub static mut pops: *mut c_void = core::ptr::null_mut();
pub static mut fwnode: *mut c_void = core::ptr::null_mut();
    if (!irq_domain_is_msi_parent(parent)) {
    return false;
    }
    if (domid >= MSI_MAX_DEVICE_IRQDOMAINS) {
    return false;
    }
    struct msi_domain_template *bundle __free(kfree) =
    kmemdup(template, sizeof!(*bundle), GFP_KERNEL);
    if (!bundle) {
    return false;
    }
    bundle.info.hwsize = hwsize;
    bundle.info.chip = &bundle.chip;
    bundle.info.ops = &bundle.ops;
    bundle.info.data = domain_data;
    bundle.info.chip_data = chip_data;
    bundle.info.alloc_data = &bundle.alloc_info;
    bundle.info.dev = dev;
    pops = parent.msi_parent_ops;
    snprintf(bundle.name, sizeof!(bundle.name), "%s%s-%s",
    pops.prefix ? : "", bundle.chip.name, dev_name(dev));
    bundle.chip.name = bundle.name;
//
// Using the device firmware node is required for wire to MSI
// device domains so that the existing firmware results in a domain
// match.
// All other device domains like PCI/MSI use the named firmware
// node as they are not guaranteed to have a fwnode. They are never
// looked up and always handled in the context of the device.
//
    struct fwnode_handle *fwnode_alloced __free(irq_domain_free_fwnode) = core::ptr::null_mut();
    if (!(bundle.info.flags & MSI_FLAG_USE_DEV_FWNODE)) {
    fwnode = fwnode_alloced = irq_domain_alloc_named_fwnode(bundle.name);
    }
    else {
    fwnode = dev.fwnode;
    }
    if (!fwnode) {
    return false;
    }
    if (msi_setup_device_data(dev)) {
    return false;
    }
    guard(msi_descs_lock)(dev);
    if (WARN_ON_ONCE!(msi_get_device_domain(dev, domid))) {
    return false;
    }
    if (!pops.init_dev_msi_info(dev, parent, parent, &bundle.info)) {
    return false;
    }
    domain = __msi_create_irq_domain(fwnode, &bundle.info, IRQ_DOMAIN_FLAG_MSI_DEVICE, parent);
    if (!domain) {
    return false;
    }
    dev.msi.data.__domains[domid].domain = domain;
    if (msi_domain_prepare_irqs(domain, dev, hwsize, &bundle.alloc_info)) {
    dev.msi.data.__domains[domid].domain = core::ptr::null_mut();
    irq_domain_remove(domain);
    return false;
    }
// @bundle and @fwnode_alloced are now in use. Prevent cleanup
    retain_and_null_ptr(bundle);
    retain_and_null_ptr(fwnode_alloced);
    return true;
    }
//
// msi_remove_device_irq_domain - Free a device MSI interrupt domain
// @dev:	Pointer to the device
// @domid:	Domain id
//
#[no_mangle]
pub unsafe extern "C" fn msi_remove_device_irq_domain(dev: *mut device, domid: c_uint) {
    let mut fwnode = core::ptr::null_mut();
pub static mut info: *mut c_void = core::ptr::null_mut();
pub static mut domain: *mut c_void = core::ptr::null_mut();
    guard(msi_descs_lock)(dev);
    domain = msi_get_device_domain(dev, domid);
    if (!domain || !irq_domain_is_msi_device(domain)) {
    return;
    }
    dev.msi.data.__domains[domid].domain = core::ptr::null_mut();
    info = domain.host_data;
    info.ops.msi_teardown(domain, info.alloc_data);
    if (irq_domain_is_msi_device(domain)) {
    fwnode = domain.fwnode;
    }
    irq_domain_remove(domain);
    irq_domain_free_fwnode(fwnode);
    kfree(container_of!(info, msi_domain_template, info));
    }
//
// msi_match_device_irq_domain - Match a device irq domain against a bus token
// @dev:	Pointer to the device
// @domid:	Domain id
// @bus_token:	Bus token to match against the domain bus token
//
// Return: True if device domain exists and bus tokens match.
//
#[no_mangle]
pub unsafe extern "C" fn msi_match_device_irq_domain(dev: *mut device, domid: c_uint, bus_token: irq_domain_bus_token) -> bool {
pub static mut info: *mut c_void = core::ptr::null_mut();
pub static mut domain: *mut c_void = core::ptr::null_mut();
    guard(msi_descs_lock)(dev);
    domain = msi_get_device_domain(dev, domid);
    if (domain && irq_domain_is_msi_device(domain)) {
    info = domain.host_data;
    return info.bus_token == bus_token;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn msi_domain_prepare_irqs(domain: *mut irq_domain, dev: *mut device, nvec: c_int, arg: *mut msi_alloc_info_t) -> c_int {
    let mut info = domain.host_data;
    let mut ops = info.ops;
    return ops.msi_prepare(domain, dev, nvec, arg);
    }
//
// Carefully check whether the device can use reservation mode. If
// reservation mode is enabled then the early activation will assign a
// dummy vector to the device. If the PCI/MSI device does not support
// masking of the entry then this can result in spurious interrupts when
// the device driver is not absolutely careful. But even then a malfunction
// of the hardware could result in a spurious interrupt on the dummy vector
// and render the device unusable. If the entry can be masked then the core
// logic will prevent the spurious interrupt and reservation mode can be
// used. For now reservation mode is restricted to PCI/MSI.
//
#[no_mangle]
pub unsafe extern "C" fn msi_check_reservation_mode(domain: *mut irq_domain, info: *mut msi_domain_info, dev: *mut device) -> bool {
pub static mut desc: *mut c_void = core::ptr::null_mut();
    match (domain.bus_token) {
    DOMAIN_BUS_PCI_MSI => {
    }
    DOMAIN_BUS_PCI_DEVICE_MSI => {
    }
    DOMAIN_BUS_PCI_DEVICE_MSIX => {
    }
    DOMAIN_BUS_VMD_MSI => {
    // break;
    }
    _ => {
    return false;
    }
    }
    if (!(info.flags & MSI_FLAG_MUST_REACTIVATE)) {
    return false;
    }
    if (info.flags & MSI_FLAG_NO_MASK) {
    return false;
    }
//
// Checking the first MSI descriptor is sufficient. MSIX supports
// masking and MSI does so when the can_mask attribute is set.
//
    desc = msi_first_desc(dev, MSI_DESC_ALL);
    return desc.pci.msi_attrib.is_msix || desc.pci.msi_attrib.can_mask;
    }
#[no_mangle]
pub unsafe extern "C" fn msi_handle_pci_fail(domain: *mut irq_domain, desc: *mut msi_desc, allocated: c_int) -> c_int {
    match (domain.bus_token) {
    DOMAIN_BUS_PCI_MSI => {
    }
    DOMAIN_BUS_PCI_DEVICE_MSI => {
    }
    DOMAIN_BUS_PCI_DEVICE_MSIX => {
    }
    DOMAIN_BUS_VMD_MSI => {
    if (IS_ENABLED!(CONFIG_PCI_MSI)) {
    // break;
    }
    fallthrough;
    }
    _ => {
    return -ENOSPC;
    }
    }
// Let a failed PCI multi MSI allocation retry
    if (desc.nvec_used > 1) {
    return 1;
    }
// If there was a successful allocation let the caller know
    return allocated ? allocated : -ENOSPC;
    }
pub const VIRQ_CAN_RESERVE: c_uint = 0x01;
pub const VIRQ_ACTIVATE: c_uint = 0x02;
#[no_mangle]
unsafe extern "C" fn msi_init_virq(domain: *mut irq_domain, virq: c_int, vflags: c_uint) -> c_int {
    let mut irqd = irq_domain_get_irq_data(domain, virq);
    let mut ret = 0;
    if (!(vflags & VIRQ_CAN_RESERVE)) {
    irqd_clr_can_reserve(irqd);
//
// If the interrupt is managed but no CPU is available to
// service it, shut it down until better times. Note that
// we only do this on the !RESERVE path as x86 (the only
// architecture using this flag) deals with this in a
// different way by using a catch-all vector.
//
    if ((vflags & VIRQ_ACTIVATE) &&
    irqd_affinity_is_managed(irqd) &&
    !cpumask_intersects(irq_data_get_affinity_mask(irqd),
    cpu_online_mask)) {
    irqd_set_managed_shutdown(irqd);
    return 0;
    }
    }
    if (!(vflags & VIRQ_ACTIVATE)) {
    return 0;
    }
    ret = irq_domain_activate_irq(irqd, vflags & VIRQ_CAN_RESERVE);
    if (ret) {
    return ret;
    }
//
// If the interrupt uses reservation mode, clear the activated bit
// so request_irq() will assign the final vector.
//
    if (vflags & VIRQ_CAN_RESERVE) {
    irqd_clr_activated(irqd);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn populate_alloc_info(domain: *mut irq_domain, dev: *mut device, nirqs: c_uint, arg: *mut msi_alloc_info_t) -> c_int {
    let mut info = domain.host_data;
//
// If the caller has provided a template alloc info, use that. Once
// all users of msi_create_irq_domain() have been eliminated, this
// should be the only source of allocation information, and the
// prepare call below should be finally removed.
//
    if (!info.alloc_data) {
    return msi_domain_prepare_irqs(domain, dev, nirqs, arg);
    }
// arg = *info->alloc_data;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __msi_domain_alloc_irqs(dev: *mut device, domain: *mut irq_domain, ctrl: *mut msi_ctrl) -> c_int {
    let mut xa = &dev.msi.data.__domains[ctrl.domid].store;
    let mut info = domain.host_data;
    let mut ops = info.ops;
pub static mut vflags: c_uint = 0;
pub static mut arg: msi_alloc_info_t = 0;
pub static mut desc: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    let mut i = 0;
    let mut ret = 0;
    let mut virq = 0;
    ret = populate_alloc_info(domain, dev, ctrl.nirqs, &arg);
    if (ret) {
    return ret;
    }
//
// This flag is set by the PCI layer as we need to activate
// the MSI entries before the PCI layer enables MSI in the
// card. Otherwise the card latches a random msi message.
//
    if (info.flags & MSI_FLAG_ACTIVATE_EARLY) {
    vflags |= VIRQ_ACTIVATE;
    }
//
// Interrupt can use a reserved vector and will not occupy
// a real device vector until the interrupt is requested.
//
    if (msi_check_reservation_mode(domain, info, dev)) {
    vflags |= VIRQ_CAN_RESERVE;
    }
    xa_for_each_range(xa, idx, desc, ctrl.first, ctrl.last) {
    if (!msi_desc_match(desc, MSI_DESC_NOTASSOCIATED)) {
    continue;
    }
// This should return -ECONFUSED...
    if (WARN_ON_ONCE!(allocated >= ctrl.nirqs)) {
    return -EINVAL;
    }
    if (ops.prepare_desc) {
    ops.prepare_desc(domain, &arg, desc);
    }
    ops.set_desc(&arg, desc);
    virq = __irq_domain_alloc_irqs(domain, -1, desc.nvec_used,
    dev_to_node(dev), &arg, false,
    desc.affinity);
    if (virq < 0) {
    return msi_handle_pci_fail(domain, desc, allocated);
    }
    while (i < desc.nvec_used) {
    irq_set_msi_desc_off(virq, i, desc);
    irq_debugfs_copy_devname(virq + i, dev);
    ret = msi_init_virq(domain, virq + i, vflags);
    if (ret) {
    return ret;
    }
    }
    if (info.flags & MSI_FLAG_DEV_SYSFS) {
    ret = msi_sysfs_populate_desc(dev, desc);
    if (ret) {
    return ret;
    }
    }
    allocated += 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn msi_domain_alloc_simple_msi_descs(dev: *mut device, info: *mut msi_domain_info, ctrl: *mut msi_ctrl) -> c_int {
    if (!(info.flags & MSI_FLAG_ALLOC_SIMPLE_MSI_DESCS)) {
    return 0;
    }
    return msi_domain_add_simple_msi_descs(dev, ctrl);
    }
#[no_mangle]
unsafe extern "C" fn __msi_domain_alloc_locked(dev: *mut device, ctrl: *mut msi_ctrl) -> c_int {
pub static mut info: *mut c_void = core::ptr::null_mut();
pub static mut ops: *mut c_void = core::ptr::null_mut();
pub static mut domain: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!msi_ctrl_valid(dev, ctrl)) {
    return -EINVAL;
    }
    domain = msi_get_device_domain(dev, ctrl.domid);
    if (!domain) {
    return -ENODEV;
    }
    info = domain.host_data;
    ret = msi_domain_alloc_simple_msi_descs(dev, info, ctrl);
    if (ret) {
    return ret;
    }
    ops = info.ops;
    if (ops.domain_alloc_irqs) {
    return ops.domain_alloc_irqs(domain, dev, ctrl.nirqs);
    }
    return __msi_domain_alloc_irqs(dev, domain, ctrl);
    }
#[no_mangle]
unsafe extern "C" fn msi_domain_alloc_locked(dev: *mut device, ctrl: *mut msi_ctrl) -> c_int {
pub static mut ret: c_int = 0;
    if (ret) {
    msi_domain_free_locked(dev, ctrl);
    }
    return ret;
    }
//
// msi_domain_alloc_irqs_range_locked - Allocate interrupts from a MSI interrupt domain
// @dev:	Pointer to device struct of the device for which the interrupts
// are allocated
// @domid:	Id of the interrupt domain to operate on
// @first:	First index to allocate (inclusive)
// @last:	Last index to allocate (inclusive)
//
// Must be invoked from within a msi_lock_descs() / msi_unlock_descs()
// pair. Use this for MSI irqdomains which implement their own descriptor
// allocation/free.
//
// Return: %0 on success or an error code.
//
#[no_mangle]
pub unsafe extern "C" fn msi_domain_alloc_irqs_range_locked(dev: *mut device, domid: c_uint, first: c_uint, last: c_uint) -> c_int {
pub static mut msi_ctrl: usize = 0;
    return msi_domain_alloc_locked(dev, &ctrl);
    }
//
// msi_domain_alloc_irqs_range - Allocate interrupts from a MSI interrupt domain
// @dev:	Pointer to device struct of the device for which the interrupts
// are allocated
// @domid:	Id of the interrupt domain to operate on
// @first:	First index to allocate (inclusive)
// @last:	Last index to allocate (inclusive)
//
// Return: %0 on success or an error code.
//
#[no_mangle]
pub unsafe extern "C" fn msi_domain_alloc_irqs_range(dev: *mut device, domid: c_uint, first: c_uint, last: c_uint) -> c_int {
    guard(msi_descs_lock)(dev);
    return msi_domain_alloc_irqs_range_locked(dev, domid, first, last);
    }
    EXPORT_SYMBOL_GPL(msi_domain_alloc_irqs_range);
//
// msi_domain_alloc_irqs_all_locked - Allocate all interrupts from a MSI interrupt domain
//
// @dev:	Pointer to device struct of the device for which the interrupts
// are allocated
// @domid:	Id of the interrupt domain to operate on
// @nirqs:	The number of interrupts to allocate
//
// This function scans all MSI descriptors of the MSI domain and allocates interrupts
// for all unassigned ones. That function is to be used for MSI domain usage where
// the descriptor allocation is handled at the call site, e.g. PCI/MSI[X].
//
// Return: %0 on success or an error code.
//
#[no_mangle]
pub unsafe extern "C" fn msi_domain_alloc_irqs_all_locked(dev: *mut device, domid: c_uint, nirqs: c_int) -> c_int {
pub static mut msi_ctrl: usize = 0;
    return msi_domain_alloc_locked(dev, &ctrl);
    }
#[no_mangle]
pub unsafe extern "C" fn __msi_domain_alloc_irq_at(dev: *mut device, domid: c_uint, index: c_uint, affdesc: *mut irq_affinity_desc, icookie: *mut union msi_instance_cookie) {
pub static mut ctrl: msi_ctrl = 0;
pub static mut domain: *mut c_void = core::ptr::null_mut();
pub static mut map: msi_map = 0;
pub static mut desc: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    domain = msi_get_device_domain(dev, domid);
    if (!domain) {
    map.index = -ENODEV;
    return map;
    }
    desc = msi_alloc_desc(dev, 1, affdesc);
    if (!desc) {
    map.index = -ENOMEM;
    return map;
    }
    if (icookie) {
    desc.data.icookie = *icookie;
    }
    ret = msi_insert_desc(dev, desc, domid, index);
    if (ret) {
    map.index = ret;
    return map;
    }
    ctrl.first = ctrl.last = desc.msi_index;
    ret = __msi_domain_alloc_irqs(dev, domain, &ctrl);
    if (ret) {
    map.index = ret;
    msi_domain_free_locked(dev, &ctrl);
    } else {
    map.index = desc.msi_index;
    map.virq = desc.irq;
    }
    return map;
    }
//
// msi_domain_alloc_irq_at - Allocate an interrupt from a MSI interrupt domain at
// a given index - or at the next free index
//
// @dev:	Pointer to device struct of the device for which the interrupts
// are allocated
// @domid:	Id of the interrupt domain to operate on
// @index:	Index for allocation. If @index == %MSI_ANY_INDEX the allocation
// uses the next free index.
// @affdesc:	Optional pointer to an interrupt affinity descriptor structure
// @icookie:	Optional pointer to a domain specific per instance cookie. If
// non-NULL the content of the cookie is stored in msi_desc::data.
// Must be NULL for MSI-X allocations
//
// This requires a MSI interrupt domain which lets the core code manage the
// MSI descriptors.
//
// Return: msi_map
//
// On success msi_map::index contains the allocated index number and
// msi_map::virq the corresponding Linux interrupt number
//
// On failure msi_map::index contains the error code and msi_map::virq
// is %0.
//
#[no_mangle]
pub unsafe extern "C" fn msi_domain_alloc_irq_at(dev: *mut device, domid: c_uint, index: c_uint, affdesc: *mut irq_affinity_desc, icookie: *mut union msi_instance_cookie) {
    guard(msi_descs_lock)(dev);
    return __msi_domain_alloc_irq_at(dev, domid, index, affdesc, icookie);
    }
//
// msi_device_domain_alloc_wired - Allocate a "wired" interrupt on @domain
// @domain:	The domain to allocate on
// @hwirq:	The hardware interrupt number to allocate for
// @type:	The interrupt type
//
// This weirdness supports wire to MSI controllers like MBIGEN.
//
// @hwirq is the hardware interrupt number which is handed in from
// irq_create_fwspec_mapping(). As the wire to MSI domain is sparse, but
// sized in firmware, the hardware interrupt number cannot be used as MSI
// index. For the underlying irq chip the MSI index is irrelevant and
// all it needs is the hardware interrupt number.
//
// To handle this the MSI index is allocated with MSI_ANY_INDEX and the
// hardware interrupt number is stored along with the type information in
// msi_desc::cookie so the underlying interrupt chip and domain code can
// retrieve it.
//
// Return: The Linux interrupt number (> 0) or an error code
//
#[no_mangle]
pub unsafe extern "C" fn msi_device_domain_alloc_wired(domain: *mut irq_domain, hwirq: c_uint, type: c_uint) -> c_int {
pub static mut domid: c_uint = 0;
pub static mut icookie: union msi_instance_cookie = 0;
    let mut dev = domain.dev;
pub static mut map: msi_map = 0;
    if (WARN_ON_ONCE!(!dev || domain.bus_token != DOMAIN_BUS_WIRED_TO_MSI)) {
    return -EINVAL;
    }
    icookie.value = ((u64)type << 32) | hwirq;
    guard(msi_descs_lock)(dev);
    if (WARN_ON_ONCE!(msi_get_device_domain(dev, domid) != domain)) {
    map.index = -EINVAL;
    }
    else {
    map = __msi_domain_alloc_irq_at(dev, domid, MSI_ANY_INDEX, core::ptr::null_mut(), &icookie);
    }
    return map.index >= 0 ? map.virq : map.index;
    }
#[no_mangle]
pub unsafe extern "C" fn __msi_domain_free_irqs(dev: *mut device, domain: *mut irq_domain, ctrl: *mut msi_ctrl) {
    let mut xa = &dev.msi.data.__domains[ctrl.domid].store;
    let mut info = domain.host_data;
pub static mut irqd: *mut c_void = core::ptr::null_mut();
pub static mut desc: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    let mut i = 0;
    xa_for_each_range(xa, idx, desc, ctrl.first, ctrl.last) {
// Only handle MSI entries which have an interrupt associated
    if (!msi_desc_match(desc, MSI_DESC_ASSOCIATED)) {
    continue;
    }
// Make sure all interrupts are deactivated
    while (i < desc.nvec_used) {
    irqd = irq_domain_get_irq_data(domain, desc.irq + i);
    if (irqd && irqd_is_activated(irqd)) {
    irq_domain_deactivate_irq(irqd);
    }
    }
    irq_domain_free_irqs(desc.irq, desc.nvec_used);
    if (info.flags & MSI_FLAG_DEV_SYSFS) {
    msi_sysfs_remove_desc(dev, desc);
    }
    desc.irq = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn msi_domain_free_locked(dev: *mut device, ctrl: *mut msi_ctrl) {
pub static mut info: *mut c_void = core::ptr::null_mut();
pub static mut ops: *mut c_void = core::ptr::null_mut();
pub static mut domain: *mut c_void = core::ptr::null_mut();
    if (!msi_ctrl_valid(dev, ctrl)) {
    return;
    }
    domain = msi_get_device_domain(dev, ctrl.domid);
    if (!domain) {
    return;
    }
    info = domain.host_data;
    ops = info.ops;
    if (ops.domain_free_irqs) {
    ops.domain_free_irqs(domain, dev);
    }
    else {
    __msi_domain_free_irqs(dev, domain, ctrl);
    }
    if (info.flags & MSI_FLAG_FREE_MSI_DESCS) {
    msi_domain_free_descs(dev, ctrl);
    }
    }
//
// msi_domain_free_irqs_range_locked - Free a range of interrupts from a MSI interrupt domain
// associated to @dev with msi_lock held
// @dev:	Pointer to device struct of the device for which the interrupts
// are freed
// @domid:	Id of the interrupt domain to operate on
// @first:	First index to free (inclusive)
// @last:	Last index to free (inclusive)
//
#[no_mangle]
pub unsafe extern "C" fn msi_domain_free_irqs_range_locked(dev: *mut device, domid: c_uint, first: c_uint, last: c_uint) {
pub static mut msi_ctrl: usize = 0;
    msi_domain_free_locked(dev, &ctrl);
    }
//
// msi_domain_free_irqs_range - Free a range of interrupts from a MSI interrupt domain
// associated to @dev
// @dev:	Pointer to device struct of the device for which the interrupts
// are freed
// @domid:	Id of the interrupt domain to operate on
// @first:	First index to free (inclusive)
// @last:	Last index to free (inclusive)
//
#[no_mangle]
pub unsafe extern "C" fn msi_domain_free_irqs_range(dev: *mut device, domid: c_uint, first: c_uint, last: c_uint) {
    guard(msi_descs_lock)(dev);
    msi_domain_free_irqs_range_locked(dev, domid, first, last);
    }
//
// msi_domain_free_irqs_all_locked - Free all interrupts from a MSI interrupt domain
// associated to a device
// @dev:	Pointer to device struct of the device for which the interrupts
// are freed
// @domid:	The id of the domain to operate on
//
// Must be invoked from within a msi_lock_descs() / msi_unlock_descs()
// pair. Use this for MSI irqdomains which implement their own vector
// allocation.
//
#[no_mangle]
pub unsafe extern "C" fn msi_domain_free_irqs_all_locked(dev: *mut device, domid: c_uint) {
    msi_domain_free_irqs_range_locked(dev, domid, 0,
    msi_domain_get_hwsize(dev, domid) - 1);
    }
//
// msi_domain_free_irqs_all - Free all interrupts from a MSI interrupt domain
// associated to a device
// @dev:	Pointer to device struct of the device for which the interrupts
// are freed
// @domid:	The id of the domain to operate on
//
#[no_mangle]
pub unsafe extern "C" fn msi_domain_free_irqs_all(dev: *mut device, domid: c_uint) {
    guard(msi_descs_lock)(dev);
    msi_domain_free_irqs_all_locked(dev, domid);
    }
    EXPORT_SYMBOL_GPL(msi_domain_free_irqs_all);
//
// msi_device_domain_free_wired - Free a wired interrupt in @domain
// @domain:	The domain to free the interrupt on
// @virq:	The Linux interrupt number to free
//
// This is the counterpart of msi_device_domain_alloc_wired() for the
// weird wired to MSI converting domains.
//
#[no_mangle]
pub unsafe extern "C" fn msi_device_domain_free_wired(domain: *mut irq_domain, virq: c_uint) {
    let mut desc = irq_get_msi_desc(virq);
    let mut dev = domain.dev;
    if (WARN_ON_ONCE!(!dev || !desc || domain.bus_token != DOMAIN_BUS_WIRED_TO_MSI)) {
    return;
    }
    guard(msi_descs_lock)(dev);
    if (WARN_ON_ONCE!(msi_get_device_domain(dev, MSI_DEFAULT_DOMAIN) != domain)) {
    return;
    }
    msi_domain_free_irqs_range_locked(dev, MSI_DEFAULT_DOMAIN, desc.msi_index,
    desc.msi_index);
    }
//
// msi_get_domain_info - Get the MSI interrupt domain info for @domain
// @domain:	The interrupt domain to retrieve data from
//
// Return: the pointer to the msi_domain_info stored in @domain->host_data.
//
#[no_mangle]
pub unsafe extern "C" fn msi_get_domain_info(domain: *mut irq_domain) -> *mut c_void {
    return domain.host_data;
    }
//
// msi_device_has_isolated_msi - True if the device has isolated MSI
// @dev: The device to check
//
// Isolated MSI means that HW modeled by an irq_domain on the path from the
// initiating device to the CPU will validate that the MSI message specifies an
// interrupt number that the device is authorized to trigger. This must block
// devices from triggering interrupts they are not authorized to trigger.
// Currently authorization means the MSI vector is one assigned to the device.
//
// This is interesting for securing VFIO use cases where a rouge MSI (eg created
// by abusing a normal PCI MemWr DMA) must not allow the VFIO userspace to
// impact outside its security domain, eg userspace triggering interrupts on
// kernel drivers, a VM triggering interrupts on the hypervisor, or a VM
// triggering interrupts on another VM.
//
#[no_mangle]
pub unsafe extern "C" fn msi_device_has_isolated_msi(dev: *mut device) -> bool {
    let mut domain = dev_get_msi_domain(dev);
    for (; domain; domain = domain.parent) {
    if (domain.flags & IRQ_DOMAIN_FLAG_ISOLATED_MSI)
    return true;
    }
    return arch_is_isolated_msi();
    }
    EXPORT_SYMBOL_GPL(msi_device_has_isolated_msi);