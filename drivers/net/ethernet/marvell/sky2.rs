//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/sky2.h
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
// Definitions for the new Marvell Yukon 2 driver.
//

// PCI config registers
// Yukon-2
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_dev_reg_1 {
    PCI_Y2_PIG_ENA	 = 1<<31, /* Enable Plug-in-Go (YUKON-2) */
    PCI_Y2_DLL_DIS	 = 1<<30, /* Disable PCI DLL (YUKON-2) */
    PCI_SW_PWR_ON_RST= 1<<30, /* SW Power on Reset (Yukon-EX) */
    PCI_Y2_PHY2_COMA = 1<<29, /* Set PHY 2 to Coma Mode (YUKON-2) */
    PCI_Y2_PHY1_COMA = 1<<28, /* Set PHY 1 to Coma Mode (YUKON-2) */
    PCI_Y2_PHY2_POWD = 1<<27, /* Set PHY 2 to Power Down (YUKON-2) */
    PCI_Y2_PHY1_POWD = 1<<26, /* Set PHY 1 to Power Down (YUKON-2) */
    PCI_Y2_PME_LEGACY= 1<<15, /* PCI Express legacy power management mode */

    PCI_PHY_LNK_TIM_MSK= 3L<<8,/* Bit  9.. 8:	GPHY Link Trigger Timer */
    PCI_ENA_L1_EVENT = 1<<7, /* Enable PEX L1 Event */
    PCI_ENA_GPHY_LNK = 1<<6, /* Enable PEX L1 on GPHY Link down */
    PCI_FORCE_PEX_L1 = 1<<5, /* Force to PEX L1 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_dev_reg_2 {
    PCI_VPD_WR_THR	= 0xffL<<24,	/* Bit 31..24:	VPD Write Threshold */
    PCI_DEV_SEL	= 0x7fL<<17,	/* Bit 23..17:	EEPROM Device Select */
    PCI_VPD_ROM_SZ	= 7L<<14,	/* Bit 16..14:	VPD ROM Size	*/

    PCI_PATCH_DIR	= 0xfL<<8,	/* Bit 11.. 8:	Ext Patches dir 3..0 */
    PCI_EXT_PATCHS	= 0xfL<<4,	/* Bit	7.. 4:	Extended Patches 3..0 */
    PCI_EN_DUMMY_RD	= 1<<3,		/* Enable Dummy Read */
    PCI_REV_DESC	= 1<<2,		/* Reverse Desc. Bytes */

    PCI_USEDATA64	= 1<<0,		/* Use 64Bit Data bus ext */
}

// PCI_OUR_REG_3		32 bit	Our Register 3 (Yukon-ECU only)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_dev_reg_3 {
    P_CLK_ASF_REGS_DIS	= 1<<18,/* Disable Clock ASF (Yukon-Ext.) */
    P_CLK_COR_REGS_D0_DIS	= 1<<17,/* Disable Clock Core Regs D0 */
    P_CLK_MACSEC_DIS	= 1<<17,/* Disable Clock MACSec (Yukon-Ext.) */
    P_CLK_PCI_REGS_D0_DIS	= 1<<16,/* Disable Clock PCI  Regs D0 */
    P_CLK_COR_YTB_ARB_DIS	= 1<<15,/* Disable Clock YTB  Arbiter */
    P_CLK_MAC_LNK1_D3_DIS	= 1<<14,/* Disable Clock MAC  Link1 D3 */
    P_CLK_COR_LNK1_D0_DIS	= 1<<13,/* Disable Clock Core Link1 D0 */
    P_CLK_MAC_LNK1_D0_DIS	= 1<<12,/* Disable Clock MAC  Link1 D0 */
    P_CLK_COR_LNK1_D3_DIS	= 1<<11,/* Disable Clock Core Link1 D3 */
    P_CLK_PCI_MST_ARB_DIS	= 1<<10,/* Disable Clock PCI  Master Arb. */
    P_CLK_COR_REGS_D3_DIS	= 1<<9,	/* Disable Clock Core Regs D3 */
    P_CLK_PCI_REGS_D3_DIS	= 1<<8,	/* Disable Clock PCI  Regs D3 */
    P_CLK_REF_LNK1_GM_DIS	= 1<<7,	/* Disable Clock Ref. Link1 GMAC */
    P_CLK_COR_LNK1_GM_DIS	= 1<<6,	/* Disable Clock Core Link1 GMAC */
    P_CLK_PCI_COMMON_DIS	= 1<<5,	/* Disable Clock PCI  Common */
    P_CLK_COR_COMMON_DIS	= 1<<4,	/* Disable Clock Core Common */
    P_CLK_PCI_LNK1_BMU_DIS	= 1<<3,	/* Disable Clock PCI  Link1 BMU */
    P_CLK_COR_LNK1_BMU_DIS	= 1<<2,	/* Disable Clock Core Link1 BMU */
    P_CLK_PCI_LNK1_BIU_DIS	= 1<<1,	/* Disable Clock PCI  Link1 BIU */
    P_CLK_COR_LNK1_BIU_DIS	= 1<<0,	/* Disable Clock Core Link1 BIU */
    PCIE_OUR3_WOL_D3_COLD_SET = P_CLK_ASF_REGS_DIS |
    P_CLK_COR_REGS_D0_DIS |
    P_CLK_COR_LNK1_D0_DIS |
    P_CLK_MAC_LNK1_D0_DIS |
    P_CLK_PCI_MST_ARB_DIS |
    P_CLK_COR_COMMON_DIS |
    P_CLK_COR_LNK1_BMU_DIS,
}

// PCI_OUR_REG_4		32 bit	Our Register 4 (Yukon-ECU only)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_dev_reg_4 {
// (Link Training & Status State Machine)
    P_PEX_LTSSM_STAT_MSK	= 0x7fL<<25,	/* Bit 31..25:	PEX LTSSM Mask */

    P_PEX_LTSSM_L1_STAT	= 0x34,
    P_PEX_LTSSM_DET_STAT	= 0x01,
    P_TIMER_VALUE_MSK	= 0xffL<<16,	/* Bit 23..16:	Timer Value Mask */
// (Active State Power Management)
    P_FORCE_ASPM_REQUEST	= 1<<15, /* Force ASPM Request (A1 only) */
    P_ASPM_GPHY_LINK_DOWN	= 1<<14, /* GPHY Link Down (A1 only) */
    P_ASPM_INT_FIFO_EMPTY	= 1<<13, /* Internal FIFO Empty (A1 only) */
    P_ASPM_CLKRUN_REQUEST	= 1<<12, /* CLKRUN Request (A1 only) */

    P_ASPM_FORCE_CLKREQ_ENA	= 1<<4,	/* Force CLKREQ Enable (A1b only) */
    P_ASPM_CLKREQ_PAD_CTL	= 1<<3,	/* CLKREQ PAD Control (A1 only) */
    P_ASPM_A1_MODE_SELECT	= 1<<2,	/* A1 Mode Select (A1 only) */
    P_CLK_GATE_PEX_UNIT_ENA	= 1<<1,	/* Enable Gate PEX Unit Clock */
    P_CLK_GATE_ROOT_COR_ENA	= 1<<0,	/* Enable Gate Root Core Clock */
    P_ASPM_CONTROL_MSK	= P_FORCE_ASPM_REQUEST | P_ASPM_GPHY_LINK_DOWN
    | P_ASPM_CLKRUN_REQUEST | P_ASPM_INT_FIFO_EMPTY,
}

// PCI_OUR_REG_5		32 bit	Our Register 5 (Yukon-ECU only)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_dev_reg_5 {
// Bit 31..27:	for A3 & later
    P_CTL_DIV_CORE_CLK_ENA	= 1<<31, /* Divide Core Clock Enable */
    P_CTL_SRESET_VMAIN_AV	= 1<<30, /* Soft Reset for Vmain_av De-Glitch */
    P_CTL_BYPASS_VMAIN_AV	= 1<<29, /* Bypass En. for Vmain_av De-Glitch */
    P_CTL_TIM_VMAIN_AV_MSK	= 3<<27, /* Bit 28..27: Timer Vmain_av Mask */
// Bit 26..16: Release Clock on Event
    P_REL_PCIE_RST_DE_ASS	= 1<<26, /* PCIe Reset De-Asserted */
    P_REL_GPHY_REC_PACKET	= 1<<25, /* GPHY Received Packet */
    P_REL_INT_FIFO_N_EMPTY	= 1<<24, /* Internal FIFO Not Empty */
    P_REL_MAIN_PWR_AVAIL	= 1<<23, /* Main Power Available */
    P_REL_CLKRUN_REQ_REL	= 1<<22, /* CLKRUN Request Release */
    P_REL_PCIE_RESET_ASS	= 1<<21, /* PCIe Reset Asserted */
    P_REL_PME_ASSERTED	= 1<<20, /* PME Asserted */
    P_REL_PCIE_EXIT_L1_ST	= 1<<19, /* PCIe Exit L1 State */
    P_REL_LOADER_NOT_FIN	= 1<<18, /* EPROM Loader Not Finished */
    P_REL_PCIE_RX_EX_IDLE	= 1<<17, /* PCIe Rx Exit Electrical Idle State */
    P_REL_GPHY_LINK_UP	= 1<<16, /* GPHY Link Up */

// Bit 10.. 0: Mask for Gate Clock
    P_GAT_PCIE_RST_ASSERTED	= 1<<10,/* PCIe Reset Asserted */
    P_GAT_GPHY_N_REC_PACKET	= 1<<9, /* GPHY Not Received Packet */
    P_GAT_INT_FIFO_EMPTY	= 1<<8, /* Internal FIFO Empty */
    P_GAT_MAIN_PWR_N_AVAIL	= 1<<7, /* Main Power Not Available */
    P_GAT_CLKRUN_REQ_REL	= 1<<6, /* CLKRUN Not Requested */
    P_GAT_PCIE_RESET_ASS	= 1<<5, /* PCIe Reset Asserted */
    P_GAT_PME_DE_ASSERTED	= 1<<4, /* PME De-Asserted */
    P_GAT_PCIE_ENTER_L1_ST	= 1<<3, /* PCIe Enter L1 State */
    P_GAT_LOADER_FINISHED	= 1<<2, /* EPROM Loader Finished */
    P_GAT_PCIE_RX_EL_IDLE	= 1<<1, /* PCIe Rx Electrical Idle State */
    P_GAT_GPHY_LINK_DOWN	= 1<<0,	/* GPHY Link Down */

    PCIE_OUR5_EVENT_CLK_D3_SET = P_REL_GPHY_REC_PACKET |
    P_REL_INT_FIFO_N_EMPTY |
    P_REL_PCIE_EXIT_L1_ST |
    P_REL_PCIE_RX_EX_IDLE |
    P_GAT_GPHY_N_REC_PACKET |
    P_GAT_INT_FIFO_EMPTY |
    P_GAT_PCIE_ENTER_L1_ST |
    P_GAT_PCIE_RX_EL_IDLE,
}

// PCI_CFG_REG_1			32 bit	Config Register 1 (Yukon-Ext only)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_cfg_reg1 {
    P_CF1_DIS_REL_EVT_RST	= 1<<24, /* Dis. Rel. Event during PCIE reset */
// Bit 23..21: Release Clock on Event
    P_CF1_REL_LDR_NOT_FIN	= 1<<23, /* EEPROM Loader Not Finished */
    P_CF1_REL_VMAIN_AVLBL	= 1<<22, /* Vmain available */
    P_CF1_REL_PCIE_RESET	= 1<<21, /* PCI-E reset */
// Bit 20..18: Gate Clock on Event
    P_CF1_GAT_LDR_NOT_FIN	= 1<<20, /* EEPROM Loader Finished */
    P_CF1_GAT_PCIE_RX_IDLE	= 1<<19, /* PCI-E Rx Electrical idle */
    P_CF1_GAT_PCIE_RESET	= 1<<18, /* PCI-E Reset */
    P_CF1_PRST_PHY_CLKREQ	= 1<<17, /* Enable PCI-E rst & PM2PHY gen. CLKREQ */
    P_CF1_PCIE_RST_CLKREQ	= 1<<16, /* Enable PCI-E rst generate CLKREQ */

    P_CF1_ENA_CFG_LDR_DONE	= 1<<8, /* Enable core level Config loader done */

    P_CF1_ENA_TXBMU_RD_IDLE	= 1<<1, /* Enable TX BMU Read  IDLE for ASPM */
    P_CF1_ENA_TXBMU_WR_IDLE	= 1<<0, /* Enable TX BMU Write IDLE for ASPM */

    PCIE_CFG1_EVENT_CLK_D3_SET = P_CF1_DIS_REL_EVT_RST |
    P_CF1_REL_LDR_NOT_FIN |
    P_CF1_REL_VMAIN_AVLBL |
    P_CF1_REL_PCIE_RESET |
    P_CF1_GAT_LDR_NOT_FIN |
    P_CF1_GAT_PCIE_RESET |
    P_CF1_PRST_PHY_CLKREQ |
    P_CF1_ENA_CFG_LDR_DONE |
    P_CF1_ENA_TXBMU_RD_IDLE |
    P_CF1_ENA_TXBMU_WR_IDLE,
}

// Yukon-Optima
// Yukon-Supreme
// PSM_CONFIG_REG4				0x0168	PSM Config Register 4
// PHY Link Detect Timer
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csr_regs {
    B0_RAP		= 0x0000,
    B0_CTST		= 0x0004,

    B0_POWER_CTRL	= 0x0007,
    B0_ISRC		= 0x0008,
    B0_IMSK		= 0x000c,
    B0_HWE_ISRC	= 0x0010,
    B0_HWE_IMSK	= 0x0014,

// Special ISR registers (Yukon-2 only)
    B0_Y2_SP_ISRC2	= 0x001c,
    B0_Y2_SP_ISRC3	= 0x0020,
    B0_Y2_SP_EISR	= 0x0024,
    B0_Y2_SP_LISR	= 0x0028,
    B0_Y2_SP_ICR	= 0x002c,

    B2_MAC_1	= 0x0100,
    B2_MAC_2	= 0x0108,
    B2_MAC_3	= 0x0110,
    B2_CONN_TYP	= 0x0118,
    B2_PMD_TYP	= 0x0119,
    B2_MAC_CFG	= 0x011a,
    B2_CHIP_ID	= 0x011b,
    B2_E_0		= 0x011c,

    B2_Y2_CLK_GATE  = 0x011d,
    B2_Y2_HW_RES	= 0x011e,
    B2_E_3		= 0x011f,
    B2_Y2_CLK_CTRL	= 0x0120,

    B2_TI_INI	= 0x0130,
    B2_TI_VAL	= 0x0134,
    B2_TI_CTRL	= 0x0138,
    B2_TI_TEST	= 0x0139,

    B2_TST_CTRL1	= 0x0158,
    B2_TST_CTRL2	= 0x0159,
    B2_GP_IO	= 0x015c,

    B2_I2C_CTRL	= 0x0160,
    B2_I2C_DATA	= 0x0164,
    B2_I2C_IRQ	= 0x0168,
    B2_I2C_SW	= 0x016c,

    Y2_PEX_PHY_DATA = 0x0170,
    Y2_PEX_PHY_ADDR = 0x0172,

    B3_RAM_ADDR	= 0x0180,
    B3_RAM_DATA_LO	= 0x0184,
    B3_RAM_DATA_HI	= 0x0188,

// RAM Interface Registers
// Yukon-2: use RAM_BUFFER() to access the RAM buffer
//
// The HW-Spec. calls this registers Timeout Value 0..11. But this names are
// not usable in SW. Please notice these are NOT real timeouts, these are
// the number of qWords transferred continuously.
//

    B3_RI_WTO_R1	= 0x0190,
    B3_RI_WTO_XA1	= 0x0191,
    B3_RI_WTO_XS1	= 0x0192,
    B3_RI_RTO_R1	= 0x0193,
    B3_RI_RTO_XA1	= 0x0194,
    B3_RI_RTO_XS1	= 0x0195,
    B3_RI_WTO_R2	= 0x0196,
    B3_RI_WTO_XA2	= 0x0197,
    B3_RI_WTO_XS2	= 0x0198,
    B3_RI_RTO_R2	= 0x0199,
    B3_RI_RTO_XA2	= 0x019a,
    B3_RI_RTO_XS2	= 0x019b,
    B3_RI_TO_VAL	= 0x019c,
    B3_RI_CTRL	= 0x01a0,
    B3_RI_TEST	= 0x01a2,
    B3_MA_TOINI_RX1	= 0x01b0,
    B3_MA_TOINI_RX2	= 0x01b1,
    B3_MA_TOINI_TX1	= 0x01b2,
    B3_MA_TOINI_TX2	= 0x01b3,
    B3_MA_TOVAL_RX1	= 0x01b4,
    B3_MA_TOVAL_RX2	= 0x01b5,
    B3_MA_TOVAL_TX1	= 0x01b6,
    B3_MA_TOVAL_TX2	= 0x01b7,
    B3_MA_TO_CTRL	= 0x01b8,
    B3_MA_TO_TEST	= 0x01ba,
    B3_MA_RCINI_RX1	= 0x01c0,
    B3_MA_RCINI_RX2	= 0x01c1,
    B3_MA_RCINI_TX1	= 0x01c2,
    B3_MA_RCINI_TX2	= 0x01c3,
    B3_MA_RCVAL_RX1	= 0x01c4,
    B3_MA_RCVAL_RX2	= 0x01c5,
    B3_MA_RCVAL_TX1	= 0x01c6,
    B3_MA_RCVAL_TX2	= 0x01c7,
    B3_MA_RC_CTRL	= 0x01c8,
    B3_MA_RC_TEST	= 0x01ca,
    B3_PA_TOINI_RX1	= 0x01d0,
    B3_PA_TOINI_RX2	= 0x01d4,
    B3_PA_TOINI_TX1	= 0x01d8,
    B3_PA_TOINI_TX2	= 0x01dc,
    B3_PA_TOVAL_RX1	= 0x01e0,
    B3_PA_TOVAL_RX2	= 0x01e4,
    B3_PA_TOVAL_TX1	= 0x01e8,
    B3_PA_TOVAL_TX2	= 0x01ec,
    B3_PA_CTRL	= 0x01f0,
    B3_PA_TEST	= 0x01f2,

    Y2_CFG_SPC	= 0x1c00,	/* PCI config space region */
    Y2_CFG_AER      = 0x1d00,	/* PCI Advanced Error Report region */
}

// B0_CTST			24 bit	Control/Status register
// B0_POWER_CTRL	 8 Bit	Power Control reg (YUKON only)
// B2_IRQM_MSK 	32 bit	IRQ Moderation Mask
// B0_Y2_SP_ISRC2	32 bit	Special Interrupt Source Reg 2
// B0_Y2_SP_ISRC3	32 bit	Special Interrupt Source Reg 3
// B0_Y2_SP_EISR	32 bit	Enter ISR Reg
// B0_Y2_SP_LISR	32 bit	Leave ISR Reg
// B2_IRQM_HWE_MSK	32 bit	IRQ Moderation HW Error Mask
// Hardware error interrupt mask for Yukon 2
// Link 2
// Link 1
// B28_DPT_CTRL	 8 bit	Descriptor Poll Timer Ctrl Reg
// B2_TST_CTRL1	 8 bit	Test Control Register 1
// B2_GPIO
// B2_MAC_CFG		 8 bit	MAC Configuration / Chip Revision
// Bit 3.. 2:	reserved
// B2_CHIP_ID		 8 bit 	Chip Identification Number
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum yukon_xl_rev {
    CHIP_REV_YU_XL_A0  = 0,
    CHIP_REV_YU_XL_A1  = 1,
    CHIP_REV_YU_XL_A2  = 2,
    CHIP_REV_YU_XL_A3  = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum yukon_ec_rev {
    CHIP_REV_YU_EC_A1    = 0,  /* Chip Rev. for Yukon-EC A1/A0 */
    CHIP_REV_YU_EC_A2    = 1,  /* Chip Rev. for Yukon-EC A2 */
    CHIP_REV_YU_EC_A3    = 2,  /* Chip Rev. for Yukon-EC A3 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum yukon_ec_u_rev {
    CHIP_REV_YU_EC_U_A0  = 1,
    CHIP_REV_YU_EC_U_A1  = 2,
    CHIP_REV_YU_EC_U_B0  = 3,
    CHIP_REV_YU_EC_U_B1  = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum yukon_fe_rev {
    CHIP_REV_YU_FE_A1    = 1,
    CHIP_REV_YU_FE_A2    = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum yukon_fe_p_rev {
    CHIP_REV_YU_FE2_A0   = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum yukon_ex_rev {
    CHIP_REV_YU_EX_A0    = 1,
    CHIP_REV_YU_EX_B0    = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum yukon_supr_rev {
    CHIP_REV_YU_SU_A0    = 0,
    CHIP_REV_YU_SU_B0    = 1,
    CHIP_REV_YU_SU_B1    = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum yukon_prm_rev {
    CHIP_REV_YU_PRM_Z1   = 1,
    CHIP_REV_YU_PRM_A0   = 2,
}

// B2_Y2_CLK_GATE	 8 bit	Clock Gating (Yukon-2 only)
// B2_Y2_HW_RES	8 bit	HW Resources (Yukon-2 only)

// B2_Y2_CLK_CTRL	32 bit	Clock Frequency Control Register (Yukon-2/EC)

// B2_TI_CTRL		 8 bit	Timer control
// B2_IRQM_CTRL	 8 bit	IRQ Moderation Timer Control
// B2_TI_TEST		 8 Bit	Timer Test
// B2_IRQM_TEST	 8 bit	IRQ Moderation Timer Test
// B28_DPT_TST		 8 bit	Descriptor Poll Timer Test Reg
// Y2_PEX_PHY_ADDR/DATA		PEX PHY address and data reg  (Yukon-2 only)
// B3_RAM_ADDR		32 bit	RAM Address, to read or write
// Bit 31..19:	reserved
pub const RAM_ADR_RAN: c_uint = 0x0007ffffL	/* Bit 18.. 0:	RAM Address Range */;
// RAM Interface Registers
// B3_RI_CTRL		16 bit	RAM Interface Control Register

// Port related registers FIFO, and Arbiter

// Transmit Arbiter Registers MAC 1 and 2, use SK_REG() to access
// TXA_ITI_INI		32 bit	Tx Arb Interval Timer Init Val
// TXA_ITI_VAL		32 bit	Tx Arb Interval Timer Value
// TXA_LIM_INI		32 bit	Tx Arb Limit Counter Init Val
// TXA_LIM_VAL		32 bit	Tx Arb Limit Counter Value
pub const TXA_MAX_VAL: c_uint = 0x00ffffffUL	/* Bit 23.. 0:	Max TXA Timer/Cnt Val */;
// TXA_CTRL		 8 bit	Tx Arbiter Control Register
//
// Bank 4 - 5
//
// Transmit Arbiter Registers MAC 1 and 2, use SK_REG() to access
// Queue Register Offsets, use Q_ADDR() to access
// Yukon-2

// Q_TEST				32 bit	Test Register
// Transmit
// Receive
// Hardware testbits not used
// Queue Prefetch Unit Offsets, use Y2_QADDR() to address (Yukon-2 only)

// RAM Buffer Register Offsets
// 0x10 - 0x1f:	reserved at Tx RAM Buffer Registers
// Receive and Transmit Queues
// Different PHY Types

// Receive GMAC FIFO (YUKON and Yukon-2)
// Q_BC			32 bit	Current Byte Counter
// BMU Control Status Registers
// B0_R1_CSR		32 bit	BMU Ctrl/Stat Rx Queue 1
// B0_R2_CSR		32 bit	BMU Ctrl/Stat Rx Queue 2
// B0_XA1_CSR		32 bit	BMU Ctrl/Stat Sync Tx Queue 1
// B0_XS1_CSR		32 bit	BMU Ctrl/Stat Async Tx Queue 1
// B0_XA2_CSR		32 bit	BMU Ctrl/Stat Sync Tx Queue 2
// B0_XS2_CSR		32 bit	BMU Ctrl/Stat Async Tx Queue 2
// Q_CSR			32 bit	BMU Control/Status Register
// Rx BMU Control / Status Registers (Yukon-2)
// Tx BMU Control / Status Registers (Yukon-2)
// Bit 31: same as for Rx
// TBMU_TEST			0x06B8	Transmit BMU Test Register
// Queue Prefetch Unit Offsets, use Y2_QADDR() to address (Yukon-2 only)
// PREF_UNIT_CTRL	32 bit	Prefetch Control register
// RAM Buffer Register Offsets, use RB_ADDR(Queue, Offs) to access
// RB_START		32 bit	RAM Buffer Start Address
// RB_END			32 bit	RAM Buffer End Address
// RB_WP			32 bit	RAM Buffer Write Pointer
// RB_RP			32 bit	RAM Buffer Read Pointer
// RB_RX_UTPP		32 bit	Rx Upper Threshold, Pause Pack
// RB_RX_LTPP		32 bit	Rx Lower Threshold, Pause Pack
// RB_RX_UTHP		32 bit	Rx Upper Threshold, High Prio
// RB_RX_LTHP		32 bit	Rx Lower Threshold, High Prio
// RB_PC			32 bit	RAM Buffer Packet Counter
// RB_LEV			32 bit	RAM Buffer Level Register
pub const RB_MSK: c_uint = 0x0007ffff	/* Bit 18.. 0:	RAM Buffer Pointer Bits */;
// RB_TST2			 8 bit	RAM Buffer Test Register 2
// RB_TST1			 8 bit	RAM Buffer Test Register 1
// RB_CTRL			 8 bit	RAM Buffer Control Register
// Transmit GMAC FIFO (YUKON only)
// Threshold values for Yukon-EC Ultra and Extreme
// Descriptor Poll Timer Registers
// Time Stamp Timer Registers (YUKON only)
// Polling Unit Registers (Yukon-2 only)
// ASF Subsystem Registers (Yukon-2 only)
// Status BMU Registers (Yukon-2 only)
// FIFO Control/Status Registers (Yukon-2 only)
// Level and ISR Timer Registers (Yukon-2 only)
// GMAC and GPHY Control Registers (YUKON only)
// Wake-up Frame Pattern Match Control Registers (YUKON only)
// WOL Pattern Length Registers (YUKON only)
// WOL Pattern Counter Registers (YUKON only)

//
// Marvel-PHY Registers, indirect addressed over GMAC
//
// Marvel-specific registers
// for 10/100 Fast Ethernet PHY (88E3082 only)
// different Marvell PHY Ids
// Advertisement register bits
// PHY_BCOM_1000T_STAT	16 bit r/o	1000Base-T Status Reg
// PHY_MARV_1000T_STAT	16 bit r/o	1000Base-T Status Reg
// Bit  9..8:	reserved
// Marvell-Specific
// special defines for FIBER (88E1011S only)
// Pause Bits (PHY_M_AN_ASP_X and PHY_M_AN_PC_X) encoding
// PHY_MARV_1000T_CTRL	16 bit r/w	1000Base-T Control Reg
// PHY_MARV_PHY_CTRL	16 bit r/w	PHY Specific Ctrl Reg

// for Yukon-EC Ultra Gigabit Ethernet PHY (88E1149 only)
// for 10/100 Fast Ethernet PHY (88E3082 only)
// PHY_MARV_PHY_STAT	16 bit r/o	PHY Specific Status Reg

// for 10/100 Fast Ethernet PHY (88E3082 only)
// PHY_MARV_EXT_CTRL	16 bit r/w	Ext. PHY Specific Ctrl
// (88E1011 only)
// (88E1111 only)
// !!! Errata in spec. (1 = disable)

// 00=1x; 01=2x; 10=3x; 11=4x

// 00=dis; 01=1x; 10=2x; 11=3x

// 000=1x; 001=2x; 010=3x; 011=4x

// 01X=0; 110=2.5; 111=25 (MHz)
// for Yukon-2 Gigabit Ethernet PHY (88E1112 only)
// !!! Errata in spec. (1 = disable)

// 100=5x; 101=6x; 110=7x; 111=8x
// PHY_MARV_LED_CTRL	16 bit r/w	LED Control Reg
// (88E1111 only)
// (88E1011 only)

// PHY_MARV_PHY_STAT (page 3)16 bit r/w	Polarity Control Reg.

// PHY_MARV_LED_OVER	16 bit r/w	Manual LED Override Reg

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum led_mode {
    MO_LED_NORM  = 0,
    MO_LED_BLINK = 1,
    MO_LED_OFF   = 2,
    MO_LED_ON    = 3,
}

// PHY_MARV_EXT_CTRL_2	16 bit r/w	Ext. PHY Specific Ctrl 2
// PHY_MARV_EXT_P_STAT 16 bit r/w	Ext. PHY Specific Status
// (88E1111 only)
// for 10/100 Fast Ethernet PHY (88E3082 only)
// PHY_MARV_FE_LED_PAR		16 bit r/w	LED Parallel Select Reg.
// Bit 15..12: reserved (used internally)

// ,PHY_MARV_FE_SPEC_2		16 bit r/w	Specific Control Reg. 2
// for Yukon-2 Gigabit Ethernet PHY (88E1112 only)
// PHY_MARV_PHY_CTRL (page 1)		16 bit r/w	Fiber Specific Ctrl
// for Yukon-2 Gigabit Ethernet PHY (88E1112 only)
// PHY_MARV_PHY_CTRL (page 2)		16 bit r/w	MAC Specific Ctrl

// PHY_MARV_PHY_CTRL (page 3)		16 bit r/w	LED Control Reg.

// GMAC registers
// Port Registers
// Source Address Registers
// Multicast Address Hash Registers
// Interrupt Source Registers
// Interrupt Mask Registers
// Serial Management Interface (SMI) Registers
// MIB Counters
//
// MIB Counters base address definitions (low word) -
// use offset 4 for access to high word	(32 bit r/o)
//
// GMAC Bit Definitions
// GM_GP_STAT	16 bit r/o	General Purpose Status Register
// GM_GP_CTRL	16 bit r/w	General Purpose Control Register

// GM_TX_CTRL			16 bit r/w	Transmit Control Register

pub const TX_COL_DEF: c_uint = 0x04;
// GM_RX_CTRL			16 bit r/w	Receive Control Register
// GM_TX_PARAM		16 bit r/w	Transmit Parameter Register

// GM_SERIAL_MODE			16 bit r/w	Serial Mode Register

pub const DATA_BLIND_DEF: c_uint = 0x04;
pub const IPG_DATA_DEF_1000: c_uint = 0x1e;
pub const IPG_DATA_DEF_10_100: c_uint = 0x18;
// GM_SMI_CTRL			16 bit r/w	SMI Control Register

// GM_PHY_ADDR				16 bit r/w	GPHY Address Register
// Receive Frame Status Encoding
// RX_GMF_CTRL_T	32 bit	Rx GMAC FIFO Control/Test
// RX_GMF_FL_CTRL	16 bit	Rx GMAC FIFO Flush Control (Yukon-Supreme)
// TX_GMF_EA		32 bit	Tx GMAC FIFO End Address
// TX_GMF_CTRL_T	32 bit	Tx GMAC FIFO Control/Test
// GMAC_TI_ST_CTRL	 8 bit	Time Stamp Timer Ctrl Reg (YUKON only)
// B28_Y2_ASF_STAT_CMD		32 bit	ASF Status and Command Reg
// B28_Y2_ASF_HOST_COM	32 bit	ASF Host Communication Reg
// HCU_CCSR	CPU Control and Status Register
// Clock Stretching Timeout
// Microcontroller State
// HCU_HCSR	Host Control and Status Register
// STAT_CTRL		32 bit	Status BMU control register (Yukon-2 only)
// GMAC_CTRL		32 bit	GMAC Control Reg (YUKON only)
// GPHY_CTRL		32 bit	GPHY Control Reg (YUKON only)
// GMAC_IRQ_SRC	 8 bit	GMAC Interrupt Source Reg (YUKON only)
// GMAC_IRQ_MSK	 8 bit	GMAC Interrupt Mask   Reg (YUKON only)

// GMAC_LINK_CTRL	16 bit	GMAC Link Control Reg (YUKON only)
// WOL_CTRL_STAT	16 bit	WOL Control/Status Reg
// Control flags
// YUKON-2 STATUS opcodes defines
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum status_css {
    CSS_TCPUDPCSOK	= 1<<7,	/* TCP / UDP checksum is ok */
    CSS_ISUDP	= 1<<6, /* packet is a UDP packet */
    CSS_ISTCP	= 1<<5, /* packet is a TCP packet */
    CSS_ISIPFRAG	= 1<<4, /* packet is a TCP/UDP frag, CS calc not done */
    CSS_ISIPV6	= 1<<3, /* packet is a IPv6 packet */
    CSS_IPV4CSUMOK	= 1<<2, /* IP v4: TCP header checksum is ok */
    CSS_ISIPV4	= 1<<1, /* packet is a IPv4 packet */
    CSS_LINK_BIT	= 1<<0, /* port number (legacy) */
}

// Yukon 2 hardware interface
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sky2_tx_le {
    pub addr: __le32,
    pub /: *mut *mut __le16 length; / also vlan tag or checksum start,
    pub ctrl: u8,
    pub opcode: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sky2_rx_le {
    pub addr: __le32,
    pub length: __le16,
    pub ctrl: u8,
    pub opcode: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sky2_status_le {
    pub /: *mut *mut __le32 status; / also checksum,
    pub /: *mut *mut __le16 length; / also vlan tag,
    pub css: u8,
    pub opcode: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_ring_info {
    pub skb: *mut sk_buff,
    pub flags: c_ulong,
pub const TX_MAP_SINGLE: c_uint = 0x0001;
pub const TX_MAP_PAGE: c_uint = 0x0002;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_ring_info {
    pub skb: *mut sk_buff,
    pub data_addr: dma_addr_t,
    pub 1]: dma_addr_t frag_addr[ETH_JUMBO_MTU >> PAGE_SHIFT ?:,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flow_control {
    FC_NONE	= 0,
    FC_TX	= 1,
    FC_RX	= 2,
    FC_BOTH	= 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sky2_stats {
    pub syncp: u64_stats_sync,
    pub packets: u64,
    pub bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sky2_port {
    pub hw: *mut sky2_hw,
    pub netdev: *mut net_device,
    pub port: unsigned,
    pub msg_enable: u32,
    pub phy_lock: spinlock_t,
    pub tx_ring: *mut tx_ring_info,
    pub tx_le: *mut sky2_tx_le,
    pub tx_stats: sky2_stats,
    pub tx_ring_size: u16,
    pub /: *mut *mut u16 tx_cons; / next le to check,
    pub /: *mut *mut u16 tx_prod; / next le to use,
    pub /: *mut *mut u16 tx_next; / debug only,
    pub tx_pending: u16,
    pub tx_last_mss: u16,
    pub tx_last_upper: u32,
    pub tx_tcpsum: u32,
    pub ____cacheline_aligned_in_smp: *mut *mut rx_ring_info rx_ring,
    pub rx_le: *mut sky2_rx_le,
    pub rx_stats: sky2_stats,
    pub /: *mut *mut u16 rx_next; / next re to check,
    pub /: *mut *mut u16 rx_put; / next le index to use,
    pub rx_pending: u16,
    pub rx_data_size: u16,
    pub rx_nfrags: u16,
    pub last_rx: c_ulong,
    pub last: c_ulong,
    pub mac_rp: u32,
    pub mac_lev: u8,
    pub fifo_rp: u8,
    pub fifo_lev: u8,
    pub check: },
    pub rx_le_map: dma_addr_t,
    pub tx_le_map: dma_addr_t,
    pub /: *mut *mut u16 advertising; / ADVERTISED_ bits,
    pub /: *mut *mut u16 speed; / SPEED_1000, SPEED_100, ...,
    pub /: *mut *mut u8 wol; / WAKE_ bits,
    pub /: *mut *mut u8 duplex; / DUPLEX_HALF, DUPLEX_FULL,
    pub flags: u16,
pub const SKY2_FLAG_AUTO_SPEED: c_uint = 0x0002;
pub const SKY2_FLAG_AUTO_PAUSE: c_uint = 0x0004;
    pub flow_mode: flow_control,
    pub flow_status: flow_control,

    pub debugfs: *mut dentry,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sky2_hw {
    pub regs: *mut void __iomem,
    pub pdev: *mut pci_dev,
    pub napi: napi_struct,
    pub dev: [*mut net_device; 2],
    pub flags: c_ulong,
pub const SKY2_HW_USE_MSI: c_uint = 0x00000001;
pub const SKY2_HW_FIBRE_PHY: c_uint = 0x00000002;
pub const SKY2_HW_GIGABIT: c_uint = 0x00000004;
pub const SKY2_HW_NEWER_PHY: c_uint = 0x00000008;
pub const SKY2_HW_RAM_BUFFER: c_uint = 0x00000010;
pub const SKY2_HW_NEW_LE: c_uint = 0x00000020	/* new LSOv2 format */;
pub const SKY2_HW_AUTO_TX_SUM: c_uint = 0x00000040	/* new IP decode for Tx */;
pub const SKY2_HW_ADV_POWER_CTL: c_uint = 0x00000080	/* additional PHY power regs */;
pub const SKY2_HW_RSS_BROKEN: c_uint = 0x00000100;
pub const SKY2_HW_VLAN_BROKEN: c_uint = 0x00000200;
pub const SKY2_HW_RSS_CHKSUM: c_uint = 0x00000400	/* RSS requires chksum */;
pub const SKY2_HW_IRQ_SETUP: c_uint = 0x00000800;
    pub chip_id: u8,
    pub chip_rev: u8,
    pub pmd_type: u8,
    pub ports: u8,
    pub st_le: *mut sky2_status_le,
    pub st_size: u32,
    pub st_idx: u32,
    pub st_dma: dma_addr_t,
    pub watchdog_timer: timer_list,
    pub restart_work: work_struct,
    pub msi_wait: wait_queue_head_t,
    pub irq_name: [c_char; ],
}

// Register accessor for memory mapped device
extern "C" {
    pub fn readl(reg: hw->regs +) -> return;
}
extern "C" {
    pub fn readw(reg: hw->regs +) -> return;
}
extern "C" {
    pub fn readb(reg: hw->regs +) -> return;
}
// Yukon PHY related registers

pub const GM_PHY_RETRIES: c_int = 100;
extern "C" {
    pub fn sky2_read16(_arg: hw, _arg: SK_GMAC_REG(port, _arg: reg)) -> return;
}
// There is no way to atomically read32 bit values from PHY, so retry
// PCI config space access
extern "C" {
    pub fn sky2_read32(_arg: hw, reg: Y2_CFG_SPC +) -> return;
}
extern "C" {
    pub fn sky2_read16(_arg: hw, reg: Y2_CFG_SPC +) -> return;
}
