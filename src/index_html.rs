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
        r###"You are connecting from <span class="red">Starlink</span> network!"###
    } else {
        r###"You are <span class="red">NOT</span> connecting from Starlink network."###
    };
    let mut query_addrs = "".to_string();
    for domain in query_domain {
        query_addrs = format!("{query_addrs}<li>{domain}</li>");
    }
    format!(
        r###"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>{title}</title>
    <style>
      .red {{
        color: #ff0000;
      }}
    </style>
  </head>
  <body>
    <h1>{main_message}</h1>
    <p>
      <a href="/json">You can use JSON format data.</a>
    </p>
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
                <th>Query Domains</th>
                <td>
                  <ul>
                    {query_addrs}
                  </ul>
                </td>
            </tr>
        </tbody>
    </table>
  </body>
</html>
"###
    )
}
