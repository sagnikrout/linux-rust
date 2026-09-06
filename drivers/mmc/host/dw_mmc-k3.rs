//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/dw_mmc-k3.c
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
// Copyright (c) 2013 Linaro Ltd.
// Copyright (c) 2013 HiSilicon Limited.
//

//
// hi6220 sd only support io voltage 1.8v and 3v
// Also need config AO_SCTRL_SEL18 accordingly
//

pub const AO_SCTRL_CTRL3: c_uint = 0x40C;
pub const DWMMC_SDIO_ID: c_int = 2;

pub const TIMING_MODE: c_int = 3;
pub const TIMING_CFG_NUM: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct k3_priv {
    pub cur_speed: u32,
    pub reg: *mut regmap,
}

    static unsigned long dw_mci_hi6220_caps[] = {
    MMC_CAP_CMD23,
    MMC_CAP_CMD23,
    0
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hs_timing {
    pub drv_phase: u32,
    pub smpl_dly: u32,
    pub smpl_phase_max: u32,
    pub smpl_phase_min: u32,
}

    static struct hs_timing hs_timing_cfg[TIMING_MODE][TIMING_CFG_NUM] = {
    { /* reserved */ },
    { /* SD */
    {7, 0, 15, 15,},  /* 0: LEGACY 400k */
    {6, 0,  4,  4,},  /* 1: MMC_HS */
    {6, 0,  3,  3,},  /* 2: SD_HS */
    {6, 0, 15, 15,},  /* 3: SDR12 */
    {6, 0,  2,  2,},  /* 4: SDR25 */
    {4, 0, 11,  0,},  /* 5: SDR50 */
    {6, 4, 15,  0,},  /* 6: SDR104 */
    {0},              /* 7: DDR50 */
    {0},              /* 8: DDR52 */
    {0},              /* 9: HS200 */
    },
    { /* SDIO */
    {7, 0, 15, 15,},  /* 0: LEGACY 400k */
    {0},              /* 1: MMC_HS */
    {6, 0, 15, 15,},  /* 2: SD_HS */
    {6, 0, 15, 15,},  /* 3: SDR12 */
    {6, 0,  0,  0,},  /* 4: SDR25 */
    {4, 0, 12,  0,},  /* 5: SDR50 */
    {5, 4, 15,  0,},  /* 6: SDR104 */
    {0},              /* 7: DDR50 */
    {0},              /* 8: DDR52 */
    {0},              /* 9: HS200 */
    }
    };
#[no_mangle]
unsafe extern "C" fn dw_mci_k3_set_ios(host: *mut dw_mci, ios: *mut mmc_ios) {
    static void dw_mci_k3_set_ios(struct dw_mci *host, struct mmc_ios *ios)
    {
    int ret;
    ret = clk_set_rate(host.ciu_clk, ios.clock);
    if (ret)
    dev_warn(host.dev, "failed to set rate %uHz\n", ios.clock);
    host.bus_hz = clk_get_rate(host.ciu_clk);
    }
    static const struct dw_mci_drv_data k3_drv_data = {
    .set_ios		= dw_mci_k3_set_ios,
    };
#[no_mangle]
unsafe extern "C" fn dw_mci_hi6220_parse_dt(host: *mut dw_mci) -> c_int {
    static int dw_mci_hi6220_parse_dt(struct dw_mci *host)
    {
    struct k3_priv *priv;
    priv = devm_kzalloc(host.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.reg = syscon_regmap_lookup_by_phandle(host.dev.of_node,
    "hisilicon,peripheral-syscon");
    if (IS_ERR(priv.reg))
    priv.reg = core::ptr::null_mut();
    host.priv = priv;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_mci_hi6220_switch_voltage(host: *mut dw_mci, ios: *mut mmc_ios) -> c_int {
    static int dw_mci_hi6220_switch_voltage(struct dw_mci *host, struct mmc_ios *ios)
    {
    struct k3_priv *priv;
    struct mmc_host *mmc = host.mmc;
    int min_uv, max_uv;
    int ret;
    priv = host.priv;
    if (!priv || !priv.reg)
    return 0;
    if (ios.signal_voltage == MMC_SIGNAL_VOLTAGE_330) {
    ret = regmap_update_bits(priv.reg, AO_SCTRL_CTRL3,
    AO_SCTRL_SEL18, 0);
    min_uv = 3000000;
    max_uv = 3000000;
    } else if (ios.signal_voltage == MMC_SIGNAL_VOLTAGE_180) {
    ret = regmap_update_bits(priv.reg, AO_SCTRL_CTRL3,
    AO_SCTRL_SEL18, AO_SCTRL_SEL18);
    min_uv = 1800000;
    max_uv = 1800000;
    } else {
    dev_dbg(host.dev, "voltage not supported\n");
    return -EINVAL;
    }
    if (ret) {
    dev_dbg(host.dev, "switch voltage failed\n");
    return ret;
    }
    if (IS_ERR_OR_NULL(mmc.supply.vqmmc))
    return 0;
    ret = regulator_set_voltage(mmc.supply.vqmmc, min_uv, max_uv);
    if (ret) {
    dev_dbg(host.dev, "Regulator set error %d: %d - %d\n",
    ret, min_uv, max_uv);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_mci_hi6220_set_ios(host: *mut dw_mci, ios: *mut mmc_ios) {
    static void dw_mci_hi6220_set_ios(struct dw_mci *host, struct mmc_ios *ios)
    {
    int ret;
    unsigned int clock;
    clock = (ios.clock <= 25000000) ? 25000000 : ios.clock;
    ret = clk_set_rate(host.biu_clk, clock);
    if (ret)
    dev_warn(host.dev, "failed to set rate %uHz\n", clock);
    host.bus_hz = clk_get_rate(host.biu_clk);
    }
#[no_mangle]
unsafe extern "C" fn dw_mci_hi6220_execute_tuning(host: *mut dw_mci, opcode: u32) -> c_int {
    static int dw_mci_hi6220_execute_tuning(struct dw_mci *host, u32 opcode)
    {
    return 0;
    }
    static const struct dw_mci_drv_data hi6220_data = {
    .caps			= dw_mci_hi6220_caps,
    .num_caps		= ARRAY_SIZE(dw_mci_hi6220_caps),
    .switch_voltage		= dw_mci_hi6220_switch_voltage,
    .set_ios		= dw_mci_hi6220_set_ios,
    .parse_dt		= dw_mci_hi6220_parse_dt,
    .execute_tuning		= dw_mci_hi6220_execute_tuning,
    };
    static int dw_mci_hs_set_timing(struct dw_mci *host, int timing,
    int smpl_phase)
    {
    u32 drv_phase;
    u32 smpl_dly;
    let mut use_smpl_dly: u32 = 0;
    let mut enable_shift: u32 = 0;
    u32 reg_value;
    int ctrl_id;
    ctrl_id = host.mmc.index;
    if (ctrl_id >= TIMING_MODE)
    return -EINVAL;
    drv_phase = hs_timing_cfg[ctrl_id][timing].drv_phase;
    smpl_dly   = hs_timing_cfg[ctrl_id][timing].smpl_dly;
    if (smpl_phase == -1)
    smpl_phase = (hs_timing_cfg[ctrl_id][timing].smpl_phase_max +
    hs_timing_cfg[ctrl_id][timing].smpl_phase_min) / 2;
    switch (timing) {
    case MMC_TIMING_UHS_SDR104:
    if (smpl_phase >= USE_DLY_MIN_SMPL &&
    smpl_phase <= USE_DLY_MAX_SMPL)
    use_smpl_dly = 1;
    fallthrough;
    case MMC_TIMING_UHS_SDR50:
    if (smpl_phase >= ENABLE_SHIFT_MIN_SMPL &&
    smpl_phase <= ENABLE_SHIFT_MAX_SMPL)
    enable_shift = 1;
    break;
    }
    mci_writel(host, GPIO, 0x0);
    usleep_range(5, 10);
    reg_value = FIELD_PREP(UHS_REG_EXT_SAMPLE_PHASE_MASK, smpl_phase) |
    FIELD_PREP(UHS_REG_EXT_SAMPLE_DLY_MASK, smpl_dly) |
    FIELD_PREP(UHS_REG_EXT_SAMPLE_DRVPHASE_MASK, drv_phase);
    mci_writel(host, UHS_REG_EXT, reg_value);
    mci_writel(host, ENABLE_SHIFT, enable_shift);
    reg_value = FIELD_PREP(GPIO_CLK_DIV_MASK, GENCLK_DIV) |
    FIELD_PREP(GPIO_USE_SAMPLE_DLY_MASK, use_smpl_dly);
    mci_writel(host, GPIO, (unsigned int)reg_value | GPIO_CLK_ENABLE);
// We should delay 1ms wait for timing setting finished.
    usleep_range(1000, 2000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_mci_hi3660_init(host: *mut dw_mci) -> c_int {
    static int dw_mci_hi3660_init(struct dw_mci *host)
    {
    mci_writel(host, CDTHRCTL, SDMMC_SET_THLD(SDCARD_RD_THRESHOLD,
    SDMMC_CARD_RD_THR_EN));
    host.bus_hz /= (GENCLK_DIV + 1);
    return dw_mci_hs_set_timing(host, MMC_TIMING_LEGACY, -1);
    }
#[no_mangle]
unsafe extern "C" fn dw_mci_set_sel18(host: *mut dw_mci, set: bool) -> c_int {
    static int dw_mci_set_sel18(struct dw_mci *host, bool set)
    {
    int ret;
    unsigned int val;
    struct k3_priv *priv;
    priv = host.priv;
    val = set ? SDCARD_IO_SEL18 : 0;
    ret = regmap_update_bits(priv.reg, SOC_SCTRL_SCPERCTRL5,
    SDCARD_IO_SEL18, val);
    if (ret) {
    dev_err(host.dev, "sel18 %u error\n", val);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_mci_hi3660_set_ios(host: *mut dw_mci, ios: *mut mmc_ios) {
    static void dw_mci_hi3660_set_ios(struct dw_mci *host, struct mmc_ios *ios)
    {
    int ret;
    unsigned long wanted;
    unsigned long actual;
    struct k3_priv *priv = host.priv;
    if (!ios.clock || ios.clock == priv.cur_speed)
    return;
    wanted = ios.clock * (GENCLK_DIV + 1);
    ret = clk_set_rate(host.ciu_clk, wanted);
    if (ret) {
    dev_err(host.dev, "failed to set rate %luHz\n", wanted);
    return;
    }
    actual = clk_get_rate(host.ciu_clk);
    dw_mci_hs_set_timing(host, ios.timing, -1);
    host.bus_hz = actual / (GENCLK_DIV + 1);
    host.current_speed = 0;
    priv.cur_speed = host.bus_hz;
    }
#[no_mangle]
unsafe extern "C" fn dw_mci_get_best_clksmpl(sample_flag: c_uint) -> c_int {
    static int dw_mci_get_best_clksmpl(unsigned int sample_flag)
    {
    int i;
    int interval;
    unsigned int v;
    unsigned int len;
    let mut range_start: c_uint = 0;
    let mut range_length: c_uint = 0;
    let mut middle_range: c_uint = 0;
    if (!sample_flag)
    return -EIO;
    if (~sample_flag == 0)
    return 0;
    i = ffs(sample_flag) - 1;
//
// A clock cycle is divided into 32 phases,
// each of which is represented by a bit,
// finding the optimal phase.
//
    while (i < 32) {
    v = ror32(sample_flag, i);
    len = ffs(~v) - 1;
    if (len > range_length) {
    range_length = len;
    range_start = i;
    }
    interval = ffs(v >> len) - 1;
    if (interval < 0)
    break;
    i += len + interval;
    }
    middle_range = range_start + range_length / 2;
    if (middle_range >= 32)
    middle_range %= 32;
    return middle_range;
    }
#[no_mangle]
unsafe extern "C" fn dw_mci_hi3660_execute_tuning(host: *mut dw_mci, opcode: u32) -> c_int {
    static int dw_mci_hi3660_execute_tuning(struct dw_mci *host, u32 opcode)
    {
    let mut i: c_int = 0;
    struct mmc_host *mmc = host.mmc;
    let mut smpl_phase: c_int = 0;
    let mut tuning_sample_flag: u32 = 0;
    let mut best_clksmpl: c_int = 0;
    for (i = 0; i < NUM_PHASES; ++i, ++smpl_phase) {
    smpl_phase %= 32;
    mci_writel(host, TMOUT, ~0);
    dw_mci_hs_set_timing(host, mmc.ios.timing, smpl_phase);
    if (!mmc_send_tuning(mmc, opcode, core::ptr::null_mut()))
    tuning_sample_flag |= (1 << smpl_phase);
    else
    tuning_sample_flag &= ~(1 << smpl_phase);
    }
    best_clksmpl = dw_mci_get_best_clksmpl(tuning_sample_flag);
    if (best_clksmpl < 0) {
    dev_err(host.dev, "All phases bad!\n");
    return -EIO;
    }
    dw_mci_hs_set_timing(host, mmc.ios.timing, best_clksmpl);
    dev_info(host.dev, "tuning ok best_clksmpl %u tuning_sample_flag %x\n",
    best_clksmpl, tuning_sample_flag);
    return 0;
    }
    static int dw_mci_hi3660_switch_voltage(struct dw_mci *host,
    struct mmc_ios *ios)
    {
    struct k3_priv *priv;
    struct mmc_host *mmc = host.mmc;
    let mut ret: c_int = 0;
    priv = host.priv;
    if (!priv || !priv.reg)
    return 0;
    if (mmc.index == DWMMC_SDIO_ID)
    return 0;
    if (ios.signal_voltage == MMC_SIGNAL_VOLTAGE_330)
    ret = dw_mci_set_sel18(host, 0);
#[no_mangle]
pub unsafe extern "C" fn if(MMC_SIGNAL_VOLTAGE_180: ios->signal_voltage ==) -> else {
    else if (ios.signal_voltage == MMC_SIGNAL_VOLTAGE_180)
    ret = dw_mci_set_sel18(host, 1);
    if (ret)
    return ret;
    if (!IS_ERR(mmc.supply.vqmmc)) {
    ret = mmc_regulator_set_vqmmc(mmc, ios);
    if (ret < 0) {
    dev_err(host.dev, "Regulator set error %d\n", ret);
    return ret;
    }
    }
    return 0;
    }
    static const struct dw_mci_drv_data hi3660_data = {
    .init = dw_mci_hi3660_init,
    .set_ios = dw_mci_hi3660_set_ios,
    .parse_dt = dw_mci_hi6220_parse_dt,
    .execute_tuning = dw_mci_hi3660_execute_tuning,
    .switch_voltage  = dw_mci_hi3660_switch_voltage,
    };
    static const struct of_device_id dw_mci_k3_match[] = {
    { .compatible = "hisilicon,hi3660-dw-mshc", .data = &hi3660_data, },
    { .compatible = "hisilicon,hi4511-dw-mshc", .data = &k3_drv_data, },
    { .compatible = "hisilicon,hi6220-dw-mshc", .data = &hi6220_data, },
    {},
    };
    MODULE_DEVICE_TABLE(of, dw_mci_k3_match);
#[no_mangle]
unsafe extern "C" fn dw_mci_k3_probe(pdev: *mut platform_device) -> c_int {
    static int dw_mci_k3_probe(struct platform_device *pdev)
    {
    const struct dw_mci_drv_data *drv_data;
    const struct of_device_id *match;
    match = of_match_node(dw_mci_k3_match, pdev.dev.of_node);
    drv_data = match.data;
    return dw_mci_pltfm_register(pdev, drv_data);
    }
    static struct platform_driver dw_mci_k3_pltfm_driver = {
    .probe		= dw_mci_k3_probe,
    .remove		= dw_mci_pltfm_remove,
    .driver		= {
    .name		= "dwmmc_k3",
    .probe_type	= PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table	= dw_mci_k3_match,
    .pm		= pm_ptr(&dw_mci_pmops),
    },
    };
    module_platform_driver(dw_mci_k3_pltfm_driver);
    MODULE_DESCRIPTION("K3 Specific DW-MSHC Driver Extension");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:dwmmc_k3");
