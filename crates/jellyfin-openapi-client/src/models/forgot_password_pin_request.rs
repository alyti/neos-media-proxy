/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ForgotPasswordPinRequest : Forgot Password Pin enter request body DTO.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ForgotPasswordPinRequest {
    /// Gets or sets the entered pin to have the password reset.
    #[serde(rename = "Pin")]
    pub pin: String,
}

impl ForgotPasswordPinRequest {
    /// Forgot Password Pin enter request body DTO.
    pub fn new(pin: String) -> ForgotPasswordPinRequest {
        ForgotPasswordPinRequest {
            pin,
        }
    }
}

