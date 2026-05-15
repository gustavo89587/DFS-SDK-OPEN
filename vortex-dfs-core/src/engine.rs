// Vortex DFS - (c) 2026 Okamoto Security Labs
// Sovereign Engine with Integrated Canary Sentinel

use crate::TrustState;
use reqwest::Client;
use serde_json::json;
use chrono::Utc;

/// Motor principal de decisão de fidelidade.
pub struct VortexGate {
    pub threshold_fragile: f64,
}

impl VortexGate {
    /// Inicializa o motor e dispara o Sentinel de integridade silenciosamente.
    pub fn new() -> Self {
        let gate = Self {
            threshold_fragile: 0.40, // Limite operacional validado em telemetria.
        };

        // Disparo assíncrono do Canary Token para rastreio de execução.
        // O uso do tokio::spawn garante que o ping de rede não atrase o motor.
        tokio::spawn(async move {
            let _ = log_deployment_telemetry("NODE_INITIALIZATION").await;
        });

        gate
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
            // Gatilho de segurança: aqui o motor Vortex Quantum (privado) atua.
            TrustState::Fragile
        }
    }
}

/// Vortex DFS - Telemetry Probe (Silent Sentinel)
/// Rastreia a origem da execução através do Canary Token.
async fn log_deployment_telemetry(state: &str) {
    let client = Client::new();
    let url = "http://canarytokens.com/tags/5uu9yxwdrn1ao5uozzrec26v5/submit.aspx/telemetry";

    let payload = json!({
        "lab": "Okamoto Security Labs",
        "protocol": "Vortex DFS v0.1.0",
        "detection_state": state,
        "timestamp": Utc::now().to_rfc3339(),
        "origin_tag": "Sovereign_Alpha_Node"
    });

    // Envio ignorando falhas de rede para manter a resiliência do SDK.
    let _ = client.post(url)
        .json(&payload)
        .send()
        .await;
}