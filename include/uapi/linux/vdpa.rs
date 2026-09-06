//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/vdpa.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// vdpa device management interface
// Copyright (c) 2020 Mellanox Technologies Ltd. All rights reserved.
//

pub const VDPA_GENL_VERSION: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vdpa_command {
    VDPA_CMD_UNSPEC,
    VDPA_CMD_MGMTDEV_NEW,
    VDPA_CMD_MGMTDEV_GET,		/* can dump */
    VDPA_CMD_DEV_NEW,
    VDPA_CMD_DEV_DEL,
    VDPA_CMD_DEV_GET,		/* can dump */
    VDPA_CMD_DEV_CONFIG_GET,	/* can dump */
    VDPA_CMD_DEV_VSTATS_GET,
    VDPA_CMD_DEV_ATTR_SET,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vdpa_attr {
    VDPA_ATTR_UNSPEC,

// Pad attribute for 64b alignment
    VDPA_ATTR_PAD = VDPA_ATTR_UNSPEC,

// bus name (optional) + dev name together make the parent device handle
    VDPA_ATTR_MGMTDEV_BUS_NAME,		/* string */
    VDPA_ATTR_MGMTDEV_DEV_NAME,		/* string */
    VDPA_ATTR_MGMTDEV_SUPPORTED_CLASSES,	/* u64 */

    VDPA_ATTR_DEV_NAME,			/* string */
    VDPA_ATTR_DEV_ID,			/* u32 */
    VDPA_ATTR_DEV_VENDOR_ID,		/* u32 */
    VDPA_ATTR_DEV_MAX_VQS,			/* u32 */
    VDPA_ATTR_DEV_MAX_VQ_SIZE,		/* u16 */
    VDPA_ATTR_DEV_MIN_VQ_SIZE,		/* u16 */

    VDPA_ATTR_DEV_NET_CFG_MACADDR,		/* binary */
    VDPA_ATTR_DEV_NET_STATUS,		/* u8 */
    VDPA_ATTR_DEV_NET_CFG_MAX_VQP,		/* u16 */
    VDPA_ATTR_DEV_NET_CFG_MTU,		/* u16 */

    VDPA_ATTR_DEV_NEGOTIATED_FEATURES,	/* u64 */
    VDPA_ATTR_DEV_MGMTDEV_MAX_VQS,		/* u32 */
// virtio features that are supported by the vDPA management device
    VDPA_ATTR_DEV_SUPPORTED_FEATURES,	/* u64 */

    VDPA_ATTR_DEV_QUEUE_INDEX,              /* u32 */
    VDPA_ATTR_DEV_VENDOR_ATTR_NAME,		/* string */
    VDPA_ATTR_DEV_VENDOR_ATTR_VALUE,        /* u64 */

// virtio features that are provisioned to the vDPA device
    VDPA_ATTR_DEV_FEATURES,                 /* u64 */

    VDPA_ATTR_DEV_BLK_CFG_CAPACITY,		/* u64 */
    VDPA_ATTR_DEV_BLK_CFG_SIZE_MAX,		/* u32 */
    VDPA_ATTR_DEV_BLK_CFG_BLK_SIZE,		/* u32 */
    VDPA_ATTR_DEV_BLK_CFG_SEG_MAX,		/* u32 */
    VDPA_ATTR_DEV_BLK_CFG_NUM_QUEUES,	/* u16 */
    VDPA_ATTR_DEV_BLK_CFG_PHY_BLK_EXP,	/* u8 */
    VDPA_ATTR_DEV_BLK_CFG_ALIGN_OFFSET,	/* u8 */
    VDPA_ATTR_DEV_BLK_CFG_MIN_IO_SIZE,	/* u16 */
    VDPA_ATTR_DEV_BLK_CFG_OPT_IO_SIZE,	/* u32 */
    VDPA_ATTR_DEV_BLK_CFG_MAX_DISCARD_SEC,	/* u32 */
    VDPA_ATTR_DEV_BLK_CFG_MAX_DISCARD_SEG,	/* u32 */
    VDPA_ATTR_DEV_BLK_CFG_DISCARD_SEC_ALIGN,/* u32 */
    VDPA_ATTR_DEV_BLK_CFG_MAX_WRITE_ZEROES_SEC,	/* u32 */
    VDPA_ATTR_DEV_BLK_CFG_MAX_WRITE_ZEROES_SEG,	/* u32 */
    VDPA_ATTR_DEV_BLK_READ_ONLY,		/* u8 */
    VDPA_ATTR_DEV_BLK_FLUSH,		/* u8 */

// new attributes must be added above here
    VDPA_ATTR_MAX,
}
