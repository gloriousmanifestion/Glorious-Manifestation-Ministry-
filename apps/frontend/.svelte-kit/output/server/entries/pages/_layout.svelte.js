import { s as store_get, h as head, a as unsubscribe_stores, e as escape_html, b as attr, c as slot } from "../../chunks/index.js";
/* empty css               */
import { p as page } from "../../chunks/stores.js";
import "../../chunks/auth.js";
const SITE = {
  baseUrl: "https://propheticgloriousmanifestation.org",
  name: "Prophetic Glorious Manifestation",
  tagline: "Pastoral School",
  description: "Pastoral School under Prophetic Glorious Manifestation. Training pastors, ministers and prophets in Owerri, Imo State. February to July program, Monday to Friday.",
  keywords: [
    "Prophetic Glorious Manifestation",
    "Pastoral School",
    "Owerri",
    "Imo State",
    "church ministry",
    "pastors",
    "ministers",
    "prophets",
    "ministry training",
    "biblical studies",
    "prophetic ministry"
  ].join(", "),
  locale: "en_NG",
  twitterHandle: "@propheticgloriousmanifestation",
  creator: "Prophetic Glorious Manifestation",
  ogImage: "/og-image.png"
};
function SeoHead($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    var $$store_subs;
    let {
      title,
      description = SITE.description,
      keywords = SITE.keywords,
      image,
      imageAlt = SITE.description,
      url,
      type = "website",
      noIndex = false,
      canonical
    } = $$props;
    const origin = SITE.baseUrl.replace(/\/$/, "");
    const path = store_get($$store_subs ??= {}, "$page", page).url.pathname;
    const fullUrl = url || canonical || `${origin}${path}`;
    const fullImage = image?.startsWith("http") ? image : image ? `${origin}${image.startsWith("/") ? "" : "/"}${image}` : `${origin}${SITE.ogImage}`;
    const fullTitle = title ? `${title} — ${SITE.name}` : `${SITE.name} — ${SITE.tagline}`;
    head("1t153bb", $$renderer2, ($$renderer3) => {
      $$renderer3.title(($$renderer4) => {
        $$renderer4.push(`<title>${escape_html(fullTitle)}</title>`);
      });
      $$renderer3.push(`<meta name="description"${attr("content", description)}/> <meta name="keywords"${attr("content", keywords)}/> `);
      if (noIndex) {
        $$renderer3.push("<!--[-->");
        $$renderer3.push(`<meta name="robots" content="noindex, nofollow"/>`);
      } else {
        $$renderer3.push("<!--[!-->");
        $$renderer3.push(`<meta name="robots" content="index, follow, max-image-preview:large, max-snippet:-1, max-video-preview:-1"/>`);
      }
      $$renderer3.push(`<!--]--> <link rel="canonical"${attr("href", fullUrl)}/> <meta property="og:type"${attr("content", type)}/> <meta property="og:site_name"${attr("content", SITE.name)}/> <meta property="og:title"${attr("content", fullTitle)}/> <meta property="og:description"${attr("content", description)}/> <meta property="og:url"${attr("content", fullUrl)}/> <meta property="og:image"${attr("content", fullImage)}/> <meta property="og:image:alt"${attr("content", imageAlt)}/> <meta property="og:image:width" content="1200"/> <meta property="og:image:height" content="630"/> <meta property="og:locale"${attr("content", SITE.locale)}/> <meta name="twitter:card" content="summary_large_image"/> <meta name="twitter:site"${attr("content", SITE.twitterHandle)}/> <meta name="twitter:creator"${attr("content", SITE.creator)}/> <meta name="twitter:title"${attr("content", fullTitle)}/> <meta name="twitter:description"${attr("content", description)}/> <meta name="twitter:image"${attr("content", fullImage)}/> <meta name="twitter:image:alt"${attr("content", imageAlt)}/> <meta property="og:image:type" content="image/png"/>`);
    });
    if ($$store_subs) unsubscribe_stores($$store_subs);
  });
}
function Chatbot($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    $$renderer2.push(`<div class="fixed bottom-[max(1.5rem,env(safe-area-inset-bottom))] right-[max(1rem,env(safe-area-inset-right))] z-50">`);
    {
      $$renderer2.push("<!--[!-->");
    }
    $$renderer2.push(`<!--]--> <button type="button" class="flex h-14 w-14 min-w-[3.5rem] min-h-[3.5rem] items-center justify-center rounded-full bg-gradient-to-br from-pink-500 to-purple-600 text-white shadow-lg shadow-pink-500/30 transition hover:scale-105 hover:shadow-xl active:scale-95 touch-manipulation" aria-label="Open chat">`);
    {
      $$renderer2.push("<!--[!-->");
      $$renderer2.push(`<svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path></svg>`);
    }
    $$renderer2.push(`<!--]--></button></div>`);
  });
}
function _layout($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    SeoHead($$renderer2, {});
    $$renderer2.push(`<!----> <!--[-->`);
    slot($$renderer2, $$props, "default", {});
    $$renderer2.push(`<!--]--> `);
    Chatbot($$renderer2);
    $$renderer2.push(`<!---->`);
  });
}
export {
  _layout as default
};
