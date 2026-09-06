//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/ubi/ubi-media.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// Copyright (C) International Business Machines Corp., 2006
// Authors: Artem Bityutskiy (Битюцкий Артём)
// Thomas Gleixner
// Frank Haverkamp
// Oliver Lohmann
// Andreas Arnez
//
// This file defines the layout of UBI headers and all the other UBI on-flash
// data structures.
//

// The version of UBI images supported by this implementation
pub const UBI_VERSION: c_int = 1;
// The highest erase counter value supported by this implementation
pub const UBI_MAX_ERASECOUNTER: c_uint = 0x7FFFFFFF;
// The initial CRC32 value used when calculating CRC checksums
pub const UBI_CRC32_INIT: c_uint = 0xFFFFFFFFU;
// Erase counter header magic number (ASCII "UBI#")
pub const UBI_EC_HDR_MAGIC: c_uint = 0x55424923;
// Volume identifier header magic number (ASCII "UBI!")
pub const UBI_VID_HDR_MAGIC: c_uint = 0x55424921;
//
// Volume type constants used in the volume identifier header.
//
// @UBI_VID_DYNAMIC: dynamic volume
// @UBI_VID_STATIC: static volume
//
// Volume flags used in the volume table record.
//
// @UBI_VTBL_AUTORESIZE_FLG: auto-resize this volume
// @UBI_VTBL_SKIP_CRC_CHECK_FLG: skip the CRC check done on a static volume at
// open time. Should only be set on volumes that
// are used by upper layers doing this kind of
// check. Main use-case for this flag is
// boot-time reduction
//
// %UBI_VTBL_AUTORESIZE_FLG flag can be set only for one volume in the volume
// table. UBI automatically re-sizes the volume which has this flag and makes
// the volume to be of largest possible size. This means that if after the
// initialization UBI finds out that there are available physical eraseblocks
// present on the device, it automatically appends all of them to the volume
// (the physical eraseblocks reserved for bad eraseblocks handling and other
// reserved physical eraseblocks are not taken). So, if there is a volume with
// the %UBI_VTBL_AUTORESIZE_FLG flag set, the amount of available logical
// eraseblocks will be zero after UBI is loaded, because all of them will be
// reserved for this volume. Note, the %UBI_VTBL_AUTORESIZE_FLG bit is cleared
// after the volume had been initialized.
//
// The auto-resize feature is useful for device production purposes. For
// example, different NAND flash chips may have different amount of initial bad
// eraseblocks, depending of particular chip instance. Manufacturers of NAND
// chips usually guarantee that the amount of initial bad eraseblocks does not
// exceed certain percent, e.g. 2%. When one creates an UBI image which will be
// flashed to the end devices in production, he does not know the exact amount
// of good physical eraseblocks the NAND chip on the device will have, but this
// number is required to calculate the volume sized and put them to the volume
// table of the UBI image. In this case, one of the volumes (e.g., the one
// which will store the root file system) is marked as "auto-resizable", and
// UBI will adjust its size on the first boot if needed.
//
// Note, first UBI reserves some amount of physical eraseblocks for bad
// eraseblock handling, and then re-sizes the volume, not vice-versa. This
// means that the pool of reserved physical eraseblocks will always be present.
//
// Compatibility constants used by internal volumes.
//
// @UBI_COMPAT_DELETE: delete this internal volume before anything is written
// to the flash
// @UBI_COMPAT_RO: attach this device in read-only mode
// @UBI_COMPAT_PRESERVE: preserve this internal volume - do not touch its
// physical eraseblocks, don't allow the wear-leveling
// sub-system to move them
// @UBI_COMPAT_REJECT: reject this UBI image
//
// Sizes of UBI headers

// Sizes of UBI headers without the ending CRC

//
// struct ubi_ec_hdr - UBI erase counter header.
// @magic: erase counter header magic number (%UBI_EC_HDR_MAGIC)
// @version: version of UBI implementation which is supposed to accept this
// UBI image
// @padding1: reserved for future, zeroes
// @ec: the erase counter
// @vid_hdr_offset: where the VID header starts
// @data_offset: where the user data start
// @image_seq: image sequence number
// @padding2: reserved for future, zeroes
// @hdr_crc: erase counter header CRC checksum
//
// The erase counter header takes 64 bytes and has a plenty of unused space for
// future usage. The unused fields are zeroed. The @version field is used to
// indicate the version of UBI implementation which is supposed to be able to
// work with this UBI image. If @version is greater than the current UBI
// version, the image is rejected. This may be useful in future if something
// is changed radically. This field is duplicated in the volume identifier
// header.
//
// The @vid_hdr_offset and @data_offset fields contain the offset of the
// volume identifier header and user data, relative to the beginning of the
// physical eraseblock. These values have to be the same for all physical
// eraseblocks.
//
// The @image_seq field is used to validate a UBI image that has been prepared
// for a UBI device. The @image_seq value can be any value, but it must be the
// same on all eraseblocks. UBI will ensure that all new erase counter headers
// also contain this value, and will check the value when attaching the flash.
// One way to make use of @image_seq is to increase its value by one every time
// an image is flashed over an existing image, then, if the flashing does not
// complete, UBI will detect the error when attaching the media.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_ec_hdr {
    pub magic: __be32,
    pub version: __u8,
    pub padding1: [__u8; 3],
    pub /: *mut *mut __be64 ec; / Warning: the current limit is 31-bit anyway!,
    pub vid_hdr_offset: __be32,
    pub data_offset: __be32,
    pub image_seq: __be32,
    pub padding2: [__u8; 32],
    pub hdr_crc: __be32,
    pub __packed: },
//
// struct ubi_vid_hdr - on-flash UBI volume identifier header.
// @magic: volume identifier header magic number (%UBI_VID_HDR_MAGIC)
// @version: UBI implementation version which is supposed to accept this UBI
// image (%UBI_VERSION)
// @vol_type: volume type (%UBI_VID_DYNAMIC or %UBI_VID_STATIC)
// @copy_flag: if this logical eraseblock was copied from another physical
// eraseblock (for wear-leveling reasons)
// @compat: compatibility of this volume (%0, %UBI_COMPAT_DELETE,
// %UBI_COMPAT_IGNORE, %UBI_COMPAT_PRESERVE, or %UBI_COMPAT_REJECT)
// @vol_id: ID of this volume
// @lnum: logical eraseblock number
// @padding1: reserved for future, zeroes
// @data_size: how many bytes of data this logical eraseblock contains
// @used_ebs: total number of used logical eraseblocks in this volume
// @data_pad: how many bytes at the end of this physical eraseblock are not
// used
// @data_crc: CRC checksum of the data stored in this logical eraseblock
// @padding2: reserved for future, zeroes
// @sqnum: sequence number
// @padding3: reserved for future, zeroes
// @hdr_crc: volume identifier header CRC checksum
//
// The @sqnum is the value of the global sequence counter at the time when this
// VID header was created. The global sequence counter is incremented each time
// UBI writes a new VID header to the flash, i.e. when it maps a logical
// eraseblock to a new physical eraseblock. The global sequence counter is an
// unsigned 64-bit integer and we assume it never overflows. The @sqnum
// (sequence number) is used to distinguish between older and newer versions of
// logical eraseblocks.
//
// There are 2 situations when there may be more than one physical eraseblock
// corresponding to the same logical eraseblock, i.e., having the same @vol_id
// and @lnum values in the volume identifier header. Suppose we have a logical
// eraseblock L and it is mapped to the physical eraseblock P.
//
// 1. Because UBI may erase physical eraseblocks asynchronously, the following
// situation is possible: L is asynchronously erased, so P is scheduled for
// erasure, then L is written to,i.e. mapped to another physical eraseblock P1,
// so P1 is written to, then an unclean reboot happens. Result - there are 2
// physical eraseblocks P and P1 corresponding to the same logical eraseblock
// L. But P1 has greater sequence number, so UBI picks P1 when it attaches the
// flash.
//
// 2. From time to time UBI moves logical eraseblocks to other physical
// eraseblocks for wear-leveling reasons. If, for example, UBI moves L from P
// to P1, and an unclean reboot happens before P is physically erased, there
// are two physical eraseblocks P and P1 corresponding to L and UBI has to
// select one of them when the flash is attached. The @sqnum field says which
// PEB is the original (obviously P will have lower @sqnum) and the copy. But
// it is not enough to select the physical eraseblock with the higher sequence
// number, because the unclean reboot could have happen in the middle of the
// copying process, so the data in P is corrupted. It is also not enough to
// just select the physical eraseblock with lower sequence number, because the
// data there may be old (consider a case if more data was added to P1 after
// the copying). Moreover, the unclean reboot may happen when the erasure of P
// was just started, so it result in unstable P, which is "mostly" OK, but
// still has unstable bits.
//
// UBI uses the @copy_flag field to indicate that this logical eraseblock is a
// copy. UBI also calculates data CRC when the data is moved and stores it at
// the @data_crc field of the copy (P1). So when UBI needs to pick one physical
// eraseblock of two (P or P1), the @copy_flag of the newer one (P1) is
// examined. If it is cleared, the situation is simple and the newer one is
// picked. If it is set, the data CRC of the copy (P1) is examined. If the CRC
// checksum is correct, this physical eraseblock is selected (P1). Otherwise
// the older one (P) is selected.
//
// There are 2 sorts of volumes in UBI: user volumes and internal volumes.
// Internal volumes are not seen from outside and are used for various internal
// UBI purposes. In this implementation there is only one internal volume - the
// layout volume. Internal volumes are the main mechanism of UBI extensions.
// For example, in future one may introduce a journal internal volume. Internal
// volumes have their own reserved range of IDs.
//
// The @compat field is only used for internal volumes and contains the "degree
// of their compatibility". It is always zero for user volumes. This field
// provides a mechanism to introduce UBI extensions and to be still compatible
// with older UBI binaries. For example, if someone introduced a journal in
// future, he would probably use %UBI_COMPAT_DELETE compatibility for the
// journal volume.  And in this case, older UBI binaries, which know nothing
// about the journal volume, would just delete this volume and work perfectly
// fine. This is similar to what Ext2fs does when it is fed by an Ext3fs image
// - it just ignores the Ext3fs journal.
//
// The @data_crc field contains the CRC checksum of the contents of the logical
// eraseblock if this is a static volume. In case of dynamic volumes, it does
// not contain the CRC checksum as a rule. The only exception is when the
// data of the physical eraseblock was moved by the wear-leveling sub-system,
// then the wear-leveling sub-system calculates the data CRC and stores it in
// the @data_crc field. And of course, the @copy_flag is %in this case.
//
// The @data_size field is used only for static volumes because UBI has to know
// how many bytes of data are stored in this eraseblock. For dynamic volumes,
// this field usually contains zero. The only exception is when the data of the
// physical eraseblock was moved to another physical eraseblock for
// wear-leveling reasons. In this case, UBI calculates CRC checksum of the
// contents and uses both @data_crc and @data_size fields. In this case, the
// @data_size field contains data size.
//
// The @used_ebs field is used only for static volumes and indicates how many
// eraseblocks the data of the volume takes. For dynamic volumes this field is
// not used and always contains zero.
//
// The @data_pad is calculated when volumes are created using the alignment
// parameter. So, effectively, the @data_pad field reduces the size of logical
// eraseblocks of this volume. This is very handy when one uses block-oriented
// software (say, cramfs) on top of the UBI volume.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_vid_hdr {
    pub magic: __be32,
    pub version: __u8,
    pub vol_type: __u8,
    pub copy_flag: __u8,
    pub compat: __u8,
    pub vol_id: __be32,
    pub lnum: __be32,
    pub padding1: [__u8; 4],
    pub data_size: __be32,
    pub used_ebs: __be32,
    pub data_pad: __be32,
    pub data_crc: __be32,
    pub padding2: [__u8; 4],
    pub sqnum: __be64,
    pub padding3: [__u8; 12],
    pub hdr_crc: __be32,
    pub __packed: },
// Internal UBI volumes count
pub const UBI_INT_VOL_COUNT: c_int = 1;
//
// Starting ID of internal volumes: 0x7fffefff.
// There is reserved room for 4096 internal volumes.
//

// The layout volume contains the volume table

pub const UBI_LAYOUT_VOLUME_ALIGN: c_int = 1;
pub const UBI_LAYOUT_VOLUME_EBS: c_int = 2;

// The maximum number of volumes per one UBI device
pub const UBI_MAX_VOLUMES: c_int = 128;
// The maximum volume name length
pub const UBI_VOL_NAME_MAX: c_int = 127;
// Size of the volume table record

// Size of the volume table record without the ending CRC

//
// struct ubi_vtbl_record - a record in the volume table.
// @reserved_pebs: how many physical eraseblocks are reserved for this volume
// @alignment: volume alignment
// @data_pad: how many bytes are unused at the end of the each physical
// eraseblock to satisfy the requested alignment
// @vol_type: volume type (%UBI_DYNAMIC_VOLUME or %UBI_STATIC_VOLUME)
// @upd_marker: if volume update was started but not finished
// @name_len: volume name length
// @name: the volume name
// @flags: volume flags (%UBI_VTBL_AUTORESIZE_FLG)
// @padding: reserved, zeroes
// @crc: a CRC32 checksum of the record
//
// The volume table records are stored in the volume table, which is stored in
// the layout volume. The layout volume consists of 2 logical eraseblock, each
// of which contains a copy of the volume table (i.e., the volume table is
// duplicated). The volume table is an array of &struct ubi_vtbl_record
// objects indexed by the volume ID.
//
// If the size of the logical eraseblock is large enough to fit
// %UBI_MAX_VOLUMES records, the volume table contains %UBI_MAX_VOLUMES
// records. Otherwise, it contains as many records as it can fit (i.e., size of
// logical eraseblock divided by sizeof(struct ubi_vtbl_record)).
//
// The @upd_marker flag is used to implement volume update. It is set to %1
// before update and set to %0 after the update. So if the update operation was
// interrupted, UBI knows that the volume is corrupted.
//
// The @alignment field is specified when the volume is created and cannot be
// later changed. It may be useful, for example, when a block-oriented file
// system works on top of UBI. The @data_pad field is calculated using the
// logical eraseblock size and @alignment. The alignment must be multiple to the
// minimal flash I/O unit. If @alignment is 1, all the available space of
// the physical eraseblocks is used.
//
// Empty records contain all zeroes and the CRC checksum of those zeroes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_vtbl_record {
    pub reserved_pebs: __be32,
    pub alignment: __be32,
    pub data_pad: __be32,
    pub vol_type: __u8,
    pub upd_marker: __u8,
    pub name_len: __be16,
    pub name: [__u8; UBI_VOL_NAME_MAX+1],
    pub flags: __u8,
    pub padding: [__u8; 23],
    pub crc: __be32,
    pub __packed: },
// UBI fastmap on-flash data structures

// fastmap on-flash data structure format version
pub const UBI_FM_FMT_VERSION: c_int = 1;
pub const UBI_FM_SB_MAGIC: c_uint = 0x7B11D69F;
pub const UBI_FM_HDR_MAGIC: c_uint = 0xD4B82EF7;
pub const UBI_FM_VHDR_MAGIC: c_uint = 0xFA370ED1;
pub const UBI_FM_POOL_MAGIC: c_uint = 0x67AF4D08;
pub const UBI_FM_EBA_MAGIC: c_uint = 0xf0c040a8;
// A fastmap super block can be located between PEB 0 and
// UBI_FM_MAX_START
pub const UBI_FM_MAX_START: c_int = 64;
// A fastmap can use up to UBI_FM_MAX_BLOCKS PEBs
pub const UBI_FM_MAX_BLOCKS: c_int = 32;
// 5% of the total number of PEBs have to be scanned while attaching
// from a fastmap.
// But the size of this pool is limited to be between UBI_FM_MIN_POOL_SIZE and
// UBI_FM_MAX_POOL_SIZE
pub const UBI_FM_MIN_POOL_SIZE: c_int = 8;
pub const UBI_FM_MAX_POOL_SIZE: c_int = 256;
//
// struct ubi_fm_sb - UBI fastmap super block
// @magic: fastmap super block magic number (%UBI_FM_SB_MAGIC)
// @version: format version of this fastmap
// @data_crc: CRC over the fastmap data
// @used_blocks: number of PEBs used by this fastmap
// @block_loc: an array containing the location of all PEBs of the fastmap
// @block_ec: the erase counter of each used PEB
// @sqnum: highest sequence number value at the time while taking the fastmap
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_fm_sb {
    pub magic: __be32,
    pub version: __u8,
    pub padding1: [__u8; 3],
    pub data_crc: __be32,
    pub used_blocks: __be32,
    pub block_loc: [__be32; UBI_FM_MAX_BLOCKS],
    pub block_ec: [__be32; UBI_FM_MAX_BLOCKS],
    pub sqnum: __be64,
    pub padding2: [__u8; 32],
    pub __packed: },
//
// struct ubi_fm_hdr - header of the fastmap data set
// @magic: fastmap header magic number (%UBI_FM_HDR_MAGIC)
// @free_peb_count: number of free PEBs known by this fastmap
// @used_peb_count: number of used PEBs known by this fastmap
// @scrub_peb_count: number of to be scrubbed PEBs known by this fastmap
// @bad_peb_count: number of bad PEBs known by this fastmap
// @erase_peb_count: number of bad PEBs which have to be erased
// @vol_count: number of UBI volumes known by this fastmap
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_fm_hdr {
    pub magic: __be32,
    pub free_peb_count: __be32,
    pub used_peb_count: __be32,
    pub scrub_peb_count: __be32,
    pub bad_peb_count: __be32,
    pub erase_peb_count: __be32,
    pub vol_count: __be32,
    pub padding: [__u8; 4],
    pub __packed: },
// struct ubi_fm_hdr is followed by two struct ubi_fm_scan_pool
//
// struct ubi_fm_scan_pool - Fastmap pool PEBs to be scanned while attaching
// @magic: pool magic numer (%UBI_FM_POOL_MAGIC)
// @size: current pool size
// @max_size: maximal pool size
// @pebs: an array containing the location of all PEBs in this pool
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_fm_scan_pool {
    pub magic: __be32,
    pub size: __be16,
    pub max_size: __be16,
    pub pebs: [__be32; UBI_FM_MAX_POOL_SIZE],
    pub padding: [__be32; 4],
    pub __packed: },
// ubi_fm_scan_pool is followed by nfree+nused struct ubi_fm_ec records
//
// struct ubi_fm_ec - stores the erase counter of a PEB
// @pnum: PEB number
// @ec: ec of this PEB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_fm_ec {
    pub pnum: __be32,
    pub ec: __be32,
    pub __packed: },
//
// struct ubi_fm_volhdr - Fastmap volume header
// it identifies the start of an eba table
// @magic: Fastmap volume header magic number (%UBI_FM_VHDR_MAGIC)
// @vol_id: volume id of the fastmapped volume
// @vol_type: type of the fastmapped volume
// @data_pad: data_pad value of the fastmapped volume
// @used_ebs: number of used LEBs within this volume
// @last_eb_bytes: number of bytes used in the last LEB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_fm_volhdr {
    pub magic: __be32,
    pub vol_id: __be32,
    pub vol_type: __u8,
    pub padding1: [__u8; 3],
    pub data_pad: __be32,
    pub used_ebs: __be32,
    pub last_eb_bytes: __be32,
    pub padding2: [__u8; 8],
    pub __packed: },
// struct ubi_fm_volhdr is followed by one struct ubi_fm_eba records
//
// struct ubi_fm_eba - denotes an association between a PEB and LEB
// @magic: EBA table magic number
// @reserved_pebs: number of table entries
// @pnum: PEB number of LEB (LEB is the index)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_fm_eba {
    pub magic: __be32,
    pub reserved_pebs: __be32,
    pub pnum: [__be32; ],
    pub __packed: },
