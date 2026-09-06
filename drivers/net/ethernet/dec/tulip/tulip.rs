//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/dec/tulip/tulip.h
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

// undefine, or define to various debugging levels (>4 == obscene levels)
pub const TULIP_DEBUG: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tulip_chip_table {
    pub chip_name: *mut c_char,
    pub io_size: c_int,
    pub /: *mut *mut int valid_intrs; / CSR7 interrupt enable settings,
    pub flags: c_int,
    pub ): *mut *mut void (media_timer) (struct timer_list,
    pub media_task: work_func_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tbl_flag {
    HAS_MII			= 0x00001,
    HAS_MEDIA_TABLE		= 0x00002,
    CSR12_IN_SROM		= 0x00004,
    ALWAYS_CHECK_MII	= 0x00008,
    HAS_ACPI		= 0x00010,
    MC_HASH_ONLY		= 0x00020, /* Hash-only multicast filter. */
    HAS_PNICNWAY		= 0x00080,
    HAS_NWAY		= 0x00040, /* Uses internal NWay xcvr. */
    HAS_INTR_MITIGATION	= 0x00100,
    IS_ASIX			= 0x00200,
    HAS_8023X		= 0x00400,
    COMET_MAC_ADDR		= 0x00800,
    HAS_PCI_MWI		= 0x01000,
    HAS_PHY_IRQ		= 0x02000,
    HAS_SWAPPED_SEEPROM	= 0x04000,
    NEEDS_FAKE_MEDIA_TABLE	= 0x08000,
    COMET_PM		= 0x10000,
}

// chip types.  careful!  order is VERY IMPORTANT here, as these
// are used throughout the driver as indices into arrays
// Note 21142 == 21143.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chips {
    DC21040 = 0,
    DC21041 = 1,
    DC21140 = 2,
    DC21142 = 3, DC21143 = 3,
    LC82C168,
    MX98713,
    MX98715,
    MX98725,
    AX88140,
    PNIC2,
    COMET,
    COMPEX9881,
    I21145,
    DM910X,
    CONEXANT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MediaIs {
    MediaIsFD = 1,
    MediaAlwaysFD = 2,
    MediaIsMII = 4,
    MediaIsFx = 8,
    MediaIs100 = 16
}

// Offsets to the Command and Status Registers, "CSRs".  All accesses
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tulip_offsets {
    CSR0 = 0,
    CSR1 = 0x08,
    CSR2 = 0x10,
    CSR3 = 0x18,
    CSR4 = 0x20,
    CSR5 = 0x28,
    CSR6 = 0x30,
    CSR7 = 0x38,
    CSR8 = 0x40,
    CSR9 = 0x48,
    CSR10 = 0x50,
    CSR11 = 0x58,
    CSR12 = 0x60,
    CSR13 = 0x68,
    CSR14 = 0x70,
    CSR15 = 0x78,
    CSR18 = 0x88,
    CSR19 = 0x8c,
    CSR20 = 0x90,
    CSR27 = 0xAC,
    CSR28 = 0xB0,
}

// register offset and bits for CFDD PCI config reg
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_cfg_driver_reg {
    CFDD = 0x40,
    CFDD_Sleep = (1 << 31),
    CFDD_Snooze = (1 << 30),
}

// The bits in the CSR5 status registers, mostly interrupt sources.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum status_bits {
    TimerInt = 0x800,
    SystemError = 0x2000,
    TPLnkFail = 0x1000,
    TPLnkPass = 0x10,
    NormalIntr = 0x10000,
    AbnormalIntr = 0x8000,
    RxJabber = 0x200,
    RxDied = 0x100,
    RxNoBuf = 0x80,
    RxIntr = 0x40,
    TxFIFOUnderflow = 0x20,
    RxErrIntr = 0x10,
    TxJabber = 0x08,
    TxNoBuf = 0x04,
    TxDied = 0x02,
    TxIntr = 0x01,
}

// bit mask for CSR5 TX/RX process state
pub const CSR5_TS: c_uint = 0x00700000;
pub const CSR5_RS: c_uint = 0x000e0000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tulip_mode_bits {
    TxThreshold		= (1 << 22),
    FullDuplex		= (1 << 9),
    TxOn			= 0x2000,
    AcceptBroadcast		= 0x0100,
    AcceptAllMulticast	= 0x0080,
    AcceptAllPhys		= 0x0040,
    AcceptRunt		= 0x0008,
    RxOn			= 0x0002,
    RxTx			= (TxOn | RxOn),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tulip_busconfig_bits {
    MWI			= (1 << 24),
    MRL			= (1 << 23),
    MRM			= (1 << 21),
    CALShift		= 14,
    BurstLenShift		= 8,
}

// The Tulip Rx and Tx buffer descriptors.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tulip_rx_desc {
    pub status: __le32,
    pub length: __le32,
    pub buffer1: __le32,
    pub buffer2: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tulip_tx_desc {
    pub status: __le32,
    pub length: __le32,
    pub buffer1: __le32,
    pub /: *mut *mut __le32 buffer2; / We use only buffer 1.,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum desc_status_bits {
    DescOwned    = 0x80000000,
    DescWholePkt = 0x60000000,
    DescEndPkt   = 0x40000000,
    DescStartPkt = 0x20000000,
    DescEndRing  = 0x02000000,
    DescUseLink  = 0x01000000,

//
// Error summary flag is logical or of 'CRC Error', 'Collision Seen',
// 'Frame Too Long', 'Runt' and 'Descriptor Error' flags generated
// within tulip chip.
//
    RxDescErrorSummary = 0x8000,
    RxDescCRCError = 0x0002,
    RxDescCollisionSeen = 0x0040,

//
// 'Frame Too Long' flag is set if packet length including CRC exceeds
// 1518.  However, a full sized VLAN tagged frame is 1522 bytes
// including CRC.
//
// The tulip chip does not block oversized frames, and if this flag is
// set on a receive descriptor it does not indicate the frame has been
// truncated.  The receive descriptor also includes the actual length.
// Therefore we can safety ignore this flag and check the length
// ourselves.
//
    RxDescFrameTooLong = 0x0080,
    RxDescRunt = 0x0800,
    RxDescDescErr = 0x4000,
    RxWholePkt   = 0x00000300,
//
// Top three bits of 14 bit frame length (status bits 27-29) should
// never be set as that would make frame over 2047 bytes. The Receive
// Watchdog flag (bit 4) may indicate the length is over 2048 and the
// length field is invalid.
//
    RxLengthOver2047 = 0x38000010
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum t21143_csr6_bits {
    csr6_sc = (1<<31),
    csr6_ra = (1<<30),
    csr6_ign_dest_msb = (1<<26),
    csr6_mbo = (1<<25),
    csr6_scr = (1<<24),  /* scramble mode flag: can't be set */
    csr6_pcs = (1<<23),  /* Enables PCS functions (symbol mode requires csr6_ps be set) default is set */
    csr6_ttm = (1<<22),  /* Transmit Threshold Mode, set for 10baseT, 0 for 100BaseTX */
    csr6_sf = (1<<21),   /* Store and forward. If set ignores TR bits */
    csr6_hbd = (1<<19),  /* Heart beat disable. Disables SQE function in 10baseT */
    csr6_ps = (1<<18),   /* Port Select. 0 (defualt) = 10baseT, 1 = 100baseTX: can't be set */
    csr6_ca = (1<<17),   /* Collision Offset Enable. If set uses special algorithm in low collision situations */
    csr6_trh = (1<<15),  /* Transmit Threshold high bit */
    csr6_trl = (1<<14),  /* Transmit Threshold low bit */

//
// This table shows transmit threshold values based on media
// and these two registers (from PNIC1 & 2 docs) Note: this is
// all meaningless if sf is set.
//

//
// (trh,trl) * 100BaseTX * 10BaseT
//
// (0,0)   *     128   *    72
// (0,1)   *     256   *    96
// (1,0)   *     512   *   128
// (1,1)   *    1024   *   160
//

    csr6_fc = (1<<12),   /* Forces a collision in next transmission (for testing in loopback mode) */
    csr6_om_int_loop = (1<<10), /* internal (FIFO) loopback flag */
    csr6_om_ext_loop = (1<<11), /* external (PMD) loopback flag */
// set both and you get (PHY) loopback
    csr6_fd = (1<<9),    /* Full duplex mode, disables hearbeat, no loopback */
    csr6_pm = (1<<7),    /* Pass All Multicast */
    csr6_pr = (1<<6),    /* Promiscuous mode */
    csr6_sb = (1<<5),    /* Start(1)/Stop(0) backoff counter */
    csr6_if = (1<<4),    /* Inverse Filtering, rejects only addresses in address table: can't be set */
    csr6_pb = (1<<3),    /* Pass Bad Frames, (1) causes even bad frames to be passed on */
    csr6_ho = (1<<2),    /* Hash-only filtering mode: can't be set */
    csr6_hp = (1<<0),    /* Hash/Perfect Receive Filtering Mode: can't be set */

    csr6_mask_capture = (csr6_sc | csr6_ca),
    csr6_mask_defstate = (csr6_mask_capture | csr6_mbo),
    csr6_mask_hdcap = (csr6_mask_defstate | csr6_hbd | csr6_ps),
    csr6_mask_hdcaptt = (csr6_mask_hdcap  | csr6_trh | csr6_trl),
    csr6_mask_fullcap = (csr6_mask_hdcaptt | csr6_fd),
    csr6_mask_fullpromisc = (csr6_pr | csr6_pm),
    csr6_mask_filters = (csr6_hp | csr6_ho | csr6_if),
    csr6_mask_100bt = (csr6_scr | csr6_pcs | csr6_hbd),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tulip_comet_csr13_bits {
// The LINKOFFE and LINKONE work in conjunction with LSCE, i.e. they
// determine which link status transition wakes up if LSCE is
// enabled
    comet_csr13_linkoffe = (1 << 17),
    comet_csr13_linkone = (1 << 16),
    comet_csr13_wfre = (1 << 10),
    comet_csr13_mpre = (1 << 9),
    comet_csr13_lsce = (1 << 8),
    comet_csr13_wfr = (1 << 2),
    comet_csr13_mpr = (1 << 1),
    comet_csr13_lsc = (1 << 0),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tulip_comet_csr18_bits {
    comet_csr18_pmes_sticky = (1 << 24),
    comet_csr18_pm_mode = (1 << 19),
    comet_csr18_apm_mode = (1 << 18),
    comet_csr18_d3a = (1 << 7)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tulip_comet_csr20_bits {
    comet_csr20_pmes = (1 << 15),
}

// Keep the ring sizes a power of two for efficiency.
pub const TX_RING_SIZE: c_int = 32;
pub const RX_RING_SIZE: c_int = 128;
pub const MEDIA_MASK: c_int = 31;
// The receiver on the DC21143 rev 65 can fail to close the last
// receive descriptor in certain circumstances (see errata) when
// using MWI. This can only occur if the receive buffer ends on
// a cache line boundary, so the "+ 4" below ensures it doesn't.
//

// The UltraSparc PCI controllers will disconnect at every 64-byte
// crossing anyways so it makes no sense to tell Tulip to burst
// any more than that.
//

// Ring-wrap flag in length field, use for last ring entry.
//
pub const DESC_RING_WRAP: c_uint = 0x02000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct medialeaf {
    pub type: u8,
    pub media: u8,
    pub leafdata: *mut c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mediatable {
    pub defaultmedia: u16,
    pub leafcount: u8,
    pub /: *mut *mut u8 csr12dir; / General purpose pin directions.,
    pub has_mii:1: unsigned,
    pub has_nonmii:1: unsigned,
    pub has_reset:6: unsigned,
    pub csr15dir: u32,
    pub /: *mut *mut u32 csr15val; / 21143 NWay setting.,
    pub __counted_by(leafcount): medialeaf mleaf[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mediainfo {
    pub next: *mut mediainfo,
    pub info_type: c_int,
    pub index: c_int,
    pub info: *mut c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_info {
    pub skb: *mut sk_buff,
    pub mapping: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tulip_private {
    pub product_name: *const c_char,
    pub next_module: *mut net_device,
    pub rx_ring: *mut tulip_rx_desc,
    pub tx_ring: *mut tulip_tx_desc,
    pub rx_ring_dma: dma_addr_t,
    pub tx_ring_dma: dma_addr_t,
// The saved address of a sent-in-place packet/buffer, for skfree().
    pub tx_buffers: [ring_info; TX_RING_SIZE],
// The addresses of receive-in-place skbuffs.
    pub rx_buffers: [ring_info; RX_RING_SIZE],
    pub /: *mut *mut u16 setup_frame[96]; / Pseudo-Tx frame to init address table.,
    pub chip_id: c_int,
    pub revision: c_int,
    pub flags: c_int,
    pub napi: napi_struct,
    pub /: *mut *mut timer_list timer; / Media selection timer.,
    pub /: *mut *mut timer_list oom_timer; / Out of memory timer.,
    pub mc_filter: [u32; 2],
    pub lock: spinlock_t,
    pub mii_lock: spinlock_t,
    pub /: *mut *mut unsigned int cur_rx, cur_tx; / The next free ring entry,
    pub /: *mut *mut unsigned int dirty_rx, dirty_tx; / The ring entries to be free()ed.,

    pub mit_on: c_int,

    pub /: *mut *mut unsigned int full_duplex:1; / Full-duplex operation requested.,
    pub full_duplex_lock:1: c_uint,
    pub /: *mut *mut unsigned int fake_addr:1; / Multiport board faked address.,
    pub /: *mut *mut unsigned int default_port:4; / Last dev->if_port value.,
    pub /: *mut *mut unsigned int media2:4; / Secondary monitored media port.,
    pub /: *mut *mut unsigned int medialock:1; / Don't sense media type.,
    pub /: *mut *mut unsigned int mediasense:1; / Media sensing in progress.,
    pub /: *mut *mut unsigned int nway:1, nwayset:1; / 21143 internal NWay.,
    pub timeout_recovery:1: c_uint,
    pub /: *mut *mut unsigned int csr0; / CSR0 setting.,
    pub /: *mut *mut unsigned int csr6; / Current CSR6 control settings.,
    pub /: *mut *mut unsigned char eeprom[EEPROM_SIZE]; / Serial EEPROM contents.,
    pub csr5): *mut *mut *mut void (link_change) (struct net_device  dev, int,
    pub /: *mut *mut ethtool_wolinfo wolinfo; / WOL settings,
    pub /: *mut *mut u16 sym_advertise, mii_advertise; / NWay capabilities advertised.,
    pub /: *mut *mut u16 lpar; / 21143 Link partner ability.,
    pub advertising: [u16; 4],
    pub /: *mut *mut signed char phys[4], mii_cnt; / MII device addresses.,
    pub mtable: *mut mediatable,
    pub /: *mut *mut int cur_index; / Current media index.,
    pub saved_if_port: c_int,
    pub pdev: *mut pci_dev,
    pub ttimer: c_int,
    pub susp_rx: c_int,
    pub nir: c_ulong,
    pub base_addr: *mut void __iomem,
    pub csr12_shadow: c_int,
    pub /: *mut *mut int pad0; / Used for 8-byte alignment,
    pub media_work: work_struct,
    pub dev: *mut net_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeprom_fixup {
    pub name: *mut c_char,
    pub addr0: c_uchar,
    pub addr1: c_uchar,
    pub addr2: c_uchar,
    pub /: *mut *mut u16 newtable[32]; / Max length below.,
}

// 21142.c
extern "C" {
    pub fn t21142_media_task(work: *mut work_struct);
}
extern "C" {
    pub fn t21142_start_nway(dev: *mut net_device);
}
extern "C" {
    pub fn t21142_lnk_change(dev: *mut net_device, csr5: c_int);
}
// PNIC2.c
extern "C" {
    pub fn pnic2_lnk_change(dev: *mut net_device, csr5: c_int);
}
extern "C" {
    pub fn pnic2_timer(t: *mut timer_list);
}
extern "C" {
    pub fn pnic2_start_nway(dev: *mut net_device);
}
// eeprom.c
extern "C" {
    pub fn tulip_parse_eeprom(dev: *mut net_device);
}
extern "C" {
    pub fn tulip_read_eeprom(dev: *mut net_device, location: c_int, addr_len: c_int) -> c_int;
}
// interrupt.c
extern "C" {
    pub fn tulip_interrupt(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn tulip_refill_rx(dev: *mut net_device) -> c_int;
}

extern "C" {
    pub fn tulip_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}

// media.c
extern "C" {
    pub fn tulip_mdio_read(dev: *mut net_device, phy_id: c_int, location: c_int) -> c_int;
}
extern "C" {
    pub fn tulip_mdio_write(dev: *mut net_device, phy_id: c_int, location: c_int, value: c_int);
}
extern "C" {
    pub fn tulip_select_media(dev: *mut net_device, startup: c_int);
}
extern "C" {
    pub fn tulip_check_duplex(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn tulip_find_mii(dev: *mut net_device, board_idx: c_int);
}
// pnic.c
extern "C" {
    pub fn pnic_do_nway(dev: *mut net_device);
}
extern "C" {
    pub fn pnic_lnk_change(dev: *mut net_device, csr5: c_int);
}
extern "C" {
    pub fn pnic_timer(t: *mut timer_list);
}
// timer.c
extern "C" {
    pub fn tulip_media_task(work: *mut work_struct);
}
extern "C" {
    pub fn mxic_timer(t: *mut timer_list);
}
extern "C" {
    pub fn comet_timer(t: *mut timer_list);
}
// tulip_core.c
extern "C" {
    pub fn oom_timer(t: *mut timer_list);
}
// wait until in-flight frame completes.
// Max time @ 10BT: 1500*8b/10Mbps == 1200us (+ 100us margin)
// Typically expect this loop to end in < 50 us on 100BT.
//
// Stop and restart the chip's Tx processes.
// Trigger an immediate transmit demand.
