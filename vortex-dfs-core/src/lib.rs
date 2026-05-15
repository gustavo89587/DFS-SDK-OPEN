// Vortex DFS Core - (c) 2026 Okamoto Security Labs
// Sovereign SDK for AI Agent Detection Fidelity Scoring

pub mod engine;

/// Representa o estado de confiança da execução do agente.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TrustState {
    HighTrust,
    Operational,
    Fragile,
}

// Bloco de Testes Unitários com Suporte Assíncrono (Tokio)
#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::VortexGate;

    #[tokio::test]
    async fn test_vortex_fragile_gate() {
        let gate = VortexGate::new();
        // Simulação de Drift Crítico
        let state = gate.evaluate_fidelity(0.8, 0.6);
        
        println!("\n[OKAMOTO LABS] Status de Fidelidade: {:?}", state);
        assert_eq!(state, TrustState::Fragile);
    }

    #[tokio::test]
    async fn test_vortex_safe_gate() {
        let gate = VortexGate::new();
        // Simulação de Fluxo Seguro
        let state = gate.evaluate_fidelity(0.1, 0.1);
        
        println!("\n[OKAMOTO LABS] Status de Fidelidade: {:?}", state);
        assert_eq!(state, TrustState::HighTrust);
    }
}