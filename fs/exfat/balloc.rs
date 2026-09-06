//! Automatically rewritten from C to Rust
//! Source: fs/exfat/balloc.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2012-2013 Samsung Electronics Co., Ltd.
//

//
// Allocation Bitmap Management Functions
//
    static bool exfat_test_bitmap_range(struct super_block *sb, unsigned int clu,
    unsigned int count)
    {
    struct exfat_sb_info *sbi = EXFAT_SB(sb);
    let mut start: c_uint = clu;
    let mut end: c_uint = clu + count;
    unsigned int ent_idx, i, b;
    unsigned int bit_offset, bits_to_check;
    __le_long *bitmap_le;
    unsigned long mask, word;
    if (!is_valid_cluster(sbi, start) || !is_valid_cluster(sbi, end - 1))
    return false;
    while (start < end) {
    ent_idx = CLUSTER_TO_BITMAP_ENT(start);
    i = BITMAP_OFFSET_SECTOR_INDEX(sb, ent_idx);
    b = BITMAP_OFFSET_BIT_IN_SECTOR(sb, ent_idx);
    bitmap_le = (__le_long *)sbi.vol_amap[i].b_data;
// Calculate how many bits we can check in the current word
    bit_offset = b % BITS_PER_LONG;
    bits_to_check = min(end - start,
    (unsigned int)(BITS_PER_LONG - bit_offset));
// Create a bitmask for the range of bits to check
    if (bits_to_check >= BITS_PER_LONG)
    mask = ~0UL;
    else
    mask = ((1UL << bits_to_check) - 1) << bit_offset;
    word = lel_to_cpu(bitmap_le[b / BITS_PER_LONG]);
// Check if all bits in the mask are set
    if ((word & mask) != mask)
    return false;
    start += bits_to_check;
    }
    return true;
    }
    static int exfat_allocate_bitmap(struct super_block *sb,
    struct exfat_dentry *ep)
    {
    struct exfat_sb_info *sbi = EXFAT_SB(sb);
    long long map_size;
    unsigned int i, j, need_map_size;
    sector_t sector, end, ra;
    let mut ra_cnt: blkcnt_t = 0;
    sbi.map_clu = le32_to_cpu(ep.dentry.bitmap.start_clu);
    map_size = le64_to_cpu(ep.dentry.bitmap.size);
    need_map_size = ((EXFAT_DATA_CLUSTER_COUNT(sbi) - 1) / BITS_PER_BYTE)
    + 1;
    if (need_map_size != map_size) {
    exfat_err(sb, "bogus allocation bitmap size(need : %u, cur : %lld)",
    need_map_size, map_size);
//
// Only allowed when bogus allocation
// bitmap size is large
//
    if (need_map_size > map_size)
    return -EIO;
    }
    sbi.map_sectors = ((need_map_size - 1) >>
    (sb.s_blocksize_bits)) + 1;
    sbi.vol_amap = kvmalloc_objs(struct buffer_head *, sbi.map_sectors);
    if (!sbi.vol_amap)
    return -ENOMEM;
    sector = ra = exfat_cluster_to_sector(sbi, sbi.map_clu);
    end = sector + sbi.map_sectors - 1;
    for (i = 0; i < sbi.map_sectors; i++) {
// Trigger the next readahead in advance.
    exfat_blk_readahead(sb, sector + i, &ra, &ra_cnt, end);
    sbi.vol_amap[i] = sb_bread(sb, sector + i);
    if (!sbi.vol_amap[i])
    goto err_out;
    }
    if (exfat_test_bitmap_range(sb, sbi.map_clu,
    exfat_bytes_to_cluster_round_up(sbi, map_size)) == false)
    goto err_out;
    return 0;
    err_out:
    j = 0;
// release all buffers and free vol_amap
    while (j < i)
    brelse(sbi.vol_amap[j++]);
    kvfree(sbi.vol_amap);
    sbi.vol_amap = core::ptr::null_mut();
    return -EIO;
    }
#[no_mangle]
pub unsafe extern "C" fn exfat_load_bitmap(sb: *mut super_block) -> c_int {
    int exfat_load_bitmap(struct super_block *sb)
    {
    unsigned int i, type;
    struct exfat_chain clu;
    struct exfat_sb_info *sbi = EXFAT_SB(sb);
    exfat_chain_set(&clu, sbi.root_dir, 0, ALLOC_FAT_CHAIN);
    while (clu.dir != EXFAT_EOF_CLUSTER) {
    for (i = 0; i < sbi.dentries_per_clu; i++) {
    struct exfat_dentry *ep;
    struct buffer_head *bh;
    ep = exfat_get_dentry(sb, &clu, i, &bh);
    if (!ep)
    return -EIO;
    type = exfat_get_entry_type(ep);
    if (type == TYPE_BITMAP &&
    ep.dentry.bitmap.flags == 0x0) {
    int err;
    err = exfat_allocate_bitmap(sb, ep);
    brelse(bh);
    return err;
    }
    brelse(bh);
    if (type == TYPE_UNUSED)
    return -EINVAL;
    }
    if (exfat_get_next_cluster(sb, &clu.dir))
    return -EIO;
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn exfat_free_bitmap(sbi: *mut exfat_sb_info) {
    void exfat_free_bitmap(struct exfat_sb_info *sbi)
    {
    int i;
    for (i = 0; i < sbi.map_sectors; i++)
    __brelse(sbi.vol_amap[i]);
    kvfree(sbi.vol_amap);
    }
#[no_mangle]
pub unsafe extern "C" fn exfat_set_bitmap(sb: *mut super_block, clu: c_uint, sync: bool) -> c_int {
    int exfat_set_bitmap(struct super_block *sb, unsigned int clu, bool sync)
    {
    int i, b;
    unsigned int ent_idx;
    struct exfat_sb_info *sbi = EXFAT_SB(sb);
    if (!is_valid_cluster(sbi, clu))
    return -EINVAL;
    ent_idx = CLUSTER_TO_BITMAP_ENT(clu);
    i = BITMAP_OFFSET_SECTOR_INDEX(sb, ent_idx);
    b = BITMAP_OFFSET_BIT_IN_SECTOR(sb, ent_idx);
    set_bit_le(b, sbi.vol_amap[i].b_data);
    exfat_update_bh(sbi.vol_amap[i], sync);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn exfat_clear_bitmap(sb: *mut super_block, clu: c_uint, sync: bool) -> c_int {
    int exfat_clear_bitmap(struct super_block *sb, unsigned int clu, bool sync)
    {
    int i, b;
    unsigned int ent_idx;
    struct exfat_sb_info *sbi = EXFAT_SB(sb);
    if (!is_valid_cluster(sbi, clu))
    return -EIO;
    ent_idx = CLUSTER_TO_BITMAP_ENT(clu);
    i = BITMAP_OFFSET_SECTOR_INDEX(sb, ent_idx);
    b = BITMAP_OFFSET_BIT_IN_SECTOR(sb, ent_idx);
    if (!test_bit_le(b, sbi.vol_amap[i].b_data))
    return -EIO;
    clear_bit_le(b, sbi.vol_amap[i].b_data);
    exfat_update_bh(sbi.vol_amap[i], sync);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn exfat_test_bitmap(sb: *mut super_block, clu: c_uint) -> bool {
    bool exfat_test_bitmap(struct super_block *sb, unsigned int clu)
    {
    int i, b;
    unsigned int ent_idx;
    struct exfat_sb_info *sbi = EXFAT_SB(sb);
    if (!sbi.vol_amap)
    return true;
    if (!is_valid_cluster(sbi, clu))
    return false;
    ent_idx = CLUSTER_TO_BITMAP_ENT(clu);
    i = BITMAP_OFFSET_SECTOR_INDEX(sb, ent_idx);
    b = BITMAP_OFFSET_BIT_IN_SECTOR(sb, ent_idx);
    if (!test_bit_le(b, sbi.vol_amap[i].b_data))
    return false;
    return true;
    }
//
// If the value of "clu" is 0, it means cluster 2 which is the first cluster of
// the cluster heap.
//
#[no_mangle]
pub unsafe extern "C" fn exfat_find_free_bitmap(sb: *mut super_block, clu: c_uint) -> c_uint {
    unsigned int exfat_find_free_bitmap(struct super_block *sb, unsigned int clu)
    {
    unsigned int i, map_i, map_b, ent_idx;
    unsigned int clu_base, clu_free;
    unsigned long clu_bits, clu_mask;
    struct exfat_sb_info *sbi = EXFAT_SB(sb);
    __le_long bitval;
    WARN_ON(clu < EXFAT_FIRST_CLUSTER);
    ent_idx = ALIGN_DOWN(CLUSTER_TO_BITMAP_ENT(clu), BITS_PER_LONG);
    clu_base = BITMAP_ENT_TO_CLUSTER(ent_idx);
    clu_mask = IGNORED_BITS_REMAINED(clu, clu_base);
    map_i = BITMAP_OFFSET_SECTOR_INDEX(sb, ent_idx);
    map_b = BITMAP_OFFSET_BYTE_IN_SECTOR(sb, ent_idx);
    for (i = EXFAT_FIRST_CLUSTER; i < sbi.num_clusters;
    i += BITS_PER_LONG) {
    bitval = *(__le_long *)(sbi.vol_amap[map_i].b_data + map_b);
    if (clu_mask > 0) {
    bitval |= cpu_to_lel(clu_mask);
    clu_mask = 0;
    }
    if (lel_to_cpu(bitval) != ULONG_MAX) {
    clu_bits = lel_to_cpu(bitval);
    clu_free = clu_base + ffz(clu_bits);
    if (clu_free < sbi.num_clusters)
    return clu_free;
    }
    clu_base += BITS_PER_LONG;
    map_b += sizeof(long);
    if (map_b >= sb.s_blocksize ||
    clu_base >= sbi.num_clusters) {
    if (++map_i >= sbi.map_sectors) {
    clu_base = EXFAT_FIRST_CLUSTER;
    map_i = 0;
    }
    map_b = 0;
    }
    }
    return EXFAT_EOF_CLUSTER;
    }
#[no_mangle]
pub unsafe extern "C" fn exfat_count_used_clusters(sb: *mut super_block, ret_count: *mut c_uint) -> c_int {
    int exfat_count_used_clusters(struct super_block *sb, unsigned int *ret_count)
    {
    struct exfat_sb_info *sbi = EXFAT_SB(sb);
    let mut count: c_uint = 0;
    unsigned int i, map_i = 0, map_b = 0;
    let mut total_clus: c_uint = EXFAT_DATA_CLUSTER_COUNT(sbi);
    let mut last_mask: c_uint = total_clus & (BITS_PER_LONG - 1);
    unsigned long *bitmap, clu_bits;
    total_clus &= ~last_mask;
    for (i = 0; i < total_clus; i += BITS_PER_LONG) {
    bitmap = (void *)(sbi.vol_amap[map_i].b_data + map_b);
    count += hweight_long(*bitmap);
    map_b += sizeof(long);
    if (map_b >= (unsigned int)sb.s_blocksize) {
    map_i++;
    map_b = 0;
    }
    }
    if (last_mask) {
    bitmap = (void *)(sbi.vol_amap[map_i].b_data + map_b);
    clu_bits = lel_to_cpu(*(__le_long *)bitmap);
    count += hweight_long(clu_bits & BITMAP_LAST_WORD_MASK(last_mask));
    }
// ret_count = count;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn exfat_trim_fs(inode: *mut inode, range: *mut fstrim_range) -> c_int {
    int exfat_trim_fs(struct inode *inode, struct fstrim_range *range)
    {
    unsigned int trim_begin, trim_end, count, next_free_clu;
    u64 clu_start, clu_end, trim_minlen, trimmed_total = 0;
    struct super_block *sb = inode.i_sb;
    struct exfat_sb_info *sbi = EXFAT_SB(sb);
    let mut err: c_int = 0;
    clu_start = max_t(u64, range.start >> sbi.cluster_size_bits,
    EXFAT_FIRST_CLUSTER);
    clu_end = clu_start + (range.len >> sbi.cluster_size_bits) - 1;
    trim_minlen = range.minlen >> sbi.cluster_size_bits;
    if (clu_start >= sbi.num_clusters || range.len < sbi.cluster_size)
    return -EINVAL;
    if (clu_end >= sbi.num_clusters)
    clu_end = sbi.num_clusters - 1;
    mutex_lock(&sbi.bitmap_lock);
    trim_begin = trim_end = exfat_find_free_bitmap(sb, clu_start);
//
// exfat_find_free_bitmap() may wrap around to the beginning of
// the bitmap. Reject a cluster outside the requested range.
//
    if (trim_begin == EXFAT_EOF_CLUSTER ||
    trim_begin < clu_start || trim_begin > clu_end)
    goto unlock;
    for (;;) {
    if (trim_end >= clu_end)
    break;
    next_free_clu = exfat_find_free_bitmap(sb, trim_end + 1);
//
// Stop if the search wrapped around or moved beyond the requested
// FITRIM range.
//
    if (next_free_clu == EXFAT_EOF_CLUSTER ||
    next_free_clu <= trim_end || next_free_clu > clu_end)
    break;
    if (next_free_clu == trim_end + 1) {
// extend trim range for continuous free cluster
    trim_end++;
    } else {
// trim current range if it's larger than trim_minlen
    count = trim_end - trim_begin + 1;
    if (count >= trim_minlen) {
    err = sb_issue_discard(sb,
    exfat_cluster_to_sector(sbi, trim_begin),
    count * sbi.sect_per_clus, GFP_NOFS, 0);
    if (err)
    goto unlock;
    trimmed_total += count;
    }
// set next start point of the free hole
    trim_begin = trim_end = next_free_clu;
    }
    if (fatal_signal_pending(current)) {
    err = -ERESTARTSYS;
    goto unlock;
    }
    }
// try to trim remainder
    count = trim_end - trim_begin + 1;
    if (count >= trim_minlen) {
    err = sb_issue_discard(sb, exfat_cluster_to_sector(sbi, trim_begin),
    count * sbi.sect_per_clus, GFP_NOFS, 0);
    if (err)
    goto unlock;
    trimmed_total += count;
    }
    unlock:
    mutex_unlock(&sbi.bitmap_lock);
    range.len = trimmed_total << sbi.cluster_size_bits;
    return err;
    }
