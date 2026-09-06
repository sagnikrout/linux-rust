//! Automatically rewritten from C Header to Rust Module
//! Source: mm/kasan/kasan.h
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

extern "C" {
    pub fn static_branch_unlikely(_arg: &kasan_flag_stacktrace) -> return;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kasan_mode {
    KASAN_MODE_SYNC,
    KASAN_MODE_ASYNC,
    KASAN_MODE_ASYMM,
}

// Static branch is never enabled with CONFIG_KASAN_VMALLOC disabled.
extern "C" {
    pub fn static_branch_likely(_arg: &kasan_flag_vmalloc) -> return;
}
// Fast-path for when sampling is disabled.

extern "C" {
    pub fn IS_ENABLED!(_arg: CONFIG_KASAN_VMALLOC) -> return;
}

//
// Generic KASAN uses per-object metadata to store alloc and free stack traces
// and the quarantine link.
//

//
// Tag-based KASAN modes do not use per-object metadata: they use the stack
// ring to store alloc and free stack traces and do not use qurantine.
//

pub const KASAN_PAGE_FREE: c_uint = 0xFF  /* freed page */;
pub const KASAN_PAGE_REDZONE: c_uint = 0xFE  /* redzone for kmalloc_large allocation */;
pub const KASAN_SLAB_REDZONE: c_uint = 0xFC  /* redzone for slab object */;
pub const KASAN_SLAB_FREE: c_uint = 0xFB  /* freed slab object */;
pub const KASAN_VMALLOC_INVALID: c_uint = 0xF8  /* inaccessible space in vmap area */;

pub const KASAN_SLAB_FREE_META: c_uint = 0xFA  /* freed slab object with free meta */;
pub const KASAN_GLOBAL_REDZONE: c_uint = 0xF9  /* redzone for global variable */;
// Stack redzone shadow values. Compiler ABI, do not change.
pub const KASAN_STACK_LEFT: c_uint = 0xF1;
pub const KASAN_STACK_MID: c_uint = 0xF2;
pub const KASAN_STACK_RIGHT: c_uint = 0xF3;
pub const KASAN_STACK_PARTIAL: c_uint = 0xF4;
// alloca redzone shadow values.
pub const KASAN_ALLOCA_LEFT: c_uint = 0xCA;
pub const KASAN_ALLOCA_RIGHT: c_uint = 0xCB;
// alloca redzone size. Compiler ABI, do not change.
pub const KASAN_ALLOCA_REDZONE_SIZE: c_int = 32;
// Stack frame marker. Compiler ABI, do not change.
pub const KASAN_CURRENT_STACK_FRAME_MAGIC: c_uint = 0x41B58AB3;
// Dummy value to avoid breaking randconfig/all*config builds.

pub const KASAN_ABI_VERSION: c_int = 1;

// Metadata layout customization.
pub const META_BYTES_PER_BLOCK: c_int = 1;
pub const META_BLOCKS_PER_ROW: c_int = 16;

pub const META_ROWS_AROUND_ADDR: c_int = 2;
pub const KASAN_STACK_DEPTH: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kasan_track {
    pub pid: u32,
    pub stack: depot_stack_handle_t,

    pub cpu:20: u64,
    pub timestamp:44: u64,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kasan_report_type {
    KASAN_REPORT_ACCESS,
    KASAN_REPORT_INVALID_FREE,
    KASAN_REPORT_DOUBLE_FREE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kasan_report_info {
// Filled in by kasan_report_*().
    pub type: kasan_report_type,
    pub access_addr: *const c_void,
    pub access_size: usize,
    pub is_write: bool,
    pub ip: c_ulong,
// Filled in by the common reporting code.
    pub first_bad_addr: *const c_void,
    pub cache: *mut kmem_cache,
    pub object: *mut c_void,
    pub alloc_size: usize,
// Filled in by the mode-specific reporting code.
    pub bug_type: *const c_char,
    pub alloc_track: kasan_track,
    pub free_track: kasan_track,
}

// Do not change the struct layout: compiler ABI.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kasan_source_location {
    pub filename: *const c_char,
    pub line_no: c_int,
    pub column_no: c_int,
}

// Do not change the struct layout: compiler ABI.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kasan_global {
//     pub /: *const *const *const c_void beg; / Address of the beginning of the global variable.,
//     pub /: *mut *mut size_t size; / Size of the global variable.,
//     pub /: *mut *mut size_t size_with_redzone; / Size of the variable + size of the redzone. 32 bytes aligned.,
    pub name: *const c_void,
//     pub /: *const *const *const c_void module_name; / Name of the module where the global variable is declared.,
//     pub /: *mut *mut unsigned long has_dynamic_init; / This is needed for C++.,

    pub location: *mut kasan_source_location,

    pub odr_indicator: *mut c_char,

}

// Structures for keeping alloc and free meta.

//
// Alloc meta contains the allocation-related information about a slab object.
// Alloc meta is saved when an object is allocated and is kept until either the
// object returns to the slab freelist (leaves quarantine for quarantined
// objects or gets freed for the non-quarantined ones) or reallocated via
// krealloc or through a mempool.
// Alloc meta is stored inside of the object's redzone.
// Alloc meta is considered valid whenever it contains non-zero data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kasan_alloc_meta {
    pub alloc_track: kasan_track,
// Free track is stored in kasan_free_meta.
    pub aux_stack: [depot_stack_handle_t; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlist_node {
    pub next: *mut qlist_node,
}

//
// Free meta is stored either in the object itself or in the redzone after the
// object. In the former case, free meta offset is 0. In the latter case, the
// offset is between 0 and INT_MAX. INT_MAX marks that free meta is not present.
//

//
// Free meta contains the freeing-related information about a slab object.
// Free meta is only kept for quarantined objects and for mempool objects until
// the object gets allocated again.
// Free meta is stored within the object's memory.
// Free meta is considered valid whenever the value of the shadow byte that
// corresponds to the first 8 bytes of the object is KASAN_SLAB_FREE_META.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kasan_free_meta {
    pub quarantine_link: qlist_node,
    pub free_track: kasan_track,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kasan_stack_ring_entry {
    pub ptr: *mut c_void,
    pub size: usize,
    pub track: kasan_track,
    pub is_free: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kasan_stack_ring {
    pub lock: rwlock_t,
    pub size: usize,
    pub pos: core::sync::atomic::AtomicI64,
    pub entries: *mut kasan_stack_ring_entry,
}

//
// kasan_check_range - Check memory region, and report if invalid access.
// @addr: the accessed address
// @size: the accessed size
// @write: true if access is a write access
// @ret_ip: return address
// @return: true if access was valid, false if invalid
//

extern "C" {
    pub fn kasan_get_alloc_size(object: *mut c_void, cache: *mut kmem_cache) -> usize;
}
extern "C" {
    pub fn kasan_complete_mode_report_info(info: *mut kasan_report_info);
}
extern "C" {
    pub fn kasan_metadata_fetch_row(buffer: *mut c_char, row: *mut c_void);
}

extern "C" {
    pub fn kasan_print_tags(addr_tag: u8, addr: *const c_void);
}

extern "C" {
    pub fn kasan_print_address_stack_frame(addr: *const c_void);
}

extern "C" {
    pub fn kasan_print_aux_stacks(cache: *mut kmem_cache, object: *const c_void);
}

extern "C" {
    pub fn kasan_report_invalid_free(object: *mut c_void, ip: c_ulong, type: kasan_report_type);
}

extern "C" {
    pub fn kasan_init_object_meta(cache: *mut kmem_cache, object: *const c_void);
}

extern "C" {
    pub fn kasan_save_stack(flags: gfp_t, depot_flags: depot_flags_t) -> depot_stack_handle_t;
}
extern "C" {
    pub fn kasan_set_track(track: *mut kasan_track, stack: depot_stack_handle_t);
}
extern "C" {
    pub fn kasan_save_track(track: *mut kasan_track, flags: gfp_t);
}
extern "C" {
    pub fn kasan_save_alloc_info(cache: *mut kmem_cache, object: *mut c_void, flags: gfp_t);
}
extern "C" {
    pub fn kasan_save_free_info(cache: *mut kmem_cache, object: *mut c_void);
}

extern "C" {
    pub fn kasan_quarantine_put(cache: *mut kmem_cache, object: *mut c_void) -> bool;
}
extern "C" {
    pub fn kasan_quarantine_reduce();
}
extern "C" {
    pub fn kasan_quarantine_remove_cache(cache: *mut kmem_cache);
}

pub const arch_kasan_get_tag(addr): c_int = 0;

extern "C" {
    pub fn kasan_enable_hw_tags();
}

extern "C" {
    pub fn kasan_init_tags() ;
}

extern "C" {
    pub fn kasan_force_async_fault();
}
extern "C" {
    pub fn kasan_write_only_enabled() -> bool;
}

extern "C" {
    pub fn kasan_random_tag() -> u8;
}

//
// kasan_poison - mark the memory range as inaccessible
// @addr: range start address, must be aligned to KASAN_GRANULE_SIZE
// @size: range size, must be aligned to KASAN_GRANULE_SIZE
// @value: value that's written to metadata for the range
// @init: whether to initialize the memory range (only for hardware tag-based)
//
extern "C" {
    pub fn kasan_poison(addr: *const c_void, size: usize, value: u8, init: bool);
}
//
// kasan_unpoison - mark the memory range as accessible
// @addr: range start address, must be aligned to KASAN_GRANULE_SIZE
// @size: range size, can be unaligned
// @init: whether to initialize the memory range (only for hardware tag-based)
//
// For the tag-based modes, the @size gets aligned to KASAN_GRANULE_SIZE before
// marking the range.
// For the generic mode, the last granule of the memory range gets partially
// unpoisoned based on the @size.
//
extern "C" {
    pub fn kasan_unpoison(addr: *const c_void, size: usize, init: bool);
}
extern "C" {
    pub fn kasan_byte_accessible(addr: *const c_void) -> bool;
}

//
// kasan_poison_last_granule - mark the last granule of the memory range as
// inaccessible
// @address: range start address, must be aligned to KASAN_GRANULE_SIZE
// @size: range size
//
// This function is only available for the generic mode, as it's the only mode
// that has partially poisoned memory granules.
//
extern "C" {
    pub fn kasan_poison_last_granule(address: *const c_void, size: usize);
}

extern "C" {
    pub fn kasan_kunit_test_suite_start();
}
extern "C" {
    pub fn kasan_kunit_test_suite_end();
}

extern "C" {
    pub fn kasan_test_rust_uaf() -> c_char;
}

extern "C" {
    pub fn kasan_save_enable_multi_shot() -> bool;
}
extern "C" {
    pub fn kasan_restore_multi_shot(enabled: bool);
}

//
// Exported functions for interfaces called from assembly or from generated
// code. Declared here to avoid warnings about missing declarations.
//
extern "C" {
    pub fn __asan_register_globals(globals: *mut c_void, size: isize);
}
extern "C" {
    pub fn __asan_unregister_globals(globals: *mut c_void, size: isize);
}
extern "C" {
    pub fn __asan_handle_no_return();
}
extern "C" {
    pub fn __asan_alloca_poison(: *mut c_void, size: isize);
}
extern "C" {
    pub fn __asan_allocas_unpoison(stack_top: *mut c_void, stack_bottom: isize);
}
extern "C" {
    pub fn __asan_load1(: *mut c_void);
}
extern "C" {
    pub fn __asan_store1(: *mut c_void);
}
extern "C" {
    pub fn __asan_load2(: *mut c_void);
}
extern "C" {
    pub fn __asan_store2(: *mut c_void);
}
extern "C" {
    pub fn __asan_load4(: *mut c_void);
}
extern "C" {
    pub fn __asan_store4(: *mut c_void);
}
extern "C" {
    pub fn __asan_load8(: *mut c_void);
}
extern "C" {
    pub fn __asan_store8(: *mut c_void);
}
extern "C" {
    pub fn __asan_load16(: *mut c_void);
}
extern "C" {
    pub fn __asan_store16(: *mut c_void);
}
extern "C" {
    pub fn __asan_loadN(: *mut c_void, size: isize);
}
extern "C" {
    pub fn __asan_storeN(: *mut c_void, size: isize);
}
extern "C" {
    pub fn __asan_load1_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_store1_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_load2_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_store2_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_load4_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_store4_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_load8_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_store8_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_load16_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_store16_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_loadN_noabort(: *mut c_void, size: isize);
}
extern "C" {
    pub fn __asan_storeN_noabort(: *mut c_void, size: isize);
}
extern "C" {
    pub fn __asan_report_load1_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_report_store1_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_report_load2_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_report_store2_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_report_load4_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_report_store4_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_report_load8_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_report_store8_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_report_load16_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_report_store16_noabort(: *mut c_void);
}
extern "C" {
    pub fn __asan_report_load_n_noabort(: *mut c_void, size: isize);
}
extern "C" {
    pub fn __asan_report_store_n_noabort(: *mut c_void, size: isize);
}
extern "C" {
    pub fn __asan_set_shadow_00(addr: *const c_void, size: isize);
}
extern "C" {
    pub fn __asan_set_shadow_f1(addr: *const c_void, size: isize);
}
extern "C" {
    pub fn __asan_set_shadow_f2(addr: *const c_void, size: isize);
}
extern "C" {
    pub fn __asan_set_shadow_f3(addr: *const c_void, size: isize);
}
extern "C" {
    pub fn __asan_set_shadow_f5(addr: *const c_void, size: isize);
}
extern "C" {
    pub fn __asan_set_shadow_f8(addr: *const c_void, size: isize);
}
extern "C" {
    pub fn __hwasan_load1_noabort(: *mut c_void);
}
extern "C" {
    pub fn __hwasan_store1_noabort(: *mut c_void);
}
extern "C" {
    pub fn __hwasan_load2_noabort(: *mut c_void);
}
extern "C" {
    pub fn __hwasan_store2_noabort(: *mut c_void);
}
extern "C" {
    pub fn __hwasan_load4_noabort(: *mut c_void);
}
extern "C" {
    pub fn __hwasan_store4_noabort(: *mut c_void);
}
extern "C" {
    pub fn __hwasan_load8_noabort(: *mut c_void);
}
extern "C" {
    pub fn __hwasan_store8_noabort(: *mut c_void);
}
extern "C" {
    pub fn __hwasan_load16_noabort(: *mut c_void);
}
extern "C" {
    pub fn __hwasan_store16_noabort(: *mut c_void);
}
extern "C" {
    pub fn __hwasan_loadN_noabort(: *mut c_void, size: isize);
}
extern "C" {
    pub fn __hwasan_storeN_noabort(: *mut c_void, size: isize);
}
extern "C" {
    pub fn __hwasan_tag_memory(: *mut c_void, tag: u8, size: isize);
}