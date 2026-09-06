//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/tun_proto.h
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


// One byte protocol values as defined by VXLAN-GPE and NSH. These will
// hopefully get a shared IANA registry.
//
pub const TUN_P_IPV4: c_uint = 0x01;
pub const TUN_P_IPV6: c_uint = 0x02;
pub const TUN_P_ETHERNET: c_uint = 0x03;
pub const TUN_P_NSH: c_uint = 0x04;
pub const TUN_P_MPLS_UC: c_uint = 0x05;
extern "C" {
    pub fn htons(_arg: ETH_P_IP) -> return;
}
extern "C" {
    pub fn htons(_arg: ETH_P_IPV6) -> return;
}
extern "C" {
    pub fn htons(_arg: ETH_P_TEB) -> return;
}
extern "C" {
    pub fn htons(_arg: ETH_P_NSH) -> return;
}
extern "C" {
    pub fn htons(_arg: ETH_P_MPLS_UC) -> return;
}
