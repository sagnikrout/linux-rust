//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/stackmap.c
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
// Copyright (c) 2016 Facebook
//

    (BPF_F_NUMA_NODE | BPF_F_RDONLY | BPF_F_WRONLY |	
    BPF_F_STACK_BUILD_ID)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_map_bucket {
    pub fnode: pcpu_freelist_node,
    pub hash: u32,
    pub nr: u32,
    pub data: [u64; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_stack_map {
    pub map: bpf_map,
    pub elems: *mut c_void,
    pub freelist: pcpu_freelist,
    pub n_buckets: u32,
    pub __counted_by(n_buckets): *mut *mut stack_map_bucket buckets[],
}

#[no_mangle]
pub unsafe extern "C" fn stack_map_use_build_id(map: *mut bpf_map) -> bool {
    return (map.map_flags & BPF_F_STACK_BUILD_ID);
    }
#[no_mangle]
pub unsafe extern "C" fn stack_map_data_size(map: *mut bpf_map) -> c_int {
    return stack_map_use_build_id(map) ?
    sizeof!(bpf_stack_build_id) : sizeof!(u64);
    }
//
// stack_map_calculate_max_depth - Calculate maximum allowed stack trace depth
// @size:  Size of the buffer/map value in bytes
// @elem_size:  Size of each stack trace element
// @flags:  BPF stack trace flags (BPF_F_USER_STACK, BPF_F_USER_BUILD_ID, ...)
//
// Return: Maximum number of stack trace entries that can be safely stored
//
#[no_mangle]
unsafe extern "C" fn stack_map_calculate_max_depth(size: u32, elem_size: u32, flags: u64) -> u32 {
pub static mut skip: u32 = 0;
    let mut max_depth = 0;
pub static mut curr_sysctl_max_stack: u32 = 0;
    max_depth = size / elem_size;
    max_depth += skip;
    if (max_depth > curr_sysctl_max_stack) {
    return curr_sysctl_max_stack;
    }
    return max_depth;
    }
#[no_mangle]
unsafe extern "C" fn prealloc_elems_and_freelist(smap: *mut bpf_stack_map) -> c_int {
    u64 elem_size = sizeof!(stack_map_bucket) +
    (u64)smap.map.value_size;
    let mut err = 0;
    smap.elems = bpf_map_area_alloc(elem_size * smap.map.max_entries,
    smap.map.numa_node);
    if (!smap.elems) {
    return -ENOMEM;
    }
    err = pcpu_freelist_init(&smap.freelist);
    if (err) {
// goto;
    }
    pcpu_freelist_populate(&smap.freelist, smap.elems, elem_size,
    smap.map.max_entries);
    return 0;
// label;
    bpf_map_area_free(smap.elems);
    return err;
    }
// Called from syscall
#[no_mangle]
pub unsafe extern "C" fn stack_map_alloc(attr: *mut union bpf_attr) -> *mut c_void {
pub static mut value_size: u32 = 0;
pub static mut smap: *mut c_void = core::ptr::null_mut();
    u64 cost, n_buckets;
    let mut err = 0;
    if (attr.map_flags & ~STACK_CREATE_FLAG_MASK) {
    return ERR_PTR(-EINVAL);
    }
// check sanity of attributes
    if (attr.max_entries == 0 || attr.key_size != 4 ||
    value_size < 8 || value_size % 8) {
    return ERR_PTR(-EINVAL);
    }
    BUILD_BUG_ON!(sizeof!(bpf_stack_build_id) % sizeof!(u64));
    if (attr.map_flags & BPF_F_STACK_BUILD_ID) {
    if (value_size % sizeof!(bpf_stack_build_id) ||
    value_size / sizeof!(bpf_stack_build_id)
    > sysctl_perf_event_max_stack) {
    return ERR_PTR(-EINVAL);
    }
    } else if (value_size / 8 > sysctl_perf_event_max_stack) {
    return ERR_PTR(-EINVAL);
    }
// hash table size must be power of 2; roundup_pow_of_two() can overflow
// into UB on 32-bit arches, so check that first
//
    if (attr.max_entries > 1UL << 31) {
    return ERR_PTR(-E2BIG);
    }
    n_buckets = roundup_pow_of_two(attr.max_entries);
    cost = n_buckets * sizeof! + sizeof!(*smap);
    smap = bpf_map_area_alloc(cost, bpf_map_attr_numa_node(attr));
    if (!smap) {
    return ERR_PTR(-ENOMEM);
    }
    bpf_map_init_from_attr(&smap.map, attr);
    smap.n_buckets = n_buckets;
    err = get_callchain_buffers(sysctl_perf_event_max_stack);
    if (err) {
// goto;
    }
    err = prealloc_elems_and_freelist(smap);
    if (err) {
// goto;
    }
    return &smap.map;
// label;
    put_callchain_buffers();
// label;
    bpf_map_area_free(smap);
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn fetch_build_id(vma: *mut vm_area_struct, build_id: *mut c_uchar, may_fault: bool) -> c_int {
    return may_fault ? build_id_parse(vma, build_id, core::ptr::null_mut())
    : build_id_parse_nofault(vma, build_id, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn stack_map_build_id_set_ip(id: *mut bpf_stack_build_id) {
    id.status = BPF_STACK_BUILD_ID_IP;
    memset(id.build_id, 0, BUILD_ID_SIZE_MAX);
    }
#[no_mangle]
pub unsafe extern "C" fn stack_map_build_id_offset(vm_pgoff: c_ulong, vm_start: c_ulong, ip: u64) -> u64 {
    return (vm_pgoff << PAGE_SHIFT) + ip - vm_start;
    }
#[no_mangle]
pub unsafe extern "C" fn stack_map_build_id_set_valid(id: *mut bpf_stack_build_id, offset: u64, build_id: *mut c_uchar) {
    id.status = BPF_STACK_BUILD_ID_VALID;
    id.offset = offset;
    if (id.build_id != build_id) {
    memcpy(id.build_id, build_id, BUILD_ID_SIZE_MAX);
    }
    }
//
// A cached VMA lookup result. The range [vm_start, vm_end) is always set.
// vm_pgoff, file, build_id are set only when the build ID was resolved.
// Zero vm_end marks the slot empty. build_id aliases the id_offs[] entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_map_cached_vma {
    pub vm_start: c_ulong,
    pub vm_end: c_ulong,
    pub vm_pgoff: c_ulong,
//     pub /: *mut *mut *mut file file; / pinned in the sleepable path; NULL otherwise,
    pub build_id: *const c_uchar,
}

//
// Per stack_map_get_build_id_offset() call cache of the last VMA with a build ID
// resolved and the last VMA with no usable build ID. Adjacent stack frames tend
// to land in the same VMA or the same backing file, so caching the last result
// of each kind lets us skip unnecessary VMA lookups and build ID parse calls.
// Keeping the two slots independent means a build-ID-less VMA doesn't evict the
// last resolved build ID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_map_build_id_cache {
    pub resolved: stack_map_cached_vma,
    pub unresolved: stack_map_cached_vma,
}

//
// Fill @id from a cached range covering @ip. On a hit this writes @id (resolved
// range -> build ID + offset, unresolved range -> raw ip) and returns 0; on a
// miss it leaves @id untouched and returns -ENOENT.
//
#[no_mangle]
pub unsafe extern "C" fn stack_map_build_id_set_from_cache(cache: *mut stack_map_build_id_cache, id: *mut bpf_stack_build_id, ip: u64) -> c_int {
    unsigned long vm_start, vm_end, vm_pgoff;
    let mut offset = 0;
    vm_start = cache.resolved.vm_start;
    vm_end = cache.resolved.vm_end;
    if (vm_end && ip >= vm_start && ip < vm_end) {
    vm_pgoff = cache.resolved.vm_pgoff;
    offset = stack_map_build_id_offset(vm_pgoff, vm_start, ip);
    stack_map_build_id_set_valid(id, offset, cache.resolved.build_id);
    return 0;
    }
    vm_start = cache.unresolved.vm_start;
    vm_end = cache.unresolved.vm_end;
    if (vm_end && ip >= vm_start && ip < vm_end) {
    stack_map_build_id_set_ip(id);
    return 0;
    }
    return -ENOENT;
    }
//
// Record @vma's build ID as the last resolved one. @file is the pinned backing
// file in the sleepable path (released when evicted), or NULL otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn stack_map_build_id_cache_set_resolved(cache: *mut stack_map_build_id_cache, file: *mut file, build_id: *mut c_uchar, vm_start: c_ulong, vm_end: c_ulong, vm_pgoff: c_ulong) {
    if (cache.resolved.file) {
    fput(cache.resolved.file);
    }
    cache.resolved = (stack_map_cached_vma){
    .vm_start = vm_start,
    .vm_end = vm_end,
    .vm_pgoff = vm_pgoff,
    .file = file,
    .build_id = build_id,
    };
    }
// Record [vm_start, vm_end) as a range with no usable build ID.
#[no_mangle]
pub unsafe extern "C" fn stack_map_build_id_cache_set_unresolved(cache: *mut stack_map_build_id_cache, vm_start: c_ulong, vm_end: c_ulong) {
    cache.unresolved = (stack_map_cached_vma){
    .vm_start = vm_start,
    .vm_end = vm_end,
    };
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_map_vma_lock {
    pub vma: *mut vm_area_struct,
    pub mm: *mut mm_struct,
}

//
// Acquire a stable read-side reference on the VMA covering @ip.
//
// With CONFIG_PER_VMA_LOCK=y this returns a VMA with its per-VMA read
// lock held and mmap_lock dropped, so the caller may sleep.
//
// With CONFIG_PER_VMA_LOCK=n it returns a VMA with mmap_lock still
// held; the caller must snapshot any fields it needs and pin vm_file
// with get_file() before stack_map_unlock_vma() drops mmap_lock, as
// the VMA may be split, merged, or freed after that.
//
// Returns NULL on failure, in which case no lock is held.
//
#[no_mangle]
pub unsafe extern "C" fn stack_map_lock_vma(lock: *mut stack_map_vma_lock, ip: c_ulong) -> *mut c_void {
    let mut mm = lock.mm;
pub static mut vma: *mut c_void = core::ptr::null_mut();
// noop under !CONFIG_PER_VMA_LOCK
    vma = lock_vma_under_rcu(mm, ip);
    if (vma) {
    lock.vma = vma;
    return vma;
    }
//
// Taking mmap_read_lock() is unsafe here, because the caller BPF
// program might already hold it, causing a deadlock.
//
    if (!mmap_read_trylock(mm)) {
    return core::ptr::null_mut();
    }
    vma = vma_lookup(mm, ip);
    if (!vma) {
    mmap_read_unlock(mm);
    return core::ptr::null_mut();
    }

    if (!vma_start_read_locked(vma)) {
    mmap_read_unlock(mm);
    return core::ptr::null_mut();
    }
    mmap_read_unlock(mm);

    lock.vma = vma;
    return vma;
    }
#[no_mangle]
unsafe extern "C" fn stack_map_unlock_vma(lock: *mut stack_map_vma_lock) {

    vma_end_read(lock.vma);

    mmap_read_unlock(lock.mm);

    lock.vma = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn stack_map_get_build_id_offset_sleepable(id_offs: *mut bpf_stack_build_id, trace_nr: u32) {
pub static mut lock: stack_map_vma_lock = 0;
pub static mut cache: stack_map_build_id_cache = 0;
    let mut res = &cache.resolved;
    unsigned long vm_pgoff, vm_start, vm_end;
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut offset = 0;
    let mut ip = 0;
    while (i < trace_nr) {
    ip = READ_ONCE(id_offs[i].ip);
    if (!stack_map_build_id_set_from_cache(&cache, &id_offs[i], ip)) {
    continue;
    }
    vma = stack_map_lock_vma(&lock, ip);
    if (!vma) {
    stack_map_build_id_set_ip(&id_offs[i]);
    continue;
    }
    vm_pgoff = vma.vm_pgoff;
    vm_start = vma.vm_start;
    vm_end = vma.vm_end;
    if (vma_is_anonymous(vma) || !vma.vm_file) {
    stack_map_unlock_vma(&lock);
    stack_map_build_id_set_ip(&id_offs[i]);
    stack_map_build_id_cache_set_unresolved(&cache, vm_start, vm_end);
    continue;
    }
    file = vma.vm_file;
    offset = stack_map_build_id_offset(vm_pgoff, vm_start, ip);
//
// Same backing file as the last resolved VMA (another mapping
// of the same ELF binary): reuse its build_id without re-parsing.
//
    if (file == res.file) {
    stack_map_unlock_vma(&lock);
    stack_map_build_id_set_valid(&id_offs[i], offset, res.build_id);
    res.vm_start = vm_start;
    res.vm_end = vm_end;
    res.vm_pgoff = vm_pgoff;
    continue;
    }
    file = get_file(file);
    stack_map_unlock_vma(&lock);
// build_id_parse_file() may block on filesystem reads
    if (build_id_parse_file(file, id_offs[i].build_id, core::ptr::null_mut())) {
    stack_map_build_id_set_ip(&id_offs[i]);
    fput(file);
    stack_map_build_id_cache_set_unresolved(&cache, vm_start, vm_end);
    continue;
    }
    stack_map_build_id_set_valid(&id_offs[i], offset, id_offs[i].build_id);
    stack_map_build_id_cache_set_resolved(&cache, file, id_offs[i].build_id,
    vm_start, vm_end, vm_pgoff);
    }
    if (res.file) {
    fput(res.file);
    }
    }
//
// Expects all id_offs[i].ip values to be set to correct initial IPs.
// They will be subsequently:
// - either adjusted in place to a file offset, if build ID fetching
// succeeds; in this case id_offs[i].build_id is set to correct build ID,
// and id_offs[i].status is set to BPF_STACK_BUILD_ID_VALID;
// - or IP will be kept intact, if build ID fetching failed; in this case
// id_offs[i].build_id is zeroed out and id_offs[i].status is set to
// BPF_STACK_BUILD_ID_IP.
//
#[no_mangle]
pub unsafe extern "C" fn stack_map_get_build_id_offset(id_offs: *mut bpf_stack_build_id, trace_nr: u32, user: bool, may_fault: bool) {
pub static mut work: *mut c_void = core::ptr::null_mut();
pub static mut has_user_ctx: bool = false;
pub static mut cache: stack_map_build_id_cache = 0;
pub static mut vma: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (may_fault && has_user_ctx) {
    stack_map_get_build_id_offset_sleepable(id_offs, trace_nr);
    return;
    }
    if (!has_user_ctx) {
// goto;
    }
    work = bpf_mmap_unlock_guard_get();
    if (IS_ERR(work)) {
// goto;
    }
    if (!mmap_read_trylock(current.mm)) {
    bpf_mmap_unlock_guard_put(work);
// goto;
    }
    while (i < trace_nr) {
pub static mut ip: u64 = 0;
    if (!stack_map_build_id_set_from_cache(&cache, &id_offs[i], ip)) {
    continue;
    }
    vma = find_vma(current.mm, ip);
    if (!vma || vma_is_anonymous(vma) ||
    fetch_build_id(vma, id_offs[i].build_id, may_fault)) {
// per entry fall back to ips; cache build-ID-less range
    stack_map_build_id_set_ip(&id_offs[i]);
    if (vma) {
    stack_map_build_id_cache_set_unresolved(&cache,
    vma.vm_start, vma.vm_end);
    }
    continue;
    }
//
// mmap_lock is held for the whole loop, so the cached VMA
// fields stay valid; no file pinning is needed here.
//
    stack_map_build_id_set_valid(&id_offs[i],
    stack_map_build_id_offset(vma.vm_pgoff, vma.vm_start, ip),
    id_offs[i].build_id);
    stack_map_build_id_cache_set_resolved(&cache, core::ptr::null_mut(), id_offs[i].build_id,
    vma.vm_start, vma.vm_end,
    vma.vm_pgoff);
    }
    bpf_mmap_unlock_mm(work, current.mm);
    return;
// label;
// cannot access current->mm, fall back to ips
    for (i = 0; i < trace_nr; i++) {
    stack_map_build_id_set_ip(&id_offs[i]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn get_callchain_entry_for_task(task: *mut task_struct, max_depth: u32) -> *mut c_void {

pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut rctx = 0;
    entry = get_callchain_entry(&rctx);
    if (!entry) {
    return core::ptr::null_mut();
    }
    entry.nr = stack_trace_save_tsk(task, entry.ip,
    max_depth, 0);
// stack_trace_save_tsk() works on unsigned long array, while
// perf_callchain_entry uses u64 array. For 32-bit systems, it is
// necessary to fix this mismatch.
//
    if (__BITS_PER_LONG != 64) {
    let mut from =  entry.ip;
    let mut to = entry.ip;
    let mut i = 0;
// copy data from the end to avoid using extra buffer
    for (i = entry.nr - 1; i >= 0; i--) {
    to[i] = (u64)(from[i]);
    }
    }
    put_callchain_entry(rctx);
    return entry;

    return core::ptr::null_mut();

    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stackid {
    pub bucket: *mut stack_map_bucket,
    pub ips: *const u64,
    pub nr: u32,
    pub len: u32,
    pub hash: u32,
    pub id: u32,
    pub hash_matches: bool,
}

#[no_mangle]
pub unsafe extern "C" fn stackid_init(stackid: *mut stackid, map: *mut bpf_map, trace: *mut perf_callchain_entry, trace_nr: u32, flags: u64) -> c_int {
    let mut smap = container_of!(map, bpf_stack_map, map);
pub static mut skip: u32 = 0;
    let mut max_depth = 0;
    if (trace_nr <= skip) {
// skipping more than usable stack trace
    return -EFAULT;
    }
    max_depth = stack_map_calculate_max_depth(map.value_size, stack_map_data_size(map), flags);
    stackid.nr = min_t(u32, trace_nr - skip, max_depth - skip);
    stackid.len = stackid.nr * sizeof!(u64);
    stackid.ips = trace.ip + skip;
    stackid.hash = jhash2(stackid.ips, stackid.len / sizeof!(u32), 0);
    stackid.id = stackid.hash & (smap.n_buckets - 1);
    stackid.bucket = READ_ONCE(smap.buckets[stackid.id]);
    stackid.hash_matches = stackid.bucket && stackid.bucket.hash == stackid.hash;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn stackid_fastpath(stackid: *mut stackid, map: *mut bpf_map, trace: *mut perf_callchain_entry, trace_nr: u32, flags: u64) -> c_int {
    let mut err = 0;
    err = stackid_init(stackid, map, trace, trace_nr, flags);
    if (err) {
    return err;
    }
// fast cmp
    if (stackid.hash_matches && flags & BPF_F_FAST_STACK_CMP) {
    return stackid.id;
    }
    if (stack_map_use_build_id(map)) {
    return -ENOENT;
    }
    if (stackid.hash_matches && stackid.bucket.nr == stackid.nr &&
    memcmp(stackid.bucket.data, stackid.ips, stackid.len) == 0) {
    return stackid.id;
    }
    if (stackid.bucket && !(flags & BPF_F_REUSE_STACKID)) {
    return -EEXIST;
    }
    return -ENOENT;
    }
#[no_mangle]
pub unsafe extern "C" fn stackid_new_bucket(stackid: *mut stackid, map: *mut bpf_map) -> *mut c_void {
    let mut smap = container_of!(map, bpf_stack_map, map);
pub static mut id_offs: *mut c_void = core::ptr::null_mut();
pub static mut bucket: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    bucket =  pcpu_freelist_pop(&smap.freelist);
    if (unlikely(!bucket)) {
    return core::ptr::null_mut();
    }
    if (stack_map_use_build_id(map)) {
    id_offs = bucket.data;
    for (i = 0; i < stackid.nr; i++) {
    id_offs[i].ip = stackid.ips[i];
    }
    } else {
    memcpy(bucket.data, stackid.ips, stackid.len);
    }
    bucket.hash = stackid.hash;
    bucket.nr = stackid.nr;
    return bucket;
    }
#[no_mangle]
pub unsafe extern "C" fn stackid_install(stackid: *mut stackid, map: *mut bpf_map, new_bucket: *mut stack_map_bucket, flags: u64) -> c_long {
    let mut smap = container_of!(map, bpf_stack_map, map);
pub static mut user: bool = false;
pub static mut old_bucket: *mut c_void = core::ptr::null_mut();
    let mut trace_len = 0;
    if (stack_map_use_build_id(map)) {
pub static mut id_offs: *mut c_void = core::ptr::null_mut();
    id_offs = new_bucket.data;
    stack_map_get_build_id_offset(id_offs, stackid.nr, user, false /* !may_fault */);
    trace_len = stackid.nr * sizeof!(bpf_stack_build_id);
    if (stackid.hash_matches && stackid.bucket.nr == stackid.nr &&
    memcmp(stackid.bucket.data, new_bucket.data, trace_len) == 0) {
    pcpu_freelist_push(&smap.freelist, &new_bucket.fnode);
    return stackid.id;
    }
    if (stackid.bucket && !(flags & BPF_F_REUSE_STACKID)) {
    pcpu_freelist_push(&smap.freelist, &new_bucket.fnode);
    return -EEXIST;
    }
    }
    old_bucket = xchg(&smap.buckets[stackid.id], new_bucket);
    if (old_bucket) {
    pcpu_freelist_push(&smap.freelist, &old_bucket.fnode);
    }
    return stackid.id;
    }
    BPF_CALL_3(bpf_get_stackid, pt_regs *, regs, bpf_map *, map,
    u64, flags)
    {
pub static mut elem_size: u32 = 0;
pub static mut user: bool = false;
pub static mut new_bucket: *mut c_void = core::ptr::null_mut();
pub static mut trace: *mut c_void = core::ptr::null_mut();
pub static mut stackid: usize = 0;
pub static mut kernel: bool = false;
    let mut max_depth = 0;
    let mut err = 0;
    if (unlikely(flags & ~(BPF_F_SKIP_FIELD_MASK | BPF_F_USER_STACK |
    BPF_F_FAST_STACK_CMP | BPF_F_REUSE_STACKID))) {
    return -EINVAL;
    }
    max_depth = stack_map_calculate_max_depth(map.value_size, elem_size, flags);
    scoped_guard(preempt) {
    trace = get_perf_callchain(regs, kernel, user, max_depth,
    false, false, 0);
    if (unlikely(!trace)) {
// couldn't fetch the stack trace
    return -EFAULT;
    }
    err = stackid_fastpath(&stackid, map, trace, trace.nr, flags);
    if (err != -ENOENT) {
    return err;
    }
    new_bucket = stackid_new_bucket(&stackid, map);
    if (!new_bucket) {
    return -ENOMEM;
    }
    }
    return stackid_install(&stackid, map, new_bucket, flags);
    }
pub static mut bpf_func_proto: usize = 0;
#[no_mangle]
unsafe extern "C" fn count_kernel_ip(trace: *const perf_callchain_entry) -> __u64 {
pub static mut nr_kernel: __u64 = 0;
    while (nr_kernel < trace.nr) {
    if (trace.ip[nr_kernel] == PERF_CONTEXT_USER) {
    break;
    }
    nr_kernel += 1;
    }
    return nr_kernel;
    }
    BPF_CALL_3(bpf_get_stackid_pe, bpf_perf_event_data_kern *, ctx, bpf_map *, map, u64, flags)
    {
pub static mut trace: *mut c_void = core::ptr::null_mut();
    let mut event = ctx.event;
pub static mut new_bucket: *mut c_void = core::ptr::null_mut();
pub static mut stackid: usize = 0;
    let mut kernel = 0;
    let mut user = 0;
    let mut nr_kernel = 0;
    let mut trace_nr = 0;
    let mut ret = 0;
// perf_sample_data doesn't have callchain, use bpf_get_stackid
    if (!(event.attr.sample_type & PERF_SAMPLE_CALLCHAIN)) {
    return bpf_get_stackid((unsigned long)(ctx.regs),
    (unsigned long) map, flags, 0, 0);
    }
    if (unlikely(flags & ~(BPF_F_SKIP_FIELD_MASK | BPF_F_USER_STACK |
    BPF_F_FAST_STACK_CMP | BPF_F_REUSE_STACKID))) {
    return -EINVAL;
    }
    user = flags & BPF_F_USER_STACK;
    kernel = !user;
    trace = ctx.data.callchain;
    if (unlikely(!trace)) {
    return -EFAULT;
    }
    nr_kernel = count_kernel_ip(trace);
    if (kernel) {
    trace_nr = nr_kernel;
    } else { /* user */
pub static mut skip: u64 = 0;
    trace_nr = trace.nr;
    skip += nr_kernel;
    if (skip > BPF_F_SKIP_FIELD_MASK) {
    return -EFAULT;
    }
    flags = (flags & ~BPF_F_SKIP_FIELD_MASK) | skip;
    }
    ret = stackid_fastpath(&stackid, map, trace, trace_nr, flags);
    if (ret != -ENOENT) {
    return ret;
    }
    new_bucket = stackid_new_bucket(&stackid, map);
    if (new_bucket) {
    return stackid_install(&stackid, map, new_bucket, flags);
    }
    return -ENOMEM;
    }
pub static mut bpf_func_proto: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn callchain_store(trace: *mut perf_callchain_entry, trace_nr: u32, buf: *mut c_void, elem_size: u32, flags: u64) -> u32 {
pub static mut user_build_id: bool = false;
pub static mut skip: u32 = 0;
pub static mut ips: *mut c_void = core::ptr::null_mut();
    let mut copy_len = 0;
    trace_nr = trace_nr - skip;
    copy_len = trace_nr * elem_size;
    ips = trace.ip + skip;
    if (user_build_id) {
    let mut id_offs = buf;
    for (u32 i = 0; i < trace_nr; i++) {
    id_offs[i].ip = ips[i];
    }
    } else {
    memcpy(buf, ips, copy_len);
    }
    return trace_nr;
    }
#[no_mangle]
pub unsafe extern "C" fn callchain_finalize(buf: *mut c_void, size: u32, trace_nr: u32, elem_size: u32, flags: u64, may_fault: bool) -> c_long {
pub static mut user_build_id: bool = false;
pub static mut user: bool = false;
pub static mut copy_len: u32 = 0;
    if (user_build_id) {
    stack_map_get_build_id_offset(buf, trace_nr, user, may_fault);
    }
    if (size > copy_len) {
    memset(buf + copy_len, 0, size - copy_len);
    }
    return copy_len;
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_get_stack(regs: *mut pt_regs, task: *mut task_struct, buf: *mut c_void, size: u32, flags: u64, may_fault: bool) -> c_long {
pub static mut user_build_id: bool = false;
pub static mut crosstask: bool = false;
pub static mut skip: u32 = 0;
pub static mut user: bool = false;
pub static mut trace: *mut c_void = core::ptr::null_mut();
    u32 trace_nr, elem_size, max_depth;
pub static mut kernel: bool = false;
pub static mut err: c_int = 0;
    if (unlikely(flags & ~(BPF_F_SKIP_FIELD_MASK | BPF_F_USER_STACK |
    BPF_F_USER_BUILD_ID))) {
// goto;
    }
    if (kernel && user_build_id) {
// goto;
    }
    elem_size = user_build_id ? sizeof!(bpf_stack_build_id) : sizeof!(u64);
    if (unlikely(size % elem_size)) {
// goto;
    }
// cannot get valid user stack for task without user_mode regs
    if (task && user && !user_mode(regs)) {
// goto;
    }
// get_perf_callchain does not support crosstask user stack walking
// but returns an empty stack instead of NULL.
//
    if (crosstask && user) {
    err = -EOPNOTSUPP;
// goto;
    }
    max_depth = stack_map_calculate_max_depth(size, elem_size, flags);
    preempt_disable();
    if (may_fault) {
    rcu_read_lock(); /* need RCU for perf's callchain below */
    }
    if (kernel && task) {
    trace = get_callchain_entry_for_task(task, max_depth);
    } else {
    trace = get_perf_callchain(regs, kernel, user, max_depth,
    crosstask, false, 0);
    }
    if (unlikely(!trace) || trace.nr < skip) {
    if (may_fault) {
    rcu_read_unlock();
    }
    preempt_enable();
// goto;
    }
    trace_nr = callchain_store(trace, trace.nr, buf, elem_size, flags);
// trace should not be dereferenced after this point
    if (may_fault) {
    rcu_read_unlock();
    }
    preempt_enable();
    return callchain_finalize(buf, size, trace_nr, elem_size, flags, may_fault);
// label;
    err = -EFAULT;
// label;
    memset(buf, 0, size);
    return err;
    }
    BPF_CALL_4(bpf_get_stack, pt_regs *, regs, void *, buf, u32, size,
    u64, flags)
    {
    return __bpf_get_stack(regs, core::ptr::null_mut(), buf, size, flags, false /* !may_fault */);
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_4(bpf_get_stack_sleepable, pt_regs *, regs, void *, buf, u32, size,
    u64, flags)
    {
    return __bpf_get_stack(regs, core::ptr::null_mut(), buf, size, flags, true /* may_fault */);
    }
pub static mut bpf_func_proto: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn __bpf_get_task_stack(task: *mut task_struct, buf: *mut c_void, size: u32, flags: u64, may_fault: bool) -> c_long {
pub static mut regs: *mut c_void = core::ptr::null_mut();
pub static mut res: c_long = 0;
    if (!try_get_task_stack(task)) {
    memset(buf, 0, size);
    return -EFAULT;
    }
    regs = task_pt_regs(task);
    if (regs) {
    res = __bpf_get_stack(regs, task, buf, size, flags, may_fault);
    }
    else {
    memset(buf, 0, size);
    }
    put_task_stack(task);
    return res;
    }
    BPF_CALL_4(bpf_get_task_stack, task_struct *, task, void *, buf,
    u32, size, u64, flags)
    {
    return __bpf_get_task_stack(task, buf, size, flags, false /* !may_fault */);
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_4(bpf_get_task_stack_sleepable, task_struct *, task, void *, buf,
    u32, size, u64, flags)
    {
    return __bpf_get_task_stack(task, buf, size, flags, true /* !may_fault */);
    }
pub static mut bpf_func_proto: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn __bpf_get_stack_pe(trace: *mut perf_callchain_entry, trace_nr: u32, buf: *mut c_void, size: u32, flags: u64) -> c_int {
pub static mut user_build_id: bool = false;
pub static mut skip: u64 = 0;
pub static mut user: bool = false;
    u32 elem_size, max_depth, nr_trace;
pub static mut kernel: bool = false;
    if (kernel && user_build_id) {
    return -EINVAL;
    }
    elem_size = user_build_id ? sizeof!(bpf_stack_build_id) : sizeof!(u64);
    if (unlikely(size % elem_size)) {
    return -EINVAL;
    }
    max_depth = stack_map_calculate_max_depth(size, elem_size, flags);
    trace_nr = min_t(u32, trace_nr, max_depth);
    if (trace_nr < skip) {
    return -EFAULT;
    }
    nr_trace = callchain_store(trace, trace_nr, buf, elem_size, flags);
    return callchain_finalize(buf, size, nr_trace, elem_size, flags, false /* !may_fault */);
    }
    BPF_CALL_4(bpf_get_stack_pe, bpf_perf_event_data_kern *, ctx,
    void *, buf, u32, size, u64, flags)
    {
    let mut regs = (ctx.regs);
pub static mut trace: *mut c_void = core::ptr::null_mut();
    let mut event = ctx.event;
    let mut kernel = 0;
    let mut user = 0;
pub static mut err: c_int = 0;
    let mut nr_kernel = 0;
    if (!(event.attr.sample_type & PERF_SAMPLE_CALLCHAIN)) {
    return __bpf_get_stack(regs, core::ptr::null_mut(), buf, size, flags, false /* !may_fault */);
    }
    if (unlikely(flags & ~(BPF_F_SKIP_FIELD_MASK | BPF_F_USER_STACK |
    BPF_F_USER_BUILD_ID))) {
// goto;
    }
    user = flags & BPF_F_USER_STACK;
    kernel = !user;
    err = -EFAULT;
    trace = ctx.data.callchain;
    if (unlikely(!trace)) {
// goto;
    }
    nr_kernel = count_kernel_ip(trace);
    if (kernel) {
    err = __bpf_get_stack_pe(trace, nr_kernel, buf, size, flags);
    } else { /* user */
pub static mut skip: u64 = 0;
    skip += nr_kernel;
    if (skip > BPF_F_SKIP_FIELD_MASK) {
// goto;
    }
    flags = (flags & ~BPF_F_SKIP_FIELD_MASK) | skip;
    err = __bpf_get_stack_pe(trace, trace.nr, buf, size, flags);
    }
// label;
    if (err < 0) {
    memset(buf, 0, size);
    }
    return err;
    }
pub static mut bpf_func_proto: usize = 0;
// Called from eBPF program
#[no_mangle]
pub unsafe extern "C" fn stack_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    return ERR_PTR(-EOPNOTSUPP);
    }
// Called from syscall
#[no_mangle]
pub unsafe extern "C" fn stack_map_lookup_and_delete_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> c_int {
    return bpf_stackmap_extract(map, key, value, true);
    }
// Called from syscall
#[no_mangle]
pub unsafe extern "C" fn bpf_stackmap_extract(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, delete: bool) -> c_int {
    let mut smap = container_of!(map, bpf_stack_map, map);
    let mut bucket = core::ptr::null_mut();
    let mut old_bucket = core::ptr::null_mut();
pub static mut id: u32 = 0;
    if (unlikely(id >= smap.n_buckets)) {
    return -ENOENT;
    }
    bucket = xchg(&smap.buckets[id], core::ptr::null_mut());
    if (!bucket) {
    return -ENOENT;
    }
    trace_len = bucket.nr * stack_map_data_size(map);
    memcpy(value, bucket.data, trace_len);
    memset(value + trace_len, 0, map.value_size - trace_len);
    if (delete) {
    old_bucket = bucket;
    }
    else {
    old_bucket = xchg(&smap.buckets[id], bucket);
    }
    if (old_bucket) {
    pcpu_freelist_push(&smap.freelist, &old_bucket.fnode);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn stack_map_get_next_key(map: *mut bpf_map, key: *mut c_void, next_key: *mut c_void) -> c_int {
    let mut smap = container_of!(map, bpf_stack_map, map);
    let mut id = 0;
    WARN_ON_ONCE!(!rcu_read_lock_held());
    if (!key) {
    id = 0;
    } else {
    id = *key;
    if (id >= smap.n_buckets || !smap.buckets[id]) {
    id = 0;
    }
    else {
    id += 1;
    }
    }
    while (id < smap.n_buckets && !smap.buckets[id]) {
    id += 1;
    }
    if (id >= smap.n_buckets) {
    return -ENOENT;
    }
// next_key = id;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn stack_map_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_long {
    return -EINVAL;
    }
// Called from syscall or from eBPF program
#[no_mangle]
unsafe extern "C" fn stack_map_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_long {
    let mut smap = container_of!(map, bpf_stack_map, map);
pub static mut old_bucket: *mut c_void = core::ptr::null_mut();
pub static mut id: u32 = 0;
    if (unlikely(id >= smap.n_buckets)) {
    return -E2BIG;
    }
    old_bucket = xchg(&smap.buckets[id], core::ptr::null_mut());
    if (old_bucket) {
    pcpu_freelist_push(&smap.freelist, &old_bucket.fnode);
    return 0;
    } else {
    return -ENOENT;
    }
    }
// Called when map->refcnt goes to zero, either from workqueue or from syscall
#[no_mangle]
unsafe extern "C" fn stack_map_free(map: *mut bpf_map) {
    let mut smap = container_of!(map, bpf_stack_map, map);
    bpf_map_area_free(smap.elems);
    pcpu_freelist_destroy(&smap.freelist);
    bpf_map_area_free(smap);
    put_callchain_buffers();
    }
#[no_mangle]
unsafe extern "C" fn stack_map_mem_usage(map: *const bpf_map) -> u64 {
    let mut smap = container_of!(map, bpf_stack_map, map);
pub static mut value_size: u64 = 0;
pub static mut n_buckets: u64 = 0;
pub static mut enties: u64 = 0;
pub static mut usage: u64 = 0;
    usage += n_buckets * sizeof!;
    usage += enties * (sizeof!(stack_map_bucket) + value_size);
    return usage;
    }
    BTF_ID_LIST_SINGLE(stack_trace_map_btf_ids, struct, bpf_stack_map)
pub static mut bpf_map_ops: usize = 0;