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
pub struct LockedExternalTaskDto {
    /// The id of the activity that this external task belongs to.
    #[serde(rename = "activityId", skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    /// The id of the activity instance that the external task belongs to.
    #[serde(rename = "activityInstanceId", skip_serializing_if = "Option::is_none")]
    pub activity_instance_id: Option<String>,
    /// The full error message submitted with the latest reported failure executing this task;`null` if no failure was reported previously or if no error message was submitted
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// The error details submitted with the latest reported failure executing this task.`null` if no failure was reported previously or if no error details was submitted
    #[serde(rename = "errorDetails", skip_serializing_if = "Option::is_none")]
    pub error_details: Option<String>,
    /// The id of the execution that the external task belongs to.
    #[serde(rename = "executionId", skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    /// The id of the external task.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The date that the task's most recent lock expires or has expired.
    #[serde(rename = "lockExpirationTime", skip_serializing_if = "Option::is_none")]
    pub lock_expiration_time: Option<String>,
    /// The id of the process definition the external task is defined in.
    #[serde(rename = "processDefinitionId", skip_serializing_if = "Option::is_none")]
    pub process_definition_id: Option<String>,
    /// The key of the process definition the external task is defined in.
    #[serde(rename = "processDefinitionKey", skip_serializing_if = "Option::is_none")]
    pub process_definition_key: Option<String>,
    /// The version tag of the process definition the external task is defined in.
    #[serde(rename = "processDefinitionVersionTag", skip_serializing_if = "Option::is_none")]
    pub process_definition_version_tag: Option<String>,
    /// The id of the process instance the external task belongs to.
    #[serde(rename = "processInstanceId", skip_serializing_if = "Option::is_none")]
    pub process_instance_id: Option<String>,
    /// The id of the tenant the external task belongs to.
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// The number of retries the task currently has left.
    #[serde(rename = "retries", skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    /// Whether the process instance the external task belongs to is suspended.
    #[serde(rename = "suspended", skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    /// The id of the worker that posesses or posessed the most recent lock.
    #[serde(rename = "workerId", skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
    /// The priority of the external task.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// The topic name of the external task.
    #[serde(rename = "topicName", skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// The business key of the process instance the external task belongs to.
    #[serde(rename = "businessKey", skip_serializing_if = "Option::is_none")]
    pub business_key: Option<String>,
    /// A JSON object containing a property for each of the requested variables. The key is the variable name, the value is a JSON object of serialized variable values with the following properties:
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, crate::models::VariableValueDto>>,
}

impl LockedExternalTaskDto {
    pub fn new() -> LockedExternalTaskDto {
        LockedExternalTaskDto {
            activity_id: None,
            activity_instance_id: None,
            error_message: None,
            error_details: None,
            execution_id: None,
            id: None,
            lock_expiration_time: None,
            process_definition_id: None,
            process_definition_key: None,
            process_definition_version_tag: None,
            process_instance_id: None,
            tenant_id: None,
            retries: None,
            suspended: None,
            worker_id: None,
            priority: None,
            topic_name: None,
            business_key: None,
            variables: None,
        }
    }
}

