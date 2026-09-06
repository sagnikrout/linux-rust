//! Automatically rewritten from C to Rust
//! Source: block/partitions/ibm.c
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
// Author(s)......: Holger Smolinski <Holger.Smolinski@de.ibm.com>
// Volker Sameske <sameske@de.ibm.com>
// Bugreports.to..: <Linux390@de.ibm.com>
// Copyright IBM Corp. 1999, 2012
//

    union label_t {
pub static mut vol: usize = 0;
pub static mut lnx: usize = 0;
pub static mut cms: usize = 0;
    };
//
// compute the block number from a
// cyl-cyl-head-head structure
//
#[no_mangle]
unsafe extern "C" fn cchh2blk(ptr: *mut vtoc_cchh, geo: *mut hd_geometry) -> sector_t {
    let mut cyl;
    let mut head = 0;
// decode cylinder and heads for large volumes
    cyl = ptr.hh & 0xFFF0;
    cyl <<= 12;
    cyl |= ptr.cc;
    head = ptr.hh & 0x000F;
    return cyl * geo.heads * geo.sectors +
    head * geo.sectors;
    }
//
// compute the block number from a
// cyl-cyl-head-head-block structure
//
#[no_mangle]
unsafe extern "C" fn cchhb2blk(ptr: *mut vtoc_cchhb, geo: *mut hd_geometry) -> sector_t {
    let mut cyl;
    let mut head = 0;
// decode cylinder and heads for large volumes
    cyl = ptr.hh & 0xFFF0;
    cyl <<= 12;
    cyl |= ptr.cc;
    head = ptr.hh & 0x000F;
    return	cyl * geo.heads * geo.sectors +
    head * geo.sectors +
    ptr.b;
    }
// Volume Label Type/ID Length
pub const DASD_VOL_TYPE_LEN: c_int = 4;
pub const DASD_VOL_ID_LEN: c_int = 6;
// Volume Label Types
pub const DASD_VOLLBL_TYPE_VOL1: c_int = 0;
pub const DASD_VOLLBL_TYPE_LNX1: c_int = 1;
pub const DASD_VOLLBL_TYPE_CMS1: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_vollabel {
    pub type: *mut c_char,
    pub idx: c_int,
}

pub static mut dasd_vollabel: usize = 0;
#[no_mangle]
unsafe extern "C" fn get_label_by_type(type: *const c_char) -> c_int {
    let mut i = 0;
    while (i < ARRAY_SIZE!(dasd_vollabels)) {
    if (!memcmp(type, dasd_vollabels[i].type, DASD_VOL_TYPE_LEN)) {
    return dasd_vollabels[i].idx;
    }
    }
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn find_label(state: *mut parsed_partitions, info: *mut dasd_information2_t, geo: *mut hd_geometry, blocksize: c_int, labelsect: *mut sector_t, label: *mut union label_t) -> c_int {
    sector_t testsect[3];
    let mut i = 0;
    let mut testcount = 0;
    let mut sect;
pub static mut data: *mut c_void = core::ptr::null_mut();
// There a three places where we may find a valid label:
// - on an ECKD disk it's block 2
// - on an FBA disk it's block 1
// - on an CMS formatted FBA disk it is sector 1, even if the block size
// is larger than 512 bytes (possible if the DIAG discipline is used)
// If we have a valid info structure, then we know exactly which case we
// have, otherwise we just search through all possebilities.
//
    if (info) {
    if ((info.cu_type == 0x6310 && info.dev_type == 0x9336) ||
    (info.cu_type == 0x3880 && info.dev_type == 0x3370)) {
    testsect[0] = info.label_block;
    }
    else {
    testsect[0] = info.label_block * (blocksize >> 9);
    }
    testcount = 1;
    } else {
    testsect[0] = 1;
    testsect[1] = (blocksize >> 9);
    testsect[2] = 2 * (blocksize >> 9);
    testcount = 3;
    }
    while (i < testcount) {
    data = read_part_sector(state, testsect[i], &sect);
    if (data == core::ptr::null_mut()) {
    continue;
    }
    memcpy(label, data, sizeof!(*label));
    memcpy(type, data, DASD_VOL_TYPE_LEN);
    EBCASC(type, DASD_VOL_TYPE_LEN);
    put_dev_sector(sect);
    switch (get_label_by_type(type)) {
    case DASD_VOLLBL_TYPE_VOL1:
    memcpy(name, label.vol.volid, DASD_VOL_ID_LEN);
    EBCASC(name, DASD_VOL_ID_LEN);
// labelsect = testsect[i];
    return 1;
    case DASD_VOLLBL_TYPE_LNX1:
    case DASD_VOLLBL_TYPE_CMS1:
    memcpy(name, label.lnx.volid, DASD_VOL_ID_LEN);
    EBCASC(name, DASD_VOL_ID_LEN);
// labelsect = testsect[i];
    return 1;
// label;
    break;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn find_vol1_partitions(state: *mut parsed_partitions, geo: *mut hd_geometry, blocksize: c_int, label: *mut union label_t) -> c_int {
    let mut blk;
    let mut counter = 0;
    let mut sect;
pub static mut data: *mut c_void = core::ptr::null_mut();
    loff_t offset, size;
pub static mut f1: usize = 0;
    let mut secperblk = 0;
    seq_buf_printf(&state.pp_buf, "VOL1/%8s:", name);
//
// get start of VTOC from the disk label and then search for format1
// and format8 labels
//
    secperblk = blocksize >> 9;
    blk = cchhb2blk(&label.vol.vtoc, geo) + 1;
    counter = 0;
    data = read_part_sector(state, blk * secperblk, &sect);
    while (data != core::ptr::null_mut()) {
    memcpy(&f1, data, sizeof!(vtoc_format1_label));
    put_dev_sector(sect);
// skip FMT4 / FMT5 / FMT7 labels
    if (f1.DS1FMTID == _ascebc['4']
    || f1.DS1FMTID == _ascebc['5']
    || f1.DS1FMTID == _ascebc['7']
    || f1.DS1FMTID == _ascebc['9']) {
    blk += 1;
    data = read_part_sector(state, blk * secperblk, &sect);
    continue;
    }
// only FMT1 and 8 labels valid at this point
    if (f1.DS1FMTID != _ascebc['1'] &&
    f1.DS1FMTID != _ascebc['8']) {
    break;
    }
// OK, we got valid partition data
    offset = cchh2blk(&f1.DS1EXT1.llimit, geo);
    size  = cchh2blk(&f1.DS1EXT1.ulimit, geo) -
    offset + geo.sectors;
    offset *= secperblk;
    size *= secperblk;
    if (counter >= state.limit) {
    break;
    }
    put_partition(state, counter + 1, offset, size);
    counter += 1;
    blk += 1;
    data = read_part_sector(state, blk * secperblk, &sect);
    }
    seq_buf_puts(&state.pp_buf, "\n");
    if (!data) {
    return -1;
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn find_lnx1_partitions(state: *mut parsed_partitions, geo: *mut hd_geometry, blocksize: c_int, label: *mut union label_t, labelsect: sector_t, nr_sectors: sector_t, info: *mut dasd_information2_t) -> c_int {
    loff_t offset, geo_size, size;
    let mut secperblk = 0;
    seq_buf_printf(&state.pp_buf, "LNX1/%8s:", name);
    secperblk = blocksize >> 9;
    if (label.lnx.ldl_version == 0xf2) {
    size = label.lnx.formatted_blocks * secperblk;
    } else {
//
// Formated w/o large volume support. If the sanity check
// 'size based on geo == size based on nr_sectors' is true, then
// we can safely assume that we know the formatted size of
// the disk, otherwise we need additional information
// that we can only get from a real DASD device.
//
    geo_size = geo.cylinders * geo.heads
// geo->sectors * secperblk;
    size = nr_sectors;
    if (size != geo_size) {
    if (!info) {
    seq_buf_puts(&state.pp_buf, "\n");
    return 1;
    }
    if (!strcmp(info.type, "ECKD")) {
    if (geo_size < size)
    size = geo_size;
    }
// else keep size based on nr_sectors
    }
    }
// first and only partition starts in the first block after the label
    offset = labelsect + secperblk;
    put_partition(state, 1, offset, size - offset);
    seq_buf_puts(&state.pp_buf, "\n");
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn find_cms1_partitions(state: *mut parsed_partitions, geo: *mut hd_geometry, blocksize: c_int, label: *mut union label_t, labelsect: sector_t) -> c_int {
    loff_t offset, size;
    let mut secperblk = 0;
//
// VM style CMS1 labeled disk
//
    blocksize = label.cms.block_size;
    secperblk = blocksize >> 9;
    if (label.cms.disk_offset != 0) {
    seq_buf_printf(&state.pp_buf, "CMS1/%8s(MDSK):", name);
// disk is reserved minidisk
    offset = label.cms.disk_offset * secperblk;
    size = (label.cms.block_count - 1) * secperblk;
    } else {
    seq_buf_printf(&state.pp_buf, "CMS1/%8s:", name);
//
// Special case for FBA devices:
// If an FBA device is CMS formatted with blocksize > 512 byte
// and the DIAG discipline is used, then the CMS label is found
// in sector 1 instead of block 1. However, the partition is
// still supposed to start in block 2.
//
    if (labelsect == 1) {
    offset = 2 * secperblk;
    }
    else {
    offset = labelsect + secperblk;
    }
    size = label.cms.block_count * secperblk;
    }
    put_partition(state, 1, offset, size-offset);
    seq_buf_puts(&state.pp_buf, "\n");
    return 1;
    }
//
// This is the main function, called by check.c
//
#[no_mangle]
pub unsafe extern "C" fn ibm_partition(state: *mut parsed_partitions) -> c_int {
    int (*fn)(gendisk *disk, dasd_information2_t *info);
    let mut disk = state.disk;
    let mut bdev = disk.part0;
    let mut blocksize = 0;
    let mut res = 0;
    loff_t offset, size;
    let mut nr_sectors;
pub static mut info: *mut c_void = core::ptr::null_mut();
pub static mut geo: *mut c_void = core::ptr::null_mut();
    char type[DASD_VOL_TYPE_LEN + 1] = "";
    char name[DASD_VOL_ID_LEN + 1] = "";
    let mut labelsect;
    union label_t *label;
    res = 0;
    if (!disk.fops.getgeo) {
// goto;
    }
    fn = symbol_get(dasd_biodasdinfo);
    blocksize = bdev_logical_block_size(bdev);
    if (blocksize <= 0) {
// goto;
    }
    nr_sectors = bdev_nr_sectors(bdev);
    if (nr_sectors == 0) {
// goto;
    }
    info = kmalloc_obj(dasd_information2_t);
    if (info == core::ptr::null_mut()) {
// goto;
    }
    geo = kmalloc_obj(hd_geometry);
    if (geo == core::ptr::null_mut()) {
// goto;
    }
    label = kmalloc_obj(union label_t);
    if (label == core::ptr::null_mut()) {
// goto;
    }
// set start if not filled by getgeo function e.g. virtblk
    geo.start = get_start_sect(bdev);
    if (disk.fops.getgeo(disk, geo)) {
// goto;
    }
    if (!fn || fn(disk, info)) {
    kfree(info);
    info = core::ptr::null_mut();
    }
    if (find_label(state, info, geo, blocksize, &labelsect, name, type, label)) {
    switch (get_label_by_type(type)) {
    case DASD_VOLLBL_TYPE_VOL1:
    res = find_vol1_partitions(state, geo, blocksize, name,
    label);
    break;
    case DASD_VOLLBL_TYPE_LNX1:
    res = find_lnx1_partitions(state, geo, blocksize, name,
    label, labelsect, nr_sectors,
    info);
    break;
    case DASD_VOLLBL_TYPE_CMS1:
    res = find_cms1_partitions(state, geo, blocksize, name,
    label, labelsect);
    break;
    }
    } else if (info) {
//
// ugly but needed for backward compatibility:
// If the block device is a DASD (i.e. BIODASDINFO2 works),
// then we claim it in any case, even though it has no valid
// label. If it has the LDL format, then we simply define a
// partition as if it had an LNX1 label.
//
    res = 1;
    if (info.format == DASD_FORMAT_LDL) {
    seq_buf_puts(&state.pp_buf, "(nonl)");
    size = nr_sectors;
    offset = (info.label_block + 1) * (blocksize >> 9);
    put_partition(state, 1, offset, size-offset);
    seq_buf_puts(&state.pp_buf, "\n");
    }
    } else {
    res = 0;
    }
// label;
    kfree(label);
// label;
    kfree(geo);
// label;
    kfree(info);
// label;
    if (fn) {
    symbol_put(dasd_biodasdinfo);
    }
// label;
    return res;
    }