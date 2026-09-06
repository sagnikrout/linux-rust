//! Automatically rewritten from C to Rust
//! Source: drivers/soc/pxa/mfp.c
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
// linux/arch/arm/plat-pxa/mfp.c
//
// Multi-Function Pin Support
//
// Copyright (C) 2007 Marvell Internation Ltd.
//
// 2007-08-21: eric miao <eric.miao@marvell.com>
// initial version
//

// MFPR register bit definitions

//
// Table that determines the low power modes outputs, with actual settings
// used in parentheses for don't-care values. Except for the float output,
// the configured driven and pulled levels match, so if there is a need for
// non-LPM pulled output, the same configuration could probably be used.
//
// Output value  sleep_oe_n  sleep_data  pullup_en  pulldown_en  pull_sel
// (bit 7)    (bit 8)    (bit 14)     (bit 13)   (bit 15)
//
// Input            0          X(0)        X(0)        X(0)       0
// Drive 0          0          0           0           X(1)       0
// Drive 1          0          1           X(1)        0	  0
// Pull hi (1)      1          X(1)        1           0	  0
// Pull lo (0)      1          X(0)        0           1	  0
// Z (float)        1          X(0)        0           0	  0
//

//
// The pullup and pulldown state of the MFP pin at run mode is by default
// determined by the selected alternate function. In case that some buggy
// devices need to override this default behavior,  the definitions below
// indicates the setting of corresponding MFPR bits
//
// Definition       pull_sel  pullup_en  pulldown_en
// MFPR_PULL_NONE       0         0        0
// MFPR_PULL_LOW        1         0        1
// MFPR_PULL_HIGH       1         1        0
// MFPR_PULL_BOTH       1         1        1
// MFPR_PULL_FLOAT	1         0        0
//

// mfp_spin_lock is used to ensure that MFP register configuration
// (most likely a read-modify-write operation) is atomic, and that
// mfp_table[] is consistent
//
    static DEFINE_SPINLOCK(mfp_spin_lock);
    static void __iomem *mfpr_mmio_base;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfp_pin {
    pub /: *mut *mut unsigned long config; / -1 for not configured,
    pub /: *mut *mut unsigned long mfpr_off; / MFPRxx Register offset,
    pub /: *mut *mut unsigned long mfpr_run; / Run-Mode Register Value,
    pub /: *mut *mut unsigned long mfpr_lpm; / Low Power Mode Register Value,
}

    static struct mfp_pin mfp_table[MFP_PIN_MAX];
// mapping of MFP_LPM_* definitions to MFPR_LPM_* register bits
    static const unsigned long mfpr_lpm[] = {
    MFPR_LPM_INPUT,
    MFPR_LPM_DRIVE_LOW,
    MFPR_LPM_DRIVE_HIGH,
    MFPR_LPM_PULL_LOW,
    MFPR_LPM_PULL_HIGH,
    MFPR_LPM_FLOAT,
    MFPR_LPM_INPUT,
    };
// mapping of MFP_PULL_* definitions to MFPR_PULL_* register bits
    static const unsigned long mfpr_pull[] = {
    MFPR_PULL_NONE,
    MFPR_PULL_LOW,
    MFPR_PULL_HIGH,
    MFPR_PULL_BOTH,
    MFPR_PULL_FLOAT,
    };
// mapping of MFP_LPM_EDGE_* definitions to MFPR_EDGE_* register bits
    static const unsigned long mfpr_edge[] = {
    MFPR_EDGE_NONE,
    MFPR_EDGE_RISE,
    MFPR_EDGE_FALL,
    MFPR_EDGE_BOTH,
    };

    __raw_readl(mfpr_mmio_base + (off))

    __raw_writel(val, mfpr_mmio_base + (off))

//
// perform a read-back of any valid MFPR register to make sure the
// previous writings are finished
//
    static unsigned long mfpr_off_readback;

#[no_mangle]
pub unsafe extern "C" fn __mfp_config_run(p: *mut mfp_pin) {
    static inline void __mfp_config_run(struct mfp_pin *p)
    {
    if (mfp_configured(p))
    mfpr_writel(p.mfpr_off, p.mfpr_run);
    }
#[no_mangle]
pub unsafe extern "C" fn __mfp_config_lpm(p: *mut mfp_pin) {
    static inline void __mfp_config_lpm(struct mfp_pin *p)
    {
    if (mfp_configured(p)) {
    let mut mfpr_clr: c_ulong = (p.mfpr_run & ~MFPR_EDGE_BOTH) | MFPR_EDGE_CLEAR;
    if (mfpr_clr != p.mfpr_run)
    mfpr_writel(p.mfpr_off, mfpr_clr);
    if (p.mfpr_lpm != mfpr_clr)
    mfpr_writel(p.mfpr_off, p.mfpr_lpm);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mfp_config(mfp_cfgs: *mut c_ulong, num: c_int) {
    void mfp_config(unsigned long *mfp_cfgs, int num)
    {
    unsigned long flags;
    int i;
    spin_lock_irqsave(&mfp_spin_lock, flags);
    for (i = 0; i < num; i++, mfp_cfgs++) {
    unsigned long tmp, c = *mfp_cfgs;
    struct mfp_pin *p;
    int pin, af, drv, lpm, edge, pull;
    pin = MFP_PIN(c);
    BUG_ON(pin >= MFP_PIN_MAX);
    p = &mfp_table[pin];
    af  = MFP_AF(c);
    drv = MFP_DS(c);
    lpm = MFP_LPM_STATE(c);
    edge = MFP_LPM_EDGE(c);
    pull = MFP_PULL(c);
// run-mode pull settings will conflict with MFPR bits of
// low power mode state,  calculate mfpr_run and mfpr_lpm
// individually if pull != MFP_PULL_NONE
//
    tmp = MFPR_AF_SEL(af) | MFPR_DRIVE(drv);
    if (likely(pull == MFP_PULL_NONE)) {
    p.mfpr_run = tmp | mfpr_lpm[lpm] | mfpr_edge[edge];
    p.mfpr_lpm = p.mfpr_run;
    } else {
    p.mfpr_lpm = tmp | mfpr_lpm[lpm] | mfpr_edge[edge];
    p.mfpr_run = tmp | mfpr_pull[pull];
    }
    p.config = c; __mfp_config_run(p);
    }
    mfpr_sync();
    spin_unlock_irqrestore(&mfp_spin_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn mfp_read(mfp: c_int) -> c_ulong {
    unsigned long mfp_read(int mfp)
    {
    unsigned long val, flags;
    BUG_ON(mfp < 0 || mfp >= MFP_PIN_MAX);
    spin_lock_irqsave(&mfp_spin_lock, flags);
    val = mfpr_readl(mfp_table[mfp].mfpr_off);
    spin_unlock_irqrestore(&mfp_spin_lock, flags);
    return val;
    }
#[no_mangle]
pub unsafe extern "C" fn mfp_write(mfp: c_int, val: c_ulong) {
    void mfp_write(int mfp, unsigned long val)
    {
    unsigned long flags;
    BUG_ON(mfp < 0 || mfp >= MFP_PIN_MAX);
    spin_lock_irqsave(&mfp_spin_lock, flags);
    mfpr_writel(mfp_table[mfp].mfpr_off, val);
    mfpr_sync();
    spin_unlock_irqrestore(&mfp_spin_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn mfp_init_base(mfpr_base: *mut void __iomem) -> void __init {
    void __init mfp_init_base(void __iomem *mfpr_base)
    {
    int i;
// initialize the table with default - unconfigured
    for (i = 0; i < ARRAY_SIZE(mfp_table); i++)
    mfp_table[i].config = -1;
    mfpr_mmio_base = mfpr_base;
    }
#[no_mangle]
pub unsafe extern "C" fn mfp_init_addr(map: *mut mfp_addr_map) -> void __init {
    void __init mfp_init_addr(struct mfp_addr_map *map)
    {
    struct mfp_addr_map *p;
    unsigned long offset, flags;
    int i;
    spin_lock_irqsave(&mfp_spin_lock, flags);
// mfp offset for readback
    mfpr_off_readback = map[0].offset;
    for (p = map; p.start != MFP_PIN_INVALID; p++) {
    offset = p.offset;
    i = p.start;
    do {
    mfp_table[i].mfpr_off = offset;
    mfp_table[i].mfpr_run = 0;
    mfp_table[i].mfpr_lpm = 0;
    offset += 4; i++;
    } while ((i <= p.end) && (p.end != -1));
    }
    spin_unlock_irqrestore(&mfp_spin_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn mfp_config_lpm() {
    void mfp_config_lpm(void)
    {
    struct mfp_pin *p = &mfp_table[0];
    int pin;
    for (pin = 0; pin < ARRAY_SIZE(mfp_table); pin++, p++)
    __mfp_config_lpm(p);
    }
#[no_mangle]
pub unsafe extern "C" fn mfp_config_run() {
    void mfp_config_run(void)
    {
    struct mfp_pin *p = &mfp_table[0];
    int pin;
    for (pin = 0; pin < ARRAY_SIZE(mfp_table); pin++, p++)
    __mfp_config_run(p);
    }
