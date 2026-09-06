//! Automatically rewritten from C to Rust
//! Source: fs/udf/directory.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// directory.c
//
// PURPOSE
// Directory related functions
//

#[no_mangle]
unsafe extern "C" fn udf_verify_fi(iter: *mut udf_fileident_iter) -> c_int {
    static int udf_verify_fi(struct udf_fileident_iter *iter)
    {
    unsigned int len;
    if (iter.fi.descTag.tagIdent != cpu_to_le16(TAG_IDENT_FID)) {
    udf_err(iter.dir.i_sb,
    "directory (ino %llu) has entry at pos %llu with incorrect tag %x\n",
    iter.dir.i_ino, (unsigned long long)iter.pos,
    le16_to_cpu(iter.fi.descTag.tagIdent));
    return -EFSCORRUPTED;
    }
    len = udf_dir_entry_len(&iter.fi);
    if (le16_to_cpu(iter.fi.lengthOfImpUse) & 3) {
    udf_err(iter.dir.i_sb,
    "directory (ino %llu) has entry at pos %llu with unaligned length of impUse field\n",
    iter.dir.i_ino, (unsigned long long)iter.pos);
    return -EFSCORRUPTED;
    }
//
// This is in fact allowed by the spec due to long impUse field but
// we don't support it. If there is real media with this large impUse
// field, support can be added.
//
    if (len > 1 << iter.dir.i_blkbits) {
    udf_err(iter.dir.i_sb,
    "directory (ino %llu) has too big (%u) entry at pos %llu\n",
    iter.dir.i_ino, len, (unsigned long long)iter.pos);
    return -EFSCORRUPTED;
    }
    if (iter.pos + len > iter.dir.i_size) {
    udf_err(iter.dir.i_sb,
    "directory (ino %llu) has entry past directory size at pos %llu\n",
    iter.dir.i_ino, (unsigned long long)iter.pos);
    return -EFSCORRUPTED;
    }
    if (udf_dir_entry_len(&iter.fi) !=
    sizeof(struct tag) + le16_to_cpu(iter.fi.descTag.descCRCLength)) {
    udf_err(iter.dir.i_sb,
    "directory (ino %llu) has entry where CRC length (%u) does not match entry length (%u)\n",
    iter.dir.i_ino,
    (unsigned)le16_to_cpu(iter.fi.descTag.descCRCLength),
    (unsigned)(udf_dir_entry_len(&iter.fi) -
    sizeof(struct tag)));
    return -EFSCORRUPTED;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn udf_copy_fi(iter: *mut udf_fileident_iter) -> c_int {
    static int udf_copy_fi(struct udf_fileident_iter *iter)
    {
    struct udf_inode_info *iinfo = UDF_I(iter.dir);
    let mut blksize: u32 = 1 << iter.dir.i_blkbits;
    u32 off, len, nameoff;
    int err;
// Skip copying when we are at EOF
    if (iter.pos >= iter.dir.i_size) {
    iter.name = core::ptr::null_mut();
    return 0;
    }
    if (iter.dir.i_size < iter.pos + sizeof(struct fileIdentDesc)) {
    udf_err(iter.dir.i_sb,
    "directory (ino %llu) has entry straddling EOF\n",
    iter.dir.i_ino);
    return -EFSCORRUPTED;
    }
    if (iinfo.i_alloc_type == ICBTAG_FLAG_AD_IN_ICB) {
    memcpy(&iter.fi, iinfo.i_data + iinfo.i_lenEAttr + iter.pos,
    sizeof(struct fileIdentDesc));
    err = udf_verify_fi(iter);
    if (err < 0)
    return err;
    iter.name = iinfo.i_data + iinfo.i_lenEAttr + iter.pos +
    sizeof(struct fileIdentDesc) +
    le16_to_cpu(iter.fi.lengthOfImpUse);
    return 0;
    }
    off = iter.pos & (blksize - 1);
    len = min_t(u32, sizeof(struct fileIdentDesc), blksize - off);
    memcpy(&iter.fi, iter.bh[0].b_data + off, len);
    if (len < sizeof(struct fileIdentDesc))
    memcpy((char *)(&iter.fi) + len, iter.bh[1].b_data,
    sizeof(struct fileIdentDesc) - len);
    err = udf_verify_fi(iter);
    if (err < 0)
    return err;
// Handle directory entry name
    nameoff = off + sizeof(struct fileIdentDesc) +
    le16_to_cpu(iter.fi.lengthOfImpUse);
    if (off + udf_dir_entry_len(&iter.fi) <= blksize) {
    iter.name = iter.bh[0].b_data + nameoff;
    } else if (nameoff >= blksize) {
    iter.name = iter.bh[1].b_data + (nameoff - blksize);
    } else {
    iter.name = iter.namebuf;
    len = blksize - nameoff;
    memcpy(iter.name, iter.bh[0].b_data + nameoff, len);
    memcpy(iter.name + len, iter.bh[1].b_data,
    iter.fi.lengthFileIdent - len);
    }
    return 0;
    }
// Readahead 8k once we are at 8k boundary
#[no_mangle]
unsafe extern "C" fn udf_readahead_dir(iter: *mut udf_fileident_iter) {
    static void udf_readahead_dir(struct udf_fileident_iter *iter)
    {
    let mut ralen: c_uint = 16 >> (iter.dir.i_blkbits - 9);
    struct buffer_head *tmp, *bha[16];
    int i, num;
    udf_pblk_t blk;
    if (iter.loffset & (ralen - 1))
    return;
    if (iter.loffset + ralen > (iter.elen >> iter.dir.i_blkbits))
    ralen = (iter.elen >> iter.dir.i_blkbits) - iter.loffset;
    num = 0;
    for (i = 0; i < ralen; i++) {
    blk = udf_get_lb_pblock(iter.dir.i_sb, &iter.eloc,
    iter.loffset + i);
    tmp = sb_getblk(iter.dir.i_sb, blk);
    if (tmp && !buffer_uptodate(tmp) && !buffer_locked(tmp))
    bha[num++] = tmp;
    else
    brelse(tmp);
    }
    if (num) {
    bh_readahead_batch(num, bha, REQ_RAHEAD);
    for (i = 0; i < num; i++)
    brelse(bha[i]);
    }
    }
    static struct buffer_head *udf_fiiter_bread_blk(struct udf_fileident_iter *iter)
    {
    udf_pblk_t blk;
    udf_readahead_dir(iter);
    blk = udf_get_lb_pblock(iter.dir.i_sb, &iter.eloc, iter.loffset);
    return sb_bread(iter.dir.i_sb, blk);
    }
//
// Updates loffset to point to next directory block; eloc, elen & epos are
// updated if we need to traverse to the next extent as well.
//
#[no_mangle]
unsafe extern "C" fn udf_fiiter_advance_blk(iter: *mut udf_fileident_iter) -> c_int {
    static int udf_fiiter_advance_blk(struct udf_fileident_iter *iter)
    {
    let mut etype: i8 = -1;
    let mut err: c_int = 0;
    iter.loffset++;
    if (iter.loffset < DIV_ROUND_UP(iter.elen, 1<<iter.dir.i_blkbits))
    return 0;
    iter.loffset = 0;
    err = udf_next_aext(iter.dir, &iter.epos, &iter.eloc,
    &iter.elen, &etype, 1);
    if (err < 0)
    return err;
#[no_mangle]
pub unsafe extern "C" fn if(30): err == 0 || etype != (EXT_RECORDED_ALLOCATED >>) -> else {
    if (iter.pos == iter.dir.i_size) {
    iter.elen = 0;
    return 0;
    }
    udf_err(iter.dir.i_sb,
    "extent after position %llu not allocated in directory (ino %llu)\n",
    (unsigned long long)iter.pos, iter.dir.i_ino);
    return -EFSCORRUPTED;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn udf_fiiter_load_bhs(iter: *mut udf_fileident_iter) -> c_int {
    static int udf_fiiter_load_bhs(struct udf_fileident_iter *iter)
    {
    let mut blksize: c_int = 1 << iter.dir.i_blkbits;
    let mut off: c_int = iter.pos & (blksize - 1);
    int err;
    struct fileIdentDesc *fi;
// Is there any further extent we can map from?
    if (!iter.bh[0] && iter.elen) {
    iter.bh[0] = udf_fiiter_bread_blk(iter);
    if (!iter.bh[0]) {
    err = -ENOMEM;
    goto out_brelse;
    }
    if (!buffer_uptodate(iter.bh[0])) {
    err = -EIO;
    goto out_brelse;
    }
    }
// There's no next block so we are done
    if (iter.pos >= iter.dir.i_size)
    return 0;
// Need to fetch next block as well?
    if (off + sizeof(struct fileIdentDesc) > blksize)
    goto fetch_next;
    fi = (struct fileIdentDesc *)(iter.bh[0].b_data + off);
// Need to fetch next block to get name?
    if (off + udf_dir_entry_len(fi) > blksize) {
    fetch_next:
    err = udf_fiiter_advance_blk(iter);
    if (err)
    goto out_brelse;
    iter.bh[1] = udf_fiiter_bread_blk(iter);
    if (!iter.bh[1]) {
    err = -ENOMEM;
    goto out_brelse;
    }
    if (!buffer_uptodate(iter.bh[1])) {
    err = -EIO;
    goto out_brelse;
    }
    }
    return 0;
    out_brelse:
    brelse(iter.bh[0]);
    brelse(iter.bh[1]);
    iter.bh[0] = iter.bh[1] = core::ptr::null_mut();
    return err;
    }
    int udf_fiiter_init(struct udf_fileident_iter *iter, struct inode *dir,
    loff_t pos)
    {
    struct udf_inode_info *iinfo = UDF_I(dir);
    let mut err: c_int = 0;
    int8_t etype;
    iter.dir = dir;
    iter.bh[0] = iter.bh[1] = core::ptr::null_mut();
    iter.pos = pos;
    iter.elen = 0;
    iter.epos.bh = core::ptr::null_mut();
    iter.name = core::ptr::null_mut();
//
// When directory is verified, we don't expect directory iteration to
// fail and it can be difficult to undo without corrupting filesystem.
// So just do not allow memory allocation failures here.
//
    iter.namebuf = kmalloc(UDF_NAME_LEN_CS0, GFP_KERNEL | __GFP_NOFAIL);
    if (iinfo.i_alloc_type == ICBTAG_FLAG_AD_IN_ICB) {
    err = udf_copy_fi(iter);
    goto out;
    }
    err = inode_bmap(dir, iter.pos >> dir.i_blkbits, &iter.epos,
    &iter.eloc, &iter.elen, &iter.loffset, &etype);
    if (err <= 0 || etype != (EXT_RECORDED_ALLOCATED >> 30)) {
    if (pos == dir.i_size)
    return 0;
    udf_err(dir.i_sb,
    "position %llu not allocated in directory (ino %llu)\n",
    (unsigned long long)pos, dir.i_ino);
    err = -EFSCORRUPTED;
    goto out;
    }
    err = udf_fiiter_load_bhs(iter);
    if (err < 0)
    goto out;
    err = udf_copy_fi(iter);
    out:
    if (err < 0)
    udf_fiiter_release(iter);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn udf_fiiter_advance(iter: *mut udf_fileident_iter) -> c_int {
    int udf_fiiter_advance(struct udf_fileident_iter *iter)
    {
    unsigned int oldoff, len;
    let mut blksize: c_int = 1 << iter.dir.i_blkbits;
    int err;
    oldoff = iter.pos & (blksize - 1);
    len = udf_dir_entry_len(&iter.fi);
    iter.pos += len;
    if (UDF_I(iter.dir).i_alloc_type != ICBTAG_FLAG_AD_IN_ICB) {
    if (oldoff + len >= blksize) {
    brelse(iter.bh[0]);
    iter.bh[0] = core::ptr::null_mut();
// Next block already loaded?
    if (iter.bh[1]) {
    iter.bh[0] = iter.bh[1];
    iter.bh[1] = core::ptr::null_mut();
    } else {
    err = udf_fiiter_advance_blk(iter);
    if (err < 0)
    return err;
    }
    }
    err = udf_fiiter_load_bhs(iter);
    if (err < 0)
    return err;
    }
    return udf_copy_fi(iter);
    }
#[no_mangle]
pub unsafe extern "C" fn udf_fiiter_release(iter: *mut udf_fileident_iter) {
    void udf_fiiter_release(struct udf_fileident_iter *iter)
    {
    iter.dir = core::ptr::null_mut();
    brelse(iter.bh[0]);
    brelse(iter.bh[1]);
    iter.bh[0] = iter.bh[1] = core::ptr::null_mut();
    kfree(iter.namebuf);
    iter.namebuf = core::ptr::null_mut();
    }
    static void udf_copy_to_bufs(void *buf1, int len1, void *buf2, int len2,
    int off, void *src, int len)
    {
    int copy;
    if (off >= len1) {
    off -= len1;
    } else {
    copy = min(off + len, len1) - off;
    memcpy(buf1 + off, src, copy);
    src += copy;
    len -= copy;
    off = 0;
    }
    if (len > 0) {
    if (WARN_ON_ONCE(off + len > len2 || !buf2))
    return;
    memcpy(buf2 + off, src, len);
    }
    }
    static uint16_t udf_crc_fi_bufs(void *buf1, int len1, void *buf2, int len2,
    int off, int len)
    {
    int copy;
    let mut crc: u16 = 0;
    if (off >= len1) {
    off -= len1;
    } else {
    copy = min(off + len, len1) - off;
    crc = crc_itu_t(crc, buf1 + off, copy);
    len -= copy;
    off = 0;
    }
    if (len > 0) {
    if (WARN_ON_ONCE(off + len > len2 || !buf2))
    return 0;
    crc = crc_itu_t(crc, buf2 + off, len);
    }
    return crc;
    }
    static void udf_copy_fi_to_bufs(char *buf1, int len1, char *buf2, int len2,
    int off, struct fileIdentDesc *fi,
    uint8_t *impuse, uint8_t *name)
    {
    uint16_t crc;
    let mut fioff: c_int = off;
    let mut crcoff: c_int = off + sizeof(struct tag);
    let mut crclen: c_uint = udf_dir_entry_len(fi) - sizeof(struct tag);
    char zeros[UDF_NAME_PAD] = {};
    let mut endoff: c_int = off + udf_dir_entry_len(fi);
    udf_copy_to_bufs(buf1, len1, buf2, len2, off, fi,
    sizeof(struct fileIdentDesc));
    off += sizeof(struct fileIdentDesc);
    if (impuse)
    udf_copy_to_bufs(buf1, len1, buf2, len2, off, impuse,
    le16_to_cpu(fi.lengthOfImpUse));
    off += le16_to_cpu(fi.lengthOfImpUse);
    if (name) {
    udf_copy_to_bufs(buf1, len1, buf2, len2, off, name,
    fi.lengthFileIdent);
    off += fi.lengthFileIdent;
    udf_copy_to_bufs(buf1, len1, buf2, len2, off, zeros,
    endoff - off);
    }
    crc = udf_crc_fi_bufs(buf1, len1, buf2, len2, crcoff, crclen);
    fi.descTag.descCRC = cpu_to_le16(crc);
    fi.descTag.descCRCLength = cpu_to_le16(crclen);
    fi.descTag.tagChecksum = udf_tag_checksum(&fi.descTag);
    udf_copy_to_bufs(buf1, len1, buf2, len2, fioff, fi, sizeof(struct tag));
    }
#[no_mangle]
pub unsafe extern "C" fn udf_fiiter_write_fi(iter: *mut udf_fileident_iter, impuse: *mut u8) {
    void udf_fiiter_write_fi(struct udf_fileident_iter *iter, uint8_t *impuse)
    {
    struct udf_inode_info *iinfo = UDF_I(iter.dir);
    void *buf1, *buf2 = core::ptr::null_mut();
    int len1, len2 = 0, off;
    let mut blksize: c_int = 1 << iter.dir.i_blkbits;
    off = iter.pos & (blksize - 1);
    if (iinfo.i_alloc_type == ICBTAG_FLAG_AD_IN_ICB) {
    buf1 = iinfo.i_data + iinfo.i_lenEAttr;
    len1 = iter.dir.i_size;
    } else {
    buf1 = iter.bh[0].b_data;
    len1 = blksize;
    if (iter.bh[1]) {
    buf2 = iter.bh[1].b_data;
    len2 = blksize;
    }
    }
    udf_copy_fi_to_bufs(buf1, len1, buf2, len2, off, &iter.fi, impuse,
    iter.name == iter.namebuf ? iter.name : core::ptr::null_mut());
    if (iinfo.i_alloc_type == ICBTAG_FLAG_AD_IN_ICB) {
    mark_inode_dirty(iter.dir);
    } else {
    mmb_mark_buffer_dirty(iter.bh[0], &iinfo.i_metadata_bhs);
    if (iter.bh[1])
    mmb_mark_buffer_dirty(iter.bh[1],
    &iinfo.i_metadata_bhs);
    }
    inode_inc_iversion(iter.dir);
    }
#[no_mangle]
pub unsafe extern "C" fn udf_fiiter_update_elen(iter: *mut udf_fileident_iter, new_elen: u32) {
    void udf_fiiter_update_elen(struct udf_fileident_iter *iter, uint32_t new_elen)
    {
    struct udf_inode_info *iinfo = UDF_I(iter.dir);
    let mut diff: c_int = new_elen - iter.elen;
// Skip update when we already went past the last extent
    if (!iter.elen)
    return;
    iter.elen = new_elen;
    if (iinfo.i_alloc_type == ICBTAG_FLAG_AD_SHORT)
    iter.epos.offset -= sizeof(struct short_ad);
#[no_mangle]
pub unsafe extern "C" fn if(ICBTAG_FLAG_AD_LONG: iinfo->i_alloc_type ==) -> else {
    else if (iinfo.i_alloc_type == ICBTAG_FLAG_AD_LONG)
    iter.epos.offset -= sizeof(struct long_ad);
    udf_write_aext(iter.dir, &iter.epos, &iter.eloc, iter.elen, 1);
    iinfo.i_lenExtents += diff;
    mark_inode_dirty(iter.dir);
    }
// Append new block to directory. @iter is expected to point at EOF
#[no_mangle]
pub unsafe extern "C" fn udf_fiiter_append_blk(iter: *mut udf_fileident_iter) -> c_int {
    int udf_fiiter_append_blk(struct udf_fileident_iter *iter)
    {
    struct udf_inode_info *iinfo = UDF_I(iter.dir);
    let mut blksize: c_int = 1 << iter.dir.i_blkbits;
    struct buffer_head *bh;
    sector_t block;
    let mut old_elen: u32 = iter.elen;
    int err;
    int8_t etype;
    if (WARN_ON_ONCE(iinfo.i_alloc_type == ICBTAG_FLAG_AD_IN_ICB))
    return -EINVAL;
// Round up last extent in the file
    udf_fiiter_update_elen(iter, ALIGN(iter.elen, blksize));
// Allocate new block and refresh mapping information
    block = iinfo.i_lenExtents >> iter.dir.i_blkbits;
    bh = udf_bread(iter.dir, block, 1, &err);
    if (!bh) {
    udf_fiiter_update_elen(iter, old_elen);
    return err;
    }
    err = inode_bmap(iter.dir, block, &iter.epos, &iter.eloc, &iter.elen,
    &iter.loffset, &etype);
    if (err <= 0 || etype != (EXT_RECORDED_ALLOCATED >> 30)) {
    udf_err(iter.dir.i_sb,
    "block %llu not allocated in directory (ino %llu)\n",
    (unsigned long long)block, iter.dir.i_ino);
    return -EFSCORRUPTED;
    }
    if (!(iter.pos & (blksize - 1))) {
    brelse(iter.bh[0]);
    iter.bh[0] = bh;
    } else {
    iter.bh[1] = bh;
    }
    return 0;
    }
    struct short_ad *udf_get_fileshortad(uint8_t *ptr, int maxoffset, uint32_t *offset,
    int inc)
    {
    struct short_ad *sa;
    if ((!ptr) || (!offset)) {
    pr_err("%s: invalidparms\n", __func__);
    return core::ptr::null_mut();
    }
    if ((*offset + sizeof(struct short_ad)) > maxoffset)
    return core::ptr::null_mut();
    else {
    sa = (struct short_ad *)ptr;
    if (sa.extLength == 0)
    return core::ptr::null_mut();
    }
    if (inc)
// offset += sizeof(struct short_ad);
    return sa;
    }
    struct long_ad *udf_get_filelongad(uint8_t *ptr, int maxoffset, uint32_t *offset, int inc)
    {
    struct long_ad *la;
    if ((!ptr) || (!offset)) {
    pr_err("%s: invalidparms\n", __func__);
    return core::ptr::null_mut();
    }
    if ((*offset + sizeof(struct long_ad)) > maxoffset)
    return core::ptr::null_mut();
    else {
    la = (struct long_ad *)ptr;
    if (la.extLength == 0)
    return core::ptr::null_mut();
    }
    if (inc)
// offset += sizeof(struct long_ad);
    return la;
    }
