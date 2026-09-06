//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx88/cx88-reg.h
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
// cx88x-hw.h - CX2388x register offsets
//
// Copyright (C) 1996,97,98 Ralph Metzler (rjkm@thp.uni-koeln.de)
// 2001 Michael Eskin
// 2002 Yurij Sysoev <yurij@naturesoft.net>
// 2003 Gerd Knorr <kraxel@bytesex.org>
//
// PCI IDs and config space
//

pub const CX88X_DEVCTRL: c_uint = 0x40;
pub const CX88X_EN_TBFX: c_uint = 0x02;
pub const CX88X_EN_VSFX: c_uint = 0x04;
//
// PCI controller registers
//
// Command and Status Register
pub const F0_CMD_STAT_MM: c_uint = 0x2f0004;
pub const F1_CMD_STAT_MM: c_uint = 0x2f0104;
pub const F2_CMD_STAT_MM: c_uint = 0x2f0204;
pub const F3_CMD_STAT_MM: c_uint = 0x2f0304;
pub const F4_CMD_STAT_MM: c_uint = 0x2f0404;
// Device Control #1
pub const F0_DEV_CNTRL1_MM: c_uint = 0x2f0040;
pub const F1_DEV_CNTRL1_MM: c_uint = 0x2f0140;
pub const F2_DEV_CNTRL1_MM: c_uint = 0x2f0240;
pub const F3_DEV_CNTRL1_MM: c_uint = 0x2f0340;
pub const F4_DEV_CNTRL1_MM: c_uint = 0x2f0440;
// Device Control #1
pub const F0_BAR0_MM: c_uint = 0x2f0010;
pub const F1_BAR0_MM: c_uint = 0x2f0110;
pub const F2_BAR0_MM: c_uint = 0x2f0210;
pub const F3_BAR0_MM: c_uint = 0x2f0310;
pub const F4_BAR0_MM: c_uint = 0x2f0410;
//
// DMA Controller registers
//
pub const MO_PDMA_STHRSH: c_uint = 0x200000 // Source threshold;
pub const MO_PDMA_STADRS: c_uint = 0x200004 // Source target address;
pub const MO_PDMA_SIADRS: c_uint = 0x200008 // Source internal address;
pub const MO_PDMA_SCNTRL: c_uint = 0x20000C // Source control;
pub const MO_PDMA_DTHRSH: c_uint = 0x200010 // Destination threshold;
pub const MO_PDMA_DTADRS: c_uint = 0x200014 // Destination target address;
pub const MO_PDMA_DIADRS: c_uint = 0x200018 // Destination internal address;
pub const MO_PDMA_DCNTRL: c_uint = 0x20001C // Destination control;
pub const MO_LD_SSID: c_uint = 0x200030 // Load subsystem ID;
pub const MO_DEV_CNTRL2: c_uint = 0x200034 // Device control;
pub const MO_PCI_INTMSK: c_uint = 0x200040 // PCI interrupt mask;
pub const MO_PCI_INTSTAT: c_uint = 0x200044 // PCI interrupt status;
pub const MO_PCI_INTMSTAT: c_uint = 0x200048 // PCI interrupt masked status;
pub const MO_VID_INTMSK: c_uint = 0x200050 // Video interrupt mask;
pub const MO_VID_INTSTAT: c_uint = 0x200054 // Video interrupt status;
pub const MO_VID_INTMSTAT: c_uint = 0x200058 // Video interrupt masked status;
pub const MO_VID_INTSSTAT: c_uint = 0x20005C // Video interrupt set status;
pub const MO_AUD_INTMSK: c_uint = 0x200060 // Audio interrupt mask;
pub const MO_AUD_INTSTAT: c_uint = 0x200064 // Audio interrupt status;
pub const MO_AUD_INTMSTAT: c_uint = 0x200068 // Audio interrupt masked status;
pub const MO_AUD_INTSSTAT: c_uint = 0x20006C // Audio interrupt set status;
pub const MO_TS_INTMSK: c_uint = 0x200070 // Transport stream interrupt mask;
pub const MO_TS_INTSTAT: c_uint = 0x200074 // Transport stream interrupt status;
pub const MO_TS_INTMSTAT: c_uint = 0x200078 // Transport stream interrupt mask status;
pub const MO_TS_INTSSTAT: c_uint = 0x20007C // Transport stream interrupt set status;
pub const MO_VIP_INTMSK: c_uint = 0x200080 // VIP interrupt mask;
pub const MO_VIP_INTSTAT: c_uint = 0x200084 // VIP interrupt status;
pub const MO_VIP_INTMSTAT: c_uint = 0x200088 // VIP interrupt masked status;
pub const MO_VIP_INTSSTAT: c_uint = 0x20008C // VIP interrupt set status;
pub const MO_GPHST_INTMSK: c_uint = 0x200090 // Host interrupt mask;
pub const MO_GPHST_INTSTAT: c_uint = 0x200094 // Host interrupt status;
pub const MO_GPHST_INTMSTAT: c_uint = 0x200098 // Host interrupt masked status;
pub const MO_GPHST_INTSSTAT: c_uint = 0x20009C // Host interrupt set status;
// DMA Channels 1-6 belong to SPIPE
pub const MO_DMA7_PTR1: c_uint = 0x300018 // {24}RW* DMA Current Ptr : Ch#7;
pub const MO_DMA8_PTR1: c_uint = 0x30001C // {24}RW* DMA Current Ptr : Ch#8;
// DMA Channels 9-20 belong to SPIPE
pub const MO_DMA21_PTR1: c_uint = 0x300080 // {24}R0* DMA Current Ptr : Ch#21;
pub const MO_DMA22_PTR1: c_uint = 0x300084 // {24}R0* DMA Current Ptr : Ch#22;
pub const MO_DMA23_PTR1: c_uint = 0x300088 // {24}R0* DMA Current Ptr : Ch#23;
pub const MO_DMA24_PTR1: c_uint = 0x30008C // {24}R0* DMA Current Ptr : Ch#24;
pub const MO_DMA25_PTR1: c_uint = 0x300090 // {24}R0* DMA Current Ptr : Ch#25;
pub const MO_DMA26_PTR1: c_uint = 0x300094 // {24}R0* DMA Current Ptr : Ch#26;
pub const MO_DMA27_PTR1: c_uint = 0x300098 // {24}R0* DMA Current Ptr : Ch#27;
pub const MO_DMA28_PTR1: c_uint = 0x30009C // {24}R0* DMA Current Ptr : Ch#28;
pub const MO_DMA29_PTR1: c_uint = 0x3000A0 // {24}R0* DMA Current Ptr : Ch#29;
pub const MO_DMA30_PTR1: c_uint = 0x3000A4 // {24}R0* DMA Current Ptr : Ch#30;
pub const MO_DMA31_PTR1: c_uint = 0x3000A8 // {24}R0* DMA Current Ptr : Ch#31;
pub const MO_DMA32_PTR1: c_uint = 0x3000AC // {24}R0* DMA Current Ptr : Ch#32;
pub const MO_DMA21_PTR2: c_uint = 0x3000C0 // {24}RW* DMA Tab Ptr : Ch#21;
pub const MO_DMA22_PTR2: c_uint = 0x3000C4 // {24}RW* DMA Tab Ptr : Ch#22;
pub const MO_DMA23_PTR2: c_uint = 0x3000C8 // {24}RW* DMA Tab Ptr : Ch#23;
pub const MO_DMA24_PTR2: c_uint = 0x3000CC // {24}RW* DMA Tab Ptr : Ch#24;
pub const MO_DMA25_PTR2: c_uint = 0x3000D0 // {24}RW* DMA Tab Ptr : Ch#25;
pub const MO_DMA26_PTR2: c_uint = 0x3000D4 // {24}RW* DMA Tab Ptr : Ch#26;
pub const MO_DMA27_PTR2: c_uint = 0x3000D8 // {24}RW* DMA Tab Ptr : Ch#27;
pub const MO_DMA28_PTR2: c_uint = 0x3000DC // {24}RW* DMA Tab Ptr : Ch#28;
pub const MO_DMA29_PTR2: c_uint = 0x3000E0 // {24}RW* DMA Tab Ptr : Ch#29;
pub const MO_DMA30_PTR2: c_uint = 0x3000E4 // {24}RW* DMA Tab Ptr : Ch#30;
pub const MO_DMA31_PTR2: c_uint = 0x3000E8 // {24}RW* DMA Tab Ptr : Ch#31;
pub const MO_DMA32_PTR2: c_uint = 0x3000EC // {24}RW* DMA Tab Ptr : Ch#32;
pub const MO_DMA21_CNT1: c_uint = 0x300100 // {11}RW* DMA Buffer Size : Ch#21;
pub const MO_DMA22_CNT1: c_uint = 0x300104 // {11}RW* DMA Buffer Size : Ch#22;
pub const MO_DMA23_CNT1: c_uint = 0x300108 // {11}RW* DMA Buffer Size : Ch#23;
pub const MO_DMA24_CNT1: c_uint = 0x30010C // {11}RW* DMA Buffer Size : Ch#24;
pub const MO_DMA25_CNT1: c_uint = 0x300110 // {11}RW* DMA Buffer Size : Ch#25;
pub const MO_DMA26_CNT1: c_uint = 0x300114 // {11}RW* DMA Buffer Size : Ch#26;
pub const MO_DMA27_CNT1: c_uint = 0x300118 // {11}RW* DMA Buffer Size : Ch#27;
pub const MO_DMA28_CNT1: c_uint = 0x30011C // {11}RW* DMA Buffer Size : Ch#28;
pub const MO_DMA29_CNT1: c_uint = 0x300120 // {11}RW* DMA Buffer Size : Ch#29;
pub const MO_DMA30_CNT1: c_uint = 0x300124 // {11}RW* DMA Buffer Size : Ch#30;
pub const MO_DMA31_CNT1: c_uint = 0x300128 // {11}RW* DMA Buffer Size : Ch#31;
pub const MO_DMA32_CNT1: c_uint = 0x30012C // {11}RW* DMA Buffer Size : Ch#32;
pub const MO_DMA21_CNT2: c_uint = 0x300140 // {11}RW* DMA Table Size : Ch#21;
pub const MO_DMA22_CNT2: c_uint = 0x300144 // {11}RW* DMA Table Size : Ch#22;
pub const MO_DMA23_CNT2: c_uint = 0x300148 // {11}RW* DMA Table Size : Ch#23;
pub const MO_DMA24_CNT2: c_uint = 0x30014C // {11}RW* DMA Table Size : Ch#24;
pub const MO_DMA25_CNT2: c_uint = 0x300150 // {11}RW* DMA Table Size : Ch#25;
pub const MO_DMA26_CNT2: c_uint = 0x300154 // {11}RW* DMA Table Size : Ch#26;
pub const MO_DMA27_CNT2: c_uint = 0x300158 // {11}RW* DMA Table Size : Ch#27;
pub const MO_DMA28_CNT2: c_uint = 0x30015C // {11}RW* DMA Table Size : Ch#28;
pub const MO_DMA29_CNT2: c_uint = 0x300160 // {11}RW* DMA Table Size : Ch#29;
pub const MO_DMA30_CNT2: c_uint = 0x300164 // {11}RW* DMA Table Size : Ch#30;
pub const MO_DMA31_CNT2: c_uint = 0x300168 // {11}RW* DMA Table Size : Ch#31;
pub const MO_DMA32_CNT2: c_uint = 0x30016C // {11}RW* DMA Table Size : Ch#32;
//
// Video registers
//
pub const MO_VIDY_DMA: c_uint = 0x310000 // {64}RWp Video Y;
pub const MO_VIDU_DMA: c_uint = 0x310008 // {64}RWp Video U;
pub const MO_VIDV_DMA: c_uint = 0x310010 // {64}RWp Video V;
pub const MO_VBI_DMA: c_uint = 0x310018 // {64}RWp VBI (Vertical blanking interval);
pub const MO_DEVICE_STATUS: c_uint = 0x310100;
pub const MO_INPUT_FORMAT: c_uint = 0x310104;
pub const MO_AGC_BURST: c_uint = 0x31010c;
pub const MO_CONTR_BRIGHT: c_uint = 0x310110;
pub const MO_UV_SATURATION: c_uint = 0x310114;
pub const MO_HUE: c_uint = 0x310118;
pub const MO_HTOTAL: c_uint = 0x310120;
pub const MO_HDELAY_EVEN: c_uint = 0x310124;
pub const MO_HDELAY_ODD: c_uint = 0x310128;
pub const MO_VDELAY_ODD: c_uint = 0x31012c;
pub const MO_VDELAY_EVEN: c_uint = 0x310130;
pub const MO_HACTIVE_EVEN: c_uint = 0x31013c;
pub const MO_HACTIVE_ODD: c_uint = 0x310140;
pub const MO_VACTIVE_EVEN: c_uint = 0x310144;
pub const MO_VACTIVE_ODD: c_uint = 0x310148;
pub const MO_HSCALE_EVEN: c_uint = 0x31014c;
pub const MO_HSCALE_ODD: c_uint = 0x310150;
pub const MO_VSCALE_EVEN: c_uint = 0x310154;
pub const MO_FILTER_EVEN: c_uint = 0x31015c;
pub const MO_VSCALE_ODD: c_uint = 0x310158;
pub const MO_FILTER_ODD: c_uint = 0x310160;
pub const MO_OUTPUT_FORMAT: c_uint = 0x310164;
pub const MO_PLL_REG: c_uint = 0x310168 // PLL register;
pub const MO_PLL_ADJ_CTRL: c_uint = 0x31016c // PLL adjust control register;
pub const MO_SCONV_REG: c_uint = 0x310170 // sample rate conversion register;
pub const MO_SCONV_FIFO: c_uint = 0x310174 // sample rate conversion fifo;
pub const MO_SUB_STEP: c_uint = 0x310178 // subcarrier step size;
pub const MO_SUB_STEP_DR: c_uint = 0x31017c // subcarrier step size for DR line;
pub const MO_CAPTURE_CTRL: c_uint = 0x310180 // capture control;
pub const MO_COLOR_CTRL: c_uint = 0x310184;
pub const MO_VBI_PACKET: c_uint = 0x310188 // vbi packet size / delay;
pub const MO_FIELD_COUNT: c_uint = 0x310190 // field counter;
pub const MO_VIP_CONFIG: c_uint = 0x310194;
pub const MO_VBOS_CONTROL: c_uint = 0x3101a8;
pub const MO_AGC_BACK_VBI: c_uint = 0x310200;
pub const MO_AGC_SYNC_TIP1: c_uint = 0x310208;
pub const MO_VIDY_GPCNT: c_uint = 0x31C020 // {16}RO Video Y general purpose counter;
pub const MO_VIDU_GPCNT: c_uint = 0x31C024 // {16}RO Video U general purpose counter;
pub const MO_VIDV_GPCNT: c_uint = 0x31C028 // {16}RO Video V general purpose counter;
pub const MO_VBI_GPCNT: c_uint = 0x31C02C // {16}RO VBI general purpose counter;
pub const MO_VIDY_GPCNTRL: c_uint = 0x31C030 // {2}WO Video Y general purpose control;
pub const MO_VIDU_GPCNTRL: c_uint = 0x31C034 // {2}WO Video U general purpose control;
pub const MO_VIDV_GPCNTRL: c_uint = 0x31C038 // {2}WO Video V general purpose control;
pub const MO_VBI_GPCNTRL: c_uint = 0x31C03C // {2}WO VBI general purpose counter;
pub const MO_VID_DMACNTRL: c_uint = 0x31C040 // {8}RW Video DMA control;
pub const MO_VID_XFR_STAT: c_uint = 0x31C044 // {1}RO Video transfer status;
//
// audio registers
//
pub const MO_AUDD_DMA: c_uint = 0x320000 // {64}RWp Audio downstream;
pub const MO_AUDU_DMA: c_uint = 0x320008 // {64}RWp Audio upstream;
pub const MO_AUDR_DMA: c_uint = 0x320010 // {64}RWp Audio RDS (downstream);
pub const MO_AUDD_GPCNT: c_uint = 0x32C020 // {16}RO Audio down general purpose counter;
pub const MO_AUDU_GPCNT: c_uint = 0x32C024 // {16}RO Audio up general purpose counter;
pub const MO_AUDR_GPCNT: c_uint = 0x32C028 // {16}RO Audio RDS general purpose counter;
pub const MO_AUDD_GPCNTRL: c_uint = 0x32C030 // {2}WO Audio down general purpose control;
pub const MO_AUDU_GPCNTRL: c_uint = 0x32C034 // {2}WO Audio up general purpose control;
pub const MO_AUDR_GPCNTRL: c_uint = 0x32C038 // {2}WO Audio RDS general purpose control;
pub const MO_AUD_DMACNTRL: c_uint = 0x32C040 // {6}RW Audio DMA control;
pub const MO_AUD_XFR_STAT: c_uint = 0x32C044 // {1}RO Audio transfer status;
pub const MO_AUDD_LNGTH: c_uint = 0x32C048 // {12}RW Audio down line length;
pub const MO_AUDR_LNGTH: c_uint = 0x32C04C // {12}RW Audio RDS line length;
pub const AUD_INIT: c_uint = 0x320100;
pub const AUD_INIT_LD: c_uint = 0x320104;
pub const AUD_SOFT_RESET: c_uint = 0x320108;
pub const AUD_I2SINPUTCNTL: c_uint = 0x320120;
pub const AUD_BAUDRATE: c_uint = 0x320124;
pub const AUD_I2SOUTPUTCNTL: c_uint = 0x320128;
pub const AAGC_HYST: c_uint = 0x320134;
pub const AAGC_GAIN: c_uint = 0x320138;
pub const AAGC_DEF: c_uint = 0x32013c;
pub const AUD_IIR1_0_SEL: c_uint = 0x320150;
pub const AUD_IIR1_0_SHIFT: c_uint = 0x320154;
pub const AUD_IIR1_1_SEL: c_uint = 0x320158;
pub const AUD_IIR1_1_SHIFT: c_uint = 0x32015c;
pub const AUD_IIR1_2_SEL: c_uint = 0x320160;
pub const AUD_IIR1_2_SHIFT: c_uint = 0x320164;
pub const AUD_IIR1_3_SEL: c_uint = 0x320168;
pub const AUD_IIR1_3_SHIFT: c_uint = 0x32016c;
pub const AUD_IIR1_4_SEL: c_uint = 0x320170;
pub const AUD_IIR1_4_SHIFT: c_uint = 0x32017c;
pub const AUD_IIR1_5_SEL: c_uint = 0x320180;
pub const AUD_IIR1_5_SHIFT: c_uint = 0x320184;
pub const AUD_IIR2_0_SEL: c_uint = 0x320190;
pub const AUD_IIR2_0_SHIFT: c_uint = 0x320194;
pub const AUD_IIR2_1_SEL: c_uint = 0x320198;
pub const AUD_IIR2_1_SHIFT: c_uint = 0x32019c;
pub const AUD_IIR2_2_SEL: c_uint = 0x3201a0;
pub const AUD_IIR2_2_SHIFT: c_uint = 0x3201a4;
pub const AUD_IIR2_3_SEL: c_uint = 0x3201a8;
pub const AUD_IIR2_3_SHIFT: c_uint = 0x3201ac;
pub const AUD_IIR3_0_SEL: c_uint = 0x3201c0;
pub const AUD_IIR3_0_SHIFT: c_uint = 0x3201c4;
pub const AUD_IIR3_1_SEL: c_uint = 0x3201c8;
pub const AUD_IIR3_1_SHIFT: c_uint = 0x3201cc;
pub const AUD_IIR3_2_SEL: c_uint = 0x3201d0;
pub const AUD_IIR3_2_SHIFT: c_uint = 0x3201d4;
pub const AUD_IIR4_0_SEL: c_uint = 0x3201e0;
pub const AUD_IIR4_0_SHIFT: c_uint = 0x3201e4;
pub const AUD_IIR4_1_SEL: c_uint = 0x3201e8;
pub const AUD_IIR4_1_SHIFT: c_uint = 0x3201ec;
pub const AUD_IIR4_2_SEL: c_uint = 0x3201f0;
pub const AUD_IIR4_2_SHIFT: c_uint = 0x3201f4;
pub const AUD_IIR4_0_CA0: c_uint = 0x320200;
pub const AUD_IIR4_0_CA1: c_uint = 0x320204;
pub const AUD_IIR4_0_CA2: c_uint = 0x320208;
pub const AUD_IIR4_0_CB0: c_uint = 0x32020c;
pub const AUD_IIR4_0_CB1: c_uint = 0x320210;
pub const AUD_IIR4_1_CA0: c_uint = 0x320214;
pub const AUD_IIR4_1_CA1: c_uint = 0x320218;
pub const AUD_IIR4_1_CA2: c_uint = 0x32021c;
pub const AUD_IIR4_1_CB0: c_uint = 0x320220;
pub const AUD_IIR4_1_CB1: c_uint = 0x320224;
pub const AUD_IIR4_2_CA0: c_uint = 0x320228;
pub const AUD_IIR4_2_CA1: c_uint = 0x32022c;
pub const AUD_IIR4_2_CA2: c_uint = 0x320230;
pub const AUD_IIR4_2_CB0: c_uint = 0x320234;
pub const AUD_IIR4_2_CB1: c_uint = 0x320238;
pub const AUD_HP_MD_IIR4_1: c_uint = 0x320250;
pub const AUD_HP_PROG_IIR4_1: c_uint = 0x320254;
pub const AUD_FM_MODE_ENABLE: c_uint = 0x320258;
pub const AUD_POLY0_DDS_CONSTANT: c_uint = 0x320270;
pub const AUD_DN0_FREQ: c_uint = 0x320274;
pub const AUD_DN1_FREQ: c_uint = 0x320278;
pub const AUD_DN1_FREQ_SHIFT: c_uint = 0x32027c;
pub const AUD_DN1_AFC: c_uint = 0x320280;
pub const AUD_DN1_SRC_SEL: c_uint = 0x320284;
pub const AUD_DN1_SHFT: c_uint = 0x320288;
pub const AUD_DN2_FREQ: c_uint = 0x32028c;
pub const AUD_DN2_FREQ_SHIFT: c_uint = 0x320290;
pub const AUD_DN2_AFC: c_uint = 0x320294;
pub const AUD_DN2_SRC_SEL: c_uint = 0x320298;
pub const AUD_DN2_SHFT: c_uint = 0x32029c;
pub const AUD_CRDC0_SRC_SEL: c_uint = 0x320300;
pub const AUD_CRDC0_SHIFT: c_uint = 0x320304;
pub const AUD_CORDIC_SHIFT_0: c_uint = 0x320308;
pub const AUD_CRDC1_SRC_SEL: c_uint = 0x32030c;
pub const AUD_CRDC1_SHIFT: c_uint = 0x320310;
pub const AUD_CORDIC_SHIFT_1: c_uint = 0x320314;
pub const AUD_DCOC_0_SRC: c_uint = 0x320320;
pub const AUD_DCOC0_SHIFT: c_uint = 0x320324;
pub const AUD_DCOC_0_SHIFT_IN0: c_uint = 0x320328;
pub const AUD_DCOC_0_SHIFT_IN1: c_uint = 0x32032c;
pub const AUD_DCOC_1_SRC: c_uint = 0x320330;
pub const AUD_DCOC1_SHIFT: c_uint = 0x320334;
pub const AUD_DCOC_1_SHIFT_IN0: c_uint = 0x320338;
pub const AUD_DCOC_1_SHIFT_IN1: c_uint = 0x32033c;
pub const AUD_DCOC_2_SRC: c_uint = 0x320340;
pub const AUD_DCOC2_SHIFT: c_uint = 0x320344;
pub const AUD_DCOC_2_SHIFT_IN0: c_uint = 0x320348;
pub const AUD_DCOC_2_SHIFT_IN1: c_uint = 0x32034c;
pub const AUD_DCOC_PASS_IN: c_uint = 0x320350;
pub const AUD_PDET_SRC: c_uint = 0x320370;
pub const AUD_PDET_SHIFT: c_uint = 0x320374;
pub const AUD_PILOT_BQD_1_K0: c_uint = 0x320380;
pub const AUD_PILOT_BQD_1_K1: c_uint = 0x320384;
pub const AUD_PILOT_BQD_1_K2: c_uint = 0x320388;
pub const AUD_PILOT_BQD_1_K3: c_uint = 0x32038c;
pub const AUD_PILOT_BQD_1_K4: c_uint = 0x320390;
pub const AUD_PILOT_BQD_2_K0: c_uint = 0x320394;
pub const AUD_PILOT_BQD_2_K1: c_uint = 0x320398;
pub const AUD_PILOT_BQD_2_K2: c_uint = 0x32039c;
pub const AUD_PILOT_BQD_2_K3: c_uint = 0x3203a0;
pub const AUD_PILOT_BQD_2_K4: c_uint = 0x3203a4;
pub const AUD_THR_FR: c_uint = 0x3203c0;
pub const AUD_X_PROG: c_uint = 0x3203c4;
pub const AUD_Y_PROG: c_uint = 0x3203c8;
pub const AUD_HARMONIC_MULT: c_uint = 0x3203cc;
pub const AUD_C1_UP_THR: c_uint = 0x3203d0;
pub const AUD_C1_LO_THR: c_uint = 0x3203d4;
pub const AUD_C2_UP_THR: c_uint = 0x3203d8;
pub const AUD_C2_LO_THR: c_uint = 0x3203dc;
pub const AUD_PLL_EN: c_uint = 0x320400;
pub const AUD_PLL_SRC: c_uint = 0x320404;
pub const AUD_PLL_SHIFT: c_uint = 0x320408;
pub const AUD_PLL_IF_SEL: c_uint = 0x32040c;
pub const AUD_PLL_IF_SHIFT: c_uint = 0x320410;
pub const AUD_BIQUAD_PLL_K0: c_uint = 0x320414;
pub const AUD_BIQUAD_PLL_K1: c_uint = 0x320418;
pub const AUD_BIQUAD_PLL_K2: c_uint = 0x32041c;
pub const AUD_BIQUAD_PLL_K3: c_uint = 0x320420;
pub const AUD_BIQUAD_PLL_K4: c_uint = 0x320424;
pub const AUD_DEEMPH0_SRC_SEL: c_uint = 0x320440;
pub const AUD_DEEMPH0_SHIFT: c_uint = 0x320444;
pub const AUD_DEEMPH0_G0: c_uint = 0x320448;
pub const AUD_DEEMPH0_A0: c_uint = 0x32044c;
pub const AUD_DEEMPH0_B0: c_uint = 0x320450;
pub const AUD_DEEMPH0_A1: c_uint = 0x320454;
pub const AUD_DEEMPH0_B1: c_uint = 0x320458;
pub const AUD_DEEMPH1_SRC_SEL: c_uint = 0x32045c;
pub const AUD_DEEMPH1_SHIFT: c_uint = 0x320460;
pub const AUD_DEEMPH1_G0: c_uint = 0x320464;
pub const AUD_DEEMPH1_A0: c_uint = 0x320468;
pub const AUD_DEEMPH1_B0: c_uint = 0x32046c;
pub const AUD_DEEMPH1_A1: c_uint = 0x320470;
pub const AUD_DEEMPH1_B1: c_uint = 0x320474;
pub const AUD_OUT0_SEL: c_uint = 0x320490;
pub const AUD_OUT0_SHIFT: c_uint = 0x320494;
pub const AUD_OUT1_SEL: c_uint = 0x320498;
pub const AUD_OUT1_SHIFT: c_uint = 0x32049c;
pub const AUD_RDSI_SEL: c_uint = 0x3204a0;
pub const AUD_RDSI_SHIFT: c_uint = 0x3204a4;
pub const AUD_RDSQ_SEL: c_uint = 0x3204a8;
pub const AUD_RDSQ_SHIFT: c_uint = 0x3204ac;
pub const AUD_DBX_IN_GAIN: c_uint = 0x320500;
pub const AUD_DBX_WBE_GAIN: c_uint = 0x320504;
pub const AUD_DBX_SE_GAIN: c_uint = 0x320508;
pub const AUD_DBX_RMS_WBE: c_uint = 0x32050c;
pub const AUD_DBX_RMS_SE: c_uint = 0x320510;
pub const AUD_DBX_SE_BYPASS: c_uint = 0x320514;
pub const AUD_FAWDETCTL: c_uint = 0x320530;
pub const AUD_FAWDETWINCTL: c_uint = 0x320534;
pub const AUD_DEEMPHGAIN_R: c_uint = 0x320538;
pub const AUD_DEEMPHNUMER1_R: c_uint = 0x32053c;
pub const AUD_DEEMPHNUMER2_R: c_uint = 0x320540;
pub const AUD_DEEMPHDENOM1_R: c_uint = 0x320544;
pub const AUD_DEEMPHDENOM2_R: c_uint = 0x320548;
pub const AUD_ERRLOGPERIOD_R: c_uint = 0x32054c;
pub const AUD_ERRINTRPTTHSHLD1_R: c_uint = 0x320550;
pub const AUD_ERRINTRPTTHSHLD2_R: c_uint = 0x320554;
pub const AUD_ERRINTRPTTHSHLD3_R: c_uint = 0x320558;
pub const AUD_NICAM_STATUS1: c_uint = 0x32055c;
pub const AUD_NICAM_STATUS2: c_uint = 0x320560;
pub const AUD_ERRLOG1: c_uint = 0x320564;
pub const AUD_ERRLOG2: c_uint = 0x320568;
pub const AUD_ERRLOG3: c_uint = 0x32056c;
pub const AUD_DAC_BYPASS_L: c_uint = 0x320580;
pub const AUD_DAC_BYPASS_R: c_uint = 0x320584;
pub const AUD_DAC_BYPASS_CTL: c_uint = 0x320588;
pub const AUD_CTL: c_uint = 0x32058c;
pub const AUD_STATUS: c_uint = 0x320590;
pub const AUD_VOL_CTL: c_uint = 0x320594;
pub const AUD_BAL_CTL: c_uint = 0x320598;
pub const AUD_START_TIMER: c_uint = 0x3205b0;
pub const AUD_MODE_CHG_TIMER: c_uint = 0x3205b4;
pub const AUD_POLYPH80SCALEFAC: c_uint = 0x3205b8;
pub const AUD_DMD_RA_DDS: c_uint = 0x3205bc;
pub const AUD_I2S_RA_DDS: c_uint = 0x3205c0;
pub const AUD_RATE_THRES_DMD: c_uint = 0x3205d0;
pub const AUD_RATE_THRES_I2S: c_uint = 0x3205d4;
pub const AUD_RATE_ADJ1: c_uint = 0x3205d8;
pub const AUD_RATE_ADJ2: c_uint = 0x3205dc;
pub const AUD_RATE_ADJ3: c_uint = 0x3205e0;
pub const AUD_RATE_ADJ4: c_uint = 0x3205e4;
pub const AUD_RATE_ADJ5: c_uint = 0x3205e8;
pub const AUD_APB_IN_RATE_ADJ: c_uint = 0x3205ec;
pub const AUD_I2SCNTL: c_uint = 0x3205ec;
pub const AUD_PHASE_FIX_CTL: c_uint = 0x3205f0;
pub const AUD_PLL_PRESCALE: c_uint = 0x320600;
pub const AUD_PLL_DDS: c_uint = 0x320604;
pub const AUD_PLL_INT: c_uint = 0x320608;
pub const AUD_PLL_FRAC: c_uint = 0x32060c;
pub const AUD_PLL_JTAG: c_uint = 0x320620;
pub const AUD_PLL_SPMP: c_uint = 0x320624;
pub const AUD_AFE_12DB_EN: c_uint = 0x320628;
// Audio QAM Register Addresses
pub const AUD_PDF_DDS_CNST_BYTE2: c_uint = 0x320d01;
pub const AUD_PDF_DDS_CNST_BYTE1: c_uint = 0x320d02;
pub const AUD_PDF_DDS_CNST_BYTE0: c_uint = 0x320d03;
pub const AUD_PHACC_FREQ_8MSB: c_uint = 0x320d2a;
pub const AUD_PHACC_FREQ_8LSB: c_uint = 0x320d2b;
pub const AUD_QAM_MODE: c_uint = 0x320d04;
//
// transport stream registers
//
pub const MO_TS_DMA: c_uint = 0x330000 // {64}RWp Transport stream downstream;
pub const MO_TS_GPCNT: c_uint = 0x33C020 // {16}RO TS general purpose counter;
pub const MO_TS_GPCNTRL: c_uint = 0x33C030 // {2}WO TS general purpose control;
pub const MO_TS_DMACNTRL: c_uint = 0x33C040 // {6}RW TS DMA control;
pub const MO_TS_XFR_STAT: c_uint = 0x33C044 // {1}RO TS transfer status;
pub const MO_TS_LNGTH: c_uint = 0x33C048 // {12}RW TS line length;
pub const TS_HW_SOP_CNTRL: c_uint = 0x33C04C;
pub const TS_GEN_CNTRL: c_uint = 0x33C050;
pub const TS_BD_PKT_STAT: c_uint = 0x33C054;
pub const TS_SOP_STAT: c_uint = 0x33C058;
pub const TS_FIFO_OVFL_STAT: c_uint = 0x33C05C;
pub const TS_VALERR_CNTRL: c_uint = 0x33C060;
//
// VIP registers
//
pub const MO_VIPD_DMA: c_uint = 0x340000 // {64}RWp VIP downstream;
pub const MO_VIPU_DMA: c_uint = 0x340008 // {64}RWp VIP upstream;
pub const MO_VIPD_GPCNT: c_uint = 0x34C020 // {16}RO VIP down general purpose counter;
pub const MO_VIPU_GPCNT: c_uint = 0x34C024 // {16}RO VIP up general purpose counter;
pub const MO_VIPD_GPCNTRL: c_uint = 0x34C030 // {2}WO VIP down general purpose control;
pub const MO_VIPU_GPCNTRL: c_uint = 0x34C034 // {2}WO VIP up general purpose control;
pub const MO_VIP_DMACNTRL: c_uint = 0x34C040 // {6}RW VIP DMA control;
pub const MO_VIP_XFR_STAT: c_uint = 0x34C044 // {1}RO VIP transfer status;
pub const MO_VIP_CFG: c_uint = 0x340048 // VIP configuration;
pub const MO_VIPU_CNTRL: c_uint = 0x34004C // VIP upstream control #1;
pub const MO_VIPD_CNTRL: c_uint = 0x340050 // VIP downstream control #2;
pub const MO_VIPD_LNGTH: c_uint = 0x340054 // VIP downstream line length;
pub const MO_VIP_BRSTLN: c_uint = 0x340058 // VIP burst length;
pub const MO_VIP_INTCNTRL: c_uint = 0x34C05C // VIP Interrupt Control;
pub const MO_VIP_XFTERM: c_uint = 0x340060 // VIP transfer terminate;
//
// misc registers
//
pub const MO_M2M_DMA: c_uint = 0x350000 // {64}RWp Mem2Mem DMA Bfr;
pub const MO_GP0_IO: c_uint = 0x350010 // {32}RW* GPIOoutput enablesdata I/O;
pub const MO_GP1_IO: c_uint = 0x350014 // {32}RW* GPIOoutput enablesdata I/O;
pub const MO_GP2_IO: c_uint = 0x350018 // {32}RW* GPIOoutput enablesdata I/O;
pub const MO_GP3_IO: c_uint = 0x35001C // {32}RW* GPIO Mode/Ctrloutput enables;
pub const MO_GPIO: c_uint = 0x350020 // {32}RW* GPIO I2C Ctrldata I/O;
pub const MO_GPOE: c_uint = 0x350024 // {32}RW  GPIO I2C Ctrloutput enables;
pub const MO_GP_ISM: c_uint = 0x350028 // {16}WO  GPIO Intr Sens/Pol;
pub const MO_PLL_B: c_uint = 0x35C008 // {32}RW* PLL Control for ASB bus clks;
pub const MO_M2M_CNT: c_uint = 0x35C024 // {32}RW  Mem2Mem DMA Cnt;
pub const MO_M2M_XSUM: c_uint = 0x35C028 // {32}RO  M2M XOR-Checksum;
pub const MO_CRC: c_uint = 0x35C02C // {16}RW  CRC16 init/result;
pub const MO_CRC_D: c_uint = 0x35C030 // {32}WO  CRC16 new data in;
pub const MO_TM_CNT_LDW: c_uint = 0x35C034 // {32}RO  Timer : Counter low dword;
pub const MO_TM_CNT_UW: c_uint = 0x35C038 // {16}RO  Timer : Counter high word;
pub const MO_TM_LMT_LDW: c_uint = 0x35C03C // {32}RW  Timer : Limit low dword;
pub const MO_TM_LMT_UW: c_uint = 0x35C040 // {32}RW  Timer : Limit high word;
pub const MO_PINMUX_IO: c_uint = 0x35C044 // {8}RW  Pin Mux Control;
pub const MO_TSTSEL_IO: c_uint = 0x35C048 // {2}RW  Pin Mux Control;
pub const MO_AFECFG_IO: c_uint = 0x35C04C // AFE configuration reg;
pub const MO_DDS_IO: c_uint = 0x35C050 // DDS Increment reg;
pub const MO_DDSCFG_IO: c_uint = 0x35C054 // DDS Configuration reg;
pub const MO_SAMPLE_IO: c_uint = 0x35C058 // IRIn sample reg;
pub const MO_SRST_IO: c_uint = 0x35C05C // Output system reset reg;
pub const MO_INT1_MSK: c_uint = 0x35C060 // DMA RISC interrupt mask;
pub const MO_INT1_STAT: c_uint = 0x35C064 // DMA RISC interrupt status;
pub const MO_INT1_MSTAT: c_uint = 0x35C068 // DMA RISC interrupt masked status;
//
// i2c bus registers
//
pub const MO_I2C: c_uint = 0x368000 // I2C data/control;

//
// general purpose host registers
//
// FIXME: tyops?  s/0x35/0x38/ ??
//
pub const MO_GPHSTD_DMA: c_uint = 0x350000 // {64}RWp Host downstream;
pub const MO_GPHSTU_DMA: c_uint = 0x350008 // {64}RWp Host upstream;
pub const MO_GPHSTU_CNTRL: c_uint = 0x380048 // Host upstream control #1;
pub const MO_GPHSTD_CNTRL: c_uint = 0x38004C // Host downstream control #2;
pub const MO_GPHSTD_LNGTH: c_uint = 0x380050 // Host downstream line length;
pub const MO_GPHST_WSC: c_uint = 0x380054 // Host wait state control;
pub const MO_GPHST_XFR: c_uint = 0x380058 // Host transfer control;
pub const MO_GPHST_WDTH: c_uint = 0x38005C // Host interface width;
pub const MO_GPHST_HDSHK: c_uint = 0x380060 // Host peripheral handshake;
pub const MO_GPHST_MUX16: c_uint = 0x380064 // Host muxed 16-bit transfer parameters;
pub const MO_GPHST_MODE: c_uint = 0x380068 // Host mode select;
pub const MO_GPHSTD_GPCNT: c_uint = 0x35C020 // Host down general purpose counter;
pub const MO_GPHSTU_GPCNT: c_uint = 0x35C024 // Host up general purpose counter;
pub const MO_GPHSTD_GPCNTRL: c_uint = 0x38C030 // Host down general purpose control;
pub const MO_GPHSTU_GPCNTRL: c_uint = 0x38C034 // Host up general purpose control;
pub const MO_GPHST_DMACNTRL: c_uint = 0x38C040 // Host DMA control;
pub const MO_GPHST_XFR_STAT: c_uint = 0x38C044 // Host transfer status;
pub const MO_GPHST_SOFT_RST: c_uint = 0x38C06C // Host software reset;
//
// RISC instructions
//
pub const RISC_SYNC: c_uint = 0x80000000;
pub const RISC_SYNC_ODD: c_uint = 0x80000000;
pub const RISC_SYNC_EVEN: c_uint = 0x80000200;
pub const RISC_RESYNC: c_uint = 0x80008000;
pub const RISC_RESYNC_ODD: c_uint = 0x80008000;
pub const RISC_RESYNC_EVEN: c_uint = 0x80008200;
pub const RISC_WRITE: c_uint = 0x10000000;
pub const RISC_WRITEC: c_uint = 0x50000000;
pub const RISC_READ: c_uint = 0x90000000;
pub const RISC_READC: c_uint = 0xA0000000;
pub const RISC_JUMP: c_uint = 0x70000000;
pub const RISC_SKIP: c_uint = 0x20000000;
pub const RISC_WRITERM: c_uint = 0xB0000000;
pub const RISC_WRITECM: c_uint = 0xC0000000;
pub const RISC_WRITECR: c_uint = 0xD0000000;
pub const RISC_IMM: c_uint = 0x00000001;
pub const RISC_SOL: c_uint = 0x08000000;
pub const RISC_EOL: c_uint = 0x04000000;
pub const RISC_IRQ2: c_uint = 0x02000000;
pub const RISC_IRQ1: c_uint = 0x01000000;
pub const RISC_CNT_NONE: c_uint = 0x00000000;
pub const RISC_CNT_INC: c_uint = 0x00010000;
pub const RISC_CNT_RSVR: c_uint = 0x00020000;
pub const RISC_CNT_RESET: c_uint = 0x00030000;
pub const RISC_JMP_SRP: c_uint = 0x01;
//
// various constants
//
// DMA
// Interrupt mask/status

pub const SEL_BTSC: c_uint = 0x01;
pub const SEL_EIAJ: c_uint = 0x02;
pub const SEL_A2: c_uint = 0x04;
pub const SEL_SAP: c_uint = 0x08;
pub const SEL_NICAM: c_uint = 0x10;
pub const SEL_FMRADIO: c_uint = 0x20;
// AUD_CTL

pub const EN_BTSC_FORCE_MONO: c_int = 0;
pub const EN_BTSC_FORCE_STEREO: c_int = 1;
pub const EN_BTSC_FORCE_SAP: c_int = 2;
pub const EN_BTSC_AUTO_STEREO: c_int = 3;
pub const EN_BTSC_AUTO_SAP: c_int = 4;
pub const EN_A2_FORCE_MONO1: c_int = 8;
pub const EN_A2_FORCE_MONO2: c_int = 9;
pub const EN_A2_FORCE_STEREO: c_int = 10;
pub const EN_A2_AUTO_MONO2: c_int = 11;
pub const EN_A2_AUTO_STEREO: c_int = 12;
pub const EN_EIAJ_FORCE_MONO1: c_int = 16;
pub const EN_EIAJ_FORCE_MONO2: c_int = 17;
pub const EN_EIAJ_FORCE_STEREO: c_int = 18;
pub const EN_EIAJ_AUTO_MONO2: c_int = 19;
pub const EN_EIAJ_AUTO_STEREO: c_int = 20;
pub const EN_NICAM_FORCE_MONO1: c_int = 32;
pub const EN_NICAM_FORCE_MONO2: c_int = 33;
pub const EN_NICAM_FORCE_STEREO: c_int = 34;
pub const EN_NICAM_AUTO_MONO2: c_int = 35;
pub const EN_NICAM_AUTO_STEREO: c_int = 36;
pub const EN_FMRADIO_FORCE_MONO: c_int = 24;
pub const EN_FMRADIO_FORCE_STEREO: c_int = 25;
pub const EN_FMRADIO_AUTO_STEREO: c_int = 26;
pub const EN_NICAM_AUTO_FALLBACK: c_uint = 0x00000040;
pub const EN_FMRADIO_EN_RDS: c_uint = 0x00000200;
pub const EN_NICAM_TRY_AGAIN_BIT: c_uint = 0x00000400;
pub const EN_DAC_ENABLE: c_uint = 0x00001000;
pub const EN_I2SOUT_ENABLE: c_uint = 0x00002000;
pub const EN_I2SIN_STR2DAC: c_uint = 0x00004000;
pub const EN_I2SIN_ENABLE: c_uint = 0x00008000;

// Video
pub const VID_CAPTURE_CONTROL: c_uint = 0x310180;

pub const VideoInputMux0: c_uint = 0x0;
pub const VideoInputMux1: c_uint = 0x1;
pub const VideoInputMux2: c_uint = 0x2;
pub const VideoInputMux3: c_uint = 0x3;
pub const VideoInputTuner: c_uint = 0x0;
pub const VideoInputComposite: c_uint = 0x1;
pub const VideoInputSVideo: c_uint = 0x2;
pub const VideoInputOther: c_uint = 0x3;
pub const Xtal0: c_uint = 0x1;
pub const Xtal1: c_uint = 0x2;
pub const XtalAuto: c_uint = 0x3;
pub const VideoFormatAuto: c_uint = 0x0;
pub const VideoFormatNTSC: c_uint = 0x1;
pub const VideoFormatNTSCJapan: c_uint = 0x2;
pub const VideoFormatNTSC443: c_uint = 0x3;
pub const VideoFormatPAL: c_uint = 0x4;
pub const VideoFormatPALB: c_uint = 0x4;
pub const VideoFormatPALD: c_uint = 0x4;
pub const VideoFormatPALG: c_uint = 0x4;
pub const VideoFormatPALH: c_uint = 0x4;
pub const VideoFormatPALI: c_uint = 0x4;
pub const VideoFormatPALBDGHI: c_uint = 0x4;
pub const VideoFormatPALM: c_uint = 0x5;
pub const VideoFormatPALN: c_uint = 0x6;
pub const VideoFormatPALNC: c_uint = 0x7;
pub const VideoFormatPAL60: c_uint = 0x8;
pub const VideoFormatSECAM: c_uint = 0x9;
pub const VideoFormatAuto27MHz: c_uint = 0x10;
pub const VideoFormatNTSC27MHz: c_uint = 0x11;
pub const VideoFormatNTSCJapan27MHz: c_uint = 0x12;
pub const VideoFormatNTSC44327MHz: c_uint = 0x13;
pub const VideoFormatPAL27MHz: c_uint = 0x14;
pub const VideoFormatPALB27MHz: c_uint = 0x14;
pub const VideoFormatPALD27MHz: c_uint = 0x14;
pub const VideoFormatPALG27MHz: c_uint = 0x14;
pub const VideoFormatPALH27MHz: c_uint = 0x14;
pub const VideoFormatPALI27MHz: c_uint = 0x14;
pub const VideoFormatPALBDGHI27MHz: c_uint = 0x14;
pub const VideoFormatPALM27MHz: c_uint = 0x15;
pub const VideoFormatPALN27MHz: c_uint = 0x16;
pub const VideoFormatPALNC27MHz: c_uint = 0x17;
pub const VideoFormatPAL6027MHz: c_uint = 0x18;
pub const VideoFormatSECAM27MHz: c_uint = 0x19;
pub const NominalUSECAM: c_uint = 0x87;
pub const NominalVSECAM: c_uint = 0x85;
pub const NominalUNTSC: c_uint = 0xFE;
pub const NominalVNTSC: c_uint = 0xB4;
pub const NominalContrast: c_uint = 0xD8;
pub const HFilterAutoFormat: c_uint = 0x0;
pub const HFilterCIF: c_uint = 0x1;
pub const HFilterQCIF: c_uint = 0x2;
pub const HFilterICON: c_uint = 0x3;
pub const VFilter2TapInterpolate: c_int = 0;
pub const VFilter3TapInterpolate: c_int = 1;
pub const VFilter4TapInterpolate: c_int = 2;
pub const VFilter5TapInterpolate: c_int = 3;
pub const VFilter2TapNoInterpolate: c_int = 4;
pub const VFilter3TapNoInterpolate: c_int = 5;
pub const VFilter4TapNoInterpolate: c_int = 6;
pub const VFilter5TapNoInterpolate: c_int = 7;
pub const ColorFormatRGB32: c_uint = 0x0000;
pub const ColorFormatRGB24: c_uint = 0x0011;
pub const ColorFormatRGB16: c_uint = 0x0022;
pub const ColorFormatRGB15: c_uint = 0x0033;
pub const ColorFormatYUY2: c_uint = 0x0044;
pub const ColorFormatBTYUV: c_uint = 0x0055;
pub const ColorFormatY8: c_uint = 0x0066;
pub const ColorFormatRGB8: c_uint = 0x0077;
pub const ColorFormatPL422: c_uint = 0x0088;
pub const ColorFormatPL411: c_uint = 0x0099;
pub const ColorFormatYUV12: c_uint = 0x00AA;
pub const ColorFormatYUV9: c_uint = 0x00BB;
pub const ColorFormatRAW: c_uint = 0x00EE;
pub const ColorFormatBSWAP: c_uint = 0x0300;
pub const ColorFormatWSWAP: c_uint = 0x0c00;
pub const ColorFormatEvenMask: c_uint = 0x050f;
pub const ColorFormatOddMask: c_uint = 0x0af0;
pub const ColorFormatGamma: c_uint = 0x1000;
pub const Interlaced: c_uint = 0x1;
pub const NonInterlaced: c_uint = 0x0;
pub const FieldEven: c_uint = 0x1;
pub const FieldOdd: c_uint = 0x0;
pub const TGReadWriteMode: c_uint = 0x0;
pub const TGEnableMode: c_uint = 0x1;
pub const DV_CbAlign: c_uint = 0x0;
pub const DV_Y0Align: c_uint = 0x1;
pub const DV_CrAlign: c_uint = 0x2;
pub const DV_Y1Align: c_uint = 0x3;
pub const DVF_Analog: c_uint = 0x0;
pub const DVF_CCIR656: c_uint = 0x1;
pub const DVF_ByteStream: c_uint = 0x2;
pub const DVF_ExtVSYNC: c_uint = 0x4;
pub const DVF_ExtField: c_uint = 0x5;
pub const CHANNEL_VID_Y: c_uint = 0x1;
pub const CHANNEL_VID_U: c_uint = 0x2;
pub const CHANNEL_VID_V: c_uint = 0x3;
pub const CHANNEL_VID_VBI: c_uint = 0x4;
pub const CHANNEL_AUD_DN: c_uint = 0x5;
pub const CHANNEL_AUD_UP: c_uint = 0x6;
pub const CHANNEL_AUD_RDS_DN: c_uint = 0x7;
pub const CHANNEL_MPEG_DN: c_uint = 0x8;
pub const CHANNEL_VIP_DN: c_uint = 0x9;
pub const CHANNEL_VIP_UP: c_uint = 0xA;
pub const CHANNEL_HOST_DN: c_uint = 0xB;
pub const CHANNEL_HOST_UP: c_uint = 0xC;
pub const CHANNEL_FIRST: c_uint = 0x1;
pub const CHANNEL_LAST: c_uint = 0xC;
pub const GP_COUNT_CONTROL_NONE: c_uint = 0x0;
pub const GP_COUNT_CONTROL_INC: c_uint = 0x1;
pub const GP_COUNT_CONTROL_RESERVED: c_uint = 0x2;
pub const GP_COUNT_CONTROL_RESET: c_uint = 0x3;
pub const PLL_PRESCALE_BY_2: c_int = 2;
pub const PLL_PRESCALE_BY_3: c_int = 3;
pub const PLL_PRESCALE_BY_4: c_int = 4;
pub const PLL_PRESCALE_BY_5: c_int = 5;
pub const HLNotchFilter4xFsc: c_int = 0;
pub const HLNotchFilterSquare: c_int = 1;
pub const HLNotchFilter135NTSC: c_int = 2;
pub const HLNotchFilter135PAL: c_int = 3;

// Default analog settings
pub const DEFAULT_HUE_NTSC: c_uint = 0x00;
pub const DEFAULT_BRIGHTNESS_NTSC: c_uint = 0x00;
pub const DEFAULT_CONTRAST_NTSC: c_uint = 0x39;
pub const DEFAULT_SAT_U_NTSC: c_uint = 0x7F;
pub const DEFAULT_SAT_V_NTSC: c_uint = 0x5A;
