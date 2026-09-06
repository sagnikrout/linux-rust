//! Automatically rewritten from C to Rust
//! Source: mm/percpu-stats.c
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
// Copyright (C) 2017		Facebook Inc.
// Copyright (C) 2017		Dennis Zhou <dennis@kernel.org>
//
// Prints statistics about the percpu allocator and backing chunks.
//

    seq_printf(m, "  %-20s: %12lld\n", X, (long long int)Y)
pub static mut pcpu_stats: usize = 0;
pub static mut pcpu_stats_ai: usize = 0;
#[no_mangle]
unsafe extern "C" fn cmpint(a: *const c_void, b: *const c_void) -> c_int {
    return *a - *b;
    }
//
// Iterates over all chunks to find the max nr_alloc entries.
//
#[no_mangle]
unsafe extern "C" fn find_max_nr_alloc() -> c_int {
pub static mut chunk: *mut c_void = core::ptr::null_mut();
    let mut slot = 0;
    let mut max_nr_alloc = 0;
    max_nr_alloc = 0;
    for (slot = 0; slot < pcpu_nr_slots; slot++) {
    list_for_each_entry(chunk, &pcpu_chunk_lists[slot], list)
    max_nr_alloc = max(max_nr_alloc, chunk.nr_alloc);
    }
    return max_nr_alloc;
    }
//
// Prints out chunk state. Fragmentation is considered between
// the beginning of the chunk to the last allocation.
//
// All statistics are in bytes unless stated otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn chunk_map_stats(m: *mut seq_file, chunk: *mut pcpu_chunk, buffer: *mut c_int) {
    let mut chunk_md = &chunk.chunk_md;
    let mut i = 0;
    let mut last_alloc = 0;
    let mut as_len = 0;
    let mut start = 0;
    let mut end = 0;
    let mut alloc_sizes = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
// statistics
pub static mut sum_frag: c_int = 0;
pub static mut cur_min_alloc: c_int = 0;
    alloc_sizes = buffer;
//
// find_last_bit returns the start value if nothing found.
// Therefore, we must determine if it is a failure of find_last_bit
// and set the appropriate value.
//
    last_alloc = find_last_bit(chunk.alloc_map,
    pcpu_chunk_map_bits(chunk) -
    chunk.end_offset / PCPU_MIN_ALLOC_SIZE - 1);
    last_alloc = test_bit(last_alloc, chunk.alloc_map) ?
    last_alloc + 1 : 0;
    as_len = 0;
    start = chunk.start_offset / PCPU_MIN_ALLOC_SIZE;
//
// If a bit is set in the allocation map, the bound_map identifies
// where the allocation ends.  If the allocation is not set, the
// bound_map does not identify free areas as it is only kept accurate
// on allocation, not free.
//
// Positive values are allocations and negative values are free
// fragments.
//
    while (start < last_alloc) {
    if (test_bit(start, chunk.alloc_map)) {
    end = find_next_bit(chunk.bound_map, last_alloc,
    start + 1);
    alloc_sizes[as_len] = 1;
    } else {
    end = find_next_bit(chunk.alloc_map, last_alloc,
    start + 1);
    alloc_sizes[as_len] = -1;
    }
    alloc_sizes[as_len++] *= (end - start) * PCPU_MIN_ALLOC_SIZE;
    start = end;
    }
//
// The negative values are free fragments and thus sorting gives the
// free fragments at the beginning in largest first order.
//
    if (as_len > 0) {
    sort(alloc_sizes, as_len, sizeof!(int), cmpint, core::ptr::null_mut());
// iterate through the unallocated fragments
    while (*p < 0 && i < as_len) {
    sum_frag -= *p;
    max_frag = max(max_frag, -1 * (*p));
    }
    cur_min_alloc = alloc_sizes[i];
    cur_med_alloc = alloc_sizes[(i + as_len - 1) / 2];
    cur_max_alloc = alloc_sizes[as_len - 1];
    }
    P("nr_alloc", chunk.nr_alloc);
    P("max_alloc_size", chunk.max_alloc_size);
    P("empty_pop_pages", chunk.nr_empty_pop_pages);
    P("first_bit", chunk_md.first_free);
    P("free_bytes", chunk.free_bytes);
    P("contig_bytes", chunk_md.contig_hint * PCPU_MIN_ALLOC_SIZE);
    P("sum_frag", sum_frag);
    P("max_frag", max_frag);
    P("cur_min_alloc", cur_min_alloc);
    P("cur_med_alloc", cur_med_alloc);
    P("cur_max_alloc", cur_max_alloc);
    seq_putc(m, '\n');
    }
#[no_mangle]
unsafe extern "C" fn percpu_stats_show(m: *mut seq_file, v: *mut c_void) -> c_int {
pub static mut chunk: *mut c_void = core::ptr::null_mut();
    let mut slot = 0;
    let mut max_nr_alloc = 0;
pub static mut buffer: *mut c_void = core::ptr::null_mut();
// label;
    spin_lock_irq(&pcpu_lock);
    max_nr_alloc = find_max_nr_alloc();
    spin_unlock_irq(&pcpu_lock);
// there can be at most this many free and allocated fragments
    buffer = vmalloc_array(2 * max_nr_alloc + 1, sizeof!(int));
    if (!buffer) {
    return -ENOMEM;
    }
    spin_lock_irq(&pcpu_lock);
// if the buffer allocated earlier is too small
    if (max_nr_alloc < find_max_nr_alloc()) {
    spin_unlock_irq(&pcpu_lock);
    vfree(buffer);
// goto;
    }

    seq_printf(m, "  %-20s: %12lld\n", #X, (long long int)pcpu_stats_ai.X)
    seq_printf(m,
    "Percpu Memory Statistics\n"
    "Allocation Info:\n"
    "----------------------------------------\n");
    PL(unit_size);
    PL(static_size);
    PL(reserved_size);
    PL(dyn_size);
    PL(atom_size);
    PL(alloc_size);
    seq_putc(m, '\n');

    seq_printf(m, "  %-20s: %12llu\n", #X, (unsigned long long)pcpu_stats.X)
    seq_printf(m,
    "Global Stats:\n"
    "----------------------------------------\n");
    PU(nr_alloc);
    PU(nr_dealloc);
    PU(nr_cur_alloc);
    PU(nr_max_alloc);
    PU(nr_chunks);
    PU(nr_max_chunks);
    PU(min_alloc_size);
    PU(max_alloc_size);
    P("empty_pop_pages", pcpu_nr_empty_pop_pages);
    seq_putc(m, '\n');

    seq_printf(m,
    "Per Chunk Stats:\n"
    "----------------------------------------\n");
    if (pcpu_reserved_chunk) {
    seq_puts(m, "Chunk: <- Reserved Chunk\n");
    chunk_map_stats(m, pcpu_reserved_chunk, buffer);
    }
    while (slot < pcpu_nr_slots) {
    list_for_each_entry(chunk, &pcpu_chunk_lists[slot], list) {
    if (chunk == pcpu_first_chunk) {
    seq_puts(m, "Chunk: <- First Chunk\n");
    }

    else if (slot == pcpu_to_depopulate_slot) {
    seq_puts(m, "Chunk (to_depopulate)\n");
    }

    else if (slot == pcpu_sidelined_slot) {
    seq_puts(m, "Chunk (sidelined):\n");
    }
    else {
    seq_puts(m, "Chunk:\n");
    }
    chunk_map_stats(m, chunk, buffer);
    }
    }
    spin_unlock_irq(&pcpu_lock);
    vfree(buffer);
    return 0;
    }
pub static mut percpu_stats: usize = 0;
#[no_mangle]
unsafe extern "C" fn init_percpu_stats_debugfs() -> c_int {
    debugfs_create_file("percpu_stats", 0444, core::ptr::null_mut(), core::ptr::null_mut(),
    &percpu_stats_fops);
    return 0;
    }
    late_initcall!(init_percpu_stats_debugfs);