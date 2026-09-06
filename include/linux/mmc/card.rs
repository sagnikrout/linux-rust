//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mmc/card.h
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
// linux/include/linux/mmc/card.h
//
// Card driver specific definitions.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_cid {
    pub manfid: c_uint,
    pub prod_name: [c_char; 8],
    pub prv: c_uchar,
    pub serial: c_uint,
    pub oemid: c_ushort,
    pub year: c_ushort,
    pub hwrev: c_uchar,
    pub fwrev: c_uchar,
    pub month: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_csd {
    pub structure: c_uchar,
    pub mmca_vsn: c_uchar,
    pub cmdclass: c_ushort,
    pub taac_clks: c_ushort,
    pub taac_ns: c_uint,
    pub c_size: c_uint,
    pub r2w_factor: c_uint,
    pub max_dtr: c_uint,
    pub /: *mut *mut unsigned int erase_size; / In sectors,
    pub wp_grp_size: c_uint,
    pub read_blkbits: c_uint,
    pub write_blkbits: c_uint,
    pub capacity: sector_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_ext_csd {
    pub rev: u8,
    pub erase_group_def: u8,
    pub sec_feature_support: u8,
    pub rel_sectors: u8,
    pub rel_param: u8,
    pub enhanced_rpmb_supported: bool,
    pub part_config: u8,
    pub cache_ctrl: u8,
    pub rst_n_function: u8,
    pub /: *mut *mut unsigned int part_time; / Units: ms,
    pub /: *mut *mut unsigned int sa_timeout; / Units: 100ns,
    pub /: *mut *mut unsigned int generic_cmd6_time; / Units: 10ms,
    pub /: *mut *mut unsigned int power_off_longtime; / Units: ms,
    pub /: *mut *mut u8 power_off_notification; / state,
    pub hs_max_dtr: c_uint,
    pub hs200_max_dtr: c_uint,
pub const MMC_HIGH_26_MAX_DTR: c_int = 26000000;
pub const MMC_HIGH_52_MAX_DTR: c_int = 52000000;
pub const MMC_HIGH_DDR_MAX_DTR: c_int = 52000000;
pub const MMC_HS200_MAX_DTR: c_int = 200000000;
    pub sectors: c_uint,
    pub /: *mut *mut unsigned int hc_erase_size; / In sectors,
    pub /: *mut *mut unsigned int hc_erase_timeout; / In milliseconds,
    pub /: *mut *mut unsigned int sec_trim_mult; / Secure trim multiplier,
    pub /: *mut *mut unsigned int sec_erase_mult; / Secure erase multiplier,
    pub /: *mut *mut unsigned int trim_timeout; / In milliseconds,
    pub /: *mut *mut bool partition_setting_completed; / enable bit,
    pub /: *mut *mut unsigned long long enhanced_area_offset; / Units: Byte,
    pub /: *mut *mut unsigned int enhanced_area_size; / Units: KB,
    pub /: *mut *mut unsigned int cache_size; / Units: KB,
    pub /: *mut *mut bool hpi_en; / HPI enablebit,
    pub /: *mut *mut bool hpi; / HPI support bit,
    pub /: *mut *mut unsigned int hpi_cmd; / cmd used as HPI,
    pub /: *mut *mut bool bkops; / background support bit,
    pub /: *mut *mut bool man_bkops_en; / manual bkops enable bit,
    pub /: *mut *mut bool auto_bkops_en; / auto bkops enable bit,
    pub /: *mut *mut unsigned int data_sector_size; / 512 bytes or 4KB,
    pub /: *mut *mut unsigned int data_tag_unit_size; / DATA TAG UNIT size,
    pub /: *mut *mut unsigned int boot_ro_lock; / ro lock support,
    pub boot_ro_lockable: bool,
    pub /: *mut *mut bool ffu_capable; / Firmware upgrade support,
    pub /: *mut *mut bool cmdq_en; / Command Queue enabled,
    pub /: *mut *mut bool cmdq_support; / Command Queue supported,
    pub /: *mut *mut unsigned int cmdq_depth; / Command Queue depth,
pub const MMC_FIRMWARE_LEN: c_int = 8;
    pub /: *mut *mut u8 fwrev[MMC_FIRMWARE_LEN]; / FW version,
    pub /: *mut *mut u8 raw_exception_status; / 54,
    pub /: *mut *mut u8 raw_partition_support; / 160,
    pub /: *mut *mut u8 raw_rpmb_size_mult; / 168,
    pub /: *mut *mut u8 raw_erased_mem_count; / 181,
    pub /: *mut *mut u8 strobe_support; / 184,
    pub /: *mut *mut u8 raw_ext_csd_structure; / 194,
    pub /: *mut *mut u8 raw_card_type; / 196,
    pub /: *mut *mut u8 raw_driver_strength; / 197,
    pub /: *mut *mut u8 out_of_int_time; / 198,
    pub /: *mut *mut u8 raw_pwr_cl_52_195; / 200,
    pub /: *mut *mut u8 raw_pwr_cl_26_195; / 201,
    pub /: *mut *mut u8 raw_pwr_cl_52_360; / 202,
    pub /: *mut *mut u8 raw_pwr_cl_26_360; / 203,
    pub /: *mut *mut u8 raw_s_a_timeout; / 217,
    pub /: *mut *mut u8 raw_hc_erase_gap_size; / 221,
    pub /: *mut *mut u8 raw_erase_timeout_mult; / 223,
    pub /: *mut *mut u8 raw_hc_erase_grp_size; / 224,
    pub /: *mut *mut u8 raw_boot_mult; / 226,
    pub /: *mut *mut u8 raw_sec_trim_mult; / 229,
    pub /: *mut *mut u8 raw_sec_erase_mult; / 230,
    pub /: *mut *mut u8 raw_sec_feature_support;/ 231,
    pub /: *mut *mut u8 raw_trim_mult; / 232,
    pub /: *mut *mut u8 raw_pwr_cl_200_195; / 236,
    pub /: *mut *mut u8 raw_pwr_cl_200_360; / 237,
    pub /: *mut *mut u8 raw_pwr_cl_ddr_52_195; / 238,
    pub /: *mut *mut u8 raw_pwr_cl_ddr_52_360; / 239,
    pub /: *mut *mut u8 raw_pwr_cl_ddr_200_360; / 253,
    pub /: *mut *mut u8 raw_bkops_status; / 246,
    pub /: *mut *mut u8 raw_sectors[4]; / 212 - 4 bytes,
    pub /: *mut *mut u8 pre_eol_info; / 267,
    pub /: *mut *mut u8 device_life_time_est_typ_a; / 268,
    pub /: *mut *mut u8 device_life_time_est_typ_b; / 269,
    pub feature_support: c_uint,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd_scr {
    pub sda_vsn: c_uchar,
    pub sda_spec3: c_uchar,
    pub sda_spec4: c_uchar,
    pub sda_specx: c_uchar,
    pub bus_widths: c_uchar,

    pub cmds: c_uchar,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd_ssr {
    pub /: *mut *mut unsigned int au; / In sectors,
    pub /: *mut *mut unsigned int erase_timeout; / In milliseconds,
    pub /: *mut *mut unsigned int erase_offset; / In milliseconds,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd_switch_caps {
    pub hs_max_dtr: c_uint,
    pub uhs_max_dtr: c_uint,
pub const HIGH_SPEED_MAX_DTR: c_int = 50000000;
pub const UHS_SDR104_MAX_DTR: c_int = 208000000;
pub const UHS_SDR50_MAX_DTR: c_int = 100000000;
pub const UHS_DDR50_MAX_DTR: c_int = 50000000;

pub const UHS_SDR12_MAX_DTR: c_int = 25000000;

    pub sd3_bus_mode: c_uint,
pub const UHS_SDR12_BUS_SPEED: c_int = 0;
pub const HIGH_SPEED_BUS_SPEED: c_int = 1;
pub const UHS_SDR25_BUS_SPEED: c_int = 1;
pub const UHS_SDR50_BUS_SPEED: c_int = 2;
pub const UHS_SDR104_BUS_SPEED: c_int = 3;
pub const UHS_DDR50_BUS_SPEED: c_int = 4;

    pub sd3_drv_type: c_uint,
pub const SD_DRIVER_TYPE_B: c_uint = 0x01;
pub const SD_DRIVER_TYPE_A: c_uint = 0x02;
pub const SD_DRIVER_TYPE_C: c_uint = 0x04;
pub const SD_DRIVER_TYPE_D: c_uint = 0x08;
    pub sd3_curr_limit: c_uint,
pub const SD_SET_CURRENT_LIMIT_200: c_int = 0;
pub const SD_SET_CURRENT_LIMIT_400: c_int = 1;
pub const SD_SET_CURRENT_LIMIT_600: c_int = 2;
pub const SD_SET_CURRENT_LIMIT_800: c_int = 3;

pub const SD4_SET_POWER_LIMIT_0_72W: c_int = 0;
pub const SD4_SET_POWER_LIMIT_1_44W: c_int = 1;
pub const SD4_SET_POWER_LIMIT_2_16W: c_int = 2;
pub const SD4_SET_POWER_LIMIT_2_88W: c_int = 3;
pub const SD4_SET_POWER_LIMIT_1_80W: c_int = 4;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd_ext_reg {
    pub fno: u8,
    pub page: u8,
    pub offset: u16,
    pub rev: u8,
    pub feature_enabled: u8,
    pub feature_support: u8,
// Power Management Function.

// Performance Enhancement Function.

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd_uhs2_config {
    pub node_id: u32,
    pub n_fcu: u32,
    pub maxblk_len: u32,
    pub n_lanes: u8,
    pub dadr_len: u8,
    pub app_type: u8,
    pub phy_minor_rev: u8,
    pub phy_major_rev: u8,
    pub can_hibernate: u8,
    pub n_lss_sync: u8,
    pub n_lss_dir: u8,
    pub link_minor_rev: u8,
    pub link_major_rev: u8,
    pub dev_type: u8,
    pub n_data_gap: u8,
    pub n_fcu_set: u32,
    pub maxblk_len_set: u32,
    pub n_lanes_set: u8,
    pub speed_range_set: u8,
    pub n_lss_sync_set: u8,
    pub n_lss_dir_set: u8,
    pub n_data_gap_set: u8,
    pub max_retry_set: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdio_cccr {
    pub sdio_vsn: c_uint,
    pub sd_vsn: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdio_cis {
    pub vendor: c_ushort,
    pub device: c_ushort,
    pub blksize: c_ushort,
    pub max_dtr: c_uint,
}

pub const SDIO_MAX_FUNCS: c_int = 7;
// The number of MMC physical partitions.  These consist of:
// boot partitions (2), general purpose partitions (4) and
// RPMB partition (1) in MMC v4.4.
//
pub const MMC_NUM_BOOT_PARTITION: c_int = 2;
pub const MMC_NUM_GP_PARTITION: c_int = 4;
pub const MMC_NUM_PHY_PARTITION: c_int = 7;
pub const MAX_MMC_PART_NAME_LEN: c_int = 20;
//
// MMC Physical partitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_part {
    pub /: *mut *mut u64 size; / partition size (in bytes),
    pub /: *mut *mut unsigned int part_cfg; / partition type,
    pub name: [c_char; MAX_MMC_PART_NAME_LEN],
    pub /: *mut *mut bool force_ro; / to make boot parts RO by default,
    pub area_type: c_uint,

}

//
// MMC device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_card {
    pub /: *mut *mut *mut mmc_host host; / the host this device belongs to,
    pub /: *mut *mut device dev; / the device,
    pub /: *mut *mut u32 ocr; / the current OCR setting,
    pub /: *mut *mut unsigned int rca; / relative card address of device,
    pub /: *mut *mut unsigned int type; / card type,

    pub /: *mut *mut unsigned int state; / (our) card state,
    pub /: *mut *mut unsigned int quirks; / card quirks,
    pub /: *mut *mut unsigned int quirk_max_rate; / max rate set by quirks,

// for byte mode

// (missing CIA registers)

// byte mode

    pub /: *mut *mut bool written_flag; / Indicates eMMC has been written since power on,
    pub /: *mut *mut bool reenable_cmdq; / Re-enable Command Queue,
    pub /: *mut *mut unsigned int erase_size; / erase size in sectors,
    pub /: *mut *mut unsigned int erase_shift; / if erase unit is power 2,
    pub /: *mut *mut unsigned int pref_erase; / in sectors,
    pub /: *mut *mut unsigned int eg_boundary; / don't cross erase-group boundaries,
    pub /: *mut *mut unsigned int erase_arg; / erase / trim / discard,
    pub /: *mut *mut u8 erased_byte; / value of erased bytes,
    pub /: *mut *mut unsigned int wp_grp_size; / write group size in sectors,
    pub /: *mut *mut u32 raw_cid[4]; / raw card CID,
    pub /: *mut *mut u32 raw_csd[4]; / raw card CSD,
    pub /: *mut *mut u32 raw_scr[2]; / raw card SCR,
    pub /: *mut *mut u32 raw_ssr[16]; / raw card SSR,
    pub /: *mut *mut mmc_cid cid; / card identification,
    pub /: *mut *mut mmc_csd csd; / card specific,
    pub /: *mut *mut mmc_ext_csd ext_csd; / mmc v4 extended card specific,
    pub /: *mut *mut sd_scr scr; / extra SD information,
    pub /: *mut *mut sd_ssr ssr; / yet more SD information,
    pub /: *mut *mut sd_switch_caps sw_caps; / switch (CMD6) caps,
    pub /: *mut *mut sd_ext_reg ext_power; / SD extension reg for PM,
    pub /: *mut *mut sd_ext_reg ext_perf; / SD extension reg for PERF,
    pub /: *mut *mut sd_uhs2_config uhs2_config; / SD UHS-II config,
    pub /: *mut *mut unsigned int sdio_funcs; / number of SDIO functions,
    pub /: *mut *mut atomic_t sdio_funcs_probed; / number of probed SDIO funcs,
    pub /: *mut *mut sdio_cccr cccr; / common card info,
    pub /: *mut *mut sdio_cis cis; / common tuple info,
    pub /: *mut *mut *mut sdio_func sdio_func[SDIO_MAX_FUNCS]; / SDIO functions (devices),
    pub /: *mut *mut *mut sdio_func sdio_single_irq; / SDIO function when only one IRQ active,
    pub /: *mut *mut u8 major_rev; / major revision number,
    pub /: *mut *mut u8 minor_rev; / minor revision number,
    pub /: *mut *mut unsigned num_info; / number of info strings,
    pub /: *const *const *const *const char info; / info strings,
    pub /: *mut *mut *mut sdio_func_tuple tuples; / unknown common tuples,
    pub /: *mut *mut unsigned int sd_bus_speed; / Bus Speed Mode set for the card,
    pub /: *mut *mut unsigned int mmc_avail_type; / supported device type by both host and card,
    pub /: *mut *mut unsigned int drive_strength; / for UHS-I, HS200 or HS400,
    pub debugfs_root: *mut dentry,
    pub /: *mut *mut mmc_part part[MMC_NUM_PHY_PARTITION]; / physical partitions,
    pub nr_parts: c_uint,
    pub /: *mut *mut *mut workqueue_complete_wq; / Private workqueue,
}

extern "C" {
    pub fn mmc_card_is_blockaddr(card: *mut mmc_card) -> bool;
}

