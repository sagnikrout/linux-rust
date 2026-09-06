//! Automatically rewritten from C to Rust
//! Source: kernel/liveupdate/kexec_handover.c
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
// kexec_handover.c - kexec handover metadata processing
// Copyright (C) 2023 Alexander Graf <graf@amazon.com>
// Copyright (C) 2025 Microsoft Corporation, Mike Rapoport <rppt@kernel.org>
// Copyright (C) 2025 Google LLC, Changyuan Lyu <changyuanl@google.com>
// Copyright (C) 2025 Pasha Tatashin <pasha.tatashin@soleen.com>
// Copyright (C) 2026 Google LLC, Jason Miu <jasonmiu@google.com>
//

//
// KHO is tightly coupled with mm init and needs access to some of mm
// internal APIs.
//

//
// This is the minimal alignment required by deferred struct page init.
// deferred_init_memmap_chunk frees memory to the buddy allocator, which looks
// at the neighboring pages (up to MAX_PAGE_ORDER) to merge them.
// If KHO scratch is not aligned to that value, buddy can access uninitialized
// struct pages, which can cause a crash.
//

    static_assert(SCRATCH_ALIGNMENT_BYTES >= CMA_MIN_ALIGNMENT_BYTES);
// The magic token for preserved pages
pub const KHO_PAGE_MAGIC: c_uint = 0x4b484f50U /* ASCII for 'KHOP' */;
//
// KHO uses page->private, which is an unsigned long, to store page metadata.
// Use it to store both the magic and the order.
//
    union kho_page_info {
    let mut page_private = 0;
    struct {
    let mut order = 0;
    let mut magic = 0;
    };
    };
    static_assert(sizeof!(union kho_page_info) == sizeof!((0).private));
pub static mut __ro_after_init: bool kho_enable = 0;
#[no_mangle]
pub unsafe extern "C" fn kho_is_enabled() -> bool {
    return kho_enable;
    }
    EXPORT_SYMBOL_GPL(kho_is_enabled);
#[no_mangle]
unsafe extern "C" fn kho_parse_enable(p: *mut c_char) -> c_int {
    return kstrtobool(p, &kho_enable);
    }
    early_param!("kho", kho_parse_enable);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kho_out {
    pub fdt: *mut c_void,
//     pub /: *mut *mut mutex lock; / protects KHO FDT,
    pub radix_tree: kho_radix_tree,
    pub dbg: kho_debugfs,
}

pub static mut kho_out: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kho_in {
    pub fdt_phys: phys_addr_t,
    pub scratch_phys: phys_addr_t,
    pub 1]: char previous_release[__NEW_UTS_LEN +,
    pub kexec_count: u32,
    pub dbg: kho_debugfs,
    pub radix_tree: kho_radix_tree,
}

pub static mut kho_in: usize = 0;
    static const void *kho_get_fdt(void)
    {
    return kho_in.fdt_phys ? phys_to_virt(kho_in.fdt_phys) : core::ptr::null_mut();
    }
//
// kho_encode_radix_key - Encodes a physical address and order into a radix key.
// @phys: The physical address of the page.
// @order: The order of the page.
//
// This function combines a page's physical address and its order into a
// single unsigned long, which is used as a key for all radix tree
// operations.
//
// Return: The encoded unsigned long radix key.
//
#[no_mangle]
unsafe extern "C" fn kho_encode_radix_key(phys: phys_addr_t, order: c_uint) -> c_ulong {
// The physical address is encoded by shifting the PFN by its order.
pub static mut shift: c_ulong = 0;
// Order bit goes right before the shifted PFN.
pub static mut h: c_ulong = 0;
// Shifted PFN.
pub static mut l: c_ulong = 0;
    return h | l;
    }
//
// kho_decode_radix_key - Decodes a radix key back into a physical address and order.
// @key: The unsigned long key to decode.
// @order: An output parameter, a pointer to an unsigned int where the decoded
// page order will be stored.
//
// This function reverses the encoding performed by kho_encode_radix_key(),
// extracting the original physical address and page order from a given key.
//
// Return: The decoded physical address.
//
#[no_mangle]
unsafe extern "C" fn kho_decode_radix_key(key: c_ulong, order: *mut c_uint) -> phys_addr_t {
// fls64() indexes starting from 1.
pub static mut order_bit: c_uint = 0;
    let mut phys;
// order bit goes right before the shifted PFN.
// order = 64 - (PAGE_SHIFT + order_bit);
// The order bit is discarded by the shift
    phys = key << (PAGE_SHIFT + *order);
    return phys;
    }
#[no_mangle]
unsafe extern "C" fn kho_radix_get_bitmap_index(key: c_ulong) -> c_ulong {
    return key % (1 << KHO_BITMAP_SIZE_LOG2);
    }
#[no_mangle]
pub unsafe extern "C" fn kho_radix_get_table_index(key: c_ulong, level: c_uint) -> c_ulong {
    let mut s = 0;
    s = ((level - 1) * KHO_TABLE_SIZE_LOG2) + KHO_BITMAP_SIZE_LOG2;
    return (key >> s) % (1 << KHO_TABLE_SIZE_LOG2);
    }
    static void __ref *kho_radix_alloc_node(void)
    {
pub static mut node: *mut c_void = core::ptr::null_mut();
    if (slab_is_available()) {
    node = get_zeroed_page(GFP_KERNEL);
    }
    else {
    node = memblock_alloc(PAGE_SIZE, PAGE_SIZE);
    }
    return node;
    }
#[no_mangle]
unsafe extern "C" fn kho_radix_free_node(node: *mut kho_radix_node) -> void __ref {
    if (slab_is_available()) {
    free_page((unsigned long)node);
    }
    else {
    memblock_free(node, PAGE_SIZE);
    }
    }
//
// kho_radix_add_key - Add a key to the radix tree.
// @tree: The KHO radix tree.
// @key: The key to add.
//
// This function traverses the radix tree based on the @key provided. It sets the
// corresponding bit in the leaf bitmap to mark the @key as present. If
// intermediate nodes do not exist along the path, they are allocated and added
// to the tree.
//
// NOTE: Currently only keys of width up to %KHO_RADIX_KEY_WIDTH are supported.
// This limit only exists because current users of the radix tree don't use more
// than that. Changing the maximum width requires changing the tree depth, which
// needs bumping the ABI version.
//
// Return: 0 on success, or a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn kho_radix_add_key(tree: *mut kho_radix_tree, key: c_ulong) -> c_int {
// Newly allocated nodes for error cleanup
    struct kho_radix_node *intermediate_nodes[KHO_TREE_MAX_DEPTH] = { 0 };
    let mut anchor_node = core::ptr::null_mut();
    let mut node = tree.root;
pub static mut new_node: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut idx = 0;
    let mut anchor_idx = 0;
pub static mut leaf: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    if (WARN_ON_ONCE!(!tree.root)) {
    return -EINVAL;
    }
    if (unlikely(fls64(key) > KHO_RADIX_KEY_WIDTH)) {
    return -ERANGE;
    }
    might_sleep();
    guard(mutex)(&tree.lock);
// Go from high levels to low levels
    while (i > 0) {
    idx = kho_radix_get_table_index(key, i);
    if (node.table[idx]) {
    node = phys_to_virt(node.table[idx]);
    continue;
    }
// Next node is empty, create a new node for it
    new_node = kho_radix_alloc_node();
    if (!new_node) {
    err = -ENOMEM;
// goto;
    }
    node.table[idx] = virt_to_phys(new_node);
//
// Capture the node where the new branch starts for cleanup
// if allocation fails.
//
    if (!anchor_node) {
    anchor_node = node;
    anchor_idx = idx;
    }
    intermediate_nodes[i] = new_node;
    node = new_node;
    }
// Handle the leaf level bitmap (level 0)
    idx = kho_radix_get_bitmap_index(key);
    leaf = node;
    __set_bit(idx, leaf.bitmap);
    return 0;
// label;
    while (i > 0) {
    if (intermediate_nodes[i]) {
    kho_radix_free_node(intermediate_nodes[i]);
    }
    }
    if (anchor_node) {
    anchor_node.table[anchor_idx] = 0;
    }
    return err;
    }
    EXPORT_SYMBOL_GPL(kho_radix_add_key);
//
// kho_radix_del_key - Removes the key from the radix tree.
// @tree: The KHO radix tree.
// @key: The key to remove.
//
// This function traverses the radix tree and clears the bit corresponding to
// the @key, effectively removing it from the tree. It does not free the tree's
// intermediate nodes, even if they become empty.
//
#[no_mangle]
pub unsafe extern "C" fn kho_radix_del_key(tree: *mut kho_radix_tree, key: c_ulong) {
    let mut node = tree.root;
pub static mut leaf: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut idx = 0;
    if (WARN_ON_ONCE!(!tree.root)) {
    return;
    }
// Keys wider than KHO_RADIX_KEY_WIDTH are not allowed to be added.
    if (unlikely(fls64(key) > KHO_RADIX_KEY_WIDTH)) {
    return;
    }
    might_sleep();
    guard(mutex)(&tree.lock);
// Go from high levels to low levels
    while (i > 0) {
    idx = kho_radix_get_table_index(key, i);
//
// Attempting to delete a page that has not been preserved,
// return with a warning.
//
    if (WARN_ON!(!node.table[idx])) {
    return;
    }
    node = phys_to_virt(node.table[idx]);
    }
// Handle the leaf level bitmap (level 0)
    leaf = node;
    idx = kho_radix_get_bitmap_index(key);
    __clear_bit(idx, leaf.bitmap);
    }
    EXPORT_SYMBOL_GPL(kho_radix_del_key);
#[no_mangle]
pub unsafe extern "C" fn __kho_radix_destroy_tree(root: *mut kho_radix_node, level: c_uint) {
    let mut i = 0;
    if (level == 0) {
    kho_radix_free_node(root);
    return;
    }
    while (i < PAGE_SIZE / sizeof!(phys_addr_t)) {
    if (root.table[i]) {
    __kho_radix_destroy_tree(phys_to_virt(root.table[i]),
    level - 1);
    }
    }
    kho_radix_free_node(root);
    }
//
// kho_radix_init_tree - initialize the radix tree.
// @tree:   the tree to initialize.
// @root:   root table of the radix tree.
//
// Initialize the radix tree with the given root node. If root is %NULL, an
// empty root table is allocated. If root is not %NULL, it is the caller's
// responsibility to make sure the root is valid and in the correct format.
//
// Return: 0 on success, -errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn kho_radix_init_tree(tree: *mut kho_radix_tree, root: *mut kho_radix_node) -> c_int {
    if (!root) {
    root = kho_radix_alloc_node();
    }
    if (!root) {
    return -ENOMEM;
    }
    tree.root = root;
    mutex_init(&tree.lock);
    return 0;
    }
    EXPORT_SYMBOL_GPL(kho_radix_init_tree);
//
// kho_radix_destroy_tree - Destroy the radix tree
// @tree: The radix tree to destroy
//
// Walk @tree and free all its nodes.
//
#[no_mangle]
pub unsafe extern "C" fn kho_radix_destroy_tree(tree: *mut kho_radix_tree) {
    if (!tree.root) {
    return;
    }
    __kho_radix_destroy_tree(tree.root, KHO_TREE_MAX_DEPTH - 1);
    tree.root = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(kho_radix_destroy_tree);
#[no_mangle]
pub unsafe extern "C" fn kho_radix_walk_leaf(leaf: *mut kho_radix_leaf, key: c_ulong, cb: *mut kho_radix_walk_cb, data: *mut c_void) -> c_int {
    let mut bitmap = leaf;
    let mut i = 0;
    let mut err = 0;
    if (cb.node) {
    err = cb.node(virt_to_phys(leaf), data);
    if (err) {
    return err;
    }
    }
    if (!cb.leaf) {
    return 0;
    }
    for_each_set_bit(i, bitmap, PAGE_SIZE * BITS_PER_BYTE) {
    err = cb.leaf(key | i, data);
    if (err) {
    return err;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __kho_radix_walk_tree(root: *mut kho_radix_node, level: c_uint, start: c_ulong, cb: *mut kho_radix_walk_cb, data: *mut c_void) -> c_int {
pub static mut node: *mut c_void = core::ptr::null_mut();
pub static mut leaf: *mut c_void = core::ptr::null_mut();
    unsigned long key, i;
    let mut shift = 0;
    let mut err = 0;
    if (cb.node) {
    err = cb.node(virt_to_phys(root), data);
    if (err) {
    return err;
    }
    }
    while (i < PAGE_SIZE / sizeof!(phys_addr_t)) {
    if (!root.table[i]) {
    continue;
    }
    shift = ((level - 1) * KHO_TABLE_SIZE_LOG2) +
    KHO_BITMAP_SIZE_LOG2;
    key = start | (i << shift);
    node = phys_to_virt(root.table[i]);
    if (level == 1) {
//
// we are at level 1,
// node is pointing to the level 0 bitmap.
//
    leaf = node;
    err = kho_radix_walk_leaf(leaf, key, cb, data);
    } else {
    err  = __kho_radix_walk_tree(node, level - 1,
    key, cb, data);
    }
    if (err) {
    return err;
    }
    }
    return 0;
    }
//
// kho_radix_walk_tree - Traverses the radix tree and calls a callback for each key.
// @tree: A pointer to the KHO radix tree to walk.
// @cb:   Set of callbacks to be invoked during the tree walk.
// @data: Opaque data pointer passed to each callback in @cb.
//
// This function walks the radix tree, searching from the top level down to the
// lowest level (level 0), invoking the appropriate callbacks.
//
// Return: 0 if the walk completed the specified tree, or the non-zero return
// value from the callback that stopped the walk.
//
#[no_mangle]
pub unsafe extern "C" fn kho_radix_walk_tree(tree: *mut kho_radix_tree, cb: *mut kho_radix_walk_cb, data: *mut c_void) -> c_int {
    if (WARN_ON_ONCE!(!tree.root)) {
    return -EINVAL;
    }
    guard(mutex)(&tree.lock);
    return __kho_radix_walk_tree(tree.root, KHO_TREE_MAX_DEPTH - 1, 0, cb,
    data);
    }
    EXPORT_SYMBOL_GPL(kho_radix_walk_tree);
// For physically contiguous 0-order pages.
#[no_mangle]
unsafe extern "C" fn kho_init_pages(page: *mut page, nr_pages: c_ulong) {
    while (i < nr_pages) {
    set_page_count(page + i, 1);
// Clear each page's codetag to avoid accounting mismatch.
    clear_page_tag_ref(page + i);
    }
    }
#[no_mangle]
unsafe extern "C" fn kho_init_folio(page: *mut page, order: c_uint) {
pub static mut nr_pages: c_ulong = 0;
// Head page gets refcount of 1.
    set_page_count(page, 1);
// Clear head page's codetag to avoid accounting mismatch.
    clear_page_tag_ref(page);
// For higher order folios, tail pages get a page count of zero.
    for (unsigned long i = 1; i < nr_pages; i++) {
    set_page_count(page + i, 0);
    }
    if (order > 0) {
    prep_compound_page(page, order);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kho_restore_page(phys: phys_addr_t, is_folio: bool) -> *mut c_void {
    let mut page = pfn_to_online_page(PHYS_PFN(phys));
    let mut nr_pages = 0;
    union kho_page_info info;
    if (!page) {
    return core::ptr::null_mut();
    }
    info.page_private = page.private;
//
// deserialize_bitmap() only sets the magic on the head page. This magic
// check also implicitly makes sure phys is order-aligned since for
// non-order-aligned phys addresses, magic will never be set.
//
    if (WARN_ON_ONCE!(info.magic != KHO_PAGE_MAGIC)) {
    return core::ptr::null_mut();
    }
    nr_pages = (1 << info.order);
// Clear private to make sure later restores on this page error out.
    page.private = 0;
    if (is_folio) {
    kho_init_folio(page, info.order);
    }
    else {
    kho_init_pages(page, nr_pages);
    }
    adjust_managed_page_count(page, nr_pages);
    return page;
    }
//
// kho_restore_folio - recreates the folio from the preserved memory.
// @phys: physical address of the folio.
//
// Return: pointer to the struct folio on success, NULL on failure.
//
#[no_mangle]
pub unsafe extern "C" fn kho_restore_folio(phys: phys_addr_t) -> *mut c_void {
    let mut page = kho_restore_page(phys, true);
    return page ? page_folio(page) : core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(kho_restore_folio);
//
// kho_restore_pages - restore list of contiguous order 0 pages.
// @phys: physical address of the first page.
// @nr_pages: number of pages.
//
// Restore a contiguous list of order 0 pages that was preserved with
// kho_preserve_pages().
//
// Return: the first page on success, NULL on failure.
//
#[no_mangle]
pub unsafe extern "C" fn kho_restore_pages(phys: phys_addr_t, nr_pages: c_ulong) -> *mut c_void {
pub static mut start_pfn: c_ulong = 0;
pub static mut end_pfn: c_ulong = 0;
pub static mut pfn: c_ulong = 0;
    while (pfn < end_pfn) {
    let mut order = min(count_trailing_zeros(pfn), ilog2(end_pfn - pfn));
    let mut page = kho_restore_page(PFN_PHYS(pfn), false);
    if (!page) {
    return core::ptr::null_mut();
    }
    pfn += 1 << order;
    }
    return pfn_to_page(start_pfn);
    }
    EXPORT_SYMBOL_GPL(kho_restore_pages);
//
// With CONFIG_DEFERRED_STRUCT_PAGE_INIT, pages in higher memory regions
// may not be initialized yet at the time KHO deserializes preserved memory.
// KHO uses the struct page to store metadata and a later initialization would
// overwrite it.
// Ensure all the struct pages in the preservation are
// initialized. kho_preserved_memory_reserve() marks the reservation as noinit
// to make sure they don't get re-initialized later.
//
    static struct page *__init kho_get_preserved_page(phys_addr_t phys,
    unsigned int order)
    {
pub static mut pfn: c_ulong = 0;
    let mut nid = 0;
    if (!IS_ENABLED!(CONFIG_DEFERRED_STRUCT_PAGE_INIT)) {
    return pfn_to_page(pfn);
    }
    nid = early_pfn_to_nid(pfn);
    for (unsigned long i = 0; i < (1UL << order); i++) {
    init_deferred_page(pfn + i, nid);
    }
    return pfn_to_page(pfn);
    }
#[no_mangle]
unsafe extern "C" fn kho_preserved_memory_reserve(key: c_ulong, data: *mut c_void) -> c_int {
    union kho_page_info info;
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut order = 0;
    let mut phys;
    let mut sz = 0;
    phys = kho_decode_radix_key(key, &order);
    sz = 1UL << (order + PAGE_SHIFT);
    page = kho_get_preserved_page(phys, order);
// Reserve the memory preserved in KHO in memblock
    memblock_reserve(phys, sz);
    memblock_reserved_mark_noinit(phys, sz);
    info.magic = KHO_PAGE_MAGIC;
    info.order = order;
    page.private = info.page_private;
    return 0;
    }
// Returns physical address of the preserved memory map from FDT
#[no_mangle]
unsafe extern "C" fn kho_get_mem_map_phys(fdt: *const c_void) -> phys_addr_t __init {
pub static mut mem_ptr: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    mem_ptr = fdt_getprop(fdt, 0, KHO_FDT_MEMORY_MAP_PROP_NAME, &len);
    if (!mem_ptr || len != sizeof!(u64)) {
    pr_err!("failed to get preserved memory map\n");
    return 0;
    }
    return get_unaligned(mem_ptr);
    }
    static void __init *kho_get_mem_map(const void *fdt)
    {
pub static mut phys: phys_addr_t = 0;
    return phys ? phys_to_virt(phys) : core::ptr::null_mut();
    }
//
// With KHO enabled, memory can become fragmented because KHO regions may
// be anywhere in physical address space. The scratch regions give us a
// safe zones that we will never see KHO allocations from. This is where we
// can later safely load our new kexec images into and then use the scratch
// area for early allocations that happen before page allocator is
// initialized.
//
pub static mut kho_scratch: *mut c_void = core::ptr::null_mut();
    let mut kho_scratch_cnt = 0;
//
// The scratch areas are scaled by default as percent of memory allocated from
// memblock. A user can override the scale with command line parameter:
//
// kho_scratch=N%
//
// It is also possible to explicitly define size for a lowmem, a global and
// per-node scratch areas:
//
// kho_scratch=l[KMG],n[KMG],m[KMG]
//
// The explicit size definition takes precedence over scale definition.
//
pub static mut __initdata: unsigned int scratch_scale = 200;
    static phys_addr_t scratch_size_global __initdata;
    static phys_addr_t scratch_size_pernode __initdata;
    static phys_addr_t scratch_size_lowmem __initdata;
#[no_mangle]
unsafe extern "C" fn kho_parse_scratch_size(p: *mut c_char) -> c_int {
    let mut len = 0;
    unsigned long sizes[3];
pub static mut total_size: usize = 0;
    let mut i = 0;
    if (!p) {
    return -EINVAL;
    }
    len = strlen(p);
    if (!len) {
    return -EINVAL;
    }
// parse nn%
    if (p[len - 1] == '%') {
// unsigned int max is 4,294,967,295, 10 chars
    char s_scale[11] = {};
pub static mut ret: c_int = 0;
    if (len > ARRAY_SIZE!(s_scale)) {
    return -EINVAL;
    }
    memcpy(s_scale, p, len - 1);
    ret = kstrtouint(s_scale, 10, &scratch_scale);
    if (!ret) {
    pr_notice("scratch scale is %d%%\n", scratch_scale);
    }
    return ret;
    }
// parse ll[KMG],mm[KMG],nn[KMG]
    while (i < ARRAY_SIZE!(sizes)) {
    let mut endp = p;
    if (i > 0) {
    if (*p != ',') {
    return -EINVAL;
    }
    p += 1;
    }
    sizes[i] = memparse(p, &endp);
    if (endp == p) {
    return -EINVAL;
    }
    p = endp;
    total_size += sizes[i];
    }
    if (!total_size) {
    return -EINVAL;
    }
// The string should be fully consumed by now.
    if (*p) {
    return -EINVAL;
    }
    scratch_size_lowmem = sizes[0];
    scratch_size_global = sizes[1];
    scratch_size_pernode = sizes[2];
    scratch_scale = 0;
    pr_notice("scratch areas: lowmem: %lluMiB global: %lluMiB pernode: %lldMiB\n",
    (u64)(scratch_size_lowmem >> 20),
    (u64)(scratch_size_global >> 20),
    (u64)(scratch_size_pernode >> 20));
    return 0;
    }
    early_param!("kho_scratch", kho_parse_scratch_size);
#[no_mangle]
unsafe extern "C" fn scratch_size_update()  {
//
// If fixed sizes are not provided via command line, calculate them now.
// Remove HugeTLB allocations from it because they never get allocated
// from scratch.
//
    if (scratch_scale) {
    let mut size;
    size = memblock_reserved_kern_size(ARCH_LOW_ADDRESS_LIMIT,
    NUMA_NO_NODE);
    size -= memblock_reserved_hugetlb_size(ARCH_LOW_ADDRESS_LIMIT,
    NUMA_NO_NODE);
    size = size * scratch_scale / 100;
    scratch_size_lowmem = size;
    size = memblock_reserved_kern_size(MEMBLOCK_ALLOC_ANYWHERE,
    NUMA_NO_NODE);
    size -= memblock_reserved_hugetlb_size(MEMBLOCK_ALLOC_ANYWHERE,
    NUMA_NO_NODE);
    size = size * scratch_scale / 100 - scratch_size_lowmem;
    scratch_size_global = size;
    }
//
// Scratch areas are released as MIGRATE_CMA. Round them up to the right
// size.
//
    scratch_size_lowmem = round_up(scratch_size_lowmem, SCRATCH_ALIGNMENT_BYTES);
    scratch_size_global = round_up(scratch_size_global, SCRATCH_ALIGNMENT_BYTES);
    }
#[no_mangle]
unsafe extern "C" fn scratch_size_node(nid: c_int) -> phys_addr_t __init {
    let mut size;
    if (scratch_scale) {
    size = memblock_reserved_kern_size(MEMBLOCK_ALLOC_ANYWHERE,
    nid);
// Do not count HugeTLB pages.
    size -= memblock_reserved_hugetlb_size(MEMBLOCK_ALLOC_ANYWHERE,
    nid);
    size = size * scratch_scale / 100;
    } else {
    size = scratch_size_pernode;
    }
    return round_up(size, SCRATCH_ALIGNMENT_BYTES);
    }
#[no_mangle]
pub unsafe extern "C" fn kho_scratch_overlap(phys: phys_addr_t, size: usize) -> bool {
    phys_addr_t scratch_start, scratch_end;
    let mut i = 0;
    while (i < kho_scratch_cnt) {
    scratch_start = kho_scratch[i].addr;
    scratch_end = kho_scratch[i].addr + kho_scratch[i].size;
    if (phys < scratch_end && (phys + size) > scratch_start) {
    return true;
    }
    }
    return false;
    }
//
// kho_reserve_scratch - Reserve a contiguous chunk of memory for kexec
//
// With KHO we can preserve arbitrary pages in the system. To ensure we still
// have a large contiguous region of memory when we search the physical address
// space for target memory, let's make sure we always have a large CMA region
// active. This CMA region will only be used for movable pages which are not a
// problem for us during KHO because we can just move them somewhere else.
//
#[no_mangle]
unsafe extern "C" fn kho_reserve_scratch()  {
    phys_addr_t addr, size;
    int nid, i = 0;
    if (!kho_enable) {
    return;
    }
    scratch_size_update();
// FIXME: deal with node hot-plug/remove
    kho_scratch_cnt = nodes_weight(node_states[N_MEMORY]) + 2;
    size = kho_scratch_cnt * sizeof!(*kho_scratch);
    kho_scratch = memblock_alloc(size, PAGE_SIZE);
    if (!kho_scratch) {
    pr_err!("Failed to reserve scratch array\n");
// goto;
    }
//
// reserve scratch area in low memory for lowmem allocations in the
// next kernel
//
    size = scratch_size_lowmem;
    addr = memblock_phys_alloc_range(size, SCRATCH_ALIGNMENT_BYTES, 0,
    ARCH_LOW_ADDRESS_LIMIT);
    if (!addr) {
    pr_err!("Failed to reserve lowmem scratch buffer\n");
// goto;
    }
    kho_scratch[i].addr = addr;
    kho_scratch[i].size = size;
    i += 1;
// reserve large contiguous area for allocations without nid
    size = scratch_size_global;
    addr = memblock_phys_alloc(size, SCRATCH_ALIGNMENT_BYTES);
    if (!addr) {
    pr_err!("Failed to reserve global scratch buffer\n");
// goto;
    }
    kho_scratch[i].addr = addr;
    kho_scratch[i].size = size;
    i += 1;
//
// Loop over nodes that have both memory and are online. Skip
// memoryless nodes, as we can not allocate scratch areas there.
//
    for_each_node_state(nid, N_MEMORY) {
    size = scratch_size_node(nid);
    addr = memblock_alloc_range_nid(size, SCRATCH_ALIGNMENT_BYTES,
    0, MEMBLOCK_ALLOC_ACCESSIBLE,
    nid, true);
    if (!addr) {
    pr_err!("Failed to reserve nid %d scratch buffer\n", nid);
// goto;
    }
    kho_scratch[i].addr = addr;
    kho_scratch[i].size = size;
    i += 1;
    }
    return;
// label;
    for (i -= 1; i >= 0; i--) {
    memblock_phys_free(kho_scratch[i].addr, kho_scratch[i].size);
    }
// label;
    memblock_free(kho_scratch, kho_scratch_cnt * sizeof!(*kho_scratch));
// label;
    pr_warn!("Failed to reserve scratch area, disabling kexec handover\n");
    kho_enable = false;
    }
//
// Look for free blocks of 1G. This is a heuristic chosen to work efficiently
// with large systems with hundreds of gigabytes of memory. It will work poorly
// on smaller systems. The algorithm itself doesn't depend on the actual value,
// so it can be changed to a different heuristic later if needed.
//

// Called for the KHO preserved memory radix tree.
#[no_mangle]
unsafe extern "C" fn kho_ext_walk_leaf(key: c_ulong, data: *mut c_void) -> c_int {
    let mut busy_blocks = data;
    phys_addr_t start, end;
    let mut order = 0;
    let mut err = 0;
//
// The key is from the KHO preserved memory radix tree. It is decoded to
// a physical address of a preservation and its order.
//
    start = kho_decode_radix_key(key, &order);
    end = start + (1UL << (order + PAGE_SHIFT));
    while (start < end) {
    err = kho_radix_add_key(busy_blocks, start >> KHO_SCRATCH_EXT_BLKSHIFT);
    if (err) {
    return err;
    }
    start += (1UL << KHO_SCRATCH_EXT_BLKSHIFT);
    }
    return 0;
    }
// Called for the KHO preserved memory radix tree.
#[no_mangle]
unsafe extern "C" fn kho_ext_walk_node(phys: phys_addr_t, data: *mut c_void) -> c_int {
    let mut busy_blocks = data;
    return kho_radix_add_key(busy_blocks, phys >> KHO_SCRATCH_EXT_BLKSHIFT);
    }
// Called for the busy block radix tree.
#[no_mangle]
unsafe extern "C" fn kho_ext_mark_scratch(key: c_ulong, data: *mut c_void) -> c_int {
    let mut prev_end = data;
pub static mut start: phys_addr_t = 0;
    let mut err = 0;
    if (start > *prev_end) {
    err = memblock_mark_kho_scratch(*prev_end, start - *prev_end);
    if (err) {
    return err;
    }
    }
// prev_end = start + (1UL << KHO_SCRATCH_EXT_BLKSHIFT);
    return 0;
    }
//
// kho_extend_scratch - Extend the scratch regions
//
// The KHO preserved memory radix tree mixes both physical address and order
// into a single key. This makes it hard to look for free ranges directly. This
// function first walks the radix tree and digests it down into another radix
// tree, whose keys identify blocks of size KHO_SCRATCH_EXT_BLKSIZE which
// contain preserved memory.
//
// Then it walks the digested radix tree and marks everything that doesn't have
// preserved memory as scratch.
//
// NOTE: This function allocates memory so it should be called when scratch has
// available space.
//
// NOTE: The pages of the KHO preserved memory radix tree tables are not marked
// as preserved in the preserved memory tree. But they are expected to remain
// untouched until the tree is fully parsed. So this function also considers
// them to be "preserved memory" and marks their blocks as busy.
//
// NOTE: efi_init()::reserve_regions() removes all regions except
// MEMBLOCK_KHO_SCRATCH. This function adds such regions but they are not KHO
// scratch memory, so they should not be removed. This function should always be
// called after reserve_regions().
//
#[no_mangle]
unsafe extern "C" fn kho_extend_scratch()  {
pub static mut kho_radix_walk_cb: usize = 0;
pub static mut kho_radix_walk_cb: usize = 0;
pub static mut busy_radix_class: usize = 0;
pub static mut busy_blocks: usize = 0;
pub static mut prev_end: phys_addr_t = 0;
pub static mut err: c_int = 0;
    err = kho_radix_init_tree(&busy_blocks, core::ptr::null_mut());
    if (err) {
// goto;
    }
//
// The walk of kho_in.radix_tree adds keys to busy_blocks. The walk
// takes the kho_in radix tree lock and adding the key takes busy_blocks
// lock. Since both are struct kho_radix_tree and share the same lock
// class, lockdep gets confused. Set a different class for
// busy_blocks.lock to make lockdep happy.
//
    lockdep_set_class(&busy_blocks.lock, &busy_radix_class);
// Walk the KHO radix tree to find busy blocks.
    err = kho_radix_walk_tree(&kho_in.radix_tree, &kho_cb, &busy_blocks);
    if (err) {
// goto;
    }
// Walk the busy blocks and mark everything between keys as scratch.
    err = kho_radix_walk_tree(&busy_blocks, &ext_cb, &prev_end);
    if (err) {
// goto;
    }
// Mark everything from last busy block to end of DRAM.
    if (prev_end < memblock_end_of_DRAM()) {
    err = memblock_mark_kho_scratch(prev_end, memblock_end_of_DRAM() - prev_end);
    }
// fallthrough
// label;
    kho_radix_destroy_tree(&busy_blocks);
// label;
    if (err) {
    pr_err!("Failed to extend scratch: %pe\n", ERR_PTR(err));
    }
    }
//
// kho_add_subtree - record the physical address of a sub blob in KHO root tree.
// @name: name of the sub tree.
// @blob: the sub tree blob.
// @size: size of the blob in bytes.
//
// Creates a new child node named @name in KHO root FDT and records
// the physical address of @blob. The pages of @blob must also be preserved
// by KHO for the new kernel to retrieve it after kexec.
//
// A debugfs blob entry is also created at
// ``/sys/kernel/debug/kho/out/sub_fdts/@name`` when kernel is configured with
// CONFIG_KEXEC_HANDOVER_DEBUGFS
//
// Return: 0 on success, error code on failure
//
#[no_mangle]
pub unsafe extern "C" fn kho_add_subtree(name: *const c_char, blob: *mut c_void, size: usize) -> c_int {
pub static mut phys: phys_addr_t = 0;
    let mut root_fdt = kho_out.fdt;
pub static mut size_u64: u64 = 0;
pub static mut err: c_int = 0;
    let mut off = 0;
    let mut fdt_err = 0;
    guard(mutex)(&kho_out.lock);
    fdt_err = fdt_open_into(root_fdt, root_fdt, PAGE_SIZE);
    if (fdt_err < 0) {
    return err;
    }
    off = fdt_add_subnode(root_fdt, 0, name);
    if (off < 0) {
    if (off == -FDT_ERR_EXISTS) {
    err = -EEXIST;
    }
// goto;
    }
    fdt_err = fdt_setprop(root_fdt, off, KHO_SUB_TREE_PROP_NAME,
    &phys, sizeof!(phys));
    if (fdt_err < 0) {
// goto;
    }
    fdt_err = fdt_setprop(root_fdt, off, KHO_SUB_TREE_SIZE_PROP_NAME,
    &size_u64, sizeof!(size_u64));
    if (fdt_err < 0) {
// goto;
    }
    WARN_ON_ONCE!(kho_debugfs_blob_add(&kho_out.dbg, name, blob,
    size, false));
    err = 0;
// goto;
// label;
    fdt_del_node(root_fdt, off);
// label;
    fdt_pack(root_fdt);
    return err;
    }
    EXPORT_SYMBOL_GPL(kho_add_subtree);
#[no_mangle]
pub unsafe extern "C" fn kho_remove_subtree(blob: *mut c_void) {
pub static mut target_phys: phys_addr_t = 0;
    let mut root_fdt = kho_out.fdt;
    let mut off = 0;
    let mut err = 0;
    guard(mutex)(&kho_out.lock);
    err = fdt_open_into(root_fdt, root_fdt, PAGE_SIZE);
    if (err < 0) {
    return;
    }
    for (off = fdt_first_subnode(root_fdt, 0); off >= 0;
    off = fdt_next_subnode(root_fdt, off)) {
pub static mut val: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    val = fdt_getprop(root_fdt, off, KHO_SUB_TREE_PROP_NAME, &len);
    if (!val || len != sizeof!(phys_addr_t)) {
    continue;
    }
    if ((phys_addr_t)*val == target_phys) {
    fdt_del_node(root_fdt, off);
    kho_debugfs_blob_remove(&kho_out.dbg, blob);
    break;
    }
    }
    fdt_pack(root_fdt);
    }
    EXPORT_SYMBOL_GPL(kho_remove_subtree);
//
// kho_preserve_folio - preserve a folio across kexec.
// @folio: folio to preserve.
//
// Instructs KHO to preserve the whole folio across kexec. The order
// will be preserved as well.
//
// Return: 0 on success, error code on failure
//
#[no_mangle]
pub unsafe extern "C" fn kho_preserve_folio(folio: *mut folio) -> c_int {
    let mut tree = &kho_out.radix_tree;
pub static mut pfn: c_ulong = 0;
pub static mut order: c_uint = 0;
    if (IS_ENABLED!(CONFIG_KEXEC_HANDOVER_DEBUG) &&
    WARN_ON!(kho_scratch_overlap(pfn << PAGE_SHIFT, PAGE_SIZE << order))) {
    return -EINVAL;
    }
    return kho_radix_add_key(tree, kho_encode_radix_key(PFN_PHYS(pfn),
    order));
    }
    EXPORT_SYMBOL_GPL(kho_preserve_folio);
//
// kho_unpreserve_folio - unpreserve a folio.
// @folio: folio to unpreserve.
//
// Instructs KHO to unpreserve a folio that was preserved by
// kho_preserve_folio() before. The provided @folio (pfn and order)
// must exactly match a previously preserved folio.
//
#[no_mangle]
pub unsafe extern "C" fn kho_unpreserve_folio(folio: *mut folio) {
    let mut tree = &kho_out.radix_tree;
pub static mut pfn: c_ulong = 0;
pub static mut order: c_uint = 0;
    kho_radix_del_key(tree, kho_encode_radix_key(PFN_PHYS(pfn), order));
    }
    EXPORT_SYMBOL_GPL(kho_unpreserve_folio);
#[no_mangle]
pub unsafe extern "C" fn __kho_preserve_pages_order(start_pfn: c_ulong, end_pfn: c_ulong) -> c_uint {
    let mut order = min(count_trailing_zeros(start_pfn),
    ilog2(end_pfn - start_pfn));
//
// Make sure all the pages in a single preservation are in the same NUMA
// node. The restore machinery can not cope with a preservation spanning
// multiple NUMA nodes.
//
    while (pfn_to_nid(start_pfn) != pfn_to_nid(start_pfn + (1UL << order) - 1)) {
    order -= 1;
    }
    return order;
    }
#[no_mangle]
pub unsafe extern "C" fn __kho_unpreserve(tree: *mut kho_radix_tree, pfn: c_ulong, end_pfn: c_ulong) {
    let mut order = 0;
    while (pfn < end_pfn) {
    order = __kho_preserve_pages_order(pfn, end_pfn);
    kho_radix_del_key(tree, kho_encode_radix_key(PFN_PHYS(pfn),
    order));
    pfn += 1 << order;
    }
    }
//
// kho_preserve_pages - preserve contiguous pages across kexec
// @page: first page in the list.
// @nr_pages: number of pages.
//
// Preserve a contiguous list of order 0 pages. Must be restored using
// kho_restore_pages() to ensure the pages are restored properly as order 0.
//
// Return: 0 on success, error code on failure
//
#[no_mangle]
pub unsafe extern "C" fn kho_preserve_pages(page: *mut page, nr_pages: c_ulong) -> c_int {
    let mut tree = &kho_out.radix_tree;
pub static mut start_pfn: c_ulong = 0;
pub static mut end_pfn: c_ulong = 0;
pub static mut pfn: c_ulong = 0;
pub static mut failed_pfn: c_ulong = 0;
pub static mut err: c_int = 0;
    if (IS_ENABLED!(CONFIG_KEXEC_HANDOVER_DEBUG) &&
    WARN_ON!(kho_scratch_overlap(start_pfn << PAGE_SHIFT,
    nr_pages << PAGE_SHIFT))) {
    return -EINVAL;
    }
    while (pfn < end_pfn) {
pub static mut order: c_uint = 0;
    err = kho_radix_add_key(tree, kho_encode_radix_key(PFN_PHYS(pfn),
    order));
    if (err) {
    failed_pfn = pfn;
    break;
    }
    pfn += 1 << order;
    }
    if (err) {
    __kho_unpreserve(tree, start_pfn, failed_pfn);
    }
    return err;
    }
    EXPORT_SYMBOL_GPL(kho_preserve_pages);
//
// kho_unpreserve_pages - unpreserve contiguous pages.
// @page: first page in the list.
// @nr_pages: number of pages.
//
// Instructs KHO to unpreserve @nr_pages contiguous pages starting from @page.
// This must be called with the same @page and @nr_pages as the corresponding
// kho_preserve_pages() call. Unpreserving arbitrary sub-ranges of larger
// preserved blocks is not supported.
//
#[no_mangle]
pub unsafe extern "C" fn kho_unpreserve_pages(page: *mut page, nr_pages: c_ulong) {
    let mut tree = &kho_out.radix_tree;
pub static mut start_pfn: c_ulong = 0;
pub static mut end_pfn: c_ulong = 0;
    __kho_unpreserve(tree, start_pfn, end_pfn);
    }
    EXPORT_SYMBOL_GPL(kho_unpreserve_pages);
// vmalloc flags KHO supports

// KHO internal flags for vmalloc preservations
pub const KHO_VMALLOC_ALLOC: c_uint = 0x0001;
pub const KHO_VMALLOC_HUGE_VMAP: c_uint = 0x0002;
#[no_mangle]
unsafe extern "C" fn vmalloc_flags_to_kho(vm_flags: c_uint) -> c_ushort {
pub static mut kho_flags: c_ushort = 0;
    if (vm_flags & VM_ALLOC) {
    kho_flags |= KHO_VMALLOC_ALLOC;
    }
    if (vm_flags & VM_ALLOW_HUGE_VMAP) {
    kho_flags |= KHO_VMALLOC_HUGE_VMAP;
    }
    return kho_flags;
    }
#[no_mangle]
unsafe extern "C" fn kho_flags_to_vmalloc(kho_flags: c_ushort) -> c_uint {
pub static mut vm_flags: c_uint = 0;
    if (kho_flags & KHO_VMALLOC_ALLOC) {
    vm_flags |= VM_ALLOC;
    }
    if (kho_flags & KHO_VMALLOC_HUGE_VMAP) {
    vm_flags |= VM_ALLOW_HUGE_VMAP;
    }
    return vm_flags;
    }
#[no_mangle]
pub unsafe extern "C" fn new_vmalloc_chunk(cur: *mut kho_vmalloc_chunk) -> *mut c_void {
pub static mut chunk: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    chunk = get_zeroed_page(GFP_KERNEL);
    if (!chunk) {
    return core::ptr::null_mut();
    }
    err = kho_preserve_pages(virt_to_page(chunk), 1);
    if (err) {
// goto;
    }
    if (cur) {
    KHOSER_STORE_PTR(cur.hdr.next, chunk);
    }
    return chunk;
// label;
    free_page((unsigned long)chunk);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn kho_vmalloc_unpreserve_chunk(chunk: *mut kho_vmalloc_chunk, order: c_ushort) {
    let mut tree = &kho_out.radix_tree;
pub static mut pfn: c_ulong = 0;
    __kho_unpreserve(tree, pfn, pfn + 1);
    while (i < ARRAY_SIZE!(chunk.phys) && chunk.phys[i]) {
    pfn = PHYS_PFN(chunk.phys[i]);
    __kho_unpreserve(tree, pfn, pfn + (1 << order));
    }
    }
//
// kho_preserve_vmalloc - preserve memory allocated with vmalloc() across kexec
// @ptr: pointer to the area in vmalloc address space
// @preservation: placeholder for preservation metadata
//
// Instructs KHO to preserve the area in vmalloc address space at @ptr. The
// physical pages mapped at @ptr will be preserved and on successful return
// @preservation will hold the physical address of a structure that describes
// the preservation.
//
// NOTE: The memory allocated with vmalloc_node() variants cannot be reliably
// restored on the same node
//
// Return: 0 on success, error code on failure
//
#[no_mangle]
pub unsafe extern "C" fn kho_preserve_vmalloc(ptr: *mut c_void, preservation: *mut kho_vmalloc) -> c_int {
pub static mut chunk: *mut c_void = core::ptr::null_mut();
    let mut vm = find_vm_area(ptr);
    let mut order = 0;
    let mut flags = 0;
    let mut nr_contig_pages = 0;
pub static mut idx: c_uint = 0;
    let mut err = 0;
    if (!vm) {
    return -EINVAL;
    }
    if (vm.flags & ~KHO_VMALLOC_SUPPORTED_FLAGS) {
    return -EOPNOTSUPP;
    }
    flags = vmalloc_flags_to_kho(vm.flags);
    order = get_vm_area_page_order(vm);
    chunk = new_vmalloc_chunk(core::ptr::null_mut());
    if (!chunk) {
    return -ENOMEM;
    }
    KHOSER_STORE_PTR(preservation.first, chunk);
    nr_contig_pages = (1 << order);
    while (i < vm.nr_pages) {
pub static mut phys: phys_addr_t = 0;
    err = kho_preserve_pages(vm.pages[i], nr_contig_pages);
    if (err) {
// goto;
    }
    chunk.phys[idx++] = phys;
    if (idx == ARRAY_SIZE!(chunk.phys)) {
    chunk = new_vmalloc_chunk(chunk);
    if (!chunk) {
    err = -ENOMEM;
// goto;
    }
    idx = 0;
    }
    }
    preservation.total_pages = vm.nr_pages;
    preservation.flags = flags;
    preservation.order = order;
    return 0;
// label;
    kho_unpreserve_vmalloc(preservation);
    return err;
    }
    EXPORT_SYMBOL_GPL(kho_preserve_vmalloc);
//
// kho_unpreserve_vmalloc - unpreserve memory allocated with vmalloc()
// @preservation: preservation metadata returned by kho_preserve_vmalloc()
//
// Instructs KHO to unpreserve the area in vmalloc address space that was
// previously preserved with kho_preserve_vmalloc().
//
#[no_mangle]
pub unsafe extern "C" fn kho_unpreserve_vmalloc(preservation: *mut kho_vmalloc) {
    let mut chunk = KHOSER_LOAD_PTR(preservation.first);
    while (chunk) {
    let mut tmp = chunk;
    kho_vmalloc_unpreserve_chunk(chunk, preservation.order);
    chunk = KHOSER_LOAD_PTR(chunk.hdr.next);
    free_page((unsigned long)tmp);
    }
    }
    EXPORT_SYMBOL_GPL(kho_unpreserve_vmalloc);
//
// kho_restore_vmalloc - recreates and populates an area in vmalloc address
// space from the preserved memory.
// @preservation: preservation metadata.
//
// Recreates an area in vmalloc address space and populates it with memory that
// was preserved using kho_preserve_vmalloc().
//
// Return: pointer to the area in the vmalloc address space, NULL on failure.
//
#[no_mangle]
pub unsafe extern "C" fn kho_restore_vmalloc(preservation: *mut kho_vmalloc) -> *mut c_void {
    let mut chunk = KHOSER_LOAD_PTR(preservation.first);
pub static mut kasan_flags: kasan_vmalloc_flags_t = 0;
    let mut align = 0;
    let mut order = 0;
    let mut shift = 0;
    let mut vm_flags = 0;
    unsigned long total_pages, contig_pages;
    unsigned long addr, size;
pub static mut area: *mut c_void = core::ptr::null_mut();
pub static mut pages: *mut c_void = core::ptr::null_mut();
pub static mut idx: c_uint = 0;
    let mut err = 0;
    vm_flags = kho_flags_to_vmalloc(preservation.flags);
    if (vm_flags & ~KHO_VMALLOC_SUPPORTED_FLAGS) {
    return core::ptr::null_mut();
    }
    total_pages = preservation.total_pages;
    pages = kvmalloc_objs(*pages, total_pages);
    if (!pages) {
    return core::ptr::null_mut();
    }
    order = preservation.order;
    contig_pages = (1 << order);
    shift = PAGE_SHIFT + order;
    align = 1 << shift;
    while (chunk) {
pub static mut page: *mut c_void = core::ptr::null_mut();
    while (i < ARRAY_SIZE!(chunk.phys) && chunk.phys[i]) {
pub static mut phys: phys_addr_t = 0;
    if (idx + contig_pages > total_pages) {
// goto;
    }
    page = kho_restore_pages(phys, contig_pages);
    if (!page) {
// goto;
    }
    for (int j = 0; j < contig_pages; j++) {
    pages[idx++] = page + j;
    }
    phys += contig_pages * PAGE_SIZE;
    }
    page = kho_restore_pages(virt_to_phys(chunk), 1);
    if (!page) {
// goto;
    }
    chunk = KHOSER_LOAD_PTR(chunk.hdr.next);
    __free_page(page);
    }
    if (idx != total_pages) {
// goto;
    }
    area = __get_vm_area_node(total_pages * PAGE_SIZE, align, shift,
    vm_flags | VM_UNINITIALIZED,
    VMALLOC_START, VMALLOC_END,
    NUMA_NO_NODE, GFP_KERNEL,
    __builtin_return_address(0));
    if (!area) {
// goto;
    }
    addr = (unsigned long)area.addr;
    size = get_vm_area_size(area);
    err = vmap_pages_range(addr, addr + size, PAGE_KERNEL, pages, shift);
    if (err) {
// goto;
    }
    area.nr_pages = total_pages;
    area.pages = pages;
    if (vm_flags & VM_ALLOC) {
    kasan_flags |= KASAN_VMALLOC_VM_ALLOC;
    }
    area.addr = kasan_unpoison_vmalloc(area.addr, total_pages * PAGE_SIZE,
    kasan_flags);
    clear_vm_uninitialized_flag(area);
    return area.addr;
// label;
    free_vm_area(area);
// label;
    kvfree(pages);
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(kho_restore_vmalloc);
//
// kho_alloc_preserve - Allocate, zero, and preserve memory.
// @size: The number of bytes to allocate.
//
// Allocates a physically contiguous block of zeroed pages that is large
// enough to hold @size bytes. The allocated memory is then registered with
// KHO for preservation across a kexec.
//
// Note: The actual allocated size will be rounded up to the nearest
// power-of-two page boundary.
//
// @return A virtual pointer to the allocated and preserved memory on success,
// or an ERR_PTR() encoded error on failure.
//
#[no_mangle]
pub unsafe extern "C" fn kho_alloc_preserve(size: size_t) -> *mut c_void {
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut order = 0;
    let mut ret = 0;
    if (!size) {
    return ERR_PTR(-EINVAL);
    }
    order = get_order(size);
    if (order > MAX_PAGE_ORDER) {
    return ERR_PTR(-E2BIG);
    }
    folio = folio_alloc(GFP_KERNEL | __GFP_ZERO, order);
    if (!folio) {
    return ERR_PTR(-ENOMEM);
    }
    ret = kho_preserve_folio(folio);
    if (ret) {
    folio_put(folio);
    return ERR_PTR(ret);
    }
    return folio_address(folio);
    }
    EXPORT_SYMBOL_GPL(kho_alloc_preserve);
//
// kho_unpreserve_free - Unpreserve and free memory.
// @mem:  Pointer to the memory allocated by kho_alloc_preserve().
//
// Unregisters the memory from KHO preservation and frees the underlying
// pages back to the system. This function should be called to clean up
// memory allocated with kho_alloc_preserve().
//
#[no_mangle]
pub unsafe extern "C" fn kho_unpreserve_free(mem: *mut c_void) {
pub static mut folio: *mut c_void = core::ptr::null_mut();
    if (!mem) {
    return;
    }
    folio = virt_to_folio(mem);
    kho_unpreserve_folio(folio);
    folio_put(folio);
    }
    EXPORT_SYMBOL_GPL(kho_unpreserve_free);
//
// kho_restore_free - Restore and free memory after kexec.
// @mem:  Pointer to the memory (in the new kernel's address space)
// that was allocated by the old kernel.
//
// This function is intended to be called in the new kernel (post-kexec)
// to take ownership of and free a memory region that was preserved by the
// old kernel using kho_alloc_preserve().
//
// It first restores the pages from KHO (using their physical address)
// and then frees the pages back to the new kernel's page allocator.
//
#[no_mangle]
pub unsafe extern "C" fn kho_restore_free(mem: *mut c_void) {
pub static mut folio: *mut c_void = core::ptr::null_mut();
    if (!mem) {
    return;
    }
    folio = kho_restore_folio(__pa(mem));
    if (!WARN_ON!(!folio)) {
    folio_put(folio);
    }
    }
    EXPORT_SYMBOL_GPL(kho_restore_free);
//
// is_kho_boot - check if current kernel was booted via KHO-enabled
// kexec
//
// This function checks if the current kernel was loaded through a kexec
// operation with KHO enabled, by verifying that a valid KHO FDT
// was passed.
//
// Note: This function returns reliable results only after
// kho_populate() has been called during early boot. Before that,
// it may return false even if KHO data is present.
//
// Return: true if booted via KHO-enabled kexec, false otherwise
//
#[no_mangle]
pub unsafe extern "C" fn is_kho_boot() -> bool {
    return !!kho_get_fdt();
    }
    EXPORT_SYMBOL_GPL(is_kho_boot);
//
// kho_retrieve_subtree - retrieve a preserved sub blob by its name.
// @name: the name of the sub blob passed to kho_add_subtree().
// @phys: if found, the physical address of the sub blob is stored in @phys.
// @size: if not NULL and found, the size of the sub blob is stored in @size.
//
// Retrieve a preserved sub blob named @name and store its physical
// address in @phys and optionally its size in @size.
//
// Return: 0 on success, error code on failure
//
#[no_mangle]
pub unsafe extern "C" fn kho_retrieve_subtree(name: *const c_char, phys: *mut phys_addr_t, size: *mut usize) -> c_int {
    let mut fdt = kho_get_fdt();
pub static mut val: *mut c_void = core::ptr::null_mut();
    let mut offset = 0;
    let mut len = 0;
    if (!fdt) {
    return -ENOENT;
    }
    if (!phys) {
    return -EINVAL;
    }
    offset = fdt_subnode_offset(fdt, 0, name);
    if (offset < 0) {
    return -ENOENT;
    }
    val = fdt_getprop(fdt, offset, KHO_SUB_TREE_PROP_NAME, &len);
    if (!val || len != sizeof!(*val)) {
    return -EINVAL;
    }
// phys = (phys_addr_t)*val;
    val = fdt_getprop(fdt, offset, KHO_SUB_TREE_SIZE_PROP_NAME, &len);
    if (!val || len != sizeof!(*val)) {
    pr_warn!("broken KHO subnode '%s': missing or invalid blob-size property\n",
    name);
    return -EINVAL;
    }
    if (size) {
// size = (size_t)*val;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(kho_retrieve_subtree);
#[no_mangle]
unsafe extern "C" fn kho_mem_retrieve()  {
pub static mut kho_radix_walk_cb: usize = 0;
    if (kho_radix_walk_tree(&kho_in.radix_tree, &cb, core::ptr::null_mut())) {
// goto;
    }
    return;
// label;
//
// Failed to initialize preserved memory. Clear FDT and radix so KHO
// users don't treat it as a KHO boot.
//
    kho_in.fdt_phys = 0;
    kho_in.radix_tree.root = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn kho_out_fdt_setup() -> __init int {
    let mut tree = &kho_out.radix_tree;
    let mut root = kho_out.fdt;
    let mut preserved_mem_tree_pa = 0;
    let mut err = 0;
    err = fdt_create(root, PAGE_SIZE);
    err |= fdt_finish_reservemap(root);
    err |= fdt_begin_node(root, "");
    err |= fdt_property_string(root, "compatible", KHO_FDT_COMPATIBLE);
    preserved_mem_tree_pa = virt_to_phys(tree.root);
    err |= fdt_property(root, KHO_FDT_MEMORY_MAP_PROP_NAME,
    &preserved_mem_tree_pa,
    sizeof!(preserved_mem_tree_pa));
    err |= fdt_end_node(root);
    err |= fdt_finish(root);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn kho_in_kexec_metadata()  {
pub static mut metadata: *mut c_void = core::ptr::null_mut();
    let mut metadata_phys;
    let mut blob_size = 0;
    let mut err = 0;
    err = kho_retrieve_subtree(KHO_METADATA_NODE_NAME, &metadata_phys,
    &blob_size);
    if (err) {
// This is fine, previous kernel didn't export metadata
    return;
    }
// Check that, at least, "version" is present
    if (blob_size < sizeof!(u32)) {
    pr_warn!("kexec-metadata blob too small (%zu bytes)\n",
    blob_size);
    return;
    }
    metadata = phys_to_virt(metadata_phys);
    if (metadata.version != KHO_KEXEC_METADATA_VERSION) {
    pr_warn!("kexec-metadata version %u not supported (expected %u)\n",
    metadata.version, KHO_KEXEC_METADATA_VERSION);
    return;
    }
    if (blob_size < sizeof!(*metadata)) {
    pr_warn!("kexec-metadata blob too small for v%u (%zu < %zu)\n",
    metadata.version, blob_size, sizeof!(*metadata));
    return;
    }
//
// Copy data to the kernel structure that will persist during
// kernel lifetime.
//
    kho_in.kexec_count = metadata.kexec_count;
    strscpy(kho_in.previous_release, metadata.previous_release,
    sizeof!(kho_in.previous_release));
    pr_info!("exec from: %s (count %u)\n",
    kho_in.previous_release, kho_in.kexec_count);
    }
//
// Create kexec metadata to pass kernel version and boot count to the
// next kernel. This keeps the core KHO ABI minimal and allows the
// metadata format to evolve independently.
//
#[no_mangle]
unsafe extern "C" fn kho_out_kexec_metadata() -> __init int {
pub static mut metadata: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    metadata = kho_alloc_preserve(sizeof!(*metadata));
    if (IS_ERR(metadata)) {
    return PTR_ERR(metadata);
    }
    metadata.version = KHO_KEXEC_METADATA_VERSION;
    strscpy(metadata.previous_release, init_uts_ns.name.release,
    sizeof!(metadata.previous_release));
// kho_in.kexec_count is set to 0 on cold boot
    metadata.kexec_count = kho_in.kexec_count + 1;
    err = kho_add_subtree(KHO_METADATA_NODE_NAME, metadata,
    sizeof!(*metadata));
    if (err) {
    kho_unpreserve_free(metadata);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn kho_kexec_metadata_init(fdt: *const c_void) -> c_int {
    let mut err = 0;
    if (fdt) {
    kho_in_kexec_metadata();
    }
// Populate kexec metadata for the possible next kexec
    err = kho_out_kexec_metadata();
    if (err) {
    pr_warn!("failed to initialize kexec-metadata subtree: %d\n",
    err);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn kho_init() -> __init int {
    let mut tree = &kho_out.radix_tree;
    let mut fdt = kho_get_fdt();
pub static mut err: c_int = 0;
    if (!kho_enable) {
    return 0;
    }
    err = kho_radix_init_tree(tree, core::ptr::null_mut());
    if (err) {
// goto;
    }
    kho_out.fdt = kho_alloc_preserve(PAGE_SIZE);
    if (IS_ERR(kho_out.fdt)) {
    err = PTR_ERR(kho_out.fdt);
// goto;
    }
    err = kho_debugfs_init();
    if (err) {
// goto;
    }
    err = kho_out_debugfs_init(&kho_out.dbg);
    if (err) {
// goto;
    }
    err = kho_out_fdt_setup();
    if (err) {
// goto;
    }
    err = kho_kexec_metadata_init(fdt);
    if (err) {
// goto;
    }
    if (fdt) {
    kho_in_debugfs_init(&kho_in.dbg, fdt);
    return 0;
    }
    while (i < kho_scratch_cnt) {
pub static mut base_pfn: c_ulong = 0;
pub static mut count: c_ulong = 0;
    let mut pfn = 0;
//
// When debug_pagealloc is enabled, __free_pages() clears the
// corresponding PRESENT bit in the kernel page table.
// Subsequent kmemleak scans of these pages cause the
// non-PRESENT page faults.
// Mark scratch areas with kmemleak_ignore_phys() to exclude
// them from kmemleak scanning.
//
    kmemleak_ignore_phys(kho_scratch[i].addr);
    for (pfn = base_pfn; pfn < base_pfn + count;
    pfn += pageblock_nr_pages) {
    init_cma_reserved_pageblock(pfn_to_page(pfn));
    }
    }
    WARN_ON_ONCE!(kho_debugfs_blob_add(&kho_out.dbg, "fdt",
    kho_out.fdt,
    fdt_totalsize(kho_out.fdt), true));
    return 0;
// label;
    kho_unpreserve_free(kho_out.fdt);
// label;
    kho_radix_destroy_tree(tree);
// label;
    kho_out.fdt = core::ptr::null_mut();
    while (i < kho_scratch_cnt) {
    let mut start = __va(kho_scratch[i].addr);
    let mut end = start + kho_scratch[i].size;
    free_reserved_area(start, end, -1, "");
    }
    kho_enable = false;
    return err;
    }
    fs_initcall!(kho_init);
#[no_mangle]
pub unsafe extern "C" fn kho_memory_init_early()  {
    let mut fdt = kho_get_fdt();
pub static mut mem_map: *mut c_void = core::ptr::null_mut();
    if (!is_kho_boot()) {
    return;
    }
//
// kho_get_mem_map() should always succeed. If it fails, kho_populate()
// catches that and never sets kho_in.scratch_phys, which stops memory
// retrieval.
//
    mem_map = kho_get_mem_map(fdt);
    if (WARN_ON!(!mem_map)) {
// goto;
    }
//
// kho_scratch_overlap() needs kho_scratch to be initialized. It
// is used by free_area_init() on KHO boots, so initialize it
// early.
//
    kho_scratch = phys_to_virt(kho_in.scratch_phys);
    if (kho_radix_init_tree(&kho_in.radix_tree, mem_map)) {
// goto;
    }
    kho_extend_scratch();
    return;
// label;
//
// Failed to initialize preserved memory radix tree. Clear FDT
// and scratch so KHO users don't treat it as a KHO boot.
//
    kho_in.fdt_phys = 0;
    kho_in.scratch_phys = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kho_memory_init()  {
    if (kho_in.scratch_phys) {
    kho_mem_retrieve();
    }
    else {
    kho_reserve_scratch();
    }
    }
    void __init kho_populate(phys_addr_t fdt_phys, u64 fdt_len,
    phys_addr_t scratch_phys, u64 scratch_len)
    {
pub static mut scratch_cnt: c_uint = 0;
    let mut scratch = core::ptr::null_mut();
    let mut mem_map_phys;
    let mut fdt = core::ptr::null_mut();
pub static mut populated: bool = false;
    let mut err = 0;
// Validate the input FDT
    fdt = early_memremap(fdt_phys, fdt_len);
    if (!fdt) {
    pr_warn!("setup: failed to memremap FDT (0x%llx)\n", fdt_phys);
// goto;
    }
    err = fdt_check_header(fdt);
    if (err) {
    pr_warn!("setup: handover FDT (0x%llx) is invalid: %d\n",
    fdt_phys, err);
// goto;
    }
    err = fdt_node_check_compatible(fdt, 0, KHO_FDT_COMPATIBLE);
    if (err) {
    pr_warn!("setup: handover FDT (0x%llx) is incompatible with '%s': %d\n",
    fdt_phys, KHO_FDT_COMPATIBLE, err);
// goto;
    }
    mem_map_phys = kho_get_mem_map_phys(fdt);
    if (!mem_map_phys) {
// goto;
    }
    scratch = early_memremap(scratch_phys, scratch_len);
    if (!scratch) {
    pr_warn!("setup: failed to memremap scratch (phys=0x%llx, len=%lld)\n",
    scratch_phys, scratch_len);
// goto;
    }
//
// We pass a safe contiguous blocks of memory to use for early boot
// purporses from the previous kernel so that we can resize the
// memblock array as needed.
//
    while (i < scratch_cnt) {
    let mut area = &scratch[i];
pub static mut size: u64 = 0;
    memblock_add(area.addr, size);
    err = memblock_mark_kho_scratch(area.addr, size);
    if (err) {
    pr_warn!("failed to mark the scratch region 0x%pa+0x%pa: %pe",
    &area.addr, &size, ERR_PTR(err));
// goto;
    }
    pr_debug!("Marked 0x%pa+0x%pa as scratch", &area.addr, &size);
    }
    memblock_reserve(scratch_phys, scratch_len);
//
// Now that we have a viable region of scratch memory, let's tell
// the memblocks allocator to only use that for any allocations.
// That way we ensure that nothing scribbles over in use data while
// we initialize the page tables which we will need to ingest all
// memory reservations from the previous kernel.
//
    memblock_set_kho_scratch_only();
    kho_in.fdt_phys = fdt_phys;
    kho_in.scratch_phys = scratch_phys;
    kho_scratch_cnt = scratch_cnt;
    populated = true;
    pr_info!("found kexec handover data.\n");
// label;
    early_memunmap(scratch, scratch_len);
// label;
    early_memunmap(fdt, fdt_len);
// label;
    if (!populated) {
    pr_warn!("disabling KHO revival\n");
    }
    }
// Helper functions for kexec_file_load
#[no_mangle]
pub unsafe extern "C" fn kho_fill_kimage(image: *mut kimage) -> c_int {
    let mut scratch_size = 0;
pub static mut err: c_int = 0;
pub static mut scratch: usize = 0;
    if (!kho_enable || image.type == KEXEC_TYPE_CRASH) {
    return 0;
    }
    image.kho.fdt = virt_to_phys(kho_out.fdt);
    scratch_size = sizeof!(*kho_scratch) * kho_scratch_cnt;
    scratch = (kexec_buf){
    .image = image,
    .buffer = kho_scratch,
    .bufsz = scratch_size,
    .mem = KEXEC_BUF_MEM_UNKNOWN,
    .memsz = scratch_size,
    .buf_align = SZ_64K, /* Makes it easier to map */
    .buf_max = ULONG_MAX,
    .top_down = true,
    };
    err = kexec_add_buffer(&scratch);
    if (err) {
    return err;
    }
    image.kho.scratch = &image.segment[image.nr_segments - 1];
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kho_walk_scratch(kbuf: *mut kexec_buf) -> c_int {
pub static mut ret: c_int = 0;
    let mut i = 0;
    while (i < kho_scratch_cnt) {
pub static mut resource: usize = 0;
// Try to fit the kimage into our KHO scratch region
    ret = func(&res, kbuf);
    if (ret) {
    break;
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn kho_locate_mem_hole(kbuf: *mut kexec_buf) -> c_int {
    let mut ret = 0;
    if (!kho_enable || kbuf.image.type == KEXEC_TYPE_CRASH) {
    return 1;
    }
    ret = kho_walk_scratch(kbuf, func);
pub static mut ret: return = 0;
    }