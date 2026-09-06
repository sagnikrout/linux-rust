//! Automatically rewritten from C to Rust
//! Source: fs/qnx4/namei.c
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
// 01-06-1998 by Richard Frowijn : first release.
// 21-06-1998 by Frank Denis : dcache support, fixed error codes.
// 04-07-1998 by Frank Denis : first step for rmdir/unlink.
//

//
// check if the filename is correct. For some obscure reason, qnx writes a
// new file twice in the directory entry, first with all possible options at 0
// and for a second time the way it is, they want us not to access the qnx
// filesystem when whe are using linux.
//
    static int qnx4_match(int len, const char *name,
    struct buffer_head *bh, unsigned long *offset)
    {
    union qnx4_directory_entry *de;
    const char *fname;
    int fnamelen;
    if (bh == core::ptr::null_mut()) {
    printk(KERN_WARNING "qnx4: matching unassigned buffer !\n");
    return 0;
    }
    de = (union qnx4_directory_entry *) (bh.b_data + *offset);
// offset += QNX4_DIR_ENTRY_SIZE;
    fname = get_entry_fname(de, &fnamelen);
    if (!fname || len != fnamelen)
    return 0;
    if (strncmp(name, fname, len) == 0)
    return 1;
    return 0;
    }
    static struct buffer_head *qnx4_find_entry(int len, struct inode *dir,
    const char *name, struct qnx4_inode_entry **res_dir, int *ino)
    {
    unsigned long block, offset, blkofs;
    struct buffer_head *bh;
// res_dir = NULL;
    bh = core::ptr::null_mut();
    block = offset = blkofs = 0;
    while (blkofs * QNX4_BLOCK_SIZE + offset < dir.i_size) {
    if (!bh) {
    block = qnx4_block_map(dir, blkofs);
    if (block)
    bh = sb_bread(dir.i_sb, block);
    if (!bh) {
    blkofs++;
    continue;
    }
    }
// res_dir = (struct qnx4_inode_entry *) (bh->b_data + offset);
    if (qnx4_match(len, name, bh, &offset)) {
// ino = block * QNX4_INODES_PER_BLOCK +
    (offset / QNX4_DIR_ENTRY_SIZE) - 1;
    return bh;
    }
    if (offset < bh.b_size) {
    continue;
    }
    brelse(bh);
    bh = core::ptr::null_mut();
    offset = 0;
    blkofs++;
    }
    brelse(bh);
// res_dir = NULL;
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn qnx4_lookup(dir: *mut inode, dentry: *mut dentry, flags: c_uint) -> *mut dentry {
    struct dentry * qnx4_lookup(struct inode *dir, struct dentry *dentry, unsigned int flags)
    {
    int ino;
    struct qnx4_inode_entry *de;
    struct qnx4_link_info *lnk;
    struct buffer_head *bh;
    const char *name = dentry.d_name.name;
    let mut len: c_int = dentry.d_name.len;
    struct inode *foundinode = core::ptr::null_mut();
    if (!(bh = qnx4_find_entry(len, dir, name, &de, &ino)))
    goto out;
// The entry is linked, let's get the real info
    if ((de.di_status & QNX4_FILE_LINK) == QNX4_FILE_LINK) {
    lnk = (struct qnx4_link_info *) de;
    ino = (le32_to_cpu(lnk.dl_inode_blk) - 1) *
    QNX4_INODES_PER_BLOCK +
    lnk.dl_inode_ndx;
    }
    brelse(bh);
    foundinode = qnx4_iget(dir.i_sb, ino);
    if (IS_ERR(foundinode))
    QNX4DEBUG((KERN_ERR "qnx4: lookup.iget . error %ld\n",
    PTR_ERR(foundinode)));
    out:
    return d_splice_alias(foundinode, dentry);
    }
