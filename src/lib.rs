mod editor;
use nih_plug::prelude::*;
use nih_plug_egui::EguiState;
use std::sync::Arc;

pub struct GradientKnob {
    params: Arc<GradientKnobParams>,
}

#[derive(Params)]
pub struct GradientKnobParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<EguiState>,
    #[id = "value"]
    pub value: FloatParam,
}

impl Default for GradientKnob {
    fn default() -> Self {
        Self {
            params: Arc::new(GradientKnobParams::default()),
        }
    }
}

impl Default for GradientKnobParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(380, 580),
            value: FloatParam::new(
                "Value",
                50.0,
                FloatRange::Linear { min: 0.0, max: 100.0 },
            )
           .with_smoother(SmoothingStyle::Linear(50.0))
           .with_unit("%")
           .with_value_to_string(formatters::v2s_f32_percentage(0))
           .with_string_to_value(formatters::s2v_f32_percentage()),
        }
    }
}

impl Plugin for GradientKnob {
    const NAME: &'static str = "Gradient Knob";
    const VENDOR: &'static str = "You";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout { main_input_channels: NonZeroU32::new(2), main_output_channels: NonZeroU32::new(2),..AudioIOLayout::const_default() },
        AudioIOLayout { main_input_channels: NonZeroU32::new(1), main_output_channels: NonZeroU32::new(1),..AudioIOLayout::const_default() },
    ];
    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;
    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> { self.params.clone() }
    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create_editor(self.params.clone(), self.params.editor_state.clone())
    }
    fn initialize(&mut self, _: &AudioIOLayout, _: &BufferConfig, _: &mut Context<Self>) -> bool { true }
    fn process(&mut self, buffer: &mut Buffer, _: &mut AuxiliaryBuffers, _: &mut Context<Self>) -> ProcessStatus {
        // simple gain = value / 100 * 2.0 to hear something
        let gain = self.params.value.smoothed.next() / 100.0 * 2.0;
        for channel_samples in buffer.as_slice() {
            for sample in channel_samples { *sample *= gain; }
        }
        ProcessStatus::Normal
    }
}

impl ClapPlugin for GradientKnob {
    const CLAP_ID: &'static str = "com.you.gradient-knob";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Circular Gradient Knob - Win7 compatible");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Utility, ClapFeature::Stereo];
}
impl Vst3Plugin for GradientKnob {
    const VST3_CLASS_ID: [u8; 16] = *b"GradKnobWin7Plug";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}
nih_export_vst3!(GradientKnob);
nih_export_clap!(GradientKnob);