//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/wsa884x.c
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
// Copyright (c) 2015-2021, The Linux Foundation. All rights reserved.
// Copyright (c) 2023, Linaro Ltd.
//

pub const WSA884X_BASE: c_uint = 0x3000;

pub const WSA884X_BOP2_PROG_BOP2_VTH_MASK: c_uint = 0xf0;
pub const WSA884X_BOP2_PROG_BOP2_VTH_SHIFT: c_int = 4;
pub const WSA884X_BOP2_PROG_BOP2_HYST_MASK: c_uint = 0x0f;
pub const WSA884X_BOP2_PROG_BOP2_HYST_SHIFT: c_int = 0;

pub const WSA884X_REF_CTRL_BG_RDY_SEL_MASK: c_uint = 0x03;
pub const WSA884X_REF_CTRL_BG_RDY_SEL_SHIFT: c_int = 0;

pub const WSA884X_VSENSE1_GAIN_VSENSE_FE_MASK: c_uint = 0xe0;
pub const WSA884X_VSENSE1_GAIN_VSENSE_FE_SHIFT: c_int = 5;

pub const WSA884X_ISENSE2_ISENSE_GAIN_CTL_MASK: c_uint = 0xe0;
pub const WSA884X_ISENSE2_ISENSE_GAIN_CTL_SHIFT: c_int = 5;

pub const WSA884X_TOP_CTRL1_OCP_LOWVBAT_ITH_EN_MASK: c_uint = 0x01;

pub const WSA884X_PWM_CLK_CTL_VCMO_INT1_IDLE_MODE_OVRT_MASK: c_uint = 0x80;
pub const WSA884X_PWM_CLK_CTL_VCMO_INT1_IDLE_MODE_OVRT_SHIFT: c_int = 7;
pub const WSA884X_PWM_CLK_CTL_REG_MCLK_DIV_RATIO_MASK: c_uint = 0x40;
pub const WSA884X_PWM_CLK_CTL_REG_MCLK_DIV_RATIO_SHIFT: c_int = 6;
pub const WSA884X_PWM_CLK_CTL_PWM_DEGLITCH_CLK_DELAY_CTRL_MASK: c_uint = 0x30;
pub const WSA884X_PWM_CLK_CTL_PWM_DEGLITCH_CLK_DELAY_CTRL_SHIFT: c_int = 4;
pub const WSA884X_PWM_CLK_CTL_PWM_CLK_FREQ_SEL_MASK: c_uint = 0x08;
pub const WSA884X_PWM_CLK_CTL_PWM_CLK_FREQ_SEL_SHIFT: c_int = 3;
pub const WSA884X_PWM_CLK_CTL_PWM_CLK_DIV_RATIO_MASK: c_uint = 0x06;
pub const WSA884X_PWM_CLK_CTL_PWM_CLK_DIV_RATIO_SHIFT: c_int = 1;
pub const WSA884X_PWM_CLK_CTL_PWM_CLK_DIV_BYPASS_MASK: c_uint = 0x01;
pub const WSA884X_PWM_CLK_CTL_PWM_CLK_DIV_BYPASS_SHIFT: c_int = 0;

pub const WSA884X_STB_CTRL1_SLOPE_COMP_CURRENT_MASK: c_uint = 0xf8;
pub const WSA884X_STB_CTRL1_SLOPE_COMP_CURRENT_SHIFT: c_int = 3;
pub const WSA884X_STB_CTRL1_VOUT_FS_MASK: c_uint = 0x07;
pub const WSA884X_STB_CTRL1_VOUT_FS_SHIFT: c_int = 0;

pub const WSA884X_CURRENT_LIMIT_CURRENT_LIMIT_OVRD_EN_MASK: c_uint = 0x80;
pub const WSA884X_CURRENT_LIMIT_CURRENT_LIMIT_OVRD_EN_SHIFT: c_int = 7;
pub const WSA884X_CURRENT_LIMIT_CURRENT_LIMIT_MASK: c_uint = 0x7c;
pub const WSA884X_CURRENT_LIMIT_CURRENT_LIMIT_SHIFT: c_int = 2;
pub const WSA884X_CURRENT_LIMIT_CLK_PHASE_SHIFT: c_int = 0;

pub const WSA884X_ZX_CTRL1_ZX_DET_EN_MASK: c_uint = 0x80;
pub const WSA884X_ZX_CTRL1_ZX_DET_EN_SHIFT: c_int = 7;
pub const WSA884X_ZX_CTRL1_ZX_DET_SW_EN_MASK: c_uint = 0x40;
pub const WSA884X_ZX_CTRL1_ZX_DET_SW_EN_SHIFT: c_int = 6;
pub const WSA884X_ZX_CTRL1_ZX_DET_STAGE_DEFAULT_MASK: c_uint = 0x20;
pub const WSA884X_ZX_CTRL1_ZX_DET_STAGE_DEFAULT_SHIFT: c_int = 5;
pub const WSA884X_ZX_CTRL1_ZX_DET_SW_SEL_MASK: c_uint = 0x18;
pub const WSA884X_ZX_CTRL1_ZX_DET_SW_SEL_SHIFT: c_int = 3;
pub const WSA884X_ZX_CTRL1_ZX_BYP_MASK_IGNORE_MASK: c_uint = 0x04;
pub const WSA884X_ZX_CTRL1_ZX_BYP_MASK_IGNORE_SHIFT: c_int = 2;
pub const WSA884X_ZX_CTRL1_ZX_BYP_MASK_DEL_MASK: c_uint = 0x02;
pub const WSA884X_ZX_CTRL1_ZX_BYP_MASK_DEL_SHIFT: c_int = 1;
pub const WSA884X_ZX_CTRL1_BOOTCAP_REFRESH_DIS_MASK: c_uint = 0x01;
pub const WSA884X_ZX_CTRL1_BOOTCAP_REFRESH_DIS_SHIFT: c_int = 0;

pub const WSA884X_ILIM_CTRL1_EN_AUTO_MAXD_SEL_MASK: c_uint = 0x80;
pub const WSA884X_ILIM_CTRL1_EN_AUTO_MAXD_SEL_SHIFT: c_uint = 0x07;
pub const WSA884X_ILIM_CTRL1_EN_ILIM_SW_CLH_MASK: c_uint = 0x40;
pub const WSA884X_ILIM_CTRL1_EN_ILIM_SW_CLH_SHIFT: c_uint = 0x06;
pub const WSA884X_ILIM_CTRL1_ILIM_OFFSET_CLH_MASK: c_uint = 0x38;
pub const WSA884X_ILIM_CTRL1_ILIM_OFFSET_CLH_SHIFT: c_uint = 0x03;
pub const WSA884X_ILIM_CTRL1_ILIM_OFFSET_PB_MASK: c_uint = 0x07;
pub const WSA884X_ILIM_CTRL1_ILIM_OFFSET_PB_SHIFT: c_uint = 0x00;

pub const WSA884X_CKWD_CTL_1_VPP_SW_CTL_MASK: c_uint = 0x20;
pub const WSA884X_CKWD_CTL_1_VPP_SW_CTL_SHIFT: c_int = 5;
pub const WSA884X_CKWD_CTL_1_CKWD_VCOMP_VREF_SEL_MASK: c_uint = 0x1f;
pub const WSA884X_CKWD_CTL_1_CKWD_VCOMP_VREF_SEL_SHIFT: c_int = 0;

pub const WSA884X_CDC_PATH_MODE_RXD_MODE_MASK: c_uint = 0x02;
pub const WSA884X_CDC_PATH_MODE_RXD_MODE_SHIFT: c_int = 0;
pub const WSA884X_CDC_PATH_MODE_TXD_MODE_MASK: c_uint = 0x01;
pub const WSA884X_CDC_PATH_MODE_TXD_MODE_SHIFT: c_int = 0;

pub const WSA884X_PA_FSM_EN_GLOBAL_PA_EN_MASK: c_uint = 0x01;
pub const WSA884X_PA_FSM_EN_GLOBAL_PA_EN_SHIFT: c_int = 0;

pub const WSA884X_PA_FSM_CTL1_NOISE_GATE_BLOCK_MASK: c_uint = 0x38;

pub const WSA884X_PA_FSM_BYP0_DC_CAL_EN_MASK: c_uint = 0x01;
pub const WSA884X_PA_FSM_BYP0_DC_CAL_EN_SHIFT: c_int = 0;
pub const WSA884X_PA_FSM_BYP0_CLK_WD_EN_MASK: c_uint = 0x02;
pub const WSA884X_PA_FSM_BYP0_CLK_WD_EN_SHIFT: c_int = 1;
pub const WSA884X_PA_FSM_BYP0_BG_EN_MASK: c_uint = 0x04;
pub const WSA884X_PA_FSM_BYP0_BG_EN_SHIFT: c_int = 2;
pub const WSA884X_PA_FSM_BYP0_BOOST_EN_MASK: c_uint = 0x08;
pub const WSA884X_PA_FSM_BYP0_BOOST_EN_SHIFT: c_int = 3;
pub const WSA884X_PA_FSM_BYP0_PA_EN_MASK: c_uint = 0x10;
pub const WSA884X_PA_FSM_BYP0_PA_EN_SHIFT: c_int = 4;
pub const WSA884X_PA_FSM_BYP0_D_UNMUTE_MASK: c_uint = 0x20;
pub const WSA884X_PA_FSM_BYP0_D_UNMUTE_SHIFT: c_int = 5;
pub const WSA884X_PA_FSM_BYP0_SPKR_PROT_EN_MASK: c_uint = 0x40;
pub const WSA884X_PA_FSM_BYP0_SPKR_PROT_EN_SHIFT: c_int = 6;
pub const WSA884X_PA_FSM_BYP0_TSADC_EN_MASK: c_uint = 0x80;
pub const WSA884X_PA_FSM_BYP0_TSADC_EN_SHIFT: c_int = 7;

pub const WSA884X_TADC_VALUE_CTL_TEMP_VALUE_RD_EN_MASK: c_uint = 0x01;
pub const WSA884X_TADC_VALUE_CTL_TEMP_VALUE_RD_EN_SHIFT: c_int = 0;
pub const WSA884X_TADC_VALUE_CTL_VBAT_VALUE_RD_EN_MASK: c_uint = 0x02;
pub const WSA884X_TADC_VALUE_CTL_VBAT_VALUE_RD_EN_SHIFT: c_int = 1;

pub const WSA884X_VBAT_THRM_FLT_CTL_THRM_COEF_SEL_MASK: c_uint = 0xe0;
pub const WSA884X_VBAT_THRM_FLT_CTL_THRM_COEF_SEL_SHIFT: c_int = 5;
pub const WSA884X_VBAT_THRM_FLT_CTL_THRM_FLT_EN_SHIFT: c_int = 4;
pub const WSA884X_VBAT_THRM_FLT_CTL_VBAT_COEF_SEL_MASK: c_uint = 0x0e;
pub const WSA884X_VBAT_THRM_FLT_CTL_VBAT_COEF_SEL_SHIFT: c_int = 1;
pub const WSA884X_VBAT_THRM_FLT_CTL_VBAT_FLT_EN_SHIFT: c_int = 0;

pub const WSA884X_VBAT_CAL_CTL_RESERVE_MASK: c_uint = 0x0e;
pub const WSA884X_VBAT_CAL_CTL_VBAT_CAL_EN_MASK: c_uint = 0x01;

pub const WSA884X_BOP_DEGLITCH_CTL_BOP_DEGLITCH_SETTING_MASK: c_uint = 0x1e;
pub const WSA884X_BOP_DEGLITCH_CTL_BOP_DEGLITCH_SETTING_SHIFT: c_int = 1;
pub const WSA884X_BOP_DEGLITCH_CTL_BOP_DEGLITCH_EN_MASK: c_uint = 0x1;
pub const WSA884X_BOP_DEGLITCH_CTL_BOP_DEGLITCH_EN_SHIFT: c_int = 0;

pub const WSA884X_CDC_SPK_DSM_C_0_COEF_C3_MASK: c_uint = 0xf0;
pub const WSA884X_CDC_SPK_DSM_C_0_COEF_C3_SHIFT: c_int = 4;
pub const WSA884X_CDC_SPK_DSM_C_0_COEF_C2_MASK: c_uint = 0x0f;
pub const WSA884X_CDC_SPK_DSM_C_0_COEF_C2_SHIFT: c_int = 0;

pub const WSA884X_CDC_SPK_DSM_C_2_COEF_C7_MASK: c_uint = 0xf0;
pub const WSA884X_CDC_SPK_DSM_C_2_COEF_C7_SHIFT: c_int = 4;
pub const WSA884X_CDC_SPK_DSM_C_2_COEF_C6_MASK: c_uint = 0x0f;
pub const WSA884X_CDC_SPK_DSM_C_2_COEF_C6_SHIFT: c_int = 0;

pub const WSA884X_CDC_SPK_DSM_C_3_COEF_C7_MASK: c_uint = 0x3f;
pub const WSA884X_CDC_SPK_DSM_C_3_COEF_C7_SHIFT: c_int = 0;

pub const WSA884X_PDM_WD_CTL_HOLD_OFF_MASK: c_uint = 0x04;
pub const WSA884X_PDM_WD_CTL_HOLD_OFF_SHIFT: c_int = 2;
pub const WSA884X_PDM_WD_CTL_TIME_OUT_SEL_MASK: c_uint = 0x02;
pub const WSA884X_PDM_WD_CTL_TIME_OUT_SEL_SHIFT: c_int = 1;
pub const WSA884X_PDM_WD_CTL_PDM_WD_EN_MASK: c_uint = 0x01;
pub const WSA884X_PDM_WD_CTL_PDM_WD_EN_SHIFT: c_int = 0;

pub const WSA884X_DRE_CTL_0_PROG_DELAY_MASK: c_uint = 0xf0;
pub const WSA884X_DRE_CTL_0_PROG_DELAY_SHIFT: c_int = 4;
pub const WSA884X_DRE_CTL_0_OFFSET_MASK: c_uint = 0x07;
pub const WSA884X_DRE_CTL_0_OFFSET_SHIFT: c_int = 0;

pub const WSA884X_DRE_CTL_1_CSR_GAIN_MASK: c_uint = 0x3e;
pub const WSA884X_DRE_CTL_1_CSR_GAIN_SHIFT: c_int = 1;
pub const WSA884X_DRE_CTL_1_CSR_GAIN_EN_MASK: c_uint = 0x01;
pub const WSA884X_DRE_CTL_1_CSR_GAIN_EN_SHIFT: c_int = 0;

pub const WSA884X_GAIN_RAMPING_MIN_MIN_GAIN_MASK: c_uint = 0x1f;
pub const WSA884X_GAIN_RAMPING_MIN_MIN_GAIN_SHIFT: c_int = 0;

pub const WSA884X_CLSH_CTL_0_CSR_GAIN_EN_SHIFT: c_int = 7;
pub const WSA884X_CLSH_CTL_0_DLY_CODE_MASK: c_uint = 0x70;
pub const WSA884X_CLSH_CTL_0_DLY_CODE_SHIFT: c_int = 4;
pub const WSA884X_CLSH_CTL_0_DLY_RST_SHIFT: c_int = 3;
pub const WSA884X_CLSH_CTL_0_DLY_EN_SHIFT: c_int = 2;
pub const WSA884X_CLSH_CTL_0_INPUT_EN_SHIFT: c_int = 1;
pub const WSA884X_CLSH_CTL_0_CLSH_EN_SHIFT: c_int = 0;

pub const WSA884X_PBR_MAX_VOLTAGE: c_int = 20;
pub const WSA884X_PBR_MAX_CODE: c_int = 255;

    ((vth) != 0 ? (((vth) - 150) * WSA884X_PBR_MAX_CODE / (WSA884X_PBR_MAX_VOLTAGE * 100) + 1) : 0)

pub const WSA884X_ANA_WO_CTL_0_MODE_SHIFT: c_int = 0;
pub const WSA884X_ANA_WO_CTL_0_VPHX_SYS_EN_MASK: c_uint = 0xc0;
pub const WSA884X_ANA_WO_CTL_0_PA_AUX_DISABLE: c_uint = 0x0;
pub const WSA884X_ANA_WO_CTL_0_PA_AUX_18_DB: c_uint = 0xa;
pub const WSA884X_ANA_WO_CTL_0_PA_AUX_0_DB: c_uint = 0x7;
pub const WSA884X_ANA_WO_CTL_0_PA_AUX_GAIN_MASK: c_uint = 0x3c;
pub const WSA884X_ANA_WO_CTL_0_PA_MIN_GAIN_BYP_MASK: c_uint = 0x02;
pub const WSA884X_ANA_WO_CTL_0_DAC_CM_CLAMP_EN_MODE_SPEAKER: c_uint = 0x1;
pub const WSA884X_ANA_WO_CTL_0_DAC_CM_CLAMP_EN_MASK: c_uint = 0x01;

pub const WSA884X_OTP_ID_WSA8840: c_uint = 0x0;
pub const WSA884X_OTP_ID_WSA8845: c_uint = 0x5;
pub const WSA884X_OTP_ID_WSA8845H: c_uint = 0xc;
pub const WSA884X_OTP_REG_0_ID_MASK: c_uint = 0x0f;

pub const WSA884X_OTP_REG_38_RESERVER_MASK: c_uint = 0xf0;
pub const WSA884X_OTP_REG_38_RESERVER_SHIFT: c_int = 4;
pub const WSA884X_OTP_REG_38_BST_CFG_SEL_MASK: c_uint = 0x08;
pub const WSA884X_OTP_REG_38_BST_CFG_SEL_SHIFT: c_int = 3;
pub const WSA884X_OTP_REG_38_BOOST_ILIM_TUNE_MASK: c_uint = 0x07;
pub const WSA884X_OTP_REG_38_BOOST_ILIM_TUNE_SHIFT: c_int = 0;

pub const WSA884X_OTP_REG_40_SPARE_TYPE2_MASK: c_uint = 0xc0;
pub const WSA884X_OTP_REG_40_SPARE_TYPE2_SHIFT: c_int = 6;
pub const WSA884X_OTP_REG_40_ISENSE_RESCAL_MASK: c_uint = 0x3c;
pub const WSA884X_OTP_REG_40_ISENSE_RESCAL_SHIFT: c_int = 2;
pub const WSA884X_OTP_REG_40_ATE_BOOST_RDSON_TEST_MASK: c_uint = 0x2;
pub const WSA884X_OTP_REG_40_ATE_BOOST_RDSON_TEST_SHIFT: c_int = 1;
pub const WSA884X_OTP_REG_40_ATE_CLASSD_RDSON_TEST_MASK: c_uint = 0x1;
pub const WSA884X_OTP_REG_40_ATE_CLASSD_RDSON_TEST_SHIFT: c_int = 0;

pub const WSA884X_SUPPLIES_NUM: c_int = 2;
pub const WSA884X_MAX_SWR_PORTS: c_int = 6;

    SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000 |\
    SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000 |\
    SNDRV_PCM_RATE_384000)
// Fractional Rates

    SNDRV_PCM_RATE_176400 | SNDRV_PCM_RATE_352800)

    SNDRV_PCM_FMTBIT_S24_LE |\
    SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S32_LE)
// Two-point trimming for temperature calibration

//
// Device will report senseless data in many cases, so discard any measurements
// outside of valid range.
//
pub const WSA884X_LOW_TEMP_THRESHOLD: c_int = 5;
pub const WSA884X_HIGH_TEMP_THRESHOLD: c_int = 45;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsa884x_priv {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub supplies: [regulator_bulk_data; WSA884X_SUPPLIES_NUM],
    pub slave: *mut sdw_slave,
    pub sconfig: sdw_stream_config,
    pub sruntime: *mut sdw_stream_runtime,
    pub port_config: [sdw_port_config; WSA884X_MAX_SWR_PORTS],
    pub sd_n: *mut gpio_desc,
    pub sd_reset: *mut reset_control,
    pub port_prepared: [bool; WSA884X_MAX_SWR_PORTS],
    pub port_enable: [bool; WSA884X_MAX_SWR_PORTS],
    pub active_ports: c_int,
    pub dev_mode: c_int,
    pub hw_init: bool,
//
// Protects temperature reading code (related to speaker protection) and
// fields: temperature and pa_on.
//
    pub sp_lock: mutex,
    pub temperature: c_uint,
    pub pa_on: bool,
}

    enum {
    COMP_OFFSET0,
    COMP_OFFSET1,
    COMP_OFFSET2,
    COMP_OFFSET3,
    COMP_OFFSET4,
    };
    enum wsa884x_gain {
    G_21_DB = 0,
    G_19P5_DB,
    G_18_DB,
    G_16P5_DB,
    G_15_DB,
    G_13P5_DB,
    G_12_DB,
    G_10P5_DB,
    G_9_DB,
    G_7P5_DB,
    G_6_DB,
    G_4P5_DB,
    G_3_DB,
    G_1P5_DB,
    G_0_DB,
    G_M1P5_DB,
    G_M3_DB,
    G_M4P5_DB,
    G_M6_DB,
    G_MAX_DB,
    };
    enum wsa884x_isense {
    ISENSE_6_DB = 0,
    ISENSE_12_DB,
    ISENSE_15_DB,
    ISENSE_18_DB,
    };
    enum wsa884x_vsense {
    VSENSE_M12_DB = 0,
    VSENSE_M15_DB,
    VSENSE_M18_DB,
    VSENSE_M21_DB,
    VSENSE_M24_DB,
    };
    enum wsa884x_port_ids {
    WSA884X_PORT_DAC,
    WSA884X_PORT_COMP,
    WSA884X_PORT_BOOST,
    WSA884X_PORT_PBR,
    WSA884X_PORT_VISENSE,
    WSA884X_PORT_CPS,
    };
    static const char * const wsa884x_supply_name[] = {
    "vdd-io",
    "vdd-1p8",
    };
    static const char * const wsa884x_dev_mode_text[] = {
    "Speaker", "Receiver"
    };
    enum wsa884x_mode {
    WSA884X_SPEAKER,
    WSA884X_RECEIVER,
    };
    static const struct soc_enum wsa884x_dev_mode_enum =
    SOC_ENUM_SINGLE_EXT(ARRAY_SIZE(wsa884x_dev_mode_text), wsa884x_dev_mode_text);
    static struct sdw_dpn_prop wsa884x_sink_dpn_prop[WSA884X_MAX_SWR_PORTS] = {
    [WSA884X_PORT_DAC] = {
    .num = WSA884X_PORT_DAC + 1,
    .type = SDW_DPN_SIMPLE,
    .min_ch = 1,
    .max_ch = 1,
    .simple_ch_prep_sm = true,
    .read_only_wordlength = true,
    },
    [WSA884X_PORT_COMP] = {
    .num = WSA884X_PORT_COMP + 1,
    .type = SDW_DPN_SIMPLE,
    .min_ch = 1,
    .max_ch = 1,
    .simple_ch_prep_sm = true,
    .read_only_wordlength = true,
    },
    [WSA884X_PORT_BOOST] = {
    .num = WSA884X_PORT_BOOST + 1,
    .type = SDW_DPN_SIMPLE,
    .min_ch = 1,
    .max_ch = 1,
    .simple_ch_prep_sm = true,
    .read_only_wordlength = true,
    },
    [WSA884X_PORT_PBR] = {
    .num = WSA884X_PORT_PBR + 1,
    .type = SDW_DPN_SIMPLE,
    .min_ch = 1,
    .max_ch = 1,
    .simple_ch_prep_sm = true,
    .read_only_wordlength = true,
    },
    [WSA884X_PORT_VISENSE] = {
    .num = WSA884X_PORT_VISENSE + 1,
    .type = SDW_DPN_SIMPLE,
    .min_ch = 1,
    .max_ch = 1,
    .simple_ch_prep_sm = true,
    .read_only_wordlength = true,
    },
    [WSA884X_PORT_CPS] = {
    .num = WSA884X_PORT_CPS + 1,
    .type = SDW_DPN_SIMPLE,
    .min_ch = 1,
    .max_ch = 1,
    .simple_ch_prep_sm = true,
    .read_only_wordlength = true,
    }
    };
    static const struct sdw_port_config wsa884x_pconfig[WSA884X_MAX_SWR_PORTS] = {
    [WSA884X_PORT_DAC] = {
    .num = WSA884X_PORT_DAC + 1,
    .ch_mask = 0x1,
    },
    [WSA884X_PORT_COMP] = {
    .num = WSA884X_PORT_COMP + 1,
    .ch_mask = 0xf,
    },
    [WSA884X_PORT_BOOST] = {
    .num = WSA884X_PORT_BOOST + 1,
    .ch_mask = 0x3,
    },
    [WSA884X_PORT_PBR] = {
    .num = WSA884X_PORT_PBR + 1,
    .ch_mask = 0x1,
    },
    [WSA884X_PORT_VISENSE] = {
    .num = WSA884X_PORT_VISENSE + 1,
    .ch_mask = 0x1,
    },
    [WSA884X_PORT_CPS] = {
    .num = WSA884X_PORT_CPS + 1,
    .ch_mask = 0x3,
    },
    };
    static const struct reg_default wsa884x_defaults[] = {
    { WSA884X_BG_CTRL,			0xa5 },
    { WSA884X_ADC_CTRL,			0x00 },
    { WSA884X_BOP1_PROG,			0x22 },
    { WSA884X_BOP2_PROG,			0x44 },
    { WSA884X_UVLO_PROG,			0x99 },
    { WSA884X_UVLO_PROG1,			0x70 },
    { WSA884X_SPARE_CTRL_0,			0x00 },
    { WSA884X_SPARE_CTRL_1,			0x00 },
    { WSA884X_SPARE_CTRL_2,			0x00 },
    { WSA884X_SPARE_CTRL_3,			0x00 },
    { WSA884X_REF_CTRL,			0xd2 },
    { WSA884X_BG_TEST_CTL,			0x06 },
    { WSA884X_BG_BIAS,			0xd7 },
    { WSA884X_ADC_PROG,			0x08 },
    { WSA884X_ADC_IREF_CTL,			0x57 },
    { WSA884X_ADC_ISENS_CTL,		0x47 },
    { WSA884X_ADC_CLK_CTL,			0x87 },
    { WSA884X_ADC_TEST_CTL,			0x00 },
    { WSA884X_ADC_BIAS,			0x51 },
    { WSA884X_VBAT_SNS,			0xa0 },
    { WSA884X_BOP_ATEST_SEL,		0x00 },
    { WSA884X_MISC0,			0x04 },
    { WSA884X_MISC1,			0x75 },
    { WSA884X_MISC2,			0x00 },
    { WSA884X_MISC3,			0x10 },
    { WSA884X_SPARE_TSBG_0,			0x00 },
    { WSA884X_SPARE_TUNE_0,			0x00 },
    { WSA884X_SPARE_TUNE_1,			0x00 },
    { WSA884X_VSENSE1,			0xe7 },
    { WSA884X_ISENSE2,			0x27 },
    { WSA884X_SPARE_CTL_1,			0x00 },
    { WSA884X_SPARE_CTL_2,			0x00 },
    { WSA884X_SPARE_CTL_3,			0x00 },
    { WSA884X_SPARE_CTL_4,			0x00 },
    { WSA884X_EN,				0x10 },
    { WSA884X_OVERRIDE1,			0x00 },
    { WSA884X_OVERRIDE2,			0x08 },
    { WSA884X_ISENSE1,			0xd4 },
    { WSA884X_ISENSE_CAL,			0x00 },
    { WSA884X_MISC,				0x00 },
    { WSA884X_ADC_0,			0x00 },
    { WSA884X_ADC_1,			0x00 },
    { WSA884X_ADC_2,			0x40 },
    { WSA884X_ADC_3,			0x80 },
    { WSA884X_ADC_4,			0x25 },
    { WSA884X_ADC_5,			0x24 },
    { WSA884X_ADC_6,			0x0a },
    { WSA884X_ADC_7,			0x81 },
    { WSA884X_IVSENSE_SPARE_TUNE_1,		0x00 },
    { WSA884X_SPARE_TUNE_2,			0x00 },
    { WSA884X_SPARE_TUNE_3,			0x00 },
    { WSA884X_SPARE_TUNE_4,			0x00 },
    { WSA884X_TOP_CTRL1,			0xd3 },
    { WSA884X_CLIP_DET_CTRL1,		0x7e },
    { WSA884X_CLIP_DET_CTRL2,		0x4c },
    { WSA884X_DAC_CTRL1,			0xa4 },
    { WSA884X_DAC_VCM_CTRL_REG1,		0x02 },
    { WSA884X_DAC_VCM_CTRL_REG2,		0x00 },
    { WSA884X_DAC_VCM_CTRL_REG3,		0x00 },
    { WSA884X_DAC_VCM_CTRL_REG4,		0x00 },
    { WSA884X_DAC_VCM_CTRL_REG5,		0x00 },
    { WSA884X_DAC_VCM_CTRL_REG6,		0x00 },
    { WSA884X_PWM_CLK_CTL,			0x20 },
    { WSA884X_DRV_LF_LDO_SEL,		0xaa },
    { WSA884X_OCP_CTL,			0xc6 },
    { WSA884X_PDRV_HS_CTL,			0x52 },
    { WSA884X_PDRV_LS_CTL,			0x4a },
    { WSA884X_SPK_TOP_SPARE_CTL_1,		0x00 },
    { WSA884X_SPK_TOP_SPARE_CTL_2,		0x00 },
    { WSA884X_SPK_TOP_SPARE_CTL_3,		0x00 },
    { WSA884X_SPK_TOP_SPARE_CTL_4,		0x00 },
    { WSA884X_SPARE_CTL_5,			0x00 },
    { WSA884X_DAC_EN_DEBUG_REG,		0x00 },
    { WSA884X_DAC_OPAMP_BIAS1_REG,		0x48 },
    { WSA884X_DAC_OPAMP_BIAS2_REG,		0x48 },
    { WSA884X_DAC_TUNE1,			0x02 },
    { WSA884X_DAC_VOLTAGE_CTRL_REG,		0x05 },
    { WSA884X_ATEST1_REG,			0x00 },
    { WSA884X_ATEST2_REG,			0x00 },
    { WSA884X_TOP_BIAS_REG1,		0x6a },
    { WSA884X_TOP_BIAS_REG2,		0x65 },
    { WSA884X_TOP_BIAS_REG3,		0x55 },
    { WSA884X_TOP_BIAS_REG4,		0xa9 },
    { WSA884X_PWRSTG_DBG2,			0x21 },
    { WSA884X_DRV_LF_BLK_EN,		0x0f },
    { WSA884X_DRV_LF_EN,			0x0a },
    { WSA884X_DRV_LF_MASK_DCC_CTL,		0x08 },
    { WSA884X_DRV_LF_MISC_CTL1,		0x30 },
    { WSA884X_DRV_LF_REG_GAIN,		0x00 },
    { WSA884X_DRV_OS_CAL_CTL,		0x00 },
    { WSA884X_DRV_OS_CAL_CTL1,		0x90 },
    { WSA884X_PWRSTG_DBG,			0x08 },
    { WSA884X_BBM_CTL,			0x92 },
    { WSA884X_TOP_MISC1,			0x00 },
    { WSA884X_DAC_VCM_CTRL_REG7,		0x00 },
    { WSA884X_TOP_BIAS_REG5,		0x15 },
    { WSA884X_DRV_LF_MISC_CTL2,		0x00 },
    { WSA884X_STB_CTRL1,			0x42 },
    { WSA884X_CURRENT_LIMIT,		0x54 },
    { WSA884X_BYP_CTRL1,			0x01 },
    { WSA884X_SPARE_CTL_0,			0x00 },
    { WSA884X_BOOST_SPARE_CTL_1,		0x00 },
    { WSA884X_IBIAS1,			0x00 },
    { WSA884X_IBIAS2,			0x00 },
    { WSA884X_IBIAS3,			0x00 },
    { WSA884X_EN_CTRL,			0x42 },
    { WSA884X_STB_CTRL2,			0x03 },
    { WSA884X_STB_CTRL3,			0x3c },
    { WSA884X_STB_CTRL4,			0x30 },
    { WSA884X_BYP_CTRL2,			0x97 },
    { WSA884X_BYP_CTRL3,			0x11 },
    { WSA884X_ZX_CTRL1,			0xf0 },
    { WSA884X_ZX_CTRL2,			0x04 },
    { WSA884X_BLEEDER_CTRL,			0x04 },
    { WSA884X_BOOST_MISC,			0x62 },
    { WSA884X_PWRSTAGE_CTRL1,		0x00 },
    { WSA884X_PWRSTAGE_CTRL2,		0x31 },
    { WSA884X_PWRSTAGE_CTRL3,		0x81 },
    { WSA884X_PWRSTAGE_CTRL4,		0x5f },
    { WSA884X_MAXD_REG1,			0x00 },
    { WSA884X_MAXD_REG2,			0x5b },
    { WSA884X_ILIM_CTRL1,			0xe2 },
    { WSA884X_ILIM_CTRL2,			0x90 },
    { WSA884X_TEST_CTRL1,			0x00 },
    { WSA884X_TEST_CTRL2,			0x00 },
    { WSA884X_SPARE1,			0x00 },
    { WSA884X_BOOT_CAP_CHECK,		0x01 },
    { WSA884X_PON_CTL_0,			0x12 },
    { WSA884X_PWRSAV_CTL,			0xaa },
    { WSA884X_PON_LDOL_SPARE_CTL_0,		0x00 },
    { WSA884X_PON_LDOL_SPARE_CTL_1,		0x00 },
    { WSA884X_PON_LDOL_SPARE_CTL_2,		0x00 },
    { WSA884X_PON_LDOL_SPARE_CTL_3,		0x00 },
    { WSA884X_PON_CLT_1,			0xe1 },
    { WSA884X_PON_CTL_2,			0x00 },
    { WSA884X_PON_CTL_3,			0x70 },
    { WSA884X_CKWD_CTL_0,			0x14 },
    { WSA884X_CKWD_CTL_1,			0x3b },
    { WSA884X_CKWD_CTL_2,			0x00 },
    { WSA884X_CKSK_CTL_0,			0x00 },
    { WSA884X_PADSW_CTL_0,			0x00 },
    { WSA884X_TEST_0,			0x00 },
    { WSA884X_TEST_1,			0x00 },
    { WSA884X_PON_LDOL_SPARE_TUNE_0,	0x00 },
    { WSA884X_PON_LDOL_SPARE_TUNE_1,	0x00 },
    { WSA884X_PON_LDOL_SPARE_TUNE_2,	0x00 },
    { WSA884X_PON_LDOL_SPARE_TUNE_3,	0x00 },
    { WSA884X_PON_LDOL_SPARE_TUNE_4,	0x00 },
    { WSA884X_DIG_CTRL0_PAGE,		0x00 },
    { WSA884X_CDC_RST_CTL,			0x01 },
    { WSA884X_SWR_RESET_EN,			0x00 },
    { WSA884X_TOP_CLK_CFG,			0x00 },
    { WSA884X_SWR_CLK_RATE,			0x00 },
    { WSA884X_CDC_PATH_MODE,		0x00 },
    { WSA884X_CDC_CLK_CTL,			0x1f },
    { WSA884X_PA_FSM_EN,			0x00 },
    { WSA884X_PA_FSM_CTL0,			0x00 },
    { WSA884X_PA_FSM_CTL1,			0xfe },
    { WSA884X_PA_FSM_TIMER0,		0x80 },
    { WSA884X_PA_FSM_TIMER1,		0x80 },
    { WSA884X_PA_FSM_ERR_CTL,		0x00 },
    { WSA884X_PA_FSM_MSK0,			0x00 },
    { WSA884X_PA_FSM_MSK1,			0x00 },
    { WSA884X_PA_FSM_BYP_CTL,		0x00 },
    { WSA884X_PA_FSM_BYP0,			0x00 },
    { WSA884X_PA_FSM_BYP1,			0x00 },
    { WSA884X_TADC_VALUE_CTL,		0x03 },
    { WSA884X_TEMP_DETECT_CTL,		0x01 },
    { WSA884X_TEMP_CONFIG0,			0x00 },
    { WSA884X_TEMP_CONFIG1,			0x00 },
    { WSA884X_VBAT_THRM_FLT_CTL,		0x7f },
    { WSA884X_VBAT_CAL_CTL,			0x01 },
    { WSA884X_UVLO_DEGLITCH_CTL,		0x05 },
    { WSA884X_BOP_DEGLITCH_CTL,		0x05 },
    { WSA884X_VBAT_ZONE_DETC_CTL,		0x31 },
    { WSA884X_CPS_CTL,			0x00 },
    { WSA884X_CDC_RX_CTL,			0xfe },
    { WSA884X_CDC_SPK_DSM_A1_0,		0x00 },
    { WSA884X_CDC_SPK_DSM_A1_1,		0x01 },
    { WSA884X_CDC_SPK_DSM_A2_0,		0x96 },
    { WSA884X_CDC_SPK_DSM_A2_1,		0x09 },
    { WSA884X_CDC_SPK_DSM_A3_0,		0xab },
    { WSA884X_CDC_SPK_DSM_A3_1,		0x05 },
    { WSA884X_CDC_SPK_DSM_A4_0,		0x1c },
    { WSA884X_CDC_SPK_DSM_A4_1,		0x02 },
    { WSA884X_CDC_SPK_DSM_A5_0,		0x17 },
    { WSA884X_CDC_SPK_DSM_A5_1,		0x02 },
    { WSA884X_CDC_SPK_DSM_A6_0,		0xaa },
    { WSA884X_CDC_SPK_DSM_A7_0,		0xe3 },
    { WSA884X_CDC_SPK_DSM_C_0,		0x69 },
    { WSA884X_CDC_SPK_DSM_C_1,		0x54 },
    { WSA884X_CDC_SPK_DSM_C_2,		0x02 },
    { WSA884X_CDC_SPK_DSM_C_3,		0x15 },
    { WSA884X_CDC_SPK_DSM_R1,		0xa4 },
    { WSA884X_CDC_SPK_DSM_R2,		0xb5 },
    { WSA884X_CDC_SPK_DSM_R3,		0x86 },
    { WSA884X_CDC_SPK_DSM_R4,		0x85 },
    { WSA884X_CDC_SPK_DSM_R5,		0xaa },
    { WSA884X_CDC_SPK_DSM_R6,		0xe2 },
    { WSA884X_CDC_SPK_DSM_R7,		0x62 },
    { WSA884X_CDC_SPK_GAIN_PDM_0,		0x00 },
    { WSA884X_CDC_SPK_GAIN_PDM_1,		0xfc },
    { WSA884X_CDC_SPK_GAIN_PDM_2,		0x05 },
    { WSA884X_PDM_WD_CTL,			0x00 },
    { WSA884X_DEM_BYPASS_DATA0,		0x00 },
    { WSA884X_DEM_BYPASS_DATA1,		0x00 },
    { WSA884X_DEM_BYPASS_DATA2,		0x00 },
    { WSA884X_DEM_BYPASS_DATA3,		0x00 },
    { WSA884X_DRE_CTL_0,			0x70 },
    { WSA884X_DRE_CTL_1,			0x04 },
    { WSA884X_DRE_IDLE_DET_CTL,		0x2f },
    { WSA884X_GAIN_RAMPING_CTL,		0x50 },
    { WSA884X_GAIN_RAMPING_MIN,		0x12 },
    { WSA884X_TAGC_CTL,			0x15 },
    { WSA884X_TAGC_TIME,			0xbc },
    { WSA884X_TAGC_FORCE_VAL,		0x00 },
    { WSA884X_VAGC_CTL,			0x01 },
    { WSA884X_VAGC_TIME,			0x0f },
    { WSA884X_VAGC_ATTN_LVL_1,		0x03 },
    { WSA884X_VAGC_ATTN_LVL_2,		0x06 },
    { WSA884X_VAGC_ATTN_LVL_3,		0x09 },
    { WSA884X_CLSH_CTL_0,			0x37 },
    { WSA884X_CLSH_CTL_1,			0x81 },
    { WSA884X_CLSH_V_HD_PA,			0x0c },
    { WSA884X_CLSH_V_PA_MIN,		0x00 },
    { WSA884X_CLSH_OVRD_VAL,		0x00 },
    { WSA884X_CLSH_HARD_MAX,		0xff },
    { WSA884X_CLSH_SOFT_MAX,		0xf5 },
    { WSA884X_CLSH_SIG_DP,			0x00 },
    { WSA884X_PBR_DELAY_CTL,		0x07 },
    { WSA884X_CLSH_SRL_MAX_PBR,		0x02 },
    { WSA884X_CLSH_VTH1,			0x00 },
    { WSA884X_CLSH_VTH2,			0x00 },
    { WSA884X_CLSH_VTH3,			0x00 },
    { WSA884X_CLSH_VTH4,			0x00 },
    { WSA884X_CLSH_VTH5,			0x00 },
    { WSA884X_CLSH_VTH6,			0x00 },
    { WSA884X_CLSH_VTH7,			0x00 },
    { WSA884X_CLSH_VTH8,			0x00 },
    { WSA884X_CLSH_VTH9,			0x00 },
    { WSA884X_CLSH_VTH10,			0x00 },
    { WSA884X_CLSH_VTH11,			0x00 },
    { WSA884X_CLSH_VTH12,			0x00 },
    { WSA884X_CLSH_VTH13,			0x00 },
    { WSA884X_CLSH_VTH14,			0x00 },
    { WSA884X_CLSH_VTH15,			0x00 },
    { WSA884X_DIG_CTRL1_PAGE,		0x00 },
    { WSA884X_PIN_CTL,			0x04 },
    { WSA884X_PIN_CTL_OE,			0x00 },
    { WSA884X_PIN_WDATA_IOPAD,		0x00 },
    { WSA884X_I2C_SLAVE_CTL,		0x00 },
    { WSA884X_SPMI_PAD_CTL0,		0x2f },
    { WSA884X_SPMI_PAD_CTL1,		0x2f },
    { WSA884X_SPMI_PAD_CTL2,		0x2f },
    { WSA884X_MEM_CTL,			0x00 },
    { WSA884X_SWR_HM_TEST0,			0x08 },
    { WSA884X_OTP_CTRL0,			0x00 },
    { WSA884X_OTP_CTRL2,			0x00 },
    { WSA884X_OTP_PRG_TCSP0,		0x77 },
    { WSA884X_OTP_PRG_TCSP1,		0x00 },
    { WSA884X_OTP_PRG_TPPS,			0x47 },
    { WSA884X_OTP_PRG_TVPS,			0x3b },
    { WSA884X_OTP_PRG_TVPH,			0x47 },
    { WSA884X_OTP_PRG_TPPR0,		0x47 },
    { WSA884X_OTP_PRG_TPPR1,		0x00 },
    { WSA884X_OTP_PRG_TPPH,			0x47 },
    { WSA884X_OTP_PRG_END,			0x47 },
    { WSA884X_WAVG_PLAY,			0x00 },
    { WSA884X_WAVG_CTL,			0x06 },
    { WSA884X_WAVG_LRA_PER_0,		0xd1 },
    { WSA884X_WAVG_LRA_PER_1,		0x00 },
    { WSA884X_WAVG_DELTA_THETA_0,		0xe6 },
    { WSA884X_WAVG_DELTA_THETA_1,		0x04 },
    { WSA884X_WAVG_DIRECT_AMP_0,		0x50 },
    { WSA884X_WAVG_DIRECT_AMP_1,		0x00 },
    { WSA884X_WAVG_PTRN_AMP0_0,		0x50 },
    { WSA884X_WAVG_PTRN_AMP0_1,		0x00 },
    { WSA884X_WAVG_PTRN_AMP1_0,		0x50 },
    { WSA884X_WAVG_PTRN_AMP1_1,		0x00 },
    { WSA884X_WAVG_PTRN_AMP2_0,		0x50 },
    { WSA884X_WAVG_PTRN_AMP2_1,		0x00 },
    { WSA884X_WAVG_PTRN_AMP3_0,		0x50 },
    { WSA884X_WAVG_PTRN_AMP3_1,		0x00 },
    { WSA884X_WAVG_PTRN_AMP4_0,		0x50 },
    { WSA884X_WAVG_PTRN_AMP4_1,		0x00 },
    { WSA884X_WAVG_PTRN_AMP5_0,		0x50 },
    { WSA884X_WAVG_PTRN_AMP5_1,		0x00 },
    { WSA884X_WAVG_PTRN_AMP6_0,		0x50 },
    { WSA884X_WAVG_PTRN_AMP6_1,		0x00 },
    { WSA884X_WAVG_PTRN_AMP7_0,		0x50 },
    { WSA884X_WAVG_PTRN_AMP7_1,		0x00 },
    { WSA884X_WAVG_PER_0_1,			0x88 },
    { WSA884X_WAVG_PER_2_3,			0x88 },
    { WSA884X_WAVG_PER_4_5,			0x88 },
    { WSA884X_WAVG_PER_6_7,			0x88 },
    { WSA884X_INTR_MODE,			0x00 },
    { WSA884X_INTR_MASK0,			0x90 },
    { WSA884X_INTR_MASK1,			0x00 },
    { WSA884X_INTR_CLEAR0,			0x00 },
    { WSA884X_INTR_CLEAR1,			0x00 },
    { WSA884X_INTR_LEVEL0,			0x04 },
    { WSA884X_INTR_LEVEL1,			0x00 },
    { WSA884X_INTR_SET0,			0x00 },
    { WSA884X_INTR_SET1,			0x00 },
    { WSA884X_INTR_TEST0,			0x00 },
    { WSA884X_INTR_TEST1,			0x00 },
    { WSA884X_PDM_TEST_MODE,		0x00 },
    { WSA884X_PA_FSM_DBG,			0x00 },
    { WSA884X_DIG_DEBUG_MODE,		0x00 },
    { WSA884X_DIG_DEBUG_SEL,		0x00 },
    { WSA884X_DIG_DEBUG_EN,			0x00 },
    { WSA884X_TADC_DETECT_DBG_CTL,		0x00 },
    { WSA884X_TADC_DEBUG_MSB,		0x00 },
    { WSA884X_TADC_DEBUG_LSB,		0x00 },
    { WSA884X_SAMPLE_EDGE_SEL,		0x7f },
    { WSA884X_SWR_EDGE_SEL,			0x00 },
    { WSA884X_TEST_MODE_CTL,		0x05 },
    { WSA884X_IOPAD_CTL,			0x00 },
    { WSA884X_ANA_CSR_DBG_ADD,		0x00 },
    { WSA884X_ANA_CSR_DBG_CTL,		0x12 },
    { WSA884X_CLK_DBG_CTL,			0x00 },
    { WSA884X_SPARE_0,			0x00 },
    { WSA884X_SPARE_1,			0x00 },
    { WSA884X_SPARE_2,			0x00 },
    { WSA884X_SCODE,			0x00 },
    { WSA884X_DIG_TRIM_PAGE,		0x00 },
    { WSA884X_EMEM_0,			0x00 },
    { WSA884X_EMEM_1,			0x00 },
    { WSA884X_EMEM_2,			0x00 },
    { WSA884X_EMEM_3,			0x00 },
    { WSA884X_EMEM_4,			0x00 },
    { WSA884X_EMEM_5,			0x00 },
    { WSA884X_EMEM_6,			0x00 },
    { WSA884X_EMEM_7,			0x00 },
    { WSA884X_EMEM_8,			0x00 },
    { WSA884X_EMEM_9,			0x00 },
    { WSA884X_EMEM_10,			0x00 },
    { WSA884X_EMEM_11,			0x00 },
    { WSA884X_EMEM_12,			0x00 },
    { WSA884X_EMEM_13,			0x00 },
    { WSA884X_EMEM_14,			0x00 },
    { WSA884X_EMEM_15,			0x00 },
    { WSA884X_EMEM_16,			0x00 },
    { WSA884X_EMEM_17,			0x00 },
    { WSA884X_EMEM_18,			0x00 },
    { WSA884X_EMEM_19,			0x00 },
    { WSA884X_EMEM_20,			0x00 },
    { WSA884X_EMEM_21,			0x00 },
    { WSA884X_EMEM_22,			0x00 },
    { WSA884X_EMEM_23,			0x00 },
    { WSA884X_EMEM_24,			0x00 },
    { WSA884X_EMEM_25,			0x00 },
    { WSA884X_EMEM_26,			0x00 },
    { WSA884X_EMEM_27,			0x00 },
    { WSA884X_EMEM_28,			0x00 },
    { WSA884X_EMEM_29,			0x00 },
    { WSA884X_EMEM_30,			0x00 },
    { WSA884X_EMEM_31,			0x00 },
    { WSA884X_EMEM_32,			0x00 },
    { WSA884X_EMEM_33,			0x00 },
    { WSA884X_EMEM_34,			0x00 },
    { WSA884X_EMEM_35,			0x00 },
    { WSA884X_EMEM_36,			0x00 },
    { WSA884X_EMEM_37,			0x00 },
    { WSA884X_EMEM_38,			0x00 },
    { WSA884X_EMEM_39,			0x00 },
    { WSA884X_EMEM_40,			0x00 },
    { WSA884X_EMEM_41,			0x00 },
    { WSA884X_EMEM_42,			0x00 },
    { WSA884X_EMEM_43,			0x00 },
    { WSA884X_EMEM_44,			0x00 },
    { WSA884X_EMEM_45,			0x00 },
    { WSA884X_EMEM_46,			0x00 },
    { WSA884X_EMEM_47,			0x00 },
    { WSA884X_EMEM_48,			0x00 },
    { WSA884X_EMEM_49,			0x00 },
    { WSA884X_EMEM_50,			0x00 },
    { WSA884X_EMEM_51,			0x00 },
    { WSA884X_EMEM_52,			0x00 },
    { WSA884X_EMEM_53,			0x00 },
    { WSA884X_EMEM_54,			0x00 },
    { WSA884X_EMEM_55,			0x00 },
    { WSA884X_EMEM_56,			0x00 },
    { WSA884X_EMEM_57,			0x00 },
    { WSA884X_EMEM_58,			0x00 },
    { WSA884X_EMEM_59,			0x00 },
    { WSA884X_EMEM_60,			0x00 },
    { WSA884X_EMEM_61,			0x00 },
    { WSA884X_EMEM_62,			0x00 },
    { WSA884X_EMEM_63,			0x00 },
    };
#[no_mangle]
unsafe extern "C" fn wsa884x_readonly_register(dev: *mut device, reg: c_uint) -> bool {
    static bool wsa884x_readonly_register(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case WSA884X_DOUT_MSB:
    case WSA884X_DOUT_LSB:
    case WSA884X_STATUS:
    case WSA884X_SPK_TOP_SPARE_TUNE_2:
    case WSA884X_SPK_TOP_SPARE_TUNE_3:
    case WSA884X_SPK_TOP_SPARE_TUNE_4:
    case WSA884X_SPARE_TUNE_5:
    case WSA884X_SPARE_TUNE_6:
    case WSA884X_SPARE_TUNE_7:
    case WSA884X_SPARE_TUNE_8:
    case WSA884X_SPARE_TUNE_9:
    case WSA884X_SPARE_TUNE_10:
    case WSA884X_PA_STATUS0:
    case WSA884X_PA_STATUS1:
    case WSA884X_PA_STATUS2:
    case WSA884X_PA_STATUS3:
    case WSA884X_PA_STATUS4:
    case WSA884X_PA_STATUS5:
    case WSA884X_SPARE_RO_1:
    case WSA884X_SPARE_RO_2:
    case WSA884X_SPARE_RO_3:
    case WSA884X_SPARE_RO_0:
    case WSA884X_BOOST_SPARE_RO_1:
    case WSA884X_STATUS_0:
    case WSA884X_STATUS_1:
    case WSA884X_CHIP_ID0:
    case WSA884X_CHIP_ID1:
    case WSA884X_CHIP_ID2:
    case WSA884X_CHIP_ID3:
    case WSA884X_BUS_ID:
    case WSA884X_PA_FSM_STA0:
    case WSA884X_PA_FSM_STA1:
    case WSA884X_PA_FSM_ERR_COND0:
    case WSA884X_PA_FSM_ERR_COND1:
    case WSA884X_TEMP_DIN_MSB:
    case WSA884X_TEMP_DIN_LSB:
    case WSA884X_TEMP_DOUT_MSB:
    case WSA884X_TEMP_DOUT_LSB:
    case WSA884X_VBAT_DIN_MSB:
    case WSA884X_VBAT_DIN_LSB:
    case WSA884X_VBAT_DOUT_MSB:
    case WSA884X_VBAT_DOUT_LSB:
    case WSA884X_VBAT_CAL_MSB:
    case WSA884X_VBAT_CAL_LSB:
    case WSA884X_VPHX_SYS_EN_STATUS:
    case WSA884X_PIN_STATUS:
    case WSA884X_SWR_HM_TEST1:
    case WSA884X_OTP_CTRL1:
    case WSA884X_OTP_STAT:
    case WSA884X_WAVG_STA:
    case WSA884X_INTR_STATUS0:
    case WSA884X_INTR_STATUS1:
    case WSA884X_ATE_TEST_MODE:
    case WSA884X_SPARE_R:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn wsa884x_writeable_register(dev: *mut device, reg: c_uint) -> bool {
    static bool wsa884x_writeable_register(struct device *dev, unsigned int reg)
    {
    return !wsa884x_readonly_register(dev, reg);
    }
#[no_mangle]
unsafe extern "C" fn wsa884x_volatile_register(dev: *mut device, reg: c_uint) -> bool {
    static bool wsa884x_volatile_register(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case WSA884X_ANA_WO_CTL_0:
    case WSA884X_ANA_WO_CTL_1:
    return true;
    }
    return wsa884x_readonly_register(dev, reg);
    }
    static const struct regmap_config wsa884x_regmap_config = {
    .reg_bits = 32,
    .val_bits = 8,
    .cache_type = REGCACHE_MAPLE,
    .reg_defaults = wsa884x_defaults,
    .max_register = WSA884X_MAX_REGISTER,
    .num_reg_defaults = ARRAY_SIZE(wsa884x_defaults),
    .volatile_reg = wsa884x_volatile_register,
    .writeable_reg = wsa884x_writeable_register,
    .reg_format_endian = REGMAP_ENDIAN_NATIVE,
    .val_format_endian = REGMAP_ENDIAN_NATIVE,
    .use_single_read = true,
    };
    static const struct reg_sequence wsa884x_reg_init[] = {
    { WSA884X_BOP2_PROG, FIELD_PREP_CONST(WSA884X_BOP2_PROG_BOP2_VTH_MASK, 0x6) |
    FIELD_PREP_CONST(WSA884X_BOP2_PROG_BOP2_HYST_MASK, 0x6) },
    { WSA884X_REF_CTRL, (0xd2 & ~WSA884X_REF_CTRL_BG_RDY_SEL_MASK) |
    FIELD_PREP_CONST(WSA884X_REF_CTRL_BG_RDY_SEL_MASK, 0x1) },
//
// Downstream suggests for batteries different than 1-Stacked (1S):
// { WSA884X_TOP_CTRL1, 0xd3 & ~WSA884X_TOP_CTRL1_OCP_LOWVBAT_ITH_EN_MASK },
//
    { WSA884X_STB_CTRL1, (0x42 & ~WSA884X_STB_CTRL1_SLOPE_COMP_CURRENT_MASK) |
    FIELD_PREP_CONST(WSA884X_STB_CTRL1_SLOPE_COMP_CURRENT_MASK, 0xd) },
    { WSA884X_CURRENT_LIMIT, (0x54 & ~WSA884X_CURRENT_LIMIT_CURRENT_LIMIT_MASK) |
    FIELD_PREP_CONST(WSA884X_CURRENT_LIMIT_CURRENT_LIMIT_MASK, 0x9) },
    { WSA884X_ZX_CTRL1, (0xf0 & ~WSA884X_ZX_CTRL1_ZX_DET_SW_SEL_MASK) |
    FIELD_PREP_CONST(WSA884X_ZX_CTRL1_ZX_DET_SW_SEL_MASK, 0x3) },
    { WSA884X_ILIM_CTRL1, (0xe2 & ~WSA884X_ILIM_CTRL1_ILIM_OFFSET_PB_MASK) |
    FIELD_PREP_CONST(WSA884X_ILIM_CTRL1_ILIM_OFFSET_PB_MASK, 0x3) },
    { WSA884X_CKWD_CTL_1, FIELD_PREP_CONST(WSA884X_CKWD_CTL_1_VPP_SW_CTL_MASK, 0x0) |
    FIELD_PREP_CONST(WSA884X_CKWD_CTL_1_CKWD_VCOMP_VREF_SEL_MASK, 0x13) },
    { WSA884X_PA_FSM_CTL1, (0xfe & ~WSA884X_PA_FSM_CTL1_NOISE_GATE_BLOCK_MASK) |
    FIELD_PREP_CONST(WSA884X_PA_FSM_CTL1_NOISE_GATE_BLOCK_MASK, 0x4) }, /* == 0xfe */
    { WSA884X_VBAT_THRM_FLT_CTL, (0x7f & ~WSA884X_VBAT_THRM_FLT_CTL_VBAT_COEF_SEL_MASK) |
    FIELD_PREP_CONST(WSA884X_VBAT_THRM_FLT_CTL_VBAT_COEF_SEL_MASK, 0x4) },
    { WSA884X_VBAT_CAL_CTL, FIELD_PREP_CONST(WSA884X_VBAT_CAL_CTL_RESERVE_MASK, 0x2) |
    FIELD_PREP_CONST(WSA884X_VBAT_CAL_CTL_VBAT_CAL_EN_MASK, 0x1) },
    { WSA884X_BOP_DEGLITCH_CTL, FIELD_PREP_CONST(WSA884X_BOP_DEGLITCH_CTL_BOP_DEGLITCH_SETTING_MASK, 0x8) |
    FIELD_PREP_CONST(WSA884X_BOP_DEGLITCH_CTL_BOP_DEGLITCH_EN_MASK, 0x1) },
    { WSA884X_CDC_SPK_DSM_A2_0, 0x0a },
    { WSA884X_CDC_SPK_DSM_A2_1, 0x08 },
    { WSA884X_CDC_SPK_DSM_A3_0, 0xf3 },
    { WSA884X_CDC_SPK_DSM_A3_1, 0x07 },
    { WSA884X_CDC_SPK_DSM_A4_0, 0x79 },
    { WSA884X_CDC_SPK_DSM_A5_0, 0x0b },
    { WSA884X_CDC_SPK_DSM_A6_0, 0x8a },
    { WSA884X_CDC_SPK_DSM_A7_0, 0x9b },
    { WSA884X_CDC_SPK_DSM_C_0, FIELD_PREP_CONST(WSA884X_CDC_SPK_DSM_C_0_COEF_C3_MASK, 0x6) |
    FIELD_PREP_CONST(WSA884X_CDC_SPK_DSM_C_0_COEF_C2_MASK, 0x8) },
    { WSA884X_CDC_SPK_DSM_C_2, FIELD_PREP_CONST(WSA884X_CDC_SPK_DSM_C_2_COEF_C7_MASK, 0xf) },
    { WSA884X_CDC_SPK_DSM_C_3, FIELD_PREP_CONST(WSA884X_CDC_SPK_DSM_C_3_COEF_C7_MASK, 0x20) },
    { WSA884X_CDC_SPK_DSM_R1, 0x83 },
    { WSA884X_CDC_SPK_DSM_R2, 0x7f },
    { WSA884X_CDC_SPK_DSM_R3, 0x9d },
    { WSA884X_CDC_SPK_DSM_R4, 0x82 },
    { WSA884X_CDC_SPK_DSM_R5, 0x8b },
    { WSA884X_CDC_SPK_DSM_R6, 0x9b },
    { WSA884X_CDC_SPK_DSM_R7, 0x3f },
// Speaker mode by default
    { WSA884X_DRE_CTL_0, FIELD_PREP_CONST(WSA884X_DRE_CTL_0_PROG_DELAY_MASK, 0x7) },
    { WSA884X_CLSH_CTL_0, (0x37 & ~WSA884X_CLSH_CTL_0_DLY_CODE_MASK) |
    FIELD_PREP_CONST(WSA884X_CLSH_CTL_0_DLY_CODE_MASK, 0x6) },
//
// WSA884X_CLSH_VTH values for speaker mode with G_21_DB system gain,
// battery 1S and rload 8 Ohms.
//
    { WSA884X_CLSH_VTH1, WSA884X_VTH_TO_REG(863), },
    { WSA884X_CLSH_VTH2, WSA884X_VTH_TO_REG(918), },
    { WSA884X_CLSH_VTH3, WSA884X_VTH_TO_REG(980), },
    { WSA884X_CLSH_VTH4, WSA884X_VTH_TO_REG(1043), },
    { WSA884X_CLSH_VTH5, WSA884X_VTH_TO_REG(1098), },
    { WSA884X_CLSH_VTH6, WSA884X_VTH_TO_REG(1137), },
    { WSA884X_CLSH_VTH7, WSA884X_VTH_TO_REG(1184), },
    { WSA884X_CLSH_VTH8, WSA884X_VTH_TO_REG(1239), },
    { WSA884X_CLSH_VTH9, WSA884X_VTH_TO_REG(1278), },
    { WSA884X_CLSH_VTH10, WSA884X_VTH_TO_REG(1380), },
    { WSA884X_CLSH_VTH11, WSA884X_VTH_TO_REG(1482), },
    { WSA884X_CLSH_VTH12, WSA884X_VTH_TO_REG(1584), },
    { WSA884X_CLSH_VTH13, WSA884X_VTH_TO_REG(1663), },
    { WSA884X_CLSH_VTH14, WSA884X_VTH_TO_REG(1780), },
    { WSA884X_CLSH_VTH15, WSA884X_VTH_TO_REG(2000), },
    { WSA884X_ANA_WO_CTL_1, 0x00 },
    { WSA884X_OTP_REG_38, 0x00 },
    { WSA884X_OTP_REG_40, FIELD_PREP_CONST(WSA884X_OTP_REG_40_ISENSE_RESCAL_MASK, 0x8) },
    };
#[no_mangle]
unsafe extern "C" fn wsa884x_set_gain_parameters(wsa884x: *mut wsa884x_priv) {
    static void wsa884x_set_gain_parameters(struct wsa884x_priv *wsa884x)
    {
    struct regmap *regmap = wsa884x.regmap;
    unsigned int min_gain, igain, vgain, comp_offset;
//
// Downstream sets gain parameters customized per boards per use-case.
// Choose here some sane values matching knowon users, like QRD8550
// board:.
//
// Values match here downstream:
// For WSA884X_RECEIVER - G_7P5_DB system gain
// For WSA884X_SPEAKER - G_21_DB system gain
//
    if (wsa884x.dev_mode == WSA884X_RECEIVER) {
    comp_offset = COMP_OFFSET4;
    min_gain = G_M6_DB;
    igain = ISENSE_18_DB;
    vgain = VSENSE_M12_DB;
    } else {
// WSA884X_SPEAKER
    comp_offset = COMP_OFFSET0;
    min_gain = G_0_DB;
    igain = ISENSE_12_DB;
    vgain = VSENSE_M24_DB;
    }
    regmap_update_bits(regmap, WSA884X_ISENSE2,
    WSA884X_ISENSE2_ISENSE_GAIN_CTL_MASK,
    FIELD_PREP(WSA884X_ISENSE2_ISENSE_GAIN_CTL_MASK, igain));
    regmap_update_bits(regmap, WSA884X_VSENSE1,
    WSA884X_VSENSE1_GAIN_VSENSE_FE_MASK,
    FIELD_PREP(WSA884X_VSENSE1_GAIN_VSENSE_FE_MASK, vgain));
    regmap_update_bits(regmap, WSA884X_GAIN_RAMPING_MIN,
    WSA884X_GAIN_RAMPING_MIN_MIN_GAIN_MASK,
    FIELD_PREP(WSA884X_GAIN_RAMPING_MIN_MIN_GAIN_MASK, min_gain));
    if (wsa884x.port_enable[WSA884X_PORT_COMP]) {
    regmap_update_bits(regmap, WSA884X_DRE_CTL_0,
    WSA884X_DRE_CTL_0_OFFSET_MASK,
    FIELD_PREP(WSA884X_DRE_CTL_0_OFFSET_MASK, comp_offset));
    regmap_update_bits(regmap, WSA884X_DRE_CTL_1,
    WSA884X_DRE_CTL_1_CSR_GAIN_EN_MASK,
    FIELD_PREP(WSA884X_DRE_CTL_1_CSR_GAIN_EN_MASK, 0x0));
    } else {
    regmap_update_bits(regmap, WSA884X_DRE_CTL_1,
    WSA884X_DRE_CTL_1_CSR_GAIN_EN_MASK,
    FIELD_PREP(WSA884X_DRE_CTL_1_CSR_GAIN_EN_MASK, 0x1));
    }
    }
#[no_mangle]
unsafe extern "C" fn wsa884x_init(wsa884x: *mut wsa884x_priv) {
    static void wsa884x_init(struct wsa884x_priv *wsa884x)
    {
    unsigned int wo_ctl_0;
    let mut variant: c_uint = 0;
    if (!regmap_read(wsa884x.regmap, WSA884X_OTP_REG_0, &variant))
    variant = variant & WSA884X_OTP_REG_0_ID_MASK;
    regmap_multi_reg_write(wsa884x.regmap, wsa884x_reg_init,
    ARRAY_SIZE(wsa884x_reg_init));
    wo_ctl_0 = 0xc;
    wo_ctl_0 |= FIELD_PREP(WSA884X_ANA_WO_CTL_0_DAC_CM_CLAMP_EN_MASK,
    WSA884X_ANA_WO_CTL_0_DAC_CM_CLAMP_EN_MODE_SPEAKER);
// Assume that compander is enabled by default unless it is haptics sku
    if (variant == WSA884X_OTP_ID_WSA8845H)
    wo_ctl_0 |= FIELD_PREP(WSA884X_ANA_WO_CTL_0_PA_AUX_GAIN_MASK,
    WSA884X_ANA_WO_CTL_0_PA_AUX_18_DB);
    else
    wo_ctl_0 |= FIELD_PREP(WSA884X_ANA_WO_CTL_0_PA_AUX_GAIN_MASK,
    WSA884X_ANA_WO_CTL_0_PA_AUX_0_DB);
    regmap_write(wsa884x.regmap, WSA884X_ANA_WO_CTL_0, wo_ctl_0);
    wsa884x_set_gain_parameters(wsa884x);
    wsa884x.hw_init = true;
    }
    static int wsa884x_update_status(struct sdw_slave *slave,
    enum sdw_slave_status status)
    {
    struct wsa884x_priv *wsa884x = dev_get_drvdata(&slave.dev);
    int ret;
    if (status == SDW_SLAVE_UNATTACHED) {
    wsa884x.hw_init = false;
    regcache_cache_only(wsa884x.regmap, true);
    regcache_mark_dirty(wsa884x.regmap);
    return 0;
    }
    if (wsa884x.hw_init || status != SDW_SLAVE_ATTACHED)
    return 0;
    regcache_cache_only(wsa884x.regmap, false);
    ret = regcache_sync(wsa884x.regmap);
    if (ret < 0) {
    dev_err(&slave.dev, "Cannot sync regmap cache\n");
    return ret;
    }
    wsa884x_init(wsa884x);
    return 0;
    }
    static int wsa884x_port_prep(struct sdw_slave *slave,
    struct sdw_prepare_ch *prepare_ch,
    enum sdw_port_prep_ops state)
    {
    struct wsa884x_priv *wsa884x = dev_get_drvdata(&slave.dev);
    if (state == SDW_OPS_PORT_POST_PREP)
    wsa884x.port_prepared[prepare_ch.num - 1] = true;
    else
    wsa884x.port_prepared[prepare_ch.num - 1] = false;
    return 0;
    }
    static const struct sdw_slave_ops wsa884x_slave_ops = {
    .update_status = wsa884x_update_status,
    .port_prep = wsa884x_port_prep,
    };
    static int wsa884x_dev_mode_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct wsa884x_priv *wsa884x = snd_soc_component_get_drvdata(component);
    ucontrol.value.enumerated.item[0] = wsa884x.dev_mode;
    return 0;
    }
    static int wsa884x_dev_mode_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct wsa884x_priv *wsa884x = snd_soc_component_get_drvdata(component);
    if (wsa884x.dev_mode == ucontrol.value.enumerated.item[0])
    return 0;
    wsa884x.dev_mode = ucontrol.value.enumerated.item[0];
    return 1;
    }
    static int wsa884x_get_swr_port(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *comp = snd_kcontrol_chip(kcontrol);
    struct wsa884x_priv *wsa884x = snd_soc_component_get_drvdata(comp);
    struct soc_mixer_control *mixer = (struct soc_mixer_control *)kcontrol.private_value;
    let mut portidx: c_int = mixer.reg;
    ucontrol.value.integer.value[0] = wsa884x.port_enable[portidx];
    return 0;
    }
    static int wsa884x_set_swr_port(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *comp = snd_kcontrol_chip(kcontrol);
    struct wsa884x_priv *wsa884x = snd_soc_component_get_drvdata(comp);
    struct soc_mixer_control *mixer = (struct soc_mixer_control *)kcontrol.private_value;
    let mut portidx: c_int = mixer.reg;
    if (ucontrol.value.integer.value[0]) {
    if (wsa884x.port_enable[portidx])
    return 0;
    wsa884x.port_enable[portidx] = true;
    } else {
    if (!wsa884x.port_enable[portidx])
    return 0;
    wsa884x.port_enable[portidx] = false;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn wsa884x_codec_probe(comp: *mut snd_soc_component) -> c_int {
    static int wsa884x_codec_probe(struct snd_soc_component *comp)
    {
    struct wsa884x_priv *wsa884x = snd_soc_component_get_drvdata(comp);
    snd_soc_component_init_regmap(comp, wsa884x.regmap);
    return 0;
    }
    static void wsa884x_spkr_post_pmu(struct snd_soc_component *component,
    struct wsa884x_priv *wsa884x)
    {
    unsigned int curr_limit, curr_ovrd_en;
    wsa884x_set_gain_parameters(wsa884x);
    if (wsa884x.dev_mode == WSA884X_RECEIVER) {
    snd_soc_component_write_field(component, WSA884X_DRE_CTL_0,
    WSA884X_DRE_CTL_0_PROG_DELAY_MASK, 0x3);
    snd_soc_component_write_field(component, WSA884X_CDC_PATH_MODE,
    WSA884X_CDC_PATH_MODE_RXD_MODE_MASK,
    0x1);
    snd_soc_component_write_field(component, WSA884X_PWM_CLK_CTL,
    WSA884X_PWM_CLK_CTL_PWM_CLK_FREQ_SEL_MASK,
    0x1);
    } else {
// WSA884X_SPEAKER
    snd_soc_component_write_field(component, WSA884X_DRE_CTL_0,
    WSA884X_DRE_CTL_0_PROG_DELAY_MASK, 0xf);
    }
    if (wsa884x.port_enable[WSA884X_PORT_PBR]) {
    curr_ovrd_en = 0x0;
    curr_limit = 0x15;
    } else {
    curr_ovrd_en = 0x1;
    if (wsa884x.dev_mode == WSA884X_RECEIVER)
    curr_limit = 0x9;
    else
    curr_limit = 0x15;
    }
    snd_soc_component_write_field(component, WSA884X_CURRENT_LIMIT,
    WSA884X_CURRENT_LIMIT_CURRENT_LIMIT_OVRD_EN_MASK,
    curr_ovrd_en);
    snd_soc_component_write_field(component, WSA884X_CURRENT_LIMIT,
    WSA884X_CURRENT_LIMIT_CURRENT_LIMIT_MASK,
    curr_limit);
    }
    static int wsa884x_spkr_event(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol, int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    struct wsa884x_priv *wsa884x = snd_soc_component_get_drvdata(component);
    switch (event) {
    case SND_SOC_DAPM_POST_PMU:
    scoped_guard(mutex, &wsa884x.sp_lock)
    wsa884x.pa_on = true;
    wsa884x_spkr_post_pmu(component, wsa884x);
    snd_soc_component_write_field(component, WSA884X_PDM_WD_CTL,
    WSA884X_PDM_WD_CTL_PDM_WD_EN_MASK,
    0x1);
    break;
    case SND_SOC_DAPM_PRE_PMD:
    snd_soc_component_write_field(component, WSA884X_PDM_WD_CTL,
    WSA884X_PDM_WD_CTL_PDM_WD_EN_MASK,
    0x0);
    scoped_guard(mutex, &wsa884x.sp_lock)
    wsa884x.pa_on = false;
    break;
    }
    return 0;
    }
    static const struct snd_soc_dapm_widget wsa884x_dapm_widgets[] = {
    SND_SOC_DAPM_INPUT("IN"),
    SND_SOC_DAPM_SPK("SPKR", wsa884x_spkr_event),
    };
    static const DECLARE_TLV_DB_SCALE(pa_gain, -900, 150, -900);
    static const struct snd_kcontrol_new wsa884x_snd_controls[] = {
    SOC_SINGLE_RANGE_TLV("PA Volume", WSA884X_DRE_CTL_1,
    WSA884X_DRE_CTL_1_CSR_GAIN_SHIFT,
    0x0, 0x1f, 1, pa_gain),
    SOC_ENUM_EXT("WSA MODE", wsa884x_dev_mode_enum,
    wsa884x_dev_mode_get, wsa884x_dev_mode_put),
    SOC_SINGLE_EXT("DAC Switch", WSA884X_PORT_DAC, 0, 1, 0,
    wsa884x_get_swr_port, wsa884x_set_swr_port),
    SOC_SINGLE_EXT("COMP Switch", WSA884X_PORT_COMP, 0, 1, 0,
    wsa884x_get_swr_port, wsa884x_set_swr_port),
    SOC_SINGLE_EXT("BOOST Switch", WSA884X_PORT_BOOST, 0, 1, 0,
    wsa884x_get_swr_port, wsa884x_set_swr_port),
    SOC_SINGLE_EXT("PBR Switch", WSA884X_PORT_PBR, 0, 1, 0,
    wsa884x_get_swr_port, wsa884x_set_swr_port),
    SOC_SINGLE_EXT("VISENSE Switch", WSA884X_PORT_VISENSE, 0, 1, 0,
    wsa884x_get_swr_port, wsa884x_set_swr_port),
    SOC_SINGLE_EXT("CPS Switch", WSA884X_PORT_CPS, 0, 1, 0,
    wsa884x_get_swr_port, wsa884x_set_swr_port),
    };
    static const struct snd_soc_dapm_route wsa884x_audio_map[] = {
    {"SPKR", core::ptr::null_mut(), "IN"},
    };
    static const struct snd_soc_component_driver wsa884x_component_drv = {
    .name = "WSA884x",
    .probe = wsa884x_codec_probe,
    .controls = wsa884x_snd_controls,
    .num_controls = ARRAY_SIZE(wsa884x_snd_controls),
    .dapm_widgets = wsa884x_dapm_widgets,
    .num_dapm_widgets = ARRAY_SIZE(wsa884x_dapm_widgets),
    .dapm_routes = wsa884x_audio_map,
    .num_dapm_routes = ARRAY_SIZE(wsa884x_audio_map),
    };
    static int wsa884x_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct wsa884x_priv *wsa884x = dev_get_drvdata(dai.dev);
    int i;
    wsa884x.active_ports = 0;
    for (i = 0; i < WSA884X_MAX_SWR_PORTS; i++) {
    if (!wsa884x.port_enable[i])
    continue;
    wsa884x.port_config[wsa884x.active_ports] = wsa884x_pconfig[i];
    wsa884x.active_ports++;
    }
    wsa884x.sconfig.frame_rate = params_rate(params);
    return sdw_stream_add_slave(wsa884x.slave, &wsa884x.sconfig,
    wsa884x.port_config, wsa884x.active_ports,
    wsa884x.sruntime);
    }
    static int wsa884x_hw_free(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct wsa884x_priv *wsa884x = dev_get_drvdata(dai.dev);
    sdw_stream_remove_slave(wsa884x.slave, wsa884x.sruntime);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wsa884x_mute_stream(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int {
    static int wsa884x_mute_stream(struct snd_soc_dai *dai, int mute, int stream)
    {
    struct snd_soc_component *component = dai.component;
    if (mute) {
    snd_soc_component_write_field(component, WSA884X_DRE_CTL_1,
    WSA884X_DRE_CTL_1_CSR_GAIN_EN_MASK,
    0x0);
    snd_soc_component_write_field(component, WSA884X_PA_FSM_EN,
    WSA884X_PA_FSM_EN_GLOBAL_PA_EN_MASK,
    0x0);
    } else {
    snd_soc_component_write_field(component, WSA884X_DRE_CTL_1,
    WSA884X_DRE_CTL_1_CSR_GAIN_EN_MASK,
    0x1);
    snd_soc_component_write_field(component, WSA884X_PA_FSM_EN,
    WSA884X_PA_FSM_EN_GLOBAL_PA_EN_MASK,
    0x1);
    }
    return 0;
    }
    static int wsa884x_set_stream(struct snd_soc_dai *dai,
    void *stream, int direction)
    {
    struct wsa884x_priv *wsa884x = dev_get_drvdata(dai.dev);
    wsa884x.sruntime = stream;
    return 0;
    }
    static const struct snd_soc_dai_ops wsa884x_dai_ops = {
    .hw_params = wsa884x_hw_params,
    .hw_free = wsa884x_hw_free,
    .mute_stream = wsa884x_mute_stream,
    .set_stream = wsa884x_set_stream,
    .mute_unmute_on_trigger = true,
    };
    static struct snd_soc_dai_driver wsa884x_dais[] = {
    {
    .name = "SPKR",
    .playback = {
    .stream_name = "SPKR Playback",
    .rates = WSA884X_RATES | WSA884X_FRAC_RATES,
    .formats = WSA884X_FORMATS,
    .rate_min = 8000,
    .rate_max = 384000,
    .channels_min = 1,
    .channels_max = 1,
    },
    .ops = &wsa884x_dai_ops,
    },
    };
#[no_mangle]
unsafe extern "C" fn wsa884x_get_temp(wsa884x: *mut wsa884x_priv, temp: *mut c_long) -> c_int {
    static int wsa884x_get_temp(struct wsa884x_priv *wsa884x, long *temp)
    {
    let mut d1_msb: c_uint = 0, d1_lsb = 0, d2_msb = 0, d2_lsb = 0;
    let mut dmeas_msb: c_uint = 0, dmeas_lsb = 0;
    int d1, d2, dmeas;
    unsigned int mask;
    long val;
    int ret;
    guard(mutex)(&wsa884x.sp_lock);
    if (wsa884x.pa_on) {
//
// Reading temperature is possible only when Power Amplifier is
// off. Report last cached data.
//
// temp = wsa884x->temperature * 1000;
    return 0;
    }
    ret = pm_runtime_resume_and_get(wsa884x.dev);
    if (ret < 0)
    return ret;
    mask = WSA884X_PA_FSM_BYP0_DC_CAL_EN_MASK |
    WSA884X_PA_FSM_BYP0_CLK_WD_EN_MASK |
    WSA884X_PA_FSM_BYP0_BG_EN_MASK |
    WSA884X_PA_FSM_BYP0_D_UNMUTE_MASK |
    WSA884X_PA_FSM_BYP0_SPKR_PROT_EN_MASK |
    WSA884X_PA_FSM_BYP0_TSADC_EN_MASK;
//
// Here and further do not care about read or update failures.
// For example, before turning on Power Amplifier for the first
// time, reading WSA884X_TEMP_DIN_MSB will always return 0.
// Instead, check if returned value is within reasonable
// thresholds.
//
    regmap_update_bits(wsa884x.regmap, WSA884X_PA_FSM_BYP0, mask, mask);
    regmap_update_bits(wsa884x.regmap, WSA884X_TADC_VALUE_CTL,
    WSA884X_TADC_VALUE_CTL_TEMP_VALUE_RD_EN_MASK,
    FIELD_PREP(WSA884X_TADC_VALUE_CTL_TEMP_VALUE_RD_EN_MASK, 0x0));
    regmap_read(wsa884x.regmap, WSA884X_TEMP_DIN_MSB, &dmeas_msb);
    regmap_read(wsa884x.regmap, WSA884X_TEMP_DIN_LSB, &dmeas_lsb);
    regmap_update_bits(wsa884x.regmap, WSA884X_TADC_VALUE_CTL,
    WSA884X_TADC_VALUE_CTL_TEMP_VALUE_RD_EN_MASK,
    FIELD_PREP(WSA884X_TADC_VALUE_CTL_TEMP_VALUE_RD_EN_MASK, 0x1));
    regmap_read(wsa884x.regmap, WSA884X_OTP_REG_1, &d1_msb);
    regmap_read(wsa884x.regmap, WSA884X_OTP_REG_2, &d1_lsb);
    regmap_read(wsa884x.regmap, WSA884X_OTP_REG_3, &d2_msb);
    regmap_read(wsa884x.regmap, WSA884X_OTP_REG_4, &d2_lsb);
    regmap_update_bits(wsa884x.regmap, WSA884X_PA_FSM_BYP0, mask, 0x0);
    dmeas = (((dmeas_msb & 0xff) << 0x8) | (dmeas_lsb & 0xff)) >> 0x6;
    d1 = (((d1_msb & 0xff) << 0x8) | (d1_lsb & 0xff)) >> 0x6;
    d2 = (((d2_msb & 0xff) << 0x8) | (d2_lsb & 0xff)) >> 0x6;
    if (d1 == d2) {
// Incorrect data in OTP?
    ret = -EINVAL;
    goto out;
    }
    val = WSA884X_T1_TEMP + (((dmeas - d1) * (WSA884X_T2_TEMP - WSA884X_T1_TEMP))/(d2 - d1));
    dev_dbg(wsa884x.dev, "Measured temp %ld (dmeas=%d, d1=%d, d2=%d)\n",
    val, dmeas, d1, d2);
    if ((val > WSA884X_LOW_TEMP_THRESHOLD) &&
    (val < WSA884X_HIGH_TEMP_THRESHOLD)) {
    wsa884x.temperature = val;
// temp = val * 1000;
    ret = 0;
    } else {
    ret = -EAGAIN;
    }
    out:
    pm_runtime_put_autosuspend(wsa884x.dev);
    return ret;
    }
    static umode_t wsa884x_hwmon_is_visible(const void *data,
    enum hwmon_sensor_types type, u32 attr,
    int channel)
    {
    if (type != hwmon_temp)
    return 0;
    switch (attr) {
    case hwmon_temp_input:
    return 0444;
    default:
    break;
    }
    return 0;
    }
    static int wsa884x_hwmon_read(struct device *dev,
    enum hwmon_sensor_types type,
    u32 attr, int channel, long *temp)
    {
    int ret;
    switch (attr) {
    case hwmon_temp_input:
    ret = wsa884x_get_temp(dev_get_drvdata(dev), temp);
    break;
    default:
    ret = -EOPNOTSUPP;
    break;
    }
    return ret;
    }
    static const struct hwmon_channel_info *const wsa884x_hwmon_info[] = {
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops wsa884x_hwmon_ops = {
    .is_visible	= wsa884x_hwmon_is_visible,
    .read		= wsa884x_hwmon_read,
    };
    static const struct hwmon_chip_info wsa884x_hwmon_chip_info = {
    .ops	= &wsa884x_hwmon_ops,
    .info	= wsa884x_hwmon_info,
    };
#[no_mangle]
unsafe extern "C" fn wsa884x_reset_powerdown(data: *mut c_void) {
    static void wsa884x_reset_powerdown(void *data)
    {
    struct wsa884x_priv *wsa884x = data;
    if (wsa884x.sd_reset)
    reset_control_assert(wsa884x.sd_reset);
    else
    gpiod_direction_output(wsa884x.sd_n, 1);
    }
#[no_mangle]
unsafe extern "C" fn wsa884x_reset_deassert(wsa884x: *mut wsa884x_priv) {
    static void wsa884x_reset_deassert(struct wsa884x_priv *wsa884x)
    {
    if (wsa884x.sd_reset)
    reset_control_deassert(wsa884x.sd_reset);
    else
    gpiod_direction_output(wsa884x.sd_n, 0);
    }
#[no_mangle]
unsafe extern "C" fn wsa884x_regulator_disable(data: *mut c_void) {
    static void wsa884x_regulator_disable(void *data)
    {
    regulator_bulk_disable(WSA884X_SUPPLIES_NUM, data);
    }
#[no_mangle]
unsafe extern "C" fn wsa884x_get_reset(dev: *mut device, wsa884x: *mut wsa884x_priv) -> c_int {
    static int wsa884x_get_reset(struct device *dev, struct wsa884x_priv *wsa884x)
    {
    wsa884x.sd_reset = devm_reset_control_get_optional_shared(dev, core::ptr::null_mut());
    if (IS_ERR(wsa884x.sd_reset))
    return dev_err_probe(dev, PTR_ERR(wsa884x.sd_reset),
    "Failed to get reset\n");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: wsa884x->sd_reset) -> else {
    else if (wsa884x.sd_reset)
    return 0;
//
// else: NULL, so use the backwards compatible way for powerdown-gpios,
// which does not handle sharing GPIO properly.
//
    wsa884x.sd_n = devm_gpiod_get_optional(dev, "powerdown",
    GPIOD_OUT_HIGH);
    if (IS_ERR(wsa884x.sd_n))
    return dev_err_probe(dev, PTR_ERR(wsa884x.sd_n),
    "Shutdown Control GPIO not found\n");
    return 0;
    }
    static int wsa884x_probe(struct sdw_slave *pdev,
    const struct sdw_device_id *id)
    {
    struct device *dev = &pdev.dev;
    struct wsa884x_priv *wsa884x;
    unsigned int i;
    int ret;
    wsa884x = devm_kzalloc(dev, sizeof(*wsa884x), GFP_KERNEL);
    if (!wsa884x)
    return -ENOMEM;
    mutex_init(&wsa884x.sp_lock);
    for (i = 0; i < WSA884X_SUPPLIES_NUM; i++)
    wsa884x.supplies[i].supply = wsa884x_supply_name[i];
    ret = devm_regulator_bulk_get(dev, WSA884X_SUPPLIES_NUM,
    wsa884x.supplies);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to get regulators\n");
    ret = regulator_bulk_enable(WSA884X_SUPPLIES_NUM, wsa884x.supplies);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to enable regulators\n");
    ret = devm_add_action_or_reset(dev, wsa884x_regulator_disable,
    wsa884x.supplies);
    if (ret)
    return ret;
    ret = wsa884x_get_reset(dev, wsa884x);
    if (ret)
    return ret;
    dev_set_drvdata(dev, wsa884x);
    wsa884x.slave = pdev;
    wsa884x.dev = dev;
    wsa884x.dev_mode = WSA884X_SPEAKER;
    wsa884x.sconfig.ch_count = 1;
    wsa884x.sconfig.bps = 1;
    wsa884x.sconfig.direction = SDW_DATA_DIR_RX;
    wsa884x.sconfig.type = SDW_STREAM_PDM;
//
// Port map index starts with 0, however the data port for this codec
// are from index 1
//
    if (of_property_read_u32_array(dev.of_node, "qcom,port-mapping", &pdev.m_port_map[1],
    WSA884X_MAX_SWR_PORTS))
    dev_dbg(dev, "Static Port mapping not specified\n");
    pdev.prop.sink_ports = GENMASK(WSA884X_MAX_SWR_PORTS - 1, 0);
    pdev.prop.simple_clk_stop_capable = true;
    pdev.prop.sink_dpn_prop = wsa884x_sink_dpn_prop;
    pdev.prop.scp_int1_mask = SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;
    wsa884x_reset_deassert(wsa884x);
    ret = devm_add_action_or_reset(dev, wsa884x_reset_powerdown, wsa884x);
    if (ret)
    return ret;
    wsa884x.regmap = devm_regmap_init_sdw(pdev, &wsa884x_regmap_config);
    if (IS_ERR(wsa884x.regmap))
    return dev_err_probe(dev, PTR_ERR(wsa884x.regmap),
    "regmap_init failed\n");
// Start in cache-only until device is enumerated
    regcache_cache_only(wsa884x.regmap, true);
    if (IS_REACHABLE(CONFIG_HWMON)) {
    struct device *hwmon;
    hwmon = devm_hwmon_device_register_with_info(dev, "wsa884x",
    wsa884x,
    &wsa884x_hwmon_chip_info,
    core::ptr::null_mut());
    if (IS_ERR(hwmon))
    return dev_err_probe(dev, PTR_ERR(hwmon),
    "Failed to register hwmon sensor\n");
    }
    pm_runtime_set_autosuspend_delay(dev, 3000);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_mark_last_busy(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    return devm_snd_soc_register_component(dev,
    &wsa884x_component_drv,
    wsa884x_dais,
    ARRAY_SIZE(wsa884x_dais));
    }
#[no_mangle]
unsafe extern "C" fn wsa884x_runtime_suspend(dev: *mut device) -> c_int {
    static int wsa884x_runtime_suspend(struct device *dev)
    {
    struct regmap *regmap = dev_get_regmap(dev, core::ptr::null_mut());
    regcache_cache_only(regmap, true);
    regcache_mark_dirty(regmap);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wsa884x_runtime_resume(dev: *mut device) -> c_int {
    static int wsa884x_runtime_resume(struct device *dev)
    {
    struct regmap *regmap = dev_get_regmap(dev, core::ptr::null_mut());
    int ret;
    regcache_cache_only(regmap, false);
    ret = regcache_sync(regmap);
    if (ret) {
    regcache_cache_only(regmap, true);
    regcache_mark_dirty(regmap);
    return ret;
    }
    return 0;
    }
    static const struct dev_pm_ops wsa884x_pm_ops = {
    RUNTIME_PM_OPS(wsa884x_runtime_suspend, wsa884x_runtime_resume, core::ptr::null_mut())
    };
    static const struct sdw_device_id wsa884x_swr_id[] = {
    SDW_SLAVE_ENTRY(0x0217, 0x204, 0),
    {},
    };
    MODULE_DEVICE_TABLE(sdw, wsa884x_swr_id);
    static struct sdw_driver wsa884x_codec_driver = {
    .driver = {
    .name = "wsa884x-codec",
    .pm = pm_ptr(&wsa884x_pm_ops),
    },
    .probe = wsa884x_probe,
    .ops = &wsa884x_slave_ops,
    .id_table = wsa884x_swr_id,
    };
    module_sdw_driver(wsa884x_codec_driver);
    MODULE_AUTHOR("Krzysztof Kozlowski <krzysztof.kozlowski@linaro.org>");
    MODULE_DESCRIPTION("WSA884x codec driver");
    MODULE_LICENSE("GPL");
