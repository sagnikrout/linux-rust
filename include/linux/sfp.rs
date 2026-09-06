//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sfp.h
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfp_eeprom_base {
    pub phys_id: u8,
    pub phys_ext_id: u8,
    pub connector: u8,

    pub e10g_base_er:1: u8,
    pub e10g_base_lrm:1: u8,
    pub e10g_base_lr:1: u8,
    pub e10g_base_sr:1: u8,
    pub if_1x_sx:1: u8,
    pub if_1x_lx:1: u8,
    pub if_1x_copper_active:1: u8,
    pub if_1x_copper_passive:1: u8,
    pub escon_mmf_1310_led:1: u8,
    pub escon_smf_1310_laser:1: u8,
    pub sonet_oc192_short_reach:1: u8,
    pub sonet_reach_bit1:1: u8,
    pub sonet_reach_bit2:1: u8,
    pub sonet_oc48_long_reach:1: u8,
    pub sonet_oc48_intermediate_reach:1: u8,
    pub sonet_oc48_short_reach:1: u8,
    pub unallocated_5_7:1: u8,
    pub sonet_oc12_smf_long_reach:1: u8,
    pub sonet_oc12_smf_intermediate_reach:1: u8,
    pub sonet_oc12_short_reach:1: u8,
    pub unallocated_5_3:1: u8,
    pub sonet_oc3_smf_long_reach:1: u8,
    pub sonet_oc3_smf_intermediate_reach:1: u8,
    pub sonet_oc3_short_reach:1: u8,
    pub e_base_px:1: u8,
    pub e_base_bx10:1: u8,
    pub e100_base_fx:1: u8,
    pub e100_base_lx:1: u8,
    pub e1000_base_t:1: u8,
    pub e1000_base_cx:1: u8,
    pub e1000_base_lx:1: u8,
    pub e1000_base_sx:1: u8,
    pub fc_ll_v:1: u8,
    pub fc_ll_s:1: u8,
    pub fc_ll_i:1: u8,
    pub fc_ll_l:1: u8,
    pub fc_ll_m:1: u8,
    pub fc_tech_sa:1: u8,
    pub fc_tech_lc:1: u8,
    pub fc_tech_electrical_inter_enclosure:1: u8,
    pub fc_tech_electrical_intra_enclosure:1: u8,
    pub fc_tech_sn:1: u8,
    pub fc_tech_sl:1: u8,
    pub fc_tech_ll:1: u8,
    pub sfp_ct_active:1: u8,
    pub sfp_ct_passive:1: u8,
    pub unallocated_8_1:1: u8,
    pub unallocated_8_0:1: u8,
    pub fc_media_tw:1: u8,
    pub fc_media_tp:1: u8,
    pub fc_media_mi:1: u8,
    pub fc_media_tv:1: u8,
    pub fc_media_m6:1: u8,
    pub fc_media_m5:1: u8,
    pub unallocated_9_1:1: u8,
    pub fc_media_sm:1: u8,
    pub fc_speed_1200:1: u8,
    pub fc_speed_800:1: u8,
    pub fc_speed_1600:1: u8,
    pub fc_speed_400:1: u8,
    pub fc_speed_3200:1: u8,
    pub fc_speed_200:1: u8,
    pub unallocated_10_1:1: u8,
    pub fc_speed_100:1: u8,

    pub if_1x_copper_passive:1: u8,
    pub if_1x_copper_active:1: u8,
    pub if_1x_lx:1: u8,
    pub if_1x_sx:1: u8,
    pub e10g_base_sr:1: u8,
    pub e10g_base_lr:1: u8,
    pub e10g_base_lrm:1: u8,
    pub e10g_base_er:1: u8,
    pub sonet_oc3_short_reach:1: u8,
    pub sonet_oc3_smf_intermediate_reach:1: u8,
    pub sonet_oc3_smf_long_reach:1: u8,
    pub unallocated_5_3:1: u8,
    pub sonet_oc12_short_reach:1: u8,
    pub sonet_oc12_smf_intermediate_reach:1: u8,
    pub sonet_oc12_smf_long_reach:1: u8,
    pub unallocated_5_7:1: u8,
    pub sonet_oc48_short_reach:1: u8,
    pub sonet_oc48_intermediate_reach:1: u8,
    pub sonet_oc48_long_reach:1: u8,
    pub sonet_reach_bit2:1: u8,
    pub sonet_reach_bit1:1: u8,
    pub sonet_oc192_short_reach:1: u8,
    pub escon_smf_1310_laser:1: u8,
    pub escon_mmf_1310_led:1: u8,
    pub e1000_base_sx:1: u8,
    pub e1000_base_lx:1: u8,
    pub e1000_base_cx:1: u8,
    pub e1000_base_t:1: u8,
    pub e100_base_lx:1: u8,
    pub e100_base_fx:1: u8,
    pub e_base_bx10:1: u8,
    pub e_base_px:1: u8,
    pub fc_tech_electrical_inter_enclosure:1: u8,
    pub fc_tech_lc:1: u8,
    pub fc_tech_sa:1: u8,
    pub fc_ll_m:1: u8,
    pub fc_ll_l:1: u8,
    pub fc_ll_i:1: u8,
    pub fc_ll_s:1: u8,
    pub fc_ll_v:1: u8,
    pub unallocated_8_0:1: u8,
    pub unallocated_8_1:1: u8,
    pub sfp_ct_passive:1: u8,
    pub sfp_ct_active:1: u8,
    pub fc_tech_ll:1: u8,
    pub fc_tech_sl:1: u8,
    pub fc_tech_sn:1: u8,
    pub fc_tech_electrical_intra_enclosure:1: u8,
    pub fc_media_sm:1: u8,
    pub unallocated_9_1:1: u8,
    pub fc_media_m5:1: u8,
    pub fc_media_m6:1: u8,
    pub fc_media_tv:1: u8,
    pub fc_media_mi:1: u8,
    pub fc_media_tp:1: u8,
    pub fc_media_tw:1: u8,
    pub fc_speed_100:1: u8,
    pub unallocated_10_1:1: u8,
    pub fc_speed_200:1: u8,
    pub fc_speed_3200:1: u8,
    pub fc_speed_400:1: u8,
    pub fc_speed_1600:1: u8,
    pub fc_speed_800:1: u8,
    pub fc_speed_1200:1: u8,

    pub encoding: u8,
    pub br_nominal: u8,
    pub rate_id: u8,
    pub link_len: [u8; 6],
    pub vendor_name: [c_char; 16],
    pub extended_cc: u8,
    pub vendor_oui: [c_char; 3],
    pub vendor_pn: [c_char; 16],
    pub vendor_rev: [c_char; 4],
    pub optical_wavelength: __be16,
    pub cable_compliance: __be16,

    pub reserved60_2:6: u8,
    pub fc_pi_4_app_h:1: u8,
    pub sff8431_app_e:1: u8,
    pub reserved61:8: u8,

    pub sff8431_app_e:1: u8,
    pub fc_pi_4_app_h:1: u8,
    pub reserved60_2:6: u8,
    pub reserved61:8: u8,

    pub passive: } __packed,

    pub reserved60_4:4: u8,
    pub fc_pi_4_lim:1: u8,
    pub sff8431_lim:1: u8,
    pub fc_pi_4_app_h:1: u8,
    pub sff8431_app_e:1: u8,
    pub reserved61:8: u8,

    pub sff8431_app_e:1: u8,
    pub fc_pi_4_app_h:1: u8,
    pub sff8431_lim:1: u8,
    pub fc_pi_4_lim:1: u8,
    pub reserved60_4:4: u8,
    pub reserved61:8: u8,

    pub active: } __packed,
    pub __packed: },
    pub reserved62: u8,
    pub cc_base: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfp_eeprom_ext {
    pub options: __be16,
    pub br_max: u8,
    pub br_min: u8,
    pub vendor_sn: [c_char; 16],
    pub datecode: [c_char; 8],
    pub diagmon: u8,
    pub enhopts: u8,
    pub sff8472_compliance: u8,
    pub cc_ext: u8,
    pub __packed: },
//
// struct sfp_eeprom_id - raw SFP module identification information
// @base: base SFP module identification structure
// @ext: extended SFP module identification structure
//
// See the SFF-8472 specification and related documents for the definition
// of these structure members. This can be obtained from
// https://www.snia.org/technology-communities/sff/specifications
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfp_eeprom_id {
    pub base: sfp_eeprom_base,
    pub ext: sfp_eeprom_ext,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfp_diag {
    pub temp_high_alarm: __be16,
    pub temp_low_alarm: __be16,
    pub temp_high_warn: __be16,
    pub temp_low_warn: __be16,
    pub volt_high_alarm: __be16,
    pub volt_low_alarm: __be16,
    pub volt_high_warn: __be16,
    pub volt_low_warn: __be16,
    pub bias_high_alarm: __be16,
    pub bias_low_alarm: __be16,
    pub bias_high_warn: __be16,
    pub bias_low_warn: __be16,
    pub txpwr_high_alarm: __be16,
    pub txpwr_low_alarm: __be16,
    pub txpwr_high_warn: __be16,
    pub txpwr_low_warn: __be16,
    pub rxpwr_high_alarm: __be16,
    pub rxpwr_low_alarm: __be16,
    pub rxpwr_high_warn: __be16,
    pub rxpwr_low_warn: __be16,
    pub laser_temp_high_alarm: __be16,
    pub laser_temp_low_alarm: __be16,
    pub laser_temp_high_warn: __be16,
    pub laser_temp_low_warn: __be16,
    pub tec_cur_high_alarm: __be16,
    pub tec_cur_low_alarm: __be16,
    pub tec_cur_high_warn: __be16,
    pub tec_cur_low_warn: __be16,
    pub cal_rxpwr4: __be32,
    pub cal_rxpwr3: __be32,
    pub cal_rxpwr2: __be32,
    pub cal_rxpwr1: __be32,
    pub cal_rxpwr0: __be32,
    pub cal_txi_slope: __be16,
    pub cal_txi_offset: __be16,
    pub cal_txpwr_slope: __be16,
    pub cal_txpwr_offset: __be16,
    pub cal_t_slope: __be16,
    pub cal_t_offset: __be16,
    pub cal_v_slope: __be16,
    pub cal_v_offset: __be16,
    pub __packed: },
// SFF8024 defined constants
// codes 01-05 not supportable on SFP, but some modules have single SC
}

// SFP EEPROM registers
// SFP Diagnostics
// Alarm and warnings stored MSB at lower address then LSB
//
// struct sfp_module_caps - sfp module capabilities
// @interfaces: bitmap of interfaces that the module may support
// @link_modes: bitmap of ethtool link modes that the module may support
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfp_module_caps {
//
// @may_have_phy: indicate whether the module may have an ethernet PHY
// There is no way to be sure that a module has a PHY as the EEPROM
// doesn't contain this information. When set, this does not mean that
// the module definitely has a PHY.
//
    pub may_have_phy: bool,
//
// @port: one of ethtool %PORT_* definitions, parsed from the module
// EEPROM, or %PORT_OTHER if the port type is not known.
//
    pub port: u8,
}

//
// struct sfp_upstream_ops - upstream operations structure
// @attach: called when the sfp socket driver is bound to the upstream
// (mandatory).
// @detach: called when the sfp socket driver is unbound from the upstream
// (mandatory).
// @module_insert: called after a module has been detected to determine
// whether the module is supported for the upstream device.
// @module_remove: called after the module has been removed.
// @module_start: called after the PHY probe step
// @module_stop: called before the PHY is removed
// @link_down: called when the link is non-operational for whatever
// reason.
// @link_up: called when the link is operational.
// @connect_phy: called when an I2C accessible PHY has been detected
// on the module.
// @disconnect_phy: called when a module with an I2C accessible PHY has
// been removed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfp_upstream_ops {
    pub bus): *mut *mut *mut void (attach)(void priv, struct sfp_bus,
    pub bus): *mut *mut *mut void (detach)(void priv, struct sfp_bus,
    pub id): *const *const *const int (module_insert)(void priv, struct sfp_eeprom_id,
    pub priv): *mut *mut void (module_remove)(void,
    pub priv): *mut *mut int (module_start)(void,
    pub priv): *mut *mut void (module_stop)(void,
    pub priv): *mut *mut void (link_down)(void,
    pub priv): *mut *mut void (link_up)(void,
    pub ): *mut *mut *mut int (connect_phy)(void priv, struct phy_device,
    pub ): *mut *mut *mut void (disconnect_phy)(void priv, struct phy_device,
}

extern "C" {
    pub fn sfp_get_module_info(bus: *mut sfp_bus, modinfo: *mut ethtool_modinfo) -> c_int;
}
extern "C" {
    pub fn sfp_upstream_start(bus: *mut sfp_bus);
}
extern "C" {
    pub fn sfp_upstream_stop(bus: *mut sfp_bus);
}
extern "C" {
    pub fn sfp_upstream_set_signal_rate(bus: *mut sfp_bus, rate_kbd: c_uint);
}
extern "C" {
    pub fn sfp_bus_put(bus: *mut sfp_bus);
}
extern "C" {
    pub fn sfp_bus_del_upstream(bus: *mut sfp_bus);
}

