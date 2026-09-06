//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/ptdump/bats.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2018, Christophe Leroy CS S.I.
// <christophe.leroy@c-s.fr>
//
// This dumps the content of BATS
//

#[no_mangle]
unsafe extern "C" fn bat_show_603(m: *mut seq_file, idx: c_int, lower: u32, upper: u32, is_d: bool) {
    static void bat_show_603(struct seq_file *m, int idx, u32 lower, u32 upper, bool is_d)
    {
    let mut bepi: u32 = upper & 0xfffe0000;
    let mut bl: u32 = (upper >> 2) & 0x7ff;
    let mut k: u32 = upper & 3;
    let mut brpn: phys_addr_t = PHYS_BAT_ADDR(lower);
    let mut size: u32 = (bl + 1) << 17;
    seq_printf(m, "%d: ", idx);
    if (k == 0) {
    seq_puts(m, "        -\n");
    return;
    }
    seq_printf(m, "0x%08x-0x%08x ", bepi, bepi + size - 1);

    seq_printf(m, "0x%016llx ", brpn);

    seq_printf(m, "0x%08x ", brpn);

    pt_dump_size(m, size);
    if (k == 1)
    seq_puts(m, "User ");
#[no_mangle]
pub unsafe extern "C" fn if(2: k ==) -> else {
    else if (k == 2)
    seq_puts(m, "Kernel ");
    else
    seq_puts(m, "Kernel/User ");
    if (lower & BPP_RX)
    seq_puts(m, is_d ? "r   " : "  x ");
#[no_mangle]
pub unsafe extern "C" fn if(BPP_RW: lower &) -> else {
    else if (lower & BPP_RW)
    seq_puts(m, is_d ? "rw  " : "  x ");
    else
    seq_puts(m, is_d ? "    " : "    ");
    seq_puts(m, lower & _PAGE_WRITETHRU ? "w " : "  ");
    seq_puts(m, lower & _PAGE_NO_CACHE ? "i " : "  ");
    seq_puts(m, lower & _PAGE_COHERENT ? "m " : "  ");
    seq_puts(m, lower & _PAGE_GUARDED ? "g " : "  ");
    seq_puts(m, "\n");
    }

#[no_mangle]
unsafe extern "C" fn bats_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int bats_show(struct seq_file *m, void *v)
    {
    seq_puts(m, "---[ Instruction Block Address Translation ]---\n");
    BAT_SHOW_603(m, 0, SPRN_IBAT0L, SPRN_IBAT0U, false);
    BAT_SHOW_603(m, 1, SPRN_IBAT1L, SPRN_IBAT1U, false);
    BAT_SHOW_603(m, 2, SPRN_IBAT2L, SPRN_IBAT2U, false);
    BAT_SHOW_603(m, 3, SPRN_IBAT3L, SPRN_IBAT3U, false);
    if (mmu_has_feature(MMU_FTR_USE_HIGH_BATS)) {
    BAT_SHOW_603(m, 4, SPRN_IBAT4L, SPRN_IBAT4U, false);
    BAT_SHOW_603(m, 5, SPRN_IBAT5L, SPRN_IBAT5U, false);
    BAT_SHOW_603(m, 6, SPRN_IBAT6L, SPRN_IBAT6U, false);
    BAT_SHOW_603(m, 7, SPRN_IBAT7L, SPRN_IBAT7U, false);
    }
    seq_puts(m, "\n---[ Data Block Address Translation ]---\n");
    BAT_SHOW_603(m, 0, SPRN_DBAT0L, SPRN_DBAT0U, true);
    BAT_SHOW_603(m, 1, SPRN_DBAT1L, SPRN_DBAT1U, true);
    BAT_SHOW_603(m, 2, SPRN_DBAT2L, SPRN_DBAT2U, true);
    BAT_SHOW_603(m, 3, SPRN_DBAT3L, SPRN_DBAT3U, true);
    if (mmu_has_feature(MMU_FTR_USE_HIGH_BATS)) {
    BAT_SHOW_603(m, 4, SPRN_DBAT4L, SPRN_DBAT4U, true);
    BAT_SHOW_603(m, 5, SPRN_DBAT5L, SPRN_DBAT5U, true);
    BAT_SHOW_603(m, 6, SPRN_DBAT6L, SPRN_DBAT6U, true);
    BAT_SHOW_603(m, 7, SPRN_DBAT7L, SPRN_DBAT7U, true);
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(bats);
#[no_mangle]
unsafe extern "C" fn bats_init() -> int __init {
    static int __init bats_init(void)
    {
    debugfs_create_file("block_address_translation", 0400,
    arch_debugfs_dir, core::ptr::null_mut(), &bats_fops);
    return 0;
    }
    device_initcall(bats_init);
