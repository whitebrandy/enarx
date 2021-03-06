// SPDX-License-Identifier: Apache-2.0

//! Types and Constants to create an ELF AUXV

use crate::auxv::Key::Null;

/// The basic AuxvType able to hold a pointer
pub type Type = usize;

/// AuxvEntry to be used with `Crt0Stack::add_auxv_entry()`
pub enum Entry<'a> {
    /// file descriptor of program
    ExecFd(Type), // core does not have RawFd

    /// program headers for program
    PHdr(Type),

    /// size of program header entry
    PHent(Type),

    /// number of program headers
    PHnum(Type),

    /// system page size
    Pagesize(Type),

    /// base address of interpreter
    Base(Type),

    /// flags
    Flags(Type),

    /// entry point of program
    Entry(Type),

    /// program is not ELF
    Notelf(bool),

    /// real uid
    Uid(Type),

    /// effective uid
    EUid(Type),

    /// real gid
    Gid(Type),

    /// effective gid
    EGid(Type),

    /// string identifying CPU for optimizations
    Platform(&'a str),

    /// arch dependent hints at CPU capabilities
    HWCap(Type),

    /// frequency at which times() increments
    ClockTick(Type),

    /// secure mode boolean
    Secure(bool),

    /// string identifying real platform, may differ from Platform.
    BasePlatform(&'a str),

    /// address of 16 random bytes
    Random([u8; 16]),

    /// extension of HWCAP
    HWCap2(Type),

    /// filename of program
    ExecFilename(&'a str),
}

#[non_exhaustive]
#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Key {
    /// end of vector
    Null = 0,

    /// entry should be ignored
    //Ignore = 1,

    /// file descriptor of program
    ExecFd = 2,

    /// program headers for program
    PHdr = 3,

    /// size of program header entry
    PHent = 4,

    /// number of program headers
    PHnum = 5,

    /// system page size
    Pagesize = 6,

    /// base address of interpreter
    Base = 7,

    /// flags
    Flags = 8,

    /// entry point of program
    Entry = 9,

    /// program is not ELF
    NotElf = 10,

    /// real uid
    Uid = 11,

    /// effective uid
    EUid = 12,

    /// real gid
    Gid = 13,

    /// effective gid
    EGid = 14,

    /// string identifying CPU for optimizations
    Platform = 15,

    /// arch dependent hints at CPU capabilities
    HWCap = 16,

    /// frequency at which times() increments
    ClockTick = 17,

    /// secure mode boolean
    Secure = 23,

    /// string identifying real platform, may differ from Platform.
    BasePlatform = 24,

    /// address of 16 random bytes
    Random = 25,

    /// extension of AT_HWCAP
    HWCap2 = 26,

    /// filename of program
    ExecFilename = 31,
}

impl Default for Key {
    fn default() -> Self {
        Null
    }
}
