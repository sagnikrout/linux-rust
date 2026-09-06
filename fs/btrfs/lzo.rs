//! Automatically rewritten from C to Rust
//! Source: fs/btrfs/lzo.c
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
// Copyright (C) 2008 Oracle.  All rights reserved.
//

pub const LZO_LEN: c_int = 4;
//
// Btrfs LZO compression format
//
// Regular and inlined LZO compressed data extents consist of:
//
// 1.  Header
// Fixed size. LZO_LEN (4) bytes long, LE32.
// Records the total size (including the header) of compressed data.
//
// 2.  Segment(s)
// Variable size. Each segment includes one segment header, followed by data
// payload.
// One regular LZO compressed extent can have one or more segments.
// For inlined LZO compressed extent, only one segment is allowed.
// One segment represents at most one sector of uncompressed data.
//
// 2.1 Segment header
// Fixed size. LZO_LEN (4) bytes long, LE32.
// Records the total size of the segment (not including the header).
// Segment header never crosses sector boundary, thus it's possible to
// have at most 3 padding zeros at the end of the sector.
//
// 2.2 Data Payload
// Variable size. Size up limit should be lzo1x_worst_compress(sectorsize)
// which is 4419 for a 4KiB sectorsize.
//
// Example with 4K sectorsize:
// Page 1:
// 0     0x2   0x4   0x6   0x8   0xa   0xc   0xe     0x10
// 0x0000   |  Header   | SegHdr 01 | Data payload 01 ...     |
// ...
// 0x0ff0   | SegHdr  N | Data payload  N     ...          |00|
// ^^ padding zeros
// Page 2:
// 0x1000   | SegHdr N+1| Data payload N+1 ...                |
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct workspace {
    pub mem: *mut c_void,
    pub /: *mut *mut *mut void buf; / where decompressed data goes,
    pub /: *mut *mut *mut void cbuf; / where compressed data goes,
    pub list: list_head,
}

#[no_mangle]
unsafe extern "C" fn workspace_buf_length(fs_info: *const btrfs_fs_info) -> u32 {
    static u32 workspace_buf_length(const struct btrfs_fs_info *fs_info)
    {
    return lzo1x_worst_compress(fs_info.sectorsize);
    }
#[no_mangle]
unsafe extern "C" fn workspace_cbuf_length(fs_info: *const btrfs_fs_info) -> u32 {
    static u32 workspace_cbuf_length(const struct btrfs_fs_info *fs_info)
    {
    return lzo1x_worst_compress(fs_info.sectorsize);
    }
#[no_mangle]
pub unsafe extern "C" fn lzo_free_workspace(ws: *mut list_head) {
    void lzo_free_workspace(struct list_head *ws)
    {
    struct workspace *workspace = list_entry(ws, struct workspace, list);
    kvfree(workspace.buf);
    kvfree(workspace.cbuf);
    kvfree(workspace.mem);
    kfree(workspace);
    }
    struct list_head *lzo_alloc_workspace(struct btrfs_fs_info *fs_info)
    {
    struct workspace *workspace;
    workspace = kzalloc_obj(*workspace);
    if (!workspace)
    return ERR_PTR(-ENOMEM);
    workspace.mem = kvmalloc(LZO1X_MEM_COMPRESS, GFP_KERNEL | __GFP_NOWARN);
    workspace.buf = kvmalloc(workspace_buf_length(fs_info), GFP_KERNEL | __GFP_NOWARN);
    workspace.cbuf = kvmalloc(workspace_cbuf_length(fs_info), GFP_KERNEL | __GFP_NOWARN);
    if (!workspace.mem || !workspace.buf || !workspace.cbuf)
    goto fail;
    INIT_LIST_HEAD(&workspace.list);
    return &workspace.list;
    fail:
    lzo_free_workspace(&workspace.list);
    return ERR_PTR(-ENOMEM);
    }
//
// Write data into @out_folio and queue it into @out_bio.
//
// Return 0 if everything is fine and @total_out will be increased.
// Return <0 for error.
//
// The @out_folio can be NULL after a full folio is queued.
// Thus the caller should check and allocate a new folio when needed.
//
    static int write_and_queue_folio(struct bio *out_bio, struct folio **out_folio,
    u32 *total_out, u32 write_len)
    {
    let mut fsize: u32 = folio_size(*out_folio);
    let mut foffset: u32 = offset_in_folio(*out_folio, *total_out);
    ASSERT(out_folio && *out_folio);
// Should not cross folio boundary.
    ASSERT(foffset + write_len <= fsize);
// We can not use bio_add_folio_nofail() which doesn't do any merge.
    if (!bio_add_folio(out_bio, *out_folio, write_len, foffset)) {
//
// We have allocated a bio that havs BTRFS_MAX_COMPRESSED_PAGES
// vecs, and all ranges inside the same folio should have been
// merged.  If bio_add_folio() still failed, that means we have
// reached the bvec limits.
//
// This should only happen at the beginning of a folio, and
// caller is responsible for releasing the folio, since it's
// not yet queued into the bio.
//
    ASSERT(IS_ALIGNED(*total_out, fsize));
    return -E2BIG;
    }
// total_out += write_len;
//
// The full folio has been filled and queued, reset @out_folio to NULL,
// so that error handling is fully handled by the bio.
//
    if (IS_ALIGNED(*total_out, fsize))
// out_folio = NULL;
    return 0;
    }
//
// Copy compressed data to bio.
//
// @out_bio:		The bio that will contain all the compressed data.
// @compressed_data:	The compressed data of this segment.
// @compressed_size:	The size of the compressed data.
// @out_folio:		The current output folio, will be updated if a new
// folio is allocated.
// @total_out:		The total bytes of current output.
// @max_out:		The maximum size of the compressed data.
//
// Will do:
//
// - Write a segment header into the destination
// - Copy the compressed buffer into the destination
// - Make sure we have enough space in the last sector to fit a segment header
// If not, we will pad at most (LZO_LEN (4)) - 1 bytes of zeros.
// - If a full folio is filled, it will be queued into @out_bio, and @out_folio
// will be updated.
//
// Will allocate new pages when needed.
//
    static int copy_compressed_data_to_bio(struct btrfs_fs_info *fs_info,
    struct bio *out_bio,
    const char *compressed_data,
    size_t compressed_size,
    struct folio **out_folio,
    u32 *total_out, u32 max_out)
    {
    let mut sectorsize: u32 = fs_info.sectorsize;
    let mut sectorsize_bits: u32 = fs_info.sectorsize_bits;
    let mut fsize: u32 = btrfs_min_folio_size(fs_info);
    let mut old_size: u32 = out_bio.bi_iter.bi_size;
    u32 copy_start;
    u32 sector_bytes_left;
    char *kaddr;
    int ret;
    ASSERT(out_folio);
// There should be at least a lzo header queued.
    ASSERT(old_size);
    ASSERT(old_size == *total_out);
//
// We never allow a segment header crossing sector boundary, previous
// run should ensure we have enough space left inside the sector.
//
    ASSERT((old_size >> sectorsize_bits) == (old_size + LZO_LEN - 1) >> sectorsize_bits);
    if (!*out_folio) {
// out_folio = btrfs_alloc_compr_folio(fs_info, GFP_NOFS);
    if (!*out_folio)
    return -ENOMEM;
    }
// Write the segment header first.
    kaddr = kmap_local_folio(*out_folio, offset_in_folio(*out_folio, *total_out));
    put_unaligned_le32(compressed_size, kaddr);
    kunmap_local(kaddr);
    ret = write_and_queue_folio(out_bio, out_folio, total_out, LZO_LEN);
    if (ret < 0)
    return ret;
    copy_start = *total_out;
// Copy compressed data.
    while (*total_out - copy_start < compressed_size) {
    u32 copy_len = min_t(u32, sectorsize - *total_out % sectorsize,
    copy_start + compressed_size - *total_out);
    let mut foffset: u32 = *total_out & (fsize - 1);
// With the range copied, we're larger than the original range.
    if (((*total_out + copy_len) >> sectorsize_bits) >=
    max_out >> sectorsize_bits)
    return -E2BIG;
    if (!*out_folio) {
// out_folio = btrfs_alloc_compr_folio(fs_info, GFP_NOFS);
    if (!*out_folio)
    return -ENOMEM;
    }
    kaddr = kmap_local_folio(*out_folio, foffset);
    memcpy(kaddr, compressed_data + *total_out - copy_start, copy_len);
    kunmap_local(kaddr);
    ret = write_and_queue_folio(out_bio, out_folio, total_out, copy_len);
    if (ret < 0)
    return ret;
    }
//
// Check if we can fit the next segment header into the remaining space
// of the sector.
//
    sector_bytes_left = round_up(*total_out, sectorsize) - *total_out;
    if (sector_bytes_left >= LZO_LEN || sector_bytes_left == 0)
    return 0;
    ASSERT(*out_folio);
// The remaining size is not enough, pad it with zeros
    folio_zero_range(*out_folio, offset_in_folio(*out_folio, *total_out), sector_bytes_left);
    return write_and_queue_folio(out_bio, out_folio, total_out, sector_bytes_left);
    }
#[no_mangle]
pub unsafe extern "C" fn lzo_compress_bio(ws: *mut list_head, cb: *mut compressed_bio) -> c_int {
    int lzo_compress_bio(struct list_head *ws, struct compressed_bio *cb)
    {
    struct btrfs_inode *inode = cb.bbio.inode;
    struct btrfs_fs_info *fs_info = inode.root.fs_info;
    struct workspace *workspace = list_entry(ws, struct workspace, list);
    struct bio *bio = &cb.bbio.bio;
    let mut start: u64 = cb.start;
    let mut len: u32 = cb.len;
    let mut sectorsize: u32 = fs_info.sectorsize;
    let mut min_folio_size: u32 = btrfs_min_folio_size(fs_info);
    struct address_space *mapping = inode.vfs_inode.i_mapping;
    struct folio *folio_in = core::ptr::null_mut();
    struct folio *folio_out = core::ptr::null_mut();
    char *sizes_ptr;
    let mut ret: c_int = 0;
// Points to the file offset of input data.
    let mut cur_in: u64 = start;
// Points to the current output byte.
    let mut total_out: u32 = 0;
    ASSERT(bio.bi_iter.bi_size == 0);
    ASSERT(len);
    folio_out = btrfs_alloc_compr_folio(fs_info, GFP_NOFS);
    if (!folio_out)
    return -ENOMEM;
// Queue a segment header first.
    ret = write_and_queue_folio(bio, &folio_out, &total_out, LZO_LEN);
// The first header should not fail.
    ASSERT(ret == 0);
    while (cur_in < start + len) {
    char *data_in;
    let mut sectorsize_mask: u32 = sectorsize - 1;
    let mut sector_off: u32 = (cur_in - start) & sectorsize_mask;
    u32 in_len;
    size_t out_len;
// Get the input page first.
    if (!folio_in) {
    ret = btrfs_compress_filemap_get_folio(mapping, cur_in, &folio_in);
    if (ret < 0)
    goto out;
    }
// Compress at most one sector of data each time.
    in_len = min_t(u32, start + len - cur_in, sectorsize - sector_off);
    ASSERT(in_len);
    data_in = kmap_local_folio(folio_in, offset_in_folio(folio_in, cur_in));
    ret = lzo1x_1_compress(data_in, in_len, workspace.cbuf, &out_len,
    workspace.mem);
    kunmap_local(data_in);
    if (unlikely(ret < 0)) {
// lzo1x_1_compress never fails.
    ret = -EIO;
    goto out;
    }
    ret = copy_compressed_data_to_bio(fs_info, bio, workspace.cbuf, out_len,
    &folio_out, &total_out, len);
    if (ret < 0)
    goto out;
    cur_in += in_len;
//
// Check if we're making it bigger after two sectors.  And if
// it is so, give up.
//
    if (cur_in - start > sectorsize * 2 && cur_in - start < total_out) {
    ret = -E2BIG;
    goto out;
    }
// Check if we have reached input folio boundary.
    if (IS_ALIGNED(cur_in, min_folio_size)) {
    folio_put(folio_in);
    folio_in = core::ptr::null_mut();
    }
    }
//
// The last folio is already queued. Bio is responsible for freeing
// those folios now.
//
    folio_out = core::ptr::null_mut();
// Store the size of all chunks of compressed data
    sizes_ptr = kmap_local_folio(bio_first_folio_all(bio), 0);
    put_unaligned_le32(total_out, sizes_ptr);
    kunmap_local(sizes_ptr);
    out:
//
// We can only free the folio that has no part queued into the bio.
//
// As any folio that is already queued into bio will be released by
// the endio function of bio.
//
    if (folio_out && IS_ALIGNED(total_out, min_folio_size)) {
    btrfs_free_compr_folio(folio_out);
    folio_out = core::ptr::null_mut();
    }
    if (folio_in)
    folio_put(folio_in);
    return ret;
    }
    static struct folio *get_current_folio(struct compressed_bio *cb, struct folio_iter *fi,
    u32 *cur_folio_index, u32 cur_in)
    {
    struct btrfs_fs_info *fs_info = cb_to_fs_info(cb);
    let mut min_folio_shift: u32 = PAGE_SHIFT + fs_info.block_min_order;
    ASSERT(cur_folio_index);
// Need to switch to the next folio.
    if (cur_in >> min_folio_shift != *cur_folio_index) {
// We can only do the switch one folio a time.
    ASSERT(cur_in >> min_folio_shift == *cur_folio_index + 1);
    bio_next_folio(fi, &cb.bbio.bio);
    (*cur_folio_index)++;
    }
    return fi.folio;
    }
//
// Copy the compressed segment payload into @dest.
//
// For the payload there will be no padding, just need to do page switching.
//
    static void copy_compressed_segment(struct compressed_bio *cb,
    struct folio_iter *fi, u32 *cur_folio_index,
    char *dest, u32 len, u32 *cur_in)
    {
    let mut orig_in: u32 = *cur_in;
    while (*cur_in < orig_in + len) {
    struct folio *cur_folio = get_current_folio(cb, fi, cur_folio_index, *cur_in);
    u32 copy_len;
    ASSERT(cur_folio);
    copy_len = min_t(u32, orig_in + len - *cur_in,
    folio_size(cur_folio) - offset_in_folio(cur_folio, *cur_in));
    ASSERT(copy_len);
    memcpy_from_folio(dest + *cur_in - orig_in, cur_folio,
    offset_in_folio(cur_folio, *cur_in), copy_len);
// cur_in += copy_len;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn lzo_decompress_bio(ws: *mut list_head, cb: *mut compressed_bio) -> c_int {
    int lzo_decompress_bio(struct list_head *ws, struct compressed_bio *cb)
    {
    struct workspace *workspace = list_entry(ws, struct workspace, list);
    struct btrfs_fs_info *fs_info = cb.bbio.inode.root.fs_info;
    let mut sectorsize: u32 = fs_info.sectorsize;
    let mut compressed_len: u32 = bio_get_size(&cb.bbio.bio);
    struct folio_iter fi;
    char *kaddr;
    int ret;
// Compressed data length, can be unaligned
    u32 len_in;
// Offset inside the compressed data
    let mut cur_in: u32 = 0;
// Bytes decompressed so far
    let mut cur_out: u32 = 0;
// The current folio index number inside the bio.
    let mut cur_folio_index: u32 = 0;
    bio_first_folio(&fi, &cb.bbio.bio, 0);
// There must be a compressed folio and matches the sectorsize.
    if (unlikely(!fi.folio))
    return -EINVAL;
    ASSERT(folio_size(fi.folio) == btrfs_min_folio_size(fs_info));
    kaddr = kmap_local_folio(fi.folio, 0);
    len_in = get_unaligned_le32(kaddr);
    kunmap_local(kaddr);
    cur_in += LZO_LEN;
//
// LZO header length check
//
// The total length should not exceed the maximum extent length,
// and all sectors should be used.
// If this happens, it means the compressed extent is corrupted.
//
    if (unlikely(len_in > min_t(size_t, BTRFS_MAX_COMPRESSED, compressed_len) ||
    round_up(len_in, sectorsize) < compressed_len)) {
    struct btrfs_inode *inode = cb.bbio.inode;
    btrfs_err(fs_info,
    "lzo header invalid, root %llu inode %llu offset %llu lzo len %u compressed len %u",
    btrfs_root_id(inode.root), btrfs_ino(inode),
    cb.start, len_in, compressed_len);
    return -EUCLEAN;
    }
// Go through each lzo segment
    while (cur_in < len_in) {
    struct folio *cur_folio;
// Length of the compressed segment
    u32 seg_len;
    u32 sector_bytes_left;
    let mut out_len: usize = lzo1x_worst_compress(sectorsize);
//
// We should always have enough space for one segment header
// inside current sector.
//
    ASSERT(cur_in / sectorsize ==
    (cur_in + LZO_LEN - 1) / sectorsize);
    cur_folio = get_current_folio(cb, &fi, &cur_folio_index, cur_in);
    ASSERT(cur_folio);
    kaddr = kmap_local_folio(cur_folio, 0);
    seg_len = get_unaligned_le32(kaddr + offset_in_folio(cur_folio, cur_in));
    kunmap_local(kaddr);
    cur_in += LZO_LEN;
    if (unlikely(seg_len > workspace_cbuf_length(fs_info))) {
    struct btrfs_inode *inode = cb.bbio.inode;
//
// seg_len shouldn't be larger than we have allocated
// for workspace->cbuf
//
    btrfs_err(fs_info,
    "lzo segment too big, root %llu inode %llu offset %llu len %u",
    btrfs_root_id(inode.root), btrfs_ino(inode),
    cb.start, seg_len);
    return -EIO;
    }
// The segment must not extend beyond the compressed input.
    if (unlikely(cur_in + seg_len > compressed_len)) {
    struct btrfs_inode *inode = cb.bbio.inode;
    btrfs_err(fs_info,
    "lzo segment overflows compressed input, root %llu inode %llu offset %llu cur_in %u len %u compressed len %u",
    btrfs_root_id(inode.root), btrfs_ino(inode),
    cb.start, cur_in, seg_len, compressed_len);
    return -EUCLEAN;
    }
// Copy the compressed segment payload into workspace
    copy_compressed_segment(cb, &fi, &cur_folio_index, workspace.cbuf,
    seg_len, &cur_in);
// Decompress the data
    ret = lzo1x_decompress_safe(workspace.cbuf, seg_len,
    workspace.buf, &out_len);
    if (unlikely(ret != LZO_E_OK)) {
    struct btrfs_inode *inode = cb.bbio.inode;
    btrfs_err(fs_info,
    "lzo decompression failed, error %d root %llu inode %llu offset %llu",
    ret, btrfs_root_id(inode.root), btrfs_ino(inode),
    cb.start);
    return -EIO;
    }
// Copy the data into inode pages
    ret = btrfs_decompress_buf2page(workspace.buf, out_len, cb, cur_out);
    cur_out += out_len;
// All data read, exit
    if (ret == 0)
    return 0;
    ret = 0;
// Check if the sector has enough space for a segment header
    sector_bytes_left = sectorsize - (cur_in % sectorsize);
    if (sector_bytes_left >= LZO_LEN)
    continue;
// Skip the padding zeros
    cur_in += sector_bytes_left;
    }
    return 0;
    }
    int lzo_decompress(struct list_head *ws, const u8 *data_in,
    struct folio *dest_folio, unsigned long dest_pgoff, size_t srclen,
    size_t destlen)
    {
    struct workspace *workspace = list_entry(ws, struct workspace, list);
    struct btrfs_fs_info *fs_info = folio_to_fs_info(dest_folio);
    let mut sectorsize: u32 = fs_info.sectorsize;
    size_t in_len;
    size_t out_len;
    let mut max_segment_len: usize = workspace_buf_length(fs_info);
    int ret;
    if (unlikely(srclen <= LZO_LEN * 2 ||
    srclen > max_segment_len + LZO_LEN * 2)) {
    btrfs_err(fs_info, "invalid lzo header length, has %zu expect (%u, %zu)",
    srclen, LZO_LEN * 2, max_segment_len + LZO_LEN * 2);
    return -EUCLEAN;
    }
    in_len = get_unaligned_le32(data_in);
    if (unlikely(in_len != srclen)) {
    btrfs_err(fs_info, "invalid lzo header length, has %zu expect %zu",
    in_len, srclen);
    return -EUCLEAN;
    }
    data_in += LZO_LEN;
    in_len = get_unaligned_le32(data_in);
    if (unlikely(in_len != srclen - LZO_LEN * 2)) {
    btrfs_err(fs_info, "invalid lzo segment length, has %zu expect %zu",
    in_len, srclen - LZO_LEN * 2);
    return -EUCLEAN;
    }
    data_in += LZO_LEN;
    out_len = sectorsize;
    ret = lzo1x_decompress_safe(data_in, in_len, workspace.buf, &out_len);
    if (unlikely(ret != LZO_E_OK)) {
    struct btrfs_inode *inode = folio_to_inode(dest_folio);
    btrfs_err(fs_info,
    "lzo decompression failed, error %d root %llu inode %llu offset %llu",
    ret, btrfs_root_id(inode.root), btrfs_ino(inode),
    folio_pos(dest_folio));
    return -EIO;
    }
    ASSERT(out_len <= sectorsize);
    memcpy_to_folio(dest_folio, dest_pgoff, workspace.buf, out_len);
// Early end, considered as an error.
    if (unlikely(out_len < destlen)) {
    folio_zero_range(dest_folio, dest_pgoff + out_len, destlen - out_len);
    return -EIO;
    }
    return 0;
    }
    const struct btrfs_compress_levels  btrfs_lzo_compress = {
    .max_level		= 1,
    .default_level		= 1,
    };
