const BTF_FACTOR_INT: i32 = 0x800000;
const BTF_N_FACTOR: f32 = -(BTF_FACTOR_INT as f32);
const BTF_P_FACTOR: f32 = BTF_FACTOR_INT as f32;

pub fn bits_to_float(bits: i32) -> f32 {
		if (bits & BTF_FACTOR_INT) != 0 {
				return ((0xFFFFFF & (-(bits))) as f32) / BTF_N_FACTOR;
		} else {
				return (bits as f32) / BTF_P_FACTOR;
		}
}
