import { i as attr_class, j as attr_style, d as stringify } from "./index.js";
function AnimateOnView($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let {
      animation = "fade-up",
      delay = 0,
      duration = 600,
      stagger = 0,
      rootMargin = "0px 0px -60px 0px",
      threshold = 0.1,
      class: className = "",
      children
    } = $$props;
    let visible = false;
    $$renderer2.push(`<div${attr_class(`animate-on-view ${stringify(animation)} ${stringify(className)}`, void 0, { "visible": visible })}${attr_style(`--anim-delay: ${stringify(delay + stagger * 100)}ms; --anim-duration: ${stringify(duration)}ms;`)}>`);
    children?.($$renderer2);
    $$renderer2.push(`<!----></div>`);
  });
}
export {
  AnimateOnView as A
};
