const INDEX_STYLE: &str = include_str!("index.css");
const SWAGGER_UI_TEMPLATE: &str = r#"
<!-- HTML for static distribution bundle build -->
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <title>Swagger UI</title>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/swagger-ui/5.18.2/swagger-ui.min.css" integrity="sha512-xRGj65XGEcpPTE7Cn6ujJWokpXVLxqLxdtNZ/n1w52+76XaCRO7UWKZl9yJHvzpk99A0EP6EW+opPcRwPDxwkA==" crossorigin="anonymous" referrerpolicy="no-referrer" />
    <style charset="UTF-8">{:index_style}</style>

    </head>

  <body>
    <div id="swagger-ui"></div>

    <script src="https://cdnjs.cloudflare.com/ajax/libs/swagger-ui/5.18.2/swagger-ui-bundle.min.js" integrity="sha512-9tBcCofqWq+PelL6USpUB7OJrCaObfefi9ht9nVZuKt1XP7eHDs7NwVljLSLVtSsErax1Tz3pG3O82eeq546Rg==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/swagger-ui/5.18.2/swagger-ui-standalone-preset.min.js" integrity="sha512-RYT3vTu8lWSgdoB5zNck/WogIqUb/ap/ivTr6t2LeS+MwqxRQsnSHkHpJRKjnC4T2fH7OMTxxQoC3jh7KGd3HA==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
    <script>
      window.onload = function() {
        window.ui = SwaggerUIBundle({
          url: "{:spec_url}",
          dom_id: '#swagger-ui',
          deepLinking: true,
          presets: [
            SwaggerUIBundle.presets.apis,
            SwaggerUIStandalonePreset
          ],
          plugins: [
            SwaggerUIBundle.plugins.DownloadUrl
          ],
          layout: "StandaloneLayout"
        });
      };
    </script>
  </body>
</html>
"#;

/// Returns the HTML for the Swagger UI page.
pub fn swagger_ui(spec_url: &'static str) -> String {
    SWAGGER_UI_TEMPLATE
        .replace("{:index_style}", INDEX_STYLE)
        .replace("{:spec_url}", spec_url)
}
