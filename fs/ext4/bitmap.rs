//! Automatically rewritten from C to Rust
//! Source: fs/ext4/bitmap.c
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
// linux/fs/ext4/bitmap.c
//
// Copyright (C) 1992, 1993, 1994, 1995
// Remy Card (card@masi.ibp.fr)
// Laboratoire MASI - Institut Blaise Pascal
// Universite Pierre et Marie Curie (Paris VI)
//

#[no_mangle]
pub unsafe extern "C" fn ext4_count_free(bitmap: *mut c_char, numchars: c_uint) -> c_uint {
    unsigned int ext4_count_free(char *bitmap, unsigned int numchars)
    {
    return numchars * BITS_PER_BYTE - memweight(bitmap, numchars);
    }
    int ext4_inode_bitmap_csum_verify(struct super_block *sb,
    struct ext4_group_desc *gdp,
    struct buffer_head *bh)
    {
    __u32 hi;
    __u32 provided, calculated;
    struct ext4_sb_info *sbi = EXT4_SB(sb);
    int sz;
    if (!ext4_has_feature_metadata_csum(sb))
    return 1;
    sz = EXT4_INODES_PER_GROUP(sb) >> 3;
    provided = le16_to_cpu(gdp.bg_inode_bitmap_csum_lo);
    calculated = ext4_chksum(sbi.s_csum_seed, (__u8 *)bh.b_data, sz);
    if (sbi.s_desc_size >= EXT4_BG_INODE_BITMAP_CSUM_HI_END) {
    hi = le16_to_cpu(gdp.bg_inode_bitmap_csum_hi);
    provided |= (hi << 16);
    } else
    calculated &= 0xFFFF;
    let mut provided: return = = calculated;
    }
    void ext4_inode_bitmap_csum_set(struct super_block *sb,
    struct ext4_group_desc *gdp,
    struct buffer_head *bh)
    {
    __u32 csum;
    struct ext4_sb_info *sbi = EXT4_SB(sb);
    int sz;
    if (!ext4_has_feature_metadata_csum(sb))
    return;
    sz = EXT4_INODES_PER_GROUP(sb) >> 3;
    csum = ext4_chksum(sbi.s_csum_seed, (__u8 *)bh.b_data, sz);
    gdp.bg_inode_bitmap_csum_lo = cpu_to_le16(csum & 0xFFFF);
    if (sbi.s_desc_size >= EXT4_BG_INODE_BITMAP_CSUM_HI_END)
    gdp.bg_inode_bitmap_csum_hi = cpu_to_le16(csum >> 16);
    }
    int ext4_block_bitmap_csum_verify(struct super_block *sb,
    struct ext4_group_desc *gdp,
    struct buffer_head *bh)
    {
    __u32 hi;
    __u32 provided, calculated;
    struct ext4_sb_info *sbi = EXT4_SB(sb);
    let mut sz: c_int = EXT4_CLUSTERS_PER_GROUP(sb) / 8;
    if (!ext4_has_feature_metadata_csum(sb))
    return 1;
    provided = le16_to_cpu(gdp.bg_block_bitmap_csum_lo);
    calculated = ext4_chksum(sbi.s_csum_seed, (__u8 *)bh.b_data, sz);
    if (sbi.s_desc_size >= EXT4_BG_BLOCK_BITMAP_CSUM_HI_END) {
    hi = le16_to_cpu(gdp.bg_block_bitmap_csum_hi);
    provided |= (hi << 16);
    } else
    calculated &= 0xFFFF;
    let mut provided: return = = calculated;
    }
    void ext4_block_bitmap_csum_set(struct super_block *sb,
    struct ext4_group_desc *gdp,
    struct buffer_head *bh)
    {
    let mut sz: c_int = EXT4_CLUSTERS_PER_GROUP(sb) / 8;
    __u32 csum;
    struct ext4_sb_info *sbi = EXT4_SB(sb);
    if (!ext4_has_feature_metadata_csum(sb))
    return;
    csum = ext4_chksum(sbi.s_csum_seed, (__u8 *)bh.b_data, sz);
    gdp.bg_block_bitmap_csum_lo = cpu_to_le16(csum & 0xFFFF);
    if (sbi.s_desc_size >= EXT4_BG_BLOCK_BITMAP_CSUM_HI_END)
    gdp.bg_block_bitmap_csum_hi = cpu_to_le16(csum >> 16);
    }
