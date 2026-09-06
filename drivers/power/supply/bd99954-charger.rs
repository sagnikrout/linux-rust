//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/power/supply/bd99954-charger.h
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
// Copyright (C) 2020 ROHM Semiconductors

pub const BD9995X_VSYS_PRECHARGE_OFFSET_MV: c_int = 200;
pub const BD99954_ID: c_uint = 0x346;
pub const BD99955_ID: c_uint = 0x221;
pub const BD99956_ID: c_uint = 0x331;
// Battery Charger Commands
pub const CHARGING_CURRENT: c_uint = 0x14;
pub const CHARGING_VOLTAGE: c_uint = 0x15;
pub const PROTECT_SET: c_uint = 0x3E;
pub const MAP_SET: c_uint = 0x3F;
// Extended commands
pub const CHGSTM_STATUS: c_uint = 0x100;
pub const VBAT_VSYS_STATUS: c_uint = 0x101;
pub const VBUS_VCC_STATUS: c_uint = 0x102;
pub const CHGOP_STATUS: c_uint = 0x103;
pub const WDT_STATUS: c_uint = 0x104;
pub const CUR_ILIM_VAL: c_uint = 0x105;
pub const SEL_ILIM_VAL: c_uint = 0x106;
pub const IBUS_LIM_SET: c_uint = 0x107;
pub const ICC_LIM_SET: c_uint = 0x108;
pub const IOTG_LIM_SET: c_uint = 0x109;
pub const VIN_CTRL_SET: c_uint = 0x10A;
pub const CHGOP_SET1: c_uint = 0x10B;
pub const CHGOP_SET2: c_uint = 0x10C;
pub const VBUSCLPS_TH_SET: c_uint = 0x10D;
pub const VCCCLPS_TH_SET: c_uint = 0x10E;
pub const CHGWDT_SET: c_uint = 0x10F;
pub const BATTWDT_SET: c_uint = 0x110;
pub const VSYSREG_SET: c_uint = 0x111;
pub const VSYSVAL_THH_SET: c_uint = 0x112;
pub const VSYSVAL_THL_SET: c_uint = 0x113;
pub const ITRICH_SET: c_uint = 0x114;
pub const IPRECH_SET: c_uint = 0x115;
pub const ICHG_SET: c_uint = 0x116;
pub const ITERM_SET: c_uint = 0x117;
pub const VPRECHG_TH_SET: c_uint = 0x118;
pub const VRBOOST_SET: c_uint = 0x119;
pub const VFASTCHG_REG_SET1: c_uint = 0x11A;
pub const VFASTCHG_REG_SET2: c_uint = 0x11B;
pub const VFASTCHG_REG_SET3: c_uint = 0x11C;
pub const VRECHG_SET: c_uint = 0x11D;
pub const VBATOVP_SET: c_uint = 0x11E;
pub const IBATSHORT_SET: c_uint = 0x11F;
pub const PROCHOT_CTRL_SET: c_uint = 0x120;
pub const PROCHOT_ICRIT_SET: c_uint = 0x121;
pub const PROCHOT_INORM_SET: c_uint = 0x122;
pub const PROCHOT_IDCHG_SET: c_uint = 0x123;
pub const PROCHOT_VSYS_SET: c_uint = 0x124;
pub const PMON_IOUT_CTRL_SET: c_uint = 0x125;
pub const PMON_DACIN_VAL: c_uint = 0x126;
pub const IOUT_DACIN_VAL: c_uint = 0x127;
pub const VCC_UCD_SET: c_uint = 0x128;
pub const VCC_UCD_STATUS: c_uint = 0x129;
pub const VCC_IDD_STATUS: c_uint = 0x12A;
pub const VCC_UCD_FCTRL_SET: c_uint = 0x12B;
pub const VCC_UCD_FCTRL_EN: c_uint = 0x12C;
pub const VBUS_UCD_SET: c_uint = 0x130;
pub const VBUS_UCD_STATUS: c_uint = 0x131;
pub const VBUS_IDD_STATUS: c_uint = 0x132;
pub const VBUS_UCD_FCTRL_SET: c_uint = 0x133;
pub const VBUS_UCD_FCTRL_EN: c_uint = 0x134;
pub const CHIP_ID: c_uint = 0x138;
pub const CHIP_REV: c_uint = 0x139;
pub const IC_SET1: c_uint = 0x13A;
pub const IC_SET2: c_uint = 0x13B;
pub const SYSTEM_STATUS: c_uint = 0x13C;
pub const SYSTEM_CTRL_SET: c_uint = 0x13D;
pub const VM_CTRL_SET: c_uint = 0x140;
pub const THERM_WINDOW_SET1: c_uint = 0x141;
pub const THERM_WINDOW_SET2: c_uint = 0x142;
pub const THERM_WINDOW_SET3: c_uint = 0x143;
pub const THERM_WINDOW_SET4: c_uint = 0x144;
pub const THERM_WINDOW_SET5: c_uint = 0x145;
pub const IBATP_TH_SET: c_uint = 0x146;
pub const IBATM_TH_SET: c_uint = 0x147;
pub const VBAT_TH_SET: c_uint = 0x148;
pub const THERM_TH_SET: c_uint = 0x149;
pub const IACP_TH_SET: c_uint = 0x14A;
pub const VACP_TH_SET: c_uint = 0x14B;
pub const VBUS_TH_SET: c_uint = 0x14C;
pub const VCC_TH_SET: c_uint = 0x14D;
pub const VSYS_TH_SET: c_uint = 0x14E;
pub const EXTIADP_TH_SET: c_uint = 0x14F;
pub const IBATP_VAL: c_uint = 0x150;
pub const IBATP_AVE_VAL: c_uint = 0x151;
pub const IBATM_VAL: c_uint = 0x152;
pub const IBATM_AVE_VAL: c_uint = 0x153;
pub const VBAT_VAL: c_uint = 0x154;
pub const VBAT_AVE_VAL: c_uint = 0x155;
pub const THERM_VAL: c_uint = 0x156;
pub const VTH_VAL: c_uint = 0x157;
pub const IACP_VAL: c_uint = 0x158;
pub const IACP_AVE_VAL: c_uint = 0x159;
pub const VACP_VAL: c_uint = 0x15A;
pub const VACP_AVE_VAL: c_uint = 0x15B;
pub const VBUS_VAL: c_uint = 0x15C;
pub const VBUS_AVE_VAL: c_uint = 0x15D;
pub const VCC_VAL: c_uint = 0x15E;
pub const VCC_AVE_VAL: c_uint = 0x15F;
pub const VSYS_VAL: c_uint = 0x160;
pub const VSYS_AVE_VAL: c_uint = 0x161;
pub const EXTIADP_VAL: c_uint = 0x162;
pub const EXTIADP_AVE_VAL: c_uint = 0x163;
pub const VACPCLPS_TH_SET: c_uint = 0x164;
pub const INT0_SET: c_uint = 0x168;
pub const INT1_SET: c_uint = 0x169;
pub const INT2_SET: c_uint = 0x16A;
pub const INT3_SET: c_uint = 0x16B;
pub const INT4_SET: c_uint = 0x16C;
pub const INT5_SET: c_uint = 0x16D;
pub const INT6_SET: c_uint = 0x16E;
pub const INT7_SET: c_uint = 0x16F;
pub const INT0_STATUS: c_uint = 0x170;
pub const INT1_STATUS: c_uint = 0x171;
pub const INT2_STATUS: c_uint = 0x172;
pub const INT3_STATUS: c_uint = 0x173;
pub const INT4_STATUS: c_uint = 0x174;
pub const INT5_STATUS: c_uint = 0x175;
pub const INT6_STATUS: c_uint = 0x176;
pub const INT7_STATUS: c_uint = 0x177;
pub const OTPREG0: c_uint = 0x17A;
pub const OTPREG1: c_uint = 0x17B;
pub const SMBREG: c_uint = 0x17C;
pub const DEBUG_MODE_SET: c_uint = 0x17F;
pub const DEBUG0x14: c_uint = 0x214;
pub const DEBUG0x1A: c_uint = 0x21A;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bd9995x_fields {
    F_PREV_CHGSTM_STATE, F_CHGSTM_STATE,
    F_VBAT_VSYS_STATUS,
    F_VBUS_VCC_STATUS,
    F_BATTEMP, F_VRECHG_DET, F_RBOOST_UV, F_RBOOSTS,
    F_THERMWDT_VAL, F_CHGWDT_VAL,
    F_CUR_ILIM_VAL,
    F_SEL_ILIM_VAL,
    F_IBUS_LIM_SET,
    F_ICC_LIM_SET,
    F_IOTG_LIM_SET,
    F_OTG_BOTH_EN,
    F_VRBOOST_TRIG,
    F_VRBOOST_EN,
    F_PP_BOTH_THRU,
    F_VIN_ORD,
    F_VBUS_EN,
    F_VCC_EN,
    F_VSYS_PRIORITY,
    F_PPC_SUB_CAP,
    F_PPC_CAP,
    F_DCP_2500_SEL,
    F_SDP_500_SEL,
    F_ILIM_AUTO_DISEN,
    F_VCC_BC_DISEN,
    F_VBUS_BC_DISEN,
    F_SDP_CHG_TRIG_EN,
    F_SDP_CHG_TRIG,
    F_AUTO_TOF,
    F_AUTO_FST,
    F_AUTO_RECH,
    F_ILIM_RESET_EN,
    F_DCDC_1MS_SEL,
    F_SEL_ILIM_DIV,
    F_BATT_LEARN,
    F_CHG_EN,
    F_USB_SUS,
    F_CHOP_SS_INIT,
    F_CHOP_ALL_INIT,
    F_DCDC_CLK_SEL,
    F_CHOP_SS,
    F_CHOP_ALL,
    F_VBUSCLPS_TH_SET,
    F_VCCCLPS_TH_SET,
    F_WDT_FST,
    F_WDT_PRE,
    F_WDT_IBAT_SHORT,
    F_WDT_THERM,
    F_VSYSREG_SET,
    F_VSYSVAL_THH_SET,
    F_VSYSVAL_THL_SET,
    F_ITRICH_SET,
    F_IPRECH_SET,
    F_ICHG_SET,
    F_ITERM_SET,
    F_VPRECHG_TH_SET,
    F_VRBOOST_SET,
    F_VFASTCHG_REG_SET1,
    F_VFASTCHG_REG_SET2,
    F_VFASTCHG_REG_SET3,
    F_VRECHG_SET,
    F_VBATOVP_SET,
    F_IBATM_SHORT_SET,
    F_PROCHOT_DG_SET,
    F_PROCHOT_ICRIT_DG_SET,
    F_PROCHOT_IDCHG_DG_SET,
    F_PROCHOT_EN,
    F_PROCHOT_ICRIT_SET,
    F_PROCHOT_INORM_SET,
    F_PROCHOT_IDCHG_SET,
    F_PROCHOT_VSYS_SET,
    F_IMON_INSEL,
    F_PMON_INSEL,
    F_IOUT_OUT_EN,
    F_IOUT_SOURCE_SEL,
    F_IOUT_GAIN_SET,
    F_PMON_OUT_EN,
    F_PMON_GAIN_SET,
    F_PMON_DACIN_VAL,
    F_IOUT_DACIN_VAL,
    F_VCC_BCSRETRY,
    F_VCC_ADCRTRY,
    F_VCC_USBDETEN,
    F_VCC_IDRDETEN,
    F_VCC_ENUMRDY,
    F_VCC_ADCPOLEN,
    F_VCC_DCDMODE,
    F_VCC_USB_SW_EN,
    F_VCC_USB_SW,
    F_VCC_DCDFAIL,
    F_VCC_CHGPORT,
    F_VCC_PUPDET,
    F_VCC_VBUS_VLD,
    F_VCC_CHGDET,
    F_VCC_OTGDET,
    F_VCC_VBINOP,
    F_VCC_EXTID,
    F_VCC_IDRDET,
    F_VCC_INDO,
    F_VCC_UCDSWEN,
    F_VCC_RREF_EN,
    F_VCC_DPPU_EN,
    F_VCC_DPREF_EN,
    F_VCC_DMREF_EN,
    F_VCC_DPDET_EN,
    F_VCC_DMDET_EN,
    F_VCC_DPSINK_EN,
    F_VCC_DMSINK_EN,
    F_VCC_DP_BUFF_EN,
    F_VCC_DM_BUFF_EN,
    F_VCC_EXTCLKENBL,
    F_VCC_PLSTESTEN,
    F_VCC_UCDSWEN_TSTENB,
    F_VCC_RREF_EN_TSTENB,
    F_VCC_DPPU_EN_TSTENB,
    F_VCC_DPREF_EN_TSTENB,
    F_VCC_DMREF_EN_TSTENB,
    F_VCC_DPDET_EN_TSTENB,
    F_VCC_DMDET_EN_TSTENB,
    F_VCC_DPSINK_EN_TSTENB,
    F_VCC_DMSINK_EN_TSTENB,
    F_VCC_DP_BUFF_EN_TSTENB,
    F_VCC_DM_BUFF_EN_TSTENB,
    F_VBUS_BCSRETRY,
    F_VBUS_ADCRTRY,
    F_VBUS_USBDETEN,
    F_VBUS_IDRDETEN,
    F_VBUS_ENUMRDY,
    F_VBUS_ADCPOLEN,
    F_VBUS_DCDMODE,
    F_VBUS_USB_SW_EN,
    F_VBUS_USB_SW,
    F_VBUS_DCDFAIL,
    F_VBUS_CHGPORT,
    F_VBUS_PUPDET,
    F_VBUS_VBUS_VLD,
    F_VBUS_CHGDET,
    F_VBUS_OTGDET,
    F_VBUS_VBINOP,
    F_VBUS_EXTID,
    F_VBUS_IDRDET,
    F_VBUS_INDO,

    F_VBUS_EXTCLKENBL,
    F_VBUS_PLSTESTEN,
    F_VBUS_UCDSWEN_TSTENB,
    F_VBUS_RREF_EN_TSTENB,
    F_VBUS_DPPU_EN_TSTENB,
    F_VBUS_DPREF_EN_TSTENB,
    F_VBUS_DMREF_EN_TSTENB,
    F_VBUS_DPDET_EN_TSTENB,
    F_VBUS_DMDET_EN_TSTENB,
    F_VBUS_DPSINK_EN_TSTENB,
    F_VBUS_DMSINK_EN_TSTENB,
    F_VBUS_DP_BUFF_EN_TSTENB,
    F_VBUS_DM_BUFF_EN_TSTENB,
    F_CHIP_ID,
    F_CHIP_REV,
    F_ONE_CELL_MODE,
    F_cell,
    F_VACP_AUTO_DISCHG,
    F_VACP_LOAD,
    F_ACOK_POL,
    F_ACOK_DISEN,
    F_DEBUG_SET1,
    F_DEBUG_SET0,
    F_MONRST_STATE,
    F_ALMRST_STATE,
    F_CHGRST_STATE,
    F_OTPLD_STATE,
    F_ALLRST_STATE,
    F_PROTECT_SET,
    F_MAP_SET,
    F_ADCINTERVAL,
    F_ADCMOD,
    F_ADCTMOD,
    F_EXTIADPEN,
    F_VSYSENB,
    F_VCCENB,
    F_VBUSENB,
    F_VACPENB,
    F_IACPENB,
    F_THERMENB,
    F_VBATENB,
    F_IBATMENB,
    F_IBATPENB,
    F_TMPTHR1B,
    F_TMPTHR1A,
    F_TMPTHR2B,
    F_TMPTHR2A,
    F_TMPTHR3B,
    F_TMPTHR3A,
    F_TMPTHR4B,
    F_TMPTHR4A,
    F_TMPTHR5B,
    F_TMPTHR5A,
    F_IBATP_TH_SET,
    F_IBATM_TH_SET,
    F_VBAT_TH_SET,
    F_THERM_TH_SET,
    F_IACP_TH_SET,
    F_VACP_TH_SET,
    F_VBUS_TH_SET,
    F_VCC_TH_SET,
    F_VSYS_TH_SET,
    F_EXTIADP_TH_SET,
    F_IBATP_VAL,
    F_IBATP_AVE_VAL,
    F_IBATM_VAL,
    F_IBATM_AVE_VAL,
    F_VBAT_VAL,
    F_VBAT_AVE_VAL,
    F_THERM_VAL,
    F_VTH_VAL,
    F_IACP_VAL,
    F_IACP_AVE_VAL,
    F_VACP_VAL,
    F_VACP_AVE_VAL,
    F_VBUS_VAL,
    F_VBUS_AVE_VAL,
    F_VCC_VAL,
    F_VCC_AVE_VAL,
    F_VSYS_VAL,
    F_VSYS_AVE_VAL,
    F_EXTIADP_VAL,
    F_EXTIADP_AVE_VAL,
    F_VACPCLPS_TH_SET,
    F_INT7_SET,
    F_INT6_SET,
    F_INT5_SET,
    F_INT4_SET,
    F_INT3_SET,
    F_INT2_SET,
    F_INT1_SET,
    F_INT0_SET,
    F_VBUS_RBUV_DET,
    F_VBUS_RBUV_RES,
    F_VBUS_TH_DET,
    F_VBUS_TH_RES,
    F_VBUS_IIN_MOD,
    F_VBUS_OV_DET,
    F_VBUS_OV_RES,
    F_VBUS_CLPS_DET,
    F_VBUS_CLPS,
    F_VBUS_DET,
    F_VBUS_RES,
    F_VCC_RBUV_DET,
    F_VCC_RBUV_RES,
    F_VCC_TH_DET,
    F_VCC_TH_RES,
    F_VCC_IIN_MOD,
    F_VCC_OVP_DET,
    F_VCC_OVP_RES,
    F_VCC_CLPS_DET,
    F_VCC_CLPS_RES,
    F_VCC_DET,
    F_VCC_RES,
    F_TH_DET,
    F_TH_RMV,
    F_TMP_OUT_DET,
    F_TMP_OUT_RES,
    F_VBAT_TH_DET,
    F_VBAT_TH_RES,
    F_IBAT_SHORT_DET,
    F_IBAT_SHORT_RES,
    F_VBAT_OV_DET,
    F_VBAT_OV_RES,
    F_BAT_ASSIST_DET,
    F_BAT_ASSIST_RES,
    F_VSYS_TH_DET,
    F_VSYS_TH_RES,
    F_VSYS_OV_DET,
    F_VSYS_OV_RES,
    F_VSYS_SHT_DET,
    F_VSYS_SHT_RES,
    F_VSYS_UV_DET,
    F_VSYS_UV_RES,
    F_OTP_LOAD_DONE,
    F_PWR_ON,
    F_EXTIADP_TRNS,
    F_EXTIADP_TH_DET,
    F_EXIADP_TH_RES,
    F_BAT_MNT_DET,
    F_BAT_MNT_RES,
    F_TSD_DET,
    F_TSD_RES,
    F_CHGWDT_EXP,
    F_THERMWDT_EXP,
    F_TMP_TRNS,
    F_CHG_TRNS,
    F_VBUS_UCD_PORT_DET,
    F_VBUS_UCD_UCHG_DET,
    F_VBUS_UCD_URID_RMV,
    F_VBUS_UCD_OTG_DET,
    F_VBUS_UCD_URID_MOD,
    F_VCC_UCD_PORT_DET,
    F_VCC_UCD_UCHG_DET,
    F_VCC_UCD_URID_RMV,
    F_VCC_UCD_OTG_DET,
    F_VCC_UCD_URID_MOD,
    F_PROCHOT_DET,
    F_PROCHOT_RES,
    F_VACP_DET,
    F_VACP_RES,
    F_VACP_TH_DET,
    F_VACP_TH_RES,
    F_IACP_TH_DET,
    F_IACP_THE_RES,
    F_THERM_TH_DET,
    F_THERM_TH_RES,
    F_IBATM_TH_DET,
    F_IBATM_TH_RES,
    F_IBATP_TH_DET,
    F_IBATP_TH_RES,
    F_INT7_STATUS,
    F_INT6_STATUS,
    F_INT5_STATUS,
    F_INT4_STATUS,
    F_INT3_STATUS,
    F_INT2_STATUS,
    F_INT1_STATUS,
    F_INT0_STATUS,
    F_ILIM_DECREASE,
    F_RESERVE_OTPREG1,
    F_POWER_SAVE_MODE,
    F_DEBUG_MODE_SET,
    F_DEBUG0x14,
    F_DEBUG0x1A,
    F_MAX_FIELDS
}

// CHGSTM_STATEs
pub const CHGSTM_SUSPEND: c_uint = 0x00;
pub const CHGSTM_TRICKLE_CHARGE: c_uint = 0x01;
pub const CHGSTM_PRE_CHARGE: c_uint = 0x02;
pub const CHGSTM_FAST_CHARGE: c_uint = 0x03;
pub const CHGSTM_TOP_OFF: c_uint = 0x04;
pub const CHGSTM_DONE: c_uint = 0x05;
pub const CHGSTM_OTG: c_uint = 0x08;
pub const CHGSTM_OTG_DONE: c_uint = 0x09;
pub const CHGSTM_TEMPERATURE_ERROR_1: c_uint = 0x10;
pub const CHGSTM_TEMPERATURE_ERROR_2: c_uint = 0x11;
pub const CHGSTM_TEMPERATURE_ERROR_3: c_uint = 0x12;
pub const CHGSTM_TEMPERATURE_ERROR_4: c_uint = 0x13;
pub const CHGSTM_TEMPERATURE_ERROR_5: c_uint = 0x14;
pub const CHGSTM_TEMPERATURE_ERROR_6: c_uint = 0x15;
pub const CHGSTM_TEMPERATURE_ERROR_7: c_uint = 0x18;
pub const CHGSTM_THERMAL_SHUT_DOWN_1: c_uint = 0x20;
pub const CHGSTM_THERMAL_SHUT_DOWN_2: c_uint = 0x21;
pub const CHGSTM_THERMAL_SHUT_DOWN_3: c_uint = 0x22;
pub const CHGSTM_THERMAL_SHUT_DOWN_4: c_uint = 0x23;
pub const CHGSTM_THERMAL_SHUT_DOWN_5: c_uint = 0x24;
pub const CHGSTM_THERMAL_SHUT_DOWN_6: c_uint = 0x25;
pub const CHGSTM_THERMAL_SHUT_DOWN_7: c_uint = 0x28;
pub const CHGSTM_BATTERY_ERROR: c_uint = 0x40;
// VBAT_VSYS_STATUS

// VBUS_VCC_STATUS

// Interrupt set/status definitions
// INT 0

pub const INT0_ALL: c_uint = 0xff;
// INT 1

// INT 2

// INT 3

// INT 4

// INT 5

// INT 6

// INT 7

// SYSTEM_CTRL_SET

// F_BATTEMP
pub const ROOM: c_uint = 0x0;
pub const HOT1: c_uint = 0x1;
pub const HOT2: c_uint = 0x2;
pub const HOT3: c_uint = 0x3;
pub const COLD1: c_uint = 0x4;
pub const COLD2: c_uint = 0x5;
pub const TEMP_DIS: c_uint = 0x6;
pub const BATT_OPEN: c_uint = 0x7;
