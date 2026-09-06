//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/sd_dif.c
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
// sd_dif.c - SCSI Data Integrity Field
//
// Copyright (C) 2007, 2008 Oracle Corporation
// Written by: Martin K. Petersen <martin.petersen@oracle.com>
//

//
// Configure exchange of protection information between OS and HBA.
//
#[no_mangle]
pub unsafe extern "C" fn sd_dif_config_host(sdkp: *mut scsi_disk, lim: *mut queue_limits) {
    void sd_dif_config_host(struct scsi_disk *sdkp, struct queue_limits *lim)
    {
    struct scsi_device *sdp = sdkp.device;
    let mut type: u8 = sdkp.protection_type;
    struct blk_integrity *bi = &lim.integrity;
    int dif, dix;
    memset(bi, 0, sizeof(*bi));
    dif = scsi_host_dif_capable(sdp.host, type);
    dix = scsi_host_dix_capable(sdp.host, type);
    if (!dix && scsi_host_dix_capable(sdp.host, 0)) {
    dif = 0; dix = 1;
    }
    if (!dix)
    return;
// Enable DMA of protection information
    if (scsi_host_get_guard(sdkp.device.host) & SHOST_DIX_GUARD_IP)
    bi.csum_type = BLK_INTEGRITY_CSUM_IP;
    else
    bi.csum_type = BLK_INTEGRITY_CSUM_CRC;
    if (type != T10_PI_TYPE3_PROTECTION)
    bi.flags |= BLK_INTEGRITY_REF_TAG;
    bi.metadata_size = sizeof(struct t10_pi_tuple);
    bi.pi_tuple_size = bi.metadata_size;
    if (dif && type) {
    bi.flags |= BLK_INTEGRITY_DEVICE_CAPABLE;
    if (!sdkp.ATO)
    return;
    if (type == T10_PI_TYPE3_PROTECTION)
    bi.tag_size = sizeof(u16) + sizeof(u32);
    else
    bi.tag_size = sizeof(u16);
    }
    sd_first_printk(KERN_NOTICE, sdkp,
    "Enabling DIX %s, application tag size %u bytes\n",
    blk_integrity_profile_name(bi), bi.tag_size);
    }
