use std::path::Path;
use crate::remote::{Computer, Connector, Command, PsExec, PsRemote, Rdp, Wmi, CompressCopier, RemoteFileCopier, Compression};
use crate::process_runner::create_report_path;
use std::thread;
use std::time::Duration;
use crate::utils::{remote_storage, remote_storage_file};
use crate::large_evidence_acquirer::LargeEvidenceAcquirer;

pub struct EventsAcquirer<'a> {
    store_directory: &'a Path,
    connector: Box<dyn Connector>,

    application_event_logs_command: Vec<String>,
    system_event_logs_command: Vec<String>,

    compress_timeout: Option<Duration>,
    compression: Compression,
}

impl<'a> EventsAcquirer<'a> {
    pub fn new(
        store_directory: &'a Path,
        connector: Box<dyn Connector>,
        compress_timeout: Option<Duration>,
        compression: Compression,
    ) -> EventsAcquirer<'a> {
        EventsAcquirer {
            store_directory,
            connector,
            application_event_logs_command: vec![
                "wevtutil".to_string(),
                "epl".to_string(),
                "application".to_string(),
            ],
            system_event_logs_command: vec![
                "wevtutil".to_string(),
                "epl".to_string(),
                "system".to_string(),
            ],
            compress_timeout,
            compression,
        }
    }

    pub fn psexec(
        store_directory: &'a Path,
        computer: Computer,
        no_7zip: bool,
    ) -> EventsAcquirer {
        EventsAcquirer::new(
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
    ) -> EventsAcquirer {
        EventsAcquirer::new(
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
    ) -> EventsAcquirer {
        EventsAcquirer::new(
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
    ) -> EventsAcquirer {
        EventsAcquirer::new(
            store_directory,
            Box::new(Rdp { computer, nla }),
            Some(compress_timeout),
            if no_7zip { Compression::No } else { Compression::YesSplit }
        )
    }

    pub fn acquire(&self) {
        let lea = LargeEvidenceAcquirer{
            store_directory: self.store_directory,
            connector: self.connector.as_ref(),
            compress_timeout: self.compress_timeout,
            compression: self.compression,
            report_extension: "evtx",
            overwrite_switch: Some("/ow:true")
        };
        let command = &self.system_event_logs_command;
        lea.run(
            command,
            "events-system"
        );
        let command = &self.application_event_logs_command;
        lea.run(
            command,
            "events-application"
        );
    }
}