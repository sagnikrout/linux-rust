//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/gr/regs.h
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


// SPDX-License-Identifier: MIT
pub const NV04_PGRAPH_DEBUG_0: c_uint = 0x00400080;
pub const NV04_PGRAPH_DEBUG_1: c_uint = 0x00400084;
pub const NV04_PGRAPH_DEBUG_2: c_uint = 0x00400088;
pub const NV04_PGRAPH_DEBUG_3: c_uint = 0x0040008c;
pub const NV10_PGRAPH_DEBUG_4: c_uint = 0x00400090;
pub const NV03_PGRAPH_INTR: c_uint = 0x00400100;
pub const NV03_PGRAPH_NSTATUS: c_uint = 0x00400104;

pub const NV03_PGRAPH_NSOURCE: c_uint = 0x00400108;

pub const NV03_PGRAPH_INTR_EN: c_uint = 0x00400140;
pub const NV40_PGRAPH_INTR_EN: c_uint = 0x0040013C;

pub const NV10_PGRAPH_CTX_CONTROL: c_uint = 0x00400144;
pub const NV10_PGRAPH_CTX_USER: c_uint = 0x00400148;

pub const NV04_PGRAPH_CTX_SWITCH1: c_uint = 0x00400160;

pub const NV04_PGRAPH_CTX_SWITCH2: c_uint = 0x00400164;
pub const NV04_PGRAPH_CTX_SWITCH3: c_uint = 0x00400168;
pub const NV04_PGRAPH_CTX_SWITCH4: c_uint = 0x0040016C;
pub const NV04_PGRAPH_CTX_CONTROL: c_uint = 0x00400170;
pub const NV04_PGRAPH_CTX_USER: c_uint = 0x00400174;
pub const NV04_PGRAPH_CTX_CACHE1: c_uint = 0x00400180;
pub const NV03_PGRAPH_CTX_CONTROL: c_uint = 0x00400190;
pub const NV03_PGRAPH_CTX_USER: c_uint = 0x00400194;
pub const NV04_PGRAPH_CTX_CACHE2: c_uint = 0x004001A0;
pub const NV04_PGRAPH_CTX_CACHE3: c_uint = 0x004001C0;
pub const NV04_PGRAPH_CTX_CACHE4: c_uint = 0x004001E0;
pub const NV40_PGRAPH_CTXCTL_0304: c_uint = 0x00400304;
pub const NV40_PGRAPH_CTXCTL_0304_XFER_CTX: c_uint = 0x00000001;
pub const NV40_PGRAPH_CTXCTL_UCODE_STAT: c_uint = 0x00400308;
pub const NV40_PGRAPH_CTXCTL_UCODE_STAT_IP_MASK: c_uint = 0xff000000;
pub const NV40_PGRAPH_CTXCTL_UCODE_STAT_IP_SHIFT: c_int = 24;
pub const NV40_PGRAPH_CTXCTL_UCODE_STAT_OP_MASK: c_uint = 0x00ffffff;
pub const NV40_PGRAPH_CTXCTL_0310: c_uint = 0x00400310;
pub const NV40_PGRAPH_CTXCTL_0310_XFER_SAVE: c_uint = 0x00000020;
pub const NV40_PGRAPH_CTXCTL_0310_XFER_LOAD: c_uint = 0x00000040;
pub const NV40_PGRAPH_CTXCTL_030C: c_uint = 0x0040030c;
pub const NV40_PGRAPH_CTXCTL_UCODE_INDEX: c_uint = 0x00400324;
pub const NV40_PGRAPH_CTXCTL_UCODE_DATA: c_uint = 0x00400328;
pub const NV40_PGRAPH_CTXCTL_CUR: c_uint = 0x0040032c;
pub const NV40_PGRAPH_CTXCTL_CUR_LOADED: c_uint = 0x01000000;
pub const NV40_PGRAPH_CTXCTL_CUR_INSTANCE: c_uint = 0x000FFFFF;
pub const NV40_PGRAPH_CTXCTL_NEXT: c_uint = 0x00400330;
pub const NV40_PGRAPH_CTXCTL_NEXT_INSTANCE: c_uint = 0x000fffff;
pub const NV50_PGRAPH_CTXCTL_CUR: c_uint = 0x0040032c;
pub const NV50_PGRAPH_CTXCTL_CUR_LOADED: c_uint = 0x80000000;
pub const NV50_PGRAPH_CTXCTL_CUR_INSTANCE: c_uint = 0x00ffffff;
pub const NV50_PGRAPH_CTXCTL_NEXT: c_uint = 0x00400330;
pub const NV50_PGRAPH_CTXCTL_NEXT_INSTANCE: c_uint = 0x00ffffff;
pub const NV03_PGRAPH_ABS_X_RAM: c_uint = 0x00400400;
pub const NV03_PGRAPH_ABS_Y_RAM: c_uint = 0x00400480;
pub const NV03_PGRAPH_X_MISC: c_uint = 0x00400500;
pub const NV03_PGRAPH_Y_MISC: c_uint = 0x00400504;
pub const NV04_PGRAPH_VALID1: c_uint = 0x00400508;
pub const NV04_PGRAPH_SOURCE_COLOR: c_uint = 0x0040050C;
pub const NV04_PGRAPH_MISC24_0: c_uint = 0x00400510;
pub const NV03_PGRAPH_XY_LOGIC_MISC0: c_uint = 0x00400514;
pub const NV03_PGRAPH_XY_LOGIC_MISC1: c_uint = 0x00400518;
pub const NV03_PGRAPH_XY_LOGIC_MISC2: c_uint = 0x0040051C;
pub const NV03_PGRAPH_XY_LOGIC_MISC3: c_uint = 0x00400520;
pub const NV03_PGRAPH_CLIPX_0: c_uint = 0x00400524;
pub const NV03_PGRAPH_CLIPX_1: c_uint = 0x00400528;
pub const NV03_PGRAPH_CLIPY_0: c_uint = 0x0040052C;
pub const NV03_PGRAPH_CLIPY_1: c_uint = 0x00400530;
pub const NV03_PGRAPH_ABS_ICLIP_XMAX: c_uint = 0x00400534;
pub const NV03_PGRAPH_ABS_ICLIP_YMAX: c_uint = 0x00400538;
pub const NV03_PGRAPH_ABS_UCLIP_XMIN: c_uint = 0x0040053C;
pub const NV03_PGRAPH_ABS_UCLIP_YMIN: c_uint = 0x00400540;
pub const NV03_PGRAPH_ABS_UCLIP_XMAX: c_uint = 0x00400544;
pub const NV03_PGRAPH_ABS_UCLIP_YMAX: c_uint = 0x00400548;
pub const NV03_PGRAPH_ABS_UCLIPA_XMIN: c_uint = 0x00400560;
pub const NV03_PGRAPH_ABS_UCLIPA_YMIN: c_uint = 0x00400564;
pub const NV03_PGRAPH_ABS_UCLIPA_XMAX: c_uint = 0x00400568;
pub const NV03_PGRAPH_ABS_UCLIPA_YMAX: c_uint = 0x0040056C;
pub const NV04_PGRAPH_MISC24_1: c_uint = 0x00400570;
pub const NV04_PGRAPH_MISC24_2: c_uint = 0x00400574;
pub const NV04_PGRAPH_VALID2: c_uint = 0x00400578;
pub const NV04_PGRAPH_PASSTHRU_0: c_uint = 0x0040057C;
pub const NV04_PGRAPH_PASSTHRU_1: c_uint = 0x00400580;
pub const NV04_PGRAPH_PASSTHRU_2: c_uint = 0x00400584;
pub const NV10_PGRAPH_DIMX_TEXTURE: c_uint = 0x00400588;
pub const NV10_PGRAPH_WDIMX_TEXTURE: c_uint = 0x0040058C;
pub const NV04_PGRAPH_COMBINE_0_ALPHA: c_uint = 0x00400590;
pub const NV04_PGRAPH_COMBINE_0_COLOR: c_uint = 0x00400594;
pub const NV04_PGRAPH_COMBINE_1_ALPHA: c_uint = 0x00400598;
pub const NV04_PGRAPH_COMBINE_1_COLOR: c_uint = 0x0040059C;
pub const NV04_PGRAPH_FORMAT_0: c_uint = 0x004005A8;
pub const NV04_PGRAPH_FORMAT_1: c_uint = 0x004005AC;
pub const NV04_PGRAPH_FILTER_0: c_uint = 0x004005B0;
pub const NV04_PGRAPH_FILTER_1: c_uint = 0x004005B4;
pub const NV03_PGRAPH_MONO_COLOR0: c_uint = 0x00400600;
pub const NV04_PGRAPH_ROP3: c_uint = 0x00400604;
pub const NV04_PGRAPH_BETA_AND: c_uint = 0x00400608;
pub const NV04_PGRAPH_BETA_PREMULT: c_uint = 0x0040060C;
pub const NV04_PGRAPH_LIMIT_VIOL_PIX: c_uint = 0x00400610;
pub const NV04_PGRAPH_FORMATS: c_uint = 0x00400618;
pub const NV10_PGRAPH_DEBUG_2: c_uint = 0x00400620;
pub const NV04_PGRAPH_BOFFSET0: c_uint = 0x00400640;
pub const NV04_PGRAPH_BOFFSET1: c_uint = 0x00400644;
pub const NV04_PGRAPH_BOFFSET2: c_uint = 0x00400648;
pub const NV04_PGRAPH_BOFFSET3: c_uint = 0x0040064C;
pub const NV04_PGRAPH_BOFFSET4: c_uint = 0x00400650;
pub const NV04_PGRAPH_BOFFSET5: c_uint = 0x00400654;
pub const NV04_PGRAPH_BBASE0: c_uint = 0x00400658;
pub const NV04_PGRAPH_BBASE1: c_uint = 0x0040065C;
pub const NV04_PGRAPH_BBASE2: c_uint = 0x00400660;
pub const NV04_PGRAPH_BBASE3: c_uint = 0x00400664;
pub const NV04_PGRAPH_BBASE4: c_uint = 0x00400668;
pub const NV04_PGRAPH_BBASE5: c_uint = 0x0040066C;
pub const NV04_PGRAPH_BPITCH0: c_uint = 0x00400670;
pub const NV04_PGRAPH_BPITCH1: c_uint = 0x00400674;
pub const NV04_PGRAPH_BPITCH2: c_uint = 0x00400678;
pub const NV04_PGRAPH_BPITCH3: c_uint = 0x0040067C;
pub const NV04_PGRAPH_BPITCH4: c_uint = 0x00400680;
pub const NV04_PGRAPH_BLIMIT0: c_uint = 0x00400684;
pub const NV04_PGRAPH_BLIMIT1: c_uint = 0x00400688;
pub const NV04_PGRAPH_BLIMIT2: c_uint = 0x0040068C;
pub const NV04_PGRAPH_BLIMIT3: c_uint = 0x00400690;
pub const NV04_PGRAPH_BLIMIT4: c_uint = 0x00400694;
pub const NV04_PGRAPH_BLIMIT5: c_uint = 0x00400698;
pub const NV04_PGRAPH_BSWIZZLE2: c_uint = 0x0040069C;
pub const NV04_PGRAPH_BSWIZZLE5: c_uint = 0x004006A0;
pub const NV03_PGRAPH_STATUS: c_uint = 0x004006B0;
pub const NV04_PGRAPH_STATUS: c_uint = 0x00400700;

pub const NV04_PGRAPH_TRAPPED_ADDR: c_uint = 0x00400704;
pub const NV04_PGRAPH_TRAPPED_DATA: c_uint = 0x00400708;
pub const NV04_PGRAPH_SURFACE: c_uint = 0x0040070C;
pub const NV10_PGRAPH_TRAPPED_DATA_HIGH: c_uint = 0x0040070C;
pub const NV04_PGRAPH_STATE: c_uint = 0x00400710;
pub const NV10_PGRAPH_SURFACE: c_uint = 0x00400710;
pub const NV04_PGRAPH_NOTIFY: c_uint = 0x00400714;
pub const NV10_PGRAPH_STATE: c_uint = 0x00400714;
pub const NV10_PGRAPH_NOTIFY: c_uint = 0x00400718;
pub const NV04_PGRAPH_FIFO: c_uint = 0x00400720;
pub const NV04_PGRAPH_BPIXEL: c_uint = 0x00400724;
pub const NV10_PGRAPH_RDI_INDEX: c_uint = 0x00400750;
pub const NV04_PGRAPH_FFINTFC_ST2: c_uint = 0x00400754;
pub const NV10_PGRAPH_RDI_DATA: c_uint = 0x00400754;
pub const NV04_PGRAPH_DMA_PITCH: c_uint = 0x00400760;
pub const NV10_PGRAPH_FFINTFC_FIFO_PTR: c_uint = 0x00400760;
pub const NV04_PGRAPH_DVD_COLORFMT: c_uint = 0x00400764;
pub const NV10_PGRAPH_FFINTFC_ST2: c_uint = 0x00400764;
pub const NV04_PGRAPH_SCALED_FORMAT: c_uint = 0x00400768;
pub const NV10_PGRAPH_FFINTFC_ST2_DL: c_uint = 0x00400768;
pub const NV10_PGRAPH_FFINTFC_ST2_DH: c_uint = 0x0040076c;
pub const NV10_PGRAPH_DMA_PITCH: c_uint = 0x00400770;
pub const NV10_PGRAPH_DVD_COLORFMT: c_uint = 0x00400774;
pub const NV10_PGRAPH_SCALED_FORMAT: c_uint = 0x00400778;
pub const NV20_PGRAPH_CHANNEL_CTX_TABLE: c_uint = 0x00400780;
pub const NV20_PGRAPH_CHANNEL_CTX_POINTER: c_uint = 0x00400784;
pub const NV20_PGRAPH_CHANNEL_CTX_XFER: c_uint = 0x00400788;
pub const NV20_PGRAPH_CHANNEL_CTX_XFER_LOAD: c_uint = 0x00000001;
pub const NV20_PGRAPH_CHANNEL_CTX_XFER_SAVE: c_uint = 0x00000002;
pub const NV04_PGRAPH_PATT_COLOR0: c_uint = 0x00400800;
pub const NV04_PGRAPH_PATT_COLOR1: c_uint = 0x00400804;
pub const NV04_PGRAPH_PATTERN: c_uint = 0x00400808;
pub const NV04_PGRAPH_PATTERN_SHAPE: c_uint = 0x00400810;
pub const NV04_PGRAPH_CHROMA: c_uint = 0x00400814;
pub const NV04_PGRAPH_CONTROL0: c_uint = 0x00400818;
pub const NV04_PGRAPH_CONTROL1: c_uint = 0x0040081C;
pub const NV04_PGRAPH_CONTROL2: c_uint = 0x00400820;
pub const NV04_PGRAPH_BLEND: c_uint = 0x00400824;
pub const NV04_PGRAPH_STORED_FMT: c_uint = 0x00400830;
pub const NV04_PGRAPH_PATT_COLORRAM: c_uint = 0x00400900;

pub const NV04_PGRAPH_U_RAM: c_uint = 0x00400D00;

pub const NV04_PGRAPH_V_RAM: c_uint = 0x00400D40;
pub const NV04_PGRAPH_W_RAM: c_uint = 0x00400D80;

pub const NV10_PGRAPH_COMBINER0_IN_ALPHA: c_uint = 0x00400E40;
pub const NV10_PGRAPH_COMBINER1_IN_ALPHA: c_uint = 0x00400E44;
pub const NV10_PGRAPH_COMBINER0_IN_RGB: c_uint = 0x00400E48;
pub const NV10_PGRAPH_COMBINER1_IN_RGB: c_uint = 0x00400E4C;
pub const NV10_PGRAPH_COMBINER_COLOR0: c_uint = 0x00400E50;
pub const NV10_PGRAPH_COMBINER_COLOR1: c_uint = 0x00400E54;
pub const NV10_PGRAPH_COMBINER0_OUT_ALPHA: c_uint = 0x00400E58;
pub const NV10_PGRAPH_COMBINER1_OUT_ALPHA: c_uint = 0x00400E5C;
pub const NV10_PGRAPH_COMBINER0_OUT_RGB: c_uint = 0x00400E60;
pub const NV10_PGRAPH_COMBINER1_OUT_RGB: c_uint = 0x00400E64;
pub const NV10_PGRAPH_COMBINER_FINAL0: c_uint = 0x00400E68;
pub const NV10_PGRAPH_COMBINER_FINAL1: c_uint = 0x00400E6C;
pub const NV10_PGRAPH_WINDOWCLIP_HORIZONTAL: c_uint = 0x00400F00;
pub const NV10_PGRAPH_WINDOWCLIP_VERTICAL: c_uint = 0x00400F20;
pub const NV10_PGRAPH_XFMODE0: c_uint = 0x00400F40;
pub const NV10_PGRAPH_XFMODE1: c_uint = 0x00400F44;
pub const NV10_PGRAPH_GLOBALSTATE0: c_uint = 0x00400F48;
pub const NV10_PGRAPH_GLOBALSTATE1: c_uint = 0x00400F4C;
pub const NV10_PGRAPH_PIPE_ADDRESS: c_uint = 0x00400F50;
pub const NV10_PGRAPH_PIPE_DATA: c_uint = 0x00400F54;
pub const NV04_PGRAPH_DMA_START_0: c_uint = 0x00401000;
pub const NV04_PGRAPH_DMA_START_1: c_uint = 0x00401004;
pub const NV04_PGRAPH_DMA_LENGTH: c_uint = 0x00401008;
pub const NV04_PGRAPH_DMA_MISC: c_uint = 0x0040100C;
pub const NV04_PGRAPH_DMA_DATA_0: c_uint = 0x00401020;
pub const NV04_PGRAPH_DMA_DATA_1: c_uint = 0x00401024;
pub const NV04_PGRAPH_DMA_RM: c_uint = 0x00401030;
pub const NV04_PGRAPH_DMA_A_XLATE_INST: c_uint = 0x00401040;
pub const NV04_PGRAPH_DMA_A_CONTROL: c_uint = 0x00401044;
pub const NV04_PGRAPH_DMA_A_LIMIT: c_uint = 0x00401048;
pub const NV04_PGRAPH_DMA_A_TLB_PTE: c_uint = 0x0040104C;
pub const NV04_PGRAPH_DMA_A_TLB_TAG: c_uint = 0x00401050;
pub const NV04_PGRAPH_DMA_A_ADJ_OFFSET: c_uint = 0x00401054;
pub const NV04_PGRAPH_DMA_A_OFFSET: c_uint = 0x00401058;
pub const NV04_PGRAPH_DMA_A_SIZE: c_uint = 0x0040105C;
pub const NV04_PGRAPH_DMA_A_Y_SIZE: c_uint = 0x00401060;
pub const NV04_PGRAPH_DMA_B_XLATE_INST: c_uint = 0x00401080;
pub const NV04_PGRAPH_DMA_B_CONTROL: c_uint = 0x00401084;
pub const NV04_PGRAPH_DMA_B_LIMIT: c_uint = 0x00401088;
pub const NV04_PGRAPH_DMA_B_TLB_PTE: c_uint = 0x0040108C;
pub const NV04_PGRAPH_DMA_B_TLB_TAG: c_uint = 0x00401090;
pub const NV04_PGRAPH_DMA_B_ADJ_OFFSET: c_uint = 0x00401094;
pub const NV04_PGRAPH_DMA_B_OFFSET: c_uint = 0x00401098;
pub const NV04_PGRAPH_DMA_B_SIZE: c_uint = 0x0040109C;
pub const NV04_PGRAPH_DMA_B_Y_SIZE: c_uint = 0x004010A0;

