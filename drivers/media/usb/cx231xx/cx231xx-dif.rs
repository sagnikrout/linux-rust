//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/cx231xx/cx231xx-dif.h
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
//
// cx231xx-dif.h - driver for Conexant Cx23100/101/102 USB video capture devices
//
// Copyright {C} 2009 <Bill.Liu@conexant.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dif_settings {
    pub if_freq: u32,
    pub register_address: u32,
    pub value: u32,
}

// case 3000000:
// BEGIN - DIF BPF register values from 30_quant.dat
// END - DIF BPF register values from 30_quant.dat
// case 3100000:
// BEGIN - DIF BPF register values from 31_quant.dat
// END - DIF BPF register values from 31_quant.dat
// case 3200000:
// BEGIN - DIF BPF register values from 32_quant.dat
// END - DIF BPF register values from 32_quant.dat
// case 3300000:
// BEGIN - DIF BPF register values from 33_quant.dat
// END - DIF BPF register values from 33_quant.dat
// case 3400000:
// BEGIN - DIF BPF register values from 34_quant.dat
// END - DIF BPF register values from 34_quant.dat
// case 3500000:
// BEGIN - DIF BPF register values from 35_quant.dat
// END - DIF BPF register values from 35_quant.dat
// case 3600000:
// BEGIN - DIF BPF register values from 36_quant.dat
// END - DIF BPF register values from 36_quant.dat
// case 3700000:
// BEGIN - DIF BPF register values from 37_quant.dat
// END - DIF BPF register values from 37_quant.dat
// case 3800000:
// BEGIN - DIF BPF register values from 38_quant.dat
// END - DIF BPF register values from 38_quant.dat
// case 3900000:
// BEGIN - DIF BPF register values from 39_quant.dat
// END - DIF BPF register values from 39_quant.dat
// case 4000000:
// BEGIN - DIF BPF register values from 40_quant.dat
// END - DIF BPF register values from 40_quant.dat
// case 4100000:
// BEGIN - DIF BPF register values from 41_quant.dat
// END - DIF BPF register values from 41_quant.dat
// case 4200000:
// BEGIN - DIF BPF register values from 42_quant.dat
// END - DIF BPF register values from 42_quant.dat
// case 4300000:
// BEGIN - DIF BPF register values from 43_quant.dat
// END - DIF BPF register values from 43_quant.dat
// case 4400000:
// BEGIN - DIF BPF register values from 44_quant.dat
// END - DIF BPF register values from 44_quant.dat
// case 4500000:
// BEGIN - DIF BPF register values from 45_quant.dat
// END - DIF BPF register values from 45_quant.dat
// case 4600000:
// BEGIN - DIF BPF register values from 46_quant.dat
// END - DIF BPF register values from 46_quant.dat
// case 4700000:
// BEGIN - DIF BPF register values from 47_quant.dat
// END - DIF BPF register values from 47_quant.dat
// case 4800000:
// BEGIN - DIF BPF register values from 48_quant.dat
// END - DIF BPF register values from 48_quant.dat
// case 4900000:
// BEGIN - DIF BPF register values from 49_quant.dat
// END - DIF BPF register values from 49_quant.dat
// case 5000000:
// BEGIN - DIF BPF register values from 50_quant.dat
// END - DIF BPF register values from 50_quant.dat
// case 5100000:
// BEGIN - DIF BPF register values from 51_quant.dat
// END - DIF BPF register values from 51_quant.dat
// case 5200000:
// BEGIN - DIF BPF register values from 52_quant.dat
// END - DIF BPF register values from 52_quant.dat
// case 5300000:
// BEGIN - DIF BPF register values from 53_quant.dat
// END - DIF BPF register values from 53_quant.dat
// case 5400000:
// BEGIN - DIF BPF register values from 54_quant.dat
// END - DIF BPF register values from 54_quant.dat
// case 5500000:
// BEGIN - DIF BPF register values from 55_quant.dat
// END - DIF BPF register values from 55_quant.dat
// case 5600000:
// BEGIN - DIF BPF register values from 56_quant.dat
// END - DIF BPF register values from 56_quant.dat
// case 5700000:
// BEGIN - DIF BPF register values from 57_quant.dat
// END - DIF BPF register values from 57_quant.dat
// case 5800000:
// BEGIN - DIF BPF register values from 58_quant.dat
// END - DIF BPF register values from 58_quant.dat
// case 5900000:
// BEGIN - DIF BPF register values from 59_quant.dat
// END - DIF BPF register values from 59_quant.dat
// case 6000000:
// BEGIN - DIF BPF register values from 60_quant.dat
// END - DIF BPF register values from 60_quant.dat
// case 6100000:
// BEGIN - DIF BPF register values from 61_quant.dat
// END - DIF BPF register values from 61_quant.dat
// case 6200000:
// BEGIN - DIF BPF register values from 62_quant.dat
// END - DIF BPF register values from 62_quant.dat
// case 6300000:
// BEGIN - DIF BPF register values from 63_quant.dat
// END - DIF BPF register values from 63_quant.dat
// case 6400000:
// BEGIN - DIF BPF register values from 64_quant.dat
// END - DIF BPF register values from 64_quant.dat
// case 6500000:
// BEGIN - DIF BPF register values from 65_quant.dat
// END - DIF BPF register values from 65_quant.dat
// case 6600000:
// BEGIN - DIF BPF register values from 66_quant.dat
// END - DIF BPF register values from 66_quant.dat
// case 6700000:
// BEGIN - DIF BPF register values from 67_quant.dat
// END - DIF BPF register values from 67_quant.dat
// case 6800000:
// BEGIN - DIF BPF register values from 68_quant.dat
// END - DIF BPF register values from 68_quant.dat
// case 6900000:
// BEGIN - DIF BPF register values from 69_quant.dat
// END - DIF BPF register values from 69_quant.dat
// case 7000000:
// BEGIN - DIF BPF register values from 70_quant.dat
// END - DIF BPF register values from 70_quant.dat
// case 7100000:
// BEGIN - DIF BPF register values from 71_quant.dat
// END - DIF BPF register values from 71_quant.dat
// case 7200000:
// BEGIN - DIF BPF register values from 72_quant.dat
// END - DIF BPF register values from 72_quant.dat
// case 7300000:
// BEGIN - DIF BPF register values from 73_quant.dat
// END - DIF BPF register values from 73_quant.dat
// case 7400000:
// BEGIN - DIF BPF register values from 74_quant.dat
// END - DIF BPF register values from 74_quant.dat
// case 7500000:
// BEGIN - DIF BPF register values from 75_quant.dat
// END - DIF BPF register values from 75_quant.dat
// case 7600000:
// BEGIN - DIF BPF register values from 76_quant.dat
// END - DIF BPF register values from 76_quant.dat
// case 7700000:
// BEGIN - DIF BPF register values from 77_quant.dat
// END - DIF BPF register values from 77_quant.dat
// case 7800000:
// BEGIN - DIF BPF register values from 78_quant.dat
// END - DIF BPF register values from 78_quant.dat
// case 7900000:
// BEGIN - DIF BPF register values from 79_quant.dat
// END - DIF BPF register values from 79_quant.dat
// case 8000000:
// BEGIN - DIF BPF register values from 80_quant.dat
// END - DIF BPF register values from 80_quant.dat
// case 8100000:
// BEGIN - DIF BPF register values from 81_quant.dat
// END - DIF BPF register values from 81_quant.dat
// case 8200000:
// BEGIN - DIF BPF register values from 82_quant.dat
// END - DIF BPF register values from 82_quant.dat
// case 8300000:
// BEGIN - DIF BPF register values from 83_quant.dat
// END - DIF BPF register values from 83_quant.dat
// case 8400000:
// BEGIN - DIF BPF register values from 84_quant.dat
// END - DIF BPF register values from 84_quant.dat
// case 8500000:
// BEGIN - DIF BPF register values from 85_quant.dat
// END - DIF BPF register values from 85_quant.dat
// case 8600000:
// BEGIN - DIF BPF register values from 86_quant.dat
// END - DIF BPF register values from 86_quant.dat
// case 8700000:
// BEGIN - DIF BPF register values from 87_quant.dat
// END - DIF BPF register values from 87_quant.dat
// case 8800000:
// BEGIN - DIF BPF register values from 88_quant.dat
// END - DIF BPF register values from 88_quant.dat
// case 8900000:
// BEGIN - DIF BPF register values from 89_quant.dat
// END - DIF BPF register values from 89_quant.dat
// case 9000000:
// BEGIN - DIF BPF register values from 90_quant.dat
// END - DIF BPF register values from 90_quant.dat
// case 9100000:
// BEGIN - DIF BPF register values from 91_quant.dat
// END - DIF BPF register values from 91_quant.dat
// case 9200000:
// BEGIN - DIF BPF register values from 92_quant.dat
// END - DIF BPF register values from 92_quant.dat
// case 9300000:
// BEGIN - DIF BPF register values from 93_quant.dat
// END - DIF BPF register values from 93_quant.dat
// case 9400000:
// BEGIN - DIF BPF register values from 94_quant.dat
// END - DIF BPF register values from 94_quant.dat
// case 9500000:
// BEGIN - DIF BPF register values from 95_quant.dat
// END - DIF BPF register values from 95_quant.dat
// case 9600000:
// BEGIN - DIF BPF register values from 96_quant.dat
// END - DIF BPF register values from 96_quant.dat
// case 9700000:
// BEGIN - DIF BPF register values from 97_quant.dat
// END - DIF BPF register values from 97_quant.dat
// case 9800000:
// BEGIN - DIF BPF register values from 98_quant.dat
// END - DIF BPF register values from 98_quant.dat
// case 9900000:
// BEGIN - DIF BPF register values from 99_quant.dat
// END - DIF BPF register values from 99_quant.dat
// case 10000000:
// BEGIN - DIF BPF register values from 100_quant.dat
// END - DIF BPF register values from 100_quant.dat
// case 10100000:
// BEGIN - DIF BPF register values from 101_quant.dat
// END - DIF BPF register values from 101_quant.dat
// case 10200000:
// BEGIN - DIF BPF register values from 102_quant.dat
// END - DIF BPF register values from 102_quant.dat
// case 10300000:
// BEGIN - DIF BPF register values from 103_quant.dat
// END - DIF BPF register values from 103_quant.dat
// case 10400000:
// BEGIN - DIF BPF register values from 104_quant.dat
// END - DIF BPF register values from 104_quant.dat
// case 10500000:
// BEGIN - DIF BPF register values from 105_quant.dat
// END - DIF BPF register values from 105_quant.dat
// case 10600000:
// BEGIN - DIF BPF register values from 106_quant.dat
// END - DIF BPF register values from 106_quant.dat
// case 10700000:
// BEGIN - DIF BPF register values from 107_quant.dat
// END - DIF BPF register values from 107_quant.dat
// case 10800000:
// BEGIN - DIF BPF register values from 108_quant.dat
// END - DIF BPF register values from 108_quant.dat
// case 10900000:
// BEGIN - DIF BPF register values from 109_quant.dat
// END - DIF BPF register values from 109_quant.dat
// case 11000000:
// BEGIN - DIF BPF register values from 110_quant.dat
// END - DIF BPF register values from 110_quant.dat
// case 11100000:
// BEGIN - DIF BPF register values from 111_quant.dat
// END - DIF BPF register values from 111_quant.dat
// case 11200000:
// BEGIN - DIF BPF register values from 112_quant.dat
// END - DIF BPF register values from 112_quant.dat
// case 11300000:
// BEGIN - DIF BPF register values from 113_quant.dat
// END - DIF BPF register values from 113_quant.dat
// case 11400000:
// BEGIN - DIF BPF register values from 114_quant.dat
// END - DIF BPF register values from 114_quant.dat
// case 11500000:
// BEGIN - DIF BPF register values from 115_quant.dat
// END - DIF BPF register values from 115_quant.dat
// case 11600000:
// BEGIN - DIF BPF register values from 116_quant.dat
// END - DIF BPF register values from 116_quant.dat
// case 11700000:
// BEGIN - DIF BPF register values from 117_quant.dat
// END - DIF BPF register values from 117_quant.dat
// case 11800000:
// BEGIN - DIF BPF register values from 118_quant.dat
// END - DIF BPF register values from 118_quant.dat
// case 11900000:
// BEGIN - DIF BPF register values from 119_quant.dat
// END - DIF BPF register values from 119_quant.dat
// case 12000000:
// BEGIN - DIF BPF register values from 120_quant.dat
// END - DIF BPF register values from 120_quant.dat
// case 12100000:
// BEGIN - DIF BPF register values from 121_quant.dat
// END - DIF BPF register values from 121_quant.dat
// case 12200000:
// BEGIN - DIF BPF register values from 122_quant.dat
// END - DIF BPF register values from 122_quant.dat
// case 12300000:
// BEGIN - DIF BPF register values from 123_quant.dat
// END - DIF BPF register values from 123_quant.dat
// case 12400000:
// BEGIN - DIF BPF register values from 124_quant.dat
// END - DIF BPF register values from 124_quant.dat
// case 12500000:
// BEGIN - DIF BPF register values from 125_quant.dat
// END - DIF BPF register values from 125_quant.dat
// case 12600000:
// BEGIN - DIF BPF register values from 126_quant.dat
// END - DIF BPF register values from 126_quant.dat
// case 12700000:
// BEGIN - DIF BPF register values from 127_quant.dat
// END - DIF BPF register values from 127_quant.dat
// case 12800000:
// BEGIN - DIF BPF register values from 128_quant.dat
// END - DIF BPF register values from 128_quant.dat
// case 12900000:
// BEGIN - DIF BPF register values from 129_quant.dat
// END - DIF BPF register values from 129_quant.dat
// case 113000000:
// BEGIN - DIF BPF register values from 130_quant.dat
// END - DIF BPF register values from 130_quant.dat
// case 13100000:
// BEGIN - DIF BPF register values from 131_quant.dat
// END - DIF BPF register values from 131_quant.dat
// case 13200000:
// BEGIN - DIF BPF register values from 132_quant.dat
// END - DIF BPF register values from 132_quant.dat
// case 13300000:
// BEGIN - DIF BPF register values from 133_quant.dat
// END - DIF BPF register values from 133_quant.dat
// case 13400000:
// BEGIN - DIF BPF register values from 134_quant.dat
// END - DIF BPF register values from 134_quant.dat
// case 13500000:
// BEGIN - DIF BPF register values from 135_quant.dat
// END - DIF BPF register values from 135_quant.dat
// case 13600000:
// BEGIN - DIF BPF register values from 136_quant.dat
// END - DIF BPF register values from 136_quant.dat
// case 13700000:
// BEGIN - DIF BPF register values from 137_quant.dat
// END - DIF BPF register values from 137_quant.dat
// case 13800000:
// BEGIN - DIF BPF register values from 138_quant.dat
// END - DIF BPF register values from 138_quant.dat
// case 13900000:
// BEGIN - DIF BPF register values from 139_quant.dat
// END - DIF BPF register values from 139_quant.dat
// case 14000000:
// BEGIN - DIF BPF register values from 140_quant.dat
// END - DIF BPF register values from 140_quant.dat
// case 14100000:
// BEGIN - DIF BPF register values from 141_quant.dat
// END - DIF BPF register values from 141_quant.dat
// case 14200000:
// BEGIN - DIF BPF register values from 142_quant.dat
// END - DIF BPF register values from 142_quant.dat
// case 14300000:
// BEGIN - DIF BPF register values from 143_quant.dat
// END - DIF BPF register values from 143_quant.dat
// case 14400000:
// BEGIN - DIF BPF register values from 144_quant.dat
// END - DIF BPF register values from 144_quant.dat
// case 14500000:
// BEGIN - DIF BPF register values from 145_quant.dat
// END - DIF BPF register values from 145_quant.dat
// case 14600000:
// BEGIN - DIF BPF register values from 146_quant.dat
// END - DIF BPF register values from 146_quant.dat
// case 14700000:
// BEGIN - DIF BPF register values from 147_quant.dat
// END - DIF BPF register values from 147_quant.dat
// case 14800000:
// BEGIN - DIF BPF register values from 148_quant.dat
// END - DIF BPF register values from 148_quant.dat
// case 14900000:
// BEGIN - DIF BPF register values from 149_quant.dat
// END - DIF BPF register values from 149_quant.dat
// case 15000000:
// BEGIN - DIF BPF register values from 150_quant.dat
// END - DIF BPF register values from 150_quant.dat
// case 15100000:
// BEGIN - DIF BPF register values from 151_quant.dat
// END - DIF BPF register values from 151_quant.dat
// case 15200000:
// BEGIN - DIF BPF register values from 152_quant.dat
// END - DIF BPF register values from 152_quant.dat
// case 115300000:
// BEGIN - DIF BPF register values from 153_quant.dat
// END - DIF BPF register values from 153_quant.dat
// case 115400000:
// BEGIN - DIF BPF register values from 154_quant.dat
// END - DIF BPF register values from 154_quant.dat
// case 115500000:
// BEGIN - DIF BPF register values from 155_quant.dat
// END - DIF BPF register values from 155_quant.dat
// case 115600000:
// BEGIN - DIF BPF register values from 156_quant.dat
// END - DIF BPF register values from 156_quant.dat
// case 115700000:
// BEGIN - DIF BPF register values from 157_quant.dat
// END - DIF BPF register values from 157_quant.dat
// case 115800000:
// BEGIN - DIF BPF register values from 158_quant.dat
// END - DIF BPF register values from 158_quant.dat
// case 115900000:
// BEGIN - DIF BPF register values from 159_quant.dat
// END - DIF BPF register values from 159_quant.dat
// case 116000000:
// BEGIN - DIF BPF register values from 160_quant.dat
// END - DIF BPF register values from 160_quant.dat
