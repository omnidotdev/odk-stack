import { createFileRoute } from "@tanstack/react-router";

import { CancelSubscription, ManageSubscription } from "@/components/profile";
import { fetchSession } from "@/server/functions/auth";
import { getSubscription } from "@/server/functions/subscriptions";

import type { Subscription } from "@/lib/providers/billing";

/**
 * Profile page.
 */
const ProfilePage = () => {
  const { auth, subscription, entityType, entityId } = Route.useLoaderData();

  return (
    <div className="mx-auto flex h-full max-w-7xl flex-col text-pretty px-4 py-8 text-center sm:text-start">
      <div className="flex flex-col">
        <h1 className="font-bold text-xl sm:text-3xl">
          Welcome, {auth?.user.name}!
        </h1>
        <h2 className="text-muted-foreground text-sm sm:text-lg">
          Review and manage details about your account below.
        </h2>
      </div>

      <div className="mt-8 flex flex-col text-start">
        <h1 className="font-bold sm:text-xl">Subscription</h1>
        <h2 className="text-muted-foreground text-xs sm:text-sm">
          View details and manage your subscription.
        </h2>

        {subscription ? (
          <SubscriptionCard
            subscription={subscription}
            entityType={entityType}
            entityId={entityId}
          />
        ) : (
          <p className="mt-4">No active subscription.</p>
        )}
      </div>
    </div>
  );
};

interface SubscriptionCardProps {
  subscription: Subscription;
  entityType: string;
  entityId: string;
}

/**
 * Subscription details card.
 */
const SubscriptionCard = ({
  subscription,
  entityType,
  entityId,
}: SubscriptionCardProps) => {
  const periodEnd = new Date(subscription.currentPeriodEnd * 1000);

  return (
    <div className="mt-4 rounded-lg border p-4">
      <div className="flex items-center justify-between">
        <div>
          <p className="font-medium">{subscription.product?.name ?? "Plan"}</p>
          <p className="text-muted-foreground text-sm">
            Status:{" "}
            <span className="capitalize">
              {subscription.cancelAt ? "Canceling" : subscription.status}
            </span>
          </p>
          {subscription.cancelAt ? (
            <p className="text-muted-foreground text-sm">
              Cancels on: {new Date(subscription.cancelAt * 1000).toLocaleDateString()}
            </p>
          ) : (
            <p className="text-muted-foreground text-sm">
              Renews on: {periodEnd.toLocaleDateString()}
            </p>
          )}
        </div>
        <div className="flex gap-1">
          <ManageSubscription entityType={entityType} entityId={entityId} />
          <CancelSubscription entityType={entityType} entityId={entityId} />
        </div>
      </div>
    </div>
  );
};

export const Route = createFileRoute("/_auth/profile")({
  loader: async () => {
    const { session } = await fetchSession();

    if (!session?.user.identityProviderId) {
      return { auth: session, subscription: null, entityType: "", entityId: "" };
    }

    const entityType = "user";
    const entityId = session.user.identityProviderId;

    const subscription = await getSubscription({
      data: { entityType, entityId },
    });

    return { auth: session, subscription, entityType, entityId };
  },
  component: ProfilePage,
});
