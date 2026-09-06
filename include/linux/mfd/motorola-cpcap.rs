//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/motorola-cpcap.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// The register defines are based on earlier cpcap.h in Motorola Linux kernel
// tree.
//
// Copyright (C) 2007-2009 Motorola, Inc.
//
// Rewritten for the real register offsets instead of enumeration
// to make the defines usable with Linux kernel regmap support
//
// Copyright (C) 2016 Tony Lindgren <tony@atomide.com>
//

pub const CPCAP_VENDOR_ST: c_int = 0;
pub const CPCAP_VENDOR_TI: c_int = 1;

pub const CPCAP_REVISION_1_0: c_uint = 0x08;
pub const CPCAP_REVISION_1_1: c_uint = 0x09;
pub const CPCAP_REVISION_2_0: c_uint = 0x10;
pub const CPCAP_REVISION_2_1: c_uint = 0x11;
// CPCAP registers
pub const CPCAP_REG_INT1: c_uint = 0x0000	/* Interrupt 1 */;
pub const CPCAP_REG_INT2: c_uint = 0x0004	/* Interrupt 2 */;
pub const CPCAP_REG_INT3: c_uint = 0x0008	/* Interrupt 3 */;
pub const CPCAP_REG_INT4: c_uint = 0x000c	/* Interrupt 4 */;
pub const CPCAP_REG_INTM1: c_uint = 0x0010	/* Interrupt Mask 1 */;
pub const CPCAP_REG_INTM2: c_uint = 0x0014	/* Interrupt Mask 2 */;
pub const CPCAP_REG_INTM3: c_uint = 0x0018	/* Interrupt Mask 3 */;
pub const CPCAP_REG_INTM4: c_uint = 0x001c	/* Interrupt Mask 4 */;
pub const CPCAP_REG_INTS1: c_uint = 0x0020	/* Interrupt Sense 1 */;
pub const CPCAP_REG_INTS2: c_uint = 0x0024	/* Interrupt Sense 2 */;
pub const CPCAP_REG_INTS3: c_uint = 0x0028	/* Interrupt Sense 3 */;
pub const CPCAP_REG_INTS4: c_uint = 0x002c	/* Interrupt Sense 4 */;
pub const CPCAP_REG_ASSIGN1: c_uint = 0x0030	/* Resource Assignment 1 */;
pub const CPCAP_REG_ASSIGN2: c_uint = 0x0034	/* Resource Assignment 2 */;
pub const CPCAP_REG_ASSIGN3: c_uint = 0x0038	/* Resource Assignment 3 */;
pub const CPCAP_REG_ASSIGN4: c_uint = 0x003c	/* Resource Assignment 4 */;
pub const CPCAP_REG_ASSIGN5: c_uint = 0x0040	/* Resource Assignment 5 */;
pub const CPCAP_REG_ASSIGN6: c_uint = 0x0044	/* Resource Assignment 6 */;
pub const CPCAP_REG_VERSC1: c_uint = 0x0048	/* Version Control 1 */;
pub const CPCAP_REG_VERSC2: c_uint = 0x004c	/* Version Control 2 */;
pub const CPCAP_REG_MI1: c_uint = 0x0200	/* Macro Interrupt 1 */;
pub const CPCAP_REG_MIM1: c_uint = 0x0204	/* Macro Interrupt Mask 1 */;
pub const CPCAP_REG_MI2: c_uint = 0x0208	/* Macro Interrupt 2 */;
pub const CPCAP_REG_MIM2: c_uint = 0x020c	/* Macro Interrupt Mask 2 */;
pub const CPCAP_REG_UCC1: c_uint = 0x0210	/* UC Control 1 */;
pub const CPCAP_REG_UCC2: c_uint = 0x0214	/* UC Control 2 */;
pub const CPCAP_REG_PC1: c_uint = 0x021c	/* Power Cut 1 */;
pub const CPCAP_REG_PC2: c_uint = 0x0220	/* Power Cut 2 */;
pub const CPCAP_REG_BPEOL: c_uint = 0x0224	/* BP and EOL */;
pub const CPCAP_REG_PGC: c_uint = 0x0228	/* Power Gate and Control */;
pub const CPCAP_REG_MT1: c_uint = 0x022c	/* Memory Transfer 1 */;
pub const CPCAP_REG_MT2: c_uint = 0x0230	/* Memory Transfer 2 */;
pub const CPCAP_REG_MT3: c_uint = 0x0234	/* Memory Transfer 3 */;
pub const CPCAP_REG_PF: c_uint = 0x0238	/* Print Format */;
pub const CPCAP_REG_SCC: c_uint = 0x0400	/* System Clock Control */;
pub const CPCAP_REG_SW1: c_uint = 0x0404	/* Stop Watch 1 */;
pub const CPCAP_REG_SW2: c_uint = 0x0408	/* Stop Watch 2 */;
pub const CPCAP_REG_UCTM: c_uint = 0x040c	/* UC Turbo Mode */;
pub const CPCAP_REG_TOD1: c_uint = 0x0410	/* Time of Day 1 */;
pub const CPCAP_REG_TOD2: c_uint = 0x0414	/* Time of Day 2 */;
pub const CPCAP_REG_TODA1: c_uint = 0x0418	/* Time of Day Alarm 1 */;
pub const CPCAP_REG_TODA2: c_uint = 0x041c	/* Time of Day Alarm 2 */;
pub const CPCAP_REG_DAY: c_uint = 0x0420	/* Day */;
pub const CPCAP_REG_DAYA: c_uint = 0x0424	/* Day Alarm */;
pub const CPCAP_REG_VAL1: c_uint = 0x0428	/* Validity 1 */;
pub const CPCAP_REG_VAL2: c_uint = 0x042c	/* Validity 2 */;
pub const CPCAP_REG_SDVSPLL: c_uint = 0x0600	/* Switcher DVS and PLL */;
pub const CPCAP_REG_SI2CC1: c_uint = 0x0604	/* Switcher I2C Control 1 */;
pub const CPCAP_REG_Si2CC2: c_uint = 0x0608	/* Switcher I2C Control 2 */;
pub const CPCAP_REG_S1C1: c_uint = 0x060c	/* Switcher 1 Control 1 */;
pub const CPCAP_REG_S1C2: c_uint = 0x0610	/* Switcher 1 Control 2 */;
pub const CPCAP_REG_S2C1: c_uint = 0x0614	/* Switcher 2 Control 1 */;
pub const CPCAP_REG_S2C2: c_uint = 0x0618	/* Switcher 2 Control 2 */;
pub const CPCAP_REG_S3C: c_uint = 0x061c	/* Switcher 3 Control */;
pub const CPCAP_REG_S4C1: c_uint = 0x0620	/* Switcher 4 Control 1 */;
pub const CPCAP_REG_S4C2: c_uint = 0x0624	/* Switcher 4 Control 2 */;
pub const CPCAP_REG_S5C: c_uint = 0x0628	/* Switcher 5 Control */;
pub const CPCAP_REG_S6C: c_uint = 0x062c	/* Switcher 6 Control */;
pub const CPCAP_REG_VCAMC: c_uint = 0x0630	/* VCAM Control */;
pub const CPCAP_REG_VCSIC: c_uint = 0x0634	/* VCSI Control */;
pub const CPCAP_REG_VDACC: c_uint = 0x0638	/* VDAC Control */;
pub const CPCAP_REG_VDIGC: c_uint = 0x063c	/* VDIG Control */;
pub const CPCAP_REG_VFUSEC: c_uint = 0x0640	/* VFUSE Control */;
pub const CPCAP_REG_VHVIOC: c_uint = 0x0644	/* VHVIO Control */;
pub const CPCAP_REG_VSDIOC: c_uint = 0x0648	/* VSDIO Control */;
pub const CPCAP_REG_VPLLC: c_uint = 0x064c	/* VPLL Control */;
pub const CPCAP_REG_VRF1C: c_uint = 0x0650	/* VRF1 Control */;
pub const CPCAP_REG_VRF2C: c_uint = 0x0654	/* VRF2 Control */;
pub const CPCAP_REG_VRFREFC: c_uint = 0x0658	/* VRFREF Control */;
pub const CPCAP_REG_VWLAN1C: c_uint = 0x065c	/* VWLAN1 Control */;
pub const CPCAP_REG_VWLAN2C: c_uint = 0x0660	/* VWLAN2 Control */;
pub const CPCAP_REG_VSIMC: c_uint = 0x0664	/* VSIM Control */;
pub const CPCAP_REG_VVIBC: c_uint = 0x0668	/* VVIB Control */;
pub const CPCAP_REG_VUSBC: c_uint = 0x066c	/* VUSB Control */;
pub const CPCAP_REG_VUSBINT1C: c_uint = 0x0670	/* VUSBINT1 Control */;
pub const CPCAP_REG_VUSBINT2C: c_uint = 0x0674	/* VUSBINT2 Control */;
pub const CPCAP_REG_URT: c_uint = 0x0678	/* Useroff Regulator Trigger */;
pub const CPCAP_REG_URM1: c_uint = 0x067c	/* Useroff Regulator Mask 1 */;
pub const CPCAP_REG_URM2: c_uint = 0x0680	/* Useroff Regulator Mask 2 */;
pub const CPCAP_REG_VAUDIOC: c_uint = 0x0800	/* VAUDIO Control */;
pub const CPCAP_REG_CC: c_uint = 0x0804	/* Codec Control */;
pub const CPCAP_REG_CDI: c_uint = 0x0808	/* Codec Digital Interface */;
pub const CPCAP_REG_SDAC: c_uint = 0x080c	/* Stereo DAC */;
pub const CPCAP_REG_SDACDI: c_uint = 0x0810	/* Stereo DAC Digital Interface */;
pub const CPCAP_REG_TXI: c_uint = 0x0814	/* TX Inputs */;
pub const CPCAP_REG_TXMP: c_uint = 0x0818	/* TX MIC PGA's */;
pub const CPCAP_REG_RXOA: c_uint = 0x081c	/* RX Output Amplifiers */;
pub const CPCAP_REG_RXVC: c_uint = 0x0820	/* RX Volume Control */;
pub const CPCAP_REG_RXCOA: c_uint = 0x0824	/* RX Codec to Output Amps */;
pub const CPCAP_REG_RXSDOA: c_uint = 0x0828	/* RX Stereo DAC to Output Amps */;
pub const CPCAP_REG_RXEPOA: c_uint = 0x082c	/* RX External PGA to Output Amps */;
pub const CPCAP_REG_RXLL: c_uint = 0x0830	/* RX Low Latency */;
pub const CPCAP_REG_A2LA: c_uint = 0x0834	/* A2 Loudspeaker Amplifier */;
pub const CPCAP_REG_MIPIS1: c_uint = 0x0838	/* MIPI Slimbus 1 */;
pub const CPCAP_REG_MIPIS2: c_uint = 0x083c	/* MIPI Slimbus 2 */;
pub const CPCAP_REG_MIPIS3: c_uint = 0x0840	/* MIPI Slimbus 3. */;
pub const CPCAP_REG_LVAB: c_uint = 0x0844	/* LMR Volume and A4 Balanced. */;
pub const CPCAP_REG_CCC1: c_uint = 0x0a00	/* Coulomb Counter Control 1 */;
pub const CPCAP_REG_CRM: c_uint = 0x0a04	/* Charger and Reverse Mode */;
pub const CPCAP_REG_CCCC2: c_uint = 0x0a08	/* Coincell and Coulomb Ctr Ctrl 2 */;
pub const CPCAP_REG_CCS1: c_uint = 0x0a0c	/* Coulomb Counter Sample 1 */;
pub const CPCAP_REG_CCS2: c_uint = 0x0a10	/* Coulomb Counter Sample 2 */;
pub const CPCAP_REG_CCA1: c_uint = 0x0a14	/* Coulomb Counter Accumulator 1 */;
pub const CPCAP_REG_CCA2: c_uint = 0x0a18	/* Coulomb Counter Accumulator 2 */;
pub const CPCAP_REG_CCM: c_uint = 0x0a1c	/* Coulomb Counter Mode */;
pub const CPCAP_REG_CCO: c_uint = 0x0a20	/* Coulomb Counter Offset */;
pub const CPCAP_REG_CCI: c_uint = 0x0a24	/* Coulomb Counter Integrator */;
pub const CPCAP_REG_ADCC1: c_uint = 0x0c00	/* A/D Converter Configuration 1 */;
pub const CPCAP_REG_ADCC2: c_uint = 0x0c04	/* A/D Converter Configuration 2 */;
pub const CPCAP_REG_ADCD0: c_uint = 0x0c08	/* A/D Converter Data 0 */;
pub const CPCAP_REG_ADCD1: c_uint = 0x0c0c	/* A/D Converter Data 1 */;
pub const CPCAP_REG_ADCD2: c_uint = 0x0c10	/* A/D Converter Data 2 */;
pub const CPCAP_REG_ADCD3: c_uint = 0x0c14	/* A/D Converter Data 3 */;
pub const CPCAP_REG_ADCD4: c_uint = 0x0c18	/* A/D Converter Data 4 */;
pub const CPCAP_REG_ADCD5: c_uint = 0x0c1c	/* A/D Converter Data 5 */;
pub const CPCAP_REG_ADCD6: c_uint = 0x0c20	/* A/D Converter Data 6 */;
pub const CPCAP_REG_ADCD7: c_uint = 0x0c24	/* A/D Converter Data 7 */;
pub const CPCAP_REG_ADCAL1: c_uint = 0x0c28	/* A/D Converter Calibration 1 */;
pub const CPCAP_REG_ADCAL2: c_uint = 0x0c2c	/* A/D Converter Calibration 2 */;
pub const CPCAP_REG_USBC1: c_uint = 0x0e00	/* USB Control 1 */;
pub const CPCAP_REG_USBC2: c_uint = 0x0e04	/* USB Control 2 */;
pub const CPCAP_REG_USBC3: c_uint = 0x0e08	/* USB Control 3 */;
pub const CPCAP_REG_UVIDL: c_uint = 0x0e0c	/* ULPI Vendor ID Low */;
pub const CPCAP_REG_UVIDH: c_uint = 0x0e10	/* ULPI Vendor ID High */;
pub const CPCAP_REG_UPIDL: c_uint = 0x0e14	/* ULPI Product ID Low */;
pub const CPCAP_REG_UPIDH: c_uint = 0x0e18	/* ULPI Product ID High */;
pub const CPCAP_REG_UFC1: c_uint = 0x0e1c	/* ULPI Function Control 1 */;
pub const CPCAP_REG_UFC2: c_uint = 0x0e20	/* ULPI Function Control 2 */;
pub const CPCAP_REG_UFC3: c_uint = 0x0e24	/* ULPI Function Control 3 */;
pub const CPCAP_REG_UIC1: c_uint = 0x0e28	/* ULPI Interface Control 1 */;
pub const CPCAP_REG_UIC2: c_uint = 0x0e2c	/* ULPI Interface Control 2 */;
pub const CPCAP_REG_UIC3: c_uint = 0x0e30	/* ULPI Interface Control 3 */;
pub const CPCAP_REG_USBOTG1: c_uint = 0x0e34	/* USB OTG Control 1 */;
pub const CPCAP_REG_USBOTG2: c_uint = 0x0e38	/* USB OTG Control 2 */;
pub const CPCAP_REG_USBOTG3: c_uint = 0x0e3c	/* USB OTG Control 3 */;
pub const CPCAP_REG_UIER1: c_uint = 0x0e40	/* USB Interrupt Enable Rising 1 */;
pub const CPCAP_REG_UIER2: c_uint = 0x0e44	/* USB Interrupt Enable Rising 2 */;
pub const CPCAP_REG_UIER3: c_uint = 0x0e48	/* USB Interrupt Enable Rising 3 */;
pub const CPCAP_REG_UIEF1: c_uint = 0x0e4c	/* USB Interrupt Enable Falling 1 */;
pub const CPCAP_REG_UIEF2: c_uint = 0x0e50	/* USB Interrupt Enable Falling 1 */;
pub const CPCAP_REG_UIEF3: c_uint = 0x0e54	/* USB Interrupt Enable Falling 1 */;
pub const CPCAP_REG_UIS: c_uint = 0x0e58	/* USB Interrupt Status */;
pub const CPCAP_REG_UIL: c_uint = 0x0e5c	/* USB Interrupt Latch */;
pub const CPCAP_REG_USBD: c_uint = 0x0e60	/* USB Debug */;
pub const CPCAP_REG_SCR1: c_uint = 0x0e64	/* Scratch 1 */;
pub const CPCAP_REG_SCR2: c_uint = 0x0e68	/* Scratch 2 */;
pub const CPCAP_REG_SCR3: c_uint = 0x0e6c	/* Scratch 3 */;
pub const CPCAP_REG_VMC: c_uint = 0x0eac	/* Video Mux Control */;
pub const CPCAP_REG_OWDC: c_uint = 0x0eb0	/* One Wire Device Control */;
pub const CPCAP_REG_GPIO0: c_uint = 0x0eb4	/* GPIO 0 Control */;
pub const CPCAP_REG_GPIO1: c_uint = 0x0ebc	/* GPIO 1 Control */;
pub const CPCAP_REG_GPIO2: c_uint = 0x0ec4	/* GPIO 2 Control */;
pub const CPCAP_REG_GPIO3: c_uint = 0x0ecc	/* GPIO 3 Control */;
pub const CPCAP_REG_GPIO4: c_uint = 0x0ed4	/* GPIO 4 Control */;
pub const CPCAP_REG_GPIO5: c_uint = 0x0edc	/* GPIO 5 Control */;
pub const CPCAP_REG_GPIO6: c_uint = 0x0ee4	/* GPIO 6 Control */;
pub const CPCAP_REG_MDLC: c_uint = 0x1000	/* Main Display Lighting Control */;
pub const CPCAP_REG_KLC: c_uint = 0x1004	/* Keypad Lighting Control */;
pub const CPCAP_REG_ADLC: c_uint = 0x1008	/* Aux Display Lighting Control */;
pub const CPCAP_REG_REDC: c_uint = 0x100c	/* Red Triode Control */;
pub const CPCAP_REG_GREENC: c_uint = 0x1010	/* Green Triode Control */;
pub const CPCAP_REG_BLUEC: c_uint = 0x1014	/* Blue Triode Control */;
pub const CPCAP_REG_CFC: c_uint = 0x1018	/* Camera Flash Control */;
pub const CPCAP_REG_ABC: c_uint = 0x101c	/* Adaptive Boost Control */;
pub const CPCAP_REG_BLEDC: c_uint = 0x1020	/* Bluetooth LED Control */;
pub const CPCAP_REG_CLEDC: c_uint = 0x1024	/* Camera Privacy LED Control */;
pub const CPCAP_REG_OW1C: c_uint = 0x1200	/* One Wire 1 Command */;
pub const CPCAP_REG_OW1D: c_uint = 0x1204	/* One Wire 1 Data */;
pub const CPCAP_REG_OW1I: c_uint = 0x1208	/* One Wire 1 Interrupt */;
pub const CPCAP_REG_OW1IE: c_uint = 0x120c	/* One Wire 1 Interrupt Enable */;
pub const CPCAP_REG_OW1: c_uint = 0x1214	/* One Wire 1 Control */;
pub const CPCAP_REG_OW2C: c_uint = 0x1220	/* One Wire 2 Command */;
pub const CPCAP_REG_OW2D: c_uint = 0x1224	/* One Wire 2 Data */;
pub const CPCAP_REG_OW2I: c_uint = 0x1228	/* One Wire 2 Interrupt */;
pub const CPCAP_REG_OW2IE: c_uint = 0x122c	/* One Wire 2 Interrupt Enable */;
pub const CPCAP_REG_OW2: c_uint = 0x1234	/* One Wire 2 Control */;
pub const CPCAP_REG_OW3C: c_uint = 0x1240	/* One Wire 3 Command */;
pub const CPCAP_REG_OW3D: c_uint = 0x1244	/* One Wire 3 Data */;
pub const CPCAP_REG_OW3I: c_uint = 0x1248	/* One Wire 3 Interrupt */;
pub const CPCAP_REG_OW3IE: c_uint = 0x124c	/* One Wire 3 Interrupt Enable */;
pub const CPCAP_REG_OW3: c_uint = 0x1254	/* One Wire 3 Control */;
pub const CPCAP_REG_GCAIC: c_uint = 0x1258	/* GCAI Clock Control */;
pub const CPCAP_REG_GCAIM: c_uint = 0x125c	/* GCAI GPIO Mode */;
pub const CPCAP_REG_LGDIR: c_uint = 0x1260	/* LMR GCAI GPIO Direction */;
pub const CPCAP_REG_LGPU: c_uint = 0x1264	/* LMR GCAI GPIO Pull-up */;
pub const CPCAP_REG_LGPIN: c_uint = 0x1268	/* LMR GCAI GPIO Pin */;
pub const CPCAP_REG_LGMASK: c_uint = 0x126c	/* LMR GCAI GPIO Mask */;
pub const CPCAP_REG_LDEB: c_uint = 0x1270	/* LMR Debounce Settings */;
pub const CPCAP_REG_LGDET: c_uint = 0x1274	/* LMR GCAI Detach Detect */;
pub const CPCAP_REG_LMISC: c_uint = 0x1278	/* LMR Misc Bits */;
pub const CPCAP_REG_LMACE: c_uint = 0x127c	/* LMR Mace IC Support */;
pub const CPCAP_REG_TEST: c_uint = 0x7c00	/* Test */;
pub const CPCAP_REG_ST_TEST1: c_uint = 0x7d08	/* ST Test1 */;
pub const CPCAP_REG_ST_TEST2: c_uint = 0x7d18	/* ST Test2 */;
//
// Helpers for child devices to check the revision and vendor.
//
// REVISIT: No documentation for the bits below, please update
// to use proper names for defines when available.
//
// revision = ((val >> 3) & 0x7) | ((val << 3) & 0x38);
// vendor = (val >> 6) & 0x7;
extern "C" {
    pub fn cpcap_sense_virq(regmap: *mut regmap, virq: c_int) -> c_int;
}
