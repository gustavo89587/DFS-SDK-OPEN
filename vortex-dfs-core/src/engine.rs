use crate::TrustState;

pub struct VortexGate {
    pub threshold_fragile: f64,
}

impl VortexGate {
    pub fn new() -> Self {
        Self {
            threshold_fragile: 0.40, // O limite que vimos na telemetria.
        }
    }

    /// Avalia a integridade da instrução baseada no drift de entropia.
    /// Esta é a base para o licenciamento do motor de bloqueio em 0.0031ms.
    pub fn evaluate_fidelity(&self, distance: f64, entropy: f64) -> TrustState {
        let drift = distance * entropy;

        if drift < 0.15 {
            TrustState::HighTrust
        } else if drift < self.threshold_fragile {
            TrustState::Operational
        } else {
            // Se chegar aqui, o motor Vortex Quantum (privado) entraria com o bloqueio.
            TrustState::Fragile
        }
    }
}