import { h as head, e as escape_html, b as attr, d as stringify } from "../../../chunks/index.js";
import { N as Nav } from "../../../chunks/Nav.js";
import { F as Footer } from "../../../chunks/Footer.js";
import { A as AnimateOnView } from "../../../chunks/AnimateOnView.js";
import { S as SITE } from "../../../chunks/config.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    const mapUrl = "https://maps.google.com/?q=" + encodeURIComponent(SITE.address);
    const whatsappUrl = "https://wa.me/" + SITE.whatsappFull.replace("+", "");
    let name = "";
    let email = "";
    let subject = "";
    let message = "";
    let loading = false;
    head("1bv7ezn", $$renderer2, ($$renderer3) => {
      $$renderer3.title(($$renderer4) => {
        $$renderer4.push(`<title>Contact — ${escape_html(SITE.name)} Pastoral School</title>`);
      });
      $$renderer3.push(`<meta name="description"${attr("content", `Contact ${stringify(SITE.name)} Pastoral School. ${stringify(SITE.infoEmail)}. ${stringify(SITE.address)}, ${stringify(SITE.location)}.`)}/>`);
    });
    $$renderer2.push(`<div class="min-h-screen flex flex-col bg-white">`);
    Nav($$renderer2);
    $$renderer2.push(`<!----> <main class="flex-1"><section class="px-4 sm:px-6 pt-24 sm:pt-32 pb-16 sm:pb-24"><div class="mx-auto max-w-2xl text-center"><h1 class="text-3xl font-semibold tracking-tight text-slate-900 sm:text-4xl">Contact the Pastoral School</h1> <p class="mt-4 text-slate-600">We're here to help with admissions, course enquiries, and visits. Reach out at your convenience.</p></div></section> <section class="px-4 sm:px-6 pb-12 sm:pb-16">`);
    AnimateOnView($$renderer2, {
      animation: "fade-up",
      children: ($$renderer3) => {
        $$renderer3.push(`<div class="mx-auto max-w-xl"><form class="space-y-4 rounded-2xl border border-slate-200/80 bg-slate-50/50 p-6 sm:p-8"><h2 class="text-lg font-semibold text-slate-900">Send us a message</h2> `);
        {
          $$renderer3.push("<!--[!-->");
          {
            $$renderer3.push("<!--[!-->");
          }
          $$renderer3.push(`<!--]--> <div><label for="contact-name" class="block text-sm font-medium text-slate-700">Name</label> <input id="contact-name" type="text"${attr("value", name)} required="" class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500" placeholder="Your name"/></div> <div><label for="contact-email" class="block text-sm font-medium text-slate-700">Email</label> <input id="contact-email" type="email"${attr("value", email)} required="" class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500" placeholder="your@email.com"/></div> <div><label for="contact-subject" class="block text-sm font-medium text-slate-700">Subject (optional)</label> <input id="contact-subject" type="text"${attr("value", subject)} class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500" placeholder="e.g. Course enquiry"/></div> <div><label for="contact-message" class="block text-sm font-medium text-slate-700">Message</label> <textarea id="contact-message" required="" rows="4" class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 focus:border-pink-500 focus:ring-1 focus:ring-pink-500" placeholder="Your message...">`);
          const $$body = escape_html(message);
          if ($$body) {
            $$renderer3.push(`${$$body}`);
          }
          $$renderer3.push(`</textarea></div> <button type="submit"${attr("disabled", loading, true)} class="w-full rounded-xl bg-slate-900 px-6 py-3.5 text-sm font-medium text-white transition hover:bg-slate-800 disabled:opacity-60">${escape_html("Send message")}</button>`);
        }
        $$renderer3.push(`<!--]--></form></div>`);
      }
    });
    $$renderer2.push(`<!----></section> <section class="px-4 sm:px-6 pb-20 sm:pb-28">`);
    AnimateOnView($$renderer2, {
      animation: "fade-up",
      children: ($$renderer3) => {
        $$renderer3.push(`<div class="mx-auto max-w-xl"><div class="space-y-8"><div class="border-b border-slate-200/80 pb-8"><p class="text-xs font-medium uppercase tracking-wider text-slate-400">Email</p> <a${attr("href", `mailto:${stringify(SITE.infoEmail)}`)} class="mt-1 block text-lg text-slate-900 hover:text-pink-600 transition break-all">${escape_html(SITE.infoEmail)}</a></div> <div class="border-b border-slate-200/80 pb-8"><p class="text-xs font-medium uppercase tracking-wider text-slate-400">Phone</p> <a${attr("href", whatsappUrl)} target="_blank" rel="noopener noreferrer" class="mt-1 block text-lg text-slate-900 hover:text-pink-600 transition">${escape_html(SITE.whatsapp)}</a></div> <div class="border-b border-slate-200/80 pb-8"><p class="text-xs font-medium uppercase tracking-wider text-slate-400">Address</p> <p class="mt-1 text-slate-900 leading-relaxed">${escape_html(SITE.address)}</p> <p class="mt-0.5 text-slate-500">${escape_html(SITE.location)}</p> <a${attr("href", mapUrl)} target="_blank" rel="noopener noreferrer" class="mt-3 inline-block text-sm font-medium text-pink-600 hover:text-pink-700">Get directions →</a></div> <div><p class="text-xs font-medium uppercase tracking-wider text-slate-400">Hours</p> <p class="mt-1 text-slate-900">${escape_html(SITE.schedule)}</p> <p class="text-slate-500">${escape_html(SITE.duration)}</p></div></div> <div class="mt-16 text-center"><a href="/register" class="inline-block rounded-full bg-slate-900 px-8 py-3.5 text-sm font-medium text-white transition hover:bg-slate-800">Register now</a></div></div>`);
      }
    });
    $$renderer2.push(`<!----></section></main> `);
    Footer($$renderer2);
    $$renderer2.push(`<!----></div>`);
  });
}
export {
  _page as default
};
