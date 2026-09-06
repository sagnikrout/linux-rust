//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/lpm_trie.c
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
// Longest prefix match list implementation
//
// Copyright (c) 2016,2017 Daniel Mack
// Copyright (c) 2016 David Herrmann
//

// Intermediate node

    let mut lpm_trie_node;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpm_trie_node {
    pub child: [*mut lpm_trie_node ; 2],
    pub prefixlen: u32,
    pub flags: u32,
    pub data: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpm_trie {
    pub map: bpf_map,
    pub root: *mut lpm_trie_node ,
    pub ma: bpf_mem_alloc,
    pub n_entries: usize,
    pub max_prefixlen: usize,
    pub data_size: usize,
    pub lock: rqspinlock_t,
}

// This trie implements a longest prefix match algorithm that can be used to
// match IP addresses to a stored set of ranges.
//
// Data stored in @data of struct bpf_lpm_key and struct lpm_trie_node is
// interpreted as big endian, so data[0] stores the most significant byte.
//
// Match ranges are internally stored in instances of struct lpm_trie_node
// which each contain their prefix length as well as two pointers that may
// lead to more nodes containing more specific matches. Each node also stores
// a value that is defined by and returned to userspace via the update_elem
// and lookup functions.
//
// For instance, let's start with a trie that was created with a prefix length
// of 32, so it can be used for IPv4 addresses, and one single element that
// matches 192.168.0.0/16. The data array would hence contain
// [0xc0, 0xa8, 0x00, 0x00] in big-endian notation. This documentation will
// stick to IP-address notation for readability though.
//
// As the trie is empty initially, the new node (1) will be places as root
// node, denoted as (R) in the example below. As there are no other node, both
// child pointers are %NULL.
//
// +----------------+
// |       (1)  (R) |
// | 192.168.0.0/16 |
// |    value: 1    |
// |   [0]    [1]   |
// +----------------+
//
// Next, let's add a new node (2) matching 192.168.0.0/24. As there is already
// a node with the same data and a smaller prefix (ie, a less specific one),
// node (2) will become a child of (1). In child index depends on the next bit
// that is outside of what (1) matches, and that bit is 0, so (2) will be
// child[0] of (1):
//
// +----------------+
// |       (1)  (R) |
// | 192.168.0.0/16 |
// |    value: 1    |
// |   [0]    [1]   |
// +----------------+
// |
// +----------------+
// |       (2)      |
// | 192.168.0.0/24 |
// |    value: 2    |
// |   [0]    [1]   |
// +----------------+
//
// The child[1] slot of (1) could be filled with another node which has bit #17
// (the next bit after the ones that (1) matches on) set to 1. For instance,
// 192.168.128.0/24:
//
// +----------------+
// |       (1)  (R) |
// | 192.168.0.0/16 |
// |    value: 1    |
// |   [0]    [1]   |
// +----------------+
// |      |
// +----------------+  +------------------+
// |       (2)      |  |        (3)       |
// | 192.168.0.0/24 |  | 192.168.128.0/24 |
// |    value: 2    |  |     value: 3     |
// |   [0]    [1]   |  |    [0]    [1]    |
// +----------------+  +------------------+
//
// Let's add another node (4) to the game for 192.168.1.0/24. In order to place
// it, node (1) is looked at first, and because (4) of the semantics laid out
// above (bit #17 is 0), it would normally be attached to (1) as child[0].
// However, that slot is already allocated, so a new node is needed in between.
// That node does not have a value attached to it and it will never be
// returned to users as result of a lookup. It is only there to differentiate
// the traversal further. It will get a prefix as wide as necessary to
// distinguish its two children:
//
// +----------------+
// |       (1)  (R) |
// | 192.168.0.0/16 |
// |    value: 1    |
// |   [0]    [1]   |
// +----------------+
// |      |
// +----------------+  +------------------+
// |       (4)  (I) |  |        (3)       |
// | 192.168.0.0/23 |  | 192.168.128.0/24 |
// |    value: ---  |  |     value: 3     |
// |   [0]    [1]   |  |    [0]    [1]    |
// +----------------+  +------------------+
// |      |
// +----------------+  +----------------+
// |       (2)      |  |       (5)      |
// | 192.168.0.0/24 |  | 192.168.1.0/24 |
// |    value: 2    |  |     value: 5   |
// |   [0]    [1]   |  |   [0]    [1]   |
// +----------------+  +----------------+
//
// 192.168.1.1/32 would be a child of (5) etc.
//
// An intermediate node will be turned into a 'real' node on demand. In the
// example above, (4) would be re-used if 192.168.0.0/23 is added to the trie.
//
// A fully populated trie would have a height of 32 nodes, as the trie was
// created with a prefix length of 32.
//
// The lookup starts at the root node. If the current node matches and if there
// is a child that can be used to become more specific, the trie is traversed
// downwards. The last node in the traversal that is a non-intermediate one is
// returned.
//
#[no_mangle]
pub unsafe extern "C" fn extract_bit(data: *const u8, index: usize) -> c_int {
    return !!(data[index / 8] & (1 << (7 - (index % 8))));
    }
//
// __longest_prefix_match() - determine the longest prefix
// @trie:	The trie to get internal sizes from
// @node:	The node to operate on
// @key:	The key to compare to @node
//
// Determine the longest prefix of @node that matches the bits in @key.
//
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn __longest_prefix_match(trie: *mut lpm_trie, node: *mut lpm_trie_node, key: *mut bpf_lpm_trie_key_u8) -> size_t {
pub static mut limit: u32 = 0;
pub static mut prefixlen: u32 = 0;
    BUILD_BUG_ON!(offsetof(lpm_trie_node, data) % sizeof!(u32));
    BUILD_BUG_ON!(offsetof(bpf_lpm_trie_key_u8, data) % sizeof!(u32));

// data_size >= 16 has very small probability.
// We do not use a loop for optimal code generation.
//
    if (trie.data_size >= 8) {
    u64 diff = be64_to_cpu(*node.data ^
// key->data);
    prefixlen = 64 - fls64(diff);
    if (prefixlen >= limit) {
    return limit;
    }
    if (diff) {
    return prefixlen;
    }
    i = 8;
    }

    while (trie.data_size >= i + 4) {
    u32 diff = be32_to_cpu(*&node.data[i] ^
// &key->data[i]);
    prefixlen += 32 - fls(diff);
    if (prefixlen >= limit) {
    return limit;
    }
    if (diff) {
    return prefixlen;
    }
    i += 4;
    }
    if (trie.data_size >= i + 2) {
    u16 diff = be16_to_cpu(*&node.data[i] ^
// &key->data[i]);
    prefixlen += 16 - fls(diff);
    if (prefixlen >= limit) {
    return limit;
    }
    if (diff) {
    return prefixlen;
    }
    i += 2;
    }
    if (trie.data_size >= i + 1) {
    prefixlen += 8 - fls(node.data[i] ^ key.data[i]);
    if (prefixlen >= limit) {
    return limit;
    }
    }
    return prefixlen;
    }
#[no_mangle]
pub unsafe extern "C" fn longest_prefix_match(trie: *mut lpm_trie, node: *mut lpm_trie_node, key: *mut bpf_lpm_trie_key_u8) -> size_t {
    return __longest_prefix_match(trie, node, key);
    }
// Called from syscall or from eBPF program
#[no_mangle]
pub unsafe extern "C" fn trie_lookup_elem(map: *mut bpf_map, _key: *mut c_void) -> *mut c_void {
    let mut trie = container_of!(map, lpm_trie, map);
    struct lpm_trie_node *node, *found = core::ptr::null_mut();
    let mut key = _key;
    if (key.prefixlen > trie.max_prefixlen) {
    return core::ptr::null_mut();
    }
// Start walking the trie from the root node ...
    while (node) {
    let mut next_bit = 0;
    let mut matchlen = 0;
// Determine the longest prefix of @node that matches @key.
// If it's the maximum possible prefix for this trie, we have
// an exact match and can return it directly.
//
    matchlen = __longest_prefix_match(trie, node, key);
    if (matchlen == trie.max_prefixlen) {
    found = node;
    break;
    }
// If the number of bits that match is smaller than the prefix
// length of @node, bail out and return the node we have seen
// last in the traversal (ie, the parent).
//
    if (matchlen < node.prefixlen) {
    break;
    }
// Consider this node as return candidate unless it is an
// artificially added intermediate one.
//
    if (!(node.flags & LPM_TREE_NODE_FLAG_IM)) {
    found = node;
    }
// If the node match is fully satisfied, let's see if we can
// become more specific. Determine the next bit in the key and
// traverse down.
//
    next_bit = extract_bit(key.data, node.prefixlen);
    node = rcu_dereference_check(node.child[next_bit],
    bpf_rcu_lock_held());
    }
    if (!found) {
    return core::ptr::null_mut();
    }
    return found.data + trie.data_size;
    }
#[no_mangle]
pub unsafe extern "C" fn lpm_trie_node_alloc(trie: *mut lpm_trie, value: *mut c_void) -> *mut c_void {
pub static mut node: *mut c_void = core::ptr::null_mut();
    node = bpf_mem_cache_alloc(&trie.ma);
    if (!node) {
    return core::ptr::null_mut();
    }
    node.flags = 0;
    if (value) {
    memcpy(node.data + trie.data_size, value,
    trie.map.value_size);
    }
    return node;
    }
#[no_mangle]
unsafe extern "C" fn trie_check_add_elem(trie: *mut lpm_trie, flags: u64) -> c_int {
    if (flags == BPF_EXIST) {
    return -ENOENT;
    }
    if (trie.n_entries == trie.map.max_entries) {
    return -ENOSPC;
    }
    trie.n_entries += 1;
    return 0;
    }
// Called from syscall or from eBPF program
#[no_mangle]
pub unsafe extern "C" fn trie_update_elem(map: *mut bpf_map, _key: *mut c_void, value: *mut c_void, flags: u64) -> c_long {
    let mut trie = container_of!(map, lpm_trie, map);
    let mut node = core::ptr::null_mut();
    let mut im_node = core::ptr::null_mut();
    let mut new_node = core::ptr::null_mut();
    let mut free_node = core::ptr::null_mut();
    let mut slot = core::ptr::null_mut();
    let mut key = _key;
    let mut irq_flags = 0;
    let mut next_bit = 0;
pub static mut matchlen: usize = 0;
pub static mut ret: c_int = 0;
    if (unlikely(flags > BPF_EXIST)) {
    return -EINVAL;
    }
    if (key.prefixlen > trie.max_prefixlen) {
    return -EINVAL;
    }
// Allocate and fill a new node
    new_node = lpm_trie_node_alloc(trie, value);
    if (!new_node) {
    return -ENOMEM;
    }
    ret = raw_res_spin_lock_irqsave(&trie.lock, irq_flags);
    if (ret) {
// goto;
    }
    new_node.prefixlen = key.prefixlen;
    RCU_INIT_POINTER(new_node.child[0], core::ptr::null_mut());
    RCU_INIT_POINTER(new_node.child[1], core::ptr::null_mut());
    memcpy(new_node.data, key.data, trie.data_size);
// Now find a slot to attach the new node. To do that, walk the tree
// from the root and match as many bits as possible for each node until
// we either find an empty slot or a slot that needs to be replaced by
// an intermediate node.
//
    slot = &trie.root;
    while ((node = rcu_dereference_protected(*slot, 1))) {
    matchlen = longest_prefix_match(trie, node, key);
    if (node.prefixlen != matchlen ||
    node.prefixlen == key.prefixlen) {
    break;
    }
    next_bit = extract_bit(key.data, node.prefixlen);
    slot = &node.child[next_bit];
    }
// If the slot is empty (a free child pointer or an empty root),
// simply assign the @new_node to that slot and be done.
//
    if (!node) {
    ret = trie_check_add_elem(trie, flags);
    if (ret) {
// goto;
    }
    rcu_assign_pointer(*slot, new_node);
// goto;
    }
// If the slot we picked already exists, replace it with @new_node
// which already has the correct data array set.
//
    if (node.prefixlen == matchlen) {
    if (!(node.flags & LPM_TREE_NODE_FLAG_IM)) {
    if (flags == BPF_NOEXIST) {
    ret = -EEXIST;
// goto;
    }
    } else {
    ret = trie_check_add_elem(trie, flags);
    if (ret) {
// goto;
    }
    }
    new_node.child[0] = node.child[0];
    new_node.child[1] = node.child[1];
    rcu_assign_pointer(*slot, new_node);
    free_node = node;
// goto;
    }
    ret = trie_check_add_elem(trie, flags);
    if (ret) {
// goto;
    }
// If the new node matches the prefix completely, it must be inserted
// as an ancestor. Simply insert it between @node and *@slot.
//
    if (matchlen == key.prefixlen) {
    next_bit = extract_bit(node.data, matchlen);
    rcu_assign_pointer(new_node.child[next_bit], node);
    rcu_assign_pointer(*slot, new_node);
// goto;
    }
    im_node = lpm_trie_node_alloc(trie, core::ptr::null_mut());
    if (!im_node) {
    trie.n_entries -= 1;
    ret = -ENOMEM;
// goto;
    }
    im_node.prefixlen = matchlen;
    im_node.flags |= LPM_TREE_NODE_FLAG_IM;
    memcpy(im_node.data, node.data, trie.data_size);
// Now determine which child to install in which slot
    if (extract_bit(key.data, matchlen)) {
    rcu_assign_pointer(im_node.child[0], node);
    rcu_assign_pointer(im_node.child[1], new_node);
    } else {
    rcu_assign_pointer(im_node.child[0], new_node);
    rcu_assign_pointer(im_node.child[1], node);
    }
// Finally, assign the intermediate node to the determined slot
    rcu_assign_pointer(*slot, im_node);
// label;
    raw_res_spin_unlock_irqrestore(&trie.lock, irq_flags);
// label;
    if (ret) {
    bpf_mem_cache_free(&trie.ma, new_node);
    }
    bpf_mem_cache_free_rcu(&trie.ma, free_node);
    return ret;
    }
// Called from syscall or from eBPF program
#[no_mangle]
unsafe extern "C" fn trie_delete_elem(map: *mut bpf_map, _key: *mut c_void) -> c_long {
    let mut trie = container_of!(map, lpm_trie, map);
    let mut free_node = core::ptr::null_mut(), *free_parent = core::ptr::null_mut();
    let mut key = _key;
    let mut trim = core::ptr::null_mut();
    let mut trim2 = core::ptr::null_mut();
    let mut node = core::ptr::null_mut();
    let mut parent = core::ptr::null_mut();
    let mut irq_flags = 0;
    let mut next_bit = 0;
pub static mut matchlen: usize = 0;
pub static mut ret: c_int = 0;
    if (key.prefixlen > trie.max_prefixlen) {
    return -EINVAL;
    }
    ret = raw_res_spin_lock_irqsave(&trie.lock, irq_flags);
    if (ret) {
    return ret;
    }
// Walk the tree looking for an exact key/length match and keeping
// track of the path we traverse.  We will need to know the node
// we wish to delete, and the slot that points to the node we want
// to delete.  We may also need to know the nodes parent and the
// slot that contains it.
//
    trim = &trie.root;
    trim2 = trim;
    parent = core::ptr::null_mut();
    while ((node = rcu_dereference_protected(*trim, 1))) {
    matchlen = longest_prefix_match(trie, node, key);
    if (node.prefixlen != matchlen ||
    node.prefixlen == key.prefixlen) {
    break;
    }
    parent = node;
    trim2 = trim;
    next_bit = extract_bit(key.data, node.prefixlen);
    trim = &node.child[next_bit];
    }
    if (!node || node.prefixlen != key.prefixlen ||
    node.prefixlen != matchlen ||
    (node.flags & LPM_TREE_NODE_FLAG_IM)) {
    ret = -ENOENT;
// goto;
    }
    trie.n_entries -= 1;
// If the node we are removing has two children, simply mark it
// as intermediate and we are done.
//
    if (rcu_access_pointer(node.child[0]) &&
    rcu_access_pointer(node.child[1])) {
    node.flags |= LPM_TREE_NODE_FLAG_IM;
// goto;
    }
// If the parent of the node we are about to delete is an intermediate
// node, and the deleted node doesn't have any children, we can delete
// the intermediate parent as well and promote its other child
// up the tree.  Doing this maintains the invariant that all
// intermediate nodes have exactly 2 children and that there are no
// unnecessary intermediate nodes in the tree.
//
    if (parent && (parent.flags & LPM_TREE_NODE_FLAG_IM) &&
    !node.child[0] && !node.child[1]) {
    if (node == rcu_access_pointer(parent.child[0])) {
    rcu_assign_pointer(
// trim2, rcu_access_pointer(parent->child[1]));
    }
    else {
    rcu_assign_pointer(
// trim2, rcu_access_pointer(parent->child[0]));
    }
    free_parent = parent;
    free_node = node;
// goto;
    }
// The node we are removing has either zero or one child. If there
// is a child, move it into the removed node's slot then delete
// the node.  Otherwise just clear the slot and delete the node.
//
    if (node.child[0]) {
    rcu_assign_pointer(*trim, rcu_access_pointer(node.child[0]));
    }

    else if (node.child[1]) {
    rcu_assign_pointer(*trim, rcu_access_pointer(node.child[1]));
    }
    else {
    RCU_INIT_POINTER(*trim, core::ptr::null_mut());
    }
    free_node = node;
// label;
    raw_res_spin_unlock_irqrestore(&trie.lock, irq_flags);
    bpf_mem_cache_free_rcu(&trie.ma, free_parent);
    bpf_mem_cache_free_rcu(&trie.ma, free_node);
    return ret;
    }
pub const LPM_DATA_SIZE_MAX: c_int = 256;
pub const LPM_DATA_SIZE_MIN: c_int = 1;

    sizeof!(lpm_trie_node))
pub const LPM_VAL_SIZE_MIN: c_int = 1;

    BPF_F_ACCESS_MASK)
#[no_mangle]
pub unsafe extern "C" fn trie_alloc(attr: *mut union bpf_attr) -> *mut c_void {
pub static mut trie: *mut c_void = core::ptr::null_mut();
    let mut leaf_size = 0;
    let mut err = 0;
// check sanity of attributes
    if (attr.max_entries == 0 ||
    !(attr.map_flags & BPF_F_NO_PREALLOC) ||
    attr.map_flags & ~LPM_CREATE_FLAG_MASK ||
    !bpf_map_flags_access_ok(attr.map_flags) ||
    attr.key_size < LPM_KEY_SIZE_MIN ||
    attr.key_size > LPM_KEY_SIZE_MAX ||
    attr.value_size < LPM_VAL_SIZE_MIN ||
    attr.value_size > LPM_VAL_SIZE_MAX) {
    return ERR_PTR(-EINVAL);
    }
    trie = bpf_map_area_alloc(sizeof!(*trie), NUMA_NO_NODE);
    if (!trie) {
    return ERR_PTR(-ENOMEM);
    }
// copy mandatory map attributes
    bpf_map_init_from_attr(&trie.map, attr);
    trie.data_size = attr.key_size -
    offsetof(bpf_lpm_trie_key_u8, data);
    trie.max_prefixlen = trie.data_size * 8;
    raw_res_spin_lock_init(&trie.lock);
// Allocate intermediate and leaf nodes from the same allocator
    leaf_size = sizeof!(lpm_trie_node) + trie.data_size +
    trie.map.value_size;
    err = bpf_mem_alloc_init(&trie.ma, leaf_size, false);
    if (err) {
// goto;
    }
    return &trie.map;
// label;
    bpf_map_area_free(trie);
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn trie_free(map: *mut bpf_map) {
    let mut trie = container_of!(map, lpm_trie, map);
    let mut slot = core::ptr::null_mut();
pub static mut node: *mut c_void = core::ptr::null_mut();
// Always start at the root and walk down to a node that has no
// children. Then free that node, nullify its reference in the parent
// and start over.
//
    for (;;) {
    slot = &trie.root;
    for (;;) {
    node = rcu_dereference_protected(*slot, 1);
    if (!node) {
// goto;
    }
    if (rcu_access_pointer(node.child[0])) {
    slot = &node.child[0];
    continue;
    }
    if (rcu_access_pointer(node.child[1])) {
    slot = &node.child[1];
    continue;
    }
// No bpf program may access the map, so freeing the
// node without waiting for the extra RCU GP.
//
    bpf_mem_cache_raw_free(node);
    RCU_INIT_POINTER(*slot, core::ptr::null_mut());
    break;
    }
    }
// label;
    bpf_mem_alloc_destroy(&trie.ma);
    bpf_map_area_free(trie);
    }
#[no_mangle]
unsafe extern "C" fn trie_get_next_key(map: *mut bpf_map, _key: *mut c_void, _next_key: *mut c_void) -> c_int {
    struct lpm_trie_node *node, *next_node = core::ptr::null_mut(), *parent, *search_root;
    let mut trie = container_of!(map, lpm_trie, map);
    let mut key = _key, *next_key = _next_key;
    let mut node_stack = core::ptr::null_mut();
pub static mut err: c_int = 0;
    let mut next_bit = 0;
pub static mut matchlen: usize = 0;
// The get_next_key follows postorder. For the 4 node example in
// the top of this file, the trie_get_next_key() returns the following
// one after another:
// 192.168.0.0/24
// 192.168.1.0/24
// 192.168.128.0/24
// 192.168.0.0/16
//
// The idea is to return more specific keys before less specific ones.
//
// Empty trie
    search_root = rcu_dereference(trie.root);
    if (!search_root) {
    return -ENOENT;
    }
// For invalid key, find the leftmost node in the trie
    if (!key || key.prefixlen > trie.max_prefixlen) {
// goto;
    }
    node_stack = kmalloc_objs(lpm_trie_node *,
    trie.max_prefixlen + 1,
    GFP_ATOMIC | __GFP_NOWARN);
    if (!node_stack) {
    return -ENOMEM;
    }
// Try to find the exact node for the given key
    while (node) {
    node_stack[++stack_ptr] = node;
    matchlen = longest_prefix_match(trie, node, key);
    if (node.prefixlen != matchlen ||
    node.prefixlen == key.prefixlen) {
    break;
    }
    next_bit = extract_bit(key.data, node.prefixlen);
    node = rcu_dereference(node.child[next_bit]);
    }
    if (!node || node.prefixlen != matchlen ||
    (node.flags & LPM_TREE_NODE_FLAG_IM)) {
// goto;
    }
// The node with the exactly-matching key has been found,
// find the first node in postorder after the matched node.
//
    node = node_stack[stack_ptr];
    while (stack_ptr > 0) {
    parent = node_stack[stack_ptr - 1];
    if (rcu_dereference(parent.child[0]) == node) {
    search_root = rcu_dereference(parent.child[1]);
    if (search_root) {
// goto;
    }
    }
    if (!(parent.flags & LPM_TREE_NODE_FLAG_IM)) {
    next_node = parent;
// goto;
    }
    node = parent;
    stack_ptr -= 1;
    }
// did not find anything
    err = -ENOENT;
// goto;
// label;
// Find the leftmost non-intermediate node, all intermediate nodes
// have exact two children, so this function will never return NULL.
//
    while (node) {
    if (node.flags & LPM_TREE_NODE_FLAG_IM) {
    node = rcu_dereference(node.child[0]);
    } else {
    next_node = node;
    node = rcu_dereference(node.child[0]);
    if (!node) {
    node = rcu_dereference(next_node.child[1]);
    }
    }
    }
// label;
    next_key.prefixlen = next_node.prefixlen;
    memcpy(next_key + offsetof(bpf_lpm_trie_key_u8, data),
    next_node.data, trie.data_size);
// label;
    kfree(node_stack);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn trie_check_btf(map: *mut bpf_map, btf: *mut btf, key_type: *mut btf_type, value_type: *mut btf_type) -> c_int {
// Keys must have struct bpf_lpm_trie_key_u8 embedded.
    return BTF_INFO_KIND(key_type.info) != BTF_KIND_STRUCT ?
    -EINVAL : 0;
    }
#[no_mangle]
unsafe extern "C" fn trie_mem_usage(map: *const bpf_map) -> u64 {
    let mut trie = container_of!(map, lpm_trie, map);
    let mut elem_size = 0;
    elem_size = sizeof!(lpm_trie_node) + trie.data_size +
    trie.map.value_size;
    return elem_size * READ_ONCE(trie.n_entries);
    }
    BTF_ID_LIST_SINGLE(trie_map_btf_ids, struct, lpm_trie)
pub static mut bpf_map_ops: usize = 0;