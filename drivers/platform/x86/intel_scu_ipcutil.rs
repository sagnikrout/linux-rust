//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel_scu_ipcutil.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Driver for the Intel SCU IPC mechanism
//
// (C) Copyright 2008-2010 Intel Corporation
// Author: Sreedhara DS (sreedhara.ds@intel.com)
//
// This driver provides IOCTL interfaces to call Intel SCU IPC driver API.
//

    static int major;
    static struct intel_scu_ipc_dev *scu;
    static DEFINE_MUTEX(scu_lock);
// IOCTL commands
pub const INTE_SCU_IPC_REGISTER_READ: c_int = 0;
pub const INTE_SCU_IPC_REGISTER_WRITE: c_int = 1;
pub const INTE_SCU_IPC_REGISTER_UPDATE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_ipc_data {
    pub /: *mut *mut u32 count; / No. of registers,
    pub /: *mut *mut u16 addr[5]; / Register addresses,
    pub /: *mut *mut u8 data[5]; / Register data,
    pub /: *mut *mut u8 mask; / Valid for read-modify-write,
}

//
// scu_reg_access		-	implement register access ioctls
// @cmd: command we are doing (read/write/update)
// @data: kernel copy of ioctl data
//
// Allow the user to perform register accesses on the SCU via the
// kernel interface
//
#[no_mangle]
unsafe extern "C" fn scu_reg_access(cmd: u32, data: *mut scu_ipc_data) -> c_int {
    static int scu_reg_access(u32 cmd, struct scu_ipc_data  *data)
    {
    let mut count: c_uint = data.count;
    if (count == 0 || count == 3 || count > 4)
    return -EINVAL;
    switch (cmd) {
    case INTE_SCU_IPC_REGISTER_READ:
    return intel_scu_ipc_dev_readv(scu, data.addr, data.data, count);
    case INTE_SCU_IPC_REGISTER_WRITE:
    return intel_scu_ipc_dev_writev(scu, data.addr, data.data, count);
    case INTE_SCU_IPC_REGISTER_UPDATE:
    return intel_scu_ipc_dev_update(scu, data.addr[0], data.data[0],
    data.mask);
    default:
    return -ENOTTY;
    }
    }
//
// scu_ipc_ioctl		-	control ioctls for the SCU
// @fp: file handle of the SCU device
// @cmd: ioctl coce
// @arg: pointer to user passed structure
//
// Support the I/O and firmware flashing interfaces of the SCU
//
    static long scu_ipc_ioctl(struct file *fp, unsigned int cmd,
    unsigned long arg)
    {
    int ret;
    struct scu_ipc_data  data;
    void __user *argp = (void __user *)arg;
    if (!capable(CAP_SYS_RAWIO))
    return -EPERM;
    if (copy_from_user(&data, argp, sizeof(struct scu_ipc_data)))
    return -EFAULT;
    ret = scu_reg_access(cmd, &data);
    if (ret < 0)
    return ret;
    if (copy_to_user(argp, &data, sizeof(struct scu_ipc_data)))
    return -EFAULT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn scu_ipc_open(inode: *mut inode, file: *mut file) -> c_int {
    static int scu_ipc_open(struct inode *inode, struct file *file)
    {
    let mut ret: c_int = 0;
// Only single open at the time
    mutex_lock(&scu_lock);
    if (scu) {
    ret = -EBUSY;
    goto unlock;
    }
    scu = intel_scu_ipc_dev_get();
    if (!scu)
    ret = -ENODEV;
    unlock:
    mutex_unlock(&scu_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn scu_ipc_release(inode: *mut inode, file: *mut file) -> c_int {
    static int scu_ipc_release(struct inode *inode, struct file *file)
    {
    mutex_lock(&scu_lock);
    intel_scu_ipc_dev_put(scu);
    scu = core::ptr::null_mut();
    mutex_unlock(&scu_lock);
    return 0;
    }
    static const struct file_operations scu_ipc_fops = {
    .unlocked_ioctl = scu_ipc_ioctl,
    .open = scu_ipc_open,
    .release = scu_ipc_release,
    };
#[no_mangle]
unsafe extern "C" fn ipc_module_init() -> int __init {
    static int __init ipc_module_init(void)
    {
    major = register_chrdev(0, "intel_mid_scu", &scu_ipc_fops);
    if (major < 0)
    return major;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipc_module_exit() -> void __exit {
    static void __exit ipc_module_exit(void)
    {
    unregister_chrdev(major, "intel_mid_scu");
    }
    module_init(ipc_module_init);
    module_exit(ipc_module_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Utility driver for intel scu ipc");
    MODULE_AUTHOR("Sreedhara <sreedhara.ds@intel.com>");
