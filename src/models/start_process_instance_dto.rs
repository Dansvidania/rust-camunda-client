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
pub struct StartProcessInstanceDto {
    /// The business key of the process instance.
    #[serde(rename = "businessKey", skip_serializing_if = "Option::is_none")]
    pub business_key: Option<String>,
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, crate::models::VariableValueDto>>,
    /// The case instance id the process instance is to be initialized with.
    #[serde(rename = "caseInstanceId", skip_serializing_if = "Option::is_none")]
    pub case_instance_id: Option<String>,
    /// **Optional**. A JSON array of instructions that specify which activities to start the process instance at. If this property is omitted, the process instance starts at its default blank start event.
    #[serde(rename = "startInstructions", skip_serializing_if = "Option::is_none")]
    pub start_instructions: Option<Vec<crate::models::ProcessInstanceModificationInstructionDto>>,
    /// Skip execution listener invocation for activities that are started or ended as part of this request. **Note**: This option is currently only respected when start instructions are submitted via the `startInstructions` property.
    #[serde(rename = "skipCustomListeners", skip_serializing_if = "Option::is_none")]
    pub skip_custom_listeners: Option<bool>,
    /// Skip execution of [input/output variable mappings](https://docs.camunda.org/manual/7.13/user-guide/process-engine/variables/#input-output-variable-mapping) for activities that are started or ended as part of this request. **Note**: This option is currently only respected when start instructions are submitted via the `startInstructions` property.
    #[serde(rename = "skipIoMappings", skip_serializing_if = "Option::is_none")]
    pub skip_io_mappings: Option<bool>,
    /// Indicates if the variables, which was used by the process instance during execution, should be returned. Default value: `false`
    #[serde(rename = "withVariablesInReturn", skip_serializing_if = "Option::is_none")]
    pub with_variables_in_return: Option<bool>,
}

impl StartProcessInstanceDto {
    pub fn new() -> StartProcessInstanceDto {
        StartProcessInstanceDto {
            business_key: None,
            variables: None,
            case_instance_id: None,
            start_instructions: None,
            skip_custom_listeners: None,
            skip_io_mappings: None,
            with_variables_in_return: None,
        }
    }
}


