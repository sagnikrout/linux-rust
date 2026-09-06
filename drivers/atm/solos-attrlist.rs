//! Automatically rewritten from C to Rust
//! Source: drivers/atm/solos-attrlist.c
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
    SOLOS_ATTR_RO(DriverVersion)
    SOLOS_ATTR_RO(APIVersion)
    SOLOS_ATTR_RO(FirmwareVersion)
    SOLOS_ATTR_RO(Version)
// SOLOS_ATTR_RO(DspVersion)
// SOLOS_ATTR_RO(CommonHandshake)
    SOLOS_ATTR_RO(Connected)
    SOLOS_ATTR_RO(OperationalMode)
    SOLOS_ATTR_RO(State)
    SOLOS_ATTR_RO(Watchdog)
    SOLOS_ATTR_RO(OperationProgress)
    SOLOS_ATTR_RO(LastFailed)
    SOLOS_ATTR_RO(TxBitRate)
    SOLOS_ATTR_RO(RxBitRate)
// SOLOS_ATTR_RO(DeltACTATPds)
// SOLOS_ATTR_RO(DeltACTATPus)
    SOLOS_ATTR_RO(TxATTNDR)
    SOLOS_ATTR_RO(RxATTNDR)
    SOLOS_ATTR_RO(AnnexType)
    SOLOS_ATTR_RO(GeneralFailure)
    SOLOS_ATTR_RO(InterleaveDpDn)
    SOLOS_ATTR_RO(InterleaveDpUp)
    SOLOS_ATTR_RO(RSCorrectedErrorsDn)
    SOLOS_ATTR_RO(RSUnCorrectedErrorsDn)
    SOLOS_ATTR_RO(RSCorrectedErrorsUp)
    SOLOS_ATTR_RO(RSUnCorrectedErrorsUp)
    SOLOS_ATTR_RO(InterleaveRDn)
    SOLOS_ATTR_RO(InterleaveRUp)
    SOLOS_ATTR_RO(BisRDn)
    SOLOS_ATTR_RO(BisRUp)
    SOLOS_ATTR_RO(INPdown)
    SOLOS_ATTR_RO(INPup)
    SOLOS_ATTR_RO(ShowtimeStart)
    SOLOS_ATTR_RO(ATURVendor)
    SOLOS_ATTR_RO(ATUCCountry)
    SOLOS_ATTR_RO(ATURANSIRev)
    SOLOS_ATTR_RO(ATURANSISTD)
    SOLOS_ATTR_RO(ATUCANSIRev)
    SOLOS_ATTR_RO(ATUCANSIId)
    SOLOS_ATTR_RO(ATUCANSISTD)
    SOLOS_ATTR_RO(DataBoost)
    SOLOS_ATTR_RO(LocalITUCountryCode)
    SOLOS_ATTR_RO(LocalSEF)
    SOLOS_ATTR_RO(LocalEndLOS)
    SOLOS_ATTR_RO(LocalSNRMargin)
    SOLOS_ATTR_RO(LocalLineAttn)
    SOLOS_ATTR_RO(RawAttn)
    SOLOS_ATTR_RO(LocalTxPower)
    SOLOS_ATTR_RO(RemoteTxPower)
    SOLOS_ATTR_RO(RemoteSEF)
    SOLOS_ATTR_RO(RemoteLOS)
    SOLOS_ATTR_RO(RemoteLineAttn)
    SOLOS_ATTR_RO(RemoteSNRMargin)
    SOLOS_ATTR_RO(LineUpCount)
    SOLOS_ATTR_RO(SRACnt)
    SOLOS_ATTR_RO(SRACntUp)
    SOLOS_ATTR_RO(ProfileStatus)
    SOLOS_ATTR_RW(Action)
    SOLOS_ATTR_RW(ActivateLine)
    SOLOS_ATTR_RO(LineStatus)
    SOLOS_ATTR_RW(HostControl)
    SOLOS_ATTR_RW(AutoStart)
    SOLOS_ATTR_RW(Failsafe)
    SOLOS_ATTR_RW(ShowtimeLed)
    SOLOS_ATTR_RW(Retrain)
    SOLOS_ATTR_RW(Defaults)
    SOLOS_ATTR_RW(LineMode)
    SOLOS_ATTR_RW(Profile)
    SOLOS_ATTR_RW(DetectNoise)
    SOLOS_ATTR_RW(BisAForceSNRMarginDn)
    SOLOS_ATTR_RW(BisMForceSNRMarginDn)
    SOLOS_ATTR_RW(BisAMaxMargin)
    SOLOS_ATTR_RW(BisMMaxMargin)
    SOLOS_ATTR_RW(AnnexAForceSNRMarginDn)
    SOLOS_ATTR_RW(AnnexAMaxMargin)
    SOLOS_ATTR_RW(AnnexMMaxMargin)
    SOLOS_ATTR_RO(SupportedAnnexes)
    SOLOS_ATTR_RO(Status)
    SOLOS_ATTR_RO(TotalStart)
    SOLOS_ATTR_RO(RecentShowtimeStart)
    SOLOS_ATTR_RO(TotalRxBlocks)
    SOLOS_ATTR_RO(TotalTxBlocks)
