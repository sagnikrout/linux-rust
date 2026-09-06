//! Automatically rewritten from C to Rust
//! Source: kernel/kexec.c
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
// kexec.c - kexec_load system call
// Copyright (C) 2002-2004 Eric Biederman  <ebiederm@xmission.com>
//

#[no_mangle]
pub unsafe extern "C" fn kimage_alloc_init() {
    let mut ret = 0;
    let mut image = core::ptr::null_mut();
pub static mut kexec_on_panic: bool = flags & KEXEC_ON_CRASH;

    if (kexec_on_panic) {
// Verify we have a valid entry point
    if ((entry < phys_to_boot_phys(crashk_res.start)) ||
    (entry > phys_to_boot_phys(crashk_res.end)))
    return -EADDRNOTAVAIL;
    }

// Allocate and initialize a controlling structure
    image = do_kimage_alloc_init();
    if (!image) {
    return -ENOMEM;
    }
    image.start = entry;
    image.nr_segments = nr_segments;
    memcpy(image.segment, segments, nr_segments * sizeof(*segments));

    if (kexec_on_panic) {
// Enable special crash kernel control page alloc policy.
    image.control_page = crashk_res.start;
    image.type = KEXEC_TYPE_CRASH;
    }

    ret = sanity_check_segment_list(image);
    if (ret) {
    goto out_free_image;
    }
//
// Find a location for the control code buffer, and add it
// the vector of segments so that it's pages will also be
// counted as destination pages.
//
    ret = -ENOMEM;
    image.control_code_page = kimage_alloc_control_pages(image,
    get_order(KEXEC_CONTROL_PAGE_SIZE));
    if (!image.control_code_page) {
    pr_err("Could not allocate control_code_buffer\n");
    goto out_free_image;
    }
    if (!kexec_on_panic) {
    image.swap_page = kimage_alloc_control_pages(image, 0);
    if (!image.swap_page) {
    pr_err("Could not allocate swap buffer\n");
    goto out_free_control_pages;
    }
    }
// rimage = image;
    return 0;
    out_free_control_pages:
    kimage_free_page_list(&image.control_pages);
    out_free_image:
    kfree(image);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn do_kexec_load() {
    struct kimage **dest_image, *image;
    let mut i = 0;
    let mut ret = 0;
//
// Because we write directly to the reserved memory region when loading
// crash kernels we need a serialization here to prevent multiple crash
// kernels from attempting to load simultaneously.
//
    if (!kexec_trylock()) {
    return -EBUSY;
    }

    if (flags & KEXEC_ON_CRASH) {
    dest_image = &kexec_crash_image;
    if (kexec_crash_image) {
    arch_kexec_unprotect_crashkres();
    }
    } else {

    dest_image = &kexec_image;
    }
    if (nr_segments == 0) {
// Uninstall image
    kimage_free(xchg(dest_image, core::ptr::null_mut()));
    ret = 0;
    goto out_unlock;
    }
    if (flags & KEXEC_ON_CRASH) {
//
// Loading another kernel to switch to if this one
// crashes.  Free any current crash dump kernel before
// we corrupt it.
//
    kimage_free(xchg(&kexec_crash_image, core::ptr::null_mut()));
    }
    ret = kimage_alloc_init(&image, entry, nr_segments, segments, flags);
    if (ret) {
    goto out_unlock;
    }
    if (flags & KEXEC_PRESERVE_CONTEXT) {
    image.preserve_context = 1;
    }

    if ((flags & KEXEC_ON_CRASH) && arch_crash_hotplug_support(image, flags)) {
    image.hotplug_support = 1;
    }

    ret = machine_kexec_prepare(image);
    if (ret) {
    goto out;
    }
//
// Some architecture(like S390) may touch the crash memory before
// machine_kexec_prepare(), we must copy vmcoreinfo data after it.
//
    ret = kimage_crash_copy_vmcoreinfo(image);
    if (ret) {
    goto out;
    }
    for (i = 0; i < nr_segments; i++) {
    ret = kimage_load_segment(image, i);
    if (ret) {
    goto out;
    }
    }
    kimage_terminate(image);
    ret = machine_kexec_post_load(image);
    if (ret) {
    goto out;
    }
// Install the new kernel and uninstall the old
    image = xchg(dest_image, image);
    out:

    if ((flags & KEXEC_ON_CRASH) && kexec_crash_image) {
    arch_kexec_protect_crashkres();
    }

    kimage_free(image);
    out_unlock:
    kexec_unlock();
    return ret;
    }
//
// Exec Kernel system call: for obvious reasons only root may call it.
//
// This call breaks up into three pieces.
// - A generic part which loads the new kernel from the current
// address space, and very carefully places the data in the
// allocated pages.
//
// - A generic part that interacts with the kernel and tells all of
// the devices to shut down.  Preventing on-going dmas, and placing
// the devices in a consistent state so a later kernel can
// reinitialize them.
//
// - A machine specific part that includes the syscall number
// and then copies the image to it's final destination.  And
// jumps into the image at entry.
//
// kexec does not sync, or unmount filesystems so if you need
// that to happen you need to do that yourself.
//
#[no_mangle]
pub unsafe extern "C" fn kexec_load_check() {
    int image_type = (flags & KEXEC_ON_CRASH) ?
    KEXEC_TYPE_CRASH : KEXEC_TYPE_DEFAULT;
    let mut result = 0;
// We only trust the superuser with rebooting the system.
    if (!kexec_load_permitted(image_type)) {
    return -EPERM;
    }
// Permit LSMs and IMA to fail the kexec
    result = security_kernel_load_data(LOADING_KEXEC_IMAGE, false);
    if (result < 0) {
    return result;
    }
//
// kexec can be used to circumvent module loading restrictions, so
// prevent loading in that case
//
    result = security_locked_down(LOCKDOWN_KEXEC);
    if (result) {
    return result;
    }
//
// Verify we have a legal set of flags
// This leaves us room for future extensions.
//
    if ((flags & KEXEC_FLAGS) != (flags & ~KEXEC_ARCH_MASK)) {
    return -EINVAL;
    }
// Put an artificial cap on the number
// of segments passed to kexec_load.
//
    if (nr_segments > KEXEC_SEGMENT_MAX) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_kexec_load() {
    let mut ksegments = core::ptr::null_mut();
    let mut result = 0;
    result = kexec_load_check(nr_segments, flags);
    if (result) {
    return result;
    }
// Verify we are on the appropriate architecture
    if (((flags & KEXEC_ARCH_MASK) != KEXEC_ARCH) &&
    ((flags & KEXEC_ARCH_MASK) != KEXEC_ARCH_DEFAULT))
    return -EINVAL;
    ksegments = memdup_array_user(segments, nr_segments, sizeof(ksegments[0]));
    if (IS_ERR(ksegments)) {
    return PTR_ERR(ksegments);
    }
    result = do_kexec_load(entry, nr_segments, ksegments, flags);
    kfree(ksegments);
    return result;
    }

#[no_mangle]
pub unsafe extern "C" fn sys_kexec_load() {
    let mut in;
    let mut ksegments = core::ptr::null_mut();
    unsigned long i, result;
    result = kexec_load_check(nr_segments, flags);
    if (result) {
    return result;
    }
// Don't allow clients that don't understand the native
// architecture to do anything.
//
    if ((flags & KEXEC_ARCH_MASK) == KEXEC_ARCH_DEFAULT) {
    return -EINVAL;
    }
    ksegments = kmalloc_objs(ksegments[0], nr_segments);
    if (!ksegments) {
    return -ENOMEM;
    }
    for (i = 0; i < nr_segments; i++) {
    result = copy_from_user(&in, &segments[i], sizeof(in));
    if (result) {
    goto fail;
    }
    ksegments[i].buf   = compat_ptr(in.buf);
    ksegments[i].bufsz = in.bufsz;
    ksegments[i].mem   = in.mem;
    ksegments[i].memsz = in.memsz;
    }
    result = do_kexec_load(entry, nr_segments, ksegments, flags);
    fail:
    kfree(ksegments);
    return result;
    }