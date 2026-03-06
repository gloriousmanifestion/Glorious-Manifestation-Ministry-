import { i as attr_class, f as ensure_array_like, j as attr_style, d as stringify, b as attr, e as escape_html, h as head } from "../../../chunks/index.js";
import { N as Nav } from "../../../chunks/Nav.js";
import { F as Footer } from "../../../chunks/Footer.js";
import { A as AnimateOnView } from "../../../chunks/AnimateOnView.js";
import { S as SITE } from "../../../chunks/config.js";
function html(value) {
  var html2 = String(value ?? "");
  var open = "<!---->";
  return open + html2 + "<!---->";
}
function HeroCarousel($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { slides, class: className = "" } = $$props;
    let currentIndex = 0;
    $$renderer2.push(`<div${attr_class(`hero-carousel relative min-h-[50vh] sm:min-h-[55vh] overflow-hidden rounded-2xl ${stringify(className)}`)} role="region" aria-label="Hero carousel"><!--[-->`);
    const each_array = ensure_array_like(slides);
    for (let i = 0, $$length = each_array.length; i < $$length; i++) {
      let slide = each_array[i];
      $$renderer2.push(`<div${attr_class("absolute inset-0 flex flex-col items-center justify-center px-6 py-16 text-center transition-opacity duration-500", void 0, {
        "opacity-0": currentIndex !== i,
        "opacity-100": currentIndex === i,
        "pointer-events-none": currentIndex !== i
      })}${attr_style(`background: ${stringify(slide.gradient)};`)} role="group" aria-roledescription="slide"${attr("aria-label", `Slide ${stringify(i + 1)} of ${stringify(slides.length)}`)}>`);
      if (slide.icon) {
        $$renderer2.push("<!--[-->");
        $$renderer2.push(`<div class="mb-4 flex h-16 w-16 items-center justify-center rounded-2xl bg-white/20 text-4xl backdrop-blur-sm">${escape_html(slide.icon)}</div>`);
      } else {
        $$renderer2.push("<!--[!-->");
      }
      $$renderer2.push(`<!--]--> <h2 class="text-2xl font-bold tracking-tight text-white drop-shadow-sm sm:text-3xl md:text-4xl">${escape_html(slide.title)}</h2> <p class="mt-3 max-w-xl text-base text-white/90 sm:text-lg">${escape_html(slide.subtitle)}</p></div>`);
    }
    $$renderer2.push(`<!--]--> <div class="absolute bottom-4 left-0 right-0 flex justify-center gap-2" role="tablist" aria-label="Carousel navigation"><!--[-->`);
    const each_array_1 = ensure_array_like(slides);
    for (let i = 0, $$length = each_array_1.length; i < $$length; i++) {
      each_array_1[i];
      $$renderer2.push(`<button type="button"${attr_class(`h-2.5 w-2.5 rounded-full transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-transparent ${stringify(currentIndex === i ? "bg-white scale-125" : "bg-white/50")}`)}${attr("aria-label", `Go to slide ${stringify(i + 1)}`)}${attr("aria-selected", currentIndex === i)} role="tab"></button>`);
    }
    $$renderer2.push(`<!--]--></div> <button type="button" class="absolute left-2 top-1/2 -translate-y-1/2 rounded-full bg-white/20 p-2 text-white backdrop-blur-sm transition hover:bg-white/30 focus:outline-none focus:ring-2 focus:ring-white" aria-label="Previous slide"><svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path></svg></button> <button type="button" class="absolute right-2 top-1/2 -translate-y-1/2 rounded-full bg-white/20 p-2 text-white backdrop-blur-sm transition hover:bg-white/30 focus:outline-none focus:ring-2 focus:ring-white" aria-label="Next slide"><svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path></svg></button></div>`);
  });
}
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    const heroSlides = [
      {
        title: "Equip for Ministry",
        subtitle: "Comprehensive training for pastors, ministers, and prophets. Learn to lead, shepherd, and serve with biblical wisdom.",
        gradient: "linear-gradient(135deg, #ec4899 0%, #a855f7 50%, #6366f1 100%)",
        icon: "📖"
      },
      {
        title: "Biblical Foundation",
        subtitle: "All teaching rooted in Scripture and sound doctrine. Grow in your understanding of God's Word.",
        gradient: "linear-gradient(135deg, #8b5cf6 0%, #6366f1 50%, #3b82f6 100%)",
        icon: "📜"
      },
      {
        title: "Practical Ministry",
        subtitle: "Combine theological education with hands-on ministry skills. Ready for real-world ministry challenges.",
        gradient: "linear-gradient(135deg, #db2777 0%, #ec4899 50%, #8b5cf6 100%)",
        icon: "⛪"
      },
      {
        title: `${SITE.duration}, ${SITE.schedule}`,
        subtitle: `A structured program in ${SITE.location}. Join a community of ministers and prophets.`,
        gradient: "linear-gradient(135deg, #6366f1 0%, #8b5cf6 50%, #a855f7 100%)",
        icon: "📅"
      },
      {
        title: "Six Core Courses",
        subtitle: "Pastoral Leadership, Prophetic Ministry, Biblical Studies, Worship, Evangelism, and Ministerial Training.",
        gradient: "linear-gradient(135deg, #a855f7 0%, #ec4899 50%, #f43f5e 100%)",
        icon: "✨"
      }
    ];
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
    const highlights = [
      { label: "Duration", value: SITE.duration },
      { label: "Schedule", value: SITE.schedule },
      { label: "Location", value: SITE.location }
    ];
    const faqs = [
      {
        q: "Who is this program for?",
        a: "Our Pastoral School is designed for pastors, ministers, and prophets who desire to grow in their calling. Whether you are already in ministry or sensing a call, this program will equip you with biblical foundations and practical skills."
      },
      {
        q: "What are the admission requirements?",
        a: 'We look for a genuine call to ministry, commitment to attend classes, and basic literacy. A recommendation from a church or ministry leader is optional but preferred. See our <a href="/admissions" class="text-pink-600 font-medium hover:underline">Admissions page</a> for the full process.'
      },
      {
        q: "How do I register?",
        a: "Create an account on our site, verify your email, then select your course and complete payment. You can also contact us via WhatsApp for assistance."
      },
      {
        q: "Where are classes held?",
        a: "Classes are held at our facility in " + SITE.location + ". Our address is " + SITE.address + "."
      }
    ];
    const programs = [
      {
        title: "Pastoral Leadership",
        description: "Develop biblical leadership skills for effective church ministry. Learn to shepherd, counsel, and lead with wisdom and integrity.",
        topics: [
          "Church administration",
          "Pastoral care",
          "Leadership principles",
          "Conflict resolution"
        ]
      },
      {
        title: "Prophetic Ministry",
        description: "Understanding and operating in the prophetic gift. Learn to hear from God, deliver prophetic words with accuracy, and minister in the prophetic.",
        topics: [
          "Hearing God",
          "Prophecy and discernment",
          "Dreams and visions",
          "Prophetic protocol"
        ]
      },
      {
        title: "Ministerial Training",
        description: "Comprehensive training for ministers in preaching, teaching, and pastoral care. Build confidence in public ministry.",
        topics: [
          "Homiletics",
          "Teaching methods",
          "Pastoral counseling",
          "Ministry ethics"
        ]
      },
      {
        title: "Biblical Studies",
        description: "In-depth study of Scripture for sound doctrine and spiritual growth. Hermeneutics, theology, and biblical interpretation.",
        topics: [
          "Hermeneutics",
          "Systematic theology",
          "Old & New Testament survey",
          "Church history"
        ]
      },
      {
        title: "Worship & Praise",
        description: "Leading worship and understanding the ministry of praise in the church. Music, flow, and the role of worship in spiritual warfare.",
        topics: [
          "Worship leading",
          "Praise and worship",
          "Music ministry",
          "Flow in the Spirit"
        ]
      },
      {
        title: "Evangelism & Outreach",
        description: "Equipping for effective soul-winning and community outreach. Share the Gospel with confidence and compassion.",
        topics: [
          "Personal evangelism",
          "Street ministry",
          "Community outreach",
          "Discipleship"
        ]
      }
    ];
    head("19bqm8i", $$renderer2, ($$renderer3) => {
      $$renderer3.title(($$renderer4) => {
        $$renderer4.push(`<title>Programs &amp; Courses — ${escape_html(SITE.name)} Pastoral School</title>`);
      });
      $$renderer3.push(`<meta name="description"${attr("content", `Learn about our Pastoral School: all courses, mission, values, and helpful information. Pastoral Leadership, Prophetic Ministry, Ministerial Training, Biblical Studies, Worship, and Evangelism. ${stringify(SITE.location)}.`)}/>`);
    });
    $$renderer2.push(`<div class="min-h-screen flex flex-col">`);
    Nav($$renderer2);
    $$renderer2.push(`<!----> <main class="flex-1"><section class="px-4 pt-24 sm:pt-28 pb-8 sm:px-6"><div class="mx-auto max-w-5xl"><div class="mb-6 text-center"><p class="text-sm font-medium uppercase tracking-[0.2em] text-pink-600">${escape_html(SITE.name)}</p> <h1 class="mt-2 text-2xl font-bold tracking-tight text-slate-900 sm:text-3xl">${escape_html(SITE.schoolName)}</h1></div> `);
    HeroCarousel($$renderer2, { slides: heroSlides });
    $$renderer2.push(`<!----></div></section> <section class="border-t border-pink-200/50 bg-white/60 py-12 sm:py-16 px-4 sm:px-6"><div class="mx-auto max-w-4xl">`);
    AnimateOnView($$renderer2, {
      animation: "fade-up",
      children: ($$renderer3) => {
        $$renderer3.push(`<h2 class="text-2xl font-bold tracking-tight text-slate-900">Our mission</h2> <p class="mt-6 text-slate-600 leading-relaxed">Under the umbrella of Prophetic Glorious Manifestation, our Pastoral School exists to equip and empower pastors, ministers, and prophets for effective ministry. We are committed to raising a generation of spiritual leaders who are grounded in the Word, led by the Spirit, and equipped to serve their communities.</p> <p class="mt-4 text-slate-600 leading-relaxed">Located in ${escape_html(SITE.location)}, we offer a structured program running ${escape_html(SITE.duration)}, ${escape_html(SITE.schedule)}. Our curriculum combines theological education with practical ministry skills for those called to pastoral ministry, prophetic ministry, or general ministerial service.</p>`);
      }
    });
    $$renderer2.push(`<!----></div></section> <section class="border-t border-pink-200/50 bg-gradient-to-b from-pink-50/50 to-purple-50/30 py-12 sm:py-16 px-4 sm:px-6"><div class="mx-auto max-w-4xl">`);
    AnimateOnView($$renderer2, {
      animation: "fade-up",
      children: ($$renderer3) => {
        $$renderer3.push(`<h2 class="text-center text-xl font-bold tracking-tight text-slate-900 sm:text-2xl">Program details</h2>`);
      }
    });
    $$renderer2.push(`<!----> <div class="mt-8 grid gap-4 sm:gap-6 grid-cols-1 sm:grid-cols-3"><!--[-->`);
    const each_array = ensure_array_like(highlights);
    for (let i = 0, $$length = each_array.length; i < $$length; i++) {
      let item = each_array[i];
      AnimateOnView($$renderer2, {
        animation: "fade-up",
        stagger: i,
        duration: 500,
        children: ($$renderer3) => {
          $$renderer3.push(`<div class="hover-lift rounded-2xl border border-pink-200/60 bg-white p-6 text-center shadow-sm hover:border-pink-300 hover:shadow-lg"><p class="text-sm font-medium uppercase tracking-wider text-pink-600">${escape_html(item.label)}</p> <p class="mt-2 text-lg font-semibold text-slate-900">${escape_html(item.value)}</p></div>`);
        }
      });
    }
    $$renderer2.push(`<!--]--></div></div></section> <section class="border-t border-pink-200/50 bg-white/60 py-12 sm:py-16 px-4 sm:px-6"><div class="mx-auto max-w-6xl">`);
    AnimateOnView($$renderer2, {
      animation: "fade-up",
      children: ($$renderer3) => {
        $$renderer3.push(`<h2 class="text-center text-xl font-bold tracking-tight text-slate-900 sm:text-2xl md:text-3xl">Our values</h2>`);
      }
    });
    $$renderer2.push(`<!----> <div class="mt-8 sm:mt-12 grid gap-4 sm:gap-6 grid-cols-1 sm:grid-cols-2 lg:grid-cols-4"><!--[-->`);
    const each_array_1 = ensure_array_like(values);
    for (let i = 0, $$length = each_array_1.length; i < $$length; i++) {
      let value = each_array_1[i];
      AnimateOnView($$renderer2, {
        animation: "fade-up",
        stagger: i,
        children: ($$renderer3) => {
          $$renderer3.push(`<div class="hover-lift rounded-2xl border border-pink-200/50 bg-white p-6 shadow-sm"><h3 class="text-lg font-semibold text-slate-900">${escape_html(value.title)}</h3> <p class="mt-2 text-sm text-slate-600">${escape_html(value.description)}</p></div>`);
        }
      });
    }
    $$renderer2.push(`<!--]--></div></div></section> <section id="courses" class="border-t border-pink-200/50 bg-gradient-to-b from-pink-50/50 to-purple-50/30 py-12 sm:py-16 px-4 sm:px-6"><div class="mx-auto max-w-4xl">`);
    AnimateOnView($$renderer2, {
      animation: "fade-up",
      children: ($$renderer3) => {
        $$renderer3.push(`<h2 class="text-center text-xl font-bold tracking-tight text-slate-900 sm:text-2xl mb-8 sm:mb-12">All courses</h2>`);
      }
    });
    $$renderer2.push(`<!----> <div class="space-y-8 sm:space-y-12"><!--[-->`);
    const each_array_2 = ensure_array_like(programs);
    for (let i = 0, $$length = each_array_2.length; i < $$length; i++) {
      let program = each_array_2[i];
      AnimateOnView($$renderer2, {
        animation: "slide-left",
        stagger: i,
        duration: 500,
        children: ($$renderer3) => {
          $$renderer3.push(`<article class="hover-lift rounded-2xl border border-pink-200/50 bg-white p-4 sm:p-8 shadow-sm hover:border-pink-300 hover:shadow-xl"><div class="flex flex-col sm:flex-row items-start gap-4"><span class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-gradient-to-br from-pink-100 to-purple-100 text-xl font-bold text-pink-600">${escape_html(i + 1)}</span> <div><h2 class="text-xl font-bold text-slate-900">${escape_html(program.title)}</h2> <p class="mt-3 text-slate-600">${escape_html(program.description)}</p> <ul class="mt-4 flex flex-wrap gap-2"><!--[-->`);
          const each_array_3 = ensure_array_like(program.topics);
          for (let $$index_2 = 0, $$length2 = each_array_3.length; $$index_2 < $$length2; $$index_2++) {
            let topic = each_array_3[$$index_2];
            $$renderer3.push(`<li class="rounded-full bg-pink-50 px-3 py-1 text-sm text-pink-700">${escape_html(topic)}</li>`);
          }
          $$renderer3.push(`<!--]--></ul></div></div></article>`);
        }
      });
    }
    $$renderer2.push(`<!--]--></div></div></section> <section class="border-t border-pink-200/50 bg-white/60 py-12 sm:py-16 px-4 sm:px-6"><div class="mx-auto max-w-3xl">`);
    AnimateOnView($$renderer2, {
      animation: "fade-up",
      children: ($$renderer3) => {
        $$renderer3.push(`<h2 class="text-2xl font-bold tracking-tight text-slate-900">Frequently asked questions</h2>`);
      }
    });
    $$renderer2.push(`<!----> <div class="mt-8 space-y-6"><!--[-->`);
    const each_array_4 = ensure_array_like(faqs);
    for (let i = 0, $$length = each_array_4.length; i < $$length; i++) {
      let faq = each_array_4[i];
      AnimateOnView($$renderer2, {
        animation: "fade-up",
        stagger: i,
        children: ($$renderer3) => {
          $$renderer3.push(`<div class="rounded-2xl border border-pink-200/50 bg-white p-6 shadow-sm"><h3 class="font-semibold text-slate-900">${escape_html(faq.q)}</h3> <p class="mt-2 text-sm text-slate-600 leading-relaxed">${html(faq.a)}</p></div>`);
        }
      });
    }
    $$renderer2.push(`<!--]--></div></div></section> <section class="border-t border-pink-200/50 bg-gradient-to-r from-pink-500/10 to-purple-500/10 py-12 sm:py-16 px-4 sm:px-6">`);
    AnimateOnView($$renderer2, {
      animation: "scale",
      children: ($$renderer3) => {
        $$renderer3.push(`<div class="mx-auto max-w-2xl text-center"><h2 class="text-2xl font-bold tracking-tight text-slate-900">Ready to answer the call?</h2> <p class="mt-4 text-slate-600">Create your account to register, or contact us for more details.</p> <div class="mt-6 flex justify-center"><a href="/register" class="btn-scale inline-flex items-center justify-center rounded-full bg-gradient-to-r from-pink-500 to-purple-600 px-8 py-3.5 text-sm font-semibold text-white shadow-lg transition hover:shadow-xl">Register now</a></div></div>`);
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
