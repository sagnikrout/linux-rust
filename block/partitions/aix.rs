//! Automatically rewritten from C to Rust
//! Source: block/partitions/aix.c
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
// fs/partitions/aix.c
//
// Copyright (C) 2012-2013 Philippe De Muyter <phdm@macqel.be>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lvm_rec {
//     pub /: *mut *mut char lvm_id[4]; / "_LVM",
    pub reserved4: [c_char; 16],
    pub lvmarea_len: __be32,
    pub vgda_len: __be32,
    pub vgda_psn: [__be32; 2],
    pub reserved36: [c_char; 10],
//     pub /: *mut *mut __be16 pp_size; / log2(pp_size),
    pub reserved46: [c_char; 12],
    pub version: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgda {
    pub secs: __be32,
    pub usec: __be32,
    pub reserved8: [c_char; 16],
    pub numlvs: __be16,
    pub maxlvs: __be16,
    pub pp_size: __be16,
    pub numpvs: __be16,
    pub total_vgdas: __be16,
    pub vgda_size: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lvd {
    pub lv_ix: __be16,
    pub res2: __be16,
    pub res4: __be16,
    pub maxsize: __be16,
    pub lv_state: __be16,
    pub mirror: __be16,
    pub mirror_policy: __be16,
    pub num_lps: __be16,
    pub res10: [__be16; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lvname {
    pub name: [c_char; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppe {
    pub lv_ix: __be16,
    pub res2: c_ushort,
    pub res4: c_ushort,
    pub lp_ix: __be16,
    pub res8: [c_ushort; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvd {
    pub reserved0: [c_char; 16],
    pub pp_count: __be16,
    pub reserved18: [c_char; 2],
    pub psn_part1: __be32,
    pub reserved24: [c_char; 8],
    pub ppe: [ppe; 1016],
}

pub const LVM_MAXLVS: c_int = 256;
//
// read_lba(): Read bytes from disk, starting at given LBA
// @state
// @lba
// @buffer
// @count
//
// Description:  Reads @count bytes from @state->disk into @buffer.
// Returns number of bytes read on success, 0 on error.
//
#[no_mangle]
pub unsafe extern "C" fn read_lba(state: *mut parsed_partitions, lba: u64, buffer: *mut u8, count: size_t) -> size_t {
pub static mut totalreadcount: usize = 0;
    if (!buffer || lba + count / 512 > get_capacity(state.disk) - 1ULL) {
    return 0;
    }
    while (count) {
pub static mut copied: c_int = 512;
    let mut sect;
    let mut data = read_part_sector(state, lba++, &sect);
    if (!data) {
    break;
    }
    if (copied > count) {
    copied = count;
    }
    memcpy(buffer, data, copied);
    put_dev_sector(sect);
    buffer += copied;
    totalreadcount += copied;
    count -= copied;
    }
    return totalreadcount;
    }
//
// alloc_pvd(): reads physical volume descriptor
// @state
// @lba
//
// Description: Returns pvd on success,  NULL on error.
// Allocates space for pvd and fill it with disk blocks at @lba
// Notes: remember to free pvd when you're done!
//
#[no_mangle]
pub unsafe extern "C" fn alloc_pvd(state: *mut parsed_partitions, lba: u32) -> *mut c_void {
pub static mut count: usize = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = kmalloc(count, GFP_KERNEL);
    if (!p) {
    return core::ptr::null_mut();
    }
    if (read_lba(state, lba,  p, count) < count) {
    kfree(p);
    return core::ptr::null_mut();
    }
    return p;
    }
//
// alloc_lvn(): reads logical volume names
// @state
// @lba
//
// Description: Returns lvn on success,  NULL on error.
// Allocates space for lvn and fill it with disk blocks at @lba
// Notes: remember to free lvn when you're done!
//
#[no_mangle]
pub unsafe extern "C" fn alloc_lvn(state: *mut parsed_partitions, lba: u32) -> *mut c_void {
pub static mut count: usize = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = kmalloc(count, GFP_KERNEL);
    if (!p) {
    return core::ptr::null_mut();
    }
    if (read_lba(state, lba,  p, count) < count) {
    kfree(p);
    return core::ptr::null_mut();
    }
    return p;
    }
#[no_mangle]
pub unsafe extern "C" fn aix_partition(state: *mut parsed_partitions) -> c_int {
pub static mut ret: c_int = 0;
    let mut sect;
pub static mut d: *mut c_void = core::ptr::null_mut();
    let mut pp_bytes_size = 0;
pub static mut pp_blocks_size: u32 = 0;
pub static mut vgda_sector: u32 = 0;
pub static mut vgda_len: u32 = 0;
pub static mut numlvs: c_int = 0;
    let mut pvd = core::ptr::null_mut();
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lv_info {
    pub pps_per_lv: c_ushort,
    pub pps_found: c_ushort,
    pub lv_is_contiguous: c_uchar,
    pub lvip: *mut },
    pub NULL: *mut *mut lvname n =,
    pub &sect): d = read_part_sector(state, 7,,
    if (d) {
    pub )d: *mut *mut lvm_rec p = (lvm_rec,
    pub be16_to_cpu(p->version): u16 lvm_version =,
    if (lvm_version == 1) {
    pub be16_to_cpu(p->pp_size): int pp_size_log2 =,
    pub pp_size_log2: pp_bytes_size = 1 <<,
    pub 512: pp_blocks_size = pp_bytes_size /,
    seq_buf_printf(&state.pp_buf,
    " AIX LVM header version %u found\n",
    pub be32_to_cpu(p->vgda_len): vgda_len =,
    pub be32_to_cpu(p->vgda_psn[0]): vgda_sector =,
    } else {
    seq_buf_printf(&state.pp_buf,
    " unsupported AIX LVM version %d found\n",
    }
    }
    if (vgda_sector && (d = read_part_sector(state, vgda_sector, &sect))) {
    pub )d: *mut *mut vgda p = (vgda,
    pub be16_to_cpu(p->numlvs): numlvs =,
    }
    pub state->limit): lvip = kzalloc_objs(lv_info,,
    if (!lvip) {
    pub 0: return,
    if (numlvs && (d = read_part_sector(state, vgda_sector + 1, &sect))) {
    }
    pub )d: *mut *mut lvd p = (lvd,
    pub i: c_int,
    pub 33): n = alloc_lvn(state, vgda_sector + vgda_len -,
    if (n) {
    pub 0: int foundlvs =,
//
// The lvd array was read as a single sector; only the
// struct lvd entries that fit in it are valid.  Bound the
// scan so an on-disk numlvs larger than that cannot walk
// the read buffer out of bounds.
//
    pub &&: for (i = 0; foundlvs < numlvs && i < state->limit,
    pub {: i < SECTOR_SIZE / (int)sizeof!(lvd); i++),
    pub be16_to_cpu(p[i].num_lps): lvip[i].pps_per_lv =,
    if (lvip[i].pps_per_lv) {
    pub 1: foundlvs +=,
    }
// pvd loops depend on n[].name and lvip[].pps_per_lv
    pub 17): pvd = alloc_pvd(state, vgda_sector +,
    }
    }
    if (pvd) {
    }
    pub be16_to_cpu(pvd->pp_count): int numpps =,
    pub be32_to_cpu(pvd->psn_part1): int psn_part1 =,
    pub i: c_int,
    pub -1: int cur_lv_ix =,
    pub 1: int next_lp_ix =,
    pub lp_ix: c_int,
//
// pvd was read into a fixed-size struct pvd whose ppe[] array
// holds ARRAY_SIZE!(pvd->ppe) entries.  pp_count is an
// unvalidated on-disk __be16, so clamp the scan to the array
// size to avoid walking past the allocation.
//
    if (numpps > ARRAY_SIZE!(pvd.ppe)) {
    pub ARRAY_SIZE!(pvd->ppe): numpps =,
    pub {: for (i = 0; i < numpps; i += 1),
    }
    pub i: *mut *mut ppe p = pvd->ppe +,
    pub lv_ix: c_uint,
    pub be16_to_cpu(p->lp_ix): lp_ix =,
    if (!lp_ix) {
    pub 1: next_lp_ix =,
    }
    pub 1: lv_ix = be16_to_cpu(p->lv_ix) -,
    if (lv_ix >= state.limit) {
    pub -1: cur_lv_ix =,
    }
    pub 1: lvip[lv_ix].pps_found +=,
    if (lp_ix == 1) {
    pub lv_ix: cur_lv_ix =,
    pub 1: next_lp_ix =,
    } else if (lv_ix != cur_lv_ix || lp_ix != next_lp_ix) {
    pub 1: next_lp_ix =,
    }
    if (lp_ix == lvip[lv_ix].pps_per_lv) {
    put_partition(state, lv_ix + 1,
    (i + 1 - lp_ix) * pp_blocks_size + psn_part1,
    pub pp_blocks_size): *mut *mut lvip[lv_ix].pps_per_lv,
    seq_buf_printf(&state.pp_buf, " <%s>\n",
    pub 1: lvip[lv_ix].lv_is_contiguous =,
    pub 1: ret =,
    pub 1: next_lp_ix =,
    } else {
    pub 1: next_lp_ix +=,
    }
    pub 1): while (i < state->limit) {
    }
    pub char: char tmp[sizeof!(n[i].name) + 1]; // null,
    pub n[i].name): snprintf(tmp, sizeof!(tmp), "%s",,
    pr_warn!("partition %s (%u pp's found) is "
    "not contiguous\n",
    pub lvip[i].pps_found): tmp,,
    }
    }
    pub ret: return,
    }