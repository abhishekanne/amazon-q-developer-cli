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
            // File operations
            (Some(query), _) if query.contains("file") || query.contains("read") || query.contains("write") => {
                vec![
                    ToolRecommendation {
                        tool_name: "fs_read".to_string(),
                        description: "Read files, directories, and images with various modes".to_string(),
                        use_case: "Exploring codebases, reading configuration files, searching for patterns".to_string(),
                        example: Some("Use fs_read to examine a specific file or search for patterns across files".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "fs_write".to_string(),
                        description: "Create and edit files with precision".to_string(),
                        use_case: "Code modifications, creating new files, making targeted edits".to_string(),
                        example: Some("Use fs_write to create new files or make specific changes to existing code".to_string()),
                    },
                ]
            },
            
            // AWS operations
            (Some(query), _) if query.contains("aws") || query.contains("cloud") => {
                vec![
                    ToolRecommendation {
                        tool_name: "use_aws".to_string(),
                        description: "Make AWS CLI API calls for resource management".to_string(),
                        use_case: "Managing AWS resources, querying services, deployment operations".to_string(),
                        example: Some("Use use_aws to list S3 buckets, describe EC2 instances, or manage Lambda functions".to_string()),
                    },
                ]
            },
            
            // Command execution
            (Some(query), _) if query.contains("command") || query.contains("execute") || query.contains("run") => {
                vec![
                    ToolRecommendation {
                        tool_name: "execute_bash".to_string(),
                        description: "Execute bash commands directly from chat".to_string(),
                        use_case: "Running tests, building projects, system operations".to_string(),
                        example: Some("Use execute_bash to run git commands, compile code, or execute scripts".to_string()),
                    },
                ]
            },
            
            // Q CLI commands and slash commands
            (Some(query), _) if query.contains("command") || query.contains("slash") || query.contains("/") => {
                vec![
                    ToolRecommendation {
                        tool_name: "/tools".to_string(),
                        description: "View all available tools and their permissions".to_string(),
                        use_case: "Quick reference for available functionality and tool status".to_string(),
                        example: Some("Type /tools to see all available tools with descriptions and permission status".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "/quit".to_string(),
                        description: "Exit the Q CLI chat session (aliases: /q, /exit)".to_string(),
                        use_case: "End your current chat session".to_string(),
                        example: Some("Type /quit, /q, or /exit to leave the chat".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "/clear".to_string(),
                        description: "Clear the conversation history".to_string(),
                        use_case: "Start fresh while keeping the same session".to_string(),
                        example: Some("Type /clear to reset conversation history".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "/agent".to_string(),
                        description: "Manage and switch between different AI agents".to_string(),
                        use_case: "Use specialized agents for different tasks".to_string(),
                        example: Some("Type /agent list to see available agents, /agent switch <name> to change".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "/context".to_string(),
                        description: "Manage context files for the chat session".to_string(),
                        use_case: "Add files or directories to conversation context".to_string(),
                        example: Some("Type /context add <file> to include files in conversation context".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "/editor".to_string(),
                        description: "Open $EDITOR to compose a multi-line prompt".to_string(),
                        use_case: "Write complex prompts or paste large text blocks".to_string(),
                        example: Some("Type /editor to open your default editor for prompt composition".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "/compact".to_string(),
                        description: "Summarize conversation to free up context space".to_string(),
                        use_case: "Manage long conversations that approach memory limits".to_string(),
                        example: Some("Type /compact to summarize and compress conversation history".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "/usage".to_string(),
                        description: "Show current session's context window usage".to_string(),
                        use_case: "Monitor how much context space you're using".to_string(),
                        example: Some("Type /usage to see context window utilization".to_string()),
                    },
                ]
            },
                vec![
            // Knowledge management
            (Some(query), _) if query.contains("knowledge") || query.contains("remember") || query.contains("store") => {
                vec![
                    ToolRecommendation {
                        tool_name: "knowledge".to_string(),
                        description: "Store and retrieve information across chat sessions".to_string(),
                        use_case: "Building personal documentation, storing project context, semantic search".to_string(),
                        example: Some("Use knowledge to store important project information or search previous conversations".to_string()),
                    },
                ]
            },
            
            // Use case based recommendations
            (_, Some("development")) | (_, Some("coding")) => {
                vec![
                    ToolRecommendation {
                        tool_name: "fs_read".to_string(),
                        description: "Read and explore code files".to_string(),
                        use_case: "Code review, understanding project structure".to_string(),
                        example: Some("Read source files to understand implementation details".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "fs_write".to_string(),
                        description: "Modify and create code files".to_string(),
                        use_case: "Implementing features, fixing bugs, refactoring".to_string(),
                        example: Some("Make precise code changes or create new modules".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "execute_bash".to_string(),
                        description: "Run development commands".to_string(),
                        use_case: "Testing, building, version control operations".to_string(),
                        example: Some("Run tests, build projects, or execute git commands".to_string()),
                    },
                ]
            },
            
            (_, Some("aws")) | (_, Some("cloud")) => {
                vec![
                    ToolRecommendation {
                        tool_name: "use_aws".to_string(),
                        description: "Manage AWS resources".to_string(),
                        use_case: "Infrastructure management, service configuration".to_string(),
                        example: Some("Query AWS services, manage resources, deploy applications".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "fs_write".to_string(),
                        description: "Create infrastructure code".to_string(),
                        use_case: "Writing CloudFormation, CDK, or Terraform files".to_string(),
                        example: Some("Generate infrastructure as code templates".to_string()),
                    },
                ]
            },
            
            // Default comprehensive overview
            _ => {
                vec![
                    ToolRecommendation {
                        tool_name: "fs_read".to_string(),
                        description: "Read files, directories, and images".to_string(),
                        use_case: "Exploring codebases, reading documentation, searching content".to_string(),
                        example: Some("Perfect for understanding project structure and finding specific code patterns".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "fs_write".to_string(),
                        description: "Create and edit files with precision".to_string(),
                        use_case: "Code modifications, creating new files, making targeted changes".to_string(),
                        example: Some("Ideal for implementing features, fixing bugs, or creating new modules".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "execute_bash".to_string(),
                        description: "Execute bash commands and scripts".to_string(),
                        use_case: "Running tests, building projects, system operations".to_string(),
                        example: Some("Great for automation, testing, and development workflow tasks".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "use_aws".to_string(),
                        description: "Make AWS CLI API calls".to_string(),
                        use_case: "Managing AWS resources, querying services, deployment".to_string(),
                        example: Some("Essential for cloud infrastructure management and AWS operations".to_string()),
                    },
                    ToolRecommendation {
                        tool_name: "knowledge".to_string(),
                        description: "Store and retrieve information across sessions".to_string(),
                        use_case: "Building documentation, storing context, semantic search".to_string(),
                        example: Some("Useful for maintaining project knowledge and finding relevant information".to_string()),
                    },
                ]
            }
        };
        
        let general_info = if self.query.is_none() && self.use_case.is_none() {
            Some("Amazon Q CLI provides intelligent assistance for development, AWS operations, and automation. You can use slash commands (like /tools, /help) for quick actions or ask me questions to get contextual tool recommendations.".to_string())
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
