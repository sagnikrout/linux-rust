//! Automatically rewritten from C to Rust
//! Source: drivers/soc/fsl/qe/qe_tdm.c
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
// Copyright (C) 2015 Freescale Semiconductor, Inc. All rights reserved.
//
// Authors:	Zhao Qiang <qiang.zhao@nxp.com>
//
// Description:
// QE TDM API Set - TDM specific routines implementations.
//

#[no_mangle]
unsafe extern "C" fn set_tdm_framer(tdm_framer_type: *const c_char) -> c_int {
    static int set_tdm_framer(const char *tdm_framer_type)
    {
    if (strcmp(tdm_framer_type, "e1") == 0)
    return TDM_FRAMER_E1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(tdm_framer_type, 0: "t1") ==) -> else {
    else if (strcmp(tdm_framer_type, "t1") == 0)
    return TDM_FRAMER_T1;
    else
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn set_si_param(utdm: *mut ucc_tdm, ut_info: *mut ucc_tdm_info) {
    static void set_si_param(struct ucc_tdm *utdm, struct ucc_tdm_info *ut_info)
    {
    struct si_mode_info *si_info = &ut_info.si_info;
    if (utdm.tdm_mode == TDM_INTERNAL_LOOPBACK) {
    si_info.simr_crt = 1;
    si_info.simr_rfsd = 0;
    }
    }
    int ucc_of_parse_tdm(struct device_node *np, struct ucc_tdm *utdm,
    struct ucc_tdm_info *ut_info)
    {
    const char *sprop;
    let mut ret: c_int = 0;
    u32 val;
    sprop = of_get_property(np, "fsl,rx-sync-clock", core::ptr::null_mut());
    if (sprop) {
    ut_info.uf_info.rx_sync = qe_clock_source(sprop);
    if ((ut_info.uf_info.rx_sync < QE_CLK_NONE) ||
    (ut_info.uf_info.rx_sync > QE_RSYNC_PIN)) {
    pr_err("QE-TDM: Invalid rx-sync-clock property\n");
    return -EINVAL;
    }
    } else {
    pr_err("QE-TDM: Invalid rx-sync-clock property\n");
    return -EINVAL;
    }
    sprop = of_get_property(np, "fsl,tx-sync-clock", core::ptr::null_mut());
    if (sprop) {
    ut_info.uf_info.tx_sync = qe_clock_source(sprop);
    if ((ut_info.uf_info.tx_sync < QE_CLK_NONE) ||
    (ut_info.uf_info.tx_sync > QE_TSYNC_PIN)) {
    pr_err("QE-TDM: Invalid tx-sync-clock property\n");
    return -EINVAL;
    }
    } else {
    pr_err("QE-TDM: Invalid tx-sync-clock property\n");
    return -EINVAL;
    }
    ret = of_property_read_u32_index(np, "fsl,tx-timeslot-mask", 0, &val);
    if (ret) {
    pr_err("QE-TDM: Invalid tx-timeslot-mask property\n");
    return -EINVAL;
    }
    utdm.tx_ts_mask = val;
    ret = of_property_read_u32_index(np, "fsl,rx-timeslot-mask", 0, &val);
    if (ret) {
    ret = -EINVAL;
    pr_err("QE-TDM: Invalid rx-timeslot-mask property\n");
    return ret;
    }
    utdm.rx_ts_mask = val;
    ret = of_property_read_u32_index(np, "fsl,tdm-id", 0, &val);
    if (ret) {
    ret = -EINVAL;
    pr_err("QE-TDM: No fsl,tdm-id property for this UCC\n");
    return ret;
    }
    utdm.tdm_port = val;
    ut_info.uf_info.tdm_num = utdm.tdm_port;
    if (of_property_read_bool(np, "fsl,tdm-internal-loopback"))
    utdm.tdm_mode = TDM_INTERNAL_LOOPBACK;
    else
    utdm.tdm_mode = TDM_NORMAL;
    sprop = of_get_property(np, "fsl,tdm-framer-type", core::ptr::null_mut());
    if (!sprop) {
    ret = -EINVAL;
    pr_err("QE-TDM: No tdm-framer-type property for UCC\n");
    return ret;
    }
    ret = set_tdm_framer(sprop);
    if (ret < 0)
    return -EINVAL;
    utdm.tdm_framer_type = ret;
    ret = of_property_read_u32_index(np, "fsl,siram-entry-id", 0, &val);
    if (ret) {
    ret = -EINVAL;
    pr_err("QE-TDM: No siram entry id for UCC\n");
    return ret;
    }
    utdm.siram_entry_id = val;
    set_si_param(utdm, ut_info);
    return ret;
    }
    EXPORT_SYMBOL(ucc_of_parse_tdm);
#[no_mangle]
pub unsafe extern "C" fn ucc_tdm_init(utdm: *mut ucc_tdm, ut_info: *mut ucc_tdm_info) {
    void ucc_tdm_init(struct ucc_tdm *utdm, struct ucc_tdm_info *ut_info)
    {
    struct si1 __iomem *si_regs;
    u16 __iomem *siram;
    u16 siram_entry_valid;
    u16 siram_entry_closed;
    u16 ucc_num;
    u8 csel;
    u16 sixmr;
    u16 tdm_port;
    u32 siram_entry_id;
    u32 mask;
    int i;
    si_regs = utdm.si_regs;
    siram = utdm.siram;
    ucc_num = ut_info.uf_info.ucc_num;
    tdm_port = utdm.tdm_port;
    siram_entry_id = utdm.siram_entry_id;
    if (utdm.tdm_framer_type == TDM_FRAMER_T1)
    utdm.num_of_ts = 24;
    if (utdm.tdm_framer_type == TDM_FRAMER_E1)
    utdm.num_of_ts = 32;
// set siram table
    csel = (ucc_num < 4) ? ucc_num + 9 : ucc_num - 3;
    siram_entry_valid = SIR_CSEL(csel) | SIR_BYTE | SIR_CNT(0);
    siram_entry_closed = SIR_IDLE | SIR_BYTE | SIR_CNT(0);
    for (i = 0; i < utdm.num_of_ts; i++) {
    mask = 0x01 << i;
    if (utdm.tx_ts_mask & mask)
    iowrite16be(siram_entry_valid,
    &siram[siram_entry_id * 32 + i]);
    else
    iowrite16be(siram_entry_closed,
    &siram[siram_entry_id * 32 + i]);
    if (utdm.rx_ts_mask & mask)
    iowrite16be(siram_entry_valid,
    &siram[siram_entry_id * 32 + 0x200 +  i]);
    else
    iowrite16be(siram_entry_closed,
    &siram[siram_entry_id * 32 + 0x200 +  i]);
    }
    qe_setbits_be16(&siram[(siram_entry_id * 32) + (utdm.num_of_ts - 1)],
    SIR_LAST);
    qe_setbits_be16(&siram[(siram_entry_id * 32) + 0x200 + (utdm.num_of_ts - 1)],
    SIR_LAST);
// Set SIxMR register
    sixmr = SIMR_SAD(siram_entry_id);
    sixmr &= ~SIMR_SDM_MASK;
    if (utdm.tdm_mode == TDM_INTERNAL_LOOPBACK)
    sixmr |= SIMR_SDM_INTERNAL_LOOPBACK;
    else
    sixmr |= SIMR_SDM_NORMAL;
    sixmr |= SIMR_RFSD(ut_info.si_info.simr_rfsd) |
    SIMR_TFSD(ut_info.si_info.simr_tfsd);
    if (ut_info.si_info.simr_crt)
    sixmr |= SIMR_CRT;
    if (ut_info.si_info.simr_sl)
    sixmr |= SIMR_SL;
    if (ut_info.si_info.simr_ce)
    sixmr |= SIMR_CE;
    if (ut_info.si_info.simr_fe)
    sixmr |= SIMR_FE;
    if (ut_info.si_info.simr_gm)
    sixmr |= SIMR_GM;
    switch (tdm_port) {
    case 0:
    iowrite16be(sixmr, &si_regs.sixmr1[0]);
    break;
    case 1:
    iowrite16be(sixmr, &si_regs.sixmr1[1]);
    break;
    case 2:
    iowrite16be(sixmr, &si_regs.sixmr1[2]);
    break;
    case 3:
    iowrite16be(sixmr, &si_regs.sixmr1[3]);
    break;
    default:
    pr_err("QE-TDM: can not find tdm sixmr reg\n");
    break;
    }
    }
    EXPORT_SYMBOL(ucc_tdm_init);
