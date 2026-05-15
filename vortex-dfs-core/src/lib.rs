// Vortex DFS Core - (c) 2026 Okamoto Security Labs
// Sovereign SDK for AI Agent Detection Fidelity Scoring

pub mod engine;

/// Representa o estado de confiança da execução do agente.
/// HighTrust: Integridade total.
/// Operational: Desvio mínimo dentro da normalidade.
/// Fragile: Risco de sequestro ou alucinação (Gatilho para o Vortex Quantum).
#[derive(Debug, PartialEq)]
pub enum TrustState {
    HighTrust,
    Operational,
    Fragile,
}

// Bloco de Testes Unitários
#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::VortexGate;

    #[test]
    fn test_vortex_fragile_gate() {
        let gate = VortexGate::new();
        
        // Simulação de Drift Crítico: 
        // Distância de execução (0.8) * Entropia de rede (0.6) = 0.48
        // O threshold default é 0.40, logo deve retornar Fragile.
        let state = gate.evaluate_fidelity(0.8, 0.6);
        
        println!("\n[OKAMOTO LABS] Status de Fidelidade: {:?}", state);
        assert_eq!(state, TrustState::Fragile);
    }

    #[test]
    fn test_vortex_safe_gate() {
        let gate = VortexGate::new();
        
        // Simulação de Fluxo Seguro:
        // Drift baixo (0.1 * 0.1 = 0.01)
        let state = gate.evaluate_fidelity(0.1, 0.1);
        
        assert_eq!(state, TrustState::HighTrust);
    }
}