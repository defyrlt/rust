// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unix-specific primitives available on all unix platforms

#![stable(feature = "raw_ext", since = "1.1.0")]

#[stable(feature = "raw_ext", since = "1.1.0")] pub type uid_t = u32;
#[stable(feature = "raw_ext", since = "1.1.0")] pub type gid_t = u32;
#[stable(feature = "raw_ext", since = "1.1.0")] pub type pid_t = i32;

#[doc(inline)]
#[unstable(feature = "pthread_t", issue = "29791")]
pub use sys::platform::raw::pthread_t;
#[doc(inline)]
#[stable(feature = "raw_ext", since = "1.1.0")]
pub use sys::platform::raw::{dev_t, ino_t, mode_t, nlink_t, off_t, blksize_t};
#[doc(inline)]
#[stable(feature = "raw_ext", since = "1.1.0")]
pub use sys::platform::raw::{blkcnt_t, time_t};
