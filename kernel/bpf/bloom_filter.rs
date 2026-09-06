//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/bloom_filter.c
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
// Copyright (c) 2021 Facebook

    (BPF_F_NUMA_NODE | BPF_F_ZERO_SEED | BPF_F_ACCESS_MASK)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_bloom_filter {
    pub map: bpf_map,
    pub bitset_mask: u32,
    pub hash_seed: u32,
    pub nr_hash_funcs: u32,
    pub bitset: [c_ulong; 0],
}

#[no_mangle]
pub unsafe extern "C" fn hash(bloom: *mut bpf_bloom_filter, value: *mut c_void, value_size: u32, index: u32) -> u32 {
    let mut h = 0;
    if (likely(value_size % 4 == 0)) {
    h = jhash2(value, value_size / 4, bloom.hash_seed + index);
    }
    else {
    h = jhash(value, value_size, bloom.hash_seed + index);
    }
    return h & bloom.bitset_mask;
    }
#[no_mangle]
unsafe extern "C" fn bloom_map_peek_elem(map: *mut bpf_map, value: *mut c_void) -> c_long {
    let mut bloom = container_of!(map, bpf_bloom_filter, map);
    u32 i, h;
    while (i < bloom.nr_hash_funcs) {
    h = hash(bloom, value, map.value_size, i);
    if (!test_bit(h % BITS_PER_LONG, bloom.bitset + BIT_WORD(h))) {
    return -ENOENT;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bloom_map_push_elem(map: *mut bpf_map, value: *mut c_void, flags: u64) -> c_long {
    let mut bloom = container_of!(map, bpf_bloom_filter, map);
    u32 i, h;
    if (flags != BPF_ANY) {
    return -EINVAL;
    }
//
// On 32-bit architectures, hashes larger than INT_MAX would be
// treated as negative by set_bit().
//
    while (i < bloom.nr_hash_funcs) {
    h = hash(bloom, value, map.value_size, i);
    set_bit(h % BITS_PER_LONG, bloom.bitset + BIT_WORD(h));
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bloom_map_pop_elem(map: *mut bpf_map, value: *mut c_void) -> c_long {
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn bloom_map_delete_elem(map: *mut bpf_map, value: *mut c_void) -> c_long {
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn bloom_map_get_next_key(map: *mut bpf_map, key: *mut c_void, next_key: *mut c_void) -> c_int {
    return -EOPNOTSUPP;
    }
// Called from syscall
#[no_mangle]
unsafe extern "C" fn bloom_map_alloc_check(attr: *mut union bpf_attr) -> c_int {
    if (attr.value_size > KMALLOC_MAX_SIZE) {
// if value_size is bigger, the user space won't be able to
// access the elements.
//
    return -E2BIG;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bloom_map_alloc(attr: *mut union bpf_attr) -> *mut c_void {
    u32 bitset_mask, nr_hash_funcs, nr_bits;
pub static mut numa_node: c_int = 0;
pub static mut bloom: *mut c_void = core::ptr::null_mut();
    let mut bitset_bytes = 0;
    if (attr.key_size != 0 || attr.value_size == 0 ||
    attr.max_entries == 0 ||
    attr.map_flags & ~BLOOM_CREATE_FLAG_MASK ||
    !bpf_map_flags_access_ok(attr.map_flags) ||
// The lower 4 bits of map_extra (0xF) specify the number
// of hash functions
//
    (attr.map_extra & ~0xF)) {
    return ERR_PTR(-EINVAL);
    }
    nr_hash_funcs = attr.map_extra;
    if (nr_hash_funcs == 0) {
// Default to using 5 hash functions if unspecified
    nr_hash_funcs = 5;
    }
// For the bloom filter, the optimal bit array size that minimizes the
// false positive probability is n * k / ln(2) where n is the number of
// expected entries in the bloom filter and k is the number of hash
// functions. We use 7 / 5 to approximate 1 / ln(2).
//
// We round this up to the nearest power of two to enable more efficient
// hashing using bitmasks. The bitmask will be the bit array size - 1.
//
// If this overflows a u32, the bit array size will have 2^32 (4
// GB) bits.
//
    if (check_mul_overflow(attr.max_entries, nr_hash_funcs, &nr_bits) ||
    check_mul_overflow(nr_bits / 5, (u32)7, &nr_bits) ||
    nr_bits > (1UL << 31)) {
    bitset_mask = U32_MAX;
    } else {
    if (nr_bits <= BITS_PER_LONG) {
    nr_bits = BITS_PER_LONG;
    }
    else {
    nr_bits = roundup_pow_of_two(nr_bits);
    }
    bitset_mask = nr_bits - 1;
    }
    bitset_bytes = BITS_TO_LONGS((u64)bitset_mask + 1) * sizeof!(unsigned long);
    bloom = bpf_map_area_alloc(sizeof!(*bloom) + bitset_bytes, numa_node);
    if (!bloom) {
    return ERR_PTR(-ENOMEM);
    }
    bpf_map_init_from_attr(&bloom.map, attr);
    bloom.nr_hash_funcs = nr_hash_funcs;
    bloom.bitset_mask = bitset_mask;
    if (!(attr.map_flags & BPF_F_ZERO_SEED)) {
    bloom.hash_seed = get_random_u32();
    }
    return &bloom.map;
    }
#[no_mangle]
unsafe extern "C" fn bloom_map_free(map: *mut bpf_map) {
    let mut bloom = container_of!(map, bpf_bloom_filter, map);
    bpf_map_area_free(bloom);
    }
#[no_mangle]
pub unsafe extern "C" fn bloom_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
// The eBPF program should use map_peek_elem instead
    return ERR_PTR(-EINVAL);
    }
#[no_mangle]
pub unsafe extern "C" fn bloom_map_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> c_long {
// The eBPF program should use map_push_elem instead
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn bloom_map_check_btf(map: *mut bpf_map, btf: *mut btf, key_type: *mut btf_type, value_type: *mut btf_type) -> c_int {
// Bloom filter maps are keyless
    return btf_type_is_void(key_type) ? 0 : -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn bloom_map_mem_usage(map: *const bpf_map) -> u64 {
pub static mut bloom: *mut c_void = core::ptr::null_mut();
    let mut bitset_bytes = 0;
    bloom = container_of!(map, bpf_bloom_filter, map);
    bitset_bytes = BITS_TO_BYTES((u64)bloom.bitset_mask + 1);
    bitset_bytes = roundup(bitset_bytes, sizeof!(unsigned long));
    return sizeof!(*bloom) + bitset_bytes;
    }
    BTF_ID_LIST_SINGLE(bpf_bloom_map_btf_ids, struct, bpf_bloom_filter)
pub static mut bpf_map_ops: usize = 0;