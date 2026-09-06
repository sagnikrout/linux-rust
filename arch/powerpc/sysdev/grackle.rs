//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/grackle.c
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
// Functions for setting up and using a MPC106 northbridge
// Extracted from arch/powerpc/platforms/powermac/pci.c.
//
// Copyright (C) 2003 Benjamin Herrenschmuidt (benh@kernel.crashing.org)
// Copyright (C) 1997 Paul Mackerras (paulus@samba.org)
//

    | (((o) & ~3) << 24))
pub const GRACKLE_PICR1_LOOPSNOOP: c_uint = 0x00000010;
#[no_mangle]
pub unsafe extern "C" fn grackle_set_loop_snoop(bp: *mut pci_controller, enable: c_int) {
    static inline void grackle_set_loop_snoop(struct pci_controller *bp, int enable)
    {
    unsigned int val;
    out_be32(bp.cfg_addr, GRACKLE_CFA(0, 0, 0xa8));
    val = in_le32(bp.cfg_data);
    val = enable? (val | GRACKLE_PICR1_LOOPSNOOP) :
    (val & ~GRACKLE_PICR1_LOOPSNOOP);
    out_be32(bp.cfg_addr, GRACKLE_CFA(0, 0, 0xa8));
    out_le32(bp.cfg_data, val);
    (void)in_le32(bp.cfg_data);
    }
#[no_mangle]
pub unsafe extern "C" fn setup_grackle(hose: *mut pci_controller) -> void __init {
    void __init setup_grackle(struct pci_controller *hose)
    {
    setup_indirect_pci(hose, 0xfec00000, 0xfee00000, 0);
    if (of_machine_is_compatible("PowerMac1,1"))
    pci_add_flags(PCI_REASSIGN_ALL_BUS);
    if (of_machine_is_compatible("AAPL,PowerBook1998"))
    grackle_set_loop_snoop(hose, 1);
    }
