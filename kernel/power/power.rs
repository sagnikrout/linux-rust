//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/power/power.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct swsusp_info {
    pub uts: new_utsname,
    pub version_code: u32,
    pub num_physpages: c_ulong,
    pub cpus: c_int,
    pub image_pages: c_ulong,
    pub pages: c_ulong,
    pub size: c_ulong,
    pub __aligned(PAGE_SIZE): },

    pub pm_sleep_fs_sync(void): extern int,
    pub filesystem_freeze_enabled: extern bool,

// kernel/power/snapshot.c
    pub hibernate_reserved_size_init(void): extern void __init,
    pub hibernate_image_size_init(void): extern void __init,

// Maximum size of architecture specific data in a hibernation header

    pub MAX_ARCH_HEADER_SIZE): return arch_hibernation_header_save(info,,
    pub NULL: "architecture specific data" :,

//
// Keep some memory free so that I/O operations can succeed without paging
// [Might this be more than 4 MB?]
//

//
// Keep 1 MB of memory free so that device drivers can allocate some pages in
// their .suspend() routines without breaking the suspend to disk.
//

    pub swsusp_save(void): asmlinkage int,
// kernel/power/hibernate.c
    pub freezer_test_done: extern bool,
    pub hib_comp_algo: [extern char; CRYPTO_MAX_ALG_NAME],
// kernel/power/swap.c
    pub swsusp_header_flags: extern unsigned int,
    pub platform_mode): extern int hibernation_snapshot(int,
    pub platform_mode): extern int hibernation_restore(int,
    pub hibernation_platform_enter(void): extern int,

// kernel/power/snapshot.c
    pub enable_restore_image_protection(void): extern void,

    pub hibernation_in_progress(void): extern bool,

    pub }: inline bool hibernation_in_progress(void) { return false;,

// Preferred image size in bytes (default 500 MB)
    pub image_size: extern unsigned long,
Size of memory reserved for drivers (default SPARE_PAGES x PAGE_SIZE)
    pub reserved_size: extern unsigned long,
    pub in_suspend: extern int,
    pub swsusp_resume_device: extern dev_t,
    pub swsusp_resume_block: extern sector_t,
    pub create_basic_memory_bitmaps(void): extern int,
    pub free_basic_memory_bitmaps(void): extern void,
    pub hibernate_preallocate_memory(void): extern int,
    pub clear_or_poison_free_pages(void): extern void,
//
// Auxiliary structure used for reading the snapshot image data and
// metadata from and writing them to the list of page backup entries
// (PBEs) which is the main data structure of swsusp.
//
// Using struct snapshot_handle we can transfer the image, including its
// metadata, as a continuous sequence of bytes with the help of
// snapshot_read_next() and snapshot_write_next().
//
// The code that writes the image to a storage or transfers it to
// the user land is required to use snapshot_read_next() for this
// purpose and it should not make any assumptions regarding the internal
// structure of the image.  Similarly, the code that reads the image from
// a storage or transfers it from the user land is required to use
// snapshot_write_next().
//
// This may allow us to change the internal structure of the image
// in the future with considerably less effort.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snapshot_handle {
    pub the: *mut *mut unsigned int cur; / number of the block of PAGE_SIZE bytes,
next operation will refer to (ie. current)
//
    pub from: *mut *mut *mut c_void buffer; / address of the block to read,
// or write to
//
    pub of: *mut *mut int sync_read; / Set to one to notify the caller,
// snapshot_write_next() that it may
// need to call wait_on_bio_chain()
//
}

// This macro returns the address from/to which the caller of
// snapshot_read_next()/snapshot_write_next() is allowed to
// read/write data after the function returns
//

extern "C" {
    pub fn snapshot_additional_pages(zone: *mut zone) -> c_uint;
}
extern "C" {
    pub fn snapshot_get_image_size() -> c_ulong;
}
extern "C" {
    pub fn snapshot_read_next(handle: *mut snapshot_handle) -> c_int;
}
extern "C" {
    pub fn snapshot_write_next(handle: *mut snapshot_handle) -> c_int;
}
extern "C" {
    pub fn snapshot_write_finalize(handle: *mut snapshot_handle) -> c_int;
}
extern "C" {
    pub fn snapshot_image_loaded(handle: *mut snapshot_handle) -> c_int;
}
extern "C" {
    pub fn hibernate_acquire() -> bool;
}
extern "C" {
    pub fn hibernate_release();
}
extern "C" {
    pub fn alloc_swapdev_block(swap: c_int) -> sector_t;
}
extern "C" {
    pub fn free_all_swap_pages(swap: c_int);
}
extern "C" {
    pub fn swsusp_swap_in_use() -> c_int;
}
//
// Flags that can be passed from the hibernatig hernel to the "boot" kernel in
// the image header.
//

pub const SF_PLATFORM_MODE: c_int = 1;
pub const SF_NOCOMPRESS_MODE: c_int = 2;
pub const SF_CRC32_MODE: c_int = 4;
pub const SF_HW_SIG: c_int = 8;
//
// Bit to indicate the compression algorithm to be used(for LZ4). The same
// could be checked while saving/loading image to/from disk to use the
// corresponding algorithms.
//
// By default, LZO compression is enabled if SF_CRC32_MODE is set. Use
// SF_COMPRESSION_ALG_LZ4 to override this behaviour and use LZ4.
//
// SF_CRC32_MODE, SF_COMPRESSION_ALG_LZO(dummy) -> Compression, LZO
// SF_CRC32_MODE, SF_COMPRESSION_ALG_LZ4 -> Compression, LZ4
//
pub const SF_COMPRESSION_ALG_LZ4: c_int = 16;
// kernel/power/hibernate.c
extern "C" {
    pub fn swsusp_check(exclusive: bool) -> c_int;
}
extern "C" {
    pub fn swsusp_free();
}
extern "C" {
    pub fn swsusp_read(flags_p: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn swsusp_write(flags: c_uint) -> c_int;
}
extern "C" {
    pub fn swsusp_close();
}

extern "C" {
    pub fn swsusp_unmark() -> c_int;
}

// kernel/power/swsusp.c
extern "C" {
    pub fn swsusp_show_speed(_arg: ktime_t, _arg: ktime_t, int: unsigned, : *mut c_char);
}

// kernel/power/suspend.c
extern "C" {
    pub fn suspend_devices_and_enter(state: suspend_state_t) -> c_int;
}

// kernel/power/suspend_test.c
extern "C" {
    pub fn suspend_test_start();
}
extern "C" {
    pub fn suspend_test_finish(label: *const c_char);
}

// kernel/power/main.c
extern "C" {
    pub fn pm_notifier_call_chain_robust(val_up: c_ulong, val_down: c_ulong) -> c_int;
}
extern "C" {
    pub fn pm_notifier_call_chain(val: c_ulong) -> c_int;
}

extern "C" {
    pub fn restore_highmem() -> c_int;
}

//
// Suspend test levels
//
// keep first
// keep last

//
// freeze_processes() automatically thaws every task if freezing
// fails. So we need not do anything extra upon error.
//
// freeze_kernel_threads() thaws only kernel threads upon freezing
// failure. So we have to thaw the userspace tasks ourselves.
//

// kernel/power/autosleep.c
extern "C" {
    pub fn pm_autosleep_init() -> c_int;
}
extern "C" {
    pub fn pm_autosleep_lock() -> c_int;
}
extern "C" {
    pub fn pm_autosleep_unlock();
}
extern "C" {
    pub fn pm_autosleep_state() -> suspend_state_t;
}
extern "C" {
    pub fn pm_autosleep_set_state(state: suspend_state_t) -> c_int;
}

// kernel/power/wakelock.c
extern "C" {
    pub fn pm_show_wakelocks(buf: *mut c_char, show_active: bool) -> isize;
}
extern "C" {
    pub fn pm_wake_lock(buf: *const c_char) -> c_int;
}
extern "C" {
    pub fn pm_wake_unlock(buf: *const c_char) -> c_int;
}

extern "C" {
    pub fn suspend_disable_secondary_cpus() -> return;
}
extern "C" {
    pub fn dpm_save_errno(err: c_int);
}