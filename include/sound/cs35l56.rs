//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/cs35l56.h
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
// Common definitions for Cirrus Logic CS35L56 smart amp
//
// Copyright (C) 2023 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

pub const CS35L56_DEVID: c_uint = 0x0000000;
pub const CS35L56_REVID: c_uint = 0x0000004;
pub const CS35L56_RELID: c_uint = 0x000000C;
pub const CS35L56_OTPID: c_uint = 0x0000010;
pub const CS35L56_SFT_RESET: c_uint = 0x0000020;
pub const CS35L56_GLOBAL_ENABLES: c_uint = 0x0002014;
pub const CS35L56_BLOCK_ENABLES: c_uint = 0x0002018;
pub const CS35L56_BLOCK_ENABLES2: c_uint = 0x000201C;
pub const CS35L56_SYNC_GPIO1_CFG: c_uint = 0x0002410;
pub const CS35L56_ASP2_DIO_GPIO13_CFG: c_uint = 0x0002440;
pub const CS35L56_UPDATE_REGS: c_uint = 0x0002A0C;
pub const CS35L56_REFCLK_INPUT: c_uint = 0x0002C04;
pub const CS35L56_GLOBAL_SAMPLE_RATE: c_uint = 0x0002C0C;
pub const CS35L56_ASP1_ENABLES1: c_uint = 0x0004800;
pub const CS35L56_ASP1_CONTROL1: c_uint = 0x0004804;
pub const CS35L56_ASP1_CONTROL2: c_uint = 0x0004808;
pub const CS35L56_ASP1_CONTROL3: c_uint = 0x000480C;
pub const CS35L56_ASP1_FRAME_CONTROL1: c_uint = 0x0004810;
pub const CS35L56_ASP1_FRAME_CONTROL5: c_uint = 0x0004820;
pub const CS35L56_ASP1_DATA_CONTROL1: c_uint = 0x0004830;
pub const CS35L56_ASP1_DATA_CONTROL5: c_uint = 0x0004840;
pub const CS35L56_DACPCM1_INPUT: c_uint = 0x0004C00;
pub const CS35L56_DACPCM2_INPUT: c_uint = 0x0004C08;
pub const CS35L56_ASP1TX1_INPUT: c_uint = 0x0004C20;
pub const CS35L56_ASP1TX2_INPUT: c_uint = 0x0004C24;
pub const CS35L56_ASP1TX3_INPUT: c_uint = 0x0004C28;
pub const CS35L56_ASP1TX4_INPUT: c_uint = 0x0004C2C;
pub const CS35L56_DSP1RX1_INPUT: c_uint = 0x0004C40;
pub const CS35L56_DSP1RX2_INPUT: c_uint = 0x0004C44;
pub const CS35L56_SWIRE_DP3_CH1_INPUT: c_uint = 0x0004C70;
pub const CS35L56_SWIRE_DP3_CH2_INPUT: c_uint = 0x0004C74;
pub const CS35L56_SWIRE_DP3_CH3_INPUT: c_uint = 0x0004C78;
pub const CS35L56_SWIRE_DP3_CH4_INPUT: c_uint = 0x0004C7C;
pub const CS35L56_IRQ1_CFG: c_uint = 0x000E000;
pub const CS35L56_IRQ1_STATUS: c_uint = 0x000E004;
pub const CS35L56_IRQ1_EINT_1: c_uint = 0x000E010;
pub const CS35L56_IRQ1_EINT_2: c_uint = 0x000E014;
pub const CS35L56_IRQ1_EINT_4: c_uint = 0x000E01C;
pub const CS35L56_IRQ1_EINT_8: c_uint = 0x000E02C;
pub const CS35L56_IRQ1_EINT_18: c_uint = 0x000E054;
pub const CS35L56_IRQ1_EINT_20: c_uint = 0x000E05C;
pub const CS35L56_IRQ1_MASK_1: c_uint = 0x000E090;
pub const CS35L56_IRQ1_MASK_2: c_uint = 0x000E094;
pub const CS35L56_IRQ1_MASK_4: c_uint = 0x000E09C;
pub const CS35L56_IRQ1_MASK_8: c_uint = 0x000E0AC;
pub const CS35L56_IRQ1_MASK_18: c_uint = 0x000E0D4;
pub const CS35L56_IRQ1_MASK_20: c_uint = 0x000E0DC;
pub const CS35L56_GPIO_STATUS1: c_uint = 0x000F000;
pub const CS35L56_GPIO1_CTRL1: c_uint = 0x000F008;
pub const CS35L56_GPIO13_CTRL1: c_uint = 0x000F038;
pub const CS35L56_MIXER_NGATE_CH1_CFG: c_uint = 0x0010004;
pub const CS35L56_MIXER_NGATE_CH2_CFG: c_uint = 0x0010008;
pub const CS35L56_DSP_MBOX_1_RAW: c_uint = 0x0011000;
pub const CS35L56_DSP_VIRTUAL1_MBOX_1: c_uint = 0x0011020;
pub const CS35L56_DSP_VIRTUAL1_MBOX_2: c_uint = 0x0011024;
pub const CS35L56_DSP_VIRTUAL1_MBOX_3: c_uint = 0x0011028;
pub const CS35L56_DSP_VIRTUAL1_MBOX_4: c_uint = 0x001102C;
pub const CS35L56_DSP_VIRTUAL1_MBOX_5: c_uint = 0x0011030;
pub const CS35L56_DSP_VIRTUAL1_MBOX_6: c_uint = 0x0011034;
pub const CS35L56_DSP_VIRTUAL1_MBOX_7: c_uint = 0x0011038;
pub const CS35L56_DSP_VIRTUAL1_MBOX_8: c_uint = 0x001103C;
pub const CS35L56_DIE_STS1: c_uint = 0x0017040;
pub const CS35L56_DIE_STS2: c_uint = 0x0017044;
pub const CS35L56_DSP_RESTRICT_STS1: c_uint = 0x00190F0;
pub const CS35L56_OTP_MEM_53: c_uint = 0x00300D4;
pub const CS35L56_OTP_MEM_54: c_uint = 0x00300D8;
pub const CS35L56_OTP_MEM_55: c_uint = 0x00300DC;
pub const CS35L56_DSP1_XMEM_PACKED_0: c_uint = 0x2000000;
pub const CS35L56_DSP1_XMEM_PACKED_6143: c_uint = 0x2005FFC;
pub const CS35L56_DSP1_XMEM_UNPACKED32_0: c_uint = 0x2400000;
pub const CS35L56_DSP1_XMEM_UNPACKED32_4095: c_uint = 0x2403FFC;
pub const CS35L56_DSP1_SYS_INFO_ID: c_uint = 0x25E0000;
pub const CS35L56_DSP1_SYS_INFO_END: c_uint = 0x25E004C;
pub const CS35L56_DSP1_AHBM_WINDOW_DEBUG_0: c_uint = 0x25E2040;
pub const CS35L56_DSP1_AHBM_WINDOW_DEBUG_1: c_uint = 0x25E2044;
pub const CS35L56_DSP1_XMEM_UNPACKED24_0: c_uint = 0x2800000;
pub const CS35L56_DSP1_FW_VER: c_uint = 0x2800010;
pub const CS35L56_DSP1_HALO_STATE: c_uint = 0x28021E0;
pub const CS35L56_B2_DSP1_HALO_STATE: c_uint = 0x2803D20;
pub const CS35L56_DSP1_PM_CUR_STATE: c_uint = 0x2804308;
pub const CS35L56_B2_DSP1_PM_CUR_STATE: c_uint = 0x2804678;
pub const CS35L56_DSP1_XMEM_UNPACKED24_8191: c_uint = 0x2807FFC;
pub const CS35L56_DSP1_CORE_BASE: c_uint = 0x2B80000;
pub const CS35L56_DSP1_SCRATCH1: c_uint = 0x2B805C0;
pub const CS35L56_DSP1_SCRATCH2: c_uint = 0x2B805C8;
pub const CS35L56_DSP1_SCRATCH3: c_uint = 0x2B805D0;
pub const CS35L56_DSP1_SCRATCH4: c_uint = 0x2B805D8;
pub const CS35L56_DSP1_YMEM_PACKED_0: c_uint = 0x2C00000;
pub const CS35L56_DSP1_YMEM_PACKED_4604: c_uint = 0x2C047F0;
pub const CS35L56_DSP1_YMEM_UNPACKED32_0: c_uint = 0x3000000;
pub const CS35L56_DSP1_YMEM_UNPACKED32_3070: c_uint = 0x3002FF8;
pub const CS35L56_DSP1_YMEM_UNPACKED24_0: c_uint = 0x3400000;
pub const CS35L56_MAIN_RENDER_USER_MUTE: c_uint = 0x3400024;
pub const CS35L56_MAIN_RENDER_USER_VOLUME: c_uint = 0x340002C;
pub const CS35L56_MAIN_POSTURE_NUMBER: c_uint = 0x3400094;
pub const CS35L56_PROTECTION_STATUS: c_uint = 0x34000D8;
pub const CS35L56_TRANSDUCER_ACTUAL_PS: c_uint = 0x3400150;
pub const CS35L56_DSP1_YMEM_UNPACKED24_6141: c_uint = 0x3405FF4;
pub const CS35L56_DSP1_PMEM_0: c_uint = 0x3800000;
pub const CS35L56_DSP1_PMEM_5114: c_uint = 0x3804FE8;

pub const CS35L63_DSP1_HALO_STATE: c_uint = 0x2803C04;
pub const CS35L63_DSP1_PM_CUR_STATE: c_uint = 0x2804518;
pub const CS35L63_PROTECTION_STATUS: c_uint = 0x340009C;
pub const CS35L63_TRANSDUCER_ACTUAL_PS: c_uint = 0x34000F4;
pub const CS35L63_MAIN_RENDER_USER_MUTE: c_uint = 0x3400020;
pub const CS35L63_MAIN_RENDER_USER_VOLUME: c_uint = 0x3400028;
pub const CS35L63_MAIN_POSTURE_NUMBER: c_uint = 0x3400068;
// DEVID
pub const CS35L56_DEVID_MASK: c_uint = 0x00FFFFFF;
// REVID
pub const CS35L56_AREVID_MASK: c_uint = 0x000000F0;
pub const CS35L56_MTLREVID_MASK: c_uint = 0x0000000F;
pub const CS35L56_REVID_B0: c_uint = 0x000000B0;
// PAD_INTF

pub const CS35L56_PAD_PULL_NONE: c_int = 0;
pub const CS35L56_PAD_PULL_UP: c_int = 1;
pub const CS35L56_PAD_PULL_DOWN: c_int = 2;
// UPDATE_REGS

// ASP_ENABLES1
pub const CS35L56_ASP_RX2_EN_SHIFT: c_int = 17;
pub const CS35L56_ASP_RX1_EN_SHIFT: c_int = 16;
pub const CS35L56_ASP_TX4_EN_SHIFT: c_int = 3;
pub const CS35L56_ASP_TX3_EN_SHIFT: c_int = 2;
pub const CS35L56_ASP_TX2_EN_SHIFT: c_int = 1;
pub const CS35L56_ASP_TX1_EN_SHIFT: c_int = 0;
// ASP_CONTROL1
pub const CS35L56_ASP_BCLK_FREQ_MASK: c_uint = 0x0000003F;
pub const CS35L56_ASP_BCLK_FREQ_SHIFT: c_int = 0;
// ASP_CONTROL2
pub const CS35L56_ASP_RX_WIDTH_MASK: c_uint = 0xFF000000;
pub const CS35L56_ASP_RX_WIDTH_SHIFT: c_int = 24;
pub const CS35L56_ASP_TX_WIDTH_MASK: c_uint = 0x00FF0000;
pub const CS35L56_ASP_TX_WIDTH_SHIFT: c_int = 16;
pub const CS35L56_ASP_FMT_MASK: c_uint = 0x00000700;
pub const CS35L56_ASP_FMT_SHIFT: c_int = 8;
pub const CS35L56_ASP_BCLK_INV_MASK: c_uint = 0x00000040;
pub const CS35L56_ASP_FSYNC_INV_MASK: c_uint = 0x00000004;
// ASP_CONTROL3
pub const CS35L56_ASP1_DOUT_HIZ_CTRL_MASK: c_uint = 0x00000003;
// ASP_DATA_CONTROL1
pub const CS35L56_ASP_TX_WL_MASK: c_uint = 0x0000003F;
// ASP_DATA_CONTROL5
pub const CS35L56_ASP_RX_WL_MASK: c_uint = 0x0000003F;
// ASPTXn_INPUT
pub const CS35L56_ASP_TXn_SRC_MASK: c_uint = 0x0000007F;
// SWIRETX[1..7]_SRC SDWTXn INPUT
pub const CS35L56_SWIRETXn_SRC_MASK: c_uint = 0x0000007F;
// IRQ1_STATUS
pub const CS35L56_IRQ1_STS_MASK: c_uint = 0x00000001;
// IRQ1_EINT_1
pub const CS35L56_AMP_SHORT_ERR_EINT1_MASK: c_uint = 0x80000000;
// IRQ1_EINT_2
pub const CS35L56_DSP_VIRTUAL2_MBOX_WR_EINT1_MASK: c_uint = 0x00200000;
// IRQ1_EINT_4
pub const CS35L56_OTP_BOOT_DONE_MASK: c_uint = 0x00000002;
// IRQ1_EINT_8
pub const CS35L56_TEMP_ERR_EINT1_MASK: c_uint = 0x80000000;
// MIXER_NGATE_CHn_CFG
pub const CS35L56_AUX_NGATE_CHn_EN: c_uint = 0x00000001;
// GPIOn_CTRL1

pub const CS35L56_GPIO_FN_GPIO: c_uint = 0x00000001;
// Mixer input sources
pub const CS35L56_INPUT_SRC_NONE: c_uint = 0x00;
pub const CS35L56_INPUT_SRC_ASP1RX1: c_uint = 0x08;
pub const CS35L56_INPUT_SRC_ASP1RX2: c_uint = 0x09;
pub const CS35L56_INPUT_SRC_VMON: c_uint = 0x18;
pub const CS35L56_INPUT_SRC_IMON: c_uint = 0x19;
pub const CS35L56_INPUT_SRC_ERR_VOL: c_uint = 0x20;
pub const CS35L56_INPUT_SRC_CLASSH: c_uint = 0x21;
pub const CS35L56_INPUT_SRC_VDDBMON: c_uint = 0x28;
pub const CS35L56_INPUT_SRC_VBSTMON: c_uint = 0x29;
pub const CS35L56_INPUT_SRC_DSP1TX1: c_uint = 0x32;
pub const CS35L56_INPUT_SRC_DSP1TX2: c_uint = 0x33;
pub const CS35L56_INPUT_SRC_DSP1TX3: c_uint = 0x34;
pub const CS35L56_INPUT_SRC_DSP1TX4: c_uint = 0x35;
pub const CS35L56_INPUT_SRC_DSP1TX5: c_uint = 0x36;
pub const CS35L56_INPUT_SRC_DSP1TX6: c_uint = 0x37;
pub const CS35L56_INPUT_SRC_DSP1TX7: c_uint = 0x38;
pub const CS35L56_INPUT_SRC_DSP1TX8: c_uint = 0x39;
pub const CS35L56_INPUT_SRC_TEMPMON: c_uint = 0x3A;
pub const CS35L56_INPUT_SRC_INTERPOLATOR: c_uint = 0x40;
pub const CS35L56_INPUT_SRC_SWIRE_DP1_CHANNEL1: c_uint = 0x44;
pub const CS35L56_INPUT_SRC_SWIRE_DP1_CHANNEL2: c_uint = 0x45;
pub const CS35L56_INPUT_MASK: c_uint = 0x7F;
pub const CS35L56_NUM_INPUT_SRC: c_int = 21;
// ASP formats
pub const CS35L56_ASP_FMT_DSP_A: c_int = 0;
pub const CS35L56_ASP_FMT_I2S: c_int = 2;
// ASP HiZ modes
pub const CS35L56_ASP_UNUSED_HIZ_OFF_HIZ: c_int = 3;
// MAIN_RENDER_ACTUAL_PS
pub const CS35L56_PS0: c_int = 0;
pub const CS35L56_PS3: c_int = 3;
// CS35L56_DSP_RESTRICT_STS1
pub const CS35L56_RESTRICTED_MASK: c_uint = 0x7;
// CS35L56_MAIN_RENDER_USER_MUTE
pub const CS35L56_MAIN_RENDER_USER_MUTE_MASK: c_int = 1;
// CS35L56_MAIN_RENDER_USER_VOLUME

pub const CS35L56_MAIN_RENDER_USER_VOLUME_MAX: c_int = 48;
pub const CS35L56_MAIN_RENDER_USER_VOLUME_MASK: c_uint = 0x0000FFC0;
pub const CS35L56_MAIN_RENDER_USER_VOLUME_SHIFT: c_int = 6;
pub const CS35L56_MAIN_RENDER_USER_VOLUME_SIGNBIT: c_int = 9;
// CS35L56_MAIN_POSTURE_NUMBER
pub const CS35L56_MAIN_POSTURE_MIN: c_int = 0;
pub const CS35L56_MAIN_POSTURE_MAX: c_int = 255;

// CS35L56_PROTECTION_STATUS

// Software Values
pub const CS35L56_HALO_STATE_SHUTDOWN: c_int = 1;
pub const CS35L56_HALO_STATE_BOOT_DONE: c_int = 2;
pub const CS35L56_MBOX_CMD_PING: c_uint = 0x0A000000;
pub const CS35L56_MBOX_CMD_AUDIO_PLAY: c_uint = 0x0B000001;
pub const CS35L56_MBOX_CMD_AUDIO_PAUSE: c_uint = 0x0B000002;
pub const CS35L56_MBOX_CMD_AUDIO_REINIT: c_uint = 0x0B000003;
pub const CS35L56_MBOX_CMD_AUDIO_CALIBRATION: c_uint = 0x0B000006;
pub const CS35L56_MBOX_CMD_HIBERNATE_NOW: c_uint = 0x02000001;
pub const CS35L56_MBOX_CMD_WAKEUP: c_uint = 0x02000002;
pub const CS35L56_MBOX_CMD_PREVENT_AUTO_HIBERNATE: c_uint = 0x02000003;
pub const CS35L56_MBOX_CMD_ALLOW_AUTO_HIBERNATE: c_uint = 0x02000004;
pub const CS35L56_MBOX_CMD_SHUTDOWN: c_uint = 0x02000005;
pub const CS35L56_MBOX_CMD_SYSTEM_RESET: c_uint = 0x02000007;
pub const CS35L56_MBOX_TIMEOUT_US: c_int = 5000;
pub const CS35L56_MBOX_POLL_US: c_int = 250;
pub const CS35L56_FW_REQ_ACTIVE_TIMEOUT_MS: c_int = 250;
pub const CS35L56_PS0_POLL_US: c_int = 500;
pub const CS35L56_PS0_TIMEOUT_US: c_int = 50000;
pub const CS35L56_PS3_POLL_US: c_int = 500;
pub const CS35L56_PS3_TIMEOUT_US: c_int = 300000;
pub const CS35L56_CAL_STATUS_SUCCESS: c_int = 1;
pub const CS35L56_CAL_STATUS_OUT_OF_RANGE: c_int = 3;
pub const CS35L56_CAL_SET_STATUS_UNKNOWN: c_int = 0;
pub const CS35L56_CAL_SET_STATUS_DEFAULT: c_int = 1;
pub const CS35L56_CAL_SET_STATUS_SET: c_int = 2;
pub const CS35L56_CONTROL_PORT_READY_US: c_int = 2200;
pub const CS35L56_HALO_STATE_POLL_US: c_int = 1000;
pub const CS35L56_HALO_STATE_TIMEOUT_US: c_int = 250000;
pub const CS35L56_RESET_PULSE_MIN_US: c_int = 1100;
pub const CS35L56_WAKE_HOLD_TIME_US: c_int = 1000;
pub const CS35L56_PAD_PULL_SETTLE_US: c_int = 10;

pub const CS35L56_SDW1_PLAYBACK_PORT: c_int = 1;
pub const CS35L56_SDW1_CAPTURE_PORT: c_int = 3;
pub const CS35L56_NUM_BULK_SUPPLIES: c_int = 3;
pub const CS35L56_NUM_DSP_REGIONS: c_int = 5;
pub const CS35L56_MAX_GPIO: c_int = 13;
pub const CS35L63_MAX_GPIO: c_int = 9;
// Additional margin for SYSTEM_RESET to control port ready on SPI

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l56_spi_payload {
    pub addr: __be32,
    pub pad: __be16,
    pub value: __be32,
    pub __packed: },
    pub 10): static_assert(sizeof(struct cs35l56_spi_payload) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l56_fw_reg {
    pub fw_ver: c_uint,
    pub halo_state: c_uint,
    pub pm_cur_stat: c_uint,
    pub prot_sts: c_uint,
    pub transducer_actual_ps: c_uint,
    pub user_mute: c_uint,
    pub user_volume: c_uint,
    pub posture_number: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l56_cal_debugfs_fops {
    pub calibrate: debugfs_short_fops,
    pub cal_temperature: debugfs_short_fops,
    pub cal_data: debugfs_short_fops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l56_base {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub dsp: *mut cs_dsp,
    pub irq: c_int,
    pub irq_lock: mutex,
    pub type: u8,
    pub rev: u8,
    pub init_done: bool,
    pub fw_patched: bool,
    pub secured: bool,
    pub can_hibernate: bool,
    pub cal_data_valid: bool,
    pub cal_index: i8,
    pub num_amps: u8,
    pub cal_data: cirrus_amp_cal_data,
    pub reset_gpio: *mut gpio_desc,
    pub spi_payload_buf: *mut cs35l56_spi_payload,
    pub fw_reg: *const cs35l56_fw_reg,
    pub calibration_controls: *const cirrus_amp_cal_controls,
    pub debugfs: *mut dentry,
    pub silicon_uid: u64,
    pub onchip_spkid_gpios: [u8; 5],
    pub num_onchip_spkid_gpios: u8,
    pub onchip_spkid_pulls: [u8; 5],
    pub num_onchip_spkid_pulls: u8,
}

extern "C" {
    pub fn cs35l56_set_asp_patch(cs35l56_base: *mut cs35l56_base) -> c_int;
}
extern "C" {
    pub fn cs35l56_set_patch(cs35l56_base: *mut cs35l56_base) -> c_int;
}
extern "C" {
    pub fn cs35l56_mbox_send(cs35l56_base: *mut cs35l56_base, command: c_uint) -> c_int;
}
extern "C" {
    pub fn cs35l56_firmware_shutdown(cs35l56_base: *mut cs35l56_base) -> c_int;
}
extern "C" {
    pub fn cs35l56_wait_for_firmware_boot(cs35l56_base: *mut cs35l56_base) -> c_int;
}
extern "C" {
    pub fn cs35l56_wait_control_port_ready();
}
extern "C" {
    pub fn cs35l56_wait_min_reset_pulse();
}
extern "C" {
    pub fn cs35l56_system_reset(cs35l56_base: *mut cs35l56_base, is_soundwire: bool);
}
extern "C" {
    pub fn cs35l56_irq_request(cs35l56_base: *mut cs35l56_base, irq: c_int) -> c_int;
}
extern "C" {
    pub fn cs35l56_is_fw_reload_needed(cs35l56_base: *mut cs35l56_base) -> c_int;
}
extern "C" {
    pub fn cs35l56_runtime_suspend_common(cs35l56_base: *mut cs35l56_base) -> c_int;
}
extern "C" {
    pub fn cs35l56_runtime_resume_common(cs35l56_base: *mut cs35l56_base, is_soundwire: bool) -> c_int;
}
extern "C" {
    pub fn cs35l56_init_cs_dsp(cs35l56_base: *mut cs35l56_base, cs_dsp: *mut cs_dsp);
}
extern "C" {
    pub fn cs35l56_get_calibration(cs35l56_base: *mut cs35l56_base) -> c_int;
}
extern "C" {
    pub fn cs35l56_factory_calibrate(cs35l56_base: *mut cs35l56_base) -> c_int;
}
extern "C" {
    pub fn cs35l56_remove_cal_debugfs(cs35l56_base: *mut cs35l56_base);
}
extern "C" {
    pub fn cs35l56_warn_if_firmware_missing(cs35l56_base: *mut cs35l56_base);
}
extern "C" {
    pub fn cs35l56_log_tuning(cs35l56_base: *mut cs35l56_base, cs_dsp: *mut cs_dsp);
}
extern "C" {
    pub fn cs35l56_hw_init(cs35l56_base: *mut cs35l56_base) -> c_int;
}
extern "C" {
    pub fn cs35l56_get_speaker_id(cs35l56_base: *mut cs35l56_base) -> c_int;
}
extern "C" {
    pub fn cs35l56_configure_onchip_spkid_pads(cs35l56_base: *mut cs35l56_base) -> c_int;
}
extern "C" {
    pub fn cs35l56_read_onchip_spkid(cs35l56_base: *mut cs35l56_base) -> c_int;
}
extern "C" {
    pub fn cs35l56_get_bclk_freq_id(freq: c_uint) -> c_int;
}
extern "C" {
    pub fn cs35l56_fill_supply_names(data: *mut regulator_bulk_data);
}
