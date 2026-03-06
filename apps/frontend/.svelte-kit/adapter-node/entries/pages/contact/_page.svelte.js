import { h as head, a0 as attr, a1 as ensure_array_like, a2 as attr_class, a3 as stringify, e as escape_html } from "../../../chunks/index.js";
/* empty css                  */
import { p as public_env } from "../../../chunks/shared-server.js";
import { p as plusUrl } from "../../../chunks/urls.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    (public_env.PUBLIC_API_URL || "").replace(/\/$/, "");
    let formStatus = "idle";
    const offices = [
      {
        country: "South Africa",
        flag: "🇿🇦",
        status: "registered",
        detail: "Headquarters",
        address: "Johannesburg, South Africa"
      },
      {
        country: "Malawi",
        flag: "🇲🇼",
        status: "registered",
        detail: "Full operations",
        address: "Lilongwe, Malawi"
      },
      {
        country: "Mozambique",
        flag: "🇲🇿",
        status: "coming",
        detail: "Pending registration",
        address: "Maputo, Mozambique"
      },
      {
        country: "Zimbabwe",
        flag: "🇿🇼",
        status: "coming",
        detail: "Coming soon",
        address: "Harare, Zimbabwe"
      },
      {
        country: "Tanzania",
        flag: "🇹🇿",
        status: "coming",
        detail: "Coming soon",
        address: "Dar es Salaam, Tanzania"
      },
      {
        country: "Zambia",
        flag: "🇿🇲",
        status: "coming",
        detail: "Coming soon",
        address: "Lusaka, Zambia"
      }
    ];
    head("1bv7ezn", $$renderer2, ($$renderer3) => {
      $$renderer3.title(($$renderer4) => {
        $$renderer4.push(`<title>Contact — 1nga Solutions</title>`);
      });
      $$renderer3.push(`<meta name="description" content="Contact 1nga Solutions. The digital transformation hub—registered in South Africa, Malawi, and expanding across Southern and East Africa."/>`);
    });
    $$renderer2.push(`<div class="min-h-screen flex flex-col bg-[#FFFCFA]"><header class="sticky top-0 left-0 right-0 z-50 border-b border-slate-200 bg-[#FFFCFA]/90 backdrop-blur-xl"><nav class="mx-auto flex max-w-6xl items-center justify-between px-6 py-4"><a href="/" class="inline-flex items-center gap-2 text-xl font-semibold tracking-tight text-slate-900 transition hover:text-amber-600"><img src="/oneauth-logo.svg" alt="" class="h-7 w-auto" aria-hidden="true"/> <span>1nga Solutions</span></a> <a${attr("href", plusUrl)} target="_blank" rel="noopener noreferrer" class="rounded-full border border-slate-300 bg-transparent px-5 py-2.5 text-sm font-medium text-slate-700 transition hover:border-amber-500/60 hover:text-amber-600">Sign in</a></nav></header> <main class="flex-1"><section class="relative px-6 pt-16 pb-12 text-center"><p class="mb-4 text-sm font-medium uppercase tracking-[0.2em] text-slate-500">Contact</p> <h1 class="mx-auto max-w-3xl text-4xl font-bold tracking-tight text-slate-900 sm:text-5xl">Get in touch</h1> <p class="mx-auto mt-6 max-w-2xl text-lg text-slate-600">Reach out to 1nga Solutions and our subsidiaries. We're here to help with your digital transformation journey.</p></section> <section class="border-t border-slate-200 bg-[#FFF8F3]/60 py-16 px-6"><div class="mx-auto max-w-3xl"><h2 class="text-2xl font-bold tracking-tight text-slate-900">About 1nga Solutions</h2> <div class="mt-6 space-y-4 text-slate-600"><p>1nga started in <strong class="text-slate-900">2025</strong> as a commerce platform. By the end of 2025, we had transitioned to a full-fledged IT solutions provider—the digital transformation hub we are today.</p> <p>We power payments, learning, agriculture, energy, B2B, and more—unified under one identity and one platform.</p></div></div></section> <section class="border-t border-slate-200 py-20 px-6"><div class="mx-auto max-w-6xl"><h2 class="text-center text-2xl font-bold tracking-tight text-slate-900 sm:text-3xl">Where we operate</h2> <p class="mx-auto mt-4 max-w-2xl text-center text-slate-600">Expanding across Southern and East Africa</p> <div class="mt-16 grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3"><!--[-->`);
    const each_array = ensure_array_like(offices);
    for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
      let office = each_array[$$index];
      $$renderer2.push(`<article${attr_class(`group relative flex flex-col overflow-hidden rounded-2xl bg-white p-6 shadow-[0_4px_24px_-4px_rgba(15,23,42,0.08),0_8px_16px_-8px_rgba(15,23,42,0.04)] ring-1 ring-slate-200/60 transition-all duration-300 hover:-translate-y-1 hover:shadow-[0_12px_40px_-8px_rgba(15,23,42,0.12),0_16px_24px_-12px_rgba(15,23,42,0.08)] hover:ring-slate-300/80 ${stringify(office.status === "registered" ? "ring-amber-500/20 hover:ring-amber-500/30" : "")}`)}><div class="flex items-start justify-between gap-4"><div${attr_class(`flex h-14 w-14 shrink-0 items-center justify-center rounded-2xl text-3xl shadow-sm ${stringify(office.status === "registered" ? "bg-amber-500/15 ring-1 ring-amber-500/20" : "bg-slate-100 ring-1 ring-slate-200/60")}`)} aria-hidden="true">${escape_html(office.flag)}</div> <span${attr_class(`shrink-0 rounded-full px-3 py-1 text-xs font-medium ${stringify(office.status === "registered" ? "bg-amber-500/15 text-amber-700 ring-1 ring-amber-500/20" : "bg-slate-100 text-slate-600 ring-1 ring-slate-200/60")}`)}>${escape_html(office.status === "registered" ? "Registered" : "Coming soon")}</span></div> <h3 class="mt-5 text-xl font-bold tracking-tight text-slate-900">${escape_html(office.country)}</h3> <p${attr_class(`mt-1 text-sm font-medium ${stringify(office.status === "registered" ? "text-amber-600" : "text-slate-500")}`)}>${escape_html(office.detail)}</p> <div class="mt-4 flex items-start gap-2"><svg class="mt-0.5 h-4 w-4 shrink-0 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"></path></svg> <p class="text-sm leading-relaxed text-slate-600">${escape_html(office.address)}</p></div></article>`);
    }
    $$renderer2.push(`<!--]--></div></div></section> <section class="border-t border-slate-200 bg-[#FFF4ED]/40 py-16 px-6"><div class="mx-auto max-w-4xl"><div class="grid gap-12 lg:grid-cols-2"><div><h2 class="text-2xl font-bold tracking-tight text-slate-900">Send a message</h2> <p class="mt-2 text-slate-600">Fill out the form and we'll get back to you as soon as possible.</p> <form action="#" method="get" class="mt-6 space-y-4"><div><label for="name" class="block text-sm font-medium text-slate-700">Name</label> <input type="text" id="name" name="name" required="" class="mt-1 block w-full rounded-xl border border-slate-200 bg-white px-4 py-3 text-slate-900 shadow-sm transition focus:border-amber-500 focus:outline-none focus:ring-1 focus:ring-amber-500" placeholder="Your name"/></div> <div><label for="email" class="block text-sm font-medium text-slate-700">Email</label> <input type="email" id="email" name="email" required="" class="mt-1 block w-full rounded-xl border border-slate-200 bg-white px-4 py-3 text-slate-900 shadow-sm transition focus:border-amber-500 focus:outline-none focus:ring-1 focus:ring-amber-500" placeholder="you@example.com"/></div> <div><label for="subject" class="block text-sm font-medium text-slate-700">Subject</label> <select id="subject" name="subject" class="mt-1 block w-full rounded-xl border border-slate-200 bg-white px-4 py-3 text-slate-900 shadow-sm transition focus:border-amber-500 focus:outline-none focus:ring-1 focus:ring-amber-500">`);
    $$renderer2.option({ value: "general" }, ($$renderer3) => {
      $$renderer3.push(`General enquiry`);
    });
    $$renderer2.option({ value: "partnership" }, ($$renderer3) => {
      $$renderer3.push(`Partnership`);
    });
    $$renderer2.option({ value: "support" }, ($$renderer3) => {
      $$renderer3.push(`Support`);
    });
    $$renderer2.option({ value: "careers" }, ($$renderer3) => {
      $$renderer3.push(`Careers`);
    });
    $$renderer2.option({ value: "other" }, ($$renderer3) => {
      $$renderer3.push(`Other`);
    });
    $$renderer2.push(`</select></div> <div><label for="message" class="block text-sm font-medium text-slate-700">Message</label> <textarea id="message" name="message" required="" rows="4" class="mt-1 block w-full rounded-xl border border-slate-200 bg-white px-4 py-3 text-slate-900 shadow-sm transition focus:border-amber-500 focus:outline-none focus:ring-1 focus:ring-amber-500" placeholder="How can we help?"></textarea></div> `);
    {
      $$renderer2.push("<!--[!-->");
    }
    $$renderer2.push(`<!--]--> <button type="submit"${attr("disabled", formStatus === "submitting", true)} class="w-full rounded-full bg-amber-500 px-6 py-3 text-sm font-semibold text-black transition hover:bg-amber-400 focus:outline-none focus:ring-2 focus:ring-amber-400 focus:ring-offset-2 disabled:opacity-60 disabled:cursor-not-allowed">${escape_html("Send message")}</button></form></div> <div><h2 class="text-2xl font-bold tracking-tight text-slate-900">Or email us directly</h2> <p class="mt-2 text-slate-600">For general enquiries, partnerships, or support.</p> <div class="mt-6 space-y-4"><a href="mailto:hello@1nga.com" class="flex items-center gap-3 rounded-xl border border-slate-200 bg-white p-4 text-slate-700 transition hover:border-amber-500/50 hover:text-amber-600"><div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-amber-500/10"><svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"></path></svg></div> <div><p class="font-medium text-slate-900">General</p> <p class="text-sm">hello@1nga.com</p></div></a> <a href="mailto:plus@1nga.com" class="flex items-center gap-3 rounded-xl border border-slate-200 bg-white p-4 text-slate-700 transition hover:border-amber-500/50 hover:text-amber-600"><div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-amber-500/10"><svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path></svg></div> <div><p class="font-medium text-slate-900">1nga Plus</p> <p class="text-sm">plus@1nga.com</p></div></a> <a href="mailto:careers@1nga.com" class="flex items-center gap-3 rounded-xl border border-slate-200 bg-white p-4 text-slate-700 transition hover:border-amber-500/50 hover:text-amber-600"><div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-amber-500/10"><svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2 2v2m4 6h.01M5 20h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"></path></svg></div> <div><p class="font-medium text-slate-900">Careers</p> <p class="text-sm">careers@1nga.com</p></div></a> <a href="mailto:business@1nga.com" class="flex items-center gap-3 rounded-xl border border-slate-200 bg-white p-4 text-slate-700 transition hover:border-amber-500/50 hover:text-amber-600"><div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-amber-500/10"><svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"></path></svg></div> <div><p class="font-medium text-slate-900">Business &amp; partnerships</p> <p class="text-sm">business@1nga.com</p></div></a></div></div></div></div></section></main> <footer class="border-t border-slate-200 bg-white/60 py-8 px-6"><div class="mx-auto flex max-w-6xl flex-col items-center justify-between gap-4 sm:flex-row"><div class="inline-flex items-center gap-2"><img src="/oneauth-logo.svg" alt="" class="h-5 w-auto" aria-hidden="true"/> <span class="text-sm text-slate-500">© ${escape_html((/* @__PURE__ */ new Date()).getFullYear())} 1nga Solutions. All rights reserved.</span></div> <div class="flex flex-wrap items-center justify-center gap-6 text-sm"><a href="/contact" class="text-slate-500 transition hover:text-slate-700">Contact</a> <a href="/careers" class="text-slate-500 transition hover:text-slate-700">Careers</a> <a href="/terms" class="text-slate-500 transition hover:text-slate-700">Terms</a> <a href="/privacy" class="text-slate-500 transition hover:text-slate-700">Privacy</a> <a href="/cookies" class="text-slate-500 transition hover:text-slate-700">Cookies</a></div></div></footer></div>`);
  });
}
export {
  _page as default
};
