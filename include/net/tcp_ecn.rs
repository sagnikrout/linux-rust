//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/tcp_ecn.h
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

// The highest ECN variant (Accurate ECN, ECN, or no ECN) that is
// attemped to be negotiated and requested for incoming connection
// and outgoing connection, respectively.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcp_ecn_mode {
    TCP_ECN_IN_NOECN_OUT_NOECN = 0,
    TCP_ECN_IN_ECN_OUT_ECN = 1,
    TCP_ECN_IN_ECN_OUT_NOECN = 2,
    TCP_ECN_IN_ACCECN_OUT_ACCECN = 3,
    TCP_ECN_IN_ACCECN_OUT_ECN = 4,
    TCP_ECN_IN_ACCECN_OUT_NOECN = 5,
}

// AccECN option sending when AccECN has been successfully negotiated
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcp_accecn_option {
    TCP_ACCECN_OPTION_DISABLED = 0,
    TCP_ACCECN_OPTION_MINIMUM = 1,
    TCP_ACCECN_OPTION_FULL = 2,
    TCP_ACCECN_OPTION_PERSIST = 3,
}

// Apply either ECT(0) or ECT(1) based on TCP_CONG_ECT_1_NEGOTIATION flag
// Do not set CWR if in AccECN mode!
// If the sender is telling us it has entered CWR, then its
// cwnd may be very low (even just 1 packet), so we should ACK
// immediately.
//
// Infer the ECT value our SYN arrived with from the echoed ACE field
// Below is an excerpt from the 1st block of Table 2 of AccECN spec
// Check ECN field transition to detect invalid transitions
// Non-ECT altered to something or something became non-ECT
// CE -> ECT(0/1)?
// Validate the 3rd ACK based on the ACE field, see Table 4 of AccECN spec
// Invalid value
// Unused but legal values
// Validation only applies to first non-data packet
// Demand the minimum # to send AccECN optnio
// Maps IP ECN field ECT/CE code point to AccECN option field number, given
// we are sending fields with Accurate ECN Order 1: ECT(1), CE, ECT(0).
//
// Maps IP ECN field ECT/CE code point to AccECN option field value offset.
// Some fields do not start from zero, to detect zeroing by middleboxes.
//
// Maps AccECN option field #nr to IP ECN field ECT/CE bits
// Based on Table 5 of the AccECN spec to map (option, order) to
// the corresponding ECN conuters (ECT-1, ECT-0, or CE).
//
// order = 0: 1st field ECT-0, 2nd field CE, 3rd field ECT-1
// order = 1: 1st field ECT-1, 2nd field CE, 3rd field ECT-0
// Handles AccECN option ECT and CE 24-bit byte counters update into
// the u32 value in tcp_sock. As we're processing TCP options, it is
// safe to access from - 1.
//
// If delta has the highest bit set (24th bit) indicating
// negative, sign extend to correct an estimation using
// sign_extend32(delta, 24 - 1)
//
// cnt += delta;
// Updates Accurate ECN received counters from the received IP ECN field
// As for accurate ECN, the TCP_ECN_SEEN flag is set by
// tcp_ecn_received_counters() when the ECN codepoint of
// received TCP data or ACK contains ECT(0), ECT(1), or CE.
//
// ACE counter tracks *all* segments including pure ACKs
// Send AccECN option at least once per 2^22-byte
// increase in any ECN byte counter.
//
// Demand Accurate ECN change-triggered ACKs. Two ACK are
// demanded to indicate unambiguously the ecnfield value
// in the latter ACK.
//
// AccECN specification, 2.2: [...] A Data Receiver maintains four counters
// initialized at the start of	the half-connection. [...] These byte counters
// reflect only the TCP payload length, excluding TCP header and TCP options.
//
// AccECN specification, 5.1: [...] a server can determine that it
// negotiated AccECN as [...] if the ACK contains an ACE field with
// the value 0b010 to 0b111 (decimal 2 to 7).
//
// Used to form the ACE flags for SYN/ACK
// TCP ACE flags of SYN/ACK are set based on IP-ECN received from SYN.
// Below is an excerpt from the 1st block of Table 2 of AccECN spec,
// in which TCP ACE flags are encoded as: (AE << 2) | (CWR << 1) | ECE
//
extern "C" {
    pub fn FIELD_PREP(_arg: TCPHDR_ACE, 0x3]: ecn_to_ace_flags[ect &) -> return;
}
// AccECN specification, 3.1.2: If a TCP server that implements AccECN
// receives a SYN with the three TCP header flags (AE, CWR and ECE) set
// to any combination other than 000, 011 or 111, it MUST negotiate the
// use of AccECN as if they had been set to 111.
//
// Used for make_synack to form the ACE flags
// TCP ACE flags of SYN/ACK are set based on IP-ECN codepoint received
// from SYN. Below is an excerpt from Table 2 of the AccECN spec:
// +====================+====================================+
// |  IP-ECN codepoint  |  Respective ACE falgs on SYN/ACK   |
// |   received on SYN  |       AE       CWR       ECE       |
// +====================+====================================+
// |      Not-ECT       |       0         1         0        |
// |      ECT(1)        |       0         1         1        |
// |      ECT(0)        |       1         0         0        |
// |        CE          |       1         1         0        |
// +====================+====================================+
//
// The final packet of the 3WHS or anything like it must reflect
// the SYN/ACK ECT instead of putting CEP into ACE field, such
// case show up in tcp_flags.
//
// Detect option zeroing: an AccECN connection "MAY check that the
// initial value of the EE0B field or the EE1B field is non-zero"
//
// Demand Accurate ECN option in response to the SYN on the SYN/ACK
// and the TCP server will try to send one more packet with an AccECN
// Option at a later point during the connection.
//
// See Table 2 of the AccECN draft
// +========+========+============+=============+
// | A      | B      |  SYN/ACK   |  Feedback   |
// |        |        |    B->A    |  Mode of A  |
// |        |        | AE CWR ECE |             |
// +========+========+============+=============+
// | AccECN | No ECN | 0   0   0  |   Not ECN   |
// | AccECN | Broken | 1   1   1  |   Not ECN   |
// +========+========+============+=============+
//
// +========+========+============+=============+
// | A      | B      |  SYN/ACK   |  Feedback   |
// |        |        |    B->A    |  Mode of A  |
// |        |        | AE CWR ECE |             |
// +========+========+============+=============+
// | AccECN | ECN    | 0   0   1  | Classic ECN |
// | Nonce  | AccECN | 0   0   1  | Classic ECN |
// | ECN    | AccECN | 0   0   1  | Classic ECN |
// +========+========+============+=============+
//
// Downgrade to classic ECN feedback
// Packet ECN state for a SYN-ACK
// Packet ECN state for a SYN.
// tp->ecn_flags are cleared at a later point in time when
// SYN ACK is ultimatively being received.
//
// Accurate ECN shall retransmit SYN/ACK with ACE=0 if the
// previously retransmitted SYN/ACK also times out.
//
