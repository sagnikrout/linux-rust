//! Automatically rewritten from C to Rust
//! Source: fs/squashfs/file.c
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
// Squashfs - a compressed read only filesystem for Linux
//
// Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008
// Phillip Lougher <phillip@squashfs.org.uk>
//
// file.c
//
// This file contains code for handling regular files.  A regular file
// consists of a sequence of contiguous compressed blocks, and/or a
// compressed fragment block (tail-end packed block).   The compressed size
// of each datablock is stored in a block list contained within the
// file inode (itself stored in one or more compressed metadata blocks).
//
// To speed up access to datablocks when reading 'large' files (256 Mbytes or
// larger), the code implements an index cache that caches the mapping from
// block index to datablock location on disk.
//
// The index cache allows Squashfs to handle large files (up to 1.75 TiB) while
// retaining a simple and space-efficient block list on disk.  The cache
// is split into slots, caching up to eight 224 GiB files (128 KiB blocks).
// Larger files use multiple slots, with 1.75 TiB files using all 8 slots.
// The index cache is designed to be memory efficient, and by default uses
// 16 KiB.
//

//
// Locate cache slot in range [offset, index] for specified inode.  If
// there's more than one return the slot closest to index.
//
    static struct meta_index *locate_meta_index(struct inode *inode, int offset,
    int index)
    {
    struct meta_index *meta = core::ptr::null_mut();
    struct squashfs_sb_info *msblk = inode.i_sb.s_fs_info;
    int i;
    mutex_lock(&msblk.meta_index_mutex);
    TRACE("locate_meta_index: index %d, offset %d\n", index, offset);
    if (msblk.meta_index == core::ptr::null_mut())
    goto not_allocated;
    for (i = 0; i < SQUASHFS_META_SLOTS; i++) {
    if (msblk.meta_index[i].inode_number == inode.i_ino &&
    msblk.meta_index[i].offset >= offset &&
    msblk.meta_index[i].offset <= index &&
    msblk.meta_index[i].locked == 0) {
    TRACE("locate_meta_index: entry %d, offset %d\n", i,
    msblk.meta_index[i].offset);
    meta = &msblk.meta_index[i];
    offset = meta.offset;
    }
    }
    if (meta)
    meta.locked = 1;
    not_allocated:
    mutex_unlock(&msblk.meta_index_mutex);
    return meta;
    }
//
// Find and initialise an empty cache slot for index offset.
//
    static struct meta_index *empty_meta_index(struct inode *inode, int offset,
    int skip)
    {
    struct squashfs_sb_info *msblk = inode.i_sb.s_fs_info;
    struct meta_index *meta = core::ptr::null_mut();
    int i;
    mutex_lock(&msblk.meta_index_mutex);
    TRACE("empty_meta_index: offset %d, skip %d\n", offset, skip);
    if (msblk.meta_index == core::ptr::null_mut()) {
//
// First time cache index has been used, allocate and
// initialise.  The cache index could be allocated at
// mount time but doing it here means it is allocated only
// if a 'large' file is read.
//
    msblk.meta_index = kzalloc_objs(*(msblk.meta_index),
    SQUASHFS_META_SLOTS);
    if (msblk.meta_index == core::ptr::null_mut()) {
    ERROR("Failed to allocate meta_index\n");
    goto failed;
    }
    for (i = 0; i < SQUASHFS_META_SLOTS; i++) {
    msblk.meta_index[i].inode_number = 0;
    msblk.meta_index[i].locked = 0;
    }
    msblk.next_meta_index = 0;
    }
    for (i = SQUASHFS_META_SLOTS; i &&
    msblk.meta_index[msblk.next_meta_index].locked; i--)
    msblk.next_meta_index = (msblk.next_meta_index + 1) %
    SQUASHFS_META_SLOTS;
    if (i == 0) {
    TRACE("empty_meta_index: failed!\n");
    goto failed;
    }
    TRACE("empty_meta_index: returned meta entry %d, %p\n",
    msblk.next_meta_index,
    &msblk.meta_index[msblk.next_meta_index]);
    meta = &msblk.meta_index[msblk.next_meta_index];
    msblk.next_meta_index = (msblk.next_meta_index + 1) %
    SQUASHFS_META_SLOTS;
    meta.inode_number = inode.i_ino;
    meta.offset = offset;
    meta.skip = skip;
    meta.entries = 0;
    meta.locked = 1;
    failed:
    mutex_unlock(&msblk.meta_index_mutex);
    return meta;
    }
#[no_mangle]
unsafe extern "C" fn release_meta_index(inode: *mut inode, meta: *mut meta_index) {
    static void release_meta_index(struct inode *inode, struct meta_index *meta)
    {
    struct squashfs_sb_info *msblk = inode.i_sb.s_fs_info;
    mutex_lock(&msblk.meta_index_mutex);
    meta.locked = 0;
    mutex_unlock(&msblk.meta_index_mutex);
    }
//
// Read the next n blocks from the block list, starting from
// metadata block <start_block, offset>.
//
    static long long read_indexes(struct super_block *sb, int n,
    u64 *start_block, int *offset)
    {
    int err, i;
    let mut block: c_longlong = 0;
    __le32 *blist = kmalloc(PAGE_SIZE, GFP_KERNEL);
    if (blist == core::ptr::null_mut()) {
    ERROR("read_indexes: Failed to allocate block_list\n");
    return -ENOMEM;
    }
    while (n) {
    let mut blocks: c_int = min_t(int, n, PAGE_SIZE >> 2);
    err = squashfs_read_metadata(sb, blist, start_block,
    offset, blocks << 2);
    if (err < 0) {
    ERROR("read_indexes: reading block [%llx:%x]\n",
// start_block, *offset);
    goto failure;
    }
    for (i = 0; i < blocks; i++) {
    let mut size: c_int = squashfs_block_size(blist[i]);
    if (size < 0) {
    err = size;
    goto failure;
    }
    block += SQUASHFS_COMPRESSED_SIZE_BLOCK(size);
    }
    n -= blocks;
    }
    kfree(blist);
    return block;
    failure:
    kfree(blist);
    return err;
    }
//
// Each cache index slot has SQUASHFS_META_ENTRIES, each of which
// can cache one index -> datablock/blocklist-block mapping.  We wish
// to distribute these over the length of the file, entry[0] maps index x,
// entry[1] maps index x + skip, entry[2] maps index x + 2 * skip, and so on.
// The larger the file, the greater the skip factor.  The skip factor is
// limited to the size of the metadata cache (SQUASHFS_CACHED_BLKS) to ensure
// the number of metadata blocks that need to be read fits into the cache.
// If the skip factor is limited in this way then the file will use multiple
// slots.
//
#[no_mangle]
pub unsafe extern "C" fn calculate_skip(blocks: u64) -> c_int {
    static inline int calculate_skip(u64 blocks)
    {
    u64 skip = blocks / ((SQUASHFS_META_ENTRIES + 1)
// SQUASHFS_META_INDEXES);
    return min((u64) SQUASHFS_CACHED_BLKS - 1, skip + 1);
    }
//
// Search and grow the index cache for the specified inode, returning the
// on-disk locations of the datablock and block list metadata block
// <index_block, index_offset> for index (scaled to nearest cache index).
//
    static int fill_meta_index(struct inode *inode, int index,
    u64 *index_block, int *index_offset, u64 *data_block)
    {
    struct squashfs_sb_info *msblk = inode.i_sb.s_fs_info;
    let mut skip: c_int = calculate_skip(i_size_read(inode) >> msblk.block_log);
    let mut offset: c_int = 0;
    struct meta_index *meta;
    struct meta_entry *meta_entry;
    let mut cur_index_block: u64 = squashfs_i(inode).block_list_start;
    let mut cur_offset: c_int = squashfs_i(inode).offset;
    let mut cur_data_block: u64 = squashfs_i(inode).start;
    int err, i;
//
// Scale index to cache index (cache slot entry)
//
    index /= SQUASHFS_META_INDEXES * skip;
    while (offset < index) {
    meta = locate_meta_index(inode, offset + 1, index);
    if (meta == core::ptr::null_mut()) {
    meta = empty_meta_index(inode, offset + 1, skip);
    if (meta == core::ptr::null_mut())
    goto all_done;
    } else {
    offset = index < meta.offset + meta.entries ? index :
    meta.offset + meta.entries - 1;
    meta_entry = &meta.meta_entry[offset - meta.offset];
    cur_index_block = meta_entry.index_block +
    msblk.inode_table;
    cur_offset = meta_entry.offset;
    cur_data_block = meta_entry.data_block;
    TRACE("get_meta_index: offset %d, meta.offset %d, "
    "meta.entries %d\n", offset, meta.offset,
    meta.entries);
    TRACE("get_meta_index: index_block 0x%llx, offset 0x%x"
    " data_block 0x%llx\n", cur_index_block,
    cur_offset, cur_data_block);
    }
//
// If necessary grow cache slot by reading block list.  Cache
// slot is extended up to index or to the end of the slot, in
// which case further slots will be used.
//
    for (i = meta.offset + meta.entries; i <= index &&
    i < meta.offset + SQUASHFS_META_ENTRIES; i++) {
    let mut blocks: c_int = skip * SQUASHFS_META_INDEXES;
    long long res = read_indexes(inode.i_sb, blocks,
    &cur_index_block, &cur_offset);
    if (res < 0) {
    if (meta.entries == 0)
//
// Don't leave an empty slot on read
// error allocated to this inode...
//
    meta.inode_number = 0;
    err = res;
    goto failed;
    }
    cur_data_block += res;
    meta_entry = &meta.meta_entry[i - meta.offset];
    meta_entry.index_block = cur_index_block -
    msblk.inode_table;
    meta_entry.offset = cur_offset;
    meta_entry.data_block = cur_data_block;
    meta.entries++;
    offset++;
    }
    TRACE("get_meta_index: meta.offset %d, meta.entries %d\n",
    meta.offset, meta.entries);
    release_meta_index(inode, meta);
    }
    all_done:
// index_block = cur_index_block;
// index_offset = cur_offset;
    if (data_block)
// data_block = cur_data_block;
//
// Scale cache index (cache slot entry) to index
//
    return offset * SQUASHFS_META_INDEXES * skip;
    failed:
    release_meta_index(inode, meta);
    return err;
    }
//
// Get the on-disk location and compressed size of the datablock
// specified by index.  Fill_meta_index() does most of the work.
//
    static int read_blocklist_ptrs(struct inode *inode, int index, u64 *start,
    int *offset, u64 *block)
    {
    long long blks;
    __le32 size;
    let mut res: c_int = fill_meta_index(inode, index, start, offset, block);
    TRACE("read_blocklist: res %d, index %d, start 0x%llx, offset 0x%x, block 0x%llx\n",
    res, index, *start, *offset, block ? *block : 0);
    if (res < 0)
    return res;
//
// res contains the index of the mapping returned by fill_meta_index(),
// this will likely be less than the desired index (because the
// meta_index cache works at a higher granularity).  Read any
// extra block indexes needed.
//
    if (res < index) {
    blks = read_indexes(inode.i_sb, index - res, start, offset);
    if (blks < 0)
    return (int) blks;
    if (block)
// block += blks;
    }
//
// Read length of block specified by index.
//
    res = squashfs_read_metadata(inode.i_sb, &size, start, offset,
    sizeof(size));
    if (res < 0)
    return res;
    return squashfs_block_size(size);
    }
#[no_mangle]
pub unsafe extern "C" fn read_blocklist(inode: *mut inode, index: c_int, block: *mut u64) -> c_int {
    static inline int read_blocklist(struct inode *inode, int index, u64 *block)
    {
    u64 start;
    int offset;
    return read_blocklist_ptrs(inode, index, &start, &offset, block);
    }
    static bool squashfs_fill_page(struct folio *folio,
    struct squashfs_cache_entry *buffer, size_t offset,
    size_t avail)
    {
    size_t copied;
    void *pageaddr;
    pageaddr = kmap_local_folio(folio, 0);
    copied = squashfs_copy_data(pageaddr, buffer, offset, avail);
    memset(pageaddr + copied, 0, PAGE_SIZE - copied);
    kunmap_local(pageaddr);
    flush_dcache_folio(folio);
    let mut copied: return = = avail;
    }
// Copy data into page cache
    void squashfs_copy_cache(struct folio *folio,
    struct squashfs_cache_entry *buffer, size_t bytes,
    size_t offset)
    {
    struct address_space *mapping = folio.mapping;
    struct inode *inode = mapping.host;
    struct squashfs_sb_info *msblk = inode.i_sb.s_fs_info;
    int i, mask = (1 << (msblk.block_log - PAGE_SHIFT)) - 1;
    let mut start_index: c_int = folio.index & ~mask, end_index = start_index | mask;
//
// Loop copying datablock into pages.  As the datablock likely covers
// many PAGE_SIZE pages (default block size is 128 KiB) explicitly
// grab the pages from the page cache, except for the page that we've
// been called to fill.
//
    for (i = start_index; i <= end_index && bytes > 0; i++,
    bytes -= PAGE_SIZE, offset += PAGE_SIZE) {
    struct folio *push_folio;
    let mut avail: usize = buffer ? min(bytes, PAGE_SIZE) : 0;
    let mut updated: bool = false;
    TRACE("bytes %zu, i %d, available_bytes %zu\n", bytes, i, avail);
    push_folio = (i == folio.index) ? folio :
    __filemap_get_folio(mapping, i,
    FGP_LOCK|FGP_CREAT|FGP_NOFS|FGP_NOWAIT,
    mapping_gfp_mask(mapping));
    if (IS_ERR(push_folio))
    continue;
    if (folio_test_uptodate(push_folio))
    goto skip_folio;
    updated = squashfs_fill_page(push_folio, buffer, offset, avail);
    skip_folio:
    folio_end_read(push_folio, updated);
    if (i != folio.index)
    folio_put(push_folio);
    }
    }
// Read datablock stored packed inside a fragment (tail-end packed block)
#[no_mangle]
unsafe extern "C" fn squashfs_readpage_fragment(folio: *mut folio, expected: c_int) -> c_int {
    static int squashfs_readpage_fragment(struct folio *folio, int expected)
    {
    struct inode *inode = folio.mapping.host;
    struct squashfs_cache_entry *buffer = squashfs_get_fragment(inode.i_sb,
    squashfs_i(inode).fragment_block,
    squashfs_i(inode).fragment_size);
    let mut res: c_int = buffer.error;
    if (res)
    ERROR("Unable to read page, block %llx, size %x\n",
    squashfs_i(inode).fragment_block,
    squashfs_i(inode).fragment_size);
    else
    squashfs_copy_cache(folio, buffer, expected,
    squashfs_i(inode).fragment_offset);
    squashfs_cache_put(buffer);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn squashfs_readpage_sparse(folio: *mut folio, expected: c_int) -> c_int {
    static int squashfs_readpage_sparse(struct folio *folio, int expected)
    {
    squashfs_copy_cache(folio, core::ptr::null_mut(), expected, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn squashfs_read_folio(file: *mut file, folio: *mut folio) -> c_int {
    static int squashfs_read_folio(struct file *file, struct folio *folio)
    {
    struct inode *inode = folio.mapping.host;
    struct squashfs_sb_info *msblk = inode.i_sb.s_fs_info;
    let mut index: c_int = folio.index >> (msblk.block_log - PAGE_SHIFT);
    let mut file_end: c_int = i_size_read(inode) >> msblk.block_log;
    int expected = index == file_end ?
    (i_size_read(inode) & (msblk.block_size - 1)) :
    msblk.block_size;
    let mut res: c_int = 0;
    TRACE("Entered squashfs_readpage, page index %lx, start block %llx\n",
    folio.index, squashfs_i(inode).start);
    if (folio.index >= ((i_size_read(inode) + PAGE_SIZE - 1) >>
    PAGE_SHIFT))
    goto out;
    if (index < file_end || squashfs_i(inode).fragment_block ==
    SQUASHFS_INVALID_BLK) {
    let mut block: u64 = 0;
    res = read_blocklist(inode, index, &block);
    if (res < 0)
    goto out;
    if (res == 0)
    res = squashfs_readpage_sparse(folio, expected);
    else
    res = squashfs_readpage_block(folio, block, res, expected);
    } else
    res = squashfs_readpage_fragment(folio, expected);
    if (!res)
    return 0;
    out:
    folio_zero_segment(folio, 0, folio_size(folio));
    folio_end_read(folio, res == 0);
    return res;
    }
    static int squashfs_readahead_fragment(struct inode *inode, struct page **page,
    unsigned int pages, unsigned int expected, loff_t start)
    {
    struct squashfs_cache_entry *buffer = squashfs_get_fragment(inode.i_sb,
    squashfs_i(inode).fragment_block,
    squashfs_i(inode).fragment_size);
    struct squashfs_sb_info *msblk = inode.i_sb.s_fs_info;
    int i, bytes, copied;
    struct squashfs_page_actor *actor;
    unsigned int offset;
    void *addr;
    struct page *last_page;
    if (buffer.error)
    goto out;
    actor = squashfs_page_actor_init_special(msblk, page, pages,
    expected, start);
    if (!actor)
    goto out;
    squashfs_actor_nobuff(actor);
    addr = squashfs_first_page(actor);
    for (copied = offset = 0; offset < expected; offset += PAGE_SIZE) {
    let mut avail: c_int = min_t(int, expected - offset, PAGE_SIZE);
    if (!IS_ERR(addr)) {
    bytes = squashfs_copy_data(addr, buffer, offset +
    squashfs_i(inode).fragment_offset, avail);
    if (bytes != avail)
    goto failed;
    }
    copied += avail;
    addr = squashfs_next_page(actor);
    }
    last_page = squashfs_page_actor_free(actor);
    if (copied == expected && !IS_ERR(last_page)) {
// Last page (if present) may have trailing bytes not filled
    bytes = copied % PAGE_SIZE;
    if (bytes && last_page)
    memzero_page(last_page, bytes, PAGE_SIZE - bytes);
    for (i = 0; i < pages; i++) {
    flush_dcache_page(page[i]);
    SetPageUptodate(page[i]);
    }
    }
    for (i = 0; i < pages; i++) {
    unlock_page(page[i]);
    put_page(page[i]);
    }
    squashfs_cache_put(buffer);
    return 0;
    failed:
    squashfs_page_actor_free(actor);
    out:
    squashfs_cache_put(buffer);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn squashfs_readahead(ractl: *mut readahead_control) {
    static void squashfs_readahead(struct readahead_control *ractl)
    {
    struct inode *inode = ractl.mapping.host;
    struct squashfs_sb_info *msblk = inode.i_sb.s_fs_info;
    let mut mask: usize = (1UL << msblk.block_log) - 1;
    let mut shift: c_ushort = msblk.block_log - PAGE_SHIFT;
    let mut start: loff_t = readahead_pos(ractl) & ~mask;
    let mut len: usize = readahead_length(ractl) + readahead_pos(ractl) - start;
    struct squashfs_page_actor *actor;
    let mut nr_pages: c_uint = 0;
    struct page **pages;
    int i;
    let mut file_end: loff_t = i_size_read(inode) >> msblk.block_log;
    let mut max_pages: c_uint = 1UL << shift;
    readahead_expand(ractl, start, (len | mask) + 1);
    pages = kmalloc_array(max_pages, sizeof(void *), GFP_KERNEL);
    if (!pages)
    return;
    for (;;) {
    int res, bsize;
    let mut block: u64 = 0;
    unsigned int expected;
    struct page *last_page;
    expected = start >> msblk.block_log == file_end ?
    (i_size_read(inode) & (msblk.block_size - 1)) :
    msblk.block_size;
    max_pages = (expected + PAGE_SIZE - 1) >> PAGE_SHIFT;
    nr_pages = __readahead_batch(ractl, pages, max_pages);
    if (!nr_pages)
    break;
    if (readahead_pos(ractl) >= i_size_read(inode))
    goto skip_pages;
    if (start >> msblk.block_log == file_end &&
    squashfs_i(inode).fragment_block != SQUASHFS_INVALID_BLK) {
    res = squashfs_readahead_fragment(inode, pages,
    nr_pages, expected, start);
    if (res)
    goto skip_pages;
    continue;
    }
    bsize = read_blocklist(inode, start >> msblk.block_log, &block);
    if (bsize == 0)
    goto skip_pages;
    actor = squashfs_page_actor_init_special(msblk, pages, nr_pages,
    expected, start);
    if (!actor)
    goto skip_pages;
    res = squashfs_read_data(inode.i_sb, block, bsize, core::ptr::null_mut(), actor);
    last_page = squashfs_page_actor_free(actor);
    if (res == expected && !IS_ERR(last_page)) {
    int bytes;
// Last page (if present) may have trailing bytes not filled
    bytes = res % PAGE_SIZE;
    if (start >> msblk.block_log == file_end && bytes && last_page)
    memzero_page(last_page, bytes,
    PAGE_SIZE - bytes);
    for (i = 0; i < nr_pages; i++) {
    flush_dcache_page(pages[i]);
    SetPageUptodate(pages[i]);
    }
    }
    for (i = 0; i < nr_pages; i++) {
    unlock_page(pages[i]);
    put_page(pages[i]);
    }
    start += readahead_batch_length(ractl);
    }
    kfree(pages);
    return;
    skip_pages:
    for (i = 0; i < nr_pages; i++) {
    unlock_page(pages[i]);
    put_page(pages[i]);
    }
    kfree(pages);
    }
#[no_mangle]
unsafe extern "C" fn seek_hole_data(file: *mut file, offset: loff_t, whence: c_int) -> loff_t {
    static loff_t seek_hole_data(struct file *file, loff_t offset, int whence)
    {
    struct inode *inode = file.f_mapping.host;
    struct super_block *sb = inode.i_sb;
    struct squashfs_sb_info *msblk = sb.s_fs_info;
    u64 start, index = offset >> msblk.block_log;
    let mut file_end: u64 = (i_size_read(inode) + msblk.block_size - 1) >> msblk.block_log;
    int s_offset, length;
    __le32 *blist = core::ptr::null_mut();
// reject offset if negative or beyond file end
    if ((unsigned long long)offset >= i_size_read(inode))
    return -ENXIO;
// is offset within tailend and is tailend packed into a fragment?
    if (index + 1 == file_end &&
    squashfs_i(inode).fragment_block != SQUASHFS_INVALID_BLK) {
    if (whence == SEEK_DATA)
    return offset;
// there is an implicit hole at the end of any file
    return i_size_read(inode);
    }
    length = read_blocklist_ptrs(inode, index, &start, &s_offset, core::ptr::null_mut());
    if (length < 0)
    return length;
// nothing more to do if offset matches desired whence value
    if ((length == 0 && whence == SEEK_HOLE) ||
    (length && whence == SEEK_DATA))
    return offset;
// skip scanning forwards if we're at file end
    if (++ index == file_end)
    goto not_found;
    blist = kmalloc(SQUASHFS_SCAN_INDEXES << 2, GFP_KERNEL);
    if (blist == core::ptr::null_mut()) {
    ERROR("%s: Failed to allocate block_list\n", __func__);
    return -ENOMEM;
    }
    while (index < file_end) {
    int i, indexes = min(file_end - index, SQUASHFS_SCAN_INDEXES);
    offset = squashfs_read_metadata(sb, blist, &start, &s_offset, indexes << 2);
    if (offset < 0)
    goto finished;
    for (i = 0; i < indexes; i++) {
    length = squashfs_block_size(blist[i]);
    if (length < 0) {
    offset = length;
    goto finished;
    }
// does this block match desired whence value?
    if ((length == 0 && whence == SEEK_HOLE) ||
    (length && whence == SEEK_DATA)) {
    offset = (index + i) << msblk.block_log;
    goto finished;
    }
    }
    index += indexes;
    }
    not_found:
// whence value determines what happens
    if (whence == SEEK_DATA)
    offset = -ENXIO;
    else
// there is an implicit hole at the end of any file
    offset = i_size_read(inode);
    finished:
    kfree(blist);
    return offset;
    }
#[no_mangle]
unsafe extern "C" fn squashfs_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t {
    static loff_t squashfs_llseek(struct file *file, loff_t offset, int whence)
    {
    struct inode *inode = file.f_mapping.host;
    switch (whence) {
    default:
    return generic_file_llseek(file, offset, whence);
    case SEEK_DATA:
    case SEEK_HOLE:
    offset = seek_hole_data(file, offset, whence);
    break;
    }
    if (offset < 0)
    return offset;
    return vfs_setpos(file, offset, inode.i_sb.s_maxbytes);
    }
    const struct address_space_operations squashfs_aops = {
    .read_folio = squashfs_read_folio,
    .readahead = squashfs_readahead
    };
    const struct file_operations squashfs_file_operations = {
    .llseek		= squashfs_llseek,
    .read_iter	= generic_file_read_iter,
    .mmap_prepare	= generic_file_readonly_mmap_prepare,
    .splice_read	= filemap_splice_read,
    .setlease	= generic_setlease,
    };
