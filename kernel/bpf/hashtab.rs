//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/hashtab.c
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
// Copyright (c) 2011-2014 PLUMgrid, http://plumgrid.com
// Copyright (c) 2016 Facebook
//

    (BPF_F_NO_PREALLOC | BPF_F_NO_COMMON_LRU | BPF_F_NUMA_NODE |	
    BPF_F_ACCESS_MASK | BPF_F_ZERO_SEED)

    .map_lookup_batch =			
    _name##_map_lookup_batch,		
    .map_lookup_and_delete_batch =		
    _name##_map_lookup_and_delete_batch,	
    .map_update_batch =			
    generic_map_update_batch,		
    .map_delete_batch =			
    generic_map_delete_batch
//
// The bucket lock has two protection scopes:
//
// 1) Serializing concurrent operations from BPF programs on different
// CPUs
//
// 2) Serializing concurrent operations from BPF programs and sys_bpf()
//
// BPF programs can execute in any context including perf, kprobes and
// tracing. As there are almost no limits where perf, kprobes and tracing
// can be invoked from the lock operations need to be protected against
// deadlocks. Deadlocks can be caused by recursion and by an invocation in
// the lock held section when functions which acquire this lock are invoked
// from sys_bpf(). BPF recursion is prevented by incrementing the per CPU
// variable bpf_prog_active, which prevents BPF programs attached to perf
// events, kprobes and tracing to be invoked before the prior invocation
// from one of these contexts completed. sys_bpf() uses the same mechanism
// by pinning the task to the current CPU and incrementing the recursion
// protection across the map operation.
//
// This has subtle implications on PREEMPT_RT. PREEMPT_RT forbids certain
// operations like memory allocations (even with GFP_ATOMIC) from atomic
// contexts. This is required because even with GFP_ATOMIC the memory
// allocator calls into code paths which acquire locks with long held lock
// sections. To ensure the deterministic behaviour these locks are regular
// spinlocks, which are converted to 'sleepable' spinlocks on RT. The only
// true atomic contexts on an RT kernel are the low level hardware
// handling, scheduling, low level interrupt handling, NMIs etc. None of
// these contexts should ever do memory allocations.
//
// As regular device interrupt handlers and soft interrupts are forced into
// thread context, the existing code which does
// spin_lock*(); alloc(GFP_ATOMIC); spin_unlock*();
// just works.
//
// In theory the BPF locks could be converted to regular spinlocks as well,
// but the bucket locks and percpu_freelist locks can be taken from
// arbitrary contexts (perf, kprobes, tracepoints) which are required to be
// atomic contexts even on RT. Before the introduction of bpf_mem_alloc,
// it is only safe to use raw spinlock for preallocated hash map on a RT kernel,
// because there is no memory allocation within the lock held sections. However
// after hash map was fully converted to use bpf_mem_alloc, there will be
// non-synchronous memory allocation for non-preallocated hash map, so it is
// safe to always use raw spinlock for bucket lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bucket {
    pub head: hlist_nulls_head,
    pub raw_lock: rqspinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_htab {
    pub map: bpf_map,
    pub ma: bpf_mem_alloc,
    pub pcpu_ma: bpf_mem_alloc,
    pub buckets: *mut bucket,
    pub elems: *mut c_void,
    union {
    pub freelist: pcpu_freelist,
    pub lru: bpf_lru,
}

    struct htab_elem * *extra_elems;
// number of elements in non-preallocated hashtable are kept
// in either pcount or count
//
pub static mut pcount: usize = 0;
    let mut count;
    let mut use_percpu_counter = 0;
    let mut n_buckets = 0;	/* number of hash buckets */
    let mut elem_size = 0;	/* size of each element in bytes */
    let mut hashrnd = 0;
    };
// each htab element is struct htab_elem + key + value
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htab_elem {
    union {
    pub hash_node: hlist_nulls_node,
    struct {
    pub padding: *mut c_void,
    union {
    pub fnode: pcpu_freelist_node,
    pub batch_flink: *mut htab_elem,
}

    };
    };
    union {
// pointer to per-cpu pointer
pub static mut ptr_to_pptr: *mut c_void = core::ptr::null_mut();
pub static mut lru_node: usize = 0;
    };
    let mut hash = 0;
    char key[] __aligned(8);
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htab_btf_record {
    pub record: *mut btf_record,
    pub key_size: u32,
}

#[no_mangle]
pub unsafe extern "C" fn htab_is_prealloc(htab: *const bpf_htab) -> bool {
    return !(htab.map.map_flags & BPF_F_NO_PREALLOC);
    }
#[no_mangle]
unsafe extern "C" fn htab_init_buckets(htab: *mut bpf_htab) {
    let mut i = 0;
    while (i < htab.n_buckets) {
    INIT_HLIST_NULLS_HEAD(&htab.buckets[i].head, i);
    raw_res_spin_lock_init(&htab.buckets[i].raw_lock);
    cond_resched();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn htab_lock_bucket(b: *mut bucket, pflags: *mut c_ulong) -> c_int {
    let mut flags = 0;
    let mut ret = 0;
    ret = raw_res_spin_lock_irqsave(&b.raw_lock, flags);
    if (ret) {
    return ret;
    }
// pflags = flags;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn htab_unlock_bucket(b: *mut bucket, flags: c_ulong) {
    raw_res_spin_unlock_irqrestore(&b.raw_lock, flags);
    }
// forward_decl: htab_lru_map_delete_node;
#[no_mangle]
unsafe extern "C" fn htab_is_lru(htab: *const bpf_htab) -> bool {
    return htab.map.map_type == BPF_MAP_TYPE_LRU_HASH ||
    htab.map.map_type == BPF_MAP_TYPE_LRU_PERCPU_HASH;
    }
#[no_mangle]
unsafe extern "C" fn htab_is_percpu(htab: *const bpf_htab) -> bool {
    return htab.map.map_type == BPF_MAP_TYPE_PERCPU_HASH ||
    htab.map.map_type == BPF_MAP_TYPE_LRU_PERCPU_HASH;
    }
#[no_mangle]
pub unsafe extern "C" fn is_fd_htab(htab: *const bpf_htab) -> bool {
    return htab.map.map_type == BPF_MAP_TYPE_HASH_OF_MAPS;
    }
#[no_mangle]
pub unsafe extern "C" fn htab_elem_value(l: *mut htab_elem, key_size: u32) -> *mut c_void {
    return l.key + round_up(key_size, 8);
    }
#[no_mangle]
pub unsafe extern "C" fn htab_elem_set_ptr(l: *mut htab_elem, key_size: u32, pptr: *mut c_void) {
// htab_elem_value(l, key_size) = pptr;
    }
    static inline void  *htab_elem_get_ptr(htab_elem *l, u32 key_size)
    {
    return *htab_elem_value(l, key_size);
    }
#[no_mangle]
pub unsafe extern "C" fn fd_htab_map_get_ptr(map: *mut bpf_map, l: *mut htab_elem) -> *mut c_void {
    return *htab_elem_value(l, map.key_size);
    }
#[no_mangle]
pub unsafe extern "C" fn get_htab_elem(htab: *mut bpf_htab, i: c_int) -> *mut c_void {
    return  (htab.elems + i * (u64)htab.elem_size);
    }
// Both percpu and fd htab support in-place update, so no need for
// extra elem. LRU itself can remove the least used element, so
// there is no need for an extra elem during map_update.
//
#[no_mangle]
unsafe extern "C" fn htab_has_extra_elems(htab: *mut bpf_htab) -> bool {
    return !htab_is_percpu(htab) && !htab_is_lru(htab) && !is_fd_htab(htab);
    }
#[no_mangle]
unsafe extern "C" fn htab_free_prealloced_internal_structs(htab: *mut bpf_htab) {
pub static mut num_entries: u32 = 0;
    let mut i = 0;
    if (htab_has_extra_elems(htab)) {
    num_entries += num_possible_cpus();
    }
    while (i < num_entries) {
pub static mut elem: *mut c_void = core::ptr::null_mut();
    elem = get_htab_elem(htab, i);
    bpf_map_free_internal_structs(&htab.map,
    htab_elem_value(elem, htab.map.key_size));
    cond_resched();
    }
    }
#[no_mangle]
unsafe extern "C" fn htab_free_prealloced_fields(htab: *mut bpf_htab) {
pub static mut num_entries: u32 = 0;
    let mut i = 0;
    if (IS_ERR_OR_NULL(htab.map.record)) {
    return;
    }
//
// Preallocated maps do not have a bpf_mem_alloc destructor, so fully
// destroy every element, including the extra elements.
//
    if (htab_has_extra_elems(htab)) {
    num_entries += num_possible_cpus();
    }
    while (i < num_entries) {
pub static mut elem: *mut c_void = core::ptr::null_mut();
    elem = get_htab_elem(htab, i);
    if (htab_is_percpu(htab)) {
    let mut pptr = htab_elem_get_ptr(elem, htab.map.key_size);
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    bpf_obj_free_fields(htab.map.record, per_cpu_ptr(pptr, cpu));
    cond_resched();
    }
    } else {
    bpf_obj_free_fields(htab.map.record,
    htab_elem_value(elem, htab.map.key_size));
    cond_resched();
    }
    cond_resched();
    }
    }
#[no_mangle]
unsafe extern "C" fn htab_free_elems(htab: *mut bpf_htab) {
    let mut i = 0;
    if (!htab_is_percpu(htab)) {
// goto;
    }
    while (i < htab.map.max_entries) {
    let mut pptr = core::ptr::null_mut();
    pptr = htab_elem_get_ptr(get_htab_elem(htab, i),
    htab.map.key_size);
    free_percpu(pptr);
    cond_resched();
    }
// label;
    bpf_map_area_free(htab.elems);
    }
// The LRU list has a lock (lru_lock). Each htab bucket has a lock
// (bucket_lock). If both locks need to be acquired together, the lock
// order is always lru_lock -> bucket_lock and this only happens in
// bpf_lru_list.c logic. For example, certain code path of
// bpf_lru_pop_free(), which is called by function prealloc_lru_pop(),
// will acquire lru_lock first followed by acquiring bucket_lock.
//
// In hashtab.c, to avoid deadlock, lock acquisition of
// bucket_lock followed by lru_lock is not allowed. In such cases,
// bucket_lock needs to be released first before acquiring lru_lock.
//
#[no_mangle]
pub unsafe extern "C" fn prealloc_lru_pop(htab: *mut bpf_htab, key: *mut c_void, hash: u32) -> *mut c_void {
    let mut node = bpf_lru_pop_free(&htab.lru, hash);
pub static mut l: *mut c_void = core::ptr::null_mut();
    if (node) {
    bpf_map_inc_elem_count(&htab.map);
    l = container_of!(node, htab_elem, lru_node);
    memcpy(l.key, key, htab.map.key_size);
    return l;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn prealloc_init(htab: *mut bpf_htab) -> c_int {
pub static mut num_entries: u32 = 0;
pub static mut err: c_int = 0;
    if (htab_has_extra_elems(htab)) {
    num_entries += num_possible_cpus();
    }
    htab.elems = bpf_map_area_alloc((u64)htab.elem_size * num_entries,
    htab.map.numa_node);
    if (!htab.elems) {
    return -ENOMEM;
    }
    if (!htab_is_percpu(htab)) {
// goto;
    }
    while (i < num_entries) {
pub static mut size: u32 = 0;
    let mut pptr = core::ptr::null_mut();
    pptr = bpf_map_alloc_percpu(&htab.map, size, 8,
    GFP_USER | __GFP_NOWARN);
    if (!pptr) {
// goto;
    }
    htab_elem_set_ptr(get_htab_elem(htab, i), htab.map.key_size,
    pptr);
    cond_resched();
    }
// label;
    if (htab_is_lru(htab)) {
    err = bpf_lru_init(&htab.lru,
    htab.map.map_flags & BPF_F_NO_COMMON_LRU,
    offsetof(htab_elem, hash) -
    offsetof(htab_elem, lru_node),
    htab_lru_map_delete_node,
    htab);
    }
    else {
    err = pcpu_freelist_init(&htab.freelist);
    }
    if (err) {
// goto;
    }
    if (htab_is_lru(htab)) {
    bpf_lru_populate(&htab.lru, htab.elems,
    offsetof(htab_elem, lru_node),
    htab.elem_size, num_entries);
    }
    else {
    pcpu_freelist_populate(&htab.freelist,
    htab.elems + offsetof(htab_elem, fnode),
    htab.elem_size, num_entries);
    }
    return 0;
// label;
    htab_free_elems(htab);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn prealloc_destroy(htab: *mut bpf_htab) {
    htab_free_elems(htab);
    if (htab_is_lru(htab)) {
    bpf_lru_destroy(&htab.lru);
    }
    else {
    pcpu_freelist_destroy(&htab.freelist);
    }
    }
#[no_mangle]
unsafe extern "C" fn alloc_extra_elems(htab: *mut bpf_htab) -> c_int {
    struct htab_elem * *pptr, *l_new;
pub static mut l: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    pptr = bpf_map_alloc_percpu(&htab.map, sizeof!, 8,
    GFP_USER | __GFP_NOWARN);
    if (!pptr) {
    return -ENOMEM;
    }
    for_each_possible_cpu(cpu) {
    l = pcpu_freelist_pop(&htab.freelist);
// pop will succeed, since prealloc_init()
// preallocated extra num_possible_cpus elements
//
    l_new = container_of!(l, htab_elem, fnode);
// per_cpu_ptr(pptr, cpu) = l_new;
    }
    htab.extra_elems = pptr;
    return 0;
    }
// Called from syscall
#[no_mangle]
unsafe extern "C" fn htab_map_alloc_check(attr: *mut union bpf_attr) -> c_int {
    let mut percpu = (attr.map_type == BPF_MAP_TYPE_PERCPU_HASH ||
    attr.map_type == BPF_MAP_TYPE_LRU_PERCPU_HASH);
    let mut lru = (attr.map_type == BPF_MAP_TYPE_LRU_HASH ||
    attr.map_type == BPF_MAP_TYPE_LRU_PERCPU_HASH);
// percpu_lru means each cpu has its own LRU list.
// it is different from BPF_MAP_TYPE_PERCPU_HASH where
// the map's value itself is percpu.  percpu_lru has
// nothing to do with the map's value.
//
pub static mut percpu_lru: bool = false;
pub static mut prealloc: bool = false;
pub static mut zero_seed: bool = false;
pub static mut numa_node: c_int = 0;
    BUILD_BUG_ON!(offsetof(htab_elem, fnode.next) !=
    offsetof(htab_elem, hash_node.pprev));
    if (zero_seed && !capable(CAP_SYS_ADMIN)) {
// Guard against local DoS, and discourage production use.
    return -EPERM;
    }
    if (attr.map_flags & ~HTAB_CREATE_FLAG_MASK ||
    !bpf_map_flags_access_ok(attr.map_flags)) {
    return -EINVAL;
    }
    if (!lru && percpu_lru) {
    return -EINVAL;
    }
    if (lru && !prealloc) {
    return -ENOTSUPP;
    }
    if (numa_node != NUMA_NO_NODE && (percpu || percpu_lru)) {
    return -EINVAL;
    }
// check sanity of attributes.
// value_size == 0 may be allowed in the future to use map as a set
//
    if (attr.max_entries == 0 || attr.key_size == 0 ||
    attr.value_size == 0) {
    return -EINVAL;
    }
    if ((u64)attr.key_size + attr.value_size >= KMALLOC_MAX_SIZE -
    sizeof!(htab_elem)) {
// if key_size + value_size is bigger, the user space won't be
// able to access the elements via bpf syscall. This check
// also makes sure that the elem_size doesn't overflow and it's
// kmalloc-able later in htab_map_update_elem()
//
    return -E2BIG;
    }
// percpu map value size is bound by PCPU_MIN_UNIT_SIZE
    if (percpu && round_up(attr.value_size, 8) > PCPU_MIN_UNIT_SIZE) {
    return -E2BIG;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn htab_mem_dtor(obj: *mut c_void, ctx: *mut c_void) {
    let mut hrec = ctx;
    let mut elem = obj;
pub static mut map_value: *mut c_void = core::ptr::null_mut();
    if (IS_ERR_OR_NULL(hrec.record)) {
    return;
    }
    map_value = htab_elem_value(elem, hrec.key_size);
    bpf_obj_free_fields(hrec.record, map_value);
    }
#[no_mangle]
unsafe extern "C" fn htab_pcpu_mem_dtor(obj: *mut c_void, ctx: *mut c_void) {
    let mut pptr = *obj;
    let mut hrec = ctx;
    let mut cpu = 0;
    if (IS_ERR_OR_NULL(hrec.record)) {
    return;
    }
    for_each_possible_cpu(cpu) {
    bpf_obj_free_fields(hrec.record, per_cpu_ptr(pptr, cpu));
    }
    }
#[no_mangle]
unsafe extern "C" fn htab_dtor_ctx_free(ctx: *mut c_void) {
    let mut hrec = ctx;
    btf_record_free(hrec.record);
    kfree(ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_ma_set_dtor(map: *mut bpf_map, ma: *mut bpf_mem_alloc) -> c_int {
pub static mut hrec: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
// No need for dtors.
    if (IS_ERR_OR_NULL(map.record)) {
    return 0;
    }
    hrec = kzalloc_obj(*hrec);
    if (!hrec) {
    return -ENOMEM;
    }
    hrec.key_size = map.key_size;
    hrec.record = btf_record_dup(map.record);
    if (IS_ERR(hrec.record)) {
    err = PTR_ERR(hrec.record);
    kfree(hrec);
    return err;
    }
    bpf_mem_alloc_set_dtor(ma, dtor, htab_dtor_ctx_free, hrec);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn htab_map_check_btf(map: *mut bpf_map, btf: *mut btf, key_type: *mut btf_type, value_type: *mut btf_type) -> c_int {
    let mut htab = container_of!(map, bpf_htab, map);
    if (htab_is_prealloc(htab)) {
    return 0;
    }
//
// We must set the dtor using this callback, as map's BTF record is not
// populated in htab_map_alloc(), so it will always appear as NULL.
//
    if (htab_is_percpu(htab)) {
    return bpf_ma_set_dtor(map, &htab.pcpu_ma, htab_pcpu_mem_dtor);
    }
    else {
    return bpf_ma_set_dtor(map, &htab.ma, htab_mem_dtor);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn htab_map_alloc(attr: *mut union bpf_attr) -> *mut c_void {
    let mut percpu = (attr.map_type == BPF_MAP_TYPE_PERCPU_HASH ||
    attr.map_type == BPF_MAP_TYPE_LRU_PERCPU_HASH);
// percpu_lru means each cpu has its own LRU list.
// it is different from BPF_MAP_TYPE_PERCPU_HASH where
// the map's value itself is percpu.  percpu_lru has
// nothing to do with the map's value.
//
pub static mut percpu_lru: bool = false;
pub static mut prealloc: bool = false;
pub static mut htab: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    htab = bpf_map_area_alloc(sizeof!(*htab), NUMA_NO_NODE);
    if (!htab) {
    return ERR_PTR(-ENOMEM);
    }
    bpf_map_init_from_attr(&htab.map, attr);
    if (percpu_lru) {
// ensure each CPU's lru list has >=1 elements.
// since we are at it, make each lru list has the same
// number of elements.
//
    htab.map.max_entries = roundup(attr.max_entries,
    num_possible_cpus());
    if (htab.map.max_entries < attr.max_entries) {
    htab.map.max_entries = rounddown(attr.max_entries,
    num_possible_cpus());
    }
    }
// hash table size must be power of 2; roundup_pow_of_two() can overflow
// into UB on 32-bit arches, so check that first
//
    err = -E2BIG;
    if (htab.map.max_entries > 1UL << 31) {
// goto;
    }
    htab.n_buckets = roundup_pow_of_two(htab.map.max_entries);
    htab.elem_size = sizeof!(htab_elem) +
    round_up(htab.map.key_size, 8);
    if (percpu) {
    htab.elem_size += sizeof!;
    }
    else {
    htab.elem_size += round_up(htab.map.value_size, 8);
    }
// check for u32 overflow
    if (htab.n_buckets > U32_MAX / sizeof!(bucket)) {
// goto;
    }
    err = bpf_map_init_elem_count(&htab.map);
    if (err) {
// goto;
    }
    err = -ENOMEM;
    htab.buckets = bpf_map_area_alloc(htab.n_buckets *
    sizeof!(bucket),
    htab.map.numa_node);
    if (!htab.buckets) {
// goto;
    }
    if (htab.map.map_flags & BPF_F_ZERO_SEED) {
    htab.hashrnd = 0;
    }
    else {
    htab.hashrnd = get_random_u32();
    }
    htab_init_buckets(htab);
// compute_batch_value() computes batch value as num_online_cpus() * 2
// and __percpu_counter_compare() needs
// htab->max_entries - cur_number_of_elems to be more than batch * num_online_cpus()
// for percpu_counter to be faster than atomic_t. In practice the average bpf
// hash map size is 10k, which means that a system with 64 cpus will fill
// hashmap to 20% of 10k before percpu_counter becomes ineffective. Therefore
// define our own batch count as 32 then 10k hash map can be filled up to 80%:
// 10k - 8k > 32 _batch_ * 64 _cpus_
// and __percpu_counter_compare() will still be fast. At that point hash map
// collisions will dominate its performance anyway. Assume that hash map filled
// to 50+% isn't going to be O(1) and use the following formula to choose
// between percpu_counter and atomic_t.
//
pub const PERCPU_COUNTER_BATCH: c_int = 32;
    if (attr.max_entries / 2 > num_online_cpus() * PERCPU_COUNTER_BATCH) {
    htab.use_percpu_counter = true;
    }
    if (htab.use_percpu_counter) {
    err = percpu_counter_init(&htab.pcount, 0, GFP_KERNEL);
    if (err) {
// goto;
    }
    }
    if (prealloc) {
    err = prealloc_init(htab);
    if (err) {
// goto;
    }
    if (htab_has_extra_elems(htab)) {
    err = alloc_extra_elems(htab);
    if (err) {
// goto;
    }
    }
    } else {
    err = bpf_mem_alloc_init(&htab.ma, htab.elem_size, false);
    if (err) {
// goto;
    }
    if (percpu) {
    err = bpf_mem_alloc_init(&htab.pcpu_ma,
    round_up(htab.map.value_size, 8), true);
    if (err) {
// goto;
    }
    }
    }
    return &htab.map;
// label;
    prealloc_destroy(htab);
// label;
    if (htab.use_percpu_counter) {
    percpu_counter_destroy(&htab.pcount);
    }
    bpf_map_area_free(htab.buckets);
    bpf_mem_alloc_destroy(&htab.pcpu_ma);
    bpf_mem_alloc_destroy(&htab.ma);
// label;
    bpf_map_free_elem_count(&htab.map);
// label;
    bpf_map_area_free(htab);
    return ERR_PTR(err);
    }
#[no_mangle]
pub unsafe extern "C" fn htab_map_hash(key: *const c_void, key_len: u32, hashrnd: u32) -> u32 {
    if (likely(key_len % 4 == 0)) {
    return jhash2(key, key_len / 4, hashrnd);
    }
    return jhash(key, key_len, hashrnd);
    }
#[no_mangle]
pub unsafe extern "C" fn __select_bucket(htab: *mut bpf_htab, hash: u32) -> *mut c_void {
    return &htab.buckets[hash & (htab.n_buckets - 1)];
    }
#[no_mangle]
pub unsafe extern "C" fn select_bucket(htab: *mut bpf_htab, hash: u32) -> *mut c_void {
    return &__select_bucket(htab, hash).head;
    }
// this lookup function can only be called with bucket lock taken
#[no_mangle]
pub unsafe extern "C" fn lookup_elem_raw(head: *mut hlist_nulls_head, hash: u32, key: *mut c_void, key_size: u32) -> *mut c_void {
pub static mut n: *mut c_void = core::ptr::null_mut();
pub static mut l: *mut c_void = core::ptr::null_mut();
    hlist_nulls_for_each_entry_rcu(l, n, head, hash_node)
    if (l.hash == hash && !memcmp(&l.key, key, key_size)) {
    return l;
    }
    return core::ptr::null_mut();
    }
// can be called without bucket lock. it will repeat the loop in
// the unlikely event when elements moved from one bucket into another
// while link list is being walked
//
#[no_mangle]
pub unsafe extern "C" fn lookup_nulls_elem_raw(head: *mut hlist_nulls_head, hash: u32, key: *mut c_void, key_size: u32, n_buckets: u32) -> *mut c_void {
pub static mut n: *mut c_void = core::ptr::null_mut();
pub static mut l: *mut c_void = core::ptr::null_mut();
// label;
    hlist_nulls_for_each_entry_rcu(l, n, head, hash_node)
    if (l.hash == hash && !memcmp(&l.key, key, key_size)) {
    return l;
    }
    if (unlikely(get_nulls_value(n) != (hash & (n_buckets - 1)))) {
// goto;
    }
    return core::ptr::null_mut();
    }
// Called from syscall or from eBPF program directly, so
// arguments have to match bpf_map_lookup_elem() exactly.
// The return value is adjusted by BPF instructions
// in htab_map_gen_lookup().
//
#[no_mangle]
pub unsafe extern "C" fn __htab_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    let mut htab = container_of!(map, bpf_htab, map);
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut l: *mut c_void = core::ptr::null_mut();
    u32 hash, key_size;
    WARN_ON_ONCE!(!bpf_rcu_lock_held());
    key_size = map.key_size;
    hash = htab_map_hash(key, key_size, htab.hashrnd);
    head = select_bucket(htab, hash);
    l = lookup_nulls_elem_raw(head, hash, key, key_size, htab.n_buckets);
    return l;
    }
#[no_mangle]
pub unsafe extern "C" fn htab_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    let mut l = __htab_map_lookup_elem(map, key);
    if (l) {
    return htab_elem_value(l, map.key_size);
    }
    return core::ptr::null_mut();
    }
// inline bpf_map_lookup_elem() call.
// Instead of:
// bpf_prog
// bpf_map_lookup_elem
// map->ops->map_lookup_elem
// htab_map_lookup_elem
// __htab_map_lookup_elem
// do:
// bpf_prog
// __htab_map_lookup_elem
//
#[no_mangle]
unsafe extern "C" fn htab_map_gen_lookup(map: *mut bpf_map, insn_buf: *mut bpf_insn) -> c_int {
    let mut insn = insn_buf;
pub static mut ret: c_int = 0;
    BUILD_BUG_ON!(!__same_type(&__htab_map_lookup_elem,
    (void *(bpf_map *map, void *key))core::ptr::null_mut()));
// insn++ = BPF_EMIT_CALL(__htab_map_lookup_elem);
// insn++ = BPF_JMP_IMM(BPF_JEQ, ret, 0, 1);
// insn++ = BPF_ALU64_IMM(BPF_ADD, ret,
    offsetof(htab_elem, key) +
    round_up(map.key_size, 8));
    return insn - insn_buf;
    }
    static __always_inline void *__htab_lru_map_lookup_elem(bpf_map *map,
    void *key, const bool mark)
    {
    let mut l = __htab_map_lookup_elem(map, key);
    if (l) {
    if (mark) {
    bpf_lru_node_set_ref(&l.lru_node);
    }
    return htab_elem_value(l, map.key_size);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn htab_lru_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    return __htab_lru_map_lookup_elem(map, key, true);
    }
#[no_mangle]
pub unsafe extern "C" fn htab_lru_map_lookup_elem_sys(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    return __htab_lru_map_lookup_elem(map, key, false);
    }
#[no_mangle]
pub unsafe extern "C" fn htab_lru_map_gen_lookup(map: *mut bpf_map, insn_buf: *mut bpf_insn) -> c_int {
    let mut insn = insn_buf;
pub static mut ret: c_int = 0;
pub static mut ref_reg: c_int = 0;
    BUILD_BUG_ON!(!__same_type(&__htab_map_lookup_elem,
    (void *(bpf_map *map, void *key))core::ptr::null_mut()));
// insn++ = BPF_EMIT_CALL(__htab_map_lookup_elem);
// insn++ = BPF_JMP_IMM(BPF_JEQ, ret, 0, 4);
// insn++ = BPF_LDX_MEM(BPF_B, ref_reg, ret,
    offsetof(htab_elem, lru_node) +
    offsetof(bpf_lru_node, ref));
// insn++ = BPF_JMP_IMM(BPF_JNE, ref_reg, 0, 1);
// insn++ = BPF_ST_MEM(BPF_B, ret,
    offsetof(htab_elem, lru_node) +
    offsetof(bpf_lru_node, ref),
    1);
// insn++ = BPF_ALU64_IMM(BPF_ADD, ret,
    offsetof(htab_elem, key) +
    round_up(map.key_size, 8));
    return insn - insn_buf;
    }
#[no_mangle]
pub unsafe extern "C" fn check_and_cancel_fields(htab: *mut bpf_htab, elem: *mut htab_elem) {
    if (IS_ERR_OR_NULL(htab.map.record)) {
    return;
    }
    if (htab_is_percpu(htab)) {
    let mut pptr = htab_elem_get_ptr(elem, htab.map.key_size);
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    bpf_obj_cancel_fields(&htab.map, per_cpu_ptr(pptr, cpu));
    }
    } else {
    let mut map_value = htab_elem_value(elem, htab.map.key_size);
    bpf_obj_cancel_fields(&htab.map, map_value);
    }
    }
// It is called from the bpf_lru_list when the LRU needs to delete
// older elements from the htab.
//
#[no_mangle]
unsafe extern "C" fn htab_lru_map_delete_node(arg: *mut c_void, node: *mut bpf_lru_node) -> bool {
    let mut htab = arg;
    let mut l = core::ptr::null_mut(), *tgt_l;
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut n: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
pub static mut b: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    tgt_l = container_of!(node, htab_elem, lru_node);
    b = __select_bucket(htab, tgt_l.hash);
    head = &b.head;
    ret = htab_lock_bucket(b, &flags);
    if (ret) {
    return false;
    }
    hlist_nulls_for_each_entry_rcu(l, n, head, hash_node)
    if (l == tgt_l) {
    hlist_nulls_del_rcu(&l.hash_node);
    bpf_map_dec_elem_count(&htab.map);
    break;
    }
    htab_unlock_bucket(b, flags);
    if (l == tgt_l) {
    check_and_cancel_fields(htab, l);
    }
pub static mut l: return = 0;
    }
// Called from syscall
#[no_mangle]
unsafe extern "C" fn htab_map_get_next_key(map: *mut bpf_map, key: *mut c_void, next_key: *mut c_void) -> c_int {
    let mut htab = container_of!(map, bpf_htab, map);
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut l = core::ptr::null_mut();
    let mut next_l = core::ptr::null_mut();
    u32 hash, key_size;
pub static mut i: c_int = 0;
    WARN_ON_ONCE!(!rcu_read_lock_held());
    key_size = map.key_size;
    if (!key) {
// goto;
    }
    hash = htab_map_hash(key, key_size, htab.hashrnd);
    head = select_bucket(htab, hash);
// lookup the key
    l = lookup_nulls_elem_raw(head, hash, key, key_size, htab.n_buckets);
    if (!l) {
// goto;
    }
// key was found, get next key in the same bucket
    next_l = hlist_nulls_entry_safe(rcu_dereference_raw(hlist_nulls_next_rcu(&l.hash_node)), htab_elem, hash_node);
    if (next_l) {
// if next elem in this hash list is non-zero, just return it
    memcpy(next_key, next_l.key, key_size);
    return 0;
    }
// no more elements in this hash list, go to the next bucket
    i = hash & (htab.n_buckets - 1);
    i += 1;
// label;
// iterate over buckets
    while (i < htab.n_buckets) {
    head = select_bucket(htab, i);
// pick first element in the bucket
    next_l = hlist_nulls_entry_safe(rcu_dereference_raw(hlist_nulls_first_rcu(head)), htab_elem, hash_node);
    if (next_l) {
// if it's not empty, just return it
    memcpy(next_key, next_l.key, key_size);
    return 0;
    }
    }
// iterated over all buckets and all elements
    return -ENOENT;
    }
#[no_mangle]
unsafe extern "C" fn htab_elem_free(htab: *mut bpf_htab, l: *mut htab_elem) {
    check_and_cancel_fields(htab, l);
    if (htab.map.map_type == BPF_MAP_TYPE_PERCPU_HASH) {
    bpf_mem_cache_free(&htab.pcpu_ma, l.ptr_to_pptr);
    }
    bpf_mem_cache_free(&htab.ma, l);
    }
#[no_mangle]
unsafe extern "C" fn htab_put_fd_value(htab: *mut bpf_htab, l: *mut htab_elem) {
    let mut map = &htab.map;
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    if (map.ops.map_fd_put_ptr) {
    ptr = fd_htab_map_get_ptr(map, l);
    map.ops.map_fd_put_ptr(map, ptr, true);
    }
    }
#[no_mangle]
unsafe extern "C" fn is_map_full(htab: *mut bpf_htab) -> bool {
    if (htab.use_percpu_counter) {
    return __percpu_counter_compare(&htab.pcount, htab.map.max_entries,
    PERCPU_COUNTER_BATCH) >= 0;
    }
    return atomic_read(&htab.count) >= htab.map.max_entries;
    }
#[no_mangle]
unsafe extern "C" fn inc_elem_count(htab: *mut bpf_htab) {
    bpf_map_inc_elem_count(&htab.map);
    if (htab.use_percpu_counter) {
    percpu_counter_add_batch(&htab.pcount, 1, PERCPU_COUNTER_BATCH);
    }
    else {
    atomic_inc(&htab.count);
    }
    }
#[no_mangle]
unsafe extern "C" fn dec_elem_count(htab: *mut bpf_htab) {
    bpf_map_dec_elem_count(&htab.map);
    if (htab.use_percpu_counter) {
    percpu_counter_add_batch(&htab.pcount, -1, PERCPU_COUNTER_BATCH);
    }
    else {
    atomic_dec(&htab.count);
    }
    }
#[no_mangle]
unsafe extern "C" fn free_htab_elem(htab: *mut bpf_htab, l: *mut htab_elem) {
    htab_put_fd_value(htab, l);
    if (htab_is_prealloc(htab)) {
    bpf_map_dec_elem_count(&htab.map);
    check_and_cancel_fields(htab, l);
    pcpu_freelist_push(&htab.freelist, &l.fnode);
    } else {
    dec_elem_count(htab);
    htab_elem_free(htab, l);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pcpu_copy_value(htab: *mut bpf_htab, pptr: *mut c_void, value: *mut c_void, onallcpus: bool, map_flags: u64) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    if (!onallcpus) {
// copy true value_size bytes
    ptr = this_cpu_ptr(pptr);
    copy_map_value(&htab.map, ptr, value);
    bpf_obj_cancel_fields(&htab.map, ptr);
    } else {
pub static mut size: u32 = 0;
pub static mut val: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    if (map_flags & BPF_F_CPU) {
    cpu = map_flags >> 32;
    ptr = per_cpu_ptr(pptr, cpu);
    copy_map_value(&htab.map, ptr, value);
    bpf_obj_cancel_fields(&htab.map, ptr);
    return;
    }
    for_each_possible_cpu(cpu) {
    ptr = per_cpu_ptr(pptr, cpu);
    val = (map_flags & BPF_F_ALL_CPUS) ? value : value + size * cpu;
    copy_map_value(&htab.map, ptr, val);
    bpf_obj_cancel_fields(&htab.map, ptr);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pcpu_init_value(htab: *mut bpf_htab, pptr: *mut c_void, value: *mut c_void, onallcpus: bool, map_flags: u64) {
// When not setting the initial value on all cpus, zero-fill element
// values for other cpus. Otherwise, bpf program has no way to ensure
// known initial values for cpus other than current one
// (onallcpus=false always when coming from bpf prog).
//
    if (!onallcpus) {
pub static mut current_cpu: c_int = 0;
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    if (cpu == current_cpu) {
    copy_map_value(&htab.map, per_cpu_ptr(pptr, cpu), value);
    }
    else /* Since elem is preallocated, we cannot touch special fields */
    zero_map_value(&htab.map, per_cpu_ptr(pptr, cpu));
    }
    } else {
    pcpu_copy_value(htab, pptr, value, onallcpus, map_flags);
    }
    }
#[no_mangle]
unsafe extern "C" fn fd_htab_map_needs_adjust(htab: *const bpf_htab) -> bool {
    return is_fd_htab(htab) && BITS_PER_LONG == 64;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_htab_elem(htab: *mut bpf_htab, key: *mut c_void, value: *mut c_void, key_size: u32, hash: u32, percpu: bool, onallcpus: bool, old_elem: *mut htab_elem, map_flags: u64) -> *mut c_void {
pub static mut size: u32 = 0;
pub static mut prealloc: bool = false;
    let mut l_new = core::ptr::null_mut();
    let mut pl_new = core::ptr::null_mut();
    let mut pptr = core::ptr::null_mut();
    if (prealloc) {
    if (old_elem) {
// if we're updating the existing element,
// use per-cpu extra elems to avoid freelist_pop/push
//
    pl_new = this_cpu_ptr(htab.extra_elems);
    l_new = *pl_new;
// pl_new = old_elem;
    } else {
pub static mut l: *mut c_void = core::ptr::null_mut();
    l = __pcpu_freelist_pop(&htab.freelist);
    if (!l) {
    return ERR_PTR(-E2BIG);
    }
    l_new = container_of!(l, htab_elem, fnode);
    bpf_map_inc_elem_count(&htab.map);
    }
    } else {
    if (is_map_full(htab)) {
    if (!old_elem)
// when map is full and update() is replacing
// old element, it's ok to allocate, since
// old element will be freed immediately.
// Otherwise return an error
//
    return ERR_PTR(-E2BIG);
    }
    inc_elem_count(htab);
    l_new = bpf_mem_cache_alloc(&htab.ma);
    if (!l_new) {
    l_new = ERR_PTR(-ENOMEM);
// goto;
    }
    }
    memcpy(l_new.key, key, key_size);
    if (percpu) {
    if (prealloc) {
    pptr = htab_elem_get_ptr(l_new, key_size);
    } else {
// alloc_percpu zero-fills
    let mut ptr = bpf_mem_cache_alloc(&htab.pcpu_ma);
    if (!ptr) {
    bpf_mem_cache_free(&htab.ma, l_new);
    l_new = ERR_PTR(-ENOMEM);
// goto;
    }
    l_new.ptr_to_pptr = ptr;
    pptr = *ptr;
    }
    pcpu_init_value(htab, pptr, value, onallcpus, map_flags);
    if (!prealloc) {
    htab_elem_set_ptr(l_new, key_size, pptr);
    }
    } else if (fd_htab_map_needs_adjust(htab)) {
    size = round_up(size, 8);
    memcpy(htab_elem_value(l_new, key_size), value, size);
    } else if (map_flags & BPF_F_LOCK) {
    copy_map_value_locked(&htab.map,
    htab_elem_value(l_new, key_size),
    value, false);
    } else {
    copy_map_value(&htab.map, htab_elem_value(l_new, key_size), value);
    }
    l_new.hash = hash;
    return l_new;
// label;
    dec_elem_count(htab);
    return l_new;
    }
#[no_mangle]
pub unsafe extern "C" fn check_flags(htab: *mut bpf_htab, l_old: *mut htab_elem, map_flags: u64) -> c_int {
    if (l_old && (map_flags & ~BPF_F_LOCK) == BPF_NOEXIST) {
// elem already exists
    return -EEXIST;
    }
    if (!l_old && (map_flags & ~BPF_F_LOCK) == BPF_EXIST) {
// elem doesn't exist, cannot update it
    return -ENOENT;
    }
    return 0;
    }
// Called from syscall or from eBPF program
#[no_mangle]
pub unsafe extern "C" fn htab_map_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_long {
    let mut htab = container_of!(map, bpf_htab, map);
    let mut l_new = core::ptr::null_mut();
    let mut l_old = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
pub static mut b: *mut c_void = core::ptr::null_mut();
    u32 key_size, hash;
    let mut ret = 0;
    if (unlikely((map_flags & ~BPF_F_LOCK) > BPF_EXIST)) {
// unknown flags
    return -EINVAL;
    }
    WARN_ON_ONCE!(!bpf_rcu_lock_held());
    key_size = map.key_size;
    hash = htab_map_hash(key, key_size, htab.hashrnd);
    b = __select_bucket(htab, hash);
    head = &b.head;
    if (unlikely(map_flags & BPF_F_LOCK)) {
    if (unlikely(!btf_record_has_field(map.record, BPF_SPIN_LOCK))) {
    return -EINVAL;
    }
// find an element without taking the bucket lock
    l_old = lookup_nulls_elem_raw(head, hash, key, key_size,
    htab.n_buckets);
    ret = check_flags(htab, l_old, map_flags);
    if (ret) {
    return ret;
    }
    if (l_old) {
// grab the element lock and update value in place
    copy_map_value_locked(map,
    htab_elem_value(l_old, key_size),
    value, false);
    return 0;
    }
// fall through, grab the bucket lock and lookup again.
// 99.9% chance that the element won't be found,
// but second lookup under lock has to be done.
//
    }
    ret = htab_lock_bucket(b, &flags);
    if (ret) {
    return ret;
    }
    l_old = lookup_elem_raw(head, hash, key, key_size);
    ret = check_flags(htab, l_old, map_flags);
    if (ret) {
// goto;
    }
    if (unlikely(l_old && (map_flags & BPF_F_LOCK))) {
// first lookup without the bucket lock didn't find the element,
// but second lookup with the bucket lock found it.
// This case is highly unlikely, but has to be dealt with:
// grab the element lock in addition to the bucket lock
// and update element in place
//
    copy_map_value_locked(map,
    htab_elem_value(l_old, key_size),
    value, false);
    ret = 0;
// goto;
    }
    l_new = alloc_htab_elem(htab, key, value, key_size, hash, false, false,
    l_old, map_flags);
    if (IS_ERR(l_new)) {
// all pre-allocated elements are in use or memory exhausted
    ret = PTR_ERR(l_new);
// goto;
    }
// add new element to the head of the list, so that
// concurrent search will find it before old elem
//
    hlist_nulls_add_head_rcu(&l_new.hash_node, head);
    if (l_old) {
    hlist_nulls_del_rcu(&l_old.hash_node);
// l_old has already been stashed in htab->extra_elems, cancel
// its reusable special fields before it is available for reuse.
//
    if (htab_is_prealloc(htab)) {
    check_and_cancel_fields(htab, l_old);
    }
    }
    htab_unlock_bucket(b, flags);
    if (l_old && !htab_is_prealloc(htab)) {
    free_htab_elem(htab, l_old);
    }
    return 0;
// label;
    htab_unlock_bucket(b, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn htab_lru_push_free(htab: *mut bpf_htab, elem: *mut htab_elem) {
    check_and_cancel_fields(htab, elem);
    bpf_map_dec_elem_count(&htab.map);
    bpf_lru_push_free(&htab.lru, &elem.lru_node);
    }
#[no_mangle]
pub unsafe extern "C" fn htab_lru_map_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_long {
    let mut htab = container_of!(map, bpf_htab, map);
    struct htab_elem *l_new, *l_old = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
pub static mut b: *mut c_void = core::ptr::null_mut();
    u32 key_size, hash;
    let mut ret = 0;
    if (unlikely(map_flags > BPF_EXIST)) {
// unknown flags
    return -EINVAL;
    }
    WARN_ON_ONCE!(!bpf_rcu_lock_held());
    key_size = map.key_size;
    hash = htab_map_hash(key, key_size, htab.hashrnd);
    b = __select_bucket(htab, hash);
    head = &b.head;
// For LRU, we need to alloc before taking bucket's
// spinlock because getting free nodes from LRU may need
// to remove older elements from htab and this removal
// operation will need a bucket lock.
//
    l_new = prealloc_lru_pop(htab, key, hash);
    if (!l_new) {
    return -ENOMEM;
    }
    copy_map_value(&htab.map, htab_elem_value(l_new, map.key_size), value);
    ret = htab_lock_bucket(b, &flags);
    if (ret) {
// goto;
    }
    l_old = lookup_elem_raw(head, hash, key, key_size);
    ret = check_flags(htab, l_old, map_flags);
    if (ret) {
// goto;
    }
// add new element to the head of the list, so that
// concurrent search will find it before old elem
//
    hlist_nulls_add_head_rcu(&l_new.hash_node, head);
    if (l_old) {
    bpf_lru_node_set_ref(&l_new.lru_node);
    hlist_nulls_del_rcu(&l_old.hash_node);
    }
    ret = 0;
// label;
    htab_unlock_bucket(b, flags);
// label;
    if (ret) {
    htab_lru_push_free(htab, l_new);
    }

    else if (l_old) {
    htab_lru_push_free(htab, l_old);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn htab_map_check_update_flags(onallcpus: bool, map_flags: u64) -> c_int {
    if (unlikely(!onallcpus && map_flags > BPF_EXIST)) {
    return -EINVAL;
    }
    if (unlikely(onallcpus && ((map_flags & BPF_F_LOCK) || (u32)map_flags > BPF_F_ALL_CPUS))) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn htab_map_update_elem_in_place(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64, percpu: bool, onallcpus: bool) -> c_long {
    let mut htab = container_of!(map, bpf_htab, map);
    let mut l_new = core::ptr::null_mut();
    let mut l_old = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut old_map_ptr = core::ptr::null_mut();
    let mut flags = 0;
pub static mut b: *mut c_void = core::ptr::null_mut();
    u32 key_size, hash;
    let mut ret = 0;
    ret = htab_map_check_update_flags(onallcpus, map_flags);
    if (unlikely(ret)) {
    return ret;
    }
    WARN_ON_ONCE!(!bpf_rcu_lock_held());
    key_size = map.key_size;
    hash = htab_map_hash(key, key_size, htab.hashrnd);
    b = __select_bucket(htab, hash);
    head = &b.head;
    ret = htab_lock_bucket(b, &flags);
    if (ret) {
    return ret;
    }
    l_old = lookup_elem_raw(head, hash, key, key_size);
    ret = check_flags(htab, l_old, map_flags);
    if (ret) {
// goto;
    }
    if (l_old) {
// Update value in-place
    if (percpu) {
    pcpu_copy_value(htab, htab_elem_get_ptr(l_old, key_size),
    value, onallcpus, map_flags);
    } else {
    let mut inner_map_pptr = htab_elem_value(l_old, key_size);
    old_map_ptr = *inner_map_pptr;
    WRITE_ONCE(*inner_map_pptr, *value);
    }
    } else {
    l_new = alloc_htab_elem(htab, key, value, key_size,
    hash, percpu, onallcpus, core::ptr::null_mut(), map_flags);
    if (IS_ERR(l_new)) {
    ret = PTR_ERR(l_new);
// goto;
    }
    hlist_nulls_add_head_rcu(&l_new.hash_node, head);
    }
// label;
    htab_unlock_bucket(b, flags);
    if (old_map_ptr) {
    map.ops.map_fd_put_ptr(map, old_map_ptr, true);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __htab_lru_percpu_map_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64, onallcpus: bool) -> c_long {
    let mut htab = container_of!(map, bpf_htab, map);
    let mut l_new = core::ptr::null_mut(), *l_old;
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
pub static mut b: *mut c_void = core::ptr::null_mut();
    u32 key_size, hash;
    let mut ret = 0;
    ret = htab_map_check_update_flags(onallcpus, map_flags);
    if (unlikely(ret)) {
    return ret;
    }
    WARN_ON_ONCE!(!bpf_rcu_lock_held());
    key_size = map.key_size;
    hash = htab_map_hash(key, key_size, htab.hashrnd);
    b = __select_bucket(htab, hash);
    head = &b.head;
// For LRU, we need to alloc before taking bucket's
// spinlock because LRU's elem alloc may need
// to remove older elem from htab and this removal
// operation will need a bucket lock.
//
    if (map_flags != BPF_EXIST) {
    l_new = prealloc_lru_pop(htab, key, hash);
    if (!l_new) {
    return -ENOMEM;
    }
    }
    ret = htab_lock_bucket(b, &flags);
    if (ret) {
// goto;
    }
    l_old = lookup_elem_raw(head, hash, key, key_size);
    ret = check_flags(htab, l_old, map_flags);
    if (ret) {
// goto;
    }
    if (l_old) {
    bpf_lru_node_set_ref(&l_old.lru_node);
// per-cpu hash map can update value in-place
    pcpu_copy_value(htab, htab_elem_get_ptr(l_old, key_size),
    value, onallcpus, map_flags);
    } else {
    pcpu_init_value(htab, htab_elem_get_ptr(l_new, key_size),
    value, onallcpus, map_flags);
    hlist_nulls_add_head_rcu(&l_new.hash_node, head);
    l_new = core::ptr::null_mut();
    }
    ret = 0;
// label;
    htab_unlock_bucket(b, flags);
// label;
    if (l_new) {
    bpf_map_dec_elem_count(&htab.map);
    bpf_lru_push_free(&htab.lru, &l_new.lru_node);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn htab_percpu_map_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_long {
    return htab_map_update_elem_in_place(map, key, value, map_flags, true, false);
    }
#[no_mangle]
pub unsafe extern "C" fn htab_lru_percpu_map_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_long {
    return __htab_lru_percpu_map_update_elem(map, key, value, map_flags,
    false);
    }
// Called from syscall or from eBPF program
#[no_mangle]
unsafe extern "C" fn htab_map_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_long {
    let mut htab = container_of!(map, bpf_htab, map);
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut b: *mut c_void = core::ptr::null_mut();
pub static mut l: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    u32 hash, key_size;
    let mut ret = 0;
    WARN_ON_ONCE!(!bpf_rcu_lock_held());
    key_size = map.key_size;
    hash = htab_map_hash(key, key_size, htab.hashrnd);
    b = __select_bucket(htab, hash);
    head = &b.head;
    ret = htab_lock_bucket(b, &flags);
    if (ret) {
    return ret;
    }
    l = lookup_elem_raw(head, hash, key, key_size);
    if (l) {
    hlist_nulls_del_rcu(&l.hash_node);
    }
    else {
    ret = -ENOENT;
    }
    htab_unlock_bucket(b, flags);
    if (l) {
    free_htab_elem(htab, l);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn htab_lru_map_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_long {
    let mut htab = container_of!(map, bpf_htab, map);
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut b: *mut c_void = core::ptr::null_mut();
pub static mut l: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    u32 hash, key_size;
    let mut ret = 0;
    WARN_ON_ONCE!(!bpf_rcu_lock_held());
    key_size = map.key_size;
    hash = htab_map_hash(key, key_size, htab.hashrnd);
    b = __select_bucket(htab, hash);
    head = &b.head;
    ret = htab_lock_bucket(b, &flags);
    if (ret) {
    return ret;
    }
    l = lookup_elem_raw(head, hash, key, key_size);
    if (l) {
    hlist_nulls_del_rcu(&l.hash_node);
    }
    else {
    ret = -ENOENT;
    }
    htab_unlock_bucket(b, flags);
    if (l) {
    htab_lru_push_free(htab, l);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn delete_all_elements(htab: *mut bpf_htab) {
    let mut i = 0;
// It's called from a worker thread and migration has been disabled,
// therefore, it is OK to invoke bpf_mem_cache_free() directly.
//
    while (i < htab.n_buckets) {
    let mut head = select_bucket(htab, i);
pub static mut n: *mut c_void = core::ptr::null_mut();
pub static mut l: *mut c_void = core::ptr::null_mut();
    hlist_nulls_for_each_entry_safe(l, n, head, hash_node) {
    hlist_nulls_del_rcu(&l.hash_node);
    htab_elem_free(htab, l);
    }
    cond_resched();
    }
    }
#[no_mangle]
unsafe extern "C" fn htab_free_malloced_internal_structs(htab: *mut bpf_htab) {
    let mut i = 0;
    rcu_read_lock();
    while (i < htab.n_buckets) {
    let mut head = select_bucket(htab, i);
pub static mut n: *mut c_void = core::ptr::null_mut();
pub static mut l: *mut c_void = core::ptr::null_mut();
    hlist_nulls_for_each_entry(l, n, head, hash_node) {
// We only free internal structs on uref dropping to zero
    bpf_map_free_internal_structs(&htab.map,
    htab_elem_value(l, htab.map.key_size));
    }
    cond_resched_rcu();
    }
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn htab_map_free_internal_structs(map: *mut bpf_map) {
    let mut htab = container_of!(map, bpf_htab, map);
// We only free internal structs on uref dropping to zero
    if (!bpf_map_has_internal_structs(map)) {
    return;
    }
    if (htab_is_prealloc(htab)) {
    htab_free_prealloced_internal_structs(htab);
    }
    else {
    htab_free_malloced_internal_structs(htab);
    }
    }
// Called when map->refcnt goes to zero, either from workqueue or from syscall
#[no_mangle]
unsafe extern "C" fn htab_map_free(map: *mut bpf_map) {
    let mut htab = container_of!(map, bpf_htab, map);
// bpf_free_used_maps() or close(map_fd) will trigger this map_free callback.
// bpf_free_used_maps() is called after bpf prog is no longer executing.
// There is no need to synchronize_rcu() here to protect map elements.
//
// htab no longer uses call_rcu() directly. bpf_mem_alloc does it
// underneath and is responsible for waiting for callbacks to finish
// during bpf_mem_alloc_destroy().
//
    if (!htab_is_prealloc(htab)) {
    delete_all_elements(htab);
    } else {
    htab_free_prealloced_fields(htab);
    prealloc_destroy(htab);
    }
    bpf_map_free_elem_count(map);
    free_percpu(htab.extra_elems);
    bpf_map_area_free(htab.buckets);
    bpf_mem_alloc_destroy(&htab.pcpu_ma);
    bpf_mem_alloc_destroy(&htab.ma);
    if (htab.use_percpu_counter) {
    percpu_counter_destroy(&htab.pcount);
    }
    bpf_map_area_free(htab);
    }
#[no_mangle]
pub unsafe extern "C" fn htab_map_seq_show_elem(map: *mut bpf_map, key: *mut c_void, m: *mut seq_file) {
pub static mut value: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    value = htab_map_lookup_elem(map, key);
    if (!value) {
    rcu_read_unlock();
    return;
    }
    btf_type_seq_show(map.btf, map.btf_key_type_id, key, m);
    seq_puts(m, ": ");
    btf_type_seq_show(map.btf, map.btf_value_type_id, value, m);
    seq_putc(m, '\n');
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn __htab_map_lookup_and_delete_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, is_lru_map: bool, is_percpu: bool, flags: u64) -> c_int {
    let mut htab = container_of!(map, bpf_htab, map);
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut bflags = 0;
pub static mut l: *mut c_void = core::ptr::null_mut();
    u32 hash, key_size;
pub static mut b: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    key_size = map.key_size;
    hash = htab_map_hash(key, key_size, htab.hashrnd);
    b = __select_bucket(htab, hash);
    head = &b.head;
    ret = htab_lock_bucket(b, &bflags);
    if (ret) {
    return ret;
    }
    l = lookup_elem_raw(head, hash, key, key_size);
    if (!l) {
    ret = -ENOENT;
// goto;
    }
    if (is_percpu) {
pub static mut roundup_value_size: u32 = 0;
    let mut pptr = core::ptr::null_mut();
pub static mut off: c_int = 0;
    pptr = htab_elem_get_ptr(l, key_size);
    for_each_possible_cpu(cpu) {
    copy_map_value_long(&htab.map, value + off, per_cpu_ptr(pptr, cpu));
    check_and_init_map_value(&htab.map, value + off);
    off += roundup_value_size;
    }
    } else {
    let mut src = htab_elem_value(l, map.key_size);
    if (flags & BPF_F_LOCK) {
    copy_map_value_locked(map, value, src, true);
    }
    else {
    copy_map_value(map, value, src);
    }
// Zeroing special fields in the temp buffer
    check_and_init_map_value(map, value);
    }
    hlist_nulls_del_rcu(&l.hash_node);
// label;
    htab_unlock_bucket(b, bflags);
    if (l) {
    if (is_lru_map) {
    htab_lru_push_free(htab, l);
    }
    else {
    free_htab_elem(htab, l);
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn htab_map_lookup_and_delete_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> c_int {
    return __htab_map_lookup_and_delete_elem(map, key, value, false, false,
    flags);
    }
#[no_mangle]
pub unsafe extern "C" fn htab_percpu_map_lookup_and_delete_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> c_int {
    return __htab_map_lookup_and_delete_elem(map, key, value, false, true,
    flags);
    }
#[no_mangle]
pub unsafe extern "C" fn htab_lru_map_lookup_and_delete_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> c_int {
    return __htab_map_lookup_and_delete_elem(map, key, value, true, false,
    flags);
    }
#[no_mangle]
pub unsafe extern "C" fn htab_lru_percpu_map_lookup_and_delete_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> c_int {
    return __htab_map_lookup_and_delete_elem(map, key, value, true, true,
    flags);
    }
#[no_mangle]
pub unsafe extern "C" fn __htab_map_lookup_and_delete_batch(map: *mut bpf_map, attr: *mut union bpf_attr, uattr: *mut union bpf_attr, do_delete: bool, is_lru_map: bool, is_percpu: bool) -> c_int {
    let mut htab = container_of!(map, bpf_htab, map);
    let mut keys = core::ptr::null_mut(), *values = core::ptr::null_mut(), *value, *dst_key, *dst_val;
    let mut uvalues = u64_to_user_ptr(attr.batch.values);
    let mut ukeys = u64_to_user_ptr(attr.batch.keys);
    let mut ubatch = u64_to_user_ptr(attr.batch.in_batch);
    u32 batch, max_count, size, bucket_size, map_id;
    u64 elem_map_flags, map_flags, allowed_flags;
    u32 bucket_cnt, total, key_size, value_size;
    let mut node_to_free = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut n: *mut c_void = core::ptr::null_mut();
pub static mut flags: c_ulong = 0;
pub static mut locked: bool = false;
pub static mut l: *mut c_void = core::ptr::null_mut();
pub static mut b: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    elem_map_flags = attr.batch.elem_flags;
    allowed_flags = BPF_F_LOCK;
    if (!do_delete && is_percpu) {
    allowed_flags |= BPF_F_CPU;
    }
    ret = bpf_map_check_op_flags(map, elem_map_flags, allowed_flags);
    if (ret) {
    return ret;
    }
    map_flags = attr.batch.flags;
    if (map_flags) {
    return -EINVAL;
    }
    max_count = attr.batch.count;
    if (!max_count) {
    return 0;
    }
    if (put_user(0, &uattr.batch.count)) {
    return -EFAULT;
    }
    batch = 0;
    if (ubatch && copy_from_user(&batch, ubatch, sizeof!(batch))) {
    return -EFAULT;
    }
    if (batch >= htab.n_buckets) {
    return -ENOENT;
    }
    key_size = htab.map.key_size;
    value_size = htab.map.value_size;
    size = round_up(value_size, 8);
    if (is_percpu && !(elem_map_flags & BPF_F_CPU)) {
    value_size = size * num_possible_cpus();
    }
    total = 0;
// while experimenting with hash tables with sizes ranging from 10 to
// 1000, it was observed that a bucket can have up to 5 entries.
//
    bucket_size = 5;
// label;
// We cannot do copy_from_user or copy_to_user inside
// the rcu_read_lock. Allocate enough space here.
//
    keys = kvmalloc_array(key_size, bucket_size, GFP_USER | __GFP_NOWARN);
    values = kvmalloc_array(value_size, bucket_size, GFP_USER | __GFP_NOWARN);
    if (!keys || !values) {
    ret = -ENOMEM;
// goto;
    }
// label;
    bpf_disable_instrumentation();
    rcu_read_lock();
// label;
    dst_key = keys;
    dst_val = values;
    b = &htab.buckets[batch];
    head = &b.head;
// do not grab the lock unless need it (bucket_cnt > 0).
    if (locked) {
    ret = htab_lock_bucket(b, &flags);
    if (ret) {
    rcu_read_unlock();
    bpf_enable_instrumentation();
// goto;
    }
    }
    bucket_cnt = 0;
    hlist_nulls_for_each_entry_rcu(l, n, head, hash_node)
    bucket_cnt += 1;
    if (bucket_cnt && !locked) {
    locked = true;
// goto;
    }
    if (bucket_cnt > (max_count - total)) {
    if (total == 0) {
    ret = -ENOSPC;
    }
// Note that since bucket_cnt > 0 here, it is implicit
// that the locked was grabbed, so release it.
//
    htab_unlock_bucket(b, flags);
    rcu_read_unlock();
    bpf_enable_instrumentation();
// goto;
    }
    if (bucket_cnt > bucket_size) {
    bucket_size = bucket_cnt;
// Note that since bucket_cnt > 0 here, it is implicit
// that the locked was grabbed, so release it.
//
    htab_unlock_bucket(b, flags);
    rcu_read_unlock();
    bpf_enable_instrumentation();
    kvfree(keys);
    kvfree(values);
// goto;
    }
// Next block is only safe to run if you have grabbed the lock
    if (!locked) {
// goto;
    }
    hlist_nulls_for_each_entry_safe(l, n, head, hash_node) {
    memcpy(dst_key, l.key, key_size);
    if (is_percpu) {
pub static mut off: c_int = 0;
    let mut pptr = core::ptr::null_mut();
    pptr = htab_elem_get_ptr(l, map.key_size);
    if (elem_map_flags & BPF_F_CPU) {
    cpu = elem_map_flags >> 32;
    copy_map_value(&htab.map, dst_val, per_cpu_ptr(pptr, cpu));
    check_and_init_map_value(&htab.map, dst_val);
    } else {
    for_each_possible_cpu(cpu) {
    copy_map_value_long(&htab.map, dst_val + off,
    per_cpu_ptr(pptr, cpu));
    check_and_init_map_value(&htab.map, dst_val + off);
    off += size;
    }
    }
    } else {
    value = htab_elem_value(l, key_size);
    if (is_fd_htab(htab)) {
    let mut inner_map = value;
// Actual value is the id of the inner map
    map_id = map.ops.map_fd_sys_lookup_elem(*inner_map);
    value = &map_id;
    }
    if (elem_map_flags & BPF_F_LOCK) {
    copy_map_value_locked(map, dst_val, value,
    true);
    }
    else {
    copy_map_value(map, dst_val, value);
    }
// Zeroing special fields in the temp buffer
    check_and_init_map_value(map, dst_val);
    }
    if (do_delete) {
    hlist_nulls_del_rcu(&l.hash_node);
// bpf_lru_push_free() will acquire lru_lock, which
// may cause deadlock. See comments in function
// prealloc_lru_pop(). Let us do bpf_lru_push_free()
// after releasing the bucket lock.
//
// For htab of maps, htab_put_fd_value() in
// free_htab_elem() may acquire a spinlock with bucket
// lock being held and it violates the lock rule, so
// invoke free_htab_elem() after unlock as well.
//
    l.batch_flink = node_to_free;
    node_to_free = l;
    }
    dst_key += key_size;
    dst_val += value_size;
    }
    htab_unlock_bucket(b, flags);
    locked = false;
    while (node_to_free) {
    l = node_to_free;
    node_to_free = node_to_free.batch_flink;
    if (is_lru_map) {
    htab_lru_push_free(htab, l);
    }
    else {
    free_htab_elem(htab, l);
    }
    }
// label;
// If we are not copying data, we can go to next bucket and avoid
// unlocking the rcu.
//
    if (!bucket_cnt && (batch + 1 < htab.n_buckets)) {
    batch += 1;
// goto;
    }
    rcu_read_unlock();
    bpf_enable_instrumentation();
    if (bucket_cnt && (copy_to_user(ukeys + total * key_size, keys,
    key_size * bucket_cnt) ||
    copy_to_user(uvalues + total * value_size, values,
    value_size * bucket_cnt))) {
    ret = -EFAULT;
// goto;
    }
    total += bucket_cnt;
    batch += 1;
    if (batch >= htab.n_buckets) {
    ret = -ENOENT;
// goto;
    }
// goto;
// label;
    if (ret == -EFAULT) {
// goto;
    }
// copy # of entries and next batch
    ubatch = u64_to_user_ptr(attr.batch.out_batch);
    if (copy_to_user(ubatch, &batch, sizeof!(batch)) ||
    put_user(total, &uattr.batch.count)) {
    ret = -EFAULT;
    }
// label;
    kvfree(keys);
    kvfree(values);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn htab_percpu_map_lookup_batch(map: *mut bpf_map, attr: *mut union bpf_attr, uattr: *mut union bpf_attr) -> c_int {
    return __htab_map_lookup_and_delete_batch(map, attr, uattr, false,
    false, true);
    }
#[no_mangle]
pub unsafe extern "C" fn htab_percpu_map_lookup_and_delete_batch(map: *mut bpf_map, attr: *mut union bpf_attr, uattr: *mut union bpf_attr) -> c_int {
    return __htab_map_lookup_and_delete_batch(map, attr, uattr, true,
    false, true);
    }
#[no_mangle]
pub unsafe extern "C" fn htab_map_lookup_batch(map: *mut bpf_map, attr: *mut union bpf_attr, uattr: *mut union bpf_attr) -> c_int {
    return __htab_map_lookup_and_delete_batch(map, attr, uattr, false,
    false, false);
    }
#[no_mangle]
pub unsafe extern "C" fn htab_map_lookup_and_delete_batch(map: *mut bpf_map, attr: *mut union bpf_attr, uattr: *mut union bpf_attr) -> c_int {
    return __htab_map_lookup_and_delete_batch(map, attr, uattr, true,
    false, false);
    }
#[no_mangle]
pub unsafe extern "C" fn htab_lru_percpu_map_lookup_batch(map: *mut bpf_map, attr: *mut union bpf_attr, uattr: *mut union bpf_attr) -> c_int {
    return __htab_map_lookup_and_delete_batch(map, attr, uattr, false,
    true, true);
    }
#[no_mangle]
pub unsafe extern "C" fn htab_lru_percpu_map_lookup_and_delete_batch(map: *mut bpf_map, attr: *mut union bpf_attr, uattr: *mut union bpf_attr) -> c_int {
    return __htab_map_lookup_and_delete_batch(map, attr, uattr, true,
    true, true);
    }
#[no_mangle]
pub unsafe extern "C" fn htab_lru_map_lookup_batch(map: *mut bpf_map, attr: *mut union bpf_attr, uattr: *mut union bpf_attr) -> c_int {
    return __htab_map_lookup_and_delete_batch(map, attr, uattr, false,
    true, false);
    }
#[no_mangle]
pub unsafe extern "C" fn htab_lru_map_lookup_and_delete_batch(map: *mut bpf_map, attr: *mut union bpf_attr, uattr: *mut union bpf_attr) -> c_int {
    return __htab_map_lookup_and_delete_batch(map, attr, uattr, true,
    true, false);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_seq_hash_map_info {
    pub map: *mut bpf_map,
    pub htab: *mut bpf_htab,
    pub hash: *mut *mut c_void percpu_value_buf; // non-zero means percpu,
    pub bucket_id: u32,
    pub skip_elems: u32,
}

#[no_mangle]
pub unsafe extern "C" fn bpf_hash_map_seq_find_next(info: *mut bpf_iter_seq_hash_map_info, prev_elem: *mut htab_elem) -> *mut c_void {
    let mut htab = info.htab;
pub static mut skip_elems: u32 = 0;
pub static mut bucket_id: u32 = 0;
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut n: *mut c_void = core::ptr::null_mut();
pub static mut elem: *mut c_void = core::ptr::null_mut();
pub static mut b: *mut c_void = core::ptr::null_mut();
    u32 i, count;
    if (bucket_id >= htab.n_buckets) {
    return core::ptr::null_mut();
    }
// try to find next elem in the same bucket
    if (prev_elem) {
// no update/deletion on this bucket, prev_elem should be still valid
// and we won't skip elements.
//
    n = rcu_dereference_raw(hlist_nulls_next_rcu(&prev_elem.hash_node));
    elem = hlist_nulls_entry_safe(n, htab_elem, hash_node);
    if (elem) {
    return elem;
    }
// not found, unlock and go to the next bucket
    b = &htab.buckets[bucket_id++];
    rcu_read_unlock();
    skip_elems = 0;
    }
    while (i < htab.n_buckets) {
    b = &htab.buckets[i];
    rcu_read_lock();
    count = 0;
    head = &b.head;
    hlist_nulls_for_each_entry_rcu(elem, n, head, hash_node) {
    if (count >= skip_elems) {
    info.bucket_id = i;
    info.skip_elems = count;
    return elem;
    }
    count += 1;
    }
    rcu_read_unlock();
    skip_elems = 0;
    }
    info.bucket_id = i;
    info.skip_elems = 0;
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_hash_map_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut info = seq.private;
pub static mut elem: *mut c_void = core::ptr::null_mut();
    elem = bpf_hash_map_seq_find_next(info, core::ptr::null_mut());
    if (!elem) {
    return core::ptr::null_mut();
    }
    if (*pos == 0) {
    ++*pos;
    }
    return elem;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_hash_map_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut info = seq.private;
    ++*pos;
    ++info.skip_elems;
    return bpf_hash_map_seq_find_next(info, v);
    }
#[no_mangle]
unsafe extern "C" fn __bpf_hash_map_seq_show(seq: *mut seq_file, elem: *mut htab_elem) -> c_int {
    let mut info = seq.private;
pub static mut ctx: bpf_iter__bpf_map_elem = 0;
    let mut map = info.map;
pub static mut meta: usize = 0;
pub static mut ret: c_int = 0;
    let mut roundup_value_size = 0;
pub static mut prog: *mut c_void = core::ptr::null_mut();
    let mut pptr = core::ptr::null_mut();
    meta.seq = seq;
    prog = bpf_iter_get_info(&meta, elem == core::ptr::null_mut());
    if (prog) {
    ctx.meta = &meta;
    ctx.map = info.map;
    if (elem) {
    ctx.key = elem.key;
    if (!info.percpu_value_buf) {
    ctx.value = htab_elem_value(elem, map.key_size);
    } else {
    roundup_value_size = round_up(map.value_size, 8);
    pptr = htab_elem_get_ptr(elem, map.key_size);
    for_each_possible_cpu(cpu) {
    copy_map_value_long(map, info.percpu_value_buf + off,
    per_cpu_ptr(pptr, cpu));
    check_and_init_map_value(map, info.percpu_value_buf + off);
    off += roundup_value_size;
    }
    ctx.value = info.percpu_value_buf;
    }
    }
    ret = bpf_iter_run_prog(prog, &ctx);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bpf_hash_map_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    return __bpf_hash_map_seq_show(seq, v);
    }
#[no_mangle]
unsafe extern "C" fn bpf_hash_map_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    if (!v) {
    (void)__bpf_hash_map_seq_show(seq, core::ptr::null_mut());
    }
    else {
    rcu_read_unlock();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_init_hash_map(priv_data: *mut c_void, aux: *mut bpf_iter_aux_info) -> c_int {
    let mut seq_info = priv_data;
    let mut map = aux.map;
pub static mut value_buf: *mut c_void = core::ptr::null_mut();
    let mut buf_size = 0;
    if (map.map_type == BPF_MAP_TYPE_PERCPU_HASH ||
    map.map_type == BPF_MAP_TYPE_LRU_PERCPU_HASH) {
    buf_size = round_up(map.value_size, 8) * num_possible_cpus();
    value_buf = kmalloc(buf_size, GFP_USER | __GFP_NOWARN);
    if (!value_buf) {
    return -ENOMEM;
    }
    seq_info.percpu_value_buf = value_buf;
    }
    bpf_map_inc_with_uref(map);
    seq_info.map = map;
    seq_info.htab = container_of!(map, bpf_htab, map);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_iter_fini_hash_map(priv_data: *mut c_void) {
    let mut seq_info = priv_data;
    bpf_map_put_with_uref(seq_info.map);
    kfree(seq_info.percpu_value_buf);
    }
pub static mut seq_operations: usize = 0;
pub static mut bpf_iter_seq_info: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_for_each_hash_elem(map: *mut bpf_map, callback_fn: bpf_callback_t, callback_ctx: *mut c_void, flags: u64) -> c_long {
    let mut htab = container_of!(map, bpf_htab, map);
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut n: *mut c_void = core::ptr::null_mut();
pub static mut elem: *mut c_void = core::ptr::null_mut();
    int i, num_elems = 0;
    let mut pptr = core::ptr::null_mut();
pub static mut b: *mut c_void = core::ptr::null_mut();
    let mut key = core::ptr::null_mut();
    let mut val = core::ptr::null_mut();
    let mut is_percpu = 0;
pub static mut ret: u64 = 0;
    cant_migrate();
    if (flags != 0) {
    return -EINVAL;
    }
    is_percpu = htab_is_percpu(htab);
// migration has been disabled, so percpu value prepared here will be
// the same as the one seen by the bpf program with
// bpf_map_lookup_elem().
//
    while (i < htab.n_buckets) {
    b = &htab.buckets[i];
    rcu_read_lock();
    head = &b.head;
    hlist_nulls_for_each_entry_safe(elem, n, head, hash_node) {
    key = elem.key;
    if (is_percpu) {
// current cpu value for percpu map
    pptr = htab_elem_get_ptr(elem, map.key_size);
    val = this_cpu_ptr(pptr);
    } else {
    val = htab_elem_value(elem, map.key_size);
    }
    num_elems += 1;
    ret = callback_fn((u64)(long)map, (u64)(long)key,
    (u64)(long)val, (u64)(long)callback_ctx, 0);
// return value: 0 - continue, 1 - stop and return
    if (ret) {
    rcu_read_unlock();
// goto;
    }
    }
    rcu_read_unlock();
    }
// label;
    return num_elems;
    }
#[no_mangle]
unsafe extern "C" fn htab_map_mem_usage(map: *const bpf_map) -> u64 {
    let mut htab = container_of!(map, bpf_htab, map);
pub static mut value_size: u32 = 0;
pub static mut prealloc: bool = false;
pub static mut percpu: bool = false;
pub static mut lru: bool = false;
    u64 num_entries, usage;
    usage = sizeof!(bpf_htab) +
    sizeof!(bucket) * htab.n_buckets;
    if (prealloc) {
    num_entries = map.max_entries;
    if (htab_has_extra_elems(htab)) {
    num_entries += num_possible_cpus();
    }
    usage += htab.elem_size * num_entries;
    if (percpu) {
    usage += value_size * num_possible_cpus() * num_entries;
    }

    else if (!lru) {
    usage += sizeof! * num_possible_cpus();
    }
    } else {

    num_entries = htab.use_percpu_counter ?
    percpu_counter_sum(&htab.pcount) :
    atomic_read(&htab.count);
    usage += (htab.elem_size + LLIST_NODE_SZ) * num_entries;
    if (percpu) {
    usage += (LLIST_NODE_SZ + sizeof!) * num_entries;
    usage += value_size * num_possible_cpus() * num_entries;
    }
    }
    return usage;
    }
    BTF_ID_LIST_SINGLE(htab_map_btf_ids, struct, bpf_htab)
pub static mut bpf_map_ops: usize = 0;
pub static mut bpf_map_ops: usize = 0;
// Called from eBPF program
#[no_mangle]
pub unsafe extern "C" fn htab_percpu_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    let mut l = __htab_map_lookup_elem(map, key);
    if (l) {
    return this_cpu_ptr(htab_elem_get_ptr(l, map.key_size));
    }
    else {
    return core::ptr::null_mut();
    }
    }
// inline bpf_map_lookup_elem() call for per-CPU hashmap
#[no_mangle]
unsafe extern "C" fn htab_percpu_map_gen_lookup(map: *mut bpf_map, insn_buf: *mut bpf_insn) -> c_int {
    let mut insn = insn_buf;
    if (!bpf_jit_supports_percpu_insn()) {
    return -EOPNOTSUPP;
    }
    BUILD_BUG_ON!(!__same_type(&__htab_map_lookup_elem,
    (void *(bpf_map *map, void *key))core::ptr::null_mut()));
// insn++ = BPF_EMIT_CALL(__htab_map_lookup_elem);
// insn++ = BPF_JMP_IMM(BPF_JEQ, BPF_REG_0, 0, 3);
// insn++ = BPF_ALU64_IMM(BPF_ADD, BPF_REG_0,
    offsetof(htab_elem, key) + roundup(map.key_size, 8));
// insn++ = BPF_LDX_MEM(BPF_DW, BPF_REG_0, BPF_REG_0, 0);
// insn++ = BPF_MOV64_PERCPU_REG(BPF_REG_0, BPF_REG_0);
    return insn - insn_buf;
    }
#[no_mangle]
pub unsafe extern "C" fn htab_percpu_map_lookup_percpu_elem(map: *mut bpf_map, key: *mut c_void, cpu: u32) -> *mut c_void {
pub static mut l: *mut c_void = core::ptr::null_mut();
    if (cpu >= nr_cpu_ids) {
    return core::ptr::null_mut();
    }
    l = __htab_map_lookup_elem(map, key);
    if (l) {
    return per_cpu_ptr(htab_elem_get_ptr(l, map.key_size), cpu);
    }
    else {
    return core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn htab_lru_percpu_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    let mut l = __htab_map_lookup_elem(map, key);
    if (l) {
    bpf_lru_node_set_ref(&l.lru_node);
    return this_cpu_ptr(htab_elem_get_ptr(l, map.key_size));
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn htab_lru_percpu_map_lookup_percpu_elem(map: *mut bpf_map, key: *mut c_void, cpu: u32) -> *mut c_void {
pub static mut l: *mut c_void = core::ptr::null_mut();
    if (cpu >= nr_cpu_ids) {
    return core::ptr::null_mut();
    }
    l = __htab_map_lookup_elem(map, key);
    if (l) {
    bpf_lru_node_set_ref(&l.lru_node);
    return per_cpu_ptr(htab_elem_get_ptr(l, map.key_size), cpu);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_percpu_hash_copy(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_int {
pub static mut l: *mut c_void = core::ptr::null_mut();
    let mut pptr = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    int cpu, off = 0;
    let mut size = 0;
// per_cpu areas are zero-filled and bpf programs can only
// access 'value_size' of them, so copying rounded areas
// will not leak any kernel data
//
    size = round_up(map.value_size, 8);
    rcu_read_lock();
    l = __htab_map_lookup_elem(map, key);
    if (!l) {
// goto;
    }
    ret = 0;
// We do not mark LRU map element here in order to not mess up
// eviction heuristics when user space does a map walk.
//
    pptr = htab_elem_get_ptr(l, map.key_size);
    if (map_flags & BPF_F_CPU) {
    cpu = map_flags >> 32;
    copy_map_value(map, value, per_cpu_ptr(pptr, cpu));
    check_and_init_map_value(map, value);
// goto;
    }
    for_each_possible_cpu(cpu) {
    copy_map_value_long(map, value + off, per_cpu_ptr(pptr, cpu));
    check_and_init_map_value(map, value + off);
    off += size;
    }
// label;
    rcu_read_unlock();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_percpu_hash_update(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_int {
    let mut htab = container_of!(map, bpf_htab, map);
    let mut ret = 0;
    rcu_read_lock();
    if (htab_is_lru(htab)) {
    ret = __htab_lru_percpu_map_update_elem(map, key, value,
    map_flags, true);
    }
    else {
    ret = htab_map_update_elem_in_place(map, key, value, map_flags,
    true, true);
    }
    rcu_read_unlock();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn htab_percpu_map_seq_show_elem(map: *mut bpf_map, key: *mut c_void, m: *mut seq_file) {
pub static mut l: *mut c_void = core::ptr::null_mut();
    let mut pptr = core::ptr::null_mut();
    let mut cpu = 0;
    rcu_read_lock();
    l = __htab_map_lookup_elem(map, key);
    if (!l) {
    rcu_read_unlock();
    return;
    }
    btf_type_seq_show(map.btf, map.btf_key_type_id, key, m);
    seq_puts(m, ": {\n");
    pptr = htab_elem_get_ptr(l, map.key_size);
    for_each_possible_cpu(cpu) {
    seq_printf(m, "\tcpu%d: ", cpu);
    btf_type_seq_show(map.btf, map.btf_value_type_id,
    per_cpu_ptr(pptr, cpu), m);
    seq_putc(m, '\n');
    }
    seq_puts(m, "}\n");
    rcu_read_unlock();
    }
pub static mut bpf_map_ops: usize = 0;
pub static mut bpf_map_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn fd_htab_map_alloc_check(attr: *mut union bpf_attr) -> c_int {
    if (attr.value_size != sizeof!(u32)) {
    return -EINVAL;
    }
    return htab_map_alloc_check(attr);
    }
#[no_mangle]
unsafe extern "C" fn fd_htab_map_free(map: *mut bpf_map) {
    let mut htab = container_of!(map, bpf_htab, map);
pub static mut n: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut l: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < htab.n_buckets) {
    head = select_bucket(htab, i);
    hlist_nulls_for_each_entry_safe(l, n, head, hash_node) {
    let mut ptr = fd_htab_map_get_ptr(map, l);
    map.ops.map_fd_put_ptr(map, ptr, false);
    }
    }
    htab_map_free(map);
    }
// only called from syscall
#[no_mangle]
pub unsafe extern "C" fn bpf_fd_htab_map_lookup_elem(map: *mut bpf_map, key: *mut c_void, value: *mut u32) -> c_int {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    if (!map.ops.map_fd_sys_lookup_elem) {
    return -ENOTSUPP;
    }
    rcu_read_lock();
    ptr = htab_map_lookup_elem(map, key);
    if (ptr) {
// value = map->ops->map_fd_sys_lookup_elem(READ_ONCE(*ptr));
    }
    else {
    ret = -ENOENT;
    }
    rcu_read_unlock();
    return ret;
    }
// Only called from syscall
#[no_mangle]
pub unsafe extern "C" fn bpf_fd_htab_map_update_elem(map: *mut bpf_map, map_file: *mut file, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_int {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    ptr = map.ops.map_fd_get_ptr(map, map_file, *value);
    if (IS_ERR(ptr)) {
    return PTR_ERR(ptr);
    }
// The htab bucket lock is always held during update operations in fd
// htab map, and the following rcu_read_lock() is only used to avoid
// the WARN_ON_ONCE in htab_map_update_elem_in_place().
//
    rcu_read_lock();
    ret = htab_map_update_elem_in_place(map, key, &ptr, map_flags, false, false);
    rcu_read_unlock();
    if (ret) {
    map.ops.map_fd_put_ptr(map, ptr, false);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn htab_of_map_alloc(attr: *mut union bpf_attr) -> *mut c_void {
    let mut map = core::ptr::null_mut();
    let mut inner_map_meta = core::ptr::null_mut();
    inner_map_meta = bpf_map_meta_alloc(attr.inner_map_fd);
    if (IS_ERR(inner_map_meta)) {
    return inner_map_meta;
    }
    map = htab_map_alloc(attr);
    if (IS_ERR(map)) {
    bpf_map_meta_free(inner_map_meta);
    return map;
    }
    map.inner_map_meta = inner_map_meta;
    return map;
    }
#[no_mangle]
pub unsafe extern "C" fn htab_of_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    let mut inner_map = htab_map_lookup_elem(map, key);
    if (!inner_map) {
    return core::ptr::null_mut();
    }
    return READ_ONCE(*inner_map);
    }
#[no_mangle]
pub unsafe extern "C" fn htab_of_map_gen_lookup(map: *mut bpf_map, insn_buf: *mut bpf_insn) -> c_int {
    let mut insn = insn_buf;
pub static mut ret: c_int = 0;
    BUILD_BUG_ON!(!__same_type(&__htab_map_lookup_elem,
    (void *(bpf_map *map, void *key))core::ptr::null_mut()));
// insn++ = BPF_EMIT_CALL(__htab_map_lookup_elem);
// insn++ = BPF_JMP_IMM(BPF_JEQ, ret, 0, 2);
// insn++ = BPF_ALU64_IMM(BPF_ADD, ret,
    offsetof(htab_elem, key) +
    round_up(map.key_size, 8));
// insn++ = BPF_LDX_MEM(BPF_DW, ret, ret, 0);
    return insn - insn_buf;
    }
#[no_mangle]
unsafe extern "C" fn htab_of_map_free(map: *mut bpf_map) {
    bpf_map_meta_free(map.inner_map_meta);
    fd_htab_map_free(map);
    }
pub static mut bpf_map_ops: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rhtab_elem {
    pub node: rhash_head,
// key bytes, then value bytes follow
    pub __aligned(8): u8 data[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_rhtab {
    pub map: bpf_map,
    pub ht: rhashtable,
    pub ma: bpf_mem_alloc,
    pub elem_size: u32,
    pub freeing_internal: bool,
}

pub static mut rhashtable_params: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn rhtab_elem_value(l: *mut rhtab_elem, key_size: u32) -> *mut c_void {
    return l.data + round_up(key_size, 8);
    }
// Specialize hash function and objcmp for long sized key
    static __always_inline int rhtab_key_cmp_long(rhashtable_compare_arg *arg,
    const void *ptr)
    {
pub static mut key1: c_ulong = 0;
    let mut key2 = ptr;
    return key1 != *key2.data;
    }
#[no_mangle]
unsafe extern "C" fn rhtab_hashfn_long(data: *const c_void, len: u32, seed: u32) -> __always_inline u32 {
pub static mut k: u64 = 0;
    return (u32)(k ^ (k >> 32)) ^ seed;
    }
pub static mut rhashtable_params: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn rhtab_map_alloc(attr: *mut union bpf_attr) -> *mut c_void {
pub static mut params: usize = 0;
pub static mut rhtab: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    rhtab = bpf_map_area_alloc(sizeof!(*rhtab), NUMA_NO_NODE);
    if (!rhtab) {
    return ERR_PTR(-ENOMEM);
    }
    bpf_map_init_from_attr(&rhtab.map, attr);
    if (rhtab.map.max_entries > 1UL << 31) {
    err = -E2BIG;
// goto;
    }
    rhtab.elem_size = sizeof!(rhtab_elem) + round_up(rhtab.map.key_size, 8) +
    round_up(rhtab.map.value_size, 8);
    params = rhtab_params;
    params.key_len = rhtab.map.key_size;
    params.nelem_hint = (u32)attr.map_extra;
    params.automatic_shrinking = true;
    if (rhtab.map.key_size == sizeof!(long)) {
    params.hashfn = rhtab_hashfn_long;
    params.obj_cmpfn = rhtab_key_cmp_long;
    }
    err = rhashtable_init(&rhtab.ht, &params);
    if (err) {
// goto;
    }
// Set max_elems after rhashtable_init() since init zeroes the struct
    rhtab.ht.max_elems = rhtab.map.max_entries;
    err = bpf_mem_alloc_init(&rhtab.ma, rhtab.elem_size, false);
    if (err) {
// goto;
    }
    return &rhtab.map;
// label;
    rhashtable_destroy(&rhtab.ht);
// label;
    bpf_map_area_free(rhtab);
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn rhtab_map_alloc_check(attr: *mut union bpf_attr) -> c_int {
    if (!(attr.map_flags & BPF_F_NO_PREALLOC)) {
    return -EINVAL;
    }
    if (attr.map_flags & BPF_F_ZERO_SEED) {
    return -EINVAL;
    }
    if (attr.key_size > U16_MAX) {
    return -E2BIG;
    }
    if (attr.map_extra >> 32) {
    return -EINVAL;
    }
    if ((u32)attr.map_extra > U16_MAX) {
    return -E2BIG;
    }
    if ((u32)attr.map_extra > attr.max_entries) {
    return -EINVAL;
    }
    return htab_map_alloc_check(attr);
    }
#[no_mangle]
pub unsafe extern "C" fn rhtab_check_and_free_fields(rhtab: *mut bpf_rhtab, elem: *mut rhtab_elem) {
    if (IS_ERR_OR_NULL(rhtab.map.record)) {
    return;
    }
    bpf_obj_free_fields(rhtab.map.record,
    rhtab_elem_value(elem, rhtab.map.key_size));
    }
#[no_mangle]
unsafe extern "C" fn rhtab_mem_dtor(obj: *mut c_void, ctx: *mut c_void) {
    let mut hrec = ctx;
    let mut elem = obj;
    if (IS_ERR_OR_NULL(hrec.record)) {
    return;
    }
    bpf_obj_free_fields(hrec.record,
    rhtab_elem_value(elem, hrec.key_size));
    }
#[no_mangle]
unsafe extern "C" fn rhtab_free_elem(ptr: *mut c_void, arg: *mut c_void) {
    let mut rhtab = arg;
    let mut elem = ptr;
    bpf_map_free_internal_structs(&rhtab.map, rhtab_elem_value(elem, rhtab.map.key_size));
    bpf_mem_cache_free_rcu(&rhtab.ma, elem);
    }
#[no_mangle]
unsafe extern "C" fn rhtab_map_free(map: *mut bpf_map) {
    let mut rhtab = container_of!(map, bpf_rhtab, map);
    rhashtable_free_and_destroy(&rhtab.ht, rhtab_free_elem, rhtab);
    bpf_mem_alloc_destroy(&rhtab.ma);
    bpf_map_area_free(rhtab);
    }
#[no_mangle]
pub unsafe extern "C" fn rhtab_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    let mut rhtab = container_of!(map, bpf_rhtab, map);
// Hold RCU lock in case sleepable program calls via gen_lookup
    guard(rcu)();
    if (map.key_size == sizeof!(long)) {
    return rhashtable_lookup_likely(&rhtab.ht, key, rhtab_params_long);
    }
    return rhashtable_lookup_likely(&rhtab.ht, key, rhtab_params);
    }
#[no_mangle]
pub unsafe extern "C" fn rhtab_map_lookup_elem(map: *mut bpf_map, RCU: *mut voidkey) __must_hold() -> *mut c_void {
pub static mut l: *mut c_void = core::ptr::null_mut();
    l = rhtab_lookup_elem(map, key);
    return l ? rhtab_elem_value(l, map.key_size) : core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn rhtab_read_elem_value(map: *mut bpf_map, dst: *mut c_void, elem: *mut rhtab_elem, flags: u64) {
    let mut src = rhtab_elem_value(elem, map.key_size);
    if (flags & BPF_F_LOCK) {
    copy_map_value_locked(map, dst, src, true);
    }
    else {
    copy_map_value(map, dst, src);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rhtab_delete_elem(rhtab: *mut bpf_rhtab, elem: *mut rhtab_elem, copy: *mut c_void, flags: u64) -> c_int {
    let mut err = 0;
//
// disable_instrumentation() mitigates the deadlock for programs running in NMI context.
// rhashtable locks bucket with local_irq_save(). Only NMI programs may reenter
// rhashtable code, bpf_disable_instrumentation() disables programs running in NMI, except
// raw tracepoints, which we don't have in rhashtable.
//
    bpf_disable_instrumentation();
    if (rhtab.map.key_size == sizeof!(long)) {
    err = rhashtable_remove_fast(&rhtab.ht, &elem.node, rhtab_params_long);
    }
    else {
    err = rhashtable_remove_fast(&rhtab.ht, &elem.node, rhtab_params);
    }
    bpf_enable_instrumentation();
    if (err) {
    return err;
    }
    if (copy) {
    rhtab_read_elem_value(&rhtab.map, copy, elem, flags);
    check_and_init_map_value(&rhtab.map, copy);
    }
// Release internal structs: kptr, bpf_timer, task_work, wq
    rhtab_check_and_free_fields(rhtab, elem);
    bpf_mem_cache_free_rcu(&rhtab.ma, elem);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rhtab_map_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_long {
    let mut rhtab = container_of!(map, bpf_rhtab, map);
pub static mut elem: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    elem = rhtab_lookup_elem(map, key);
    if (!elem) {
    return -ENOENT;
    }
    return rhtab_delete_elem(rhtab, elem, core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn rhtab_map_lookup_and_delete_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> c_int {
    let mut rhtab = container_of!(map, bpf_rhtab, map);
pub static mut elem: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    err = bpf_map_check_op_flags(map, flags, BPF_F_LOCK);
    if (err) {
    return err;
    }
    guard(rcu)();
    elem = rhtab_lookup_elem(map, key);
    if (!elem) {
    return -ENOENT;
    }
    return rhtab_delete_elem(rhtab, elem, value, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn rhtab_map_update_existing(map: *mut bpf_map, elem: *mut rhtab_elem, value: *mut c_void, map_flags: u64) -> c_long {
    let mut rhtab = container_of!(map, bpf_rhtab, map);
    let mut old_val = rhtab_elem_value(elem, map.key_size);
    if (map_flags & BPF_NOEXIST) {
    return -EEXIST;
    }
    if (map_flags & BPF_F_LOCK) {
    copy_map_value_locked(map, old_val, value, false);
    }
    else {
    copy_map_value(map, old_val, value);
    }
//
// Torn reads: a concurrent reader without BPF_F_LOCK may observe
// the value mid-copy. Callers requiring consistent reads must use
// BPF_F_LOCK, matching arraymap semantics.
//
// copy_map_value() skips special-field offsets, so old timers
// kptrs/etc. still sit in the slot. Cancel them after the copy
// to match arraymap's update semantics.
//
    rhtab_check_and_free_fields(rhtab, elem);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rhtab_map_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_long {
    let mut rhtab = container_of!(map, bpf_rhtab, map);
    let mut elem = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    if (unlikely((map_flags & ~BPF_F_LOCK) > BPF_EXIST)) {
    return -EINVAL;
    }
    if ((map_flags & BPF_F_LOCK) && !btf_record_has_field(map.record, BPF_SPIN_LOCK)) {
    return -EINVAL;
    }
    guard(rcu)();
    elem = rhtab_lookup_elem(map, key);
    if (elem) {
    return rhtab_map_update_existing(map, elem, value, map_flags);
    }
    if (map_flags & BPF_EXIST) {
    return -ENOENT;
    }
//
// Reject new insertions while map_release_uref cleanup walks the
// table. Without this, new elements could keep triggering rehash
// and prevent the walk from terminating.
//
    if (READ_ONCE(rhtab.freeing_internal)) {
    return -EBUSY;
    }
// Check max_entries limit before inserting new element
    if (atomic_read(&rhtab.ht.nelems) >= map.max_entries) {
    return -E2BIG;
    }
    elem = bpf_mem_cache_alloc(&rhtab.ma);
    if (!elem) {
    return -ENOMEM;
    }
    memcpy(elem.data, key, map.key_size);
    copy_map_value(map, rhtab_elem_value(elem, map.key_size), value);
    check_and_init_map_value(map, rhtab_elem_value(elem, map.key_size));
// Prevent deadlock for NMI programs attempting to take bucket lock
    bpf_disable_instrumentation();
    if (map.key_size == sizeof!(long)) {
    tmp = rhashtable_lookup_get_insert_fast(&rhtab.ht, &elem.node, rhtab_params_long);
    }
    else {
    tmp = rhashtable_lookup_get_insert_fast(&rhtab.ht, &elem.node, rhtab_params);
    }
    bpf_enable_instrumentation();
    if (tmp) {
    bpf_mem_cache_free(&rhtab.ma, elem);
    if (IS_ERR(tmp)) {
    return PTR_ERR(tmp);
    }
    return rhtab_map_update_existing(map, tmp, value, map_flags);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rhtab_map_gen_lookup(map: *mut bpf_map, insn_buf: *mut bpf_insn) -> c_int {
    let mut insn = insn_buf;
pub static mut ret: c_int = 0;
    BUILD_BUG_ON!(!__same_type(&rhtab_lookup_elem,
    (void *(bpf_map *map, void *key)) core::ptr::null_mut()));
// insn++ = BPF_EMIT_CALL(rhtab_lookup_elem);
// insn++ = BPF_JMP_IMM(BPF_JEQ, ret, 0, 1);
// insn++ = BPF_ALU64_IMM(BPF_ADD, ret,
    offsetof(rhtab_elem, data) + round_up(map.key_size, 8));
    return insn - insn_buf;
    }
#[no_mangle]
pub unsafe extern "C" fn rhtab_map_check_btf(map: *mut bpf_map, btf: *mut btf, key_type: *mut btf_type, value_type: *mut btf_type) -> c_int {
    let mut rhtab = container_of!(map, bpf_rhtab, map);
    return bpf_ma_set_dtor(map, &rhtab.ma, rhtab_mem_dtor);
    }
#[no_mangle]
unsafe extern "C" fn rhtab_map_free_internal_structs(map: *mut bpf_map) {
    let mut rhtab = container_of!(map, bpf_rhtab, map);
pub static mut iter: usize = 0;
pub static mut elem: *mut c_void = core::ptr::null_mut();
    if (!bpf_map_has_internal_structs(map)) {
    return;
    }
//
// Block new insertions. Once observed, no new growth is triggered,
// so any in-flight rehash will drain and the walker is guaranteed
// to stop returning -EAGAIN. Treat -EAGAIN as "rehash in progress,
// retry"; do not wait for the worker.
//
    WRITE_ONCE(rhtab.freeing_internal, true);
    rhashtable_walk_enter(&rhtab.ht, &iter);
    rhashtable_walk_start(&iter);
    while ((elem = rhashtable_walk_next(&iter))) {
    if (IS_ERR(elem)) {
    if (PTR_ERR(elem) == -EAGAIN) {
    continue;
    }
    break;
    }
    bpf_map_free_internal_structs(map, rhtab_elem_value(elem, map.key_size));
    if (need_resched()) { /* Avoid stalls on large maps */ {
    rhashtable_walk_stop(&iter);
    }
    cond_resched();
    rhashtable_walk_start(&iter);
    }
    }
    rhashtable_walk_stop(&iter);
    rhashtable_walk_exit(&iter);
    WRITE_ONCE(rhtab.freeing_internal, false);
    }
#[no_mangle]
unsafe extern "C" fn rhtab_map_get_next_key(map: *mut bpf_map, key: *mut c_void, next_key: *mut c_void) -> c_int {
    let mut rhtab = container_of!(map, bpf_rhtab, map);
pub static mut elem: *mut c_void = core::ptr::null_mut();
    elem = rhashtable_next_key(&rhtab.ht, key);
// if not found, return the first key
    if (PTR_ERR(elem) == -ENOENT) {
    elem = rhashtable_next_key(&rhtab.ht, core::ptr::null_mut());
    }
    if (IS_ERR(elem)) {
    return PTR_ERR(elem);
    }
    if (!elem) {
    return -ENOENT;
    }
    memcpy(next_key, elem.data, map.key_size);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rhtab_map_seq_show_elem(map: *mut bpf_map, key: *mut c_void, m: *mut seq_file) {
pub static mut value: *mut c_void = core::ptr::null_mut();
// Guarantee that hashtab value is not freed
    guard(rcu)();
    value = rhtab_map_lookup_elem(map, key);
    if (!value) {
    return;
    }
    btf_type_seq_show(map.btf, map.btf_key_type_id, key, m);
    seq_puts(m, ": ");
    btf_type_seq_show(map.btf, map.btf_value_type_id, value, m);
    seq_putc(m, '\n');
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_each_rhash_elem(map: *mut bpf_map, callback_fn: bpf_callback_t, callback_ctx: *mut c_void, flags: u64) -> c_long {
    let mut rhtab = container_of!(map, bpf_rhtab, map);
    let mut prev_key = core::ptr::null_mut();
pub static mut elem: *mut c_void = core::ptr::null_mut();
pub static mut num_elems: c_int = 0;
pub static mut ret: u64 = 0;
    cant_migrate();
    if (flags != 0) {
    return -EINVAL;
    }
    rcu_read_lock();
//
// Best-effort iteration: if rhashtable is concurrently resized or
// elements are deleted/inserted, there may be missed or duplicate
// elements visited.
//
    while ((elem = rhashtable_next_key(&rhtab.ht, prev_key))) {
    if (IS_ERR(elem)) {
    break;
    }
    num_elems += 1;
    ret = callback_fn((u64)(long)map,
    (u64)(long)elem.data,
    (u64)(long)rhtab_elem_value(elem, map.key_size),
    (u64)(long)callback_ctx, 0);
    if (ret) {
    break;
    }
    prev_key = elem.data;	/* valid while RCU held */
    }
    rcu_read_unlock();
    return num_elems;
    }
#[no_mangle]
unsafe extern "C" fn rhtab_map_mem_usage(map: *const bpf_map) -> u64 {
    let mut rhtab = container_of!(map, bpf_rhtab, map);
    let mut num_entries = 0;
// Excludes rhashtable bucket overhead (~ nelems * sizeof! at 75% load).
    num_entries = atomic_read(&rhtab.ht.nelems);
    return sizeof!(bpf_rhtab) + rhtab.elem_size * num_entries;
    }
#[no_mangle]
pub unsafe extern "C" fn __rhtab_map_lookup_and_delete_batch(map: *mut bpf_map, attr: *mut union bpf_attr, uattr: *mut union bpf_attr, do_delete: bool) -> c_int {
    let mut rhtab = container_of!(map, bpf_rhtab, map);
    let mut uvalues = u64_to_user_ptr(attr.batch.values);
    let mut ukeys = u64_to_user_ptr(attr.batch.keys);
    let mut ubatch = u64_to_user_ptr(attr.batch.in_batch);
    let mut cursor = core::ptr::null_mut(), *keys = core::ptr::null_mut(), *values = core::ptr::null_mut(), *dst_key, *dst_val;
    let mut del_elems = core::ptr::null_mut();
    u32 max_count, total, key_size, value_size, i;
pub static mut has_next_cursor: bool = false;
pub static mut elem: *mut c_void = core::ptr::null_mut();
    u64 elem_map_flags, map_flags;
pub static mut ret: c_int = 0;
    elem_map_flags = attr.batch.elem_flags;
    ret = bpf_map_check_op_flags(map, elem_map_flags, BPF_F_LOCK);
    if (ret) {
    return ret;
    }
    map_flags = attr.batch.flags;
    if (map_flags) {
    return -EINVAL;
    }
    max_count = attr.batch.count;
    if (!max_count) {
    return 0;
    }
    if (put_user(0, &uattr.batch.count)) {
    return -EFAULT;
    }
    key_size = map.key_size;
    value_size = map.value_size;
    keys = kvmalloc_array(max_count, key_size, GFP_USER | __GFP_NOWARN);
    values = kvmalloc_array(max_count, value_size, GFP_USER | __GFP_NOWARN);
    if (do_delete) {
    del_elems = kvmalloc_array(max_count, sizeof!,
    GFP_USER | __GFP_NOWARN);
    }
    cursor = kmalloc(key_size, GFP_USER | __GFP_NOWARN);
    if (!keys || !values || !cursor || (do_delete && !del_elems)) {
    ret = -ENOMEM;
// goto;
    }
    if (ubatch && copy_from_user(cursor, ubatch, key_size)) {
    ret = -EFAULT;
// goto;
    }
    dst_key = keys;
    dst_val = values;
    total = 0;
    rcu_read_lock();
//
// Cursor stores the key of the next-to-process element (stashed by
// the previous batch). Look it up directly so the element is included
// here rather than skipped by next_key(). If the cursor was deleted
// concurrently (or by the previous do_delete batch), return -EAGAIN
// so userspace can distinguish a lost cursor from end-of-iteration
// (-ENOENT) and restart from a NULL cursor.
//
    if (ubatch) {
    elem = rhtab_lookup_elem(map, cursor);
    if (!elem) {
    rcu_read_unlock();
    ret = -EAGAIN;
// goto;
    }
    } else {
    elem = rhashtable_next_key(&rhtab.ht, core::ptr::null_mut());
    }
    while (elem && !IS_ERR(elem) && total < max_count) {
    memcpy(dst_key, elem.data, key_size);
    rhtab_read_elem_value(map, dst_val, elem, elem_map_flags);
    check_and_init_map_value(map, dst_val);
    if (do_delete) {
    del_elems[total] = elem;
    }
    elem = rhashtable_next_key(&rhtab.ht, dst_key);
    dst_key += key_size;
    dst_val += value_size;
    total += 1;
// Bail to userspace to avoid stalls.
    if (need_resched()) {
    break;
    }
    }
    if (elem && !IS_ERR(elem)) {
// Stash next-to-process key as cursor for the next batch.
    memcpy(cursor, elem.data, key_size);
    has_next_cursor = true;
    }
    if (do_delete) {
    for (i = 0; i < total; i++) {
    rhtab_delete_elem(rhtab, del_elems[i], core::ptr::null_mut(), 0);
    }
    }
    rcu_read_unlock();
    if (total == 0) {
    ret = -ENOENT;
// goto;
    }
// No more elements after this batch.
    if (!has_next_cursor) {
    ret = -ENOENT;
    }
    if (copy_to_user(ukeys, keys, (size_t)total * key_size) ||
    copy_to_user(uvalues, values, (size_t)total * value_size) ||
    put_user(total, &uattr.batch.count) ||
    (has_next_cursor &&
    copy_to_user(u64_to_user_ptr(attr.batch.out_batch),
    cursor, key_size))) {
    ret = -EFAULT;
// goto;
    }
// label;
    kfree(cursor);
    kvfree(keys);
    kvfree(values);
    kvfree(del_elems);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn rhtab_map_lookup_batch(map: *mut bpf_map, attr: *mut union bpf_attr, uattr: *mut union bpf_attr) -> c_int {
    return __rhtab_map_lookup_and_delete_batch(map, attr, uattr, false);
    }
#[no_mangle]
pub unsafe extern "C" fn rhtab_map_lookup_and_delete_batch(map: *mut bpf_map, attr: *mut union bpf_attr, uattr: *mut union bpf_attr) -> c_int {
    return __rhtab_map_lookup_and_delete_batch(map, attr, uattr, true);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_seq_rhash_map_info {
    pub map: *mut bpf_map,
    pub rhtab: *mut bpf_rhtab,
    pub iter: rhashtable_iter,
}

#[no_mangle]
pub unsafe extern "C" fn bpf_rhash_map_seq_start(seq: *mut seq_file, RCU: *mut loff_tpos)
    __acquires() -> *mut c_void {
    let mut info = seq.private;
pub static mut elem: *mut c_void = core::ptr::null_mut();
    rhashtable_walk_start(&info.iter);
//
// Re-deliver the element returned by walk_next() at the end of the
// previous read() — bpf_seq_read may have stopped before show()
// consumed it. Rehash rewinds the walker; retry on -EAGAIN.
//
    do {
    elem = rhashtable_walk_peek(&info.iter);
    } while (PTR_ERR(elem) == -EAGAIN);
    if (IS_ERR(elem)) {
    return core::ptr::null_mut();
    }
    if (elem && *pos == 0) {
    ++*pos;
    }
    return elem;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_rhash_map_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut info = seq.private;
pub static mut elem: *mut c_void = core::ptr::null_mut();
    ++*pos;
// Rehash rewinds the walker; retry until it stops returning -EAGAIN.
    do {
    elem = rhashtable_walk_next(&info.iter);
    } while (PTR_ERR(elem) == -EAGAIN);
    if (IS_ERR(elem)) {
    return core::ptr::null_mut();
    }
    return elem;
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_rhash_map_seq_show(seq: *mut seq_file, elem: *mut rhtab_elem) -> c_int {
    let mut info = seq.private;
pub static mut ctx: bpf_iter__bpf_map_elem = 0;
pub static mut meta: usize = 0;
pub static mut prog: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    meta.seq = seq;
    prog = bpf_iter_get_info(&meta, elem == core::ptr::null_mut());
    if (prog) {
    ctx.meta = &meta;
    ctx.map = info.map;
    if (elem) {
    ctx.key = elem.data;
    ctx.value = rhtab_elem_value(elem, info.map.key_size);
    }
    ret = bpf_iter_run_prog(prog, &ctx);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bpf_rhash_map_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    return __bpf_rhash_map_seq_show(seq, v);
    }
#[no_mangle]
unsafe extern "C" fn bpf_rhash_map_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    let mut info = seq.private;
    if (!v) {
    (void)__bpf_rhash_map_seq_show(seq, core::ptr::null_mut());
    }
    rhashtable_walk_stop(&info.iter);
    }
#[no_mangle]
unsafe extern "C" fn bpf_iter_init_rhash_map(priv_data: *mut c_void, aux: *mut bpf_iter_aux_info) -> c_int {
    let mut info = priv_data;
    let mut map = aux.map;
    bpf_map_inc_with_uref(map);
    info.map = map;
    info.rhtab = container_of!(map, bpf_rhtab, map);
    rhashtable_walk_enter(&info.rhtab.ht, &info.iter);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_iter_fini_rhash_map(priv_data: *mut c_void) {
    let mut info = priv_data;
    rhashtable_walk_exit(&info.iter);
    bpf_map_put_with_uref(info.map);
    }
pub static mut seq_operations: usize = 0;
pub static mut bpf_iter_seq_info: usize = 0;
    BTF_ID_LIST_SINGLE(rhtab_map_btf_ids, struct, bpf_rhtab)
pub static mut bpf_map_ops: usize = 0;