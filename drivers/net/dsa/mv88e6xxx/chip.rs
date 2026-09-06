//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/mv88e6xxx/chip.h
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
// Marvell 88E6xxx Ethernet switch single-chip definition
//
// Copyright (c) 2008 Marvell Semiconductor
//

pub const EDSA_HLEN: c_int = 8;
pub const MV88E6XXX_N_FID: c_int = 4096;
pub const MV88E6XXX_N_SID: c_int = 64;
pub const MV88E6XXX_FID_STANDALONE: c_int = 0;
pub const MV88E6XXX_FID_BRIDGED: c_int = 1;
// PVT limits for 4-bit port and 5-bit switch
pub const MV88E6XXX_MAX_PVT_SWITCHES: c_int = 32;
pub const MV88E6XXX_MAX_PVT_PORTS: c_int = 16;

pub const MV88E6XXX_MAX_GPIO: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mv88e6xxx_egress_mode {
    MV88E6XXX_EGRESS_MODE_UNMODIFIED,
    MV88E6XXX_EGRESS_MODE_UNTAGGED,
    MV88E6XXX_EGRESS_MODE_TAGGED,
    MV88E6XXX_EGRESS_MODE_ETHERTYPE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mv88e6xxx_egress_direction {
    MV88E6XXX_EGRESS_DIR_INGRESS,
    MV88E6XXX_EGRESS_DIR_EGRESS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mv88e6xxx_frame_mode {
    MV88E6XXX_FRAME_MODE_NORMAL,
    MV88E6XXX_FRAME_MODE_DSA,
    MV88E6XXX_FRAME_MODE_PROVIDER,
    MV88E6XXX_FRAME_MODE_ETHERTYPE,
}

// List of supported models
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mv88e6xxx_model {
    MV88E6020,
    MV88E6071,
    MV88E6085,
    MV88E6095,
    MV88E6097,
    MV88E6123,
    MV88E6131,
    MV88E6141,
    MV88E6161,
    MV88E6165,
    MV88E6171,
    MV88E6172,
    MV88E6175,
    MV88E6176,
    MV88E6185,
    MV88E6190,
    MV88E6190X,
    MV88E6191,
    MV88E6191X,
    MV88E6193X,
    MV88E6220,
    MV88E6240,
    MV88E6250,
    MV88E6290,
    MV88E6320,
    MV88E6321,
    MV88E6341,
    MV88E6350,
    MV88E6351,
    MV88E6352,
    MV88E6361,
    MV88E6390,
    MV88E6390X,
    MV88E6393X,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mv88e6xxx_family {
    MV88E6XXX_FAMILY_NONE,
    MV88E6XXX_FAMILY_6065,	/* 6031 6035 6061 6065 */
    MV88E6XXX_FAMILY_6095,	/* 6092 6095 */
    MV88E6XXX_FAMILY_6097,	/* 6046 6085 6096 6097 */
    MV88E6XXX_FAMILY_6165,	/* 6123 6161 6165 */
    MV88E6XXX_FAMILY_6185,	/* 6108 6121 6122 6131 6152 6155 6182 6185 */
    MV88E6XXX_FAMILY_6250,	/* 6220 6250 6020 6071 */
    MV88E6XXX_FAMILY_6320,	/* 6320 6321 */
    MV88E6XXX_FAMILY_6341,	/* 6141 6341 */
    MV88E6XXX_FAMILY_6351,	/* 6171 6175 6350 6351 */
    MV88E6XXX_FAMILY_6352,	/* 6172 6176 6240 6352 */
    MV88E6XXX_FAMILY_6390,  /* 6190 6190X 6191 6290 6390 6390X */
    MV88E6XXX_FAMILY_6393,	/* 6191X 6193X 6361 6393X */
}

//
// enum mv88e6xxx_edsa_support - Ethertype DSA tag support level
// @MV88E6XXX_EDSA_UNSUPPORTED:  Device has no support for EDSA tags
// @MV88E6XXX_EDSA_UNDOCUMENTED: Documentation indicates that
// egressing FORWARD frames with an EDSA
// tag is reserved for future use, but
// empirical data shows that this mode
// is supported.
// @MV88E6XXX_EDSA_SUPPORTED:    EDSA tags are fully supported.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mv88e6xxx_edsa_support {
    MV88E6XXX_EDSA_UNSUPPORTED = 0,
    MV88E6XXX_EDSA_UNDOCUMENTED,
    MV88E6XXX_EDSA_SUPPORTED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_info {
    pub family: mv88e6xxx_family,
    pub prod_num: u16,
    pub name: *const c_char,
    pub num_databases: c_uint,
    pub num_macs: c_uint,
    pub num_ports: c_uint,
    pub num_internal_phys: c_uint,
    pub num_gpio: c_uint,
    pub num_tcam_entries: c_uint,
    pub max_vid: c_uint,
    pub max_sid: c_uint,
    pub port_base_addr: c_uint,
    pub phy_base_addr: c_uint,
    pub global1_addr: c_uint,
    pub global2_addr: c_uint,
    pub tcam_addr: c_uint,
    pub age_time_coeff: c_uint,
    pub g1_irqs: c_uint,
    pub g2_irqs: c_uint,
    pub stats_type: c_int,
    pub pvt: bool,
// Mark certain ports as invalid. This is required for example for the
// MV88E6220 (which is in general a MV88E6250 with 7 ports) but the
// ports 2-4 are not routet to pins.
//
    pub invalid_port_mask: c_uint,
// Multi-chip Addressing Mode.
// Some chips respond to only 2 registers of its own SMI device address
// when it is non-zero, and use indirect access to internal registers.
//
    pub multi_chip: bool,
// Dual-chip Addressing Mode
// Some chips respond to only half of the 32 SMI addresses,
// allowing two to coexist on the same SMI interface.
//
    pub dual_chip: bool,
    pub edsa_support: mv88e6xxx_edsa_support,
// Mask for FromPort and ToPort value of PortVec used in ATU Move
// operation. 0 means that the ATU Move operation is not supported.
//
    pub atu_move_port_mask: u8,
    pub ops: *const mv88e6xxx_ops,
// Supports PTP
    pub ptp_support: bool,
// Internal PHY start index. 0 means that internal PHYs range starts at
// port 0, 1 means internal PHYs range starts at port 1, etc
//
    pub internal_phys_offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_atu_entry {
    pub state: u8,
    pub trunk: bool,
    pub portvec: u16,
    pub mac: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_vtu_entry {
    pub vid: u16,
    pub fid: u16,
    pub sid: u8,
    pub valid: bool,
    pub policy: bool,
    pub member: [u8; DSA_MAX_PORTS],
    pub /: *mut *mut u8 state[DSA_MAX_PORTS]; / Older silicon has no STU,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_stu_entry {
    pub sid: u8,
    pub valid: bool,
    pub state: [u8; DSA_MAX_PORTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_irq {
    pub masked: u16,
    pub chip: irq_chip,
    pub domain: *mut irq_domain,
    pub nirqs: c_int,
}

// state flags for mv88e6xxx_port_hwtstamp::state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_port_hwtstamp {
// Port index
    pub port_id: c_int,
// Timestamping state
    pub state: c_ulong,
// Resources for receive timestamping
    pub rx_queue: sk_buff_head,
    pub rx_queue2: sk_buff_head,
// Resources for transmit timestamping
    pub tx_tstamp_start: c_ulong,
    pub tx_skb: *mut sk_buff,
    pub tx_seq_id: u16,
// Current timestamp configuration
    pub tstamp_config: kernel_hwtstamp_config,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mv88e6xxx_policy_mapping {
    MV88E6XXX_POLICY_MAPPING_DA,
    MV88E6XXX_POLICY_MAPPING_SA,
    MV88E6XXX_POLICY_MAPPING_VTU,
    MV88E6XXX_POLICY_MAPPING_ETYPE,
    MV88E6XXX_POLICY_MAPPING_PPPOE,
    MV88E6XXX_POLICY_MAPPING_VBAS,
    MV88E6XXX_POLICY_MAPPING_OPT82,
    MV88E6XXX_POLICY_MAPPING_UDP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mv88e6xxx_policy_action {
    MV88E6XXX_POLICY_ACTION_NORMAL,
    MV88E6XXX_POLICY_ACTION_MIRROR,
    MV88E6XXX_POLICY_ACTION_TRAP,
    MV88E6XXX_POLICY_ACTION_DISCARD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_policy {
    pub mapping: mv88e6xxx_policy_mapping,
    pub action: mv88e6xxx_policy_action,
    pub fs: ethtool_rx_flow_spec,
    pub addr: [u8; ETH_ALEN],
    pub port: c_int,
    pub vid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_vlan {
    pub vid: u16,
    pub valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_port {
    pub chip: *mut mv88e6xxx_chip,
    pub port: c_int,
    pub fwnode: *mut fwnode_handle,
    pub bridge_pvid: mv88e6xxx_vlan,
    pub serdes_stats: [u64; 2],
    pub atu_member_violation: u64,
    pub atu_miss_violation: u64,
    pub atu_full_violation: u64,
    pub vtu_member_violation: u64,
    pub vtu_miss_violation: u64,
    pub interface: phy_interface_t,
    pub cmode: u8,
    pub mirror_ingress: bool,
    pub mirror_egress: bool,
    pub region: *mut devlink_region,
    pub pcs_private: *mut c_void,
// LED related information
    pub fiber: bool,
    pub led0: led_classdev,
    pub led1: led_classdev,
// MacAuth Bypass control flag
    pub mab: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mv88e6xxx_region_id {
    MV88E6XXX_REGION_GLOBAL1 = 0,
    MV88E6XXX_REGION_GLOBAL2,
    MV88E6XXX_REGION_ATU,
    MV88E6XXX_REGION_VTU,
    MV88E6XXX_REGION_STU,
    MV88E6XXX_REGION_PVT,

    _MV88E6XXX_REGION_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_region_priv {
    pub id: mv88e6xxx_region_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_mst {
    pub node: list_head,
    pub refcnt: refcount_t,
    pub br: *mut net_device,
    pub msti: u16,
    pub stu: mv88e6xxx_stu_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_hw_stat {
    pub string: [c_char; ETH_GSTRING_LEN],
    pub size: usize,
    pub reg: c_int,
    pub type: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_tcam {
    pub entries: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_chip {
    pub info: *const mv88e6xxx_info,
// Currently configured tagging protocol
    pub tag_protocol: dsa_tag_protocol,
// The dsa_switch this private structure is related to
    pub ds: *mut dsa_switch,
// The device this structure is associated to
    pub dev: *mut device,
// This mutex protects the access to the switch registers
    pub reg_lock: mutex,
// The MII bus and the address on the bus that is used to
// communication with the switch
//
    pub smi_ops: *const mv88e6xxx_bus_ops,
    pub bus: *mut mii_bus,
    pub sw_addr: c_int,
// Handles automatic disabling and re-enabling of the PHY
// polling unit.
//
    pub phy_ops: *const mv88e6xxx_bus_ops,
    pub ppu_mutex: mutex,
    pub ppu_disabled: c_int,
    pub ppu_work: work_struct,
    pub ppu_timer: timer_list,
// This mutex serialises access to the statistics unit.
// Hold this mutex over snapshot + dump sequences.
//
    pub stats_mutex: mutex,
// A switch may have a GPIO line tied to its reset pin. Parse
// this from the device tree, and use it before performing
// switch soft reset.
//
    pub reset: *mut gpio_desc,
// set to size of eeprom if supported by the switch
    pub eeprom_len: u32,
// List of mdio busses
    pub mdios: list_head,
// Policy Control List IDs and rules
    pub policies: idr,
// There can be two interrupt controllers, which are chained
// off a GPIO as interrupt source
//
    pub g1_irq: mv88e6xxx_irq,
    pub g2_irq: mv88e6xxx_irq,
    pub irq: c_int,
    pub irq_name: [c_char; 64],
    pub device_irq: c_int,
    pub device_irq_name: [c_char; 64],
    pub watchdog_irq: c_int,
    pub watchdog_irq_name: [c_char; 64],
    pub atu_prob_irq: c_int,
    pub atu_prob_irq_name: [c_char; 64],
    pub vtu_prob_irq: c_int,
    pub vtu_prob_irq_name: [c_char; 64],
    pub kworker: *mut kthread_worker,
    pub irq_poll_work: kthread_delayed_work,
// GPIO resources
    pub gpio_data: [u8; 2],
// This cyclecounter abstracts the switch PTP time.
// reg_lock must be held for any operation that read()s.
//
    pub tstamp_cc: cyclecounter,
    pub tstamp_tc: timecounter,
    pub overflow_work: delayed_work,
    pub cc_coeffs: *const mv88e6xxx_cc_coeffs,
    pub ptp_clock: *mut ptp_clock,
    pub ptp_clock_info: ptp_clock_info,
    pub tai_event_work: delayed_work,
    pub pin_config: [ptp_pin_desc; MV88E6XXX_MAX_GPIO],
    pub enable_count: u16,
// Current ingress and egress monitor ports
    pub egress_dest_port: c_int,
    pub ingress_dest_port: c_int,
// Per-port timestamping resources.
    pub port_hwtstamp: [mv88e6xxx_port_hwtstamp; DSA_MAX_PORTS],
// Array of port structures.
    pub ports: [mv88e6xxx_port; DSA_MAX_PORTS],
// devlink regions
    pub regions: [*mut devlink_region; _MV88E6XXX_REGION_MAX],
// Bridge MST to SID mappings
    pub msts: list_head,
// FID map
    pub MV88E6XXX_N_FID): DECLARE_BITMAP(fid_bitmap,,
// TCAM entries
    pub tcam: mv88e6xxx_tcam,
// Global2 scratch register config data3
    pub g2_scratch_config3: u8,
}

pub const TCAM_MATCH_SIZE: c_int = 96;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_tcam_key {
    pub spv: u16,
    pub spv_mask: u16,
    pub frame_data: [u8; TCAM_MATCH_SIZE],
    pub frame_mask: [u8; TCAM_MATCH_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_tcam_action {
    pub dpv_mode: u8,
    pub dpv: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_tcam_entry {
    pub list: list_head,
    pub cookie: c_ulong,
    pub prio: u32,
    pub hw_idx: u8,
    pub key: mv88e6xxx_tcam_key,
    pub action: mv88e6xxx_tcam_action,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_bus_ops {
    pub val): *mut *mut *mut int (read)(struct mv88e6xxx_chip chip, int addr, int reg, u16,
    pub val): *mut *mut *mut int (write)(struct mv88e6xxx_chip chip, int addr, int reg, u16,
    pub chip): *mut *mut int (init)(struct mv88e6xxx_chip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_mdio_bus {
    pub bus: *mut mii_bus,
    pub chip: *mut mv88e6xxx_chip,
    pub list: list_head,
    pub external: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_ops {
// Switch Setup Errata, called early in the switch setup to
// allow any errata actions to be performed
//
    pub chip): *mut *mut int (setup_errata)(struct mv88e6xxx_chip,
    pub chip): *mut *mut int (ieee_pri_map)(struct mv88e6xxx_chip,
    pub chip): *mut *mut int (ip_pri_map)(struct mv88e6xxx_chip,
// Ingress Rate Limit unit (IRL) operations
    pub port): *mut *mut *mut int (irl_init_all)(struct mv88e6xxx_chip chip, int,
    pub data): *mut *mut ethtool_eeprom eeprom, u8,
    pub data): *mut *mut ethtool_eeprom eeprom, u8,
    pub addr): *mut *mut *mut int (set_switch_mac)(struct mv88e6xxx_chip chip, u8,
    pub val): *mut int addr, int reg, u16,
    pub val): int addr, int reg, u16,
    pub val): *mut int addr, int devad, int reg, u16,
    pub val): int addr, int devad, int reg, u16,
// Priority Override Table operations
    pub chip): *mut *mut int (pot_clear)(struct mv88e6xxx_chip,
// PHY Polling Unit (PPU) operations
    pub chip): *mut *mut int (ppu_enable)(struct mv88e6xxx_chip,
    pub chip): *mut *mut int (ppu_disable)(struct mv88e6xxx_chip,
// Additional handlers to run before and after hard reset, to make sure
// that the switch and EEPROM are in a good state.
//
    pub chip): *mut *mut int (hardware_reset_pre)(struct mv88e6xxx_chip,
    pub chip): *mut *mut int (hardware_reset_post)(struct mv88e6xxx_chip,
// Switch Software Reset
    pub chip): *mut *mut int (reset)(struct mv88e6xxx_chip,
// RGMII Receive/Transmit Timing Control
// Add delay on PHY_INTERFACE_MODE_RGMII_*ID, no delay otherwise.
//
    pub mode): phy_interface_t,
pub const LINK_FORCED_DOWN: c_int = 0;
pub const LINK_FORCED_UP: c_int = 1;

// Port's MAC link state
// Use LINK_FORCED_UP or LINK_FORCED_DOWN to force link up or down,
// or LINK_UNFORCED for normal link detection.
//
    pub link): *mut *mut *mut int (port_set_link)(struct mv88e6xxx_chip chip, int port, int,
// Synchronise the port link state with that of the SERDES
//
    pub isup): *mut *mut *mut int (port_sync_link)(struct mv88e6xxx_chip chip, int port, unsigned int mode, bool,
pub const PAUSE_ON: c_int = 1;
pub const PAUSE_OFF: c_int = 0;
// Enable/disable sending Pause
    pub pause): c_int,

// Port's MAC speed (in Mbps) and MAC duplex mode
//
// Depending on the chip, 10, 100, 200, 1000, 2500, 10000 are valid.
// Use SPEED_UNFORCED for normal detection.
//
// Use DUPLEX_HALF or DUPLEX_FULL to force half or full duplex,
// or DUPLEX_UNFORCED for normal duplex detection.
//
    pub duplex): int speed, int,
    pub port): *mut *mut *mut int (port_tag_remap)(struct mv88e6xxx_chip chip, int,
    pub action): mv88e6xxx_policy_action,
    pub mode): mv88e6xxx_frame_mode,
    pub unicast): bool,
    pub multicast): bool,
    pub etype): u16,
    pub size): usize,
    pub port): *mut *mut *mut int (port_egress_rate_limiting)(struct mv88e6xxx_chip chip, int,
    pub out): u8,
    pub port): *mut *mut *mut int (port_disable_learn_limit)(struct mv88e6xxx_chip chip, int,
    pub port): *mut *mut *mut int (port_disable_pri_override)(struct mv88e6xxx_chip chip, int,
    pub port): *mut *mut *mut int (port_setup_message_port)(struct mv88e6xxx_chip chip, int,
// CMODE control what PHY mode the MAC will use, eg. SGMII, RGMII, etc.
// Some chips allow this to be configured on specific ports.
//
    pub mode): phy_interface_t,
    pub cmode): *mut *mut *mut int (port_get_cmode)(struct mv88e6xxx_chip chip, int port, u8,
// LED control
    pub port): *mut *mut *mut int (port_setup_leds)(struct mv88e6xxx_chip chip, int,
// Some devices have a per port register indicating what is
// the upstream port this port should forward to.
//
    pub upstream_port): c_int,
// Snapshot the statistics for a port. The statistics can then
// be read back a leisure but still with a consistent view.
//
    pub port): *mut *mut *mut int (stats_snapshot)(struct mv88e6xxx_chip chip, int,
// Set the histogram mode for statistics, when the control registers
// are separated out of the STATS_OP register.
//
    pub chip): *mut *mut int (stats_set_histogram)(struct mv88e6xxx_chip,
// Return the number of strings describing statistics
    pub chip): *mut *mut int (stats_get_sset_count)(struct mv88e6xxx_chip,
    pub data): *mut *mut *mut void (stats_get_strings)(struct mv88e6xxx_chip chip, uint8_t,
    pub data): *mut u64,
    pub port): *mut *mut *mut int (set_cpu_port)(struct mv88e6xxx_chip chip, int,
    pub port): c_int,
pub const MV88E6XXX_CASCADE_PORT_NONE: c_uint = 0xe;
pub const MV88E6XXX_CASCADE_PORT_MULTIPLE: c_uint = 0xf;
    pub port): *mut *mut *mut int (set_cascade_port)(struct mv88e6xxx_chip chip, int,
    pub watchdog_ops: *const mv88e6xxx_irq_ops,
    pub chip): *mut *mut int (mgmt_rsvd2cpu)(struct mv88e6xxx_chip,
// SERDES lane mapping
    pub port): *mut *mut *mut int (serdes_get_lane)(struct mv88e6xxx_chip chip, int,
// SERDES interrupt handling
    pub port): c_int,
// Statistics from the SERDES interface
    pub port): *mut *mut *mut int (serdes_get_sset_count)(struct mv88e6xxx_chip chip, int,
    pub data): *mut u8,
    pub data): *mut u64,
// SERDES registers for ethtool
    pub port): *mut *mut *mut int (serdes_get_regs_len)(struct mv88e6xxx_chip chip, int,
    pub _p): *mut c_void,
// Address Translation Unit operations
    pub hash): *mut *mut *mut int (atu_get_hash)(struct mv88e6xxx_chip chip, u8,
    pub hash): *mut *mut *mut int (atu_set_hash)(struct mv88e6xxx_chip chip, u8,
// VLAN Translation Unit operations
    pub entry): *mut mv88e6xxx_vtu_entry,
    pub entry): *mut mv88e6xxx_vtu_entry,
// Spanning Tree Unit operations
    pub entry): *mut mv88e6xxx_stu_entry,
    pub entry): *mut mv88e6xxx_stu_entry,
// GPIO operations
    pub gpio_ops: *const mv88e6xxx_gpio_ops,
// Interface to the AVB/PTP registers
    pub avb_ops: *const mv88e6xxx_avb_ops,
// Remote Management Unit operations
    pub chip): *mut *mut int (rmu_disable)(struct mv88e6xxx_chip,
// Precision Time Protocol operations
    pub ptp_ops: *const mv88e6xxx_ptp_ops,
// Phylink
    pub config): *mut phylink_config,
    pub pcs_ops: *const mv88e6xxx_pcs_ops,
// Max Frame Size
    pub mtu): *mut *mut *mut int (set_max_frame_size)(struct mv88e6xxx_chip chip, int,
    pub port): *mut *mut *mut int (port_enable_tcam)(struct mv88e6xxx_chip chip, int,
// Ternary Content Addressable Memory operations
    pub tcam_ops: *const mv88e6xxx_tcam_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_irq_ops {
// Action to be performed when the interrupt happens
    pub irq): *mut *mut *mut int (irq_action)(struct mv88e6xxx_chip chip, int,
// Setup the hardware to generate the interrupt
    pub chip): *mut *mut int (irq_setup)(struct mv88e6xxx_chip,
// Reset the hardware to stop generating the interrupt
    pub chip): *mut *mut void (irq_free)(struct mv88e6xxx_chip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_gpio_ops {
// Get/set data on GPIO pin
    pub pin): *mut *mut *mut int (get_data)(struct mv88e6xxx_chip chip, unsigned int,
    pub value): c_int,
// get/set GPIO direction
    pub pin): *mut *mut *mut int (get_dir)(struct mv88e6xxx_chip chip, unsigned int,
    pub input): bool,
// get/set GPIO pin control
    pub func): *mut c_int,
    pub func): c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_avb_ops {
// Access port-scoped Precision Time Protocol registers
    pub len): *mut *mut u16 data, int,
    pub data): u16,
// Access global Precision Time Protocol registers
    pub len): c_int,
    pub data): *mut *mut *mut int (ptp_write)(struct mv88e6xxx_chip chip, int addr, u16,
// Access global Time Application Interface registers
    pub len): c_int,
    pub data): *mut *mut *mut int (tai_write)(struct mv88e6xxx_chip chip, int addr, u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_ptp_ops {
    pub cc): *mut *mut u64 (clock_read)(struct cyclecounter,
    pub on): *mut *mut ptp_clock_request rq, int,
    pub chan): ptp_pin_function func, unsigned int,
    pub ugly): *mut *mut void (event_work)(struct work_struct,
    pub port): *mut *mut *mut int (port_enable)(struct mv88e6xxx_chip chip, int,
    pub port): *mut *mut *mut int (port_disable)(struct mv88e6xxx_chip chip, int,
    pub chip): *mut *mut int (global_enable)(struct mv88e6xxx_chip,
    pub chip): *mut *mut int (global_disable)(struct mv88e6xxx_chip,
    pub port): *mut *mut *mut int (set_ptp_cpu_port)(struct mv88e6xxx_chip chip, int,
    pub n_ext_ts: c_int,
    pub arr0_sts_reg: c_int,
    pub arr1_sts_reg: c_int,
    pub dep_sts_reg: c_int,
    pub rx_filters: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_pcs_ops {
    pub port): *mut *mut *mut int (pcs_init)(struct mv88e6xxx_chip chip, int,
    pub port): *mut *mut *mut void (pcs_teardown)(struct mv88e6xxx_chip chip, int,
    pub mode): phy_interface_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6xxx_tcam_ops {
    pub idx): *mut *mut mv88e6xxx_tcam_entry entry, u8,
    pub chip): *mut *mut int (flush_tcam)(struct mv88e6xxx_chip,
}

extern "C" {
    pub fn GENMASK(1: (s32)mv88e6xxx_num_ports(chip) -, _arg: 0) -> return;
}
extern "C" {
    pub fn mv88e6xxx_read(chip: *mut mv88e6xxx_chip, addr: c_int, reg: c_int, val: *mut u16) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_write(chip: *mut mv88e6xxx_chip, addr: c_int, reg: c_int, val: u16) -> c_int;
}
