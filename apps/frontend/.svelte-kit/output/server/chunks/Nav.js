import { s as store_get, i as attr_class, e as escape_html, f as ensure_array_like, b as attr, d as stringify, a as unsubscribe_stores } from "./index.js";
import { p as page } from "./stores.js";
import { S as SITE } from "./config.js";
import { u as userStore } from "./auth.js";
function Nav($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    var $$store_subs;
    const navLinks = [
      { href: "/", label: "Home" },
      { href: "/programs", label: "Programs" },
      { href: "/admissions", label: "Admissions" },
      { href: "/events", label: "Events" },
      { href: "/contact", label: "Contact" }
    ];
    let mobileOpen = false;
    const user = store_get($$store_subs ??= {}, "$userStore", userStore);
    const isLoggedIn = user !== null;
    const pathname = store_get($$store_subs ??= {}, "$page", page).url.pathname;
    function isActive(href) {
      if (href === "/") return pathname === "/";
      return pathname === href || pathname.startsWith(href + "/");
    }
    $$renderer2.push(`<header${attr_class(`fixed top-0 left-0 right-0 z-50 pt-[env(safe-area-inset-top)] transition-all duration-300 ${stringify("border-b border-transparent bg-transparent")}`)}><nav class="mx-auto flex max-w-6xl items-center justify-between px-4 py-3 sm:px-6 sm:py-4"><a href="/" class="inline-flex items-center gap-2 text-xl font-semibold tracking-tight text-slate-900 transition-all duration-300 hover:text-pink-600"><img src="/favicon.png" alt="" class="h-9 w-9 shrink-0 rounded-full object-cover" aria-hidden="true"/> <span class="sm:hidden">${escape_html(SITE.schoolName)}</span> <span class="hidden sm:inline">PGMM | ${escape_html(SITE.schoolName)}</span></a> <div class="hidden items-center gap-1 md:flex"><!--[-->`);
    const each_array = ensure_array_like(navLinks);
    for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
      let link = each_array[$$index];
      $$renderer2.push(`<a${attr("href", link.href)}${attr_class(`rounded-lg px-4 py-2 text-sm font-medium transition-all duration-300 ${stringify(isActive(link.href) ? "bg-pink-50 text-pink-600" : "text-slate-600 hover:bg-pink-50 hover:text-pink-600")}`)}${attr("aria-current", isActive(link.href) ? "page" : void 0)}>${escape_html(link.label)}</a>`);
    }
    $$renderer2.push(`<!--]--> `);
    if (isLoggedIn) {
      $$renderer2.push("<!--[-->");
      $$renderer2.push(`<a href="/dashboard"${attr_class(`ml-2 rounded-full px-5 py-2.5 text-sm font-semibold transition ${stringify(isActive("/dashboard") ? "border-2 border-pink-500 bg-pink-100 text-pink-700" : "border border-pink-300 bg-pink-50 text-pink-600 hover:bg-pink-100")}`)}${attr("aria-current", isActive("/dashboard") ? "page" : void 0)}>Dashboard</a>`);
    } else {
      $$renderer2.push("<!--[!-->");
      $$renderer2.push(`<a href="/login" class="ml-2 rounded-lg px-4 py-2.5 text-sm font-medium text-slate-600 transition hover:bg-pink-50 hover:text-pink-600">Log in</a> <a href="/register" class="ml-2 rounded-full border border-pink-300 bg-pink-50 px-5 py-2.5 text-sm font-semibold text-pink-600 transition hover:bg-pink-100">Register</a>`);
    }
    $$renderer2.push(`<!--]--></div> <button type="button" class="rounded-lg p-3 -m-1 text-slate-600 hover:bg-pink-50 hover:text-pink-600 active:bg-pink-100 md:hidden touch-manipulation"${attr("aria-expanded", mobileOpen)} aria-controls="mobile-menu">`);
    {
      $$renderer2.push("<!--[!-->");
      $$renderer2.push(`<svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>`);
    }
    $$renderer2.push(`<!--]--></button></nav> `);
    {
      $$renderer2.push("<!--[!-->");
    }
    $$renderer2.push(`<!--]--></header>`);
    if ($$store_subs) unsubscribe_stores($$store_subs);
  });
}
export {
  Nav as N
};
