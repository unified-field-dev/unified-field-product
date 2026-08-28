import { test as base, expect, type Page } from "@playwright/test";

export type SeedAuthKind =
  | "anonymous"
  | "authenticated_verified"
  | "authenticated_unverified";

export type SeedPageView = {
  app_id: string;
  viewer_key?: string;
  age_secs?: number;
};

export async function seedAuth(
  page: Page,
  auth: SeedAuthKind,
  opts?: {
    page_views?: SeedPageView[];
    /** Distinct Spectra viewer so scenarios do not share usage rows. */
    usage_viewer?: string;
    /** E2e welcome-admin session flag (no Gauge Chronon on this host). */
    welcome_admin?: boolean;
    /**
     * When true, clear Help tour localStorage so spotlight scenarios can assert
     * first-visit behavior. Default false: mark seeded steps seen so other
     * product UI tests are not blocked by the tour overlay.
     */
    help_tour?: boolean;
  },
) {
  const helpTour = opts?.help_tour ?? false;
  await page.addInitScript((enableTour: boolean) => {
    try {
      if (enableTour) {
        // Clear once per tab session so completing a tour survives reload.
        if (!sessionStorage.getItem("uf.help.e2e_tour_cleared")) {
          localStorage.removeItem("uf.help.tour_steps");
          sessionStorage.setItem("uf.help.e2e_tour_cleared", "1");
        }
        return;
      }
      localStorage.setItem(
        "uf.help.tour_steps",
        JSON.stringify([
          {
            route: "/apps",
            feature_highlight: "apps-search",
            spotlight: "apps-search-input",
            replay: false,
          },
          {
            route: "/apps",
            feature_highlight: "apps-application-cards",
            spotlight: "apps-first-application-card",
            replay: false,
          },
          {
            route: "/apps/:app_name",
            feature_highlight: "app-overview-more-information",
            spotlight: "app-overview-more-information",
            replay: false,
          },
          {
            route: "/apps/:app_name",
            feature_highlight: "app-overview-source-code",
            spotlight: "app-overview-source-code",
            replay: false,
          },
          {
            route: "/apps/:app_name",
            feature_highlight: "app-overview-documentation",
            spotlight: "app-overview-documentation",
            replay: false,
          },
          {
            route: "/apps/:app_name",
            feature_highlight: "app-overview-product-link",
            spotlight: "app-overview-product-link",
            replay: false,
          },
          {
            route: "/welcome",
            feature_highlight: "welcome-featured",
            spotlight: "welcome-featured-card",
            replay: false,
          },
          {
            route: "/welcome",
            feature_highlight: "welcome-featured-view-all",
            spotlight: "welcome-featured-view-all",
            replay: false,
          },
          {
            route: "/welcome",
            feature_highlight: "welcome-recent",
            spotlight: "welcome-recent-apps-card",
            replay: false,
          },
          {
            route: "/welcome",
            feature_highlight: "welcome-most-used",
            spotlight: "welcome-most-used-card",
            replay: false,
          },
          {
            route: "/welcome",
            feature_highlight: "welcome-popular",
            spotlight: "welcome-popular-apps-card",
            replay: false,
          },
          {
            route: "/coming-soon",
            feature_highlight: "coming-soon-intro",
            spotlight: null,
            replay: false,
          },
        ]),
      );
    } catch {
      /* ignore */
    }
  }, helpTour);

  const res = await page.request.post("/api/test/seed-data", {
    data: {
      auth,
      usage_viewer: opts?.usage_viewer,
      welcome_admin: opts?.welcome_admin ?? false,
      page_views: opts?.page_views ?? [],
    },
  });
  expect(res.ok()).toBeTruthy();
  return res.json() as Promise<{
    ok: boolean;
    auth: string;
    usage_viewer?: string;
    welcome_admin?: boolean;
    page_views: number;
    recent_preview?: string[];
  }>;
}

/** Wait until Orbital boot overlay dismisses (WASM hydrate + `hide_boot_loader`). */
export async function waitForHydrated(page: Page) {
  await expect(page.locator("html")).toHaveAttribute("data-orbital-hydrated", "true", {
    timeout: 240_000,
  });
  await expect(page.getByTestId("orbital-boot-overlay")).toHaveCount(0, {
    timeout: 60_000,
  });
}

export const test = base;
export { expect };
