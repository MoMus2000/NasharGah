pub fn create_http_response() -> String {
    let body = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>My Page</title>
        </head>
        <body>
            <h1>Welcome!</h1>
            <p>This is a simple HTML response.</p>
        </body>
        </html>
    "#;

    let content_length = body.len();

    format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         \r\n\
         {}",
        content_length,
        body
    )
}