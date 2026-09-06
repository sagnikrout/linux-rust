//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nouveau_reg.h
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
pub const NV04_PFB_BOOT_0: c_uint = 0x00100000;

pub const NV04_PFB_DEBUG_0: c_uint = 0x00100080;

pub const NV04_PFB_CFG0: c_uint = 0x00100200;

pub const NV04_PFB_CFG1: c_uint = 0x00100204;
pub const NV04_PFB_FIFO_DATA: c_uint = 0x0010020c;

pub const NV10_PFB_REFCTRL: c_uint = 0x00100210;

pub const NV04_PFB_PAD: c_uint = 0x0010021c;

pub const NV10_PFB_TILE__SIZE: c_int = 8;

pub const NV04_PFB_REF: c_uint = 0x001002d0;

pub const NV04_PFB_PRE: c_uint = 0x001002d4;

pub const NV10_PFB_CLOSE_PAGE2: c_uint = 0x0010033c;

pub const NV40_PFB_TILE__SIZE_0: c_int = 12;
pub const NV40_PFB_TILE__SIZE_1: c_int = 15;

pub const NV40_PFB_UNK_800: c_uint = 0x00100800;
pub const NV_PEXTDEV_BOOT_0: c_uint = 0x00101000;
pub const NV_PEXTDEV_BOOT_0_RAMCFG: c_uint = 0x0000003c;

pub const NV_PEXTDEV_BOOT_3: c_uint = 0x0010100c;
pub const NV_RAMIN: c_uint = 0x00700000;
pub const NV_RAMHT_HANDLE_OFFSET: c_int = 0;
pub const NV_RAMHT_CONTEXT_OFFSET: c_int = 4;

// Some object classes we care about in the drm
pub const NV_CLASS_DMA_FROM_MEMORY: c_uint = 0x00000002;
pub const NV_CLASS_DMA_TO_MEMORY: c_uint = 0x00000003;
pub const NV_CLASS_NULL: c_uint = 0x00000030;
pub const NV_CLASS_DMA_IN_MEMORY: c_uint = 0x0000003D;

pub const NV03_USER__SIZE: c_int = 16;
pub const NV10_USER__SIZE: c_int = 32;
pub const NV03_USER_SIZE: c_uint = 0x00010000;

pub const NV03_USER_DMA_PUT__SIZE: c_int = 16;
pub const NV10_USER_DMA_PUT__SIZE: c_int = 32;

pub const NV03_USER_DMA_GET__SIZE: c_int = 16;
pub const NV10_USER_DMA_GET__SIZE: c_int = 32;

pub const NV03_USER_REF_CNT__SIZE: c_int = 16;
pub const NV10_USER_REF_CNT__SIZE: c_int = 32;

pub const NV40_USER_SIZE: c_uint = 0x00001000;

pub const NV40_USER_DMA_PUT__SIZE: c_int = 32;

pub const NV40_USER_DMA_GET__SIZE: c_int = 32;

pub const NV40_USER_REF_CNT__SIZE: c_int = 32;

pub const NV50_USER_SIZE: c_uint = 0x00002000;

pub const NV50_USER_DMA_PUT__SIZE: c_int = 128;

pub const NV50_USER_DMA_GET__SIZE: c_int = 128;

pub const NV50_USER_REF_CNT__SIZE: c_int = 128;
pub const NV03_FIFO_SIZE: c_uint = 0x8000UL;
pub const NV03_PMC_BOOT_0: c_uint = 0x00000000;
pub const NV03_PMC_BOOT_1: c_uint = 0x00000004;
pub const NV03_PMC_INTR_0: c_uint = 0x00000100;

pub const NV03_PMC_INTR_EN_0: c_uint = 0x00000140;

pub const NV03_PMC_ENABLE: c_uint = 0x00000200;

// Disabling the below bit breaks newer (G7X only?) mobile chipsets,
// the card will hang early on in the X init process.
//

pub const NV40_PMC_GRAPH_UNITS: c_uint = 0x00001540;
pub const NV40_PMC_BACKLIGHT: c_uint = 0x000015f0;

pub const NV40_PMC_1700: c_uint = 0x00001700;
pub const NV40_PMC_1704: c_uint = 0x00001704;
pub const NV40_PMC_1708: c_uint = 0x00001708;
pub const NV40_PMC_170C: c_uint = 0x0000170C;
// probably PMC ?
pub const NV50_PUNK_BAR0_PRAMIN: c_uint = 0x00001700;
pub const NV50_PUNK_BAR_CFG_BASE: c_uint = 0x00001704;

pub const NV50_PUNK_BAR1_CTXDMA: c_uint = 0x00001708;

pub const NV50_PUNK_BAR3_CTXDMA: c_uint = 0x0000170C;

pub const NV50_PUNK_UNK1710: c_uint = 0x00001710;
pub const NV04_PBUS_PCI_NV_1: c_uint = 0x00001804;
pub const NV04_PBUS_PCI_NV_19: c_uint = 0x0000184C;
pub const NV04_PBUS_PCI_NV_20: c_uint = 0x00001850;

pub const NV04_PTIMER_INTR_0: c_uint = 0x00009100;
pub const NV04_PTIMER_INTR_EN_0: c_uint = 0x00009140;
pub const NV04_PTIMER_NUMERATOR: c_uint = 0x00009200;
pub const NV04_PTIMER_DENOMINATOR: c_uint = 0x00009210;
pub const NV04_PTIMER_TIME_0: c_uint = 0x00009400;
pub const NV04_PTIMER_TIME_1: c_uint = 0x00009410;
pub const NV04_PTIMER_ALARM_0: c_uint = 0x00009420;
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

// It's a guess that this works on NV03. Confirmed on NV04, though
pub const NV04_PFIFO_DELAY_0: c_uint = 0x00002040;
pub const NV04_PFIFO_DMA_TIMESLICE: c_uint = 0x00002044;
pub const NV04_PFIFO_NEXT_CHANNEL: c_uint = 0x00002050;
pub const NV03_PFIFO_INTR_0: c_uint = 0x00002100;
pub const NV03_PFIFO_INTR_EN_0: c_uint = 0x00002140;

pub const NV03_PFIFO_RAMHT: c_uint = 0x00002210;
pub const NV03_PFIFO_RAMFC: c_uint = 0x00002214;
pub const NV03_PFIFO_RAMRO: c_uint = 0x00002218;
pub const NV40_PFIFO_RAMFC: c_uint = 0x00002220;
pub const NV03_PFIFO_CACHES: c_uint = 0x00002500;
pub const NV04_PFIFO_MODE: c_uint = 0x00002504;
pub const NV04_PFIFO_DMA: c_uint = 0x00002508;
pub const NV04_PFIFO_SIZE: c_uint = 0x0000250c;

pub const NV50_PFIFO_CTX_TABLE__SIZE: c_int = 128;

pub const NV50_PFIFO_CTX_TABLE_INSTANCE_MASK_G80: c_uint = 0x0FFFFFFF;
pub const NV50_PFIFO_CTX_TABLE_INSTANCE_MASK_G84: c_uint = 0x00FFFFFF;
pub const NV03_PFIFO_CACHE0_PUSH0: c_uint = 0x00003000;
pub const NV03_PFIFO_CACHE0_PULL0: c_uint = 0x00003040;
pub const NV04_PFIFO_CACHE0_PULL0: c_uint = 0x00003050;
pub const NV04_PFIFO_CACHE0_PULL1: c_uint = 0x00003054;
pub const NV03_PFIFO_CACHE1_PUSH0: c_uint = 0x00003200;
pub const NV03_PFIFO_CACHE1_PUSH1: c_uint = 0x00003204;

pub const NV03_PFIFO_CACHE1_PUSH1_CHID_MASK: c_uint = 0x0000000f;
pub const NV10_PFIFO_CACHE1_PUSH1_CHID_MASK: c_uint = 0x0000001f;
pub const NV50_PFIFO_CACHE1_PUSH1_CHID_MASK: c_uint = 0x0000007f;
pub const NV03_PFIFO_CACHE1_PUT: c_uint = 0x00003210;
pub const NV04_PFIFO_CACHE1_DMA_PUSH: c_uint = 0x00003220;
pub const NV04_PFIFO_CACHE1_DMA_FETCH: c_uint = 0x00003224;

pub const NV04_PFIFO_CACHE1_DMA_STATE: c_uint = 0x00003228;
pub const NV04_PFIFO_CACHE1_DMA_INSTANCE: c_uint = 0x0000322c;
pub const NV04_PFIFO_CACHE1_DMA_CTL: c_uint = 0x00003230;
pub const NV04_PFIFO_CACHE1_DMA_PUT: c_uint = 0x00003240;
pub const NV04_PFIFO_CACHE1_DMA_GET: c_uint = 0x00003244;
pub const NV10_PFIFO_CACHE1_REF_CNT: c_uint = 0x00003248;
pub const NV10_PFIFO_CACHE1_DMA_SUBROUTINE: c_uint = 0x0000324C;
pub const NV03_PFIFO_CACHE1_PULL0: c_uint = 0x00003240;
pub const NV04_PFIFO_CACHE1_PULL0: c_uint = 0x00003250;

pub const NV03_PFIFO_CACHE1_PULL1: c_uint = 0x00003250;
pub const NV04_PFIFO_CACHE1_PULL1: c_uint = 0x00003254;
pub const NV04_PFIFO_CACHE1_HASH: c_uint = 0x00003258;
pub const NV10_PFIFO_CACHE1_ACQUIRE_TIMEOUT: c_uint = 0x00003260;
pub const NV10_PFIFO_CACHE1_ACQUIRE_TIMESTAMP: c_uint = 0x00003264;
pub const NV10_PFIFO_CACHE1_ACQUIRE_VALUE: c_uint = 0x00003268;
pub const NV10_PFIFO_CACHE1_SEMAPHORE: c_uint = 0x0000326C;
pub const NV03_PFIFO_CACHE1_GET: c_uint = 0x00003270;
pub const NV04_PFIFO_CACHE1_ENGINE: c_uint = 0x00003280;
pub const NV04_PFIFO_CACHE1_DMA_DCOUNT: c_uint = 0x000032A0;
pub const NV40_PFIFO_GRCTX_INSTANCE: c_uint = 0x000032E0;
pub const NV40_PFIFO_UNK32E4: c_uint = 0x000032E4;

pub const NV_CRTC0_INTSTAT: c_uint = 0x00600100;
pub const NV_CRTC0_INTEN: c_uint = 0x00600140;
pub const NV_CRTC1_INTSTAT: c_uint = 0x00602100;
pub const NV_CRTC1_INTEN: c_uint = 0x00602140;

pub const NV04_PRAMIN: c_uint = 0x00700000;
// Fifo commands. These are not regs, neither masks
pub const NV03_FIFO_CMD_JUMP: c_uint = 0x20000000;
pub const NV03_FIFO_CMD_JUMP_OFFSET_MASK: c_uint = 0x1ffffffc;

// This is a partial import from rules-ng, a few things may be duplicated.
// Eventually we should completely import everything from rules-ng.
// For the moment check rules-ng for docs.
//
pub const NV50_PMC: c_uint = 0x00000000;
pub const NV50_PMC__LEN: c_uint = 0x1;
pub const NV50_PMC__ESIZE: c_uint = 0x2000;

pub const NV50_PCONNECTOR: c_uint = 0x0000e000;
pub const NV50_PCONNECTOR__LEN: c_uint = 0x1;
pub const NV50_PCONNECTOR__ESIZE: c_uint = 0x1000;

pub const NV50_AUXCH_DATA_OUT__SIZE: c_int = 4;

pub const NV50_AUXCH_DATA_IN__SIZE: c_int = 4;

pub const NV50_AUXCH_CTRL_LINKSTAT: c_uint = 0x01000000;
pub const NV50_AUXCH_CTRL_LINKSTAT_NOT_READY: c_uint = 0x00000000;
pub const NV50_AUXCH_CTRL_LINKSTAT_READY: c_uint = 0x01000000;
pub const NV50_AUXCH_CTRL_LINKEN: c_uint = 0x00100000;
pub const NV50_AUXCH_CTRL_LINKEN_DISABLED: c_uint = 0x00000000;
pub const NV50_AUXCH_CTRL_LINKEN_ENABLED: c_uint = 0x00100000;
pub const NV50_AUXCH_CTRL_EXEC: c_uint = 0x00010000;
pub const NV50_AUXCH_CTRL_EXEC_COMPLETE: c_uint = 0x00000000;
pub const NV50_AUXCH_CTRL_EXEC_IN_PROCESS: c_uint = 0x00010000;
pub const NV50_AUXCH_CTRL_CMD: c_uint = 0x0000f000;
pub const NV50_AUXCH_CTRL_CMD_SHIFT: c_int = 12;
pub const NV50_AUXCH_CTRL_LEN: c_uint = 0x0000000f;
pub const NV50_AUXCH_CTRL_LEN_SHIFT: c_int = 0;

pub const NV50_AUXCH_STAT_STATE: c_uint = 0x10000000;
pub const NV50_AUXCH_STAT_STATE_NOT_READY: c_uint = 0x00000000;
pub const NV50_AUXCH_STAT_STATE_READY: c_uint = 0x10000000;
pub const NV50_AUXCH_STAT_REPLY: c_uint = 0x000f0000;
pub const NV50_AUXCH_STAT_REPLY_AUX: c_uint = 0x00030000;
pub const NV50_AUXCH_STAT_REPLY_AUX_ACK: c_uint = 0x00000000;
pub const NV50_AUXCH_STAT_REPLY_AUX_NACK: c_uint = 0x00010000;
pub const NV50_AUXCH_STAT_REPLY_AUX_DEFER: c_uint = 0x00020000;
pub const NV50_AUXCH_STAT_REPLY_I2C: c_uint = 0x000c0000;
pub const NV50_AUXCH_STAT_REPLY_I2C_ACK: c_uint = 0x00000000;
pub const NV50_AUXCH_STAT_REPLY_I2C_NACK: c_uint = 0x00040000;
pub const NV50_AUXCH_STAT_REPLY_I2C_DEFER: c_uint = 0x00080000;
pub const NV50_AUXCH_STAT_COUNT: c_uint = 0x0000001f;
pub const NV50_PBUS: c_uint = 0x00088000;
pub const NV50_PBUS__LEN: c_uint = 0x1;
pub const NV50_PBUS__ESIZE: c_uint = 0x1000;

pub const NV50_PFB: c_uint = 0x00100000;
pub const NV50_PFB__LEN: c_uint = 0x1;
pub const NV50_PFB__ESIZE: c_uint = 0x1000;
pub const NV50_PEXTDEV: c_uint = 0x00101000;
pub const NV50_PEXTDEV__LEN: c_uint = 0x1;
pub const NV50_PEXTDEV__ESIZE: c_uint = 0x1000;
pub const NV50_PROM: c_uint = 0x00300000;
pub const NV50_PROM__LEN: c_uint = 0x1;
pub const NV50_PROM__ESIZE: c_uint = 0x10000;
pub const NV50_PGRAPH: c_uint = 0x00400000;
pub const NV50_PGRAPH__LEN: c_uint = 0x1;
pub const NV50_PGRAPH__ESIZE: c_uint = 0x10000;
pub const NV50_PDISPLAY: c_uint = 0x00610000;
pub const NV50_PDISPLAY_OBJECTS: c_uint = 0x00610010;
pub const NV50_PDISPLAY_INTR_0: c_uint = 0x00610020;
pub const NV50_PDISPLAY_INTR_1: c_uint = 0x00610024;
pub const NV50_PDISPLAY_INTR_1_VBLANK_CRTC: c_uint = 0x0000000c;
pub const NV50_PDISPLAY_INTR_1_VBLANK_CRTC_SHIFT: c_int = 2;

pub const NV50_PDISPLAY_INTR_1_VBLANK_CRTC_0: c_uint = 0x00000004;
pub const NV50_PDISPLAY_INTR_1_VBLANK_CRTC_1: c_uint = 0x00000008;
pub const NV50_PDISPLAY_INTR_1_CLK_UNK10: c_uint = 0x00000010;
pub const NV50_PDISPLAY_INTR_1_CLK_UNK20: c_uint = 0x00000020;
pub const NV50_PDISPLAY_INTR_1_CLK_UNK40: c_uint = 0x00000040;
pub const NV50_PDISPLAY_INTR_EN_0: c_uint = 0x00610028;
pub const NV50_PDISPLAY_INTR_EN_1: c_uint = 0x0061002c;
pub const NV50_PDISPLAY_INTR_EN_1_VBLANK_CRTC: c_uint = 0x0000000c;

pub const NV50_PDISPLAY_INTR_EN_1_VBLANK_CRTC_0: c_uint = 0x00000004;
pub const NV50_PDISPLAY_INTR_EN_1_VBLANK_CRTC_1: c_uint = 0x00000008;
pub const NV50_PDISPLAY_INTR_EN_1_CLK_UNK10: c_uint = 0x00000010;
pub const NV50_PDISPLAY_INTR_EN_1_CLK_UNK20: c_uint = 0x00000020;
pub const NV50_PDISPLAY_INTR_EN_1_CLK_UNK40: c_uint = 0x00000040;
pub const NV50_PDISPLAY_UNK30_CTRL: c_uint = 0x00610030;
pub const NV50_PDISPLAY_UNK30_CTRL_UPDATE_VCLK0: c_uint = 0x00000200;
pub const NV50_PDISPLAY_UNK30_CTRL_UPDATE_VCLK1: c_uint = 0x00000400;
pub const NV50_PDISPLAY_UNK30_CTRL_PENDING: c_uint = 0x80000000;

pub const NV50_PDISPLAY_EVO_CTRL_DMA: c_uint = 0x00000010;
pub const NV50_PDISPLAY_EVO_CTRL_DMA_DISABLED: c_uint = 0x00000000;
pub const NV50_PDISPLAY_EVO_CTRL_DMA_ENABLED: c_uint = 0x00000010;

pub const NV50_PDISPLAY_EVO_DMA_CB_LOCATION: c_uint = 0x00000002;
pub const NV50_PDISPLAY_EVO_DMA_CB_LOCATION_VRAM: c_uint = 0x00000000;
pub const NV50_PDISPLAY_EVO_DMA_CB_LOCATION_SYSTEM: c_uint = 0x00000002;
pub const NV50_PDISPLAY_EVO_DMA_CB_VALID: c_uint = 0x00000001;

pub const NV50_PDISPLAY_CURSOR: c_uint = 0x00610270;

pub const NV50_PDISPLAY_CURSOR_CURSOR_CTRL2_ON: c_uint = 0x00000001;
pub const NV50_PDISPLAY_CURSOR_CURSOR_CTRL2_STATUS: c_uint = 0x00030000;
pub const NV50_PDISPLAY_CURSOR_CURSOR_CTRL2_STATUS_ACTIVE: c_uint = 0x00010000;
pub const NV50_PDISPLAY_PIO_CTRL: c_uint = 0x00610300;
pub const NV50_PDISPLAY_PIO_CTRL_PENDING: c_uint = 0x80000000;
pub const NV50_PDISPLAY_PIO_CTRL_MTHD: c_uint = 0x00001ffc;
pub const NV50_PDISPLAY_PIO_CTRL_ENABLED: c_uint = 0x00000001;
pub const NV50_PDISPLAY_PIO_DATA: c_uint = 0x00610304;

pub const NV50_PDISPLAY_CRTC_CLUT_MODE: c_uint = 0x00610a24;
pub const NV50_PDISPLAY_CRTC_INTERLACE: c_uint = 0x00610a48;
pub const NV50_PDISPLAY_CRTC_SCALE_CTRL: c_uint = 0x00610a50;
pub const NV50_PDISPLAY_CRTC_CURSOR_CTRL: c_uint = 0x00610a58;

pub const NV50_PDISPLAY_CRTC_UNK0AB8: c_uint = 0x00610ab8;
pub const NV50_PDISPLAY_CRTC_DEPTH: c_uint = 0x00610ac8;
pub const NV50_PDISPLAY_CRTC_CLOCK: c_uint = 0x00610ad0;
pub const NV50_PDISPLAY_CRTC_COLOR_CTRL: c_uint = 0x00610ae0;
pub const NV50_PDISPLAY_CRTC_SYNC_START_TO_BLANK_END: c_uint = 0x00610ae8;
pub const NV50_PDISPLAY_CRTC_MODE_UNK1: c_uint = 0x00610af0;
pub const NV50_PDISPLAY_CRTC_DISPLAY_TOTAL: c_uint = 0x00610af8;
pub const NV50_PDISPLAY_CRTC_SYNC_DURATION: c_uint = 0x00610b00;
pub const NV50_PDISPLAY_CRTC_MODE_UNK2: c_uint = 0x00610b08;

pub const NV50_PDISPLAY_CRTC_FB_SIZE: c_uint = 0x00610b18;
pub const NV50_PDISPLAY_CRTC_FB_PITCH: c_uint = 0x00610b20;
pub const NV50_PDISPLAY_CRTC_FB_PITCH_LINEAR: c_uint = 0x00100000;
pub const NV50_PDISPLAY_CRTC_FB_POS: c_uint = 0x00610b28;
pub const NV50_PDISPLAY_CRTC_SCALE_CENTER_OFFSET: c_uint = 0x00610b38;
pub const NV50_PDISPLAY_CRTC_REAL_RES: c_uint = 0x00610b40;
pub const NV50_PDISPLAY_CRTC_SCALE_RES1: c_uint = 0x00610b48;
pub const NV50_PDISPLAY_CRTC_SCALE_RES2: c_uint = 0x00610b50;

pub const NV50_PDISPLAY_CRTC_CLK: c_uint = 0x00614000;

pub const NV50_PDISPLAY_CRTC_CLK_CTRL1_CONNECTED: c_uint = 0x00000600;

pub const NV50_PDISPLAY_DAC_CLK: c_uint = 0x00614000;

pub const NV50_PDISPLAY_SOR_CLK: c_uint = 0x00614000;

pub const NV50_PDISPLAY_DAC: c_uint = 0x0061a000;

pub const NV50_PDISPLAY_DAC_DPMS_CTRL_HSYNC_OFF: c_uint = 0x00000001;
pub const NV50_PDISPLAY_DAC_DPMS_CTRL_VSYNC_OFF: c_uint = 0x00000004;
pub const NV50_PDISPLAY_DAC_DPMS_CTRL_BLANKED: c_uint = 0x00000010;
pub const NV50_PDISPLAY_DAC_DPMS_CTRL_OFF: c_uint = 0x00000040;
pub const NV50_PDISPLAY_DAC_DPMS_CTRL_PENDING: c_uint = 0x80000000;

pub const NV50_PDISPLAY_DAC_LOAD_CTRL_ACTIVE: c_uint = 0x00100000;
pub const NV50_PDISPLAY_DAC_LOAD_CTRL_PRESENT: c_uint = 0x38000000;
pub const NV50_PDISPLAY_DAC_LOAD_CTRL_DONE: c_uint = 0x80000000;

pub const NV50_PDISPLAY_DAC_CLK_CTRL1_CONNECTED: c_uint = 0x00000600;
pub const NV50_PDISPLAY_SOR: c_uint = 0x0061c000;

pub const NV50_PDISPLAY_SOR_DPMS_CTRL_PENDING: c_uint = 0x80000000;
pub const NV50_PDISPLAY_SOR_DPMS_CTRL_ON: c_uint = 0x00000001;

pub const NV50_PDISPLAY_SOR_CLK_CTRL1_CONNECTED: c_uint = 0x00000600;

pub const NV50_PDISPLAY_SOR_DPMS_STATE_ACTIVE: c_uint = 0x00030000;
pub const NV50_PDISPLAY_SOR_DPMS_STATE_BLANKED: c_uint = 0x00080000;
pub const NV50_PDISPLAY_SOR_DPMS_STATE_WAIT: c_uint = 0x10000000;

pub const NV50_PDISP_SOR_PWM_CTL_NEW: c_uint = 0x80000000;
pub const NVA3_PDISP_SOR_PWM_CTL_UNK: c_uint = 0x40000000;
pub const NV50_PDISP_SOR_PWM_CTL_VAL: c_uint = 0x000007ff;
pub const NVA3_PDISP_SOR_PWM_CTL_VAL: c_uint = 0x00ffffff;

pub const NV50_SOR_DP_CTRL_ENABLED: c_uint = 0x00000001;
pub const NV50_SOR_DP_CTRL_ENHANCED_FRAME_ENABLED: c_uint = 0x00004000;
pub const NV50_SOR_DP_CTRL_LANE_MASK: c_uint = 0x001f0000;
pub const NV50_SOR_DP_CTRL_LANE_0_ENABLED: c_uint = 0x00010000;
pub const NV50_SOR_DP_CTRL_LANE_1_ENABLED: c_uint = 0x00020000;
pub const NV50_SOR_DP_CTRL_LANE_2_ENABLED: c_uint = 0x00040000;
pub const NV50_SOR_DP_CTRL_LANE_3_ENABLED: c_uint = 0x00080000;
pub const NV50_SOR_DP_CTRL_TRAINING_PATTERN: c_uint = 0x0f000000;
pub const NV50_SOR_DP_CTRL_TRAINING_PATTERN_DISABLED: c_uint = 0x00000000;
pub const NV50_SOR_DP_CTRL_TRAINING_PATTERN_1: c_uint = 0x01000000;
pub const NV50_SOR_DP_CTRL_TRAINING_PATTERN_2: c_uint = 0x02000000;

pub const NV50_PDISPLAY_CURSOR_USER: c_uint = 0x00647000;

