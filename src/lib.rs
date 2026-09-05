#![forbid(unsafe_code)]

pub const SCHEMA_VERSION: &str = "ores.lambda-health.v1";
pub const SUPPORTED_PROVIDERS: &[&str] = &[
    "aws-lambda",
    "gcp-cloud-run",
    "azure-functions",
    "vercel",
    "cloudflare-workers",
    "oci",
    "local",
];

pub const fn health_receipt() -> (&'static str, &'static str) {
    (SCHEMA_VERSION, "ok")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_contract_is_stable() {
        assert_eq!(health_receipt(), ("ores.lambda-health.v1", "ok"));
        assert!(SUPPORTED_PROVIDERS.contains(&"oci"));
        assert!(SUPPORTED_PROVIDERS.contains(&"aws-lambda"));
    }
}
