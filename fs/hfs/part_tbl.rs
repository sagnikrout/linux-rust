//! Automatically rewritten from C to Rust
//! Source: fs/hfs/part_tbl.c
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
// linux/fs/hfs/part_tbl.c
//
// Copyright (C) 1996-1997  Paul H. Hargrove
// (C) 2003 Ardis Technologies <roman@ardistech.com>
// This file may be distributed under the terms of the GNU General Public License.
//
// Original code to handle the new style Mac partition table based on
// a patch contributed by Holger Schemel (aeglos@valinor.owl.de).
//

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
//
// hfs_part_find()
//
// Parse the partition map looking for the
// start and length of the 'part'th HFS partition.
//
    int hfs_part_find(struct super_block *sb,
    sector_t *part_start, sector_t *part_size)
    {
    pub bh: *mut buffer_head,
    pub data: *mut __be16,
    pub res: int i, size,,
    pub -ENOENT: res =,
    pub data): *mut *mut bh = sb_bread512(sb, part_start + HFS_PMAP_BLK,,
    if (!bh)
    pub -EIO: return,
    switch (be16_to_cpu(*data)) {
    case HFS_OLD_PMAP_MAGIC:
    {
    pub pm: *mut old_pmap,
    pub p: *mut old_pmap_entry,
    pub )bh->b_data: *mut pm = (struct old_pmap,
    pub pm->pdEntry: p =,
    pub 42: size =,
    pub {: for (i = 0; i < size; p++, i++),
    if (p.pdStart && p.pdSize &&
    p.pdFSID == cpu_to_be32(0x54465331)/*"TFS1"*/ &&
    (HFS_SB(sb).part < 0 || HFS_SB(sb).part == i)) {
// part_start += be32_to_cpu(p->pdStart);
// part_size = be32_to_cpu(p->pdSize);
    pub 0: res =,
    }
    }
    }
    case HFS_NEW_PMAP_MAGIC:
    {
    pub pm: *mut new_pmap,
    pub )bh->b_data: *mut pm = (struct new_pmap,
    pub be32_to_cpu(pm->pmMapBlkCnt): size =,
    pub {: for (i = 0; i < size;),
    if (!memcmp(pm.pmPartType,"Apple_HFS", 9) &&
    (HFS_SB(sb).part < 0 || HFS_SB(sb).part == i)) {
// part_start += be32_to_cpu(pm->pmPyPartStart);
// part_size = be32_to_cpu(pm->pmPartBlkCnt);
    pub 0: res =,
    }
    pub pm): *mut *mut bh = sb_bread512(sb, part_start + HFS_PMAP_BLK + ++i,,
    if (!bh)
    pub -EIO: return,
    if (pm.pmSig != cpu_to_be16(HFS_NEW_PMAP_MAGIC))
    }
    }
    }
    pub res: return,
    }
