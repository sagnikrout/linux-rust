//! Automatically rewritten from C to Rust
//! Source: block/partitions/msdos.c
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
// fs/partitions/msdos.c
//
// Code extracted from drivers/block/genhd.c
// Copyright (C) 1991-1998  Linus Torvalds
//
// Thanks to Branko Lankester, lankeste@fwi.uva.nl, who found a bug
// in the early extended-partition checks and added DM partitions
//
// Support for DiskManager v6.0x added by Mark Lord,
// with information provided by OnTrack.  This now works for linux fdisk
// and LILO, as well as loadlin and bootln.  Note that disks other than
// /dev/hda *must* have a "DOS" type 0x51 partition in the first slot (hda1).
//
// More flexible handling of extended partitions - aeb, 950831
//
// Check partition table on IDE disks for common CHS translations
//
// Re-organised Feb 1998 Russell King
//
// BSD disklabel support by Yossi Gottlieb <yogo@math.tau.ac.il>
// updated by Marc Espie <Marc.Espie@openbsd.org>
//
// Unixware slices support by Andrzej Krzysztofowicz <ankry@mif.pg.gda.pl>
// and Krzysztof G. Baranowski <kgb@knm.org.pl>
//

//
// Many architectures don't like unaligned accesses, while
// the nr_sects and start_sect partition table entries are
// at a 2 (mod 4) address.
//

#[no_mangle]
pub unsafe extern "C" fn nr_sects(p: *mut msdos_partition) -> sector_t {
    return (sector_t)get_unaligned_le32(&p.nr_sects);
    }
#[no_mangle]
pub unsafe extern "C" fn start_sect(p: *mut msdos_partition) -> sector_t {
    return (sector_t)get_unaligned_le32(&p.start_sect);
    }
#[no_mangle]
pub unsafe extern "C" fn is_extended_partition(p: *mut msdos_partition) -> c_int {
    return (p.sys_ind == DOS_EXTENDED_PARTITION ||
    p.sys_ind == WIN98_EXTENDED_PARTITION ||
    p.sys_ind == LINUX_EXTENDED_PARTITION);
    }
pub const MSDOS_LABEL_MAGIC1: c_uint = 0x55;
pub const MSDOS_LABEL_MAGIC2: c_uint = 0xAA;
#[no_mangle]
pub unsafe extern "C" fn msdos_magic_present(p: *mut c_uchar) -> c_int {
    return (p[0] == MSDOS_LABEL_MAGIC1 && p[1] == MSDOS_LABEL_MAGIC2);
    }
// Value is EBCDIC 'IBMA'
pub const AIX_LABEL_MAGIC1: c_uint = 0xC9;
pub const AIX_LABEL_MAGIC2: c_uint = 0xC2;
pub const AIX_LABEL_MAGIC3: c_uint = 0xD4;
pub const AIX_LABEL_MAGIC4: c_uint = 0xC1;
#[no_mangle]
unsafe extern "C" fn aix_magic_present(state: *mut parsed_partitions, p: *mut c_uchar) -> c_int {
    let mut pt =  (p + 0x1be);
    let mut sect;
pub static mut d: *mut c_void = core::ptr::null_mut();
    int slot, ret = 0;
    if (!(p[0] == AIX_LABEL_MAGIC1 &&
    p[1] == AIX_LABEL_MAGIC2 &&
    p[2] == AIX_LABEL_MAGIC3 &&
    p[3] == AIX_LABEL_MAGIC4)) {
    return 0;
    }
//
// Assume the partition table is valid if Linux partitions exists.
// Note that old Solaris/x86 partitions use the same indicator as
// Linux swap partitions, so we consider that a Linux partition as
// well.
//
    while (slot <= 4) {
    if (pt.sys_ind == SOLARIS_X86_PARTITION ||
    pt.sys_ind == LINUX_RAID_PARTITION ||
    pt.sys_ind == LINUX_DATA_PARTITION ||
    pt.sys_ind == LINUX_LVM_PARTITION ||
    is_extended_partition(pt)) {
    return 0;
    }
    }
    d = read_part_sector(state, 7, &sect);
    if (d) {
    if (d[0] == '_' && d[1] == 'L' && d[2] == 'V' && d[3] == 'M') {
    ret = 1;
    }
    put_dev_sector(sect);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn set_info(state: *mut parsed_partitions, slot: c_int, disksig: u32) {
    let mut info = &state.parts[slot].info;
    snprintf(info.uuid, sizeof!(info.uuid), "%08x-%02x", disksig,
    slot);
    info.volname[0] = 0;
    state.parts[slot].has_info = true;
    }
//
// Create devices for each logical partition in an extended partition.
// The logical partitions form a linked list, with each entry being
// a partition table with two entries.  The first entry
// is the real data partition (with a start relative to the partition
// table start).  The second is a pointer to the next logical partition
// (with a start relative to the entire extended partition).
// We do not create a Linux partition for the partition tables, but
// only for the actual data partitions.
//
#[no_mangle]
pub unsafe extern "C" fn parse_extended(state: *mut parsed_partitions, first_sector: sector_t, first_size: sector_t, disksig: u32) {
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut sect;
pub static mut data: *mut c_void = core::ptr::null_mut();
    sector_t this_sector, this_size;
    let mut sector_size;
    let mut loopct = 0;		// number of links followed
    without finding a data partition */
    let mut i = 0;
    sector_size = queue_logical_block_size(state.disk.queue) / 512;
    this_sector = first_sector;
    this_size = first_size;
    while (1) {
    if (++loopct > 100) {
    return;
    }
    if (state.next == state.limit) {
    return;
    }
    data = read_part_sector(state, this_sector, &sect);
    if (!data) {
    return;
    }
    if (!msdos_magic_present(data + 510)) {
// goto;
    }
    p =  (data + 0x1be);
//
// Usually, the first entry is the real data partition,
// the 2nd entry is the next extended partition, or empty,
// and the 3rd and 4th entries are unused.
// However, DRDOS sometimes has the extended partition as
// the first entry (when the data partition is empty),
// and OS/2 seems to use all four entries.
//
// First process the data partition(s)
//
    while (i < 4) {
    sector_t offs, size, next;
    if (!nr_sects(p) || is_extended_partition(p)) {
    continue;
    }
// Check the 3rd and 4th entries -
    these sometimes contain random garbage */
    offs = start_sect(p)*sector_size;
    size = nr_sects(p)*sector_size;
    next = this_sector + offs;
    if (i >= 2) {
    if (offs + size > this_size) {
    continue;
    }
    if (next < first_sector) {
    continue;
    }
    if (next + size > first_sector + first_size) {
    continue;
    }
    }
    put_partition(state, state.next, next, size);
    set_info(state, state.next, disksig);
    if (p.sys_ind == LINUX_RAID_PARTITION) {
    state.parts[state.next].flags = ADDPART_FLAG_RAID;
    }
    loopct = 0;
    if (++state.next == state.limit) {
// goto;
    }
    }
//
// Next, process the (first) extended partition, if present.
// (So far, there seems to be no reason to make
// parse_extended()  recursive and allow a tree
// of extended partitions.)
// It should be a link to the next logical partition.
//
    p -= 4;
    for (i = 0; i < 4; i++, p++) {
    if (nr_sects(p) && is_extended_partition(p))
    break;
    }
    if (i == 4) {
// goto;	 /* nothing left to do */
    }
    this_sector = first_sector + start_sect(p) * sector_size;
    this_size = nr_sects(p) * sector_size;
    put_dev_sector(sect);
    }
// label;
    put_dev_sector(sect);
    }
pub const SOLARIS_X86_NUMSLICE: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct solaris_x86_slice {
//     pub /: *mut *mut __le16 s_tag; / ID tag of partition,
//     pub /: *mut *mut __le16 s_flag; / permission flags,
//     pub /: *mut *mut __le32 s_start; / start sector no of partition,
//     pub /: *mut *mut __le32 s_size; / # of blocks in partition,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct solaris_x86_vtoc {
//     pub /: *mut *mut unsigned int v_bootinfo[3]; / info needed by mboot,
//     pub /: *mut *mut __le32 v_sanity; / to verify vtoc sanity,
//     pub /: *mut *mut __le32 v_version; / layout version,
//     pub /: *mut *mut char v_volume[8]; / volume name,
//     pub /: *mut *mut __le16 v_sectorsz; / sector size in bytes,
//     pub /: *mut *mut __le16 v_nparts; / number of partitions,
//     pub /: *mut *mut unsigned int v_reserved[10]; / free space, solaris_x86_slice
//     pub /: *mut *mut v_slice[SOLARIS_X86_NUMSLICE]; / slice headers,
//     pub /: *mut *mut unsigned int timestamp[SOLARIS_X86_NUMSLICE]; / timestamp,
//     pub /: *mut *mut char v_asciilabel[128]; / for compatibility,
}

// james@bpgc.com: Solaris has a nasty indicator: 0x82 which also
    indicates linux swap.  Be careful before believing this is Solaris. */
#[no_mangle]
pub unsafe extern "C" fn parse_solaris_x86(state: *mut parsed_partitions, offset: sector_t, size: sector_t, origin: c_int) {

    let mut sect;
pub static mut v: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut max_nparts = 0;
    v = read_part_sector(state, offset + 1, &sect);
    if (!v) {
    return;
    }
    if (le32_to_cpu(v.v_sanity) != SOLARIS_X86_VTOC_SANE) {
    put_dev_sector(sect);
    return;
    }
    seq_buf_printf(&state.pp_buf, " %s%d: <solaris:", state.name, origin);
    if (le32_to_cpu(v.v_version) != 1) {
    seq_buf_printf(&state.pp_buf,
    "  cannot handle version %d vtoc>\n",
    le32_to_cpu(v.v_version));
    put_dev_sector(sect);
    return;
    }
// Ensure we can handle previous case of VTOC with 8 entries gracefully
    max_nparts = le16_to_cpu(v.v_nparts) > 8 ? SOLARIS_X86_NUMSLICE : 8;
    while (i < max_nparts && state.next < state.limit) {
    let mut s = &v.v_slice[i];
    if (s.s_size == 0) {
    continue;
    }
    seq_buf_printf(&state.pp_buf, " [s%d]", i);
// solaris partitions are relative to current MS-DOS
// one; must add the offset of the current partition
    put_partition(state, state.next++,
    le32_to_cpu(s.s_start)+offset,
    le32_to_cpu(s.s_size));
    }
    put_dev_sector(sect);
    seq_buf_puts(&state.pp_buf, " >\n");

    }
// check against BSD src/sys/sys/disklabel.h for consistency

pub const BSD_MAXPARTITIONS: c_int = 16;
pub const OPENBSD_MAXPARTITIONS: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsd_disklabel {
//     pub /: *mut *mut __le32 d_magic; / the magic number,
//     pub /: *mut *mut __s16 d_type; / drive type,
//     pub /: *mut *mut __s16 d_subtype; / controller/d_type specific,
//     pub /: *mut *mut char d_typename[16]; / type name, e.g. "eagle",
//     pub /: *mut *mut char d_packname[16]; / pack identifier,
//     pub /: *mut *mut __u32 d_secsize; / # of bytes per sector,
//     pub /: *mut *mut __u32 d_nsectors; / # of data sectors per track,
//     pub /: *mut *mut __u32 d_ntracks; / # of tracks per cylinder,
//     pub /: *mut *mut __u32 d_ncylinders; / # of data cylinders per unit,
//     pub /: *mut *mut __u32 d_secpercyl; / # of data sectors per cylinder,
//     pub /: *mut *mut __u32 d_secperunit; / # of data sectors per unit,
//     pub /: *mut *mut __u16 d_sparespertrack; / # of spare sectors per track,
//     pub /: *mut *mut __u16 d_sparespercyl; / # of spare sectors per cylinder,
//     pub /: *mut *mut __u32 d_acylinders; / # of alt. cylinders per unit,
//     pub /: *mut *mut __u16 d_rpm; / rotational speed,
//     pub /: *mut *mut __u16 d_interleave; / hardware sector interleave,
//     pub /: *mut *mut __u16 d_trackskew; / sector 0 skew, per track,
//     pub /: *mut *mut __u16 d_cylskew; / sector 0 skew, per cylinder,
//     pub /: *mut *mut __u32 d_headswitch; / head switch time, usec,
//     pub /: *mut *mut __u32 d_trkseek; / track-to-track seek, usec,
//     pub /: *mut *mut __u32 d_flags; / generic flags,
pub const NDDATA: c_int = 5;
//     pub /: *mut *mut __u32 d_drivedata[NDDATA]; / drive-type specific information,
pub const NSPARE: c_int = 5;
//     pub /: *mut *mut __u32 d_spare[NSPARE]; / reserved for future use,
//     pub /: *mut *mut __le32 d_magic2; / the magic number (again),
//     pub /: *mut *mut __le16 d_checksum; / xor of data incl. partitions,
// filesystem and partition information:
//     pub /: *mut *mut __le16 d_npartitions; / number of partitions in following,
//     pub /: *mut *mut __le32 d_bbsize; / size of boot area at sn0, bytes,
//     pub /: *mut *mut __le32 d_sbsize; / max size of fs superblock, bytes,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsd_partition {
//     pub /: *mut *mut __le32 p_size; / number of sectors in partition,
//     pub /: *mut *mut __le32 p_offset; / starting sector,
//     pub /: *mut *mut __le32 p_fsize; / filesystem basic fragment size,
//     pub /: *mut *mut __u8 p_fstype; / filesystem type, see below,
//     pub /: *mut *mut __u8 p_frag; / filesystem fragments per block,
//     pub /: *mut *mut __le16 p_cpg; / filesystem cylinders per group,
//     pub /: *mut *mut } d_partitions[BSD_MAXPARTITIONS]; / actually may be more,
}

//
// Create devices for BSD partitions listed in a disklabel, under a
// dos-like partition. See parse_extended() for more information.
//
#[no_mangle]
pub unsafe extern "C" fn parse_bsd(state: *mut parsed_partitions, offset: sector_t, size: sector_t, origin: c_int, flavour: *mut c_char, max_partitions: c_int) {
    let mut sect;
pub static mut l: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    l = read_part_sector(state, offset + 1, &sect);
    if (!l) {
    return;
    }
    if (le32_to_cpu(l.d_magic) != BSD_DISKMAGIC) {
    put_dev_sector(sect);
    return;
    }
    seq_buf_printf(&state.pp_buf, " %s%d: <%s:", state.name, origin, flavour);
    if (le16_to_cpu(l.d_npartitions) < max_partitions) {
    max_partitions = le16_to_cpu(l.d_npartitions);
    }
    while (p - l.d_partitions < max_partitions) {
    sector_t bsd_start, bsd_size;
    if (state.next == state.limit) {
    break;
    }
    if (p.p_fstype == BSD_FS_UNUSED) {
    continue;
    }
    bsd_start = le32_to_cpu(p.p_offset);
    bsd_size = le32_to_cpu(p.p_size);
// FreeBSD has relative offset if C partition offset is zero
    if (memcmp(flavour, "bsd\0", 4) == 0 &&
    le32_to_cpu(l.d_partitions[2].p_offset) == 0) {
    bsd_start += offset;
    }
    if (offset == bsd_start && size == bsd_size) {
// full parent partition, we have it already
    continue;
    }
    if (offset > bsd_start || offset+size < bsd_start+bsd_size) {
    seq_buf_puts(&state.pp_buf, "bad subpartition - ignored\n");
    continue;
    }
    put_partition(state, state.next++, bsd_start, bsd_size);
    }
    put_dev_sector(sect);
    if (le16_to_cpu(l.d_npartitions) > max_partitions) {
    seq_buf_printf(&state.pp_buf, " (ignored %d more)",
    le16_to_cpu(l.d_npartitions) - max_partitions);
    }
    seq_buf_puts(&state.pp_buf, " >\n");
    }

#[no_mangle]
pub unsafe extern "C" fn parse_freebsd(state: *mut parsed_partitions, offset: sector_t, size: sector_t, origin: c_int) {

    parse_bsd(state, offset, size, origin, "bsd", BSD_MAXPARTITIONS);

    }
#[no_mangle]
pub unsafe extern "C" fn parse_netbsd(state: *mut parsed_partitions, offset: sector_t, size: sector_t, origin: c_int) {

    parse_bsd(state, offset, size, origin, "netbsd", BSD_MAXPARTITIONS);

    }
#[no_mangle]
pub unsafe extern "C" fn parse_openbsd(state: *mut parsed_partitions, offset: sector_t, size: sector_t, origin: c_int) {

    parse_bsd(state, offset, size, origin, "openbsd",
    OPENBSD_MAXPARTITIONS);

    }

pub const UNIXWARE_NUMSLICE: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unixware_slice {
//     pub /: *mut *mut __le16 s_label; / label,
//     pub /: *mut *mut __le16 s_flags; / permission flags,
//     pub /: *mut *mut __le32 start_sect; / starting sector,
//     pub /: *mut *mut __le32 nr_sects; / number of sectors in slice,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unixware_disklabel {
//     pub /: *mut *mut __le32 d_type; / drive type,
//     pub /: *mut *mut __le32 d_magic; / the magic number,
//     pub /: *mut *mut __le32 d_version; / version number,
//     pub /: *mut *mut char d_serial[12]; / serial number of the device,
//     pub /: *mut *mut __le32 d_ncylinders; / # of data cylinders per device,
//     pub /: *mut *mut __le32 d_ntracks; / # of tracks per cylinder,
//     pub /: *mut *mut __le32 d_nsectors; / # of data sectors per track,
//     pub /: *mut *mut __le32 d_secsize; / # of bytes per sector,
    pub partition*/: *mut *mut __le32 d_part_start; / # of first sector of this,
//     pub /: *mut *mut __le32 d_unknown1[12]; / ?,
//     pub /: *mut *mut __le32 d_alt_tbl; / byte offset of alternate table,
//     pub /: *mut *mut __le32 d_alt_len; / byte length of alternate table,
//     pub /: *mut *mut __le32 d_phys_cyl; / # of physical cylinders per device,
//     pub /: *mut *mut __le32 d_phys_trk; / # of physical tracks per cylinder,
//     pub /: *mut *mut __le32 d_phys_sec; / # of physical sectors per track,
//     pub /: *mut *mut __le32 d_phys_bytes; / # of physical bytes per sector,
//     pub /: *mut *mut __le32 d_unknown2; / ?,
//     pub /: *mut *mut __le32 d_unknown3; / ?,
//     pub /: *mut *mut __le32 d_pad[8]; / pad,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unixware_vtoc {
//     pub /: *mut *mut __le32 v_magic; / the magic number,
//     pub /: *mut *mut __le32 v_version; / version number,
//     pub /: *mut *mut char v_name[8]; / volume name,
//     pub /: *mut *mut __le16 v_nslices; / # of slices,
//     pub /: *mut *mut __le16 v_unknown1; / ?,
//     pub /: *mut *mut __le32 v_reserved[10]; / reserved, unixware_slice
//     pub /: *mut *mut v_slice[UNIXWARE_NUMSLICE]; / slice headers,
    pub vtoc: },
}

//
// Create devices for Unixware partitions listed in a disklabel, under a
// dos-like partition. See parse_extended() for more information.
//
#[no_mangle]
pub unsafe extern "C" fn parse_unixware(state: *mut parsed_partitions, offset: sector_t, size: sector_t, origin: c_int) {

    let mut sect;
pub static mut l: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    l = read_part_sector(state, offset + 29, &sect);
    if (!l) {
    return;
    }
    if (le32_to_cpu(l.d_magic) != UNIXWARE_DISKMAGIC ||
    le32_to_cpu(l.vtoc.v_magic) != UNIXWARE_DISKMAGIC2) {
    put_dev_sector(sect);
    return;
    }
    seq_buf_printf(&state.pp_buf, " %s%d: <unixware:", state.name, origin);
    p = &l.vtoc.v_slice[1];
// I omit the 0th slice as it is the same as whole disk.
    while (p - &l.vtoc.v_slice[0] < UNIXWARE_NUMSLICE) {
    if (state.next == state.limit) {
    break;
    }
    if (p.s_label != UNIXWARE_FS_UNUSED) {
    put_partition(state, state.next++,
    le32_to_cpu(p.start_sect),
    le32_to_cpu(p.nr_sects));
    }
    p += 1;
    }
    put_dev_sector(sect);
    seq_buf_puts(&state.pp_buf, " >\n");

    }
pub const MINIX_NR_SUBPARTITIONS: c_int = 4;
//
// Minix 2.0.0/2.0.2 subpartition support.
// Anand Krishnamurthy <anandk@wiproge.med.ge.com>
// Rajeev V. Pillai    <rajeevvp@yahoo.com>
//
#[no_mangle]
pub unsafe extern "C" fn parse_minix(state: *mut parsed_partitions, offset: sector_t, size: sector_t, origin: c_int) {

    let mut sect;
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    data = read_part_sector(state, offset, &sect);
    if (!data) {
    return;
    }
    p = (data + 0x1be);
// The first sector of a Minix partition can have either
// a secondary MBR describing its subpartitions, or
// the normal boot sector.
    if (msdos_magic_present(data + 510) &&
    p.sys_ind == MINIX_PARTITION) { /* subpartition table present */ {
    seq_buf_printf(&state.pp_buf, " %s%d: <minix:", state.name, origin);
    }
    while (i < MINIX_NR_SUBPARTITIONS) {
    if (state.next == state.limit) {
    break;
    }
// add each partition in use
    if (p.sys_ind == MINIX_PARTITION) {
    put_partition(state, state.next++,
    start_sect(p), nr_sects(p));
    }
    }
    seq_buf_puts(&state.pp_buf, " >\n");
    }
    put_dev_sector(sect);

    }
pub static mut subtypes: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn msdos_partition(state: *mut parsed_partitions) -> c_int {
    let mut sector_size;
    let mut sect;
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut fb: *mut c_void = core::ptr::null_mut();
    let mut slot = 0;
    let mut disksig = 0;
    sector_size = queue_logical_block_size(state.disk.queue) / 512;
    data = read_part_sector(state, 0, &sect);
    if (!data) {
    return -1;
    }
//
// Note order! (some AIX disks, e.g. unbootable kind,
have no MSDOS 55aa)
//
    if (aix_magic_present(state, data)) {
    put_dev_sector(sect);

    return aix_partition(state);

    seq_buf_puts(&state.pp_buf, " [AIX]");
    return 0;

    }
    if (!msdos_magic_present(data + 510)) {
    put_dev_sector(sect);
    return 0;
    }
//
// Now that the 55aa signature is present, this is probably
// either the boot sector of a FAT filesystem or a DOS-type
// partition table. Reject this in case the boot indicator
// is not 0 or 0x80.
//
    p =  (data + 0x1be);
    while (slot <= 4) {
    if (p.boot_ind != 0 && p.boot_ind != 0x80) {
//
// Even without a valid boot indicator value
// its still possible this is valid FAT filesystem
// without a partition table.
//
    fb =  data;
    if (slot == 1 && fb.reserved && fb.fats
    && fat_valid_media(fb.media)) {
    seq_buf_puts(&state.pp_buf, "\n");
    put_dev_sector(sect);
    return 1;
    } else {
    put_dev_sector(sect);
    return 0;
    }
    }
    }

    p =  (data + 0x1be);
    while (slot <= 4 ) {
// If this is an EFI GPT disk, msdos should ignore it.
    if (p.sys_ind == EFI_PMBR_OSTYPE_EFI_GPT) {
    put_dev_sector(sect);
    return 0;
    }
    }

    p =  (data + 0x1be);
    disksig = le32_to_cpup((data + 0x1b8));
//
// Look for partitions in two passes:
// First find the primary and DOS-type extended partitions.
// On the second pass look inside *BSD, Unixware and Solaris partitions.
//
    state.next = 5;
    while (slot <= 4 ) {
pub static mut start: sector_t = 0;
pub static mut size: sector_t = 0;
    if (!size) {
    continue;
    }
    if (is_extended_partition(p)) {
//
// prevent someone doing mkfs or mkswap on an
// extended partition, but leave room for LILO
// FIXME: this uses one logical sector for > 512b
// sector, although it may not be enough/proper.
//
pub static mut n: sector_t = 2;
    n = min(size, max(sector_size, n));
    put_partition(state, slot, start, n);
    seq_buf_puts(&state.pp_buf, " <");
    parse_extended(state, start, size, disksig);
    seq_buf_puts(&state.pp_buf, " >");
    continue;
    }
    put_partition(state, slot, start, size);
    set_info(state, slot, disksig);
    if (p.sys_ind == LINUX_RAID_PARTITION) {
    state.parts[slot].flags = ADDPART_FLAG_RAID;
    }
    if (p.sys_ind == DM6_PARTITION) {
    seq_buf_puts(&state.pp_buf, "[DM]");
    }
    if (p.sys_ind == EZD_PARTITION) {
    seq_buf_puts(&state.pp_buf, "[EZD]");
    }
    }
    seq_buf_puts(&state.pp_buf, "\n");
// second pass - output for each on a separate line
    p =  (0x1be + data);
    while (slot <= 4 ) {
pub static mut id: c_uchar = 0;
    let mut n = 0;
    if (!nr_sects(p)) {
    continue;
    }
    for (n = 0; subtypes[n].parse && id != subtypes[n].id; n++) {
    ;
    }
    if (!subtypes[n].parse) {
    continue;
    }
    subtypes[n].parse(state, start_sect(p) * sector_size,
    nr_sects(p) * sector_size, slot);
    }
    put_dev_sector(sect);
    return 1;
    }