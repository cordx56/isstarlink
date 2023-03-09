pub fn generate_index_html(
    query_addr: &str,
    remote_addr: &str,
    query_domain: &[String],
    is_starlink: bool,
) -> String {
    let title = if is_starlink {
        "Starlink! - isstarlink.com"
    } else {
        "Not Starlink - isstarlink.com"
    };
    let main_message = if is_starlink {
        "You are connecting from Starlink network!"
    } else {
        "You are not connecting from Starlink network."
    };
    format!(
        r###"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>{title}</title>
  </head>
  <body>
    <h1>{main_message}</h1>
  </body>
</html>"###
    )
}
