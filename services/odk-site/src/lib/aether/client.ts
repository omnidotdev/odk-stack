import { BILLING_BASE_URL } from "@/lib/config/env.config";

/**
 * Entitlement from Aether.
 */
export interface Entitlement {
  id: string;
  productId: string;
  featureKey: string;
  value: string | null;
  source: string;
  validFrom: string;
  validUntil: string | null;
}

/**
 * Entitlements response from Aether.
 */
export interface EntitlementsResponse {
  billingAccountId: string;
  entityType: string;
  entityId: string;
  entitlementVersion: number;
  entitlements: Entitlement[];
}

/**
 * Subscription details from Aether.
 */
export interface Subscription {
  id: string;
  status: string;
  cancelAt: number | null;
  currentPeriodEnd: number;
  priceId: string;
  product: {
    id: string;
    name: string;
    description: string | null;
    marketing_features: Array<{ name: string }>;
  } | null;
}

/**
 * Get all entitlements for an entity.
 * Optionally filter by product.
 *
 * Returns null if BILLING_BASE_URL is not configured.
 */
export async function getEntitlements(
  entityType: string,
  entityId: string,
  productId?: string,
  accessToken?: string,
): Promise<EntitlementsResponse | null> {
  if (!BILLING_BASE_URL) {
    return null;
  }

  try {
    const url = new URL(
      `${BILLING_BASE_URL}/entitlements/${entityType}/${entityId}`,
    );
    if (productId) {
      url.searchParams.set("productId", productId);
    }

    const headers: HeadersInit = {};
    if (accessToken) {
      headers.Authorization = `Bearer ${accessToken}`;
    }

    const res = await fetch(url, { headers });

    if (!res.ok) {
      return null;
    }

    return res.json();
  } catch {
    return null;
  }
}

/**
 * Get subscription details for an entity via billing service.
 */
export async function getSubscription(
  entityType: string,
  entityId: string,
  accessToken: string,
): Promise<Subscription | null> {
  if (!BILLING_BASE_URL) {
    return null;
  }

  try {
    const response = await fetch(
      `${BILLING_BASE_URL}/billing-portal/subscription/${entityType}/${entityId}`,
      {
        headers: {
          Authorization: `Bearer ${accessToken}`,
        },
      },
    );

    if (!response.ok) {
      console.error("Failed to fetch subscription:", await response.text());
      return null;
    }

    const { subscription } = await response.json();
    return subscription;
  } catch {
    return null;
  }
}

/**
 * Get billing portal URL for an entity.
 */
export async function getBillingPortalUrl(
  entityType: string,
  entityId: string,
  productId: string,
  returnUrl: string,
  accessToken: string,
): Promise<string> {
  if (!BILLING_BASE_URL) {
    throw new Error("BILLING_BASE_URL not configured");
  }

  const response = await fetch(
    `${BILLING_BASE_URL}/billing-portal/${entityType}/${entityId}`,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${accessToken}`,
      },
      body: JSON.stringify({
        productId,
        returnUrl,
      }),
    },
  );

  if (!response.ok) {
    const error = await response.json().catch(() => ({}));
    throw new Error(error.error || "Failed to get billing portal URL");
  }

  const { url } = await response.json();
  return url;
}

/**
 * Cancel a subscription for an entity via billing service.
 */
export async function cancelSubscription(
  entityType: string,
  entityId: string,
  accessToken: string,
): Promise<string> {
  if (!BILLING_BASE_URL) {
    throw new Error("BILLING_BASE_URL not configured");
  }

  const response = await fetch(
    `${BILLING_BASE_URL}/billing-portal/subscription/${entityType}/${entityId}/cancel`,
    {
      method: "POST",
      headers: {
        Authorization: `Bearer ${accessToken}`,
      },
    },
  );

  if (!response.ok) {
    const error = await response.json().catch(() => ({}));
    throw new Error(error.error || "Failed to cancel subscription");
  }

  const { id } = await response.json();
  return id;
}

/**
 * Renew a subscription (remove scheduled cancellation) via billing service.
 */
export async function renewSubscription(
  entityType: string,
  entityId: string,
  accessToken: string,
): Promise<void> {
  if (!BILLING_BASE_URL) {
    throw new Error("BILLING_BASE_URL not configured");
  }

  const response = await fetch(
    `${BILLING_BASE_URL}/billing-portal/subscription/${entityType}/${entityId}/renew`,
    {
      method: "POST",
      headers: {
        Authorization: `Bearer ${accessToken}`,
      },
    },
  );

  if (!response.ok) {
    const error = await response.json().catch(() => ({}));
    throw new Error(error.error || "Failed to renew subscription");
  }
}
