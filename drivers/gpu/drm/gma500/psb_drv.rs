//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/gma500/psb_drv.h
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
// Copyright (c) 2007-2011, Intel Corporation.
// All Rights Reserved.
//

pub const DRIVER_MAJOR: c_int = 1;
pub const DRIVER_MINOR: c_int = 0;
pub const DRIVER_PATCHLEVEL: c_int = 0;
// Append new drm mode definition here, align with libdrm definition
pub const DRM_MODE_SCALE_NO_SCALE: c_int = 2;

// Hardware offsets
pub const PSB_VDC_OFFSET: c_uint = 0x00000000;
pub const PSB_VDC_SIZE: c_uint = 0x000080000;
pub const MRST_MMIO_SIZE: c_uint = 0x0000C0000;
pub const PSB_SGX_SIZE: c_uint = 0x8000;
pub const PSB_SGX_OFFSET: c_uint = 0x00040000;
pub const MRST_SGX_OFFSET: c_uint = 0x00080000;
// PCI resource identifiers
pub const PSB_MMIO_RESOURCE: c_int = 0;
pub const PSB_AUX_RESOURCE: c_int = 0;
pub const PSB_GATT_RESOURCE: c_int = 2;
pub const PSB_GTT_RESOURCE: c_int = 3;
// PCI configuration
pub const PSB_GMCH_CTRL: c_uint = 0x52;
pub const PSB_BSM: c_uint = 0x5C;
pub const _PSB_GMCH_ENABLED: c_uint = 0x4;
pub const PSB_PGETBL_CTL: c_uint = 0x2020;
pub const _PSB_PGETBL_ENABLED: c_uint = 0x00000001;
pub const PSB_SGX_2D_SLAVE_PORT: c_uint = 0x4000;
pub const PSB_LPC_GBA: c_uint = 0x44;
// TODO: To get rid of

// SGX side MMU definitions (these can probably go)
// Flags for external memory type field
pub const PSB_MMU_CACHED_MEMORY: c_uint = 0x0001	/* Bind to MMU only */;
pub const PSB_MMU_RO_MEMORY: c_uint = 0x0002	/* MMU RO memory */;
pub const PSB_MMU_WO_MEMORY: c_uint = 0x0004	/* MMU WO memory */;
// PTE's and PDE's
pub const PSB_PDE_MASK: c_uint = 0x003FFFFF;
pub const PSB_PDE_SHIFT: c_int = 22;
pub const PSB_PTE_SHIFT: c_int = 12;
// Cache control
pub const PSB_PTE_VALID: c_uint = 0x0001	/* PTE / PDE valid */;
pub const PSB_PTE_WO: c_uint = 0x0002	/* Write only */;
pub const PSB_PTE_RO: c_uint = 0x0004	/* Read only */;
pub const PSB_PTE_CACHED: c_uint = 0x0008	/* CPU cache coherent */;
// VDC registers and bits
pub const PSB_MSVDX_CLOCKGATING: c_uint = 0x2064;
pub const PSB_TOPAZ_CLOCKGATING: c_uint = 0x2068;
pub const PSB_HWSTAM: c_uint = 0x2098;
pub const PSB_INSTPM: c_uint = 0x20C0;
pub const PSB_INT_IDENTITY_R: c_uint = 0x20A4;

pub const PSB_INT_IDENTITY_R: c_uint = 0x20A4;
pub const PSB_INT_MASK_R: c_uint = 0x20A8;
pub const PSB_INT_ENABLE_R: c_uint = 0x20A0;
pub const _PSB_MMU_ER_MASK: c_uint = 0x0001FF00;

pub const GPIOA: c_uint = 0x5010;
pub const GPIOB: c_uint = 0x5014;
pub const GPIOC: c_uint = 0x5018;
pub const GPIOD: c_uint = 0x501c;
pub const GPIOE: c_uint = 0x5020;
pub const GPIOF: c_uint = 0x5024;
pub const GPIOG: c_uint = 0x5028;
pub const GPIOH: c_uint = 0x502c;

pub const VCLK_DIVISOR_VGA0: c_uint = 0x6000;
pub const VCLK_DIVISOR_VGA1: c_uint = 0x6004;
pub const VCLK_POST_DIV: c_uint = 0x6010;

pub const PSB_UIRQ_VISTEST: c_int = 1;
pub const PSB_UIRQ_OOM_REPLY: c_int = 2;
pub const PSB_UIRQ_FIRE_TA_REPLY: c_int = 3;
pub const PSB_UIRQ_FIRE_RASTER_REPLY: c_int = 4;

pub const PSB_MAX_RELOC_PAGES: c_int = 1024;
pub const PSB_LOW_REG_OFFS: c_uint = 0x0204;
pub const PSB_HIGH_REG_OFFS: c_uint = 0x0600;
pub const PSB_NUM_VBLANKS: c_int = 2;

pub const PSB_MAX_BRIGHTNESS: c_int = 100;
pub const PSB_PWR_STATE_ON: c_int = 1;
pub const PSB_PWR_STATE_OFF: c_int = 2;
pub const PSB_PMPOLICY_NOPM: c_int = 0;
pub const PSB_PMPOLICY_CLOCKGATING: c_int = 1;
pub const PSB_PMPOLICY_POWERDOWN: c_int = 2;
pub const PSB_PMSTATE_POWERUP: c_int = 0;
pub const PSB_PMSTATE_CLOCKGATED: c_int = 1;
pub const PSB_PMSTATE_POWERDOWN: c_int = 2;
pub const PSB_PCIx_MSI_ADDR_LOC: c_uint = 0x94;
pub const PSB_PCIx_MSI_DATA_LOC: c_uint = 0x98;
// Medfield crystal settings
pub const KSEL_CRYSTAL_19: c_int = 1;
pub const KSEL_BYPASS_19: c_int = 5;
pub const KSEL_BYPASS_25: c_int = 6;
pub const KSEL_BYPASS_83_100: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psb_intel_opregion {
    pub header: *mut opregion_header,
    pub acpi: *mut opregion_acpi,
    pub swsci: *mut opregion_swsci,
    pub asle: *mut opregion_asle,
    pub vbt: *mut c_void,
    pub lid_state: *mut u32 __iomem,
    pub asle_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdvo_device_mapping {
    pub initialized: u8,
    pub dvo_port: u8,
    pub target_addr: u8,
    pub dvo_wiring: u8,
    pub i2c_pin: u8,
    pub i2c_speed: u8,
    pub ddc_pin: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gmbus {
    pub adapter: i2c_adapter,
    pub force_bit: *mut i2c_adapter,
    pub reg0: u32,
}

// Register offset maps
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psb_offset {
    pub fp0: u32,
    pub fp1: u32,
    pub cntr: u32,
    pub conf: u32,
    pub src: u32,
    pub dpll: u32,
    pub dpll_md: u32,
    pub htotal: u32,
    pub hblank: u32,
    pub hsync: u32,
    pub vtotal: u32,
    pub vblank: u32,
    pub vsync: u32,
    pub stride: u32,
    pub size: u32,
    pub pos: u32,
    pub surf: u32,
    pub addr: u32,
    pub base: u32,
    pub status: u32,
    pub linoff: u32,
    pub tileoff: u32,
    pub palette: u32,
}

//
// Register save state. This is used to hold the context when the
// device is powered off. In the case of Oaktrail this can (but does not
// yet) include screen blank. Operations occuring during the save
// update the register cache instead.
//
// Common status for pipes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psb_pipe {
    pub fp0: u32,
    pub fp1: u32,
    pub cntr: u32,
    pub conf: u32,
    pub src: u32,
    pub dpll: u32,
    pub dpll_md: u32,
    pub htotal: u32,
    pub hblank: u32,
    pub hsync: u32,
    pub vtotal: u32,
    pub vblank: u32,
    pub vsync: u32,
    pub stride: u32,
    pub size: u32,
    pub pos: u32,
    pub base: u32,
    pub surf: u32,
    pub addr: u32,
    pub status: u32,
    pub linoff: u32,
    pub tileoff: u32,
    pub palette: [u32; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psb_state {
    pub saveVCLK_DIVISOR_VGA0: u32,
    pub saveVCLK_DIVISOR_VGA1: u32,
    pub saveVCLK_POST_DIV: u32,
    pub saveVGACNTRL: u32,
    pub saveADPA: u32,
    pub saveLVDS: u32,
    pub saveDVOA: u32,
    pub saveDVOB: u32,
    pub saveDVOC: u32,
    pub savePP_ON: u32,
    pub savePP_OFF: u32,
    pub savePP_CONTROL: u32,
    pub savePP_CYCLE: u32,
    pub savePFIT_CONTROL: u32,
    pub saveCLOCKGATING: u32,
    pub saveDSPARB: u32,
    pub savePFIT_AUTO_RATIOS: u32,
    pub savePFIT_PGM_RATIOS: u32,
    pub savePP_ON_DELAYS: u32,
    pub savePP_OFF_DELAYS: u32,
    pub savePP_DIVISOR: u32,
    pub saveBCLRPAT_A: u32,
    pub saveBCLRPAT_B: u32,
    pub savePERF_MODE: u32,
    pub saveDSPFW1: u32,
    pub saveDSPFW2: u32,
    pub saveDSPFW3: u32,
    pub saveDSPFW4: u32,
    pub saveDSPFW5: u32,
    pub saveDSPFW6: u32,
    pub saveCHICKENBIT: u32,
    pub saveDSPACURSOR_CTRL: u32,
    pub saveDSPBCURSOR_CTRL: u32,
    pub saveDSPACURSOR_BASE: u32,
    pub saveDSPBCURSOR_BASE: u32,
    pub saveDSPACURSOR_POS: u32,
    pub saveDSPBCURSOR_POS: u32,
    pub saveOV_OVADD: u32,
    pub saveOV_OGAMC0: u32,
    pub saveOV_OGAMC1: u32,
    pub saveOV_OGAMC2: u32,
    pub saveOV_OGAMC3: u32,
    pub saveOV_OGAMC4: u32,
    pub saveOV_OGAMC5: u32,
    pub saveOVC_OVADD: u32,
    pub saveOVC_OGAMC0: u32,
    pub saveOVC_OGAMC1: u32,
    pub saveOVC_OGAMC2: u32,
    pub saveOVC_OGAMC3: u32,
    pub saveOVC_OGAMC4: u32,
    pub saveOVC_OGAMC5: u32,
// DPST register save
    pub saveHISTOGRAM_INT_CONTROL_REG: u32,
    pub saveHISTOGRAM_LOGIC_CONTROL_REG: u32,
    pub savePWM_CONTROL_LOGIC: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdv_state {
    pub saveDSPCLK_GATE_D: u32,
    pub saveRAMCLK_GATE_D: u32,
    pub saveDSPARB: u32,
    pub saveDSPFW: [u32; 6],
    pub saveADPA: u32,
    pub savePP_CONTROL: u32,
    pub savePFIT_PGM_RATIOS: u32,
    pub saveLVDS: u32,
    pub savePFIT_CONTROL: u32,
    pub savePP_ON_DELAYS: u32,
    pub savePP_OFF_DELAYS: u32,
    pub savePP_CYCLE: u32,
    pub saveVGACNTRL: u32,
    pub saveIER: u32,
    pub saveIMR: u32,
    pub saveLBB: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psb_save_area {
    pub pipe: [psb_pipe; 3],
    pub saveBSM: u32,
    pub saveVBT: u32,
    pub psb: psb_state,
    pub cdv: cdv_state,
}

pub const PSB_NUM_PIPE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_psb_private {
    pub dev: drm_device,
    pub /: *mut *mut *mut pci_dev aux_pdev; / Currently only used by mrst,
    pub /: *mut *mut *mut pci_dev lpc_pdev; / Currently only used by mrst,
    pub ops: *const psb_ops,
    pub regmap: *const psb_offset,
    pub child_dev: *mut child_device_config,
    pub child_dev_num: c_int,
    pub gtt: psb_gtt,
// GTT Memory manager
    pub gtt_mm: *mut psb_gtt_mm,
    pub scratch_page: *mut page,
    pub gtt_map: *mut u32 __iomem,
    pub stolen_base: u32,
    pub vram_addr: *mut u8 __iomem,
    pub vram_stolen_size: c_ulong,
    pub /: *mut *mut u16 gmch_ctrl; / Saved GTT setup,
    pub pge_ctl: u32,
    pub gtt_mutex: mutex,
    pub /: *mut *mut *mut resource gtt_mem; / Our PCI resource,
    pub mmap_mutex: mutex,
    pub mmu: *mut psb_mmu_driver,
    pub pf_pd: *mut psb_mmu_pd,
// Register base
    pub sgx_reg: *mut uint8_t __iomem,
    pub vdc_reg: *mut uint8_t __iomem,
    pub /: *mut *mut *mut uint8_t __iomem aux_reg; / Auxillary vdc pipe regs,
    pub lpc_gpio_base: u16,
    pub gatt_free_offset: u32,
// Fencing / irq
    pub vdc_irq_mask: u32,
    pub pipestat: [u32; PSB_NUM_PIPE],
    pub irqmask_lock: spinlock_t,
    pub irq_enabled: bool,
// Power
    pub pm_initialized: bool,
// Modesetting
    pub mode_dev: psb_intel_mode_device,
    pub /: *mut *mut bool modeset; / true if we have done the mode_device setup,
    pub plane_to_crtc_mapping: [*mut drm_crtc; PSB_NUM_PIPE],
    pub pipe_to_crtc_mapping: [*mut drm_crtc; PSB_NUM_PIPE],
    pub num_pipe: u32,
// OSPM info (Power management base) (TODO: can go ?)
    pub ospm_base: u32,
// Sizes info
    pub fuse_reg_value: u32,
    pub video_device_fuse: u32,
// PCI revision ID for B0:D2:F0
    pub platform_rev_id: u8,
// gmbus
    pub gmbus: *mut intel_gmbus,
    pub gmbus_reg: *mut uint8_t __iomem,
// Used by SDVO
    pub crt_ddc_pin: c_int,
// FIXME: The mappings should be parsed from bios but for now we can
    pub sdvo_mappings: [sdvo_device_mapping; 2],
    pub hotplug_supported_mask: u32,
    pub broadcast_rgb_property: *mut drm_property,
    pub force_audio_property: *mut drm_property,
// LVDS info
    pub /: *mut *mut int backlight_duty_cycle; / restore backlight to this value,
    pub panel_wants_dither: bool,
    pub panel_fixed_mode: *mut drm_display_mode,
    pub lfp_lvds_vbt_mode: *mut drm_display_mode,
    pub sdvo_lvds_vbt_mode: *mut drm_display_mode,
    pub /: *mut *mut *mut bdb_lvds_backlight lvds_bl; / LVDS backlight info from VBT,
    pub /: *mut *mut *mut gma_i2c_chan lvds_i2c_bus; / FIXME: Remove this?,
// Feature bits from the VBIOS
    pub int_tv_support:1: c_uint,
    pub lvds_dither:1: c_uint,
    pub lvds_vbt:1: c_uint,
    pub int_crt_support:1: c_uint,
    pub lvds_use_ssc:1: c_uint,
    pub lvds_ssc_freq: c_int,
    pub is_lvds_on: bool,
    pub is_mipi_on: bool,
    pub lvds_enabled_in_vbt: bool,
    pub mipi_ctrl_display: u32,
    pub core_freq: c_uint,
    pub iLVDS_enable: u32,
// MID specific
    pub use_msi: bool,
    pub has_gct: bool,
    pub gct_data: oaktrail_gct_data,
// Oaktrail HDMI state
    pub hdmi_priv: *mut oaktrail_hdmi_dev,
// Register state
    pub regs: psb_save_area,
// Hotplug handling
    pub hotplug_work: work_struct,
    pub opregion: psb_intel_opregion,
// Watchdog
    pub apm_reg: u32,
    pub apm_base: u16,
//
// Used for modifying backlight from
// xrandr -- consider removing and using HAL instead
//
    pub scu: *mut intel_scu_ipc_dev,
    pub backlight_device: *mut backlight_device,
    pub backlight_property: *mut drm_property,
    pub backlight_enabled: bool,
    pub backlight_level: c_int,
    pub blc_adj1: u32,
    pub blc_adj2: u32,
    pub dsr_enable: bool,
    pub dsr_fb_update: u32,
    pub dpi_panel_on: [bool; 3],
    pub dsi_configs: [*mut c_void; 2],
    pub bpp: u32,
    pub bpp2: u32,
    pub pipeconf: [u32; 3],
    pub dspcntr: [u32; 3],
    pub /: *mut *mut bool dplla_96mhz; / DPLL data from the VBT,
    pub rate: c_int,
    pub lanes: c_int,
    pub preemphasis: c_int,
    pub vswing: c_int,
    pub initialized: bool,
    pub support: bool,
    pub bpp: c_int,
    pub pps: edp_power_seq,
    pub edp: },
    pub panel_type: u8,
}

extern "C" {
    pub fn container_of(_arg: dev, drm_psb_private: struct, _arg: dev) -> return;
}
// Operations for each board type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psb_ops {
    pub name: *const c_char,
    pub /: *mut *mut int pipes; / Number of output pipes,
    pub /: *mut *mut int crtcs; / Number of CRTCs,
    pub /: *mut *mut int sgx_offset; / Base offset of SGX device,
    pub /: *mut *mut int hdmi_mask; / Mask of HDMI CRTCs,
    pub /: *mut *mut int lvds_mask; / Mask of LVDS CRTCs,
    pub /: *mut *mut int sdvo_mask; / Mask of SDVO CRTCs,
    pub /: *mut *mut int cursor_needs_phys; / If cursor base reg need physical address,
// Sub functions
    pub crtc_helper: *const drm_crtc_helper_funcs,
    pub clock_funcs: *const gma_clock_funcs,
// Setup hooks
    pub dev): *mut *mut int (chip_setup)(struct drm_device,
    pub dev): *mut *mut void (chip_teardown)(struct drm_device,
// Optional helper caller after modeset
    pub dev): *mut *mut void (errata)(struct drm_device,
// Display management hooks
    pub dev): *mut *mut int (output_init)(struct drm_device,
    pub dev): *mut *mut int (hotplug)(struct drm_device,
    pub on): *mut *mut *mut void (hotplug_enable)(struct drm_device dev, bool,
// Power management hooks
    pub dev): *mut *mut void (init_pm)(struct drm_device,
    pub dev): *mut *mut int (save_regs)(struct drm_device,
    pub dev): *mut *mut int (restore_regs)(struct drm_device,
    pub crtc): *mut *mut void (save_crtc)(struct drm_crtc,
    pub crtc): *mut *mut void (restore_crtc)(struct drm_crtc,
    pub dev): *mut *mut int (power_up)(struct drm_device,
    pub dev): *mut *mut int (power_down)(struct drm_device,
    pub crtc): *mut *mut *mut void (update_wm)(struct drm_device dev, struct drm_crtc,
    pub dev): *mut *mut void (disable_sr)(struct drm_device,
    pub on): *mut *mut *mut void (lvds_bl_power)(struct drm_device dev, bool,
// Backlight
    pub dev): *mut *mut int (backlight_init)(struct drm_device,
    pub level): *mut *mut *mut void (backlight_set)(struct drm_device dev, int,
    pub dev): *mut *mut int (backlight_get)(struct drm_device,
    pub backlight_name: *const c_char,
    pub /: *mut *mut int i2c_bus; / I2C bus identifier for Moorestown,
}

// modesetting
extern "C" {
    pub fn psb_modeset_init(dev: *mut drm_device);
}
extern "C" {
    pub fn psb_modeset_cleanup(dev: *mut drm_device);
}
// fbdev

// backlight.c
extern "C" {
    pub fn gma_backlight_init(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn gma_backlight_exit(dev: *mut drm_device);
}
extern "C" {
    pub fn gma_backlight_disable(dev: *mut drm_device);
}
extern "C" {
    pub fn gma_backlight_enable(dev: *mut drm_device);
}
extern "C" {
    pub fn gma_backlight_set(dev: *mut drm_device, v: c_int);
}
// oaktrail_crtc.c
// oaktrail_lvds.c
// psb_intel_display.c
// psb_intel_lvds.c
// gem.c
// psb_device.c
// oaktrail_device.c
// cdv_device.c
// Utilities
extern "C" {
    pub fn ioread32(reg: dev_priv->vdc_reg +) -> return;
}
extern "C" {
    pub fn ioread32(reg: dev_priv->aux_reg +) -> return;
}

// Useful for post reads

