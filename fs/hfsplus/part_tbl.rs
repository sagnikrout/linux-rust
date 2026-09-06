//! Automatically rewritten from C to Rust
//! Source: fs/hfsplus/part_tbl.c
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


//
// linux/fs/hfsplus/part_tbl.c
//
// Copyright (C) 1996-1997  Paul H. Hargrove
// This file may be distributed under the terms of
// the GNU General Public License.
//
// Original code to handle the new style Mac partition table based on
// a patch contributed by Holger Schemel (aeglos@valinor.owl.de).
//
// In function preconditions the term "valid" applied to a pointer to
// a structure means that the pointer is non-NULL and the structure it
// points to has all fields initialized to consistent values.
//

// offsets to various blocks

// magic numbers for various disk blocks
pub const HFS_DRVR_DESC_MAGIC: c_uint = 0x4552 /* "ER": driver descriptor map */;
pub const HFS_OLD_PMAP_MAGIC: c_uint = 0x5453 /* "TS": old-type partition map */;
pub const HFS_NEW_PMAP_MAGIC: c_uint = 0x504D /* "PM": new-type partition map */;
pub const HFS_SUPER_MAGIC: c_uint = 0x4244 /* "BD": HFS MDB (super block) */;
pub const HFS_MFS_SUPER_MAGIC: c_uint = 0xD2D7 /* MFS MDB (super block) */;
//
// The new style Mac partition map
//
// For each partition on the media there is a physical block (512-byte
// block) containing one of these structures.  These blocks are
// contiguous starting at block 1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct new_pmap {
    pub /: *mut *mut __be16 pmSig; / signature,
    pub /: *mut *mut __be16 reSigPad; / padding,
    pub /: *mut *mut __be32 pmMapBlkCnt; / partition blocks count,
    pub /: *mut *mut __be32 pmPyPartStart; / physical block start of partition,
    pub /: *mut *mut __be32 pmPartBlkCnt; / physical block count of partition,
    pub string: *mut *mut u8 pmPartName[32]; / (null terminated?),
    giving the name of this
    partition */
    pub string: *mut *mut u8 pmPartType[32]; / (null terminated?),
    giving the type of this
    partition */
// a bunch more stuff we don't need
    pub __packed: },
//
// The old style Mac partition map
//
// The partition map consists for a 2-byte signature followed by an
// array of these structures.  The map is terminated with an all-zero
// one of these.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct old_pmap {
    pub /: *mut *mut __be16 pdSig; / Signature bytes,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct old_pmap_entry {
    pub pdStart: __be32,
    pub pdSize: __be32,
    pub pdFSID: __be32,
    pub pdEntry: [}; 42],
    pub __packed: },
    static int hfs_parse_old_pmap(struct super_block *sb, struct old_pmap *pm,
    sector_t *part_start, sector_t *part_size)
    {
    pub HFSPLUS_SB(sb): *mut *mut hfsplus_sb_info sbi =,
    pub i: c_int,
    pub {: for (i = 0; i < 42; i++),
    pub &pm->pdEntry[i]: *mut *mut old_pmap_entry p =,
    if (p.pdStart && p.pdSize &&
    p.pdFSID == cpu_to_be32(0x54465331)/*"TFS1"*/ &&
    (sbi.part < 0 || sbi.part == i)) {
// part_start += be32_to_cpu(p->pdStart);
// part_size = be32_to_cpu(p->pdSize);
    pub 0: return,
    }
    }
    pub -ENOENT: return,
    }
    static int hfs_parse_new_pmap(struct super_block *sb, void *buf,
    struct new_pmap *pm, sector_t *part_start, sector_t *part_size)
    {
    pub HFSPLUS_SB(sb): *mut *mut hfsplus_sb_info sbi =,
    pub be32_to_cpu(pm->pmMapBlkCnt): int size =,
    pub hfsplus_min_io_size(sb): int buf_size =,
    pub res: c_int,
    pub 0: int i =,
    do {
    if (!memcmp(pm.pmPartType, "Apple_HFS", 9) &&
    (sbi.part < 0 || sbi.part == i)) {
// part_start += be32_to_cpu(pm->pmPyPartStart);
// part_size = be32_to_cpu(pm->pmPartBlkCnt);
    pub 0: return,
    }
    if (++i >= size)
    pub -ENOENT: return,
    pub HFSPLUS_SECTOR_SIZE): *mut *mut *mut pm = (struct new_pmap )((u8 )pm +,
    if ((u8 *)pm - (u8 *)buf >= buf_size) {
    res = hfsplus_submit_bio(sb,
// part_start + HFS_PMAP_BLK + i,
    pub REQ_OP_READ): *mut *mut *mut buf, (void )&pm,,
    if (res)
    pub res: return,
    }
    pub cpu_to_be16(HFS_NEW_PMAP_MAGIC)): } while (pm->pmSig ==,
    pub -ENOENT: return,
    }
//
// Parse the partition map looking for the start and length of a
// HFS/HFS+ partition.
//
    int hfs_part_find(struct super_block *sb,
    sector_t *part_start, sector_t *part_size)
    {
    pub data: *mut *mut void buf,,
    pub res: c_int,
    pub GFP_KERNEL): buf = kmalloc(hfsplus_min_io_size(sb),,
    if (!buf)
    pub -ENOMEM: return,
    res = hfsplus_submit_bio(sb, *part_start + HFS_PMAP_BLK,
    pub REQ_OP_READ): buf, &data,,
    if (res)
    pub out: goto,
    switch (be16_to_cpu(*((__be16 *)data))) {
    case HFS_OLD_PMAP_MAGIC:
    pub part_size): res = hfs_parse_old_pmap(sb, data, part_start,,
    case HFS_NEW_PMAP_MAGIC:
    pub part_size): res = hfs_parse_new_pmap(sb, buf, data, part_start,,
    default:
    pub -ENOENT: res =,
    }
    out:
    pub res: return,
    }
