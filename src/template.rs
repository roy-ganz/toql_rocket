//! A macro to generate a bad request HTML page.

 #[macro_export]
 macro_rules! bad_request_template {
    ($description:expr) => (
        format!(r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="utf-8">
                <title>400 Bad Request</title>
            </head>
            <body align="center">
                <div align="center">
                    <h1>400: Bad Request</h1>
                    <p>Request failed, because {}.</p>
                    <hr />
                    <small>Rocket</small>
                </div>
            </body>
            </html>
        "#, $description
        )
    )
}