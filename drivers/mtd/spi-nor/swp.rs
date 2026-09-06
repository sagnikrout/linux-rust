//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/spi-nor/swp.c
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
// SPI NOR Software Write Protection logic.
//
// Copyright (C) 2005, Intec Automation Inc.
// Copyright (C) 2014, Freescale Semiconductor, Inc.
//

#[no_mangle]
unsafe extern "C" fn spi_nor_get_sr_bp_mask(nor: *mut spi_nor) -> u8 {
    static u8 spi_nor_get_sr_bp_mask(struct spi_nor *nor)
    {
    let mut mask: u8 = SR_BP2 | SR_BP1 | SR_BP0;
    if (nor.flags & SNOR_F_HAS_SR_BP3_BIT6)
    return mask | SR_BP3_BIT6;
    if (nor.flags & SNOR_F_HAS_4BIT_BP)
    return mask | SR_BP3;
    return mask;
    }
#[no_mangle]
unsafe extern "C" fn spi_nor_get_sr_tb_mask(nor: *mut spi_nor) -> u8 {
    static u8 spi_nor_get_sr_tb_mask(struct spi_nor *nor)
    {
    if (nor.flags & SNOR_F_HAS_SR_TB_BIT6)
    return SR_TB_BIT6;
#[no_mangle]
pub unsafe extern "C" fn if(SNOR_F_HAS_SR_TB: nor->flags &) -> else {
    else if (nor.flags & SNOR_F_HAS_SR_TB)
    return SR_TB_BIT5;
    else
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spi_nor_get_sr_cmp_mask(nor: *mut spi_nor) -> u8 {
    static u8 spi_nor_get_sr_cmp_mask(struct spi_nor *nor)
    {
    if (!(nor.flags & SNOR_F_NO_READ_CR) &&
    nor.flags & SNOR_F_HAS_SR2_CMP_BIT6)
    return SR2_CMP_BIT6;
    else
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn spi_nor_get_min_prot_length_sr(nor: *mut spi_nor) -> u64 {
    u64 spi_nor_get_min_prot_length_sr(struct spi_nor *nor)
    {
    unsigned int bp_slots, bp_slots_needed;
//
// sector_size will eventually be replaced with the max erase size of
// the flash. For now, we need to have that ugly default.
//
    let mut sector_size: c_uint = nor.info.sector_size ?: SPI_NOR_DEFAULT_SECTOR_SIZE;
    let mut n_sectors: u64 = div_u64(nor.params.size, sector_size);
    let mut mask: u8 = spi_nor_get_sr_bp_mask(nor);
// Reserved one for "protect none" and one for "protect all".
    bp_slots = (1 << hweight8(mask)) - 2;
    bp_slots_needed = ilog2(n_sectors);
    if (bp_slots_needed > bp_slots)
    return sector_size << (bp_slots_needed - bp_slots);
    else
    return sector_size;
    }
    void spi_nor_get_locked_range_sr(struct spi_nor *nor, const u8 *sr, loff_t *ofs,
    u64 *len)
    {
    u64 min_prot_len;
    let mut bp_mask: u8 = spi_nor_get_sr_bp_mask(nor);
    let mut tb_mask: u8 = spi_nor_get_sr_tb_mask(nor);
    let mut cmp_mask: u8 = spi_nor_get_sr_cmp_mask(nor);
    u8 bp, val = sr[0] & bp_mask;
    let mut tb: bool = (nor.flags & SNOR_F_HAS_SR_TB) ? sr[0] & tb_mask : 0;
    let mut cmp: bool = sr[1] & cmp_mask;
    if (nor.flags & SNOR_F_HAS_SR_BP3_BIT6 && val & SR_BP3_BIT6)
    val = (val & ~SR_BP3_BIT6) | SR_BP3;
    bp = val >> SR_BP_SHIFT;
    if (!bp) {
    if (!cmp) {
// No protection
// ofs = 0;
// len = 0;
    } else {
// Full protection
// ofs = 0;
// len = nor->params->size;
    }
    return;
    }
    min_prot_len = spi_nor_get_min_prot_length_sr(nor);
// len = min_prot_len << (bp - 1);
    if (*len > nor.params.size)
// len = nor->params->size;
    if (cmp)
// len = nor->params->size - *len;
    if (!cmp) {
    if (tb)
// ofs = 0;
    else
// ofs = nor->params->size - *len;
    } else {
    if (tb)
// ofs = nor->params->size - *len;
    else
// ofs = 0;
    }
    }
//
// Return true if the entire region is locked (if @locked is true) or unlocked
// (if @locked is false); false otherwise.
//
    static bool spi_nor_check_lock_status_sr(struct spi_nor *nor, loff_t ofs,
    u64 len, const u8 *sr, bool locked)
    {
    loff_t lock_offs, lock_offs_max, offs_max;
    u64 lock_len;
    if (!len)
    return true;
    spi_nor_get_locked_range_sr(nor, sr, &lock_offs, &lock_len);
    lock_offs_max = lock_offs + lock_len;
    offs_max = ofs + len;
    if (locked)
// Requested range is a sub-range of locked range
    return (offs_max <= lock_offs_max) && (ofs >= lock_offs);
    else
// Requested range does not overlap with locked range
    return (ofs >= lock_offs_max) || (offs_max <= lock_offs);
    }
#[no_mangle]
pub unsafe extern "C" fn spi_nor_is_locked_sr(nor: *mut spi_nor, ofs: loff_t, len: u64, sr: *const u8) -> bool {
    bool spi_nor_is_locked_sr(struct spi_nor *nor, loff_t ofs, u64 len, const u8 *sr)
    {
    return spi_nor_check_lock_status_sr(nor, ofs, len, sr, true);
    }
    static bool spi_nor_is_unlocked_sr(struct spi_nor *nor, loff_t ofs, u64 len,
    const u8 *sr)
    {
    return spi_nor_check_lock_status_sr(nor, ofs, len, sr, false);
    }
#[no_mangle]
unsafe extern "C" fn spi_nor_sr_set_bp_mask(nor: *mut spi_nor, sr: *mut u8, pow: u8) -> c_int {
    static int spi_nor_sr_set_bp_mask(struct spi_nor *nor, u8 *sr, u8 pow)
    {
    let mut mask: u8 = spi_nor_get_sr_bp_mask(nor);
    let mut val: u8 = pow << SR_BP_SHIFT;
    if (nor.flags & SNOR_F_HAS_SR_BP3_BIT6 && val & SR_BP3)
    val = (val & ~SR_BP3) | SR_BP3_BIT6;
    if (val & ~mask)
    return -EINVAL;
    sr[0] |= val;
    return 0;
    }
    static int spi_nor_build_sr(struct spi_nor *nor, const u8 *old_sr, u8 *new_sr,
    u8 pow, bool use_top, bool cmp)
    {
    let mut bp_mask: u8 = spi_nor_get_sr_bp_mask(nor);
    let mut tb_mask: u8 = spi_nor_get_sr_tb_mask(nor);
    let mut cmp_mask: u8 = spi_nor_get_sr_cmp_mask(nor);
    int ret;
    new_sr[0] = old_sr[0] & ~bp_mask & ~tb_mask;
    new_sr[1] = old_sr[1] & ~cmp_mask;
// Build BP field
    ret = spi_nor_sr_set_bp_mask(nor, &new_sr[0], pow);
    if (ret)
    return ret;
// Build TB field
    if ((!cmp && !use_top) || (cmp && use_top))
    new_sr[0] |= tb_mask;
// Build CMP field
    if (cmp)
    new_sr[1] |= cmp_mask;
    return 0;
    }
//
// Keep a local cache containing all lock-related bits for debugfs use only.
// This way, debugfs never needs to access the flash directly.
//
#[no_mangle]
pub unsafe extern "C" fn spi_nor_cache_sr_lock_bits(nor: *mut spi_nor, sr: *mut u8) {
    void spi_nor_cache_sr_lock_bits(struct spi_nor *nor, u8 *sr)
    {
    let mut bp_mask: u8 = spi_nor_get_sr_bp_mask(nor);
    let mut tb_mask: u8 = spi_nor_get_sr_tb_mask(nor);
    let mut cmp_mask: u8 = spi_nor_get_sr_cmp_mask(nor);
    u8 sr_cr[2] = {};
    if (!sr) {
    if (spi_nor_read_sr(nor, nor.bouncebuf))
    return;
    sr_cr[0] = nor.bouncebuf[0];
    if (!(nor.flags & SNOR_F_NO_READ_CR)) {
    if (spi_nor_read_cr(nor, nor.bouncebuf))
    return;
    }
    sr_cr[1] = nor.bouncebuf[0];
    sr = sr_cr;
    }
    nor.dfs_sr_cache[0] = sr[0] & (bp_mask | tb_mask | SR_SRWD);
    nor.dfs_sr_cache[1] = sr[1] & cmp_mask;
    }
//
// Lock a region of the flash. Compatible with ST Micro and similar flash.
// Supports the block protection bits BP{0,1,2}/BP{0,1,2,3} in the status
// register
// (SR). Does not support these features found in newer SR bitfields:
// - SEC: sector/block protect - only handle SEC=0 (block protect)
//
// Support for the following is provided conditionally for some flash:
// - TB: top/bottom protect
// - CMP: complement protect (BP and TP describe the unlocked part, while
// the reminder is locked)
//
// Sample table portion for 8MB flash (Winbond w25q64fw):
//
// SEC  |  TB   |  BP2  |  BP1  |  BP0  |  Prot Length  | Protected Portion
// --------------------------------------------------------------------------
// X   |   X   |   0   |   0   |   0   |  NONE         | NONE
// 0   |   0   |   0   |   0   |   1   |  128 KB       | Upper 1/64
// 0   |   0   |   0   |   1   |   0   |  256 KB       | Upper 1/32
// 0   |   0   |   0   |   1   |   1   |  512 KB       | Upper 1/16
// 0   |   0   |   1   |   0   |   0   |  1 MB         | Upper 1/8
// 0   |   0   |   1   |   0   |   1   |  2 MB         | Upper 1/4
// 0   |   0   |   1   |   1   |   0   |  4 MB         | Upper 1/2
// X   |   X   |   1   |   1   |   1   |  8 MB         | ALL
// ------|-------|-------|-------|-------|---------------|-------------------
// 0   |   1   |   0   |   0   |   1   |  128 KB       | Lower 1/64
// 0   |   1   |   0   |   1   |   0   |  256 KB       | Lower 1/32
// 0   |   1   |   0   |   1   |   1   |  512 KB       | Lower 1/16
// 0   |   1   |   1   |   0   |   0   |  1 MB         | Lower 1/8
// 0   |   1   |   1   |   0   |   1   |  2 MB         | Lower 1/4
// 0   |   1   |   1   |   1   |   0   |  4 MB         | Lower 1/2
//
// Returns negative on errors, 0 on success.
//
#[no_mangle]
unsafe extern "C" fn spi_nor_sr_lock(nor: *mut spi_nor, ofs: loff_t, len: u64) -> c_int {
    static int spi_nor_sr_lock(struct spi_nor *nor, loff_t ofs, u64 len)
    {
    let mut min_prot_len: u64 = spi_nor_get_min_prot_length_sr(nor);
    u8 status_old[2] = {}, status_new[2] = {}, status_new_cmp[2] = {};
    u8 *best_status_new = status_new;
    loff_t ofs_old, ofs_new, ofs_new_cmp;
    u64 len_old, len_new, len_new_cmp;
    loff_t lock_len;
    bool can_be_top = true, can_be_bottom = nor.flags & SNOR_F_HAS_SR_TB,
    can_be_cmp = spi_nor_get_sr_cmp_mask(nor);
    bool use_top;
    int ret;
    u8 pow;
    ret = spi_nor_read_sr(nor, nor.bouncebuf);
    if (ret)
    return ret;
    status_old[0] = nor.bouncebuf[0];
    if (!(nor.flags & SNOR_F_NO_READ_CR)) {
    ret = spi_nor_read_cr(nor, nor.bouncebuf);
    if (ret)
    return ret;
    status_old[1] = nor.bouncebuf[0];
    }
// If nothing in our range is unlocked, we don't need to do anything
    if (spi_nor_is_locked_sr(nor, ofs, len, status_old))
    return 0;
// If anything below us is unlocked, we can't use 'bottom' protection
    if (!spi_nor_is_locked_sr(nor, 0, ofs, status_old))
    can_be_bottom = false;
// If anything above us is unlocked, we can't use 'top' protection
    if (!spi_nor_is_locked_sr(nor, ofs + len, nor.params.size - (ofs + len),
    status_old))
    can_be_top = false;
    if (!can_be_bottom && !can_be_top)
    return -EINVAL;
// Prefer top, if both are valid
    use_top = can_be_top;
// lock_len: length of region that should end up locked
    if (use_top)
    lock_len = nor.params.size - ofs;
    else
    lock_len = ofs + len;
    if (lock_len == nor.params.size)
    pow = (nor.flags & SNOR_F_HAS_4BIT_BP) ? GENMASK(3, 0) : GENMASK(2, 0);
    else
    pow = ilog2(lock_len) - ilog2(min_prot_len) + 1;
    ret = spi_nor_build_sr(nor, status_old, status_new, pow, use_top, false);
    if (ret)
    return ret;
//
// In case the region asked is not fully met, maybe we can try with the
// complement feature
//
    spi_nor_get_locked_range_sr(nor, status_new, &ofs_new, &len_new);
    if (can_be_cmp && len_new != lock_len) {
    pow = ilog2(nor.params.size - lock_len) - ilog2(min_prot_len) + 1;
    ret = spi_nor_build_sr(nor, status_old, status_new_cmp, pow, use_top, true);
    if (ret)
    return ret;
//
// ilog2() "floors" the result, which means in some cases we may have to
// manually reduce the scope when the complement feature is used.
// The uAPI is to never lock more than what is requested, but less is accepted.
// Make sure we are not covering a too wide range, reduce it otherwise.
//
    spi_nor_get_locked_range_sr(nor, status_new_cmp, &ofs_new_cmp, &len_new_cmp);
    if (len_new_cmp > lock_len) {
    pow++;
    ret = spi_nor_build_sr(nor, status_old, status_new_cmp, pow, use_top, true);
    if (ret)
    return ret;
    }
// Pick the CMP configuration if we cover a closer range
    spi_nor_get_locked_range_sr(nor, status_new, &ofs_new, &len_new);
    spi_nor_get_locked_range_sr(nor, status_new_cmp, &ofs_new_cmp, &len_new_cmp);
    if (len_new_cmp <= lock_len &&
    (lock_len - len_new_cmp) < (lock_len - len_new))
    best_status_new = status_new_cmp;
    }
//
// Disallow further writes if WP# pin is neither left floating nor
// wrongly tied to GND (that includes internal pull-downs).
// WP# pin hard strapped to GND can be a valid use case.
//
    if (!(nor.flags & SNOR_F_NO_WP))
    best_status_new[0] |= SR_SRWD;
    spi_nor_get_locked_range_sr(nor, status_old, &ofs_old, &len_old);
    spi_nor_get_locked_range_sr(nor, best_status_new, &ofs_new, &len_new);
// Don't "lock" with no region!
    if (!len_new)
    return -EINVAL;
// Don't bother if they're the same
    if (best_status_new[0] == status_old[0] && best_status_new[1] == status_old[1])
    return 0;
// Only modify protection if it will not unlock other areas
    if (len_old &&
    (ofs_old < ofs_new || (ofs_new + len_new) < (ofs_old + len_old)))
    return -EINVAL;
    if (nor.flags & SNOR_F_NO_READ_CR)
    ret = spi_nor_write_sr_and_check(nor, best_status_new[0]);
    else
    ret = spi_nor_write_sr_cr_and_check(nor, best_status_new);
    if (ret)
    return ret;
    spi_nor_cache_sr_lock_bits(nor, best_status_new);
    return 0;
    }
//
// Unlock a region of the flash. See spi_nor_sr_lock() for more info
//
// Returns negative on errors, 0 on success.
//
#[no_mangle]
unsafe extern "C" fn spi_nor_sr_unlock(nor: *mut spi_nor, ofs: loff_t, len: u64) -> c_int {
    static int spi_nor_sr_unlock(struct spi_nor *nor, loff_t ofs, u64 len)
    {
    let mut min_prot_len: u64 = spi_nor_get_min_prot_length_sr(nor);
    u8 status_old[2] = {}, status_new[2] = {}, status_new_cmp[2] = {};
    u8 *best_status_new = status_new;
    loff_t ofs_old, ofs_new, ofs_new_cmp;
    u64 len_old, len_new, len_new_cmp;
    loff_t lock_len;
    bool can_be_top = true, can_be_bottom = nor.flags & SNOR_F_HAS_SR_TB,
    can_be_cmp = spi_nor_get_sr_cmp_mask(nor);
    bool use_top;
    int ret;
    u8 pow;
    ret = spi_nor_read_sr(nor, nor.bouncebuf);
    if (ret)
    return ret;
    status_old[0] = nor.bouncebuf[0];
    if (!(nor.flags & SNOR_F_NO_READ_CR)) {
    ret = spi_nor_read_cr(nor, nor.bouncebuf);
    if (ret)
    return ret;
    status_old[1] = nor.bouncebuf[0];
    }
// If nothing in our range is locked, we don't need to do anything
    if (spi_nor_is_unlocked_sr(nor, ofs, len, status_old))
    return 0;
// If anything below us is locked, we can't use 'top' protection
    if (!spi_nor_is_unlocked_sr(nor, 0, ofs, status_old))
    can_be_top = false;
// If anything above us is locked, we can't use 'bottom' protection
    if (!spi_nor_is_unlocked_sr(nor, ofs + len, nor.params.size - (ofs + len),
    status_old))
    can_be_bottom = false;
    if (!can_be_bottom && !can_be_top)
    return -EINVAL;
// Prefer top, if both are valid
    use_top = can_be_top;
//
// lock_len: length of region that should remain locked.
//
// When can_be_top and can_be_bottom booleans are true, both adjacent
// regions are unlocked, thus the entire flash can be unlocked.
//
    if (can_be_top && can_be_bottom)
    lock_len = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: use_top) -> else {
    else if (use_top)
    lock_len = nor.params.size - (ofs + len);
    else
    lock_len = ofs;
    if (lock_len == 0)
    pow = 0; /* fully unlocked */
    else
    pow = ilog2(lock_len) - ilog2(min_prot_len) + 1;
    ret = spi_nor_build_sr(nor, status_old, status_new, pow, use_top, false);
    if (ret)
    return ret;
//
// In case the region asked is not fully met, maybe we can try with the
// complement feature
//
    spi_nor_get_locked_range_sr(nor, status_new, &ofs_new, &len_new);
    if (can_be_cmp && len_new != lock_len) {
    pow = ilog2(nor.params.size - lock_len) - ilog2(min_prot_len) + 1;
    ret = spi_nor_build_sr(nor, status_old, status_new_cmp, pow, use_top, true);
    if (ret)
    return ret;
//
// ilog2() "floors" the result, which means in some cases we may have to
// manually reduce the scope when the complement feature is used.
// The uAPI is to never unlock more than what is requested, but less is accepted.
// Make sure we are not covering a too small range, increase it otherwise.
//
    spi_nor_get_locked_range_sr(nor, status_new_cmp, &ofs_new_cmp, &len_new_cmp);
    if (len_new_cmp < lock_len) {
    pow--;
    ret = spi_nor_build_sr(nor, status_old, status_new_cmp, pow, use_top, true);
    if (ret)
    return ret;
    }
// Pick the CMP configuration if we cover a closer range
    spi_nor_get_locked_range_sr(nor, status_new, &ofs_new, &len_new);
    spi_nor_get_locked_range_sr(nor, status_new_cmp, &ofs_new_cmp, &len_new_cmp);
    if (len_new_cmp <= lock_len &&
    (lock_len - len_new_cmp) < (lock_len - len_new))
    best_status_new = status_new_cmp;
    }
// Don't protect status register if we're fully unlocked
    if (lock_len == 0)
    best_status_new[0] &= ~SR_SRWD;
// Don't bother if they're the same
    if (best_status_new[0] == status_old[0] && best_status_new[1] == status_old[1])
    return 0;
// Only modify protection if it will not lock other areas
    spi_nor_get_locked_range_sr(nor, status_old, &ofs_old, &len_old);
    spi_nor_get_locked_range_sr(nor, best_status_new, &ofs_new, &len_new);
    if (len_old && len_new &&
    (ofs_new < ofs_old || (ofs_old + len_old) < (ofs_new + len_new)))
    return -EINVAL;
    if (nor.flags & SNOR_F_NO_READ_CR)
    ret = spi_nor_write_sr_and_check(nor, best_status_new[0]);
    else
    ret = spi_nor_write_sr_cr_and_check(nor, best_status_new);
    if (ret)
    return ret;
    spi_nor_cache_sr_lock_bits(nor, best_status_new);
    return 0;
    }
//
// Check if a region of the flash is (completely) locked. See spi_nor_sr_lock()
// for more info.
//
// Returns 1 if entire region is locked, 0 if any portion is unlocked, and
// negative on errors.
//
#[no_mangle]
unsafe extern "C" fn spi_nor_sr_is_locked(nor: *mut spi_nor, ofs: loff_t, len: u64) -> c_int {
    static int spi_nor_sr_is_locked(struct spi_nor *nor, loff_t ofs, u64 len)
    {
    u8 sr_cr[2] = {};
    int ret;
    ret = spi_nor_read_sr(nor, nor.bouncebuf);
    if (ret)
    return ret;
    sr_cr[0] = nor.bouncebuf[0];
    if (!(nor.flags & SNOR_F_NO_READ_CR)) {
    ret = spi_nor_read_cr(nor, nor.bouncebuf);
    if (ret)
    return ret;
    sr_cr[1] = nor.bouncebuf[0];
    }
    return spi_nor_is_locked_sr(nor, ofs, len, sr_cr);
    }
    static const struct spi_nor_locking_ops spi_nor_sr_locking_ops = {
    .lock = spi_nor_sr_lock,
    .unlock = spi_nor_sr_unlock,
    .is_locked = spi_nor_sr_is_locked,
    };
#[no_mangle]
pub unsafe extern "C" fn spi_nor_init_default_locking_ops(nor: *mut spi_nor) {
    void spi_nor_init_default_locking_ops(struct spi_nor *nor)
    {
    nor.params.locking_ops = &spi_nor_sr_locking_ops;
    }
#[no_mangle]
pub unsafe extern "C" fn spi_nor_has_default_locking_ops(nor: *mut spi_nor) -> bool {
    bool spi_nor_has_default_locking_ops(struct spi_nor *nor)
    {
    return nor.params.locking_ops == &spi_nor_sr_locking_ops;
    }
#[no_mangle]
unsafe extern "C" fn spi_nor_lock(mtd: *mut mtd_info, ofs: loff_t, len: u64) -> c_int {
    static int spi_nor_lock(struct mtd_info *mtd, loff_t ofs, u64 len)
    {
    struct spi_nor *nor = mtd_to_spi_nor(mtd);
    int ret;
    ret = spi_nor_prep_and_lock(nor);
    if (ret)
    return ret;
    ret = nor.params.locking_ops.lock(nor, ofs, len);
    spi_nor_unlock_and_unprep(nor);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn spi_nor_unlock(mtd: *mut mtd_info, ofs: loff_t, len: u64) -> c_int {
    static int spi_nor_unlock(struct mtd_info *mtd, loff_t ofs, u64 len)
    {
    struct spi_nor *nor = mtd_to_spi_nor(mtd);
    int ret;
    ret = spi_nor_prep_and_lock(nor);
    if (ret)
    return ret;
    ret = nor.params.locking_ops.unlock(nor, ofs, len);
    spi_nor_unlock_and_unprep(nor);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn spi_nor_is_locked(mtd: *mut mtd_info, ofs: loff_t, len: u64) -> c_int {
    static int spi_nor_is_locked(struct mtd_info *mtd, loff_t ofs, u64 len)
    {
    struct spi_nor *nor = mtd_to_spi_nor(mtd);
    int ret;
    ret = spi_nor_prep_and_lock(nor);
    if (ret)
    return ret;
    ret = nor.params.locking_ops.is_locked(nor, ofs, len);
    spi_nor_unlock_and_unprep(nor);
    return ret;
    }
//
// spi_nor_try_unlock_all() - Tries to unlock the entire flash memory array.
// @nor:	pointer to a 'struct spi_nor'.
//
// Some SPI NOR flashes are write protected by default after a power-on reset
// cycle, in order to avoid inadvertent writes during power-up. Backward
// compatibility imposes to unlock the entire flash memory array at power-up
// by default.
//
// Unprotecting the entire flash array will fail for boards which are hardware
// write-protected. Thus any errors are ignored.
//
#[no_mangle]
pub unsafe extern "C" fn spi_nor_try_unlock_all(nor: *mut spi_nor) {
    void spi_nor_try_unlock_all(struct spi_nor *nor)
    {
    int ret;
    if (!(nor.flags & SNOR_F_HAS_LOCK))
    return;
    dev_dbg(nor.dev, "Unprotecting entire flash array\n");
    ret = spi_nor_unlock(&nor.mtd, 0, nor.params.size);
    if (ret)
    dev_dbg(nor.dev, "Failed to unlock the entire flash memory array\n");
    }
#[no_mangle]
pub unsafe extern "C" fn spi_nor_set_mtd_locking_ops(nor: *mut spi_nor) {
    void spi_nor_set_mtd_locking_ops(struct spi_nor *nor)
    {
    struct mtd_info *mtd = &nor.mtd;
    if (!nor.params.locking_ops)
    return;
    mtd._lock = spi_nor_lock;
    mtd._unlock = spi_nor_unlock;
    mtd._is_locked = spi_nor_is_locked;
    }
