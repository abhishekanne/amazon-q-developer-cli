use std::io::Write;

use eyre::Result;
use serde::{Deserialize, Serialize};

use super::{InvokeOutput, OutputKind};
use crate::os::Os;

#[derive(Debug, Clone, Deserialize)]
pub struct Help {
    #[serde(default)]
    query: Option<String>,
    #[serde(default)]
    use_case: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HelpResponse {
    recommendations: Vec<ToolRecommendation>,
    general_info: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ToolRecommendation {
    tool_name: String,
    description: String,
    use_case: String,
    example: Option<String>,
}

impl Help {
    pub async fn invoke(&self, _os: &Os, _updates: impl Write) -> Result<InvokeOutput> {
        let recommendations = match (self.query.as_deref(), self.use_case.as_deref()) {
            // Context and repo mapping - broader matching
            (Some(query), _) if query.contains("context") || query.contains("add files") || query.contains("repo") || query.contains("map") || query.contains("understand") || query.contains("include files") => {
                vec![
                    ToolRecommendation {
                        tool_name: "/context".to_string(),
                        description: "Add files or directories to conversation context".to_string(),
                        use_case: "Map out your repo structure and keep relevant files in context".to_string(),
                        example: Some("Use '/context add .' to add your entire repo, or '/context add src/' for specific directories".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "knowledge".to_string(),
                        description: "Store and retrieve project information across sessions".to_string(),
                        use_case: "Build a persistent understanding of your codebase".to_string(),
                        example: Some("Store important project insights that persist across chat sessions".to_string()),
                    },
                ]
            },
            
            // Knowledge and memory
            (Some(query), _) if query.contains("knowledge") || query.contains("remember") || query.contains("store") || query.contains("save") => {
                vec![
                    ToolRecommendation {
                        tool_name: "knowledge".to_string(),
                        description: "Store and retrieve information across chat sessions".to_string(),
                        use_case: "Build persistent project documentation and context".to_string(),
                        example: Some("Store important findings, decisions, or project context that you want to remember".to_string()),
                    },
                ]
            },
            
            // Agent switching and specialization
            (Some(query), _) if query.contains("agent") || query.contains("switch") || query.contains("specialize") => {
                vec![
                    ToolRecommendation {
                        tool_name: "/agent".to_string(),
                        description: "Switch between specialized AI agents for different tasks".to_string(),
                        use_case: "Use different agents optimized for specific workflows".to_string(),
                        example: Some("Type '/agent list' to see available agents, '/agent switch <name>' to change".to_string()),
                    },
                ]
            },
            
            // Session management
            (Some(query), _) if query.contains("session") || query.contains("clear") || query.contains("reset") || query.contains("compact") => {
                vec![
                    ToolRecommendation {
                        tool_name: "/clear".to_string(),
                        description: "Clear conversation history".to_string(),
                        use_case: "Start fresh while keeping the same session".to_string(),
                        example: Some("Reset your conversation when switching to a new topic".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "/compact".to_string(),
                        description: "Summarize conversation to free up context space".to_string(),
                        use_case: "Manage long conversations that approach memory limits".to_string(),
                        example: Some("Compress conversation history while preserving key information".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "/usage".to_string(),
                        description: "Show current session's context window usage".to_string(),
                        use_case: "Monitor how much context space you're using".to_string(),
                        example: Some("Check if you're approaching context limits".to_string()),
                    },
                ]
            },
            
            // Editor and complex prompts
            (Some(query), _) if query.contains("editor") || query.contains("multiline") || query.contains("complex") => {
                vec![
                    ToolRecommendation {
                        tool_name: "/editor".to_string(),
                        description: "Open $EDITOR to compose multi-line prompts".to_string(),
                        use_case: "Write complex prompts or paste large text blocks".to_string(),
                        example: Some("Perfect for detailed requirements or when you need to paste code/logs".to_string()),
                    },
                ]
            },
            
            // General Q CLI workflow
            _ => {
                vec![
                    ToolRecommendation {
                        tool_name: "/context".to_string(),
                        description: "Add files/directories to conversation context".to_string(),
                        use_case: "Keep relevant project files accessible throughout your session".to_string(),
                        example: Some("Essential for working with codebases - use '/context add .' for full repo context".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "knowledge".to_string(),
                        description: "Store information across chat sessions".to_string(),
                        use_case: "Build persistent project understanding and documentation".to_string(),
                        example: Some("Remember important insights and decisions between sessions".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "/agent".to_string(),
                        description: "Switch between specialized AI agents".to_string(),
                        use_case: "Use agents optimized for specific tasks or domains".to_string(),
                        example: Some("Different agents excel at different types of work".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "/tools".to_string(),
                        description: "View all available tools and permissions".to_string(),
                        use_case: "Quick reference for Q's capabilities".to_string(),
                        example: Some("See what tools are available and their current status".to_string()),
                    },
                ]
            }
        };
        
        let general_info = if self.query.is_none() && self.use_case.is_none() {
            Some("Amazon Q CLI offers unique workflow features beyond basic file operations. Use /context to map your repo, knowledge to remember insights across sessions, /agent to switch between specialized assistants, and other slash commands for session management.".to_string())
        } else {
            None
        };
        
        let response = HelpResponse {
            recommendations,
            general_info,
        };
        
        Ok(InvokeOutput {
            output: OutputKind::Json(serde_json::to_value(&response)?),
        })
    }

    pub fn queue_description(&self, output: &mut impl Write) -> Result<()> {
        use crossterm::{queue, style};
        queue!(
            output,
            style::Print("Getting Q CLI help and tool recommendations")
        )?;
        Ok(())
    }

    pub async fn validate(&self, _os: &Os) -> Result<()> {
        Ok(())
    }
}
