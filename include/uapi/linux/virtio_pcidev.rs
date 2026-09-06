//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_pcidev.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
//
// Copyright (C) 2021 Intel Corporation
// Author: Johannes Berg <johannes@sipsolutions.net>
//

//
// enum virtio_pcidev_ops - virtual PCI device operations
// @VIRTIO_PCIDEV_OP_RESERVED: reserved to catch errors
// @VIRTIO_PCIDEV_OP_CFG_READ: read config space, size is 1, 2, 4 or 8;
// the @data field should be filled in by the device (in little endian).
// @VIRTIO_PCIDEV_OP_CFG_WRITE: write config space, size is 1, 2, 4 or 8;
// the @data field contains the data to write (in little endian).
// @VIRTIO_PCIDEV_OP_MMIO_READ: read BAR mem/pio, size can be variable;
// the @data field should be filled in by the device (in little endian).
// @VIRTIO_PCIDEV_OP_MMIO_WRITE: write BAR mem/pio, size can be variable;
// the @data field contains the data to write (in little endian).
// @VIRTIO_PCIDEV_OP_MMIO_MEMSET: memset MMIO, size is variable but
// the @data field only has one byte (unlike @VIRTIO_PCIDEV_OP_MMIO_WRITE)
// @VIRTIO_PCIDEV_OP_INT: legacy INTx# pin interrupt, the addr field is 1-4 for
// the number
// @VIRTIO_PCIDEV_OP_MSI: MSI(-X) interrupt, this message basically transports
// the 16- or 32-bit write that would otherwise be done into memory,
// analogous to the write messages (@VIRTIO_PCIDEV_OP_MMIO_WRITE) above
// @VIRTIO_PCIDEV_OP_PME: Dummy message whose content is ignored (and should be
// all zeroes) to signal the PME# pin.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtio_pcidev_ops {
    VIRTIO_PCIDEV_OP_RESERVED = 0,
    VIRTIO_PCIDEV_OP_CFG_READ,
    VIRTIO_PCIDEV_OP_CFG_WRITE,
    VIRTIO_PCIDEV_OP_MMIO_READ,
    VIRTIO_PCIDEV_OP_MMIO_WRITE,
    VIRTIO_PCIDEV_OP_MMIO_MEMSET,
    VIRTIO_PCIDEV_OP_INT,
    VIRTIO_PCIDEV_OP_MSI,
    VIRTIO_PCIDEV_OP_PME,
}

//
// struct virtio_pcidev_msg - virtio PCI device operation
// @op: the operation to do
// @bar: the bar (only with BAR read/write messages)
// @reserved: reserved
// @size: the size of the read/write (in bytes)
// @addr: the address to read/write
// @data: the data, normally @size long, but just one byte for
// %VIRTIO_PCIDEV_OP_MMIO_MEMSET
//
// Note: the fields are all in native (CPU) endian, however, the
// @data values will often be in little endian (see the ops above.)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pcidev_msg {
    pub op: __u8,
    pub bar: __u8,
    pub reserved: __u16,
    pub size: __u32,
    pub addr: __u64,
    pub data: [__u8; ],
}
