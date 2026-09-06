//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/pensando/ionic/ionic_fw.c
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
// Copyright(c) 2020 Pensando Systems, Inc

// The worst case wait for the install activity is about 25 minutes when
// installing a new CPLD, which is very seldom.  Normal is about 30-35
// seconds.  Since the driver can't tell if a CPLD update will happen we
// set the timeout for the ugly case.
//

pub const IONIC_FW_SELECT_TIMEOUT: c_int = 30;
// Number of periodic log updates during fw file download
pub const IONIC_FW_INTERVAL_FRACTION: c_int = 32;
    static void ionic_dev_cmd_firmware_download(struct ionic_dev *idev, u64 addr,
    u32 offset, u32 length)
    {
    union ionic_dev_cmd cmd = {
    .fw_download.opcode = IONIC_CMD_FW_DOWNLOAD,
    .fw_download.offset = cpu_to_le32(offset),
    .fw_download.addr = cpu_to_le64(addr),
    .fw_download.length = cpu_to_le32(length),
    };
    ionic_dev_cmd_go(idev, &cmd);
    }
#[no_mangle]
unsafe extern "C" fn ionic_dev_cmd_firmware_install(idev: *mut ionic_dev) {
    static void ionic_dev_cmd_firmware_install(struct ionic_dev *idev)
    {
    union ionic_dev_cmd cmd = {
    .fw_control.opcode = IONIC_CMD_FW_CONTROL,
    .fw_control.oper = IONIC_FW_INSTALL_ASYNC
    };
    ionic_dev_cmd_go(idev, &cmd);
    }
#[no_mangle]
unsafe extern "C" fn ionic_dev_cmd_firmware_activate(idev: *mut ionic_dev, slot: u8) {
    static void ionic_dev_cmd_firmware_activate(struct ionic_dev *idev, u8 slot)
    {
    union ionic_dev_cmd cmd = {
    .fw_control.opcode = IONIC_CMD_FW_CONTROL,
    .fw_control.oper = IONIC_FW_ACTIVATE_ASYNC,
    .fw_control.slot = slot
    };
    ionic_dev_cmd_go(idev, &cmd);
    }
    static int ionic_fw_status_long_wait(struct ionic *ionic,
    const char *label,
    unsigned long timeout,
    u8 fw_cmd,
    struct netlink_ext_ack *extack)
    {
    union ionic_dev_cmd cmd = {
    .fw_control.opcode = IONIC_CMD_FW_CONTROL,
    .fw_control.oper = fw_cmd,
    };
    unsigned long start_time;
    unsigned long end_time;
    int err;
    start_time = jiffies;
    end_time = start_time + (timeout * HZ);
    do {
    mutex_lock(&ionic.dev_cmd_lock);
    ionic_dev_cmd_go(&ionic.idev, &cmd);
    err = ionic_dev_cmd_wait(ionic, DEVCMD_TIMEOUT);
    mutex_unlock(&ionic.dev_cmd_lock);
    msleep(20);
    } while (time_before(jiffies, end_time) && (err == -EAGAIN || err == -ETIMEDOUT));
    if (err == -EAGAIN || err == -ETIMEDOUT) {
    NL_SET_ERR_MSG_MOD(extack, "Firmware wait timed out");
    dev_err(ionic.dev, "DEV_CMD firmware wait %s timed out\n", label);
    } else if (err) {
    NL_SET_ERR_MSG_MOD(extack, "Firmware wait failed");
    }
    return err;
    }
    int ionic_firmware_update(struct ionic_lif *lif, const struct firmware *fw,
    struct netlink_ext_ack *extack)
    {
    struct ionic_dev *idev = &lif.ionic.idev;
    struct net_device *netdev = lif.netdev;
    struct ionic *ionic = lif.ionic;
    union ionic_dev_cmd_comp comp;
    u32 buf_sz, copy_sz, offset;
    struct devlink *dl;
    int next_interval;
    let mut err: c_int = 0;
    u8 fw_slot;
    netdev_info(netdev, "Installing firmware\n");
    dl = priv_to_devlink(ionic);
    devlink_flash_update_status_notify(dl, "Preparing to flash", core::ptr::null_mut(), 0, 0);
    if (!idev.dev_cmd_regs) {
    err = -ENXIO;
    goto err_out;
    }
    buf_sz = sizeof(idev.dev_cmd_regs.data);
    netdev_dbg(netdev,
    "downloading firmware - size %d part_sz %d nparts %lu\n",
    (int)fw.size, buf_sz, DIV_ROUND_UP(fw.size, buf_sz));
    offset = 0;
    next_interval = 0;
    while (offset < fw.size) {
    if (offset >= next_interval) {
    devlink_flash_update_status_notify(dl, "Downloading", core::ptr::null_mut(),
    offset, fw.size);
    next_interval = offset + (fw.size / IONIC_FW_INTERVAL_FRACTION);
    }
    copy_sz = min_t(unsigned int, buf_sz, fw.size - offset);
    mutex_lock(&ionic.dev_cmd_lock);
    memcpy_toio(&idev.dev_cmd_regs.data, fw.data + offset, copy_sz);
    ionic_dev_cmd_firmware_download(idev,
    offsetof(union ionic_dev_cmd_regs, data),
    offset, copy_sz);
    err = ionic_dev_cmd_wait(ionic, DEVCMD_TIMEOUT);
    mutex_unlock(&ionic.dev_cmd_lock);
    if (err) {
    netdev_err(netdev,
    "download failed offset 0x%x addr 0x%lx len 0x%x\n",
    offset, offsetof(union ionic_dev_cmd_regs, data),
    copy_sz);
    NL_SET_ERR_MSG_MOD(extack, "Segment download failed");
    goto err_out;
    }
    offset += copy_sz;
    }
    devlink_flash_update_status_notify(dl, "Downloading", core::ptr::null_mut(),
    fw.size, fw.size);
    devlink_flash_update_timeout_notify(dl, "Installing", core::ptr::null_mut(),
    IONIC_FW_INSTALL_TIMEOUT);
    mutex_lock(&ionic.dev_cmd_lock);
    ionic_dev_cmd_firmware_install(idev);
    err = ionic_dev_cmd_wait(ionic, DEVCMD_TIMEOUT);
    ionic_dev_cmd_comp(idev, (union ionic_dev_cmd_comp *)&comp);
    fw_slot = comp.fw_control.slot;
    mutex_unlock(&ionic.dev_cmd_lock);
    if (err) {
    NL_SET_ERR_MSG_MOD(extack, "Failed to start firmware install");
    goto err_out;
    }
    err = ionic_fw_status_long_wait(ionic, "Installing",
    IONIC_FW_INSTALL_TIMEOUT,
    IONIC_FW_INSTALL_STATUS,
    extack);
    if (err)
    goto err_out;
    devlink_flash_update_timeout_notify(dl, "Selecting", core::ptr::null_mut(),
    IONIC_FW_SELECT_TIMEOUT);
    mutex_lock(&ionic.dev_cmd_lock);
    ionic_dev_cmd_firmware_activate(idev, fw_slot);
    err = ionic_dev_cmd_wait(ionic, DEVCMD_TIMEOUT);
    mutex_unlock(&ionic.dev_cmd_lock);
    if (err) {
    NL_SET_ERR_MSG_MOD(extack, "Failed to start firmware select");
    goto err_out;
    }
    err = ionic_fw_status_long_wait(ionic, "Selecting",
    IONIC_FW_SELECT_TIMEOUT,
    IONIC_FW_ACTIVATE_STATUS,
    extack);
    if (err)
    goto err_out;
    netdev_info(netdev, "Firmware update completed\n");
    err_out:
    if (err)
    devlink_flash_update_status_notify(dl, "Flash failed", core::ptr::null_mut(), 0, 0);
    else
    devlink_flash_update_status_notify(dl, "Flash done", core::ptr::null_mut(), 0, 0);
    return err;
    }
