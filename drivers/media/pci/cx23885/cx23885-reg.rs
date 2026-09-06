//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx23885/cx23885-reg.h
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
// Driver for the Conexant CX23885 PCIe bridge
//
// Copyright (c) 2006 Steven Toth <stoth@linuxtv.org>
//
// Risc Instructions
pub const RISC_CNT_INC: c_uint = 0x00010000;
pub const RISC_CNT_RESET: c_uint = 0x00030000;
pub const RISC_IRQ1: c_uint = 0x01000000;
pub const RISC_IRQ2: c_uint = 0x02000000;
pub const RISC_EOL: c_uint = 0x04000000;
pub const RISC_SOL: c_uint = 0x08000000;
pub const RISC_WRITE: c_uint = 0x10000000;
pub const RISC_SKIP: c_uint = 0x20000000;
pub const RISC_JUMP: c_uint = 0x70000000;
pub const RISC_SYNC: c_uint = 0x80000000;
pub const RISC_RESYNC: c_uint = 0x80008000;
pub const RISC_READ: c_uint = 0x90000000;
pub const RISC_WRITERM: c_uint = 0xB0000000;
pub const RISC_WRITECM: c_uint = 0xC0000000;
pub const RISC_WRITECR: c_uint = 0xD0000000;
pub const RISC_WRITEC: c_uint = 0x50000000;
pub const RISC_READC: c_uint = 0xA0000000;
// Audio and Video Core
pub const HOST_REG1: c_uint = 0x00000000;
pub const HOST_REG2: c_uint = 0x00000001;
pub const HOST_REG3: c_uint = 0x00000002;
// Chip Configuration Registers
pub const CHIP_CTRL: c_uint = 0x00000100;
pub const AFE_CTRL: c_uint = 0x00000104;
pub const VID_PLL_INT_POST: c_uint = 0x00000108;
pub const VID_PLL_FRAC: c_uint = 0x0000010C;
pub const AUX_PLL_INT_POST: c_uint = 0x00000110;
pub const AUX_PLL_FRAC: c_uint = 0x00000114;
pub const SYS_PLL_INT_POST: c_uint = 0x00000118;
pub const SYS_PLL_FRAC: c_uint = 0x0000011C;
pub const PIN_CTRL: c_uint = 0x00000120;
pub const AUD_IO_CTRL: c_uint = 0x00000124;
pub const AUD_LOCK1: c_uint = 0x00000128;
pub const AUD_LOCK2: c_uint = 0x0000012C;
pub const POWER_CTRL: c_uint = 0x00000130;
pub const AFE_DIAG_CTRL1: c_uint = 0x00000134;
pub const AFE_DIAG_CTRL3: c_uint = 0x0000013C;
pub const PLL_DIAG_CTRL: c_uint = 0x00000140;
pub const AFE_CLK_OUT_CTRL: c_uint = 0x00000144;
pub const DLL1_DIAG_CTRL: c_uint = 0x0000015C;
// GPIO[23:19] Output Enable
pub const GPIO2_OUT_EN_REG: c_uint = 0x00000160;
// GPIO[23:19] Data Registers
pub const GPIO2: c_uint = 0x00000164;
pub const IFADC_CTRL: c_uint = 0x00000180;
// Infrared Remote Registers
pub const IR_CNTRL_REG: c_uint = 0x00000200;
pub const IR_TXCLK_REG: c_uint = 0x00000204;
pub const IR_RXCLK_REG: c_uint = 0x00000208;
pub const IR_CDUTY_REG: c_uint = 0x0000020C;
pub const IR_STAT_REG: c_uint = 0x00000210;
pub const IR_IRQEN_REG: c_uint = 0x00000214;
pub const IR_FILTR_REG: c_uint = 0x00000218;
pub const IR_FIFO_REG: c_uint = 0x0000023C;
// Video Decoder Registers
pub const MODE_CTRL: c_uint = 0x00000400;
pub const OUT_CTRL1: c_uint = 0x00000404;
pub const OUT_CTRL2: c_uint = 0x00000408;
pub const GEN_STAT: c_uint = 0x0000040C;
pub const INT_STAT_MASK: c_uint = 0x00000410;
pub const LUMA_CTRL: c_uint = 0x00000414;
pub const HSCALE_CTRL: c_uint = 0x00000418;
pub const VSCALE_CTRL: c_uint = 0x0000041C;
pub const CHROMA_CTRL: c_uint = 0x00000420;
pub const VBI_LINE_CTRL1: c_uint = 0x00000424;
pub const VBI_LINE_CTRL2: c_uint = 0x00000428;
pub const VBI_LINE_CTRL3: c_uint = 0x0000042C;
pub const VBI_LINE_CTRL4: c_uint = 0x00000430;
pub const VBI_LINE_CTRL5: c_uint = 0x00000434;
pub const VBI_FC_CFG: c_uint = 0x00000438;
pub const VBI_MISC_CFG1: c_uint = 0x0000043C;
pub const VBI_MISC_CFG2: c_uint = 0x00000440;
pub const VBI_PAY1: c_uint = 0x00000444;
pub const VBI_PAY2: c_uint = 0x00000448;
pub const VBI_CUST1_CFG1: c_uint = 0x0000044C;
pub const VBI_CUST1_CFG2: c_uint = 0x00000450;
pub const VBI_CUST1_CFG3: c_uint = 0x00000454;
pub const VBI_CUST2_CFG1: c_uint = 0x00000458;
pub const VBI_CUST2_CFG2: c_uint = 0x0000045C;
pub const VBI_CUST2_CFG3: c_uint = 0x00000460;
pub const VBI_CUST3_CFG1: c_uint = 0x00000464;
pub const VBI_CUST3_CFG2: c_uint = 0x00000468;
pub const VBI_CUST3_CFG3: c_uint = 0x0000046C;
pub const HORIZ_TIM_CTRL: c_uint = 0x00000470;
pub const VERT_TIM_CTRL: c_uint = 0x00000474;
pub const SRC_COMB_CFG: c_uint = 0x00000478;
pub const CHROMA_VBIOFF_CFG: c_uint = 0x0000047C;
pub const FIELD_COUNT: c_uint = 0x00000480;
pub const MISC_TIM_CTRL: c_uint = 0x00000484;
pub const DFE_CTRL1: c_uint = 0x00000488;
pub const DFE_CTRL2: c_uint = 0x0000048C;
pub const DFE_CTRL3: c_uint = 0x00000490;
pub const PLL_CTRL: c_uint = 0x00000494;
pub const HTL_CTRL: c_uint = 0x00000498;
pub const COMB_CTRL: c_uint = 0x0000049C;
pub const CRUSH_CTRL: c_uint = 0x000004A0;
pub const SOFT_RST_CTRL: c_uint = 0x000004A4;
pub const CX885_VERSION: c_uint = 0x000004B4;
pub const VBI_PASS_CTRL: c_uint = 0x000004BC;
// Audio Decoder Registers
// 8051 Configuration
pub const DL_CTL: c_uint = 0x00000800;
pub const STD_DET_STATUS: c_uint = 0x00000804;
pub const STD_DET_CTL: c_uint = 0x00000808;
pub const DW8051_INT: c_uint = 0x0000080C;
pub const GENERAL_CTL: c_uint = 0x00000810;
pub const AAGC_CTL: c_uint = 0x00000814;
pub const DEMATRIX_CTL: c_uint = 0x000008CC;
pub const PATH1_CTL1: c_uint = 0x000008D0;
pub const PATH1_VOL_CTL: c_uint = 0x000008D4;
pub const PATH1_EQ_CTL: c_uint = 0x000008D8;
pub const PATH1_SC_CTL: c_uint = 0x000008DC;
pub const PATH2_CTL1: c_uint = 0x000008E0;
pub const PATH2_VOL_CTL: c_uint = 0x000008E4;
pub const PATH2_EQ_CTL: c_uint = 0x000008E8;
pub const PATH2_SC_CTL: c_uint = 0x000008EC;
// Sample Rate Converter
pub const SRC_CTL: c_uint = 0x000008F0;
pub const SRC_LF_COEF: c_uint = 0x000008F4;
pub const SRC1_CTL: c_uint = 0x000008F8;
pub const SRC2_CTL: c_uint = 0x000008FC;
pub const SRC3_CTL: c_uint = 0x00000900;
pub const SRC4_CTL: c_uint = 0x00000904;
pub const SRC5_CTL: c_uint = 0x00000908;
pub const SRC6_CTL: c_uint = 0x0000090C;
pub const BAND_OUT_SEL: c_uint = 0x00000910;
pub const I2S_N_CTL: c_uint = 0x00000914;
pub const I2S_OUT_CTL: c_uint = 0x00000918;
pub const AUTOCONFIG_REG: c_uint = 0x000009C4;
// Audio ADC Registers
pub const DSM_CTRL1: c_uint = 0x00000000;
pub const DSM_CTRL2: c_uint = 0x00000001;
pub const CHP_EN_CTRL: c_uint = 0x00000002;
pub const CHP_CLK_CTRL1: c_uint = 0x00000004;
pub const CHP_CLK_CTRL2: c_uint = 0x00000005;
pub const BG_REF_CTRL: c_uint = 0x00000006;
pub const SD2_SW_CTRL1: c_uint = 0x00000008;
pub const SD2_SW_CTRL2: c_uint = 0x00000009;
pub const SD2_BIAS_CTRL: c_uint = 0x0000000A;
pub const AMP_BIAS_CTRL: c_uint = 0x0000000C;
pub const CH_PWR_CTRL1: c_uint = 0x0000000E;

pub const CH_PWR_CTRL2: c_uint = 0x0000000F;
pub const DSM_STATUS1: c_uint = 0x00000010;
pub const DSM_STATUS2: c_uint = 0x00000011;
pub const DIG_CTL1: c_uint = 0x00000012;
pub const DIG_CTL2: c_uint = 0x00000013;
pub const I2S_TX_CFG: c_uint = 0x0000001A;
pub const DEV_CNTRL2: c_uint = 0x00040000;

pub const PCI_MSK_VID_A: c_int = 1;
pub const PCI_INT_MSK: c_uint = 0x00040010;
pub const PCI_INT_STAT: c_uint = 0x00040014;
pub const PCI_INT_MSTAT: c_uint = 0x00040018;
pub const VID_A_INT_MSK: c_uint = 0x00040020;
pub const VID_A_INT_STAT: c_uint = 0x00040024;
pub const VID_A_INT_MSTAT: c_uint = 0x00040028;
pub const VID_A_INT_SSTAT: c_uint = 0x0004002C;
pub const VID_B_INT_MSK: c_uint = 0x00040030;

pub const VID_B_MSK_RISCI1: c_int = 1;
pub const VID_B_INT_STAT: c_uint = 0x00040034;
pub const VID_B_INT_MSTAT: c_uint = 0x00040038;
pub const VID_B_INT_SSTAT: c_uint = 0x0004003C;

pub const VID_B_MSK_RISCI1: c_int = 1;

pub const VID_C_MSK_RISCI1: c_int = 1;
// A superset for testing purposes

pub const VID_BC_MSK_RISCI1: c_int = 1;
pub const VID_C_INT_MSK: c_uint = 0x00040040;
pub const VID_C_INT_STAT: c_uint = 0x00040044;
pub const VID_C_INT_MSTAT: c_uint = 0x00040048;
pub const VID_C_INT_SSTAT: c_uint = 0x0004004C;
pub const AUDIO_INT_INT_MSK: c_uint = 0x00040050;
pub const AUDIO_INT_INT_STAT: c_uint = 0x00040054;
pub const AUDIO_INT_INT_MSTAT: c_uint = 0x00040058;
pub const AUDIO_INT_INT_SSTAT: c_uint = 0x0004005C;
pub const AUDIO_EXT_INT_MSK: c_uint = 0x00040060;
pub const AUDIO_EXT_INT_STAT: c_uint = 0x00040064;
pub const AUDIO_EXT_INT_MSTAT: c_uint = 0x00040068;
pub const AUDIO_EXT_INT_SSTAT: c_uint = 0x0004006C;
// Bits [7:0] set in both TC_REQ and TC_REQ_SET
// indicate a stall in the RISC engine for a
// particular rider traffic class. This causes
// the 885 and 888 bridges (unknown about 887)
// to become inoperable. Setting bits in
// TC_REQ_SET resets the corresponding bits
// in TC_REQ (and TC_REQ_SET) allowing
// operation to continue.
//
pub const TC_REQ: c_uint = 0x00040090;
pub const TC_REQ_SET: c_uint = 0x00040094;
pub const RDR_CFG0: c_uint = 0x00050000;
pub const RDR_CFG1: c_uint = 0x00050004;
pub const RDR_CFG2: c_uint = 0x00050008;
pub const RDR_RDRCTL1: c_uint = 0x0005030c;
pub const RDR_TLCTL0: c_uint = 0x00050318;
// APB DMAC Current Buffer Pointer
pub const DMA1_PTR1: c_uint = 0x00100000;
pub const DMA2_PTR1: c_uint = 0x00100004;
pub const DMA3_PTR1: c_uint = 0x00100008;
pub const DMA4_PTR1: c_uint = 0x0010000C;
pub const DMA5_PTR1: c_uint = 0x00100010;
pub const DMA6_PTR1: c_uint = 0x00100014;
pub const DMA7_PTR1: c_uint = 0x00100018;
pub const DMA8_PTR1: c_uint = 0x0010001C;
// APB DMAC Current Table Pointer
pub const DMA1_PTR2: c_uint = 0x00100040;
pub const DMA2_PTR2: c_uint = 0x00100044;
pub const DMA3_PTR2: c_uint = 0x00100048;
pub const DMA4_PTR2: c_uint = 0x0010004C;
pub const DMA5_PTR2: c_uint = 0x00100050;
pub const DMA6_PTR2: c_uint = 0x00100054;
pub const DMA7_PTR2: c_uint = 0x00100058;
pub const DMA8_PTR2: c_uint = 0x0010005C;
// APB DMAC Buffer Limit
pub const DMA1_CNT1: c_uint = 0x00100080;
pub const DMA2_CNT1: c_uint = 0x00100084;
pub const DMA3_CNT1: c_uint = 0x00100088;
pub const DMA4_CNT1: c_uint = 0x0010008C;
pub const DMA5_CNT1: c_uint = 0x00100090;
pub const DMA6_CNT1: c_uint = 0x00100094;
pub const DMA7_CNT1: c_uint = 0x00100098;
pub const DMA8_CNT1: c_uint = 0x0010009C;
// APB DMAC Table Size
pub const DMA1_CNT2: c_uint = 0x001000C0;
pub const DMA2_CNT2: c_uint = 0x001000C4;
pub const DMA3_CNT2: c_uint = 0x001000C8;
pub const DMA4_CNT2: c_uint = 0x001000CC;
pub const DMA5_CNT2: c_uint = 0x001000D0;
pub const DMA6_CNT2: c_uint = 0x001000D4;
pub const DMA7_CNT2: c_uint = 0x001000D8;
pub const DMA8_CNT2: c_uint = 0x001000DC;
// Timer Counters
pub const TM_CNT_LDW: c_uint = 0x00110000;
pub const TM_CNT_UW: c_uint = 0x00110004;
pub const TM_LMT_LDW: c_uint = 0x00110008;
pub const TM_LMT_UW: c_uint = 0x0011000C;
// GPIO
pub const GP0_IO: c_uint = 0x00110010;
pub const GPIO_ISM: c_uint = 0x00110014;
pub const SOFT_RESET: c_uint = 0x0011001C;
// GPIO (417 Microsoftcontroller) RW Data
pub const MC417_RWD: c_uint = 0x00110020;
// GPIO (417 Microsoftcontroller) Output Enable, Low Active
pub const MC417_OEN: c_uint = 0x00110024;
pub const MC417_CTL: c_uint = 0x00110028;
pub const ALT_PIN_OUT_SEL: c_uint = 0x0011002C;
pub const CLK_DELAY: c_uint = 0x00110048;
pub const PAD_CTRL: c_uint = 0x0011004C;
// Video A Interface
pub const VID_A_GPCNT: c_uint = 0x00130020;
pub const VBI_A_GPCNT: c_uint = 0x00130024;
pub const VID_A_GPCNT_CTL: c_uint = 0x00130030;
pub const VBI_A_GPCNT_CTL: c_uint = 0x00130034;
pub const VID_A_DMA_CTL: c_uint = 0x00130040;
pub const VID_A_VIP_CTRL: c_uint = 0x00130080;
pub const VID_A_PIXEL_FRMT: c_uint = 0x00130084;
pub const VID_A_VBI_CTRL: c_uint = 0x00130088;
// Video B Interface
pub const VID_B_DMA: c_uint = 0x00130100;
pub const VBI_B_DMA: c_uint = 0x00130108;
pub const VID_B_GPCNT: c_uint = 0x00130120;
pub const VBI_B_GPCNT: c_uint = 0x00130124;
pub const VID_B_GPCNT_CTL: c_uint = 0x00130134;
pub const VBI_B_GPCNT_CTL: c_uint = 0x00130138;
pub const VID_B_DMA_CTL: c_uint = 0x00130140;
pub const VID_B_SRC_SEL: c_uint = 0x00130144;
pub const VID_B_LNGTH: c_uint = 0x00130150;
pub const VID_B_HW_SOP_CTL: c_uint = 0x00130154;
pub const VID_B_GEN_CTL: c_uint = 0x00130158;
pub const VID_B_BD_PKT_STATUS: c_uint = 0x0013015C;
pub const VID_B_SOP_STATUS: c_uint = 0x00130160;
pub const VID_B_FIFO_OVFL_STAT: c_uint = 0x00130164;
pub const VID_B_VLD_MISC: c_uint = 0x00130168;
pub const VID_B_TS_CLK_EN: c_uint = 0x0013016C;
pub const VID_B_VIP_CTRL: c_uint = 0x00130180;
pub const VID_B_PIXEL_FRMT: c_uint = 0x00130184;
// Video C Interface
pub const VID_C_DMA: c_uint = 0x00130200;
pub const VBI_C_DMA: c_uint = 0x00130208;
pub const VID_C_GPCNT: c_uint = 0x00130220;
pub const VID_C_GPCNT_CTL: c_uint = 0x00130230;
pub const VBI_C_GPCNT_CTL: c_uint = 0x00130234;
pub const VID_C_DMA_CTL: c_uint = 0x00130240;
pub const VID_C_LNGTH: c_uint = 0x00130250;
pub const VID_C_HW_SOP_CTL: c_uint = 0x00130254;
pub const VID_C_GEN_CTL: c_uint = 0x00130258;
pub const VID_C_BD_PKT_STATUS: c_uint = 0x0013025C;
pub const VID_C_SOP_STATUS: c_uint = 0x00130260;
pub const VID_C_FIFO_OVFL_STAT: c_uint = 0x00130264;
pub const VID_C_VLD_MISC: c_uint = 0x00130268;
pub const VID_C_TS_CLK_EN: c_uint = 0x0013026C;
// Internal Audio Interface
pub const AUD_INT_A_GPCNT: c_uint = 0x00140020;
pub const AUD_INT_B_GPCNT: c_uint = 0x00140024;
pub const AUD_INT_A_GPCNT_CTL: c_uint = 0x00140030;
pub const AUD_INT_B_GPCNT_CTL: c_uint = 0x00140034;
pub const AUD_INT_DMA_CTL: c_uint = 0x00140040;
pub const AUD_INT_A_LNGTH: c_uint = 0x00140050;
pub const AUD_INT_B_LNGTH: c_uint = 0x00140054;
pub const AUD_INT_A_MODE: c_uint = 0x00140058;
pub const AUD_INT_B_MODE: c_uint = 0x0014005C;
// External Audio Interface
pub const AUD_EXT_DMA: c_uint = 0x00140100;
pub const AUD_EXT_GPCNT: c_uint = 0x00140120;
pub const AUD_EXT_GPCNT_CTL: c_uint = 0x00140130;
pub const AUD_EXT_DMA_CTL: c_uint = 0x00140140;
pub const AUD_EXT_LNGTH: c_uint = 0x00140150;
pub const AUD_EXT_A_MODE: c_uint = 0x00140158;
// I2C Bus 1
pub const I2C1_ADDR: c_uint = 0x00180000;
pub const I2C1_WDATA: c_uint = 0x00180004;
pub const I2C1_CTRL: c_uint = 0x00180008;
pub const I2C1_RDATA: c_uint = 0x0018000C;
pub const I2C1_STAT: c_uint = 0x00180010;
// I2C Bus 2
pub const I2C2_ADDR: c_uint = 0x00190000;
pub const I2C2_WDATA: c_uint = 0x00190004;
pub const I2C2_CTRL: c_uint = 0x00190008;
pub const I2C2_RDATA: c_uint = 0x0019000C;
pub const I2C2_STAT: c_uint = 0x00190010;
// I2C Bus 3
pub const I2C3_ADDR: c_uint = 0x001A0000;
pub const I2C3_WDATA: c_uint = 0x001A0004;
pub const I2C3_CTRL: c_uint = 0x001A0008;
pub const I2C3_RDATA: c_uint = 0x001A000C;
pub const I2C3_STAT: c_uint = 0x001A0010;
// UART
pub const UART_CTL: c_uint = 0x001B0000;
pub const UART_BRD: c_uint = 0x001B0004;
pub const UART_ISR: c_uint = 0x001B000C;
pub const UART_CNT: c_uint = 0x001B0010;
