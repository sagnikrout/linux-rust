//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/esp_scsi.h
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
// esp_scsi.h: Defines and structures for the ESP driver.
//
// Copyright (C) 2007 David S. Miller (davem@davemloft.net)
//
// Access    Description      Offset
pub const ESP_TCLOW: c_uint = 0x00UL		/* rw  Low bits transfer count 0x00  */;
pub const ESP_TCMED: c_uint = 0x01UL		/* rw  Mid bits transfer count 0x04  */;
pub const ESP_FDATA: c_uint = 0x02UL		/* rw  FIFO data bits          0x08  */;
pub const ESP_CMD: c_uint = 0x03UL		/* rw  SCSI command bits       0x0c  */;
pub const ESP_STATUS: c_uint = 0x04UL		/* ro  ESP status register     0x10  */;

pub const ESP_INTRPT: c_uint = 0x05UL		/* ro  Kind of interrupt       0x14  */;

pub const ESP_SSTEP: c_uint = 0x06UL		/* ro  Sequence step register  0x18  */;

pub const ESP_FFLAGS: c_uint = 0x07UL		/* ro  Bits current FIFO info  0x1c  */;

pub const ESP_CFG1: c_uint = 0x08UL		/* rw  First cfg register      0x20  */;
pub const ESP_CFACT: c_uint = 0x09UL		/* wo  Clock conv factor       0x24  */;

pub const ESP_CTEST: c_uint = 0x0aUL		/* wo  Chip test register      0x28  */;
pub const ESP_CFG2: c_uint = 0x0bUL		/* rw  Second cfg register     0x2c  */;
pub const ESP_CFG3: c_uint = 0x0cUL		/* rw  Third cfg register      0x30  */;
pub const ESP_CFG4: c_uint = 0x0dUL		/* rw  Fourth cfg register     0x34  */;
pub const ESP_TCHI: c_uint = 0x0eUL		/* rw  High bits transf count  0x38  */;

pub const ESP_FGRND: c_uint = 0x0fUL		/* rw  Data base for fifo      0x3c  */;

pub const SBUS_ESP_REG_SIZE: c_uint = 0x40UL;
// Bitfield meanings for the above registers.
// ESP config reg 1, read-write, found on all ESP chips
pub const ESP_CONFIG1_ID: c_uint = 0x07      /* My BUS ID bits */;
pub const ESP_CONFIG1_CHTEST: c_uint = 0x08      /* Enable ESP chip tests */;
pub const ESP_CONFIG1_PENABLE: c_uint = 0x10      /* Enable parity checks */;
pub const ESP_CONFIG1_PARTEST: c_uint = 0x20      /* Parity test mode enabled? */;
pub const ESP_CONFIG1_SRRDISAB: c_uint = 0x40      /* Disable SCSI reset reports */;
pub const ESP_CONFIG1_SLCABLE: c_uint = 0x80      /* Enable slow cable mode */;
// ESP config reg 2, read-write, found only on esp100a+esp200+esp236 chips
pub const ESP_CONFIG2_DMAPARITY: c_uint = 0x01      /* enable DMA Parity (200,236) */;
pub const ESP_CONFIG2_REGPARITY: c_uint = 0x02      /* enable reg Parity (200,236) */;
pub const ESP_CONFIG2_BADPARITY: c_uint = 0x04      /* Bad parity target abort  */;
pub const ESP_CONFIG2_SCSI2ENAB: c_uint = 0x08      /* Enable SCSI-2 features (tgtmode) */;
pub const ESP_CONFIG2_HI: c_uint = 0x10      /* High Impedance DREQ ???  */;
pub const ESP_CONFIG2_HMEFENAB: c_uint = 0x10      /* HME features enable */;
pub const ESP_CONFIG2_BCM: c_uint = 0x20      /* Enable byte-ctrl (236)   */;
pub const ESP_CONFIG2_DISPINT: c_uint = 0x20      /* Disable pause irq (hme) */;
pub const ESP_CONFIG2_FENAB: c_uint = 0x40      /* Enable features (fas100,216) */;
pub const ESP_CONFIG2_SPL: c_uint = 0x40      /* Enable status-phase latch (236) */;
pub const ESP_CONFIG2_MKDONE: c_uint = 0x40      /* HME magic feature */;
pub const ESP_CONFIG2_HME32: c_uint = 0x80      /* HME 32 extended */;
pub const ESP_CONFIG2_MAGIC: c_uint = 0xe0      /* Invalid bits... */;
// ESP config register 3 read-write, found only esp236+fas236+fas100a+hme chips
pub const ESP_CONFIG3_FCLOCK: c_uint = 0x01     /* FAST SCSI clock rate (esp100a/hme) */;
pub const ESP_CONFIG3_TEM: c_uint = 0x01     /* Enable thresh-8 mode (esp/fas236)  */;
pub const ESP_CONFIG3_FAST: c_uint = 0x02     /* Enable FAST SCSI     (esp100a/hme) */;
pub const ESP_CONFIG3_ADMA: c_uint = 0x02     /* Enable alternate-dma (esp/fas236)  */;
pub const ESP_CONFIG3_TENB: c_uint = 0x04     /* group2 SCSI2 support (esp100a/hme) */;
pub const ESP_CONFIG3_SRB: c_uint = 0x04     /* Save residual byte   (esp/fas236)  */;
pub const ESP_CONFIG3_TMS: c_uint = 0x08     /* Three-byte msg's ok  (esp100a/hme) */;
pub const ESP_CONFIG3_FCLK: c_uint = 0x08     /* Fast SCSI clock rate (esp/fas236)  */;
pub const ESP_CONFIG3_IDMSG: c_uint = 0x10     /* ID message checking  (esp100a/hme) */;
pub const ESP_CONFIG3_FSCSI: c_uint = 0x10     /* Enable FAST SCSI     (esp/fas236)  */;
pub const ESP_CONFIG3_GTM: c_uint = 0x20     /* group2 SCSI2 support (esp/fas236)  */;
pub const ESP_CONFIG3_IDBIT3: c_uint = 0x20     /* Bit 3 of HME SCSI-ID (hme)         */;
pub const ESP_CONFIG3_TBMS: c_uint = 0x40     /* Three-byte msg's ok  (esp/fas236)  */;
pub const ESP_CONFIG3_EWIDE: c_uint = 0x40     /* Enable Wide-SCSI     (hme)         */;
pub const ESP_CONFIG3_IMS: c_uint = 0x80     /* ID msg chk'ng        (esp/fas236)  */;
pub const ESP_CONFIG3_OBPUSH: c_uint = 0x80     /* Push odd-byte to dma (hme)         */;
// ESP config register 4 read-write
pub const ESP_CONFIG4_BBTE: c_uint = 0x01     /* Back-to-back transfers     (fsc)   */;
pub const ESP_CONFIG4_TEST: c_uint = 0x02     /* Transfer counter test mode (fsc)   */;
pub const ESP_CONFIG4_RADE: c_uint = 0x04     /* Active negation   (am53c974/fsc)   */;
pub const ESP_CONFIG4_RAE: c_uint = 0x08     /* Act. negation REQ/ACK (am53c974)   */;
pub const ESP_CONFIG4_PWD: c_uint = 0x20     /* Reduced power feature (am53c974)   */;
pub const ESP_CONFIG4_GE0: c_uint = 0x40     /* Glitch eater bit 0    (am53c974)   */;
pub const ESP_CONFIG4_GE1: c_uint = 0x80     /* Glitch eater bit 1    (am53c974)   */;

// ESP command register read-write
// Group 1 commands:  These may be sent at any point in time to the ESP
// chip.  None of them can generate interrupts 'cept
// the "SCSI bus reset" command if you have not disabled
// SCSI reset interrupts in the config1 ESP register.
//
pub const ESP_CMD_NULL: c_uint = 0x00     /* Null command, ie. a nop */;
pub const ESP_CMD_FLUSH: c_uint = 0x01     /* FIFO Flush */;
pub const ESP_CMD_RC: c_uint = 0x02     /* Chip reset */;
pub const ESP_CMD_RS: c_uint = 0x03     /* SCSI bus reset */;
// Group 2 commands:  ESP must be an initiator and connected to a target
// for these commands to work.
//
pub const ESP_CMD_TI: c_uint = 0x10     /* Transfer Information */;
pub const ESP_CMD_ICCSEQ: c_uint = 0x11     /* Initiator cmd complete sequence */;
pub const ESP_CMD_MOK: c_uint = 0x12     /* Message okie-dokie */;
pub const ESP_CMD_TPAD: c_uint = 0x18     /* Transfer Pad */;
pub const ESP_CMD_SATN: c_uint = 0x1a     /* Set ATN */;
pub const ESP_CMD_RATN: c_uint = 0x1b     /* De-assert ATN */;
// Group 3 commands:  ESP must be in the MSGOUT or MSGIN state and be connected
// to a target as the initiator for these commands to work.
//
pub const ESP_CMD_SMSG: c_uint = 0x20     /* Send message */;
pub const ESP_CMD_SSTAT: c_uint = 0x21     /* Send status */;
pub const ESP_CMD_SDATA: c_uint = 0x22     /* Send data */;
pub const ESP_CMD_DSEQ: c_uint = 0x23     /* Discontinue Sequence */;
pub const ESP_CMD_TSEQ: c_uint = 0x24     /* Terminate Sequence */;
pub const ESP_CMD_TCCSEQ: c_uint = 0x25     /* Target cmd cmplt sequence */;
pub const ESP_CMD_DCNCT: c_uint = 0x27     /* Disconnect */;
pub const ESP_CMD_RMSG: c_uint = 0x28     /* Receive Message */;
pub const ESP_CMD_RCMD: c_uint = 0x29     /* Receive Command */;
pub const ESP_CMD_RDATA: c_uint = 0x2a     /* Receive Data */;
pub const ESP_CMD_RCSEQ: c_uint = 0x2b     /* Receive cmd sequence */;
// Group 4 commands:  The ESP must be in the disconnected state and must
// not be connected to any targets as initiator for
// these commands to work.
//
pub const ESP_CMD_RSEL: c_uint = 0x40     /* Reselect */;
pub const ESP_CMD_SEL: c_uint = 0x41     /* Select w/o ATN */;
pub const ESP_CMD_SELA: c_uint = 0x42     /* Select w/ATN */;
pub const ESP_CMD_SELAS: c_uint = 0x43     /* Select w/ATN & STOP */;
pub const ESP_CMD_ESEL: c_uint = 0x44     /* Enable selection */;
pub const ESP_CMD_DSEL: c_uint = 0x45     /* Disable selections */;
pub const ESP_CMD_SA3: c_uint = 0x46     /* Select w/ATN3 */;
pub const ESP_CMD_RSEL3: c_uint = 0x47     /* Reselect3 */;
// This bit enables the ESP's DMA on the SBus
pub const ESP_CMD_DMA: c_uint = 0x80     /* Do DMA? */;
// ESP status register read-only
pub const ESP_STAT_PIO: c_uint = 0x01     /* IO phase bit */;
pub const ESP_STAT_PCD: c_uint = 0x02     /* CD phase bit */;
pub const ESP_STAT_PMSG: c_uint = 0x04     /* MSG phase bit */;
pub const ESP_STAT_PMASK: c_uint = 0x07     /* Mask of phase bits */;
pub const ESP_STAT_TDONE: c_uint = 0x08     /* Transfer Completed */;
pub const ESP_STAT_TCNT: c_uint = 0x10     /* Transfer Counter Is Zero */;
pub const ESP_STAT_PERR: c_uint = 0x20     /* Parity error */;
pub const ESP_STAT_SPAM: c_uint = 0x40     /* Real bad error */;
// This indicates the 'interrupt pending' condition on esp236, it is a reserved
// bit on other revs of the ESP.
//
pub const ESP_STAT_INTR: c_uint = 0x80             /* Interrupt */;
// The status register can be masked with ESP_STAT_PMASK and compared
// with the following values to determine the current phase the ESP
// (at least thinks it) is in.  For our purposes we also add our own
// software 'done' bit for our phase management engine.
//

// HME only: status 2 register
pub const ESP_STAT2_SCHBIT: c_uint = 0x01 /* Upper bits 3-7 of sstep enabled */;
pub const ESP_STAT2_FFLAGS: c_uint = 0x02 /* The fifo flags are now latched */;
pub const ESP_STAT2_XCNT: c_uint = 0x04 /* The transfer counter is latched */;
pub const ESP_STAT2_CREGA: c_uint = 0x08 /* The command reg is active now */;
pub const ESP_STAT2_WIDE: c_uint = 0x10 /* Interface on this adapter is wide */;
pub const ESP_STAT2_F1BYTE: c_uint = 0x20 /* There is one byte at top of fifo */;
pub const ESP_STAT2_FMSB: c_uint = 0x40 /* Next byte in fifo is most significant */;
pub const ESP_STAT2_FEMPTY: c_uint = 0x80 /* FIFO is empty */;
// ESP interrupt register read-only
pub const ESP_INTR_S: c_uint = 0x01     /* Select w/o ATN */;
pub const ESP_INTR_SATN: c_uint = 0x02     /* Select w/ATN */;
pub const ESP_INTR_RSEL: c_uint = 0x04     /* Reselected */;
pub const ESP_INTR_FDONE: c_uint = 0x08     /* Function done */;
pub const ESP_INTR_BSERV: c_uint = 0x10     /* Bus service */;
pub const ESP_INTR_DC: c_uint = 0x20     /* Disconnect */;
pub const ESP_INTR_IC: c_uint = 0x40     /* Illegal command given */;
pub const ESP_INTR_SR: c_uint = 0x80     /* SCSI bus reset detected */;
// ESP sequence step register read-only
pub const ESP_STEP_VBITS: c_uint = 0x07     /* Valid bits */;
pub const ESP_STEP_ASEL: c_uint = 0x00     /* Selection&Arbitrate cmplt */;
pub const ESP_STEP_SID: c_uint = 0x01     /* One msg byte sent */;
pub const ESP_STEP_NCMD: c_uint = 0x02     /* Was not in command phase */;
pub const ESP_STEP_PPC: c_uint = 0x03     /* Early phase chg caused cmnd;
// bytes to be lost
//
pub const ESP_STEP_FINI4: c_uint = 0x04     /* Command was sent ok */;
// Ho hum, some ESP's set the step register to this as well...
pub const ESP_STEP_FINI5: c_uint = 0x05;
pub const ESP_STEP_FINI6: c_uint = 0x06;
pub const ESP_STEP_FINI7: c_uint = 0x07;
// ESP chip-test register read-write
pub const ESP_TEST_TARG: c_uint = 0x01     /* Target test mode */;
pub const ESP_TEST_INI: c_uint = 0x02     /* Initiator test mode */;
pub const ESP_TEST_TS: c_uint = 0x04     /* Tristate test mode */;
// ESP unique ID register read-only, found on fas236+fas100a only
pub const ESP_UID_FAM: c_uint = 0xf8     /* ESP family bitmask */;

// Values for the ESP family bits
pub const ESP_UID_F100A: c_uint = 0x00     /* ESP FAS100A  */;
pub const ESP_UID_F236: c_uint = 0x02     /* ESP FAS236   */;
pub const ESP_UID_HME: c_uint = 0x0a     /* FAS HME      */;
pub const ESP_UID_FSC: c_uint = 0x14     /* NCR/Symbios Logic 53CF9x-2 */;
// ESP fifo flags register read-only
// Note that the following implies a 16 byte FIFO on the ESP.
pub const ESP_FF_FBYTES: c_uint = 0x1f     /* Num bytes in FIFO */;
pub const ESP_FF_ONOTZERO: c_uint = 0x20     /* offset ctr not zero (esp100) */;
pub const ESP_FF_SSTEP: c_uint = 0xe0     /* Sequence step */;
// ESP clock conversion factor register write-only
pub const ESP_CCF_F0: c_uint = 0x00     /* 35.01MHz - 40MHz */;
pub const ESP_CCF_NEVER: c_uint = 0x01     /* Set it to this and die */;
pub const ESP_CCF_F2: c_uint = 0x02     /* 10MHz */;
pub const ESP_CCF_F3: c_uint = 0x03     /* 10.01MHz - 15MHz */;
pub const ESP_CCF_F4: c_uint = 0x04     /* 15.01MHz - 20MHz */;
pub const ESP_CCF_F5: c_uint = 0x05     /* 20.01MHz - 25MHz */;
pub const ESP_CCF_F6: c_uint = 0x06     /* 25.01MHz - 30MHz */;
pub const ESP_CCF_F7: c_uint = 0x07     /* 30.01MHz - 35MHz */;
// HME only...
pub const ESP_BUSID_RESELID: c_uint = 0x10;
pub const ESP_BUSID_CTR32BIT: c_uint = 0x40;

pub const ESP_TIMEO_CONST: c_int = 8192;

// For slow to medium speed input clock rates we shoot for 5mb/s, but for high
// input clock rates we try to do 10mb/s although I don't think a transfer can
// even run that fast with an ESP even with DMA2 scatter gather pipelining.
//
pub const SYNC_DEFP_SLOW: c_uint = 0x32   /* 5mb/s  */;
pub const SYNC_DEFP_FAST: c_uint = 0x19   /* 10mb/s */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esp_cmd_priv {
    pub num_sg: c_int,
    pub cur_residue: c_int,
    pub prv_sg: *mut scatterlist,
    pub cur_sg: *mut scatterlist,
    pub tot_residue: c_int,
}

// NOTE: this enum is ordered based on chip features!
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum esp_rev {
    ESP100,  /* NCR53C90 - very broken */
    ESP100A, /* NCR53C90A */
    ESP236,
    FAS236,
    PCSCSI,  /* AM53c974 */
    FSC,     /* NCR/Symbios Logic 53CF9x-2 */
    FAS100A,
    FAST,
    FASHME,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esp_cmd_entry {
    pub list: list_head,
    pub cmd: *mut scsi_cmnd,
    pub saved_cur_residue: c_uint,
    pub saved_prv_sg: *mut scatterlist,
    pub saved_cur_sg: *mut scatterlist,
    pub saved_tot_residue: c_uint,
    pub flags: u8,
pub const ESP_CMD_FLAG_WRITE: c_uint = 0x01 /* DMA is a write */;
pub const ESP_CMD_FLAG_AUTOSENSE: c_uint = 0x04 /* Doing automatic REQUEST_SENSE */;
pub const ESP_CMD_FLAG_RESIDUAL: c_uint = 0x08 /* AM53c974 BLAST residual */;
    pub tag: [u8; 2],
    pub orig_tag: [u8; 2],
    pub status: u8,
    pub message: u8,
    pub sense_ptr: *mut c_uchar,
    pub saved_sense_ptr: *mut c_uchar,
    pub sense_dma: dma_addr_t,
    pub eh_done: *mut completion,
}

pub const ESP_DEFAULT_TAGS: c_int = 16;
pub const ESP_MAX_TARGET: c_int = 16;
pub const ESP_MAX_LUN: c_int = 8;
pub const ESP_MAX_TAG: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esp_lun_data {
    pub non_tagged_cmd: *mut esp_cmd_entry,
    pub num_tagged: c_int,
    pub hold: c_int,
    pub tagged_cmds: [*mut esp_cmd_entry; ESP_MAX_TAG],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esp_target_data {
// These are the ESP_STP, ESP_SOFF, and ESP_CFG3 register values which
// match the currently negotiated settings for this target.  The SCSI
// protocol values are maintained in spi_{offset,period,wide}(starget).
//
    pub esp_period: u8,
    pub esp_offset: u8,
    pub esp_config3: u8,
    pub flags: u8,
pub const ESP_TGT_WIDE: c_uint = 0x01;
pub const ESP_TGT_DISCONNECT: c_uint = 0x02;
pub const ESP_TGT_NEGO_WIDE: c_uint = 0x04;
pub const ESP_TGT_NEGO_SYNC: c_uint = 0x08;
pub const ESP_TGT_CHECK_NEGO: c_uint = 0x40;
pub const ESP_TGT_BROKEN: c_uint = 0x80;
// When ESP_TGT_CHECK_NEGO is set, on the next scsi command to this
// device we will try to negotiate the following parameters.
//
    pub nego_goal_period: u8,
    pub nego_goal_offset: u8,
    pub nego_goal_width: u8,
    pub nego_goal_tags: u8,
    pub starget: *mut scsi_target,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esp_event_ent {
    pub type: u8,
pub const ESP_EVENT_TYPE_EVENT: c_uint = 0x01;
pub const ESP_EVENT_TYPE_CMD: c_uint = 0x02;
    pub val: u8,
    pub sreg: u8,
    pub seqreg: u8,
    pub sreg2: u8,
    pub ireg: u8,
    pub select_state: u8,
    pub event: u8,
    pub __pad: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esp_driver_ops {
// Read and write the ESP 8-bit registers.  On some
// applications of the ESP chip the registers are at 4-byte
// instead of 1-byte intervals.
//
    pub reg): *mut *mut *mut void (esp_write8)(struct esp esp, u8 val, unsigned long,
    pub reg): *mut *mut *mut u8 (esp_read8)(struct esp esp, unsigned long,
// Return non-zero if there is an IRQ pending.  Usually this
// status bit lives in the DMA controller sitting in front of
// the ESP.  This has to be accurate or else the ESP interrupt
// handler will not run.
//
    pub esp): *mut *mut int (irq_pending)(struct esp,
// Return the maximum allowable size of a DMA transfer for a
// given buffer.
//
    pub dma_len): u32,
// Reset the DMA engine entirely.  On return, ESP interrupts
// should be enabled.  Often the interrupt enabling is
// controlled in the DMA engine.
//
    pub esp): *mut *mut void (reset_dma)(struct esp,
// Drain any pending DMA in the DMA engine after a transfer.
// This is for writes to memory.
//
    pub esp): *mut *mut void (dma_drain)(struct esp,
// Invalidate the DMA engine after a DMA transfer.
    pub esp): *mut *mut void (dma_invalidate)(struct esp,
// Setup an ESP command that will use a DMA transfer.
// The 'esp_count' specifies what transfer length should be
// programmed into the ESP transfer counter registers, whereas
// the 'dma_count' is the length that should be programmed into
// the DMA controller.  Usually they are the same.  If 'write'
// is non-zero, this transfer is a write into memory.  'cmd'
// holds the ESP command that should be issued by calling
// scsi_esp_cmd() at the appropriate time while programming
// the DMA hardware.
//
    pub cmd): u32 dma_count, int write, u8,
// Return non-zero if the DMA engine is reporting an error
// currently.
//
    pub esp): *mut *mut int (dma_error)(struct esp,
}

pub const ESP_MAX_MSG_SZ: c_int = 8;
pub const ESP_EVENT_LOG_SZ: c_int = 32;
pub const ESP_QUICKIRQ_LIMIT: c_int = 100;
pub const ESP_RESELECT_TAG_LIMIT: c_int = 2500;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esp {
    pub regs: *mut void __iomem,
    pub dma_regs: *mut void __iomem,
    pub ops: *const esp_driver_ops,
    pub host: *mut Scsi_Host,
    pub dev: *mut device,
    pub active_cmd: *mut esp_cmd_entry,
    pub queued_cmds: list_head,
    pub active_cmds: list_head,
    pub command_block: *mut u8,
    pub command_block_dma: dma_addr_t,
    pub data_dma_len: c_uint,
// The following are used to determine the cause of an IRQ. Upon every
// IRQ entry we synchronize these with the hardware registers.
//
    pub sreg: u8,
    pub seqreg: u8,
    pub sreg2: u8,
    pub ireg: u8,
    pub prev_hme_dmacsr: u32,
    pub prev_soff: u8,
    pub prev_stp: u8,
    pub prev_cfg3: u8,
    pub num_tags: u8,
    pub esp_cmd_pool: list_head,
    pub target: [esp_target_data; ESP_MAX_TARGET],
    pub fifo_cnt: c_int,
    pub fifo: [u8; 16],
    pub esp_event_log: [esp_event_ent; ESP_EVENT_LOG_SZ],
    pub esp_event_cur: c_int,
    pub msg_out: [u8; ESP_MAX_MSG_SZ],
    pub msg_out_len: c_int,
    pub msg_in: [u8; ESP_MAX_MSG_SZ],
    pub msg_in_len: c_int,
    pub bursts: u8,
    pub config1: u8,
    pub config2: u8,
    pub config4: u8,
    pub scsi_id: u8,
    pub scsi_id_mask: u32,
    pub rev: esp_rev,
    pub flags: u32,
pub const ESP_FLAG_DIFFERENTIAL: c_uint = 0x00000001;
pub const ESP_FLAG_RESETTING: c_uint = 0x00000002;
pub const ESP_FLAG_WIDE_CAPABLE: c_uint = 0x00000008;
pub const ESP_FLAG_QUICKIRQ_CHECK: c_uint = 0x00000010;
pub const ESP_FLAG_DISABLE_SYNC: c_uint = 0x00000020;
pub const ESP_FLAG_USE_FIFO: c_uint = 0x00000040;
pub const ESP_FLAG_NO_DMA_MAP: c_uint = 0x00000080;
    pub select_state: u8,
pub const ESP_SELECT_NONE: c_uint = 0x00 /* Not selecting */;
pub const ESP_SELECT_BASIC: c_uint = 0x01 /* Select w/o MSGOUT phase */;
pub const ESP_SELECT_MSGOUT: c_uint = 0x02 /* Select with MSGOUT */;
// When we are not selecting, we are expecting an event.
    pub event: u8,
pub const ESP_EVENT_NONE: c_uint = 0x00;
pub const ESP_EVENT_CMD_START: c_uint = 0x01;
pub const ESP_EVENT_CMD_DONE: c_uint = 0x02;
pub const ESP_EVENT_DATA_IN: c_uint = 0x03;
pub const ESP_EVENT_DATA_OUT: c_uint = 0x04;
pub const ESP_EVENT_DATA_DONE: c_uint = 0x05;
pub const ESP_EVENT_MSGIN: c_uint = 0x06;
pub const ESP_EVENT_MSGIN_MORE: c_uint = 0x07;
pub const ESP_EVENT_MSGIN_DONE: c_uint = 0x08;
pub const ESP_EVENT_MSGOUT: c_uint = 0x09;
pub const ESP_EVENT_MSGOUT_DONE: c_uint = 0x0a;
pub const ESP_EVENT_STATUS: c_uint = 0x0b;
pub const ESP_EVENT_FREE_BUS: c_uint = 0x0c;
pub const ESP_EVENT_CHECK_PHASE: c_uint = 0x0d;
pub const ESP_EVENT_RESET: c_uint = 0x10;
// Probed in esp_get_clock_params()
    pub cfact: u32,
    pub cfreq: u32,
    pub ccycle: u32,
    pub ctick: u32,
    pub neg_defp: u32,
    pub sync_defp: u32,
// Computed in esp_reset_esp()
    pub max_period: u32,
    pub min_period: u32,
    pub radelay: u32,
// ESP_CMD_SELAS command state
    pub cmd_bytes_ptr: *mut u8,
    pub cmd_bytes_left: c_int,
    pub eh_reset: *mut completion,
    pub dma: *mut c_void,
    pub dmarev: c_int,
// These are used by esp_send_pio_cmd()
    pub fifo_reg: *mut u8 __iomem,
    pub send_cmd_error: c_int,
    pub send_cmd_residual: u32,
}

// A front-end driver for the ESP chip should do the following in
// it's device probe routine:
// 1) Allocate the host and private area using scsi_host_alloc()
// with size 'sizeof(struct esp)'.  The first argument to
// scsi_host_alloc() should be &scsi_esp_template.
// 2) Set host->max_id as appropriate.
// 3) Set esp->host to the scsi_host itself, and esp->dev
// to the device object pointer.
// 4) Hook up esp->ops to the front-end implementation.
// 5) If the ESP chip supports wide transfers, set ESP_FLAG_WIDE_CAPABLE
// in esp->flags.
// 6) Map the DMA and ESP chip registers.
// 7) DMA map the ESP command block, store the DMA address
// in esp->command_block_dma.
// 8) Register the scsi_esp_intr() interrupt handler.
// 9) Probe for and provide the following chip properties:
// esp->scsi_id (assign to esp->host->this_id too)
// esp->scsi_id_mask
// If ESP bus is differential, set ESP_FLAG_DIFFERENTIAL
// esp->cfreq
// DMA burst bit mask in esp->bursts, if necessary
// 10) Perform any actions necessary before the ESP device can
// be programmed for the first time.  On some configs, for
// example, the DMA engine has to be reset before ESP can
// be programmed.
// 11) If necessary, call dev_set_drvdata() as needed.
// 12) Call scsi_esp_register() with prepared 'esp' structure.
// 13) Check scsi_esp_register() return value, release all resources
// if an error was returned.
//
extern "C" {
    pub fn scsi_esp_register(: *mut esp) -> c_int;
}
extern "C" {
    pub fn scsi_esp_unregister(: *mut esp);
}
extern "C" {
    pub fn scsi_esp_intr(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn scsi_esp_cmd(: *mut esp, _arg: u8);
}
