//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/atheros/atl1e/atl1e_param.c
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
// Copyright(c) 2007 Atheros Corporation. All rights reserved.
//
// Derived from Intel e1000 driver
// Copyright(c) 1999 - 2005 Intel Corporation. All rights reserved.
//

// This is the only thing that needs to be changed to adjust the
// maximum number of ports that the driver can manage.
//
pub const ATL1E_MAX_NIC: c_int = 32;

pub const OPTION_DISABLED: c_int = 0;
pub const OPTION_ENABLED: c_int = 1;
// All parameters are treated the same, as an integer array of values.
// This macro just reduces the need to repeat the same declaration code
// over and over (plus this helps to avoid typo bugs).
//

    static int x[ATL1E_MAX_NIC + 1] = ATL1E_PARAM_INIT; \
    static unsigned int num_##x; \
    module_param_array_named(x, x, int, &num_##x, 0); \
    MODULE_PARM_DESC(x, desc);
// Transmit Memory count
//
// Valid Range: 64-2048
//
// Default Value: 128
//
pub const ATL1E_MIN_TX_DESC_CNT: c_int = 32;
pub const ATL1E_MAX_TX_DESC_CNT: c_int = 1020;
pub const ATL1E_DEFAULT_TX_DESC_CNT: c_int = 128;
    ATL1E_PARAM(tx_desc_cnt, "Transmit description count");
// Receive Memory Block Count
//
// Valid Range: 16-512
//
// Default Value: 128
//

    ATL1E_PARAM(rx_mem_size, "memory size of rx buffer(KB)");
// User Specified MediaType Override
//
// Valid Range: 0-5
// - 0    - auto-negotiate at all supported speeds
// - 1    - only link at 100Mbps Full Duplex
// - 2    - only link at 100Mbps Half Duplex
// - 3    - only link at 10Mbps Full Duplex
// - 4    - only link at 10Mbps Half Duplex
// Default Value: 0
//
    ATL1E_PARAM(media_type, "MediaType Select");
// Interrupt Moderate Timer in units of 2 us
//
// Valid Range: 10-65535
//
// Default Value: 45000(90ms)
//

pub const INT_MOD_MAX_CNT: c_int = 65000;
pub const INT_MOD_MIN_CNT: c_int = 50;
    ATL1E_PARAM(int_mod_timer, "Interrupt Moderator Timer");
pub const AUTONEG_ADV_DEFAULT: c_uint = 0x2F;
pub const AUTONEG_ADV_MASK: c_uint = 0x2F;

pub const FLASH_VENDOR_DEFAULT: c_int = 0;
pub const FLASH_VENDOR_MIN: c_int = 0;
pub const FLASH_VENDOR_MAX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1e_option {
    pub type: enum { enable_option, range_option, list_option },
    pub name: *mut c_char,
    pub err: *mut c_char,
    pub def: c_int,
    union {
    struct { /* range_option info */
    pub min: c_int,
    pub max: c_int,
    pub r: },
    struct { /* list_option info */
    pub nr: c_int,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1e_opt_list {
    pub l: },
    pub arg: },
}

    static int atl1e_validate_option(int *value, struct atl1e_option *opt,
    struct atl1e_adapter *adapter)
    {
    if (*value == OPTION_UNSET) {
// value = opt->def;
    return 0;
    }
    switch (opt.type) {
    case enable_option:
    switch (*value) {
    case OPTION_ENABLED:
    netdev_info(adapter.netdev,
    "%s Enabled\n", opt.name);
    return 0;
    case OPTION_DISABLED:
    netdev_info(adapter.netdev,
    "%s Disabled\n", opt.name);
    return 0;
    }
    break;
    case range_option:
    if (*value >= opt.arg.r.min && *value <= opt.arg.r.max) {
    netdev_info(adapter.netdev, "%s set to %i\n",
    opt.name, *value);
    return 0;
    }
    break;
    case list_option:{
    int i;
    struct atl1e_opt_list *ent;
    for (i = 0; i < opt.arg.l.nr; i++) {
    ent = &opt.arg.l.p[i];
    if (*value == ent.i) {
    if (ent.str[0] != '\0')
    netdev_info(adapter.netdev,
    "%s\n", ent.str);
    return 0;
    }
    }
    break;
    }
    default:
    BUG();
    }
    netdev_info(adapter.netdev, "Invalid %s specified (%i) %s\n",
    opt.name, *value, opt.err);
// value = opt->def;
    return -1;
    }
//
// atl1e_check_options - Range Checking for Command Line Parameters
// @adapter: board private structure
//
// This routine checks all command line parameters for valid user
// input.  If an invalid value is given, or if no user specified
// value exists, a default value is used.  The final value is stored
// in a variable in the adapter structure.
//
#[no_mangle]
pub unsafe extern "C" fn atl1e_check_options(adapter: *mut atl1e_adapter) {
    void atl1e_check_options(struct atl1e_adapter *adapter)
    {
    let mut bd: c_int = adapter.bd_number;
    if (bd >= ATL1E_MAX_NIC) {
    netdev_notice(adapter.netdev,
    "no configuration for board #%i\n", bd);
    netdev_notice(adapter.netdev,
    "Using defaults for all values\n");
    }
    { 		/* Transmit Ring Size */
    struct atl1e_option opt = {
    .type = range_option,
    .name = "Transmit Ddescription Count",
    .err  = "using default of "
    __MODULE_STRING(ATL1E_DEFAULT_TX_DESC_CNT),
    .def  = ATL1E_DEFAULT_TX_DESC_CNT,
    .arg  = { .r = { .min = ATL1E_MIN_TX_DESC_CNT,
    .max = ATL1E_MAX_TX_DESC_CNT} }
    };
    int val;
    if (num_tx_desc_cnt > bd) {
    val = tx_desc_cnt[bd];
    atl1e_validate_option(&val, &opt, adapter);
    adapter.tx_ring.count = (u16) val & 0xFFFC;
    } else
    adapter.tx_ring.count = (u16)opt.def;
    }
    { 		/* Receive Memory Block Count */
    struct atl1e_option opt = {
    .type = range_option,
    .name = "Memory size of rx buffer(KB)",
    .err  = "using default of "
    __MODULE_STRING(ATL1E_DEFAULT_RX_MEM_SIZE),
    .def  = ATL1E_DEFAULT_RX_MEM_SIZE,
    .arg  = { .r = { .min = ATL1E_MIN_RX_MEM_SIZE,
    .max = ATL1E_MAX_RX_MEM_SIZE} }
    };
    int val;
    if (num_rx_mem_size > bd) {
    val = rx_mem_size[bd];
    atl1e_validate_option(&val, &opt, adapter);
    adapter.rx_ring.page_size = (u32)val * 1024;
    } else {
    adapter.rx_ring.page_size = (u32)opt.def * 1024;
    }
    }
    { 		/* Interrupt Moderate Timer */
    struct atl1e_option opt = {
    .type = range_option,
    .name = "Interrupt Moderate Timer",
    .err  = "using default of "
    __MODULE_STRING(INT_MOD_DEFAULT_CNT),
    .def  = INT_MOD_DEFAULT_CNT,
    .arg  = { .r = { .min = INT_MOD_MIN_CNT,
    .max = INT_MOD_MAX_CNT} }
    } ;
    int val;
    if (num_int_mod_timer > bd) {
    val = int_mod_timer[bd];
    atl1e_validate_option(&val, &opt, adapter);
    adapter.hw.imt = (u16) val;
    } else
    adapter.hw.imt = (u16)(opt.def);
    }
    { 		/* MediaType */
    struct atl1e_option opt = {
    .type = range_option,
    .name = "Speed/Duplex Selection",
    .err  = "using default of "
    __MODULE_STRING(MEDIA_TYPE_AUTO_SENSOR),
    .def  = MEDIA_TYPE_AUTO_SENSOR,
    .arg  = { .r = { .min = MEDIA_TYPE_AUTO_SENSOR,
    .max = MEDIA_TYPE_10M_HALF} }
    } ;
    int val;
    if (num_media_type > bd) {
    val = media_type[bd];
    atl1e_validate_option(&val, &opt, adapter);
    adapter.hw.media_type = (u16) val;
    } else
    adapter.hw.media_type = (u16)(opt.def);
    }
    }
