#[macro_use]
extern crate vst;

use vst::buffer::{AudioBuffer};
use vst::plugin::{Info, Plugin, PluginParameters};
use std::sync::atomic::{AtomicBool, Ordering};

use std::sync::Arc;

/*
const BTF_N_FACTOR_INT: i32 = 0x800000;
const BTF_N_FACTOR: f32 = -(BTF_N_FACTOR_INT as f32);
const BTF_P_FACTOR: f32 = BTF_N_FACTOR_INT as f32;
 */

struct ESParameters {
		gate1: AtomicBool,
		gate2: AtomicBool,
		gate3: AtomicBool,
		gate4: AtomicBool,
		gate5: AtomicBool,
		gate6: AtomicBool,
}

impl Default for ESParameters {
		fn default() -> ESParameters {
				ESParameters {
						gate1: AtomicBool::new(false),
						gate2: AtomicBool::new(false),
						gate3: AtomicBool::new(false),
						gate4: AtomicBool::new(false),
						gate5: AtomicBool::new(false),
						gate6: AtomicBool::new(false),
				}
		}
}

/*
fn bits_to_float(bits: i32) -> f32 {
		if bits & BTF_N_FACTOR_INT != 0 {
				(0xFFFFFF as f32 & -(bits as f32)) / BTF_N_FACTOR
		} else {
				return (bits as f32) / BTF_P_FACTOR;
		}
}
*/

impl PluginParameters for ESParameters {
		fn get_parameter(&self, index: i32) -> f32 {
				match index {
						0 => self.gate1.load(Ordering::Acquire) as i32 as f32,
						1 => self.gate2.load(Ordering::Acquire) as i32 as f32,
						2 => self.gate3.load(Ordering::Acquire) as i32 as f32,
						3 => self.gate4.load(Ordering::Acquire) as i32 as f32,
						4 => self.gate5.load(Ordering::Acquire) as i32 as f32,
						5 => self.gate6.load(Ordering::Acquire) as i32 as f32,
						_ => 0.0,
				}
		}

		fn set_parameter(&self, index: i32, val: f32) {
				match index {
						0 => self.gate1.store(val != 0.0, Ordering::Relaxed),
						1 => self.gate2.store(val != 0.0, Ordering::Relaxed),
						2 => self.gate3.store(val != 0.0, Ordering::Relaxed),
						3 => self.gate4.store(val != 0.0, Ordering::Relaxed),
						4 => self.gate5.store(val != 0.0, Ordering::Relaxed),
						5 => self.gate6.store(val != 0.0, Ordering::Relaxed),
						_ => (),
				}
		}

		fn get_parameter_text(&self, index: i32) -> String {
				match index {
						0 => format!("{}", self.gate1.load(Ordering::Acquire)),
						1 => format!("{}", self.gate2.load(Ordering::Acquire)),
						2 => format!("{}", self.gate3.load(Ordering::Acquire)),
						3 => format!("{}", self.gate4.load(Ordering::Acquire)),
						4 => format!("{}", self.gate5.load(Ordering::Acquire)),
						5 => format!("{}", self.gate6.load(Ordering::Acquire)),
						_ => "".to_string(),
				}
		}

		fn get_parameter_name(&self, index: i32) -> String {
				match index {
						0 => "Out 1",
						1 => "Out 2",
						2 => "Out 3",
						3 => "Out 4",
						4 => "Out 5",
						5 => "Out 6",
						_ => "",
				}
				.to_string()
		}
}

#[derive(Clone)]
struct ESPlugin {
		params: Arc<ESParameters>,
}

impl Default for ESPlugin {
		fn default() -> ESPlugin {
				ESPlugin {
						params: Arc::new(ESParameters::default()),
				}
		}
}

impl Plugin for ESPlugin {
		fn get_info(&self) -> Info {
				Info {
						name: "ES-5 output".to_string(),
						unique_id: 472389432,
						version: 0001,
						inputs: 0,
						outputs: 2, // ES-5 takes stereo audio.
						parameters: 6,
						..Default::default()
				}
		}

		fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
				//bitsR;
				// (0 << 16) | (1 << 8) | 2
				let bits_l = (
						((self.params.gate1.load(Ordering::Acquire) as i32) << 16) |
						((self.params.gate2.load(Ordering::Acquire) as i32) << 8) |
						self.params.gate3.load(Ordering::Acquire) as i32
				);
				// (3 << 16) | (4 << 8) | 5
				let bits_r = (
						((self.params.gate4.load(Ordering::Acquire) as i32) << 16) |
						((self.params.gate5.load(Ordering::Acquire) as i32) << 8) |
						self.params.gate6.load(Ordering::Acquire) as i32
				);
				let samples = buffer.samples();
				let (_, mut outputs) = buffer.split();
				let val_l;
				if (0x800000 & bits_l) as f32 > 0.0 {
						val_l = (bits_l as f32) / (-1.0 * (0x800000 as f32));
				} else {
						val_l = (0xFFFFFF & bits_l) as f32 / (0x800000 as f32);
				}
				let val_r;
				if (0x800000 & bits_r) as f32 > 0.0 {
						val_r = (bits_r as f32) / (-1.0 * (0x800000 as f32));
				} else {
						val_r = (0xFFFFFF & bits_r) as f32 / (0x800000 as f32);
				}
				for sample_idx in 0..samples {
						let buff_l = outputs.get_mut(0);
						let buff_r = outputs.get_mut(1);
						buff_l[sample_idx] = val_l;
						buff_r[sample_idx] = val_r;
				}
				
				/*
				let mut bits = 0;
				if self.params.gate1.load(Ordering::Acquire) {
						bits += 1 << 0;
				}
				if self.params.gate2.load(Ordering::Acquire) {
						bits += 1 << 1;
				}
				if self.params.gate3.load(Ordering::Acquire) {
						bits += 1 << 2;
				}

				let samples = buffer.samples();
				let (_, mut outputs) = buffer.split();
				let output_count = outputs.len();
				let val;
				if (0x800000 & bits) as f32 > 0.0 {
						val = (bits as f32) / (-1.0 * (0x800000 as f32));
				} else {
						val = (0xFFFFFF & bits) as f32 / (0x800000 as f32);
				}
				for sample_idx in 0..samples {
						
						/*
						for buf_idx in 0..output_count {
								let buff = outputs.get_mut(buf_idx);
								buff[sample_idx] = val + 0.1;
						}
						 */
				}
				*/
		}

		fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
				Arc::clone(&self.params) as Arc<dyn PluginParameters>
		}
}

plugin_main!(ESPlugin);
