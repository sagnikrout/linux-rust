//! Automatically rewritten from C to Rust
//! Source: block/partitions/acorn.c
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
// Copyright (c) 1996-2000 Russell King.
//
// Scan ADFS partitions on hard disk drives.  Unfortunately, there
// isn't a standard for partitioning drives on Acorn machines, so
// every single manufacturer of SCSI and IDE cards created their own
// method.
//

//
// Partition types. (Oh for reusability)
//
pub const PARTITION_RISCIX_MFM: c_int = 1;
pub const PARTITION_RISCIX_SCSI: c_int = 2;
pub const PARTITION_LINUX: c_int = 9;

    defined(CONFIG_ACORN_PARTITION_ADFS)
#[no_mangle]
pub unsafe extern "C" fn adfs_partition(state: *mut parsed_partitions, name: *mut c_char, data: *mut c_char, first_sector: c_ulong, slot: c_int) -> *mut c_void {
pub static mut dr: *mut c_void = core::ptr::null_mut();
    let mut nr_sects = 0;
    if (adfs_checkbblk(data)) {
    return core::ptr::null_mut();
    }
    dr = (data + 0x1c0);
    if (dr.disc_size == 0 && dr.disc_size_high == 0) {
    return core::ptr::null_mut();
    }
    nr_sects = (le32_to_cpu(dr.disc_size_high) << 23) |
    (le32_to_cpu(dr.disc_size) >> 9);
    if (name) {
    seq_buf_printf(&state.pp_buf, " [%s]", name);
    }
    put_partition(state, slot, first_sector, nr_sects);
    return dr;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscix_part {
    pub start: __le32,
    pub length: __le32,
    pub one: __le32,
    pub name: [c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscix_record {
    pub magic: __le32,

    pub date: __le32,
    pub part: [riscix_part; 8],
}

    defined(CONFIG_ACORN_PARTITION_ADFS)
#[no_mangle]
pub unsafe extern "C" fn riscix_partition(state: *mut parsed_partitions, first_sect: c_ulong, slot: c_int, nr_sects: c_ulong) -> c_int {
    let mut sect;
pub static mut rr: *mut c_void = core::ptr::null_mut();
    rr = read_part_sector(state, first_sect, &sect);
    if (!rr) {
    return -1;
    }
    seq_buf_puts(&state.pp_buf, " [RISCiX]");
    if (rr.magic == RISCIX_MAGIC) {
pub static mut size: c_ulong = 0;
    let mut part = 0;
    seq_buf_puts(&state.pp_buf, " <");
    put_partition(state, slot++, first_sect, size);
    while (part < 8) {
    if (rr.part[part].one &&
    memcmp(rr.part[part].name, "All\0", 4)) {
    put_partition(state, slot++,
    le32_to_cpu(rr.part[part].start),
    le32_to_cpu(rr.part[part].length));
    seq_buf_printf(&state.pp_buf, "(%s)", rr.part[part].name);
    }
    }
    seq_buf_puts(&state.pp_buf, " >\n");
    } else {
    put_partition(state, slot++, first_sect, nr_sects);
    }
    put_dev_sector(sect);
    return slot;
    }

pub const LINUX_NATIVE_MAGIC: c_uint = 0xdeafa1de;
pub const LINUX_SWAP_MAGIC: c_uint = 0xdeafab1e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct linux_part {
    pub magic: __le32,
    pub start_sect: __le32,
    pub nr_sects: __le32,
}

    defined(CONFIG_ACORN_PARTITION_ADFS)
#[no_mangle]
pub unsafe extern "C" fn linux_partition(state: *mut parsed_partitions, first_sect: c_ulong, slot: c_int, nr_sects: c_ulong) -> c_int {
    let mut sect;
pub static mut linuxp: *mut c_void = core::ptr::null_mut();
pub static mut size: c_ulong = 0;
    seq_buf_puts(&state.pp_buf, " [Linux]");
    put_partition(state, slot++, first_sect, size);
    linuxp = read_part_sector(state, first_sect, &sect);
    if (!linuxp) {
    return -1;
    }
    seq_buf_puts(&state.pp_buf, " <");
    while (linuxp.magic == cpu_to_le32(LINUX_NATIVE_MAGIC) ||
    linuxp.magic == cpu_to_le32(LINUX_SWAP_MAGIC)) {
    if (slot == state.limit) {
    break;
    }
    put_partition(state, slot++, first_sect +
    le32_to_cpu(linuxp.start_sect),
    le32_to_cpu(linuxp.nr_sects));
    linuxp ++;
    }
    seq_buf_puts(&state.pp_buf, " >");
    put_dev_sector(sect);
    return slot;
    }

#[no_mangle]
pub unsafe extern "C" fn adfspart_check_CUMANA(state: *mut parsed_partitions) -> c_int {
pub static mut first_sector: c_ulong = 0;
pub static mut start_blk: c_uint = 0;
    let mut sect;
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut name = "CUMANA/ADFS";
pub static mut first: c_int = 1;
pub static mut slot: c_int = 1;
//
// Try Cumana style partitions - sector 6 contains ADFS boot block
// with pointer to next 'drive'.
//
// There are unknowns in this code - is the 'cylinder number' of the
// next partition relative to the start of this one - I'm assuming
// it is.
//
// Also, which ID did Cumana use?
//
// This is totally unfinished, and will require more work to get it
// going. Hence it is totally untested.
//
    do {
pub static mut dr: *mut c_void = core::ptr::null_mut();
    let mut nr_sects = 0;
    data = read_part_sector(state, start_blk * 2 + 6, &sect);
    if (!data) {
    return -1;
    }
    if (slot == state.limit) {
    break;
    }
    dr = adfs_partition(state, name, data, first_sector, slot++);
    if (!dr) {
    break;
    }
    name = core::ptr::null_mut();
    nr_sects = (data[0x1fd] + (data[0x1fe] << 8)) *
    (dr.heads + (dr.lowsector & 0x40 ? 1 : 0)) *
    dr.secspertrack;
    if (!nr_sects) {
    break;
    }
    first = 0;
    first_sector += nr_sects;
    start_blk += nr_sects >> (BLOCK_SIZE_BITS - 9);
    nr_sects = 0; /* hmm - should be partition size */
    match (data[0x1fc] & 15) {
    0 => {
    // break;

    }
    PARTITION_RISCIX_SCSI => {
// RISCiX - we don't know how to find the next one.
    slot = riscix_partition(state, first_sector, slot,
    nr_sects);
    // break;

    }
    PARTITION_LINUX => {
    slot = linux_partition(state, first_sector, slot,
    nr_sects);
    // break;
    }
    }
    put_dev_sector(sect);
    if (slot == -1) {
    return -1;
    }
    } while (1);
    put_dev_sector(sect);
    return first ? 0 : 1;
    }

//
// Purpose: allocate ADFS partitions.
//
// Params : hd		- pointer to gendisk structure to store partition info.
// dev		- device number to access.
//
// Returns: -1 on error, 0 for no ADFS boot sector, 1 for ok.
//
// Alloc  : hda  = whole drive
// hda1 = ADFS partition on first drive.
// hda2 = non-ADFS partition.
//
#[no_mangle]
pub unsafe extern "C" fn adfspart_check_ADFS(state: *mut parsed_partitions) -> c_int {
    unsigned long start_sect, nr_sects, sectscyl, heads;
    let mut sect;
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut dr: *mut c_void = core::ptr::null_mut();
    let mut id = 0;
pub static mut slot: c_int = 1;
    data = read_part_sector(state, 6, &sect);
    if (!data) {
    return -1;
    }
    dr = adfs_partition(state, "ADFS", data, 0, slot++);
    if (!dr) {
    put_dev_sector(sect);
    return 0;
    }
    heads = dr.heads + ((dr.lowsector >> 6) & 1);
    sectscyl = dr.secspertrack * heads;
    start_sect = ((data[0x1fe] << 8) + data[0x1fd]) * sectscyl;
    id = data[0x1fc] & 15;
    put_dev_sector(sect);
//
// Work out start of non-adfs partition.
//
    nr_sects = get_capacity(state.disk) - start_sect;
    if (start_sect) {
    match (id) {

    PARTITION_RISCIX_SCSI => {
    }
    PARTITION_RISCIX_MFM => {
    riscix_partition(state, start_sect, slot,
    nr_sects);
    // break;

    }
    PARTITION_LINUX => {
    linux_partition(state, start_sect, slot,
    nr_sects);
    // break;
    }
    }
    }
    seq_buf_puts(&state.pp_buf, "\n");
    return 1;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ics_part {
    pub start: __le32,
    pub size: __le32,
}

#[no_mangle]
pub unsafe extern "C" fn adfspart_check_ICSLinux(state: *mut parsed_partitions, block: c_ulong) -> c_int {
    let mut sect;
    let mut data = read_part_sector(state, block, &sect);
pub static mut result: c_int = 0;
    if (data) {
    if (memcmp(data, "LinuxPart", 9) == 0) {
    result = 1;
    }
    put_dev_sector(sect);
    }
    return result;
    }
//
// Check for a valid ICS partition using the checksum.
//
#[no_mangle]
pub unsafe extern "C" fn valid_ics_sector(data: *const c_uchar) -> c_int {
    let mut sum = 0;
    let mut i = 0;
    for (i = 0, sum = 0x50617274; i < 508; i++) {
    sum += data[i];
    }
    sum -= le32_to_cpu(*(&data[508]));
pub static mut sum: return = 0;
    }
//
// Purpose: allocate ICS partitions.
// Params : hd		- pointer to gendisk structure to store partition info.
// dev		- device number to access.
// Returns: -1 on error, 0 for no ICS table, 1 for partitions ok.
// Alloc  : hda  = whole drive
// hda1 = ADFS partition 0 on first drive.
// hda2 = ADFS partition 1 on first drive.
// ..etc..
//
#[no_mangle]
pub unsafe extern "C" fn adfspart_check_ICS(state: *mut parsed_partitions) -> c_int {
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut slot = 0;
    let mut sect;
//
// Try ICS style partitions - sector 0 contains partition info.
//
    data = read_part_sector(state, 0, &sect);
    if (!data) {
    return -1;
    }
    if (!valid_ics_sector(data)) {
    put_dev_sector(sect);
    return 0;
    }
    seq_buf_puts(&state.pp_buf, " [ICS]");
    while (p.size) {
pub static mut start: u32 = 0;
    s32 size = le32_to_cpu(p.size); /* yes, it's signed. */
    if (slot == state.limit) {
    break;
    }
//
// Negative sizes tell the RISC OS ICS driver to ignore
// this partition - in effect it says that this does not
// contain an ADFS filesystem.
//
    if (size < 0) {
    size = -size;
//
// Our own extension - We use the first sector
// of the partition to identify what type this
// partition is.  We must not make this visible
// to the filesystem.
//
    if (size > 1 && adfspart_check_ICSLinux(state, start)) {
    start += 1;
    size -= 1;
    }
    }
    if (size) {
    put_partition(state, slot++, start, size);
    }
    }
    put_dev_sector(sect);
    seq_buf_puts(&state.pp_buf, "\n");
    return 1;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptec_part {
    pub unused1: __le32,
    pub unused2: __le32,
    pub start: __le32,
    pub size: __le32,
    pub unused5: __le32,
    pub type: [c_char; 8],
}

#[no_mangle]
pub unsafe extern "C" fn valid_ptec_sector(data: *const c_uchar) -> c_int {
pub static mut checksum: c_uchar = 0;
    let mut i = 0;
//
// If it looks like a PC/BIOS partition, then it
// probably isn't PowerTec.
//
    if (data[510] == 0x55 && data[511] == 0xaa) {
    return 0;
    }
    for (i = 0; i < 511; i++) {
    checksum += data[i];
    }
pub static mut checksum: return = 0;
    }
//
// Purpose: allocate ICS partitions.
// Params : hd		- pointer to gendisk structure to store partition info.
// dev		- device number to access.
// Returns: -1 on error, 0 for no ICS table, 1 for partitions ok.
// Alloc  : hda  = whole drive
// hda1 = ADFS partition 0 on first drive.
// hda2 = ADFS partition 1 on first drive.
// ..etc..
//
#[no_mangle]
pub unsafe extern "C" fn adfspart_check_POWERTEC(state: *mut parsed_partitions) -> c_int {
    let mut sect;
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut slot: c_int = 1;
    let mut i = 0;
    data = read_part_sector(state, 0, &sect);
    if (!data) {
    return -1;
    }
    if (!valid_ptec_sector(data)) {
    put_dev_sector(sect);
    return 0;
    }
    seq_buf_puts(&state.pp_buf, " [POWERTEC]");
    while (i < 12) {
pub static mut start: u32 = 0;
pub static mut size: u32 = 0;
    if (size) {
    put_partition(state, slot++, start, size);
    }
    }
    put_dev_sector(sect);
    seq_buf_puts(&state.pp_buf, "\n");
    return 1;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eesox_part {
    pub magic: [c_char; 6],
    pub name: [c_char; 10],
    pub start: __le32,
    pub unused6: __le32,
    pub unused7: __le32,
    pub unused8: __le32,
}

//
// Guess who created this format?
//
    static const char eesox_name[] = {
    'N', 'e', 'i', 'l', ' ',
    'C', 'r', 'i', 't', 'c', 'h', 'e', 'l', 'l', ' ', ' '
    };
//
// EESOX SCSI partition format.
//
// This is a goddamned awful partition format.  We don't seem to store
// the size of the partition in this table, only the start addresses.
//
// There are two possibilities where the size comes from:
// 1. The individual ADFS boot block entries that are placed on the disk.
// 2. The start address of the next entry.
//
#[no_mangle]
pub unsafe extern "C" fn adfspart_check_EESOX(state: *mut parsed_partitions) -> c_int {
    let mut sect;
pub static mut data: *mut c_void = core::ptr::null_mut();
    unsigned char buffer[256];
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut start: sector_t = 0;
    int i, slot = 1;
    data = read_part_sector(state, 7, &sect);
    if (!data) {
    return -1;
    }
//
// "Decrypt" the partition table.  God knows why...
//
    for (i = 0; i < 256; i++) {
    buffer[i] = data[i] ^ eesox_name[i & 15];
    }
    put_dev_sector(sect);
    while (i < 8) {
    let mut next;
    if (memcmp(p.magic, "Eesox", 6)) {
    break;
    }
    next = le32_to_cpu(p.start);
    if (i) {
    put_partition(state, slot++, start, next - start);
    }
    start = next;
    }
    if (i != 0) {
    let mut size;
    size = get_capacity(state.disk);
    put_partition(state, slot++, start, size - start);
    seq_buf_puts(&state.pp_buf, "\n");
    }
    return i ? 1 : 0;
    }