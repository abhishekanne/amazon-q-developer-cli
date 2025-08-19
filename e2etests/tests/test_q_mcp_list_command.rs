use q_cli_e2e_tests::q_chat_helper::QChatSession;

#[test]
#[cfg(feature = "mcp")]
fn test_q_mcp_list_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing q mcp list command...");
    
    let mut chat = QChatSession::new()?;
    println!("✅ Q Chat session started");
    
    let response = chat.execute_command("q mcp list")?;
    
    println!("📝 MCP list response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify tool execution prompt
    assert!(response.contains("Using tool:"), "Missing tool execution indicator");
    assert!(response.contains("q mcp list"), "Missing command in tool execution");
    assert!(response.contains("List available MCP servers"), "Missing purpose description");
    assert!(response.contains("Allow this action?") && response.contains("to trust (always allow) this tool for the session."), "Missing permission prompt");
    println!("✅ Found tool execution permission prompt");
    
    // Allow the tool execution
    let allow_response = chat.execute_command("y")?;
    
    println!("📝 Allow response: {} bytes", allow_response.len());
    println!("📝 ALLOW RESPONSE:");
    println!("{}", allow_response);
    println!("📝 END ALLOW RESPONSE");
    
    
    // Verify MCP server listing
    assert!(allow_response.contains("q_cli_default"), "Missing q_cli_default server");
    assert!(allow_response.contains("default"), "Missing default tag");
    assert!(allow_response.contains("global"), "Missing global tag");
    println!("✅ Found MCP server listing with  servers and completion");
    
    chat.quit()?;
    println!("✅ Test completed successfully");
    
    Ok(())
}