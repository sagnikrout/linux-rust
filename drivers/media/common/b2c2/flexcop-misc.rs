//! Automatically rewritten from C to Rust
//! Source: drivers/media/common/b2c2/flexcop-misc.c
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
// Linux driver for digital TV devices equipped with B2C2 FlexcopII(b)/III
// flexcop-misc.c - miscellaneous functions
// see flexcop.c for copyright information
//

#[no_mangle]
pub unsafe extern "C" fn flexcop_determine_revision(fc: *mut flexcop_device) {
    void flexcop_determine_revision(struct flexcop_device *fc)
    {
    let mut v: flexcop_ibi_value = fc.read_ibi_reg(fc,misc_204);
    switch (v.misc_204.Rev_N_sig_revision_hi) {
    case 0x2:
    deb_info("found a FlexCopII.\n");
    fc.rev = FLEXCOP_II;
    break;
    case 0x3:
    deb_info("found a FlexCopIIb.\n");
    fc.rev = FLEXCOP_IIB;
    break;
    case 0x0:
    deb_info("found a FlexCopIII.\n");
    fc.rev = FLEXCOP_III;
    break;
    default:
    err("unknown FlexCop Revision: %x. Please report this to linux-dvb@linuxtv.org.",
    v.misc_204.Rev_N_sig_revision_hi);
    break;
    }
    if ((fc.has_32_hw_pid_filter = v.misc_204.Rev_N_sig_caps))
    deb_info("this FlexCop has the additional 32 hardware pid filter.\n");
    else
    deb_info("this FlexCop has the 6 basic main hardware pid filter.\n");
// bus parts have to decide if hw pid filtering is used or not.
    }
    static const char *flexcop_revision_names[] = {
    "Unknown chip",
    "FlexCopII",
    "FlexCopIIb",
    "FlexCopIII",
    };
    static const char *flexcop_device_names[] = {
    [FC_UNK]	= "Unknown device",
    [FC_CABLE]	= "Cable2PC/CableStar 2 DVB-C",
    [FC_AIR_DVBT]	= "Air2PC/AirStar 2 DVB-T",
    [FC_AIR_ATSC1]	= "Air2PC/AirStar 2 ATSC 1st generation",
    [FC_AIR_ATSC2]	= "Air2PC/AirStar 2 ATSC 2nd generation",
    [FC_AIR_ATSC3]	= "Air2PC/AirStar 2 ATSC 3rd generation (HD5000)",
    [FC_SKY_REV23]	= "Sky2PC/SkyStar 2 DVB-S rev 2.3 (old version)",
    [FC_SKY_REV26]	= "Sky2PC/SkyStar 2 DVB-S rev 2.6",
    [FC_SKY_REV27]	= "Sky2PC/SkyStar 2 DVB-S rev 2.7a/u",
    [FC_SKY_REV28]	= "Sky2PC/SkyStar 2 DVB-S rev 2.8",
    [FC_SKYS2_REV33] = "Sky2PC/SkyStar S2 DVB-S/S2 rev 3.3",
    };
    static const char *flexcop_bus_names[] = {
    "USB",
    "PCI",
    };
    void flexcop_device_name(struct flexcop_device *fc,
    const char *prefix, const char *suffix)
    {
    info("%s '%s' at the '%s' bus controlled by a '%s' %s",
    prefix,	flexcop_device_names[fc.dev_type],
    flexcop_bus_names[fc.bus_type],
    flexcop_revision_names[fc.rev], suffix);
    }
