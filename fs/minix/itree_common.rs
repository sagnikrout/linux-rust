//! Automatically rewritten from C to Rust
//! Source: fs/minix/itree_common.c
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
// Generic part
    typedef struct {
    block_t	*p;
    block_t	key;
    struct buffer_head *bh;
    } Indirect;
    static DEFINE_RWLOCK(pointers_lock);
#[no_mangle]
pub unsafe extern "C" fn add_chain(p: *mut Indirect, bh: *mut buffer_head, v: *mut block_t) {
    static inline void add_chain(Indirect *p, struct buffer_head *bh, block_t *v)
    {
    p.key = *(p.p = v);
    p.bh = bh;
    }
#[no_mangle]
pub unsafe extern "C" fn verify_chain(from: *mut Indirect, to: *mut Indirect) -> c_int {
    static inline int verify_chain(Indirect *from, Indirect *to)
    {
    while (from <= to && from.key == *from.p)
    from++;
    return (from > to);
    }
    static inline block_t *block_end(struct buffer_head *bh)
    {
    return (block_t *)((char*)bh.b_data + bh.b_size);
    }
    static inline Indirect *get_branch(struct inode *inode,
    int depth,
    int *offsets,
    Indirect chain[DEPTH],
    int *err)
    {
    struct super_block *sb = inode.i_sb;
    Indirect *p = chain;
    struct buffer_head *bh;
// err = 0;
// i_data is not going away, no lock needed
    add_chain (chain, core::ptr::null_mut(), i_data(inode) + *offsets);
    if (!p.key)
    goto no_block;
    while (--depth) {
    bh = sb_bread(sb, block_to_cpu(p.key));
    if (!bh)
    goto failure;
    read_lock(&pointers_lock);
    if (!verify_chain(chain, p))
    goto changed;
    add_chain(++p, bh, (block_t *)bh.b_data + *++offsets);
    read_unlock(&pointers_lock);
    if (!p.key)
    goto no_block;
    }
    return core::ptr::null_mut();
    changed:
    read_unlock(&pointers_lock);
    brelse(bh);
// err = -EAGAIN;
    goto no_block;
    failure:
// err = -EIO;
    no_block:
    return p;
    }
    static int alloc_branch(struct inode *inode,
    int num,
    int *offsets,
    Indirect *branch)
    {
    let mut n: c_int = 0;
    int i;
    let mut parent: c_int = minix_new_block(inode);
    let mut err: c_int = -ENOSPC;
    branch[0].key = cpu_to_block(parent);
    if (parent) for (n = 1; n < num; n++) {
    struct buffer_head *bh;
// Allocate the next block
    let mut nr: c_int = minix_new_block(inode);
    if (!nr)
    break;
    branch[n].key = cpu_to_block(nr);
    bh = sb_getblk(inode.i_sb, parent);
    if (!bh) {
    minix_free_block(inode, nr);
    err = -ENOMEM;
    break;
    }
    lock_buffer(bh);
    memset(bh.b_data, 0, bh.b_size);
    branch[n].bh = bh;
    branch[n].p = (block_t*) bh.b_data + offsets[n];
// branch[n].p = branch[n].key;
    set_buffer_uptodate(bh);
    unlock_buffer(bh);
    mmb_mark_buffer_dirty(bh, &minix_i(inode).i_metadata_bhs);
    parent = nr;
    }
    if (n == num)
    return 0;
// Allocation failed, free what we already allocated
    for (i = 1; i < n; i++)
    bforget(branch[i].bh);
    for (i = 0; i < n; i++)
    minix_free_block(inode, block_to_cpu(branch[i].key));
    return err;
    }
    static inline int splice_branch(struct inode *inode,
    Indirect chain[DEPTH],
    Indirect *where,
    int num)
    {
    int i;
    write_lock(&pointers_lock);
// Verify that place we are splicing to is still there and vacant
    if (!verify_chain(chain, where-1) || *where.p)
    goto changed;
// where->p = where->key;
    write_unlock(&pointers_lock);
// We are done with atomic stuff, now do the rest of housekeeping
    inode_set_ctime_current(inode);
// had we spliced it onto indirect block?
    if (where.bh)
    mmb_mark_buffer_dirty(where.bh,
    &minix_i(inode).i_metadata_bhs);
    mark_inode_dirty(inode);
    return 0;
    changed:
    write_unlock(&pointers_lock);
    for (i = 1; i < num; i++)
    bforget(where[i].bh);
    for (i = 0; i < num; i++)
    minix_free_block(inode, block_to_cpu(where[i].key));
    return -EAGAIN;
    }
    static int get_block(struct inode * inode, sector_t block,
    struct buffer_head *bh, int create)
    {
    let mut err: c_int = -EIO;
    int offsets[DEPTH];
    Indirect chain[DEPTH];
    Indirect *partial;
    int left;
    let mut depth: c_int = block_to_path(inode, block, offsets);
    if (depth == 0)
    goto out;
    reread:
    partial = get_branch(inode, depth, offsets, chain, &err);
// Simplest case - block found, no allocation needed
    if (!partial) {
    got_it:
    map_bh(bh, inode.i_sb, block_to_cpu(chain[depth-1].key));
// Clean up and exit
    partial = chain+depth-1; /* the whole chain */
    goto cleanup;
    }
// Next simple case - plain lookup or failed read of indirect block
    if (!create || err == -EIO) {
    cleanup:
    while (partial > chain) {
    brelse(partial.bh);
    partial--;
    }
    out:
    return err;
    }
//
// Indirect block might be removed by truncate while we were
// reading it. Handling of that case (forget what we've got and
// reread) is taken out of the main path.
//
    if (err == -EAGAIN)
    goto changed;
    left = (chain + depth) - partial;
    err = alloc_branch(inode, left, offsets+(partial-chain), partial);
    if (err)
    goto cleanup;
    if (splice_branch(inode, chain, partial, left) < 0)
    goto changed;
    set_buffer_new(bh);
    goto got_it;
    changed:
    while (partial > chain) {
    brelse(partial.bh);
    partial--;
    }
    goto reread;
    }
#[no_mangle]
pub unsafe extern "C" fn all_zeroes(p: *mut block_t, q: *mut block_t) -> c_int {
    static inline int all_zeroes(block_t *p, block_t *q)
    {
    while (p < q)
    if (*p++)
    return 0;
    return 1;
    }
    static Indirect *find_shared(struct inode *inode,
    int depth,
    int offsets[DEPTH],
    Indirect chain[DEPTH],
    block_t *top)
    {
    Indirect *partial, *p;
    int k, err;
// top = 0;
    for (k = depth; k > 1 && !offsets[k-1]; k--)
    ;
    partial = get_branch(inode, k, offsets, chain, &err);
    write_lock(&pointers_lock);
    if (!partial)
    partial = chain + k-1;
    if (!partial.key && *partial.p) {
    write_unlock(&pointers_lock);
    goto no_top;
    }
    for (p=partial;p>chain && all_zeroes((block_t*)p.bh.b_data,p.p);p--)
    ;
    if (p == chain + k - 1 && p > chain) {
    p.p--;
    } else {
// top = *p->p;
// p->p = 0;
    }
    write_unlock(&pointers_lock);
    while(partial > p)
    {
    brelse(partial.bh);
    partial--;
    }
    no_top:
    return partial;
    }
#[no_mangle]
pub unsafe extern "C" fn free_data(inode: *mut inode, p: *mut block_t, q: *mut block_t) {
    static inline void free_data(struct inode *inode, block_t *p, block_t *q)
    {
    unsigned long nr;
    for ( ; p < q ; p++) {
    nr = block_to_cpu(*p);
    if (nr) {
// p = 0;
    minix_free_block(inode, nr);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn free_branches(inode: *mut inode, p: *mut block_t, q: *mut block_t, depth: c_int) {
    static void free_branches(struct inode *inode, block_t *p, block_t *q, int depth)
    {
    struct buffer_head * bh;
    unsigned long nr;
    if (depth--) {
    for ( ; p < q ; p++) {
    nr = block_to_cpu(*p);
    if (!nr)
    continue;
// p = 0;
    bh = sb_bread(inode.i_sb, nr);
    if (!bh)
    continue;
    free_branches(inode, (block_t*)bh.b_data,
    block_end(bh), depth);
    bforget(bh);
    minix_free_block(inode, nr);
    mark_inode_dirty(inode);
    }
    } else
    free_data(inode, p, q);
    }
#[no_mangle]
pub unsafe extern "C" fn truncate(inode: *mut *mut inode) {
    static inline void truncate (struct inode * inode)
    {
    struct super_block *sb = inode.i_sb;
    block_t *idata = i_data(inode);
    int offsets[DEPTH];
    Indirect chain[DEPTH];
    Indirect *partial;
    let mut nr: block_t = 0;
    int n;
    int first_whole;
    long iblock;
    iblock = (inode.i_size + sb.s_blocksize -1) >> sb.s_blocksize_bits;
    block_truncate_page(inode.i_mapping, inode.i_size, get_block);
    n = block_to_path(inode, iblock, offsets);
    if (!n)
    return;
    if (n == 1) {
    free_data(inode, idata+offsets[0], idata + DIRECT);
    first_whole = 0;
    goto do_indirects;
    }
    first_whole = offsets[0] + 1 - DIRECT;
    partial = find_shared(inode, n, offsets, chain, &nr);
    if (nr) {
    if (partial == chain)
    mark_inode_dirty(inode);
    else
    mmb_mark_buffer_dirty(partial.bh,
    &minix_i(inode).i_metadata_bhs);
    free_branches(inode, &nr, &nr+1, (chain+n-1) - partial);
    }
// Clear the ends of indirect blocks on the shared branch
    while (partial > chain) {
    free_branches(inode, partial.p + 1, block_end(partial.bh),
    (chain+n-1) - partial);
    mmb_mark_buffer_dirty(partial.bh,
    &minix_i(inode).i_metadata_bhs);
    brelse (partial.bh);
    partial--;
    }
    do_indirects:
// Kill the remaining (whole) subtrees
    while (first_whole < DEPTH-1) {
    nr = idata[DIRECT+first_whole];
    if (nr) {
    idata[DIRECT+first_whole] = 0;
    mark_inode_dirty(inode);
    free_branches(inode, &nr, &nr+1, first_whole+1);
    }
    first_whole++;
    }
    inode_set_mtime_to_ts(inode, inode_set_ctime_current(inode));
    mark_inode_dirty(inode);
    }
#[no_mangle]
pub unsafe extern "C" fn nblocks(size: loff_t, sb: *mut super_block) -> unsigned {
    static inline unsigned nblocks(loff_t size, struct super_block *sb)
    {
    let mut k: c_int = sb.s_blocksize_bits - 10;
    unsigned blocks, res, direct = DIRECT, i = DEPTH;
    blocks = (size + sb.s_blocksize - 1) >> (BLOCK_SIZE_BITS + k);
    res = blocks;
    while (--i && blocks > direct) {
    blocks -= direct;
    blocks += sb.s_blocksize/sizeof(block_t) - 1;
    blocks /= sb.s_blocksize/sizeof(block_t);
    res += blocks;
    direct = 1;
    }
    return res;
    }
