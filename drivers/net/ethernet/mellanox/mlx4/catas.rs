//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx4/catas.c
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


//
// Copyright (c) 2007 Cisco Systems, Inc. All rights reserved.
// Copyright (c) 2007, 2008 Mellanox Technologies. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

    enum {
    MLX4_CATAS_POLL_INTERVAL	= 5 * HZ,
    };
    let mut mlx4_internal_err_reset: c_int = 1;
    module_param_named(internal_err_reset, mlx4_internal_err_reset,  int, 0644);
    MODULE_PARM_DESC(internal_err_reset,
    "Reset device on internal errors if non-zero (default 1)");
#[no_mangle]
unsafe extern "C" fn read_vendor_id(dev: *mut mlx4_dev) -> c_int {
    static int read_vendor_id(struct mlx4_dev *dev)
    {
    let mut vendor_id: u16 = 0;
    int ret;
    ret = pci_read_config_word(dev.persist.pdev, 0, &vendor_id);
    if (ret) {
    mlx4_err(dev, "Failed to read vendor ID, ret=%d\n", ret);
    return ret;
    }
    if (vendor_id == 0xffff) {
    mlx4_err(dev, "PCI can't be accessed to read vendor id\n");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mlx4_reset_master(dev: *mut mlx4_dev) -> c_int {
    static int mlx4_reset_master(struct mlx4_dev *dev)
    {
    let mut err: c_int = 0;
    if (mlx4_is_master(dev))
    mlx4_report_internal_err_comm_event(dev);
    if (!pci_channel_offline(dev.persist.pdev)) {
    err = read_vendor_id(dev);
// If PCI can't be accessed to read vendor ID we assume that its
// link was disabled and chip was already reset.
//
    if (err)
    return 0;
    err = mlx4_reset(dev);
    if (err)
    mlx4_err(dev, "Fail to reset HCA\n");
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mlx4_reset_slave(dev: *mut mlx4_dev) -> c_int {
    static int mlx4_reset_slave(struct mlx4_dev *dev)
    {
pub const COM_CHAN_RST_REQ_OFFSET: c_uint = 0x10;
pub const COM_CHAN_RST_ACK_OFFSET: c_uint = 0x08;
    u32 comm_flags;
    u32 rst_req;
    u32 rst_ack;
    unsigned long end;
    struct mlx4_priv *priv = mlx4_priv(dev);
    if (pci_channel_offline(dev.persist.pdev))
    return 0;
    comm_flags = swab32(readl((__iomem char *)priv.mfunc.comm +
    MLX4_COMM_CHAN_FLAGS));
    if (comm_flags == 0xffffffff) {
    mlx4_err(dev, "VF reset is not needed\n");
    return 0;
    }
    if (!(dev.caps.vf_caps & MLX4_VF_CAP_FLAG_RESET)) {
    mlx4_err(dev, "VF reset is not supported\n");
    return -EOPNOTSUPP;
    }
    rst_req = (comm_flags & (u32)(1 << COM_CHAN_RST_REQ_OFFSET)) >>
    COM_CHAN_RST_REQ_OFFSET;
    rst_ack = (comm_flags & (u32)(1 << COM_CHAN_RST_ACK_OFFSET)) >>
    COM_CHAN_RST_ACK_OFFSET;
    if (rst_req != rst_ack) {
    mlx4_err(dev, "Communication channel isn't sync, fail to send reset\n");
    return -EIO;
    }
    rst_req ^= 1;
    mlx4_warn(dev, "VF is sending reset request to Firmware\n");
    comm_flags = rst_req << COM_CHAN_RST_REQ_OFFSET;
    __raw_writel(( u32)cpu_to_be32(comm_flags),
    (__iomem char *)priv.mfunc.comm + MLX4_COMM_CHAN_FLAGS);
    end = msecs_to_jiffies(MLX4_COMM_TIME) + jiffies;
    while (time_before(jiffies, end)) {
    comm_flags = swab32(readl((__iomem char *)priv.mfunc.comm +
    MLX4_COMM_CHAN_FLAGS));
    rst_ack = (comm_flags & (u32)(1 << COM_CHAN_RST_ACK_OFFSET)) >>
    COM_CHAN_RST_ACK_OFFSET;
// Reading rst_req again since the communication channel can
// be reset at any time by the PF and all its bits will be
// set to zero.
//
    rst_req = (comm_flags & (u32)(1 << COM_CHAN_RST_REQ_OFFSET)) >>
    COM_CHAN_RST_REQ_OFFSET;
    if (rst_ack == rst_req) {
    mlx4_warn(dev, "VF Reset succeed\n");
    return 0;
    }
    cond_resched();
    }
    mlx4_err(dev, "Fail to send reset over the communication channel\n");
    return -ETIMEDOUT;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_comm_internal_err(slave_read: u32) -> c_int {
    int mlx4_comm_internal_err(u32 slave_read)
    {
    return (u32)COMM_CHAN_EVENT_INTERNAL_ERR ==
    (slave_read & (u32)COMM_CHAN_EVENT_INTERNAL_ERR) ? 1 : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_enter_error_state(persist: *mut mlx4_dev_persistent) {
    void mlx4_enter_error_state(struct mlx4_dev_persistent *persist)
    {
    int err;
    struct mlx4_dev *dev;
    if (!mlx4_internal_err_reset)
    return;
    mutex_lock(&persist.device_state_mutex);
    if (persist.state & MLX4_DEVICE_STATE_INTERNAL_ERROR)
    goto out;
    dev = persist.dev;
    mlx4_err(dev, "device is going to be reset\n");
    if (mlx4_is_slave(dev)) {
    err = mlx4_reset_slave(dev);
    } else {
    mlx4_crdump_collect(dev);
    err = mlx4_reset_master(dev);
    }
    if (!err) {
    mlx4_err(dev, "device was reset successfully\n");
    } else {
// EEH could have disabled the PCI channel during reset. That's
// recoverable and the PCI error flow will handle it.
//
    if (!pci_channel_offline(dev.persist.pdev))
    BUG_ON(1);
    }
    dev.persist.state |= MLX4_DEVICE_STATE_INTERNAL_ERROR;
    mutex_unlock(&persist.device_state_mutex);
// At that step HW was already reset, now notify clients
    mlx4_dispatch_event(dev, MLX4_DEV_EVENT_CATASTROPHIC_ERROR, core::ptr::null_mut());
    mlx4_cmd_wake_completions(dev);
    return;
    out:
    mutex_unlock(&persist.device_state_mutex);
    }
#[no_mangle]
unsafe extern "C" fn mlx4_handle_error_state(persist: *mut mlx4_dev_persistent) {
    static void mlx4_handle_error_state(struct mlx4_dev_persistent *persist)
    {
    struct mlx4_dev *dev = persist.dev;
    struct devlink *devlink;
    let mut err: c_int = 0;
    mlx4_enter_error_state(persist);
    devlink = priv_to_devlink(mlx4_priv(dev));
    devl_lock(devlink);
    mutex_lock(&persist.interface_state_mutex);
    if (persist.interface_state & MLX4_INTERFACE_STATE_UP &&
    !(persist.interface_state & MLX4_INTERFACE_STATE_DELETION)) {
    err = mlx4_restart_one(persist.pdev);
    mlx4_info(persist.dev, "mlx4_restart_one was ended, ret=%d\n",
    err);
    }
    mutex_unlock(&persist.interface_state_mutex);
    devl_unlock(devlink);
    }
#[no_mangle]
unsafe extern "C" fn dump_err_buf(dev: *mut mlx4_dev) {
    static void dump_err_buf(struct mlx4_dev *dev)
    {
    struct mlx4_priv *priv = mlx4_priv(dev);
    int i;
    mlx4_err(dev, "Internal error detected:\n");
    for (i = 0; i < priv.fw.catas_size; ++i)
    mlx4_err(dev, "  buf[%02x]: %08x\n",
    i, swab32(readl(priv.catas_err.map + i)));
    }
#[no_mangle]
unsafe extern "C" fn poll_catas(t: *mut timer_list) {
    static void poll_catas(struct timer_list *t)
    {
    struct mlx4_priv *priv = timer_container_of(priv, t, catas_err.timer);
    struct mlx4_dev *dev = &priv.dev;
    u32 slave_read;
    if (mlx4_is_slave(dev)) {
    slave_read = swab32(readl(&priv.mfunc.comm.slave_read));
    if (mlx4_comm_internal_err(slave_read)) {
    mlx4_warn(dev, "Internal error detected on the communication channel\n");
    goto internal_err;
    }
    } else if (readl(priv.catas_err.map)) {
    dump_err_buf(dev);
    goto internal_err;
    }
    if (dev.persist.state & MLX4_DEVICE_STATE_INTERNAL_ERROR) {
    mlx4_warn(dev, "Internal error mark was detected on device\n");
    goto internal_err;
    }
    mod_timer(&priv.catas_err.timer,
    round_jiffies(jiffies + MLX4_CATAS_POLL_INTERVAL));
    return;
    internal_err:
    if (mlx4_internal_err_reset)
    queue_work(dev.persist.catas_wq, &dev.persist.catas_work);
    }
#[no_mangle]
unsafe extern "C" fn catas_reset(work: *mut work_struct) {
    static void catas_reset(struct work_struct *work)
    {
    struct mlx4_dev_persistent *persist =
    container_of(work, struct mlx4_dev_persistent,
    catas_work);
    mlx4_handle_error_state(persist);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_start_catas_poll(dev: *mut mlx4_dev) {
    void mlx4_start_catas_poll(struct mlx4_dev *dev)
    {
    struct mlx4_priv *priv = mlx4_priv(dev);
    phys_addr_t addr;
    INIT_LIST_HEAD(&priv.catas_err.list);
    timer_setup(&priv.catas_err.timer, poll_catas, 0);
    priv.catas_err.map = core::ptr::null_mut();
    if (!mlx4_is_slave(dev)) {
    addr = pci_resource_start(dev.persist.pdev,
    priv.fw.catas_bar) +
    priv.fw.catas_offset;
    priv.catas_err.map = ioremap(addr, priv.fw.catas_size * 4);
    if (!priv.catas_err.map) {
    mlx4_warn(dev, "Failed to map internal error buffer at 0x%llx\n",
    (unsigned long long)addr);
    return;
    }
    }
    priv.catas_err.timer.expires  =
    round_jiffies(jiffies + MLX4_CATAS_POLL_INTERVAL);
    add_timer(&priv.catas_err.timer);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_stop_catas_poll(dev: *mut mlx4_dev) {
    void mlx4_stop_catas_poll(struct mlx4_dev *dev)
    {
    struct mlx4_priv *priv = mlx4_priv(dev);
    timer_delete_sync(&priv.catas_err.timer);
    if (priv.catas_err.map) {
    iounmap(priv.catas_err.map);
    priv.catas_err.map = core::ptr::null_mut();
    }
    if (dev.persist.interface_state & MLX4_INTERFACE_STATE_DELETION)
    flush_workqueue(dev.persist.catas_wq);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_catas_init(dev: *mut mlx4_dev) -> c_int {
    int  mlx4_catas_init(struct mlx4_dev *dev)
    {
    INIT_WORK(&dev.persist.catas_work, catas_reset);
    dev.persist.catas_wq = create_singlethread_workqueue("mlx4_health");
    if (!dev.persist.catas_wq)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_catas_end(dev: *mut mlx4_dev) {
    void mlx4_catas_end(struct mlx4_dev *dev)
    {
    if (dev.persist.catas_wq) {
    destroy_workqueue(dev.persist.catas_wq);
    dev.persist.catas_wq = core::ptr::null_mut();
    }
    }
