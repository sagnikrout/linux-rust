//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/sof-client-ipc-kernel-injector.c
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
// Copyright(c) 2023 Google Inc
//
// Author: Curtis Malainey <cujomalainey@chromium.org>
//

pub const SOF_IPC_CLIENT_SUSPEND_DELAY_MS: c_int = 3000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_msg_inject_priv {
    pub kernel_dfs_file: *mut dentry,
    pub max_msg_size: usize,
    pub kernel_buffer: *mut c_void,
}

#[no_mangle]
unsafe extern "C" fn sof_msg_inject_dfs_open(inode: *mut inode, file: *mut file) -> c_int {
    static int sof_msg_inject_dfs_open(struct inode *inode, struct file *file)
    {
    let mut ret: c_int = debugfs_file_get(file.f_path.dentry);
    if (unlikely(ret))
    return ret;
    ret = simple_open(inode, file);
    if (ret)
    debugfs_file_put(file.f_path.dentry);
    return ret;
    }
    static ssize_t sof_kernel_msg_inject_dfs_write(struct file *file, const char __user *buffer,
    size_t count, loff_t *ppos)
    {
    struct sof_client_dev *cdev = file.private_data;
    struct sof_msg_inject_priv *priv = cdev.data;
    struct sof_ipc_cmd_hdr *hdr = priv.kernel_buffer;
    struct device *dev = &cdev.auxdev.dev;
    ssize_t size;
    int ret;
    if (*ppos)
    return 0;
    size = simple_write_to_buffer(priv.kernel_buffer, priv.max_msg_size,
    ppos, buffer, count);
    if (size < 0)
    return size;
    if (size != count)
    return -EFAULT;
    ret = pm_runtime_resume_and_get(dev);
    if (ret < 0 && ret != -EACCES) {
    dev_err_ratelimited(dev, "debugfs write failed to resume %d\n", ret);
    return ret;
    }
    ret = sof_client_boot_dsp(cdev);
    if (!ret)
    sof_client_ipc_rx_message(cdev, hdr, priv.kernel_buffer);
    ret = pm_runtime_put_autosuspend(dev);
    if (ret < 0)
    dev_err_ratelimited(dev, "debugfs write failed to idle %d\n", ret);
    return count;
    };
#[no_mangle]
unsafe extern "C" fn sof_msg_inject_dfs_release(inode: *mut inode, file: *mut file) -> c_int {
    static int sof_msg_inject_dfs_release(struct inode *inode, struct file *file)
    {
    debugfs_file_put(file.f_path.dentry);
    return 0;
    }
    static const struct file_operations sof_kernel_msg_inject_fops = {
    .open = sof_msg_inject_dfs_open,
    .write = sof_kernel_msg_inject_dfs_write,
    .release = sof_msg_inject_dfs_release,
    .owner = THIS_MODULE,
    };
    static int sof_msg_inject_probe(struct auxiliary_device *auxdev,
    const struct auxiliary_device_id *id)
    {
    struct sof_client_dev *cdev = auxiliary_dev_to_sof_client_dev(auxdev);
    struct dentry *debugfs_root = sof_client_get_debugfs_root(cdev);
    struct device *dev = &auxdev.dev;
    struct sof_msg_inject_priv *priv;
    size_t alloc_size;
// allocate memory for client data
    priv = devm_kzalloc(&auxdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.max_msg_size = sof_client_get_ipc_max_payload_size(cdev);
    alloc_size = priv.max_msg_size;
    priv.kernel_buffer = devm_kmalloc(dev, alloc_size, GFP_KERNEL);
    if (!priv.kernel_buffer)
    return -ENOMEM;
    cdev.data = priv;
    priv.kernel_dfs_file = debugfs_create_file("kernel_ipc_msg_inject", 0644,
    debugfs_root, cdev,
    &sof_kernel_msg_inject_fops);
// enable runtime PM
    pm_runtime_set_autosuspend_delay(dev, SOF_IPC_CLIENT_SUSPEND_DELAY_MS);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    pm_runtime_mark_last_busy(dev);
    pm_runtime_idle(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sof_msg_inject_remove(auxdev: *mut auxiliary_device) {
    static void sof_msg_inject_remove(struct auxiliary_device *auxdev)
    {
    struct sof_client_dev *cdev = auxiliary_dev_to_sof_client_dev(auxdev);
    struct sof_msg_inject_priv *priv = cdev.data;
    pm_runtime_disable(&auxdev.dev);
    debugfs_remove(priv.kernel_dfs_file);
    }
    static const struct auxiliary_device_id sof_msg_inject_client_id_table[] = {
    { .name = "snd_sof.kernel_injector" },
    {},
    };
    MODULE_DEVICE_TABLE(auxiliary, sof_msg_inject_client_id_table);
//
// No need for driver pm_ops as the generic pm callbacks in the auxiliary bus
// type are enough to ensure that the parent SOF device resumes to bring the DSP
// back to D0.
// Driver name will be set based on KBUILD_MODNAME.
//
    static struct auxiliary_driver sof_msg_inject_client_drv = {
    .probe = sof_msg_inject_probe,
    .remove = sof_msg_inject_remove,
    .id_table = sof_msg_inject_client_id_table,
    };
    module_auxiliary_driver(sof_msg_inject_client_drv);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("SOF IPC Kernel Injector Client Driver");
    MODULE_IMPORT_NS("SND_SOC_SOF_CLIENT");
