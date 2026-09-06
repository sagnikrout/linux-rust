//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bcma/bcma_driver_pcie2.h
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
pub const BCMA_CORE_PCIE2_CLK_CONTROL: c_uint = 0x0000;
pub const PCIE2_CLKC_RST_OE: c_uint = 0x0001 /* When set, drives PCI_RESET out to pin */;
pub const PCIE2_CLKC_RST: c_uint = 0x0002 /* Value driven out to pin */;
pub const PCIE2_CLKC_SPERST: c_uint = 0x0004 /* SurvivePeRst */;
pub const PCIE2_CLKC_DISABLE_L1CLK_GATING: c_uint = 0x0010;
pub const PCIE2_CLKC_DLYPERST: c_uint = 0x0100 /* Delay PeRst to CoE Core */;
pub const PCIE2_CLKC_DISSPROMLD: c_uint = 0x0200 /* DisableSpromLoadOnPerst */;
pub const PCIE2_CLKC_WAKE_MODE_L2: c_uint = 0x1000 /* Wake on L2 */;
pub const BCMA_CORE_PCIE2_RC_PM_CONTROL: c_uint = 0x0004;
pub const BCMA_CORE_PCIE2_RC_PM_STATUS: c_uint = 0x0008;
pub const BCMA_CORE_PCIE2_EP_PM_CONTROL: c_uint = 0x000C;
pub const BCMA_CORE_PCIE2_EP_PM_STATUS: c_uint = 0x0010;
pub const BCMA_CORE_PCIE2_EP_LTR_CONTROL: c_uint = 0x0014;
pub const BCMA_CORE_PCIE2_EP_LTR_STATUS: c_uint = 0x0018;
pub const BCMA_CORE_PCIE2_EP_OBFF_STATUS: c_uint = 0x001C;
pub const BCMA_CORE_PCIE2_PCIE_ERR_STATUS: c_uint = 0x0020;
pub const BCMA_CORE_PCIE2_RC_AXI_CONFIG: c_uint = 0x0100;
pub const BCMA_CORE_PCIE2_EP_AXI_CONFIG: c_uint = 0x0104;
pub const BCMA_CORE_PCIE2_RXDEBUG_STATUS0: c_uint = 0x0108;
pub const BCMA_CORE_PCIE2_RXDEBUG_CONTROL0: c_uint = 0x010C;
pub const BCMA_CORE_PCIE2_CONFIGINDADDR: c_uint = 0x0120;
pub const BCMA_CORE_PCIE2_CONFIGINDDATA: c_uint = 0x0124;
pub const BCMA_CORE_PCIE2_MDIOCONTROL: c_uint = 0x0128;
pub const BCMA_CORE_PCIE2_MDIOWRDATA: c_uint = 0x012C;
pub const BCMA_CORE_PCIE2_MDIORDDATA: c_uint = 0x0130;
pub const BCMA_CORE_PCIE2_DATAINTF: c_uint = 0x0180;
pub const BCMA_CORE_PCIE2_D2H_INTRLAZY_0: c_uint = 0x0188;
pub const BCMA_CORE_PCIE2_H2D_INTRLAZY_0: c_uint = 0x018c;
pub const BCMA_CORE_PCIE2_H2D_INTSTAT_0: c_uint = 0x0190;
pub const BCMA_CORE_PCIE2_H2D_INTMASK_0: c_uint = 0x0194;
pub const BCMA_CORE_PCIE2_D2H_INTSTAT_0: c_uint = 0x0198;
pub const BCMA_CORE_PCIE2_D2H_INTMASK_0: c_uint = 0x019c;
pub const BCMA_CORE_PCIE2_LTR_STATE: c_uint = 0x01A0 /* Latency Tolerance Reporting */;
pub const PCIE2_LTR_ACTIVE: c_int = 2;
pub const PCIE2_LTR_ACTIVE_IDLE: c_int = 1;
pub const PCIE2_LTR_SLEEP: c_int = 0;
pub const PCIE2_LTR_FINAL_MASK: c_uint = 0x300;
pub const PCIE2_LTR_FINAL_SHIFT: c_int = 8;
pub const BCMA_CORE_PCIE2_PWR_INT_STATUS: c_uint = 0x01A4;
pub const BCMA_CORE_PCIE2_PWR_INT_MASK: c_uint = 0x01A8;
pub const BCMA_CORE_PCIE2_CFG_ADDR: c_uint = 0x01F8;
pub const BCMA_CORE_PCIE2_CFG_DATA: c_uint = 0x01FC;
pub const BCMA_CORE_PCIE2_SYS_EQ_PAGE: c_uint = 0x0200;
pub const BCMA_CORE_PCIE2_SYS_MSI_PAGE: c_uint = 0x0204;
pub const BCMA_CORE_PCIE2_SYS_MSI_INTREN: c_uint = 0x0208;
pub const BCMA_CORE_PCIE2_SYS_MSI_CTRL0: c_uint = 0x0210;
pub const BCMA_CORE_PCIE2_SYS_MSI_CTRL1: c_uint = 0x0214;
pub const BCMA_CORE_PCIE2_SYS_MSI_CTRL2: c_uint = 0x0218;
pub const BCMA_CORE_PCIE2_SYS_MSI_CTRL3: c_uint = 0x021C;
pub const BCMA_CORE_PCIE2_SYS_MSI_CTRL4: c_uint = 0x0220;
pub const BCMA_CORE_PCIE2_SYS_MSI_CTRL5: c_uint = 0x0224;
pub const BCMA_CORE_PCIE2_SYS_EQ_HEAD0: c_uint = 0x0250;
pub const BCMA_CORE_PCIE2_SYS_EQ_TAIL0: c_uint = 0x0254;
pub const BCMA_CORE_PCIE2_SYS_EQ_HEAD1: c_uint = 0x0258;
pub const BCMA_CORE_PCIE2_SYS_EQ_TAIL1: c_uint = 0x025C;
pub const BCMA_CORE_PCIE2_SYS_EQ_HEAD2: c_uint = 0x0260;
pub const BCMA_CORE_PCIE2_SYS_EQ_TAIL2: c_uint = 0x0264;
pub const BCMA_CORE_PCIE2_SYS_EQ_HEAD3: c_uint = 0x0268;
pub const BCMA_CORE_PCIE2_SYS_EQ_TAIL3: c_uint = 0x026C;
pub const BCMA_CORE_PCIE2_SYS_EQ_HEAD4: c_uint = 0x0270;
pub const BCMA_CORE_PCIE2_SYS_EQ_TAIL4: c_uint = 0x0274;
pub const BCMA_CORE_PCIE2_SYS_EQ_HEAD5: c_uint = 0x0278;
pub const BCMA_CORE_PCIE2_SYS_EQ_TAIL5: c_uint = 0x027C;
pub const BCMA_CORE_PCIE2_SYS_RC_INTX_EN: c_uint = 0x0330;
pub const BCMA_CORE_PCIE2_SYS_RC_INTX_CSR: c_uint = 0x0334;
pub const BCMA_CORE_PCIE2_SYS_MSI_REQ: c_uint = 0x0340;
pub const BCMA_CORE_PCIE2_SYS_HOST_INTR_EN: c_uint = 0x0344;
pub const BCMA_CORE_PCIE2_SYS_HOST_INTR_CSR: c_uint = 0x0348;
pub const BCMA_CORE_PCIE2_SYS_HOST_INTR0: c_uint = 0x0350;
pub const BCMA_CORE_PCIE2_SYS_HOST_INTR1: c_uint = 0x0354;
pub const BCMA_CORE_PCIE2_SYS_HOST_INTR2: c_uint = 0x0358;
pub const BCMA_CORE_PCIE2_SYS_HOST_INTR3: c_uint = 0x035C;
pub const BCMA_CORE_PCIE2_SYS_EP_INT_EN0: c_uint = 0x0360;
pub const BCMA_CORE_PCIE2_SYS_EP_INT_EN1: c_uint = 0x0364;
pub const BCMA_CORE_PCIE2_SYS_EP_INT_CSR0: c_uint = 0x0370;
pub const BCMA_CORE_PCIE2_SYS_EP_INT_CSR1: c_uint = 0x0374;

pub const BCMA_CORE_PCIE2_FUNC0_IMAP0_0: c_uint = 0x0C00;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP0_1: c_uint = 0x0C04;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP0_2: c_uint = 0x0C08;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP0_3: c_uint = 0x0C0C;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP0_4: c_uint = 0x0C10;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP0_5: c_uint = 0x0C14;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP0_6: c_uint = 0x0C18;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP0_7: c_uint = 0x0C1C;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP0_0: c_uint = 0x0C20;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP0_1: c_uint = 0x0C24;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP0_2: c_uint = 0x0C28;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP0_3: c_uint = 0x0C2C;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP0_4: c_uint = 0x0C30;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP0_5: c_uint = 0x0C34;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP0_6: c_uint = 0x0C38;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP0_7: c_uint = 0x0C3C;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP1: c_uint = 0x0C80;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP1: c_uint = 0x0C88;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP2: c_uint = 0x0CC0;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP2: c_uint = 0x0CC8;
pub const BCMA_CORE_PCIE2_IARR0_LOWER: c_uint = 0x0D00;
pub const BCMA_CORE_PCIE2_IARR0_UPPER: c_uint = 0x0D04;
pub const BCMA_CORE_PCIE2_IARR1_LOWER: c_uint = 0x0D08;
pub const BCMA_CORE_PCIE2_IARR1_UPPER: c_uint = 0x0D0C;
pub const BCMA_CORE_PCIE2_IARR2_LOWER: c_uint = 0x0D10;
pub const BCMA_CORE_PCIE2_IARR2_UPPER: c_uint = 0x0D14;
pub const BCMA_CORE_PCIE2_OARR0: c_uint = 0x0D20;
pub const BCMA_CORE_PCIE2_OARR1: c_uint = 0x0D28;
pub const BCMA_CORE_PCIE2_OARR2: c_uint = 0x0D30;
pub const BCMA_CORE_PCIE2_OMAP0_LOWER: c_uint = 0x0D40;
pub const BCMA_CORE_PCIE2_OMAP0_UPPER: c_uint = 0x0D44;
pub const BCMA_CORE_PCIE2_OMAP1_LOWER: c_uint = 0x0D48;
pub const BCMA_CORE_PCIE2_OMAP1_UPPER: c_uint = 0x0D4C;
pub const BCMA_CORE_PCIE2_OMAP2_LOWER: c_uint = 0x0D50;
pub const BCMA_CORE_PCIE2_OMAP2_UPPER: c_uint = 0x0D54;
pub const BCMA_CORE_PCIE2_FUNC1_IARR1_SIZE: c_uint = 0x0D58;
pub const BCMA_CORE_PCIE2_FUNC1_IARR2_SIZE: c_uint = 0x0D5C;
pub const BCMA_CORE_PCIE2_MEM_CONTROL: c_uint = 0x0F00;
pub const BCMA_CORE_PCIE2_MEM_ECC_ERRLOG0: c_uint = 0x0F04;
pub const BCMA_CORE_PCIE2_MEM_ECC_ERRLOG1: c_uint = 0x0F08;
pub const BCMA_CORE_PCIE2_LINK_STATUS: c_uint = 0x0F0C;
pub const BCMA_CORE_PCIE2_STRAP_STATUS: c_uint = 0x0F10;
pub const BCMA_CORE_PCIE2_RESET_STATUS: c_uint = 0x0F14;
pub const BCMA_CORE_PCIE2_RESETEN_IN_LINKDOWN: c_uint = 0x0F18;
pub const BCMA_CORE_PCIE2_MISC_INTR_EN: c_uint = 0x0F1C;
pub const BCMA_CORE_PCIE2_TX_DEBUG_CFG: c_uint = 0x0F20;
pub const BCMA_CORE_PCIE2_MISC_CONFIG: c_uint = 0x0F24;
pub const BCMA_CORE_PCIE2_MISC_STATUS: c_uint = 0x0F28;
pub const BCMA_CORE_PCIE2_INTR_EN: c_uint = 0x0F30;
pub const BCMA_CORE_PCIE2_INTR_CLEAR: c_uint = 0x0F34;
pub const BCMA_CORE_PCIE2_INTR_STATUS: c_uint = 0x0F38;
// PCIE gen2 config regs
pub const PCIE2_INTSTATUS: c_uint = 0x090;
pub const PCIE2_INTMASK: c_uint = 0x094;
pub const PCIE2_SBMBX: c_uint = 0x098;
pub const PCIE2_PMCR_REFUP: c_uint = 0x1814 /* Trefup time */;
pub const PCIE2_CAP_DEVSTSCTRL2_OFFSET: c_uint = 0xD4;
pub const PCIE2_CAP_DEVSTSCTRL2_LTRENAB: c_uint = 0x400;
pub const PCIE2_PVT_REG_PM_CLK_PERIOD: c_uint = 0x184c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcma_drv_pcie2 {
    pub core: *mut bcma_device,
    pub reqsize: u16,
}

