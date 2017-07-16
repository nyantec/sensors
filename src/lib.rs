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

pub struct Chip {
	chip_name: libsensors_sys::sensors_chip_name
}

pub struct Feature {
	feature: libsensors_sys::sensors_feature
}

pub struct Subfeature {
	subfeature: libsensors_sys::sensors_subfeature
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

impl Drop for Chip {
	fn drop(&mut self) {
		unsafe {
			libsensors_sys::sensors_free_chip_name(&mut self.chip_name);
		}
	}
}
