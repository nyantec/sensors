#![feature(libc)]

extern crate libc;
extern crate libsensors_sys;

use std::sync::{Once, ONCE_INIT};
use std::marker::PhantomData;
use std::ptr;

static INIT: Once = ONCE_INIT;

#[derive(Copy, Clone)]
pub struct Sensors {
	marker: PhantomData<()>
}

impl Sensors {
	pub fn new() -> Self {
		INIT.call_once(|| {
			unsafe {
				assert_eq!(libsensors_sys::sensors_init(ptr::null_mut()), 0);
				assert_eq!(libc::atexit(Self::cleanup), 0);
			}
		});

		Sensors {
			marker: PhantomData
		}
	}

	extern fn cleanup() {
		unsafe {
			libsensors_sys::sensors_cleanup();
		}
	}
}
