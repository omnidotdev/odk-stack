import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Trash2Icon } from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { cancelSubscription } from "@/server/functions/subscriptions";

interface Props {
  entityType: string;
  entityId: string;
}

/**
 * Cancel subscription.
 */
const CancelSubscription = ({ entityType, entityId }: Props) => {
  const queryClient = useQueryClient();

  const { mutateAsync: cancel, isPending } = useMutation({
    mutationFn: async () =>
      await cancelSubscription({
        data: { entityType, entityId },
      }),
    onSuccess: () => {
      toast.success("Subscription cancelled");
      queryClient.invalidateQueries({ queryKey: ["subscription"] });
    },
    onError: (error) => toast.error(error.message),
  });

  return (
    <Button
      variant="ghost"
      size="icon"
      disabled={isPending}
      onClick={() => cancel()}
    >
      <Trash2Icon className="text-red-500" />
    </Button>
  );
};

export default CancelSubscription;
