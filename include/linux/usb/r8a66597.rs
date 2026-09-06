//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/r8a66597.h
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
// R8A66597 driver platform data
//
// Copyright (C) 2009  Renesas Solutions Corp.
//
// Author : Yoshihiro Shimoda <yoshihiro.shimoda.uh@renesas.com>
//
pub const R8A66597_PLATDATA_XTAL_12MHZ: c_uint = 0x01;
pub const R8A66597_PLATDATA_XTAL_24MHZ: c_uint = 0x02;
pub const R8A66597_PLATDATA_XTAL_48MHZ: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r8a66597_platdata {
// This callback can control port power instead of DVSTCTR register.
    pub power): *mut *mut void (port_power)(int port, int,
// This parameter is for BUSWAIT
    pub buswait: u16,
// set one = on chip controller, set zero = external controller
    pub on_chip:1: unsigned,
// (external controller only) set R8A66597_PLATDATA_XTAL_nnMHZ
    pub xtal:2: unsigned,
// set one = 3.3V, set zero = 1.5V
    pub vif:1: unsigned,
// set one = big endian, set zero = little endian
    pub endian:1: unsigned,
// (external controller only) set one = WR0_N shorted to WR1_N
    pub wr0_shorted_to_wr1:1: unsigned,
// set one = using SUDMAC
    pub sudmac:1: unsigned,
}

// Register definitions
pub const SYSCFG0: c_uint = 0x00;
pub const SYSCFG1: c_uint = 0x02;
pub const SYSSTS0: c_uint = 0x04;
pub const SYSSTS1: c_uint = 0x06;
pub const DVSTCTR0: c_uint = 0x08;
pub const DVSTCTR1: c_uint = 0x0A;
pub const TESTMODE: c_uint = 0x0C;
pub const PINCFG: c_uint = 0x0E;
pub const DMA0CFG: c_uint = 0x10;
pub const DMA1CFG: c_uint = 0x12;
pub const CFIFO: c_uint = 0x14;
pub const D0FIFO: c_uint = 0x18;
pub const D1FIFO: c_uint = 0x1C;
pub const CFIFOSEL: c_uint = 0x20;
pub const CFIFOCTR: c_uint = 0x22;
pub const CFIFOSIE: c_uint = 0x24;
pub const D0FIFOSEL: c_uint = 0x28;
pub const D0FIFOCTR: c_uint = 0x2A;
pub const D1FIFOSEL: c_uint = 0x2C;
pub const D1FIFOCTR: c_uint = 0x2E;
pub const INTENB0: c_uint = 0x30;
pub const INTENB1: c_uint = 0x32;
pub const INTENB2: c_uint = 0x34;
pub const BRDYENB: c_uint = 0x36;
pub const NRDYENB: c_uint = 0x38;
pub const BEMPENB: c_uint = 0x3A;
pub const SOFCFG: c_uint = 0x3C;
pub const INTSTS0: c_uint = 0x40;
pub const INTSTS1: c_uint = 0x42;
pub const INTSTS2: c_uint = 0x44;
pub const BRDYSTS: c_uint = 0x46;
pub const NRDYSTS: c_uint = 0x48;
pub const BEMPSTS: c_uint = 0x4A;
pub const FRMNUM: c_uint = 0x4C;
pub const UFRMNUM: c_uint = 0x4E;
pub const USBADDR: c_uint = 0x50;
pub const USBREQ: c_uint = 0x54;
pub const USBVAL: c_uint = 0x56;
pub const USBINDX: c_uint = 0x58;
pub const USBLENG: c_uint = 0x5A;
pub const DCPCFG: c_uint = 0x5C;
pub const DCPMAXP: c_uint = 0x5E;
pub const DCPCTR: c_uint = 0x60;
pub const PIPESEL: c_uint = 0x64;
pub const PIPECFG: c_uint = 0x68;
pub const PIPEBUF: c_uint = 0x6A;
pub const PIPEMAXP: c_uint = 0x6C;
pub const PIPEPERI: c_uint = 0x6E;
pub const PIPE1CTR: c_uint = 0x70;
pub const PIPE2CTR: c_uint = 0x72;
pub const PIPE3CTR: c_uint = 0x74;
pub const PIPE4CTR: c_uint = 0x76;
pub const PIPE5CTR: c_uint = 0x78;
pub const PIPE6CTR: c_uint = 0x7A;
pub const PIPE7CTR: c_uint = 0x7C;
pub const PIPE8CTR: c_uint = 0x7E;
pub const PIPE9CTR: c_uint = 0x80;
pub const PIPE1TRE: c_uint = 0x90;
pub const PIPE1TRN: c_uint = 0x92;
pub const PIPE2TRE: c_uint = 0x94;
pub const PIPE2TRN: c_uint = 0x96;
pub const PIPE3TRE: c_uint = 0x98;
pub const PIPE3TRN: c_uint = 0x9A;
pub const PIPE4TRE: c_uint = 0x9C;
pub const PIPE4TRN: c_uint = 0x9E;
pub const PIPE5TRE: c_uint = 0xA0;
pub const PIPE5TRN: c_uint = 0xA2;
pub const DEVADD0: c_uint = 0xD0;
pub const DEVADD1: c_uint = 0xD2;
pub const DEVADD2: c_uint = 0xD4;
pub const DEVADD3: c_uint = 0xD6;
pub const DEVADD4: c_uint = 0xD8;
pub const DEVADD5: c_uint = 0xDA;
pub const DEVADD6: c_uint = 0xDC;
pub const DEVADD7: c_uint = 0xDE;
pub const DEVADD8: c_uint = 0xE0;
pub const DEVADD9: c_uint = 0xE2;
pub const DEVADDA: c_uint = 0xE4;
// System Configuration Control Register
pub const XTAL: c_uint = 0xC000	/* b15-14: Crystal selection */;
pub const XTAL48: c_uint = 0x8000	  /* 48MHz */;
pub const XTAL24: c_uint = 0x4000	  /* 24MHz */;
pub const XTAL12: c_uint = 0x0000	  /* 12MHz */;
pub const XCKE: c_uint = 0x2000	/* b13: External clock enable */;
pub const PLLC: c_uint = 0x0800	/* b11: PLL control */;
pub const SCKE: c_uint = 0x0400	/* b10: USB clock enable */;
pub const PCSDIS: c_uint = 0x0200	/* b9: not CS wakeup */;
pub const LPSME: c_uint = 0x0100	/* b8: Low power sleep mode */;
pub const HSE: c_uint = 0x0080	/* b7: Hi-speed enable */;
pub const DCFM: c_uint = 0x0040	/* b6: Controller function select  */;
pub const DRPD: c_uint = 0x0020	/* b5: D+/- pull down control */;
pub const DPRPU: c_uint = 0x0010	/* b4: D+ pull up control */;
pub const USBE: c_uint = 0x0001	/* b0: USB module operation enable */;
// System Configuration Status Register
pub const OVCBIT: c_uint = 0x8000	/* b15-14: Over-current bit */;
pub const OVCMON: c_uint = 0xC000	/* b15-14: Over-current monitor */;
pub const SOFEA: c_uint = 0x0020	/* b5: SOF monitor */;
pub const IDMON: c_uint = 0x0004	/* b3: ID-pin monitor */;
pub const LNST: c_uint = 0x0003	/* b1-0: D+, D- line status */;
pub const SE1: c_uint = 0x0003	  /* SE1 */;
pub const FS_KSTS: c_uint = 0x0002	  /* Full-Speed K State */;
pub const FS_JSTS: c_uint = 0x0001	  /* Full-Speed J State */;
pub const LS_JSTS: c_uint = 0x0002	  /* Low-Speed J State */;
pub const LS_KSTS: c_uint = 0x0001	  /* Low-Speed K State */;
pub const SE0: c_uint = 0x0000	  /* SE0 */;
// Device State Control Register
pub const EXTLP0: c_uint = 0x0400	/* b10: External port */;
pub const VBOUT: c_uint = 0x0200	/* b9: VBUS output */;
pub const WKUP: c_uint = 0x0100	/* b8: Remote wakeup */;
pub const RWUPE: c_uint = 0x0080	/* b7: Remote wakeup sense */;
pub const USBRST: c_uint = 0x0040	/* b6: USB reset enable */;
pub const RESUME: c_uint = 0x0020	/* b5: Resume enable */;
pub const UACT: c_uint = 0x0010	/* b4: USB bus enable */;
pub const RHST: c_uint = 0x0007	/* b1-0: Reset handshake status */;
pub const HSPROC: c_uint = 0x0004	  /* HS handshake is processing */;
pub const HSMODE: c_uint = 0x0003	  /* Hi-Speed mode */;
pub const FSMODE: c_uint = 0x0002	  /* Full-Speed mode */;
pub const LSMODE: c_uint = 0x0001	  /* Low-Speed mode */;
pub const UNDECID: c_uint = 0x0000	  /* Undecided */;
// Test Mode Register
pub const UTST: c_uint = 0x000F	/* b3-0: Test select */;
pub const H_TST_PACKET: c_uint = 0x000C	  /* HOST TEST Packet */;
pub const H_TST_SE0_NAK: c_uint = 0x000B	  /* HOST TEST SE0 NAK */;
pub const H_TST_K: c_uint = 0x000A	  /* HOST TEST K */;
pub const H_TST_J: c_uint = 0x0009	  /* HOST TEST J */;
pub const H_TST_NORMAL: c_uint = 0x0000	  /* HOST Normal Mode */;
pub const P_TST_PACKET: c_uint = 0x0004	  /* PERI TEST Packet */;
pub const P_TST_SE0_NAK: c_uint = 0x0003	  /* PERI TEST SE0 NAK */;
pub const P_TST_K: c_uint = 0x0002	  /* PERI TEST K */;
pub const P_TST_J: c_uint = 0x0001	  /* PERI TEST J */;
pub const P_TST_NORMAL: c_uint = 0x0000	  /* PERI Normal Mode */;
// Data Pin Configuration Register
pub const LDRV: c_uint = 0x8000	/* b15: Drive Current Adjust */;
pub const VIF1: c_uint = 0x0000		/* VIF = 1.8V */;
pub const VIF3: c_uint = 0x8000		/* VIF = 3.3V */;
pub const INTA: c_uint = 0x0001	/* b1: USB INT-pin active */;
// DMAx Pin Configuration Register
pub const DREQA: c_uint = 0x4000	/* b14: Dreq active select */;
pub const BURST: c_uint = 0x2000	/* b13: Burst mode */;
pub const DACKA: c_uint = 0x0400	/* b10: Dack active select */;
pub const DFORM: c_uint = 0x0380	/* b9-7: DMA mode select */;
pub const CPU_ADR_RD_WR: c_uint = 0x0000	  /* Address + RD/WR mode (CPU bus) */;
pub const CPU_DACK_RD_WR: c_uint = 0x0100	  /* DACK + RD/WR mode (CPU bus) */;
pub const CPU_DACK_ONLY: c_uint = 0x0180	  /* DACK only mode (CPU bus) */;
pub const SPLIT_DACK_ONLY: c_uint = 0x0200	  /* DACK only mode (SPLIT bus) */;
pub const DENDA: c_uint = 0x0040	/* b6: Dend active select */;
pub const PKTM: c_uint = 0x0020	/* b5: Packet mode */;
pub const DENDE: c_uint = 0x0010	/* b4: Dend enable */;
pub const OBUS: c_uint = 0x0004	/* b2: OUTbus mode */;
// CFIFO/DxFIFO Port Select Register
pub const RCNT: c_uint = 0x8000	/* b15: Read count mode */;
pub const REW: c_uint = 0x4000	/* b14: Buffer rewind */;
pub const DCLRM: c_uint = 0x2000	/* b13: DMA buffer clear mode */;
pub const DREQE: c_uint = 0x1000	/* b12: DREQ output enable */;
pub const MBW_8: c_uint = 0x0000	  /*  8bit */;
pub const MBW_16: c_uint = 0x0400	  /* 16bit */;
pub const MBW_32: c_uint = 0x0800   /* 32bit */;
pub const BIGEND: c_uint = 0x0100	/* b8: Big endian mode */;
pub const BYTE_LITTLE: c_uint = 0x0000		/* little dendian */;
pub const BYTE_BIG: c_uint = 0x0100		/* big endifan */;
pub const ISEL: c_uint = 0x0020	/* b5: DCP FIFO port direction select */;
pub const CURPIPE: c_uint = 0x000F	/* b2-0: PIPE select */;
// CFIFO/DxFIFO Port Control Register
pub const BVAL: c_uint = 0x8000	/* b15: Buffer valid flag */;
pub const BCLR: c_uint = 0x4000	/* b14: Buffer clear */;
pub const FRDY: c_uint = 0x2000	/* b13: FIFO ready */;
pub const DTLN: c_uint = 0x0FFF	/* b11-0: FIFO received data length */;
// Interrupt Enable Register 0
pub const VBSE: c_uint = 0x8000	/* b15: VBUS interrupt */;
pub const RSME: c_uint = 0x4000	/* b14: Resume interrupt */;
pub const SOFE: c_uint = 0x2000	/* b13: Frame update interrupt */;
pub const DVSE: c_uint = 0x1000	/* b12: Device state transition interrupt */;
pub const CTRE: c_uint = 0x0800	/* b11: Control transfer stage transition interrupt */;
pub const BEMPE: c_uint = 0x0400	/* b10: Buffer empty interrupt */;
pub const NRDYE: c_uint = 0x0200	/* b9: Buffer not ready interrupt */;
pub const BRDYE: c_uint = 0x0100	/* b8: Buffer ready interrupt */;
// Interrupt Enable Register 1
pub const OVRCRE: c_uint = 0x8000	/* b15: Over-current interrupt */;
pub const BCHGE: c_uint = 0x4000	/* b14: USB us chenge interrupt */;
pub const DTCHE: c_uint = 0x1000	/* b12: Detach sense interrupt */;
pub const ATTCHE: c_uint = 0x0800	/* b11: Attach sense interrupt */;
pub const EOFERRE: c_uint = 0x0040	/* b6: EOF error interrupt */;
pub const SIGNE: c_uint = 0x0020	/* b5: SETUP IGNORE interrupt */;
pub const SACKE: c_uint = 0x0010	/* b4: SETUP ACK interrupt */;
// BRDY Interrupt Enable/Status Register
pub const BRDY9: c_uint = 0x0200	/* b9: PIPE9 */;
pub const BRDY8: c_uint = 0x0100	/* b8: PIPE8 */;
pub const BRDY7: c_uint = 0x0080	/* b7: PIPE7 */;
pub const BRDY6: c_uint = 0x0040	/* b6: PIPE6 */;
pub const BRDY5: c_uint = 0x0020	/* b5: PIPE5 */;
pub const BRDY4: c_uint = 0x0010	/* b4: PIPE4 */;
pub const BRDY3: c_uint = 0x0008	/* b3: PIPE3 */;
pub const BRDY2: c_uint = 0x0004	/* b2: PIPE2 */;
pub const BRDY1: c_uint = 0x0002	/* b1: PIPE1 */;
pub const BRDY0: c_uint = 0x0001	/* b1: PIPE0 */;
// NRDY Interrupt Enable/Status Register
pub const NRDY9: c_uint = 0x0200	/* b9: PIPE9 */;
pub const NRDY8: c_uint = 0x0100	/* b8: PIPE8 */;
pub const NRDY7: c_uint = 0x0080	/* b7: PIPE7 */;
pub const NRDY6: c_uint = 0x0040	/* b6: PIPE6 */;
pub const NRDY5: c_uint = 0x0020	/* b5: PIPE5 */;
pub const NRDY4: c_uint = 0x0010	/* b4: PIPE4 */;
pub const NRDY3: c_uint = 0x0008	/* b3: PIPE3 */;
pub const NRDY2: c_uint = 0x0004	/* b2: PIPE2 */;
pub const NRDY1: c_uint = 0x0002	/* b1: PIPE1 */;
pub const NRDY0: c_uint = 0x0001	/* b1: PIPE0 */;
// BEMP Interrupt Enable/Status Register
pub const BEMP9: c_uint = 0x0200	/* b9: PIPE9 */;
pub const BEMP8: c_uint = 0x0100	/* b8: PIPE8 */;
pub const BEMP7: c_uint = 0x0080	/* b7: PIPE7 */;
pub const BEMP6: c_uint = 0x0040	/* b6: PIPE6 */;
pub const BEMP5: c_uint = 0x0020	/* b5: PIPE5 */;
pub const BEMP4: c_uint = 0x0010	/* b4: PIPE4 */;
pub const BEMP3: c_uint = 0x0008	/* b3: PIPE3 */;
pub const BEMP2: c_uint = 0x0004	/* b2: PIPE2 */;
pub const BEMP1: c_uint = 0x0002	/* b1: PIPE1 */;
pub const BEMP0: c_uint = 0x0001	/* b0: PIPE0 */;
// SOF Pin Configuration Register
pub const TRNENSEL: c_uint = 0x0100	/* b8: Select transaction enable period */;
pub const BRDYM: c_uint = 0x0040	/* b6: BRDY clear timing */;
pub const INTL: c_uint = 0x0020	/* b5: Interrupt sense select */;
pub const EDGESTS: c_uint = 0x0010	/* b4:  */;
pub const SOFMODE: c_uint = 0x000C	/* b3-2: SOF pin select */;
pub const SOF_125US: c_uint = 0x0008	  /* SOF OUT 125us Frame Signal */;
pub const SOF_1MS: c_uint = 0x0004	  /* SOF OUT 1ms Frame Signal */;
pub const SOF_DISABLE: c_uint = 0x0000	  /* SOF OUT Disable */;
// Interrupt Status Register 0
pub const VBINT: c_uint = 0x8000	/* b15: VBUS interrupt */;
pub const RESM: c_uint = 0x4000	/* b14: Resume interrupt */;
pub const SOFR: c_uint = 0x2000	/* b13: SOF frame update interrupt */;
pub const DVST: c_uint = 0x1000	/* b12: Device state transition interrupt */;
pub const CTRT: c_uint = 0x0800	/* b11: Control transfer stage transition interrupt */;
pub const BEMP: c_uint = 0x0400	/* b10: Buffer empty interrupt */;
pub const NRDY: c_uint = 0x0200	/* b9: Buffer not ready interrupt */;
pub const BRDY: c_uint = 0x0100	/* b8: Buffer ready interrupt */;
pub const VBSTS: c_uint = 0x0080	/* b7: VBUS input port */;
pub const DVSQ: c_uint = 0x0070	/* b6-4: Device state */;
pub const DS_SPD_CNFG: c_uint = 0x0070	  /* Suspend Configured */;
pub const DS_SPD_ADDR: c_uint = 0x0060	  /* Suspend Address */;
pub const DS_SPD_DFLT: c_uint = 0x0050	  /* Suspend Default */;
pub const DS_SPD_POWR: c_uint = 0x0040	  /* Suspend Powered */;
pub const DS_SUSP: c_uint = 0x0040	  /* Suspend */;
pub const DS_CNFG: c_uint = 0x0030	  /* Configured */;
pub const DS_ADDS: c_uint = 0x0020	  /* Address */;
pub const DS_DFLT: c_uint = 0x0010	  /* Default */;
pub const DS_POWR: c_uint = 0x0000	  /* Powered */;
pub const DVSQS: c_uint = 0x0030	/* b5-4: Device state */;
pub const VALID: c_uint = 0x0008	/* b3: Setup packet detected flag */;
pub const CTSQ: c_uint = 0x0007	/* b2-0: Control transfer stage */;
pub const CS_SQER: c_uint = 0x0006	  /* Sequence error */;
pub const CS_WRND: c_uint = 0x0005	  /* Control write nodata status stage */;
pub const CS_WRSS: c_uint = 0x0004	  /* Control write status stage */;
pub const CS_WRDS: c_uint = 0x0003	  /* Control write data stage */;
pub const CS_RDSS: c_uint = 0x0002	  /* Control read status stage */;
pub const CS_RDDS: c_uint = 0x0001	  /* Control read data stage */;
pub const CS_IDST: c_uint = 0x0000	  /* Idle or setup stage */;
// Interrupt Status Register 1
pub const OVRCR: c_uint = 0x8000	/* b15: Over-current interrupt */;
pub const BCHG: c_uint = 0x4000	/* b14: USB bus chenge interrupt */;
pub const DTCH: c_uint = 0x1000	/* b12: Detach sense interrupt */;
pub const ATTCH: c_uint = 0x0800	/* b11: Attach sense interrupt */;
pub const EOFERR: c_uint = 0x0040	/* b6: EOF-error interrupt */;
pub const SIGN: c_uint = 0x0020	/* b5: Setup ignore interrupt */;
pub const SACK: c_uint = 0x0010	/* b4: Setup acknowledge interrupt */;
// Frame Number Register
pub const OVRN: c_uint = 0x8000	/* b15: Overrun error */;
pub const CRCE: c_uint = 0x4000	/* b14: Received data error */;
pub const FRNM: c_uint = 0x07FF	/* b10-0: Frame number */;
// Micro Frame Number Register
pub const UFRNM: c_uint = 0x0007	/* b2-0: Micro frame number */;
// Default Control Pipe Maxpacket Size Register
// Pipe Maxpacket Size Register
pub const DEVSEL: c_uint = 0xF000	/* b15-14: Device address select */;
pub const MAXP: c_uint = 0x007F	/* b6-0: Maxpacket size of default control pipe */;
// Default Control Pipe Control Register
pub const BSTS: c_uint = 0x8000	/* b15: Buffer status */;
pub const SUREQ: c_uint = 0x4000	/* b14: Send USB request  */;
pub const CSCLR: c_uint = 0x2000	/* b13: complete-split status clear */;
pub const CSSTS: c_uint = 0x1000	/* b12: complete-split status */;
pub const SUREQCLR: c_uint = 0x0800	/* b11: stop setup request */;
pub const SQCLR: c_uint = 0x0100	/* b8: Sequence toggle bit clear */;
pub const SQSET: c_uint = 0x0080	/* b7: Sequence toggle bit set */;
pub const SQMON: c_uint = 0x0040	/* b6: Sequence toggle bit monitor */;
pub const PBUSY: c_uint = 0x0020	/* b5: pipe busy */;
pub const PINGE: c_uint = 0x0010	/* b4: ping enable */;
pub const CCPL: c_uint = 0x0004	/* b2: Enable control transfer complete */;
pub const PID: c_uint = 0x0003	/* b1-0: Response PID */;
pub const PID_STALL11: c_uint = 0x0003	  /* STALL */;
pub const PID_STALL: c_uint = 0x0002	  /* STALL */;
pub const PID_BUF: c_uint = 0x0001	  /* BUF */;
pub const PID_NAK: c_uint = 0x0000	  /* NAK */;
// Pipe Window Select Register
pub const PIPENM: c_uint = 0x0007	/* b2-0: Pipe select */;
// Pipe Configuration Register
pub const R8A66597_TYP: c_uint = 0xC000	/* b15-14: Transfer type */;
pub const R8A66597_ISO: c_uint = 0xC000		  /* Isochronous */;
pub const R8A66597_INT: c_uint = 0x8000		  /* Interrupt */;
pub const R8A66597_BULK: c_uint = 0x4000		  /* Bulk */;
pub const R8A66597_BFRE: c_uint = 0x0400	/* b10: Buffer ready interrupt mode select */;
pub const R8A66597_DBLB: c_uint = 0x0200	/* b9: Double buffer mode select */;
pub const R8A66597_CNTMD: c_uint = 0x0100	/* b8: Continuous transfer mode select */;
pub const R8A66597_SHTNAK: c_uint = 0x0080	/* b7: Transfer end NAK */;
pub const R8A66597_DIR: c_uint = 0x0010	/* b4: Transfer direction select */;
pub const R8A66597_EPNUM: c_uint = 0x000F	/* b3-0: Eendpoint number select */;
// Pipe Buffer Configuration Register
pub const BUFSIZE: c_uint = 0x7C00	/* b14-10: Pipe buffer size */;
pub const BUFNMB: c_uint = 0x007F	/* b6-0: Pipe buffer number */;
pub const PIPE0BUF: c_int = 256;
pub const PIPExBUF: c_int = 64;
// Pipe Maxpacket Size Register
pub const MXPS: c_uint = 0x07FF	/* b10-0: Maxpacket size */;
// Pipe Cycle Configuration Register
pub const IFIS: c_uint = 0x1000	/* b12: Isochronous in-buffer flush mode select */;
pub const IITV: c_uint = 0x0007	/* b2-0: Isochronous interval */;
// Pipex Control Register
pub const BSTS: c_uint = 0x8000	/* b15: Buffer status */;
pub const INBUFM: c_uint = 0x4000	/* b14: IN buffer monitor (Only for PIPE1 to 5) */;
pub const CSCLR: c_uint = 0x2000	/* b13: complete-split status clear */;
pub const CSSTS: c_uint = 0x1000	/* b12: complete-split status */;
pub const ATREPM: c_uint = 0x0400	/* b10: Auto repeat mode */;
pub const ACLRM: c_uint = 0x0200	/* b9: Out buffer auto clear mode */;
pub const SQCLR: c_uint = 0x0100	/* b8: Sequence toggle bit clear */;
pub const SQSET: c_uint = 0x0080	/* b7: Sequence toggle bit set */;
pub const SQMON: c_uint = 0x0040	/* b6: Sequence toggle bit monitor */;
pub const PBUSY: c_uint = 0x0020	/* b5: pipe busy */;
pub const PID: c_uint = 0x0003	/* b1-0: Response PID */;
// PIPExTRE
pub const TRENB: c_uint = 0x0200	/* b9: Transaction counter enable */;
pub const TRCLR: c_uint = 0x0100	/* b8: Transaction counter clear */;
// PIPExTRN
pub const TRNCNT: c_uint = 0xFFFF	/* b15-0: Transaction counter */;
// DEVADDx
pub const UPPHUB: c_uint = 0x7800;
pub const HUBPORT: c_uint = 0x0700;
pub const USBSPD: c_uint = 0x00C0;
pub const RTPORT: c_uint = 0x0001;
// SUDMAC registers
pub const CH0CFG: c_uint = 0x00;
pub const CH1CFG: c_uint = 0x04;
pub const CH0BA: c_uint = 0x10;
pub const CH1BA: c_uint = 0x14;
pub const CH0BBC: c_uint = 0x18;
pub const CH1BBC: c_uint = 0x1C;
pub const CH0CA: c_uint = 0x20;
pub const CH1CA: c_uint = 0x24;
pub const CH0CBC: c_uint = 0x28;
pub const CH1CBC: c_uint = 0x2C;
pub const CH0DEN: c_uint = 0x30;
pub const CH1DEN: c_uint = 0x34;
pub const DSTSCLR: c_uint = 0x38;
pub const DBUFCTRL: c_uint = 0x3C;
pub const DINTCTRL: c_uint = 0x40;
pub const DINTSTS: c_uint = 0x44;
pub const DINTSTSCLR: c_uint = 0x48;
pub const CH0SHCTRL: c_uint = 0x50;
pub const CH1SHCTRL: c_uint = 0x54;
// SUDMAC Configuration Registers
pub const SENDBUFM: c_uint = 0x1000 /* b12: Transmit Buffer Mode */;
pub const RCVENDM: c_uint = 0x0100 /* b8: Receive Data Transfer End Mode */;
pub const LBA_WAIT: c_uint = 0x0030 /* b5-4: Local Bus Access Wait */;
// DMA Enable Registers
pub const DEN: c_uint = 0x0001 /* b1: DMA Transfer Enable */;
// DMA Status Clear Register
pub const CH1STCLR: c_uint = 0x0002 /* b2: Ch1 DMA Status Clear */;
pub const CH0STCLR: c_uint = 0x0001 /* b1: Ch0 DMA Status Clear */;
// DMA Buffer Control Register
pub const CH1BUFW: c_uint = 0x0200 /* b9: Ch1 DMA Buffer Data Transfer Enable */;
pub const CH0BUFW: c_uint = 0x0100 /* b8: Ch0 DMA Buffer Data Transfer Enable */;
pub const CH1BUFS: c_uint = 0x0002 /* b2: Ch1 DMA Buffer Data Status */;
pub const CH0BUFS: c_uint = 0x0001 /* b1: Ch0 DMA Buffer Data Status */;
// DMA Interrupt Control Register
pub const CH1ERRE: c_uint = 0x0200 /* b9: Ch1 SHwy Res Err Detect Int Enable */;
pub const CH0ERRE: c_uint = 0x0100 /* b8: Ch0 SHwy Res Err Detect Int Enable */;
pub const CH1ENDE: c_uint = 0x0002 /* b2: Ch1 DMA Transfer End Int Enable */;
pub const CH0ENDE: c_uint = 0x0001 /* b1: Ch0 DMA Transfer End Int Enable */;
// DMA Interrupt Status Register
pub const CH1ERRS: c_uint = 0x0200 /* b9: Ch1 SHwy Res Err Detect Int Status */;
pub const CH0ERRS: c_uint = 0x0100 /* b8: Ch0 SHwy Res Err Detect Int Status */;
pub const CH1ENDS: c_uint = 0x0002 /* b2: Ch1 DMA Transfer End Int Status */;
pub const CH0ENDS: c_uint = 0x0001 /* b1: Ch0 DMA Transfer End Int Status */;
// DMA Interrupt Status Clear Register
pub const CH1ERRC: c_uint = 0x0200 /* b9: Ch1 SHwy Res Err Detect Int Stat Clear */;
pub const CH0ERRC: c_uint = 0x0100 /* b8: Ch0 SHwy Res Err Detect Int Stat Clear */;
pub const CH1ENDC: c_uint = 0x0002 /* b2: Ch1 DMA Transfer End Int Stat Clear */;
pub const CH0ENDC: c_uint = 0x0001 /* b1: Ch0 DMA Transfer End Int Stat Clear */;
