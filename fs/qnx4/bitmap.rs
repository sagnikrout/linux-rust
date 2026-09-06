//! Automatically rewritten from C to Rust
//! Source: fs/qnx4/bitmap.c
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
// QNX4 file system, Linux implementation.
//
// Version : 0.2.1
//
// Using parts of the xiafs filesystem.
//
// History :
//
// 28-05-1998 by Richard Frowijn : first release.
// 20-06-1998 by Frank Denis : basic optimisations.
// 25-06-1998 by Frank Denis : qnx4_is_free, qnx4_set_bitmap, qnx4_bmap .
// 28-06-1998 by Frank Denis : qnx4_free_inode (to be fixed) .
//

#[no_mangle]
pub unsafe extern "C" fn qnx4_count_free_blocks(sb: *mut super_block) -> c_ulong {
    unsigned long qnx4_count_free_blocks(struct super_block *sb)
    {
    let mut start: c_int = le32_to_cpu(qnx4_sb(sb).BitMap.di_first_xtnt.xtnt_blk) - 1;
    let mut total: c_int = 0;
    let mut total_free: c_int = 0;
    let mut offset: c_int = 0;
    let mut size: c_int = le32_to_cpu(qnx4_sb(sb).BitMap.di_size);
    struct buffer_head *bh;
    while (total < size) {
    let mut bytes: c_int = min(size - total, QNX4_BLOCK_SIZE);
    if ((bh = sb_bread(sb, start + offset)) == core::ptr::null_mut()) {
    printk(KERN_ERR "qnx4: I/O error in counting free blocks\n");
    break;
    }
    total_free += bytes * BITS_PER_BYTE -
    memweight(bh.b_data, bytes);
    brelse(bh);
    total += bytes;
    offset++;
    }
    return total_free;
    }
