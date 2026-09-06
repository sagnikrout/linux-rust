//! Automatically rewritten from C to Rust
//! Source: kernel/liveupdate/kho_block.c
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
// Copyright (c) 2026, Google LLC.
// Pasha Tatashin <pasha.tatashin@soleen.com>
//
// DOC: KHO Serialization Blocks
//
// KHO provides a mechanism to preserve stateful data across a kexec handover
// by serializing it into memory blocks, and provides the common
// infrastructure for managing these blocks.
//
// Each block consists of a header (kho_block_header_ser) followed by an
// array of serialized entries. Multiple blocks are linked together via a
// physical pointer in the header, forming a linked list that can be easily
// traversed in both the current and the next kernel.
//

//
// Safeguard limit for the number of serialization blocks. This is used to
// prevent infinite loops and excessive memory allocation in case of memory
// corruption in the preserved state.
//
// With a 4KB page size, 10k blocks is about 40MB. For 32-byte entries
// (e.g. 4 u64s), each block holds up to 127 entries (accounting for the
// 16-byte header), allowing the block set to hold up to 1.27M entries.
//
pub const KHO_MAX_BLOCKS: c_int = 10000;
//
// kho_block_set_init - Initialize a block set.
// @bs:         The block set to initialize.
// @entry_size: The size of each entry in the blocks.
//
#[no_mangle]
pub unsafe extern "C" fn kho_block_set_init(bs: *mut kho_block_set, entry_size: usize) {
// bs = (kho_block_set)KHO_BLOCK_SET_INIT(*bs, entry_size);
    WARN_ON_ONCE!(!bs.count_per_block);
    }
// Serialized entries start immediately after the block header
#[no_mangle]
pub unsafe extern "C" fn kho_block_entries(block: *mut kho_block) -> *mut c_void {
    return (block.ser + 1);
    }
// Get the address of the serialized entry at the specified index
#[no_mangle]
pub unsafe extern "C" fn kho_block_entry(it: *mut kho_block_set_it, index: u64) -> *mut c_void {
    return kho_block_entries(it.block) + (index * it.bs.entry_size);
    }
// Free serialized data
#[no_mangle]
pub unsafe extern "C" fn kho_block_free_ser(bs: *mut kho_block_set, ser: *mut kho_block_header_ser) {
    if (bs.incoming) {
    kho_restore_free(ser);
    }
    else {
    kho_unpreserve_free(ser);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kho_block_alloc_ser(bs: *mut kho_block_set) -> *mut c_void {
    WARN_ON_ONCE!(bs.incoming);
    return kho_alloc_preserve(KHO_BLOCK_SIZE);
    }
#[no_mangle]
pub unsafe extern "C" fn kho_block_add(bs: *mut kho_block_set, ser: *mut kho_block_header_ser) -> c_int {
    let mut block = core::ptr::null_mut();
    let mut last = core::ptr::null_mut();
    if (bs.nblocks >= KHO_MAX_BLOCKS) {
    return -ENOSPC;
    }
    block = kzalloc_obj(*block);
    if (!block) {
    return -ENOMEM;
    }
    block.ser = ser;
    last = list_last_entry_or_null(&bs.blocks, kho_block, list);
    list_add_tail(&block.list, &bs.blocks);
    bs.nblocks += 1;
    if (last) {
    last.ser.next = virt_to_phys(ser);
    }
    else {
    bs.head_pa = virt_to_phys(ser);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kho_block_set_grow_one(bs: *mut kho_block_set) -> c_int {
pub static mut ser: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    ser = kho_block_alloc_ser(bs);
    if (IS_ERR(ser)) {
    return PTR_ERR(ser);
    }
    err = kho_block_add(bs, ser);
    if (err) {
    kho_block_free_ser(bs, ser);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kho_block_set_shrink_one(bs: *mut kho_block_set) {
    let mut last = core::ptr::null_mut();
    let mut new_last = core::ptr::null_mut();
    if (list_empty(&bs.blocks)) {
    return;
    }
    last = list_last_entry(&bs.blocks, kho_block, list);
    list_del(&last.list);
    bs.nblocks -= 1;
    kho_block_free_ser(bs, last.ser);
    kfree(last);
    new_last = list_last_entry_or_null(&bs.blocks, kho_block, list);
    if (new_last) {
    new_last.ser.next = 0;
    }
    else {
    bs.head_pa = 0;
    }
    }
//
// kho_block_set_grow - Expand the block set to accommodate the target count.
// @bs:    The block set.
// @count: The target number of valid entries to accommodate.
//
// Dynamically preallocates and links preserved memory blocks if the target
// entry count exceeds the current total capacity of the set, ensuring they
// are available during serialization/deserialization.
//
// Context: Caller must hold a lock protecting the block set.
// Return: 0 on success, or a negative errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn kho_block_set_grow(bs: *mut kho_block_set, count: u64) -> c_int {
pub static mut orig_nblocks: c_long = 0;
    let mut err = 0;
    if (WARN_ON_ONCE!(bs.incoming)) {
    return -EINVAL;
    }
    while (count > bs.nblocks * bs.count_per_block) {
    err = kho_block_set_grow_one(bs);
    if (err) {
// goto;
    }
    }
    return 0;
// label;
    while (bs.nblocks > orig_nblocks) {
    kho_block_set_shrink_one(bs);
    }
    return err;
    }
//
// kho_block_set_shrink - Shrink the block set to accommodate the target count.
// @bs:              The block set.
// @count:           The target number of valid entries to accommodate.
//
// Releases and unallocates redundant preserved memory blocks. Checks if the
// last block in the set can be removed because the remaining entry count is
// fully accommodated by the preceding blocks.
//
// Note: It is the caller's responsibility to ensure that entries are removed
// in the reverse order of their insertion. Because shrinking destroys the last
// block in the set, removing entries in any other order would corrupt active
// data.
//
// Context: Caller must hold a lock protecting the block set.
//
#[no_mangle]
pub unsafe extern "C" fn kho_block_set_shrink(bs: *mut kho_block_set, count: u64) {
    while (bs.nblocks > 0 && count <= (bs.nblocks - 1) * bs.count_per_block) {
    kho_block_set_shrink_one(bs);
    }
    }
//
// kho_block_set_is_cyclic - Check for cycles in a linked list of blocks.
// Uses Floyd's cycle-finding algorithm to ensure sanity of the incoming list.
//
// Return: true if a cycle or corruption is detected, false otherwise.
//
#[no_mangle]
unsafe extern "C" fn kho_block_set_is_cyclic(bs: *mut kho_block_set) -> bool {
pub static mut fast: *mut c_void = core::ptr::null_mut();
pub static mut slow: *mut c_void = core::ptr::null_mut();
pub static mut count: c_int = 0;
    fast = phys_to_virt(bs.head_pa);
    slow = fast;
    while (fast) {
    if (count++ >= KHO_MAX_BLOCKS) {
    pr_err!("Block set is corrupted\n");
    return true;
    }
    if (!fast.next) {
    break;
    }
    fast = phys_to_virt(fast.next);
    if (!fast.next) {
    break;
    }
    fast = phys_to_virt(fast.next);
    slow = phys_to_virt(slow.next);
    if (slow == fast) {
    pr_err!("Block set is corrupted\n");
    return true;
    }
    }
    return false;
    }
//
// kho_block_set_restore - Restore a block set from a physical address.
// @bs:      The block set to restore.
// @head_pa: Physical address of the first block header.
//
// Restores a serialized block set from a given physical address. The caller is
// responsible for ensuring that the block set @bs has been allocated and
// initialized prior to calling this function.
//
// Return: 0 on success, or a negative errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn kho_block_set_restore(bs: *mut kho_block_set, head_pa: u64) -> c_int {
pub static mut ser: *mut c_void = core::ptr::null_mut();
pub static mut next_pa: u64 = 0;
    let mut err = 0;
// Restored block sets use size from the previous kernel
    bs.incoming = true;
    if (!head_pa) {
    return 0;
    }
    bs.head_pa = head_pa;
    if (kho_block_set_is_cyclic(bs)) {
    bs.head_pa = 0;
    return -EINVAL;
    }
    while (next_pa) {
    ser = phys_to_virt(next_pa);
    if (!ser.count || ser.count > bs.count_per_block) {
    pr_warn!("Block contains invalid entry count: %llu\n",
    ser.count);
    err = -EINVAL;
// goto;
    }
    err = kho_block_add(bs, ser);
    if (err) {
// goto;
    }
    next_pa = ser.next;
    }
    return 0;
// label;
    kho_block_set_destroy(bs);
// Free the remaining un-restored blocks in the physical chain
    while (next_pa) {
    let mut next_ser = phys_to_virt(next_pa);
    next_pa = next_ser.next;
    kho_block_free_ser(bs, next_ser);
    }
    return err;
    }
//
// kho_block_set_destroy - Destroy all blocks in a block set.
// @bs:          The block set.
//
#[no_mangle]
pub unsafe extern "C" fn kho_block_set_destroy(bs: *mut kho_block_set) {
    let mut block = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    list_for_each_entry_safe(block, tmp, &bs.blocks, list) {
    list_del(&block.list);
    kho_block_free_ser(bs, block.ser);
    kfree(block);
    }
    bs.nblocks = 0;
    bs.head_pa = 0;
    }
//
// kho_block_set_clear - Clear all serialized data in a block set.
// @bs: The block set to clear.
//
#[no_mangle]
pub unsafe extern "C" fn kho_block_set_clear(bs: *mut kho_block_set) {
pub static mut block: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(block, &bs.blocks, list) {
    block.ser.count = 0;
    memset(block.ser + 1, 0, KHO_BLOCK_SIZE - sizeof!(*block.ser));
    }
    }
//
// kho_block_set_it_init - Initialize a block set iterator.
// @it:         The iterator to initialize.
// @bs:         The block set to iterate over.
//
#[no_mangle]
pub unsafe extern "C" fn kho_block_set_it_init(it: *mut kho_block_set_it, bs: *mut kho_block_set) {
    it.bs = bs;
    it.block = list_first_entry_or_null(&bs.blocks, kho_block, list);
    it.i = 0;
    }
//
// kho_block_set_it_reserve_entry - Reserve and return the next available slot for writing.
// @it: The block iterator.
//
// Reserves a slot in the current block during state serialization to add a new
// entry, advancing the internal index. If the current block is full, it
// automatically moves to the next block in the set.
//
// Return: A pointer to the reserved entry slot, or NULL if the block set's
// capacity is fully exhausted.
//
#[no_mangle]
pub unsafe extern "C" fn kho_block_set_it_reserve_entry(it: *mut kho_block_set_it) -> *mut c_void {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    if (!it.block) {
    return core::ptr::null_mut();
    }
    if (it.i == it.bs.count_per_block) {
    if (list_is_last(&it.block.list, &it.bs.blocks)) {
    return core::ptr::null_mut();
    }
    it.block = list_next_entry(it.block, list);
    it.i = 0;
    }
    entry = kho_block_entry(it, it.i++);
    it.block.ser.count = it.i;
    return entry;
    }
//
// kho_block_set_it_read_entry - Read the next serialized entry from the block set.
// @it: The block iterator.
//
// Iterates through previously written entries during state deserialization,
// respecting the actual count stored in each block's header.
//
// Return: A pointer to the next serialized entry, or NULL if all serialized
// entries have been read.
//
#[no_mangle]
pub unsafe extern "C" fn kho_block_set_it_read_entry(it: *mut kho_block_set_it) -> *mut c_void {
    if (!it.block) {
    return core::ptr::null_mut();
    }
    if (it.i == it.block.ser.count) {
    if (list_is_last(&it.block.list, &it.bs.blocks)) {
    return core::ptr::null_mut();
    }
    it.block = list_next_entry(it.block, list);
    it.i = 0;
    }
    return kho_block_entry(it, it.i++);
    }
//
// kho_block_set_it_prev - Return the previous entry slot in the block set.
// @it: The block iterator.
//
// If the current index is at the start of a block, it automatically moves to
// the end of the previous block.
//
// Return: A pointer to the previous entry slot, or NULL if at the very
// beginning of the block set.
//
#[no_mangle]
pub unsafe extern "C" fn kho_block_set_it_prev(it: *mut kho_block_set_it) -> *mut c_void {
    if (!it.block) {
    return core::ptr::null_mut();
    }
    if (it.i == 0) {
    if (list_is_first(&it.block.list, &it.bs.blocks)) {
    return core::ptr::null_mut();
    }
    it.block = list_prev_entry(it.block, list);
    it.i = it.bs.count_per_block;
    }
    return kho_block_entry(it, --it.i);
    }