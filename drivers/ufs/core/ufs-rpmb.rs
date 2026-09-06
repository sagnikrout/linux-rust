//! Automatically rewritten from C to Rust
//! Source: drivers/ufs/core/ufs-rpmb.c
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
// UFS OP-TEE based RPMB Driver
//
// Copyright (C) 2025 Micron Technology, Inc.
// Copyright (C) 2025 Qualcomm Technologies, Inc.
//
// Authors:
// Bean Huo <beanhuo@micron.com>
// Can Guo <can.guo@oss.qualcomm.com>
//

pub const UFS_RPMB_SEC_PROTOCOL: c_uint = 0xEC	/* JEDEC UFS application */;
pub const UFS_RPMB_SEC_PROTOCOL_ID: c_uint = 0x01	/* JEDEC UFS RPMB protocol ID, CDB byte3 */;
    static const struct bus_type ufs_rpmb_bus_type = {
    .name = "ufs_rpmb",
    };
// UFS RPMB device structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_rpmb_dev {
    pub region_id: u8,
    pub dev: device,
    pub rdev: *mut rpmb_dev,
    pub hba: *mut ufs_hba,
    pub node: list_head,
}

#[no_mangle]
unsafe extern "C" fn ufs_sec_submit(hba: *mut ufs_hba, spsp: u16, buffer: *mut c_void, len: usize, send: bool) -> c_int {
    static int ufs_sec_submit(struct ufs_hba *hba, u16 spsp, void *buffer, size_t len, bool send)
    {
    struct scsi_device *sdev = hba.ufs_rpmb_wlun;
    u8 cdb[12] = { };
    cdb[0] = send ? SECURITY_PROTOCOL_OUT : SECURITY_PROTOCOL_IN;
    cdb[1] = UFS_RPMB_SEC_PROTOCOL;
    put_unaligned_be16(spsp, &cdb[2]);
    put_unaligned_be32(len, &cdb[6]);
    return scsi_execute_cmd(sdev, cdb, send ? REQ_OP_DRV_OUT : REQ_OP_DRV_IN,
    buffer, len, /*timeout=*/30 * HZ, 0, core::ptr::null_mut());
    }
// UFS RPMB route frames implementation
    static int ufs_rpmb_route_frames(struct device *dev, u8 *req, unsigned int req_len, u8 *resp,
    unsigned int resp_len)
    {
    struct ufs_rpmb_dev *ufs_rpmb = dev_get_drvdata(dev);
    struct rpmb_frame *frm_out = (struct rpmb_frame *)req;
    let mut need_result_read: bool = true;
    u16 req_type, protocol_id;
    struct ufs_hba *hba;
    int ret;
    if (!ufs_rpmb) {
    dev_err(dev, "Missing driver data\n");
    return -ENODEV;
    }
    hba = ufs_rpmb.hba;
// req_resp is at the end of an RPMB frame.
    if (req_len < sizeof(*frm_out))
    return -EINVAL;
    req_type = get_unaligned_be16(&frm_out.req_resp);
    switch (req_type) {
    case RPMB_PROGRAM_KEY:
    if (req_len != sizeof(struct rpmb_frame) || resp_len != sizeof(struct rpmb_frame))
    return -EINVAL;
    break;
    case RPMB_GET_WRITE_COUNTER:
    if (req_len != sizeof(struct rpmb_frame) || resp_len != sizeof(struct rpmb_frame))
    return -EINVAL;
    need_result_read = false;
    break;
    case RPMB_WRITE_DATA:
    if (req_len % sizeof(struct rpmb_frame) || resp_len != sizeof(struct rpmb_frame))
    return -EINVAL;
    break;
    case RPMB_READ_DATA:
    if (req_len != sizeof(struct rpmb_frame) || resp_len % sizeof(struct rpmb_frame))
    return -EINVAL;
    need_result_read = false;
    break;
    default:
    dev_err(dev, "Unknown request type=0x%04x\n", req_type);
    return -EINVAL;
    }
    protocol_id = ufs_rpmb.region_id << 8 | UFS_RPMB_SEC_PROTOCOL_ID;
    ret = ufs_sec_submit(hba, protocol_id, req, req_len, true);
    if (ret) {
    dev_err(dev, "Command failed with ret=%d\n", ret);
    return ret;
    }
    if (need_result_read) {
    struct rpmb_frame *frm_resp = (struct rpmb_frame *)resp;
    memset(frm_resp, 0, sizeof(*frm_resp));
    put_unaligned_be16(RPMB_RESULT_READ, &frm_resp.req_resp);
    ret = ufs_sec_submit(hba, protocol_id, resp, resp_len, true);
    if (ret) {
    dev_err(dev, "Result read request failed with ret=%d\n", ret);
    return ret;
    }
    }
    if (!ret) {
    ret = ufs_sec_submit(hba, protocol_id, resp, resp_len, false);
    if (ret)
    dev_err(dev, "Response read failed with ret=%d\n", ret);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ufs_rpmb_device_release(dev: *mut device) {
    static void ufs_rpmb_device_release(struct device *dev)
    {
    struct ufs_rpmb_dev *ufs_rpmb = dev_get_drvdata(dev);
    rpmb_dev_unregister(ufs_rpmb.rdev);
    }
// UFS RPMB device registration
#[no_mangle]
pub unsafe extern "C" fn ufs_rpmb_probe(hba: *mut ufs_hba) -> c_int {
    int ufs_rpmb_probe(struct ufs_hba *hba)
    {
    struct ufs_rpmb_dev *ufs_rpmb, *it, *tmp;
    struct rpmb_dev *rdev;
    char *cid = core::ptr::null_mut();
    int region;
    u32 cap;
    int ret;
    if (!hba.ufs_rpmb_wlun || hba.dev_info.b_advanced_rpmb_en) {
    dev_info(hba.dev, "Skip OP-TEE RPMB registration\n");
    return -ENODEV;
    }
// Check if device_id is available
    if (!hba.dev_info.device_id) {
    dev_err(hba.dev, "UFS Device ID not available\n");
    return -EINVAL;
    }
    struct rpmb_descr descr = {
    .type = RPMB_TYPE_UFS,
    .route_frames = ufs_rpmb_route_frames,
    .reliable_wr_count = hba.dev_info.rpmb_io_size,
    };
    for (region = 0; region < ARRAY_SIZE(hba.dev_info.rpmb_region_size); region++) {
    cap = hba.dev_info.rpmb_region_size[region];
    if (!cap)
    continue;
    ufs_rpmb = devm_kzalloc(hba.dev, sizeof(*ufs_rpmb), GFP_KERNEL);
    if (!ufs_rpmb) {
    ret = -ENOMEM;
    goto err_out;
    }
    ufs_rpmb.hba = hba;
    ufs_rpmb.dev.parent = &hba.ufs_rpmb_wlun.sdev_gendev;
    ufs_rpmb.dev.bus = &ufs_rpmb_bus_type;
    ufs_rpmb.dev.release = ufs_rpmb_device_release;
    dev_set_name(&ufs_rpmb.dev, "ufs_rpmb%d", region);
// Set driver data BEFORE device_register
    dev_set_drvdata(&ufs_rpmb.dev, ufs_rpmb);
    ret = device_register(&ufs_rpmb.dev);
    if (ret) {
    dev_err(hba.dev, "Failed to register UFS RPMB device %d\n", region);
    put_device(&ufs_rpmb.dev);
    goto err_out;
    }
// Create unique ID by appending region number to device_id
    cid = kasprintf(GFP_KERNEL, "%s-R%d", hba.dev_info.device_id, region);
    if (!cid) {
    device_unregister(&ufs_rpmb.dev);
    ret = -ENOMEM;
    goto err_out;
    }
    descr.dev_id = cid;
    descr.dev_id_len = strlen(cid);
    descr.capacity = cap;
// Register RPMB device
    rdev = rpmb_dev_register(&ufs_rpmb.dev, &descr);
    if (IS_ERR(rdev)) {
    dev_err(hba.dev, "Failed to register UFS RPMB device.\n");
    device_unregister(&ufs_rpmb.dev);
    ret = PTR_ERR(rdev);
    goto err_out;
    }
    kfree(cid);
    cid = core::ptr::null_mut();
    ufs_rpmb.rdev = rdev;
    ufs_rpmb.region_id = region;
    list_add_tail(&ufs_rpmb.node, &hba.rpmbs);
    dev_info(hba.dev, "UFS RPMB region %d registered (capacity=%u)\n", region, cap);
    }
    return 0;
    err_out:
    kfree(cid);
    list_for_each_entry_safe(it, tmp, &hba.rpmbs, node) {
    list_del(&it.node);
    device_unregister(&it.dev);
    }
    return ret;
    }
// UFS RPMB remove handler
#[no_mangle]
pub unsafe extern "C" fn ufs_rpmb_remove(hba: *mut ufs_hba) {
    void ufs_rpmb_remove(struct ufs_hba *hba)
    {
    struct ufs_rpmb_dev *ufs_rpmb, *tmp;
    if (list_empty(&hba.rpmbs))
    return;
// Remove all registered RPMB devices
    list_for_each_entry_safe(ufs_rpmb, tmp, &hba.rpmbs, node) {
    dev_info(hba.dev, "Removing UFS RPMB region %d\n", ufs_rpmb.region_id);
// Remove from list first
    list_del(&ufs_rpmb.node);
// Unregister device
    device_unregister(&ufs_rpmb.dev);
    }
    dev_info(hba.dev, "All UFS RPMB devices unregistered\n");
    }
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("OP-TEE UFS RPMB driver");
