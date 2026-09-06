//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bcm963xx_tag.h
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

pub const NUM_PIRELLI: c_int = 2;
pub const IMAGETAG_CRC_START: c_uint = 0xFFFFFFFF;

// Extended flash address, needs to be subtracted
// from bcm_tag flash image offsets.
//
pub const BCM963XX_EXTENDED_SIZE: c_uint = 0xBFC00000;
//
// The broadcom firmware assumes the rootfs starts the image,
// therefore uses the rootfs start (flash_image_address)
// to determine where to flash the image.  Since we have the kernel first
// we have to give it the kernel address, but the crc uses the length
// associated with this address (root_length), which is added to the kernel
// length (kernel_length) to determine the length of image to flash and thus
// needs to be rootfs + deadcode (jffs2 EOF marker)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_tag {
// 0-3: Version of the image tag
    pub tag_version: [c_char; TAGVER_LEN],
// 4-23: Company Line 1
    pub sig_1: [c_char; SIG1_LEN],
// 24-37: Company Line 2
    pub sig_2: [c_char; SIG2_LEN],
// 38-43: Chip this image is for
    pub chip_id: [c_char; CHIPID_LEN],
// 44-59: Board name
    pub board_id: [c_char; BOARDID_LEN],
// 60-61: Map endianness -- 1 BE 0 LE
    pub big_endian: [c_char; ENDIANFLAG_LEN],
// 62-71: Total length of image
    pub total_length: [c_char; IMAGE_LEN],
// 72-83: Address in memory of CFE
    pub cfe__address: [c_char; ADDRESS_LEN],
// 84-93: Size of CFE
    pub cfe_length: [c_char; IMAGE_LEN],
// 94-105: Address in memory of image start
// (kernel for OpenWRT, rootfs for stock firmware)
//
    pub flash_image_start: [c_char; ADDRESS_LEN],
// 106-115: Size of rootfs
    pub root_length: [c_char; IMAGE_LEN],
// 116-127: Address in memory of kernel
    pub kernel_address: [c_char; ADDRESS_LEN],
// 128-137: Size of kernel
    pub kernel_length: [c_char; IMAGE_LEN],
// 138-141: Image sequence number
// (to be incremented when flashed with a new image)
//
    pub image_sequence: [c_char; IMAGE_SEQUENCE_LEN],
// 142-161: RSA Signature (not used; some vendors may use this)
    pub rsa_signature: [c_char; RSASIG_LEN],
// 162-191: Compilation and related information (not used in OpenWrt)
    pub information1: [c_char; TAGINFO1_LEN],
// 192-195: Version flash layout
    pub flash_layout_ver: [c_char; FLASHLAYOUTVER_LEN],
// 196-199: kernel+rootfs CRC32
    pub fskernel_crc: __u32,
// 200-215: Unused except on Alice Gate where it is information
    pub information2: [c_char; TAGINFO2_LEN],
// 216-219: CRC32 of image less imagetag (kernel for Alice Gate)
    pub image_crc: __u32,
// 220-223: CRC32 of rootfs partition
    pub rootfs_crc: __u32,
// 224-227: CRC32 of kernel partition
    pub kernel_crc: __u32,
// 228-235: Unused at present
    pub reserved1: [c_char; 8],
// 236-239: CRC32 of header excluding last 20 bytes
    pub header_crc: __u32,
// 240-255: Unused at present
    pub reserved2: [c_char; 16],
}
