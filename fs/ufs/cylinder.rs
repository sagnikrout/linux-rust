//! Automatically rewritten from C to Rust
//! Source: fs/ufs/cylinder.c
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
// linux/fs/ufs/cylinder.c
//
// Copyright (C) 1998
// Daniel Pirkl <daniel.pirkl@email.cz>
// Charles University, Faculty of Mathematics and Physics
//
// ext2 - inode (block) bitmap caching inspired
//

//
// Read cylinder group into cache. The memory space for ufs_cg_private_info
// structure is already allocated during ufs_read_super.
//
    static bool ufs_read_cylinder(struct super_block *sb,
    unsigned cgno, unsigned bitmap_nr)
    {
    let mut sbi: *mut ufs_sb_info = UFS_SB(sb);
    struct ufs_sb_private_info * uspi;
    struct ufs_cg_private_info * ucpi;
    struct ufs_cylinder_group * ucg;
    unsigned i, j;
    UFSD("ENTER, cgno %u, bitmap_nr %u\n", cgno, bitmap_nr);
    uspi = sbi.s_uspi;
    ucpi = sbi.s_ucpi[bitmap_nr];
    ucg = (struct ufs_cylinder_group *)sbi.s_ucg[cgno].b_data;
    UCPI_UBH(ucpi).fragment = ufs_cgcmin(cgno);
    UCPI_UBH(ucpi).count = uspi.s_cgsize >> sb.s_blocksize_bits;
//
// We have already the first fragment of cylinder group block in buffer
//
    UCPI_UBH(ucpi).bh[0] = sbi.s_ucg[cgno];
    for (i = 1; i < UCPI_UBH(ucpi).count; i++) {
    UCPI_UBH(ucpi).bh[i] = sb_bread(sb, UCPI_UBH(ucpi).fragment + i);
    if (!UCPI_UBH(ucpi).bh[i])
    goto failed;
    }
    sbi.s_cgno[bitmap_nr] = cgno;
    ucpi.c_cgx	= fs32_to_cpu(sb, ucg.cg_cgx);
    ucpi.c_ncyl	= fs16_to_cpu(sb, ucg.cg_ncyl);
    ucpi.c_niblk	= fs16_to_cpu(sb, ucg.cg_niblk);
    ucpi.c_ndblk	= fs32_to_cpu(sb, ucg.cg_ndblk);
    ucpi.c_rotor	= fs32_to_cpu(sb, ucg.cg_rotor);
    ucpi.c_frotor	= fs32_to_cpu(sb, ucg.cg_frotor);
    ucpi.c_irotor	= fs32_to_cpu(sb, ucg.cg_irotor);
    ucpi.c_btotoff	= fs32_to_cpu(sb, ucg.cg_btotoff);
    ucpi.c_boff	= fs32_to_cpu(sb, ucg.cg_boff);
    ucpi.c_iusedoff = fs32_to_cpu(sb, ucg.cg_iusedoff);
    ucpi.c_freeoff	= fs32_to_cpu(sb, ucg.cg_freeoff);
    ucpi.c_nextfreeoff = fs32_to_cpu(sb, ucg.cg_nextfreeoff);
    ucpi.c_clustersumoff = fs32_to_cpu(sb, ucg.cg_u.cg_44.cg_clustersumoff);
    ucpi.c_clusteroff = fs32_to_cpu(sb, ucg.cg_u.cg_44.cg_clusteroff);
    ucpi.c_nclusterblks = fs32_to_cpu(sb, ucg.cg_u.cg_44.cg_nclusterblks);
    UFSD("EXIT\n");
    return true;
    failed:
    for (j = 1; j < i; j++)
    brelse(UCPI_UBH(ucpi).bh[j]);
    sbi.s_cgno[bitmap_nr] = UFS_CGNO_EMPTY;
    ufs_error (sb, "ufs_read_cylinder", "can't read cylinder group block %u", cgno);
    return false;
    }
//
// Remove cylinder group from cache, doesn't release memory
// allocated for cylinder group (this is done at ufs_put_super only).
//
#[no_mangle]
pub unsafe extern "C" fn ufs_put_cylinder(sb: *mut *mut super_block, bitmap_nr: unsigned) {
    void ufs_put_cylinder (struct super_block * sb, unsigned bitmap_nr)
    {
    let mut sbi: *mut ufs_sb_info = UFS_SB(sb);
    struct ufs_sb_private_info * uspi;
    struct ufs_cg_private_info * ucpi;
    struct ufs_cylinder_group * ucg;
    unsigned i;
    UFSD("ENTER, bitmap_nr %u\n", bitmap_nr);
    uspi = sbi.s_uspi;
    if (sbi.s_cgno[bitmap_nr] == UFS_CGNO_EMPTY) {
    UFSD("EXIT\n");
    return;
    }
    ucpi = sbi.s_ucpi[bitmap_nr];
    ucg = ubh_get_ucg(UCPI_UBH(ucpi));
    if (uspi.s_ncg > UFS_MAX_GROUP_LOADED && bitmap_nr >= sbi.s_cg_loaded) {
    ufs_panic (sb, "ufs_put_cylinder", "internal error");
    return;
    }
//
// rotor is not so important data, so we put it to disk
// at the end of working with cylinder
//
    ucg.cg_rotor = cpu_to_fs32(sb, ucpi.c_rotor);
    ucg.cg_frotor = cpu_to_fs32(sb, ucpi.c_frotor);
    ucg.cg_irotor = cpu_to_fs32(sb, ucpi.c_irotor);
    ubh_mark_buffer_dirty (UCPI_UBH(ucpi));
    for (i = 1; i < UCPI_UBH(ucpi).count; i++) {
    brelse (UCPI_UBH(ucpi).bh[i]);
    }
    sbi.s_cgno[bitmap_nr] = UFS_CGNO_EMPTY;
    UFSD("EXIT\n");
    }
//
// Find cylinder group in cache and return it as pointer.
// If cylinder group is not in cache, we will load it from disk.
//
// The cache is managed by LRU algorithm.
//
    struct ufs_cg_private_info * ufs_load_cylinder (
    struct super_block * sb, unsigned cgno)
    {
    let mut sbi: *mut ufs_sb_info = UFS_SB(sb);
    struct ufs_sb_private_info * uspi;
    struct ufs_cg_private_info * ucpi;
    unsigned cg, i, j;
    UFSD("ENTER, cgno %u\n", cgno);
    uspi = sbi.s_uspi;
    if (cgno >= uspi.s_ncg) {
    ufs_panic (sb, "ufs_load_cylinder", "internal error, high number of cg");
    return core::ptr::null_mut();
    }
//
// Cylinder group number cg it in cache and it was last used
//
    if (sbi.s_cgno[0] == cgno) {
    UFSD("EXIT\n");
    return sbi.s_ucpi[0];
    }
//
// Number of cylinder groups is not higher than UFS_MAX_GROUP_LOADED
//
    if (uspi.s_ncg <= UFS_MAX_GROUP_LOADED) {
    if (sbi.s_cgno[cgno] != UFS_CGNO_EMPTY) {
    if (sbi.s_cgno[cgno] != cgno) {
    ufs_panic (sb, "ufs_load_cylinder", "internal error, wrong number of cg in cache");
    UFSD("EXIT (FAILED)\n");
    return core::ptr::null_mut();
    }
    } else {
    if (unlikely(!ufs_read_cylinder (sb, cgno, cgno))) {
    UFSD("EXIT (FAILED)\n");
    return core::ptr::null_mut();
    }
    }
    UFSD("EXIT\n");
    return sbi.s_ucpi[cgno];
    }
//
// Cylinder group number cg is in cache but it was not last used,
// we will move to the first position
//
    for (i = 0; i < sbi.s_cg_loaded && sbi.s_cgno[i] != cgno; i++);
    if (i < sbi.s_cg_loaded && sbi.s_cgno[i] == cgno) {
    cg = sbi.s_cgno[i];
    ucpi = sbi.s_ucpi[i];
    for (j = i; j > 0; j--) {
    sbi.s_cgno[j] = sbi.s_cgno[j-1];
    sbi.s_ucpi[j] = sbi.s_ucpi[j-1];
    }
    sbi.s_cgno[0] = cg;
    sbi.s_ucpi[0] = ucpi;
//
// Cylinder group number cg is not in cache, we will read it from disk
// and put it to the first position
//
    } else {
    if (sbi.s_cg_loaded < UFS_MAX_GROUP_LOADED)
    sbi.s_cg_loaded++;
    else
    ufs_put_cylinder (sb, UFS_MAX_GROUP_LOADED-1);
    ucpi = sbi.s_ucpi[sbi.s_cg_loaded - 1];
    for (j = sbi.s_cg_loaded - 1; j > 0; j--) {
    sbi.s_cgno[j] = sbi.s_cgno[j-1];
    sbi.s_ucpi[j] = sbi.s_ucpi[j-1];
    }
    sbi.s_ucpi[0] = ucpi;
    if (unlikely(!ufs_read_cylinder (sb, cgno, 0))) {
    UFSD("EXIT (FAILED)\n");
    return core::ptr::null_mut();
    }
    }
    UFSD("EXIT\n");
    return sbi.s_ucpi[0];
    }
