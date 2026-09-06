//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/libarena/include/libarena/bitmap.h
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


pub const BITS_PER_BYTE: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arena_bitmap {
    pub bits: [u64; 0],
}

extern "C" {
    pub fn bmp_free(bmp: *mut arena_bitmap __arena);
}
extern "C" {
    pub fn __bmp_set_bit(bit: u32, bmp: *mut arena_bitmap __arena);
}
extern "C" {
    pub fn __bmp_clear_bit(bit: u32, bmp: *mut arena_bitmap __arena);
}
extern "C" {
    pub fn bmp_set_bit(bit: u32, bmp: *mut arena_bitmap __arena);
}
extern "C" {
    pub fn bmp_clear_bit(bit: u32, bmp: *mut arena_bitmap __arena);
}
extern "C" {
    pub fn bmp_test_bit(bit: u32, bmp: *mut arena_bitmap __arena) -> bool;
}
extern "C" {
    pub fn bmp_test_and_clear_bit(bit: u32, bmp: *mut arena_bitmap __arena) -> bool;
}
extern "C" {
    pub fn bmp_test_and_set_bit(bit: u32, bmp: *mut arena_bitmap __arena) -> bool;
}
extern "C" {
    pub fn bmp_clear(bits: usize, bmp: *mut arena_bitmap __arena);
}
extern "C" {
    pub fn bmp_and(bits: usize, dst: *mut arena_bitmap __arena, src1: *mut arena_bitmap __arena, src2: *mut arena_bitmap __arena);
}
extern "C" {
    pub fn bmp_or(bits: usize, dst: *mut arena_bitmap __arena, src1: *mut arena_bitmap __arena, src2: *mut arena_bitmap __arena);
}
extern "C" {
    pub fn bmp_empty(bits: usize, bmp: *mut arena_bitmap __arena) -> bool;
}
extern "C" {
    pub fn bmp_copy(bits: usize, dst: *mut arena_bitmap __arena, src: *mut arena_bitmap __arena);
}
extern "C" {
    pub fn bmp_intersects(bits: usize, arg1: *mut arena_bitmap __arena, arg2: *mut arena_bitmap __arena) -> bool;
}
extern "C" {
    pub fn bmp_subset(bits: usize, big: *mut arena_bitmap __arena, small: *mut arena_bitmap __arena) -> bool;
}
extern "C" {
    pub fn bmp_print(bits: usize, bmp: *mut arena_bitmap __arena);
}
