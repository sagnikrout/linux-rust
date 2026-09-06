//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/scsi_cmnd.h
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
// MAX_COMMAND_SIZE is:
// The longest fixed-length SCSI CDB as per the SCSI standard.
// fixed-length means: commands that their size can be determined
// by their opcode and the CDB does not carry a length specifier, (unlike
// the VARIABLE_LENGTH_CMD(0x7f) command). This is actually not exactly
// true and the SCSI standard also defines extended commands and
// vendor specific commands that can be bigger than 16 bytes. The kernel
// will support these using the same infrastructure used for VARLEN CDB's.
// So in effect MAX_COMMAND_SIZE means the maximum size command scsi-ml
// supports without specifying a cmd_len by ULD's
//
pub const MAX_COMMAND_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_data_buffer {
    pub table: sg_table,
    pub length: unsigned,
}

// embedded in scsi_cmnd
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_pointer {
    pub /: *mut *mut *mut char ptr; / data pointer,
    pub /: *mut *mut int this_residual; / left in this buffer,
    pub /: *mut *mut *mut scatterlist buffer; / which buffer,
    pub /: *mut *mut int buffers_residual; / how many buffers left,
    pub dma_handle: dma_addr_t,
    pub Status: volatile int,
    pub Message: volatile int,
    pub have_data_in: volatile int,
    pub sent_command: volatile int,
    pub phase: volatile int,
}

// for scmd->flags

//
// libata uses SCSI EH to fetch sense data for successful commands.
// SCSI EH should not overwrite scmd->result when SCMD_FORCE_EH_SUCCESS is set.
//

// flags preserved across unprep / reprep

// for scmd->state
pub const SCMD_STATE_COMPLETE: c_int = 0;
pub const SCMD_STATE_INFLIGHT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_cmnd_submitter {
    SUBMITTED_BY_BLOCK_LAYER = 0,
    SUBMITTED_BY_SCSI_ERROR_HANDLER = 1,
    SUBMITTED_BY_SCSI_RESET_IOCTL = 2,
    } __packed;

    struct scsi_cmnd {
    struct scsi_device *device;
    struct list_head eh_entry; /* entry for the host eh_abort_list/eh_cmd_q */
    struct delayed_work abort_work;

    struct rcu_head rcu;

    int eh_eflags;		/* Used by error handlr */

    int budget_token;

//
// This is set to jiffies as it was when the command was first
// allocated.  It is used to time how long the command has
// been outstanding
//
    unsigned long jiffies_at_alloc;

    int retries;
    int allowed;

    unsigned char prot_op;
    unsigned char prot_type;
    unsigned char prot_flags;
    enum scsi_cmnd_submitter submitter;

    unsigned short cmd_len;
    enum dma_data_direction sc_data_direction;

    unsigned char cmnd[32]; /* SCSI CDB */

// These elements define the operation we ultimately want to perform
    struct scsi_data_buffer sdb;
    struct scsi_data_buffer *prot_sdb;

    unsigned underflow;	/* Return error if less than
    this amount is transferred */

    unsigned transfersize;	/* How much we are guaranteed to
    transfer with each SCSI transfer
    (ie, between disconnect /
    reconnects.   Probably == sector
    size */
    unsigned resid_len;	/* residual count */
    unsigned sense_len;
    unsigned char *sense_buffer;
// obtained by REQUEST SENSE when
// CHECK CONDITION is received on original
// command (auto-sense). Length must be
// SCSI_SENSE_BUFFERSIZE bytes.

    int flags;		/* Command flags */
    unsigned long state;	/* Command completion state */

    unsigned int extra_len;	/* length of alignment and padding */

//
// The fields below can be modified by the LLD but the fields above
// must not be modified.
//

    unsigned char *host_scribble;	/* The host adapter is allowed to
// call scsi_malloc and get some memory
// and hang it here.  The host adapter
// is also expected to call scsi_free
// to release this memory.  (The memory
// obtained by scsi_malloc is guaranteed
// to be at an address < 16Mb).

    int result;		/* Status code from lower level driver */
}

// Variant of blk_mq_rq_from_pdu() that verifies the type of its argument.
extern "C" {
    pub fn blk_mq_rq_from_pdu(_arg: scmd) -> return;
}
//
// Return the driver private allocation behind the command.
// Only works if cmd_size is set in the host template.
//
extern "C" {
    pub fn scsi_done(cmd: *mut scsi_cmnd);
}
extern "C" {
    pub fn scsi_done_direct(cmd: *mut scsi_cmnd);
}
extern "C" {
    pub fn scsi_finish_command(cmd: *mut scsi_cmnd);
}
extern "C" {
    pub fn scsi_kunmap_atomic_sg(virt: *mut c_void);
}
extern "C" {
    pub fn scsi_alloc_sgtables(cmd: *mut scsi_cmnd) -> blk_status_t;
}
extern "C" {
    pub fn scsi_free_sgtables(cmd: *mut scsi_cmnd);
}

extern "C" {
    pub fn scsi_dma_map(cmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn scsi_dma_unmap(cmd: *mut scsi_cmnd);
}

extern "C" {
    pub fn blk_rq_pos(_arg: scsi_cmd_to_rq(scmd)) -> return;
}
//
// The operations below are hints that tell the controller driver how
// to handle I/Os with DIF or similar types of protection information.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_prot_operations {
// Normal I/O
    SCSI_PROT_NORMAL = 0,

// OS-HBA: Protected, HBA-Target: Unprotected
    SCSI_PROT_READ_INSERT,
    SCSI_PROT_WRITE_STRIP,

// OS-HBA: Unprotected, HBA-Target: Protected
    SCSI_PROT_READ_STRIP,
    SCSI_PROT_WRITE_INSERT,

// OS-HBA: Protected, HBA-Target: Protected
    SCSI_PROT_READ_PASS,
    SCSI_PROT_WRITE_PASS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_prot_flags {
    SCSI_PROT_TRANSFER_PI		= 1 << 0,
    SCSI_PROT_GUARD_CHECK		= 1 << 1,
    SCSI_PROT_REF_CHECK		= 1 << 2,
    SCSI_PROT_REF_INCREMENT		= 1 << 3,
    SCSI_PROT_IP_CHECKSUM		= 1 << 4,
}

//
// The controller usually does not know anything about the target it
// is communicating with.  However, when DIX is enabled the controller
// must be know target type so it can verify the protection
// information passed along with the I/O.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_prot_target_type {
    SCSI_PROT_DIF_TYPE0 = 0,
    SCSI_PROT_DIF_TYPE1,
    SCSI_PROT_DIF_TYPE2,
    SCSI_PROT_DIF_TYPE3,
}

extern "C" {
    pub fn t10_pi_ref_tag(_arg: rq) -> return;
}

//
// scsi_msg_to_host_byte() - translate message byte
// @cmd: the SCSI command
// @msg: the SCSI parallel message byte to translate
//
// Translate the SCSI parallel message byte to a matching
// host byte setting. A message of COMMAND_COMPLETE indicates
// a successful command execution, any other message indicate
// an error. As the messages themselves only have a meaning
// for the SCSI parallel protocol this function translates
// them into a matching host byte value for SCSI EH.
//
