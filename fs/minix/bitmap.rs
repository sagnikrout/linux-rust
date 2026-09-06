//! Automatically rewritten from C to Rust
//! Source: fs/minix/bitmap.c
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
// linux/fs/minix/bitmap.c
//
// Copyright (C) 1991, 1992  Linus Torvalds
//
// Modified for 680x0 by Hamish Macdonald
// Fixed for 680x0 by Andreas Schwab
//
// bitmap.c contains the code that handles the inode and block bitmaps

    static DEFINE_SPINLOCK(bitmap_lock);
//
// bitmap consists of blocks filled with 16bit words
// bit set == busy, bit clear == free
// endianness is a mess, but for counting zero bits it really doesn't matter...
//
#[no_mangle]
unsafe extern "C" fn count_free(map[]: *mut buffer_head, blocksize: unsigned, numbits: __u32) -> __u32 {
    static __u32 count_free(struct buffer_head *map[], unsigned blocksize, __u32 numbits)
    {
    let mut sum: __u32 = 0;
    let mut blocks: unsigned = DIV_ROUND_UP(numbits, blocksize * 8);
    while (blocks--) {
    let mut words: unsigned = blocksize / 2;
    __u16 *p = (__u16 *)(*map++).b_data;
    while (words--)
    sum += 16 - hweight16(*p++);
    }
    return sum;
    }
#[no_mangle]
pub unsafe extern "C" fn minix_free_block(inode: *mut inode, block: c_ulong) {
    void minix_free_block(struct inode *inode, unsigned long block)
    {
    struct super_block *sb = inode.i_sb;
    struct minix_sb_info *sbi = minix_sb(sb);
    struct buffer_head *bh;
    let mut k: c_int = sb.s_blocksize_bits + 3;
    unsigned long bit, zone;
    if (block < sbi.s_firstdatazone || block >= sbi.s_nzones) {
    printk("Trying to free block not in datazone\n");
    return;
    }
    zone = block - sbi.s_firstdatazone + 1;
    bit = zone & ((1<<k) - 1);
    zone >>= k;
    if (zone >= sbi.s_zmap_blocks) {
    printk("minix_free_block: nonexistent bitmap buffer\n");
    return;
    }
    bh = sbi.s_zmap[zone];
    spin_lock(&bitmap_lock);
    if (!minix_test_and_clear_bit(bit, bh.b_data))
    printk("minix_free_block (%s:%lu): bit already cleared\n",
    sb.s_id, block);
    spin_unlock(&bitmap_lock);
    mark_buffer_dirty(bh);
    return;
    }
#[no_mangle]
pub unsafe extern "C" fn minix_new_block(inode: *mut *mut inode) -> c_int {
    int minix_new_block(struct inode * inode)
    {
    struct minix_sb_info *sbi = minix_sb(inode.i_sb);
    let mut bits_per_zone: c_int = 8 * inode.i_sb.s_blocksize;
    int i;
    for (i = 0; i < sbi.s_zmap_blocks; i++) {
    struct buffer_head *bh = sbi.s_zmap[i];
    int j;
    spin_lock(&bitmap_lock);
    j = minix_find_first_zero_bit(bh.b_data, bits_per_zone);
    if (j < bits_per_zone) {
    minix_set_bit(j, bh.b_data);
    spin_unlock(&bitmap_lock);
    mark_buffer_dirty(bh);
    j += i * bits_per_zone + sbi.s_firstdatazone-1;
    if (j < sbi.s_firstdatazone || j >= sbi.s_nzones)
    break;
    return j;
    }
    spin_unlock(&bitmap_lock);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn minix_count_free_blocks(sb: *mut super_block) -> c_ulong {
    unsigned long minix_count_free_blocks(struct super_block *sb)
    {
    struct minix_sb_info *sbi = minix_sb(sb);
    let mut bits: u32 = sbi.s_nzones - sbi.s_firstdatazone + 1;
    return (count_free(sbi.s_zmap, sb.s_blocksize, bits)
    << sbi.s_log_zone_size);
    }
    struct minix_inode *
    minix_V1_raw_inode(struct super_block *sb, ino_t ino, struct buffer_head **bh)
    {
    int block;
    struct minix_sb_info *sbi = minix_sb(sb);
    struct minix_inode *p;
    if (!ino || ino > sbi.s_ninodes) {
    printk("Bad inode number on dev %s: %ld is out of range\n",
    sb.s_id, (long)ino);
    return core::ptr::null_mut();
    }
    ino--;
    block = 2 + sbi.s_imap_blocks + sbi.s_zmap_blocks +
    ino / MINIX_INODES_PER_BLOCK;
// bh = sb_bread(sb, block);
    if (!*bh) {
    printk("Unable to read inode block\n");
    return core::ptr::null_mut();
    }
    p = (void *)(*bh).b_data;
    return p + ino % MINIX_INODES_PER_BLOCK;
    }
    struct minix2_inode *
    minix_V2_raw_inode(struct super_block *sb, ino_t ino, struct buffer_head **bh)
    {
    int block;
    struct minix_sb_info *sbi = minix_sb(sb);
    struct minix2_inode *p;
    let mut minix2_inodes_per_block: c_int = sb.s_blocksize / sizeof(struct minix2_inode);
// bh = NULL;
    if (!ino || ino > sbi.s_ninodes) {
    printk("Bad inode number on dev %s: %ld is out of range\n",
    sb.s_id, (long)ino);
    return core::ptr::null_mut();
    }
    ino--;
    block = 2 + sbi.s_imap_blocks + sbi.s_zmap_blocks +
    ino / minix2_inodes_per_block;
// bh = sb_bread(sb, block);
    if (!*bh) {
    printk("Unable to read inode block\n");
    return core::ptr::null_mut();
    }
    p = (void *)(*bh).b_data;
    return p + ino % minix2_inodes_per_block;
    }
// Clear the link count and mode of a deleted inode on disk.
#[no_mangle]
unsafe extern "C" fn minix_clear_inode(inode: *mut inode) {
    static void minix_clear_inode(struct inode *inode)
    {
    struct buffer_head *bh = core::ptr::null_mut();
    if (INODE_VERSION(inode) == MINIX_V1) {
    struct minix_inode *raw_inode;
    raw_inode = minix_V1_raw_inode(inode.i_sb, inode.i_ino, &bh);
    if (raw_inode) {
    raw_inode.i_nlinks = 0;
    raw_inode.i_mode = 0;
    }
    } else {
    struct minix2_inode *raw_inode;
    raw_inode = minix_V2_raw_inode(inode.i_sb, inode.i_ino, &bh);
    if (raw_inode) {
    raw_inode.i_nlinks = 0;
    raw_inode.i_mode = 0;
    }
    }
    if (bh) {
    mark_buffer_dirty(bh);
    brelse (bh);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn minix_free_inode(inode: *mut *mut inode) {
    void minix_free_inode(struct inode * inode)
    {
    struct super_block *sb = inode.i_sb;
    struct minix_sb_info *sbi = minix_sb(inode.i_sb);
    struct buffer_head *bh;
    let mut k: c_int = sb.s_blocksize_bits + 3;
    unsigned long ino, bit;
    ino = inode.i_ino;
    if (ino < 1 || ino > sbi.s_ninodes) {
    printk("minix_free_inode: inode 0 or nonexistent inode\n");
    return;
    }
    bit = ino & ((1<<k) - 1);
    ino >>= k;
    if (ino >= sbi.s_imap_blocks) {
    printk("minix_free_inode: nonexistent imap in superblock\n");
    return;
    }
    minix_clear_inode(inode);	/* clear on-disk copy */
    bh = sbi.s_imap[ino];
    spin_lock(&bitmap_lock);
    if (!minix_test_and_clear_bit(bit, bh.b_data))
    printk("minix_free_inode: bit %lu already cleared\n", bit);
    spin_unlock(&bitmap_lock);
    mark_buffer_dirty(bh);
    }
    struct inode *minix_new_inode(const struct inode *dir, umode_t mode)
    {
    struct super_block *sb = dir.i_sb;
    struct minix_sb_info *sbi = minix_sb(sb);
    struct inode *inode = new_inode(sb);
    struct buffer_head * bh;
    let mut bits_per_zone: c_int = 8 * sb.s_blocksize;
    unsigned long j;
    int i;
    if (!inode)
    return ERR_PTR(-ENOMEM);
    j = bits_per_zone;
    bh = core::ptr::null_mut();
    spin_lock(&bitmap_lock);
    for (i = 0; i < sbi.s_imap_blocks; i++) {
    bh = sbi.s_imap[i];
    j = minix_find_first_zero_bit(bh.b_data, bits_per_zone);
    if (j < bits_per_zone)
    break;
    }
    if (!bh || j >= bits_per_zone) {
    spin_unlock(&bitmap_lock);
    iput(inode);
    return ERR_PTR(-ENOSPC);
    }
    if (minix_test_and_set_bit(j, bh.b_data)) {	/* shouldn't happen */
    spin_unlock(&bitmap_lock);
    printk("minix_new_inode: bit already set\n");
    iput(inode);
    return ERR_PTR(-ENOSPC);
    }
    spin_unlock(&bitmap_lock);
    mark_buffer_dirty(bh);
    j += i * bits_per_zone;
    if (!j || j > sbi.s_ninodes) {
    iput(inode);
    return ERR_PTR(-EFSCORRUPTED);
    }
    inode_init_owner(&nop_mnt_idmap, inode, dir, mode);
    inode.i_ino = j;
    simple_inode_init_ts(inode);
    inode.i_blocks = 0;
    memset(&minix_i(inode).u, 0, sizeof(minix_i(inode).u));
    insert_inode_hash(inode);
    mark_inode_dirty(inode);
    return inode;
    }
#[no_mangle]
pub unsafe extern "C" fn minix_count_free_inodes(sb: *mut super_block) -> c_ulong {
    unsigned long minix_count_free_inodes(struct super_block *sb)
    {
    struct minix_sb_info *sbi = minix_sb(sb);
    let mut bits: u32 = sbi.s_ninodes + 1;
    return count_free(sbi.s_imap, sb.s_blocksize, bits);
    }
