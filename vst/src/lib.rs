#[macro_use]
extern crate vst;

use vst::buffer::{AudioBuffer};
//use vst::util::AtomicFloat;
use vst::plugin::{Info, Plugin, PluginParameters};
use std::sync::atomic::{AtomicBool, Ordering};

use std::sync::Arc;

const BTF_FACTOR_INT: i32 = 0x800000;
const BTF_N_FACTOR: f32 = -(BTF_FACTOR_INT as f32);
const BTF_P_FACTOR: f32 = BTF_FACTOR_INT as f32;

fn bits_to_float(bits: i32) -> f32 {
		if (bits & BTF_FACTOR_INT) != 0 {
				return ((0xFFFFFF & (-(bits))) as f32) / BTF_N_FACTOR;
		} else {
				return (bits as f32) / BTF_P_FACTOR;
		}
}


struct ESParameters {
		gates: [AtomicBool; 8]
}

impl Default for ESParameters {
		fn default() -> ESParameters {
				ESParameters {
						gates: [
								AtomicBool::new(false),
								AtomicBool::new(false),
								AtomicBool::new(false),
								AtomicBool::new(false),
								AtomicBool::new(false),
								AtomicBool::new(false),
								AtomicBool::new(false),
								AtomicBool::new(false),
						],
				}
		}
}


impl PluginParameters for ESParameters {
		fn get_parameter(&self, index: i32) -> f32 {
				if index >=0 && index <= 7 {
						return self.gates[index as usize].load(Ordering::Acquire) as u32 as f32;
				}
				return 0.0;
		}

		fn set_parameter(&self, index: i32, val: f32) {
				if index >=0 && index <= 7 {
						self.gates[index as usize].store(val != 0.0, Ordering::Relaxed);
				}
		}

		fn get_parameter_text(&self, index: i32) -> String {
				if index >=0 && index <= 7 {
						return format!("{}", self.gates[index as usize].load(Ordering::Acquire));
				}
				return "".to_string();
		}

		fn get_parameter_name(&self, index: i32) -> String {
				match index {
						0 => "Gate 1",
						1 => "Gate 2",
						2 => "Gate 3",
						3 => "Gate 4",
						4 => "Gate 5",
						5 => "Gate 6",
						6 => "Gate 7",
						7 => "Gate 8",
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
						inputs: 0, // Two input audio signals.
						outputs: 1,
						parameters: 8,
						..Default::default()
				}
		}

		fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
				let value =
						(self.params.gates[0].load(Ordering::Acquire) as i32) |
				(self.params.gates[1].load(Ordering::Acquire) as i32) << 1 |
				(self.params.gates[2].load(Ordering::Acquire) as i32) << 2 |
				(self.params.gates[3].load(Ordering::Acquire) as i32) << 3 |
				(self.params.gates[4].load(Ordering::Acquire) as i32) << 4 |
				(self.params.gates[5].load(Ordering::Acquire) as i32) << 5 |
				(self.params.gates[6].load(Ordering::Acquire) as i32) << 6 |
				(self.params.gates[7].load(Ordering::Acquire) as i32) << 7;
				
				let samples = buffer.samples();
				let (_, mut outputs) = buffer.split();
				for sample_idx in 0..samples {
						let buff_l = outputs.get_mut(0);
						buff_l[sample_idx] = bits_to_float(value << 16);
				}
		}

		fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
				Arc::clone(&self.params) as Arc<dyn PluginParameters>
		}
}

plugin_main!(ESPlugin);
