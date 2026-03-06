import { h as head, f as ensure_array_like, i as attr_class, d as stringify, e as escape_html, b as attr } from "../../../chunks/index.js";
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
    const steps = [
      { id: 1, title: "Personal info", desc: "Your contact details" },
      { id: 2, title: "Ministry info", desc: "Your calling & church" },
      { id: 3, title: "Review", desc: "Confirm & submit" }
    ];
    let currentStep = 1;
    let formData = {
      name: "",
      email: "",
      phone: "",
      password: "",
      confirmPassword: ""
    };
    let errors = {};
    let submitting = false;
    head("52fghe", $$renderer2, ($$renderer3) => {
      $$renderer3.title(($$renderer4) => {
        $$renderer4.push(`<title>Register — ${escape_html(SITE.name)} Pastoral School</title>`);
      });
      $$renderer3.push(`<meta name="description" content="Register for the Pastoral School at Prophetic Glorious Manifestation."/>`);
    });
    $$renderer2.push(`<div class="min-h-screen flex flex-col bg-slate-50">`);
    Nav($$renderer2);
    $$renderer2.push(`<!----> <main class="flex-1 pt-20 sm:pt-24 pb-12 sm:pb-16"><div class="mx-auto max-w-5xl px-4 sm:px-6"><div class="mb-10 text-center"><h1 class="text-3xl font-bold tracking-tight text-slate-900 sm:text-4xl">Create your account</h1> <p class="mt-2 text-slate-600">Join our Pastoral School. Complete the form below to get started.</p></div> <div class="flex flex-col overflow-hidden rounded-2xl border border-pink-200/50 bg-white shadow-xl lg:flex-row"><aside class="border-b border-pink-200/50 bg-gradient-to-b from-pink-50/80 to-purple-50/50 px-4 py-6 sm:px-8 sm:py-10 lg:w-72 lg:border-b-0 lg:border-r"><nav class="flex flex-col gap-1" aria-label="Registration steps"><!--[-->`);
    const each_array = ensure_array_like(steps);
    for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
      let step = each_array[$$index];
      $$renderer2.push(`<button type="button"${attr_class(`flex items-center gap-4 rounded-xl px-4 py-3 text-left transition-all duration-200 ${stringify(currentStep === step.id ? "bg-white shadow-md ring-1 ring-pink-200/60" : currentStep > step.id ? "text-pink-600" : "text-slate-500 hover:bg-white/60")}`)}><span${attr_class(`flex h-9 w-9 shrink-0 items-center justify-center rounded-full text-sm font-semibold ${stringify(currentStep === step.id ? "bg-gradient-to-br from-pink-500 to-purple-600 text-white" : currentStep > step.id ? "bg-pink-100 text-pink-600" : "bg-slate-200 text-slate-500")}`)}>`);
      if (currentStep > step.id) {
        $$renderer2.push("<!--[-->");
        $$renderer2.push(`<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path></svg>`);
      } else {
        $$renderer2.push("<!--[!-->");
        $$renderer2.push(`${escape_html(step.id)}`);
      }
      $$renderer2.push(`<!--]--></span> <div><p class="font-medium text-slate-900">${escape_html(step.title)}</p> <p class="text-xs text-slate-500">${escape_html(step.desc)}</p></div></button>`);
    }
    $$renderer2.push(`<!--]--></nav></aside> <div class="flex-1 p-4 sm:p-6 lg:p-12">`);
    {
      $$renderer2.push("<!--[-->");
      $$renderer2.push(`<div class="space-y-6"><h2 class="text-xl font-semibold text-slate-900">Personal information</h2> <div><label for="name" class="block text-sm font-medium text-slate-700">Full name</label> <input id="name" type="text"${attr("value", formData.name)}${attr_class(`mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500 ${stringify(errors.name ? "border-red-300" : "")}`)} placeholder="John Doe"/> `);
      if (errors.name) {
        $$renderer2.push("<!--[-->");
        $$renderer2.push(`<p class="mt-1 text-sm text-red-600">${escape_html(errors.name)}</p>`);
      } else {
        $$renderer2.push("<!--[!-->");
      }
      $$renderer2.push(`<!--]--></div> <div><label for="email" class="block text-sm font-medium text-slate-700">Email</label> <input id="email" type="email"${attr("value", formData.email)}${attr_class(`mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500 ${stringify(errors.email ? "border-red-300" : "")}`)} placeholder="John@example.com"/> `);
      if (errors.email) {
        $$renderer2.push("<!--[-->");
        $$renderer2.push(`<p class="mt-1 text-sm text-red-600">${escape_html(errors.email)}</p>`);
      } else {
        $$renderer2.push("<!--[!-->");
      }
      $$renderer2.push(`<!--]--></div> <div><label for="phone" class="block text-sm font-medium text-slate-700">Phone number</label> <input id="phone" type="tel"${attr("value", formData.phone)}${attr_class(`mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500 ${stringify(errors.phone ? "border-red-300" : "")}`)} placeholder="080 1234 5678"/> `);
      if (errors.phone) {
        $$renderer2.push("<!--[-->");
        $$renderer2.push(`<p class="mt-1 text-sm text-red-600">${escape_html(errors.phone)}</p>`);
      } else {
        $$renderer2.push("<!--[!-->");
      }
      $$renderer2.push(`<!--]--></div> <div><label for="password" class="block text-sm font-medium text-slate-700">Password</label> <input id="password" type="password"${attr("value", formData.password)}${attr_class(`mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500 ${stringify(errors.password ? "border-red-300" : "")}`)} placeholder="At least 8 characters"/> `);
      if (errors.password) {
        $$renderer2.push("<!--[-->");
        $$renderer2.push(`<p class="mt-1 text-sm text-red-600">${escape_html(errors.password)}</p>`);
      } else {
        $$renderer2.push("<!--[!-->");
      }
      $$renderer2.push(`<!--]--></div> <div><label for="confirmPassword" class="block text-sm font-medium text-slate-700">Confirm password</label> <input id="confirmPassword" type="password"${attr("value", formData.confirmPassword)}${attr_class(`mt-1 block w-full rounded-xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm transition focus:border-pink-500 focus:ring-1 focus:ring-pink-500 ${stringify(errors.confirmPassword ? "border-red-300" : "")}`)} placeholder="Re-enter your password"/> `);
      if (errors.confirmPassword) {
        $$renderer2.push("<!--[-->");
        $$renderer2.push(`<p class="mt-1 text-sm text-red-600">${escape_html(errors.confirmPassword)}</p>`);
      } else {
        $$renderer2.push("<!--[!-->");
      }
      $$renderer2.push(`<!--]--></div></div>`);
    }
    $$renderer2.push(`<!--]--> <div class="mt-6 sm:mt-10 flex items-center justify-between gap-4 border-t border-slate-200 pt-6 sm:pt-8"><button type="button"${attr_class(`rounded-xl px-5 py-2.5 text-sm font-medium text-slate-600 transition hover:bg-slate-100 ${stringify("invisible")}`)}>Back</button> <button type="button"${attr("disabled", submitting, true)} class="rounded-xl bg-gradient-to-r from-pink-500 to-purple-600 px-6 py-2.5 text-sm font-semibold text-white shadow-lg transition hover:from-pink-600 hover:to-purple-700 disabled:opacity-60 disabled:cursor-not-allowed">${escape_html("Continue")}</button></div></div></div></div></main> `);
    Footer($$renderer2);
    $$renderer2.push(`<!----></div>`);
  });
}
export {
  _page as default
};
