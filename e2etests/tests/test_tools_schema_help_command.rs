use q_cli_e2e_tests::q_chat_helper::QChatSession;

#[test]
#[cfg(feature = "tools")]
fn test_tools_schema_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing /tools schema --help command...");
    
    let mut chat = QChatSession::new()?;
    println!("✅ Q Chat session started");
    
    let response = chat.execute_command("/tools schema --help")?;
    
    println!("📝 Tools schema help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify command description
    assert!(response.contains("Show the input schema for all available tools"), "Missing command description");
    println!("✅ Found command description");
    
    // Verify usage format
    assert!(response.contains("Usage:") && response.contains("/tools schema"), "Missing usage format");
    println!("✅ Found usage format");
    
    // Verify options section
    assert!(response.contains("Options:"), "Missing Options section");
    assert!(response.contains("-h") && response.contains("--help")&& response.contains("Print help"), "Missing help option");
    println!("✅ Found options section with help flag");
    
    println!("✅ All tools schema help functionality verified!");
    
    chat.quit()?;
    println!("✅ Test completed successfully");
    
    Ok(())
}