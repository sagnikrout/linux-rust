//! Automatically rewritten from C to Rust
//! Source: drivers/nfc/nxp-nci/firmware.c
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
// Generic driver for NXP NCI NFC chips
//
// Copyright (C) 2014  NXP Semiconductors  All rights reserved.
//
// Author: Clément Perrochaud <clement.perrochaud@nxp.com>
//
// Derived from PN544 device driver:
// Copyright (C) 2012  Intel Corporation. All rights reserved.
//

// Crypto operations can take up to 30 seconds

pub const NXP_NCI_FW_CMD_RESET: c_uint = 0xF0;
pub const NXP_NCI_FW_CMD_GETVERSION: c_uint = 0xF1;
pub const NXP_NCI_FW_CMD_CHECKINTEGRITY: c_uint = 0xE0;
pub const NXP_NCI_FW_CMD_WRITE: c_uint = 0xC0;
pub const NXP_NCI_FW_CMD_READ: c_uint = 0xA2;
pub const NXP_NCI_FW_CMD_GETSESSIONSTATE: c_uint = 0xF2;
pub const NXP_NCI_FW_CMD_LOG: c_uint = 0xA7;
pub const NXP_NCI_FW_CMD_FORCE: c_uint = 0xD0;
pub const NXP_NCI_FW_CMD_GET_DIE_ID: c_uint = 0xF4;
pub const NXP_NCI_FW_CHUNK_FLAG: c_uint = 0x0400;
pub const NXP_NCI_FW_RESULT_OK: c_uint = 0x00;
pub const NXP_NCI_FW_RESULT_INVALID_ADDR: c_uint = 0x01;
pub const NXP_NCI_FW_RESULT_GENERIC_ERROR: c_uint = 0x02;
pub const NXP_NCI_FW_RESULT_UNKNOWN_CMD: c_uint = 0x0B;
pub const NXP_NCI_FW_RESULT_ABORTED_CMD: c_uint = 0x0C;
pub const NXP_NCI_FW_RESULT_PLL_ERROR: c_uint = 0x0D;
pub const NXP_NCI_FW_RESULT_ADDR_RANGE_OFL_ERROR: c_uint = 0x1E;
pub const NXP_NCI_FW_RESULT_BUFFER_OFL_ERROR: c_uint = 0x1F;
pub const NXP_NCI_FW_RESULT_MEM_BSY: c_uint = 0x20;
pub const NXP_NCI_FW_RESULT_SIGNATURE_ERROR: c_uint = 0x21;
pub const NXP_NCI_FW_RESULT_FIRMWARE_VERSION_ERROR: c_uint = 0x24;
pub const NXP_NCI_FW_RESULT_PROTOCOL_ERROR: c_uint = 0x28;
pub const NXP_NCI_FW_RESULT_SFWU_DEGRADED: c_uint = 0x2A;
pub const NXP_NCI_FW_RESULT_PH_STATUS_FIRST_CHUNK: c_uint = 0x2D;
pub const NXP_NCI_FW_RESULT_PH_STATUS_NEXT_CHUNK: c_uint = 0x2E;
pub const NXP_NCI_FW_RESULT_PH_STATUS_INTERNAL_ERROR_5: c_uint = 0xC5;
#[no_mangle]
pub unsafe extern "C" fn nxp_nci_fw_work_complete(info: *mut nxp_nci_info, result: c_int) {
    void nxp_nci_fw_work_complete(struct nxp_nci_info *info, int result)
    {
    struct nxp_nci_fw_info *fw_info = &info.fw_info;
    int r;
    if (info.phy_ops.set_mode) {
    r = info.phy_ops.set_mode(info.phy_id, NXP_NCI_MODE_COLD);
    if (r < 0 && result == 0)
    result = -r;
    }
    info.mode = NXP_NCI_MODE_COLD;
    if (fw_info.fw) {
    release_firmware(fw_info.fw);
    fw_info.fw = core::ptr::null_mut();
    }
    nfc_fw_download_done(info.ndev.nfc_dev, fw_info.name, (u32) -result);
    }
// crc_ccitt cannot be used since it is computed MSB first and not LSB first
#[no_mangle]
unsafe extern "C" fn nxp_nci_fw_crc(buffer: *const u8, len: usize) -> u16 {
    static u16 nxp_nci_fw_crc(u8 const *buffer, size_t len)
    {
    let mut crc: u16 = 0xffff;
    while (len--) {
    crc = ((crc >> 8) | (crc << 8)) ^ *buffer++;
    crc ^= (crc & 0xff) >> 4;
    crc ^= (crc & 0xff) << 12;
    crc ^= (crc & 0xff) << 5;
    }
    return crc;
    }
#[no_mangle]
unsafe extern "C" fn nxp_nci_fw_send_chunk(info: *mut nxp_nci_info) -> c_int {
    static int nxp_nci_fw_send_chunk(struct nxp_nci_info *info)
    {
    struct nxp_nci_fw_info *fw_info = &info.fw_info;
    u16 header, crc;
    struct sk_buff *skb;
    size_t chunk_len;
    size_t remaining_len;
    int r;
    skb = nci_skb_alloc(info.ndev, info.max_payload, GFP_KERNEL);
    if (!skb)
    return -ENOMEM;
    chunk_len = info.max_payload - NXP_NCI_FW_HDR_LEN - NXP_NCI_FW_CRC_LEN;
    remaining_len = fw_info.frame_size - fw_info.written;
    if (remaining_len > chunk_len) {
    header = NXP_NCI_FW_CHUNK_FLAG;
    } else {
    chunk_len = remaining_len;
    header = 0x0000;
    }
    header |= chunk_len & NXP_NCI_FW_FRAME_LEN_MASK;
    put_unaligned_be16(header, skb_put(skb, NXP_NCI_FW_HDR_LEN));
    skb_put_data(skb, fw_info.data + fw_info.written, chunk_len);
    crc = nxp_nci_fw_crc(skb.data, chunk_len + NXP_NCI_FW_HDR_LEN);
    put_unaligned_be16(crc, skb_put(skb, NXP_NCI_FW_CRC_LEN));
    r = info.phy_ops.write(info.phy_id, skb);
    if (r >= 0)
    r = chunk_len;
    kfree_skb(skb);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn nxp_nci_fw_send(info: *mut nxp_nci_info) -> c_int {
    static int nxp_nci_fw_send(struct nxp_nci_info *info)
    {
    struct nxp_nci_fw_info *fw_info = &info.fw_info;
    long completion_rc;
    int r;
    reinit_completion(&fw_info.cmd_completion);
    if (fw_info.written == 0) {
    fw_info.frame_size = get_unaligned_be16(fw_info.data) &
    NXP_NCI_FW_FRAME_LEN_MASK;
    fw_info.data += NXP_NCI_FW_HDR_LEN;
    fw_info.size -= NXP_NCI_FW_HDR_LEN;
    }
    if (fw_info.frame_size > fw_info.size)
    return -EMSGSIZE;
    r = nxp_nci_fw_send_chunk(info);
    if (r < 0)
    return r;
    fw_info.written += r;
    if (*fw_info.data == NXP_NCI_FW_CMD_RESET) {
    fw_info.cmd_result = 0;
    if (fw_info.fw)
    schedule_work(&fw_info.work);
    } else {
    completion_rc = wait_for_completion_interruptible_timeout(
    &fw_info.cmd_completion, NXP_NCI_FW_ANSWER_TIMEOUT);
    if (completion_rc == 0)
    return -ETIMEDOUT;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nxp_nci_fw_work(work: *mut work_struct) {
    void nxp_nci_fw_work(struct work_struct *work)
    {
    struct nxp_nci_info *info;
    struct nxp_nci_fw_info *fw_info;
    int r;
    fw_info = container_of(work, struct nxp_nci_fw_info, work);
    info = container_of(fw_info, struct nxp_nci_info, fw_info);
    mutex_lock(&info.info_lock);
    r = fw_info.cmd_result;
    if (r < 0)
    goto exit_work;
    if (fw_info.written == fw_info.frame_size) {
    fw_info.data += fw_info.frame_size;
    fw_info.size -= fw_info.frame_size;
    fw_info.written = 0;
    }
    if (fw_info.size > 0)
    r = nxp_nci_fw_send(info);
    exit_work:
    if (r < 0 || fw_info.size == 0)
    nxp_nci_fw_work_complete(info, r);
    mutex_unlock(&info.info_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn nxp_nci_fw_download(ndev: *mut nci_dev, firmware_name: *const c_char) -> c_int {
    int nxp_nci_fw_download(struct nci_dev *ndev, const char *firmware_name)
    {
    struct nxp_nci_info *info = nci_get_drvdata(ndev);
    struct nxp_nci_fw_info *fw_info = &info.fw_info;
    int r;
    mutex_lock(&info.info_lock);
    if (!info.phy_ops.set_mode || !info.phy_ops.write) {
    r = -ENOTSUPP;
    goto fw_download_exit;
    }
    if (!firmware_name || firmware_name[0] == '\0') {
    r = -EINVAL;
    goto fw_download_exit;
    }
    strscpy(fw_info.name, firmware_name);
    r = request_firmware(&fw_info.fw, firmware_name,
    ndev.nfc_dev.dev.parent);
    if (r < 0)
    goto fw_download_exit;
    r = info.phy_ops.set_mode(info.phy_id, NXP_NCI_MODE_FW);
    if (r < 0) {
    release_firmware(fw_info.fw);
    goto fw_download_exit;
    }
    info.mode = NXP_NCI_MODE_FW;
    fw_info.data = fw_info.fw.data;
    fw_info.size = fw_info.fw.size;
    fw_info.written = 0;
    fw_info.frame_size = 0;
    fw_info.cmd_result = 0;
    schedule_work(&fw_info.work);
    fw_download_exit:
    mutex_unlock(&info.info_lock);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn nxp_nci_fw_read_status(stat: u8) -> c_int {
    static int nxp_nci_fw_read_status(u8 stat)
    {
    switch (stat) {
    case NXP_NCI_FW_RESULT_OK:
    return 0;
    case NXP_NCI_FW_RESULT_INVALID_ADDR:
    return -EINVAL;
    case NXP_NCI_FW_RESULT_UNKNOWN_CMD:
    return -EINVAL;
    case NXP_NCI_FW_RESULT_ABORTED_CMD:
    return -EMSGSIZE;
    case NXP_NCI_FW_RESULT_ADDR_RANGE_OFL_ERROR:
    return -EADDRNOTAVAIL;
    case NXP_NCI_FW_RESULT_BUFFER_OFL_ERROR:
    return -ENOBUFS;
    case NXP_NCI_FW_RESULT_MEM_BSY:
    return -ENOKEY;
    case NXP_NCI_FW_RESULT_SIGNATURE_ERROR:
    return -EKEYREJECTED;
    case NXP_NCI_FW_RESULT_FIRMWARE_VERSION_ERROR:
    return -EALREADY;
    case NXP_NCI_FW_RESULT_PROTOCOL_ERROR:
    return -EPROTO;
    case NXP_NCI_FW_RESULT_SFWU_DEGRADED:
    return -EHWPOISON;
    case NXP_NCI_FW_RESULT_PH_STATUS_FIRST_CHUNK:
    return 0;
    case NXP_NCI_FW_RESULT_PH_STATUS_NEXT_CHUNK:
    return 0;
    case NXP_NCI_FW_RESULT_PH_STATUS_INTERNAL_ERROR_5:
    return -EINVAL;
    default:
    return -EIO;
    }
    }
#[no_mangle]
unsafe extern "C" fn nxp_nci_fw_check_crc(skb: *mut sk_buff) -> u16 {
    static u16 nxp_nci_fw_check_crc(struct sk_buff *skb)
    {
    u16 crc, frame_crc;
    let mut len: usize = skb.len - NXP_NCI_FW_CRC_LEN;
    crc = nxp_nci_fw_crc(skb.data, len);
    frame_crc = get_unaligned_be16(skb.data + len);
    return (crc ^ frame_crc);
    }
#[no_mangle]
pub unsafe extern "C" fn nxp_nci_fw_recv_frame(ndev: *mut nci_dev, skb: *mut sk_buff) {
    void nxp_nci_fw_recv_frame(struct nci_dev *ndev, struct sk_buff *skb)
    {
    struct nxp_nci_info *info = nci_get_drvdata(ndev);
    struct nxp_nci_fw_info *fw_info = &info.fw_info;
    complete(&fw_info.cmd_completion);
    if (skb) {
    if (nxp_nci_fw_check_crc(skb) != 0x00)
    fw_info.cmd_result = -EBADMSG;
    else
    fw_info.cmd_result = nxp_nci_fw_read_status(*(u8 *)skb_pull(skb, NXP_NCI_FW_HDR_LEN));
    kfree_skb(skb);
    } else {
    fw_info.cmd_result = -EIO;
    }
    if (fw_info.fw)
    schedule_work(&fw_info.work);
    }
    EXPORT_SYMBOL(nxp_nci_fw_recv_frame);
