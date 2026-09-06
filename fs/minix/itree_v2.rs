//! Automatically rewritten from C to Rust
//! Source: fs/minix/itree_v2.c
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

    enum {DIRECT = 7, DEPTH = 4};	/* Have triple indirect */
    typedef u32 block_t;	/* 32 bit, host order */
#[no_mangle]
pub unsafe extern "C" fn block_to_cpu(n: block_t) -> c_ulong {
    static inline unsigned long block_to_cpu(block_t n)
    {
    return n;
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_to_block(n: c_ulong) -> block_t {
    static inline block_t cpu_to_block(unsigned long n)
    {
    return n;
    }
    static inline block_t *i_data(struct inode *inode)
    {
    return (block_t *)minix_i(inode).u.i2_data;
    }
pub const DIRCOUNT: c_int = 7;

#[no_mangle]
unsafe extern "C" fn block_to_path(inode: *mut *mut inode, block: c_long, offsets[DEPTH]: c_int) -> c_int {
    static int block_to_path(struct inode * inode, long block, int offsets[DEPTH])
    {
    let mut n: c_int = 0;
    struct super_block *sb = inode.i_sb;
    if (block < 0) {
    printk("MINIX-fs: block_to_path: block %ld < 0 on dev %pg\n",
    block, sb.s_bdev);
    return 0;
    }
    if ((u64)block * (u64)sb.s_blocksize >= sb.s_maxbytes)
    return 0;
    if (block < DIRCOUNT) {
    offsets[n++] = block;
    } else if ((block -= DIRCOUNT) < INDIRCOUNT(sb)) {
    offsets[n++] = DIRCOUNT;
    offsets[n++] = block;
    } else if ((block -= INDIRCOUNT(sb)) < INDIRCOUNT(sb) * INDIRCOUNT(sb)) {
    offsets[n++] = DIRCOUNT + 1;
    offsets[n++] = block / INDIRCOUNT(sb);
    offsets[n++] = block % INDIRCOUNT(sb);
    } else {
    block -= INDIRCOUNT(sb) * INDIRCOUNT(sb);
    offsets[n++] = DIRCOUNT + 2;
    offsets[n++] = (block / INDIRCOUNT(sb)) / INDIRCOUNT(sb);
    offsets[n++] = (block / INDIRCOUNT(sb)) % INDIRCOUNT(sb);
    offsets[n++] = block % INDIRCOUNT(sb);
    }
    return n;
    }

    int V2_minix_get_block(struct inode * inode, long block,
    struct buffer_head *bh_result, int create)
    {
    return get_block(inode, block, bh_result, create);
    }
#[no_mangle]
pub unsafe extern "C" fn V2_minix_truncate(inode: *mut *mut inode) {
    void V2_minix_truncate(struct inode * inode)
    {
    truncate(inode);
    }
#[no_mangle]
pub unsafe extern "C" fn V2_minix_blocks(size: loff_t, sb: *mut super_block) -> unsigned {
    unsigned V2_minix_blocks(loff_t size, struct super_block *sb)
    {
    return nblocks(size, sb);
    }
