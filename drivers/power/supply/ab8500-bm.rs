//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/power/supply/ab8500-bm.h
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
// System control 2 register offsets.
// bank = 0x02
//
pub const AB8500_MAIN_WDOG_CTRL_REG: c_uint = 0x01;
pub const AB8500_LOW_BAT_REG: c_uint = 0x03;
pub const AB8500_BATT_OK_REG: c_uint = 0x04;
//
// USB/ULPI register offsets
// Bank : 0x5
//
pub const AB8500_USB_LINE_STAT_REG: c_uint = 0x80;
pub const AB8500_USB_LINE_CTRL2_REG: c_uint = 0x82;
pub const AB8500_USB_LINK1_STAT_REG: c_uint = 0x94;
//
// Charger / status register offfsets
// Bank : 0x0B
//
pub const AB8500_CH_STATUS1_REG: c_uint = 0x00;
pub const AB8500_CH_STATUS2_REG: c_uint = 0x01;
pub const AB8500_CH_USBCH_STAT1_REG: c_uint = 0x02;
pub const AB8500_CH_USBCH_STAT2_REG: c_uint = 0x03;
pub const AB8540_CH_USBCH_STAT3_REG: c_uint = 0x04;
pub const AB8500_CH_STAT_REG: c_uint = 0x05;
//
// Charger / control register offfsets
// Bank : 0x0B
//
pub const AB8500_CH_VOLT_LVL_REG: c_uint = 0x40;
pub const AB8500_CH_VOLT_LVL_MAX_REG: c_uint = 0x41  /*Only in Cut2.0*/;
pub const AB8500_CH_OPT_CRNTLVL_REG: c_uint = 0x42;
pub const AB8500_CH_OPT_CRNTLVL_MAX_REG: c_uint = 0x43  /*Only in Cut2.0*/;
pub const AB8500_CH_WD_TIMER_REG: c_uint = 0x50;
pub const AB8500_CHARG_WD_CTRL: c_uint = 0x51;
pub const AB8500_BTEMP_HIGH_TH: c_uint = 0x52;
pub const AB8500_LED_INDICATOR_PWM_CTRL: c_uint = 0x53;
pub const AB8500_LED_INDICATOR_PWM_DUTY: c_uint = 0x54;
pub const AB8500_BATT_OVV: c_uint = 0x55;
pub const AB8500_CHARGER_CTRL: c_uint = 0x56;
pub const AB8500_BAT_CTRL_CURRENT_SOURCE: c_uint = 0x60  /*Only in Cut2.0*/;
//
// Charger / main control register offsets
// Bank : 0x0B
//
pub const AB8500_MCH_CTRL1: c_uint = 0x80;
pub const AB8500_MCH_CTRL2: c_uint = 0x81;
pub const AB8500_MCH_IPT_CURLVL_REG: c_uint = 0x82;
pub const AB8500_CH_WD_REG: c_uint = 0x83;
//
// Charger / USB control register offsets
// Bank : 0x0B
//
pub const AB8500_USBCH_CTRL1_REG: c_uint = 0xC0;
pub const AB8500_USBCH_CTRL2_REG: c_uint = 0xC1;
pub const AB8500_USBCH_IPT_CRNTLVL_REG: c_uint = 0xC2;
pub const AB8540_USB_PP_MODE_REG: c_uint = 0xC5;
pub const AB8540_USB_PP_CHR_REG: c_uint = 0xC6;
//
// Gas Gauge register offsets
// Bank : 0x0C
//
pub const AB8500_GASG_CC_CTRL_REG: c_uint = 0x00;
pub const AB8500_GASG_CC_ACCU1_REG: c_uint = 0x01;
pub const AB8500_GASG_CC_ACCU2_REG: c_uint = 0x02;
pub const AB8500_GASG_CC_ACCU3_REG: c_uint = 0x03;
pub const AB8500_GASG_CC_ACCU4_REG: c_uint = 0x04;
pub const AB8500_GASG_CC_SMPL_CNTRL_REG: c_uint = 0x05;
pub const AB8500_GASG_CC_SMPL_CNTRH_REG: c_uint = 0x06;
pub const AB8500_GASG_CC_SMPL_CNVL_REG: c_uint = 0x07;
pub const AB8500_GASG_CC_SMPL_CNVH_REG: c_uint = 0x08;
pub const AB8500_GASG_CC_CNTR_AVGOFF_REG: c_uint = 0x09;
pub const AB8500_GASG_CC_OFFSET_REG: c_uint = 0x0A;
pub const AB8500_GASG_CC_NCOV_ACCU: c_uint = 0x10;
pub const AB8500_GASG_CC_NCOV_ACCU_CTRL: c_uint = 0x11;
pub const AB8500_GASG_CC_NCOV_ACCU_LOW: c_uint = 0x12;
pub const AB8500_GASG_CC_NCOV_ACCU_MED: c_uint = 0x13;
pub const AB8500_GASG_CC_NCOV_ACCU_HIGH: c_uint = 0x14;
//
// Interrupt register offsets
// Bank : 0x0E
//
pub const AB8500_IT_SOURCE2_REG: c_uint = 0x01;
pub const AB8500_IT_SOURCE21_REG: c_uint = 0x14;
//
// RTC register offsets
// Bank: 0x0F
//
pub const AB8500_RTC_BACKUP_CHG_REG: c_uint = 0x0C;
pub const AB8500_RTC_CC_CONF_REG: c_uint = 0x01;
pub const AB8500_RTC_CTRL_REG: c_uint = 0x0B;
pub const AB8500_RTC_CTRL1_REG: c_uint = 0x11;
//
// OTP register offsets
// Bank : 0x15
//
pub const AB8500_OTP_CONF_15: c_uint = 0x0E;
// GPADC constants from AB8500 spec, UM0836
pub const ADC_RESOLUTION: c_int = 1024;
pub const ADC_CH_MAIN_MIN: c_int = 0;
pub const ADC_CH_MAIN_MAX: c_int = 20030;
pub const ADC_CH_VBUS_MIN: c_int = 0;
pub const ADC_CH_VBUS_MAX: c_int = 20030;
pub const ADC_CH_VBAT_MIN: c_int = 2300;
pub const ADC_CH_VBAT_MAX: c_int = 4800;
pub const ADC_CH_BKBAT_MIN: c_int = 0;
pub const ADC_CH_BKBAT_MAX: c_int = 3200;
// Main charge i/p current
pub const MAIN_CH_IP_CUR_0P9A: c_uint = 0x80;
pub const MAIN_CH_IP_CUR_1P0A: c_uint = 0x90;
pub const MAIN_CH_IP_CUR_1P1A: c_uint = 0xA0;
pub const MAIN_CH_IP_CUR_1P2A: c_uint = 0xB0;
pub const MAIN_CH_IP_CUR_1P3A: c_uint = 0xC0;
pub const MAIN_CH_IP_CUR_1P4A: c_uint = 0xD0;
pub const MAIN_CH_IP_CUR_1P5A: c_uint = 0xE0;
// ChVoltLevel
pub const CH_VOL_LVL_3P5: c_uint = 0x00;
pub const CH_VOL_LVL_4P0: c_uint = 0x14;
pub const CH_VOL_LVL_4P05: c_uint = 0x16;
pub const CH_VOL_LVL_4P1: c_uint = 0x1B;
pub const CH_VOL_LVL_4P15: c_uint = 0x20;
pub const CH_VOL_LVL_4P2: c_uint = 0x25;
pub const CH_VOL_LVL_4P6: c_uint = 0x4D;
// ChOutputCurrentLevel
pub const CH_OP_CUR_LVL_0P1: c_uint = 0x00;
pub const CH_OP_CUR_LVL_0P2: c_uint = 0x01;
pub const CH_OP_CUR_LVL_0P3: c_uint = 0x02;
pub const CH_OP_CUR_LVL_0P4: c_uint = 0x03;
pub const CH_OP_CUR_LVL_0P5: c_uint = 0x04;
pub const CH_OP_CUR_LVL_0P6: c_uint = 0x05;
pub const CH_OP_CUR_LVL_0P7: c_uint = 0x06;
pub const CH_OP_CUR_LVL_0P8: c_uint = 0x07;
pub const CH_OP_CUR_LVL_0P9: c_uint = 0x08;
pub const CH_OP_CUR_LVL_1P4: c_uint = 0x0D;
pub const CH_OP_CUR_LVL_1P5: c_uint = 0x0E;
pub const CH_OP_CUR_LVL_1P6: c_uint = 0x0F;
pub const CH_OP_CUR_LVL_2P: c_uint = 0x3F;
// BTEMP High thermal limits
pub const BTEMP_HIGH_TH_57_0: c_uint = 0x00;
pub const BTEMP_HIGH_TH_52: c_uint = 0x01;
pub const BTEMP_HIGH_TH_57_1: c_uint = 0x02;
pub const BTEMP_HIGH_TH_62: c_uint = 0x03;
pub const LOW_BAT_3P1V: c_uint = 0x20;
pub const LOW_BAT_2P3V: c_uint = 0x00;
pub const LOW_BAT_RESET: c_uint = 0x01;
pub const LOW_BAT_ENABLE: c_uint = 0x01;
// Backup battery constants
pub const BUP_ICH_SEL_50UA: c_uint = 0x00;
pub const BUP_ICH_SEL_150UA: c_uint = 0x04;
pub const BUP_ICH_SEL_300UA: c_uint = 0x08;
pub const BUP_ICH_SEL_700UA: c_uint = 0x0C;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bup_vch_sel {
    BUP_VCH_SEL_2P5V,
    BUP_VCH_SEL_2P6V,
    BUP_VCH_SEL_2P8V,
    BUP_VCH_SEL_3P1V,
//
// Note that the following 5 values 2.7v, 2.9v, 3.0v, 3.2v, 3.3v
// are only available on ab8540. You can't choose these 5
// voltage on ab8500/ab8505/ab9540.
//
    BUP_VCH_SEL_2P7V,
    BUP_VCH_SEL_2P9V,
    BUP_VCH_SEL_3P0V,
    BUP_VCH_SEL_3P2V,
    BUP_VCH_SEL_3P3V,
}

pub const BUP_VCH_RANGE: c_uint = 0x02;
pub const VBUP33_VRTCN: c_uint = 0x01;
// Battery OVV constants
pub const BATT_OVV_ENA: c_uint = 0x02;
pub const BATT_OVV_TH_3P7: c_uint = 0x00;
pub const BATT_OVV_TH_4P75: c_uint = 0x01;
// A value to indicate over voltage (microvolts)
pub const BATT_OVV_VALUE: c_int = 4750000;
// VBUS OVV constants
pub const VBUS_OVV_SELECT_MASK: c_uint = 0x78;
pub const VBUS_OVV_SELECT_5P6V: c_uint = 0x00;
pub const VBUS_OVV_SELECT_5P7V: c_uint = 0x08;
pub const VBUS_OVV_SELECT_5P8V: c_uint = 0x10;
pub const VBUS_OVV_SELECT_5P9V: c_uint = 0x18;
pub const VBUS_OVV_SELECT_6P0V: c_uint = 0x20;
pub const VBUS_OVV_SELECT_6P1V: c_uint = 0x28;
pub const VBUS_OVV_SELECT_6P2V: c_uint = 0x30;
pub const VBUS_OVV_SELECT_6P3V: c_uint = 0x38;
pub const VBUS_AUTO_IN_CURR_LIM_ENA: c_uint = 0x04;
// Fuel Gauge constants
pub const RESET_ACCU: c_uint = 0x02;
pub const READ_REQ: c_uint = 0x01;
pub const CC_DEEP_SLEEP_ENA: c_uint = 0x02;
pub const CC_PWR_UP_ENA: c_uint = 0x01;
pub const CC_SAMPLES_40: c_uint = 0x28;
pub const RD_NCONV_ACCU_REQ: c_uint = 0x01;
pub const CC_CALIB: c_uint = 0x08;
pub const CC_INTAVGOFFSET_ENA: c_uint = 0x10;
pub const CC_MUXOFFSET: c_uint = 0x80;
pub const CC_INT_CAL_N_AVG_MASK: c_uint = 0x60;
pub const CC_INT_CAL_SAMPLES_16: c_uint = 0x40;
pub const CC_INT_CAL_SAMPLES_8: c_uint = 0x20;
pub const CC_INT_CAL_SAMPLES_4: c_uint = 0x00;
// RTC constants
pub const RTC_BUP_CH_ENA: c_uint = 0x10;
// BatCtrl Current Source Constants
pub const BAT_CTRL_7U_ENA: c_uint = 0x01;
pub const BAT_CTRL_20U_ENA: c_uint = 0x02;
pub const BAT_CTRL_18U_ENA: c_uint = 0x01;
pub const BAT_CTRL_16U_ENA: c_uint = 0x02;
pub const BAT_CTRL_CMP_ENA: c_uint = 0x04;
pub const FORCE_BAT_CTRL_CMP_HIGH: c_uint = 0x08;
pub const BAT_CTRL_PULL_UP_ENA: c_uint = 0x10;
// Battery type
pub const BATTERY_UNKNOWN: c_int = 00;
// Registers for pcut feature in ab8505 and ab9540
pub const AB8505_RTC_PCUT_CTL_STATUS_REG: c_uint = 0x12;
pub const AB8505_RTC_PCUT_TIME_REG: c_uint = 0x13;
pub const AB8505_RTC_PCUT_MAX_TIME_REG: c_uint = 0x14;
pub const AB8505_RTC_PCUT_FLAG_TIME_REG: c_uint = 0x15;
pub const AB8505_RTC_PCUT_RESTART_REG: c_uint = 0x16;
pub const AB8505_RTC_PCUT_DEBOUNCE_REG: c_uint = 0x17;
// USB Power Path constants for ab8540
pub const BUS_VSYS_VOL_SELECT_MASK: c_uint = 0x06;
pub const BUS_VSYS_VOL_SELECT_3P6V: c_uint = 0x00;
pub const BUS_VSYS_VOL_SELECT_3P325V: c_uint = 0x02;
pub const BUS_VSYS_VOL_SELECT_3P9V: c_uint = 0x04;
pub const BUS_VSYS_VOL_SELECT_4P3V: c_uint = 0x06;
pub const BUS_POWER_PATH_MODE_ENA: c_uint = 0x01;
pub const BUS_PP_PRECHG_CURRENT_MASK: c_uint = 0x0E;
pub const BUS_POWER_PATH_PRECHG_ENA: c_uint = 0x01;
// Forward declaration
//
// struct ab8500_fg_parameters - Fuel gauge algorithm parameters, in seconds
// if not specified
// @recovery_sleep_timer:	Time between measurements while recovering
// @recovery_total_time:	Total recovery time
// @init_timer:			Measurement interval during startup
// @init_discard_time:		Time we discard voltage measurement at startup
// @init_total_time:		Total init time during startup
// @high_curr_time:		Time current has to be high to go to recovery
// @accu_charging:		FG accumulation time while charging
// @accu_high_curr_ua:		FG accumulation time in high current mode
// @high_curr_threshold_ua:	High current threshold, in uA
// @lowbat_threshold_uv:	Low battery threshold, in uV
// @battok_falling_th_sel0	Threshold in mV for battOk signal sel0
// Resolution in 50 mV step.
// @battok_raising_th_sel1	Threshold in mV for battOk signal sel1
// Resolution in 50 mV step.
// @user_cap_limit		Capacity reported from user must be within this
// limit to be considered as sane, in percentage
// points.
// @maint_thres			This is the threshold where we stop reporting
// battery full while in maintenance, in per cent
// @pcut_enable:			Enable power cut feature in ab8505
// @pcut_max_time:		Max time threshold
// @pcut_flag_time:		Flagtime threshold
// @pcut_max_restart:		Max number of restarts
// @pcut_debounce_time:		Sets battery debounce time
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ab8500_fg_parameters {
    pub recovery_sleep_timer: c_int,
    pub recovery_total_time: c_int,
    pub init_timer: c_int,
    pub init_discard_time: c_int,
    pub init_total_time: c_int,
    pub high_curr_time: c_int,
    pub accu_charging: c_int,
    pub accu_high_curr: c_int,
    pub high_curr_threshold_ua: c_int,
    pub lowbat_threshold_uv: c_int,
    pub battok_falling_th_sel0: c_int,
    pub battok_raising_th_sel1: c_int,
    pub user_cap_limit: c_int,
    pub maint_thres: c_int,
    pub pcut_enable: bool,
    pub pcut_max_time: u8,
    pub pcut_flag_time: u8,
    pub pcut_max_restart: u8,
    pub pcut_debounce_time: u8,
}

//
// struct ab8500_charger_maximization - struct used by the board config.
// @use_maxi:		Enable maximization for this battery type
// @maxi_chg_curr_ua:	Maximum charger current allowed in microampere
// @maxi_wait_cycles:	cycles to wait before setting charger current
// @charger_curr_step_ua: delta between two charger current settings (uA)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ab8500_maxim_parameters {
    pub ena_maxi: bool,
    pub chg_curr_ua: c_int,
    pub wait_cycles: c_int,
    pub charger_curr_step_ua: c_int,
}

//
// struct ab8500_bm_capacity_levels - ab8500 capacity level data
// @critical:		critical capacity level in percent
// @low:		low capacity level in percent
// @normal:		normal capacity level in percent
// @high:		high capacity level in percent
// @full:		full capacity level in percent
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ab8500_bm_capacity_levels {
    pub critical: c_int,
    pub low: c_int,
    pub normal: c_int,
    pub high: c_int,
    pub full: c_int,
}

//
// struct ab8500_bm_charger_parameters - Charger specific parameters
// @usb_volt_max_uv:	maximum allowed USB charger voltage in uV
// @usb_curr_max_ua:	maximum allowed USB charger current in uA
// @ac_volt_max_uv:	maximum allowed AC charger voltage in uV
// @ac_curr_max_ua:	maximum allowed AC charger current in uA
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ab8500_bm_charger_parameters {
    pub usb_volt_max_uv: c_int,
    pub usb_curr_max_ua: c_int,
    pub ac_volt_max_uv: c_int,
    pub ac_curr_max_ua: c_int,
}

//
// struct ab8500_bm_data - ab8500 battery management data
// @bi			battery info from device tree
// @temp_now		present battery temperature
// @temp_interval_chg	temperature measurement interval in s when charging
// @temp_interval_nochg	temperature measurement interval in s when not charging
// @main_safety_tmr_h	safety timer for main charger
// @usb_safety_tmr_h	safety timer for usb charger
// @bkup_bat_v		voltage which we charge the backup battery with
// @bkup_bat_i		current which we charge the backup battery with
// @capacity_scaling    indicates whether capacity scaling is to be used
// @chg_unknown_bat	flag to enable charging of unknown batteries
// @enable_overshoot	flag to enable VBAT overshoot control
// @auto_trig		flag to enable auto adc trigger
// @fg_res		resistance of FG resistor in 0.1mOhm
// @interval_charging	charge alg cycle period time when charging (sec)
// @interval_not_charging charge alg cycle period time when not charging (sec)
// @temp_hysteresis	temperature hysteresis
// @maxi		maximization parameters
// @cap_levels		capacity in percent for the different capacity levels
// @chg_params		charger parameters
// @fg_params		fuel gauge parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ab8500_bm_data {
    pub bi: *mut power_supply_battery_info,
    pub temp_now: c_int,
    pub temp_interval_chg: c_int,
    pub temp_interval_nochg: c_int,
    pub main_safety_tmr_h: c_int,
    pub usb_safety_tmr_h: c_int,
    pub bkup_bat_v: c_int,
    pub bkup_bat_i: c_int,
    pub capacity_scaling: bool,
    pub chg_unknown_bat: bool,
    pub enable_overshoot: bool,
    pub auto_trig: bool,
    pub fg_res: c_int,
    pub interval_charging: c_int,
    pub interval_not_charging: c_int,
    pub temp_hysteresis: c_int,
    pub maxi: *const ab8500_maxim_parameters,
    pub cap_levels: *const ab8500_bm_capacity_levels,
    pub chg_params: *const ab8500_bm_charger_parameters,
    pub fg_params: *const ab8500_fg_parameters,
}

// Forward declaration
extern "C" {
    pub fn ab8500_charger_usb_state_changed(bm_usb_state: u8, mA: u16);
}
extern "C" {
    pub fn ab8500_fg_inst_curr_blocking(dev: *mut ab8500_fg) -> c_int;
}
extern "C" {
    pub fn ab8500_fg_inst_curr_start(di: *mut ab8500_fg) -> c_int;
}
extern "C" {
    pub fn ab8500_fg_inst_curr_finalize(di: *mut ab8500_fg, res: *mut c_int) -> c_int;
}
extern "C" {
    pub fn ab8500_fg_inst_curr_started(di: *mut ab8500_fg) -> c_int;
}
extern "C" {
    pub fn ab8500_fg_inst_curr_done(di: *mut ab8500_fg) -> c_int;
}
