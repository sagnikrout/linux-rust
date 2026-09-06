//! Automatically rewritten from C to Rust
//! Source: block/partitions/sun.c
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
// fs/partitions/sun.c
//
// Code extracted from drivers/block/genhd.c
//
// Copyright (C) 1991-1998  Linus Torvalds
// Re-organised Feb 1998 Russell King
//

pub const SUN_LABEL_MAGIC: c_uint = 0xDABE;
pub const SUN_VTOC_SANITY: c_uint = 0x600DDEEE;
    enum {
    SUN_WHOLE_DISK = 5,
    LINUX_RAID_PARTITION = 0xfd,	/* autodetect RAID partition */
    };
#[no_mangle]
pub unsafe extern "C" fn sun_partition(state: *mut parsed_partitions) -> c_int {
    let mut i = 0;
    let mut csum;
pub static mut slot: c_int = 1;
pub static mut ush: *mut c_void = core::ptr::null_mut();
    let mut sect;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun_disklabel {
//     pub /: *mut *mut unsigned char info[128]; / Informative text string,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun_vtoc {
//     pub /: *mut *mut __be32 version; / Layout version,
//     pub /: *mut *mut char volume[8]; / Volume name,
//     pub /: *mut *mut __be16 nparts; / Number of partitions,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun_info {
    pub id: __be16,
    pub flags: __be16,
    pub infos: [}; 8],
//     pub /: *mut *mut __be16 padding; / Alignment padding,
//     pub /: *mut *mut __be32 bootinfo[3]; / Info needed by mboot,
//     pub /: *mut *mut __be32 sanity; / To verify vtoc sanity,
//     pub /: *mut *mut __be32 reserved[10]; / Free space,
//     pub /: *mut *mut __be32 timestamp[8]; / Partition timestamp,
    pub vtoc: },
//     pub /: *mut *mut __be32 write_reinstruct; / sectors to skip, writes,
//     pub /: *mut *mut __be32 read_reinstruct; / sectors to skip, reads,
//     pub /: *mut *mut unsigned char spare[148]; / Padding,
//     pub /: *mut *mut __be16 rspeed; / Disk rotational speed,
//     pub /: *mut *mut __be16 pcylcount; / Physical cylinder count,
//     pub /: *mut *mut __be16 sparecyl; / extra sects per cylinder,
//     pub /: *mut *mut __be16 obs1; / gap1,
//     pub /: *mut *mut __be16 obs2; / gap2,
//     pub /: *mut *mut __be16 ilfact; / Interleave factor,
//     pub /: *mut *mut __be16 ncyl; / Data cylinder count,
//     pub /: *mut *mut __be16 nacyl; / Alt. cylinder count,
//     pub /: *mut *mut __be16 ntrks; / Tracks per cylinder,
//     pub /: *mut *mut __be16 nsect; / Sectors per track,
//     pub /: *mut *mut __be16 obs3; / bhead - Label head offset,
//     pub /: *mut *mut __be16 obs4; / ppart - Physical Partition,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun_partition {
    pub start_cylinder: __be32,
    pub num_sectors: __be32,
    pub partitions: [}; 8],
//     pub /: *mut *mut __be16 magic; / Magic number,
//     pub /: *mut *mut __be16 csum; / Label xor'd checksum,
    pub label: *mut *mut },
    pub p: *mut sun_partition,
    pub spc: c_ulong,
    pub use_vtoc: c_int,
    pub nparts: c_int,
    pub &sect): label = read_part_sector(state, 0,,
    if (!label) {
    pub -1: return,
    pub label->partitions: p =,
    if (be16_to_cpu(label.magic) != SUN_LABEL_MAGIC) {
    }
    pub 0: return,
    }
// Look at the checksum
    pub 1: *mut *mut ush = ((__be16 ) (label+1)) -,
    pub label);): *mut *mut for (csum = 0; ush >= ((__be16 ),
    pub ush--: *mut csum ^=,
    if (csum) {
    printk("Dev %s Sun disklabel: Csum bad, label corrupted\n",
    pub 0: return,
    }
// Check to see if we can use the VTOC table
    use_vtoc = ((be32_to_cpu(label.vtoc.sanity) == SUN_VTOC_SANITY) &&
    (be32_to_cpu(label.vtoc.version) == 1) &&
    pub 8)): (be16_to_cpu(label->vtoc.nparts) <=,
// Use 8 partition entries if not specified in validated VTOC
    pub 8: nparts = (use_vtoc) ? be16_to_cpu(label->vtoc.nparts) :,
//
// So that old Linux-Sun partitions continue to work,
// alow the VTOC to be used under the additional condition ...
//
    use_vtoc = use_vtoc || !(label.vtoc.sanity ||
    pub label->vtoc.nparts): label->vtoc.version ||,
    pub be16_to_cpu(label->nsect): *mut *mut spc = be16_to_cpu(label->ntrks),
    pub {: for (i = 0; i < nparts; i++, p++),
    pub st_sector: c_ulong,
    pub num_sectors: c_uint,
    pub spc: *mut *mut st_sector = be32_to_cpu(p->start_cylinder),
    pub be32_to_cpu(p->num_sectors): num_sectors =,
    if (num_sectors) {
    pub num_sectors): put_partition(state, slot, st_sector,,
    pub 0: state->parts[slot].flags =,
    if (use_vtoc) {
    if (be16_to_cpu(label.vtoc.infos[i].id) == LINUX_RAID_PARTITION) {
    pub ADDPART_FLAG_RAID: state->parts[slot].flags |=,

    else if (be16_to_cpu(label.vtoc.infos[i].id) == SUN_WHOLE_DISK)
    pub ADDPART_FLAG_WHOLEDISK: state->parts[slot].flags |=,
    }
    }
    }
    pub "\n"): seq_buf_puts(&state->pp_buf,,
    pub 1: return,
    }
    }