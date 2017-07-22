extern crate libc;
extern crate libsensors_sys;

use std::sync::{Once, ONCE_INIT};
use std::marker::PhantomData;
use std::ptr;
use std::mem;
use std::ffi::CStr;

static INIT: Once = ONCE_INIT;

#[derive(Copy, Clone)]
pub struct Sensors {
	marker: PhantomData<()>,
	chip_iterator_index: i32
}

#[derive(Debug)]
pub struct Chip {
	chip_name: libsensors_sys::sensors_chip_name,
    feature_iterator_index: i32
}
#[derive(Debug)]
pub struct Feature {
    chip_name: libsensors_sys::sensors_chip_name,
	feature: libsensors_sys::sensors_feature,
    subfeature_iterator_index: i32
}
#[derive(Debug)]
pub struct Subfeature {
	subfeature: libsensors_sys::sensors_subfeature,
    name: String,
    value: f64
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
			marker: PhantomData,
			chip_iterator_index: 0
		}
	}

	extern fn cleanup() {
		unsafe {
			libsensors_sys::sensors_cleanup();
		}
	}
}

impl Iterator for Sensors {
    type Item = Chip;

    fn next(&mut self) -> Option<Self::Item> {
    	let chip_ = unsafe { libsensors_sys::sensors_get_detected_chips(ptr::null_mut(), &mut self.chip_iterator_index) };
        if chip_ != ptr::null_mut() {
            return Some(Chip { chip_name: unsafe { *chip_ }, feature_iterator_index: 0});
        };
        None
    }
}

impl Iterator for Chip {
    type Item = Feature;

    fn next(&mut self) -> Option<Self::Item> {
        let feature_ = unsafe { libsensors_sys::sensors_get_features(&self.chip_name, &mut self.feature_iterator_index) };
        if feature_ != ptr::null_mut() {
            return Some(Feature { feature: unsafe { *feature_ }, chip_name: self.chip_name, subfeature_iterator_index: 0 });
        };
        None
    }
}

impl Iterator for Feature {
    type Item = Subfeature;

    fn next(&mut self) -> Option<Self::Item> {
        let subfeature_ = unsafe { libsensors_sys::sensors_get_all_subfeatures(&self.chip_name, &self.feature, &mut self.subfeature_iterator_index) };
        if subfeature_ != ptr::null_mut() {
            let subfeature = unsafe { *subfeature_ };
            // val will be overwritten, no risk
            let mut val = unsafe { mem::zeroed() };
            // Check that subfeature is readable
            if (subfeature.flags & libsensors_sys::SENSORS_MODE_R) != 0 {
                // Call to c function, no risk
                let r = unsafe { libsensors_sys::sensors_get_value(&self.chip_name, subfeature.number, &mut val) };
                // check that value was retrieved (if r<0, error occcured)
                if r >= 0 {
                    let name = unsafe { CStr::from_ptr(subfeature.name) };
                    return Some(Subfeature { subfeature: unsafe { *subfeature_ }, value:val, name: name.to_string_lossy().into_owned() });
                }
            }
        };
        None
    }
}


#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn iter_chips() {
    	let s = Sensors::new();
        for c in s {
            for f in c {
                for sf in f {
                    println!("{:?}", sf);
                }

            }
        }
    }
}
