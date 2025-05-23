import { z } from "zod";

const answerSchema = z.object({
  Status: z.number(),
  Answer: z.object({ name: z.string(), data: z.string() }).array(),
});

const query = async (
  host: string,
  questions: { type: string; name: string },
) => {
  const resp = await fetch(
    `https://${host}/dns-query?name=${questions.name}&type=${questions.type}`,
    { headers: { accept: "application/dns-json" } },
  );
  return answerSchema.safeParse(await resp.json()).data || null;
};

const html = (
  starlink: boolean,
  query: string | undefined,
  remote: string,
  answers: string[],
) => `
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>${starlink ? "Starlink! - isstar.link" : "Not Starlink - isstar.link"}</title>
    <style>
      .red {
        color: #ff0000;
      }
    </style>
  </head>
  <body>
    <h1>${starlink ? 'You are connecting from <span class="red">Starlink</span> network!' : 'You are <span class="red">NOT</span> connecting from Starlink network.'}</h1>
    <p>
      <a href="/json">You can use JSON format data.</a>
    </p>
    <p>
      Source code: <a href="https://github.com/cordx56/isstarlink" target="_blank" rel="noreferrer">https://github.com/cordx56/isstarlink</a>
    </p>
    <table>
        <tbody>
            <tr>
                <th>Query IP Address</th>
                <td>${query || remote}</td>
            </tr>
            <tr>
                <th>Remote IP Address</th>
                <td>${remote}</td>
            </tr>
            <tr>
                <th>Query Domains</th>
                <td>
                    <ul>
                        ${answers.map((v) => `<li>${v}</li>`).join("")}
                    </ul>
                </td>
            </tr>
        </tbody>
    </table>
  </body>
</html>
`;

const formatIpv6 = (addr: string): string => {
  const sections = addr.split(":");
  if (sections.includes("")) {
    const count = 8 - sections.length + 1;
    const index = sections.indexOf("");
    sections.splice(index, 1);
    for (let i = 0; i < count; i++) {
      sections.splice(index, 0, "0");
    }
  }
  for (let i = 0; i < sections.length; i++) {
    for (let j = sections[i].length; j < 4; j++) {
      sections[i] = "0" + sections[i];
    }
  }
  return sections.join(":");
};

const ipArpa = (addr: string): string => {
  if (addr.includes(":")) {
    return [
      ...addr.replaceAll(":", "").split("").reverse(),
      "ip6",
      "arpa.",
    ].join(".");
  } else {
    return [...addr.split(".").reverse(), "in-addr", "arpa."].join(".");
  }
};

export default {
  async fetch(req, _env, _ctx): Promise<Response> {
    try {
      const url = new URL(req.url);
      const query_addr = url.searchParams.get("addr")?.trim();
      const remote_addr = req.headers.get("CF-Connecting-IP")?.trim();
      if (remote_addr) {
        let addr = query_addr || remote_addr;
        if (addr) {
          if (addr.includes(":")) {
            addr = formatIpv6(addr);
          }
          const resp = await query("cloudflare-dns.com", {
            name: ipArpa(addr),
            type: "PTR",
          });
          if (resp) {
            const query_domain =
              resp.Answer.map((v) =>
                v.data.endsWith(".") ? v.data.slice(0, -1) : v.data,
              ) || [];
            const is_starlink =
              query_domain.filter((v) => v.endsWith(".starlinkisp.net"))
                .length !== 0;
            const result = {
              success: true,
              query_addr,
              remote_addr,
              query_domain,
              is_starlink,
            };
            if (url.pathname === "/json") {
              return Response.json(result);
            } else {
              return new Response(
                html(is_starlink, query_addr, remote_addr, query_domain),
                { headers: { "Content-Type": "text/html;charset=UTF-8" } },
              );
            }
          }
        }
      }
      return Response.json({ success: false, query_addr, remote_addr });
    } catch (e) {
      return Response.json({ success: false, message: `${e}` });
    }
  },
} satisfies ExportedHandler<Env>;
