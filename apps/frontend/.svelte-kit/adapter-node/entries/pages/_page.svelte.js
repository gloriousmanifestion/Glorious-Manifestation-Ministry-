import { h as head, a0 as attr, e as escape_html, a1 as ensure_array_like } from "../../chunks/index.js";
import { p as plusUrl, b as businessUrl } from "../../chunks/urls.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    const words = ["business", "operations", "community", "city", "life"];
    let tick = 0;
    const currentWord = words[tick % words.length];
    const products = [
      {
        name: "1nga Plus",
        tagline: "One identity, all your apps",
        description: "Single sign-on, encryption, and subscription-gated access across every 1nga solution."
      },
      {
        name: "Wave",
        tagline: "Payments & invoicing",
        description: "Send and receive money, issue invoices, and manage transactions with ease."
      },
      {
        name: "1nga Online",
        tagline: "Learning & courses",
        description: "Education and training platforms for individuals and organizations."
      },
      {
        name: "1nga B2B",
        tagline: "Business-to-business",
        description: "Wholesale, orders, and business profiles for B2B commerce."
      },
      {
        name: "1nga Farms",
        tagline: "Agriculture & supply chain",
        description: "Farm management, supply chain, and agribusiness solutions."
      },
      {
        name: "Mawatt",
        tagline: "Energy & utilities",
        description: "Energy management and utilities for modern operations."
      },
      {
        name: "skiLit",
        tagline: "Training & skills",
        description: "Learning, upskilling, and training (1nga Learn)."
      },
      {
        name: "1nga Scounts",
        tagline: "Engagement & rewards",
        description: "Loyalty, points, and engagement across 1nga apps."
      },
      {
        name: "1nga Security",
        tagline: "Trust & compliance",
        description: "Security, identity, and compliance for enterprises."
      }
    ];
    head("1uha8ag", $$renderer2, ($$renderer3) => {
      $$renderer3.title(($$renderer4) => {
        $$renderer4.push(`<title>1nga Solutions — Digital transformation Hub</title>`);
      });
    });
    $$renderer2.push(`<div class="min-h-screen flex flex-col"><header class="fixed top-0 left-0 right-0 z-50 border-b border-slate-200 bg-[#FFFCFA]/90 backdrop-blur-xl"><nav class="mx-auto flex max-w-6xl items-center justify-between px-6 py-4"><a href="/" class="inline-flex items-center gap-2 text-xl font-semibold tracking-tight text-slate-900 transition hover:text-amber-600"><img src="/oneauth-logo.svg" alt="" class="h-7 w-auto" aria-hidden="true"/> <span>1nga Solutions</span></a> <a${attr("href", plusUrl)} target="_blank" rel="noopener noreferrer" class="rounded-full border border-slate-300 bg-transparent px-5 py-2.5 text-sm font-medium text-slate-700 transition hover:border-amber-500/60 hover:text-amber-600">Sign in</a></nav></header> <main class="flex-1"><section class="gradient-mesh relative min-h-[90vh] flex flex-col items-center justify-center px-6 pt-28 pb-24 text-center"><p class="mb-4 text-sm font-medium uppercase tracking-[0.2em] text-slate-500">1nga Solutions</p> <h1 class="max-w-4xl text-4xl font-bold tracking-tight text-slate-900 sm:text-5xl md:text-6xl lg:text-7xl">The <span class="bg-gradient-to-r from-amber-600 via-amber-500 to-amber-600 bg-clip-text text-transparent">Digital transformation Hub</span></h1> <div class="mt-6 h-px w-32 rounded-full shine-line" role="presentation"></div> <p class="mt-8 max-w-2xl text-lg text-slate-600 sm:text-xl">We are a digital transformation hub where new thinking and advanced systems come together to move your <span class="inline-block min-w-[11ch] text-center align-baseline"><!---->`);
    {
      $$renderer2.push(`<span class="word-swap inline-block bg-gradient-to-r from-amber-600 to-amber-500 bg-clip-text font-semibold text-transparent">${escape_html(currentWord)}</span>`);
    }
    $$renderer2.push(`<!----></span> forward.</p> <div class="mt-12 flex flex-wrap items-center justify-center gap-4"><a href="#solutions" class="rounded-full bg-amber-500 px-6 py-3 text-sm font-semibold text-black transition hover:bg-amber-400 focus:outline-none focus:ring-2 focus:ring-amber-400 focus:ring-offset-2 focus:ring-offset-[#FFFCFA]">Our solutions</a> <a${attr("href", plusUrl)} target="_blank" rel="noopener noreferrer" class="rounded-full border border-slate-300 px-6 py-3 text-sm font-medium text-slate-700 transition hover:border-amber-500/60 hover:text-amber-600">Get started</a></div></section> <section id="solutions" class="border-t border-slate-200 bg-[#FFF8F3]/60 py-24 px-6"><div class="mx-auto max-w-6xl"><p class="text-center text-sm font-medium uppercase tracking-[0.2em] text-slate-500">What we offer</p> <h2 class="mt-4 text-center text-3xl font-bold tracking-tight text-slate-900 sm:text-4xl">Products &amp; services</h2> <p class="mx-auto mt-4 max-w-2xl text-center text-slate-600">From payments and learning to agriculture and energy—one ecosystem for your entire digital journey.</p> <ul class="mt-16 grid gap-6 sm:grid-cols-2 lg:grid-cols-3"><!--[-->`);
    const each_array = ensure_array_like(products);
    for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
      let product = each_array[$$index];
      $$renderer2.push(`<li class="group rounded-2xl border border-slate-200 bg-white/80 p-6 shadow-sm transition hover:border-amber-500/30 hover:bg-white hover:shadow-md"><h3 class="text-lg font-semibold text-slate-900 group-hover:text-amber-600">${escape_html(product.name)}</h3> <p class="mt-1 text-sm font-medium text-amber-600">${escape_html(product.tagline)}</p> <p class="mt-3 text-sm leading-relaxed text-slate-600">${escape_html(product.description)}</p></li>`);
    }
    $$renderer2.push(`<!--]--></ul></div></section> <section class="border-t border-slate-200 bg-[#FFF4ED]/40 py-20 px-6"><div class="mx-auto max-w-3xl text-center"><h2 class="text-2xl font-bold tracking-tight text-slate-900 sm:text-3xl">Ready to transform?</h2> <p class="mt-4 text-slate-600">Use our apps, or partner with us to build custom solutions for your business.</p> <div class="mt-8 flex flex-wrap items-center justify-center gap-4"><a${attr("href", plusUrl)} target="_blank" rel="noopener noreferrer" class="inline-block rounded-full bg-amber-500 px-8 py-3.5 text-sm font-semibold text-black transition hover:bg-amber-400 focus:outline-none focus:ring-2 focus:ring-amber-400 focus:ring-offset-2 focus:ring-offset-[#FFFCFA]">Sign in to 1nga Plus</a> <a${attr("href", businessUrl)} target="_blank" rel="noopener noreferrer" class="inline-block rounded-full border border-slate-300 px-8 py-3.5 text-sm font-semibold text-slate-700 transition hover:border-amber-500/60 hover:text-amber-600 focus:outline-none focus:ring-2 focus:ring-amber-400/50 focus:ring-offset-2 focus:ring-offset-[#FFFCFA]">Partner with us</a></div> <p class="mt-6 text-sm text-slate-500">Corporates &amp; businesses: bring your solutions, or let us help you innovate and transform your systems.</p></div></section></main> <footer class="border-t border-slate-200 bg-white/60 py-8 px-6"><div class="mx-auto flex max-w-6xl flex-col items-center justify-between gap-4 sm:flex-row"><div class="inline-flex items-center gap-2"><img src="/oneauth-logo.svg" alt="" class="h-5 w-auto" aria-hidden="true"/> <span class="text-sm text-slate-500">© ${escape_html((/* @__PURE__ */ new Date()).getFullYear())} 1nga Solutions. All rights reserved.</span></div> <div class="flex flex-wrap items-center justify-center gap-6 text-sm"><a href="/contact" class="text-slate-500 transition hover:text-slate-700">Contact</a> <a href="/careers" class="text-slate-500 transition hover:text-slate-700">Careers</a> <a href="/terms" class="text-slate-500 transition hover:text-slate-700">Terms</a> <a href="/privacy" class="text-slate-500 transition hover:text-slate-700">Privacy</a> <a href="/cookies" class="text-slate-500 transition hover:text-slate-700">Cookies</a></div></div></footer></div>`);
  });
}
export {
  _page as default
};
