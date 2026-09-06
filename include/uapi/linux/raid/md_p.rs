//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/raid/md_p.h
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

//
// RAID superblock.
//
// The RAID superblock maintains some statistics on each RAID configuration.
// Each real device in the RAID set contains it near the end of the device.
// Some of the ideas are copied from the ext2fs implementation.
//
// We currently use 4096 bytes as follows:
//
// word offset	function
//
// 0  -    31	Constant generic RAID device information.
// 32  -    63   Generic state information.
// 64  -   127	Personality specific information.
// 128  -   511	12 32-words descriptors of the disks in the raid set.
// 512  -   911	Reserved.
// 912  -  1023	Disk specific descriptor.
//
// If x is the real device size in bytes, we return an apparent size of:
//
// y = (x & ~(MD_RESERVED_BYTES - 1)) - MD_RESERVED_BYTES
//
// and place the 4kB superblock at offset y.
//

pub const MD_SB_BYTES: c_int = 4096;

//
// The following are counted in 32-bit words
//
pub const MD_SB_GENERIC_OFFSET: c_int = 0;
pub const MD_SB_PERSONALITY_OFFSET: c_int = 64;
pub const MD_SB_DISKS_OFFSET: c_int = 128;
pub const MD_SB_DESCRIPTOR_OFFSET: c_int = 992;
pub const MD_SB_GENERIC_CONSTANT_WORDS: c_int = 32;
pub const MD_SB_GENERIC_STATE_WORDS: c_int = 32;

pub const MD_SB_PERSONALITY_WORDS: c_int = 64;
pub const MD_SB_DESCRIPTOR_WORDS: c_int = 32;
pub const MD_SB_DISKS: c_int = 27;

//
// Device "operational" state bits
//

// For clustered enviroments only.
//

// For clustered enviroments only.
//

// devices available - and don't try to
// correct read errors.
//

// read requests will only be sent here in
// dire need
//

pub const MD_DISK_ROLE_SPARE: c_uint = 0xffff;
pub const MD_DISK_ROLE_FAULTY: c_uint = 0xfffe;
pub const MD_DISK_ROLE_JOURNAL: c_uint = 0xfffd;
pub const MD_DISK_ROLE_MAX: c_uint = 0xff00 /* max value of regular disk role */;
pub const MD_SB_MAGIC: c_uint = 0xa92b4efc;
//
// Superblock state bits
//
pub const MD_SB_CLEAN: c_int = 0;
pub const MD_SB_ERRORS: c_int = 1;

//
// Notes:
// - if an array is being reshaped (restriped) in order to change
// the number of active devices in the array, 'raid_disks' will be
// the larger of the old and new numbers.  'delta_disks' will
// be the "new - old".  So if +ve, raid_disks is the new value, and
// "raid_disks-delta_disks" is the old.  If -ve, raid_disks is the
// old value and "raid_disks+delta_disks" is the new (smaller) value.
//
// Constant generic information
//
// Generic state information
//

// There are only valid for minor_version > 90
//
// Personality information
//
// Disks information
//
// Reserved
//
// Active descriptor
//

//
// The version-1 superblock :
// All numeric fields are little-endian.
//
// total size: 256 bytes plus 2 per device.
// 1K allows 384 devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_superblock_1 {
// constant array information - 128 bytes
    pub /: *mut *mut __le32 magic; / MD_SB_MAGIC: 0xa92b4efc - little endian,
    pub /: *mut *mut __le32 major_version; / 1,
    pub /: *mut *mut __le32 feature_map; / bit 0 set if 'bitmap_offset' is meaningful,
    pub /: *mut *mut __le32 pad0; / always set to 0 when writing,
    pub /: *mut *mut __u8 set_uuid[16]; / user-space generated.,
    pub /: *mut *mut char set_name[32]; / set and interpreted by user-space,
    pub 0*/: *mut *mut __le64 ctime; / lo 40 bits are seconds, top 24 are microseconds or,
    pub /: *mut *mut __le32 level; / 0,1,4,5, -1 (linear),
    pub /: *mut *mut __le32 layout; / only for raid5 and raid10 currently,
    pub /: *mut *mut __le64 size; / used size of component devices, in 512byte sectors,
    pub /: *mut *mut __le32 chunksize; / in 512byte sectors,
    pub raid_disks: __le32,
    pub starts: *mut *mut __le32 bitmap_offset; / sectors after start of superblock that bitmap,
// NOTE: signed, so bitmap can be before superblock
// only meaningful of feature_map[0] is set.
//
// only meaningful when feature_map[MD_FEATURE_PPL] is set
    pub /: *mut *mut __le16 offset; / sectors from start of superblock that ppl starts (signed),
    pub /: *mut *mut __le16 size; / ppl size in sectors,
    pub ppl: },
}

// These are only valid with feature bit '4'
// layout.  0 == no-change.  This can be
// different on each device in the array.
//
// constant this-device information - 64 bytes

// Bad block log.  If there are any bad blocks the feature flag is set.
// If offset and size are non-zero, that space is reserved and available
//
// signed - not unsigned
// array state information - 64 bytes
// device state information. Indexed by dev_number.
// 2 bytes per device
// Note there are no per-device state flags. State information is rolled
// into the 'roles' value.  If a device is spare or faulty, then it doesn't
// have a meaningful role.
//
// feature_map bits
pub const MD_FEATURE_BITMAP_OFFSET: c_int = 1;

// must be honoured
//
pub const MD_FEATURE_RESHAPE_ACTIVE: c_int = 4;

// active device with same 'role'.
// 'recovery_offset' is also set.
//

// of devices, but is going
// backwards anyway.
//

// is guided by bitmap.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r5l_payload_header {
    pub type: __le16,
    pub flags: __le16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum r5l_payload_type {
    R5LOG_PAYLOAD_DATA = 0,
    R5LOG_PAYLOAD_PARITY = 1,
    R5LOG_PAYLOAD_FLUSH = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r5l_payload_data_parity {
    pub header: r5l_payload_header,
    pub 4k: *mut *mut __le32 size; / sector. data/parity size. each,
// has a checksum
    pub For: *mut *mut __le64 location; / sector. For data, it's raid sector.,
// parity, it's stripe sector
    pub checksum: [__le32; ],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum r5l_payload_data_parity_flag {
    R5LOG_PAYLOAD_FLAG_DISCARD = 1, /* payload is discard */
//
// RESHAPED/RESHAPING is only set when there is reshape activity. Note,
// both data/parity of a stripe should have the same flag set
//
// RESHAPED: reshape is running, and this stripe finished reshape
// RESHAPING: reshape is running, and this stripe isn't reshaped
//
    R5LOG_PAYLOAD_FLAG_RESHAPED = 2,
    R5LOG_PAYLOAD_FLAG_RESHAPING = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r5l_payload_flush {
    pub header: r5l_payload_header,
    pub /: *mut *mut __le32 size; / flush_stripes size, bytes,
    pub flush_stripes: [__le64; ],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum r5l_payload_flush_flag {
    R5LOG_PAYLOAD_FLAG_FLUSH_STRIPE = 1, /* data represents whole stripe */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r5l_meta_block {
    pub magic: __le32,
    pub checksum: __le32,
    pub version: __u8,
    pub __zero_pading_1: __u8,
    pub __zero_pading_2: __le16,
    pub /: *mut *mut __le32 meta_size; / whole size of the block,
    pub seq: __le64,
    pub /: *mut *mut __le64 position; / sector, start from rdev->data_offset, current position,
    pub payloads: [r5l_payload_header; ],
// C attribute field omitted
pub const R5LOG_VERSION: c_uint = 0x1;
pub const R5LOG_MAGIC: c_uint = 0x6433c509;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppl_header_entry {
    pub /: *mut *mut __le64 data_sector; / raid sector of the new data,
    pub /: *mut *mut __le32 pp_size; / length of partial parity,
    pub /: *mut *mut __le32 data_size; / length of data,
    pub /: *mut *mut __le32 parity_disk; / member disk containing parity,
    pub this: *mut *mut __le32 checksum; / checksum of partial parity data for,
// entry (~crc32c)
// C attribute field omitted
pub const PPL_HEADER_SIZE: c_int = 4096;
pub const PPL_HDR_RESERVED: c_int = 512;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppl_header {
    pub /: *mut *mut __u8 reserved[PPL_HDR_RESERVED];/ reserved space, fill with 0xff,
    pub /: *mut *mut __le32 signature; / signature (family number of volume),
    pub /: *mut *mut __le32 padding; / zero pad,
    pub /: *mut *mut __le64 generation; / generation number of the header,
    pub /: *mut *mut __le32 entries_count; / number of entries in entry array,
    pub /: *mut *mut __le32 checksum; / checksum of the header (~crc32c),
    pub entries: [ppl_header_entry; PPL_HDR_MAX_ENTRIES],
// C attribute field omitted
