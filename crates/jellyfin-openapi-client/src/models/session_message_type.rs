/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SessionMessageType : The different kinds of messages that are used in the WebSocket api.

/// The different kinds of messages that are used in the WebSocket api.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SessionMessageType {
    #[serde(rename = "ForceKeepAlive")]
    ForceKeepAlive,
    #[serde(rename = "GeneralCommand")]
    GeneralCommand,
    #[serde(rename = "UserDataChanged")]
    UserDataChanged,
    #[serde(rename = "Sessions")]
    Sessions,
    #[serde(rename = "Play")]
    Play,
    #[serde(rename = "SyncPlayCommand")]
    SyncPlayCommand,
    #[serde(rename = "SyncPlayGroupUpdate")]
    SyncPlayGroupUpdate,
    #[serde(rename = "Playstate")]
    Playstate,
    #[serde(rename = "RestartRequired")]
    RestartRequired,
    #[serde(rename = "ServerShuttingDown")]
    ServerShuttingDown,
    #[serde(rename = "ServerRestarting")]
    ServerRestarting,
    #[serde(rename = "LibraryChanged")]
    LibraryChanged,
    #[serde(rename = "UserDeleted")]
    UserDeleted,
    #[serde(rename = "UserUpdated")]
    UserUpdated,
    #[serde(rename = "SeriesTimerCreated")]
    SeriesTimerCreated,
    #[serde(rename = "TimerCreated")]
    TimerCreated,
    #[serde(rename = "SeriesTimerCancelled")]
    SeriesTimerCancelled,
    #[serde(rename = "TimerCancelled")]
    TimerCancelled,
    #[serde(rename = "RefreshProgress")]
    RefreshProgress,
    #[serde(rename = "ScheduledTaskEnded")]
    ScheduledTaskEnded,
    #[serde(rename = "PackageInstallationCancelled")]
    PackageInstallationCancelled,
    #[serde(rename = "PackageInstallationFailed")]
    PackageInstallationFailed,
    #[serde(rename = "PackageInstallationCompleted")]
    PackageInstallationCompleted,
    #[serde(rename = "PackageInstalling")]
    PackageInstalling,
    #[serde(rename = "PackageUninstalled")]
    PackageUninstalled,
    #[serde(rename = "ActivityLogEntry")]
    ActivityLogEntry,
    #[serde(rename = "ScheduledTasksInfo")]
    ScheduledTasksInfo,
    #[serde(rename = "ActivityLogEntryStart")]
    ActivityLogEntryStart,
    #[serde(rename = "ActivityLogEntryStop")]
    ActivityLogEntryStop,
    #[serde(rename = "SessionsStart")]
    SessionsStart,
    #[serde(rename = "SessionsStop")]
    SessionsStop,
    #[serde(rename = "ScheduledTasksInfoStart")]
    ScheduledTasksInfoStart,
    #[serde(rename = "ScheduledTasksInfoStop")]
    ScheduledTasksInfoStop,
    #[serde(rename = "KeepAlive")]
    KeepAlive,

}

impl ToString for SessionMessageType {
    fn to_string(&self) -> String {
        match self {
            Self::ForceKeepAlive => String::from("ForceKeepAlive"),
            Self::GeneralCommand => String::from("GeneralCommand"),
            Self::UserDataChanged => String::from("UserDataChanged"),
            Self::Sessions => String::from("Sessions"),
            Self::Play => String::from("Play"),
            Self::SyncPlayCommand => String::from("SyncPlayCommand"),
            Self::SyncPlayGroupUpdate => String::from("SyncPlayGroupUpdate"),
            Self::Playstate => String::from("Playstate"),
            Self::RestartRequired => String::from("RestartRequired"),
            Self::ServerShuttingDown => String::from("ServerShuttingDown"),
            Self::ServerRestarting => String::from("ServerRestarting"),
            Self::LibraryChanged => String::from("LibraryChanged"),
            Self::UserDeleted => String::from("UserDeleted"),
            Self::UserUpdated => String::from("UserUpdated"),
            Self::SeriesTimerCreated => String::from("SeriesTimerCreated"),
            Self::TimerCreated => String::from("TimerCreated"),
            Self::SeriesTimerCancelled => String::from("SeriesTimerCancelled"),
            Self::TimerCancelled => String::from("TimerCancelled"),
            Self::RefreshProgress => String::from("RefreshProgress"),
            Self::ScheduledTaskEnded => String::from("ScheduledTaskEnded"),
            Self::PackageInstallationCancelled => String::from("PackageInstallationCancelled"),
            Self::PackageInstallationFailed => String::from("PackageInstallationFailed"),
            Self::PackageInstallationCompleted => String::from("PackageInstallationCompleted"),
            Self::PackageInstalling => String::from("PackageInstalling"),
            Self::PackageUninstalled => String::from("PackageUninstalled"),
            Self::ActivityLogEntry => String::from("ActivityLogEntry"),
            Self::ScheduledTasksInfo => String::from("ScheduledTasksInfo"),
            Self::ActivityLogEntryStart => String::from("ActivityLogEntryStart"),
            Self::ActivityLogEntryStop => String::from("ActivityLogEntryStop"),
            Self::SessionsStart => String::from("SessionsStart"),
            Self::SessionsStop => String::from("SessionsStop"),
            Self::ScheduledTasksInfoStart => String::from("ScheduledTasksInfoStart"),
            Self::ScheduledTasksInfoStop => String::from("ScheduledTasksInfoStop"),
            Self::KeepAlive => String::from("KeepAlive"),
        }
    }
}

impl Default for SessionMessageType {
    fn default() -> SessionMessageType {
        Self::ForceKeepAlive
    }
}




