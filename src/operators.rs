/// Operators for Tensorflow micro
///
/// See lite/micro/kernels/all_ops_resolver.cc
use crate::micro_op_resolver::MutableOpResolver;

impl MutableOpResolver {
    /// Use the Abs operator in this op resolver
    pub fn add_abs(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddAbs();
        });

        self
    }

    /// Use the Add operator in this op resolver
    pub fn add_add(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddAdd();
        });

        self
    }

    /// Use the AddN operator in this op resolver
    pub fn add_add_n(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddAddN();
        });

        self
    }

    /// Use the ArgMax operator in this op resolver
    pub fn add_argmax(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddArgMax();
        });

        self
    }

    /// Use the ArgMin operator in this op resolver
    pub fn add_argmin(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddArgMin();
        });

        self
    }

    /// Use the AssignVariable operator in this op resolver
    pub fn add_assign_variable(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddAssignVariable();
        });

        self
    }

    /// Use the AveragePool2D operator in this op resolver
    pub fn add_average_pool_2d(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddAveragePool2D();
        });

        self
    }

    /// Use the BatchMatMul operator in this op resolver
    pub fn add_batch_mat_mul(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBatchMatMul();
        });

        self
    }

    /// Use the BatchToSpaceNd operator in this op resolver
    pub fn add_batch_to_space_nd(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBatchToSpaceNd();
        });

        self
    }

    /// Use the BroadcastArgs operator in this op resolver
    pub fn add_broadcast_args(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBroadcastArgs();
        });

        self
    }

    /// Use the BroadcastTo operator in this op resolver
    pub fn add_broadcast_to(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddBroadcastTo();
        });

        self
    }

    /// Use the CallOnce operator in this op resolver
    pub fn add_call_once(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddCallOnce();
        });

        self
    }

    /// Use the Cast operator in this op resolver
    pub fn add_cast(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddCast();
        });

        self
    }

    /// Use the Ceil operator in this op resolver
    pub fn add_ceil(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddCeil();
        });

        self
    }

    /// Use the CircularBuffer operator in this op resolver
    pub fn add_circular_buffer(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddCircularBuffer();
        });

        self
    }

    /// Use the Concatenation operator in this op resolver
    pub fn add_concatenation(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddConcatenation();
        });

        self
    }

    /// Use the Conv2D operator in this op resolver
    pub fn add_conv_2d(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddConv2D();
        });

        self
    }

    /// Use the Cos operator in this op resolver
    pub fn add_cos(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddCos();
        });

        self
    }

    /// Use the CumSum operator in this op resolver
    pub fn add_cum_sum(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddCumSum();
        });

        self
    }

    /// Use the Delay operator in this op resolver
    pub fn add_delay(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddDelay();
        });

        self
    }

    /// Use the DepthToSpace operator in this op resolver
    pub fn add_depth_to_space(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddDepthToSpace();
        });

        self
    }

    /// Use the DepthwiseConv2D operator in this op resolver
    pub fn add_depthwise_conv_2d(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddDepthwiseConv2D();
        });

        self
    }

    /// Use the Dequantize operator in this op resolver
    pub fn add_dequantize(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddDequantize();
        });

        self
    }

    /// Use the DetectionPostprocess operator in this op resolver
    pub fn add_detection_postprocess(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddDetectionPostprocess();
        });

        self
    }

    /// Use the Div operator in this op resolver
    pub fn add_div(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddDiv();
        });

        self
    }

    /// Use the EmbeddingLookup operator in this op resolver
    pub fn add_embedding_lookup(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddEmbeddingLookup();
        });

        self
    }

    /// Use the Energy operator in this op resolver
    pub fn add_energy(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddEnergy();
        });

        self
    }

    /// Use the Elu operator in this op resolver
    pub fn add_elu(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddElu();
        });

        self
    }

    /// Use the Equal operator in this op resolver
    pub fn add_equal(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddEqual();
        });

        self
    }

    /// Use the EthosU operator in this op resolver
    pub fn add_ethos_u(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddEthosU();
        });

        self
    }

    /// Use the Exp operator in this op resolver
    pub fn add_exp(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddExp();
        });

        self
    }

    /// Use the ExpandDims operator in this op resolver
    pub fn add_expand_dims(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddExpandDims();
        });

        self
    }

    /// Use the FftAutoScale operator in this op resolver
    pub fn add_fft_auto_scale(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddFftAutoScale();
        });

        self
    }

    /// Use the Fill operator in this op resolver
    pub fn add_fill(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddFill();
        });

        self
    }

    /// Use the FilterBank operator in this op resolver
    pub fn add_filter_bank(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddFilterBank();
        });

        self
    }

    /// Use the FilterBankLog operator in this op resolver
    pub fn add_filter_bank_log(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddFilterBankLog();
        });

        self
    }

    /// Use the FilterBankSquareRoot operator in this op resolver
    pub fn add_filter_bank_square_root(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddFilterBankSquareRoot();
        });

        self
    }

    /// Use the FilterBankSpectralSubtraction operator in this op resolver
    pub fn add_filter_bank_spectral_subtraction(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddFilterBankSpectralSubtraction();
        });

        self
    }

    /// Use the Floor operator in this op resolver
    pub fn add_floor(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddFloor();
        });

        self
    }

    /// Use the FloorDiv operator in this op resolver
    pub fn add_floor_div(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddFloorDiv();
        });

        self
    }

    /// Use the FloorMod operator in this op resolver
    pub fn add_floor_mod(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddFloorMod();
        });

        self
    }

    /// Use the Framer operator in this op resolver
    pub fn add_framer(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddFramer();
        });

        self
    }

    /// Use the FullyConnected operator in this op resolver
    pub fn add_fully_connected(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddFullyConnected();
        });

        self
    }

    /// Use the Gather operator in this op resolver
    pub fn add_gather(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddGather();
        });

        self
    }

    /// Use the GatherNd operator in this op resolver
    pub fn add_gather_nd(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddGatherNd();
        });

        self
    }

    /// Use the Greater operator in this op resolver
    pub fn add_greater(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddGreater();
        });

        self
    }

    /// Use the GreaterEqual operator in this op resolver
    pub fn add_greater_equal(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddGreaterEqual();
        });

        self
    }

    /// Use the HardSwish operator in this op resolver
    pub fn add_hard_swish(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddHardSwish();
        });

        self
    }

    /// Use the If operator in this op resolver
    pub fn add_if(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddIf();
        });

        self
    }

    /// Use the Irfft operator in this op resolver
    pub fn add_irfft(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddIrfft();
        });

        self
    }

    /// Use the L2Normalization operator in this op resolver
    pub fn add_l2_normalization(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddL2Normalization();
        });

        self
    }

    /// Use the L2Pool2D operator in this op resolver
    pub fn add_l2_pool_2d(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddL2Pool2D();
        });

        self
    }

    /// Use the LeakyRelu operator in this op resolver
    pub fn add_leaky_relu(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddLeakyRelu();
        });

        self
    }

    /// Use the Less operator in this op resolver
    pub fn add_less(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddLess();
        });

        self
    }

    /// Use the LessEqual operator in this op resolver
    pub fn add_less_equal(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddLessEqual();
        });

        self
    }

    /// Use the Log operator in this op resolver
    pub fn add_log(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddLog();
        });

        self
    }

    /// Use the LogicalAnd operator in this op resolver
    pub fn add_logical_and(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddLogicalAnd();
        });

        self
    }

    /// Use the LogicalNot operator in this op resolver
    pub fn add_logical_not(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddLogicalNot();
        });

        self
    }

    /// Use the LogicalOr operator in this op resolver
    pub fn add_logical_or(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddLogicalOr();
        });

        self
    }

    /// Use the Logistic operator in this op resolver
    pub fn add_logistic(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddLogistic();
        });

        self
    }

    /// Use the LogSoftmax operator in this op resolver
    pub fn add_log_softmax(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddLogSoftmax();
        });

        self
    }

    /// Use the Maximum operator in this op resolver
    pub fn add_maximum(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddMaximum();
        });

        self
    }

    /// Use the MaxPool2D operator in this op resolver
    pub fn add_max_pool_2d(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddMaxPool2D();
        });

        self
    }

    /// Use the MirrorPad operator in this op resolver
    pub fn add_mirror_pad(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddMirrorPad();
        });

        self
    }

    /// Use the Mean operator in this op resolver
    pub fn add_mean(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddMean();
        });

        self
    }

    /// Use the Minimum operator in this op resolver
    pub fn add_minimum(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddMinimum();
        });

        self
    }

    /// Use the Mul operator in this op resolver
    pub fn add_mul(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddMul();
        });

        self
    }

    /// Use the Neg operator in this op resolver
    pub fn add_neg(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddNeg();
        });

        self
    }

    /// Use the NotEqual operator in this op resolver
    pub fn add_not_equal(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddNotEqual();
        });

        self
    }

    /// Use the OverlapAdd operator in this op resolver
    pub fn add_overlap_add(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddOverlapAdd();
        });

        self
    }

    /// Use the Pack operator in this op resolver
    pub fn add_pack(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddPack();
        });

        self
    }

    /// Use the PadV2 operator in this op resolver
    pub fn add_pad_v2(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddPadV2();
        });

        self
    }

    /// Use the PCAN operator in this op resolver
    pub fn add_pcan(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddPCAN();
        });

        self
    }

    /// Use the Prelu operator in this op resolver
    pub fn add_prelu(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddPrelu();
        });

        self
    }

    /// Use the Quantize operator in this op resolver
    pub fn add_quantize(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddQuantize();
        });

        self
    }

    /// Use the ReadVariable operator in this op resolver
    pub fn add_read_variable(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddReadVariable();
        });

        self
    }

    /// Use the ReduceMax operator in this op resolver
    pub fn add_reduce_max(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddReduceMax();
        });

        self
    }

    /// Use the Relu operator in this op resolver
    pub fn add_relu(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddRelu();
        });

        self
    }

    /// Use the Relu6 operator in this op resolver
    pub fn add_relu6(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddRelu6();
        });

        self
    }

    /// Use the Reshape operator in this op resolver
    pub fn add_reshape(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddReshape();
        });

        self
    }

    /// Use the ResizeBilinear operator in this op resolver
    pub fn add_resize_bilinear(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddResizeBilinear();
        });

        self
    }

    /// Use the ResizeNearestNeighbor operator in this op resolver
    pub fn add_resize_nearest_neighbor(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddResizeNearestNeighbor();
        });

        self
    }

    /// Use the Rfft operator in this op resolver
    pub fn add_rfft(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddRfft();
        });

        self
    }

    /// Use the Round operator in this op resolver
    pub fn add_round(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddRound();
        });

        self
    }

    /// Use the Rsqrt operator in this op resolver
    pub fn add_rsqrt(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddRsqrt();
        });

        self
    }

    /// Use the SelectV2 operator in this op resolver
    pub fn add_select_v2(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddSelectV2();
        });

        self
    }

    /// Use the Shape operator in this op resolver
    pub fn add_shape(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddShape();
        });

        self
    }

    /// Use the Sin operator in this op resolver
    pub fn add_sin(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddSin();
        });

        self
    }

    /// Use the Slice operator in this op resolver
    pub fn add_slice(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddSlice();
        });

        self
    }

    /// Use the Softmax operator in this op resolver
    pub fn add_softmax(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddSoftmax();
        });

        self
    }

    /// Use the SpaceToBatchNd operator in this op resolver
    pub fn add_space_to_batch_nd(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddSpaceToBatchNd();
        });

        self
    }

    /// Use the SpaceToDepth operator in this op resolver
    pub fn add_space_to_depth(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddSpaceToDepth();
        });

        self
    }

    /// Use the Split operator in this op resolver
    pub fn add_split(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddSplit();
        });

        self
    }

    /// Use the SplitV operator in this op resolver
    pub fn add_split_v(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddSplitV();
        });

        self
    }

    /// Use the Squeeze operator in this op resolver
    pub fn add_squeeze(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddSqueeze();
        });

        self
    }

    /// Use the Sqrt operator in this op resolver
    pub fn add_sqrt(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddSqrt();
        });

        self
    }

    /// Use the Square operator in this op resolver
    pub fn add_square(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddSquare();
        });

        self
    }

    /// Use the SquaredDifference operator in this op resolver
    pub fn add_squared_difference(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddSquaredDifference();
        });

        self
    }

    /// Use the StridedSlice operator in this op resolver
    pub fn add_strided_slice(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddStridedSlice();
        });

        self
    }

    /// Use the Stacker operator in this op resolver
    pub fn add_stacker(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddStacker();
        });

        self
    }

    /// Use the Sub operator in this op resolver
    pub fn add_sub(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddSub();
        });

        self
    }

    /// Use the Sum operator in this op resolver
    pub fn add_sum(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddSum();
        });

        self
    }

    /// Use the Svdf operator in this op resolver
    pub fn add_svdf(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddSvdf();
        });

        self
    }

    /// Use the Tanh operator in this op resolver
    pub fn add_tanh(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddTanh();
        });

        self
    }

    /// Use the TransposeConv operator in this op resolver
    pub fn add_transpose_conv(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddTransposeConv();
        });

        self
    }

    /// Use the Unpack operator in this op resolver
    pub fn add_unpack(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddUnpack();
        });

        self
    }

    /// Use the UnidirectionalSquenceLSTM operator in this op resolver
    pub fn add_unidirectional_sequence_lstm(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddUnidirectionalSequenceLSTM();
        });

        self
    }

    /// Use the VarHandle operator in this op resolver
    pub fn add_var_handle(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddVarHandle();
        });

        self
    }

    /// Use the While operator in this op resolver
    pub fn add_while(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddWhile();
        });

        self
    }

    /// Use the Window operator in this op resolver
    pub fn add_window(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddWindow();
        });

        self
    }

    /// Use the ZerosLike operator in this op resolver
    pub fn add_zeros_like(mut self) -> Self {
        self.check_then_inc_len();
        let inner_ref = &mut self.inner;

        cpp!(unsafe [inner_ref as "tflite::MicroMutableOpResolver<128>*"] {
            inner_ref->AddZerosLike();
        });

        self
    }
}
