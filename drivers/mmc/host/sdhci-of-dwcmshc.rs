//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/sdhci-of-dwcmshc.c
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
// Driver for Synopsys DesignWare Cores Mobile Storage Host Controller
//
// Copyright (C) 2018 Synaptics Incorporated
//
// Author: Jisheng Zhang <jszhang@kernel.org>
//

// DWCMSHC specific Mode Select value
pub const DWCMSHC_CTRL_HS400: c_uint = 0x7;
// DWC IP vendor area 1 pointer
pub const DWCMSHC_P_VENDOR_AREA1: c_uint = 0xe8;

// Offset inside the  vendor area 1
pub const DWCMSHC_HOST_CTRL3: c_uint = 0x8;

pub const DWCMSHC_EMMC_CONTROL: c_uint = 0x2c;
// HPE GSC SoC MSHCCS register

pub const DWCMSHC_EMMC_ATCTRL: c_uint = 0x40;
pub const DWCMSHC_AT_STAT: c_uint = 0x44;
// Tuning and auto-tuning fields in AT_CTRL_R control register

pub const AT_CTRL_WIN_EDGE_SEL: c_uint = 0xf /* sampling window edge select */;

pub const AT_CTRL_PRE_CHANGE_DLY: c_uint = 0x1  /* 2-cycle latency */;

pub const AT_CTRL_POST_CHANGE_DLY: c_uint = 0x3  /* 4-cycle latency */;

pub const AT_CTRL_SWIN_TH_VAL: c_uint = 0x9  /* sampling window threshold */;
// DWC IP vendor area 2 pointer
pub const DWCMSHC_P_VENDOR_AREA2: c_uint = 0xea;
// Sophgo CV18XX specific Registers
pub const CV18XX_SDHCI_MSHC_CTRL: c_uint = 0x00;

pub const CV18XX_SDHCI_PHY_TX_RX_DLY: c_uint = 0x40;

pub const CV18XX_PHY_TX_SRC_INVERT_CLK_TX: c_uint = 0x1;

pub const CV18XX_PHY_RX_SRC_INVERT_RX_CLK: c_uint = 0x1;
pub const CV18XX_SDHCI_PHY_CONFIG: c_uint = 0x4c;

pub const CV18XX_TUNE_MAX: c_int = 128;
pub const CV18XX_TUNE_STEP: c_int = 1;
pub const CV18XX_RETRY_TUNING_MAX: c_int = 50;
// Rockchip specific Registers
pub const DWCMSHC_EMMC_DLL_CTRL: c_uint = 0x800;
pub const DWCMSHC_EMMC_DLL_RXCLK: c_uint = 0x804;
pub const DWCMSHC_EMMC_DLL_TXCLK: c_uint = 0x808;
pub const DWCMSHC_EMMC_DLL_STRBIN: c_uint = 0x80c;
pub const DECMSHC_EMMC_DLL_CMDOUT: c_uint = 0x810;
pub const DECMSHC_EMMC_MISC_CON: c_uint = 0x81C;

pub const DWCMSHC_EMMC_DLL_STATUS0: c_uint = 0x840;

pub const DWCMSHC_EMMC_DLL_RXCLK_SRCSEL: c_int = 29;
pub const DWCMSHC_EMMC_DLL_START_POINT: c_int = 16;
pub const DWCMSHC_EMMC_DLL_INC: c_int = 8;

pub const DLL_TXCLK_TAPNUM_DEFAULT: c_uint = 0x10;
pub const DLL_TXCLK_TAPNUM_90_DEGREES: c_uint = 0xA;

pub const DLL_STRBIN_TAPNUM_DEFAULT: c_uint = 0x4;

pub const DLL_STRBIN_DELAY_NUM_OFFSET: c_int = 16;
pub const DLL_STRBIN_DELAY_NUM_DEFAULT: c_uint = 0x16;
pub const DLL_RXCLK_NO_INVERTER: c_int = 1;
pub const DLL_RXCLK_INVERTER: c_int = 0;
pub const DLL_CMDOUT_TAPNUM_90_DEGREES: c_uint = 0x8;

    ((((x) & DWCMSHC_EMMC_DLL_LOCKED) == DWCMSHC_EMMC_DLL_LOCKED) && \
    (((x) & DWCMSHC_EMMC_DLL_TIMEOUT) == 0))
// PHY register area pointer
pub const DWC_MSHC_PTR_PHY_R: c_uint = 0x300;
// PHY general configuration

pub const PHY_CNFG_RSTN_DEASSERT: c_uint = 0x1  /* Deassert PHY reset */;

pub const PHY_CNFG_PAD_SP: c_uint = 0x0c /* PMOS TX drive strength */;
pub const PHY_CNFG_PAD_SP_k230: c_uint = 0x09 /* PMOS TX drive strength for k230 */;
pub const PHY_CNFG_PAD_SP_SG2042: c_uint = 0x09 /* PMOS TX drive strength for SG2042 */;

pub const PHY_CNFG_PAD_SN: c_uint = 0x0c /* NMOS TX drive strength */;
pub const PHY_CNFG_PAD_SN_k230: c_uint = 0x08 /* NMOS TX drive strength for k230 */;
pub const PHY_CNFG_PAD_SN_SG2042: c_uint = 0x08 /* NMOS TX drive strength for SG2042 */;
// PHY command/response pad settings

// PHY data pad settings

// PHY clock pad settings

// PHY strobe pad settings

// PHY reset pad settings

// Bitfields are common for all pad settings
pub const PHY_PAD_RXSEL_1V8: c_uint = 0x1 /* Receiver type select for 1.8V */;
pub const PHY_PAD_RXSEL_3V3: c_uint = 0x2 /* Receiver type select for 3.3V */;

pub const PHY_PAD_WEAKPULL_DISABLED: c_uint = 0x0 /* Weak pull up and pull down disabled */;
pub const PHY_PAD_WEAKPULL_PULLUP: c_uint = 0x1 /* Weak pull up enabled */;
pub const PHY_PAD_WEAKPULL_PULLDOWN: c_uint = 0x2 /* Weak pull down enabled */;

pub const PHY_PAD_TXSLEW_CTRL_P: c_uint = 0x3 /* Slew control for P-Type pad TX */;
pub const PHY_PAD_TXSLEW_CTRL_P_k230: c_uint = 0x2 /* Slew control for P-Type pad TX for k230 */;

pub const PHY_PAD_TXSLEW_CTRL_N: c_uint = 0x3 /* Slew control for N-Type pad TX */;
pub const PHY_PAD_TXSLEW_CTRL_N_SG2042: c_uint = 0x2 /* Slew control for N-Type pad TX for SG2042 */;
pub const PHY_PAD_TXSLEW_CTRL_N_k230: c_uint = 0x2 /* Slew control for N-Type pad TX for k230 */;
// PHY Common DelayLine config settings

// PHY CLK delay line settings

// PHY CLK delay line delay code

pub const PHY_SDCLKDL_DC_INITIAL: c_uint = 0x40 /* initial delay code */;
pub const PHY_SDCLKDL_DC_DEFAULT: c_uint = 0x32 /* default delay code */;
pub const PHY_SDCLKDL_DC_HS400: c_uint = 0x18 /* delay code for HS400 mode */;

pub const PHY_SMPLDL_CNFG_INPSEL: c_uint = 0x3 /* delay line input source */;
// PHY drift_cclk_rx delay line configuration setting

pub const PHY_ATDL_CNFG_INPSEL: c_uint = 0x3 /* delay line input source */;
pub const PHY_ATDL_CNFG_INPSEL_SG2042: c_uint = 0x2 /* delay line input source for SG2042 */;
// PHY DLL control settings

pub const PHY_DLL_CTRL_DISABLE: c_uint = 0x0 /* PHY DLL is enabled */;
pub const PHY_DLL_CTRL_ENABLE: c_uint = 0x1 /* PHY DLL is disabled */;
// PHY DLL  configuration register 1

pub const PHY_DLL_CNFG1_SLVDLY: c_uint = 0x2 /* DLL slave update delay input */;
pub const PHY_DLL_CNFG1_WAITCYCLE: c_uint = 0x5 /* DLL wait cycle input */;
// PHY DLL configuration register 2

pub const PHY_DLL_CNFG2_JUMPSTEP: c_uint = 0xa /* DLL jump step input */;
// PHY DLL master and slave delay line configuration settings

pub const PHY_DLLDL_CNFG_SLV_INPSEL: c_uint = 0x3 /* clock source select for slave DL */;
// PHY DLL offset setting register

// DLL LBT setting register

// DLL Status register

//
// Captures the value of DLL's lock error status information. Value is valid
// only when LOCK_STS is set.
//

    ((addr | (SZ_128M - 1)) == ((addr + len - 1) | (SZ_128M - 1)))

    SDHCI_TRNS_BLK_CNT_EN | \
    SDHCI_TRNS_DMA)

    container_of((priv).dwcmshc_pdata, struct name##_pltfm_data, dwcmshc_pdata)
// SMC call for BlueField-3 eMMC RST_N
pub const BLUEFIELD_SMC_SET_EMMC_RST_N: c_uint = 0x82000007;
// Canaan specific Registers
pub const SD0_CTRL: c_uint = 0x00;

pub const SD1_CTRL: c_uint = 0x08;

// Eswin specific Registers

    EIC7700_INT_ACLK_STABLE | \
    EIC7700_INT_BCLK_STABLE | \
    EIC7700_INT_TMCLK_STABLE)

// strength definition
pub const PHYCTRL_DR_33OHM: c_uint = 0xee;
pub const PHYCTRL_DR_40OHM: c_uint = 0xcc;
pub const PHYCTRL_DR_50OHM: c_uint = 0x88;
pub const PHYCTRL_DR_66OHM: c_uint = 0x44;
pub const PHYCTRL_DR_100OHM: c_uint = 0x00;
pub const MAX_PHASE_CODE: c_uint = 0xff;
pub const TUNING_RANGE_THRESHOLD: c_int = 40;
pub const PHY_CLK_MAX_DELAY_MASK: c_uint = 0x7f;
pub const PHY_DELAY_CODE_MAX: c_uint = 0x7f;
pub const PHY_DELAY_CODE_EMMC: c_uint = 0x17;
pub const PHY_DELAY_CODE_SD: c_uint = 0x55;
pub const PHY_DELAY_CODE_SDIO: c_uint = 0x29;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rk35xx_priv {
    pub reset: *mut reset_control,
    pub txclk_tapnum: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eic7700_priv {
    pub reset: *mut reset_control,
    pub drive_impedance: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct k230_priv {
// Canaan k230 specific
    pub hi_sys_regmap: *mut regmap,
}

pub const DWCMSHC_MAX_OTHER_CLKS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwcmshc_priv {
    pub bus_clk: *mut clk,
    pub /: *mut *mut int vendor_specific_area1; / P_VENDOR_SPECIFIC_AREA1 reg,
    pub /: *mut *mut int vendor_specific_area2; / P_VENDOR_SPECIFIC_AREA2 reg,
    pub num_other_clks: c_int,
    pub other_clks: [clk_bulk_data; DWCMSHC_MAX_OTHER_CLKS],
    pub dwcmshc_pdata: *const dwcmshc_pltfm_data,
    pub /: *mut *mut *mut void priv; / pointer to SoC private stuff,
    pub delay_line: u16,
    pub flags: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwcmshc_pltfm_data {
    pub pdata: sdhci_pltfm_data,
    pub cqhci_host_ops: *const cqhci_host_ops,
    pub dwc_priv): *mut *mut *mut *mut int (init)(struct device dev, struct sdhci_host host, struct dwcmshc_priv,
    pub dwc_priv): *mut *mut *mut void (postinit)(struct sdhci_host host, struct dwcmshc_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct k230_pltfm_data {
    pub dwcmshc_pdata: dwcmshc_pltfm_data,
    pub is_emmc: bool,
    pub ctrl_reg: u32,
    pub vol_stable_bit: u32,
    pub write_prot_bit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_pltfm_data {
    pub dwcmshc_pdata: dwcmshc_pltfm_data,
//
// The controller hardware has two known revisions documented internally:
// - Revision 0: Exclusively used by RK3566 and RK3568 SoCs.
// - Revision 1: Implemented in all other Rockchip SoCs, including RK3576, RK3588, etc.
//
    pub revision: c_int,
}

#[no_mangle]
unsafe extern "C" fn dwcmshc_enable_card_clk(host: *mut sdhci_host) {
    static void dwcmshc_enable_card_clk(struct sdhci_host *host)
    {
    u16 ctrl;
    ctrl = sdhci_readw(host, SDHCI_CLOCK_CONTROL);
    if ((ctrl & SDHCI_CLOCK_INT_EN) && !(ctrl & SDHCI_CLOCK_CARD_EN)) {
    ctrl |= SDHCI_CLOCK_CARD_EN;
    sdhci_writew(host, ctrl, SDHCI_CLOCK_CONTROL);
    }
    }
    static int dwcmshc_get_enable_other_clks(struct device *dev,
    struct dwcmshc_priv *priv,
    int num_clks,
    const char * const clk_ids[])
    {
    int err;
    if (num_clks > DWCMSHC_MAX_OTHER_CLKS)
    return -EINVAL;
    for (int i = 0; i < num_clks; i++)
    priv.other_clks[i].id = clk_ids[i];
    err = devm_clk_bulk_get_optional(dev, num_clks, priv.other_clks);
    if (err) {
    dev_err(dev, "failed to get clocks %d\n", err);
    return err;
    }
    err = clk_bulk_prepare_enable(num_clks, priv.other_clks);
    if (err)
    dev_err(dev, "failed to enable clocks %d\n", err);
    priv.num_other_clks = num_clks;
    return err;
    }
//
// If DMA addr spans 128MB boundary, we split the DMA transfer into two
// so that each DMA transfer doesn't exceed the boundary.
//
    static void dwcmshc_adma_write_desc(struct sdhci_host *host, void **desc,
    dma_addr_t addr, int len, unsigned int cmd)
    {
    int tmplen, offset;
    if (likely(!len || BOUNDARY_OK(addr, len))) {
    sdhci_adma_write_desc(host, desc, addr, len, cmd);
    return;
    }
    offset = addr & (SZ_128M - 1);
    tmplen = SZ_128M - offset;
    sdhci_adma_write_desc(host, desc, addr, tmplen, cmd);
    addr += tmplen;
    len -= tmplen;
    sdhci_adma_write_desc(host, desc, addr, len, cmd);
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_reset(host: *mut sdhci_host, mask: u8) {
    static void dwcmshc_reset(struct sdhci_host *host, u8 mask)
    {
    sdhci_reset(host, mask);
// The dwcmshc does not comply with the SDHCI specification
// regarding the "Software Reset for CMD line should clear 'Command
// Complete' in the Normal Interrupt Status Register." Clear the bit
// here to compensate for this quirk.
//
    if (mask & SDHCI_RESET_CMD)
    sdhci_writel(host, SDHCI_INT_RESPONSE, SDHCI_INT_STATUS);
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_get_max_clock(host: *mut sdhci_host) -> c_uint {
    static unsigned int dwcmshc_get_max_clock(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    if (pltfm_host.clk)
    return sdhci_pltfm_clk_get_max_clock(host);
    else
    return pltfm_host.clock;
    }
#[no_mangle]
unsafe extern "C" fn rk35xx_get_max_clock(host: *mut sdhci_host) -> c_uint {
    static unsigned int rk35xx_get_max_clock(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    return clk_round_rate(pltfm_host.clk, ULONG_MAX);
    }
    static void dwcmshc_check_auto_cmd23(struct mmc_host *mmc,
    struct mmc_request *mrq)
    {
    struct sdhci_host *host = mmc_priv(mmc);
//
// No matter V4 is enabled or not, ARGUMENT2 register is 32-bit
// block count register which doesn't support stuff bits of
// CMD23 argument on dwcmsch host controller.
//
    if (mrq.sbc && (mrq.sbc.arg & SDHCI_DWCMSHC_ARG2_STUFF))
    host.flags &= ~SDHCI_AUTO_CMD23;
    else
    host.flags |= SDHCI_AUTO_CMD23;
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_request(mmc: *mut mmc_host, mrq: *mut mmc_request) {
    static void dwcmshc_request(struct mmc_host *mmc, struct mmc_request *mrq)
    {
    dwcmshc_check_auto_cmd23(mmc, mrq);
    sdhci_request(mmc, mrq);
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_phy_init(host: *mut sdhci_host) {
    static void dwcmshc_phy_init(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *priv = sdhci_pltfm_priv(pltfm_host);
    let mut rxsel: u32 = PHY_PAD_RXSEL_3V3;
    u32 val;
    if (priv.flags & FLAG_IO_FIXED_1V8 ||
    host.mmc.ios.timing & MMC_SIGNAL_VOLTAGE_180)
    rxsel = PHY_PAD_RXSEL_1V8;
// deassert phy reset & set tx drive strength
    val = PHY_CNFG_RSTN_DEASSERT;
    val |= FIELD_PREP(PHY_CNFG_PAD_SP_MASK, PHY_CNFG_PAD_SP);
    val |= FIELD_PREP(PHY_CNFG_PAD_SN_MASK, PHY_CNFG_PAD_SN);
    sdhci_writel(host, val, PHY_CNFG_R);
// disable delay line
    sdhci_writeb(host, PHY_SDCLKDL_CNFG_UPDATE, PHY_SDCLKDL_CNFG_R);
// set delay line
    sdhci_writeb(host, priv.delay_line, PHY_SDCLKDL_DC_R);
    sdhci_writeb(host, PHY_DLL_CNFG2_JUMPSTEP, PHY_DLL_CNFG2_R);
// enable delay lane
    val = sdhci_readb(host, PHY_SDCLKDL_CNFG_R);
    val &= ~(PHY_SDCLKDL_CNFG_UPDATE);
    sdhci_writeb(host, val, PHY_SDCLKDL_CNFG_R);
// configure phy pads
    val = rxsel;
    val |= FIELD_PREP(PHY_PAD_WEAKPULL_MASK, PHY_PAD_WEAKPULL_PULLUP);
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_P_MASK, PHY_PAD_TXSLEW_CTRL_P);
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_N_MASK, PHY_PAD_TXSLEW_CTRL_N);
    sdhci_writew(host, val, PHY_CMDPAD_CNFG_R);
    sdhci_writew(host, val, PHY_DATAPAD_CNFG_R);
    sdhci_writew(host, val, PHY_RSTNPAD_CNFG_R);
    val = FIELD_PREP(PHY_PAD_TXSLEW_CTRL_P_MASK, PHY_PAD_TXSLEW_CTRL_P);
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_N_MASK, PHY_PAD_TXSLEW_CTRL_N);
    sdhci_writew(host, val, PHY_CLKPAD_CNFG_R);
    val = rxsel;
    val |= FIELD_PREP(PHY_PAD_WEAKPULL_MASK, PHY_PAD_WEAKPULL_PULLDOWN);
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_P_MASK, PHY_PAD_TXSLEW_CTRL_P);
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_N_MASK, PHY_PAD_TXSLEW_CTRL_N);
    sdhci_writew(host, val, PHY_STBPAD_CNFG_R);
// enable data strobe mode
    if (rxsel == PHY_PAD_RXSEL_1V8) {
    let mut sel: u8 = FIELD_PREP(PHY_DLLDL_CNFG_SLV_INPSEL_MASK, PHY_DLLDL_CNFG_SLV_INPSEL);
    sdhci_writeb(host, sel, PHY_DLLDL_CNFG_R);
    }
// enable phy dll
    sdhci_writeb(host, PHY_DLL_CTRL_ENABLE, PHY_DLL_CTRL_R);
    }
#[no_mangle]
unsafe extern "C" fn th1520_sdhci_set_phy(host: *mut sdhci_host) {
    static void th1520_sdhci_set_phy(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *priv = sdhci_pltfm_priv(pltfm_host);
    let mut emmc_caps: u32 = MMC_CAP2_NO_SD | MMC_CAP2_NO_SDIO;
    u16 emmc_ctrl;
    dwcmshc_phy_init(host);
    if ((host.mmc.caps2 & emmc_caps) == emmc_caps) {
    emmc_ctrl = sdhci_readw(host, priv.vendor_specific_area1 + DWCMSHC_EMMC_CONTROL);
    emmc_ctrl |= DWCMSHC_CARD_IS_EMMC;
    sdhci_writew(host, emmc_ctrl, priv.vendor_specific_area1 + DWCMSHC_EMMC_CONTROL);
    }
    sdhci_writeb(host, FIELD_PREP(PHY_DLL_CNFG1_SLVDLY_MASK, PHY_DLL_CNFG1_SLVDLY) |
    PHY_DLL_CNFG1_WAITCYCLE, PHY_DLL_CNFG1_R);
    }
    static void dwcmshc_set_uhs_signaling(struct sdhci_host *host,
    unsigned int timing)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *priv = sdhci_pltfm_priv(pltfm_host);
    u16 ctrl, ctrl_2;
    ctrl_2 = sdhci_readw(host, SDHCI_HOST_CONTROL2);
// Select Bus Speed Mode for host
    ctrl_2 &= ~SDHCI_CTRL_UHS_MASK;
    if ((timing == MMC_TIMING_MMC_HS200) ||
    (timing == MMC_TIMING_UHS_SDR104))
    ctrl_2 |= SDHCI_CTRL_UHS_SDR104;
#[no_mangle]
pub unsafe extern "C" fn if(MMC_TIMING_UHS_SDR12: timing ==) -> else {
    else if (timing == MMC_TIMING_UHS_SDR12)
    ctrl_2 |= SDHCI_CTRL_UHS_SDR12;
    else if ((timing == MMC_TIMING_UHS_SDR25) ||
    (timing == MMC_TIMING_MMC_HS))
    ctrl_2 |= SDHCI_CTRL_UHS_SDR25;
#[no_mangle]
pub unsafe extern "C" fn if(MMC_TIMING_UHS_SDR50: timing ==) -> else {
    else if (timing == MMC_TIMING_UHS_SDR50)
    ctrl_2 |= SDHCI_CTRL_UHS_SDR50;
    else if ((timing == MMC_TIMING_UHS_DDR50) ||
    (timing == MMC_TIMING_MMC_DDR52))
    ctrl_2 |= SDHCI_CTRL_UHS_DDR50;
#[no_mangle]
pub unsafe extern "C" fn if(MMC_TIMING_MMC_HS400: timing ==) -> else {
// set CARD_IS_EMMC bit to enable Data Strobe for HS400
    ctrl = sdhci_readw(host, priv.vendor_specific_area1 + DWCMSHC_EMMC_CONTROL);
    ctrl |= DWCMSHC_CARD_IS_EMMC;
    sdhci_writew(host, ctrl, priv.vendor_specific_area1 + DWCMSHC_EMMC_CONTROL);
    ctrl_2 |= DWCMSHC_CTRL_HS400;
    }
    if (priv.flags & FLAG_IO_FIXED_1V8)
    ctrl_2 |= SDHCI_CTRL_VDD_180;
    sdhci_writew(host, ctrl_2, SDHCI_HOST_CONTROL2);
    }
    static void th1520_set_uhs_signaling(struct sdhci_host *host,
    unsigned int timing)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *priv = sdhci_pltfm_priv(pltfm_host);
    dwcmshc_set_uhs_signaling(host, timing);
    if (timing == MMC_TIMING_MMC_HS400)
    priv.delay_line = PHY_SDCLKDL_DC_HS400;
    else
    sdhci_writeb(host, 0, PHY_DLLDL_CNFG_R);
    th1520_sdhci_set_phy(host);
    }
    static void dwcmshc_hs400_enhanced_strobe(struct mmc_host *mmc,
    struct mmc_ios *ios)
    {
    u32 vendor;
    struct sdhci_host *host = mmc_priv(mmc);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *priv = sdhci_pltfm_priv(pltfm_host);
    let mut reg: c_int = priv.vendor_specific_area1 + DWCMSHC_EMMC_CONTROL;
    vendor = sdhci_readl(host, reg);
    if (ios.enhanced_strobe)
    vendor |= DWCMSHC_ENHANCED_STROBE;
    else
    vendor &= ~DWCMSHC_ENHANCED_STROBE;
    sdhci_writel(host, vendor, reg);
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_execute_tuning(mmc: *mut mmc_host, opcode: u32) -> c_int {
    static int dwcmshc_execute_tuning(struct mmc_host *mmc, u32 opcode)
    {
    let mut err: c_int = sdhci_execute_tuning(mmc, opcode);
    struct sdhci_host *host = mmc_priv(mmc);
    if (err)
    return err;
//
// Tuning can leave the IP in an active state (Buffer Read Enable bit
// set) which prevents the entry to low power states (i.e. S0i3). Data
// reset will clear it.
//
    sdhci_reset(host, SDHCI_RESET_DATA);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_cqe_irq_handler(host: *mut sdhci_host, intmask: u32) -> u32 {
    static u32 dwcmshc_cqe_irq_handler(struct sdhci_host *host, u32 intmask)
    {
    let mut cmd_error: c_int = 0;
    let mut data_error: c_int = 0;
    if (!sdhci_cqe_irq(host, intmask, &cmd_error, &data_error))
    return intmask;
    cqhci_irq(host.mmc, cmd_error, data_error);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_sdhci_cqe_enable(mmc: *mut mmc_host) {
    static void dwcmshc_sdhci_cqe_enable(struct mmc_host *mmc)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    u8 ctrl;
    sdhci_writew(host, DWCMSHC_SDHCI_CQE_TRNS_MODE, SDHCI_TRANSFER_MODE);
    sdhci_cqe_enable(mmc);
//
// The "DesignWare Cores Mobile Storage Host Controller
// DWC_mshc / DWC_mshc_lite Databook" says:
// when Host Version 4 Enable" is 1 in Host Control 2 register,
// SDHCI_CTRL_ADMA32 bit means ADMA2 is selected.
// Selection of 32-bit/64-bit System Addressing:
// either 32-bit or 64-bit system addressing is selected by
// 64-bit Addressing bit in Host Control 2 register.
//
// On the other hand the "DesignWare Cores Mobile Storage Host
// Controller DWC_mshc / DWC_mshc_lite User Guide" says, that we have to
// set DMA_SEL to ADMA2 _only_ mode in the Host Control 2 register.
//
    ctrl = sdhci_readb(host, SDHCI_HOST_CONTROL);
    ctrl &= ~SDHCI_CTRL_DMA_MASK;
    ctrl |= SDHCI_CTRL_ADMA32;
    sdhci_writeb(host, ctrl, SDHCI_HOST_CONTROL);
    }
    static void dwcmshc_set_tran_desc(struct cqhci_host *cq_host, u8 **desc,
    dma_addr_t addr, int len, bool end, bool dma64)
    {
    int tmplen, offset;
    if (likely(!len || BOUNDARY_OK(addr, len))) {
    cqhci_set_tran_desc(*desc, addr, len, end, dma64);
    return;
    }
    offset = addr & (SZ_128M - 1);
    tmplen = SZ_128M - offset;
    cqhci_set_tran_desc(*desc, addr, tmplen, false, dma64);
    addr += tmplen;
    len -= tmplen;
// desc += cq_host->trans_desc_len;
    cqhci_set_tran_desc(*desc, addr, len, end, dma64);
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_cqhci_dumpregs(mmc: *mut mmc_host) {
    static void dwcmshc_cqhci_dumpregs(struct mmc_host *mmc)
    {
    sdhci_dumpregs(mmc_priv(mmc));
    }
#[no_mangle]
unsafe extern "C" fn rk35xx_sdhci_cqe_pre_enable(mmc: *mut mmc_host) {
    static void rk35xx_sdhci_cqe_pre_enable(struct mmc_host *mmc)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *dwc_priv = sdhci_pltfm_priv(pltfm_host);
    u32 reg;
// Set Send Status Command Idle Timer to 10.66us (256 * 1 / 24)
    reg = sdhci_readl(host, dwc_priv.vendor_specific_area2 + CQHCI_SSC1);
    reg = (reg & ~CQHCI_SSC1_CIT_MASK) | 0x0100;
    sdhci_writel(host, reg, dwc_priv.vendor_specific_area2 + CQHCI_SSC1);
    reg = sdhci_readl(host, dwc_priv.vendor_specific_area2 + CQHCI_CFG);
    reg |= CQHCI_ENABLE;
    sdhci_writel(host, reg, dwc_priv.vendor_specific_area2 + CQHCI_CFG);
    }
#[no_mangle]
unsafe extern "C" fn rk35xx_sdhci_cqe_enable(mmc: *mut mmc_host) {
    static void rk35xx_sdhci_cqe_enable(struct mmc_host *mmc)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    u32 reg;
    reg = sdhci_readl(host, SDHCI_PRESENT_STATE);
    while (reg & SDHCI_DATA_AVAILABLE) {
    sdhci_readl(host, SDHCI_BUFFER);
    reg = sdhci_readl(host, SDHCI_PRESENT_STATE);
    }
    sdhci_writew(host, DWCMSHC_SDHCI_CQE_TRNS_MODE, SDHCI_TRANSFER_MODE);
    sdhci_cqe_enable(mmc);
    }
#[no_mangle]
unsafe extern "C" fn rk35xx_sdhci_cqe_disable(mmc: *mut mmc_host, recovery: bool) {
    static void rk35xx_sdhci_cqe_disable(struct mmc_host *mmc, bool recovery)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    unsigned long flags;
    u32 ctrl;
//
// During CQE command transfers, command complete bit gets latched.
// So s/w should clear command complete interrupt status when CQE is
// either halted or disabled. Otherwise unexpected SDCHI legacy
// interrupt gets triggered when CQE is halted/disabled.
//
    spin_lock_irqsave(&host.lock, flags);
    ctrl = sdhci_readl(host, SDHCI_INT_ENABLE);
    ctrl |= SDHCI_INT_RESPONSE;
    sdhci_writel(host,  ctrl, SDHCI_INT_ENABLE);
    sdhci_writel(host, SDHCI_INT_RESPONSE, SDHCI_INT_STATUS);
    spin_unlock_irqrestore(&host.lock, flags);
    sdhci_cqe_disable(mmc, recovery);
    }
#[no_mangle]
unsafe extern "C" fn rk35xx_sdhci_cqe_post_disable(mmc: *mut mmc_host) {
    static void rk35xx_sdhci_cqe_post_disable(struct mmc_host *mmc)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *dwc_priv = sdhci_pltfm_priv(pltfm_host);
    u32 ctrl;
    ctrl = sdhci_readl(host, dwc_priv.vendor_specific_area2 + CQHCI_CFG);
    ctrl &= ~CQHCI_ENABLE;
    sdhci_writel(host, ctrl, dwc_priv.vendor_specific_area2 + CQHCI_CFG);
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_rk3568_set_clock(host: *mut sdhci_host, clock: c_uint) {
    static void dwcmshc_rk3568_set_clock(struct sdhci_host *host, unsigned int clock)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *dwc_priv = sdhci_pltfm_priv(pltfm_host);
    struct rk35xx_priv *priv = dwc_priv.priv;
    const struct rockchip_pltfm_data *rockchip_pdata = to_pltfm_data(dwc_priv, rockchip);
    let mut txclk_tapnum: u8 = DLL_TXCLK_TAPNUM_DEFAULT;
    u32 extra, reg;
    int err;
    host.mmc.actual_clock = 0;
    if (clock == 0) {
// Disable interface clock at initial state.
    sdhci_set_clock(host, clock);
    return;
    }
// Rockchip platform only support 375KHz for identify mode
    if (clock <= 400000)
    clock = 375000;
    err = clk_set_rate(pltfm_host.clk, clock);
    if (err)
    dev_err(mmc_dev(host.mmc), "fail to set clock %d", clock);
    sdhci_set_clock(host, clock);
// Disable cmd conflict check and internal clock gate
    reg = dwc_priv.vendor_specific_area1 + DWCMSHC_HOST_CTRL3;
    extra = sdhci_readl(host, reg);
    extra &= ~BIT(0);
    extra |= BIT(4);
    sdhci_writel(host, extra, reg);
// Disable clock while config DLL
    sdhci_writew(host, 0, SDHCI_CLOCK_CONTROL);
    if (clock <= 52000000) {
    if (host.mmc.ios.timing == MMC_TIMING_MMC_HS200 ||
    host.mmc.ios.timing == MMC_TIMING_MMC_HS400) {
    dev_err(mmc_dev(host.mmc),
    "Can't reduce the clock below 52MHz in HS200/HS400 mode");
    goto enable_clk;
    }
//
// Disable DLL and reset both of sample and drive clock.
// The bypass bit and start bit need to be set if DLL is not locked.
//
    sdhci_writel(host, DWCMSHC_EMMC_DLL_BYPASS | DWCMSHC_EMMC_DLL_START, DWCMSHC_EMMC_DLL_CTRL);
    sdhci_writel(host, DLL_RXCLK_ORI_GATE, DWCMSHC_EMMC_DLL_RXCLK);
    sdhci_writel(host, 0, DWCMSHC_EMMC_DLL_TXCLK);
    sdhci_writel(host, 0, DECMSHC_EMMC_DLL_CMDOUT);
//
// Before switching to hs400es mode, the driver will enable
// enhanced strobe first. PHY needs to configure the parameters
// of enhanced strobe first.
//
    extra = DWCMSHC_EMMC_DLL_DLYENA |
    DLL_STRBIN_DELAY_NUM_SEL |
    DLL_STRBIN_DELAY_NUM_DEFAULT << DLL_STRBIN_DELAY_NUM_OFFSET;
    sdhci_writel(host, extra, DWCMSHC_EMMC_DLL_STRBIN);
    goto enable_clk;
    }
// Reset DLL
    sdhci_writel(host, BIT(1), DWCMSHC_EMMC_DLL_CTRL);
    udelay(1);
    sdhci_writel(host, 0x0, DWCMSHC_EMMC_DLL_CTRL);
//
// We shouldn't set DLL_RXCLK_NO_INVERTER for identify mode but
// we must set it in higher speed mode.
//
    extra = DWCMSHC_EMMC_DLL_DLYENA;
    if (rockchip_pdata.revision == 0)
    extra |= DLL_RXCLK_NO_INVERTER << DWCMSHC_EMMC_DLL_RXCLK_SRCSEL;
    sdhci_writel(host, extra, DWCMSHC_EMMC_DLL_RXCLK);
// Init DLL settings
    extra = 0x5 << DWCMSHC_EMMC_DLL_START_POINT |
    0x2 << DWCMSHC_EMMC_DLL_INC |
    DWCMSHC_EMMC_DLL_START;
    sdhci_writel(host, extra, DWCMSHC_EMMC_DLL_CTRL);
    err = readl_poll_timeout(host.ioaddr + DWCMSHC_EMMC_DLL_STATUS0,
    extra, DLL_LOCK_WO_TMOUT(extra), 1,
    500 * USEC_PER_MSEC);
    if (err) {
    dev_err(mmc_dev(host.mmc), "DLL lock timeout!\n");
    goto enable_clk;
    }
    extra = 0x1 << 16 | /* tune clock stop en */
    0x3 << 17 | /* pre-change delay */
    0x3 << 19;  /* post-change delay */
    sdhci_writel(host, extra, dwc_priv.vendor_specific_area1 + DWCMSHC_EMMC_ATCTRL);
    if (host.mmc.ios.timing == MMC_TIMING_MMC_HS200 ||
    host.mmc.ios.timing == MMC_TIMING_MMC_HS400)
    txclk_tapnum = priv.txclk_tapnum;
    if (rockchip_pdata.revision == 1 && host.mmc.ios.timing == MMC_TIMING_MMC_HS400) {
    txclk_tapnum = DLL_TXCLK_TAPNUM_90_DEGREES;
    extra = DLL_CMDOUT_SRC_CLK_NEG |
    DLL_CMDOUT_EN_SRC_CLK_NEG |
    DWCMSHC_EMMC_DLL_DLYENA |
    DLL_CMDOUT_TAPNUM_90_DEGREES |
    DLL_CMDOUT_TAPNUM_FROM_SW;
    sdhci_writel(host, extra, DECMSHC_EMMC_DLL_CMDOUT);
    }
    extra = DWCMSHC_EMMC_DLL_DLYENA |
    DLL_TXCLK_TAPNUM_FROM_SW |
    DLL_RXCLK_NO_INVERTER << DWCMSHC_EMMC_DLL_RXCLK_SRCSEL |
    txclk_tapnum;
    sdhci_writel(host, extra, DWCMSHC_EMMC_DLL_TXCLK);
    extra = DWCMSHC_EMMC_DLL_DLYENA |
    DLL_STRBIN_TAPNUM_DEFAULT |
    DLL_STRBIN_TAPNUM_FROM_SW;
    sdhci_writel(host, extra, DWCMSHC_EMMC_DLL_STRBIN);
    enable_clk:
//
// The sdclk frequency select bits in SDHCI_CLOCK_CONTROL are not functional
// on Rockchip's SDHCI implementation. Instead, the clock frequency is fully
// controlled via external clk provider by calling clk_set_rate(). Consequently,
// passing 0 to sdhci_enable_clk() only re-enables the already-configured clock,
// which matches the hardware's actual behavior.
//
    sdhci_enable_clk(host, 0);
    }
#[no_mangle]
unsafe extern "C" fn rk35xx_sdhci_reset(host: *mut sdhci_host, mask: u8) {
    static void rk35xx_sdhci_reset(struct sdhci_host *host, u8 mask)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *dwc_priv = sdhci_pltfm_priv(pltfm_host);
    struct rk35xx_priv *priv = dwc_priv.priv;
    let mut extra: u32 = sdhci_readl(host, DECMSHC_EMMC_MISC_CON);
    if ((host.mmc.caps2 & MMC_CAP2_CQE) && (mask & SDHCI_RESET_ALL))
    cqhci_deactivate(host.mmc);
    if (mask & SDHCI_RESET_ALL && priv.reset) {
    reset_control_assert(priv.reset);
    udelay(1);
    reset_control_deassert(priv.reset);
    }
    sdhci_reset(host, mask);
// Enable INTERNAL CLOCK
    sdhci_writel(host, MISC_INTCLK_EN | extra, DECMSHC_EMMC_MISC_CON);
    }
    static int dwcmshc_rk35xx_init(struct device *dev, struct sdhci_host *host,
    struct dwcmshc_priv *dwc_priv)
    {
    static const char * const clk_ids[] = {"axi", "block", "timer"};
    struct rk35xx_priv *priv;
    int err;
    priv = devm_kzalloc(dev, sizeof(struct rk35xx_priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.reset = devm_reset_control_array_get_optional_exclusive(mmc_dev(host.mmc));
    if (IS_ERR(priv.reset))
    return dev_err_probe(mmc_dev(host.mmc), PTR_ERR(priv.reset),
    "failed to get reset control\n");
    err = dwcmshc_get_enable_other_clks(mmc_dev(host.mmc), dwc_priv,
    ARRAY_SIZE(clk_ids), clk_ids);
    if (err)
    return err;
    if (of_property_read_u8(mmc_dev(host.mmc).of_node, "rockchip,txclk-tapnum",
    &priv.txclk_tapnum))
    priv.txclk_tapnum = DLL_TXCLK_TAPNUM_DEFAULT;
// Disable cmd conflict check
    sdhci_writel(host, 0x0, dwc_priv.vendor_specific_area1 + DWCMSHC_HOST_CTRL3);
// Reset previous settings
    sdhci_writel(host, 0, DWCMSHC_EMMC_DLL_TXCLK);
    sdhci_writel(host, 0, DWCMSHC_EMMC_DLL_STRBIN);
    dwc_priv.priv = priv;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_rk35xx_postinit(host: *mut sdhci_host, dwc_priv: *mut dwcmshc_priv) {
    static void dwcmshc_rk35xx_postinit(struct sdhci_host *host, struct dwcmshc_priv *dwc_priv)
    {
//
// Don't support highspeed bus mode with low clk speed as we
// cannot use DLL for this condition.
//
    if (host.mmc.f_max <= 52000000) {
    dev_info(mmc_dev(host.mmc), "Disabling HS200/HS400, frequency too low (%d)\n",
    host.mmc.f_max);
    host.mmc.caps2 &= ~(MMC_CAP2_HS200 | MMC_CAP2_HS400);
    host.mmc.caps &= ~(MMC_CAP_3_3V_DDR | MMC_CAP_1_8V_DDR);
    }
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_rk3576_postinit(host: *mut sdhci_host, dwc_priv: *mut dwcmshc_priv) {
    static void dwcmshc_rk3576_postinit(struct sdhci_host *host, struct dwcmshc_priv *dwc_priv)
    {
    struct device *dev = mmc_dev(host.mmc);
    int ret;
//
// This works around the design of the RK3576's power domains, which
// makes the PD_NVM power domain, which the sdhci controller on the
// RK3576 is in, never come back the same way once it's run-time
// suspended once. This can happen during early kernel boot if no driver
// is using either PD_NVM or its child power domain PD_SDGMAC for a
// short moment, leading to it being turned off to save power. By
// keeping it on, sdhci suspending won't lead to PD_NVM becoming a
// candidate for getting turned off.
//
    ret = dev_pm_genpd_rpm_always_on(dev, true);
    if (ret && ret != -EOPNOTSUPP)
    dev_warn(dev, "failed to set PD rpm always on, SoC may hang later: %pe\n",
    ERR_PTR(ret));
    dwcmshc_rk35xx_postinit(host, dwc_priv);
    }
#[no_mangle]
unsafe extern "C" fn th1520_execute_tuning(host: *mut sdhci_host, opcode: u32) -> c_int {
    static int th1520_execute_tuning(struct sdhci_host *host, u32 opcode)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *priv = sdhci_pltfm_priv(pltfm_host);
    let mut val: u32 = 0;
    if (host.flags & SDHCI_HS400_TUNING)
    return 0;
    sdhci_writeb(host, FIELD_PREP(PHY_ATDL_CNFG_INPSEL_MASK, PHY_ATDL_CNFG_INPSEL),
    PHY_ATDL_CNFG_R);
    val = sdhci_readl(host, priv.vendor_specific_area1 + DWCMSHC_EMMC_ATCTRL);
//
// configure tuning settings:
// - center phase select code driven in block gap interval
// - disable reporting of framing errors
// - disable software managed tuning
// - disable user selection of sampling window edges,
// instead tuning calculated edges are used
//
    val &= ~(AT_CTRL_CI_SEL | AT_CTRL_RPT_TUNE_ERR | AT_CTRL_SW_TUNE_EN |
    FIELD_PREP(AT_CTRL_WIN_EDGE_SEL_MASK, AT_CTRL_WIN_EDGE_SEL));
//
// configure tuning settings:
// - enable auto-tuning
// - enable sampling window threshold
// - stop clocks during phase code change
// - set max latency in cycles between tx and rx clocks
// - set max latency in cycles to switch output phase
// - set max sampling window threshold value
//
    val |= AT_CTRL_AT_EN | AT_CTRL_SWIN_TH_EN | AT_CTRL_TUNE_CLK_STOP_EN;
    val |= FIELD_PREP(AT_CTRL_PRE_CHANGE_DLY_MASK, AT_CTRL_PRE_CHANGE_DLY);
    val |= FIELD_PREP(AT_CTRL_POST_CHANGE_DLY_MASK, AT_CTRL_POST_CHANGE_DLY);
    val |= FIELD_PREP(AT_CTRL_SWIN_TH_VAL_MASK, AT_CTRL_SWIN_TH_VAL);
    sdhci_writel(host, val, priv.vendor_specific_area1 + DWCMSHC_EMMC_ATCTRL);
    val = sdhci_readl(host, priv.vendor_specific_area1 + DWCMSHC_EMMC_ATCTRL);
// perform tuning
    sdhci_start_tuning(host);
    host.tuning_loop_count = 128;
    host.tuning_err = __sdhci_execute_tuning(host, opcode);
    if (host.tuning_err) {
// disable auto-tuning upon tuning error
    val &= ~AT_CTRL_AT_EN;
    sdhci_writel(host, val, priv.vendor_specific_area1 + DWCMSHC_EMMC_ATCTRL);
    dev_err(mmc_dev(host.mmc), "tuning failed: %d\n", host.tuning_err);
    return -EIO;
    }
    sdhci_end_tuning(host);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn th1520_sdhci_reset(host: *mut sdhci_host, mask: u8) {
    static void th1520_sdhci_reset(struct sdhci_host *host, u8 mask)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *priv = sdhci_pltfm_priv(pltfm_host);
    u16 ctrl_2;
    dwcmshc_reset(host, mask);
    if (priv.flags & FLAG_IO_FIXED_1V8) {
    ctrl_2 = sdhci_readw(host, SDHCI_HOST_CONTROL2);
    if (!(ctrl_2 & SDHCI_CTRL_VDD_180)) {
    ctrl_2 |= SDHCI_CTRL_VDD_180;
    sdhci_writew(host, ctrl_2, SDHCI_HOST_CONTROL2);
    }
    }
    }
    static int th1520_init(struct device *dev,
    struct sdhci_host *host,
    struct dwcmshc_priv *dwc_priv)
    {
    dwc_priv.delay_line = PHY_SDCLKDL_DC_DEFAULT;
    if (device_property_read_bool(dev, "mmc-ddr-1_8v") ||
    device_property_read_bool(dev, "mmc-hs200-1_8v") ||
    device_property_read_bool(dev, "mmc-hs400-1_8v"))
    dwc_priv.flags |= FLAG_IO_FIXED_1V8;
    else
    dwc_priv.flags &= ~FLAG_IO_FIXED_1V8;
//
// start_signal_voltage_switch() will try 3.3V first
// then 1.8V. Use SDHCI_SIGNALING_180 rather than
// SDHCI_SIGNALING_330 to avoid setting voltage to 3.3V
// in sdhci_start_signal_voltage_switch().
//
    if (dwc_priv.flags & FLAG_IO_FIXED_1V8) {
    host.flags &= ~SDHCI_SIGNALING_330;
    host.flags |=  SDHCI_SIGNALING_180;
    }
    sdhci_enable_v4_mode(host);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cv18xx_sdhci_reset(host: *mut sdhci_host, mask: u8) {
    static void cv18xx_sdhci_reset(struct sdhci_host *host, u8 mask)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *priv = sdhci_pltfm_priv(pltfm_host);
    u32 val, emmc_caps = MMC_CAP2_NO_SD | MMC_CAP2_NO_SDIO;
    dwcmshc_reset(host, mask);
    if ((host.mmc.caps2 & emmc_caps) == emmc_caps) {
    val = sdhci_readl(host, priv.vendor_specific_area1 + CV18XX_SDHCI_MSHC_CTRL);
    val |= CV18XX_EMMC_FUNC_EN;
    sdhci_writel(host, val, priv.vendor_specific_area1 + CV18XX_SDHCI_MSHC_CTRL);
    }
    val = sdhci_readl(host, priv.vendor_specific_area1 + CV18XX_SDHCI_MSHC_CTRL);
    val |= CV18XX_LATANCY_1T;
    sdhci_writel(host, val, priv.vendor_specific_area1 + CV18XX_SDHCI_MSHC_CTRL);
    val = sdhci_readl(host, priv.vendor_specific_area1 + CV18XX_SDHCI_PHY_CONFIG);
    val |= CV18XX_PHY_TX_BPS;
    sdhci_writel(host, val, priv.vendor_specific_area1 + CV18XX_SDHCI_PHY_CONFIG);
    val =  (FIELD_PREP(CV18XX_PHY_TX_DLY_MSK, 0) |
    FIELD_PREP(CV18XX_PHY_TX_SRC_MSK, CV18XX_PHY_TX_SRC_INVERT_CLK_TX) |
    FIELD_PREP(CV18XX_PHY_RX_DLY_MSK, 0) |
    FIELD_PREP(CV18XX_PHY_RX_SRC_MSK, CV18XX_PHY_RX_SRC_INVERT_RX_CLK));
    sdhci_writel(host, val, priv.vendor_specific_area1 + CV18XX_SDHCI_PHY_TX_RX_DLY);
    }
#[no_mangle]
unsafe extern "C" fn cv18xx_sdhci_set_tap(host: *mut sdhci_host, tap: c_int) {
    static void cv18xx_sdhci_set_tap(struct sdhci_host *host, int tap)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *priv = sdhci_pltfm_priv(pltfm_host);
    u16 clk;
    u32 val;
    clk = sdhci_readw(host, SDHCI_CLOCK_CONTROL);
    clk &= ~SDHCI_CLOCK_CARD_EN;
    sdhci_writew(host, clk, SDHCI_CLOCK_CONTROL);
    val = sdhci_readl(host, priv.vendor_specific_area1 + CV18XX_SDHCI_MSHC_CTRL);
    val &= ~CV18XX_LATANCY_1T;
    sdhci_writel(host, val, priv.vendor_specific_area1 + CV18XX_SDHCI_MSHC_CTRL);
    val =  (FIELD_PREP(CV18XX_PHY_TX_DLY_MSK, 0) |
    FIELD_PREP(CV18XX_PHY_TX_SRC_MSK, CV18XX_PHY_TX_SRC_INVERT_CLK_TX) |
    FIELD_PREP(CV18XX_PHY_RX_DLY_MSK, tap));
    sdhci_writel(host, val, priv.vendor_specific_area1 + CV18XX_SDHCI_PHY_TX_RX_DLY);
    sdhci_writel(host, 0, priv.vendor_specific_area1 + CV18XX_SDHCI_PHY_CONFIG);
    clk |= SDHCI_CLOCK_CARD_EN;
    sdhci_writew(host, clk, SDHCI_CLOCK_CONTROL);
    usleep_range(1000, 2000);
    }
#[no_mangle]
unsafe extern "C" fn cv18xx_retry_tuning(mmc: *mut mmc_host, opcode: u32, cmd_error: *mut c_int) -> c_int {
    static int cv18xx_retry_tuning(struct mmc_host *mmc, u32 opcode, int *cmd_error)
    {
    int ret, retry = 0;
    while (retry < CV18XX_RETRY_TUNING_MAX) {
    ret = mmc_send_tuning(mmc, opcode, core::ptr::null_mut());
    if (ret)
    return ret;
    retry++;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cv18xx_sdhci_post_tuning(host: *mut sdhci_host) {
    static void cv18xx_sdhci_post_tuning(struct sdhci_host *host)
    {
    u32 val;
    val = sdhci_readl(host, SDHCI_INT_STATUS);
    val |= SDHCI_INT_DATA_AVAIL;
    sdhci_writel(host, val, SDHCI_INT_STATUS);
    dwcmshc_reset(host, SDHCI_RESET_CMD | SDHCI_RESET_DATA);
    }
#[no_mangle]
unsafe extern "C" fn cv18xx_sdhci_execute_tuning(host: *mut sdhci_host, opcode: u32) -> c_int {
    static int cv18xx_sdhci_execute_tuning(struct sdhci_host *host, u32 opcode)
    {
    int min, max, avg, ret;
    int win_length, target_min, target_max, target_win_length;
    min = max = 0;
    target_win_length = 0;
    sdhci_reset_tuning(host);
    while (max < CV18XX_TUNE_MAX) {
// find the mininum delay first which can pass tuning
    while (min < CV18XX_TUNE_MAX) {
    cv18xx_sdhci_set_tap(host, min);
    if (!cv18xx_retry_tuning(host.mmc, opcode, core::ptr::null_mut()))
    break;
    min += CV18XX_TUNE_STEP;
    }
// find the maxinum delay which can not pass tuning
    max = min + CV18XX_TUNE_STEP;
    while (max < CV18XX_TUNE_MAX) {
    cv18xx_sdhci_set_tap(host, max);
    if (cv18xx_retry_tuning(host.mmc, opcode, core::ptr::null_mut())) {
    max -= CV18XX_TUNE_STEP;
    break;
    }
    max += CV18XX_TUNE_STEP;
    }
    win_length = max - min + 1;
// get the largest pass window
    if (win_length > target_win_length) {
    target_win_length = win_length;
    target_min = min;
    target_max = max;
    }
// continue to find the next pass window
    min = max + CV18XX_TUNE_STEP;
    }
    cv18xx_sdhci_post_tuning(host);
// use average delay to get the best timing
    avg = (target_min + target_max) / 2;
    cv18xx_sdhci_set_tap(host, avg);
    ret = mmc_send_tuning(host.mmc, opcode, core::ptr::null_mut());
    dev_dbg(mmc_dev(host.mmc), "tuning %s at 0x%x ret %d\n",
    ret ? "failed" : "passed", avg, ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sg2042_sdhci_phy_init(host: *mut sdhci_host) {
    static inline void sg2042_sdhci_phy_init(struct sdhci_host *host)
    {
    u32 val;
// Asset phy reset & set tx drive strength
    val = sdhci_readl(host, PHY_CNFG_R);
    val &= ~PHY_CNFG_RSTN_DEASSERT;
    val |= FIELD_PREP(PHY_CNFG_PHY_PWRGOOD_MASK, 1);
    val |= FIELD_PREP(PHY_CNFG_PAD_SP_MASK, PHY_CNFG_PAD_SP_SG2042);
    val |= FIELD_PREP(PHY_CNFG_PAD_SN_MASK, PHY_CNFG_PAD_SN_SG2042);
    sdhci_writel(host, val, PHY_CNFG_R);
// Configure phy pads
    val = PHY_PAD_RXSEL_3V3;
    val |= FIELD_PREP(PHY_PAD_WEAKPULL_MASK, PHY_PAD_WEAKPULL_PULLUP);
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_P_MASK, PHY_PAD_TXSLEW_CTRL_P);
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_N_MASK, PHY_PAD_TXSLEW_CTRL_N_SG2042);
    sdhci_writew(host, val, PHY_CMDPAD_CNFG_R);
    sdhci_writew(host, val, PHY_DATAPAD_CNFG_R);
    sdhci_writew(host, val, PHY_RSTNPAD_CNFG_R);
    val = PHY_PAD_RXSEL_3V3;
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_P_MASK, PHY_PAD_TXSLEW_CTRL_P);
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_N_MASK, PHY_PAD_TXSLEW_CTRL_N_SG2042);
    sdhci_writew(host, val, PHY_CLKPAD_CNFG_R);
    val = PHY_PAD_RXSEL_3V3;
    val |= FIELD_PREP(PHY_PAD_WEAKPULL_MASK, PHY_PAD_WEAKPULL_PULLDOWN);
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_P_MASK, PHY_PAD_TXSLEW_CTRL_P);
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_N_MASK, PHY_PAD_TXSLEW_CTRL_N_SG2042);
    sdhci_writew(host, val, PHY_STBPAD_CNFG_R);
// Configure delay line
// Enable fixed delay
    sdhci_writeb(host, PHY_SDCLKDL_CNFG_EXTDLY_EN, PHY_SDCLKDL_CNFG_R);
//
// Set delay line.
// Its recommended that bit UPDATE_DC[4] is 1 when SDCLKDL_DC is being written.
// Ensure UPDATE_DC[4] is '0' when not updating code.
//
    val = sdhci_readb(host, PHY_SDCLKDL_CNFG_R);
    val |= PHY_SDCLKDL_CNFG_UPDATE;
    sdhci_writeb(host, val, PHY_SDCLKDL_CNFG_R);
// Add 10 * 70ps = 0.7ns for output delay
    sdhci_writeb(host, 10, PHY_SDCLKDL_DC_R);
    val = sdhci_readb(host, PHY_SDCLKDL_CNFG_R);
    val &= ~(PHY_SDCLKDL_CNFG_UPDATE);
    sdhci_writeb(host, val, PHY_SDCLKDL_CNFG_R);
// Set SMPLDL_CNFG, Bypass
    sdhci_writeb(host, PHY_SMPLDL_CNFG_BYPASS_EN, PHY_SMPLDL_CNFG_R);
// Set ATDL_CNFG, tuning clk not use for init
    val = FIELD_PREP(PHY_ATDL_CNFG_INPSEL_MASK, PHY_ATDL_CNFG_INPSEL_SG2042);
    sdhci_writeb(host, val, PHY_ATDL_CNFG_R);
// Deasset phy reset
    val = sdhci_readl(host, PHY_CNFG_R);
    val |= PHY_CNFG_RSTN_DEASSERT;
    sdhci_writel(host, val, PHY_CNFG_R);
    }
#[no_mangle]
unsafe extern "C" fn sg2042_sdhci_reset(host: *mut sdhci_host, mask: u8) {
    static void sg2042_sdhci_reset(struct sdhci_host *host, u8 mask)
    {
    sdhci_reset(host, mask);
    if (mask & SDHCI_RESET_ALL)
    sg2042_sdhci_phy_init(host);
    }
    static int sg2042_init(struct device *dev, struct sdhci_host *host,
    struct dwcmshc_priv *dwc_priv)
    {
    static const char * const clk_ids[] = {"timer"};
    return dwcmshc_get_enable_other_clks(mmc_dev(host.mmc), dwc_priv,
    ARRAY_SIZE(clk_ids), clk_ids);
    }
//
// HPE GSC-specific vendor configuration: disable command conflict check
// and program Auto-Tuning Control register.
//
#[no_mangle]
unsafe extern "C" fn dwcmshc_hpe_vendor_specific(host: *mut sdhci_host) {
    static void dwcmshc_hpe_vendor_specific(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *dwc_priv = sdhci_pltfm_priv(pltfm_host);
    u32 atctrl;
    u8 extra;
    extra = sdhci_readb(host, dwc_priv.vendor_specific_area1 + DWCMSHC_HOST_CTRL3);
    extra &= ~DWCMSHC_HOST_CTRL3_CMD_CONFLICT;
    sdhci_writeb(host, extra, dwc_priv.vendor_specific_area1 + DWCMSHC_HOST_CTRL3);
    atctrl = AT_CTRL_AT_EN | AT_CTRL_SWIN_TH_EN | AT_CTRL_TUNE_CLK_STOP_EN |
    FIELD_PREP(AT_CTRL_PRE_CHANGE_DLY_MASK, 3) |
    FIELD_PREP(AT_CTRL_POST_CHANGE_DLY_MASK, AT_CTRL_POST_CHANGE_DLY) |
    FIELD_PREP(AT_CTRL_SWIN_TH_VAL_MASK, 2);
    sdhci_writel(host, atctrl, dwc_priv.vendor_specific_area1 + DWCMSHC_EMMC_ATCTRL);
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_hpe_set_emmc(host: *mut sdhci_host) {
    static void dwcmshc_hpe_set_emmc(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *dwc_priv = sdhci_pltfm_priv(pltfm_host);
    u16 ctrl;
    ctrl = sdhci_readw(host, dwc_priv.vendor_specific_area1 + DWCMSHC_EMMC_CONTROL);
    ctrl |= DWCMSHC_CARD_IS_EMMC;
    sdhci_writew(host, ctrl, dwc_priv.vendor_specific_area1 + DWCMSHC_EMMC_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_hpe_reset(host: *mut sdhci_host, mask: u8) {
    static void dwcmshc_hpe_reset(struct sdhci_host *host, u8 mask)
    {
    dwcmshc_reset(host, mask);
    dwcmshc_hpe_vendor_specific(host);
    dwcmshc_hpe_set_emmc(host);
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_hpe_set_uhs_signaling(host: *mut sdhci_host, timing: c_uint) {
    static void dwcmshc_hpe_set_uhs_signaling(struct sdhci_host *host, unsigned int timing)
    {
    dwcmshc_set_uhs_signaling(host, timing);
    dwcmshc_hpe_set_emmc(host);
    }
//
// HPE GSC eMMC controller clock setup.
//
// The GSC SoC wires the freq_sel field of SDHCI_CLOCK_CONTROL directly to a
// clock mux rather than a divider. Force freq_sel = 1 when running at
// 200 MHz (HS200) so the mux selects the correct clock source.
//
#[no_mangle]
unsafe extern "C" fn dwcmshc_hpe_set_clock(host: *mut sdhci_host, clock: c_uint) {
    static void dwcmshc_hpe_set_clock(struct sdhci_host *host, unsigned int clock)
    {
    u16 clk;
    host.mmc.actual_clock = 0;
    sdhci_writew(host, 0, SDHCI_CLOCK_CONTROL);
    if (clock == 0)
    return;
    clk = sdhci_calc_clk(host, clock, &host.mmc.actual_clock);
    if (host.mmc.actual_clock == 200000000)
    clk |= (1 << SDHCI_DIVIDER_SHIFT);
    sdhci_enable_clk(host, clk);
    }
//
// HPE GSC eMMC controller init.
//
// The GSC SoC requires configuring MSHCCS.  Bit 18 (SCGSyncDis) disables clock
// synchronisation for phase-select values going to the HS200 RX delay lines,
// allowing the card clock to be stopped while the delay selection settles and
// the phase shift is applied.  This must be used together with the ATCTRL
// settings programmed in dwcmshc_hpe_vendor_specific():
// AT_CTRL_R.TUNE_CLK_STOP_EN  = 0x1
// AT_CTRL_R.POST_CHANGE_DLY   = 0x3
// AT_CTRL_R.PRE_CHANGE_DLY    = 0x3
//
// The DTS node provides a syscon phandle ('hpe,gxp-sysreg') with the
// MSHCCS register offset as an argument.
//
    static int dwcmshc_hpe_gsc_init(struct device *dev, struct sdhci_host *host,
    struct dwcmshc_priv *dwc_priv)
    {
    unsigned int reg_offset;
    struct regmap *soc_ctrl;
    int ret;
// Disable cmd conflict check and configure auto-tuning
    dwcmshc_hpe_vendor_specific(host);
// Look up the GXP sysreg syscon and MSHCCS offset
    soc_ctrl = syscon_regmap_lookup_by_phandle_args(dev.of_node,
    "hpe,gxp-sysreg",
    1, &reg_offset);
    if (IS_ERR(soc_ctrl)) {
    dev_err(dev, "failed to get hpe,gxp-sysreg syscon\n");
    return PTR_ERR(soc_ctrl);
    }
// Set SCGSyncDis (bit 18) to disable sync on HS200 RX delay lines
    ret = regmap_update_bits(soc_ctrl, reg_offset,
    HPE_GSC_MSHCCS_SCGSYNCDIS,
    HPE_GSC_MSHCCS_SCGSYNCDIS);
    if (ret) {
    dev_err(dev, "failed to set SCGSyncDis in MSHCCS\n");
    return ret;
    }
    sdhci_enable_v4_mode(host);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_eic7700_set_clock(host: *mut sdhci_host, clock: c_uint) {
    static void sdhci_eic7700_set_clock(struct sdhci_host *host, unsigned int clock)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    u16 clk;
    host.mmc.actual_clock = clock;
    if (clock == 0) {
    sdhci_set_clock(host, clock);
    return;
    }
    clk_set_rate(pltfm_host.clk, clock);
    clk = sdhci_readw(host, SDHCI_CLOCK_CONTROL);
    sdhci_enable_clk(host, clk);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_eic7700_config_phy_delay(host: *mut sdhci_host, delay: c_int) {
    static void sdhci_eic7700_config_phy_delay(struct sdhci_host *host, int delay)
    {
    delay &= PHY_CLK_MAX_DELAY_MASK;
// phy clk delay line config
    sdhci_writeb(host, PHY_SDCLKDL_CNFG_UPDATE, PHY_SDCLKDL_CNFG_R);
    sdhci_writeb(host, delay, PHY_SDCLKDL_DC_R);
    sdhci_writeb(host, 0x0, PHY_SDCLKDL_CNFG_R);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_eic7700_config_phy(host: *mut sdhci_host) {
    static void sdhci_eic7700_config_phy(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *dwc_priv = sdhci_pltfm_priv(pltfm_host);
    let mut emmc_caps: u32 = MMC_CAP2_NO_SD | MMC_CAP2_NO_SDIO;
    struct eic7700_priv *priv = dwc_priv.priv;
    unsigned int val, drv;
    drv = FIELD_PREP(PHY_CNFG_PAD_SP_MASK, priv.drive_impedance & 0xF);
    drv |= FIELD_PREP(PHY_CNFG_PAD_SN_MASK, (priv.drive_impedance >> 4) & 0xF);
    if ((host.mmc.caps2 & emmc_caps) == emmc_caps) {
    val = sdhci_readw(host, dwc_priv.vendor_specific_area1 + DWCMSHC_EMMC_CONTROL);
    val |= DWCMSHC_CARD_IS_EMMC;
    sdhci_writew(host, val, dwc_priv.vendor_specific_area1 + DWCMSHC_EMMC_CONTROL);
    }
// reset phy, config phy's pad
    sdhci_writel(host, drv | ~PHY_CNFG_RSTN_DEASSERT, PHY_CNFG_R);
// configure phy pads
    val = FIELD_PREP(PHY_PAD_TXSLEW_CTRL_P_MASK, PHY_PAD_TXSLEW_CTRL_N_SG2042);
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_N_MASK, PHY_PAD_TXSLEW_CTRL_N_SG2042);
    val |= FIELD_PREP(PHY_PAD_WEAKPULL_MASK, PHY_PAD_WEAKPULL_PULLUP);
    val |= PHY_PAD_RXSEL_1V8;
    sdhci_writew(host, val, PHY_CMDPAD_CNFG_R);
    sdhci_writew(host, val, PHY_DATAPAD_CNFG_R);
    sdhci_writew(host, val, PHY_RSTNPAD_CNFG_R);
// Clock PAD Setting
    val = FIELD_PREP(PHY_PAD_TXSLEW_CTRL_P_MASK, PHY_PAD_TXSLEW_CTRL_N_SG2042);
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_N_MASK, PHY_PAD_TXSLEW_CTRL_N_SG2042);
    sdhci_writew(host, val, PHY_CLKPAD_CNFG_R);
// PHY strobe PAD setting (EMMC only)
    if ((host.mmc.caps2 & emmc_caps) == emmc_caps) {
    val = FIELD_PREP(PHY_PAD_TXSLEW_CTRL_P_MASK, PHY_PAD_TXSLEW_CTRL_N_SG2042);
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_N_MASK, PHY_PAD_TXSLEW_CTRL_N_SG2042);
    val |= PHY_PAD_RXSEL_1V8;
    sdhci_writew(host, val, PHY_STBPAD_CNFG_R);
    }
    usleep_range(2000, 3000);
    sdhci_writel(host, drv | PHY_CNFG_RSTN_DEASSERT, PHY_CNFG_R);
    sdhci_eic7700_config_phy_delay(host, dwc_priv.delay_line);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_eic7700_reset(host: *mut sdhci_host, mask: u8) {
    static void sdhci_eic7700_reset(struct sdhci_host *host, u8 mask)
    {
    dwcmshc_reset(host, mask);
// after reset all, the phy's config will be clear
    if (mask == SDHCI_RESET_ALL)
    sdhci_eic7700_config_phy(host);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_eic7700_reset_init(dev: *mut device, priv: *mut eic7700_priv) -> c_int {
    static int sdhci_eic7700_reset_init(struct device *dev, struct eic7700_priv *priv)
    {
    int ret;
    priv.reset = devm_reset_control_array_get_optional_exclusive(dev);
    if (IS_ERR(priv.reset)) {
    ret = PTR_ERR(priv.reset);
    dev_err(dev, "failed to get reset control %d\n", ret);
    return ret;
    }
    ret = reset_control_assert(priv.reset);
    if (ret) {
    dev_err(dev, "Failed to assert reset signals: %d\n", ret);
    return ret;
    }
    usleep_range(2000, 2100);
    ret = reset_control_deassert(priv.reset);
    if (ret) {
    dev_err(dev, "Failed to deassert reset signals: %d\n", ret);
    return ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn eic7700_convert_drive_impedance_ohm(dev: *mut device, dr_ohm: c_uint) -> c_uint {
    static unsigned int eic7700_convert_drive_impedance_ohm(struct device *dev, unsigned int dr_ohm)
    {
    switch (dr_ohm) {
    case 100:
    return PHYCTRL_DR_100OHM;
    case 66:
    return PHYCTRL_DR_66OHM;
    case 50:
    return PHYCTRL_DR_50OHM;
    case 40:
    return PHYCTRL_DR_40OHM;
    case 33:
    return PHYCTRL_DR_33OHM;
    }
    dev_warn(dev, "Invalid value %u for drive-impedance-ohms.\n", dr_ohm);
    return PHYCTRL_DR_50OHM;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_eic7700_delay_tuning(host: *mut sdhci_host, opcode: u32) -> c_int {
    static int sdhci_eic7700_delay_tuning(struct sdhci_host *host, u32 opcode)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *dwc_priv = sdhci_pltfm_priv(pltfm_host);
    let mut delay_min: c_int = -1;
    let mut delay_max: c_int = -1;
    let mut cmd_error: c_int = 0;
    let mut delay: c_int = 0;
    let mut i: c_int = 0;
    int ret;
    for (i = 0; i <= PHY_DELAY_CODE_MAX; i++) {
    sdhci_eic7700_config_phy_delay(host, i);
    ret = mmc_send_tuning(host.mmc, opcode, &cmd_error);
    if (ret) {
    host.ops.reset(host, SDHCI_RESET_CMD | SDHCI_RESET_DATA);
    usleep_range(200, 210);
    if (delay_min != -1 && delay_max != -1)
    break;
    } else {
    if (delay_min == -1) {
    delay_min = i;
    continue;
    } else {
    delay_max = i;
    continue;
    }
    }
    }
    if (delay_min == -1 && delay_max == -1) {
    pr_err("%s: delay code tuning failed!\n", mmc_hostname(host.mmc));
    sdhci_eic7700_config_phy_delay(host, dwc_priv.delay_line);
    return ret;
    }
    delay = (delay_min + delay_max) / 2;
    sdhci_eic7700_config_phy_delay(host, delay);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_eic7700_phase_code_tuning(host: *mut sdhci_host, opcode: u32) -> c_int {
    static int sdhci_eic7700_phase_code_tuning(struct sdhci_host *host, u32 opcode)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *priv = sdhci_pltfm_priv(pltfm_host);
    let mut emmc_caps: u32 = MMC_CAP2_NO_SD | MMC_CAP2_NO_SDIO;
    let mut phase_code: c_int = -1;
    let mut code_range: c_int = -1;
    let mut code_min: c_int = -1;
    let mut code_max: c_int = -1;
    let mut cmd_error: c_int = 0;
    bool is_emmc;
    let mut ret: c_int = 0;
    let mut i: c_int = 0;
    is_emmc = (host.mmc.caps2 & emmc_caps) == emmc_caps;
    for (i = 0; i <= MAX_PHASE_CODE; i++) {
// Centered Phase code
    sdhci_writew(host, i, priv.vendor_specific_area1 + DWCMSHC_AT_STAT);
    ret = mmc_send_tuning(host.mmc, opcode, &cmd_error);
    host.ops.reset(host, SDHCI_RESET_CMD | SDHCI_RESET_DATA);
    if (ret) {
// SD/SDIO specific range tracking
    if (!is_emmc && code_min != -1 && code_max != -1) {
    if (code_max - code_min > code_range) {
    code_range = code_max - code_min;
    phase_code = (code_min + code_max) / 2;
    if (code_range > TUNING_RANGE_THRESHOLD)
    break;
    }
    code_min = -1;
    code_max = -1;
    }
// EMMC breaks after first valid range
    if (is_emmc && code_min != -1 && code_max != -1)
    break;
    } else {
// Track valid phase code range
    if (code_min == -1) {
    code_min = i;
    if (is_emmc)
    continue;
    }
    code_max = i;
    if (!is_emmc && i == MAX_PHASE_CODE) {
    if (code_max - code_min > code_range) {
    code_range = code_max - code_min;
    phase_code = (code_min + code_max) / 2;
    }
    }
    }
    }
// Handle tuning failure case
    if ((!is_emmc && phase_code == -1) ||
    (is_emmc && code_min == -1 && code_max == -1)) {
    pr_err("%s: phase code tuning failed!\n", mmc_hostname(host.mmc));
    sdhci_writew(host, 0, priv.vendor_specific_area1 + DWCMSHC_AT_STAT);
    return -EIO;
    }
    if (is_emmc)
    phase_code = (code_min + code_max) / 2;
    sdhci_writew(host, phase_code, priv.vendor_specific_area1 + DWCMSHC_AT_STAT);
// SD/SDIO specific final verification
    if (!is_emmc) {
    ret = mmc_send_tuning(host.mmc, opcode, &cmd_error);
    host.ops.reset(host, SDHCI_RESET_CMD | SDHCI_RESET_DATA);
    if (ret) {
    pr_err("%s: Final phase code 0x%x verification failed!\n",
    mmc_hostname(host.mmc), phase_code);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_eic7700_executing_tuning(host: *mut sdhci_host, opcode: u32) -> c_int {
    static int sdhci_eic7700_executing_tuning(struct sdhci_host *host, u32 opcode)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *priv = sdhci_pltfm_priv(pltfm_host);
    let mut emmc_caps: u32 = MMC_CAP2_NO_SD | MMC_CAP2_NO_SDIO;
    let mut ret: c_int = 0;
    u16 ctrl;
    u32 val;
    ctrl = sdhci_readw(host, SDHCI_HOST_CONTROL2);
    ctrl &= ~SDHCI_CTRL_TUNED_CLK;
    sdhci_writew(host, ctrl, SDHCI_HOST_CONTROL2);
    val = sdhci_readl(host, priv.vendor_specific_area1 + DWCMSHC_EMMC_ATCTRL);
    val |= AT_CTRL_SW_TUNE_EN;
    sdhci_writew(host, val, priv.vendor_specific_area1 + DWCMSHC_EMMC_ATCTRL);
    sdhci_writew(host, 0, priv.vendor_specific_area1 + DWCMSHC_AT_STAT);
    sdhci_writew(host, 0x0, SDHCI_CMD_DATA);
    if ((host.mmc.caps2 & emmc_caps) == emmc_caps) {
    ret = sdhci_eic7700_delay_tuning(host, opcode);
    if (ret)
    return ret;
    }
    ret = sdhci_eic7700_phase_code_tuning(host, opcode);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_eic7700_set_uhs_signaling(host: *mut sdhci_host, timing: c_uint) {
    static void sdhci_eic7700_set_uhs_signaling(struct sdhci_host *host, unsigned int timing)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *priv = sdhci_pltfm_priv(pltfm_host);
    u8 status;
    u32 val;
    int ret;
    dwcmshc_set_uhs_signaling(host, timing);
// here need make dll locked when in hs400 at 200MHz
    if (timing == MMC_TIMING_MMC_HS400 && host.clock == 200000000) {
    val = sdhci_readl(host, priv.vendor_specific_area1 + DWCMSHC_EMMC_ATCTRL);
    val &= ~(FIELD_PREP(AT_CTRL_POST_CHANGE_DLY_MASK, AT_CTRL_POST_CHANGE_DLY));
// 2-cycle latency
    val |= FIELD_PREP(AT_CTRL_POST_CHANGE_DLY_MASK, 0x2);
    sdhci_writew(host, val, priv.vendor_specific_area1 + DWCMSHC_EMMC_ATCTRL);
    sdhci_writeb(host, FIELD_PREP(PHY_DLL_CNFG1_SLVDLY_MASK, PHY_DLL_CNFG1_SLVDLY) |
    0x3, PHY_DLL_CNFG1_R);/* DLL wait cycle input */
// DLL jump step input
    sdhci_writeb(host, 0x02, PHY_DLL_CNFG2_R);
    sdhci_writeb(host, FIELD_PREP(PHY_DLLDL_CNFG_SLV_INPSEL_MASK,
    PHY_DLLDL_CNFG_SLV_INPSEL), PHY_DLLDL_CNFG_R);
// Sets the value of DLL's offset input
    sdhci_writeb(host, 0x00, PHY_DLL_OFFST_R);
//
// Sets the value of DLL's olbt loadval input. Controls the Ibt
// timer's timeout value at which DLL runs a revalidation cycle.
//
    sdhci_writew(host, 0xffff, PHY_DLLBT_CNFG_R);
    sdhci_writeb(host, PHY_DLL_CTRL_ENABLE, PHY_DLL_CTRL_R);
    usleep_range(100, 110);
    ret = read_poll_timeout(sdhci_readb, status, status & DLL_LOCK_STS, 100, 1000000,
    false, host, PHY_DLL_STATUS_R);
    if (ret) {
    pr_err("%s: DLL lock timeout! status: 0x%x\n",
    mmc_hostname(host.mmc), status);
    return;
    }
    status = sdhci_readb(host, PHY_DLL_STATUS_R);
    if (status & DLL_ERROR_STS) {
    pr_err("%s: DLL lock failed!err_status:0x%x\n",
    mmc_hostname(host.mmc), status);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn sdhci_eic7700_set_uhs_wrapper(host: *mut sdhci_host, timing: c_uint) {
    static void sdhci_eic7700_set_uhs_wrapper(struct sdhci_host *host, unsigned int timing)
    {
    let mut emmc_caps: u32 = MMC_CAP2_NO_SD | MMC_CAP2_NO_SDIO;
    if ((host.mmc.caps2 & emmc_caps) != emmc_caps)
    sdhci_set_uhs_signaling(host, timing);
    else
    sdhci_eic7700_set_uhs_signaling(host, timing);
    }
#[no_mangle]
unsafe extern "C" fn eic7700_init(dev: *mut device, host: *mut sdhci_host, dwc_priv: *mut dwcmshc_priv) -> c_int {
    static int eic7700_init(struct device *dev, struct sdhci_host *host, struct dwcmshc_priv *dwc_priv)
    {
    let mut emmc_caps: u32 = MMC_CAP2_NO_SD | MMC_CAP2_NO_SDIO;
    let mut sd_caps: u32 = MMC_CAP2_NO_MMC | MMC_CAP2_NO_SDIO;
    unsigned int val, hsp_int_status, hsp_pwr_ctrl;
    static const char * const clk_ids[] = {"axi"};
    struct of_phandle_args args;
    struct eic7700_priv *priv;
    struct regmap *hsp_regmap;
    int ret;
    priv = devm_kzalloc(dev, sizeof(struct eic7700_priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    dwc_priv.priv = priv;
    ret = sdhci_eic7700_reset_init(dev, dwc_priv.priv);
    if (ret)
    return dev_err_probe(dev, ret, "failed to reset\n");
    ret = dwcmshc_get_enable_other_clks(mmc_dev(host.mmc), dwc_priv,
    ARRAY_SIZE(clk_ids), clk_ids);
    if (ret)
    return ret;
    ret = of_parse_phandle_with_fixed_args(dev.of_node, "eswin,hsp-sp-csr", 2, 0, &args);
    if (ret)
    return dev_err_probe(dev, ret, "Fail to parse 'eswin,hsp-sp-csr' phandle\n");
    hsp_regmap = syscon_node_to_regmap(args.np);
    if (IS_ERR(hsp_regmap)) {
    of_node_put(args.np);
    return dev_err_probe(dev, PTR_ERR(hsp_regmap),
    "Failed to get regmap for 'eswin,hsp-sp-csr'\n");
    }
    hsp_int_status = args.args[0];
    hsp_pwr_ctrl = args.args[1];
    of_node_put(args.np);
//
// Assert clock stability: write EIC7700_INT_CLK_STABLE to hsp_int_status.
// This signals to the eMMC controller that platform clocks (card, ACLK,
// BCLK, TMCLK) are enabled and stable.
//
    regmap_write(hsp_regmap, hsp_int_status, EIC7700_INT_CLK_STABLE);
//
// Assert voltage stability: write EIC7700_HOST_VAL_STABLE to hsp_pwr_ctrl.
// This signals that VDD is stable and permits transition to high-speed
// modes (e.g., UHS-I).
//
    regmap_write(hsp_regmap, hsp_pwr_ctrl, EIC7700_HOST_VAL_STABLE);
    if ((host.mmc.caps2 & emmc_caps) == emmc_caps)
    dwc_priv.delay_line = PHY_DELAY_CODE_EMMC;
#[no_mangle]
pub unsafe extern "C" fn if(sd_caps: (host->mmc->caps2 & sd_caps) ==) -> else {
    else if ((host.mmc.caps2 & sd_caps) == sd_caps)
    dwc_priv.delay_line = PHY_DELAY_CODE_SD;
    else
    dwc_priv.delay_line = PHY_DELAY_CODE_SDIO;
    if (!of_property_read_u32(dev.of_node, "eswin,drive-impedance-ohms", &val))
    priv.drive_impedance = eic7700_convert_drive_impedance_ohm(dev, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_k230_sdhci_set_clock(host: *mut sdhci_host, clock: c_uint) {
    static void dwcmshc_k230_sdhci_set_clock(struct sdhci_host *host, unsigned int clock)
    {
    u16 clk;
    sdhci_set_clock(host, clock);
    clk = sdhci_readw(host, SDHCI_CLOCK_CONTROL);
//
// It is necessary to enable SDHCI_PROG_CLOCK_MODE. This is a
// vendor-specific quirk. If this is not done, the eMMC will be
// unable to read or write.
//
    clk |= SDHCI_PROG_CLOCK_MODE;
    sdhci_writew(host, clk, SDHCI_CLOCK_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_k230_config_phy_delay(host: *mut sdhci_host) {
    static void sdhci_k230_config_phy_delay(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *dwc_priv = sdhci_pltfm_priv(pltfm_host);
    u32 val;
    sdhci_writeb(host, PHY_COMMDL_CNFG_DLSTEP_SEL, PHY_COMMDL_CNFG);
    sdhci_writeb(host, 0x0, PHY_SDCLKDL_CNFG_R);
    sdhci_writeb(host, PHY_SDCLKDL_DC_INITIAL, PHY_SDCLKDL_DC_R);
    val = PHY_SMPLDL_CNFG_EXTDLY_EN;
    val |= FIELD_PREP(PHY_SMPLDL_CNFG_INPSEL_MASK, PHY_SMPLDL_CNFG_INPSEL);
    sdhci_writeb(host, val, PHY_SMPLDL_CNFG_R);
    sdhci_writeb(host, FIELD_PREP(PHY_ATDL_CNFG_INPSEL_MASK, PHY_ATDL_CNFG_INPSEL),
    PHY_ATDL_CNFG_R);
    val = sdhci_readl(host, dwc_priv.vendor_specific_area1 + DWCMSHC_EMMC_ATCTRL);
    val |= AT_CTRL_TUNE_CLK_STOP_EN;
    val |= FIELD_PREP(AT_CTRL_PRE_CHANGE_DLY_MASK, AT_CTRL_PRE_CHANGE_DLY);
    val |= FIELD_PREP(AT_CTRL_POST_CHANGE_DLY_MASK, AT_CTRL_POST_CHANGE_DLY);
    sdhci_writel(host, val, dwc_priv.vendor_specific_area1 + DWCMSHC_EMMC_ATCTRL);
    sdhci_writel(host, 0x0, dwc_priv.vendor_specific_area1 + DWCMSHC_AT_STAT);
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_k230_phy_init(host: *mut sdhci_host) -> c_int {
    static int dwcmshc_k230_phy_init(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *dwc_priv = sdhci_pltfm_priv(pltfm_host);
    u32 rxsel;
    u32 val;
    u32 reg;
    int ret;
// reset phy
    sdhci_writew(host, 0, PHY_CNFG_R);
// Disable the clock
    sdhci_writew(host, 0, SDHCI_CLOCK_CONTROL);
    rxsel = dwc_priv.flags & FLAG_IO_FIXED_1V8 ? PHY_PAD_RXSEL_1V8 : PHY_PAD_RXSEL_3V3;
    val = rxsel;
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_P_MASK, PHY_PAD_TXSLEW_CTRL_P_k230);
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_N_MASK, PHY_PAD_TXSLEW_CTRL_N_k230);
    val |= FIELD_PREP(PHY_PAD_WEAKPULL_MASK, PHY_PAD_WEAKPULL_PULLUP);
    sdhci_writew(host, val, PHY_CMDPAD_CNFG_R);
    sdhci_writew(host, val, PHY_DATAPAD_CNFG_R);
    sdhci_writew(host, val, PHY_RSTNPAD_CNFG_R);
    val = rxsel;
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_P_MASK, PHY_PAD_TXSLEW_CTRL_P_k230);
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_N_MASK, PHY_PAD_TXSLEW_CTRL_N_k230);
    sdhci_writew(host, val, PHY_CLKPAD_CNFG_R);
    val = rxsel;
    val |= FIELD_PREP(PHY_PAD_WEAKPULL_MASK, PHY_PAD_WEAKPULL_PULLDOWN);
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_P_MASK, PHY_PAD_TXSLEW_CTRL_P_k230);
    val |= FIELD_PREP(PHY_PAD_TXSLEW_CTRL_N_MASK, PHY_PAD_TXSLEW_CTRL_N_k230);
    sdhci_writew(host, val, PHY_STBPAD_CNFG_R);
    sdhci_k230_config_phy_delay(host);
// Wait max 150 ms
    ret = read_poll_timeout(sdhci_readl, reg,
    (reg & FIELD_PREP(PHY_CNFG_PHY_PWRGOOD_MASK, 1)),
    10, 150000, false, host, PHY_CNFG_R);
    if (ret) {
    dev_err(mmc_dev(host.mmc), "READ PHY PWRGOOD timeout!\n");
    return -ETIMEDOUT;
    }
    reg = FIELD_PREP(PHY_CNFG_PAD_SN_MASK, PHY_CNFG_PAD_SN_k230) |
    FIELD_PREP(PHY_CNFG_PAD_SP_MASK, PHY_CNFG_PAD_SP_k230);
    sdhci_writel(host, reg, PHY_CNFG_R);
// de-assert the phy
    reg |= PHY_CNFG_RSTN_DEASSERT;
    sdhci_writel(host, reg, PHY_CNFG_R);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_k230_sdhci_reset(host: *mut sdhci_host, mask: u8) {
    static void dwcmshc_k230_sdhci_reset(struct sdhci_host *host, u8 mask)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *dwc_priv = sdhci_pltfm_priv(pltfm_host);
    const struct k230_pltfm_data *k230_pdata = to_pltfm_data(dwc_priv, k230);
    u8 emmc_ctrl;
    dwcmshc_reset(host, mask);
    if (mask != SDHCI_RESET_ALL)
    return;
    emmc_ctrl = sdhci_readw(host, dwc_priv.vendor_specific_area1 + DWCMSHC_EMMC_CONTROL);
    sdhci_writeb(host, emmc_ctrl, dwc_priv.vendor_specific_area1 + DWCMSHC_EMMC_CONTROL);
    if (k230_pdata.is_emmc)
    dwcmshc_k230_phy_init(host);
    else
    sdhci_writel(host, 0x0, dwc_priv.vendor_specific_area1 + DWCMSHC_HOST_CTRL3);
    }
    static int dwcmshc_k230_init(struct device *dev, struct sdhci_host *host,
    struct dwcmshc_priv *dwc_priv)
    {
    const struct k230_pltfm_data *k230_pdata = to_pltfm_data(dwc_priv, k230);
    static const char * const clk_ids[] = {"block", "timer", "axi"};
    struct device_node *usb_phy_node;
    struct k230_priv *k230_priv;
    u32 data;
    int ret;
    k230_priv = devm_kzalloc(dev, sizeof(struct k230_priv), GFP_KERNEL);
    if (!k230_priv)
    return -ENOMEM;
    dwc_priv.priv = k230_priv;
    usb_phy_node = of_parse_phandle(dev.of_node, "canaan,usb-phy", 0);
    if (!usb_phy_node)
    return dev_err_probe(dev, -ENODEV, "Failed to find canaan,usb-phy phandle\n");
    k230_priv.hi_sys_regmap = device_node_to_regmap(usb_phy_node);
    of_node_put(usb_phy_node);
    if (IS_ERR(k230_priv.hi_sys_regmap))
    return dev_err_probe(dev, PTR_ERR(k230_priv.hi_sys_regmap),
    "Failed to get k230-usb-phy regmap\n");
    ret = dwcmshc_get_enable_other_clks(mmc_dev(host.mmc), dwc_priv,
    ARRAY_SIZE(clk_ids), clk_ids);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to get/enable k230 mmc other clocks\n");
    if (k230_pdata.is_emmc) {
    host.flags &= ~SDHCI_SIGNALING_330;
    dwc_priv.flags |= FLAG_IO_FIXED_1V8;
    } else {
    host.mmc.caps |= MMC_CAP_SD_HIGHSPEED;
    host.quirks2 |= SDHCI_QUIRK2_NO_1_8_V;
    }
    ret = regmap_read(k230_priv.hi_sys_regmap, k230_pdata.ctrl_reg, &data);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to read control reg 0x%x\n",
    k230_pdata.ctrl_reg);
    data |= k230_pdata.write_prot_bit | k230_pdata.vol_stable_bit;
    ret = regmap_write(k230_priv.hi_sys_regmap, k230_pdata.ctrl_reg, data);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to write control reg 0x%x\n",
    k230_pdata.ctrl_reg);
    return 0;
    }
    static const struct sdhci_ops sdhci_dwcmshc_ops = {
    .set_clock		= sdhci_set_clock,
    .set_bus_width		= sdhci_set_bus_width,
    .set_uhs_signaling	= dwcmshc_set_uhs_signaling,
    .get_max_clock		= dwcmshc_get_max_clock,
    .reset			= dwcmshc_reset,
    .adma_write_desc	= dwcmshc_adma_write_desc,
    .irq			= dwcmshc_cqe_irq_handler,
    };

#[no_mangle]
unsafe extern "C" fn dwcmshc_bf3_hw_reset(host: *mut sdhci_host) {
    static void dwcmshc_bf3_hw_reset(struct sdhci_host *host)
    {
    let mut res: arm_smccc_res = { 0 };
    pr_debug("%s: resetting...\n", __func__);
    arm_smccc_smc(BLUEFIELD_SMC_SET_EMMC_RST_N, 0, 0, 0, 0, 0, 0, 0, &res);
    if (res.a0)
    pr_err("%s: RST_N failed.\n", mmc_hostname(host.mmc));
    }
    static const struct sdhci_ops sdhci_dwcmshc_bf3_ops = {
    .set_clock		= sdhci_set_clock,
    .set_bus_width		= sdhci_set_bus_width,
    .set_uhs_signaling	= dwcmshc_set_uhs_signaling,
    .get_max_clock		= dwcmshc_get_max_clock,
    .reset			= sdhci_reset,
    .adma_write_desc	= dwcmshc_adma_write_desc,
    .irq			= dwcmshc_cqe_irq_handler,
    .hw_reset		= dwcmshc_bf3_hw_reset,
    };

    static const struct sdhci_ops sdhci_dwcmshc_rk35xx_ops = {
    .set_clock		= dwcmshc_rk3568_set_clock,
    .set_bus_width		= sdhci_set_bus_width,
    .set_uhs_signaling	= dwcmshc_set_uhs_signaling,
    .get_max_clock		= rk35xx_get_max_clock,
    .reset			= rk35xx_sdhci_reset,
    .adma_write_desc	= dwcmshc_adma_write_desc,
    .irq			= dwcmshc_cqe_irq_handler,
    };
    static const struct sdhci_ops sdhci_dwcmshc_th1520_ops = {
    .set_clock		= sdhci_set_clock,
    .set_bus_width		= sdhci_set_bus_width,
    .set_uhs_signaling	= th1520_set_uhs_signaling,
    .get_max_clock		= dwcmshc_get_max_clock,
    .reset			= th1520_sdhci_reset,
    .adma_write_desc	= dwcmshc_adma_write_desc,
    .voltage_switch		= dwcmshc_phy_init,
    .platform_execute_tuning = th1520_execute_tuning,
    };
    static const struct sdhci_ops sdhci_dwcmshc_cv18xx_ops = {
    .set_clock		= sdhci_set_clock,
    .set_bus_width		= sdhci_set_bus_width,
    .set_uhs_signaling	= dwcmshc_set_uhs_signaling,
    .get_max_clock		= dwcmshc_get_max_clock,
    .reset			= cv18xx_sdhci_reset,
    .adma_write_desc	= dwcmshc_adma_write_desc,
    .platform_execute_tuning = cv18xx_sdhci_execute_tuning,
    };
    static const struct sdhci_ops sdhci_dwcmshc_sg2042_ops = {
    .set_clock		= sdhci_set_clock,
    .set_bus_width		= sdhci_set_bus_width,
    .set_uhs_signaling	= dwcmshc_set_uhs_signaling,
    .get_max_clock		= dwcmshc_get_max_clock,
    .reset			= sg2042_sdhci_reset,
    .adma_write_desc	= dwcmshc_adma_write_desc,
    .platform_execute_tuning = th1520_execute_tuning,
    };
    static const struct sdhci_ops sdhci_dwcmshc_eic7700_ops = {
    .set_clock = sdhci_eic7700_set_clock,
    .get_max_clock = sdhci_pltfm_clk_get_max_clock,
    .get_timeout_clock = sdhci_pltfm_clk_get_max_clock,
    .set_bus_width = sdhci_set_bus_width,
    .reset = sdhci_eic7700_reset,
    .set_uhs_signaling = sdhci_eic7700_set_uhs_wrapper,
    .set_power = sdhci_set_power_and_bus_voltage,
    .irq = dwcmshc_cqe_irq_handler,
    .adma_write_desc = dwcmshc_adma_write_desc,
    .platform_execute_tuning = sdhci_eic7700_executing_tuning,
    };
    static const struct sdhci_ops sdhci_dwcmshc_k230_ops = {
    .set_clock = dwcmshc_k230_sdhci_set_clock,
    .set_bus_width = sdhci_set_bus_width,
    .set_uhs_signaling = dwcmshc_set_uhs_signaling,
    .get_max_clock = sdhci_pltfm_clk_get_max_clock,
    .reset = dwcmshc_k230_sdhci_reset,
    .adma_write_desc = dwcmshc_adma_write_desc,
    };
    static const struct dwcmshc_pltfm_data sdhci_dwcmshc_pdata = {
    .pdata = {
    .ops = &sdhci_dwcmshc_ops,
    .quirks = SDHCI_QUIRK_CAP_CLOCK_BASE_BROKEN,
    .quirks2 = SDHCI_QUIRK2_PRESET_VALUE_BROKEN,
    },
    };

    static const struct dwcmshc_pltfm_data sdhci_dwcmshc_bf3_pdata = {
    .pdata = {
    .ops = &sdhci_dwcmshc_bf3_ops,
    .quirks = SDHCI_QUIRK_CAP_CLOCK_BASE_BROKEN,
    .quirks2 = SDHCI_QUIRK2_PRESET_VALUE_BROKEN |
    SDHCI_QUIRK2_ACMD23_BROKEN,
    },
    };

    static const struct cqhci_host_ops rk35xx_cqhci_ops = {
    .pre_enable	= rk35xx_sdhci_cqe_pre_enable,
    .enable		= rk35xx_sdhci_cqe_enable,
    .disable	= rk35xx_sdhci_cqe_disable,
    .post_disable	= rk35xx_sdhci_cqe_post_disable,
    .dumpregs	= dwcmshc_cqhci_dumpregs,
    .set_tran_desc	= dwcmshc_set_tran_desc,
    };
    static const struct rockchip_pltfm_data sdhci_dwcmshc_rk3568_pdata = {
    .dwcmshc_pdata = {
    .pdata = {
    .ops = &sdhci_dwcmshc_rk35xx_ops,
    .quirks = SDHCI_QUIRK_CAP_CLOCK_BASE_BROKEN |
    SDHCI_QUIRK_BROKEN_TIMEOUT_VAL,
    .quirks2 = SDHCI_QUIRK2_PRESET_VALUE_BROKEN |
    SDHCI_QUIRK2_CLOCK_DIV_ZERO_BROKEN,
    },
    .cqhci_host_ops = &rk35xx_cqhci_ops,
    .init = dwcmshc_rk35xx_init,
    .postinit = dwcmshc_rk35xx_postinit,
    },
    .revision = 0,
    };
    static const struct rockchip_pltfm_data sdhci_dwcmshc_rk3576_pdata = {
    .dwcmshc_pdata = {
    .pdata = {
    .ops = &sdhci_dwcmshc_rk35xx_ops,
    .quirks = SDHCI_QUIRK_CAP_CLOCK_BASE_BROKEN |
    SDHCI_QUIRK_BROKEN_TIMEOUT_VAL,
    .quirks2 = SDHCI_QUIRK2_PRESET_VALUE_BROKEN |
    SDHCI_QUIRK2_CLOCK_DIV_ZERO_BROKEN,
    },
    .cqhci_host_ops = &rk35xx_cqhci_ops,
    .init = dwcmshc_rk35xx_init,
    .postinit = dwcmshc_rk3576_postinit,
    },
    .revision = 1,
    };
    static const struct rockchip_pltfm_data sdhci_dwcmshc_rk3588_pdata = {
    .dwcmshc_pdata = {
    .pdata = {
    .ops = &sdhci_dwcmshc_rk35xx_ops,
    .quirks = SDHCI_QUIRK_CAP_CLOCK_BASE_BROKEN |
    SDHCI_QUIRK_BROKEN_TIMEOUT_VAL,
    .quirks2 = SDHCI_QUIRK2_PRESET_VALUE_BROKEN |
    SDHCI_QUIRK2_CLOCK_DIV_ZERO_BROKEN,
    },
    .cqhci_host_ops = &rk35xx_cqhci_ops,
    .init = dwcmshc_rk35xx_init,
    .postinit = dwcmshc_rk35xx_postinit,
    },
    .revision = 1,
    };
    static const struct dwcmshc_pltfm_data sdhci_dwcmshc_th1520_pdata = {
    .pdata = {
    .ops = &sdhci_dwcmshc_th1520_ops,
    .quirks = SDHCI_QUIRK_CAP_CLOCK_BASE_BROKEN,
    .quirks2 = SDHCI_QUIRK2_PRESET_VALUE_BROKEN,
    },
    .init = th1520_init,
    };
    static const struct dwcmshc_pltfm_data sdhci_dwcmshc_cv18xx_pdata = {
    .pdata = {
    .ops = &sdhci_dwcmshc_cv18xx_ops,
    .quirks = SDHCI_QUIRK_CAP_CLOCK_BASE_BROKEN,
    .quirks2 = SDHCI_QUIRK2_PRESET_VALUE_BROKEN,
    },
    };
    static const struct dwcmshc_pltfm_data sdhci_dwcmshc_sg2042_pdata = {
    .pdata = {
    .ops = &sdhci_dwcmshc_sg2042_ops,
    .quirks = SDHCI_QUIRK_CAP_CLOCK_BASE_BROKEN,
    .quirks2 = SDHCI_QUIRK2_PRESET_VALUE_BROKEN,
    },
    .init = sg2042_init,
    };
    static const struct dwcmshc_pltfm_data sdhci_dwcmshc_eic7700_pdata = {
    .pdata = {
    .ops = &sdhci_dwcmshc_eic7700_ops,
    .quirks = SDHCI_QUIRK_CAP_CLOCK_BASE_BROKEN |
    SDHCI_QUIRK_BROKEN_TIMEOUT_VAL,
    .quirks2 = SDHCI_QUIRK2_PRESET_VALUE_BROKEN |
    SDHCI_QUIRK2_CLOCK_DIV_ZERO_BROKEN,
    },
    .init = eic7700_init,
    };
    static const struct k230_pltfm_data k230_emmc_data = {
    .dwcmshc_pdata = {
    .pdata = {
    .ops = &sdhci_dwcmshc_k230_ops,
    .quirks = SDHCI_QUIRK_CAP_CLOCK_BASE_BROKEN |
    SDHCI_QUIRK_MULTIBLOCK_READ_ACMD12,
    },
    .init = dwcmshc_k230_init,
    },
    .is_emmc = true,
    .ctrl_reg = SD0_CTRL,
    .vol_stable_bit = SD0_HOST_REG_VOL_STABLE,
    .write_prot_bit = SD0_CARD_WRITE_PROT,
    };
    static const struct k230_pltfm_data k230_sdio_data = {
    .dwcmshc_pdata = {
    .pdata = {
    .ops = &sdhci_dwcmshc_k230_ops,
    .quirks = SDHCI_QUIRK_CAP_CLOCK_BASE_BROKEN |
    SDHCI_QUIRK_MULTIBLOCK_READ_ACMD12,
    },
    .init = dwcmshc_k230_init,
    },
    .is_emmc = false,
    .ctrl_reg = SD1_CTRL,
    .vol_stable_bit = SD1_HOST_REG_VOL_STABLE,
    .write_prot_bit = SD1_CARD_WRITE_PROT,
    };
    static const struct sdhci_ops sdhci_dwcmshc_hpe_ops = {
    .set_clock		= dwcmshc_hpe_set_clock,
    .set_bus_width		= sdhci_set_bus_width,
    .set_uhs_signaling	= dwcmshc_hpe_set_uhs_signaling,
    .get_max_clock		= dwcmshc_get_max_clock,
    .reset			= dwcmshc_hpe_reset,
    .adma_write_desc	= dwcmshc_adma_write_desc,
    .irq			= dwcmshc_cqe_irq_handler,
    };
    static const struct dwcmshc_pltfm_data sdhci_dwcmshc_hpe_gsc_pdata = {
    .pdata = {
    .ops = &sdhci_dwcmshc_hpe_ops,
    .quirks = SDHCI_QUIRK_CAP_CLOCK_BASE_BROKEN,
    .quirks2 = SDHCI_QUIRK2_PRESET_VALUE_BROKEN,
    },
    .init = dwcmshc_hpe_gsc_init,
    };
    static const struct cqhci_host_ops dwcmshc_cqhci_ops = {
    .enable		= dwcmshc_sdhci_cqe_enable,
    .disable	= sdhci_cqe_disable,
    .dumpregs	= dwcmshc_cqhci_dumpregs,
    .set_tran_desc	= dwcmshc_set_tran_desc,
    };
    static void dwcmshc_cqhci_init(struct sdhci_host *host, struct platform_device *pdev,
    const struct dwcmshc_pltfm_data *pltfm_data)
    {
    struct cqhci_host *cq_host;
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *priv = sdhci_pltfm_priv(pltfm_host);
    let mut dma64: bool = false;
    u16 clk;
    int err;
    host.mmc.caps2 |= MMC_CAP2_CQE | MMC_CAP2_CQE_DCMD;
    cq_host = devm_kzalloc(&pdev.dev, sizeof(*cq_host), GFP_KERNEL);
    if (!cq_host) {
    dev_err(mmc_dev(host.mmc), "Unable to setup CQE: not enough memory\n");
    goto dsbl_cqe_caps;
    }
//
// For dwcmshc host controller we have to enable internal clock
// before access to some registers from Vendor Specific Area 2.
//
    clk = sdhci_readw(host, SDHCI_CLOCK_CONTROL);
    clk |= SDHCI_CLOCK_INT_EN;
    sdhci_writew(host, clk, SDHCI_CLOCK_CONTROL);
    clk = sdhci_readw(host, SDHCI_CLOCK_CONTROL);
    if (!(clk & SDHCI_CLOCK_INT_EN)) {
    dev_err(mmc_dev(host.mmc), "Unable to setup CQE: internal clock enable error\n");
    goto free_cq_host;
    }
    cq_host.mmio = host.ioaddr + priv.vendor_specific_area2;
    if (pltfm_data.cqhci_host_ops)
    cq_host.ops = pltfm_data.cqhci_host_ops;
    else
    cq_host.ops = &dwcmshc_cqhci_ops;
// Enable using of 128-bit task descriptors
    dma64 = host.flags & SDHCI_USE_64_BIT_DMA;
    if (dma64) {
    dev_dbg(mmc_dev(host.mmc), "128-bit task descriptors\n");
    cq_host.caps |= CQHCI_TASK_DESC_SZ_128;
    }
    err = cqhci_init(cq_host, host.mmc, dma64);
    if (err) {
    dev_err(mmc_dev(host.mmc), "Unable to setup CQE: error %d\n", err);
    goto int_clock_disable;
    }
    dev_dbg(mmc_dev(host.mmc), "CQE init done\n");
    return;
    int_clock_disable:
    clk = sdhci_readw(host, SDHCI_CLOCK_CONTROL);
    clk &= ~SDHCI_CLOCK_INT_EN;
    sdhci_writew(host, clk, SDHCI_CLOCK_CONTROL);
    free_cq_host:
    devm_kfree(&pdev.dev, cq_host);
    dsbl_cqe_caps:
    host.mmc.caps2 &= ~(MMC_CAP2_CQE | MMC_CAP2_CQE_DCMD);
    }
    static const struct of_device_id sdhci_dwcmshc_dt_ids[] = {
    {
    .compatible = "canaan,k230-emmc",
    .data = &k230_emmc_data.dwcmshc_pdata,
    },
    {
    .compatible = "canaan,k230-sdio",
    .data = &k230_sdio_data.dwcmshc_pdata,
    },
    {
    .compatible = "rockchip,rk3588-dwcmshc",
    .data = &sdhci_dwcmshc_rk3588_pdata,
    },
    {
    .compatible = "rockchip,rk3576-dwcmshc",
    .data = &sdhci_dwcmshc_rk3576_pdata,
    },
    {
    .compatible = "rockchip,rk3568-dwcmshc",
    .data = &sdhci_dwcmshc_rk3568_pdata,
    },
    {
    .compatible = "snps,dwcmshc-sdhci",
    .data = &sdhci_dwcmshc_pdata,
    },
    {
    .compatible = "sophgo,cv1800b-dwcmshc",
    .data = &sdhci_dwcmshc_cv18xx_pdata,
    },
    {
    .compatible = "sophgo,sg2002-dwcmshc",
    .data = &sdhci_dwcmshc_cv18xx_pdata,
    },
    {
    .compatible = "thead,th1520-dwcmshc",
    .data = &sdhci_dwcmshc_th1520_pdata,
    },
    {
    .compatible = "sophgo,sg2042-dwcmshc",
    .data = &sdhci_dwcmshc_sg2042_pdata,
    },
    {
    .compatible = "eswin,eic7700-dwcmshc",
    .data = &sdhci_dwcmshc_eic7700_pdata,
    },
    {
    .compatible = "hpe,gsc-dwcmshc",
    .data = &sdhci_dwcmshc_hpe_gsc_pdata,
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, sdhci_dwcmshc_dt_ids);

    static const struct acpi_device_id sdhci_dwcmshc_acpi_ids[] = {
    {
    .id = "MLNXBF30",
    .driver_data = (kernel_ulong_t)&sdhci_dwcmshc_bf3_pdata,
    },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, sdhci_dwcmshc_acpi_ids);

#[no_mangle]
unsafe extern "C" fn dwcmshc_probe(pdev: *mut platform_device) -> c_int {
    static int dwcmshc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct sdhci_pltfm_host *pltfm_host;
    struct sdhci_host *host;
    struct dwcmshc_priv *priv;
    const struct dwcmshc_pltfm_data *pltfm_data;
    int err;
    u32 extra, caps;
    pltfm_data = device_get_match_data(&pdev.dev);
    if (!pltfm_data)
    return dev_err_probe(&pdev.dev, -ENODEV, "No device match data found\n");
    host = sdhci_pltfm_init(pdev, &pltfm_data.pdata,
    sizeof(struct dwcmshc_priv));
    if (IS_ERR(host))
    return PTR_ERR(host);
//
// extra adma table cnt for cross 128M boundary handling.
//
    extra = DIV_ROUND_UP_ULL(dma_get_required_mask(dev), SZ_128M);
    if (extra > SDHCI_MAX_SEGS)
    extra = SDHCI_MAX_SEGS;
    host.adma_table_cnt += extra;
    pltfm_host = sdhci_priv(host);
    priv = sdhci_pltfm_priv(pltfm_host);
    priv.dwcmshc_pdata = pltfm_data;
    if (dev.of_node) {
    pltfm_host.clk = devm_clk_get(dev, "core");
    if (IS_ERR(pltfm_host.clk))
    return dev_err_probe(dev, PTR_ERR(pltfm_host.clk),
    "failed to get core clk\n");
    err = clk_prepare_enable(pltfm_host.clk);
    if (err)
    return err;
    priv.bus_clk = devm_clk_get(dev, "bus");
    if (!IS_ERR(priv.bus_clk)) {
    err = clk_prepare_enable(priv.bus_clk);
    if (err)
    goto err_clk;
    }
    }
    err = mmc_of_parse(host.mmc);
    if (err)
    goto err_bus_clk;
    sdhci_get_of_property(pdev);
    priv.vendor_specific_area1 =
    sdhci_readl(host, DWCMSHC_P_VENDOR_AREA1) & DWCMSHC_AREA1_MASK;
    host.mmc_host_ops.request = dwcmshc_request;
    host.mmc_host_ops.hs400_enhanced_strobe = dwcmshc_hs400_enhanced_strobe;
    host.mmc_host_ops.execute_tuning = dwcmshc_execute_tuning;
    if (pltfm_data.init) {
    err = pltfm_data.init(&pdev.dev, host, priv);
    if (err)
    goto err_bus_clk;
    }

    if (pltfm_data == &sdhci_dwcmshc_bf3_pdata)
    sdhci_enable_v4_mode(host);

    caps = sdhci_readl(host, SDHCI_CAPABILITIES);
    if (caps & SDHCI_CAN_64BIT_V4)
    sdhci_enable_v4_mode(host);
    host.mmc.caps |= MMC_CAP_WAIT_WHILE_BUSY;
    pm_runtime_get_noresume(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    err = sdhci_setup_host(host);
    if (err)
    goto err_rpm;
// Setup Command Queue Engine if enabled
    if (device_property_read_bool(&pdev.dev, "supports-cqe")) {
    priv.vendor_specific_area2 =
    sdhci_readw(host, DWCMSHC_P_VENDOR_AREA2);
    dwcmshc_cqhci_init(host, pdev, pltfm_data);
    }
    if (pltfm_data.postinit)
    pltfm_data.postinit(host, priv);
    err = __sdhci_add_host(host);
    if (err)
    goto err_setup_host;
    pm_runtime_put(dev);
    return 0;
    err_setup_host:
    sdhci_cleanup_host(host);
    err_rpm:
    pm_runtime_disable(dev);
    pm_runtime_put_noidle(dev);
    err_bus_clk:
    clk_disable_unprepare(priv.bus_clk);
    err_clk:
    clk_disable_unprepare(pltfm_host.clk);
    clk_bulk_disable_unprepare(priv.num_other_clks, priv.other_clks);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_disable_card_clk(host: *mut sdhci_host) {
    static void dwcmshc_disable_card_clk(struct sdhci_host *host)
    {
    u16 ctrl;
    ctrl = sdhci_readw(host, SDHCI_CLOCK_CONTROL);
    if (ctrl & SDHCI_CLOCK_CARD_EN) {
    ctrl &= ~SDHCI_CLOCK_CARD_EN;
    sdhci_writew(host, ctrl, SDHCI_CLOCK_CONTROL);
    }
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_remove(pdev: *mut platform_device) {
    static void dwcmshc_remove(struct platform_device *pdev)
    {
    struct sdhci_host *host = platform_get_drvdata(pdev);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *priv = sdhci_pltfm_priv(pltfm_host);
    pm_runtime_get_sync(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    pm_runtime_put_noidle(&pdev.dev);
    sdhci_remove_host(host, 0);
    dwcmshc_disable_card_clk(host);
    clk_disable_unprepare(pltfm_host.clk);
    clk_disable_unprepare(priv.bus_clk);
    clk_bulk_disable_unprepare(priv.num_other_clks, priv.other_clks);
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_suspend(dev: *mut device) -> c_int {
    static int dwcmshc_suspend(struct device *dev)
    {
    struct sdhci_host *host = dev_get_drvdata(dev);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *priv = sdhci_pltfm_priv(pltfm_host);
    int ret;
    pm_runtime_resume(dev);
    if (host.mmc.caps2 & MMC_CAP2_CQE) {
    ret = cqhci_suspend(host.mmc);
    if (ret)
    return ret;
    }
    ret = sdhci_suspend_host(host);
    if (ret)
    return ret;
    clk_disable_unprepare(pltfm_host.clk);
    clk_disable_unprepare(priv.bus_clk);
    clk_bulk_disable_unprepare(priv.num_other_clks, priv.other_clks);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_resume(dev: *mut device) -> c_int {
    static int dwcmshc_resume(struct device *dev)
    {
    struct sdhci_host *host = dev_get_drvdata(dev);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct dwcmshc_priv *priv = sdhci_pltfm_priv(pltfm_host);
    int ret;
    ret = clk_prepare_enable(pltfm_host.clk);
    if (ret)
    return ret;
    if (!IS_ERR(priv.bus_clk)) {
    ret = clk_prepare_enable(priv.bus_clk);
    if (ret)
    goto disable_clk;
    }
    ret = clk_bulk_prepare_enable(priv.num_other_clks, priv.other_clks);
    if (ret)
    goto disable_bus_clk;
    ret = sdhci_resume_host(host);
    if (ret)
    goto disable_other_clks;
    if (host.mmc.caps2 & MMC_CAP2_CQE) {
    ret = cqhci_resume(host.mmc);
    if (ret)
    goto disable_other_clks;
    }
    return 0;
    disable_other_clks:
    clk_bulk_disable_unprepare(priv.num_other_clks, priv.other_clks);
    disable_bus_clk:
    clk_disable_unprepare(priv.bus_clk);
    disable_clk:
    clk_disable_unprepare(pltfm_host.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_runtime_suspend(dev: *mut device) -> c_int {
    static int dwcmshc_runtime_suspend(struct device *dev)
    {
    struct sdhci_host *host = dev_get_drvdata(dev);
    dwcmshc_disable_card_clk(host);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwcmshc_runtime_resume(dev: *mut device) -> c_int {
    static int dwcmshc_runtime_resume(struct device *dev)
    {
    struct sdhci_host *host = dev_get_drvdata(dev);
    dwcmshc_enable_card_clk(host);
    return 0;
    }
    static const struct dev_pm_ops dwcmshc_pmops = {
    SYSTEM_SLEEP_PM_OPS(dwcmshc_suspend, dwcmshc_resume)
    RUNTIME_PM_OPS(dwcmshc_runtime_suspend, dwcmshc_runtime_resume, core::ptr::null_mut())
    };
    static struct platform_driver sdhci_dwcmshc_driver = {
    .driver	= {
    .name	= "sdhci-dwcmshc",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = sdhci_dwcmshc_dt_ids,
    .acpi_match_table = ACPI_PTR(sdhci_dwcmshc_acpi_ids),
    .pm = pm_ptr(&dwcmshc_pmops),
    },
    .probe	= dwcmshc_probe,
    .remove = dwcmshc_remove,
    };
    module_platform_driver(sdhci_dwcmshc_driver);
    MODULE_DESCRIPTION("SDHCI platform driver for Synopsys DWC MSHC");
    MODULE_AUTHOR("Jisheng Zhang <jszhang@kernel.org>");
    MODULE_LICENSE("GPL v2");
