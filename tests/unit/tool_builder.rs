use openrouter_rs::types::tool::{FunctionDefinition, ServerTool, Tool, ToolDefinition};
use serde_json::json;

#[test]
fn test_tool_builder_preserves_fields_when_name_is_last() {
    let tool = Tool::builder()
        .description("Get current weather for a location")
        .parameters(json!({
            "type": "object",
            "properties": {
                "location": {"type": "string"}
            }
        }))
        .name("get_weather")
        .build()
        .expect("tool should build when name is set last");

    assert_eq!(tool.function.name, "get_weather");
    assert_eq!(
        tool.function.description,
        "Get current weather for a location"
    );
    assert_eq!(tool.function.parameters["type"], "object");
    assert_eq!(
        tool.function.parameters["properties"]["location"]["type"],
        "string"
    );
}

#[test]
fn test_tool_builder_renaming_does_not_reset_other_fields() {
    let tool = Tool::builder()
        .name("draft_name")
        .description("Performs a calculation")
        .parameters(json!({"type": "object"}))
        .name("calculator")
        .build()
        .expect("tool should build after rename");

    assert_eq!(tool.function.name, "calculator");
    assert_eq!(tool.function.description, "Performs a calculation");
    assert_eq!(tool.function.parameters["type"], "object");
}

#[test]
fn test_tool_builder_accepts_full_function_definition_override() {
    let function = FunctionDefinition::builder()
        .name("lookup_user")
        .description("Find a user by id")
        .parameters(json!({
            "type": "object",
            "required": ["user_id"]
        }))
        .build()
        .expect("function definition should build");

    let tool = Tool::builder()
        .tool_type("function")
        .function(function)
        .build()
        .expect("tool should build from full function definition");

    assert_eq!(tool.function.name, "lookup_user");
    assert_eq!(tool.function.description, "Find a user by id");
    assert_eq!(tool.function.parameters["required"][0], "user_id");
}

#[test]
fn test_tool_builder_requires_name() {
    let err = Tool::builder()
        .description("No name set")
        .parameters(json!({"type": "object"}))
        .build()
        .expect_err("tool should not build without a name");

    assert!(err.to_string().contains("Tool name is required"));
}

#[test]
fn test_server_tool_web_search_serializes() {
    let tool = ServerTool::openrouter_web_search()
        .option("engine", "exa")
        .option("max_total_results", 10);

    let value = serde_json::to_value(&tool).expect("server tool should serialize");

    assert_eq!(value["type"], "openrouter:web_search");
    assert_eq!(value["parameters"]["engine"], "exa");
    assert_eq!(value["parameters"]["max_total_results"], 10);
}

#[test]
fn test_tool_definition_serializes_function_tool_without_shape_change() {
    let tool = Tool::new(
        "weather",
        "Get weather",
        json!({
            "type": "object",
            "properties": {
                "location": {"type": "string"}
            }
        }),
    );

    let function_value = serde_json::to_value(&tool).expect("function tool should serialize");
    let definition_value =
        serde_json::to_value(ToolDefinition::from(tool)).expect("tool definition should serialize");

    assert_eq!(definition_value, function_value);
}
