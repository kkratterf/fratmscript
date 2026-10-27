// Page analytics with PostHog, without cookies: `persistence: 'memory'`
// keeps the anonymous id in memory only (no cookies, no localStorage), so
// the site needs no consent banner. It loads only on the production host,
// so local dev, previews and forks send nothing. The project key is public
// by design: it can only send events, not read them.
const POSTHOG_KEY = 'phc_rz5QWsFu47b3BsBKcnpwNd7CfcXVqqecs3dfLtZVQb3a'
const POSTHOG_HOST = 'https://eu.i.posthog.com'
const PRODUCTION_HOST = 'fratmscript.kkratter.com'

const CONFIG = {
  api_host: POSTHOG_HOST,
  persistence: 'memory',
  person_profiles: 'identified_only',
  // The playground changes the URL hash with history.replaceState
  capture_pageview: 'history_change',
  disable_session_recording: true,
}

/** PostHog's loader snippet, run only on the production host. */
export function analyticsScript(): string {
  return `<script>
if (location.hostname === ${JSON.stringify(PRODUCTION_HOST)}) {
!function(t,e){var o,n,p,r;e.__SV||(window.posthog=e,e._i=[],e.init=function(i,s,a){function g(t,e){var o=e.split(".");2==o.length&&(t=t[o[0]],e=o[1]),t[e]=function(){t.push([e].concat(Array.prototype.slice.call(arguments,0)))}}(p=t.createElement("script")).type="text/javascript",p.crossOrigin="anonymous",p.async=!0,p.src=s.api_host.replace(".i.posthog.com","-assets.i.posthog.com")+"/static/array.js",(r=t.getElementsByTagName("script")[0]).parentNode.insertBefore(p,r);var u=e;for(void 0!==a?u=e[a]=[]:a="posthog",u.people=u.people||[],u.toString=function(t){var e="posthog";return"posthog"!==a&&(e+="."+a),t||(e+=" (stub)"),e},u.people.toString=function(){return u.toString(1)+".people (stub)"},o="init capture register register_once unregister identify reset get_distinct_id set_config opt_in_capturing opt_out_capturing has_opted_out_capturing".split(" "),n=0;n<o.length;n++)g(u,o[n]);e._i.push([i,s,a])},e.__SV=1)}(document,window.posthog||[]);
posthog.init(${JSON.stringify(POSTHOG_KEY)}, ${JSON.stringify(CONFIG)});
}
</script>`
}
