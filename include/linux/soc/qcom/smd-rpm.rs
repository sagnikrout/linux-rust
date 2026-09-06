//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/qcom/smd-rpm.h
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

pub const QCOM_SMD_RPM_ACTIVE_STATE: c_int = 0;
pub const QCOM_SMD_RPM_SLEEP_STATE: c_int = 1;
pub const QCOM_SMD_RPM_STATE_NUM: c_int = 2;
//
// Constants used for addressing resources in the RPM.
//
pub const QCOM_SMD_RPM_BBYB: c_uint = 0x62796262;
pub const QCOM_SMD_RPM_BOBB: c_uint = 0x62626f62;
pub const QCOM_SMD_RPM_BOOST: c_uint = 0x61747362;
pub const QCOM_SMD_RPM_BUS_CLK: c_uint = 0x316b6c63;
pub const QCOM_SMD_RPM_BUS_MASTER: c_uint = 0x73616d62;
pub const QCOM_SMD_RPM_BUS_SLAVE: c_uint = 0x766c7362;
pub const QCOM_SMD_RPM_CLK_BUF_A: c_uint = 0x616B6C63;
pub const QCOM_SMD_RPM_LDOA: c_uint = 0x616f646c;
pub const QCOM_SMD_RPM_LDOB: c_uint = 0x626F646C;
pub const QCOM_SMD_RPM_LDOE: c_uint = 0x656f646c;
pub const QCOM_SMD_RPM_RWCX: c_uint = 0x78637772;
pub const QCOM_SMD_RPM_RWMX: c_uint = 0x786d7772;
pub const QCOM_SMD_RPM_RWLC: c_uint = 0x636c7772;
pub const QCOM_SMD_RPM_RWLM: c_uint = 0x6d6c7772;
pub const QCOM_SMD_RPM_MEM_CLK: c_uint = 0x326b6c63;
pub const QCOM_SMD_RPM_MISC_CLK: c_uint = 0x306b6c63;
pub const QCOM_SMD_RPM_NCPA: c_uint = 0x6170636E;
pub const QCOM_SMD_RPM_NCPB: c_uint = 0x6270636E;
pub const QCOM_SMD_RPM_OCMEM_PWR: c_uint = 0x706d636f;
pub const QCOM_SMD_RPM_QPIC_CLK: c_uint = 0x63697071;
pub const QCOM_SMD_RPM_QUP_CLK: c_uint = 0x707571;
pub const QCOM_SMD_RPM_SMPA: c_uint = 0x61706d73;
pub const QCOM_SMD_RPM_SMPB: c_uint = 0x62706d73;
pub const QCOM_SMD_RPM_SMPE: c_uint = 0x65706d73;
pub const QCOM_SMD_RPM_SPDM: c_uint = 0x63707362;
pub const QCOM_SMD_RPM_VSA: c_uint = 0x00617376;
pub const QCOM_SMD_RPM_MMAXI_CLK: c_uint = 0x69786d6d;
pub const QCOM_SMD_RPM_IPA_CLK: c_uint = 0x617069;
pub const QCOM_SMD_RPM_CE_CLK: c_uint = 0x6563;
pub const QCOM_SMD_RPM_AGGR_CLK: c_uint = 0x72676761;
pub const QCOM_SMD_RPM_HWKM_CLK: c_uint = 0x6d6b7768;
pub const QCOM_SMD_RPM_PKA_CLK: c_uint = 0x616b70;
pub const QCOM_SMD_RPM_MCFG_CLK: c_uint = 0x6766636d;
pub const QCOM_RPM_KEY_SOFTWARE_ENABLE: c_uint = 0x6e657773;
pub const QCOM_RPM_KEY_PIN_CTRL_CLK_BUFFER_ENABLE_KEY: c_uint = 0x62636370;
pub const QCOM_RPM_SMD_KEY_RATE: c_uint = 0x007a484b;
pub const QCOM_RPM_SMD_KEY_ENABLE: c_uint = 0x62616e45;
pub const QCOM_RPM_SMD_KEY_STATE: c_uint = 0x54415453;
pub const QCOM_RPM_SCALING_ENABLE_ID: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_smd_rpm_req {
    pub key: __le32,
    pub nbytes: __le32,
    pub value: __le32,
}
