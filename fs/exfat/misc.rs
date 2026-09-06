//! Automatically rewritten from C to Rust
//! Source: fs/exfat/misc.c
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
// Written 1992,1993 by Werner Almesberger
// 22/11/2000 - Fixed fat_date_unix2dos for dates earlier than 01/01/1980
// and date_dos2unix for date==0 by Igor Zhbanov(bsg@uniyar.ac.ru)
// Copyright (C) 2012-2013 Samsung Electronics Co., Ltd.
//

//
// exfat_fs_error reports a file system problem that might indicate fa data
// corruption/inconsistency. Depending on 'errors' mount option the
// panic() is called, or error message is printed FAT and nothing is done,
// or filesystem is remounted read-only (default behavior).
// In case the file system is remounted read-only, it can be made writable
// again by remounting it.
//
#[no_mangle]
pub unsafe extern "C" fn __exfat_fs_error(sb: *mut super_block, report: c_int, fmt: *const c_char, ...) {
    void __exfat_fs_error(struct super_block *sb, int report, const char *fmt, ...)
    {
    struct exfat_mount_options *opts = &EXFAT_SB(sb).options;
    va_list args;
    struct va_format vaf;
    if (report) {
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    exfat_err(sb, "error, %pV", &vaf);
    va_end(args);
    }
    if (opts.errors == EXFAT_ERRORS_PANIC) {
    panic("exFAT-fs (%s): fs panic from previous error\n",
    sb.s_id);
    } else if (opts.errors == EXFAT_ERRORS_RO && !sb_rdonly(sb)) {
    sb.s_flags |= SB_RDONLY;
    exfat_err(sb, "Filesystem has been set read-only");
    }
    }

#[no_mangle]
unsafe extern "C" fn exfat_adjust_tz(ts: *mut timespec64, tz_off: u8) {
    static void exfat_adjust_tz(struct timespec64 *ts, u8 tz_off)
    {
    if (tz_off <= 0x3F)
    ts.tv_sec -= TIMEZONE_SEC(tz_off);
    else /* 0x40 <= (tz_off & 0x7F) <=0x7F */
    ts.tv_sec += TIMEZONE_SEC(0x80 - tz_off);
    }
#[no_mangle]
pub unsafe extern "C" fn exfat_tz_offset(sbi: *mut exfat_sb_info) -> c_int {
    static inline int exfat_tz_offset(struct exfat_sb_info *sbi)
    {
    if (sbi.options.sys_tz)
    return -sys_tz.tz_minuteswest;
    return sbi.options.time_offset;
    }
// Convert a EXFAT time/date pair to a UNIX date (seconds since 1 1 70).
    void exfat_get_entry_time(struct exfat_sb_info *sbi, struct timespec64 *ts,
    u8 tz, __le16 time, __le16 date, u8 time_cs)
    {
    let mut t: u16 = le16_to_cpu(time);
    let mut d: u16 = le16_to_cpu(date);
    ts.tv_sec = mktime64(1980 + (d >> 9), d >> 5 & 0x000F, d & 0x001F,
    t >> 11, (t >> 5) & 0x003F, (t & 0x001F) << 1);
// time_cs field represent 0 ~ 199cs(1990 ms)
    if (time_cs) {
    ts.tv_sec += time_cs / 100;
    ts.tv_nsec = (time_cs % 100) * 10 * NSEC_PER_MSEC;
    } else
    ts.tv_nsec = 0;
    if (tz & EXFAT_TZ_VALID)
// Adjust timezone to UTC0.
    exfat_adjust_tz(ts, tz & ~EXFAT_TZ_VALID);
    else
    ts.tv_sec -= exfat_tz_offset(sbi) * SECS_PER_MIN;
    }
// Convert linear UNIX date to a EXFAT time/date pair.
    void exfat_set_entry_time(struct exfat_sb_info *sbi, struct timespec64 *ts,
    u8 *tz, __le16 *time, __le16 *date, u8 *time_cs)
    {
    struct tm tm;
    u16 t, d;
    time64_to_tm(ts.tv_sec, 0, &tm);
    t = (tm.tm_hour << 11) | (tm.tm_min << 5) | (tm.tm_sec >> 1);
    d = ((tm.tm_year - 80) <<  9) | ((tm.tm_mon + 1) << 5) | tm.tm_mday;
// time = cpu_to_le16(t);
// date = cpu_to_le16(d);
// time_cs field represent 0 ~ 199cs(1990 ms)
    if (time_cs)
// time_cs = (tm.tm_sec & 1) * 100 +
    ts.tv_nsec / (10 * NSEC_PER_MSEC);
//
// Record 00h value for OffsetFromUtc field and 1 value for OffsetValid
// to indicate that local time and UTC are the same.
//
// tz = EXFAT_TZ_VALID;
    }
//
// The timestamp for access_time has double seconds granularity.
// (There is no 10msIncrement field for access_time unlike create/modify_time)
// atime also has only a 2-second resolution.
//
#[no_mangle]
pub unsafe extern "C" fn exfat_truncate_atime(ts: *mut timespec64) {
    void exfat_truncate_atime(struct timespec64 *ts)
    {
    ts.tv_sec = round_down(ts.tv_sec, 2);
    ts.tv_nsec = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn exfat_truncate_inode_atime(inode: *mut inode) {
    void exfat_truncate_inode_atime(struct inode *inode)
    {
    let mut atime: timespec64 = inode_get_atime(inode);
    exfat_truncate_atime(&atime);
    inode_set_atime_to_ts(inode, atime);
    }
#[no_mangle]
pub unsafe extern "C" fn exfat_calc_chksum16(data: *mut c_void, len: c_int, chksum: u16, type: c_int) -> u16 {
    u16 exfat_calc_chksum16(void *data, int len, u16 chksum, int type)
    {
    int i;
    u8 *c = (u8 *)data;
    for (i = 0; i < len; i++, c++) {
    if (unlikely(type == CS_DIR_ENTRY && (i == 2 || i == 3)))
    continue;
    chksum = ((chksum << 15) | (chksum >> 1)) + *c;
    }
    return chksum;
    }
#[no_mangle]
pub unsafe extern "C" fn exfat_calc_chksum32(data: *mut c_void, len: c_int, chksum: u32, type: c_int) -> u32 {
    u32 exfat_calc_chksum32(void *data, int len, u32 chksum, int type)
    {
    int i;
    u8 *c = (u8 *)data;
    for (i = 0; i < len; i++, c++) {
    if (unlikely(type == CS_BOOT_SECTOR &&
    (i == 106 || i == 107 || i == 112)))
    continue;
    chksum = ((chksum << 31) | (chksum >> 1)) + *c;
    }
    return chksum;
    }
#[no_mangle]
pub unsafe extern "C" fn exfat_update_bh(bh: *mut buffer_head, sync: c_int) -> c_int {
    int exfat_update_bh(struct buffer_head *bh, int sync)
    {
    let mut err: c_int = 0;
    set_buffer_uptodate(bh);
    mark_buffer_dirty(bh);
    if (sync)
    err = sync_dirty_buffer(bh);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn exfat_update_bhs(bhs: *mut buffer_head, nr_bhs: c_int, sync: c_int) -> c_int {
    int exfat_update_bhs(struct buffer_head **bhs, int nr_bhs, int sync)
    {
    int i, err = 0;
    for (i = 0; i < nr_bhs; i++) {
    set_buffer_uptodate(bhs[i]);
    mark_buffer_dirty(bhs[i]);
    if (sync)
    write_dirty_buffer(bhs[i], REQ_SYNC);
    }
    for (i = 0; i < nr_bhs && sync; i++) {
    wait_on_buffer(bhs[i]);
    if (!err && !buffer_uptodate(bhs[i]))
    err = -EIO;
    }
    return err;
    }
    void exfat_chain_set(struct exfat_chain *ec, unsigned int dir,
    unsigned int size, unsigned char flags)
    {
    ec.dir = dir;
    ec.size = size;
    ec.flags = flags;
    }
#[no_mangle]
pub unsafe extern "C" fn exfat_chain_dup(dup: *mut exfat_chain, ec: *mut exfat_chain) {
    void exfat_chain_dup(struct exfat_chain *dup, struct exfat_chain *ec)
    {
    return exfat_chain_set(dup, ec.dir, ec.size, ec.flags);
    }
