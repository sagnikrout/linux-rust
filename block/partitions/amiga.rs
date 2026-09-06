//! Automatically rewritten from C to Rust
//! Source: block/partitions/amiga.c
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
// fs/partitions/amiga.c
//
// Code extracted from drivers/block/genhd.c
//
// Copyright (C) 1991-1998  Linus Torvalds
// Re-organised Feb 1998 Russell King
//

// magic offsets in partition DosEnvVec
pub const NR_HD: c_int = 3;
pub const NR_SECT: c_int = 5;
pub const LO_CYL: c_int = 9;
pub const HI_CYL: c_int = 10;
    static __inline__ u32
    checksum_block(__be32 *m, int size)
    {
pub static mut sum: u32 = 0;
    while (size--) {
    sum += be32_to_cpu(*m++);
    }
    return sum;
    }
#[no_mangle]
pub unsafe extern "C" fn amiga_partition(state: *mut parsed_partitions) -> c_int {
    let mut sect;
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut rdb: *mut c_void = core::ptr::null_mut();
pub static mut pb: *mut c_void = core::ptr::null_mut();
    u64 start_sect, nr_sects;
    sector_t blk, end_sect;
    let mut cylblk = 0;		/* rdb_CylBlocks = nr_heads*sect_per_track */
    u32 nr_hd, nr_sect, lo_cyl, hi_cyl;
    int part, res = 0;
    let mut blksize = 1;	/* Multiplier for disk block size */
pub static mut slot: c_int = 1;
    for (blk = 0; ; blk++, put_dev_sector(sect)) {
    if (blk == RDB_ALLOCATION_LIMIT) {
// goto;
    }
    data = read_part_sector(state, blk, &sect);
    if (!data) {
    pr_err!("Dev %s: unable to read RDB block %llu\n",
    state.disk.disk_name, blk);
    res = -1;
// goto;
    }
    if (*data != cpu_to_be32(IDNAME_RIGIDDISK)) {
    continue;
    }
    rdb = data;
    if (checksum_block(data, be32_to_cpu(rdb.rdb_SummedLongs) & 0x7F) == 0) {
    break;
    }
// Try again with 0xdc..0xdf zeroed, Windows might have
// trashed it.
//
// (data+0xdc) = 0;
    if (checksum_block(data,
    be32_to_cpu(rdb.rdb_SummedLongs) & 0x7F)==0) {
    pr_err!("Trashed word at 0xd0 in block %llu ignored in checksum calculation\n",
    blk);
    break;
    }
    pr_err!("Dev %s: RDB in block %llu has bad checksum\n",
    state.disk.disk_name, blk);
    }
// blksize is blocks per 512 byte standard block
    blksize = be32_to_cpu( rdb.rdb_BlockBytes ) / 512;
// Be more informative
    seq_buf_printf(&state.pp_buf, " RDSK (%d)", blksize * 512);
    blk = be32_to_cpu(rdb.rdb_PartitionList);
    put_dev_sector(sect);
    for (part = 1; (s32) blk>0 && part<=16; part++, put_dev_sector(sect)) {
// Read in terms partition table understands
    if (check_mul_overflow(blk, (sector_t) blksize, &blk)) {
    pr_err!("Dev %s: overflow calculating partition block %llu! Skipping partitions %u and beyond\n",
    state.disk.disk_name, blk, part);
    break;
    }
    data = read_part_sector(state, blk, &sect);
    if (!data) {
    pr_err!("Dev %s: unable to read partition block %llu\n",
    state.disk.disk_name, blk);
    res = -1;
// goto;
    }
    pb  = data;
    blk = be32_to_cpu(pb.pb_Next);
    if (pb.pb_ID != cpu_to_be32(IDNAME_PARTITION)) {
    continue;
    }
    if (checksum_block(pb, be32_to_cpu(pb.pb_SummedLongs) & 0x7F) != 0 ) {
    continue;
    }
// RDB gives us more than enough rope to hang ourselves with,
// many times over (2^128 bytes if all fields max out).
// Some careful checks are in order, so check for potential
// overflows.
// We are multiplying four 32 bit numbers to one sector_t!
//
    nr_hd   = be32_to_cpu(pb.pb_Environment[NR_HD]);
    nr_sect = be32_to_cpu(pb.pb_Environment[NR_SECT]);
// CylBlocks is total number of blocks per cylinder
    if (check_mul_overflow(nr_hd, nr_sect, &cylblk)) {
    pr_err!("Dev %s: heads*sects %u overflows u32, skipping partition!\n",
    state.disk.disk_name, cylblk);
    continue;
    }
// check for consistency with RDB defined CylBlocks
    if (cylblk > be32_to_cpu(rdb.rdb_CylBlocks)) {
    pr_warn!("Dev %s: cylblk %u > rdb_CylBlocks %u!\n",
    state.disk.disk_name, cylblk,
    be32_to_cpu(rdb.rdb_CylBlocks));
    }
// RDB allows for variable logical block size -
// normalize to 512 byte blocks and check result.
//
    if (check_mul_overflow(cylblk, blksize, &cylblk)) {
    pr_err!("Dev %s: partition %u bytes per cyl. overflows u32, skipping partition!\n",
    state.disk.disk_name, part);
    continue;
    }
// Calculate partition start and end. Limit of 32 bit on cylblk
// guarantees no overflow occurs if LBD support is enabled.
//
    lo_cyl = be32_to_cpu(pb.pb_Environment[LO_CYL]);
    start_sect = ((u64) lo_cyl * cylblk);
    hi_cyl = be32_to_cpu(pb.pb_Environment[HI_CYL]);
    nr_sects = (((u64) hi_cyl - lo_cyl + 1) * cylblk);
    if (!nr_sects) {
    continue;
    }
// Warn user if partition end overflows u32 (AmigaDOS limit)
    if ((start_sect + nr_sects) > UINT_MAX) {
    pr_warn!("Dev %s: partition %u (%llu-%llu) needs 64 bit device support!\n",
    state.disk.disk_name, part,
    start_sect, start_sect + nr_sects);
    }
    if (check_add_overflow(start_sect, nr_sects, &end_sect)) {
    pr_err!("Dev %s: partition %u (%llu-%llu) needs LBD device support, skipping partition!\n",
    state.disk.disk_name, part,
    start_sect, end_sect);
    continue;
    }
// Tell Kernel about it
    put_partition(state,slot++,start_sect,nr_sects);
    {
// Be even more informative to aid mounting
    char dostype[4];
    let mut dt = dostype;
// dt = pb->pb_Environment[16];
    if (dostype[3] < ' ') {
    seq_buf_printf(&state.pp_buf,
    " (%c%c%c^%c)",
    dostype[0], dostype[1],
    dostype[2],
    dostype[3] + '@');
    }
    else {
    seq_buf_printf(&state.pp_buf,
    " (%c%c%c%c)",
    dostype[0], dostype[1],
    dostype[2], dostype[3]);
    }
    seq_buf_printf(&state.pp_buf, "(res %d spb %d)",
    be32_to_cpu(pb.pb_Environment[6]),
    be32_to_cpu(pb.pb_Environment[4]));
    }
    res = 1;
    }
    seq_buf_puts(&state.pp_buf, "\n");
// label;
    return res;
    }