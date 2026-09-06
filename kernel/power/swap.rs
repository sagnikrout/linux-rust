//! Automatically rewritten from C to Rust
//! Source: kernel/power/swap.c
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
// linux/kernel/power/swap.c
//
// This file provides functions for reading the suspend image from
// and writing it to a swap partition.
//
// Copyright (C) 1998,2001-2005 Pavel Machek <pavel@ucw.cz>
// Copyright (C) 2006 Rafael J. Wysocki <rjw@sisk.pl>
// Copyright (C) 2010-2012 Bojan Smojver <bojan@rexursive.com>
//

    let mut swsusp_hardware_signature = 0;
//
// When reading an {un,}compressed image, we may restore pages in place,
// in which case some architectures need these pages cleaning before they
// can be executed. We don't know which pages these may be, so clean the lot.
//
    static bool clean_pages_on_read;
    static bool clean_pages_on_decompress;
//
// The swap map is a data structure used for keeping track of each page
// written to a swap partition.  It consists of many swap_map_page structures
// that contain each an array of MAP_PAGE_ENTRIES swap entries.  These
// structures are stored on the swap and linked together with the help of the
// .next_swap member.
//
// The swap map is created during suspend.  The swap map pages are allocated and
// populated one at a time, so we only need one memory page to set up the entire
// structure.
//
// During resume we pick up all swap_map_page structures into a list.
//

//
// Number of free pages that are not high.
//
#[no_mangle]
pub unsafe extern "C" fn low_free_pages() -> c_ulong {
    return nr_free_pages() - nr_free_highpages();
    }
//
// Number of pages required to be kept free while writing the image. Always
// half of all available low pages before the writing starts.
//
#[no_mangle]
pub unsafe extern "C" fn reqd_free_pages() -> c_ulong {
    return low_free_pages() / 2;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swap_map_page {
    pub entries: [sector_t; MAP_PAGE_ENTRIES],
    pub next_swap: sector_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct swap_map_page_list {
    pub map: *mut swap_map_page,
    pub next: *mut swap_map_page_list,
}

//
// The swap_map_handle structure is used for handling swap in a file-alike way.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swap_map_handle {
    pub cur: *mut swap_map_page,
    pub maps: *mut swap_map_page_list,
    pub cur_swap: sector_t,
    pub first_sector: sector_t,
    pub k: c_uint,
    pub reqd_free_pages: c_ulong,
    pub crc32: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct swsusp_header {
    char reserved[PAGE_SIZE - 20 - sizeof!(sector_t) - sizeof!(int) -
    pub sizeof!(u32)]: sizeof!(u32) -,
    pub hw_sig: u32,
    pub crc32: u32,
    pub image: sector_t,
//     pub /: *mut *mut unsigned int flags; / Flags to pass to the "boot" kernel,
    pub orig_sig: [c_char; 10],
    pub sig: [c_char; 10],
    pub __packed: },
    pub swsusp_header: *mut static struct swsusp_header,
//
// The following functions are used for tracing the allocated swap pages, so
// that they can be freed in case of an error.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swsusp_extent {
    pub node: rb_node,
    pub start: c_ulong,
    pub end: c_ulong,
}

pub static mut swsusp_extents: rb_root = 0;
#[no_mangle]
unsafe extern "C" fn swsusp_extents_insert(swap_offset: c_ulong) -> c_int {
    let mut new = &(swsusp_extents.rb_node);
    let mut parent = core::ptr::null_mut();
pub static mut ext: *mut c_void = core::ptr::null_mut();
// Figure out where to put the new node
    while (*new) {
    ext = rb_entry(*new, swsusp_extent, node);
    parent = *new;
    if (swap_offset < ext.start) {
// Try to merge
    if (swap_offset == ext.start - 1) {
    ext.start -= 1;
    return 0;
    }
    new = &((*new).rb_left);
    } else if (swap_offset > ext.end) {
// Try to merge
    if (swap_offset == ext.end + 1) {
    ext.end += 1;
    return 0;
    }
    new = &((*new).rb_right);
    } else {
// It already is in the tree
    return -EINVAL;
    }
    }
// Add the new node and rebalance the tree.
    ext = kzalloc_obj(swsusp_extent);
    if (!ext) {
    return -ENOMEM;
    }
    ext.start = swap_offset;
    ext.end = swap_offset;
    rb_link_node(&ext.node, parent, new);
    rb_insert_color(&ext.node, &swsusp_extents);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_swapdev_block(swap: c_int) -> sector_t {
    let mut offset = 0;
//
// Allocate a swap page and register that it has been allocated, so that
// it can be freed in case of an error.
//
    offset = swp_offset(swap_alloc_hibernation_slot(swap));
    if (offset) {
    if (swsusp_extents_insert(offset)) {
    swap_free_hibernation_slot(swp_entry(swap, offset));
    }
    else {
    return swapdev_block(swap, offset);
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn free_all_swap_pages(swap: c_int) {
    let mut offset = 0;
pub static mut node: *mut c_void = core::ptr::null_mut();
//
// Free swap pages allocated for saving image data.  It also frees the
// extents used to register which swap entries had been allocated.
//
    while ((node = swsusp_extents.rb_node)) {
pub static mut ext: *mut c_void = core::ptr::null_mut();
    ext = rb_entry(node, swsusp_extent, node);
    rb_erase(node, &swsusp_extents);
    for (offset = ext.start; offset <= ext.end; offset++) {
    swap_free_hibernation_slot(swp_entry(swap, offset));
    }
    kfree(ext);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn swsusp_swap_in_use() -> c_int {
    return (swsusp_extents.rb_node != core::ptr::null_mut());
    }
//
// General things
//
pub static mut root_swap: unsigned short = 0;
pub static mut hib_resume_bdev_file: *mut c_void = core::ptr::null_mut();
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hib_bio_batch {
    pub count: core::sync::atomic::AtomicI32,
    pub wait: wait_queue_head_t,
    pub error: blk_status_t,
    pub plug: blk_plug,
}

#[no_mangle]
unsafe extern "C" fn hib_init_batch(hb: *mut hib_bio_batch) {
    atomic_set(&hb.count, 0);
    init_waitqueue_head(&hb.wait);
    hb.error = BLK_STS_OK;
    blk_start_plug(&hb.plug);
    }
#[no_mangle]
unsafe extern "C" fn hib_finish_batch(hb: *mut hib_bio_batch) {
    blk_finish_plug(&hb.plug);
    }
#[no_mangle]
unsafe extern "C" fn hib_end_io(bio: *mut bio) {
    let mut hb = bio.bi_private;
    let mut page = bio_first_page_all(bio);
    if (bio.bi_status) {
    pr_alert("Read-error on swap-device (%u:%u:%Lu)\n",
    MAJOR(bio_dev(bio)), MINOR(bio_dev(bio)),
    (unsigned long long)bio.bi_iter.bi_sector);
    }
    if (bio_data_dir(bio) == WRITE) {
    put_page(page);
    }

    else if (clean_pages_on_read) {
    flush_icache_range((unsigned long)page_address(page),
    (unsigned long)page_address(page) + PAGE_SIZE);
    }
    if (bio.bi_status && !hb.error) {
    hb.error = bio.bi_status;
    }
    if (atomic_dec_and_test(&hb.count)) {
    wake_up(&hb.wait);
    }
    bio_put(bio);
    }
#[no_mangle]
unsafe extern "C" fn hib_submit_io_sync(opf: blk_opf_t, page_off: pgoff_t, addr: *mut c_void) -> c_int {
    return bdev_rw_virt(file_bdev(hib_resume_bdev_file),
    page_off * (PAGE_SIZE >> 9), addr, PAGE_SIZE, opf);
    }
#[no_mangle]
pub unsafe extern "C" fn hib_submit_io_async(opf: blk_opf_t, page_off: pgoff_t, addr: *mut c_void, hb: *mut hib_bio_batch) -> c_int {
pub static mut bio: *mut c_void = core::ptr::null_mut();
    bio = bio_alloc(file_bdev(hib_resume_bdev_file), 1, opf,
    GFP_NOIO | __GFP_HIGH);
    bio.bi_iter.bi_sector = page_off * (PAGE_SIZE >> 9);
    bio_add_virt_nofail(bio, addr, PAGE_SIZE);
    bio.bi_end_io = hib_end_io;
    bio.bi_private = hb;
    atomic_inc(&hb.count);
    submit_bio(bio);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hib_wait_io(hb: *mut hib_bio_batch) -> c_int {
//
// We are relying on the behavior of blk_plug that a thread with
// a plug will flush the plug list before sleeping.
//
    wait_event(hb.wait, atomic_read(&hb.count) == 0);
    return blk_status_to_errno(hb.error);
    }
//
// Saving part
//
#[no_mangle]
unsafe extern "C" fn mark_swapfiles(handle: *mut swap_map_handle, flags: c_uint) -> c_int {
    let mut error = 0;
    hib_submit_io_sync(REQ_OP_READ, swsusp_resume_block, swsusp_header);
    if (!memcmp("SWAP-SPACE",swsusp_header.sig, 10) ||
    !memcmp("SWAPSPACE2",swsusp_header.sig, 10)) {
    memcpy(swsusp_header.orig_sig,swsusp_header.sig, 10);
    memcpy(swsusp_header.sig, HIBERNATE_SIG, 10);
    swsusp_header.image = handle.first_sector;
    if (swsusp_hardware_signature) {
    swsusp_header.hw_sig = swsusp_hardware_signature;
    flags |= SF_HW_SIG;
    }
    swsusp_header.flags = flags;
    if (flags & SF_CRC32_MODE) {
    swsusp_header.crc32 = handle.crc32;
    }
    error = hib_submit_io_sync(REQ_OP_WRITE | REQ_SYNC,
    swsusp_resume_block, swsusp_header);
    } else {
    pr_err!("Swap header not found!\n");
    error = -ENODEV;
    }
    return error;
    }
//
// Hold the swsusp_header flag. This is used in software_resume() in
// 'kernel/power/hibernate' to check if the image is compressed and query
// for the compression algorithm support(if so).
//
    let mut swsusp_header_flags = 0;
#[no_mangle]
unsafe extern "C" fn swsusp_swap_check() -> c_int {
    let mut res = 0;
//
// Check if the resume device is a swap device and get its index (if so).
// This is called before saving the image.
//
    if (swsusp_resume_device) {
    res = find_hibernation_swap_type(swsusp_resume_device, swsusp_resume_block);
    }
    else {
    res = find_first_swap(&swsusp_resume_device);
    }
    if (res < 0) {
    return res;
    }
    root_swap = res;
    hib_resume_bdev_file = bdev_file_open_by_dev(swsusp_resume_device,
    BLK_OPEN_WRITE, core::ptr::null_mut(), core::ptr::null_mut());
    if (IS_ERR(hib_resume_bdev_file)) {
    return PTR_ERR(hib_resume_bdev_file);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn write_page(buf: *mut c_void, offset: sector_t, hb: *mut hib_bio_batch) -> c_int {
pub static mut gfp: gfp_t = 0;
pub static mut src: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!offset) {
    return -ENOSPC;
    }
    if (!hb) {
// goto;
    }
    src = __get_free_page(gfp);
    if (!src) {
    ret = hib_wait_io(hb); /* Free pages */
    if (ret) {
    return ret;
    }
    src = __get_free_page(gfp);
    if (WARN_ON_ONCE!(!src)) {
// goto;
    }
    }
    copy_page(src, buf);
    return hib_submit_io_async(REQ_OP_WRITE | REQ_SYNC, offset, src, hb);
// label;
    return hib_submit_io_sync(REQ_OP_WRITE | REQ_SYNC, offset, buf);
    }
#[no_mangle]
unsafe extern "C" fn release_swap_writer(handle: *mut swap_map_handle) {
    if (handle.cur) {
    free_page((unsigned long)handle.cur);
    }
    handle.cur = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn get_swap_writer(handle: *mut swap_map_handle) -> c_int {
    let mut ret = 0;
    ret = swsusp_swap_check();
    if (ret) {
    if (ret != -ENOSPC) {
    pr_err!("Cannot find swap device, try swapon -a\n");
    }
    return ret;
    }
    handle.cur = get_zeroed_page(GFP_KERNEL);
    if (!handle.cur) {
    ret = -ENOMEM;
// goto;
    }
    handle.cur_swap = alloc_swapdev_block(root_swap);
    if (!handle.cur_swap) {
    ret = -ENOSPC;
// goto;
    }
    handle.k = 0;
    handle.reqd_free_pages = reqd_free_pages();
    handle.first_sector = handle.cur_swap;
    return 0;
// label;
    release_swap_writer(handle);
// label;
    swsusp_close();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn swap_write_page(handle: *mut swap_map_handle, buf: *mut c_void, hb: *mut hib_bio_batch) -> c_int {
    let mut error = 0;
    let mut offset;
    if (!handle.cur) {
    return -EINVAL;
    }
//
// If the current map page is full, allocate and link next one first.
// Delaying this until here avoids writing an empty swap map page when
// the image size is an exact MAP_PAGE_ENTRIES multiple.
//
    if (handle.k >= MAP_PAGE_ENTRIES) {
    offset = alloc_swapdev_block(root_swap);
    if (!offset) {
    return -ENOSPC;
    }
    handle.cur.next_swap = offset;
    error = write_page(handle.cur, handle.cur_swap, hb);
    if (error) {
    return error;
    }
    clear_page(handle.cur);
    handle.cur_swap = offset;
    handle.k = 0;
    if (hb && low_free_pages() <= handle.reqd_free_pages) {
    error = hib_wait_io(hb);
    if (error) {
    return error;
    }
//
// Recalculate the number of required free pages, to
// make sure we never take more than half.
//
    handle.reqd_free_pages = reqd_free_pages();
    }
    }
    offset = alloc_swapdev_block(root_swap);
    error = write_page(buf, offset, hb);
    if (error) {
    return error;
    }
    handle.cur.entries[handle.k++] = offset;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn flush_swap_writer(handle: *mut swap_map_handle) -> c_int {
    if (handle.cur && handle.cur_swap && handle.k) {
    return write_page(handle.cur, handle.cur_swap, core::ptr::null_mut());
    }

    else if (handle.cur && handle.cur_swap) {
    return 0;
    }
    else {
    return -EINVAL;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn swap_writer_finish(handle: *mut swap_map_handle, flags: c_uint, error: c_int) -> c_int {
    if (!error) {
    pr_info!("S");
    error = mark_swapfiles(handle, flags);
    pr_cont("|\n");
    flush_swap_writer(handle);
    }
    if (error) {
    free_all_swap_pages(root_swap);
    }
    release_swap_writer(handle);
    swsusp_close();
    return error;
    }
//
// Bytes we need for compressed data in worst case. We assume(limitation)
// this is the worst of all the compression algorithms.
//

// We need to remember how much compressed data we need to read.

// Number of pages/bytes we'll compress at one time.
pub const UNC_PAGES: c_int = 32;

// Number of pages we need for compressed data (worst case).

    CMP_HEADER, PAGE_SIZE)

// Default number of threads for compression/decompression.
pub const CMP_THREADS: c_int = 3;
pub static mut hibernate_compression_threads: unsigned int = 0;
// Minimum/maximum number of pages for read buffering.
pub const CMP_MIN_RD_PAGES: c_int = 1024;
pub const CMP_MAX_RD_PAGES: c_int = 8192;
#[no_mangle]
pub unsafe extern "C" fn save_image(handle: *mut swap_map_handle, snapshot: *mut snapshot_handle, nr_to_write: c_uint) -> c_int {
    let mut m = 0;
    let mut ret = 0;
    let mut nr_pages = 0;
    let mut err2 = 0;
pub static mut hb: usize = 0;
    let mut start;
    let mut stop;
    hib_init_batch(&hb);
    pr_info!("Saving image data pages (%u pages)...\n",
    nr_to_write);
    m = nr_to_write / 10;
    if (!m) {
    m = 1;
    }
    nr_pages = 0;
    start = ktime_get();
    while (1) {
    ret = snapshot_read_next(snapshot);
    if (ret <= 0) {
    break;
    }
    ret = swap_write_page(handle, data_of(*snapshot), &hb);
    if (ret) {
    break;
    }
    if (!(nr_pages % m)) {
    pr_info!("Image saving progress: %3d%%\n",
    nr_pages / m * 10);
    }
    nr_pages += 1;
    }
    err2 = hib_wait_io(&hb);
    hib_finish_batch(&hb);
    stop = ktime_get();
    if (!ret) {
    ret = err2;
    }
    if (!ret) {
    pr_info!("Image saving done\n");
    }
    swsusp_show_speed(start, stop, nr_to_write, "Wrote");
    return ret;
    }
//
// Structure used for CRC32.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crc_data {
//     pub /: *mut *mut *mut task_thr; / thread,
//     pub /: *mut *mut atomic_t ready; / ready to start flag,
//     pub /: *mut *mut atomic_t stop; / ready to stop flag,
//     pub /: *mut *mut unsigned run_threads; / nr current threads,
//     pub /: *mut *mut wait_queue_head_t go; / start crc update,
//     pub /: *mut *mut wait_queue_head_t done; / crc update done,
//     pub /: *mut *mut *mut u32 crc32; / points to handle's crc32,
//     pub /: *mut *mut *mut *mut size_t unc_len; / uncompressed lengths,
//     pub /: *mut *mut *mut unsigned char unc[]; / uncompressed data,
}

#[no_mangle]
pub unsafe extern "C" fn alloc_crc_data(nr_threads: c_int) -> *mut c_void {
pub static mut crc: *mut c_void = core::ptr::null_mut();
    crc = kzalloc_flex(*crc, unc, nr_threads);
    if (!crc) {
    return core::ptr::null_mut();
    }
    crc.unc_len = kzalloc_objs(*crc.unc_len, nr_threads);
    if (!crc.unc_len) {
// goto;
    }
    return crc;
// label;
    kfree(crc);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn free_crc_data(crc: *mut crc_data) {
    if (!crc) {
    return;
    }
    if (crc.thr) {
    kthread_stop(crc.thr);
    }
    kfree(crc.unc_len);
    kfree(crc);
    }
#[no_mangle]
unsafe extern "C" fn crc32_threadfn(data: *mut c_void) -> c_int {
    let mut d = data;
    let mut i: c_uint = 0;
    while (1) {
    wait_event(d.go, atomic_read_acquire(&d.ready) ||
    kthread_should_stop());
    if (kthread_should_stop()) {
    d.thr = core::ptr::null_mut();
    atomic_set_release(&d.stop, 1);
    wake_up(&d.done);
    break;
    }
    atomic_set(&d.ready, 0);
    for (i = 0; i < d.run_threads; i++) {
// d->crc32 = crc32_le(*d->crc32,
    d.unc[i], *d.unc_len[i]);
    }
    atomic_set_release(&d.stop, 1);
    wake_up(&d.done);
    }
    return 0;
    }
//
// Structure used for data compression.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmp_data {
//     pub /: *mut *mut *mut task_thr; / thread,
//     pub /: *mut *mut *mut crypto_acomp cc; / crypto compressor,
//     pub /: *mut *mut *mut acomp_req cr; / crypto request,
//     pub /: *mut *mut atomic_t ready; / ready to start flag,
//     pub /: *mut *mut atomic_t stop; / ready to stop flag,
//     pub /: *mut *mut int ret; / return code,
//     pub /: *mut *mut wait_queue_head_t go; / start compression,
//     pub /: *mut *mut wait_queue_head_t done; / compression done,
//     pub /: *mut *mut size_t unc_len; / uncompressed length,
//     pub /: *mut *mut size_t cmp_len; / compressed length,
//     pub /: *mut *mut unsigned char unc[UNC_SIZE]; / uncompressed buffer,
//     pub /: *mut *mut unsigned char cmp[CMP_SIZE]; / compressed buffer,
}

// Indicates the image size after compression
pub static mut compressed_size: atomic64_t = 0;
#[no_mangle]
unsafe extern "C" fn compress_threadfn(data: *mut c_void) -> c_int {
    let mut d = data;
    while (1) {
    wait_event(d.go, atomic_read_acquire(&d.ready) ||
    kthread_should_stop());
    if (kthread_should_stop()) {
    d.thr = core::ptr::null_mut();
    d.ret = -1;
    atomic_set_release(&d.stop, 1);
    wake_up(&d.done);
    break;
    }
    atomic_set(&d.ready, 0);
    acomp_request_set_callback(d.cr, CRYPTO_TFM_REQ_MAY_SLEEP,
    core::ptr::null_mut(), core::ptr::null_mut());
    acomp_request_set_src_nondma(d.cr, d.unc, d.unc_len);
    acomp_request_set_dst_nondma(d.cr, d.cmp + CMP_HEADER,
    CMP_SIZE - CMP_HEADER);
    d.ret = crypto_acomp_compress(d.cr);
    d.cmp_len = d.cr.dlen;
    atomic64_add(d.cmp_len, &compressed_size);
    atomic_set_release(&d.stop, 1);
    wake_up(&d.done);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn save_compressed_image(handle: *mut swap_map_handle, snapshot: *mut snapshot_handle, nr_to_write: c_uint) -> c_int {
    let mut m = 0;
pub static mut ret: c_int = 0;
    let mut nr_pages = 0;
    let mut err2 = 0;
pub static mut hb: usize = 0;
    let mut start;
    let mut stop;
    let mut off = 0;
    let mut thr = 0;
    let mut run_threads = 0;
    let mut nr_threads = 0;
    let mut page = core::ptr::null_mut();
    let mut data = core::ptr::null_mut();
    let mut crc = core::ptr::null_mut();
    hib_init_batch(&hb);
    atomic64_set(&compressed_size, 0);
//
// We'll limit the number of threads for compression to limit memory
// footprint.
//
    nr_threads = num_online_cpus() - 1;
    nr_threads = clamp_val(nr_threads, 1, hibernate_compression_threads);
    page = __get_free_page(GFP_NOIO | __GFP_HIGH);
    if (!page) {
    pr_err!("Failed to allocate %s page\n", hib_comp_algo);
    ret = -ENOMEM;
// goto;
    }
    data = vcalloc(nr_threads, sizeof!(*data));
    if (!data) {
    pr_err!("Failed to allocate %s data\n", hib_comp_algo);
    ret = -ENOMEM;
// goto;
    }
    crc = alloc_crc_data(nr_threads);
    if (!crc) {
    pr_err!("Failed to allocate crc\n");
    ret = -ENOMEM;
// goto;
    }
//
// Start the compression threads.
//
    while (thr < nr_threads) {
    init_waitqueue_head(&data[thr].go);
    init_waitqueue_head(&data[thr].done);
    data[thr].cc = crypto_alloc_acomp(hib_comp_algo, 0, CRYPTO_ALG_ASYNC);
    if (IS_ERR_OR_NULL(data[thr].cc)) {
    pr_err!("Could not allocate comp stream %pe\n", data[thr].cc);
    ret = -EFAULT;
// goto;
    }
    data[thr].cr = acomp_request_alloc(data[thr].cc);
    if (!data[thr].cr) {
    pr_err!("Could not allocate comp request\n");
    ret = -ENOMEM;
// goto;
    }
    data[thr].thr = kthread_run(compress_threadfn,
    &data[thr],
    "image_compress/%u", thr);
    if (IS_ERR(data[thr].thr)) {
    data[thr].thr = core::ptr::null_mut();
    pr_err!("Cannot start compression threads\n");
    ret = -ENOMEM;
// goto;
    }
    }
//
// Start the CRC32 thread.
//
    init_waitqueue_head(&crc.go);
    init_waitqueue_head(&crc.done);
    handle.crc32 = 0;
    crc.crc32 = &handle.crc32;
    while (thr < nr_threads) {
    crc.unc[thr] = data[thr].unc;
    crc.unc_len[thr] = &data[thr].unc_len;
    }
    crc.thr = kthread_run(crc32_threadfn, crc, "image_crc32");
    if (IS_ERR(crc.thr)) {
    crc.thr = core::ptr::null_mut();
    pr_err!("Cannot start CRC32 thread\n");
    ret = -ENOMEM;
// goto;
    }
//
// Adjust the number of required free pages after all allocations have
// been done. We don't want to run out of pages when writing.
//
    handle.reqd_free_pages = reqd_free_pages();
    pr_info!("Using %u thread(s) for %s compression\n", nr_threads, hib_comp_algo);
    pr_info!("Compressing and saving image data (%u pages)...\n",
    nr_to_write);
    m = nr_to_write / 10;
    if (!m) {
    m = 1;
    }
    nr_pages = 0;
    start = ktime_get();
    for (;;) {
    while (thr < nr_threads) {
    while (off < UNC_SIZE) {
    ret = snapshot_read_next(snapshot);
    if (ret < 0) {
// goto;
    }
    if (!ret) {
    break;
    }
    memcpy(data[thr].unc + off,
    data_of(*snapshot), PAGE_SIZE);
    if (!(nr_pages % m)) {
    pr_info!("Image saving progress: %3d%%\n",
    nr_pages / m * 10);
    }
    nr_pages += 1;
    }
    if (!off) {
    break;
    }
    data[thr].unc_len = off;
    atomic_set_release(&data[thr].ready, 1);
    wake_up(&data[thr].go);
    }
    if (!thr) {
    break;
    }
    crc.run_threads = thr;
    atomic_set_release(&crc.ready, 1);
    wake_up(&crc.go);
    while (thr < run_threads) {
    wait_event(data[thr].done,
    atomic_read_acquire(&data[thr].stop));
    atomic_set(&data[thr].stop, 0);
    ret = data[thr].ret;
    if (ret < 0) {
    pr_err!("%s compression failed\n", hib_comp_algo);
// goto;
    }
    if (unlikely(!data[thr].cmp_len ||
    data[thr].cmp_len >
    bytes_worst_compress(data[thr].unc_len))) {
    pr_err!("Invalid %s compressed length\n", hib_comp_algo);
    ret = -1;
// goto;
    }
// data[thr].cmp = data[thr].cmp_len;
//
// Given we are writing one page at a time to disk, we
// copy that much from the buffer, although the last
// bit will likely be smaller than full page. This is
// OK - we saved the length of the compressed data, so
// any garbage at the end will be discarded when we
// read it.
//
    while (off < CMP_HEADER + data[thr].cmp_len) {
    memcpy(page, data[thr].cmp + off, PAGE_SIZE);
    ret = swap_write_page(handle, page, &hb);
    if (ret) {
// goto;
    }
    }
    }
    wait_event(crc.done, atomic_read_acquire(&crc.stop));
    atomic_set(&crc.stop, 0);
    }
// label;
    err2 = hib_wait_io(&hb);
    stop = ktime_get();
    if (!ret) {
    ret = err2;
    }
    if (!ret) {
    swsusp_show_speed(start, stop, nr_to_write, "Wrote");
    pr_info!("Image size after compression: %lld kbytes\n",
    (atomic64_read(&compressed_size) / 1024));
    pr_info!("Image saving done\n");
    } else {
    pr_err!("Image saving failed: %d\n", ret);
    }
// label;
    hib_finish_batch(&hb);
    free_crc_data(crc);
    if (data) {
    while (thr < nr_threads) {
    if (data[thr].thr) {
    kthread_stop(data[thr].thr);
    }
    acomp_request_free(data[thr].cr);
    if (!IS_ERR_OR_NULL(data[thr].cc)) {
    crypto_free_acomp(data[thr].cc);
    }
    }
    vfree(data);
    }
    if (page) {
    free_page((unsigned long)page);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn enough_swap(nr_pages: c_uint) -> c_int {
pub static mut free_swap: c_uint = 0;
    let mut required = 0;
    pr_debug!("Free swap pages: %u\n", free_swap);
    required = PAGES_FOR_IO + nr_pages;
    return free_swap > required;
    }
//
// swsusp_write - Write entire image and metadata.
// @flags: flags to pass to the "boot" kernel in the image header
//
// It is important _NOT_ to umount filesystems at this point. We want them
// synced (in case something goes wrong) but we DO not want to mark filesystem
// clean: it is not. (And it does not matter, if we resume correctly, we'll mark
// system clean, anyway.)
//
// Return: 0 on success, negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn swsusp_write(flags: c_uint) -> c_int {
pub static mut handle: usize = 0;
pub static mut snapshot: usize = 0;
pub static mut header: *mut c_void = core::ptr::null_mut();
    let mut pages = 0;
    let mut error = 0;
    pages = snapshot_get_image_size();
    error = get_swap_writer(&handle);
    if (error) {
    pr_err!("Cannot get swap writer\n");
    return error;
    }
    if (flags & SF_NOCOMPRESS_MODE) {
    if (!enough_swap(pages)) {
    pr_err!("Not enough free swap\n");
    error = -ENOSPC;
// goto;
    }
    }
    memset(&snapshot, 0, sizeof!(snapshot_handle));
    error = snapshot_read_next(&snapshot);
    if (error < (int)PAGE_SIZE) {
    if (error >= 0) {
    error = -EFAULT;
    }
// goto;
    }
    header = data_of(snapshot);
    error = swap_write_page(&handle, header, core::ptr::null_mut());
    if (!error) {
    error = (flags & SF_NOCOMPRESS_MODE) ?
    save_image(&handle, &snapshot, pages - 1) :
    save_compressed_image(&handle, &snapshot, pages - 1);
    }
// label;
    error = swap_writer_finish(&handle, flags, error);
    return error;
    }
//
// The following functions allow us to read data using a swap map in a file-like
// way.
//
#[no_mangle]
unsafe extern "C" fn release_swap_reader(handle: *mut swap_map_handle) {
pub static mut tmp: *mut c_void = core::ptr::null_mut();
    while (handle.maps) {
    if (handle.maps.map) {
    free_page((unsigned long)handle.maps.map);
    }
    tmp = handle.maps;
    handle.maps = handle.maps.next;
    kfree(tmp);
    }
    handle.cur = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn get_swap_reader(handle: *mut swap_map_handle, flags_p: *mut c_uint) -> c_int {
    let mut error = 0;
    let mut tmp = core::ptr::null_mut();
    let mut last = core::ptr::null_mut();
    let mut offset;
// flags_p = swsusp_header->flags;
    if (!swsusp_header.image) /* how can this happen? */ {
    return -EINVAL;
    }
    handle.cur = core::ptr::null_mut();
    last = handle.maps = core::ptr::null_mut();
    offset = swsusp_header.image;
    while (offset) {
    tmp = kzalloc_obj(*handle.maps);
    if (!tmp) {
    release_swap_reader(handle);
    return -ENOMEM;
    }
    if (!handle.maps) {
    handle.maps = tmp;
    }
    if (last) {
    last.next = tmp;
    }
    last = tmp;
    tmp.map = 
    __get_free_page(GFP_NOIO | __GFP_HIGH);
    if (!tmp.map) {
    release_swap_reader(handle);
    return -ENOMEM;
    }
    error = hib_submit_io_sync(REQ_OP_READ, offset, tmp.map);
    if (error) {
    release_swap_reader(handle);
    return error;
    }
    offset = tmp.map.next_swap;
    }
    handle.k = 0;
    handle.cur = handle.maps.map;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn swap_read_page(handle: *mut swap_map_handle, buf: *mut c_void, hb: *mut hib_bio_batch) -> c_int {
    let mut offset;
    let mut error = 0;
pub static mut tmp: *mut c_void = core::ptr::null_mut();
    if (!handle.cur) {
    return -EINVAL;
    }
    offset = handle.cur.entries[handle.k];
    if (!offset) {
    return -EFAULT;
    }
    if (hb) {
    error = hib_submit_io_async(REQ_OP_READ, offset, buf, hb);
    }
    else {
    error = hib_submit_io_sync(REQ_OP_READ, offset, buf);
    }
    if (error) {
    return error;
    }
    if (++handle.k >= MAP_PAGE_ENTRIES) {
    handle.k = 0;
    free_page((unsigned long)handle.maps.map);
    tmp = handle.maps;
    handle.maps = handle.maps.next;
    kfree(tmp);
    if (!handle.maps) {
    release_swap_reader(handle);
    }
    else {
    handle.cur = handle.maps.map;
    }
    }
    return error;
    }
#[no_mangle]
unsafe extern "C" fn swap_reader_finish(handle: *mut swap_map_handle) -> c_int {
    release_swap_reader(handle);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn load_image(handle: *mut swap_map_handle, snapshot: *mut snapshot_handle, nr_to_read: c_uint) -> c_int {
    let mut m = 0;
pub static mut ret: c_int = 0;
    let mut start;
    let mut stop;
pub static mut hb: usize = 0;
    let mut err2 = 0;
    let mut nr_pages: c_uint = 0;
    hib_init_batch(&hb);
    clean_pages_on_read = true;
    pr_info!("Loading image data pages (%u pages)...\n", nr_to_read);
    m = nr_to_read / 10;
    if (!m) {
    m = 1;
    }
    nr_pages = 0;
    start = ktime_get();
    while ( ) {
    ret = snapshot_write_next(snapshot);
    if (ret <= 0) {
    break;
    }
    ret = swap_read_page(handle, data_of(*snapshot), &hb);
    if (ret) {
    break;
    }
    if (snapshot.sync_read) {
    ret = hib_wait_io(&hb);
    }
    if (ret) {
    break;
    }
    if (!(nr_pages % m)) {
    pr_info!("Image loading progress: %3d%%\n",
    nr_pages / m * 10);
    }
    nr_pages += 1;
    }
    err2 = hib_wait_io(&hb);
    hib_finish_batch(&hb);
    stop = ktime_get();
    if (!ret) {
    ret = err2;
    }
    if (!ret) {
    pr_info!("Image loading done\n");
    ret = snapshot_write_finalize(snapshot);
    if (!ret && !snapshot_image_loaded(snapshot)) {
    ret = -ENODATA;
    }
    }
    swsusp_show_speed(start, stop, nr_to_read, "Read");
    return ret;
    }
//
// Structure used for data decompression.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dec_data {
//     pub /: *mut *mut *mut task_thr; / thread,
//     pub /: *mut *mut *mut crypto_acomp cc; / crypto compressor,
//     pub /: *mut *mut *mut acomp_req cr; / crypto request,
//     pub /: *mut *mut atomic_t ready; / ready to start flag,
//     pub /: *mut *mut atomic_t stop; / ready to stop flag,
//     pub /: *mut *mut int ret; / return code,
//     pub /: *mut *mut wait_queue_head_t go; / start decompression,
//     pub /: *mut *mut wait_queue_head_t done; / decompression done,
//     pub /: *mut *mut size_t unc_len; / uncompressed length,
//     pub /: *mut *mut size_t cmp_len; / compressed length,
//     pub /: *mut *mut unsigned char unc[UNC_SIZE]; / uncompressed buffer,
//     pub /: *mut *mut unsigned char cmp[CMP_SIZE]; / compressed buffer,
}

#[no_mangle]
unsafe extern "C" fn decompress_threadfn(data: *mut c_void) -> c_int {
    let mut d = data;
    while (1) {
    wait_event(d.go, atomic_read_acquire(&d.ready) ||
    kthread_should_stop());
    if (kthread_should_stop()) {
    d.thr = core::ptr::null_mut();
    d.ret = -1;
    atomic_set_release(&d.stop, 1);
    wake_up(&d.done);
    break;
    }
    atomic_set(&d.ready, 0);
    acomp_request_set_callback(d.cr, CRYPTO_TFM_REQ_MAY_SLEEP,
    core::ptr::null_mut(), core::ptr::null_mut());
    acomp_request_set_src_nondma(d.cr, d.cmp + CMP_HEADER,
    d.cmp_len);
    acomp_request_set_dst_nondma(d.cr, d.unc, UNC_SIZE);
    d.ret = crypto_acomp_decompress(d.cr);
    d.unc_len = d.cr.dlen;
    if (clean_pages_on_decompress) {
    flush_icache_range((unsigned long)d.unc,
    (unsigned long)d.unc + d.unc_len);
    }
    atomic_set_release(&d.stop, 1);
    wake_up(&d.done);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn load_compressed_image(handle: *mut swap_map_handle, snapshot: *mut snapshot_handle, nr_to_read: c_uint) -> c_int {
    let mut m = 0;
pub static mut ret: c_int = 0;
pub static mut eof: c_int = 0;
pub static mut hb: usize = 0;
    let mut start;
    let mut stop;
    let mut nr_pages: c_uint = 0;
    let mut off = 0;
    let mut i = 0;
    let mut thr = 0;
    let mut run_threads = 0;
    let mut nr_threads = 0;
    unsigned ring = 0, pg = 0, ring_size = 0,
    have = 0, want, need, asked = 0;
pub static mut read_pages: c_ulong = 0;
    let mut page = core::ptr::null_mut();
    let mut data = core::ptr::null_mut();
    let mut crc = core::ptr::null_mut();
    hib_init_batch(&hb);
//
// We'll limit the number of threads for decompression to limit memory
// footprint.
//
    nr_threads = num_online_cpus() - 1;
    nr_threads = clamp_val(nr_threads, 1, hibernate_compression_threads);
    page = vmalloc_array(CMP_MAX_RD_PAGES, sizeof!(*page));
    if (!page) {
    pr_err!("Failed to allocate %s page\n", hib_comp_algo);
    ret = -ENOMEM;
// goto;
    }
    data = vcalloc(nr_threads, sizeof!(*data));
    if (!data) {
    pr_err!("Failed to allocate %s data\n", hib_comp_algo);
    ret = -ENOMEM;
// goto;
    }
    crc = alloc_crc_data(nr_threads);
    if (!crc) {
    pr_err!("Failed to allocate crc\n");
    ret = -ENOMEM;
// goto;
    }
    clean_pages_on_decompress = true;
//
// Start the decompression threads.
//
    while (thr < nr_threads) {
    init_waitqueue_head(&data[thr].go);
    init_waitqueue_head(&data[thr].done);
    data[thr].cc = crypto_alloc_acomp(hib_comp_algo, 0, CRYPTO_ALG_ASYNC);
    if (IS_ERR_OR_NULL(data[thr].cc)) {
    pr_err!("Could not allocate comp stream %pe\n", data[thr].cc);
    ret = -EFAULT;
// goto;
    }
    data[thr].cr = acomp_request_alloc(data[thr].cc);
    if (!data[thr].cr) {
    pr_err!("Could not allocate comp request\n");
    ret = -ENOMEM;
// goto;
    }
    data[thr].thr = kthread_run(decompress_threadfn,
    &data[thr],
    "image_decompress/%u", thr);
    if (IS_ERR(data[thr].thr)) {
    data[thr].thr = core::ptr::null_mut();
    pr_err!("Cannot start decompression threads\n");
    ret = -ENOMEM;
// goto;
    }
    }
//
// Start the CRC32 thread.
//
    init_waitqueue_head(&crc.go);
    init_waitqueue_head(&crc.done);
    handle.crc32 = 0;
    crc.crc32 = &handle.crc32;
    while (thr < nr_threads) {
    crc.unc[thr] = data[thr].unc;
    crc.unc_len[thr] = &data[thr].unc_len;
    }
    crc.thr = kthread_run(crc32_threadfn, crc, "image_crc32");
    if (IS_ERR(crc.thr)) {
    crc.thr = core::ptr::null_mut();
    pr_err!("Cannot start CRC32 thread\n");
    ret = -ENOMEM;
// goto;
    }
//
// Set the number of pages for read buffering.
// This is complete guesswork, because we'll only know the real
// picture once prepare_image() is called, which is much later on
// during the image load phase. We'll assume the worst case and
// say that none of the image pages are from high memory.
//
    if (low_free_pages() > snapshot_get_image_size()) {
    read_pages = (low_free_pages() - snapshot_get_image_size()) / 2;
    }
    read_pages = clamp_val(read_pages, CMP_MIN_RD_PAGES, CMP_MAX_RD_PAGES);
    while (i < read_pages) {
    page[i] = __get_free_page(i < CMP_PAGES ?
    GFP_NOIO | __GFP_HIGH :
    GFP_NOIO | __GFP_NOWARN |
    __GFP_NORETRY);
    if (!page[i]) {
    if (i < CMP_PAGES) {
    ring_size = i;
    pr_err!("Failed to allocate %s pages\n", hib_comp_algo);
    ret = -ENOMEM;
// goto;
    } else {
    break;
    }
    }
    }
    want = ring_size = i;
    pr_info!("Using %u thread(s) for %s decompression\n", nr_threads, hib_comp_algo);
    pr_info!("Loading and decompressing image data (%u pages)...\n",
    nr_to_read);
    m = nr_to_read / 10;
    if (!m) {
    m = 1;
    }
    nr_pages = 0;
    start = ktime_get();
    ret = snapshot_write_next(snapshot);
    if (ret <= 0) {
// goto;
    }
    for(;;) {
    while (!eof && i < want) {
    ret = swap_read_page(handle, page[ring], &hb);
    if (ret) {
//
// On real read error, finish. On end of data,
// set EOF flag and just exit the read loop.
//
    if (handle.cur &&
    handle.cur.entries[handle.k]) {
// goto;
    } else {
    eof = 1;
    break;
    }
    }
    if (++ring >= ring_size) {
    ring = 0;
    }
    }
    asked += i;
    want -= i;
//
// We are out of data, wait for some more.
//
    if (!have) {
    if (!asked) {
    break;
    }
    ret = hib_wait_io(&hb);
    if (ret) {
// goto;
    }
    have += asked;
    asked = 0;
    if (eof) {
    eof = 2;
    }
    }
    if (crc.run_threads) {
    wait_event(crc.done, atomic_read_acquire(&crc.stop));
    atomic_set(&crc.stop, 0);
    crc.run_threads = 0;
    }
    while (have && thr < nr_threads) {
    data[thr].cmp_len = *page[pg];
    if (unlikely(!data[thr].cmp_len ||
    data[thr].cmp_len >
    bytes_worst_compress(UNC_SIZE))) {
    pr_err!("Invalid %s compressed length\n", hib_comp_algo);
    ret = -1;
// goto;
    }
    need = DIV_ROUND_UP(data[thr].cmp_len + CMP_HEADER,
    PAGE_SIZE);
    if (need > have) {
    if (eof > 1) {
    ret = -1;
// goto;
    }
    break;
    }
    while (off < CMP_HEADER + data[thr].cmp_len) {
    memcpy(data[thr].cmp + off,
    page[pg], PAGE_SIZE);
    have -= 1;
    want += 1;
    if (++pg >= ring_size) {
    pg = 0;
    }
    }
    atomic_set_release(&data[thr].ready, 1);
    wake_up(&data[thr].go);
    }
//
// Wait for more data while we are decompressing.
//
    if (have < CMP_PAGES && asked) {
    ret = hib_wait_io(&hb);
    if (ret) {
// goto;
    }
    have += asked;
    asked = 0;
    if (eof) {
    eof = 2;
    }
    }
    while (thr < run_threads) {
    wait_event(data[thr].done,
    atomic_read_acquire(&data[thr].stop));
    atomic_set(&data[thr].stop, 0);
    ret = data[thr].ret;
    if (ret < 0) {
    pr_err!("%s decompression failed\n", hib_comp_algo);
// goto;
    }
    if (unlikely(!data[thr].unc_len ||
    data[thr].unc_len > UNC_SIZE ||
    data[thr].unc_len & (PAGE_SIZE - 1))) {
    pr_err!("Invalid %s uncompressed length\n", hib_comp_algo);
    ret = -1;
// goto;
    }
    while (off < data[thr].unc_len) {
    memcpy(data_of(*snapshot),
    data[thr].unc + off, PAGE_SIZE);
    if (!(nr_pages % m)) {
    pr_info!("Image loading progress: %3d%%\n",
    nr_pages / m * 10);
    }
    nr_pages += 1;
    ret = snapshot_write_next(snapshot);
    if (ret <= 0) {
    crc.run_threads = thr + 1;
    atomic_set_release(&crc.ready, 1);
    wake_up(&crc.go);
// goto;
    }
    }
    }
    crc.run_threads = thr;
    atomic_set_release(&crc.ready, 1);
    wake_up(&crc.go);
    }
// label;
    if (crc.run_threads) {
    wait_event(crc.done, atomic_read_acquire(&crc.stop));
    atomic_set(&crc.stop, 0);
    }
    stop = ktime_get();
    if (!ret) {
    pr_info!("Image loading done\n");
    ret = snapshot_write_finalize(snapshot);
    if (!ret && !snapshot_image_loaded(snapshot)) {
    ret = -ENODATA;
    }
    if (!ret) {
    if (swsusp_header.flags & SF_CRC32_MODE) {
    if(handle.crc32 != swsusp_header.crc32) {
    pr_err!("Invalid image CRC32!\n");
    ret = -ENODATA;
    }
    }
    }
    }
    swsusp_show_speed(start, stop, nr_to_read, "Read");
// label;
    hib_finish_batch(&hb);
    for (i = 0; i < ring_size; i++) {
    free_page((unsigned long)page[i]);
    }
    free_crc_data(crc);
    if (data) {
    while (thr < nr_threads) {
    if (data[thr].thr) {
    kthread_stop(data[thr].thr);
    }
    acomp_request_free(data[thr].cr);
    if (!IS_ERR_OR_NULL(data[thr].cc)) {
    crypto_free_acomp(data[thr].cc);
    }
    }
    vfree(data);
    }
    vfree(page);
    return ret;
    }
//
// swsusp_read - read the hibernation image.
// @flags_p: flags passed by the "frozen" kernel in the image header should
// be written into this memory location
//
// Return: 0 on success, negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn swsusp_read(flags_p: *mut c_uint) -> c_int {
    let mut error = 0;
pub static mut handle: usize = 0;
pub static mut snapshot: usize = 0;
pub static mut header: *mut c_void = core::ptr::null_mut();
    memset(&snapshot, 0, sizeof!(snapshot_handle));
    error = snapshot_write_next(&snapshot);
    if (error < (int)PAGE_SIZE) {
    return error < 0 ? error : -EFAULT;
    }
    header = data_of(snapshot);
    error = get_swap_reader(&handle, flags_p);
    if (error) {
// goto;
    }
    if (!error) {
    error = swap_read_page(&handle, header, core::ptr::null_mut());
    }
    if (!error) {
    error = (*flags_p & SF_NOCOMPRESS_MODE) ?
    load_image(&handle, &snapshot, header.pages - 1) :
    load_compressed_image(&handle, &snapshot, header.pages - 1);
    }
    swap_reader_finish(&handle);
// label;
    if (!error) {
    pr_debug!("Image successfully loaded\n");
    }
    else {
    pr_debug!("Error %d resuming\n", error);
    }
    return error;
    }
pub static mut swsusp_holder: *mut c_void = core::ptr::null_mut();
//
// swsusp_check - Open the resume device and check for the swsusp signature.
// @exclusive: Open the resume device exclusively.
//
// Return: 0 if a valid image is found, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn swsusp_check(exclusive: bool) -> c_int {
    let mut holder = exclusive ? &swsusp_holder : core::ptr::null_mut();
    let mut error = 0;
    hib_resume_bdev_file = bdev_file_open_by_dev(swsusp_resume_device,
    BLK_OPEN_READ, holder, core::ptr::null_mut());
    if (!IS_ERR(hib_resume_bdev_file)) {
    clear_page(swsusp_header);
    error = hib_submit_io_sync(REQ_OP_READ, swsusp_resume_block,
    swsusp_header);
    if (error) {
// goto;
    }
    if (!memcmp(HIBERNATE_SIG, swsusp_header.sig, 10)) {
    memcpy(swsusp_header.sig, swsusp_header.orig_sig, 10);
    swsusp_header_flags = swsusp_header.flags;
// Reset swap signature now
    error = hib_submit_io_sync(REQ_OP_WRITE | REQ_SYNC,
    swsusp_resume_block,
    swsusp_header);
    } else {
    error = -EINVAL;
    }
    if (!error && swsusp_header.flags & SF_HW_SIG &&
    swsusp_header.hw_sig != swsusp_hardware_signature) {
    pr_info!("Suspend image hardware signature mismatch (%08x now %08x); aborting resume.\n",
    swsusp_header.hw_sig, swsusp_hardware_signature);
    error = -EINVAL;
    }
// label;
    if (error) {
    bdev_fput(hib_resume_bdev_file);
    }
    else {
    pr_debug!("Image signature found, resuming\n");
    }
    } else {
    error = PTR_ERR(hib_resume_bdev_file);
    }
    if (error) {
    pr_debug!("Image not found (code %d)\n", error);
    }
    return error;
    }
//
// swsusp_close - close resume device.
//
#[no_mangle]
pub unsafe extern "C" fn swsusp_close() {
    if (IS_ERR(hib_resume_bdev_file)) {
    pr_debug!("Image device not initialised\n");
    return;
    }
    fput(hib_resume_bdev_file);
    }
//
// swsusp_unmark - Unmark swsusp signature in the resume device
//
// Return: 0 on success, negative error code on failure.
//

#[no_mangle]
pub unsafe extern "C" fn swsusp_unmark() -> c_int {
    let mut error = 0;
    hib_submit_io_sync(REQ_OP_READ, swsusp_resume_block, swsusp_header);
    if (!memcmp(HIBERNATE_SIG,swsusp_header.sig, 10)) {
    memcpy(swsusp_header.sig,swsusp_header.orig_sig, 10);
    error = hib_submit_io_sync(REQ_OP_WRITE | REQ_SYNC,
    swsusp_resume_block,
    swsusp_header);
    } else {
    pr_err!("Cannot find swsusp signature!\n");
    error = -ENODEV;
    }
//
// We just returned from suspend, we don't need the image any more.
//
    free_all_swap_pages(root_swap);
    return error;
    }

#[no_mangle]
pub unsafe extern "C" fn hibernate_compression_threads_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%d\n", hibernate_compression_threads);
    }
#[no_mangle]
pub unsafe extern "C" fn hibernate_compression_threads_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
    let mut val = 0;
    if (kstrtoul(buf, 0, &val)) {
    return -EINVAL;
    }
    if (val < 1) {
    return -EINVAL;
    }
    hibernate_compression_threads = val;
    return n;
    }
    power_attr(hibernate_compression_threads);
    static struct attribute *g[] = {
    &hibernate_compression_threads_attr.attr,
    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
#[no_mangle]
unsafe extern "C" fn swsusp_header_init() -> c_int {
    let mut error = 0;
    error = sysfs_create_group(power_kobj, &attr_group);
    if (error) {
    return -ENOMEM;
    }
    swsusp_header = (swsusp_header*) __get_free_page(GFP_KERNEL);
    if (!swsusp_header) {
    panic("Could not allocate memory for swsusp_header\n");
    }
    return 0;
    }
    core_initcall!(swsusp_header_init);
#[no_mangle]
unsafe extern "C" fn hibernate_compression_threads_setup(str: *mut c_char) -> c_int {
pub static mut rc: c_int = 0;
    if (rc) {
    return rc;
    }
    if (hibernate_compression_threads < 1) {
    hibernate_compression_threads = CMP_THREADS;
    }
    return 1;
    }
    __setup!("hibernate_compression_threads=", hibernate_compression_threads_setup);