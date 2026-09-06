//! Automatically rewritten from C to Rust
//! Source: drivers/nfc/microread/mei.c
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
// Copyright (C) 2013 Intel Corporation. All rights reserved.
//
// HCI based Driver for Inside Secure microread NFC Chip
//

    static int microread_mei_probe(struct mei_cl_device *cldev,
    const struct mei_cl_device_id *id)
    {
    struct nfc_mei_phy *phy;
    int r;
    phy = nfc_mei_phy_alloc(cldev);
    if (!phy)
    return -ENOMEM;
    r = microread_probe(phy, &mei_phy_ops, LLC_NOP_NAME,
    MEI_NFC_HEADER_SIZE, 0, MEI_NFC_MAX_HCI_PAYLOAD,
    &phy.hdev);
    if (r < 0) {
    nfc_mei_phy_free(phy);
    return r;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn microread_mei_remove(cldev: *mut mei_cl_device) {
    static void microread_mei_remove(struct mei_cl_device *cldev)
    {
    struct nfc_mei_phy *phy = mei_cldev_get_drvdata(cldev);
    microread_remove(phy.hdev);
    nfc_mei_phy_free(phy);
    }
    static struct mei_cl_device_id microread_mei_tbl[] = {
    {
    .name = MICROREAD_DRIVER_NAME,
    .uuid = MEI_NFC_UUID,
    .version = MEI_CL_VERSION_ANY,
    },
    { /* required last entry */ }
    };
    MODULE_DEVICE_TABLE(mei, microread_mei_tbl);
    static struct mei_cl_driver microread_driver = {
    .id_table = microread_mei_tbl,
    .name = MICROREAD_DRIVER_NAME,
    .probe = microread_mei_probe,
    .remove = microread_mei_remove,
    };
    module_mei_cl_driver(microread_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION(DRIVER_DESC);
