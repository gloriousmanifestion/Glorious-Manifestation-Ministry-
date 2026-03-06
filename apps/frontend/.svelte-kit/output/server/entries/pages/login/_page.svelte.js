import { h as head, b as attr, e as escape_html } from "../../../chunks/index.js";
import "@sveltejs/kit/internal";
import "../../../chunks/exports.js";
import "../../../chunks/utils2.js";
import "@sveltejs/kit/internal/server";
import "../../../chunks/root.js";
import "../../../chunks/state.svelte.js";
import { N as Nav } from "../../../chunks/Nav.js";
import { F as Footer } from "../../../chunks/Footer.js";
import "../../../chunks/auth.js";
import { S as SITE } from "../../../chunks/config.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let email = "";
    let password = "";
    let loading = false;
    head("1x05zx6", $$renderer2, ($$renderer3) => {
      $$renderer3.title(($$renderer4) => {
        $$renderer4.push(`<title>Log in — ${escape_html(SITE.name)} Pastoral School</title>`);
      });
      $$renderer3.push(`<meta name="description" content="Log in to your Pastoral School dashboard."/>`);
    });
    $$renderer2.push(`<div class="min-h-screen flex flex-col bg-slate-50">`);
    Nav($$renderer2);
    $$renderer2.push(`<!----> <main class="flex-1 pt-20 sm:pt-24 pb-12 sm:pb-16"><div class="mx-auto max-w-md px-4 sm:px-6"><div class="rounded-2xl border border-pink-200/50 bg-white p-6 sm:p-8 shadow-xl"><h1 class="text-2xl font-bold tracking-tight text-slate-900">Log in</h1> <p class="mt-2 text-slate-600">Sign in to access your Pastoral School dashboard.</p> <form class="mt-8 space-y-6">`);
    {
      $$renderer2.push("<!--[!-->");
    }
    $$renderer2.push(`<!--]--> <div><label for="email" class="block text-sm font-medium text-slate-700">Email</label> <input id="email" type="email"${attr("value", email)} required="" class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500" placeholder="you@example.com"/></div> <div><label for="password" class="block text-sm font-medium text-slate-700">Password</label> <input id="password" type="password"${attr("value", password)} required="" class="mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500" placeholder="Your password"/></div> <button type="submit"${attr("disabled", loading, true)} class="w-full rounded-xl bg-gradient-to-r from-pink-500 to-purple-600 px-6 py-3 text-sm font-semibold text-white shadow-lg transition hover:from-pink-600 hover:to-purple-700 disabled:opacity-60">${escape_html("Sign in")}</button></form> <p class="mt-6 text-center text-sm text-slate-600">Don't have an account? <a href="/register" class="font-medium text-pink-600 hover:text-pink-500">Register</a></p></div></div></main> `);
    Footer($$renderer2);
    $$renderer2.push(`<!----></div>`);
  });
}
export {
  _page as default
};
