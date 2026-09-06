//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/qemu_fw_cfg.h
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


// SPDX-License-Identifier: BSD-3-Clause

// selector key values for "well-known" fw_cfg entries
pub const FW_CFG_SIGNATURE: c_uint = 0x00;
pub const FW_CFG_ID: c_uint = 0x01;
pub const FW_CFG_UUID: c_uint = 0x02;
pub const FW_CFG_RAM_SIZE: c_uint = 0x03;
pub const FW_CFG_NOGRAPHIC: c_uint = 0x04;
pub const FW_CFG_NB_CPUS: c_uint = 0x05;
pub const FW_CFG_MACHINE_ID: c_uint = 0x06;
pub const FW_CFG_KERNEL_ADDR: c_uint = 0x07;
pub const FW_CFG_KERNEL_SIZE: c_uint = 0x08;
pub const FW_CFG_KERNEL_CMDLINE: c_uint = 0x09;
pub const FW_CFG_INITRD_ADDR: c_uint = 0x0a;
pub const FW_CFG_INITRD_SIZE: c_uint = 0x0b;
pub const FW_CFG_BOOT_DEVICE: c_uint = 0x0c;
pub const FW_CFG_NUMA: c_uint = 0x0d;
pub const FW_CFG_BOOT_MENU: c_uint = 0x0e;
pub const FW_CFG_MAX_CPUS: c_uint = 0x0f;
pub const FW_CFG_KERNEL_ENTRY: c_uint = 0x10;
pub const FW_CFG_KERNEL_DATA: c_uint = 0x11;
pub const FW_CFG_INITRD_DATA: c_uint = 0x12;
pub const FW_CFG_CMDLINE_ADDR: c_uint = 0x13;
pub const FW_CFG_CMDLINE_SIZE: c_uint = 0x14;
pub const FW_CFG_CMDLINE_DATA: c_uint = 0x15;
pub const FW_CFG_SETUP_ADDR: c_uint = 0x16;
pub const FW_CFG_SETUP_SIZE: c_uint = 0x17;
pub const FW_CFG_SETUP_DATA: c_uint = 0x18;
pub const FW_CFG_FILE_DIR: c_uint = 0x19;
pub const FW_CFG_FILE_FIRST: c_uint = 0x20;
pub const FW_CFG_FILE_SLOTS_MIN: c_uint = 0x10;
pub const FW_CFG_WRITE_CHANNEL: c_uint = 0x4000;
pub const FW_CFG_ARCH_LOCAL: c_uint = 0x8000;

pub const FW_CFG_INVALID: c_uint = 0xffff;
// width in bytes of fw_cfg control register
pub const FW_CFG_CTL_SIZE: c_uint = 0x02;
// fw_cfg "file name" is up to 56 characters (including terminating nul)
pub const FW_CFG_MAX_FILE_PATH: c_int = 56;
// size in bytes of fw_cfg signature
pub const FW_CFG_SIG_SIZE: c_int = 4;
// FW_CFG_ID bits
pub const FW_CFG_VERSION: c_uint = 0x01;
pub const FW_CFG_VERSION_DMA: c_uint = 0x02;
// fw_cfg file directory entry type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_cfg_file {
    pub size: __be32,
    pub select: __be16,
    pub reserved: __u16,
    pub name: [c_char; FW_CFG_MAX_FILE_PATH],
}

// FW_CFG_DMA_CONTROL bits
pub const FW_CFG_DMA_CTL_ERROR: c_uint = 0x01;
pub const FW_CFG_DMA_CTL_READ: c_uint = 0x02;
pub const FW_CFG_DMA_CTL_SKIP: c_uint = 0x04;
pub const FW_CFG_DMA_CTL_SELECT: c_uint = 0x08;
pub const FW_CFG_DMA_CTL_WRITE: c_uint = 0x10;
pub const FW_CFG_DMA_SIGNATURE: c_uint = 0x51454d5520434647ULL /* "QEMU CFG" */;
// Control as first field allows for different structures selected by this
// field, which might be useful in the future
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_cfg_dma_access {
    pub control: __be32,
    pub length: __be32,
    pub address: __be64,
}

pub const FW_CFG_VMCOREINFO_FORMAT_NONE: c_uint = 0x0;
pub const FW_CFG_VMCOREINFO_FORMAT_ELF: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_cfg_vmcoreinfo {
    pub host_format: __le16,
    pub guest_format: __le16,
    pub size: __le32,
    pub paddr: __le64,
}
