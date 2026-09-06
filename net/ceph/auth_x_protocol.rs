//! Automatically rewritten from C Header to Rust Module
//! Source: net/ceph/auth_x_protocol.h
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
pub const CEPHX_GET_AUTH_SESSION_KEY: c_uint = 0x0100;
pub const CEPHX_GET_PRINCIPAL_SESSION_KEY: c_uint = 0x0200;
pub const CEPHX_GET_ROTATING_KEY: c_uint = 0x0400;
// Client <-> AuthMonitor
//
// The AUTH session's connection secret: encrypted with the AUTH
// ticket session key
//
pub const CEPHX_KEY_USAGE_AUTH_CONNECTION_SECRET: c_uint = 0x03;
//
// The ticket's blob for the client ("blob for me", contains the
// session key): encrypted with the client's secret key in case of
// the AUTH ticket and the AUTH ticket session key in case of other
// service tickets
//
pub const CEPHX_KEY_USAGE_TICKET_SESSION_KEY: c_uint = 0x04;
//
// The ticket's blob for the service (ceph_x_ticket_blob): possibly
// encrypted with the old AUTH ticket session key in case of the AUTH
// ticket and not encrypted in case of other service tickets
//
pub const CEPHX_KEY_USAGE_TICKET_BLOB: c_uint = 0x05;
// Client <-> Service
//
// The client's authorization request (ceph_x_authorize_b):
// encrypted with the service ticket session key
//
pub const CEPHX_KEY_USAGE_AUTHORIZE: c_uint = 0x10;
//
// The service's challenge (ceph_x_authorize_challenge):
// encrypted with the service ticket session key
//
pub const CEPHX_KEY_USAGE_AUTHORIZE_CHALLENGE: c_uint = 0x11;
//
// The service's final reply (ceph_x_authorize_reply + the service
// session's connection secret): encrypted with the service ticket
// session key
//
pub const CEPHX_KEY_USAGE_AUTHORIZE_REPLY: c_uint = 0x12;
// common bits
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_x_ticket_blob {
    pub struct_v: __u8,
    pub secret_id: __le64,
    pub blob_len: __le32,
    pub blob: [c_char; ],
// C attribute field omitted
// common request/reply headers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_x_request_header {
    pub op: __le16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_x_reply_header {
    pub op: __le16,
    pub result: __le32,
// C attribute field omitted
// authenticate handshake
// initial hello (no reply header)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_x_server_challenge {
    pub struct_v: __u8,
    pub server_challenge: __le64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_x_authenticate {
    pub struct_v: __u8,
    pub client_challenge: __le64,
    pub key: __le64,
// old_ticket blob
// nautilus+: other_keys
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_x_service_ticket_request {
    pub struct_v: __u8,
    pub keys: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_x_challenge_blob {
    pub server_challenge: __le64,
    pub client_challenge: __le64,
// C attribute field omitted
// authorize handshake
//
// The authorizer consists of two pieces:
// a - service id, ticket blob
// b - encrypted with session key
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_x_authorize_a {
    pub struct_v: __u8,
    pub global_id: __le64,
    pub service_id: __le32,
    pub ticket_blob: ceph_x_ticket_blob,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_x_authorize_b {
    pub struct_v: __u8,
    pub nonce: __le64,
    pub have_challenge: __u8,
    pub server_challenge_plus_one: __le64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_x_authorize_challenge {
    pub struct_v: __u8,
    pub server_challenge: __le64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_x_authorize_reply {
    pub struct_v: __u8,
    pub nonce_plus_one: __le64,
// C attribute field omitted
//
// encryption bundle
//
pub const CEPHX_ENC_MAGIC: c_uint = 0xff009cad8826aa55ull;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_x_encrypt_header {
    pub struct_v: __u8,
    pub magic: __le64,
// C attribute field omitted
