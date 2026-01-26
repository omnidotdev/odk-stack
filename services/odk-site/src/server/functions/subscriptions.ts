import { createServerFn } from "@tanstack/react-start";
import { z } from "zod";

import app from "@/lib/config/app.config";
import { BASE_URL } from "@/lib/config/env.config";
import billing from "@/lib/providers/billing";
import { authMiddleware } from "@/server/middleware";

const subscriptionSchema = z.object({
  entityType: z.string(),
  entityId: z.string().uuid(),
});

const checkoutSchema = z.object({
  entityType: z.string(),
  entityId: z.string().uuid(),
  priceId: z.string().startsWith("price_"),
  successUrl: z.string().url().optional(),
});

const billingPortalSchema = z.object({
  entityType: z.string(),
  entityId: z.string().uuid(),
  returnUrl: z.string().url().optional(),
});

/**
 * Get subscription details for an entity.
 */
export const getSubscription = createServerFn()
  .inputValidator((data) => subscriptionSchema.parse(data))
  .middleware([authMiddleware])
  .handler(async ({ data, context }) => {
    return billing.getSubscription(
      data.entityType,
      data.entityId,
      context.session.accessToken!,
    );
  });

/**
 * Create a checkout session for a new subscription.
 */
export const getCheckoutUrl = createServerFn({ method: "POST" })
  .inputValidator((data) => checkoutSchema.parse(data))
  .middleware([authMiddleware])
  .handler(async ({ data, context }) => {
    return billing.createCheckoutSession({
      priceId: data.priceId,
      successUrl: data.successUrl ?? `${BASE_URL}/pricing`,
      customerEmail: context.session.user.email!,
      customerName: context.session.user.name ?? undefined,
      metadata: {
        entityType: data.entityType,
        entityId: data.entityId,
        omniProduct: app.name.toLowerCase(),
      },
    });
  });

/**
 * Get billing portal URL for managing a subscription.
 */
export const getBillingPortalUrl = createServerFn({ method: "POST" })
  .inputValidator((data) => billingPortalSchema.parse(data))
  .middleware([authMiddleware])
  .handler(async ({ data, context }) => {
    return billing.getBillingPortalUrl(
      data.entityType,
      data.entityId,
      app.name.toLowerCase(),
      data.returnUrl ?? `${BASE_URL}/profile`,
      context.session.accessToken!,
    );
  });

/**
 * Cancel a subscription.
 */
export const cancelSubscription = createServerFn({ method: "POST" })
  .inputValidator((data) => subscriptionSchema.parse(data))
  .middleware([authMiddleware])
  .handler(async ({ data, context }) => {
    return billing.cancelSubscription(
      data.entityType,
      data.entityId,
      context.session.accessToken!,
    );
  });

/**
 * Renew a subscription (remove scheduled cancellation).
 */
export const renewSubscription = createServerFn({ method: "POST" })
  .inputValidator((data) => subscriptionSchema.parse(data))
  .middleware([authMiddleware])
  .handler(async ({ data, context }) => {
    return billing.renewSubscription(
      data.entityType,
      data.entityId,
      context.session.accessToken!,
    );
  });
