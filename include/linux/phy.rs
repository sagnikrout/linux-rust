//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/phy.h
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
// Framework and drivers for configuring and reading different PHYs
// Based on code in sungem_phy.c and (long-removed) gianfar_phy.c
//
// Author: Andy Fleming
//
// Copyright (c) 2004 Freescale Semiconductor, Inc.
//

//
// Set phydev->irq to PHY_POLL if interrupts are not supported,
// or not desired for this PHY.  Set to PHY_MAC_INTERRUPT if
// the attached MAC driver handles the interrupt
//

pub const PHY_IS_INTERNAL: c_uint = 0x00000001;
pub const PHY_RST_AFTER_CLK_EN: c_uint = 0x00000002;
pub const PHY_POLL_CABLE_TEST: c_uint = 0x00000004;
pub const PHY_ALWAYS_CALL_SUSPEND: c_uint = 0x00000008;
pub const MDIO_DEVICE_IS_PHY: c_uint = 0x80000000;
//
// enum phy_interface_t - Interface Mode definitions
//
// @PHY_INTERFACE_MODE_NA: Not Applicable - don't touch
// @PHY_INTERFACE_MODE_INTERNAL: No interface, MAC and PHY combined
// @PHY_INTERFACE_MODE_MII: Media-independent interface
// @PHY_INTERFACE_MODE_GMII: Gigabit media-independent interface
// @PHY_INTERFACE_MODE_SGMII: Serial gigabit media-independent interface
// @PHY_INTERFACE_MODE_TBI: Ten Bit Interface
// @PHY_INTERFACE_MODE_REVMII: Reverse Media Independent Interface
// @PHY_INTERFACE_MODE_RMII: Reduced Media Independent Interface
// @PHY_INTERFACE_MODE_REVRMII: Reduced Media Independent Interface in PHY role
// @PHY_INTERFACE_MODE_RGMII: Reduced gigabit media-independent interface
// @PHY_INTERFACE_MODE_RGMII_ID: RGMII with Internal RX+TX delay
// @PHY_INTERFACE_MODE_RGMII_RXID: RGMII with Internal RX delay
// @PHY_INTERFACE_MODE_RGMII_TXID: RGMII with Internal TX delay
// @PHY_INTERFACE_MODE_RTBI: Reduced TBI
// @PHY_INTERFACE_MODE_SMII: Serial MII
// @PHY_INTERFACE_MODE_XGMII: 10 gigabit media-independent interface
// @PHY_INTERFACE_MODE_XLGMII:40 gigabit media-independent interface
// @PHY_INTERFACE_MODE_MOCA: Multimedia over Coax
// @PHY_INTERFACE_MODE_PSGMII: Penta SGMII
// @PHY_INTERFACE_MODE_QSGMII: Quad SGMII
// @PHY_INTERFACE_MODE_TRGMII: Turbo RGMII
// @PHY_INTERFACE_MODE_100BASEX: 100 BaseX
// @PHY_INTERFACE_MODE_1000BASEX: 1000 BaseX
// @PHY_INTERFACE_MODE_2500BASEX: 2500 BaseX
// @PHY_INTERFACE_MODE_5GBASER: 5G BaseR
// @PHY_INTERFACE_MODE_RXAUI: Reduced XAUI
// @PHY_INTERFACE_MODE_XAUI: 10 Gigabit Attachment Unit Interface
// @PHY_INTERFACE_MODE_10GBASER: 10G BaseR
// @PHY_INTERFACE_MODE_25GBASER: 25G BaseR
// @PHY_INTERFACE_MODE_USXGMII:  Universal Serial 10GE MII
// @PHY_INTERFACE_MODE_10GKR: 10GBASE-KR - with Clause 73 AN
// @PHY_INTERFACE_MODE_QUSGMII: Quad Universal SGMII
// @PHY_INTERFACE_MODE_1000BASEKX: 1000Base-KX - with Clause 73 AN
// @PHY_INTERFACE_MODE_10G_QXGMII: 10G-QXGMII - 4 ports over 10G USXGMII
// @PHY_INTERFACE_MODE_50GBASER: 50GBase-R - with Clause 134 FEC
// @PHY_INTERFACE_MODE_LAUI: 50 Gigabit Attachment Unit Interface
// @PHY_INTERFACE_MODE_100GBASEP: 100GBase-P - with Clause 134 FEC
// @PHY_INTERFACE_MODE_MIILITE: MII-Lite - MII without RXER TXER CRS COL
// @PHY_INTERFACE_MODE_MAX: Book keeping
//
// Describes the interface between the MAC and PHY.
//
// 10GBASE-R, XFI, SFI - single lane 10G Serdes
// 10GBASE-KR - with Clause 73 AN
// PHY interface mode bitmap handling

extern "C" {
    pub fn bitmap_empty(_arg: intf, _arg: PHY_INTERFACE_MODE_MAX) -> return;
}
extern "C" {
    pub fn bitmap_weight(_arg: intf, _arg: PHY_INTERFACE_MODE_MAX) -> return;
}
//
// phy_modes - map phy_interface_t enum to device tree binding of phy-mode
// @interface: enum phy_interface_t value
//
// Description: maps enum &phy_interface_t defined in this file
// into the device tree binding of 'phy-mode', so that Ethernet
// device driver can get PHY interface from device tree.
//
// rgmii_clock - map link speed to the clock rate
// @speed: link speed value
//
// Description: maps RGMII supported link speeds into the clock rates.
// This can also be used for MII, GMII, and RMII interface modes as the
// clock rates are identical, but the caller must be aware that errors
// for unsupported clock rates will not be signalled.
//
// Returns: clock rate or negative errno
//
pub const PHY_MAX_ADDR: c_int = 32;
// Used when trying to connect to a specific phy (mii bus id:phy device id)

pub const MII_BUS_ID_SIZE: c_int = 61;
//
// struct mdio_bus_stats - Statistics counters for MDIO busses
// @transfers: Total number of transfers, i.e. @writes + @reads
// @errors: Number of MDIO transfers that returned an error
// @writes: Number of write transfers
// @reads: Number of read transfers
// @syncp: Synchronisation for incrementing statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdio_bus_stats {
    pub transfers: u64_stats_t,
    pub errors: u64_stats_t,
    pub writes: u64_stats_t,
    pub reads: u64_stats_t,
// Must be last, add new statistics above
    pub syncp: u64_stats_sync,
}

//
// struct mii_bus - Represents an MDIO bus
//
// @owner: Who owns this device
// @name: User friendly name for this MDIO device, or driver name
// @id: Unique identifier for this bus, typical from bus hierarchy
// @priv: Driver private data
//
// The Bus class for PHYs.  Devices which provide access to
// PHYs should register using this structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mii_bus {
    pub owner: *mut module,
    pub name: *const c_char,
    pub id: [c_char; MII_BUS_ID_SIZE],
    pub priv: *mut c_void,
// @read: Perform a read transfer on the bus
    pub regnum): *mut *mut *mut int (read)(struct mii_bus bus, int addr, int,
// @write: Perform a write transfer on the bus
    pub val): *mut *mut *mut int (write)(struct mii_bus bus, int addr, int regnum, u16,
// @read_c45: Perform a C45 read transfer on the bus
    pub regnum): *mut *mut *mut int (read_c45)(struct mii_bus bus, int addr, int devnum, int,
// @write_c45: Perform a C45 write transfer on the bus
    pub val): int regnum, u16,
// @reset: Perform a reset of the bus
    pub bus): *mut *mut int (reset)(struct mii_bus,
// @stats: Statistic counters per device on the bus
    pub stats: [mdio_bus_stats; PHY_MAX_ADDR],
//
// @mdio_lock: A lock to ensure that only one thing can read/write
// the MDIO bus at a time
//
    pub mdio_lock: mutex,
// @parent: Parent device of this bus
    pub parent: *mut device,
// @state: State of bus structure
    pub state: },
// @dev: Kernel device representation
    pub dev: device,
// @mdio_map: list of all MDIO devices on bus
    pub mdio_map: [*mut mdio_device; PHY_MAX_ADDR],
// @phy_mask: PHY addresses to be ignored when probing
    pub phy_mask: u32,
// @phy_ignore_ta_mask: PHY addresses to ignore the TA/read failure
    pub phy_ignore_ta_mask: u32,
//
// @irq: An array of interrupts, each PHY's interrupt at the index
// matching its address
//
    pub irq: [c_int; PHY_MAX_ADDR],
// @reset_delay_us: GPIO reset pulse width in microseconds
    pub reset_delay_us: c_int,
// @reset_post_delay_us: GPIO reset deassert delay in microseconds
    pub reset_post_delay_us: c_int,
// @reset_gpiod: Reset GPIO descriptor pointer
    pub reset_gpiod: *mut gpio_desc,
// @shared_lock: protect access to the shared element
    pub shared_lock: mutex,

// @shared: shared state across different PHYs
    pub shared: [*mut phy_package_shared; PHY_MAX_ADDR],
}

//
// mdiobus_alloc - Allocate an MDIO bus structure
//
// The internal state of the MDIO bus will be set of MDIOBUS_ALLOCATED ready
// for the driver to register the bus.
//
extern "C" {
    pub fn mdiobus_alloc_size(_arg: 0) -> return;
}
extern "C" {
    pub fn __mdiobus_register(bus: *mut mii_bus, owner: *mut module) -> c_int;
}

extern "C" {
    pub fn mdiobus_unregister(bus: *mut mii_bus);
}
extern "C" {
    pub fn mdiobus_free(bus: *mut mii_bus);
}
extern "C" {
    pub fn devm_mdiobus_alloc_size(_arg: dev, _arg: 0) -> return;
}

//
// enum phy_state - PHY state machine states:
//
// @PHY_DOWN: PHY device and driver are not ready for anything.  probe
// should be called if and only if the PHY is in this state,
// given that the PHY device exists.
// - PHY driver probe function will set the state to @PHY_READY
//
// @PHY_READY: PHY is ready to send and receive packets, but the
// controller is not.  By default, PHYs which do not implement
// probe will be set to this state by phy_probe().
// - start will set the state to UP
//
// @PHY_UP: The PHY and attached device are ready to do work.
// Interrupts should be started here.
// - timer moves to @PHY_NOLINK or @PHY_RUNNING
//
// @PHY_NOLINK: PHY is up, but not currently plugged in.
// - irq or timer will set @PHY_RUNNING if link comes back
// - phy_stop moves to @PHY_HALTED
//
// @PHY_RUNNING: PHY is currently up, running, and possibly sending
// and/or receiving packets
// - irq or timer will set @PHY_NOLINK if link goes down
// - phy_stop moves to @PHY_HALTED
//
// @PHY_CABLETEST: PHY is performing a cable test. Packet reception/sending
// is not expected to work, carrier will be indicated as down. PHY will be
// poll once per second, or on interrupt for it current state.
// Once complete, move to UP to restart the PHY.
// - phy_stop aborts the running test and moves to @PHY_HALTED
//
// @PHY_HALTED: PHY is up, but no polling or interrupts are done.
// - phy_start moves to @PHY_UP
//
// @PHY_ERROR: PHY is up, but is in an error state.
// - phy_stop moves to @PHY_HALTED
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_state {
    PHY_DOWN = 0,
    PHY_READY,
    PHY_HALTED,
    PHY_ERROR,
    PHY_UP,
    PHY_RUNNING,
    PHY_NOLINK,
    PHY_CABLETEST,
}

pub const MDIO_MMD_NUM: c_int = 32;
//
// struct phy_c45_device_ids - 802.3-c45 Device Identifiers
// @devices_in_package: IEEE 802.3 devices in package register value.
// @mmds_present: bit vector of MMDs present.
// @device_ids: The device identifier for each present device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_c45_device_ids {
    pub devices_in_package: u32,
    pub mmds_present: u32,
    pub device_ids: [u32; MDIO_MMD_NUM],
}

//
// struct phy_oatc14_sqi_capability - SQI capability information for OATC14
// 10Base-T1S PHY
// @updated: Indicates whether the SQI capability fields have been updated.
// @sqi_max: Maximum supported Signal Quality Indicator (SQI) level reported by
// the PHY.
// @sqiplus_bits: Bits for SQI+ levels supported by the PHY.
// 0 - SQI+ is not supported
// 3 - SQI+ is supported, using 3 bits (8 levels)
// 4 - SQI+ is supported, using 4 bits (16 levels)
// 5 - SQI+ is supported, using 5 bits (32 levels)
// 6 - SQI+ is supported, using 6 bits (64 levels)
// 7 - SQI+ is supported, using 7 bits (128 levels)
// 8 - SQI+ is supported, using 8 bits (256 levels)
//
// This structure is used by the OATC14 10Base-T1S PHY driver to store the SQI
// and SQI+ capability information retrieved from the PHY.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_oatc14_sqi_capability {
    pub updated: bool,
    pub sqi_max: c_int,
    pub sqiplus_bits: u8,
}

//
// struct phy_device - An instance of a PHY
//
// @mdio: MDIO bus this PHY is on
// @drv: Pointer to the driver for this PHY instance
// @devlink: Create a link between phy dev and mac dev, if the external phy
// used by current mac interface is managed by another mac interface.
// @phyindex: Unique id across the phy's parent tree of phys to address the PHY
// from userspace, similar to ifindex. A zero index means the PHY
// wasn't assigned an id yet.
// @phy_id: UID for this device found during discovery
// @c45_ids: 802.3-c45 Device Identifiers if is_c45.
// @is_c45:  Set to true if this PHY uses clause 45 addressing.
// @is_internal: Set to true if this PHY is internal to a MAC.
// @is_pseudo_fixed_link: Set to true if this PHY is an Ethernet switch, etc.
// @is_gigabit_capable: Set to true if PHY supports 1000Mbps
// @has_fixups: Set to true if this PHY has fixups/quirks.
// @suspended: Set to true if this PHY has been suspended successfully.
// @suspended_by_mdio_bus: Set to true if this PHY was suspended by MDIO bus.
// @sysfs_links: Internal boolean tracking sysfs symbolic links setup/removal.
// @loopback_enabled: Set true if this PHY has been loopbacked successfully.
// @downshifted_rate: Set true if link speed has been downshifted.
// @is_on_sfp_module: Set true if PHY is located on an SFP module.
// @mac_managed_pm: Set true if MAC driver takes of suspending/resuming PHY
// @wol_enabled: Set to true if the PHY or the attached MAC have Wake-on-LAN
// enabled.
// @is_genphy_driven: PHY is driven by one of the generic PHY drivers
// @state: State of the PHY for management purposes
// @dev_flags: Device-specific flags used by the PHY driver.
//
// - Bits [15:0] are free to use by the PHY driver to communicate
// driver specific behavior.
// - Bits [23:16] are currently reserved for future use.
// - Bits [31:24] are reserved for defining generic
// PHY driver behavior.
// @irq: IRQ number of the PHY's interrupt (-1 if none)
// @phylink: Pointer to phylink instance for this PHY
// @sfp_bus_attached: Flag indicating whether the SFP bus has been attached
// @sfp_bus: SFP bus attached to this PHY's fiber port
// @attached_dev: The attached enet driver's device instance ptr
// @adjust_link: Callback for the enet controller to respond to changes: in the
// link state.
// @phy_link_change: Callback for phylink for notification of link change
// @macsec_ops: MACsec offloading ops.
//
// @speed: Current link speed
// @duplex: Current duplex
// @port: Current port
// @pause: Current pause
// @asym_pause: Current asymmetric pause
// @supported: Combined MAC/PHY supported linkmodes
// @advertising: Currently advertised linkmodes
// @adv_old: Saved advertised while power saving for WoL
// @supported_eee: supported PHY EEE linkmodes
// @advertising_eee: Currently advertised EEE linkmodes
// @enable_tx_lpi: When True, MAC should transmit LPI to PHY
// @eee_active: phylib private state, indicating that EEE has been negotiated
// @autonomous_eee_disabled: Set when autonomous EEE has been disabled,
// used to re-apply after PHY soft reset
// @eee_cfg: User configuration of EEE
// @lp_advertising: Current link partner advertised linkmodes
// @host_interfaces: PHY interface modes supported by host
// @eee_disabled_modes: Energy efficient ethernet modes not to be advertised
// @autoneg: Flag autoneg being used
// @rate_matching: Current rate matching mode
// @link: Current link state
// @autoneg_complete: Flag auto negotiation of the link has completed
// @mdix: Current crossover
// @mdix_ctrl: User setting of crossover
// @pma_extable: Cached value of PMA/PMD Extended Abilities Register
// @interrupts: Flag interrupts have been enabled
// @irq_suspended: Flag indicating PHY is suspended and therefore interrupt
// handling shall be postponed until PHY has resumed
// @irq_rerun: Flag indicating interrupts occurred while PHY was suspended,
// requiring a rerun of the interrupt handler after resume
// @default_timestamp: Flag indicating whether we are using the phy
// timestamp as the default one
// @interface: enum phy_interface_t value
// @possible_interfaces: bitmap if interface modes that the attached PHY
// will switch between depending on media speed.
// @skb: Netlink message for cable diagnostics
// @nest: Netlink nest used for cable diagnostics
// @ehdr: nNtlink header for cable diagnostics
// @phy_led_triggers: Array of LED triggers
// @phy_num_led_triggers: Number of triggers in @phy_led_triggers
// @led_link_trigger: LED trigger for link up/down
// @last_triggered: last LED trigger for link speed
// @leds: list of PHY LED structures
// @master_slave_set: User requested master/slave configuration
// @master_slave_get: Current master/slave advertisement
// @master_slave_state: Current master/slave configuration
// @mii_ts: Pointer to time stamper callbacks
// @psec: Pointer to Power Sourcing Equipment control struct
// @ports: List of PHY ports structures
// @n_ports: Number of ports currently attached to the PHY
// @max_n_ports: Max number of ports this PHY can expose
// @lock:  Mutex for serialization access to PHY
// @state_queue: Work queue for state machine
// @link_down_events: Number of times link was lost
// @shared: Pointer to private data shared by phys in one package
// @priv: Pointer to driver private data
// @oatc14_sqi_capability: SQI capability information for OATC14 10Base-T1S PHY
//
// interrupts currently only supports enabled or disabled,
// but could be changed in the future to support enabling
// and disabling specific interrupts
//
// Contains some infrastructure for polling and interrupt
// handling, as well as handling shifts in PHY hardware state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_device {
    pub mdio: mdio_device,
// Information about the PHY type
// And management functions
    pub drv: *const phy_driver,
    pub devlink: *mut device_link,
    pub phyindex: u32,
    pub phy_id: u32,
    pub c45_ids: phy_c45_device_ids,
    pub is_c45:1: unsigned,
    pub is_internal:1: unsigned,
    pub is_pseudo_fixed_link:1: unsigned,
    pub is_gigabit_capable:1: unsigned,
    pub has_fixups:1: unsigned,
    pub suspended:1: unsigned,
    pub suspended_by_mdio_bus:1: unsigned,
    pub sysfs_links:1: unsigned,
    pub loopback_enabled:1: unsigned,
    pub downshifted_rate:1: unsigned,
    pub is_on_sfp_module:1: unsigned,
    pub mac_managed_pm:1: unsigned,
    pub wol_enabled:1: unsigned,
    pub is_genphy_driven:1: unsigned,
    pub autoneg:1: unsigned,
// The most recently read link state
    pub link:1: unsigned,
    pub autoneg_complete:1: unsigned,
    pub pause:1: bool,
    pub asym_pause:1: bool,
// Interrupts are enabled
    pub interrupts:1: unsigned,
    pub irq_suspended:1: unsigned,
    pub irq_rerun:1: unsigned,
    pub default_timestamp:1: unsigned,
    pub rate_matching: c_int,
    pub state: phy_state,
    pub dev_flags: u32,
    pub interface: phy_interface_t,
//
// forced speed & duplex (no autoneg)
// partner speed & duplex & pause (autoneg)
//
    pub speed: c_int,
    pub duplex: c_int,
    pub port: c_int,
    pub master_slave_get: u8,
    pub master_slave_set: u8,
    pub master_slave_state: u8,
// Union of PHY and Attached devices' supported link modes
// See ethtool.h for more info
// used with phy_speed_down
// used for eee validation and configuration
// Energy efficient ethernet modes which should be prohibited
    pub enable_tx_lpi: bool,
    pub eee_active: bool,
    pub autonomous_eee_disabled: bool,
    pub eee_cfg: eee_config,
// Host supported PHY interface types. Should be ignored if empty.

    pub phy_led_triggers: *mut phy_led_trigger,
    pub phy_num_led_triggers: c_uint,
    pub last_triggered: *mut phy_led_trigger,
    pub led_link_trigger: *mut phy_led_trigger,

    pub leds: list_head,
//
// Interrupt number for this PHY
// -1 means no interrupt
//
    pub irq: c_int,
// private data pointer
// For use by PHYs to maintain extra state
    pub priv: *mut c_void,

// shared data pointer
// For use by PHYs inside the same package that need a shared state.
    pub shared: *mut phy_package_shared,

// Reporting cable test results
    pub skb: *mut sk_buff,
    pub ehdr: *mut c_void,
    pub nest: *mut nlattr,
// Interrupt and Polling infrastructure
    pub state_queue: delayed_work,
    pub lock: mutex,
// This may be modified under the rtnl lock
    pub sfp_bus_attached: bool,
    pub sfp_bus: *mut sfp_bus,
    pub phylink: *mut phylink,
    pub attached_dev: *mut net_device,
    pub mii_ts: *mut mii_timestamper,
    pub psec: *mut pse_control,
    pub ports: list_head,
    pub n_ports: c_int,
    pub max_n_ports: c_int,
    pub mdix: u8,
    pub mdix_ctrl: u8,
    pub pma_extable: c_int,
    pub link_down_events: c_uint,
    pub up): *mut *mut *mut void (phy_link_change)(struct phy_device phydev, bool,
    pub dev): *mut *mut void (adjust_link)(struct net_device,

// MACsec management functions
    pub macsec_ops: *const macsec_ops,

    pub oatc14_sqi_capability: phy_oatc14_sqi_capability,
}

// Generic phy_device::dev_flags
pub const PHY_F_NO_IRQ: c_uint = 0x80000000;
pub const PHY_F_RXC_ALWAYS_ON: c_uint = 0x40000000;
pub const PHY_F_KEEP_PREAMBLE_BEFORE_SFD: c_uint = 0x20000000;

//
// struct phy_tdr_config - Configuration of a TDR raw test
//
// @first: Distance for first data collection point
// @last: Distance for last data collection point
// @step: Step between data collection points
// @pair: Bitmap of cable pairs to collect data for
//
// A structure containing possible configuration parameters
// for a TDR cable test. The driver does not need to implement
// all the parameters, but should report what is actually used.
// All distances are in centimeters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_tdr_config {
    pub first: u32,
    pub last: u32,
    pub step: u32,
    pub pair: i8,
}

//
// enum link_inband_signalling - in-band signalling modes that are supported
//
// @LINK_INBAND_DISABLE: in-band signalling can be disabled
// @LINK_INBAND_ENABLE: in-band signalling can be enabled without bypass
// @LINK_INBAND_BYPASS: in-band signalling can be enabled with bypass
//
// The possible and required bits can only be used if the valid bit is set.
// If possible is clear, that means inband signalling can not be used.
// Required is only valid when possible is set, and means that inband
// signalling must be used.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum link_inband_signalling {
    LINK_INBAND_DISABLE		= BIT(0),
    LINK_INBAND_ENABLE		= BIT(1),
    LINK_INBAND_BYPASS		= BIT(2),
}

//
// struct phy_plca_cfg - Configuration of the PLCA (Physical Layer Collision
// Avoidance) Reconciliation Sublayer.
//
// @version: read-only PLCA register map version. -1 = not available. Ignored
// when setting the configuration. Format is the same as reported by the PLCA
// IDVER register (31.CA00). -1 = not available.
// @enabled: PLCA configured mode (enabled/disabled). -1 = not available / don't
// set. 0 = disabled, anything else = enabled.
// @node_id: the PLCA local node identifier. -1 = not available / don't set.
// Allowed values [0 .. 254]. 255 = node disabled.
// @node_cnt: the PLCA node count (maximum number of nodes having a TO). Only
// meaningful for the coordinator (node_id = 0). -1 = not available / don't
// set. Allowed values [1 .. 255].
// @to_tmr: The value of the PLCA to_timer in bit-times, which determines the
// PLCA transmit opportunity window opening. See IEEE802.3 Clause 148 for
// more details. The to_timer shall be set equal over all nodes.
// -1 = not available / don't set. Allowed values [0 .. 255].
// @burst_cnt: controls how many additional frames a node is allowed to send in
// single transmit opportunity (TO). The default value of 0 means that the
// node is allowed exactly one frame per TO. A value of 1 allows two frames
// per TO, and so on. -1 = not available / don't set.
// Allowed values [0 .. 255].
// @burst_tmr: controls how many bit times to wait for the MAC to send a new
// frame before interrupting the burst. This value should be set to a value
// greater than the MAC inter-packet gap (which is typically 96 bits).
// -1 = not available / don't set. Allowed values [0 .. 255].
//
// A structure containing configuration parameters for setting/getting the PLCA
// RS configuration. The driver does not need to implement all the parameters,
// but should report what is actually used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_plca_cfg {
    pub version: c_int,
    pub enabled: c_int,
    pub node_id: c_int,
    pub node_cnt: c_int,
    pub to_tmr: c_int,
    pub burst_cnt: c_int,
    pub burst_tmr: c_int,
}

//
// struct phy_plca_status - Status of the PLCA (Physical Layer Collision
// Avoidance) Reconciliation Sublayer.
//
// @pst: The PLCA status as reported by the PST bit in the PLCA STATUS
// register(31.CA03), indicating BEACON activity.
//
// A structure containing status information of the PLCA RS configuration.
// The driver does not need to implement all the parameters, but should report
// what is actually used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_plca_status {
    pub pst: bool,
}

// Modes for PHY LED configuration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_led_modes {
    PHY_LED_ACTIVE_HIGH = 0,
    PHY_LED_ACTIVE_LOW = 1,
    PHY_LED_INACTIVE_HIGH_IMPEDANCE = 2,

// keep it last
    __PHY_LED_MODES_NUM,
}

//
// struct phy_led: An LED driven by the PHY
//
// @list: List of LEDs
// @phydev: PHY this LED is attached to
// @led_cdev: Standard LED class structure
// @index: Number of the LED
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_led {
    pub list: list_head,
    pub phydev: *mut phy_device,
    pub led_cdev: led_classdev,
    pub index: u8,
}

//
// PHY_MSE_CAP_* - Bitmask flags for Mean Square Error (MSE) capabilities
//
// These flags describe which MSE metrics and selectors are implemented
// by the PHY for the current link mode. They are used in
// struct phy_mse_capability.supported_caps.
//
// Standardization:
// The OPEN Alliance (OA) defines the presence of MSE/SQI/pMSE but not their
// numeric scaling, update intervals, or aggregation windows.  See:
// OA 100BASE-T1 TC1 v1.0, sections 6.1.1-6.1.3
// OA 1000BASE-T1 TC12 v2.2, sections 6.1.1-6.1.2
//
// Description of flags:
//
// PHY_MSE_CAP_CHANNEL_A
// Per-pair diagnostics for Channel A are supported.  Mapping to the
// physical wire pair may depend on MDI/MDI-X polarity.
//
// PHY_MSE_CAP_CHANNEL_B, _C, _D
// Same as above for channels B-D.
//
// PHY_MSE_CAP_WORST_CHANNEL
// The PHY or driver can identify and report the single worst-performing
// channel without querying each one individually.
//
// PHY_MSE_CAP_LINK
// The PHY provides only a link-wide aggregate measurement or cannot map
// results to a specific pair (for example 100BASE-TX with unknown
// MDI/MDI-X).
//
// PHY_MSE_CAP_AVG
// Average MSE (mean DCQ metric) is supported.  For 100/1000BASE-T1 the OA
// recommends 2^16 symbols, scaled 0..511, but the exact scaling is
// vendor-specific.
//
// PHY_MSE_CAP_PEAK
// Peak MSE (current peak within the measurement window) is supported.
// Defined as pMSE for 100BASE-T1; vendor-specific for others.
//
// PHY_MSE_CAP_WORST_PEAK
// Latched worst-case peak MSE since the last read (read-to-clear if
// implemented).  Optional in OA 100BASE-T1 TC1 6.1.3.
//

//
// enum phy_mse_channel - Identifiers for selecting MSE measurement channels
//
// PHY_MSE_CHANNEL_A - PHY_MSE_CHANNEL_D
// Select per-pair measurement for the corresponding channel.
//
// PHY_MSE_CHANNEL_WORST
// Select the single worst-performing channel reported by hardware.
//
// PHY_MSE_CHANNEL_LINK
// Select link-wide aggregate data (used when per-pair results are
// unavailable).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_mse_channel {
    PHY_MSE_CHANNEL_A,
    PHY_MSE_CHANNEL_B,
    PHY_MSE_CHANNEL_C,
    PHY_MSE_CHANNEL_D,
    PHY_MSE_CHANNEL_WORST,
    PHY_MSE_CHANNEL_LINK,
}

//
// struct phy_mse_capability - Capabilities of Mean Square Error (MSE)
// measurement interface
//
// Standardization notes:
//
// - Presence of MSE/SQI/pMSE is defined by OPEN Alliance specs, but numeric
// scaling, refresh/update rate and aggregation windows are not fixed and
// are vendor-/product-specific. (OA 100BASE-T1 TC1 v1.0 6.1.*;
// OA 1000BASE-T1 TC12 v2.2 6.1.*)
//
// - Typical recommendations: 2^16 symbols and 0..511 scaling for MSE; pMSE only
// defined for 100BASE-T1 (sliding window example), others are vendor
// extensions. Drivers must report actual scale/limits here.
//
// Describes the MSE measurement capabilities for the current link mode. These
// properties are dynamic and may change when link settings are modified.
// Callers should re-query this capability after any link state change to
// ensure they have the most up-to-date information.
//
// Callers should only request measurements for channels and types that are
// indicated as supported by the @supported_caps bitmask. If @supported_caps
// is 0, the device provides no MSE diagnostics, and driver operations should
// typically return -EOPNOTSUPP.
//
// Snapshot values for average and peak MSE can be normalized to a 0..1 ratio
// by dividing the raw snapshot by the corresponding @max_average_mse or
// @max_peak_mse value.
//
// @max_average_mse: The maximum value for an average MSE snapshot. This
// defines the scale for the measurement. If the PHY_MSE_CAP_AVG capability is
// supported, this value MUST be greater than 0. (vendor-specific units).
// @max_peak_mse: The maximum value for a peak MSE snapshot. If either
// PHY_MSE_CAP_PEAK or PHY_MSE_CAP_WORST_PEAK is supported, this value MUST
// be greater than 0. (vendor-specific units).
// @refresh_rate_ps: The typical interval, in picoseconds, between hardware
// updates of the MSE values. This is an estimate, and callers should not
// assume synchronous sampling. (vendor-specific units).
// @num_symbols: The number of symbols aggregated per hardware sample to
// calculate the MSE. (vendor-specific units).
// @supported_caps: A bitmask of PHY_MSE_CAP_* values indicating which
// measurement types (e.g., average, peak) and channels
// (e.g., per-pair or link-wide) are supported.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_mse_capability {
    pub max_average_mse: u64,
    pub max_peak_mse: u64,
    pub refresh_rate_ps: u64,
    pub num_symbols: u64,
    pub supported_caps: u32,
}

//
// struct phy_mse_snapshot - A snapshot of Mean Square Error (MSE) diagnostics
//
// Holds a set of MSE diagnostic values that were all captured from a single
// measurement window.
//
// Values are raw, device-scaled and not normalized. Use struct
// phy_mse_capability to interpret the scale and sampling window.
//
// @average_mse: The average MSE value over the measurement window.
// OPEN Alliance references MSE as a DCQ metric; recommends 2^16 symbols and
// 0..511 scaling. Exact scale and refresh are vendor-specific.
// (100BASE-T1 TC1 v1.0 6.1.1; 1000BASE-T1 TC12 v2.2 6.1.1).
//
// @peak_mse: The peak MSE value observed within the measurement window.
// For 100BASE-T1, "pMSE" is optional and may be implemented via a sliding
// 128-symbol window with periodic capture; not standardized for 1000BASE-T1.
// (100BASE-T1 TC1 v1.0 6.1.3, Table "DCQ.peakMSE").
//
// @worst_peak_mse: A latched high-water mark of the peak MSE since last read
// (read-to-clear if implemented). OPEN Alliance shows a latched "worst case
// peak MSE" for 100BASE-T1 pMSE; availability/semantics outside that are
// vendor-specific. (100BASE-T1 TC1 v1.0 6.1.3, DCQ.peakMSE high byte;
// 1000BASE-T1 TC12 v2.2 treats DCQ details as vendor-specific.)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_mse_snapshot {
    pub average_mse: u64,
    pub peak_mse: u64,
    pub worst_peak_mse: u64,
}

//
// struct phy_driver - Driver structure for a particular PHY type
//
// @mdiodrv: Data common to all MDIO devices
// @phy_id: The result of reading the UID registers of this PHY
// type, and ANDing them with the phy_id_mask.  This driver
// only works for PHYs with IDs which match this field
// @name: The friendly name of this PHY type
// @phy_id_mask: Defines the important bits of the phy_id
// @features: A mandatory list of features (speed, duplex, etc)
// supported by this PHY
// @flags: A bitfield defining certain other features this PHY
// supports (like interrupts)
// @driver_data: Static driver data
//
// All functions are optional. If config_aneg or read_status
// are not implemented, the phy core uses the genphy versions.
// Note that none of these functions should be called from
// interrupt time. The goal is for the bus read/write functions
// to be able to block when the bus transaction is happening,
// and be freed up by an interrupt (The MPC85xx has this ability,
// though it is not currently supported in the driver).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_driver {
    pub mdiodrv: mdio_driver_common,
    pub phy_id: u32,
    pub name: *mut c_char,
    pub phy_id_mask: u32,
    pub features: *const *const c_ulong,
    pub flags: u32,
    pub driver_data: *const c_void,
//
// @soft_reset: Called to issue a PHY software reset
//
    pub phydev): *mut *mut int (soft_reset)(struct phy_device,
//
// @config_init: Called to initialize the PHY,
// including after a reset
//
    pub phydev): *mut *mut int (config_init)(struct phy_device,
//
// @probe: Called during discovery.  Used to set
// up device-specific structures, if any
//
    pub phydev): *mut *mut int (probe)(struct phy_device,
//
// @get_features: Probe the hardware to determine what
// abilities it has.  Should only set phydev->supported.
//
    pub phydev): *mut *mut int (get_features)(struct phy_device,
//
// @inband_caps: query whether in-band is supported for the given PHY
// interface mode. Returns a bitmask of bits defined by enum
// link_inband_signalling.
//
    pub interface): phy_interface_t,
//
// @config_inband: configure in-band mode for the PHY
//
    pub modes): *mut *mut *mut int (config_inband)(struct phy_device phydev, unsigned int,
//
// @get_rate_matching: Get the supported type of rate matching for a
// particular phy interface. This is used by phy consumers to determine
// whether to advertise lower-speed modes for that interface. It is
// assumed that if a rate matching mode is supported on an interface,
// then that interface's rate can be adapted to all slower link speeds
// supported by the phy. If the interface is not supported, this should
// return %RATE_MATCH_NONE.
//
    pub iface): phy_interface_t,
// PHY Power Management
// @suspend: Suspend the hardware, saving state if needed
    pub phydev): *mut *mut int (suspend)(struct phy_device,
// @resume: Resume the hardware, restoring state if needed
    pub phydev): *mut *mut int (resume)(struct phy_device,
//
// @config_aneg: Configures the advertisement and resets
// autonegotiation if phydev->autoneg is on,
// forces the speed to the current settings in phydev
// if phydev->autoneg is off
//
    pub phydev): *mut *mut int (config_aneg)(struct phy_device,
// @aneg_done: Determines the auto negotiation result
    pub phydev): *mut *mut int (aneg_done)(struct phy_device,
// @read_status: Determines the negotiated speed and duplex
    pub phydev): *mut *mut int (read_status)(struct phy_device,
//
// @config_intr: Enables or disables interrupts.
// It should also clear any pending interrupts prior to enabling the
// IRQs and after disabling them.
//
    pub phydev): *mut *mut int (config_intr)(struct phy_device,
// @handle_interrupt: Override default interrupt handling
    pub phydev): *mut *mut irqreturn_t (handle_interrupt)(struct phy_device,
// @remove: Clears up any memory if needed
    pub phydev): *mut *mut void (remove)(struct phy_device,
//
// @match_phy_device: Returns true if this is a suitable
// driver for the given phydev.	 If NULL, matching is based on
// phy_id and phy_id_mask.
//
    pub phydrv): *const phy_driver,
//
// @set_wol: Some devices (e.g. qnap TS-119P II) require PHY
// register changes to enable Wake on LAN, so set_wol is
// provided to be called in the ethernet driver's set_wol
// function.
//
    pub wol): *mut *mut *mut int (set_wol)(struct phy_device dev, struct ethtool_wolinfo,
//
// @get_wol: See set_wol, but for checking whether Wake on LAN
// is enabled.
//
    pub wol): *mut *mut *mut void (get_wol)(struct phy_device dev, struct ethtool_wolinfo,
//
// @link_change_notify: Called to inform a PHY device driver
// when the core is about to change the link state. This
// callback is supposed to be used as fixup hook for drivers
// that need to take action when the link state
// changes. Drivers are by no means allowed to mess with the
// PHY device structure in their implementations.
//
    pub dev): *mut *mut void (link_change_notify)(struct phy_device,
//
// @read_mmd: PHY specific driver override for reading a MMD
// register.  This function is optional for PHY specific
// drivers.  When not provided, the default MMD read function
// will be used by phy_read_mmd(), which will use either a
// direct read for Clause 45 PHYs or an indirect read for
// Clause 22 PHYs.  devnum is the MMD device number within the
// PHY device, regnum is the register within the selected MMD
// device.
//
    pub regnum): *mut *mut *mut int (read_mmd)(struct phy_device dev, int devnum, u16,
//
// @write_mmd: PHY specific driver override for writing a MMD
// register.  This function is optional for PHY specific
// drivers.  When not provided, the default MMD write function
// will be used by phy_write_mmd(), which will use either a
// direct write for Clause 45 PHYs, or an indirect write for
// Clause 22 PHYs.  devnum is the MMD device number within the
// PHY device, regnum is the register within the selected MMD
// device.  val is the value to be written.
//
    pub val): u16,
// @read_page: Return the current PHY register page number
    pub dev): *mut *mut int (read_page)(struct phy_device,
// @write_page: Set the current PHY register page number
    pub page): *mut *mut *mut int (write_page)(struct phy_device dev, int,
//
// @module_info: Get the size and type of the eeprom contained
// within a plug-in module
//
    pub modinfo): *mut ethtool_modinfo,
//
// @module_eeprom: Get the eeprom information from the plug-in
// module
//
    pub data): *mut *mut ethtool_eeprom ee, u8,
// @cable_test_start: Start a cable test
    pub dev): *mut *mut int (cable_test_start)(struct phy_device,
// @cable_test_tdr_start: Start a raw TDR cable test
    pub config): *const phy_tdr_config,
//
// @cable_test_get_status: Once per second, or on interrupt,
// request the status of the test.
//
    pub finished): *mut *mut *mut int (cable_test_get_status)(struct phy_device dev, bool,
// Get statistics from the PHY using ethtool
//
// @get_phy_stats: Retrieve PHY statistics.
// @dev: The PHY device for which the statistics are retrieved.
// @eth_stats: structure where Ethernet PHY stats will be stored.
// @stats: structure where additional PHY-specific stats will be stored.
//
// Retrieves the supported PHY statistics and populates the provided
// structures. The input structures are pre-initialized with
// `ETHTOOL_STAT_NOT_SET`, and the driver must only modify members
// corresponding to supported statistics. Unmodified members will remain
// set to `ETHTOOL_STAT_NOT_SET` and will not be returned to userspace.
//
    pub stats): *mut ethtool_phy_stats,
//
// @get_link_stats: Retrieve link statistics.
// @dev: The PHY device for which the statistics are retrieved.
// @link_stats: structure where link-specific stats will be stored.
//
// Retrieves link-related statistics for the given PHY device. The input
// structure is pre-initialized with `ETHTOOL_STAT_NOT_SET`, and the
// driver must only modify members corresponding to supported
// statistics. Unmodified members will remain set to
// `ETHTOOL_STAT_NOT_SET` and will not be returned to userspace.
//
    pub link_stats): *mut ethtool_link_ext_stats,
//
// @update_stats: Trigger periodic statistics updates.
// @dev: The PHY device for which statistics updates are triggered.
//
// Periodically gathers statistics from the PHY device to update locally
// maintained 64-bit counters. This is necessary for PHYs that implement
// reduced-width counters (e.g., 16-bit or 32-bit) which can overflow
// more frequently compared to 64-bit counters. By invoking this
// callback, drivers can fetch the current counter values, handle
// overflow detection, and accumulate the results into local 64-bit
// counters for accurate reporting through the `get_phy_stats` and
// `get_link_stats` interfaces.
//
// Return: 0 on success or a negative error code on failure.
//
    pub dev): *mut *mut int (update_stats)(struct phy_device,
// @get_sset_count: Number of statistic counters
    pub dev): *mut *mut int (get_sset_count)(struct phy_device,
// @get_strings: Names of the statistic counters
    pub data): *mut *mut *mut void (get_strings)(struct phy_device dev, u8,
// @get_stats: Return the statistic counter values
    pub data): *mut *mut ethtool_stats stats, u64,
//
// @disable_autonomous_eee: Disable PHY-autonomous EEE
//
// Some PHYs manage EEE autonomously, preventing the MAC from
// controlling LPI signaling. This callback disables autonomous
// EEE at the PHY.
//
// Return: 0 on success, negative errno on failure.
//
    pub dev): *mut *mut int (disable_autonomous_eee)(struct phy_device,
// Get and Set PHY tunables
// @get_tunable: Return the value of a tunable
    pub data): *mut *mut ethtool_tunable tuna, void,
// @set_tunable: Set the value of a tunable
    pub data): *const c_void,
//
// @set_loopback: Set the loopback mode of the PHY
// enable selects if the loopback mode is enabled or disabled. If the
// loopback mode is enabled, then the speed of the loopback mode can be
// requested with the speed argument. If the speed argument is zero,
// then any speed can be selected. If the speed argument is > 0, then
// this speed shall be selected for the loopback mode or EOPNOTSUPP
// shall be returned if speed selection is not supported.
//
    pub speed): *mut *mut *mut int (set_loopback)(struct phy_device dev, bool enable, int,
// @get_sqi: Get the signal quality indication
    pub dev): *mut *mut int (get_sqi)(struct phy_device,
// @get_sqi_max: Get the maximum signal quality indication
    pub dev): *mut *mut int (get_sqi_max)(struct phy_device,
//
// @get_mse_capability: Get capabilities and scale of MSE measurement
// @dev:    PHY device
// @cap: Output (filled on success)
//
// Fill @cap with the PHY's MSE capability for the current
// link mode: scale limits (max_average_mse, max_peak_mse), update
// interval (refresh_rate_ps), sample length (num_symbols) and the
// capability bitmask (supported_caps).
//
// Implementations may defer capability report until hardware has
// converged; in that case they should return -EAGAIN and allow the
// caller to retry later.
//
// Return: 0 on success. On failure, returns a negative errno code, such
// as -EOPNOTSUPP if MSE measurement is not supported by the PHY or in
// the current link mode, or -EAGAIN if the capability information is
// not yet available.
//
    pub cap): *mut phy_mse_capability,
//
// @get_mse_snapshot: Retrieve a snapshot of MSE diagnostic values
// @dev:      PHY device
// @channel:  Channel identifier (PHY_MSE_CHANNEL_*)
// @snapshot: Output (filled on success)
//
// Fill @snapshot with a correlated set of MSE values from the most
// recent measurement window.
//
// Callers must validate @channel against supported_caps returned by
// get_mse_capability(). Drivers must not coerce @channel; if the
// requested selector is not implemented by the device or current link
// mode, the operation must fail.
//
// worst_peak_mse is latched and must be treated as read-to-clear.
//
// Return: 0 on success. On failure, returns a negative errno code, such
// as -EOPNOTSUPP if MSE measurement is not supported by the PHY or in
// the current link mode, or -EAGAIN if measurements are not yet
// available.
//
    pub snapshot): *mut phy_mse_snapshot,
// PLCA RS interface
// @get_plca_cfg: Return the current PLCA configuration
    pub plca_cfg): *mut phy_plca_cfg,
// @set_plca_cfg: Set the PLCA configuration
    pub plca_cfg): *const phy_plca_cfg,
// @get_plca_status: Return the current PLCA status info
    pub plca_st): *mut phy_plca_status,
//
// @led_brightness_set: Set a PHY LED brightness. Index
// indicates which of the PHYs led should be set. Value
// follows the standard LED class meaning, e.g. LED_OFF,
// LED_HALF, LED_FULL.
//
    pub value): u8 index, enum led_brightness,
//
// @led_blink_set: Set a PHY LED blinking.  Index indicates
// which of the PHYs led should be configured to blink. Delays
// are in milliseconds and if both are zero then a sensible
// default should be chosen.  The call should adjust the
// timings in that case and if it can't match the values
// specified exactly.
//
    pub delay_off): *mut c_ulong,
//
// @led_hw_is_supported: Can the HW support the given rules.
// @dev: PHY device which has the LED
// @index: Which LED of the PHY device
// @rules The core is interested in these rules
//
// Return 0 if yes,  -EOPNOTSUPP if not, or an error code.
//
    pub rules): c_ulong,
//
// @led_hw_control_set: Set the HW to control the LED
// @dev: PHY device which has the LED
// @index: Which LED of the PHY device
// @rules The rules used to control the LED
//
// Returns 0, or a an error code.
//
    pub rules): c_ulong,
//
// @led_hw_control_get: Get how the HW is controlling the LED
// @dev: PHY device which has the LED
// @index: Which LED of the PHY device
// @rules Pointer to the rules used to control the LED
//
// Set *@rules to how the HW is currently blinking. Returns 0
// on success, or a error code if the current blinking cannot
// be represented in rules, or some other error happens.
//
    pub rules): *mut c_ulong,
//
// @led_polarity_set: Set the LED polarity modes
// @dev: PHY device which has the LED
// @index: Which LED of the PHY device
// @modes: bitmap of LED polarity modes
//
// Configure LED with all the required polarity modes in @modes
// to make it correctly turn ON or OFF.
//
// Returns 0, or an error code.
//
    pub modes): c_ulong,
//
// @get_next_update_time: Get the time until the next update event
// @dev: PHY device
//
// Callback to determine the time (in jiffies) until the next
// update event for the PHY state  machine. Allows PHY drivers to
// dynamically adjust polling intervals based on link state or other
// conditions.
//
// Returns the time in jiffies until the next update event.
//
    pub dev): *mut *mut unsigned int (get_next_update_time)(struct phy_device,
//
// @attach_mii_port: Attach the given MII port to the PHY device
// @dev: PHY device to notify
// @port: The port being added
//
// Called when an MII port that needs to be driven by the PHY is found.
//
// The port that is being passed may or may not be initialized. If it is
// already initialized, it is by the generic port representation from
// devicetree, which superseeds any strapping or vendor-specific
// properties.
//
// If the port isn't initialized, the port->mediums and port->lanes
// fields must be set, possibly according to strapping information.
//
// The PHY driver must set the port->interfaces field to indicate the
// possible MII modes that this PHY can output on the port.
//
// Returns 0, or an error code.
//
    pub port): *mut *mut *mut int (attach_mii_port)(struct phy_device dev, struct phy_port,
//
// @attach_mdi_port: Attach the given MII port to the PHY device
// @dev: PHY device to notify
// @port: The port being added
//
// Called when a port that needs to be driven by the PHY is found. The
// number of time this will be called depends on phydev->max_n_ports,
// which the driver can change in .probe().
//
// The port that is being passed may or may not be initialized. If it is
// already initialized, it is by the generic port representation from
// devicetree, which superseeds any strapping or vendor-specific
// properties.
//
// If the port isn't initialized, the port->mediums and port->lanes
// fields must be set, possibly according to strapping information.
//
// Returns 0, or an error code.
//
    pub port): *mut *mut *mut int (attach_mdi_port)(struct phy_device dev, struct phy_port,
}

//
// phy_id_compare - compare @id1 with @id2 taking account of @mask
// @id1: first PHY ID
// @id2: second PHY ID
// @mask: the PHY ID mask, set bits are significant in matching
//
// Return true if the bits from @id1 and @id2 specified by @mask match.
// This uses an equivalent test to (@id & @mask) == (@phy_id & @mask).
//
// phy_id_compare_vendor - compare @id with @vendor mask
// @id: PHY ID
// @vendor_mask: PHY Vendor mask
//
// Return: true if the bits from @id match @vendor using the
// generic PHY Vendor mask.
//
extern "C" {
    pub fn phy_id_compare(_arg: id, _arg: vendor_mask, _arg: PHY_ID_MATCH_VENDOR_MASK) -> return;
}
//
// phy_id_compare_model - compare @id with @model mask
// @id: PHY ID
// @model_mask: PHY Model mask
//
// Return: true if the bits from @id match @model using the
// generic PHY Model mask.
//
extern "C" {
    pub fn phy_id_compare(_arg: id, _arg: model_mask, _arg: PHY_ID_MATCH_MODEL_MASK) -> return;
}
//
// phydev_id_compare - compare @id with the PHY's Clause 22 ID
// @phydev: the PHY device
// @id: the PHY ID to be matched
//
// Compare the @phydev clause 22 ID with the provided @id and return true or
// false depending whether it matches, using the bound driver mask. The
// @phydev must be bound to a driver.
//
extern "C" {
    pub fn phy_id_compare(_arg: id, _arg: phydev->phy_id, _arg: phydev->drv->phy_id_mask) -> return;
}
extern "C" {
    pub fn phy_interface_num_ports(interface: phy_interface_t) -> c_int;
}
//
// phy_is_started - Convenience function to check whether PHY is started
// @phydev: The phy_device struct
//
// phy_driver_is_genphy - Convenience function to check whether PHY is driven
// by one of the generic PHY drivers
// @phydev: The phy_device struct
// Return: true if PHY is driven by one of the genphy drivers
//
// phy_disable_eee_mode - Don't advertise an EEE mode.
// @phydev: The phy_device struct
// @link_mode: The EEE mode to be disabled
//
// phy_can_wakeup() - indicate whether PHY has driver model wakeup capabilities
// @phydev: The phy_device struct
//
// Returns: true/false depending on the PHY driver's device_set_wakeup_capable()
// setting.
//
extern "C" {
    pub fn device_can_wakeup(_arg: &phydev->mdio.dev) -> return;
}
//
// phy_may_wakeup() - indicate whether PHY has wakeup enabled
// @phydev: The phy_device struct
//
// Returns: true/false depending on the PHY driver's device_set_wakeup_enabled()
// setting if using the driver model, otherwise the legacy determination.
//
extern "C" {
    pub fn phy_may_wakeup(phydev: *mut phy_device) -> bool;
}
extern "C" {
    pub fn phy_resolve_aneg_pause(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_resolve_aneg_linkmode(phydev: *mut phy_device);
}
//
// phy_read - Convenience function for reading a given PHY register
// @phydev: the phy_device struct
// @regnum: register number to read
//
// NOTE: MUST NOT be called from interrupt context,
// because the bus read/write functions may wait for an interrupt
// to conclude the operation.
//
extern "C" {
    pub fn mdiobus_read(_arg: phydev->mdio.bus, _arg: phydev->mdio.addr, _arg: regnum) -> return;
}

//
// __phy_read - convenience function for reading a given PHY register
// @phydev: the phy_device struct
// @regnum: register number to read
//
// The caller must have taken the MDIO bus lock.
//
extern "C" {
    pub fn __mdiobus_read(_arg: phydev->mdio.bus, _arg: phydev->mdio.addr, _arg: regnum) -> return;
}
//
// phy_write - Convenience function for writing a given PHY register
// @phydev: the phy_device struct
// @regnum: register number to write
// @val: value to write to @regnum
//
// NOTE: MUST NOT be called from interrupt context,
// because the bus read/write functions may wait for an interrupt
// to conclude the operation.
//
extern "C" {
    pub fn mdiobus_write(_arg: phydev->mdio.bus, _arg: phydev->mdio.addr, _arg: regnum, _arg: val) -> return;
}
//
// __phy_write - Convenience function for writing a given PHY register
// @phydev: the phy_device struct
// @regnum: register number to write
// @val: value to write to @regnum
//
// The caller must have taken the MDIO bus lock.
//
// __phy_modify_changed() - Convenience function for modifying a PHY register
// @phydev: a pointer to a &struct phy_device
// @regnum: register number
// @mask: bit mask of bits to clear
// @set: bit mask of bits to set
//
// Unlocked helper function which allows a PHY register to be modified as
// new register value = (old register value & ~mask) | set
//
// Returns negative errno, 0 if there was no change, and 1 in case of change
//
// phy_read_mmd - Convenience function for reading a register
// from an MMD on a given PHY.
//
extern "C" {
    pub fn phy_read_mmd(phydev: *mut phy_device, devad: c_int, regnum: u32) -> c_int;
}
//
// phy_read_mmd_poll_timeout - Periodically poll a PHY register until a
// condition is met or a timeout occurs
//
// @phydev: The phy_device struct
// @devaddr: The MMD to read from
// @regnum: The register on the MMD to read
// @val: Variable to read the register into
// @cond: Break condition (usually involving @val)
// @sleep_us: Maximum time to sleep between reads in us (0 tight-loops). Please
// read usleep_range() function description for details and
// limitations.
// @timeout_us: Timeout in us, 0 means never timeout
// @sleep_before_read: if it is true, sleep @sleep_us before read.
//
// Returns: 0 on success and -ETIMEDOUT upon a timeout. In either
// case, the last read value at @args is stored in @val. Must not
// be called from atomic context if sleep_us or timeout_us are used.
//

//
// __phy_read_mmd - Convenience function for reading a register
// from an MMD on a given PHY.
//
extern "C" {
    pub fn __phy_read_mmd(phydev: *mut phy_device, devad: c_int, regnum: u32) -> c_int;
}
//
// phy_write_mmd - Convenience function for writing a register
// on an MMD on a given PHY.
//
extern "C" {
    pub fn phy_write_mmd(phydev: *mut phy_device, devad: c_int, regnum: u32, val: u16) -> c_int;
}
//
// __phy_write_mmd - Convenience function for writing a register
// on an MMD on a given PHY.
//
extern "C" {
    pub fn __phy_write_mmd(phydev: *mut phy_device, devad: c_int, regnum: u32, val: u16) -> c_int;
}
extern "C" {
    pub fn __phy_modify(phydev: *mut phy_device, regnum: u32, mask: u16, set: u16) -> c_int;
}
extern "C" {
    pub fn phy_modify(phydev: *mut phy_device, regnum: u32, mask: u16, set: u16) -> c_int;
}
//
// __phy_set_bits - Convenience function for setting bits in a PHY register
// @phydev: the phy_device struct
// @regnum: register number to write
// @val: bits to set
//
// The caller must have taken the MDIO bus lock.
//
extern "C" {
    pub fn __phy_modify(_arg: phydev, _arg: regnum, _arg: 0, _arg: val) -> return;
}
//
// __phy_clear_bits - Convenience function for clearing bits in a PHY register
// @phydev: the phy_device struct
// @regnum: register number to write
// @val: bits to clear
//
// The caller must have taken the MDIO bus lock.
//
extern "C" {
    pub fn __phy_modify(_arg: phydev, _arg: regnum, _arg: val, _arg: 0) -> return;
}
//
// phy_set_bits - Convenience function for setting bits in a PHY register
// @phydev: the phy_device struct
// @regnum: register number to write
// @val: bits to set
//
extern "C" {
    pub fn phy_modify(_arg: phydev, _arg: regnum, _arg: 0, _arg: val) -> return;
}
//
// phy_clear_bits - Convenience function for clearing bits in a PHY register
// @phydev: the phy_device struct
// @regnum: register number to write
// @val: bits to clear
//
extern "C" {
    pub fn phy_modify(_arg: phydev, _arg: regnum, _arg: val, _arg: 0) -> return;
}
//
// __phy_set_bits_mmd - Convenience function for setting bits in a register
// on MMD
// @phydev: the phy_device struct
// @devad: the MMD containing register to modify
// @regnum: register number to modify
// @val: bits to set
//
// The caller must have taken the MDIO bus lock.
//
extern "C" {
    pub fn __phy_modify_mmd(_arg: phydev, _arg: devad, _arg: regnum, _arg: 0, _arg: val) -> return;
}
//
// __phy_clear_bits_mmd - Convenience function for clearing bits in a register
// on MMD
// @phydev: the phy_device struct
// @devad: the MMD containing register to modify
// @regnum: register number to modify
// @val: bits to clear
//
// The caller must have taken the MDIO bus lock.
//
extern "C" {
    pub fn __phy_modify_mmd(_arg: phydev, _arg: devad, _arg: regnum, _arg: val, _arg: 0) -> return;
}
//
// phy_set_bits_mmd - Convenience function for setting bits in a register
// on MMD
// @phydev: the phy_device struct
// @devad: the MMD containing register to modify
// @regnum: register number to modify
// @val: bits to set
//
extern "C" {
    pub fn phy_modify_mmd(_arg: phydev, _arg: devad, _arg: regnum, _arg: 0, _arg: val) -> return;
}
//
// phy_clear_bits_mmd - Convenience function for clearing bits in a register
// on MMD
// @phydev: the phy_device struct
// @devad: the MMD containing register to modify
// @regnum: register number to modify
// @val: bits to clear
//
extern "C" {
    pub fn phy_modify_mmd(_arg: phydev, _arg: devad, _arg: regnum, _arg: val, _arg: 0) -> return;
}
//
// phy_interrupt_is_valid - Convenience function for testing a given PHY irq
// @phydev: the phy_device struct
//
// NOTE: must be kept in sync with addition/removal of PHY_POLL and
// PHY_MAC_INTERRUPT
//
// phy_polling_mode - Convenience function for testing whether polling is
// used to detect PHY status changes
// @phydev: the phy_device struct
//
// phy_has_hwtstamp - Tests whether a PHY time stamp configuration.
// @phydev: the phy_device struct
//
// phy_has_rxtstamp - Tests whether a PHY supports receive time stamping.
// @phydev: the phy_device struct
//
// phy_has_tsinfo - Tests whether a PHY reports time stamping and/or
// PTP hardware clock capabilities.
// @phydev: the phy_device struct
//
// phy_has_txtstamp - Tests whether a PHY supports transmit time stamping.
// @phydev: the phy_device struct
//
// phy_is_default_hwtstamp - Is the PHY hwtstamp the default timestamp
// @phydev: Pointer to phy_device
//
// This is used to get default timestamping device taking into account
// the new API choice, which is selecting the timestamping from MAC by
// default if the phydev does not have default_timestamp flag enabled.
//
// Return: True if phy is the default hw timestamp, false otherwise.
//
// phy_on_sfp - Convenience function for testing if a PHY is on an SFP module
// @phydev: the phy_device struct
//
// phy_interface_mode_is_rgmii - Convenience function for testing if a
// PHY interface mode is RGMII (all variants)
// @mode: the &phy_interface_t enum
//
// phy_interface_mode_is_8023z() - does the PHY interface mode use 802.3z
// negotiation
// @mode: one of &enum phy_interface_t
//
// Returns true if the PHY interface mode uses the 16-bit negotiation
// word as defined in 802.3z. (See 802.3-2015 37.2.1 Config_Reg encoding)
//
// phy_interface_is_rgmii - Convenience function for testing if a PHY interface
// is RGMII (all variants)
// @phydev: the phy_device struct
//
extern "C" {
    pub fn phy_interface_mode_is_rgmii(_arg: phydev->interface) -> return;
}
//
// phy_is_pseudo_fixed_link - Convenience function for testing if this
// PHY is the CPU port facing side of an Ethernet switch, or similar.
// @phydev: the phy_device struct
//
extern "C" {
    pub fn phy_save_page(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn phy_select_page(phydev: *mut phy_device, page: c_int) -> c_int;
}
extern "C" {
    pub fn phy_restore_page(phydev: *mut phy_device, oldpage: c_int, ret: c_int) -> c_int;
}
extern "C" {
    pub fn phy_read_paged(phydev: *mut phy_device, page: c_int, regnum: u32) -> c_int;
}
extern "C" {
    pub fn phy_write_paged(phydev: *mut phy_device, page: c_int, regnum: u32, val: u16) -> c_int;
}
extern "C" {
    pub fn fwnode_get_phy_id(fwnode: *mut fwnode_handle, phy_id: *mut u32) -> c_int;
}
extern "C" {
    pub fn phy_device_register(phy: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn phy_device_free(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_device_remove(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_get_c45_ids(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn phy_init_hw(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn phy_suspend(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn phy_resume(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn __phy_resume(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn phy_loopback(phydev: *mut phy_device, enable: bool, speed: c_int) -> c_int;
}
extern "C" {
    pub fn phy_disconnect(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_detach(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_start(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_stop(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_config_aneg(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn _phy_start_aneg(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn phy_start_aneg(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn phy_aneg_done(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn phy_config_inband(phydev: *mut phy_device, modes: c_uint) -> c_int;
}
extern "C" {
    pub fn phy_speed_down(phydev: *mut phy_device, sync: bool) -> c_int;
}
extern "C" {
    pub fn phy_speed_up(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn phy_check_valid(speed: c_int, duplex: c_int, features: *mut c_ulong) -> bool;
}
extern "C" {
    pub fn phy_restart_aneg(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn phy_reset_after_clk_enable(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn phy_find_next(_arg: bus, _arg: NULL) -> return;
}

extern "C" {
    pub fn mdiodev_has_reset(_arg: &phydev->mdio) -> return;
}

extern "C" {
    pub fn dev_name(_arg: &phydev->mdio.dev) -> return;
}
extern "C" {
    pub fn phy_attached_info(phydev: *mut phy_device);
}
// Clause 22 PHY
extern "C" {
    pub fn genphy_read_abilities(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_setup_forced(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_restart_aneg(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_check_and_restart_aneg(phydev: *mut phy_device, restart: bool) -> c_int;
}
extern "C" {
    pub fn __genphy_config_aneg(phydev: *mut phy_device, changed: bool) -> c_int;
}
extern "C" {
    pub fn genphy_aneg_done(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_update_link(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_read_lpa(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_read_status_fixed(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_read_status(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_read_master_slave(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_suspend(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_resume(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_loopback(phydev: *mut phy_device, enable: bool, speed: c_int) -> c_int;
}
extern "C" {
    pub fn genphy_soft_reset(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_handle_interrupt_no_ack(phydev: *mut phy_device) -> irqreturn_t;
}
extern "C" {
    pub fn __genphy_config_aneg(_arg: phydev, _arg: false) -> return;
}
extern "C" {
    pub fn genphy_read_mmd_c45(phydev: *mut phy_device, devnum: c_int, regnum: u16) -> c_int;
}
// Clause 37
extern "C" {
    pub fn genphy_c37_config_aneg(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c37_read_status(phydev: *mut phy_device, changed: *mut bool) -> c_int;
}
// Clause 45 PHY
extern "C" {
    pub fn genphy_c45_restart_aneg(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_check_and_restart_aneg(phydev: *mut phy_device, restart: bool) -> c_int;
}
extern "C" {
    pub fn genphy_c45_pma_soft_reset(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_aneg_done(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_read_link(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_read_lpa(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_read_pma(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_pma_setup_forced(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_pma_baset1_setup_master_slave(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_an_config_aneg(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_an_disable_aneg(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_read_mdix(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_pma_read_abilities(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_pma_read_ext_abilities(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_pma_baset1_read_abilities(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_read_eee_abilities(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_pma_baset1_read_master_slave(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_read_status(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_baset1_read_status(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_config_aneg(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_loopback(phydev: *mut phy_device, enable: bool, speed: c_int) -> c_int;
}
extern "C" {
    pub fn genphy_c45_pma_resume(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_pma_suspend(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_fast_retrain(phydev: *mut phy_device, enable: bool) -> c_int;
}
extern "C" {
    pub fn genphy_c45_eee_is_active(phydev: *mut phy_device, lp: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn genphy_c45_an_config_eee_aneg(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_oatc14_cable_test_start(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_oatc14_get_sqi_max(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_c45_oatc14_get_sqi(phydev: *mut phy_device) -> c_int;
}
// The gen10g_* functions are the old Clause 45 stub
extern "C" {
    pub fn gen10g_config_aneg(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn genphy_read_status(_arg: phydev) -> return;
}
extern "C" {
    pub fn phy_drivers_unregister(drv: *mut phy_driver, n: c_int);
}
extern "C" {
    pub fn phy_error(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_state_machine(work: *mut work_struct);
}
extern "C" {
    pub fn phy_trigger_machine(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_mac_interrupt(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_start_machine(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_stop_machine(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_mii_ioctl(phydev: *mut phy_device, ifr: *mut ifreq, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn phy_do_ioctl(dev: *mut net_device, ifr: *mut ifreq, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn phy_do_ioctl_running(dev: *mut net_device, ifr: *mut ifreq, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn phy_disable_interrupts(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn phy_request_interrupt(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_free_interrupt(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_print_status(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_set_max_speed(phydev: *mut phy_device, max_speed: u32);
}
extern "C" {
    pub fn phy_remove_link_mode(phydev: *mut phy_device, link_mode: u32);
}
extern "C" {
    pub fn phy_advertise_supported(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_advertise_eee_all(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_support_sym_pause(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_support_asym_pause(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_support_eee(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_disable_eee(phydev: *mut phy_device);
}
extern "C" {
    pub fn phy_set_asym_pause(phydev: *mut phy_device, rx: bool, tx: bool);
}
extern "C" {
    pub fn phy_get_pause(phydev: *mut phy_device, tx_pause: *mut bool, rx_pause: *mut bool);
}
extern "C" {
    pub fn phy_eee_tx_clock_stop_capable(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn phy_eee_rx_clock_stop(phydev: *mut phy_device, clk_stop_enable: bool) -> c_int;
}
extern "C" {
    pub fn phy_init_eee(phydev: *mut phy_device, clk_stop_enable: bool) -> c_int;
}
extern "C" {
    pub fn phy_get_eee_err(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn phy_ethtool_set_eee(phydev: *mut phy_device, data: *mut ethtool_keee) -> c_int;
}
extern "C" {
    pub fn phy_ethtool_get_eee(phydev: *mut phy_device, data: *mut ethtool_keee) -> c_int;
}
extern "C" {
    pub fn phy_ethtool_set_wol(phydev: *mut phy_device, wol: *mut ethtool_wolinfo) -> c_int;
}
extern "C" {
    pub fn phy_ethtool_nway_reset(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn phy_ethtool_get_strings(phydev: *mut phy_device, data: *mut u8) -> c_int;
}
extern "C" {
    pub fn phy_ethtool_get_sset_count(phydev: *mut phy_device) -> c_int;
}
//
// phy_module_driver() - Helper macro for registering PHY drivers
// @__phy_drivers: array of PHY drivers to register
// @__count: Numbers of members in array
//
// Helper macro for PHY drivers which do not do anything special in module
// init/exit. Each module may only use this macro once, and calling it
// replaces module_init() and module_exit().
//

