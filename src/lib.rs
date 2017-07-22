extern crate libc;
extern crate libsensors_sys;

use std::sync::{Once, ONCE_INIT};
use std::marker::PhantomData;
use std::ptr;
use std::mem;
use std::ffi::CStr;

static INIT: Once = ONCE_INIT;

#[derive(Copy, Clone, Debug)]
pub struct Sensors {
	marker: PhantomData<()>,

}

#[derive(Debug)]
pub struct Chip {
	pub chip_name: libsensors_sys::sensors_chip_name,
}

pub struct ChipIterator {
    index: i32
}

#[derive(Debug)]
pub struct Feature {
    pub chip_name: libsensors_sys::sensors_chip_name,
	pub feature: libsensors_sys::sensors_feature
}

pub struct FeatureIterator {
    chip_name: libsensors_sys::sensors_chip_name,
    index: i32
}

#[derive(Debug)]
pub struct Subfeature {
    pub chip_name: libsensors_sys::sensors_chip_name,
    pub feature: libsensors_sys::sensors_feature,
	pub subfeature: libsensors_sys::sensors_subfeature,
    pub name: String,
    pub value: f64
}

pub struct SubfeatureIterator {
    chip_name: libsensors_sys::sensors_chip_name,
    feature: libsensors_sys::sensors_feature,
    index: i32
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

impl IntoIterator for Sensors {
    type Item = Chip;
    type IntoIter = ChipIterator;

    fn into_iter(self) -> Self::IntoIter {
        ChipIterator { index: 0 }.into_iter()
    }

}

impl Iterator for ChipIterator {
    type Item = Chip;

    fn next(&mut self) -> Option<Self::Item> {
        let chip_ = unsafe { libsensors_sys::sensors_get_detected_chips(ptr::null_mut(), &mut self.index) };
        if chip_ != ptr::null_mut() {
            return Some(Chip { chip_name: unsafe { *chip_ }});
        };
        None
    }
}

impl IntoIterator for Chip {
    type Item = Feature;
    type IntoIter = FeatureIterator;

    fn into_iter(self) -> Self::IntoIter {
        FeatureIterator { index: 0, chip_name: self.chip_name }.into_iter()
    }
}

impl Iterator for FeatureIterator {
    type Item = Feature;

    fn next(&mut self) -> Option<Self::Item> {
        let feature_ = unsafe { libsensors_sys::sensors_get_features(&self.chip_name, &mut self.index) };
        if feature_ != ptr::null_mut() {
            return Some(Feature { feature: unsafe { *feature_ }, chip_name: self.chip_name });
        };
        None
    }
}

impl IntoIterator for Feature {
    type Item = Subfeature;
    type IntoIter = SubfeatureIterator;

    fn into_iter(self) -> Self::IntoIter {
        SubfeatureIterator { index: 0, chip_name: self.chip_name, feature: self.feature }.into_iter()
    }
}

impl Iterator for SubfeatureIterator {
    type Item = Subfeature;

    fn next(&mut self) -> Option<Self::Item> {
        let subfeature_ = unsafe { libsensors_sys::sensors_get_all_subfeatures(&self.chip_name, &self.feature, &mut self.index) };
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
                    return Some(Subfeature { subfeature: unsafe { *subfeature_ },
                                             value:val,
                                             name: name.to_string_lossy().into_owned(),
                                             chip_name: self.chip_name,
                                             feature: self.feature });
                }
            }
        };
        None
    }
}
