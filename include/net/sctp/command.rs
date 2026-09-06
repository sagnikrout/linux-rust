//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/sctp/command.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// SCTP kernel Implementation
// (C) Copyright IBM Corp. 2001, 2004
// Copyright (C) 1999-2001 Cisco, Motorola
//
// This file is part of the SCTP kernel implementation
//
// These are the definitions needed for the command object.
//
// Please send any bug reports or fixes you make to the
// email address(es):
// lksctp developers <linux-sctp@vger.kernel.org>
//
// Written or modified by:
// La Monte H.P. Yarroll <piggy@acm.org>
// Karl Knutson <karl@athena.chicago.il.us>
// Ardelle Fan <ardelle.fan@intel.com>
// Sridhar Samudrala <sri@us.ibm.com>
//

// Macro flag: #define __net_sctp_command_h__

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_verb {
    SCTP_CMD_NOP = 0,	/* Do nothing. */
    SCTP_CMD_NEW_ASOC,	/* Register a new association.  */
    SCTP_CMD_DELETE_TCB,	/* Delete the current association. */
    SCTP_CMD_NEW_STATE,	/* Enter a new state.  */
    SCTP_CMD_REPORT_TSN,	/* Record the arrival of a TSN.  */
    SCTP_CMD_GEN_SACK,	/* Send a Selective ACK (maybe).  */
    SCTP_CMD_PROCESS_SACK,	/* Process an inbound SACK.  */
    SCTP_CMD_GEN_INIT_ACK,	/* Generate an INIT ACK chunk.  */
    SCTP_CMD_PEER_INIT,	/* Process a INIT from the peer.  */
    SCTP_CMD_GEN_COOKIE_ECHO, /* Generate a COOKIE ECHO chunk. */
    SCTP_CMD_CHUNK_ULP,	/* Send a chunk to the sockets layer.  */
    SCTP_CMD_EVENT_ULP,	/* Send a notification to the sockets layer. */
    SCTP_CMD_REPLY,		/* Send a chunk to our peer.  */
    SCTP_CMD_SEND_PKT,	/* Send a full packet to our peer.  */
    SCTP_CMD_RETRAN,	/* Mark a transport for retransmission.  */
    SCTP_CMD_ECN_CE,        /* Do delayed CE processing.   */
    SCTP_CMD_ECN_ECNE,	/* Do delayed ECNE processing. */
    SCTP_CMD_ECN_CWR,	/* Do delayed CWR processing.  */
    SCTP_CMD_TIMER_START,	/* Start a timer.  */
    SCTP_CMD_TIMER_START_ONCE, /* Start a timer once */
    SCTP_CMD_TIMER_RESTART,	/* Restart a timer. */
    SCTP_CMD_TIMER_STOP,	/* Stop a timer. */
    SCTP_CMD_INIT_CHOOSE_TRANSPORT, /* Choose transport for an INIT. */
    SCTP_CMD_INIT_COUNTER_RESET, /* Reset init counter. */
    SCTP_CMD_INIT_COUNTER_INC,   /* Increment init counter. */
    SCTP_CMD_INIT_RESTART,  /* High level, do init timer work. */
    SCTP_CMD_COOKIEECHO_RESTART,  /* High level, do cookie-echo timer work. */
    SCTP_CMD_INIT_FAILED,   /* High level, do init failure work. */
    SCTP_CMD_REPORT_DUP,	/* Report a duplicate TSN.  */
    SCTP_CMD_STRIKE,	/* Mark a strike against a transport.  */
    SCTP_CMD_HB_TIMERS_START,    /* Start the heartbeat timers. */
    SCTP_CMD_HB_TIMER_UPDATE,    /* Update a heartbeat timers.  */
    SCTP_CMD_HB_TIMERS_STOP,     /* Stop the heartbeat timers.  */
    SCTP_CMD_PROBE_TIMER_UPDATE, /* Update a probe timer.  */
    SCTP_CMD_TRANSPORT_HB_SENT,  /* Reset the status of a transport. */
    SCTP_CMD_TRANSPORT_IDLE,     /* Do manipulations on idle transport */
    SCTP_CMD_TRANSPORT_ON,       /* Mark the transport as active. */
    SCTP_CMD_REPORT_ERROR,   /* Pass this error back out of the sm. */
    SCTP_CMD_REPORT_BAD_TAG, /* Verification tags didn't match. */
    SCTP_CMD_PROCESS_CTSN,   /* Sideeffect from shutdown. */
    SCTP_CMD_ASSOC_FAILED,	 /* Handle association failure. */
    SCTP_CMD_DISCARD_PACKET, /* Discard the whole packet. */
    SCTP_CMD_GEN_SHUTDOWN,   /* Generate a SHUTDOWN chunk. */
    SCTP_CMD_PURGE_OUTQUEUE, /* Purge all data waiting to be sent. */
    SCTP_CMD_SETUP_T2,       /* Hi-level, setup T2-shutdown parms.  */
    SCTP_CMD_RTO_PENDING,	 /* Set transport's rto_pending. */
    SCTP_CMD_PART_DELIVER,	 /* Partial data delivery considerations. */
    SCTP_CMD_RENEGE,         /* Renege data on an association. */
    SCTP_CMD_SETUP_T4,	 /* ADDIP, setup T4 RTO timer parms. */
    SCTP_CMD_PROCESS_OPERR,  /* Process an ERROR chunk. */
    SCTP_CMD_REPORT_FWDTSN,	 /* Report new cumulative TSN Ack. */
    SCTP_CMD_PROCESS_FWDTSN, /* Skips were reported, so process further. */
    SCTP_CMD_CLEAR_INIT_TAG, /* Clears association peer's inittag. */
    SCTP_CMD_DEL_NON_PRIMARY, /* Removes non-primary peer transports. */
    SCTP_CMD_T3_RTX_TIMERS_STOP, /* Stops T3-rtx pending timers */
    SCTP_CMD_FORCE_PRIM_RETRAN,  /* Forces retrans. over primary path. */
    SCTP_CMD_SET_SK_ERR,	 /* Set sk_err */
    SCTP_CMD_ASSOC_CHANGE,	 /* generate and send assoc_change event */
    SCTP_CMD_ADAPTATION_IND, /* generate and send adaptation event */
    SCTP_CMD_PEER_NO_AUTH,   /* generate and send authentication event */
    SCTP_CMD_ASSOC_SHKEY,    /* generate the association shared keys */
    SCTP_CMD_T1_RETRAN,	 /* Mark for retransmission after T1 timeout  */
    SCTP_CMD_UPDATE_INITTAG, /* Update peer inittag */
    SCTP_CMD_SEND_MSG,	 /* Send the whole use message */
    SCTP_CMD_PURGE_ASCONF_QUEUE, /* Purge all asconf queues.*/
    SCTP_CMD_SET_ASOC,	 /* Restore association context */
    SCTP_CMD_LAST
}

// How many commands can you put in an struct sctp_cmd_seq?
// This is a rather arbitrary number, ideally derived from a careful
// analysis of the state functions, but in reality just taken from
// thin air in the hopes othat we don't trigger a kernel panic.
//
pub const SCTP_MAX_NUM_COMMANDS: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub union sctp_arg {
    pub /: *mut *mut *mut void zero_all; / Set to NULL to clear the entire union,
    pub i32: __s32,
    pub u32: __u32,
    pub be32: __be32,
    pub u16: __u16,
    pub u8: __u8,
    pub error: c_int,
    pub err: __be16,
    pub state: sctp_state,
    pub to: sctp_event_timeout,
    pub chunk: *mut sctp_chunk,
    pub asoc: *mut sctp_association,
    pub transport: *mut sctp_transport,
    pub bp: *mut sctp_bind_addr,
    pub init: *mut sctp_init_chunk,
    pub ulpevent: *mut sctp_ulpevent,
    pub packet: *mut sctp_packet,
    pub sackh: *mut sctp_sackhdr,
    pub msg: *mut sctp_datamsg,
}

// We are simulating ML type constructors here.
//
// SCTP_ARG_CONSTRUCTOR(NAME, TYPE, ELT) builds a function called
// SCTP_NAME() which takes an argument of type TYPE and returns an
// union sctp_arg.  It does this by inserting the sole argument into
// the ELT union element of a local union sctp_arg.
//
// E.g., SCTP_ARG_CONSTRUCTOR(I32, __s32, i32) builds SCTP_I32(arg),
// which takes an __s32 and returns a union sctp_arg containing the
// __s32.  So, after foo = SCTP_I32(arg), foo.i32 == arg.
//

extern "C" {
    pub fn SCTP_I32(_arg: 1) -> return;
}
extern "C" {
    pub fn SCTP_I32(_arg: 0) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_cmd {
    pub obj: sctp_arg,
    pub verb: sctp_verb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_cmd_seq {
    pub cmds: [sctp_cmd; SCTP_MAX_NUM_COMMANDS],
    pub last_used_slot: *mut sctp_cmd,
    pub next_cmd: *mut sctp_cmd,
}

// Initialize a block of memory as a command sequence.
// Return 0 if the initialization fails.
//
// cmds[] is filled backwards to simplify the overflow BUG() check
// Add a command to an struct sctp_cmd_seq.
//
// Use the SCTP_* constructors defined by SCTP_ARG_CONSTRUCTOR() above
// to wrap data which goes in the obj argument.
//
// Return the next command structure in an sctp_cmd_seq.
// Return NULL at the end of the sequence.
//
