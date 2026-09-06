//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/kvm/s390/gaccess.h
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
// access guest memory
//
// Copyright IBM Corp. 2008, 2014
//
// Author(s): Carsten Otte <cotte@de.ibm.com>
//

//
// kvm_s390_real_to_abs - convert guest real address to guest absolute address
// @prefix - guest prefix
// @gra - guest real address
//
// Returns the guest absolute address that corresponds to the passed guest real
// address @gra of by applying the given prefix.
//
// kvm_s390_real_to_abs - convert guest real address to guest absolute address
// @vcpu - guest virtual cpu
// @gra - guest real address
//
// Returns the guest absolute address that corresponds to the passed guest real
// address @gra of a virtual guest cpu by applying its prefix.
//
extern "C" {
    pub fn _kvm_s390_real_to_abs(_arg: kvm_s390_get_prefix(vcpu), _arg: gra) -> return;
}
//
// _kvm_s390_logical_to_effective - convert guest logical to effective address
// @psw: psw of the guest
// @ga: guest logical address
//
// Convert a guest logical address to an effective address by applying the
// rules of the addressing mode defined by bits 31 and 32 of the given PSW
// (extendended/basic addressing mode).
//
// Depending on the addressing mode, the upper 40 bits (24 bit addressing
// mode), 33 bits (31 bit addressing mode) or no bits (64 bit addressing
// mode) of @ga will be zeroed and the remaining bits will be returned.
//
// kvm_s390_logical_to_effective - convert guest logical to effective address
// @vcpu: guest virtual cpu
// @ga: guest logical address
//
// Convert a guest vcpu logical address to a guest vcpu effective address by
// applying the rules of the vcpu's addressing mode defined by PSW bits 31
// and 32 (extendended/basic addressing mode).
//
// Depending on the vcpu's addressing mode the upper 40 bits (24 bit addressing
// mode), 33 bits (31 bit addressing mode) or no bits (64 bit addressing mode)
// of @ga will be zeroed and the remaining bits will be returned.
//
extern "C" {
    pub fn _kvm_s390_logical_to_effective(_arg: &vcpu->arch.sie_block->gpsw, _arg: ga) -> return;
}
//
// put_guest_lc, read_guest_lc and write_guest_lc are guest access functions
// which shall only be used to access the lowcore of a vcpu.
// These functions should be used for e.g. interrupt handlers where no
// guest memory access protection facilities, like key or low address
// protection, are applicable.
// At a later point guest vcpu lowcore access should happen via pinned
// prefix pages, so that these pages can be accessed directly via the
// kernel mapping. All of these *_lc functions can be removed then.
//
// put_guest_lc - write a simple variable to a guest vcpu's lowcore
// @vcpu: virtual cpu
// @x: value to copy to guest
// @gra: vcpu's destination guest real address
//
// Copies a simple value from kernel space to a guest vcpu's lowcore.
// The size of the variable may be 1, 2, 4 or 8 bytes. The destination
// must be located in the vcpu's lowcore. Otherwise the result is undefined.
//
// Returns zero on success or -EFAULT on error.
//
// Note: an error indicates that either the kernel is out of memory or
// the guest memory mapping is broken. In any case the best solution
// would be to terminate the guest.
// It is wrong to inject a guest exception.
//

//
// write_guest_lc - copy data from kernel space to guest vcpu's lowcore
// @vcpu: virtual cpu
// @gra: vcpu's source guest real address
// @data: source address in kernel space
// @len: number of bytes to copy
//
// Copy data from kernel space to guest vcpu's lowcore. The entire range must
// be located within the vcpu's lowcore, otherwise the result is undefined.
//
// Returns zero on success or -EFAULT on error.
//
// Note: an error indicates that either the kernel is out of memory or
// the guest memory mapping is broken. In any case the best solution
// would be to terminate the guest.
// It is wrong to inject a guest exception.
//
extern "C" {
    pub fn kvm_write_guest(_arg: vcpu->kvm, _arg: gpa, _arg: data, _arg: len) -> return;
}
//
// read_guest_lc - copy data from guest vcpu's lowcore to kernel space
// @vcpu: virtual cpu
// @gra: vcpu's source guest real address
// @data: destination address in kernel space
// @len: number of bytes to copy
//
// Copy data from guest vcpu's lowcore to kernel space. The entire range must
// be located within the vcpu's lowcore, otherwise the result is undefined.
//
// Returns zero on success or -EFAULT on error.
//
// Note: an error indicates that either the kernel is out of memory or
// the guest memory mapping is broken. In any case the best solution
// would be to terminate the guest.
// It is wrong to inject a guest exception.
//
extern "C" {
    pub fn kvm_read_guest(_arg: vcpu->kvm, _arg: gpa, _arg: data, _arg: len) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gacc_mode {
    GACC_FETCH,
    GACC_STORE,
    GACC_IFETCH,
}

//
// write_guest_with_key - copy data from kernel space to guest space
// @vcpu: virtual cpu
// @ga: guest address
// @ar: access register
// @data: source address in kernel space
// @len: number of bytes to copy
// @access_key: access key the storage key needs to match
//
// Copy @len bytes from @data (kernel space) to @ga (guest address).
// In order to copy data to guest space the PSW of the vcpu is inspected:
// If DAT is off data will be copied to guest real or absolute memory.
// If DAT is on data will be copied to the address space as specified by
// the address space bits of the PSW:
// Primary, secondary, home space or access register mode.
// The addressing mode of the PSW is also inspected, so that address wrap
// around is taken into account for 24-, 31- and 64-bit addressing mode,
// if the to be copied data crosses page boundaries in guest address space.
// In addition low address, DAT and key protection checks are performed before
// copying any data.
//
// This function modifies the 'struct kvm_s390_pgm_info pgm' member of @vcpu.
// In case of an access exception (e.g. protection exception) pgm will contain
// all data necessary so that a subsequent call to 'kvm_s390_inject_prog_vcpu()'
// will inject a correct exception into the guest.
// If no access exception happened, the contents of pgm are undefined when
// this function returns.
//
// Returns:  - zero on success
// - a negative value if e.g. the guest mapping is broken or in
// case of out-of-memory. In this case the contents of pgm are
// undefined. Also parts of @data may have been copied to guest
// space.
// - a positive value if an access exception happened. In this case
// the returned value is the program interruption code and the
// contents of pgm may be used to inject an exception into the
// guest. No data has been copied to guest space.
//
// Note: in case an access exception is recognized no data has been copied to
// guest space (this is also true, if the to be copied data would cross
// one or more page boundaries in guest space).
// Therefore this function may be used for nullifying and suppressing
// instruction emulation.
// It may also be used for terminating instructions, if it is undefined
// if data has been changed in guest space in case of an exception.
//
// write_guest - copy data from kernel space to guest space
// @vcpu: virtual cpu
// @ga: guest address
// @ar: access register
// @data: source address in kernel space
// @len: number of bytes to copy
//
// The behaviour of write_guest is identical to write_guest_with_key, except
// that the PSW access key is used instead of an explicit argument.
//
extern "C" {
    pub fn write_guest_with_key(_arg: vcpu, _arg: ga, _arg: ar, _arg: data, _arg: len, _arg: access_key) -> return;
}
//
// read_guest_with_key - copy data from guest space to kernel space
// @vcpu: virtual cpu
// @ga: guest address
// @ar: access register
// @data: destination address in kernel space
// @len: number of bytes to copy
// @access_key: access key the storage key needs to match
//
// Copy @len bytes from @ga (guest address) to @data (kernel space).
//
// The behaviour of read_guest_with_key is identical to write_guest_with_key,
// except that data will be copied from guest space to kernel space.
//
// read_guest - copy data from guest space to kernel space
// @vcpu: virtual cpu
// @ga: guest address
// @ar: access register
// @data: destination address in kernel space
// @len: number of bytes to copy
//
// Copy @len bytes from @ga (guest address) to @data (kernel space).
//
// The behaviour of read_guest is identical to read_guest_with_key, except
// that the PSW access key is used instead of an explicit argument.
//
extern "C" {
    pub fn read_guest_with_key(_arg: vcpu, _arg: ga, _arg: ar, _arg: data, _arg: len, _arg: access_key) -> return;
}
//
// read_guest_instr - copy instruction data from guest space to kernel space
// @vcpu: virtual cpu
// @ga: guest address
// @data: destination address in kernel space
// @len: number of bytes to copy
//
// Copy @len bytes from the given address (guest space) to @data (kernel
// space).
//
// The behaviour of read_guest_instr is identical to read_guest, except that
// instruction data will be read from primary space when in home-space or
// address-space mode.
//
// write_guest_abs - copy data from kernel space to guest space absolute
// @vcpu: virtual cpu
// @gpa: guest physical (absolute) address
// @data: source address in kernel space
// @len: number of bytes to copy
//
// Copy @len bytes from @data (kernel space) to @gpa (guest absolute address).
// It is up to the caller to ensure that the entire guest memory range is
// valid memory before calling this function.
// Guest low address and key protection are not checked.
//
// Returns zero on success or -EFAULT on error.
//
// If an error occurs data may have been copied partially to guest memory.
//
extern "C" {
    pub fn kvm_write_guest(_arg: vcpu->kvm, _arg: gpa, _arg: data, _arg: len) -> return;
}
//
// read_guest_abs - copy data from guest space absolute to kernel space
// @vcpu: virtual cpu
// @gpa: guest physical (absolute) address
// @data: destination address in kernel space
// @len: number of bytes to copy
//
// Copy @len bytes from @gpa (guest absolute address) to @data (kernel space).
// It is up to the caller to ensure that the entire guest memory range is
// valid memory before calling this function.
// Guest key protection is not checked.
//
// Returns zero on success or -EFAULT on error.
//
// If an error occurs data may have been copied partially to kernel space.
//
extern "C" {
    pub fn kvm_read_guest(_arg: vcpu->kvm, _arg: gpa, _arg: data, _arg: len) -> return;
}
//
// write_guest_real - copy data from kernel space to guest space real
// @vcpu: virtual cpu
// @gra: guest real address
// @data: source address in kernel space
// @len: number of bytes to copy
//
// Copy @len bytes from @data (kernel space) to @gra (guest real address).
// Guest low address and key protection are not checked.
//
// Returns zero on success, -EFAULT when copying from @data failed, or
// PGM_ADRESSING in case @gra is outside a memslot. In this case, pgm check info
// is also stored to allow injecting into the guest (if applicable) using
// kvm_s390_inject_prog_cond().
//
// If an error occurs data may have been copied partially to guest memory.
//
extern "C" {
    pub fn access_guest_real(_arg: vcpu, _arg: gra, _arg: data, _arg: len, _arg: 1) -> return;
}
//
// read_guest_real - copy data from guest space real to kernel space
// @vcpu: virtual cpu
// @gra: guest real address
// @data: destination address in kernel space
// @len: number of bytes to copy
//
// Copy @len bytes from @gra (guest real address) to @data (kernel space).
// Guest key protection is not checked.
//
// Returns zero on success, -EFAULT when copying to @data failed, or
// PGM_ADRESSING in case @gra is outside a memslot. In this case, pgm check info
// is also stored to allow injecting into the guest (if applicable) using
// kvm_s390_inject_prog_cond().
//
// If an error occurs data may have been copied partially to kernel space.
//
extern "C" {
    pub fn access_guest_real(_arg: vcpu, _arg: gra, _arg: data, _arg: len, _arg: 0) -> return;
}
extern "C" {
    pub fn ipte_lock(kvm: *mut kvm);
}
extern "C" {
    pub fn ipte_unlock(kvm: *mut kvm);
}
extern "C" {
    pub fn ipte_lock_held(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvm_s390_check_low_addr_prot_real(vcpu: *mut kvm_vcpu, gra: c_ulong) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union mvpg_pei {
    pub val: c_ulong,
    pub 61: unsigned long addr :,
    pub 1: unsigned long not_pte :,
    pub 1: unsigned long dat_prot:,
    pub 1: unsigned long real :,
}
