//! Automatically rewritten from C to Rust
//! Source: drivers/clk/bcm/clk-bcm2835.c
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
// Copyright (C) 2010,2015 Broadcom
// Copyright (C) 2012 Stephen Warren
//
// DOC: BCM2835 CPRMAN (clock manager for the "audio" domain)
//
// The clock tree on the 2835 has several levels.  There's a root
// oscillator running at 19.2Mhz.  After the oscillator there are 5
// PLLs, roughly divided as "camera", "ARM", "core", "DSI displays",
// and "HDMI displays".  Those 5 PLLs each can divide their output to
// produce up to 4 channels.  Finally, there is the level of clocks to
// be consumed by other hardware components (like "H264" or "HDMI
// state machine"), which divide off of some subset of the PLL
// channels.
//
// All of the clocks in the tree are exposed in the DT, because the DT
// may want to make assignments of the final layer of clocks to the
// PLL channels, and some components of the hardware will actually
// skip layers of the tree (for example, the pixel clock comes
// directly from the PLLH PIX channel without using a CM_*CTL clock
// generator).
//

pub const CM_PASSWORD: c_uint = 0x5a000000;
pub const CM_GNRICCTL: c_uint = 0x000;
pub const CM_GNRICDIV: c_uint = 0x004;

pub const CM_VPUCTL: c_uint = 0x008;
pub const CM_VPUDIV: c_uint = 0x00c;
pub const CM_SYSCTL: c_uint = 0x010;
pub const CM_SYSDIV: c_uint = 0x014;
pub const CM_PERIACTL: c_uint = 0x018;
pub const CM_PERIADIV: c_uint = 0x01c;
pub const CM_PERIICTL: c_uint = 0x020;
pub const CM_PERIIDIV: c_uint = 0x024;
pub const CM_H264CTL: c_uint = 0x028;
pub const CM_H264DIV: c_uint = 0x02c;
pub const CM_ISPCTL: c_uint = 0x030;
pub const CM_ISPDIV: c_uint = 0x034;
pub const CM_V3DCTL: c_uint = 0x038;
pub const CM_V3DDIV: c_uint = 0x03c;
pub const CM_CAM0CTL: c_uint = 0x040;
pub const CM_CAM0DIV: c_uint = 0x044;
pub const CM_CAM1CTL: c_uint = 0x048;
pub const CM_CAM1DIV: c_uint = 0x04c;
pub const CM_CCP2CTL: c_uint = 0x050;
pub const CM_CCP2DIV: c_uint = 0x054;
pub const CM_DSI0ECTL: c_uint = 0x058;
pub const CM_DSI0EDIV: c_uint = 0x05c;
pub const CM_DSI0PCTL: c_uint = 0x060;
pub const CM_DSI0PDIV: c_uint = 0x064;
pub const CM_DPICTL: c_uint = 0x068;
pub const CM_DPIDIV: c_uint = 0x06c;
pub const CM_GP0CTL: c_uint = 0x070;
pub const CM_GP0DIV: c_uint = 0x074;
pub const CM_GP1CTL: c_uint = 0x078;
pub const CM_GP1DIV: c_uint = 0x07c;
pub const CM_GP2CTL: c_uint = 0x080;
pub const CM_GP2DIV: c_uint = 0x084;
pub const CM_HSMCTL: c_uint = 0x088;
pub const CM_HSMDIV: c_uint = 0x08c;
pub const CM_OTPCTL: c_uint = 0x090;
pub const CM_OTPDIV: c_uint = 0x094;
pub const CM_PCMCTL: c_uint = 0x098;
pub const CM_PCMDIV: c_uint = 0x09c;
pub const CM_PWMCTL: c_uint = 0x0a0;
pub const CM_PWMDIV: c_uint = 0x0a4;
pub const CM_SLIMCTL: c_uint = 0x0a8;
pub const CM_SLIMDIV: c_uint = 0x0ac;
pub const CM_SMICTL: c_uint = 0x0b0;
pub const CM_SMIDIV: c_uint = 0x0b4;
// no definition for 0x0b8  and 0x0bc
pub const CM_TCNTCTL: c_uint = 0x0c0;

pub const CM_TCNTCNT: c_uint = 0x0c4;
pub const CM_TECCTL: c_uint = 0x0c8;
pub const CM_TECDIV: c_uint = 0x0cc;
pub const CM_TD0CTL: c_uint = 0x0d0;
pub const CM_TD0DIV: c_uint = 0x0d4;
pub const CM_TD1CTL: c_uint = 0x0d8;
pub const CM_TD1DIV: c_uint = 0x0dc;
pub const CM_TSENSCTL: c_uint = 0x0e0;
pub const CM_TSENSDIV: c_uint = 0x0e4;
pub const CM_TIMERCTL: c_uint = 0x0e8;
pub const CM_TIMERDIV: c_uint = 0x0ec;
pub const CM_UARTCTL: c_uint = 0x0f0;
pub const CM_UARTDIV: c_uint = 0x0f4;
pub const CM_VECCTL: c_uint = 0x0f8;
pub const CM_VECDIV: c_uint = 0x0fc;
pub const CM_PULSECTL: c_uint = 0x190;
pub const CM_PULSEDIV: c_uint = 0x194;
pub const CM_SDCCTL: c_uint = 0x1a8;
pub const CM_SDCDIV: c_uint = 0x1ac;
pub const CM_ARMCTL: c_uint = 0x1b0;
pub const CM_AVEOCTL: c_uint = 0x1b8;
pub const CM_AVEODIV: c_uint = 0x1bc;
pub const CM_EMMCCTL: c_uint = 0x1c0;
pub const CM_EMMCDIV: c_uint = 0x1c4;
pub const CM_EMMC2CTL: c_uint = 0x1d0;
pub const CM_EMMC2DIV: c_uint = 0x1d4;
// General bits for the CM_*CTL regs

pub const CM_OSCCOUNT: c_uint = 0x100;
pub const CM_PLLA: c_uint = 0x104;

pub const CM_PLLC: c_uint = 0x108;

pub const CM_PLLD: c_uint = 0x10c;

pub const CM_PLLH: c_uint = 0x110;

pub const CM_LOCK: c_uint = 0x114;

pub const CM_EVENT: c_uint = 0x118;
pub const CM_DSI1ECTL: c_uint = 0x158;
pub const CM_DSI1EDIV: c_uint = 0x15c;
pub const CM_DSI1PCTL: c_uint = 0x160;
pub const CM_DSI1PDIV: c_uint = 0x164;
pub const CM_DFTCTL: c_uint = 0x168;
pub const CM_DFTDIV: c_uint = 0x16c;
pub const CM_PLLB: c_uint = 0x170;

pub const A2W_PLLA_CTRL: c_uint = 0x1100;
pub const A2W_PLLC_CTRL: c_uint = 0x1120;
pub const A2W_PLLD_CTRL: c_uint = 0x1140;
pub const A2W_PLLH_CTRL: c_uint = 0x1160;
pub const A2W_PLLB_CTRL: c_uint = 0x11e0;

pub const A2W_PLLA_ANA0: c_uint = 0x1010;
pub const A2W_PLLC_ANA0: c_uint = 0x1030;
pub const A2W_PLLD_ANA0: c_uint = 0x1050;
pub const A2W_PLLH_ANA0: c_uint = 0x1070;
pub const A2W_PLLB_ANA0: c_uint = 0x10f0;
pub const A2W_PLL_KA_SHIFT: c_int = 7;

pub const A2W_PLL_KI_SHIFT: c_int = 19;

pub const A2W_PLL_KP_SHIFT: c_int = 15;

pub const A2W_PLLH_KA_SHIFT: c_int = 19;

pub const A2W_PLLH_KI_LOW_SHIFT: c_int = 22;

pub const A2W_PLLH_KI_HIGH_SHIFT: c_int = 0;

pub const A2W_PLLH_KP_SHIFT: c_int = 1;

pub const A2W_XOSC_CTRL: c_uint = 0x1190;

pub const A2W_PLLA_FRAC: c_uint = 0x1200;
pub const A2W_PLLC_FRAC: c_uint = 0x1220;
pub const A2W_PLLD_FRAC: c_uint = 0x1240;
pub const A2W_PLLH_FRAC: c_uint = 0x1260;
pub const A2W_PLLB_FRAC: c_uint = 0x12e0;

pub const A2W_PLL_DIV_BITS: c_int = 8;
pub const A2W_PLL_DIV_SHIFT: c_int = 0;
pub const A2W_PLLA_DSI0: c_uint = 0x1300;
pub const A2W_PLLA_CORE: c_uint = 0x1400;
pub const A2W_PLLA_PER: c_uint = 0x1500;
pub const A2W_PLLA_CCP2: c_uint = 0x1600;
pub const A2W_PLLC_CORE2: c_uint = 0x1320;
pub const A2W_PLLC_CORE1: c_uint = 0x1420;
pub const A2W_PLLC_PER: c_uint = 0x1520;
pub const A2W_PLLC_CORE0: c_uint = 0x1620;
pub const A2W_PLLD_DSI0: c_uint = 0x1340;
pub const A2W_PLLD_CORE: c_uint = 0x1440;
pub const A2W_PLLD_PER: c_uint = 0x1540;
pub const A2W_PLLD_DSI1: c_uint = 0x1640;
pub const A2W_PLLH_AUX: c_uint = 0x1360;
pub const A2W_PLLH_RCAL: c_uint = 0x1460;
pub const A2W_PLLH_PIX: c_uint = 0x1560;
pub const A2W_PLLH_STS: c_uint = 0x1660;
pub const A2W_PLLH_CTRLR: c_uint = 0x1960;
pub const A2W_PLLH_FRACR: c_uint = 0x1a60;
pub const A2W_PLLH_AUXR: c_uint = 0x1b60;
pub const A2W_PLLH_RCALR: c_uint = 0x1c60;
pub const A2W_PLLH_PIXR: c_uint = 0x1d60;
pub const A2W_PLLH_STSR: c_uint = 0x1e60;
pub const A2W_PLLB_ARM: c_uint = 0x13e0;
pub const A2W_PLLB_SP0: c_uint = 0x14e0;
pub const A2W_PLLB_SP1: c_uint = 0x15e0;
pub const A2W_PLLB_SP2: c_uint = 0x16e0;
pub const LOCK_TIMEOUT_NS: c_int = 100000000;

//
// Names of clocks used within the driver that need to be replaced
// with an external parent's name.  This array is in the order that
// the clocks node in the DT references external clocks.
//
    static const char *const cprman_parent_names[] = {
    "xosc",
    "dsi0_byte",
    "dsi0_ddr2",
    "dsi0_ddr",
    "dsi1_byte",
    "dsi1_ddr2",
    "dsi1_ddr",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_cprman {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub /: *mut *mut spinlock_t regs_lock; / spinlock for all clocks,
    pub soc: c_uint,
//
// Real names of cprman clock parents looked up through
// of_clk_get_parent_name(), which will be used in the
// parent_names[] arrays for clock registration.
//
    pub real_parent_names: [*const c_char; ARRAY_SIZE(cprman_parent_names)],
// Must be last
    pub onecell: clk_hw_onecell_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cprman_plat_data {
    pub soc: c_uint,
}

#[no_mangle]
pub unsafe extern "C" fn cprman_write(cprman: *mut bcm2835_cprman, reg: u32, val: u32) {
    static inline void cprman_write(struct bcm2835_cprman *cprman, u32 reg, u32 val)
    {
    writel(CM_PASSWORD | val, cprman.regs + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn cprman_read(cprman: *mut bcm2835_cprman, reg: u32) -> u32 {
    static inline u32 cprman_read(struct bcm2835_cprman *cprman, u32 reg)
    {
    return readl(cprman.regs + reg);
    }
// Does a cycle of measuring a clock through the TCNT clock, which may
// source from many other clocks in the system.
//
    static unsigned long bcm2835_measure_tcnt_mux(struct bcm2835_cprman *cprman,
    u32 tcnt_mux)
    {
    u32 osccount = 19200; /* 1ms */
    u32 count;
    ktime_t timeout;
    spin_lock(&cprman.regs_lock);
    cprman_write(cprman, CM_TCNTCTL, CM_KILL);
    cprman_write(cprman, CM_TCNTCTL,
    (tcnt_mux & CM_SRC_MASK) |
    (tcnt_mux >> CM_SRC_BITS) << CM_TCNT_SRC1_SHIFT);
    cprman_write(cprman, CM_OSCCOUNT, osccount);
// do a kind delay at the start
    mdelay(1);
// Finish off whatever is left of OSCCOUNT
    timeout = ktime_add_ns(ktime_get(), LOCK_TIMEOUT_NS);
    while (cprman_read(cprman, CM_OSCCOUNT)) {
    if (ktime_after(ktime_get(), timeout)) {
    dev_err(cprman.dev, "timeout waiting for OSCCOUNT\n");
    count = 0;
    goto out;
    }
    cpu_relax();
    }
// Wait for BUSY to clear.
    timeout = ktime_add_ns(ktime_get(), LOCK_TIMEOUT_NS);
    while (cprman_read(cprman, CM_TCNTCTL) & CM_BUSY) {
    if (ktime_after(ktime_get(), timeout)) {
    dev_err(cprman.dev, "timeout waiting for !BUSY\n");
    count = 0;
    goto out;
    }
    cpu_relax();
    }
    count = cprman_read(cprman, CM_TCNTCNT);
    cprman_write(cprman, CM_TCNTCTL, 0);
    out:
    spin_unlock(&cprman.regs_lock);
    return count * 1000;
    }
    static void bcm2835_debugfs_regset(struct bcm2835_cprman *cprman, u32 base,
    const struct debugfs_reg32 *regs,
    size_t nregs, struct dentry *dentry)
    {
    struct debugfs_regset32 *regset;
    regset = devm_kzalloc(cprman.dev, sizeof(*regset), GFP_KERNEL);
    if (!regset)
    return;
    regset.regs = regs;
    regset.nregs = nregs;
    regset.base = cprman.regs + base;
    debugfs_create_regset32("regdump", S_IRUGO, dentry, regset);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_pll_data {
    pub name: *const c_char,
    pub cm_ctrl_reg: u32,
    pub a2w_ctrl_reg: u32,
    pub frac_reg: u32,
    pub ana_reg_base: u32,
    pub reference_enable_mask: u32,
// Bit in CM_LOCK to indicate when the PLL has locked.
    pub lock_mask: u32,
    pub flags: u32,
    pub ana: *const bcm2835_pll_ana_bits,
    pub min_rate: c_ulong,
    pub max_rate: c_ulong,
//
// Highest rate for the VCO before we have to use the
// pre-divide-by-2.
//
    pub max_fb_rate: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_pll_ana_bits {
    pub mask0: u32,
    pub set0: u32,
    pub mask1: u32,
    pub set1: u32,
    pub mask3: u32,
    pub set3: u32,
    pub fb_prediv_mask: u32,
}

    static const struct bcm2835_pll_ana_bits bcm2835_ana_default = {
    .mask0 = 0,
    .set0 = 0,
    .mask1 = A2W_PLL_KI_MASK | A2W_PLL_KP_MASK,
    .set1 = (2 << A2W_PLL_KI_SHIFT) | (8 << A2W_PLL_KP_SHIFT),
    .mask3 = A2W_PLL_KA_MASK,
    .set3 = (2 << A2W_PLL_KA_SHIFT),
    .fb_prediv_mask = BIT(14),
    };
    static const struct bcm2835_pll_ana_bits bcm2835_ana_pllh = {
    .mask0 = A2W_PLLH_KA_MASK | A2W_PLLH_KI_LOW_MASK,
    .set0 = (2 << A2W_PLLH_KA_SHIFT) | (2 << A2W_PLLH_KI_LOW_SHIFT),
    .mask1 = A2W_PLLH_KI_HIGH_MASK | A2W_PLLH_KP_MASK,
    .set1 = (6 << A2W_PLLH_KP_SHIFT),
    .mask3 = 0,
    .set3 = 0,
    .fb_prediv_mask = BIT(11),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_pll_divider_data {
    pub name: *const c_char,
    pub source_pll: *const c_char,
    pub cm_reg: u32,
    pub a2w_reg: u32,
    pub load_mask: u32,
    pub hold_mask: u32,
    pub fixed_divider: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_clock_data {
    pub name: *const c_char,
    pub parents: *const *const c_char,
    pub num_mux_parents: c_int,
// Bitmap encoding which parents accept rate change propagation.
    pub set_rate_parent: c_uint,
    pub ctl_reg: u32,
    pub div_reg: u32,
// Number of integer bits in the divider
    pub int_bits: u32,
// Number of fractional bits in the divider
    pub frac_bits: u32,
    pub flags: u32,
    pub is_vpu_clock: bool,
    pub is_mash_clock: bool,
    pub low_jitter: bool,
    pub tcnt_mux: u32,
    pub round_up: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_gate_data {
    pub name: *const c_char,
    pub parent: *const c_char,
    pub ctl_reg: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_pll {
    pub hw: clk_hw,
    pub cprman: *mut bcm2835_cprman,
    pub data: *const bcm2835_pll_data,
}

#[no_mangle]
unsafe extern "C" fn bcm2835_pll_is_on(hw: *mut clk_hw) -> c_int {
    static int bcm2835_pll_is_on(struct clk_hw *hw)
    {
    struct bcm2835_pll *pll = container_of(hw, struct bcm2835_pll, hw);
    struct bcm2835_cprman *cprman = pll.cprman;
    const struct bcm2835_pll_data *data = pll.data;
    return cprman_read(cprman, data.a2w_ctrl_reg) &
    A2W_PLL_CTRL_PRST_DISABLE;
    }
    static u32 bcm2835_pll_get_prediv_mask(struct bcm2835_cprman *cprman,
    const struct bcm2835_pll_data *data)
    {
//
// On BCM2711 there isn't a pre-divisor available in the PLL feedback
// loop. Bits 13:14 of ANA1 (PLLA,PLLB,PLLC,PLLD) have been re-purposed
// for to for VCO RANGE bits.
//
    if (cprman.soc & SOC_BCM2711)
    return 0;
    return data.ana.fb_prediv_mask;
    }
    static void bcm2835_pll_choose_ndiv_and_fdiv(unsigned long rate,
    unsigned long parent_rate,
    u32 *ndiv, u32 *fdiv)
    {
    u64 div;
    div = (u64)rate << A2W_PLL_FRAC_BITS;
    do_div(div, parent_rate);
// ndiv = div >> A2W_PLL_FRAC_BITS;
// fdiv = div & ((1 << A2W_PLL_FRAC_BITS) - 1);
    }
    static long bcm2835_pll_rate_from_divisors(unsigned long parent_rate,
    u32 ndiv, u32 fdiv, u32 pdiv)
    {
    u64 rate;
    if (pdiv == 0)
    return 0;
    rate = (u64)parent_rate * ((ndiv << A2W_PLL_FRAC_BITS) + fdiv);
    do_div(rate, pdiv);
    return rate >> A2W_PLL_FRAC_BITS;
    }
    static int bcm2835_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct bcm2835_pll *pll = container_of(hw, struct bcm2835_pll, hw);
    const struct bcm2835_pll_data *data = pll.data;
    u32 ndiv, fdiv;
    req.rate = clamp(req.rate, data.min_rate, data.max_rate);
    bcm2835_pll_choose_ndiv_and_fdiv(req.rate, req.best_parent_rate,
    &ndiv, &fdiv);
    req.rate = bcm2835_pll_rate_from_divisors(req.best_parent_rate,
    ndiv, fdiv,
    1);
    return 0;
    }
    static unsigned long bcm2835_pll_get_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct bcm2835_pll *pll = container_of(hw, struct bcm2835_pll, hw);
    struct bcm2835_cprman *cprman = pll.cprman;
    const struct bcm2835_pll_data *data = pll.data;
    let mut a2wctrl: u32 = cprman_read(cprman, data.a2w_ctrl_reg);
    u32 ndiv, pdiv, fdiv;
    bool using_prediv;
    if (parent_rate == 0)
    return 0;
    fdiv = cprman_read(cprman, data.frac_reg) & A2W_PLL_FRAC_MASK;
    ndiv = (a2wctrl & A2W_PLL_CTRL_NDIV_MASK) >> A2W_PLL_CTRL_NDIV_SHIFT;
    pdiv = (a2wctrl & A2W_PLL_CTRL_PDIV_MASK) >> A2W_PLL_CTRL_PDIV_SHIFT;
    using_prediv = cprman_read(cprman, data.ana_reg_base + 4) &
    bcm2835_pll_get_prediv_mask(cprman, data);
    if (using_prediv) {
    ndiv *= 2;
    fdiv *= 2;
    }
    return bcm2835_pll_rate_from_divisors(parent_rate, ndiv, fdiv, pdiv);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_pll_off(hw: *mut clk_hw) {
    static void bcm2835_pll_off(struct clk_hw *hw)
    {
    struct bcm2835_pll *pll = container_of(hw, struct bcm2835_pll, hw);
    struct bcm2835_cprman *cprman = pll.cprman;
    const struct bcm2835_pll_data *data = pll.data;
    spin_lock(&cprman.regs_lock);
    cprman_write(cprman, data.cm_ctrl_reg, CM_PLL_ANARST);
    cprman_write(cprman, data.a2w_ctrl_reg,
    cprman_read(cprman, data.a2w_ctrl_reg) |
    A2W_PLL_CTRL_PWRDN);
    spin_unlock(&cprman.regs_lock);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_pll_on(hw: *mut clk_hw) -> c_int {
    static int bcm2835_pll_on(struct clk_hw *hw)
    {
    struct bcm2835_pll *pll = container_of(hw, struct bcm2835_pll, hw);
    struct bcm2835_cprman *cprman = pll.cprman;
    const struct bcm2835_pll_data *data = pll.data;
    ktime_t timeout;
    cprman_write(cprman, data.a2w_ctrl_reg,
    cprman_read(cprman, data.a2w_ctrl_reg) &
    ~A2W_PLL_CTRL_PWRDN);
// Take the PLL out of reset.
    spin_lock(&cprman.regs_lock);
    cprman_write(cprman, data.cm_ctrl_reg,
    cprman_read(cprman, data.cm_ctrl_reg) & ~CM_PLL_ANARST);
    spin_unlock(&cprman.regs_lock);
// Wait for the PLL to lock.
    timeout = ktime_add_ns(ktime_get(), LOCK_TIMEOUT_NS);
    while (!(cprman_read(cprman, CM_LOCK) & data.lock_mask)) {
    if (ktime_after(ktime_get(), timeout)) {
    dev_err(cprman.dev, "%s: couldn't lock PLL\n",
    clk_hw_get_name(hw));
    return -ETIMEDOUT;
    }
    cpu_relax();
    }
    cprman_write(cprman, data.a2w_ctrl_reg,
    cprman_read(cprman, data.a2w_ctrl_reg) |
    A2W_PLL_CTRL_PRST_DISABLE);
    return 0;
    }
    static void
    bcm2835_pll_write_ana(struct bcm2835_cprman *cprman, u32 ana_reg_base, u32 *ana)
    {
    int i;
//
// ANA register setup is done as a series of writes to
// ANA3-ANA0, in that order.  This lets us write all 4
// registers as a single cycle of the serdes interface (taking
// 100 xosc clocks), whereas if we were to update ana0, 1, and
// 3 individually through their partial-write registers, each
// would be their own serdes cycle.
//
    for (i = 3; i >= 0; i--)
    cprman_write(cprman, ana_reg_base + i * 4, ana[i]);
    }
    static int bcm2835_pll_set_rate(struct clk_hw *hw,
    unsigned long rate, unsigned long parent_rate)
    {
    struct bcm2835_pll *pll = container_of(hw, struct bcm2835_pll, hw);
    struct bcm2835_cprman *cprman = pll.cprman;
    const struct bcm2835_pll_data *data = pll.data;
    let mut prediv_mask: u32 = bcm2835_pll_get_prediv_mask(cprman, data);
    bool was_using_prediv, use_fb_prediv, do_ana_setup_first;
    u32 ndiv, fdiv, a2w_ctl;
    u32 ana[4];
    int i;
    if (rate > data.max_fb_rate) {
    use_fb_prediv = true;
    rate /= 2;
    } else {
    use_fb_prediv = false;
    }
    bcm2835_pll_choose_ndiv_and_fdiv(rate, parent_rate, &ndiv, &fdiv);
    for (i = 3; i >= 0; i--)
    ana[i] = cprman_read(cprman, data.ana_reg_base + i * 4);
    was_using_prediv = ana[1] & prediv_mask;
    ana[0] &= ~data.ana.mask0;
    ana[0] |= data.ana.set0;
    ana[1] &= ~data.ana.mask1;
    ana[1] |= data.ana.set1;
    ana[3] &= ~data.ana.mask3;
    ana[3] |= data.ana.set3;
    if (was_using_prediv && !use_fb_prediv) {
    ana[1] &= ~prediv_mask;
    do_ana_setup_first = true;
    } else if (!was_using_prediv && use_fb_prediv) {
    ana[1] |= prediv_mask;
    do_ana_setup_first = false;
    } else {
    do_ana_setup_first = true;
    }
// Unmask the reference clock from the oscillator.
    spin_lock(&cprman.regs_lock);
    cprman_write(cprman, A2W_XOSC_CTRL,
    cprman_read(cprman, A2W_XOSC_CTRL) |
    data.reference_enable_mask);
    spin_unlock(&cprman.regs_lock);
    if (do_ana_setup_first)
    bcm2835_pll_write_ana(cprman, data.ana_reg_base, ana);
// Set the PLL multiplier from the oscillator.
    cprman_write(cprman, data.frac_reg, fdiv);
    a2w_ctl = cprman_read(cprman, data.a2w_ctrl_reg);
    a2w_ctl &= ~A2W_PLL_CTRL_NDIV_MASK;
    a2w_ctl |= ndiv << A2W_PLL_CTRL_NDIV_SHIFT;
    a2w_ctl &= ~A2W_PLL_CTRL_PDIV_MASK;
    a2w_ctl |= 1 << A2W_PLL_CTRL_PDIV_SHIFT;
    cprman_write(cprman, data.a2w_ctrl_reg, a2w_ctl);
    if (!do_ana_setup_first)
    bcm2835_pll_write_ana(cprman, data.ana_reg_base, ana);
    return 0;
    }
    static void bcm2835_pll_debug_init(struct clk_hw *hw,
    struct dentry *dentry)
    {
    struct bcm2835_pll *pll = container_of(hw, struct bcm2835_pll, hw);
    struct bcm2835_cprman *cprman = pll.cprman;
    const struct bcm2835_pll_data *data = pll.data;
    struct debugfs_reg32 *regs;
    regs = devm_kcalloc(cprman.dev, 7, sizeof(*regs), GFP_KERNEL);
    if (!regs)
    return;
    regs[0].name = "cm_ctrl";
    regs[0].offset = data.cm_ctrl_reg;
    regs[1].name = "a2w_ctrl";
    regs[1].offset = data.a2w_ctrl_reg;
    regs[2].name = "frac";
    regs[2].offset = data.frac_reg;
    regs[3].name = "ana0";
    regs[3].offset = data.ana_reg_base + 0 * 4;
    regs[4].name = "ana1";
    regs[4].offset = data.ana_reg_base + 1 * 4;
    regs[5].name = "ana2";
    regs[5].offset = data.ana_reg_base + 2 * 4;
    regs[6].name = "ana3";
    regs[6].offset = data.ana_reg_base + 3 * 4;
    bcm2835_debugfs_regset(cprman, 0, regs, 7, dentry);
    }
    static const struct clk_ops bcm2835_pll_clk_ops = {
    .is_prepared = bcm2835_pll_is_on,
    .prepare = bcm2835_pll_on,
    .unprepare = bcm2835_pll_off,
    .recalc_rate = bcm2835_pll_get_rate,
    .set_rate = bcm2835_pll_set_rate,
    .determine_rate = bcm2835_pll_determine_rate,
    .debug_init = bcm2835_pll_debug_init,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_pll_divider {
    pub div: clk_divider,
    pub cprman: *mut bcm2835_cprman,
    pub data: *const bcm2835_pll_divider_data,
}

    static struct bcm2835_pll_divider *
    bcm2835_pll_divider_from_hw(struct clk_hw *hw)
    {
    return container_of(hw, struct bcm2835_pll_divider, div.hw);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_pll_divider_is_on(hw: *mut clk_hw) -> c_int {
    static int bcm2835_pll_divider_is_on(struct clk_hw *hw)
    {
    struct bcm2835_pll_divider *divider = bcm2835_pll_divider_from_hw(hw);
    struct bcm2835_cprman *cprman = divider.cprman;
    const struct bcm2835_pll_divider_data *data = divider.data;
    return !(cprman_read(cprman, data.a2w_reg) & A2W_PLL_CHANNEL_DISABLE);
    }
    static int bcm2835_pll_divider_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    return clk_divider_ops.determine_rate(hw, req);
    }
    static unsigned long bcm2835_pll_divider_get_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    return clk_divider_ops.recalc_rate(hw, parent_rate);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_pll_divider_off(hw: *mut clk_hw) {
    static void bcm2835_pll_divider_off(struct clk_hw *hw)
    {
    struct bcm2835_pll_divider *divider = bcm2835_pll_divider_from_hw(hw);
    struct bcm2835_cprman *cprman = divider.cprman;
    const struct bcm2835_pll_divider_data *data = divider.data;
    spin_lock(&cprman.regs_lock);
    cprman_write(cprman, data.cm_reg,
    (cprman_read(cprman, data.cm_reg) &
    ~data.load_mask) | data.hold_mask);
    cprman_write(cprman, data.a2w_reg,
    cprman_read(cprman, data.a2w_reg) |
    A2W_PLL_CHANNEL_DISABLE);
    spin_unlock(&cprman.regs_lock);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_pll_divider_on(hw: *mut clk_hw) -> c_int {
    static int bcm2835_pll_divider_on(struct clk_hw *hw)
    {
    struct bcm2835_pll_divider *divider = bcm2835_pll_divider_from_hw(hw);
    struct bcm2835_cprman *cprman = divider.cprman;
    const struct bcm2835_pll_divider_data *data = divider.data;
    spin_lock(&cprman.regs_lock);
    cprman_write(cprman, data.a2w_reg,
    cprman_read(cprman, data.a2w_reg) &
    ~A2W_PLL_CHANNEL_DISABLE);
    cprman_write(cprman, data.cm_reg,
    cprman_read(cprman, data.cm_reg) & ~data.hold_mask);
    spin_unlock(&cprman.regs_lock);
    return 0;
    }
    static int bcm2835_pll_divider_set_rate(struct clk_hw *hw,
    unsigned long rate,
    unsigned long parent_rate)
    {
    struct bcm2835_pll_divider *divider = bcm2835_pll_divider_from_hw(hw);
    struct bcm2835_cprman *cprman = divider.cprman;
    const struct bcm2835_pll_divider_data *data = divider.data;
    u32 cm, div, max_div = 1 << A2W_PLL_DIV_BITS;
    div = DIV_ROUND_UP_ULL(parent_rate, rate);
    div = min(div, max_div);
    if (div == max_div)
    div = 0;
    cprman_write(cprman, data.a2w_reg, div);
    cm = cprman_read(cprman, data.cm_reg);
    cprman_write(cprman, data.cm_reg, cm | data.load_mask);
    cprman_write(cprman, data.cm_reg, cm & ~data.load_mask);
    return 0;
    }
    static void bcm2835_pll_divider_debug_init(struct clk_hw *hw,
    struct dentry *dentry)
    {
    struct bcm2835_pll_divider *divider = bcm2835_pll_divider_from_hw(hw);
    struct bcm2835_cprman *cprman = divider.cprman;
    const struct bcm2835_pll_divider_data *data = divider.data;
    struct debugfs_reg32 *regs;
    regs = devm_kcalloc(cprman.dev, 7, sizeof(*regs), GFP_KERNEL);
    if (!regs)
    return;
    regs[0].name = "cm";
    regs[0].offset = data.cm_reg;
    regs[1].name = "a2w";
    regs[1].offset = data.a2w_reg;
    bcm2835_debugfs_regset(cprman, 0, regs, 2, dentry);
    }
    static const struct clk_ops bcm2835_pll_divider_clk_ops = {
    .is_prepared = bcm2835_pll_divider_is_on,
    .prepare = bcm2835_pll_divider_on,
    .unprepare = bcm2835_pll_divider_off,
    .recalc_rate = bcm2835_pll_divider_get_rate,
    .set_rate = bcm2835_pll_divider_set_rate,
    .determine_rate = bcm2835_pll_divider_determine_rate,
    .debug_init = bcm2835_pll_divider_debug_init,
    };
//
// The CM dividers do fixed-point division, so we can't use the
// generic integer divider code like the PLL dividers do (and we can't
// fake it by having some fixed shifts preceding it in the clock tree,
// because we'd run out of bits in a 32-bit unsigned long).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_clock {
    pub hw: clk_hw,
    pub cprman: *mut bcm2835_cprman,
    pub data: *const bcm2835_clock_data,
}

    static struct bcm2835_clock *bcm2835_clock_from_hw(struct clk_hw *hw)
    {
    return container_of(hw, struct bcm2835_clock, hw);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_clock_is_on(hw: *mut clk_hw) -> c_int {
    static int bcm2835_clock_is_on(struct clk_hw *hw)
    {
    struct bcm2835_clock *clock = bcm2835_clock_from_hw(hw);
    struct bcm2835_cprman *cprman = clock.cprman;
    const struct bcm2835_clock_data *data = clock.data;
    return (cprman_read(cprman, data.ctl_reg) & CM_ENABLE) != 0;
    }
    static u32 bcm2835_clock_choose_div(struct clk_hw *hw,
    unsigned long rate,
    unsigned long parent_rate)
    {
    struct bcm2835_clock *clock = bcm2835_clock_from_hw(hw);
    const struct bcm2835_clock_data *data = clock.data;
    u32 unused_frac_mask =
    GENMASK(CM_DIV_FRAC_BITS - data.frac_bits, 0) >> 1;
    let mut temp: u64 = (u64)parent_rate << CM_DIV_FRAC_BITS;
    u32 div, mindiv, maxdiv;
    do_div(temp, rate);
    div = temp;
    div &= ~unused_frac_mask;
// different clamping limits apply for a mash clock
    if (data.is_mash_clock) {
// clamp to min divider of 2
    mindiv = 2 << CM_DIV_FRAC_BITS;
// clamp to the highest possible integer divider
    maxdiv = (BIT(data.int_bits) - 1) << CM_DIV_FRAC_BITS;
    } else {
// clamp to min divider of 1
    mindiv = 1 << CM_DIV_FRAC_BITS;
// clamp to the highest possible fractional divider
    maxdiv = GENMASK(data.int_bits + CM_DIV_FRAC_BITS - 1,
    CM_DIV_FRAC_BITS - data.frac_bits);
    }
// apply the clamping  limits
    div = max_t(u32, div, mindiv);
    div = min_t(u32, div, maxdiv);
    return div;
    }
    static unsigned long bcm2835_clock_rate_from_divisor(struct bcm2835_clock *clock,
    unsigned long parent_rate,
    u32 div)
    {
    const struct bcm2835_clock_data *data = clock.data;
    u64 temp;
    if (data.int_bits == 0 && data.frac_bits == 0)
    return parent_rate;
//
// The divisor is a 12.12 fixed point field, but only some of
// the bits are populated in any given clock.
//
    div >>= CM_DIV_FRAC_BITS - data.frac_bits;
    div &= (1 << (data.int_bits + data.frac_bits)) - 1;
    if (div == 0)
    return 0;
    temp = (u64)parent_rate << data.frac_bits;
    do_div(temp, div);
    return temp;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_round_rate(rate: c_ulong) -> c_ulong {
    static unsigned long bcm2835_round_rate(unsigned long rate)
    {
    unsigned long scaler;
    unsigned long limit;
    limit = rate / 100000;
    scaler = 1;
    while (scaler < limit)
    scaler *= 10;
//
// If increasing a clock by less than 0.1% changes it
// from ..999.. to ..000.., round up.
//
    if ((rate + scaler - 1) / scaler % 1000 == 0)
    rate = roundup(rate, scaler);
    return rate;
    }
    static unsigned long bcm2835_clock_get_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct bcm2835_clock *clock = bcm2835_clock_from_hw(hw);
    struct bcm2835_cprman *cprman = clock.cprman;
    const struct bcm2835_clock_data *data = clock.data;
    unsigned long rate;
    u32 div;
    if (data.int_bits == 0 && data.frac_bits == 0)
    return parent_rate;
    div = cprman_read(cprman, data.div_reg);
    rate = bcm2835_clock_rate_from_divisor(clock, parent_rate, div);
    if (data.round_up)
    rate = bcm2835_round_rate(rate);
    return rate;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_clock_wait_busy(clock: *mut bcm2835_clock) {
    static void bcm2835_clock_wait_busy(struct bcm2835_clock *clock)
    {
    struct bcm2835_cprman *cprman = clock.cprman;
    const struct bcm2835_clock_data *data = clock.data;
    let mut timeout: ktime_t = ktime_add_ns(ktime_get(), LOCK_TIMEOUT_NS);
    while (cprman_read(cprman, data.ctl_reg) & CM_BUSY) {
    if (ktime_after(ktime_get(), timeout)) {
    dev_err(cprman.dev, "%s: couldn't lock PLL\n",
    clk_hw_get_name(&clock.hw));
    return;
    }
    cpu_relax();
    }
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_clock_off(hw: *mut clk_hw) {
    static void bcm2835_clock_off(struct clk_hw *hw)
    {
    struct bcm2835_clock *clock = bcm2835_clock_from_hw(hw);
    struct bcm2835_cprman *cprman = clock.cprman;
    const struct bcm2835_clock_data *data = clock.data;
    spin_lock(&cprman.regs_lock);
    cprman_write(cprman, data.ctl_reg,
    cprman_read(cprman, data.ctl_reg) & ~CM_ENABLE);
    spin_unlock(&cprman.regs_lock);
// BUSY will remain high until the divider completes its cycle.
    bcm2835_clock_wait_busy(clock);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_clock_on(hw: *mut clk_hw) -> c_int {
    static int bcm2835_clock_on(struct clk_hw *hw)
    {
    struct bcm2835_clock *clock = bcm2835_clock_from_hw(hw);
    struct bcm2835_cprman *cprman = clock.cprman;
    const struct bcm2835_clock_data *data = clock.data;
    spin_lock(&cprman.regs_lock);
    cprman_write(cprman, data.ctl_reg,
    cprman_read(cprman, data.ctl_reg) |
    CM_ENABLE |
    CM_GATE);
    spin_unlock(&cprman.regs_lock);
// Debug code to measure the clock once it's turned on to see
// if it's ticking at the rate we expect.
//
    if (data.tcnt_mux && false) {
    dev_info(cprman.dev,
    "clk %s: rate %ld, measure %ld\n",
    data.name,
    clk_hw_get_rate(hw),
    bcm2835_measure_tcnt_mux(cprman, data.tcnt_mux));
    }
    return 0;
    }
    static int bcm2835_clock_set_rate(struct clk_hw *hw,
    unsigned long rate, unsigned long parent_rate)
    {
    struct bcm2835_clock *clock = bcm2835_clock_from_hw(hw);
    struct bcm2835_cprman *cprman = clock.cprman;
    const struct bcm2835_clock_data *data = clock.data;
    let mut div: u32 = bcm2835_clock_choose_div(hw, rate, parent_rate);
    u32 ctl;
    spin_lock(&cprman.regs_lock);
//
// Setting up frac support
//
// In principle it is recommended to stop/start the clock first,
// but as we set CLK_SET_RATE_GATE during registration of the
// clock this requirement should be take care of by the
// clk-framework.
//
    ctl = cprman_read(cprman, data.ctl_reg) & ~CM_FRAC;
    ctl |= (div & CM_DIV_FRAC_MASK) ? CM_FRAC : 0;
    cprman_write(cprman, data.ctl_reg, ctl);
    cprman_write(cprman, data.div_reg, div);
    spin_unlock(&cprman.regs_lock);
    return 0;
    }
    static bool
    bcm2835_clk_is_pllc(struct clk_hw *hw)
    {
    if (!hw)
    return false;
    return strncmp(clk_hw_get_name(hw), "pllc", 4) == 0;
    }
    static unsigned long bcm2835_clock_choose_div_and_prate(struct clk_hw *hw,
    int parent_idx,
    unsigned long rate,
    u32 *div,
    unsigned long *prate,
    unsigned long *avgrate)
    {
    struct bcm2835_clock *clock = bcm2835_clock_from_hw(hw);
    struct bcm2835_cprman *cprman = clock.cprman;
    const struct bcm2835_clock_data *data = clock.data;
    let mut best_rate: c_ulong = 0;
    u32 curdiv, mindiv, maxdiv;
    struct clk_hw *parent;
    parent = clk_hw_get_parent_by_index(hw, parent_idx);
    if (!(BIT(parent_idx) & data.set_rate_parent)) {
// prate = clk_hw_get_rate(parent);
// div = bcm2835_clock_choose_div(hw, rate, *prate);
// avgrate = bcm2835_clock_rate_from_divisor(clock, *prate, *div);
    if (data.low_jitter && (*div & CM_DIV_FRAC_MASK)) {
    unsigned long high, low;
    let mut int_div: u32 = *div & ~CM_DIV_FRAC_MASK;
    high = bcm2835_clock_rate_from_divisor(clock, *prate,
    int_div);
    int_div += CM_DIV_FRAC_MASK + 1;
    low = bcm2835_clock_rate_from_divisor(clock, *prate,
    int_div);
//
// Return a value which is the maximum deviation
// below the ideal rate, for use as a metric.
//
    return *avgrate - max(*avgrate - low, high - *avgrate);
    }
    return *avgrate;
    }
    if (data.frac_bits)
    dev_warn(cprman.dev,
    "frac bits are not used when propagating rate change");
// clamp to min divider of 2 if we're dealing with a mash clock
    mindiv = data.is_mash_clock ? 2 : 1;
    maxdiv = BIT(data.int_bits) - 1;
// TODO: Be smart, and only test a subset of the available divisors.
    for (curdiv = mindiv; curdiv <= maxdiv; curdiv++) {
    unsigned long tmp_rate;
    tmp_rate = clk_hw_round_rate(parent, rate * curdiv);
    tmp_rate /= curdiv;
    if (curdiv == mindiv ||
    (tmp_rate > best_rate && tmp_rate <= rate))
    best_rate = tmp_rate;
    if (best_rate == rate)
    break;
    }
// div = curdiv << CM_DIV_FRAC_BITS;
// prate = curdiv * best_rate;
// avgrate = best_rate;
    return best_rate;
    }
    static int bcm2835_clock_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_hw *parent, *best_parent = core::ptr::null_mut();
    bool current_parent_is_pllc;
    unsigned long rate, best_rate = 0;
    unsigned long prate, best_prate = 0;
    unsigned long avgrate, best_avgrate = 0;
    size_t i;
    u32 div;
    current_parent_is_pllc = bcm2835_clk_is_pllc(clk_hw_get_parent(hw));
//
// Select parent clock that results in the closest but lower rate
//
    for (i = 0; i < clk_hw_get_num_parents(hw); ++i) {
    parent = clk_hw_get_parent_by_index(hw, i);
    if (!parent)
    continue;
//
// Don't choose a PLLC-derived clock as our parent
// unless it had been manually set that way.  PLLC's
// frequency gets adjusted by the firmware due to
// over-temp or under-voltage conditions, without
// prior notification to our clock consumer.
//
    if (bcm2835_clk_is_pllc(parent) && !current_parent_is_pllc)
    continue;
    rate = bcm2835_clock_choose_div_and_prate(hw, i, req.rate,
    &div, &prate,
    &avgrate);
    if (abs(req.rate - rate) < abs(req.rate - best_rate)) {
    best_parent = parent;
    best_prate = prate;
    best_rate = rate;
    best_avgrate = avgrate;
    }
    }
    if (!best_parent)
    return -EINVAL;
    req.best_parent_hw = best_parent;
    req.best_parent_rate = best_prate;
    req.rate = best_avgrate;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_clock_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int bcm2835_clock_set_parent(struct clk_hw *hw, u8 index)
    {
    struct bcm2835_clock *clock = bcm2835_clock_from_hw(hw);
    struct bcm2835_cprman *cprman = clock.cprman;
    const struct bcm2835_clock_data *data = clock.data;
    let mut src: u8 = (index << CM_SRC_SHIFT) & CM_SRC_MASK;
    cprman_write(cprman, data.ctl_reg, src);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_clock_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 bcm2835_clock_get_parent(struct clk_hw *hw)
    {
    struct bcm2835_clock *clock = bcm2835_clock_from_hw(hw);
    struct bcm2835_cprman *cprman = clock.cprman;
    const struct bcm2835_clock_data *data = clock.data;
    let mut src: u32 = cprman_read(cprman, data.ctl_reg);
    return (src & CM_SRC_MASK) >> CM_SRC_SHIFT;
    }
    static const struct debugfs_reg32 bcm2835_debugfs_clock_reg32[] = {
    {
    .name = "ctl",
    .offset = 0,
    },
    {
    .name = "div",
    .offset = 4,
    },
    };
    static void bcm2835_clock_debug_init(struct clk_hw *hw,
    struct dentry *dentry)
    {
    struct bcm2835_clock *clock = bcm2835_clock_from_hw(hw);
    struct bcm2835_cprman *cprman = clock.cprman;
    const struct bcm2835_clock_data *data = clock.data;
    bcm2835_debugfs_regset(cprman, data.ctl_reg,
    bcm2835_debugfs_clock_reg32,
    ARRAY_SIZE(bcm2835_debugfs_clock_reg32),
    dentry);
    }
    static const struct clk_ops bcm2835_clock_clk_ops = {
    .is_prepared = bcm2835_clock_is_on,
    .prepare = bcm2835_clock_on,
    .unprepare = bcm2835_clock_off,
    .recalc_rate = bcm2835_clock_get_rate,
    .set_rate = bcm2835_clock_set_rate,
    .determine_rate = bcm2835_clock_determine_rate,
    .set_parent = bcm2835_clock_set_parent,
    .get_parent = bcm2835_clock_get_parent,
    .debug_init = bcm2835_clock_debug_init,
    };
#[no_mangle]
unsafe extern "C" fn bcm2835_vpu_clock_is_on(hw: *mut clk_hw) -> c_int {
    static int bcm2835_vpu_clock_is_on(struct clk_hw *hw)
    {
    return true;
    }
//
// The VPU clock can never be disabled (it doesn't have an ENABLE
// bit), so it gets its own set of clock ops.
//
    static const struct clk_ops bcm2835_vpu_clock_clk_ops = {
    .is_prepared = bcm2835_vpu_clock_is_on,
    .recalc_rate = bcm2835_clock_get_rate,
    .set_rate = bcm2835_clock_set_rate,
    .determine_rate = bcm2835_clock_determine_rate,
    .set_parent = bcm2835_clock_set_parent,
    .get_parent = bcm2835_clock_get_parent,
    .debug_init = bcm2835_clock_debug_init,
    };
    static struct clk_hw *bcm2835_register_pll(struct bcm2835_cprman *cprman,
    const void *data)
    {
    const struct bcm2835_pll_data *pll_data = data;
    struct bcm2835_pll *pll;
    struct clk_init_data init;
    int ret;
    memset(&init, 0, sizeof(init));
// All of the PLLs derive from the external oscillator.
    init.parent_names = &cprman.real_parent_names[0];
    init.num_parents = 1;
    init.name = pll_data.name;
    init.ops = &bcm2835_pll_clk_ops;
    init.flags = pll_data.flags | CLK_IGNORE_UNUSED;
    pll = kzalloc_obj(*pll);
    if (!pll)
    return core::ptr::null_mut();
    pll.cprman = cprman;
    pll.data = pll_data;
    pll.hw.init = &init;
    ret = devm_clk_hw_register(cprman.dev, &pll.hw);
    if (ret) {
    kfree(pll);
    return core::ptr::null_mut();
    }
    return &pll.hw;
    }
    static struct clk_hw *
    bcm2835_register_pll_divider(struct bcm2835_cprman *cprman,
    const void *data)
    {
    const struct bcm2835_pll_divider_data *divider_data = data;
    struct bcm2835_pll_divider *divider;
    struct clk_init_data init;
    const char *divider_name;
    int ret;
    if (divider_data.fixed_divider != 1) {
    divider_name = devm_kasprintf(cprman.dev, GFP_KERNEL,
    "%s_prediv", divider_data.name);
    if (!divider_name)
    return core::ptr::null_mut();
    } else {
    divider_name = divider_data.name;
    }
    memset(&init, 0, sizeof(init));
    init.parent_names = &divider_data.source_pll;
    init.num_parents = 1;
    init.name = divider_name;
    init.ops = &bcm2835_pll_divider_clk_ops;
    init.flags = divider_data.flags | CLK_IGNORE_UNUSED;
    divider = devm_kzalloc(cprman.dev, sizeof(*divider), GFP_KERNEL);
    if (!divider)
    return core::ptr::null_mut();
    divider.div.reg = cprman.regs + divider_data.a2w_reg;
    divider.div.shift = A2W_PLL_DIV_SHIFT;
    divider.div.width = A2W_PLL_DIV_BITS;
    divider.div.flags = CLK_DIVIDER_MAX_AT_ZERO;
    divider.div.lock = &cprman.regs_lock;
    divider.div.hw.init = &init;
    divider.div.table = core::ptr::null_mut();
    divider.cprman = cprman;
    divider.data = divider_data;
    ret = devm_clk_hw_register(cprman.dev, &divider.div.hw);
    if (ret)
    return ERR_PTR(ret);
//
// PLLH's channels have a fixed divide by 10 afterwards, which
// is what our consumers are actually using.
//
    if (divider_data.fixed_divider != 1) {
    return clk_hw_register_fixed_factor(cprman.dev,
    divider_data.name,
    divider_name,
    CLK_SET_RATE_PARENT,
    1,
    divider_data.fixed_divider);
    }
    return &divider.div.hw;
    }
    static struct clk_hw *bcm2835_register_clock(struct bcm2835_cprman *cprman,
    const void *data)
    {
    const struct bcm2835_clock_data *clock_data = data;
    struct bcm2835_clock *clock;
    struct clk_init_data init;
    const char *parents[1 << CM_SRC_BITS];
    size_t i;
    int ret;
//
// Replace our strings referencing parent clocks with the
// actual clock-output-name of the parent.
//
    for (i = 0; i < clock_data.num_mux_parents; i++) {
    parents[i] = clock_data.parents[i];
    ret = match_string(cprman_parent_names,
    ARRAY_SIZE(cprman_parent_names),
    parents[i]);
    if (ret >= 0)
    parents[i] = cprman.real_parent_names[ret];
    }
    memset(&init, 0, sizeof(init));
    init.parent_names = parents;
    init.num_parents = clock_data.num_mux_parents;
    init.name = clock_data.name;
    init.flags = clock_data.flags | CLK_IGNORE_UNUSED;
//
// Pass the CLK_SET_RATE_PARENT flag if we are allowed to propagate
// rate changes on at least of the parents.
//
    if (clock_data.set_rate_parent)
    init.flags |= CLK_SET_RATE_PARENT;
    if (clock_data.is_vpu_clock) {
    init.ops = &bcm2835_vpu_clock_clk_ops;
    } else {
    init.ops = &bcm2835_clock_clk_ops;
    init.flags |= CLK_SET_RATE_GATE | CLK_SET_PARENT_GATE;
// If the clock wasn't actually enabled at boot, it's not
// critical.
//
    if (!(cprman_read(cprman, clock_data.ctl_reg) & CM_ENABLE))
    init.flags &= ~CLK_IS_CRITICAL;
    }
    clock = devm_kzalloc(cprman.dev, sizeof(*clock), GFP_KERNEL);
    if (!clock)
    return core::ptr::null_mut();
    clock.cprman = cprman;
    clock.data = clock_data;
    clock.hw.init = &init;
    ret = devm_clk_hw_register(cprman.dev, &clock.hw);
    if (ret)
    return ERR_PTR(ret);
    return &clock.hw;
    }
    static struct clk_hw *bcm2835_register_gate(struct bcm2835_cprman *cprman,
    const void *data)
    {
    const struct bcm2835_gate_data *gate_data = data;
    return clk_hw_register_gate(cprman.dev, gate_data.name,
    gate_data.parent,
    CLK_IGNORE_UNUSED | CLK_SET_RATE_GATE,
    cprman.regs + gate_data.ctl_reg,
    CM_GATE_BIT, 0, &cprman.regs_lock);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_clk_desc {
    struct clk_hw *(*clk_register)(struct bcm2835_cprman *cprman,
    pub data): *const c_void,
    pub supported: c_uint,
    pub data: *const c_void,
}

// assignment helper macros for different clock types

    .supported = s,				\
    .data = __VA_ARGS__ }

    s,				\
    &(struct bcm2835_pll_data)	\
    {__VA_ARGS__})

    s,				  \
    &(struct bcm2835_pll_divider_data) \
    {__VA_ARGS__})

    s,				\
    &(struct bcm2835_clock_data)	\
    {__VA_ARGS__})

    s,				\
    &(struct bcm2835_gate_data)	\
    {__VA_ARGS__})
// parent mux arrays plus helper macros
// main oscillator parent mux
    static const char *const bcm2835_clock_osc_parents[] = {
    "gnd",
    "xosc",
    "testdebug0",
    "testdebug1"
    };

    s,								\
    .num_mux_parents = ARRAY_SIZE(bcm2835_clock_osc_parents),	\
    .parents = bcm2835_clock_osc_parents,				\
    __VA_ARGS__)
// main peripheral parent mux
    static const char *const bcm2835_clock_per_parents[] = {
    "gnd",
    "xosc",
    "testdebug0",
    "testdebug1",
    "plla_per",
    "pllc_per",
    "plld_per",
    "pllh_aux",
    };

    s,								\
    .num_mux_parents = ARRAY_SIZE(bcm2835_clock_per_parents),	\
    .parents = bcm2835_clock_per_parents,				\
    __VA_ARGS__)
//
// Restrict clock sources for the PCM peripheral to the oscillator and
// PLLD_PER because other source may have varying rates or be switched
// off.
//
// Prevent other sources from being selected by replacing their names in
// the list of potential parents with dummy entries (entry index is
// significant).
//
    static const char *const bcm2835_pcm_per_parents[] = {
    "-",
    "xosc",
    "-",
    "-",
    "-",
    "-",
    "plld_per",
    "-",
    };

    s,								\
    .num_mux_parents = ARRAY_SIZE(bcm2835_pcm_per_parents),		\
    .parents = bcm2835_pcm_per_parents,				\
    __VA_ARGS__)
// main vpu parent mux
    static const char *const bcm2835_clock_vpu_parents[] = {
    "gnd",
    "xosc",
    "testdebug0",
    "testdebug1",
    "plla_core",
    "pllc_core0",
    "plld_core",
    "pllh_aux",
    "pllc_core1",
    "pllc_core2",
    };

    s,								\
    .num_mux_parents = ARRAY_SIZE(bcm2835_clock_vpu_parents),	\
    .parents = bcm2835_clock_vpu_parents,				\
    __VA_ARGS__)
//
// DSI parent clocks.  The DSI byte/DDR/DDR2 clocks come from the DSI
// analog PHY.  The _inv variants are generated internally to cprman,
// but we don't use them so they aren't hooked up.
//
    static const char *const bcm2835_clock_dsi0_parents[] = {
    "gnd",
    "xosc",
    "testdebug0",
    "testdebug1",
    "dsi0_ddr",
    "dsi0_ddr_inv",
    "dsi0_ddr2",
    "dsi0_ddr2_inv",
    "dsi0_byte",
    "dsi0_byte_inv",
    };
    static const char *const bcm2835_clock_dsi1_parents[] = {
    "gnd",
    "xosc",
    "testdebug0",
    "testdebug1",
    "dsi1_ddr",
    "dsi1_ddr_inv",
    "dsi1_ddr2",
    "dsi1_ddr2_inv",
    "dsi1_byte",
    "dsi1_byte_inv",
    };

    s,								\
    .num_mux_parents = ARRAY_SIZE(bcm2835_clock_dsi0_parents),	\
    .parents = bcm2835_clock_dsi0_parents,				\
    __VA_ARGS__)

    s,								\
    .num_mux_parents = ARRAY_SIZE(bcm2835_clock_dsi1_parents),	\
    .parents = bcm2835_clock_dsi1_parents,				\
    __VA_ARGS__)
//
// the real definition of all the pll, pll_dividers and clocks
// these make use of the above REGISTER_* macros
//
    static const struct bcm2835_clk_desc clk_desc_array[] = {
// the PLL + PLL dividers
//
// PLLA is the auxiliary PLL, used to drive the CCP2
// (Compact Camera Port 2) transmitter clock.
//
// It is in the PX LDO power domain, which is on when the
// AUDIO domain is on.
//
    [BCM2835_PLLA]		= REGISTER_PLL(
    SOC_ALL,
    .name = "plla",
    .cm_ctrl_reg = CM_PLLA,
    .a2w_ctrl_reg = A2W_PLLA_CTRL,
    .frac_reg = A2W_PLLA_FRAC,
    .ana_reg_base = A2W_PLLA_ANA0,
    .reference_enable_mask = A2W_XOSC_CTRL_PLLA_ENABLE,
    .lock_mask = CM_LOCK_FLOCKA,
    .ana = &bcm2835_ana_default,
    .min_rate = 600000000u,
    .max_rate = 2400000000u,
    .max_fb_rate = BCM2835_MAX_FB_RATE),
    [BCM2835_PLLA_CORE]	= REGISTER_PLL_DIV(
    SOC_ALL,
    .name = "plla_core",
    .source_pll = "plla",
    .cm_reg = CM_PLLA,
    .a2w_reg = A2W_PLLA_CORE,
    .load_mask = CM_PLLA_LOADCORE,
    .hold_mask = CM_PLLA_HOLDCORE,
    .fixed_divider = 1,
    .flags = CLK_SET_RATE_PARENT),
    [BCM2835_PLLA_PER]	= REGISTER_PLL_DIV(
    SOC_ALL,
    .name = "plla_per",
    .source_pll = "plla",
    .cm_reg = CM_PLLA,
    .a2w_reg = A2W_PLLA_PER,
    .load_mask = CM_PLLA_LOADPER,
    .hold_mask = CM_PLLA_HOLDPER,
    .fixed_divider = 1,
    .flags = CLK_SET_RATE_PARENT),
    [BCM2835_PLLA_DSI0]	= REGISTER_PLL_DIV(
    SOC_ALL,
    .name = "plla_dsi0",
    .source_pll = "plla",
    .cm_reg = CM_PLLA,
    .a2w_reg = A2W_PLLA_DSI0,
    .load_mask = CM_PLLA_LOADDSI0,
    .hold_mask = CM_PLLA_HOLDDSI0,
    .fixed_divider = 1),
    [BCM2835_PLLA_CCP2]	= REGISTER_PLL_DIV(
    SOC_ALL,
    .name = "plla_ccp2",
    .source_pll = "plla",
    .cm_reg = CM_PLLA,
    .a2w_reg = A2W_PLLA_CCP2,
    .load_mask = CM_PLLA_LOADCCP2,
    .hold_mask = CM_PLLA_HOLDCCP2,
    .fixed_divider = 1,
    .flags = CLK_SET_RATE_PARENT),
// PLLB is used for the ARM's clock.
    [BCM2835_PLLB]		= REGISTER_PLL(
    SOC_ALL,
    .name = "pllb",
    .cm_ctrl_reg = CM_PLLB,
    .a2w_ctrl_reg = A2W_PLLB_CTRL,
    .frac_reg = A2W_PLLB_FRAC,
    .ana_reg_base = A2W_PLLB_ANA0,
    .reference_enable_mask = A2W_XOSC_CTRL_PLLB_ENABLE,
    .lock_mask = CM_LOCK_FLOCKB,
    .ana = &bcm2835_ana_default,
    .min_rate = 600000000u,
    .max_rate = 3000000000u,
    .max_fb_rate = BCM2835_MAX_FB_RATE,
    .flags = CLK_GET_RATE_NOCACHE),
    [BCM2835_PLLB_ARM]	= REGISTER_PLL_DIV(
    SOC_ALL,
    .name = "pllb_arm",
    .source_pll = "pllb",
    .cm_reg = CM_PLLB,
    .a2w_reg = A2W_PLLB_ARM,
    .load_mask = CM_PLLB_LOADARM,
    .hold_mask = CM_PLLB_HOLDARM,
    .fixed_divider = 1,
    .flags = CLK_SET_RATE_PARENT | CLK_GET_RATE_NOCACHE),
//
// PLLC is the core PLL, used to drive the core VPU clock.
//
// It is in the PX LDO power domain, which is on when the
// AUDIO domain is on.
//
    [BCM2835_PLLC]		= REGISTER_PLL(
    SOC_ALL,
    .name = "pllc",
    .cm_ctrl_reg = CM_PLLC,
    .a2w_ctrl_reg = A2W_PLLC_CTRL,
    .frac_reg = A2W_PLLC_FRAC,
    .ana_reg_base = A2W_PLLC_ANA0,
    .reference_enable_mask = A2W_XOSC_CTRL_PLLC_ENABLE,
    .lock_mask = CM_LOCK_FLOCKC,
    .ana = &bcm2835_ana_default,
    .min_rate = 600000000u,
    .max_rate = 3000000000u,
    .max_fb_rate = BCM2835_MAX_FB_RATE),
    [BCM2835_PLLC_CORE0]	= REGISTER_PLL_DIV(
    SOC_ALL,
    .name = "pllc_core0",
    .source_pll = "pllc",
    .cm_reg = CM_PLLC,
    .a2w_reg = A2W_PLLC_CORE0,
    .load_mask = CM_PLLC_LOADCORE0,
    .hold_mask = CM_PLLC_HOLDCORE0,
    .fixed_divider = 1,
    .flags = CLK_SET_RATE_PARENT),
    [BCM2835_PLLC_CORE1]	= REGISTER_PLL_DIV(
    SOC_ALL,
    .name = "pllc_core1",
    .source_pll = "pllc",
    .cm_reg = CM_PLLC,
    .a2w_reg = A2W_PLLC_CORE1,
    .load_mask = CM_PLLC_LOADCORE1,
    .hold_mask = CM_PLLC_HOLDCORE1,
    .fixed_divider = 1,
    .flags = CLK_SET_RATE_PARENT),
    [BCM2835_PLLC_CORE2]	= REGISTER_PLL_DIV(
    SOC_ALL,
    .name = "pllc_core2",
    .source_pll = "pllc",
    .cm_reg = CM_PLLC,
    .a2w_reg = A2W_PLLC_CORE2,
    .load_mask = CM_PLLC_LOADCORE2,
    .hold_mask = CM_PLLC_HOLDCORE2,
    .fixed_divider = 1,
    .flags = CLK_SET_RATE_PARENT),
    [BCM2835_PLLC_PER]	= REGISTER_PLL_DIV(
    SOC_ALL,
    .name = "pllc_per",
    .source_pll = "pllc",
    .cm_reg = CM_PLLC,
    .a2w_reg = A2W_PLLC_PER,
    .load_mask = CM_PLLC_LOADPER,
    .hold_mask = CM_PLLC_HOLDPER,
    .fixed_divider = 1,
    .flags = CLK_IS_CRITICAL | CLK_SET_RATE_PARENT),
//
// PLLD is the display PLL, used to drive DSI display panels.
//
// It is in the PX LDO power domain, which is on when the
// AUDIO domain is on.
//
    [BCM2835_PLLD]		= REGISTER_PLL(
    SOC_ALL,
    .name = "plld",
    .cm_ctrl_reg = CM_PLLD,
    .a2w_ctrl_reg = A2W_PLLD_CTRL,
    .frac_reg = A2W_PLLD_FRAC,
    .ana_reg_base = A2W_PLLD_ANA0,
    .reference_enable_mask = A2W_XOSC_CTRL_DDR_ENABLE,
    .lock_mask = CM_LOCK_FLOCKD,
    .ana = &bcm2835_ana_default,
    .min_rate = 600000000u,
    .max_rate = 2400000000u,
    .max_fb_rate = BCM2835_MAX_FB_RATE),
    [BCM2835_PLLD_CORE]	= REGISTER_PLL_DIV(
    SOC_ALL,
    .name = "plld_core",
    .source_pll = "plld",
    .cm_reg = CM_PLLD,
    .a2w_reg = A2W_PLLD_CORE,
    .load_mask = CM_PLLD_LOADCORE,
    .hold_mask = CM_PLLD_HOLDCORE,
    .fixed_divider = 1,
    .flags = CLK_SET_RATE_PARENT),
//
// VPU firmware assumes that PLLD_PER isn't disabled by the ARM core.
// Otherwise this could cause firmware lookups. That's why we mark
// it as critical.
//
    [BCM2835_PLLD_PER]	= REGISTER_PLL_DIV(
    SOC_ALL,
    .name = "plld_per",
    .source_pll = "plld",
    .cm_reg = CM_PLLD,
    .a2w_reg = A2W_PLLD_PER,
    .load_mask = CM_PLLD_LOADPER,
    .hold_mask = CM_PLLD_HOLDPER,
    .fixed_divider = 1,
    .flags = CLK_IS_CRITICAL | CLK_SET_RATE_PARENT),
    [BCM2835_PLLD_DSI0]	= REGISTER_PLL_DIV(
    SOC_ALL,
    .name = "plld_dsi0",
    .source_pll = "plld",
    .cm_reg = CM_PLLD,
    .a2w_reg = A2W_PLLD_DSI0,
    .load_mask = CM_PLLD_LOADDSI0,
    .hold_mask = CM_PLLD_HOLDDSI0,
    .fixed_divider = 1),
    [BCM2835_PLLD_DSI1]	= REGISTER_PLL_DIV(
    SOC_ALL,
    .name = "plld_dsi1",
    .source_pll = "plld",
    .cm_reg = CM_PLLD,
    .a2w_reg = A2W_PLLD_DSI1,
    .load_mask = CM_PLLD_LOADDSI1,
    .hold_mask = CM_PLLD_HOLDDSI1,
    .fixed_divider = 1),
//
// PLLH is used to supply the pixel clock or the AUX clock for the
// TV encoder.
//
// It is in the HDMI power domain.
//
    [BCM2835_PLLH]		= REGISTER_PLL(
    SOC_BCM2835,
    "pllh",
    .cm_ctrl_reg = CM_PLLH,
    .a2w_ctrl_reg = A2W_PLLH_CTRL,
    .frac_reg = A2W_PLLH_FRAC,
    .ana_reg_base = A2W_PLLH_ANA0,
    .reference_enable_mask = A2W_XOSC_CTRL_PLLC_ENABLE,
    .lock_mask = CM_LOCK_FLOCKH,
    .ana = &bcm2835_ana_pllh,
    .min_rate = 600000000u,
    .max_rate = 3000000000u,
    .max_fb_rate = BCM2835_MAX_FB_RATE),
    [BCM2835_PLLH_RCAL]	= REGISTER_PLL_DIV(
    SOC_BCM2835,
    .name = "pllh_rcal",
    .source_pll = "pllh",
    .cm_reg = CM_PLLH,
    .a2w_reg = A2W_PLLH_RCAL,
    .load_mask = CM_PLLH_LOADRCAL,
    .hold_mask = 0,
    .fixed_divider = 10,
    .flags = CLK_SET_RATE_PARENT),
    [BCM2835_PLLH_AUX]	= REGISTER_PLL_DIV(
    SOC_BCM2835,
    .name = "pllh_aux",
    .source_pll = "pllh",
    .cm_reg = CM_PLLH,
    .a2w_reg = A2W_PLLH_AUX,
    .load_mask = CM_PLLH_LOADAUX,
    .hold_mask = 0,
    .fixed_divider = 1,
    .flags = CLK_SET_RATE_PARENT),
    [BCM2835_PLLH_PIX]	= REGISTER_PLL_DIV(
    SOC_BCM2835,
    .name = "pllh_pix",
    .source_pll = "pllh",
    .cm_reg = CM_PLLH,
    .a2w_reg = A2W_PLLH_PIX,
    .load_mask = CM_PLLH_LOADPIX,
    .hold_mask = 0,
    .fixed_divider = 10,
    .flags = CLK_SET_RATE_PARENT),
// the clocks
// clocks with oscillator parent mux
// One Time Programmable Memory clock.  Maximum 10Mhz.
    [BCM2835_CLOCK_OTP]	= REGISTER_OSC_CLK(
    SOC_ALL,
    .name = "otp",
    .ctl_reg = CM_OTPCTL,
    .div_reg = CM_OTPDIV,
    .int_bits = 4,
    .frac_bits = 0,
    .tcnt_mux = 6),
//
// Used for a 1Mhz clock for the system clocksource, and also used
// bythe watchdog timer and the camera pulse generator.
//
    [BCM2835_CLOCK_TIMER]	= REGISTER_OSC_CLK(
    SOC_ALL,
    .name = "timer",
    .ctl_reg = CM_TIMERCTL,
    .div_reg = CM_TIMERDIV,
    .int_bits = 6,
    .frac_bits = 12),
//
// Clock for the temperature sensor.
// Generally run at 2Mhz, max 5Mhz.
//
    [BCM2835_CLOCK_TSENS]	= REGISTER_OSC_CLK(
    SOC_ALL,
    .name = "tsens",
    .ctl_reg = CM_TSENSCTL,
    .div_reg = CM_TSENSDIV,
    .int_bits = 5,
    .frac_bits = 0),
    [BCM2835_CLOCK_TEC]	= REGISTER_OSC_CLK(
    SOC_ALL,
    .name = "tec",
    .ctl_reg = CM_TECCTL,
    .div_reg = CM_TECDIV,
    .int_bits = 6,
    .frac_bits = 0),
// clocks with vpu parent mux
    [BCM2835_CLOCK_H264]	= REGISTER_VPU_CLK(
    SOC_ALL,
    .name = "h264",
    .ctl_reg = CM_H264CTL,
    .div_reg = CM_H264DIV,
    .int_bits = 4,
    .frac_bits = 8,
    .tcnt_mux = 1),
    [BCM2835_CLOCK_ISP]	= REGISTER_VPU_CLK(
    SOC_ALL,
    .name = "isp",
    .ctl_reg = CM_ISPCTL,
    .div_reg = CM_ISPDIV,
    .int_bits = 4,
    .frac_bits = 8,
    .tcnt_mux = 2),
//
// Secondary SDRAM clock.  Used for low-voltage modes when the PLL
// in the SDRAM controller can't be used.
//
    [BCM2835_CLOCK_SDRAM]	= REGISTER_VPU_CLK(
    SOC_ALL,
    .name = "sdram",
    .ctl_reg = CM_SDCCTL,
    .div_reg = CM_SDCDIV,
    .int_bits = 6,
    .frac_bits = 0,
    .tcnt_mux = 3),
    [BCM2835_CLOCK_V3D]	= REGISTER_VPU_CLK(
    SOC_ALL,
    .name = "v3d",
    .ctl_reg = CM_V3DCTL,
    .div_reg = CM_V3DDIV,
    .int_bits = 4,
    .frac_bits = 8,
    .tcnt_mux = 4),
//
// VPU clock.  This doesn't have an enable bit, since it drives
// the bus for everything else, and is special so it doesn't need
// to be gated for rate changes.  It is also known as "clk_audio"
// in various hardware documentation.
//
    [BCM2835_CLOCK_VPU]	= REGISTER_VPU_CLK(
    SOC_ALL,
    .name = "vpu",
    .ctl_reg = CM_VPUCTL,
    .div_reg = CM_VPUDIV,
    .int_bits = 12,
    .frac_bits = 8,
    .flags = CLK_IS_CRITICAL,
    .is_vpu_clock = true,
    .tcnt_mux = 5),
// clocks with per parent mux
    [BCM2835_CLOCK_AVEO]	= REGISTER_PER_CLK(
    SOC_ALL,
    .name = "aveo",
    .ctl_reg = CM_AVEOCTL,
    .div_reg = CM_AVEODIV,
    .int_bits = 4,
    .frac_bits = 0,
    .tcnt_mux = 38),
    [BCM2835_CLOCK_CAM0]	= REGISTER_PER_CLK(
    SOC_ALL,
    .name = "cam0",
    .ctl_reg = CM_CAM0CTL,
    .div_reg = CM_CAM0DIV,
    .int_bits = 4,
    .frac_bits = 8,
    .tcnt_mux = 14),
    [BCM2835_CLOCK_CAM1]	= REGISTER_PER_CLK(
    SOC_ALL,
    .name = "cam1",
    .ctl_reg = CM_CAM1CTL,
    .div_reg = CM_CAM1DIV,
    .int_bits = 4,
    .frac_bits = 8,
    .tcnt_mux = 15),
    [BCM2835_CLOCK_DFT]	= REGISTER_PER_CLK(
    SOC_ALL,
    .name = "dft",
    .ctl_reg = CM_DFTCTL,
    .div_reg = CM_DFTDIV,
    .int_bits = 5,
    .frac_bits = 0),
    [BCM2835_CLOCK_DPI]	= REGISTER_PER_CLK(
    SOC_ALL,
    .name = "dpi",
    .ctl_reg = CM_DPICTL,
    .div_reg = CM_DPIDIV,
    .int_bits = 4,
    .frac_bits = 8,
    .tcnt_mux = 17),
// Arasan EMMC clock
    [BCM2835_CLOCK_EMMC]	= REGISTER_PER_CLK(
    SOC_ALL,
    .name = "emmc",
    .ctl_reg = CM_EMMCCTL,
    .div_reg = CM_EMMCDIV,
    .int_bits = 4,
    .frac_bits = 8,
    .tcnt_mux = 39),
// EMMC2 clock (only available for BCM2711)
    [BCM2711_CLOCK_EMMC2]	= REGISTER_PER_CLK(
    SOC_BCM2711,
    .name = "emmc2",
    .ctl_reg = CM_EMMC2CTL,
    .div_reg = CM_EMMC2DIV,
    .int_bits = 4,
    .frac_bits = 8,
    .tcnt_mux = 42),
// General purpose (GPIO) clocks
    [BCM2835_CLOCK_GP0]	= REGISTER_PER_CLK(
    SOC_ALL,
    .name = "gp0",
    .ctl_reg = CM_GP0CTL,
    .div_reg = CM_GP0DIV,
    .int_bits = 12,
    .frac_bits = 12,
    .is_mash_clock = true,
    .tcnt_mux = 20),
    [BCM2835_CLOCK_GP1]	= REGISTER_PER_CLK(
    SOC_ALL,
    .name = "gp1",
    .ctl_reg = CM_GP1CTL,
    .div_reg = CM_GP1DIV,
    .int_bits = 12,
    .frac_bits = 12,
    .flags = CLK_IS_CRITICAL,
    .is_mash_clock = true,
    .tcnt_mux = 21),
    [BCM2835_CLOCK_GP2]	= REGISTER_PER_CLK(
    SOC_ALL,
    .name = "gp2",
    .ctl_reg = CM_GP2CTL,
    .div_reg = CM_GP2DIV,
    .int_bits = 12,
    .frac_bits = 12,
    .flags = CLK_IS_CRITICAL),
// HDMI state machine
    [BCM2835_CLOCK_HSM]	= REGISTER_PER_CLK(
    SOC_ALL,
    .name = "hsm",
    .ctl_reg = CM_HSMCTL,
    .div_reg = CM_HSMDIV,
    .int_bits = 4,
    .frac_bits = 8,
    .tcnt_mux = 22),
    [BCM2835_CLOCK_PCM]	= REGISTER_PCM_CLK(
    SOC_ALL,
    .name = "pcm",
    .ctl_reg = CM_PCMCTL,
    .div_reg = CM_PCMDIV,
    .int_bits = 12,
    .frac_bits = 12,
    .is_mash_clock = true,
    .low_jitter = true,
    .tcnt_mux = 23),
    [BCM2835_CLOCK_PWM]	= REGISTER_PER_CLK(
    SOC_ALL,
    .name = "pwm",
    .ctl_reg = CM_PWMCTL,
    .div_reg = CM_PWMDIV,
    .int_bits = 12,
    .frac_bits = 12,
    .is_mash_clock = true,
    .tcnt_mux = 24),
    [BCM2835_CLOCK_SLIM]	= REGISTER_PER_CLK(
    SOC_ALL,
    .name = "slim",
    .ctl_reg = CM_SLIMCTL,
    .div_reg = CM_SLIMDIV,
    .int_bits = 12,
    .frac_bits = 12,
    .is_mash_clock = true,
    .tcnt_mux = 25),
    [BCM2835_CLOCK_SMI]	= REGISTER_PER_CLK(
    SOC_ALL,
    .name = "smi",
    .ctl_reg = CM_SMICTL,
    .div_reg = CM_SMIDIV,
    .int_bits = 4,
    .frac_bits = 8,
    .tcnt_mux = 27),
    [BCM2835_CLOCK_UART]	= REGISTER_PER_CLK(
    SOC_ALL,
    .name = "uart",
    .ctl_reg = CM_UARTCTL,
    .div_reg = CM_UARTDIV,
    .int_bits = 10,
    .frac_bits = 12,
    .tcnt_mux = 28,
    .round_up = true),
// TV encoder clock.  Only operating frequency is 108Mhz.
    [BCM2835_CLOCK_VEC]	= REGISTER_PER_CLK(
    SOC_ALL,
    .name = "vec",
    .ctl_reg = CM_VECCTL,
    .div_reg = CM_VECDIV,
    .int_bits = 4,
    .frac_bits = 0,
//
// Allow rate change propagation only on PLLH_AUX which is
// assigned index 7 in the parent array.
//
    .set_rate_parent = BIT(7),
    .tcnt_mux = 29),
// dsi clocks
    [BCM2835_CLOCK_DSI0E]	= REGISTER_PER_CLK(
    SOC_ALL,
    .name = "dsi0e",
    .ctl_reg = CM_DSI0ECTL,
    .div_reg = CM_DSI0EDIV,
    .int_bits = 4,
    .frac_bits = 8,
    .tcnt_mux = 18),
    [BCM2835_CLOCK_DSI1E]	= REGISTER_PER_CLK(
    SOC_ALL,
    .name = "dsi1e",
    .ctl_reg = CM_DSI1ECTL,
    .div_reg = CM_DSI1EDIV,
    .int_bits = 4,
    .frac_bits = 8,
    .tcnt_mux = 19),
    [BCM2835_CLOCK_DSI0P]	= REGISTER_DSI0_CLK(
    SOC_ALL,
    .name = "dsi0p",
    .ctl_reg = CM_DSI0PCTL,
    .div_reg = CM_DSI0PDIV,
    .int_bits = 0,
    .frac_bits = 0,
    .tcnt_mux = 12),
    [BCM2835_CLOCK_DSI1P]	= REGISTER_DSI1_CLK(
    SOC_ALL,
    .name = "dsi1p",
    .ctl_reg = CM_DSI1PCTL,
    .div_reg = CM_DSI1PDIV,
    .int_bits = 0,
    .frac_bits = 0,
    .tcnt_mux = 13),
// the gates
//
// CM_PERIICTL (and CM_PERIACTL, CM_SYSCTL and CM_VPUCTL if
// you have the debug bit set in the power manager, which we
// don't bother exposing) are individual gates off of the
// non-stop vpu clock.
//
    [BCM2835_CLOCK_PERI_IMAGE] = REGISTER_GATE(
    SOC_ALL,
    .name = "peri_image",
    .parent = "vpu",
    .ctl_reg = CM_PERIICTL),
    };
//
// Permanently take a reference on the parent of the SDRAM clock.
//
// While the SDRAM is being driven by its dedicated PLL most of the
// time, there is a little loop running in the firmware that
// periodically switches the SDRAM to using our CM clock to do PVT
// recalibration, with the assumption that the previously configured
// SDRAM parent is still enabled and running.
//
#[no_mangle]
unsafe extern "C" fn bcm2835_mark_sdc_parent_critical(sdc: *mut clk) -> c_int {
    static int bcm2835_mark_sdc_parent_critical(struct clk *sdc)
    {
    struct clk *parent = clk_get_parent(sdc);
    if (IS_ERR(parent))
    return PTR_ERR(parent);
    return clk_prepare_enable(parent);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_clk_probe(pdev: *mut platform_device) -> c_int {
    static int bcm2835_clk_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct clk_hw **hws;
    struct bcm2835_cprman *cprman;
    const struct bcm2835_clk_desc *desc;
    let mut asize: usize = ARRAY_SIZE(clk_desc_array);
    const struct cprman_plat_data *pdata;
    size_t i;
    int ret;
    pdata = of_device_get_match_data(&pdev.dev);
    if (!pdata)
    return -ENODEV;
    cprman = devm_kzalloc(dev,
    struct_size(cprman, onecell.hws, asize),
    GFP_KERNEL);
    if (!cprman)
    return -ENOMEM;
    spin_lock_init(&cprman.regs_lock);
    cprman.dev = dev;
    cprman.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(cprman.regs))
    return PTR_ERR(cprman.regs);
    memcpy(cprman.real_parent_names, cprman_parent_names,
    sizeof(cprman_parent_names));
    of_clk_parent_fill(dev.of_node, cprman.real_parent_names,
    ARRAY_SIZE(cprman_parent_names));
//
// Make sure the external oscillator has been registered.
//
// The other (DSI) clocks are not present on older device
// trees, which we still need to support for backwards
// compatibility.
//
    if (!cprman.real_parent_names[0])
    return -ENODEV;
    platform_set_drvdata(pdev, cprman);
    cprman.onecell.num = asize;
    cprman.soc = pdata.soc;
    hws = cprman.onecell.hws;
    for (i = 0; i < asize; i++) {
    desc = &clk_desc_array[i];
    if (desc.clk_register && desc.data &&
    (desc.supported & pdata.soc)) {
    hws[i] = desc.clk_register(cprman, desc.data);
    }
    }
    ret = bcm2835_mark_sdc_parent_critical(hws[BCM2835_CLOCK_SDRAM].clk);
    if (ret)
    return ret;
    return of_clk_add_hw_provider(dev.of_node, of_clk_hw_onecell_get,
    &cprman.onecell);
    }
    static const struct cprman_plat_data cprman_bcm2835_plat_data = {
    .soc = SOC_BCM2835,
    };
    static const struct cprman_plat_data cprman_bcm2711_plat_data = {
    .soc = SOC_BCM2711,
    };
    static const struct of_device_id bcm2835_clk_of_match[] = {
    { .compatible = "brcm,bcm2835-cprman", .data = &cprman_bcm2835_plat_data },
    { .compatible = "brcm,bcm2711-cprman", .data = &cprman_bcm2711_plat_data },
    {}
    };
    MODULE_DEVICE_TABLE(of, bcm2835_clk_of_match);
    static struct platform_driver bcm2835_clk_driver = {
    .driver = {
    .name = "bcm2835-clk",
    .of_match_table = bcm2835_clk_of_match,
    },
    .probe          = bcm2835_clk_probe,
    };
    builtin_platform_driver(bcm2835_clk_driver);
    MODULE_AUTHOR("Eric Anholt <eric@anholt.net>");
    MODULE_DESCRIPTION("BCM2835 clock driver");
