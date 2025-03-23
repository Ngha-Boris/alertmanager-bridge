use serde::Deserialize;

#[derive(Deserialize)]
pub struct AlertmanagerPayload {
    pub alerts: Vec<Alert>,
}

#[derive(Deserialize)]
pub struct Alert {
    pub labels: Labels,
    pub annotations: Annotations,
    pub status: String,
}

#[derive(Deserialize)]
pub struct Labels {
    pub alertname: String,
}

#[derive(Deserialize)]
pub struct Annotations {
    pub summary: Option<String>,
    pub description: Option<String>,
}
