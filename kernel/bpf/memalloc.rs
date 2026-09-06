//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/memalloc.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

// Any context (including NMI) BPF specific memory allocator.
//
// Tracing BPF programs can attach to kprobe and fentry. Hence they
// run in unknown context where calling plain kmalloc() might not be safe.
//
// Front-end kmalloc() with per-cpu per-bucket cache of free elements.
// Refill this cache asynchronously from irq_work.
//
// CPU_0 buckets
// 16 32 64 96 128 196 256 512 1024 2048 4096
// ...
// CPU_N buckets
// 16 32 64 96 128 196 256 512 1024 2048 4096
//
// The buckets are prefilled at the start.
// BPF programs always run with migration disabled.
// It's safe to allocate from cache of the current cpu with irqs disabled.
// Free-ing is always done into bucket of the current cpu as well.
// irq_work trims extra free elements from buckets with kfree
// and refills them with kmalloc, so global kmalloc logic takes care
// of freeing objects allocated by one cpu and freed on another.
//
// Every allocated objected is padded with extra 8 bytes that contains
// struct llist_node.
//

pub const BPF_MEM_ALLOC_SIZE_MAX: c_int = 4096;
// similar to kmalloc, but sizeof == 8 bucket is gone
    static u8 size_index[24] __ro_after_init = {
    3,	/* 8 */
    3,	/* 16 */
    4,	/* 24 */
    4,	/* 32 */
    5,	/* 40 */
    5,	/* 48 */
    5,	/* 56 */
    5,	/* 64 */
    1,	/* 72 */
    1,	/* 80 */
    1,	/* 88 */
    1,	/* 96 */
    6,	/* 104 */
    6,	/* 112 */
    6,	/* 120 */
    6,	/* 128 */
    2,	/* 136 */
    2,	/* 144 */
    2,	/* 152 */
    2,	/* 160 */
    2,	/* 168 */
    2,	/* 176 */
    2,	/* 184 */
    2	/* 192 */
    };
#[no_mangle]
unsafe extern "C" fn bpf_mem_cache_idx(size: usize) -> c_int {
    if (!size || size > BPF_MEM_ALLOC_SIZE_MAX) {
    return -1;
    }
    if (size <= 192) {
    return size_index[(size - 1) / 8] - 1;
    }
    return fls(size - 1) - 2;
    }
pub const NUM_CACHES: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_mem_cache {
// per-cpu list of free objects of size 'unit_size'.
// All accesses are done with interrupts disabled and 'active' counter
// protection with __llist_add() and __llist_del_first().
//
    pub free_llist: llist_head,
    pub active: local_t,
// Operations on the free_list from unit_alloc/unit_free/bpf_mem_refill
// are sequenced by per-cpu 'active' counter. But unit_free() cannot
// fail. When 'active' is busy the unit_free() will add an object to
// free_llist_extra.
//
    pub free_llist_extra: llist_head,
    pub refill_work: irq_work,
    pub objcg: *mut obj_cgroup,
    pub unit_size: c_int,
// count of objects in free_llist
    pub free_cnt: c_int,
    pub batch: int low_watermark, high_watermark,,
    pub percpu_size: c_int,
    pub draining: bool,
    pub tgt: *mut bpf_mem_cache,
    pub ctx): *mut *mut *mut c_void (dtor)(void obj, void,
    pub dtor_ctx: *mut c_void,
// list of objects to be freed after RCU GP
    pub free_by_rcu: llist_head,
    pub free_by_rcu_tail: *mut llist_node,
    pub waiting_for_gp: llist_head,
    pub waiting_for_gp_tail: *mut llist_node,
    pub rcu: rcu_head,
    pub call_rcu_in_progress: core::sync::atomic::AtomicI32,
    pub free_llist_extra_rcu: llist_head,
// list of objects to be freed after RCU tasks trace GP
    pub free_by_rcu_ttrace: llist_head,
    pub waiting_for_gp_ttrace: llist_head,
    pub rcu_ttrace: rcu_head,
    pub call_rcu_ttrace_in_progress: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_mem_caches {
    pub cache: [bpf_mem_cache; NUM_CACHES],
}

    static const u16 sizes[NUM_CACHES] = {96, 192, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096};
    static struct llist_node notrace *__llist_del_first(llist_head *head)
    {
    let mut entry = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    entry = head.first;
    if (!entry) {
    return core::ptr::null_mut();
    }
    next = entry.next;
    head.first = next;
    return entry;
    }
#[no_mangle]
pub unsafe extern "C" fn __alloc(c: *mut bpf_mem_cache, node: c_int, flags: gfp_t) -> *mut c_void {
    if (c.percpu_size) {
    let mut obj = kmalloc_node(c.percpu_size, flags, node);
    let mut pptr = __alloc_percpu_gfp(c.unit_size, 8, flags);
    if (!obj || !pptr) {
    free_percpu(pptr);
    kfree(obj);
    return core::ptr::null_mut();
    }
    obj[1] = pptr;
    return obj;
    }
    return kmalloc_node(c.unit_size, flags | __GFP_ZERO, node);
    }
#[no_mangle]
pub unsafe extern "C" fn get_memcg(c: *mut bpf_mem_cache) -> *mut c_void {

    if (c.objcg) {
    return get_mem_cgroup_from_objcg(c.objcg);
    }
    return root_mem_cgroup;

    return core::ptr::null_mut();

    }
#[no_mangle]
unsafe extern "C" fn inc_active(c: *mut bpf_mem_cache, flags: *mut c_ulong) {
    if (IS_ENABLED!(CONFIG_PREEMPT_RT)) {
// In RT irq_work runs in per-cpu kthread, so disable
// interrupts to avoid preemption and interrupts and
// reduce the chance of bpf prog executing on this cpu
// when active counter is busy.
//
    local_irq_save(*flags);
    }
// alloc_bulk runs from irq_work which will not preempt a bpf
// program that does unit_alloc/unit_free since IRQs are
// disabled there. There is no race to increment 'active'
// counter. It protects free_llist from corruption in case NMI
// bpf prog preempted this loop.
//
    WARN_ON_ONCE!(local_inc_return(&c.active) != 1);
    }
#[no_mangle]
unsafe extern "C" fn dec_active(c: *mut bpf_mem_cache, flags: *mut c_ulong) {
    local_dec(&c.active);
    if (IS_ENABLED!(CONFIG_PREEMPT_RT)) {
    local_irq_restore(*flags);
    }
    }
#[no_mangle]
unsafe extern "C" fn add_obj_to_free_list(c: *mut bpf_mem_cache, obj: *mut c_void) {
    let mut flags = 0;
    inc_active(c, &flags);
    __llist_add(obj, &c.free_llist);
    c.free_cnt += 1;
    dec_active(c, &flags);
    }
// Mostly runs from irq_work except __init phase.
#[no_mangle]
unsafe extern "C" fn alloc_bulk(c: *mut bpf_mem_cache, cnt: c_int, node: c_int, atomic: bool) {
    let mut memcg = core::ptr::null_mut(), *old_memcg;
    let mut gfp;
pub static mut obj: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    gfp = __GFP_NOWARN | __GFP_ACCOUNT;
    gfp |= atomic ? GFP_NOWAIT : GFP_KERNEL;
    while (i < cnt) {
//
// For every 'c' llist_del_first(&c->free_by_rcu_ttrace); is
// done only by one CPU == current CPU. Other CPUs might
// llist_add() and llist_del_all() in parallel.
//
    obj = llist_del_first(&c.free_by_rcu_ttrace);
    if (!obj) {
    break;
    }
    add_obj_to_free_list(c, obj);
    }
    if (i >= cnt) {
    return;
    }
    while (i < cnt) {
    obj = llist_del_first(&c.waiting_for_gp_ttrace);
    if (!obj) {
    break;
    }
    add_obj_to_free_list(c, obj);
    }
    if (i >= cnt) {
    return;
    }
    memcg = get_memcg(c);
    old_memcg = set_active_memcg(memcg);
    while (i < cnt) {
// Allocate, but don't deplete atomic reserves that typical
// GFP_ATOMIC would do. irq_work runs on this cpu and kmalloc
// will allocate from the current numa node which is what we
// want here.
//
    obj = __alloc(c, node, gfp);
    if (!obj) {
    break;
    }
    add_obj_to_free_list(c, obj);
    }
    set_active_memcg(old_memcg);
    mem_cgroup_put(memcg);
    }
#[no_mangle]
unsafe extern "C" fn free_one(obj: *mut c_void, percpu: bool) {
    if (percpu) {
    free_percpu((obj)[1]);
    }
    kfree(obj);
    }
#[no_mangle]
unsafe extern "C" fn free_all(c: *mut bpf_mem_cache, llnode: *mut llist_node, percpu: bool) -> c_int {
    let mut pos = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
pub static mut cnt: c_int = 0;
    llist_for_each_safe(pos, t, llnode) {
    if (c.dtor) {
    c.dtor(pos + LLIST_NODE_SZ, c.dtor_ctx);
    }
    free_one(pos, percpu);
    cnt += 1;
    }
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn __free_rcu(head: *mut rcu_head) {
    let mut c = container_of!(head, bpf_mem_cache, rcu_ttrace);
    free_all(c, llist_del_all(&c.waiting_for_gp_ttrace), !!c.percpu_size);
    atomic_set(&c.call_rcu_ttrace_in_progress, 0);
    }
#[no_mangle]
unsafe extern "C" fn enque_to_free(c: *mut bpf_mem_cache, obj: *mut c_void) {
    let mut llnode = obj;
// bpf_mem_cache is a per-cpu object. Freeing happens in irq_work.
// Nothing races to add to free_by_rcu_ttrace list.
//
    llist_add(llnode, &c.free_by_rcu_ttrace);
    }
#[no_mangle]
unsafe extern "C" fn do_call_rcu_ttrace(c: *mut bpf_mem_cache) {
    let mut llnode = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    if (atomic_xchg(&c.call_rcu_ttrace_in_progress, 1)) {
    if (unlikely(READ_ONCE(c.draining))) {
    llnode = llist_del_all(&c.free_by_rcu_ttrace);
    free_all(c, llnode, !!c.percpu_size);
    }
    return;
    }
    WARN_ON_ONCE!(!llist_empty(&c.waiting_for_gp_ttrace));
    llist_for_each_safe(llnode, t, llist_del_all(&c.free_by_rcu_ttrace)) {
    llist_add(llnode, &c.waiting_for_gp_ttrace);
    }
    if (unlikely(READ_ONCE(c.draining))) {
    __free_rcu(&c.rcu_ttrace);
    return;
    }
//
// Use call_rcu_tasks_trace() to wait for sleepable progs to finish.
// RCU Tasks Trace grace period implies RCU grace period, so pass
// __free_rcu directly as the callback.
//
    call_rcu_tasks_trace(&c.rcu_ttrace, __free_rcu);
    }
#[no_mangle]
unsafe extern "C" fn free_bulk(c: *mut bpf_mem_cache) {
    let mut tgt = c.tgt;
    let mut llnode = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    let mut flags = 0;
    let mut cnt = 0;
    WARN_ON_ONCE!(tgt.unit_size != c.unit_size);
    WARN_ON_ONCE!(tgt.percpu_size != c.percpu_size);
    do {
    inc_active(c, &flags);
    llnode = __llist_del_first(&c.free_llist);
    if (llnode) {
    cnt = --c.free_cnt;
    }
    else {
    cnt = 0;
    }
    dec_active(c, &flags);
    if (llnode) {
    enque_to_free(tgt, llnode);
    }
    } while (cnt > (c.high_watermark + c.low_watermark) / 2);
// and drain free_llist_extra
    llist_for_each_safe(llnode, t, llist_del_all(&c.free_llist_extra)) {
    enque_to_free(tgt, llnode);
    }
    do_call_rcu_ttrace(tgt);
    }
#[no_mangle]
unsafe extern "C" fn __free_by_rcu(head: *mut rcu_head) {
    let mut c = container_of!(head, bpf_mem_cache, rcu);
    let mut tgt = c.tgt;
pub static mut llnode: *mut c_void = core::ptr::null_mut();
    WARN_ON_ONCE!(tgt.unit_size != c.unit_size);
    WARN_ON_ONCE!(tgt.percpu_size != c.percpu_size);
    llnode = llist_del_all(&c.waiting_for_gp);
    if (!llnode) {
// goto;
    }
    llist_add_batch(llnode, c.waiting_for_gp_tail, &tgt.free_by_rcu_ttrace);
// Objects went through regular RCU GP. Send them to RCU tasks trace
    do_call_rcu_ttrace(tgt);
// label;
    atomic_set(&c.call_rcu_in_progress, 0);
    }
#[no_mangle]
unsafe extern "C" fn check_free_by_rcu(c: *mut bpf_mem_cache) {
    let mut llnode = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    let mut flags = 0;
// drain free_llist_extra_rcu
    if (unlikely(!llist_empty(&c.free_llist_extra_rcu))) {
    inc_active(c, &flags);
    llist_for_each_safe(llnode, t, llist_del_all(&c.free_llist_extra_rcu)) {
    if (__llist_add(llnode, &c.free_by_rcu))
    c.free_by_rcu_tail = llnode;
    }
    dec_active(c, &flags);
    }
    if (llist_empty(&c.free_by_rcu)) {
    return;
    }
    if (atomic_xchg(&c.call_rcu_in_progress, 1)) {
//
// Instead of kmalloc-ing new rcu_head and triggering 10k
// call_rcu() to hit rcutree.qhimark and force RCU to notice
// the overload just ask RCU to hurry up. There could be many
// objects in free_by_rcu list.
// This hint reduces memory consumption for an artificial
// benchmark from 2 Gbyte to 150 Mbyte.
//
    rcu_request_urgent_qs_task(current);
    return;
    }
    WARN_ON_ONCE!(!llist_empty(&c.waiting_for_gp));
    inc_active(c, &flags);
    WRITE_ONCE(c.waiting_for_gp.first, __llist_del_all(&c.free_by_rcu));
    c.waiting_for_gp_tail = c.free_by_rcu_tail;
    dec_active(c, &flags);
    if (unlikely(READ_ONCE(c.draining))) {
    free_all(c, llist_del_all(&c.waiting_for_gp), !!c.percpu_size);
    atomic_set(&c.call_rcu_in_progress, 0);
    } else {
    call_rcu_hurry(&c.rcu, __free_by_rcu);
    }
    }
#[no_mangle]
unsafe extern "C" fn bpf_mem_refill(work: *mut irq_work) {
    let mut c = container_of!(work, bpf_mem_cache, refill_work);
    let mut cnt = 0;
// Racy access to free_cnt. It doesn't need to be 100% accurate
    cnt = c.free_cnt;
    if (cnt < c.low_watermark) {
// irq_work runs on this cpu and kmalloc will allocate
// from the current numa node which is what we want here.
//
    alloc_bulk(c, c.batch, NUMA_NO_NODE, true);
    }

    else if (cnt > c.high_watermark) {
    free_bulk(c);
    }
    check_free_by_rcu(c);
    }
#[no_mangle]
unsafe extern "C" fn irq_work_raise(c: *mut bpf_mem_cache) -> void notrace {
    irq_work_queue(&c.refill_work);
    }
// For typical bpf map case that uses bpf_mem_cache_alloc and single bucket
// the freelist cache will be elem_size * 64 (or less) on each cpu.
//
// For bpf programs that don't have statically known allocation sizes and
// assuming (low_mark + high_mark) / 2 as an average number of elements per
// bucket and all buckets are used the total amount of memory in freelists
// on each cpu will be:
// 64*16 + 64*32 + 64*64 + 64*96 + 64*128 + 64*196 + 64*256 + 32*512 + 16*1024 + 8*2048 + 4*4096
// == ~ 116 Kbyte using below heuristic.
// Initialized, but unused bpf allocator (not bpf map specific one) will
// consume ~ 11 Kbyte per cpu.
// Typical case will be between 11K and 116K closer to 11K.
// bpf progs can and should share bpf_mem_cache when possible.
//
// Percpu allocation is typically rare. To avoid potential unnecessary large
// memory consumption, set low_mark = 1 and high_mark = 3, resulting in c->batch = 1.
//
#[no_mangle]
unsafe extern "C" fn init_refill_work(c: *mut bpf_mem_cache) {
    init_irq_work(&c.refill_work, bpf_mem_refill);
    if (c.percpu_size) {
    c.low_watermark = 1;
    c.high_watermark = 3;
    } else if (c.unit_size <= 256) {
    c.low_watermark = 32;
    c.high_watermark = 96;
    } else {
// When page_size == 4k, order-0 cache will have low_mark == 2
// and high_mark == 6 with batch alloc of 3 individual pages at
// a time.
// 8k allocs and above low == 1, high == 3, batch == 1.
//
    c.low_watermark = max(32 * 256 / c.unit_size, 1);
    c.high_watermark = max(96 * 256 / c.unit_size, 3);
    }
    c.batch = max((c.high_watermark - c.low_watermark) / 4 * 3, 1);
    }
#[no_mangle]
unsafe extern "C" fn prefill_mem_cache(c: *mut bpf_mem_cache, cpu: c_int) {
pub static mut cnt: c_int = 1;
// To avoid consuming memory, for non-percpu allocation, assume that
// 1st run of bpf prog won't be doing more than 4 map_update_elem from
// irq disabled region if unit size is less than or equal to 256.
// For all other cases, let us just do one allocation.
//
    if (!c.percpu_size && c.unit_size <= 256) {
    cnt = 4;
    }
    alloc_bulk(c, cnt, cpu_to_node(cpu), false);
    }
// When size != 0 bpf_mem_cache for each cpu.
// This is typical bpf hash map use case when all elements have equal size.
//
// When size == 0 allocate 11 bpf_mem_cache-s for each cpu, then rely on
// kmalloc/kfree. Max allocation size is 4096 in this case.
// This is bpf_dynptr and bpf_kptr use case.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_mem_alloc_init(ma: *mut bpf_mem_alloc, size: c_int, percpu: bool) -> c_int {
pub static mut cc: *mut c_void = core::ptr::null_mut(); struct bpf_mem_caches  *pcc;
pub static mut c: *mut c_void = core::ptr::null_mut(); struct bpf_mem_cache  *pc;
    let mut objcg = core::ptr::null_mut();
    int cpu, i, unit_size, percpu_size = 0;
    if (percpu && size == 0) {
    return -EINVAL;
    }
// room for llist_node and per-cpu pointer
    if (percpu) {
    percpu_size = LLIST_NODE_SZ + sizeof!;
    }
    ma.percpu = percpu;
    if (size) {
    pc = __alloc_percpu_gfp(sizeof!(*pc), 8, GFP_KERNEL);
    if (!pc) {
    return -ENOMEM;
    }
    if (!percpu) {
    size += LLIST_NODE_SZ; /* room for llist_node */
    }
    unit_size = size;

    if (memcg_bpf_enabled()) {
    objcg = get_obj_cgroup_from_current();
    }

    ma.objcg = objcg;
    for_each_possible_cpu(cpu) {
    c = per_cpu_ptr(pc, cpu);
    c.unit_size = unit_size;
    c.objcg = objcg;
    c.percpu_size = percpu_size;
    c.tgt = c;
    init_refill_work(c);
    prefill_mem_cache(c, cpu);
    }
    ma.cache = pc;
    return 0;
    }
    pcc = __alloc_percpu_gfp(sizeof!(*cc), 8, GFP_KERNEL);
    if (!pcc) {
    return -ENOMEM;
    }

    objcg = get_obj_cgroup_from_current();

    ma.objcg = objcg;
    for_each_possible_cpu(cpu) {
    cc = per_cpu_ptr(pcc, cpu);
    while (i < NUM_CACHES) {
    c = &cc.cache[i];
    c.unit_size = sizes[i];
    c.objcg = objcg;
    c.percpu_size = percpu_size;
    c.tgt = c;
    init_refill_work(c);
    prefill_mem_cache(c, cpu);
    }
    }
    ma.caches = pcc;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mem_alloc_percpu_init(ma: *mut bpf_mem_alloc, objcg: *mut obj_cgroup) -> c_int {
    let mut pcc = core::ptr::null_mut();
    pcc = __alloc_percpu_gfp(sizeof!(bpf_mem_caches), 8, GFP_KERNEL);
    if (!pcc) {
    return -ENOMEM;
    }
    ma.caches = pcc;
    ma.objcg = objcg;
    ma.percpu = true;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mem_alloc_percpu_unit_init(ma: *mut bpf_mem_alloc, size: c_int) -> c_int {
pub static mut cc: *mut c_void = core::ptr::null_mut(); struct bpf_mem_caches  *pcc;
    let mut cpu = 0;
    let mut i = 0;
    let mut unit_size = 0;
    let mut percpu_size = 0;
pub static mut objcg: *mut c_void = core::ptr::null_mut();
pub static mut c: *mut c_void = core::ptr::null_mut();
    i = bpf_mem_cache_idx(size);
    if (i < 0) {
    return -EINVAL;
    }
// room for llist_node and per-cpu pointer
    percpu_size = LLIST_NODE_SZ + sizeof!;
    unit_size = sizes[i];
    objcg = ma.objcg;
    pcc = ma.caches;
    for_each_possible_cpu(cpu) {
    cc = per_cpu_ptr(pcc, cpu);
    c = &cc.cache[i];
    if (c.unit_size) {
    break;
    }
    c.unit_size = unit_size;
    c.objcg = objcg;
    c.percpu_size = percpu_size;
    c.tgt = c;
    init_refill_work(c);
    prefill_mem_cache(c, cpu);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drain_mem_cache(c: *mut bpf_mem_cache) {
pub static mut percpu: bool = false;
// No progs are using this bpf_mem_cache, but htab_map_free() called
// bpf_mem_cache_free() for all remaining elements and they can be in
// free_by_rcu_ttrace or in waiting_for_gp_ttrace lists, so drain those lists now.
//
// Except for waiting_for_gp_ttrace list, there are no concurrent operations
// on these lists, so it is safe to use __llist_del_all().
//
    free_all(c, llist_del_all(&c.free_by_rcu_ttrace), percpu);
    free_all(c, llist_del_all(&c.waiting_for_gp_ttrace), percpu);
    free_all(c, __llist_del_all(&c.free_llist), percpu);
    free_all(c, __llist_del_all(&c.free_llist_extra), percpu);
    free_all(c, __llist_del_all(&c.free_by_rcu), percpu);
    free_all(c, __llist_del_all(&c.free_llist_extra_rcu), percpu);
    free_all(c, llist_del_all(&c.waiting_for_gp), percpu);
    }
#[no_mangle]
unsafe extern "C" fn check_mem_cache(c: *mut bpf_mem_cache) {
    WARN_ON_ONCE!(!llist_empty(&c.free_by_rcu_ttrace));
    WARN_ON_ONCE!(!llist_empty(&c.waiting_for_gp_ttrace));
    WARN_ON_ONCE!(!llist_empty(&c.free_llist));
    WARN_ON_ONCE!(!llist_empty(&c.free_llist_extra));
    WARN_ON_ONCE!(!llist_empty(&c.free_by_rcu));
    WARN_ON_ONCE!(!llist_empty(&c.free_llist_extra_rcu));
    WARN_ON_ONCE!(!llist_empty(&c.waiting_for_gp));
    }
#[no_mangle]
unsafe extern "C" fn check_leaked_objs(ma: *mut bpf_mem_alloc) {
pub static mut cc: *mut c_void = core::ptr::null_mut();
pub static mut c: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    let mut i = 0;
    if (ma.cache) {
    for_each_possible_cpu(cpu) {
    c = per_cpu_ptr(ma.cache, cpu);
    check_mem_cache(c);
    }
    }
    if (ma.caches) {
    for_each_possible_cpu(cpu) {
    cc = per_cpu_ptr(ma.caches, cpu);
    while (i < NUM_CACHES) {
    c = &cc.cache[i];
    check_mem_cache(c);
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn free_mem_alloc_no_barrier(ma: *mut bpf_mem_alloc) {
// We can free dtor ctx only once all callbacks are done using it.
    if (ma.dtor_ctx_free) {
    ma.dtor_ctx_free(ma.dtor_ctx);
    }
    check_leaked_objs(ma);
    free_percpu(ma.cache);
    free_percpu(ma.caches);
    ma.cache = core::ptr::null_mut();
    ma.caches = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn free_mem_alloc(ma: *mut bpf_mem_alloc) {
//
// waiting_for_gp[_ttrace] lists were drained, but RCU callbacks
// might still execute. Wait for them.
//
// rcu_barrier_tasks_trace() doesn't imply synchronize_rcu_tasks_trace(),
// but rcu_barrier_tasks_trace() and rcu_barrier() below are only used
// to wait for the pending __free_by_rcu(), and __free_rcu(). RCU Tasks
// Trace grace period implies RCU grace period, so all __free_rcu don't
// need extra call_rcu() (and thus extra rcu_barrier() here).
//
    rcu_barrier(); /* wait for __free_by_rcu */
    rcu_barrier_tasks_trace(); /* wait for __free_rcu */
    free_mem_alloc_no_barrier(ma);
    }
#[no_mangle]
unsafe extern "C" fn free_mem_alloc_deferred(work: *mut work_struct) {
    let mut ma = container_of!(work, bpf_mem_alloc, work);
    free_mem_alloc(ma);
    kfree(ma);
    }
#[no_mangle]
unsafe extern "C" fn destroy_mem_alloc(ma: *mut bpf_mem_alloc, rcu_in_progress: c_int) {
pub static mut copy: *mut c_void = core::ptr::null_mut();
    if (!rcu_in_progress) {
// Fast path. No callbacks are pending, hence no need to do
// rcu_barrier-s.
//
    free_mem_alloc_no_barrier(ma);
    return;
    }
    copy = kmemdup(ma, sizeof!(*ma), GFP_KERNEL);
    if (!copy) {
// Slow path with inline barrier-s
    free_mem_alloc(ma);
    return;
    }
// Defer barriers into worker to let the rest of map memory to be freed
    memset(ma, 0, sizeof!(*ma));
    INIT_WORK(&copy.work, free_mem_alloc_deferred);
    queue_work(system_dfl_wq, &copy.work);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mem_alloc_destroy(ma: *mut bpf_mem_alloc) {
pub static mut cc: *mut c_void = core::ptr::null_mut();
pub static mut c: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    let mut i = 0;
    let mut rcu_in_progress = 0;
    if (ma.cache) {
    rcu_in_progress = 0;
    for_each_possible_cpu(cpu) {
    c = per_cpu_ptr(ma.cache, cpu);
    WRITE_ONCE(c.draining, true);
    irq_work_sync(&c.refill_work);
    drain_mem_cache(c);
    rcu_in_progress += atomic_read(&c.call_rcu_ttrace_in_progress);
    rcu_in_progress += atomic_read(&c.call_rcu_in_progress);
    }
    obj_cgroup_put(ma.objcg);
    destroy_mem_alloc(ma, rcu_in_progress);
    }
    if (ma.caches) {
    rcu_in_progress = 0;
    for_each_possible_cpu(cpu) {
    cc = per_cpu_ptr(ma.caches, cpu);
    while (i < NUM_CACHES) {
    c = &cc.cache[i];
    WRITE_ONCE(c.draining, true);
    irq_work_sync(&c.refill_work);
    drain_mem_cache(c);
    rcu_in_progress += atomic_read(&c.call_rcu_ttrace_in_progress);
    rcu_in_progress += atomic_read(&c.call_rcu_in_progress);
    }
    }
    obj_cgroup_put(ma.objcg);
    destroy_mem_alloc(ma, rcu_in_progress);
    }
    }
// notrace is necessary here and in other functions to make sure
// bpf programs cannot attach to them and cause llist corruptions.
//
    static void notrace *unit_alloc(bpf_mem_cache *c)
    {
    let mut llnode = core::ptr::null_mut();
    let mut flags = 0;
pub static mut cnt: c_int = 0;
// Disable irqs to prevent the following race for majority of prog types:
// prog_A
// bpf_mem_alloc
// preemption or irq -> prog_B
// bpf_mem_alloc
//
// but prog_B could be a perf_event NMI prog.
// Use per-cpu 'active' counter to order free_list access between
// unit_alloc/unit_free/bpf_mem_refill.
//
    local_irq_save(flags);
    if (local_inc_return(&c.active) == 1) {
    llnode = __llist_del_first(&c.free_llist);
    if (llnode) {
    cnt = --c.free_cnt;
// llnode = c;
    }
    }
    local_dec(&c.active);
    WARN_ON!(cnt < 0);
    if (cnt < c.low_watermark) {
    irq_work_raise(c);
    }
// Enable IRQ after the enqueue of irq work completes, so irq work
// will run after IRQ is enabled and free_llist may be refilled by
// irq work before other task preempts current task.
//
    local_irq_restore(flags);
    return llnode;
    }
// Though 'ptr' object could have been allocated on a different cpu
// add it to the free_llist of the current cpu.
// Let kfree() logic deal with it when it's later called from irq_work.
//
#[no_mangle]
unsafe extern "C" fn unit_free(c: *mut bpf_mem_cache, ptr: *mut c_void) -> void notrace {
    let mut llnode = ptr - LLIST_NODE_SZ;
    let mut flags = 0;
pub static mut cnt: c_int = 0;
    BUILD_BUG_ON!(LLIST_NODE_SZ > 8);
//
// Remember bpf_mem_cache that allocated this object.
// The hint is not accurate.
//
    c.tgt = *llnode;
    local_irq_save(flags);
    if (local_inc_return(&c.active) == 1) {
    __llist_add(llnode, &c.free_llist);
    cnt = ++c.free_cnt;
    } else {
// unit_free() cannot fail. Therefore add an object to atomic
// llist. free_bulk() will drain it. Though free_llist_extra is
// a per-cpu list we have to use atomic llist_add here, since
// it also can be interrupted by bpf nmi prog that does another
// unit_free() into the same free_llist_extra.
//
    llist_add(llnode, &c.free_llist_extra);
    }
    local_dec(&c.active);
    if (cnt > c.high_watermark) {
// free few objects from current cpu into global kmalloc pool
    irq_work_raise(c);
    }
// Enable IRQ after irq_work_raise() completes, otherwise when current
// task is preempted by task which does unit_alloc(), unit_alloc() may
// return NULL unexpectedly because irq work is already pending but can
// not been triggered and free_llist can not be refilled timely.
//
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn unit_free_rcu(c: *mut bpf_mem_cache, ptr: *mut c_void) -> void notrace {
    let mut llnode = ptr - LLIST_NODE_SZ;
    let mut flags = 0;
    c.tgt = *llnode;
    local_irq_save(flags);
    if (local_inc_return(&c.active) == 1) {
    if (__llist_add(llnode, &c.free_by_rcu)) {
    c.free_by_rcu_tail = llnode;
    }
    } else {
    llist_add(llnode, &c.free_llist_extra_rcu);
    }
    local_dec(&c.active);
    if (!atomic_read(&c.call_rcu_in_progress)) {
    irq_work_raise(c);
    }
    local_irq_restore(flags);
    }
// Called from BPF program or from sys_bpf syscall.
// In both cases migration is disabled.
//
    void notrace *bpf_mem_alloc(bpf_mem_alloc *ma, size_t size)
    {
    let mut idx = 0;
pub static mut ret: *mut c_void = core::ptr::null_mut();
    if (!size) {
    return core::ptr::null_mut();
    }
    if (!ma.percpu) {
    size += LLIST_NODE_SZ;
    }
    idx = bpf_mem_cache_idx(size);
    if (idx < 0) {
    return core::ptr::null_mut();
    }
    ret = unit_alloc(this_cpu_ptr(ma.caches).cache + idx);
    return !ret ? core::ptr::null_mut() : ret + LLIST_NODE_SZ;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mem_free(ma: *mut bpf_mem_alloc, ptr: *mut c_void) -> void notrace {
pub static mut c: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    if (!ptr) {
    return;
    }
    c = *(ptr - LLIST_NODE_SZ);
    idx = bpf_mem_cache_idx(c.unit_size);
    if (WARN_ON_ONCE!(idx < 0)) {
    return;
    }
    unit_free(this_cpu_ptr(ma.caches).cache + idx, ptr);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mem_free_rcu(ma: *mut bpf_mem_alloc, ptr: *mut c_void) -> void notrace {
pub static mut c: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    if (!ptr) {
    return;
    }
    c = *(ptr - LLIST_NODE_SZ);
    idx = bpf_mem_cache_idx(c.unit_size);
    if (WARN_ON_ONCE!(idx < 0)) {
    return;
    }
    unit_free_rcu(this_cpu_ptr(ma.caches).cache + idx, ptr);
    }
    void notrace *bpf_mem_cache_alloc(bpf_mem_alloc *ma)
    {
pub static mut ret: *mut c_void = core::ptr::null_mut();
    ret = unit_alloc(this_cpu_ptr(ma.cache));
    return !ret ? core::ptr::null_mut() : ret + LLIST_NODE_SZ;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mem_cache_free(ma: *mut bpf_mem_alloc, ptr: *mut c_void) -> void notrace {
    if (!ptr) {
    return;
    }
    unit_free(this_cpu_ptr(ma.cache), ptr);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mem_cache_free_rcu(ma: *mut bpf_mem_alloc, ptr: *mut c_void) -> void notrace {
    if (!ptr) {
    return;
    }
    unit_free_rcu(this_cpu_ptr(ma.cache), ptr);
    }
// Directly does a kfree() without putting 'ptr' back to the free_llist
// for reuse and without waiting for a rcu_tasks_trace gp.
// The caller must first go through the rcu_tasks_trace gp for 'ptr'
// before calling bpf_mem_cache_raw_free().
// It could be used when the rcu_tasks_trace callback does not have
// a hold on the original bpf_mem_alloc object that allocated the
// 'ptr'. This should only be used in the uncommon code path.
// Otherwise, the bpf_mem_alloc's free_llist cannot be refilled
// and may affect performance.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_mem_cache_raw_free(ptr: *mut c_void) {
    if (!ptr) {
    return;
    }
    kfree(ptr - LLIST_NODE_SZ);
    }
// When flags == GFP_KERNEL, it signals that the caller will not cause
// deadlock when using kmalloc. bpf_mem_cache_alloc_flags() will use
// kmalloc if the free_llist is empty.
//
    void notrace *bpf_mem_cache_alloc_flags(bpf_mem_alloc *ma, gfp_t flags)
    {
pub static mut c: *mut c_void = core::ptr::null_mut();
pub static mut ret: *mut c_void = core::ptr::null_mut();
    c = this_cpu_ptr(ma.cache);
    ret = unit_alloc(c);
    if (!ret && flags == GFP_KERNEL) {
    let mut memcg = core::ptr::null_mut();
    let mut old_memcg = core::ptr::null_mut();
    memcg = get_memcg(c);
    old_memcg = set_active_memcg(memcg);
    ret = __alloc(c, NUMA_NO_NODE, GFP_KERNEL | __GFP_NOWARN | __GFP_ACCOUNT);
    if (ret) {
// ret = c;
    }
    set_active_memcg(old_memcg);
    mem_cgroup_put(memcg);
    }
    return !ret ? core::ptr::null_mut() : ret + LLIST_NODE_SZ;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mem_alloc_check_size(percpu: bool, size: usize) -> c_int {
// The size of percpu allocation doesn't have LLIST_NODE_SZ overhead
    if ((percpu && size > BPF_MEM_ALLOC_SIZE_MAX) ||
    (!percpu && size > BPF_MEM_ALLOC_SIZE_MAX - LLIST_NODE_SZ)) {
    return -E2BIG;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_mem_alloc_set_dtor(ma: *mut bpf_mem_alloc, obj: *mut *mut c_void (dtor)(void, ctx: *mut c_void) {
pub static mut cc: *mut c_void = core::ptr::null_mut();
pub static mut c: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    let mut i = 0;
    ma.dtor_ctx_free = dtor_ctx_free;
    ma.dtor_ctx = ctx;
    if (ma.cache) {
    for_each_possible_cpu(cpu) {
    c = per_cpu_ptr(ma.cache, cpu);
    c.dtor = dtor;
    c.dtor_ctx = ctx;
    }
    }
    if (ma.caches) {
    for_each_possible_cpu(cpu) {
    cc = per_cpu_ptr(ma.caches, cpu);
    while (i < NUM_CACHES) {
    c = &cc.cache[i];
    c.dtor = dtor;
    c.dtor_ctx = ctx;
    }
    }
    }
    }