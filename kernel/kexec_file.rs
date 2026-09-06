//! Automatically rewritten from C to Rust
//! Source: kernel/kexec_file.c
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
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

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
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0-only
//
// kexec: kexec_file_load system call
//
// Copyright (C) 2014 Red Hat Inc.
// Authors:
// Vivek Goyal <vgoyal@redhat.com>
//

pub static mut sig_enforce: bool = false;
#[no_mangle]
pub unsafe extern "C" fn set_kexec_sig_enforced() {
    sig_enforce = true;
    }

#[no_mangle]
unsafe extern "C" fn check_ima_segment_index(image: *mut kimage, i: c_int) -> bool {
    if (image.is_ima_segment_index_set && i == image.ima_segment_index) {
    return true;
    }
    else {
    return false;
    }
    }

#[no_mangle]
unsafe extern "C" fn check_ima_segment_index(image: *mut kimage, i: c_int) -> bool {
    return false;
    }

// forward_decl: kexec_calculate_store_digests;
// Maximum size in bytes for kernel/initrd files.

//
// Currently this is the only default function that is exported as some
// architectures need it to do additional handlings.
// In the future, other default functions may be exported too if required.
//
#[no_mangle]
pub unsafe extern "C" fn kexec_image_probe_default() {
    const struct kexec_file_ops * const *fops;
pub static mut ret: c_int = 0;
    while (*fops && (*fops).probe) {
    ret = (*fops).probe(buf, buf_len);
    if (!ret) {
    image.fops = *fops;
    return ret;
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn kexec_image_load_default() {
    if (!image.fops || !image.fops.load) {
    return ERR_PTR(-ENOEXEC);
    }
    return image.fops.load(image, image.kernel_buf,
    image.kernel_buf_len, image.initrd_buf,
    image.initrd_buf_len, image.cmdline_buf,
    image.cmdline_buf_len);
    }
#[no_mangle]
pub unsafe extern "C" fn kexec_image_post_load_cleanup_default(image: *mut kimage) -> c_int {
    if (!image.fops || !image.fops.cleanup) {
    return 0;
    }
    return image.fops.cleanup(image.image_loader_data);
    }
//
// Free up memory used by kernel, initrd, and command line. This is temporary
// memory allocation which is not needed any more after these buffers have
// been loaded into separate segments and have been copied elsewhere.
//
#[no_mangle]
pub unsafe extern "C" fn kimage_file_post_load_cleanup(image: *mut kimage) {
    let mut pi = &image.purgatory_info;
    vfree(image.kernel_buf);
    image.kernel_buf = core::ptr::null_mut();
    vfree(image.initrd_buf);
    image.initrd_buf = core::ptr::null_mut();
    kfree(image.cmdline_buf);
    image.cmdline_buf = core::ptr::null_mut();
    vfree(pi.purgatory_buf);
    pi.purgatory_buf = core::ptr::null_mut();
    vfree(pi.sechdrs);
    pi.sechdrs = core::ptr::null_mut();

    vfree(image.ima_buffer);
    image.ima_buffer = core::ptr::null_mut();

// See if architecture has anything to cleanup post load
    arch_kimage_file_post_load_cleanup(image);
//
// Above call should have called into bootloader to free up
// any data stored in kimage->image_loader_data. It should
// be ok now to free it up.
//
    kfree(image.image_loader_data);
    image.image_loader_data = core::ptr::null_mut();
    kexec_file_dbg_print = false;
    }

#[no_mangle]
pub unsafe extern "C" fn kexec_kernel_verify_pe_sig(kernel: *const c_char, kernel_len: c_ulong) -> c_int {
    let mut ret = 0;
    ret = verify_pefile_signature(kernel, kernel_len,
    VERIFY_USE_SECONDARY_KEYRING,
    VERIFYING_KEXEC_PE_SIGNATURE);
    if (ret == -ENOKEY && IS_ENABLED!(CONFIG_INTEGRITY_PLATFORM_KEYRING)) {
    ret = verify_pefile_signature(kernel, kernel_len,
    VERIFY_USE_PLATFORM_KEYRING,
    VERIFYING_KEXEC_PE_SIGNATURE);
    }
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn kexec_image_verify_sig() {
    if (!image.fops || !image.fops.verify_sig) {
    pr_debug!("kernel loader does not support signature verification.\n");
    return -EKEYREJECTED;
    }
    return image.fops.verify_sig(buf, buf_len);
    }
#[no_mangle]
pub unsafe extern "C" fn kimage_validate_signature() {
    let mut ret = 0;
    ret = kexec_image_verify_sig(image, image.kernel_buf,
    image.kernel_buf_len);
    if (ret) {
    if (sig_enforce) {
    pr_notice("Enforced kernel signature verification failed (%d).\n", ret);
    return ret;
    }
//
// If IMA is guaranteed to appraise a signature on the kexec
// image, permit it even if the kernel is otherwise locked
// down.
//
    if (!ima_appraise_signature(READING_KEXEC_IMAGE) &&
    security_locked_down(LOCKDOWN_KEXEC)) {
    return -EPERM;
    }
    pr_debug!("kernel signature verification failed (%d).\n", ret);
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn kexec_post_load(image: *mut kimage, flags: c_ulong) -> c_int {

    if (!(flags & KEXEC_FILE_ON_CRASH)) {
    ima_kexec_post_load(image);
    }

    return machine_kexec_post_load(image);
    }
//
// In file mode list of segments is prepared by kernel. Copy relevant
// data from user space, do error checking, prepare segment list
//
#[no_mangle]
pub unsafe extern "C" fn kimage_file_prepare_segments() {
    let mut ret = 0;
    let mut ldata = core::ptr::null_mut();
    ret = kernel_read_file_from_fd(kernel_fd, 0, &image.kernel_buf,
    KEXEC_FILE_SIZE_MAX, core::ptr::null_mut(),
    READING_KEXEC_IMAGE);
    if (ret < 0) {
    return ret;
    }
    image.kernel_buf_len = ret;
    kexec_dprintk("kernel: %p kernel_size: %#lx\n",
    image.kernel_buf, image.kernel_buf_len);
// Call arch image probe handlers
    ret = arch_kexec_kernel_image_probe(image, image.kernel_buf,
    image.kernel_buf_len);
    if (ret) {
// goto;
    }

    ret = kimage_validate_signature(image);
    if (ret) {
// goto;
    }

// It is possible that there no initramfs is being loaded
    if (!(flags & KEXEC_FILE_NO_INITRAMFS)) {
    ret = kernel_read_file_from_fd(initrd_fd, 0, &image.initrd_buf,
    KEXEC_FILE_SIZE_MAX, core::ptr::null_mut(),
    READING_KEXEC_INITRAMFS);
    if (ret < 0) {
// goto;
    }
    image.initrd_buf_len = ret;
    ret = 0;
    }
    image.no_cma = !!(flags & KEXEC_FILE_NO_CMA);
    image.force_dtb = flags & KEXEC_FILE_FORCE_DTB;
    if (cmdline_len) {
    image.cmdline_buf = memdup_user(cmdline_ptr, cmdline_len);
    if (IS_ERR(image.cmdline_buf)) {
    ret = PTR_ERR(image.cmdline_buf);
    image.cmdline_buf = core::ptr::null_mut();
// goto;
    }
    image.cmdline_buf_len = cmdline_len;
// command line should be a string with last byte null
    if (image.cmdline_buf[cmdline_len - 1] != '\0') {
    ret = -EINVAL;
// goto;
    }
    ima_kexec_cmdline(kernel_fd, image.cmdline_buf,
    image.cmdline_buf_len - 1);
    }
// IMA needs to pass the measurement list to the next kernel.
    ima_add_kexec_buffer(image);
// If KHO is active, add its images to the list
    ret = kho_fill_kimage(image);
    if (ret) {
// goto;
    }
// Call image load handler
    ldata = kexec_image_load_default(image);
    if (IS_ERR(ldata)) {
    ret = PTR_ERR(ldata);
// goto;
    }
    image.image_loader_data = ldata;
// label;
// In case of error, free up all allocated memory in this function
    if (ret) {
    kimage_file_post_load_cleanup(image);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn kimage_file_alloc_init() {
    let mut ret = 0;
    let mut image = core::ptr::null_mut();
pub static mut kexec_on_panic: bool = false;
    image = do_kimage_alloc_init();
    if (!image) {
    return -ENOMEM;
    }
    kexec_file_dbg_print = !!(flags & KEXEC_FILE_DEBUG);
    image.file_mode = 1;

    if (kexec_on_panic) {
// Enable special crash kernel control page alloc policy.
    image.control_page = crashk_res.start;
    image.type = KEXEC_TYPE_CRASH;
    }

    ret = kimage_file_prepare_segments(image, kernel_fd, initrd_fd,
    cmdline_ptr, cmdline_len, flags);
    if (ret) {
// goto;
    }
    ret = sanity_check_segment_list(image);
    if (ret) {
// goto;
    }
    ret = -ENOMEM;
    image.control_code_page = kimage_alloc_control_pages(image,
    get_order(KEXEC_CONTROL_PAGE_SIZE));
    if (!image.control_code_page) {
    pr_err!("Could not allocate control_code_buffer\n");
// goto;
    }
    if (!kexec_on_panic) {
    image.swap_page = kimage_alloc_control_pages(image, 0);
    if (!image.swap_page) {
    pr_err!("Could not allocate swap buffer\n");
// goto;
    }
    }
// rimage = image;
    return 0;
// label;
    kimage_free_page_list(&image.control_pages);
// label;
    kimage_file_post_load_cleanup(image);
// label;
    kfree(image);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_kexec_file_load() {
    let mut image_type = (flags & KEXEC_FILE_ON_CRASH) ?
    KEXEC_TYPE_CRASH : KEXEC_TYPE_DEFAULT;
    let mut dest_image = core::ptr::null_mut();
    let mut image = core::ptr::null_mut();
pub static mut ret: c_int = 0;
// We only trust the superuser with rebooting the system.
    if (!kexec_load_permitted(image_type)) {
    return -EPERM;
    }
// Make sure we have a legal set of flags
    if (flags != (flags & KEXEC_FILE_FLAGS)) {
    return -EINVAL;
    }
    image = core::ptr::null_mut();
    if (!kexec_trylock()) {
    return -EBUSY;
    }

    if (image_type == KEXEC_TYPE_CRASH) {
    dest_image = &kexec_crash_image;
    if (kexec_crash_image) {
    arch_kexec_unprotect_crashkres();
    }
    } else {

    dest_image = &kexec_image;
    }
    if (flags & KEXEC_FILE_UNLOAD) {
// goto;
    }
//
// In case of crash, new kernel gets loaded in reserved region. It is
// same memory where old crash kernel might be loaded. Free any
// current crash dump kernel before we corrupt it.
//
    if (flags & KEXEC_FILE_ON_CRASH) {
    kimage_free(xchg(&kexec_crash_image, core::ptr::null_mut()));
    }
    ret = kimage_file_alloc_init(&image, kernel_fd, initrd_fd, cmdline_ptr,
    cmdline_len, flags);
    if (ret) {
// goto;
    }

    if ((flags & KEXEC_FILE_ON_CRASH) && arch_crash_hotplug_support(image, flags)) {
    image.hotplug_support = 1;
    }

    ret = machine_kexec_prepare(image);
    if (ret) {
// goto;
    }
//
// Some architecture(like S390) may touch the crash memory before
// machine_kexec_prepare(), we must copy vmcoreinfo data after it.
//
    ret = kimage_crash_copy_vmcoreinfo(image);
    if (ret) {
// goto;
    }
    ret = kexec_calculate_store_digests(image);
    if (ret) {
// goto;
    }
    kexec_dprintk("nr_segments = %lu\n", image.nr_segments);
    while (i < image.nr_segments) {
    let mut ksegment = core::ptr::null_mut();
    ksegment = &image.segment[i];
    kexec_dprintk("segment[%d]: buf=0x%p bufsz=0x%zx mem=0x%lx memsz=0x%zx\n",
    i, ksegment.buf, ksegment.bufsz, ksegment.mem,
    ksegment.memsz);
    ret = kimage_load_segment(image, i);
    if (ret) {
// goto;
    }
    }
    kimage_terminate(image);
    ret = kexec_post_load(image, flags);
    if (ret) {
// goto;
    }
    kexec_dprintk("kexec_file_load: type:%u, start:0x%lx head:0x%lx flags:0x%lx\n",
    image.type, image.start, image.head, flags);
//
// Free up any temporary buffers allocated which are not needed
// after image has been loaded
//
    kimage_file_post_load_cleanup(image);
// label;
    image = xchg(dest_image, image);
// label;
    if ((flags & KEXEC_FILE_ON_CRASH) && kexec_crash_image) {
    arch_kexec_protect_crashkres();
    }

    kexec_unlock();
    kimage_free(image);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn locate_mem_hole_top_down() {
    let mut image = kbuf.image;
    unsigned long temp_start, temp_end;
    temp_end = min(end, kbuf.buf_max);
    temp_start = temp_end - kbuf.memsz + 1;
    kexec_random_range_start(temp_start, temp_end, kbuf, &temp_start);
    do {
// align down start
    temp_start = ALIGN_DOWN(temp_start, kbuf.buf_align);
    if (temp_start < start || temp_start < kbuf.buf_min) {
    return 0;
    }
    temp_end = temp_start + kbuf.memsz - 1;
//
// Make sure this does not conflict with any of existing
// segments
//
    if (kimage_is_destination_range(image, temp_start, temp_end)) {
    temp_start = temp_start - PAGE_SIZE;
    continue;
    }
// Make sure this does not conflict with exclude range
    if (arch_check_excluded_range(image, temp_start, temp_end)) {
    temp_start = temp_start - PAGE_SIZE;
    continue;
    }
// We found a suitable memory range
    break;
    } while (1);
// If we are here, we found a suitable memory range
    kbuf.mem = temp_start;
// Success, stop navigating through remaining System RAM ranges
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn locate_mem_hole_bottom_up() {
    let mut image = kbuf.image;
    unsigned long temp_start, temp_end;
    temp_start = max(start, kbuf.buf_min);
    kexec_random_range_start(temp_start, end, kbuf, &temp_start);
    do {
    temp_start = ALIGN(temp_start, kbuf.buf_align);
    temp_end = temp_start + kbuf.memsz - 1;
    if (temp_end > end || temp_end > kbuf.buf_max) {
    return 0;
    }
//
// Make sure this does not conflict with any of existing
// segments
//
    if (kimage_is_destination_range(image, temp_start, temp_end)) {
    temp_start = temp_start + PAGE_SIZE;
    continue;
    }
// Make sure this does not conflict with exclude range
    if (arch_check_excluded_range(image, temp_start, temp_end)) {
    temp_start = temp_start + PAGE_SIZE;
    continue;
    }
// We found a suitable memory range
    break;
    } while (1);
// If we are here, we found a suitable memory range
    kbuf.mem = temp_start;
// Success, stop navigating through remaining System RAM ranges
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn locate_mem_hole_callback(res: *mut resource, arg: *mut c_void) -> c_int {
    let mut kbuf = arg;
pub static mut start: u64 = 0;
pub static mut sz: c_ulong = 0;
// Returning 0 will take to next memory range
// Don't use memory that will be detected and handled by a driver.
    if (res.flags & IORESOURCE_SYSRAM_DRIVER_MANAGED) {
    return 0;
    }
    if (sz < kbuf.memsz) {
    return 0;
    }
    if (end < kbuf.buf_min || start > kbuf.buf_max) {
    return 0;
    }
//
// Allocate memory top down with-in ram range. Otherwise bottom up
// allocation.
//
    if (kbuf.top_down) {
    return locate_mem_hole_top_down(start, end, kbuf);
    }
    return locate_mem_hole_bottom_up(start, end, kbuf);
    }

#[no_mangle]
pub unsafe extern "C" fn kexec_walk_memblock() {
pub static mut ret: c_int = 0;
    let mut i = 0;
    phys_addr_t mstart, mend;
pub static mut res: resource = 0;

    if (kbuf.image.type == KEXEC_TYPE_CRASH) {
    return func(&crashk_res, kbuf);
    }

//
// Using MEMBLOCK_NONE will properly skip MEMBLOCK_DRIVER_MANAGED. See
// IORESOURCE_SYSRAM_DRIVER_MANAGED handling in
// locate_mem_hole_callback().
//
    if (kbuf.top_down) {
    for_each_free_mem_range_reverse(i, NUMA_NO_NODE, MEMBLOCK_NONE,
    &mstart, &mend, core::ptr::null_mut()) {
//
// In memblock, end points to the first byte after the
// range while in kexec, end points to the last byte
// in the range.
//
    res.start = mstart;
    res.end = mend - 1;
    ret = func(&res, kbuf);
    if (ret) {
    break;
    }
    }
    } else {
    for_each_free_mem_range(i, NUMA_NO_NODE, MEMBLOCK_NONE,
    &mstart, &mend, core::ptr::null_mut()) {
//
// In memblock, end points to the first byte after the
// range while in kexec, end points to the last byte
// in the range.
//
    res.start = mstart;
    res.end = mend - 1;
    ret = func(&res, kbuf);
    if (ret) {
    break;
    }
    }
    }
    return ret;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: kexec_walk_memblock
pub unsafe extern "C" fn kexec_walk_memblock_dup() {
    return 0;
    }

//
// kexec_walk_resources - call func(data) on free memory regions
// @kbuf:	Context info for the search. Also passed to @func.
// @func:	Function to call for each memory region.
//
// Return: The memory walk will stop when func returns a non-zero value
// and that value will be returned. If all free regions are visited without
// func returning non-zero, then zero will be returned.
//
#[no_mangle]
pub unsafe extern "C" fn kexec_walk_resources() {

    if (kbuf.image.type == KEXEC_TYPE_CRASH) {
    return walk_iomem_res_desc(crashk_res.desc,
    IORESOURCE_SYSTEM_RAM | IORESOURCE_BUSY,
    crashk_res.start, crashk_res.end,
    kbuf, func);
    }

    if (kbuf.top_down) {
    return walk_system_ram_res_rev(0, ULONG_MAX, kbuf, func);
    }
    else {
    return walk_system_ram_res(0, ULONG_MAX, kbuf, func);
    }
    }
#[no_mangle]
unsafe extern "C" fn kexec_alloc_contig(kbuf: *mut kexec_buf) -> c_int {
pub static mut nr_pages: usize = 0;
    let mut mem = 0;
    let mut p = core::ptr::null_mut();
// User space disabled CMA allocations, bail out.
    if (kbuf.image.no_cma) {
    return -EPERM;
    }
// Skip CMA logic for crash kernel
    if (kbuf.image.type == KEXEC_TYPE_CRASH) {
    return -EPERM;
    }
    p = dma_alloc_from_contiguous(core::ptr::null_mut(), nr_pages, get_order(kbuf.buf_align), true);
    if (!p) {
    return -ENOMEM;
    }
    pr_debug!("allocated %zu DMA pages at 0x%lx", nr_pages, page_to_boot_pfn(p));
    mem = page_to_boot_pfn(p) << PAGE_SHIFT;
    if (kimage_is_destination_range(kbuf.image, mem, mem + kbuf.memsz)) {
// Our region is already in use by a statically defined one. Bail out.
    pr_debug!("CMA overlaps existing mem: 0x%lx+0x%lx\n", mem, kbuf.memsz);
    dma_release_from_contiguous(core::ptr::null_mut(), p, nr_pages);
    return -EBUSY;
    }
    kbuf.mem = page_to_boot_pfn(p) << PAGE_SHIFT;
    kbuf.cma = p;
    arch_kexec_post_alloc_pages(page_address(p), (int)nr_pages, 0);
    return 0;
    }
//
// kexec_locate_mem_hole - find free memory for the purgatory or the next kernel
// @kbuf:	Parameters for the memory search.
//
// On success, kbuf->mem will have the start address of the memory region found.
//
// Return: 0 on success, negative errno on error.
//
#[no_mangle]
pub unsafe extern "C" fn kexec_locate_mem_hole(kbuf: *mut kexec_buf) -> c_int {
    let mut ret = 0;
// Arch knows where to place
    if (kbuf.mem != KEXEC_BUF_MEM_UNKNOWN) {
    return 0;
    }
//
// If KHO is active, only use KHO scratch memory. All other memory
// could potentially be handed over.
//
    ret = kho_locate_mem_hole(kbuf, locate_mem_hole_callback);
    if (ret <= 0) {
    return ret;
    }
//
// Try to find a free physically contiguous block of memory first. With that, we
// can avoid any copying at kexec time.
//
    if (!kexec_alloc_contig(kbuf)) {
    return 0;
    }
    if (!IS_ENABLED!(CONFIG_ARCH_KEEP_MEMBLOCK)) {
    ret = kexec_walk_resources(kbuf, locate_mem_hole_callback);
    }
    else {
    ret = kexec_walk_memblock(kbuf, locate_mem_hole_callback);
    }
pub static mut ret: return = 0;
    }
//
// kexec_add_buffer - place a buffer in a kexec segment
// @kbuf:	Buffer contents and memory parameters.
//
// This function assumes that kexec_lock is held.
// On successful return, @kbuf->mem will have the physical address of
// the buffer in memory.
//
// Return: 0 on success, negative errno on error.
//
#[no_mangle]
pub unsafe extern "C" fn kexec_add_buffer(kbuf: *mut kexec_buf) -> c_int {
    let mut ksegment = core::ptr::null_mut();
    let mut ret = 0;
// Currently adding segment this way is allowed only in file mode
    if (!kbuf.image.file_mode) {
    return -EINVAL;
    }
    if (kbuf.image.nr_segments >= KEXEC_SEGMENT_MAX) {
    return -EINVAL;
    }
//
// Make sure we are not trying to add buffer after allocating
// control pages. All segments need to be placed first before
// any control pages are allocated. As control page allocation
// logic goes through list of segments to make sure there are
// no destination overlaps.
//
    if (!list_empty(&kbuf.image.control_pages)) {
// WARN_ON;
    return -EINVAL;
    }
// Ensure minimum alignment needed for segments.
    kbuf.memsz = ALIGN(kbuf.memsz, PAGE_SIZE);
    kbuf.buf_align = max(kbuf.buf_align, PAGE_SIZE);
    kbuf.cma = core::ptr::null_mut();
// Walk the RAM ranges and allocate a suitable range for the buffer
    ret = arch_kexec_locate_mem_hole(kbuf);
    if (ret) {
    return ret;
    }
// Found a suitable memory range
    ksegment = &kbuf.image.segment[kbuf.image.nr_segments];
    ksegment.kbuf = kbuf.buffer;
    ksegment.bufsz = kbuf.bufsz;
    ksegment.mem = kbuf.mem;
    ksegment.memsz = kbuf.memsz;
    kbuf.image.segment_cma[kbuf.image.nr_segments] = kbuf.cma;
    kbuf.image.nr_segments += 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kexec_only_cma_segments(image: *mut kimage) -> bool {
    while (i < image.nr_segments) {
    if (!image.segment_cma[i]) {
    return false;
    }
    }
    return true;
    }
// Calculate and store the digest of segments
#[no_mangle]
unsafe extern "C" fn kexec_calculate_store_digests(image: *mut kimage) -> c_int {
    let mut sctx;
pub static mut ret: c_int = 0;
    let mut nullsz = 0;
    u8 digest[SHA256_DIGEST_SIZE];
    let mut zero_buf = core::ptr::null_mut();
    let mut sha_regions = core::ptr::null_mut();
    let mut pi = &image.purgatory_info;
    if (!IS_ENABLED!(CONFIG_ARCH_SUPPORTS_KEXEC_PURGATORY)) {
    return 0;
    }
    zero_buf = __va(page_to_pfn(ZERO_PAGE(0)) << PAGE_SHIFT);
    zero_buf_sz = PAGE_SIZE;
    sha_region_sz = KEXEC_SEGMENT_MAX * sizeof!(kexec_sha_region);
    sha_regions = vzalloc(sha_region_sz);
    if (!sha_regions) {
    return -ENOMEM;
    }
    sha256_init(&sctx);
//
// If KHO is enabled, the destinations are located in KHO scratch.
// KHO scratch can only contain early boot allocations and movable
// allocations. That means there is no risk of memory corruption by
// uncancelled DMA.
//
// If all segments were loaded into contiguous memory, there will be no
// relocations at all, so also no risk of corruption.
//
    if (image.type != KEXEC_TYPE_CRASH &&
    (kho_is_enabled() || kexec_only_cma_segments(image))) {
    pr_debug!("disabling checksum verification in purgatory\n");
// goto;
    }
    while (i < image.nr_segments) {
    let mut ksegment = core::ptr::null_mut();

// Exclude elfcorehdr segment to allow future changes via hotplug
    if (i == image.elfcorehdr_index) {
    continue;
    }

    ksegment = &image.segment[i];
//
// Skip purgatory as it will be modified once we put digest
// info in purgatory.
//
    if (ksegment.kbuf == pi.purgatory_buf) {
    continue;
    }
//
// Skip the segment if ima_segment_index is set and matches
// the current index
//
    if (check_ima_segment_index(image, i)) {
    continue;
    }
    sha256_update(&sctx, ksegment.kbuf, ksegment.bufsz);
//
// Assume rest of the buffer is filled with zero and
// update digest accordingly.
//
    nullsz = ksegment.memsz - ksegment.bufsz;
    while (nullsz) {
pub static mut bytes: c_ulong = 0;
    if (bytes > zero_buf_sz) {
    bytes = zero_buf_sz;
    }
    sha256_update(&sctx, zero_buf, bytes);
    nullsz -= bytes;
    }
    sha_regions[j].start = ksegment.mem;
    sha_regions[j].len = ksegment.memsz;
    j += 1;
    }
// label;
    sha256_final(&sctx, digest);
    ret = kexec_purgatory_get_set_symbol(image, "purgatory_sha_regions",
    sha_regions, sha_region_sz, 0);
    if (ret) {
// goto;
    }
    ret = kexec_purgatory_get_set_symbol(image, "purgatory_sha256_digest",
    digest, SHA256_DIGEST_SIZE, 0);
// label;
    vfree(sha_regions);
    return ret;
    }

//
// kexec_purgatory_find_symbol - find a symbol in the purgatory
// @pi:		Purgatory to search in.
// @name:	Name of the symbol.
//
// Return: pointer to symbol in read-only symtab on success, NULL on error.
//
    static const Elf_Sym *kexec_purgatory_find_symbol(purgatory_info *pi,
    const char *name)
    {
    let mut sechdrs = core::ptr::null_mut();
    let mut ehdr = core::ptr::null_mut();
    let mut syms = core::ptr::null_mut();
    let mut strtab = core::ptr::null_mut();
    let mut i = 0;
    let mut k = 0;
    if (!pi.ehdr) {
    return core::ptr::null_mut();
    }
    ehdr = pi.ehdr;
    sechdrs = ehdr + ehdr.e_shoff;
    while (i < ehdr.e_shnum) {
    if (sechdrs[i].sh_type != SHT_SYMTAB) {
    continue;
    }
    if (sechdrs[i].sh_link >= ehdr.e_shnum) {
// Invalid strtab section number
    continue;
    }
    strtab = ehdr + sechdrs[sechdrs[i].sh_link].sh_offset;
    syms = ehdr + sechdrs[i].sh_offset;
// Go through symbols for a match
    while (k < sechdrs[i].sh_size/sizeof!(Elf_Sym)) {
    if (ELF_ST_BIND(syms[k].st_info) != STB_GLOBAL) {
    continue;
    }
    if (strcmp(strtab + syms[k].st_name, name) != 0) {
    continue;
    }
    if (syms[k].st_shndx == SHN_UNDEF ||
    syms[k].st_shndx >= ehdr.e_shnum) {
    pr_debug!("Symbol: %s has bad section index %d.\n",
    name, syms[k].st_shndx);
    return core::ptr::null_mut();
    }
// Found the symbol we are looking for
    return &syms[k];
    }
    }
    return core::ptr::null_mut();
    }
//
// kexec_purgatory_setup_kbuf - prepare buffer to load purgatory.
// @pi:		Purgatory to be loaded.
// @kbuf:	Buffer to setup.
//
// Allocates the memory needed for the buffer. Caller is responsible to free
// the memory after use.
//
// Return: 0 on success, negative errno on error.
//
#[no_mangle]
pub unsafe extern "C" fn kexec_purgatory_setup_kbuf() {
    let mut sechdrs = core::ptr::null_mut();
    let mut bss_align = 0;
    let mut bss_sz = 0;
    let mut align = 0;
    let mut i = 0;
    let mut ret = 0;
    sechdrs = pi.ehdr + pi.ehdr.e_shoff;
    kbuf.buf_align = bss_align = 1;
    kbuf.bufsz = bss_sz = 0;
    while (i < pi.ehdr.e_shnum) {
    if (!(sechdrs[i].sh_flags & SHF_ALLOC)) {
    continue;
    }
    align = sechdrs[i].sh_addralign;
    if (sechdrs[i].sh_type != SHT_NOBITS) {
    if (kbuf.buf_align < align) {
    kbuf.buf_align = align;
    }
    kbuf.bufsz = ALIGN(kbuf.bufsz, align);
    kbuf.bufsz += sechdrs[i].sh_size;
    } else {
    if (bss_align < align) {
    bss_align = align;
    }
    bss_sz = ALIGN(bss_sz, align);
    bss_sz += sechdrs[i].sh_size;
    }
    }
    kbuf.bufsz = ALIGN(kbuf.bufsz, bss_align);
    kbuf.memsz = kbuf.bufsz + bss_sz;
    if (kbuf.buf_align < bss_align) {
    kbuf.buf_align = bss_align;
    }
    kbuf.buffer = vzalloc(kbuf.bufsz);
    if (!kbuf.buffer) {
    return -ENOMEM;
    }
    pi.purgatory_buf = kbuf.buffer;
    ret = kexec_add_buffer(kbuf);
    if (ret) {
// goto;
    }
    return 0;
// label;
    vfree(pi.purgatory_buf);
    pi.purgatory_buf = core::ptr::null_mut();
    return ret;
    }
//
// kexec_purgatory_setup_sechdrs - prepares the pi->sechdrs buffer.
// @pi:		Purgatory to be loaded.
// @kbuf:	Buffer prepared to store purgatory.
//
// Allocates the memory needed for the buffer. Caller is responsible to free
// the memory after use.
//
// Return: 0 on success, negative errno on error.
//
#[no_mangle]
pub unsafe extern "C" fn kexec_purgatory_setup_sechdrs() {
    let mut bss_addr = 0;
    let mut offset = 0;
    let mut sechdrs_size = 0;
    let mut sechdrs = core::ptr::null_mut();
    let mut entry_sym = core::ptr::null_mut();
pub static mut entry_shndx: u16 = 0;
pub static mut entry_off: c_ulong = 0;
pub static mut start_fixed: bool = false;
    let mut i = 0;
//
// The section headers in kexec_purgatory are read-only. In order to
// have them modifiable make a temporary copy.
//
    sechdrs_size = array_size(sizeof!(Elf_Shdr), pi.ehdr.e_shnum);
    sechdrs = vzalloc(sechdrs_size);
    if (!sechdrs) {
    return -ENOMEM;
    }
    memcpy(sechdrs, pi.ehdr + pi.ehdr.e_shoff, sechdrs_size);
    pi.sechdrs = sechdrs;
    offset = 0;
    bss_addr = kbuf.mem + kbuf.bufsz;
    kbuf.image.start = pi.ehdr.e_entry;
    entry_sym = kexec_purgatory_find_symbol(pi, "purgatory_start");
    if (entry_sym) {
    entry_shndx = entry_sym.st_shndx;
    entry_off = entry_sym.st_value;
    }
    while (i < pi.ehdr.e_shnum) {
    let mut align = 0;
    let mut src = core::ptr::null_mut();
    let mut dst = core::ptr::null_mut();
    if (!(sechdrs[i].sh_flags & SHF_ALLOC)) {
    continue;
    }
    align = sechdrs[i].sh_addralign;
    if (sechdrs[i].sh_type == SHT_NOBITS) {
    bss_addr = ALIGN(bss_addr, align);
    sechdrs[i].sh_addr = bss_addr;
    bss_addr += sechdrs[i].sh_size;
    continue;
    }
    offset = ALIGN(offset, align);
    if (!start_fixed && entry_sym && i == entry_shndx &&
    (sechdrs[i].sh_flags & SHF_EXECINSTR) &&
    entry_off < sechdrs[i].sh_size) {
    kbuf.image.start = kbuf.mem + offset + entry_off;
    start_fixed = true;
    }
//
// Check if the segment contains the entry point, if so,
// calculate the value of image->start based on it.
// If the compiler has produced more than one .text section
// (Eg: .text.hot), they are generally after the main .text
// section, and they shall not be used to calculate
// image->start. So do not re-calculate image->start if it
// is not set to the initial value, and warn the user so they
// have a chance to fix their purgatory's linker script.
//
    if (!start_fixed && sechdrs[i].sh_flags & SHF_EXECINSTR &&
    pi.ehdr.e_entry >= sechdrs[i].sh_addr &&
    pi.ehdr.e_entry < (sechdrs[i].sh_addr
    + sechdrs[i].sh_size) &&
    kbuf.image.start == pi.ehdr.e_entry) {
    kbuf.image.start -= sechdrs[i].sh_addr;
    kbuf.image.start += kbuf.mem + offset;
    start_fixed = true;
    }
    src = pi.ehdr + sechdrs[i].sh_offset;
    dst = pi.purgatory_buf + offset;
    memcpy(dst, src, sechdrs[i].sh_size);
    sechdrs[i].sh_addr = kbuf.mem + offset;
    sechdrs[i].sh_offset = offset;
    offset += sechdrs[i].sh_size;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kexec_apply_relocations(image: *mut kimage) -> c_int {
    let mut i = 0;
    let mut ret = 0;
    let mut pi = &image.purgatory_info;
    let mut sechdrs = core::ptr::null_mut();
    sechdrs = pi.ehdr + pi.ehdr.e_shoff;
    while (i < pi.ehdr.e_shnum) {
    let mut relsec = core::ptr::null_mut();
    let mut symtab = core::ptr::null_mut();
    let mut section = core::ptr::null_mut();
    relsec = sechdrs + i;
    if (relsec.sh_type != SHT_RELA &&
    relsec.sh_type != SHT_REL) {
    continue;
    }
//
// For section of type SHT_RELA/SHT_REL,
// ->sh_link contains section header index of associated
// symbol table. And ->sh_info contains section header
// index of section to which relocations apply.
//
    if (relsec.sh_info >= pi.ehdr.e_shnum ||
    relsec.sh_link >= pi.ehdr.e_shnum) {
    return -ENOEXEC;
    }
    section = pi.sechdrs + relsec.sh_info;
    symtab = sechdrs + relsec.sh_link;
    if (!(section.sh_flags & SHF_ALLOC)) {
    continue;
    }
//
// symtab->sh_link contain section header index of associated
// string table.
//
    if (symtab.sh_link >= pi.ehdr.e_shnum) {
// Invalid section number?
    continue;
    }
//
// Respective architecture needs to provide support for applying
// relocations of type SHT_RELA/SHT_REL.
//
    if (relsec.sh_type == SHT_RELA) {
    ret = arch_kexec_apply_relocations_add(pi, section,
    relsec, symtab);
    }

    else if (relsec.sh_type == SHT_REL) {
    ret = arch_kexec_apply_relocations(pi, section,
    relsec, symtab);
    }
    if (ret) {
    return ret;
    }
    }
    return 0;
    }
//
// kexec_load_purgatory - Load and relocate the purgatory object.
// @image:	Image to add the purgatory to.
// @kbuf:	Memory parameters to use.
//
// Allocates the memory needed for image->purgatory_info.sechdrs and
// image->purgatory_info.purgatory_buf/kbuf->buffer. Caller is responsible
// to free the memory after use.
//
// Return: 0 on success, negative errno on error.
//
#[no_mangle]
pub unsafe extern "C" fn kexec_load_purgatory(image: *mut kimage, kbuf: *mut kexec_buf) -> c_int {
    let mut pi = &image.purgatory_info;
    let mut ret = 0;
    if (kexec_purgatory_size <= 0) {
    return -EINVAL;
    }
    pi.ehdr = kexec_purgatory;
    ret = kexec_purgatory_setup_kbuf(pi, kbuf);
    if (ret) {
    return ret;
    }
    ret = kexec_purgatory_setup_sechdrs(pi, kbuf);
    if (ret) {
// goto;
    }
    ret = kexec_apply_relocations(image);
    if (ret) {
// goto;
    }
    return 0;
// label;
    vfree(pi.sechdrs);
    pi.sechdrs = core::ptr::null_mut();
// label;
    vfree(pi.purgatory_buf);
    pi.purgatory_buf = core::ptr::null_mut();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn kexec_purgatory_get_symbol_addr() {
    let mut pi = &image.purgatory_info;
    let mut sym = core::ptr::null_mut();
    let mut sechdr = core::ptr::null_mut();
    sym = kexec_purgatory_find_symbol(pi, name);
    if (!sym) {
    return ERR_PTR(-EINVAL);
    }
    sechdr = &pi.sechdrs[sym.st_shndx];
//
// Returns the address where symbol will finally be loaded after
// kexec_load_segment()
//
    return (sechdr.sh_addr + sym.st_value);
    }
//
// Get or set value of a symbol. If "get_value" is true, symbol value is
// returned in buf otherwise symbol value is set based on value in buf.
//
#[no_mangle]
pub unsafe extern "C" fn kexec_purgatory_get_set_symbol() {
    let mut pi = &image.purgatory_info;
    let mut sym = core::ptr::null_mut();
    let mut sec = core::ptr::null_mut();
    let mut sym_buf = core::ptr::null_mut();
    sym = kexec_purgatory_find_symbol(pi, name);
    if (!sym) {
    return -EINVAL;
    }
    if (sym.st_size != size) {
    pr_err!("symbol %s size mismatch: expected %lu actual %u\n",
    name, (unsigned long)sym.st_size, size);
    return -EINVAL;
    }
    sec = pi.sechdrs + sym.st_shndx;
    if (sec.sh_type == SHT_NOBITS) {
    pr_err!("symbol %s is in a bss section. Cannot %s\n", name,
    get_value ? "get" : "set");
    return -EINVAL;
    }
    sym_buf = pi.purgatory_buf + sec.sh_offset + sym.st_value;
    if (get_value) {
    memcpy(buf, sym_buf, size);
    }
    else {
    memcpy(sym_buf, buf, size);
    }
    return 0;
    }