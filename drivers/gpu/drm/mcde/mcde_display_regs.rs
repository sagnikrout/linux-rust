//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/mcde/mcde_display_regs.h
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
// PP (pixel processor) interrupts
pub const MCDE_IMSCPP: c_uint = 0x00000104;
pub const MCDE_RISPP: c_uint = 0x00000114;
pub const MCDE_MISPP: c_uint = 0x00000124;
pub const MCDE_SISPP: c_uint = 0x00000134;

// Overlay interrupts
pub const MCDE_IMSCOVL: c_uint = 0x00000108;
pub const MCDE_RISOVL: c_uint = 0x00000118;
pub const MCDE_MISOVL: c_uint = 0x00000128;
pub const MCDE_SISOVL: c_uint = 0x00000138;
// Channel interrupts
pub const MCDE_IMSCCHNL: c_uint = 0x0000010C;
pub const MCDE_RISCHNL: c_uint = 0x0000011C;
pub const MCDE_MISCHNL: c_uint = 0x0000012C;
pub const MCDE_SISCHNL: c_uint = 0x0000013C;
// X = 0..9
pub const MCDE_EXTSRCXA0: c_uint = 0x00000200;
pub const MCDE_EXTSRCXA0_GROUPOFFSET: c_uint = 0x20;
pub const MCDE_EXTSRCXA0_BASEADDRESS0_SHIFT: c_int = 3;
pub const MCDE_EXTSRCXA0_BASEADDRESS0_MASK: c_uint = 0xFFFFFFF8;
pub const MCDE_EXTSRCXA1: c_uint = 0x00000204;
pub const MCDE_EXTSRCXA1_GROUPOFFSET: c_uint = 0x20;
pub const MCDE_EXTSRCXA1_BASEADDRESS1_SHIFT: c_int = 3;
pub const MCDE_EXTSRCXA1_BASEADDRESS1_MASK: c_uint = 0xFFFFFFF8;
// External sources 0..9
pub const MCDE_EXTSRC0CONF: c_uint = 0x0000020C;
pub const MCDE_EXTSRC1CONF: c_uint = 0x0000022C;
pub const MCDE_EXTSRC2CONF: c_uint = 0x0000024C;
pub const MCDE_EXTSRC3CONF: c_uint = 0x0000026C;
pub const MCDE_EXTSRC4CONF: c_uint = 0x0000028C;
pub const MCDE_EXTSRC5CONF: c_uint = 0x000002AC;
pub const MCDE_EXTSRC6CONF: c_uint = 0x000002CC;
pub const MCDE_EXTSRC7CONF: c_uint = 0x000002EC;
pub const MCDE_EXTSRC8CONF: c_uint = 0x0000030C;
pub const MCDE_EXTSRC9CONF: c_uint = 0x0000032C;
pub const MCDE_EXTSRCXCONF_GROUPOFFSET: c_uint = 0x20;
pub const MCDE_EXTSRCXCONF_BUF_ID_SHIFT: c_int = 0;
pub const MCDE_EXTSRCXCONF_BUF_ID_MASK: c_uint = 0x00000003;
pub const MCDE_EXTSRCXCONF_BUF_NB_SHIFT: c_int = 2;
pub const MCDE_EXTSRCXCONF_BUF_NB_MASK: c_uint = 0x0000000C;
pub const MCDE_EXTSRCXCONF_PRI_OVLID_SHIFT: c_int = 4;
pub const MCDE_EXTSRCXCONF_PRI_OVLID_MASK: c_uint = 0x000000F0;
pub const MCDE_EXTSRCXCONF_BPP_SHIFT: c_int = 8;
pub const MCDE_EXTSRCXCONF_BPP_MASK: c_uint = 0x00000F00;
pub const MCDE_EXTSRCXCONF_BPP_1BPP_PAL: c_int = 0;
pub const MCDE_EXTSRCXCONF_BPP_2BPP_PAL: c_int = 1;
pub const MCDE_EXTSRCXCONF_BPP_4BPP_PAL: c_int = 2;
pub const MCDE_EXTSRCXCONF_BPP_8BPP_PAL: c_int = 3;
pub const MCDE_EXTSRCXCONF_BPP_RGB444: c_int = 4;
pub const MCDE_EXTSRCXCONF_BPP_ARGB4444: c_int = 5;
pub const MCDE_EXTSRCXCONF_BPP_IRGB1555: c_int = 6;
pub const MCDE_EXTSRCXCONF_BPP_RGB565: c_int = 7;
pub const MCDE_EXTSRCXCONF_BPP_RGB888: c_int = 8;
pub const MCDE_EXTSRCXCONF_BPP_XRGB8888: c_int = 9;
pub const MCDE_EXTSRCXCONF_BPP_ARGB8888: c_int = 10;
pub const MCDE_EXTSRCXCONF_BPP_YCBCR422: c_int = 11;

pub const MCDE_EXTSRCXCONF_TUNNELING_BUFFER_HEIGHT_SHIFT: c_int = 16;
pub const MCDE_EXTSRCXCONF_TUNNELING_BUFFER_HEIGHT_MASK: c_uint = 0x0FFF0000;
// External sources 0..9
pub const MCDE_EXTSRC0CR: c_uint = 0x00000210;
pub const MCDE_EXTSRC1CR: c_uint = 0x00000230;
pub const MCDE_EXTSRC2CR: c_uint = 0x00000250;
pub const MCDE_EXTSRC3CR: c_uint = 0x00000270;
pub const MCDE_EXTSRC4CR: c_uint = 0x00000290;
pub const MCDE_EXTSRC5CR: c_uint = 0x000002B0;
pub const MCDE_EXTSRC6CR: c_uint = 0x000002D0;
pub const MCDE_EXTSRC7CR: c_uint = 0x000002F0;
pub const MCDE_EXTSRC8CR: c_uint = 0x00000310;
pub const MCDE_EXTSRC9CR: c_uint = 0x00000330;
pub const MCDE_EXTSRCXCR_SEL_MOD_SHIFT: c_int = 0;
pub const MCDE_EXTSRCXCR_SEL_MOD_MASK: c_uint = 0x00000003;
pub const MCDE_EXTSRCXCR_SEL_MOD_EXTERNAL_SEL: c_int = 0;
pub const MCDE_EXTSRCXCR_SEL_MOD_AUTO_TOGGLE: c_int = 1;
pub const MCDE_EXTSRCXCR_SEL_MOD_SOFTWARE_SEL: c_int = 2;

// Only external source 6 has a second address register
pub const MCDE_EXTSRC6A2: c_uint = 0x000002C8;
// 6 overlays
pub const MCDE_OVL0CR: c_uint = 0x00000400;
pub const MCDE_OVL1CR: c_uint = 0x00000420;
pub const MCDE_OVL2CR: c_uint = 0x00000440;
pub const MCDE_OVL3CR: c_uint = 0x00000460;
pub const MCDE_OVL4CR: c_uint = 0x00000480;
pub const MCDE_OVL5CR: c_uint = 0x000004A0;

pub const MCDE_OVLXCR_COLCCTRL_DISABLED: c_int = 0;

pub const MCDE_OVLXCR_FETCH_ROPC_SHIFT: c_int = 8;
pub const MCDE_OVLXCR_FETCH_ROPC_MASK: c_uint = 0x0000FF00;
pub const MCDE_OVLXCR_STBPRIO_SHIFT: c_int = 16;
pub const MCDE_OVLXCR_STBPRIO_MASK: c_uint = 0x000F0000;
pub const MCDE_OVLXCR_BURSTSIZE_SHIFT: c_int = 20;
pub const MCDE_OVLXCR_BURSTSIZE_MASK: c_uint = 0x00F00000;
pub const MCDE_OVLXCR_BURSTSIZE_1W: c_int = 0;
pub const MCDE_OVLXCR_BURSTSIZE_2W: c_int = 1;
pub const MCDE_OVLXCR_BURSTSIZE_4W: c_int = 2;
pub const MCDE_OVLXCR_BURSTSIZE_8W: c_int = 3;
pub const MCDE_OVLXCR_BURSTSIZE_16W: c_int = 4;
pub const MCDE_OVLXCR_BURSTSIZE_HW_1W: c_int = 8;
pub const MCDE_OVLXCR_BURSTSIZE_HW_2W: c_int = 9;
pub const MCDE_OVLXCR_BURSTSIZE_HW_4W: c_int = 10;
pub const MCDE_OVLXCR_BURSTSIZE_HW_8W: c_int = 11;
pub const MCDE_OVLXCR_BURSTSIZE_HW_16W: c_int = 12;
pub const MCDE_OVLXCR_MAXOUTSTANDING_SHIFT: c_int = 24;
pub const MCDE_OVLXCR_MAXOUTSTANDING_MASK: c_uint = 0x0F000000;
pub const MCDE_OVLXCR_MAXOUTSTANDING_1_REQ: c_int = 0;
pub const MCDE_OVLXCR_MAXOUTSTANDING_2_REQ: c_int = 1;
pub const MCDE_OVLXCR_MAXOUTSTANDING_4_REQ: c_int = 2;
pub const MCDE_OVLXCR_MAXOUTSTANDING_8_REQ: c_int = 3;
pub const MCDE_OVLXCR_MAXOUTSTANDING_16_REQ: c_int = 4;
pub const MCDE_OVLXCR_ROTBURSTSIZE_SHIFT: c_int = 28;
pub const MCDE_OVLXCR_ROTBURSTSIZE_MASK: c_uint = 0xF0000000;
pub const MCDE_OVLXCR_ROTBURSTSIZE_1W: c_int = 0;
pub const MCDE_OVLXCR_ROTBURSTSIZE_2W: c_int = 1;
pub const MCDE_OVLXCR_ROTBURSTSIZE_4W: c_int = 2;
pub const MCDE_OVLXCR_ROTBURSTSIZE_8W: c_int = 3;
pub const MCDE_OVLXCR_ROTBURSTSIZE_16W: c_int = 4;
pub const MCDE_OVLXCR_ROTBURSTSIZE_HW_1W: c_int = 8;
pub const MCDE_OVLXCR_ROTBURSTSIZE_HW_2W: c_int = 9;
pub const MCDE_OVLXCR_ROTBURSTSIZE_HW_4W: c_int = 10;
pub const MCDE_OVLXCR_ROTBURSTSIZE_HW_8W: c_int = 11;
pub const MCDE_OVLXCR_ROTBURSTSIZE_HW_16W: c_int = 12;
pub const MCDE_OVL0CONF: c_uint = 0x00000404;
pub const MCDE_OVL1CONF: c_uint = 0x00000424;
pub const MCDE_OVL2CONF: c_uint = 0x00000444;
pub const MCDE_OVL3CONF: c_uint = 0x00000464;
pub const MCDE_OVL4CONF: c_uint = 0x00000484;
pub const MCDE_OVL5CONF: c_uint = 0x000004A4;
pub const MCDE_OVLXCONF_PPL_SHIFT: c_int = 0;
pub const MCDE_OVLXCONF_PPL_MASK: c_uint = 0x000007FF;
pub const MCDE_OVLXCONF_EXTSRC_ID_SHIFT: c_int = 11;
pub const MCDE_OVLXCONF_EXTSRC_ID_MASK: c_uint = 0x00007800;
pub const MCDE_OVLXCONF_LPF_SHIFT: c_int = 16;
pub const MCDE_OVLXCONF_LPF_MASK: c_uint = 0x07FF0000;
pub const MCDE_OVL0CONF2: c_uint = 0x00000408;
pub const MCDE_OVL1CONF2: c_uint = 0x00000428;
pub const MCDE_OVL2CONF2: c_uint = 0x00000448;
pub const MCDE_OVL3CONF2: c_uint = 0x00000468;
pub const MCDE_OVL4CONF2: c_uint = 0x00000488;
pub const MCDE_OVL5CONF2: c_uint = 0x000004A8;
pub const MCDE_OVLXCONF2_BP_PER_PIXEL_ALPHA: c_int = 0;

pub const MCDE_OVLXCONF2_ALPHAVALUE_SHIFT: c_int = 1;
pub const MCDE_OVLXCONF2_ALPHAVALUE_MASK: c_uint = 0x000001FE;

pub const MCDE_OVLXCONF2_PIXOFF_SHIFT: c_int = 10;
pub const MCDE_OVLXCONF2_PIXOFF_MASK: c_uint = 0x0000FC00;
pub const MCDE_OVLXCONF2_PIXELFETCHERWATERMARKLEVEL_SHIFT: c_int = 16;
pub const MCDE_OVLXCONF2_PIXELFETCHERWATERMARKLEVEL_MASK: c_uint = 0x1FFF0000;
pub const MCDE_OVL0LJINC: c_uint = 0x0000040C;
pub const MCDE_OVL1LJINC: c_uint = 0x0000042C;
pub const MCDE_OVL2LJINC: c_uint = 0x0000044C;
pub const MCDE_OVL3LJINC: c_uint = 0x0000046C;
pub const MCDE_OVL4LJINC: c_uint = 0x0000048C;
pub const MCDE_OVL5LJINC: c_uint = 0x000004AC;
pub const MCDE_OVL0CROP: c_uint = 0x00000410;
pub const MCDE_OVL1CROP: c_uint = 0x00000430;
pub const MCDE_OVL2CROP: c_uint = 0x00000450;
pub const MCDE_OVL3CROP: c_uint = 0x00000470;
pub const MCDE_OVL4CROP: c_uint = 0x00000490;
pub const MCDE_OVL5CROP: c_uint = 0x000004B0;
pub const MCDE_OVLXCROP_TMRGN_SHIFT: c_int = 0;
pub const MCDE_OVLXCROP_TMRGN_MASK: c_uint = 0x003FFFFF;
pub const MCDE_OVLXCROP_LMRGN_SHIFT: c_int = 22;
pub const MCDE_OVLXCROP_LMRGN_MASK: c_uint = 0xFFC00000;
pub const MCDE_OVL0COMP: c_uint = 0x00000414;
pub const MCDE_OVL1COMP: c_uint = 0x00000434;
pub const MCDE_OVL2COMP: c_uint = 0x00000454;
pub const MCDE_OVL3COMP: c_uint = 0x00000474;
pub const MCDE_OVL4COMP: c_uint = 0x00000494;
pub const MCDE_OVL5COMP: c_uint = 0x000004B4;
pub const MCDE_OVLXCOMP_XPOS_SHIFT: c_int = 0;
pub const MCDE_OVLXCOMP_XPOS_MASK: c_uint = 0x000007FF;
pub const MCDE_OVLXCOMP_CH_ID_SHIFT: c_int = 11;
pub const MCDE_OVLXCOMP_CH_ID_MASK: c_uint = 0x00007800;
pub const MCDE_OVLXCOMP_YPOS_SHIFT: c_int = 16;
pub const MCDE_OVLXCOMP_YPOS_MASK: c_uint = 0x07FF0000;
pub const MCDE_OVLXCOMP_Z_SHIFT: c_int = 27;
pub const MCDE_OVLXCOMP_Z_MASK: c_uint = 0x78000000;
// DPI/TV configuration registers, channel A and B
pub const MCDE_TVCRA: c_uint = 0x00000838;
pub const MCDE_TVCRB: c_uint = 0x00000A38;

// TV blanking control register 1, channel A and B
pub const MCDE_TVBL1A: c_uint = 0x0000083C;
pub const MCDE_TVBL1B: c_uint = 0x00000A3C;

// Pixel processing TV start line, channel A and B
pub const MCDE_TVISLA: c_uint = 0x00000840;
pub const MCDE_TVISLB: c_uint = 0x00000A40;

// Pixel processing TV DVO offset
pub const MCDE_TVDVOA: c_uint = 0x00000844;
pub const MCDE_TVDVOB: c_uint = 0x00000A44;

pub const MCDE_TVDVO_DVO2_SHIFT: c_int = 16;
//
// Pixel processing TV Timing 1
// HBP horizontal back porch 11 bits horizontal offset
// 0 = 1 pixel HBP, 255 = 256 pixels, so actual value - 1
//
pub const MCDE_TVTIM1A: c_uint = 0x0000084C;
pub const MCDE_TVTIM1B: c_uint = 0x00000A4C;
// Pixel processing TV LBALW
// 0 = 1 clock cycle, 255 = 256 clock cycles
pub const MCDE_TVLBALWA: c_uint = 0x00000850;
pub const MCDE_TVLBALWB: c_uint = 0x00000A50;

// TV blanking control register 1, channel A and B
pub const MCDE_TVBL2A: c_uint = 0x00000854;
pub const MCDE_TVBL2B: c_uint = 0x00000A54;

// Pixel processing TV background
pub const MCDE_TVBLUA: c_uint = 0x00000858;
pub const MCDE_TVBLUB: c_uint = 0x00000A58;

// Pixel processing LCD timing 1
pub const MCDE_LCDTIM1A: c_uint = 0x00000860;
pub const MCDE_LCDTIM1B: c_uint = 0x00000A60;
// inverted vertical sync pulse for HRTFT 0 = active low, 1 active high

// inverted vertical sync, 0 = active high (the normal), 1 = active low

// inverted horizontal sync, 0 = active high (the normal), 1 = active low

// inverted panel clock 0 = rising edge data out, 1 = falling edge data out

// invert output enable 0 = active high, 1 = active low

pub const MCDE_CRC: c_uint = 0x00000C00;

pub const MCDE_CRC_SYNCCTRL_SHIFT: c_int = 29;
pub const MCDE_CRC_SYNCCTRL_MASK: c_uint = 0x60000000;
pub const MCDE_CRC_SYNCCTRL_NO_SYNC: c_int = 0;
pub const MCDE_CRC_SYNCCTRL_DBI0: c_int = 1;
pub const MCDE_CRC_SYNCCTRL_DBI1: c_int = 2;
pub const MCDE_CRC_SYNCCTRL_PING_PONG: c_int = 3;

pub const MCDE_VSCRC0: c_uint = 0x00000C5C;
pub const MCDE_VSCRC1: c_uint = 0x00000C60;
pub const MCDE_VSCRC_VSPMIN_MASK: c_uint = 0x00000FFF;
pub const MCDE_VSCRC_VSPMAX_SHIFT: c_int = 12;
pub const MCDE_VSCRC_VSPMAX_MASK: c_uint = 0x00FFF000;
pub const MCDE_VSCRC_VSPDIV_SHIFT: c_int = 24;
pub const MCDE_VSCRC_VSPDIV_MASK: c_uint = 0x07000000;
pub const MCDE_VSCRC_VSPDIV_MCDECLK_DIV_1: c_int = 0;
pub const MCDE_VSCRC_VSPDIV_MCDECLK_DIV_2: c_int = 1;
pub const MCDE_VSCRC_VSPDIV_MCDECLK_DIV_4: c_int = 2;
pub const MCDE_VSCRC_VSPDIV_MCDECLK_DIV_8: c_int = 3;
pub const MCDE_VSCRC_VSPDIV_MCDECLK_DIV_16: c_int = 4;
pub const MCDE_VSCRC_VSPDIV_MCDECLK_DIV_32: c_int = 5;
pub const MCDE_VSCRC_VSPDIV_MCDECLK_DIV_64: c_int = 6;
pub const MCDE_VSCRC_VSPDIV_MCDECLK_DIV_128: c_int = 7;

// Channel config 0..3
pub const MCDE_CHNL0CONF: c_uint = 0x00000600;
pub const MCDE_CHNL1CONF: c_uint = 0x00000620;
pub const MCDE_CHNL2CONF: c_uint = 0x00000640;
pub const MCDE_CHNL3CONF: c_uint = 0x00000660;
pub const MCDE_CHNLXCONF_PPL_SHIFT: c_int = 0;
pub const MCDE_CHNLXCONF_PPL_MASK: c_uint = 0x000007FF;
pub const MCDE_CHNLXCONF_LPF_SHIFT: c_int = 16;
pub const MCDE_CHNLXCONF_LPF_MASK: c_uint = 0x07FF0000;
pub const MCDE_MAX_WIDTH: c_int = 2048;
// Channel status 0..3
pub const MCDE_CHNL0STAT: c_uint = 0x00000604;
pub const MCDE_CHNL1STAT: c_uint = 0x00000624;
pub const MCDE_CHNL2STAT: c_uint = 0x00000644;
pub const MCDE_CHNL3STAT: c_uint = 0x00000664;

// Sync settings for channel 0..3
pub const MCDE_CHNL0SYNCHMOD: c_uint = 0x00000608;
pub const MCDE_CHNL1SYNCHMOD: c_uint = 0x00000628;
pub const MCDE_CHNL2SYNCHMOD: c_uint = 0x00000648;
pub const MCDE_CHNL3SYNCHMOD: c_uint = 0x00000668;
pub const MCDE_CHNLXSYNCHMOD_SRC_SYNCH_SHIFT: c_int = 0;
pub const MCDE_CHNLXSYNCHMOD_SRC_SYNCH_MASK: c_uint = 0x00000003;
pub const MCDE_CHNLXSYNCHMOD_SRC_SYNCH_HARDWARE: c_int = 0;
pub const MCDE_CHNLXSYNCHMOD_SRC_SYNCH_NO_SYNCH: c_int = 1;
pub const MCDE_CHNLXSYNCHMOD_SRC_SYNCH_SOFTWARE: c_int = 2;
pub const MCDE_CHNLXSYNCHMOD_OUT_SYNCH_SRC_SHIFT: c_int = 2;
pub const MCDE_CHNLXSYNCHMOD_OUT_SYNCH_SRC_MASK: c_uint = 0x0000001C;
pub const MCDE_CHNLXSYNCHMOD_OUT_SYNCH_SRC_FORMATTER: c_int = 0;
pub const MCDE_CHNLXSYNCHMOD_OUT_SYNCH_SRC_TE0: c_int = 1;
pub const MCDE_CHNLXSYNCHMOD_OUT_SYNCH_SRC_TE1: c_int = 2;
// Software sync triggers for channel 0..3
pub const MCDE_CHNL0SYNCHSW: c_uint = 0x0000060C;
pub const MCDE_CHNL1SYNCHSW: c_uint = 0x0000062C;
pub const MCDE_CHNL2SYNCHSW: c_uint = 0x0000064C;
pub const MCDE_CHNL3SYNCHSW: c_uint = 0x0000066C;

pub const MCDE_CHNL0BCKGNDCOL: c_uint = 0x00000610;
pub const MCDE_CHNL1BCKGNDCOL: c_uint = 0x00000630;
pub const MCDE_CHNL2BCKGNDCOL: c_uint = 0x00000650;
pub const MCDE_CHNL3BCKGNDCOL: c_uint = 0x00000670;
pub const MCDE_CHNLXBCKGNDCOL_B_SHIFT: c_int = 0;
pub const MCDE_CHNLXBCKGNDCOL_B_MASK: c_uint = 0x000000FF;
pub const MCDE_CHNLXBCKGNDCOL_G_SHIFT: c_int = 8;
pub const MCDE_CHNLXBCKGNDCOL_G_MASK: c_uint = 0x0000FF00;
pub const MCDE_CHNLXBCKGNDCOL_R_SHIFT: c_int = 16;
pub const MCDE_CHNLXBCKGNDCOL_R_MASK: c_uint = 0x00FF0000;
pub const MCDE_CHNL0MUXING: c_uint = 0x00000614;
pub const MCDE_CHNL1MUXING: c_uint = 0x00000634;
pub const MCDE_CHNL2MUXING: c_uint = 0x00000654;
pub const MCDE_CHNL3MUXING: c_uint = 0x00000674;
pub const MCDE_CHNLXMUXING_FIFO_ID_FIFO_A: c_int = 0;
pub const MCDE_CHNLXMUXING_FIFO_ID_FIFO_B: c_int = 1;
pub const MCDE_CHNLXMUXING_FIFO_ID_FIFO_C0: c_int = 2;
pub const MCDE_CHNLXMUXING_FIFO_ID_FIFO_C1: c_int = 3;
// Pixel processing control registers for channel A B,
pub const MCDE_CRA0: c_uint = 0x00000800;
pub const MCDE_CRB0: c_uint = 0x00000A00;

pub const MCDE_CRX0_KEYCTRL_SHIFT: c_int = 7;
pub const MCDE_CRX0_KEYCTRL_MASK: c_uint = 0x00000380;
pub const MCDE_CRX0_KEYCTRL_OFF: c_int = 0;
pub const MCDE_CRX0_KEYCTRL_ALPHA_RGB: c_int = 1;
pub const MCDE_CRX0_KEYCTRL_RGB: c_int = 2;
pub const MCDE_CRX0_KEYCTRL_FALPHA_FRGB: c_int = 4;
pub const MCDE_CRX0_KEYCTRL_FRGB: c_int = 5;

pub const MCDE_CRX0_FLICKMODE_SHIFT: c_int = 11;
pub const MCDE_CRX0_FLICKMODE_MASK: c_uint = 0x00001800;
pub const MCDE_CRX0_FLICKMODE_FORCE_FILTER_0: c_int = 0;
pub const MCDE_CRX0_FLICKMODE_ADAPTIVE: c_int = 1;
pub const MCDE_CRX0_FLICKMODE_TEST_MODE: c_int = 2;

pub const MCDE_CRX0_ALPHABLEND_SHIFT: c_int = 16;
pub const MCDE_CRX0_ALPHABLEND_MASK: c_uint = 0x00FF0000;

pub const MCDE_CRA1: c_uint = 0x00000804;
pub const MCDE_CRB1: c_uint = 0x00000A04;
pub const MCDE_CRX1_PCD_SHIFT: c_int = 0;
pub const MCDE_CRX1_PCD_MASK: c_uint = 0x000003FF;
pub const MCDE_CRX1_PCD_BITS: c_int = 10;
pub const MCDE_CRX1_CLKSEL_SHIFT: c_int = 10;
pub const MCDE_CRX1_CLKSEL_MASK: c_uint = 0x00001C00;
pub const MCDE_CRX1_CLKSEL_CLKPLL72: c_int = 0;
pub const MCDE_CRX1_CLKSEL_CLKPLL27: c_int = 2;
pub const MCDE_CRX1_CLKSEL_TV1CLK: c_int = 3;
pub const MCDE_CRX1_CLKSEL_TV2CLK: c_int = 4;
pub const MCDE_CRX1_CLKSEL_MCDECLK: c_int = 5;
pub const MCDE_CRX1_CDWIN_SHIFT: c_int = 13;
pub const MCDE_CRX1_CDWIN_MASK: c_uint = 0x0001E000;
pub const MCDE_CRX1_CDWIN_8BPP_C1: c_int = 0;
pub const MCDE_CRX1_CDWIN_12BPP_C1: c_int = 1;
pub const MCDE_CRX1_CDWIN_12BPP_C2: c_int = 2;
pub const MCDE_CRX1_CDWIN_16BPP_C1: c_int = 3;
pub const MCDE_CRX1_CDWIN_16BPP_C2: c_int = 4;
pub const MCDE_CRX1_CDWIN_16BPP_C3: c_int = 5;
pub const MCDE_CRX1_CDWIN_18BPP_C1: c_int = 6;
pub const MCDE_CRX1_CDWIN_18BPP_C2: c_int = 7;
pub const MCDE_CRX1_CDWIN_24BPP: c_int = 8;
pub const MCDE_CRX1_OUTBPP_SHIFT: c_int = 25;
pub const MCDE_CRX1_OUTBPP_MASK: c_uint = 0x1E000000;
pub const MCDE_CRX1_OUTBPP_MONO1: c_int = 0;
pub const MCDE_CRX1_OUTBPP_MONO2: c_int = 1;
pub const MCDE_CRX1_OUTBPP_MONO4: c_int = 2;
pub const MCDE_CRX1_OUTBPP_MONO8: c_int = 3;
pub const MCDE_CRX1_OUTBPP_8BPP: c_int = 4;
pub const MCDE_CRX1_OUTBPP_12BPP: c_int = 5;
pub const MCDE_CRX1_OUTBPP_15BPP: c_int = 6;
pub const MCDE_CRX1_OUTBPP_16BPP: c_int = 7;
pub const MCDE_CRX1_OUTBPP_18BPP: c_int = 8;
pub const MCDE_CRX1_OUTBPP_24BPP: c_int = 9;

pub const MCDE_COLKEYA: c_uint = 0x00000808;
pub const MCDE_COLKEYB: c_uint = 0x00000A08;
pub const MCDE_FCOLKEYA: c_uint = 0x0000080C;
pub const MCDE_FCOLKEYB: c_uint = 0x00000A0C;
pub const MCDE_RGBCONV1A: c_uint = 0x00000810;
pub const MCDE_RGBCONV1B: c_uint = 0x00000A10;
pub const MCDE_RGBCONV2A: c_uint = 0x00000814;
pub const MCDE_RGBCONV2B: c_uint = 0x00000A14;
pub const MCDE_RGBCONV3A: c_uint = 0x00000818;
pub const MCDE_RGBCONV3B: c_uint = 0x00000A18;
pub const MCDE_RGBCONV4A: c_uint = 0x0000081C;
pub const MCDE_RGBCONV4B: c_uint = 0x00000A1C;
pub const MCDE_RGBCONV5A: c_uint = 0x00000820;
pub const MCDE_RGBCONV5B: c_uint = 0x00000A20;
pub const MCDE_RGBCONV6A: c_uint = 0x00000824;
pub const MCDE_RGBCONV6B: c_uint = 0x00000A24;
// Rotation
pub const MCDE_ROTACONF: c_uint = 0x0000087C;
pub const MCDE_ROTBCONF: c_uint = 0x00000A7C;
// Synchronization event configuration
pub const MCDE_SYNCHCONFA: c_uint = 0x00000880;
pub const MCDE_SYNCHCONFB: c_uint = 0x00000A80;
pub const MCDE_SYNCHCONF_HWREQVEVENT_SHIFT: c_int = 0;

// Channel A+B control registers
pub const MCDE_CTRLA: c_uint = 0x00000884;
pub const MCDE_CTRLB: c_uint = 0x00000A84;
pub const MCDE_CTRLX_FIFOWTRMRK_SHIFT: c_int = 0;
pub const MCDE_CTRLX_FIFOWTRMRK_MASK: c_uint = 0x000003FF;

pub const MCDE_CTRLX_FORMID_SHIFT: c_int = 16;
pub const MCDE_CTRLX_FORMID_MASK: c_uint = 0x00070000;
pub const MCDE_CTRLX_FORMID_DSI0VID: c_int = 0;
pub const MCDE_CTRLX_FORMID_DSI0CMD: c_int = 1;
pub const MCDE_CTRLX_FORMID_DSI1VID: c_int = 2;
pub const MCDE_CTRLX_FORMID_DSI1CMD: c_int = 3;
pub const MCDE_CTRLX_FORMID_DSI2VID: c_int = 4;
pub const MCDE_CTRLX_FORMID_DSI2CMD: c_int = 5;
pub const MCDE_CTRLX_FORMID_DPIA: c_int = 0;
pub const MCDE_CTRLX_FORMID_DPIB: c_int = 1;
pub const MCDE_CTRLX_FORMTYPE_SHIFT: c_int = 20;
pub const MCDE_CTRLX_FORMTYPE_MASK: c_uint = 0x00700000;
pub const MCDE_CTRLX_FORMTYPE_DPITV: c_int = 0;
pub const MCDE_CTRLX_FORMTYPE_DBI: c_int = 1;
pub const MCDE_CTRLX_FORMTYPE_DSI: c_int = 2;
pub const MCDE_DSIVID0CONF0: c_uint = 0x00000E00;
pub const MCDE_DSICMD0CONF0: c_uint = 0x00000E20;
pub const MCDE_DSIVID1CONF0: c_uint = 0x00000E40;
pub const MCDE_DSICMD1CONF0: c_uint = 0x00000E60;
pub const MCDE_DSIVID2CONF0: c_uint = 0x00000E80;
pub const MCDE_DSICMD2CONF0: c_uint = 0x00000EA0;
pub const MCDE_DSICONF0_BLANKING_SHIFT: c_int = 0;
pub const MCDE_DSICONF0_BLANKING_MASK: c_uint = 0x000000FF;
pub const MCDE_DSICONF0_VID_MODE_CMD: c_int = 0;

pub const MCDE_DSICONF0_PACKING_SHIFT: c_int = 20;
pub const MCDE_DSICONF0_PACKING_MASK: c_uint = 0x00700000;
pub const MCDE_DSICONF0_PACKING_RGB565: c_int = 0;
pub const MCDE_DSICONF0_PACKING_RGB666: c_int = 1;
pub const MCDE_DSICONF0_PACKING_RGB888: c_int = 2;
pub const MCDE_DSICONF0_PACKING_BGR888: c_int = 3;
pub const MCDE_DSICONF0_PACKING_HDTV: c_int = 4;
pub const MCDE_DSIVID0FRAME: c_uint = 0x00000E04;
pub const MCDE_DSICMD0FRAME: c_uint = 0x00000E24;
pub const MCDE_DSIVID1FRAME: c_uint = 0x00000E44;
pub const MCDE_DSICMD1FRAME: c_uint = 0x00000E64;
pub const MCDE_DSIVID2FRAME: c_uint = 0x00000E84;
pub const MCDE_DSICMD2FRAME: c_uint = 0x00000EA4;
pub const MCDE_DSIVID0PKT: c_uint = 0x00000E08;
pub const MCDE_DSICMD0PKT: c_uint = 0x00000E28;
pub const MCDE_DSIVID1PKT: c_uint = 0x00000E48;
pub const MCDE_DSICMD1PKT: c_uint = 0x00000E68;
pub const MCDE_DSIVID2PKT: c_uint = 0x00000E88;
pub const MCDE_DSICMD2PKT: c_uint = 0x00000EA8;
pub const MCDE_DSIVID0SYNC: c_uint = 0x00000E0C;
pub const MCDE_DSICMD0SYNC: c_uint = 0x00000E2C;
pub const MCDE_DSIVID1SYNC: c_uint = 0x00000E4C;
pub const MCDE_DSICMD1SYNC: c_uint = 0x00000E6C;
pub const MCDE_DSIVID2SYNC: c_uint = 0x00000E8C;
pub const MCDE_DSICMD2SYNC: c_uint = 0x00000EAC;
pub const MCDE_DSIVID0CMDW: c_uint = 0x00000E10;
pub const MCDE_DSICMD0CMDW: c_uint = 0x00000E30;
pub const MCDE_DSIVID1CMDW: c_uint = 0x00000E50;
pub const MCDE_DSICMD1CMDW: c_uint = 0x00000E70;
pub const MCDE_DSIVID2CMDW: c_uint = 0x00000E90;
pub const MCDE_DSICMD2CMDW: c_uint = 0x00000EB0;
pub const MCDE_DSIVIDXCMDW_CMDW_CONTINUE_SHIFT: c_int = 0;
pub const MCDE_DSIVIDXCMDW_CMDW_CONTINUE_MASK: c_uint = 0x0000FFFF;
pub const MCDE_DSIVIDXCMDW_CMDW_START_SHIFT: c_int = 16;
pub const MCDE_DSIVIDXCMDW_CMDW_START_MASK: c_uint = 0xFFFF0000;
pub const MCDE_DSIVID0DELAY0: c_uint = 0x00000E14;
pub const MCDE_DSICMD0DELAY0: c_uint = 0x00000E34;
pub const MCDE_DSIVID1DELAY0: c_uint = 0x00000E54;
pub const MCDE_DSICMD1DELAY0: c_uint = 0x00000E74;
pub const MCDE_DSIVID2DELAY0: c_uint = 0x00000E94;
pub const MCDE_DSICMD2DELAY0: c_uint = 0x00000EB4;
pub const MCDE_DSIVID0DELAY1: c_uint = 0x00000E18;
pub const MCDE_DSICMD0DELAY1: c_uint = 0x00000E38;
pub const MCDE_DSIVID1DELAY1: c_uint = 0x00000E58;
pub const MCDE_DSICMD1DELAY1: c_uint = 0x00000E78;
pub const MCDE_DSIVID2DELAY1: c_uint = 0x00000E98;
pub const MCDE_DSICMD2DELAY1: c_uint = 0x00000EB8;
