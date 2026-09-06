//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/intel/ivsc/mei_ace.c
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
// Copyright (C) 2023 Intel Corporation. All rights reserved.
// Intel Visual Sensing Controller ACE Linux driver
//
// To set ownership of camera sensor, there is specific command, which
// is sent via MEI protocol. That's a two-step scheme where the firmware
// first acks receipt of the command and later responses the command was
// executed. The command sending function uses "completion" as the
// synchronization mechanism. The notification for command is received
// via a mei callback which wakes up the caller. There can be only one
// outstanding command at a time.
//
// The power line of camera sensor is directly connected to IVSC instead
// of host, when camera sensor ownership is switched to host, sensor is
// already powered up by firmware.
//

// indicating driver message
pub const ACE_DRV_MSG: c_int = 1;
// indicating set command
pub const ACE_CMD_SET: c_int = 4;
// command timeout determined experimentally

// indicating the first command block
pub const ACE_CMD_INIT_BLOCK: c_int = 1;
// indicating the last command block
pub const ACE_CMD_FINAL_BLOCK: c_int = 1;
// size of camera status notification content
pub const ACE_CAMERA_STATUS_SIZE: c_int = 5;
// UUID used to get firmware id

    0xE3, 0x84, 0x17, 0x71, 0xAA, 0x79, 0x0B)
// UUID used to get csi device

    0xAF, 0x93, 0x7b, 0x44, 0x53, 0xAC, 0x29, 0xDA)
// identify firmware event type
    enum ace_event_type {
// firmware ready
    ACE_FW_READY = 0x8,
// command response
    ACE_CMD_RESPONSE = 0x10,
    };
// identify camera sensor ownership
    enum ace_camera_owner {
    ACE_CAMERA_IVSC,
    ACE_CAMERA_HOST,
    };
// identify the command id supported by firmware IPC
    enum ace_cmd_id {
// used to switch camera sensor to host
    ACE_SWITCH_CAMERA_TO_HOST = 0x13,
// used to switch camera sensor to IVSC
    ACE_SWITCH_CAMERA_TO_IVSC = 0x14,
// used to get firmware id
    ACE_GET_FW_ID = 0x1A,
    };
// ACE command header structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ace_cmd_hdr {
    pub 16: u32 firmware_id :,
    pub 8: u32 instance_id :,
    pub 5: u32 type :,
    pub 1: u32 rsp :,
    pub 1: u32 msg_tgt :,
    pub 1: u32 _hw_rsvd_1 :,
    pub 20: u32 param_size :,
    pub 8: u32 cmd_id :,
    pub 1: u32 final_block :,
    pub 1: u32 init_block :,
    pub 2: u32 _hw_rsvd_2 :,
    pub __packed: },
// ACE command parameter structure
    union ace_cmd_param {
    pub uuid: uuid_le,
    pub param: u32,
}

// ACE command structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ace_cmd {
    pub hdr: ace_cmd_hdr,
    pub param: union ace_cmd_param,
    pub __packed: },
// ACE notification header
    union ace_notif_hdr {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _confirm {
    pub 24: u32 status :,
    pub 5: u32 type :,
    pub 1: u32 rsp :,
    pub 1: u32 msg_tgt :,
    pub 1: u32 _hw_rsvd_1 :,
    pub 20: u32 param_size :,
    pub 8: u32 cmd_id :,
    pub 1: u32 final_block :,
    pub 1: u32 init_block :,
    pub 2: u32 _hw_rsvd_2 :,
    pub ack: } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _event {
    pub 16: u32 rsvd1 :,
    pub 8: u32 event_type :,
    pub 5: u32 type :,
    pub 1: u32 ack :,
    pub 1: u32 msg_tgt :,
    pub 1: u32 _hw_rsvd_1 :,
    pub 30: u32 rsvd2 :,
    pub 2: u32 _hw_rsvd_2 :,
    pub event: } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _response {
    pub 16: u32 event_id :,
    pub 8: u32 notif_type :,
    pub 5: u32 type :,
    pub 1: u32 rsp :,
    pub 1: u32 msg_tgt :,
    pub 1: u32 _hw_rsvd_1 :,
    pub 16: u32 event_data_size :,
    pub 1: u32 request_target :,
    pub 5: u32 request_type :,
    pub 8: u32 cmd_id :,
    pub 2: u32 _hw_rsvd_2 :,
    pub response: } __packed,
}

// ACE notification content
    union ace_notif_cont {
    u16 firmware_id;
    u8 state_notif;
    u8 camera_status[ACE_CAMERA_STATUS_SIZE];
    };
// ACE notification structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ace_notif {
    pub hdr: union ace_notif_hdr,
    pub cont: union ace_notif_cont,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_ace {
    pub cldev: *mut mei_cl_device,
// command ack
    pub cmd_ack: ace_notif,
// command response
    pub cmd_response: ace_notif,
// used to wait for command ack and response
    pub cmd_completion: completion,
// lock used to prevent multiple call to send command
    pub lock: mutex,
// used to construct command
    pub firmware_id: u16,
    pub csi_dev: *mut device,
// runtime PM link from ace to csi
    pub csi_link: *mut device_link,
    pub work: work_struct,
}

#[no_mangle]
pub unsafe extern "C" fn init_cmd_hdr(hdr: *mut ace_cmd_hdr) {
    static inline void init_cmd_hdr(struct ace_cmd_hdr *hdr)
    {
    memset(hdr, 0, sizeof(struct ace_cmd_hdr));
    hdr.type = ACE_CMD_SET;
    hdr.msg_tgt = ACE_DRV_MSG;
    hdr.init_block = ACE_CMD_INIT_BLOCK;
    hdr.final_block = ACE_CMD_FINAL_BLOCK;
    }
    static int construct_command(struct mei_ace *ace, struct ace_cmd *cmd,
    enum ace_cmd_id cmd_id)
    {
    union ace_cmd_param *param = &cmd.param;
    struct ace_cmd_hdr *hdr = &cmd.hdr;
    init_cmd_hdr(hdr);
    hdr.cmd_id = cmd_id;
    switch (cmd_id) {
    case ACE_GET_FW_ID:
    param.uuid = ACE_GET_FW_ID_UUID;
    hdr.param_size = sizeof(param.uuid);
    break;
    case ACE_SWITCH_CAMERA_TO_IVSC:
    param.param = 0;
    hdr.firmware_id = ace.firmware_id;
    hdr.param_size = sizeof(param.param);
    break;
    case ACE_SWITCH_CAMERA_TO_HOST:
    hdr.firmware_id = ace.firmware_id;
    break;
    default:
    return -EINVAL;
    }
    return hdr.param_size + sizeof(cmd.hdr);
    }
// send command to firmware
    static int mei_ace_send(struct mei_ace *ace, struct ace_cmd *cmd,
    size_t len, bool only_ack)
    {
    union ace_notif_hdr *resp_hdr = &ace.cmd_response.hdr;
    union ace_notif_hdr *ack_hdr = &ace.cmd_ack.hdr;
    struct ace_cmd_hdr *cmd_hdr = &cmd.hdr;
    int ret;
    mutex_lock(&ace.lock);
    reinit_completion(&ace.cmd_completion);
    ret = mei_cldev_send(ace.cldev, (u8 *)cmd, len);
    if (ret < 0)
    goto out;
    ret = wait_for_completion_killable_timeout(&ace.cmd_completion,
    ACE_CMD_TIMEOUT);
    if (ret < 0) {
    goto out;
    } else if (!ret) {
    ret = -ETIMEDOUT;
    goto out;
    }
    if (ack_hdr.ack.cmd_id != cmd_hdr.cmd_id) {
    ret = -EINVAL;
    goto out;
    }
// command ack status
    ret = ack_hdr.ack.status;
    if (ret) {
    ret = -EIO;
    goto out;
    }
    if (only_ack)
    goto out;
    ret = wait_for_completion_killable_timeout(&ace.cmd_completion,
    ACE_CMD_TIMEOUT);
    if (ret < 0) {
    goto out;
    } else if (!ret) {
    ret = -ETIMEDOUT;
    goto out;
    } else {
    ret = 0;
    }
    if (resp_hdr.response.cmd_id != cmd_hdr.cmd_id)
    ret = -EINVAL;
    out:
    mutex_unlock(&ace.lock);
    return ret;
    }
    static int ace_set_camera_owner(struct mei_ace *ace,
    enum ace_camera_owner owner)
    {
    enum ace_cmd_id cmd_id;
    struct ace_cmd cmd;
    int cmd_size;
    int ret;
    if (owner == ACE_CAMERA_IVSC)
    cmd_id = ACE_SWITCH_CAMERA_TO_IVSC;
    else
    cmd_id = ACE_SWITCH_CAMERA_TO_HOST;
    cmd_size = construct_command(ace, &cmd, cmd_id);
    if (cmd_size >= 0)
    ret = mei_ace_send(ace, &cmd, cmd_size, false);
    else
    ret = cmd_size;
    return ret;
    }
// the first command downloaded to firmware
#[no_mangle]
pub unsafe extern "C" fn ace_get_firmware_id(ace: *mut mei_ace) -> c_int {
    static inline int ace_get_firmware_id(struct mei_ace *ace)
    {
    struct ace_cmd cmd;
    int cmd_size;
    int ret;
    cmd_size = construct_command(ace, &cmd, ACE_GET_FW_ID);
    if (cmd_size >= 0)
    ret = mei_ace_send(ace, &cmd, cmd_size, true);
    else
    ret = cmd_size;
    return ret;
    }
    static void handle_command_response(struct mei_ace *ace,
    struct ace_notif *resp, int len)
    {
    union ace_notif_hdr *hdr = &resp.hdr;
    switch (hdr.response.cmd_id) {
    case ACE_SWITCH_CAMERA_TO_IVSC:
    case ACE_SWITCH_CAMERA_TO_HOST:
    memcpy(&ace.cmd_response, resp, len);
    complete(&ace.cmd_completion);
    break;
    case ACE_GET_FW_ID:
    break;
    default:
    break;
    }
    }
    static void handle_command_ack(struct mei_ace *ace,
    struct ace_notif *ack, int len)
    {
    union ace_notif_hdr *hdr = &ack.hdr;
    switch (hdr.ack.cmd_id) {
    case ACE_GET_FW_ID:
    ace.firmware_id = ack.cont.firmware_id;
    fallthrough;
    case ACE_SWITCH_CAMERA_TO_IVSC:
    case ACE_SWITCH_CAMERA_TO_HOST:
    memcpy(&ace.cmd_ack, ack, len);
    complete(&ace.cmd_completion);
    break;
    default:
    break;
    }
    }
// callback for receive
#[no_mangle]
unsafe extern "C" fn mei_ace_rx(cldev: *mut mei_cl_device) {
    static void mei_ace_rx(struct mei_cl_device *cldev)
    {
    struct mei_ace *ace = mei_cldev_get_drvdata(cldev);
    struct ace_notif event;
    union ace_notif_hdr *hdr = &event.hdr;
    int ret;
    ret = mei_cldev_recv(cldev, (u8 *)&event, sizeof(event));
    if (ret < 0) {
    dev_err(&cldev.dev, "recv error: %d\n", ret);
    return;
    }
    if (hdr.event.ack) {
    handle_command_ack(ace, &event, ret);
    return;
    }
    switch (hdr.event.event_type) {
    case ACE_CMD_RESPONSE:
    handle_command_response(ace, &event, ret);
    break;
    case ACE_FW_READY:
//
// firmware ready notification sent to driver
// after HECI client connected with firmware.
//
    dev_dbg(&cldev.dev, "firmware ready\n");
    break;
    default:
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn mei_ace_setup_dev_link(ace: *mut mei_ace) -> c_int {
    static int mei_ace_setup_dev_link(struct mei_ace *ace)
    {
    struct device *dev = &ace.cldev.dev;
    let mut uuid: uuid_le = MEI_CSI_UUID;
    struct device *csi_dev;
    char name[64];
    int ret;
    snprintf(name, sizeof(name), "%s-%pUl", dev_name(dev.parent), &uuid);
    csi_dev = device_find_child_by_name(dev.parent, name);
    if (!csi_dev) {
    ret = -EPROBE_DEFER;
    goto err;
    } else if (!dev_fwnode(csi_dev)) {
    ret = -EPROBE_DEFER;
    goto err_put;
    }
// setup link between mei_ace and mei_csi
    ace.csi_link = device_link_add(csi_dev, dev, DL_FLAG_PM_RUNTIME |
    DL_FLAG_RPM_ACTIVE | DL_FLAG_STATELESS);
    put_device(csi_dev);
    if (!ace.csi_link) {
    ret = -EINVAL;
    dev_err(dev, "failed to link to %s\n", dev_name(csi_dev));
    goto err;
    }
    ace.csi_dev = csi_dev;
    return 0;
    err_put:
    put_device(csi_dev);
    err:
    return ret;
    }
// switch camera to host before probe sensor device
#[no_mangle]
unsafe extern "C" fn mei_ace_post_probe_work(work: *mut work_struct) {
    static void mei_ace_post_probe_work(struct work_struct *work)
    {
    struct acpi_device *adev;
    struct mei_ace *ace;
    struct device *dev;
    int ret;
    ace = container_of(work, struct mei_ace, work);
    dev = &ace.cldev.dev;
    ret = ace_set_camera_owner(ace, ACE_CAMERA_HOST);
    if (ret) {
    dev_err(dev, "switch camera to host failed: %d\n", ret);
    return;
    }
    adev = ACPI_COMPANION(dev.parent);
    if (!adev)
    return;
    acpi_dev_clear_dependencies(adev);
    }
    static int mei_ace_probe(struct mei_cl_device *cldev,
    const struct mei_cl_device_id *id)
    {
    struct device *dev = &cldev.dev;
    struct mei_ace *ace;
    int ret;
    ace = devm_kzalloc(dev, sizeof(struct mei_ace), GFP_KERNEL);
    if (!ace)
    return -ENOMEM;
    ace.cldev = cldev;
    mutex_init(&ace.lock);
    init_completion(&ace.cmd_completion);
    INIT_WORK(&ace.work, mei_ace_post_probe_work);
    mei_cldev_set_drvdata(cldev, ace);
    ret = mei_cldev_enable(cldev);
    if (ret < 0) {
    dev_err(dev, "mei_cldev_enable failed: %d\n", ret);
    goto destroy_mutex;
    }
    ret = mei_cldev_register_rx_cb(cldev, mei_ace_rx);
    if (ret) {
    dev_err(dev, "event cb registration failed: %d\n", ret);
    goto err_disable;
    }
    ret = ace_get_firmware_id(ace);
    if (ret) {
    dev_err(dev, "get firmware id failed: %d\n", ret);
    goto err_disable;
    }
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    ret = mei_ace_setup_dev_link(ace);
    if (ret)
    goto disable_pm;
    schedule_work(&ace.work);
    return 0;
    disable_pm:
    pm_runtime_disable(dev);
    pm_runtime_set_suspended(dev);
    err_disable:
    mei_cldev_disable(cldev);
    destroy_mutex:
    mutex_destroy(&ace.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mei_ace_remove(cldev: *mut mei_cl_device) {
    static void mei_ace_remove(struct mei_cl_device *cldev)
    {
    struct mei_ace *ace = mei_cldev_get_drvdata(cldev);
    cancel_work_sync(&ace.work);
    device_link_del(ace.csi_link);
    pm_runtime_disable(&cldev.dev);
    pm_runtime_set_suspended(&cldev.dev);
    ace_set_camera_owner(ace, ACE_CAMERA_IVSC);
    mei_cldev_disable(cldev);
    mutex_destroy(&ace.lock);
    }
#[no_mangle]
unsafe extern "C" fn mei_ace_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused mei_ace_runtime_suspend(struct device *dev)
    {
    struct mei_ace *ace = dev_get_drvdata(dev);
    return ace_set_camera_owner(ace, ACE_CAMERA_IVSC);
    }
#[no_mangle]
unsafe extern "C" fn mei_ace_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused mei_ace_runtime_resume(struct device *dev)
    {
    struct mei_ace *ace = dev_get_drvdata(dev);
    return ace_set_camera_owner(ace, ACE_CAMERA_HOST);
    }
    static const struct dev_pm_ops mei_ace_pm_ops = {
    SET_RUNTIME_PM_OPS(mei_ace_runtime_suspend,
    mei_ace_runtime_resume, core::ptr::null_mut())
    };

    0x9B, 0x78, 0x03, 0x61, 0x63, 0x5E, 0x24, 0x47)
    static const struct mei_cl_device_id mei_ace_tbl[] = {
    { .uuid = MEI_ACE_UUID, .version = MEI_CL_VERSION_ANY },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(mei, mei_ace_tbl);
    static struct mei_cl_driver mei_ace_driver = {
    .id_table = mei_ace_tbl,
    .name = KBUILD_MODNAME,
    .probe = mei_ace_probe,
    .remove = mei_ace_remove,
    .driver = {
    .pm = &mei_ace_pm_ops,
    },
    };
    module_mei_cl_driver(mei_ace_driver);
    MODULE_AUTHOR("Wentong Wu");
    MODULE_AUTHOR("Zhifeng Wang <zhifeng.wang@intel.com>");
    MODULE_DESCRIPTION("Device driver for IVSC ACE");
    MODULE_LICENSE("GPL");
