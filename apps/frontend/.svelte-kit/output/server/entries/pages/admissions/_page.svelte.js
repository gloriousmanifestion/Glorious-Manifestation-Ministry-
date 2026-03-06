import { h as head, e as escape_html, f as ensure_array_like } from "../../../chunks/index.js";
import { N as Nav } from "../../../chunks/Nav.js";
import { F as Footer } from "../../../chunks/Footer.js";
import { A as AnimateOnView } from "../../../chunks/AnimateOnView.js";
import { S as SITE } from "../../../chunks/config.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    const requirements = [
      "A genuine call to pastoral, ministerial, or prophetic ministry",
      "Commitment to attend classes Monday to Friday",
      "Basic literacy and ability to study",
      "Recommendation from a church or ministry leader (optional but preferred)",
      "Heart for serving God and His people"
    ];
    const steps = [
      {
        num: 1,
        title: "Register",
        desc: "Create your account with our simple registration form."
      },
      {
        num: 2,
        title: "Verify account",
        desc: "Verify your email to activate your account."
      },
      {
        num: 3,
        title: "Choose course & pay",
        desc: "Select your program and complete payment."
      },
      {
        num: 4,
        title: "Begin classes",
        desc: "Start your journey in February and complete by July."
      }
    ];
    head("1cypryr", $$renderer2, ($$renderer3) => {
      $$renderer3.title(($$renderer4) => {
        $$renderer4.push(`<title>Admissions — ${escape_html(SITE.name)} Pastoral School</title>`);
      });
      $$renderer3.push(`<meta name="description" content="Apply to the Pastoral School at Prophetic Glorious Manifestation. For pastors, ministers and prophets. February to July, Owerri, Imo State."/>`);
    });
    $$renderer2.push(`<div class="min-h-screen flex flex-col">`);
    Nav($$renderer2);
    $$renderer2.push(`<!----> <main class="flex-1"><section class="gradient-mesh px-4 sm:px-6 pt-24 sm:pt-28 pb-12 sm:pb-16 text-center"><p class="hero-enter-delay-1 mb-4 text-sm font-medium uppercase tracking-[0.2em] text-pink-600">Apply now</p> <h1 class="hero-enter-delay-2 mx-auto max-w-3xl text-3xl font-bold tracking-tight text-slate-900 sm:text-4xl md:text-5xl">Admissions</h1> <p class="hero-enter-delay-3 mx-auto mt-6 max-w-2xl text-lg text-slate-600">${escape_html(SITE.schoolName)} is now open for pastors, ministers and prophets. Program runs ${escape_html(SITE.duration)}, ${escape_html(SITE.schedule)}.</p></section> <section class="border-t border-pink-200/50 bg-white/60 py-12 sm:py-16 px-4 sm:px-6"><div class="mx-auto max-w-3xl">`);
    AnimateOnView($$renderer2, {
      animation: "fade-up",
      children: ($$renderer3) => {
        $$renderer3.push(`<h2 class="text-xl font-bold tracking-tight text-slate-900 sm:text-2xl">Program details</h2>`);
      }
    });
    $$renderer2.push(`<!----> <div class="mt-6 grid gap-4 grid-cols-1 sm:grid-cols-3">`);
    AnimateOnView($$renderer2, {
      animation: "fade-up",
      stagger: 0,
      children: ($$renderer3) => {
        $$renderer3.push(`<div class="hover-lift rounded-xl border border-pink-200/50 bg-white p-4"><p class="text-sm font-medium text-pink-600">Duration</p> <p class="mt-1 font-semibold text-slate-900">${escape_html(SITE.duration)}</p></div>`);
      }
    });
    $$renderer2.push(`<!----> `);
    AnimateOnView($$renderer2, {
      animation: "fade-up",
      stagger: 1,
      children: ($$renderer3) => {
        $$renderer3.push(`<div class="hover-lift rounded-xl border border-pink-200/50 bg-white p-4"><p class="text-sm font-medium text-pink-600">Schedule</p> <p class="mt-1 font-semibold text-slate-900">${escape_html(SITE.schedule)}</p></div>`);
      }
    });
    $$renderer2.push(`<!----> `);
    AnimateOnView($$renderer2, {
      animation: "fade-up",
      stagger: 2,
      children: ($$renderer3) => {
        $$renderer3.push(`<div class="hover-lift rounded-xl border border-pink-200/50 bg-white p-4"><p class="text-sm font-medium text-pink-600">Location</p> <p class="mt-1 font-semibold text-slate-900">${escape_html(SITE.location)}</p></div>`);
      }
    });
    $$renderer2.push(`<!----></div></div></section> <section class="border-t border-pink-200/50 bg-gradient-to-b from-pink-50/50 to-purple-50/30 py-12 sm:py-16 px-4 sm:px-6"><div class="mx-auto max-w-3xl">`);
    AnimateOnView($$renderer2, {
      animation: "fade-up",
      children: ($$renderer3) => {
        $$renderer3.push(`<h2 class="text-2xl font-bold tracking-tight text-slate-900">Requirements</h2>`);
      }
    });
    $$renderer2.push(`<!----> <ul class="mt-6 space-y-3"><!--[-->`);
    const each_array = ensure_array_like(requirements);
    for (let i = 0, $$length = each_array.length; i < $$length; i++) {
      let req = each_array[i];
      AnimateOnView($$renderer2, {
        animation: "slide-right",
        stagger: i,
        duration: 400,
        children: ($$renderer3) => {
          $$renderer3.push(`<li class="flex items-start gap-3"><span class="mt-1 flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-pink-500 text-xs font-bold text-white">✓</span> <span class="text-slate-600">${escape_html(req)}</span></li>`);
        }
      });
    }
    $$renderer2.push(`<!--]--></ul></div></section> <section class="border-t border-pink-200/50 bg-white/60 py-12 sm:py-16 px-4 sm:px-6"><div class="mx-auto max-w-3xl">`);
    AnimateOnView($$renderer2, {
      animation: "fade-up",
      children: ($$renderer3) => {
        $$renderer3.push(`<h2 class="text-xl font-bold tracking-tight text-slate-900 sm:text-2xl">How to apply</h2>`);
      }
    });
    $$renderer2.push(`<!----> <div class="mt-6 sm:mt-8 space-y-6"><!--[-->`);
    const each_array_1 = ensure_array_like(steps);
    for (let i = 0, $$length = each_array_1.length; i < $$length; i++) {
      let step = each_array_1[i];
      AnimateOnView($$renderer2, {
        animation: "fade-up",
        stagger: i,
        children: ($$renderer3) => {
          $$renderer3.push(`<div class="flex flex-col sm:flex-row gap-4"><span class="flex h-12 w-12 shrink-0 items-center justify-center rounded-full bg-gradient-to-br from-pink-500 to-purple-600 text-lg font-bold text-white">${escape_html(step.num)}</span> <div><h3 class="font-semibold text-slate-900">${escape_html(step.title)}</h3> <p class="mt-1 text-slate-600">${escape_html(step.desc)}</p></div></div>`);
        }
      });
    }
    $$renderer2.push(`<!--]--></div></div></section> <section class="border-t border-pink-200/50 bg-gradient-to-r from-pink-500/10 to-purple-500/10 py-16 sm:py-20 px-4 sm:px-6">`);
    AnimateOnView($$renderer2, {
      animation: "scale",
      children: ($$renderer3) => {
        $$renderer3.push(`<div class="mx-auto max-w-2xl text-center"><h2 class="text-2xl font-bold tracking-tight text-slate-900">Ready to apply?</h2> <p class="mt-4 text-slate-600">Create your account and complete registration. You'll verify and pay from your dashboard.</p> <a href="/register" class="btn-scale mt-8 inline-flex items-center gap-2 rounded-full bg-gradient-to-r from-pink-500 to-purple-600 px-10 py-4 text-base font-semibold text-white shadow-lg transition hover:from-pink-600 hover:to-purple-700">Create account</a> <p class="mt-4 text-sm text-slate-500">Questions? <a href="https://wa.me/2348026954759" target="_blank" rel="noopener noreferrer" class="font-medium text-pink-600 hover:underline">Contact us on WhatsApp</a></p></div>`);
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
