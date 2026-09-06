//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/gma500/intel_bios.h
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
// Copyright (c) 2006 Intel Corporation
//
// Authors:
// Eric Anholt <eric@anholt.net>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbt_header {
    pub /: *mut *mut *mut u8 signature[20]; /< Always starts with 'VBT$',
    pub /: *mut *mut *mut u16 version; /< decimal,
    pub /: *mut *mut *mut u16 header_size; /< in bytes,
    pub /: *mut *mut *mut u16 vbt_size; /< in bytes,
    pub vbt_checksum: u8,
    pub reserved0: u8,
    pub /: *mut *mut *mut u32 bdb_offset; /< from beginning of VBT,
    pub /: *mut *mut *mut u32 aim_offset[4]; /< from beginning of VBT,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_header {
    pub /: *mut *mut *mut u8 signature[16]; /< Always 'BIOS_DATA_BLOCK',
    pub /: *mut *mut *mut u16 version; /< decimal,
    pub /: *mut *mut *mut u16 header_size; /< in bytes,
    pub /: *mut *mut *mut u16 bdb_size; /< in bytes,
}

// strictly speaking, this is a "skip" block, but it has interesting info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbios_data {
    pub /: *mut *mut u8 type; / 0 == desktop, 1 == mobile,
    pub relstage: u8,
    pub chipset: u8,
    pub lvds_present:1: u8,
    pub tv_present:1: u8,
    pub /: *mut *mut u8 rsvd2:6; / finish byte,
    pub rsvd3: [u8; 4],
    pub signon: [u8; 155],
    pub copyright: [u8; 61],
    pub code_segment: u16,
    pub dos_boot_mode: u8,
    pub bandwidth_percent: u8,
    pub /: *mut *mut u8 rsvd4; / popup memory size,
    pub resize_pci_bios: u8,
    pub /: *mut *mut u8 rsvd5; / is crt already on ddc2,
    pub __packed: },
//
// There are several types of BIOS data blocks (BDBs), each block has
// an ID and size in the first 3 bytes (ID in first, size in next 2).
// Known types are listed below.
//
pub const BDB_GENERAL_FEATURES: c_int = 1;
pub const BDB_GENERAL_DEFINITIONS: c_int = 2;
pub const BDB_OLD_TOGGLE_LIST: c_int = 3;
pub const BDB_MODE_SUPPORT_LIST: c_int = 4;
pub const BDB_GENERIC_MODE_TABLE: c_int = 5;
pub const BDB_EXT_MMIO_REGS: c_int = 6;
pub const BDB_SWF_IO: c_int = 7;
pub const BDB_SWF_MMIO: c_int = 8;
pub const BDB_DOT_CLOCK_TABLE: c_int = 9;
pub const BDB_MODE_REMOVAL_TABLE: c_int = 10;
pub const BDB_CHILD_DEVICE_TABLE: c_int = 11;
pub const BDB_DRIVER_FEATURES: c_int = 12;
pub const BDB_DRIVER_PERSISTENCE: c_int = 13;
pub const BDB_EXT_TABLE_PTRS: c_int = 14;
pub const BDB_DOT_CLOCK_OVERRIDE: c_int = 15;
pub const BDB_DISPLAY_SELECT: c_int = 16;
// 17 rsvd
pub const BDB_DRIVER_ROTATION: c_int = 18;
pub const BDB_DISPLAY_REMOVE: c_int = 19;
pub const BDB_OEM_CUSTOM: c_int = 20;

pub const BDB_SDVO_LVDS_OPTIONS: c_int = 22;
pub const BDB_SDVO_PANEL_DTDS: c_int = 23;
pub const BDB_SDVO_LVDS_PNP_IDS: c_int = 24;
pub const BDB_SDVO_LVDS_POWER_SEQ: c_int = 25;
pub const BDB_TV_OPTIONS: c_int = 26;
pub const BDB_EDP: c_int = 27;
pub const BDB_LVDS_OPTIONS: c_int = 40;
pub const BDB_LVDS_LFP_DATA_PTRS: c_int = 41;
pub const BDB_LVDS_LFP_DATA: c_int = 42;
pub const BDB_LVDS_BACKLIGHT: c_int = 43;
pub const BDB_LVDS_POWER: c_int = 44;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_general_features {
// bits 1
    pub panel_fitting:2: u8,
    pub flexaim:1: u8,
    pub msg_enable:1: u8,
    pub clear_screen:3: u8,
    pub color_flip:1: u8,
// bits 2
    pub download_ext_vbt:1: u8,
    pub enable_ssc:1: u8,
    pub ssc_freq:1: u8,
    pub enable_lfp_on_override:1: u8,
    pub disable_ssc_ddt:1: u8,
    pub /: *mut *mut u8 rsvd8:3; / finish byte,
// bits 3
    pub disable_smooth_vision:1: u8,
    pub single_dvi:1: u8,
    pub /: *mut *mut u8 rsvd9:6; / finish byte,
// bits 4
    pub legacy_monitor_detect: u8,
// bits 5
    pub int_crt_support:1: u8,
    pub int_tv_support:1: u8,
    pub int_efp_support:1: u8,
    pub /: *mut *mut u8 dp_ssc_enb:1; / PCH attached eDP supports SSC,
    pub /: *mut *mut u8 dp_ssc_freq:1; / SSC freq for PCH attached eDP,
    pub /: *mut *mut u8 rsvd11:3; / finish byte,
    pub __packed: },
// pre-915
pub const GPIO_PIN_DVI_LVDS: c_uint = 0x03 /* "DVI/LVDS DDC GPIO pins" */;
pub const GPIO_PIN_ADD_I2C: c_uint = 0x05 /* "ADDCARD I2C GPIO pins" */;
pub const GPIO_PIN_ADD_DDC: c_uint = 0x04 /* "ADDCARD DDC GPIO pins" */;
pub const GPIO_PIN_ADD_DDC_I2C: c_uint = 0x06 /* "ADDCARD DDC/I2C GPIO pins" */;
// Pre 915
pub const DEVICE_TYPE_NONE: c_uint = 0x00;
pub const DEVICE_TYPE_CRT: c_uint = 0x01;
pub const DEVICE_TYPE_TV: c_uint = 0x09;
pub const DEVICE_TYPE_EFP: c_uint = 0x12;
pub const DEVICE_TYPE_LFP: c_uint = 0x22;
// On 915+
pub const DEVICE_TYPE_CRT_DPMS: c_uint = 0x6001;
pub const DEVICE_TYPE_CRT_DPMS_HOTPLUG: c_uint = 0x4001;
pub const DEVICE_TYPE_TV_COMPOSITE: c_uint = 0x0209;
pub const DEVICE_TYPE_TV_MACROVISION: c_uint = 0x0289;
pub const DEVICE_TYPE_TV_RF_COMPOSITE: c_uint = 0x020c;
pub const DEVICE_TYPE_TV_SVIDEO_COMPOSITE: c_uint = 0x0609;
pub const DEVICE_TYPE_TV_SCART: c_uint = 0x0209;
pub const DEVICE_TYPE_TV_CODEC_HOTPLUG_PWR: c_uint = 0x6009;
pub const DEVICE_TYPE_EFP_HOTPLUG_PWR: c_uint = 0x6012;
pub const DEVICE_TYPE_EFP_DVI_HOTPLUG_PWR: c_uint = 0x6052;
pub const DEVICE_TYPE_EFP_DVI_I: c_uint = 0x6053;
pub const DEVICE_TYPE_EFP_DVI_D_DUAL: c_uint = 0x6152;
pub const DEVICE_TYPE_EFP_DVI_D_HDCP: c_uint = 0x60d2;
pub const DEVICE_TYPE_OPENLDI_HOTPLUG_PWR: c_uint = 0x6062;
pub const DEVICE_TYPE_OPENLDI_DUALPIX: c_uint = 0x6162;
pub const DEVICE_TYPE_LFP_PANELLINK: c_uint = 0x5012;
pub const DEVICE_TYPE_LFP_CMOS_PWR: c_uint = 0x5042;
pub const DEVICE_TYPE_LFP_LVDS_PWR: c_uint = 0x5062;
pub const DEVICE_TYPE_LFP_LVDS_DUAL: c_uint = 0x5162;
pub const DEVICE_TYPE_LFP_LVDS_DUAL_HDCP: c_uint = 0x51e2;
pub const DEVICE_CFG_NONE: c_uint = 0x00;
pub const DEVICE_CFG_12BIT_DVOB: c_uint = 0x01;
pub const DEVICE_CFG_12BIT_DVOC: c_uint = 0x02;
pub const DEVICE_CFG_24BIT_DVOBC: c_uint = 0x09;
pub const DEVICE_CFG_24BIT_DVOCB: c_uint = 0x0a;
pub const DEVICE_CFG_DUAL_DVOB: c_uint = 0x11;
pub const DEVICE_CFG_DUAL_DVOC: c_uint = 0x12;
pub const DEVICE_CFG_DUAL_DVOBC: c_uint = 0x13;
pub const DEVICE_CFG_DUAL_LINK_DVOBC: c_uint = 0x19;
pub const DEVICE_CFG_DUAL_LINK_DVOCB: c_uint = 0x1a;
pub const DEVICE_WIRE_NONE: c_uint = 0x00;
pub const DEVICE_WIRE_DVOB: c_uint = 0x01;
pub const DEVICE_WIRE_DVOC: c_uint = 0x02;
pub const DEVICE_WIRE_DVOBC: c_uint = 0x03;
pub const DEVICE_WIRE_DVOBB: c_uint = 0x05;
pub const DEVICE_WIRE_DVOCC: c_uint = 0x06;
pub const DEVICE_WIRE_DVOB_MASTER: c_uint = 0x0d;
pub const DEVICE_WIRE_DVOC_MASTER: c_uint = 0x0e;
pub const DEVICE_PORT_DVOA: c_uint = 0x00 /* none on 845+ */;
pub const DEVICE_PORT_DVOB: c_uint = 0x01;
pub const DEVICE_PORT_DVOC: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct child_device_config {
    pub handle: u16,
    pub device_type: u16,
    pub /: *mut *mut u8 device_id[10]; / ascii string,
    pub addin_offset: u16,
    pub /: *mut *mut *mut u8 dvo_port; / See Device_PORT_ above,
    pub i2c_pin: u8,
    pub target_addr: u8,
    pub ddc_pin: u8,
    pub edid_ptr: u16,
    pub /: *mut *mut *mut u8 dvo_cfg; / See DEVICE_CFG_ above,
    pub dvo2_port: u8,
    pub i2c2_pin: u8,
    pub target2_addr: u8,
    pub ddc2_pin: u8,
    pub capabilities: u8,
    pub /: *mut *mut *mut u8 dvo_wiring;/ See DEVICE_WIRE_ above,
    pub dvo2_wiring: u8,
    pub extended_type: u16,
    pub dvo_function: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_general_definitions {
// DDC GPIO
    pub crt_ddc_gmbus_pin: u8,
// DPMS bits
    pub dpms_acpi:1: u8,
    pub skip_boot_crt_detect:1: u8,
    pub dpms_aim:1: u8,
    pub /: *mut *mut u8 rsvd1:5; / finish byte,
// boot device bits
    pub boot_display: [u8; 2],
    pub child_dev_size: u8,
//
// Device info:
// If TV is present, it'll be at devices[0].
// LVDS will be next, either devices[0] or [1], if present.
// On some platforms the number of device is 6. But could be as few as
// 4 if both TV and LVDS are missing.
// And the device num is related with the size of general definition
// block. It is obtained by using the following formula:
// number = (block_size - sizeof(bdb_general_definitions))
// sizeof(child_device_config);
//
    pub devices: [child_device_config; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_lvds_options {
    pub panel_type: u8,
    pub rsvd1: u8,
// LVDS capabilities, stored in a dword
    pub pfit_mode:2: u8,
    pub pfit_text_mode_enhanced:1: u8,
    pub pfit_gfx_mode_enhanced:1: u8,
    pub pfit_ratio_auto:1: u8,
    pub pixel_dither:1: u8,
    pub lvds_edid:1: u8,
    pub rsvd2:1: u8,
    pub rsvd4: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_lvds_backlight {
    pub type:2: u8,
    pub pol:1: u8,
    pub gpio:3: u8,
    pub gmbus:2: u8,
    pub freq: u16,
    pub minbrightness: u8,
    pub i2caddr: u8,
    pub brightnesscmd: u8,
// FIXME: more...
    pub __packed: },
// LFP pointer table contains entries to the struct below
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_lvds_lfp_data_ptr {
    pub /: *mut *mut u16 fp_timing_offset; / offsets are from start of bdb,
    pub fp_table_size: u8,
    pub dvo_timing_offset: u16,
    pub dvo_table_size: u8,
    pub panel_pnp_id_offset: u16,
    pub pnp_table_size: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_lvds_lfp_data_ptrs {
    pub /: *mut *mut u8 lvds_entries; / followed by one or more lvds_data_ptr structs,
    pub ptr: [bdb_lvds_lfp_data_ptr; 16],
    pub __packed: },
// LFP data has 3 blocks per entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lvds_fp_timing {
    pub x_res: u16,
    pub y_res: u16,
    pub lvds_reg: u32,
    pub lvds_reg_val: u32,
    pub pp_on_reg: u32,
    pub pp_on_reg_val: u32,
    pub pp_off_reg: u32,
    pub pp_off_reg_val: u32,
    pub pp_cycle_reg: u32,
    pub pp_cycle_reg_val: u32,
    pub pfit_reg: u32,
    pub pfit_reg_val: u32,
    pub terminator: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lvds_dvo_timing {
    pub /: *mut *mut *mut u16 clock; /< In 10khz,
    pub hactive_lo: u8,
    pub hblank_lo: u8,
    pub hblank_hi:4: u8,
    pub hactive_hi:4: u8,
    pub vactive_lo: u8,
    pub vblank_lo: u8,
    pub vblank_hi:4: u8,
    pub vactive_hi:4: u8,
    pub hsync_off_lo: u8,
    pub hsync_pulse_width: u8,
    pub vsync_pulse_width:4: u8,
    pub vsync_off:4: u8,
    pub rsvd0:6: u8,
    pub hsync_off_hi:2: u8,
    pub h_image: u8,
    pub v_image: u8,
    pub max_hv: u8,
    pub h_border: u8,
    pub v_border: u8,
    pub rsvd1:3: u8,
    pub digital:2: u8,
    pub vsync_positive:1: u8,
    pub hsync_positive:1: u8,
    pub rsvd2:1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lvds_pnp_id {
    pub mfg_name: u16,
    pub product_code: u16,
    pub serial: u32,
    pub mfg_week: u8,
    pub mfg_year: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_lvds_lfp_data_entry {
    pub fp_timing: lvds_fp_timing,
    pub dvo_timing: lvds_dvo_timing,
    pub pnp_id: lvds_pnp_id,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_lvds_lfp_data {
    pub data: [bdb_lvds_lfp_data_entry; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aimdb_header {
    pub signature: [c_char; 16],
    pub oem_device: [c_char; 20],
    pub aimdb_version: u16,
    pub aimdb_header_size: u16,
    pub aimdb_size: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aimdb_block {
    pub aimdb_id: u8,
    pub aimdb_size: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vch_panel_data {
    pub fp_timing_offset: u16,
    pub fp_timing_size: u8,
    pub dvo_timing_offset: u16,
    pub dvo_timing_size: u8,
    pub text_fitting_offset: u16,
    pub text_fitting_size: u8,
    pub graphics_fitting_offset: u16,
    pub graphics_fitting_size: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vch_bdb_22 {
    pub aimdb_block: aimdb_block,
    pub panels: [vch_panel_data; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_sdvo_lvds_options {
    pub panel_backlight: u8,
    pub h40_set_panel_type: u8,
    pub panel_type: u8,
    pub ssc_clk_freq: u8,
    pub als_low_trip: u16,
    pub als_high_trip: u16,
    pub sclalarcoeff_tab_row_num: u8,
    pub sclalarcoeff_tab_row_size: u8,
    pub coefficient: [u8; 8],
    pub panel_misc_bits_1: u8,
    pub panel_misc_bits_2: u8,
    pub panel_misc_bits_3: u8,
    pub panel_misc_bits_4: u8,
    pub __packed: },
pub const BDB_DRIVER_FEATURE_NO_LVDS: c_int = 0;
pub const BDB_DRIVER_FEATURE_INT_LVDS: c_int = 1;
pub const BDB_DRIVER_FEATURE_SDVO_LVDS: c_int = 2;
pub const BDB_DRIVER_FEATURE_EDP: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_driver_features {
    pub boot_dev_algorithm:1: u8,
    pub block_display_switch:1: u8,
    pub allow_display_switch:1: u8,
    pub hotplug_dvo:1: u8,
    pub dual_view_zoom:1: u8,
    pub int15h_hook:1: u8,
    pub sprite_in_clone:1: u8,
    pub primary_lfp_id:1: u8,
    pub boot_mode_x: u16,
    pub boot_mode_y: u16,
    pub boot_mode_bpp: u8,
    pub boot_mode_refresh: u8,
    pub enable_lfp_primary:1: u16,
    pub selective_mode_pruning:1: u16,
    pub dual_frequency:1: u16,
    pub /: *mut *mut u16 render_clock_freq:1; / 0: high freq; 1: low freq,
    pub nt_clone_support:1: u16,
    pub /: *mut *mut u16 power_scheme_ui:1; / 0: CUI; 1: 3rd party,
    pub /: *mut *mut u16 sprite_display_assign:1; / 0: secondary; 1: primary,
    pub cui_aspect_scaling:1: u16,
    pub preserve_aspect_ratio:1: u16,
    pub sdvo_device_power_down:1: u16,
    pub crt_hotplug:1: u16,
    pub lvds_config:2: u16,
    pub tv_hotplug:1: u16,
    pub hdmi_config:2: u16,
    pub static_display:1: u8,
    pub reserved2:7: u8,
    pub legacy_crt_max_x: u16,
    pub legacy_crt_max_y: u16,
    pub legacy_crt_max_refresh: u8,
    pub hdmi_termination: u8,
    pub custom_vbt_version: u8,
    pub __packed: },
pub const EDP_18BPP: c_int = 0;
pub const EDP_24BPP: c_int = 1;
pub const EDP_30BPP: c_int = 2;
pub const EDP_RATE_1_62: c_int = 0;
pub const EDP_RATE_2_7: c_int = 1;
pub const EDP_LANE_1: c_int = 0;
pub const EDP_LANE_2: c_int = 1;
pub const EDP_LANE_4: c_int = 3;
pub const EDP_PREEMPHASIS_NONE: c_int = 0;
pub const EDP_PREEMPHASIS_3_5dB: c_int = 1;
pub const EDP_PREEMPHASIS_6dB: c_int = 2;
pub const EDP_PREEMPHASIS_9_5dB: c_int = 3;
pub const EDP_VSWING_0_4V: c_int = 0;
pub const EDP_VSWING_0_6V: c_int = 1;
pub const EDP_VSWING_0_8V: c_int = 2;
pub const EDP_VSWING_1_2V: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edp_power_seq {
    pub t1_t3: u16,
    pub t8: u16,
    pub t9: u16,
    pub t10: u16,
    pub t11_t12: u16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edp_link_params {
    pub rate:4: u8,
    pub lanes:4: u8,
    pub preemphasis:4: u8,
    pub vswing:4: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_edp {
    pub power_seqs: [edp_power_seq; 16],
    pub color_depth: u32,
    pub sdrrs_msa_timing_delay: u32,
    pub link_params: [edp_link_params; 16],
// C attribute field omitted
    pub dev): *mut extern int psb_intel_init_bios(struct drm_device,
    pub dev): *mut extern void psb_intel_destroy_bios(struct drm_device,
//
// Driver<->VBIOS interaction occurs through scratch bits in
// GR18 & SWF*.
//
// GR18 bits are set on display switch and hotkey events

pub const GR18_HOTKEY_MASK: c_uint = 0x78 /* See also SWF4 15:0 */;

// Set by driver, cleared by VBIOS
pub const SWF00_YRES_SHIFT: c_int = 16;
pub const SWF00_XRES_SHIFT: c_int = 0;
pub const SWF00_RES_MASK: c_uint = 0xffff;
// Set by VBIOS at boot time and driver at runtime
pub const SWF01_TV2_FORMAT_SHIFT: c_int = 8;
pub const SWF01_TV1_FORMAT_SHIFT: c_int = 0;
pub const SWF01_TV_FORMAT_MASK: c_uint = 0xffff;

pub const SWF10_OLD_TOGGLE: c_uint = 0x0;
pub const SWF10_TOGGLE_LIST_1: c_uint = 0x1;
pub const SWF10_TOGGLE_LIST_2: c_uint = 0x2;
pub const SWF10_TOGGLE_LIST_3: c_uint = 0x3;
pub const SWF10_TOGGLE_LIST_4: c_uint = 0x4;

pub const SWF10_ACTIVE_DISP_MASK: c_uint = 0xffff;

pub const SWF11_MEMORY_SIZE_SHIFT: c_int = 16;

pub const SWF11_DPMS_MASK: c_uint = 0x07;

pub const SWF11_DPMS_ON: c_int = 0;

// 21:19 rsvd
pub const SWF14_PM_TYPE_MASK: c_uint = 0x00070000;

pub const SWF14_HK_REQUEST_MASK: c_uint = 0x0000ffff /* see GR18 6:3 for event type */;
// if GR18 indicates a display switch

// if GR18 indicates a panel fitting request

// if GR18 indicates an APM change request
pub const SWF14_APM_HIBERNATE: c_uint = 0x4;
pub const SWF14_APM_SUSPEND: c_uint = 0x3;
pub const SWF14_APM_STANDBY: c_uint = 0x1;
pub const SWF14_APM_RESTORE: c_uint = 0x0;
// Add the device class for LFP, TV, HDMI
pub const DEVICE_TYPE_INT_LFP: c_uint = 0x1022;
pub const DEVICE_TYPE_INT_TV: c_uint = 0x1009;
pub const DEVICE_TYPE_HDMI: c_uint = 0x60D2;
pub const DEVICE_TYPE_DP: c_uint = 0x68C6;
pub const DEVICE_TYPE_eDP: c_uint = 0x78C6;
// define the DVO port for HDMI output type
pub const DVO_B: c_int = 1;
pub const DVO_C: c_int = 2;
pub const DVO_D: c_int = 3;
// define the PORT for DP output type
pub const PORT_IDPB: c_int = 7;
pub const PORT_IDPC: c_int = 8;
pub const PORT_IDPD: c_int = 9;
