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
    let query_addr_len = query_domain.len();
    let mut query_addrs = "".to_string();
    for domain in query_domain {
        query_addrs = format!("{query_addrs}<td>{domain}</td>");
    }
    format!(
        r###"<!DOCTYPE html>
        <html lang="en">
          <head>
            <meta charset="utf-8">
            <title>{title}</title>
          </head>
          <body>
            <h1>{main_message}</h1>
            <table>
                <tbody>
                    <tr>
                        <th>Query IP Address</th>
                        <td>{query_addr}</td>
                    </tr>
                    <tr>
                        <th>Remote IP Address</th>
                        <td>{remote_addr}</td>
                    </tr>
                    <tr>
                        <th rowspan="{query_addr_len}">Query Domains</th>
                        {query_addrs}
                    </tr>
                </tbody>
            </table>
          </body>
        </html>
        "###
    )
}
