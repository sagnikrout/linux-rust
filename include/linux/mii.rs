//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mii.h
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
//
// linux/mii.h: definitions for MII-compatible transceivers
// Originally drivers/net/sunhme.h.
//
// Copyright (C) 1996, 1999, 2001 David S. Miller (davem@redhat.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mii_if_info {
    pub phy_id: c_int,
    pub advertising: c_int,
    pub phy_id_mask: c_int,
    pub reg_num_mask: c_int,
    pub /: *mut *mut unsigned int full_duplex : 1; / is full duplex?,
    pub /: *mut *mut unsigned int force_media : 1; / is autoneg. disabled?,
    pub /: *mut *mut unsigned int supports_gmii : 1; / are GMII registers supported?,
    pub dev: *mut net_device,
    pub location): *mut *mut *mut int (mdio_read) (struct net_device dev, int phy_id, int,
    pub val): *mut *mut *mut void (mdio_write) (struct net_device dev, int phy_id, int location, int,
}

extern "C" {
    pub fn mii_link_ok(mii: *mut mii_if_info) -> c_int;
}
extern "C" {
    pub fn mii_nway_restart(mii: *mut mii_if_info) -> c_int;
}
extern "C" {
    pub fn mii_ethtool_gset(mii: *mut mii_if_info, ecmd: *mut ethtool_cmd);
}
extern "C" {
    pub fn mii_ethtool_sset(mii: *mut mii_if_info, ecmd: *mut ethtool_cmd) -> c_int;
}
extern "C" {
    pub fn mii_check_gmii_support(mii: *mut mii_if_info) -> c_int;
}
extern "C" {
    pub fn mii_check_link(mii: *mut mii_if_info);
}
//
// mii_nway_result
// @negotiated: value of MII ANAR and'd with ANLPAR
//
// Given a set of MII abilities, check each bit and returns the
// currently supported media, in the priority order defined by
// IEEE 802.3u.  We use LPA_xxx constants but note this is not the
// value of LPA solely, as described above.
//
// The one exception to IEEE 802.3u is that 100baseT4 is placed
// between 100T-full and 100T-half.  If your phy does not support
// 100T4 this is fine.  If your phy places 100T4 elsewhere in the
// priority order, you will need to roll your own function.
//
// mii_duplex
// @duplex_lock: Non-zero if duplex is locked at full
// @negotiated: value of MII ANAR and'd with ANLPAR
//
// A small helper function for a common case.  Returns one
// if the media is operating or locked at full duplex, and
// returns zero otherwise.
//
// ethtool_adv_to_mii_adv_t
// @ethadv: the ethtool advertisement settings
//
// A small helper function that translates ethtool advertisement
// settings to phy autonegotiation advertisements for the
// MII_ADVERTISE register.
//
// linkmode_adv_to_mii_adv_t
// @advertising: the linkmode advertisement settings
//
// A small helper function that translates linkmode advertisement
// settings to phy autonegotiation advertisements for the
// MII_ADVERTISE register.
//
// mii_adv_to_ethtool_adv_t
// @adv: value of the MII_ADVERTISE register
//
// A small helper function that translates MII_ADVERTISE bits
// to ethtool advertisement settings.
//
// ethtool_adv_to_mii_ctrl1000_t
// @ethadv: the ethtool advertisement settings
//
// A small helper function that translates ethtool advertisement
// settings to phy autonegotiation advertisements for the
// MII_CTRL1000 register when in 1000T mode.
//
// linkmode_adv_to_mii_ctrl1000_t
// @advertising: the linkmode advertisement settings
//
// A small helper function that translates linkmode advertisement
// settings to phy autonegotiation advertisements for the
// MII_CTRL1000 register when in 1000T mode.
//
// mii_ctrl1000_to_ethtool_adv_t
// @adv: value of the MII_CTRL1000 register
//
// A small helper function that translates MII_CTRL1000
// bits, when in 1000Base-T mode, to ethtool
// advertisement settings.
//
// mii_lpa_to_ethtool_lpa_t
// @adv: value of the MII_LPA register
//
// A small helper function that translates MII_LPA
// bits, when in 1000Base-T mode, to ethtool
// LP advertisement settings.
//
// mii_stat1000_to_ethtool_lpa_t
// @adv: value of the MII_STAT1000 register
//
// A small helper function that translates MII_STAT1000
// bits, when in 1000Base-T mode, to ethtool
// advertisement settings.
//
// mii_stat1000_mod_linkmode_lpa_t
// @advertising: target the linkmode advertisement settings
// @adv: value of the MII_STAT1000 register
//
// A small helper function that translates MII_STAT1000 bits, when in
// 1000Base-T mode, to linkmode advertisement settings. Other bits in
// advertising are not changes.
//
// ethtool_adv_to_mii_adv_x
// @ethadv: the ethtool advertisement settings
//
// A small helper function that translates ethtool advertisement
// settings to phy autonegotiation advertisements for the
// MII_CTRL1000 register when in 1000Base-X mode.
//
// mii_adv_to_ethtool_adv_x
// @adv: value of the MII_CTRL1000 register
//
// A small helper function that translates MII_CTRL1000
// bits, when in 1000Base-X mode, to ethtool
// advertisement settings.
//
// mii_adv_mod_linkmode_adv_t
// @advertising:pointer to destination link mode.
// @adv: value of the MII_ADVERTISE register
//
// A small helper function that translates MII_ADVERTISE bits to
// linkmode advertisement settings. Leaves other bits unchanged.
//
// mii_adv_to_linkmode_adv_t
// @advertising:pointer to destination link mode.
// @adv: value of the MII_ADVERTISE register
//
// A small helper function that translates MII_ADVERTISE bits
// to linkmode advertisement settings. Clears the old value
// of advertising.
//
// mii_lpa_to_linkmode_lpa_t
// @adv: value of the MII_LPA register
//
// A small helper function that translates MII_LPA bits, when in
// 1000Base-T mode, to linkmode LP advertisement settings. Clears the
// old value of advertising
//
// mii_lpa_mod_linkmode_lpa_t
// @adv: value of the MII_LPA register
//
// A small helper function that translates MII_LPA bits, when in
// 1000Base-T mode, to linkmode LP advertisement settings. Leaves
// other bits unchanged.
//
// linkmode_adv_to_lcl_adv_t
// @advertising:pointer to linkmode advertising
//
// A small helper function that translates linkmode advertising to LVL
// pause capabilities.
//
// mii_lpa_mod_linkmode_x - decode the link partner's config_reg to linkmodes
// @linkmodes: link modes array
// @lpa: config_reg word from link partner
// @fd_bit: link mode for 1000XFULL bit
//
// linkmode_adv_to_mii_adv_x - encode a linkmode to config_reg
// @linkmodes: linkmodes
// @fd_bit: full duplex bit
//
// mii_advertise_flowctrl - get flow control advertisement flags
// @cap: Flow control capabilities (FLOW_CTRL_RX, FLOW_CTRL_TX or both)
//
// mii_resolve_flowctrl_fdx
// @lcladv: value of MII ADVERTISE register
// @rmtadv: value of MII LPA register
//
// Resolve full duplex flow control as per IEEE 802.3-2005 table 28B-3
//
// mii_bmcr_encode_fixed - encode fixed speed/duplex settings to a BMCR value
// @speed: a SPEED_* value
// @duplex: a DUPLEX_* value
//
// Encode the speed and duplex to a BMCR value. 2500, 1000, 100 and 10 Mbps are
// supported. 2500Mbps is encoded to 1000Mbps. Other speeds are encoded as 10
// Mbps. Unknown duplex values are encoded to half-duplex.
//
