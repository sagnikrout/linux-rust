//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/omap2/omapfb/dss/hdmi5_core.h
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
// HDMI driver definition for TI OMAP5 processors.
//
// Copyright (C) 2011-2012 Texas Instruments Incorporated - https://www.ti.com
//

// HDMI IP Core System
// HDMI Identification
pub const HDMI_CORE_DESIGN_ID: c_uint = 0x00000;
pub const HDMI_CORE_REVISION_ID: c_uint = 0x00004;
pub const HDMI_CORE_PRODUCT_ID0: c_uint = 0x00008;
pub const HDMI_CORE_PRODUCT_ID1: c_uint = 0x0000C;
pub const HDMI_CORE_CONFIG0_ID: c_uint = 0x00010;
pub const HDMI_CORE_CONFIG1_ID: c_uint = 0x00014;
pub const HDMI_CORE_CONFIG2_ID: c_uint = 0x00018;
pub const HDMI_CORE_CONFIG3_ID: c_uint = 0x0001C;
// HDMI Interrupt
pub const HDMI_CORE_IH_FC_STAT0: c_uint = 0x00400;
pub const HDMI_CORE_IH_FC_STAT1: c_uint = 0x00404;
pub const HDMI_CORE_IH_FC_STAT2: c_uint = 0x00408;
pub const HDMI_CORE_IH_AS_STAT0: c_uint = 0x0040C;
pub const HDMI_CORE_IH_PHY_STAT0: c_uint = 0x00410;
pub const HDMI_CORE_IH_I2CM_STAT0: c_uint = 0x00414;
pub const HDMI_CORE_IH_CEC_STAT0: c_uint = 0x00418;
pub const HDMI_CORE_IH_VP_STAT0: c_uint = 0x0041C;
pub const HDMI_CORE_IH_I2CMPHY_STAT0: c_uint = 0x00420;
pub const HDMI_CORE_IH_MUTE: c_uint = 0x007FC;
// HDMI Video Sampler
pub const HDMI_CORE_TX_INVID0: c_uint = 0x00800;
pub const HDMI_CORE_TX_INSTUFFING: c_uint = 0x00804;
pub const HDMI_CORE_TX_RGYDATA0: c_uint = 0x00808;
pub const HDMI_CORE_TX_RGYDATA1: c_uint = 0x0080C;
pub const HDMI_CORE_TX_RCRDATA0: c_uint = 0x00810;
pub const HDMI_CORE_TX_RCRDATA1: c_uint = 0x00814;
pub const HDMI_CORE_TX_BCBDATA0: c_uint = 0x00818;
pub const HDMI_CORE_TX_BCBDATA1: c_uint = 0x0081C;
// HDMI Video Packetizer
pub const HDMI_CORE_VP_STATUS: c_uint = 0x02000;
pub const HDMI_CORE_VP_PR_CD: c_uint = 0x02004;
pub const HDMI_CORE_VP_STUFF: c_uint = 0x02008;
pub const HDMI_CORE_VP_REMAP: c_uint = 0x0200C;
pub const HDMI_CORE_VP_CONF: c_uint = 0x02010;
pub const HDMI_CORE_VP_STAT: c_uint = 0x02014;
pub const HDMI_CORE_VP_INT: c_uint = 0x02018;
pub const HDMI_CORE_VP_MASK: c_uint = 0x0201C;
pub const HDMI_CORE_VP_POL: c_uint = 0x02020;
// Frame Composer
pub const HDMI_CORE_FC_INVIDCONF: c_uint = 0x04000;
pub const HDMI_CORE_FC_INHACTIV0: c_uint = 0x04004;
pub const HDMI_CORE_FC_INHACTIV1: c_uint = 0x04008;
pub const HDMI_CORE_FC_INHBLANK0: c_uint = 0x0400C;
pub const HDMI_CORE_FC_INHBLANK1: c_uint = 0x04010;
pub const HDMI_CORE_FC_INVACTIV0: c_uint = 0x04014;
pub const HDMI_CORE_FC_INVACTIV1: c_uint = 0x04018;
pub const HDMI_CORE_FC_INVBLANK: c_uint = 0x0401C;
pub const HDMI_CORE_FC_HSYNCINDELAY0: c_uint = 0x04020;
pub const HDMI_CORE_FC_HSYNCINDELAY1: c_uint = 0x04024;
pub const HDMI_CORE_FC_HSYNCINWIDTH0: c_uint = 0x04028;
pub const HDMI_CORE_FC_HSYNCINWIDTH1: c_uint = 0x0402C;
pub const HDMI_CORE_FC_VSYNCINDELAY: c_uint = 0x04030;
pub const HDMI_CORE_FC_VSYNCINWIDTH: c_uint = 0x04034;
pub const HDMI_CORE_FC_INFREQ0: c_uint = 0x04038;
pub const HDMI_CORE_FC_INFREQ1: c_uint = 0x0403C;
pub const HDMI_CORE_FC_INFREQ2: c_uint = 0x04040;
pub const HDMI_CORE_FC_CTRLDUR: c_uint = 0x04044;
pub const HDMI_CORE_FC_EXCTRLDUR: c_uint = 0x04048;
pub const HDMI_CORE_FC_EXCTRLSPAC: c_uint = 0x0404C;
pub const HDMI_CORE_FC_CH0PREAM: c_uint = 0x04050;
pub const HDMI_CORE_FC_CH1PREAM: c_uint = 0x04054;
pub const HDMI_CORE_FC_CH2PREAM: c_uint = 0x04058;
pub const HDMI_CORE_FC_AVICONF3: c_uint = 0x0405C;
pub const HDMI_CORE_FC_GCP: c_uint = 0x04060;
pub const HDMI_CORE_FC_AVICONF0: c_uint = 0x04064;
pub const HDMI_CORE_FC_AVICONF1: c_uint = 0x04068;
pub const HDMI_CORE_FC_AVICONF2: c_uint = 0x0406C;
pub const HDMI_CORE_FC_AVIVID: c_uint = 0x04070;
pub const HDMI_CORE_FC_AVIETB0: c_uint = 0x04074;
pub const HDMI_CORE_FC_AVIETB1: c_uint = 0x04078;
pub const HDMI_CORE_FC_AVISBB0: c_uint = 0x0407C;
pub const HDMI_CORE_FC_AVISBB1: c_uint = 0x04080;
pub const HDMI_CORE_FC_AVIELB0: c_uint = 0x04084;
pub const HDMI_CORE_FC_AVIELB1: c_uint = 0x04088;
pub const HDMI_CORE_FC_AVISRB0: c_uint = 0x0408C;
pub const HDMI_CORE_FC_AVISRB1: c_uint = 0x04090;
pub const HDMI_CORE_FC_AUDICONF0: c_uint = 0x04094;
pub const HDMI_CORE_FC_AUDICONF1: c_uint = 0x04098;
pub const HDMI_CORE_FC_AUDICONF2: c_uint = 0x0409C;
pub const HDMI_CORE_FC_AUDICONF3: c_uint = 0x040A0;
pub const HDMI_CORE_FC_VSDIEEEID0: c_uint = 0x040A4;
pub const HDMI_CORE_FC_VSDSIZE: c_uint = 0x040A8;
pub const HDMI_CORE_FC_VSDIEEEID1: c_uint = 0x040C0;
pub const HDMI_CORE_FC_VSDIEEEID2: c_uint = 0x040C4;

pub const HDMI_CORE_FC_SPDDEVICEINF: c_uint = 0x04188;
pub const HDMI_CORE_FC_AUDSCONF: c_uint = 0x0418C;
pub const HDMI_CORE_FC_AUDSSTAT: c_uint = 0x04190;
pub const HDMI_CORE_FC_AUDSV: c_uint = 0x04194;
pub const HDMI_CORE_FC_AUDSU: c_uint = 0x04198;

pub const HDMI_CORE_FC_CTRLQHIGH: c_uint = 0x041CC;
pub const HDMI_CORE_FC_CTRLQLOW: c_uint = 0x041D0;
pub const HDMI_CORE_FC_ACP0: c_uint = 0x041D4;

pub const HDMI_CORE_FC_ISCR1_0: c_uint = 0x04248;

pub const HDMI_CORE_FC_DATAUTO0: c_uint = 0x042CC;
pub const HDMI_CORE_FC_DATAUTO1: c_uint = 0x042D0;
pub const HDMI_CORE_FC_DATAUTO2: c_uint = 0x042D4;
pub const HDMI_CORE_FC_DATMAN: c_uint = 0x042D8;
pub const HDMI_CORE_FC_DATAUTO3: c_uint = 0x042DC;

pub const HDMI_CORE_FC_STAT0: c_uint = 0x04340;
pub const HDMI_CORE_FC_INT0: c_uint = 0x04344;
pub const HDMI_CORE_FC_MASK0: c_uint = 0x04348;
pub const HDMI_CORE_FC_POL0: c_uint = 0x0434C;
pub const HDMI_CORE_FC_STAT1: c_uint = 0x04350;
pub const HDMI_CORE_FC_INT1: c_uint = 0x04354;
pub const HDMI_CORE_FC_MASK1: c_uint = 0x04358;
pub const HDMI_CORE_FC_POL1: c_uint = 0x0435C;
pub const HDMI_CORE_FC_STAT2: c_uint = 0x04360;
pub const HDMI_CORE_FC_INT2: c_uint = 0x04364;
pub const HDMI_CORE_FC_MASK2: c_uint = 0x04368;
pub const HDMI_CORE_FC_POL2: c_uint = 0x0436C;
pub const HDMI_CORE_FC_PRCONF: c_uint = 0x04380;
pub const HDMI_CORE_FC_GMD_STAT: c_uint = 0x04400;
pub const HDMI_CORE_FC_GMD_EN: c_uint = 0x04404;
pub const HDMI_CORE_FC_GMD_UP: c_uint = 0x04408;
pub const HDMI_CORE_FC_GMD_CONF: c_uint = 0x0440C;
pub const HDMI_CORE_FC_GMD_HB: c_uint = 0x04410;

pub const HDMI_CORE_FC_DBGFORCE: c_uint = 0x04800;
pub const HDMI_CORE_FC_DBGAUD0CH0: c_uint = 0x04804;
pub const HDMI_CORE_FC_DBGAUD1CH0: c_uint = 0x04808;
pub const HDMI_CORE_FC_DBGAUD2CH0: c_uint = 0x0480C;
pub const HDMI_CORE_FC_DBGAUD0CH1: c_uint = 0x04810;
pub const HDMI_CORE_FC_DBGAUD1CH1: c_uint = 0x04814;
pub const HDMI_CORE_FC_DBGAUD2CH1: c_uint = 0x04818;
pub const HDMI_CORE_FC_DBGAUD0CH2: c_uint = 0x0481C;
pub const HDMI_CORE_FC_DBGAUD1CH2: c_uint = 0x04820;
pub const HDMI_CORE_FC_DBGAUD2CH2: c_uint = 0x04824;
pub const HDMI_CORE_FC_DBGAUD0CH3: c_uint = 0x04828;
pub const HDMI_CORE_FC_DBGAUD1CH3: c_uint = 0x0482C;
pub const HDMI_CORE_FC_DBGAUD2CH3: c_uint = 0x04830;
pub const HDMI_CORE_FC_DBGAUD0CH4: c_uint = 0x04834;
pub const HDMI_CORE_FC_DBGAUD1CH4: c_uint = 0x04838;
pub const HDMI_CORE_FC_DBGAUD2CH4: c_uint = 0x0483C;
pub const HDMI_CORE_FC_DBGAUD0CH5: c_uint = 0x04840;
pub const HDMI_CORE_FC_DBGAUD1CH5: c_uint = 0x04844;
pub const HDMI_CORE_FC_DBGAUD2CH5: c_uint = 0x04848;
pub const HDMI_CORE_FC_DBGAUD0CH6: c_uint = 0x0484C;
pub const HDMI_CORE_FC_DBGAUD1CH6: c_uint = 0x04850;
pub const HDMI_CORE_FC_DBGAUD2CH6: c_uint = 0x04854;
pub const HDMI_CORE_FC_DBGAUD0CH7: c_uint = 0x04858;
pub const HDMI_CORE_FC_DBGAUD1CH7: c_uint = 0x0485C;
pub const HDMI_CORE_FC_DBGAUD2CH7: c_uint = 0x04860;
pub const HDMI_CORE_FC_DBGTMDS0: c_uint = 0x04864;
pub const HDMI_CORE_FC_DBGTMDS1: c_uint = 0x04868;
pub const HDMI_CORE_FC_DBGTMDS2: c_uint = 0x0486C;
pub const HDMI_CORE_PHY_MASK0: c_uint = 0x0C018;
pub const HDMI_CORE_PHY_I2CM_INT_ADDR: c_uint = 0x0C09C;
pub const HDMI_CORE_PHY_I2CM_CTLINT_ADDR: c_uint = 0x0C0A0;
// HDMI Audio
pub const HDMI_CORE_AUD_CONF0: c_uint = 0x0C400;
pub const HDMI_CORE_AUD_CONF1: c_uint = 0x0C404;
pub const HDMI_CORE_AUD_INT: c_uint = 0x0C408;
pub const HDMI_CORE_AUD_N1: c_uint = 0x0C800;
pub const HDMI_CORE_AUD_N2: c_uint = 0x0C804;
pub const HDMI_CORE_AUD_N3: c_uint = 0x0C808;
pub const HDMI_CORE_AUD_CTS1: c_uint = 0x0C80C;
pub const HDMI_CORE_AUD_CTS2: c_uint = 0x0C810;
pub const HDMI_CORE_AUD_CTS3: c_uint = 0x0C814;
pub const HDMI_CORE_AUD_INCLKFS: c_uint = 0x0C818;
pub const HDMI_CORE_AUD_CC08: c_uint = 0x0CC08;
pub const HDMI_CORE_AUD_GP_CONF0: c_uint = 0x0D400;
pub const HDMI_CORE_AUD_GP_CONF1: c_uint = 0x0D404;
pub const HDMI_CORE_AUD_GP_CONF2: c_uint = 0x0D408;
pub const HDMI_CORE_AUD_D010: c_uint = 0x0D010;
pub const HDMI_CORE_AUD_GP_STAT: c_uint = 0x0D40C;
pub const HDMI_CORE_AUD_GP_INT: c_uint = 0x0D410;
pub const HDMI_CORE_AUD_GP_POL: c_uint = 0x0D414;
pub const HDMI_CORE_AUD_GP_MASK: c_uint = 0x0D418;
// HDMI Main Controller
pub const HDMI_CORE_MC_CLKDIS: c_uint = 0x10004;
pub const HDMI_CORE_MC_SWRSTZREQ: c_uint = 0x10008;
pub const HDMI_CORE_MC_FLOWCTRL: c_uint = 0x10010;
pub const HDMI_CORE_MC_PHYRSTZ: c_uint = 0x10014;
pub const HDMI_CORE_MC_LOCKONCLOCK: c_uint = 0x10018;
// HDMI COLOR SPACE CONVERTER
pub const HDMI_CORE_CSC_CFG: c_uint = 0x10400;
pub const HDMI_CORE_CSC_SCALE: c_uint = 0x10404;
pub const HDMI_CORE_CSC_COEF_A1_MSB: c_uint = 0x10408;
pub const HDMI_CORE_CSC_COEF_A1_LSB: c_uint = 0x1040C;
pub const HDMI_CORE_CSC_COEF_A2_MSB: c_uint = 0x10410;
pub const HDMI_CORE_CSC_COEF_A2_LSB: c_uint = 0x10414;
pub const HDMI_CORE_CSC_COEF_A3_MSB: c_uint = 0x10418;
pub const HDMI_CORE_CSC_COEF_A3_LSB: c_uint = 0x1041C;
pub const HDMI_CORE_CSC_COEF_A4_MSB: c_uint = 0x10420;
pub const HDMI_CORE_CSC_COEF_A4_LSB: c_uint = 0x10424;
pub const HDMI_CORE_CSC_COEF_B1_MSB: c_uint = 0x10428;
pub const HDMI_CORE_CSC_COEF_B1_LSB: c_uint = 0x1042C;
pub const HDMI_CORE_CSC_COEF_B2_MSB: c_uint = 0x10430;
pub const HDMI_CORE_CSC_COEF_B2_LSB: c_uint = 0x10434;
pub const HDMI_CORE_CSC_COEF_B3_MSB: c_uint = 0x10438;
pub const HDMI_CORE_CSC_COEF_B3_LSB: c_uint = 0x1043C;
pub const HDMI_CORE_CSC_COEF_B4_MSB: c_uint = 0x10440;
pub const HDMI_CORE_CSC_COEF_B4_LSB: c_uint = 0x10444;
pub const HDMI_CORE_CSC_COEF_C1_MSB: c_uint = 0x10448;
pub const HDMI_CORE_CSC_COEF_C1_LSB: c_uint = 0x1044C;
pub const HDMI_CORE_CSC_COEF_C2_MSB: c_uint = 0x10450;
pub const HDMI_CORE_CSC_COEF_C2_LSB: c_uint = 0x10454;
pub const HDMI_CORE_CSC_COEF_C3_MSB: c_uint = 0x10458;
pub const HDMI_CORE_CSC_COEF_C3_LSB: c_uint = 0x1045C;
pub const HDMI_CORE_CSC_COEF_C4_MSB: c_uint = 0x10460;
pub const HDMI_CORE_CSC_COEF_C4_LSB: c_uint = 0x10464;
// HDMI HDCP
pub const HDMI_CORE_HDCP_MASK: c_uint = 0x14020;
// HDMI CEC
pub const HDMI_CORE_CEC_MASK: c_uint = 0x17408;
// HDMI I2C Master
pub const HDMI_CORE_I2CM_SLAVE: c_uint = 0x157C8;
pub const HDMI_CORE_I2CM_ADDRESS: c_uint = 0x157CC;
pub const HDMI_CORE_I2CM_DATAO: c_uint = 0x157D0;

pub const HDMI_CORE_I2CM_OPERATION: c_uint = 0x157D8;
pub const HDMI_CORE_I2CM_INT: c_uint = 0x157DC;
pub const HDMI_CORE_I2CM_CTLINT: c_uint = 0x157E0;
pub const HDMI_CORE_I2CM_DIV: c_uint = 0x157E4;
pub const HDMI_CORE_I2CM_SEGADDR: c_uint = 0x157E8;
pub const HDMI_CORE_I2CM_SOFTRSTZ: c_uint = 0x157EC;
pub const HDMI_CORE_I2CM_SEGPTR: c_uint = 0x157F0;
pub const HDMI_CORE_I2CM_SS_SCL_HCNT_1_ADDR: c_uint = 0x157F4;
pub const HDMI_CORE_I2CM_SS_SCL_HCNT_0_ADDR: c_uint = 0x157F8;
pub const HDMI_CORE_I2CM_SS_SCL_LCNT_1_ADDR: c_uint = 0x157FC;
pub const HDMI_CORE_I2CM_SS_SCL_LCNT_0_ADDR: c_uint = 0x15800;
pub const HDMI_CORE_I2CM_FS_SCL_HCNT_1_ADDR: c_uint = 0x15804;
pub const HDMI_CORE_I2CM_FS_SCL_HCNT_0_ADDR: c_uint = 0x15808;
pub const HDMI_CORE_I2CM_FS_SCL_LCNT_1_ADDR: c_uint = 0x1580C;
pub const HDMI_CORE_I2CM_FS_SCL_LCNT_0_ADDR: c_uint = 0x15810;
pub const HDMI_CORE_I2CM_SDA_HOLD_ADDR: c_uint = 0x15814;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_core_packet_mode {
    HDMI_PACKETMODERESERVEDVALUE = 0,
    HDMI_PACKETMODE24BITPERPIXEL = 4,
    HDMI_PACKETMODE30BITPERPIXEL = 5,
    HDMI_PACKETMODE36BITPERPIXEL = 6,
    HDMI_PACKETMODE48BITPERPIXEL = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_core_vid_config {
    pub v_fc_config: hdmi_config,
    pub packet_mode: hdmi_core_packet_mode,
    pub data_enable_pol: c_int,
    pub vblank_osc: c_int,
    pub hblank: c_int,
    pub vblank: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csc_table {
    pub a4: u16 a1, a2, a3,,
    pub b4: u16 b1, b2, b3,,
    pub c4: u16 c1, c2, c3,,
}

extern "C" {
    pub fn hdmi5_read_edid(core: *mut hdmi_core_data, edid: *mut u8, len: c_int) -> c_int;
}
extern "C" {
    pub fn hdmi5_core_dump(core: *mut hdmi_core_data, s: *mut seq_file);
}
extern "C" {
    pub fn hdmi5_core_init(pdev: *mut platform_device, core: *mut hdmi_core_data) -> c_int;
}
