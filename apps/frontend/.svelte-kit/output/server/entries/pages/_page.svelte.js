import { b as attr, d as stringify, e as escape_html, h as head, f as ensure_array_like } from "../../chunks/index.js";
import { N as Nav } from "../../chunks/Nav.js";
import { F as Footer } from "../../chunks/Footer.js";
import { A as AnimateOnView } from "../../chunks/AnimateOnView.js";
import { S as SITE } from "../../chunks/config.js";
function LeaderSection($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    const leader = SITE.leader;
    $$renderer2.push(`<section class="border-t border-slate-200/50 bg-white py-20 sm:py-28 px-4 sm:px-6"><div class="mx-auto max-w-5xl">`);
    AnimateOnView($$renderer2, {
      animation: "fade-up",
      children: ($$renderer3) => {
        $$renderer3.push(`<div class="flex flex-col lg:flex-row lg:items-center lg:gap-20"><div class="lg:w-[42%] shrink-0 order-2 lg:order-1"><div class="relative max-w-sm mx-auto lg:mx-0"><div class="absolute -inset-4 rounded-[1.25rem] border border-amber-200/50 bg-gradient-to-br from-amber-50/60 to-amber-100/30" aria-hidden="true"></div> <div class="relative overflow-hidden rounded-xl shadow-2xl shadow-slate-300/40 ring-1 ring-slate-200/60"><img src="/prophetess-leader.png"${attr("alt", `${stringify(leader.name)} — ${stringify(leader.title)} of the Pastoral School`)} class="w-full object-cover object-top aspect-[3/4]"/></div></div></div> <div class="order-1 lg:order-2 lg:flex-1"><p class="text-[11px] font-semibold uppercase tracking-[0.3em] text-amber-800/70">Our Leader</p> <h2 class="mt-5 text-2xl font-semibold tracking-tight text-slate-900 sm:text-3xl">${escape_html(leader.name)}</h2> <p class="mt-2 text-base font-medium text-slate-600">${escape_html(leader.title)}</p> <div class="mt-8 w-12 h-px bg-gradient-to-r from-amber-400/80 to-amber-200/40" aria-hidden="true"></div> <p class="mt-8 text-slate-600 leading-[1.7] text-[15px]">${escape_html(leader.bio)}</p></div></div>`);
      }
    });
    $$renderer2.push(`<!----></div></section>`);
  });
}
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    const values = [
      {
        title: "Biblical Foundation",
        description: "All teaching rooted in Scripture and sound doctrine."
      },
      {
        title: "Spiritual Growth",
        description: "Nurturing personal relationship with God and spiritual maturity."
      },
      {
        title: "Practical Ministry",
        description: "Hands-on training for real-world ministry challenges."
      },
      {
        title: "Community",
        description: "Building lasting fellowship among ministers and prophets."
      }
    ];
    const programs = [
      {
        title: "Pastoral Leadership",
        description: "Develop biblical leadership skills for effective church ministry and shepherding.",
        icon: "📖"
      },
      {
        title: "Prophetic Ministry",
        description: "Understanding and operating in the prophetic gift with wisdom and discernment.",
        icon: "✨"
      },
      {
        title: "Ministerial Training",
        description: "Comprehensive training for ministers in preaching, teaching, and pastoral care.",
        icon: "⛪"
      },
      {
        title: "Biblical Studies",
        description: "In-depth study of Scripture for sound doctrine and spiritual growth.",
        icon: "📜"
      },
      {
        title: "Worship & Praise",
        description: "Leading worship and understanding the ministry of praise in the church.",
        icon: "🎵"
      },
      {
        title: "Evangelism & Outreach",
        description: "Equipping for effective soul-winning and community outreach.",
        icon: "🌍"
      }
    ];
    const highlights = [
      { label: "Duration", value: SITE.duration },
      { label: "Schedule", value: SITE.schedule },
      { label: "Location", value: SITE.location }
    ];
    head("1uha8ag", $$renderer2, ($$renderer3) => {
      $$renderer3.title(($$renderer4) => {
        $$renderer4.push(`<title>Pastoral School — ${escape_html(SITE.name)}</title>`);
      });
      $$renderer3.push(`<meta name="description" content="Pastoral School under Prophetic Glorious Manifestation. Training pastors, ministers and prophets in Owerri, Imo State. Our mission, values, programs and admissions. February to July program."/>`);
    });
    $$renderer2.push(`<div class="min-h-screen flex flex-col">`);
    Nav($$renderer2);
    $$renderer2.push(`<!----> <main class="flex-1"><section class="gradient-mesh relative min-h-[85vh] sm:min-h-[90vh] flex flex-col items-center justify-center px-4 pt-24 pb-16 sm:px-6 sm:pt-28 sm:pb-24 text-center overflow-hidden"><div class="animate-gradient-bg absolute inset-0" aria-hidden="true" style="background: radial-gradient(circle at 30% 20%, rgba(236,72,153,0.3) 0%, transparent 50%), radial-gradient(circle at 70% 80%, rgba(168,85,247,0.3) 0%, transparent 50%);"></div> <p class="hero-enter-delay-1 relative mb-4 text-sm font-medium uppercase tracking-[0.2em] text-pink-600/90">${escape_html(SITE.name)}</p> <h1 class="hero-enter-delay-2 relative max-w-4xl text-3xl font-bold tracking-tight text-slate-900 sm:text-4xl md:text-5xl lg:text-6xl xl:text-7xl"><span class="bg-gradient-to-r from-pink-600 via-purple-600 to-pink-600 bg-clip-text text-transparent">${escape_html(SITE.schoolName)}</span></h1> <div class="hero-enter-delay-3 relative mt-6 flex items-center gap-3"><span class="animate-glow-pulse inline-block rounded-full bg-pink-500 px-5 py-2 text-sm font-bold uppercase tracking-wider text-white shadow-lg glow-pink">Now Open</span></div> <div class="hero-enter-delay-3 relative mt-6 h-px w-32 rounded-full shine-line" role="presentation"></div> <p class="hero-enter-delay-4 relative mt-6 sm:mt-8 max-w-2xl text-base text-slate-600 sm:text-lg md:text-xl px-1">For pastors, ministers and prophets. Equipping the next generation of spiritual leaders with sound doctrine and practical ministry skills.</p> <div class="hero-enter-delay-5 relative mt-8 sm:mt-12 flex flex-col sm:flex-row flex-wrap items-stretch sm:items-center justify-center gap-3 sm:gap-4"><a href="/register" class="btn-scale rounded-full bg-gradient-to-r from-pink-500 to-purple-600 px-6 py-3.5 sm:px-8 text-sm font-semibold text-white shadow-lg glow-pink text-center touch-manipulation active:scale-[0.98]">Register now</a> <a href="#programs" class="btn-scale inline-flex items-center justify-center gap-2 rounded-full border-2 border-pink-300 bg-white px-6 py-3.5 sm:px-8 text-sm font-semibold text-pink-600 transition hover:bg-pink-50 hover:border-pink-400 touch-manipulation active:scale-[0.98]">Learn more <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"></path></svg></a></div></section> <section id="about" class="border-t border-pink-200/50 bg-white/60 py-12 sm:py-16 px-4 sm:px-6"><div class="mx-auto max-w-4xl">`);
    AnimateOnView($$renderer2, {
      animation: "fade-up",
      children: ($$renderer3) => {
        $$renderer3.push(`<div class="mb-12 overflow-hidden rounded-2xl border border-pink-200/50 shadow-lg transition-shadow duration-500 hover:shadow-xl"><img src="/pastoral-school-flyer.png" alt="Pastoral School promotional flyer - Now Open for pastors, ministers and prophets" class="w-full object-cover transition-transform duration-700 hover:scale-[1.02]"/></div>`);
      }
    });
    $$renderer2.push(`<!----> `);
    AnimateOnView($$renderer2, {
      animation: "fade-up",
      delay: 100,
      children: ($$renderer3) => {
        $$renderer3.push(`<h2 class="text-2xl font-bold tracking-tight text-slate-900">Our mission</h2> <p class="mt-6 text-slate-600 leading-relaxed">Under the umbrella of Prophetic Glorious Manifestation, our Pastoral School exists to equip and empower pastors, ministers, and prophets for effective ministry. We are committed to raising a generation of spiritual leaders who are grounded in the Word, led by the Spirit, and equipped to serve their communities.</p> <p class="mt-4 text-slate-600 leading-relaxed">Located in ${escape_html(SITE.location)}, we offer a structured program running ${escape_html(SITE.duration)}, ${escape_html(SITE.schedule)}. Our curriculum combines theological education with practical ministry skills for those called to pastoral ministry, prophetic ministry, or general ministerial service.</p>`);
      }
    });
    $$renderer2.push(`<!----></div></section> `);
    LeaderSection($$renderer2);
    $$renderer2.push(`<!----> <section class="border-t border-pink-200/50 bg-gradient-to-b from-pink-50/50 to-purple-50/30 py-12 sm:py-16 px-4 sm:px-6"><div class="mx-auto max-w-6xl">`);
    AnimateOnView($$renderer2, {
      animation: "fade-up",
      children: ($$renderer3) => {
        $$renderer3.push(`<h2 class="text-center text-xl font-bold tracking-tight text-slate-900 sm:text-2xl md:text-3xl">Our values</h2>`);
      }
    });
    $$renderer2.push(`<!----> <div class="mt-8 sm:mt-12 grid gap-4 sm:gap-6 grid-cols-1 sm:grid-cols-2 lg:grid-cols-4"><!--[-->`);
    const each_array = ensure_array_like(values);
    for (let i = 0, $$length = each_array.length; i < $$length; i++) {
      let value = each_array[i];
      AnimateOnView($$renderer2, {
        animation: "fade-up",
        stagger: i,
        children: ($$renderer3) => {
          $$renderer3.push(`<div class="hover-lift rounded-2xl border border-pink-200/50 bg-white p-6 shadow-sm"><h3 class="text-lg font-semibold text-slate-900">${escape_html(value.title)}</h3> <p class="mt-2 text-sm text-slate-600">${escape_html(value.description)}</p></div>`);
        }
      });
    }
    $$renderer2.push(`<!--]--></div></div></section> <section class="border-t border-pink-200/50 bg-white/60 py-12 sm:py-16 px-4 sm:px-6"><div class="mx-auto max-w-4xl"><div class="grid gap-4 sm:gap-6 grid-cols-1 sm:grid-cols-3"><!--[-->`);
    const each_array_1 = ensure_array_like(highlights);
    for (let i = 0, $$length = each_array_1.length; i < $$length; i++) {
      let item = each_array_1[i];
      AnimateOnView($$renderer2, {
        animation: "fade-up",
        stagger: i,
        duration: 500,
        children: ($$renderer3) => {
          $$renderer3.push(`<div class="hover-lift rounded-2xl border border-pink-200/60 bg-white p-6 text-center shadow-sm hover:border-pink-300 hover:shadow-lg"><p class="text-sm font-medium uppercase tracking-wider text-pink-600">${escape_html(item.label)}</p> <p class="mt-2 text-lg font-semibold text-slate-900">${escape_html(item.value)}</p></div>`);
        }
      });
    }
    $$renderer2.push(`<!--]--></div></div></section> <section id="programs" class="border-t border-pink-200/50 bg-gradient-to-b from-pink-50/50 to-purple-50/30 py-16 sm:py-24 px-4 sm:px-6"><div class="mx-auto max-w-6xl">`);
    AnimateOnView($$renderer2, {
      animation: "fade-up",
      children: ($$renderer3) => {
        $$renderer3.push(`<p class="text-center text-sm font-medium uppercase tracking-[0.2em] text-pink-600">Our curriculum</p> <h2 class="mt-4 text-center text-2xl font-bold tracking-tight text-slate-900 sm:text-3xl md:text-4xl">Programs &amp; courses</h2> <p class="mx-auto mt-4 max-w-2xl text-center text-slate-600">Comprehensive training designed to equip pastors, ministers, and prophets for effective ministry.</p>`);
      }
    });
    $$renderer2.push(`<!----> <ul class="mt-10 sm:mt-16 grid gap-4 sm:gap-6 grid-cols-1 sm:grid-cols-2 lg:grid-cols-3"><!--[-->`);
    const each_array_2 = ensure_array_like(programs);
    for (let i = 0, $$length = each_array_2.length; i < $$length; i++) {
      let program = each_array_2[i];
      AnimateOnView($$renderer2, {
        animation: "fade-up",
        stagger: i,
        duration: 500,
        children: ($$renderer3) => {
          $$renderer3.push(`<li class="hover-lift group rounded-2xl border border-pink-200/50 bg-white p-6 shadow-sm transition hover:border-pink-300 hover:shadow-xl hover:shadow-pink-100/50"><div class="mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-gradient-to-br from-pink-100 to-purple-100 text-2xl transition-transform duration-300 group-hover:scale-110">${escape_html(program.icon)}</div> <h3 class="text-lg font-semibold text-slate-900 transition-colors group-hover:text-pink-600">${escape_html(program.title)}</h3> <p class="mt-3 text-sm leading-relaxed text-slate-600">${escape_html(program.description)}</p></li>`);
        }
      });
    }
    $$renderer2.push(`<!--]--></ul></div></section> <section class="border-t border-pink-200/50 bg-gradient-to-r from-pink-500/10 via-purple-500/10 to-pink-500/10 py-16 sm:py-20 px-4 sm:px-6">`);
    AnimateOnView($$renderer2, {
      animation: "scale",
      duration: 600,
      children: ($$renderer3) => {
        $$renderer3.push(`<div class="mx-auto max-w-3xl text-center"><h2 class="text-xl font-bold tracking-tight text-slate-900 sm:text-2xl md:text-3xl">Ready to answer the call?</h2> <p class="mt-4 text-slate-600">Join our Pastoral School and be equipped for effective ministry.</p> <div class="mt-6 sm:mt-8 flex justify-center"><a href="/register" class="btn-scale inline-flex items-center justify-center rounded-full bg-gradient-to-r from-pink-500 to-purple-600 px-8 py-3.5 sm:px-10 text-sm font-semibold text-white shadow-lg transition hover:shadow-xl touch-manipulation active:scale-[0.98]">Register now</a></div></div>`);
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
