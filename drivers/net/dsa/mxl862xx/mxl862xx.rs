//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/mxl862xx/mxl862xx.h
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

pub const MXL862XX_MAX_PORTS: c_int = 17;
pub const MXL862XX_FIRST_SERDES_PORT: c_int = 9;
pub const MXL862XX_SERDES_SLOTS: c_int = 4;
pub const MXL862XX_DEFAULT_BRIDGE: c_int = 0;
pub const MXL862XX_MAX_BRIDGES: c_int = 48;
pub const MXL862XX_MAX_BRIDGE_PORTS: c_int = 128;
pub const MXL862XX_TOTAL_EVLAN_ENTRIES: c_int = 1024;
pub const MXL862XX_TOTAL_VF_ENTRIES: c_int = 1024;
// Number of __le16 words in a firmware portmap (128-bit bitmap).

//
// mxl862xx_fw_portmap_set_bit - set a single port bit in a firmware portmap
// @map: firmware portmap array (MXL862XX_FW_PORTMAP_WORDS entries)
// @port: port index (0..MXL862XX_MAX_BRIDGE_PORTS-1)
//
// mxl862xx_fw_portmap_clear_bit - clear a single port bit in a firmware portmap
// @map: firmware portmap array (MXL862XX_FW_PORTMAP_WORDS entries)
// @port: port index (0..MXL862XX_MAX_BRIDGE_PORTS-1)
//
// mxl862xx_fw_portmap_is_empty - check whether a firmware portmap has no
// bits set
// @map: firmware portmap array (MXL862XX_FW_PORTMAP_WORDS entries)
//
// Return: true if every word in @map is zero.
//
// struct mxl862xx_vf_vid - Per-VID entry within a VLAN Filter block
// @list:     Linked into &mxl862xx_vf_block.vids
// @vid:      VLAN ID
// @index:    Entry index within the VLAN Filter HW block
// @untagged: Strip tag on egress for this VID (drives EVLAN tag-stripping)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_vf_vid {
    pub list: list_head,
    pub vid: u16,
    pub index: u16,
    pub untagged: bool,
}

//
// struct mxl862xx_vf_block - Per-port VLAN Filter block
// @allocated:    Whether the HW block has been allocated via VLANFILTER_ALLOC
// @block_id:     HW VLAN Filter block ID from VLANFILTER_ALLOC
// @block_size:   Total entries allocated in this block
// @active_count: Number of ALLOW entries at indices [0, active_count).
// The bridge port config sends max(active_count, 1) as
// block_size to narrow the HW scan window.
// discard_unmatched_tagged handles frames outside this range.
// @vids:         List of &mxl862xx_vf_vid entries programmed in this block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_vf_block {
    pub allocated: bool,
    pub block_id: u16,
    pub block_size: u16,
    pub active_count: u16,
    pub vids: list_head,
}

//
// struct mxl862xx_evlan_block - Per-port per-direction extended VLAN block
// @allocated:  Whether the HW block has been allocated via EXTENDEDVLAN_ALLOC.
// Guards alloc/free idempotency--the block_id is only valid
// while allocated is true.
// @in_use:     Whether the EVLAN engine should be enabled for this block
// on the bridge port (sent as the enable flag in
// set_bridge_port). Can be false while allocated is still
// true -- e.g. when all egress VIDs are removed (idx == 0 in
// evlan_program_egress) the block stays allocated for
// potential reuse, but the engine is disabled so an empty
// rule set does not discard all traffic.
// @block_id:   HW block ID from EXTENDEDVLAN_ALLOC
// @block_size: Total entries allocated
// @n_active:   Number of HW entries currently written. The bridge port
// config sends this as the egress scan window, so entries
// beyond n_active are never scanned. Always equals
// block_size for ingress blocks (fixed catchall rules).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_evlan_block {
    pub allocated: bool,
    pub in_use: bool,
    pub block_id: u16,
    pub block_size: u16,
    pub n_active: u16,
}

//
// struct mxl862xx_port_stats - 64-bit accumulated hardware port statistics
// @rx_packets: total received packets
// @tx_packets: total transmitted packets
// @rx_bytes: total received bytes
// @tx_bytes: total transmitted bytes
// @rx_errors: total receive errors
// @tx_errors: total transmit errors
// @rx_dropped: total received packets dropped
// @tx_dropped: total transmitted packets dropped
// @multicast: total received multicast packets
// @collisions: total transmit collisions
// @rx_length_errors: received length errors (undersize + oversize)
// @rx_crc_errors: received FCS errors
// @rx_frame_errors: received alignment errors
// @prev_rx_good_pkts: previous snapshot of rx good packet counter
// @prev_tx_good_pkts: previous snapshot of tx good packet counter
// @prev_rx_good_bytes: previous snapshot of rx good byte counter
// @prev_tx_good_bytes: previous snapshot of tx good byte counter
// @prev_rx_fcserror_pkts: previous snapshot of rx FCS error counter
// @prev_rx_under_size_error_pkts: previous snapshot of rx undersize
// error counter
// @prev_rx_oversize_error_pkts: previous snapshot of rx oversize
// error counter
// @prev_rx_align_error_pkts: previous snapshot of rx alignment
// error counter
// @prev_tx_dropped_pkts: previous snapshot of tx dropped counter
// @prev_rx_dropped_pkts: previous snapshot of rx dropped counter
// @prev_rx_evlan_discard_pkts: previous snapshot of extended VLAN
// discard counter
// @prev_mtu_exceed_discard_pkts: previous snapshot of MTU exceed
// discard counter
// @prev_tx_acm_dropped_pkts: previous snapshot of tx ACM dropped
// counter
// @prev_rx_multicast_pkts: previous snapshot of rx multicast counter
// @prev_tx_coll_count: previous snapshot of tx collision counter
//
// The firmware RMON counters are 32-bit free-running (64-bit for byte
// counters). This structure holds 64-bit accumulators alongside the
// previous raw snapshot so that deltas can be computed across polls,
// handling 32-bit wrap correctly via unsigned subtraction.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_port_stats {
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub rx_dropped: u64,
    pub tx_dropped: u64,
    pub multicast: u64,
    pub collisions: u64,
    pub rx_length_errors: u64,
    pub rx_crc_errors: u64,
    pub rx_frame_errors: u64,
    pub prev_rx_good_pkts: u32,
    pub prev_tx_good_pkts: u32,
    pub prev_rx_good_bytes: u64,
    pub prev_tx_good_bytes: u64,
    pub prev_rx_fcserror_pkts: u32,
    pub prev_rx_under_size_error_pkts: u32,
    pub prev_rx_oversize_error_pkts: u32,
    pub prev_rx_align_error_pkts: u32,
    pub prev_tx_dropped_pkts: u32,
    pub prev_rx_dropped_pkts: u32,
    pub prev_rx_evlan_discard_pkts: u32,
    pub prev_mtu_exceed_discard_pkts: u32,
    pub prev_tx_acm_dropped_pkts: u32,
    pub prev_rx_multicast_pkts: u32,
    pub prev_tx_coll_count: u32,
}

//
// struct mxl862xx_port - per-port state tracked by the driver
// @priv:                back-pointer to switch private data; needed by
// deferred work handlers to access ds and priv
// @fid:                 firmware FID for the permanent single-port bridge;
// kept alive for the lifetime of the port so traffic is
// never forwarded while the port is unbridged
// @flood_block:         bitmask of firmware meter indices that are currently
// rate-limiting flood traffic on this port (zero-rate
// meters used to block flooding)
// @learning:            true when address learning is enabled on this port
// @setup_done:          set at end of port_setup, cleared at start of
// port_teardown; guards deferred work against
// acting on torn-down state
// @pvid:                port VLAN ID (native VLAN) assigned to untagged traffic
// @vlan_filtering:      true when VLAN filtering is enabled on this port
// @vf:                  per-port VLAN Filter block state
// @ingress_evlan:       ingress extended VLAN block state
// @egress_evlan:        egress extended VLAN block state
// @host_flood_uc:       desired host unicast flood state (true = flood);
// updated atomically by port_set_host_flood, consumed
// by the deferred host_flood_work
// @host_flood_mc:       desired host multicast flood state (true = flood)
// @host_flood_work:     deferred work for applying host flood changes;
// port_set_host_flood runs in atomic context (under
// netif_addr_lock) so firmware calls must be deferred.
// The worker acquires rtnl_lock() to serialize with
// DSA callbacks and checks @setup_done to avoid
// acting on torn-down ports.
// @stats:               64-bit accumulated hardware statistics; updated
// periodically by the stats polling work
// @stats_lock:          protects accumulator reads in .get_stats64 against
// concurrent updates from the polling work
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_port {
    pub priv: *mut mxl862xx_priv,
    pub fid: u16,
    pub flood_block: c_ulong,
    pub learning: bool,
    pub setup_done: bool,
    pub pvid: u16,
    pub vlan_filtering: bool,
    pub vf: mxl862xx_vf_block,
    pub ingress_evlan: mxl862xx_evlan_block,
    pub egress_evlan: mxl862xx_evlan_block,
    pub host_flood_uc: bool,
    pub host_flood_mc: bool,
    pub host_flood_work: work_struct,
    pub stats: mxl862xx_port_stats,
    pub /: *mut *mut spinlock_t stats_lock; / protects stats accumulators,
}

//
// struct mxl862xx_pcs - link SerDes interfaces to bridge ports
// @pcs:       &struct phylink_pcs instance
// @priv:      pointer to &struct mxl862xx_priv
// @serdes_id: SerDes instance index (0 or 1)
// @slot:      slot within the SerDes (0-3 for QSGMII/QUSXGMII, 0 otherwise)
// @interface: cached PHY interface, last value passed to pcs_config().
// %PHY_INTERFACE_MODE_NA before the first successful
// pcs_config().  Used by pcs_an_restart() to populate the
// firmware command and by pcs_disable() to skip the
// firmware power-down for shared (QSGMII/QUSXGMII) modes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_pcs {
    pub pcs: phylink_pcs,
    pub priv: *mut mxl862xx_priv,
    pub serdes_id: c_int,
    pub slot: c_int,
    pub interface: phy_interface_t,
}

//
// struct mxl862xx_fw_version - firmware version for comparison and display
// @major: firmware major version
// @minor: firmware minor version
// @revision: firmware revision number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_fw_version {
    pub major: u8,
    pub minor: u8,
    pub revision: u16,
}

// Bit indices for struct mxl862xx_priv::flags
pub const MXL862XX_FLAG_CRC_ERR: c_int = 0;
pub const MXL862XX_FLAG_WORK_STOPPED: c_int = 1;
//
// struct mxl862xx_priv - driver private data for an MxL862xx switch
// @ds:                 pointer to the DSA switch instance
// @mdiodev:            MDIO device used to communicate with the switch firmware
// @crc_err_work:       deferred work for shutting down all ports on MDIO CRC
// errors
// @flags:              atomic status flags; %MXL862XX_FLAG_CRC_ERR is set
// before CRC-triggered shutdown and cleared after;
// %MXL862XX_FLAG_WORK_STOPPED is set before cancelling
// stats_work to prevent rescheduling during teardown
// @drop_meter:         index of the single shared zero-rate firmware meter
// used to unconditionally drop traffic (used to block
// flooding)
// @fw_version:         cached firmware version, populated at probe and
// compared with MXL862XX_FW_VER_MIN()
// @serdes_ports:       SerDes interfaces incl. sub-interfaces in case of
// 10G_QXGMII or QSGMII
// @serdes_refcount:    per-XPCS count of sub-ports enabled by phylink;
// pcs_disable powers an XPCS down when the count
// reaches zero. Protected by @serdes_lock.
// @serdes_lock:        serializes the @serdes_refcount transitions with
// the XPCS power-down so a sibling sub-port enable
// cannot race a power-down to zero
// @ports:              per-port state, indexed by switch port number
// @bridges:            maps DSA bridge number to firmware bridge ID;
// zero means no firmware bridge allocated for that
// DSA bridge number. Indexed by dsa_bridge.num
// (0 .. ds->max_num_bridges).
// @evlan_ingress_size: per-port ingress Extended VLAN block size
// @evlan_egress_size:  per-port egress Extended VLAN block size
// @vf_block_size:      per-port VLAN Filter block size
// @stats_work:         periodic work item that polls RMON hardware counters
// and accumulates them into 64-bit per-port stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_priv {
    pub ds: *mut dsa_switch,
    pub mdiodev: *mut mdio_device,
    pub crc_err_work: work_struct,
    pub flags: c_ulong,
    pub drop_meter: u16,
    pub fw_version: mxl862xx_fw_version,
    pub serdes_ports: [mxl862xx_pcs; 8],
    pub serdes_refcount: [c_int; 2],
    pub serdes_lock: mutex,
    pub ports: [mxl862xx_port; MXL862XX_MAX_PORTS],
    pub 1]: u16 bridges[MXL862XX_MAX_BRIDGES +,
    pub evlan_ingress_size: u16,
    pub evlan_egress_size: u16,
    pub vf_block_size: u16,
    pub stats_work: delayed_work,
}
