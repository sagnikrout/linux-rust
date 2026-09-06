//! Automatically rewritten from C Header to Rust Module
//! Source: mm/percpu-internal.h
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
// pcpu_block_md is the metadata block struct.
// Each chunk's bitmap is split into a number of full blocks.
// All units are in terms of bits.
//
// The scan hint is the largest known contiguous area before the contig hint.
// It is not necessarily the actual largest contig hint though.  There is an
// invariant that the scan_hint_start > contig_hint_start iff
// scan_hint == contig_hint.  This is necessary because when scanning forward,
// we don't know if a new contig hint would be better than the current one.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu_block_md {
//     pub /: *mut *mut int scan_hint; / scan hint for block,
    pub starting: *mut *mut int scan_hint_start; / block relative,
//     pub /: *mut *mut int contig_hint; / contig hint for block,
    pub starting: *mut *mut int contig_hint_start; / block relative,
    pub along: *mut *mut int left_free; / size of free space,
    pub along: *mut *mut int right_free; / size of free space,
//     pub /: *mut *mut int first_free; / block position of first free,
//     pub /: *mut *mut int nr_bits; / total bits responsible for,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpuobj_ext {

    pub cgroup: *mut obj_cgroup,

    pub tag: codetag_ref,

}

// Macro flag: #define NEED_PCPUOBJ_EXT

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu_chunk {

//     pub /: *mut *mut int nr_alloc; / # of allocations,
//     pub /: *mut *mut size_t max_alloc_size; / largest allocation size,

//     pub /: *mut *mut list_head list; / linked to pcpu_slot lists,
//     pub /: *mut *mut int free_bytes; / free bytes in the chunk,
    pub chunk_md: pcpu_block_md,
//     pub /: *mut *mut *mut unsigned long bound_map; / boundary map,
//
// base_addr is the base address of this chunk.
// To reduce false sharing, current layout is optimized to make sure
// base_addr locate in the different cacheline with free_bytes and
// chunk_md.
//
    pub ____cacheline_aligned_in_smp: *mut *mut c_void base_addr,
//     pub /: *mut *mut *mut unsigned long alloc_map; / allocation map,
//     pub /: *mut *mut *mut pcpu_block_md md_blocks; / metadata blocks,
//     pub /: *mut *mut *mut c_void data; / chunk data,
//     pub /: *mut *mut bool immutable; / no [de]population allowed,
    pub chunk: *mut *mut bool isolated; / isolated from active,
    pub previous: *mut *mut int start_offset; / the overlap with the,
    pub to: *mut *mut int end_offset; / additional area required,
//     pub /: *mut *mut int nr_pages; / # of pages served by this chunk,
//     pub /: *mut *mut int nr_populated; / # of populated pages,
//     pub /: *mut *mut int nr_empty_pop_pages; / # of empty populated pages,

//     pub /: *mut *mut *mut pcpuobj_ext obj_exts; / vector of object cgroups,

//     pub /: *mut *mut unsigned long populated[]; / populated bitmap,
}

//
// pcpu_chunk_nr_blocks - converts nr_pages to # of md_blocks
// @chunk: chunk of interest
//
// This conversion is from the number of physical pages that the chunk
// serves to the number of bitmap blocks used.
//
// pcpu_nr_pages_to_map_bits - converts the pages to size of bitmap
// @pages: number of physical pages
//
// This conversion is from physical pages to the number of bits
// required in the bitmap.
//
// pcpu_chunk_map_bits - helper to convert nr_pages to size of bitmap
// @chunk: chunk of interest
//
// This conversion is from the number of physical pages that the chunk
// serves to the number of bits in the bitmap.
//
extern "C" {
    pub fn pcpu_nr_pages_to_map_bits(_arg: chunk->nr_pages) -> return;
}
//
// pcpu_obj_full_size - helper to calculate size of each accounted object
// @size: size of area to allocate in bytes
//
// For each accounted object there is an extra space which is used to store
// obj_cgroup membership if kmemcg is not disabled. Charge it too.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_stats {
//     pub /: *mut *mut u64 nr_alloc; / lifetime # of allocations,
//     pub /: *mut *mut u64 nr_dealloc; / lifetime # of deallocations,
//     pub /: *mut *mut u64 nr_cur_alloc; / current # of allocations,
//     pub /: *mut *mut u64 nr_max_alloc; / max # of live allocations,
//     pub /: *mut *mut u32 nr_chunks; / current # of live chunks,
//     pub /: *mut *mut u32 nr_max_chunks; / max # of live chunks,
//     pub /: *mut *mut size_t min_alloc_size; / min allocation size,
//     pub /: *mut *mut size_t max_alloc_size; / max allocation size,
}

//
// For debug purposes. We don't care about the flexible array.
//
// initialize min_alloc_size to unit_size
//
// pcpu_stats_area_alloc - increment area allocation stats
// @chunk: the location of the area being allocated
// @size: size of area to allocate in bytes
//
// CONTEXT:
// pcpu_lock.
//
// pcpu_stats_area_dealloc - decrement allocation stats
// @chunk: the location of the area being deallocated
//
// CONTEXT:
// pcpu_lock.
//
// pcpu_stats_chunk_alloc - increment chunk stats
//
// pcpu_stats_chunk_dealloc - decrement chunk stats
//