//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/st.h
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

// Descriptor for analyzed sense data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_cmdstatus {
    pub midlevel_result: c_int,
    pub sense_hdr: scsi_sense_hdr,
    pub have_sense: c_int,
    pub residual: c_int,
    pub uremainder64: u64,
    pub flags: u8,
    pub remainder_valid: u8,
    pub fixed_format: u8,
    pub deferred: u8,
}

// scsi tape command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_request {
    pub cmd: [c_uchar; MAX_COMMAND_SIZE],
    pub sense: [c_uchar; SCSI_SENSE_BUFFERSIZE],
    pub result: c_int,
    pub stp: *mut scsi_tape,
    pub waiting: *mut completion,
    pub bio: *mut bio,
}

// The tape buffer descriptor.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_buffer {
    pub /: *mut *mut unsigned char cleared; / internal buffer cleared after open?,
    pub /: *mut *mut unsigned short do_dio; / direct i/o set up?,
    pub buffer_size: c_int,
    pub buffer_blocks: c_int,
    pub buffer_bytes: c_int,
    pub read_pointer: c_int,
    pub writing: c_int,
    pub syscall_result: c_int,
    pub last_SRpnt: *mut st_request,
    pub cmdstat: st_cmdstatus,
    pub reserved_pages: *mut page,
    pub reserved_page_order: c_int,
    pub mapped_pages: *mut page,
    pub map_data: rq_map_data,
    pub b_data: *mut c_uchar,
    pub /: *mut *mut unsigned short use_sg; / zero or max number of s/g segments for this adapter,
    pub /: *mut *mut unsigned short sg_segs; / number of segments in s/g list,
    pub /: *mut *mut unsigned short frp_segs; / number of buffer segments,
}

// The tape mode definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_modedef {
    pub defined: c_uchar,
    pub /: *mut *mut unsigned char sysv; / SYS V semantics?,
    pub do_async_writes: c_uchar,
    pub do_buffer_writes: c_uchar,
    pub do_read_ahead: c_uchar,
    pub defaults_for_writes: c_uchar,
    pub /: *mut *mut unsigned char default_compression; / 0 = don't touch, etc,
    pub /: *mut *mut short default_density; / Forced density, -1 = no value,
    pub /: *mut *mut int default_blksize; / Forced blocksize, -1 = no value,
    pub tape: *mut scsi_tape,
    pub /: *mut *mut *mut device devs[2]; / Auto-rewind and non-rewind devices,
    pub /: *mut *mut *mut cdev cdevs[2]; / Auto-rewind and non-rewind devices,
}

// Number of modes can be changed by changing ST_NBR_MODE_BITS. The maximum
pub const ST_NBR_MODE_BITS: c_int = 2;

// The status related to each partition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_partstat {
    pub rw: c_uchar,
    pub eof: c_uchar,
    pub at_sm: c_uchar,
    pub last_block_valid: c_uchar,
    pub last_block_visited: u32,
    pub /: *mut *mut int drv_block; / The block where the drive head is,
    pub drv_file: c_int,
}

// Tape statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_tape_stats {
    pub /: *mut *mut atomic64_t read_byte_cnt; / bytes read,
    pub /: *mut *mut atomic64_t write_byte_cnt; / bytes written,
    pub /: *mut *mut atomic64_t in_flight; / Number of I/Os in flight,
    pub /: *mut *mut atomic64_t read_cnt; / Count of read requests,
    pub /: *mut *mut atomic64_t write_cnt; / Count of write requests,
    pub either: *mut *mut atomic64_t other_cnt; / Count of other requests,
// implicit or from user space
// ioctl.
    pub /: *mut *mut atomic64_t resid_cnt; / Count of resid_len > 0,
    pub /: *mut *mut atomic64_t tot_read_time; / ktime spent completing reads,
    pub /: *mut *mut atomic64_t tot_write_time; / ktime spent completing writes,
    pub /: *mut *mut atomic64_t tot_io_time; / ktime spent doing any I/O,
    pub /: *mut *mut ktime_t read_time; / holds ktime request was queued,
    pub /: *mut *mut ktime_t write_time; / holds ktime request was queued,
    pub /: *mut *mut ktime_t other_time; / holds ktime request was queued,
    pub /: *mut *mut atomic_t last_read_size; / Number of bytes issued for last read,
    pub /: *mut *mut atomic_t last_write_size; / Number of bytes issued for last write,
}

pub const ST_NBR_PARTITIONS: c_int = 4;
// The tape drive descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_tape {
    pub device: *mut scsi_device,
    pub /: *mut *mut mutex lock; / For serialization,
    pub /: *mut *mut completion wait; / For SCSI commands,
    pub buffer: *mut st_buffer,
    pub index: c_int,
// Drive characteristics
    pub omit_blklims: c_uchar,
    pub do_auto_lock: c_uchar,
    pub can_bsr: c_uchar,
    pub can_partitions: c_uchar,
    pub two_fm: c_uchar,
    pub fast_mteom: c_uchar,
    pub immediate: c_uchar,
    pub scsi2_logical: c_uchar,
    pub /: *mut *mut unsigned char default_drvbuffer; / 0xff = don't touch, value 3 bits,
    pub /: *mut *mut unsigned char cln_mode; / 0 = none, otherwise sense byte nbr,
    pub cln_sense_value: c_uchar,
    pub cln_sense_mask: c_uchar,
    pub /: *mut *mut unsigned char use_pf; / Set Page Format bit in all mode selects?,
    pub /: *mut *mut unsigned char try_dio; / try direct i/o in general?,
    pub /: *mut *mut unsigned char try_dio_now; / try direct i/o before next close?,
    pub /: *mut *mut unsigned char c_algo; / compression algorithm,
    pub /: *mut *mut unsigned char pos_unknown; / after reset position unknown,
    pub /: *mut *mut unsigned char sili; / use SILI when reading in variable b mode,
    pub /: *mut *mut unsigned char immediate_filemark; / write filemark immediately,
    pub tape_type: c_int,
    pub /: *mut *mut int long_timeout; / timeout for commands known to take long time,
// Mode characteristics
    pub modes: [st_modedef; ST_NBR_MODES],
    pub current_mode: c_int,
// Status variables
    pub partition: c_int,
    pub new_partition: c_int,
    pub /: *mut *mut int nbr_partitions; / zero until partition support enabled,
    pub ps: [st_partstat; ST_NBR_PARTITIONS],
    pub dirty: c_uchar,
    pub ready: c_uchar,
    pub write_prot: c_uchar,
    pub drv_write_prot: c_uchar,
    pub in_use: c_uchar,
    pub blksize_changed: c_uchar,
    pub density_changed: c_uchar,
    pub compression_changed: c_uchar,
    pub drv_buffer: c_uchar,
    pub density: c_uchar,
    pub changed_density: c_uchar,
    pub door_locked: c_uchar,
    pub /: *mut *mut unsigned char autorew_dev; / auto-rewind device,
    pub /: *mut *mut unsigned char rew_at_close; / rewind necessary at close,
    pub inited: c_uchar,
    pub /: *mut *mut unsigned char cleaning_req; / cleaning requested?,
    pub /: *mut *mut unsigned char first_tur; / first TEST UNIT READY,
    pub block_size: c_int,
    pub changed_blksize: c_int,
    pub min_block: c_int,
    pub max_block: c_int,
    pub /: *mut *mut int recover_count; / From tape opening,
    pub /: *mut *mut int recover_reg; / From last status call,
// The saved values of midlevel counters
    pub new_media_ctr: c_uint,
    pub por_ctr: c_uint,

    pub write_pending: c_uchar,
    pub nbr_finished: c_int,
    pub nbr_waits: c_int,
    pub nbr_requests: c_int,
    pub nbr_dio: c_int,
    pub nbr_pages: c_int,
    pub last_cmnd: [c_uchar; 6],
    pub last_sense: [c_uchar; 16],    pub name: [c_char; DISK_NAME_LEN],
    pub kref: kref,
    pub stats: *mut scsi_tape_stats,
}

// Bit masks for use_pf
pub const USE_PF: c_int = 1;
pub const PF_TESTED: c_int = 2;
// Values of eof
pub const ST_NOEOF: c_int = 0;
pub const ST_FM_HIT: c_int = 1;
pub const ST_FM: c_int = 2;
pub const ST_EOM_OK: c_int = 3;
pub const ST_EOM_ERROR: c_int = 4;
pub const ST_EOD_1: c_int = 5;
pub const ST_EOD_2: c_int = 6;
pub const ST_EOD: c_int = 7;
// EOD hit while reading => ST_EOD_1 => return zero => ST_EOD_2 =>
// When writing: ST_EOM_OK == early warning found, write OK
// Values of rw
pub const ST_IDLE: c_int = 0;
pub const ST_READING: c_int = 1;
pub const ST_WRITING: c_int = 2;
// Values of ready state
pub const ST_READY: c_int = 0;
pub const ST_NOT_READY: c_int = 1;
pub const ST_NO_TAPE: c_int = 2;
// Values for door lock state
pub const ST_UNLOCKED: c_int = 0;
pub const ST_LOCKED_EXPLICIT: c_int = 1;
pub const ST_LOCKED_AUTO: c_int = 2;
pub const ST_LOCK_FAILS: c_int = 3;
// Positioning SCSI-commands for Tandberg, etc. drives
pub const QFA_REQUEST_BLOCK: c_uint = 0x02;
pub const QFA_SEEK_BLOCK: c_uint = 0x0c;
// Setting the binary options
pub const ST_DONT_TOUCH: c_int = 0;
pub const ST_NO: c_int = 1;
pub const ST_YES: c_int = 2;
pub const EXTENDED_SENSE_START: c_int = 18;
// Masks for some conditions in the sense data
pub const SENSE_FMK: c_uint = 0x80;
pub const SENSE_EOM: c_uint = 0x40;
pub const SENSE_ILI: c_uint = 0x20;
