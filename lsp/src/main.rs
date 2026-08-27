use bevywind_core::parse_class;
use lsp_server::{Connection, Message, Notification as ServerNotification, Request, Response};
use lsp_types::notification::{
    DidChangeTextDocument, DidOpenTextDocument, Notification as LspNotification, PublishDiagnostics,
};
use lsp_types::*;
use std::collections::HashMap;
use std::error::Error;

#[derive(Default)]
struct Server {
    documents: HashMap<Url, String>,
}
type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

fn main() -> Result<()> {
    let (connection, io_thread) = Connection::stdio();
    let capabilities = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        ..Default::default()
    };
    let init = InitializeResult {
        capabilities,
        server_info: Some(ServerInfo {
            name: "bevywind-lsp".into(),
            version: Some(env!("CARGO_PKG_VERSION").into()),
        }),
    };
    let params = connection.initialize(serde_json::to_value(init)?)?;
    let _params: InitializeParams = serde_json::from_value(params)?;
    let mut server = Server::default();
    for message in &connection.receiver {
        match message {
            Message::Request(request) => {
                if connection.handle_shutdown(&request)? {
                    break;
                }
                handle_request(&connection, &mut server, request)?;
            }
            Message::Notification(notification) => {
                handle_notification(&connection, &mut server, notification)?
            }
            Message::Response(_) => {}
        }
    }
    io_thread.join()?;
    Ok(())
}

fn handle_notification(
    connection: &Connection,
    server: &mut Server,
    note: ServerNotification,
) -> Result<()> {
    match note.method.as_str() {
        DidOpenTextDocument::METHOD => {
            let params: DidOpenTextDocumentParams = serde_json::from_value(note.params)?;
            let uri = params.text_document.uri;
            server
                .documents
                .insert(uri.clone(), params.text_document.text);
            publish_diagnostics(connection, server, &uri)?;
        }
        DidChangeTextDocument::METHOD => {
            let params: DidChangeTextDocumentParams = serde_json::from_value(note.params)?;
            if let Some(change) = params.content_changes.into_iter().next() {
                let uri = params.text_document.uri;
                server.documents.insert(uri.clone(), change.text);
                publish_diagnostics(connection, server, &uri)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_request(connection: &Connection, _server: &mut Server, request: Request) -> Result<()> {
    send_ok(connection, request.id, &())?;
    Ok(())
}

fn publish_diagnostics(connection: &Connection, server: &Server, uri: &Url) -> Result<()> {
    let source = server
        .documents
        .get(uri)
        .map(String::as_str)
        .unwrap_or_default();
    let diagnostics = style_classes(source)
        .into_iter()
        .filter_map(|(class, offset)| parse_class(&class, offset).err())
        .map(|error| Diagnostic {
            range: Range::new(
                position_at(source, error.offset),
                position_at(source, error.offset + error.length),
            ),
            severity: Some(DiagnosticSeverity::ERROR),
            code: None,
            code_description: None,
            source: Some("bevywind".into()),
            message: error.message,
            related_information: None,
            tags: None,
            data: None,
        })
        .collect();
    let params = PublishDiagnosticsParams {
        uri: uri.clone(),
        diagnostics,
        version: None,
    };
    connection
        .sender
        .send(Message::Notification(ServerNotification::new(
            PublishDiagnostics::METHOD.into(),
            params,
        )))?;
    Ok(())
}

fn style_classes(source: &str) -> Vec<(String, usize)> {
    let mut classes = Vec::new();
    let mut search_from = 0;
    while let Some(relative) = source[search_from..].find("bstyle!(") {
        let start = search_from + relative + "bstyle!(".len();
        let Some(end_relative) = source[start..].find(')') else {
            break;
        };
        let end = start + end_relative;
        let mut content_start = start;
        let mut content = &source[start..end];
        if content.starts_with('"') && content.ends_with('"') {
            content_start += 1;
            content = &content[1..content.len() - 1];
        }
        for (relative, class) in content
            .split_whitespace()
            .map(|class| (content.find(class).unwrap_or(0), class))
        {
            classes.push((class.to_owned(), content_start + relative));
        }
        search_from = end + 1;
    }
    classes
}

fn position_at(source: &str, offset: usize) -> Position {
    let prefix = &source[..offset.min(source.len())];
    Position::new(
        prefix.bytes().filter(|byte| *byte == b'\n').count() as u32,
        prefix
            .rsplit('\n')
            .next()
            .unwrap_or_default()
            .chars()
            .count() as u32,
    )
}

fn send_ok<T: serde::Serialize>(
    connection: &Connection,
    id: lsp_server::RequestId,
    result: &T,
) -> Result<()> {
    connection.sender.send(Message::Response(Response::new_ok(
        id,
        serde_json::to_value(result)?,
    )))?;
    Ok(())
}
