//! Automatically rewritten from C to Rust
//! Source: block/blk-crypto-profile.c
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
// Copyright 2019 Google LLC
//
// DOC: blk-crypto profiles
//
// 'struct blk_crypto_profile' contains all generic inline encryption-related
// state for a particular inline encryption device.  blk_crypto_profile serves
// as the way that drivers for inline encryption hardware expose their crypto
// capabilities and certain functions (e.g., functions to program and evict
// keys) to upper layers.  Device drivers that want to support inline encryption
// construct a crypto profile, then associate it with the disk's request_queue.
//
// If the device has keyslots, then its blk_crypto_profile also handles managing
// these keyslots in a device-independent way, using the driver-provided
// functions to program and evict keys as needed.  This includes keeping track
// of which key and how many I/O requests are using each keyslot, getting
// keyslots for I/O requests, and handling key eviction requests.
//
// For more information, see Documentation/block/inline-encryption.rst.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_crypto_keyslot {
    pub slot_refs: core::sync::atomic::AtomicI32,
    pub idle_slot_node: list_head,
    pub hash_node: hlist_node,
    pub key: *const blk_crypto_key,
    pub profile: *mut blk_crypto_profile,
}

#[no_mangle]
pub unsafe extern "C" fn blk_crypto_hw_enter(profile: *mut blk_crypto_profile) {
//
// Calling into the driver requires profile->lock held and the device
// resumed.  But we must resume the device first, since that can acquire
// and release profile->lock via blk_crypto_reprogram_all_keys().
//
    if (profile.dev) {
    pm_runtime_get_sync(profile.dev);
    }
    down_write(&profile.lock);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_hw_exit(profile: *mut blk_crypto_profile) {
    up_write(&profile.lock);
    if (profile.dev) {
    pm_runtime_put_sync(profile.dev);
    }
    }
//
// blk_crypto_profile_init() - Initialize a blk_crypto_profile
// @profile: the blk_crypto_profile to initialize
// @num_slots: the number of keyslots
//
// Storage drivers must call this when starting to set up a blk_crypto_profile,
// before filling in additional fields.
//
// Return: 0 on success, or else a negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_profile_init(profile: *mut blk_crypto_profile, num_slots: c_uint) -> c_int {
    let mut slot = 0;
    let mut i = 0;
    let mut slot_hashtable_size = 0;
    memset(profile, 0, sizeof!(*profile));
//
// profile->lock of an underlying device can nest inside profile->lock
// of a device-mapper device, so use a dynamic lock class to avoid
// false-positive lockdep reports.
//
    lockdep_register_key(&profile.lockdep_key);
    __init_rwsem(&profile.lock, "&profile.lock", &profile.lockdep_key);
    if (num_slots == 0) {
    return 0;
    }
// Initialize keyslot management data.
    profile.slots = kvzalloc_objs(profile.slots[0], num_slots);
    if (!profile.slots) {
// goto;
    }
    profile.num_slots = num_slots;
    init_waitqueue_head(&profile.idle_slots_wait_queue);
    INIT_LIST_HEAD(&profile.idle_slots);
    while (slot < num_slots) {
    profile.slots[slot].profile = profile;
    list_add_tail(&profile.slots[slot].idle_slot_node,
    &profile.idle_slots);
    }
    spin_lock_init(&profile.idle_slots_lock);
    slot_hashtable_size = roundup_pow_of_two(num_slots);
//
// hash_ptr() assumes bits != 0, so ensure the hash table has at least 2
// buckets.  This only makes a difference when there is only 1 keyslot.
//
    if (slot_hashtable_size < 2) {
    slot_hashtable_size = 2;
    }
    profile.log_slot_ht_size = ilog2(slot_hashtable_size);
    profile.slot_hashtable =
    kvmalloc_objs(profile.slot_hashtable[0], slot_hashtable_size);
    if (!profile.slot_hashtable) {
// goto;
    }
    for (i = 0; i < slot_hashtable_size; i++) {
    INIT_HLIST_HEAD(&profile.slot_hashtable[i]);
    }
    return 0;
// label;
    blk_crypto_profile_destroy(profile);
    return -ENOMEM;
    }
    EXPORT_SYMBOL_GPL(blk_crypto_profile_init);
#[no_mangle]
unsafe extern "C" fn blk_crypto_profile_destroy_callback(profile: *mut c_void) {
    blk_crypto_profile_destroy(profile);
    }
//
// devm_blk_crypto_profile_init() - Resource-managed blk_crypto_profile_init()
// @dev: the device which owns the blk_crypto_profile
// @profile: the blk_crypto_profile to initialize
// @num_slots: the number of keyslots
//
// Like blk_crypto_profile_init(), but causes blk_crypto_profile_destroy() to be
// called automatically on driver detach.
//
// Return: 0 on success, or else a negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn devm_blk_crypto_profile_init(dev: *mut device, profile: *mut blk_crypto_profile, num_slots: c_uint) -> c_int {
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    return devm_add_action_or_reset(dev,
    blk_crypto_profile_destroy_callback,
    profile);
    }
    EXPORT_SYMBOL_GPL(devm_blk_crypto_profile_init);
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_hash_bucket_for_key(profile: *mut blk_crypto_profile, key: *mut blk_crypto_key) -> *mut c_void {
    return &profile.slot_hashtable[
    hash_ptr(key, profile.log_slot_ht_size)];
    }
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_remove_slot_from_lru_list(slot: *mut blk_crypto_keyslot) {
    let mut profile = slot.profile;
    let mut flags = 0;
    spin_lock_irqsave(&profile.idle_slots_lock, flags);
    list_del(&slot.idle_slot_node);
    spin_unlock_irqrestore(&profile.idle_slots_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_find_keyslot(profile: *mut blk_crypto_profile, key: *mut blk_crypto_key) -> *mut c_void {
    let mut head = blk_crypto_hash_bucket_for_key(profile, key);
pub static mut slotp: *mut c_void = core::ptr::null_mut();
    hlist_for_each_entry(slotp, head, hash_node) {
    if (slotp.key == key) {
    return slotp;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_find_and_grab_keyslot(profile: *mut blk_crypto_profile, key: *mut blk_crypto_key) -> *mut c_void {
pub static mut slot: *mut c_void = core::ptr::null_mut();
    slot = blk_crypto_find_keyslot(profile, key);
    if (!slot) {
    return core::ptr::null_mut();
    }
    if (atomic_inc_return(&slot.slot_refs) == 1) {
// Took first reference to this slot; remove it from LRU list
    blk_crypto_remove_slot_from_lru_list(slot);
    }
    return slot;
    }
//
// blk_crypto_keyslot_index() - Get the index of a keyslot
// @slot: a keyslot that blk_crypto_get_keyslot() returned
//
// Return: the 0-based index of the keyslot within the device's keyslots.
//
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_keyslot_index(slot: *mut blk_crypto_keyslot) -> c_uint {
    return slot - slot.profile.slots;
    }
    EXPORT_SYMBOL_GPL(blk_crypto_keyslot_index);
//
// blk_crypto_get_keyslot() - Get a keyslot for a key, if needed.
// @profile: the crypto profile of the device the key will be used on
// @key: the key that will be used
// @slot_ptr: If a keyslot is allocated, an opaque pointer to the keyslot struct
// will be stored here.  blk_crypto_put_keyslot() must be called
// later to release it.  Otherwise, NULL will be stored here.
//
// If the device has keyslots, this gets a keyslot that's been programmed with
// the specified key.  If the key is already in a slot, this reuses it;
// otherwise this waits for a slot to become idle and programs the key into it.
//
// Context: Process context. Takes and releases profile->lock.
// Return: BLK_STS_OK on success, meaning that either a keyslot was allocated or
// one wasn't needed; or a blk_status_t error on failure.
//
    blk_status_t blk_crypto_get_keyslot(blk_crypto_profile *profile,
    const struct blk_crypto_key *key, blk_crypto_keyslot **slot_ptr)
    {
pub static mut slot: *mut c_void = core::ptr::null_mut();
    let mut slot_idx = 0;
    let mut err = 0;
// slot_ptr = NULL;
//
// If the device has no concept of "keyslots", then there is no need to
// get one.
//
    if (profile.num_slots == 0) {
    return BLK_STS_OK;
    }
    down_read(&profile.lock);
    slot = blk_crypto_find_and_grab_keyslot(profile, key);
    up_read(&profile.lock);
    if (slot) {
// goto;
    }
    for (;;) {
    blk_crypto_hw_enter(profile);
    slot = blk_crypto_find_and_grab_keyslot(profile, key);
    if (slot) {
    blk_crypto_hw_exit(profile);
// goto;
    }
//
// If we're here, that means there wasn't a slot that was
// already programmed with the key. So try to program it.
//
    if (!list_empty(&profile.idle_slots)) {
    break;
    }
    blk_crypto_hw_exit(profile);
    wait_event(profile.idle_slots_wait_queue,
    !list_empty(&profile.idle_slots));
    }
    slot = list_first_entry(&profile.idle_slots, blk_crypto_keyslot,
    idle_slot_node);
    slot_idx = blk_crypto_keyslot_index(slot);
    err = profile.ll_ops.keyslot_program(profile, key, slot_idx);
    if (err) {
    wake_up(&profile.idle_slots_wait_queue);
    blk_crypto_hw_exit(profile);
    return errno_to_blk_status(err);
    }
// Move this slot to the hash list for the new key.
    if (slot.key) {
    hlist_del(&slot.hash_node);
    }
    slot.key = key;
    hlist_add_head(&slot.hash_node,
    blk_crypto_hash_bucket_for_key(profile, key));
    atomic_set(&slot.slot_refs, 1);
    blk_crypto_remove_slot_from_lru_list(slot);
    blk_crypto_hw_exit(profile);
// label;
// slot_ptr = slot;
    return BLK_STS_OK;
    }
//
// blk_crypto_put_keyslot() - Release a reference to a keyslot
// @slot: The keyslot to release the reference of
//
// Context: Any context.
//
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_put_keyslot(slot: *mut blk_crypto_keyslot) {
    let mut profile = slot.profile;
    let mut flags = 0;
    if (atomic_dec_and_lock_irqsave(&slot.slot_refs,
    &profile.idle_slots_lock, flags)) {
    list_add_tail(&slot.idle_slot_node, &profile.idle_slots);
    spin_unlock_irqrestore(&profile.idle_slots_lock, flags);
    wake_up(&profile.idle_slots_wait_queue);
    }
    }
//
// This is an internal function that evicts a key from an inline encryption
// device that can be either a real device or the blk-crypto-fallback "device".
// It is used only by blk_crypto_evict_key(); see that function for details.
//
#[no_mangle]
pub unsafe extern "C" fn __blk_crypto_evict_key(profile: *mut blk_crypto_profile, key: *mut blk_crypto_key) -> c_int {
pub static mut slot: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (profile.num_slots == 0) {
    if (profile.ll_ops.keyslot_evict) {
    blk_crypto_hw_enter(profile);
    err = profile.ll_ops.keyslot_evict(profile, key, -1);
    blk_crypto_hw_exit(profile);
    return err;
    }
    return 0;
    }
    blk_crypto_hw_enter(profile);
    slot = blk_crypto_find_keyslot(profile, key);
    if (!slot) {
//
// Not an error, since a key not in use by I/O is not guaranteed
// to be in a keyslot.  There can be more keys than keyslots.
//
    err = 0;
// goto;
    }
    if (WARN_ON_ONCE!(atomic_read(&slot.slot_refs) != 0)) {
// BUG: key is still in use by I/O
    err = -EBUSY;
// goto;
    }
    err = profile.ll_ops.keyslot_evict(profile, key,
    blk_crypto_keyslot_index(slot));
// label;
//
// Callers free the key even on error, so unlink the key from the hash
// table and clear slot->key even on error.
//
    hlist_del(&slot.hash_node);
    slot.key = core::ptr::null_mut();
// label;
    blk_crypto_hw_exit(profile);
    return err;
    }
//
// blk_crypto_reprogram_all_keys() - Re-program all keyslots.
// @profile: The crypto profile
//
// Re-program all keyslots that are supposed to have a key programmed.  This is
// intended only for use by drivers for hardware that loses its keys on reset.
//
// Context: Process context. Takes and releases profile->lock.
//
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_reprogram_all_keys(profile: *mut blk_crypto_profile) {
    let mut slot = 0;
    if (profile.num_slots == 0) {
    return;
    }
// This is for device initialization, so don't resume the device
    down_write(&profile.lock);
    while (slot < profile.num_slots) {
    let mut key = profile.slots[slot].key;
    let mut err = 0;
    if (!key) {
    continue;
    }
    err = profile.ll_ops.keyslot_program(profile, key, slot);
    WARN_ON!(err);
    }
    up_write(&profile.lock);
    }
    EXPORT_SYMBOL_GPL(blk_crypto_reprogram_all_keys);
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_profile_destroy(profile: *mut blk_crypto_profile) {
    if (!profile) {
    return;
    }
    lockdep_unregister_key(&profile.lockdep_key);
    kvfree(profile.slot_hashtable);
    kvfree_sensitive(profile.slots,
    sizeof!(profile.slots[0]) * profile.num_slots);
    memzero_explicit(profile, sizeof!(*profile));
    }
    EXPORT_SYMBOL_GPL(blk_crypto_profile_destroy);
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_register(profile: *mut blk_crypto_profile, q: *mut request_queue) -> bool {
    if (blk_integrity_queue_supports_integrity(q)) {
    pr_warn!("Integrity and hardware inline encryption are not supported together. Disabling hardware inline encryption.\n");
    return false;
    }
    q.crypto_profile = profile;
    return true;
    }
    EXPORT_SYMBOL_GPL(blk_crypto_register);
//
// blk_crypto_derive_sw_secret() - Derive software secret from wrapped key
// @bdev: a block device that supports hardware-wrapped keys
// @eph_key: a hardware-wrapped key in ephemerally-wrapped form
// @eph_key_size: size of @eph_key in bytes
// @sw_secret: (output) the software secret
//
// Given a hardware-wrapped key in ephemerally-wrapped form (the same form that
// it is used for I/O), ask the hardware to derive the secret which software can
// use for cryptographic tasks other than inline encryption.  This secret is
// guaranteed to be cryptographically isolated from the inline encryption key,
// i.e. derived with a different KDF context.
//
// Return: 0 on success, -EOPNOTSUPP if the block device doesn't support
// hardware-wrapped keys, -EBADMSG if the key isn't a valid
// ephemerally-wrapped key, or another -errno code.
//
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_derive_sw_secret(bdev: *mut block_device, eph_key: *mut u8, eph_key_size: size_t) -> c_int {
    let mut profile = bdev_get_queue(bdev).crypto_profile;
    let mut err = 0;
    if (!profile) {
    return -EOPNOTSUPP;
    }
    if (!(profile.key_types_supported & BLK_CRYPTO_KEY_TYPE_HW_WRAPPED)) {
    return -EOPNOTSUPP;
    }
    if (!profile.ll_ops.derive_sw_secret) {
    return -EOPNOTSUPP;
    }
    blk_crypto_hw_enter(profile);
    err = profile.ll_ops.derive_sw_secret(profile, eph_key, eph_key_size,
    sw_secret);
    blk_crypto_hw_exit(profile);
    return err;
    }
    EXPORT_SYMBOL_GPL(blk_crypto_derive_sw_secret);
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_import_key(profile: *mut blk_crypto_profile, raw_key: *mut u8, raw_key_size: size_t) -> c_int {
    let mut ret = 0;
    if (!profile) {
    return -EOPNOTSUPP;
    }
    if (!(profile.key_types_supported & BLK_CRYPTO_KEY_TYPE_HW_WRAPPED)) {
    return -EOPNOTSUPP;
    }
    if (!profile.ll_ops.import_key) {
    return -EOPNOTSUPP;
    }
    blk_crypto_hw_enter(profile);
    ret = profile.ll_ops.import_key(profile, raw_key, raw_key_size,
    lt_key);
    blk_crypto_hw_exit(profile);
    return ret;
    }
    EXPORT_SYMBOL_GPL(blk_crypto_import_key);
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_generate_key(profile: *mut blk_crypto_profile) -> c_int {
    let mut ret = 0;
    if (!profile) {
    return -EOPNOTSUPP;
    }
    if (!(profile.key_types_supported & BLK_CRYPTO_KEY_TYPE_HW_WRAPPED)) {
    return -EOPNOTSUPP;
    }
    if (!profile.ll_ops.generate_key) {
    return -EOPNOTSUPP;
    }
    blk_crypto_hw_enter(profile);
    ret = profile.ll_ops.generate_key(profile, lt_key);
    blk_crypto_hw_exit(profile);
    return ret;
    }
    EXPORT_SYMBOL_GPL(blk_crypto_generate_key);
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_prepare_key(profile: *mut blk_crypto_profile, lt_key: *mut u8, lt_key_size: size_t) -> c_int {
    let mut ret = 0;
    if (!profile) {
    return -EOPNOTSUPP;
    }
    if (!(profile.key_types_supported & BLK_CRYPTO_KEY_TYPE_HW_WRAPPED)) {
    return -EOPNOTSUPP;
    }
    if (!profile.ll_ops.prepare_key) {
    return -EOPNOTSUPP;
    }
    blk_crypto_hw_enter(profile);
    ret = profile.ll_ops.prepare_key(profile, lt_key, lt_key_size,
    eph_key);
    blk_crypto_hw_exit(profile);
    return ret;
    }
    EXPORT_SYMBOL_GPL(blk_crypto_prepare_key);
//
// blk_crypto_intersect_capabilities() - restrict supported crypto capabilities
// by child device
// @parent: the crypto profile for the parent device
// @child: the crypto profile for the child device, or NULL
//
// This clears all crypto capabilities in @parent that aren't set in @child.  If
// @child is NULL, then this clears all parent capabilities.
//
// Only use this when setting up the crypto profile for a layered device, before
// it's been exposed yet.
//
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_intersect_capabilities(parent: *mut blk_crypto_profile, child: *mut blk_crypto_profile) {
    if (child) {
    let mut i = 0;
    parent.max_dun_bytes_supported =
    min(parent.max_dun_bytes_supported,
    child.max_dun_bytes_supported);
    for (i = 0; i < ARRAY_SIZE!(child.modes_supported); i++) {
    parent.modes_supported[i] &= child.modes_supported[i];
    }
    parent.key_types_supported &= child.key_types_supported;
    } else {
    parent.max_dun_bytes_supported = 0;
    memset(parent.modes_supported, 0,
    sizeof!(parent.modes_supported));
    parent.key_types_supported = 0;
    }
    }
    EXPORT_SYMBOL_GPL(blk_crypto_intersect_capabilities);
//
// blk_crypto_has_capabilities() - Check whether @target supports at least all
// the crypto capabilities that @reference does.
// @target: the target profile
// @reference: the reference profile
//
// Return: %true if @target supports all the crypto capabilities of @reference.
//
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_has_capabilities(target: *mut blk_crypto_profile, reference: *mut blk_crypto_profile) -> bool {
    let mut i = 0;
    if (!reference) {
    return true;
    }
    if (!target) {
    return false;
    }
    while (i < ARRAY_SIZE!(target.modes_supported)) {
    if (reference.modes_supported[i] & ~target.modes_supported[i]) {
    return false;
    }
    }
    if (reference.max_dun_bytes_supported >
    target.max_dun_bytes_supported) {
    return false;
    }
    if (reference.key_types_supported & ~target.key_types_supported) {
    return false;
    }
    return true;
    }
    EXPORT_SYMBOL_GPL(blk_crypto_has_capabilities);
//
// blk_crypto_update_capabilities() - Update the capabilities of a crypto
// profile to match those of another crypto
// profile.
// @dst: The crypto profile whose capabilities to update.
// @src: The crypto profile whose capabilities this function will update @dst's
// capabilities to.
//
// Blk-crypto requires that crypto capabilities that were
// advertised when a bio was created continue to be supported by the
// device until that bio is ended. This is turn means that a device cannot
// shrink its advertised crypto capabilities without any explicit
// synchronization with upper layers. So if there's no such explicit
// synchronization, @src must support all the crypto capabilities that
// @dst does (i.e. we need blk_crypto_has_capabilities(@src, @dst)).
//
// Note also that as long as the crypto capabilities are being expanded, the
// order of updates becoming visible is not important because it's alright
// for blk-crypto to see stale values - they only cause blk-crypto to
// believe that a crypto capability isn't supported when it actually is (which
// might result in blk-crypto-fallback being used if available, or the bio being
// failed).
//
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_update_capabilities(dst: *mut blk_crypto_profile, src: *mut blk_crypto_profile) {
    memcpy(dst.modes_supported, src.modes_supported,
    sizeof!(dst.modes_supported));
    dst.max_dun_bytes_supported = src.max_dun_bytes_supported;
    dst.key_types_supported = src.key_types_supported;
    }
    EXPORT_SYMBOL_GPL(blk_crypto_update_capabilities);