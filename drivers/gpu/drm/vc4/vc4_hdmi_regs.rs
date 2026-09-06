//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vc4/vc4_hdmi_regs.h
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


pub const VC4_HDMI_PACKET_STRIDE: c_uint = 0x24;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vc4_hdmi_regs {
    VC4_INVALID = 0,
    VC4_HDMI,
    VC4_HD,
    VC5_CEC,
    VC5_CSC,
    VC5_DVP,
    VC5_PHY,
    VC5_RAM,
    VC5_RM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vc4_hdmi_field {
    HDMI_AUDIO_PACKET_CONFIG,
    HDMI_CEC_CNTRL_1,
    HDMI_CEC_CNTRL_2,
    HDMI_CEC_CNTRL_3,
    HDMI_CEC_CNTRL_4,
    HDMI_CEC_CNTRL_5,
    HDMI_CEC_CPU_CLEAR,
    HDMI_CEC_CPU_MASK_CLEAR,
    HDMI_CEC_CPU_MASK_SET,
    HDMI_CEC_CPU_MASK_STATUS,
    HDMI_CEC_CPU_STATUS,
    HDMI_CEC_CPU_SET,

//
// Transmit data, first byte is low byte of the 32-bit reg.
// MSB of each byte transmitted first.
//
    HDMI_CEC_RX_DATA_1,
    HDMI_CEC_RX_DATA_2,
    HDMI_CEC_RX_DATA_3,
    HDMI_CEC_RX_DATA_4,
    HDMI_CEC_TX_DATA_1,
    HDMI_CEC_TX_DATA_2,
    HDMI_CEC_TX_DATA_3,
    HDMI_CEC_TX_DATA_4,
    HDMI_CLOCK_STOP,
    HDMI_CORE_REV,
    HDMI_CRP_CFG,
    HDMI_CSC_12_11,
    HDMI_CSC_14_13,
    HDMI_CSC_22_21,
    HDMI_CSC_24_23,
    HDMI_CSC_32_31,
    HDMI_CSC_34_33,
    HDMI_CSC_CHANNEL_CTL,
    HDMI_CSC_CTL,

//
// 20-bit fields containing CTS values to be transmitted if
// !EXTERNAL_CTS_EN
//
    HDMI_CTS_0,
    HDMI_CTS_1,
    HDMI_DEEP_COLOR_CONFIG_1,
    HDMI_DVP_CTL,
    HDMI_FIFO_CTL,
    HDMI_FRAME_COUNT,
    HDMI_GCP_CONFIG,
    HDMI_GCP_WORD_1,
    HDMI_HORZA,
    HDMI_HORZB,
    HDMI_HOTPLUG,
    HDMI_HOTPLUG_INT,

//
// 3 bits per field, where each field maps from that
// corresponding MAI bus channel to the given HDMI channel.
//
    HDMI_MAI_CHANNEL_MAP,
    HDMI_MAI_CONFIG,
    HDMI_MAI_CTL,

//
// Register for DMAing in audio data to be transported over
// the MAI bus to the Falcon core.
//
    HDMI_MAI_DATA,

// Format header to be placed on the MAI data. Unused.
    HDMI_MAI_FMT,

// Last received format word on the MAI bus.
    HDMI_MAI_FORMAT,
    HDMI_MAI_SMP,
    HDMI_MAI_THR,
    HDMI_M_CTL,
    HDMI_RAM_PACKET_CONFIG,
    HDMI_RAM_PACKET_START,
    HDMI_RAM_PACKET_STATUS,
    HDMI_RM_CONTROL,
    HDMI_RM_FORMAT,
    HDMI_RM_OFFSET,
    HDMI_SCHEDULER_CONTROL,
    HDMI_SCRAMBLER_CTL,
    HDMI_SW_RESET_CONTROL,
    HDMI_TX_PHY_CHANNEL_SWAP,
    HDMI_TX_PHY_CLK_DIV,
    HDMI_TX_PHY_CTL_0,
    HDMI_TX_PHY_CTL_1,
    HDMI_TX_PHY_CTL_2,
    HDMI_TX_PHY_CTL_3,
    HDMI_TX_PHY_CTL_CK,
    HDMI_TX_PHY_PLL_CALIBRATION_CONFIG_1,
    HDMI_TX_PHY_PLL_CALIBRATION_CONFIG_2,
    HDMI_TX_PHY_PLL_CALIBRATION_CONFIG_4,
    HDMI_TX_PHY_PLL_CFG,
    HDMI_TX_PHY_PLL_CFG_PDIV,
    HDMI_TX_PHY_PLL_CTL_0,
    HDMI_TX_PHY_PLL_CTL_1,
    HDMI_TX_PHY_PLL_MISC_0,
    HDMI_TX_PHY_PLL_MISC_1,
    HDMI_TX_PHY_PLL_MISC_2,
    HDMI_TX_PHY_PLL_MISC_3,
    HDMI_TX_PHY_PLL_MISC_4,
    HDMI_TX_PHY_PLL_MISC_5,
    HDMI_TX_PHY_PLL_MISC_6,
    HDMI_TX_PHY_PLL_MISC_7,
    HDMI_TX_PHY_PLL_MISC_8,
    HDMI_TX_PHY_PLL_POST_KDIV,
    HDMI_TX_PHY_PLL_POWERUP_CTL,
    HDMI_TX_PHY_PLL_REFCLK,
    HDMI_TX_PHY_PLL_RESET_CTL,
    HDMI_TX_PHY_PLL_VCOCLK_DIV,
    HDMI_TX_PHY_POWERDOWN_CTL,
    HDMI_TX_PHY_POWERUP_CTL,
    HDMI_TX_PHY_RESET_CTL,
    HDMI_TX_PHY_TMDS_CLK_WORD_SEL,
    HDMI_VEC_INTERFACE_CFG,
    HDMI_VEC_INTERFACE_XBAR,
    HDMI_VERTA0,
    HDMI_VERTA1,
    HDMI_VERTB0,
    HDMI_VERTB1,
    HDMI_VID_CTL,
    HDMI_MISC_CONTROL,
    HDMI_FORMAT_DET_1,
    HDMI_FORMAT_DET_2,
    HDMI_FORMAT_DET_3,
    HDMI_FORMAT_DET_4,
    HDMI_FORMAT_DET_5,
    HDMI_FORMAT_DET_6,
    HDMI_FORMAT_DET_7,
    HDMI_FORMAT_DET_8,
    HDMI_FORMAT_DET_9,
    HDMI_FORMAT_DET_10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_hdmi_register {
    pub name: *mut c_char,
    pub reg: vc4_hdmi_regs,
    pub offset: c_uint,
}

extern "C" {
    pub fn readl(field->offset: base +) -> return;
}

