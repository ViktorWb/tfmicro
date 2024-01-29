//! Tensorflow Lite Op Resolvers
//!

use core::fmt;

cpp! {{
    #include "tensorflow/lite/micro/micro_mutable_op_resolver.h"
}}

#[cfg(target_pointer_width = "64")]
type InnerMutableOpResolver128 = [u64; 8728 / 8];

#[cfg(target_pointer_width = "32")]
type InnerMutableOpResolver128 = [u32; 4620 / 4];

/// An Op Resolver that has no operators by default, but can be added by
/// calling methods in a builder pattern
pub struct MutableOpResolver {
    pub(crate) inner: InnerMutableOpResolver128,
    capacity: usize,
    len: usize,
}
impl fmt::Debug for MutableOpResolver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("MutableOpResolver (ops = {})", self.len))
    }
}
impl Default for MutableOpResolver {
    fn default() -> Self {
        Self::empty()
    }
}

impl MutableOpResolver {
    pub(crate) fn to_inner(self) -> InnerMutableOpResolver128 {
        self.inner
    }

    /// Check the number of operators is OK
    pub(crate) fn check_then_inc_len(&mut self) {
        assert!(
            self.len < self.capacity,
            "Tensorflow micro does not support more than {} operators.",
            self.capacity
        );

        self.len += 1;
    }

    /// Returns the current number of operators in this resolver
    pub fn len(&self) -> usize {
        self.len
    }

    /// Return whether there are zero operators
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Create a new MutableOpResolver, initially empty
    pub fn empty() -> Self {
        // Maximum number of registrations
        let tflite_registrations_max = 128;

        let micro_op_resolver = unsafe {
            cpp!([] -> InnerMutableOpResolver128 as
                 "tflite::MicroMutableOpResolver<128>" {

                tflite::MicroMutableOpResolver<128> resolver;
                return resolver;
            })
        };

        Self {
            inner: micro_op_resolver,
            capacity: tflite_registrations_max,
            len: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutable_op_resolver() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut ops = MutableOpResolver::empty()
            .add_abs()
            .add_add()
            .add_add_n()
            .add_argmax()
            .add_argmin()
            .add_assign_variable()
            .add_average_pool_2d()
            .add_batch_mat_mul()
            .add_batch_to_space_nd()
            .add_broadcast_args()
            .add_broadcast_to()
            .add_call_once()
            .add_cast()
            .add_ceil()
            .add_concatenation()
            .add_conv_2d()
            .add_cos()
            .add_cum_sum()
            .add_depth_to_space()
            .add_depthwise_conv_2d()
            .add_dequantize()
            .add_div()
            .add_elu()
            .add_embedding_lookup()
            .add_equal()
            .add_ethos_u()
            .add_exp()
            .add_expand_dims()
            .add_fill()
            .add_floor()
            .add_floor_div()
            .add_floor_mod()
            .add_fully_connected()
            .add_gather()
            .add_gather_nd()
            .add_greater()
            .add_greater_equal()
            .add_hard_swish()
            .add_if()
            .add_l2_normalization()
            .add_l2_pool_2d()
            .add_leaky_relu()
            .add_less()
            .add_less_equal()
            .add_log()
            .add_log_softmax()
            .add_maximum()
            .add_max_pool_2d()
            .add_mirror_pad()
            .add_mean()
            .add_minimum()
            .add_mul()
            .add_neg()
            .add_not_equal()
            .add_pack()
            .add_pad_v2()
            .add_prelu()
            .add_quantize()
            .add_read_variable()
            .add_reduce_max()
            .add_relu()
            .add_relu6()
            .add_reshape()
            .add_resize_bilinear()
            .add_resize_nearest_neighbor()
            .add_round()
            .add_rsqrt()
            .add_select_v2()
            .add_shape()
            .add_sin()
            .add_slice()
            .add_softmax()
            .add_space_to_batch_nd()
            .add_space_to_depth()
            .add_split()
            .add_split_v()
            .add_squeeze()
            .add_sqrt()
            .add_square()
            .add_squared_difference()
            .add_strided_slice()
            .add_sub()
            .add_svdf()
            .add_tanh()
            .add_transpose_conv()
            .add_unpack()
            .add_unidirectional_sequence_lstm()
            .add_var_handle()
            .add_zeros_like();

        #[cfg(feature = "cpp-std")]
        {
            ops.add_circular_buffer()
                .add_delay()
                .add_detection_postprocess()
                .add_energy()
                .add_fft_auto_scale()
                .add_filter_bank()
                .add_filter_bank_log()
                .add_filter_bank_spectral_subtraction()
                .add_filter_bank_square_root()
                .add_framer()
                .add_irfft()
                .add_overlap_add()
                .add_pcan()
                .add_rfft()
                .add_stacker()
                .add_window();
        }
    }
}
