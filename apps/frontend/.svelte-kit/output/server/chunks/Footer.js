import { e as escape_html, f as ensure_array_like, b as attr, d as stringify } from "./index.js";
import { S as SITE } from "./config.js";
function Footer($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    const quickLinks = [
      { href: "/programs", label: "Programs" },
      { href: "/admissions", label: "Admissions" },
      { href: "/events", label: "Events" },
      { href: "/contact", label: "Contact" }
    ];
    const legalLinks = [
      { href: "/privacy", label: "Privacy" },
      { href: "/terms", label: "Terms" },
      { href: "/cookies", label: "Cookies" }
    ];
    $$renderer2.push(`<footer class="border-t border-slate-200/80 bg-slate-50"><div class="mx-auto max-w-6xl px-4 py-12 sm:px-6 lg:px-8"><div class="grid gap-10 sm:grid-cols-2 lg:grid-cols-3"><div class="sm:col-span-2 lg:col-span-1 min-w-0"><a href="/" class="inline-flex items-center gap-3"><div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-gradient-to-br from-pink-500 to-purple-600 text-white text-sm font-bold shadow-lg shadow-pink-500/20" aria-hidden="true">P</div> <div><span class="block font-semibold text-slate-900">${escape_html(SITE.schoolName)}</span> <span class="block text-xs text-slate-500">${escape_html(SITE.name)}</span></div></a> <p class="mt-4 max-w-xs text-sm leading-relaxed text-slate-600">Equipping pastors, ministers and prophets for effective ministry. ${escape_html(SITE.duration)}, ${escape_html(SITE.schedule)}.</p></div> <div class="min-w-0"><h3 class="text-sm font-semibold uppercase tracking-wider text-slate-900">Quick links</h3> <ul class="mt-4 space-y-3"><!--[-->`);
    const each_array = ensure_array_like(quickLinks);
    for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
      let link = each_array[$$index];
      $$renderer2.push(`<li><a${attr("href", link.href)} class="text-sm text-slate-600 transition hover:text-pink-600">${escape_html(link.label)}</a></li>`);
    }
    $$renderer2.push(`<!--]--></ul></div> <div class="min-w-0"><h3 class="text-sm font-semibold uppercase tracking-wider text-slate-900">Contact</h3> <address class="mt-4 not-italic"><p class="text-sm text-slate-600 break-words">${escape_html(SITE.address)}</p> <p class="mt-1 text-sm text-slate-500">${escape_html(SITE.location)}</p> <div class="mt-4 space-y-2"><a${attr("href", `https://wa.me/${SITE.whatsappFull.replace("+", "")}`)} target="_blank" rel="noopener noreferrer" class="flex items-center gap-2 text-sm text-slate-600 transition hover:text-[#25d366]"><svg class="h-4 w-4 shrink-0" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51-.173-.008-.371-.01-.57-.01-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 01-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 01-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 012.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0012.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 005.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 00-3.48-8.413z"></path></svg> ${escape_html(SITE.whatsapp)}</a> <a${attr("href", `mailto:${stringify(SITE.infoEmail)}`)} class="block text-sm text-slate-600 transition hover:text-pink-600 break-words">${escape_html(SITE.infoEmail)}</a></div></address></div></div> <div class="mt-12 flex flex-col items-center justify-between gap-4 border-t border-slate-200/80 pt-8 sm:flex-row"><p class="text-sm text-slate-500">© ${escape_html((/* @__PURE__ */ new Date()).getFullYear())} ${escape_html(SITE.legalName)}. All rights reserved.</p> <div class="flex flex-wrap items-center justify-center gap-6 text-sm"><!--[-->`);
    const each_array_1 = ensure_array_like(legalLinks);
    for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
      let link = each_array_1[$$index_1];
      $$renderer2.push(`<a${attr("href", link.href)} class="text-slate-500 transition hover:text-slate-700">${escape_html(link.label)}</a>`);
    }
    $$renderer2.push(`<!--]--></div></div> <p class="mt-4 text-center text-xs text-slate-400 pb-[max(2rem,calc(2rem+env(safe-area-inset-bottom)))]">Powered by <a href="https://www.1nga.com" target="_blank" rel="noopener noreferrer" class="text-slate-500 transition hover:text-slate-700">1nga Solutions</a></p></div></footer>`);
  });
}
export {
  Footer as F
};
