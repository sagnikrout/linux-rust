//! Automatically rewritten from C to Rust
//! Source: fs/omfs/bitmap.c
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

#[no_mangle]
pub unsafe extern "C" fn omfs_count_free(sb: *mut super_block) -> c_ulong {
    unsigned long omfs_count_free(struct super_block *sb)
    {
    unsigned int i;
    let mut sum: c_ulong = 0;
    struct omfs_sb_info *sbi = OMFS_SB(sb);
    let mut nbits: c_int = sb.s_blocksize * 8;
    for (i = 0; i < sbi.s_imap_size; i++)
    sum += nbits - bitmap_weight(sbi.s_imap[i], nbits);
    return sum;
    }
//
// Counts the run of zero bits starting at bit up to max.
// It handles the case where a run might spill over a buffer.
// Called with bitmap lock.
//
    static int count_run(unsigned long **addr, int nbits,
    int addrlen, int bit, int max)
    {
    let mut count: c_int = 0;
    int x;
    for (; addrlen > 0; addrlen--, addr++) {
    x = find_next_bit(*addr, nbits, bit);
    count += x - bit;
    if (x < nbits || count > max)
    return min(count, max);
    bit = 0;
    }
    return min(count, max);
    }
//
// Sets or clears the run of count bits starting with bit.
// Called with bitmap lock.
//
    static int set_run(struct super_block *sb, int map,
    int nbits, int bit, int count, int set)
    {
    int i;
    int err;
    struct buffer_head *bh;
    struct omfs_sb_info *sbi = OMFS_SB(sb);
    err = -ENOMEM;
    bh = sb_bread(sb, clus_to_blk(sbi, sbi.s_bitmap_ino) + map);
    if (!bh)
    goto out;
    for (i = 0; i < count; i++, bit++) {
    if (bit >= nbits) {
    bit = 0;
    map++;
    mark_buffer_dirty(bh);
    brelse(bh);
    bh = sb_bread(sb,
    clus_to_blk(sbi, sbi.s_bitmap_ino) + map);
    if (!bh)
    goto out;
    }
    if (set) {
    set_bit(bit, sbi.s_imap[map]);
    set_bit(bit, (unsigned long *)bh.b_data);
    } else {
    clear_bit(bit, sbi.s_imap[map]);
    clear_bit(bit, (unsigned long *)bh.b_data);
    }
    }
    mark_buffer_dirty(bh);
    brelse(bh);
    err = 0;
    out:
    return err;
    }
//
// Tries to allocate exactly one block.  Returns true if successful.
//
#[no_mangle]
pub unsafe extern "C" fn omfs_allocate_block(sb: *mut super_block, block: u64) -> c_int {
    int omfs_allocate_block(struct super_block *sb, u64 block)
    {
    struct buffer_head *bh;
    struct omfs_sb_info *sbi = OMFS_SB(sb);
    let mut bits_per_entry: c_int = 8 * sb.s_blocksize;
    unsigned int map, bit;
    let mut ret: c_int = 0;
    u64 tmp;
    tmp = block;
    bit = do_div(tmp, bits_per_entry);
    map = tmp;
    mutex_lock(&sbi.s_bitmap_lock);
    if (map >= sbi.s_imap_size || test_and_set_bit(bit, sbi.s_imap[map]))
    goto out;
    if (sbi.s_bitmap_ino > 0) {
    bh = sb_bread(sb, clus_to_blk(sbi, sbi.s_bitmap_ino) + map);
    if (!bh)
    goto out;
    set_bit(bit, (unsigned long *)bh.b_data);
    mark_buffer_dirty(bh);
    brelse(bh);
    }
    ret = 1;
    out:
    mutex_unlock(&sbi.s_bitmap_lock);
    return ret;
    }
//
// Tries to allocate a set of blocks.	The request size depends on the
// type: for inodes, we must allocate sbi->s_mirrors blocks, and for file
// blocks, we try to allocate sbi->s_clustersize, but can always get away
// with just one block.
//
    int omfs_allocate_range(struct super_block *sb,
    int min_request,
    int max_request,
    u64 *return_block,
    int *return_size)
    {
    struct omfs_sb_info *sbi = OMFS_SB(sb);
    let mut bits_per_entry: c_int = 8 * sb.s_blocksize;
    let mut ret: c_int = 0;
    int i, run, bit;
    mutex_lock(&sbi.s_bitmap_lock);
    for (i = 0; i < sbi.s_imap_size; i++) {
    bit = 0;
    while (bit < bits_per_entry) {
    bit = find_next_zero_bit(sbi.s_imap[i], bits_per_entry,
    bit);
    if (bit == bits_per_entry)
    break;
    run = count_run(&sbi.s_imap[i], bits_per_entry,
    sbi.s_imap_size-i, bit, max_request);
    if (run >= min_request)
    goto found;
    bit += run;
    }
    }
    ret = -ENOSPC;
    goto out;
    found:
// return_block = (u64) i * bits_per_entry + bit;
// return_size = run;
    ret = set_run(sb, i, bits_per_entry, bit, run, 1);
    out:
    mutex_unlock(&sbi.s_bitmap_lock);
    return ret;
    }
//
// Clears count bits starting at a given block.
//
#[no_mangle]
pub unsafe extern "C" fn omfs_clear_range(sb: *mut super_block, block: u64, count: c_int) -> c_int {
    int omfs_clear_range(struct super_block *sb, u64 block, int count)
    {
    struct omfs_sb_info *sbi = OMFS_SB(sb);
    let mut bits_per_entry: c_int = 8 * sb.s_blocksize;
    u64 tmp;
    unsigned int map, bit;
    int ret;
    tmp = block;
    bit = do_div(tmp, bits_per_entry);
    map = tmp;
    if (map >= sbi.s_imap_size)
    return 0;
    mutex_lock(&sbi.s_bitmap_lock);
    ret = set_run(sb, map, bits_per_entry, bit, count, 0);
    mutex_unlock(&sbi.s_bitmap_lock);
    return ret;
    }
