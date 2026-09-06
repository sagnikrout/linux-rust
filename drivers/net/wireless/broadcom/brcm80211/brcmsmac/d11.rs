//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmsmac/d11.h
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


//
// Copyright (c) 2010 Broadcom Corporation
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

// RX FIFO numbers

// TX FIFO numbers using WME Access Category

// Addr is byte address used by SW; offset is word offset used by uCode
// Per AC TX limit settings

// Legacy TX FIFO numbers

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intctrlregs {
    pub intstatus: u32,
    pub intmask: u32,
}

// PIO structure,
// support two PIO format: 2 bytes access and 4 bytes access
// basic FIFO register set is per channel(transmit or receive)
// a pair of channels is defined for convenience
//
// 2byte-wide pio register set per channel(xmt or rcv)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pio2regs {
    pub fifocontrol: u16,
    pub fifodata: u16,
    pub /: *mut *mut u16 fifofree; / only valid in xmt channel, not in rcv channel,
    pub PAD: u16,
}

// a pair of pio channels(tx and rx)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pio2regp {
    pub tx: pio2regs,
    pub rx: pio2regs,
}

// 4byte-wide pio register set per channel(xmt or rcv)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pio4regs {
    pub fifocontrol: u32,
    pub fifodata: u32,
}

// a pair of pio channels(tx and rx)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pio4regp {
    pub tx: pio4regs,
    pub rx: pio4regs,
}

// read: 32-bit register that can be read as 32-bit or as 2 16-bit
// write: only low 16b-it half can be written
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union pmqreg {
    pub /: *mut *mut u32 pmqhostdata; / read only!,
    pub /: *mut *mut u16 pmqctrlstatus; / read/write,
    pub PAD: u16,
    pub w: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fifo64 {
    pub /: *mut *mut dma64regs dmaxmt; / dma tx,
    pub /: *mut *mut pio4regs piotx; / pio tx,
    pub /: *mut *mut dma64regs dmarcv; / dma rx,
    pub /: *mut *mut pio4regs piorx; / pio rx,
}

//
// Host Interface Registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d11regs {
// Device Control ("semi-standard host registers")
    pub /: *mut *mut u32 PAD[3]; / 0x0 - 0x8,
    pub /: *mut *mut u32 biststatus; / 0xC,
    pub /: *mut *mut u32 biststatus2; / 0x10,
    pub /: *mut *mut u32 PAD; / 0x14,
    pub /: *mut *mut u32 gptimer; / 0x18,
    pub /: *mut *mut *mut *mut u32 usectimer; / 0x1c // for corerev >= 26,
// Interrupt Control *//* 0x20
    pub intctrlregs: [intctrlregs; 8],
    pub /: *mut *mut u32 PAD[40]; / 0x60 - 0xFC,
    pub /: *mut *mut u32 intrcvlazy[4]; / 0x100 - 0x10C,
    pub /: *mut *mut u32 PAD[4]; / 0x110 - 0x11c,
    pub /: *mut *mut u32 maccontrol; / 0x120,
    pub /: *mut *mut u32 maccommand; / 0x124,
    pub /: *mut *mut u32 macintstatus; / 0x128,
    pub /: *mut *mut u32 macintmask; / 0x12C,
// Transmit Template Access
    pub /: *mut *mut u32 tplatewrptr; / 0x130,
    pub /: *mut *mut u32 tplatewrdata; / 0x134,
    pub /: *mut *mut u32 PAD[2]; / 0x138 - 0x13C,
// PMQ registers
    pub /: *mut *mut pmqreg pmqreg; / 0x140,
    pub /: *mut *mut u32 pmqpatl; / 0x144,
    pub /: *mut *mut u32 pmqpath; / 0x148,
    pub /: *mut *mut u32 PAD; / 0x14C,
    pub /: *mut *mut u32 chnstatus; / 0x150,
    pub /: *mut *mut u32 psmdebug; / 0x154,
    pub /: *mut *mut u32 phydebug; / 0x158,
    pub /: *mut *mut u32 machwcap; / 0x15C,
// Extended Internal Objects
    pub /: *mut *mut u32 objaddr; / 0x160,
    pub /: *mut *mut u32 objdata; / 0x164,
    pub /: *mut *mut u32 PAD[2]; / 0x168 - 0x16c,
    pub /: *mut *mut u32 frmtxstatus; / 0x170,
    pub /: *mut *mut u32 frmtxstatus2; / 0x174,
    pub /: *mut *mut u32 PAD[2]; / 0x178 - 0x17c,
// TSF host access
    pub /: *mut *mut u32 tsf_timerlow; / 0x180,
    pub /: *mut *mut u32 tsf_timerhigh; / 0x184,
    pub /: *mut *mut u32 tsf_cfprep; / 0x188,
    pub /: *mut *mut u32 tsf_cfpstart; / 0x18c,
    pub /: *mut *mut u32 tsf_cfpmaxdur32; / 0x190,
    pub /: *mut *mut u32 PAD[3]; / 0x194 - 0x19c,
    pub /: *mut *mut u32 maccontrol1; / 0x1a0,
    pub /: *mut *mut u32 machwcap1; / 0x1a4,
    pub /: *mut *mut u32 PAD[14]; / 0x1a8 - 0x1dc,
// Clock control and hardware workarounds
    pub /: *mut *mut u32 clk_ctl_st; / 0x1e0,
    pub hw_war: u32,
    pub are: *mut *mut u32 d11_phypllctl; / the phypll request/avail bits,
// moved to clk_ctl_st
//
    pub /: *mut *mut u32 PAD[5]; / 0x1ec - 0x1fc,
// 0x200-0x37F dma/pio registers
    pub fifo64regs: [fifo64; 6],
// FIFO diagnostic port access
    pub /: *mut *mut dma32diag dmafifo; / 0x380 - 0x38C,
    pub /: *mut *mut u32 aggfifocnt; / 0x390,
    pub /: *mut *mut u32 aggfifodata; / 0x394,
    pub /: *mut *mut u32 PAD[16]; / 0x398 - 0x3d4,
    pub /: *mut *mut u16 radioregaddr; / 0x3d8,
    pub /: *mut *mut u16 radioregdata; / 0x3da,
//
// time delay between the change on rf disable input and
// radio shutdown
//
    pub /: *mut *mut u32 rfdisabledly; / 0x3DC,
// PHY register access
    pub /: *mut *mut u16 phyversion; / 0x3e0 - 0x0,
    pub /: *mut *mut u16 phybbconfig; / 0x3e2 - 0x1,
    pub /: *mut *mut u16 phyadcbias; / 0x3e4 - 0x2 Bphy only,
    pub /: *mut *mut u16 phyanacore; / 0x3e6 - 0x3 pwwrdwn on aphy,
    pub /: *mut *mut u16 phyrxstatus0; / 0x3e8 - 0x4,
    pub /: *mut *mut u16 phyrxstatus1; / 0x3ea - 0x5,
    pub /: *mut *mut u16 phycrsth; / 0x3ec - 0x6,
    pub /: *mut *mut u16 phytxerror; / 0x3ee - 0x7,
    pub /: *mut *mut u16 phychannel; / 0x3f0 - 0x8,
    pub /: *mut *mut u16 PAD[1]; / 0x3f2 - 0x9,
    pub /: *mut *mut u16 phytest; / 0x3f4 - 0xa,
    pub /: *mut *mut u16 phy4waddr; / 0x3f6 - 0xb,
    pub /: *mut *mut u16 phy4wdatahi; / 0x3f8 - 0xc,
    pub /: *mut *mut u16 phy4wdatalo; / 0x3fa - 0xd,
    pub /: *mut *mut u16 phyregaddr; / 0x3fc - 0xe,
    pub /: *mut *mut u16 phyregdata; / 0x3fe - 0xf,
// IHR *//* 0x400 - 0x7FE
// RXE Block
    pub /: *mut *mut u16 PAD[3]; / 0x400 - 0x406,
    pub /: *mut *mut u16 rcv_fifo_ctl; / 0x406,
    pub /: *mut *mut u16 PAD; / 0x408 - 0x40a,
    pub /: *mut *mut u16 rcv_frm_cnt; / 0x40a,
    pub /: *mut *mut u16 PAD[4]; / 0x40a - 0x414,
    pub /: *mut *mut u16 rssi; / 0x414,
    pub /: *mut *mut u16 PAD[5]; / 0x414 - 0x420,
    pub /: *mut *mut u16 rcm_ctl; / 0x420,
    pub /: *mut *mut u16 rcm_mat_data; / 0x422,
    pub /: *mut *mut u16 rcm_mat_mask; / 0x424,
    pub /: *mut *mut u16 rcm_mat_dly; / 0x426,
    pub /: *mut *mut u16 rcm_cond_mask_l; / 0x428,
    pub /: *mut *mut u16 rcm_cond_mask_h; / 0x42A,
    pub /: *mut *mut u16 rcm_cond_dly; / 0x42C,
    pub /: *mut *mut u16 PAD[1]; / 0x42E,
    pub /: *mut *mut u16 ext_ihr_addr; / 0x430,
    pub /: *mut *mut u16 ext_ihr_data; / 0x432,
    pub /: *mut *mut u16 rxe_phyrs_2; / 0x434,
    pub /: *mut *mut u16 rxe_phyrs_3; / 0x436,
    pub /: *mut *mut u16 phy_mode; / 0x438,
    pub /: *mut *mut u16 rcmta_ctl; / 0x43a,
    pub /: *mut *mut u16 rcmta_size; / 0x43c,
    pub /: *mut *mut u16 rcmta_addr0; / 0x43e,
    pub /: *mut *mut u16 rcmta_addr1; / 0x440,
    pub /: *mut *mut u16 rcmta_addr2; / 0x442,
    pub /: *mut *mut u16 PAD[30]; / 0x444 - 0x480,
// PSM Block *//* 0x480 - 0x500
    pub /: *mut *mut u16 PAD; / 0x480,
    pub /: *mut *mut u16 psm_maccontrol_h; / 0x482,
    pub /: *mut *mut u16 psm_macintstatus_l; / 0x484,
    pub /: *mut *mut u16 psm_macintstatus_h; / 0x486,
    pub /: *mut *mut u16 psm_macintmask_l; / 0x488,
    pub /: *mut *mut u16 psm_macintmask_h; / 0x48A,
    pub /: *mut *mut u16 PAD; / 0x48C,
    pub /: *mut *mut u16 psm_maccommand; / 0x48E,
    pub /: *mut *mut u16 psm_brc; / 0x490,
    pub /: *mut *mut u16 psm_phy_hdr_param; / 0x492,
    pub /: *mut *mut u16 psm_postcard; / 0x494,
    pub /: *mut *mut u16 psm_pcard_loc_l; / 0x496,
    pub /: *mut *mut u16 psm_pcard_loc_h; / 0x498,
    pub /: *mut *mut u16 psm_gpio_in; / 0x49A,
    pub /: *mut *mut u16 psm_gpio_out; / 0x49C,
    pub /: *mut *mut u16 psm_gpio_oe; / 0x49E,
    pub /: *mut *mut u16 psm_bred_0; / 0x4A0,
    pub /: *mut *mut u16 psm_bred_1; / 0x4A2,
    pub /: *mut *mut u16 psm_bred_2; / 0x4A4,
    pub /: *mut *mut u16 psm_bred_3; / 0x4A6,
    pub /: *mut *mut u16 psm_brcl_0; / 0x4A8,
    pub /: *mut *mut u16 psm_brcl_1; / 0x4AA,
    pub /: *mut *mut u16 psm_brcl_2; / 0x4AC,
    pub /: *mut *mut u16 psm_brcl_3; / 0x4AE,
    pub /: *mut *mut u16 psm_brpo_0; / 0x4B0,
    pub /: *mut *mut u16 psm_brpo_1; / 0x4B2,
    pub /: *mut *mut u16 psm_brpo_2; / 0x4B4,
    pub /: *mut *mut u16 psm_brpo_3; / 0x4B6,
    pub /: *mut *mut u16 psm_brwk_0; / 0x4B8,
    pub /: *mut *mut u16 psm_brwk_1; / 0x4BA,
    pub /: *mut *mut u16 psm_brwk_2; / 0x4BC,
    pub /: *mut *mut u16 psm_brwk_3; / 0x4BE,
    pub /: *mut *mut u16 psm_base_0; / 0x4C0,
    pub /: *mut *mut u16 psm_base_1; / 0x4C2,
    pub /: *mut *mut u16 psm_base_2; / 0x4C4,
    pub /: *mut *mut u16 psm_base_3; / 0x4C6,
    pub /: *mut *mut u16 psm_base_4; / 0x4C8,
    pub /: *mut *mut u16 psm_base_5; / 0x4CA,
    pub /: *mut *mut u16 psm_base_6; / 0x4CC,
    pub /: *mut *mut u16 psm_pc_reg_0; / 0x4CE,
    pub /: *mut *mut u16 psm_pc_reg_1; / 0x4D0,
    pub /: *mut *mut u16 psm_pc_reg_2; / 0x4D2,
    pub /: *mut *mut u16 psm_pc_reg_3; / 0x4D4,
    pub /: *mut *mut u16 PAD[0xD]; / 0x4D6 - 0x4DE,
    pub /: *mut *mut *mut *mut u16 psm_corectlsts; / 0x4f0 // Corerev >= 13,
    pub /: *mut *mut u16 PAD[0x7]; / 0x4f2 - 0x4fE,
// TXE0 Block *//* 0x500 - 0x580
    pub /: *mut *mut u16 txe_ctl; / 0x500,
    pub /: *mut *mut u16 txe_aux; / 0x502,
    pub /: *mut *mut u16 txe_ts_loc; / 0x504,
    pub /: *mut *mut u16 txe_time_out; / 0x506,
    pub /: *mut *mut u16 txe_wm_0; / 0x508,
    pub /: *mut *mut u16 txe_wm_1; / 0x50A,
    pub /: *mut *mut u16 txe_phyctl; / 0x50C,
    pub /: *mut *mut u16 txe_status; / 0x50E,
    pub /: *mut *mut u16 txe_mmplcp0; / 0x510,
    pub /: *mut *mut u16 txe_mmplcp1; / 0x512,
    pub /: *mut *mut u16 txe_phyctl1; / 0x514,
    pub /: *mut *mut u16 PAD[0x05]; / 0x510 - 0x51E,
// Transmit control
    pub /: *mut *mut u16 xmtfifodef; / 0x520,
    pub /: *mut *mut *mut *mut u16 xmtfifo_frame_cnt; / 0x522 // Corerev >= 16,
    pub /: *mut *mut *mut *mut u16 xmtfifo_byte_cnt; / 0x524 // Corerev >= 16,
    pub /: *mut *mut *mut *mut u16 xmtfifo_head; / 0x526 // Corerev >= 16,
    pub /: *mut *mut *mut *mut u16 xmtfifo_rd_ptr; / 0x528 // Corerev >= 16,
    pub /: *mut *mut *mut *mut u16 xmtfifo_wr_ptr; / 0x52A // Corerev >= 16,
    pub /: *mut *mut *mut *mut u16 xmtfifodef1; / 0x52C // Corerev >= 16,
    pub /: *mut *mut u16 PAD[0x09]; / 0x52E - 0x53E,
    pub /: *mut *mut u16 xmtfifocmd; / 0x540,
    pub /: *mut *mut u16 xmtfifoflush; / 0x542,
    pub /: *mut *mut u16 xmtfifothresh; / 0x544,
    pub /: *mut *mut u16 xmtfifordy; / 0x546,
    pub /: *mut *mut u16 xmtfifoprirdy; / 0x548,
    pub /: *mut *mut u16 xmtfiforqpri; / 0x54A,
    pub /: *mut *mut u16 xmttplatetxptr; / 0x54C,
    pub /: *mut *mut u16 PAD; / 0x54E,
    pub /: *mut *mut u16 xmttplateptr; / 0x550,
    pub /: *mut *mut *mut *mut u16 smpl_clct_strptr; / 0x552 // Corerev >= 22,
    pub /: *mut *mut *mut *mut u16 smpl_clct_stpptr; / 0x554 // Corerev >= 22,
    pub /: *mut *mut *mut *mut u16 smpl_clct_curptr; / 0x556 // Corerev >= 22,
    pub /: *mut *mut u16 PAD[0x04]; / 0x558 - 0x55E,
    pub /: *mut *mut u16 xmttplatedatalo; / 0x560,
    pub /: *mut *mut u16 xmttplatedatahi; / 0x562,
    pub /: *mut *mut u16 PAD[2]; / 0x564 - 0x566,
    pub /: *mut *mut u16 xmtsel; / 0x568,
    pub /: *mut *mut u16 xmttxcnt; / 0x56A,
    pub /: *mut *mut u16 xmttxshmaddr; / 0x56C,
    pub /: *mut *mut u16 PAD[0x09]; / 0x56E - 0x57E,
// TXE1 Block
    pub /: *mut *mut u16 PAD[0x40]; / 0x580 - 0x5FE,
// TSF Block
    pub /: *mut *mut u16 PAD[0X02]; / 0x600 - 0x602,
    pub /: *mut *mut u16 tsf_cfpstrt_l; / 0x604,
    pub /: *mut *mut u16 tsf_cfpstrt_h; / 0x606,
    pub /: *mut *mut u16 PAD[0X05]; / 0x608 - 0x610,
    pub /: *mut *mut u16 tsf_cfppretbtt; / 0x612,
    pub /: *mut *mut u16 PAD[0XD]; / 0x614 - 0x62C,
    pub /: *mut *mut u16 tsf_clk_frac_l; / 0x62E,
    pub /: *mut *mut u16 tsf_clk_frac_h; / 0x630,
    pub /: *mut *mut u16 PAD[0X14]; / 0x632 - 0x658,
    pub /: *mut *mut u16 tsf_random; / 0x65A,
    pub /: *mut *mut u16 PAD[0x05]; / 0x65C - 0x664,
// GPTimer 2 registers
    pub /: *mut *mut u16 tsf_gpt2_stat; / 0x666,
    pub /: *mut *mut u16 tsf_gpt2_ctr_l; / 0x668,
    pub /: *mut *mut u16 tsf_gpt2_ctr_h; / 0x66A,
    pub /: *mut *mut u16 tsf_gpt2_val_l; / 0x66C,
    pub /: *mut *mut u16 tsf_gpt2_val_h; / 0x66E,
    pub /: *mut *mut u16 tsf_gptall_stat; / 0x670,
    pub /: *mut *mut u16 PAD[0x07]; / 0x672 - 0x67E,
// IFS Block
    pub /: *mut *mut u16 ifs_sifs_rx_tx_tx; / 0x680,
    pub /: *mut *mut u16 ifs_sifs_nav_tx; / 0x682,
    pub /: *mut *mut u16 ifs_slot; / 0x684,
    pub /: *mut *mut u16 PAD; / 0x686,
    pub /: *mut *mut u16 ifs_ctl; / 0x688,
    pub /: *mut *mut u16 PAD[0x3]; / 0x68a - 0x68F,
    pub /: *mut *mut u16 ifsstat; / 0x690,
    pub /: *mut *mut u16 ifsmedbusyctl; / 0x692,
    pub /: *mut *mut u16 iftxdur; / 0x694,
    pub /: *mut *mut u16 PAD[0x3]; / 0x696 - 0x69b,
// EDCF support in dot11macs
    pub /: *mut *mut u16 ifs_aifsn; / 0x69c,
    pub /: *mut *mut u16 ifs_ctl1; / 0x69e,
// slow clock registers
    pub /: *mut *mut u16 scc_ctl; / 0x6a0,
    pub /: *mut *mut u16 scc_timer_l; / 0x6a2,
    pub /: *mut *mut u16 scc_timer_h; / 0x6a4,
    pub /: *mut *mut u16 scc_frac; / 0x6a6,
    pub /: *mut *mut u16 scc_fastpwrup_dly; / 0x6a8,
    pub /: *mut *mut u16 scc_per; / 0x6aa,
    pub /: *mut *mut u16 scc_per_frac; / 0x6ac,
    pub /: *mut *mut u16 scc_cal_timer_l; / 0x6ae,
    pub /: *mut *mut u16 scc_cal_timer_h; / 0x6b0,
    pub /: *mut *mut u16 PAD; / 0x6b2,
    pub PAD: [u16; 0x26],
// NAV Block
    pub /: *mut *mut u16 nav_ctl; / 0x700,
    pub /: *mut *mut u16 navstat; / 0x702,
    pub /: *mut *mut u16 PAD[0x3e]; / 0x702 - 0x77E,
// WEP/PMQ Block *//* 0x780 - 0x7FE
    pub /: *mut *mut u16 PAD[0x20]; / 0x780 - 0x7BE,
    pub /: *mut *mut u16 wepctl; / 0x7C0,
    pub /: *mut *mut u16 wepivloc; / 0x7C2,
    pub /: *mut *mut u16 wepivkey; / 0x7C4,
    pub /: *mut *mut u16 wepwkey; / 0x7C6,
    pub /: *mut *mut u16 PAD[4]; / 0x7C8 - 0x7CE,
    pub /: *mut *mut u16 pcmctl; / 0X7D0,
    pub /: *mut *mut u16 pcmstat; / 0X7D2,
    pub /: *mut *mut u16 PAD[6]; / 0x7D4 - 0x7DE,
    pub /: *mut *mut u16 pmqctl; / 0x7E0,
    pub /: *mut *mut u16 pmqstatus; / 0x7E2,
    pub /: *mut *mut u16 pmqpat0; / 0x7E4,
    pub /: *mut *mut u16 pmqpat1; / 0x7E6,
    pub /: *mut *mut u16 pmqpat2; / 0x7E8,
    pub /: *mut *mut u16 pmqdat; / 0x7EA,
    pub /: *mut *mut u16 pmqdator; / 0x7EC,
    pub /: *mut *mut u16 pmqhst; / 0x7EE,
    pub /: *mut *mut u16 pmqpath0; / 0x7F0,
    pub /: *mut *mut u16 pmqpath1; / 0x7F2,
    pub /: *mut *mut u16 pmqpath2; / 0x7F4,
    pub /: *mut *mut u16 pmqdath; / 0x7F6,
    pub /: *mut *mut u16 PAD[0x04]; / 0x7F8 - 0x7FE,
// SHM *//* 0x800 - 0xEFE
    pub /: *mut *mut u16 PAD[0x380]; / 0x800 - 0xEFE,
}

// d11 register field offset

pub const PIHR_BASE: c_uint = 0x0400	/* byte address of packed IHR region */;
// biststatus

// intstatus and intmask

// interrupt receive lazy
pub const IRL_TO_MASK: c_uint = 0x00ffffff	/* timeout */;
pub const IRL_FC_MASK: c_uint = 0xff000000	/* frame count */;

// == maccontrol register ==

pub const MCTL_GPOUT_SEL_SHIFT: c_int = 14;

// == maccommand register ==

// == macintstatus/macintmask ==
// gracefully suspended

// beacon template available

// TBTT indication

// beacon successfully tx'd

// beacon canceled (IBSS)

// end of ATIM-window (IBSS)

// PMQ entries available

// non-specific gen-stat bits that are set by PSM

// non-specific gen-stat bits that are set by PSM

// MAC level Tx error

// non-specific gen-stat bits that are set by PSM

// PHY Tx error

// Power Management Event

// General-purpose timer0

// General-purpose timer1

// (ORed) DMA-interrupts

// MAC has completed a TX FIFO Suspend/Flush

// MAC has completed a CCA measurement

// MAC has collected background noise samples

// MBSS DTIM TBTT indication

// Probe response queue needs attention

// Radio/PHY has been powered back up.

// MAC detected change on RF Disable input

// MAC has completed a TX

// A phy status change wrt G mode

// general purpose timeout

// Mac capabilities registers
// == machwcap ==
pub const MCAP_TKIPMIC: c_uint = 0x80000000	/* TKIP MIC hardware present */;
// == pmqhost data ==
// data entry of head pmq entry
pub const PMQH_DATA_MASK: c_uint = 0xffff0000;
// PM entry for BSS config
pub const PMQH_BSSCFG: c_uint = 0x00100000;
// PM Mode OFF: power save off
pub const PMQH_PMOFF: c_uint = 0x00010000;
// PM Mode ON: power save on
pub const PMQH_PMON: c_uint = 0x00020000;
// Dis-associated or De-authenticated
pub const PMQH_DASAT: c_uint = 0x00040000;
// ATIM not acknowledged
pub const PMQH_ATIMFAIL: c_uint = 0x00080000;
// delete head entry
pub const PMQH_DEL_ENTRY: c_uint = 0x00000001;
// delete head entry to cur read pointer -1
pub const PMQH_DEL_MULT: c_uint = 0x00000002;
// pmq overflow indication
pub const PMQH_OFLO: c_uint = 0x00000004;
// entries are present in pmq
pub const PMQH_NOT_EMPTY: c_uint = 0x00000008;
// == phydebug ==
// phy is asserting carrier sense

// phy is taking xmit byte from mac this cycle

// mac is instructing the phy to transmit a frame

// phy is signalling a transmit Error to the mac

// phy detected the end of a valid frame preamble

// phy detected the end of a valid PLCP header

// rx start not asserted

// mac is taking receive byte from phy this cycle

// RF portion of the radio is disabled

// == objaddr register ==
pub const OBJADDR_SEL_MASK: c_uint = 0x000F0000;
pub const OBJADDR_UCM_SEL: c_uint = 0x00000000;
pub const OBJADDR_SHM_SEL: c_uint = 0x00010000;
pub const OBJADDR_SCR_SEL: c_uint = 0x00020000;
pub const OBJADDR_IHR_SEL: c_uint = 0x00030000;
pub const OBJADDR_RCMTA_SEL: c_uint = 0x00040000;
pub const OBJADDR_SRCHM_SEL: c_uint = 0x00060000;
pub const OBJADDR_WINC: c_uint = 0x01000000;
pub const OBJADDR_RINC: c_uint = 0x02000000;
pub const OBJADDR_AUTO_INC: c_uint = 0x03000000;
pub const WEP_PCMADDR: c_uint = 0x07d4;
pub const WEP_PCMDATA: c_uint = 0x07d6;
// == frmtxstatus ==

pub const TXS_STATUS_MASK: c_uint = 0xffff;
pub const TXS_FID_MASK: c_uint = 0xffff0000;
pub const TXS_FID_SHIFT: c_int = 16;
// == frmtxstatus2 ==
pub const TXS_SEQ_MASK: c_uint = 0xffff;
pub const TXS_PTX_MASK: c_uint = 0xff0000;
pub const TXS_PTX_SHIFT: c_int = 16;
pub const TXS_MU_MASK: c_uint = 0x01000000;
pub const TXS_MU_SHIFT: c_int = 24;
// == clk_ctl_st ==
pub const CCS_ERSRC_REQ_D11PLL: c_uint = 0x00000100	/* d11 core pll request */;
pub const CCS_ERSRC_REQ_PHYPLL: c_uint = 0x00000200	/* PHY pll request */;
pub const CCS_ERSRC_AVAIL_D11PLL: c_uint = 0x01000000	/* d11 core pll available */;
pub const CCS_ERSRC_AVAIL_PHYPLL: c_uint = 0x02000000	/* PHY pll available */;
// HT Cloclk Ctrl and Clock Avail for 4313
pub const CCS_ERSRC_REQ_HT: c_uint = 0x00000010	/* HT avail request */;
pub const CCS_ERSRC_AVAIL_HT: c_uint = 0x00020000	/* HT clock available */;
// tsf_cfprep register
pub const CFPREP_CBI_MASK: c_uint = 0xffffffc0;
pub const CFPREP_CBI_SHIFT: c_int = 6;
pub const CFPREP_CFPP: c_uint = 0x00000001;
// tx fifo sizes values are in terms of 256 byte blocks

// == phy versions (PhyVersion:Revision field) ==
// analog block version
pub const PV_AV_MASK: c_uint = 0xf000;
// analog block version bitfield offset
pub const PV_AV_SHIFT: c_int = 12;
// phy type
pub const PV_PT_MASK: c_uint = 0x0f00;
// phy type bitfield offset
pub const PV_PT_SHIFT: c_int = 8;
// phy version
pub const PV_PV_MASK: c_uint = 0x000f;

// == phy types (PhyVersion:PhyType field) ==

pub const PHY_TYPE_NULL: c_uint = 0xf	/* Invalid Phy value */;
// == analog types (PhyVersion:AnalogType field) ==
pub const ANA_11N_013: c_int = 5;
// 802.11a PLCP header def
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ofdm_phy_hdr {
    pub /: *mut *mut u8 rlpt[3]; / rate, length, parity, tail,
    pub service: u16,
    pub pad: u8,
    pub __packed: },

// rate encoded per 802.11a-1999 sec 17.3.4.1

// set reserved field to zero

// length is number of octets in PSDU

// set the tail to all zeros

// 802.11b PLCP header def
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cck_phy_hdr {
    pub signal: u8,
    pub service: u8,
    pub length: u16,
    pub crc: u16,
    pub __packed: },
pub const D11B_PHY_HDR_LEN: c_int = 6;

pub const MIMO_PLCP_MCS_MASK: c_uint = 0x7f	/* mcs index */;
pub const MIMO_PLCP_40MHZ: c_uint = 0x80	/* 40 Hz frame */;
pub const MIMO_PLCP_AMPDU: c_uint = 0x08	/* ampdu */;

    pub \: plcp[1] = len & 0xff;,
    pub \: plcp[2] = ((len >> 8) & 0xff);,

//
// The dot11a PLCP header is 5 bytes.  To simplify the software (so that we
// don't need e.g. different tx DMA headers for 11a and 11b), the PLCP header
// has padding added in the ucode.
//
pub const D11_PHY_HDR_LEN: c_int = 6;
// TX DMA buffer header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d11txh {
    pub /: *mut *mut __le16 MacTxControlLow; / 0x0,
    pub /: *mut *mut __le16 MacTxControlHigh; / 0x1,
    pub /: *mut *mut __le16 MacFrameControl; / 0x2,
    pub /: *mut *mut __le16 TxFesTimeNormal; / 0x3,
    pub /: *mut *mut __le16 PhyTxControlWord; / 0x4,
    pub /: *mut *mut __le16 PhyTxControlWord_1; / 0x5,
    pub /: *mut *mut __le16 PhyTxControlWord_1_Fbr; / 0x6,
    pub /: *mut *mut __le16 PhyTxControlWord_1_Rts; / 0x7,
    pub /: *mut *mut __le16 PhyTxControlWord_1_FbrRts; / 0x8,
    pub /: *mut *mut __le16 MainRates; / 0x9,
    pub /: *mut *mut __le16 XtraFrameTypes; / 0xa,
    pub /: *mut *mut u8 IV[16]; / 0x0b - 0x12,
    pub /: *mut *mut u8 TxFrameRA[6]; / 0x13 - 0x15,
    pub /: *mut *mut __le16 TxFesTimeFallback; / 0x16,
    pub /: *mut *mut u8 RTSPLCPFallback[6]; / 0x17 - 0x19,
    pub /: *mut *mut __le16 RTSDurFallback; / 0x1a,
    pub /: *mut *mut u8 FragPLCPFallback[6]; / 0x1b - 1d,
    pub /: *mut *mut __le16 FragDurFallback; / 0x1e,
    pub /: *mut *mut __le16 MModeLen; / 0x1f,
    pub /: *mut *mut __le16 MModeFbrLen; / 0x20,
    pub /: *mut *mut __le16 TstampLow; / 0x21,
    pub /: *mut *mut __le16 TstampHigh; / 0x22,
    pub /: *mut *mut __le16 ABI_MimoAntSel; / 0x23,
    pub /: *mut *mut __le16 PreloadSize; / 0x24,
    pub /: *mut *mut __le16 AmpduSeqCtl; / 0x25,
    pub /: *mut *mut __le16 TxFrameID; / 0x26,
    pub /: *mut *mut __le16 TxStatus; / 0x27,
    pub /: *mut *mut __le16 MaxNMpdus; / 0x28,
    pub /: *mut *mut __le16 MaxABytes_MRT; / 0x29,
    pub /: *mut *mut __le16 MaxABytes_FBR; / 0x2a,
    pub /: *mut *mut __le16 MinMBytes; / 0x2b,
    pub /: *mut *mut u8 RTSPhyHeader[D11_PHY_HDR_LEN]; / 0x2c - 0x2e,
    pub /: *mut *mut ieee80211_rts rts_frame; / 0x2f - 0x36,
    pub /: *mut *mut u16 PAD; / 0x37,
    pub __aligned(2): } __packed,

// Frame Types
pub const FT_CCK: c_int = 0;
pub const FT_OFDM: c_int = 1;
pub const FT_HT: c_int = 2;
pub const FT_N: c_int = 3;
//
// Position of MPDU inside A-MPDU; indicated with bits 10:9
// of MacTxControlLow
//

// == MacTxControlLow ==
pub const TXC_AMIC: c_uint = 0x8000;
pub const TXC_SENDCTS: c_uint = 0x0800;
pub const TXC_AMPDU_MASK: c_uint = 0x0600;
pub const TXC_BW_40: c_uint = 0x0100;
pub const TXC_FREQBAND_5G: c_uint = 0x0080;
pub const TXC_DFCS: c_uint = 0x0040;
pub const TXC_IGNOREPMQ: c_uint = 0x0020;
pub const TXC_HWSEQ: c_uint = 0x0010;
pub const TXC_STARTMSDU: c_uint = 0x0008;
pub const TXC_SENDRTS: c_uint = 0x0004;
pub const TXC_LONGFRAME: c_uint = 0x0002;
pub const TXC_IMMEDACK: c_uint = 0x0001;
// == MacTxControlHigh ==
// RTS fallback preamble type 1 = SHORT 0 = LONG
pub const TXC_PREAMBLE_RTS_FB_SHORT: c_uint = 0x8000;
// RTS main rate preamble type 1 = SHORT 0 = LONG
pub const TXC_PREAMBLE_RTS_MAIN_SHORT: c_uint = 0x4000;
//
// Main fallback rate preamble type
// 1 = SHORT for OFDM/GF for MIMO
// 0 = LONG for CCK/MM for MIMO
//
pub const TXC_PREAMBLE_DATA_FB_SHORT: c_uint = 0x2000;
// TXC_PREAMBLE_DATA_MAIN is in PhyTxControl bit 5
// use fallback rate for this AMPDU
pub const TXC_AMPDU_FBR: c_uint = 0x1000;
pub const TXC_SECKEY_MASK: c_uint = 0x0FF0;
pub const TXC_SECKEY_SHIFT: c_int = 4;
// Use alternate txpwr defined at loc. M_ALT_TXPWR_IDX
pub const TXC_ALT_TXPWR: c_uint = 0x0008;
pub const TXC_SECTYPE_MASK: c_uint = 0x0007;
pub const TXC_SECTYPE_SHIFT: c_int = 0;
// Null delimiter for Fallback rate

// PhyTxControl for Mimophy
pub const PHY_TXC_PWR_MASK: c_uint = 0xFC00;
pub const PHY_TXC_PWR_SHIFT: c_int = 10;
pub const PHY_TXC_ANT_MASK: c_uint = 0x03C0	/* bit 6, 7, 8, 9 */;
pub const PHY_TXC_ANT_SHIFT: c_int = 6;
pub const PHY_TXC_ANT_0_1: c_uint = 0x00C0	/* auto, last rx */;
pub const PHY_TXC_LCNPHY_ANT_LAST: c_uint = 0x0000;
pub const PHY_TXC_ANT_3: c_uint = 0x0200	/* virtual antenna 3 */;
pub const PHY_TXC_ANT_2: c_uint = 0x0100	/* virtual antenna 2 */;
pub const PHY_TXC_ANT_1: c_uint = 0x0080	/* virtual antenna 1 */;
pub const PHY_TXC_ANT_0: c_uint = 0x0040	/* virtual antenna 0 */;
pub const PHY_TXC_SHORT_HDR: c_uint = 0x0010;
pub const PHY_TXC_OLD_ANT_0: c_uint = 0x0000;
pub const PHY_TXC_OLD_ANT_1: c_uint = 0x0100;
pub const PHY_TXC_OLD_ANT_LAST: c_uint = 0x0300;
// PhyTxControl_1 for Mimophy
pub const PHY_TXC1_BW_MASK: c_uint = 0x0007;
pub const PHY_TXC1_BW_10MHZ: c_int = 0;
pub const PHY_TXC1_BW_10MHZ_UP: c_int = 1;
pub const PHY_TXC1_BW_20MHZ: c_int = 2;
pub const PHY_TXC1_BW_20MHZ_UP: c_int = 3;
pub const PHY_TXC1_BW_40MHZ: c_int = 4;
pub const PHY_TXC1_BW_40MHZ_DUP: c_int = 5;
pub const PHY_TXC1_MODE_SHIFT: c_int = 3;
pub const PHY_TXC1_MODE_MASK: c_uint = 0x0038;
pub const PHY_TXC1_MODE_SISO: c_int = 0;
pub const PHY_TXC1_MODE_CDD: c_int = 1;
pub const PHY_TXC1_MODE_STBC: c_int = 2;
pub const PHY_TXC1_MODE_SDM: c_int = 3;
// PhyTxControl for HTphy that are different from Mimophy
pub const PHY_TXC_HTANT_MASK: c_uint = 0x3fC0	/* bits 6-13 */;
// XtraFrameTypes
pub const XFTS_RTS_FT_SHIFT: c_int = 2;
pub const XFTS_FBRRTS_FT_SHIFT: c_int = 4;
pub const XFTS_CHANNEL_SHIFT: c_int = 8;
// Antenna diversity bit in ant_wr_settle
pub const PHY_AWS_ANTDIV: c_uint = 0x2000;
// IFS ctl

// IFS ctl1

// ABI_MimoAntSel
pub const ABI_MAS_ADDR_BMP_IDX_MASK: c_uint = 0x0f00;
pub const ABI_MAS_ADDR_BMP_IDX_SHIFT: c_int = 8;
pub const ABI_MAS_FBR_ANT_PTN_MASK: c_uint = 0x00f0;
pub const ABI_MAS_FBR_ANT_PTN_SHIFT: c_int = 4;
pub const ABI_MAS_MRT_ANT_PTN_MASK: c_uint = 0x000f;
// tx status packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_status {
    pub framelen: u16,
    pub PAD: u16,
    pub frameid: u16,
    pub status: u16,
    pub lasttxtime: u16,
    pub sequence: u16,
    pub phyerr: u16,
    pub ackphyrxsh: u16,
    pub __packed: },
pub const TXSTATUS_LEN: c_int = 16;
// status field bit definitions
pub const TX_STATUS_FRM_RTX_MASK: c_uint = 0xF000;
pub const TX_STATUS_FRM_RTX_SHIFT: c_int = 12;
pub const TX_STATUS_RTS_RTX_MASK: c_uint = 0x0F00;
pub const TX_STATUS_RTS_RTX_SHIFT: c_int = 8;
pub const TX_STATUS_MASK: c_uint = 0x00FE;

pub const TX_STATUS_SUPR_MASK: c_uint = 0x1C	 /* suppress status bits (4:2) */;
pub const TX_STATUS_SUPR_SHIFT: c_int = 2;

pub const TX_STATUS_NO_ACK: c_int = 0;
// suppress status reason codes

// Unexpected tx status for rate update

// Unexpected tx status for A-MPDU rate update

pub const TX_STATUS_BA_BMAP03_MASK: c_uint = 0xF000	/* ba bitmap 0:3 in 1st pkg */;

pub const TX_STATUS_BA_BMAP47_MASK: c_uint = 0x001E	/* ba bitmap 4:7 in 2nd pkg */;

// RXE (Receive Engine)
// RCM_CTL
pub const RCM_INC_MASK_H: c_uint = 0x0080;
pub const RCM_INC_MASK_L: c_uint = 0x0040;
pub const RCM_INC_DATA: c_uint = 0x0020;
pub const RCM_INDEX_MASK: c_uint = 0x001F;
pub const RCM_SIZE: c_int = 15;

pub const RCM_WEP_TA0_OFFSET: c_int = 16;
pub const RCM_WEP_TA1_OFFSET: c_int = 19;
pub const RCM_WEP_TA2_OFFSET: c_int = 22;
pub const RCM_WEP_TA3_OFFSET: c_int = 25;
// PSM Block
// psm_phy_hdr_param bits
pub const MAC_PHY_RESET: c_int = 1;
pub const MAC_PHY_CLOCK_EN: c_int = 2;
pub const MAC_PHY_FORCE_CLK: c_int = 4;
// WEP Block
// WEP_WKEY

pub const WKEY_SEL_MASK: c_uint = 0x1F;
// WEP data formats
// the number of RCMTA entries
pub const RCMTA_SIZE: c_int = 50;

pub const M_ADDR_BMP_BLK_SZ: c_int = 12;

pub const ADDR_BMP_BSS_IDX_SHIFT: c_int = 8;
pub const WSEC_MAX_RCMTA_KEYS: c_int = 54;
// max keys in M_TKMICKEYS_BLK

// max RXE match registers
pub const WSEC_MAX_RXE_KEYS: c_int = 4;
// SECKINDXALGO (Security Key Index & Algorithm Block) word format
// SKL (Security Key Lookup)
pub const SKL_ALGO_MASK: c_uint = 0x0007;
pub const SKL_ALGO_SHIFT: c_int = 0;
pub const SKL_KEYID_MASK: c_uint = 0x0008;
pub const SKL_KEYID_SHIFT: c_int = 3;
pub const SKL_INDEX_MASK: c_uint = 0x03F0;
pub const SKL_INDEX_SHIFT: c_int = 4;
pub const SKL_GRP_ALGO_MASK: c_uint = 0x1c00;
pub const SKL_GRP_ALGO_SHIFT: c_int = 10;
// additional bits defined for IBSS group key support
pub const SKL_IBSS_INDEX_MASK: c_uint = 0x01F0;
pub const SKL_IBSS_INDEX_SHIFT: c_int = 4;
pub const SKL_IBSS_KEYID1_MASK: c_uint = 0x0600;
pub const SKL_IBSS_KEYID1_SHIFT: c_int = 9;
pub const SKL_IBSS_KEYID2_MASK: c_uint = 0x1800;
pub const SKL_IBSS_KEYID2_SHIFT: c_int = 11;
pub const SKL_IBSS_KEYALGO_MASK: c_uint = 0xE000;
pub const SKL_IBSS_KEYALGO_SHIFT: c_int = 13;
pub const WSEC_MODE_OFF: c_int = 0;
pub const WSEC_MODE_HW: c_int = 1;
pub const WSEC_MODE_SW: c_int = 2;
pub const WSEC_ALGO_OFF: c_int = 0;
pub const WSEC_ALGO_WEP1: c_int = 1;
pub const WSEC_ALGO_TKIP: c_int = 2;
pub const WSEC_ALGO_AES: c_int = 3;
pub const WSEC_ALGO_WEP128: c_int = 4;
pub const WSEC_ALGO_AES_LEGACY: c_int = 5;
pub const WSEC_ALGO_NALG: c_int = 6;
pub const AES_MODE_NONE: c_int = 0;
pub const AES_MODE_CCM: c_int = 1;
// WEP_CTL (Rev 0)
pub const WECR0_KEYREG_SHIFT: c_int = 0;
pub const WECR0_KEYREG_MASK: c_uint = 0x7;

pub const WECR0_WEPALG_SHIFT: c_int = 5;

pub const WECR0_WKEYSEL_SHIFT: c_int = 8;

// Frame template map byte offsets

// Shared Mem byte offsets
// Location where the ucode expects the corerev

// Location where the ucode expects the MAC capabilities

// WME shared memory

// PS-mode related parameters

// Beacon-related parameters

// MAX Rx Frame len

// ACK/CTS related params

// Hardware Power Control

// Rx-related parameters

// WEP Shared mem data

pub const M_SECKINDXALGO_BLK_SZ: c_int = 54;

pub const D11_MAX_KEY_SIZE: c_int = 16;

// Probe response related parameters

// Delta between OFDM and CCK power in CCK power boost mode

// TSSI for last 4 11b/g CCK packets transmitted

// Host flags to turn on ucode options

pub const M_HOST_FLAGS_SZ: c_int = 16;

// TSSI for last 4 11a OFDM packets transmitted

// noise interference measurement

// TSSI for last 4 11g OFDM packets transmitted

// Background noise measure

// TX fifo sizes

// Current channel number plus upper bits

pub const D11_CURCHANNEL_5G: c_uint = 0x0100;;
pub const D11_CURCHANNEL_40: c_uint = 0x0200;;
pub const D11_CURCHANNEL_MAX: c_uint = 0x00FF;;
// last posted frameid on the bcmc fifo

pub const INVALIDFID: c_uint = 0xffff;
// extended beacon phyctl bytes for 11N

// idle busy ratio to duty_cycle requirement

// CW RSSI for LCNPHY
pub const M_LCN_RSSI_0: c_uint = 0x1332;
pub const M_LCN_RSSI_1: c_uint = 0x1338;
pub const M_LCN_RSSI_2: c_uint = 0x133e;
pub const M_LCN_RSSI_3: c_uint = 0x1344;
// SNR for LCNPHY
pub const M_LCN_SNR_A_0: c_uint = 0x1334;
pub const M_LCN_SNR_B_0: c_uint = 0x1336;
pub const M_LCN_SNR_A_1: c_uint = 0x133a;
pub const M_LCN_SNR_B_1: c_uint = 0x133c;
pub const M_LCN_SNR_A_2: c_uint = 0x1340;
pub const M_LCN_SNR_B_2: c_uint = 0x1342;
pub const M_LCN_SNR_A_3: c_uint = 0x1346;
pub const M_LCN_SNR_B_3: c_uint = 0x1348;

// Rate table offsets

// Rate table entry offsets
pub const M_RT_PRS_PLCP_POS: c_int = 10;
pub const M_RT_PRS_DUR_POS: c_int = 16;
pub const M_RT_OFDM_PCTL1_POS: c_int = 18;

// SHM locations where ucode stores the current power index

// Antenna Diversity Testing

pub const MIMO_MAXSYM_DEF: c_uint = 0x8000	/* 32k */;
pub const MIMO_MAXSYM_MAX: c_uint = 0xffff	/* 64k */;

pub const WATCHDOG_8TU_DEF: c_int = 5;
pub const WATCHDOG_8TU_MAX: c_int = 10;
// Manufacturing Test Variables
// PER test mode

// IFS for TX mode

// Lower word of tx frmcnt/rx lostcnt

// Upper word of tx frmcnt/rx lostcnt

// Index variation in vbat ripple

// M_PKTENG_CTRL bit definitions
pub const M_PKTENG_MODE_TX: c_uint = 0x0001;
pub const M_PKTENG_MODE_TX_RIFS: c_uint = 0x0004;
pub const M_PKTENG_MODE_TX_CTS: c_uint = 0x0008;
pub const M_PKTENG_MODE_RX: c_uint = 0x0002;
pub const M_PKTENG_MODE_RX_WITH_ACK: c_uint = 0x0402;
pub const M_PKTENG_MODE_MASK: c_uint = 0x0003;
// TX frames indicated in the frmcnt reg
pub const M_PKTENG_FRMCNT_VLD: c_uint = 0x0100;
// Sample Collect parameters (bitmap and type)
// Trigger bitmap for sample collect

// Sample collect type

pub const ANTSEL_CLKDIV_4MHZ: c_int = 6;
pub const MIMO_ANTSEL_BUSY: c_uint = 0x4000	/* bit 14 (busy) */;
pub const MIMO_ANTSEL_SEL: c_uint = 0x8000	/* bit 15 write the value */;

pub const MIMO_ANTSEL_OVERRIDE: c_uint = 0x8000	/* flag */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_acparams {
    pub txop: u16,
    pub cwmin: u16,
    pub cwmax: u16,
    pub cwcur: u16,
    pub aifs: u16,
    pub bslots: u16,
    pub reggap: u16,
    pub status: u16,
    pub rsvd: [u16; 8],
    pub __packed: },

// M_HOST_FLAGS

// Flags in M_HOST_FLAGS
// Enable ucode antenna diversity help
pub const MHF1_ANTDIV: c_uint = 0x0001;
// Enable EDCF access control
pub const MHF1_EDCF: c_uint = 0x0100;
pub const MHF1_IQSWAP_WAR: c_uint = 0x0200;
// Disable Slow clock request, for corerev < 11
pub const MHF1_FORCEFASTCLK: c_uint = 0x0400;
// Flags in M_HOST_FLAGS2
// Flush BCMC FIFO immediately
pub const MHF2_TXBCMC_NOW: c_uint = 0x0040;
// Enable ucode/hw power control
pub const MHF2_HWPWRCTL: c_uint = 0x0080;
pub const MHF2_NPHY40MHZ_WAR: c_uint = 0x0800;
// Flags in M_HOST_FLAGS3
// enabled mimo antenna selection
pub const MHF3_ANTSEL_EN: c_uint = 0x0001;
// antenna selection mode: 0: 2x3, 1: 2x4
pub const MHF3_ANTSEL_MODE: c_uint = 0x0002;
pub const MHF3_RESERVED1: c_uint = 0x0004;
pub const MHF3_RESERVED2: c_uint = 0x0008;
pub const MHF3_NPHY_MLADV_WAR: c_uint = 0x0010;
// Flags in M_HOST_FLAGS4
// force bphy Tx on core 0 (board level WAR)
pub const MHF4_BPHY_TXCORE0: c_uint = 0x0080;
// for 4313A0 FEM boards
pub const MHF4_EXTPA_ENABLE: c_uint = 0x4000;
// Flags in M_HOST_FLAGS5
pub const MHF5_4313_GPIOCTRL: c_uint = 0x0001;
pub const MHF5_RESERVED1: c_uint = 0x0002;
pub const MHF5_RESERVED2: c_uint = 0x0004;
// Radio power setting for ucode

// phy noise recorded by ucode right after tx

pub const PHY_NOISE_MASK: c_uint = 0x00ff;
//
// Receive Frame Data Header for 802.11b DCF-only frames
//
// RxFrameSize: Actual byte length of the frame data received
// PAD: padding (not used)
// PhyRxStatus_0: PhyRxStatus 15:0
// PhyRxStatus_1: PhyRxStatus 31:16
// PhyRxStatus_2: PhyRxStatus 47:32
// PhyRxStatus_3: PhyRxStatus 63:48
// PhyRxStatus_4: PhyRxStatus 79:64
// PhyRxStatus_5: PhyRxStatus 95:80
// RxStatus1: MAC Rx Status
// RxStatus2: extended MAC Rx status
// RxTSFTime: RxTSFTime time of first MAC symbol + M_PHY_PLCPRX_DLY
// RxChan: gain code, channel radio code, and phy type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d11rxhdr_le {
    pub RxFrameSize: __le16,
    pub PAD: u16,
    pub PhyRxStatus_0: __le16,
    pub PhyRxStatus_1: __le16,
    pub PhyRxStatus_2: __le16,
    pub PhyRxStatus_3: __le16,
    pub PhyRxStatus_4: __le16,
    pub PhyRxStatus_5: __le16,
    pub RxStatus1: __le16,
    pub RxStatus2: __le16,
    pub RxTSFTime: __le16,
    pub RxChan: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d11rxhdr {
    pub RxFrameSize: u16,
    pub PAD: u16,
    pub PhyRxStatus_0: u16,
    pub PhyRxStatus_1: u16,
    pub PhyRxStatus_2: u16,
    pub PhyRxStatus_3: u16,
    pub PhyRxStatus_4: u16,
    pub PhyRxStatus_5: u16,
    pub RxStatus1: u16,
    pub RxStatus2: u16,
    pub RxTSFTime: u16,
    pub RxChan: u16,
    pub __packed: },
// PhyRxStatus_0:
// NPHY only: CCK, OFDM, preN, N
pub const PRXS0_FT_MASK: c_uint = 0x0003;
// NPHY only: clip count adjustment steps by AGC
pub const PRXS0_CLIP_MASK: c_uint = 0x000C;
pub const PRXS0_CLIP_SHIFT: c_int = 2;
// PHY received a frame with unsupported rate
pub const PRXS0_UNSRATE: c_uint = 0x0010;
// GPHY: rx ant, NPHY: upper sideband
pub const PRXS0_RXANT_UPSUBBAND: c_uint = 0x0020;
// CCK frame only: lost crs during cck frame reception
pub const PRXS0_LCRS: c_uint = 0x0040;
// Short Preamble
pub const PRXS0_SHORTH: c_uint = 0x0080;
// PLCP violation
pub const PRXS0_PLCPFV: c_uint = 0x0100;
// PLCP header integrity check failed
pub const PRXS0_PLCPHCF: c_uint = 0x0200;
// legacy PHY gain control
pub const PRXS0_GAIN_CTL: c_uint = 0x4000;
// NPHY: Antennas used for received frame, bitmask
pub const PRXS0_ANTSEL_MASK: c_uint = 0xF000;
pub const PRXS0_ANTSEL_SHIFT: c_uint = 0x12;
// subfield PRXS0_FT_MASK
pub const PRXS0_CCK: c_uint = 0x0000;
// valid only for G phy, use rxh->RxChan for A phy
pub const PRXS0_OFDM: c_uint = 0x0001;
pub const PRXS0_PREN: c_uint = 0x0002;
pub const PRXS0_STDN: c_uint = 0x0003;
// subfield PRXS0_ANTSEL_MASK
pub const PRXS0_ANTSEL_0: c_uint = 0x0	/* antenna 0 is used */;
pub const PRXS0_ANTSEL_1: c_uint = 0x2	/* antenna 1 is used */;
pub const PRXS0_ANTSEL_2: c_uint = 0x4	/* antenna 2 is used */;
pub const PRXS0_ANTSEL_3: c_uint = 0x8	/* antenna 3 is used */;
// PhyRxStatus_1:
pub const PRXS1_JSSI_MASK: c_uint = 0x00FF;
pub const PRXS1_JSSI_SHIFT: c_int = 0;
pub const PRXS1_SQ_MASK: c_uint = 0xFF00;
pub const PRXS1_SQ_SHIFT: c_int = 8;
// nphy PhyRxStatus_1:
pub const PRXS1_nphy_PWR0_MASK: c_uint = 0x00FF;
pub const PRXS1_nphy_PWR1_MASK: c_uint = 0xFF00;
// HTPHY Rx Status defines
// htphy PhyRxStatus_0: those bit are overlapped with PhyRxStatus_0
pub const PRXS0_BAND: c_uint = 0x0400	/* 0 = 2.4G, 1 = 5G */;
pub const PRXS0_RSVD: c_uint = 0x0800	/* reserved; set to 0 */;
pub const PRXS0_UNUSED: c_uint = 0xF000	/* unused and not defined; set to 0 */;
// htphy PhyRxStatus_1:
// core enables for {3..0}, 0=disabled, 1=enabled
pub const PRXS1_HTPHY_CORE_MASK: c_uint = 0x000F;
// antenna configuration
pub const PRXS1_HTPHY_ANTCFG_MASK: c_uint = 0x00F0;
// Mixmode PLCP Length low byte mask
pub const PRXS1_HTPHY_MMPLCPLenL_MASK: c_uint = 0xFF00;
// htphy PhyRxStatus_2:
// Mixmode PLCP Length high byte maskw
pub const PRXS2_HTPHY_MMPLCPLenH_MASK: c_uint = 0x000F;
// Mixmode PLCP rate mask
pub const PRXS2_HTPHY_MMPLCH_RATE_MASK: c_uint = 0x00F0;
// Rx power on core 0
pub const PRXS2_HTPHY_RXPWR_ANT0: c_uint = 0xFF00;
// htphy PhyRxStatus_3:
// Rx power on core 1
pub const PRXS3_HTPHY_RXPWR_ANT1: c_uint = 0x00FF;
// Rx power on core 2
pub const PRXS3_HTPHY_RXPWR_ANT2: c_uint = 0xFF00;
// htphy PhyRxStatus_4:
// Rx power on core 3
pub const PRXS4_HTPHY_RXPWR_ANT3: c_uint = 0x00FF;
// Coarse frequency offset
pub const PRXS4_HTPHY_CFO: c_uint = 0xFF00;
// htphy PhyRxStatus_5:
// Fine frequency offset
pub const PRXS5_HTPHY_FFO: c_uint = 0x00FF;
// Advance Retard
pub const PRXS5_HTPHY_AR: c_uint = 0xFF00;

// Get Rx power on core 0

// Get Rx power on core 1

// Get Rx power on core 2

// ucode RxStatus1:
pub const RXS_BCNSENT: c_uint = 0x8000;
pub const RXS_SECKINDX_MASK: c_uint = 0x07e0;
pub const RXS_SECKINDX_SHIFT: c_int = 5;

// PAD bytes to make IP data 4 bytes aligned

// ucode RxStatus2:
pub const RXS_AMSDU_MASK: c_int = 1;
pub const RXS_AGGTYPE_MASK: c_uint = 0x6;
pub const RXS_AGGTYPE_SHIFT: c_int = 1;

pub const RXS_RXANT_MASK: c_uint = 0x3;
pub const RXS_RXANT_SHIFT: c_int = 12;
// RxChan
pub const RXS_CHAN_40: c_uint = 0x1000;
pub const RXS_CHAN_5G: c_uint = 0x0800;
pub const RXS_CHAN_ID_MASK: c_uint = 0x07f8;
pub const RXS_CHAN_ID_SHIFT: c_int = 3;
pub const RXS_CHAN_PHYTYPE_MASK: c_uint = 0x0007;
pub const RXS_CHAN_PHYTYPE_SHIFT: c_int = 0;
// Index of attenuations used during ucode power control.

// M_PWRIND_MAP(core) macro

// PSM SHM variable offsets
pub const M_PSM_SOFT_REGS: c_uint = 0x0;

// offset to the target txpwr

// PKTENG Rx Stats Block

// ucode debug status codes
// not valid really
pub const DBGST_INACTIVE: c_int = 0;
// after zeroing SHM, before suspending at init
pub const DBGST_INIT: c_int = 1;
// "normal" state
pub const DBGST_ACTIVE: c_int = 2;
// suspended
pub const DBGST_SUSPENDED: c_int = 3;
// asleep (PS mode)
pub const DBGST_ASLEEP: c_int = 4;
// Scratch Reg defs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _ePsmScratchPadRegDefinitions {
    S_RSV0 = 0,
    S_RSV1,
    S_RSV2,

// offset 0x03: scratch registers for Dot11-contants
    S_DOT11_CWMIN,		/* CW-minimum */
    S_DOT11_CWMAX,		/* CW-maximum */
    S_DOT11_CWCUR,		/* CW-current */
    S_DOT11_SRC_LMT,	/* short retry count limit */
    S_DOT11_LRC_LMT,	/* long retry count limit */
    S_DOT11_DTIMCOUNT,	/* DTIM-count */

// offset 0x09: Tx-side scratch registers
    S_SEQ_NUM,		/* hardware sequence number reg */
    S_SEQ_NUM_FRAG,		/* seq num for frags (at the start of MSDU) */
    S_FRMRETX_CNT,		/* frame retx count */
    S_SSRC,			/* Station short retry count */
    S_SLRC,			/* Station long retry count */
    S_EXP_RSP,		/* Expected response frame */
    S_OLD_BREM,		/* Remaining backoff ctr */
    S_OLD_CWWIN,		/* saved-off CW-cur */
    S_TXECTL,		/* TXE-Ctl word constructed in scr-pad */
    S_CTXTST,		/* frm type-subtype as read from Tx-descr */

// offset 0x13: Rx-side scratch registers
    S_RXTST,		/* Type and subtype in Rxframe */

// Global state register
    S_STREG,		/* state storage actual bit maps below */

    S_TXPWR_SUM,		/* Tx power control: accumulator */
    S_TXPWR_ITER,		/* Tx power control: iteration */
    S_RX_FRMTYPE,		/* Rate and PHY type for frames */
    S_THIS_AGG,		/* Size of this AGG (A-MSDU) */

    S_KEYINDX,
    S_RXFRMLEN,		/* Receive MPDU length in bytes */

// offset 0x1B: Receive TSF time stored in SCR
    S_RXTSFTMRVAL_WD3,	/* TSF value at the start of rx */
    S_RXTSFTMRVAL_WD2,	/* TSF value at the start of rx */
    S_RXTSFTMRVAL_WD1,	/* TSF value at the start of rx */
    S_RXTSFTMRVAL_WD0,	/* TSF value at the start of rx */
    S_RXSSN,		/* Received start seq number for A-MPDU BA */
    S_RXQOSFLD,		/* Rx-QoS field (if present) */

// offset 0x21: Scratch pad regs used in microcode as temp storage
    S_TMP0,			/* stmp0 */
    S_TMP1,			/* stmp1 */
    S_TMP2,			/* stmp2 */
    S_TMP3,			/* stmp3 */
    S_TMP4,			/* stmp4 */
    S_TMP5,			/* stmp5 */
    S_PRQPENALTY_CTR,	/* Probe response queue penalty counter */
    S_ANTCNT,		/* unsuccessful attempts on current ant. */
    S_SYMBOL,		/* flag for possible symbol ctl frames */
    S_RXTP,			/* rx frame type */
    S_STREG2,		/* extra state storage */
    S_STREG3,		/* even more extra state storage */
    S_STREG4,		/* ... */
    S_STREG5,		/* remember to initialize it to zero */

    S_ADJPWR_IDX,
    S_CUR_PTR,		/* Temp pointer for A-MPDU re-Tx SHM table */
    S_REVID4,		/* 0x33 */
    S_INDX,			/* 0x34 */
    S_ADDR0,		/* 0x35 */
    S_ADDR1,		/* 0x36 */
    S_ADDR2,		/* 0x37 */
    S_ADDR3,		/* 0x38 */
    S_ADDR4,		/* 0x39 */
    S_ADDR5,		/* 0x3A */
    S_TMP6,			/* 0x3B */
    S_KEYINDX_BU,		/* Backup for Key index */
    S_MFGTEST_TMP0,		/* Temp regs used for RX test calculations */
    S_RXESN,		/* Received end sequence number for A-MPDU BA */
    S_STREG6,		/* 0x3F */
}

// IHR SLOW_CTRL values

// ucode mac statistic counters in shared memory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macstat {
    pub /: *mut *mut u16 txallfrm; / 0x80,
    pub /: *mut *mut u16 txrtsfrm; / 0x82,
    pub /: *mut *mut u16 txctsfrm; / 0x84,
    pub /: *mut *mut u16 txackfrm; / 0x86,
    pub /: *mut *mut u16 txdnlfrm; / 0x88,
    pub /: *mut *mut u16 txbcnfrm; / 0x8a,
    pub /: *mut *mut u16 txfunfl[8]; / 0x8c - 0x9b,
    pub /: *mut *mut u16 txtplunfl; / 0x9c,
    pub /: *mut *mut u16 txphyerr; / 0x9e,
    pub /: *mut *mut u16 pktengrxducast; / 0xa0,
    pub /: *mut *mut u16 pktengrxdmcast; / 0xa2,
    pub /: *mut *mut u16 rxfrmtoolong; / 0xa4,
    pub /: *mut *mut u16 rxfrmtooshrt; / 0xa6,
    pub /: *mut *mut u16 rxinvmachdr; / 0xa8,
    pub /: *mut *mut u16 rxbadfcs; / 0xaa,
    pub /: *mut *mut u16 rxbadplcp; / 0xac,
    pub /: *mut *mut u16 rxcrsglitch; / 0xae,
    pub /: *mut *mut u16 rxstrt; / 0xb0,
    pub /: *mut *mut u16 rxdfrmucastmbss; / 0xb2,
    pub /: *mut *mut u16 rxmfrmucastmbss; / 0xb4,
    pub /: *mut *mut u16 rxcfrmucast; / 0xb6,
    pub /: *mut *mut u16 rxrtsucast; / 0xb8,
    pub /: *mut *mut u16 rxctsucast; / 0xba,
    pub /: *mut *mut u16 rxackucast; / 0xbc,
    pub /: *mut *mut u16 rxdfrmocast; / 0xbe,
    pub /: *mut *mut u16 rxmfrmocast; / 0xc0,
    pub /: *mut *mut u16 rxcfrmocast; / 0xc2,
    pub /: *mut *mut u16 rxrtsocast; / 0xc4,
    pub /: *mut *mut u16 rxctsocast; / 0xc6,
    pub /: *mut *mut u16 rxdfrmmcast; / 0xc8,
    pub /: *mut *mut u16 rxmfrmmcast; / 0xca,
    pub /: *mut *mut u16 rxcfrmmcast; / 0xcc,
    pub /: *mut *mut u16 rxbeaconmbss; / 0xce,
    pub /: *mut *mut u16 rxdfrmucastobss; / 0xd0,
    pub /: *mut *mut u16 rxbeaconobss; / 0xd2,
    pub /: *mut *mut u16 rxrsptmout; / 0xd4,
    pub /: *mut *mut u16 bcntxcancl; / 0xd6,
    pub PAD: u16,
    pub /: *mut *mut u16 rxf0ovfl; / 0xda,
    pub /: *mut *mut u16 rxf1ovfl; / 0xdc,
    pub /: *mut *mut u16 rxf2ovfl; / 0xde,
    pub /: *mut *mut u16 txsfovfl; / 0xe0,
    pub /: *mut *mut u16 pmqovfl; / 0xe2,
    pub /: *mut *mut u16 rxcgprqfrm; / 0xe4,
    pub /: *mut *mut u16 rxcgprsqovfl; / 0xe6,
    pub /: *mut *mut u16 txcgprsfail; / 0xe8,
    pub /: *mut *mut u16 txcgprssuc; / 0xea,
    pub /: *mut *mut u16 prs_timeout; / 0xec,
    pub rxnack: u16,
    pub frmscons: u16,
    pub txnack: u16,
    pub txglitch_nack: u16,
    pub /: *mut *mut u16 txburst; / 0xf6 # tx bursts,
    pub /: *mut *mut u16 bphy_rxcrsglitch; / bphy rx crs glitch,
    pub /: *mut *mut u16 phywatchdog; / 0xfa # of phy watchdog events,
    pub PAD: u16,
    pub /: *mut *mut u16 bphy_badplcp; / bphy bad plcp,
}

// dot11 core-specific control flags
pub const SICF_PCLKE: c_uint = 0x0004	/* PHY clock enable */;
pub const SICF_PRST: c_uint = 0x0008	/* PHY reset */;
pub const SICF_MPCLKE: c_uint = 0x0010	/* MAC PHY clockcontrol enable */;
pub const SICF_FREF: c_uint = 0x0020	/* PLL FreqRefSelect */;
// NOTE: the following bw bits only apply when the core is attached
// to a NPHY
//
pub const SICF_BWMASK: c_uint = 0x00c0	/* phy clock mask (b6 & b7) */;
pub const SICF_BW40: c_uint = 0x0080	/* 40MHz BW (160MHz phyclk) */;
pub const SICF_BW20: c_uint = 0x0040	/* 20MHz BW (80MHz phyclk) */;
pub const SICF_BW10: c_uint = 0x0000	/* 10MHz BW (40MHz phyclk) */;
pub const SICF_GMODE: c_uint = 0x2000	/* gmode enable */;
// dot11 core-specific status flags
pub const SISF_2G_PHY: c_uint = 0x0001	/* 2.4G capable phy */;
pub const SISF_5G_PHY: c_uint = 0x0002	/* 5G capable phy */;
pub const SISF_FCLKA: c_uint = 0x0004	/* FastClkAvailable */;
pub const SISF_DB_PHY: c_uint = 0x0008	/* Dualband phy */;
// === End of MAC reg, Beginning of PHY(b/a/g/n) reg ===
// radio and LPPHY regs are separated
pub const BPHY_REG_OFT_BASE: c_uint = 0x0;
// offsets for indirect access to bphy registers
pub const BPHY_BB_CONFIG: c_uint = 0x01;
pub const BPHY_ADCBIAS: c_uint = 0x02;
pub const BPHY_ANACORE: c_uint = 0x03;
pub const BPHY_PHYCRSTH: c_uint = 0x06;
pub const BPHY_TEST: c_uint = 0x0a;
pub const BPHY_PA_TX_TO: c_uint = 0x10;
pub const BPHY_SYNTH_DC_TO: c_uint = 0x11;
pub const BPHY_PA_TX_TIME_UP: c_uint = 0x12;
pub const BPHY_RX_FLTR_TIME_UP: c_uint = 0x13;
pub const BPHY_TX_POWER_OVERRIDE: c_uint = 0x14;
pub const BPHY_RF_OVERRIDE: c_uint = 0x15;
pub const BPHY_RF_TR_LOOKUP1: c_uint = 0x16;
pub const BPHY_RF_TR_LOOKUP2: c_uint = 0x17;
pub const BPHY_COEFFS: c_uint = 0x18;
pub const BPHY_PLL_OUT: c_uint = 0x19;
pub const BPHY_REFRESH_MAIN: c_uint = 0x1a;
pub const BPHY_REFRESH_TO0: c_uint = 0x1b;
pub const BPHY_REFRESH_TO1: c_uint = 0x1c;
pub const BPHY_RSSI_TRESH: c_uint = 0x20;
pub const BPHY_IQ_TRESH_HH: c_uint = 0x21;
pub const BPHY_IQ_TRESH_H: c_uint = 0x22;
pub const BPHY_IQ_TRESH_L: c_uint = 0x23;
pub const BPHY_IQ_TRESH_LL: c_uint = 0x24;
pub const BPHY_GAIN: c_uint = 0x25;
pub const BPHY_LNA_GAIN_RANGE: c_uint = 0x26;
pub const BPHY_JSSI: c_uint = 0x27;
pub const BPHY_TSSI_CTL: c_uint = 0x28;
pub const BPHY_TSSI: c_uint = 0x29;
pub const BPHY_TR_LOSS_CTL: c_uint = 0x2a;
pub const BPHY_LO_LEAKAGE: c_uint = 0x2b;
pub const BPHY_LO_RSSI_ACC: c_uint = 0x2c;
pub const BPHY_LO_IQMAG_ACC: c_uint = 0x2d;
pub const BPHY_TX_DC_OFF1: c_uint = 0x2e;
pub const BPHY_TX_DC_OFF2: c_uint = 0x2f;
pub const BPHY_PEAK_CNT_THRESH: c_uint = 0x30;
pub const BPHY_FREQ_OFFSET: c_uint = 0x31;
pub const BPHY_DIVERSITY_CTL: c_uint = 0x32;
pub const BPHY_PEAK_ENERGY_LO: c_uint = 0x33;
pub const BPHY_PEAK_ENERGY_HI: c_uint = 0x34;
pub const BPHY_SYNC_CTL: c_uint = 0x35;
pub const BPHY_TX_PWR_CTRL: c_uint = 0x36;
pub const BPHY_TX_EST_PWR: c_uint = 0x37;
pub const BPHY_STEP: c_uint = 0x38;
pub const BPHY_WARMUP: c_uint = 0x39;
pub const BPHY_LMS_CFF_READ: c_uint = 0x3a;
pub const BPHY_LMS_COEFF_I: c_uint = 0x3b;
pub const BPHY_LMS_COEFF_Q: c_uint = 0x3c;
pub const BPHY_SIG_POW: c_uint = 0x3d;
pub const BPHY_RFDC_CANCEL_CTL: c_uint = 0x3e;
pub const BPHY_HDR_TYPE: c_uint = 0x40;
pub const BPHY_SFD_TO: c_uint = 0x41;
pub const BPHY_SFD_CTL: c_uint = 0x42;
pub const BPHY_DEBUG: c_uint = 0x43;
pub const BPHY_RX_DELAY_COMP: c_uint = 0x44;
pub const BPHY_CRS_DROP_TO: c_uint = 0x45;
pub const BPHY_SHORT_SFD_NZEROS: c_uint = 0x46;
pub const BPHY_DSSS_COEFF1: c_uint = 0x48;
pub const BPHY_DSSS_COEFF2: c_uint = 0x49;
pub const BPHY_CCK_COEFF1: c_uint = 0x4a;
pub const BPHY_CCK_COEFF2: c_uint = 0x4b;
pub const BPHY_TR_CORR: c_uint = 0x4c;
pub const BPHY_ANGLE_SCALE: c_uint = 0x4d;
pub const BPHY_TX_PWR_BASE_IDX: c_uint = 0x4e;
pub const BPHY_OPTIONAL_MODES2: c_uint = 0x4f;
pub const BPHY_CCK_LMS_STEP: c_uint = 0x50;
pub const BPHY_BYPASS: c_uint = 0x51;
pub const BPHY_CCK_DELAY_LONG: c_uint = 0x52;
pub const BPHY_CCK_DELAY_SHORT: c_uint = 0x53;
pub const BPHY_PPROC_CHAN_DELAY: c_uint = 0x54;
pub const BPHY_DDFS_ENABLE: c_uint = 0x58;
pub const BPHY_PHASE_SCALE: c_uint = 0x59;
pub const BPHY_FREQ_CONTROL: c_uint = 0x5a;
pub const BPHY_LNA_GAIN_RANGE_10: c_uint = 0x5b;
pub const BPHY_LNA_GAIN_RANGE_32: c_uint = 0x5c;
pub const BPHY_OPTIONAL_MODES: c_uint = 0x5d;
pub const BPHY_RX_STATUS2: c_uint = 0x5e;
pub const BPHY_RX_STATUS3: c_uint = 0x5f;
pub const BPHY_DAC_CONTROL: c_uint = 0x60;
pub const BPHY_ANA11G_FILT_CTRL: c_uint = 0x62;
pub const BPHY_REFRESH_CTRL: c_uint = 0x64;
pub const BPHY_RF_OVERRIDE2: c_uint = 0x65;
pub const BPHY_SPUR_CANCEL_CTRL: c_uint = 0x66;
pub const BPHY_FINE_DIGIGAIN_CTRL: c_uint = 0x67;
pub const BPHY_RSSI_LUT: c_uint = 0x88;
pub const BPHY_RSSI_LUT_END: c_uint = 0xa7;
pub const BPHY_TSSI_LUT: c_uint = 0xa8;
pub const BPHY_TSSI_LUT_END: c_uint = 0xc7;
pub const BPHY_TSSI2PWR_LUT: c_uint = 0x380;
pub const BPHY_TSSI2PWR_LUT_END: c_uint = 0x39f;
pub const BPHY_LOCOMP_LUT: c_uint = 0x3a0;
pub const BPHY_LOCOMP_LUT_END: c_uint = 0x3bf;
pub const BPHY_TXGAIN_LUT: c_uint = 0x3c0;
pub const BPHY_TXGAIN_LUT_END: c_uint = 0x3ff;
// Bits in BB_CONFIG:
pub const PHY_BBC_ANT_MASK: c_uint = 0x0180;
pub const PHY_BBC_ANT_SHIFT: c_int = 7;
pub const BB_DARWIN: c_uint = 0x1000;
pub const BBCFG_RESETCCA: c_uint = 0x4000;
pub const BBCFG_RESETRX: c_uint = 0x8000;
// Bits in phytest(0x0a):
pub const TST_DDFS: c_uint = 0x2000;
pub const TST_TXFILT1: c_uint = 0x0800;
pub const TST_UNSCRAM: c_uint = 0x0400;
pub const TST_CARR_SUPP: c_uint = 0x0200;
pub const TST_DC_COMP_LOOP: c_uint = 0x0100;
pub const TST_LOOPBACK: c_uint = 0x0080;
pub const TST_TXFILT0: c_uint = 0x0040;
pub const TST_TXTEST_ENABLE: c_uint = 0x0020;
pub const TST_TXTEST_RATE: c_uint = 0x0018;
pub const TST_TXTEST_PHASE: c_uint = 0x0007;
// phytest txTestRate values
pub const TST_TXTEST_RATE_1MBPS: c_int = 0;
pub const TST_TXTEST_RATE_2MBPS: c_int = 1;
pub const TST_TXTEST_RATE_5_5MBPS: c_int = 2;
pub const TST_TXTEST_RATE_11MBPS: c_int = 3;
pub const TST_TXTEST_RATE_SHIFT: c_int = 3;
pub const SHM_BYT_CNT: c_uint = 0x2	/* IHR location */;
pub const MAX_BYT_CNT: c_uint = 0x600	/* Maximum frame len */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d11cnt {
    pub txfrag: u32,
    pub txmulti: u32,
    pub txfail: u32,
    pub txretry: u32,
    pub txretrie: u32,
    pub rxdup: u32,
    pub txrts: u32,
    pub txnocts: u32,
    pub txnoack: u32,
    pub rxfrag: u32,
    pub rxmulti: u32,
    pub rxcrc: u32,
    pub txfrmsnt: u32,
    pub rxundec: u32,
}
