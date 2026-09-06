//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmsmac/main.h
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

// max # brcms_c_module_register() calls
pub const BRCMS_MAXMODULES: c_int = 22;
pub const SEQNUM_SHIFT: c_int = 4;
pub const SEQNUM_MAX: c_uint = 0x1000;

// Maximum wait time for a MAC suspend
// uS: 83mS is max packet time (64KB ampdu @ 6Mbps)
pub const BRCMS_MAX_MAC_SUSPEND: c_int = 83000;
// responses for probe requests older that this are tossed, zero to disable

// transmit buffer max headroom for protocol headers

// Macros for doing definition and get/set of bitfields
// Usage example, e.g. a three-bit field (bits 4-6):
// #define <NAME>_M	BITFIELD_MASK(3)
// #define <NAME>_S	4
// ...
// regval = R_REG(osh, &regs->regfoo);
// field = GFIELD(regval, <NAME>);
// regval = SFIELD(regval, <NAME>, 1);
// W_REG(osh, &regs->regfoo, regval);
//

// max # supported core revisions (0 .. MAXCOREREV - 1)
pub const MAXCOREREV: c_int = 28;
// Double check that unsupported cores are not enabled

// values for shortslot_override

// value for short/long and mixmode/greenfield preamble

// TxFrameID
// seq and frag bits: SEQNUM_SHIFT, FRAGNUM_MASK (802.11.h)
// rate epoch bits: TXFID_RATE_SHIFT, TXFID_RATE_MASK ((wlc_rate.c)
pub const TXFID_QUEUE_MASK: c_uint = 0x0007	/* Bits 0-2 */;
pub const TXFID_SEQ_MASK: c_uint = 0x7FE0	/* Bits 5-15 */;

pub const TXFID_RATE_PROBE_MASK: c_uint = 0x8000	/* Bit 15 for rate probe */;
pub const TXFID_RATE_MASK: c_uint = 0x0018	/* Mask for bits 3 and 4 */;

// promote boardrev
pub const BOARDREV_PROMOTABLE: c_uint = 0xFF	/* from */;

// Ucode MCTL_WAKE override bits
pub const BRCMS_WAKE_OVERRIDE_CLKCTL: c_uint = 0x01;
pub const BRCMS_WAKE_OVERRIDE_PHYREG: c_uint = 0x02;
pub const BRCMS_WAKE_OVERRIDE_MACSUSPEND: c_uint = 0x04;
pub const BRCMS_WAKE_OVERRIDE_TXFIFO: c_uint = 0x08;
pub const BRCMS_WAKE_OVERRIDE_FORCEFAST: c_uint = 0x10;
// stuff pulled in from wlc.c
// Interrupt bit error summary.  Don't include I_RU: we refill DMA at other
// times; and if we run out, constant I_RU interrupts may cause lockup.  We
// will still get error counts from rx0ovfl.
//

// default software intmasks

// frameburst

// PLL requests
// pll is shared on old chips
pub const BRCMS_PLLREQ_SHARED: c_uint = 0x1;
// hold pll for radio monitor register checking
pub const BRCMS_PLLREQ_RADIO_MON: c_uint = 0x2;
// hold/release pll for some short operation
pub const BRCMS_PLLREQ_FLIP: c_uint = 0x4;

//
// 802.11 protection information
//
// _g: use g spec protection, driver internal.
// g_override: override for use of g spec protection.
// gmode_user: user config gmode, operating band->gmode is different.
// overlap: Overlap BSS/IBSS protection for both 11g and 11n.
// nmode_user: user config nmode, operating pub->nmode is different.
// n_cfg: use OFDM protection on MIMO frames.
// n_cfg_override: override for use of N protection.
// nongf: non-GF present protection.
// nongf_override: override for use of GF protection.
// n_pam_override: override for preamble: MM or GF.
// n_obss: indicated OBSS Non-HT STA present.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_protection {
    pub _g: bool,
    pub g_override: i8,
    pub gmode_user: u8,
    pub overlap: i8,
    pub nmode_user: i8,
    pub n_cfg: i8,
    pub n_cfg_override: i8,
    pub nongf: bool,
    pub nongf_override: i8,
    pub n_pam_override: i8,
    pub n_obss: bool,
}

//
// anything affecting the single/dual streams/antenna operation
//
// hw_txchain: HW txchain bitmap cfg.
// txchain: txchain bitmap being used.
// txstreams: number of txchains being used.
// hw_rxchain: HW rxchain bitmap cfg.
// rxchain: rxchain bitmap being used.
// rxstreams: number of rxchains being used.
// ant_rx_ovr: rx antenna override.
// txant: userTx antenna setting.
// phytxant: phyTx antenna setting in txheader.
// ss_opmode: singlestream Operational mode, 0:siso; 1:cdd.
// ss_algosel_auto: if true, use wlc->stf->ss_algo_channel;
// else use wlc->band->stf->ss_mode_band.
// ss_algo_channel: ss based on per-channel algo: 0: SISO, 1: CDD 2: STBC.
// rxchain_restore_delay: delay time to restore default rxchain.
// ldpc: AUTO/ON/OFF ldpc cap supported.
// txcore[MAX_STREAMS_SUPPORTED + 1]: bitmap of selected core for each Nsts.
// spatial_policy:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_stf {
    pub hw_txchain: u8,
    pub txchain: u8,
    pub txstreams: u8,
    pub hw_rxchain: u8,
    pub rxchain: u8,
    pub rxstreams: u8,
    pub ant_rx_ovr: u8,
    pub txant: i8,
    pub phytxant: u16,
    pub ss_opmode: u8,
    pub ss_algosel_auto: bool,
    pub ss_algo_channel: u16,
    pub rxchain_restore_delay: u8,
    pub ldpc: i8,
    pub 1]: u8 txcore[MAX_STREAMS_SUPPORTED +,
    pub spatial_policy: i8,
}

//
// core state (mac)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_core {
    pub /: *mut *mut uint coreidx; / # sb enumerated core,
// fifo
    pub /: *mut *mut *mut uint txavail[NFIFO]; / # tx descriptors available,
    pub /: *mut *mut *mut macstat macstat_snapshot; / mac hw prev read values,
}

//
// band state (phy+ana+radio)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_band {
    pub /: *mut *mut int bandtype; / BRCM_BAND_2G, BRCM_BAND_5G,
    pub /: *mut *mut uint bandunit; / bandstate[] index,
    pub /: *mut *mut u16 phytype; / phytype,
    pub phyrev: u16,
    pub radioid: u16,
    pub radiorev: u16,
    pub /: *mut *mut *mut brcms_phy_pub pi; / pointer to phy specific information,
    pub abgphy_encore: bool,
    pub /: *mut *mut u8 gmode; / currently active gmode,
    pub /: *mut *mut *mut scb hwrs_scb; / permanent scb for hw rateset,
// band-specific copy of default_bss.rateset
    pub defrateset: brcms_c_rateset,
    pub /: *mut *mut u8 band_stf_ss_mode; / Configured STF type, 0:siso; 1:cdd,
    pub /: *mut *mut s8 band_stf_stbc_tx; / STBC TX 0:off; 1:force on; -1:auto,
// rates supported by chip (phy-specific)
    pub hw_rateset: brcms_c_rateset,
    pub /: *mut *mut u8 basic_rate[BRCM_MAXRATE + 1]; / basic rates indexed by rate,
    pub /: *mut *mut bool mimo_cap_40; / 40 MHz cap enabled on this band,
    pub /: *mut *mut s8 antgain; / antenna gain from srom,
    pub /: *mut *mut u16 CWmin; / minimum size of contention window, in unit of aSlotTime,
    pub /: *mut *mut u16 CWmax; / maximum size of contention window, in unit of aSlotTime,
    pub band: ieee80211_supported_band,
}

// module control blocks
#[repr(C)]
#[derive(Copy, Clone)]
pub struct modulecb {
// module name : NULL indicates empty array member
    pub name: [c_char; 32],
// handle passed when handler 'doiovar' is called
    pub hdl: *mut brcms_info,
    pub returned: *mut *mut *mut *mut int (down_fn)(void handle); / down handler. Note: the int,
// by the down function is a count of the
// number of timers that could not be
// freed.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_hw_band {
    pub /: *mut *mut int bandtype; / BRCM_BAND_2G, BRCM_BAND_5G,
    pub /: *mut *mut uint bandunit; / bandstate[] index,
    pub /: *mut *mut u16 mhfs[MHFMAX]; / MHF array shadow,
    pub /: *mut *mut u8 bandhw_stf_ss_mode; / HW configured STF type, 0:siso; 1:cdd,
    pub CWmin: u16,
    pub CWmax: u16,
    pub core_flags: u32,
    pub /: *mut *mut u16 phytype; / phytype,
    pub phyrev: u16,
    pub radioid: u16,
    pub radiorev: u16,
    pub /: *mut *mut *mut brcms_phy_pub pi; / pointer to phy specific information,
    pub abgphy_encore: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_hardware {
    pub /: *mut *mut bool _piomode; / true if pio mode,
    pub wlc: *mut brcms_c_info,
// fifo
    pub /: *mut *mut *mut dma_pub di[NFIFO]; / dma handles, per fifo,
    pub /: *mut *mut uint unit; / device instance number,
// version info
    pub /: *mut *mut u16 vendorid; / PCI vendor id,
    pub /: *mut *mut u16 deviceid; / PCI device id,
    pub /: *mut *mut uint corerev; / core revision,
    pub /: *mut *mut u8 sromrev; / version # of the srom,
    pub /: *mut *mut u16 boardrev; / version # of particular board,
    pub /: *mut *mut u32 boardflags; / Board specific flags from srom,
    pub /: *mut *mut u32 boardflags2; / More board flags if sromrev >= 4,
    pub /: *mut *mut u32 machwcap; / MAC capabilities,
    pub /: *mut *mut u32 machwcap_backup; / backup of machwcap,
    pub /: *mut *mut *mut si_pub sih; / SI handle (cookie for siutils calls),
    pub /: *mut *mut *mut bcma_device d11core; / pointer to 802.11 core,
    pub /: *mut *mut *mut phy_shim_info physhim; / phy shim layer handler,
    pub /: *mut *mut *mut shared_phy phy_sh; / pointer to shared phy state,
    pub /: *mut *mut *mut brcms_hw_band band;/ pointer to active per-band state,
// band state per phy/radio
    pub bandstate: [*mut brcms_hw_band; MAXBANDS],
    pub /: *mut *mut u16 bmac_phytxant; / cache of high phytxant state,
    pub /: *mut *mut bool shortslot; / currently using 11g ShortSlot timing,
    pub /: *mut *mut u16 SRL; / 802.11 dot11ShortRetryLimit,
    pub /: *mut *mut u16 LRL; / 802.11 dot11LongRetryLimit,
    pub /: *mut *mut u16 SFBL; / Short Frame Rate Fallback Limit,
    pub /: *mut *mut u16 LFBL; / Long Frame Rate Fallback Limit,
    pub /: *mut *mut bool up; / d11 hardware up and running,
    pub /: *mut *mut uint now; / # elapsed seconds,
    pub /: *mut *mut uint _nbands; / # bands supported,
    pub /: *mut *mut u16 chanspec; / bmac chanspec shadow,
    pub /: *mut *mut *mut uint txavail[NFIFO]; / # tx descriptors available,
    pub /: *const *const *const u16 xmtfifo_sz; / fifo size in 256B for each xmt fifo,
    pub /: *mut *mut u32 pllreq; / pll requests to keep PLL on,
    pub /: *mut *mut u8 suspended_fifos; / Which TX fifo to remain awake for,
    pub /: *mut *mut u32 maccontrol; / Cached value of maccontrol,
    pub /: *mut *mut uint mac_suspend_depth; / current depth of mac_suspend levels,
    pub /: *mut *mut u32 wake_override; / bit flags to force MAC to WAKE mode,
    pub /: *mut *mut u32 mute_override; / Prevent ucode from sending beacons,
    pub /: *mut *mut u8 etheraddr[ETH_ALEN]; / currently configured ethernet address,
    pub /: *mut *mut bool noreset; / true= do not reset hw, used by WLC_OUT,
    pub /: *mut *mut bool forcefastclk; / true if h/w is forcing to use fast clk,
    pub /: *mut *mut bool clk; / core is out of reset and has clock,
    pub /: *mut *mut bool sbclk; / sb has clock,
    pub /: *mut *mut bool phyclk; / phy is out of reset and has clock,
    pub /: *mut *mut bool ucode_loaded; / true after ucode downloaded,
    pub /: *mut *mut u8 hw_stf_ss_opmode; / STF single stream operation mode,
    pub switch-logic: *mut *mut u8 antsel_type; / Type of boardlevel mimo antenna,
// 0 = N/A, 1 = 2x4 board, 2 = 2x3 CB2 board
//
    pub /*: *mut u32 antsel_avail;,
// put struct antsel_info here if more info is
// needed
//
}

//
// Principal common driver data structure.
//
// pub: pointer to driver public state.
// wl: pointer to specific private state.
// hw: HW related state.
// clkreq_override: setting for clkreq for PCIE : Auto, 0, 1.
// fastpwrup_dly: time in us needed to bring up d11 fast clock.
// macintstatus: bit channel between isr and dpc.
// macintmask: sw runtime master macintmask value.
// defmacintmask: default "on" macintmask value.
// clk: core is out of reset and has clock.
// core: pointer to active io core.
// band: pointer to active per-band state.
// corestate: per-core state (one per hw core).
// bandstate: per-band state (one per phy/radio).
// qvalid: DirFrmQValid and BcMcFrmQValid.
// ampdu: ampdu module handler.
// asi: antsel module handler.
// cmi: channel manager module handler.
// vendorid: PCI vendor id.
// deviceid: PCI device id.
// ucode_rev: microcode revision.
// machwcap: MAC capabilities, BMAC shadow.
// perm_etheraddr: original sprom local ethernet address.
// bandlocked: disable auto multi-band switching.
// bandinit_pending: track band init in auto band.
// radio_monitor: radio timer is running.
// going_down: down path intermediate variable.
// wdtimer: timer for watchdog routine.
// radio_timer: timer for hw radio button monitor routine.
// monitor: monitor (MPDU sniffing) mode.
// bcnmisc_monitor: bcns promisc mode override for monitor.
// _rifs: enable per-packet rifs.
// bcn_li_bcn: beacon listen interval in # beacons.
// bcn_li_dtim: beacon listen interval in # dtims.
// WDarmed: watchdog timer is armed.
// WDlast: last time wlc_watchdog() was called.
// edcf_txop[IEEE80211_NUM_ACS]: current txop for each ac.
// wme_retries: per-AC retry limits.
// bsscfg: set of BSS configurations, idx 0 is default and always valid.
// cfg: the primary bsscfg (can be AP or STA).
// modulecb:
// mimoft: SIGN or 11N.
// cck_40txbw: 11N, cck tx b/w override when in 40MHZ mode.
// ofdm_40txbw: 11N, ofdm tx b/w override when in 40MHZ mode.
// mimo_40txbw: 11N, mimo tx b/w override when in 40MHZ mode.
// default_bss: configured BSS parameters.
// mc_fid_counter: BC/MC FIFO frame ID counter.
// country_default: saved country for leaving 802.11d auto-country mode.
// autocountry_default: initial country for 802.11d auto-country mode.
// prb_resp_timeout: do not send prb resp if request older
// than this, 0 = disable.
// home_chanspec: shared home chanspec.
// chanspec: target operational channel.
// usr_fragthresh: user configured fragmentation threshold.
// fragthresh[NFIFO]: per-fifo fragmentation thresholds.
// RTSThresh: 802.11 dot11RTSThreshold.
// SRL: 802.11 dot11ShortRetryLimit.
// LRL: 802.11 dot11LongRetryLimit.
// SFBL: Short Frame Rate Fallback Limit.
// LFBL: Long Frame Rate Fallback Limit.
// shortslot: currently using 11g ShortSlot timing.
// shortslot_override: 11g ShortSlot override.
// include_legacy_erp: include Legacy ERP info elt ID 47 as well as g ID 42.
// PLCPHdr_override: 802.11b Preamble Type override.
// stf:
// bcn_rspec: save bcn ratespec purpose.
// tempsense_lasttime;
// tx_duty_cycle_ofdm: maximum allowed duty cycle for OFDM.
// tx_duty_cycle_cck: maximum allowed duty cycle for CCK.
// wiphy:
// pri_scb: primary Station Control Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_c_info {
    pub pub: *mut brcms_pub,
    pub wl: *mut brcms_info,
    pub hw: *mut brcms_hardware,
// clock
    pub fastpwrup_dly: u16,
// interrupt
    pub macintstatus: u32,
    pub macintmask: u32,
    pub defmacintmask: u32,
    pub clk: bool,
// multiband
    pub core: *mut brcms_core,
    pub band: *mut brcms_band,
    pub corestate: *mut brcms_core,
    pub bandstate: [*mut brcms_band; MAXBANDS],
// packet queue
    pub qvalid: c_uint,
    pub ampdu: *mut ampdu_info,
    pub asi: *mut antsel_info,
    pub cmi: *mut brcms_cm_info,
    pub vendorid: u16,
    pub deviceid: u16,
    pub ucode_rev: c_uint,
    pub perm_etheraddr: [u8; ETH_ALEN],
    pub bandlocked: bool,
    pub bandinit_pending: bool,
    pub radio_monitor: bool,
    pub going_down: bool,
    pub beacon_template_virgin: bool,
    pub wdtimer: *mut brcms_timer,
    pub radio_timer: *mut brcms_timer,
// promiscuous
    pub filter_flags: c_uint,
// driver feature
    pub _rifs: bool,
// AP-STA synchronization, power save
    pub bcn_li_bcn: u8,
    pub bcn_li_dtim: u8,
    pub WDarmed: bool,
    pub WDlast: u32,
// WME
    pub edcf_txop: [u16; IEEE80211_NUM_ACS],
    pub wme_retries: [u16; IEEE80211_NUM_ACS],
    pub bsscfg: *mut brcms_bss_cfg,
    pub modulecb: *mut modulecb,
    pub mimoft: u8,
    pub cck_40txbw: i8,
    pub ofdm_40txbw: i8,
    pub mimo_40txbw: i8,
    pub default_bss: *mut brcms_bss_info,
    pub mc_fid_counter: u16,
    pub country_default: [c_char; BRCM_CNTRY_BUF_SZ],
    pub autocountry_default: [c_char; BRCM_CNTRY_BUF_SZ],
    pub prb_resp_timeout: u16,
    pub home_chanspec: u16,
// PHY parameters
    pub chanspec: u16,
    pub usr_fragthresh: u16,
    pub fragthresh: [u16; NFIFO],
    pub RTSThresh: u16,
    pub SRL: u16,
    pub LRL: u16,
    pub SFBL: u16,
    pub LFBL: u16,
// network config
    pub shortslot: bool,
    pub shortslot_override: i8,
    pub include_legacy_erp: bool,
    pub protection: *mut brcms_protection,
    pub PLCPHdr_override: i8,
    pub stf: *mut brcms_stf,
    pub bcn_rspec: u32,
    pub tempsense_lasttime: c_uint,
    pub tx_duty_cycle_ofdm: u16,
    pub tx_duty_cycle_cck: u16,
    pub wiphy: *mut wiphy,
    pub pri_scb: scb,
    pub vif: *mut ieee80211_vif,
    pub beacon: *mut sk_buff,
    pub beacon_tim_offset: u16,
    pub beacon_dtim_period: u16,
    pub probe_resp: *mut sk_buff,
}

// antsel module specific state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct antsel_info {
    pub /: *mut *mut *mut brcms_c_info wlc; / pointer to main wlc structure,
    pub /: *mut *mut *mut brcms_pub pub; / pointer to public fn,
    pub switch-logic: *mut *mut u8 antsel_type; / Type of boardlevel mimo antenna,
// 0 = N/A, 1 = 2x4 board, 2 = 2x3 CB2 board
//
    pub /: *mut *mut u8 antsel_antswitch; / board level antenna switch type,
    pub /: *mut *mut bool antsel_avail; / Ant selection availability (SROM based),
    pub /: *mut *mut brcms_antselcfg antcfg_11n; / antenna configuration,
    pub /: *mut *mut brcms_antselcfg antcfg_cur; / current antenna config (auto),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcms_bss_type {
    BRCMS_TYPE_STATION,
    BRCMS_TYPE_AP,
    BRCMS_TYPE_ADHOC,
}

//
// BSS configuration state
//
// wlc: wlc to which this bsscfg belongs to.
// type: interface type
// SSID_len: the length of SSID
// SSID: SSID string
//
// BSSID: BSSID (associated)
// cur_etheraddr: h/w address
// flags: BSSCFG flags; see below
//
// current_bss: BSS parameters in ASSOCIATED state
//
// ID: 'unique' ID of this bsscfg, assigned at bsscfg allocation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_bss_cfg {
    pub wlc: *mut brcms_c_info,
    pub type: brcms_bss_type,
    pub SSID_len: u8,
    pub SSID: [u8; IEEE80211_MAX_SSID_LEN],
    pub BSSID: [u8; ETH_ALEN],
    pub current_bss: brcms_bss_info,
}

extern "C" {
    pub fn brcms_c_txfifo(wlc: *mut brcms_c_info, fifo: c_uint, p: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn brcms_c_set_gmode(wlc: *mut brcms_c_info, gmode: u8, config: bool) -> c_int;
}
extern "C" {
    pub fn brcms_c_mac_promisc(wlc: *mut brcms_c_info, filter_flags: c_uint);
}
extern "C" {
    pub fn brcms_c_calc_lsig_len(wlc: *mut brcms_c_info, ratespec: u32, mac_len: c_uint) -> u16;
}
extern "C" {
    pub fn brcms_c_update_probe_resp(wlc: *mut brcms_c_info, suspend: bool);
}
extern "C" {
    pub fn brcms_c_set_nmode(wlc: *mut brcms_c_info) -> c_int;
}
extern "C" {
    pub fn brcms_c_beacon_phytxctl_txant_upd(wlc: *mut brcms_c_info, bcn_rate: u32);
}
extern "C" {
    pub fn brcms_b_antsel_type_set(wlc_hw: *mut brcms_hardware, antsel_type: u8);
}
extern "C" {
    pub fn brcms_b_write_shm(wlc_hw: *mut brcms_hardware, offset: c_uint, v: u16);
}
extern "C" {
    pub fn brcms_b_read_shm(wlc_hw: *mut brcms_hardware, offset: c_uint) -> u16;
}
extern "C" {
    pub fn brcms_b_corereset(wlc_hw: *mut brcms_hardware, flags: u32);
}
extern "C" {
    pub fn brcms_b_mctrl(wlc_hw: *mut brcms_hardware, mask: u32, val: u32);
}
extern "C" {
    pub fn brcms_b_phy_reset(wlc_hw: *mut brcms_hardware);
}
extern "C" {
    pub fn brcms_b_bw_set(wlc_hw: *mut brcms_hardware, bw: u16);
}
extern "C" {
    pub fn brcms_b_core_phypll_reset(wlc_hw: *mut brcms_hardware);
}
extern "C" {
    pub fn brcms_b_rate_shm_offset(wlc_hw: *mut brcms_hardware, rate: u8) -> u16;
}
extern "C" {
    pub fn brcms_b_switch_macfreq(wlc_hw: *mut brcms_hardware, spurmode: u8);
}
extern "C" {
    pub fn brcms_b_get_txant(wlc_hw: *mut brcms_hardware) -> u16;
}
extern "C" {
    pub fn brcms_b_phyclk_fgc(wlc_hw: *mut brcms_hardware, clk: bool);
}
extern "C" {
    pub fn brcms_b_macphyclk_set(wlc_hw: *mut brcms_hardware, clk: bool);
}
extern "C" {
    pub fn brcms_b_core_phypll_ctl(wlc_hw: *mut brcms_hardware, on: bool);
}
extern "C" {
    pub fn brcms_b_txant_set(wlc_hw: *mut brcms_hardware, phytxant: u16);
}
extern "C" {
    pub fn brcms_b_band_stf_ss_set(wlc_hw: *mut brcms_hardware, stf_mode: u8);
}
extern "C" {
    pub fn brcms_c_init_scb(scb: *mut scb);
}
