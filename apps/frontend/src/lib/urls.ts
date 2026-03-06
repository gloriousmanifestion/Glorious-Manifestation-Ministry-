/**
 * Environment-aware URLs for 1nga Plus and Business.
 * Set at build time by CI/CD: staging → beta-plus/beta-bus, production → plus.1nga.com/business.1nga.com
 */
import { env } from '$env/dynamic/public';

const PLUS_STAGING = 'https://beta-plus.1nga.online';
const BUSINESS_STAGING = 'https://beta-bus.1nga.online';

export const plusUrl = (env.PUBLIC_PLUS_URL || '').trim() || PLUS_STAGING;

export const businessUrl = (env.PUBLIC_BUSINESS_URL || '').trim() || BUSINESS_STAGING;
