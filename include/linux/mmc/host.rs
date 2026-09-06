//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mmc/host.h
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
// linux/include/linux/mmc/host.h
//
// Host driver specific definitions.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_ios {
    pub /: *mut *mut unsigned int clock; / clock rate,
    pub vdd: c_ushort,
    pub /: *mut *mut unsigned int power_delay_ms; / waiting for stable power,
// vdd stores the bit number of the selected voltage range from below.
    pub /: *mut *mut unsigned char bus_mode; / command output mode,
pub const MMC_BUSMODE_OPENDRAIN: c_int = 1;
pub const MMC_BUSMODE_PUSHPULL: c_int = 2;
    pub /: *mut *mut unsigned char chip_select; / SPI chip select,
pub const MMC_CS_DONTCARE: c_int = 0;
pub const MMC_CS_HIGH: c_int = 1;
pub const MMC_CS_LOW: c_int = 2;
    pub /: *mut *mut unsigned char power_mode; / power supply mode,
pub const MMC_POWER_OFF: c_int = 0;
pub const MMC_POWER_UP: c_int = 1;
pub const MMC_POWER_ON: c_int = 2;
pub const MMC_POWER_UNDEFINED: c_int = 3;
    pub /: *mut *mut unsigned char bus_width; / data bus width,
pub const MMC_BUS_WIDTH_1: c_int = 0;
pub const MMC_BUS_WIDTH_4: c_int = 2;
pub const MMC_BUS_WIDTH_8: c_int = 3;
    pub /: *mut *mut unsigned char timing; / timing specification used,
pub const MMC_TIMING_LEGACY: c_int = 0;
pub const MMC_TIMING_MMC_HS: c_int = 1;
pub const MMC_TIMING_SD_HS: c_int = 2;
pub const MMC_TIMING_UHS_SDR12: c_int = 3;
pub const MMC_TIMING_UHS_SDR25: c_int = 4;
pub const MMC_TIMING_UHS_SDR50: c_int = 5;
pub const MMC_TIMING_UHS_SDR104: c_int = 6;
pub const MMC_TIMING_UHS_DDR50: c_int = 7;
pub const MMC_TIMING_MMC_DDR52: c_int = 8;
pub const MMC_TIMING_MMC_HS200: c_int = 9;
pub const MMC_TIMING_MMC_HS400: c_int = 10;
pub const MMC_TIMING_SD_EXP: c_int = 11;
pub const MMC_TIMING_SD_EXP_1_2V: c_int = 12;
pub const MMC_TIMING_UHS2_SPEED_A: c_int = 13;
pub const MMC_TIMING_UHS2_SPEED_A_HD: c_int = 14;
pub const MMC_TIMING_UHS2_SPEED_B: c_int = 15;
pub const MMC_TIMING_UHS2_SPEED_B_HD: c_int = 16;
    pub /: *mut *mut unsigned char signal_voltage; / signalling voltage (1.8V or 3.3V),
pub const MMC_SIGNAL_VOLTAGE_330: c_int = 0;
pub const MMC_SIGNAL_VOLTAGE_180: c_int = 1;
pub const MMC_SIGNAL_VOLTAGE_120: c_int = 2;
    pub vqmmc2_voltage: c_uchar,
pub const MMC_VQMMC2_VOLTAGE_180: c_int = 0;
    pub /: *mut *mut unsigned char drv_type; / driver type (A, B, C, D),
pub const MMC_SET_DRIVER_TYPE_B: c_int = 0;
pub const MMC_SET_DRIVER_TYPE_A: c_int = 1;
pub const MMC_SET_DRIVER_TYPE_C: c_int = 2;
pub const MMC_SET_DRIVER_TYPE_D: c_int = 3;
    pub /: *mut *mut bool enhanced_strobe; / hs400es selection,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_clk_phase {
    pub valid: bool,
    pub in_deg: u16,
    pub out_deg: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_clk_phase_map {
    pub phase: [mmc_clk_phase; MMC_NUM_CLK_PHASES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd_uhs2_caps {
    pub dap: u32,
    pub gap: u32,
    pub group_desc: u32,
    pub maxblk_len: u32,
    pub n_fcu: u32,
    pub n_lanes: u8,
    pub addr64: u8,
    pub card_type: u8,
    pub phy_rev: u8,
    pub speed_range: u8,
    pub n_lss_sync: u8,
    pub n_lss_dir: u8,
    pub link_rev: u8,
    pub host_type: u8,
    pub n_data_gap: u8,
    pub maxblk_len_set: u32,
    pub n_fcu_set: u32,
    pub n_lanes_set: u8,
    pub n_lss_sync_set: u8,
    pub n_lss_dir_set: u8,
    pub n_data_gap_set: u8,
    pub max_retry_set: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sd_uhs2_operation {
    UHS2_PHY_INIT = 0,
    UHS2_SET_CONFIG,
    UHS2_ENABLE_INT,
    UHS2_DISABLE_INT,
    UHS2_ENABLE_CLK,
    UHS2_DISABLE_CLK,
    UHS2_CHECK_DORMANT,
    UHS2_SET_IOS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmc_err_stat {
    MMC_ERR_CMD_TIMEOUT,
    MMC_ERR_CMD_CRC,
    MMC_ERR_DAT_TIMEOUT,
    MMC_ERR_DAT_CRC,
    MMC_ERR_AUTO_CMD,
    MMC_ERR_ADMA,
    MMC_ERR_TUNING,
    MMC_ERR_CMDQ_RED,
    MMC_ERR_CMDQ_GCE,
    MMC_ERR_CMDQ_ICCE,
    MMC_ERR_REQ_TIMEOUT,
    MMC_ERR_CMDQ_REQ_TIMEOUT,
    MMC_ERR_ICE_CFG,
    MMC_ERR_CTRL_TIMEOUT,
    MMC_ERR_UNEXPECTED_IRQ,
    MMC_ERR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_host_ops {
//
// It is optional for the host to implement pre_req and post_req in
// order to support double buffering of requests (prepare one
// request while another request is active).
// pre_req() must always be followed by a post_req().
// To undo a call made to pre_req(), call post_req() with
// a nonzero err condition.
//
    pub err): c_int,
    pub req): *mut *mut *mut void (pre_req)(struct mmc_host host, struct mmc_request,
    pub req): *mut *mut *mut void (request)(struct mmc_host host, struct mmc_request,
// Submit one request to host in atomic context.
    pub req): *mut mmc_request,
//
// Avoid calling the next three functions too often or in a "fast
// path", since underlaying controller might implement them in an
// expensive and/or slow way. Also note that these functions might
// sleep, so don't call them in the atomic contexts!
//
// Notes to the set_ios callback:
// ios->clock might be 0. For some controllers, setting 0Hz
// as any other frequency works. However, some controllers
// explicitly need to disable the clock. Otherwise e.g. voltage
// switching might fail because the SDCLK is not really quiet.
//
    pub ios): *mut *mut *mut void (set_ios)(struct mmc_host host, struct mmc_ios,
//
// Return values for the get_ro callback should be:
// 0 for a read/write card
// 1 for a read-only card
// -ENOSYS when not supported (equal to NULL callback)
// or a negative errno value when something bad happened
//
    pub host): *mut *mut int (get_ro)(struct mmc_host,
//
// Return values for the get_cd callback should be:
// 0 for a absent card
// 1 for a present card
// -ENOSYS when not supported (equal to NULL callback)
// or a negative errno value when something bad happened
//
    pub host): *mut *mut int (get_cd)(struct mmc_host,
    pub enable): *mut *mut *mut void (enable_sdio_irq)(struct mmc_host host, int,
// Mandatory callback when using MMC_CAP2_SDIO_IRQ_NOTHREAD.
    pub host): *mut *mut void (ack_sdio_irq)(struct mmc_host,
// optional callback for HC quirks
    pub card): *mut *mut *mut void (init_card)(struct mmc_host host, struct mmc_card,
    pub ios): *mut *mut *mut int (start_signal_voltage_switch)(struct mmc_host host, struct mmc_ios,
// Check if the card is pulling dat[0] low
    pub host): *mut *mut int (card_busy)(struct mmc_host,
// The tuning command opcode value is different for SD and eMMC cards
    pub opcode): *mut *mut *mut int (execute_tuning)(struct mmc_host host, u32,
// Prepare HS400 target operating frequency depending host driver
    pub ios): *mut *mut *mut int (prepare_hs400_tuning)(struct mmc_host host, struct mmc_ios,
// Execute HS400 tuning depending host driver
    pub card): *mut *mut *mut int (execute_hs400_tuning)(struct mmc_host host, struct mmc_card,
// Optional callback to prepare for SD high-speed tuning
    pub card): *mut *mut *mut int (prepare_sd_hs_tuning)(struct mmc_host host, struct mmc_card,
// Optional callback to execute SD high-speed tuning
    pub card): *mut *mut *mut int (execute_sd_hs_tuning)(struct mmc_host host, struct mmc_card,
// Prepare switch to DDR during the HS400 init sequence
    pub host): *mut *mut int (hs400_prepare_ddr)(struct mmc_host,
// Prepare for switching from HS400 to HS200
    pub host): *mut *mut void (hs400_downgrade)(struct mmc_host,
// Complete selection of HS400
    pub host): *mut *mut void (hs400_complete)(struct mmc_host,
// Prepare enhanced strobe depending host driver
    pub ios): *mut mmc_ios,
    pub drv_type): *mut int card_drv, int,
// Reset the eMMC card via RST_n
    pub host): *mut *mut void (card_hw_reset)(struct mmc_host,
    pub host): *mut *mut void (card_event)(struct mmc_host,
//
// Optional callback to support controllers with HW issues for multiple
// I/O. Returns the number of supported blocks for the request.
//
    pub blk_size): unsigned int direction, int,
// Initialize an SD express card, mandatory for MMC_CAP2_SD_EXP.
    pub ios): *mut *mut *mut int (init_sd_express)(struct mmc_host host, struct mmc_ios,
//
// The uhs2_control callback is used to execute SD UHS-II specific
// operations. It's mandatory to implement for hosts that supports the
// SD UHS-II interface (MMC_CAP2_SD_UHS2). Expected return values are a
// negative errno in case of a failure or zero for success.
//
    pub op): *mut *mut *mut int (uhs2_control)(struct mmc_host host, enum sd_uhs2_operation,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_cqe_ops {
// Allocate resources, and make the CQE operational
    pub card): *mut *mut *mut int (cqe_enable)(struct mmc_host host, struct mmc_card,
// Free resources, and make the CQE non-operational
    pub host): *mut *mut void (cqe_disable)(struct mmc_host,
//
// Issue a read, write or DCMD request to the CQE. Also deal with the
// effect of ->cqe_off().
//
    pub mrq): *mut *mut *mut int (cqe_request)(struct mmc_host host, struct mmc_request,
// Free resources (e.g. DMA mapping) associated with the request
    pub mrq): *mut *mut *mut void (cqe_post_req)(struct mmc_host host, struct mmc_request,
//
// Prepare the CQE and host controller to accept non-CQ commands. There
// is no corresponding ->cqe_on(), instead ->cqe_request() is required
// to deal with that.
//
    pub host): *mut *mut void (cqe_off)(struct mmc_host,
//
// Wait for all CQE tasks to complete. Return an error if recovery
// becomes necessary.
//
    pub host): *mut *mut int (cqe_wait_for_idle)(struct mmc_host,
//
// Notify CQE that a request has timed out. Return false if the request
// completed or true if a timeout happened in which case indicate if
// recovery is needed.
//
    pub recovery_needed): *mut bool,
//
// Stop all CQE activity and prepare the CQE and host controller to
// accept recovery commands.
//
    pub host): *mut *mut void (cqe_recovery_start)(struct mmc_host,
//
// Clear the queue and call mmc_cqe_request_done() on all requests.
// Requests that errored will have the error set on the mmc_request
// (data->error or cmd->error for DCMD).  Requests that did not error
// will have zero data bytes transferred.
//
    pub host): *mut *mut void (cqe_recovery_finish)(struct mmc_host,
}

//
// struct mmc_slot - MMC slot functions
//
// @cd_irq:		MMC/SD-card slot hotplug detection IRQ or -EINVAL
// @handler_priv:	MMC/SD-card slot context
//
// Some MMC/SD host controllers implement slot-functions like card and
// write-protect detection natively. However, a large number of controllers
// leave these functions to the CPU. This struct provides a hook to attach
// such slot-function drivers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_slot {
    pub cd_irq: c_int,
    pub cd_wake_enabled: bool,
    pub handler_priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_supply {
    pub /: *mut *mut *mut regulator vmmc; / Card power supply,
    pub /: *mut *mut *mut regulator vqmmc; / Optional Vccq supply,
    pub /: *mut *mut *mut regulator vqmmc2; / Optional supply for phy,
    pub /: *mut *mut notifier_block vmmc_nb; / Notifier for vmmc,
    pub /: *mut *mut work_uv_work; / Undervoltage work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_ctx {
    pub task: *mut task_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_host {
    pub parent: *mut device,
    pub class_dev: device,
    pub index: c_int,
    pub ops: *const mmc_host_ops,
    pub pwrseq: *mut mmc_pwrseq,
    pub f_min: c_uint,
    pub f_max: c_uint,
    pub f_init: c_uint,
    pub ocr_avail: u32,
    pub /: *mut *mut u32 ocr_avail_sdio; / SDIO-specific OCR,
    pub /: *mut *mut u32 ocr_avail_sd; / SD-specific OCR,
    pub /: *mut *mut u32 ocr_avail_mmc; / MMC-specific OCR,
    pub /: *mut *mut *mut wakeup_source ws; / Enable consume of uevents,
    pub max_current_330: u32,
    pub max_current_300: u32,
    pub max_current_180: u32,
pub const MMC_VDD_165_195: c_uint = 0x00000080	/* VDD voltage 1.65 - 1.95 */;
pub const MMC_VDD_20_21: c_uint = 0x00000100	/* VDD voltage 2.0 ~ 2.1 */;
pub const MMC_VDD_21_22: c_uint = 0x00000200	/* VDD voltage 2.1 ~ 2.2 */;
pub const MMC_VDD_22_23: c_uint = 0x00000400	/* VDD voltage 2.2 ~ 2.3 */;
pub const MMC_VDD_23_24: c_uint = 0x00000800	/* VDD voltage 2.3 ~ 2.4 */;
pub const MMC_VDD_24_25: c_uint = 0x00001000	/* VDD voltage 2.4 ~ 2.5 */;
pub const MMC_VDD_25_26: c_uint = 0x00002000	/* VDD voltage 2.5 ~ 2.6 */;
pub const MMC_VDD_26_27: c_uint = 0x00004000	/* VDD voltage 2.6 ~ 2.7 */;
pub const MMC_VDD_27_28: c_uint = 0x00008000	/* VDD voltage 2.7 ~ 2.8 */;
pub const MMC_VDD_28_29: c_uint = 0x00010000	/* VDD voltage 2.8 ~ 2.9 */;
pub const MMC_VDD_29_30: c_uint = 0x00020000	/* VDD voltage 2.9 ~ 3.0 */;
pub const MMC_VDD_30_31: c_uint = 0x00040000	/* VDD voltage 3.0 ~ 3.1 */;
pub const MMC_VDD_31_32: c_uint = 0x00080000	/* VDD voltage 3.1 ~ 3.2 */;
pub const MMC_VDD_32_33: c_uint = 0x00100000	/* VDD voltage 3.2 ~ 3.3 */;
pub const MMC_VDD_33_34: c_uint = 0x00200000	/* VDD voltage 3.3 ~ 3.4 */;
pub const MMC_VDD_34_35: c_uint = 0x00400000	/* VDD voltage 3.4 ~ 3.5 */;
pub const MMC_VDD_35_36: c_uint = 0x00800000	/* VDD voltage 3.5 ~ 3.6 */;
    pub /: *mut *mut u32 caps; / Host capabilities,

    pub /: *mut *mut u32 caps2; / More host capabilities,

pub const MMC_CAP2_CRYPTO: c_int = 0;

    pub /: *mut *mut bool uhs2_sd_tran; / UHS-II flag for SD_TRAN state,
    pub /: *mut *mut bool uhs2_app_cmd; / UHS-II flag for APP command,
    pub /: *mut *mut sd_uhs2_caps uhs2_caps; / Host UHS-II capabilities,
    pub /: *mut *mut int fixed_drv_type; / fixed driver type for non-removable media,
    pub /: *mut *mut mmc_pm_flag_t pm_caps; / supported pm features,
// host specific block data
    pub /: *mut *mut unsigned int max_seg_size; / lim->max_segment_size,
    pub /: *mut *mut unsigned short max_segs; / lim->max_segments,
    pub unused: c_ushort,
    pub /: *mut *mut unsigned int max_req_size; / maximum number of bytes in one req,
    pub /: *mut *mut unsigned int max_blk_size; / maximum size of one mmc block,
    pub /: *mut *mut unsigned int max_blk_count; / maximum number of blocks in one req,
    pub /: *mut *mut unsigned int max_busy_timeout; / max busy timeout in ms,
// private data
    pub /: *mut *mut spinlock_t lock; / lock for claim and bus ops,
    pub /: *mut *mut mmc_ios ios; / current io bus settings,
    pub /: *mut *mut bool claimed; / host exclusively claimed,
// group bitfields together to minimize padding
    pub use_spi_crc:1: c_uint,
    pub /: *mut *mut unsigned int doing_init_tune:1; / initial tuning in progress,
    pub /: *mut *mut unsigned int doing_retune:1; / re-tuning in progress,
    pub /: *mut *mut unsigned int retune_crc_disable:1; / don't trigger retune upon crc,
    pub /: *mut *mut unsigned int can_dma_map_merge:1; / merging can be used,
    pub /: *mut *mut unsigned int vqmmc_enabled:1; / vqmmc regulator is enabled,
//
// Indicates if an undervoltage event has already been handled.
// This prevents repeated regulator notifiers from triggering
// multiple REGULATOR_EVENT_UNDER_VOLTAGE events.
//
    pub /: *mut *mut unsigned int undervoltage:1; / Undervoltage state,
    pub /: *mut *mut int rescan_disable; / disable card detection,
    pub /: *mut *mut int rescan_entered; / used with nonremovable devices,
    pub /: *mut *mut bool can_retune; / re-tuning can be used,
    pub /: *mut *mut bool retune_now; / do re-tuning at next req,
    pub /: *mut *mut bool retune_paused; / re-tuning is temporarily disabled,
    pub /: *mut *mut int need_retune; / re-tuning is needed,
    pub /: *mut *mut int hold_retune; / hold off re-tuning,
    pub /: *mut *mut unsigned int retune_period; / re-tuning period in secs,
    pub /: *mut *mut timer_list retune_timer; / for periodic re-tuning,
    pub /: *mut *mut bool trigger_card_event; / card_event necessary,
    pub /: *mut *mut *mut mmc_card card; / device attached to this host,
    pub wq: wait_queue_head_t,
    pub /: *mut *mut *mut mmc_ctx claimer; / context that has host claimed,
    pub /: *mut *mut int claim_cnt; / "claim" nesting count,
    pub /: *mut *mut mmc_ctx default_ctx; / default context,
    pub detect: delayed_work,
    pub /: *mut *mut int detect_change; / card detect flag,
    pub slot: mmc_slot,
    pub /: *const *const *const mmc_bus_ops bus_ops; / current bus driver,
    pub sdio_irqs: c_uint,
    pub sdio_irq_thread: *mut task_struct,
    pub sdio_irq_work: work_struct,
    pub sdio_irq_pending: bool,
    pub sdio_irq_thread_abort: core::sync::atomic::AtomicI32,
    pub /: *mut *mut mmc_pm_flag_t pm_flags; / requested pm features,
    pub /: *mut *mut *mut led_trigger led; / activity led,

    pub /: *mut *mut bool regulator_enabled; / regulator state,

    pub supply: mmc_supply,
    pub debugfs_root: *mut dentry,
// Ongoing data transfer that allows commands during transfer
    pub ongoing_mrq: *mut mmc_request,

    pub fail_mmc_request: fault_attr,

    pub /: *mut *mut unsigned int actual_clock; / Actual HC clock rate,
    pub /: *mut *mut unsigned int slotno; / used for sdio acpi binding,
    pub /: *mut *mut int dsr_req; / DSR value is valid,
    pub /: *mut *mut u32 dsr; / optional driver stage (DSR) value,
// Command Queue Engine (CQE) support
    pub cqe_ops: *const mmc_cqe_ops,
    pub cqe_private: *mut c_void,
    pub cqe_qdepth: c_int,
    pub cqe_enabled: bool,
    pub cqe_on: bool,
// Inline encryption support

    pub crypto_profile: blk_crypto_profile,

// Host Software Queue support
    pub hsq_enabled: bool,
    pub hsq_depth: c_int,
    pub err_stats: [u32; MMC_ERR_MAX],
    pub max_sd_hs_hz: u32,
    pub ____cacheline_aligned: unsigned long private[],
}

extern "C" {
    pub fn mmc_add_host(: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_remove_host(: *mut mmc_host);
}
extern "C" {
    pub fn mmc_free_host(: *mut mmc_host);
}
extern "C" {
    pub fn mmc_of_parse(host: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_of_parse_voltage(host: *mut mmc_host, mask: *mut u32) -> c_int;
}
extern "C" {
    pub fn container_of(_arg: priv, mmc_host: struct, _arg: private) -> return;
}

extern "C" {
    pub fn container_of(_arg: profile, mmc_host: struct, _arg: crypto_profile) -> return;
}

extern "C" {
    pub fn mmc_detect_change(: *mut mmc_host, delay: c_ulong);
}
extern "C" {
    pub fn mmc_request_done(: *mut mmc_host, : *mut mmc_request);
}
extern "C" {
    pub fn mmc_command_done(host: *mut mmc_host, mrq: *mut mmc_request);
}
extern "C" {
    pub fn mmc_cqe_request_done(host: *mut mmc_host, mrq: *mut mmc_request);
}
//
// May be called from host driver's system/runtime suspend/resume callbacks,
// to know if SDIO IRQs has been claimed.
//
extern "C" {
    pub fn sdio_signal_irq(host: *mut mmc_host);
}

extern "C" {
    pub fn mmc_regulator_set_vqmmc(mmc: *mut mmc_host, ios: *mut mmc_ios) -> c_int;
}
extern "C" {
    pub fn mmc_regulator_set_vqmmc2(mmc: *mut mmc_host, ios: *mut mmc_ios) -> c_int;
}

extern "C" {
    pub fn mmc_regulator_get_supply(mmc: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_regulator_enable_vqmmc(mmc: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_regulator_disable_vqmmc(mmc: *mut mmc_host);
}
// TODO: Move to private header
extern "C" {
    pub fn mmc_retune_timer_stop(host: *mut mmc_host);
}
extern "C" {
    pub fn mmc_send_status(card: *mut mmc_card, status: *mut u32) -> c_int;
}
extern "C" {
    pub fn mmc_send_tuning(host: *mut mmc_host, opcode: u32, cmd_error: *mut c_int) -> c_int;
}
extern "C" {
    pub fn mmc_send_abort_tuning(host: *mut mmc_host, opcode: u32) -> c_int;
}
extern "C" {
    pub fn mmc_get_ext_csd(card: *mut mmc_card, new_ext_csd: *mut u8) -> c_int;
}
extern "C" {
    pub fn mmc_read_tuning(host: *mut mmc_host, blksz: c_uint, blocks: c_uint) -> c_int;
}
