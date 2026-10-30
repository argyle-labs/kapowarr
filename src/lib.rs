//! Kapowarr's `ServiceBackend` — deploy/backup/restore of the Kapowarr
//! instance itself. Kapowarr is not Servarr-shaped (its own API, not the
//! shared *arr `X-Api-Key` client), so this plugin carries no typed API
//! client and no `#[orca_tool]` surface — service-backend only.

use plugin_toolkit::service::{
    BoxFuture, Routes, Runtime, ServiceBackend, ServiceCapability, ServiceError, ServiceStatus,
    WorkloadSpec,
};

#[derive(Debug, Clone)]
pub struct KapowarrServiceBackend {
    provider: &'static str,
}

impl KapowarrServiceBackend {
    pub fn new(provider: &'static str) -> Self {
        Self { provider }
    }
}

impl ServiceBackend for KapowarrServiceBackend {
    fn provider(&self) -> &str {
        self.provider
    }

    fn runtimes(&self) -> Vec<Runtime> {
        vec![Runtime::Docker, Runtime::Podman, Runtime::Lxc, Runtime::Vm]
    }

    fn capabilities(&self) -> Vec<ServiceCapability> {
        vec![
            ServiceCapability::Deploy,
            ServiceCapability::Backup,
            ServiceCapability::Restore,
            ServiceCapability::Configure,
            ServiceCapability::Status,
        ]
    }

    fn default_port(&self) -> u16 {
        5656
    }

    /// `/app/db` is the only persistent state; `/downloads` and
    /// `/data/media/*` are bulk/operational mounts excluded from backup.
    fn data_paths(&self) -> Vec<String> {
        vec!["/app/db".to_string()]
    }

    fn workload_spec<'a>(
        &'a self,
        _runtime: Runtime,
        _instance: &'a str,
        _routes: &'a Routes,
    ) -> BoxFuture<'a, Result<WorkloadSpec, ServiceError>> {
        Box::pin(async move { Err(ServiceError::unimplemented("kapowarr.workload_spec")) })
    }

    fn configure<'a>(
        &'a self,
        _instance: &'a str,
        _routes: &'a Routes,
        _config: &'a str,
    ) -> BoxFuture<'a, Result<(), ServiceError>> {
        Box::pin(async move { Err(ServiceError::unimplemented("kapowarr.configure")) })
    }

    fn status<'a>(
        &'a self,
        _instance: &'a str,
        _routes: &'a Routes,
    ) -> BoxFuture<'a, Result<ServiceStatus, ServiceError>> {
        Box::pin(async move { Err(ServiceError::unimplemented("kapowarr.status")) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_backend_declares_config_only() {
        let k = KapowarrServiceBackend::new("kapowarr");
        assert_eq!(k.provider(), "kapowarr");
        assert_eq!(k.default_port(), 5656);
        assert_eq!(k.data_paths(), vec!["/app/db".to_string()]);
    }
}
