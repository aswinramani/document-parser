use rmcp::{
    ServiceExt, handler::server::wrapper::Parameters, schemars, tool, tool_router,
    transport::stdio,
};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct ParseProblemsParams {
    /// Path to a C-CDA XML file
    path: String,
}

#[derive(Clone)]
struct ParserServer;

#[tool_router(server_handler)]
impl ParserServer {
    #[tool(description = "Parse the Problems section of a C-CDA XML file and return it as JSON")]
    fn parse_problems(
        &self,
        Parameters(ParseProblemsParams { path }): Parameters<ParseProblemsParams>,
    ) -> String {
        match clinical_document_parser::parser::problem_section(&path) {
            Ok(section) => serde_json::to_string(&section)
                .unwrap_or_else(|e| format!("could not serialise result: {e}")),
            Err(e) => format!("parse error: {e}"),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service = ParserServer.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}

