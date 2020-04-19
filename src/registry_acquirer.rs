use std::path::Path;
use crate::remote::{Computer, Connector, Command, PsExec, PsRemote, Rdp, Wmi, CompressCopier, RemoteFileCopier, Compression};
use crate::process_runner::create_report_path;
use std::thread;
use std::time::Duration;
use crate::utils::remote_storage;
use crate::large_evidence_acquirer::LargeEvidenceAcquirer;

pub struct RegistryAcquirer<'a> {
    store_directory: &'a Path,
    connector: Box<dyn Connector>,

    registry_hklm_command: Vec<String>,
    registry_hkcu_command: Vec<String>,
    registry_hkcr_command: Vec<String>,
    registry_hku_command: Vec<String>,
    registry_hkcc_command: Vec<String>,

    compress_timeout: Option<Duration>,
    compression: Compression,
}

impl<'a> RegistryAcquirer<'a> {
    pub fn new(
        store_directory: &'a Path,
        connector: Box<dyn Connector>,
        compress_timeout: Option<Duration>,
        compression: Compression,
    ) -> RegistryAcquirer<'a> {
        RegistryAcquirer {
            store_directory,
            connector,
            registry_hklm_command: vec![
                "reg".to_string(),
                "export".to_string(),
                "HKLM".to_string(),
            ],
            registry_hkcu_command: vec![
                "reg".to_string(),
                "export".to_string(),
                "HKCU".to_string(),
            ],
            registry_hkcr_command: vec![
                "reg".to_string(),
                "export".to_string(),
                "HKCR".to_string(),
            ],
            registry_hku_command: vec![
                "reg".to_string(),
                "export".to_string(),
                "HKU".to_string(),
            ],
            registry_hkcc_command: vec![
                "reg".to_string(),
                "export".to_string(),
                "HKCC".to_string(),
            ],
            compress_timeout,
            compression,
        }
    }

    pub fn psexec(
        store_directory: &'a Path,
        computer: Computer,
        no_7zip: bool,
    ) -> RegistryAcquirer {
        RegistryAcquirer::new(
            store_directory,
            Box::new(PsExec::psexec(computer)),
            None,
            if no_7zip { Compression::No } else { Compression::Yes },
        )
    }

    pub fn psremote(
        store_directory: &'a Path,
        computer: Computer,
        _no_7zip: bool,
    ) -> RegistryAcquirer {
        RegistryAcquirer::new(
            store_directory,
            Box::new(PsRemote::new(computer)),
            None,
            Compression::No,
        )
    }

    pub fn wmi(
        store_directory: &'a Path,
        computer: Computer,
        compress_timeout: Duration,
        no_7zip: bool,
    ) -> RegistryAcquirer {
        RegistryAcquirer::new(
            store_directory,
            Box::new(Wmi { computer }),
            Some(compress_timeout),
            if no_7zip { Compression::No } else { Compression::YesSplit }
        )
    }

    pub fn rdp(
        store_directory: &'a Path,
        computer: Computer,
        compress_timeout: Duration,
        nla: bool,
        no_7zip: bool,
    ) -> RegistryAcquirer {
        RegistryAcquirer::new(
            store_directory,
            Box::new(Rdp { computer, nla }),
            Some(compress_timeout),
            if no_7zip { Compression::No } else { Compression::YesSplit }
        )
    }

    pub fn acquire(&self) {
        let command = &self.registry_hklm_command;
        let lea = LargeEvidenceAcquirer{
            store_directory: self.store_directory,
            connector: self.connector.as_ref(),
            compress_timeout: self.compress_timeout,
            compression: self.compression,
            report_extension: "txt",
            overwrite_switch: Some("/y")
        };
        lea.run(
            command,
            "registry-hklm",
        );
        let command = &self.registry_hku_command;
        lea.run(
            command,
            "registry-hku",
        );
        let command = &self.registry_hkcu_command;
        lea.run(
            command,
            "registry-hkcu",
        );
        let command = &self.registry_hkcr_command;
        lea.run(
            command,
            "registry-hkcr",
        );
        let command = &self.registry_hkcc_command;
        lea.run(
            command,
            "registry-hkcc",
        );
    }
}