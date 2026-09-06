//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/phylink.h
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


// PCS "negotiation" mode.
// PHYLINK_PCS_NEG_NONE - protocol has no inband capability
// PHYLINK_PCS_NEG_OUTBAND - some out of band or fixed link setting
// PHYLINK_PCS_NEG_INBAND_DISABLED - inband mode disabled, e.g.
// 1000base-X with autoneg off
// PHYLINK_PCS_NEG_INBAND_ENABLED - inband mode enabled
// Additionally, this can be tested using bitmasks:
// PHYLINK_PCS_NEG_INBAND - inband mode selected
// PHYLINK_PCS_NEG_ENABLED - negotiation mode enabled
//
// MAC_SYM_PAUSE and MAC_ASYM_PAUSE are used when configuring our
// autonegotiation advertisement. They correspond to the PAUSE and
// ASM_DIR bits defined by 802.3, respectively.
//
// The following table lists the values of tx_pause and rx_pause which
// might be requested in mac_link_up. The exact values depend on either
// the results of autonegotation (if MLO_PAUSE_AN is set) or user
// configuration (if MLO_PAUSE_AN is not set).
//
// MAC_SYM_PAUSE MAC_ASYM_PAUSE MLO_PAUSE_AN tx_pause/rx_pause
// ============= ============== ============ ==================
// 0              0            0 0/0
// 0              0            1 0/0
// 0              1            0 0/0, 0/1, 1/0, 1/1
// 0              1            1 0/0,      1/0
// 1              0            0 0/0,           1/1
// 1              0            1 0/0,           1/1
// 1              1            0 0/0, 0/1, 1/0, 1/1
// 1              1            1 0/0, 0/1,      1/1
//
// If you set MAC_ASYM_PAUSE, the user may request any combination of
// tx_pause and rx_pause. You do not have to support these
// combinations.
//
// However, you should support combinations of tx_pause and rx_pause
// which might be the result of autonegotation. For example, don't set
// MAC_SYM_PAUSE unless your device can support tx_pause and rx_pause
// at the same time.
//
// struct phylink_link_state - link state structure
// @advertising: ethtool bitmask containing advertised link modes
// @lp_advertising: ethtool bitmask containing link partner advertised link
// modes
// @interface: link &typedef phy_interface_t mode
// @speed: link speed, one of the SPEED_* constants.
// @duplex: link duplex mode, one of DUPLEX_* constants.
// @pause: link pause state, described by MLO_PAUSE_* constants.
// @rate_matching: rate matching being performed, one of the RATE_MATCH_
// constants. If rate matching is taking place, then the speed/duplex of
// the medium link mode (@speed and @duplex) and the speed/duplex of the phy
// interface mode (@interface) are different.
// @link: true if the link is up.
// @an_complete: true if autonegotiation has completed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phylink_link_state {
    pub interface: phy_interface_t,
    pub speed: c_int,
    pub duplex: c_int,
    pub pause: c_int,
    pub rate_matching: c_int,
    pub link:1: c_uint,
    pub an_complete:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phylink_op_type {
    PHYLINK_NETDEV = 0,
    PHYLINK_DEV,
}

//
// struct phylink_config - PHYLINK configuration structure
// @dev: a pointer to a struct device associated with the MAC
// @type: operation type of PHYLINK instance
// @poll_fixed_state: if true, starts link_poll,
// if MAC link is at %MLO_AN_FIXED mode.
// @mac_managed_pm: if true, indicate the MAC driver is responsible for PHY PM.
// @mac_requires_rxc: if true, the MAC always requires a receive clock from PHY.
// The PHY driver should start the clock signal as soon as
// possible and avoid stopping it during suspend events.
// @default_an_inband: if true, defaults to MLO_AN_INBAND rather than
// MLO_AN_PHY. A fixed-link specification will override.
// @eee_rx_clk_stop_enable: if true, PHY can stop the receive clock during LPI
// @get_fixed_state: callback to execute to determine the fixed link state,
// if MAC link is at %MLO_AN_FIXED mode.
// @supported_interfaces: bitmap describing which PHY_INTERFACE_MODE_xxx
// are supported by the MAC/PCS.
// @lpi_interfaces: bitmap describing which PHY interface modes can support
// LPI signalling.
// @mac_capabilities: MAC pause/speed/duplex capabilities.
// @lpi_capabilities: MAC speeds which can support LPI signalling
// @lpi_timer_default: Default EEE LPI timer setting.
// @eee_enabled_default: If set, EEE will be enabled by phylink at creation time
// @wol_phy_legacy: Use Wake-on-Lan with PHY even if phy_can_wakeup() is false
// @wol_phy_speed_ctrl: Use phy speed control on suspend/resume
// @wol_mac_support: Bitmask of MAC supported %WAKE_* options
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phylink_config {
    pub dev: *mut device,
    pub type: phylink_op_type,
    pub poll_fixed_state: bool,
    pub mac_managed_pm: bool,
    pub mac_requires_rxc: bool,
    pub default_an_inband: bool,
    pub eee_rx_clk_stop_enable: bool,
    pub state): *mut phylink_link_state,
    pub mac_capabilities: c_ulong,
    pub lpi_capabilities: c_ulong,
    pub lpi_timer_default: u32,
    pub eee_enabled_default: bool,
// Wake-on-Lan support
    pub wol_phy_legacy: bool,
    pub wol_phy_speed_ctrl: bool,
    pub wol_mac_support: u32,
}

extern "C" {
    pub fn phylink_limit_mac_speed(config: *mut phylink_config, max_speed: u32);
}
//
// struct phylink_mac_ops - MAC operations structure.
// @mac_get_caps: Get MAC capabilities for interface mode.
// @mac_select_pcs: Select a PCS for the interface mode.
// @mac_prepare: prepare for a major reconfiguration of the interface.
// @mac_config: configure the MAC for the selected mode and state.
// @mac_finish: finish a major reconfiguration of the interface.
// @mac_link_down: take the link down.
// @mac_link_up: allow the link to come up.
// @mac_disable_tx_lpi: disable LPI.
// @mac_enable_tx_lpi: enable and configure LPI.
// @mac_wol_set: configure Wake-on-Lan settings at the MAC.
//
// The individual methods are described more fully below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phylink_mac_ops {
    pub interface): phy_interface_t,
    pub interface): phy_interface_t,
    pub iface): phy_interface_t,
    pub state): *const phylink_link_state,
    pub iface): phy_interface_t,
    pub interface): phy_interface_t,
    pub rx_pause): bool tx_pause, bool,
    pub config): *mut *mut void (mac_disable_tx_lpi)(struct phylink_config,
    pub tx_clk_stop): bool,
    pub sopass): *const u8,
}

//
// mac_get_caps: Get MAC capabilities for interface mode.
// @config: a pointer to a &struct phylink_config.
// @interface: PHY interface mode.
//
// Optional method. When not provided, config->mac_capabilities will be used.
// When implemented, this returns the MAC capabilities for the specified
// interface mode where there is some special handling required by the MAC
// driver (e.g. not supporting half-duplex in certain interface modes.)
//
// mac_select_pcs: Select a PCS for the interface mode.
// @config: a pointer to a &struct phylink_config.
// @interface: PHY interface mode for PCS
//
// Return the &struct phylink_pcs for the specified interface mode, or
// NULL if none is required, or an error pointer on error.
//
// This must not modify any state. It is used to query which PCS should
// be used. Phylink will use this during validation to ensure that the
// configuration is valid, and when setting a configuration to internally
// set the PCS that will be used.
//
// mac_prepare() - prepare to change the PHY interface mode
// @config: a pointer to a &struct phylink_config.
// @mode: one of %MLO_AN_FIXED, %MLO_AN_PHY, %MLO_AN_INBAND.
// @iface: interface mode to switch to
//
// phylink will call this method at the beginning of a full initialisation
// of the link, which includes changing the interface mode or at initial
// startup time. It may be called for the current mode. The MAC driver
// should perform whatever actions are required, e.g. disabling the
// Serdes PHY.
//
// This will be the first call in the sequence:
// - mac_prepare()
// - mac_config()
// - pcs_config()
// - possible pcs_an_restart()
// - mac_finish()
//
// Returns zero on success, or negative errno on failure which will be
// reported to the kernel log.
//
// mac_config() - configure the MAC for the selected mode and state
// @config: a pointer to a &struct phylink_config.
// @mode: one of %MLO_AN_FIXED, %MLO_AN_PHY, %MLO_AN_INBAND.
// @state: a pointer to a &struct phylink_link_state.
//
// Note - not all members of @state are valid.  In particular,
// @state->lp_advertising, @state->link, @state->an_complete are never
// guaranteed to be correct, and so any mac_config() implementation must
// never reference these fields.
//
// This will only be called to reconfigure the MAC for a "major" change in
// e.g. interface mode. It will not be called for changes in speed, duplex
// or pause modes or to change the in-band advertisement.
//
// In all negotiation modes, as defined by @mode, @state->pause indicates the
// pause settings which should be applied as follows. If %MLO_PAUSE_AN is not
// set, %MLO_PAUSE_TX and %MLO_PAUSE_RX indicate whether the MAC should send
// pause frames and/or act on received pause frames respectively. Otherwise,
// the results of in-band negotiation/status from the MAC PCS should be used
// to control the MAC pause mode settings.
//
// The action performed depends on the currently selected mode:
//
// %MLO_AN_FIXED, %MLO_AN_PHY:
// Configure for non-inband negotiation mode, where the link settings
// are completely communicated via mac_link_up().  The physical link
// protocol from the MAC is specified by @state->interface.
//
// @state->advertising may be used, but is not required.
//
// Older drivers (prior to the mac_link_up() change) may use @state->speed,
// @state->duplex and @state->pause to configure the MAC, but this is
// deprecated; such drivers should be converted to use mac_link_up().
//
// Other members of @state must be ignored.
//
// Valid state members: interface, advertising.
// Deprecated state members: speed, duplex, pause.
//
// %MLO_AN_INBAND:
// place the link in an inband negotiation mode (such as 802.3z
// 1000base-X or Cisco SGMII mode depending on the @state->interface
// mode). In both cases, link state management (whether the link
// is up or not) is performed by the MAC, and reported via the
// pcs_get_state() callback. Changes in link state must be made
// by calling phylink_mac_change().
//
// Interface mode specific details are mentioned below.
//
// If in 802.3z mode, the link speed is fixed, dependent on the
// @state->interface. Duplex and pause modes are negotiated via
// the in-band configuration word. Advertised pause modes are set
// according to @state->advertising. Beware of MACs which only
// support full duplex at gigabit and higher speeds.
//
// If in Cisco SGMII mode, the link speed and duplex mode are passed
// in the serial bitstream 16-bit configuration word, and the MAC
// should be configured to read these bits and acknowledge the
// configuration word. Nothing is advertised by the MAC. The MAC is
// responsible for reading the configuration word and configuring
// itself accordingly.
//
// Valid state members: interface, pause, advertising.
//
// Implementations are expected to update the MAC to reflect the
// requested settings - i.o.w., if nothing has changed between two
// calls, no action is expected.  If only flow control settings have
// changed, flow control should be updated *without* taking the link
// down.  This "update" behaviour is critical to avoid bouncing the
// link up status.
//
// mac_finish() - finish a to change the PHY interface mode
// @config: a pointer to a &struct phylink_config.
// @mode: one of %MLO_AN_FIXED, %MLO_AN_PHY, %MLO_AN_INBAND.
// @iface: interface mode to switch to
//
// phylink will call this if it called mac_prepare() to allow the MAC to
// complete any necessary steps after the MAC and PCS have been configured
// for the @mode and @iface. E.g. a MAC driver may wish to re-enable the
// Serdes PHY here if it was previously disabled by mac_prepare().
//
// Returns zero on success, or negative errno on failure which will be
// reported to the kernel log.
//
// mac_link_down() - notification that the link has gone down
// @config: a pointer to a &struct phylink_config.
// @mode: link autonegotiation mode
// @interface: link &typedef phy_interface_t mode
//
// Notifies the MAC that the link has gone down. This will not be called
// unless mac_link_up() has been previously called.
//
// The MAC should stop processing packets for transmission and reception.
// phylink will have called netif_carrier_off() to notify the networking
// stack that the link has gone down, so MAC drivers should not make this
// call.
//
// If @mode is %MLO_AN_INBAND, then this function must not prevent the
// link coming up.
//
// mac_link_up() - notification that the link has come up
// @config: a pointer to a &struct phylink_config.
// @phy: any attached phy (deprecated - please use LPI interfaces)
// @mode: link autonegotiation mode
// @interface: link &typedef phy_interface_t mode
// @speed: link speed
// @duplex: link duplex
// @tx_pause: link transmit pause enablement status
// @rx_pause: link receive pause enablement status
//
// Notifies the MAC that the link has come up, and the parameters of the
// link as seen from the MACs point of view. If mac_link_up() has been
// called previously, there will be an intervening call to mac_link_down()
// before this method will be subsequently called.
//
// @speed, @duplex, @tx_pause and @rx_pause indicate the finalised link
// settings, and should be used to configure the MAC block appropriately
// where these settings are not automatically conveyed from the PCS block,
// or if in-band negotiation (as defined by phylink_autoneg_inband(@mode))
// is disabled.
//
// Note that when 802.3z in-band negotiation is in use, it is possible
// that the user wishes to override the pause settings, and this should
// be allowed when considering the implementation of this method.
//
// Once configured, the MAC may begin to process packets for transmission
// and reception.
//
// Interface type selection must be done in mac_config().
//
// mac_disable_tx_lpi() - disable LPI generation at the MAC
// @config: a pointer to a &struct phylink_config.
//
// Disable generation of LPI at the MAC, effectively preventing the MAC
// from indicating that it is idle.
//
extern "C" {
    pub fn mac_disable_tx_lpi(config: *mut phylink_config);
}
//
// mac_enable_tx_lpi() - configure and enable LPI generation at the MAC
// @config: a pointer to a &struct phylink_config.
// @timer: LPI timeout in microseconds.
// @tx_clk_stop: allow xMII transmit clock to be stopped during LPI
//
// Configure the LPI timeout accordingly. This will only be called when
// the link is already up, to cater for situations where the hardware
// needs to be programmed according to the link speed.
//
// Enable LPI generation at the MAC, and configure whether the xMII transmit
// clock may be stopped.
//
// Returns: 0 on success. Please consult with rmk before returning an error.
//
// mac_wol_set() - configure the Wake-on-Lan parameters
// @config: a pointer to a &struct phylink_config.
// @wolopts: Bitmask of %WAKE_* flags for enabled Wake-On-Lan modes.
// @sopass: SecureOn(tm) password; meaningful only for %WAKE_MAGICSECURE
//
// Enable the specified Wake-on-Lan options at the MAC. Options that the
// PHY can handle will have been removed from @wolopts.
//
// The presence of this method enables phylink-managed WoL support.
//
// Returns: 0 on success.
//

//
// struct phylink_pcs - PHYLINK PCS instance
// @supported_interfaces: describing which PHY_INTERFACE_MODE_xxx
// are supported by this PCS.
// @ops: a pointer to the &struct phylink_pcs_ops structure
// @phylink: pointer to &struct phylink_config
// @poll: poll the PCS for link changes
// @rxc_always_on: The MAC driver requires the reference clock
// to always be on. Standalone PCS drivers which
// do not have access to a PHY device can check
// this instead of PHY_F_RXC_ALWAYS_ON.
//
// This structure is designed to be embedded within the PCS private data,
// and will be passed between phylink and the PCS.
//
// The @phylink member is private to phylink and must not be touched by
// the PCS driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phylink_pcs {
    pub ops: *const phylink_pcs_ops,
    pub phylink: *mut phylink,
    pub poll: bool,
    pub rxc_always_on: bool,
}

//
// struct phylink_pcs_ops - MAC PCS operations structure.
// @pcs_validate: validate the link configuration.
// @pcs_inband_caps: query inband support for interface mode.
// @pcs_enable: enable the PCS.
// @pcs_disable: disable the PCS.
// @pcs_pre_config: pre-mac_config method (for errata)
// @pcs_post_config: post-mac_config method (for arrata)
// @pcs_get_state: read the current MAC PCS link state from the hardware.
// @pcs_config: configure the MAC PCS for the selected mode and state.
// @pcs_an_restart: restart 802.3z BaseX autonegotiation.
// @pcs_link_up: program the PCS for the resolved link configuration
// (where necessary).
// @pcs_disable_eee: optional notification to PCS that EEE has been disabled
// at the MAC.
// @pcs_enable_eee: optional notification to PCS that EEE will be enabled at
// the MAC.
// @pcs_pre_init: configure PCS components necessary for MAC hardware
// initialization e.g. RX clock for stmmac.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phylink_pcs_ops {
    pub state): *const phylink_link_state,
    pub interface): phy_interface_t,
    pub pcs): *mut *mut int (pcs_enable)(struct phylink_pcs,
    pub pcs): *mut *mut void (pcs_disable)(struct phylink_pcs,
    pub interface): phy_interface_t,
    pub interface): phy_interface_t,
    pub state): *mut phylink_link_state,
    pub permit_pause_to_mac): bool,
    pub pcs): *mut *mut void (pcs_an_restart)(struct phylink_pcs,
    pub duplex): phy_interface_t interface, int speed, int,
    pub pcs): *mut *mut void (pcs_disable_eee)(struct phylink_pcs,
    pub pcs): *mut *mut void (pcs_enable_eee)(struct phylink_pcs,
    pub pcs): *mut *mut int (pcs_pre_init)(struct phylink_pcs,
}

//
// pcs_validate() - validate the link configuration.
// @pcs: a pointer to a &struct phylink_pcs.
// @supported: ethtool bitmask for supported link modes.
// @state: a const pointer to a &struct phylink_link_state.
//
// Validate the interface mode, and advertising's autoneg bit, removing any
// media ethtool link modes that would not be supportable from the supported
// mask. Phylink will propagate the changes to the advertising mask. See the
// &struct phylink_mac_ops validate() method.
//
// Returns -EINVAL if the interface mode/autoneg mode is not supported.
// Returns non-zero positive if the link state can be supported.
//
// pcs_inband_caps - query PCS in-band capabilities for interface mode.
// @pcs: a pointer to a &struct phylink_pcs.
// @interface: interface mode to be queried
//
// Returns zero if it is unknown what in-band signalling is supported by the
// PHY (e.g. because the PHY driver doesn't implement the method.) Otherwise,
// returns a bit mask of the LINK_INBAND_* values from
// &enum link_inband_signalling to describe which inband modes are supported
// for this interface mode.
//
// pcs_enable() - enable the PCS.
// @pcs: a pointer to a &struct phylink_pcs.
//
extern "C" {
    pub fn pcs_enable(pcs: *mut phylink_pcs) -> c_int;
}
//
// pcs_disable() - disable the PCS.
// @pcs: a pointer to a &struct phylink_pcs.
//
extern "C" {
    pub fn pcs_disable(pcs: *mut phylink_pcs);
}
//
// pcs_get_state() - Read the current inband link state from the hardware
// @pcs: a pointer to a &struct phylink_pcs.
// @neg_mode: link negotiation mode (PHYLINK_PCS_NEG_xxx)
// @state: a pointer to a &struct phylink_link_state.
//
// Read the current inband link state from the MAC PCS, reporting the
// current speed in @state->speed, duplex mode in @state->duplex, pause
// mode in @state->pause using the %MLO_PAUSE_RX and %MLO_PAUSE_TX bits,
// negotiation completion state in @state->an_complete, and link up state
// in @state->link. If possible, @state->lp_advertising should also be
// populated.
//
// Note that the @neg_mode parameter is always the PHYLINK_PCS_NEG_xxx
// state, not MLO_AN_xxx.
//
// pcs_config() - Configure the PCS mode and advertisement
// @pcs: a pointer to a &struct phylink_pcs.
// @neg_mode: link negotiation mode (see below)
// @interface: interface mode to be used
// @advertising: adertisement ethtool link mode mask
// @permit_pause_to_mac: permit forwarding pause resolution to MAC
//
// Configure the PCS for the operating mode, the interface mode, and set
// the advertisement mask. @permit_pause_to_mac indicates whether the
// hardware may forward the pause mode resolution to the MAC.
//
// When operating in %MLO_AN_INBAND, inband should always be enabled,
// otherwise inband should be disabled.
//
// For SGMII, there is no advertisement from the MAC side, the PCS should
// be programmed to acknowledge the inband word from the PHY.
//
// For 1000BASE-X, the advertisement should be programmed into the PCS.
//
// For most 10GBASE-R, there is no advertisement.
//
// The %neg_mode argument should be tested via the phylink_mode_*() family of
// functions, or for PCS that set pcs->neg_mode true, should be tested
// against the PHYLINK_PCS_NEG_* definitions.
//
// pcs_config() will be called when configuration of the PCS is required
// or when the advertisement is possibly updated. It must not unnecessarily
// disrupt an established link.
//
// When an autonegotiation restart is required for 802.3z modes, .pcs_config()
// should return a positive non-zero integer (e.g. 1) to indicate to phylink
// to call the pcs_an_restart() method.
//
// pcs_an_restart() - restart 802.3z BaseX autonegotiation
// @pcs: a pointer to a &struct phylink_pcs.
//
// When PCS ops are present, this overrides mac_an_restart() in &struct
// phylink_mac_ops.
//
extern "C" {
    pub fn pcs_an_restart(pcs: *mut phylink_pcs);
}
//
// pcs_link_up() - program the PCS for the resolved link configuration
// @pcs: a pointer to a &struct phylink_pcs.
// @neg_mode: link negotiation mode (see below)
// @interface: link &typedef phy_interface_t mode
// @speed: link speed
// @duplex: link duplex
//
// This call will be made just before mac_link_up() to inform the PCS of
// the resolved link parameters. For example, a PCS operating in SGMII
// mode without in-band AN needs to be manually configured for the link
// and duplex setting. Otherwise, this should be a no-op.
//
// The %mode argument should be tested via the phylink_mode_*() family of
// functions, or for PCS that set pcs->neg_mode true, should be tested
// against the PHYLINK_PCS_NEG_* definitions.
//
// pcs_disable_eee() - Disable EEE at the PCS
// @pcs: a pointer to a &struct phylink_pcs
//
// Optional method informing the PCS that EEE has been disabled at the MAC.
//
extern "C" {
    pub fn pcs_disable_eee(pcs: *mut phylink_pcs);
}
//
// pcs_enable_eee() - Enable EEE at the PCS
// @pcs: a pointer to a &struct phylink_pcs
//
// Optional method informing the PCS that EEE is about to be enabled at the MAC.
//
extern "C" {
    pub fn pcs_enable_eee(pcs: *mut phylink_pcs);
}
//
// pcs_pre_init() - Configure PCS components necessary for MAC initialization
// @pcs: a pointer to a &struct phylink_pcs.
//
// This function can be called by MAC drivers through the
// phylink_pcs_pre_init() wrapper, before their hardware is initialized. It
// should not be called after the link is brought up, as reconfiguring the PCS
// at this point could break the link.
//
// Some MAC devices require specific hardware initialization to be performed by
// their associated PCS device before they can properly initialize their own
// hardware. An example of this is the initialization of stmmac controllers,
// which requires an active REF_CLK signal to be provided by the PHY/PCS.
//
// By calling phylink_pcs_pre_init(), MAC drivers can ensure that the PCS is
// setup in a way that allows for successful hardware initialization.
//
// The specific configuration performed by pcs_pre_init() is dependent on the
// model of PCS and the requirements of the MAC device attached to it. PCS
// driver authors should consider whether their target device is to be used in
// conjunction with a MAC device whose driver calls phylink_pcs_pre_init(). MAC
// driver authors should document their requirements for the PCS
// pre-initialization.
//
extern "C" {
    pub fn pcs_pre_init(pcs: *mut phylink_pcs) -> c_int;
}

extern "C" {
    pub fn phylink_destroy(: *mut phylink);
}
extern "C" {
    pub fn phylink_expects_phy(pl: *mut phylink) -> bool;
}
extern "C" {
    pub fn phylink_connect_phy(: *mut phylink, : *mut phy_device) -> c_int;
}
extern "C" {
    pub fn phylink_of_phy_connect(: *mut phylink, : *mut device_node, flags: u32) -> c_int;
}
extern "C" {
    pub fn phylink_disconnect_phy(: *mut phylink);
}
extern "C" {
    pub fn phylink_mac_change(: *mut phylink, up: bool);
}
extern "C" {
    pub fn phylink_pcs_change(: *mut phylink_pcs, up: bool);
}
extern "C" {
    pub fn phylink_pcs_pre_init(pl: *mut phylink, pcs: *mut phylink_pcs) -> c_int;
}
extern "C" {
    pub fn phylink_start(: *mut phylink);
}
extern "C" {
    pub fn phylink_stop(: *mut phylink);
}
extern "C" {
    pub fn phylink_rx_clk_stop_block(: *mut phylink);
}
extern "C" {
    pub fn phylink_rx_clk_stop_unblock(: *mut phylink);
}
extern "C" {
    pub fn phylink_suspend(pl: *mut phylink, mac_wol: bool);
}
extern "C" {
    pub fn phylink_prepare_resume(pl: *mut phylink);
}
extern "C" {
    pub fn phylink_resume(pl: *mut phylink);
}
extern "C" {
    pub fn phylink_ethtool_get_wol(: *mut phylink, : *mut ethtool_wolinfo);
}
extern "C" {
    pub fn phylink_ethtool_set_wol(: *mut phylink, : *mut ethtool_wolinfo) -> c_int;
}
extern "C" {
    pub fn phylink_ethtool_nway_reset(: *mut phylink) -> c_int;
}
extern "C" {
    pub fn phylink_get_eee_err(: *mut phylink) -> c_int;
}
extern "C" {
    pub fn phylink_ethtool_get_eee(link: *mut phylink, eee: *mut ethtool_keee) -> c_int;
}
extern "C" {
    pub fn phylink_ethtool_set_eee(link: *mut phylink, eee: *mut ethtool_keee) -> c_int;
}
extern "C" {
    pub fn phylink_mii_ioctl(: *mut phylink, : *mut ifreq, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn phylink_speed_down(pl: *mut phylink, sync: bool) -> c_int;
}
extern "C" {
    pub fn phylink_speed_up(pl: *mut phylink) -> c_int;
}

extern "C" {
    pub fn phylink_set_port_modes(bits: *mut c_ulong);
}
//
// phylink_get_link_timer_ns - return the PCS link timer value
// @interface: link &typedef phy_interface_t mode
//
// Return the PCS link timer setting in nanoseconds for the PHY @interface
// mode, or -EINVAL if not appropriate.
//
// phylink_mac_implements_lpi() - determine if MAC implements LPI ops
// @ops: phylink_mac_ops structure
//
// Returns true if the phylink MAC operations structure indicates that the
// LPI operations have been implemented, false otherwise.
//
extern "C" {
    pub fn phylink_mii_c22_pcs_an_restart(pcs: *mut mdio_device);
}
extern "C" {
    pub fn phylink_resolve_c73(state: *mut phylink_link_state);
}
extern "C" {
    pub fn phylink_replay_link_begin(pl: *mut phylink);
}
extern "C" {
    pub fn phylink_replay_link_end(pl: *mut phylink);
}
