//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/au8522_priv.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//

pub const AU8522_ANALOG_MODE: c_int = 0;
pub const AU8522_DIGITAL_MODE: c_int = 1;
pub const AU8522_SUSPEND_MODE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum au8522_pads {
    AU8522_PAD_IF_INPUT,
    AU8522_PAD_VID_OUT,
    AU8522_PAD_AUDIO_OUT,
    AU8522_NUM_PADS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct au8522_state {
    pub c: *mut i2c_client,
    pub i2c: *mut i2c_adapter,
    pub operational_mode: u8,
// Used for sharing of the state between analog and digital mode
    pub i2c_props: tuner_i2c_props,
    pub hybrid_tuner_instance_list: list_head,
// configuration settings
    pub config: au8522_config,
    pub frontend: dvb_frontend,
    pub current_frequency: u32,
    pub current_modulation: fe_modulation,
    pub fe_status: u32,
    pub led_state: c_uint,
// Analog settings
    pub sd: v4l2_subdev,
    pub std: v4l2_std_id,
    pub vid_input: c_int,
    pub aud_input: c_int,
    pub id: u32,
    pub rev: u32,
    pub hdl: v4l2_ctrl_handler,
    pub pads: [media_pad; AU8522_NUM_PADS],
}

// These are routines shared by both the VSB/QAM demodulator and the analog
extern "C" {
    pub fn au8522_writereg(state: *mut au8522_state, reg: u16, data: u8) -> c_int;
}
extern "C" {
    pub fn au8522_readreg(state: *mut au8522_state, reg: u16) -> u8;
}
extern "C" {
    pub fn au8522_init(fe: *mut dvb_frontend) -> c_int;
}
extern "C" {
    pub fn au8522_sleep(fe: *mut dvb_frontend) -> c_int;
}
extern "C" {
    pub fn au8522_release_state(state: *mut au8522_state);
}
extern "C" {
    pub fn au8522_i2c_gate_ctrl(fe: *mut dvb_frontend, enable: c_int) -> c_int;
}
extern "C" {
    pub fn au8522_analog_i2c_gate_ctrl(fe: *mut dvb_frontend, enable: c_int) -> c_int;
}
extern "C" {
    pub fn au8522_led_ctrl(state: *mut au8522_state, led: c_int) -> c_int;
}
// REGISTERS
pub const AU8522_INPUT_CONTROL_REG081H: c_uint = 0x081;
pub const AU8522_PGA_CONTROL_REG082H: c_uint = 0x082;
pub const AU8522_CLAMPING_CONTROL_REG083H: c_uint = 0x083;
pub const AU8522_MODULE_CLOCK_CONTROL_REG0A3H: c_uint = 0x0A3;
pub const AU8522_SYSTEM_MODULE_CONTROL_0_REG0A4H: c_uint = 0x0A4;
pub const AU8522_SYSTEM_MODULE_CONTROL_1_REG0A5H: c_uint = 0x0A5;
pub const AU8522_AGC_CONTROL_RANGE_REG0A6H: c_uint = 0x0A6;
pub const AU8522_SYSTEM_GAIN_CONTROL_REG0A7H: c_uint = 0x0A7;
pub const AU8522_TUNER_AGC_RF_STOP_REG0A8H: c_uint = 0x0A8;
pub const AU8522_TUNER_AGC_RF_START_REG0A9H: c_uint = 0x0A9;
pub const AU8522_TUNER_RF_AGC_DEFAULT_REG0AAH: c_uint = 0x0AA;
pub const AU8522_TUNER_AGC_IF_STOP_REG0ABH: c_uint = 0x0AB;
pub const AU8522_TUNER_AGC_IF_START_REG0ACH: c_uint = 0x0AC;
pub const AU8522_TUNER_AGC_IF_DEFAULT_REG0ADH: c_uint = 0x0AD;
pub const AU8522_TUNER_AGC_STEP_REG0AEH: c_uint = 0x0AE;
pub const AU8522_TUNER_GAIN_STEP_REG0AFH: c_uint = 0x0AF;
// Receiver registers
pub const AU8522_FRMREGTHRD1_REG0B0H: c_uint = 0x0B0;
pub const AU8522_FRMREGAGC1H_REG0B1H: c_uint = 0x0B1;
pub const AU8522_FRMREGSHIFT1_REG0B2H: c_uint = 0x0B2;
pub const AU8522_TOREGAGC1_REG0B3H: c_uint = 0x0B3;
pub const AU8522_TOREGASHIFT1_REG0B4H: c_uint = 0x0B4;
pub const AU8522_FRMREGBBH_REG0B5H: c_uint = 0x0B5;
pub const AU8522_FRMREGBBM_REG0B6H: c_uint = 0x0B6;
pub const AU8522_FRMREGBBL_REG0B7H: c_uint = 0x0B7;
// 0xB8 TO 0xD7 are the filter coefficients
pub const AU8522_FRMREGTHRD2_REG0D8H: c_uint = 0x0D8;
pub const AU8522_FRMREGAGC2H_REG0D9H: c_uint = 0x0D9;
pub const AU8522_TOREGAGC2_REG0DAH: c_uint = 0x0DA;
pub const AU8522_TOREGSHIFT2_REG0DBH: c_uint = 0x0DB;
pub const AU8522_FRMREGPILOTH_REG0DCH: c_uint = 0x0DC;
pub const AU8522_FRMREGPILOTM_REG0DDH: c_uint = 0x0DD;
pub const AU8522_FRMREGPILOTL_REG0DEH: c_uint = 0x0DE;
pub const AU8522_TOREGFREQ_REG0DFH: c_uint = 0x0DF;
pub const AU8522_RX_PGA_RFOUT_REG0EBH: c_uint = 0x0EB;
pub const AU8522_RX_PGA_IFOUT_REG0ECH: c_uint = 0x0EC;
pub const AU8522_RX_PGA_PGAOUT_REG0EDH: c_uint = 0x0ED;
pub const AU8522_CHIP_MODE_REG0FEH: c_uint = 0x0FE;
// I2C bus control registers
pub const AU8522_I2C_CONTROL_REG0_REG090H: c_uint = 0x090;
pub const AU8522_I2C_CONTROL_REG1_REG091H: c_uint = 0x091;
pub const AU8522_I2C_STATUS_REG092H: c_uint = 0x092;
pub const AU8522_I2C_WR_DATA0_REG093H: c_uint = 0x093;
pub const AU8522_I2C_WR_DATA1_REG094H: c_uint = 0x094;
pub const AU8522_I2C_WR_DATA2_REG095H: c_uint = 0x095;
pub const AU8522_I2C_WR_DATA3_REG096H: c_uint = 0x096;
pub const AU8522_I2C_WR_DATA4_REG097H: c_uint = 0x097;
pub const AU8522_I2C_WR_DATA5_REG098H: c_uint = 0x098;
pub const AU8522_I2C_WR_DATA6_REG099H: c_uint = 0x099;
pub const AU8522_I2C_WR_DATA7_REG09AH: c_uint = 0x09A;
pub const AU8522_I2C_RD_DATA0_REG09BH: c_uint = 0x09B;
pub const AU8522_I2C_RD_DATA1_REG09CH: c_uint = 0x09C;
pub const AU8522_I2C_RD_DATA2_REG09DH: c_uint = 0x09D;
pub const AU8522_I2C_RD_DATA3_REG09EH: c_uint = 0x09E;
pub const AU8522_I2C_RD_DATA4_REG09FH: c_uint = 0x09F;
pub const AU8522_I2C_RD_DATA5_REG0A0H: c_uint = 0x0A0;
pub const AU8522_I2C_RD_DATA6_REG0A1H: c_uint = 0x0A1;
pub const AU8522_I2C_RD_DATA7_REG0A2H: c_uint = 0x0A2;
pub const AU8522_ENA_USB_REG101H: c_uint = 0x101;
pub const AU8522_I2S_CTRL_0_REG110H: c_uint = 0x110;
pub const AU8522_I2S_CTRL_1_REG111H: c_uint = 0x111;
pub const AU8522_I2S_CTRL_2_REG112H: c_uint = 0x112;
pub const AU8522_FRMREGFFECONTROL_REG121H: c_uint = 0x121;
pub const AU8522_FRMREGDFECONTROL_REG122H: c_uint = 0x122;
pub const AU8522_CARRFREQOFFSET0_REG201H: c_uint = 0x201;
pub const AU8522_CARRFREQOFFSET1_REG202H: c_uint = 0x202;
pub const AU8522_DECIMATION_GAIN_REG21AH: c_uint = 0x21A;
pub const AU8522_FRMREGIFSLP_REG21BH: c_uint = 0x21B;
pub const AU8522_FRMREGTHRDL2_REG21CH: c_uint = 0x21C;
pub const AU8522_FRMREGSTEP3DB_REG21DH: c_uint = 0x21D;
pub const AU8522_DAGC_GAIN_ADJUSTMENT_REG21EH: c_uint = 0x21E;
pub const AU8522_FRMREGPLLMODE_REG21FH: c_uint = 0x21F;
pub const AU8522_FRMREGCSTHRD_REG220H: c_uint = 0x220;
pub const AU8522_FRMREGCRLOCKDMAX_REG221H: c_uint = 0x221;
pub const AU8522_FRMREGCRPERIODMASK_REG222H: c_uint = 0x222;
pub const AU8522_FRMREGCRLOCK0THH_REG223H: c_uint = 0x223;
pub const AU8522_FRMREGCRLOCK1THH_REG224H: c_uint = 0x224;
pub const AU8522_FRMREGCRLOCK0THL_REG225H: c_uint = 0x225;
pub const AU8522_FRMREGCRLOCK1THL_REG226H: c_uint = 0x226;
pub const AU_FRMREGPLLACQPHASESCL_REG227H: c_uint = 0x227;
pub const AU8522_FRMREGFREQFBCTRL_REG228H: c_uint = 0x228;
// Analog TV Decoder
pub const AU8522_TVDEC_STATUS_REG000H: c_uint = 0x000;
pub const AU8522_TVDEC_INT_STATUS_REG001H: c_uint = 0x001;
pub const AU8522_TVDEC_MACROVISION_STATUS_REG002H: c_uint = 0x002;
pub const AU8522_TVDEC_SHARPNESSREG009H: c_uint = 0x009;
pub const AU8522_TVDEC_BRIGHTNESS_REG00AH: c_uint = 0x00A;
pub const AU8522_TVDEC_CONTRAST_REG00BH: c_uint = 0x00B;
pub const AU8522_TVDEC_SATURATION_CB_REG00CH: c_uint = 0x00C;
pub const AU8522_TVDEC_SATURATION_CR_REG00DH: c_uint = 0x00D;
pub const AU8522_TVDEC_HUE_H_REG00EH: c_uint = 0x00E;
pub const AU8522_TVDEC_HUE_L_REG00FH: c_uint = 0x00F;
pub const AU8522_TVDEC_INT_MASK_REG010H: c_uint = 0x010;
pub const AU8522_VIDEO_MODE_REG011H: c_uint = 0x011;
pub const AU8522_TVDEC_PGA_REG012H: c_uint = 0x012;
pub const AU8522_TVDEC_COMB_MODE_REG015H: c_uint = 0x015;
pub const AU8522_REG016H: c_uint = 0x016;
pub const AU8522_TVDED_DBG_MODE_REG060H: c_uint = 0x060;
pub const AU8522_TVDEC_FORMAT_CTRL1_REG061H: c_uint = 0x061;
pub const AU8522_TVDEC_FORMAT_CTRL2_REG062H: c_uint = 0x062;
pub const AU8522_TVDEC_VCR_DET_LLIM_REG063H: c_uint = 0x063;
pub const AU8522_TVDEC_VCR_DET_HLIM_REG064H: c_uint = 0x064;
pub const AU8522_TVDEC_COMB_VDIF_THR1_REG065H: c_uint = 0x065;
pub const AU8522_TVDEC_COMB_VDIF_THR2_REG066H: c_uint = 0x066;
pub const AU8522_TVDEC_COMB_VDIF_THR3_REG067H: c_uint = 0x067;
pub const AU8522_TVDEC_COMB_NOTCH_THR_REG068H: c_uint = 0x068;
pub const AU8522_TVDEC_COMB_HDIF_THR1_REG069H: c_uint = 0x069;
pub const AU8522_TVDEC_COMB_HDIF_THR2_REG06AH: c_uint = 0x06A;
pub const AU8522_TVDEC_COMB_HDIF_THR3_REG06BH: c_uint = 0x06B;
pub const AU8522_TVDEC_COMB_DCDIF_THR1_REG06CH: c_uint = 0x06C;
pub const AU8522_TVDEC_COMB_DCDIF_THR2_REG06DH: c_uint = 0x06D;
pub const AU8522_TVDEC_COMB_DCDIF_THR3_REG06EH: c_uint = 0x06E;
pub const AU8522_TVDEC_UV_SEP_THR_REG06FH: c_uint = 0x06F;
pub const AU8522_TVDEC_COMB_DC_THR1_NTSC_REG070H: c_uint = 0x070;
pub const AU8522_TVDEC_COMB_DC_THR2_NTSC_REG073H: c_uint = 0x073;
pub const AU8522_TVDEC_DCAGC_CTRL_REG077H: c_uint = 0x077;
pub const AU8522_TVDEC_PIC_START_ADJ_REG078H: c_uint = 0x078;
pub const AU8522_TVDEC_AGC_HIGH_LIMIT_REG079H: c_uint = 0x079;
pub const AU8522_TVDEC_MACROVISION_SYNC_THR_REG07AH: c_uint = 0x07A;
pub const AU8522_TVDEC_INTRP_CTRL_REG07BH: c_uint = 0x07B;
pub const AU8522_TVDEC_PLL_STATUS_REG07EH: c_uint = 0x07E;
pub const AU8522_TVDEC_FSC_FREQ_REG07FH: c_uint = 0x07F;
pub const AU8522_TVDEC_AGC_LOW_LIMIT_REG0E4H: c_uint = 0x0E4;
pub const AU8522_TOREGAAGC_REG0E5H: c_uint = 0x0E5;
pub const AU8522_TVDEC_CHROMA_AGC_REG401H: c_uint = 0x401;
pub const AU8522_TVDEC_CHROMA_SFT_REG402H: c_uint = 0x402;
pub const AU8522_FILTER_COEF_R410: c_uint = 0x410;
pub const AU8522_FILTER_COEF_R411: c_uint = 0x411;
pub const AU8522_FILTER_COEF_R412: c_uint = 0x412;
pub const AU8522_FILTER_COEF_R413: c_uint = 0x413;
pub const AU8522_FILTER_COEF_R414: c_uint = 0x414;
pub const AU8522_FILTER_COEF_R415: c_uint = 0x415;
pub const AU8522_FILTER_COEF_R416: c_uint = 0x416;
pub const AU8522_FILTER_COEF_R417: c_uint = 0x417;
pub const AU8522_FILTER_COEF_R418: c_uint = 0x418;
pub const AU8522_FILTER_COEF_R419: c_uint = 0x419;
pub const AU8522_FILTER_COEF_R41A: c_uint = 0x41A;
pub const AU8522_FILTER_COEF_R41B: c_uint = 0x41B;
pub const AU8522_FILTER_COEF_R41C: c_uint = 0x41C;
pub const AU8522_FILTER_COEF_R41D: c_uint = 0x41D;
pub const AU8522_FILTER_COEF_R41E: c_uint = 0x41E;
pub const AU8522_FILTER_COEF_R41F: c_uint = 0x41F;
pub const AU8522_FILTER_COEF_R420: c_uint = 0x420;
pub const AU8522_FILTER_COEF_R421: c_uint = 0x421;
pub const AU8522_FILTER_COEF_R422: c_uint = 0x422;
pub const AU8522_FILTER_COEF_R423: c_uint = 0x423;
pub const AU8522_FILTER_COEF_R424: c_uint = 0x424;
pub const AU8522_FILTER_COEF_R425: c_uint = 0x425;
pub const AU8522_FILTER_COEF_R426: c_uint = 0x426;
pub const AU8522_FILTER_COEF_R427: c_uint = 0x427;
pub const AU8522_FILTER_COEF_R428: c_uint = 0x428;
pub const AU8522_FILTER_COEF_R429: c_uint = 0x429;
pub const AU8522_FILTER_COEF_R42A: c_uint = 0x42A;
pub const AU8522_FILTER_COEF_R42B: c_uint = 0x42B;
pub const AU8522_FILTER_COEF_R42C: c_uint = 0x42C;
pub const AU8522_FILTER_COEF_R42D: c_uint = 0x42D;
// VBI Control Registers
pub const AU8522_TVDEC_VBI_RX_FIFO_CONTAIN_REG004H: c_uint = 0x004;
pub const AU8522_TVDEC_VBI_TX_FIFO_CONTAIN_REG005H: c_uint = 0x005;
pub const AU8522_TVDEC_VBI_RX_FIFO_READ_REG006H: c_uint = 0x006;
pub const AU8522_TVDEC_VBI_FIFO_STATUS_REG007H: c_uint = 0x007;
pub const AU8522_TVDEC_VBI_CTRL_H_REG017H: c_uint = 0x017;
pub const AU8522_TVDEC_VBI_CTRL_L_REG018H: c_uint = 0x018;
pub const AU8522_TVDEC_VBI_USER_TOTAL_BITS_REG019H: c_uint = 0x019;
pub const AU8522_TVDEC_VBI_USER_TUNIT_H_REG01AH: c_uint = 0x01A;
pub const AU8522_TVDEC_VBI_USER_TUNIT_L_REG01BH: c_uint = 0x01B;
pub const AU8522_TVDEC_VBI_USER_THRESH1_REG01CH: c_uint = 0x01C;
pub const AU8522_TVDEC_VBI_USER_FRAME_PAT2_REG01EH: c_uint = 0x01E;
pub const AU8522_TVDEC_VBI_USER_FRAME_PAT1_REG01FH: c_uint = 0x01F;
pub const AU8522_TVDEC_VBI_USER_FRAME_PAT0_REG020H: c_uint = 0x020;
pub const AU8522_TVDEC_VBI_USER_FRAME_MASK2_REG021H: c_uint = 0x021;
pub const AU8522_TVDEC_VBI_USER_FRAME_MASK1_REG022H: c_uint = 0x022;
pub const AU8522_TVDEC_VBI_USER_FRAME_MASK0_REG023H: c_uint = 0x023;
pub const AU8522_REG071H: c_uint = 0x071;
pub const AU8522_REG072H: c_uint = 0x072;
pub const AU8522_REG074H: c_uint = 0x074;
pub const AU8522_REG075H: c_uint = 0x075;
// Digital Demodulator Registers
pub const AU8522_FRAME_COUNT0_REG084H: c_uint = 0x084;
pub const AU8522_RS_STATUS_G0_REG085H: c_uint = 0x085;
pub const AU8522_RS_STATUS_B0_REG086H: c_uint = 0x086;
pub const AU8522_RS_STATUS_E_REG087H: c_uint = 0x087;
pub const AU8522_DEMODULATION_STATUS_REG088H: c_uint = 0x088;
pub const AU8522_TOREGTRESTATUS_REG0E6H: c_uint = 0x0E6;
pub const AU8522_TSPORT_CONTROL_REG10BH: c_uint = 0x10B;
pub const AU8522_TSTHES_REG10CH: c_uint = 0x10C;
pub const AU8522_FRMREGDFEKEEP_REG301H: c_uint = 0x301;
pub const AU8522_DFE_AVERAGE_REG302H: c_uint = 0x302;
pub const AU8522_FRMREGEQLERRWIN_REG303H: c_uint = 0x303;
pub const AU8522_FRMREGFFEKEEP_REG304H: c_uint = 0x304;
pub const AU8522_FRMREGDFECONTROL1_REG305H: c_uint = 0x305;
pub const AU8522_FRMREGEQLERRLOW_REG306H: c_uint = 0x306;
pub const AU8522_REG42EH: c_uint = 0x42E;
pub const AU8522_REG42FH: c_uint = 0x42F;
pub const AU8522_REG430H: c_uint = 0x430;
pub const AU8522_REG431H: c_uint = 0x431;
pub const AU8522_REG432H: c_uint = 0x432;
pub const AU8522_REG433H: c_uint = 0x433;
pub const AU8522_REG434H: c_uint = 0x434;
pub const AU8522_REG435H: c_uint = 0x435;
pub const AU8522_REG436H: c_uint = 0x436;
// GPIO Registers
pub const AU8522_GPIO_CONTROL_REG0E0H: c_uint = 0x0E0;
pub const AU8522_GPIO_STATUS_REG0E1H: c_uint = 0x0E1;
pub const AU8522_GPIO_DATA_REG0E2H: c_uint = 0x0E2;
// Audio Control Registers
pub const AU8522_AUDIOAGC_REG0EEH: c_uint = 0x0EE;
pub const AU8522_AUDIO_STATUS_REG0F0H: c_uint = 0x0F0;
pub const AU8522_AUDIO_MODE_REG0F1H: c_uint = 0x0F1;
pub const AU8522_AUDIO_VOLUME_L_REG0F2H: c_uint = 0x0F2;
pub const AU8522_AUDIO_VOLUME_R_REG0F3H: c_uint = 0x0F3;
pub const AU8522_AUDIO_VOLUME_REG0F4H: c_uint = 0x0F4;
pub const AU8522_FRMREGAUPHASE_REG0F7H: c_uint = 0x0F7;
pub const AU8522_REG0F9H: c_uint = 0x0F9;
pub const AU8522_AUDIOAGC2_REG605H: c_uint = 0x605;
pub const AU8522_AUDIOFREQ_REG606H: c_uint = 0x606;
//
// Format control 1
// VCR Mode 7-6
pub const AU8522_TVDEC_FORMAT_CTRL1_REG061H_VCR_MODE_YES: c_uint = 0x80;
pub const AU8522_TVDEC_FORMAT_CTRL1_REG061H_VCR_MODE_NO: c_uint = 0x40;
pub const AU8522_TVDEC_FORMAT_CTRL1_REG061H_VCR_MODE_AUTO: c_uint = 0x00;
// Field len 5-4
pub const AU8522_TVDEC_FORMAT_CTRL1_REG061H_FIELD_LEN_625: c_uint = 0x20;
pub const AU8522_TVDEC_FORMAT_CTRL1_REG061H_FIELD_LEN_525: c_uint = 0x10;
pub const AU8522_TVDEC_FORMAT_CTRL1_REG061H_FIELD_LEN_AUTO: c_uint = 0x00;
// Line len (us) 3-2
pub const AU8522_TVDEC_FORMAT_CTRL1_REG061H_LINE_LEN_64_000: c_uint = 0x0b;
pub const AU8522_TVDEC_FORMAT_CTRL1_REG061H_LINE_LEN_63_492: c_uint = 0x08;
pub const AU8522_TVDEC_FORMAT_CTRL1_REG061H_LINE_LEN_63_556: c_uint = 0x04;
// Subcarrier freq 1-0
pub const AU8522_TVDEC_FORMAT_CTRL1_REG061H_SUBCARRIER_NTSC_AUTO: c_uint = 0x03;
pub const AU8522_TVDEC_FORMAT_CTRL1_REG061H_SUBCARRIER_NTSC_443: c_uint = 0x02;
pub const AU8522_TVDEC_FORMAT_CTRL1_REG061H_SUBCARRIER_NTSC_MN: c_uint = 0x01;
pub const AU8522_TVDEC_FORMAT_CTRL1_REG061H_SUBCARRIER_NTSC_50: c_uint = 0x00;
// Format control 2
pub const AU8522_TVDEC_FORMAT_CTRL2_REG062H_STD_AUTODETECT: c_uint = 0x00;
pub const AU8522_TVDEC_FORMAT_CTRL2_REG062H_STD_NTSC: c_uint = 0x01;
pub const AU8522_TVDEC_FORMAT_CTRL2_REG062H_STD_PAL_M: c_uint = 0x02;
pub const AU8522_INPUT_CONTROL_REG081H_ATSC: c_uint = 0xC4;
pub const AU8522_INPUT_CONTROL_REG081H_ATVRF: c_uint = 0xC4;
pub const AU8522_INPUT_CONTROL_REG081H_ATVRF13: c_uint = 0xC4;
pub const AU8522_INPUT_CONTROL_REG081H_J83B64: c_uint = 0xC4;
pub const AU8522_INPUT_CONTROL_REG081H_J83B256: c_uint = 0xC4;
pub const AU8522_INPUT_CONTROL_REG081H_CVBS: c_uint = 0x20;
pub const AU8522_INPUT_CONTROL_REG081H_CVBS_CH1: c_uint = 0xA2;
pub const AU8522_INPUT_CONTROL_REG081H_CVBS_CH2: c_uint = 0xA0;
pub const AU8522_INPUT_CONTROL_REG081H_CVBS_CH3: c_uint = 0x69;
pub const AU8522_INPUT_CONTROL_REG081H_CVBS_CH4: c_uint = 0x68;
pub const AU8522_INPUT_CONTROL_REG081H_CVBS_CH4_SIF: c_uint = 0x28;
// CH1 AS Y,CH3 AS C
pub const AU8522_INPUT_CONTROL_REG081H_SVIDEO_CH13: c_uint = 0x23;
// CH2 AS Y,CH4 AS C
pub const AU8522_INPUT_CONTROL_REG081H_SVIDEO_CH24: c_uint = 0x20;
pub const AU8522_MODULE_CLOCK_CONTROL_REG0A3H_ATSC: c_uint = 0x0C;
pub const AU8522_MODULE_CLOCK_CONTROL_REG0A3H_J83B64: c_uint = 0x09;
pub const AU8522_MODULE_CLOCK_CONTROL_REG0A3H_J83B256: c_uint = 0x09;
pub const AU8522_MODULE_CLOCK_CONTROL_REG0A3H_CVBS: c_uint = 0x12;
pub const AU8522_MODULE_CLOCK_CONTROL_REG0A3H_ATVRF: c_uint = 0x1A;
pub const AU8522_MODULE_CLOCK_CONTROL_REG0A3H_ATVRF13: c_uint = 0x1A;
pub const AU8522_MODULE_CLOCK_CONTROL_REG0A3H_SVIDEO: c_uint = 0x02;
pub const AU8522_SYSTEM_MODULE_CONTROL_0_REG0A4H_CLEAR: c_uint = 0x00;
pub const AU8522_SYSTEM_MODULE_CONTROL_0_REG0A4H_SVIDEO: c_uint = 0x9C;
pub const AU8522_SYSTEM_MODULE_CONTROL_0_REG0A4H_CVBS: c_uint = 0x9D;
pub const AU8522_SYSTEM_MODULE_CONTROL_0_REG0A4H_ATSC: c_uint = 0xE8;
pub const AU8522_SYSTEM_MODULE_CONTROL_0_REG0A4H_J83B256: c_uint = 0xCA;
pub const AU8522_SYSTEM_MODULE_CONTROL_0_REG0A4H_J83B64: c_uint = 0xCA;
pub const AU8522_SYSTEM_MODULE_CONTROL_0_REG0A4H_ATVRF: c_uint = 0xDD;
pub const AU8522_SYSTEM_MODULE_CONTROL_0_REG0A4H_ATVRF13: c_uint = 0xDD;
pub const AU8522_SYSTEM_MODULE_CONTROL_0_REG0A4H_PAL: c_uint = 0xDD;
pub const AU8522_SYSTEM_MODULE_CONTROL_0_REG0A4H_FM: c_uint = 0xDD;
pub const AU8522_SYSTEM_MODULE_CONTROL_1_REG0A5H_ATSC: c_uint = 0x80;
pub const AU8522_SYSTEM_MODULE_CONTROL_1_REG0A5H_J83B256: c_uint = 0x80;
pub const AU8522_SYSTEM_MODULE_CONTROL_1_REG0A5H_J83B64: c_uint = 0x80;
pub const AU8522_SYSTEM_MODULE_CONTROL_1_REG0A5H_DONGLE_ATSC: c_uint = 0x40;
pub const AU8522_SYSTEM_MODULE_CONTROL_1_REG0A5H_DONGLE_J83B256: c_uint = 0x40;
pub const AU8522_SYSTEM_MODULE_CONTROL_1_REG0A5H_DONGLE_J83B64: c_uint = 0x40;
pub const AU8522_SYSTEM_MODULE_CONTROL_1_REG0A5H_DONGLE_CLEAR: c_uint = 0x00;
pub const AU8522_SYSTEM_MODULE_CONTROL_1_REG0A5H_ATVRF: c_uint = 0x01;
pub const AU8522_SYSTEM_MODULE_CONTROL_1_REG0A5H_ATVRF13: c_uint = 0x01;
pub const AU8522_SYSTEM_MODULE_CONTROL_1_REG0A5H_SVIDEO: c_uint = 0x04;
pub const AU8522_SYSTEM_MODULE_CONTROL_1_REG0A5H_CVBS: c_uint = 0x01;
pub const AU8522_SYSTEM_MODULE_CONTROL_1_REG0A5H_PWM: c_uint = 0x03;
pub const AU8522_SYSTEM_MODULE_CONTROL_1_REG0A5H_IIS: c_uint = 0x09;
pub const AU8522_SYSTEM_MODULE_CONTROL_1_REG0A5H_PAL: c_uint = 0x01;
pub const AU8522_SYSTEM_MODULE_CONTROL_1_REG0A5H_FM: c_uint = 0x01;
// STILL NEED TO BE REFACTORED @@@@@@@@@@@@@@
pub const AU8522_TVDEC_CONTRAST_REG00BH_CVBS: c_uint = 0x79;
pub const AU8522_TVDEC_SATURATION_CB_REG00CH_CVBS: c_uint = 0x80;
pub const AU8522_TVDEC_SATURATION_CR_REG00DH_CVBS: c_uint = 0x80;
pub const AU8522_TVDEC_HUE_H_REG00EH_CVBS: c_uint = 0x00;
pub const AU8522_TVDEC_HUE_L_REG00FH_CVBS: c_uint = 0x00;
pub const AU8522_TVDEC_PGA_REG012H_CVBS: c_uint = 0x0F;
pub const AU8522_TVDEC_COMB_MODE_REG015H_CVBS: c_uint = 0x00;
pub const AU8522_REG016H_CVBS: c_uint = 0x00;
pub const AU8522_TVDED_DBG_MODE_REG060H_CVBS: c_uint = 0x00;
pub const AU8522_TVDEC_VCR_DET_LLIM_REG063H_CVBS: c_uint = 0x19;
pub const AU8522_REG0F9H_AUDIO: c_uint = 0x20;
pub const AU8522_TVDEC_VCR_DET_HLIM_REG064H_CVBS: c_uint = 0xA7;
pub const AU8522_TVDEC_COMB_VDIF_THR1_REG065H_CVBS: c_uint = 0x0A;
pub const AU8522_TVDEC_COMB_VDIF_THR2_REG066H_CVBS: c_uint = 0x32;
pub const AU8522_TVDEC_COMB_VDIF_THR3_REG067H_CVBS: c_uint = 0x19;
pub const AU8522_TVDEC_COMB_NOTCH_THR_REG068H_CVBS: c_uint = 0x23;
pub const AU8522_TVDEC_COMB_HDIF_THR1_REG069H_CVBS: c_uint = 0x41;
pub const AU8522_TVDEC_COMB_HDIF_THR2_REG06AH_CVBS: c_uint = 0x0A;
pub const AU8522_TVDEC_COMB_HDIF_THR3_REG06BH_CVBS: c_uint = 0x32;
pub const AU8522_TVDEC_COMB_DCDIF_THR1_REG06CH_CVBS: c_uint = 0x34;
pub const AU8522_TVDEC_COMB_DCDIF_THR1_REG06CH_SVIDEO: c_uint = 0x2a;
pub const AU8522_TVDEC_COMB_DCDIF_THR2_REG06DH_CVBS: c_uint = 0x05;
pub const AU8522_TVDEC_COMB_DCDIF_THR2_REG06DH_SVIDEO: c_uint = 0x15;
pub const AU8522_TVDEC_COMB_DCDIF_THR3_REG06EH_CVBS: c_uint = 0x6E;
pub const AU8522_TVDEC_UV_SEP_THR_REG06FH_CVBS: c_uint = 0x0F;
pub const AU8522_TVDEC_COMB_DC_THR1_NTSC_REG070H_CVBS: c_uint = 0x80;
pub const AU8522_REG071H_CVBS: c_uint = 0x18;
pub const AU8522_REG072H_CVBS: c_uint = 0x30;
pub const AU8522_TVDEC_COMB_DC_THR2_NTSC_REG073H_CVBS: c_uint = 0xF0;
pub const AU8522_REG074H_CVBS: c_uint = 0x80;
pub const AU8522_REG075H_CVBS: c_uint = 0xF0;
pub const AU8522_TVDEC_DCAGC_CTRL_REG077H_CVBS: c_uint = 0xFB;
pub const AU8522_TVDEC_PIC_START_ADJ_REG078H_CVBS: c_uint = 0x04;
pub const AU8522_TVDEC_AGC_HIGH_LIMIT_REG079H_CVBS: c_uint = 0x00;
pub const AU8522_TVDEC_MACROVISION_SYNC_THR_REG07AH_CVBS: c_uint = 0x00;
pub const AU8522_TVDEC_INTRP_CTRL_REG07BH_CVBS: c_uint = 0xEE;
pub const AU8522_TVDEC_AGC_LOW_LIMIT_REG0E4H_CVBS: c_uint = 0xFE;
pub const AU8522_TOREGAAGC_REG0E5H_CVBS: c_uint = 0x00;
pub const AU8522_TVDEC_VBI6A_REG035H_CVBS: c_uint = 0x40;
// Enables Closed captioning
pub const AU8522_TVDEC_VBI_CTRL_H_REG017H_CCON: c_uint = 0x21;
