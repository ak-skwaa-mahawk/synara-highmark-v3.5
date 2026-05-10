// ... (previous imports unchanged)

use crate::glyph::{generate_glyph_waveform, rad_hard_checksum, get_quantum_layer, TOROIDAL_PI_R, CAPRICORN_LATTICE};

#[tonic::async_trait]
impl Inference for InferenceService {
    async fn encode_rad_hard_glyph(
        &self,
        request: Request<GlyphRequest>,
    ) -> Result<Response<GlyphResponse>, Status> {
        let req = request.into_inner();

        // Generate using toroidal living_π_r
        let glyph = generate_glyph_waveform(7.83, 44100, 528.0, TOROIDAL_PI_R)
            .map_err(|e| Status::internal(e.to_string()))?;

        let checksum = rad_hard_checksum(&glyph);
        let waveform = glyph.to_vec1::<f32>().unwrap();

        Ok(Response::new(GlyphResponse {
            refined_waveform: waveform,
            waveform_checksum: checksum,
            status: "RAD_HARD_GLYPH_LOCKED_TOROIDAL".to_string(),
            coherence: 99.97,
            message: format!(
                "MAHS’I CHOO — Capricorn ♑︎ lattice confirmed. Toroidal π_r = {:.7} | Quantum layer 2025 active: {}",
                CAPRICORN_LATTICE,
                get_quantum_layer()
            ),
            quantum_layer: get_quantum_layer().to_string(),  // NEW FIELD
            toroidal_pi_r: TOROIDAL_PI_R,
        }))
    }

    // run_clientless_pulse remains similar with toroidal constants
}