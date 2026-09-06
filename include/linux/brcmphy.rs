//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/brcmphy.h
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

// All Broadcom Ethernet switches have a pseudo-PHY at address 30 which is used
// to configure the switch internal registers via MDIO accesses.
//
pub const BRCM_PSEUDO_PHY_ADDR: c_int = 30;
pub const PHY_ID_BCM50610: c_uint = 0x0143bd60;
pub const PHY_ID_BCM50610M: c_uint = 0x0143bd70;
pub const PHY_ID_BCM5221: c_uint = 0x004061e0;
pub const PHY_ID_BCM5241: c_uint = 0x0143bc30;
pub const PHY_ID_BCMAC131: c_uint = 0x0143bc70;
pub const PHY_ID_BCM5481: c_uint = 0x0143bca0;
pub const PHY_ID_BCM5395: c_uint = 0x0143bcf0;
pub const PHY_ID_BCM53125: c_uint = 0x03625f20;
pub const PHY_ID_BCM53128: c_uint = 0x03625e10;
pub const PHY_ID_BCM54810: c_uint = 0x03625d00;
pub const PHY_ID_BCM54811: c_uint = 0x03625cc0;
pub const PHY_ID_BCM5482: c_uint = 0x0143bcb0;
pub const PHY_ID_BCM5411: c_uint = 0x00206070;
pub const PHY_ID_BCM5421: c_uint = 0x002060e0;
pub const PHY_ID_BCM54210E: c_uint = 0x600d84a0;
pub const PHY_ID_BCM5464: c_uint = 0x002060b0;
pub const PHY_ID_BCM5461: c_uint = 0x002060c0;
pub const PHY_ID_BCM54612E: c_uint = 0x03625e60;
pub const PHY_ID_BCM54616S: c_uint = 0x03625d10;
pub const PHY_ID_BCM54140: c_uint = 0xae025009;
pub const PHY_ID_BCM57780: c_uint = 0x03625d90;
pub const PHY_ID_BCM89610: c_uint = 0x03625cd0;
pub const PHY_ID_BCM72113: c_uint = 0x35905310;
pub const PHY_ID_BCM72116: c_uint = 0x35905350;
pub const PHY_ID_BCM72165: c_uint = 0x35905340;
pub const PHY_ID_BCM7250: c_uint = 0xae025280;
pub const PHY_ID_BCM7255: c_uint = 0xae025120;
pub const PHY_ID_BCM7260: c_uint = 0xae025190;
pub const PHY_ID_BCM7268: c_uint = 0xae025090;
pub const PHY_ID_BCM7271: c_uint = 0xae0253b0;
pub const PHY_ID_BCM7278: c_uint = 0xae0251a0;
pub const PHY_ID_BCM7364: c_uint = 0xae025260;
pub const PHY_ID_BCM7366: c_uint = 0x600d8490;
pub const PHY_ID_BCM7346: c_uint = 0x600d8650;
pub const PHY_ID_BCM7362: c_uint = 0x600d84b0;
pub const PHY_ID_BCM74165: c_uint = 0x359052c0;
pub const PHY_ID_BCM7425: c_uint = 0x600d86b0;
pub const PHY_ID_BCM7429: c_uint = 0x600d8730;
pub const PHY_ID_BCM7435: c_uint = 0x600d8750;
pub const PHY_ID_BCM74371: c_uint = 0xae0252e0;
pub const PHY_ID_BCM7439: c_uint = 0x600d8480;
pub const PHY_ID_BCM7439_2: c_uint = 0xae025080;
pub const PHY_ID_BCM7445: c_uint = 0x600d8510;
pub const PHY_ID_BCM7712: c_uint = 0x35905330;
pub const PHY_ID_BCM_CYGNUS: c_uint = 0xae025200;
pub const PHY_ID_BCM_OMEGA: c_uint = 0xae025100;
pub const PHY_BCM_OUI_MASK: c_uint = 0xfffffc00;
pub const PHY_BCM_OUI_1: c_uint = 0x00206000;
pub const PHY_BCM_OUI_2: c_uint = 0x0143bc00;
pub const PHY_BCM_OUI_3: c_uint = 0x03625c00;
pub const PHY_BCM_OUI_4: c_uint = 0x600d8400;
pub const PHY_BCM_OUI_5: c_uint = 0x03625e00;
pub const PHY_BCM_OUI_6: c_uint = 0xae025000;
pub const PHY_BRCM_AUTO_PWRDWN_ENABLE: c_uint = 0x00000001;
pub const PHY_BRCM_RX_REFCLK_UNUSED: c_uint = 0x00000002;
pub const PHY_BRCM_CLEAR_RGMII_MODE: c_uint = 0x00000004;
pub const PHY_BRCM_DIS_TXCRXC_NOENRGY: c_uint = 0x00000008;
pub const PHY_BRCM_EN_MASTER_MODE: c_uint = 0x00000010;
pub const PHY_BRCM_IDDQ_SUSPEND: c_uint = 0x00000020;
// Broadcom BCM7xxx specific workarounds

pub const PHY_BCM_FLAGS_VALID: c_uint = 0x80000000;
// Broadcom BCM54XX register definitions, common to most Broadcom PHYs
pub const MII_BCM54XX_ECR: c_uint = 0x10	/* BCM54xx extended control register */;
pub const MII_BCM54XX_ECR_IM: c_uint = 0x1000	/* Interrupt mask */;
pub const MII_BCM54XX_ECR_IF: c_uint = 0x0800	/* Interrupt force */;
pub const MII_BCM54XX_ECR_FIFOE: c_uint = 0x0001	/* FIFO elasticity */;
pub const MII_BCM54XX_ESR: c_uint = 0x11	/* BCM54xx extended status register */;
pub const MII_BCM54XX_ESR_IS: c_uint = 0x1000	/* Interrupt status */;
pub const MII_BCM54XX_EXP_DATA: c_uint = 0x15	/* Expansion register data */;
pub const MII_BCM54XX_EXP_SEL: c_uint = 0x17	/* Expansion register select */;
pub const MII_BCM54XX_EXP_SEL_TOP: c_uint = 0x0d00	/* TOP_MISC expansion register select */;
pub const MII_BCM54XX_EXP_SEL_SSD: c_uint = 0x0e00	/* Secondary SerDes select */;
pub const MII_BCM54XX_EXP_SEL_WOL: c_uint = 0x0e00	/* Wake-on-LAN expansion select register */;
pub const MII_BCM54XX_EXP_SEL_ER: c_uint = 0x0f00	/* Expansion register select */;
pub const MII_BCM54XX_EXP_SEL_ETC: c_uint = 0x0d00	/* Expansion register spare + 2k mem */;
pub const MII_BCM54XX_AUX_CTL: c_uint = 0x18	/* Auxiliary control register */;
pub const MII_BCM54XX_ISR: c_uint = 0x1a	/* BCM54xx interrupt status register */;
pub const MII_BCM54XX_IMR: c_uint = 0x1b	/* BCM54xx interrupt mask register */;
pub const MII_BCM54XX_INT_CRCERR: c_uint = 0x0001	/* CRC error */;
pub const MII_BCM54XX_INT_LINK: c_uint = 0x0002	/* Link status changed */;
pub const MII_BCM54XX_INT_SPEED: c_uint = 0x0004	/* Link speed change */;
pub const MII_BCM54XX_INT_DUPLEX: c_uint = 0x0008	/* Duplex mode changed */;
pub const MII_BCM54XX_INT_LRS: c_uint = 0x0010	/* Local receiver status changed */;
pub const MII_BCM54XX_INT_RRS: c_uint = 0x0020	/* Remote receiver status changed */;
pub const MII_BCM54XX_INT_SSERR: c_uint = 0x0040	/* Scrambler synchronization error */;
pub const MII_BCM54XX_INT_UHCD: c_uint = 0x0080	/* Unsupported HCD negotiated */;
pub const MII_BCM54XX_INT_NHCD: c_uint = 0x0100	/* No HCD */;
pub const MII_BCM54XX_INT_NHCDL: c_uint = 0x0200	/* No HCD link */;
pub const MII_BCM54XX_INT_ANPR: c_uint = 0x0400	/* Auto-negotiation page received */;
pub const MII_BCM54XX_INT_LC: c_uint = 0x0800	/* All counters below 128 */;
pub const MII_BCM54XX_INT_HC: c_uint = 0x1000	/* Counter above 32768 */;
pub const MII_BCM54XX_INT_MDIX: c_uint = 0x2000	/* MDIX status change */;
pub const MII_BCM54XX_INT_PSERR: c_uint = 0x4000	/* Pair swap error */;
pub const MII_BCM54XX_SHD: c_uint = 0x1c	/* 0x1c shadow registers */;
pub const MII_BCM54XX_SHD_WRITE: c_uint = 0x8000;

pub const MII_BCM54XX_RDB_ADDR: c_uint = 0x1e;
pub const MII_BCM54XX_RDB_DATA: c_uint = 0x1f;
// legacy access control via rdb/expansion register
pub const BCM54XX_RDB_REG0087: c_uint = 0x0087;

//
// AUXILIARY CONTROL SHADOW ACCESS REGISTERS.  (PHY REG 0x18)
//
pub const MII_BCM54XX_AUXCTL_SHDWSEL_AUXCTL: c_uint = 0x00;
pub const MII_BCM54XX_AUXCTL_ACTL_TX_6DB: c_uint = 0x0400;
pub const MII_BCM54XX_AUXCTL_ACTL_SMDSP_ENA: c_uint = 0x0800;
pub const MII_BCM54XX_AUXCTL_ACTL_EXT_PKT_LEN: c_uint = 0x4000;
pub const MII_BCM54XX_AUXCTL_SHDWSEL_MISC: c_uint = 0x07;
pub const MII_BCM54XX_AUXCTL_SHDWSEL_MISC_WIRESPEED_EN: c_uint = 0x0010;
pub const MII_BCM54XX_AUXCTL_SHDWSEL_MISC_RSVD: c_uint = 0x0060;
pub const MII_BCM54XX_AUXCTL_SHDWSEL_MISC_RGMII_EN: c_uint = 0x0080;
pub const MII_BCM54XX_AUXCTL_SHDWSEL_MISC_RGMII_SKEW_EN: c_uint = 0x0100;
pub const MII_BCM54XX_AUXCTL_MISC_FORCE_AMDIX: c_uint = 0x0200;
pub const MII_BCM54XX_AUXCTL_MISC_WREN: c_uint = 0x8000;
pub const MII_BCM54XX_AUXCTL_SHDWSEL_READ_SHIFT: c_int = 12;
pub const MII_BCM54XX_AUXCTL_SHDWSEL_MASK: c_uint = 0x0007;
//
// Broadcom LED source encodings.  These are used in BCM5461, BCM5481,
// BCM5482, and possibly some others.
//
pub const BCM_LED_SRC_LINKSPD1: c_uint = 0x0;
pub const BCM_LED_SRC_LINKSPD2: c_uint = 0x1;
pub const BCM_LED_SRC_XMITLED: c_uint = 0x2;
pub const BCM_LED_SRC_ACTIVITYLED: c_uint = 0x3;
pub const BCM_LED_SRC_FDXLED: c_uint = 0x4;
pub const BCM_LED_SRC_SLAVE: c_uint = 0x5;
pub const BCM_LED_SRC_INTR: c_uint = 0x6;
pub const BCM_LED_SRC_QUALITY: c_uint = 0x7;
pub const BCM_LED_SRC_RCVLED: c_uint = 0x8;
pub const BCM_LED_SRC_WIRESPEED: c_uint = 0x9;
pub const BCM_LED_SRC_MULTICOLOR1: c_uint = 0xa;
pub const BCM_LED_SRC_OPENSHORT: c_uint = 0xb;
pub const BCM_LED_SRC_OFF: c_uint = 0xe	/* Tied high */;
pub const BCM_LED_SRC_ON: c_uint = 0xf	/* Tied low */;

//
// Broadcom Multicolor LED configurations (expansion register 4)
//

pub const BCM_LED_MULTICOLOR_LINK_ACT: c_uint = 0x0;
pub const BCM_LED_MULTICOLOR_SPEED: c_uint = 0x1;
pub const BCM_LED_MULTICOLOR_ACT_FLASH: c_uint = 0x2;
pub const BCM_LED_MULTICOLOR_FDX: c_uint = 0x3;
pub const BCM_LED_MULTICOLOR_OFF: c_uint = 0x4;
pub const BCM_LED_MULTICOLOR_ON: c_uint = 0x5;
pub const BCM_LED_MULTICOLOR_ALT: c_uint = 0x6;
pub const BCM_LED_MULTICOLOR_FLASH: c_uint = 0x7;
pub const BCM_LED_MULTICOLOR_LINK: c_uint = 0x8;
pub const BCM_LED_MULTICOLOR_ACT: c_uint = 0x9;
pub const BCM_LED_MULTICOLOR_PROGRAM: c_uint = 0xa;
//
// Broadcom Synchronous Ethernet Controls (expansion register 0x0E)
//

//
// BCM5482: Shadow registers
// Shadow values go into bits [14:10] of register 0x1c to select a shadow
// register to access.
//
// 00100: Reserved control register 2
pub const BCM54XX_SHD_SCR2: c_uint = 0x04;
pub const BCM54XX_SHD_SCR2_WSPD_RTRY_DIS: c_uint = 0x100;
pub const BCM54XX_SHD_SCR2_WSPD_RTRY_LMT_SHIFT: c_int = 2;
pub const BCM54XX_SHD_SCR2_WSPD_RTRY_LMT_OFFSET: c_int = 2;
pub const BCM54XX_SHD_SCR2_WSPD_RTRY_LMT_MASK: c_uint = 0x7;
// 00101: Spare Control Register 3
pub const BCM54XX_SHD_SCR3: c_uint = 0x05;
pub const BCM54XX_SHD_SCR3_DEF_CLK125: c_uint = 0x0001;
pub const BCM54XX_SHD_SCR3_DLLAPD_DIS: c_uint = 0x0002;
pub const BCM54XX_SHD_SCR3_TRDDAPD: c_uint = 0x0004;
pub const BCM54XX_SHD_SCR3_RXCTXC_DIS: c_uint = 0x0100;
// 01010: Auto Power-Down
pub const BCM54XX_SHD_APD: c_uint = 0x0a;
pub const BCM_APD_CLR_MASK: c_uint = 0xFE9F /* clear bits 5, 6 & 8 */;
pub const BCM54XX_SHD_APD_EN: c_uint = 0x0020;
pub const BCM_NO_ANEG_APD_EN: c_uint = 0x0060 /* bits 5 & 6 */;
pub const BCM_APD_SINGLELP_EN: c_uint = 0x0100 /* Bit 8 */;
pub const BCM54XX_SHD_LEDS1: c_uint = 0x0d	/* 01101: LED Selector 1 */;
// LED3 / ~LINKSPD[2] selector

// LED1 / ~LINKSPD[1] selector

pub const BCM54XX_SHD_LEDS2: c_uint = 0x0e	/* 01110: LED Selector 2 */;
pub const BCM54XX_SHD_RGMII_MODE: c_uint = 0x0b	/* 01011: RGMII Mode Selector */;
pub const BCM5482_SHD_SSD: c_uint = 0x14	/* 10100: Secondary SerDes control */;
pub const BCM5482_SHD_SSD_LEDM: c_uint = 0x0008	/* SSD LED Mode enable */;
pub const BCM5482_SHD_SSD_EN: c_uint = 0x0001	/* SSD enable */;
// 10011: SerDes 100-FX Control Register
pub const BCM54616S_SHD_100FX_CTRL: c_uint = 0x13;

// 11111: Mode Control Register
pub const BCM54XX_SHD_MODE: c_uint = 0x1f;

pub const BCM54XX_SHD_INTF_SEL_RGMII: c_uint = 0x02;
pub const BCM54XX_SHD_INTF_SEL_SGMII: c_uint = 0x04;
pub const BCM54XX_SHD_INTF_SEL_GBIC: c_uint = 0x06;

//
// EXPANSION SHADOW ACCESS REGISTERS.  (PHY REG 0x15, 0x16, and 0x17)
//
pub const MII_BCM54XX_EXP_AADJ1CH0: c_uint = 0x001f;
pub const MII_BCM54XX_EXP_AADJ1CH0_SWP_ABCD_OEN: c_uint = 0x0200;
pub const MII_BCM54XX_EXP_AADJ1CH0_SWSEL_THPF: c_uint = 0x0100;
pub const MII_BCM54XX_EXP_AADJ1CH3: c_uint = 0x601f;
pub const MII_BCM54XX_EXP_AADJ1CH3_ADCCKADJ: c_uint = 0x0002;
pub const MII_BCM54XX_EXP_EXP08: c_uint = 0x0F08;
pub const MII_BCM54XX_EXP_EXP08_RJCT_2MHZ: c_uint = 0x0001;
pub const MII_BCM54XX_EXP_EXP08_EARLY_DAC_WAKE: c_uint = 0x0200;
pub const MII_BCM54XX_EXP_EXP08_FORCE_DAC_WAKE: c_uint = 0x0100;
pub const MII_BCM54XX_EXP_EXP75: c_uint = 0x0f75;
pub const MII_BCM54XX_EXP_EXP75_VDACCTRL: c_uint = 0x003c;
pub const MII_BCM54XX_EXP_EXP75_CM_OSC: c_uint = 0x0001;
pub const MII_BCM54XX_EXP_EXP96: c_uint = 0x0f96;
pub const MII_BCM54XX_EXP_EXP96_MYST: c_uint = 0x0010;
pub const MII_BCM54XX_EXP_EXP97: c_uint = 0x0f97;
pub const MII_BCM54XX_EXP_EXP97_MYST: c_uint = 0x0c0c;
// Top-MISC expansion registers

//
// BCM5482: Secondary SerDes registers
//
pub const BCM5482_SSD_1000BX_CTL: c_uint = 0x00	/* 1000BASE-X Control */;
pub const BCM5482_SSD_1000BX_CTL_PWRDOWN: c_uint = 0x0800	/* Power-down SSD */;
pub const BCM5482_SSD_SGMII_SLAVE: c_uint = 0x15	/* SGMII Slave Register */;
pub const BCM5482_SSD_SGMII_SLAVE_EN: c_uint = 0x0002	/* Slave mode enable */;
pub const BCM5482_SSD_SGMII_SLAVE_AD: c_uint = 0x0001	/* Slave auto-detection */;
// BroadR-Reach LRE Registers.
pub const MII_BCM54XX_LRECR: c_uint = 0x00	/* LRE Control Register                    */;
pub const MII_BCM54XX_LRESR: c_uint = 0x01	/* LRE Status Register                     */;
pub const MII_BCM54XX_LREPHYSID1: c_uint = 0x02	/* LRE PHYS ID 1                           */;
pub const MII_BCM54XX_LREPHYSID2: c_uint = 0x03	/* LRE PHYS ID 2                           */;
pub const MII_BCM54XX_LREANAA: c_uint = 0x04	/* LDS Auto-Negotiation Advertised Ability */;
pub const MII_BCM54XX_LREANAC: c_uint = 0x05	/* LDS Auto-Negotiation Advertised Control */;
pub const MII_BCM54XX_LREANPT: c_uint = 0x06	/* LDS Ability Next Page Transmit          */;
pub const MII_BCM54XX_LRELPA: c_uint = 0x07	/* LDS Link Partner Ability                */;
pub const MII_BCM54XX_LRELPNPM: c_uint = 0x08	/* LDS Link Partner Next Page Message      */;
pub const MII_BCM54XX_LRELPNPC: c_uint = 0x09	/* LDS Link Partner Next Page Control      */;
pub const MII_BCM54XX_LRELDSE: c_uint = 0x0a	/* LDS Expansion Register                  */;
pub const MII_BCM54XX_LREES: c_uint = 0x0f	/* LRE Extended Status                     */;
// LRE control register.
pub const LRECR_RESET: c_uint = 0x8000	/* Reset to default state      */;
pub const LRECR_LOOPBACK: c_uint = 0x4000	/* Internal Loopback           */;
pub const LRECR_LDSRES: c_uint = 0x2000	/* Restart LDS Process         */;
pub const LRECR_LDSEN: c_uint = 0x1000	/* LDS Enable                  */;
pub const LRECR_PDOWN: c_uint = 0x0800	/* Enable low power state      */;
pub const LRECR_ISOLATE: c_uint = 0x0400	/* Isolate data paths from MII */;
pub const LRECR_SPEED100: c_uint = 0x0200	/* Select 100 Mbps             */;
pub const LRECR_SPEED10: c_uint = 0x0000	/* Select 10 Mbps              */;
pub const LRECR_4PAIRS: c_uint = 0x0020	/* Select 4 Pairs              */;
pub const LRECR_2PAIRS: c_uint = 0x0010	/* Select 2 Pairs              */;
pub const LRECR_1PAIR: c_uint = 0x0000	/* Select 1 Pair               */;
pub const LRECR_MASTER: c_uint = 0x0008	/* Force Master when LDS disabled */;
pub const LRECR_SLAVE: c_uint = 0x0000	/* Force Slave when LDS disabled  */;
// LRE status register.
pub const LRESR_100_1PAIR: c_uint = 0x2000	/* Can do 100Mbps 1 Pair       */;
pub const LRESR_100_4PAIR: c_uint = 0x1000	/* Can do 100Mbps 4 Pairs      */;
pub const LRESR_100_2PAIR: c_uint = 0x0800	/* Can do 100Mbps 2 Pairs      */;
pub const LRESR_10_2PAIR: c_uint = 0x0400	/* Can do 10Mbps 2 Pairs       */;
pub const LRESR_10_1PAIR: c_uint = 0x0200	/* Can do 10Mbps 1 Pair        */;
pub const LRESR_ESTATEN: c_uint = 0x0100	/* Extended Status in R15      */;
pub const LRESR_RESV: c_uint = 0x0080	/* Unused...                   */;
pub const LRESR_MFPS: c_uint = 0x0040	/* Can suppress Management Frames Preamble */;
pub const LRESR_LDSCOMPLETE: c_uint = 0x0020	/* LDS Auto-negotiation complete */;
pub const LRESR_8023: c_uint = 0x0010	/* Has IEEE 802.3 Support      */;
pub const LRESR_LDSABILITY: c_uint = 0x0008	/* LDS auto-negotiation capable */;
pub const LRESR_LSTATUS: c_uint = 0x0004	/* Link status                 */;
pub const LRESR_JCD: c_uint = 0x0002	/* Jabber detected             */;
pub const LRESR_ERCAP: c_uint = 0x0001	/* Ext-reg capability          */;
// LDS Auto-Negotiation Advertised Ability.
pub const LREANAA_PAUSE_ASYM: c_uint = 0x8000	/* Can pause asymmetrically    */;
pub const LREANAA_PAUSE: c_uint = 0x4000	/* Can pause                   */;
pub const LREANAA_100_1PAIR: c_uint = 0x0020	/* Can do 100Mbps 1 Pair       */;
pub const LREANAA_100_4PAIR: c_uint = 0x0010	/* Can do 100Mbps 4 Pair       */;
pub const LREANAA_100_2PAIR: c_uint = 0x0008	/* Can do 100Mbps 2 Pair       */;
pub const LREANAA_10_2PAIR: c_uint = 0x0004	/* Can do 10Mbps 2 Pair        */;
pub const LREANAA_10_1PAIR: c_uint = 0x0002	/* Can do 10Mbps 1 Pair        */;

// LDS Link Partner Ability.
pub const LRELPA_PAUSE_ASYM: c_uint = 0x8000	/* Supports asymmetric pause   */;
pub const LRELPA_PAUSE: c_uint = 0x4000	/* Supports pause capability   */;
pub const LRELPA_100_1PAIR: c_uint = 0x0020	/* 100Mbps 1 Pair capable      */;
pub const LRELPA_100_4PAIR: c_uint = 0x0010	/* 100Mbps 4 Pair capable      */;
pub const LRELPA_100_2PAIR: c_uint = 0x0008	/* 100Mbps 2 Pair capable      */;
pub const LRELPA_10_2PAIR: c_uint = 0x0004	/* 10Mbps 2 Pair capable       */;
pub const LRELPA_10_1PAIR: c_uint = 0x0002	/* 10Mbps 1 Pair capable       */;
// LDS Expansion register.
pub const LDSE_DOWNGRADE: c_uint = 0x8000	/* Can do LDS Speed Downgrade  */;
pub const LDSE_MASTER: c_uint = 0x4000	/* Master / Slave              */;
pub const LDSE_PAIRS_MASK: c_uint = 0x3000	/* Pair Count Mask             */;
pub const LDSE_PAIRS_SHIFT: c_int = 12;

pub const LDSE_CABLEN_MASK: c_uint = 0x0FFF	/* Cable Length Mask           */;
// BCM54810 Registers

pub const BCM54810_SHD_CLK_CTL: c_uint = 0x3;

// BCM54811 Registers

// Access Control Override Enable

// Access Control Override Value

// Access Control Value

// BCM54612E Registers

// Wake-on-LAN registers

pub const BCM54XX_WOL_MODE_SINGLE_MPD: c_int = 0;
pub const BCM54XX_WOL_MODE_SINGLE_MPDSEC: c_int = 1;
pub const BCM54XX_WOL_MODE_DUAL: c_int = 2;
pub const BCM54XX_WOL_MODE_SHIFT: c_int = 1;
pub const BCM54XX_WOL_MODE_MASK: c_uint = 0x3;

pub const BCM54XX_WOL_SECKEY_OPT_4B: c_int = 0;
pub const BCM54XX_WOL_SECKEY_OPT_6B: c_int = 1;
pub const BCM54XX_WOL_SECKEY_OPT_8B: c_int = 2;
pub const BCM54XX_WOL_SECKEY_OPT_SHIFT: c_int = 4;
pub const BCM54XX_WOL_SECKEY_OPT_MASK: c_uint = 0x3;

pub const BCM54XX_WOL_MASK_MODE_DA_FF: c_int = 0;
pub const BCM54XX_WOL_MASK_MODE_DA_MPD: c_int = 1;
pub const BCM54XX_WOL_MASK_MODE_DA_ONLY: c_int = 2;
pub const BCM54XX_WOL_MASK_MODE_MPD: c_int = 3;
pub const BCM54XX_WOL_MASK_MODE_SHIFT: c_int = 14;
pub const BCM54XX_WOL_MASK_MODE_MASK: c_uint = 0x3;

// BCM5221 Registers
pub const BCM5221_AEGSR: c_uint = 0x1C;

//
// Fast Ethernet Transceiver definitions.
//
pub const MII_BRCM_FET_INTREG: c_uint = 0x1a	/* Interrupt register */;
pub const MII_BRCM_FET_IR_MASK: c_uint = 0x0100	/* Mask all interrupts */;
pub const MII_BRCM_FET_IR_LINK_EN: c_uint = 0x0200	/* Link status change enable */;
pub const MII_BRCM_FET_IR_SPEED_EN: c_uint = 0x0400	/* Link speed change enable */;
pub const MII_BRCM_FET_IR_DUPLEX_EN: c_uint = 0x0800	/* Duplex mode change enable */;
pub const MII_BRCM_FET_IR_ENABLE: c_uint = 0x4000	/* Interrupt enable */;
pub const MII_BRCM_FET_BRCMTEST: c_uint = 0x1f	/* Brcm test register */;
pub const MII_BRCM_FET_BT_SRE: c_uint = 0x0080	/* Shadow register enable */;
// Shadow register definitions
pub const MII_BRCM_FET_SHDW_MISCCTRL: c_uint = 0x10	/* Shadow misc ctrl */;
pub const MII_BRCM_FET_SHDW_MC_FAME: c_uint = 0x4000	/* Force Auto MDIX enable */;
pub const MII_BRCM_FET_SHDW_AUXMODE4: c_uint = 0x1a	/* Auxiliary mode 4 */;
pub const MII_BRCM_FET_SHDW_AM4_STANDBY: c_uint = 0x0008	/* Standby enable */;
pub const MII_BRCM_FET_SHDW_AM4_LED_MASK: c_uint = 0x0003;
pub const MII_BRCM_FET_SHDW_AM4_LED_MODE1: c_uint = 0x0001;
pub const MII_BRCM_FET_SHDW_AUXSTAT2: c_uint = 0x1b	/* Auxiliary status 2 */;
pub const MII_BRCM_FET_SHDW_AS2_APDE: c_uint = 0x0020	/* Auto power down enable */;
pub const BRCM_CL45VEN_EEE_CONTROL: c_uint = 0x803d;
pub const LPI_FEATURE_EN: c_uint = 0x8000;
pub const LPI_FEATURE_EN_DIG1000X: c_uint = 0x4000;
pub const BRCM_CL45VEN_EEE_LPI_CNT: c_uint = 0x803f;
// Core register definitions
pub const MII_BRCM_CORE_BASE12: c_uint = 0x12;
pub const MII_BRCM_CORE_BASE13: c_uint = 0x13;
pub const MII_BRCM_CORE_BASE14: c_uint = 0x14;
pub const MII_BRCM_CORE_BASE1E: c_uint = 0x1E;
pub const MII_BRCM_CORE_EXPB0: c_uint = 0xB0;
pub const MII_BRCM_CORE_EXPB1: c_uint = 0xB1;
// Enhanced Cable Diagnostics
pub const BCM54XX_RDB_ECD_CTRL: c_uint = 0x2a0;

// during test
//

// short check
//

pub const BCM54XX_RDB_ECD_FAULT_TYPE: c_uint = 0x2a1;

pub const BCM54XX_ECD_FAULT_TYPE_INVALID: c_uint = 0x0;
pub const BCM54XX_ECD_FAULT_TYPE_OK: c_uint = 0x1;
pub const BCM54XX_ECD_FAULT_TYPE_OPEN: c_uint = 0x2;
pub const BCM54XX_ECD_FAULT_TYPE_SAME_SHORT: c_uint = 0x3 /* short same pair */;
pub const BCM54XX_ECD_FAULT_TYPE_CROSS_SHORT: c_uint = 0x4 /* short different pairs */;
pub const BCM54XX_ECD_FAULT_TYPE_BUSY: c_uint = 0x9;

pub const BCM54XX_ECD_PAIR_A_LENGTH_RESULTS: c_uint = 0x2a2;
pub const BCM54XX_ECD_PAIR_B_LENGTH_RESULTS: c_uint = 0x2a3;
pub const BCM54XX_ECD_PAIR_C_LENGTH_RESULTS: c_uint = 0x2a4;
pub const BCM54XX_ECD_PAIR_D_LENGTH_RESULTS: c_uint = 0x2a5;
pub const BCM54XX_RDB_ECD_PAIR_A_LENGTH_RESULTS: c_uint = 0x2a2;

pub const BCM54XX_RDB_ECD_PAIR_B_LENGTH_RESULTS: c_uint = 0x2a3;

pub const BCM54XX_RDB_ECD_PAIR_C_LENGTH_RESULTS: c_uint = 0x2a4;

pub const BCM54XX_RDB_ECD_PAIR_D_LENGTH_RESULTS: c_uint = 0x2a5;

pub const BCM54XX_ECD_LENGTH_RESULTS_INVALID: c_uint = 0xffff;
