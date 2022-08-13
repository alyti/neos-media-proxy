/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateConfigurationRequest : Represents the server configuration.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateConfigurationRequest {
    /// Gets or sets the number of days we should retain log files.
    #[serde(rename = "LogFileRetentionDays", skip_serializing_if = "Option::is_none")]
    pub log_file_retention_days: Option<i32>,
    /// Gets or sets a value indicating whether this instance is first run.
    #[serde(rename = "IsStartupWizardCompleted", skip_serializing_if = "Option::is_none")]
    pub is_startup_wizard_completed: Option<bool>,
    /// Gets or sets the cache path.
    #[serde(rename = "CachePath", skip_serializing_if = "Option::is_none")]
    pub cache_path: Option<String>,
    /// Gets or sets the last known version that was ran using the configuration.
    #[serde(rename = "PreviousVersion", skip_serializing_if = "Option::is_none")]
    pub previous_version: Option<String>,
    /// Gets or sets the stringified PreviousVersion to be stored/loaded,  because System.Version itself isn't xml-serializable.
    #[serde(rename = "PreviousVersionStr", skip_serializing_if = "Option::is_none")]
    pub previous_version_str: Option<String>,
    /// Gets or sets a value indicating whether to enable prometheus metrics exporting.
    #[serde(rename = "EnableMetrics", skip_serializing_if = "Option::is_none")]
    pub enable_metrics: Option<bool>,
    #[serde(rename = "EnableNormalizedItemByNameIds", skip_serializing_if = "Option::is_none")]
    pub enable_normalized_item_by_name_ids: Option<bool>,
    /// Gets or sets a value indicating whether this instance is port authorized.
    #[serde(rename = "IsPortAuthorized", skip_serializing_if = "Option::is_none")]
    pub is_port_authorized: Option<bool>,
    /// Gets or sets a value indicating whether quick connect is available for use on this server.
    #[serde(rename = "QuickConnectAvailable", skip_serializing_if = "Option::is_none")]
    pub quick_connect_available: Option<bool>,
    /// Gets or sets a value indicating whether [enable case sensitive item ids].
    #[serde(rename = "EnableCaseSensitiveItemIds", skip_serializing_if = "Option::is_none")]
    pub enable_case_sensitive_item_ids: Option<bool>,
    #[serde(rename = "DisableLiveTvChannelUserDataName", skip_serializing_if = "Option::is_none")]
    pub disable_live_tv_channel_user_data_name: Option<bool>,
    /// Gets or sets the metadata path.
    #[serde(rename = "MetadataPath", skip_serializing_if = "Option::is_none")]
    pub metadata_path: Option<String>,
    #[serde(rename = "MetadataNetworkPath", skip_serializing_if = "Option::is_none")]
    pub metadata_network_path: Option<String>,
    /// Gets or sets the preferred metadata language.
    #[serde(rename = "PreferredMetadataLanguage", skip_serializing_if = "Option::is_none")]
    pub preferred_metadata_language: Option<String>,
    /// Gets or sets the metadata country code.
    #[serde(rename = "MetadataCountryCode", skip_serializing_if = "Option::is_none")]
    pub metadata_country_code: Option<String>,
    /// Gets or sets characters to be replaced with a ' ' in strings to create a sort name.
    #[serde(rename = "SortReplaceCharacters", skip_serializing_if = "Option::is_none")]
    pub sort_replace_characters: Option<Vec<String>>,
    /// Gets or sets characters to be removed from strings to create a sort name.
    #[serde(rename = "SortRemoveCharacters", skip_serializing_if = "Option::is_none")]
    pub sort_remove_characters: Option<Vec<String>>,
    /// Gets or sets words to be removed from strings to create a sort name.
    #[serde(rename = "SortRemoveWords", skip_serializing_if = "Option::is_none")]
    pub sort_remove_words: Option<Vec<String>>,
    /// Gets or sets the minimum percentage of an item that must be played in order for playstate to be updated.
    #[serde(rename = "MinResumePct", skip_serializing_if = "Option::is_none")]
    pub min_resume_pct: Option<i32>,
    /// Gets or sets the maximum percentage of an item that can be played while still saving playstate. If this percentage is crossed playstate will be reset to the beginning and the item will be marked watched.
    #[serde(rename = "MaxResumePct", skip_serializing_if = "Option::is_none")]
    pub max_resume_pct: Option<i32>,
    /// Gets or sets the minimum duration that an item must have in order to be eligible for playstate updates..
    #[serde(rename = "MinResumeDurationSeconds", skip_serializing_if = "Option::is_none")]
    pub min_resume_duration_seconds: Option<i32>,
    /// Gets or sets the minimum minutes of a book that must be played in order for playstate to be updated.
    #[serde(rename = "MinAudiobookResume", skip_serializing_if = "Option::is_none")]
    pub min_audiobook_resume: Option<i32>,
    /// Gets or sets the remaining minutes of a book that can be played while still saving playstate. If this percentage is crossed playstate will be reset to the beginning and the item will be marked watched.
    #[serde(rename = "MaxAudiobookResume", skip_serializing_if = "Option::is_none")]
    pub max_audiobook_resume: Option<i32>,
    /// Gets or sets the delay in seconds that we will wait after a file system change to try and discover what has been added/removed  Some delay is necessary with some items because their creation is not atomic.  It involves the creation of several  different directories and files.
    #[serde(rename = "LibraryMonitorDelay", skip_serializing_if = "Option::is_none")]
    pub library_monitor_delay: Option<i32>,
    /// Gets or sets the image saving convention.
    #[serde(rename = "ImageSavingConvention", skip_serializing_if = "Option::is_none")]
    pub image_saving_convention: Option<Box<crate::models::ImageSavingConvention>>,
    #[serde(rename = "MetadataOptions", skip_serializing_if = "Option::is_none")]
    pub metadata_options: Option<Vec<crate::models::MetadataOptions>>,
    #[serde(rename = "SkipDeserializationForBasicTypes", skip_serializing_if = "Option::is_none")]
    pub skip_deserialization_for_basic_types: Option<bool>,
    #[serde(rename = "ServerName", skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "UICulture", skip_serializing_if = "Option::is_none")]
    pub ui_culture: Option<String>,
    #[serde(rename = "SaveMetadataHidden", skip_serializing_if = "Option::is_none")]
    pub save_metadata_hidden: Option<bool>,
    #[serde(rename = "ContentTypes", skip_serializing_if = "Option::is_none")]
    pub content_types: Option<Vec<crate::models::NameValuePair>>,
    #[serde(rename = "RemoteClientBitrateLimit", skip_serializing_if = "Option::is_none")]
    pub remote_client_bitrate_limit: Option<i32>,
    #[serde(rename = "EnableFolderView", skip_serializing_if = "Option::is_none")]
    pub enable_folder_view: Option<bool>,
    #[serde(rename = "EnableGroupingIntoCollections", skip_serializing_if = "Option::is_none")]
    pub enable_grouping_into_collections: Option<bool>,
    #[serde(rename = "DisplaySpecialsWithinSeasons", skip_serializing_if = "Option::is_none")]
    pub display_specials_within_seasons: Option<bool>,
    #[serde(rename = "CodecsUsed", skip_serializing_if = "Option::is_none")]
    pub codecs_used: Option<Vec<String>>,
    #[serde(rename = "PluginRepositories", skip_serializing_if = "Option::is_none")]
    pub plugin_repositories: Option<Vec<crate::models::RepositoryInfo>>,
    #[serde(rename = "EnableExternalContentInSuggestions", skip_serializing_if = "Option::is_none")]
    pub enable_external_content_in_suggestions: Option<bool>,
    #[serde(rename = "ImageExtractionTimeoutMs", skip_serializing_if = "Option::is_none")]
    pub image_extraction_timeout_ms: Option<i32>,
    #[serde(rename = "PathSubstitutions", skip_serializing_if = "Option::is_none")]
    pub path_substitutions: Option<Vec<crate::models::PathSubstitution>>,
    /// Gets or sets a value indicating whether slow server responses should be logged as a warning.
    #[serde(rename = "EnableSlowResponseWarning", skip_serializing_if = "Option::is_none")]
    pub enable_slow_response_warning: Option<bool>,
    /// Gets or sets the threshold for the slow response time warning in ms.
    #[serde(rename = "SlowResponseThresholdMs", skip_serializing_if = "Option::is_none")]
    pub slow_response_threshold_ms: Option<i64>,
    /// Gets or sets the cors hosts.
    #[serde(rename = "CorsHosts", skip_serializing_if = "Option::is_none")]
    pub cors_hosts: Option<Vec<String>>,
    /// Gets or sets the number of days we should retain activity logs.
    #[serde(rename = "ActivityLogRetentionDays", skip_serializing_if = "Option::is_none")]
    pub activity_log_retention_days: Option<i32>,
    /// Gets or sets the how the library scan fans out.
    #[serde(rename = "LibraryScanFanoutConcurrency", skip_serializing_if = "Option::is_none")]
    pub library_scan_fanout_concurrency: Option<i32>,
    /// Gets or sets the how many metadata refreshes can run concurrently.
    #[serde(rename = "LibraryMetadataRefreshConcurrency", skip_serializing_if = "Option::is_none")]
    pub library_metadata_refresh_concurrency: Option<i32>,
    /// Gets or sets a value indicating whether older plugins should automatically be deleted from the plugin folder.
    #[serde(rename = "RemoveOldPlugins", skip_serializing_if = "Option::is_none")]
    pub remove_old_plugins: Option<bool>,
    /// Gets or sets a value indicating whether clients should be allowed to upload logs.
    #[serde(rename = "AllowClientLogUpload", skip_serializing_if = "Option::is_none")]
    pub allow_client_log_upload: Option<bool>,
}

impl UpdateConfigurationRequest {
    /// Represents the server configuration.
    pub fn new() -> UpdateConfigurationRequest {
        UpdateConfigurationRequest {
            log_file_retention_days: None,
            is_startup_wizard_completed: None,
            cache_path: None,
            previous_version: None,
            previous_version_str: None,
            enable_metrics: None,
            enable_normalized_item_by_name_ids: None,
            is_port_authorized: None,
            quick_connect_available: None,
            enable_case_sensitive_item_ids: None,
            disable_live_tv_channel_user_data_name: None,
            metadata_path: None,
            metadata_network_path: None,
            preferred_metadata_language: None,
            metadata_country_code: None,
            sort_replace_characters: None,
            sort_remove_characters: None,
            sort_remove_words: None,
            min_resume_pct: None,
            max_resume_pct: None,
            min_resume_duration_seconds: None,
            min_audiobook_resume: None,
            max_audiobook_resume: None,
            library_monitor_delay: None,
            image_saving_convention: None,
            metadata_options: None,
            skip_deserialization_for_basic_types: None,
            server_name: None,
            ui_culture: None,
            save_metadata_hidden: None,
            content_types: None,
            remote_client_bitrate_limit: None,
            enable_folder_view: None,
            enable_grouping_into_collections: None,
            display_specials_within_seasons: None,
            codecs_used: None,
            plugin_repositories: None,
            enable_external_content_in_suggestions: None,
            image_extraction_timeout_ms: None,
            path_substitutions: None,
            enable_slow_response_warning: None,
            slow_response_threshold_ms: None,
            cors_hosts: None,
            activity_log_retention_days: None,
            library_scan_fanout_concurrency: None,
            library_metadata_refresh_concurrency: None,
            remove_old_plugins: None,
            allow_client_log_upload: None,
        }
    }
}


