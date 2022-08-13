/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemFields : Used to control the data that gets attached to DtoBaseItems.

/// Used to control the data that gets attached to DtoBaseItems.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ItemFields {
    #[serde(rename = "AirTime")]
    AirTime,
    #[serde(rename = "CanDelete")]
    CanDelete,
    #[serde(rename = "CanDownload")]
    CanDownload,
    #[serde(rename = "ChannelInfo")]
    ChannelInfo,
    #[serde(rename = "Chapters")]
    Chapters,
    #[serde(rename = "ChildCount")]
    ChildCount,
    #[serde(rename = "CumulativeRunTimeTicks")]
    CumulativeRunTimeTicks,
    #[serde(rename = "CustomRating")]
    CustomRating,
    #[serde(rename = "DateCreated")]
    DateCreated,
    #[serde(rename = "DateLastMediaAdded")]
    DateLastMediaAdded,
    #[serde(rename = "DisplayPreferencesId")]
    DisplayPreferencesId,
    #[serde(rename = "Etag")]
    Etag,
    #[serde(rename = "ExternalUrls")]
    ExternalUrls,
    #[serde(rename = "Genres")]
    Genres,
    #[serde(rename = "HomePageUrl")]
    HomePageUrl,
    #[serde(rename = "ItemCounts")]
    ItemCounts,
    #[serde(rename = "MediaSourceCount")]
    MediaSourceCount,
    #[serde(rename = "MediaSources")]
    MediaSources,
    #[serde(rename = "OriginalTitle")]
    OriginalTitle,
    #[serde(rename = "Overview")]
    Overview,
    #[serde(rename = "ParentId")]
    ParentId,
    #[serde(rename = "Path")]
    Path,
    #[serde(rename = "People")]
    People,
    #[serde(rename = "PlayAccess")]
    PlayAccess,
    #[serde(rename = "ProductionLocations")]
    ProductionLocations,
    #[serde(rename = "ProviderIds")]
    ProviderIds,
    #[serde(rename = "PrimaryImageAspectRatio")]
    PrimaryImageAspectRatio,
    #[serde(rename = "RecursiveItemCount")]
    RecursiveItemCount,
    #[serde(rename = "Settings")]
    Settings,
    #[serde(rename = "ScreenshotImageTags")]
    ScreenshotImageTags,
    #[serde(rename = "SeriesPrimaryImage")]
    SeriesPrimaryImage,
    #[serde(rename = "SeriesStudio")]
    SeriesStudio,
    #[serde(rename = "SortName")]
    SortName,
    #[serde(rename = "SpecialEpisodeNumbers")]
    SpecialEpisodeNumbers,
    #[serde(rename = "Studios")]
    Studios,
    #[serde(rename = "BasicSyncInfo")]
    BasicSyncInfo,
    #[serde(rename = "SyncInfo")]
    SyncInfo,
    #[serde(rename = "Taglines")]
    Taglines,
    #[serde(rename = "Tags")]
    Tags,
    #[serde(rename = "RemoteTrailers")]
    RemoteTrailers,
    #[serde(rename = "MediaStreams")]
    MediaStreams,
    #[serde(rename = "SeasonUserData")]
    SeasonUserData,
    #[serde(rename = "ServiceName")]
    ServiceName,
    #[serde(rename = "ThemeSongIds")]
    ThemeSongIds,
    #[serde(rename = "ThemeVideoIds")]
    ThemeVideoIds,
    #[serde(rename = "ExternalEtag")]
    ExternalEtag,
    #[serde(rename = "PresentationUniqueKey")]
    PresentationUniqueKey,
    #[serde(rename = "InheritedParentalRatingValue")]
    InheritedParentalRatingValue,
    #[serde(rename = "ExternalSeriesId")]
    ExternalSeriesId,
    #[serde(rename = "SeriesPresentationUniqueKey")]
    SeriesPresentationUniqueKey,
    #[serde(rename = "DateLastRefreshed")]
    DateLastRefreshed,
    #[serde(rename = "DateLastSaved")]
    DateLastSaved,
    #[serde(rename = "RefreshState")]
    RefreshState,
    #[serde(rename = "ChannelImage")]
    ChannelImage,
    #[serde(rename = "EnableMediaSourceDisplay")]
    EnableMediaSourceDisplay,
    #[serde(rename = "Width")]
    Width,
    #[serde(rename = "Height")]
    Height,
    #[serde(rename = "ExtraIds")]
    ExtraIds,
    #[serde(rename = "LocalTrailerCount")]
    LocalTrailerCount,
    #[serde(rename = "IsHD")]
    IsHD,
    #[serde(rename = "SpecialFeatureCount")]
    SpecialFeatureCount,

}

impl ToString for ItemFields {
    fn to_string(&self) -> String {
        match self {
            Self::AirTime => String::from("AirTime"),
            Self::CanDelete => String::from("CanDelete"),
            Self::CanDownload => String::from("CanDownload"),
            Self::ChannelInfo => String::from("ChannelInfo"),
            Self::Chapters => String::from("Chapters"),
            Self::ChildCount => String::from("ChildCount"),
            Self::CumulativeRunTimeTicks => String::from("CumulativeRunTimeTicks"),
            Self::CustomRating => String::from("CustomRating"),
            Self::DateCreated => String::from("DateCreated"),
            Self::DateLastMediaAdded => String::from("DateLastMediaAdded"),
            Self::DisplayPreferencesId => String::from("DisplayPreferencesId"),
            Self::Etag => String::from("Etag"),
            Self::ExternalUrls => String::from("ExternalUrls"),
            Self::Genres => String::from("Genres"),
            Self::HomePageUrl => String::from("HomePageUrl"),
            Self::ItemCounts => String::from("ItemCounts"),
            Self::MediaSourceCount => String::from("MediaSourceCount"),
            Self::MediaSources => String::from("MediaSources"),
            Self::OriginalTitle => String::from("OriginalTitle"),
            Self::Overview => String::from("Overview"),
            Self::ParentId => String::from("ParentId"),
            Self::Path => String::from("Path"),
            Self::People => String::from("People"),
            Self::PlayAccess => String::from("PlayAccess"),
            Self::ProductionLocations => String::from("ProductionLocations"),
            Self::ProviderIds => String::from("ProviderIds"),
            Self::PrimaryImageAspectRatio => String::from("PrimaryImageAspectRatio"),
            Self::RecursiveItemCount => String::from("RecursiveItemCount"),
            Self::Settings => String::from("Settings"),
            Self::ScreenshotImageTags => String::from("ScreenshotImageTags"),
            Self::SeriesPrimaryImage => String::from("SeriesPrimaryImage"),
            Self::SeriesStudio => String::from("SeriesStudio"),
            Self::SortName => String::from("SortName"),
            Self::SpecialEpisodeNumbers => String::from("SpecialEpisodeNumbers"),
            Self::Studios => String::from("Studios"),
            Self::BasicSyncInfo => String::from("BasicSyncInfo"),
            Self::SyncInfo => String::from("SyncInfo"),
            Self::Taglines => String::from("Taglines"),
            Self::Tags => String::from("Tags"),
            Self::RemoteTrailers => String::from("RemoteTrailers"),
            Self::MediaStreams => String::from("MediaStreams"),
            Self::SeasonUserData => String::from("SeasonUserData"),
            Self::ServiceName => String::from("ServiceName"),
            Self::ThemeSongIds => String::from("ThemeSongIds"),
            Self::ThemeVideoIds => String::from("ThemeVideoIds"),
            Self::ExternalEtag => String::from("ExternalEtag"),
            Self::PresentationUniqueKey => String::from("PresentationUniqueKey"),
            Self::InheritedParentalRatingValue => String::from("InheritedParentalRatingValue"),
            Self::ExternalSeriesId => String::from("ExternalSeriesId"),
            Self::SeriesPresentationUniqueKey => String::from("SeriesPresentationUniqueKey"),
            Self::DateLastRefreshed => String::from("DateLastRefreshed"),
            Self::DateLastSaved => String::from("DateLastSaved"),
            Self::RefreshState => String::from("RefreshState"),
            Self::ChannelImage => String::from("ChannelImage"),
            Self::EnableMediaSourceDisplay => String::from("EnableMediaSourceDisplay"),
            Self::Width => String::from("Width"),
            Self::Height => String::from("Height"),
            Self::ExtraIds => String::from("ExtraIds"),
            Self::LocalTrailerCount => String::from("LocalTrailerCount"),
            Self::IsHD => String::from("IsHD"),
            Self::SpecialFeatureCount => String::from("SpecialFeatureCount"),
        }
    }
}

impl Default for ItemFields {
    fn default() -> ItemFields {
        Self::AirTime
    }
}



