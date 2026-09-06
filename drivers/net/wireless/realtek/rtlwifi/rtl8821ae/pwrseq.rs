//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8821ae/pwrseq.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright(c) 2009-2010  Realtek Corporation.

pub const RTL8812_TRANS_CARDEMU_TO_ACT_STEPS: c_int = 15;
pub const RTL8812_TRANS_ACT_TO_CARDEMU_STEPS: c_int = 15;
pub const RTL8812_TRANS_CARDEMU_TO_SUS_STEPS: c_int = 15;
pub const RTL8812_TRANS_SUS_TO_CARDEMU_STEPS: c_int = 15;
pub const RTL8812_TRANS_CARDEMU_TO_PDN_STEPS: c_int = 25;
pub const RTL8812_TRANS_PDN_TO_CARDEMU_STEPS: c_int = 15;
pub const RTL8812_TRANS_ACT_TO_LPS_STEPS: c_int = 15;
pub const RTL8812_TRANS_LPS_TO_ACT_STEPS: c_int = 15;
pub const RTL8812_TRANS_END_STEPS: c_int = 1;
// The following macros have the following format:
// { offset, cut_msk, fab_msk|interface_msk, base|cmd, msk, value
// comments },
//

// disable SW LPS 0x04[10]=0*/},	\
// wait till 0x04[17] = 1    power ready*/},	\
// disable HWPDN 0x04[15]=0*/}, \
// disable WL suspend*/},	\
// polling until return 0*/},	\

// 0xc00[7:0] = 4	turn off 3-wire */},	\
// 0xe00[7:0] = 4	turn off 3-wire */},	\
// 0x2[0] = 0	 RESET BB, CLOSE RF */},	\
// Delay 1us*/},	\
// Whole BB is reset*/},			\
// 0x07[7:0] = 0x28 sps pwm mode 0x2a for BT coex*/},	\
// 0x8[1] = 0 ANA clk =500k */},	\
// 0x04[9] = 1 turn off MAC by HW state machine*/},	\
// wait till 0x04[9] = 0 polling until return 0 to disable*/},

// gpio11 input mode, gpio10~8 output mode */},	\
// gpio 0~7 output same value as input ?? */},	\
// gpio0~7 output mode */},	\
// 0x47[7:0] = 00 gpio mode */},	\
// suspend option all off */},	\
// 0x14[7] = 1 turn on ZCD */},	\
// 0x15[0] =1 trun on ZCD */},	\
// 0x23[4] = 1 hpon LDO sleep mode */},	\
// 0x8[1] = 0 ANA clk =500k */},	\
// 0x04[11] = 2b'11 enable WL suspend for PCIe*/},

// 0x04[11] = 2b'01enable WL suspend*/},   \
// 0x23[4] = 0 hpon LDO sleep mode leave */},	\
// 0x15[0] =0 trun off ZCD */},	\
// 0x14[7] = 0 turn off ZCD */},	\
// gpio0~7 input mode */},	\
// gpio11 input mode, gpio10~8 input mode */},

// 0x03[2] = 0, reset 8051*/},	\
// 0x80=05h if reload fw, fill the default value of host_CPU handshake field*/},	\
// gpio11 input mode, gpio10~8 output mode */},	\
// gpio 0~7 output same value as input ?? */},	\
// gpio0~7 output mode */},	\
// 0x47[7:0] = 00 gpio mode */},	\
// 0x14[7] = 1 turn on ZCD */},	\
// 0x15[0] =1 trun on ZCD */},	\
// 0x12[0] = 0 force PFM mode */},	\
// 0x23[4] = 1 hpon LDO sleep mode */},	\
// 0x8[1] = 0 ANA clk =500k */},	\
// 0x07=0x20 , SOP option to disable BG/MB*/},	\
// 0x01f[1]=0 , disable RFC_0  control  REG_RF_CTRL_8812 */},	\
// 0x076[1]=0 , disable RFC_1  control REG_OPT_CTRL_8812 +2 */},	\
// 0x04[11] = 2b'01 enable WL suspend*/},

// 0x12[0] = 1 force PWM mode */},	\
// 0x14[7] = 0 turn off ZCD */},	\
// 0x15[0] =0 trun off ZCD */},	\
// 0x23[4] = 0 hpon LDO leave sleep mode */},	\
// gpio0~7 input mode */},	\
// gpio11 input mode, gpio10~8 input mode */}, \
// 0x04[10] = 0, enable SW LPS PCIE only*/},	\
// 0x04[11] = 2b'01enable WL suspend*/},	\
// 0x03[2] = 1, enable 8051*/},	\
// PCIe DMA start*/},

// 0x04[15] = 1*/},

// 0x04[15] = 0*/},

// PCIe DMA stop*/},	\
// Tx Pause*/},		\
// Should be zero if no packet is transmitting*/},	\
// 0xc00[7:0] = 4	turn off 3-wire */},	\
// 0xe00[7:0] = 4	turn off 3-wire */},	\
// CCK and OFDM are disabled,and clock are gated,and RF closed*/},	\
// Delay 1us*/},	\
// Whole BB is reset*/},			\
// Reset MAC TRX*/},			\
// check if removed later*/},		\
// Respond TxOK to scheduler*/},

// SDIO RPWM*/},	\
// USB RPWM*/},	\
// PCIe RPWM*/},	\
// Delay*/},	\
// .	0x08[4] = 0		 switch TSF to 40M*/},	\
// Polling 0x109[7]=0  TSF in 40M*/},			\
// .	0x29[7:6] = 2b'00	 enable BB clock*/},	\
// .	0x101[1] = 1*/},					\
// .	0x100[7:0] = 0xFF	 enable WMAC TRX*/},	\
// .	0x02[1:0] = 2b'11	 enable BB macro*/},	\
// .	0x522 = 0*/},

// Check document WM-20130516-JackieLau-RTL8821A_Power_Architecture-R10.vsd
// There are 6 HW Power States:
// 0: POFF--Power Off
// 1: PDN--Power Down
// 2: CARDEMU--Card Emulation
// 3: ACT--Active Mode
// 4: LPS--Low Power State
// 5: SUS--Suspend
//
// The transision from different states are defined below
// TRANS_CARDEMU_TO_ACT
// TRANS_ACT_TO_CARDEMU
// TRANS_CARDEMU_TO_SUS
// TRANS_SUS_TO_CARDEMU
// TRANS_CARDEMU_TO_PDN
// TRANS_ACT_TO_LPS
// TRANS_LPS_TO_ACT
//
// TRANS_END
//
pub const RTL8821A_TRANS_CARDEMU_TO_ACT_STEPS: c_int = 25;
pub const RTL8821A_TRANS_ACT_TO_CARDEMU_STEPS: c_int = 15;
pub const RTL8821A_TRANS_CARDEMU_TO_SUS_STEPS: c_int = 15;
pub const RTL8821A_TRANS_SUS_TO_CARDEMU_STEPS: c_int = 15;
pub const RTL8821A_TRANS_CARDDIS_TO_CARDEMU_STEPS: c_int = 15;
pub const RTL8821A_TRANS_CARDEMU_TO_PDN_STEPS: c_int = 15;
pub const RTL8821A_TRANS_PDN_TO_CARDEMU_STEPS: c_int = 15;
pub const RTL8821A_TRANS_ACT_TO_LPS_STEPS: c_int = 15;
pub const RTL8821A_TRANS_LPS_TO_ACT_STEPS: c_int = 15;
pub const RTL8821A_TRANS_END_STEPS: c_int = 1;

// 0x20[0] = 1b'1 enable LDOA12 MACRO block for all interface*/},   \
// 0x67[0] = 0 to disable BT_GPS_SEL pins*/},	\
// Delay 1ms*/},   \
// 0x00[5] = 1b'0 release analog Ips to digital ,1:isolation*/},   \
// disable SW LPS 0x04[10]=0 and WLSUS_EN 0x04[12:11]=0*/},	\
// Disable USB suspend */},	\
// wait till 0x04[17] = 1    power ready*/},	\
// Enable USB suspend */},	\
// release WLON reset  0x04[16]=1*/},	\
// disable HWPDN 0x04[15]=0*/},	\
// disable WL suspend*/},	\
// polling until return 0*/},	\
// },	\
// 0x4C[24] = 0x4F[0] = 1, switch DPDT_SEL_P output from WL BB */},\
// 0x66[13] = 0x67[5] = 1, switch for PAPE_G/PAPE_A 	\
// anapar_mac<118> , 0x25[6]=0 by wlan single function*/},\
// Enable falling edge triggering interrupt*/},\
// Enable GPIO9 interrupt mode*/},\
// Enable GPIO9 input mode*/},\
// Enable HSISR GPIO[C:0] interrupt*/},\
// Enable HSISR GPIO9 interrupt*/},\
// 0x7A = 0x3A start BT*/},\
// 0x2C[23:12]=0x820 ; XTAL trim */}, \
// 0x10[6]=1  */},

// 0x1F[7:0] = 0 turn off RF*/},	\
// 0x4C[24] = 0x4F[0] = 0, switch DPDT_SEL_P output from		\
// Enable rising edge triggering interrupt*/}, \
// 0x04[9] = 1 turn off MAC by HW state machine*/},	\
// wait till 0x04[9] = 0 polling until return 0 to disable*/},	\
// 0x00[5] = 1b'1 analog Ips to digital ,1:isolation*/},   \
// 0x20[0] = 1b'0 disable LDOA12 MACRO block*/},

// 0x04[12:11] = 2b'11 enable WL suspend for PCIe*/},	\
// 0x04[12:11] = 2b'01 enable WL suspend*/},	\
// 0x23[4] = 1b'1 12H LDO enter sleep mode*/},   \
// 0x07[7:0] = 0x20 SDIO SOP option to disable BG/MB/ACK/SWR*/},   \
// 0x04[12:11] = 2b'11 enable WL suspend for PCIe*/},	\
// Set SDIO suspend local register*/},	\
// wait power state to suspend*/},

// clear suspend enable and power down enable*/},	\
// Set SDIO suspend local register*/},	\
// wait power state to suspend*/},\
// 0x23[4] = 1b'0 12H LDO enter normal mode*/},   \
// 0x04[12:11] = 2b'00 disable WL suspend*/},

// 0x07=0x20 , SOP option to disable BG/MB*/},	\
// 0x04[12:11] = 2b'01 enable WL suspend*/},	\
// 0x04[10] = 1, enable SW LPS*/},	\
// 0x48[16] = 1 to enable GPIO9 as EXT WAKEUP*/},   \
// 0x23[4] = 1b'1 12H LDO enter sleep mode*/},   \
// Set SDIO suspend local register*/},	\
// wait power state to suspend*/},

// clear suspend enable and power down enable*/},	\
// Set SDIO suspend local register*/},	\
// wait power state to suspend*/},\
// 0x48[16] = 0 to disable GPIO9 as EXT WAKEUP*/},   \
// 0x04[12:11] = 2b'00 disable WL suspend*/},\
// 0x23[4] = 1b'0 12H LDO enter normal mode*/},   \
// PCIe DMA start*/},

// 0x23[4] = 1b'1 12H LDO enter sleep mode*/},   \
// 0x07[7:0] = 0x20 SOP option to disable BG/MB/ACK/SWR*/},   \
// 0x04[16] = 0*/},\
// 0x04[15] = 1*/},

// 0x04[15] = 0*/},

// PCIe DMA stop*/},	\
// Tx Pause*/},	\
// Should be zero if no packet is transmitting*/},	\
// CCK and OFDM are disabled,and clock are gated*/},	\
// Delay 1us*/},	\
// Whole BB is reset*/},	\
// Reset MAC TRX*/},	\
// check if removed later*/},	\
// When driver enter Sus/ Disable, enable LOP for BT*/},	\
// Respond TxOK to scheduler*/},

// SDIO RPWM*/},\
// USB RPWM*/},\
// PCIe RPWM*/},\
// Delay*/},\
// .	0x08[4] = 0		 switch TSF to 40M*/},\
// Polling 0x109[7]=0  TSF in 40M*/},\
// .	0x29[7:6] = 2b'00	 enable BB clock*/},\
// .	0x101[1] = 1*/},\
// .	0x100[7:0] = 0xFF	 enable WMAC TRX*/},\
// .	0x02[1:0] = 2b'11	 enable BB macro*/},\
// .	0x522 = 0*/},

// RTL8812 Power Configuration CMDs for PCIe interface

// RTL8821 Power Configuration CMDs for PCIe interface

