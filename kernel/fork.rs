//! Automatically rewritten from C to Rust
//! Source: kernel/fork.c
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
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

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
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0-only
//
// linux/kernel/fork.c
//
// Copyright (C) 1991, 1992  Linus Torvalds
//
// 'fork.c' contains the help-routines for the 'fork' system call
// (see also entry.S and others).
// Fork is rather simple, once you get the hang of it, but the memory
// management can be a bitch. See 'mm/memory.c': 'copy_page_range()'
//

// For dup_mmap().

// Macro flag: #define CREATE_TRACE_POINTS

//
// Minimum number of threads to boot the kernel
//
pub const MIN_THREADS: c_int = 20;
//
// Maximum number of threads
//

//
// Protected counters by write_lock_irq(&tasklist_lock)
//
    let mut total_forks = 0;	/* Handle normal Linux uptimes. */
    let mut nr_threads = 0;			/* The idle threads do not count.. */
    static int max_threads ;		/* tunable limit on nr_threads */

    static const char * const resident_page_types[] = {
    NAMED_ARRAY_INDEX(MM_FILEPAGES),
    NAMED_ARRAY_INDEX(MM_ANONPAGES),
    NAMED_ARRAY_INDEX(MM_SWAPENTS),
    NAMED_ARRAY_INDEX(MM_SHMEMPAGES),
    };
    DEFINE_PER_CPU(unsigned long, process_counts) = 0;
    __cacheline_aligned DEFINE_RWLOCK(tasklist_lock);  /* outer */

#[no_mangle]
pub unsafe extern "C" fn lockdep_tasklist_lock_is_held() -> c_int {
    return lockdep_is_held(&tasklist_lock);
    }
// EXPORT_SYMBOL_GPL;

#[no_mangle]
pub unsafe extern "C" fn nr_processes() -> c_int {
    let mut cpu = 0;
pub static mut total: c_int = 0;
    for_each_possible_cpu(cpu) {
    total += per_cpu(process_counts, cpu);
    }
    return total;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_release_task_struct(tsk: *mut task_struct) -> void __weak {
    }
pub static mut task_struct_cachep: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn alloc_task_struct_node() {
    return kmem_cache_alloc_node(task_struct_cachep, GFP_KERNEL, node);
    }
#[no_mangle]
pub unsafe extern "C" fn free_task_struct(tsk: *mut task_struct) {
    kmem_cache_free(task_struct_cachep, tsk);
    }

//
// vmalloc() is a bit slow, and calling vfree() enough times will force a TLB
// flush.  Try to minimize the number of calls by caching stacks.
//
pub const NR_CACHED_STACKS: c_int = 2;
// static DEFINE_PER_CPU(vm_struct *, cached_stacks[NR_CACHED_STACKS]);
//
// Allocated stacks are cached and later reused by new threads, so memcg
// accounting is performed by the code assigning/releasing stacks to tasks.
// We need a zeroed memory without __GFP_ACCOUNT.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_stack {
    pub rcu: rcu_head,
    pub stack_vm_area: *mut vm_struct,
}

#[no_mangle]
pub unsafe extern "C" fn alloc_thread_stack_node_from_cache() {
    let mut vm_area = core::ptr::null_mut();
    let mut i = 0;
//
// If the node has memory, we are guaranteed the stacks are backed by local pages.
// Otherwise the pages are arbitrary.
//
// Note that depending on cpuset it is possible we will get migrated to a different
// node immediately after allocating here, so this does *not* guarantee locality for
// arbitrary callers.
//
    scoped_guard(preempt) {
    if (node != NUMA_NO_NODE && numa_node_id() != node) {
    return core::ptr::null_mut();
    }
    while (i < NR_CACHED_STACKS) {
    vm_area = this_cpu_xchg(cached_stacks[i], core::ptr::null_mut());
    if (vm_area) {
    return vm_area;
    }
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn try_release_thread_stack_to_cache(vm_area: *mut vm_struct) -> bool {
    let mut i = 0;
    let mut nid = 0;
//
// Don't cache stacks if any of the pages don't match the local domain, unless
// there is no local memory to begin with.
//
// Note that lack of local memory does not automatically mean it makes no difference
// performance-wise which other domain backs the stack. In this case we are merely
// trying to avoid constantly going to vmalloc.
//
    scoped_guard(preempt) {
    nid = numa_node_id();
    if (node_state(nid, N_MEMORY)) {
    while (i < vm_area.nr_pages) {
    let mut page = vm_area.pages[i];
    if (page_to_nid(page) != nid) {
    return false;
    }
    }
    }
    while (i < NR_CACHED_STACKS) {
    let mut tmp = core::ptr::null_mut();
    if (this_cpu_try_cmpxchg(cached_stacks[i], &tmp, vm_area)) {
    return true;
    }
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn thread_stack_free_rcu(rh: *mut rcu_head) {
    let mut vm_stack = container_of!(rh, vm_stack, rcu);
    let mut vm_area = vm_stack.stack_vm_area;
    if (try_release_thread_stack_to_cache(vm_stack.stack_vm_area)) {
    return;
    }
    vfree(vm_area.addr);
    }
#[no_mangle]
unsafe extern "C" fn thread_stack_delayed_free(tsk: *mut task_struct) {
    let mut vm_stack = tsk.stack;
    vm_stack.stack_vm_area = tsk.stack_vm_area;
    call_rcu(&vm_stack.rcu, thread_stack_free_rcu);
    }
#[no_mangle]
unsafe extern "C" fn free_vm_stack_cache(cpu: c_uint) -> c_int {
    let mut cached_vm_stack_areas = per_cpu_ptr(cached_stacks, cpu);
    let mut i = 0;
    while (i < NR_CACHED_STACKS) {
    let mut vm_area = cached_vm_stack_areas[i];
    if (!vm_area) {
    continue;
    }
    vfree(vm_area.addr);
    cached_vm_stack_areas[i] = core::ptr::null_mut();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn memcg_charge_kernel_stack(vm_area: *mut vm_struct) -> c_int {
    let mut i = 0;
    let mut ret = 0;
pub static mut nr_charged: c_int = 0;
// BUG_ON;
    while (i < THREAD_SIZE / PAGE_SIZE) {
    ret = memcg_kmem_charge_page(vm_area.pages[i], GFP_KERNEL, 0);
    if (ret) {
// goto;
    }
    nr_charged += 1;
    }
    return 0;
// label;
    for (i = 0; i < nr_charged; i++) {
    memcg_kmem_uncharge_page(vm_area.pages[i], 0);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn alloc_thread_stack_node(tsk: *mut task_struct, node: c_int) -> c_int {
    let mut vm_area = core::ptr::null_mut();
    let mut stack = core::ptr::null_mut();
    vm_area = alloc_thread_stack_node_from_cache(tsk, node);
    if (vm_area) {
    if (memcg_charge_kernel_stack(vm_area)) {
    vfree(vm_area.addr);
    return -ENOMEM;
    }
// Reset stack metadata.
    if (!kasan_hw_tags_enabled()) {
    kasan_unpoison_range(vm_area.addr, THREAD_SIZE);
    }
    stack = kasan_reset_tag(vm_area.addr);
// Clear stale pointers from reused stack.
    clear_pages(vm_area.addr, vm_area.nr_pages);
    tsk.stack_vm_area = vm_area;
    tsk.stack = stack;
    return 0;
    }
    stack = __vmalloc_node(THREAD_SIZE, THREAD_ALIGN,
    GFP_VMAP_STACK,
    node, __builtin_return_address(0));
    if (!stack) {
    return -ENOMEM;
    }
    vm_area = find_vm_area(stack);
    if (memcg_charge_kernel_stack(vm_area)) {
    vfree(stack);
    return -ENOMEM;
    }
//
// We can't call find_vm_area() in interrupt context, and
// free_thread_stack() can be called in interrupt context,
// so cache the vm_struct.
//
    tsk.stack_vm_area = vm_area;
    stack = kasan_reset_tag(stack);
    tsk.stack = stack;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_thread_stack(tsk: *mut task_struct) {
    if (!try_release_thread_stack_to_cache(tsk.stack_vm_area)) {
    thread_stack_delayed_free(tsk);
    }
    tsk.stack = core::ptr::null_mut();
    tsk.stack_vm_area = core::ptr::null_mut();
    }

//
// Allocate pages if THREAD_SIZE is >= PAGE_SIZE, otherwise use a
// kmemcache based allocator.
//

#[no_mangle]
unsafe extern "C" fn thread_stack_free_rcu(rh: *mut rcu_head) {
    __free_pages(virt_to_page(rh), THREAD_SIZE_ORDER);
    }
#[no_mangle]
unsafe extern "C" fn thread_stack_delayed_free(tsk: *mut task_struct) {
    let mut rh = tsk.stack;
    call_rcu(rh, thread_stack_free_rcu);
    }
#[no_mangle]
unsafe extern "C" fn alloc_thread_stack_node(tsk: *mut task_struct, node: c_int) -> c_int {
    let mut page = alloc_pages_node(node, THREADINFO_GFP,
    THREAD_SIZE_ORDER);
    if (likely(page)) {
    tsk.stack = kasan_reset_tag(page_address(page));
    return 0;
    }
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn free_thread_stack(tsk: *mut task_struct) {
    thread_stack_delayed_free(tsk);
    tsk.stack = core::ptr::null_mut();
    }

pub static mut thread_stack_cache: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn thread_stack_free_rcu(rh: *mut rcu_head) {
    kmem_cache_free(thread_stack_cache, rh);
    }
#[no_mangle]
unsafe extern "C" fn thread_stack_delayed_free(tsk: *mut task_struct) {
    let mut rh = tsk.stack;
    call_rcu(rh, thread_stack_free_rcu);
    }
#[no_mangle]
unsafe extern "C" fn alloc_thread_stack_node(tsk: *mut task_struct, node: c_int) -> c_int {
pub static mut stack: *mut c_void = core::ptr::null_mut();
    stack = kmem_cache_alloc_node(thread_stack_cache, THREADINFO_GFP, node);
    stack = kasan_reset_tag(stack);
    tsk.stack = stack;
    return stack ? 0 : -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn free_thread_stack(tsk: *mut task_struct) {
    thread_stack_delayed_free(tsk);
    tsk.stack = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn thread_stack_cache_init() {
    thread_stack_cache = kmem_cache_create_usercopy("thread_stack",
    THREAD_SIZE, THREAD_SIZE, 0, 0,
    THREAD_SIZE, core::ptr::null_mut());
// BUG_ON;
    }

// SLAB cache for signal_struct structures (tsk->signal)
pub static mut signal_cachep: *mut c_void = core::ptr::null_mut();
// SLAB cache for sighand_struct structures (tsk->sighand)
    let mut sighand_cachep = core::ptr::null_mut();
// SLAB cache for files_struct structures (tsk->files)
    let mut files_cachep = core::ptr::null_mut();
// SLAB cache for fs_struct structures (tsk->fs)
    let mut fs_cachep = core::ptr::null_mut();
// SLAB cache for mm_struct structures (tsk->mm)
pub static mut mm_cachep: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn account_kernel_stack(tsk: *mut task_struct, account: c_int) {
    if (IS_ENABLED!(CONFIG_VMAP_STACK)) {
    let mut vm_area = task_stack_vm_area(tsk);
    let mut i = 0;
    for (i = 0; i < THREAD_SIZE / PAGE_SIZE; i++) {
    mod_lruvec_page_state(vm_area.pages[i], NR_KERNEL_STACK_KB,
    account * (PAGE_SIZE / 1024));
    }
    } else {
    let mut stack = task_stack_page(tsk);
// All stack pages are in the same node.
    mod_lruvec_kmem_state(stack, NR_KERNEL_STACK_KB,
    account * (THREAD_SIZE / 1024));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn exit_task_stack_account(tsk: *mut task_struct) {
    account_kernel_stack(tsk, -1);
    if (IS_ENABLED!(CONFIG_VMAP_STACK)) {
    let mut vm_area = core::ptr::null_mut();
    let mut i = 0;
    vm_area = task_stack_vm_area(tsk);
    for (i = 0; i < THREAD_SIZE / PAGE_SIZE; i++) {
    memcg_kmem_uncharge_page(vm_area.pages[i], 0);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn release_task_stack(tsk: *mut task_struct) {
    if (WARN_ON!(READ_ONCE(tsk.__state) != TASK_DEAD)) {
    return;  /* Better to leak the stack than to free prematurely */
    }
    free_thread_stack(tsk);
    }

#[no_mangle]
pub unsafe extern "C" fn put_task_stack(tsk: *mut task_struct) {
    if (refcount_dec_and_test(&tsk.stack_refcount)) {
    release_task_stack(tsk);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn free_task(tsk: *mut task_struct) {

// WARN_ON_ONCE;

    release_user_cpus_ptr(tsk);
    scs_release(tsk);
    smp_task_ipi_mask_free(tsk);

//
// The task is finally done with both the stack and thread_info,
// so free both.
//
    release_task_stack(tsk);

//
// If the task had a separate stack allocation, it should be gone
// by now.
//
// WARN_ON_ONCE;

    rt_mutex_debug_task_free(tsk);
    ftrace_graph_exit_task(tsk);
    arch_release_task_struct(tsk);
    if (tsk.flags & PF_KTHREAD) {
    free_kthread_struct(tsk);
    }
    bpf_task_storage_free(tsk);
    put_task_exec_state(rcu_access_pointer(tsk.exec_state));
    free_task_struct(tsk);
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn dup_mm_exe_file(mm: *mut mm_struct, oldmm: *mut mm_struct) {
    let mut exe_file = core::ptr::null_mut();
    exe_file = get_mm_exe_file(oldmm);
// RCU_INIT_POINTER;
//
// We depend on the oldmm having properly denied write access to the
// exe_file already.
//
    if (exe_file && exe_file_deny_write_access(exe_file)) {
    pr_warn_once("exe_file_deny_write_access() failed in %s\n", __func__);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn mm_alloc_pgd(mm: *mut mm_struct) -> c_int {
    mm.pgd = pgd_alloc(mm);
    if (unlikely(!mm.pgd)) {
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mm_free_pgd(mm: *mut mm_struct) {
    pgd_free(mm, mm.pgd);
    }

// Macro flag: #define mm_free_pgd(mm)
// static DEFINE_IDA(mm_ida);
#[no_mangle]
pub unsafe extern "C" fn mm_alloc_id(mm: *mut mm_struct) -> c_int {
    let mut ret = 0;
    ret = ida_alloc_range(&mm_ida, MM_ID_MIN, MM_ID_MAX, GFP_KERNEL);
    if (ret < 0) {
    return ret;
    }
    mm.mm_id = ret;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mm_free_id(mm: *mut mm_struct) {
pub static mut id: mm_id_t = 0;
    mm.mm_id = MM_ID_DUMMY;
    if (id == MM_ID_DUMMY) {
    return;
    }
    if (WARN_ON_ONCE!(id < MM_ID_MIN || id > MM_ID_MAX)) {
    return;
    }
    ida_free(&mm_ida, id);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: mm_alloc_id
pub unsafe extern "C" fn mm_alloc_id_dup() { return 0; }
#[no_mangle]
#[no_mangle]
// duplicate fn: mm_free_id
pub unsafe extern "C" fn mm_free_id_dup() {}

#[no_mangle]
unsafe extern "C" fn check_mm(mm: *mut mm_struct) {
    let mut i = 0;
    BUILD_BUG_ON_MSG(ARRAY_SIZE!(resident_page_types) != NR_MM_COUNTERS,
    "Please make sure 'struct resident_page_types[]' is updated as well");
    while (i < NR_MM_COUNTERS) {
pub static mut x: c_long = 0;
    if (unlikely(x)) {
    pr_alert("BUG: Bad rss-counter state mm:%p type:%s val:%ld Comm:%s Pid:%d\n",
    mm, resident_page_types[i], x,
    current.comm,
    task_pid_nr(current));
    }
    }
    if (mm_pgtables_bytes(mm)) {
    pr_alert("BUG: non-zero pgtables_bytes on freeing mm: %ld\n",
    mm_pgtables_bytes(mm));
    }

// VM_BUG_ON_MM;

    }

#[no_mangle]
unsafe extern "C" fn do_check_lazy_tlb(arg: *mut c_void) {
    let mut mm = arg;
// WARN_ON_ONCE;
    }
#[no_mangle]
unsafe extern "C" fn do_shoot_lazy_tlb(arg: *mut c_void) {
    let mut mm = arg;
    if (current.active_mm == mm) {
// WARN_ON_ONCE;
    current.active_mm = &init_mm;
    switch_mm(mm, &init_mm, current);
    }
    }
#[no_mangle]
unsafe extern "C" fn cleanup_lazy_tlbs(mm: *mut mm_struct) {
    if (!IS_ENABLED!(CONFIG_MMU_LAZY_TLB_SHOOTDOWN)) {
//
// In this case, lazy tlb mms are refounted and would not reach
// __mmdrop until all CPUs have switched away and mmdrop()ed.
//
    return;
    }
//
// Lazy mm shootdown does not refcount "lazy tlb mm" usage, rather it
// requires lazy mm users to switch to another mm when the refcount
// drops to zero, before the mm is freed. This requires IPIs here to
// switch kernel threads to init_mm.
//
// archs that use IPIs to flush TLBs can piggy-back that lazy tlb mm
// switch with the final userspace teardown TLB flush which leaves the
// mm lazy on this CPU but no others, reducing the need for additional
// IPIs here. There are cases where a final IPI is still required here,
// such as the final mmdrop being performed on a different CPU than the
// one exiting, or kernel threads using the mm when userspace exits.
//
// IPI overheads have not found to be expensive, but they could be
// reduced in a number of possible ways, for example (roughly
// increasing order of complexity):
// - The last lazy reference created by exit_mm() could instead switch
// to init_mm, however it's probable this will run on the same CPU
// immediately afterwards, so this may not reduce IPIs much.
// - A batch of mms requiring IPIs could be gathered and freed at once.
// - CPUs store active_mm where it can be remotely checked without a
// lock, to filter out false-positives in the cpumask.
// - After mm_users or mm_count reaches zero, switching away from the
// mm could clear mm_cpumask to reduce some IPIs, perhaps together
// with some batching or delaying of the final IPIs.
// - A delayed freeing and RCU-like quiescing sequence based on mm
// switching to avoid IPIs completely.
//
    on_each_cpu_mask(mm_cpumask(mm), do_shoot_lazy_tlb, mm, 1);
    if (IS_ENABLED!(CONFIG_DEBUG_VM_SHOOT_LAZIES)) {
    on_each_cpu(do_check_lazy_tlb, mm, 1);
    }
    }
//
// Called when the last reference to the mm
// is dropped: either by a lazy thread or by
// mmput. Free the page directory and the mm.
//
#[no_mangle]
pub unsafe extern "C" fn __mmdrop(mm: *mut mm_struct) {
// BUG_ON;
// WARN_ON_ONCE;
// Ensure no CPUs are using this as their lazy tlb mm
    cleanup_lazy_tlbs(mm);
// WARN_ON_ONCE;
    mm_destroy_sched(mm);
    mm_free_pgd(mm);
    mm_free_id(mm);
    destroy_context(mm);
    mmu_notifier_subscriptions_destroy(mm);
    check_mm(mm);
    mm_pasid_drop(mm);
    mm_destroy_cid(mm);
    percpu_counter_destroy_many(mm.rss_stat, NR_MM_COUNTERS);
    free_mm(mm);
    }
// EXPORT_SYMBOL_GPL;
#[no_mangle]
unsafe extern "C" fn mmdrop_async_fn(work: *mut work_struct) {
    let mut mm = core::ptr::null_mut();
    mm = container_of!(work, mm_struct, async_put_work);
    __mmdrop(mm);
    }
#[no_mangle]
unsafe extern "C" fn mmdrop_async(mm: *mut mm_struct) {
    if (unlikely(atomic_dec_and_test(&mm.mm_count))) {
// INIT_WORK;
    schedule_work(&mm.async_put_work);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn free_signal_struct(sig: *mut signal_struct) {
    taskstats_tgid_free(sig);
    sched_autogroup_exit(sig);
//
// __mmdrop is not safe to call from softirq context on x86 due to
// pgd_dtor so postpone it to the async context
//
    if (sig.oom_mm) {
    mmdrop_async(sig.oom_mm);
    }
    kmem_cache_free(signal_cachep, sig);
    }
#[no_mangle]
pub unsafe extern "C" fn put_signal_struct(sig: *mut signal_struct) {
    if (refcount_dec_and_test(&sig.sigcnt)) {
    free_signal_struct(sig);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __put_task_struct(tsk: *mut task_struct) {
// WARN_ON;
// WARN_ON;
// WARN_ON;
    unwind_task_free(tsk);
    io_uring_free(tsk);
    cgroup_task_free(tsk);
    task_numa_free(tsk, true);
    security_task_free(tsk);
    exit_creds(tsk);
    delayacct_tsk_free(tsk);
    put_signal_struct(tsk.signal);
    sched_core_free(tsk);
    free_task(tsk);
    }
// EXPORT_SYMBOL_GPL;
#[no_mangle]
pub unsafe extern "C" fn __put_task_struct_rcu_cb(rhp: *mut rcu_head) {
    let mut task = container_of!(rhp, task_struct, rcu);
    __put_task_struct(task);
    }
// EXPORT_SYMBOL_GPL;
    void __init __weak arch_task_cache_init(void) { }
//
// set_max_threads
//
#[no_mangle]
unsafe extern "C" fn set_max_threads(max_threads_suggested: c_uint) -> c_int {
    let mut threads = 0;
pub static mut nr_pages: c_ulong = 0;
//
// The number of threads shall be limited such that the thread
// structures may only consume a small part of the available memory.
//
    if (fls64(nr_pages) + fls64(PAGE_SIZE) > 64) {
    threads = MAX_THREADS;
    }
    else {
    threads = div64_u64((u64) nr_pages * (u64) PAGE_SIZE,
    (u64) THREAD_SIZE * 8UL);
    }
    if (threads > max_threads_suggested) {
    threads = max_threads_suggested;
    }
    max_threads = clamp_t(u64, threads, MIN_THREADS, MAX_THREADS);
    }

// Initialized by the architecture:
    let mut arch_task_struct_size = 0;

#[no_mangle]
unsafe extern "C" fn task_struct_whitelist(offset: *mut c_ulong, size: *mut c_ulong) -> c_int {
// Fetch thread_struct whitelist for the architecture.
    arch_thread_struct_whitelist(offset, size);
//
// Handle zero-sized whitelist or empty thread_struct, otherwise
// adjust offset to position of thread_struct in task_struct.
//
    if (unlikely(*size == 0)) {
// offset = 0;
    }
    else {
// offset += offsetof(task_struct, thread);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn fork_init() -> c_int {
    let mut i = 0;

pub const ARCH_MIN_TASKALIGN: c_int = 0;

pub static mut align: c_int = 0;
    unsigned long useroffset, usersize;
// create a slab on which task_structs can be allocated
    task_struct_whitelist(&useroffset, &usersize);
    task_struct_cachep = kmem_cache_create_usercopy("task_struct",
    arch_task_struct_size, align,
    SLAB_PANIC|SLAB_ACCOUNT,
    useroffset, usersize, core::ptr::null_mut());
// do the arch specific task caches init
    arch_task_cache_init();
    set_max_threads(MAX_THREADS);
    init_task.signal.rlim[RLIMIT_NPROC].rlim_cur = max_threads/2;
    init_task.signal.rlim[RLIMIT_NPROC].rlim_max = max_threads/2;
    init_task.signal.rlim[RLIMIT_SIGPENDING] =
    init_task.signal.rlim[RLIMIT_NPROC];
    for (i = 0; i < UCOUNT_COUNTS; i++) {
    init_user_ns.ucount_max[i] = max_threads/2;
    }
    set_userns_rlimit_max(&init_user_ns, UCOUNT_RLIMIT_NPROC,      RLIM_INFINITY);
    set_userns_rlimit_max(&init_user_ns, UCOUNT_RLIMIT_MSGQUEUE,   RLIM_INFINITY);
    set_userns_rlimit_max(&init_user_ns, UCOUNT_RLIMIT_SIGPENDING, RLIM_INFINITY);
    set_userns_rlimit_max(&init_user_ns, UCOUNT_RLIMIT_MEMLOCK,    RLIM_INFINITY);

    cpuhp_setup_state(CPUHP_BP_PREPARE_DYN, "fork:vm_stack_cache",
    core::ptr::null_mut(), free_vm_stack_cache);

    scs_init();
    lockdep_init_task(&init_task);
    uprobes_init();
    }
    int __weak arch_dup_task_struct(task_struct *dst, task_struct *src)
    {
// dst = *src;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn set_task_stack_end_magic(tsk: *mut task_struct) {
pub static mut stackend: *mut c_void = core::ptr::null_mut();
    stackend = end_of_stack(tsk);
// stackend = STACK_END_MAGIC;	// for overflow detection
    }
#[no_mangle]
pub unsafe extern "C" fn dup_task_struct() {
    let mut tsk = core::ptr::null_mut();
    let mut err = 0;
    if (node == NUMA_NO_NODE) {
    node = tsk_fork_get_node(orig);
    }
    tsk = alloc_task_struct_node(node);
    if (!tsk) {
    return core::ptr::null_mut();
    }
    err = arch_dup_task_struct(tsk, orig);
    if (err) {
// goto;
    }
    err = alloc_thread_stack_node(tsk, node);
    if (err) {
// goto;
    }

    refcount_set(&tsk.stack_refcount, 1);

    account_kernel_stack(tsk, 1);
    err = smp_task_ipi_mask_alloc(tsk);
    if (err) {
// goto;
    }
    err = scs_prepare(tsk, node);
    if (err) {
// goto;
    }

//
// We must handle setting up seccomp filters once we're under
// the sighand lock in case orig has changed between now and
// then. Until then, filter must be NULL to avoid messing up
// the usage counts on the error path calling free_task.
//
    tsk.seccomp.filter = core::ptr::null_mut();

// RCU_INIT_POINTER;
    setup_thread_stack(tsk, orig);
    clear_user_return_notifier(tsk);
    clear_tsk_need_resched(tsk);
    set_task_stack_end_magic(tsk);
    clear_syscall_work_syscall_user_dispatch(tsk);

    tsk.stack_canary = get_random_canary();

    if (orig.cpus_ptr == &orig.cpus_mask) {
    tsk.cpus_ptr = &tsk.cpus_mask;
    }
    dup_user_cpus_ptr(tsk, orig, node);
//
// One for the user space visible state that goes away when reaped.
// One for the scheduler.
//
    refcount_set(&tsk.rcu_users, 2);
// One for the rcu users
    refcount_set(&tsk.usage, 1);

    tsk.btrace_seq = 0;

    tsk.splice_pipe = core::ptr::null_mut();
    tsk.task_frag.page = core::ptr::null_mut();
    tsk.wake_q.next = core::ptr::null_mut();
    tsk.worker_private = core::ptr::null_mut();
    kcov_task_init(tsk);
    kmsan_task_create(tsk);
    kmap_local_fork(tsk);

    tsk.fail_nth = 0;

    tsk.throttle_disk = core::ptr::null_mut();
    tsk.use_memdelay = 0;

    tsk.pasid_activated = 0;

    tsk.active_memcg = core::ptr::null_mut();

    tsk.reported_split_lock = 0;

    tsk.mm_cid.cid = MM_CID_UNSET;
    tsk.mm_cid.active = 0;
// INIT_HLIST_NODE;

// RCU_INIT_POINTER;
    tsk.bpf_ctx = core::ptr::null_mut();

    return tsk;
// label;
    smp_task_ipi_mask_free(tsk);
// label;
    exit_task_stack_account(tsk);
    free_thread_stack(tsk);
// label;
    free_task_struct(tsk);
    return core::ptr::null_mut();
    }
    __cacheline_aligned_in_smp DEFINE_SPINLOCK(mmlist_lock);
pub static mut coredump_filter: unsigned long = 0;
#[no_mangle]
unsafe extern "C" fn coredump_filter_setup(s: *mut c_char) -> c_int {
    if (kstrtoul(s, 0, &coredump_filter)) {
    return 0;
    }
    coredump_filter <<= MMF_DUMP_FILTER_SHIFT;
    coredump_filter &= MMF_DUMP_FILTER_MASK;
    return 1;
    }
// __setup;

#[no_mangle]
unsafe extern "C" fn mm_init_aio(mm: *mut mm_struct) {

    spin_lock_init(&mm.ioctx_lock);
    mm.ioctx_table = core::ptr::null_mut();

    }
    static __always_inline void mm_clear_owner(mm_struct *mm, task_struct *p)
    {

    if (mm.owner == p) {
// WRITE_ONCE;
    }

    }
#[no_mangle]
unsafe extern "C" fn mm_init_owner(mm: *mut mm_struct, p: *mut task_struct) {

    mm.owner = p;

    }
#[no_mangle]
unsafe extern "C" fn mm_init_uprobes_state(mm: *mut mm_struct) {

    mm.uprobes_state.xol_area = core::ptr::null_mut();

    }
#[no_mangle]
unsafe extern "C" fn mmap_init_lock(mm: *mut mm_struct) {
    init_rwsem(&mm.mmap_lock);
    mm_lock_seqcount_init(mm);

    rcuwait_init(&mm.vma_writer_wait);

    }
#[no_mangle]
pub unsafe extern "C" fn mm_init() {
    mt_init_flags(&mm.mm_mt, MM_MT_FLAGS);
    mt_set_external_lock(&mm.mm_mt, &mm.mmap_lock);
    atomic_set(&mm.mm_users, 1);
    atomic_set(&mm.mm_count, 1);
    seqcount_init(&mm.write_protect_seq);
    mmap_init_lock(mm);
// INIT_LIST_HEAD;
    mm_pgtables_bytes_init(mm);
    mm.map_count = 0;
    mm.locked_vm = 0;
    atomic64_set(&mm.pinned_vm, 0);
    memset(&mm.rss_stat, 0, sizeof!(mm.rss_stat));
    spin_lock_init(&mm.page_table_lock);
    spin_lock_init(&mm.arg_lock);
    mm_init_cpumask(mm);
    mm_init_aio(mm);
    mm_init_owner(mm, p);
    mm_pasid_init(mm);
// RCU_INIT_POINTER;
    mmu_notifier_subscriptions_init(mm);
    init_tlb_flush_pending(mm);

    mm.pmd_huge_pte = core::ptr::null_mut();

    mm_init_uprobes_state(mm);
    hugetlb_count_init(mm);
    futex_mm_init(mm);
    mm_flags_clear_all(mm);
    if (current.mm) {
pub static mut flags: c_ulong = 0;
    __mm_flags_overwrite_word(mm, mmf_init_legacy_flags(flags));
    mm.def_flags = current.mm.def_flags & VM_INIT_DEF_MASK;
    } else {
    __mm_flags_overwrite_word(mm, coredump_filter);
    mm.def_flags = 0;
    }
    if (mm_alloc_pgd(mm)) {
// goto;
    }
    if (mm_alloc_id(mm)) {
// goto;
    }
    if (init_new_context(p, mm)) {
// goto;
    }
    if (mm_alloc_cid(mm, p)) {
// goto;
    }
    if (mm_alloc_sched(mm)) {
// goto;
    }
    if (percpu_counter_init_many(mm.rss_stat, 0, GFP_KERNEL_ACCOUNT,
    NR_MM_COUNTERS)) {
// goto;
    }
    lru_gen_init_mm(mm);
    return mm;
// label;
    mm_destroy_sched(mm);
// label;
    mm_destroy_cid(mm);
// label;
    destroy_context(mm);
// label;
    mm_free_id(mm);
// label;
    mm_free_pgd(mm);
// label;
    free_mm(mm);
    return core::ptr::null_mut();
    }
//
// Allocate and initialize an mm_struct.
//
#[no_mangle]
pub unsafe extern "C" fn mm_alloc() {
    let mut mm = core::ptr::null_mut();
    mm = allocate_mm();
    if (!mm) {
    return core::ptr::null_mut();
    }
    memset(mm, 0, sizeof!(*mm));
    return mm_init(mm, current);
    }
// EXPORT_SYMBOL_IF_KUNIT;
#[no_mangle]
pub unsafe extern "C" fn __mmput(mm: *mut mm_struct) {
// VM_BUG_ON;
    uprobe_clear_state(mm);
    exit_aio(mm);
    ksm_exit(mm);
    khugepaged_exit(mm); /* must run before exit_mmap */
    exit_mmap(mm);
    mm_put_huge_zero_folio(mm);
    set_mm_exe_file(mm, core::ptr::null_mut());
    if (!list_empty(&mm.mmlist)) {
    spin_lock(&mmlist_lock);
    list_del(&mm.mmlist);
    spin_unlock(&mmlist_lock);
    }
    if (mm.binfmt) {
// module_put;
    }
    lru_gen_del_mm(mm);
    futex_hash_free(mm);
    mmdrop(mm);
    }
//
// Decrement the use count and release all resources for an mm.
//
#[no_mangle]
pub unsafe extern "C" fn mmput(mm: *mut mm_struct) {
    might_sleep();
    if (atomic_dec_and_test(&mm.mm_users)) {
    __mmput(mm);
    }
    }
// EXPORT_SYMBOL_GPL;

#[no_mangle]
unsafe extern "C" fn mmput_async_fn(work: *mut work_struct) {
    let mut mm = container_of!(work, mm_struct,
    async_put_work);
    __mmput(mm);
    }
#[no_mangle]
pub unsafe extern "C" fn mmput_async(mm: *mut mm_struct) {
    if (atomic_dec_and_test(&mm.mm_users)) {
// INIT_WORK;
    schedule_work(&mm.async_put_work);
    }
    }
// EXPORT_SYMBOL_GPL;

//
// set_mm_exe_file - change a reference to the mm's executable file
// @mm: The mm to change.
// @new_exe_file: The new file to use.
//
// This changes mm's executable file (shown as symlink /proc/[pid]/exe).
//
// Main users are mmput() and sys_execve(). Callers prevent concurrent
// invocations: in mmput() nobody alive left, in execve it happens before
// the new mm is made visible to anyone.
//
// Can only fail if new_exe_file != NULL.
//
#[no_mangle]
pub unsafe extern "C" fn set_mm_exe_file(mm: *mut mm_struct, new_exe_file: *mut file) -> c_int {
    let mut old_exe_file = core::ptr::null_mut();
//
// It is safe to dereference the exe_file without RCU as
// this function is only called if nobody else can access
// this mm -- see comment above for justification.
//
    old_exe_file = rcu_dereference_raw(mm.exe_file);
    if (new_exe_file) {
//
// We expect the caller (i.e., sys_execve) to already denied
// write access, so this is unlikely to fail.
//
    if (unlikely(exe_file_deny_write_access(new_exe_file))) {
    return -EACCES;
    }
    get_file(new_exe_file);
    }
    rcu_assign_pointer(mm.exe_file, new_exe_file);
    if (old_exe_file) {
    exe_file_allow_write_access(old_exe_file);
    fput(old_exe_file);
    }
    return 0;
    }
//
// replace_mm_exe_file - replace a reference to the mm's executable file
// @mm: The mm to change.
// @new_exe_file: The new file to use.
//
// This changes mm's executable file (shown as symlink /proc/[pid]/exe).
//
// Main user is sys_prctl(PR_SET_MM_MAP/EXE_FILE).
//
#[no_mangle]
pub unsafe extern "C" fn replace_mm_exe_file(mm: *mut mm_struct, new_exe_file: *mut file) -> c_int {
    let mut vma = core::ptr::null_mut();
    let mut old_exe_file = core::ptr::null_mut();
pub static mut ret: c_int = 0;
// Forbid mm->exe_file change if old file still mapped.
    old_exe_file = get_mm_exe_file(mm);
    if (old_exe_file) {
// VMA_ITERATOR;
    mmap_read_lock(mm);
    for_each_vma(vmi, vma) {
    if (!vma.vm_file) {
    continue;
    }
    if (path_equal(&vma.vm_file.f_path,
    &old_exe_file.f_path)) {
    ret = -EBUSY;
    break;
    }
    }
    mmap_read_unlock(mm);
    fput(old_exe_file);
    if (ret) {
    return ret;
    }
    }
    ret = exe_file_deny_write_access(new_exe_file);
    if (ret) {
    return -EACCES;
    }
    get_file(new_exe_file);
// set the new file
    mmap_write_lock(mm);
    old_exe_file = rcu_dereference_raw(mm.exe_file);
    rcu_assign_pointer(mm.exe_file, new_exe_file);
    mmap_write_unlock(mm);
    if (old_exe_file) {
    exe_file_allow_write_access(old_exe_file);
    fput(old_exe_file);
    }
    return 0;
    }
//
// get_mm_exe_file - acquire a reference to the mm's executable file
// @mm: The mm of interest.
//
// Returns %NULL if mm has no associated executable file.
// User must release file via fput().
//
#[no_mangle]
pub unsafe extern "C" fn get_mm_exe_file() {
    let mut exe_file = core::ptr::null_mut();
    rcu_read_lock();
    exe_file = get_file_rcu(&mm.exe_file);
    rcu_read_unlock();
    return exe_file;
    }
//
// get_task_exe_file - acquire a reference to the task's executable file
// @task: The task.
//
// Returns %NULL if task's mm (if any) has no associated executable file or
// this is a kernel thread with borrowed mm (see the comment above get_task_mm).
// User must release file via fput().
//
#[no_mangle]
pub unsafe extern "C" fn get_task_exe_file() {
    let mut exe_file = core::ptr::null_mut();
    let mut mm = core::ptr::null_mut();
    if (task.flags & PF_KTHREAD) {
    return core::ptr::null_mut();
    }
    task_lock(task);
    mm = task.mm;
    if (mm) {
    exe_file = get_mm_exe_file(mm);
    }
    task_unlock(task);
    return exe_file;
    }
//
// get_task_mm - acquire a reference to the task's mm
// @task: The task.
//
// Returns %NULL if the task has no mm.  Checks PF_KTHREAD (meaning
// this kernel workthread has transiently adopted a user mm with kthread_use_mm,
// to do its AIO) is not set and if so returns a reference to it, after
// bumping up the use count.  User must release the mm via mmput()
// after use.  Typically used by /proc and ptrace.
//
#[no_mangle]
pub unsafe extern "C" fn get_task_mm() {
    let mut mm = core::ptr::null_mut();
    if (task.flags & PF_KTHREAD) {
    return core::ptr::null_mut();
    }
    task_lock(task);
    mm = task.mm;
    if (mm) {
    mmget(mm);
    }
    task_unlock(task);
    return mm;
    }
// EXPORT_SYMBOL_GPL;
#[no_mangle]
unsafe extern "C" fn may_access_mm(mm: *mut mm_struct, task: *mut task_struct, mode: c_uint) -> bool {
    if (mm == current.mm) {
    return true;
    }
    if (ptrace_may_access(task, mode)) {
    return true;
    }
    if ((mode & PTRACE_MODE_READ) && perfmon_capable()) {
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn mm_access() {
    let mut mm = core::ptr::null_mut();
    let mut err = 0;
    err =  down_read_killable(&task.signal.exec_update_lock);
    if (err) {
    return ERR_PTR(err);
    }
    mm = get_task_mm(task);
    if (!mm) {
    mm = ERR_PTR(-ESRCH);
    } else if (!may_access_mm(mm, task, mode)) {
    mmput(mm);
    mm = ERR_PTR(-EACCES);
    }
    up_read(&task.signal.exec_update_lock);
    return mm;
    }
#[no_mangle]
unsafe extern "C" fn complete_vfork_done(tsk: *mut task_struct) {
    let mut vfork = core::ptr::null_mut();
    task_lock(tsk);
    vfork = tsk.vfork_done;
    if (likely(vfork)) {
    tsk.vfork_done = core::ptr::null_mut();
    complete(vfork);
    }
    task_unlock(tsk);
    }
#[no_mangle]
pub unsafe extern "C" fn wait_for_vfork_done() {
pub static mut state: c_uint = 0;
    let mut killed = 0;
    cgroup_enter_frozen();
    killed = wait_for_completion_state(vfork, state);
    cgroup_leave_frozen(false);
    if (killed) {
    task_lock(child);
    child.vfork_done = core::ptr::null_mut();
    task_unlock(child);
    }
    put_task_struct(child);
    return killed;
    }
// Please note the differences between mmput and mm_release.
// mmput is called whenever we stop holding onto a mm_struct,
// error success whatever.
//
// mm_release is called after a mm_struct has been removed
// from the current process.
//
// This difference is important for error handling, when we
// only half set up a mm_struct for a new process and need to restore
// the old one.  Because we mmput the new mm_struct before
// restoring the old one. . .
// Eric Biederman 10 January 1998
//
#[no_mangle]
unsafe extern "C" fn mm_release(tsk: *mut task_struct, mm: *mut mm_struct) {
    uprobe_free_utask(tsk);
// Get rid of any cached register state
    deactivate_mm(tsk, mm);
//
// Signal userspace if we're not exiting with a core dump
// because we want to leave the value intact for debugging
// purposes.
//
    if (tsk.clear_child_tid) {
    if (atomic_read(&mm.mm_users) > 1) {
//
// We don't check the error code - if userspace has
// not set up a proper pointer then tough luck.
//
    put_user(0, tsk.clear_child_tid);
    do_futex(tsk.clear_child_tid, FUTEX_WAKE,
    1, core::ptr::null_mut(), core::ptr::null_mut(), 0, 0);
    }
    tsk.clear_child_tid = core::ptr::null_mut();
    }
//
// All done, finally we can wake up parent and return this mm to him.
// Also kthread_stop() uses this completion for synchronization.
//
    if (tsk.vfork_done) {
    complete_vfork_done(tsk);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mm_exit_exec_release(tsk: *mut task_struct, mm: *mut mm_struct) {
    futex_exit_exec_release(tsk);
    mm_release(tsk, mm);
    }
//
// dup_mm() - duplicates an existing mm structure
// @tsk: the task_struct with which the new mm will be associated.
// @oldmm: the mm to duplicate.
//
// Allocates a new mm structure and duplicates the provided @oldmm structure
// content into it.
//
// Return: the duplicated mm or NULL on failure.
//
#[no_mangle]
pub unsafe extern "C" fn dup_mm() {
    let mut mm = core::ptr::null_mut();
    let mut err = 0;
    mm = allocate_mm();
    if (!mm) {
// goto;
    }
    memcpy(mm, oldmm, sizeof!(*mm));
    if (!mm_init(mm, tsk)) {
// goto;
    }
    uprobe_start_dup_mmap();
    err = dup_mmap(mm, oldmm);
    if (err) {
// goto;
    }
    uprobe_end_dup_mmap();
    mm.hiwater_rss = get_mm_rss(mm);
    mm.hiwater_vm = mm.total_vm;
    if (mm.binfmt && !try_module_get(mm.binfmt.module)) {
// goto;
    }
    return mm;
// label;
// don't put binfmt in mmput, we haven't got module yet
    mm.binfmt = core::ptr::null_mut();
    mm_init_owner(mm, core::ptr::null_mut());
    mmput(mm);
    if (err) {
    uprobe_end_dup_mmap();
    }
// label;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn copy_mm(clone_flags: u64, tsk: *mut task_struct) -> c_int {
    let mut mm = core::ptr::null_mut();
    let mut oldmm = core::ptr::null_mut();
    tsk.min_flt = tsk.maj_flt = 0;
    tsk.nvcsw = tsk.nivcsw = 0;

    tsk.last_switch_count = tsk.nvcsw + tsk.nivcsw;
    tsk.last_switch_time = 0;

    tsk.mm = core::ptr::null_mut();
    tsk.active_mm = core::ptr::null_mut();
//
// Are we cloning a kernel thread?
//
// We need to steal a active VM for that..
//
    oldmm = current.mm;
    if (!oldmm) {
    return 0;
    }
    if (clone_flags & CLONE_VM) {
    mmget(oldmm);
    mm = oldmm;
    } else {
    mm = dup_mm(tsk, current.mm);
    if (!mm) {
    return -ENOMEM;
    }
    }
    tsk.mm = mm;
    tsk.active_mm = mm;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn copy_exec_state(clone_flags: u64, tsk: *mut task_struct) -> c_int {
    let mut exec_state = core::ptr::null_mut();
// CLONE_VM siblings refcount-share the parent's exec_state.
    if (clone_flags & CLONE_VM) {
    exec_state = rcu_dereference_protected(current.exec_state, true);
    refcount_inc(&exec_state.count);
    rcu_assign_pointer(tsk.exec_state, exec_state);
    return 0;
    }
// Everyone else inherits a fresh copy.
    return task_exec_state_copy(tsk);
    }
#[no_mangle]
unsafe extern "C" fn copy_fs(clone_flags: u64, tsk: *mut task_struct, umh: bool) -> c_int {
    let mut fs = core::ptr::null_mut();
//
// Usermodehelper may copy userspace_init_fs filesystem state but
// they don't get to create mount namespaces, share the
// filesystem state, or be started from a non-initial mount
// namespace.
//
    if (umh) {
    if (clone_flags & (CLONE_NEWNS | CLONE_FS)) {
    return -EINVAL;
    }
    if (current.nsproxy.mnt_ns != &init_mnt_ns) {
    return -EINVAL;
    }
    fs = userspace_init_fs;
    } else {
    fs = current.fs;
// VFS_WARN_ON_ONCE;
    }
    if (clone_flags & CLONE_FS) {
// tsk->fs is already what we want
    read_seqlock_excl(&fs.seq);
// "users" and "in_exec" locked for check_unsafe_exec()
    if (fs.in_exec) {
    read_sequnlock_excl(&fs.seq);
    return -EAGAIN;
    }
    fs.users += 1;
    read_sequnlock_excl(&fs.seq);
    return 0;
    }
    tsk.real_fs = tsk.fs = copy_fs_struct(fs);
    if (!tsk.fs) {
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn copy_files() {
    let mut oldf = core::ptr::null_mut();
    let mut newf = core::ptr::null_mut();
//
// A background process may not have any files ...
//
    oldf = current.files;
    if (!oldf) {
    return 0;
    }
    if (no_files) {
    tsk.files = core::ptr::null_mut();
    return 0;
    }
    if (clone_flags & CLONE_FILES) {
    atomic_inc(&oldf.count);
    return 0;
    }
    newf = dup_fd(oldf, core::ptr::null_mut());
    if (IS_ERR(newf)) {
    return PTR_ERR(newf);
    }
    tsk.files = newf;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn copy_sighand(clone_flags: u64, tsk: *mut task_struct) -> c_int {
    let mut sig = core::ptr::null_mut();
    if (clone_flags & CLONE_SIGHAND) {
    refcount_inc(&current.sighand.count);
    return 0;
    }
    sig = kmem_cache_alloc(sighand_cachep, GFP_KERNEL);
// RCU_INIT_POINTER;
    if (!sig) {
    return -ENOMEM;
    }
    refcount_set(&sig.count, 1);
    spin_lock_irq(&current.sighand.siglock);
    memcpy(sig.action, current.sighand.action, sizeof!(sig.action));
    spin_unlock_irq(&current.sighand.siglock);
// Reset all signal handler not set to SIG_IGN to SIG_DFL.
    if (clone_flags & CLONE_CLEAR_SIGHAND) {
    flush_signal_handlers(tsk, 0);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __cleanup_sighand(sighand: *mut sighand_struct) {
    if (refcount_dec_and_test(&sighand.count)) {
    signalfd_cleanup(sighand);
//
// sighand_cachep is SLAB_TYPESAFE_BY_RCU so we can free it
// without an RCU grace period, see __lock_task_sighand().
//
    kmem_cache_free(sighand_cachep, sighand);
    }
    }
//
// Initialize POSIX timer handling for a thread group.
//
#[no_mangle]
unsafe extern "C" fn posix_cpu_timers_init_group(sig: *mut signal_struct) {
    let mut pct = &sig.posix_cputimers;
    let mut cpu_limit = 0;
    cpu_limit = READ_ONCE(sig.rlim[RLIMIT_CPU].rlim_cur);
    posix_cputimers_group_init(pct, cpu_limit);
    }
#[no_mangle]
unsafe extern "C" fn copy_signal(clone_flags: u64, tsk: *mut task_struct) -> c_int {
    let mut sig = core::ptr::null_mut();
    if (clone_flags & CLONE_THREAD) {
    return 0;
    }
    sig = kmem_cache_zalloc(signal_cachep, GFP_KERNEL);
    tsk.signal = sig;
    if (!sig) {
    return -ENOMEM;
    }
    sig.nr_threads = 1;
    sig.quick_threads = 1;
    atomic_set(&sig.live, 1);
    refcount_set(&sig.sigcnt, 1);
// list_add(thread_node, thread_head) without INIT_LIST_HEAD()
    sig.thread_head = (list_head)LIST_HEAD_INIT(tsk.thread_node);
    tsk.thread_node = (list_head)LIST_HEAD_INIT(sig.thread_head);
    init_waitqueue_head(&sig.wait_chldexit);
    sig.curr_target = tsk;
    init_sigpending(&sig.shared_pending);
// INIT_HLIST_HEAD;
    seqlock_init(&sig.stats_lock);
    prev_cputime_init(&sig.prev_cputime);

// INIT_HLIST_HEAD;
// INIT_HLIST_HEAD;
    hrtimer_setup(&sig.real_timer, it_real_fn, CLOCK_MONOTONIC, HRTIMER_MODE_REL);

    task_lock(current.group_leader);
    memcpy(sig.rlim, current.signal.rlim, sizeof sig.rlim);
    task_unlock(current.group_leader);
    posix_cpu_timers_init_group(sig);
    tty_audit_fork(sig);
    sched_autogroup_fork(sig);

    init_rwsem(&sig.cgroup_threadgroup_rwsem);

    sig.oom_score_adj = current.signal.oom_score_adj;
    sig.oom_score_adj_min = current.signal.oom_score_adj_min;
    mutex_init(&sig.cred_guard_mutex);
    init_rwsem(&sig.exec_update_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn copy_seccomp(p: *mut task_struct) {

//
// Must be called with sighand->lock held, which is common to
// all threads in the group. Holding cred_guard_mutex is not
// needed because this new task is not yet running and cannot
// be racing exec.
//
    assert_spin_locked(&current.sighand.siglock);
// Ref-count the new filter user, and assign it.
    get_seccomp_filter(current);
    p.seccomp = current.seccomp;
//
// Explicitly enable no_new_privs here in case it got set
// between the task_struct being duplicated and holding the
// sighand lock. The seccomp state and nnp must be in sync.
//
    if (task_no_new_privs(current)) {
    task_set_no_new_privs(p);
    }
//
// If the parent gained a seccomp mode after copying thread
// flags and between before we held the sighand lock, we have
// to manually enable the seccomp thread flag here.
//
    if (p.seccomp.mode != SECCOMP_MODE_DISABLED) {
    set_task_syscall_work(p, SECCOMP);
    }

    }
#[no_mangle]
pub unsafe extern "C" fn sys_set_tid_address() {
    current.clear_child_tid = tidptr;
    return task_pid_vnr(current);
    }
#[no_mangle]
unsafe extern "C" fn rt_mutex_init_task(p: *mut task_struct) {
    raw_spin_lock_init(&p.pi_lock);

    p.pi_waiters = RB_ROOT_CACHED;
    p.pi_top_task = core::ptr::null_mut();
    p.pi_blocked_on = core::ptr::null_mut();

    }
#[no_mangle]
pub unsafe extern "C" fn init_task_pid_links(task: *mut task_struct) {
    enum pid_type type;
    for (type = PIDTYPE_PID; type < PIDTYPE_MAX; ++type) {
// INIT_HLIST_NODE;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn init_task_pid() {
    if (type == PIDTYPE_PID) {
    task.thread_pid = pid;
    }
    else {
    task.signal.pids[type] = pid;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rcu_copy_process(p: *mut task_struct) {

    p.rcu_read_lock_nesting = 0;
    p.rcu_read_unlock_special.s = 0;
    p.rcu_blocked_node = core::ptr::null_mut();
// INIT_LIST_HEAD;

    p.rcu_tasks_holdout = false;
// INIT_LIST_HEAD;
    p.rcu_tasks_idle_cpu = -1;
// INIT_LIST_HEAD;

    p.trc_reader_nesting = 0;

    }
//
// pidfd_prepare - allocate a new pidfd_file and reserve a pidfd
// @pid:   the struct pid for which to create a pidfd
// @flags: flags of the new @pidfd
// @ret_file: return the new pidfs file
//
// Allocate a new file that stashes @pid and reserve a new pidfd number in the
// caller's file descriptor table. The pidfd is reserved but not installed yet.
//
// The helper verifies that @pid is still in use, without PIDFD_THREAD the
// task identified by @pid must be a thread-group leader.
//
// If this function returns successfully the caller is responsible to either
// call fd_install() passing the returned pidfd and pidfd file as arguments in
// order to install the pidfd into its file descriptor table or they must use
// put_unused_fd() and fput() on the returned pidfd and pidfd file
// respectively.
//
// This function is useful when a pidfd must already be reserved but there
// might still be points of failure afterwards and the caller wants to ensure
// that no pidfd is leaked into its file descriptor table.
//
// Return: On success, a reserved pidfd is returned from the function and a new
// pidfd file is returned in the last argument to the function. On
// error, a negative error code is returned from the function and the
// last argument remains unchanged.
//
#[no_mangle]
pub unsafe extern "C" fn pidfd_prepare(pid: *mut pid, flags: c_uint, ret_file: *mut file) -> c_int {
    let mut pidfs_file = core::ptr::null_mut();
//
// PIDFD_STALE is only allowed to be passed if the caller knows
// that @pid is already registered in pidfs and thus
// PIDFD_INFO_EXIT information is guaranteed to be available.
//
    if (!(flags & PIDFD_STALE)) {
//
// While holding the pidfd waitqueue lock removing the
// task linkage for the thread-group leader pid
// (PIDTYPE_TGID) isn't possible. Thus, if there's still
// task linkage for PIDTYPE_PID not having thread-group
// leader linkage for the pid means it wasn't a
// thread-group leader in the first place.
//
    guard(spinlock_irq)(&pid.wait_pidfd.lock);
// Task has already been reaped.
    if (!pid_has_task(pid, PIDTYPE_PID)) {
    return -ESRCH;
    }
//
// If this struct pid isn't used as a thread-group
// leader but the caller requested to create a
// thread-group leader pidfd then report ENOENT.
//
    if (!(flags & PIDFD_THREAD) && !pid_has_task(pid, PIDTYPE_TGID)) {
    return -ENOENT;
    }
    }
// CLASS;
    if (pidfd < 0) {
    return pidfd;
    }
    pidfs_file = pidfs_alloc_file(pid, flags | O_RDWR);
    if (IS_ERR(pidfs_file)) {
    return PTR_ERR(pidfs_file);
    }
// ret_file = pidfs_file;
    return take_fd(pidfd);
    }
#[no_mangle]
unsafe extern "C" fn __delayed_free_task(rhp: *mut rcu_head) {
    let mut tsk = container_of!(rhp, task_struct, rcu);
    free_task(tsk);
    }
#[no_mangle]
unsafe extern "C" fn delayed_free_task(tsk: *mut task_struct) -> __always_inline void {
    if (IS_ENABLED!(CONFIG_MEMCG)) {
    call_rcu(&tsk.rcu, __delayed_free_task);
    }
    else {
    free_task(tsk);
    }
    }
#[no_mangle]
unsafe extern "C" fn copy_oom_score_adj(clone_flags: u64, tsk: *mut task_struct) {
// Skip if kernel thread
    if (!tsk.mm) {
    return;
    }
// Skip if spawning a thread or using vfork
    if ((clone_flags & (CLONE_VM | CLONE_THREAD | CLONE_VFORK)) != CLONE_VM) {
    return;
    }
// We need to synchronize with __set_oom_adj
    mutex_lock(&oom_adj_mutex);
    mm_flags_set(MMF_MULTIPROCESS, tsk.mm);
// Update the values in case they were changed after copy_signal
    tsk.signal.oom_score_adj = current.signal.oom_score_adj;
    tsk.signal.oom_score_adj_min = current.signal.oom_score_adj_min;
    mutex_unlock(&oom_adj_mutex);
    }

#[no_mangle]
unsafe extern "C" fn rv_task_fork(p: *mut task_struct) {
    memset(&p.rv, 0, sizeof!(p.rv));
    }

#[no_mangle]
unsafe extern "C" fn need_futex_hash_allocate_default(clone_flags: u64) -> bool {
//
// Allocate a default futex hash for any sibling that will
// share the parent's mm, except vfork.
//
    return (clone_flags & (CLONE_VM | CLONE_VFORK)) == CLONE_VM;
    }
//
// This creates a new process as a copy of the old one,
// but does not actually start it yet.
//
// It copies the registers, and all the appropriate
// parts of the process environment (as per the clone
// flags). The actual kick-off is left to the caller.
//
    __latent_entropy struct task_struct *copy_process(pid *pid,
    int trace,
    int node, kernel_clone_args *args)
    {
pub static mut pidfd: c_int = 0;
    let mut p = core::ptr::null_mut();
    let mut delayed;
    let mut pidfile = core::ptr::null_mut();
pub static mut clone_flags: u64 = 0;
    let mut nsp = current.nsproxy;
//
// Don't allow sharing the root directory with processes in a different
// namespace
//
    if ((clone_flags & (CLONE_NEWNS|CLONE_FS)) == (CLONE_NEWNS|CLONE_FS)) {
    return ERR_PTR(-EINVAL);
    }
    if ((clone_flags & (CLONE_NEWUSER|CLONE_FS)) == (CLONE_NEWUSER|CLONE_FS)) {
    return ERR_PTR(-EINVAL);
    }
//
// Thread groups must share signals as well, and detached threads
// can only be started up within the thread group.
//
    if ((clone_flags & CLONE_THREAD) && !(clone_flags & CLONE_SIGHAND)) {
    return ERR_PTR(-EINVAL);
    }
//
// Shared signal handlers imply shared VM. By way of the above,
// thread groups also imply shared VM. Blocking this case allows
// for various simplifications in other code.
//
    if ((clone_flags & CLONE_SIGHAND) && !(clone_flags & CLONE_VM)) {
    return ERR_PTR(-EINVAL);
    }
//
// Siblings of global init remain as zombies on exit since they are
// not reaped by their parent (swapper). To solve this and to avoid
// multi-rooted process trees, prevent global and container-inits
// from creating siblings.
//
    if ((clone_flags & CLONE_PARENT) &&
    current.signal.flags & SIGNAL_UNKILLABLE) {
    return ERR_PTR(-EINVAL);
    }
//
// If the new process will be in a different pid or user namespace
// do not allow it to share a thread group with the forking task.
//
    if (clone_flags & CLONE_THREAD) {
    if ((clone_flags & (CLONE_NEWUSER | CLONE_NEWPID)) ||
    (task_active_pid_ns(current) != nsp.pid_ns_for_children)) {
    return ERR_PTR(-EINVAL);
    }
    }
    if (clone_flags & CLONE_PIDFD) {
//
// - CLONE_DETACHED is blocked so that we can potentially
// reuse it later for CLONE_PIDFD.
//
    if (clone_flags & CLONE_DETACHED) {
    return ERR_PTR(-EINVAL);
    }
    }
    if (clone_flags & CLONE_AUTOREAP) {
    if (clone_flags & CLONE_THREAD) {
    return ERR_PTR(-EINVAL);
    }
    if (clone_flags & CLONE_PARENT) {
    return ERR_PTR(-EINVAL);
    }
    if (args.exit_signal) {
    return ERR_PTR(-EINVAL);
    }
    }
    if ((clone_flags & CLONE_PARENT) && current.signal.autoreap) {
    return ERR_PTR(-EINVAL);
    }
    if (clone_flags & CLONE_NNP) {
    if (clone_flags & CLONE_THREAD) {
    return ERR_PTR(-EINVAL);
    }
    }
    if (clone_flags & CLONE_PIDFD_AUTOKILL) {
    if (!(clone_flags & CLONE_PIDFD)) {
    return ERR_PTR(-EINVAL);
    }
    if (!(clone_flags & CLONE_AUTOREAP)) {
    return ERR_PTR(-EINVAL);
    }
    if (clone_flags & CLONE_THREAD) {
    return ERR_PTR(-EINVAL);
    }
//
// Without CLONE_NNP the child could escalate privileges
// after being spawned, so require CAP_SYS_ADMIN.
// With CLONE_NNP the child can't gain new privileges,
// so allow unprivileged usage.
//
    if (!(clone_flags & CLONE_NNP) &&
    !ns_capable(current_user_ns(), CAP_SYS_ADMIN)) {
    return ERR_PTR(-EPERM);
    }
    }
//
// Force any signals received before this point to be delivered
// before the fork happens.  Collect up signals sent to multiple
// processes that happen during the fork and delay them so that
// they appear to happen after the fork.
//
    sigemptyset(&delayed.signal);
// INIT_HLIST_NODE;
    spin_lock_irq(&current.sighand.siglock);
    if (!(clone_flags & CLONE_THREAD)) {
    hlist_add_head(&delayed.node, &current.signal.multiprocess);
    }
    recalc_sigpending();
    spin_unlock_irq(&current.sighand.siglock);
    retval = -ERESTARTNOINTR;
    if (task_sigpending(current)) {
// goto;
    }
    retval = -ENOMEM;
    p = dup_task_struct(current, node);
    if (!p) {
// goto;
    }
    retval = copy_exec_state(clone_flags, p);
    if (retval) {
// goto;
    }
    p.flags &= ~PF_KTHREAD;
    if (args.kthread) {
    p.flags |= PF_KTHREAD;
    }
    if (args.user_worker) {
//
// Mark us a user worker, and block any signal that isn't
// fatal or STOP
//
    p.flags |= PF_USER_WORKER;
    siginitsetinv(&p.blocked, sigmask(SIGKILL)|sigmask(SIGSTOP));
    }
    if (args.io_thread) {
    p.flags |= PF_IO_WORKER;
    }
    if (args.name) {
    strscpy_pad(p.comm, args.name, sizeof!(p.comm));
    }
    p.set_child_tid = (clone_flags & CLONE_CHILD_SETTID) ? args.child_tid : core::ptr::null_mut();
//
// TID is cleared in mm_release() when the task exits
//
    p.clear_child_tid = (clone_flags & CLONE_CHILD_CLEARTID) ? args.child_tid : core::ptr::null_mut();
    ftrace_graph_init_task(p);
    rt_mutex_init_task(p);
    raw_spin_lock_init(&p.blocked_lock);
    lockdep_assert_irqs_enabled();

// DEBUG_LOCKS_WARN_ON;

    retval = copy_creds(p, clone_flags);
    if (retval < 0) {
// goto;
    }
    retval = -EAGAIN;
    if (is_rlimit_overlimit(task_ucounts(p), UCOUNT_RLIMIT_NPROC, rlimit(RLIMIT_NPROC))) {
    if (p.real_cred.user != INIT_USER &&
    !capable(CAP_SYS_RESOURCE) && !capable(CAP_SYS_ADMIN)) {
// goto;
    }
    }
    current.flags &= ~PF_NPROC_EXCEEDED;
//
// If multiple threads are within copy_process(), then this check
// triggers too late. This doesn't hurt, the check is only there
// to stop root fork bombs.
//
    retval = -EAGAIN;
    if (data_race(nr_threads >= max_threads)) {
// goto;
    }
    delayacct_tsk_init(p);	/* Must remain after dup_task_struct() */
    p.flags &= ~(PF_SUPERPRIV | PF_WQ_WORKER | PF_IDLE | PF_NO_SETAFFINITY);
    p.flags |= PF_FORKNOEXEC;
// INIT_LIST_HEAD;
// INIT_LIST_HEAD;
    rcu_copy_process(p);
    p.vfork_done = core::ptr::null_mut();
    spin_lock_init(&p.alloc_lock);
    init_sigpending(&p.pending);
    p.utime = p.stime = p.gtime = 0;

    p.utimescaled = p.stimescaled = 0;

    prev_cputime_init(&p.prev_cputime);

    seqcount_init(&p.vtime.seqcount);
    p.vtime.starttime = 0;
    p.vtime.state = VTIME_INACTIVE;

    p.io_uring = core::ptr::null_mut();
    retval = io_uring_fork(p);
    if (unlikely(retval)) {
// goto;
    }
    retval = -EAGAIN;

    p.default_timer_slack_ns = current.timer_slack_ns;

    p.psi_flags = 0;

    task_io_accounting_init(&p.ioac);
    acct_clear_integrals(p);
    posix_cputimers_init(&p.posix_cputimers);
    tick_dep_init_task(p);
    p.io_context = core::ptr::null_mut();
    audit_set_context(p, core::ptr::null_mut());
    cgroup_fork(p);
    if (args.kthread) {
    if (!set_kthread_struct(p)) {
// goto;
    }
    }

    p.mempolicy = mpol_dup(p.mempolicy);
    if (IS_ERR(p.mempolicy)) {
    retval = PTR_ERR(p.mempolicy);
    p.mempolicy = core::ptr::null_mut();
// goto;
    }

    p.cpuset_mem_spread_rotor = NUMA_NO_NODE;
    seqcount_spinlock_init(&p.mems_allowed_seq, &p.alloc_lock);

    memset(&p.irqtrace, 0, sizeof!(p.irqtrace));
    p.irqtrace.hardirq_disable_ip	= _THIS_IP_;
    p.irqtrace.softirq_enable_ip	= _THIS_IP_;
    p.softirqs_enabled		= 1;
    p.softirq_context		= 0;

    p.pagefault_disabled = 0;
    lockdep_init_task(p);
    p.blocked_on = core::ptr::null_mut(); /* not blocked yet */
    p.blocked_donor = core::ptr::null_mut(); /* nobody is boosting p yet */

    p.sequential_io	= 0;
    p.sequential_io_avg	= 0;

    unwind_task_init(p);
// Perform scheduler related setup. Assign this task to a CPU.
    retval = sched_fork(clone_flags, p);
    if (retval) {
// goto;
    }
    retval = perf_event_init_task(p, clone_flags);
    if (retval) {
// goto;
    }
    retval = audit_alloc(p);
    if (retval) {
// goto;
    }
// copy all the process information
    shm_init_task(p);
    retval = security_task_alloc(p, clone_flags);
    if (retval) {
// goto;
    }
    retval = copy_semundo(clone_flags, p);
    if (retval) {
// goto;
    }
    retval = copy_files(clone_flags, p, args.no_files);
    if (retval) {
// goto;
    }
    retval = copy_fs(clone_flags, p, args.umh);
    if (retval) {
// goto;
    }
    retval = copy_sighand(clone_flags, p);
    if (retval) {
// goto;
    }
    retval = copy_signal(clone_flags, p);
    if (retval) {
// goto;
    }
    retval = copy_mm(clone_flags, p);
    if (retval) {
// goto;
    }
    retval = copy_namespaces(clone_flags, p);
    if (retval) {
// goto;
    }
    retval = copy_io(clone_flags, p);
    if (retval) {
// goto;
    }
    retval = copy_thread(p, args);
    if (retval) {
// goto;
    }
    stackleak_task_init(p);
    if (pid != &init_struct_pid) {
    pid = alloc_pid(p.nsproxy.pid_ns_for_children, args.set_tid,
    args.set_tid_size);
    if (IS_ERR(pid)) {
    retval = PTR_ERR(pid);
// goto;
    }
    }
//
// This has to happen after we've potentially unshared the file
// descriptor table (so that the pidfd doesn't leak into the child
// if the fd table isn't shared).
//
    if (clone_flags & CLONE_PIDFD) {
pub static mut flags: unsigned = 0;
    if (clone_flags & CLONE_THREAD) {
    flags |= PIDFD_THREAD;
    }
    if (clone_flags & CLONE_PIDFD_AUTOKILL) {
    flags |= PIDFD_AUTOKILL;
    }
//
// Note that no task has been attached to @pid yet indicate
// that via CLONE_PIDFD.
//
    retval = pidfd_prepare(pid, flags, &pidfile);
    if (retval < 0) {
// goto;
    }
    pidfd = retval;
    retval = put_user(pidfd, args.pidfd);
    if (retval) {
// goto;
    }
    }

    p.plug = core::ptr::null_mut();
    p.flags &= ~PF_BLOCK_TS;

    futex_init_task(p);
//
// sigaltstack should be cleared when sharing the same VM
//
    if ((clone_flags & (CLONE_VM|CLONE_VFORK)) == CLONE_VM) {
    sas_ss_reset(p);
    }
//
// Syscall tracing and stepping should be turned off in the
// child regardless of CLONE_PTRACE.
//
    user_disable_single_step(p);
    clear_task_syscall_work(p, SYSCALL_TRACE);

    clear_task_syscall_work(p, SYSCALL_EMU);

    clear_tsk_latency_tracing(p);
// ok, now we should be set up..
    p.pid = pid_nr(pid);
    if (clone_flags & CLONE_THREAD) {
    p.group_leader = current.group_leader;
    p.tgid = current.tgid;
    } else {
    p.group_leader = p;
    p.tgid = p.pid;
    }
    p.nr_dirtied = 0;
    p.nr_dirtied_pause = 128 >> (PAGE_SHIFT - 10);
    p.dirty_paused_when = 0;
    p.pdeath_signal = 0;
    p.task_works = core::ptr::null_mut();
    clear_posix_cputimers_work(p);

    p.kretprobe_instances.first = core::ptr::null_mut();

    p.rethooks.first = core::ptr::null_mut();

//
// Ensure that the cgroup subsystem policies allow the new process to be
// forked. It should be noted that the new process's css_set can be changed
// between here and cgroup_post_fork() if an organisation operation is in
// progress.
//
    retval = cgroup_can_fork(p, args);
    if (retval) {
// goto;
    }
//
// Now that the cgroups are pinned, re-clone the parent cgroup and put
// the new task on the correct runqueue. All this *before* the task
// becomes visible.
//
// This isn't part of ->can_fork() because while the re-cloning is
// cgroup specific, it unconditionally needs to place the task on a
// runqueue.
//
    retval = sched_cgroup_fork(p, args);
    if (retval) {
// goto;
    }
    if (need_futex_hash_allocate_default(clone_flags)) {
    retval = futex_hash_allocate_default();
    if (retval) {
// goto;
    }
//
// If we fail beyond this point we don't free the allocated
// futex hash map. We assume that another thread will be created
// and makes use of it. The hash map will be freed once the main
// thread terminates.
//
    }
//
// From this point on we must avoid any synchronous user-space
// communication until we take the tasklist-lock. In particular, we do
// not want user-space to be able to predict the process start-time by
// stalling fork(2) after we recorded the start_time but before it is
// visible to the system.
//
    p.start_time = ktime_get_ns();
    p.start_boottime = ktime_get_boottime_ns();
//
// Make it visible to the rest of the system, but dont wake it up yet.
// Need tasklist lock for parent etc handling!
//
    write_lock_irq(&tasklist_lock);
// CLONE_PARENT re-uses the old parent
    if (clone_flags & (CLONE_PARENT|CLONE_THREAD)) {
    p.real_parent = current.real_parent;
    p.parent_exec_id = current.parent_exec_id;
    if (clone_flags & CLONE_THREAD) {
    p.exit_signal = -1;
    }
    else {
    p.exit_signal = current.group_leader.exit_signal;
    }
    } else {
    p.real_parent = current;
    p.parent_exec_id = current.self_exec_id;
    p.exit_signal = args.exit_signal;
    }
    klp_copy_process(p);
    sched_core_fork(p);
    spin_lock(&current.sighand.siglock);
    rv_task_fork(p);
    rseq_fork(p, clone_flags);
//
// If zap_pid_ns_processes() was called after alloc_pid(), the new
// child missed SIGKILL.  If current is not in the same namespace,
// we can't rely on fatal_signal_pending() below.
//
    if (unlikely(!(ns_of_pid(pid).pid_allocated & PIDNS_ADDING))) {
    retval = -ENOMEM;
// goto;
    }
// Let kill terminate clone/fork in the middle
    if (fatal_signal_pending(current)) {
    retval = -EINTR;
// goto;
    }
// No more failure paths after this point.
//
// Copy seccomp details explicitly here, in case they were changed
// before holding sighand lock.
//
    copy_seccomp(p);
    if (clone_flags & CLONE_NNP) {
    task_set_no_new_privs(p);
    }
    init_task_pid_links(p);
    if (likely(p.pid)) {
    ptrace_init_task(p, (clone_flags & CLONE_PTRACE) || trace);
    init_task_pid(p, PIDTYPE_PID, pid);
    if (thread_group_leader(p)) {
    init_task_pid(p, PIDTYPE_TGID, pid);
    init_task_pid(p, PIDTYPE_PGID, task_pgrp(current));
    init_task_pid(p, PIDTYPE_SID, task_session(current));
    if (is_child_reaper(pid)) {
    let mut ns = ns_of_pid(pid);
// ASSERT_EXCLUSIVE_WRITER;
// WRITE_ONCE;
    p.signal.flags |= SIGNAL_UNKILLABLE;
    }
    p.signal.shared_pending.signal = delayed.signal;
    p.signal.tty = tty_kref_get(current.signal.tty);
//
// Inherit has_child_subreaper flag under the same
// tasklist_lock with adding child to the process tree
// for propagate_has_child_subreaper optimization.
//
    p.signal.has_child_subreaper = p.real_parent.signal.has_child_subreaper ||
    p.real_parent.signal.is_child_subreaper;
    if (clone_flags & CLONE_AUTOREAP) {
    p.signal.autoreap = 1;
    }
    list_add_tail(&p.sibling, &p.real_parent.children);
    list_add_tail_rcu(&p.tasks, &init_task.tasks);
    attach_pid(p, PIDTYPE_TGID);
    attach_pid(p, PIDTYPE_PGID);
    attach_pid(p, PIDTYPE_SID);
    __this_cpu_inc(process_counts);
    } else {
    current.signal.nr_threads += 1;
    current.signal.quick_threads += 1;
    atomic_inc(&current.signal.live);
    refcount_inc(&current.signal.sigcnt);
    task_join_group_stop(p);
    list_add_tail_rcu(&p.thread_node,
    &p.signal.thread_head);
    }
    attach_pid(p, PIDTYPE_PID);
    nr_threads += 1;
    }
    total_forks += 1;
    hlist_del_init(&delayed.node);
    spin_unlock(&current.sighand.siglock);
    syscall_tracepoint_update(p);
    write_unlock_irq(&tasklist_lock);
    if (pidfile) {
    fd_install(pidfd, pidfile);
    }
    proc_fork_connector(p);
//
// sched_ext needs @p to be associated with its cgroup in its post_fork
// hook. cgroup_post_fork() should come before sched_post_fork().
//
    cgroup_post_fork(p, args);
    sched_post_fork(p);
    perf_event_fork(p);
    trace_task_newtask(p, clone_flags);
    uprobe_copy_process(p, clone_flags);
    user_events_fork(p, clone_flags);
    copy_oom_score_adj(clone_flags, p);
    return p;
// label;
    sched_core_free(p);
    spin_unlock(&current.sighand.siglock);
    write_unlock_irq(&tasklist_lock);
// label;
    cgroup_cancel_fork(p, args);
// label;
    if (clone_flags & CLONE_PIDFD) {
    fput(pidfile);
    put_unused_fd(pidfd);
    }
// label;
    if (pid != &init_struct_pid) {
    free_pid(pid);
    }
// label;
    exit_thread(p);
// label;
    if (p.io_context) {
    exit_io_context(p);
    }
// label;
    exit_nsproxy_namespaces(p);
// label;
    if (p.mm) {
    mm_clear_owner(p.mm, p);
    mmput(p.mm);
    }
// label;
    if (!(clone_flags & CLONE_THREAD)) {
    free_signal_struct(p.signal);
    }
// label;
    __cleanup_sighand(p.sighand);
// label;
    exit_fs(p); /* blocking */
// label;
    exit_files(p); /* blocking */
// label;
    exit_sem(p);
// label;
    security_task_free(p);
// label;
    audit_free(p);
// label;
    perf_event_free_task(p);
// label;
    sched_cancel_fork(p);
// label;
    lockdep_free_task(p);

    mpol_put(p.mempolicy);

// label;
    io_uring_free(p);
    delayacct_tsk_free(p);
// label;
    dec_rlimit_ucounts(task_ucounts(p), UCOUNT_RLIMIT_NPROC, 1);
    exit_cred_namespaces(p);
    exit_creds(p);
// label;
// WRITE_ONCE;
    exit_task_stack_account(p);
    put_task_stack(p);
    delayed_free_task(p);
// label;
    spin_lock_irq(&current.sighand.siglock);
    hlist_del_init(&delayed.node);
    spin_unlock_irq(&current.sighand.siglock);
    return ERR_PTR(retval);
    }
#[no_mangle]
pub unsafe extern "C" fn init_idle_pids(idle: *mut task_struct) {
    enum pid_type type;
    while (type < PIDTYPE_MAX) {
// INIT_HLIST_NODE; /* not really needed */
    init_task_pid(idle, type, &init_struct_pid);
    }
    }
#[no_mangle]
unsafe extern "C" fn idle_dummy(dummy: *mut c_void) -> c_int {
// This function is never called
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn fork_idle(cpu: c_int) -> *mut task_ __init {
    let mut task = core::ptr::null_mut();
pub static mut kernel_clone_args: usize = 0;
    task = copy_process(&init_struct_pid, 0, cpu_to_node(cpu), &args);
    if (!IS_ERR(task)) {
    init_idle_pids(task);
    init_idle(task, cpu);
    }
    return task;
    }
//
// This is like kernel_clone(), but shaved down and tailored to just
// creating io_uring workers. It returns a created task, or an error pointer.
// The returned task is inactive, and the caller must fire it up through
// wake_up_new_task(p). All signals are blocked in the created task.
//
#[no_mangle]
pub unsafe extern "C" fn create_io_thread() {
    let mut flags = CLONE_FS|CLONE_FILES|CLONE_SIGHAND|CLONE_THREAD|
    CLONE_IO|CLONE_VM|CLONE_UNTRACED;
pub static mut kernel_clone_args: usize = 0;
    return copy_process(core::ptr::null_mut(), 0, node, &args);
    }
//
// Ok, this is the main fork-routine.
//
// It copies the process, and if successful kick-starts
// it and waits for it to finish using the VM if required.
//
#[no_mangle]
pub unsafe extern "C" fn kernel_clone(args: *mut kernel_clone_args) -> pid_t {
pub static mut clone_flags: u64 = 0;
    let mut vfork;
    let mut pid = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
pub static mut trace: c_int = 0;
    let mut nr = 0;
//
// Creating an empty mount namespace implies creating a new mount
// namespace.  Set this before copy_process() so that the
// CLONE_NEWNS|CLONE_FS mutual exclusion check works correctly.
//
    if (clone_flags & CLONE_EMPTY_MNTNS) {
    clone_flags |= CLONE_NEWNS;
    args.flags = clone_flags;
    }
//
// For legacy clone() calls, CLONE_PIDFD uses the parent_tid argument
// to return the pidfd. Hence, CLONE_PIDFD and CLONE_PARENT_SETTID are
// mutually exclusive. With clone3() CLONE_PIDFD has grown a separate
// field in struct clone_args and it still doesn't make sense to have
// them both point at the same memory location. Performing this check
// here has the advantage that we don't need to have a separate helper
// to check for legacy clone().
//
    if ((clone_flags & CLONE_PIDFD) &&
    (clone_flags & CLONE_PARENT_SETTID) &&
    (args.pidfd == args.parent_tid)) {
    return -EINVAL;
    }
    if (!valid_signal(args.exit_signal)) {
    return -EINVAL;
    }
//
// Determine whether and which event to report to ptracer.  When
// called from kernel_thread or CLONE_UNTRACED is explicitly
// requested, no event is reported; otherwise, report if the event
// for the type of forking is enabled.
//
    if (!(clone_flags & CLONE_UNTRACED)) {
    if (clone_flags & CLONE_VFORK) {
    trace = PTRACE_EVENT_VFORK;
    }

    else if (args.exit_signal != SIGCHLD) {
    trace = PTRACE_EVENT_CLONE;
    }
    else {
    trace = PTRACE_EVENT_FORK;
    }
    if (likely(!ptrace_event_enabled(current, trace))) {
    trace = 0;
    }
    }
    p = copy_process(core::ptr::null_mut(), trace, NUMA_NO_NODE, args);
    add_latent_entropy();
    if (IS_ERR(p)) {
    return PTR_ERR(p);
    }
//
// Do this prior waking up the new thread - the thread pointer
// might get invalid after that point, if the thread exits quickly.
//
    trace_sched_process_fork(current, p);
    pid = get_task_pid(p, PIDTYPE_PID);
    nr = pid_vnr(pid);
    if (clone_flags & CLONE_PARENT_SETTID) {
    put_user(nr, args.parent_tid);
    }
    if (clone_flags & CLONE_VFORK) {
    p.vfork_done = &vfork;
    init_completion(&vfork);
    get_task_struct(p);
    }
    if (IS_ENABLED!(CONFIG_LRU_GEN_WALKS_MMU) && !(clone_flags & CLONE_VM)) {
// lock the task to synchronize with memcg migration
    task_lock(p);
    lru_gen_add_mm(p.mm);
    task_unlock(p);
    }
    wake_up_new_task(p);
// forking complete and child started to run, tell ptracer
    if (unlikely(trace)) {
    ptrace_event_pid(trace, pid);
    }
    if (clone_flags & CLONE_VFORK) {
    if (!wait_for_vfork_done(p, &vfork)) {
    ptrace_event_pid(PTRACE_EVENT_VFORK_DONE, pid);
    }
    }
    put_pid(pid);
    return nr;
    }
//
// Create a kernel thread.
//
#[no_mangle]
pub unsafe extern "C" fn kernel_thread() {
pub static mut kernel_clone_args: usize = 0;
    return kernel_clone(&args);
    }
//
// Create a user mode thread.
//
#[no_mangle]
pub unsafe extern "C" fn user_mode_thread(): *mut *mut int (fn)(void, arg: *mut c_void, flags: c_ulong) -> pid_t {
#[no_mangle]
#[no_mangle]
// duplicate fn: user_mode_thread
pub unsafe extern "C" fn user_mode_thread_dup() {
pub static mut kernel_clone_args: usize = 0;
    return kernel_clone(&args);
    }

#[no_mangle]
pub unsafe extern "C" fn sys_fork() {

pub static mut kernel_clone_args: usize = 0;
    return kernel_clone(&args);

// can not support in nommu mode
    return -EINVAL;

    }

#[no_mangle]
pub unsafe extern "C" fn sys_vfork() {
pub static mut kernel_clone_args: usize = 0;
    return kernel_clone(&args);
    }

    SYSCALL_DEFINE5(clone, unsigned long, clone_flags, unsigned long, newsp,
    int  *, parent_tidptr,
    unsigned long, tls,
    int  *, child_tidptr)

    SYSCALL_DEFINE5(clone, unsigned long, newsp, unsigned long, clone_flags,
    int  *, parent_tidptr,
    int  *, child_tidptr,
    unsigned long, tls)

    SYSCALL_DEFINE6(clone, unsigned long, clone_flags, unsigned long, newsp,
    int, stack_size,
    int  *, parent_tidptr,
    int  *, child_tidptr,
    unsigned long, tls)

#[no_mangle]
pub unsafe extern "C" fn sys_clone() {
pub static mut kernel_clone_args: usize = 0;
    return kernel_clone(&args);
    }

    static noinline int copy_clone_args_from_user(kernel_clone_args *kargs, clone_args  *uargs,
    size_t usize)
    {
    let mut err = 0;
    let mut args;
    let mut kset_tid = kargs.set_tid;
    BUILD_BUG_ON!(offsetofend(clone_args, tls) !=
    CLONE_ARGS_SIZE_VER0);
    BUILD_BUG_ON!(offsetofend(clone_args, set_tid_size) !=
    CLONE_ARGS_SIZE_VER1);
    BUILD_BUG_ON!(offsetofend(clone_args, cgroup) !=
    CLONE_ARGS_SIZE_VER2);
// BUILD_BUG_ON;
    if (unlikely(usize > PAGE_SIZE)) {
    return -E2BIG;
    }
    if (unlikely(usize < CLONE_ARGS_SIZE_VER0)) {
    return -EINVAL;
    }
    err = copy_struct_from_user(&args, sizeof!(args), uargs, usize);
    if (err) {
    return err;
    }
    if (unlikely(args.set_tid_size > MAX_PID_NS_LEVEL)) {
    return -EINVAL;
    }
    if (unlikely(!args.set_tid && args.set_tid_size > 0)) {
    return -EINVAL;
    }
    if (unlikely(args.set_tid && args.set_tid_size == 0)) {
    return -EINVAL;
    }
//
// Verify that higher 32bits of exit_signal are unset
//
    if (unlikely(args.exit_signal & ~((u64)CSIGNAL))) {
    return -EINVAL;
    }
    if ((args.flags & CLONE_INTO_CGROUP) &&
    (args.cgroup > INT_MAX || usize < CLONE_ARGS_SIZE_VER2)) {
    return -EINVAL;
    }
// kargs = (kernel_clone_args){
    .flags		= args.flags,
    .pidfd		= u64_to_user_ptr(args.pidfd),
    .child_tid	= u64_to_user_ptr(args.child_tid),
    .parent_tid	= u64_to_user_ptr(args.parent_tid),
    .exit_signal	= args.exit_signal,
    .stack		= args.stack,
    .stack_size	= args.stack_size,
    .tls		= args.tls,
    .set_tid_size	= args.set_tid_size,
    .cgroup		= args.cgroup,
    };
    if (args.set_tid &&
    copy_from_user(kset_tid, u64_to_user_ptr(args.set_tid),
    (kargs.set_tid_size * sizeof!(pid_t)))) {
    return -EFAULT;
    }
    kargs.set_tid = kset_tid;
    return 0;
    }
//
// clone3_stack_valid - check and prepare stack
// @kargs: kernel clone args
//
// Verify that the stack arguments userspace gave us are sane.
// In addition, set the stack direction for userspace since it's easy for us to
// determine.
//
#[no_mangle]
pub unsafe extern "C" fn clone3_stack_valid(kargs: *mut kernel_clone_args) -> bool {
    if (kargs.stack == 0) {
    if (kargs.stack_size > 0) {
    return false;
    }
    } else {
    if (kargs.stack_size == 0) {
    return false;
    }
    if (!access_ok(kargs.stack, kargs.stack_size)) {
    return false;
    }

    kargs.stack += kargs.stack_size;

    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn clone3_args_valid(kargs: *mut kernel_clone_args) -> bool {
// Verify that no unknown flags are passed along.
    if (kargs.flags &
    ~(CLONE_LEGACY_FLAGS | CLONE_CLEAR_SIGHAND |
    CLONE_INTO_CGROUP | CLONE_AUTOREAP | CLONE_NNP |
    CLONE_PIDFD_AUTOKILL | CLONE_EMPTY_MNTNS)) {
    return false;
    }
//
// - make the CLONE_DETACHED bit reusable for clone3
// - make the CSIGNAL bits reusable for clone3
//
    if (kargs.flags & (CLONE_DETACHED | (CSIGNAL & (~CLONE_NEWTIME)))) {
    return false;
    }
    if ((kargs.flags & (CLONE_SIGHAND | CLONE_CLEAR_SIGHAND)) ==
    (CLONE_SIGHAND | CLONE_CLEAR_SIGHAND)) {
    return false;
    }
    if ((kargs.flags & (CLONE_THREAD | CLONE_PARENT)) &&
    kargs.exit_signal) {
    return false;
    }
    if (!clone3_stack_valid(kargs)) {
    return false;
    }
    return true;
    }
//
// sys_clone3 - create a new process with specific properties
// @uargs: argument structure
// @size:  size of @uargs
//
// clone3() is the extensible successor to clone()/clone2().
// It takes a struct as argument that is versioned by its size.
//
// Return: On success, a positive PID for the child process.
// On error, a negative errno number.
//
#[no_mangle]
pub unsafe extern "C" fn sys_clone3() {
    let mut err = 0;
    let mut kargs;
    pid_t set_tid[MAX_PID_NS_LEVEL];

    return -ENOSYS;

    kargs.set_tid = set_tid;
    err = copy_clone_args_from_user(&kargs, uargs, size);
    if (err) {
    return err;
    }
    if (!clone3_args_valid(&kargs)) {
    return -EINVAL;
    }
    return kernel_clone(&kargs);
    }
#[no_mangle]
pub unsafe extern "C" fn walk_process_tree(top: *mut task_struct, visitor: proc_visitor, data: *mut c_void) {
    let mut leader = core::ptr::null_mut();
    let mut parent = core::ptr::null_mut();
    let mut child = core::ptr::null_mut();
    let mut res = 0;
    read_lock(&tasklist_lock);
    leader = top = top.group_leader;
// label;
    for_each_thread(leader, parent) {
    list_for_each_entry(child, &parent.children, sibling) {
    res = visitor(child, data);
    if (res) {
    if (res < 0) {
// goto;
    }
    leader = child;
// goto;
    }
// label;
    ;
    }
    }
    if (leader != top) {
    child = leader;
    parent = child.real_parent;
    leader = parent.group_leader;
// goto;
    }
// label;
    read_unlock(&tasklist_lock);
    }

pub const ARCH_MIN_MMSTRUCT_ALIGN: c_int = 0;

#[no_mangle]
unsafe extern "C" fn sighand_ctor(data: *mut c_void) {
    let mut sighand = data;
    spin_lock_init(&sighand.siglock);
    init_waitqueue_head(&sighand.signalfd_wqh);
    }
#[no_mangle]
pub unsafe extern "C" fn mm_cache_init() -> c_int {
    let mut mm_size = 0;
//
// The mm_cpumask is located at the end of mm_struct, and is
// dynamically sized based on the maximum CPU number this system
// can have, taking hotplug into account (nr_cpu_ids).
//
    mm_size = sizeof!(mm_struct) + cpumask_size() + mm_cid_size();
    mm_cachep = kmem_cache_create_usercopy("mm_struct",
    mm_size, ARCH_MIN_MMSTRUCT_ALIGN,
    SLAB_HWCACHE_ALIGN|SLAB_PANIC|SLAB_ACCOUNT,
    offsetof(mm_struct, saved_auxv),
    sizeof_field(mm_struct, saved_auxv),
    core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn proc_caches_init() -> c_int {
    sighand_cachep = kmem_cache_create("sighand_cache",
    sizeof!(sighand_struct), 0,
    SLAB_HWCACHE_ALIGN|SLAB_PANIC|SLAB_TYPESAFE_BY_RCU|
    SLAB_ACCOUNT, sighand_ctor);
    signal_cachep = kmem_cache_create("signal_cache",
    sizeof!(signal_struct), 0,
    SLAB_HWCACHE_ALIGN|SLAB_PANIC|SLAB_ACCOUNT,
    core::ptr::null_mut());
    exec_state_init();
    files_cachep = kmem_cache_create("files_cache",
    sizeof!(files_struct), 0,
    SLAB_HWCACHE_ALIGN|SLAB_PANIC|SLAB_ACCOUNT,
    core::ptr::null_mut());
    fs_cachep = kmem_cache_create("fs_cache",
    sizeof!(fs_struct), 0,
    SLAB_HWCACHE_ALIGN|SLAB_PANIC|SLAB_ACCOUNT,
    core::ptr::null_mut());
    mmap_init();
    nsproxy_cache_init();
    }
//
// Check constraints on flags passed to the unshare system call.
//
#[no_mangle]
unsafe extern "C" fn check_unshare_flags(unshare_flags: c_ulong) -> c_int {
    if (unshare_flags & ~(CLONE_THREAD|CLONE_FS|CLONE_SIGHAND|
    CLONE_VM|CLONE_FILES|CLONE_SYSVSEM|
    CLONE_NS_ALL | UNSHARE_EMPTY_MNTNS)) {
    return -EINVAL;
    }
//
// Not implemented, but pretend it works if there is nothing
// to unshare.  Note that unsharing the address space or the
// signal handlers also need to unshare the signal queues (aka
// CLONE_THREAD).
//
    if (unshare_flags & (CLONE_THREAD | CLONE_SIGHAND | CLONE_VM)) {
    if (!thread_group_empty(current)) {
    return -EINVAL;
    }
    }
    if (unshare_flags & (CLONE_SIGHAND | CLONE_VM)) {
    if (refcount_read(&current.sighand.count) > 1) {
    return -EINVAL;
    }
    }
    if (unshare_flags & CLONE_VM) {
    if (!current_is_single_threaded()) {
    return -EINVAL;
    }
    }
    return 0;
    }
//
// Unshare the filesystem structure if it is being shared
//
#[no_mangle]
unsafe extern "C" fn unshare_fs(unshare_flags: c_ulong, new_fsp: *mut fs_struct) -> c_int {
    let mut fs = current.fs;
    if (!(unshare_flags & CLONE_FS) || !fs) {
    return 0;
    }
// don't need lock here; in the worst case we'll do useless copy
    if (!(unshare_flags & CLONE_NEWNS) && fs.users == 1) {
    return 0;
    }
// new_fsp = copy_fs_struct(fs);
    if (!*new_fsp) {
    return -ENOMEM;
    }
    return 0;
    }
//
// Unshare file descriptor table if it is being shared
//
#[no_mangle]
unsafe extern "C" fn unshare_fd(unshare_flags: c_ulong, new_fdp: *mut files_struct) -> c_int {
    let mut fd = current.files;
    if ((unshare_flags & CLONE_FILES) &&
    (fd && atomic_read(&fd.count) > 1)) {
    fd = dup_fd(fd, core::ptr::null_mut());
    if (IS_ERR(fd)) {
    return PTR_ERR(fd);
    }
// new_fdp = fd;
    }
    return 0;
    }
//
// unshare allows a process to 'unshare' part of the process
// context which was originally shared using clone.  copy_
// functions used by kernel_clone() cannot be used here directly
// because they modify an inactive task_struct that is being
// constructed. Here we are modifying the current, active,
// task_struct.
//
#[no_mangle]
pub unsafe extern "C" fn ksys_unshare(unshare_flags: c_ulong) -> c_int {
    let mut new_fs = core::ptr::null_mut();
    let mut new_fd = core::ptr::null_mut();
    let mut new_cred = core::ptr::null_mut();
    let mut new_nsproxy = core::ptr::null_mut();
pub static mut do_sysvsem: c_int = 0;
    let mut err = 0;
//
// If unsharing a user namespace must also unshare the thread group
// and unshare the filesystem root and working directories.
//
    if (unshare_flags & CLONE_NEWUSER) {
    unshare_flags |= CLONE_THREAD | CLONE_FS;
    }
//
// If unsharing vm, must also unshare signal handlers.
//
    if (unshare_flags & CLONE_VM) {
    unshare_flags |= CLONE_SIGHAND;
    }
//
// If unsharing a signal handlers, must also unshare the signal queues.
//
    if (unshare_flags & CLONE_SIGHAND) {
    unshare_flags |= CLONE_THREAD;
    }
//
// If unsharing namespace, must also unshare filesystem information.
//
    if (unshare_flags & UNSHARE_EMPTY_MNTNS) {
    unshare_flags |= CLONE_NEWNS;
    }
    if (unshare_flags & CLONE_NEWNS) {
    unshare_flags |= CLONE_FS;
    }
// No unsharing with overriden fs state
    VFS_WARN_ON_ONCE(unshare_flags & (CLONE_NEWNS | CLONE_FS) &&
    current.fs != current.real_fs);
    err = check_unshare_flags(unshare_flags);
    if (err) {
// goto;
    }
//
// CLONE_NEWIPC must also detach from the undolist: after switching
// to a new ipc namespace, the semaphore arrays from the old
// namespace are unreachable.
//
    if (unshare_flags & (CLONE_NEWIPC|CLONE_SYSVSEM)) {
    do_sysvsem = 1;
    }
    err = unshare_fs(unshare_flags, &new_fs);
    if (err) {
// goto;
    }
    err = unshare_fd(unshare_flags, &new_fd);
    if (err) {
// goto;
    }
    err = unshare_userns(unshare_flags, &new_cred);
    if (err) {
// goto;
    }
    err = unshare_nsproxy_namespaces(unshare_flags, &new_nsproxy,
    new_cred, new_fs);
    if (err) {
// goto;
    }
    if (new_cred) {
    err = set_cred_ucounts(new_cred);
    if (err) {
// goto;
    }
    }
    if (new_fs || new_fd || do_sysvsem || new_cred || new_nsproxy) {
    if (do_sysvsem) {
//
// CLONE_SYSVSEM is equivalent to sys_exit().
//
    exit_sem(current);
    }
    if (unshare_flags & CLONE_NEWIPC) {
// Orphan segments in old ns (see sem above).
    exit_shm(current);
    shm_init_task(current);
    }
    if (new_nsproxy) {
    switch_task_namespaces(current, new_nsproxy);
    new_nsproxy = core::ptr::null_mut();
    }
    if (new_fs) {
    new_fs = switch_fs_struct(new_fs);
    }
    if (new_fd) {
    guard(task_lock)(current);
    swap(current.files, new_fd);
    }
    if (new_cred) {
// Install the new user namespace
    commit_creds(new_cred);
    new_cred = core::ptr::null_mut();
    }
    }
    perf_event_namespaces(current);
// label;
    if (new_nsproxy) {
    put_nsproxy(new_nsproxy);
    }
// label;
    if (new_cred) {
    put_cred(new_cred);
    }
// label;
    if (new_fd) {
    put_files_struct(new_fd);
    }
// label;
    if (new_fs) {
    free_fs_struct(new_fs);
    }
// label;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_unshare() {
    return ksys_unshare(unshare_flags);
    }
//
// Helper to unshare the files of the current task.
// We don't want to expose copy_files internals to
// the exec layer of the kernel.
//
#[no_mangle]
pub unsafe extern "C" fn unshare_files() -> c_int {
    let mut task = current;
    struct files_struct *old, *copy = core::ptr::null_mut();
    let mut error = 0;
    error = unshare_fd(CLONE_FILES, &copy);
    if (error || !copy) {
    return error;
    }
    old = task.files;
    task_lock(task);
    task.files = copy;
    task_unlock(task);
    put_files_struct(old);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sysctl_max_threads() {
    let mut t;
    let mut ret = 0;
pub static mut threads: c_int = 0;
pub static mut min: c_int = 1;
pub static mut max: c_int = 0;
    t = *table;
    t.data = &threads;
    t.extra1 = &min;
    t.extra2 = &max;
    ret = proc_dointvec_minmax(&t, write, buffer, lenp, ppos);
    if (ret || !write) {
    return ret;
    }
    max_threads = threads;
    return 0;
    }
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn init_fork_sysctl() -> c_int {
    register_sysctl_init("kernel", fork_sysctl_table);
    return 0;
    }
// subsys_initcall;

}