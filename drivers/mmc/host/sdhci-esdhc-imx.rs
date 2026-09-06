//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/sdhci-esdhc-imx.c
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
// Freescale eSDHC i.MX controller driver for the platform bus.
//
// derived from the OF-version.
//
// Copyright (c) 2010 Pengutronix e.K.
// Author: Wolfram Sang <kernel@pengutronix.de>
//

pub const ESDHC_CTRL_D3CD: c_uint = 0x08;

// VENDOR SPEC register
pub const ESDHC_VENDOR_SPEC: c_uint = 0xc0;

pub const ESDHC_DEBUG_SEL_AND_STATUS_REG: c_uint = 0xc2;
pub const ESDHC_DEBUG_SEL_REG: c_uint = 0xc3;
pub const ESDHC_DEBUG_SEL_MASK: c_uint = 0xf;
pub const ESDHC_DEBUG_SEL_CMD_STATE: c_int = 1;
pub const ESDHC_DEBUG_SEL_DATA_STATE: c_int = 2;
pub const ESDHC_DEBUG_SEL_TRANS_STATE: c_int = 3;
pub const ESDHC_DEBUG_SEL_DMA_STATE: c_int = 4;
pub const ESDHC_DEBUG_SEL_ADMA_STATE: c_int = 5;
pub const ESDHC_DEBUG_SEL_FIFO_STATE: c_int = 6;
pub const ESDHC_DEBUG_SEL_ASYNC_FIFO_STATE: c_int = 7;
pub const ESDHC_WTMK_LVL: c_uint = 0x44;
pub const ESDHC_WTMK_DEFAULT_VAL: c_uint = 0x10401040;
pub const ESDHC_WTMK_LVL_RD_WML_MASK: c_uint = 0x000000FF;
pub const ESDHC_WTMK_LVL_RD_WML_SHIFT: c_int = 0;
pub const ESDHC_WTMK_LVL_WR_WML_MASK: c_uint = 0x00FF0000;
pub const ESDHC_WTMK_LVL_WR_WML_SHIFT: c_int = 16;
pub const ESDHC_WTMK_LVL_WML_VAL_DEF: c_int = 64;
pub const ESDHC_WTMK_LVL_WML_VAL_MAX: c_int = 128;
pub const ESDHC_MIX_CTRL: c_uint = 0x48;

// Bits 3 and 6 are not SDHCI standard definitions
pub const ESDHC_MIX_CTRL_SDHCI_MASK: c_uint = 0xb7;
// Tuning bits
pub const ESDHC_MIX_CTRL_TUNING_MASK: c_uint = 0x03c00000;
// dll control register
pub const ESDHC_DLL_CTRL: c_uint = 0x60;
pub const ESDHC_DLL_OVERRIDE_VAL_SHIFT: c_int = 9;
pub const ESDHC_DLL_OVERRIDE_EN_SHIFT: c_int = 8;
// tune control register
pub const ESDHC_TUNE_CTRL_STATUS: c_uint = 0x68;
pub const ESDHC_TUNE_CTRL_STEP: c_int = 1;
pub const ESDHC_TUNE_CTRL_MIN: c_int = 0;

// strobe dll register
pub const ESDHC_STROBE_DLL_CTRL: c_uint = 0x70;

pub const ESDHC_STROBE_DLL_CTRL_SLV_DLY_TARGET_DEFAULT: c_uint = 0x7;
pub const ESDHC_STROBE_DLL_CTRL_SLV_DLY_TARGET_SHIFT: c_int = 3;

pub const ESDHC_STROBE_DLL_STATUS: c_uint = 0x74;

pub const ESDHC_STROBE_DLL_STS_SLV_LOCK: c_uint = 0x1;
pub const ESDHC_VEND_SPEC2: c_uint = 0xc8;

pub const ESDHC_TUNING_CTRL: c_uint = 0xcc;

// NOTE: the minimum valid tuning start tap for mx6sl is 1
pub const ESDHC_TUNING_START_TAP_DEFAULT: c_uint = 0x1;
pub const ESDHC_TUNING_START_TAP_MASK: c_uint = 0x7f;

pub const ESDHC_TUNING_STEP_DEFAULT: c_uint = 0x1;
pub const ESDHC_TUNING_STEP_MASK: c_uint = 0x00070000;
pub const ESDHC_TUNING_STEP_SHIFT: c_int = 16;
// pinctrl state

//
// Our interpretation of the SDHCI_HOST_CONTROL register
//

//
// There is an INT DMA ERR mismatch between eSDHC and STD SDHC SPEC:
// Bit25 is used in STD SPEC, and is reserved in fsl eSDHC design,
// but bit28 is used as the INT DMA ERR in fsl eSDHC design.
// Define this macro DMA error INT for fsl eSDHC
//

// the address offset of CQHCI
pub const ESDHC_CQHCI_ADDR_OFFSET: c_uint = 0x100;
//
// The CMDTYPE of the CMD register (offset 0xE) should be set to
// "11" when the STOP CMD12 is issued on imx53 to abort one
// open ended multi-blk IO. Otherwise the TC INT wouldn't
// be generated.
// In exact block transfer, the controller doesn't complete the
// operations automatically as required at the end of the
// transfer and remains on hold if the abort command is not sent.
// As a result, the TC flag is not asserted and SW received timeout
// exception. Bit1 of Vendor Spec register is used to fix it.
//

//
// The flag tells that the ESDHC controller is an USDHC block that is
// integrated on the i.MX6 series.
//

// The IP supports manual tuning process

// The IP supports standard tuning process

// The IP has SDHCI_CAPABILITIES_1 register

//
// The IP has erratum ERR004536
// uSDHC: ADMA Length Mismatch Error occurs if the AHB read access is slow,
// when reading data from the card
// This flag is also set for i.MX25 and i.MX35 in order to get
// SDHCI_QUIRK_BROKEN_ADMA, but for different reasons (ADMA capability bits).
//

// The IP supports HS200 mode

// The IP supports HS400 mode

//
// The IP has errata ERR010450
// uSDHC: At 1.8V due to the I/O timing limit, for SDR mode, SD card
// clock can't exceed 150MHz, for DDR mode, SD card clock can't exceed 45MHz.
//

// The IP supports HS400ES mode

// The IP has Host Controller Interface for Command Queuing

// need request pmqos during low power

// The IP state got lost in low power mode

// The IP lost clock rate in PM_RUNTIME

//
// The IP do not support the ACMD23 feature completely when use ADMA mode.
// In ADMA mode, it only use the 16 bit block count of the register 0x4
// (BLOCK_ATT) as the CMD23's argument for ACMD23 mode, which means it will
// ignore the upper 16 bit of the CMD23's argument. This will block the reliable
// write operation in RPMB, because RPMB reliable write need to set the bit31
// of the CMD23's argument.
// imx6qpdl/imx6sx/imx6sl/imx7d has this limitation only for ADMA mode, SDMA
// do not has this limitation. so when these SoC use ADMA mode, it need to
// disable the ACMD23 feature.
//

// ERR004536 is not applicable for the IP

// The IP does not have GPIO CD wake capabilities

// the controller has dummy pad for clock loopback

pub const ESDHC_AUTO_TUNING_WINDOW: c_int = 3;
// 100ms timeout for data inhibit
pub const ESDHC_DATA_INHIBIT_WAIT_US: c_int = 100000;
    enum wp_types {
    ESDHC_WP_NONE,		/* no WP, neither controller nor gpio */
    ESDHC_WP_CONTROLLER,	/* mmc controller internal WP */
    ESDHC_WP_GPIO,		/* external gpio pin for WP */
    };
    enum cd_types {
    ESDHC_CD_NONE,		/* no CD, neither controller nor gpio */
    ESDHC_CD_CONTROLLER,	/* mmc controller internal CD */
    ESDHC_CD_GPIO,		/* external gpio pin for CD */
    ESDHC_CD_PERMANENT,	/* no CD, card permanently wired to host */
    };
//
// struct esdhc_platform_data - platform data for esdhc on i.MX
//
// ESDHC_WP(CD)_CONTROLLER type is not available on i.MX25/35.
//
// @wp_type:	type of write_protect method (see wp_types enum above)
// @cd_type:	type of card_detect method (see cd_types enum above)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esdhc_platform_data {
    pub wp_type: enum wp_types,
    pub cd_type: enum cd_types,
    pub max_bus_width: c_int,
    pub delay_line: c_uint,
    pub /: *mut *mut unsigned int tuning_step; / The delay cell steps in tuning procedure,
    pub /: *mut *mut unsigned int tuning_start_tap; / The start delay cell point in tuning procedure,
    pub /: *mut *mut unsigned int strobe_dll_delay_target; / The delay cell for strobe pad (read clock),
    pub /: *mut *mut unsigned int saved_tuning_delay_cell; / save the value of tuning delay cell,
    pub /: *mut *mut unsigned int saved_auto_tuning_window; / save the auto tuning window width,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esdhc_soc_data {
    pub flags: u32,
    pub quirks: u32,
}

    static const struct esdhc_soc_data esdhc_imx25_data = {
    .flags = ESDHC_FLAG_ERR004536,
    };
    static const struct esdhc_soc_data esdhc_imx35_data = {
    .flags = ESDHC_FLAG_ERR004536,
    };
    static const struct esdhc_soc_data esdhc_imx51_data = {
    .flags = 0,
    };
    static const struct esdhc_soc_data esdhc_imx53_data = {
    .flags = ESDHC_FLAG_MULTIBLK_NO_INT,
    };
    static const struct esdhc_soc_data usdhc_imx6q_data = {
    .flags = ESDHC_FLAG_USDHC | ESDHC_FLAG_MAN_TUNING
    | ESDHC_FLAG_BROKEN_AUTO_CMD23,
    };
    static const struct esdhc_soc_data usdhc_imx6sl_data = {
    .flags = ESDHC_FLAG_USDHC | ESDHC_FLAG_MAN_TUNING
    | ESDHC_FLAG_HAVE_CAP1 | ESDHC_FLAG_ERR004536
    | ESDHC_FLAG_HS200
    | ESDHC_FLAG_BROKEN_AUTO_CMD23,
    };
    static const struct esdhc_soc_data usdhc_imx6sll_data = {
    .flags = ESDHC_FLAG_USDHC | ESDHC_FLAG_MAN_TUNING
    | ESDHC_FLAG_HAVE_CAP1 | ESDHC_FLAG_HS200
    | ESDHC_FLAG_HS400
    | ESDHC_FLAG_STATE_LOST_IN_LPMODE,
    };
    static const struct esdhc_soc_data usdhc_imx6sx_data = {
    .flags = ESDHC_FLAG_USDHC | ESDHC_FLAG_MAN_TUNING
    | ESDHC_FLAG_HAVE_CAP1 | ESDHC_FLAG_HS200
    | ESDHC_FLAG_STATE_LOST_IN_LPMODE
    | ESDHC_FLAG_BROKEN_AUTO_CMD23,
    };
    static const struct esdhc_soc_data usdhc_imx6ull_data = {
    .flags = ESDHC_FLAG_USDHC | ESDHC_FLAG_MAN_TUNING
    | ESDHC_FLAG_HAVE_CAP1 | ESDHC_FLAG_HS200
    | ESDHC_FLAG_ERR010450
    | ESDHC_FLAG_STATE_LOST_IN_LPMODE,
    };
    static const struct esdhc_soc_data usdhc_imx7d_data = {
    .flags = ESDHC_FLAG_USDHC | ESDHC_FLAG_MAN_TUNING
    | ESDHC_FLAG_HAVE_CAP1 | ESDHC_FLAG_HS200
    | ESDHC_FLAG_HS400
    | ESDHC_FLAG_STATE_LOST_IN_LPMODE
    | ESDHC_FLAG_BROKEN_AUTO_CMD23,
    };
    static struct esdhc_soc_data usdhc_s32g2_data = {
    .flags = ESDHC_FLAG_USDHC | ESDHC_FLAG_MAN_TUNING
    | ESDHC_FLAG_HAVE_CAP1 | ESDHC_FLAG_HS200
    | ESDHC_FLAG_HS400 | ESDHC_FLAG_HS400_ES
    | ESDHC_FLAG_SKIP_ERR004536 | ESDHC_FLAG_SKIP_CD_WAKE,
    .quirks = SDHCI_QUIRK_NO_LED,
    };
    static struct esdhc_soc_data usdhc_s32n79_data = {
    .flags = ESDHC_FLAG_USDHC | ESDHC_FLAG_MAN_TUNING
    | ESDHC_FLAG_HAVE_CAP1 | ESDHC_FLAG_HS200
    | ESDHC_FLAG_HS400 | ESDHC_FLAG_HS400_ES
    | ESDHC_FLAG_SKIP_ERR004536,
    .quirks = SDHCI_QUIRK_NO_LED,
    };
    static struct esdhc_soc_data usdhc_imx7ulp_data = {
    .flags = ESDHC_FLAG_USDHC | ESDHC_FLAG_MAN_TUNING
    | ESDHC_FLAG_HAVE_CAP1 | ESDHC_FLAG_HS200
    | ESDHC_FLAG_PMQOS | ESDHC_FLAG_HS400
    | ESDHC_FLAG_STATE_LOST_IN_LPMODE,
    .quirks = SDHCI_QUIRK_NO_LED,
    };
    static struct esdhc_soc_data usdhc_imxrt1050_data = {
    .flags = ESDHC_FLAG_USDHC | ESDHC_FLAG_STD_TUNING
    | ESDHC_FLAG_HAVE_CAP1 | ESDHC_FLAG_HS200,
    .quirks = SDHCI_QUIRK_NO_LED,
    };
    static struct esdhc_soc_data usdhc_imx8qxp_data = {
    .flags = ESDHC_FLAG_USDHC | ESDHC_FLAG_MAN_TUNING
    | ESDHC_FLAG_HAVE_CAP1 | ESDHC_FLAG_HS200
    | ESDHC_FLAG_HS400 | ESDHC_FLAG_HS400_ES
    | ESDHC_FLAG_STATE_LOST_IN_LPMODE
    | ESDHC_FLAG_CLK_RATE_LOST_IN_PM_RUNTIME,
    .quirks = SDHCI_QUIRK_NO_LED,
    };
    static struct esdhc_soc_data usdhc_imx8mm_data = {
    .flags = ESDHC_FLAG_USDHC | ESDHC_FLAG_MAN_TUNING
    | ESDHC_FLAG_HAVE_CAP1 | ESDHC_FLAG_HS200
    | ESDHC_FLAG_HS400 | ESDHC_FLAG_HS400_ES
    | ESDHC_FLAG_STATE_LOST_IN_LPMODE,
    .quirks = SDHCI_QUIRK_NO_LED,
    };
    static struct esdhc_soc_data usdhc_imx95_data = {
    .flags = ESDHC_FLAG_USDHC | ESDHC_FLAG_MAN_TUNING
    | ESDHC_FLAG_HAVE_CAP1 | ESDHC_FLAG_HS200
    | ESDHC_FLAG_HS400 | ESDHC_FLAG_HS400_ES
    | ESDHC_FLAG_STATE_LOST_IN_LPMODE
    | ESDHC_FLAG_DUMMY_PAD,
    .quirks = SDHCI_QUIRK_NO_LED,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pltfm_imx_data {
    pub scratchpad: u32,
    pub pinctrl: *mut pinctrl,
    pub pins_100mhz: *mut pinctrl_state,
    pub pins_200mhz: *mut pinctrl_state,
    pub socdata: *const esdhc_soc_data,
    pub boarddata: esdhc_platform_data,
    pub clk_ipg: *mut clk,
    pub clk_ahb: *mut clk,
    pub clk_per: *mut clk,
    pub actual_clock: c_uint,
//
// USDHC has one limition, require the SDIO device a different
// register setting. Driver has to recognize card type during
// the card init, but at this stage, mmc_host->card is not
// available. So involve this field to save the card type
// during card init through usdhc_init_card().
//
    pub init_card_type: c_uint,
    enum {
    NO_CMD_PENDING,      /* no multiblock command pending */
    MULTIBLK_IN_PROCESS, /* exact multiblock cmd in process */
    WAIT_FOR_INT,        /* sent CMD12, waiting for response INT */
    pub multiblock_status: },
    pub is_ddr: u32,
    pub pm_qos_req: pm_qos_request,
}

    static const struct of_device_id imx_esdhc_dt_ids[] = {
    { .compatible = "fsl,imx25-esdhc", .data = &esdhc_imx25_data, },
    { .compatible = "fsl,imx35-esdhc", .data = &esdhc_imx35_data, },
    { .compatible = "fsl,imx51-esdhc", .data = &esdhc_imx51_data, },
    { .compatible = "fsl,imx53-esdhc", .data = &esdhc_imx53_data, },
    { .compatible = "fsl,imx6sx-usdhc", .data = &usdhc_imx6sx_data, },
    { .compatible = "fsl,imx6sl-usdhc", .data = &usdhc_imx6sl_data, },
    { .compatible = "fsl,imx6sll-usdhc", .data = &usdhc_imx6sll_data, },
    { .compatible = "fsl,imx6q-usdhc", .data = &usdhc_imx6q_data, },
    { .compatible = "fsl,imx6ull-usdhc", .data = &usdhc_imx6ull_data, },
    { .compatible = "fsl,imx7d-usdhc", .data = &usdhc_imx7d_data, },
    { .compatible = "fsl,imx7ulp-usdhc", .data = &usdhc_imx7ulp_data, },
    { .compatible = "fsl,imx8qxp-usdhc", .data = &usdhc_imx8qxp_data, },
    { .compatible = "fsl,imx8mm-usdhc", .data = &usdhc_imx8mm_data, },
    { .compatible = "fsl,imx94-usdhc", .data = &usdhc_imx95_data, },
    { .compatible = "fsl,imx95-usdhc", .data = &usdhc_imx95_data, },
    { .compatible = "fsl,imxrt1050-usdhc", .data = &usdhc_imxrt1050_data, },
    { .compatible = "nxp,s32g2-usdhc", .data = &usdhc_s32g2_data, },
    { .compatible = "nxp,s32n79-usdhc", .data = &usdhc_s32n79_data, },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, imx_esdhc_dt_ids);
#[no_mangle]
pub unsafe extern "C" fn is_imx25_esdhc(data: *mut pltfm_imx_data) -> c_int {
    static inline int is_imx25_esdhc(struct pltfm_imx_data *data)
    {
    return data.socdata == &esdhc_imx25_data;
    }
#[no_mangle]
pub unsafe extern "C" fn is_imx53_esdhc(data: *mut pltfm_imx_data) -> c_int {
    static inline int is_imx53_esdhc(struct pltfm_imx_data *data)
    {
    return data.socdata == &esdhc_imx53_data;
    }
#[no_mangle]
pub unsafe extern "C" fn esdhc_is_usdhc(data: *mut pltfm_imx_data) -> c_int {
    static inline int esdhc_is_usdhc(struct pltfm_imx_data *data)
    {
    return !!(data.socdata.flags & ESDHC_FLAG_USDHC);
    }
#[no_mangle]
pub unsafe extern "C" fn esdhc_clrset_le(host: *mut sdhci_host, mask: u32, val: u32, reg: c_int) {
    static inline void esdhc_clrset_le(struct sdhci_host *host, u32 mask, u32 val, int reg)
    {
    void __iomem *base = host.ioaddr + (reg & ~0x3);
    let mut shift: u32 = (reg & 0x3) * 8;
    writel(((readl(base) & ~(mask << shift)) | (val << shift)), base);
    }

    pr_err("%s: " DRIVER_NAME ": " f, mmc_hostname(host.mmc), ## x)
#[no_mangle]
unsafe extern "C" fn esdhc_dump_debug_regs(host: *mut sdhci_host) {
    static void esdhc_dump_debug_regs(struct sdhci_host *host)
    {
    int i;
    char *debug_status[7] = {
    "cmd debug status",
    "data debug status",
    "trans debug status",
    "dma debug status",
    "adma debug status",
    "fifo debug status",
    "async fifo debug status"
    };
    ESDHC_IMX_DUMP("========= ESDHC IMX DEBUG STATUS DUMP =========\n");
    for (i = 0; i < 7; i++) {
    esdhc_clrset_le(host, ESDHC_DEBUG_SEL_MASK,
    ESDHC_DEBUG_SEL_CMD_STATE + i, ESDHC_DEBUG_SEL_REG);
    ESDHC_IMX_DUMP("%s:  0x%04x\n", debug_status[i],
    readw(host.ioaddr + ESDHC_DEBUG_SEL_AND_STATUS_REG));
    }
    esdhc_clrset_le(host, ESDHC_DEBUG_SEL_MASK, 0, ESDHC_DEBUG_SEL_REG);
    }
#[no_mangle]
pub unsafe extern "C" fn esdhc_wait_for_card_clock_gate_off(host: *mut sdhci_host) {
    static inline void esdhc_wait_for_card_clock_gate_off(struct sdhci_host *host)
    {
    u32 present_state;
    int ret;
    ret = readl_poll_timeout(host.ioaddr + ESDHC_PRSSTAT, present_state,
    (present_state & ESDHC_CLOCK_GATE_OFF), 2, 100);
    if (ret == -ETIMEDOUT)
    dev_warn(mmc_dev(host.mmc), "%s: card clock still not gate off in 100us!.\n", __func__);
    }
// Enable the auto tuning circuit to check the CMD line and BUS line
#[no_mangle]
pub unsafe extern "C" fn usdhc_auto_tuning_mode_sel_and_en(host: *mut sdhci_host) {
    static inline void usdhc_auto_tuning_mode_sel_and_en(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    u32 buswidth, auto_tune_buswidth;
    u32 reg;
    buswidth = USDHC_GET_BUSWIDTH(readl(host.ioaddr + SDHCI_HOST_CONTROL));
    switch (buswidth) {
    case ESDHC_CTRL_8BITBUS:
    auto_tune_buswidth = ESDHC_VEND_SPEC2_AUTO_TUNE_8BIT_EN;
    break;
    case ESDHC_CTRL_4BITBUS:
    auto_tune_buswidth = ESDHC_VEND_SPEC2_AUTO_TUNE_4BIT_EN;
    break;
    default:	/* 1BITBUS */
    auto_tune_buswidth = ESDHC_VEND_SPEC2_AUTO_TUNE_1BIT_EN;
    break;
    }
//
// For USDHC, auto tuning circuit can not handle the async sdio
// device interrupt correctly. When sdio device use 4 data lines,
// async sdio interrupt will use the shared DAT[1], if enable auto
// tuning circuit check these 4 data lines, include the DAT[1],
// this circuit will detect this interrupt, take this as a data on
// DAT[1], and adjust the delay cell wrongly.
// This is the hardware design limitation, to avoid this, for sdio
// device, config the auto tuning circuit only check DAT[0] and CMD
// line.
//
    if (imx_data.init_card_type == MMC_TYPE_SDIO)
    auto_tune_buswidth = ESDHC_VEND_SPEC2_AUTO_TUNE_1BIT_EN;
    esdhc_clrset_le(host, ESDHC_VEND_SPEC2_AUTO_TUNE_MODE_MASK,
    auto_tune_buswidth | ESDHC_VEND_SPEC2_AUTO_TUNE_CMD_EN,
    ESDHC_VEND_SPEC2);
    reg = readl(host.ioaddr + ESDHC_MIX_CTRL);
    reg |= ESDHC_MIX_CTRL_AUTO_TUNE_EN;
    writel(reg, host.ioaddr + ESDHC_MIX_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn esdhc_readl_le(host: *mut sdhci_host, reg: c_int) -> u32 {
    static u32 esdhc_readl_le(struct sdhci_host *host, int reg)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    let mut val: u32 = readl(host.ioaddr + reg);
    if (unlikely(reg == SDHCI_PRESENT_STATE)) {
    let mut fsl_prss: u32 = val;
// save the least 20 bits
    val = fsl_prss & 0x000FFFFF;
// move dat[0-3] bits
    val |= (fsl_prss & 0x0F000000) >> 4;
// move cmd line bit
    val |= (fsl_prss & 0x00800000) << 1;
    }
    if (unlikely(reg == SDHCI_CAPABILITIES)) {
// ignore bit[0-15] as it stores cap_1 register val for mx6sl
    if (imx_data.socdata.flags & ESDHC_FLAG_HAVE_CAP1)
    val &= 0xffff0000;
// In FSL esdhc IC module, only bit20 is used to indicate the
// ADMA2 capability of esdhc, but this bit is messed up on
// some SOCs (e.g. on MX25, MX35 this bit is set, but they
// don't actually support ADMA2). So set the BROKEN_ADMA
// quirk on MX25/35 platforms.
//
    if (val & SDHCI_CAN_DO_ADMA1) {
    val &= ~SDHCI_CAN_DO_ADMA1;
    val |= SDHCI_CAN_DO_ADMA2;
    }
    }
    if (unlikely(reg == SDHCI_CAPABILITIES_1)) {
    if (esdhc_is_usdhc(imx_data)) {
    if (imx_data.socdata.flags & ESDHC_FLAG_HAVE_CAP1)
    val = readl(host.ioaddr + SDHCI_CAPABILITIES) & 0xFFFF;
    else
// imx6q/dl does not have cap_1 register, fake one
    val = SDHCI_SUPPORT_DDR50 | SDHCI_SUPPORT_SDR104
    | SDHCI_SUPPORT_SDR50
    | SDHCI_USE_SDR50_TUNING
    | FIELD_PREP(SDHCI_RETUNING_MODE_MASK,
    SDHCI_TUNING_MODE_3);
//
// Do not advertise faster UHS modes if there are no
// pinctrl states for 100MHz/200MHz.
//
    if (IS_ERR_OR_NULL(imx_data.pins_100mhz))
    val &= ~(SDHCI_SUPPORT_SDR50 | SDHCI_SUPPORT_DDR50);
    if (IS_ERR_OR_NULL(imx_data.pins_200mhz))
    val &= ~(SDHCI_SUPPORT_SDR104 | SDHCI_SUPPORT_HS400);
    }
    }
    if (unlikely(reg == SDHCI_MAX_CURRENT) && esdhc_is_usdhc(imx_data)) {
    val = 0;
    val |= FIELD_PREP(SDHCI_MAX_CURRENT_330_MASK, 0xFF);
    val |= FIELD_PREP(SDHCI_MAX_CURRENT_300_MASK, 0xFF);
    val |= FIELD_PREP(SDHCI_MAX_CURRENT_180_MASK, 0xFF);
    }
    if (unlikely(reg == SDHCI_INT_STATUS)) {
    if (val & ESDHC_INT_VENDOR_SPEC_DMA_ERR) {
    val &= ~ESDHC_INT_VENDOR_SPEC_DMA_ERR;
    val |= SDHCI_INT_ADMA_ERROR;
    }
//
// mask off the interrupt we get in response to the manually
// sent CMD12
//
    if ((imx_data.multiblock_status == WAIT_FOR_INT) &&
    ((val & SDHCI_INT_RESPONSE) == SDHCI_INT_RESPONSE)) {
    val &= ~SDHCI_INT_RESPONSE;
    writel(SDHCI_INT_RESPONSE, host.ioaddr +
    SDHCI_INT_STATUS);
    imx_data.multiblock_status = NO_CMD_PENDING;
    }
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn esdhc_writel_le(host: *mut sdhci_host, val: u32, reg: c_int) {
    static void esdhc_writel_le(struct sdhci_host *host, u32 val, int reg)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    u32 data;
    if (unlikely(reg == SDHCI_INT_ENABLE || reg == SDHCI_SIGNAL_ENABLE ||
    reg == SDHCI_INT_STATUS)) {
    if ((val & SDHCI_INT_CARD_INT) && !esdhc_is_usdhc(imx_data)) {
//
// Clear and then set D3CD bit to avoid missing the
// card interrupt. This is an eSDHC controller problem
// so we need to apply the following workaround: clear
// and set D3CD bit will make eSDHC re-sample the card
// interrupt. In case a card interrupt was lost,
// re-sample it by the following steps.
//
    data = readl(host.ioaddr + SDHCI_HOST_CONTROL);
    data &= ~ESDHC_CTRL_D3CD;
    writel(data, host.ioaddr + SDHCI_HOST_CONTROL);
    data |= ESDHC_CTRL_D3CD;
    writel(data, host.ioaddr + SDHCI_HOST_CONTROL);
    }
    if (val & SDHCI_INT_ADMA_ERROR) {
    val &= ~SDHCI_INT_ADMA_ERROR;
    val |= ESDHC_INT_VENDOR_SPEC_DMA_ERR;
    }
    }
    if (unlikely((imx_data.socdata.flags & ESDHC_FLAG_MULTIBLK_NO_INT)
    && (reg == SDHCI_INT_STATUS)
    && (val & SDHCI_INT_DATA_END))) {
    u32 v;
    v = readl(host.ioaddr + ESDHC_VENDOR_SPEC);
    v &= ~ESDHC_VENDOR_SPEC_SDIO_QUIRK;
    writel(v, host.ioaddr + ESDHC_VENDOR_SPEC);
    if (imx_data.multiblock_status == MULTIBLK_IN_PROCESS)
    {
// send a manual CMD12 with RESPTYP=none
    data = MMC_STOP_TRANSMISSION << 24 |
    SDHCI_CMD_ABORTCMD << 16;
    writel(data, host.ioaddr + SDHCI_TRANSFER_MODE);
    imx_data.multiblock_status = WAIT_FOR_INT;
    }
    }
    writel(val, host.ioaddr + reg);
    }
#[no_mangle]
unsafe extern "C" fn esdhc_readw_le(host: *mut sdhci_host, reg: c_int) -> u16 {
    static u16 esdhc_readw_le(struct sdhci_host *host, int reg)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    let mut ret: u16 = 0;
    u32 val;
    if (unlikely(reg == SDHCI_HOST_VERSION)) {
    reg ^= 2;
    if (esdhc_is_usdhc(imx_data)) {
//
// The usdhc register returns a wrong host version.
// Correct it here.
//
    return SDHCI_SPEC_300;
    }
    }
    if (unlikely(reg == SDHCI_HOST_CONTROL2)) {
    val = readl(host.ioaddr + ESDHC_VENDOR_SPEC);
    if (val & ESDHC_VENDOR_SPEC_VSELECT)
    ret |= SDHCI_CTRL_VDD_180;
    if (esdhc_is_usdhc(imx_data)) {
    if (imx_data.socdata.flags & ESDHC_FLAG_MAN_TUNING)
    val = readl(host.ioaddr + ESDHC_MIX_CTRL);
#[no_mangle]
pub unsafe extern "C" fn if(ESDHC_FLAG_STD_TUNING: imx_data->socdata->flags &) -> else {
    else if (imx_data.socdata.flags & ESDHC_FLAG_STD_TUNING)
// the std tuning bits is in ACMD12_ERR for imx6sl
    val = readl(host.ioaddr + SDHCI_AUTO_CMD_STATUS);
    }
    if (val & ESDHC_MIX_CTRL_EXE_TUNE)
    ret |= SDHCI_CTRL_EXEC_TUNING;
    if (val & ESDHC_MIX_CTRL_SMPCLK_SEL)
    ret |= SDHCI_CTRL_TUNED_CLK;
    ret &= ~SDHCI_CTRL_PRESET_VAL_ENABLE;
    return ret;
    }
    if (unlikely(reg == SDHCI_TRANSFER_MODE)) {
    if (esdhc_is_usdhc(imx_data)) {
    let mut m: u32 = readl(host.ioaddr + ESDHC_MIX_CTRL);
    ret = m & ESDHC_MIX_CTRL_SDHCI_MASK;
// Swap AC23 bit
    if (m & ESDHC_MIX_CTRL_AC23EN) {
    ret &= ~ESDHC_MIX_CTRL_AC23EN;
    ret |= SDHCI_TRNS_AUTO_CMD23;
    }
    } else {
    ret = readw(host.ioaddr + SDHCI_TRANSFER_MODE);
    }
    return ret;
    }
    return readw(host.ioaddr + reg);
    }
#[no_mangle]
unsafe extern "C" fn esdhc_writew_le(host: *mut sdhci_host, val: u16, reg: c_int) {
    static void esdhc_writew_le(struct sdhci_host *host, u16 val, int reg)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    let mut new_val: u32 = 0;
    switch (reg) {
    case SDHCI_CLOCK_CONTROL:
    new_val = readl(host.ioaddr + ESDHC_VENDOR_SPEC);
    if (val & SDHCI_CLOCK_CARD_EN)
    new_val |= ESDHC_VENDOR_SPEC_FRC_SDCLK_ON;
    else
    new_val &= ~ESDHC_VENDOR_SPEC_FRC_SDCLK_ON;
    writel(new_val, host.ioaddr + ESDHC_VENDOR_SPEC);
    if (!(new_val & ESDHC_VENDOR_SPEC_FRC_SDCLK_ON))
    esdhc_wait_for_card_clock_gate_off(host);
    return;
    case SDHCI_HOST_CONTROL2:
    new_val = readl(host.ioaddr + ESDHC_VENDOR_SPEC);
    if (val & SDHCI_CTRL_VDD_180)
    new_val |= ESDHC_VENDOR_SPEC_VSELECT;
    else
    new_val &= ~ESDHC_VENDOR_SPEC_VSELECT;
    writel(new_val, host.ioaddr + ESDHC_VENDOR_SPEC);
    if (imx_data.socdata.flags & ESDHC_FLAG_STD_TUNING) {
    let mut v: u32 = readl(host.ioaddr + SDHCI_AUTO_CMD_STATUS);
    if (val & SDHCI_CTRL_TUNED_CLK)
    v |= ESDHC_MIX_CTRL_SMPCLK_SEL;
    else
    v &= ~ESDHC_MIX_CTRL_SMPCLK_SEL;
    if (val & SDHCI_CTRL_EXEC_TUNING)
    v |= ESDHC_MIX_CTRL_EXE_TUNE;
    else
    v &= ~ESDHC_MIX_CTRL_EXE_TUNE;
    writel(v, host.ioaddr + SDHCI_AUTO_CMD_STATUS);
    }
    return;
    case SDHCI_TRANSFER_MODE:
    if ((imx_data.socdata.flags & ESDHC_FLAG_MULTIBLK_NO_INT)
    && (host.cmd.opcode == SD_IO_RW_EXTENDED)
    && (host.cmd.data.blocks > 1)
    && (host.cmd.data.flags & MMC_DATA_READ)) {
    u32 v;
    v = readl(host.ioaddr + ESDHC_VENDOR_SPEC);
    v |= ESDHC_VENDOR_SPEC_SDIO_QUIRK;
    writel(v, host.ioaddr + ESDHC_VENDOR_SPEC);
    }
    if (esdhc_is_usdhc(imx_data)) {
    u32 wml;
    let mut m: u32 = readl(host.ioaddr + ESDHC_MIX_CTRL);
// Swap AC23 bit
    if (val & SDHCI_TRNS_AUTO_CMD23) {
    val &= ~SDHCI_TRNS_AUTO_CMD23;
    val |= ESDHC_MIX_CTRL_AC23EN;
    }
    m = val | (m & ~ESDHC_MIX_CTRL_SDHCI_MASK);
    writel(m, host.ioaddr + ESDHC_MIX_CTRL);
// Set watermark levels for PIO access to maximum value
// (128 words) to accommodate full 512 bytes buffer.
// For DMA access restore the levels to default value.
//
    m = readl(host.ioaddr + ESDHC_WTMK_LVL);
    if (val & SDHCI_TRNS_DMA) {
    wml = ESDHC_WTMK_LVL_WML_VAL_DEF;
    } else {
    u8 ctrl;
    wml = ESDHC_WTMK_LVL_WML_VAL_MAX;
//
// Since already disable DMA mode, so also need
// to clear the DMASEL. Otherwise, for standard
// tuning, when send tuning command, usdhc will
// still prefetch the ADMA script from wrong
// DMA address, then we will see IOMMU report
// some error which show lack of TLB mapping.
//
    ctrl = sdhci_readb(host, SDHCI_HOST_CONTROL);
    ctrl &= ~SDHCI_CTRL_DMA_MASK;
    sdhci_writeb(host, ctrl, SDHCI_HOST_CONTROL);
    }
    m &= ~(ESDHC_WTMK_LVL_RD_WML_MASK |
    ESDHC_WTMK_LVL_WR_WML_MASK);
    m |= (wml << ESDHC_WTMK_LVL_RD_WML_SHIFT) |
    (wml << ESDHC_WTMK_LVL_WR_WML_SHIFT);
    writel(m, host.ioaddr + ESDHC_WTMK_LVL);
    } else {
//
// Postpone this write, we must do it together with a
// command write that is down below.
//
    imx_data.scratchpad = val;
    }
    return;
    case SDHCI_COMMAND:
    if (host.cmd.opcode == MMC_STOP_TRANSMISSION)
    val |= SDHCI_CMD_ABORTCMD;
    if ((host.cmd.opcode == MMC_SET_BLOCK_COUNT) &&
    (imx_data.socdata.flags & ESDHC_FLAG_MULTIBLK_NO_INT))
    imx_data.multiblock_status = MULTIBLK_IN_PROCESS;
    if (esdhc_is_usdhc(imx_data))
    writel(val << 16,
    host.ioaddr + SDHCI_TRANSFER_MODE);
    else
    writel(val << 16 | imx_data.scratchpad,
    host.ioaddr + SDHCI_TRANSFER_MODE);
    return;
    case SDHCI_BLOCK_SIZE:
    val &= ~SDHCI_MAKE_BLKSZ(0x7, 0);
    break;
    }
    esdhc_clrset_le(host, 0xffff, val, reg);
    }
#[no_mangle]
unsafe extern "C" fn esdhc_readb_le(host: *mut sdhci_host, reg: c_int) -> u8 {
    static u8 esdhc_readb_le(struct sdhci_host *host, int reg)
    {
    u8 ret;
    u32 val;
    switch (reg) {
    case SDHCI_HOST_CONTROL:
    val = readl(host.ioaddr + reg);
    ret = val & SDHCI_CTRL_LED;
    ret |= (val >> 5) & SDHCI_CTRL_DMA_MASK;
    ret |= (val & ESDHC_CTRL_4BITBUS);
    ret |= (val & ESDHC_CTRL_8BITBUS) << 3;
    return ret;
    }
    return readb(host.ioaddr + reg);
    }
#[no_mangle]
unsafe extern "C" fn esdhc_writeb_le(host: *mut sdhci_host, val: u8, reg: c_int) {
    static void esdhc_writeb_le(struct sdhci_host *host, u8 val, int reg)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    let mut new_val: u32 = 0;
    u32 mask;
    switch (reg) {
    case SDHCI_POWER_CONTROL:
//
// FSL put some DMA bits here
// If your board has a regulator, code should be here
//
    return;
    case SDHCI_HOST_CONTROL:
// FSL messed up here, so we need to manually compose it.
    new_val = val & SDHCI_CTRL_LED;
// ensure the endianness
    new_val |= ESDHC_HOST_CONTROL_LE;
// bits 8&9 are reserved on mx25
    if (!is_imx25_esdhc(imx_data)) {
// DMA mode bits are shifted
    new_val |= (val & SDHCI_CTRL_DMA_MASK) << 5;
    }
//
// Do not touch buswidth bits here. This is done in
// esdhc_pltfm_bus_width.
// Do not touch the D3CD bit either which is used for the
// SDIO interrupt erratum workaround.
//
    mask = 0xffff & ~(ESDHC_CTRL_BUSWIDTH_MASK | ESDHC_CTRL_D3CD);
    esdhc_clrset_le(host, mask, new_val, reg);
    return;
    case SDHCI_TIMEOUT_CONTROL:
    esdhc_clrset_le(host, ESDHC_SYS_CTRL_DTOCV_MASK,
    FIELD_PREP(ESDHC_SYS_CTRL_DTOCV_MASK, val),
    ESDHC_SYSTEM_CONTROL);
    return;
    case SDHCI_SOFTWARE_RESET:
    if (val & SDHCI_RESET_DATA)
    new_val = readl(host.ioaddr + SDHCI_HOST_CONTROL);
    break;
    }
    esdhc_clrset_le(host, 0xff, val, reg);
    if (reg == SDHCI_SOFTWARE_RESET) {
    if (val & SDHCI_RESET_ALL) {
//
// The esdhc has a design violation to SDHC spec which
// tells that software reset should not affect card
// detection circuit. But esdhc clears its SYSCTL
// register bits [0..2] during the software reset. This
// will stop those clocks that card detection circuit
// relies on. To work around it, we turn the clocks on
// back to keep card detection circuit functional.
//
    esdhc_clrset_le(host, 0x7, 0x7, ESDHC_SYSTEM_CONTROL);
//
// The reset on usdhc fails to clear MIX_CTRL register.
// Do it manually here.
//
    if (esdhc_is_usdhc(imx_data)) {
//
// the tuning bits should be kept during reset
//
    new_val = readl(host.ioaddr + ESDHC_MIX_CTRL);
    writel(new_val & ESDHC_MIX_CTRL_TUNING_MASK,
    host.ioaddr + ESDHC_MIX_CTRL);
    imx_data.is_ddr = 0;
    }
    } else if (val & SDHCI_RESET_DATA) {
//
// The eSDHC DAT line software reset clears at least the
// data transfer width on i.MX25, so make sure that the
// Host Control register is unaffected.
//
    esdhc_clrset_le(host, 0xff, new_val,
    SDHCI_HOST_CONTROL);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn esdhc_pltfm_get_max_clock(host: *mut sdhci_host) -> c_uint {
    static unsigned int esdhc_pltfm_get_max_clock(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    return pltfm_host.clock;
    }
#[no_mangle]
unsafe extern "C" fn esdhc_pltfm_get_min_clock(host: *mut sdhci_host) -> c_uint {
    static unsigned int esdhc_pltfm_get_min_clock(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    return pltfm_host.clock / 256 / 16;
    }
    static inline void esdhc_pltfm_set_clock(struct sdhci_host *host,
    unsigned int clock)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    let mut host_clock: c_uint = pltfm_host.clock;
    let mut ddr_pre_div: c_int = imx_data.is_ddr ? 2 : 1;
    let mut pre_div: c_int = 1;
    let mut div: c_int = 1;
    int ret;
    u32 temp, val;
    if (esdhc_is_usdhc(imx_data)) {
    val = readl(host.ioaddr + ESDHC_VENDOR_SPEC);
    writel(val & ~ESDHC_VENDOR_SPEC_FRC_SDCLK_ON,
    host.ioaddr + ESDHC_VENDOR_SPEC);
    esdhc_wait_for_card_clock_gate_off(host);
    }
    if (clock == 0) {
    host.mmc.actual_clock = 0;
    return;
    }
// For i.MX53 eSDHCv3, SYSCTL.SDCLKFS may not be set to 0.
    if (is_imx53_esdhc(imx_data)) {
//
// According to the i.MX53 reference manual, if DLLCTRL[10] can
// be set, then the controller is eSDHCv3, else it is eSDHCv2.
//
    val = readl(host.ioaddr + ESDHC_DLL_CTRL);
    writel(val | BIT(10), host.ioaddr + ESDHC_DLL_CTRL);
    temp = readl(host.ioaddr + ESDHC_DLL_CTRL);
    writel(val, host.ioaddr + ESDHC_DLL_CTRL);
    if (temp & BIT(10))
    pre_div = 2;
    }
    temp = sdhci_readl(host, ESDHC_SYSTEM_CONTROL);
    temp &= ~(ESDHC_CLOCK_IPGEN | ESDHC_CLOCK_HCKEN | ESDHC_CLOCK_PEREN
    | ESDHC_CLOCK_MASK);
    sdhci_writel(host, temp, ESDHC_SYSTEM_CONTROL);
    if ((imx_data.socdata.flags & ESDHC_FLAG_ERR010450) &&
    (!(host.quirks2 & SDHCI_QUIRK2_NO_1_8_V))) {
    unsigned int max_clock;
    max_clock = imx_data.is_ddr ? 45000000 : 150000000;
    clock = min(clock, max_clock);
    }
    while (host_clock / (16 * pre_div * ddr_pre_div) > clock &&
    pre_div < 256)
    pre_div *= 2;
    while (host_clock / (div * pre_div * ddr_pre_div) > clock && div < 16)
    div++;
    host.mmc.actual_clock = host_clock / (div * pre_div * ddr_pre_div);
    dev_dbg(mmc_dev(host.mmc), "desired SD clock: %d, actual: %d\n",
    clock, host.mmc.actual_clock);
    pre_div >>= 1;
    div--;
    temp = sdhci_readl(host, ESDHC_SYSTEM_CONTROL);
    temp |= (ESDHC_CLOCK_IPGEN | ESDHC_CLOCK_HCKEN | ESDHC_CLOCK_PEREN
    | (div << ESDHC_DIVIDER_SHIFT)
    | (pre_div << ESDHC_PREDIV_SHIFT));
    sdhci_writel(host, temp, ESDHC_SYSTEM_CONTROL);
// need to wait the bit 3 of the PRSSTAT to be set, make sure card clock is stable
    ret = readl_poll_timeout(host.ioaddr + ESDHC_PRSSTAT, temp,
    (temp & ESDHC_CLOCK_STABLE), 2, 100);
    if (ret == -ETIMEDOUT)
    dev_warn(mmc_dev(host.mmc), "card clock still not stable in 100us!.\n");
    if (esdhc_is_usdhc(imx_data)) {
    val = readl(host.ioaddr + ESDHC_VENDOR_SPEC);
    writel(val | ESDHC_VENDOR_SPEC_FRC_SDCLK_ON,
    host.ioaddr + ESDHC_VENDOR_SPEC);
    }
    }
#[no_mangle]
unsafe extern "C" fn esdhc_pltfm_get_ro(host: *mut sdhci_host) -> c_uint {
    static unsigned int esdhc_pltfm_get_ro(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    struct esdhc_platform_data *boarddata = &imx_data.boarddata;
    switch (boarddata.wp_type) {
    case ESDHC_WP_GPIO:
    return mmc_gpio_get_ro(host.mmc);
    case ESDHC_WP_CONTROLLER:
    return !(readl(host.ioaddr + SDHCI_PRESENT_STATE) &
    SDHCI_WRITE_PROTECT);
    case ESDHC_WP_NONE:
    break;
    }
    return -ENOSYS;
    }
#[no_mangle]
unsafe extern "C" fn esdhc_pltfm_set_bus_width(host: *mut sdhci_host, width: c_int) {
    static void esdhc_pltfm_set_bus_width(struct sdhci_host *host, int width)
    {
    u32 ctrl;
    switch (width) {
    case MMC_BUS_WIDTH_8:
    ctrl = ESDHC_CTRL_8BITBUS;
    break;
    case MMC_BUS_WIDTH_4:
    ctrl = ESDHC_CTRL_4BITBUS;
    break;
    default:
    ctrl = 0;
    break;
    }
    esdhc_clrset_le(host, ESDHC_CTRL_BUSWIDTH_MASK, ctrl,
    SDHCI_HOST_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn esdhc_reset_tuning(host: *mut sdhci_host) {
    static void esdhc_reset_tuning(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    u32 ctrl, tuning_ctrl, sys_ctrl;
    int ret;
// Reset the tuning circuit
    if (esdhc_is_usdhc(imx_data)) {
    ctrl = readl(host.ioaddr + ESDHC_MIX_CTRL);
    ctrl &= ~ESDHC_MIX_CTRL_AUTO_TUNE_EN;
    if (imx_data.socdata.flags & ESDHC_FLAG_MAN_TUNING) {
    ctrl &= ~ESDHC_MIX_CTRL_SMPCLK_SEL;
    writel(ctrl, host.ioaddr + ESDHC_MIX_CTRL);
    writel(0, host.ioaddr + ESDHC_TUNE_CTRL_STATUS);
    } else if (imx_data.socdata.flags & ESDHC_FLAG_STD_TUNING) {
    writel(ctrl, host.ioaddr + ESDHC_MIX_CTRL);
//
// enable the std tuning just in case it cleared in
// sdhc_esdhc_tuning_restore.
//
    tuning_ctrl = readl(host.ioaddr + ESDHC_TUNING_CTRL);
    if (!(tuning_ctrl & ESDHC_STD_TUNING_EN)) {
    tuning_ctrl |= ESDHC_STD_TUNING_EN;
    writel(tuning_ctrl, host.ioaddr + ESDHC_TUNING_CTRL);
    }
// set the reset tuning bit
    sys_ctrl = readl(host.ioaddr + ESDHC_SYSTEM_CONTROL);
    sys_ctrl |= ESDHC_SYS_CTRL_RESET_TUNING;
    writel(sys_ctrl, host.ioaddr + ESDHC_SYSTEM_CONTROL);
    ctrl = readl(host.ioaddr + SDHCI_AUTO_CMD_STATUS);
    ctrl &= ~ESDHC_MIX_CTRL_SMPCLK_SEL;
    ctrl &= ~ESDHC_MIX_CTRL_EXE_TUNE;
    writel(ctrl, host.ioaddr + SDHCI_AUTO_CMD_STATUS);
// Make sure ESDHC_MIX_CTRL_EXE_TUNE cleared
    ret = readl_poll_timeout(host.ioaddr + SDHCI_AUTO_CMD_STATUS,
    ctrl, !(ctrl & ESDHC_MIX_CTRL_EXE_TUNE), 1, 50);
    if (ret == -ETIMEDOUT)
    dev_warn(mmc_dev(host.mmc),
    "Warning! clear execute tuning bit failed\n");
//
// SDHCI_INT_DATA_AVAIL is W1C bit, set this bit will clear the
// usdhc IP internal logic flag execute_tuning_with_clr_buf, which
// will finally make sure the normal data transfer logic correct.
//
    ctrl = readl(host.ioaddr + SDHCI_INT_STATUS);
    ctrl |= SDHCI_INT_DATA_AVAIL;
    writel(ctrl, host.ioaddr + SDHCI_INT_STATUS);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn usdhc_init_card(mmc: *mut mmc_host, card: *mut mmc_card) {
    static void usdhc_init_card(struct mmc_host *mmc, struct mmc_card *card)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    imx_data.init_card_type = card.type;
    }
#[no_mangle]
unsafe extern "C" fn usdhc_execute_tuning(mmc: *mut mmc_host, opcode: u32) -> c_int {
    static int usdhc_execute_tuning(struct mmc_host *mmc, u32 opcode)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    int err;
//
// i.MX uSDHC internally already uses a fixed optimized timing for
// DDR50, normally does not require tuning for DDR50 mode.
//
    if (host.timing == MMC_TIMING_UHS_DDR50)
    return 0;
//
// Reset tuning circuit logic. If not, the previous tuning result
// will impact current tuning, make current tuning can't set the
// correct delay cell.
//
    esdhc_reset_tuning(host);
    err = sdhci_execute_tuning(mmc, opcode);
// If tuning done, enable auto tuning
    if (!err && !host.tuning_err)
    usdhc_auto_tuning_mode_sel_and_en(host);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn esdhc_prepare_tuning(host: *mut sdhci_host, val: u32) {
    static void esdhc_prepare_tuning(struct sdhci_host *host, u32 val)
    {
    u32 reg, sys_ctrl;
    u8 sw_rst;
    int ret;
// FIXME: delay a bit for card to be ready for next tuning due to errors
    mdelay(1);
// IC suggest to reset USDHC before every tuning command
    esdhc_clrset_le(host, 0xff, SDHCI_RESET_ALL, SDHCI_SOFTWARE_RESET);
    ret = readb_poll_timeout(host.ioaddr + SDHCI_SOFTWARE_RESET, sw_rst,
    !(sw_rst & SDHCI_RESET_ALL), 10, 100);
    if (ret == -ETIMEDOUT)
    dev_warn(mmc_dev(host.mmc),
    "warning! RESET_ALL never complete before sending tuning command\n");
    reg = readl(host.ioaddr + ESDHC_MIX_CTRL);
    reg |= ESDHC_MIX_CTRL_EXE_TUNE | ESDHC_MIX_CTRL_SMPCLK_SEL;
    writel(reg, host.ioaddr + ESDHC_MIX_CTRL);
    writel(FIELD_PREP(ESDHC_TUNE_CTRL_STATUS_DLY_CELL_SET_PRE_MASK, val),
    host.ioaddr + ESDHC_TUNE_CTRL_STATUS);
    dev_dbg(mmc_dev(host.mmc),
    "tuning with delay 0x%x ESDHC_TUNE_CTRL_STATUS 0x%x\n",
    val, readl(host.ioaddr + ESDHC_TUNE_CTRL_STATUS));
// set RST_FIFO to reset the async FIFO, and wat it to self-clear
    sys_ctrl = readl(host.ioaddr + ESDHC_SYSTEM_CONTROL);
    sys_ctrl |= ESDHC_SYS_CTRL_RST_FIFO;
    writel(sys_ctrl, host.ioaddr + ESDHC_SYSTEM_CONTROL);
    ret = readl_poll_timeout(host.ioaddr + ESDHC_SYSTEM_CONTROL, sys_ctrl,
    !(sys_ctrl & ESDHC_SYS_CTRL_RST_FIFO), 10, 100);
    if (ret == -ETIMEDOUT)
    dev_warn(mmc_dev(host.mmc),
    "warning! RST_FIFO not clear in 100us\n");
    }
#[no_mangle]
unsafe extern "C" fn esdhc_post_tuning(host: *mut sdhci_host) {
    static void esdhc_post_tuning(struct sdhci_host *host)
    {
    u32 reg;
    reg = readl(host.ioaddr + ESDHC_MIX_CTRL);
    reg &= ~ESDHC_MIX_CTRL_EXE_TUNE;
    writel(reg, host.ioaddr + ESDHC_MIX_CTRL);
    }
//
// find the largest pass window, and use the average delay of this
// largest window to get the best timing.
//
#[no_mangle]
unsafe extern "C" fn esdhc_executing_tuning(host: *mut sdhci_host, opcode: u32) -> c_int {
    static int esdhc_executing_tuning(struct sdhci_host *host, u32 opcode)
    {
    int min, max, avg, ret;
    int win_length, target_min, target_max, target_win_length;
    u32 clk_tune_ctrl_status, temp;
    min = target_min = ESDHC_TUNE_CTRL_MIN;
    max = target_max = ESDHC_TUNE_CTRL_MIN;
    target_win_length = 0;
    while (max < ESDHC_TUNE_CTRL_MAX) {
// find the mininum delay first which can pass tuning
    while (min < ESDHC_TUNE_CTRL_MAX) {
    esdhc_prepare_tuning(host, min);
    if (!mmc_send_tuning(host.mmc, opcode, core::ptr::null_mut()))
    break;
    min += ESDHC_TUNE_CTRL_STEP;
    }
// find the maxinum delay which can not pass tuning
    max = min + ESDHC_TUNE_CTRL_STEP;
    while (max < ESDHC_TUNE_CTRL_MAX) {
    esdhc_prepare_tuning(host, max);
    if (mmc_send_tuning(host.mmc, opcode, core::ptr::null_mut())) {
    max -= ESDHC_TUNE_CTRL_STEP;
    break;
    }
    max += ESDHC_TUNE_CTRL_STEP;
    }
    win_length = max - min + 1;
// get the largest pass window
    if (win_length > target_win_length) {
    target_win_length = win_length;
    target_min = min;
    target_max = max;
    }
// continue to find the next pass window
    min = max + ESDHC_TUNE_CTRL_STEP;
    }
// use average delay to get the best timing
    avg = (target_min + target_max) / 2;
    esdhc_prepare_tuning(host, avg);
//
// adjust the delay according to tuning window, make preparation
// for the auto-tuning logic. According to hardware suggest, need
// to config the auto tuning window width to 3, to make the auto
// tuning logic have enough space to handle the sample point shift
// caused by temperature change.
//
    clk_tune_ctrl_status = FIELD_PREP(ESDHC_TUNE_CTRL_STATUS_DLY_CELL_SET_PRE_MASK,
    avg - ESDHC_AUTO_TUNING_WINDOW) |
    FIELD_PREP(ESDHC_TUNE_CTRL_STATUS_DLY_CELL_SET_OUT_MASK,
    ESDHC_AUTO_TUNING_WINDOW) |
    FIELD_PREP(ESDHC_TUNE_CTRL_STATUS_DLY_CELL_SET_POST_MASK,
    ESDHC_AUTO_TUNING_WINDOW);
    writel(clk_tune_ctrl_status, host.ioaddr + ESDHC_TUNE_CTRL_STATUS);
    ret = readl_poll_timeout(host.ioaddr + ESDHC_TUNE_CTRL_STATUS, temp,
    clk_tune_ctrl_status ==
    FIELD_GET(ESDHC_TUNE_CTRL_STATUS_TAP_SEL_MASK, temp),
    1, 10);
    if (ret == -ETIMEDOUT)
    dev_warn(mmc_dev(host.mmc),
    "clock tuning control status not set in 10us\n");
    ret = mmc_send_tuning(host.mmc, opcode, core::ptr::null_mut());
    esdhc_post_tuning(host);
    dev_dbg(mmc_dev(host.mmc), "tuning %s at 0x%x ret %d\n",
    ret ? "failed" : "passed", avg, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn esdhc_hs400_enhanced_strobe(mmc: *mut mmc_host, ios: *mut mmc_ios) {
    static void esdhc_hs400_enhanced_strobe(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    u32 m;
    m = readl(host.ioaddr + ESDHC_MIX_CTRL);
    if (ios.enhanced_strobe)
    m |= ESDHC_MIX_CTRL_HS400_ES_EN;
    else
    m &= ~ESDHC_MIX_CTRL_HS400_ES_EN;
    writel(m, host.ioaddr + ESDHC_MIX_CTRL);
    }
    static int esdhc_change_pinstate(struct sdhci_host *host,
    unsigned int uhs)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    struct pinctrl_state *pinctrl;
    dev_dbg(mmc_dev(host.mmc), "change pinctrl state for uhs %d\n", uhs);
    if (IS_ERR(imx_data.pinctrl))
    return -EINVAL;
    switch (uhs) {
    case MMC_TIMING_UHS_SDR50:
    case MMC_TIMING_UHS_DDR50:
    if (IS_ERR(imx_data.pins_100mhz))
    return -EINVAL;
    pinctrl = imx_data.pins_100mhz;
    break;
    case MMC_TIMING_UHS_SDR104:
    case MMC_TIMING_MMC_HS200:
    case MMC_TIMING_MMC_HS400:
    if (IS_ERR(imx_data.pins_200mhz))
    return -EINVAL;
    pinctrl = imx_data.pins_200mhz;
    break;
    default:
// back to default state for other legacy timing
    return pinctrl_select_default_state(mmc_dev(host.mmc));
    }
    return pinctrl_select_state(imx_data.pinctrl, pinctrl);
    }
#[no_mangle]
unsafe extern "C" fn esdhc_set_dll_override(host: *mut sdhci_host) {
    static void esdhc_set_dll_override(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    struct esdhc_platform_data *boarddata = &imx_data.boarddata;
    u32 v;
    if (!boarddata.delay_line)
    return;
    v = boarddata.delay_line << ESDHC_DLL_OVERRIDE_VAL_SHIFT |
    (1 << ESDHC_DLL_OVERRIDE_EN_SHIFT);
    if (is_imx53_esdhc(imx_data))
    v <<= 1;
    writel(v, host.ioaddr + ESDHC_DLL_CTRL);
    }
//
// For HS400 eMMC, there is a data_strobe line. This signal is generated
// by the device and used for data output and CRC status response output
// in HS400 mode. The frequency of this signal follows the frequency of
// CLK generated by host. The host receives the data which is aligned to the
// edge of data_strobe line. Due to the time delay between CLK line and
// data_strobe line, if the delay time is larger than one clock cycle,
// then CLK and data_strobe line will be misaligned, read error shows up.
//
#[no_mangle]
unsafe extern "C" fn esdhc_set_strobe_dll(host: *mut sdhci_host) {
    static void esdhc_set_strobe_dll(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    u32 strobe_delay;
    u32 v;
    int ret;
// disable clock before enabling strobe dll
    writel(readl(host.ioaddr + ESDHC_VENDOR_SPEC) &
    ~ESDHC_VENDOR_SPEC_FRC_SDCLK_ON,
    host.ioaddr + ESDHC_VENDOR_SPEC);
    esdhc_wait_for_card_clock_gate_off(host);
// force a reset on strobe dll
    writel(ESDHC_STROBE_DLL_CTRL_RESET,
    host.ioaddr + ESDHC_STROBE_DLL_CTRL);
// clear the reset bit on strobe dll before any setting
    writel(0, host.ioaddr + ESDHC_STROBE_DLL_CTRL);
//
// enable strobe dll ctrl and adjust the delay target
// for the uSDHC loopback read clock
//
    if (imx_data.boarddata.strobe_dll_delay_target)
    strobe_delay = imx_data.boarddata.strobe_dll_delay_target;
    else
    strobe_delay = ESDHC_STROBE_DLL_CTRL_SLV_DLY_TARGET_DEFAULT;
    v = ESDHC_STROBE_DLL_CTRL_ENABLE |
    ESDHC_STROBE_DLL_CTRL_SLV_UPDATE_INT_DEFAULT |
    (strobe_delay << ESDHC_STROBE_DLL_CTRL_SLV_DLY_TARGET_SHIFT);
    writel(v, host.ioaddr + ESDHC_STROBE_DLL_CTRL);
// wait max 50us to get the REF/SLV lock
    ret = readl_poll_timeout(host.ioaddr + ESDHC_STROBE_DLL_STATUS, v,
    ((v & ESDHC_STROBE_DLL_STS_REF_LOCK) && (v & ESDHC_STROBE_DLL_STS_SLV_LOCK)), 1, 50);
    if (ret == -ETIMEDOUT)
    dev_warn(mmc_dev(host.mmc),
    "warning! HS400 strobe DLL status REF/SLV not lock in 50us, STROBE DLL status is %x!\n", v);
    }
#[no_mangle]
unsafe extern "C" fn esdhc_set_uhs_signaling(host: *mut sdhci_host, timing: unsigned) {
    static void esdhc_set_uhs_signaling(struct sdhci_host *host, unsigned timing)
    {
    u32 m;
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
// disable ddr mode and disable HS400 mode
    m = readl(host.ioaddr + ESDHC_MIX_CTRL);
    m &= ~(ESDHC_MIX_CTRL_DDREN | ESDHC_MIX_CTRL_HS400_EN);
    imx_data.is_ddr = 0;
    switch (timing) {
    case MMC_TIMING_UHS_SDR12:
    case MMC_TIMING_UHS_SDR25:
    case MMC_TIMING_UHS_SDR50:
    case MMC_TIMING_UHS_SDR104:
    case MMC_TIMING_MMC_HS:
    case MMC_TIMING_MMC_HS200:
    writel(m, host.ioaddr + ESDHC_MIX_CTRL);
    break;
    case MMC_TIMING_UHS_DDR50:
    case MMC_TIMING_MMC_DDR52:
    m |= ESDHC_MIX_CTRL_DDREN;
    writel(m, host.ioaddr + ESDHC_MIX_CTRL);
    imx_data.is_ddr = 1;
    esdhc_set_dll_override(host);
    break;
    case MMC_TIMING_MMC_HS400:
    m |= ESDHC_MIX_CTRL_DDREN | ESDHC_MIX_CTRL_HS400_EN;
    writel(m, host.ioaddr + ESDHC_MIX_CTRL);
    imx_data.is_ddr = 1;
// update clock after enable DDR for strobe DLL lock
    host.ops.set_clock(host, host.clock);
    esdhc_set_strobe_dll(host);
    break;
    case MMC_TIMING_LEGACY:
    default:
    esdhc_reset_tuning(host);
    break;
    }
    if (!(imx_data.socdata.flags & ESDHC_FLAG_DUMMY_PAD) &&
    (timing == MMC_TIMING_UHS_SDR104 ||
    timing == MMC_TIMING_MMC_HS200 ||
    timing == MMC_TIMING_MMC_HS400))
    m |= ESDHC_MIX_CTRL_FBCLK_SEL;
    else
    m &= ~ESDHC_MIX_CTRL_FBCLK_SEL;
    writel(m, host.ioaddr + ESDHC_MIX_CTRL);
    esdhc_change_pinstate(host, timing);
    }
#[no_mangle]
unsafe extern "C" fn esdhc_reset(host: *mut sdhci_host, mask: u8) {
    static void esdhc_reset(struct sdhci_host *host, u8 mask)
    {
    u32 present_state;
    int ret;
//
// For data or full reset, ensure any active data transfer completes
// before resetting to avoid system hang.
//
    if (mask & (SDHCI_RESET_DATA | SDHCI_RESET_ALL)) {
    ret = readl_poll_timeout_atomic(host.ioaddr + ESDHC_PRSSTAT, present_state,
    !(present_state & SDHCI_DATA_INHIBIT), 2,
    ESDHC_DATA_INHIBIT_WAIT_US);
    if (ret == -ETIMEDOUT)
    dev_warn(mmc_dev(host.mmc),
    "timeout waiting for data transfer completion\n");
    }
    sdhci_and_cqhci_reset(host, mask);
    sdhci_writel(host, host.ier, SDHCI_INT_ENABLE);
    sdhci_writel(host, host.ier, SDHCI_SIGNAL_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn esdhc_get_max_timeout_count(host: *mut sdhci_host) -> c_uint {
    static unsigned int esdhc_get_max_timeout_count(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
// Doc Erratum: the uSDHC actual maximum timeout count is 1 << 29
    return esdhc_is_usdhc(imx_data) ? 1 << 29 : 1 << 27;
    }
#[no_mangle]
unsafe extern "C" fn esdhc_cqhci_irq(host: *mut sdhci_host, intmask: u32) -> u32 {
    static u32 esdhc_cqhci_irq(struct sdhci_host *host, u32 intmask)
    {
    let mut cmd_error: c_int = 0;
    let mut data_error: c_int = 0;
    if (!sdhci_cqe_irq(host, intmask, &cmd_error, &data_error))
    return intmask;
    cqhci_irq(host.mmc, cmd_error, data_error);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn esdhc_hw_reset(host: *mut sdhci_host) {
    static void esdhc_hw_reset(struct sdhci_host *host)
    {
    esdhc_clrset_le(host, ESDHC_SYS_CTRL_IPP_RST_N, 0, ESDHC_SYSTEM_CONTROL);
// eMMC spec requires minimum 1us, here delay between 1-10us
    usleep_range(1, 10);
    esdhc_clrset_le(host, ESDHC_SYS_CTRL_IPP_RST_N,
    ESDHC_SYS_CTRL_IPP_RST_N, ESDHC_SYSTEM_CONTROL);
// eMMC spec requires minimum 200us, here delay between 200-300us
    usleep_range(200, 300);
    }
    static struct sdhci_ops sdhci_esdhc_ops = {
    .read_l = esdhc_readl_le,
    .read_w = esdhc_readw_le,
    .read_b = esdhc_readb_le,
    .write_l = esdhc_writel_le,
    .write_w = esdhc_writew_le,
    .write_b = esdhc_writeb_le,
    .set_clock = esdhc_pltfm_set_clock,
    .get_max_clock = esdhc_pltfm_get_max_clock,
    .get_min_clock = esdhc_pltfm_get_min_clock,
    .get_max_timeout_count = esdhc_get_max_timeout_count,
    .get_ro = esdhc_pltfm_get_ro,
    .set_bus_width = esdhc_pltfm_set_bus_width,
    .set_uhs_signaling = esdhc_set_uhs_signaling,
    .reset = esdhc_reset,
    .irq = esdhc_cqhci_irq,
    .dump_vendor_regs = esdhc_dump_debug_regs,
    .hw_reset = esdhc_hw_reset,
    };
    static const struct sdhci_pltfm_data sdhci_esdhc_imx_pdata = {
    .quirks = ESDHC_DEFAULT_QUIRKS | SDHCI_QUIRK_NO_HISPD_BIT
    | SDHCI_QUIRK_NO_ENDATTR_IN_NOPDESC
    | SDHCI_QUIRK_BROKEN_ADMA_ZEROLEN_DESC
    | SDHCI_QUIRK_BROKEN_CARD_DETECTION,
    .ops = &sdhci_esdhc_ops,
    };
#[no_mangle]
unsafe extern "C" fn sdhci_esdhc_imx_hwinit(host: *mut sdhci_host) {
    static void sdhci_esdhc_imx_hwinit(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    struct cqhci_host *cq_host = host.mmc.cqe_private;
    u32 tmp;
    if (esdhc_is_usdhc(imx_data)) {
//
// The imx6q ROM code will change the default watermark
// level setting to something insane.  Change it back here.
//
    writel(ESDHC_WTMK_DEFAULT_VAL, host.ioaddr + ESDHC_WTMK_LVL);
//
// ROM code will change the bit burst_length_enable setting
// to zero if this usdhc is chosen to boot system. Change
// it back here, otherwise it will impact the performance a
// lot. This bit is used to enable/disable the burst length
// for the external AHB2AXI bridge. It's useful especially
// for INCR transfer because without burst length indicator,
// the AHB2AXI bridge does not know the burst length in
// advance. And without burst length indicator, AHB INCR
// transfer can only be converted to singles on the AXI side.
//
    writel(readl(host.ioaddr + SDHCI_HOST_CONTROL)
    | ESDHC_BURST_LEN_EN_INCR,
    host.ioaddr + SDHCI_HOST_CONTROL);
//
// erratum ESDHC_FLAG_ERR004536 fix for MX6Q TO1.2 and MX6DL
// TO1.1, it's harmless for MX6SL
//
    if (!(imx_data.socdata.flags & ESDHC_FLAG_SKIP_ERR004536)) {
    writel(readl(host.ioaddr + 0x6c) & ~BIT(7),
    host.ioaddr + 0x6c);
    }
// disable DLL_CTRL delay line settings
    writel(0x0, host.ioaddr + ESDHC_DLL_CTRL);
//
// For the case of command with busy, if set the bit
// ESDHC_VEND_SPEC2_EN_BUSY_IRQ, USDHC will generate a
// transfer complete interrupt when busy is deasserted.
// When CQHCI use DCMD to send a CMD need R1b respons,
// CQHCI require to set ESDHC_VEND_SPEC2_EN_BUSY_IRQ,
// otherwise DCMD will always meet timeout waiting for
// hardware interrupt issue.
//
    if (imx_data.socdata.flags & ESDHC_FLAG_CQHCI) {
    tmp = readl(host.ioaddr + ESDHC_VEND_SPEC2);
    tmp |= ESDHC_VEND_SPEC2_EN_BUSY_IRQ;
    writel(tmp, host.ioaddr + ESDHC_VEND_SPEC2);
    host.quirks &= ~SDHCI_QUIRK_NO_BUSY_IRQ;
    }
    if (imx_data.socdata.flags & ESDHC_FLAG_STD_TUNING) {
    tmp = readl(host.ioaddr + ESDHC_TUNING_CTRL);
    tmp |= ESDHC_STD_TUNING_EN;
//
// ROM code or bootloader may config the start tap
// and step, unmask them first.
//
    tmp &= ~(ESDHC_TUNING_START_TAP_MASK | ESDHC_TUNING_STEP_MASK);
    if (imx_data.boarddata.tuning_start_tap)
    tmp |= imx_data.boarddata.tuning_start_tap;
    else
    tmp |= ESDHC_TUNING_START_TAP_DEFAULT;
    if (imx_data.boarddata.tuning_step) {
    tmp |= imx_data.boarddata.tuning_step
    << ESDHC_TUNING_STEP_SHIFT;
    } else {
    tmp |= ESDHC_TUNING_STEP_DEFAULT
    << ESDHC_TUNING_STEP_SHIFT;
    }
//
// Config the tuning window to the hardware suggested value 3.
// This tuning window is used for auto tuning logic. The default
// tuning window is 2, here change to 3 make the window a bit
// wider, give auto tuning enough space to handle the sample
// point shift cause by temperature change.
//
    tmp &= ~ESDHC_TUNING_WINDOW_MASK;
    tmp |= FIELD_PREP(ESDHC_TUNING_WINDOW_MASK, ESDHC_AUTO_TUNING_WINDOW);
// Disable the CMD CRC check for tuning, if not, need to
// add some delay after every tuning command, because
// hardware standard tuning logic will directly go to next
// step once it detect the CMD CRC error, will not wait for
// the card side to finally send out the tuning data, trigger
// the buffer read ready interrupt immediately. If usdhc send
// the next tuning command some eMMC card will stuck, can't
// response, block the tuning procedure or the first command
// after the whole tuning procedure always can't get any response.
//
    tmp |= ESDHC_TUNING_CMD_CRC_CHECK_DISABLE;
    writel(tmp, host.ioaddr + ESDHC_TUNING_CTRL);
    } else if (imx_data.socdata.flags & ESDHC_FLAG_MAN_TUNING) {
//
// ESDHC_STD_TUNING_EN may be configured in bootloader
// or ROM code, so clear this bit here to make sure
// the manual tuning can work.
//
    tmp = readl(host.ioaddr + ESDHC_TUNING_CTRL);
    tmp &= ~ESDHC_STD_TUNING_EN;
    writel(tmp, host.ioaddr + ESDHC_TUNING_CTRL);
    }
//
// On i.MX8MM, we are running Dual Linux OS, with 1st Linux using SD Card
// as rootfs storage, 2nd Linux using eMMC as rootfs storage. We let
// the 1st linux configure power/clock for the 2nd Linux.
//
// When the 2nd Linux is booting into rootfs stage, we let the 1st Linux
// to destroy the 2nd linux, then restart the 2nd linux, we met SDHCI dump.
// After we clear the pending interrupt and halt CQCTL, issue gone.
//
    if (cq_host) {
    tmp = cqhci_readl(cq_host, CQHCI_IS);
    cqhci_writel(cq_host, tmp, CQHCI_IS);
    cqhci_writel(cq_host, CQHCI_HALT, CQHCI_CTL);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn sdhc_esdhc_tuning_save(host: *mut sdhci_host) {
    static void sdhc_esdhc_tuning_save(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    u32 reg;
//
// SD/eMMC do not need this tuning save because it will re-init
// after system resume back.
// Here save the tuning delay value for SDIO device since it may
// keep power during system PM. And for usdhc, only SDR50 and
// SDR104 mode for SDIO device need to do tuning, and need to
// save/restore.
//
    if (host.timing == MMC_TIMING_UHS_SDR50 ||
    host.timing == MMC_TIMING_UHS_SDR104) {
    reg = readl(host.ioaddr + ESDHC_TUNE_CTRL_STATUS);
    reg = FIELD_GET(ESDHC_TUNE_CTRL_STATUS_TAP_SEL_PRE_MASK, reg);
    imx_data.boarddata.saved_tuning_delay_cell = reg;
    }
    }
#[no_mangle]
unsafe extern "C" fn sdhc_esdhc_tuning_restore(host: *mut sdhci_host) {
    static void sdhc_esdhc_tuning_restore(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    u32 reg;
    if (host.timing == MMC_TIMING_UHS_SDR50 ||
    host.timing == MMC_TIMING_UHS_SDR104) {
//
// restore the tuning delay value actually is a
// manual tuning method, so clear the standard
// tuning enable bit here. Will set back this
// ESDHC_STD_TUNING_EN in esdhc_reset_tuning()
// when trigger re-tuning.
//
    reg = readl(host.ioaddr + ESDHC_TUNING_CTRL);
    reg &= ~ESDHC_STD_TUNING_EN;
    writel(reg, host.ioaddr + ESDHC_TUNING_CTRL);
    reg = readl(host.ioaddr + ESDHC_MIX_CTRL);
    reg |= ESDHC_MIX_CTRL_SMPCLK_SEL;
    if (!(imx_data.socdata.flags & ESDHC_FLAG_DUMMY_PAD))
    reg |= ESDHC_MIX_CTRL_FBCLK_SEL;
    writel(reg, host.ioaddr + ESDHC_MIX_CTRL);
    writel(FIELD_PREP(ESDHC_TUNE_CTRL_STATUS_DLY_CELL_SET_PRE_MASK,
    imx_data.boarddata.saved_tuning_delay_cell) |
    FIELD_PREP(ESDHC_TUNE_CTRL_STATUS_DLY_CELL_SET_OUT_MASK,
    ESDHC_AUTO_TUNING_WINDOW) |
    FIELD_PREP(ESDHC_TUNE_CTRL_STATUS_DLY_CELL_SET_POST_MASK,
    ESDHC_AUTO_TUNING_WINDOW),
    host.ioaddr + ESDHC_TUNE_CTRL_STATUS);
    }
    }
#[no_mangle]
unsafe extern "C" fn esdhc_cqe_enable(mmc: *mut mmc_host) {
    static void esdhc_cqe_enable(struct mmc_host *mmc)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    struct cqhci_host *cq_host = mmc.cqe_private;
    u32 reg;
    u16 mode;
    let mut count: c_int = 10;
//
// CQE gets stuck if it sees Buffer Read Enable bit set, which can be
// the case after tuning, so ensure the buffer is drained.
//
    reg = sdhci_readl(host, SDHCI_PRESENT_STATE);
    while (reg & SDHCI_DATA_AVAILABLE) {
    sdhci_readl(host, SDHCI_BUFFER);
    reg = sdhci_readl(host, SDHCI_PRESENT_STATE);
    if (count-- == 0) {
    dev_warn(mmc_dev(host.mmc),
    "CQE may get stuck because the Buffer Read Enable bit is set\n");
    break;
    }
    mdelay(1);
    }
//
// Runtime resume will reset the entire host controller, which
// will also clear the DMAEN/BCEN of register ESDHC_MIX_CTRL.
// Here set DMAEN and BCEN when enable CMDQ.
//
    mode = sdhci_readw(host, SDHCI_TRANSFER_MODE);
    if (host.flags & SDHCI_REQ_USE_DMA)
    mode |= SDHCI_TRNS_DMA;
    if (!(host.quirks2 & SDHCI_QUIRK2_SUPPORT_SINGLE))
    mode |= SDHCI_TRNS_BLK_CNT_EN;
    sdhci_writew(host, mode, SDHCI_TRANSFER_MODE);
//
// Though Runtime resume reset the entire host controller,
// but do not impact the CQHCI side, need to clear the
// HALT bit, avoid CQHCI stuck in the first request when
// system resume back.
//
    cqhci_writel(cq_host, 0, CQHCI_CTL);
    if (cqhci_readl(cq_host, CQHCI_CTL) & CQHCI_HALT)
    dev_err(mmc_dev(host.mmc),
    "failed to exit halt state when enable CQE\n");
    sdhci_cqe_enable(mmc);
    }
#[no_mangle]
unsafe extern "C" fn esdhc_sdhci_dumpregs(mmc: *mut mmc_host) {
    static void esdhc_sdhci_dumpregs(struct mmc_host *mmc)
    {
    sdhci_dumpregs(mmc_priv(mmc));
    }
    static const struct cqhci_host_ops esdhc_cqhci_ops = {
    .enable		= esdhc_cqe_enable,
    .disable	= sdhci_cqe_disable,
    .dumpregs	= esdhc_sdhci_dumpregs,
    };
    static int
    sdhci_esdhc_imx_probe_dt(struct platform_device *pdev,
    struct sdhci_host *host,
    struct pltfm_imx_data *imx_data)
    {
    struct device_node *np = pdev.dev.of_node;
    struct esdhc_platform_data *boarddata = &imx_data.boarddata;
    int ret;
    if (of_property_read_bool(np, "fsl,wp-controller"))
    boarddata.wp_type = ESDHC_WP_CONTROLLER;
//
// If we have this property, then activate WP check.
// Retrieving and requesting the actual WP GPIO will happen
// in the call to mmc_of_parse().
//
    if (of_property_present(np, "wp-gpios"))
    boarddata.wp_type = ESDHC_WP_GPIO;
    of_property_read_u32(np, "fsl,tuning-step", &boarddata.tuning_step);
    of_property_read_u32(np, "fsl,tuning-start-tap",
    &boarddata.tuning_start_tap);
    of_property_read_u32(np, "fsl,strobe-dll-delay-target",
    &boarddata.strobe_dll_delay_target);
    if (of_property_read_u32(np, "fsl,delay-line", &boarddata.delay_line))
    boarddata.delay_line = 0;
    mmc_of_parse_voltage(host.mmc, &host.ocr_mask);
    if (esdhc_is_usdhc(imx_data) && !IS_ERR(imx_data.pinctrl)) {
    imx_data.pins_100mhz = pinctrl_lookup_state(imx_data.pinctrl,
    ESDHC_PINCTRL_STATE_100MHZ);
    imx_data.pins_200mhz = pinctrl_lookup_state(imx_data.pinctrl,
    ESDHC_PINCTRL_STATE_200MHZ);
    }
// call to generic mmc_of_parse to support additional capabilities
    ret = mmc_of_parse(host.mmc);
    if (ret)
    return ret;
    sdhci_get_property(pdev);
    if (mmc_gpio_get_cd(host.mmc) >= 0)
    host.quirks &= ~SDHCI_QUIRK_BROKEN_CARD_DETECTION;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_esdhc_imx_probe(pdev: *mut platform_device) -> c_int {
    static int sdhci_esdhc_imx_probe(struct platform_device *pdev)
    {
    struct sdhci_pltfm_host *pltfm_host;
    struct sdhci_host *host;
    struct cqhci_host *cq_host;
    int err;
    struct pltfm_imx_data *imx_data;
    host = sdhci_pltfm_init(pdev, &sdhci_esdhc_imx_pdata,
    sizeof(*imx_data));
    if (IS_ERR(host))
    return PTR_ERR(host);
    pltfm_host = sdhci_priv(host);
    imx_data = sdhci_pltfm_priv(pltfm_host);
    imx_data.socdata = device_get_match_data(&pdev.dev);
    host.quirks |= imx_data.socdata.quirks;
    if (imx_data.socdata.flags & ESDHC_FLAG_PMQOS)
    cpu_latency_qos_add_request(&imx_data.pm_qos_req, 0);
    imx_data.clk_ipg = devm_clk_get(&pdev.dev, "ipg");
    if (IS_ERR(imx_data.clk_ipg)) {
    err = PTR_ERR(imx_data.clk_ipg);
    goto free_sdhci;
    }
    imx_data.clk_ahb = devm_clk_get(&pdev.dev, "ahb");
    if (IS_ERR(imx_data.clk_ahb)) {
    err = PTR_ERR(imx_data.clk_ahb);
    goto free_sdhci;
    }
    imx_data.clk_per = devm_clk_get(&pdev.dev, "per");
    if (IS_ERR(imx_data.clk_per)) {
    err = PTR_ERR(imx_data.clk_per);
    goto free_sdhci;
    }
    pltfm_host.clk = imx_data.clk_per;
    err = clk_prepare_enable(imx_data.clk_per);
    if (err)
    goto free_sdhci;
    err = clk_prepare_enable(imx_data.clk_ipg);
    if (err)
    goto disable_per_clk;
    err = clk_prepare_enable(imx_data.clk_ahb);
    if (err)
    goto disable_ipg_clk;
    pltfm_host.clock = clk_get_rate(pltfm_host.clk);
    if (!pltfm_host.clock) {
    dev_err(mmc_dev(host.mmc), "could not get clk rate\n");
    err = -EINVAL;
    goto disable_ahb_clk;
    }
    imx_data.pinctrl = devm_pinctrl_get(&pdev.dev);
    if (IS_ERR(imx_data.pinctrl))
    dev_warn(mmc_dev(host.mmc), "could not get pinctrl\n");
    if (esdhc_is_usdhc(imx_data)) {
    host.quirks2 |= SDHCI_QUIRK2_PRESET_VALUE_BROKEN;
    host.mmc.caps |= MMC_CAP_1_8V_DDR | MMC_CAP_3_3V_DDR;
// GPIO CD can be set as a wakeup source
    if (!(imx_data.socdata.flags & ESDHC_FLAG_SKIP_CD_WAKE))
    host.mmc.caps |= MMC_CAP_CD_WAKE;
    if (!(imx_data.socdata.flags & ESDHC_FLAG_HS200))
    host.quirks2 |= SDHCI_QUIRK2_BROKEN_HS200;
// clear tuning bits in case ROM has set it already
    writel(0x0, host.ioaddr + ESDHC_MIX_CTRL);
    writel(0x0, host.ioaddr + SDHCI_AUTO_CMD_STATUS);
    writel(0x0, host.ioaddr + ESDHC_TUNE_CTRL_STATUS);
//
// Link usdhc specific mmc_host_ops execute_tuning function,
// to replace the standard one in sdhci_ops.
//
    host.mmc_host_ops.execute_tuning = usdhc_execute_tuning;
//
// Link usdhc specific mmc_host_ops init card function,
// to distinguish the card type.
//
    host.mmc_host_ops.init_card = usdhc_init_card;
    host.max_timeout_count = 0xF;
    }
    if (imx_data.socdata.flags & ESDHC_FLAG_MAN_TUNING)
    sdhci_esdhc_ops.platform_execute_tuning =
    esdhc_executing_tuning;
    if (imx_data.socdata.flags & ESDHC_FLAG_ERR004536)
    host.quirks |= SDHCI_QUIRK_BROKEN_ADMA;
    if (imx_data.socdata.flags & ESDHC_FLAG_HS400)
    host.mmc.caps2 |= MMC_CAP2_HS400;
    if (imx_data.socdata.flags & ESDHC_FLAG_BROKEN_AUTO_CMD23)
    host.quirks2 |= SDHCI_QUIRK2_ACMD23_BROKEN;
    if (imx_data.socdata.flags & ESDHC_FLAG_HS400_ES) {
    host.mmc.caps2 |= MMC_CAP2_HS400_ES;
    host.mmc_host_ops.hs400_enhanced_strobe =
    esdhc_hs400_enhanced_strobe;
    }
    if (imx_data.socdata.flags & ESDHC_FLAG_CQHCI) {
    host.mmc.caps2 |= MMC_CAP2_CQE | MMC_CAP2_CQE_DCMD;
    cq_host = devm_kzalloc(&pdev.dev, sizeof(*cq_host), GFP_KERNEL);
    if (!cq_host) {
    err = -ENOMEM;
    goto disable_ahb_clk;
    }
    cq_host.mmio = host.ioaddr + ESDHC_CQHCI_ADDR_OFFSET;
    cq_host.ops = &esdhc_cqhci_ops;
    err = cqhci_init(cq_host, host.mmc, false);
    if (err)
    goto disable_ahb_clk;
    }
    err = sdhci_esdhc_imx_probe_dt(pdev, host, imx_data);
    if (err)
    goto disable_ahb_clk;
    sdhci_esdhc_imx_hwinit(host);
    err = sdhci_add_host(host);
    if (err)
    goto disable_ahb_clk;
//
// Setup the wakeup capability here, let user to decide
// whether need to enable this wakeup through sysfs interface.
//
    if ((host.mmc.pm_caps & MMC_PM_KEEP_POWER) &&
    (host.mmc.pm_caps & MMC_PM_WAKE_SDIO_IRQ))
    device_set_wakeup_capable(&pdev.dev, true);
    pm_runtime_set_active(&pdev.dev);
    pm_runtime_set_autosuspend_delay(&pdev.dev, 50);
    pm_runtime_use_autosuspend(&pdev.dev);
    pm_suspend_ignore_children(&pdev.dev, 1);
    pm_runtime_enable(&pdev.dev);
    return 0;
    disable_ahb_clk:
    clk_disable_unprepare(imx_data.clk_ahb);
    disable_ipg_clk:
    clk_disable_unprepare(imx_data.clk_ipg);
    disable_per_clk:
    clk_disable_unprepare(imx_data.clk_per);
    free_sdhci:
    if (imx_data.socdata.flags & ESDHC_FLAG_PMQOS)
    cpu_latency_qos_remove_request(&imx_data.pm_qos_req);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_esdhc_imx_remove(pdev: *mut platform_device) {
    static void sdhci_esdhc_imx_remove(struct platform_device *pdev)
    {
    struct sdhci_host *host = platform_get_drvdata(pdev);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    int dead;
    pm_runtime_get_sync(&pdev.dev);
    dead = (readl(host.ioaddr + SDHCI_INT_STATUS) == 0xffffffff);
    pm_runtime_disable(&pdev.dev);
    pm_runtime_put_noidle(&pdev.dev);
    sdhci_remove_host(host, dead);
    clk_disable_unprepare(imx_data.clk_per);
    clk_disable_unprepare(imx_data.clk_ipg);
    clk_disable_unprepare(imx_data.clk_ahb);
    if (imx_data.socdata.flags & ESDHC_FLAG_PMQOS)
    cpu_latency_qos_remove_request(&imx_data.pm_qos_req);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_esdhc_suspend(dev: *mut device) -> c_int {
    static int sdhci_esdhc_suspend(struct device *dev)
    {
    struct sdhci_host *host = dev_get_drvdata(dev);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    int ret;
//
// Switch to runtime resume for two reasons:
// 1, there is register access (e.g., wakeup control register), so
// need to make sure gate on ipg clock.
// 2, make sure the pm_runtime_force_resume() in sdhci_esdhc_resume() really
// invoke its ->runtime_resume callback (needs_force_resume = 1).
//
    ret = pm_runtime_resume_and_get(dev);
    if (ret)
    return ret;
    if ((imx_data.socdata.flags & ESDHC_FLAG_STATE_LOST_IN_LPMODE) &&
    (host.tuning_mode != SDHCI_TUNING_MODE_1)) {
    mmc_retune_timer_stop(host.mmc);
    mmc_retune_needed(host.mmc);
    }
//
// For the device need to keep power during system PM, need
// to save the tuning delay value just in case the usdhc
// lost power during system PM.
//
    if (mmc_card_keep_power(host.mmc) && esdhc_is_usdhc(imx_data))
    sdhc_esdhc_tuning_save(host);
// The irqs of imx are not shared. It is safe to disable
    disable_irq(host.irq);
    if (device_may_wakeup(dev)) {
    if (!sdhci_enable_irq_wakeups(host))
    dev_warn(dev, "Failed to enable irq wakeup\n");
    } else {
//
// For the device which works as wakeup source, no need
// to change the pinctrl to sleep state.
// e.g. For SDIO device, the interrupt share with data pin,
// but the pinctrl sleep state may config the data pin to
// other function like GPIO function to save power in PM,
// which finally block the SDIO wakeup function.
//
    if (pinctrl_pm_select_sleep_state(dev))
    dev_warn(dev, "Failed to select sleep pinctrl state\n");
    }
    if (mmc_gpio_set_cd_wake(host.mmc, true))
    dev_warn(dev, "Failed to enable cd wake\n");
//
// Make sure invoke runtime_suspend to gate off clock.
// uSDHC IP supports in-band SDIO wakeup even without clock.
//
    pm_runtime_force_suspend(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_esdhc_resume(dev: *mut device) -> c_int {
    static int sdhci_esdhc_resume(struct device *dev)
    {
    struct sdhci_host *host = dev_get_drvdata(dev);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    int ret;
    if (!device_may_wakeup(dev)) {
    ret = esdhc_change_pinstate(host, host.timing);
    if (ret)
    dev_warn(dev, "Failed to restore pinctrl state\n");
    }
    ret = pm_runtime_force_resume(dev);
    if (ret)
    return ret;
    mmc_gpio_set_cd_wake(host.mmc, false);
// re-initialize hw state in case it's lost in low power mode
    sdhci_esdhc_imx_hwinit(host);
    if (host.irq_wake_enabled)
    sdhci_disable_irq_wakeups(host);
    enable_irq(host.irq);
//
// restore the saved tuning delay value for the device which keep
// power during system PM.
//
    if (mmc_card_keep_power(host.mmc) && esdhc_is_usdhc(imx_data)) {
    sdhc_esdhc_tuning_restore(host);
//
// Restore DLL override for DDR modes. hwinit unconditionally
// clears ESDHC_DLL_CTRL, but the card is still in DDR mode.
//
    if (host.timing == MMC_TIMING_UHS_DDR50 ||
    host.timing == MMC_TIMING_MMC_DDR52)
    esdhc_set_dll_override(host);
    }
    pm_runtime_put_autosuspend(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_esdhc_runtime_suspend(dev: *mut device) -> c_int {
    static int sdhci_esdhc_runtime_suspend(struct device *dev)
    {
    struct sdhci_host *host = dev_get_drvdata(dev);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    int ret;
    if (host.mmc.caps2 & MMC_CAP2_CQE) {
    ret = cqhci_suspend(host.mmc);
    if (ret)
    return ret;
    }
    sdhci_runtime_suspend_host(host);
    if (host.tuning_mode != SDHCI_TUNING_MODE_3)
    mmc_retune_needed(host.mmc);
    imx_data.actual_clock = host.mmc.actual_clock;
    esdhc_pltfm_set_clock(host, 0);
    clk_disable_unprepare(imx_data.clk_per);
    clk_disable_unprepare(imx_data.clk_ipg);
    clk_disable_unprepare(imx_data.clk_ahb);
    if (imx_data.socdata.flags & ESDHC_FLAG_PMQOS)
    cpu_latency_qos_remove_request(&imx_data.pm_qos_req);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_esdhc_runtime_resume(dev: *mut device) -> c_int {
    static int sdhci_esdhc_runtime_resume(struct device *dev)
    {
    struct sdhci_host *host = dev_get_drvdata(dev);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct pltfm_imx_data *imx_data = sdhci_pltfm_priv(pltfm_host);
    int err;
    if (imx_data.socdata.flags & ESDHC_FLAG_PMQOS)
    cpu_latency_qos_add_request(&imx_data.pm_qos_req, 0);
    if (imx_data.socdata.flags & ESDHC_FLAG_CLK_RATE_LOST_IN_PM_RUNTIME)
    clk_set_rate(imx_data.clk_per, pltfm_host.clock);
    err = clk_prepare_enable(imx_data.clk_ahb);
    if (err)
    goto remove_pm_qos_request;
    err = clk_prepare_enable(imx_data.clk_per);
    if (err)
    goto disable_ahb_clk;
    err = clk_prepare_enable(imx_data.clk_ipg);
    if (err)
    goto disable_per_clk;
    esdhc_pltfm_set_clock(host, imx_data.actual_clock);
    sdhci_runtime_resume_host(host, 0);
    if (host.mmc.caps2 & MMC_CAP2_CQE)
    err = cqhci_resume(host.mmc);
    return err;
    disable_per_clk:
    clk_disable_unprepare(imx_data.clk_per);
    disable_ahb_clk:
    clk_disable_unprepare(imx_data.clk_ahb);
    remove_pm_qos_request:
    if (imx_data.socdata.flags & ESDHC_FLAG_PMQOS)
    cpu_latency_qos_remove_request(&imx_data.pm_qos_req);
    return err;
    }
    static const struct dev_pm_ops sdhci_esdhc_pmops = {
    SYSTEM_SLEEP_PM_OPS(sdhci_esdhc_suspend, sdhci_esdhc_resume)
    RUNTIME_PM_OPS(sdhci_esdhc_runtime_suspend, sdhci_esdhc_runtime_resume, core::ptr::null_mut())
    };
    static struct platform_driver sdhci_esdhc_imx_driver = {
    .driver		= {
    .name	= "sdhci-esdhc-imx",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = imx_esdhc_dt_ids,
    .pm	= pm_ptr(&sdhci_esdhc_pmops),
    },
    .probe		= sdhci_esdhc_imx_probe,
    .remove		= sdhci_esdhc_imx_remove,
    };
    module_platform_driver(sdhci_esdhc_imx_driver);
    MODULE_DESCRIPTION("SDHCI driver for Freescale i.MX eSDHC");
    MODULE_AUTHOR("Wolfram Sang <kernel@pengutronix.de>");
    MODULE_LICENSE("GPL v2");
