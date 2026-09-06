//! Automatically rewritten from C to Rust
//! Source: fs/qnx4/dir.c
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
// 20-06-1998 by Frank Denis : Linux 2.1.99+ & dcache support.
//

#[no_mangle]
unsafe extern "C" fn qnx4_readdir(file: *mut file, ctx: *mut dir_context) -> c_int {
    static int qnx4_readdir(struct file *file, struct dir_context *ctx)
    {
    struct inode *inode = file_inode(file);
    unsigned int offset;
    struct buffer_head *bh;
    unsigned long blknum;
    int ix, ino;
    int size;
    QNX4DEBUG((KERN_INFO "qnx4_readdir:i_size = %ld\n", (long) inode.i_size));
    QNX4DEBUG((KERN_INFO "pos                 = %ld\n", (long) ctx.pos));
    while (ctx.pos < inode.i_size) {
    blknum = qnx4_block_map(inode, ctx.pos >> QNX4_BLOCK_SIZE_BITS);
    bh = sb_bread(inode.i_sb, blknum);
    if (bh == core::ptr::null_mut()) {
    printk(KERN_ERR "qnx4_readdir: bread failed (%ld)\n", blknum);
    return 0;
    }
    ix = (ctx.pos >> QNX4_DIR_ENTRY_SIZE_BITS) % QNX4_INODES_PER_BLOCK;
    for (; ix < QNX4_INODES_PER_BLOCK; ix++, ctx.pos += QNX4_DIR_ENTRY_SIZE) {
    union qnx4_directory_entry *de;
    const char *fname;
    offset = ix * QNX4_DIR_ENTRY_SIZE;
    de = (union qnx4_directory_entry *) (bh.b_data + offset);
    fname = get_entry_fname(de, &size);
    if (!fname)
    continue;
    if (!(de.de_status & QNX4_FILE_LINK)) {
    ino = blknum * QNX4_INODES_PER_BLOCK + ix - 1;
    } else {
    ino = ( le32_to_cpu(de.link.dl_inode_blk) - 1 ) *
    QNX4_INODES_PER_BLOCK +
    de.link.dl_inode_ndx;
    }
    QNX4DEBUG((KERN_INFO "qnx4_readdir:%.*s\n", size, fname));
    if (!dir_emit(ctx, fname, size, ino, DT_UNKNOWN)) {
    brelse(bh);
    return 0;
    }
    }
    brelse(bh);
    }
    return 0;
    }
    const struct file_operations qnx4_dir_operations =
    {
    .llseek		= generic_file_llseek,
    .read		= generic_read_dir,
    .iterate_shared	= qnx4_readdir,
    .fsync		= simple_fsync,
    .setlease	= generic_setlease,
    };
    const struct inode_operations qnx4_dir_inode_operations =
    {
    .lookup		= qnx4_lookup,
    };
