//! Automatically rewritten from C to Rust
//! Source: kernel/power/user.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/kernel/power/user.c
//
// This file provides the user space interface for software suspend/resume.
//
// Copyright (C) 2006 Rafael J. Wysocki <rjw@sisk.pl>
//

    static bool need_wait;
    static struct snapshot_data {
    struct snapshot_handle handle;
    int swap;
    int mode;
    bool frozen;
    bool ready;
    bool platform_support;
    bool free_bitmaps;
    dev_t dev;
    } snapshot_state;
#[no_mangle]
pub unsafe extern "C" fn is_hibernate_resume_dev(dev: dev_t) -> c_int {
    int is_hibernate_resume_dev(dev_t dev)
    {
    return hibernation_available() && snapshot_state.dev == dev;
    }
#[no_mangle]
unsafe extern "C" fn snapshot_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int snapshot_open(struct inode *inode, struct file *filp)
    {
    struct snapshot_data *data;
    unsigned int sleep_flags;
    int error;
    if (!hibernation_available())
    return -EPERM;
    sleep_flags = lock_system_sleep();
    if (!hibernate_acquire()) {
    error = -EBUSY;
    goto Unlock;
    }
    if ((filp.f_flags & O_ACCMODE) == O_RDWR) {
    hibernate_release();
    error = -ENOSYS;
    goto Unlock;
    }
    nonseekable_open(inode, filp);
    data = &snapshot_state;
    filp.private_data = data;
    memset(&data.handle, 0, sizeof(struct snapshot_handle));
    if ((filp.f_flags & O_ACCMODE) == O_RDONLY) {
// Hibernating.  The image device should be accessible.
    data.swap = pin_hibernation_swap_type(swsusp_resume_device, 0);
    data.mode = O_RDONLY;
    data.free_bitmaps = false;
    error = pm_notifier_call_chain_robust(PM_HIBERNATION_PREPARE, PM_POST_HIBERNATION);
    } else {
//
// Resuming.  We may need to wait for the image device to
// appear.
//
    need_wait = true;
    data.swap = -1;
    data.mode = O_WRONLY;
    error = pm_notifier_call_chain_robust(PM_RESTORE_PREPARE, PM_POST_RESTORE);
    if (!error) {
    error = create_basic_memory_bitmaps();
    data.free_bitmaps = !error;
    }
    }
    if (error) {
    unpin_hibernation_swap_type(data.swap);
    hibernate_release();
    }
    data.frozen = false;
    data.ready = false;
    data.platform_support = false;
    data.dev = 0;
    Unlock:
    unlock_system_sleep(sleep_flags);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn snapshot_release(inode: *mut inode, filp: *mut file) -> c_int {
    static int snapshot_release(struct inode *inode, struct file *filp)
    {
    struct snapshot_data *data;
    unsigned int sleep_flags;
    sleep_flags = lock_system_sleep();
    swsusp_free();
    data = filp.private_data;
    data.dev = 0;
    free_all_swap_pages(data.swap);
    unpin_hibernation_swap_type(data.swap);
    if (data.frozen) {
    pm_restore_gfp_mask();
    free_basic_memory_bitmaps();
    thaw_processes();
    } else if (data.free_bitmaps) {
    free_basic_memory_bitmaps();
    }
    pm_notifier_call_chain(data.mode == O_RDONLY ?
    PM_POST_HIBERNATION : PM_POST_RESTORE);
    hibernate_release();
    unlock_system_sleep(sleep_flags);
    return 0;
    }
    static ssize_t snapshot_read(struct file *filp, char __user *buf,
    size_t count, loff_t *offp)
    {
    let mut pg_offp: loff_t = *offp & ~PAGE_MASK;
    struct snapshot_data *data;
    unsigned int sleep_flags;
    ssize_t res;
    sleep_flags = lock_system_sleep();
    data = filp.private_data;
    if (!data.ready) {
    res = -ENODATA;
    goto Unlock;
    }
    if (!pg_offp) { /* on page boundary? */
    res = snapshot_read_next(&data.handle);
    if (res <= 0)
    goto Unlock;
    } else {
    res = PAGE_SIZE - pg_offp;
    }
    res = simple_read_from_buffer(buf, count, &pg_offp,
    data_of(data.handle), res);
    if (res > 0)
// offp += res;
    Unlock:
    unlock_system_sleep(sleep_flags);
    return res;
    }
    static ssize_t snapshot_write(struct file *filp, const char __user *buf,
    size_t count, loff_t *offp)
    {
    let mut pg_offp: loff_t = *offp & ~PAGE_MASK;
    struct snapshot_data *data;
    unsigned long sleep_flags;
    ssize_t res;
    if (need_wait) {
    wait_for_device_probe();
    need_wait = false;
    }
    sleep_flags = lock_system_sleep();
    data = filp.private_data;
    if (!pg_offp) {
    res = snapshot_write_next(&data.handle);
    if (res <= 0)
    goto unlock;
    } else {
    res = PAGE_SIZE;
    }
    if (!data_of(data.handle)) {
    res = -EINVAL;
    goto unlock;
    }
    res = simple_write_to_buffer(data_of(data.handle), res, &pg_offp,
    buf, count);
    if (res > 0)
// offp += res;
    unlock:
    unlock_system_sleep(sleep_flags);
    return res;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_resume_swap_area {
    pub offset: compat_loff_t,
    pub dev: u32,
    pub __packed: },
    static int snapshot_set_swap_area(struct snapshot_data *data,
    void __user *argp)
    {
    pub offset: sector_t,
    pub swdev: dev_t,
    if (swsusp_swap_in_use())
    pub -EPERM: return,
    if (in_compat_syscall()) {
    pub swap_area: compat_resume_swap_area,
    if (copy_from_user(&swap_area, argp, sizeof(swap_area)))
    pub -EFAULT: return,
    pub new_decode_dev(swap_area.dev): swdev =,
    pub swap_area.offset: offset =,
    } else {
    pub swap_area: resume_swap_area,
    if (copy_from_user(&swap_area, argp, sizeof(swap_area)))
    pub -EFAULT: return,
    pub new_decode_dev(swap_area.dev): swdev =,
    pub swap_area.offset: offset =,
    }
//
// Unpin the swap device if a swap area was already
// set by SNAPSHOT_SET_SWAP_AREA.
//
// User space encodes device types as two-byte values,
// so we need to recode them
//
    pub offset): data->swap = pin_hibernation_swap_type(swdev,,
    if (data.swap < 0)
    pub -EINVAL: return swdev ? -ENODEV :,
    pub swdev: data->dev =,
    pub 0: return,
    }
    static long snapshot_ioctl(struct file *filp, unsigned int cmd,
    unsigned long arg)
    {
    pub 0: int error =,
    pub data: *mut snapshot_data,
    pub size: loff_t,
    pub offset: sector_t,
    if (need_wait) {
    pub false: need_wait =,
    }
    if (_IOC_TYPE(cmd) != SNAPSHOT_IOC_MAGIC)
    pub -ENOTTY: return,
    if (_IOC_NR(cmd) > SNAPSHOT_IOC_MAXNR)
    pub -ENOTTY: return,
    if (!capable(CAP_SYS_ADMIN))
    pub -EPERM: return,
    if (!mutex_trylock(&system_transition_mutex))
    pub -EBUSY: return,
    pub filp->private_data: data =,
    switch (cmd) {
    case SNAPSHOT_FREEZE:
    if (data.frozen)
    pub pm_sleep_fs_sync(): error =,
    if (error)
    pub freeze_processes(): error =,
    if (error)
    pub create_basic_memory_bitmaps(): error =,
    if (error)
    else
    pub true: data->frozen =,
    case SNAPSHOT_UNFREEZE:
    if (!data.frozen || data.ready)
    pub false: data->free_bitmaps =,
    pub false: data->frozen =,
    case SNAPSHOT_CREATE_IMAGE:
    if (data.mode != O_RDONLY || !data.frozen  || data.ready) {
    pub -EPERM: error =,
    }
    pub hibernation_snapshot(data->platform_support): error =,
    if (!error) {
    pub )arg): *mut error = put_user(in_suspend, (int __user,
    pub !error: data->ready = !freezer_test_done &&,
    pub false: freezer_test_done =,
    }
    case SNAPSHOT_ATOMIC_RESTORE:
    pub snapshot_write_finalize(&data->handle): error =,
    if (error)
    if (data.mode != O_WRONLY || !data.frozen) {
    pub -EPERM: error =,
    }
    if (!snapshot_image_loaded(&data.handle)) {
    pub -ENODATA: error =,
    }
    pub hibernation_restore(data->platform_support): error =,
    case SNAPSHOT_FREE:
    pub snapshot_handle)): memset(&data->handle, 0, sizeof(struct,
    pub false: data->ready =,
//
// It is necessary to thaw kernel threads here, because
// SNAPSHOT_CREATE_IMAGE may be invoked directly after
// SNAPSHOT_FREE.  In that case, if kernel threads were not
// thawed, the preallocation of memory carried out by
// hibernation_snapshot() might run into problems (i.e. it
// might fail or even deadlock).
//
    case SNAPSHOT_PREF_IMAGE_SIZE:
    pub arg: image_size =,
    case SNAPSHOT_GET_IMAGE_SIZE:
    if (!data.ready) {
    pub -ENODATA: error =,
    }
    pub snapshot_get_image_size(): size =,
    pub PAGE_SHIFT: size <<=,
    pub )arg): *mut error = put_user(size, (loff_t __user,
    case SNAPSHOT_AVAIL_SWAP_SIZE:
    pub 1): size = count_swap_pages(data->swap,,
    pub PAGE_SHIFT: size <<=,
    pub )arg): *mut error = put_user(size, (loff_t __user,
    case SNAPSHOT_ALLOC_SWAP_PAGE:
    if (data.swap < 0 || data.swap >= MAX_SWAPFILES) {
    pub -ENODEV: error =,
    }
    pub alloc_swapdev_block(data->swap): offset =,
    if (offset) {
    pub PAGE_SHIFT: offset <<=,
    pub )arg): *mut error = put_user(offset, (loff_t __user,
    } else {
    pub -ENOSPC: error =,
    }
    case SNAPSHOT_FREE_SWAP_PAGES:
    if (data.swap < 0 || data.swap >= MAX_SWAPFILES) {
    pub -ENODEV: error =,
    }
    case SNAPSHOT_S2RAM:
    if (!data.frozen) {
    pub -EPERM: error =,
    }
//
// Tasks are frozen and the notifiers have been called with
// PM_HIBERNATION_PREPARE
//
    pub suspend_devices_and_enter(PM_SUSPEND_MEM): error =,
    pub false: data->ready =,
    case SNAPSHOT_PLATFORM_SUPPORT:
    pub !!arg: data->platform_support =,
    case SNAPSHOT_POWER_OFF:
    if (data.platform_support)
    pub hibernation_platform_enter(): error =,
    case SNAPSHOT_SET_SWAP_AREA:
    pub )arg): *mut error = snapshot_set_swap_area(data, (void __user,
    default:
    pub -ENOTTY: error =,
    }
    pub error: return,
    }

    static long
    snapshot_compat_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
    {
    pub sizeof(compat_loff_t)): BUILD_BUG_ON(sizeof(loff_t) !=,
    switch (cmd) {
    case SNAPSHOT_GET_IMAGE_SIZE:
    case SNAPSHOT_AVAIL_SWAP_SIZE:
    case SNAPSHOT_ALLOC_SWAP_PAGE:
    case SNAPSHOT_CREATE_IMAGE:
    case SNAPSHOT_SET_SWAP_AREA:
    return snapshot_ioctl(file, cmd,
    pub compat_ptr(arg)): (unsigned long),
    default:
    pub arg): return snapshot_ioctl(file, cmd,,
    }
    }

    static const struct file_operations snapshot_fops = {
    .open = snapshot_open,
    .release = snapshot_release,
    .read = snapshot_read,
    .write = snapshot_write,
    .unlocked_ioctl = snapshot_ioctl,

    .compat_ioctl = snapshot_compat_ioctl,

}

    static struct miscdevice snapshot_device = {
    .minor = SNAPSHOT_MINOR,
    .name = "snapshot",
    .fops = &snapshot_fops,
    };
#[no_mangle]
unsafe extern "C" fn snapshot_device_init() -> int __init {
    static int __init snapshot_device_init(void)
    {
    return misc_register(&snapshot_device);
    };
    device_initcall(snapshot_device_init);
