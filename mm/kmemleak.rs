//! Automatically rewritten from C to Rust
//! Source: mm/kmemleak.c
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
// mm/kmemleak.c
//
// Copyright (C) 2008 ARM Limited
// Written by Catalin Marinas <catalin.marinas@arm.com>
//
// For more information on the algorithm and kmemleak usage, please see
// Documentation/dev-tools/kmemleak.rst.
//
// Notes on locking
// ----------------
//
// The following locks and mutexes are used by kmemleak:
//
// - kmemleak_lock (raw_spinlock_t): protects the object_list as well as
// del_state modifications and accesses to the object trees
// (object_tree_root, object_phys_tree_root, object_percpu_tree_root). The
// object_list is the main list holding the metadata (struct
// kmemleak_object) for the allocated memory blocks. The object trees are
// red black trees used to look-up metadata based on a pointer to the
// corresponding memory block. The kmemleak_object structures are added to
// the object_list and the object tree root in the create_object() function
// called from the kmemleak_alloc{,_phys,_percpu}() callback and removed in
// delete_object() called from the kmemleak_free{,_phys,_percpu}() callback
// - kmemleak_object.lock (raw_spinlock_t): protects a kmemleak_object.
// Accesses to the metadata (e.g. count) are protected by this lock. Note
// that some members of this structure may be protected by other means
// (atomic or kmemleak_lock). This lock is also held when scanning the
// corresponding memory block to avoid the kernel freeing it via the
// kmemleak_free() callback. This is less heavyweight than holding a global
// lock like kmemleak_lock during scanning.
// - scan_mutex (mutex): ensures that only one thread may scan the memory for
// unreferenced objects at a time. The gray_list contains the objects which
// are already referenced or marked as false positives and need to be
// scanned. This list is only modified during a scanning episode when the
// scan_mutex is held. At the end of a scan, the gray_list is always empty.
// Note that the kmemleak_object.use_count is incremented when an object is
// added to the gray_list and therefore cannot be freed. This mutex also
// prevents multiple users of the "kmemleak" debugfs file together with
// modifications to the memory scanning parameters including the scan_thread
// pointer
//
// Locks and mutexes are acquired/nested in the following order:
//
// scan_mutex [-> object->lock] -> kmemleak_lock -> other_object->lock (SINGLE_DEPTH_NESTING)
//
// No kmemleak_lock and object->lock nesting is allowed outside scan_mutex
// regions.
//
// The kmemleak_object structures have a use_count incremented or decremented
// using the get_object()/put_object() functions. When the use_count becomes
// 0, this count can no longer be incremented and put_object() schedules the
// kmemleak_object freeing via an RCU callback. All calls to the get_object()
// function must be protected by rcu_read_lock() to avoid accessing a freed
// structure.
//

//
// Kmemleak configuration and common defines.
//

// scanning area inside a memory block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmemleak_scan_area {
    pub node: hlist_node,
    pub start: c_ulong,
    pub size: usize,
}

pub const KMEMLEAK_GREY: c_int = 0;

//
// Structure holding the metadata for each allocated memory block.
// Modifications to such objects should be made while holding the
// object->lock. Insertions or deletions from object_list, gray_list or
// rb_node are already protected by the corresponding locks or mutex (see
// the notes on locking above). These objects are reference-counted
// (use_count) and freed using the RCU mechanism.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmemleak_object {
    pub lock: raw_spinlock_t,
//     pub /: *mut *mut unsigned int flags; / object status flags,
    pub object_list: list_head,
    pub gray_list: list_head,
    pub rb_node: rb_node,
//     pub /: *mut *mut rcu_head rcu; / object_list lockless traversal,
// object usage count; object freed when use_count == 0
    pub use_count: core::sync::atomic::AtomicI32,
//     pub /: *mut *mut unsigned int del_state; / deletion state,
    pub pointer: c_ulong,
    pub size: usize,
// pass surplus references to this pointer
    pub excess_ref: c_ulong,
// minimum number of a pointers found before it is considered leak
    pub min_count: c_int,
// the total number of pointers found pointing to this object
    pub count: c_int,
// consecutive scans the object has been seen unreferenced
    pub unref_scans: c_uint,
// checksum for detecting modified objects
    pub checksum: u32,
    pub trace_handle: depot_stack_handle_t,
memory ranges to be scanned inside an object (empty for all)
    pub area_list: hlist_head,
//     pub /: *mut *mut unsigned long jiffies; / creation timestamp,
//     pub /: *mut *mut pid_t pid; / pid of the current task,
// per-scan dedup count, valid only while in scan-local dedup xarray
    pub dup_count: c_uint,
//     pub /: *mut *mut char comm[TASK_COMM_LEN]; / executable name,
}

// flag representing the memory block allocation status

// flag set after the first reporting of an unreference object

// flag set to not scan the object

// flag set to fully scan the object when scan_area allocation failed

// flag set for object allocated with physical address

// flag set for per-CPU pointers

// flag set on an object left unreferenced by the full scan, pending confirmation

// set when __remove_object() called

// set to temporarily prevent deletion from object_list

// number of bytes to print per line; must be 16 or 32
pub const HEX_ROW_SIZE: c_int = 16;
// number of bytes to print at a time (1, 2, 4, 8)
pub const HEX_GROUP_SIZE: c_int = 1;
// include ASCII after the hex output
pub const HEX_ASCII: c_int = 1;
// max number of lines to be printed
pub const HEX_MAX_LINES: c_int = 2;
// the list of all allocated objects
pub static mut object_list: usize = 0;
// the list of gray-colored objects (see color_gray comment below)
pub static mut gray_list: usize = 0;
// memory pool allocation
    static struct kmemleak_object mem_pool[CONFIG_DEBUG_KMEMLEAK_MEM_POOL_SIZE];
pub static mut mem_pool_free_count: int = 0;
pub static mut mem_pool_free_list: usize = 0;
// search tree for object boundaries
pub static mut object_tree_root: rb_root = 0;
// search tree for object (with OBJECT_PHYS flag) boundaries
pub static mut object_phys_tree_root: rb_root = 0;
// search tree for object (with OBJECT_PERCPU flag) boundaries
pub static mut object_percpu_tree_root: rb_root = 0;
// protecting the access to object_list, object_tree_root (or object_phys_tree_root)
pub static mut kmemleak_lock: usize = 0;
// allocation caches for kmemleak internal data
pub static mut object_cache: *mut c_void = core::ptr::null_mut();
pub static mut scan_area_cache: *mut c_void = core::ptr::null_mut();
// set if tracing memory operations is enabled
pub static mut : int kmemleak_enabled = 1;
// same as above but only for the kmemleak_free() callback
pub static mut : int kmemleak_free_enabled = 1;
// set in the late_initcall if there were no errors
    static int kmemleak_late_initialized;
// set if a fatal kmemleak error has occurred
    static int kmemleak_error;
// minimum and maximum address that may be valid pointers
pub static mut min_addr: unsigned long = 0;
    static unsigned long max_addr;
// minimum and maximum address that may be valid per-CPU pointers
pub static mut min_percpu_addr: unsigned long = 0;
    static unsigned long max_percpu_addr;
pub static mut scan_thread: *mut c_void = core::ptr::null_mut();
// used to avoid reporting of recently allocated objects
    static unsigned long jiffies_min_age;
// consecutive scans an object must stay unreferenced before reporting
    static unsigned int min_unref_scans =
    IS_ENABLED!(CONFIG_DEBUG_KMEMLEAK_VERBOSE) ? 2 : 1;
    module_param!(min_unref_scans, uint, 0644);
    static unsigned long jiffies_last_scan;
// delay between automatic memory scannings
    static unsigned long jiffies_scan_wait;
// number of objects flagged OBJECT_SUSPECT during the current scan
    static int nr_suspects;
// enables or disables the task stacks scanning
pub static mut kmemleak_stack_scan: int = 1;
// protects the memory scanning, parameters and debug/kmemleak file access
pub static mut scan_mutex: usize = 0;
// setting kmemleak=on, will set this var, skipping the disable
    static int kmemleak_skip_disable;
// If there are leaks that can be reported
    static bool kmemleak_found_leaks;
pub static mut kmemleak_verbose: bool = false;
    module_param_named!(verbose, kmemleak_verbose, bool, 0600);
// forward_decl: kmemleak_disable;
//
// Print a warning and dump the stack trace.
//

    pr_warn!(x);				
    dump_stack();				
    } while (0)
//
// Macro invoked when a serious kmemleak condition occurred and cannot be
// recovered from. Kmemleak will be disabled and further allocation/freeing
// tracing no longer available.
//

    kmemleak_warn(x);		
    kmemleak_disable();		
    } while (0)

    if (seq)					 {
    seq_printf(seq, fmt, ##__VA_ARGS__);	
    }
    else {
    pr_warn!(fmt, ##__VA_ARGS__);		
    }
    } while (0)
#[no_mangle]
pub unsafe extern "C" fn warn_or_seq_hex_dump(seq: *mut seq_file, prefix_type: c_int, rowsize: c_int, groupsize: c_int, buf: *mut c_void, len: size_t, ascii: bool) {
    if (seq) {
    seq_hex_dump(seq, HEX_PREFIX, prefix_type, rowsize, groupsize,
    buf, len, ascii);
    }
    else {
    print_hex_dump(KERN_WARNING, pr_fmt(HEX_PREFIX), prefix_type,
    rowsize, groupsize, buf, len, ascii);
    }
    }
//
// Printing of the objects hex dump to the seq file. The number of lines to be
// printed is limited to HEX_MAX_LINES to prevent seq file spamming. The
// actual number of printed bytes depends on HEX_ROW_SIZE. It must be called
// with the object->lock held.
//
#[no_mangle]
pub unsafe extern "C" fn hex_dump_object(seq: *mut seq_file, object: *mut kmemleak_object) {
    let mut ptr = object.pointer;
    let mut len = 0;
    if (WARN_ON_ONCE!(object.flags & OBJECT_PHYS)) {
    return;
    }
    if (object.flags & OBJECT_PERCPU) {
    ptr = this_cpu_ptr(object.pointer);
    }
// limit the number of lines to HEX_MAX_LINES
    len = min_t(size_t, object.size, HEX_MAX_LINES * HEX_ROW_SIZE);
    if (object.flags & OBJECT_PERCPU) {
    warn_or_seq_printf(seq, "  hex dump (first %zu bytes on cpu %d):\n",
    len, raw_smp_processor_id());
    }
    else {
    warn_or_seq_printf(seq, "  hex dump (first %zu bytes):\n", len);
    }
    kasan_disable_current();
    warn_or_seq_hex_dump(seq, DUMP_PREFIX_NONE, HEX_ROW_SIZE,
    HEX_GROUP_SIZE, kasan_reset_tag(ptr), len, HEX_ASCII);
    kasan_enable_current();
    }
//
// Object colors, encoded with count and min_count:
// - white - orphan object, not enough references to it (count < min_count)
// - gray  - not orphan, not marked as false positive (min_count == 0) or
// sufficient references to it (count >= min_count)
// - black - ignore, it doesn't contain references (e.g. text section)
// (min_count == -1). No function defined for this color.
//
#[no_mangle]
unsafe extern "C" fn color_white(object: *const kmemleak_object) -> bool {
    return object.count != KMEMLEAK_BLACK &&
    object.count < object.min_count;
    }
#[no_mangle]
unsafe extern "C" fn color_gray(object: *const kmemleak_object) -> bool {
    return object.min_count != KMEMLEAK_BLACK &&
    object.count >= object.min_count;
    }
//
// Objects are considered unreferenced only if their color is white, they have
// not be deleted and have a minimum age to avoid false positives caused by
// pointers temporarily stored in CPU registers.
//
#[no_mangle]
unsafe extern "C" fn unreferenced_object(object: *mut kmemleak_object) -> bool {
    return (color_white(object) && object.flags & OBJECT_ALLOCATED) &&
    time_before_eq(object.jiffies + jiffies_min_age,
    jiffies_last_scan);
    }
    static const char *__object_type_str(kmemleak_object *object)
    {
    if (object.flags & OBJECT_PHYS) {
    return " (phys)";
    }
    if (object.flags & OBJECT_PERCPU) {
    return " (percpu)";
    }
    return "";
    }
//
// Printing of the unreferenced objects information to the seq file. The
// print_unreferenced function must be called with the object->lock held.
//
#[no_mangle]
pub unsafe extern "C" fn __print_unreferenced(seq: *mut seq_file, object: *mut kmemleak_object, hex_dump: bool) {
    let mut i = 0;
pub static mut entries: *mut c_void = core::ptr::null_mut();
    let mut nr_entries = 0;
    nr_entries = stack_depot_fetch(object.trace_handle, &entries);
    warn_or_seq_printf(seq, "unreferenced object%s 0x%08lx (size %zu):\n",
    __object_type_str(object),
    object.pointer, object.size);
    warn_or_seq_printf(seq, "  comm \"%s\", pid %d, jiffies %lu\n",
    object.comm, object.pid, object.jiffies);
    if (hex_dump) {
    hex_dump_object(seq, object);
    }
    warn_or_seq_printf(seq, "  backtrace (crc %x):\n", object.checksum);
    while (i < nr_entries) {
    let mut ptr = entries[i];
    warn_or_seq_printf(seq, "    %pS\n", ptr);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn print_unreferenced(seq: *mut seq_file, object: *mut kmemleak_object) {
    __print_unreferenced(seq, object, true);
    }
//
// Print the kmemleak_object information. This function is used mainly for
// debugging special cases when kmemleak operations. It must be called with
// the object->lock held.
//
#[no_mangle]
unsafe extern "C" fn dump_object_info(object: *mut kmemleak_object) {
    pr_notice("Object%s 0x%08lx (size %zu):\n",
    __object_type_str(object), object.pointer, object.size);
    pr_notice("  comm \"%s\", pid %d, jiffies %lu\n",
    object.comm, object.pid, object.jiffies);
    pr_notice("  min_count = %d\n", object.min_count);
    pr_notice("  count = %d\n", object.count);
    pr_notice("  flags = 0x%x\n", object.flags);
    pr_notice("  checksum = %u\n", object.checksum);
    pr_notice("  backtrace:\n");
    if (object.trace_handle) {
    stack_depot_print(object.trace_handle);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn object_tree(objflags: c_ulong) -> *mut c_void {
    if (objflags & OBJECT_PHYS) {
    return &object_phys_tree_root;
    }
    if (objflags & OBJECT_PERCPU) {
    return &object_percpu_tree_root;
    }
    return &object_tree_root;
    }
//
// Look-up a memory block metadata (kmemleak_object) in the object search
// tree based on a pointer value. If alias is 0, only values pointing to the
// beginning of the memory block are allowed. The kmemleak_lock must be held
// when calling this function.
//
#[no_mangle]
pub unsafe extern "C" fn __lookup_object(ptr: c_ulong, alias: c_int, objflags: c_uint) -> *mut c_void {
    let mut rb = object_tree(objflags).rb_node;
pub static mut untagged_ptr: c_ulong = 0;
    while (rb) {
pub static mut object: *mut c_void = core::ptr::null_mut();
    let mut untagged_objp = 0;
    object = rb_entry(rb, kmemleak_object, rb_node);
    untagged_objp = (unsigned long)kasan_reset_tag(object.pointer);
    if (untagged_ptr < untagged_objp) {
    rb = object.rb_node.rb_left;
    }

    else if (untagged_objp + object.size <= untagged_ptr) {
    rb = object.rb_node.rb_right;
    }

    else if (untagged_objp == untagged_ptr || alias) {
    return object;
    }
    else {
//
// Printk deferring due to the kmemleak_lock held.
// This is done to avoid deadlock.
//
    printk_deferred_enter();
    kmemleak_warn("Found object by alias at 0x%08lx\n",
    ptr);
    dump_object_info(object);
    printk_deferred_exit();
    break;
    }
    }
    return core::ptr::null_mut();
    }
// Look-up a kmemleak object which allocated with virtual address.
#[no_mangle]
pub unsafe extern "C" fn lookup_object(ptr: c_ulong, alias: c_int) -> *mut c_void {
    return __lookup_object(ptr, alias, 0);
    }
//
// Increment the object use_count. Return 1 if successful or 0 otherwise. Note
// that once an object's use_count reached 0, the RCU freeing was already
// registered and the object should no longer be used. This function must be
// called under the protection of rcu_read_lock().
//
#[no_mangle]
unsafe extern "C" fn get_object(object: *mut kmemleak_object) -> c_int {
    return atomic_inc_not_zero(&object.use_count);
    }
//
// Memory pool allocation and freeing. kmemleak_lock must not be held.
//
#[no_mangle]
pub unsafe extern "C" fn mem_pool_alloc(gfp: gfp_t) -> *mut c_void {
    let mut flags = 0;
pub static mut object: *mut c_void = core::ptr::null_mut();
pub static mut warn: bool = false;
// try the slab allocator first
    if (object_cache) {
    object = kmem_cache_alloc_noprof(object_cache,
    gfp_nested_mask(gfp));
    if (object) {
    return object;
    }
    }
// slab allocation failed, try the memory pool
    raw_spin_lock_irqsave(&kmemleak_lock, flags);
    object = list_first_entry_or_null(&mem_pool_free_list,
    typeof(*object), object_list);
    if (object) {
    list_del(&object.object_list);
    }

    else if (mem_pool_free_count) {
    object = &mem_pool[--mem_pool_free_count];
    }
    else {
    warn = true;
    }
    raw_spin_unlock_irqrestore(&kmemleak_lock, flags);
    if (warn) {
    pr_warn_once("Memory pool empty, consider increasing CONFIG_DEBUG_KMEMLEAK_MEM_POOL_SIZE\n");
    }
    return object;
    }
//
// Return the object to either the slab allocator or the memory pool.
//
#[no_mangle]
unsafe extern "C" fn mem_pool_free(object: *mut kmemleak_object) {
    let mut flags = 0;
    if (object < mem_pool || object >= ARRAY_END(mem_pool)) {
    kmem_cache_free(object_cache, object);
    return;
    }
// add the object to the memory pool free list
    raw_spin_lock_irqsave(&kmemleak_lock, flags);
    list_add(&object.object_list, &mem_pool_free_list);
    raw_spin_unlock_irqrestore(&kmemleak_lock, flags);
    }
//
// RCU callback to free a kmemleak_object.
//
#[no_mangle]
unsafe extern "C" fn free_object_rcu(rcu: *mut rcu_head) {
pub static mut tmp: *mut c_void = core::ptr::null_mut();
pub static mut area: *mut c_void = core::ptr::null_mut();
    let mut object = container_of!(rcu, kmemleak_object, rcu);
//
// Once use_count is 0 (guaranteed by put_object), there is no other
// code accessing this object, hence no need for locking.
//
    hlist_for_each_entry_safe(area, tmp, &object.area_list, node) {
    hlist_del(&area.node);
    kmem_cache_free(scan_area_cache, area);
    }
    mem_pool_free(object);
    }
//
// Decrement the object use_count. Once the count is 0, free the object using
// an RCU callback. Since put_object() may be called via the kmemleak_free() ->
// delete_object() path, the delayed RCU freeing ensures that there is no
// recursive call to the kernel allocator. Lock-less RCU object_list traversal
// is also possible.
//
#[no_mangle]
unsafe extern "C" fn put_object(object: *mut kmemleak_object) {
    if (!atomic_dec_and_test(&object.use_count)) {
    return;
    }
// should only get here after delete_object was called
    WARN_ON!(object.flags & OBJECT_ALLOCATED);
//
// It may be too early for the RCU callbacks, however, there is no
// concurrent object_list traversal when !object_cache and all objects
// came from the memory pool. Free the object directly.
//
    if (object_cache) {
    call_rcu(&object.rcu, free_object_rcu);
    }
    else {
    free_object_rcu(&object.rcu);
    }
    }
//
// Look up an object in the object search tree and increase its use_count.
//
#[no_mangle]
pub unsafe extern "C" fn __find_and_get_object(ptr: c_ulong, alias: c_int, objflags: c_uint) -> *mut c_void {
    let mut flags = 0;
pub static mut object: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    raw_spin_lock_irqsave(&kmemleak_lock, flags);
    object = __lookup_object(ptr, alias, objflags);
    raw_spin_unlock_irqrestore(&kmemleak_lock, flags);
// check whether the object is still available
    if (object && !get_object(object)) {
    object = core::ptr::null_mut();
    }
    rcu_read_unlock();
    return object;
    }
// Look up and get an object which allocated with virtual address.
#[no_mangle]
pub unsafe extern "C" fn find_and_get_object(ptr: c_ulong, alias: c_int) -> *mut c_void {
    return __find_and_get_object(ptr, alias, 0);
    }
//
// Remove an object from its object tree and object_list. Must be called with
// the kmemleak_lock held _if_ kmemleak is still enabled.
//
#[no_mangle]
unsafe extern "C" fn __remove_object(object: *mut kmemleak_object) {
    rb_erase(&object.rb_node, object_tree(object.flags));
    if (!(object.del_state & DELSTATE_NO_DELETE)) {
    list_del_rcu(&object.object_list);
    }
    object.del_state |= DELSTATE_REMOVED;
    }
#[no_mangle]
pub unsafe extern "C" fn __find_and_remove_object(ptr: c_ulong, alias: c_int, objflags: c_uint) -> *mut c_void {
pub static mut object: *mut c_void = core::ptr::null_mut();
    object = __lookup_object(ptr, alias, objflags);
    if (object) {
    __remove_object(object);
    }
    return object;
    }
//
// Look up an object in the object search tree and remove it from both object
// tree root and object_list. The returned object's use_count should be at
// least 1, as initially set by create_object().
//
#[no_mangle]
pub unsafe extern "C" fn find_and_remove_object(ptr: c_ulong, alias: c_int, objflags: c_uint) -> *mut c_void {
    let mut flags = 0;
pub static mut object: *mut c_void = core::ptr::null_mut();
    raw_spin_lock_irqsave(&kmemleak_lock, flags);
    object = __find_and_remove_object(ptr, alias, objflags);
    raw_spin_unlock_irqrestore(&kmemleak_lock, flags);
    return object;
    }
#[no_mangle]
unsafe extern "C" fn set_track_prepare() -> noinline depot_stack_handle_t {
    let mut trace_handle;
    unsigned long entries[MAX_TRACE];
    let mut nr_entries = 0;
//
// Use object_cache to determine whether kmemleak_init() has
// been invoked. stack_depot_early_init() is called before
// kmemleak_init() in mm_core_init().
//
    if (!object_cache) {
    return 0;
    }
    nr_entries = stack_trace_save(entries, ARRAY_SIZE!(entries), 3);
    trace_handle = stack_depot_save(entries, nr_entries, GFP_NOWAIT);
    return trace_handle;
    }
#[no_mangle]
pub unsafe extern "C" fn __alloc_object(gfp: gfp_t) -> *mut c_void {
pub static mut object: *mut c_void = core::ptr::null_mut();
    object = mem_pool_alloc(gfp);
    if (!object) {
    pr_warn!("Cannot allocate a kmemleak_object structure\n");
    kmemleak_disable();
    return core::ptr::null_mut();
    }
    INIT_LIST_HEAD(&object.object_list);
    INIT_LIST_HEAD(&object.gray_list);
    INIT_HLIST_HEAD(&object.area_list);
    raw_spin_lock_init(&object.lock);
    atomic_set(&object.use_count, 1);
    object.excess_ref = 0;
    object.count = 0;			/* white color initially */
    object.checksum = ~0;
    object.unref_scans = 0;
    object.del_state = 0;
// task information
    if (in_hardirq()) {
    object.pid = 0;
    strscpy(object.comm, "hardirq");
    } else if (in_serving_softirq()) {
    object.pid = 0;
    strscpy(object.comm, "softirq");
    } else {
    object.pid = current.pid;
//
// There is a small chance of a race with set_task_comm(),
// however using get_task_comm() here may cause locking
// dependency issues with current->alloc_lock. In the worst
// case, the command line is not correct.
//
    strscpy(object.comm, current.comm);
    }
// kernel backtrace
    object.trace_handle = set_track_prepare();
    return object;
    }
#[no_mangle]
pub unsafe extern "C" fn __link_object(object: *mut kmemleak_object, ptr: c_ulong, size: size_t, min_count: c_int, objflags: c_uint) -> c_int {
pub static mut parent: *mut c_void = core::ptr::null_mut();
    let mut link = core::ptr::null_mut();
    let mut rb_parent = core::ptr::null_mut();
    let mut untagged_ptr = 0;
    let mut untagged_objp = 0;
    object.flags = OBJECT_ALLOCATED | objflags;
    object.pointer = ptr;
    object.size = kfence_ksize(ptr) ?: size;
    object.min_count = min_count;
    object.jiffies = jiffies;
    untagged_ptr = (unsigned long)kasan_reset_tag(ptr);
//
// Only update min_addr and max_addr with object storing virtual
// address. And update min_percpu_addr max_percpu_addr for per-CPU
// objects.
//
    if (objflags & OBJECT_PERCPU) {
    min_percpu_addr = min(min_percpu_addr, untagged_ptr);
    max_percpu_addr = max(max_percpu_addr, untagged_ptr + size);
    } else if (!(objflags & OBJECT_PHYS)) {
    min_addr = min(min_addr, untagged_ptr);
    max_addr = max(max_addr, untagged_ptr + size);
    }
    link = &object_tree(objflags).rb_node;
    rb_parent = core::ptr::null_mut();
    while (*link) {
    rb_parent = *link;
    parent = rb_entry(rb_parent, kmemleak_object, rb_node);
    untagged_objp = (unsigned long)kasan_reset_tag(parent.pointer);
    if (untagged_ptr + size <= untagged_objp) {
    link = &parent.rb_node.rb_left;
    }

    else if (untagged_objp + parent.size <= untagged_ptr) {
    link = &parent.rb_node.rb_right;
    }
    else {
//
// Printk deferring due to the kmemleak_lock held.
// This is done to avoid deadlock.
//
    printk_deferred_enter();
    kmemleak_stop("Cannot insert 0x%lx into the object search tree (overlaps existing)\n",
    ptr);
//
// No need for parent->lock here since "parent" cannot
// be freed while the kmemleak_lock is held.
//
    dump_object_info(parent);
    printk_deferred_exit();
    return -EEXIST;
    }
    }
    rb_link_node(&object.rb_node, rb_parent, link);
    rb_insert_color(&object.rb_node, object_tree(objflags));
    list_add_tail_rcu(&object.object_list, &object_list);
    return 0;
    }
//
// Create the metadata (kmemleak_object) corresponding to an allocated
// memory block and add it to the object_list and object tree.
//
#[no_mangle]
pub unsafe extern "C" fn __create_object(ptr: c_ulong, size: size_t, min_count: c_int, gfp: gfp_t, objflags: c_uint) {
pub static mut object: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut ret = 0;
    object = __alloc_object(gfp);
    if (!object) {
    return;
    }
    raw_spin_lock_irqsave(&kmemleak_lock, flags);
    ret = __link_object(object, ptr, size, min_count, objflags);
    raw_spin_unlock_irqrestore(&kmemleak_lock, flags);
    if (ret) {
    mem_pool_free(object);
    }
    }
// Create kmemleak object which allocated with virtual address.
#[no_mangle]
pub unsafe extern "C" fn create_object(ptr: c_ulong, size: size_t, min_count: c_int, gfp: gfp_t) {
    __create_object(ptr, size, min_count, gfp, 0);
    }
// Create kmemleak object which allocated with physical address.
#[no_mangle]
pub unsafe extern "C" fn create_object_phys(ptr: c_ulong, size: size_t, min_count: c_int, gfp: gfp_t) {
    __create_object(ptr, size, min_count, gfp, OBJECT_PHYS);
    }
// Create kmemleak object corresponding to a per-CPU allocation.
#[no_mangle]
pub unsafe extern "C" fn create_object_percpu(ptr: c_ulong, size: size_t, min_count: c_int, gfp: gfp_t) {
    __create_object(ptr, size, min_count, gfp, OBJECT_PERCPU);
    }
//
// Mark the object as not allocated and schedule RCU freeing via put_object().
//
#[no_mangle]
unsafe extern "C" fn __delete_object(object: *mut kmemleak_object) {
    let mut flags = 0;
    WARN_ON!(!(object.flags & OBJECT_ALLOCATED));
    WARN_ON!(atomic_read(&object.use_count) < 1);
//
// Locking here also ensures that the corresponding memory block
// cannot be freed when it is being scanned.
//
    raw_spin_lock_irqsave(&object.lock, flags);
    object.flags &= ~OBJECT_ALLOCATED;
    raw_spin_unlock_irqrestore(&object.lock, flags);
    put_object(object);
    }
//
// Look up the metadata (kmemleak_object) corresponding to ptr and
// delete it.
//
#[no_mangle]
unsafe extern "C" fn delete_object_full(ptr: c_ulong, objflags: c_uint) {
pub static mut object: *mut c_void = core::ptr::null_mut();
    object = find_and_remove_object(ptr, 0, objflags);
    if (!object) {
//
// kmalloc_nolock() -> kfree() calls kmemleak_free()
// without kmemleak_alloc().
//
    return;
    }
    __delete_object(object);
    }
//
// Look up the metadata (kmemleak_object) corresponding to ptr and
// delete it. If the memory block is partially freed, the function may create
// additional metadata for the remaining parts of the block.
//
#[no_mangle]
pub unsafe extern "C" fn delete_object_part(ptr: c_ulong, size: size_t, objflags: c_uint) {
    let mut object = core::ptr::null_mut();
    let mut object_l = core::ptr::null_mut();
    let mut object_r = core::ptr::null_mut();
    unsigned long start, end, flags;
    object_l = __alloc_object(GFP_KERNEL);
    if (!object_l) {
    return;
    }
    object_r = __alloc_object(GFP_KERNEL);
    if (!object_r) {
// goto;
    }
    raw_spin_lock_irqsave(&kmemleak_lock, flags);
    object = __find_and_remove_object(ptr, 1, objflags);
    if (!object) {
// goto;
    }
//
// Create one or two objects that may result from the memory block
// split. Note that partial freeing is only done by free_bootmem() and
// this happens before kmemleak_init() is called.
//
    start = object.pointer;
    end = object.pointer + object.size;
    if ((ptr > start) &&
    !__link_object(object_l, start, ptr - start,
    object.min_count, objflags)) {
    object_l = core::ptr::null_mut();
    }
    if ((ptr + size < end) &&
    !__link_object(object_r, ptr + size, end - ptr - size,
    object.min_count, objflags)) {
    object_r = core::ptr::null_mut();
    }
// label;
    raw_spin_unlock_irqrestore(&kmemleak_lock, flags);
    if (object) {
    __delete_object(object);
    } else {

    kmemleak_warn("Partially freeing unknown object at 0x%08lx (size %zu)\n",
    ptr, size);

    }
// label;
    if (object_l) {
    mem_pool_free(object_l);
    }
    if (object_r) {
    mem_pool_free(object_r);
    }
    }
#[no_mangle]
unsafe extern "C" fn __paint_it(object: *mut kmemleak_object, color: c_int) {
    object.min_count = color;
    if (color == KMEMLEAK_BLACK) {
    object.flags |= OBJECT_NO_SCAN;
    }
    }
#[no_mangle]
unsafe extern "C" fn paint_it(object: *mut kmemleak_object, color: c_int) {
    let mut flags = 0;
    raw_spin_lock_irqsave(&object.lock, flags);
    __paint_it(object, color);
    raw_spin_unlock_irqrestore(&object.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn paint_ptr(ptr: c_ulong, color: c_int, objflags: c_uint) {
pub static mut object: *mut c_void = core::ptr::null_mut();
    object = __find_and_get_object(ptr, 0, objflags);
    if (!object) {
//
// kmalloc_nolock() -> kfree_rcu() calls kmemleak_ignore()
// without kmemleak_alloc().
//
    return;
    }
    paint_it(object, color);
    put_object(object);
    }
//
// Mark an object permanently as gray-colored so that it can no longer be
// reported as a leak. This is used in general to mark a false positive.
//
#[no_mangle]
unsafe extern "C" fn make_gray_object(ptr: c_ulong) {
    paint_ptr(ptr, KMEMLEAK_GREY, 0);
    }
//
// Mark the object as black-colored so that it is ignored from scans and
// reporting.
//
#[no_mangle]
unsafe extern "C" fn make_black_object(ptr: c_ulong, objflags: c_uint) {
    paint_ptr(ptr, KMEMLEAK_BLACK, objflags);
    }
//
// Reset the checksum of an object. The immediate effect is that it will not
// be reported as a leak during the next scan until its checksum is updated.
//
#[no_mangle]
unsafe extern "C" fn reset_checksum(ptr: c_ulong) {
    let mut flags = 0;
pub static mut object: *mut c_void = core::ptr::null_mut();
    object = find_and_get_object(ptr, 0);
    if (!object) {
    kmemleak_warn("Not resetting the checksum of an unknown object at 0x%08lx\n",
    ptr);
    return;
    }
    raw_spin_lock_irqsave(&object.lock, flags);
    object.checksum = ~0;
    raw_spin_unlock_irqrestore(&object.lock, flags);
    put_object(object);
    }
//
// Add a scanning area to the object. If at least one such area is added,
// kmemleak will only scan these ranges rather than the whole memory block.
//
#[no_mangle]
unsafe extern "C" fn add_scan_area(ptr: c_ulong, size: usize, gfp: gfp_t) {
    let mut flags = 0;
pub static mut object: *mut c_void = core::ptr::null_mut();
    let mut area = core::ptr::null_mut();
    let mut untagged_ptr = 0;
    let mut untagged_objp = 0;
    object = find_and_get_object(ptr, 1);
    if (!object) {
    kmemleak_warn("Adding scan area to unknown object at 0x%08lx\n",
    ptr);
    return;
    }
    untagged_ptr = (unsigned long)kasan_reset_tag(ptr);
    untagged_objp = (unsigned long)kasan_reset_tag(object.pointer);
    if (scan_area_cache) {
    area = kmem_cache_alloc_noprof(scan_area_cache,
    gfp_nested_mask(gfp));
    }
    raw_spin_lock_irqsave(&object.lock, flags);
    if (!area) {
    pr_warn_once("Cannot allocate a scan area, scanning the full object\n");
// mark the object for full scan to avoid false positives
    object.flags |= OBJECT_FULL_SCAN;
// goto;
    }
    if (size == SIZE_MAX) {
    size = untagged_objp + object.size - untagged_ptr;
    } else if (untagged_ptr + size > untagged_objp + object.size) {
    kmemleak_warn("Scan area larger than object 0x%08lx\n", ptr);
    dump_object_info(object);
    kmem_cache_free(scan_area_cache, area);
// goto;
    }
    INIT_HLIST_NODE(&area.node);
    area.start = ptr;
    area.size = size;
    hlist_add_head(&area.node, &object.area_list);
// label;
    raw_spin_unlock_irqrestore(&object.lock, flags);
    put_object(object);
    }
//
// Any surplus references (object already gray) to 'ptr' are passed to
// 'excess_ref'. This is used in the vmalloc() case where a pointer to
// vm_struct may be used as an alternative reference to the vmalloc'ed object
// (see free_thread_stack()).
//
#[no_mangle]
unsafe extern "C" fn object_set_excess_ref(ptr: c_ulong, excess_ref: c_ulong) {
    let mut flags = 0;
pub static mut object: *mut c_void = core::ptr::null_mut();
    object = find_and_get_object(ptr, 0);
    if (!object) {
    kmemleak_warn("Setting excess_ref on unknown object at 0x%08lx\n",
    ptr);
    return;
    }
    raw_spin_lock_irqsave(&object.lock, flags);
    object.excess_ref = excess_ref;
    raw_spin_unlock_irqrestore(&object.lock, flags);
    put_object(object);
    }
//
// Set the OBJECT_NO_SCAN flag for the object corresponding to the given
// pointer. Such object will not be scanned by kmemleak but references to it
// are searched.
//
#[no_mangle]
unsafe extern "C" fn object_no_scan(ptr: c_ulong) {
    let mut flags = 0;
pub static mut object: *mut c_void = core::ptr::null_mut();
    object = find_and_get_object(ptr, 0);
    if (!object) {
    kmemleak_warn("Not scanning unknown object at 0x%08lx\n", ptr);
    return;
    }
    raw_spin_lock_irqsave(&object.lock, flags);
    object.flags |= OBJECT_NO_SCAN;
    raw_spin_unlock_irqrestore(&object.lock, flags);
    put_object(object);
    }
//
// kmemleak_alloc - register a newly allocated object
// @ptr:	pointer to beginning of the object
// @size:	size of the object
// @min_count:	minimum number of references to this object. If during memory
// scanning a number of references less than @min_count is found,
// the object is reported as a memory leak. If @min_count is 0,
// the object is never reported as a leak. If @min_count is -1,
the object is ignored (not scanned and not reported as a leak)
// @gfp:	kmalloc() flags used for kmemleak internal memory allocations
//
// This function is called from the kernel allocators when a new object
// (memory block) is allocated (kmem_cache_alloc, kmalloc etc.).
//
    void __ref kmemleak_alloc(const void *ptr, size_t size, int min_count,
    gfp_t gfp)
    {
    pr_debug!("%s(0x%px, %zu, %d)\n", __func__, ptr, size, min_count);
    if (kmemleak_enabled && ptr && !IS_ERR(ptr)) {
    create_object((unsigned long)ptr, size, min_count, gfp);
    }
    }
    EXPORT_SYMBOL_GPL(kmemleak_alloc);
//
// kmemleak_alloc_percpu - register a newly allocated  object
// @ptr:	 pointer to beginning of the object
// @size:	size of the object
// @gfp:	flags used for kmemleak internal memory allocations
//
// This function is called from the kernel percpu allocator when a new object
// (memory block) is allocated (alloc_percpu).
//
    void __ref kmemleak_alloc_percpu(const void  *ptr, size_t size,
    gfp_t gfp)
    {
    pr_debug!("%s(0x%px, %zu)\n", __func__, ptr, size);
    if (kmemleak_enabled && ptr && !IS_ERR_PCPU(ptr)) {
    create_object_percpu(( unsigned long)ptr, size, 1, gfp);
    }
    }
    EXPORT_SYMBOL_GPL(kmemleak_alloc_percpu);
//
// kmemleak_vmalloc - register a newly vmalloc'ed object
// @area:	pointer to vm_struct
// @size:	size of the object
// @gfp:	__vmalloc() flags used for kmemleak internal memory allocations
//
// This function is called from the vmalloc() kernel allocator when a new
// object (memory block) is allocated.
//
#[no_mangle]
pub unsafe extern "C" fn kmemleak_vmalloc(area: *const vm_struct, size: usize, gfp: gfp_t) -> void __ref {
    pr_debug!("%s(0x%px, %zu)\n", __func__, area, size);
//
// A min_count = 2 is needed because vm_struct contains a reference to
// the virtual address of the vmalloc'ed block.
//
    if (kmemleak_enabled) {
    create_object((unsigned long)area.addr, size, 2, gfp);
    object_set_excess_ref((unsigned long)area,
    (unsigned long)area.addr);
    }
    }
    EXPORT_SYMBOL_GPL(kmemleak_vmalloc);
//
// kmemleak_free - unregister a previously registered object
// @ptr:	pointer to beginning of the object
//
// This function is called from the kernel allocators when an object (memory
// block) is freed (kmem_cache_free, kfree, vfree etc.).
//
#[no_mangle]
pub unsafe extern "C" fn kmemleak_free(ptr: *const c_void) -> void __ref {
    pr_debug!("%s(0x%px)\n", __func__, ptr);
    if (kmemleak_free_enabled && ptr && !IS_ERR(ptr)) {
    delete_object_full((unsigned long)ptr, 0);
    }
    }
    EXPORT_SYMBOL_GPL(kmemleak_free);
//
// kmemleak_free_part - partially unregister a previously registered object
// @ptr:	pointer to the beginning or inside the object. This also
// represents the start of the range to be freed
// @size:	size to be unregistered
//
// This function is called when only a part of a memory block is freed
// (usually from the bootmem allocator).
//
#[no_mangle]
pub unsafe extern "C" fn kmemleak_free_part(ptr: *const c_void, size: usize) -> void __ref {
    pr_debug!("%s(0x%px)\n", __func__, ptr);
    if (kmemleak_enabled && ptr && !IS_ERR(ptr)) {
    delete_object_part((unsigned long)ptr, size, 0);
    }
    }
    EXPORT_SYMBOL_GPL(kmemleak_free_part);
//
// kmemleak_free_percpu - unregister a previously registered  object
// @ptr:	 pointer to beginning of the object
//
// This function is called from the kernel percpu allocator when an object
// (memory block) is freed (free_percpu).
//
#[no_mangle]
pub unsafe extern "C" fn kmemleak_free_percpu(ptr: *const c_void ) -> void __ref {
    pr_debug!("%s(0x%px)\n", __func__, ptr);
    if (kmemleak_free_enabled && ptr && !IS_ERR_PCPU(ptr)) {
    delete_object_full(( unsigned long)ptr, OBJECT_PERCPU);
    }
    }
    EXPORT_SYMBOL_GPL(kmemleak_free_percpu);
//
// kmemleak_update_trace - update object allocation stack trace
// @ptr:	pointer to beginning of the object
//
// Override the object allocation stack trace for cases where the actual
// allocation place is not always useful.
//
#[no_mangle]
pub unsafe extern "C" fn kmemleak_update_trace(ptr: *const c_void) -> void __ref {
pub static mut object: *mut c_void = core::ptr::null_mut();
    let mut trace_handle;
    let mut flags = 0;
    pr_debug!("%s(0x%px)\n", __func__, ptr);
    if (!kmemleak_enabled || IS_ERR_OR_NULL(ptr)) {
    return;
    }
    object = find_and_get_object((unsigned long)ptr, 1);
    if (!object) {

    kmemleak_warn("Updating stack trace for unknown object at %p\n",
    ptr);

    return;
    }
    trace_handle = set_track_prepare();
    raw_spin_lock_irqsave(&object.lock, flags);
    object.trace_handle = trace_handle;
    raw_spin_unlock_irqrestore(&object.lock, flags);
    put_object(object);
    }
    EXPORT_SYMBOL(kmemleak_update_trace);
//
// kmemleak_not_leak - mark an allocated object as false positive
// @ptr:	pointer to beginning of the object
//
// Calling this function on an object will cause the memory block to no longer
// be reported as leak and always be scanned.
//
#[no_mangle]
pub unsafe extern "C" fn kmemleak_not_leak(ptr: *const c_void) -> void __ref {
    pr_debug!("%s(0x%px)\n", __func__, ptr);
    if (kmemleak_enabled && ptr && !IS_ERR(ptr)) {
    make_gray_object((unsigned long)ptr);
    }
    }
    EXPORT_SYMBOL(kmemleak_not_leak);
//
// kmemleak_transient_leak - mark an allocated object as transient false positive
// @ptr:	pointer to beginning of the object
//
// Calling this function on an object will cause the memory block to not be
// reported as a leak temporarily. This may happen, for example, if the object
// is part of a singly linked list and the ->next reference to it is changed.
//
#[no_mangle]
pub unsafe extern "C" fn kmemleak_transient_leak(ptr: *const c_void) -> void __ref {
    pr_debug!("%s(0x%px)\n", __func__, ptr);
    if (kmemleak_enabled && ptr && !IS_ERR(ptr)) {
    reset_checksum((unsigned long)ptr);
    }
    }
    EXPORT_SYMBOL(kmemleak_transient_leak);
//
// kmemleak_ignore_percpu - similar to kmemleak_ignore but taking a percpu
// address argument
// @ptr:	percpu address of the object
//
#[no_mangle]
pub unsafe extern "C" fn kmemleak_ignore_percpu(ptr: *const c_void ) -> void __ref {
    pr_debug!("%s(0x%px)\n", __func__, ptr);
    if (kmemleak_enabled && ptr && !IS_ERR_PCPU(ptr)) {
    make_black_object((unsigned long)ptr, OBJECT_PERCPU);
    }
    }
    EXPORT_SYMBOL_GPL(kmemleak_ignore_percpu);
//
// kmemleak_ignore - ignore an allocated object
// @ptr:	pointer to beginning of the object
//
// Calling this function on an object will cause the memory block to be
// ignored (not scanned and not reported as a leak). This is usually done when
// it is known that the corresponding block is not a leak and does not contain
// any references to other allocated memory blocks.
//
#[no_mangle]
pub unsafe extern "C" fn kmemleak_ignore(ptr: *const c_void) -> void __ref {
    pr_debug!("%s(0x%px)\n", __func__, ptr);
    if (kmemleak_enabled && ptr && !IS_ERR(ptr)) {
    make_black_object((unsigned long)ptr, 0);
    }
    }
    EXPORT_SYMBOL(kmemleak_ignore);
//
// kmemleak_scan_area - limit the range to be scanned in an allocated object
// @ptr:	pointer to beginning or inside the object. This also
// represents the start of the scan area
// @size:	size of the scan area
// @gfp:	kmalloc() flags used for kmemleak internal memory allocations
//
// This function is used when it is known that only certain parts of an object
// contain references to other objects. Kmemleak will only scan these areas
// reducing the number false negatives.
//
#[no_mangle]
pub unsafe extern "C" fn kmemleak_scan_area(ptr: *const c_void, size: usize, gfp: gfp_t) -> void __ref {
    pr_debug!("%s(0x%px)\n", __func__, ptr);
    if (kmemleak_enabled && ptr && size && !IS_ERR(ptr)) {
    add_scan_area((unsigned long)ptr, size, gfp);
    }
    }
    EXPORT_SYMBOL(kmemleak_scan_area);
//
// kmemleak_no_scan - do not scan an allocated object
// @ptr:	pointer to beginning of the object
//
// This function notifies kmemleak not to scan the given memory block. Useful
// in situations where it is known that the given object does not contain any
// references to other objects. Kmemleak will not scan such objects reducing
// the number of false negatives.
//
#[no_mangle]
pub unsafe extern "C" fn kmemleak_no_scan(ptr: *const c_void) -> void __ref {
    pr_debug!("%s(0x%px)\n", __func__, ptr);
    if (kmemleak_enabled && ptr && !IS_ERR(ptr)) {
    object_no_scan((unsigned long)ptr);
    }
    }
    EXPORT_SYMBOL(kmemleak_no_scan);
//
// kmemleak_alloc_phys - similar to kmemleak_alloc but taking a physical
// address argument
// @phys:	physical address of the object
// @size:	size of the object
// @gfp:	kmalloc() flags used for kmemleak internal memory allocations
//
#[no_mangle]
pub unsafe extern "C" fn kmemleak_alloc_phys(phys: phys_addr_t, size: usize, gfp: gfp_t) -> void __ref {
    pr_debug!("%s(0x%px, %zu)\n", __func__, &phys, size);
    if (kmemleak_enabled) {
//
// Create object with OBJECT_PHYS flag and
// assume min_count 0.
//
    create_object_phys((unsigned long)phys, size, 0, gfp);
    }
    }
    EXPORT_SYMBOL(kmemleak_alloc_phys);
//
// kmemleak_free_part_phys - similar to kmemleak_free_part but taking a
// physical address argument
// @phys:	physical address if the beginning or inside an object. This
// also represents the start of the range to be freed
// @size:	size to be unregistered
//
#[no_mangle]
pub unsafe extern "C" fn kmemleak_free_part_phys(phys: phys_addr_t, size: usize) -> void __ref {
    pr_debug!("%s(0x%px)\n", __func__, &phys);
    if (kmemleak_enabled) {
    delete_object_part((unsigned long)phys, size, OBJECT_PHYS);
    }
    }
    EXPORT_SYMBOL(kmemleak_free_part_phys);
//
// kmemleak_ignore_phys - similar to kmemleak_ignore but taking a physical
// address argument
// @phys:	physical address of the object
//
#[no_mangle]
pub unsafe extern "C" fn kmemleak_ignore_phys(phys: phys_addr_t) -> void __ref {
    pr_debug!("%s(0x%px)\n", __func__, &phys);
    if (kmemleak_enabled) {
    make_black_object((unsigned long)phys, OBJECT_PHYS);
    }
    }
    EXPORT_SYMBOL(kmemleak_ignore_phys);
//
// Update an object's checksum and return true if it was modified.
//
#[no_mangle]
unsafe extern "C" fn update_checksum(object: *mut kmemleak_object) -> bool {
pub static mut old_csum: u32 = 0;
    if (WARN_ON_ONCE!(object.flags & OBJECT_PHYS)) {
    return false;
    }
    kasan_disable_current();
    kcsan_disable_current();
    if (object.flags & OBJECT_PERCPU) {
    let mut cpu = 0;
    object.checksum = 0;
    for_each_possible_cpu(cpu) {
    let mut ptr = per_cpu_ptr(object.pointer, cpu);
    object.checksum = crc32(object.checksum,
    kasan_reset_tag(ptr), object.size);
    }
    } else {
    object.checksum = crc32(0, kasan_reset_tag(object.pointer), object.size);
    }
    kasan_enable_current();
    kcsan_enable_current();
    return object.checksum != old_csum;
    }
//
// Update an object's references. object->lock must be held by the caller.
//
#[no_mangle]
unsafe extern "C" fn update_refs(object: *mut kmemleak_object) {
    if (!color_white(object)) {
// non-orphan, ignored or new
    return;
    }
//
// Increase the object's reference count (number of pointers to the
// memory block). If this count reaches the required minimum, the
// object's color will become gray and it will be added to the
// gray_list.
//
    object.count += 1;
    if (color_gray(object)) {
// referenced after all, no longer a suspect
    if (object.flags & OBJECT_SUSPECT) {
    object.flags &= ~OBJECT_SUSPECT;
    nr_suspects -= 1;
    }
// put_object() called when removing from gray_list
    WARN_ON!(!get_object(object));
    list_add_tail(&object.gray_list, &gray_list);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pointer_update_refs(scanned: *mut kmemleak_object, pointer: c_ulong, objflags: c_uint) {
pub static mut object: *mut c_void = core::ptr::null_mut();
    let mut untagged_ptr = 0;
    let mut excess_ref = 0;
    untagged_ptr = (unsigned long)kasan_reset_tag(pointer);
    if (objflags & OBJECT_PERCPU) {
    if (untagged_ptr < min_percpu_addr || untagged_ptr >= max_percpu_addr) {
    return;
    }
    } else {
    if (untagged_ptr < min_addr || untagged_ptr >= max_addr) {
    return;
    }
    }
//
// No need for get_object() here since we hold kmemleak_lock.
// object->use_count cannot be dropped to 0 while the object
// is still present in object_tree_root and object_list
// (with updates protected by kmemleak_lock).
//
    object = __lookup_object(pointer, 1, objflags);
    if (!object) {
    return;
    }
    if (object == scanned) {
// self referenced, ignore
    return;
    }
//
// Avoid the lockdep recursive warning on object->lock being
// previously acquired in scan_object(). These locks are
// enclosed by scan_mutex.
//
    raw_spin_lock_nested(&object.lock, SINGLE_DEPTH_NESTING);
// only pass surplus references (object already gray)
    if (color_gray(object)) {
    excess_ref = object.excess_ref;
// no need for update_refs() if object already gray
    } else {
    excess_ref = 0;
    update_refs(object);
    }
    raw_spin_unlock(&object.lock);
    if (excess_ref) {
    object = lookup_object(excess_ref, 0);
    if (!object) {
    return;
    }
    if (object == scanned) {
// circular reference, ignore
    return;
    }
    raw_spin_lock_nested(&object.lock, SINGLE_DEPTH_NESTING);
    update_refs(object);
    raw_spin_unlock(&object.lock);
    }
    }
//
// Memory scanning is a long process and it needs to be interruptible. This
// function checks whether such interrupt condition occurred.
//
#[no_mangle]
unsafe extern "C" fn scan_should_stop() -> c_int {
    if (!kmemleak_enabled) {
    return 1;
    }
//
// This function may be called from either process or kthread context,
// hence the need to check for both stop conditions.
//
    if (current.flags & PF_KTHREAD) {
    return kthread_should_stop();
    }
    return signal_pending(current);
    }
//
// Scan a memory block (exclusive range) for valid pointers and add those
// found to the gray list. Return non-zero if the scan was interrupted.
//
#[no_mangle]
pub unsafe extern "C" fn scan_block(_start: *mut c_void, _end: *mut c_void, scanned: *mut kmemleak_object) -> c_int {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    let mut start = PTR_ALIGN(_start, BYTES_PER_POINTER);
    let mut end = _end - (BYTES_PER_POINTER - 1);
    let mut flags = 0;
pub static mut stop: c_int = 0;
    raw_spin_lock_irqsave(&kmemleak_lock, flags);
    while (ptr < end) {
    let mut pointer = 0;
    if (scan_should_stop()) {
    stop = 1;
    break;
    }
    kasan_disable_current();
    pointer = *kasan_reset_tag(ptr);
    kasan_enable_current();
    pointer_update_refs(scanned, pointer, 0);
    pointer_update_refs(scanned, pointer, OBJECT_PERCPU);
    }
    raw_spin_unlock_irqrestore(&kmemleak_lock, flags);
    return stop;
    }
//
// Scan a large memory block in MAX_SCAN_SIZE chunks to reduce the latency.
// Return non-zero if the scan was interrupted.
//

#[no_mangle]
unsafe extern "C" fn scan_large_block(start: *mut c_void, end: *mut c_void) -> c_int {
pub static mut next: *mut c_void = core::ptr::null_mut();
    while (start < end) {
    next = min(start + MAX_SCAN_SIZE, end);
    if (scan_block(start, next, core::ptr::null_mut())) {
    return 1;
    }
    start = next;
    cond_resched_tasks_rcu_qs();
    }
    return 0;
    }

//
// Scan a memory block corresponding to a kmemleak_object. A condition is
// that object->use_count >= 1.
//
#[no_mangle]
unsafe extern "C" fn scan_object(object: *mut kmemleak_object) {
pub static mut area: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
//
// Once the object->lock is acquired, the corresponding memory block
// cannot be freed (the same lock is acquired in delete_object).
//
    raw_spin_lock_irqsave(&object.lock, flags);
    if (object.flags & OBJECT_NO_SCAN) {
// goto;
    }
    if (!(object.flags & OBJECT_ALLOCATED)) {
// already freed object
// goto;
    }
    if (object.flags & OBJECT_PERCPU) {
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    let mut start = per_cpu_ptr(object.pointer, cpu);
    let mut end = start + object.size;
    scan_block(start, end, object);
    raw_spin_unlock_irqrestore(&object.lock, flags);
    cond_resched_tasks_rcu_qs();
    raw_spin_lock_irqsave(&object.lock, flags);
    if (!(object.flags & OBJECT_ALLOCATED)) {
    break;
    }
    }
    } else if (hlist_empty(&object.area_list) ||
    object.flags & OBJECT_FULL_SCAN) {
    let mut start = object.flags & OBJECT_PHYS ?
    __va((phys_addr_t)object.pointer) :
    object.pointer;
    let mut end = start + object.size;
pub static mut next: *mut c_void = core::ptr::null_mut();
    do {
    next = min(start + MAX_SCAN_SIZE, end);
    scan_block(start, next, object);
    start = next;
    if (start >= end) {
    break;
    }
    raw_spin_unlock_irqrestore(&object.lock, flags);
    cond_resched_tasks_rcu_qs();
    raw_spin_lock_irqsave(&object.lock, flags);
    } while (object.flags & OBJECT_ALLOCATED);
    } else {
    hlist_for_each_entry(area, &object.area_list, node)
    scan_block(area.start,
    (area.start + area.size),
    object);
    }
// label;
    raw_spin_unlock_irqrestore(&object.lock, flags);
    }
//
// Scan the objects already referenced (gray objects). More objects will be
// referenced and, if there are no memory leaks, all the objects are scanned.
//
#[no_mangle]
unsafe extern "C" fn scan_gray_list() {
    let mut object = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
//
// The list traversal is safe for both tail additions and removals
// from inside the loop. The kmemleak objects cannot be freed from
// outside the loop because their use_count was incremented.
//
    object = list_entry(gray_list.next, typeof(*object), gray_list);
    while (&object.gray_list != &gray_list) {
    cond_resched_tasks_rcu_qs();
// may add new objects to the list
    if (!scan_should_stop()) {
    scan_object(object);
    }
    tmp = list_entry(object.gray_list.next, typeof(*object),
    gray_list);
// remove the object from the list and release it
    list_del(&object.gray_list);
    put_object(object);
    object = tmp;
    }
    WARN_ON!(!list_empty(&gray_list));
    }
//
// Conditionally call resched() in an object iteration loop while making sure
// that the given object won't go away without RCU read lock by performing a
// get_object() if necessaary.
//
#[no_mangle]
unsafe extern "C" fn kmemleak_cond_resched(object: *mut kmemleak_object) {
    if (!get_object(object)) {
    return;	/* Try next object */
    }
    raw_spin_lock_irq(&kmemleak_lock);
    if (object.del_state & DELSTATE_REMOVED) {
// goto;	/* Object removed */
    }
    object.del_state |= DELSTATE_NO_DELETE;
    raw_spin_unlock_irq(&kmemleak_lock);
    rcu_read_unlock();
    cond_resched_tasks_rcu_qs();
    rcu_read_lock();
    raw_spin_lock_irq(&kmemleak_lock);
    if (object.del_state & DELSTATE_REMOVED) {
    list_del_rcu(&object.object_list);
    }
    object.del_state &= ~DELSTATE_NO_DELETE;
// label;
    raw_spin_unlock_irq(&kmemleak_lock);
    put_object(object);
    }
//
// Scan all task kernel stacks, rescheduling between tasks. Each task is looked
// up and pinned within its own RCU read-side section, so no lock is held across
// the scan and the walk cannot trip the soft lockup watchdog.
//
#[no_mangle]
unsafe extern "C" fn kmemleak_scan_task_stacks() {
pub static mut pid: *mut c_void = core::ptr::null_mut();
pub static mut nr: c_int = 1;
pub static mut stop: c_int = 0;
    do {
    let mut p = core::ptr::null_mut();
    rcu_read_lock();
    pid = find_ge_pid(nr, &init_pid_ns);
    if (pid) {
    nr = pid_nr(pid) + 1;
    p = pid_task(pid, PIDTYPE_PID);
    if (p) {
    get_task_struct(p);
    }
    }
    rcu_read_unlock();
    if (p) {
    let mut stack = try_get_task_stack(p);
    if (stack) {
    stop = scan_block(stack, stack + THREAD_SIZE, core::ptr::null_mut());
    put_task_stack(p);
    }
    put_task_struct(p);
    }
    cond_resched_tasks_rcu_qs();
    } while (pid && !stop);
    }
//
// Print one leak inline. The hex dump is gated on OBJECT_ALLOCATED so it
// does not touch user memory that was freed concurrently; the rest of the
// report (backtrace, comm, pid) is always emitted since the kmemleak_object
// metadata is pinned by the caller.
//
#[no_mangle]
unsafe extern "C" fn print_leak_locked(object: *mut kmemleak_object, hex_dump: bool) {
    raw_spin_lock_irq(&object.lock);
    __print_unreferenced(core::ptr::null_mut(), object,
    hex_dump && (object.flags & OBJECT_ALLOCATED));
    raw_spin_unlock_irq(&object.lock);
    }
//
// Per-scan dedup table for verbose leak printing. The xarray is keyed by
// stackdepot trace_handle and stores a pointer to the representative
// kmemleak_object. The per-scan repeat count lives in object->dup_count.
//
// dedup_record() must run outside object->lock: xa_store() may take
// mutexes (xa_node slab allocation) which lockdep would flag against the
// raw spinlock object->lock.
//
#[no_mangle]
pub unsafe extern "C" fn dedup_record(dedup: *mut xarray, object: *mut kmemleak_object, trace_handle: depot_stack_handle_t) {
pub static mut rep: *mut c_void = core::ptr::null_mut();
pub static mut old: *mut c_void = core::ptr::null_mut();
//
// No stack trace to dedup against: early-boot allocation tracked
// before kmemleak_init() set up object_cache, or stack_depot_save()
// failure under memory pressure.
//
    if (!trace_handle) {
    print_leak_locked(object, true);
    return;
    }
// stack is available, now we can de-dup
    rep = xa_load(dedup, trace_handle);
    if (rep) {
    rep.dup_count += 1;
    return;
    }
//
// Object is being torn down (use_count already hit zero); the
// tracked memory at object->pointer is unsafe to read, so skip.
//
    if (!get_object(object)) {
    return;
    }
    object.dup_count = 1;
    old = xa_store(dedup, trace_handle, object, GFP_ATOMIC);
    if (xa_is_err(old)) {
// xa_node allocation failed; fall back to inline print.
    print_leak_locked(object, true);
    put_object(object);
    return;
    }
//
// scan_mutex serialises all writers to the dedup xarray, so xa_store()
// after a NULL xa_load() must always overwrite an empty slot.
//
    WARN_ON_ONCE!(old);
    }
//
// Drain the dedup table. Re-acquires object->lock and re-checks
// OBJECT_ALLOCATED before printing: while get_object() pins the
// kmemleak_object metadata, the underlying tracked allocation may have
// been freed since the scan walked it (kmemleak_free clears
// OBJECT_ALLOCATED under object->lock before the user memory goes away).
// The hex dump is skipped for coalesced entries since the bytes would
// differ across objects anyway.
//
#[no_mangle]
unsafe extern "C" fn dedup_flush(dedup: *mut xarray) {
pub static mut object: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    let mut dup = 0;
    let mut coalesced = 0;
    xa_for_each(dedup, idx, object) {
    dup = object.dup_count;
    coalesced = dup > 1;
    print_leak_locked(object, !coalesced);
    if (coalesced) {
    pr_warn!("  ... and %u more object(s) with the same backtrace\n",
    dup - 1);
    }
    put_object(object);
    xa_erase(dedup, idx);
    }
    }
//
// Scan data sections and all the referenced memory blocks allocated via the
// kernel's standard allocators. This function must be called with the
// scan_mutex held.
//
#[no_mangle]
unsafe extern "C" fn __kmemleak_scan(full: bool) -> c_int {
pub static mut object: *mut c_void = core::ptr::null_mut();
pub static mut zone: *mut c_void = core::ptr::null_mut();
    int __maybe_unused i;
pub static mut stop: c_int = 0;
    jiffies_last_scan = jiffies;
    if (full) {
    nr_suspects = 0;
    }
// prepare the kmemleak_object's
    rcu_read_lock();
    list_for_each_entry_rcu(object, &object_list, object_list) {
    raw_spin_lock_irq(&object.lock);

//
// With a few exceptions there should be a maximum of
// 1 reference to any object at this point.
//
    if (atomic_read(&object.use_count) > 1) {
    pr_debug!("object.use_count = %d\n",
    atomic_read(&object.use_count));
    dump_object_info(object);
    }

// ignore objects outside lowmem (paint them black)
    if ((object.flags & OBJECT_PHYS) &&
    !(object.flags & OBJECT_NO_SCAN)) {
pub static mut phys: c_ulong = 0;
    if (PHYS_PFN(phys) < min_low_pfn ||
    PHYS_PFN(phys + object.size) > max_low_pfn) {
    __paint_it(object, KMEMLEAK_BLACK);
    }
    }
// referenced last scan: restart the unreferenced run
    if (!color_white(object)) {
    object.unref_scans = 0;
    }
// reset the reference count (whiten the object)
    object.count = 0;
    if (full) {
    object.flags &= ~OBJECT_SUSPECT;
    }
    if (color_gray(object) && get_object(object)) {
    list_add_tail(&object.gray_list, &gray_list);
    }
    raw_spin_unlock_irq(&object.lock);
    if (need_resched()) {
    kmemleak_cond_resched(object);
    }
    }
    rcu_read_unlock();

// per-cpu sections scanning
    for_each_possible_cpu(i) {
    if (scan_large_block(__per_cpu_start + per_cpu_offset(i),
    __per_cpu_end + per_cpu_offset(i))) {
// goto;
    }
    }

//
// Struct page scanning for each node.
//
    get_online_mems();
    for_each_populated_zone(zone) {
pub static mut start_pfn: c_ulong = 0;
pub static mut end_pfn: c_ulong = 0;
    let mut pfn = 0;
    while (pfn < end_pfn) {
    let mut page = pfn_to_online_page(pfn);
    if (!(pfn & 63)) {
    cond_resched_tasks_rcu_qs();
    }
    if (!page) {
    continue;
    }
// only scan pages belonging to this zone
    if (page_zone(page) != zone) {
    continue;
    }
// only scan if page is in use
    if (page_count(page) == 0) {
    continue;
    }
    stop = scan_block(page, page + 1, core::ptr::null_mut());
    if (stop) {
    break;
    }
    }
    if (stop) {
    break;
    }
    }
    put_online_mems();
    if (stop) {
// goto;
    }
//
// Scanning the task stacks (may introduce false negatives).
//
    if (kmemleak_stack_scan) {
    kmemleak_scan_task_stacks();
    }
//
// Scan the objects already referenced from the sections scanned
// above.
//
// label;
    scan_gray_list();
// a confirmation scan does not look for modified objects
    if (!full) {
    return nr_suspects;
    }
//
// Check for new or unreferenced objects modified since the previous
// scan and color them gray until the next scan.
//
    rcu_read_lock();
    list_for_each_entry_rcu(object, &object_list, object_list) {
    if (need_resched()) {
    kmemleak_cond_resched(object);
    }
//
// This is racy but we can save the overhead of lock/unlock
// calls. The missed objects, if any, should be caught in
// the next scan.
//
    if (!color_white(object)) {
    continue;
    }
    raw_spin_lock_irq(&object.lock);
    if (color_white(object) && (object.flags & OBJECT_ALLOCATED)
    && update_checksum(object) && get_object(object)) {
// color it gray temporarily
    object.count = object.min_count;
    list_add_tail(&object.gray_list, &gray_list);
    } else if (unreferenced_object(object) &&
    !(object.flags & OBJECT_REPORTED)) {
// flag the objects left unreferenced by this scan
    object.flags |= OBJECT_SUSPECT;
    nr_suspects += 1;
    }
    raw_spin_unlock_irq(&object.lock);
    }
    rcu_read_unlock();
//
// Re-scan the gray list for modified unreferenced objects.
//
    scan_gray_list();
    return nr_suspects;
    }
//
// Promote a suspected object to a reported leak once it has stayed
// unreferenced for min_unref_scans consecutive scans. Called with
// object->lock held; returns true when the object is newly reported.
//
#[no_mangle]
unsafe extern "C" fn confirm_leak(object: *mut kmemleak_object) -> bool {
    if (!unreferenced_object(object) ||
    !(object.flags & OBJECT_SUSPECT) ||
    (object.flags & OBJECT_REPORTED)) {
    return false;
    }
    object.unref_scans += 1;
    if (object.unref_scans < min_unref_scans) {
    return false;
    }
    object.flags |= OBJECT_REPORTED;
    return true;
    }
//
// Scan the memory and report the unreferenced objects as leaks. Must be
// called with the scan_mutex held.
//
#[no_mangle]
unsafe extern "C" fn kmemleak_scan() {
pub static mut object: *mut c_void = core::ptr::null_mut();
pub static mut dedup: usize = 0;
pub static mut new_leaks: c_int = 0;
//
// Full scan. Objects left unreferenced are flagged OBJECT_SUSPECT and
// counted in the return value; nothing to confirm or report otherwise.
//
    if (!__kmemleak_scan(true)) {
    return;
    }
//
// If scanning was stopped do not report any new unreferenced objects.
//
    if (scan_should_stop()) {
    return;
    }
//
// A live object whose only reference is moved by, for example, a
// concurrent RCU update can be missed for one scan and reported as a
// transient false positive. Scan again and only report the objects
// left unreferenced (still flagged OBJECT_SUSPECT) by both scans.
//
    __kmemleak_scan(false);
    if (scan_should_stop()) {
    return;
    }
//
// Scanning result reporting. When verbose printing is enabled, dedupe
// by stackdepot trace_handle so each unique backtrace is logged once
// per scan, annotated with the number of objects that share it. The
// per-leak count below still reflects every object, and
// /sys/kernel/debug/kmemleak still lists them individually.
//
    xa_init(&dedup);
    rcu_read_lock();
    list_for_each_entry_rcu(object, &object_list, object_list) {
    let mut trace_handle;
    let mut dedup_print = 0;
    if (need_resched()) {
    kmemleak_cond_resched(object);
    }
//
// This is racy but we can save the overhead of lock/unlock
// calls. The missed objects, if any, should be caught in
// the next scan.
//
    if (!color_white(object)) {
    continue;
    }
    raw_spin_lock_irq(&object.lock);
    trace_handle = 0;
    dedup_print = false;
    if (confirm_leak(object)) {
    if (kmemleak_verbose) {
    trace_handle = object.trace_handle;
    dedup_print = true;
    }
    new_leaks += 1;
    }
    raw_spin_unlock_irq(&object.lock);
//
// Defer the verbose print outside object->lock: xa_store()
// may take xa_node slab locks at a higher wait-context level
// which lockdep would flag against the raw_spinlock_t
// object->lock. rcu_read_lock() keeps the kmemleak_object
// alive across the call.
//
    if (dedup_print) {
    dedup_record(&dedup, object, trace_handle);
    }
    }
    rcu_read_unlock();
// Flush'em all
    dedup_flush(&dedup);
    xa_destroy(&dedup);
    if (new_leaks) {
    kmemleak_found_leaks = true;
    pr_info!("%d new suspected memory leaks (see /sys/kernel/debug/kmemleak)\n",
    new_leaks);
    }
    }
//
// Thread function performing automatic memory scanning. Unreferenced objects
// at the end of a memory scan are reported but only the first time.
//
#[no_mangle]
unsafe extern "C" fn kmemleak_scan_thread(arg: *mut c_void) -> c_int {
pub static mut first_run: int = 0;
    pr_info!("Automatic memory scanning thread started\n");
    set_user_nice(current, 10);
//
// Wait before the first scan to allow the system to fully initialize.
//
    if (first_run) {
pub static mut timeout: signed long = 0;
    first_run = 0;
    while (timeout && !kthread_should_stop()) {
    timeout = schedule_timeout_interruptible(timeout);
    }
    }
    while (!kthread_should_stop()) {
pub static mut timeout: signed long = 0;
    mutex_lock(&scan_mutex);
    kmemleak_scan();
    mutex_unlock(&scan_mutex);
// wait before the next scan
    while (timeout && !kthread_should_stop()) {
    timeout = schedule_timeout_interruptible(timeout);
    }
    }
    pr_info!("Automatic memory scanning thread ended\n");
    return 0;
    }
//
// Start the automatic memory scanning thread. This function must be called
// with the scan_mutex held.
//
#[no_mangle]
unsafe extern "C" fn start_scan_thread() {
    if (scan_thread) {
    return;
    }
    scan_thread = kthread_run(kmemleak_scan_thread, core::ptr::null_mut(), "kmemleak");
    if (IS_ERR(scan_thread)) {
    pr_warn!("Failed to create the scan thread\n");
    scan_thread = core::ptr::null_mut();
    }
    }
//
// Stop the automatic memory scanning thread.
//
#[no_mangle]
unsafe extern "C" fn stop_scan_thread() {
    if (scan_thread) {
    kthread_stop(scan_thread);
    scan_thread = core::ptr::null_mut();
    }
    }
//
// Iterate over the object_list and return the first valid object at or after
// the required position with its use_count incremented. The function triggers
// a memory scanning when the pos argument points to the first position.
//
#[no_mangle]
pub unsafe extern "C" fn kmemleak_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
pub static mut object: *mut c_void = core::ptr::null_mut();
pub static mut n: loff_t = 0;
    let mut err = 0;
    err = mutex_lock_interruptible(&scan_mutex);
    if (err < 0) {
    return ERR_PTR(err);
    }
    rcu_read_lock();
    list_for_each_entry_rcu(object, &object_list, object_list) {
    if (n-- > 0) {
    continue;
    }
    if (get_object(object)) {
// goto;
    }
    }
    object = core::ptr::null_mut();
// label;
    return object;
    }
//
// Return the next object in the object_list. The function decrements the
// use_count of the previous object and increases that of the next one.
//
#[no_mangle]
pub unsafe extern "C" fn kmemleak_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut prev_obj = v;
    let mut next_obj = core::ptr::null_mut();
    let mut obj = prev_obj;
    ++(*pos);
    list_for_each_entry_continue_rcu(obj, &object_list, object_list) {
    if (get_object(obj)) {
    next_obj = obj;
    break;
    }
    }
    put_object(prev_obj);
    return next_obj;
    }
//
// Decrement the use_count of the last object required, if any.
//
#[no_mangle]
unsafe extern "C" fn kmemleak_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    if (!IS_ERR(v)) {
//
// kmemleak_seq_start may return ERR_PTR if the scan_mutex
// waiting was interrupted, so only release it if !IS_ERR.
//
    rcu_read_unlock();
    mutex_unlock(&scan_mutex);
    if (v) {
    put_object(v);
    }
    }
    }
//
// Print the information for an unreferenced object to the seq file.
//
#[no_mangle]
unsafe extern "C" fn kmemleak_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let mut object = v;
    let mut flags = 0;
    raw_spin_lock_irqsave(&object.lock, flags);
    if ((object.flags & OBJECT_REPORTED) && unreferenced_object(object)) {
    print_unreferenced(seq, object);
    }
    raw_spin_unlock_irqrestore(&object.lock, flags);
    return 0;
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn kmemleak_open(inode: *mut inode, file: *mut file) -> c_int {
    return seq_open(file, &kmemleak_seq_ops);
    }
#[no_mangle]
unsafe extern "C" fn __dump_str_object_info(addr: c_ulong, objflags: c_uint) -> bool {
    let mut flags = 0;
pub static mut object: *mut c_void = core::ptr::null_mut();
    object = __find_and_get_object(addr, 1, objflags);
    if (!object) {
    return false;
    }
    raw_spin_lock_irqsave(&object.lock, flags);
    dump_object_info(object);
    raw_spin_unlock_irqrestore(&object.lock, flags);
    put_object(object);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn dump_str_object_info(str: *const c_char) -> c_int {
    let mut addr = 0;
pub static mut found: bool = false;
    if (kstrtoul(str, 0, &addr)) {
    return -EINVAL;
    }
    found |= __dump_str_object_info(addr, 0);
    found |= __dump_str_object_info(addr, OBJECT_PHYS);
    found |= __dump_str_object_info(addr, OBJECT_PERCPU);
    if (!found) {
    pr_info!("Unknown object at 0x%08lx\n", addr);
    return -EINVAL;
    }
    return 0;
    }
//
// We use grey instead of black to ensure we can do future scans on the same
// objects. If we did not do future scans these black objects could
// potentially contain references to newly allocated objects in the future and
// we'd end up with false positives.
//
#[no_mangle]
unsafe extern "C" fn kmemleak_clear() {
pub static mut object: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    list_for_each_entry_rcu(object, &object_list, object_list) {
    raw_spin_lock_irq(&object.lock);
    if ((object.flags & OBJECT_REPORTED) &&
    unreferenced_object(object)) {
    __paint_it(object, KMEMLEAK_GREY);
    }
    raw_spin_unlock_irq(&object.lock);
    }
    rcu_read_unlock();
    kmemleak_found_leaks = false;
    }
// forward_decl: __kmemleak_do_cleanup;
//
// File write operation to configure kmemleak at run-time. The following
// commands can be written to the /sys/kernel/debug/kmemleak file:
// off	- disable kmemleak (irreversible)
// stack=on	- enable the task stacks scanning
// stack=off	- disable the tasks stacks scanning
// scan=on	- start the automatic memory scanning thread
// scan=off	- stop the automatic memory scanning thread
// scan=...	- set the automatic memory scanning period in seconds (0 to
// disable it)
// scan	- trigger a memory scan
// clear	- mark all current reported unreferenced kmemleak objects as
// grey to ignore printing them, or free all kmemleak objects
// if kmemleak has been disabled.
// dump=...	- dump information about the object found at the given address
//
#[no_mangle]
pub unsafe extern "C" fn kmemleak_write(file: *mut file, user_buf: *mut c_char, size: size_t, ppos: *mut loff_t) -> ssize_t {
    char buf[64];
    let mut buf_size = 0;
    let mut ret = 0;
    buf_size = min(size, (sizeof!(buf) - 1));
    if (strncpy_from_user(buf, user_buf, buf_size) < 0) {
    return -EFAULT;
    }
    buf[buf_size] = 0;
    ret = mutex_lock_interruptible(&scan_mutex);
    if (ret < 0) {
    return ret;
    }
    if (strncmp(buf, "clear", 5) == 0) {
    if (kmemleak_enabled) {
    kmemleak_clear();
    }
    else {
    __kmemleak_do_cleanup();
    }
// goto;
    }
    if (!kmemleak_enabled) {
    ret = -EPERM;
// goto;
    }
    if (strncmp(buf, "off", 3) == 0) {
    kmemleak_disable();
    }

    else if (strncmp(buf, "stack=on", 8) == 0) {
    kmemleak_stack_scan = 1;
    }

    else if (strncmp(buf, "stack=off", 9) == 0) {
    kmemleak_stack_scan = 0;
    }

    else if (strncmp(buf, "scan=on", 7) == 0) {
    start_scan_thread();
    }

    else if (strncmp(buf, "scan=off", 8) == 0) {
    stop_scan_thread();
    }
if true {
    let mut secs: c_uint = 0;
    let mut msecs = 0;
    ret = kstrtouint(buf + 5, 0, &secs);
    if (ret < 0) {
// goto;
    }
    msecs = secs * MSEC_PER_SEC;
    if (msecs > UINT_MAX) {
    msecs = UINT_MAX;
    }
    stop_scan_thread();
    if (msecs) {
    WRITE_ONCE(jiffies_scan_wait, msecs_to_jiffies(msecs));
    start_scan_thread();
    }
    } else if (strncmp(buf, "scan", 4) == 0) {
    kmemleak_scan();
    }

    else if (strncmp(buf, "dump=", 5) == 0) {
    ret = dump_str_object_info(buf + 5);
    }
    else {
    ret = -EINVAL;
    }
// label;
    mutex_unlock(&scan_mutex);
    if (ret < 0) {
    return ret;
    }
// ignore the rest of the buffer, only one command at a time
// ppos += size;
    return size;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn __kmemleak_do_cleanup() {
    let mut object = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
pub static mut cnt: c_uint = 0;
//
// Kmemleak has already been disabled, no need for RCU list traversal
// or kmemleak_lock held.
//
    list_for_each_entry_safe(object, tmp, &object_list, object_list) {
    __remove_object(object);
    __delete_object(object);
// Call cond_resched() once per 64 iterations to avoid soft lockup
    if (!(++cnt & 0x3f)) {
    cond_resched();
    }
    }
    }
//
// Stop the memory scanning thread and free the kmemleak internal objects if
// no previous scan thread (otherwise, kmemleak may still have some useful
// information on memory leaks).
//
#[no_mangle]
unsafe extern "C" fn kmemleak_do_cleanup(work: *mut work_struct) {
    stop_scan_thread();
    mutex_lock(&scan_mutex);
//
// Once it is made sure that kmemleak_scan has stopped, it is safe to no
// longer track object freeing. Ordering of the scan thread stopping and
// the memory accesses below is guaranteed by the kthread_stop()
// function.
//
    kmemleak_free_enabled = 0;
    mutex_unlock(&scan_mutex);
    if (!kmemleak_found_leaks) {
    __kmemleak_do_cleanup();
    }
    else {
    pr_info!("Kmemleak disabled without freeing internal data. Reclaim the memory with \"echo clear > /sys/kernel/debug/kmemleak\".\n");
    }
    }
pub static mut cleanup_work: usize = 0;
//
// Disable kmemleak. No memory allocation/freeing will be traced once this
// function is called. Disabling kmemleak is an irreversible operation.
//
#[no_mangle]
unsafe extern "C" fn kmemleak_disable() {
// atomically check whether it was already invoked
    if (cmpxchg(&kmemleak_error, 0, 1)) {
    return;
    }
// stop any memory operation tracing
    kmemleak_enabled = 0;
// check whether it is too early for a kernel thread
    if (kmemleak_late_initialized) {
    schedule_work(&cleanup_work);
    }
    else {
    kmemleak_free_enabled = 0;
    }
    pr_info!("Kernel memory leak detector disabled\n");
    }
//
// Allow boot-time kmemleak disabling (enabled by default).
//
#[no_mangle]
unsafe extern "C" fn kmemleak_boot_config(str: *mut c_char) -> c_int {
    if (!str) {
    return -EINVAL;
    }
    if (strcmp(str, "off") == 0) {
    kmemleak_disable();
    }
if true {
    kmemleak_skip_disable = 1;
    stack_depot_request_early_init();
    }
    else {
    return -EINVAL;
    }
    return 0;
    }
    early_param!("kmemleak", kmemleak_boot_config);
//
// Kmemleak initialization.
//
#[no_mangle]
pub unsafe extern "C" fn kmemleak_init()  {

    if (!kmemleak_skip_disable) {
    kmemleak_disable();
    return;
    }

    if (kmemleak_error) {
    return;
    }
    jiffies_min_age = msecs_to_jiffies(MSECS_MIN_AGE);
    jiffies_scan_wait = secs_to_jiffies(SECS_SCAN_WAIT);
    object_cache = KMEM_CACHE(kmemleak_object, SLAB_NOLEAKTRACE);
    scan_area_cache = KMEM_CACHE(kmemleak_scan_area, SLAB_NOLEAKTRACE);
// register the data/bss sections
    create_object((unsigned long)_sdata, _edata - _sdata,
    KMEMLEAK_GREY, GFP_ATOMIC);
    create_object((unsigned long)__bss_start, __bss_stop - __bss_start,
    KMEMLEAK_GREY, GFP_ATOMIC);
// only register .data..ro_after_init if not within .data
    if (&__start_ro_after_init < &_sdata || &__end_ro_after_init > &_edata) {
    create_object((unsigned long)__start_ro_after_init,
    __end_ro_after_init - __start_ro_after_init,
    KMEMLEAK_GREY, GFP_ATOMIC);
    }
    }
//
// Late initialization function.
//
#[no_mangle]
unsafe extern "C" fn kmemleak_late_init() -> c_int {
    kmemleak_late_initialized = 1;
    debugfs_create_file("kmemleak", 0644, core::ptr::null_mut(), core::ptr::null_mut(), &kmemleak_fops);
    if (kmemleak_error) {
//
// Some error occurred and kmemleak was disabled. There is a
// small chance that kmemleak_disable() was called immediately
// after setting kmemleak_late_initialized and we may end up with
// two clean-up threads but serialized by scan_mutex.
//
    schedule_work(&cleanup_work);
    return -ENOMEM;
    }
    if (IS_ENABLED!(CONFIG_DEBUG_KMEMLEAK_AUTO_SCAN)) {
    mutex_lock(&scan_mutex);
    start_scan_thread();
    mutex_unlock(&scan_mutex);
    }
    pr_info!("Kernel memory leak detector initialized (mem pool available: %d)\n",
    mem_pool_free_count);
    return 0;
    }
    late_initcall!(kmemleak_late_init);