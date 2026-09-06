//! Automatically rewritten from C to Rust
//! Source: kernel/module/decompress.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2021 Google LLC.
//

#[no_mangle]
unsafe extern "C" fn module_extend_max_pages!(info: *mut load_info, extent: c_uint) -> c_int {
pub static mut new_pages: *mut c_void = core::ptr::null_mut();
pub static mut new_max: c_uint = 0;
    new_pages = kvrealloc(info.pages,
    size_mul(new_max, sizeof!(*info.pages)),
    GFP_KERNEL);
    if (!new_pages) {
    return -ENOMEM;
    }
    info.pages = new_pages;
    info.max_pages = new_max;
    return 0;
    }
    static struct page *module_get_next_page!(load_info *info)
    {
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut error = 0;
    if (info.max_pages == info.used_pages) {
    error = module_extend_max_pages!(info, info.used_pages);
    if (error) {
    return ERR_PTR(error);
    }
    }
    page = alloc_page(GFP_KERNEL | __GFP_HIGHMEM);
    if (!page) {
    return ERR_PTR(-ENOMEM);
    }
    info.pages[info.used_pages++] = page;
    return page;
    }

//
// Calculate length of the header which consists of signature, header
// flags, time stamp and operating system ID (10 bytes total), plus
// an optional filename.
//
#[no_mangle]
unsafe extern "C" fn module_gzip_header_len!(buf: *const u8, size: usize) -> usize {
    const u8 signature[] = { 0x1f, 0x8b, 0x08 };
pub static mut len: usize = 10;
    if (size < len || memcmp(buf, signature, sizeof!(signature))) {
    return 0;
    }
    if (buf[3] & 0x08) {
    do {
//
// If we can't find the end of the file name we must
// be dealing with a corrupted file.
//
    if (len == size) {
    return 0;
    }
    } while (buf[len++] != '\0');
    }
    return len;
    }
    static ssize_t module_gzip_decompress!(load_info *info,
    const void *buf, size_t size)
    {
pub static mut s: z_stream_s = 0;
pub static mut new_size: usize = 0;
    let mut gzip_hdr_len = 0;
    let mut retval = 0;
    let mut rc = 0;
    gzip_hdr_len = module_gzip_header_len!(buf, size);
    if (!gzip_hdr_len) {
    pr_err!("not a gzip compressed module\n");
    return -EINVAL;
    }
    s.next_in = buf + gzip_hdr_len;
    s.avail_in = size - gzip_hdr_len;
    s.workspace = kvmalloc(zlib_inflate_workspacesize(), GFP_KERNEL);
    if (!s.workspace) {
    return -ENOMEM;
    }
    rc = zlib_inflateInit2(&s, -MAX_WBITS);
    if (rc != Z_OK) {
    pr_err!("failed to initialize decompressor: %d\n", rc);
    retval = -EINVAL;
// goto;
    }
    do {
    let mut page = module_get_next_page!(info);
    if (IS_ERR(page)) {
    retval = PTR_ERR(page);
// goto;
    }
    s.next_out = kmap_local_page(page);
    s.avail_out = PAGE_SIZE;
    rc = zlib_inflate(&s, 0);
    kunmap_local(s.next_out);
    new_size += PAGE_SIZE - s.avail_out;
    } while (rc == Z_OK);
    if (rc != Z_STREAM_END) {
    pr_err!("decompression failed with status %d\n", rc);
    retval = -EINVAL;
// goto;
    }
    retval = new_size;
// label;
    zlib_inflateEnd(&s);
// label;
    kvfree(s.workspace);
    return retval;
    }

    static ssize_t module_xz_decompress!(load_info *info,
    const void *buf, size_t size)
    {
    static const u8 signature[] = { 0xfd, '7', 'z', 'X', 'Z', 0 };
pub static mut xz_dec: *mut c_void = core::ptr::null_mut();
pub static mut xz_buf: usize = 0;
    enum xz_ret xz_ret;
pub static mut new_size: usize = 0;
    let mut retval = 0;
    if (size < sizeof!(signature) ||
    memcmp(buf, signature, sizeof!(signature))) {
    pr_err!("not an xz compressed module\n");
    return -EINVAL;
    }
    xz_dec = xz_dec_init(XZ_DYNALLOC, (u32)-1);
    if (!xz_dec) {
    return -ENOMEM;
    }
    xz_buf.in_size = size;
    xz_buf.in = buf;
    xz_buf.in_pos = 0;
    do {
    let mut page = module_get_next_page!(info);
    if (IS_ERR(page)) {
    retval = PTR_ERR(page);
// goto;
    }
    xz_buf.out = kmap_local_page(page);
    xz_buf.out_pos = 0;
    xz_buf.out_size = PAGE_SIZE;
    xz_ret = xz_dec_run(xz_dec, &xz_buf);
    kunmap_local(xz_buf.out);
    new_size += xz_buf.out_pos;
    } while (xz_buf.out_pos == PAGE_SIZE && xz_ret == XZ_OK);
    if (xz_ret != XZ_STREAM_END) {
    pr_err!("decompression failed with status %d\n", xz_ret);
    retval = -EINVAL;
// goto;
    }
    retval = new_size;
// label;
    xz_dec_end(xz_dec);
    return retval;
    }

    static ssize_t module_zstd_decompress!(load_info *info,
    const void *buf, size_t size)
    {
    static const u8 signature[] = { 0x28, 0xb5, 0x2f, 0xfd };
    let mut zstd_dec;
    let mut zstd_buf;
    let mut header;
    let mut wksp_size = 0;
    let mut wksp = core::ptr::null_mut();
pub static mut dstream: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
pub static mut new_size: usize = 0;
    let mut retval = 0;
    if (size < sizeof!(signature) ||
    memcmp(buf, signature, sizeof!(signature))) {
    pr_err!("not a zstd compressed module\n");
    return -EINVAL;
    }
    zstd_buf.src = buf;
    zstd_buf.pos = 0;
    zstd_buf.size = size;
    ret = zstd_get_frame_header(&header, zstd_buf.src, zstd_buf.size);
    if (ret != 0) {
    pr_err!("ZSTD-compressed data has an incomplete frame header\n");
    retval = -EINVAL;
// goto;
    }
    if (header.windowSize > (1 << ZSTD_WINDOWLOG_MAX)) {
    pr_err!("ZSTD-compressed data has too large a window size\n");
    retval = -EINVAL;
// goto;
    }
    wksp_size = zstd_dstream_workspace_bound(header.windowSize);
    wksp = kvmalloc(wksp_size, GFP_KERNEL);
    if (!wksp) {
    retval = -ENOMEM;
// goto;
    }
    dstream = zstd_init_dstream(header.windowSize, wksp, wksp_size);
    if (!dstream) {
    pr_err!("Can't initialize ZSTD stream\n");
    retval = -ENOMEM;
// goto;
    }
    do {
    let mut page = module_get_next_page!(info);
    if (IS_ERR(page)) {
    retval = PTR_ERR(page);
// goto;
    }
    zstd_dec.dst = kmap_local_page(page);
    zstd_dec.pos = 0;
    zstd_dec.size = PAGE_SIZE;
    ret = zstd_decompress_stream(dstream, &zstd_dec, &zstd_buf);
    kunmap_local(zstd_dec.dst);
    retval = zstd_get_error_code(ret);
    if (retval) {
    break;
    }
    new_size += zstd_dec.pos;
    } while (zstd_dec.pos == PAGE_SIZE && ret != 0);
    if (retval) {
    pr_err!("ZSTD-decompression failed with status %d\n", retval);
    retval = -EINVAL;
// goto;
    }
    retval = new_size;
// label;
    kvfree(wksp);
    return retval;
    }

#[no_mangle]
pub unsafe extern "C" fn module_decompress!(info: *mut load_info, buf: *const c_void, size: usize) -> c_int {
    let mut n_pages = 0;
    let mut data_size = 0;
    let mut error = 0;

    info.compressed_len = size;

//
// Start with number of pages twice as big as needed for
// compressed data.
//
    n_pages = DIV_ROUND_UP(size, PAGE_SIZE) * 2;
    error = module_extend_max_pages!(info, n_pages);
    if (error) {
    return error;
    }
    data_size = MODULE_DECOMPRESS_FN(info, buf, size);
    if (data_size < 0) {
    error = data_size;
// goto;
    }
    info.hdr = vmap(info.pages, info.used_pages, VM_MAP, PAGE_KERNEL);
    if (!info.hdr) {
    error = -ENOMEM;
// goto;
    }
    info.len = data_size;
    return 0;
// label;
    module_decompress_cleanup!(info);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn module_decompress_cleanup!(info: *mut load_info) {
    let mut i = 0;
    if (info.hdr) {
    vunmap(info.hdr);
    }
    for (i = 0; i < info.used_pages; i++) {
    __free_page(info.pages[i]);
    }
    kvfree(info.pages);
    info.pages = core::ptr::null_mut();
    info.max_pages = info.used_pages = 0;
    }

#[no_mangle]
pub unsafe extern "C" fn compression_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, __stringify(MODULE_COMPRESSION) "\n");
    }
pub static mut module_compression_attr: kobj_attribute = 0;
#[no_mangle]
unsafe extern "C" fn module_decompress_sysfs_init!() -> c_int {
    let mut error = 0;
    error = sysfs_create_file(&module_kset.kobj,
    &module_compression_attr.attr);
    if (error) {
    pr_warn!("Failed to create 'compression' attribute");
    }
    return 0;
    }
    late_initcall!(module_decompress_sysfs_init);