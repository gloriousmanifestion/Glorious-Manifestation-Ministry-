import { s as store_get, h as head, a as unsubscribe_stores, e as escape_html } from "../../../chunks/index.js";
import "@sveltejs/kit/internal";
import "../../../chunks/exports.js";
import "../../../chunks/utils2.js";
import "@sveltejs/kit/internal/server";
import "../../../chunks/root.js";
import "../../../chunks/state.svelte.js";
import { S as SITE } from "../../../chunks/config.js";
import { u as userStore } from "../../../chunks/auth.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    var $$store_subs;
    let courses = [];
    const user = store_get($$store_subs ??= {}, "$userStore", userStore);
    user && !user.verified;
    user && !user.paymentComplete;
    user?.selectedCourseId ? courses.find((c) => c.id === user.selectedCourseId) : null;
    head("x1i5gj", $$renderer2, ($$renderer3) => {
      $$renderer3.title(($$renderer4) => {
        $$renderer4.push(`<title>Dashboard — ${escape_html(SITE.name)} Pastoral School</title>`);
      });
      $$renderer3.push(`<meta name="description" content="Your Pastoral School dashboard."/>`);
    });
    {
      $$renderer2.push("<!--[-->");
      $$renderer2.push(`<div class="flex min-h-screen items-center justify-center bg-slate-50"><div class="h-8 w-8 animate-spin rounded-full border-2 border-pink-500 border-t-transparent"></div></div>`);
    }
    $$renderer2.push(`<!--]-->`);
    if ($$store_subs) unsubscribe_stores($$store_subs);
  });
}
export {
  _page as default
};
