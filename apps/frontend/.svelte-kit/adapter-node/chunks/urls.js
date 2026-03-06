import { p as public_env } from "./shared-server.js";
const PLUS_STAGING = "https://beta-plus.1nga.online";
const BUSINESS_STAGING = "https://beta-bus.1nga.online";
const plusUrl = (public_env.PUBLIC_PLUS_URL || "").trim() || PLUS_STAGING;
const businessUrl = (public_env.PUBLIC_BUSINESS_URL || "").trim() || BUSINESS_STAGING;
export {
  businessUrl as b,
  plusUrl as p
};
