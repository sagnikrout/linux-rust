//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/realtek/rtl8365mb_main.c
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
// Realtek SMI subdriver for the Realtek RTL8365MB-VC ethernet switch.
//
// Copyright (C) 2021 Alvin Šipraga <alsi@bang-olufsen.dk>
// Copyright (C) 2021 Michael Rasmussen <mir@bang-olufsen.dk>
//
// The RTL8365MB-VC is a 4+1 port 10/100/1000M switch controller. It includes 4
// integrated PHYs for the user facing ports, and an extension interface which
// can be connected to the CPU - or another PHY - via either MII, RMII, or
// RGMII. The switch is configured via the Realtek Simple Management Interface
// (SMI), which uses the MDIO/MDC lines.
//
// Below is a simplified block diagram of the chip and its relevant interfaces.
//
// .-----------------------------------.
// |                                   |
// UTP <---------------> Giga PHY <-> PCS <-> P0 GMAC   |
// UTP <---------------> Giga PHY <-> PCS <-> P1 GMAC   |
// UTP <---------------> Giga PHY <-> PCS <-> P2 GMAC   |
// UTP <---------------> Giga PHY <-> PCS <-> P3 GMAC   |
// |                                   |
// CPU/PHY <-MII/RMII/RGMII--->  Extension  <---> Extension |
// |       interface 1        GMAC 1   |
// |                                   |
// SMI driver/ <-MDC/SCL---> Management    ~~~~~~~~~~~~~~   |
// EEPROM   <-MDIO/SDA--> interface     ~REALTEK ~~~~~   |
// |                  ~RTL8365MB ~~~   |
// |                  ~GXXXC TAIWAN~   |
// GPIO <--------------> Reset          ~~~~~~~~~~~~~~   |
// |                                   |
// Interrupt  <----------> Link UP/DOWN events             |
// controller          |                                   |
// '-----------------------------------'
//
// The driver uses DSA to integrate the 4 user and 1 extension ports into the
// kernel. Netdevices are created for the user ports, as are PHY devices for
// their integrated PHYs. The device tree firmware should also specify the link
// partner of the extension port - either via a fixed-link or other phy-handle.
// See the device tree bindings for more detailed information. Note that the
// driver has only been tested with a fixed-link, but in principle it should not
// matter.
//
// NOTE: Currently, only the RGMII, SGMII and HSGMII interfaces are implemented
// in this driver.
//
// The interrupt line is asserted on link UP/DOWN events. The driver creates a
// custom irqchip to handle this interrupt and demultiplex the events by reading
// the status registers via SMI. Interrupts are then propagated to the relevant
// PHY device.
//
// The EEPROM contains initial register values which the chip will read over I2C
// upon hardware reset. It is also possible to omit the EEPROM. In both cases,
// the driver will manually reprogram some registers using jam tables to reach
// an initial state defined by the vendor driver.
//
// This Linux driver is written based on an OS-agnostic vendor driver from
// Realtek. The reference GPL-licensed sources can be found in the OpenWrt
// source tree under the name rtl8367c. The vendor driver claims to support a
// number of similar switch controllers from Realtek, but the only hardware we
// have is the RTL8365MB-VC. Moreover, there does not seem to be any chip under
// the name RTL8367C. Although one wishes that the 'C' stood for some kind of
// common hardware revision, there exist examples of chips with the suffix -VC
// which are explicitly not supported by the rtl8367c driver and which instead
// require the rtl8367d vendor driver. With all this uncertainty, the driver has
// been modestly named rtl8365mb. Future implementors may wish to rename things
// accordingly.
//
// In the same family of chips, some carry up to 8 user ports and up to 2
// extension ports. Where possible this driver tries to make things generic, but
// more work must be done to support these configurations. According to
// documentation from Realtek, the family should include the following chips:
//
// - RTL8363NB
// - RTL8363NB-VB
// - RTL8363SC
// - RTL8363SC-VB
// - RTL8364NB
// - RTL8364NB-VB
// - RTL8365MB-VC
// - RTL8366SC
// - RTL8367RB-VB
// - RTL8367SB
// - RTL8367S
// - RTL8370MB
// - RTL8310SR
//
// Some of the register logic for these additional chips has been skipped over
// while implementing this driver. It is therefore not possible to assume that
// things will work out-of-the-box for other chips, and a careful review of the
// vendor driver may be needed to expand support. The RTL8365MB-VC seems to be
// one of the simpler chips.
//

// Family-specific data and limits
pub const RTL8365MB_PHYADDRMAX: c_int = 7;
pub const RTL8365MB_NUM_PHYREGS: c_int = 32;

pub const RTL8365MB_MAX_NUM_PORTS: c_int = 11;
// Valid for the whole family except RTL8370B, which has 4160 entries.
// RTL8370B is mentioned in vendor code but it might not even belong
// to the same RTL8367C family.
//
pub const RTL8365MB_LEARN_LIMIT_MAX: c_int = 2112;
pub const RTL8365MB_MAX_NUM_EXTINTS: c_int = 3;
// Chip identification registers
pub const RTL8365MB_CHIP_ID_REG: c_uint = 0x1300;
pub const RTL8365MB_CHIP_VER_REG: c_uint = 0x1301;
pub const RTL8365MB_MAGIC_REG: c_uint = 0x13C2;
pub const RTL8365MB_MAGIC_VALUE: c_uint = 0x0249;
// Chip reset register
pub const RTL8365MB_CHIP_RESET_REG: c_uint = 0x1322;
pub const RTL8365MB_CHIP_RESET_DW8051_MASK: c_uint = 0x0010;
pub const RTL8365MB_CHIP_RESET_SW_MASK: c_uint = 0x0002;
pub const RTL8365MB_CHIP_RESET_HW_MASK: c_uint = 0x0001;
// Interrupt polarity register
pub const RTL8365MB_INTR_POLARITY_REG: c_uint = 0x1100;
pub const RTL8365MB_INTR_POLARITY_MASK: c_uint = 0x0001;
pub const RTL8365MB_INTR_POLARITY_HIGH: c_int = 0;
pub const RTL8365MB_INTR_POLARITY_LOW: c_int = 1;
// Interrupt control/status register - enable/check specific interrupt types
pub const RTL8365MB_INTR_CTRL_REG: c_uint = 0x1101;
pub const RTL8365MB_INTR_STATUS_REG: c_uint = 0x1102;
pub const RTL8365MB_INTR_SLIENT_START_2_MASK: c_uint = 0x1000;
pub const RTL8365MB_INTR_SLIENT_START_MASK: c_uint = 0x0800;
pub const RTL8365MB_INTR_ACL_ACTION_MASK: c_uint = 0x0200;
pub const RTL8365MB_INTR_CABLE_DIAG_FIN_MASK: c_uint = 0x0100;
pub const RTL8365MB_INTR_INTERRUPT_8051_MASK: c_uint = 0x0080;
pub const RTL8365MB_INTR_LOOP_DETECTION_MASK: c_uint = 0x0040;
pub const RTL8365MB_INTR_GREEN_TIMER_MASK: c_uint = 0x0020;
pub const RTL8365MB_INTR_SPECIAL_CONGEST_MASK: c_uint = 0x0010;
pub const RTL8365MB_INTR_SPEED_CHANGE_MASK: c_uint = 0x0008;
pub const RTL8365MB_INTR_LEARN_OVER_MASK: c_uint = 0x0004;
pub const RTL8365MB_INTR_METER_EXCEEDED_MASK: c_uint = 0x0002;
pub const RTL8365MB_INTR_LINK_CHANGE_MASK: c_uint = 0x0001;

    (RTL8365MB_INTR_SLIENT_START_2_MASK |  \
    RTL8365MB_INTR_SLIENT_START_MASK |    \
    RTL8365MB_INTR_ACL_ACTION_MASK |      \
    RTL8365MB_INTR_CABLE_DIAG_FIN_MASK |  \
    RTL8365MB_INTR_INTERRUPT_8051_MASK |  \
    RTL8365MB_INTR_LOOP_DETECTION_MASK |  \
    RTL8365MB_INTR_GREEN_TIMER_MASK |     \
    RTL8365MB_INTR_SPECIAL_CONGEST_MASK | \
    RTL8365MB_INTR_SPEED_CHANGE_MASK |    \
    RTL8365MB_INTR_LEARN_OVER_MASK |      \
    RTL8365MB_INTR_METER_EXCEEDED_MASK |  \
    RTL8365MB_INTR_LINK_CHANGE_MASK)
// Per-port interrupt type status registers
pub const RTL8365MB_PORT_LINKDOWN_IND_REG: c_uint = 0x1106;
pub const RTL8365MB_PORT_LINKDOWN_IND_MASK: c_uint = 0x07FF;
pub const RTL8365MB_PORT_LINKUP_IND_REG: c_uint = 0x1107;
pub const RTL8365MB_PORT_LINKUP_IND_MASK: c_uint = 0x07FF;
// PHY indirect access registers
pub const RTL8365MB_INDIRECT_ACCESS_CTRL_REG: c_uint = 0x1F00;
pub const RTL8365MB_INDIRECT_ACCESS_CTRL_RW_MASK: c_uint = 0x0002;
pub const RTL8365MB_INDIRECT_ACCESS_CTRL_RW_READ: c_int = 0;
pub const RTL8365MB_INDIRECT_ACCESS_CTRL_RW_WRITE: c_int = 1;
pub const RTL8365MB_INDIRECT_ACCESS_CTRL_CMD_MASK: c_uint = 0x0001;
pub const RTL8365MB_INDIRECT_ACCESS_CTRL_CMD_VALUE: c_int = 1;
pub const RTL8365MB_INDIRECT_ACCESS_STATUS_REG: c_uint = 0x1F01;
pub const RTL8365MB_INDIRECT_ACCESS_ADDRESS_REG: c_uint = 0x1F02;

pub const RTL8365MB_PHY_BASE: c_uint = 0x2000;
pub const RTL8365MB_INDIRECT_ACCESS_WRITE_DATA_REG: c_uint = 0x1F03;
pub const RTL8365MB_INDIRECT_ACCESS_READ_DATA_REG: c_uint = 0x1F04;
// PHY OCP address prefix register
pub const RTL8365MB_GPHY_OCP_MSB_0_REG: c_uint = 0x1D15;
pub const RTL8365MB_GPHY_OCP_MSB_0_CFG_CPU_OCPADR_MASK: c_uint = 0x0FC0;
pub const RTL8365MB_PHY_OCP_ADDR_PREFIX_MASK: c_uint = 0xFC00;
// The PHY OCP addresses of PHY registers 0~31 start here
pub const RTL8365MB_PHY_OCP_ADDR_PHYREG_BASE: c_uint = 0xA400;
// External interface port mode values - used in DIGITAL_INTERFACE_SELECT
pub const RTL8365MB_EXT_PORT_MODE_DISABLE: c_int = 0;
pub const RTL8365MB_EXT_PORT_MODE_RGMII: c_int = 1;
pub const RTL8365MB_EXT_PORT_MODE_MII_MAC: c_int = 2;
pub const RTL8365MB_EXT_PORT_MODE_MII_PHY: c_int = 3;
pub const RTL8365MB_EXT_PORT_MODE_TMII_MAC: c_int = 4;
pub const RTL8365MB_EXT_PORT_MODE_TMII_PHY: c_int = 5;
pub const RTL8365MB_EXT_PORT_MODE_GMII: c_int = 6;
pub const RTL8365MB_EXT_PORT_MODE_RMII_MAC: c_int = 7;
pub const RTL8365MB_EXT_PORT_MODE_RMII_PHY: c_int = 8;
pub const RTL8365MB_EXT_PORT_MODE_SGMII: c_int = 9;
pub const RTL8365MB_EXT_PORT_MODE_HSGMII: c_int = 10;
pub const RTL8365MB_EXT_PORT_MODE_1000X_100FX: c_int = 11;
pub const RTL8365MB_EXT_PORT_MODE_1000X: c_int = 12;
pub const RTL8365MB_EXT_PORT_MODE_100FX: c_int = 13;
// External interface mode configuration registers 0~1
pub const RTL8365MB_DIGITAL_INTERFACE_SELECT_REG0: c_uint = 0x1305 /* EXT0,EXT1 */;
pub const RTL8365MB_DIGITAL_INTERFACE_SELECT_REG1: c_uint = 0x13C3 /* EXT2 */;

    ((_extint) <= 1 ? RTL8365MB_DIGITAL_INTERFACE_SELECT_REG0 : \
    (_extint) == 2 ? RTL8365MB_DIGITAL_INTERFACE_SELECT_REG1 : \
    0x0)

    (0xF << (((_extint) % 2) * 4))

    (((_extint) % 2) * 4)
// External interface RGMII TX/RX delay configuration registers 0~2
pub const RTL8365MB_EXT_RGMXF_REG0: c_uint = 0x1306 /* EXT0 */;
pub const RTL8365MB_EXT_RGMXF_REG1: c_uint = 0x1307 /* EXT1 */;
pub const RTL8365MB_EXT_RGMXF_REG2: c_uint = 0x13C5 /* EXT2 */;

    ((_extint) == 0 ? RTL8365MB_EXT_RGMXF_REG0 : \
    (_extint) == 1 ? RTL8365MB_EXT_RGMXF_REG1 : \
    (_extint) == 2 ? RTL8365MB_EXT_RGMXF_REG2 : \
    0x0)
pub const RTL8365MB_EXT_RGMXF_RXDELAY_MASK: c_uint = 0x0007;
pub const RTL8365MB_EXT_RGMXF_TXDELAY_MASK: c_uint = 0x0008;
// External interface line rate bypass register - one bit per external
// interface, indexed by the external port number with port 5 (the first
// external port) as the base. Other RTL8367 families index this register
// differently (e.g. the RTL8367R uses (id + 1) % 2), so this mapping only
// holds for the RTL8367C-style parts this driver supports.
//
pub const RTL8365MB_BYPASS_LINE_RATE_REG: c_uint = 0x03F7;

// Port 6 ingress and egress rate limiter registers. Each limit is a 19-bit
// value in units of 8 Kbps, split across a 16-bit LSB register (CTRL0) and a
// 3-bit MSB field (CTRL1). The chip resets them to 0x1FFFF; see
// rtl8365mb_sds_raise_rate_limits().
//
pub const RTL8365MB_INGRESSBW_PORT6_RATE_CTRL0_REG: c_uint = 0x00CF;
pub const RTL8365MB_INGRESSBW_PORT6_RATE_CTRL1_REG: c_uint = 0x00D0;
pub const RTL8365MB_INGRESSBW_PORT6_RATE_CTRL1_MASK: c_uint = 0x0007;
pub const RTL8365MB_PORT6_EGRESSBW_CTRL0_REG: c_uint = 0x0398;
pub const RTL8365MB_PORT6_EGRESSBW_CTRL1_REG: c_uint = 0x0399;
pub const RTL8365MB_PORT6_EGRESSBW_CTRL1_MASK: c_uint = 0x0007;
// SerDes indirect access registers
pub const RTL8365MB_SDS_INDACS_CMD_REG: c_uint = 0x6600;
pub const RTL8365MB_SDS_INDACS_CMD_BUSY_MASK: c_uint = 0x0100;
pub const RTL8365MB_SDS_INDACS_CMD_RUN_MASK: c_uint = 0x0080;
pub const RTL8365MB_SDS_INDACS_CMD_WR_MASK: c_uint = 0x0040;
pub const RTL8365MB_SDS_INDACS_ADR_REG: c_uint = 0x6601;
pub const RTL8365MB_SDS_INDACS_DATA_REG: c_uint = 0x6602;
// SerDes miscellaneous configuration register
pub const RTL8365MB_SDS_MISC_REG: c_uint = 0x1D11;
pub const RTL8365MB_SDS_MISC_SGMII_RXFC_MASK: c_uint = 0x4000;
pub const RTL8365MB_SDS_MISC_SGMII_TXFC_MASK: c_uint = 0x2000;
pub const RTL8365MB_SDS_MISC_MAC8_SEL_HSGMII_MASK: c_uint = 0x0800;
pub const RTL8365MB_SDS_MISC_SGMII_FDUP_MASK: c_uint = 0x0400;
pub const RTL8365MB_SDS_MISC_SGMII_LINK_MASK: c_uint = 0x0200;
pub const RTL8365MB_SDS_MISC_SGMII_SPD_MASK: c_uint = 0x0180;
pub const RTL8365MB_SDS_MISC_MAC8_SEL_SGMII_MASK: c_uint = 0x0040;
// SerDes internal registers, accessed via the SDS_INDACS registers. The BMCR
// data path reset holds BMCR_ANENABLE | BMCR_ISOLATE while toggling the
// vendor-specific low bits from phase 1 to phase 2, which triggers a data path
// reset and PLL resync.
//
pub const RTL8365MB_SDS_REG_BMCR: c_uint = 0x0000;

pub const RTL8365MB_SDS_REG_NWAY: c_uint = 0x0002;
pub const RTL8365MB_SDS_NWAY_EN_MASK: c_uint = 0x0200;
pub const RTL8365MB_SDS_NWAY_RESTART_MASK: c_uint = 0x0100;
pub const RTL8365MB_SDS_REG_RESET: c_uint = 0x0003;
pub const RTL8365MB_SDS_RESET_DEASSERT: c_uint = 0x7106;
pub const RTL8365MB_SDS_REG_LINK_STATUS: c_uint = 0x003d;
pub const RTL8365MB_SDS_LINK_STATUS_LINK_MASK: c_uint = 0x0010;
// The embedded SerDes can only be muxed to external interface 1 (MAC8),
// which is port 6.
//
pub const RTL8365MB_SDS_EXT_INTERFACE_ID: c_int = 1;
pub const RTL8365MB_SDS_EXT_INTERFACE_PORT: c_int = 6;
// Line rate bypass bit for the SerDes external interface

    RTL8365MB_BYPASS_LINE_RATE_MASK(RTL8365MB_SDS_EXT_INTERFACE_PORT)
// SerDes tuning parameter variant selector. The vendor driver picks between
// two sets of SerDes tuning parameters based on this chip option. Reading it
// requires first arming the read by writing a magic key to the arm register,
// then disarming it afterwards.
//
pub const RTL8365MB_SDS_OPTION_ARM_REG: c_uint = 0x13C0;
pub const RTL8365MB_SDS_OPTION_ARM_KEY: c_uint = 0x0249;
pub const RTL8365MB_SDS_OPTION_REG: c_uint = 0x13C1;
// Embedded DW8051 microcontroller control registers. The microcontroller
// can run firmware to manage the SerDes link, but this driver keeps it in
// reset and disabled: phylink already performs the link management that
// the firmware would otherwise do.
//
pub const RTL8365MB_MISC_CFG0_REG: c_uint = 0x130C;
pub const RTL8365MB_MISC_CFG0_DW8051_EN_MASK: c_uint = 0x0020;
// External interface port speed values - used in DIGITAL_INTERFACE_FORCE
pub const RTL8365MB_PORT_SPEED_10M: c_int = 0;
pub const RTL8365MB_PORT_SPEED_100M: c_int = 1;
pub const RTL8365MB_PORT_SPEED_1000M: c_int = 2;
// External interface force configuration registers 0~2
pub const RTL8365MB_DIGITAL_INTERFACE_FORCE_REG0: c_uint = 0x1310 /* EXT0 */;
pub const RTL8365MB_DIGITAL_INTERFACE_FORCE_REG1: c_uint = 0x1311 /* EXT1 */;
pub const RTL8365MB_DIGITAL_INTERFACE_FORCE_REG2: c_uint = 0x13C4 /* EXT2 */;

    ((_extint) == 0 ? RTL8365MB_DIGITAL_INTERFACE_FORCE_REG0 : \
    (_extint) == 1 ? RTL8365MB_DIGITAL_INTERFACE_FORCE_REG1 : \
    (_extint) == 2 ? RTL8365MB_DIGITAL_INTERFACE_FORCE_REG2 : \
    0x0)
pub const RTL8365MB_DIGITAL_INTERFACE_FORCE_EN_MASK: c_uint = 0x1000;
pub const RTL8365MB_DIGITAL_INTERFACE_FORCE_NWAY_MASK: c_uint = 0x0080;
pub const RTL8365MB_DIGITAL_INTERFACE_FORCE_TXPAUSE_MASK: c_uint = 0x0040;
pub const RTL8365MB_DIGITAL_INTERFACE_FORCE_RXPAUSE_MASK: c_uint = 0x0020;
pub const RTL8365MB_DIGITAL_INTERFACE_FORCE_LINK_MASK: c_uint = 0x0010;
pub const RTL8365MB_DIGITAL_INTERFACE_FORCE_DUPLEX_MASK: c_uint = 0x0004;
pub const RTL8365MB_DIGITAL_INTERFACE_FORCE_SPEED_MASK: c_uint = 0x0003;
// CPU port mask register - controls which ports are treated as CPU ports
pub const RTL8365MB_CPU_PORT_MASK_REG: c_uint = 0x1219;
pub const RTL8365MB_CPU_PORT_MASK_MASK: c_uint = 0x07FF;
// CPU control register
pub const RTL8365MB_CPU_CTRL_REG: c_uint = 0x121A;
pub const RTL8365MB_CPU_CTRL_TRAP_PORT_EXT_MASK: c_uint = 0x0400;
pub const RTL8365MB_CPU_CTRL_TAG_FORMAT_MASK: c_uint = 0x0200;
pub const RTL8365MB_CPU_CTRL_RXBYTECOUNT_MASK: c_uint = 0x0080;
pub const RTL8365MB_CPU_CTRL_TAG_POSITION_MASK: c_uint = 0x0040;
pub const RTL8365MB_CPU_CTRL_TRAP_PORT_MASK: c_uint = 0x0038;
pub const RTL8365MB_CPU_CTRL_INSERTMODE_MASK: c_uint = 0x0006;
pub const RTL8365MB_CPU_CTRL_EN_MASK: c_uint = 0x0001;
// Maximum packet length register
pub const RTL8365MB_CFG0_MAX_LEN_REG: c_uint = 0x088C;
pub const RTL8365MB_CFG0_MAX_LEN_MASK: c_uint = 0x3FFF;
pub const RTL8365MB_CFG0_MAX_LEN_MAX: c_uint = 0x3FFF;
// Port learning limit registers
pub const RTL8365MB_LUT_PORT_LEARN_LIMIT_BASE: c_uint = 0x0A20;

    (RTL8365MB_LUT_PORT_LEARN_LIMIT_BASE + (_physport))
// Port isolation (forwarding mask) registers
pub const RTL8365MB_PORT_ISOLATION_REG_BASE: c_uint = 0x08A2;

    (RTL8365MB_PORT_ISOLATION_REG_BASE + (_physport))
pub const RTL8365MB_PORT_ISOLATION_MASK: c_uint = 0x07FF;
// Extended filter ID registers - used to key forwarding database with IVL

pub const RTL8365MB_PORT_EFID_REG_BASE: c_uint = 0x0A32;

    (RTL8365MB_PORT_EFID_REG_BASE + ((_p) >> 2))

    (RTL8365MB_EFID_MASK << RTL8365MB_PORT_EFID_OFFSET(_p))
// MSTP port state registers - indexed by tree instance
pub const RTL8365MB_MSTI_CTRL_BASE: c_uint = 0x0A00;

    (RTL8365MB_MSTI_CTRL_BASE + ((_msti) << 1) + ((_physport) >> 3))

    (0x3 << RTL8365MB_MSTI_CTRL_PORT_STATE_OFFSET((_physport)))
// Unknown unicast DA flooding port mask
pub const RTL8365MB_UNKNOWN_UNICAST_FLOODING_PMASK_REG: c_uint = 0x0890;
pub const RTL8365MB_UNKNOWN_UNICAST_FLOODING_PMASK_MASK: c_uint = 0x07FF;
// Unknown multicast DA flooding port mask
pub const RTL8365MB_UNKNOWN_MULTICAST_FLOODING_PMASK_REG: c_uint = 0x0891;
pub const RTL8365MB_UNKNOWN_MULTICAST_FLOODING_PMASK_MASK: c_uint = 0x07FF;
// Broadcast flooding port mask
pub const RTL8365MB_UNKNOWN_BROADCAST_FLOODING_PMASK_REG: c_uint = 0x0892;
pub const RTL8365MB_UNKNOWN_BROADCAST_FLOODING_PMASK_MASK: c_uint = 0x07FF;

    (BR_LEARNING | BR_FLOOD | BR_MCAST_FLOOD | BR_BCAST_FLOOD)
// Miscellaneous port configuration register, incl. VLAN egress mode
pub const RTL8365MB_PORT_MISC_CFG_REG_BASE: c_uint = 0x000E;

    (RTL8365MB_PORT_MISC_CFG_REG_BASE + ((_p) << 5))
pub const RTL8365MB_PORT_MISC_CFG_SMALL_TAG_IPG_MASK: c_uint = 0x8000;
pub const RTL8365MB_PORT_MISC_CFG_TX_ITFSP_MODE_MASK: c_uint = 0x4000;
pub const RTL8365MB_PORT_MISC_CFG_FLOWCTRL_INDEP_MASK: c_uint = 0x2000;
pub const RTL8365MB_PORT_MISC_CFG_DOT1Q_REMARK_ENABLE_MASK: c_uint = 0x1000;
pub const RTL8365MB_PORT_MISC_CFG_INGRESSBW_FLOWCTRL_MASK: c_uint = 0x0800;
pub const RTL8365MB_PORT_MISC_CFG_INGRESSBW_IFG_MASK: c_uint = 0x0400;
pub const RTL8365MB_PORT_MISC_CFG_RX_SPC_MASK: c_uint = 0x0200;
pub const RTL8365MB_PORT_MISC_CFG_CRC_SKIP_MASK: c_uint = 0x0100;
pub const RTL8365MB_PORT_MISC_CFG_PKTGEN_TX_FIRST_MASK: c_uint = 0x0080;
pub const RTL8365MB_PORT_MISC_CFG_MAC_LOOPBACK_MASK: c_uint = 0x0040;
// See &rtl8365mb_vlan_egress_mode
pub const RTL8365MB_PORT_MISC_CFG_VLAN_EGRESS_MODE_MASK: c_uint = 0x0030;
pub const RTL8365MB_PORT_MISC_CFG_CONGESTION_SUSTAIN_TIME_MASK: c_uint = 0x000F;
//
// enum rtl8365mb_vlan_egress_mode - port VLAN egress mode
// @RTL8365MB_VLAN_EGRESS_MODE_ORIGINAL: follow untag mask in VLAN4k table entry
// @RTL8365MB_VLAN_EGRESS_MODE_KEEP: the VLAN tag format of egressed packets
// will remain the same as their ingressed format, but the priority and VID
// fields may be altered
// @RTL8365MB_VLAN_EGRESS_MODE_PRI_TAG: always egress with priority tag
// @RTL8365MB_VLAN_EGRESS_MODE_REAL_KEEP: the VLAN tag format of egressed
// packets will remain the same as their ingressed format, and neither the
// priority nor VID fields can be altered
//
    enum rtl8365mb_vlan_egress_mode {
    RTL8365MB_VLAN_EGRESS_MODE_ORIGINAL = 0,
    RTL8365MB_VLAN_EGRESS_MODE_KEEP = 1,
    RTL8365MB_VLAN_EGRESS_MODE_PRI_TAG = 2,
    RTL8365MB_VLAN_EGRESS_MODE_REAL_KEEP = 3,
    };
// VLAN control register
pub const RTL8365MB_VLAN_CTRL_REG: c_uint = 0x07A8;
pub const RTL8365MB_VLAN_CTRL_EN_MASK: c_uint = 0x0001;
// VLAN ingress filter register
pub const RTL8365MB_VLAN_INGRESS_REG: c_uint = 0x07A9;

// VLAN "transparent" setting registers
pub const RTL8365MB_VLAN_EGRESS_TRANSPARENT_REG_BASE: c_uint = 0x09D0;

    (RTL8365MB_VLAN_EGRESS_TRANSPARENT_REG_BASE + (_p))
// MIB counter value registers
pub const RTL8365MB_MIB_COUNTER_BASE: c_uint = 0x1000;

// MIB counter address register
pub const RTL8365MB_MIB_ADDRESS_REG: c_uint = 0x1004;
pub const RTL8365MB_MIB_ADDRESS_PORT_OFFSET: c_uint = 0x007C;

    (((RTL8365MB_MIB_ADDRESS_PORT_OFFSET) * (_p) + (_x)) >> 2)
pub const RTL8365MB_MIB_CTRL0_REG: c_uint = 0x1005;
pub const RTL8365MB_MIB_CTRL0_RESET_MASK: c_uint = 0x0002;
pub const RTL8365MB_MIB_CTRL0_BUSY_MASK: c_uint = 0x0001;
// The DSA callback .get_stats64 runs in atomic context, so we are not allowed
// to block. On the other hand, accessing MIB counters absolutely requires us to
// block. The solution is thus to schedule work which polls the MIB counters
// asynchronously and updates some private data, which the callback can then
// fetch atomically. Three seconds should be a good enough polling interval.
//

    enum rtl8365mb_mib_counter_index {
    RTL8365MB_MIB_ifInOctets,
    RTL8365MB_MIB_dot3StatsFCSErrors,
    RTL8365MB_MIB_dot3StatsSymbolErrors,
    RTL8365MB_MIB_dot3InPauseFrames,
    RTL8365MB_MIB_dot3ControlInUnknownOpcodes,
    RTL8365MB_MIB_etherStatsFragments,
    RTL8365MB_MIB_etherStatsJabbers,
    RTL8365MB_MIB_ifInUcastPkts,
    RTL8365MB_MIB_etherStatsDropEvents,
    RTL8365MB_MIB_ifInMulticastPkts,
    RTL8365MB_MIB_ifInBroadcastPkts,
    RTL8365MB_MIB_inMldChecksumError,
    RTL8365MB_MIB_inIgmpChecksumError,
    RTL8365MB_MIB_inMldSpecificQuery,
    RTL8365MB_MIB_inMldGeneralQuery,
    RTL8365MB_MIB_inIgmpSpecificQuery,
    RTL8365MB_MIB_inIgmpGeneralQuery,
    RTL8365MB_MIB_inMldLeaves,
    RTL8365MB_MIB_inIgmpLeaves,
    RTL8365MB_MIB_etherStatsOctets,
    RTL8365MB_MIB_etherStatsUnderSizePkts,
    RTL8365MB_MIB_etherOversizeStats,
    RTL8365MB_MIB_etherStatsPkts64Octets,
    RTL8365MB_MIB_etherStatsPkts65to127Octets,
    RTL8365MB_MIB_etherStatsPkts128to255Octets,
    RTL8365MB_MIB_etherStatsPkts256to511Octets,
    RTL8365MB_MIB_etherStatsPkts512to1023Octets,
    RTL8365MB_MIB_etherStatsPkts1024to1518Octets,
    RTL8365MB_MIB_ifOutOctets,
    RTL8365MB_MIB_dot3StatsSingleCollisionFrames,
    RTL8365MB_MIB_dot3StatsMultipleCollisionFrames,
    RTL8365MB_MIB_dot3StatsDeferredTransmissions,
    RTL8365MB_MIB_dot3StatsLateCollisions,
    RTL8365MB_MIB_etherStatsCollisions,
    RTL8365MB_MIB_dot3StatsExcessiveCollisions,
    RTL8365MB_MIB_dot3OutPauseFrames,
    RTL8365MB_MIB_ifOutDiscards,
    RTL8365MB_MIB_dot1dTpPortInDiscards,
    RTL8365MB_MIB_ifOutUcastPkts,
    RTL8365MB_MIB_ifOutMulticastPkts,
    RTL8365MB_MIB_ifOutBroadcastPkts,
    RTL8365MB_MIB_outOampduPkts,
    RTL8365MB_MIB_inOampduPkts,
    RTL8365MB_MIB_inIgmpJoinsSuccess,
    RTL8365MB_MIB_inIgmpJoinsFail,
    RTL8365MB_MIB_inMldJoinsSuccess,
    RTL8365MB_MIB_inMldJoinsFail,
    RTL8365MB_MIB_inReportSuppressionDrop,
    RTL8365MB_MIB_inLeaveSuppressionDrop,
    RTL8365MB_MIB_outIgmpReports,
    RTL8365MB_MIB_outIgmpLeaves,
    RTL8365MB_MIB_outIgmpGeneralQuery,
    RTL8365MB_MIB_outIgmpSpecificQuery,
    RTL8365MB_MIB_outMldReports,
    RTL8365MB_MIB_outMldLeaves,
    RTL8365MB_MIB_outMldGeneralQuery,
    RTL8365MB_MIB_outMldSpecificQuery,
    RTL8365MB_MIB_inKnownMulticastPkts,
    RTL8365MB_MIB_END,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8365mb_mib_counter {
    pub offset: u32,
    pub length: u32,
    pub name: *const c_char,
}

    [RTL8365MB_MIB_ ## _name] = { _offset, _length, #_name }
    static struct rtl8365mb_mib_counter rtl8365mb_mib_counters[] = {
    RTL8365MB_MAKE_MIB_COUNTER(0, 4, ifInOctets),
    RTL8365MB_MAKE_MIB_COUNTER(4, 2, dot3StatsFCSErrors),
    RTL8365MB_MAKE_MIB_COUNTER(6, 2, dot3StatsSymbolErrors),
    RTL8365MB_MAKE_MIB_COUNTER(8, 2, dot3InPauseFrames),
    RTL8365MB_MAKE_MIB_COUNTER(10, 2, dot3ControlInUnknownOpcodes),
    RTL8365MB_MAKE_MIB_COUNTER(12, 2, etherStatsFragments),
    RTL8365MB_MAKE_MIB_COUNTER(14, 2, etherStatsJabbers),
    RTL8365MB_MAKE_MIB_COUNTER(16, 2, ifInUcastPkts),
    RTL8365MB_MAKE_MIB_COUNTER(18, 2, etherStatsDropEvents),
    RTL8365MB_MAKE_MIB_COUNTER(20, 2, ifInMulticastPkts),
    RTL8365MB_MAKE_MIB_COUNTER(22, 2, ifInBroadcastPkts),
    RTL8365MB_MAKE_MIB_COUNTER(24, 2, inMldChecksumError),
    RTL8365MB_MAKE_MIB_COUNTER(26, 2, inIgmpChecksumError),
    RTL8365MB_MAKE_MIB_COUNTER(28, 2, inMldSpecificQuery),
    RTL8365MB_MAKE_MIB_COUNTER(30, 2, inMldGeneralQuery),
    RTL8365MB_MAKE_MIB_COUNTER(32, 2, inIgmpSpecificQuery),
    RTL8365MB_MAKE_MIB_COUNTER(34, 2, inIgmpGeneralQuery),
    RTL8365MB_MAKE_MIB_COUNTER(36, 2, inMldLeaves),
    RTL8365MB_MAKE_MIB_COUNTER(38, 2, inIgmpLeaves),
    RTL8365MB_MAKE_MIB_COUNTER(40, 4, etherStatsOctets),
    RTL8365MB_MAKE_MIB_COUNTER(44, 2, etherStatsUnderSizePkts),
    RTL8365MB_MAKE_MIB_COUNTER(46, 2, etherOversizeStats),
    RTL8365MB_MAKE_MIB_COUNTER(48, 2, etherStatsPkts64Octets),
    RTL8365MB_MAKE_MIB_COUNTER(50, 2, etherStatsPkts65to127Octets),
    RTL8365MB_MAKE_MIB_COUNTER(52, 2, etherStatsPkts128to255Octets),
    RTL8365MB_MAKE_MIB_COUNTER(54, 2, etherStatsPkts256to511Octets),
    RTL8365MB_MAKE_MIB_COUNTER(56, 2, etherStatsPkts512to1023Octets),
    RTL8365MB_MAKE_MIB_COUNTER(58, 2, etherStatsPkts1024to1518Octets),
    RTL8365MB_MAKE_MIB_COUNTER(60, 4, ifOutOctets),
    RTL8365MB_MAKE_MIB_COUNTER(64, 2, dot3StatsSingleCollisionFrames),
    RTL8365MB_MAKE_MIB_COUNTER(66, 2, dot3StatsMultipleCollisionFrames),
    RTL8365MB_MAKE_MIB_COUNTER(68, 2, dot3StatsDeferredTransmissions),
    RTL8365MB_MAKE_MIB_COUNTER(70, 2, dot3StatsLateCollisions),
    RTL8365MB_MAKE_MIB_COUNTER(72, 2, etherStatsCollisions),
    RTL8365MB_MAKE_MIB_COUNTER(74, 2, dot3StatsExcessiveCollisions),
    RTL8365MB_MAKE_MIB_COUNTER(76, 2, dot3OutPauseFrames),
    RTL8365MB_MAKE_MIB_COUNTER(78, 2, ifOutDiscards),
    RTL8365MB_MAKE_MIB_COUNTER(80, 2, dot1dTpPortInDiscards),
    RTL8365MB_MAKE_MIB_COUNTER(82, 2, ifOutUcastPkts),
    RTL8365MB_MAKE_MIB_COUNTER(84, 2, ifOutMulticastPkts),
    RTL8365MB_MAKE_MIB_COUNTER(86, 2, ifOutBroadcastPkts),
    RTL8365MB_MAKE_MIB_COUNTER(88, 2, outOampduPkts),
    RTL8365MB_MAKE_MIB_COUNTER(90, 2, inOampduPkts),
    RTL8365MB_MAKE_MIB_COUNTER(92, 4, inIgmpJoinsSuccess),
    RTL8365MB_MAKE_MIB_COUNTER(96, 2, inIgmpJoinsFail),
    RTL8365MB_MAKE_MIB_COUNTER(98, 2, inMldJoinsSuccess),
    RTL8365MB_MAKE_MIB_COUNTER(100, 2, inMldJoinsFail),
    RTL8365MB_MAKE_MIB_COUNTER(102, 2, inReportSuppressionDrop),
    RTL8365MB_MAKE_MIB_COUNTER(104, 2, inLeaveSuppressionDrop),
    RTL8365MB_MAKE_MIB_COUNTER(106, 2, outIgmpReports),
    RTL8365MB_MAKE_MIB_COUNTER(108, 2, outIgmpLeaves),
    RTL8365MB_MAKE_MIB_COUNTER(110, 2, outIgmpGeneralQuery),
    RTL8365MB_MAKE_MIB_COUNTER(112, 2, outIgmpSpecificQuery),
    RTL8365MB_MAKE_MIB_COUNTER(114, 2, outMldReports),
    RTL8365MB_MAKE_MIB_COUNTER(116, 2, outMldLeaves),
    RTL8365MB_MAKE_MIB_COUNTER(118, 2, outMldGeneralQuery),
    RTL8365MB_MAKE_MIB_COUNTER(120, 2, outMldSpecificQuery),
    RTL8365MB_MAKE_MIB_COUNTER(122, 2, inKnownMulticastPkts),
    };
    static_assert(ARRAY_SIZE(rtl8365mb_mib_counters) == RTL8365MB_MIB_END);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8365mb_jam_tbl_entry {
    pub reg: u16,
    pub val: u16,
}

// Lifted from the vendor driver sources
    static const struct rtl8365mb_jam_tbl_entry rtl8365mb_init_jam_8365mb_vc[] = {
    { 0x13EB, 0x15BB }, { 0x1303, 0x06D6 }, { 0x1304, 0x0700 },
    { 0x13E2, 0x003F }, { 0x13F9, 0x0090 }, { 0x121E, 0x03CA },
    { 0x1233, 0x0352 }, { 0x1237, 0x00A0 }, { 0x123A, 0x0030 },
    { 0x1239, 0x0084 }, { 0x0301, 0x1000 }, { 0x1349, 0x001F },
    { 0x18E0, 0x4004 }, { 0x122B, 0x241C }, { 0x1305, 0xC000 },
    { 0x13F0, 0x0000 },
    };
    static const struct rtl8365mb_jam_tbl_entry rtl8365mb_init_jam_common[] = {
    { 0x1200, 0x7FCB }, { 0x0884, 0x0003 }, { 0x06EB, 0x0001 },
    { 0x03Fa, 0x0007 }, { 0x08C8, 0x00C0 }, { 0x0A30, 0x020E },
    { 0x0800, 0x0000 }, { 0x0802, 0x0000 }, { 0x09DA, 0x0013 },
    { 0x1D32, 0x0002 },
    };
// SGMII SerDes tuning parameters, lifted from the vendor driver sources. The
// vendor driver keeps two variants of this table and selects between them
// based on the chip option register; these are the values for a non-zero
// option, which is what RTL8367S parts seen so far report. See
// rtl8365mb_sds_probe_option().
//
    static const struct rtl8365mb_jam_tbl_entry rtl8365mb_sds_jam_sgmii[] = {
    { 0x0480, 0x04D7 }, { 0x0481, 0xF994 }, { 0x0482, 0x2420 },
    { 0x0483, 0x6960 }, { 0x0484, 0x9728 }, { 0x0423, 0x9D85 },
    { 0x0424, 0xD810 }, { 0x002E, 0x83F2 },
    };
// HSGMII SerDes tuning parameters, lifted from the vendor driver sources. As
// with the SGMII table, the vendor driver keeps several variants and selects
// one based on the chip option register; these are the values for a non-zero
// option, which is what RTL8367S parts seen so far report. See
// rtl8365mb_sds_probe_option().
//
    static const struct rtl8365mb_jam_tbl_entry rtl8365mb_sds_jam_hsgmii[] = {
    { 0x0500, 0x82F0 }, { 0x0501, 0xF195 }, { 0x0502, 0x31A2 },
    { 0x0503, 0x7960 }, { 0x0504, 0x9728 }, { 0x0423, 0x9D85 },
    { 0x0424, 0xD810 }, { 0x0001, 0x0F80 }, { 0x002E, 0x83F2 },
    };
    enum rtl8365mb_phy_interface_mode {
    RTL8365MB_PHY_INTERFACE_MODE_INVAL = 0,
    RTL8365MB_PHY_INTERFACE_MODE_INTERNAL = BIT(0),
    RTL8365MB_PHY_INTERFACE_MODE_MII = BIT(1),
    RTL8365MB_PHY_INTERFACE_MODE_TMII = BIT(2),
    RTL8365MB_PHY_INTERFACE_MODE_RMII = BIT(3),
    RTL8365MB_PHY_INTERFACE_MODE_RGMII = BIT(4),
    RTL8365MB_PHY_INTERFACE_MODE_SGMII = BIT(5),
    RTL8365MB_PHY_INTERFACE_MODE_HSGMII = BIT(6),
    };
//
// struct rtl8365mb_extint - external interface info
// @port: the port with an external interface
// @id: the external interface ID, which is either 0, 1, or 2
// @supported_interfaces: a bitmask of supported PHY interface modes
//
// Represents a mapping: port -> { id, supported_interfaces }. To be embedded
// in &struct rtl8365mb_chip_info for every port with an external interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8365mb_extint {
    pub port: c_int,
    pub id: c_int,
    pub supported_interfaces: c_uint,
}

//
// struct rtl8365mb_chip_info - static chip-specific info
// @name: human-readable chip name
// @chip_id: chip identifier
// @chip_ver: chip silicon revision
// @extints: available external interfaces
// @jam_table: chip-specific initialization jam table
// @jam_size: size of the chip's jam table
//
// These data are specific to a given chip in the family of switches supported
// by this driver. When adding support for another chip in the family, a new
// chip info should be added to the rtl8365mb_chip_infos array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8365mb_chip_info {
    pub name: *const c_char,
    pub chip_id: u32,
    pub chip_ver: u32,
    pub extints: [rtl8365mb_extint; RTL8365MB_MAX_NUM_EXTINTS],
    pub jam_table: *const rtl8365mb_jam_tbl_entry,
    pub jam_size: usize,
}

// Chip info for each supported switch in the family

    static const struct rtl8365mb_chip_info rtl8365mb_chip_infos[] = {
    {
    .name = "RTL8365MB-VC",
    .chip_id = 0x6367,
    .chip_ver = 0x0040,
    .extints = {
    { 6, 1, PHY_INTF(MII) | PHY_INTF(TMII) |
    PHY_INTF(RMII) | PHY_INTF(RGMII) },
    },
    .jam_table = rtl8365mb_init_jam_8365mb_vc,
    .jam_size = ARRAY_SIZE(rtl8365mb_init_jam_8365mb_vc),
    },
    {
    .name = "RTL8367S",
    .chip_id = 0x6367,
    .chip_ver = 0x00A0,
    .extints = {
    { 6, 1, PHY_INTF(SGMII) | PHY_INTF(HSGMII) },
    { 7, 2, PHY_INTF(MII) | PHY_INTF(TMII) |
    PHY_INTF(RMII) | PHY_INTF(RGMII) },
    },
    .jam_table = rtl8365mb_init_jam_8365mb_vc,
    .jam_size = ARRAY_SIZE(rtl8365mb_init_jam_8365mb_vc),
    },
    {
    .name = "RTL8367SB",
    .chip_id = 0x6367,
    .chip_ver = 0x0010,
    .extints = {
    { 6, 1, PHY_INTF(MII) | PHY_INTF(TMII) |
    PHY_INTF(RMII) | PHY_INTF(RGMII) |
    PHY_INTF(SGMII) | PHY_INTF(HSGMII) },
    { 7, 2, PHY_INTF(MII) | PHY_INTF(TMII) |
    PHY_INTF(RMII) | PHY_INTF(RGMII) },
    },
    .jam_table = rtl8365mb_init_jam_8365mb_vc,
    .jam_size = ARRAY_SIZE(rtl8365mb_init_jam_8365mb_vc),
    },
    {
    .name = "RTL8367RB-VB",
    .chip_id = 0x6367,
    .chip_ver = 0x0020,
    .extints = {
    { 6, 1, PHY_INTF(MII) | PHY_INTF(TMII) |
    PHY_INTF(RMII) | PHY_INTF(RGMII) },
    { 7, 2, PHY_INTF(MII) | PHY_INTF(TMII) |
    PHY_INTF(RMII) | PHY_INTF(RGMII) },
    },
    .jam_table = rtl8365mb_init_jam_8365mb_vc,
    .jam_size = ARRAY_SIZE(rtl8365mb_init_jam_8365mb_vc),
    },
    };
    enum rtl8365mb_stp_state {
    RTL8365MB_STP_STATE_DISABLED = 0,
    RTL8365MB_STP_STATE_BLOCKING = 1,
    RTL8365MB_STP_STATE_LEARNING = 2,
    RTL8365MB_STP_STATE_FORWARDING = 3,
    };
    enum rtl8365mb_cpu_insert {
    RTL8365MB_CPU_INSERT_TO_ALL = 0,
    RTL8365MB_CPU_INSERT_TO_TRAPPING = 1,
    RTL8365MB_CPU_INSERT_TO_NONE = 2,
    };
    enum rtl8365mb_cpu_position {
    RTL8365MB_CPU_POS_AFTER_SA = 0,
    RTL8365MB_CPU_POS_BEFORE_CRC = 1,
    };
    enum rtl8365mb_cpu_format {
    RTL8365MB_CPU_FORMAT_8BYTES = 0,
    RTL8365MB_CPU_FORMAT_4BYTES = 1,
    };
    enum rtl8365mb_cpu_rxlen {
    RTL8365MB_CPU_RXLEN_72BYTES = 0,
    RTL8365MB_CPU_RXLEN_64BYTES = 1,
    };
//
// struct rtl8365mb_cpu - CPU port configuration
// @enable: enable/disable hardware insertion of CPU tag in switch->CPU frames
// @mask: port mask of ports that parse should parse CPU tags
// @trap_port: forward trapped frames to this port
// @insert: CPU tag insertion mode in switch->CPU frames
// @position: position of CPU tag in frame
// @rx_length: minimum CPU RX length
// @format: CPU tag format
//
// Represents the CPU tagging and CPU port configuration of the switch. These
// settings are configurable at runtime.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8365mb_cpu {
    pub enable: bool,
    pub mask: u32,
    pub trap_port: u32,
    pub insert: enum rtl8365mb_cpu_insert,
    pub position: enum rtl8365mb_cpu_position,
    pub rx_length: enum rtl8365mb_cpu_rxlen,
    pub format: enum rtl8365mb_cpu_format,
}

//
// struct rtl8365mb_port - private per-port data
// @priv: pointer to parent realtek_priv data
// @index: DSA port index, same as dsa_port::index
// @stats: link statistics populated by rtl8365mb_stats_poll, ready for atomic
// access via rtl8365mb_get_stats64
// @stats_lock: protect the stats structure during read/update
// @mib_work: delayed work for polling MIB counters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8365mb_port {
    pub priv: *mut realtek_priv,
    pub index: c_uint,
    pub stats: rtnl_link_stats64,
    pub stats_lock: spinlock_t,
    pub mib_work: delayed_work,
}

//
// struct rtl8365mb - driver private data
// @priv: pointer to parent realtek_priv data
// @irq: registered IRQ or zero
// @chip_info: chip-specific info about the attached switch
// @cpu: CPU tagging and CPU port configuration for this chip
// @mib_lock: prevent concurrent reads of MIB counters
// @ports: per-port data
// @pcs: PCS for the SerDes external interface
// @sds_supported: SerDes tuning parameters match the chip option, so the
// SerDes interface modes can be advertised
//
// Private data for this driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8365mb {
    pub priv: *mut realtek_priv,
    pub irq: c_int,
    pub chip_info: *const rtl8365mb_chip_info,
    pub cpu: rtl8365mb_cpu,
    pub mib_lock: mutex,
    pub ports: [rtl8365mb_port; RTL8365MB_MAX_NUM_PORTS],
    pub pcs: phylink_pcs,
    pub sds_supported: bool,
}

#[no_mangle]
unsafe extern "C" fn rtl8365mb_phy_poll_busy(priv: *mut realtek_priv) -> c_int {
    static int rtl8365mb_phy_poll_busy(struct realtek_priv *priv)
    {
    u32 val;
    return regmap_read_poll_timeout(priv.map_nolock,
    RTL8365MB_INDIRECT_ACCESS_STATUS_REG,
    val, !val, 10, 100);
    }
    static int rtl8365mb_phy_ocp_prepare(struct realtek_priv *priv, int phy,
    u32 ocp_addr)
    {
    u32 val;
    int ret;
// Set OCP prefix
    val = FIELD_GET(RTL8365MB_PHY_OCP_ADDR_PREFIX_MASK, ocp_addr);
    ret = regmap_update_bits(
    priv.map_nolock, RTL8365MB_GPHY_OCP_MSB_0_REG,
    RTL8365MB_GPHY_OCP_MSB_0_CFG_CPU_OCPADR_MASK,
    FIELD_PREP(RTL8365MB_GPHY_OCP_MSB_0_CFG_CPU_OCPADR_MASK, val));
    if (ret)
    return ret;
// Set PHY register address
    val = RTL8365MB_PHY_BASE;
    val |= FIELD_PREP(RTL8365MB_INDIRECT_ACCESS_ADDRESS_PHYNUM_MASK, phy);
    val |= FIELD_PREP(RTL8365MB_INDIRECT_ACCESS_ADDRESS_OCPADR_5_1_MASK,
    ocp_addr >> 1);
    val |= FIELD_PREP(RTL8365MB_INDIRECT_ACCESS_ADDRESS_OCPADR_9_6_MASK,
    ocp_addr >> 6);
    ret = regmap_write(priv.map_nolock,
    RTL8365MB_INDIRECT_ACCESS_ADDRESS_REG, val);
    if (ret)
    return ret;
    return 0;
    }
    static int rtl8365mb_phy_ocp_read(struct realtek_priv *priv, int phy,
    u32 ocp_addr, u16 *data)
    {
    u32 val;
    int ret;
    rtl83xx_lock(priv);
    ret = rtl8365mb_phy_poll_busy(priv);
    if (ret)
    goto out;
    ret = rtl8365mb_phy_ocp_prepare(priv, phy, ocp_addr);
    if (ret)
    goto out;
// Execute read operation
    val = FIELD_PREP(RTL8365MB_INDIRECT_ACCESS_CTRL_CMD_MASK,
    RTL8365MB_INDIRECT_ACCESS_CTRL_CMD_VALUE) |
    FIELD_PREP(RTL8365MB_INDIRECT_ACCESS_CTRL_RW_MASK,
    RTL8365MB_INDIRECT_ACCESS_CTRL_RW_READ);
    ret = regmap_write(priv.map_nolock, RTL8365MB_INDIRECT_ACCESS_CTRL_REG,
    val);
    if (ret)
    goto out;
    ret = rtl8365mb_phy_poll_busy(priv);
    if (ret)
    goto out;
// Get PHY register data
    ret = regmap_read(priv.map_nolock,
    RTL8365MB_INDIRECT_ACCESS_READ_DATA_REG, &val);
    if (ret)
    goto out;
// data = val & 0xFFFF;
    out:
    rtl83xx_unlock(priv);
    return ret;
    }
    static int rtl8365mb_phy_ocp_write(struct realtek_priv *priv, int phy,
    u32 ocp_addr, u16 data)
    {
    u32 val;
    int ret;
    rtl83xx_lock(priv);
    ret = rtl8365mb_phy_poll_busy(priv);
    if (ret)
    goto out;
    ret = rtl8365mb_phy_ocp_prepare(priv, phy, ocp_addr);
    if (ret)
    goto out;
// Set PHY register data
    ret = regmap_write(priv.map_nolock,
    RTL8365MB_INDIRECT_ACCESS_WRITE_DATA_REG, data);
    if (ret)
    goto out;
// Execute write operation
    val = FIELD_PREP(RTL8365MB_INDIRECT_ACCESS_CTRL_CMD_MASK,
    RTL8365MB_INDIRECT_ACCESS_CTRL_CMD_VALUE) |
    FIELD_PREP(RTL8365MB_INDIRECT_ACCESS_CTRL_RW_MASK,
    RTL8365MB_INDIRECT_ACCESS_CTRL_RW_WRITE);
    ret = regmap_write(priv.map_nolock, RTL8365MB_INDIRECT_ACCESS_CTRL_REG,
    val);
    if (ret)
    goto out;
    ret = rtl8365mb_phy_poll_busy(priv);
    if (ret)
    goto out;
    out:
    rtl83xx_unlock(priv);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_phy_read(priv: *mut realtek_priv, phy: c_int, regnum: c_int) -> c_int {
    static int rtl8365mb_phy_read(struct realtek_priv *priv, int phy, int regnum)
    {
    u32 ocp_addr;
    u16 val;
    int ret;
    if (phy > RTL8365MB_PHYADDRMAX)
    return -EINVAL;
    if (regnum > RTL8365MB_PHYREGMAX)
    return -EINVAL;
    ocp_addr = RTL8365MB_PHY_OCP_ADDR_PHYREG_BASE + regnum * 2;
    ret = rtl8365mb_phy_ocp_read(priv, phy, ocp_addr, &val);
    if (ret) {
    dev_err(priv.dev,
    "failed to read PHY%d reg %02x @ %04x, ret %pe\n", phy,
    regnum, ocp_addr, ERR_PTR(ret));
    return ret;
    }
    dev_dbg(priv.dev, "read PHY%d register 0x%02x @ %04x, val <- %04x\n",
    phy, regnum, ocp_addr, val);
    return val;
    }
    static int rtl8365mb_phy_write(struct realtek_priv *priv, int phy, int regnum,
    u16 val)
    {
    u32 ocp_addr;
    int ret;
    if (phy > RTL8365MB_PHYADDRMAX)
    return -EINVAL;
    if (regnum > RTL8365MB_PHYREGMAX)
    return -EINVAL;
    ocp_addr = RTL8365MB_PHY_OCP_ADDR_PHYREG_BASE + regnum * 2;
    ret = rtl8365mb_phy_ocp_write(priv, phy, ocp_addr, val);
    if (ret) {
    dev_err(priv.dev,
    "failed to write PHY%d reg %02x @ %04x, ret %pe\n", phy,
    regnum, ocp_addr, ERR_PTR(ret));
    return ret;
    }
    dev_dbg(priv.dev, "write PHY%d register 0x%02x @ %04x, val . %04x\n",
    phy, regnum, ocp_addr, val);
    return 0;
    }
    static const struct rtl8365mb_extint *
    rtl8365mb_get_port_extint(struct realtek_priv *priv, int port)
    {
    struct rtl8365mb *mb = priv.chip_data;
    int i;
    for (i = 0; i < RTL8365MB_MAX_NUM_EXTINTS; i++) {
    const struct rtl8365mb_extint *extint =
    &mb.chip_info.extints[i];
    if (!extint.supported_interfaces)
    continue;
    if (extint.port == port)
    return extint;
    }
    return core::ptr::null_mut();
    }
    static enum dsa_tag_protocol
    rtl8365mb_get_tag_protocol(struct dsa_switch *ds, int port,
    enum dsa_tag_protocol mp)
    {
    struct realtek_priv *priv = ds.priv;
    struct rtl8365mb_cpu *cpu;
    struct rtl8365mb *mb;
    mb = priv.chip_data;
    cpu = &mb.cpu;
    if (cpu.position == RTL8365MB_CPU_POS_BEFORE_CRC)
    return DSA_TAG_PROTO_RTL8_4T;
    return DSA_TAG_PROTO_RTL8_4;
    }
    static int rtl8365mb_ext_config_rgmii(struct realtek_priv *priv, int port,
    phy_interface_t interface)
    {
    const struct rtl8365mb_extint *extint =
    rtl8365mb_get_port_extint(priv, port);
    struct dsa_switch *ds = &priv.ds;
    struct device_node *dn;
    struct dsa_port *dp;
    let mut tx_delay: c_int = 0;
    let mut rx_delay: c_int = 0;
    u32 val;
    int ret;
    if (!extint)
    return -ENODEV;
    dp = dsa_to_port(ds, port);
    dn = dp.dn;
// Set the RGMII TX/RX delay
//
// The Realtek vendor driver indicates the following possible
// configuration settings:
//
// TX delay:
// 0 = no delay, 1 = 2 ns delay
// RX delay:
// 0 = no delay, 7 = maximum delay
// Each step is approximately 0.3 ns, so the maximum delay is about
// 2.1 ns.
//
// The vendor driver also states that this must be configured *before
// forcing the external interface into a particular mode, which is done
// in the rtl8365mb_phylink_mac_link_{up,down} functions.
//
// Only configure an RGMII TX (resp. RX) delay if the
// tx-internal-delay-ps (resp. rx-internal-delay-ps) OF property is
// specified. We ignore the detail of the RGMII interface mode
// (RGMII_{RXID, TXID, etc.}), as this is considered to be a PHY-only
// property.
//
    if (!of_property_read_u32(dn, "tx-internal-delay-ps", &val)) {
    val = val / 1000; /* convert to ns */
    if (val == 0 || val == 2)
    tx_delay = val / 2;
    else
    dev_warn(priv.dev,
    "RGMII TX delay must be 0 or 2 ns\n");
    }
    if (!of_property_read_u32(dn, "rx-internal-delay-ps", &val)) {
    val = DIV_ROUND_CLOSEST(val, 300); /* convert to 0.3 ns step */
    if (val <= 7)
    rx_delay = val;
    else
    dev_warn(priv.dev,
    "RGMII RX delay must be 0 to 2.1 ns\n");
    }
    ret = regmap_update_bits(
    priv.map, RTL8365MB_EXT_RGMXF_REG(extint.id),
    RTL8365MB_EXT_RGMXF_TXDELAY_MASK |
    RTL8365MB_EXT_RGMXF_RXDELAY_MASK,
    FIELD_PREP(RTL8365MB_EXT_RGMXF_TXDELAY_MASK, tx_delay) |
    FIELD_PREP(RTL8365MB_EXT_RGMXF_RXDELAY_MASK, rx_delay));
    if (ret)
    return ret;
    ret = regmap_update_bits(
    priv.map, RTL8365MB_DIGITAL_INTERFACE_SELECT_REG(extint.id),
    RTL8365MB_DIGITAL_INTERFACE_SELECT_MODE_MASK(extint.id),
    RTL8365MB_EXT_PORT_MODE_RGMII
    << RTL8365MB_DIGITAL_INTERFACE_SELECT_MODE_OFFSET(
    extint.id));
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_sds_write(priv: *mut realtek_priv, addr: u16, data: u16) -> c_int {
    static int rtl8365mb_sds_write(struct realtek_priv *priv, u16 addr, u16 data)
    {
    int ret;
    ret = regmap_write(priv.map, RTL8365MB_SDS_INDACS_DATA_REG, data);
    if (ret)
    return ret;
    ret = regmap_write(priv.map, RTL8365MB_SDS_INDACS_ADR_REG, addr);
    if (ret)
    return ret;
// The SerDes indirect access engine completes the command within the
// register write transaction, so there is no need to wait or poll for
// completion before the next access, matching the vendor driver.
//
    return regmap_write(priv.map, RTL8365MB_SDS_INDACS_CMD_REG,
    RTL8365MB_SDS_INDACS_CMD_RUN_MASK |
    RTL8365MB_SDS_INDACS_CMD_WR_MASK);
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_sds_read(priv: *mut realtek_priv, addr: u16, data: *mut u16) -> c_int {
    static int rtl8365mb_sds_read(struct realtek_priv *priv, u16 addr, u16 *data)
    {
    u32 val;
    int ret;
    ret = regmap_write(priv.map, RTL8365MB_SDS_INDACS_ADR_REG, addr);
    if (ret)
    return ret;
    ret = regmap_write(priv.map, RTL8365MB_SDS_INDACS_CMD_REG,
    RTL8365MB_SDS_INDACS_CMD_RUN_MASK);
    if (ret)
    return ret;
// Wait for the indirect read to complete: the engine clears the BUSY
// bit once the data register holds the result.
//
    ret = regmap_read_poll_timeout(priv.map, RTL8365MB_SDS_INDACS_CMD_REG,
    val,
    !(val & RTL8365MB_SDS_INDACS_CMD_BUSY_MASK),
    10, 1000);
    if (ret)
    return ret;
    ret = regmap_read(priv.map, RTL8365MB_SDS_INDACS_DATA_REG, &val);
    if (ret)
    return ret;
// data = val;
    return 0;
    }
// The vendor driver selects between two sets of SerDes tuning parameters based
// on the chip option register. Only the variant for a non-zero option has been
// tested on real hardware - the RTL8367S parts seen so far all report 1. The
// variant for option 0 uses different tuning values that cannot be verified,
// so probe the option once at setup and only advertise the SerDes interface
// modes when the tuning parameters are known to match, so that an unsupported
// variant fails at phylink validation time rather than when configuring the
// link.
//
#[no_mangle]
unsafe extern "C" fn rtl8365mb_sds_probe_option(priv: *mut realtek_priv) -> c_int {
    static int rtl8365mb_sds_probe_option(struct realtek_priv *priv)
    {
    struct rtl8365mb *mb = priv.chip_data;
    const struct rtl8365mb_extint *extint;
    u32 option;
    int ret;
    int i;
// Nothing to probe if no external interface is wired to the SerDes
    for (i = 0; i < RTL8365MB_MAX_NUM_EXTINTS; i++) {
    extint = &mb.chip_info.extints[i];
    if (extint.supported_interfaces &
    (RTL8365MB_PHY_INTERFACE_MODE_SGMII |
    RTL8365MB_PHY_INTERFACE_MODE_HSGMII))
    break;
    }
    if (i == RTL8365MB_MAX_NUM_EXTINTS)
    return 0;
    ret = regmap_write(priv.map, RTL8365MB_SDS_OPTION_ARM_REG,
    RTL8365MB_SDS_OPTION_ARM_KEY);
    if (ret)
    return ret;
    ret = regmap_read(priv.map, RTL8365MB_SDS_OPTION_REG, &option);
    if (ret)
    return ret;
    ret = regmap_write(priv.map, RTL8365MB_SDS_OPTION_ARM_REG, 0);
    if (ret)
    return ret;
    if (option == 0) {
    dev_warn(priv.dev,
    "unsupported SerDes tuning variant (chip option 0), disabling SerDes interface modes\n");
    return 0;
    }
    mb.sds_supported = true;
    return 0;
    }
// The vendor driver raises the port 6 ingress and egress rate limiters to
// their maximum in its switch init, unconditionally for the whole chip
// family. The chip reset in rtl8365mb_setup() puts them back to their reset
// default of 0x1FFFF, a ~1.048 Gbps limit which caps the aggregate
// throughput of an HSGMII CPU port at roughly 1 Gbps. The vendor
// documentation describes the reset default as disabling the limiter, but
// the cap has been observed on hardware. Raise them likewise, to 0x7FFFF
// (~4.19 Gbps, above the HSGMII line rate). The related HSGMII scheduler
// line rate register (LINE_RATE_HSG_H, 0x03FA) is already set to its
// maximum by the common init jam table.
//
#[no_mangle]
unsafe extern "C" fn rtl8365mb_sds_raise_rate_limits(priv: *mut realtek_priv) -> c_int {
    static int rtl8365mb_sds_raise_rate_limits(struct realtek_priv *priv)
    {
    int ret;
    ret = regmap_write(priv.map, RTL8365MB_INGRESSBW_PORT6_RATE_CTRL0_REG,
    0xFFFF);
    if (ret)
    return ret;
    ret = regmap_update_bits(priv.map,
    RTL8365MB_INGRESSBW_PORT6_RATE_CTRL1_REG,
    RTL8365MB_INGRESSBW_PORT6_RATE_CTRL1_MASK,
    RTL8365MB_INGRESSBW_PORT6_RATE_CTRL1_MASK);
    if (ret)
    return ret;
    ret = regmap_write(priv.map, RTL8365MB_PORT6_EGRESSBW_CTRL0_REG,
    0xFFFF);
    if (ret)
    return ret;
    return regmap_update_bits(priv.map, RTL8365MB_PORT6_EGRESSBW_CTRL1_REG,
    RTL8365MB_PORT6_EGRESSBW_CTRL1_MASK,
    RTL8365MB_PORT6_EGRESSBW_CTRL1_MASK);
    }
    static int rtl8365mb_pcs_config(struct phylink_pcs *pcs, unsigned int neg_mode,
    phy_interface_t interface,
    const unsigned long *advertising,
    bool permit_pause_to_mac)
    {
    const struct rtl8365mb_jam_tbl_entry *sds_jam;
    let mut id: c_int = RTL8365MB_SDS_EXT_INTERFACE_ID;
    struct rtl8365mb *mb = pcs_to_rtl8365mb(pcs);
    struct realtek_priv *priv;
    size_t sds_jam_size;
    u32 mode;
    u16 val;
    int ret;
    int i;
    priv = mb.priv;
    if (interface == PHY_INTERFACE_MODE_2500BASEX) {
    sds_jam = rtl8365mb_sds_jam_hsgmii;
    sds_jam_size = ARRAY_SIZE(rtl8365mb_sds_jam_hsgmii);
    mode = RTL8365MB_EXT_PORT_MODE_HSGMII;
    } else {
    sds_jam = rtl8365mb_sds_jam_sgmii;
    sds_jam_size = ARRAY_SIZE(rtl8365mb_sds_jam_sgmii);
    mode = RTL8365MB_EXT_PORT_MODE_SGMII;
    }
// Hold the embedded DW8051 microcontroller in reset and keep it
// disabled. The vendor driver loads firmware into it to manage the
// SerDes link, but the firmware only duplicates work that phylink
// already does: it polls the port status and forces the external
// interface configuration in the very registers this driver manages.
// Letting it run would race with phylink.
//
    ret = regmap_update_bits(priv.map, RTL8365MB_CHIP_RESET_REG,
    RTL8365MB_CHIP_RESET_DW8051_MASK,
    RTL8365MB_CHIP_RESET_DW8051_MASK);
    if (ret)
    return ret;
    ret = regmap_update_bits(priv.map, RTL8365MB_MISC_CFG0_REG,
    RTL8365MB_MISC_CFG0_DW8051_EN_MASK, 0);
    if (ret)
    return ret;
// The vendor driver clears the line rate bypass for all interface
// modes except TMII.
//
    ret = regmap_update_bits(priv.map, RTL8365MB_BYPASS_LINE_RATE_REG,
    RTL8365MB_SDS_BYPASS_LINE_RATE_MASK, 0);
    if (ret)
    return ret;
// Tune the SerDes with vendor-prescribed parameters
    for (i = 0; i < sds_jam_size; i++) {
    ret = rtl8365mb_sds_write(priv, sds_jam[i].reg,
    sds_jam[i].val);
    if (ret)
    return ret;
    }
// Mux the SerDes to MAC8 in the requested mode
    ret = regmap_update_bits(priv.map, RTL8365MB_SDS_MISC_REG,
    RTL8365MB_SDS_MISC_MAC8_SEL_SGMII_MASK |
    RTL8365MB_SDS_MISC_MAC8_SEL_HSGMII_MASK,
    mode == RTL8365MB_EXT_PORT_MODE_SGMII ?
    RTL8365MB_SDS_MISC_MAC8_SEL_SGMII_MASK :
    RTL8365MB_SDS_MISC_MAC8_SEL_HSGMII_MASK);
    if (ret)
    return ret;
    val = mode << RTL8365MB_DIGITAL_INTERFACE_SELECT_MODE_OFFSET(id);
    ret = regmap_update_bits(priv.map,
    RTL8365MB_DIGITAL_INTERFACE_SELECT_REG(id),
    RTL8365MB_DIGITAL_INTERFACE_SELECT_MODE_MASK(id),
    val);
    if (ret)
    return ret;
// Take the SerDes out of reset. The vendor driver does this only
// after the SerDes mux and the interface mode are configured.
//
    ret = rtl8365mb_sds_write(priv, RTL8365MB_SDS_REG_RESET,
    RTL8365MB_SDS_RESET_DEASSERT);
    if (ret)
    return ret;
// Reset the SerDes data path and resync its PLL, mirroring what the
// vendor firmware does right after deasserting the SerDes reset.
// This flushes the FIFOs and ensures a clean state for the link,
// preventing silent drops and CRC errors.
//
    ret = rtl8365mb_sds_write(priv, RTL8365MB_SDS_REG_BMCR,
    RTL8365MB_SDS_BMCR_DPRST_PHASE1);
    if (ret)
    return ret;
    ret = rtl8365mb_sds_write(priv, RTL8365MB_SDS_REG_BMCR,
    RTL8365MB_SDS_BMCR_DPRST_PHASE2);
    if (ret)
    return ret;
// Keep SGMII in-band autonegotiation disabled: the link parameters are
// forced from rtl8365mb_pcs_link_up() instead.
//
    ret = rtl8365mb_sds_read(priv, RTL8365MB_SDS_REG_NWAY, &val);
    if (ret)
    return ret;
    val &= ~RTL8365MB_SDS_NWAY_EN_MASK;
    val |= RTL8365MB_SDS_NWAY_RESTART_MASK;
    return rtl8365mb_sds_write(priv, RTL8365MB_SDS_REG_NWAY, val);
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_interface_is_serdes(interface: phy_interface_t) -> bool {
    static bool rtl8365mb_interface_is_serdes(phy_interface_t interface)
    {
    return interface == PHY_INTERFACE_MODE_SGMII ||
    interface == PHY_INTERFACE_MODE_2500BASEX;
    }
    static unsigned int rtl8365mb_pcs_inband_caps(struct phylink_pcs *pcs,
    phy_interface_t interface)
    {
// In-band autonegotiation is not implemented; the link is always
// forced. Report that to phylink so that it never selects an
// in-band-enabled negotiation mode for this PCS.
//
    return LINK_INBAND_DISABLE;
    }
    static void rtl8365mb_pcs_get_state(struct phylink_pcs *pcs,
    unsigned int neg_mode,
    struct phylink_link_state *state)
    {
    struct rtl8365mb *mb = pcs_to_rtl8365mb(pcs);
    struct realtek_priv *priv = mb.priv;
    u16 status;
    u32 val;
    int ret;
// In-band autonegotiation is not implemented, so the link parameters are
// forced from rtl8365mb_pcs_link_up(). The real link state must still be
// read from the SerDes itself: the embedded DW8051 microcontroller that
// the vendor firmware uses to poll the SerDes is kept disabled (see
// rtl8365mb_pcs_config()), so the link status register can be read
// directly through the SDS_INDACS window without racing the auto-poll.
//
    ret = rtl8365mb_sds_read(priv, RTL8365MB_SDS_REG_LINK_STATUS, &status);
    if (ret) {
    state.link = false;
    return;
    }
    state.link = !!(status & RTL8365MB_SDS_LINK_STATUS_LINK_MASK);
    state.an_complete = state.link;
    if (!state.link)
    return;
// The speed and duplex are forced; read them back from the values
// programmed into the SerDes MISC register.
//
    ret = regmap_read(priv.map, RTL8365MB_SDS_MISC_REG, &val);
    if (ret) {
    state.link = false;
    return;
    }
    state.duplex = (val & RTL8365MB_SDS_MISC_SGMII_FDUP_MASK) ?
    DUPLEX_FULL : DUPLEX_HALF;
    switch (FIELD_GET(RTL8365MB_SDS_MISC_SGMII_SPD_MASK, val)) {
    case RTL8365MB_PORT_SPEED_1000M:
    state.speed =
    state.interface == PHY_INTERFACE_MODE_2500BASEX ?
    SPEED_2500 : SPEED_1000;
    break;
    case RTL8365MB_PORT_SPEED_100M:
    state.speed = SPEED_100;
    break;
    case RTL8365MB_PORT_SPEED_10M:
    state.speed = SPEED_10;
    break;
    }
    }
    static void rtl8365mb_pcs_link_up(struct phylink_pcs *pcs,
    unsigned int neg_mode,
    phy_interface_t interface, int speed,
    int duplex)
    {
    struct rtl8365mb *mb = pcs_to_rtl8365mb(pcs);
    struct realtek_priv *priv = mb.priv;
    u32 mask = RTL8365MB_SDS_MISC_SGMII_FDUP_MASK |
    RTL8365MB_SDS_MISC_SGMII_LINK_MASK |
    RTL8365MB_SDS_MISC_SGMII_SPD_MASK;
    let mut val: u32 = RTL8365MB_SDS_MISC_SGMII_LINK_MASK;
    u32 r_speed;
    int ret;
// The speed field has no value for 2.5 Gbps: the rate is determined by
// the HSGMII SerDes configuration, and the vendor driver programs the
// 1 Gbps value here.
//
    if (speed == SPEED_2500 || speed == SPEED_1000) {
    r_speed = RTL8365MB_PORT_SPEED_1000M;
    } else if (speed == SPEED_100) {
    r_speed = RTL8365MB_PORT_SPEED_100M;
    } else if (speed == SPEED_10) {
    r_speed = RTL8365MB_PORT_SPEED_10M;
    } else {
    dev_err(priv.dev, "unsupported SerDes speed %s\n",
    phy_speed_to_str(speed));
    return;
    }
    val |= FIELD_PREP(RTL8365MB_SDS_MISC_SGMII_SPD_MASK, r_speed);
    if (duplex == DUPLEX_FULL)
    val |= RTL8365MB_SDS_MISC_SGMII_FDUP_MASK;
// pcs_link_up() carries no pause information, so the SerDes flow
// control bits are programmed together with the MAC external interface
// force from rtl8365mb_phylink_mac_link_up(), where the resolved pause
// modes are known.
//
    ret = regmap_update_bits(priv.map, RTL8365MB_SDS_MISC_REG, mask, val);
    if (ret) {
    dev_err(priv.dev, "failed to force SerDes link: %pe\n",
    ERR_PTR(ret));
    return;
    }
    }
    static const struct phylink_pcs_ops rtl8365mb_pcs_ops = {
    .pcs_inband_caps = rtl8365mb_pcs_inband_caps,
    .pcs_config = rtl8365mb_pcs_config,
    .pcs_get_state = rtl8365mb_pcs_get_state,
    .pcs_link_up = rtl8365mb_pcs_link_up,
    };
    static int rtl8365mb_ext_config_forcemode(struct realtek_priv *priv, int port,
    bool link, int speed, int duplex,
    bool tx_pause, bool rx_pause)
    {
    const struct rtl8365mb_extint *extint =
    rtl8365mb_get_port_extint(priv, port);
    u32 r_tx_pause;
    u32 r_rx_pause;
    u32 r_duplex;
    u32 r_speed;
    u32 r_link;
    int val;
    int ret;
    if (!extint)
    return -ENODEV;
    if (link) {
// Force the link up with the desired configuration
    r_link = 1;
    r_rx_pause = rx_pause ? 1 : 0;
    r_tx_pause = tx_pause ? 1 : 0;
// The speed field has no value for 2.5 Gbps: the rate is
// determined by the HSGMII SerDes configuration, and the
// vendor driver programs the 1 Gbps value here.
//
    if (speed == SPEED_2500 || speed == SPEED_1000) {
    r_speed = RTL8365MB_PORT_SPEED_1000M;
    } else if (speed == SPEED_100) {
    r_speed = RTL8365MB_PORT_SPEED_100M;
    } else if (speed == SPEED_10) {
    r_speed = RTL8365MB_PORT_SPEED_10M;
    } else {
    dev_err(priv.dev, "unsupported port speed %s\n",
    phy_speed_to_str(speed));
    return -EINVAL;
    }
    if (duplex == DUPLEX_FULL) {
    r_duplex = 1;
    } else if (duplex == DUPLEX_HALF) {
    r_duplex = 0;
    } else {
    dev_err(priv.dev, "unsupported duplex %s\n",
    phy_duplex_to_str(duplex));
    return -EINVAL;
    }
    } else {
// Force the link down and reset any programmed configuration
    r_link = 0;
    r_tx_pause = 0;
    r_rx_pause = 0;
    r_speed = 0;
    r_duplex = 0;
    }
    val = FIELD_PREP(RTL8365MB_DIGITAL_INTERFACE_FORCE_EN_MASK, 1) |
    FIELD_PREP(RTL8365MB_DIGITAL_INTERFACE_FORCE_TXPAUSE_MASK,
    r_tx_pause) |
    FIELD_PREP(RTL8365MB_DIGITAL_INTERFACE_FORCE_RXPAUSE_MASK,
    r_rx_pause) |
    FIELD_PREP(RTL8365MB_DIGITAL_INTERFACE_FORCE_LINK_MASK, r_link) |
    FIELD_PREP(RTL8365MB_DIGITAL_INTERFACE_FORCE_DUPLEX_MASK,
    r_duplex) |
    FIELD_PREP(RTL8365MB_DIGITAL_INTERFACE_FORCE_SPEED_MASK, r_speed);
    ret = regmap_write(priv.map,
    RTL8365MB_DIGITAL_INTERFACE_FORCE_REG(extint.id),
    val);
    if (ret)
    return ret;
    return 0;
    }
    static void rtl8365mb_phylink_get_caps(struct dsa_switch *ds, int port,
    struct phylink_config *config)
    {
    const struct rtl8365mb_extint *extint =
    rtl8365mb_get_port_extint(ds.priv, port);
    struct realtek_priv *priv = ds.priv;
    struct rtl8365mb *mb = priv.chip_data;
    config.mac_capabilities = MAC_SYM_PAUSE | MAC_ASYM_PAUSE |
    MAC_10 | MAC_100 | MAC_1000FD;
    if (!extint) {
    __set_bit(PHY_INTERFACE_MODE_INTERNAL,
    config.supported_interfaces);
// GMII is the default interface mode for phylib, so
// we have to support it for ports with integrated PHY.
//
    __set_bit(PHY_INTERFACE_MODE_GMII,
    config.supported_interfaces);
    return;
    }
// Populate according to the modes supported by _this driver_,
// not necessarily the modes supported by the hardware, some of
// which remain unimplemented.
//
    if (extint.supported_interfaces & RTL8365MB_PHY_INTERFACE_MODE_RGMII)
    phy_interface_set_rgmii(config.supported_interfaces);
    if (extint.supported_interfaces & RTL8365MB_PHY_INTERFACE_MODE_SGMII &&
    mb.sds_supported)
    __set_bit(PHY_INTERFACE_MODE_SGMII,
    config.supported_interfaces);
    if (extint.supported_interfaces & RTL8365MB_PHY_INTERFACE_MODE_HSGMII &&
    mb.sds_supported) {
    __set_bit(PHY_INTERFACE_MODE_2500BASEX,
    config.supported_interfaces);
    config.mac_capabilities |= MAC_2500FD;
    }
    }
    static struct phylink_pcs *
    rtl8365mb_phylink_mac_select_pcs(struct phylink_config *config,
    phy_interface_t interface)
    {
    struct dsa_port *dp = dsa_phylink_to_port(config);
    struct realtek_priv *priv = dp.ds.priv;
    struct rtl8365mb *mb = priv.chip_data;
    if (rtl8365mb_interface_is_serdes(interface))
    return &mb.pcs;
    return core::ptr::null_mut();
    }
    static void rtl8365mb_phylink_mac_config(struct phylink_config *config,
    unsigned int mode,
    const struct phylink_link_state *state)
    {
    struct dsa_port *dp = dsa_phylink_to_port(config);
    struct realtek_priv *priv = dp.ds.priv;
    let mut port: u8 = dp.index;
    int ret;
    if (mode != MLO_AN_PHY && mode != MLO_AN_FIXED) {
    dev_err(priv.dev,
    "port %d supports only conventional PHY or fixed-link\n",
    port);
    return;
    }
    if (phy_interface_mode_is_rgmii(state.interface)) {
    ret = rtl8365mb_ext_config_rgmii(priv, port, state.interface);
    if (ret)
    dev_err(priv.dev,
    "failed to configure RGMII mode on port %d: %pe\n",
    port, ERR_PTR(ret));
    return;
    }
// SGMII and 2500base-x are handled by the SerDes PCS, configured
// through the phylink_pcs ops, so nothing to do here for them.
//
    if (rtl8365mb_interface_is_serdes(state.interface))
    return;
// TODO: Implement MII and RMII modes, which the RTL8365MB-VC also
// supports
//
    }
    static void rtl8365mb_phylink_mac_link_down(struct phylink_config *config,
    unsigned int mode,
    phy_interface_t interface)
    {
    struct dsa_port *dp = dsa_phylink_to_port(config);
    struct realtek_priv *priv = dp.ds.priv;
    struct rtl8365mb_port *p;
    struct rtl8365mb *mb;
    let mut port: u8 = dp.index;
    int ret;
    mb = priv.chip_data;
    p = &mb.ports[port];
    cancel_delayed_work_sync(&p.mib_work);
// phylink has no pcs_link_down callback, so on the SerDes path only the
// MAC external interface force is reset here. Clearing the MAC force is
// enough to bring the link down; the SerDes keeps presenting its last
// forced state until the next pcs_link_up() reprograms it.
//
    if (phy_interface_mode_is_rgmii(interface) ||
    rtl8365mb_interface_is_serdes(interface)) {
    ret = rtl8365mb_ext_config_forcemode(priv, port, false, 0, 0,
    false, false);
    if (ret)
    dev_err(priv.dev,
    "failed to reset forced mode on port %d: %pe\n",
    port, ERR_PTR(ret));
    return;
    }
    }
    static void rtl8365mb_phylink_mac_link_up(struct phylink_config *config,
    struct phy_device *phydev,
    unsigned int mode,
    phy_interface_t interface,
    int speed, int duplex, bool tx_pause,
    bool rx_pause)
    {
    struct dsa_port *dp = dsa_phylink_to_port(config);
    struct realtek_priv *priv = dp.ds.priv;
    struct rtl8365mb_port *p;
    struct rtl8365mb *mb;
    let mut port: u8 = dp.index;
    int ret;
    mb = priv.chip_data;
    p = &mb.ports[port];
    schedule_delayed_work(&p.mib_work, 0);
// The SerDes forced link state is programmed by the PCS in
// rtl8365mb_pcs_link_up(); here only the MAC external interface force
// is configured, for both RGMII and SerDes.
//
    if (phy_interface_mode_is_rgmii(interface) ||
    rtl8365mb_interface_is_serdes(interface)) {
    ret = rtl8365mb_ext_config_forcemode(priv, port, true, speed,
    duplex, tx_pause,
    rx_pause);
    if (ret) {
    dev_err(priv.dev,
    "failed to force mode on port %d: %pe\n", port,
    ERR_PTR(ret));
    return;
    }
// The SerDes has its own pause enables; program them from
// the resolved pause modes, as the vendor driver does when
// forcing the link on a SerDes external interface. These
// bits, not the MAC force pause bits, gate pause on the
// SerDes external interface: flow control testing shows
// that pause frames are only emitted with the SerDes TXFC
// bit set, while the MAC force pause bits alone have no
// effect on this port. This is done here rather than in
// rtl8365mb_pcs_link_up() because pcs_link_up() carries no
// pause information.
//
    if (rtl8365mb_interface_is_serdes(interface)) {
    let mut val: u32 = 0;
    if (tx_pause)
    val |= RTL8365MB_SDS_MISC_SGMII_TXFC_MASK;
    if (rx_pause)
    val |= RTL8365MB_SDS_MISC_SGMII_RXFC_MASK;
    ret = regmap_update_bits(priv.map,
    RTL8365MB_SDS_MISC_REG,
    RTL8365MB_SDS_MISC_SGMII_TXFC_MASK |
    RTL8365MB_SDS_MISC_SGMII_RXFC_MASK,
    val);
    if (ret)
    dev_err(priv.dev,
    "failed to force SerDes pause modes on port %d: %pe\n",
    port, ERR_PTR(ret));
    }
    return;
    }
    }
    static int rtl8365mb_port_change_mtu(struct dsa_switch *ds, int port,
    int new_mtu)
    {
    struct realtek_priv *priv = ds.priv;
    int frame_size;
// When a new MTU is set, DSA always sets the CPU port's MTU to the
// largest MTU of the user ports. Because the switch only has a global
// RX length register, only allowing CPU port here is enough.
//
    if (!dsa_is_cpu_port(ds, port))
    return 0;
    frame_size = new_mtu + VLAN_ETH_HLEN + ETH_FCS_LEN;
    dev_dbg(priv.dev, "changing mtu to %d (frame size: %d)\n",
    new_mtu, frame_size);
    return regmap_update_bits(priv.map, RTL8365MB_CFG0_MAX_LEN_REG,
    RTL8365MB_CFG0_MAX_LEN_MASK,
    FIELD_PREP(RTL8365MB_CFG0_MAX_LEN_MASK,
    frame_size));
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_port_max_mtu(ds: *mut dsa_switch, port: c_int) -> c_int {
    static int rtl8365mb_port_max_mtu(struct dsa_switch *ds, int port)
    {
    return RTL8365MB_CFG0_MAX_LEN_MAX - VLAN_ETH_HLEN - ETH_FCS_LEN;
    }
    static void rtl8365mb_port_stp_state_set(struct dsa_switch *ds, int port,
    u8 state)
    {
    struct realtek_priv *priv = ds.priv;
    enum rtl8365mb_stp_state val;
    let mut msti: c_int = 0;
    switch (state) {
    case BR_STATE_DISABLED:
    val = RTL8365MB_STP_STATE_DISABLED;
    break;
    case BR_STATE_BLOCKING:
    case BR_STATE_LISTENING:
    val = RTL8365MB_STP_STATE_BLOCKING;
    break;
    case BR_STATE_LEARNING:
    val = RTL8365MB_STP_STATE_LEARNING;
    break;
    case BR_STATE_FORWARDING:
    val = RTL8365MB_STP_STATE_FORWARDING;
    break;
    default:
    dev_err(priv.dev, "invalid STP state: %u\n", state);
    return;
    }
    regmap_update_bits(priv.map, RTL8365MB_MSTI_CTRL_REG(msti, port),
    RTL8365MB_MSTI_CTRL_PORT_STATE_MASK(port),
    val << RTL8365MB_MSTI_CTRL_PORT_STATE_OFFSET(port));
    }
    static int rtl8365mb_port_set_transparent(struct realtek_priv *priv,
    int igr_port, int egr_port,
    bool enable)
    {
    dev_dbg(priv.dev, "%s transparent VLAN from %d to %d\n",
    enable ? "Enable" : "Disable", igr_port, egr_port);
// "Transparent" between the two ports means that packets forwarded by
// igr_port and egressed on egr_port will not be filtered by the usual
// VLAN membership settings.
//
    return regmap_update_bits(priv.map,
    RTL8365MB_VLAN_EGRESS_TRANSPARENT_REG(egr_port),
    BIT(igr_port), enable ? BIT(igr_port) : 0);
    }
    static int rtl8365mb_port_set_ingress_filtering(struct realtek_priv *priv,
    int port, bool enable)
    {
// Ingress filtering enabled: Discard VLAN-tagged frames if the port is
// not a member of the VLAN with which the packet is associated.
// Untagged packets will also be discarded unless the port has a PVID
// programmed. Priority-tagged frames are treated as untagged frames.
//
// Ingress filtering disabled: Accept all tagged and untagged frames.
//
    return regmap_update_bits(priv.map, RTL8365MB_VLAN_INGRESS_REG,
    RTL8365MB_VLAN_INGRESS_FILTER_PORT_EN_MASK(port),
    enable ?
    RTL8365MB_VLAN_INGRESS_FILTER_PORT_EN_MASK(port) :
    0);
    }
    static int
    rtl8365mb_port_set_vlan_egress_mode(struct realtek_priv *priv, int port,
    enum rtl8365mb_vlan_egress_mode mode)
    {
    u32 val;
    val = FIELD_PREP(RTL8365MB_PORT_MISC_CFG_VLAN_EGRESS_MODE_MASK, mode);
    return regmap_update_bits(priv.map,
    RTL8365MB_PORT_MISC_CFG_REG(port),
    RTL8365MB_PORT_MISC_CFG_VLAN_EGRESS_MODE_MASK, val);
    }
    static int rtl8365mb_port_vlan_filtering(struct dsa_switch *ds, int port,
    bool vlan_filtering,
    struct netlink_ext_ack *extack)
    {
    enum rtl8365mb_frame_ingress accepted_frame, prev_accepted_frame;
    enum rtl8365mb_vlan_egress_mode mode;
    struct realtek_priv *priv = ds.priv;
    let mut configured_ports: u32 = 0;
    struct dsa_port *dp;
    u16 pvid_vid;
    int ret;
    dev_dbg(priv.dev, "port %d: %s VLAN filtering\n", port,
    vlan_filtering ? "enable" : "disable");
    ret = rtl8365mb_vlan_port_get_framefilter(priv, port,
    &prev_accepted_frame);
    if (ret) {
    NL_SET_ERR_MSG_MOD(extack,
    "Failed to get current framefilter");
    return ret;
    }
// While filtering, only accepts untagged frames if PVID is enabled
    if (vlan_filtering) {
    ret = rtl8365mb_vlan_port_get_pvid(priv, port, &pvid_vid);
    if (ret)
    return ret;
    if (pvid_vid)
    accepted_frame = RTL8365MB_FRAME_TYPE_ANY_FRAME;
    else
    accepted_frame = RTL8365MB_FRAME_TYPE_TAGGED_ONLY;
    } else {
    accepted_frame = RTL8365MB_FRAME_TYPE_ANY_FRAME;
    }
// When vlan filter is enable/disabled in a bridge, this function is
// called for all member ports. We need to enable/disable ingress
// VLAN membership check.
//
    ret = rtl8365mb_port_set_ingress_filtering(priv, port, vlan_filtering);
    if (ret)
    return ret;
// However, we also enable/disable egress filtering because the switch
// still consider the egress interface VLAN membership to forward the
// traffic. We enable/disable that check disabling/enabling transparent
// VLAN between the ingress port and all other available ports.
//
    dsa_switch_for_each_available_port(dp, ds) {
// port isolation will still keep traffic inside the bridge
    ret = rtl8365mb_port_set_transparent(priv, port, dp.index,
    !vlan_filtering);
    if (ret)
    goto undo_transparent;
    configured_ports |= BIT(dp.index);
    }
    if (accepted_frame != prev_accepted_frame) {
    ret = rtl8365mb_vlan_port_set_framefilter(priv, port,
    accepted_frame);
    if (ret) {
    NL_SET_ERR_MSG_MOD(extack,
    "Failed to set port framefilter");
    goto undo_transparent;
    }
    }
// When VLAN filtering is disabled, preserve frames exactly as received.
// Otherwise, the VLAN egress pipeline may still alter tag state
// according to VLAN membership and untag configuration.
//
    if (vlan_filtering)
    mode = RTL8365MB_VLAN_EGRESS_MODE_ORIGINAL;
    else
    mode = RTL8365MB_VLAN_EGRESS_MODE_REAL_KEEP;
    ret = rtl8365mb_port_set_vlan_egress_mode(priv, port, mode);
    if (ret)
    goto undo_set_framefilter;
    return ret;
    undo_set_framefilter:
    if (prev_accepted_frame != accepted_frame)
    rtl8365mb_vlan_port_set_framefilter(priv, port,
    prev_accepted_frame);
    undo_transparent:
// The DSA core guarantees this callback is only invoked on an actual
// state transition, ensuring the previous hardware state was the
// opposite (!vlan_filtering). It is also called during setup but, in
// that case, any failure here aborts the entire switch initialization.
//
// VLAN_INGRESS and VLAN_EGRESS_TRANSPARENT states are directly derived
// from vlan_filtering. That way, we can simply undo it without
// checking the current HW state as we do with VLAN_EGRESS_MODE.
//
    dsa_switch_for_each_port(dp, ds) {
    if (configured_ports & BIT(dp.index))
    rtl8365mb_port_set_transparent(priv, port, dp.index,
    vlan_filtering);
    }
    rtl8365mb_port_set_ingress_filtering(priv, port, !vlan_filtering);
    return ret;
    }
    static int rtl8365mb_port_vlan_add(struct dsa_switch *ds, int port,
    const struct switchdev_obj_port_vlan *vlan,
    struct netlink_ext_ack *extack)
    {
    let mut untagged: bool = !!(vlan.flags & BRIDGE_VLAN_INFO_UNTAGGED);
    let mut pvid: bool = !!(vlan.flags & BRIDGE_VLAN_INFO_PVID);
    u16 pvid_vid;
    struct realtek_priv *priv = ds.priv;
    int ret;
    dev_dbg(priv.dev, "add VLAN %d on port %d, %s, %s\n",
    vlan.vid, port, untagged ? "untagged" : "tagged",
    pvid ? "PVID" : "no PVID");
// VID == 0 is reserved in this driver
    if (vlan.vid == 0) {
    NL_SET_ERR_MSG_MOD(extack,
    "VLAN 0 is reserved by this driver");
    return -EOPNOTSUPP;
    }
    mutex_lock(&priv.vlan_lock);
    ret = rtl8365mb_vlan_port_get_pvid(priv, port, &pvid_vid);
    if (ret)
    goto out_unlock;
// Set PVID if needed
    if (pvid) {
    ret = rtl8365mb_vlan_pvid_port_set(ds, port, vlan.vid,
    extack);
    if (ret)
    goto out_unlock;
    } else {
// or try to unset it if not
    ret = rtl8365mb_vlan_pvid_port_clear(ds, port, vlan.vid);
    if (ret)
    goto out_unlock;
    }
// add port to vlan4k. It knows nothing about PVID
    ret = rtl8365mb_vlan_4k_port_add(ds, port, vlan, extack);
    if (ret)
    goto undo_set_pvid;
    ret = 0;
    goto out_unlock;
    undo_set_pvid:
// undo the pvid definition
    if (pvid != (pvid_vid == vlan.vid)) {
    if (pvid_vid)
    (void)rtl8365mb_vlan_pvid_port_set(ds, port, pvid_vid,
    core::ptr::null_mut());
    else
    (void)rtl8365mb_vlan_pvid_port_clear(ds, port,
    vlan.vid);
    }
    out_unlock:
    mutex_unlock(&priv.vlan_lock);
    return ret;
    }
    static int rtl8365mb_port_vlan_del(struct dsa_switch *ds, int port,
    const struct switchdev_obj_port_vlan *vlan)
    {
    let mut untagged: bool = !!(vlan.flags & BRIDGE_VLAN_INFO_UNTAGGED);
    let mut pvid: bool = !!(vlan.flags & BRIDGE_VLAN_INFO_PVID);
    struct realtek_priv *priv = ds.priv;
    int ret;
    dev_dbg(priv.dev, "del VLAN %d on port %d, %s, %s\n",
    vlan.vid, port, untagged ? "untagged" : "tagged",
    pvid ? "PVID" : "no PVID");
// VID == 0 is reserved in this driver
    if (vlan.vid == 0)
    return -EOPNOTSUPP;
    mutex_lock(&priv.vlan_lock);
    ret = rtl8365mb_vlan_pvid_port_clear(ds, port, vlan.vid);
    if (ret)
    goto out_unlock;
    ret = rtl8365mb_vlan_4k_port_del(ds, port, vlan);
// There is little incentive to try to undo the removal of PVID (if it
// was really in use) as an error here might indicate the ASIC stopped
// to answer.
//
    out_unlock:
    mutex_unlock(&priv.vlan_lock);
    return ret;
    }
// VLAN support is always enabled in the switch.
//
// Standalone forwarding relies on transparent VLAN mode combined with per-port
// isolation masks restricting egress to CPU ports only.
//
#[no_mangle]
unsafe extern "C" fn rtl8365mb_vlan_setup(ds: *mut dsa_switch) -> c_int {
    static int rtl8365mb_vlan_setup(struct dsa_switch *ds)
    {
    struct realtek_priv *priv = ds.priv;
    struct dsa_port *dp;
    int ret;
    dsa_switch_for_each_available_port(dp, ds) {
// Disable vlan-filtering for all ports
    ret = rtl8365mb_port_vlan_filtering(ds, dp.index, false, core::ptr::null_mut());
    if (ret) {
    dev_err(priv.dev,
    "Failed to disable vlan filtering on port %d\n",
    dp.index);
    return ret;
    }
    }
// VLAN is always enabled.
    ret = regmap_update_bits(priv.map, RTL8365MB_VLAN_CTRL_REG,
    RTL8365MB_VLAN_CTRL_EN_MASK,
    FIELD_PREP(RTL8365MB_VLAN_CTRL_EN_MASK, 1));
    return ret;
    }
    static int rtl8365mb_port_set_learning(struct realtek_priv *priv, int port,
    bool enable)
    {
// Enable/disable learning by limiting the number of L2 addresses the
// port can learn. Realtek documentation states that a limit of zero
// disables learning. When enabling learning, set it to the chip's
// maximum.
//
    return regmap_write(priv.map, RTL8365MB_LUT_PORT_LEARN_LIMIT_REG(port),
    enable ? RTL8365MB_LEARN_LIMIT_MAX : 0);
    }
    static int rtl8365mb_port_set_ucast_flood(struct realtek_priv *priv, int port,
    bool enable)
    {
// Frames with unknown unicast DA will be flooded to a programmable
// port mask that by default includes all ports. Add or remove
// the specified port from this port mask accordingly.
//
    return regmap_update_bits(priv.map,
    RTL8365MB_UNKNOWN_UNICAST_FLOODING_PMASK_REG,
    BIT(port), enable ? BIT(port) : 0);
    }
    static int rtl8365mb_port_set_mcast_flood(struct realtek_priv *priv, int port,
    bool enable)
    {
    return regmap_update_bits(priv.map,
    RTL8365MB_UNKNOWN_MULTICAST_FLOODING_PMASK_REG,
    BIT(port), enable ? BIT(port) : 0);
    }
    static int rtl8365mb_port_set_bcast_flood(struct realtek_priv *priv, int port,
    bool enable)
    {
    return regmap_update_bits(priv.map,
    RTL8365MB_UNKNOWN_BROADCAST_FLOODING_PMASK_REG,
    BIT(port), enable ? BIT(port) : 0);
    }
    static int rtl8365mb_port_pre_bridge_flags(struct dsa_switch *ds, int port,
    struct switchdev_brport_flags flags,
    struct netlink_ext_ack *extack)
    {
    struct realtek_priv *priv = ds.priv;
    dev_dbg(priv.dev, "pre_bridge_flags port:%d flags:%lx supported:%lx\n",
    port, flags.mask, RTL8365MB_SUPPORTED_BRIDGE_FLAGS);
    if (flags.mask & ~RTL8365MB_SUPPORTED_BRIDGE_FLAGS)
    return -EINVAL;
    return 0;
    }
    static int rtl8365mb_port_set_efid(struct realtek_priv *priv, int port,
    u32 efid)
    {
    return regmap_update_bits(priv.map, RTL8365MB_PORT_EFID_REG(port),
    RTL8365MB_PORT_EFID_MASK(port),
    efid << RTL8365MB_PORT_EFID_OFFSET(port));
    }
// Port isolation manipulation functions.
//
// The port isolation register controls the forwarding mask of a given
// port. The switch will not forward packets ingressed on a given port
// to ports which are not enabled in its forwarding mask.
//
// The port forwarding mask has the highest priority in forwarding
// decisions. The only exception to this rule is when the switch
// receives a packet on its CPU port with ALLOW=0. In that case the TX
// field of the CPU tag will override the forwarding port mask.
//
    static int rtl8365mb_port_set_isolation(struct realtek_priv *priv, int port,
    u32 mask)
    {
    return regmap_write(priv.map, RTL8365MB_PORT_ISOLATION_REG(port),
    mask);
    }
    static int rtl8365mb_port_add_isolation(struct realtek_priv *priv, int port,
    u32 mask)
    {
    return regmap_update_bits(priv.map, RTL8365MB_PORT_ISOLATION_REG(port),
    mask, mask);
    }
    static int rtl8365mb_port_remove_isolation(struct realtek_priv *priv, int port,
    u32 mask)
    {
    return regmap_update_bits(priv.map, RTL8365MB_PORT_ISOLATION_REG(port),
    mask, 0);
    }
    static int rtl8365mb_mib_counter_read(struct realtek_priv *priv, int port,
    u32 offset, u32 length, u64 *mibvalue)
    {
    let mut tmpvalue: u64 = 0;
    u32 val;
    int ret;
    int i;
// The MIB address is an SRAM address. We request a particular address
// and then poll the control register before reading the value from some
// counter registers.
//
    ret = regmap_write(priv.map, RTL8365MB_MIB_ADDRESS_REG,
    RTL8365MB_MIB_ADDRESS(port, offset));
    if (ret)
    return ret;
// Poll for completion
    ret = regmap_read_poll_timeout(priv.map, RTL8365MB_MIB_CTRL0_REG, val,
    !(val & RTL8365MB_MIB_CTRL0_BUSY_MASK),
    10, 100);
    if (ret)
    return ret;
// Presumably this indicates a MIB counter read failure
    if (val & RTL8365MB_MIB_CTRL0_RESET_MASK)
    return -EIO;
// There are four MIB counter registers each holding a 16 bit word of a
// MIB counter. Depending on the offset, we should read from the upper
// two or lower two registers. In case the MIB counter is 4 words, we
// read from all four registers.
//
    if (length == 4)
    offset = 3;
    else
    offset = (offset + 1) % 4;
// Read the MIB counter 16 bits at a time
    for (i = 0; i < length; i++) {
    ret = regmap_read(priv.map,
    RTL8365MB_MIB_COUNTER_REG(offset - i), &val);
    if (ret)
    return ret;
    tmpvalue = ((tmpvalue) << 16) | (val & 0xFFFF);
    }
// Only commit the result if no error occurred
// mibvalue = tmpvalue;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_get_ethtool_stats(ds: *mut dsa_switch, port: c_int, data: *mut u64) {
    static void rtl8365mb_get_ethtool_stats(struct dsa_switch *ds, int port, u64 *data)
    {
    struct realtek_priv *priv = ds.priv;
    struct rtl8365mb *mb;
    int ret;
    int i;
    mb = priv.chip_data;
    mutex_lock(&mb.mib_lock);
    for (i = 0; i < RTL8365MB_MIB_END; i++) {
    struct rtl8365mb_mib_counter *mib = &rtl8365mb_mib_counters[i];
    ret = rtl8365mb_mib_counter_read(priv, port, mib.offset,
    mib.length, &data[i]);
    if (ret) {
    dev_err(priv.dev,
    "failed to read port %d counters: %pe\n", port,
    ERR_PTR(ret));
    break;
    }
    }
    mutex_unlock(&mb.mib_lock);
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_get_strings(ds: *mut dsa_switch, port: c_int, stringset: u32, data: *mut u8) {
    static void rtl8365mb_get_strings(struct dsa_switch *ds, int port, u32 stringset, u8 *data)
    {
    int i;
    if (stringset != ETH_SS_STATS)
    return;
    for (i = 0; i < RTL8365MB_MIB_END; i++) {
    struct rtl8365mb_mib_counter *mib = &rtl8365mb_mib_counters[i];
    ethtool_puts(&data, mib.name);
    }
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_get_sset_count(ds: *mut dsa_switch, port: c_int, sset: c_int) -> c_int {
    static int rtl8365mb_get_sset_count(struct dsa_switch *ds, int port, int sset)
    {
    if (sset != ETH_SS_STATS)
    return -EOPNOTSUPP;
    return RTL8365MB_MIB_END;
    }
    static void rtl8365mb_get_phy_stats(struct dsa_switch *ds, int port,
    struct ethtool_eth_phy_stats *phy_stats)
    {
    struct realtek_priv *priv = ds.priv;
    struct rtl8365mb_mib_counter *mib;
    struct rtl8365mb *mb;
    mb = priv.chip_data;
    mib = &rtl8365mb_mib_counters[RTL8365MB_MIB_dot3StatsSymbolErrors];
    mutex_lock(&mb.mib_lock);
    rtl8365mb_mib_counter_read(priv, port, mib.offset, mib.length,
    &phy_stats.SymbolErrorDuringCarrier);
    mutex_unlock(&mb.mib_lock);
    }
    static void rtl8365mb_get_mac_stats(struct dsa_switch *ds, int port,
    struct ethtool_eth_mac_stats *mac_stats)
    {
    u64 cnt[RTL8365MB_MIB_END] = {
    [RTL8365MB_MIB_ifOutOctets] = 1,
    [RTL8365MB_MIB_ifOutUcastPkts] = 1,
    [RTL8365MB_MIB_ifOutMulticastPkts] = 1,
    [RTL8365MB_MIB_ifOutBroadcastPkts] = 1,
    [RTL8365MB_MIB_dot3OutPauseFrames] = 1,
    [RTL8365MB_MIB_ifOutDiscards] = 1,
    [RTL8365MB_MIB_ifInOctets] = 1,
    [RTL8365MB_MIB_ifInUcastPkts] = 1,
    [RTL8365MB_MIB_ifInMulticastPkts] = 1,
    [RTL8365MB_MIB_ifInBroadcastPkts] = 1,
    [RTL8365MB_MIB_dot3InPauseFrames] = 1,
    [RTL8365MB_MIB_dot3StatsSingleCollisionFrames] = 1,
    [RTL8365MB_MIB_dot3StatsMultipleCollisionFrames] = 1,
    [RTL8365MB_MIB_dot3StatsFCSErrors] = 1,
    [RTL8365MB_MIB_dot3StatsDeferredTransmissions] = 1,
    [RTL8365MB_MIB_dot3StatsLateCollisions] = 1,
    [RTL8365MB_MIB_dot3StatsExcessiveCollisions] = 1,
    };
    struct realtek_priv *priv = ds.priv;
    struct rtl8365mb *mb;
    int ret;
    int i;
    mb = priv.chip_data;
    mutex_lock(&mb.mib_lock);
    for (i = 0; i < RTL8365MB_MIB_END; i++) {
    struct rtl8365mb_mib_counter *mib = &rtl8365mb_mib_counters[i];
// Only fetch required MIB counters (marked = 1 above)
    if (!cnt[i])
    continue;
    ret = rtl8365mb_mib_counter_read(priv, port, mib.offset,
    mib.length, &cnt[i]);
    if (ret)
    break;
    }
    mutex_unlock(&mb.mib_lock);
// The RTL8365MB-VC exposes MIB objects, which we have to translate into
// IEEE 802.3 Managed Objects. This is not always completely faithful,
// but we try out best. See RFC 3635 for a detailed treatment of the
// subject.
//
    mac_stats.FramesTransmittedOK = cnt[RTL8365MB_MIB_ifOutUcastPkts] +
    cnt[RTL8365MB_MIB_ifOutMulticastPkts] +
    cnt[RTL8365MB_MIB_ifOutBroadcastPkts] +
    cnt[RTL8365MB_MIB_dot3OutPauseFrames] -
    cnt[RTL8365MB_MIB_ifOutDiscards];
    mac_stats.SingleCollisionFrames =
    cnt[RTL8365MB_MIB_dot3StatsSingleCollisionFrames];
    mac_stats.MultipleCollisionFrames =
    cnt[RTL8365MB_MIB_dot3StatsMultipleCollisionFrames];
    mac_stats.FramesReceivedOK = cnt[RTL8365MB_MIB_ifInUcastPkts] +
    cnt[RTL8365MB_MIB_ifInMulticastPkts] +
    cnt[RTL8365MB_MIB_ifInBroadcastPkts] +
    cnt[RTL8365MB_MIB_dot3InPauseFrames];
    mac_stats.FrameCheckSequenceErrors =
    cnt[RTL8365MB_MIB_dot3StatsFCSErrors];
    mac_stats.OctetsTransmittedOK = cnt[RTL8365MB_MIB_ifOutOctets] -
    18 * mac_stats.FramesTransmittedOK;
    mac_stats.FramesWithDeferredXmissions =
    cnt[RTL8365MB_MIB_dot3StatsDeferredTransmissions];
    mac_stats.LateCollisions = cnt[RTL8365MB_MIB_dot3StatsLateCollisions];
    mac_stats.FramesAbortedDueToXSColls =
    cnt[RTL8365MB_MIB_dot3StatsExcessiveCollisions];
    mac_stats.OctetsReceivedOK = cnt[RTL8365MB_MIB_ifInOctets] -
    18 * mac_stats.FramesReceivedOK;
    mac_stats.MulticastFramesXmittedOK =
    cnt[RTL8365MB_MIB_ifOutMulticastPkts];
    mac_stats.BroadcastFramesXmittedOK =
    cnt[RTL8365MB_MIB_ifOutBroadcastPkts];
    mac_stats.MulticastFramesReceivedOK =
    cnt[RTL8365MB_MIB_ifInMulticastPkts];
    mac_stats.BroadcastFramesReceivedOK =
    cnt[RTL8365MB_MIB_ifInBroadcastPkts];
    }
    static void rtl8365mb_get_ctrl_stats(struct dsa_switch *ds, int port,
    struct ethtool_eth_ctrl_stats *ctrl_stats)
    {
    struct realtek_priv *priv = ds.priv;
    struct rtl8365mb_mib_counter *mib;
    struct rtl8365mb *mb;
    mb = priv.chip_data;
    mib = &rtl8365mb_mib_counters[RTL8365MB_MIB_dot3ControlInUnknownOpcodes];
    mutex_lock(&mb.mib_lock);
    rtl8365mb_mib_counter_read(priv, port, mib.offset, mib.length,
    &ctrl_stats.UnsupportedOpcodesReceived);
    mutex_unlock(&mb.mib_lock);
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_stats_update(priv: *mut realtek_priv, port: c_int) {
    static void rtl8365mb_stats_update(struct realtek_priv *priv, int port)
    {
    u64 cnt[RTL8365MB_MIB_END] = {
    [RTL8365MB_MIB_ifOutOctets] = 1,
    [RTL8365MB_MIB_ifOutUcastPkts] = 1,
    [RTL8365MB_MIB_ifOutMulticastPkts] = 1,
    [RTL8365MB_MIB_ifOutBroadcastPkts] = 1,
    [RTL8365MB_MIB_ifOutDiscards] = 1,
    [RTL8365MB_MIB_ifInOctets] = 1,
    [RTL8365MB_MIB_ifInUcastPkts] = 1,
    [RTL8365MB_MIB_ifInMulticastPkts] = 1,
    [RTL8365MB_MIB_ifInBroadcastPkts] = 1,
    [RTL8365MB_MIB_etherStatsDropEvents] = 1,
    [RTL8365MB_MIB_etherStatsCollisions] = 1,
    [RTL8365MB_MIB_etherStatsFragments] = 1,
    [RTL8365MB_MIB_etherStatsJabbers] = 1,
    [RTL8365MB_MIB_dot3StatsFCSErrors] = 1,
    [RTL8365MB_MIB_dot3StatsLateCollisions] = 1,
    };
    struct rtl8365mb *mb = priv.chip_data;
    struct rtnl_link_stats64 *stats;
    int ret;
    int i;
    stats = &mb.ports[port].stats;
    mutex_lock(&mb.mib_lock);
    for (i = 0; i < RTL8365MB_MIB_END; i++) {
    struct rtl8365mb_mib_counter *c = &rtl8365mb_mib_counters[i];
// Only fetch required MIB counters (marked = 1 above)
    if (!cnt[i])
    continue;
    ret = rtl8365mb_mib_counter_read(priv, port, c.offset,
    c.length, &cnt[i]);
    if (ret)
    break;
    }
    mutex_unlock(&mb.mib_lock);
// Don't update statistics if there was an error reading the counters
    if (ret)
    return;
    spin_lock(&mb.ports[port].stats_lock);
    stats.rx_packets = cnt[RTL8365MB_MIB_ifInUcastPkts] +
    cnt[RTL8365MB_MIB_ifInMulticastPkts] +
    cnt[RTL8365MB_MIB_ifInBroadcastPkts];
    stats.tx_packets = cnt[RTL8365MB_MIB_ifOutUcastPkts] +
    cnt[RTL8365MB_MIB_ifOutMulticastPkts] +
    cnt[RTL8365MB_MIB_ifOutBroadcastPkts];
// if{In,Out}Octets includes FCS - remove it
    stats.rx_bytes = cnt[RTL8365MB_MIB_ifInOctets] - 4 * stats.rx_packets;
    stats.tx_bytes =
    cnt[RTL8365MB_MIB_ifOutOctets] - 4 * stats.tx_packets;
    stats.rx_dropped = cnt[RTL8365MB_MIB_etherStatsDropEvents];
    stats.tx_dropped = cnt[RTL8365MB_MIB_ifOutDiscards];
    stats.multicast = cnt[RTL8365MB_MIB_ifInMulticastPkts];
    stats.collisions = cnt[RTL8365MB_MIB_etherStatsCollisions];
    stats.rx_length_errors = cnt[RTL8365MB_MIB_etherStatsFragments] +
    cnt[RTL8365MB_MIB_etherStatsJabbers];
    stats.rx_crc_errors = cnt[RTL8365MB_MIB_dot3StatsFCSErrors];
    stats.rx_errors = stats.rx_length_errors + stats.rx_crc_errors;
    stats.tx_aborted_errors = cnt[RTL8365MB_MIB_ifOutDiscards];
    stats.tx_window_errors = cnt[RTL8365MB_MIB_dot3StatsLateCollisions];
    stats.tx_errors = stats.tx_aborted_errors + stats.tx_window_errors;
    spin_unlock(&mb.ports[port].stats_lock);
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_stats_poll(work: *mut work_struct) {
    static void rtl8365mb_stats_poll(struct work_struct *work)
    {
    struct rtl8365mb_port *p = container_of(to_delayed_work(work),
    struct rtl8365mb_port,
    mib_work);
    struct realtek_priv *priv = p.priv;
    rtl8365mb_stats_update(priv, p.index);
    schedule_delayed_work(&p.mib_work, RTL8365MB_STATS_INTERVAL_JIFFIES);
    }
    static void rtl8365mb_get_stats64(struct dsa_switch *ds, int port,
    struct rtnl_link_stats64 *s)
    {
    struct realtek_priv *priv = ds.priv;
    struct rtl8365mb_port *p;
    struct rtl8365mb *mb;
    mb = priv.chip_data;
    p = &mb.ports[port];
    spin_lock(&p.stats_lock);
    memcpy(s, &p.stats, sizeof(*s));
    spin_unlock(&p.stats_lock);
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_stats_setup(priv: *mut realtek_priv) -> c_int {
    static int rtl8365mb_stats_setup(struct realtek_priv *priv)
    {
    struct rtl8365mb *mb = priv.chip_data;
    struct dsa_switch *ds = &priv.ds;
    struct dsa_port *dp;
    int ret;
// Per-chip global mutex to protect MIB counter access, since doing
// so requires accessing a series of registers in a particular order.
//
    ret = devm_mutex_init(priv.dev, &mb.mib_lock);
    if (ret)
    return ret;
    dsa_switch_for_each_available_port(dp, ds) {
    struct rtl8365mb_port *p = &mb.ports[dp.index];
// Per-port spinlock to protect the stats64 data
    spin_lock_init(&p.stats_lock);
// This work polls the MIB counters and keeps the stats64 data
// up-to-date.
//
    INIT_DELAYED_WORK(&p.mib_work, rtl8365mb_stats_poll);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_stats_teardown(priv: *mut realtek_priv) {
    static void rtl8365mb_stats_teardown(struct realtek_priv *priv)
    {
    struct rtl8365mb *mb = priv.chip_data;
    struct dsa_switch *ds = &priv.ds;
    struct dsa_port *dp;
    dsa_switch_for_each_available_port(dp, ds) {
    struct rtl8365mb_port *p = &mb.ports[dp.index];
    cancel_delayed_work_sync(&p.mib_work);
    }
    }
    static int rtl8365mb_get_and_clear_status_reg(struct realtek_priv *priv, u32 reg,
    u32 *val)
    {
    int ret;
    ret = regmap_read(priv.map, reg, val);
    if (ret)
    return ret;
    return regmap_write(priv.map, reg, *val);
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t rtl8365mb_irq(int irq, void *data)
    {
    struct realtek_priv *priv = data;
    let mut line_changes: c_ulong = 0;
    u32 stat;
    int line;
    int ret;
    ret = rtl8365mb_get_and_clear_status_reg(priv, RTL8365MB_INTR_STATUS_REG,
    &stat);
    if (ret)
    goto out_error;
    if (stat & RTL8365MB_INTR_LINK_CHANGE_MASK) {
    u32 linkdown_ind;
    u32 linkup_ind;
    u32 val;
    ret = rtl8365mb_get_and_clear_status_reg(
    priv, RTL8365MB_PORT_LINKUP_IND_REG, &val);
    if (ret)
    goto out_error;
    linkup_ind = FIELD_GET(RTL8365MB_PORT_LINKUP_IND_MASK, val);
    ret = rtl8365mb_get_and_clear_status_reg(
    priv, RTL8365MB_PORT_LINKDOWN_IND_REG, &val);
    if (ret)
    goto out_error;
    linkdown_ind = FIELD_GET(RTL8365MB_PORT_LINKDOWN_IND_MASK, val);
    line_changes = linkup_ind | linkdown_ind;
    }
    if (!line_changes)
    goto out_none;
    for_each_set_bit(line, &line_changes, priv.num_ports) {
    let mut child_irq: c_int = irq_find_mapping(priv.irqdomain, line);
    if (!child_irq)
    continue;
    handle_nested_irq(child_irq);
    }
    return IRQ_HANDLED;
    out_error:
    dev_err(priv.dev, "failed to read interrupt status: %pe\n",
    ERR_PTR(ret));
    out_none:
    return IRQ_NONE;
    }
    static struct irq_chip rtl8365mb_irq_chip = {
    .name = "rtl8365mb",
// The hardware doesn't support masking IRQs on a per-port basis
    };
    static int rtl8365mb_irq_map(struct irq_domain *domain, unsigned int irq,
    irq_hw_number_t hwirq)
    {
    struct realtek_priv *priv = domain.host_data;
    struct rtl8365mb *mb = priv.chip_data;
    irq_set_chip_data(irq, priv);
    irq_set_chip_and_handler(irq, &rtl8365mb_irq_chip, handle_simple_irq);
    irq_set_nested_thread(irq, 1);
    irq_set_noprobe(irq);
    irq_set_parent(irq, mb.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_irq_unmap(d: *mut irq_domain, irq: c_uint) {
    static void rtl8365mb_irq_unmap(struct irq_domain *d, unsigned int irq)
    {
    irq_set_nested_thread(irq, 0);
    irq_set_chip_and_handler(irq, core::ptr::null_mut(), core::ptr::null_mut());
    irq_set_chip_data(irq, core::ptr::null_mut());
    }
    static const struct irq_domain_ops rtl8365mb_irqdomain_ops = {
    .map = rtl8365mb_irq_map,
    .unmap = rtl8365mb_irq_unmap,
    .xlate = irq_domain_xlate_onecell,
    };
#[no_mangle]
unsafe extern "C" fn rtl8365mb_set_irq_enable(priv: *mut realtek_priv, enable: bool) -> c_int {
    static int rtl8365mb_set_irq_enable(struct realtek_priv *priv, bool enable)
    {
    return regmap_update_bits(priv.map, RTL8365MB_INTR_CTRL_REG,
    RTL8365MB_INTR_LINK_CHANGE_MASK,
    FIELD_PREP(RTL8365MB_INTR_LINK_CHANGE_MASK,
    enable ? 1 : 0));
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_irq_enable(priv: *mut realtek_priv) -> c_int {
    static int rtl8365mb_irq_enable(struct realtek_priv *priv)
    {
    return rtl8365mb_set_irq_enable(priv, true);
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_irq_disable(priv: *mut realtek_priv) -> c_int {
    static int rtl8365mb_irq_disable(struct realtek_priv *priv)
    {
    return rtl8365mb_set_irq_enable(priv, false);
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_irq_setup(priv: *mut realtek_priv) -> c_int {
    static int rtl8365mb_irq_setup(struct realtek_priv *priv)
    {
    struct rtl8365mb *mb = priv.chip_data;
    struct dsa_switch *ds = &priv.ds;
    struct device_node *intc;
    struct dsa_port *dp;
    u32 irq_trig;
    int virq;
    int irq;
    u32 val;
    int ret;
    intc = of_get_child_by_name(priv.dev.of_node, "interrupt-controller");
    if (!intc) {
    dev_err(priv.dev, "missing child interrupt-controller node\n");
    return -EINVAL;
    }
// rtl8365mb IRQs cascade off this one
    irq = of_irq_get(intc, 0);
    if (irq <= 0) {
    if (!irq) {
    dev_err(priv.dev, "failed to map IRQ\n");
    ret = -EINVAL;
    } else {
    ret = dev_err_probe(priv.dev, irq,
    "failed to get parent irq\n");
    }
    goto out_put_node;
    }
// Store the irq so that we know to map and free it during teardown
    mb.irq = irq;
    priv.irqdomain = irq_domain_create_linear(of_fwnode_handle(intc), priv.num_ports,
    &rtl8365mb_irqdomain_ops, priv);
    if (!priv.irqdomain) {
    dev_err(priv.dev, "failed to add irq domain\n");
    ret = -ENOMEM;
    goto out_put_node;
    }
    dsa_switch_for_each_available_port(dp, ds) {
    virq = irq_create_mapping(priv.irqdomain, dp.index);
    if (!virq) {
    dev_err(priv.dev,
    "failed to create irq domain mapping\n");
    ret = -EINVAL;
    goto out_remove_irqdomain;
    }
    irq_set_parent(virq, irq);
    }
// Configure chip interrupt signal polarity
    irq_trig = irq_get_trigger_type(irq);
    switch (irq_trig) {
    case IRQF_TRIGGER_RISING:
    case IRQF_TRIGGER_HIGH:
    val = RTL8365MB_INTR_POLARITY_HIGH;
    break;
    case IRQF_TRIGGER_FALLING:
    case IRQF_TRIGGER_LOW:
    val = RTL8365MB_INTR_POLARITY_LOW;
    break;
    default:
    dev_err(priv.dev, "unsupported irq trigger type %u\n",
    irq_trig);
    ret = -EINVAL;
    goto out_remove_irqdomain;
    }
    ret = regmap_update_bits(priv.map, RTL8365MB_INTR_POLARITY_REG,
    RTL8365MB_INTR_POLARITY_MASK,
    FIELD_PREP(RTL8365MB_INTR_POLARITY_MASK, val));
    if (ret)
    goto out_remove_irqdomain;
// Disable the interrupt in case the chip has it enabled on reset
    ret = rtl8365mb_irq_disable(priv);
    if (ret)
    goto out_remove_irqdomain;
// Clear the interrupt status register
    ret = regmap_write(priv.map, RTL8365MB_INTR_STATUS_REG,
    RTL8365MB_INTR_ALL_MASK);
    if (ret)
    goto out_remove_irqdomain;
    ret = request_threaded_irq(irq, core::ptr::null_mut(), rtl8365mb_irq, IRQF_ONESHOT,
    "rtl8365mb", priv);
    if (ret) {
    dev_err(priv.dev, "failed to request irq: %pe\n",
    ERR_PTR(ret));
    goto out_remove_irqdomain;
    }
    ret = rtl8365mb_irq_enable(priv);
    if (ret)
    goto out_free_irq;
    of_node_put(intc);
    return 0;
    out_free_irq:
    free_irq(mb.irq, priv);
    out_remove_irqdomain:
    dsa_switch_for_each_port(dp, ds) {
    virq = irq_find_mapping(priv.irqdomain, dp.index);
    if (virq)
    irq_dispose_mapping(virq);
    }
    irq_domain_remove(priv.irqdomain);
    priv.irqdomain = core::ptr::null_mut();
    out_put_node:
    mb.irq = 0;
    of_node_put(intc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_irq_teardown(priv: *mut realtek_priv) {
    static void rtl8365mb_irq_teardown(struct realtek_priv *priv)
    {
    struct rtl8365mb *mb = priv.chip_data;
    struct dsa_switch *ds = &priv.ds;
    struct dsa_port *dp;
    int virq;
    if (mb.irq) {
    free_irq(mb.irq, priv);
    mb.irq = 0;
    }
    if (priv.irqdomain) {
// Unused ports with a linked PHY still have an active IRQ
// mapping that must be disposed of during teardown. Loop
// through all ports.
//
    dsa_switch_for_each_port(dp, ds) {
    virq = irq_find_mapping(priv.irqdomain, dp.index);
    if (virq)
    irq_dispose_mapping(virq);
    }
    irq_domain_remove(priv.irqdomain);
    priv.irqdomain = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_cpu_config(priv: *mut realtek_priv) -> c_int {
    static int rtl8365mb_cpu_config(struct realtek_priv *priv)
    {
    struct rtl8365mb *mb = priv.chip_data;
    struct rtl8365mb_cpu *cpu = &mb.cpu;
    u32 val;
    int ret;
    ret = regmap_update_bits(priv.map, RTL8365MB_CPU_PORT_MASK_REG,
    RTL8365MB_CPU_PORT_MASK_MASK,
    FIELD_PREP(RTL8365MB_CPU_PORT_MASK_MASK,
    cpu.mask));
    if (ret)
    return ret;
    val = FIELD_PREP(RTL8365MB_CPU_CTRL_EN_MASK, cpu.enable ? 1 : 0) |
    FIELD_PREP(RTL8365MB_CPU_CTRL_INSERTMODE_MASK, cpu.insert) |
    FIELD_PREP(RTL8365MB_CPU_CTRL_TAG_POSITION_MASK, cpu.position) |
    FIELD_PREP(RTL8365MB_CPU_CTRL_RXBYTECOUNT_MASK, cpu.rx_length) |
    FIELD_PREP(RTL8365MB_CPU_CTRL_TAG_FORMAT_MASK, cpu.format) |
    FIELD_PREP(RTL8365MB_CPU_CTRL_TRAP_PORT_MASK, cpu.trap_port & 0x7) |
    FIELD_PREP(RTL8365MB_CPU_CTRL_TRAP_PORT_EXT_MASK,
    cpu.trap_port >> 3 & 0x1);
    ret = regmap_write(priv.map, RTL8365MB_CPU_CTRL_REG, val);
    if (ret)
    return ret;
    return 0;
    }
    static int rtl8365mb_change_tag_protocol(struct dsa_switch *ds,
    enum dsa_tag_protocol proto)
    {
    struct realtek_priv *priv = ds.priv;
    struct rtl8365mb_cpu *cpu;
    struct rtl8365mb *mb;
    mb = priv.chip_data;
    cpu = &mb.cpu;
    switch (proto) {
    case DSA_TAG_PROTO_RTL8_4:
    cpu.format = RTL8365MB_CPU_FORMAT_8BYTES;
    cpu.position = RTL8365MB_CPU_POS_AFTER_SA;
    break;
    case DSA_TAG_PROTO_RTL8_4T:
    cpu.format = RTL8365MB_CPU_FORMAT_8BYTES;
    cpu.position = RTL8365MB_CPU_POS_BEFORE_CRC;
    break;
// The switch also supports a 4-byte format, similar to rtl4a but with
// the same 0x04 8-bit version and probably 8-bit port source/dest.
// There is no public doc about it. Not supported yet and it will probably
// never be.
//
    default:
    return -EPROTONOSUPPORT;
    }
    return rtl8365mb_cpu_config(priv);
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_switch_init(priv: *mut realtek_priv) -> c_int {
    static int rtl8365mb_switch_init(struct realtek_priv *priv)
    {
    struct rtl8365mb *mb = priv.chip_data;
    const struct rtl8365mb_chip_info *ci;
    int ret;
    int i;
    ci = mb.chip_info;
// Do any chip-specific init jam before getting to the common stuff
    if (ci.jam_table) {
    for (i = 0; i < ci.jam_size; i++) {
    ret = regmap_write(priv.map, ci.jam_table[i].reg,
    ci.jam_table[i].val);
    if (ret)
    return ret;
    }
    }
// Common init jam
    for (i = 0; i < ARRAY_SIZE(rtl8365mb_init_jam_common); i++) {
    ret = regmap_write(priv.map, rtl8365mb_init_jam_common[i].reg,
    rtl8365mb_init_jam_common[i].val);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_reset_chip(priv: *mut realtek_priv) -> c_int {
    static int rtl8365mb_reset_chip(struct realtek_priv *priv)
    {
    u32 val;
    priv.write_reg_noack(priv, RTL8365MB_CHIP_RESET_REG,
    FIELD_PREP(RTL8365MB_CHIP_RESET_HW_MASK, 1));
// Realtek documentation says the chip needs 1 second to reset. Sleep
// for 100 ms before accessing any registers to prevent ACK timeouts.
//
    msleep(100);
    return regmap_read_poll_timeout(priv.map, RTL8365MB_CHIP_RESET_REG, val,
    !(val & RTL8365MB_CHIP_RESET_HW_MASK),
    20000, 1e6);
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_setup(ds: *mut dsa_switch) -> c_int {
    static int rtl8365mb_setup(struct dsa_switch *ds)
    {
    struct realtek_priv *priv = ds.priv;
    struct rtl8365mb_cpu *cpu;
    let mut downports_mask: u32 = 0;
    let mut upports_mask: u32 = 0;
    struct rtl8365mb *mb;
    struct dsa_port *dp;
    int ret;
    mb = priv.chip_data;
    cpu = &mb.cpu;
    mb.pcs.ops = &rtl8365mb_pcs_ops;
// The SerDes has no link interrupt wired up, so phylink must poll the
// PCS for link changes when it tracks the link through pcs_get_state()
// (in-band mode with autonegotiation disabled).
//
    mb.pcs.poll = true;
    ret = rtl8365mb_reset_chip(priv);
    if (ret) {
    dev_err(priv.dev, "failed to reset chip: %pe\n",
    ERR_PTR(ret));
    goto out_error;
    }
    ret = rtl8365mb_sds_probe_option(priv);
    if (ret) {
    dev_err(priv.dev, "failed to probe SerDes chip option: %pe\n",
    ERR_PTR(ret));
    goto out_error;
    }
// Configure switch to vendor-defined initial state
    ret = rtl8365mb_switch_init(priv);
    if (ret) {
    dev_err(priv.dev, "failed to initialize switch: %pe\n",
    ERR_PTR(ret));
    goto out_error;
    }
    if (mb.sds_supported) {
    ret = rtl8365mb_sds_raise_rate_limits(priv);
    if (ret) {
    dev_err(priv.dev,
    "failed to raise port rate limits: %pe\n",
    ERR_PTR(ret));
    goto out_error;
    }
    }
// Set up cascading IRQs
    ret = rtl8365mb_irq_setup(priv);
    if (ret == -EPROBE_DEFER)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ret) -> else {
    else if (ret)
    dev_info(priv.dev, "no interrupt support\n");
    dsa_switch_for_each_port(dp, ds) {
// Cascading (DSA links) is not supported yet.
// Historically, the driver has always been broken
// without a dedicated CPU port because CPU tagging
// would be disabled, rendering the switch entirely
// non-functional for DSA operations.
//
    if (dsa_port_is_dsa(dp)) {
    dev_err(priv.dev, "Cascading (DSA link) not supported\n");
    ret = -EOPNOTSUPP;
    goto out_teardown_irq;
    }
    }
// Start with all ports blocked, including unused ports
    dsa_switch_for_each_port(dp, ds) {
    struct rtl8365mb_port *p = &mb.ports[dp.index];
// Set the initial STP state of all ports to DISABLED, otherwise
// ports will still forward frames to the CPU despite being
// administratively down by default.
//
    rtl8365mb_port_stp_state_set(ds, dp.index, BR_STATE_DISABLED);
// Start with all port completely isolated
    ret = rtl8365mb_port_set_isolation(priv, dp.index, 0);
    if (ret)
    goto out_teardown_irq;
// Set the default EFID 0 for standalone mode
    ret = rtl8365mb_port_set_efid(priv, dp.index, 0);
    if (ret)
    goto out_teardown_irq;
// Disable learning
    ret = rtl8365mb_port_set_learning(priv, dp.index, false);
    if (ret)
    goto out_teardown_irq;
// Enable all types of flooding
    ret = rtl83xx_setup_port_flood_control(priv, dp.index);
    if (ret)
    goto out_teardown_irq;
// Set up per-port private data
    p.priv = priv;
    p.index = dp.index;
// Collect CPU ports. If we support cascade switches, it should
// also include the upstream DSA ports.
//
    if (!dsa_port_is_cpu(dp))
    continue;
    upports_mask |= BIT(dp.index);
    }
// Configure user ports
    dsa_switch_for_each_port(dp, ds) {
    if (!dsa_port_is_user(dp))
    continue;
// Forward only to the CPU
    ret = rtl8365mb_port_set_isolation(priv, dp.index,
    upports_mask);
    if (ret)
    goto out_teardown_irq;
// If we support cascade switches, it should also include the
// downstream DSA ports.
//
    downports_mask |= BIT(dp.index);
    }
// Configure CPU tagging
// If we support cascade switches, it should also include the upstream
// DSA ports.
//
    dsa_switch_for_each_cpu_port(dp, ds) {
// Use the first CPU port as trap_port
    if (cpu.trap_port == RTL8365MB_MAX_NUM_PORTS)
    cpu.trap_port = dp.index;
// Forward to all user ports
    ret = rtl8365mb_port_set_isolation(priv, dp.index,
    downports_mask);
    if (ret)
    goto out_teardown_irq;
    }
    cpu.mask = upports_mask;
    cpu.enable = cpu.mask > 0;
    if (!cpu.enable) {
    dev_err(priv.dev, "no CPU port defined\n");
    ret = -EINVAL;
    goto out_teardown_irq;
    }
    ret = rtl8365mb_cpu_config(priv);
    if (ret)
    goto out_teardown_irq;
    ret = rtl8365mb_port_change_mtu(ds, cpu.trap_port, ETH_DATA_LEN);
    if (ret)
    goto out_teardown_irq;
    ds.assisted_learning_on_cpu_port = true;
    ds.fdb_isolation = true;
// The EFID is 3 bits, but EFID 0 is reserved for standalone ports
    ds.max_num_bridges = FIELD_MAX(RTL8365MB_EFID_MASK);
    ds.configure_vlan_while_not_filtering = true;
// Set up VLAN
    ret = rtl8365mb_vlan_setup(ds);
    if (ret)
    goto out_teardown_irq;
    ret = rtl83xx_setup_user_mdio(ds);
    if (ret) {
    dev_err(priv.dev, "could not set up MDIO bus\n");
    goto out_teardown_irq;
    }
// Start statistics counter polling
    ret = rtl8365mb_stats_setup(priv);
    if (ret) {
    dev_err(priv.dev, "failed to setup stats: %pe\n",
    ERR_PTR(ret));
    goto out_teardown_irq;
    }
    return 0;
    out_teardown_irq:
    rtl8365mb_irq_teardown(priv);
    out_error:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_teardown(ds: *mut dsa_switch) {
    static void rtl8365mb_teardown(struct dsa_switch *ds)
    {
    struct realtek_priv *priv = ds.priv;
    rtl8365mb_stats_teardown(priv);
    rtl8365mb_irq_teardown(priv);
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_get_chip_id_and_ver(map: *mut regmap, id: *mut u32, ver: *mut u32) -> c_int {
    static int rtl8365mb_get_chip_id_and_ver(struct regmap *map, u32 *id, u32 *ver)
    {
    int ret;
// For some reason we have to write a magic value to an arbitrary
// register whenever accessing the chip ID/version registers.
//
    ret = regmap_write(map, RTL8365MB_MAGIC_REG, RTL8365MB_MAGIC_VALUE);
    if (ret)
    return ret;
    ret = regmap_read(map, RTL8365MB_CHIP_ID_REG, id);
    if (ret)
    return ret;
    ret = regmap_read(map, RTL8365MB_CHIP_VER_REG, ver);
    if (ret)
    return ret;
// Reset magic register
    ret = regmap_write(map, RTL8365MB_MAGIC_REG, 0);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8365mb_detect(priv: *mut realtek_priv) -> c_int {
    static int rtl8365mb_detect(struct realtek_priv *priv)
    {
    struct rtl8365mb *mb = priv.chip_data;
    u32 chip_id;
    u32 chip_ver;
    int ret;
    int i;
    ret = rtl8365mb_get_chip_id_and_ver(priv.map, &chip_id, &chip_ver);
    if (ret) {
    dev_err(priv.dev, "failed to read chip id and version: %pe\n",
    ERR_PTR(ret));
    return ret;
    }
    for (i = 0; i < ARRAY_SIZE(rtl8365mb_chip_infos); i++) {
    const struct rtl8365mb_chip_info *ci = &rtl8365mb_chip_infos[i];
    if (ci.chip_id == chip_id && ci.chip_ver == chip_ver) {
    mb.chip_info = ci;
    break;
    }
    }
    if (!mb.chip_info) {
    dev_err(priv.dev,
    "unrecognized switch (id=0x%04x, ver=0x%04x)", chip_id,
    chip_ver);
    return -ENODEV;
    }
    dev_info(priv.dev, "found an %s switch\n", mb.chip_info.name);
    priv.num_ports = RTL8365MB_MAX_NUM_PORTS;
    mb.priv = priv;
    mb.cpu.trap_port = RTL8365MB_MAX_NUM_PORTS;
    mb.cpu.insert = RTL8365MB_CPU_INSERT_TO_ALL;
    mb.cpu.position = RTL8365MB_CPU_POS_AFTER_SA;
    mb.cpu.rx_length = RTL8365MB_CPU_RXLEN_64BYTES;
    mb.cpu.format = RTL8365MB_CPU_FORMAT_8BYTES;
    return 0;
    }
    static const struct phylink_mac_ops rtl8365mb_phylink_mac_ops = {
    .mac_select_pcs = rtl8365mb_phylink_mac_select_pcs,
    .mac_config = rtl8365mb_phylink_mac_config,
    .mac_link_down = rtl8365mb_phylink_mac_link_down,
    .mac_link_up = rtl8365mb_phylink_mac_link_up,
    };
    static const struct dsa_switch_ops rtl8365mb_switch_ops = {
    .get_tag_protocol = rtl8365mb_get_tag_protocol,
    .change_tag_protocol = rtl8365mb_change_tag_protocol,
    .setup = rtl8365mb_setup,
    .teardown = rtl8365mb_teardown,
    .phylink_get_caps = rtl8365mb_phylink_get_caps,
    .port_bridge_join = rtl83xx_port_bridge_join,
    .port_bridge_leave = rtl83xx_port_bridge_leave,
    .port_pre_bridge_flags = rtl8365mb_port_pre_bridge_flags,
    .port_bridge_flags = rtl83xx_port_bridge_flags,
    .port_stp_state_set = rtl8365mb_port_stp_state_set,
    .port_fast_age = rtl83xx_port_fast_age,
    .port_fdb_add = rtl83xx_port_fdb_add,
    .port_fdb_del = rtl83xx_port_fdb_del,
    .port_fdb_dump = rtl83xx_port_fdb_dump,
    .port_mdb_add = rtl83xx_port_mdb_add,
    .port_mdb_del = rtl83xx_port_mdb_del,
    .port_vlan_add = rtl8365mb_port_vlan_add,
    .port_vlan_del = rtl8365mb_port_vlan_del,
    .port_vlan_filtering = rtl8365mb_port_vlan_filtering,
    .get_strings = rtl8365mb_get_strings,
    .get_ethtool_stats = rtl8365mb_get_ethtool_stats,
    .get_sset_count = rtl8365mb_get_sset_count,
    .get_eth_phy_stats = rtl8365mb_get_phy_stats,
    .get_eth_mac_stats = rtl8365mb_get_mac_stats,
    .get_eth_ctrl_stats = rtl8365mb_get_ctrl_stats,
    .get_stats64 = rtl8365mb_get_stats64,
    .port_change_mtu = rtl8365mb_port_change_mtu,
    .port_max_mtu = rtl8365mb_port_max_mtu,
    .port_hsr_join = dsa_port_simple_hsr_join,
    .port_hsr_leave = dsa_port_simple_hsr_leave,
    };
    static const struct realtek_ops rtl8365mb_ops = {
    .detect = rtl8365mb_detect,
    .port_add_isolation = rtl8365mb_port_add_isolation,
    .port_remove_isolation = rtl8365mb_port_remove_isolation,
    .port_set_efid = rtl8365mb_port_set_efid,
    .port_set_learning = rtl8365mb_port_set_learning,
    .port_set_ucast_flood = rtl8365mb_port_set_ucast_flood,
    .port_set_mcast_flood = rtl8365mb_port_set_mcast_flood,
    .port_set_bcast_flood = rtl8365mb_port_set_bcast_flood,
    .l2_add_uc = rtl8365mb_l2_add_uc,
    .l2_del_uc = rtl8365mb_l2_del_uc,
    .l2_get_next_uc = rtl8365mb_l2_get_next_uc,
    .l2_add_mc = rtl8365mb_l2_add_mc,
    .l2_del_mc = rtl8365mb_l2_del_mc,
    .l2_flush = rtl8365mb_l2_flush,
    .phy_read = rtl8365mb_phy_read,
    .phy_write = rtl8365mb_phy_write,
    };
    const struct realtek_variant rtl8365mb_variant = {
    .ds_ops = &rtl8365mb_switch_ops,
    .ops = &rtl8365mb_ops,
    .phylink_mac_ops = &rtl8365mb_phylink_mac_ops,
    .clk_delay = 10,
    .cmd_read = 0xb9,
    .cmd_write = 0xb8,
    .chip_data_sz = sizeof(struct rtl8365mb),
    };
    static const struct of_device_id rtl8365mb_of_match[] = {
    { .compatible = "realtek,rtl8365mb", .data = &rtl8365mb_variant, },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, rtl8365mb_of_match);
    static struct platform_driver rtl8365mb_smi_driver = {
    .driver = {
    .name = "rtl8365mb-smi",
    .of_match_table = rtl8365mb_of_match,
    },
    .probe  = realtek_smi_probe,
    .remove = realtek_smi_remove,
    .shutdown = realtek_smi_shutdown,
    };
    static struct mdio_driver rtl8365mb_mdio_driver = {
    .mdiodrv.driver = {
    .name = "rtl8365mb-mdio",
    .of_match_table = rtl8365mb_of_match,
    },
    .probe  = realtek_mdio_probe,
    .remove = realtek_mdio_remove,
    .shutdown = realtek_mdio_shutdown,
    };
#[no_mangle]
unsafe extern "C" fn rtl8365mb_init() -> c_int {
    static int rtl8365mb_init(void)
    {
    int ret;
    ret = realtek_mdio_driver_register(&rtl8365mb_mdio_driver);
    if (ret)
    return ret;
    ret = realtek_smi_driver_register(&rtl8365mb_smi_driver);
    if (ret) {
    realtek_mdio_driver_unregister(&rtl8365mb_mdio_driver);
    return ret;
    }
    return 0;
    }
    module_init(rtl8365mb_init);
#[no_mangle]
unsafe extern "C" fn rtl8365mb_exit() -> void __exit {
    static void __exit rtl8365mb_exit(void)
    {
    realtek_smi_driver_unregister(&rtl8365mb_smi_driver);
    realtek_mdio_driver_unregister(&rtl8365mb_mdio_driver);
    }
    module_exit(rtl8365mb_exit);
    MODULE_AUTHOR("Alvin Šipraga <alsi@bang-olufsen.dk>");
    MODULE_DESCRIPTION("Driver for RTL8365MB-VC ethernet switch");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("REALTEK_DSA");
