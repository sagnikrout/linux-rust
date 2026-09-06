//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_vbt_defs.h
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


//
// Copyright © 2006-2016 Intel Corporation
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Authors:
// Eric Anholt <eric@anholt.net>
//
// This information is private to VBT parsing in intel_bios.c.
//
// Please do NOT include anywhere else.
//

// EDID derived structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_edid_pnp_id {
    pub mfg_name: u16,
    pub product_code: u16,
    pub serial: u32,
    pub mfg_week: u8,
    pub mfg_year: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_edid_product_name {
    pub name: [c_char; 13],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_edid_dtd {
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
    pub hsync_pulse_width_lo: u8,
    pub vsync_pulse_width_lo:4: u8,
    pub vsync_off_lo:4: u8,
    pub vsync_pulse_width_hi:2: u8,
    pub vsync_off_hi:2: u8,
    pub hsync_pulse_width_hi:2: u8,
    pub hsync_off_hi:2: u8,
    pub himage_lo: u8,
    pub vimage_lo: u8,
    pub vimage_hi:4: u8,
    pub himage_hi:4: u8,
    pub h_border: u8,
    pub v_border: u8,
    pub rsvd1:3: u8,
    pub digital:2: u8,
    pub vsync_positive:1: u8,
    pub hsync_positive:1: u8,
    pub non_interlaced:1: u8,
    pub __packed: },
//
// struct vbt_header - VBT Header structure
// @signature:		VBT signature, always starts with "$VBT"
// @version:		Version of this structure
// @header_size:	Size of this structure
// @vbt_size:		Size of VBT (VBT Header, BDB Header and data blocks)
// @vbt_checksum:	Checksum
// @reserved0:		Reserved
// @bdb_offset:		Offset of &struct bdb_header from beginning of VBT
// @aim_offset:		Offsets of add-in data blocks from beginning of VBT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbt_header {
    pub signature: [u8; 20],
    pub version: u16,
    pub header_size: u16,
    pub vbt_size: u16,
    pub vbt_checksum: u8,
    pub reserved0: u8,
    pub bdb_offset: u32,
    pub aim_offset: [u32; 4],
    pub __packed: },
//
// struct bdb_header - BDB Header structure
// @signature:		BDB signature "BIOS_DATA_BLOCK"
// @version:		Version of the data block definitions
// @header_size:	Size of this structure
// @bdb_size:		Size of BDB (BDB Header and data blocks)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_header {
    pub signature: [u8; 16],
    pub version: u16,
    pub header_size: u16,
    pub bdb_size: u16,
    pub __packed: },
//
// BDB version number dependencies are documented as:
//
// <start>+
// indicates the field was introduced in version <start>
// and is still valid
//
// <start>-<end>
// indicates the field was introduced in version <start>
// and obsoleted in version <end>+1.
//
// ??? indicates the specific version number is unknown
//
// There are several types of BIOS data blocks (BDBs), each block has
// an ID and size in the first 3 bytes (ID in first, size in next 2).
// Known types are listed below.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bdb_block_id {
    BDB_GENERAL_FEATURES		= 1,
    BDB_GENERAL_DEFINITIONS		= 2,
    BDB_DISPLAY_TOGGLE		= 3,
    BDB_MODE_SUPPORT_LIST		= 4,
    BDB_GENERIC_MODE_TABLE		= 5,
    BDB_EXT_MMIO_REGS		= 6, /* VBIOS only */
    BDB_SWF_IO			= 7, /* VBIOS only */
    BDB_SWF_MMIO			= 8, /* VBIOS only */
    BDB_DOT_CLOCK_OVERRIDE_ALM	= 9,
    BDB_PSR				= 9, /* 165+ */
    BDB_MODE_REMOVAL_TABLE		= 10,
    BDB_CHILD_DEVICE_TABLE		= 11,
    BDB_DRIVER_FEATURES		= 12,
    BDB_DRIVER_PERSISTENCE		= 13,
    BDB_EXT_TABLE_PTRS		= 14, /* VBIOS only */
    BDB_DOT_CLOCK_OVERRIDE		= 15,
    BDB_DISPLAY_SELECT_OLD		= 16,
    BDB_SV_TEST_FUNCTIONS		= 17,
    BDB_DRIVER_ROTATION		= 18,
    BDB_DISPLAY_REMOVE_OLD		= 19,
    BDB_OEM_CUSTOM			= 20,
    BDB_EFP_LIST			= 21, /* workarounds for VGA hsync/vsync */
    BDB_SDVO_LVDS_OPTIONS		= 22,
    BDB_SDVO_LVDS_DTD		= 23,
    BDB_SDVO_LVDS_PNP_ID		= 24,
    BDB_SDVO_LVDS_PPS		= 25,
    BDB_TV_OPTIONS			= 26,
    BDB_EDP				= 27,
    BDB_EFP_DTD			= 28, /* 161+ */
    BDB_DISPLAY_SELECT_IVB		= 29, /* 164+ */
    BDB_DISPLAY_REMOVE_IVB		= 30, /* 164+ */
    BDB_DISPLAY_SELECT_HSW		= 31, /* 166+ */
    BDB_DISPLAY_REMOVE_HSW		= 32, /* 166+ */
    BDB_LFP_OPTIONS			= 40,
    BDB_LFP_DATA_PTRS		= 41,
    BDB_LFP_DATA			= 42,
    BDB_LFP_BACKLIGHT		= 43,
    BDB_LFP_POWER			= 44,
    BDB_EDP_BFI			= 45, /* 160+ */
    BDB_CHROMATICITY		= 46, /* 169+ */
    BDB_MIPI			= 50, /* 170-172 */
    BDB_FIXED_SET_MODE		= 51, /* 172+ */
    BDB_MIPI_CONFIG			= 52, /* 175+ */
    BDB_MIPI_SEQUENCE		= 53, /* 177+ */
    BDB_RGB_PALETTE			= 54, /* 180+ */
    BDB_COMPRESSION_PARAMETERS_OLD	= 55, /* 198-212 */
    BDB_COMPRESSION_PARAMETERS	= 56, /* 213+ */
    BDB_VSWING_PREEMPH		= 57, /* 218+ */
    BDB_GENERIC_DTD			= 58, /* 229+ */
    BDB_INT15_HOOK			= 252, /* VBIOS only */
    BDB_PRD_TABLE			= 253,
    BDB_SKIP			= 254, /* VBIOS only */
}

//
// Block 1 - General Bit Definitions
//
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
    pub underscan_vga_timings:1: u8,
    pub display_clock_mode:1: u8,
    pub vbios_hotplug_support:1: u8,
// bits 3
    pub disable_smooth_vision:1: u8,
    pub single_dvi:1: u8,
    pub /: *mut *mut u8 rotate_180:1; / 181+,
    pub fdi_rx_polarity_inverted:1: u8,
    pub /: *mut *mut u8 vbios_extended_mode:1; / 160+,
    pub /: *mut *mut u8 copy_ilfp_dtd_to_sdvo_lvds_dtd:1; / 160+,
    pub /: *mut *mut u8 panel_best_fit_timing:1; / 160+,
    pub /: *mut *mut u8 ignore_strap_state:1; / 160+,
// bits 4
    pub legacy_monitor_detect: u8,
// bits 5
    pub int_crt_support:1: u8,
    pub int_tv_support:1: u8,
    pub int_efp_support:1: u8,
    pub /: *mut *mut u8 dp_ssc_enable:1; / PCH attached eDP supports SSC,
    pub /: *mut *mut u8 dp_ssc_freq:1; / SSC freq for PCH attached eDP,
    pub dp_ssc_dongle_supported:1: u8,
    pub /: *mut *mut u8 rsvd11:2; / finish byte,
// bits 6
    pub /: *mut *mut u8 tc_hpd_retry_timeout:7; / 242+,
    pub rsvd12:1: u8,
// bits 7
    pub /: *mut *mut u8 afc_startup_config:2; / 249+,
    pub rsvd13:6: u8,
    pub __packed: },
//
// Block 2 - General Bytes Definition
//
// pre-915
pub const GPIO_PIN_DVI_LVDS: c_uint = 0x03 /* "DVI/LVDS DDC GPIO pins" */;
pub const GPIO_PIN_ADD_I2C: c_uint = 0x05 /* "ADDCARD I2C GPIO pins" */;
pub const GPIO_PIN_ADD_DDC: c_uint = 0x04 /* "ADDCARD DDC GPIO pins" */;
pub const GPIO_PIN_ADD_DDC_I2C: c_uint = 0x06 /* "ADDCARD DDC/I2C GPIO pins" */;
// Device handle
pub const DEVICE_HANDLE_CRT: c_uint = 0x0001;
pub const DEVICE_HANDLE_TV: c_uint = 0x0002 /* ???-214 */;
pub const DEVICE_HANDLE_EFP1: c_uint = 0x0004;
pub const DEVICE_HANDLE_EFP2: c_uint = 0x0040;
pub const DEVICE_HANDLE_EFP3: c_uint = 0x0020;
pub const DEVICE_HANDLE_EFP4: c_uint = 0x0010;
pub const DEVICE_HANDLE_EFP5: c_uint = 0x0002 /* 215+ */;
pub const DEVICE_HANDLE_EFP6: c_uint = 0x0001 /* 217+ */;
pub const DEVICE_HANDLE_EFP7: c_uint = 0x0100 /* 217+ */;
pub const DEVICE_HANDLE_EFP8: c_uint = 0x0200 /* 217+ */;
pub const DEVICE_HANDLE_LFP1: c_uint = 0x0008;
pub const DEVICE_HANDLE_LFP2: c_uint = 0x0080;
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
// Add the device class for LFP, TV, HDMI
pub const DEVICE_TYPE_INT_LFP: c_uint = 0x1022;
pub const DEVICE_TYPE_INT_TV: c_uint = 0x1009;
pub const DEVICE_TYPE_HDMI: c_uint = 0x60D2;
pub const DEVICE_TYPE_DP: c_uint = 0x68C6;
pub const DEVICE_TYPE_DP_DUAL_MODE: c_uint = 0x60D6;
pub const DEVICE_TYPE_eDP: c_uint = 0x78C6;

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
// dvo_port pre BDB 155
pub const DEVICE_PORT_DVOA: c_uint = 0x00 /* none on 845+ */;
pub const DEVICE_PORT_DVOB: c_uint = 0x01;
pub const DEVICE_PORT_DVOC: c_uint = 0x02;
// dvo_port BDB 155+
pub const DVO_PORT_HDMIA: c_int = 0;
pub const DVO_PORT_HDMIB: c_int = 1;
pub const DVO_PORT_HDMIC: c_int = 2;
pub const DVO_PORT_HDMID: c_int = 3;
pub const DVO_PORT_LVDS: c_int = 4;
pub const DVO_PORT_TV: c_int = 5;
pub const DVO_PORT_CRT: c_int = 6;
pub const DVO_PORT_DPB: c_int = 7;
pub const DVO_PORT_DPC: c_int = 8;
pub const DVO_PORT_DPD: c_int = 9;
pub const DVO_PORT_DPA: c_int = 10;

pub const LEGACY_CHILD_DEVICE_CONFIG_SIZE: c_int = 33;
// DDC Bus DDI Type 155+
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vbt_gmbus_ddi {
    DDC_BUS_DDI_B = 0x1,
    DDC_BUS_DDI_C,
    DDC_BUS_DDI_D,
    DDC_BUS_DDI_F,
    ICL_DDC_BUS_DDI_A = 0x1,
    ICL_DDC_BUS_DDI_B,
    TGL_DDC_BUS_DDI_C,
    RKL_DDC_BUS_DDI_D = 0x3,
    RKL_DDC_BUS_DDI_E,
    ICL_DDC_BUS_PORT_1 = 0x4,
    ICL_DDC_BUS_PORT_2,
    ICL_DDC_BUS_PORT_3,
    ICL_DDC_BUS_PORT_4,
    TGL_DDC_BUS_PORT_5,
    TGL_DDC_BUS_PORT_6,
    ADLS_DDC_BUS_PORT_TC1 = 0x2,
    ADLS_DDC_BUS_PORT_TC2,
    ADLS_DDC_BUS_PORT_TC3,
    ADLS_DDC_BUS_PORT_TC4,
    ADLP_DDC_BUS_PORT_TC1 = 0x3,
    ADLP_DDC_BUS_PORT_TC2,
    ADLP_DDC_BUS_PORT_TC3,
    ADLP_DDC_BUS_PORT_TC4

}

pub const DP_AUX_A: c_uint = 0x40;
pub const DP_AUX_B: c_uint = 0x10;
pub const DP_AUX_C: c_uint = 0x20;
pub const DP_AUX_D: c_uint = 0x30;
pub const DP_AUX_E: c_uint = 0x50;
pub const DP_AUX_F: c_uint = 0x60;
pub const DP_AUX_G: c_uint = 0x70;
pub const DP_AUX_H: c_uint = 0x80;
pub const DP_AUX_I: c_uint = 0x90;
// DP max link rate 216+
pub const BDB_216_VBT_DP_MAX_LINK_RATE_HBR3: c_int = 0;
pub const BDB_216_VBT_DP_MAX_LINK_RATE_HBR2: c_int = 1;
pub const BDB_216_VBT_DP_MAX_LINK_RATE_HBR: c_int = 2;
pub const BDB_216_VBT_DP_MAX_LINK_RATE_LBR: c_int = 3;
// DP max link rate 230+
pub const BDB_230_VBT_DP_MAX_LINK_RATE_DEF: c_int = 0;
pub const BDB_230_VBT_DP_MAX_LINK_RATE_LBR: c_int = 1;
pub const BDB_230_VBT_DP_MAX_LINK_RATE_HBR: c_int = 2;
pub const BDB_230_VBT_DP_MAX_LINK_RATE_HBR2: c_int = 3;
pub const BDB_230_VBT_DP_MAX_LINK_RATE_HBR3: c_int = 4;
pub const BDB_230_VBT_DP_MAX_LINK_RATE_UHBR10: c_int = 5;
pub const BDB_230_VBT_DP_MAX_LINK_RATE_UHBR13P5: c_int = 6;
pub const BDB_230_VBT_DP_MAX_LINK_RATE_UHBR20: c_int = 7;
// EDP link rate 263+

pub const BDB_263_VBT_EDP_NUM_RATES: c_int = 12;

//
// The child device config, aka the display device data structure, provides a
// description of a port and its configuration on the platform.
//
// The child device config size has been increased, and fields have been added
// and their meaning has changed over time. Care must be taken when accessing
// basically any of the fields to ensure the correct interpretation for the BDB
// version in question.
//
// When we copy the child device configs to display->vbt.child_dev, we
// reserve space for the full structure below, and initialize the tail not
// actually present in VBT to zeros. Accessing those fields is fine, as long as
// the default zero is taken into account, again according to the BDB version.
//
// BDB versions 155 and below are considered legacy, and version 155 seems to be
// a baseline for some of the VBT documentation. When adding new fields, please
// include the BDB version when the field was added, if it's above that.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct child_device_config {
    pub handle: u16,
    pub /: *mut *mut *mut u16 device_type; / See DEVICE_TYPE_ above,
    pub /: *mut *mut u8 device_id[10]; / ascii string,
    pub i2c_speed: u8,
    pub /: *mut *mut u8 dp_onboard_redriver_preemph:3; / 158+,
    pub /: *mut *mut u8 dp_onboard_redriver_vswing:3; / 158+,
    pub /: *mut *mut u8 dp_onboard_redriver_present:1; / 158+,
    pub reserved0:1: u8,
    pub /: *mut *mut u8 dp_ondock_redriver_preemph:3; / 158+,
    pub /: *mut *mut u8 dp_ondock_redriver_vswing:3; / 158+,
    pub /: *mut *mut u8 dp_ondock_redriver_present:1; / 158+,
    pub reserved1:1: u8,
    pub /: *mut *mut u8 hdmi_level_shifter_value:5; / 158+,
    pub /: *mut *mut u8 hdmi_max_data_rate:3; / 204+,
    pub /: *mut *mut u16 dtd_buf_ptr; / 161+,
    pub /: *mut *mut u8 edidless_efp:1; / 161+,
    pub /: *mut *mut u8 compression_enable:1; / 198+,
    pub /: *mut *mut u8 compression_method_cps:1; / 198+,
    pub /: *mut *mut u8 ganged_edp:1; / 202+,
    pub /: *mut *mut u8 lttpr_non_transparent:1; / 235+,
    pub /: *mut *mut u8 disable_compression_for_ext_disp:1; / 251+,
    pub reserved2:2: u8,
    pub /: *mut *mut u8 compression_structure_index:4; / 198+,
    pub reserved3:4: u8,
    pub /: *mut *mut u8 hdmi_max_frl_rate:4; / 237+,
    pub /: *mut *mut u8 hdmi_max_frl_rate_valid:1; / 237+,
    pub /: *mut *mut u8 reserved4:3; / 237+,
    pub reserved5: u8,
    pub __packed: },
    pub __packed: },
    pub addin_offset: u16,
    pub /: *mut *mut *mut *mut u8 dvo_port; / See DEVICE_PORT_ and DVO_PORT_ above,
    pub i2c_pin: u8,
    pub target_addr: u8,
    pub ddc_pin: u8,
    pub edid_ptr: u16,
    pub /: *mut *mut *mut u8 dvo_cfg; / See DEVICE_CFG_ above,
    pub dvo2_port: u8,
    pub i2c2_pin: u8,
    pub target2_addr: u8,
    pub ddc2_pin: u8,
    pub __packed: },
    pub /: *mut *mut u8 efp_routed:1; / 158+,
    pub /: *mut *mut u8 lane_reversal:1; / 184+,
    pub /: *mut *mut u8 lspcon:1; / 192+,
    pub /: *mut *mut u8 iboost:1; / 196+,
    pub /: *mut *mut u8 hpd_invert:1; / 196+,
    pub /: *mut *mut u8 use_vbt_vswing:1; / 218+,
    pub /: *mut *mut u8 dp_max_lane_count:2; / 244+,
    pub /: *mut *mut u8 hdmi_support:1; / 158+,
    pub /: *mut *mut u8 dp_support:1; / 158+,
    pub /: *mut *mut u8 tmds_support:1; / 158+,
    pub support_reserved:5: u8,
    pub aux_channel: u8,
    pub dongle_detect: u8,
    pub __packed: },
    pub __packed: },
    pub pipe_cap:2: u8,
    pub /: *mut *mut u8 sdvo_stall:1; / 158+,
    pub hpd_status:2: u8,
    pub integrated_encoder:1: u8,
    pub capabilities_reserved:2: u8,
    pub /: *mut *mut *mut u8 dvo_wiring; / See DEVICE_WIRE_ above,
    pub dvo2_wiring: u8,
    pub /: *mut *mut u8 mipi_bridge_type; / 171+,
    pub __packed: },
    pub extended_type: u16,
    pub dvo_function: u8,
    pub /: *mut *mut u8 dp_usb_type_c:1; / 195+,
    pub /: *mut *mut u8 tbt:1; / 209+,
    pub /: *mut *mut u8 dedicated_external:1; / 264+,
    pub /: *mut *mut u8 dyn_port_over_tc:1; / 264+,
    pub /: *mut *mut u8 dp_port_trace_length:4; / 209+,
    pub /: *mut *mut u8 dp_gpio_index; / 195+,
    pub /: *mut *mut u16 dp_gpio_pin_num; / 195+,
    pub /: *mut *mut u8 dp_iboost_level:4; / 196+,
    pub /: *mut *mut u8 hdmi_iboost_level:4; / 196+,
    pub /: *mut *mut u8 dp_max_link_rate:3; / 216+,
    pub /: *mut *mut u8 dp_max_link_rate_reserved:5; / 216+,
    pub /: *mut *mut u8 efp_index; / 256+,
    pub /: *mut *mut u32 edp_data_rate_override:12; / 263+,
    pub /: *mut *mut u32 edp_data_rate_override_reserved:20; / 263+,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_general_definitions {
// DDC GPIO
    pub crt_ddc_gmbus_pin: u8,
// DPMS bits
    pub dpms_non_acpi:1: u8,
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
// defs->child_dev_size;
//
    pub devices: [u8; ],
    pub __packed: },
//
// Block 3 - Display Toggle Option Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_display_toggle {
    pub feature_bits: u8,
    pub /: *mut *mut u16 num_entries; / ALM only,
    pub /: *mut *mut u16 list[]; / ALM only,
    pub __packed: },
//
// Block 4 - Mode Support List
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_mode_support_list {
    pub intel_mode_number: [u8; 0],
    pub mode_list_length: u16,
    pub __packed: },
//
// Block 5 - Generic Mode Table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct generic_mode_table {
    pub x_res: u16,
    pub y_res: u16,
    pub color_depths: u8,
    pub refresh_rate: [u8; 3],
    pub reserved: u8,
    pub text_cols: u8,
    pub text_rows: u8,
    pub font_height: u8,
    pub page_size: u16,
    pub misc: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct generic_mode_timings {
    pub dotclock_khz: u32,
    pub hdisplay: u16,
    pub htotal: u16,
    pub hblank_start: u16,
    pub hblank_end: u16,
    pub hsync_start: u16,
    pub hsync_end: u16,
    pub vdisplay: u16,
    pub vtotal: u16,
    pub vblank_start: u16,
    pub vblank_end: u16,
    pub vsync_start: u16,
    pub vsync_end: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct generic_mode_timings_alm {
    pub timings: generic_mode_timings,
    pub wm_8bpp: u8,
    pub burst_8bpp: u8,
    pub wm_16bpp: u8,
    pub burst_16bpp: u8,
    pub wm_32bpp: u8,
    pub burst_32bpp: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_generic_mode_table_alm {
    pub table: generic_mode_table,
    pub timings: [generic_mode_timings_alm; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_generic_mode_table_mgm {
    pub mode_flag: u16,
    pub table: generic_mode_table,
    pub timings: [generic_mode_timings; 3],
    pub __packed: },
//
// Block 6 - Extended MMIO Register Table, VBIOS only
// Block 7 - IO Software Flag Table, VBIOS only
// Block 8 - MMIO SWF Register Table, VBIOS only
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_reg_table {
    pub table_id: u16,
    pub data_access_size: u8,
//
// offset,value tuples:
// data_access_size==0xce -> u8,u8
// data_access_size==0x02 -> u32,u32
//
// u16 table_end_marker;
    pub __packed: },
//
// Block 9 - Undocumented table (ALM only)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dot_clock_override_entry_gen2 {
    pub dotclock: u32,
    pub n: u8,
    pub m1: u8,
    pub m2: u8,
    pub p1:5: u8,
    pub p1_div_by_2:1: u8,
    pub reserved:1: u8,
    pub p2_div_by_4:1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_dot_clock_override_alm {
    pub t: [dot_clock_override_entry_gen2; 0],
    pub __packed: },
//
// Block 9 - SRD Feature Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psr_table {
// Feature bits
    pub /: *mut *mut u8 full_link:1; / 165+,
    pub /: *mut *mut u8 require_aux_to_wakeup:1; / 165+,
    pub feature_bits_rsvd:6: u8,
// Wait times
    pub /: *mut *mut u8 idle_frames:4; / 165+,
    pub /: *mut *mut u8 lines_to_wait:3; / 165+,
    pub wait_times_rsvd:1: u8,
// TP wake up time in multiple of 100
    pub /: *mut *mut u16 tp1_wakeup_time; / 165+,
    pub /: *mut *mut u16 tp2_tp3_wakeup_time; / 165+,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_psr {
    pub psr_table: [psr_table; 16],
// PSR2 TP2/TP3 wakeup time for 16 panels
    pub /: *mut *mut u32 psr2_tp2_tp3_wakeup_time; / 226+,
    pub __packed: },
//
// Block 10 - Mode Removal Table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mode_removal_table {
    pub x_res: u16,
    pub y_res: u16,
    pub bpp: u8,
    pub refresh_rate: u16,
    pub removal_flags: u8,
    pub panel_flags: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_mode_removal {
    pub /: *mut *mut u8 row_size; / 8 or 10 bytes,
//
// VBT spec says this is always 20 entries,
// but ALM seems to have only 15 entries.
//
    pub modes: [mode_removal_table; ],
// u16 terminator; 0x0000
    pub __packed: },
//
// Block 12 - Driver Features Data Block
//
pub const BDB_DRIVER_FEATURE_NO_LVDS: c_int = 0;
pub const BDB_DRIVER_FEATURE_INT_LVDS: c_int = 1;
pub const BDB_DRIVER_FEATURE_SDVO_LVDS: c_int = 2;
pub const BDB_DRIVER_FEATURE_INT_SDVO_LVDS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_driver_features {
// Driver bits
    pub boot_dev_algorithm:1: u8,
    pub allow_display_switch_dvd:1: u8,
    pub allow_display_switch_dos:1: u8,
    pub hotplug_dvo:1: u8,
    pub dual_view_zoom:1: u8,
    pub int15h_hook:1: u8,
    pub sprite_in_clone:1: u8,
    pub primary_lfp_id:1: u8,
    pub boot_mode_x: u16,
    pub boot_mode_y: u16,
    pub boot_mode_bpp: u8,
    pub boot_mode_refresh: u8,
// Extended Driver Bits 1
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
// Driver Flags 1
    pub /: *mut *mut u8 static_display:1; / 163+,
    pub /: *mut *mut u8 embedded_platform:1; / 163+,
    pub /: *mut *mut u8 display_subsystem_enable:1; / 163+,
    pub reserved0:5: u8,
    pub legacy_crt_max_x: u16,
    pub legacy_crt_max_y: u16,
    pub legacy_crt_max_refresh: u8,
// Extended Driver Bits 2
    pub hdmi_termination:1: u8,
    pub cea861d_hdmi_support:1: u8,
    pub self_refresh_enable:1: u8,
    pub reserved1:5: u8,
    pub /: *mut *mut u8 custom_vbt_version; / 155+,
// Driver Feature Flags
    pub /: *mut *mut u16 rmpm_enabled:1; / 159+,
    pub /: *mut *mut u16 s2ddt_enabled:1; / 159+,
    pub /: *mut *mut u16 dpst_enabled:1; / 159-227,
    pub /: *mut *mut u16 bltclt_enabled:1; / 159+,
    pub /: *mut *mut u16 adb_enabled:1; / 159-227,
    pub /: *mut *mut u16 drrs_enabled:1; / 159-227,
    pub /: *mut *mut u16 grs_enabled:1; / 159+,
    pub /: *mut *mut u16 gpmt_enabled:1; / 159+,
    pub /: *mut *mut u16 tbt_enabled:1; / 159+,
    pub /: *mut *mut u16 psr_enabled:1; / 165-227,
    pub /: *mut *mut u16 ips_enabled:1; / 165+,
    pub /: *mut *mut u16 dfps_enabled:1; / 165+,
    pub /: *mut *mut u16 dmrrs_enabled:1; / 174-227,
    pub /: *mut *mut u16 adt_enabled:1; / ???-228,
    pub /: *mut *mut u16 hpd_wake:1; / 201-240,
    pub /: *mut *mut u16 pc_feature_valid:1; / 159+,
    pub __packed: },
//
// Block 13 - Driver Persistent Algorithm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_driver_persistence {
    pub hotkey_persistent_algorithm:1: u16,
    pub lid_switch_persistent_algorithm:1: u16,
    pub power_management_persistent_algorithm:1: u16,
    pub hotkey_persistent_on_mds_twin:1: u16,
    pub hotkey_persistent_on_refresh_rate:1: u16,
    pub hotkey_persistent_on_restore_pipe:1: u16,
    pub hotkey_persistent_on_mode:1: u16,
    pub edid_persistent_on_mode:1: u16,
    pub dvo_hotplug_persistent_on_mode:1: u16,
    pub docking_persistent_algorithm:1: u16,
    pub rsvd:6: u16,
    pub persistent_max_config: u8,
    pub __packed: },
//
// Block 15 - Dot Clock Override Table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dot_clock_override_entry_gen3 {
    pub dotclock: u32,
    pub n: u8,
    pub m1: u8,
    pub m2: u8,
    pub p1: u8,
    pub p2: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_dot_clock_override {
    pub /: *mut *mut u8 row_size; / 8 == gen2, 9 == gen3+,
    pub num_rows: u8,
    pub /: *mut *mut dot_clock_override_entry_gen3 table[]; / or _gen2,
    pub __packed: },
//
// Block 16 - Toggle List Block (pre-HSW)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toggle_list_entry_old {
    pub display_select_pipe_a: u8,
    pub display_select_pipe_b: u8,
    pub caps: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toggle_list_table_old {
    pub num_entries: u16,
    pub entry_size: u8,
    pub list: [toggle_list_entry_old; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_display_select_old {
// each table has variable size!
    pub tables: [toggle_list_table_old; 4],
    pub __packed: },
//
// Block 17 - SV Test Functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_sv_test_functions {
    pub sv_bits: [u8; 8],
    pub __packed: },
//
// Block 18 - Driver Rotation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_driver_rotation {
    pub rotation_enable: u8,
    pub rotation_flags_1: u8,
    pub rotation_flags_2: u16,
    pub rotation_flags_3: u32,
    pub rotation_flags_4: u32,
    pub __packed: },
//
// Block 19 - Display Configuration Removal Table (pre-IVB)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_remove_entry_old {
    pub display_select_pipe_a: u8,
    pub display_select_pipe_b: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_display_remove_old {
    pub num_entries: u8,
    pub entry_size: u8,
    pub table: [display_remove_entry_old; ],
    pub __packed: },
//
// Block 20 - OEM Customizable Modes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct oem_mode {
    pub enable_in_vbios:1: u8,
    pub enable_in_os:1: u8,
    pub /: *mut *mut u8 enable_in_gop:1; / 207+,
    pub reserved:5: u8,
    pub /: *mut *mut u8 display_flags; / ???-216,
    pub x_res: u16,
    pub y_res: u16,
    pub color_depth: u8,
    pub refresh_rate: u8,
    pub dtd: bdb_edid_dtd,
    pub /: *mut *mut u16 display_flags_2; / 217+,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_oem_custom {
    pub num_entries: u8,
    pub entry_size: u8,
    pub modes: [oem_mode; ],
    pub __packed: },
//
// Block 21 - EFP List
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efp_entry {
    pub mfg_name: u16,
    pub product_code: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_efp_list {
    pub num_entries: u8,
    pub entry_size: u8,
    pub efp: [efp_entry; ],
    pub __packed: },
//
// Block 22 - SDVO LVDS General Options
//
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
//
// Block 23 - SDVO LVDS DTD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_sdvo_lvds_dtd {
    pub dtd: [bdb_edid_dtd; 4],
    pub __packed: },
//
// Block 24 - SDVO LVDS PnP ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_sdvo_lvds_pnp_id {
    pub pnp_id: [bdb_edid_pnp_id; 4],
    pub __packed: },
//
// Block 25 - SDVO LVDS PPS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdvo_lvds_pps {
    pub /: *mut *mut u16 t0; / power on,
    pub /: *mut *mut u16 t1; / backlight on,
    pub /: *mut *mut u16 t2; / backlight off,
    pub /: *mut *mut u16 t3; / power off,
    pub /: *mut *mut u16 t4; / power cycle,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_sdvo_lvds_pps {
    pub pps: [sdvo_lvds_pps; 4],
    pub __packed: },
//
// Block 26 - TV Options Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_tv_options {
    pub underscan_overscan_hdtv_component:2: u16,
    pub rsvd1:10: u16,
    pub underscan_overscan_hdtv_dvi:2: u16,
    pub add_modes_to_avoid_overscan_issue:1: u16,
    pub d_connector_support:1: u16,
    pub __packed: },
//
// Block 27 - eDP VBT Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edp_power_seq {
    pub t1_t3: u16,
    pub t8: u16,
    pub t9: u16,
    pub t10: u16,
    pub t11_t12: u16,
    pub __packed: },
pub const EDP_18BPP: c_int = 0;
pub const EDP_24BPP: c_int = 1;
pub const EDP_30BPP: c_int = 2;
pub const EDP_RATE_1_62: c_int = 0;
pub const EDP_RATE_2_7: c_int = 1;
pub const EDP_RATE_5_4: c_int = 2;
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
pub struct edp_fast_link_params {
    pub /: *mut *mut u8 rate:4; / ???-223,
    pub lanes:4: u8,
    pub preemphasis:4: u8,
    pub vswing:4: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edp_pwm_delays {
    pub pwm_on_to_backlight_enable: u16,
    pub backlight_disable_to_pwm_off: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edp_full_link_params {
    pub preemphasis:4: u8,
    pub vswing:4: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edp_apical_params {
    pub panel_oui: u32,
    pub dpcd_base_address: u32,
    pub dpcd_idridix_control_0: u32,
    pub dpcd_option_select: u32,
    pub dpcd_backlight: u32,
    pub ambient_light: u32,
    pub backlight_scale: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_edp {
    pub power_seqs: [edp_power_seq; 16],
    pub color_depth: u32,
    pub fast_link_params: [edp_fast_link_params; 16],
    pub sdrrs_msa_timing_delay: u32,
// ith bit indicates enabled/disabled for (i+1)th panel
    pub /: *mut *mut u16 edp_s3d_feature; / 162+,
    pub /: *mut *mut u16 edp_t3_optimization; / 165+,
    pub /: *mut *mut u64 edp_vswing_preemph; / 173+,
    pub /: *mut *mut u16 fast_link_training; / 182+,
    pub /: *mut *mut u16 dpcd_600h_write_required; / 185+,
    pub /: *mut *mut edp_pwm_delays pwm_delays[16]; / 186+,
    pub /: *mut *mut u16 full_link_params_provided; / 199+,
    pub /: *mut *mut edp_full_link_params full_link_params[16]; / 199+,
    pub /: *mut *mut u16 apical_enable; / 203+,
    pub /: *mut *mut edp_apical_params apical_params[16]; / 203+,
    pub /: *mut *mut u16 edp_fast_link_training_rate[16]; / 224+,
    pub /: *mut *mut u16 edp_max_port_link_rate[16]; / 244+,
    pub /: *mut *mut u16 edp_dsc_disable; / 251+,
    pub /: *mut *mut u16 t6_delay_support; / 260+,
    pub /: *mut *mut u16 link_idle_time[16]; / 260+,
    pub /: *mut *mut u16 pipe_joiner_enable; / 261+,
    pub __packed: },
//
// Block 28 - EFP DTD Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_efp_dtd {
    pub dtd: [bdb_edid_dtd; 3],
    pub __packed: },
//
// Block 29 - Toggle List Block (IVB)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toggle_list_entry_ivb {
    pub display_select: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toggle_list_table_ivb {
    pub num_entries: u16,
    pub entry_size: u8,
    pub list: [toggle_list_entry_ivb; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_display_select_ivb {
// each table has variable size!
    pub tables: [toggle_list_table_ivb; 4],
    pub __packed: },
//
// Block 30 - Display Configuration Removal Table (IVB)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_remove_entry_ivb {
    pub display_select: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_display_remove_ivb {
    pub num_entries: u8,
    pub entry_size: u8,
    pub table: [display_remove_entry_ivb; ],
    pub __packed: },
//
// Block 31 - Toggle List Block (HSW+)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toggle_list_entry_hsw {
    pub display_select: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toggle_list_table_hsw {
    pub num_entries: u16,
    pub entry_size: u8,
    pub list: [toggle_list_entry_hsw; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_display_select_hsw {
// each table has variable size!
    pub tables: [toggle_list_table_hsw; 4],
    pub __packed: },
//
// Block 32 - Display Configuration Removal Table (HSW+)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_remove_entry_hsw {
    pub display_select: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_display_remove_hsw {
    pub num_entries: u8,
    pub entry_size: u8,
    pub table: [display_remove_entry_hsw; ],
    pub __packed: },
//
// Block 40 - LFP Data Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_lfp_options {
    pub panel_type: u8,
    pub /: *mut *mut u8 panel_type2; / 212+,
// LVDS capabilities, stored in a dword
    pub pfit_mode:2: u8,
    pub pfit_text_mode_enhanced:1: u8,
    pub pfit_gfx_mode_enhanced:1: u8,
    pub pfit_ratio_auto:1: u8,
    pub pixel_dither:1: u8,
    pub /: *mut *mut u8 lvds_edid:1; / ???-240,
    pub rsvd2:1: u8,
    pub rsvd4: u8,
// LVDS Panel channel bits stored here
    pub lvds_panel_channel_bits: u32,
// LVDS SSC (Spread Spectrum Clock) bits stored here.
    pub ssc_bits: u16,
    pub ssc_freq: u16,
    pub ssc_ddt: u16,
// Panel color depth defined here
    pub panel_color_depth: u16,
// LVDS panel type bits stored here
    pub dps_panel_type_bits: u32,
// LVDS backlight control type bits stored here
    pub /: *mut *mut u32 blt_control_type_bits; / ???-240,
    pub /: *mut *mut u16 lcdvcc_s0_enable; / 200+,
    pub /: *mut *mut u32 rotation; / 228+,
    pub /: *mut *mut u32 position; / 240+,
    pub __packed: },
//
// Block 41 - LFP Data Table Pointers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lfp_data_ptr_table {
    pub /: *mut *mut u16 offset; / offsets are from start of bdb,
    pub table_size: u8,
    pub __packed: },
// LFP pointer table contains entries to the struct below
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lfp_data_ptr {
    pub fp_timing: lfp_data_ptr_table,
    pub dvo_timing: lfp_data_ptr_table,
    pub panel_pnp_id: lfp_data_ptr_table,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_lfp_data_ptrs {
    pub num_entries: u8,
    pub ptr: [lfp_data_ptr; 16],
    pub /: *mut *mut lfp_data_ptr_table panel_name; / (156-163?)+,
    pub __packed: },
//
// Block 42 - LFP Data Tables
//
// LFP data has 3 blocks per entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fp_timing {
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
//
// For reference only. fp_timing has variable size so
// the data must be accessed using the data table pointers.
// Do not use this directly!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lfp_data_entry {
    pub fp_timing: fp_timing,
    pub dvo_timing: bdb_edid_dtd,
    pub pnp_id: bdb_edid_pnp_id,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_lfp_data {
    pub data: [lfp_data_entry; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lfp_black_border {
    pub /: *mut *mut u8 top; / 227+,
    pub /: *mut *mut u8 bottom; / 227+,
    pub /: *mut *mut u8 left; / 238+,
    pub /: *mut *mut u8 right; / 238+,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_lfp_data_tail {
    pub /: *mut *mut bdb_edid_product_name panel_name[16]; / (156-163?)+,
    pub /: *mut *mut u16 scaling_enable; / 187+,
    pub /: *mut *mut u8 seamless_drrs_min_refresh_rate[16]; / 188+,
    pub /: *mut *mut u8 pixel_overlap_count[16]; / 208+,
    pub /: *mut *mut lfp_black_border black_border[16]; / 227+,
    pub /: *mut *mut u16 dual_lfp_port_sync_enable; / 231+,
    pub /: *mut *mut u16 gpu_dithering_for_banding_artifacts; / 245+,
    pub __packed: },
//
// Block 43 - LFP Backlight Control Data Block
//
pub const BDB_BACKLIGHT_TYPE_NONE: c_int = 0;
pub const BDB_BACKLIGHT_TYPE_PWM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lfp_backlight_data_entry {
    pub type:2: u8,
    pub active_low_pwm:1: u8,
    pub /: *mut *mut u8 i2c_pin:3; / obsolete since ?,
    pub /: *mut *mut u8 i2c_speed:2; / obsolete since ?,
    pub pwm_freq_hz: u16,
    pub /: *mut *mut u8 min_brightness; / ???-233,
    pub /: *mut *mut u8 i2c_address; / obsolete since ?,
    pub /: *mut *mut u8 i2c_command; / obsolete since ?,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lfp_backlight_control_method {
    pub type:4: u8,
    pub controller:4: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lfp_brightness_level {
    pub level: u16,
    pub reserved: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_lfp_backlight {
    pub entry_size: u8,
    pub data: [lfp_backlight_data_entry; 16],
    pub /: *mut *mut u8 level[16]; / 162-233,
    pub /: *mut *mut lfp_backlight_control_method backlight_control[16]; / 191+,
    pub /: *mut *mut lfp_brightness_level brightness_level[16]; / 234+,
    pub /: *mut *mut lfp_brightness_level brightness_min_level[16]; / 234+,
    pub /: *mut *mut u8 brightness_precision_bits[16]; / 236+,
    pub /: *mut *mut u16 hdr_dpcd_refresh_timeout[16]; / 239+,
    pub __packed: },
//
// Block 44 - LFP Power Conservation Features Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lfp_power_features {
    pub /: *mut *mut u8 dpst_support:1; / ???-159,
    pub power_conservation_pref:3: u8,
    pub reserved2:1: u8,
    pub /: *mut *mut u8 lace_enabled_status:1; / 210+,
    pub /: *mut *mut u8 lace_support:1; / 210+,
    pub als_enable:1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct als_data_entry {
    pub backlight_adjust: u16,
    pub lux: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aggressiveness_profile_entry {
    pub /: *mut *mut u8 dpst_aggressiveness : 4; / (228/252)-256,
    pub 4: u8 lace_aggressiveness :,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aggressiveness_profile2_entry {
    pub 4: u8 opst_aggressiveness :,
    pub 4: u8 elp_aggressiveness :,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aggressiveness_profile3_entry {
    pub apd_aggressiveness:4: u8,
    pub pixoptix_aggressiveness:4: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aggressiveness_profile4_entry {
    pub xpst_aggressiveness:4: u8,
    pub tcon_aggressiveness:4: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panel_identification {
    pub panel_technology:4: u8,
    pub reserved:4: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_lfp_power {
    pub /: *mut *mut lfp_power_features features; / ???-227,
    pub als: [als_data_entry; 5],
    pub /: *mut *mut u8 lace_aggressiveness_profile:3; / 210-227,
    pub reserved1:5: u8,
    pub /: *mut *mut u16 dpst; / 228-256,
    pub /: *mut *mut u16 psr; / 228+,
    pub /: *mut *mut u16 drrs; / 228+,
    pub /: *mut *mut u16 lace_support; / 228+,
    pub /: *mut *mut u16 adt; / 228+,
    pub /: *mut *mut u16 dmrrs; / 228+,
    pub /: *mut *mut u16 adb; / 228+,
    pub /: *mut *mut u16 lace_enabled_status; / 228+,
    pub aggressiveness: [aggressiveness_profile_entry; 16],
    pub /: *mut *mut u16 hobl; / 232+,
    pub /: *mut *mut u16 vrr_feature_enabled; / 233+,
    pub /: *mut *mut u16 elp; / 247-256,
    pub /: *mut *mut u16 opst; / 247-256,
    pub /: *mut *mut aggressiveness_profile2_entry aggressiveness2[16]; / 247-256,
    pub /: *mut *mut u16 apd; / 253-256,
    pub /: *mut *mut u16 pixoptix; / 253-256,
    pub /: *mut *mut aggressiveness_profile3_entry aggressiveness3[16]; / 253-256,
    pub /: *mut *mut panel_identification panel_identification[16]; / 257+,
    pub /: *mut *mut u16 xpst_support; / 257+,
    pub /: *mut *mut u16 tcon_based_backlight_optimization; / 257+,
    pub /: *mut *mut aggressiveness_profile4_entry aggressiveness4[16]; / 257+,
    pub /: *mut *mut u16 tcon_backlight_xpst_coexistence; / 257+,
    pub __packed: },
//
// Block 45 - eDP BFI Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edp_bfi {
    pub enable_bfi_in_driver:1: u8,
    pub enable_brightness_control_in_cui:1: u8,
    pub reserved:6: u8,
    pub brightness_percentage_when_bfi_disabled: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_edp_bfi {
    pub bfi_structure_size: u8,
    pub bfi: [edp_bfi; 16],
    pub __packed: },
//
// Block 46 - Chromaticity For Narrow Gamut Panel Configuration Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chromaticity {
    pub chromaticity_enable:1: u8,
    pub chromaticity_from_edid_base_block:1: u8,
    pub rsvd:6: u8,
    pub green_y_lo:2: u8,
    pub green_x_lo:2: u8,
    pub red_y_lo:2: u8,
    pub red_x_lo:2: u8,
    pub white_y_lo:2: u8,
    pub white_x_lo:2: u8,
    pub blue_y_lo:2: u8,
    pub blue_x_lo:2: u8,
    pub red_x_hi: u8,
    pub red_y_hi: u8,
    pub green_x_hi: u8,
    pub green_y_hi: u8,
    pub blue_x_hi: u8,
    pub blue_y_hi: u8,
    pub white_x_hi: u8,
    pub white_y_hi: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luminance_and_gamma {
    pub /: *mut *mut u8 luminance_enable:1; / 211+,
    pub /: *mut *mut u8 gamma_enable:1; / 211+,
    pub rsvd:6: u8,
    pub /: *mut *mut u16 min_luminance; / 211+,
    pub /: *mut *mut u16 max_luminance; / 211+,
    pub /: *mut *mut u16 one_percent_max_luminance; / 211+,
    pub /: *mut *mut u8 gamma; / 211+,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_chromaticity {
    pub chromaticity: [chromaticity; 16],
    pub /: *mut *mut luminance_and_gamma luminance_and_gamma[16]; / 211+,
    pub __packed: },
//
// Block 50 - MIPI Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_data {
    pub panel_identifier: u16,
    pub bridge_revision: u16,
    pub dithering:1: u32,
    pub pixel_format_18bpp:1: u32,
    pub reserved1:1: u32,
    pub dphy_params_valid:1: u32,
    pub reserved2:28: u32,
    pub port_info: u16,
    pub reserved3:2: u16,
    pub num_lanes:2: u16,
    pub reserved4:12: u16,
    pub virtual_channel_num:2: u16,
    pub video_transfer_mode:2: u16,
    pub reserved5:12: u16,
    pub dsi_ddr_clock: u32,
    pub renesas_bridge_ref_clock: u32,
    pub power_conservation: u16,
    pub prepare_count:5: u32,
    pub reserved6:3: u32,
    pub clk_zero_count:8: u32,
    pub trail_count:5: u32,
    pub reserved7:3: u32,
    pub exit_zero_count:6: u32,
    pub reserved8:2: u32,
    pub high_low_switch_count: u32,
    pub lp_byte_clock: u32,
    pub clock_lane_switch_time_counter: u32,
    pub panel_color_depth: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_mipi {
    pub mipi: [mipi_data; 16],
    pub __packed: },
//
// Block 51 - Fixed Set Mode Table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_fixed_set_mode {
    pub enable: u8,
    pub x_res: u32,
    pub y_res: u32,
    pub __packed: },
//
// Block 52 - MIPI Configuration Block
//
pub const MAX_MIPI_CONFIGURATIONS: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_mipi_config {
    pub /: *mut *mut mipi_config config[MAX_MIPI_CONFIGURATIONS]; / 175+,
    pub /: *mut *mut mipi_pps_data pps[MAX_MIPI_CONFIGURATIONS]; / 177+,
    pub /: *mut *mut edp_pwm_delays pwm_delays[MAX_MIPI_CONFIGURATIONS]; / 186+,
    pub /: *mut *mut u8 pmic_i2c_bus_number[MAX_MIPI_CONFIGURATIONS]; / 190+,
    pub __packed: },
//
// Block 53 - MIPI Sequence Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_mipi_sequence {
    pub version: u8,
    pub /: *mut *mut u8 data[]; / up to 6 variable length blocks,
    pub __packed: },
//
// Block 55 - RGB Palette Table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_rgb_palette {
    pub is_enabled: u8,
    pub red: [u8; 256],
    pub blue: [u8; 256],
    pub green: [u8; 256],
    pub __packed: },
//
// Block 56 - Compression Parameters
//
pub const VBT_RC_BUFFER_BLOCK_SIZE_1KB: c_int = 0;
pub const VBT_RC_BUFFER_BLOCK_SIZE_4KB: c_int = 1;
pub const VBT_RC_BUFFER_BLOCK_SIZE_16KB: c_int = 2;
pub const VBT_RC_BUFFER_BLOCK_SIZE_64KB: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_compression_parameters_entry {
    pub version_major:4: u8,
    pub version_minor:4: u8,
    pub rc_buffer_block_size:2: u8,
    pub reserved1:6: u8,
//
// Buffer size in bytes:
//
// 4 ^ rc_buffer_block_size * 1024 * (rc_buffer_size + 1) bytes
//
    pub rc_buffer_size: u8,
    pub slices_per_line: u32,
    pub line_buffer_depth:4: u8,
    pub reserved2:4: u8,
// Flag Bits 1
    pub block_prediction_enable:1: u8,
    pub reserved3:7: u8,
    pub /: *mut *mut u8 max_bpp; / mapping,
// Color depth capabilities
    pub reserved4:1: u8,
    pub support_8bpc:1: u8,
    pub support_10bpc:1: u8,
    pub support_12bpc:1: u8,
    pub reserved5:4: u8,
    pub slice_height: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_compression_parameters {
    pub entry_size: u16,
    pub data: [dsc_compression_parameters_entry; 16],
    pub __packed: },
//
// Block 57 -  Vswing PreEmphasis Table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_vswing_preemph {
    pub num_tables: u8,
    pub num_columns: u8,
    pub tables: [u32; ],
    pub __packed: },
//
// Block 58 - Generic DTD Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct generic_dtd_entry {
    pub pixel_clock: u32,
    pub hactive: u16,
    pub hblank: u16,
    pub hfront_porch: u16,
    pub hsync: u16,
    pub vactive: u16,
    pub vblank: u16,
    pub vfront_porch: u16,
    pub vsync: u16,
    pub width_mm: u16,
    pub height_mm: u16,
// Flags
    pub rsvd_flags:6: u8,
    pub vsync_positive_polarity:1: u8,
    pub hsync_positive_polarity:1: u8,
    pub rsvd: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_generic_dtd {
    pub gdtd_size: u16,
    pub /: *mut *mut generic_dtd_entry dtd[]; / up to 24 DTD's,
    pub __packed: },
//
// Block 253 - PRD Table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prd_entry_old {
    pub displays_attached: u8,
    pub display_in_pipe_a: u8,
    pub display_in_pipe_b: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_prd_table_old {
    pub /: *mut *mut prd_entry_old list[0]; / ???-216,
    pub /: *mut *mut u16 num_entries; / ???-216,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prd_entry_new {
    pub primary_display: u16,
    pub secondary_display: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdb_prd_table_new {
    pub /: *mut *mut u16 num_entries; / 217+,
    pub /: *mut *mut prd_entry_new list[]; / 217+,
    pub __packed: },
