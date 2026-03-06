import { w as writable } from "./exports.js";
const USER_KEY = "pms_user";
function loadUser() {
  if (typeof window === "undefined") return null;
  try {
    const raw = localStorage.getItem(USER_KEY);
    if (!raw) return null;
    return JSON.parse(raw);
  } catch {
    return null;
  }
}
const userStore = writable(loadUser());
export {
  userStore as u
};
