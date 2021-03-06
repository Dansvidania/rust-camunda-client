/*
 * Camunda BPM REST API
 *
 * OpenApi Spec for Camunda BPM REST API.
 *
 * The version of the OpenAPI document: 7.13.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MissingAuthorizationDto {
    /// The permission name that the user is missing.
    #[serde(rename = "permissionName", skip_serializing_if = "Option::is_none")]
    pub permission_name: Option<String>,
    /// The name of the resource that the user is missing permission for.
    #[serde(rename = "resourceName", skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// The id of the resource that the user is missing permission for.
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

impl MissingAuthorizationDto {
    pub fn new() -> MissingAuthorizationDto {
        MissingAuthorizationDto {
            permission_name: None,
            resource_name: None,
            resource_id: None,
        }
    }
}


